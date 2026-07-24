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
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> size_t;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: size_t) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
    fn otl_anchor_absent() -> otl_Anchor;
    fn otl_read_anchor(
        data: font_file_pointer,
        tableLength: uint32_t,
        offset: uint32_t,
    ) -> otl_Anchor;
    fn otl_dump_anchor(a: otl_Anchor) -> *mut json_value;
    fn otl_parse_anchor(v: *mut json_value) -> otl_Anchor;
    fn bkFromAnchor(a: otl_Anchor) -> *mut bk_Block;
}
use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage, otl_Coverage};
use crate::support::handle::{handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle, HANDLE_STATE_EMPTY};
use crate::support::binio::{read_16u};
use crate::support::cvec::{
    cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push,
    cvec_resize_to, CVecRaw,
};
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
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
pub struct __caryll_vectorinterface_subtable_gpos_cursive {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut subtable_gpos_cursive, *const subtable_gpos_cursive) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, *mut subtable_gpos_cursive) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, subtable_gpos_cursive) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, subtable_gpos_cursive) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_cursive>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut subtable_gpos_cursive>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, otl_GposCursiveEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> otl_GposCursiveEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_cursive,
            Option<
                unsafe extern "C" fn(*const otl_GposCursiveEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_cursive,
            Option<
                unsafe extern "C" fn(
                    *const otl_GposCursiveEntry,
                    *const otl_GposCursiveEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_GposCursiveEntry {
    pub init: Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry, *const otl_GposCursiveEntry) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry, *mut otl_GposCursiveEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry, otl_GposCursiveEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_GposCursiveEntry, otl_GposCursiveEntry) -> ()>,
}
pub type otl_BuildHeuristics = ::core::ffi::c_uint;
pub const OTL_BH_GSUB_VERT: otl_BuildHeuristics = 1;
pub const OTL_BH_NORMAL: otl_BuildHeuristics = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
unsafe extern "C" fn deleteGposCursiveEntry(mut entry: *mut otl_GposCursiveEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).target);
}
static mut gss_typeinfo: __caryll_elementinterface_otl_GposCursiveEntry = {
    __caryll_elementinterface_otl_GposCursiveEntry {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(
            deleteGposCursiveEntry as unsafe extern "C" fn(*mut otl_GposCursiveEntry) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_move(dst: *mut subtable_gpos_cursive, src: *mut subtable_gpos_cursive) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_resizeTo(arr: *mut subtable_gpos_cursive, target: size_t) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_filterEnv(
    mut arr: *mut subtable_gpos_cursive,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_GposCursiveEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: size_t = 0 as size_t;
    let mut k: size_t = 0 as size_t;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_GposCursiveEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gss_typeinfo.dispose.is_some() {
                gss_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_GposCursiveEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn as_cvec(arr: *mut subtable_gpos_cursive) -> *mut CVecRaw<otl_GposCursiveEntry> {
    arr as *mut CVecRaw<otl_GposCursiveEntry>
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_init(arr: *mut subtable_gpos_cursive) {
    cvec_init(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_disposeItem(
    mut arr: *mut subtable_gpos_cursive,
    mut n: size_t,
) {
    if gss_typeinfo.dispose.is_some() {
        gss_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_GposCursiveEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_sort(
    mut arr: *mut subtable_gpos_cursive,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_GposCursiveEntry,
            *const otl_GposCursiveEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_GposCursiveEntry>() as size_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_GposCursiveEntry,
                    *const otl_GposCursiveEntry,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_fill(
    mut arr: *mut subtable_gpos_cursive,
    mut n: size_t,
) {
    while (*arr).length < n {
        let mut x: otl_GposCursiveEntry = otl_GposCursiveEntry {
            target: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            enter: otl_Anchor {
                present: false,
                x: 0.,
                y: 0.,
            },
            exit: otl_Anchor {
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
                ::core::mem::size_of::<otl_GposCursiveEntry>() as size_t,
            );
        }
        subtable_gpos_cursive_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_push(arr: *mut subtable_gpos_cursive, elem: otl_GposCursiveEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_grow(arr: *mut subtable_gpos_cursive) {
    cvec_grow(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_growTo(arr: *mut subtable_gpos_cursive, target: size_t) {
    cvec_grow_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_pop(arr: *mut subtable_gpos_cursive) -> otl_GposCursiveEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_copyReplace(
    mut dst: *mut subtable_gpos_cursive,
    src: subtable_gpos_cursive,
) {
    subtable_gpos_cursive_dispose(dst);
    subtable_gpos_cursive_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_copy(
    mut dst: *mut subtable_gpos_cursive,
    mut src: *const subtable_gpos_cursive,
) {
    subtable_gpos_cursive_init(dst);
    subtable_gpos_cursive_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gss_typeinfo.copy.is_some() {
        let mut j: size_t = 0 as size_t;
        while j < (*src).length {
            gss_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_GposCursiveEntry,
                (*src).items.offset(j as isize) as *mut otl_GposCursiveEntry
                    as *const otl_GposCursiveEntry,
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
unsafe extern "C" fn subtable_gpos_cursive_dispose(mut arr: *mut subtable_gpos_cursive) {
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
                (*arr).items.offset(j as isize) as *mut otl_GposCursiveEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_GposCursiveEntry>();
    (*arr).length = 0 as size_t;
    (*arr).capacity = 0 as size_t;
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_replace(
    mut dst: *mut subtable_gpos_cursive,
    src: subtable_gpos_cursive,
) {
    subtable_gpos_cursive_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_cursive>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_initCapN(
    mut arr: *mut subtable_gpos_cursive,
    mut n: size_t,
) {
    subtable_gpos_cursive_init(arr);
    subtable_gpos_cursive_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_growToN(arr: *mut subtable_gpos_cursive, target: size_t) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_initN(
    mut arr: *mut subtable_gpos_cursive,
    mut n: size_t,
) {
    subtable_gpos_cursive_init(arr);
    subtable_gpos_cursive_growToN(arr, n);
    subtable_gpos_cursive_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_free(mut x: *mut subtable_gpos_cursive) {
    if x.is_null() {
        return;
    }
    subtable_gpos_cursive_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_createN(mut n: size_t) -> *mut subtable_gpos_cursive {
    let mut t: *mut subtable_gpos_cursive =
        malloc(::core::mem::size_of::<subtable_gpos_cursive>() as size_t)
            as *mut subtable_gpos_cursive;
    subtable_gpos_cursive_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_create() -> *mut subtable_gpos_cursive {
    let mut x: *mut subtable_gpos_cursive =
        malloc(::core::mem::size_of::<subtable_gpos_cursive>() as size_t)
            as *mut subtable_gpos_cursive;
    subtable_gpos_cursive_init(x);
    return x;
}
#[no_mangle]
pub static mut iSubtable_gpos_cursive: __caryll_vectorinterface_subtable_gpos_cursive = {
    __caryll_vectorinterface_subtable_gpos_cursive {
        init: Some(
            subtable_gpos_cursive_init as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> (),
        ),
        copy: Some(
            subtable_gpos_cursive_copy
                as unsafe extern "C" fn(
                    *mut subtable_gpos_cursive,
                    *const subtable_gpos_cursive,
                ) -> (),
        ),
        move_0: Some(
            subtable_gpos_cursive_move
                as unsafe extern "C" fn(
                    *mut subtable_gpos_cursive,
                    *mut subtable_gpos_cursive,
                ) -> (),
        ),
        dispose: Some(
            subtable_gpos_cursive_dispose as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> (),
        ),
        replace: Some(
            subtable_gpos_cursive_replace
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, subtable_gpos_cursive) -> (),
        ),
        copyReplace: Some(
            subtable_gpos_cursive_copyReplace
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, subtable_gpos_cursive) -> (),
        ),
        create: Some(subtable_gpos_cursive_create),
        free: Some(
            subtable_gpos_cursive_free as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> (),
        ),
        initN: Some(
            subtable_gpos_cursive_initN
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, size_t) -> (),
        ),
        initCapN: Some(
            subtable_gpos_cursive_initCapN
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, size_t) -> (),
        ),
        createN: Some(
            subtable_gpos_cursive_createN
                as unsafe extern "C" fn(size_t) -> *mut subtable_gpos_cursive,
        ),
        fill: Some(
            subtable_gpos_cursive_fill
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, size_t) -> (),
        ),
        clear: Some(
            subtable_gpos_cursive_dispose as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> (),
        ),
        push: Some(
            subtable_gpos_cursive_push
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, otl_GposCursiveEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gpos_cursive_shrinkToFit
                as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> (),
        ),
        pop: Some(
            subtable_gpos_cursive_pop
                as unsafe extern "C" fn(*mut subtable_gpos_cursive) -> otl_GposCursiveEntry,
        ),
        disposeItem: Some(
            subtable_gpos_cursive_disposeItem
                as unsafe extern "C" fn(*mut subtable_gpos_cursive, size_t) -> (),
        ),
        filterEnv: Some(
            subtable_gpos_cursive_filterEnv
                as unsafe extern "C" fn(
                    *mut subtable_gpos_cursive,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GposCursiveEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gpos_cursive_sort
                as unsafe extern "C" fn(
                    *mut subtable_gpos_cursive,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GposCursiveEntry,
                            *const otl_GposCursiveEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_cursive_shrinkToFit(mut arr: *mut subtable_gpos_cursive) {
    subtable_gpos_cursive_resizeTo(arr, (*arr).length);
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gpos_cursive(
    data: font_file_pointer,
    mut tableLength: uint32_t,
    mut offset: uint32_t,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut valueCount: glyphid_t = 0;
    let mut subtable: *mut subtable_gpos_cursive =
        (
            iSubtable_gpos_cursive
                .create
                .expect("non-null function pointer"))();
    let mut targets: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    if !(tableLength < offset.wrapping_add(6 as uint32_t)) {
        targets = readCoverage(
            data as *const uint8_t,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const uint8_t,
            ) as uint32_t),
        );
        if !(targets.is_null()
            || (*targets).numGlyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            valueCount = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const uint8_t,
            ) as glyphid_t;
            if !(tableLength
                < offset.wrapping_add(6 as uint32_t).wrapping_add(
                    (4 as ::core::ffi::c_int * valueCount as ::core::ffi::c_int) as uint32_t,
                ))
            {
                if !(valueCount as ::core::ffi::c_int != (*targets).numGlyphs as ::core::ffi::c_int)
                {
                    let mut j: glyphid_t = 0 as glyphid_t;
                    while (j as ::core::ffi::c_int) < valueCount as ::core::ffi::c_int {
                        let mut enterOffset: uint16_t = read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (4 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                                ) as *const uint8_t,
                        );
                        let mut exitOffset: uint16_t = read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (4 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                                )
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const uint8_t,
                        );
                        let mut enter: otl_Anchor = otl_anchor_absent();
                        let mut exit: otl_Anchor = otl_anchor_absent();
                        if enterOffset != 0 {
                            enter = otl_read_anchor(
                                data,
                                tableLength,
                                offset.wrapping_add(enterOffset as uint32_t),
                            );
                        }
                        if exitOffset != 0 {
                            exit = otl_read_anchor(
                                data,
                                tableLength,
                                offset.wrapping_add(exitOffset as uint32_t),
                            );
                        }
                        iSubtable_gpos_cursive
                            .push
                            .expect("non-null function pointer")(
                            subtable,
                            otl_GposCursiveEntry {
                                target: otfcc_Handle_dup(
                                    *(*targets).glyphs.offset(j as isize) as otfcc_Handle,
                                ) as otfcc_GlyphHandle,
                                enter: enter,
                                exit: exit,
                            },
                        );
                        j = j.wrapping_add(1);
                    }
                    if !targets.is_null() {
                        otl_Coverage_free(targets);
                    }
                    return subtable as *mut otl_Subtable;
                }
            }
        }
    }
    if !targets.is_null() {
        otl_Coverage_free(targets);
    }
    iSubtable_gpos_cursive
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_dump_cursive(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gpos_cursive = &raw const (*_subtable).gpos_cursive;
    let mut st: *mut json_value = json_object_new((*subtable).length);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as size_t) < (*subtable).length {
        let mut rec: *mut json_value = json_object_new(2 as size_t);
        json_object_push(
            rec,
            b"enter\0" as *const u8 as *const ::core::ffi::c_char,
            otl_dump_anchor((*(*subtable).items.offset(j as isize)).enter),
        );
        json_object_push(
            rec,
            b"exit\0" as *const u8 as *const ::core::ffi::c_char,
            otl_dump_anchor((*(*subtable).items.offset(j as isize)).exit),
        );
        json_object_push(
            st,
            (*(*subtable).items.offset(j as isize)).target.name as *const ::core::ffi::c_char,
            preserialize(rec),
        );
        j = j.wrapping_add(1);
    }
    return st;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_parse_cursive(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut subtable: *mut subtable_gpos_cursive =
        (
            iSubtable_gpos_cursive
                .create
                .expect("non-null function pointer"))();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*_subtable).u.object.length {
        if !(*(*_subtable).u.object.values.offset(j as isize))
            .value
            .is_null()
            && (*(*(*_subtable).u.object.values.offset(j as isize)).value).type_0
                as ::core::ffi::c_uint
                == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut gname: sds = sdsnewlen(
                (*(*_subtable).u.object.values.offset(j as isize)).name
                    as *const ::core::ffi::c_void,
                (*(*_subtable).u.object.values.offset(j as isize)).name_length as size_t,
            );
            iSubtable_gpos_cursive
                .push
                .expect("non-null function pointer")(
                subtable,
                otl_GposCursiveEntry {
                    target: handle_fromName(gname)
                        as otfcc_GlyphHandle,
                    enter: otl_parse_anchor(json_obj_get(
                        (*(*_subtable).u.object.values.offset(j as isize)).value,
                        b"enter\0" as *const u8 as *const ::core::ffi::c_char,
                    )),
                    exit: otl_parse_anchor(json_obj_get(
                        (*(*_subtable).u.object.values.offset(j as isize)).value,
                        b"exit\0" as *const u8 as *const ::core::ffi::c_char,
                    )),
                },
            );
        }
        j = j.wrapping_add(1);
    }
    return subtable as *mut otl_Subtable;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gpos_cursive(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gpos_cursive = &raw const (*_subtable).gpos_cursive;
    let mut cov: *mut otl_Coverage = otl_Coverage_create();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as size_t) < (*subtable).length {
        pushToCoverage(
            cov,
            otfcc_Handle_dup(
                (*(*subtable).items.offset(j as isize)).target as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(cov)),
        b16 as ::core::ffi::c_int,
        (*subtable).length,
        bkover as ::core::ffi::c_int,
    );
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as size_t) < (*subtable).length {
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            bkFromAnchor((*(*subtable).items.offset(j_0 as isize)).enter),
            p16 as ::core::ffi::c_int,
            bkFromAnchor((*(*subtable).items.offset(j_0 as isize)).exit),
            bkover as ::core::ffi::c_int,
        );
        j_0 = j_0.wrapping_add(1);
    }
    otl_Coverage_free(cov);
    return bk_build_Block(root);
}
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
