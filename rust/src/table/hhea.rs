#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};
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


use crate::support::json_funcs::{json_obj_get_type, json_obj_getnum_fallback};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32s};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{f16dot16, font_file_pointer};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_object, json_value};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_hhea {
    pub version: f16dot16,
    pub ascender: i16,
    pub descender: i16,
    pub lineGap: i16,
    pub advanceWidthMax: u16,
    pub minLeftSideBearing: i16,
    pub minRightSideBearing: i16,
    pub xMaxExtent: i16,
    pub caretSlopeRise: i16,
    pub caretSlopeRun: i16,
    pub caretOffset: i16,
    pub reserved: [i16; 4],
    pub metricDataFormat: i16,
    pub numberOfMetrics: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_hhea {
    pub init: Option<unsafe extern "C" fn(*mut table_hhea) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_hhea, *const table_hhea) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_hhea, *mut table_hhea) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_hhea) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_hhea, table_hhea) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_hhea, table_hhea) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_hhea>,
    pub free: Option<unsafe extern "C" fn(*mut table_hhea) -> ()>,
}
#[inline]
unsafe extern "C" fn initHhea(mut hhea: *mut table_hhea) {
    memset(
        hhea as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_hhea>() as usize,
    );
    (*hhea).version = 0x10000 as ::core::ffi::c_int as f16dot16;
}
#[inline]
unsafe extern "C" fn disposeHhea(mut _hhea: *mut table_hhea) {}
#[inline]
unsafe extern "C" fn table_hhea_free(mut x: *mut table_hhea) {
    if x.is_null() {
        return;
    }
    table_hhea_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub static table_iHhea: __caryll_elementinterface_table_hhea = {
    __caryll_elementinterface_table_hhea {
        init: Some(table_hhea_init as unsafe extern "C" fn(*mut table_hhea) -> ()),
        copy: Some(
            table_hhea_copy as unsafe extern "C" fn(*mut table_hhea, *const table_hhea) -> (),
        ),
        move_0: Some(
            table_hhea_move as unsafe extern "C" fn(*mut table_hhea, *mut table_hhea) -> (),
        ),
        dispose: Some(table_hhea_dispose as unsafe extern "C" fn(*mut table_hhea) -> ()),
        replace: Some(
            table_hhea_replace as unsafe extern "C" fn(*mut table_hhea, table_hhea) -> (),
        ),
        copyReplace: Some(
            table_hhea_copyReplace as unsafe extern "C" fn(*mut table_hhea, table_hhea) -> (),
        ),
        create: Some(table_hhea_create),
        free: Some(table_hhea_free as unsafe extern "C" fn(*mut table_hhea) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_hhea_dispose(mut x: *mut table_hhea) {
    disposeHhea(x);
}
#[inline]
unsafe extern "C" fn table_hhea_create() -> *mut table_hhea {
    let mut x: *mut table_hhea =
        malloc(::core::mem::size_of::<table_hhea>() as usize) as *mut table_hhea;
    table_hhea_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_hhea_init(mut x: *mut table_hhea) {
    initHhea(x);
}
#[inline]
unsafe extern "C" fn table_hhea_replace(mut dst: *mut table_hhea, src: table_hhea) {
    table_hhea_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hhea>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hhea_copy(mut dst: *mut table_hhea, mut src: *const table_hhea) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hhea>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hhea_move(mut dst: *mut table_hhea, mut src: *mut table_hhea) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hhea>() as usize,
    );
    table_hhea_init(src);
}
#[inline]
unsafe extern "C" fn table_hhea_copyReplace(mut dst: *mut table_hhea, src: table_hhea) {
    table_hhea_dispose(dst);
    table_hhea_copy(dst, &raw const src);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_readHhea(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_hhea {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1751672161i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    if length < 36 as u32 {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important,
                            log_type_warning,
                            crate::sdsbuild!(sdsempty(), b"table 'hhea' corrupted.\n"),
                        );
                    } else {
                        let mut hhea: *mut table_hhea = ::core::ptr::null_mut::<table_hhea>();
                        hhea = __caryll_allocate_clean(
                            ::core::mem::size_of::<table_hhea>() as usize,
                            23 as ::core::ffi::c_ulong,
                        ) as *mut table_hhea;
                        (*hhea).version = read_32s(data as *const u8) as f16dot16;
                        (*hhea).ascender = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*hhea).descender = read_16u(
                            data.offset(6 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*hhea).lineGap = read_16u(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*hhea).advanceWidthMax = read_16u(
                            data.offset(10 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*hhea).minLeftSideBearing = read_16u(
                            data.offset(12 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).minRightSideBearing = read_16u(
                            data.offset(14 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).xMaxExtent = read_16u(
                            data.offset(16 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).caretSlopeRise = read_16u(
                            data.offset(18 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).caretSlopeRun = read_16u(
                            data.offset(20 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).caretOffset = read_16u(
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
                        (*hhea).metricDataFormat = read_16u(
                            data.offset(32 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).numberOfMetrics = read_16u(
                            data.offset(34 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        return hhea;
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
    return ::core::ptr::null_mut::<table_hhea>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_dumpHhea(
    mut table: *const table_hhea,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"hhea"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut hhea: *mut json_value = json_object_new(13 as usize);
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
            json_integer_new((*table).lineGap as i64),
        );
        json_object_push(
            hhea,
            b"advanceWidthMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).advanceWidthMax as i64),
        );
        json_object_push(
            hhea,
            b"minLeftSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).minLeftSideBearing as i64),
        );
        json_object_push(
            hhea,
            b"minRightSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).minRightSideBearing as i64),
        );
        json_object_push(
            hhea,
            b"xMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).xMaxExtent as i64),
        );
        json_object_push(
            hhea,
            b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caretSlopeRise as i64),
        );
        json_object_push(
            hhea,
            b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caretSlopeRun as i64),
        );
        json_object_push(
            hhea,
            b"caretOffset\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caretOffset as i64),
        );
        json_object_push(
            root,
            b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
            hhea,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_parseHhea(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_hhea {
    let mut hhea: *mut table_hhea = (
        table_iHhea.create.expect("non-null function pointer"))();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"hhea"),
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
            (*hhea).lineGap = json_obj_getnum_fallback(
                table,
                b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).advanceWidthMax = json_obj_getnum_fallback(
                table,
                b"advanceWidthMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*hhea).minLeftSideBearing = json_obj_getnum_fallback(
                table,
                b"minLeftSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).minRightSideBearing = json_obj_getnum_fallback(
                table,
                b"minRightSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).xMaxExtent = json_obj_getnum_fallback(
                table,
                b"xMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caretSlopeRise = json_obj_getnum_fallback(
                table,
                b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caretSlopeRun = json_obj_getnum_fallback(
                table,
                b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caretOffset = json_obj_getnum_fallback(
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
    return hhea;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_buildHhea(
    mut hhea: *const table_hhea,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if hhea.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite32b(buf, (*hhea).version as u32);
    bufwrite16b(buf, (*hhea).ascender as u16);
    bufwrite16b(buf, (*hhea).descender as u16);
    bufwrite16b(buf, (*hhea).lineGap as u16);
    bufwrite16b(buf, (*hhea).advanceWidthMax);
    bufwrite16b(buf, (*hhea).minLeftSideBearing as u16);
    bufwrite16b(buf, (*hhea).minRightSideBearing as u16);
    bufwrite16b(buf, (*hhea).xMaxExtent as u16);
    bufwrite16b(buf, (*hhea).caretSlopeRise as u16);
    bufwrite16b(buf, (*hhea).caretSlopeRun as u16);
    bufwrite16b(buf, (*hhea).caretOffset as u16);
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
    bufwrite16b(buf, (*hhea).numberOfMetrics);
    return buf;
}
