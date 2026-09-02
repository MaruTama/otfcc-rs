#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::support::primitives::F16Dot16;
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::vendor::json::JsonType;

#[derive(Copy, Clone)]
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
pub fn otfcc_read_vhea(packet: &Packet, options: &Options) -> Option<Box<VheaTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_VHEA)?;
    match parse_vhea(&table.data) {
        Ok(vhea) => Some(Box::new(vhea)),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
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
    root: &mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t as *const VheaTable,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"vhea"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut vhea = BuiltValue::new_object(11);
        vhea.push_field(
            b"version",
            BuiltValue::Double(otfcc_from_fixed((*table).version)),
        );
        vhea.push_field(b"ascent", BuiltValue::Int((*table).ascent as i64));
        vhea.push_field(b"descent", BuiltValue::Int((*table).descent as i64));
        vhea.push_field(b"lineGap", BuiltValue::Int((*table).line_gap as i64));
        vhea.push_field(
            b"advanceHeightMax",
            BuiltValue::Int((*table).advance_height_max as i64),
        );
        vhea.push_field(b"minTop", BuiltValue::Int((*table).min_top as i64));
        vhea.push_field(b"minBottom", BuiltValue::Int((*table).min_bottom as i64));
        vhea.push_field(b"yMaxExtent", BuiltValue::Int((*table).y_max_extent as i64));
        vhea.push_field(
            b"caretSlopeRise",
            BuiltValue::Int((*table).caret_slope_rise as i64),
        );
        vhea.push_field(
            b"caretSlopeRun",
            BuiltValue::Int((*table).caret_slope_run as i64),
        );
        vhea.push_field(b"caretOffset", BuiltValue::Int((*table).caret_offset as i64));
        root.push_field(b"vhea", vhea);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_vhea(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<VheaTable>> {
    let mut vhea_box: Option<Box<VheaTable>> = None;
    let vhea: *mut VheaTable;
    let table = unsafe { root.as_ref() }.and_then(|r| r.get_typed(b"vhea", JsonType::Object));
    if let Some(table) = table {
        vhea_box = Some(Box::new(::core::mem::zeroed()));
        vhea = vhea_box.as_deref_mut().unwrap() as *mut VheaTable;
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"vhea"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*vhea).version = otfcc_to_fixed(table.get_num(b"version"));
            (*vhea).ascent = table.get_num(b"ascent") as i16;
            (*vhea).descent = table.get_num(b"descent") as i16;
            (*vhea).line_gap = table.get_num(b"lineGap") as i16;
            (*vhea).advance_height_max = table.get_num(b"advanceHeightMax") as i16;
            (*vhea).min_top = table.get_num(b"minTop") as i16;
            (*vhea).min_bottom = table.get_num(b"minBottom") as i16;
            (*vhea).y_max_extent = table.get_num(b"yMaxExtent") as i16;
            (*vhea).caret_slope_rise = table.get_num(b"caretSlopeRise") as i16;
            (*vhea).caret_slope_run = table.get_num(b"caretSlopeRun") as i16;
            (*vhea).caret_offset = table.get_num(b"caretOffset") as i16;
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    return vhea_box;
}
#[allow(improper_ctypes_definitions)]
pub fn otfcc_build_vhea(vhea: Option<&VheaTable>) -> Option<Buffer> {
    let vhea = vhea?;
    let mut buf = Buffer::new();
    buf.write_u32be(vhea.version as u32);
    buf.write_u16be(vhea.ascent as u16);
    buf.write_u16be(vhea.descent as u16);
    buf.write_u16be(vhea.line_gap as u16);
    buf.write_u16be(vhea.advance_height_max as u16);
    buf.write_u16be(vhea.min_top as u16);
    buf.write_u16be(vhea.min_bottom as u16);
    buf.write_u16be(vhea.y_max_extent as u16);
    buf.write_u16be(vhea.caret_slope_rise as u16);
    buf.write_u16be(vhea.caret_slope_run as u16);
    buf.write_u16be(vhea.caret_offset as u16);
    buf.write_u16be(0_u16);
    buf.write_u16be(0_u16);
    buf.write_u16be(0_u16);
    buf.write_u16be(0_u16);
    buf.write_u16be(0_u16);
    buf.write_u16be(vhea.num_of_long_ver_metrics);
    Some(buf)
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
