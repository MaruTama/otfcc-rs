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
pub struct MaxpTable {
    pub version: F16Dot16,
    pub num_glyphs: u16,
    pub max_points: u16,
    pub max_contours: u16,
    pub max_composite_points: u16,
    pub max_composite_contours: u16,
    pub max_zones: u16,
    pub max_twilight_points: u16,
    pub max_storage: u16,
    pub max_function_defs: u16,
    pub max_instruction_defs: u16,
    pub max_stack_elements: u16,
    pub max_size_of_instructions: u16,
    pub max_component_elements: u16,
    pub max_component_depth: u16,
}
// Stage 6-4 "Box化": every field is a scalar, so no `Drop` impl is
// needed -- `Box::new` construction is sufficient (`Copy, Clone` stay
// on the struct, same reasoning as `Os2Table`/`HheaTable`/`VheaTable`/
// `HeadTable`). The entire vtable is deleted: grepping the bare
// `TABLE_I_MAXP` identifier confirmed only `.create`/`.free` were ever
// called, both internal to this crate.
// `length` must be *exactly* 32 (version 1.0, full table) or 6 (version 0.5,
// version+numGlyphs only) -- not merely "at least" -- matching the original
// guard. A table that claims to be the 6-byte short form but whose first 4
// bytes happen to spell version 1.0 now correctly fails to parse (dropping
// the whole table) instead of reading the 26 version-1.0-only fields past
// the buffer's actual end, which the original pointer-arithmetic version
// would have done unconditionally once past the length check.
fn parse_maxp(data: &[u8]) -> Result<MaxpTable, ReadError> {
    if data.len() != 32 && data.len() != 6 {
        return Err(ReadError {
            needed: 32,
            available: data.len(),
        });
    }
    let mut r = FontReader::new(data);
    let version = r.i32()? as F16Dot16;
    let num_glyphs = r.u16()?;
    let mut maxp = MaxpTable {
        version,
        num_glyphs,
        max_points: 0,
        max_contours: 0,
        max_composite_points: 0,
        max_composite_contours: 0,
        max_zones: 0,
        max_twilight_points: 0,
        max_storage: 0,
        max_function_defs: 0,
        max_instruction_defs: 0,
        max_stack_elements: 0,
        max_size_of_instructions: 0,
        max_component_elements: 0,
        max_component_depth: 0,
    };
    if version == 0x10000 {
        maxp.max_points = r.u16()?;
        maxp.max_contours = r.u16()?;
        maxp.max_composite_points = r.u16()?;
        maxp.max_composite_contours = r.u16()?;
        maxp.max_zones = r.u16()?;
        maxp.max_twilight_points = r.u16()?;
        maxp.max_storage = r.u16()?;
        maxp.max_function_defs = r.u16()?;
        maxp.max_instruction_defs = r.u16()?;
        maxp.max_stack_elements = r.u16()?;
        maxp.max_size_of_instructions = r.u16()?;
        maxp.max_component_elements = r.u16()?;
        maxp.max_component_depth = r.u16()?;
    }
    Ok(maxp)
}
pub fn otfcc_read_maxp(packet: &Packet, options: &Options) -> Option<Box<MaxpTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_MAXP)?;
    match parse_maxp(&table.data) {
        Ok(maxp) => Some(Box::new(maxp)),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'maxp' corrupted.\n"),
            );
            None
        }
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_maxp(
    table: Option<&MaxpTable>,
    root: &mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t as *const MaxpTable,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"maxp"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut maxp = BuiltValue::new_object(15);
        maxp.push_field(
            b"version",
            BuiltValue::Double(otfcc_from_fixed((*table).version)),
        );
        maxp.push_field(b"numGlyphs", BuiltValue::Int((*table).num_glyphs as i64));
        maxp.push_field(b"maxPoints", BuiltValue::Int((*table).max_points as i64));
        maxp.push_field(b"maxContours", BuiltValue::Int((*table).max_contours as i64));
        maxp.push_field(
            b"maxCompositePoints",
            BuiltValue::Int((*table).max_composite_points as i64),
        );
        maxp.push_field(
            b"maxCompositeContours",
            BuiltValue::Int((*table).max_composite_contours as i64),
        );
        maxp.push_field(b"maxZones", BuiltValue::Int((*table).max_zones as i64));
        maxp.push_field(
            b"maxTwilightPoints",
            BuiltValue::Int((*table).max_twilight_points as i64),
        );
        maxp.push_field(b"maxStorage", BuiltValue::Int((*table).max_storage as i64));
        maxp.push_field(
            b"maxFunctionDefs",
            BuiltValue::Int((*table).max_function_defs as i64),
        );
        maxp.push_field(
            b"maxInstructionDefs",
            BuiltValue::Int((*table).max_instruction_defs as i64),
        );
        maxp.push_field(
            b"maxStackElements",
            BuiltValue::Int((*table).max_stack_elements as i64),
        );
        maxp.push_field(
            b"maxSizeOfInstructions",
            BuiltValue::Int((*table).max_size_of_instructions as i64),
        );
        maxp.push_field(
            b"maxComponentElements",
            BuiltValue::Int((*table).max_component_elements as i64),
        );
        maxp.push_field(
            b"maxComponentDepth",
            BuiltValue::Int((*table).max_component_depth as i64),
        );
        root.push_field(b"maxp", maxp);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_maxp(
    root: &ParsedValue,
    options: &Options,
) -> Option<Box<MaxpTable>> {
    // `.version` carries `init_maxp`'s `0x10000` default through if the
    // "maxp" JSON key is absent (never overwritten below in that case);
    // `.max_size_of_instructions`/`.max_component_elements`/
    // `.max_component_depth` are never set anywhere in this function's
    // body regardless, so their zeroed default matches the old
    // `memset`-based one exactly.
    let mut maxp_val: MaxpTable = ::core::mem::zeroed();
    maxp_val.version = 0x10000_i32 as F16Dot16;
    let mut maxp_box: Box<MaxpTable> = Box::new(maxp_val);
    let maxp: *mut MaxpTable = maxp_box.as_mut() as *mut MaxpTable;
    let table = root.get_typed(b"maxp", JsonType::Object);
    if let Some(table) = table {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"maxp"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*maxp).version = otfcc_to_fixed(table.get_num(b"version"));
            (*maxp).num_glyphs = table.get_num(b"numGlyphs") as u16;
            (*maxp).max_zones = table.get_num(b"maxZones") as u16;
            (*maxp).max_twilight_points = table.get_num(b"maxTwilightPoints") as u16;
            (*maxp).max_storage = table.get_num(b"maxStorage") as u16;
            (*maxp).max_function_defs = table.get_num(b"maxFunctionDefs") as u16;
            (*maxp).max_instruction_defs = table.get_num(b"maxInstructionDefs") as u16;
            (*maxp).max_stack_elements = table.get_num(b"maxStackElements") as u16;
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    return Some(maxp_box);
}
#[allow(improper_ctypes_definitions)]
pub fn otfcc_build_maxp(maxp: Option<&MaxpTable>) -> Option<Buffer> {
    let maxp = maxp?;
    let mut buf = Buffer::new();
    buf.write_u32be(maxp.version as u32);
    buf.write_u16be(maxp.num_glyphs);
    if maxp.version > 0x5000 as F16Dot16 {
        buf.write_u16be(maxp.max_points);
        buf.write_u16be(maxp.max_contours);
        buf.write_u16be(maxp.max_composite_points);
        buf.write_u16be(maxp.max_composite_contours);
        buf.write_u16be(maxp.max_zones);
        buf.write_u16be(maxp.max_twilight_points);
        buf.write_u16be(maxp.max_storage);
        buf.write_u16be(maxp.max_function_defs);
        buf.write_u16be(maxp.max_instruction_defs);
        buf.write_u16be(maxp.max_stack_elements);
        buf.write_u16be(maxp.max_size_of_instructions);
        buf.write_u16be(maxp.max_component_elements);
        buf.write_u16be(maxp.max_component_depth);
    }
    Some(buf)
}

#[cfg(test)]
mod parse_maxp_tests {
    use super::*;

    #[test]
    fn version_0_5_reads_only_num_glyphs() {
        let mut data = vec![0u8; 6];
        data[0..4].copy_from_slice(&0x0000_5000u32.to_be_bytes());
        data[4..6].copy_from_slice(&42u16.to_be_bytes());
        let maxp = parse_maxp(&data).unwrap();
        assert_eq!(maxp.num_glyphs, 42);
        assert_eq!(maxp.max_points, 0);
    }

    #[test]
    fn version_1_0_reads_every_field() {
        let mut data = vec![0u8; 32];
        data[0..4].copy_from_slice(&0x0001_0000u32.to_be_bytes());
        data[4..6].copy_from_slice(&42u16.to_be_bytes());
        data[6..8].copy_from_slice(&99u16.to_be_bytes()); // maxPoints
        let maxp = parse_maxp(&data).unwrap();
        assert_eq!(maxp.num_glyphs, 42);
        assert_eq!(maxp.max_points, 99);
    }

    #[test]
    fn length_between_6_and_32_is_rejected() {
        let data = vec![0u8; 20];
        assert!(parse_maxp(&data).is_err());
    }

    #[test]
    fn six_byte_table_claiming_version_1_0_is_rejected_instead_of_reading_oob() {
        // The version field itself (the first 4 bytes) can claim 1.0 even
        // though the table is only the 6-byte short form -- the original
        // pointer-arithmetic reader would have read the 26 version-1.0-only
        // bytes straight past this 6-byte buffer's end once it took that
        // branch.
        let mut data = vec![0u8; 6];
        data[0..4].copy_from_slice(&0x0001_0000u32.to_be_bytes());
        assert!(parse_maxp(&data).is_err());
    }
}
