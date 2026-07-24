extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: size_t) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdsgrowzero(s: sds, len: size_t) -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn bufseek(buf: *mut caryll_Buffer, pos: size_t);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t);
    fn bufwrite_bytes(buf: *mut caryll_Buffer, size: size_t, str: *const uint8_t);
    fn bufwrite_buf(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    fn json_array_new(length: size_t) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: size_t) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: int64_t) -> *mut json_value;
    fn base64_encode(src: *const uint8_t, len: size_t, out_len: *mut size_t) -> *mut uint8_t;
    fn base64_decode(src: *const uint8_t, len: size_t, out_len: *mut size_t) -> *mut uint8_t;
    fn utf16be_to_utf8(inb: *const uint8_t, inlenb: ::core::ffi::c_int) -> sds;
    fn utf8toutf16be(_in: sds, out_bytes: *mut size_t) -> *mut uint8_t;
}
use crate::support::binio::{read_16u};
use crate::support::cvec::{
    cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push,
    cvec_resize_to, CVecRaw,
};
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
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
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed_4 = 10;
pub const log_vl_info: C2RustUnnamed_4 = 5;
pub const log_vl_notice: C2RustUnnamed_4 = 2;
pub const log_vl_important: C2RustUnnamed_4 = 1;
pub const log_vl_critical: C2RustUnnamed_4 = 0;
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
pub struct otfcc_NameRecord {
    pub platformID: uint16_t,
    pub encodingID: uint16_t,
    pub languageID: uint16_t,
    pub nameID: uint16_t,
    pub nameString: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otfcc_NameRecord {
    pub init: Option<unsafe extern "C" fn(*mut otfcc_NameRecord) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otfcc_NameRecord, *const otfcc_NameRecord) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otfcc_NameRecord, *mut otfcc_NameRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_NameRecord) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otfcc_NameRecord, otfcc_NameRecord) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otfcc_NameRecord, otfcc_NameRecord) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_name {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otfcc_NameRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_table_name {
    pub init: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_name, *const table_name) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_name, *mut table_name) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_name, table_name) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_name, table_name) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_name>,
    pub free: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_name, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_name, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut table_name>,
    pub fill: Option<unsafe extern "C" fn(*mut table_name, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_name, otfcc_NameRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_name) -> otfcc_NameRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_name, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_name,
            Option<unsafe extern "C" fn(*const otfcc_NameRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_name,
            Option<
                unsafe extern "C" fn(
                    *const otfcc_NameRecord,
                    *const otfcc_NameRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
pub type font_file_pointer = *mut uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as size_t,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as size_t;
        }
        _ => {}
    }
    return 0 as size_t;
}
pub const COPYRIGHT_LEN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C" fn nameRecordDtor(mut entry: *mut otfcc_NameRecord) {
    sdsfree((*entry).nameString);
    (*entry).nameString = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_move(
    mut dst: *mut otfcc_NameRecord,
    mut src: *mut otfcc_NameRecord,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_NameRecord>() as size_t,
    );
    otfcc_NameRecord_init(src);
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_init(mut x: *mut otfcc_NameRecord) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<otfcc_NameRecord>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_replace(
    mut dst: *mut otfcc_NameRecord,
    src: otfcc_NameRecord,
) {
    otfcc_NameRecord_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_NameRecord>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_copyReplace(
    mut dst: *mut otfcc_NameRecord,
    src: otfcc_NameRecord,
) {
    otfcc_NameRecord_dispose(dst);
    otfcc_NameRecord_copy(dst, &raw const src);
}
#[no_mangle]
pub static mut otfcc_iNameRecord: __caryll_elementinterface_otfcc_NameRecord = {
    __caryll_elementinterface_otfcc_NameRecord {
        init: Some(otfcc_NameRecord_init as unsafe extern "C" fn(*mut otfcc_NameRecord) -> ()),
        copy: Some(
            otfcc_NameRecord_copy
                as unsafe extern "C" fn(*mut otfcc_NameRecord, *const otfcc_NameRecord) -> (),
        ),
        move_0: Some(
            otfcc_NameRecord_move
                as unsafe extern "C" fn(*mut otfcc_NameRecord, *mut otfcc_NameRecord) -> (),
        ),
        dispose: Some(
            otfcc_NameRecord_dispose as unsafe extern "C" fn(*mut otfcc_NameRecord) -> (),
        ),
        replace: Some(
            otfcc_NameRecord_replace
                as unsafe extern "C" fn(*mut otfcc_NameRecord, otfcc_NameRecord) -> (),
        ),
        copyReplace: Some(
            otfcc_NameRecord_copyReplace
                as unsafe extern "C" fn(*mut otfcc_NameRecord, otfcc_NameRecord) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otfcc_NameRecord_dispose(mut x: *mut otfcc_NameRecord) {
    nameRecordDtor(x);
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_copy(
    mut dst: *mut otfcc_NameRecord,
    mut src: *const otfcc_NameRecord,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_NameRecord>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_name_growTo(arr: *mut table_name, target: size_t) {
    cvec_grow_to(table_name_as_cvec(arr), target);
}
#[inline]
unsafe fn table_name_as_cvec(arr: *mut table_name) -> *mut CVecRaw<otfcc_NameRecord> {
    arr as *mut CVecRaw<otfcc_NameRecord>
}
#[inline]
unsafe extern "C" fn table_name_init(arr: *mut table_name) {
    cvec_init(table_name_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_name_filterEnv(
    mut arr: *mut table_name,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otfcc_NameRecord, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: size_t = 0 as size_t;
    let mut k: size_t = 0 as size_t;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otfcc_NameRecord,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otfcc_iNameRecord.dispose.is_some() {
                otfcc_iNameRecord
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otfcc_NameRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_name_disposeItem(mut arr: *mut table_name, mut n: size_t) {
    if otfcc_iNameRecord.dispose.is_some() {
        otfcc_iNameRecord
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otfcc_NameRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_name_sort(
    mut arr: *mut table_name,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otfcc_NameRecord,
            *const otfcc_NameRecord,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otfcc_NameRecord>() as size_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otfcc_NameRecord,
                    *const otfcc_NameRecord,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn table_name_fill(mut arr: *mut table_name, mut n: size_t) {
    while (*arr).length < n {
        let mut x: otfcc_NameRecord = otfcc_NameRecord {
            platformID: 0,
            encodingID: 0,
            languageID: 0,
            nameID: 0,
            nameString: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        if otfcc_iNameRecord.init.is_some() {
            otfcc_iNameRecord.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otfcc_NameRecord>() as size_t,
            );
        }
        table_name_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_name_push(arr: *mut table_name, elem: otfcc_NameRecord) {
    cvec_push(table_name_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_name_grow(arr: *mut table_name) {
    cvec_grow(table_name_as_cvec(arr));
}
#[no_mangle]
pub static mut table_iName: __caryll_vectorinterface_table_name = {
    __caryll_vectorinterface_table_name {
        init: Some(table_name_init as unsafe extern "C" fn(*mut table_name) -> ()),
        copy: Some(
            table_name_copy as unsafe extern "C" fn(*mut table_name, *const table_name) -> (),
        ),
        move_0: Some(
            table_name_move as unsafe extern "C" fn(*mut table_name, *mut table_name) -> (),
        ),
        dispose: Some(table_name_dispose as unsafe extern "C" fn(*mut table_name) -> ()),
        replace: Some(
            table_name_replace as unsafe extern "C" fn(*mut table_name, table_name) -> (),
        ),
        copyReplace: Some(
            table_name_copyReplace as unsafe extern "C" fn(*mut table_name, table_name) -> (),
        ),
        create: Some(table_name_create),
        free: Some(table_name_free as unsafe extern "C" fn(*mut table_name) -> ()),
        initN: Some(table_name_initN as unsafe extern "C" fn(*mut table_name, size_t) -> ()),
        initCapN: Some(table_name_initCapN as unsafe extern "C" fn(*mut table_name, size_t) -> ()),
        createN: Some(table_name_createN as unsafe extern "C" fn(size_t) -> *mut table_name),
        fill: Some(table_name_fill as unsafe extern "C" fn(*mut table_name, size_t) -> ()),
        clear: Some(table_name_dispose as unsafe extern "C" fn(*mut table_name) -> ()),
        push: Some(
            table_name_push as unsafe extern "C" fn(*mut table_name, otfcc_NameRecord) -> (),
        ),
        shrinkToFit: Some(table_name_shrinkToFit as unsafe extern "C" fn(*mut table_name) -> ()),
        pop: Some(table_name_pop as unsafe extern "C" fn(*mut table_name) -> otfcc_NameRecord),
        disposeItem: Some(
            table_name_disposeItem as unsafe extern "C" fn(*mut table_name, size_t) -> (),
        ),
        filterEnv: Some(
            table_name_filterEnv
                as unsafe extern "C" fn(
                    *mut table_name,
                    Option<
                        unsafe extern "C" fn(
                            *const otfcc_NameRecord,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_name_sort
                as unsafe extern "C" fn(
                    *mut table_name,
                    Option<
                        unsafe extern "C" fn(
                            *const otfcc_NameRecord,
                            *const otfcc_NameRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn table_name_pop(arr: *mut table_name) -> otfcc_NameRecord {
    cvec_pop(table_name_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_name_copyReplace(mut dst: *mut table_name, src: table_name) {
    table_name_dispose(dst);
    table_name_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_name_copy(mut dst: *mut table_name, mut src: *const table_name) {
    table_name_init(dst);
    table_name_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otfcc_iNameRecord.copy.is_some() {
        let mut j: size_t = 0 as size_t;
        while j < (*src).length {
            otfcc_iNameRecord.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otfcc_NameRecord,
                (*src).items.offset(j as isize) as *mut otfcc_NameRecord as *const otfcc_NameRecord,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: size_t = 0 as size_t;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn table_name_dispose(mut arr: *mut table_name) {
    if arr.is_null() {
        return;
    }
    if otfcc_iNameRecord.dispose.is_some() {
        let mut j: size_t = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            otfcc_iNameRecord
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otfcc_NameRecord
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otfcc_NameRecord>();
    (*arr).length = 0 as size_t;
    (*arr).capacity = 0 as size_t;
}
#[inline]
unsafe extern "C" fn table_name_replace(mut dst: *mut table_name, src: table_name) {
    table_name_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_name>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_name_initCapN(mut arr: *mut table_name, mut n: size_t) {
    table_name_init(arr);
    table_name_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn table_name_growToN(arr: *mut table_name, target: size_t) {
    cvec_grow_to_n(table_name_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_name_initN(mut arr: *mut table_name, mut n: size_t) {
    table_name_init(arr);
    table_name_growToN(arr, n);
    table_name_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_name_free(mut x: *mut table_name) {
    if x.is_null() {
        return;
    }
    table_name_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_name_createN(mut n: size_t) -> *mut table_name {
    let mut t: *mut table_name =
        malloc(::core::mem::size_of::<table_name>() as size_t) as *mut table_name;
    table_name_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_name_create() -> *mut table_name {
    let mut x: *mut table_name =
        malloc(::core::mem::size_of::<table_name>() as size_t) as *mut table_name;
    table_name_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_name_shrinkToFit(mut arr: *mut table_name) {
    table_name_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_name_resizeTo(arr: *mut table_name, target: size_t) {
    cvec_resize_to(table_name_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_name_move(dst: *mut table_name, src: *mut table_name) {
    cvec_move(table_name_as_cvec(dst), table_name_as_cvec(src));
}
unsafe extern "C" fn shouldDecodeAsUTF16(mut record: *const otfcc_NameRecord) -> bool {
    return (*record).platformID as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*record).platformID as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            && (*record).encodingID as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        || (*record).platformID as ::core::ffi::c_int == 3 as ::core::ffi::c_int
            && ((*record).encodingID as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*record).encodingID as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                || (*record).encodingID as ::core::ffi::c_int == 10 as ::core::ffi::c_int);
}
unsafe extern "C" fn shouldDecodeAsBytes(mut record: *const otfcc_NameRecord) -> bool {
    return (*record).platformID as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && (*record).encodingID as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*record).languageID as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readName(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_name {
    let mut count: uint32_t = 0;
    let mut stringOffset: uint32_t = 0;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1851878757i32 as uint32_t {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut name: *mut table_name = ::core::ptr::null_mut::<table_name>();
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: uint32_t = table.length;
                    if !(length < 6 as uint32_t) {
                        count = read_16u(
                            data.offset(2 as ::core::ffi::c_int as isize) as *const uint8_t
                        ) as uint32_t;
                        stringOffset = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const uint8_t
                        ) as uint32_t;
                        if !(length
                            < (6 as uint32_t).wrapping_add((12 as uint32_t).wrapping_mul(count)))
                        {
                            name = (
                                table_iName.create.expect("non-null function pointer"))();
                            let mut j: uint16_t = 0 as uint16_t;
                            while (j as uint32_t) < count {
                                let mut record: otfcc_NameRecord = otfcc_NameRecord {
                                    platformID: 0,
                                    encodingID: 0,
                                    languageID: 0,
                                    nameID: 0,
                                    nameString: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                };
                                record.platformID = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize).offset(
                                        (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const uint8_t,
                                );
                                record.encodingID = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const uint8_t,
                                );
                                record.languageID = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(4 as ::core::ffi::c_int as isize)
                                        as *const uint8_t,
                                );
                                record.nameID = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(6 as ::core::ffi::c_int as isize)
                                        as *const uint8_t,
                                );
                                record.nameString = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                let mut length_0: uint16_t = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(8 as ::core::ffi::c_int as isize)
                                        as *const uint8_t,
                                );
                                let mut offset: uint16_t = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(10 as ::core::ffi::c_int as isize)
                                        as *const uint8_t,
                                );
                                if shouldDecodeAsBytes(&raw mut record) {
                                    let mut nameString: sds = sdsnewlen(
                                        data.offset(stringOffset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const ::core::ffi::c_void,
                                        length_0 as size_t,
                                    );
                                    record.nameString = nameString;
                                } else if shouldDecodeAsUTF16(&raw mut record) {
                                    let mut nameString_0: sds = utf16be_to_utf8(
                                        data.offset(stringOffset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const uint8_t,
                                        length_0 as ::core::ffi::c_int,
                                    );
                                    record.nameString = nameString_0;
                                } else {
                                    let mut len: size_t = 0 as size_t;
                                    let mut buf: *mut uint8_t = base64_encode(
                                        data.offset(stringOffset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const uint8_t,
                                        length_0 as size_t,
                                        &raw mut len,
                                    );
                                    record.nameString =
                                        sdsnewlen(buf as *const ::core::ffi::c_void, len);
                                    free(buf as *mut ::core::ffi::c_void);
                                    buf = ::core::ptr::null_mut::<uint8_t>();
                                }
                                table_iName.push.expect("non-null function pointer")(name, record);
                                j = j.wrapping_add(1);
                            }
                            return name;
                        }
                    }
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as uint8_t,
                        log_type_warning,
                        sdscatprintf(
                            sdsempty(),
                            b"table 'name' corrupted.\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        ),
                    );
                    if !name.is_null() {
                        table_iName.free.expect("non-null function pointer")(name);
                        name = ::core::ptr::null_mut::<table_name>();
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
    return ::core::ptr::null_mut::<table_name>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpName(
    mut name: *const table_name,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if name.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"name\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _name: *mut json_value = json_array_new((*name).length);
        let mut j: uint16_t = 0 as uint16_t;
        while (j as size_t) < (*name).length {
            let mut r: *mut otfcc_NameRecord =
                (*name).items.offset(j as isize) as *mut otfcc_NameRecord;
            let mut record: *mut json_value = json_object_new(5 as size_t);
            json_object_push(
                record,
                b"platformID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).platformID as int64_t),
            );
            json_object_push(
                record,
                b"encodingID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).encodingID as int64_t),
            );
            json_object_push(
                record,
                b"languageID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).languageID as int64_t),
            );
            json_object_push(
                record,
                b"nameID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).nameID as int64_t),
            );
            json_object_push(
                record,
                b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                json_string_new_length(
                    sdslen((*r).nameString) as ::core::ffi::c_uint,
                    (*r).nameString as *const ::core::ffi::c_char,
                ),
            );
            json_array_push(_name, record);
            j = j.wrapping_add(1);
        }
        json_object_push(
            root,
            b"name\0" as *const u8 as *const ::core::ffi::c_char,
            _name,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
unsafe extern "C" fn name_record_sort(
    mut a: *const otfcc_NameRecord,
    mut b: *const otfcc_NameRecord,
) -> ::core::ffi::c_int {
    if (*a).platformID as ::core::ffi::c_int != (*b).platformID as ::core::ffi::c_int {
        return (*a).platformID as ::core::ffi::c_int - (*b).platformID as ::core::ffi::c_int;
    }
    if (*a).encodingID as ::core::ffi::c_int != (*b).encodingID as ::core::ffi::c_int {
        return (*a).encodingID as ::core::ffi::c_int - (*b).encodingID as ::core::ffi::c_int;
    }
    if (*a).languageID as ::core::ffi::c_int != (*b).languageID as ::core::ffi::c_int {
        return (*a).languageID as ::core::ffi::c_int - (*b).languageID as ::core::ffi::c_int;
    }
    return (*a).nameID as ::core::ffi::c_int - (*b).nameID as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseName(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_name {
    let mut name: *mut table_name = (
        table_iName.create.expect("non-null function pointer"))();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"name\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut j: uint32_t = 0 as uint32_t;
            while j < (*table).u.array.length as uint32_t {
                if !(*(*table).u.array.values.offset(j as isize)).is_null()
                    && (**(*table).u.array.values.offset(j as isize)).type_0 as ::core::ffi::c_uint
                        == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let mut _record: *mut json_value =
                        *(*table).u.array.values.offset(j as isize) as *mut json_value;
                    if json_obj_get_type(
                        _record,
                        b"platformID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as uint8_t,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Missing or invalid platformID for name entry %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                j,
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"encodingID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as uint8_t,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Missing or invalid encodingID for name entry %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                j,
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"languageID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as uint8_t,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Missing or invalid languageID for name entry %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                j,
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"nameID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as uint8_t,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Missing or invalid nameID for name entry %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                j,
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as uint8_t,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Missing or invalid name string for name entry %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                j,
                            ),
                        );
                    } else {
                        let mut record: otfcc_NameRecord = otfcc_NameRecord {
                            platformID: 0,
                            encodingID: 0,
                            languageID: 0,
                            nameID: 0,
                            nameString: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        };
                        record.platformID = json_obj_getint(
                            _record,
                            b"platformID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as uint16_t;
                        record.encodingID = json_obj_getint(
                            _record,
                            b"encodingID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as uint16_t;
                        record.languageID = json_obj_getint(
                            _record,
                            b"languageID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as uint16_t;
                        record.nameID = json_obj_getint(
                            _record,
                            b"nameID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as uint16_t;
                        let mut str: *mut json_value = json_obj_get_type(
                            _record,
                            b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                            json_string,
                        );
                        record.nameString = sdsnewlen(
                            (*str).u.string.ptr as *const ::core::ffi::c_void,
                            (*str).u.string.length as size_t,
                        );
                        table_iName.push.expect("non-null function pointer")(name, record);
                    }
                }
                j = j.wrapping_add(1);
            }
            table_iName.sort.expect("non-null function pointer")(
                name,
                Some(
                    name_record_sort
                        as unsafe extern "C" fn(
                            *const otfcc_NameRecord,
                            *const otfcc_NameRecord,
                        ) -> ::core::ffi::c_int,
                ),
            );
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildName(
    mut name: *const table_name,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if name.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite16b(buf, 0 as uint16_t);
    bufwrite16b(buf, (*name).length as uint16_t);
    bufwrite16b(buf, 0 as uint16_t);
    let mut strings: *mut caryll_Buffer = bufnew();
    let mut j: uint16_t = 0 as uint16_t;
    while (j as size_t) < (*name).length {
        let mut record: *mut otfcc_NameRecord =
            (*name).items.offset(j as isize) as *mut otfcc_NameRecord;
        bufwrite16b(buf, (*record).platformID);
        bufwrite16b(buf, (*record).encodingID);
        bufwrite16b(buf, (*record).languageID);
        bufwrite16b(buf, (*record).nameID);
        let mut cbefore: size_t = (*strings).cursor;
        if shouldDecodeAsUTF16(record) {
            let mut words: size_t = 0;
            let mut u16: *mut uint8_t = utf8toutf16be((*record).nameString, &raw mut words);
            bufwrite_bytes(strings, words, u16);
            free(u16 as *mut ::core::ffi::c_void);
            u16 = ::core::ptr::null_mut::<uint8_t>();
        } else if shouldDecodeAsBytes(record) {
            bufwrite_bytes(
                strings,
                sdslen((*record).nameString),
                (*record).nameString as *mut uint8_t,
            );
        } else {
            let mut length: size_t = 0;
            let mut decoded: *mut uint8_t = base64_decode(
                (*record).nameString as *mut uint8_t,
                sdslen((*record).nameString),
                &raw mut length,
            );
            bufwrite_bytes(strings, length, decoded);
            free(decoded as *mut ::core::ffi::c_void);
            decoded = ::core::ptr::null_mut::<uint8_t>();
        }
        let mut cafter: size_t = (*strings).cursor;
        bufwrite16b(buf, cafter.wrapping_sub(cbefore) as uint16_t);
        bufwrite16b(buf, cbefore as uint16_t);
        j = j.wrapping_add(1);
    }
    let mut copyright: sds = sdscatprintf(
        sdsempty(),
        b"-- By OTFCC %d.%d.%d --\0" as *const u8 as *const ::core::ffi::c_char,
        MAIN_VER,
        SECONDARY_VER,
        PATCH_VER,
    );
    sdsgrowzero(copyright, COPYRIGHT_LEN as size_t);
    bufwrite_bytes(strings, COPYRIGHT_LEN as size_t, copyright as *mut uint8_t);
    sdsfree(copyright);
    let mut stringsOffset: size_t = (*buf).cursor;
    bufwrite_buf(buf, strings);
    bufseek(buf, 4 as size_t);
    bufwrite16b(buf, stringsOffset as uint16_t);
    buffree(strings);
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
#[inline]
unsafe extern "C" fn json_obj_getint(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> int32_t {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as int32_t;
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as int32_t;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl as int32_t;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0 as int32_t;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAIN_VER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SECONDARY_VER: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PATCH_VER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
