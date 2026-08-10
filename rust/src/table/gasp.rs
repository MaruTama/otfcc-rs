#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::binio::{read_16u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphSize, TableId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::json_funcs::{json_obj_get_type, json_obj_getbool, json_obj_getint_fallback};
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_boolean_new, json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsempty};

#[derive(Copy, Clone)]
#[repr(C)]
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
pub const GASP_DOGRAY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const GASP_GRIDFIT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const GASP_SYMMETRIC_GRIDFIT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const GASP_SYMMETRIC_SMOOTHING: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub unsafe extern "C" fn otfcc_read_gasp(
    packet: Packet,
    mut options: *const Options,
) -> Option<Box<GaspTable>> {
    let mut num_ranges: TableId = 0;
    let mut gasp: Option<Box<GaspTable>> = None;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1734439792i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if !(length < 4 as u32) {
                        let version = read_16u(data as *const u8);
                        gasp = Some(Box::new(GaspTable { version, records: Vec::new() }));
                        num_ranges = read_16u(
                            data.offset(2 as ::core::ffi::c_int as isize) as *const u8
                        ) as TableId;
                        if !(length
                            < (4 as ::core::ffi::c_int
                                + num_ranges as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as u32)
                        {
                            let mut j: u32 = 0 as u32;
                            while j < num_ranges as u32 {
                                let mut record: GaspRecord = GaspRecord {
                                    range_max_ppem: 0,
                                    dogray: false,
                                    gridfit: false,
                                    symmetric_smoothing: false,
                                    symmetric_gridfit: false,
                                };
                                record.range_max_ppem = read_16u(
                                    data.offset(4 as ::core::ffi::c_int as isize)
                                        .offset(j.wrapping_mul(4 as u32) as isize)
                                        as *const u8,
                                )
                                    as GlyphSize;
                                let mut range_gasp_behavior: u16 = read_16u(
                                    data.offset(4 as ::core::ffi::c_int as isize)
                                        .offset(j.wrapping_mul(4 as u32) as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                record.dogray =
                                    range_gasp_behavior as ::core::ffi::c_int & GASP_DOGRAY != 0;
                                record.gridfit =
                                    range_gasp_behavior as ::core::ffi::c_int & GASP_GRIDFIT != 0;
                                record.symmetric_smoothing = range_gasp_behavior
                                    as ::core::ffi::c_int
                                    & GASP_SYMMETRIC_SMOOTHING
                                    != 0;
                                record.symmetric_gridfit = range_gasp_behavior as ::core::ffi::c_int
                                    & GASP_SYMMETRIC_GRIDFIT
                                    != 0;
                                gasp.as_mut().unwrap().records.push(record);
                                j = j.wrapping_add(1);
                            }
                            return gasp;
                        }
                    }
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(sdsempty(), b"table 'gasp' corrupted.\n"),
                    );
                    gasp = None;
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
pub unsafe extern "C" fn otfcc_dump_gasp(
    table: Option<&GaspTable>,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"gasp"),
    );
    let records: &Vec<GaspRecord> = &(*table).records;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut t: *mut JsonValue = json_array_new(records.len());
        let mut j: u16 = 0 as u16;
        while (j as usize) < records.len() {
            let mut rec: *mut JsonValue = json_object_new(5 as usize);
            json_object_push(
                rec,
                b"rangeMaxPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new(records[j as usize].range_max_ppem as i64),
            );
            json_object_push(
                rec,
                b"dogray\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(records[j as usize].dogray as ::core::ffi::c_int),
            );
            json_object_push(
                rec,
                b"gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(records[j as usize].gridfit as ::core::ffi::c_int),
            );
            json_object_push(
                rec,
                b"symmetric_smoothing\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    records[j as usize].symmetric_smoothing as ::core::ffi::c_int,
                ),
            );
            json_object_push(
                rec,
                b"symmetric_gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    records[j as usize].symmetric_gridfit as ::core::ffi::c_int,
                ),
            );
            json_array_push(t, rec);
            j = j.wrapping_add(1);
        }
        json_object_push(
            root,
            b"gasp\0" as *const u8 as *const ::core::ffi::c_char,
            t,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_gasp(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> Option<Box<GaspTable>> {
    let mut gasp: Option<Box<GaspTable>> = None;
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"gasp\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if !table.is_null() {
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"gasp"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            gasp = Some(Box::new(GaspTable { version: 1, records: Vec::new() }));
            let mut j: u16 = 0 as u16;
            while (j as ::core::ffi::c_uint) < (*table).u.array.length {
                let mut r: *mut JsonValue =
                    *(*table).u.array.values.offset(j as isize) as *mut JsonValue;
                if !(r.is_null()
                    || (*r).type_0 != JsonType::Object)
                {
                    let mut record: GaspRecord = GaspRecord {
                        range_max_ppem: 0,
                        dogray: false,
                        gridfit: false,
                        symmetric_smoothing: false,
                        symmetric_gridfit: false,
                    };
                    record.range_max_ppem = json_obj_getint_fallback(
                        r,
                        b"rangeMaxPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                        0xffff as i32,
                    ) as GlyphSize;
                    record.dogray =
                        json_obj_getbool(r, b"dogray\0" as *const u8 as *const ::core::ffi::c_char);
                    record.gridfit = json_obj_getbool(
                        r,
                        b"gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    record.symmetric_smoothing = json_obj_getbool(
                        r,
                        b"symmetric_smoothing\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    record.symmetric_gridfit = json_obj_getbool(
                        r,
                        b"symmetric_gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    gasp.as_mut().unwrap().records.push(record);
                }
                j = j.wrapping_add(1);
            }
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return gasp;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_gasp(
    gasp: Option<&GaspTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let gasp = match gasp {
        Some(g) => g,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    let records: &Vec<GaspRecord> = &(*gasp).records;
    bufwrite16b(buf, 1 as u16);
    bufwrite16b(buf, records.len() as u16);
    let mut j: u16 = 0 as u16;
    while (j as usize) < records.len() {
        let mut r: *const GaspRecord = &records[j as usize];
        bufwrite16b(buf, (*r).range_max_ppem as u16);
        bufwrite16b(
            buf,
            ((if (*r).dogray as ::core::ffi::c_int != 0 {
                GASP_DOGRAY
            } else {
                0 as ::core::ffi::c_int
            }) | (if (*r).gridfit as ::core::ffi::c_int != 0 {
                GASP_GRIDFIT
            } else {
                0 as ::core::ffi::c_int
            }) | (if (*r).symmetric_gridfit as ::core::ffi::c_int != 0 {
                GASP_SYMMETRIC_GRIDFIT
            } else {
                0 as ::core::ffi::c_int
            }) | (if (*r).symmetric_smoothing as ::core::ffi::c_int != 0 {
                GASP_SYMMETRIC_SMOOTHING
            } else {
                0 as ::core::ffi::c_int
            })) as u16,
        );
        j = j.wrapping_add(1);
    }
    return buf;
}
