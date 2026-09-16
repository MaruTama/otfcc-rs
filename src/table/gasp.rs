use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
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
pub const GASP_DOGRAY: i32 = 0x2_i32;
pub const GASP_GRIDFIT: i32 = 0x1_i32;
pub const GASP_SYMMETRIC_GRIDFIT: i32 = 0x4_i32;
pub const GASP_SYMMETRIC_SMOOTHING: i32 = 0x8_i32;
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
pub fn otfcc_read_gasp(packet: &Packet, options: &Options) -> Option<Box<GaspTable>> {
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
pub fn otfcc_dump_gasp(table: Option<&GaspTable>, root: &mut BuiltValue, options: &Options) {
    let Some(table) = table else {
        return;
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"gasp"),
    );
    let mut t = BuiltValue::new_array(table.records.len());
    for r in &table.records {
        let mut rec = BuiltValue::new_object(5);
        rec.push_field(b"rangeMaxPPEM", BuiltValue::Int(r.range_max_ppem as i64));
        rec.push_field(b"dogray", BuiltValue::Bool(r.dogray));
        rec.push_field(b"gridfit", BuiltValue::Bool(r.gridfit));
        rec.push_field(
            b"symmetric_smoothing",
            BuiltValue::Bool(r.symmetric_smoothing),
        );
        rec.push_field(b"symmetric_gridfit", BuiltValue::Bool(r.symmetric_gridfit));
        t.push_item(rec);
    }
    root.push_field(b"gasp", t);
    logger_finish(&mut *options.logger.borrow_mut());
}
pub fn otfcc_parse_gasp(root: &ParsedValue, options: &Options) -> Option<Box<GaspTable>> {
    let table = root.get_typed(b"gasp", JsonType::Array)?;
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"gasp"),
    );
    let mut gasp = GaspTable {
        version: 1,
        records: Vec::new(),
    };
    if let Some(items) = table.as_array() {
        for r in items {
            if r.as_object().is_some() {
                gasp.records.push(GaspRecord {
                    range_max_ppem: r.get_int_or(b"rangeMaxPPEM", 0xffff_i32) as GlyphSize,
                    dogray: r.get_bool(b"dogray"),
                    gridfit: r.get_bool(b"gridfit"),
                    symmetric_smoothing: r.get_bool(b"symmetric_smoothing"),
                    symmetric_gridfit: r.get_bool(b"symmetric_gridfit"),
                });
            }
        }
    }
    logger_finish(&mut *options.logger.borrow_mut());
    Some(Box::new(gasp))
}
pub fn otfcc_build_gasp(gasp: Option<&GaspTable>) -> Option<Buffer> {
    let gasp = gasp?;
    let mut buf = Buffer::new();
    buf.write_u16be(1_u16);
    buf.write_u16be(gasp.records.len() as u16);
    for r in &gasp.records {
        buf.write_u16be(r.range_max_ppem);
        buf.write_u16be(
            ((if r.dogray { GASP_DOGRAY } else { 0_i32 })
                | (if r.gridfit { GASP_GRIDFIT } else { 0_i32 })
                | (if r.symmetric_gridfit {
                    GASP_SYMMETRIC_GRIDFIT
                } else {
                    0_i32
                })
                | (if r.symmetric_smoothing {
                    GASP_SYMMETRIC_SMOOTHING
                } else {
                    0_i32
                })) as u16,
        );
    }
    Some(buf)
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
