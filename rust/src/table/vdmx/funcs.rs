#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::vendor::json::JsonType;

use crate::bk::bkgraph::bk_build_block_no_minimize;
use crate::support::built_json::BuiltValue;
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
pub fn otfcc_read_vdmx(packet: &Packet, options: &Options) -> Option<Box<VdmxTable>> {
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
    root: &mut BuiltValue,
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
        let mut _vdmx = BuiltValue::new_object(2);
        _vdmx.push_field(b"version", BuiltValue::Int((*vdmx).version as i64));
        let ratios: &Vec<VdmxRatioRange> = &(*vdmx).ratios;
        let mut _ratios = BuiltValue::new_array(ratios.len());
        for rr in ratios.iter() {
            let mut _rr = BuiltValue::new_object(5);
            _rr.push_field(b"bCharset", BuiltValue::Int(rr.b_charset as i64));
            _rr.push_field(b"xRatio", BuiltValue::Int(rr.x_ratio as i64));
            _rr.push_field(b"yStartRatio", BuiltValue::Int(rr.y_start_ratio as i64));
            _rr.push_field(b"yEndRatio", BuiltValue::Int(rr.y_end_ratio as i64));
            let mut _records = BuiltValue::new_array(rr.records.len());
            for r in rr.records.iter() {
                let mut _r = BuiltValue::new_object(3);
                _r.push_field(b"yPelHeight", BuiltValue::Int(r.y_pel_height as i64));
                _r.push_field(b"yMax", BuiltValue::Int(r.y_max as i64));
                _r.push_field(b"yMin", BuiltValue::Int(r.y_min as i64));
                _records.push_item(_r);
            }
            _rr.push_field(b"records", _records);
            _ratios.push_item(_rr);
        }
        _vdmx.push_field(b"ratios", _ratios);
        root.push_field(b"VDMX", _vdmx);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_vdmx(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<VdmxTable>> {
    let vdmx_dump = root.as_ref().and_then(|r| r.get_typed(b"VDMX", JsonType::Object))?;
    let mut vdmx: Box<VdmxTable> = Box::new(VdmxTable {
        version: 0,
        ratios: Vec::new(),
    });
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"VDMX"),
    );
    vdmx.version = vdmx_dump.get_num(b"version") as u16;
    if let Some(ratio_items) = vdmx_dump
        .get_typed(b"ratios", JsonType::Array)
        .and_then(ParsedValue::as_array)
    {
        for _rr in ratio_items {
            if _rr.as_object().is_none() {
                continue;
            }
            let mut r: VdmxRatioRange = VdmxRatioRange {
                b_charset: _rr.get_num(b"bCharset") as u8,
                x_ratio: _rr.get_num(b"xRatio") as u8,
                y_start_ratio: _rr.get_num(b"yStartRatio") as u8,
                y_end_ratio: _rr.get_num(b"yEndRatio") as u8,
                records: Vec::new(),
            };
            // A `records` array with the right type is required for this
            // ratio range to be kept at all -- matches the pre-migration
            // control flow exactly, `r` (and the fields already read above)
            // is discarded when it's absent.
            if let Some(record_items) = _rr
                .get_typed(b"records", JsonType::Array)
                .and_then(ParsedValue::as_array)
            {
                for _r in record_items {
                    if _r.as_object().is_none() {
                        continue;
                    }
                    r.records.push(VdmxRecord {
                        y_pel_height: _r.get_num(b"yPelHeight") as u16,
                        y_max: _r.get_num(b"yMax") as i16,
                        y_min: _r.get_num(b"yMin") as i16,
                    });
                }
                vdmx.ratios.push(r);
            }
        }
    }
    logger_finish(&mut *options.logger.borrow_mut());
    Some(vdmx)
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_vdmx(vdmx: Option<&VdmxTable>) -> Option<Buffer> {
    let vdmx = vdmx?;
    let ratios: &Vec<VdmxRatioRange> = &(*vdmx).ratios;
    if ratios.is_empty() {
        return None;
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(
            BkCellType::B16,
            ((*vdmx).version as i32) as u32,
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
                    bk_int(BkCellType::B8, (rr.b_charset as i32) as u32),
                    bk_int(BkCellType::B8, (rr.x_ratio as i32) as u32),
                    bk_int(
                        BkCellType::B8,
                        (rr.y_start_ratio as i32) as u32,
                    ),
                    bk_int(
                        BkCellType::B8,
                        (rr.y_end_ratio as i32) as u32,
                    ),
                ],
            );
            keep = (keep == 0) as i32 as usize;
        }
        keep = (keep == 0) as i32 as usize;
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
                    if startsz as i32 > r.y_pel_height as i32 {
                        startsz = r.y_pel_height;
                    }
                    if (endsz as i32) < r.y_pel_height as i32 {
                        endsz = r.y_pel_height;
                    }
                    keep_1 = (keep_1 == 0) as i32 as usize;
                }
                keep_1 = (keep_1 == 0) as i32 as usize;
                __caryll_index_1 = __caryll_index_1.wrapping_add(1);
            }
            let group: *mut BkBlock = bk_new_block(&[
                bk_int(BkCellType::B16, (rr_0.records.len()) as u32),
                bk_int(BkCellType::B8, (startsz as i32) as u32),
                bk_int(BkCellType::B8, (endsz as i32) as u32),
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
                                (r_0.y_pel_height as i32) as u32,
                            ),
                            bk_int(BkCellType::B16, (r_0.y_max as i32) as u32),
                            bk_int(BkCellType::B16, (r_0.y_min as i32) as u32),
                        ],
                    );
                    keep_2 = (keep_2 == 0) as i32 as usize;
                }
                keep_2 = (keep_2 == 0) as i32 as usize;
                __caryll_index_2 = __caryll_index_2.wrapping_add(1);
            }
            bk_push(root, &[bk_ptr(BkCellType::P16, group)]);
            keep_0 = (keep_0 == 0) as i32 as usize;
        }
        keep_0 = (keep_0 == 0) as i32 as usize;
        __caryll_index_0 = __caryll_index_0.wrapping_add(1);
    }
    Some(bk_build_block_no_minimize(root))
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
