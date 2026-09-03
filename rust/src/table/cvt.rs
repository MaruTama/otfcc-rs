#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::base64::base64_decode;
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::font_reader::FontReader;
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::vendor::json::JsonType;

// Stage 7-2-c "inner Vec化": `words` was the only allocation this struct
// owned (Stage 6-4 already Box-ified the outer `CvtTable` itself), so
// `Vec<u16>` plus its own drop glue replaces the manual `free`-based `impl
// Drop` that used to live here. `length` is gone too -- it always equaled
// `words.len()` at every construction site (the allocation size and the
// read/write loop bound were always derived from the same count), so every
// former read of `(*table).length` below now reads `.words.len()` instead.
pub struct CvtTable {
    pub words: Vec<u16>,
}
// Unlike every other table in this batch, the original C (and the first
// Rust translation) here was already memory-safe without a separate length
// guard: `table_length` is derived directly from the table's own declared
// length (`length >> 1`, i.e. `length / 2`), and the read loop is bounded
// by that exact same `table_length` -- so `2 * table_length <= length`
// always holds and no read can go past the end. Migrated anyway for
// consistency with the rest of this batch (dropping `__fortable_*`/
// `.offset()`), not because it fixes a bug.
pub fn otfcc_read_cvt(packet: &Packet, tag: u32) -> Option<Box<CvtTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == tag)?;
    let table_length = (table.data.len() / 2) as u32;
    let mut words: Vec<u16> = Vec::with_capacity(table_length as usize);
    let mut r = FontReader::new(&table.data);
    for _ in 0..table_length as usize {
        words.push(r.u16().expect(
            "table_length is derived from data.len(), so table_length u16 reads always fit",
        ));
    }
    Some(Box::new(CvtTable { words }))
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_cvt(
    table: Option<&CvtTable>,
    root: &mut BuiltValue,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"cvt"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut arr = BuiltValue::new_array(table.words.len());
        for &w in &table.words {
            arr.push_item(BuiltValue::Int(w as i64));
        }
        root.push_field(::core::ffi::CStr::from_ptr(tag).to_bytes(), arr);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_cvt(
    root: &ParsedValue,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) -> Option<Box<CvtTable>> {
    let key = ::core::ffi::CStr::from_ptr(tag).to_bytes();
    if let Some(items) = root
        .get_typed(key, JsonType::Array)
        .and_then(ParsedValue::as_array)
    {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"cvt"),
        );
        let mut words: Vec<u16> = Vec::with_capacity(items.len());
        for record in items {
            words.push(match record {
                ParsedValue::Int(i) => *i as u16,
                ParsedValue::Double(d) => *d as u16,
                _ => 0_u16,
            });
        }
        logger_finish(&mut *options.logger.borrow_mut());
        return Some(Box::new(CvtTable { words }));
    }
    if let Some(bytes) = root
        .get_typed(key, JsonType::String)
        .and_then(ParsedValue::as_str_bytes)
    {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"cvt"),
        );
        let raw = base64_decode(bytes).unwrap_or_default();
        let table_length = raw.len() / 2;
        let mut words: Vec<u16> = Vec::with_capacity(table_length);
        for j in 0..table_length {
            words.push(u16::from_be_bytes([raw[2 * j], raw[2 * j + 1]]));
        }
        logger_finish(&mut *options.logger.borrow_mut());
        return Some(Box::new(CvtTable { words }));
    }
    None
}
pub fn otfcc_build_cvt(table: Option<&CvtTable>) -> Option<Buffer> {
    let table = table?;
    let mut buf = Buffer::new();
    for &w in &table.words {
        buf.write_u16be(w);
    }
    Some(buf)
}
