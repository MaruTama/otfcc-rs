extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t);
    fn json_array_new(length: size_t) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: int64_t) -> *mut json_value;
    fn base64_decode(src: *const uint8_t, len: size_t, out_len: *mut size_t) -> *mut uint8_t;
}

use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type json_type = ::core::ffi::c_uint;
pub const json_pre_serialized: json_type = 8;
pub const json_null: json_type = 7;
pub const json_boolean: json_type = 6;
pub const json_string: json_type = 5;
pub const json_double: json_type = 4;
pub const json_integer: json_type = 3;
pub const json_array: json_type = 2;
pub const json_object: json_type = 1;
pub const json_none: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_value {
    pub parent: *mut _json_value,
    pub type_0: json_type,
    pub u: C2RustUnnamed_0,
    pub _reserved: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub next_alloc: *mut _json_value,
    pub object_mem: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub boolean: ::core::ffi::c_int,
    pub integer: int64_t,
    pub dbl: ::core::ffi::c_double,
    pub string: C2RustUnnamed_3,
    pub object: C2RustUnnamed_2,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut json_object_entry,
}
pub type json_object_entry = _json_object_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_object_entry {
    pub name: *mut ::core::ffi::c_char,
    pub name_length: ::core::ffi::c_uint,
    pub value: *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub length: ::core::ffi::c_uint,
    pub ptr: *mut ::core::ffi::c_char,
}
pub type json_value = _json_value;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILoggerTarget {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()>,
}
pub type otfcc_LoggerType = ::core::ffi::c_uint;
pub const log_type_progress: otfcc_LoggerType = 3;
pub const log_type_info: otfcc_LoggerType = 2;
pub const log_type_warning: otfcc_LoggerType = 1;
pub const log_type_error: otfcc_LoggerType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILogger {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub indent: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub indentSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub start: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub startSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub log: Option<
        unsafe extern "C" fn(
            *mut otfcc_ILogger,
            uint8_t,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t) -> ()>,
    pub getTarget: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Options {
    pub debug_wait_on_start: bool,
    pub ignore_glyph_order: bool,
    pub ignore_hints: bool,
    pub has_vertical_metrics: bool,
    pub export_fdselect: bool,
    pub keep_average_char_width: bool,
    pub keep_unicode_ranges: bool,
    pub short_post: bool,
    pub dummy_DSIG: bool,
    pub keep_modified_time: bool,
    pub instr_as_bytes: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub cff_short_vmtx: bool,
    pub merge_lookups: bool,
    pub merge_features: bool,
    pub force_cid: bool,
    pub cff_rollCharString: bool,
    pub cff_doSubroutinize: bool,
    pub stub_cmap4: bool,
    pub decimal_cmap: bool,
    pub name_glyphs_by_hash: bool,
    pub name_glyphs_by_gid: bool,
    pub glyph_name_prefix: *mut ::core::ffi::c_char,
    pub logger: *mut otfcc_ILogger,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_PacketPiece {
    pub tag: uint32_t,
    pub checkSum: uint32_t,
    pub offset: uint32_t,
    pub length: uint32_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Packet {
    pub sfnt_version: uint32_t,
    pub numTables: uint16_t,
    pub searchRange: uint16_t,
    pub entrySelector: uint16_t,
    pub rangeShift: uint16_t,
    pub pieces: *mut otfcc_PacketPiece,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_cvt {
    pub length: uint32_t,
    pub words: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_cvt {
    pub init: Option<unsafe extern "C" fn(*mut table_cvt) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_cvt, *const table_cvt) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_cvt, *mut table_cvt) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_cvt) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_cvt, table_cvt) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_cvt, table_cvt) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_cvt>,
    pub free: Option<unsafe extern "C" fn(*mut table_cvt) -> ()>,
}
pub type font_file_pointer = *mut uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn disposeCvt(mut table: *mut table_cvt) {
    if !(*table).words.is_null() {
        free((*table).words as *mut ::core::ffi::c_void);
        (*table).words = ::core::ptr::null_mut::<uint16_t>();
    }
}
#[inline]
unsafe extern "C" fn table_cvt_copy(mut dst: *mut table_cvt, mut src: *const table_cvt) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_cvt>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_cvt_free(mut x: *mut table_cvt) {
    if x.is_null() {
        return;
    }
    table_cvt_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_cvt_create() -> *mut table_cvt {
    let mut x: *mut table_cvt =
        malloc(::core::mem::size_of::<table_cvt>() as size_t) as *mut table_cvt;
    table_cvt_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_cvt_init(mut x: *mut table_cvt) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_cvt>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_cvt_copyReplace(mut dst: *mut table_cvt, src: table_cvt) {
    table_cvt_dispose(dst);
    table_cvt_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_cvt_move(mut dst: *mut table_cvt, mut src: *mut table_cvt) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_cvt>() as size_t,
    );
    table_cvt_init(src);
}
#[inline]
unsafe extern "C" fn table_cvt_replace(mut dst: *mut table_cvt, src: table_cvt) {
    table_cvt_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_cvt>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_cvt_dispose(mut x: *mut table_cvt) {
    disposeCvt(x);
}
#[no_mangle]
pub static mut table_iCvt: __caryll_elementinterface_table_cvt = {
    __caryll_elementinterface_table_cvt {
        init: Some(table_cvt_init as unsafe extern "C" fn(*mut table_cvt) -> ()),
        copy: Some(table_cvt_copy as unsafe extern "C" fn(*mut table_cvt, *const table_cvt) -> ()),
        move_0: Some(table_cvt_move as unsafe extern "C" fn(*mut table_cvt, *mut table_cvt) -> ()),
        dispose: Some(table_cvt_dispose as unsafe extern "C" fn(*mut table_cvt) -> ()),
        replace: Some(table_cvt_replace as unsafe extern "C" fn(*mut table_cvt, table_cvt) -> ()),
        copyReplace: Some(
            table_cvt_copyReplace as unsafe extern "C" fn(*mut table_cvt, table_cvt) -> (),
        ),
        create: Some(table_cvt_create),
        free: Some(table_cvt_free as unsafe extern "C" fn(*mut table_cvt) -> ()),
    }
};
#[no_mangle]
pub unsafe extern "C" fn otfcc_readCvt(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
    mut tag: uint32_t,
) -> *mut table_cvt {
    let mut t: *mut table_cvt = ::core::ptr::null_mut::<table_cvt>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == tag {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: uint32_t = table.length;
                    t = __caryll_allocate_clean(
                        ::core::mem::size_of::<table_cvt>() as size_t,
                        16 as ::core::ffi::c_ulong,
                    ) as *mut table_cvt;
                    (*t).length = length >> 1 as ::core::ffi::c_int;
                    (*t).words = __caryll_allocate_clean(
                        (::core::mem::size_of::<uint16_t>() as size_t)
                            .wrapping_mul((*t).length.wrapping_add(1 as uint32_t) as size_t),
                        18 as ::core::ffi::c_ulong,
                    ) as *mut uint16_t;
                    let mut j: uint16_t = 0 as uint16_t;
                    while (j as uint32_t) < (*t).length {
                        *(*t).words.offset(j as isize) =
                            read_16u(data.offset(
                                (2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                            ) as *const uint8_t);
                        j = j.wrapping_add(1);
                    }
                    return t;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_cvt>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpCvt(
    mut table: *const table_cvt,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"cvt\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut arr: *mut json_value = json_array_new((*table).length as size_t);
        let mut j: uint16_t = 0 as uint16_t;
        while (j as uint32_t) < (*table).length {
            json_array_push(
                arr,
                json_integer_new(*(*table).words.offset(j as isize) as int64_t),
            );
            j = j.wrapping_add(1);
        }
        json_object_push(root, tag, arr);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseCvt(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut table_cvt {
    let mut t: *mut table_cvt = ::core::ptr::null_mut::<table_cvt>();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(root, tag, json_array);
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"cvt\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            t = __caryll_allocate_clean(
                ::core::mem::size_of::<table_cvt>() as size_t,
                44 as ::core::ffi::c_ulong,
            ) as *mut table_cvt;
            (*t).length = (*table).u.array.length as uint32_t;
            (*t).words = __caryll_allocate_clean(
                (::core::mem::size_of::<uint16_t>() as size_t)
                    .wrapping_mul((*t).length.wrapping_add(1 as uint32_t) as size_t),
                46 as ::core::ffi::c_ulong,
            ) as *mut uint16_t;
            let mut j: uint16_t = 0 as uint16_t;
            while (j as uint32_t) < (*t).length {
                let mut record: *mut json_value =
                    *(*table).u.array.values.offset(j as isize) as *mut json_value;
                if (*record).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    *(*t).words.offset(j as isize) = (*record).u.integer as uint16_t;
                } else if (*record).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    *(*t).words.offset(j as isize) = (*record).u.dbl as uint16_t;
                } else {
                    *(*t).words.offset(j as isize) = 0 as uint16_t;
                }
                j = j.wrapping_add(1);
            }
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    } else {
        table = json_obj_get_type(root, tag, json_string);
        if !table.is_null() {
            (*(*options).logger)
                .startSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                sdscatprintf(
                    sdsempty(),
                    b"cvt\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
            let mut ___loggedstep_v_0: bool = true;
            while ___loggedstep_v_0 {
                t = __caryll_allocate_clean(
                    ::core::mem::size_of::<table_cvt>() as size_t,
                    61 as ::core::ffi::c_ulong,
                ) as *mut table_cvt;
                let mut len: size_t = 0;
                let mut raw: *mut uint8_t = base64_decode(
                    (*table).u.string.ptr as *mut uint8_t,
                    (*table).u.string.length as size_t,
                    &raw mut len,
                );
                (*t).length = (len >> 1 as ::core::ffi::c_int) as uint32_t;
                (*t).words = __caryll_allocate_clean(
                    (::core::mem::size_of::<uint16_t>() as size_t)
                        .wrapping_mul((*t).length.wrapping_add(1 as uint32_t) as size_t),
                    66 as ::core::ffi::c_ulong,
                ) as *mut uint16_t;
                let mut j_0: uint16_t = 0 as uint16_t;
                while (j_0 as uint32_t) < (*t).length {
                    *(*t).words.offset(j_0 as isize) = read_16u(
                        raw.offset((2 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize),
                    );
                    j_0 = j_0.wrapping_add(1);
                }
                free(raw as *mut ::core::ffi::c_void);
                raw = ::core::ptr::null_mut::<uint8_t>();
                ___loggedstep_v_0 = false;
                (*(*options).logger)
                    .finish
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger
                );
            }
        }
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildCvt(
    mut table: *const table_cvt,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if table.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    let mut j: uint16_t = 0 as uint16_t;
    while (j as uint32_t) < (*table).length {
        bufwrite16b(buf, *(*table).words.offset(j as isize));
        j = j.wrapping_add(1);
    }
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
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
