#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::{ParsedValue, json_obj_get_type, json_obj_getnum_fallback};
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
pub struct HheaTable {
    pub version: F16Dot16,
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
    pub advance_width_max: u16,
    pub min_left_side_bearing: i16,
    pub min_right_side_bearing: i16,
    pub x_max_extent: i16,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    pub reserved: [i16; 4],
    pub metric_data_format: i16,
    pub number_of_metrics: u16,
}
// Stage 6-4 "Box化": every field is a scalar/fixed-size array, so no
// `Drop` impl is needed -- `Box::new` construction is sufficient
// (`Copy, Clone` stay on the struct, same reasoning as `Os2Table`). The
// entire vtable is deleted: grepping the bare `TABLE_I_HHEA` identifier
// confirmed only `.create`/`.free` were ever called, both internal to
// this crate.
pub unsafe extern "C" fn otfcc_read_hhea(
    packet: &Packet,
    mut options: *const Options,
) -> Option<Box<HheaTable>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_HHEA {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    let mut length: u32 = table.length;
                    if length < 36 as u32 {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"table 'hhea' corrupted.\n"),
                        );
                    } else {
                        let mut hhea_box: Box<HheaTable> = Box::new(::core::mem::zeroed());
                        let hhea: *mut HheaTable = hhea_box.as_mut() as *mut HheaTable;
                        (*hhea).version = read_32s(data as *const u8) as F16Dot16;
                        (*hhea).ascender = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*hhea).descender = read_16u(
                            data.offset(6 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*hhea).line_gap = read_16u(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*hhea).advance_width_max = read_16u(
                            data.offset(10 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*hhea).min_left_side_bearing = read_16u(
                            data.offset(12 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).min_right_side_bearing = read_16u(
                            data.offset(14 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).x_max_extent = read_16u(
                            data.offset(16 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).caret_slope_rise = read_16u(
                            data.offset(18 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).caret_slope_run = read_16u(
                            data.offset(20 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).caret_offset = read_16u(
                            data.offset(22 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).reserved[0 as ::core::ffi::c_int as usize] = read_16u(
                            data.offset(24 as ::core::ffi::c_int as isize) as *const u8,
                        )
                            as i16;
                        (*hhea).reserved[1 as ::core::ffi::c_int as usize] = read_16u(
                            data.offset(26 as ::core::ffi::c_int as isize) as *const u8,
                        )
                            as i16;
                        (*hhea).reserved[2 as ::core::ffi::c_int as usize] = read_16u(
                            data.offset(28 as ::core::ffi::c_int as isize) as *const u8,
                        )
                            as i16;
                        (*hhea).reserved[3 as ::core::ffi::c_int as usize] = read_16u(
                            data.offset(30 as ::core::ffi::c_int as isize) as *const u8,
                        )
                            as i16;
                        (*hhea).metric_data_format = read_16u(
                            data.offset(32 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).number_of_metrics = read_16u(
                            data.offset(34 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        return Some(hhea_box);
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
pub unsafe extern "C" fn otfcc_dump_hhea(
    table: Option<&HheaTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let table = match table {
        Some(t) => t as *const HheaTable,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::bytesbuild!(b"hhea"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut hhea: *mut BuiltValue = json_object_new(13 as usize);
        json_object_push(
            hhea,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            hhea,
            b"ascender\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ascender as i64),
        );
        json_object_push(
            hhea,
            b"descender\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).descender as i64),
        );
        json_object_push(
            hhea,
            b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).line_gap as i64),
        );
        json_object_push(
            hhea,
            b"advanceWidthMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).advance_width_max as i64),
        );
        json_object_push(
            hhea,
            b"minLeftSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).min_left_side_bearing as i64),
        );
        json_object_push(
            hhea,
            b"minRightSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).min_right_side_bearing as i64),
        );
        json_object_push(
            hhea,
            b"xMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).x_max_extent as i64),
        );
        json_object_push(
            hhea,
            b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caret_slope_rise as i64),
        );
        json_object_push(
            hhea,
            b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caret_slope_run as i64),
        );
        json_object_push(
            hhea,
            b"caretOffset\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caret_offset as i64),
        );
        json_object_push(
            root,
            b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
            hhea,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_hhea(
    mut root: *const ParsedValue,
    mut options: *const Options,
) -> Option<Box<HheaTable>> {
    let mut hhea_val: HheaTable = ::core::mem::zeroed();
    hhea_val.version = 0x10000 as ::core::ffi::c_int as F16Dot16;
    let mut hhea_box: Box<HheaTable> = Box::new(hhea_val);
    let hhea: *mut HheaTable = hhea_box.as_mut() as *mut HheaTable;
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get_type(
        root,
        b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::bytesbuild!(b"hhea"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*hhea).version = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ));
            (*hhea).ascender = json_obj_getnum_fallback(
                table,
                b"ascender\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).descender = json_obj_getnum_fallback(
                table,
                b"descender\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).line_gap = json_obj_getnum_fallback(
                table,
                b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).advance_width_max = json_obj_getnum_fallback(
                table,
                b"advanceWidthMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*hhea).min_left_side_bearing = json_obj_getnum_fallback(
                table,
                b"minLeftSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).min_right_side_bearing = json_obj_getnum_fallback(
                table,
                b"minRightSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).x_max_extent = json_obj_getnum_fallback(
                table,
                b"xMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caret_slope_rise = json_obj_getnum_fallback(
                table,
                b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caret_slope_run = json_obj_getnum_fallback(
                table,
                b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caret_offset = json_obj_getnum_fallback(
                table,
                b"caretOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return Some(hhea_box);
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_hhea(
    hhea: Option<&HheaTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let hhea = match hhea {
        Some(h) => h as *const HheaTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*hhea).version as u32);
    bufwrite16b(buf, (*hhea).ascender as u16);
    bufwrite16b(buf, (*hhea).descender as u16);
    bufwrite16b(buf, (*hhea).line_gap as u16);
    bufwrite16b(buf, (*hhea).advance_width_max);
    bufwrite16b(buf, (*hhea).min_left_side_bearing as u16);
    bufwrite16b(buf, (*hhea).min_right_side_bearing as u16);
    bufwrite16b(buf, (*hhea).x_max_extent as u16);
    bufwrite16b(buf, (*hhea).caret_slope_rise as u16);
    bufwrite16b(buf, (*hhea).caret_slope_run as u16);
    bufwrite16b(buf, (*hhea).caret_offset as u16);
    bufwrite16b(
        buf,
        (*hhea).reserved[0 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(
        buf,
        (*hhea).reserved[1 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(
        buf,
        (*hhea).reserved[2 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(
        buf,
        (*hhea).reserved[3 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, (*hhea).number_of_metrics);
    return buf;
}
