#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, strcmp};
unsafe extern "C" {
    fn sdsempty() -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_16s, read_32s};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{f16dot16, font_file_pointer};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_double, json_integer, json_object, json_type, json_value};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_vhea {
    pub version: f16dot16,
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
pub struct __caryll_elementinterface_table_vhea {
    pub init: Option<unsafe extern "C" fn(*mut table_vhea) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_vhea, *const table_vhea) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_vhea, *mut table_vhea) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_vhea) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_vhea, table_vhea) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_vhea, table_vhea) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_vhea>,
    pub free: Option<unsafe extern "C" fn(*mut table_vhea) -> ()>,
}
#[inline]
unsafe extern "C" fn initVhea(mut vhea: *mut table_vhea) {
    memset(
        vhea as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_vhea>() as usize,
    );
    (*vhea).version = 0x10000 as ::core::ffi::c_int as f16dot16;
}
#[inline]
unsafe extern "C" fn disposeVhea(mut _vhea: *mut table_vhea) {}
#[inline]
unsafe extern "C" fn table_vhea_dispose(mut x: *mut table_vhea) {
    disposeVhea(x);
}
#[inline]
unsafe extern "C" fn table_vhea_free(mut x: *mut table_vhea) {
    if x.is_null() {
        return;
    }
    table_vhea_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_vhea_create() -> *mut table_vhea {
    let mut x: *mut table_vhea =
        malloc(::core::mem::size_of::<table_vhea>() as usize) as *mut table_vhea;
    table_vhea_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_vhea_init(mut x: *mut table_vhea) {
    initVhea(x);
}
#[unsafe(no_mangle)]
pub static table_iVhea: __caryll_elementinterface_table_vhea = {
    __caryll_elementinterface_table_vhea {
        init: Some(table_vhea_init as unsafe extern "C" fn(*mut table_vhea) -> ()),
        copy: Some(
            table_vhea_copy as unsafe extern "C" fn(*mut table_vhea, *const table_vhea) -> (),
        ),
        move_0: Some(
            table_vhea_move as unsafe extern "C" fn(*mut table_vhea, *mut table_vhea) -> (),
        ),
        dispose: Some(table_vhea_dispose as unsafe extern "C" fn(*mut table_vhea) -> ()),
        replace: Some(
            table_vhea_replace as unsafe extern "C" fn(*mut table_vhea, table_vhea) -> (),
        ),
        copyReplace: Some(
            table_vhea_copyReplace as unsafe extern "C" fn(*mut table_vhea, table_vhea) -> (),
        ),
        create: Some(table_vhea_create),
        free: Some(table_vhea_free as unsafe extern "C" fn(*mut table_vhea) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_vhea_copyReplace(mut dst: *mut table_vhea, src: table_vhea) {
    table_vhea_dispose(dst);
    table_vhea_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_vhea_copy(mut dst: *mut table_vhea, mut src: *const table_vhea) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_vhea>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_vhea_move(mut dst: *mut table_vhea, mut src: *mut table_vhea) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_vhea>() as usize,
    );
    table_vhea_init(src);
}
#[inline]
unsafe extern "C" fn table_vhea_replace(mut dst: *mut table_vhea, src: table_vhea) {
    table_vhea_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_vhea>() as usize,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_readVhea(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_vhea {
    let mut vhea: *mut table_vhea = ::core::ptr::null_mut::<table_vhea>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1986553185i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: usize = table.length as usize;
                    if length >= 36 as usize {
                        vhea = __caryll_allocate_clean(
                            ::core::mem::size_of::<table_vhea>() as usize,
                            20 as ::core::ffi::c_ulong,
                        ) as *mut table_vhea;
                        (*vhea).version = read_32s(data as *const u8) as f16dot16;
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
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
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
    return ::core::ptr::null_mut::<table_vhea>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_dumpVhea(
    mut table: *const table_vhea,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if table.is_null() {
        return;
    }
    let mut vhea: *mut json_value = json_object_new(11 as usize);
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_parseVhea(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_vhea {
    let mut vhea: *mut table_vhea = ::core::ptr::null_mut::<table_vhea>();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"vhea\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        vhea = (
            table_iVhea.create.expect("non-null function pointer"))();
        if vhea.is_null() {
            return ::core::ptr::null_mut::<table_vhea>();
        }
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
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
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return vhea;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_buildVhea(
    mut vhea: *const table_vhea,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if vhea.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
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
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_getnum(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0.0f64;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0.0f64;
}
#[inline]
unsafe extern "C" fn json_obj_getnum_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return fallback;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return fallback;
}
