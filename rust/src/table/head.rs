#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b, bufwrite64b};
use crate::support::built_json::{
    BuiltValue, json_double_new, json_integer_new, json_object_new, json_object_push,
    otfcc_dump_flags,
};
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_obj_get, json_obj_get_type, json_obj_getnum_fallback, otfcc_parse_flags,
};
use crate::support::primitives::F16Dot16;
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::vendor::json::JsonType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeadTable {
    pub version: F16Dot16,
    pub font_revision: u32,
    pub check_sum_adjustment: u32,
    pub magic_number: u32,
    pub flags: u16,
    pub units_per_em: u16,
    pub created: i64,
    pub modified: i64,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
    pub mac_style: u16,
    pub lowest_rec_ppem: u16,
    pub font_directory_hint: i16,
    pub index_to_loc_format: i16,
    pub glyph_data_format: i16,
}
// Stage 6-4 "Box化": every field is a scalar, so no `Drop` impl is
// needed -- `Box::new` construction is sufficient (`Copy, Clone` stay
// on the struct, same reasoning as `Os2Table`/`HheaTable`/`VheaTable`).
// The entire vtable is deleted: grepping the bare `TABLE_I_HEAD`
// identifier confirmed only `.create`/`.free` were ever called, both
// internal to this crate.
fn parse_head(data: &[u8]) -> Result<HeadTable, ReadError> {
    let mut r = FontReader::new(data);
    Ok(HeadTable {
        version: r.i32()? as F16Dot16,
        font_revision: r.u32()?,
        check_sum_adjustment: r.u32()?,
        magic_number: r.u32()?,
        flags: r.u16()?,
        units_per_em: r.u16()?,
        created: r.u64()? as i64,
        modified: r.u64()? as i64,
        x_min: r.i16()?,
        y_min: r.i16()?,
        x_max: r.i16()?,
        y_max: r.i16()?,
        mac_style: r.u16()?,
        lowest_rec_ppem: r.u16()?,
        font_directory_hint: r.i16()?,
        index_to_loc_format: r.i16()?,
        glyph_data_format: r.i16()?,
    })
}
pub unsafe fn otfcc_read_head(packet: &Packet, options: &Options) -> Option<Box<HeadTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_HEAD)?;
    match parse_head(&table.data) {
        Ok(head) => Some(Box::new(head)),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'head' corrupted.\n"),
            );
            None
        }
    }
}
static HEAD_FLAGS_LABELS: [&::core::ffi::CStr; 15] = [
    c"baselineAtY_0",
    c"lsbAtX_0",
    c"instrMayDependOnPointSize",
    c"alwaysUseIntegerSize",
    c"instrMayAlterAdvanceWidth",
    c"designedForVertical",
    c"_reserved1",
    c"designedForComplexScript",
    c"hasMetamorphosisEffects",
    c"containsStrongRTL",
    c"containsIndicRearrangement",
    c"fontIsLossless",
    c"fontIsConverted",
    c"optimizedForCleartype",
    c"lastResortFont",
];
static MAC_STYLE_LABELS: [&::core::ffi::CStr; 7] = [
    c"bold",
    c"italic",
    c"underline",
    c"outline",
    c"shadow",
    c"condensed",
    c"extended",
];
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_head(
    table: Option<&HeadTable>,
    mut root: *mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t as *const HeadTable,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"head"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut head: *mut BuiltValue = json_object_new(15 as usize);
        json_object_push(
            head,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            head,
            b"fontRevision\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).font_revision as F16Dot16)),
        );
        json_object_push(
            head,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags((*table).flags as ::core::ffi::c_int, &HEAD_FLAGS_LABELS),
        );
        json_object_push(
            head,
            b"unitsPerEm\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).units_per_em as i64),
        );
        json_object_push(
            head,
            b"created\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).created),
        );
        json_object_push(
            head,
            b"modified\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).modified),
        );
        json_object_push(
            head,
            b"xMin\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).x_min as i64),
        );
        json_object_push(
            head,
            b"xMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).x_max as i64),
        );
        json_object_push(
            head,
            b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).y_min as i64),
        );
        json_object_push(
            head,
            b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).y_max as i64),
        );
        json_object_push(
            head,
            b"macStyle\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags((*table).mac_style as ::core::ffi::c_int, &MAC_STYLE_LABELS),
        );
        json_object_push(
            head,
            b"lowestRecPPEM\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).lowest_rec_ppem as i64),
        );
        json_object_push(
            head,
            b"fontDirectoryHint\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).font_directory_hint as i64),
        );
        json_object_push(
            head,
            b"indexToLocFormat\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).index_to_loc_format as i64),
        );
        json_object_push(
            head,
            b"glyphDataFormat\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).glyph_data_format as i64),
        );
        json_object_push(
            root,
            b"head\0" as *const u8 as *const ::core::ffi::c_char,
            head,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_head(
    mut root: *const ParsedValue,
    options: &Options,
) -> Option<Box<HeadTable>> {
    // Reproduces `init_head`'s two non-zero defaults exactly:
    // `.magic_number` is never set anywhere in this function's body below
    // (unlike every other field), so it must carry this default through;
    // `.units_per_em` *is* always overwritten by the `json_obj_getnum_fallback`
    // call below, so its zeroed value here is immediately discarded either way.
    let mut head_val: HeadTable = ::core::mem::zeroed();
    head_val.magic_number = 0x5f0f3cf5 as u32;
    head_val.units_per_em = 1000 as u16;
    let mut head_box: Box<HeadTable> = Box::new(head_val);
    let head: *mut HeadTable = head_box.as_mut() as *mut HeadTable;
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get_type(
        root,
        b"head\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"head"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*head).version = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ));
            (*head).font_revision = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"fontRevision\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            )) as u32;
            (*head).flags = otfcc_parse_flags(
                json_obj_get(table, b"flags\0" as *const u8 as *const ::core::ffi::c_char),
                &HEAD_FLAGS_LABELS,
            ) as u16;
            (*head).units_per_em = json_obj_getnum_fallback(
                table,
                b"unitsPerEm\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*head).created = json_obj_getnum_fallback(
                table,
                b"created\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i64;
            (*head).modified = json_obj_getnum_fallback(
                table,
                b"modified\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i64;
            (*head).x_min = json_obj_getnum_fallback(
                table,
                b"xMin\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).x_max = json_obj_getnum_fallback(
                table,
                b"xMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).y_min = json_obj_getnum_fallback(
                table,
                b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).y_max = json_obj_getnum_fallback(
                table,
                b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).mac_style = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"macStyle\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &MAC_STYLE_LABELS,
            ) as u16;
            (*head).lowest_rec_ppem = json_obj_getnum_fallback(
                table,
                b"lowestRecPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*head).font_directory_hint = json_obj_getnum_fallback(
                table,
                b"fontDirectoryHint\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).index_to_loc_format = json_obj_getnum_fallback(
                table,
                b"indexToLocFormat\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).glyph_data_format = json_obj_getnum_fallback(
                table,
                b"glyphDataFormat\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    return Some(head_box);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_head(head: Option<&HeadTable>) -> *mut Buffer {
    let head = match head {
        Some(h) => h as *const HeadTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*head).version as u32);
    bufwrite32b(buf, (*head).font_revision);
    bufwrite32b(buf, (*head).check_sum_adjustment);
    bufwrite32b(buf, (*head).magic_number);
    bufwrite16b(buf, (*head).flags);
    bufwrite16b(buf, (*head).units_per_em);
    bufwrite64b(buf, (*head).created as u64);
    bufwrite64b(buf, (*head).modified as u64);
    bufwrite16b(buf, (*head).x_min as u16);
    bufwrite16b(buf, (*head).y_min as u16);
    bufwrite16b(buf, (*head).x_max as u16);
    bufwrite16b(buf, (*head).y_max as u16);
    bufwrite16b(buf, (*head).mac_style);
    bufwrite16b(buf, (*head).lowest_rec_ppem);
    bufwrite16b(buf, (*head).font_directory_hint as u16);
    bufwrite16b(buf, (*head).index_to_loc_format as u16);
    bufwrite16b(buf, (*head).glyph_data_format as u16);
    return buf;
}

#[cfg(test)]
mod parse_head_tests {
    use super::*;

    fn well_formed_head() -> Vec<u8> {
        let mut data = vec![0u8; 54];
        data[0..4].copy_from_slice(&0x0001_0000u32.to_be_bytes()); // version
        data[18..20].copy_from_slice(&1000u16.to_be_bytes()); // unitsPerEm
        data[50..52].copy_from_slice(&1i16.to_be_bytes()); // indexToLocFormat
        data
    }

    #[test]
    fn well_formed_54_byte_table_parses_every_field() {
        let head = parse_head(&well_formed_head()).unwrap();
        assert_eq!(head.version, 0x0001_0000);
        assert_eq!(head.units_per_em, 1000);
        assert_eq!(head.index_to_loc_format, 1);
    }

    #[test]
    fn table_one_byte_short_of_54_is_rejected_instead_of_reading_oob() {
        let mut data = well_formed_head();
        data.truncate(53);
        assert!(parse_head(&data).is_err());
    }
}
