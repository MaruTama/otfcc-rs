#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, strcmp};
unsafe extern "C" {
    fn sdsempty() -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn bufwrite64b(buf: *mut caryll_Buffer, x: u64);
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
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u, read_32s, read_64u};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{f16dot16, font_file_pointer};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_double, json_integer, json_object, json_type, json_value};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};
use crate::support::json_funcs::{otfcc_dump_flags, otfcc_parse_flags};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_head {
    pub version: f16dot16,
    pub fontRevision: u32,
    pub checkSumAdjustment: u32,
    pub magicNumber: u32,
    pub flags: u16,
    pub unitsPerEm: u16,
    pub created: i64,
    pub modified: i64,
    pub xMin: i16,
    pub yMin: i16,
    pub xMax: i16,
    pub yMax: i16,
    pub macStyle: u16,
    pub lowestRecPPEM: u16,
    pub fontDirectoryHint: i16,
    pub indexToLocFormat: i16,
    pub glyphDataFormat: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_head {
    pub init: Option<unsafe extern "C" fn(*mut table_head) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_head, *const table_head) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_head, *mut table_head) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_head) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_head, table_head) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_head, table_head) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_head>,
    pub free: Option<unsafe extern "C" fn(*mut table_head) -> ()>,
}
#[inline]
unsafe extern "C" fn initHead(mut head: *mut table_head) {
    memset(
        head as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_head>() as usize,
    );
    (*head).magicNumber = 0x5f0f3cf5 as u32;
    (*head).unitsPerEm = 1000 as u16;
}
#[inline]
unsafe extern "C" fn disposeHead(mut _head: *mut table_head) {}
#[inline]
unsafe extern "C" fn table_head_replace(mut dst: *mut table_head, src: table_head) {
    table_head_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_head>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_head_free(mut x: *mut table_head) {
    if x.is_null() {
        return;
    }
    table_head_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_head_copyReplace(mut dst: *mut table_head, src: table_head) {
    table_head_dispose(dst);
    table_head_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_head_copy(mut dst: *mut table_head, mut src: *const table_head) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_head>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_head_dispose(mut x: *mut table_head) {
    disposeHead(x);
}
#[inline]
unsafe extern "C" fn table_head_move(mut dst: *mut table_head, mut src: *mut table_head) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_head>() as usize,
    );
    table_head_init(src);
}
#[unsafe(no_mangle)]
pub static table_iHead: __caryll_elementinterface_table_head = {
    __caryll_elementinterface_table_head {
        init: Some(table_head_init as unsafe extern "C" fn(*mut table_head) -> ()),
        copy: Some(
            table_head_copy as unsafe extern "C" fn(*mut table_head, *const table_head) -> (),
        ),
        move_0: Some(
            table_head_move as unsafe extern "C" fn(*mut table_head, *mut table_head) -> (),
        ),
        dispose: Some(table_head_dispose as unsafe extern "C" fn(*mut table_head) -> ()),
        replace: Some(
            table_head_replace as unsafe extern "C" fn(*mut table_head, table_head) -> (),
        ),
        copyReplace: Some(
            table_head_copyReplace as unsafe extern "C" fn(*mut table_head, table_head) -> (),
        ),
        create: Some(table_head_create),
        free: Some(table_head_free as unsafe extern "C" fn(*mut table_head) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_head_create() -> *mut table_head {
    let mut x: *mut table_head =
        malloc(::core::mem::size_of::<table_head>() as usize) as *mut table_head;
    table_head_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_head_init(mut x: *mut table_head) {
    initHead(x);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_readHead(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_head {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1751474532i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    if length < 54 as u32 {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important,
                            log_type_warning,
                            crate::sdsbuild!(sdsempty(), b"table 'head' corrupted.\n"),
                        );
                    } else {
                        let mut head: *mut table_head = ::core::ptr::null_mut::<table_head>();
                        head = __caryll_allocate_clean(
                            ::core::mem::size_of::<table_head>() as usize,
                            24 as ::core::ffi::c_ulong,
                        ) as *mut table_head;
                        (*head).version = read_32s(data as *const u8) as f16dot16;
                        (*head).fontRevision = read_32u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).checkSumAdjustment = read_32u(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).magicNumber = read_32u(
                            data.offset(12 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).flags = read_16u(
                            data.offset(16 as ::core::ffi::c_int as isize) as *const u8
                        );
                        (*head).unitsPerEm = read_16u(
                            data.offset(18 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).created = read_64u(
                            data.offset(20 as ::core::ffi::c_int as isize) as *const u8
                        ) as i64;
                        (*head).modified = read_64u(
                            data.offset(28 as ::core::ffi::c_int as isize) as *const u8
                        ) as i64;
                        (*head).xMin = read_16u(
                            data.offset(36 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).yMin = read_16u(
                            data.offset(38 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).xMax = read_16u(
                            data.offset(40 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).yMax = read_16u(
                            data.offset(42 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).macStyle = read_16u(
                            data.offset(44 as ::core::ffi::c_int as isize) as *const u8
                        );
                        (*head).lowestRecPPEM = read_16u(
                            data.offset(46 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).fontDirectoryHint = read_16u(
                            data.offset(48 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*head).indexToLocFormat = read_16u(
                            data.offset(50 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*head).glyphDataFormat = read_16u(
                            data.offset(52 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        return head;
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
    return ::core::ptr::null_mut::<table_head>();
}
static headFlagsLabels: [&::core::ffi::CStr; 15] = [
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
static macStyleLabels: [&::core::ffi::CStr; 7] = [
    c"bold",
    c"italic",
    c"underline",
    c"outline",
    c"shadow",
    c"condensed",
    c"extended",
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_dumpHead(
    mut table: *const table_head,
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
        crate::sdsbuild!(sdsempty(), b"head"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut head: *mut json_value = json_object_new(15 as usize);
        json_object_push(
            head,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            head,
            b"fontRevision\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).fontRevision as f16dot16)),
        );
        json_object_push(
            head,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).flags as ::core::ffi::c_int,
                &headFlagsLabels,
            ),
        );
        json_object_push(
            head,
            b"unitsPerEm\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).unitsPerEm as i64),
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
            json_integer_new((*table).xMin as i64),
        );
        json_object_push(
            head,
            b"xMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).xMax as i64),
        );
        json_object_push(
            head,
            b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).yMin as i64),
        );
        json_object_push(
            head,
            b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).yMax as i64),
        );
        json_object_push(
            head,
            b"macStyle\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).macStyle as ::core::ffi::c_int,
                &macStyleLabels,
            ),
        );
        json_object_push(
            head,
            b"lowestRecPPEM\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).lowestRecPPEM as i64),
        );
        json_object_push(
            head,
            b"fontDirectoryHint\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).fontDirectoryHint as i64),
        );
        json_object_push(
            head,
            b"indexToLocFormat\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).indexToLocFormat as i64),
        );
        json_object_push(
            head,
            b"glyphDataFormat\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).glyphDataFormat as i64),
        );
        json_object_push(
            root,
            b"head\0" as *const u8 as *const ::core::ffi::c_char,
            head,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_parseHead(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_head {
    let mut head: *mut table_head = (
        table_iHead.create.expect("non-null function pointer"))();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"head\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"head"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*head).version = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ));
            (*head).fontRevision = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"fontRevision\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            )) as u32;
            (*head).flags = otfcc_parse_flags(
                json_obj_get(table, b"flags\0" as *const u8 as *const ::core::ffi::c_char),
                &headFlagsLabels,
            ) as u16;
            (*head).unitsPerEm = json_obj_getnum_fallback(
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
            (*head).xMin = json_obj_getnum_fallback(
                table,
                b"xMin\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).xMax = json_obj_getnum_fallback(
                table,
                b"xMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).yMin = json_obj_getnum_fallback(
                table,
                b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).yMax = json_obj_getnum_fallback(
                table,
                b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).macStyle = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"macStyle\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &macStyleLabels,
            ) as u16;
            (*head).lowestRecPPEM = json_obj_getnum_fallback(
                table,
                b"lowestRecPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*head).fontDirectoryHint = json_obj_getnum_fallback(
                table,
                b"fontDirectoryHint\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).indexToLocFormat = json_obj_getnum_fallback(
                table,
                b"indexToLocFormat\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).glyphDataFormat = json_obj_getnum_fallback(
                table,
                b"glyphDataFormat\0" as *const u8 as *const ::core::ffi::c_char,
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
    return head;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_buildHead(
    mut head: *const table_head,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if head.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite32b(buf, (*head).version as u32);
    bufwrite32b(buf, (*head).fontRevision);
    bufwrite32b(buf, (*head).checkSumAdjustment);
    bufwrite32b(buf, (*head).magicNumber);
    bufwrite16b(buf, (*head).flags);
    bufwrite16b(buf, (*head).unitsPerEm);
    bufwrite64b(buf, (*head).created as u64);
    bufwrite64b(buf, (*head).modified as u64);
    bufwrite16b(buf, (*head).xMin as u16);
    bufwrite16b(buf, (*head).yMin as u16);
    bufwrite16b(buf, (*head).xMax as u16);
    bufwrite16b(buf, (*head).yMax as u16);
    bufwrite16b(buf, (*head).macStyle);
    bufwrite16b(buf, (*head).lowestRecPPEM);
    bufwrite16b(buf, (*head).fontDirectoryHint as u16);
    bufwrite16b(buf, (*head).indexToLocFormat as u16);
    bufwrite16b(buf, (*head).glyphDataFormat as u16);
    return buf;
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 != json_object
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
unsafe extern "C" fn json_obj_getnum_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 != json_object
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
                && (*cv).type_0 == json_integer
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 == json_double
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return fallback;
}
