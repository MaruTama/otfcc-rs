#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getnum, json_type_of,
};
use crate::vendor::json::JsonType;

use crate::bk::bkgraph::bk_build_block_no_minimize;
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new,
    json_object_push,
};
use crate::table::vdmx::types::{VdmxRatioRange, VdmxRecord, VdmxTable};
// `group_offset` (read from the per-ratio offset table) used to be handed
// straight to `data.offset()` with no check against the table's actual
// length at all -- not even the wrapping-arithmetic-defeated kind of guard
// `table/meta/read.rs` had, just no guard whatsoever. A crafted
// `group_offset` pointing anywhere past the table (or a `recs` count
// implying entries past it) read arbitrarily far out of bounds. Every
// offset below -- the ratio range, the offset table, and the group itself
// -- now goes through `FontReader::at`, so an out-of-range offset fails the
// read instead of dereferencing it.
fn parse_vdmx(data: &[u8]) -> Result<VdmxTable, ReadError> {
    let mut r = FontReader::new(data);
    let version = r.u16()?;
    r.skip(2)?; // numRecs: unused, each group carries its own record count
    let num_ratios = r.u16()? as usize;
    r.require_room(num_ratios, 6)?; // 4-byte ratio range + 2-byte offset, per ratio
    let mut ratios = Vec::with_capacity(num_ratios);
    for g in 0..num_ratios {
        let ratio_range_offset = 6 + 4 * g;
        let offset_offset = 6 + 4 * num_ratios + 2 * g;
        let mut rr = FontReader::new(data).at(ratio_range_offset)?;
        let b_charset = rr.u8()?;
        let x_ratio = rr.u8()?;
        let y_start_ratio = rr.u8()?;
        let y_end_ratio = rr.u8()?;
        let group_offset = FontReader::new(data).at(offset_offset)?.u16()? as usize;
        let mut gr = FontReader::new(data).at(group_offset)?;
        let recs = gr.u16()?;
        gr.skip(2)?; // startSize, endSize: unused
        let mut records = Vec::with_capacity(recs as usize);
        for _ in 0..recs {
            records.push(VdmxRecord {
                y_pel_height: gr.u16()?,
                y_max: gr.i16()?,
                y_min: gr.i16()?,
            });
        }
        ratios.push(VdmxRatioRange {
            b_charset,
            x_ratio,
            y_start_ratio,
            y_end_ratio,
            records,
        });
    }
    Ok(VdmxTable { version, ratios })
}
pub unsafe fn otfcc_read_vdmx(packet: &Packet, options: &Options) -> Option<Box<VdmxTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_VDMX)?;
    match parse_vdmx(&table.data) {
        Ok(vdmx) => Some(Box::new(vdmx)),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"Table 'VDMX' corrupted.\n"),
            );
            None
        }
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_vdmx(
    vdmx: Option<&VdmxTable>,
    root: *mut BuiltValue,
    options: &Options,
) {
    let vdmx = match vdmx {
        Some(v) => v,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"VDMX"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _vdmx: *mut BuiltValue = json_object_new(2_usize);
        json_object_push(
            _vdmx,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*vdmx).version as i64),
        );
        let ratios: &Vec<VdmxRatioRange> = &(*vdmx).ratios;
        let mut _ratios: *mut BuiltValue = json_array_new(ratios.len());
        let mut __caryll_index: usize = 0_usize;
        let mut keep: usize = 1_usize;
        while keep != 0 && __caryll_index < ratios.len() {
            let rr: &VdmxRatioRange = &ratios[__caryll_index];
            while keep != 0 {
                let mut _rr: *mut BuiltValue = json_object_new(5_usize);
                json_object_push(
                    _rr,
                    b"bCharset\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).b_charset as i64),
                );
                json_object_push(
                    _rr,
                    b"xRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).x_ratio as i64),
                );
                json_object_push(
                    _rr,
                    b"yStartRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).y_start_ratio as i64),
                );
                json_object_push(
                    _rr,
                    b"yEndRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).y_end_ratio as i64),
                );
                let mut _records: *mut BuiltValue = json_array_new(rr.records.len());
                let mut __caryll_index_0: usize = 0_usize;
                let mut keep_0: usize = 1_usize;
                while keep_0 != 0 && __caryll_index_0 < rr.records.len() {
                    let r: &VdmxRecord = &rr.records[__caryll_index_0];
                    while keep_0 != 0 {
                        let mut _r: *mut BuiltValue = json_object_new(3_usize);
                        json_object_push(
                            _r,
                            b"yPelHeight\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).y_pel_height as i64),
                        );
                        json_object_push(
                            _r,
                            b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).y_max as i64),
                        );
                        json_object_push(
                            _r,
                            b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).y_min as i64),
                        );
                        json_array_push(_records, _r);
                        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    }
                    keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    __caryll_index_0 = __caryll_index_0.wrapping_add(1);
                }
                json_object_push(
                    _rr,
                    b"records\0" as *const u8 as *const ::core::ffi::c_char,
                    _records,
                );
                json_array_push(_ratios, _rr);
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            _vdmx,
            b"ratios\0" as *const u8 as *const ::core::ffi::c_char,
            _ratios,
        );
        json_object_push(
            root,
            b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
            _vdmx,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_vdmx(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<VdmxTable>> {
    let mut _vdmx: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    _vdmx = json_obj_get_type(
        root,
        b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _vdmx.is_null() {
        return None;
    }
    let mut vdmx: Box<VdmxTable> = Box::new(VdmxTable {
        version: 0,
        ratios: Vec::new(),
    });
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"VDMX"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        (*vdmx).version = json_obj_getnum(
            _vdmx,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u16;
        let mut _ratios: *const ParsedValue = json_obj_get_type(
            _vdmx,
            b"ratios\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        );
        let mut j: usize = 0_usize;
        while j < json_arr_len(_ratios) as usize {
            let mut _rr: *const ParsedValue = json_arr_at(_ratios, j as u32);
            if !(_rr.is_null() || json_type_of(_rr) != JsonType::Object) {
                let mut r: VdmxRatioRange = VdmxRatioRange {
                    b_charset: 0,
                    x_ratio: 0,
                    y_start_ratio: 0,
                    y_end_ratio: 0,
                    records: Vec::new(),
                };
                r.b_charset = json_obj_getnum(
                    _rr,
                    b"bCharset\0" as *const u8 as *const ::core::ffi::c_char,
                ) as u8;
                r.x_ratio =
                    json_obj_getnum(_rr, b"xRatio\0" as *const u8 as *const ::core::ffi::c_char)
                        as u8;
                r.y_start_ratio = json_obj_getnum(
                    _rr,
                    b"yStartRatio\0" as *const u8 as *const ::core::ffi::c_char,
                ) as u8;
                r.y_end_ratio = json_obj_getnum(
                    _rr,
                    b"yEndRatio\0" as *const u8 as *const ::core::ffi::c_char,
                ) as u8;
                let mut _records: *const ParsedValue = json_obj_get_type(
                    _rr,
                    b"records\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                );
                if !_records.is_null() {
                    let mut j_0: usize = 0_usize;
                    while j_0 < json_arr_len(_records) as usize {
                        let mut _r: *const ParsedValue = json_arr_at(_records, j_0 as u32);
                        if !(_r.is_null() || json_type_of(_r) != JsonType::Object) {
                            r.records.push(VdmxRecord {
                                y_pel_height: json_obj_getnum(
                                    _r,
                                    b"yPelHeight\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as u16,
                                y_max: json_obj_getnum(
                                    _r,
                                    b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as i16,
                                y_min: json_obj_getnum(
                                    _r,
                                    b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as i16,
                            });
                        }
                        j_0 = j_0.wrapping_add(1);
                    }
                    (*vdmx).ratios.push(r);
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    return Some(vdmx);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_vdmx(vdmx: Option<&VdmxTable>) -> *mut Buffer {
    let vdmx = match vdmx {
        Some(v) => v,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let ratios: &Vec<VdmxRatioRange> = &(*vdmx).ratios;
    if ratios.is_empty() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(
            BkCellType::B16,
            ((*vdmx).version as ::core::ffi::c_int) as u32,
        ),
        bk_int(BkCellType::B16, (ratios.len()) as u32),
        bk_int(BkCellType::B16, (ratios.len()) as u32),
    ]);
    let mut __caryll_index: usize = 0_usize;
    let mut keep: usize = 1_usize;
    while keep != 0 && __caryll_index < ratios.len() {
        let rr: &VdmxRatioRange = &ratios[__caryll_index];
        while keep != 0 {
            bk_push(
                root,
                &[
                    bk_int(BkCellType::B8, (rr.b_charset as ::core::ffi::c_int) as u32),
                    bk_int(BkCellType::B8, (rr.x_ratio as ::core::ffi::c_int) as u32),
                    bk_int(
                        BkCellType::B8,
                        (rr.y_start_ratio as ::core::ffi::c_int) as u32,
                    ),
                    bk_int(
                        BkCellType::B8,
                        (rr.y_end_ratio as ::core::ffi::c_int) as u32,
                    ),
                ],
            );
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut __caryll_index_0: usize = 0_usize;
    let mut keep_0: usize = 1_usize;
    while keep_0 != 0 && __caryll_index_0 < ratios.len() {
        let rr_0: &VdmxRatioRange = &ratios[__caryll_index_0];
        while keep_0 != 0 {
            let mut startsz: u16 = 0xffff_u16;
            let mut endsz: u16 = 0_u16;
            let mut __caryll_index_1: usize = 0_usize;
            let mut keep_1: usize = 1_usize;
            while keep_1 != 0 && __caryll_index_1 < rr_0.records.len() {
                let r: &VdmxRecord = &rr_0.records[__caryll_index_1];
                while keep_1 != 0 {
                    if startsz as ::core::ffi::c_int > r.y_pel_height as ::core::ffi::c_int {
                        startsz = r.y_pel_height;
                    }
                    if (endsz as ::core::ffi::c_int) < r.y_pel_height as ::core::ffi::c_int {
                        endsz = r.y_pel_height;
                    }
                    keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
                }
                keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
                __caryll_index_1 = __caryll_index_1.wrapping_add(1);
            }
            let group: *mut BkBlock = bk_new_block(&[
                bk_int(BkCellType::B16, (rr_0.records.len()) as u32),
                bk_int(BkCellType::B8, (startsz as ::core::ffi::c_int) as u32),
                bk_int(BkCellType::B8, (endsz as ::core::ffi::c_int) as u32),
            ]);
            let mut __caryll_index_2: usize = 0_usize;
            let mut keep_2: usize = 1_usize;
            while keep_2 != 0 && __caryll_index_2 < rr_0.records.len() {
                let r_0: &VdmxRecord = &rr_0.records[__caryll_index_2];
                while keep_2 != 0 {
                    bk_push(
                        group,
                        &[
                            bk_int(
                                BkCellType::B16,
                                (r_0.y_pel_height as ::core::ffi::c_int) as u32,
                            ),
                            bk_int(BkCellType::B16, (r_0.y_max as ::core::ffi::c_int) as u32),
                            bk_int(BkCellType::B16, (r_0.y_min as ::core::ffi::c_int) as u32),
                        ],
                    );
                    keep_2 = (keep_2 == 0) as ::core::ffi::c_int as usize;
                }
                keep_2 = (keep_2 == 0) as ::core::ffi::c_int as usize;
                __caryll_index_2 = __caryll_index_2.wrapping_add(1);
            }
            bk_push(root, &[bk_ptr(BkCellType::P16, group)]);
            keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
        }
        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
        __caryll_index_0 = __caryll_index_0.wrapping_add(1);
    }
    return bk_build_block_no_minimize(root);
}

#[cfg(test)]
mod parse_vdmx_tests {
    use super::*;

    fn well_formed_one_ratio_vdmx() -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes()); // version
        data.extend_from_slice(&1u16.to_be_bytes()); // numRecs (unused by the reader)
        data.extend_from_slice(&1u16.to_be_bytes()); // numRatios
        data.extend_from_slice(&[1, 1, 0, 0]); // ratio range: bCharSet, xRatio, yStartRatio, yEndRatio
        data.extend_from_slice(&12u16.to_be_bytes()); // offset table: group at byte 12
        assert_eq!(data.len(), 12);
        data.extend_from_slice(&1u16.to_be_bytes()); // group.recs
        data.extend_from_slice(&0u16.to_be_bytes()); // group.startSize/endSize
        data.extend_from_slice(&12u16.to_be_bytes()); // record.yPelHeight
        data.extend_from_slice(&900i16.to_be_bytes()); // record.yMax
        data.extend_from_slice(&(-200i16).to_be_bytes()); // record.yMin
        data
    }

    #[test]
    fn well_formed_table_follows_the_group_offset() {
        let vdmx = parse_vdmx(&well_formed_one_ratio_vdmx()).unwrap();
        assert_eq!(vdmx.ratios.len(), 1);
        assert_eq!(vdmx.ratios[0].records.len(), 1);
        assert_eq!(vdmx.ratios[0].records[0].y_pel_height, 12);
        assert_eq!(vdmx.ratios[0].records[0].y_max, 900);
        assert_eq!(vdmx.ratios[0].records[0].y_min, -200);
    }

    #[test]
    fn group_offset_past_the_table_end_errs_instead_of_reading_oob() {
        // The original had no bounds check on `group_offset` at all -- it
        // was handed straight to pointer arithmetic. This is the case that
        // used to read arbitrarily far past the table.
        let mut data = well_formed_one_ratio_vdmx();
        let bogus_offset = (data.len() as u16) + 1000;
        data[10..12].copy_from_slice(&bogus_offset.to_be_bytes());
        assert!(parse_vdmx(&data).is_err());
    }

    #[test]
    fn recs_implying_entries_past_the_table_end_errs_instead_of_reading_oob() {
        let mut data = well_formed_one_ratio_vdmx();
        let recs_field_start = 12;
        data[recs_field_start..recs_field_start + 2].copy_from_slice(&9000u16.to_be_bytes());
        assert!(parse_vdmx(&data).is_err());
    }

    #[test]
    fn num_ratios_large_enough_to_overflow_the_multiplication_errs() {
        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&0xFFFFu16.to_be_bytes()); // numRatios
        assert!(parse_vdmx(&data).is_err());
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(parse_vdmx(&[0x00, 0x00]).is_err());
    }
}
