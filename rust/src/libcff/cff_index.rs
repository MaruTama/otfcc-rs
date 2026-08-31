#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::buffer::Buffer;
use crate::support::buffer::{bufnew, bufwrite8, bufwrite_bytes};
use crate::support::font_reader::FontReader;
use crate::support::primitives::Arity;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CffIndexCountType {
    U16 = 0,
    U32 = 1,
}
// `offset`/`data` were `__caryll_allocate_clean`'d/`free`'d raw arrays,
// sized from a font-byte-derived `count` in `extract_index` (the parse
// path) -- a genuine untrusted-input-driven allocation, not just style.
// `Vec` removes the manual free pair and the OOB-write risk a counting
// mistake there would have caused. Neither array is ever aliased outside
// this struct's own accessor functions, so no `Copy`/`Clone` derive
// survives (matches every other malloc-array-to-Vec conversion this crate
// has made).
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
// name (`cff_charset.rs`/`cff_fdselect.rs`/`cff_parser.rs` each still have
// their own; converting those is separately scoped follow-up work).
// `FontReader::u8()`/`u16()`/`u24()`/`u32()` are exactly these four reads,
// checked against the buffer's real length.
#[inline]
unsafe fn dispose_cff_index(in_0: *mut CffIndex) {
    (*in_0).offset = Vec::new();
    (*in_0).data = Vec::new();
}
#[inline]
pub(crate) unsafe fn cff_index_dispose(x: *mut CffIndex) {
    dispose_cff_index(x);
}
#[inline]
pub(crate) unsafe fn cff_index_free(x: *mut CffIndex) {
    if x.is_null() {
        return;
    }
    // `offset`/`data` are still freed here exactly as before -- only the
    // outer shell's own allocator changed, from a bare `malloc`/`free`
    // pair to `Box::into_raw`/`Box::from_raw`. Every `cff_index_create`/
    // `cff_index_free` call site pairs consistently (confirmed by grep: no
    // generic adapter reclaims a `*mut CffIndex` any other way, unlike
    // `GposPairSubtable`'s `subtable_from_raw`), so this is self-contained.
    cff_index_dispose(x);
    drop(Box::from_raw(x));
}
// A real, valid, empty `CffIndex` value -- as opposed to the all-zero bit
// pattern `__caryll_allocate_clean` (calloc) would produce, which is NOT a
// valid `CffIndex` since it owns two `Vec`s. Also used by `cff_parser.rs`'s
// `cff_open_stream` to build a whole `CffFile` (which embeds 7 of these) as
// one real value up front, instead of calloc'ing `CffFile` and letting each
// field's first write be a plain `=` onto still-invalid zeroed memory (see
// [[otfcc-vec-field-assign-needs-calloc]]).
pub(crate) fn new_empty_cff_index() -> CffIndex {
    CffIndex {
        count_type: CffIndexCountType::U16,
        count: 0 as Arity,
        off_size: 0,
        offset: Vec::new(),
        data: Vec::new(),
    }
}
#[inline]
pub(crate) unsafe fn cff_index_create() -> *mut CffIndex {
    // `Box::new` of an explicit all-zero literal, not `malloc` + `cff_index_
    // init`'s `memset`: same fields, same zero values, but a real Rust
    // allocation from here on -- see `cff_index_free`'s matching `Box::
    // from_raw`. `cff_index_init` itself stays (and keeps using `memset`):
    // it also zero-initializes a stack-local `CffIndex` at its one other
    // call site (`table/cff.rs`), which was never a `malloc`/`Box`
    // allocation to begin with.
    Box::into_raw(Box::new(new_empty_cff_index()))
}
#[inline]
pub(crate) unsafe fn cff_index_init(x: *mut CffIndex) {
    // No all-zero bit pattern is a valid `CffIndex` any more (it owns two
    // `Vec` fields), so place a valid empty value directly instead of the
    // old `memset`.
    ::core::ptr::write(
        x,
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0 as Arity,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        },
    );
}
pub(crate) unsafe fn get_index_length(i: *const CffIndex) -> u32 {
    if (*i).count != 0 as Arity {
        let offset = &(*i).offset;
        return 3_u32
            .wrapping_add((offset[(*i).count as usize]).wrapping_sub(1_u32))
            .wrapping_add(
                (*i).count
                    .wrapping_add(1_u32)
                    .wrapping_mul((*i).off_size as u32),
            );
    } else {
        return 3_u32;
    };
}
pub(crate) unsafe fn empty_index(i: *mut CffIndex) {
    cff_index_dispose(i);
    (*i).count_type = CffIndexCountType::U16;
    (*i).count = 0 as Arity;
    (*i).off_size = 0;
}
// This used to run entirely off a bare `*mut u8` with no length at all --
// `count`/`off_size` and the whole `offset[]` array were read with no
// bounds checking whatsoever, and the final `data_len` (the INDEX's data
// block size) was computed as `offset[count].wrapping_sub(1)`: a
// malformed INDEX whose last entry is 0 (invalid per spec -- offsets are
// 1-based and non-decreasing, so a well-formed INDEX's last offset is
// always >= 1) wrapped that subtraction to `0xFFFFFFFF`, and the `memcpy`
// that followed copied up to ~4GB from wherever `data` happened to point
// (the exact bug the plan's own writeup names by file and line). Every
// read here now goes through `FontReader`, checked against `table_length`
// -- that alone closes the wraparound (a `data_len` this large can never
// fit in a real table, so `bytes()` below simply fails) without needing a
// separate `checked_sub` special case. On any bounds failure `in_0` is
// left as an empty index (matching this function's own existing "count
// == 0" branch) rather than reading adjacent bytes -- the original never
// had a failure path to distinguish "malformed" from "legitimately
// empty" at all.
pub(crate) unsafe fn extract_index(
    data: *mut u8,
    table_length: u32,
    pos: u32,
    in_0: *mut CffIndex,
) {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let result: Option<()> = 'parse: {
        let Ok(mut r) = FontReader::new(slice).at(pos as usize) else {
            break 'parse None;
        };
        let Ok(count) = r.u16().map(|v| v as Arity) else {
            break 'parse None;
        };
        let Ok(off_size) = r.u8() else {
            break 'parse None;
        };
        (*in_0).count = count;
        (*in_0).off_size = off_size;
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
            (*in_0).offset = offset;
            (*in_0).data = body.to_vec();
        } else {
            (*in_0).offset = Vec::new();
            (*in_0).data = Vec::new();
        }
        break 'parse Some(());
    };
    if result.is_none() {
        (*in_0).count = 0 as Arity;
        (*in_0).off_size = 0;
        (*in_0).offset = Vec::new();
        (*in_0).data = Vec::new();
    }
}
// Was a `context: *mut c_void` + function-pointer pair (three call sites,
// each with a genuinely different context shape: `subr.rs`'s `from_array`
// walked an existing `Vec<Buffer>` via `.offset()`, `table/cff.rs`'s
// `callback_makestringindex` indexed a `Vec<*mut Buffer>`, and
// `callback_makefd` built a fresh `Buffer` per call from an
// `FdArrayCompileContext`). All three call in strictly increasing `i`
// order, once each -- an `impl Iterator<Item = Buffer>` replaces the
// `void*` erasure entirely: each call site now builds an iterator that
// matches its own real type directly, no shared context struct needed.
// This also folds in what used to be the callback's own allocation +
// this function's matching `buffree`: the iterator yields owned
// `Buffer`s, each consumed (and dropped) exactly once per `.next()`.
pub(crate) unsafe fn new_index_by_callback(
    length: u32,
    mut items: impl Iterator<Item = Buffer>,
) -> *mut CffIndex {
    let idx: *mut CffIndex = (cff_index_create)();
    (*idx).count = length as Arity;
    let mut offset: Vec<u32> = vec![0_u32; (*idx).count.wrapping_add(1 as Arity) as usize];
    offset[0_usize] = 1_u32;
    let mut data: Vec<u8> = Vec::new();
    let mut used: usize = 0_usize;
    let mut blank: usize = 0_usize;
    let mut i: Arity = 0 as Arity;
    while i < length {
        let blob: Buffer = items.next().expect("iterator shorter than length");
        let blob_size: usize = blob.data.len();
        if blank < blob_size {
            used = used.wrapping_add(blob_size);
            blank = used >> 1_i32 & 0xffffff_i32 as usize;
            data.resize(used.wrapping_add(blank), 0_u8);
        } else {
            used = used.wrapping_add(blob_size);
            blank = blank.wrapping_sub(blob_size);
        }
        let write_at: usize = (offset[i as usize] as usize).wrapping_sub(1_usize);
        offset[i.wrapping_add(1 as Arity) as usize] =
            blob_size.wrapping_add(offset[i as usize] as usize) as u32;
        data[write_at..write_at.wrapping_add(blob_size)].copy_from_slice(&blob.data);
        i = i.wrapping_add(1);
    }
    data.truncate(used);
    (*idx).offset = offset;
    (*idx).data = data;
    (*idx).off_size = 4_u8;
    return idx;
}
pub(crate) unsafe fn build_index(index: *const CffIndex) -> *mut Buffer {
    let blob: *mut Buffer = bufnew();
    if (*index).count == 0 {
        bufwrite8(blob, 0_u8);
        bufwrite8(blob, 0_u8);
        bufwrite8(blob, 0_u8);
        return blob;
    }
    let offset = &(*index).offset;
    let last_offset: u32 = offset[(*index).count as usize];
    let off_size: u8;
    if last_offset < 0x100_u32 {
        off_size = 1_u8;
    } else if last_offset < 0x10000_u32 {
        off_size = 2_u8;
    } else if last_offset < 0x1000000_u32 {
        off_size = 3_u8;
    } else {
        off_size = 4_u8;
    }
    bufwrite8(blob, (*index).count.wrapping_div(256 as Arity) as u8);
    bufwrite8(blob, (*index).count.wrapping_rem(256 as Arity) as u8);
    bufwrite8(blob, off_size);
    if (*index).count > 0 as Arity {
        let mut i: Arity = 0 as Arity;
        while i <= (*index).count {
            let offset_i: u32 = offset[i as usize];
            match off_size as i32 {
                1 => {
                    bufwrite8(blob, offset_i as u8);
                }
                2 => {
                    bufwrite8(blob, offset_i.wrapping_div(256_u32) as u8);
                    bufwrite8(blob, offset_i.wrapping_rem(256_u32) as u8);
                }
                3 => {
                    bufwrite8(blob, offset_i.wrapping_div(65536_u32) as u8);
                    bufwrite8(
                        blob,
                        offset_i.wrapping_rem(65536_u32).wrapping_div(256_u32) as u8,
                    );
                    bufwrite8(
                        blob,
                        offset_i.wrapping_rem(65536_u32).wrapping_rem(256_u32) as u8,
                    );
                }
                4 => {
                    bufwrite8(
                        blob,
                        offset_i.wrapping_div(65536_u32).wrapping_div(256_u32) as u8,
                    );
                    bufwrite8(
                        blob,
                        offset_i.wrapping_div(65536_u32).wrapping_rem(256_u32) as u8,
                    );
                    bufwrite8(
                        blob,
                        offset_i.wrapping_rem(65536_u32).wrapping_div(256_u32) as u8,
                    );
                    bufwrite8(
                        blob,
                        offset_i.wrapping_rem(65536_u32).wrapping_rem(256_u32) as u8,
                    );
                }
                _ => {}
            }
            i = i.wrapping_add(1);
        }
        if !(*index).data.is_empty() {
            let n = (offset[(*index).count as usize]).wrapping_sub(1_u32) as usize;
            bufwrite_bytes(blob, n, (*index).data.as_ptr());
        }
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
        unsafe {
            let idx = cff_index_create();
            extract_index(data.as_ptr() as *mut u8, data.len() as u32, 0, idx);
            assert_eq!((*idx).count, 1);
            assert_eq!((*idx).off_size, 1);
            assert_eq!((*idx).offset, vec![1, 3]);
            assert_eq!((*idx).data, vec![0xAA, 0xBB]);
            cff_index_free(idx);
        }
    }

    #[test]
    fn reads_an_empty_index() {
        let data = [0x00u8, 0x00, 0x00]; // count=0, off_size=0
        unsafe {
            let idx = cff_index_create();
            extract_index(data.as_ptr() as *mut u8, data.len() as u32, 0, idx);
            assert_eq!((*idx).count, 0);
            assert!((*idx).offset.is_empty());
            assert!((*idx).data.is_empty());
            cff_index_free(idx);
        }
    }

    #[test]
    fn last_offset_of_zero_is_rejected_instead_of_a_4gb_memcpy() {
        // count=1, off_size=1, offset=[1,0] -- the last offset entry is 0,
        // which is invalid per spec (offsets are 1-based and
        // non-decreasing). The original computed `data_len =
        // offset[count].wrapping_sub(1)`, which wraps a 0 to
        // 0xFFFFFFFF and `memcpy`s up to ~4GB from wherever `data`
        // happened to point -- the exact bug the plan's own writeup
        // names by file and line.
        let data = [0x00u8, 0x01, 0x01, 0x01, 0x00];
        unsafe {
            let idx = cff_index_create();
            extract_index(data.as_ptr() as *mut u8, data.len() as u32, 0, idx);
            assert_eq!((*idx).count, 0);
            assert!((*idx).offset.is_empty());
            assert!((*idx).data.is_empty());
            cff_index_free(idx);
        }
    }

    #[test]
    fn truncated_offset_array_is_rejected_instead_of_reading_oob() {
        // count=5, off_size=4, but the table ends right after off_size --
        // the offset array (and everything past it) is missing entirely.
        // The original had no length parameter to check this against at
        // all.
        let data = [0x00u8, 0x05, 0x04];
        unsafe {
            let idx = cff_index_create();
            extract_index(data.as_ptr() as *mut u8, data.len() as u32, 0, idx);
            assert_eq!((*idx).count, 0);
            assert!((*idx).offset.is_empty());
            assert!((*idx).data.is_empty());
            cff_index_free(idx);
        }
    }

    #[test]
    fn data_block_longer_than_the_table_is_rejected_instead_of_reading_oob() {
        // count=1, off_size=1, offset=[1,200] (implying a 199-byte data
        // block) but the table only has 2 more bytes after the offset
        // array -- previously unguarded even when the offsets themselves
        // are internally well-formed (not the wraparound case above).
        let data = [0x00u8, 0x01, 0x01, 0x01, 200u8, 0xAA, 0xBB];
        unsafe {
            let idx = cff_index_create();
            extract_index(data.as_ptr() as *mut u8, data.len() as u32, 0, idx);
            assert_eq!((*idx).count, 0);
            assert!((*idx).offset.is_empty());
            assert!((*idx).data.is_empty());
            cff_index_free(idx);
        }
    }

    #[test]
    fn non_decreasing_offsets_are_required_not_just_the_last_one() {
        // count=2, off_size=1, offset=[1, 5, 3] -- entry 1 (5) is *larger*
        // than entry 2 (3), so the array isn't monotonic even though the
        // final entry (3) alone would pass the old checked_sub(1) check
        // fine. A consumer computing `offset[i + 1] - offset[i]` for the
        // *first* entry's length (5 - 1 = 4) would demand data this index
        // never actually promised -- and one going the other direction
        // (`offset[2] - offset[1]` = 3 - 5, unsigned) is exactly the
        // `get_cff_sid` wraparound this guard exists to close.
        let data = [0x00u8, 0x02, 0x01, 0x01, 0x05, 0x03, 0xAA, 0xBB, 0xCC];
        unsafe {
            let idx = cff_index_create();
            extract_index(data.as_ptr() as *mut u8, data.len() as u32, 0, idx);
            assert_eq!((*idx).count, 0);
            assert!((*idx).offset.is_empty());
            cff_index_free(idx);
        }
    }

    #[test]
    fn invalid_off_size_is_rejected_instead_of_producing_all_zero_offsets() {
        // off_size must be 1-4; the original's `match` fell through to
        // pushing 0 for every offset entry on an out-of-range value,
        // which is just another way to reach the same wraparound bug
        // above (an all-zero offset array's last entry is 0).
        let data = [0x00u8, 0x01, 0x05, 0x00, 0x00];
        unsafe {
            let idx = cff_index_create();
            extract_index(data.as_ptr() as *mut u8, data.len() as u32, 0, idx);
            assert_eq!((*idx).count, 0);
            cff_index_free(idx);
        }
    }
}
