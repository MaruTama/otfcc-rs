#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{calloc, free};
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
#[derive(Clone)]
pub struct GaspTable {
    pub version: u16,
    pub records: Vec<GaspRecord>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GaspTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GaspTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut GaspTable, *const GaspTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GaspTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GaspTable>,
    pub free: Option<unsafe extern "C" fn(*mut GaspTable) -> ()>,
}
pub const GASP_DOGRAY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const GASP_GRIDFIT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const GASP_SYMMETRIC_GRIDFIT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const GASP_SYMMETRIC_SMOOTHING: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn init_gasp(mut gasp: *mut GaspTable) {
    (*gasp).version = 1 as u16;
    (*gasp).records = Vec::new();
}
#[inline]
unsafe extern "C" fn dispose_gasp(mut gasp: *mut GaspTable) {
    (*gasp).records = Vec::new();
}
#[inline]
unsafe extern "C" fn table_gasp_create() -> *mut GaspTable {
    // `calloc`, not `malloc`: `table_gasp_init` assigns straight into
    // `(*x).records` (`= Vec::new()`), which drops whatever was already
    // there first. Zeroed memory makes that a no-op (`Vec`'s drop is a no-op
    // when capacity is 0); uninitialized memory makes it read a garbage
    // capacity and attempt to deallocate through a garbage pointer.
    let mut x: *mut GaspTable =
        calloc(1, ::core::mem::size_of::<GaspTable>() as usize) as *mut GaspTable;
    table_gasp_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_gasp_dispose(mut x: *mut GaspTable) {
    dispose_gasp(x);
}
#[inline]
unsafe extern "C" fn table_gasp_copy(mut dst: *mut GaspTable, mut src: *const GaspTable) {
    (*dst).version = (*src).version;
    (*dst).records = (*src).records.clone();
}
#[inline]
unsafe extern "C" fn table_gasp_init(mut x: *mut GaspTable) {
    init_gasp(x);
}
pub static TABLE_I_GASP: GaspTableElementInterface = {
    GaspTableElementInterface {
        init: Some(table_gasp_init as unsafe extern "C" fn(*mut GaspTable) -> ()),
        copy: Some(
            table_gasp_copy as unsafe extern "C" fn(*mut GaspTable, *const GaspTable) -> (),
        ),
        dispose: Some(table_gasp_dispose as unsafe extern "C" fn(*mut GaspTable) -> ()),
        create: Some(table_gasp_create),
        free: Some(table_gasp_free as unsafe extern "C" fn(*mut GaspTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_gasp_free(mut x: *mut GaspTable) {
    if x.is_null() {
        return;
    }
    table_gasp_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_read_gasp(
    packet: Packet,
    mut options: *const Options,
) -> *mut GaspTable {
    let mut num_ranges: TableId = 0;
    let mut gasp: *mut GaspTable = ::core::ptr::null_mut::<GaspTable>();
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
                        gasp = (
                            TABLE_I_GASP.create.expect("non-null function pointer"))();
                        (*gasp).version = read_16u(data as *const u8);
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
                                (*gasp).records.push(record);
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
                    TABLE_I_GASP.free.expect("non-null function pointer")(gasp);
                    gasp = ::core::ptr::null_mut::<GaspTable>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<GaspTable>();
}
pub unsafe extern "C" fn otfcc_dump_gasp(
    mut table: *const GaspTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if table.is_null() {
        return;
    }
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
) -> *mut GaspTable {
    let mut gasp: *mut GaspTable = ::core::ptr::null_mut::<GaspTable>();
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
            gasp = (
                TABLE_I_GASP.create.expect("non-null function pointer"))();
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
                    (*gasp).records.push(record);
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
pub unsafe extern "C" fn otfcc_build_gasp(
    mut gasp: *const GaspTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if gasp.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
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
