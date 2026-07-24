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
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> size_t;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: size_t) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}

use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage, otl_Coverage};
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_dispose, otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle, HANDLE_STATE_EMPTY};
use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::support::cvec::{
    cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push,
    cvec_resize_to, CVecRaw,
};
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_serialize_opts {
    pub mode: ::core::ffi::c_int,
    pub opts: ::core::ffi::c_int,
    pub indent_size: ::core::ffi::c_int,
}
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
pub type tableid_t = uint16_t;
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
pub struct __otfcc_ICoverage {
    pub init: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_Coverage, *const otl_Coverage) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_Coverage) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_Coverage>,
    pub free: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_Coverage, uint32_t) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const uint8_t, uint32_t, uint32_t) -> *mut otl_Coverage>,
    pub dump: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_Coverage>,
    pub build: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut caryll_Buffer>,
    pub buildFormat:
        Option<unsafe extern "C" fn(*const otl_Coverage, uint16_t) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_Coverage, bool) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_Coverage, otfcc_GlyphHandle) -> ()>,
}
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
pub type otl_LookupType = ::core::ffi::c_uint;
pub const otl_type_gpos_extend: otl_LookupType = 41;
pub const otl_type_gpos_chaining: otl_LookupType = 40;
pub const otl_type_gpos_context: otl_LookupType = 39;
pub const otl_type_gpos_markToMark: otl_LookupType = 38;
pub const otl_type_gpos_markToLigature: otl_LookupType = 37;
pub const otl_type_gpos_markToBase: otl_LookupType = 36;
pub const otl_type_gpos_cursive: otl_LookupType = 35;
pub const otl_type_gpos_pair: otl_LookupType = 34;
pub const otl_type_gpos_single: otl_LookupType = 33;
pub const otl_type_gpos_unknown: otl_LookupType = 32;
pub const otl_type_gsub_reverse: otl_LookupType = 24;
pub const otl_type_gsub_extend: otl_LookupType = 23;
pub const otl_type_gsub_chaining: otl_LookupType = 22;
pub const otl_type_gsub_context: otl_LookupType = 21;
pub const otl_type_gsub_ligature: otl_LookupType = 20;
pub const otl_type_gsub_alternate: otl_LookupType = 19;
pub const otl_type_gsub_multiple: otl_LookupType = 18;
pub const otl_type_gsub_single: otl_LookupType = 17;
pub const otl_type_gsub_unknown: otl_LookupType = 16;
pub const otl_type_unknown: otl_LookupType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _otl_subtable {
    pub gsub_single: subtable_gsub_single,
    pub gsub_multi: subtable_gsub_multi,
    pub gsub_ligature: subtable_gsub_ligature,
    pub chaining: subtable_chaining,
    pub gsub_reverse: subtable_gsub_reverse,
    pub gpos_single: subtable_gpos_single,
    pub gpos_pair: subtable_gpos_pair,
    pub gpos_cursive: subtable_gpos_cursive,
    pub gpos_markToSingle: subtable_gpos_markToSingle,
    pub gpos_markToLigature: subtable_gpos_markToLigature,
    pub extend: subtable_extend,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_extend {
    pub type_0: otl_LookupType,
    pub subtable: *mut otl_Subtable,
}
pub type otl_Subtable = _otl_subtable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_markToLigature {
    pub classCount: glyphclass_t,
    pub markArray: otl_MarkArray,
    pub ligArray: otl_LigatureArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LigatureArray {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LigatureBaseRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LigatureBaseRecord {
    pub glyph: otfcc_GlyphHandle,
    pub componentCount: glyphid_t,
    pub anchors: *mut *mut otl_Anchor,
}
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
pub struct subtable_gpos_markToSingle {
    pub classCount: glyphclass_t,
    pub markArray: otl_MarkArray,
    pub baseArray: otl_BaseArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseArray {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_BaseRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseRecord {
    pub glyph: otfcc_GlyphHandle,
    pub anchors: *mut otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_cursive {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GposCursiveEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GposCursiveEntry {
    pub target: otfcc_GlyphHandle,
    pub enter: otl_Anchor,
    pub exit: otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_pair {
    pub first: *mut otl_ClassDef,
    pub second: *mut otl_ClassDef,
    pub firstValues: *mut *mut otl_PositionValue,
    pub secondValues: *mut *mut otl_PositionValue,
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
pub struct subtable_gpos_single {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GposSingleEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GposSingleEntry {
    pub target: otfcc_GlyphHandle,
    pub value: otl_PositionValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_reverse {
    pub matchCount: tableid_t,
    pub inputIndex: tableid_t,
    pub match_0: *mut *mut otl_Coverage,
    pub to: *mut otl_Coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_chaining {
    pub type_0: otl_chaining_type,
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub rulesCount: tableid_t,
    pub rules: *mut *mut otl_ChainingRule,
    pub bc: *mut otl_ClassDef,
    pub ic: *mut otl_ClassDef,
    pub fc: *mut otl_ClassDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ChainingRule {
    pub matchCount: tableid_t,
    pub inputBegins: tableid_t,
    pub inputEnds: tableid_t,
    pub match_0: *mut *mut otl_Coverage,
    pub applyCount: tableid_t,
    pub apply: *mut otl_ChainLookupApplication,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ChainLookupApplication {
    pub index: tableid_t,
    pub lookup: otfcc_LookupHandle,
}
pub type otl_chaining_type = ::core::ffi::c_uint;
pub const otl_chaining_classified: otl_chaining_type = 2;
pub const otl_chaining_poly: otl_chaining_type = 1;
pub const otl_chaining_canonical: otl_chaining_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_ligature {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GsubLigatureEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GsubLigatureEntry {
    pub from: *mut otl_Coverage,
    pub to: otfcc_GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_multi {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GsubMultiEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GsubMultiEntry {
    pub from: otfcc_GlyphHandle,
    pub to: *mut otl_Coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_single {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GsubSingleEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GsubSingleEntry {
    pub from: otfcc_GlyphHandle,
    pub to: otfcc_GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_subtable_gsub_ligature {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut subtable_gsub_ligature, *const subtable_gsub_ligature) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut subtable_gsub_ligature, *mut subtable_gsub_ligature) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, subtable_gsub_ligature) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, subtable_gsub_ligature) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gsub_ligature>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut subtable_gsub_ligature>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, otl_GsubLigatureEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> otl_GsubLigatureEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_ligature,
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubLigatureEntry,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_ligature,
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubLigatureEntry,
                    *const otl_GsubLigatureEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_GsubLigatureEntry {
    pub init: Option<unsafe extern "C" fn(*mut otl_GsubLigatureEntry) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut otl_GsubLigatureEntry, *const otl_GsubLigatureEntry) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_GsubLigatureEntry, *mut otl_GsubLigatureEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_GsubLigatureEntry) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut otl_GsubLigatureEntry, otl_GsubLigatureEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_GsubLigatureEntry, otl_GsubLigatureEntry) -> ()>,
}
pub type otl_BuildHeuristics = ::core::ffi::c_uint;
pub const OTL_BH_GSUB_VERT: otl_BuildHeuristics = 1;
pub const OTL_BH_NORMAL: otl_BuildHeuristics = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ligature_aggerator {
    pub gid: ::core::ffi::c_int,
    pub hh: UT_hash_handle,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const json_serialize_mode_packed: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
unsafe extern "C" fn deleteGsubLigatureEntry(mut entry: *mut otl_GsubLigatureEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).to);
    otl_Coverage_free((*entry).from);
    (*entry).from = ::core::ptr::null_mut::<otl_Coverage>();
}
static mut gss_typeinfo: __caryll_elementinterface_otl_GsubLigatureEntry = {
    __caryll_elementinterface_otl_GsubLigatureEntry {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(
            deleteGsubLigatureEntry as unsafe extern "C" fn(*mut otl_GsubLigatureEntry) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_move(dst: *mut subtable_gsub_ligature, src: *mut subtable_gsub_ligature) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_growTo(arr: *mut subtable_gsub_ligature, target: size_t) {
    cvec_grow_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_free(mut x: *mut subtable_gsub_ligature) {
    if x.is_null() {
        return;
    }
    subtable_gsub_ligature_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_pop(arr: *mut subtable_gsub_ligature) -> otl_GsubLigatureEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_copyReplace(
    mut dst: *mut subtable_gsub_ligature,
    src: subtable_gsub_ligature,
) {
    subtable_gsub_ligature_dispose(dst);
    subtable_gsub_ligature_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_copy(
    mut dst: *mut subtable_gsub_ligature,
    mut src: *const subtable_gsub_ligature,
) {
    subtable_gsub_ligature_init(dst);
    subtable_gsub_ligature_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gss_typeinfo.copy.is_some() {
        let mut j: size_t = 0 as size_t;
        while j < (*src).length {
            gss_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_GsubLigatureEntry,
                (*src).items.offset(j as isize) as *mut otl_GsubLigatureEntry
                    as *const otl_GsubLigatureEntry,
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
unsafe extern "C" fn subtable_gsub_ligature_dispose(mut arr: *mut subtable_gsub_ligature) {
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
                (*arr).items.offset(j as isize) as *mut otl_GsubLigatureEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_GsubLigatureEntry>();
    (*arr).length = 0 as size_t;
    (*arr).capacity = 0 as size_t;
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_replace(
    mut dst: *mut subtable_gsub_ligature,
    src: subtable_gsub_ligature,
) {
    subtable_gsub_ligature_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_ligature>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_initCapN(
    mut arr: *mut subtable_gsub_ligature,
    mut n: size_t,
) {
    subtable_gsub_ligature_init(arr);
    subtable_gsub_ligature_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_growToN(arr: *mut subtable_gsub_ligature, target: size_t) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_initN(
    mut arr: *mut subtable_gsub_ligature,
    mut n: size_t,
) {
    subtable_gsub_ligature_init(arr);
    subtable_gsub_ligature_growToN(arr, n);
    subtable_gsub_ligature_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_create() -> *mut subtable_gsub_ligature {
    let mut x: *mut subtable_gsub_ligature =
        malloc(::core::mem::size_of::<subtable_gsub_ligature>() as size_t)
            as *mut subtable_gsub_ligature;
    subtable_gsub_ligature_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_createN(mut n: size_t) -> *mut subtable_gsub_ligature {
    let mut t: *mut subtable_gsub_ligature =
        malloc(::core::mem::size_of::<subtable_gsub_ligature>() as size_t)
            as *mut subtable_gsub_ligature;
    subtable_gsub_ligature_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_filterEnv(
    mut arr: *mut subtable_gsub_ligature,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_GsubLigatureEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: size_t = 0 as size_t;
    let mut k: size_t = 0 as size_t;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_GsubLigatureEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gss_typeinfo.dispose.is_some() {
                gss_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_GsubLigatureEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn as_cvec(arr: *mut subtable_gsub_ligature) -> *mut CVecRaw<otl_GsubLigatureEntry> {
    arr as *mut CVecRaw<otl_GsubLigatureEntry>
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_init(arr: *mut subtable_gsub_ligature) {
    cvec_init(as_cvec(arr));
}
#[no_mangle]
pub static mut iSubtable_gsub_ligature: __caryll_vectorinterface_subtable_gsub_ligature = {
    __caryll_vectorinterface_subtable_gsub_ligature {
        init: Some(
            subtable_gsub_ligature_init as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> (),
        ),
        copy: Some(
            subtable_gsub_ligature_copy
                as unsafe extern "C" fn(
                    *mut subtable_gsub_ligature,
                    *const subtable_gsub_ligature,
                ) -> (),
        ),
        move_0: Some(
            subtable_gsub_ligature_move
                as unsafe extern "C" fn(
                    *mut subtable_gsub_ligature,
                    *mut subtable_gsub_ligature,
                ) -> (),
        ),
        dispose: Some(
            subtable_gsub_ligature_dispose
                as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> (),
        ),
        replace: Some(
            subtable_gsub_ligature_replace
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, subtable_gsub_ligature) -> (),
        ),
        copyReplace: Some(
            subtable_gsub_ligature_copyReplace
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, subtable_gsub_ligature) -> (),
        ),
        create: Some(subtable_gsub_ligature_create),
        free: Some(
            subtable_gsub_ligature_free as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> (),
        ),
        initN: Some(
            subtable_gsub_ligature_initN
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, size_t) -> (),
        ),
        initCapN: Some(
            subtable_gsub_ligature_initCapN
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, size_t) -> (),
        ),
        createN: Some(
            subtable_gsub_ligature_createN
                as unsafe extern "C" fn(size_t) -> *mut subtable_gsub_ligature,
        ),
        fill: Some(
            subtable_gsub_ligature_fill
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, size_t) -> (),
        ),
        clear: Some(
            subtable_gsub_ligature_dispose
                as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> (),
        ),
        push: Some(
            subtable_gsub_ligature_push
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, otl_GsubLigatureEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gsub_ligature_shrinkToFit
                as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> (),
        ),
        pop: Some(
            subtable_gsub_ligature_pop
                as unsafe extern "C" fn(*mut subtable_gsub_ligature) -> otl_GsubLigatureEntry,
        ),
        disposeItem: Some(
            subtable_gsub_ligature_disposeItem
                as unsafe extern "C" fn(*mut subtable_gsub_ligature, size_t) -> (),
        ),
        filterEnv: Some(
            subtable_gsub_ligature_filterEnv
                as unsafe extern "C" fn(
                    *mut subtable_gsub_ligature,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GsubLigatureEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gsub_ligature_sort
                as unsafe extern "C" fn(
                    *mut subtable_gsub_ligature,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GsubLigatureEntry,
                            *const otl_GsubLigatureEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_resizeTo(arr: *mut subtable_gsub_ligature, target: size_t) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_shrinkToFit(mut arr: *mut subtable_gsub_ligature) {
    subtable_gsub_ligature_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_disposeItem(
    mut arr: *mut subtable_gsub_ligature,
    mut n: size_t,
) {
    if gss_typeinfo.dispose.is_some() {
        gss_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_GsubLigatureEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_sort(
    mut arr: *mut subtable_gsub_ligature,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_GsubLigatureEntry,
            *const otl_GsubLigatureEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_GsubLigatureEntry>() as size_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubLigatureEntry,
                    *const otl_GsubLigatureEntry,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_fill(
    mut arr: *mut subtable_gsub_ligature,
    mut n: size_t,
) {
    while (*arr).length < n {
        let mut x: otl_GsubLigatureEntry = otl_GsubLigatureEntry {
            from: ::core::ptr::null_mut::<otl_Coverage>(),
            to: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
        };
        if gss_typeinfo.init.is_some() {
            gss_typeinfo.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_GsubLigatureEntry>() as size_t,
            );
        }
        subtable_gsub_ligature_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_push(arr: *mut subtable_gsub_ligature, elem: otl_GsubLigatureEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gsub_ligature_grow(arr: *mut subtable_gsub_ligature) {
    cvec_grow(as_cvec(arr));
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gsub_ligature(
    data: font_file_pointer,
    mut tableLength: uint32_t,
    mut offset: uint32_t,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut startCoverage: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    let mut setCount: glyphid_t = 0;
    let mut ligatureCount: uint32_t = 0;
    let mut current_block: u64;
    let mut subtable: *mut subtable_gsub_ligature =
        (
            iSubtable_gsub_ligature
                .create
                .expect("non-null function pointer"))();
    if !(tableLength < offset.wrapping_add(6 as uint32_t)) {
        startCoverage = readCoverage(
            data as *const uint8_t,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const uint8_t,
            ) as uint32_t),
        );
        if !startCoverage.is_null() {
            setCount = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const uint8_t,
            ) as glyphid_t;
            if !(setCount as ::core::ffi::c_int != (*startCoverage).numGlyphs as ::core::ffi::c_int)
            {
                if !(tableLength
                    < offset.wrapping_add(6 as uint32_t).wrapping_add(
                        (setCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as uint32_t,
                    ))
                {
                    ligatureCount = 0 as uint32_t;
                    let mut j: glyphid_t = 0 as glyphid_t;
                    loop {
                        if !((j as ::core::ffi::c_int) < setCount as ::core::ffi::c_int) {
                            current_block = 17860125682698302841;
                            break;
                        }
                        let mut setOffset: uint32_t = offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const uint8_t,
                        )
                            as uint32_t);
                        if tableLength < setOffset.wrapping_add(2 as uint32_t) {
                            current_block = 3443835632518673764;
                            break;
                        }
                        ligatureCount = ligatureCount.wrapping_add(read_16u(
                            data.offset(setOffset as isize) as *const uint8_t,
                        )
                            as uint32_t);
                        if tableLength
                            < setOffset.wrapping_add(2 as uint32_t).wrapping_add(
                                (read_16u(data.offset(setOffset as isize) as *const uint8_t)
                                    as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int)
                                    as uint32_t,
                            )
                        {
                            current_block = 3443835632518673764;
                            break;
                        }
                        j = j.wrapping_add(1);
                    }
                    match current_block {
                        3443835632518673764 => {}
                        _ => {
                            let mut j_0: glyphid_t = 0 as glyphid_t;
                            's_77: loop {
                                if !((j_0 as ::core::ffi::c_int) < setCount as ::core::ffi::c_int) {
                                    current_block = 11932355480408055363;
                                    break;
                                }
                                let mut setOffset_0: uint32_t = offset.wrapping_add(read_16u(
                                    data.offset(offset as isize)
                                        .offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const uint8_t,
                                )
                                    as uint32_t);
                                let mut lc: glyphid_t =
                                    read_16u(data.offset(setOffset_0 as isize) as *const uint8_t)
                                        as glyphid_t;
                                let mut k: glyphid_t = 0 as glyphid_t;
                                while (k as ::core::ffi::c_int) < lc as ::core::ffi::c_int {
                                    let mut ligOffset: uint32_t = setOffset_0.wrapping_add(
                                        read_16u(
                                            data.offset(setOffset_0 as isize)
                                                .offset(2 as ::core::ffi::c_int as isize)
                                                .offset(
                                                    (k as ::core::ffi::c_int
                                                        * 2 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                as *const uint8_t,
                                        ) as uint32_t,
                                    );
                                    if tableLength < ligOffset.wrapping_add(4 as uint32_t) {
                                        current_block = 3443835632518673764;
                                        break 's_77;
                                    }
                                    let mut ligComponents: glyphid_t = read_16u(
                                        data.offset(ligOffset as isize)
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            as *const uint8_t,
                                    )
                                        as glyphid_t;
                                    if tableLength
                                        < ligOffset.wrapping_add(2 as uint32_t).wrapping_add(
                                            (ligComponents as ::core::ffi::c_int
                                                * 2 as ::core::ffi::c_int)
                                                as uint32_t,
                                        )
                                    {
                                        current_block = 3443835632518673764;
                                        break 's_77;
                                    }
                                    let mut cov: *mut otl_Coverage =
                                        otl_Coverage_create();
                                    pushToCoverage(
                                        cov,
                                        handle_fromIndex(
                                            (*(*startCoverage).glyphs.offset(j_0 as isize)).index,
                                        )
                                            as otfcc_GlyphHandle,
                                    );
                                    let mut m: glyphid_t = 1 as glyphid_t;
                                    while (m as ::core::ffi::c_int)
                                        < ligComponents as ::core::ffi::c_int
                                    {
                                        pushToCoverage(
                                            cov,
                                            handle_fromIndex(
                                                read_16u(
                                                    data.offset(ligOffset as isize)
                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (m as ::core::ffi::c_int
                                                                * 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        as *const uint8_t,
                                                )
                                                    as glyphid_t,
                                            )
                                                as otfcc_GlyphHandle,
                                        );
                                        m = m.wrapping_add(1);
                                    }
                                    iSubtable_gsub_ligature
                                        .push
                                        .expect("non-null function pointer")(
                                        subtable,
                                        otl_GsubLigatureEntry {
                                            from: cov,
                                            to: handle_fromIndex(
                                                read_16u(data.offset(ligOffset as isize)
                                                    as *const uint8_t)
                                                    as glyphid_t,
                                            )
                                                as otfcc_GlyphHandle,
                                        },
                                    );
                                    k = k.wrapping_add(1);
                                }
                                j_0 = j_0.wrapping_add(1);
                            }
                            match current_block {
                                3443835632518673764 => {}
                                _ => {
                                    otl_Coverage_free(
                                        startCoverage,
                                    );
                                    return subtable as *mut otl_Subtable;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    iSubtable_gsub_ligature
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_dump_ligature(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gsub_ligature = &raw const (*_subtable).gsub_ligature;
    let mut st: *mut json_value = json_array_new((*subtable).length);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as size_t) < (*subtable).length {
        let mut entry: *mut json_value = json_object_new(2 as size_t);
        json_object_push(
            entry,
            b"from\0" as *const u8 as *const ::core::ffi::c_char,
            otl_iCoverage.dump.expect("non-null function pointer")(
                (*(*subtable).items.offset(j as isize)).from,
            ),
        );
        json_object_push(
            entry,
            b"to\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new_length(
                sdslen((*(*subtable).items.offset(j as isize)).to.name) as ::core::ffi::c_uint,
                (*(*subtable).items.offset(j as isize)).to.name as *const ::core::ffi::c_char,
            ),
        );
        json_array_push(st, preserialize(entry));
        j = j.wrapping_add(1);
    }
    let mut ret: *mut json_value = json_object_new(1 as size_t);
    json_object_push(
        ret,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        st,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_parse_ligature(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    if !json_obj_get_type(
        _subtable,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    )
    .is_null()
    {
        _subtable = json_obj_get_type(
            _subtable,
            b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
            json_array,
        );
        let mut st: *mut subtable_gsub_ligature =
            (
                iSubtable_gsub_ligature
                    .create
                    .expect("non-null function pointer"))();
        let mut n: glyphid_t = (*_subtable).u.array.length as glyphid_t;
        let mut k: glyphid_t = 0 as glyphid_t;
        while (k as ::core::ffi::c_int) < n as ::core::ffi::c_int {
            let mut entry: *mut json_value =
                *(*_subtable).u.array.values.offset(k as isize) as *mut json_value;
            let mut _from: *mut json_value = json_obj_get_type(
                entry,
                b"from\0" as *const u8 as *const ::core::ffi::c_char,
                json_array,
            );
            let mut _to: *mut json_value = json_obj_get_type(
                entry,
                b"to\0" as *const u8 as *const ::core::ffi::c_char,
                json_string,
            );
            if !(_from.is_null() || _to.is_null()) {
                iSubtable_gsub_ligature
                    .push
                    .expect("non-null function pointer")(
                    st,
                    otl_GsubLigatureEntry {
                        from: otl_iCoverage.parse.expect("non-null function pointer")(_from),
                        to: handle_fromName(sdsnewlen(
                            (*_to).u.string.ptr as *const ::core::ffi::c_void,
                            (*_to).u.string.length as size_t,
                        )) as otfcc_GlyphHandle,
                    },
                );
            }
            k = k.wrapping_add(1);
        }
        return st as *mut otl_Subtable;
    } else {
        let mut st_0: *mut subtable_gsub_ligature =
            (
                iSubtable_gsub_ligature
                    .create
                    .expect("non-null function pointer"))();
        let mut n_0: glyphid_t = (*_subtable).u.array.length as glyphid_t;
        let mut k_0: glyphid_t = 0 as glyphid_t;
        while (k_0 as ::core::ffi::c_int) < n_0 as ::core::ffi::c_int {
            let mut _from_0: *mut json_value =
                (*(*_subtable).u.object.values.offset(k_0 as isize)).value as *mut json_value;
            if !(_from_0.is_null()
                || (*_from_0).type_0 as ::core::ffi::c_uint
                    != json_array as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                iSubtable_gsub_ligature
                    .push
                    .expect("non-null function pointer")(
                    st_0,
                    otl_GsubLigatureEntry {
                        from: otl_iCoverage.parse.expect("non-null function pointer")(_from_0),
                        to: handle_fromName(sdsnewlen(
                            (*(*_subtable).u.object.values.offset(k_0 as isize)).name
                                as *const ::core::ffi::c_void,
                            (*(*_subtable).u.object.values.offset(k_0 as isize)).name_length
                                as size_t,
                        )) as otfcc_GlyphHandle,
                    },
                );
            }
            k_0 = k_0.wrapping_add(1);
        }
        return st_0 as *mut otl_Subtable;
    };
}
unsafe extern "C" fn by_gid(
    mut a: *mut ligature_aggerator,
    mut b: *mut ligature_aggerator,
) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gsub_ligature_subtable(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gsub_ligature = &raw const (*_subtable).gsub_ligature;
    let mut h: *mut ligature_aggerator = ::core::ptr::null_mut::<ligature_aggerator>();
    let mut s: *mut ligature_aggerator = ::core::ptr::null_mut::<ligature_aggerator>();
    let mut tmp: *mut ligature_aggerator = ::core::ptr::null_mut::<ligature_aggerator>();
    let mut nLigatures: glyphid_t = (*subtable).length as glyphid_t;
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < nLigatures as ::core::ffi::c_int {
        let mut sgid: ::core::ffi::c_int = (*(*(*(*subtable).items.offset(j as isize)).from)
            .glyphs
            .offset(0 as ::core::ffi::c_int as isize))
        .index as ::core::ffi::c_int;
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut sgid as *const ::core::ffi::c_uchar;
        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
        while _hj_k >= 12 as ::core::ffi::c_uint {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
        _hf_hashv = _hf_hashv
            .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
        let mut current_block_50: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 483194043190260627;
            }
            10 => {
                current_block_50 = 483194043190260627;
            }
            9 => {
                current_block_50 = 9392992341002218192;
            }
            8 => {
                current_block_50 = 14840068175916424037;
            }
            7 => {
                current_block_50 = 2003362535987825465;
            }
            6 => {
                current_block_50 = 7629293359809983242;
            }
            5 => {
                current_block_50 = 11376947495104746217;
            }
            4 => {
                current_block_50 = 16637993436199044631;
            }
            3 => {
                current_block_50 = 6546859112865444725;
            }
            2 => {
                current_block_50 = 10505030521387687196;
            }
            1 => {
                current_block_50 = 5259327757700886538;
            }
            _ => {
                current_block_50 = 1356832168064818221;
            }
        }
        match current_block_50 {
            483194043190260627 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 9392992341002218192;
            }
            _ => {}
        }
        match current_block_50 {
            9392992341002218192 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 14840068175916424037;
            }
            _ => {}
        }
        match current_block_50 {
            14840068175916424037 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 2003362535987825465;
            }
            _ => {}
        }
        match current_block_50 {
            2003362535987825465 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 7629293359809983242;
            }
            _ => {}
        }
        match current_block_50 {
            7629293359809983242 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 11376947495104746217;
            }
            _ => {}
        }
        match current_block_50 {
            11376947495104746217 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_50 = 16637993436199044631;
            }
            _ => {}
        }
        match current_block_50 {
            16637993436199044631 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 6546859112865444725;
            }
            _ => {}
        }
        match current_block_50 {
            6546859112865444725 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 10505030521387687196;
            }
            _ => {}
        }
        match current_block_50 {
            10505030521387687196 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 5259327757700886538;
            }
            _ => {}
        }
        match current_block_50 {
            5259327757700886538 => {
                _hj_i =
                    _hj_i
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
        s = ::core::ptr::null_mut::<ligature_aggerator>();
        if !h.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if !(*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize))
                    .hh_head
                    .is_null()
                {
                    s = ((*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                        as *mut ::core::ffi::c_char)
                        .offset(-(*(*h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut ligature_aggerator
                        as *mut ligature_aggerator;
                } else {
                    s = ::core::ptr::null_mut::<ligature_aggerator>();
                }
                while !s.is_null() {
                    if (*s).hh.hashv == _hf_hashv
                        && (*s).hh.keylen as usize
                            == ::core::mem::size_of::<::core::ffi::c_int>()
                    {
                        if memcmp(
                            (*s).hh.key,
                            &raw mut sgid as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*s).hh.hh_next.is_null() {
                        s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ligature_aggerator
                            as *mut ligature_aggerator;
                    } else {
                        s = ::core::ptr::null_mut::<ligature_aggerator>();
                    }
                }
            }
        }
        if s.is_null() {
            s = __caryll_allocate_clean(
                ::core::mem::size_of::<ligature_aggerator>() as size_t,
                132 as ::core::ffi::c_ulong,
            ) as *mut ligature_aggerator;
            (*s).gid = sgid;
            let mut _ha_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i_0: ::core::ffi::c_uint = 0;
            let mut _hj_j_0: ::core::ffi::c_uint = 0;
            let mut _hj_k_0: ::core::ffi::c_uint = 0;
            let mut _hj_key_0: *const ::core::ffi::c_uchar =
                &raw mut (*s).gid as *const ::core::ffi::c_uchar;
            _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_0 = _hj_j_0;
            _hj_k_0 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
                    (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_166: u64;
            match _hj_k_0 {
                11 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 3105948935974009916;
                }
                10 => {
                    current_block_166 = 3105948935974009916;
                }
                9 => {
                    current_block_166 = 16488506295619998735;
                }
                8 => {
                    current_block_166 = 2165477741955893522;
                }
                7 => {
                    current_block_166 = 16420434121503669123;
                }
                6 => {
                    current_block_166 = 4773154127383362184;
                }
                5 => {
                    current_block_166 = 11477443392506837243;
                }
                4 => {
                    current_block_166 = 4049670823543782160;
                }
                3 => {
                    current_block_166 = 8032402168972998897;
                }
                2 => {
                    current_block_166 = 13476765613733092207;
                }
                1 => {
                    current_block_166 = 7622613029762834038;
                }
                _ => {
                    current_block_166 = 7157669805658135323;
                }
            }
            match current_block_166 {
                3105948935974009916 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 16488506295619998735;
                }
                _ => {}
            }
            match current_block_166 {
                16488506295619998735 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 2165477741955893522;
                }
                _ => {}
            }
            match current_block_166 {
                2165477741955893522 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 16420434121503669123;
                }
                _ => {}
            }
            match current_block_166 {
                16420434121503669123 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 4773154127383362184;
                }
                _ => {}
            }
            match current_block_166 {
                4773154127383362184 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 11477443392506837243;
                }
                _ => {}
            }
            match current_block_166 {
                11477443392506837243 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_166 = 4049670823543782160;
                }
                _ => {}
            }
            match current_block_166 {
                4049670823543782160 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 8032402168972998897;
                }
                _ => {}
            }
            match current_block_166 {
                8032402168972998897 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 13476765613733092207;
                }
                _ => {}
            }
            match current_block_166 {
                13476765613733092207 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 7622613029762834038;
                }
                _ => {}
            }
            match current_block_166 {
                7622613029762834038 => {
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
            (*s).hh.key = &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            (*s).hh.keylen = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            if h.is_null() {
                (*s).hh.next = NULL;
                (*s).hh.prev = NULL;
                (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                    as *mut UT_hash_table as *mut UT_hash_table;
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
                    (*(*s).hh.tbl).buckets = malloc(
                        (32 as size_t)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                    ) as *mut UT_hash_bucket;
                    (*(*s).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                    if (*(*s).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as size_t)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                        );
                    }
                }
                h = s;
            } else {
                (*s).hh.tbl = (*h).hh.tbl;
                (*s).hh.next = NULL;
                (*s).hh.prev = ((*(*h).hh.tbl).tail as *mut ::core::ffi::c_char)
                    .offset(-(*(*h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void;
                (*(*(*h).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                (*(*h).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
            }
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UT_hash_bucket =
                (*(*h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
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
                let mut _he_thh: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_hh_nxt: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_new_buckets: *mut UT_hash_bucket =
                    ::core::ptr::null_mut::<UT_hash_bucket>();
                let mut _he_newbkt: *mut UT_hash_bucket = ::core::ptr::null_mut::<UT_hash_bucket>();
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
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
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
                        _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UT_hash_handle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*s).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                            (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                            if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                (*(*s).hh.tbl).nonideal_items =
                                    (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt).expand_mult = (*_he_newbkt)
                                    .count
                                    .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
                            }
                            (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UT_hash_handle;
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
    if !h.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*h).hh as *mut UT_hash_handle;
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
                            .offset((*(*h).hh.tbl).hho)
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
                                .offset((*(*h).hh.tbl).hho)
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
                                    .offset((*(*h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if by_gid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ligature_aggerator,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ligature_aggerator,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*h).hh.tbl).hho)
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
                                .offset((*(*h).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*h).hh.tbl).hho)
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
                                .offset(-(*(*h).hh.tbl).hho)
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
                (*(*h).hh.tbl).tail = _hs_tail;
                h = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut ligature_aggerator
                    as *mut ligature_aggerator;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut startcov: *mut otl_Coverage = otl_Coverage_create();
    s = h;
    while !s.is_null() {
        pushToCoverage(
            startcov,
            handle_fromIndex((*s).gid as glyphid_t)
                as otfcc_GlyphHandle,
        );
        s = (*s).hh.next as *mut ligature_aggerator;
    }
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            startcov,
        )),
        b16 as ::core::ffi::c_int,
        (*startcov).numGlyphs as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    s = h;
    while !s.is_null() {
        let mut nLigsHere: glyphid_t = 0 as glyphid_t;
        let mut j_0: glyphid_t = 0 as glyphid_t;
        while (j_0 as ::core::ffi::c_int) < nLigatures as ::core::ffi::c_int {
            if (*(*(*(*subtable).items.offset(j_0 as isize)).from)
                .glyphs
                .offset(0 as ::core::ffi::c_int as isize))
            .index as ::core::ffi::c_int
                == (*s).gid
            {
                nLigsHere = nLigsHere.wrapping_add(1);
            }
            j_0 = j_0.wrapping_add(1);
        }
        let mut ligset: *mut bk_Block = bk_new_Block(
            b16 as ::core::ffi::c_int,
            nLigsHere as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        let mut j_1: glyphid_t = 0 as glyphid_t;
        while (j_1 as ::core::ffi::c_int) < nLigatures as ::core::ffi::c_int {
            if (*(*(*(*subtable).items.offset(j_1 as isize)).from)
                .glyphs
                .offset(0 as ::core::ffi::c_int as isize))
            .index as ::core::ffi::c_int
                == (*s).gid
            {
                let mut ligdef: *mut bk_Block = bk_new_Block(
                    b16 as ::core::ffi::c_int,
                    (*(*subtable).items.offset(j_1 as isize)).to.index as ::core::ffi::c_int,
                    b16 as ::core::ffi::c_int,
                    (*(*(*subtable).items.offset(j_1 as isize)).from).numGlyphs
                        as ::core::ffi::c_int,
                    bkover as ::core::ffi::c_int,
                );
                let mut m: glyphid_t = 1 as glyphid_t;
                while (m as ::core::ffi::c_int)
                    < (*(*(*subtable).items.offset(j_1 as isize)).from).numGlyphs
                        as ::core::ffi::c_int
                {
                    bk_push(
                        ligdef,
                        b16 as ::core::ffi::c_int,
                        (*(*(*(*subtable).items.offset(j_1 as isize)).from)
                            .glyphs
                            .offset(m as isize))
                        .index as ::core::ffi::c_int,
                        bkover as ::core::ffi::c_int,
                    );
                    m = m.wrapping_add(1);
                }
                bk_push(
                    ligset,
                    p16 as ::core::ffi::c_int,
                    ligdef,
                    bkover as ::core::ffi::c_int,
                );
            }
            j_1 = j_1.wrapping_add(1);
        }
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            ligset,
            bkover as ::core::ffi::c_int,
        );
        s = (*s).hh.next as *mut ligature_aggerator;
    }
    otl_Coverage_free(startcov);
    s = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut ligature_aggerator
        as *mut ligature_aggerator;
    while !s.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<ligature_aggerator>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh2 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut ligature_aggerator as *mut ligature_aggerator;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh3 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<ligature_aggerator>();
        s = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut ligature_aggerator
            as *mut ligature_aggerator;
    }
    return bk_build_Block(root);
}
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
