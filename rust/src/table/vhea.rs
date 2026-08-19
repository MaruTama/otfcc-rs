#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::{ParsedValue, json_obj_get_type, json_obj_getnum, json_obj_getnum_fallback};
use crate::support::font_reader::{FontReader, ReadError};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, logger_finish, logger_log_sds, logger_start_sds};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b};
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::support::built_json::{BuiltValue, json_double_new, json_integer_new, json_object_new, json_object_push};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VheaTable {
    pub version: F16Dot16,
    pub ascent: i16,
    pub descent: i16,
    pub line_gap: i16,
    pub advance_height_max: i16,
    pub min_top: i16,
    pub min_bottom: i16,
    pub y_max_extent: i16,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    pub dummy0: i16,
    pub dummy1: i16,
    pub dummy2: i16,
    pub dummy3: i16,
    pub metric_data_format: i16,
    pub num_of_long_ver_metrics: u16,
}
// Stage 6-4 "Box化": every field is a scalar, so no `Drop` impl is
// needed -- `Box::new` construction is sufficient (`Copy, Clone` stay
// on the struct, same reasoning as `Os2Table`/`HheaTable`). The entire
// vtable is deleted: grepping the bare `TABLE_I_VHEA` identifier
// confirmed only `.create`/`.free` were ever called, both internal to
// this crate.
// `dummy0..3` and `metric_data_format` are never read from the table data --
// only zeroed -- matching the original, which set them directly rather than
// reading bytes 24..34; only `num_of_long_ver_metrics` at offset 34 follows
// that gap.
fn parse_vhea(data: &[u8]) -> Result<VheaTable, ReadError> {
    let mut r = FontReader::new(data);
    let version = r.i32()? as F16Dot16;
    let ascent = r.i16()?;
    let descent = r.i16()?;
    let line_gap = r.i16()?;
    let advance_height_max = r.i16()?;
    let min_top = r.i16()?;
    let min_bottom = r.i16()?;
    let y_max_extent = r.i16()?;
    let caret_slope_rise = r.i16()?;
    let caret_slope_run = r.i16()?;
    let caret_offset = r.i16()?;
    r.skip(10)?;
    let num_of_long_ver_metrics = r.u16()?;
    Ok(VheaTable {
        version,
        ascent,
        descent,
        line_gap,
        advance_height_max,
        min_top,
        min_bottom,
        y_max_extent,
        caret_slope_rise,
        caret_slope_run,
        caret_offset,
        dummy0: 0,
        dummy1: 0,
        dummy2: 0,
        dummy3: 0,
        metric_data_format: 0,
        num_of_long_ver_metrics,
    })
}
pub unsafe fn otfcc_read_vhea(
    packet: &Packet,
    mut options: *const Options,
) -> Option<Box<VheaTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_VHEA)?;
    match parse_vhea(&table.data) {
        Ok(vhea) => Some(Box::new(vhea)),
        Err(_) => {
            logger_log_sds(
                (*options).logger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"Table 'vhea' corrupted."),
            );
            None
        }
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_vhea(
    table: Option<&VheaTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let table = match table {
        Some(t) => t as *const VheaTable,
        None => return,
    };
    let mut vhea: *mut BuiltValue = json_object_new(11 as usize);
    logger_start_sds(
        (*options).logger,
        crate::bytesbuild!(b"vhea"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        json_object_push(
            vhea,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            vhea,
            b"ascent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ascent as i64),
        );
        json_object_push(
            vhea,
            b"descent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).descent as i64),
        );
        json_object_push(
            vhea,
            b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).line_gap as i64),
        );
        json_object_push(
            vhea,
            b"advanceHeightMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).advance_height_max as i64),
        );
        json_object_push(
            vhea,
            b"minTop\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).min_top as i64),
        );
        json_object_push(
            vhea,
            b"minBottom\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).min_bottom as i64),
        );
        json_object_push(
            vhea,
            b"yMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).y_max_extent as i64),
        );
        json_object_push(
            vhea,
            b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caret_slope_rise as i64),
        );
        json_object_push(
            vhea,
            b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caret_slope_run as i64),
        );
        json_object_push(
            vhea,
            b"caretOffset\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caret_offset as i64),
        );
        json_object_push(
            root,
            b"vhea\0" as *const u8 as *const ::core::ffi::c_char,
            vhea,
        );
        ___loggedstep_v = false;
        logger_finish((*options).logger);
    }
}
pub unsafe fn otfcc_parse_vhea(
    mut root: *const ParsedValue,
    mut options: *const Options,
) -> Option<Box<VheaTable>> {
    let mut vhea_box: Option<Box<VheaTable>> = None;
    let mut vhea: *mut VheaTable = ::core::ptr::null_mut::<VheaTable>();
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get_type(
        root,
        b"vhea\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        vhea_box = Some(Box::new(::core::mem::zeroed()));
        vhea = vhea_box.as_deref_mut().unwrap() as *mut VheaTable;
        logger_start_sds(
            (*options).logger,
            crate::bytesbuild!(b"vhea"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*vhea).version = otfcc_to_fixed(json_obj_getnum(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            (*vhea).ascent = json_obj_getnum_fallback(
                table,
                b"ascent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).descent = json_obj_getnum_fallback(
                table,
                b"descent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).line_gap = json_obj_getnum_fallback(
                table,
                b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).advance_height_max = json_obj_getnum_fallback(
                table,
                b"advanceHeightMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).min_top = json_obj_getnum_fallback(
                table,
                b"minTop\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).min_bottom = json_obj_getnum_fallback(
                table,
                b"minBottom\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).y_max_extent = json_obj_getnum_fallback(
                table,
                b"yMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).caret_slope_rise = json_obj_getnum_fallback(
                table,
                b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).caret_slope_run = json_obj_getnum_fallback(
                table,
                b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).caret_offset = json_obj_getnum_fallback(
                table,
                b"caretOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            ___loggedstep_v = false;
            logger_finish(
                (*options).logger
            );
        }
    }
    return vhea_box;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_vhea(
    vhea: Option<&VheaTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let vhea = match vhea {
        Some(v) => v as *const VheaTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*vhea).version as u32);
    bufwrite16b(buf, (*vhea).ascent as u16);
    bufwrite16b(buf, (*vhea).descent as u16);
    bufwrite16b(buf, (*vhea).line_gap as u16);
    bufwrite16b(buf, (*vhea).advance_height_max as u16);
    bufwrite16b(buf, (*vhea).min_top as u16);
    bufwrite16b(buf, (*vhea).min_bottom as u16);
    bufwrite16b(buf, (*vhea).y_max_extent as u16);
    bufwrite16b(buf, (*vhea).caret_slope_rise as u16);
    bufwrite16b(buf, (*vhea).caret_slope_run as u16);
    bufwrite16b(buf, (*vhea).caret_offset as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, (*vhea).num_of_long_ver_metrics);
    return buf;
}

#[cfg(test)]
mod parse_vhea_tests {
    use super::*;

    fn well_formed_vhea() -> Vec<u8> {
        let mut data = vec![0u8; 36];
        data[0..4].copy_from_slice(&0x0001_0000u32.to_be_bytes()); // version
        data[4..6].copy_from_slice(&950i16.to_be_bytes()); // ascent
        data[34..36].copy_from_slice(&7u16.to_be_bytes()); // numOfLongVerMetrics
        data
    }

    #[test]
    fn well_formed_36_byte_table_parses_and_zeroes_the_unread_reserved_fields() {
        let vhea = parse_vhea(&well_formed_vhea()).unwrap();
        assert_eq!(vhea.version, 0x0001_0000);
        assert_eq!(vhea.ascent, 950);
        assert_eq!(vhea.num_of_long_ver_metrics, 7);
        assert_eq!(vhea.dummy0, 0);
        assert_eq!(vhea.metric_data_format, 0);
    }

    #[test]
    fn table_one_byte_short_of_36_is_rejected_instead_of_reading_oob() {
        let mut data = well_formed_vhea();
        data.truncate(35);
        assert!(parse_vhea(&data).is_err());
    }
}
