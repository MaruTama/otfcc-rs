use otfcc_binary::bk::block::{
    BkBlock, BkCellType, bk_int, bk_new_block, bk_new_block_from_bytes, bk_ptr, bk_push,
};
use otfcc_binary::bk::graph::bk_build_block;
use crate::font::sfnt::Packet;
use crate::support::base64::{base64_decode, base64_encode};
use otfcc_binary::Buffer;
use otfcc_json::BuiltValue;
use otfcc_binary::{FontReader, ReadError};
use otfcc_json::ParsedValue;
use otfcc_json::JsonType;

// `.data` holds either a UTF-8 string tag's bytes or raw (possibly
// non-UTF-8) base64-decoded bytes, so `Vec<u8>`, not `String`.
#[derive(Debug)]
pub struct MetaEntry {
    pub tag: u32,
    pub data: Vec<u8>,
}
// Stage 6-4 "Box化": every field this struct (transitively) owns is
// already a `Vec`/scalar, so no `Drop` impl is needed -- `Box::new`
// construction plus the standard drop glue is sufficient. The entire
// `MetaTableElementInterface` vtable is deleted: grepping confirmed only
// `.create`/`.free` were ever called from outside this file.
#[derive(Debug)]
pub struct MetaTable {
    pub version: u32,
    pub flags: u32,
    pub entries: Vec<MetaEntry>,
}

// The original guarded the entry array with `table.length <
// 16.wrapping_add(12.wrapping_mul(data_maps_count))` -- a `data_maps_count`
// large enough to overflow `12 * count` (e.g. 0x1555_5556) wraps the sum
// back down to something small, so the guard passes even though the real
// entry array is nowhere near that short; the loop then read each entry's
// `tag`/`offset`/`length` straight past the table's actual end.
// `require_room` closes this the same way it does everywhere else in this
// stage: `checked_mul`/`checked_add`, so an overflowing count fails the
// guard instead of wrapping through it.
//
// Each entry's own data span (`offset..offset+length`) had the same
// wrapping-arithmetic gap (`table.length < offset.wrapping_add(length)`);
// `FontReader::sub`'s `checked_add` replaces it. Unlike the header guard,
// a single entry failing this check does not drop the whole table --
// matching the original, which silently skipped just that one entry and
// kept going.
fn decode_meta(data: &[u8]) -> Result<MetaTable, ReadError> {
    let mut r = FontReader::new(data);
    let version = r.u32()?;
    let flags = r.u32()?;
    r.skip(4)?; // reserved
    let data_maps_count = r.u32()? as usize;
    r.require_room(data_maps_count, 12)?;
    let mut entries = Vec::with_capacity(data_maps_count);
    for _ in 0..data_maps_count {
        let tag = r.u32()?;
        let offset = r.u32()?;
        let length = r.u32()?;
        if let Ok(bytes) = FontReader::new(data)
            .sub(offset as usize, length as usize)
            .and_then(|mut sr| sr.bytes(length as usize))
        {
            entries.push(MetaEntry {
                tag,
                data: bytes.to_vec(),
            });
        }
    }
    Ok(MetaTable {
        version,
        flags,
        entries,
    })
}
pub fn read_meta(packet: &Packet) -> Option<Box<MetaTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_META)?;
    match decode_meta(&table.data) {
        Ok(meta) => Some(Box::new(meta)),
        Err(_) => {
            tracing::warn!("Table 'meta' corrupted.\n");
            None
        }
    }
}

// `extern "C"` is a c2rust artifact -- this is only ever called from
// `parse_meta` in this same file, never across a real FFI boundary,
// same reasoning as every other `#[allow(improper_ctypes_definitions)]`
// in this migration.
#[allow(improper_ctypes_definitions)]
pub fn parse_meta_data(v: Option<&ParsedValue>) -> Option<Vec<u8>> {
    let v = v?;
    if let Some(bytes) = v.as_str_bytes() {
        return Some(bytes.to_vec());
    }
    if v.as_object().is_some() {
        if let Some(bytes) = v.get_bytes(b"string") {
            return Some(bytes.to_vec());
        }
        if let Some(bytes) = v.get_bytes(b"base64") {
            // Unlike the `string` field above, a malformed `base64` field
            // (a character count not a multiple of 4) now makes this whole
            // entry `None` instead of silently keeping an empty decoded
            // value -- the same "drop what doesn't parse" choice already
            // made everywhere else malformed JSON-build input is handled.
            return base64_decode(bytes);
        }
    }
    None
}
pub fn parse_meta(root: &ParsedValue) -> Option<Box<MetaTable>> {
    let _meta = root.get_typed(b"meta", JsonType::Object)?;
    let entries = _meta
        .get_typed(b"entries", JsonType::Array)
        .and_then(ParsedValue::as_array)?;
    let mut meta: Box<MetaTable> = Box::new(MetaTable {
        version: 1,
        flags: 0,
        entries: Vec::new(),
    });
    let stage = crate::logger::stage("meta");
    for _e in entries {
        let Some(tag_bytes) = _e
            .get_typed(b"tag", JsonType::String)
            .and_then(ParsedValue::as_str_bytes)
            .filter(|b| b.len() == 4)
        else {
            continue;
        };
        let tag: u32 = str2tag(Some(tag_bytes));
        if let Some(data) = parse_meta_data(Some(_e)) {
            meta.entries.push(MetaEntry { tag, data });
        }
    }
    stage.finish();
    Some(meta)
}
#[inline]
fn str2tag(tags: Option<&[u8]>) -> u32 {
    let Some(tags) = tags else {
        return 0_u32;
    };
    let mut tag: u32 = 0_u32;
    let mut len: u8 = 0_u8;
    for &b in tags.iter().take(4) {
        tag = tag << 8_i32 | b as u32;
        len = len.wrapping_add(1);
    }
    for _ in len..4_u8 {
        tag = tag << 8_i32 | ' ' as i32 as u32;
    }
    tag
}

#[inline]
fn is_string_tag(tag: u32) -> bool {
    return tag == crate::tag::TAG_DLNG || tag == crate::tag::TAG_SLNG;
}
pub fn dump_meta(meta: Option<&MetaTable>, root: &mut BuiltValue) {
    let Some(meta) = meta else {
        return;
    };
    let stage = crate::logger::stage("meta");
    let mut _meta = BuiltValue::new_object(3);
    _meta.push_field(b"version", BuiltValue::Int(meta.version as i64));
    _meta.push_field(b"flags", BuiltValue::Int(meta.flags as i64));
    let entries: &Vec<MetaEntry> = &meta.entries;
    let mut _entries = BuiltValue::new_array(entries.len());
    for e in entries.iter() {
        let mut _e = BuiltValue::new_object(2);
        let tag_bytes: [u8; 4] = [
            ((e.tag & 0xff000000u32) >> 24) as u8,
            ((e.tag & 0xff0000u32) >> 16) as u8,
            ((e.tag & 0xff00u32) >> 8) as u8,
            (e.tag & 0xffu32) as u8,
        ];
        _e.push_field(b"tag", BuiltValue::Str(tag_bytes.to_vec()));
        if is_string_tag(e.tag) {
            _e.push_field(b"string", BuiltValue::Str(e.data.clone()));
        } else {
            let encoded = base64_encode(&e.data);
            _e.push_field(b"base64", BuiltValue::Str(encoded));
        }
        _entries.push_item(_e);
    }
    _meta.push_field(b"entries", _entries);
    root.push_field(b"meta", _meta);
    stage.finish();
}

#[allow(improper_ctypes_definitions)]
pub fn build_meta(meta: Option<&MetaTable>) -> Option<Buffer> {
    let meta = match meta {
        Some(m) if !m.entries.is_empty() => m,
        _ => return None,
    };
    let entries: &Vec<MetaEntry> = &meta.entries;
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B32, meta.version),
        bk_int(BkCellType::B32, meta.flags),
        bk_int(BkCellType::B32, 0_u32),
        bk_int(BkCellType::B32, entries.len() as u32),
    ]);
    for e in entries.iter() {
        bk_push(
            &mut root,
            vec![
                bk_int(BkCellType::B32, e.tag),
                bk_ptr(
                    BkCellType::P32,
                    Some(bk_new_block_from_bytes(&e.data)),
                ),
                bk_int(BkCellType::B32, (e.data.len()) as u32),
            ],
        );
    }
    Some(bk_build_block(root))
}

#[cfg(test)]
mod parse_meta_tests {
    use super::*;

    fn header(version: u32, flags: u32, data_maps_count: u32) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&version.to_be_bytes());
        b.extend_from_slice(&flags.to_be_bytes());
        b.extend_from_slice(&0u32.to_be_bytes()); // reserved
        b.extend_from_slice(&data_maps_count.to_be_bytes());
        b
    }

    #[test]
    fn well_formed_table_reads_one_entry() {
        let mut data = header(1, 0, 1);
        data.extend_from_slice(b"dlng"); // tag
        data.extend_from_slice(&28u32.to_be_bytes()); // offset: right after the 16-byte header + 12-byte entry
        data.extend_from_slice(&3u32.to_be_bytes()); // length
        data.extend_from_slice(b"en-US");
        let meta = decode_meta(&data).unwrap();
        assert_eq!(meta.entries.len(), 1);
        assert_eq!(meta.entries[0].data, b"en-".to_vec());
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(decode_meta(&header(1, 0, 0)[..10]).is_err());
    }

    #[test]
    fn data_maps_count_large_enough_to_overflow_the_multiplication_errs() {
        // 0x1555_5556 * 12 overflows u32/usize-on-32-bit math back to a
        // small number under wrapping arithmetic; `require_room`'s
        // `checked_mul` must reject this instead of wrapping through it.
        let data = header(1, 0, 0x1555_5556);
        assert!(decode_meta(&data).is_err());
    }

    #[test]
    fn entry_whose_span_overflows_offset_plus_length_is_dropped_not_the_whole_table() {
        let mut data = header(1, 0, 1);
        data.extend_from_slice(b"dlng");
        data.extend_from_slice(&0xFFFF_FFF0u32.to_be_bytes()); // offset
        data.extend_from_slice(&0x0000_0020u32.to_be_bytes()); // length; offset+length overflows u32
        let meta = decode_meta(&data).unwrap();
        assert!(meta.entries.is_empty());
    }

    #[test]
    fn entry_span_past_the_table_end_is_dropped_not_the_whole_table() {
        let mut data = header(1, 0, 1);
        data.extend_from_slice(b"dlng");
        data.extend_from_slice(&100u32.to_be_bytes()); // offset past the table end
        data.extend_from_slice(&3u32.to_be_bytes());
        let meta = decode_meta(&data).unwrap();
        assert!(meta.entries.is_empty());
    }
}
