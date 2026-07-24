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
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t);
    fn json_array_new(length: size_t) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: size_t) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: int64_t) -> *mut json_value;
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
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
pub type int32_t = __int32_t;
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
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type glyphsize_t = uint16_t;
pub type tableid_t = uint16_t;
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
pub struct gasp_Record {
    pub rangeMaxPPEM: glyphsize_t,
    pub dogray: bool,
    pub gridfit: bool,
    pub symmetric_smoothing: bool,
    pub symmetric_gridfit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_gasp_Record {
    pub init: Option<unsafe extern "C" fn(*mut gasp_Record) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut gasp_Record, *const gasp_Record) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut gasp_Record, *mut gasp_Record) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut gasp_Record) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut gasp_Record, gasp_Record) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut gasp_Record, gasp_Record) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gasp_RecordList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut gasp_Record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_gasp_RecordList {
    pub init: Option<unsafe extern "C" fn(*mut gasp_RecordList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut gasp_RecordList, *const gasp_RecordList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut gasp_RecordList, *mut gasp_RecordList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut gasp_RecordList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut gasp_RecordList, gasp_RecordList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut gasp_RecordList, gasp_RecordList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut gasp_RecordList>,
    pub free: Option<unsafe extern "C" fn(*mut gasp_RecordList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut gasp_RecordList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut gasp_RecordList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut gasp_RecordList>,
    pub fill: Option<unsafe extern "C" fn(*mut gasp_RecordList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut gasp_RecordList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut gasp_RecordList, gasp_Record) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut gasp_RecordList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut gasp_RecordList) -> gasp_Record>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut gasp_RecordList, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut gasp_RecordList,
            Option<unsafe extern "C" fn(*const gasp_Record, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut gasp_RecordList,
            Option<
                unsafe extern "C" fn(*const gasp_Record, *const gasp_Record) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_gasp {
    pub version: uint16_t,
    pub records: gasp_RecordList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_gasp {
    pub init: Option<unsafe extern "C" fn(*mut table_gasp) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_gasp, *const table_gasp) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_gasp, *mut table_gasp) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_gasp) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_gasp, table_gasp) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_gasp, table_gasp) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_gasp>,
    pub free: Option<unsafe extern "C" fn(*mut table_gasp) -> ()>,
}
pub type font_file_pointer = *mut uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const GASP_DOGRAY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const GASP_GRIDFIT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const GASP_SYMMETRIC_GRIDFIT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const GASP_SYMMETRIC_SMOOTHING: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn gasp_Record_copy(mut dst: *mut gasp_Record, mut src: *const gasp_Record) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<gasp_Record>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn gasp_Record_dispose(mut _x: *mut gasp_Record) {}
#[inline]
unsafe extern "C" fn gasp_Record_replace(mut dst: *mut gasp_Record, src: gasp_Record) {
    gasp_Record_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<gasp_Record>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn gasp_Record_move(mut dst: *mut gasp_Record, mut src: *mut gasp_Record) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<gasp_Record>() as size_t,
    );
    gasp_Record_init(src);
}
#[inline]
unsafe extern "C" fn gasp_Record_init(mut x: *mut gasp_Record) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<gasp_Record>() as size_t,
    );
}
#[no_mangle]
pub static mut gasp_iRecord: __caryll_elementinterface_gasp_Record = {
    __caryll_elementinterface_gasp_Record {
        init: Some(gasp_Record_init as unsafe extern "C" fn(*mut gasp_Record) -> ()),
        copy: Some(
            gasp_Record_copy as unsafe extern "C" fn(*mut gasp_Record, *const gasp_Record) -> (),
        ),
        move_0: Some(
            gasp_Record_move as unsafe extern "C" fn(*mut gasp_Record, *mut gasp_Record) -> (),
        ),
        dispose: Some(gasp_Record_dispose as unsafe extern "C" fn(*mut gasp_Record) -> ()),
        replace: Some(
            gasp_Record_replace as unsafe extern "C" fn(*mut gasp_Record, gasp_Record) -> (),
        ),
        copyReplace: Some(
            gasp_Record_copyReplace as unsafe extern "C" fn(*mut gasp_Record, gasp_Record) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn gasp_Record_copyReplace(mut dst: *mut gasp_Record, src: gasp_Record) {
    gasp_Record_dispose(dst);
    gasp_Record_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn gasp_RecordList_shrinkToFit(mut arr: *mut gasp_RecordList) {
    gasp_RecordList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn gasp_RecordList_resizeTo(arr: *mut gasp_RecordList, target: size_t) {
    cvec_resize_to(gasp_RecordList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn gasp_RecordList_grow(arr: *mut gasp_RecordList) {
    cvec_grow(gasp_RecordList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn gasp_RecordList_growTo(arr: *mut gasp_RecordList, target: size_t) {
    cvec_grow_to(gasp_RecordList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn gasp_RecordList_pop(arr: *mut gasp_RecordList) -> gasp_Record {
    cvec_pop(gasp_RecordList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn gasp_RecordList_copyReplace(
    mut dst: *mut gasp_RecordList,
    src: gasp_RecordList,
) {
    gasp_RecordList_dispose(dst);
    gasp_RecordList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn gasp_RecordList_copy(
    mut dst: *mut gasp_RecordList,
    mut src: *const gasp_RecordList,
) {
    gasp_RecordList_init(dst);
    gasp_RecordList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gasp_iRecord.copy.is_some() {
        let mut j: size_t = 0 as size_t;
        while j < (*src).length {
            gasp_iRecord.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut gasp_Record,
                (*src).items.offset(j as isize) as *mut gasp_Record as *const gasp_Record,
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
unsafe extern "C" fn gasp_RecordList_dispose(mut arr: *mut gasp_RecordList) {
    if arr.is_null() {
        return;
    }
    if gasp_iRecord.dispose.is_some() {
        let mut j: size_t = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            gasp_iRecord.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut gasp_Record,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<gasp_Record>();
    (*arr).length = 0 as size_t;
    (*arr).capacity = 0 as size_t;
}
#[inline]
unsafe extern "C" fn gasp_RecordList_replace(mut dst: *mut gasp_RecordList, src: gasp_RecordList) {
    gasp_RecordList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<gasp_RecordList>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn gasp_RecordList_initCapN(mut arr: *mut gasp_RecordList, mut n: size_t) {
    gasp_RecordList_init(arr);
    gasp_RecordList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn gasp_RecordList_growToN(arr: *mut gasp_RecordList, target: size_t) {
    cvec_grow_to_n(gasp_RecordList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn gasp_RecordList_initN(mut arr: *mut gasp_RecordList, mut n: size_t) {
    gasp_RecordList_init(arr);
    gasp_RecordList_growToN(arr, n);
    gasp_RecordList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn gasp_RecordList_free(mut x: *mut gasp_RecordList) {
    if x.is_null() {
        return;
    }
    gasp_RecordList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn gasp_RecordList_createN(mut n: size_t) -> *mut gasp_RecordList {
    let mut t: *mut gasp_RecordList =
        malloc(::core::mem::size_of::<gasp_RecordList>() as size_t) as *mut gasp_RecordList;
    gasp_RecordList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn gasp_RecordList_create() -> *mut gasp_RecordList {
    let mut x: *mut gasp_RecordList =
        malloc(::core::mem::size_of::<gasp_RecordList>() as size_t) as *mut gasp_RecordList;
    gasp_RecordList_init(x);
    return x;
}
#[inline]
unsafe fn gasp_RecordList_as_cvec(arr: *mut gasp_RecordList) -> *mut CVecRaw<gasp_Record> {
    arr as *mut CVecRaw<gasp_Record>
}
#[inline]
unsafe extern "C" fn gasp_RecordList_init(arr: *mut gasp_RecordList) {
    cvec_init(gasp_RecordList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn gasp_RecordList_move(dst: *mut gasp_RecordList, src: *mut gasp_RecordList) {
    cvec_move(gasp_RecordList_as_cvec(dst), gasp_RecordList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn gasp_RecordList_filterEnv(
    mut arr: *mut gasp_RecordList,
    mut fn_0: Option<unsafe extern "C" fn(*const gasp_Record, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: size_t = 0 as size_t;
    let mut k: size_t = 0 as size_t;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut gasp_Record,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gasp_iRecord.dispose.is_some() {
                gasp_iRecord.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut gasp_Record,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[no_mangle]
pub static mut gasp_iRecordList: __caryll_vectorinterface_gasp_RecordList = {
    __caryll_vectorinterface_gasp_RecordList {
        init: Some(gasp_RecordList_init as unsafe extern "C" fn(*mut gasp_RecordList) -> ()),
        copy: Some(
            gasp_RecordList_copy
                as unsafe extern "C" fn(*mut gasp_RecordList, *const gasp_RecordList) -> (),
        ),
        move_0: Some(
            gasp_RecordList_move
                as unsafe extern "C" fn(*mut gasp_RecordList, *mut gasp_RecordList) -> (),
        ),
        dispose: Some(gasp_RecordList_dispose as unsafe extern "C" fn(*mut gasp_RecordList) -> ()),
        replace: Some(
            gasp_RecordList_replace
                as unsafe extern "C" fn(*mut gasp_RecordList, gasp_RecordList) -> (),
        ),
        copyReplace: Some(
            gasp_RecordList_copyReplace
                as unsafe extern "C" fn(*mut gasp_RecordList, gasp_RecordList) -> (),
        ),
        create: Some(gasp_RecordList_create),
        free: Some(gasp_RecordList_free as unsafe extern "C" fn(*mut gasp_RecordList) -> ()),
        initN: Some(
            gasp_RecordList_initN as unsafe extern "C" fn(*mut gasp_RecordList, size_t) -> (),
        ),
        initCapN: Some(
            gasp_RecordList_initCapN as unsafe extern "C" fn(*mut gasp_RecordList, size_t) -> (),
        ),
        createN: Some(
            gasp_RecordList_createN as unsafe extern "C" fn(size_t) -> *mut gasp_RecordList,
        ),
        fill: Some(
            gasp_RecordList_fill as unsafe extern "C" fn(*mut gasp_RecordList, size_t) -> (),
        ),
        clear: Some(gasp_RecordList_dispose as unsafe extern "C" fn(*mut gasp_RecordList) -> ()),
        push: Some(
            gasp_RecordList_push as unsafe extern "C" fn(*mut gasp_RecordList, gasp_Record) -> (),
        ),
        shrinkToFit: Some(
            gasp_RecordList_shrinkToFit as unsafe extern "C" fn(*mut gasp_RecordList) -> (),
        ),
        pop: Some(gasp_RecordList_pop as unsafe extern "C" fn(*mut gasp_RecordList) -> gasp_Record),
        disposeItem: Some(
            gasp_RecordList_disposeItem as unsafe extern "C" fn(*mut gasp_RecordList, size_t) -> (),
        ),
        filterEnv: Some(
            gasp_RecordList_filterEnv
                as unsafe extern "C" fn(
                    *mut gasp_RecordList,
                    Option<
                        unsafe extern "C" fn(*const gasp_Record, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            gasp_RecordList_sort
                as unsafe extern "C" fn(
                    *mut gasp_RecordList,
                    Option<
                        unsafe extern "C" fn(
                            *const gasp_Record,
                            *const gasp_Record,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn gasp_RecordList_disposeItem(mut arr: *mut gasp_RecordList, mut n: size_t) {
    if gasp_iRecord.dispose.is_some() {
        gasp_iRecord.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut gasp_Record
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn gasp_RecordList_sort(
    mut arr: *mut gasp_RecordList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const gasp_Record, *const gasp_Record) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<gasp_Record>() as size_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const gasp_Record, *const gasp_Record) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn gasp_RecordList_fill(mut arr: *mut gasp_RecordList, mut n: size_t) {
    while (*arr).length < n {
        let mut x: gasp_Record = gasp_Record {
            rangeMaxPPEM: 0,
            dogray: false,
            gridfit: false,
            symmetric_smoothing: false,
            symmetric_gridfit: false,
        };
        if gasp_iRecord.init.is_some() {
            gasp_iRecord.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<gasp_Record>() as size_t,
            );
        }
        gasp_RecordList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn gasp_RecordList_push(arr: *mut gasp_RecordList, elem: gasp_Record) {
    cvec_push(gasp_RecordList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn initGasp(mut gasp: *mut table_gasp) {
    (*gasp).version = 1 as uint16_t;
    gasp_iRecordList.init.expect("non-null function pointer")(&raw mut (*gasp).records);
}
#[inline]
unsafe extern "C" fn disposeGasp(mut gasp: *mut table_gasp) {
    gasp_iRecordList.dispose.expect("non-null function pointer")(&raw mut (*gasp).records);
}
#[inline]
unsafe extern "C" fn table_gasp_create() -> *mut table_gasp {
    let mut x: *mut table_gasp =
        malloc(::core::mem::size_of::<table_gasp>() as size_t) as *mut table_gasp;
    table_gasp_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_gasp_dispose(mut x: *mut table_gasp) {
    disposeGasp(x);
}
#[inline]
unsafe extern "C" fn table_gasp_copy(mut dst: *mut table_gasp, mut src: *const table_gasp) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_gasp>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_gasp_init(mut x: *mut table_gasp) {
    initGasp(x);
}
#[inline]
unsafe extern "C" fn table_gasp_move(mut dst: *mut table_gasp, mut src: *mut table_gasp) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_gasp>() as size_t,
    );
    table_gasp_init(src);
}
#[inline]
unsafe extern "C" fn table_gasp_replace(mut dst: *mut table_gasp, src: table_gasp) {
    table_gasp_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_gasp>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_gasp_copyReplace(mut dst: *mut table_gasp, src: table_gasp) {
    table_gasp_dispose(dst);
    table_gasp_copy(dst, &raw const src);
}
#[no_mangle]
pub static mut table_iGasp: __caryll_elementinterface_table_gasp = {
    __caryll_elementinterface_table_gasp {
        init: Some(table_gasp_init as unsafe extern "C" fn(*mut table_gasp) -> ()),
        copy: Some(
            table_gasp_copy as unsafe extern "C" fn(*mut table_gasp, *const table_gasp) -> (),
        ),
        move_0: Some(
            table_gasp_move as unsafe extern "C" fn(*mut table_gasp, *mut table_gasp) -> (),
        ),
        dispose: Some(table_gasp_dispose as unsafe extern "C" fn(*mut table_gasp) -> ()),
        replace: Some(
            table_gasp_replace as unsafe extern "C" fn(*mut table_gasp, table_gasp) -> (),
        ),
        copyReplace: Some(
            table_gasp_copyReplace as unsafe extern "C" fn(*mut table_gasp, table_gasp) -> (),
        ),
        create: Some(table_gasp_create),
        free: Some(table_gasp_free as unsafe extern "C" fn(*mut table_gasp) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_gasp_free(mut x: *mut table_gasp) {
    if x.is_null() {
        return;
    }
    table_gasp_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readGasp(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_gasp {
    let mut numRanges: tableid_t = 0;
    let mut gasp: *mut table_gasp = ::core::ptr::null_mut::<table_gasp>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1734439792i32 as uint32_t {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: uint32_t = table.length;
                    if !(length < 4 as uint32_t) {
                        gasp = (
                            table_iGasp.create.expect("non-null function pointer"))();
                        (*gasp).version = read_16u(data as *const uint8_t);
                        numRanges = read_16u(
                            data.offset(2 as ::core::ffi::c_int as isize) as *const uint8_t
                        ) as tableid_t;
                        if !(length
                            < (4 as ::core::ffi::c_int
                                + numRanges as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as uint32_t)
                        {
                            let mut j: uint32_t = 0 as uint32_t;
                            while j < numRanges as uint32_t {
                                let mut record: gasp_Record = gasp_Record {
                                    rangeMaxPPEM: 0,
                                    dogray: false,
                                    gridfit: false,
                                    symmetric_smoothing: false,
                                    symmetric_gridfit: false,
                                };
                                record.rangeMaxPPEM = read_16u(
                                    data.offset(4 as ::core::ffi::c_int as isize)
                                        .offset(j.wrapping_mul(4 as uint32_t) as isize)
                                        as *const uint8_t,
                                )
                                    as glyphsize_t;
                                let mut rangeGaspBehavior: uint16_t = read_16u(
                                    data.offset(4 as ::core::ffi::c_int as isize)
                                        .offset(j.wrapping_mul(4 as uint32_t) as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const uint8_t,
                                );
                                record.dogray =
                                    rangeGaspBehavior as ::core::ffi::c_int & GASP_DOGRAY != 0;
                                record.gridfit =
                                    rangeGaspBehavior as ::core::ffi::c_int & GASP_GRIDFIT != 0;
                                record.symmetric_smoothing = rangeGaspBehavior
                                    as ::core::ffi::c_int
                                    & GASP_SYMMETRIC_SMOOTHING
                                    != 0;
                                record.symmetric_gridfit = rangeGaspBehavior as ::core::ffi::c_int
                                    & GASP_SYMMETRIC_GRIDFIT
                                    != 0;
                                gasp_iRecordList.push.expect("non-null function pointer")(
                                    &raw mut (*gasp).records,
                                    record,
                                );
                                j = j.wrapping_add(1);
                            }
                            return gasp;
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
                            b"table 'gasp' corrupted.\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        ),
                    );
                    table_iGasp.free.expect("non-null function pointer")(gasp);
                    gasp = ::core::ptr::null_mut::<table_gasp>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_gasp>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpGasp(
    mut table: *const table_gasp,
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
        sdscatprintf(
            sdsempty(),
            b"gasp\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut t: *mut json_value = json_array_new((*table).records.length);
        let mut j: uint16_t = 0 as uint16_t;
        while (j as size_t) < (*table).records.length {
            let mut rec: *mut json_value = json_object_new(5 as size_t);
            json_object_push(
                rec,
                b"rangeMaxPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new(
                    (*(*table).records.items.offset(j as isize)).rangeMaxPPEM as int64_t,
                ),
            );
            json_object_push(
                rec,
                b"dogray\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    (*(*table).records.items.offset(j as isize)).dogray as ::core::ffi::c_int,
                ),
            );
            json_object_push(
                rec,
                b"gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    (*(*table).records.items.offset(j as isize)).gridfit as ::core::ffi::c_int,
                ),
            );
            json_object_push(
                rec,
                b"symmetric_smoothing\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    (*(*table).records.items.offset(j as isize)).symmetric_smoothing
                        as ::core::ffi::c_int,
                ),
            );
            json_object_push(
                rec,
                b"symmetric_gridfit\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    (*(*table).records.items.offset(j as isize)).symmetric_gridfit
                        as ::core::ffi::c_int,
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseGasp(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_gasp {
    let mut gasp: *mut table_gasp = ::core::ptr::null_mut::<table_gasp>();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"gasp\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"gasp\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            gasp = (
                table_iGasp.create.expect("non-null function pointer"))();
            let mut j: uint16_t = 0 as uint16_t;
            while (j as ::core::ffi::c_uint) < (*table).u.array.length {
                let mut r: *mut json_value =
                    *(*table).u.array.values.offset(j as isize) as *mut json_value;
                if !(r.is_null()
                    || (*r).type_0 as ::core::ffi::c_uint
                        != json_object as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    let mut record: gasp_Record = gasp_Record {
                        rangeMaxPPEM: 0,
                        dogray: false,
                        gridfit: false,
                        symmetric_smoothing: false,
                        symmetric_gridfit: false,
                    };
                    record.rangeMaxPPEM = json_obj_getint_fallback(
                        r,
                        b"rangeMaxPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                        0xffff as int32_t,
                    ) as glyphsize_t;
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
                    gasp_iRecordList.push.expect("non-null function pointer")(
                        &raw mut (*gasp).records,
                        record,
                    );
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
    }
    return gasp;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildGasp(
    mut gasp: *const table_gasp,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if gasp.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite16b(buf, 1 as uint16_t);
    bufwrite16b(buf, (*gasp).records.length as uint16_t);
    let mut j: uint16_t = 0 as uint16_t;
    while (j as size_t) < (*gasp).records.length {
        let mut r: *mut gasp_Record = (*gasp).records.items.offset(j as isize) as *mut gasp_Record;
        bufwrite16b(buf, (*r).rangeMaxPPEM as uint16_t);
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
            })) as uint16_t,
        );
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
#[inline]
unsafe extern "C" fn json_obj_getint_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: int32_t,
) -> int32_t {
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
    return fallback;
}
#[inline]
unsafe extern "C" fn json_obj_getbool(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> bool {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false;
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_boolean as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.boolean != 0;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return false;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
