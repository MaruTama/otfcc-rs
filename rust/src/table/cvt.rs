#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::base64::base64_decode;
use crate::support::binio::read_16u;
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::font_reader::FontReader;
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_dbl_val, json_int_val, json_obj_get_type,
    json_str_len, json_str_ptr, json_type_of,
};
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
    root: *const ParsedValue,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) -> Option<Box<CvtTable>> {
    let mut t: Option<Box<CvtTable>> = None;
    let mut table: *const ParsedValue;
    table = json_obj_get_type(root, tag, JsonType::Array);
    if !table.is_null() {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"cvt"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let table_length = json_arr_len(table);
            let mut words: Vec<u16> = Vec::with_capacity(table_length as usize);
            let mut j: u16 = 0_u16;
            while (j as u32) < table_length {
                let record: *const ParsedValue = json_arr_at(table, j as u32);
                if json_type_of(record) == JsonType::Integer {
                    words.push(json_int_val(record) as u16);
                } else if json_type_of(record) == JsonType::Double {
                    words.push(json_dbl_val(record) as u16);
                } else {
                    words.push(0_u16);
                }
                j = j.wrapping_add(1);
            }
            t = Some(Box::new(CvtTable { words }));
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    } else {
        table = json_obj_get_type(root, tag, JsonType::String);
        if !table.is_null() {
            logger_start_sds(
                &mut *options.logger.borrow_mut(),
                crate::bytesbuild!(b"cvt"),
            );
            let mut ___loggedstep_v_0: bool = true;
            while ___loggedstep_v_0 {
                let raw = base64_decode(::core::slice::from_raw_parts(
                    json_str_ptr(table) as *const u8,
                    json_str_len(table) as usize,
                ))
                .unwrap_or_default();
                let table_length = (raw.len() >> 1_i32) as u32;
                let mut words: Vec<u16> = Vec::with_capacity(table_length as usize);
                let mut j_0: u16 = 0_u16;
                while (j_0 as u32) < table_length {
                    words.push(read_16u(raw.as_ptr().offset(
                        (2_i32 * j_0 as i32) as isize,
                    )));
                    j_0 = j_0.wrapping_add(1);
                }
                t = Some(Box::new(CvtTable { words }));
                ___loggedstep_v_0 = false;
                logger_finish(&mut *options.logger.borrow_mut());
            }
        }
    }
    return t;
}
pub fn otfcc_build_cvt(table: Option<&CvtTable>) -> Option<Buffer> {
    let table = table?;
    let mut buf = Buffer::new();
    for &w in &table.words {
        buf.write_u16be(w);
    }
    Some(buf)
}
