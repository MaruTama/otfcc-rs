#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::{ParsedValue, json_obj_get_type, json_obj_getnum};
use crate::support::binio::{read_16u, read_32s};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, FontFilePointer};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
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
pub unsafe fn otfcc_read_maxp(
    packet: &Packet,
    mut options: *const Options,
) -> Option<Box<MaxpTable>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_MAXP {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    let mut length: u32 = table.length;
                    if length != 32 as u32 && length != 6 as u32 {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"table 'maxp' corrupted.\n"),
                        );
                    } else {
                        let mut maxp_box: Box<MaxpTable> = Box::new(::core::mem::zeroed());
                        let maxp: *mut MaxpTable = maxp_box.as_mut() as *mut MaxpTable;
                        (*maxp).version = read_32s(data as *const u8) as F16Dot16;
                        (*maxp).num_glyphs = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if (*maxp).version == 0x10000 as F16Dot16 {
                            (*maxp).max_points = read_16u(
                                data.offset(6 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            (*maxp).max_contours = read_16u(
                                data.offset(8 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            (*maxp).max_composite_points =
                                read_16u(data.offset(10 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).max_composite_contours =
                                read_16u(data.offset(12 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).max_zones =
                                read_16u(data.offset(14 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).max_twilight_points =
                                read_16u(data.offset(16 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).max_storage =
                                read_16u(data.offset(18 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).max_function_defs =
                                read_16u(data.offset(20 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).max_instruction_defs =
                                read_16u(data.offset(22 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).max_stack_elements =
                                read_16u(data.offset(24 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).max_size_of_instructions =
                                read_16u(data.offset(26 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).max_component_elements =
                                read_16u(data.offset(28 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).max_component_depth =
                                read_16u(data.offset(30 as ::core::ffi::c_int as isize)
                                    as *const u8);
                        } else {
                            (*maxp).max_points = 0 as u16;
                            (*maxp).max_contours = 0 as u16;
                            (*maxp).max_composite_points = 0 as u16;
                            (*maxp).max_composite_contours = 0 as u16;
                            (*maxp).max_zones = 0 as u16;
                            (*maxp).max_twilight_points = 0 as u16;
                            (*maxp).max_storage = 0 as u16;
                            (*maxp).max_function_defs = 0 as u16;
                            (*maxp).max_instruction_defs = 0 as u16;
                            (*maxp).max_stack_elements = 0 as u16;
                            (*maxp).max_size_of_instructions = 0 as u16;
                            (*maxp).max_component_elements = 0 as u16;
                            (*maxp).max_component_depth = 0 as u16;
                        }
                        return Some(maxp_box);
                    }
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_maxp(
    table: Option<&MaxpTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let table = match table {
        Some(t) => t as *const MaxpTable,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
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
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe fn otfcc_parse_maxp(
    mut root: *const ParsedValue,
    mut options: *const Options,
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
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
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
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return Some(maxp_box);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_maxp(
    maxp: Option<&MaxpTable>,
    mut _options: *const Options,
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
