#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::{ParsedValue, otfcc_parse_flags};
use crate::support::primitives::F16Dot16;
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::vendor::json::JsonType;
#[derive(Copy, Clone)]
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
pub fn otfcc_read_head(packet: &Packet, options: &Options) -> Option<Box<HeadTable>> {
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
    root: &mut BuiltValue,
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
        let mut head = BuiltValue::new_object(15);
        head.push_field(
            b"version",
            BuiltValue::Double(otfcc_from_fixed((*table).version)),
        );
        head.push_field(
            b"fontRevision",
            BuiltValue::Double(otfcc_from_fixed((*table).font_revision as F16Dot16)),
        );
        head.push_field(
            b"flags",
            BuiltValue::dump_flags((*table).flags as i32, &HEAD_FLAGS_LABELS),
        );
        head.push_field(b"unitsPerEm", BuiltValue::Int((*table).units_per_em as i64));
        head.push_field(b"created", BuiltValue::Int((*table).created));
        head.push_field(b"modified", BuiltValue::Int((*table).modified));
        head.push_field(b"xMin", BuiltValue::Int((*table).x_min as i64));
        head.push_field(b"xMax", BuiltValue::Int((*table).x_max as i64));
        head.push_field(b"yMin", BuiltValue::Int((*table).y_min as i64));
        head.push_field(b"yMax", BuiltValue::Int((*table).y_max as i64));
        head.push_field(
            b"macStyle",
            BuiltValue::dump_flags((*table).mac_style as i32, &MAC_STYLE_LABELS),
        );
        head.push_field(
            b"lowestRecPPEM",
            BuiltValue::Int((*table).lowest_rec_ppem as i64),
        );
        head.push_field(
            b"fontDirectoryHint",
            BuiltValue::Int((*table).font_directory_hint as i64),
        );
        head.push_field(
            b"indexToLocFormat",
            BuiltValue::Int((*table).index_to_loc_format as i64),
        );
        head.push_field(
            b"glyphDataFormat",
            BuiltValue::Int((*table).glyph_data_format as i64),
        );
        root.push_field(b"head", head);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_head(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<HeadTable>> {
    // Reproduces `init_head`'s two non-zero defaults exactly:
    // `.magic_number` is never set anywhere in this function's body below
    // (unlike every other field), so it must carry this default through;
    // `.units_per_em` *is* always overwritten by the `get_num_or` call
    // below, so its zeroed value here is immediately discarded either way.
    let mut head_val: HeadTable = ::core::mem::zeroed();
    head_val.magic_number = 0x5f0f3cf5_u32;
    head_val.units_per_em = 1000_u16;
    let mut head_box: Box<HeadTable> = Box::new(head_val);
    let Some(table) = root.as_ref().and_then(|r| r.get_typed(b"head", JsonType::Object)) else {
        return Some(head_box);
    };
    let head: *mut HeadTable = head_box.as_mut() as *mut HeadTable;
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"head"),
    );
    (*head).version = otfcc_to_fixed(table.get_num_or(b"version", 0.0));
    (*head).font_revision = otfcc_to_fixed(table.get_num_or(b"fontRevision", 0.0)) as u32;
    (*head).flags = otfcc_parse_flags(
        table
            .get(b"flags")
            .map_or(::core::ptr::null(), |v| v as *const ParsedValue),
        &HEAD_FLAGS_LABELS,
    ) as u16;
    (*head).units_per_em = table.get_num_or(b"unitsPerEm", 0.0) as u16;
    (*head).created = table.get_num_or(b"created", 0.0) as i64;
    (*head).modified = table.get_num_or(b"modified", 0.0) as i64;
    (*head).x_min = table.get_num_or(b"xMin", 0.0) as i16;
    (*head).x_max = table.get_num_or(b"xMax", 0.0) as i16;
    (*head).y_min = table.get_num_or(b"yMin", 0.0) as i16;
    (*head).y_max = table.get_num_or(b"yMax", 0.0) as i16;
    (*head).mac_style = otfcc_parse_flags(
        table
            .get(b"macStyle")
            .map_or(::core::ptr::null(), |v| v as *const ParsedValue),
        &MAC_STYLE_LABELS,
    ) as u16;
    (*head).lowest_rec_ppem = table.get_num_or(b"lowestRecPPEM", 0.0) as u16;
    (*head).font_directory_hint = table.get_num_or(b"fontDirectoryHint", 0.0) as i16;
    (*head).index_to_loc_format = table.get_num_or(b"indexToLocFormat", 0.0) as i16;
    (*head).glyph_data_format = table.get_num_or(b"glyphDataFormat", 0.0) as i16;
    logger_finish(&mut *options.logger.borrow_mut());
    Some(head_box)
}
#[allow(improper_ctypes_definitions)]
pub fn otfcc_build_head(head: Option<&HeadTable>) -> Option<Buffer> {
    let head = head?;
    let mut buf = Buffer::new();
    buf.write_u32be(head.version as u32);
    buf.write_u32be(head.font_revision);
    buf.write_u32be(head.check_sum_adjustment);
    buf.write_u32be(head.magic_number);
    buf.write_u16be(head.flags);
    buf.write_u16be(head.units_per_em);
    buf.write_u64be(head.created as u64);
    buf.write_u64be(head.modified as u64);
    buf.write_u16be(head.x_min as u16);
    buf.write_u16be(head.y_min as u16);
    buf.write_u16be(head.x_max as u16);
    buf.write_u16be(head.y_max as u16);
    buf.write_u16be(head.mac_style);
    buf.write_u16be(head.lowest_rec_ppem);
    buf.write_u16be(head.font_directory_hint as u16);
    buf.write_u16be(head.index_to_loc_format as u16);
    buf.write_u16be(head.glyph_data_format as u16);
    Some(buf)
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
