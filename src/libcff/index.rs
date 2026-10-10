use otfcc_binary::Buffer;
use otfcc_binary::FontReader;
use crate::support::primitives::Arity;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CffIndexCountType {
    U16 = 0,
    U32 = 1,
}
#[derive(Debug)]
pub struct CffIndex {
    pub count_type: CffIndexCountType,
    pub count: Arity,
    pub off_size: u8,
    pub offset: Vec<u32>,
    pub data: Vec<u8>,
}
// `gu1`/`gu2`/`gu3`/`gu4` (1/2/3/4-byte big-endian unsigned reads, no
// bounds checking, no length parameter at all) are gone from this file --
// one of ten near-identical copies across `libcff/` the plan calls out by
// name (`charset.rs`/`fdselect.rs`/`parser.rs` each still have
// their own; converting those is separately scoped follow-up work).
// `FontReader::u8()`/`u16()`/`u24()`/`u32()` are exactly these four reads,
// checked against the buffer's real length.
#[inline]
pub(crate) fn cff_index_dispose(x: &mut CffIndex) {
    x.offset = Vec::new();
    x.data = Vec::new();
}
/// An empty INDEX. `CffFile` is built from these, so every field starts as a
/// valid value.
pub(crate) fn new_empty_cff_index() -> CffIndex {
    CffIndex {
        count_type: CffIndexCountType::U16,
        count: 0 as Arity,
        off_size: 0,
        offset: Vec::new(),
        data: Vec::new(),
    }
}
pub(crate) fn get_index_length(i: &CffIndex) -> u32 {
    if i.count != 0 {
        // Read from the font, so the arithmetic wraps rather than panics on
        // a corrupt offset.
        let data_len = i.offset[i.count as usize].wrapping_sub(1);
        let offsets_len = i.count.wrapping_add(1).wrapping_mul(i.off_size as u32);
        return 3u32.wrapping_add(data_len).wrapping_add(offsets_len);
    } else {
        return 3;
    };
}
pub(crate) fn empty_index(i: &mut CffIndex) {
    cff_index_dispose(i);
    i.count_type = CffIndexCountType::U16;
    i.count = 0 as Arity;
    i.off_size = 0;
}
// Reads the INDEX at `pos` into `in_0`. Every read is checked against the
// table, so a malformed INDEX (one whose last offset is 0 would otherwise
// claim a data block of nearly 4 GB) leaves `in_0` empty, like an INDEX
// with a count of 0.
pub(crate) fn extract_index(data: &[u8], pos: u32, in_0: &mut CffIndex) {
    let result: Option<()> = 'parse: {
        let Ok(mut r) = FontReader::new(data).at(pos as usize) else {
            break 'parse None;
        };
        let Ok(count) = r.u16().map(|v| v as Arity) else {
            break 'parse None;
        };
        let Ok(off_size) = r.u8() else {
            break 'parse None;
        };
        in_0.count = count;
        in_0.off_size = off_size;
        if count > 0 as Arity {
            if !(1..=4).contains(&off_size) {
                break 'parse None;
            }
            if r.require_room(count as usize + 1, off_size as usize)
                .is_err()
            {
                break 'parse None;
            }
            let mut offset: Vec<u32> = Vec::with_capacity(count as usize + 1);
            for _ in 0..=count {
                let Ok(v) = (match off_size {
                    1 => r.u8().map(|v| v as u32),
                    2 => r.u16().map(|v| v as u32),
                    3 => r.u24(),
                    _ => r.u32(),
                }) else {
                    break 'parse None;
                };
                offset.push(v);
            }
            // CFF INDEX offsets are 1-based and required to be non-
            // decreasing (Adobe TN #5176 section 5) -- every reader of
            // `.offset[]` (`get_cff_sid`, `locate_subr`) assumes both, but
            // nothing enforced either here before this. A malformed INDEX
            // with `offset[i] > offset[i + 1]` for some `i` made a caller
            // computing `offset[i + 1].wrapping_sub(offset[i])` (subtracting
            // the *larger* value from the smaller) wrap around to a length
            // near `u32::MAX`, either requesting a multi-gigabyte
            // allocation or reading arbitrarily far past `data`'s real end
            // -- found by `cargo fuzz run otf_parse` as a heap-buffer-
            // overflow inside `get_cff_sid` (confirmed under a higher
            // memory limit than the plain OOM report alone showed).
            // `locate_subr` already guarded its own two-entry read this
            // way (`start < 1 || end < start`); validating the whole array
            // once here, at the one place it's actually parsed, covers
            // every current and future reader instead of relying on each
            // call site to remember its own copy of the same check.
            if offset.iter().any(|&v| v < 1) || offset.windows(2).any(|w| w[1] < w[0]) {
                break 'parse None;
            }
            let Some(data_len) = offset[count as usize].checked_sub(1) else {
                break 'parse None;
            };
            let Ok(body) = r.bytes(data_len as usize) else {
                break 'parse None;
            };
            in_0.offset = offset;
            in_0.data = body.to_vec();
        } else {
            in_0.offset = Vec::new();
            in_0.data = Vec::new();
        }
        break 'parse Some(());
    };
    if result.is_none() {
        in_0.count = 0 as Arity;
        in_0.off_size = 0;
        in_0.offset = Vec::new();
        in_0.data = Vec::new();
    }
}
// Builds an INDEX from `length` buffers taken from `items`.
pub(crate) fn new_index_by_callback(
    length: u32,
    mut items: impl Iterator<Item = Buffer>,
) -> CffIndex {
    let count = length as Arity;
    let mut offset: Vec<u32> = Vec::with_capacity(count as usize + 1);
    offset.push(1);
    let mut data: Vec<u8> = Vec::new();
    for _ in 0..length {
        let blob: Buffer = items.next().expect("iterator shorter than length");
        data.extend_from_slice(&blob.data);
        offset.push(data.len() as u32 + 1);
    }
    CffIndex {
        count_type: CffIndexCountType::U16,
        count,
        off_size: 4,
        offset,
        data,
    }
}
pub(crate) fn build_index(index: &CffIndex) -> Buffer {
    let mut blob = Buffer::new();
    if index.count == 0 {
        blob.write_bytes(&[0, 0, 0]);
        return blob;
    }
    let offset = &index.offset;
    let last_offset: u32 = offset[index.count as usize];
    let off_size: usize = if last_offset < 0x100 {
        1
    } else if last_offset < 0x10000 {
        2
    } else if last_offset < 0x1000000 {
        3
    } else {
        4
    };
    blob.write_u16be(index.count as u16);
    blob.write_u8(off_size as u8);
    for &offset_i in &offset[..=index.count as usize] {
        // Each offset big-endian, in its last `off_size` bytes.
        blob.write_bytes(&offset_i.to_be_bytes()[4 - off_size..]);
    }
    if !index.data.is_empty() {
        let n = offset[index.count as usize].wrapping_sub(1) as usize;
        blob.write_bytes(&index.data[..n]);
    }
    return blob;
}

#[cfg(test)]
mod extract_index_tests {
    use super::*;

    #[test]
    fn reads_a_well_formed_one_entry_index() {
        // count=1, off_size=1, offset=[1,3] (data is 2 bytes), data=[0xAA,0xBB]
        let data = [0x00u8, 0x01, 0x01, 0x01, 0x03, 0xAA, 0xBB];
        let mut idx = new_empty_cff_index();
            extract_index(&data, 0, &mut idx);
            assert_eq!(idx.count, 1);
            assert_eq!(idx.off_size, 1);
            assert_eq!(idx.offset, vec![1, 3]);
            assert_eq!(idx.data, vec![0xAA, 0xBB]);
    }

    #[test]
    fn reads_an_empty_index() {
        let data = [0x00u8, 0x00, 0x00]; // count=0, off_size=0
        let mut idx = new_empty_cff_index();
            extract_index(&data, 0, &mut idx);
            assert_eq!(idx.count, 0);
            assert!(idx.offset.is_empty());
            assert!(idx.data.is_empty());
    }

    #[test]
    fn last_offset_of_zero_is_rejected_instead_of_a_4gb_memcpy() {
        // count=1, off_size=1, offset=[1,0] -- the last offset entry is 0,
        // which is invalid per spec (offsets are 1-based and
        // non-decreasing). `offset[count] - 1` must not wrap to 0xFFFFFFFF
        // and copy ~4GB.
        let data = [0x00u8, 0x01, 0x01, 0x01, 0x00];
        let mut idx = new_empty_cff_index();
            extract_index(&data, 0, &mut idx);
            assert_eq!(idx.count, 0);
            assert!(idx.offset.is_empty());
            assert!(idx.data.is_empty());
    }

    #[test]
    fn truncated_offset_array_is_rejected_instead_of_reading_oob() {
        // count=5, off_size=4, but the table ends right after off_size --
        // the offset array (and everything past it) is missing entirely.
        let data = [0x00u8, 0x05, 0x04];
        let mut idx = new_empty_cff_index();
            extract_index(&data, 0, &mut idx);
            assert_eq!(idx.count, 0);
            assert!(idx.offset.is_empty());
            assert!(idx.data.is_empty());
    }

    #[test]
    fn data_block_longer_than_the_table_is_rejected_instead_of_reading_oob() {
        // count=1, off_size=1, offset=[1,200] (implying a 199-byte data
        // block) but the table only has 2 more bytes after the offset
        // array -- previously unguarded even when the offsets themselves
        // are internally well-formed (not the wraparound case above).
        let data = [0x00u8, 0x01, 0x01, 0x01, 200u8, 0xAA, 0xBB];
        let mut idx = new_empty_cff_index();
            extract_index(&data, 0, &mut idx);
            assert_eq!(idx.count, 0);
            assert!(idx.offset.is_empty());
            assert!(idx.data.is_empty());
    }

    #[test]
    fn non_decreasing_offsets_are_required_not_just_the_last_one() {
        // count=2, off_size=1, offset=[1, 5, 3] -- entry 1 (5) is *larger*
        // than entry 2 (3), so the array isn't monotonic even though the
        // final entry (3) alone looks fine. A consumer computing
        // `offset[i + 1] - offset[i]` for the *first* entry's length
        // (5 - 1 = 4) would demand data this index never actually promised
        // -- and one going the other direction (`offset[2] - offset[1]` =
        // 3 - 5, unsigned) is exactly the `get_cff_sid` wraparound this
        // guard exists to close.
        let data = [0x00u8, 0x02, 0x01, 0x01, 0x05, 0x03, 0xAA, 0xBB, 0xCC];
        let mut idx = new_empty_cff_index();
            extract_index(&data, 0, &mut idx);
            assert_eq!(idx.count, 0);
            assert!(idx.offset.is_empty());
    }

    #[test]
    fn invalid_off_size_is_rejected_instead_of_producing_all_zero_offsets() {
        // off_size must be 1-4; an out-of-range value would otherwise read
        // as an all-zero offset array, whose last entry is 0 -- the same
        // wraparound as above.
        let data = [0x00u8, 0x01, 0x05, 0x00, 0x00];
        let mut idx = new_empty_cff_index();
            extract_index(&data, 0, &mut idx);
            assert_eq!(idx.count, 0);
    }
}
