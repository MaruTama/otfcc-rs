extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn json_object_new(length: size_t) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: int64_t) -> *mut json_value;
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
    fn json_null_new() -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> size_t;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: size_t) -> sds;
    fn sdsfree(s: sds);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t);
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
}

use crate::src::lib::table::otl::coverage::{otl_Coverage};
use crate::src::lib::support::handle::{handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle, otfcc_GlyphHandle, HANDLE_STATE_EMPTY};
use crate::src::lib::support::stdio::FILE;
use crate::src::lib::support::alloc::{__caryll_allocate_clean};
use crate::src::lib::support::binio::{read_16u, read_16s};
use crate::src::lib::support::cvec::{
    cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push,
    cvec_resize_to, CVecRaw,
};
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int16_t = __int16_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_serialize_opts {
    pub mode: ::core::ffi::c_int,
    pub opts: ::core::ffi::c_int,
    pub indent_size: ::core::ffi::c_int,
}
pub type sds = *mut ::core::ffi::c_char;
pub type ptrdiff_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: ::core::ffi::c_uint,
    pub expand_mult: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut ::core::ffi::c_void,
    pub next: *mut ::core::ffi::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut ::core::ffi::c_void,
    pub keylen: ::core::ffi::c_uint,
    pub hashv: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: ::core::ffi::c_uint,
    pub log2_num_buckets: ::core::ffi::c_uint,
    pub num_items: ::core::ffi::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: ::core::ffi::c_uint,
    pub nonideal_items: ::core::ffi::c_uint,
    pub ineff_expands: ::core::ffi::c_uint,
    pub noexpand: ::core::ffi::c_uint,
    pub signature: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type glyphid_t = uint16_t;
pub type glyphclass_t = uint16_t;
pub type pos_t = ::core::ffi::c_double;
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
pub type font_file_pointer = *mut uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_bkblock {
    pub _visitstate: bk_cell_visit_state,
    pub _index: uint32_t,
    pub _height: uint32_t,
    pub _depth: uint32_t,
    pub length: uint32_t,
    pub free: uint32_t,
    pub cells: *mut bk_Cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bk_Cell {
    pub t: bk_CellType,
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub z: uint32_t,
    pub p: *mut __caryll_bkblock,
}
pub type bk_CellType = ::core::ffi::c_uint;
pub const bkembed: bk_CellType = 255;
pub const bkcopy: bk_CellType = 254;
pub const sp32: bk_CellType = 129;
pub const sp16: bk_CellType = 128;
pub const p32: bk_CellType = 17;
pub const p16: bk_CellType = 16;
pub const b32: bk_CellType = 3;
pub const b16: bk_CellType = 2;
pub const b8: bk_CellType = 1;
pub const bkover: bk_CellType = 0;
pub type bk_cell_visit_state = ::core::ffi::c_uint;
pub const VISIT_BLACK: bk_cell_visit_state = 2;
pub const VISIT_GRAY: bk_cell_visit_state = 1;
pub const VISIT_WHITE: bk_cell_visit_state = 0;
pub type bk_Block = __caryll_bkblock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_Anchor {
    pub present: bool,
    pub x: pos_t,
    pub y: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_MarkArray {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_MarkRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_MarkRecord {
    pub glyph: otfcc_GlyphHandle,
    pub markClass: glyphclass_t,
    pub anchor: otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_PositionValue {
    pub dx: pos_t,
    pub dy: pos_t,
    pub dWidth: pos_t,
    pub dHeight: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_MarkArray {
    pub init: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_MarkArray, *const otl_MarkArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_MarkArray, *mut otl_MarkArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkArray) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_MarkArray>,
    pub free: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_MarkArray, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_MarkArray, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut otl_MarkArray>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_MarkArray, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> otl_MarkRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_MarkArray, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_MarkArray,
            Option<unsafe extern "C" fn(*const otl_MarkRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_MarkArray,
            Option<
                unsafe extern "C" fn(
                    *const otl_MarkRecord,
                    *const otl_MarkRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_MarkRecord {
    pub init: Option<unsafe extern "C" fn(*mut otl_MarkRecord) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_MarkRecord, *const otl_MarkRecord) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_MarkRecord, *mut otl_MarkRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_MarkRecord) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_MarkRecord, otl_MarkRecord) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_MarkRecord, otl_MarkRecord) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ClassnameHash {
    pub className: sds,
    pub classID: glyphclass_t,
    pub hh: UT_hash_handle,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const json_serialize_mode_packed: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const HASH_INITIAL_NUM_BUCKETS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const HASH_INITIAL_NUM_BUCKETS_LOG2: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const HASH_BKT_CAPACITY_THRESH: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
pub const HASH_SIGNATURE: ::core::ffi::c_uint = 0xa0111fe1 as ::core::ffi::c_uint;
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
unsafe extern "C" fn json_new_position(mut z: pos_t) -> *mut json_value {
    if round(z as ::core::ffi::c_double) == z {
        return json_integer_new(z as int64_t);
    } else {
        return json_double_new(z as ::core::ffi::c_double);
    };
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
#[inline]
unsafe extern "C" fn preserialize(mut x: *mut json_value) -> *mut json_value {
    let mut opts: json_serialize_opts = json_serialize_opts {
        mode: json_serialize_mode_packed,
        opts: 0,
        indent_size: 0,
    };
    let mut preserialize_len: size_t = json_measure_ex(x, opts);
    let mut buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let mut xx: *mut json_value = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as size_t) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = json_pre_serialized;
    return xx;
}
unsafe extern "C" fn deleteMarkArrayItem(mut entry: *mut otl_MarkRecord) {
    otfcc_Handle_dispose(&raw mut (*entry).glyph);
}
static mut gss_typeinfo: __caryll_elementinterface_otl_MarkRecord = {
    __caryll_elementinterface_otl_MarkRecord {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(deleteMarkArrayItem as unsafe extern "C" fn(*mut otl_MarkRecord) -> ()),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn otl_MarkArray_disposeItem(mut arr: *mut otl_MarkArray, mut n: size_t) {
    if gss_typeinfo.dispose.is_some() {
        gss_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_MarkRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_MarkArray_growTo(arr: *mut otl_MarkArray, target: size_t) {
    cvec_grow_to(otl_MarkArray_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_MarkArray_pop(arr: *mut otl_MarkArray) -> otl_MarkRecord {
    cvec_pop(otl_MarkArray_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_MarkArray_copyReplace(mut dst: *mut otl_MarkArray, src: otl_MarkArray) {
    otl_MarkArray_dispose(dst);
    otl_MarkArray_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_MarkArray_copy(
    mut dst: *mut otl_MarkArray,
    mut src: *const otl_MarkArray,
) {
    otl_MarkArray_init(dst);
    otl_MarkArray_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gss_typeinfo.copy.is_some() {
        let mut j: size_t = 0 as size_t;
        while j < (*src).length {
            gss_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_MarkRecord,
                (*src).items.offset(j as isize) as *mut otl_MarkRecord as *const otl_MarkRecord,
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
unsafe extern "C" fn otl_MarkArray_dispose(mut arr: *mut otl_MarkArray) {
    if arr.is_null() {
        return;
    }
    if gss_typeinfo.dispose.is_some() {
        let mut j: size_t = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            gss_typeinfo.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_MarkRecord,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_MarkRecord>();
    (*arr).length = 0 as size_t;
    (*arr).capacity = 0 as size_t;
}
#[inline]
unsafe extern "C" fn otl_MarkArray_replace(mut dst: *mut otl_MarkArray, src: otl_MarkArray) {
    otl_MarkArray_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_MarkArray>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn otl_MarkArray_initCapN(mut arr: *mut otl_MarkArray, mut n: size_t) {
    otl_MarkArray_init(arr);
    otl_MarkArray_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_MarkArray_growToN(arr: *mut otl_MarkArray, target: size_t) {
    cvec_grow_to_n(otl_MarkArray_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_MarkArray_initN(mut arr: *mut otl_MarkArray, mut n: size_t) {
    otl_MarkArray_init(arr);
    otl_MarkArray_growToN(arr, n);
    otl_MarkArray_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_MarkArray_free(mut x: *mut otl_MarkArray) {
    if x.is_null() {
        return;
    }
    otl_MarkArray_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_MarkArray_createN(mut n: size_t) -> *mut otl_MarkArray {
    let mut t: *mut otl_MarkArray =
        malloc(::core::mem::size_of::<otl_MarkArray>() as size_t) as *mut otl_MarkArray;
    otl_MarkArray_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_MarkArray_create() -> *mut otl_MarkArray {
    let mut x: *mut otl_MarkArray =
        malloc(::core::mem::size_of::<otl_MarkArray>() as size_t) as *mut otl_MarkArray;
    otl_MarkArray_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_MarkArray_move(dst: *mut otl_MarkArray, src: *mut otl_MarkArray) {
    cvec_move(otl_MarkArray_as_cvec(dst), otl_MarkArray_as_cvec(src));
}
#[inline]
unsafe fn otl_MarkArray_as_cvec(arr: *mut otl_MarkArray) -> *mut CVecRaw<otl_MarkRecord> {
    arr as *mut CVecRaw<otl_MarkRecord>
}
#[inline]
unsafe extern "C" fn otl_MarkArray_init(arr: *mut otl_MarkArray) {
    cvec_init(otl_MarkArray_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_MarkArray_filterEnv(
    mut arr: *mut otl_MarkArray,
    mut fn_0: Option<unsafe extern "C" fn(*const otl_MarkRecord, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: size_t = 0 as size_t;
    let mut k: size_t = 0 as size_t;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_MarkRecord,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gss_typeinfo.dispose.is_some() {
                gss_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_MarkRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[no_mangle]
pub static mut otl_iMarkArray: __caryll_vectorinterface_otl_MarkArray = {
    __caryll_vectorinterface_otl_MarkArray {
        init: Some(otl_MarkArray_init as unsafe extern "C" fn(*mut otl_MarkArray) -> ()),
        copy: Some(
            otl_MarkArray_copy
                as unsafe extern "C" fn(*mut otl_MarkArray, *const otl_MarkArray) -> (),
        ),
        move_0: Some(
            otl_MarkArray_move
                as unsafe extern "C" fn(*mut otl_MarkArray, *mut otl_MarkArray) -> (),
        ),
        dispose: Some(otl_MarkArray_dispose as unsafe extern "C" fn(*mut otl_MarkArray) -> ()),
        replace: Some(
            otl_MarkArray_replace as unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkArray) -> (),
        ),
        copyReplace: Some(
            otl_MarkArray_copyReplace
                as unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkArray) -> (),
        ),
        create: Some(otl_MarkArray_create),
        free: Some(otl_MarkArray_free as unsafe extern "C" fn(*mut otl_MarkArray) -> ()),
        initN: Some(otl_MarkArray_initN as unsafe extern "C" fn(*mut otl_MarkArray, size_t) -> ()),
        initCapN: Some(
            otl_MarkArray_initCapN as unsafe extern "C" fn(*mut otl_MarkArray, size_t) -> (),
        ),
        createN: Some(otl_MarkArray_createN as unsafe extern "C" fn(size_t) -> *mut otl_MarkArray),
        fill: Some(otl_MarkArray_fill as unsafe extern "C" fn(*mut otl_MarkArray, size_t) -> ()),
        clear: Some(otl_MarkArray_dispose as unsafe extern "C" fn(*mut otl_MarkArray) -> ()),
        push: Some(
            otl_MarkArray_push as unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkRecord) -> (),
        ),
        shrinkToFit: Some(
            otl_MarkArray_shrinkToFit as unsafe extern "C" fn(*mut otl_MarkArray) -> (),
        ),
        pop: Some(otl_MarkArray_pop as unsafe extern "C" fn(*mut otl_MarkArray) -> otl_MarkRecord),
        disposeItem: Some(
            otl_MarkArray_disposeItem as unsafe extern "C" fn(*mut otl_MarkArray, size_t) -> (),
        ),
        filterEnv: Some(
            otl_MarkArray_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_MarkArray,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_MarkRecord,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_MarkArray_sort
                as unsafe extern "C" fn(
                    *mut otl_MarkArray,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_MarkRecord,
                            *const otl_MarkRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_MarkArray_shrinkToFit(mut arr: *mut otl_MarkArray) {
    otl_MarkArray_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_MarkArray_resizeTo(arr: *mut otl_MarkArray, target: size_t) {
    cvec_resize_to(otl_MarkArray_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_MarkArray_sort(
    mut arr: *mut otl_MarkArray,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_MarkRecord, *const otl_MarkRecord) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_MarkRecord>() as size_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_MarkRecord,
                    *const otl_MarkRecord,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_MarkArray_fill(mut arr: *mut otl_MarkArray, mut n: size_t) {
    while (*arr).length < n {
        let mut x: otl_MarkRecord = otl_MarkRecord {
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            markClass: 0,
            anchor: otl_Anchor {
                present: false,
                x: 0.,
                y: 0.,
            },
        };
        if gss_typeinfo.init.is_some() {
            gss_typeinfo.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_MarkRecord>() as size_t,
            );
        }
        otl_MarkArray_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_MarkArray_push(arr: *mut otl_MarkArray, elem: otl_MarkRecord) {
    cvec_push(otl_MarkArray_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_MarkArray_grow(arr: *mut otl_MarkArray) {
    cvec_grow(otl_MarkArray_as_cvec(arr));
}
#[no_mangle]
pub unsafe extern "C" fn otl_readMarkArray(
    mut array: *mut otl_MarkArray,
    mut cov: *mut otl_Coverage,
    mut data: font_file_pointer,
    mut tableLength: uint32_t,
    mut offset: uint32_t,
) {
    let mut markCount: glyphid_t = 0;
    if !(tableLength < offset.wrapping_add(2 as uint32_t)) {
        markCount = read_16u(data.offset(offset as isize) as *const uint8_t) as glyphid_t;
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as ::core::ffi::c_int) < markCount as ::core::ffi::c_int {
            let mut markClass: glyphclass_t = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset((j as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as *const uint8_t,
            ) as glyphclass_t;
            let mut delta: uint16_t = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset((j as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const uint8_t,
            );
            if delta != 0 {
                otl_iMarkArray.push.expect("non-null function pointer")(
                    array,
                    otl_MarkRecord {
                        glyph: otfcc_Handle_dup(
                            *(*cov).glyphs.offset(j as isize) as otfcc_Handle,
                        ) as otfcc_GlyphHandle,
                        markClass: markClass,
                        anchor: otl_read_anchor(
                            data,
                            tableLength,
                            offset.wrapping_add(delta as uint32_t),
                        ),
                    },
                );
            } else {
                otl_iMarkArray.push.expect("non-null function pointer")(
                    array,
                    otl_MarkRecord {
                        glyph: otfcc_Handle_dup(
                            *(*cov).glyphs.offset(j as isize) as otfcc_Handle,
                        ) as otfcc_GlyphHandle,
                        markClass: markClass,
                        anchor: otl_anchor_absent(),
                    },
                );
            }
            j = j.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn compare_classHash(
    mut a: *mut otl_ClassnameHash,
    mut b: *mut otl_ClassnameHash,
) -> ::core::ffi::c_int {
    return strcmp(
        (*a).className as *const ::core::ffi::c_char,
        (*b).className as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn otl_parseMarkArray(
    mut _marks: *mut json_value,
    mut array: *mut otl_MarkArray,
    mut h: *mut *mut otl_ClassnameHash,
    mut _options: *const otfcc_Options,
) {
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*_marks).u.object.length {
        let mut mark: otl_MarkRecord = otl_MarkRecord {
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            markClass: 0,
            anchor: otl_Anchor {
                present: false,
                x: 0.,
                y: 0.,
            },
        };
        let mut gname: *mut ::core::ffi::c_char =
            (*(*_marks).u.object.values.offset(j as isize)).name;
        let mut anchorRecord: *mut json_value =
            (*(*_marks).u.object.values.offset(j as isize)).value as *mut json_value;
        mark.glyph = handle_fromName(sdsnewlen(
            gname as *const ::core::ffi::c_void,
            (*(*_marks).u.object.values.offset(j as isize)).name_length as size_t,
        )) as otfcc_GlyphHandle;
        mark.markClass = 0 as glyphclass_t;
        mark.anchor = otl_anchor_absent();
        if anchorRecord.is_null()
            || (*anchorRecord).type_0 as ::core::ffi::c_uint
                != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            otl_iMarkArray.push.expect("non-null function pointer")(array, mark);
        } else {
            let mut _className: *mut json_value = json_obj_get_type(
                anchorRecord,
                b"class\0" as *const u8 as *const ::core::ffi::c_char,
                json_string,
            );
            if _className.is_null() {
                otl_iMarkArray.push.expect("non-null function pointer")(array, mark);
            } else {
                let mut className: sds = sdsnewlen(
                    (*_className).u.string.ptr as *const ::core::ffi::c_void,
                    (*_className).u.string.length as size_t,
                );
                let mut s: *mut otl_ClassnameHash = ::core::ptr::null_mut::<otl_ClassnameHash>();
                let mut _hf_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i: ::core::ffi::c_uint = 0;
                let mut _hj_j: ::core::ffi::c_uint = 0;
                let mut _hj_k: ::core::ffi::c_uint = 0;
                let mut _hj_key: *const ::core::ffi::c_uchar =
                    className as *const ::core::ffi::c_uchar;
                _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i = _hj_j;
                _hj_k = strlen(className as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                while _hj_k >= 12 as ::core::ffi::c_uint {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_i = _hj_i.wrapping_sub(_hj_j);
                    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                    _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
                    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                    _hj_j = _hj_j.wrapping_sub(_hj_i);
                    _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                    _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
                    _hj_i = _hj_i.wrapping_sub(_hj_j);
                    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                    _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
                    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                    _hj_j = _hj_j.wrapping_sub(_hj_i);
                    _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                    _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
                    _hj_i = _hj_i.wrapping_sub(_hj_j);
                    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                    _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
                    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                    _hj_j = _hj_j.wrapping_sub(_hj_i);
                    _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                    _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
                    _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
                    _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
                }
                _hf_hashv = _hf_hashv.wrapping_add(
                    strlen(className as *const ::core::ffi::c_char) as ::core::ffi::c_uint
                );
                let mut current_block_55: u64;
                match _hj_k {
                    11 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_55 = 4184970055425330224;
                    }
                    10 => {
                        current_block_55 = 4184970055425330224;
                    }
                    9 => {
                        current_block_55 = 13800538852034404314;
                    }
                    8 => {
                        current_block_55 = 15463853910180707538;
                    }
                    7 => {
                        current_block_55 = 17555299131552860298;
                    }
                    6 => {
                        current_block_55 = 3137062391300010043;
                    }
                    5 => {
                        current_block_55 = 14607299304144994639;
                    }
                    4 => {
                        current_block_55 = 1223993795581701498;
                    }
                    3 => {
                        current_block_55 = 4239524923856774895;
                    }
                    2 => {
                        current_block_55 = 9705619680375831020;
                    }
                    1 => {
                        current_block_55 = 7019012577490997641;
                    }
                    _ => {
                        current_block_55 = 5141539773904409130;
                    }
                }
                match current_block_55 {
                    4184970055425330224 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_55 = 13800538852034404314;
                    }
                    _ => {}
                }
                match current_block_55 {
                    13800538852034404314 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_55 = 15463853910180707538;
                    }
                    _ => {}
                }
                match current_block_55 {
                    15463853910180707538 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_55 = 17555299131552860298;
                    }
                    _ => {}
                }
                match current_block_55 {
                    17555299131552860298 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_55 = 3137062391300010043;
                    }
                    _ => {}
                }
                match current_block_55 {
                    3137062391300010043 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_55 = 14607299304144994639;
                    }
                    _ => {}
                }
                match current_block_55 {
                    14607299304144994639 => {
                        _hj_j = _hj_j
                            .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_55 = 1223993795581701498;
                    }
                    _ => {}
                }
                match current_block_55 {
                    1223993795581701498 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_55 = 4239524923856774895;
                    }
                    _ => {}
                }
                match current_block_55 {
                    4239524923856774895 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_55 = 9705619680375831020;
                    }
                    _ => {}
                }
                match current_block_55 {
                    9705619680375831020 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_55 = 7019012577490997641;
                    }
                    _ => {}
                }
                match current_block_55 {
                    7019012577490997641 => {
                        _hj_i = _hj_i
                            .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
                s = ::core::ptr::null_mut::<otl_ClassnameHash>();
                if !(*h).is_null() {
                    let mut _hf_bkt: ::core::ffi::c_uint = 0;
                    _hf_bkt = _hf_hashv
                        & (*(**h).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize))
                            .hh_head
                            .is_null()
                        {
                            s = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut otl_ClassnameHash
                                as *mut otl_ClassnameHash;
                        } else {
                            s = ::core::ptr::null_mut::<otl_ClassnameHash>();
                        }
                        while !s.is_null() {
                            if (*s).hh.hashv == _hf_hashv
                                && (*s).hh.keylen
                                    == strlen(className as *const ::core::ffi::c_char)
                                        as ::core::ffi::c_uint
                            {
                                if memcmp(
                                    (*s).hh.key,
                                    className as *const ::core::ffi::c_void,
                                    strlen(className as *const ::core::ffi::c_char)
                                        as ::core::ffi::c_uint
                                        as size_t,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if !(*s).hh.hh_next.is_null() {
                                s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                    .offset(-(*(**h).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut otl_ClassnameHash
                                    as *mut otl_ClassnameHash;
                            } else {
                                s = ::core::ptr::null_mut::<otl_ClassnameHash>();
                            }
                        }
                    }
                }
                if s.is_null() {
                    s = __caryll_allocate_clean(
                        ::core::mem::size_of::<otl_ClassnameHash>() as size_t,
                        61 as ::core::ffi::c_ulong,
                    ) as *mut otl_ClassnameHash;
                    (*s).className = className;
                    (*s).classID = (if !(*h).is_null() {
                        (*(**h).hh.tbl).num_items
                    } else {
                        0 as ::core::ffi::c_uint
                    }) as glyphclass_t;
                    let mut _ha_hashv: ::core::ffi::c_uint = 0;
                    let mut _hj_i_0: ::core::ffi::c_uint = 0;
                    let mut _hj_j_0: ::core::ffi::c_uint = 0;
                    let mut _hj_k_0: ::core::ffi::c_uint = 0;
                    let mut _hj_key_0: *const ::core::ffi::c_uchar =
                        (*s).className.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_uchar;
                    _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                    _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                    _hj_i_0 = _hj_j_0;
                    _hj_k_0 =
                        strlen((*s).className as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                    while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                ),
                        );
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                ),
                        );
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_0.offset(11 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                ),
                        );
                        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                        _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                        _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                        _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
                        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                        _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                        _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                        _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
                        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                        _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                        _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                        _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
                        _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
                        _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
                    }
                    _ha_hashv = _ha_hashv
                        .wrapping_add(strlen((*s).className as *const ::core::ffi::c_char)
                            as ::core::ffi::c_uint);
                    let mut current_block_172: u64;
                    match _hj_k_0 {
                        11 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_172 = 898468936263527080;
                        }
                        10 => {
                            current_block_172 = 898468936263527080;
                        }
                        9 => {
                            current_block_172 = 9698739436995956065;
                        }
                        8 => {
                            current_block_172 = 10600459493909746493;
                        }
                        7 => {
                            current_block_172 = 9773837153719584703;
                        }
                        6 => {
                            current_block_172 = 6790636896960429817;
                        }
                        5 => {
                            current_block_172 = 1376536996662481425;
                        }
                        4 => {
                            current_block_172 = 16671533464018076928;
                        }
                        3 => {
                            current_block_172 = 6609393363672095876;
                        }
                        2 => {
                            current_block_172 = 9029570263382640504;
                        }
                        1 => {
                            current_block_172 = 9698912694449281435;
                        }
                        _ => {
                            current_block_172 = 5219368551394180541;
                        }
                    }
                    match current_block_172 {
                        898468936263527080 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_172 = 9698739436995956065;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        9698739436995956065 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_172 = 10600459493909746493;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        10600459493909746493 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_172 = 9773837153719584703;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        9773837153719584703 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_172 = 6790636896960429817;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        6790636896960429817 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_172 = 1376536996662481425;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        1376536996662481425 => {
                            _hj_j_0 = _hj_j_0
                                .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_172 = 16671533464018076928;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        16671533464018076928 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_172 = 6609393363672095876;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        6609393363672095876 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_172 = 9029570263382640504;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        9029570263382640504 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_172 = 9698912694449281435;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        9698912694449281435 => {
                            _hj_i_0 = _hj_i_0
                                .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                        }
                        _ => {}
                    }
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                    _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                    _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                    _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                    _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                    _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                    _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                    _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                    _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                    _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                    _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                    _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                    _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                    _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                    _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                    _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
                    (*s).hh.hashv = _ha_hashv;
                    (*s).hh.key = (*s).className.offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void;
                    (*s).hh.keylen =
                        strlen((*s).className as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                    if (*h).is_null() {
                        (*s).hh.next = NULL;
                        (*s).hh.prev = NULL;
                        (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                            as *mut UT_hash_table
                            as *mut UT_hash_table;
                        if (*s).hh.tbl.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*s).hh.tbl as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                ::core::mem::size_of::<UT_hash_table>() as size_t,
                            );
                            (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                            (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                            (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                            (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                                .offset_from(s as *mut ::core::ffi::c_char)
                                as ::core::ffi::c_long
                                as ptrdiff_t;
                            (*(*s).hh.tbl).buckets =
                                malloc((32 as size_t).wrapping_mul(::core::mem::size_of::<
                                    UT_hash_bucket,
                                >(
                                )
                                    as size_t))
                                    as *mut UT_hash_bucket;
                            (*(*s).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                            if (*(*s).hh.tbl).buckets.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (32 as size_t).wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as size_t,
                                    ),
                                );
                            }
                        }
                        *h = s;
                    } else {
                        (*s).hh.tbl = (**h).hh.tbl;
                        (*s).hh.next = NULL;
                        (*s).hh.prev = ((*(**h).hh.tbl).tail as *mut ::core::ffi::c_char)
                            .offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void;
                        (*(*(**h).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                        (*(**h).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                    }
                    let mut _ha_bkt: ::core::ffi::c_uint = 0;
                    (*(**h).hh.tbl).num_items = (*(**h).hh.tbl).num_items.wrapping_add(1);
                    _ha_bkt = _ha_hashv
                        & (*(**h).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _ha_head: *mut UT_hash_bucket =
                        (*(**h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                    (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                    (*s).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
                    (*s).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                    if !(*_ha_head).hh_head.is_null() {
                        (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UT_hash_handle;
                    }
                    (*_ha_head).hh_head = &raw mut (*s).hh as *mut UT_hash_handle;
                    if (*_ha_head).count
                        >= (*_ha_head)
                            .expand_mult
                            .wrapping_add(1 as ::core::ffi::c_uint)
                            .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                        && (*(*s).hh.tbl).noexpand == 0
                    {
                        let mut _he_bkt: ::core::ffi::c_uint = 0;
                        let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                        let mut _he_thh: *mut UT_hash_handle =
                            ::core::ptr::null_mut::<UT_hash_handle>();
                        let mut _he_hh_nxt: *mut UT_hash_handle =
                            ::core::ptr::null_mut::<UT_hash_handle>();
                        let mut _he_new_buckets: *mut UT_hash_bucket =
                            ::core::ptr::null_mut::<UT_hash_bucket>();
                        let mut _he_newbkt: *mut UT_hash_bucket =
                            ::core::ptr::null_mut::<UT_hash_bucket>();
                        _he_new_buckets = malloc(
                            (2 as size_t)
                                .wrapping_mul((*(*s).hh.tbl).num_buckets as size_t)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                        ) as *mut UT_hash_bucket;
                        if _he_new_buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                _he_new_buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (2 as size_t)
                                    .wrapping_mul((*(*s).hh.tbl).num_buckets as size_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as size_t
                                    ),
                            );
                            (*(*s).hh.tbl).ideal_chain_maxlen = ((*(*s).hh.tbl).num_items
                                >> (*(*s).hh.tbl)
                                    .log2_num_buckets
                                    .wrapping_add(1 as ::core::ffi::c_uint))
                            .wrapping_add(
                                if (*(*s).hh.tbl).num_items
                                    & (*(*s).hh.tbl)
                                        .num_buckets
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                    != 0 as ::core::ffi::c_uint
                                {
                                    1 as ::core::ffi::c_uint
                                } else {
                                    0 as ::core::ffi::c_uint
                                },
                            );
                            (*(*s).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                            _he_bkt_i = 0 as ::core::ffi::c_uint;
                            while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                                _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize))
                                    .hh_head
                                    as *mut UT_hash_handle;
                                while !_he_thh.is_null() {
                                    _he_hh_nxt = (*_he_thh).hh_next;
                                    _he_bkt = (*_he_thh).hashv
                                        & (*(*s).hh.tbl)
                                            .num_buckets
                                            .wrapping_mul(2 as ::core::ffi::c_uint)
                                            .wrapping_sub(1 as ::core::ffi::c_uint);
                                    _he_newbkt = _he_new_buckets.offset(_he_bkt as isize)
                                        as *mut UT_hash_bucket;
                                    (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                    if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                        (*(*s).hh.tbl).nonideal_items =
                                            (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                        (*_he_newbkt).expand_mult = (*_he_newbkt)
                                            .count
                                            .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
                                    }
                                    (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                                    (*_he_thh).hh_next =
                                        (*_he_newbkt).hh_head as *mut UT_hash_handle;
                                    if !(*_he_newbkt).hh_head.is_null() {
                                        (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                                    }
                                    (*_he_newbkt).hh_head = _he_thh as *mut UT_hash_handle;
                                    _he_thh = _he_hh_nxt;
                                }
                                _he_bkt_i = _he_bkt_i.wrapping_add(1);
                            }
                            free((*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void);
                            (*(*s).hh.tbl).num_buckets = (*(*s).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint);
                            (*(*s).hh.tbl).log2_num_buckets =
                                (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
                            (*(*s).hh.tbl).buckets = _he_new_buckets;
                            (*(*s).hh.tbl).ineff_expands = if (*(*s).hh.tbl).nonideal_items
                                > (*(*s).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                            {
                                (*(*s).hh.tbl)
                                    .ineff_expands
                                    .wrapping_add(1 as ::core::ffi::c_uint)
                            } else {
                                0 as ::core::ffi::c_uint
                            };
                            if (*(*s).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                                (*(*s).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                            }
                        }
                    }
                } else {
                    sdsfree(className);
                }
                mark.markClass = (*s).classID;
                mark.anchor.present = true;
                mark.anchor.x = json_obj_getnum(
                    anchorRecord,
                    b"x\0" as *const u8 as *const ::core::ffi::c_char,
                ) as pos_t;
                mark.anchor.y = json_obj_getnum(
                    anchorRecord,
                    b"y\0" as *const u8 as *const ::core::ffi::c_char,
                ) as pos_t;
                otl_iMarkArray.push.expect("non-null function pointer")(array, mark);
            }
        }
        j = j.wrapping_add(1);
    }
    let mut _hs_i: ::core::ffi::c_uint = 0;
    let mut _hs_looping: ::core::ffi::c_uint = 0;
    let mut _hs_nmerges: ::core::ffi::c_uint = 0;
    let mut _hs_insize: ::core::ffi::c_uint = 0;
    let mut _hs_psize: ::core::ffi::c_uint = 0;
    let mut _hs_qsize: ::core::ffi::c_uint = 0;
    let mut _hs_p: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_q: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_e: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_list: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_tail: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    if !(*h).is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (**h).hh as *mut UT_hash_handle;
        while _hs_looping != 0 as ::core::ffi::c_uint {
            _hs_p = _hs_list;
            _hs_list = ::core::ptr::null_mut::<UT_hash_handle>();
            _hs_tail = ::core::ptr::null_mut::<UT_hash_handle>();
            _hs_nmerges = 0 as ::core::ffi::c_uint;
            while !_hs_p.is_null() {
                _hs_nmerges = _hs_nmerges.wrapping_add(1);
                _hs_q = _hs_p;
                _hs_psize = 0 as ::core::ffi::c_uint;
                _hs_i = 0 as ::core::ffi::c_uint;
                while _hs_i < _hs_insize {
                    _hs_psize = _hs_psize.wrapping_add(1);
                    _hs_q = (if !(*_hs_q).next.is_null() {
                        ((*_hs_q).next as *mut ::core::ffi::c_char)
                            .offset((*(**h).hh.tbl).hho)
                            as *mut UT_hash_handle
                    } else {
                        ::core::ptr::null_mut::<UT_hash_handle>()
                    }) as *mut UT_hash_handle;
                    if _hs_q.is_null() {
                        break;
                    }
                    _hs_i = _hs_i.wrapping_add(1);
                }
                _hs_qsize = _hs_insize;
                while _hs_psize != 0 as ::core::ffi::c_uint
                    || _hs_qsize != 0 as ::core::ffi::c_uint && !_hs_q.is_null()
                {
                    if _hs_psize == 0 as ::core::ffi::c_uint {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(**h).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    } else if _hs_qsize == 0 as ::core::ffi::c_uint || _hs_q.is_null() {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(**h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if compare_classHash(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut otl_ClassnameHash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut otl_ClassnameHash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(**h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(**h).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    } else {
                        _hs_list = _hs_e;
                    }
                    if !_hs_e.is_null() {
                        (*_hs_e).prev = if !_hs_tail.is_null() {
                            (_hs_tail as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    }
                    _hs_tail = _hs_e;
                }
                _hs_p = _hs_q;
            }
            if !_hs_tail.is_null() {
                (*_hs_tail).next = NULL;
            }
            if _hs_nmerges <= 1 as ::core::ffi::c_uint {
                _hs_looping = 0 as ::core::ffi::c_uint;
                (*(**h).hh.tbl).tail = _hs_tail;
                *h = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut otl_ClassnameHash
                    as *mut otl_ClassnameHash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut jAnchorIndex: glyphid_t = 0 as glyphid_t;
    let mut s_0: *mut otl_ClassnameHash = ::core::ptr::null_mut::<otl_ClassnameHash>();
    s_0 = *h;
    while !s_0.is_null() {
        (*s_0).classID = jAnchorIndex as glyphclass_t;
        jAnchorIndex = jAnchorIndex.wrapping_add(1);
        s_0 = (*s_0).hh.next as *mut otl_ClassnameHash;
    }
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as size_t) < (*array).length {
        if (*(*array).items.offset(j_0 as isize)).anchor.present {
            let mut anchorRecord_0: *mut json_value =
                (*(*_marks).u.object.values.offset(j_0 as isize)).value as *mut json_value;
            let mut _className_0: *mut json_value = json_obj_get_type(
                anchorRecord_0,
                b"class\0" as *const u8 as *const ::core::ffi::c_char,
                json_string,
            );
            let mut className_0: sds = sdsnewlen(
                (*_className_0).u.string.ptr as *const ::core::ffi::c_void,
                (*_className_0).u.string.length as size_t,
            );
            let mut s_1: *mut otl_ClassnameHash = ::core::ptr::null_mut::<otl_ClassnameHash>();
            let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
            let mut _hj_i_1: ::core::ffi::c_uint = 0;
            let mut _hj_j_1: ::core::ffi::c_uint = 0;
            let mut _hj_k_1: ::core::ffi::c_uint = 0;
            let mut _hj_key_1: *const ::core::ffi::c_uchar =
                className_0 as *const ::core::ffi::c_uchar;
            _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_1 = _hj_j_1;
            _hj_k_1 = strlen(className_0 as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
            while _hj_k_1 >= 12 as ::core::ffi::c_uint {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
                _hj_i_1 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
                _hf_hashv_0 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
                _hj_i_1 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
                _hf_hashv_0 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
                _hj_i_1 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
                _hf_hashv_0 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
                _hj_key_1 = _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
                _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                strlen(className_0 as *const ::core::ffi::c_char) as ::core::ffi::c_uint,
            );
            let mut current_block_445: u64;
            match _hj_k_1 {
                11 => {
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                        (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_445 = 4247882375651910782;
                }
                10 => {
                    current_block_445 = 4247882375651910782;
                }
                9 => {
                    current_block_445 = 16226128822898203720;
                }
                8 => {
                    current_block_445 = 9931377106148496891;
                }
                7 => {
                    current_block_445 = 14712804663128342644;
                }
                6 => {
                    current_block_445 = 5445440423012481275;
                }
                5 => {
                    current_block_445 = 62599083018845058;
                }
                4 => {
                    current_block_445 = 161668369318445431;
                }
                3 => {
                    current_block_445 = 10547560897237185998;
                }
                2 => {
                    current_block_445 = 1296394692977688829;
                }
                1 => {
                    current_block_445 = 15921629307266798929;
                }
                _ => {
                    current_block_445 = 18272884058186558579;
                }
            }
            match current_block_445 {
                4247882375651910782 => {
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                        (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_445 = 16226128822898203720;
                }
                _ => {}
            }
            match current_block_445 {
                16226128822898203720 => {
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                        (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_445 = 9931377106148496891;
                }
                _ => {}
            }
            match current_block_445 {
                9931377106148496891 => {
                    _hj_j_1 = _hj_j_1.wrapping_add(
                        (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_445 = 14712804663128342644;
                }
                _ => {}
            }
            match current_block_445 {
                14712804663128342644 => {
                    _hj_j_1 = _hj_j_1.wrapping_add(
                        (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_445 = 5445440423012481275;
                }
                _ => {}
            }
            match current_block_445 {
                5445440423012481275 => {
                    _hj_j_1 = _hj_j_1.wrapping_add(
                        (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_445 = 62599083018845058;
                }
                _ => {}
            }
            match current_block_445 {
                62599083018845058 => {
                    _hj_j_1 = _hj_j_1
                        .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_445 = 161668369318445431;
                }
                _ => {}
            }
            match current_block_445 {
                161668369318445431 => {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_445 = 10547560897237185998;
                }
                _ => {}
            }
            match current_block_445 {
                10547560897237185998 => {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_445 = 1296394692977688829;
                }
                _ => {}
            }
            match current_block_445 {
                1296394692977688829 => {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_445 = 15921629307266798929;
                }
                _ => {}
            }
            match current_block_445 {
                15921629307266798929 => {
                    _hj_i_1 = _hj_i_1
                        .wrapping_add(*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                }
                _ => {}
            }
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
            s_1 = ::core::ptr::null_mut::<otl_ClassnameHash>();
            if !(*h).is_null() {
                let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
                _hf_bkt_0 = _hf_hashv_0
                    & (*(**h).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt_0 as isize))
                        .hh_head
                        .is_null()
                    {
                        s_1 = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt_0 as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut otl_ClassnameHash
                            as *mut otl_ClassnameHash;
                    } else {
                        s_1 = ::core::ptr::null_mut::<otl_ClassnameHash>();
                    }
                    while !s_1.is_null() {
                        if (*s_1).hh.hashv == _hf_hashv_0
                            && (*s_1).hh.keylen
                                == strlen(className_0 as *const ::core::ffi::c_char)
                                    as ::core::ffi::c_uint
                        {
                            if memcmp(
                                (*s_1).hh.key,
                                className_0 as *const ::core::ffi::c_void,
                                strlen(className_0 as *const ::core::ffi::c_char)
                                    as ::core::ffi::c_uint
                                    as size_t,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*s_1).hh.hh_next.is_null() {
                            s_1 = ((*s_1).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut otl_ClassnameHash
                                as *mut otl_ClassnameHash;
                        } else {
                            s_1 = ::core::ptr::null_mut::<otl_ClassnameHash>();
                        }
                    }
                }
            }
            if !s_1.is_null() {
                (*(*array).items.offset(j_0 as isize)).markClass = (*s_1).classID;
            } else {
                (*(*array).items.offset(j_0 as isize)).markClass = 0 as glyphclass_t;
            }
            sdsfree(className_0);
        }
        j_0 = j_0.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otl_anchor_absent() -> otl_Anchor {
    let mut anchor: otl_Anchor = otl_Anchor {
        present: false,
        x: 0 as ::core::ffi::c_int as pos_t,
        y: 0 as ::core::ffi::c_int as pos_t,
    };
    return anchor;
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_anchor(
    mut data: font_file_pointer,
    mut tableLength: uint32_t,
    mut offset: uint32_t,
) -> otl_Anchor {
    let mut anchor: otl_Anchor = otl_Anchor {
        present: false,
        x: 0 as ::core::ffi::c_int as pos_t,
        y: 0 as ::core::ffi::c_int as pos_t,
    };
    if tableLength < offset.wrapping_add(6 as uint32_t) {
        anchor.present = false;
        anchor.x = 0 as ::core::ffi::c_int as pos_t;
        anchor.y = 0 as ::core::ffi::c_int as pos_t;
        return anchor;
    } else {
        anchor.present = true;
        anchor.x = read_16s(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const uint8_t,
        ) as pos_t;
        anchor.y = read_16s(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const uint8_t,
        ) as pos_t;
        return anchor;
    };
}
#[no_mangle]
pub unsafe extern "C" fn otl_dump_anchor(mut a: otl_Anchor) -> *mut json_value {
    if a.present {
        let mut v: *mut json_value = json_object_new(2 as size_t);
        json_object_push(
            v,
            b"x\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(a.x),
        );
        json_object_push(
            v,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(a.y),
        );
        return v;
    } else {
        return json_null_new();
    };
}
#[no_mangle]
pub unsafe extern "C" fn otl_parse_anchor(mut v: *mut json_value) -> otl_Anchor {
    let mut anchor: otl_Anchor = otl_Anchor {
        present: false,
        x: 0 as ::core::ffi::c_int as pos_t,
        y: 0 as ::core::ffi::c_int as pos_t,
    };
    if v.is_null()
        || (*v).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return anchor;
    }
    anchor.present = true;
    anchor.x = json_obj_getnum_fallback(
        v,
        b"x\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as pos_t;
    anchor.y = json_obj_getnum_fallback(
        v,
        b"y\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as pos_t;
    return anchor;
}
#[no_mangle]
pub unsafe extern "C" fn bkFromAnchor(mut a: otl_Anchor) -> *mut bk_Block {
    if !a.present {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    return bk_new_Block(
        b16 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        b16 as ::core::ffi::c_int,
        a.x as int16_t as ::core::ffi::c_int,
        b16 as ::core::ffi::c_int,
        a.y as int16_t as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub static mut FORMAT_DX: uint8_t = 1 as uint8_t;
#[no_mangle]
pub static mut FORMAT_DY: uint8_t = 2 as uint8_t;
#[no_mangle]
pub static mut FORMAT_DWIDTH: uint8_t = 4 as uint8_t;
#[no_mangle]
pub static mut FORMAT_DHEIGHT: uint8_t = 8 as uint8_t;
#[no_mangle]
pub static mut bits_in: [uint8_t; 256] = [
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn position_format_length(mut format: uint16_t) -> uint8_t {
    return ((bits_in[(format as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn position_zero() -> otl_PositionValue {
    let mut v: otl_PositionValue = otl_PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        dWidth: 0.0f64,
        dHeight: 0.0f64,
    };
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn read_gpos_value(
    mut data: font_file_pointer,
    mut tableLength: uint32_t,
    mut offset: uint32_t,
    mut format: uint16_t,
) -> otl_PositionValue {
    let mut v: otl_PositionValue = otl_PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        dWidth: 0.0f64,
        dHeight: 0.0f64,
    };
    if tableLength < offset.wrapping_add(position_format_length(format) as uint32_t) {
        return v;
    }
    if format as ::core::ffi::c_int & FORMAT_DX as ::core::ffi::c_int != 0 {
        v.dx = read_16s(data.offset(offset as isize) as *const uint8_t) as pos_t;
        offset = offset.wrapping_add(2 as uint32_t);
    }
    if format as ::core::ffi::c_int & FORMAT_DY as ::core::ffi::c_int != 0 {
        v.dy = read_16s(data.offset(offset as isize) as *const uint8_t) as pos_t;
        offset = offset.wrapping_add(2 as uint32_t);
    }
    if format as ::core::ffi::c_int & FORMAT_DWIDTH as ::core::ffi::c_int != 0 {
        v.dWidth = read_16s(data.offset(offset as isize) as *const uint8_t) as pos_t;
        offset = offset.wrapping_add(2 as uint32_t);
    }
    if format as ::core::ffi::c_int & FORMAT_DHEIGHT as ::core::ffi::c_int != 0 {
        v.dHeight = read_16s(data.offset(offset as isize) as *const uint8_t) as pos_t;
        offset = offset.wrapping_add(2 as uint32_t);
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gpos_dump_value(mut value: otl_PositionValue) -> *mut json_value {
    let mut v: *mut json_value = json_object_new(4 as size_t);
    if value.dx != 0. {
        json_object_push(
            v,
            b"dx\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(value.dx),
        );
    }
    if value.dy != 0. {
        json_object_push(
            v,
            b"dy\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(value.dy),
        );
    }
    if value.dWidth != 0. {
        json_object_push(
            v,
            b"dWidth\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(value.dWidth),
        );
    }
    if value.dHeight != 0. {
        json_object_push(
            v,
            b"dHeight\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(value.dHeight),
        );
    }
    return preserialize(v);
}
#[no_mangle]
pub unsafe extern "C" fn gpos_parse_value(mut pos: *mut json_value) -> otl_PositionValue {
    let mut v: otl_PositionValue = otl_PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        dWidth: 0.0f64,
        dHeight: 0.0f64,
    };
    if pos.is_null()
        || (*pos).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return v;
    }
    v.dx = json_obj_getnum(pos, b"dx\0" as *const u8 as *const ::core::ffi::c_char) as pos_t;
    v.dy = json_obj_getnum(pos, b"dy\0" as *const u8 as *const ::core::ffi::c_char) as pos_t;
    v.dWidth =
        json_obj_getnum(pos, b"dWidth\0" as *const u8 as *const ::core::ffi::c_char) as pos_t;
    v.dHeight =
        json_obj_getnum(pos, b"dHeight\0" as *const u8 as *const ::core::ffi::c_char) as pos_t;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn required_position_format(mut v: otl_PositionValue) -> uint8_t {
    return ((if v.dx != 0. {
        FORMAT_DX as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) | (if v.dy != 0. {
        FORMAT_DY as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) | (if v.dWidth != 0. {
        FORMAT_DWIDTH as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) | (if v.dHeight != 0. {
        FORMAT_DHEIGHT as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    })) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn write_gpos_value(
    mut buf: *mut caryll_Buffer,
    mut v: otl_PositionValue,
    mut format: uint16_t,
) {
    if format as ::core::ffi::c_int & FORMAT_DX as ::core::ffi::c_int != 0 {
        bufwrite16b(buf, v.dx as int16_t as uint16_t);
    }
    if format as ::core::ffi::c_int & FORMAT_DY as ::core::ffi::c_int != 0 {
        bufwrite16b(buf, v.dy as int16_t as uint16_t);
    }
    if format as ::core::ffi::c_int & FORMAT_DWIDTH as ::core::ffi::c_int != 0 {
        bufwrite16b(buf, v.dWidth as int16_t as uint16_t);
    }
    if format as ::core::ffi::c_int & FORMAT_DHEIGHT as ::core::ffi::c_int != 0 {
        bufwrite16b(buf, v.dHeight as int16_t as uint16_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bk_gpos_value(
    mut v: otl_PositionValue,
    mut format: uint16_t,
) -> *mut bk_Block {
    let mut b: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
    if format as ::core::ffi::c_int & FORMAT_DX as ::core::ffi::c_int != 0 {
        bk_push(
            b,
            b16 as ::core::ffi::c_int,
            v.dx as int16_t as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
    }
    if format as ::core::ffi::c_int & FORMAT_DY as ::core::ffi::c_int != 0 {
        bk_push(
            b,
            b16 as ::core::ffi::c_int,
            v.dy as int16_t as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
    }
    if format as ::core::ffi::c_int & FORMAT_DWIDTH as ::core::ffi::c_int != 0 {
        bk_push(
            b,
            b16 as ::core::ffi::c_int,
            v.dWidth as int16_t as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
    }
    if format as ::core::ffi::c_int & FORMAT_DHEIGHT as ::core::ffi::c_int != 0 {
        bk_push(
            b,
            b16 as ::core::ffi::c_int,
            v.dHeight as int16_t as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
    }
    return b;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
