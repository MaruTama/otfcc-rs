#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::{ParsedValue, json_obj_get_type, json_obj_getnum};
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
        return Err(ReadError { needed: 32, available: data.len() });
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
pub unsafe fn otfcc_read_maxp(
    packet: &Packet,
    options: &Options,
) -> Option<Box<MaxpTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_MAXP)?;
    match parse_maxp(&table.data) {
        Ok(maxp) => Some(Box::new(maxp)),
        Err(_) => {
            logger_log_sds(
                options.logger,
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
    mut root: *mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t as *const MaxpTable,
        None => return,
    };
    logger_start_sds(
        options.logger,
        crate::bytesbuild!(b"maxp"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut maxp: *mut BuiltValue = json_object_new(15 as usize);
        json_object_push(
            maxp,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            maxp,
            b"numGlyphs\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).num_glyphs as i64),
        );
        json_object_push(
            maxp,
            b"maxPoints\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_points as i64),
        );
        json_object_push(
            maxp,
            b"maxContours\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_contours as i64),
        );
        json_object_push(
            maxp,
            b"maxCompositePoints\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_composite_points as i64),
        );
        json_object_push(
            maxp,
            b"maxCompositeContours\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_composite_contours as i64),
        );
        json_object_push(
            maxp,
            b"maxZones\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_zones as i64),
        );
        json_object_push(
            maxp,
            b"maxTwilightPoints\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_twilight_points as i64),
        );
        json_object_push(
            maxp,
            b"maxStorage\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_storage as i64),
        );
        json_object_push(
            maxp,
            b"maxFunctionDefs\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_function_defs as i64),
        );
        json_object_push(
            maxp,
            b"maxInstructionDefs\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_instruction_defs as i64),
        );
        json_object_push(
            maxp,
            b"maxStackElements\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_stack_elements as i64),
        );
        json_object_push(
            maxp,
            b"maxSizeOfInstructions\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_size_of_instructions as i64),
        );
        json_object_push(
            maxp,
            b"maxComponentElements\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_component_elements as i64),
        );
        json_object_push(
            maxp,
            b"maxComponentDepth\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_component_depth as i64),
        );
        json_object_push(
            root,
            b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
            maxp,
        );
        ___loggedstep_v = false;
        logger_finish(options.logger);
    }
}
pub unsafe fn otfcc_parse_maxp(
    mut root: *const ParsedValue,
    options: &Options,
) -> Option<Box<MaxpTable>> {
    // `.version` carries `init_maxp`'s `0x10000` default through if the
    // "maxp" JSON key is absent (never overwritten below in that case);
    // `.max_size_of_instructions`/`.max_component_elements`/
    // `.max_component_depth` are never set anywhere in this function's
    // body regardless, so their zeroed default matches the old
    // `memset`-based one exactly.
    let mut maxp_val: MaxpTable = ::core::mem::zeroed();
    maxp_val.version = 0x10000 as ::core::ffi::c_int as F16Dot16;
    let mut maxp_box: Box<MaxpTable> = Box::new(maxp_val);
    let maxp: *mut MaxpTable = maxp_box.as_mut() as *mut MaxpTable;
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get_type(
        root,
        b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        logger_start_sds(
            options.logger,
            crate::bytesbuild!(b"maxp"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*maxp).version = otfcc_to_fixed(json_obj_getnum(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            (*maxp).num_glyphs = json_obj_getnum(
                table,
                b"numGlyphs\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).max_zones = json_obj_getnum(
                table,
                b"maxZones\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).max_twilight_points = json_obj_getnum(
                table,
                b"maxTwilightPoints\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).max_storage = json_obj_getnum(
                table,
                b"maxStorage\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).max_function_defs = json_obj_getnum(
                table,
                b"maxFunctionDefs\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).max_instruction_defs = json_obj_getnum(
                table,
                b"maxInstructionDefs\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).max_stack_elements = json_obj_getnum(
                table,
                b"maxStackElements\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            ___loggedstep_v = false;
            logger_finish(
                options.logger
            );
        }
    }
    return Some(maxp_box);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_maxp(
    maxp: Option<&MaxpTable>,
) -> *mut Buffer {
    let maxp = match maxp {
        Some(m) => m as *const MaxpTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*maxp).version as u32);
    bufwrite16b(buf, (*maxp).num_glyphs);
    if (*maxp).version > 0x5000 as F16Dot16 {
        bufwrite16b(buf, (*maxp).max_points);
        bufwrite16b(buf, (*maxp).max_contours);
        bufwrite16b(buf, (*maxp).max_composite_points);
        bufwrite16b(buf, (*maxp).max_composite_contours);
        bufwrite16b(buf, (*maxp).max_zones);
        bufwrite16b(buf, (*maxp).max_twilight_points);
        bufwrite16b(buf, (*maxp).max_storage);
        bufwrite16b(buf, (*maxp).max_function_defs);
        bufwrite16b(buf, (*maxp).max_instruction_defs);
        bufwrite16b(buf, (*maxp).max_stack_elements);
        bufwrite16b(buf, (*maxp).max_size_of_instructions);
        bufwrite16b(buf, (*maxp).max_component_elements);
        bufwrite16b(buf, (*maxp).max_component_depth);
    }
    return buf;
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
