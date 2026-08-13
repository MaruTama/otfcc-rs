#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::binio::{read_16u, read_32u, read_32s, read_64u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, FontFilePointer};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::parsed_json::{ParsedValue, json_obj_get, json_obj_get_type, json_obj_getnum_fallback, otfcc_parse_flags};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b, bufwrite64b};
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::support::built_json::{BuiltValue, json_double_new, json_integer_new, json_object_new, json_object_push, otfcc_dump_flags};
#[derive(Copy, Clone)]
#[repr(C)]
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
pub unsafe extern "C" fn otfcc_read_head(
    packet: Packet,
    mut options: *const Options,
) -> Option<Box<HeadTable>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_HEAD {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if length < 54 as u32 {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"table 'head' corrupted.\n"),
                        );
                    } else {
                        let mut head_box: Box<HeadTable> = Box::new(::core::mem::zeroed());
                        let head: *mut HeadTable = head_box.as_mut() as *mut HeadTable;
                        (*head).version = read_32s(data as *const u8) as F16Dot16;
                        (*head).font_revision = read_32u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).check_sum_adjustment = read_32u(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).magic_number = read_32u(
                            data.offset(12 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).flags = read_16u(
                            data.offset(16 as ::core::ffi::c_int as isize) as *const u8
                        );
                        (*head).units_per_em = read_16u(
                            data.offset(18 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).created = read_64u(
                            data.offset(20 as ::core::ffi::c_int as isize) as *const u8
                        ) as i64;
                        (*head).modified = read_64u(
                            data.offset(28 as ::core::ffi::c_int as isize) as *const u8
                        ) as i64;
                        (*head).x_min = read_16u(
                            data.offset(36 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).y_min = read_16u(
                            data.offset(38 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).x_max = read_16u(
                            data.offset(40 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).y_max = read_16u(
                            data.offset(42 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).mac_style = read_16u(
                            data.offset(44 as ::core::ffi::c_int as isize) as *const u8
                        );
                        (*head).lowest_rec_ppem = read_16u(
                            data.offset(46 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).font_directory_hint = read_16u(
                            data.offset(48 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*head).index_to_loc_format = read_16u(
                            data.offset(50 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*head).glyph_data_format = read_16u(
                            data.offset(52 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        return Some(head_box);
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
pub unsafe extern "C" fn otfcc_dump_head(
    table: Option<&HeadTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let table = match table {
        Some(t) => t as *const HeadTable,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::bytesbuild!(b"head"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut head: *mut BuiltValue = json_object_new(15 as usize);
        json_object_push(
            head,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            head,
            b"fontRevision\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).font_revision as F16Dot16)),
        );
        json_object_push(
            head,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).flags as ::core::ffi::c_int,
                &HEAD_FLAGS_LABELS,
            ),
        );
        json_object_push(
            head,
            b"unitsPerEm\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).units_per_em as i64),
        );
        json_object_push(
            head,
            b"created\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).created),
        );
        json_object_push(
            head,
            b"modified\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).modified),
        );
        json_object_push(
            head,
            b"xMin\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).x_min as i64),
        );
        json_object_push(
            head,
            b"xMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).x_max as i64),
        );
        json_object_push(
            head,
            b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).y_min as i64),
        );
        json_object_push(
            head,
            b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).y_max as i64),
        );
        json_object_push(
            head,
            b"macStyle\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).mac_style as ::core::ffi::c_int,
                &MAC_STYLE_LABELS,
            ),
        );
        json_object_push(
            head,
            b"lowestRecPPEM\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).lowest_rec_ppem as i64),
        );
        json_object_push(
            head,
            b"fontDirectoryHint\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).font_directory_hint as i64),
        );
        json_object_push(
            head,
            b"indexToLocFormat\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).index_to_loc_format as i64),
        );
        json_object_push(
            head,
            b"glyphDataFormat\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).glyph_data_format as i64),
        );
        json_object_push(
            root,
            b"head\0" as *const u8 as *const ::core::ffi::c_char,
            head,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_head(
    mut root: *const ParsedValue,
    mut options: *const Options,
) -> Option<Box<HeadTable>> {
    // Reproduces `init_head`'s two non-zero defaults exactly:
    // `.magic_number` is never set anywhere in this function's body below
    // (unlike every other field), so it must carry this default through;
    // `.units_per_em` *is* always overwritten by the `json_obj_getnum_fallback`
    // call below, so its zeroed value here is immediately discarded either way.
    let mut head_val: HeadTable = ::core::mem::zeroed();
    head_val.magic_number = 0x5f0f3cf5 as u32;
    head_val.units_per_em = 1000 as u16;
    let mut head_box: Box<HeadTable> = Box::new(head_val);
    let head: *mut HeadTable = head_box.as_mut() as *mut HeadTable;
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get_type(
        root,
        b"head\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::bytesbuild!(b"head"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*head).version = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ));
            (*head).font_revision = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"fontRevision\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            )) as u32;
            (*head).flags = otfcc_parse_flags(
                json_obj_get(table, b"flags\0" as *const u8 as *const ::core::ffi::c_char),
                &HEAD_FLAGS_LABELS,
            ) as u16;
            (*head).units_per_em = json_obj_getnum_fallback(
                table,
                b"unitsPerEm\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*head).created = json_obj_getnum_fallback(
                table,
                b"created\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i64;
            (*head).modified = json_obj_getnum_fallback(
                table,
                b"modified\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i64;
            (*head).x_min = json_obj_getnum_fallback(
                table,
                b"xMin\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).x_max = json_obj_getnum_fallback(
                table,
                b"xMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).y_min = json_obj_getnum_fallback(
                table,
                b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).y_max = json_obj_getnum_fallback(
                table,
                b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).mac_style = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"macStyle\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &MAC_STYLE_LABELS,
            ) as u16;
            (*head).lowest_rec_ppem = json_obj_getnum_fallback(
                table,
                b"lowestRecPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*head).font_directory_hint = json_obj_getnum_fallback(
                table,
                b"fontDirectoryHint\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).index_to_loc_format = json_obj_getnum_fallback(
                table,
                b"indexToLocFormat\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).glyph_data_format = json_obj_getnum_fallback(
                table,
                b"glyphDataFormat\0" as *const u8 as *const ::core::ffi::c_char,
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
    return Some(head_box);
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_head(
    head: Option<&HeadTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let head = match head {
        Some(h) => h as *const HeadTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*head).version as u32);
    bufwrite32b(buf, (*head).font_revision);
    bufwrite32b(buf, (*head).check_sum_adjustment);
    bufwrite32b(buf, (*head).magic_number);
    bufwrite16b(buf, (*head).flags);
    bufwrite16b(buf, (*head).units_per_em);
    bufwrite64b(buf, (*head).created as u64);
    bufwrite64b(buf, (*head).modified as u64);
    bufwrite16b(buf, (*head).x_min as u16);
    bufwrite16b(buf, (*head).y_min as u16);
    bufwrite16b(buf, (*head).x_max as u16);
    bufwrite16b(buf, (*head).y_max as u16);
    bufwrite16b(buf, (*head).mac_style);
    bufwrite16b(buf, (*head).lowest_rec_ppem);
    bufwrite16b(buf, (*head).font_directory_hint as u16);
    bufwrite16b(buf, (*head).index_to_loc_format as u16);
    bufwrite16b(buf, (*head).glyph_data_format as u16);
    return buf;
}
