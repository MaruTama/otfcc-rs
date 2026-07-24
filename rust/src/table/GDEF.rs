extern "C" {
    fn malloc(__size: usize) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: usize,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: usize,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    static otl_iClassDef: __otfcc_IClassDef;
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}
use crate::table::otl::classdef::{otl_ClassDef_free, readClassDef, otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage, otl_Coverage};
use crate::support::handle::{handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle_empty, otfcc_Handle, otfcc_GlyphHandle, HANDLE_STATE_EMPTY};
use crate::support::binio::{read_16u};
use crate::support::cvec::{
    cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push,
    cvec_resize_to, CVecRaw,
};
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
    pub integer: i64,
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
    pub cursor: usize,
    pub size: usize,
    pub free: usize,
    pub data: *mut u8,
}
pub type glyphid_t = u16;
pub type glyphclass_t = u16;
pub type shapeid_t = u16;
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
            u8,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, u8, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, u8) -> ()>,
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
    pub tag: u32,
    pub checkSum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Packet {
    pub sfnt_version: u32,
    pub numTables: u16,
    pub searchRange: u16,
    pub entrySelector: u16,
    pub rangeShift: u16,
    pub pieces: *mut otfcc_PacketPiece,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __otfcc_ICoverage {
    pub init: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_Coverage, *const otl_Coverage) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_Coverage) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_Coverage>,
    pub free: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_Coverage, u32) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_Coverage>,
    pub dump: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_Coverage>,
    pub build: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut caryll_Buffer>,
    pub buildFormat:
        Option<unsafe extern "C" fn(*const otl_Coverage, u16) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_Coverage, bool) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_Coverage, otfcc_GlyphHandle) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __otfcc_IClassDef {
    pub init: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_ClassDef, *const otl_ClassDef) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_ClassDef, *mut otl_ClassDef) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_ClassDef>,
    pub free: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut otl_ClassDef, otfcc_GlyphHandle, glyphclass_t) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_ClassDef>,
    pub expand:
        Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_ClassDef) -> *mut otl_ClassDef>,
    pub dump: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_ClassDef>,
    pub build: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_CaretValue {
    pub format: i8,
    pub coordiante: pos_t,
    pub pointIndex: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_CaretValue {
    pub init: Option<unsafe extern "C" fn(*mut otl_CaretValue) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_CaretValue, *const otl_CaretValue) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_CaretValue, *mut otl_CaretValue) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_CaretValue) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_CaretValue, otl_CaretValue) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_CaretValue, otl_CaretValue) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_CaretValueList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_CaretValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_CaretValueList {
    pub init: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_CaretValueList, *const otl_CaretValueList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_CaretValueList, *mut otl_CaretValueList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_CaretValueList, otl_CaretValueList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_CaretValueList, otl_CaretValueList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_CaretValueList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_CaretValueList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_CaretValueList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_CaretValueList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_CaretValueList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_CaretValueList, otl_CaretValue) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> otl_CaretValue>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_CaretValueList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_CaretValueList,
            Option<unsafe extern "C" fn(*const otl_CaretValue, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_CaretValueList,
            Option<
                unsafe extern "C" fn(
                    *const otl_CaretValue,
                    *const otl_CaretValue,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_CaretValueRecord {
    pub glyph: otfcc_GlyphHandle,
    pub carets: otl_CaretValueList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_CaretValueRecord {
    pub init: Option<unsafe extern "C" fn(*mut otl_CaretValueRecord) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_CaretValueRecord, *const otl_CaretValueRecord) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_CaretValueRecord, *mut otl_CaretValueRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_CaretValueRecord) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut otl_CaretValueRecord, otl_CaretValueRecord) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_CaretValueRecord, otl_CaretValueRecord) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LigCaretTable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_CaretValueRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_LigCaretTable {
    pub init: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, *const otl_LigCaretTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, *mut otl_LigCaretTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, otl_LigCaretTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, otl_LigCaretTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_LigCaretTable>,
    pub free: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_LigCaretTable>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, otl_CaretValueRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> otl_CaretValueRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_LigCaretTable,
            Option<
                unsafe extern "C" fn(*const otl_CaretValueRecord, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_LigCaretTable,
            Option<
                unsafe extern "C" fn(
                    *const otl_CaretValueRecord,
                    *const otl_CaretValueRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_GDEF {
    pub glyphClassDef: *mut otl_ClassDef,
    pub markAttachClassDef: *mut otl_ClassDef,
    pub ligCarets: otl_LigCaretTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_GDEF {
    pub init: Option<unsafe extern "C" fn(*mut table_GDEF) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_GDEF, *const table_GDEF) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_GDEF, *mut table_GDEF) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_GDEF) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_GDEF, table_GDEF) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_GDEF, table_GDEF) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_GDEF>,
    pub free: Option<unsafe extern "C" fn(*mut table_GDEF) -> ()>,
}
pub type font_file_pointer = *mut u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_serialize_opts {
    pub mode: ::core::ffi::c_int,
    pub opts: ::core::ffi::c_int,
    pub indent_size: ::core::ffi::c_int,
}
pub type bk_Block = __caryll_bkblock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_bkblock {
    pub _visitstate: bk_cell_visit_state,
    pub _index: u32,
    pub _height: u32,
    pub _depth: u32,
    pub length: u32,
    pub free: u32,
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
    pub z: u32,
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut otl_iCaretValue: __caryll_elementinterface_otl_CaretValue =
    __caryll_elementinterface_otl_CaretValue {
        init: None,
        copy: None,
        move_0: None,
        dispose: None,
        replace: None,
        copyReplace: None,
    };
#[inline]
unsafe extern "C" fn otl_CaretValueList_grow(arr: *mut otl_CaretValueList) {
    cvec_grow(otl_CaretValueList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_growTo(arr: *mut otl_CaretValueList, target: usize) {
    cvec_grow_to(otl_CaretValueList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_pop(arr: *mut otl_CaretValueList) -> otl_CaretValue {
    cvec_pop(otl_CaretValueList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_copyReplace(
    mut dst: *mut otl_CaretValueList,
    src: otl_CaretValueList,
) {
    otl_CaretValueList_dispose(dst);
    otl_CaretValueList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_copy(
    mut dst: *mut otl_CaretValueList,
    mut src: *const otl_CaretValueList,
) {
    otl_CaretValueList_init(dst);
    otl_CaretValueList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iCaretValue.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iCaretValue.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_CaretValue,
                (*src).items.offset(j as isize) as *mut otl_CaretValue as *const otl_CaretValue,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_dispose(mut arr: *mut otl_CaretValueList) {
    if arr.is_null() {
        return;
    }
    if otl_iCaretValue.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            otl_iCaretValue.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_CaretValue,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_CaretValue>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_replace(
    mut dst: *mut otl_CaretValueList,
    src: otl_CaretValueList,
) {
    otl_CaretValueList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_CaretValueList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_initCapN(mut arr: *mut otl_CaretValueList, mut n: usize) {
    otl_CaretValueList_init(arr);
    otl_CaretValueList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_growToN(arr: *mut otl_CaretValueList, target: usize) {
    cvec_grow_to_n(otl_CaretValueList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_initN(mut arr: *mut otl_CaretValueList, mut n: usize) {
    otl_CaretValueList_init(arr);
    otl_CaretValueList_growToN(arr, n);
    otl_CaretValueList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_free(mut x: *mut otl_CaretValueList) {
    if x.is_null() {
        return;
    }
    otl_CaretValueList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_createN(mut n: usize) -> *mut otl_CaretValueList {
    let mut t: *mut otl_CaretValueList =
        malloc(::core::mem::size_of::<otl_CaretValueList>() as usize) as *mut otl_CaretValueList;
    otl_CaretValueList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_create() -> *mut otl_CaretValueList {
    let mut x: *mut otl_CaretValueList =
        malloc(::core::mem::size_of::<otl_CaretValueList>() as usize) as *mut otl_CaretValueList;
    otl_CaretValueList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_shrinkToFit(mut arr: *mut otl_CaretValueList) {
    otl_CaretValueList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_resizeTo(arr: *mut otl_CaretValueList, target: usize) {
    cvec_resize_to(otl_CaretValueList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_move(dst: *mut otl_CaretValueList, src: *mut otl_CaretValueList) {
    cvec_move(otl_CaretValueList_as_cvec(dst), otl_CaretValueList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_filterEnv(
    mut arr: *mut otl_CaretValueList,
    mut fn_0: Option<unsafe extern "C" fn(*const otl_CaretValue, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_CaretValue,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otl_iCaretValue.dispose.is_some() {
                otl_iCaretValue.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_CaretValue,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn otl_CaretValueList_as_cvec(arr: *mut otl_CaretValueList) -> *mut CVecRaw<otl_CaretValue> {
    arr as *mut CVecRaw<otl_CaretValue>
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_init(arr: *mut otl_CaretValueList) {
    cvec_init(otl_CaretValueList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_push(arr: *mut otl_CaretValueList, elem: otl_CaretValue) {
    cvec_push(otl_CaretValueList_as_cvec(arr), elem);
}
#[no_mangle]
pub static mut otl_iCaretValueList: __caryll_vectorinterface_otl_CaretValueList = {
    __caryll_vectorinterface_otl_CaretValueList {
        init: Some(otl_CaretValueList_init as unsafe extern "C" fn(*mut otl_CaretValueList) -> ()),
        copy: Some(
            otl_CaretValueList_copy
                as unsafe extern "C" fn(*mut otl_CaretValueList, *const otl_CaretValueList) -> (),
        ),
        move_0: Some(
            otl_CaretValueList_move
                as unsafe extern "C" fn(*mut otl_CaretValueList, *mut otl_CaretValueList) -> (),
        ),
        dispose: Some(
            otl_CaretValueList_dispose as unsafe extern "C" fn(*mut otl_CaretValueList) -> (),
        ),
        replace: Some(
            otl_CaretValueList_replace
                as unsafe extern "C" fn(*mut otl_CaretValueList, otl_CaretValueList) -> (),
        ),
        copyReplace: Some(
            otl_CaretValueList_copyReplace
                as unsafe extern "C" fn(*mut otl_CaretValueList, otl_CaretValueList) -> (),
        ),
        create: Some(otl_CaretValueList_create),
        free: Some(otl_CaretValueList_free as unsafe extern "C" fn(*mut otl_CaretValueList) -> ()),
        initN: Some(
            otl_CaretValueList_initN as unsafe extern "C" fn(*mut otl_CaretValueList, usize) -> (),
        ),
        initCapN: Some(
            otl_CaretValueList_initCapN
                as unsafe extern "C" fn(*mut otl_CaretValueList, usize) -> (),
        ),
        createN: Some(
            otl_CaretValueList_createN as unsafe extern "C" fn(usize) -> *mut otl_CaretValueList,
        ),
        fill: Some(
            otl_CaretValueList_fill as unsafe extern "C" fn(*mut otl_CaretValueList, usize) -> (),
        ),
        clear: Some(
            otl_CaretValueList_dispose as unsafe extern "C" fn(*mut otl_CaretValueList) -> (),
        ),
        push: Some(
            otl_CaretValueList_push
                as unsafe extern "C" fn(*mut otl_CaretValueList, otl_CaretValue) -> (),
        ),
        shrinkToFit: Some(
            otl_CaretValueList_shrinkToFit as unsafe extern "C" fn(*mut otl_CaretValueList) -> (),
        ),
        pop: Some(
            otl_CaretValueList_pop
                as unsafe extern "C" fn(*mut otl_CaretValueList) -> otl_CaretValue,
        ),
        disposeItem: Some(
            otl_CaretValueList_disposeItem
                as unsafe extern "C" fn(*mut otl_CaretValueList, usize) -> (),
        ),
        filterEnv: Some(
            otl_CaretValueList_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_CaretValueList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_CaretValue,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_CaretValueList_sort
                as unsafe extern "C" fn(
                    *mut otl_CaretValueList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_CaretValue,
                            *const otl_CaretValue,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_CaretValueList_sort(
    mut arr: *mut otl_CaretValueList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_CaretValue, *const otl_CaretValue) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_CaretValue>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_CaretValue,
                    *const otl_CaretValue,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_disposeItem(
    mut arr: *mut otl_CaretValueList,
    mut n: usize,
) {
    if otl_iCaretValue.dispose.is_some() {
        otl_iCaretValue.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_CaretValue,
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_CaretValueList_fill(mut arr: *mut otl_CaretValueList, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_CaretValue = otl_CaretValue {
            format: 0,
            coordiante: 0.,
            pointIndex: 0,
        };
        if otl_iCaretValue.init.is_some() {
            otl_iCaretValue.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_CaretValue>() as usize,
            );
        }
        otl_CaretValueList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn initGdefLigCaretRec(mut v: *mut otl_CaretValueRecord) {
    (*v).glyph = otfcc_Handle_empty() as otfcc_GlyphHandle;
    otl_iCaretValueList.init.expect("non-null function pointer")(&raw mut (*v).carets);
}
unsafe extern "C" fn deleteGdefLigCaretRec(mut v: *mut otl_CaretValueRecord) {
    otfcc_Handle_dispose(&raw mut (*v).glyph);
    otl_iCaretValueList
        .dispose
        .expect("non-null function pointer")(&raw mut (*v).carets);
}
#[no_mangle]
pub static mut otl_iCaretValueRecord: __caryll_elementinterface_otl_CaretValueRecord = {
    __caryll_elementinterface_otl_CaretValueRecord {
        init: Some(initGdefLigCaretRec as unsafe extern "C" fn(*mut otl_CaretValueRecord) -> ()),
        copy: None,
        move_0: None,
        dispose: Some(
            deleteGdefLigCaretRec as unsafe extern "C" fn(*mut otl_CaretValueRecord) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe fn otl_LigCaretTable_as_cvec(arr: *mut otl_LigCaretTable) -> *mut CVecRaw<otl_CaretValueRecord> {
    arr as *mut CVecRaw<otl_CaretValueRecord>
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_init(arr: *mut otl_LigCaretTable) {
    cvec_init(otl_LigCaretTable_as_cvec(arr));
}
#[no_mangle]
pub static mut otl_iLigCaretTable: __caryll_vectorinterface_otl_LigCaretTable = {
    __caryll_vectorinterface_otl_LigCaretTable {
        init: Some(otl_LigCaretTable_init as unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()),
        copy: Some(
            otl_LigCaretTable_copy
                as unsafe extern "C" fn(*mut otl_LigCaretTable, *const otl_LigCaretTable) -> (),
        ),
        move_0: Some(
            otl_LigCaretTable_move
                as unsafe extern "C" fn(*mut otl_LigCaretTable, *mut otl_LigCaretTable) -> (),
        ),
        dispose: Some(
            otl_LigCaretTable_dispose as unsafe extern "C" fn(*mut otl_LigCaretTable) -> (),
        ),
        replace: Some(
            otl_LigCaretTable_replace
                as unsafe extern "C" fn(*mut otl_LigCaretTable, otl_LigCaretTable) -> (),
        ),
        copyReplace: Some(
            otl_LigCaretTable_copyReplace
                as unsafe extern "C" fn(*mut otl_LigCaretTable, otl_LigCaretTable) -> (),
        ),
        create: Some(otl_LigCaretTable_create),
        free: Some(otl_LigCaretTable_free as unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()),
        initN: Some(
            otl_LigCaretTable_initN as unsafe extern "C" fn(*mut otl_LigCaretTable, usize) -> (),
        ),
        initCapN: Some(
            otl_LigCaretTable_initCapN
                as unsafe extern "C" fn(*mut otl_LigCaretTable, usize) -> (),
        ),
        createN: Some(
            otl_LigCaretTable_createN as unsafe extern "C" fn(usize) -> *mut otl_LigCaretTable,
        ),
        fill: Some(
            otl_LigCaretTable_fill as unsafe extern "C" fn(*mut otl_LigCaretTable, usize) -> (),
        ),
        clear: Some(
            otl_LigCaretTable_dispose as unsafe extern "C" fn(*mut otl_LigCaretTable) -> (),
        ),
        push: Some(
            otl_LigCaretTable_push
                as unsafe extern "C" fn(*mut otl_LigCaretTable, otl_CaretValueRecord) -> (),
        ),
        shrinkToFit: Some(
            otl_LigCaretTable_shrinkToFit as unsafe extern "C" fn(*mut otl_LigCaretTable) -> (),
        ),
        pop: Some(
            otl_LigCaretTable_pop
                as unsafe extern "C" fn(*mut otl_LigCaretTable) -> otl_CaretValueRecord,
        ),
        disposeItem: Some(
            otl_LigCaretTable_disposeItem
                as unsafe extern "C" fn(*mut otl_LigCaretTable, usize) -> (),
        ),
        filterEnv: Some(
            otl_LigCaretTable_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_LigCaretTable,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_CaretValueRecord,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_LigCaretTable_sort
                as unsafe extern "C" fn(
                    *mut otl_LigCaretTable,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_CaretValueRecord,
                            *const otl_CaretValueRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LigCaretTable_shrinkToFit(mut arr: *mut otl_LigCaretTable) {
    otl_LigCaretTable_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_resizeTo(arr: *mut otl_LigCaretTable, target: usize) {
    cvec_resize_to(otl_LigCaretTable_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_move(dst: *mut otl_LigCaretTable, src: *mut otl_LigCaretTable) {
    cvec_move(otl_LigCaretTable_as_cvec(dst), otl_LigCaretTable_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_filterEnv(
    mut arr: *mut otl_LigCaretTable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_CaretValueRecord, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_CaretValueRecord,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otl_iCaretValueRecord.dispose.is_some() {
                otl_iCaretValueRecord
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_CaretValueRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_disposeItem(mut arr: *mut otl_LigCaretTable, mut n: usize) {
    if otl_iCaretValueRecord.dispose.is_some() {
        otl_iCaretValueRecord
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_CaretValueRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_sort(
    mut arr: *mut otl_LigCaretTable,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_CaretValueRecord,
            *const otl_CaretValueRecord,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_CaretValueRecord>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_CaretValueRecord,
                    *const otl_CaretValueRecord,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_fill(mut arr: *mut otl_LigCaretTable, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_CaretValueRecord = otl_CaretValueRecord {
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            carets: otl_CaretValueList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<otl_CaretValue>(),
            },
        };
        if otl_iCaretValueRecord.init.is_some() {
            otl_iCaretValueRecord
                .init
                .expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_CaretValueRecord>() as usize,
            );
        }
        otl_LigCaretTable_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_push(arr: *mut otl_LigCaretTable, elem: otl_CaretValueRecord) {
    cvec_push(otl_LigCaretTable_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_grow(arr: *mut otl_LigCaretTable) {
    cvec_grow(otl_LigCaretTable_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_growTo(arr: *mut otl_LigCaretTable, target: usize) {
    cvec_grow_to(otl_LigCaretTable_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_pop(arr: *mut otl_LigCaretTable) -> otl_CaretValueRecord {
    cvec_pop(otl_LigCaretTable_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_copyReplace(
    mut dst: *mut otl_LigCaretTable,
    src: otl_LigCaretTable,
) {
    otl_LigCaretTable_dispose(dst);
    otl_LigCaretTable_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_copy(
    mut dst: *mut otl_LigCaretTable,
    mut src: *const otl_LigCaretTable,
) {
    otl_LigCaretTable_init(dst);
    otl_LigCaretTable_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iCaretValueRecord.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iCaretValueRecord
                .copy
                .expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_CaretValueRecord,
                (*src).items.offset(j as isize) as *mut otl_CaretValueRecord
                    as *const otl_CaretValueRecord,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_dispose(mut arr: *mut otl_LigCaretTable) {
    if arr.is_null() {
        return;
    }
    if otl_iCaretValueRecord.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            otl_iCaretValueRecord
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_CaretValueRecord
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_CaretValueRecord>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_replace(
    mut dst: *mut otl_LigCaretTable,
    src: otl_LigCaretTable,
) {
    otl_LigCaretTable_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LigCaretTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_initCapN(mut arr: *mut otl_LigCaretTable, mut n: usize) {
    otl_LigCaretTable_init(arr);
    otl_LigCaretTable_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_growToN(arr: *mut otl_LigCaretTable, target: usize) {
    cvec_grow_to_n(otl_LigCaretTable_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_initN(mut arr: *mut otl_LigCaretTable, mut n: usize) {
    otl_LigCaretTable_init(arr);
    otl_LigCaretTable_growToN(arr, n);
    otl_LigCaretTable_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_free(mut x: *mut otl_LigCaretTable) {
    if x.is_null() {
        return;
    }
    otl_LigCaretTable_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_createN(mut n: usize) -> *mut otl_LigCaretTable {
    let mut t: *mut otl_LigCaretTable =
        malloc(::core::mem::size_of::<otl_LigCaretTable>() as usize) as *mut otl_LigCaretTable;
    otl_LigCaretTable_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_LigCaretTable_create() -> *mut otl_LigCaretTable {
    let mut x: *mut otl_LigCaretTable =
        malloc(::core::mem::size_of::<otl_LigCaretTable>() as usize) as *mut otl_LigCaretTable;
    otl_LigCaretTable_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn initGDEF(mut gdef: *mut table_GDEF) {
    (*gdef).glyphClassDef = ::core::ptr::null_mut::<otl_ClassDef>();
    (*gdef).markAttachClassDef = ::core::ptr::null_mut::<otl_ClassDef>();
    otl_iLigCaretTable.init.expect("non-null function pointer")(&raw mut (*gdef).ligCarets);
}
#[inline]
unsafe extern "C" fn disposeGDEF(mut gdef: *mut table_GDEF) {
    if gdef.is_null() {
        return;
    }
    if !(*gdef).glyphClassDef.is_null() {
        otl_ClassDef_free((*gdef).glyphClassDef);
    }
    if !(*gdef).markAttachClassDef.is_null() {
        otl_ClassDef_free((*gdef).markAttachClassDef);
    }
    otl_iLigCaretTable
        .dispose
        .expect("non-null function pointer")(&raw mut (*gdef).ligCarets);
}
#[inline]
unsafe extern "C" fn table_GDEF_init(mut x: *mut table_GDEF) {
    initGDEF(x);
}
#[inline]
unsafe extern "C" fn table_GDEF_dispose(mut x: *mut table_GDEF) {
    disposeGDEF(x);
}
#[no_mangle]
pub static mut table_iGDEF: __caryll_elementinterface_table_GDEF = {
    __caryll_elementinterface_table_GDEF {
        init: Some(table_GDEF_init as unsafe extern "C" fn(*mut table_GDEF) -> ()),
        copy: Some(
            table_GDEF_copy as unsafe extern "C" fn(*mut table_GDEF, *const table_GDEF) -> (),
        ),
        move_0: Some(
            table_GDEF_move as unsafe extern "C" fn(*mut table_GDEF, *mut table_GDEF) -> (),
        ),
        dispose: Some(table_GDEF_dispose as unsafe extern "C" fn(*mut table_GDEF) -> ()),
        replace: Some(
            table_GDEF_replace as unsafe extern "C" fn(*mut table_GDEF, table_GDEF) -> (),
        ),
        copyReplace: Some(
            table_GDEF_copyReplace as unsafe extern "C" fn(*mut table_GDEF, table_GDEF) -> (),
        ),
        create: Some(table_GDEF_create),
        free: Some(table_GDEF_free as unsafe extern "C" fn(*mut table_GDEF) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_GDEF_create() -> *mut table_GDEF {
    let mut x: *mut table_GDEF =
        malloc(::core::mem::size_of::<table_GDEF>() as usize) as *mut table_GDEF;
    table_GDEF_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_GDEF_move(mut dst: *mut table_GDEF, mut src: *mut table_GDEF) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_GDEF>() as usize,
    );
    table_GDEF_init(src);
}
#[inline]
unsafe extern "C" fn table_GDEF_free(mut x: *mut table_GDEF) {
    if x.is_null() {
        return;
    }
    table_GDEF_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_GDEF_copyReplace(mut dst: *mut table_GDEF, src: table_GDEF) {
    table_GDEF_dispose(dst);
    table_GDEF_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_GDEF_copy(mut dst: *mut table_GDEF, mut src: *const table_GDEF) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_GDEF>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_GDEF_replace(mut dst: *mut table_GDEF, src: table_GDEF) {
    table_GDEF_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_GDEF>() as usize,
    );
}
unsafe extern "C" fn readCaretValue(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
) -> otl_CaretValue {
    let mut v: otl_CaretValue = otl_CaretValue {
        format: 0,
        coordiante: 0.,
        pointIndex: 0,
    };
    v.format = 0 as i8;
    v.coordiante = 0 as ::core::ffi::c_int as pos_t;
    v.pointIndex = 0xffff as ::core::ffi::c_int as i16;
    if !(tableLength < offset.wrapping_add(4 as u32)) {
        v.format = read_16u(data.offset(offset as isize) as *const u8) as i8;
        if v.format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            v.pointIndex = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as i16;
        } else {
            v.coordiante = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as pos_t;
        }
    }
    return v;
}
unsafe extern "C" fn readLigCaretRecord(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
) -> otl_CaretValueRecord {
    let mut caretCount: shapeid_t = 0;
    let mut g: otl_CaretValueRecord = otl_CaretValueRecord {
        glyph: otfcc_Handle {
            state: HANDLE_STATE_EMPTY,
            index: 0,
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        carets: otl_CaretValueList {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<otl_CaretValue>(),
        },
    };
    otl_iCaretValueRecord
        .init
        .expect("non-null function pointer")(&raw mut g);
    if !(tableLength < offset.wrapping_add(2 as u32)) {
        caretCount = read_16u(data.offset(offset as isize) as *const u8) as shapeid_t;
        if !(tableLength
            < offset.wrapping_add(2 as u32).wrapping_add(
                (caretCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
            ))
        {
            let mut j: glyphid_t = 0 as glyphid_t;
            while (j as ::core::ffi::c_int) < caretCount as ::core::ffi::c_int {
                otl_iCaretValueList.push.expect("non-null function pointer")(
                    &raw mut g.carets,
                    readCaretValue(
                        data,
                        tableLength,
                        offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        ) as u32),
                    ),
                );
                j = j.wrapping_add(1);
            }
        }
    }
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readGDEF(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
) -> *mut table_GDEF {
    let mut classdefOffset: u16 = 0;
    let mut ligCaretOffset: u16 = 0;
    let mut markAttachDefOffset: u16 = 0;
    let mut current_block: u64;
    let mut gdef: *mut table_GDEF = ::core::ptr::null_mut::<table_GDEF>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1195656518i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut tableLength: u32 = table.length;
                    if !(tableLength < 12 as u32) {
                        gdef = (
                            table_iGDEF.create.expect("non-null function pointer"))();
                        classdefOffset = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if classdefOffset != 0 {
                            (*gdef).glyphClassDef =
                                readClassDef(
                                    data as *const u8,
                                    tableLength,
                                    classdefOffset as u32,
                                );
                        }
                        ligCaretOffset = read_16u(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if ligCaretOffset != 0 {
                            if tableLength
                                < (ligCaretOffset as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                    as u32
                            {
                                current_block = 10802812094495641425;
                            } else {
                                let mut cov: *mut otl_Coverage =
                                    readCoverage(
                                        data as *const u8,
                                        tableLength,
                                        (ligCaretOffset as ::core::ffi::c_int
                                            + read_16u(data.offset(
                                                ligCaretOffset as ::core::ffi::c_int as isize,
                                            )
                                                as *const u8)
                                                as ::core::ffi::c_int)
                                            as u32,
                                    );
                                if cov.is_null()
                                    || (*cov).numGlyphs as ::core::ffi::c_int
                                        != read_16u(
                                            data.offset(
                                                ligCaretOffset as ::core::ffi::c_int as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        )
                                            as ::core::ffi::c_int
                                {
                                    current_block = 10802812094495641425;
                                } else if tableLength
                                    < (ligCaretOffset as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int
                                        + (*cov).numGlyphs as ::core::ffi::c_int
                                            * 2 as ::core::ffi::c_int)
                                        as u32
                                {
                                    current_block = 10802812094495641425;
                                } else {
                                    let mut j: glyphid_t = 0 as glyphid_t;
                                    while (j as ::core::ffi::c_int)
                                        < (*cov).numGlyphs as ::core::ffi::c_int
                                    {
                                        let mut v: otl_CaretValueRecord = readLigCaretRecord(
                                            data,
                                            tableLength,
                                            (ligCaretOffset as ::core::ffi::c_int
                                                + read_16u(
                                                    data.offset(
                                                        ligCaretOffset as ::core::ffi::c_int
                                                            as isize,
                                                    )
                                                    .offset(4 as ::core::ffi::c_int as isize)
                                                    .offset(
                                                        (j as ::core::ffi::c_int
                                                            * 2 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as *const u8,
                                                )
                                                    as ::core::ffi::c_int)
                                                as u32,
                                        );
                                        v.glyph =
                                            otfcc_Handle_dup(
                                                *(*cov).glyphs.offset(j as isize) as otfcc_Handle,
                                            )
                                                as otfcc_GlyphHandle;
                                        otl_iLigCaretTable.push.expect("non-null function pointer")(
                                            &raw mut (*gdef).ligCarets,
                                            v,
                                        );
                                        j = j.wrapping_add(1);
                                    }
                                    otl_Coverage_free(cov);
                                    current_block = 11307063007268554308;
                                }
                            }
                        } else {
                            current_block = 11307063007268554308;
                        }
                        match current_block {
                            10802812094495641425 => {}
                            _ => {
                                markAttachDefOffset =
                                    read_16u(data.offset(10 as ::core::ffi::c_int as isize)
                                        as *const u8);
                                if markAttachDefOffset != 0 {
                                    (*gdef).markAttachClassDef =
                                        readClassDef(
                                            data as *const u8,
                                            tableLength,
                                            markAttachDefOffset as u32,
                                        );
                                }
                                return gdef;
                            }
                        }
                    }
                    table_iGDEF.free.expect("non-null function pointer")(gdef);
                    gdef = ::core::ptr::null_mut::<table_GDEF>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return gdef;
}
unsafe extern "C" fn dumpGDEFLigCarets(mut gdef: *const table_GDEF) -> *mut json_value {
    let mut _carets: *mut json_value = json_object_new((*gdef).ligCarets.length);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*gdef).ligCarets.length {
        let mut name: sds = (*(*gdef).ligCarets.items.offset(j as isize)).glyph.name;
        let mut _record: *mut json_value =
            json_array_new((*(*gdef).ligCarets.items.offset(j as isize)).carets.length);
        let mut k: glyphid_t = 0 as glyphid_t;
        while (k as usize) < (*(*gdef).ligCarets.items.offset(j as isize)).carets.length {
            let mut _cv: *mut json_value = json_object_new(1 as usize);
            if (*(*(*gdef).ligCarets.items.offset(j as isize))
                .carets
                .items
                .offset(k as isize))
            .format as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int
            {
                json_object_push(
                    _cv,
                    b"atPoint\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (*(*(*gdef).ligCarets.items.offset(j as isize))
                            .carets
                            .items
                            .offset(k as isize))
                        .pointIndex as i64,
                    ),
                );
            } else {
                json_object_push(
                    _cv,
                    b"at\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (*(*(*gdef).ligCarets.items.offset(j as isize))
                            .carets
                            .items
                            .offset(k as isize))
                        .coordiante as i64,
                    ),
                );
            }
            json_array_push(_record, _cv);
            k = k.wrapping_add(1);
        }
        json_object_push(
            _carets,
            name as *const ::core::ffi::c_char,
            preserialize(_record),
        );
        j = j.wrapping_add(1);
    }
    return _carets;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpGDEF(
    mut gdef: *const table_GDEF,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if gdef.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"GDEF\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _gdef: *mut json_value = json_object_new(4 as usize);
        if !(*gdef).glyphClassDef.is_null() {
            json_object_push(
                _gdef,
                b"glyphClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                otl_iClassDef.dump.expect("non-null function pointer")((*gdef).glyphClassDef),
            );
        }
        if !(*gdef).markAttachClassDef.is_null() {
            json_object_push(
                _gdef,
                b"markAttachClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                otl_iClassDef.dump.expect("non-null function pointer")((*gdef).markAttachClassDef),
            );
        }
        if (*gdef).ligCarets.length != 0 {
            json_object_push(
                _gdef,
                b"ligCarets\0" as *const u8 as *const ::core::ffi::c_char,
                dumpGDEFLigCarets(gdef),
            );
        }
        json_object_push(
            root,
            b"GDEF\0" as *const u8 as *const ::core::ffi::c_char,
            _gdef,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
unsafe extern "C" fn ligCaretFromJson(
    mut _carets: *const json_value,
    mut lc: *mut otl_LigCaretTable,
) {
    if _carets.is_null()
        || (*_carets).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*_carets).u.object.length {
        let mut a: *mut json_value =
            (*(*_carets).u.object.values.offset(j as isize)).value as *mut json_value;
        if !(a.is_null()
            || (*a).type_0 as ::core::ffi::c_uint
                != json_array as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            let mut v: otl_CaretValueRecord = otl_CaretValueRecord {
                glyph: otfcc_Handle {
                    state: HANDLE_STATE_EMPTY,
                    index: 0,
                    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                },
                carets: otl_CaretValueList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<otl_CaretValue>(),
                },
            };
            otl_iCaretValueRecord
                .init
                .expect("non-null function pointer")(&raw mut v);
            v.glyph = handle_fromName(sdsnewlen(
                (*(*_carets).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
                (*(*_carets).u.object.values.offset(j as isize)).name_length as usize,
            )) as otfcc_GlyphHandle;
            let mut caretCount: shapeid_t = (*a).u.array.length as shapeid_t;
            let mut k: glyphid_t = 0 as glyphid_t;
            while (k as ::core::ffi::c_int) < caretCount as ::core::ffi::c_int {
                let mut caret: otl_CaretValue = otl_CaretValue {
                    format: 0,
                    coordiante: 0.,
                    pointIndex: 0,
                };
                caret.format = 1 as i8;
                caret.coordiante = 0 as ::core::ffi::c_int as pos_t;
                caret.pointIndex = 0xffff as ::core::ffi::c_int as i16;
                let mut _caret: *mut json_value =
                    *(*a).u.array.values.offset(k as isize) as *mut json_value;
                if !_caret.is_null()
                    && (*_caret).type_0 as ::core::ffi::c_uint
                        == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if !json_obj_get_type(
                        _caret,
                        b"atPoint\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer,
                    )
                    .is_null()
                    {
                        caret.format = 2 as i8;
                        caret.pointIndex = json_obj_getint(
                            _caret,
                            b"atPoint\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as i16;
                    } else {
                        caret.coordiante = json_obj_getnum(
                            _caret,
                            b"at\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as pos_t;
                    }
                }
                otl_iCaretValueList.push.expect("non-null function pointer")(
                    &raw mut v.carets,
                    caret,
                );
                k = k.wrapping_add(1);
            }
            otl_iLigCaretTable.push.expect("non-null function pointer")(lc, v);
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseGDEF(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_GDEF {
    let mut gdef: *mut table_GDEF = ::core::ptr::null_mut::<table_GDEF>();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"GDEF\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"GDEF\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            gdef = (
                table_iGDEF.create.expect("non-null function pointer"))();
            (*gdef).glyphClassDef =
                otl_iClassDef.parse.expect("non-null function pointer")(json_obj_get(
                    table,
                    b"glyphClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            (*gdef).markAttachClassDef =
                otl_iClassDef.parse.expect("non-null function pointer")(json_obj_get(
                    table,
                    b"markAttachClassDef\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            ligCaretFromJson(
                json_obj_get(
                    table,
                    b"ligCarets\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &raw mut (*gdef).ligCarets,
            );
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return gdef;
}
unsafe extern "C" fn writeLigCaretRec(mut cr: *mut otl_CaretValueRecord) -> *mut bk_Block {
    let mut bcr: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        (*cr).carets.length,
        bkover as ::core::ffi::c_int,
    );
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*cr).carets.length {
        bk_push(
            bcr,
            p16 as ::core::ffi::c_int,
            bk_new_Block(
                b16 as ::core::ffi::c_int,
                (*(*cr).carets.items.offset(j as isize)).format as ::core::ffi::c_int,
                b16 as ::core::ffi::c_int,
                if (*(*cr).carets.items.offset(j as isize)).format as ::core::ffi::c_int
                    == 2 as ::core::ffi::c_int
                {
                    (*(*cr).carets.items.offset(j as isize)).pointIndex as ::core::ffi::c_int
                } else {
                    (*(*cr).carets.items.offset(j as isize)).coordiante as i16
                        as ::core::ffi::c_int
                },
                bkover as ::core::ffi::c_int,
            ),
            bkover as ::core::ffi::c_int,
        );
        j = j.wrapping_add(1);
    }
    return bcr;
}
unsafe extern "C" fn writeLigCarets(mut lc: *const otl_LigCaretTable) -> *mut bk_Block {
    let mut cov: *mut otl_Coverage = otl_Coverage_create();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*lc).length {
        pushToCoverage(
            cov,
            otfcc_Handle_dup(
                (*(*lc).items.offset(j as isize)).glyph as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut lct: *mut bk_Block = bk_new_Block(
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(cov)),
        b16 as ::core::ffi::c_int,
        (*lc).length,
        bkover as ::core::ffi::c_int,
    );
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as usize) < (*lc).length {
        bk_push(
            lct,
            p16 as ::core::ffi::c_int,
            writeLigCaretRec((*lc).items.offset(j_0 as isize) as *mut otl_CaretValueRecord),
            bkover as ::core::ffi::c_int,
        );
        j_0 = j_0.wrapping_add(1);
    }
    otl_Coverage_free(cov);
    return lct;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildGDEF(
    mut gdef: *const table_GDEF,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if gdef.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut bGlyphClassDef: *mut bk_Block = ::core::ptr::null_mut::<bk_Block>();
    let mut bAttachList: *mut bk_Block = ::core::ptr::null_mut::<bk_Block>();
    let mut bLigCaretList: *mut bk_Block = ::core::ptr::null_mut::<bk_Block>();
    let mut bMarkAttachClassDef: *mut bk_Block = ::core::ptr::null_mut::<bk_Block>();
    if !(*gdef).glyphClassDef.is_null() {
        bGlyphClassDef =
            bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
                (*gdef).glyphClassDef,
            ));
    }
    if (*gdef).ligCarets.length != 0 {
        bLigCaretList = writeLigCarets(&raw const (*gdef).ligCarets);
    }
    if !(*gdef).markAttachClassDef.is_null() {
        bMarkAttachClassDef =
            bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
                (*gdef).markAttachClassDef,
            ));
    }
    let mut root: *mut bk_Block = bk_new_Block(
        b32 as ::core::ffi::c_int,
        0x10000 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bGlyphClassDef,
        p16 as ::core::ffi::c_int,
        bAttachList,
        p16 as ::core::ffi::c_int,
        bLigCaretList,
        p16 as ::core::ffi::c_int,
        bMarkAttachClassDef,
        bkover as ::core::ffi::c_int,
    );
    return bk_build_Block(root);
}
pub const json_serialize_mode_packed: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
unsafe extern "C" fn json_obj_getint(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> i32 {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as i32;
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
                return (*cv).u.integer as i32;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl as i32;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn preserialize(mut x: *mut json_value) -> *mut json_value {
    let mut opts: json_serialize_opts = json_serialize_opts {
        mode: json_serialize_mode_packed,
        opts: 0,
        indent_size: 0,
    };
    let mut preserialize_len: usize = json_measure_ex(x, opts);
    let mut buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let mut xx: *mut json_value = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as usize) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = json_pre_serialized;
    return xx;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
