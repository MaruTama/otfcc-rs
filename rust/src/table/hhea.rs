#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b};
use crate::support::built_json::{
    BuiltValue, json_double_new, json_integer_new, json_object_new, json_object_push,
};
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::{ParsedValue, json_obj_get_type, json_obj_getnum_fallback};
use crate::support::primitives::F16Dot16;
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::vendor::json::JsonType;

#[derive(Copy, Clone)]
pub struct HheaTable {
    pub version: F16Dot16,
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
    pub advance_width_max: u16,
    pub min_left_side_bearing: i16,
    pub min_right_side_bearing: i16,
    pub x_max_extent: i16,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    pub reserved: [i16; 4],
    pub metric_data_format: i16,
    pub number_of_metrics: u16,
}
// Stage 6-4 "Box化": every field is a scalar/fixed-size array, so no
// `Drop` impl is needed -- `Box::new` construction is sufficient
// (`Copy, Clone` stay on the struct, same reasoning as `Os2Table`). The
// entire vtable is deleted: grepping the bare `TABLE_I_HHEA` identifier
// confirmed only `.create`/`.free` were ever called, both internal to
// this crate.
fn parse_hhea(data: &[u8]) -> Result<HheaTable, ReadError> {
    let mut r = FontReader::new(data);
    Ok(HheaTable {
        version: r.i32()? as F16Dot16,
        ascender: r.i16()?,
        descender: r.i16()?,
        line_gap: r.i16()?,
        advance_width_max: r.u16()?,
        min_left_side_bearing: r.i16()?,
        min_right_side_bearing: r.i16()?,
        x_max_extent: r.i16()?,
        caret_slope_rise: r.i16()?,
        caret_slope_run: r.i16()?,
        caret_offset: r.i16()?,
        reserved: [r.i16()?, r.i16()?, r.i16()?, r.i16()?],
        metric_data_format: r.i16()?,
        number_of_metrics: r.u16()?,
    })
}
pub unsafe fn otfcc_read_hhea(packet: &Packet, options: &Options) -> Option<Box<HheaTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_HHEA)?;
    match parse_hhea(&table.data) {
        Ok(hhea) => Some(Box::new(hhea)),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'hhea' corrupted.\n"),
            );
            None
        }
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_hhea(
    table: Option<&HheaTable>,
    root: *mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t as *const HheaTable,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"hhea"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let hhea: *mut BuiltValue = json_object_new(13_usize);
        json_object_push(
            hhea,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            hhea,
            b"ascender\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ascender as i64),
        );
        json_object_push(
            hhea,
            b"descender\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).descender as i64),
        );
        json_object_push(
            hhea,
            b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).line_gap as i64),
        );
        json_object_push(
            hhea,
            b"advanceWidthMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).advance_width_max as i64),
        );
        json_object_push(
            hhea,
            b"minLeftSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).min_left_side_bearing as i64),
        );
        json_object_push(
            hhea,
            b"minRightSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).min_right_side_bearing as i64),
        );
        json_object_push(
            hhea,
            b"xMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).x_max_extent as i64),
        );
        json_object_push(
            hhea,
            b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caret_slope_rise as i64),
        );
        json_object_push(
            hhea,
            b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caret_slope_run as i64),
        );
        json_object_push(
            hhea,
            b"caretOffset\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caret_offset as i64),
        );
        json_object_push(
            root,
            b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
            hhea,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_hhea(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<HheaTable>> {
    let mut hhea_val: HheaTable = ::core::mem::zeroed();
    hhea_val.version = 0x10000 as ::core::ffi::c_int as F16Dot16;
    let mut hhea_box: Box<HheaTable> = Box::new(hhea_val);
    let hhea: *mut HheaTable = hhea_box.as_mut() as *mut HheaTable;
    let table: *const ParsedValue;
    table = json_obj_get_type(
        root,
        b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"hhea"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*hhea).version = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ));
            (*hhea).ascender = json_obj_getnum_fallback(
                table,
                b"ascender\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).descender = json_obj_getnum_fallback(
                table,
                b"descender\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).line_gap = json_obj_getnum_fallback(
                table,
                b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).advance_width_max = json_obj_getnum_fallback(
                table,
                b"advanceWidthMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*hhea).min_left_side_bearing = json_obj_getnum_fallback(
                table,
                b"minLeftSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).min_right_side_bearing = json_obj_getnum_fallback(
                table,
                b"minRightSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).x_max_extent = json_obj_getnum_fallback(
                table,
                b"xMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caret_slope_rise = json_obj_getnum_fallback(
                table,
                b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caret_slope_run = json_obj_getnum_fallback(
                table,
                b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caret_offset = json_obj_getnum_fallback(
                table,
                b"caretOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    return Some(hhea_box);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_hhea(hhea: Option<&HheaTable>) -> *mut Buffer {
    let hhea = match hhea {
        Some(h) => h as *const HheaTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*hhea).version as u32);
    bufwrite16b(buf, (*hhea).ascender as u16);
    bufwrite16b(buf, (*hhea).descender as u16);
    bufwrite16b(buf, (*hhea).line_gap as u16);
    bufwrite16b(buf, (*hhea).advance_width_max);
    bufwrite16b(buf, (*hhea).min_left_side_bearing as u16);
    bufwrite16b(buf, (*hhea).min_right_side_bearing as u16);
    bufwrite16b(buf, (*hhea).x_max_extent as u16);
    bufwrite16b(buf, (*hhea).caret_slope_rise as u16);
    bufwrite16b(buf, (*hhea).caret_slope_run as u16);
    bufwrite16b(buf, (*hhea).caret_offset as u16);
    bufwrite16b(
        buf,
        (*hhea).reserved[0 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(
        buf,
        (*hhea).reserved[1 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(
        buf,
        (*hhea).reserved[2 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(
        buf,
        (*hhea).reserved[3 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(buf, 0_u16);
    bufwrite16b(buf, (*hhea).number_of_metrics);
    return buf;
}

#[cfg(test)]
mod parse_hhea_tests {
    use super::*;

    fn well_formed_hhea() -> Vec<u8> {
        let mut data = vec![0u8; 36];
        data[0..4].copy_from_slice(&0x0001_0000u32.to_be_bytes()); // version
        data[4..6].copy_from_slice(&900i16.to_be_bytes()); // ascender
        data[34..36].copy_from_slice(&5u16.to_be_bytes()); // numberOfMetrics
        data
    }

    #[test]
    fn well_formed_36_byte_table_parses_every_field() {
        let hhea = parse_hhea(&well_formed_hhea()).unwrap();
        assert_eq!(hhea.version, 0x0001_0000);
        assert_eq!(hhea.ascender, 900);
        assert_eq!(hhea.number_of_metrics, 5);
    }

    #[test]
    fn table_one_byte_short_of_36_is_rejected_instead_of_reading_oob() {
        let mut data = well_formed_hhea();
        data.truncate(35);
        assert!(parse_hhea(&data).is_err());
    }
}
