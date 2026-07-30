#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::json_funcs::{json_obj_get_type, json_obj_getnum, json_obj_getnum_fallback};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_16s, read_32s};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, FontFilePointer};
use crate::vendor::json::{JsonType, JsonValue};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b};
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::vendor::json_builder::{json_double_new, json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsempty};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VheaTable {
    pub version: F16Dot16,
    pub ascent: i16,
    pub descent: i16,
    pub lineGap: i16,
    pub advanceHeightMax: i16,
    pub minTop: i16,
    pub minBottom: i16,
    pub yMaxExtent: i16,
    pub caretSlopeRise: i16,
    pub caretSlopeRun: i16,
    pub caretOffset: i16,
    pub dummy0: i16,
    pub dummy1: i16,
    pub dummy2: i16,
    pub dummy3: i16,
    pub metricDataFormat: i16,
    pub numOfLongVerMetrics: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VheaTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VheaTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VheaTable, *const VheaTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VheaTable, *mut VheaTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VheaTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VheaTable, VheaTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VheaTable, VheaTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VheaTable>,
    pub free: Option<unsafe extern "C" fn(*mut VheaTable) -> ()>,
}
#[inline]
unsafe extern "C" fn init_vhea(mut vhea: *mut VheaTable) {
    memset(
        vhea as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VheaTable>() as usize,
    );
    (*vhea).version = 0x10000 as ::core::ffi::c_int as F16Dot16;
}
#[inline]
unsafe extern "C" fn dispose_vhea(mut _vhea: *mut VheaTable) {}
#[inline]
unsafe extern "C" fn table_vhea_dispose(mut x: *mut VheaTable) {
    dispose_vhea(x);
}
#[inline]
unsafe extern "C" fn table_vhea_free(mut x: *mut VheaTable) {
    if x.is_null() {
        return;
    }
    table_vhea_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_vhea_create() -> *mut VheaTable {
    let mut x: *mut VheaTable =
        malloc(::core::mem::size_of::<VheaTable>() as usize) as *mut VheaTable;
    table_vhea_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_vhea_init(mut x: *mut VheaTable) {
    init_vhea(x);
}
pub static TABLE_I_VHEA: VheaTableElementInterface = {
    VheaTableElementInterface {
        init: Some(table_vhea_init as unsafe extern "C" fn(*mut VheaTable) -> ()),
        copy: Some(
            table_vhea_copy as unsafe extern "C" fn(*mut VheaTable, *const VheaTable) -> (),
        ),
        move_0: Some(
            table_vhea_move as unsafe extern "C" fn(*mut VheaTable, *mut VheaTable) -> (),
        ),
        dispose: Some(table_vhea_dispose as unsafe extern "C" fn(*mut VheaTable) -> ()),
        replace: Some(
            table_vhea_replace as unsafe extern "C" fn(*mut VheaTable, VheaTable) -> (),
        ),
        copyReplace: Some(
            table_vhea_copy_replace as unsafe extern "C" fn(*mut VheaTable, VheaTable) -> (),
        ),
        create: Some(table_vhea_create),
        free: Some(table_vhea_free as unsafe extern "C" fn(*mut VheaTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_vhea_copy_replace(mut dst: *mut VheaTable, src: VheaTable) {
    table_vhea_dispose(dst);
    table_vhea_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_vhea_copy(mut dst: *mut VheaTable, mut src: *const VheaTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VheaTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_vhea_move(mut dst: *mut VheaTable, mut src: *mut VheaTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VheaTable>() as usize,
    );
    table_vhea_init(src);
}
#[inline]
unsafe extern "C" fn table_vhea_replace(mut dst: *mut VheaTable, src: VheaTable) {
    table_vhea_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VheaTable>() as usize,
    );
}
pub unsafe extern "C" fn otfcc_read_vhea(
    packet: Packet,
    mut options: *const Options,
) -> *mut VheaTable {
    let mut vhea: *mut VheaTable = ::core::ptr::null_mut::<VheaTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1986553185i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: usize = table.length as usize;
                    if length >= 36 as usize {
                        vhea = __caryll_allocate_clean(
                            ::core::mem::size_of::<VheaTable>() as usize,
                            20 as ::core::ffi::c_ulong,
                        ) as *mut VheaTable;
                        (*vhea).version = read_32s(data as *const u8) as F16Dot16;
                        (*vhea).ascent = read_16s(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        );
                        (*vhea).descent = read_16s(
                            data.offset(6 as ::core::ffi::c_int as isize) as *const u8
                        );
                        (*vhea).lineGap = read_16s(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const u8
                        );
                        (*vhea).advanceHeightMax = read_16s(
                            data.offset(10 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*vhea).minTop = read_16s(
                            data.offset(12 as ::core::ffi::c_int as isize) as *const u8
                        );
                        (*vhea).minBottom = read_16s(
                            data.offset(14 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*vhea).yMaxExtent = read_16s(
                            data.offset(16 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*vhea).caretSlopeRise = read_16s(
                            data.offset(18 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*vhea).caretSlopeRun = read_16s(
                            data.offset(20 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*vhea).caretOffset = read_16s(
                            data.offset(22 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*vhea).dummy0 = 0 as i16;
                        (*vhea).dummy1 = 0 as i16;
                        (*vhea).dummy2 = 0 as i16;
                        (*vhea).dummy3 = 0 as i16;
                        (*vhea).metricDataFormat = 0 as i16;
                        (*vhea).numOfLongVerMetrics = read_16u(
                            data.offset(34 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        return vhea;
                    } else {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::sdsbuild!(sdsempty(), b"Table 'vhea' corrupted."),
                        );
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
    return ::core::ptr::null_mut::<VheaTable>();
}
pub unsafe extern "C" fn otfcc_dump_vhea(
    mut table: *const VheaTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if table.is_null() {
        return;
    }
    let mut vhea: *mut JsonValue = json_object_new(11 as usize);
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"vhea"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        json_object_push(
            vhea,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            vhea,
            b"ascent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ascent as i64),
        );
        json_object_push(
            vhea,
            b"descent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).descent as i64),
        );
        json_object_push(
            vhea,
            b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).lineGap as i64),
        );
        json_object_push(
            vhea,
            b"advanceHeightMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).advanceHeightMax as i64),
        );
        json_object_push(
            vhea,
            b"minTop\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).minTop as i64),
        );
        json_object_push(
            vhea,
            b"minBottom\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).minBottom as i64),
        );
        json_object_push(
            vhea,
            b"yMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).yMaxExtent as i64),
        );
        json_object_push(
            vhea,
            b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caretSlopeRise as i64),
        );
        json_object_push(
            vhea,
            b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caretSlopeRun as i64),
        );
        json_object_push(
            vhea,
            b"caretOffset\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caretOffset as i64),
        );
        json_object_push(
            root,
            b"vhea\0" as *const u8 as *const ::core::ffi::c_char,
            vhea,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_vhea(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut VheaTable {
    let mut vhea: *mut VheaTable = ::core::ptr::null_mut::<VheaTable>();
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"vhea\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        vhea = (
            TABLE_I_VHEA.create.expect("non-null function pointer"))();
        if vhea.is_null() {
            return ::core::ptr::null_mut::<VheaTable>();
        }
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"vhea"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*vhea).version = otfcc_to_fixed(json_obj_getnum(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            (*vhea).ascent = json_obj_getnum_fallback(
                table,
                b"ascent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).descent = json_obj_getnum_fallback(
                table,
                b"descent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).lineGap = json_obj_getnum_fallback(
                table,
                b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).advanceHeightMax = json_obj_getnum_fallback(
                table,
                b"advanceHeightMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).minTop = json_obj_getnum_fallback(
                table,
                b"minTop\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).minBottom = json_obj_getnum_fallback(
                table,
                b"minBottom\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).yMaxExtent = json_obj_getnum_fallback(
                table,
                b"yMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).caretSlopeRise = json_obj_getnum_fallback(
                table,
                b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).caretSlopeRun = json_obj_getnum_fallback(
                table,
                b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*vhea).caretOffset = json_obj_getnum_fallback(
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
    return vhea;
}
pub unsafe extern "C" fn otfcc_build_vhea(
    mut vhea: *const VheaTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if vhea.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*vhea).version as u32);
    bufwrite16b(buf, (*vhea).ascent as u16);
    bufwrite16b(buf, (*vhea).descent as u16);
    bufwrite16b(buf, (*vhea).lineGap as u16);
    bufwrite16b(buf, (*vhea).advanceHeightMax as u16);
    bufwrite16b(buf, (*vhea).minTop as u16);
    bufwrite16b(buf, (*vhea).minBottom as u16);
    bufwrite16b(buf, (*vhea).yMaxExtent as u16);
    bufwrite16b(buf, (*vhea).caretSlopeRise as u16);
    bufwrite16b(buf, (*vhea).caretSlopeRun as u16);
    bufwrite16b(buf, (*vhea).caretOffset as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, (*vhea).numOfLongVerMetrics);
    return buf;
}
