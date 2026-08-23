#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_boolean_new, json_integer_new,
    json_object_new, json_object_push,
};
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getbool,
    json_obj_getint_fallback, json_type_of,
};
use crate::support::primitives::GlyphSize;
use crate::vendor::json::JsonType;

#[derive(Copy, Clone)]
pub struct GaspRecord {
    pub range_max_ppem: GlyphSize,
    pub dogray: bool,
    pub gridfit: bool,
    pub symmetric_smoothing: bool,
    pub symmetric_gridfit: bool,
}
// Stage 6-4 "Box化": every field this struct owns is already a
// `Vec`/scalar, so no `Drop` impl is needed -- `Box::new` construction
// plus the standard drop glue is sufficient. The entire
// `GaspTableElementInterface` vtable is deleted: grepping confirmed only
// `.create`/`.free` were ever called from outside this file.
#[derive(Clone)]
pub struct GaspTable {
    pub version: u16,
    pub records: Vec<GaspRecord>,
}
pub const GASP_DOGRAY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const GASP_GRIDFIT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const GASP_SYMMETRIC_GRIDFIT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const GASP_SYMMETRIC_SMOOTHING: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
fn parse_gasp(data: &[u8]) -> Result<GaspTable, ReadError> {
    let mut r = FontReader::new(data);
    let version = r.u16()?;
    let num_ranges = r.u16()? as usize;
    r.require_room(num_ranges, 4)?;
    let mut records = Vec::with_capacity(num_ranges);
    for _ in 0..num_ranges {
        let range_max_ppem = r.u16()? as GlyphSize;
        let range_gasp_behavior = r.u16()?;
        records.push(GaspRecord {
            range_max_ppem,
            dogray: range_gasp_behavior & GASP_DOGRAY as u16 != 0,
            gridfit: range_gasp_behavior & GASP_GRIDFIT as u16 != 0,
            symmetric_smoothing: range_gasp_behavior & GASP_SYMMETRIC_SMOOTHING as u16 != 0,
            symmetric_gridfit: range_gasp_behavior & GASP_SYMMETRIC_GRIDFIT as u16 != 0,
        });
    }
    Ok(GaspTable { version, records })
}
pub unsafe fn otfcc_read_gasp(packet: &Packet, options: &Options) -> Option<Box<GaspTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_GASP)?;
    match parse_gasp(&table.data) {
        Ok(gasp) => Some(Box::new(gasp)),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'gasp' corrupted.\n"),
            );
            None
        }
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_gasp(
    table: Option<&GaspTable>,
    mut root: *mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"gasp"),
    );
    let records: &Vec<GaspRecord> = &(*table).records;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut t: *mut BuiltValue = json_array_new(records.len());
        let mut j: u16 = 0 as u16;
        while (j as usize) < records.len() {
            let mut rec: *mut BuiltValue = json_object_new(5 as usize);
            json_object_push(
                rec,
                b"rangeMaxPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new(records[j as usize].range_max_ppem as i64),
            );
            json_object_push(
                rec,
                b"dogray\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(records[j as usize].dogray as ::core::ffi::c_int),
            );
            json_object_push(
                rec,
                b"gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(records[j as usize].gridfit as ::core::ffi::c_int),
            );
            json_object_push(
                rec,
                b"symmetric_smoothing\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(records[j as usize].symmetric_smoothing as ::core::ffi::c_int),
            );
            json_object_push(
                rec,
                b"symmetric_gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(records[j as usize].symmetric_gridfit as ::core::ffi::c_int),
            );
            json_array_push(t, rec);
            j = j.wrapping_add(1);
        }
        json_object_push(
            root,
            b"gasp\0" as *const u8 as *const ::core::ffi::c_char,
            t,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_gasp(
    mut root: *const ParsedValue,
    options: &Options,
) -> Option<Box<GaspTable>> {
    let mut gasp: Option<Box<GaspTable>> = None;
    let mut table: *const ParsedValue = ::core::ptr::null();
    table = json_obj_get_type(
        root,
        b"gasp\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if !table.is_null() {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"gasp"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            gasp = Some(Box::new(GaspTable {
                version: 1,
                records: Vec::new(),
            }));
            let mut j: u16 = 0 as u16;
            while (j as ::core::ffi::c_uint) < json_arr_len(table) {
                let mut r: *const ParsedValue = json_arr_at(table, j as u32);
                if !(r.is_null() || json_type_of(r) != JsonType::Object) {
                    let mut record: GaspRecord = GaspRecord {
                        range_max_ppem: 0,
                        dogray: false,
                        gridfit: false,
                        symmetric_smoothing: false,
                        symmetric_gridfit: false,
                    };
                    record.range_max_ppem = json_obj_getint_fallback(
                        r,
                        b"rangeMaxPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                        0xffff as i32,
                    ) as GlyphSize;
                    record.dogray =
                        json_obj_getbool(r, b"dogray\0" as *const u8 as *const ::core::ffi::c_char);
                    record.gridfit = json_obj_getbool(
                        r,
                        b"gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    record.symmetric_smoothing = json_obj_getbool(
                        r,
                        b"symmetric_smoothing\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    record.symmetric_gridfit = json_obj_getbool(
                        r,
                        b"symmetric_gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    gasp.as_mut().unwrap().records.push(record);
                }
                j = j.wrapping_add(1);
            }
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    return gasp;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_gasp(gasp: Option<&GaspTable>) -> *mut Buffer {
    let gasp = match gasp {
        Some(g) => g,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    let records: &Vec<GaspRecord> = &(*gasp).records;
    bufwrite16b(buf, 1 as u16);
    bufwrite16b(buf, records.len() as u16);
    let mut j: u16 = 0 as u16;
    while (j as usize) < records.len() {
        let mut r: *const GaspRecord = &records[j as usize];
        bufwrite16b(buf, (*r).range_max_ppem as u16);
        bufwrite16b(
            buf,
            ((if (*r).dogray as ::core::ffi::c_int != 0 {
                GASP_DOGRAY
            } else {
                0 as ::core::ffi::c_int
            }) | (if (*r).gridfit as ::core::ffi::c_int != 0 {
                GASP_GRIDFIT
            } else {
                0 as ::core::ffi::c_int
            }) | (if (*r).symmetric_gridfit as ::core::ffi::c_int != 0 {
                GASP_SYMMETRIC_GRIDFIT
            } else {
                0 as ::core::ffi::c_int
            }) | (if (*r).symmetric_smoothing as ::core::ffi::c_int != 0 {
                GASP_SYMMETRIC_SMOOTHING
            } else {
                0 as ::core::ffi::c_int
            })) as u16,
        );
        j = j.wrapping_add(1);
    }
    return buf;
}

#[cfg(test)]
mod parse_gasp_tests {
    use super::*;

    #[test]
    fn well_formed_table_parses_flags_from_behavior_bits() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // version
        data.extend_from_slice(&1u16.to_be_bytes()); // numRanges
        data.extend_from_slice(&65535u16.to_be_bytes()); // rangeMaxPPEM
        data.extend_from_slice(
            &(GASP_GRIDFIT as u16 | GASP_SYMMETRIC_SMOOTHING as u16).to_be_bytes(),
        );
        let gasp = parse_gasp(&data).unwrap();
        assert_eq!(gasp.records.len(), 1);
        assert_eq!(gasp.records[0].range_max_ppem, 65535);
        assert!(gasp.records[0].gridfit);
        assert!(gasp.records[0].symmetric_smoothing);
        assert!(!gasp.records[0].dogray);
    }

    #[test]
    fn num_ranges_large_enough_to_overflow_the_multiplication_errs() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&0xFFFFu16.to_be_bytes()); // numRanges, far more than the data holds
        assert!(parse_gasp(&data).is_err());
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(parse_gasp(&[0x00, 0x01]).is_err());
    }
}
