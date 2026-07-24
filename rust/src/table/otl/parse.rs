use libc::{exit, free, malloc, memcmp, memset, strcmp, strlen, strncmp};
extern "C" {
    fn json_value_free(_: *mut json_value);
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn json_ident(a: *const json_value, b: *const json_value) -> bool;
    static otl_iSubtableList: __caryll_vectorinterface_otl_SubtableList;
    static otl_iLookupPtr: __caryll_elementinterface_otl_LookupPtr;
    static otl_iLookupList: __caryll_vectorinterface_otl_LookupList;
    static otl_iLookupRefList: __caryll_vectorinterface_otl_LookupRefList;
    static otl_iFeaturePtr: __caryll_elementinterface_otl_FeaturePtr;
    static otl_iFeatureList: __caryll_vectorinterface_otl_FeatureList;
    static otl_iFeatureRefList: __caryll_vectorinterface_otl_FeatureRefList;
    static otl_iLanguageSystem: __caryll_elementinterface_otl_LanguageSystemPtr;
    static otl_iLangSystemList: __caryll_vectorinterface_otl_LangSystemList;
    static table_iOTL: __caryll_elementinterface_table_OTL;
    fn otl_gsub_parse_single(
        _subtable: *const json_value,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_gsub_parse_multi(
        _subtable: *const json_value,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_gsub_parse_ligature(
        _subtable: *const json_value,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_gsub_parse_reverse(
        _subtable: *const json_value,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_gpos_parse_single(
        _subtable: *const json_value,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_gpos_parse_cursive(
        _subtable: *const json_value,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_gpos_parse_markToSingle(
        _subtable: *const json_value,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_gpos_parse_markToLigature(
        _subtable: *const json_value,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_parse_chaining(
        _subtable: *const json_value,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    static SCRIPT_LANGUAGE_SEPARATOR: ::core::ffi::c_char;
    static mut lookupFlagsLabels: [*const ::core::ffi::c_char; 0];
    static mut tableNames: [*const ::core::ffi::c_char; 0];
    fn otfcc_delete_lookup(lookup: *mut otl_Lookup);
    fn otl_gpos_parse_pair(
        _subtable: *const json_value,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
}

use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{otfcc_GlyphHandle, otfcc_LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
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
    pub hho: isize,
    pub ideal_chain_maxlen: ::core::ffi::c_uint,
    pub nonideal_items: ::core::ffi::c_uint,
    pub ineff_expands: ::core::ffi::c_uint,
    pub noexpand: ::core::ffi::c_uint,
    pub signature: u32,
}
pub type glyphid_t = u16;
pub type glyphclass_t = u16;
pub type tableid_t = u16;
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
pub struct _otl_lookup {
    pub name: sds,
    pub type_0: otl_LookupType,
    pub _offset: u32,
    pub flags: u16,
    pub subtables: otl_SubtableList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_SubtableList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_SubtablePtr,
}
pub type otl_SubtablePtr = *mut otl_Subtable;
pub type otl_Lookup = _otl_lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_SubtableList {
    pub init: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_SubtableList, *const otl_SubtableList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_SubtableList, *mut otl_SubtableList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_SubtableList, otl_SubtableList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_SubtableList, otl_SubtableList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_SubtableList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_SubtableList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_SubtableList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_SubtableList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_SubtableList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_SubtableList, otl_SubtablePtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> otl_SubtablePtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_SubtableList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_SubtableList,
            Option<unsafe extern "C" fn(*const otl_SubtablePtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_SubtableList,
            Option<
                unsafe extern "C" fn(
                    *const otl_SubtablePtr,
                    *const otl_SubtablePtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
    pub disposeDependent:
        Option<unsafe extern "C" fn(*mut otl_SubtableList, *const otl_Lookup) -> ()>,
}
pub type otl_LookupPtr = *mut otl_Lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_LookupPtr {
    pub init: Option<unsafe extern "C" fn(*mut otl_LookupPtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LookupPtr, *const otl_LookupPtr) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LookupPtr, *mut otl_LookupPtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LookupPtr) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LookupPtr, otl_LookupPtr) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LookupPtr, otl_LookupPtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LookupList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_LookupPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_LookupList {
    pub init: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LookupList, *const otl_LookupList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LookupList, *mut otl_LookupList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LookupList, otl_LookupList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LookupList, otl_LookupList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_LookupList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_LookupList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LookupList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_LookupList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LookupList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LookupList, otl_LookupPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LookupList) -> otl_LookupPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LookupList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_LookupList,
            Option<unsafe extern "C" fn(*const otl_LookupPtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_LookupList,
            Option<
                unsafe extern "C" fn(
                    *const otl_LookupPtr,
                    *const otl_LookupPtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
pub type otl_LookupRef = *const otl_Lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LookupRefList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_LookupRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_LookupRefList {
    pub init: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LookupRefList, *const otl_LookupRefList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LookupRefList, *mut otl_LookupRefList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LookupRefList, otl_LookupRefList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LookupRefList, otl_LookupRefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_LookupRefList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_LookupRefList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LookupRefList, otl_LookupRef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> otl_LookupRef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_LookupRefList,
            Option<unsafe extern "C" fn(*const otl_LookupRef, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_LookupRefList,
            Option<
                unsafe extern "C" fn(
                    *const otl_LookupRef,
                    *const otl_LookupRef,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_Feature {
    pub name: sds,
    pub lookups: otl_LookupRefList,
}
pub type otl_FeaturePtr = *mut otl_Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_FeaturePtr {
    pub init: Option<unsafe extern "C" fn(*mut otl_FeaturePtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_FeaturePtr, *const otl_FeaturePtr) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_FeaturePtr, *mut otl_FeaturePtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_FeaturePtr) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_FeaturePtr, otl_FeaturePtr) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_FeaturePtr, otl_FeaturePtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_FeatureList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_FeaturePtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_FeatureList {
    pub init: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_FeatureList, *const otl_FeatureList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_FeatureList, *mut otl_FeatureList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_FeatureList, otl_FeatureList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_FeatureList, otl_FeatureList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_FeatureList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_FeatureList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_FeatureList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_FeatureList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_FeatureList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_FeatureList, otl_FeaturePtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> otl_FeaturePtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_FeatureList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_FeatureList,
            Option<unsafe extern "C" fn(*const otl_FeaturePtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_FeatureList,
            Option<
                unsafe extern "C" fn(
                    *const otl_FeaturePtr,
                    *const otl_FeaturePtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
pub type otl_FeatureRef = *const otl_Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_FeatureRefList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_FeatureRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_FeatureRefList {
    pub init: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_FeatureRefList, *const otl_FeatureRefList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_FeatureRefList, *mut otl_FeatureRefList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, otl_FeatureRefList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_FeatureRefList, otl_FeatureRefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_FeatureRefList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_FeatureRefList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, otl_FeatureRef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> otl_FeatureRef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_FeatureRefList,
            Option<unsafe extern "C" fn(*const otl_FeatureRef, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_FeatureRefList,
            Option<
                unsafe extern "C" fn(
                    *const otl_FeatureRef,
                    *const otl_FeatureRef,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LanguageSystem {
    pub name: sds,
    pub requiredFeature: otl_FeatureRef,
    pub features: otl_FeatureRefList,
}
pub type otl_LanguageSystemPtr = *mut otl_LanguageSystem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_LanguageSystemPtr {
    pub init: Option<unsafe extern "C" fn(*mut otl_LanguageSystemPtr) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut otl_LanguageSystemPtr, *const otl_LanguageSystemPtr) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_LanguageSystemPtr, *mut otl_LanguageSystemPtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LanguageSystemPtr) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut otl_LanguageSystemPtr, otl_LanguageSystemPtr) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_LanguageSystemPtr, otl_LanguageSystemPtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LangSystemList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_LanguageSystemPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_LangSystemList {
    pub init: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_LangSystemList, *const otl_LangSystemList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_LangSystemList, *mut otl_LangSystemList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LangSystemList, otl_LangSystemList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_LangSystemList, otl_LangSystemList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_LangSystemList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_LangSystemList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LangSystemList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_LangSystemList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LangSystemList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LangSystemList, otl_LanguageSystemPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> otl_LanguageSystemPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LangSystemList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_LangSystemList,
            Option<
                unsafe extern "C" fn(
                    *const otl_LanguageSystemPtr,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_LangSystemList,
            Option<
                unsafe extern "C" fn(
                    *const otl_LanguageSystemPtr,
                    *const otl_LanguageSystemPtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_OTL {
    pub lookups: otl_LookupList,
    pub features: otl_FeatureList,
    pub languages: otl_LangSystemList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_OTL {
    pub init: Option<unsafe extern "C" fn(*mut table_OTL) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_OTL, *const table_OTL) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_OTL, *mut table_OTL) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_OTL) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_OTL, table_OTL) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_OTL, table_OTL) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_OTL>,
    pub free: Option<unsafe extern "C" fn(*mut table_OTL) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct language_hash {
    pub name: *mut ::core::ffi::c_char,
    pub language: *mut otl_LanguageSystem,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct feature_hash {
    pub name: *mut ::core::ffi::c_char,
    pub alias: bool,
    pub feature: *mut otl_Feature,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lookup_hash {
    pub name: *mut ::core::ffi::c_char,
    pub lookup: *mut otl_Lookup,
    pub hh: UT_hash_handle,
    pub orderType: lookup_order_type,
    pub orderVal: u16,
}
pub type lookup_order_type = ::core::ffi::c_uint;
pub const LOOKUP_ORDER_FILE: lookup_order_type = 1;
pub const LOOKUP_ORDER_FORCE: lookup_order_type = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
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
#[inline]
unsafe extern "C" fn otfcc_parse_flags(
    mut v: *const json_value,
    mut labels: *mut *const ::core::ffi::c_char,
) -> u32 {
    if v.is_null() {
        return 0 as u32;
    }
    if (*v).type_0 as ::core::ffi::c_uint
        == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*v).u.integer as u32;
    } else if (*v).type_0 as ::core::ffi::c_uint
        == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*v).u.dbl as u32;
    } else if (*v).type_0 as ::core::ffi::c_uint
        == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut flags: u32 = 0 as u32;
        let mut j: u16 = 0 as u16;
        while !(*labels.offset(j as isize)).is_null() {
            if json_obj_getbool(v, *labels.offset(j as isize)) {
                flags |= ((1 as ::core::ffi::c_int) << j as ::core::ffi::c_int) as u32;
            }
            j = j.wrapping_add(1);
        }
        return flags;
    } else {
        return 0 as u32;
    };
}
unsafe extern "C" fn _parse_lookup(
    mut lookup: *mut json_value,
    mut lookupName: *mut ::core::ffi::c_char,
    mut options: *const otfcc_Options,
    mut lh: *mut *mut lookup_hash,
) -> bool {
    let mut parsed: bool = false;
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gsub_single as ::core::ffi::c_int as isize),
            otl_type_gsub_single,
            Some(
                otl_gsub_parse_single
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gsub_multiple as ::core::ffi::c_int as isize),
            otl_type_gsub_multiple,
            Some(
                otl_gsub_parse_multi
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gsub_alternate as ::core::ffi::c_int as isize),
            otl_type_gsub_alternate,
            Some(
                otl_gsub_parse_multi
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gsub_ligature as ::core::ffi::c_int as isize),
            otl_type_gsub_ligature,
            Some(
                otl_gsub_parse_ligature
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gsub_chaining as ::core::ffi::c_int as isize),
            otl_type_gsub_chaining,
            Some(
                otl_parse_chaining
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gsub_reverse as ::core::ffi::c_int as isize),
            otl_type_gsub_reverse,
            Some(
                otl_gsub_parse_reverse
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gpos_single as ::core::ffi::c_int as isize),
            otl_type_gpos_single,
            Some(
                otl_gpos_parse_single
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gpos_pair as ::core::ffi::c_int as isize),
            otl_type_gpos_pair,
            Some(
                otl_gpos_parse_pair
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gpos_cursive as ::core::ffi::c_int as isize),
            otl_type_gpos_cursive,
            Some(
                otl_gpos_parse_cursive
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gpos_chaining as ::core::ffi::c_int as isize),
            otl_type_gpos_chaining,
            Some(
                otl_parse_chaining
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gpos_markToBase as ::core::ffi::c_int as isize),
            otl_type_gpos_markToBase,
            Some(
                otl_gpos_parse_markToSingle
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gpos_markToMark as ::core::ffi::c_int as isize),
            otl_type_gpos_markToMark,
            Some(
                otl_gpos_parse_markToSingle
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declareLookupParser(
            *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
                .offset(otl_type_gpos_markToLigature as ::core::ffi::c_int as isize),
            otl_type_gpos_markToLigature,
            Some(
                otl_gpos_parse_markToLigature
                    as unsafe extern "C" fn(
                        *const json_value,
                        *const otfcc_Options,
                    ) -> *mut otl_Subtable,
            ),
            lookup,
            lookupName,
            options,
            lh,
        );
    }
    return parsed;
}
unsafe extern "C" fn _declareLookupParser(
    mut lt: *const ::core::ffi::c_char,
    mut llt: otl_LookupType,
    mut parser: Option<
        unsafe extern "C" fn(*const json_value, *const otfcc_Options) -> *mut otl_Subtable,
    >,
    mut _lookup: *mut json_value,
    mut lookupName: *mut ::core::ffi::c_char,
    mut options: *const otfcc_Options,
    mut lh: *mut *mut lookup_hash,
) -> bool {
    let mut type_0: *mut json_value = json_obj_get_type(
        _lookup,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
        json_string,
    );
    if type_0.is_null() || strcmp((*type_0).u.string.ptr, lt) != 0 {
        if type_0.is_null() {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"Lookup %s does not have a valid 'type' field.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    lookupName,
                ),
            );
        }
        return false;
    }
    let mut item: *mut lookup_hash = ::core::ptr::null_mut::<lookup_hash>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = lookupName as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = strlen(lookupName) as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(strlen(lookupName) as ::core::ffi::c_uint);
    let mut current_block_56: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_56 = 12987634765625610914;
        }
        10 => {
            current_block_56 = 12987634765625610914;
        }
        9 => {
            current_block_56 = 5841171998411354662;
        }
        8 => {
            current_block_56 = 4033618348666952069;
        }
        7 => {
            current_block_56 = 8587070255151231925;
        }
        6 => {
            current_block_56 = 203113926914940710;
        }
        5 => {
            current_block_56 = 9677870101952677128;
        }
        4 => {
            current_block_56 = 15218297217691294388;
        }
        3 => {
            current_block_56 = 15505743585181210549;
        }
        2 => {
            current_block_56 = 2096039366741040039;
        }
        1 => {
            current_block_56 = 18227449997886760986;
        }
        _ => {
            current_block_56 = 2122094917359643297;
        }
    }
    match current_block_56 {
        12987634765625610914 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_56 = 5841171998411354662;
        }
        _ => {}
    }
    match current_block_56 {
        5841171998411354662 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_56 = 4033618348666952069;
        }
        _ => {}
    }
    match current_block_56 {
        4033618348666952069 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_56 = 8587070255151231925;
        }
        _ => {}
    }
    match current_block_56 {
        8587070255151231925 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_56 = 203113926914940710;
        }
        _ => {}
    }
    match current_block_56 {
        203113926914940710 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_56 = 9677870101952677128;
        }
        _ => {}
    }
    match current_block_56 {
        9677870101952677128 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_56 = 15218297217691294388;
        }
        _ => {}
    }
    match current_block_56 {
        15218297217691294388 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_56 = 15505743585181210549;
        }
        _ => {}
    }
    match current_block_56 {
        15505743585181210549 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_56 = 2096039366741040039;
        }
        _ => {}
    }
    match current_block_56 {
        2096039366741040039 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_56 = 18227449997886760986;
        }
        _ => {}
    }
    match current_block_56 {
        18227449997886760986 => {
            _hj_i = _hj_i.wrapping_add(
                *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
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
    item = ::core::ptr::null_mut::<lookup_hash>();
    if !(*lh).is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(**lh).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(**lh).hh.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                item = ((*(*(**lh).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(**lh).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut lookup_hash
                    as *mut lookup_hash;
            } else {
                item = ::core::ptr::null_mut::<lookup_hash>();
            }
            while !item.is_null() {
                if (*item).hh.hashv == _hf_hashv
                    && (*item).hh.keylen == strlen(lookupName) as ::core::ffi::c_uint
                {
                    if memcmp(
                        (*item).hh.key,
                        lookupName as *const ::core::ffi::c_void,
                        strlen(lookupName) as ::core::ffi::c_uint as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*item).hh.hh_next.is_null() {
                    item = ((*item).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(**lh).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut lookup_hash
                        as *mut lookup_hash;
                } else {
                    item = ::core::ptr::null_mut::<lookup_hash>();
                }
            }
        }
    }
    if !item.is_null() {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            sdscatprintf(
                sdsempty(),
                b"Lookup %s already exists.\0" as *const u8 as *const ::core::ffi::c_char,
                lookupName,
            ),
        );
        return false;
    }
    let mut _subtables: *mut json_value = json_obj_get_type(
        _lookup,
        b"subtables\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if _subtables.is_null() {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            sdscatprintf(
                sdsempty(),
                b"Lookup %s does not have a valid subtable list.\0" as *const u8
                    as *const ::core::ffi::c_char,
                lookupName,
            ),
        );
        return false;
    }
    let mut lookup: *mut otl_Lookup = ::core::ptr::null_mut::<otl_Lookup>();
    otl_iLookupPtr.init.expect("non-null function pointer")(&raw mut lookup);
    (*lookup).type_0 = llt;
    (*lookup).flags = otfcc_parse_flags(
        json_obj_get(
            _lookup,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut lookupFlagsLabels as *mut *const ::core::ffi::c_char,
    ) as u16;
    let mut markAttachmentType: u16 = json_obj_getint(
        _lookup,
        b"markAttachmentType\0" as *const u8 as *const ::core::ffi::c_char,
    ) as u16;
    if markAttachmentType != 0 {
        (*lookup).flags = ((*lookup).flags as ::core::ffi::c_int
            | (markAttachmentType as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as u16;
    }
    let mut subtableCount: tableid_t = (*_subtables).u.array.length as tableid_t;
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            lookupName,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < subtableCount as ::core::ffi::c_int {
            let mut _subtable: *mut json_value =
                *(*_subtables).u.array.values.offset(j as isize) as *mut json_value;
            if !_subtable.is_null()
                && (*_subtable).type_0 as ::core::ffi::c_uint
                    == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut _st: *mut otl_Subtable =
                    parser.expect("non-null function pointer")(_subtable, options);
                otl_iSubtableList.push.expect("non-null function pointer")(
                    &raw mut (*lookup).subtables,
                    _st as otl_SubtablePtr,
                );
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    if (*lookup).subtables.length == 0 {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            sdscatprintf(
                sdsempty(),
                b"Lookup %s does not have any subtables.\0" as *const u8
                    as *const ::core::ffi::c_char,
                lookupName,
            ),
        );
        otfcc_delete_lookup(lookup);
        return false;
    }
    item = __caryll_allocate_clean(
        ::core::mem::size_of::<lookup_hash>() as usize,
        105 as ::core::ffi::c_ulong,
    ) as *mut lookup_hash;
    (*item).name = sdsnew(lookupName) as *mut ::core::ffi::c_char;
    (*lookup).name = sdsdup((*item).name as sds);
    (*item).lookup = lookup;
    (*item).orderType = LOOKUP_ORDER_FILE;
    (*item).orderVal = (if !(*lh).is_null() {
        (*(**lh).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) as u16;
    let mut _ha_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i_0: ::core::ffi::c_uint = 0;
    let mut _hj_j_0: ::core::ffi::c_uint = 0;
    let mut _hj_k_0: ::core::ffi::c_uint = 0;
    let mut _hj_key_0: *const ::core::ffi::c_uchar =
        (*item).name.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
            as *const ::core::ffi::c_uchar;
    _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i_0 = _hj_j_0;
    _hj_k_0 = strlen((*item).name) as ::core::ffi::c_uint;
    while _hj_k_0 >= 12 as ::core::ffi::c_uint {
        _hj_i_0 = _hj_i_0.wrapping_add(
            (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hj_j_0 = _hj_j_0.wrapping_add(
            (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _ha_hashv = _ha_hashv.wrapping_add(
            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
    _ha_hashv = _ha_hashv.wrapping_add(strlen((*item).name) as ::core::ffi::c_uint);
    let mut current_block_205: u64;
    match _hj_k_0 {
        11 => {
            _ha_hashv = _ha_hashv.wrapping_add(
                (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_205 = 1839161661621657450;
        }
        10 => {
            current_block_205 = 1839161661621657450;
        }
        9 => {
            current_block_205 = 16860414839728626549;
        }
        8 => {
            current_block_205 = 7305183797713827960;
        }
        7 => {
            current_block_205 = 7755468498603755133;
        }
        6 => {
            current_block_205 = 12200266031468625273;
        }
        5 => {
            current_block_205 = 17932084609987081382;
        }
        4 => {
            current_block_205 = 12667862594686939941;
        }
        3 => {
            current_block_205 = 5284947427293792646;
        }
        2 => {
            current_block_205 = 14529294992491918670;
        }
        1 => {
            current_block_205 = 7890507820729860149;
        }
        _ => {
            current_block_205 = 16580259026179177070;
        }
    }
    match current_block_205 {
        1839161661621657450 => {
            _ha_hashv = _ha_hashv.wrapping_add(
                (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_205 = 16860414839728626549;
        }
        _ => {}
    }
    match current_block_205 {
        16860414839728626549 => {
            _ha_hashv = _ha_hashv.wrapping_add(
                (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_205 = 7305183797713827960;
        }
        _ => {}
    }
    match current_block_205 {
        7305183797713827960 => {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_205 = 7755468498603755133;
        }
        _ => {}
    }
    match current_block_205 {
        7755468498603755133 => {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_205 = 12200266031468625273;
        }
        _ => {}
    }
    match current_block_205 {
        12200266031468625273 => {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_205 = 17932084609987081382;
        }
        _ => {}
    }
    match current_block_205 {
        17932084609987081382 => {
            _hj_j_0 = _hj_j_0.wrapping_add(
                *_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_205 = 12667862594686939941;
        }
        _ => {}
    }
    match current_block_205 {
        12667862594686939941 => {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_205 = 5284947427293792646;
        }
        _ => {}
    }
    match current_block_205 {
        5284947427293792646 => {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_205 = 14529294992491918670;
        }
        _ => {}
    }
    match current_block_205 {
        14529294992491918670 => {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_205 = 7890507820729860149;
        }
        _ => {}
    }
    match current_block_205 {
        7890507820729860149 => {
            _hj_i_0 = _hj_i_0.wrapping_add(
                *_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
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
    (*item).hh.hashv = _ha_hashv;
    (*item).hh.key = (*item).name.offset(0 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
    (*item).hh.keylen = strlen((*item).name) as ::core::ffi::c_uint;
    if (*lh).is_null() {
        (*item).hh.next = NULL;
        (*item).hh.prev = NULL;
        (*item).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
            as *mut UT_hash_table as *mut UT_hash_table;
        if (*item).hh.tbl.is_null() {
            exit(-(1 as ::core::ffi::c_int));
        } else {
            memset(
                (*item).hh.tbl as *mut ::core::ffi::c_void,
                '\0' as i32,
                ::core::mem::size_of::<UT_hash_table>() as usize,
            );
            (*(*item).hh.tbl).tail = &raw mut (*item).hh as *mut UT_hash_handle;
            (*(*item).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
            (*(*item).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
            (*(*item).hh.tbl).hho = (&raw mut (*item).hh as *mut ::core::ffi::c_char)
                .offset_from(item as *mut ::core::ffi::c_char)
                as ::core::ffi::c_long as isize;
            (*(*item).hh.tbl).buckets = malloc(
                (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            ) as *mut UT_hash_bucket;
            (*(*item).hh.tbl).signature = HASH_SIGNATURE as u32;
            if (*(*item).hh.tbl).buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                );
            }
        }
        *lh = item;
    } else {
        (*item).hh.tbl = (**lh).hh.tbl;
        (*item).hh.next = NULL;
        (*item).hh.prev = ((*(**lh).hh.tbl).tail as *mut ::core::ffi::c_char)
            .offset(-(*(**lh).hh.tbl).hho)
            as *mut ::core::ffi::c_void;
        (*(*(**lh).hh.tbl).tail).next = item as *mut ::core::ffi::c_void;
        (*(**lh).hh.tbl).tail = &raw mut (*item).hh as *mut UT_hash_handle;
    }
    let mut _ha_bkt: ::core::ffi::c_uint = 0;
    (*(**lh).hh.tbl).num_items = (*(**lh).hh.tbl).num_items.wrapping_add(1);
    _ha_bkt = _ha_hashv
        & (*(**lh).hh.tbl)
            .num_buckets
            .wrapping_sub(1 as ::core::ffi::c_uint);
    let mut _ha_head: *mut UT_hash_bucket =
        (*(**lh).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
    (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
    (*item).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
    (*item).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
    if !(*_ha_head).hh_head.is_null() {
        (*(*_ha_head).hh_head).hh_prev = &raw mut (*item).hh as *mut UT_hash_handle;
    }
    (*_ha_head).hh_head = &raw mut (*item).hh as *mut UT_hash_handle;
    if (*_ha_head).count
        >= (*_ha_head)
            .expand_mult
            .wrapping_add(1 as ::core::ffi::c_uint)
            .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
        && (*(*item).hh.tbl).noexpand == 0
    {
        let mut _he_bkt: ::core::ffi::c_uint = 0;
        let mut _he_bkt_i: ::core::ffi::c_uint = 0;
        let mut _he_thh: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
        let mut _he_hh_nxt: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
        let mut _he_new_buckets: *mut UT_hash_bucket = ::core::ptr::null_mut::<UT_hash_bucket>();
        let mut _he_newbkt: *mut UT_hash_bucket = ::core::ptr::null_mut::<UT_hash_bucket>();
        _he_new_buckets = malloc(
            (2 as usize)
                .wrapping_mul((*(*item).hh.tbl).num_buckets as usize)
                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets.is_null() {
            exit(-(1 as ::core::ffi::c_int));
        } else {
            memset(
                _he_new_buckets as *mut ::core::ffi::c_void,
                '\0' as i32,
                (2 as usize)
                    .wrapping_mul((*(*item).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            );
            (*(*item).hh.tbl).ideal_chain_maxlen = ((*(*item).hh.tbl).num_items
                >> (*(*item).hh.tbl)
                    .log2_num_buckets
                    .wrapping_add(1 as ::core::ffi::c_uint))
            .wrapping_add(
                if (*(*item).hh.tbl).num_items
                    & (*(*item).hh.tbl)
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
            (*(*item).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
            _he_bkt_i = 0 as ::core::ffi::c_uint;
            while _he_bkt_i < (*(*item).hh.tbl).num_buckets {
                _he_thh = (*(*(*item).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                    as *mut UT_hash_handle;
                while !_he_thh.is_null() {
                    _he_hh_nxt = (*_he_thh).hh_next;
                    _he_bkt = (*_he_thh).hashv
                        & (*(*item).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint)
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    _he_newbkt = _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                    (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                    if (*_he_newbkt).count > (*(*item).hh.tbl).ideal_chain_maxlen {
                        (*(*item).hh.tbl).nonideal_items =
                            (*(*item).hh.tbl).nonideal_items.wrapping_add(1);
                        (*_he_newbkt).expand_mult = (*_he_newbkt)
                            .count
                            .wrapping_div((*(*item).hh.tbl).ideal_chain_maxlen);
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
            free((*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void);
            (*(*item).hh.tbl).num_buckets = (*(*item).hh.tbl)
                .num_buckets
                .wrapping_mul(2 as ::core::ffi::c_uint);
            (*(*item).hh.tbl).log2_num_buckets = (*(*item).hh.tbl).log2_num_buckets.wrapping_add(1);
            (*(*item).hh.tbl).buckets = _he_new_buckets;
            (*(*item).hh.tbl).ineff_expands = if (*(*item).hh.tbl).nonideal_items
                > (*(*item).hh.tbl).num_items >> 1 as ::core::ffi::c_int
            {
                (*(*item).hh.tbl)
                    .ineff_expands
                    .wrapping_add(1 as ::core::ffi::c_uint)
            } else {
                0 as ::core::ffi::c_uint
            };
            if (*(*item).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                (*(*item).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
            }
        }
    }
    return true;
}
unsafe extern "C" fn figureOutLookupsFromJSON(
    mut lookups: *mut json_value,
    mut options: *const otfcc_Options,
) -> *mut lookup_hash {
    let mut lh: *mut lookup_hash = ::core::ptr::null_mut::<lookup_hash>();
    let mut j: u32 = 0 as u32;
    while j < (*lookups).u.object.length as u32 {
        let mut lookupName: *mut ::core::ffi::c_char =
            (*(*lookups).u.object.values.offset(j as isize)).name;
        if (*(*(*lookups).u.object.values.offset(j as isize)).value).type_0 as ::core::ffi::c_uint
            == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut parsed: bool = _parse_lookup(
                (*(*lookups).u.object.values.offset(j as isize)).value as *mut json_value,
                lookupName,
                options,
                &raw mut lh,
            );
            if !parsed {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[OTFCC-fea] Ignoring invalid or unsupported lookup %s.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        lookupName,
                    ),
                );
            }
        } else if (*(*(*lookups).u.object.values.offset(j as isize)).value).type_0
            as ::core::ffi::c_uint
            == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut thatname: *mut ::core::ffi::c_char =
                (*(*(*lookups).u.object.values.offset(j as isize)).value)
                    .u
                    .string
                    .ptr;
            let mut s: *mut lookup_hash = ::core::ptr::null_mut::<lookup_hash>();
            let mut _hf_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i: ::core::ffi::c_uint = 0;
            let mut _hj_j: ::core::ffi::c_uint = 0;
            let mut _hj_k: ::core::ffi::c_uint = 0;
            let mut _hj_key: *const ::core::ffi::c_uchar = thatname as *const ::core::ffi::c_uchar;
            _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i = _hj_j;
            _hj_k = strlen(thatname) as ::core::ffi::c_uint;
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
            _hf_hashv = _hf_hashv.wrapping_add(strlen(thatname) as ::core::ffi::c_uint);
            let mut current_block_55: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_55 = 6724697507717081597;
                }
                10 => {
                    current_block_55 = 6724697507717081597;
                }
                9 => {
                    current_block_55 = 18439360672589124415;
                }
                8 => {
                    current_block_55 = 3470465739294658462;
                }
                7 => {
                    current_block_55 = 6898480480848611516;
                }
                6 => {
                    current_block_55 = 14457307756934643757;
                }
                5 => {
                    current_block_55 = 1742737506509593365;
                }
                4 => {
                    current_block_55 = 9115561672764076001;
                }
                3 => {
                    current_block_55 = 16665723564706850773;
                }
                2 => {
                    current_block_55 = 18217527008902992848;
                }
                1 => {
                    current_block_55 = 2550976313518393825;
                }
                _ => {
                    current_block_55 = 3689906465960840878;
                }
            }
            match current_block_55 {
                6724697507717081597 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_55 = 18439360672589124415;
                }
                _ => {}
            }
            match current_block_55 {
                18439360672589124415 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_55 = 3470465739294658462;
                }
                _ => {}
            }
            match current_block_55 {
                3470465739294658462 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_55 = 6898480480848611516;
                }
                _ => {}
            }
            match current_block_55 {
                6898480480848611516 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_55 = 14457307756934643757;
                }
                _ => {}
            }
            match current_block_55 {
                14457307756934643757 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_55 = 1742737506509593365;
                }
                _ => {}
            }
            match current_block_55 {
                1742737506509593365 => {
                    _hj_j = _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_55 = 9115561672764076001;
                }
                _ => {}
            }
            match current_block_55 {
                9115561672764076001 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_55 = 16665723564706850773;
                }
                _ => {}
            }
            match current_block_55 {
                16665723564706850773 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_55 = 18217527008902992848;
                }
                _ => {}
            }
            match current_block_55 {
                18217527008902992848 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_55 = 2550976313518393825;
                }
                _ => {}
            }
            match current_block_55 {
                2550976313518393825 => {
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
            s = ::core::ptr::null_mut::<lookup_hash>();
            if !lh.is_null() {
                let mut _hf_bkt: ::core::ffi::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & (*(*lh).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(*lh).hh.tbl).buckets.offset(_hf_bkt as isize))
                        .hh_head
                        .is_null()
                    {
                        s = ((*(*(*lh).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(*lh).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut lookup_hash as *mut lookup_hash;
                    } else {
                        s = ::core::ptr::null_mut::<lookup_hash>();
                    }
                    while !s.is_null() {
                        if (*s).hh.hashv == _hf_hashv
                            && (*s).hh.keylen == strlen(thatname) as ::core::ffi::c_uint
                        {
                            if memcmp(
                                (*s).hh.key,
                                thatname as *const ::core::ffi::c_void,
                                strlen(thatname) as ::core::ffi::c_uint as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*s).hh.hh_next.is_null() {
                            s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(*lh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut lookup_hash
                                as *mut lookup_hash;
                        } else {
                            s = ::core::ptr::null_mut::<lookup_hash>();
                        }
                    }
                }
            }
            if !s.is_null() {
                let mut dup: *mut lookup_hash = ::core::ptr::null_mut::<lookup_hash>();
                dup = __caryll_allocate_clean(
                    ::core::mem::size_of::<lookup_hash>() as usize,
                    132 as ::core::ffi::c_ulong,
                ) as *mut lookup_hash;
                (*dup).name = sdsnew(lookupName) as *mut ::core::ffi::c_char;
                (*dup).lookup = (*s).lookup;
                (*dup).orderType = LOOKUP_ORDER_FILE;
                (*dup).orderVal = (if !lh.is_null() {
                    (*(*lh).hh.tbl).num_items
                } else {
                    0 as ::core::ffi::c_uint
                }) as u16;
                let mut _ha_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i_0: ::core::ffi::c_uint = 0;
                let mut _hj_j_0: ::core::ffi::c_uint = 0;
                let mut _hj_k_0: ::core::ffi::c_uint = 0;
                let mut _hj_key_0: *const ::core::ffi::c_uchar = (*dup)
                    .name
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_uchar;
                _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_0 = _hj_j_0;
                _hj_k_0 = strlen((*dup).name) as ::core::ffi::c_uint;
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
                _ha_hashv = _ha_hashv.wrapping_add(strlen((*dup).name) as ::core::ffi::c_uint);
                let mut current_block_174: u64;
                match _hj_k_0 {
                    11 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_174 = 7040648290900150297;
                    }
                    10 => {
                        current_block_174 = 7040648290900150297;
                    }
                    9 => {
                        current_block_174 = 5642623898150869965;
                    }
                    8 => {
                        current_block_174 = 11200057012384285421;
                    }
                    7 => {
                        current_block_174 = 10179178712498762659;
                    }
                    6 => {
                        current_block_174 = 18120062161455140658;
                    }
                    5 => {
                        current_block_174 = 4992233192120486274;
                    }
                    4 => {
                        current_block_174 = 9894433423109553829;
                    }
                    3 => {
                        current_block_174 = 14164855200032804067;
                    }
                    2 => {
                        current_block_174 = 2054541336423690087;
                    }
                    1 => {
                        current_block_174 = 12820098332637308464;
                    }
                    _ => {
                        current_block_174 = 5710330377809666066;
                    }
                }
                match current_block_174 {
                    7040648290900150297 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_174 = 5642623898150869965;
                    }
                    _ => {}
                }
                match current_block_174 {
                    5642623898150869965 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_174 = 11200057012384285421;
                    }
                    _ => {}
                }
                match current_block_174 {
                    11200057012384285421 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_174 = 10179178712498762659;
                    }
                    _ => {}
                }
                match current_block_174 {
                    10179178712498762659 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_174 = 18120062161455140658;
                    }
                    _ => {}
                }
                match current_block_174 {
                    18120062161455140658 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_174 = 4992233192120486274;
                    }
                    _ => {}
                }
                match current_block_174 {
                    4992233192120486274 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_174 = 9894433423109553829;
                    }
                    _ => {}
                }
                match current_block_174 {
                    9894433423109553829 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_174 = 14164855200032804067;
                    }
                    _ => {}
                }
                match current_block_174 {
                    14164855200032804067 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_174 = 2054541336423690087;
                    }
                    _ => {}
                }
                match current_block_174 {
                    2054541336423690087 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_174 = 12820098332637308464;
                    }
                    _ => {}
                }
                match current_block_174 {
                    12820098332637308464 => {
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
                (*dup).hh.hashv = _ha_hashv;
                (*dup).hh.key = (*dup).name.offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void;
                (*dup).hh.keylen = strlen((*dup).name) as ::core::ffi::c_uint;
                if lh.is_null() {
                    (*dup).hh.next = NULL;
                    (*dup).hh.prev = NULL;
                    (*dup).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                        as *mut UT_hash_table
                        as *mut UT_hash_table;
                    if (*dup).hh.tbl.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*dup).hh.tbl as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            ::core::mem::size_of::<UT_hash_table>() as usize,
                        );
                        (*(*dup).hh.tbl).tail = &raw mut (*dup).hh as *mut UT_hash_handle;
                        (*(*dup).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                        (*(*dup).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                        (*(*dup).hh.tbl).hho = (&raw mut (*dup).hh as *mut ::core::ffi::c_char)
                            .offset_from(dup as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long
                            as isize;
                        (*(*dup).hh.tbl).buckets = malloc(
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        ) as *mut UT_hash_bucket;
                        (*(*dup).hh.tbl).signature = HASH_SIGNATURE as u32;
                        if (*(*dup).hh.tbl).buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*(*dup).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (32 as usize).wrapping_mul(
                                    ::core::mem::size_of::<UT_hash_bucket>() as usize,
                                ),
                            );
                        }
                    }
                    lh = dup;
                } else {
                    (*dup).hh.tbl = (*lh).hh.tbl;
                    (*dup).hh.next = NULL;
                    (*dup).hh.prev = ((*(*lh).hh.tbl).tail as *mut ::core::ffi::c_char)
                        .offset(-(*(*lh).hh.tbl).hho)
                        as *mut ::core::ffi::c_void;
                    (*(*(*lh).hh.tbl).tail).next = dup as *mut ::core::ffi::c_void;
                    (*(*lh).hh.tbl).tail = &raw mut (*dup).hh as *mut UT_hash_handle;
                }
                let mut _ha_bkt: ::core::ffi::c_uint = 0;
                (*(*lh).hh.tbl).num_items = (*(*lh).hh.tbl).num_items.wrapping_add(1);
                _ha_bkt = _ha_hashv
                    & (*(*lh).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head: *mut UT_hash_bucket =
                    (*(*lh).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                (*dup).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
                (*dup).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                if !(*_ha_head).hh_head.is_null() {
                    (*(*_ha_head).hh_head).hh_prev = &raw mut (*dup).hh as *mut UT_hash_handle;
                }
                (*_ha_head).hh_head = &raw mut (*dup).hh as *mut UT_hash_handle;
                if (*_ha_head).count
                    >= (*_ha_head)
                        .expand_mult
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                    && (*(*dup).hh.tbl).noexpand == 0
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
                        (2 as usize)
                            .wrapping_mul((*(*dup).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    ) as *mut UT_hash_bucket;
                    if _he_new_buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            _he_new_buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (2 as usize)
                                .wrapping_mul((*(*dup).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        );
                        (*(*dup).hh.tbl).ideal_chain_maxlen = ((*(*dup).hh.tbl).num_items
                            >> (*(*dup).hh.tbl)
                                .log2_num_buckets
                                .wrapping_add(1 as ::core::ffi::c_uint))
                        .wrapping_add(
                            if (*(*dup).hh.tbl).num_items
                                & (*(*dup).hh.tbl)
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
                        (*(*dup).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                        _he_bkt_i = 0 as ::core::ffi::c_uint;
                        while _he_bkt_i < (*(*dup).hh.tbl).num_buckets {
                            _he_thh = (*(*(*dup).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                                as *mut UT_hash_handle;
                            while !_he_thh.is_null() {
                                _he_hh_nxt = (*_he_thh).hh_next;
                                _he_bkt = (*_he_thh).hashv
                                    & (*(*dup).hh.tbl)
                                        .num_buckets
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint);
                                _he_newbkt =
                                    _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                                (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                if (*_he_newbkt).count > (*(*dup).hh.tbl).ideal_chain_maxlen {
                                    (*(*dup).hh.tbl).nonideal_items =
                                        (*(*dup).hh.tbl).nonideal_items.wrapping_add(1);
                                    (*_he_newbkt).expand_mult = (*_he_newbkt)
                                        .count
                                        .wrapping_div((*(*dup).hh.tbl).ideal_chain_maxlen);
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
                        free((*(*dup).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        (*(*dup).hh.tbl).num_buckets = (*(*dup).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint);
                        (*(*dup).hh.tbl).log2_num_buckets =
                            (*(*dup).hh.tbl).log2_num_buckets.wrapping_add(1);
                        (*(*dup).hh.tbl).buckets = _he_new_buckets;
                        (*(*dup).hh.tbl).ineff_expands = if (*(*dup).hh.tbl).nonideal_items
                            > (*(*dup).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                        {
                            (*(*dup).hh.tbl)
                                .ineff_expands
                                .wrapping_add(1 as ::core::ffi::c_uint)
                        } else {
                            0 as ::core::ffi::c_uint
                        };
                        if (*(*dup).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                            (*(*dup).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                        }
                    }
                }
            }
        }
        j = j.wrapping_add(1);
    }
    return lh;
}
unsafe extern "C" fn feature_merger_activate(
    mut d: *mut json_value,
    sametag: bool,
    mut objtype: *const ::core::ffi::c_char,
    mut options: *const otfcc_Options,
) {
    let mut j: u32 = 0 as u32;
    while j < (*d).u.object.length as u32 {
        let mut jthis: *mut json_value =
            (*(*d).u.object.values.offset(j as isize)).value as *mut json_value;
        let mut kthis: *mut ::core::ffi::c_char = (*(*d).u.object.values.offset(j as isize)).name;
        let mut nkthis: u32 =
            (*(*d).u.object.values.offset(j as isize)).name_length as u32;
        if !((*jthis).type_0 as ::core::ffi::c_uint
            != json_array as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*jthis).type_0 as ::core::ffi::c_uint
                != json_object as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            let mut k: u32 = j.wrapping_add(1 as u32);
            while k < (*d).u.object.length as u32 {
                let mut jthat: *mut json_value =
                    (*(*d).u.object.values.offset(k as isize)).value as *mut json_value;
                let mut kthat: *mut ::core::ffi::c_char =
                    (*(*d).u.object.values.offset(k as isize)).name;
                if json_ident(jthis, jthat) as ::core::ffi::c_int != 0
                    && (if sametag as ::core::ffi::c_int != 0 {
                        (strncmp(kthis, kthat, 4 as usize) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                    } else {
                        true_0
                    }) != 0
                {
                    json_value_free(jthat);
                    let mut v: *mut json_value =
                        json_string_new_length(nkthis as ::core::ffi::c_uint, kthis);
                    (*v).parent = d as *mut _json_value;
                    let ref mut fresh6 = (*(*d).u.object.values.offset(k as isize)).value;
                    *fresh6 = v as *mut _json_value;
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_notice as ::core::ffi::c_int as u8,
                        log_type_info,
                        sdscatprintf(
                            sdsempty(),
                            b"[OTFCC-fea] Merged duplicate %s '%s' into '%s'.\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            objtype,
                            kthat,
                            kthis,
                        ),
                    );
                }
                k = k.wrapping_add(1);
            }
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn figureOutFeaturesFromJSON(
    mut features: *mut json_value,
    mut lh: *mut lookup_hash,
    mut tag: *const ::core::ffi::c_char,
    mut options: *const otfcc_Options,
) -> *mut feature_hash {
    let mut fh: *mut feature_hash = ::core::ptr::null_mut::<feature_hash>();
    if (*options).merge_features {
        feature_merger_activate(
            features,
            true,
            b"feature\0" as *const u8 as *const ::core::ffi::c_char,
            options,
        );
    }
    let mut j: u32 = 0 as u32;
    while j < (*features).u.object.length as u32 {
        let mut featureName: *mut ::core::ffi::c_char =
            (*(*features).u.object.values.offset(j as isize)).name;
        let mut _feature: *mut json_value =
            (*(*features).u.object.values.offset(j as isize)).value as *mut json_value;
        if (*_feature).type_0 as ::core::ffi::c_uint
            == json_array as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut al: otl_LookupRefList = otl_LookupRefList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<otl_LookupRef>(),
            };
            otl_iLookupRefList.init.expect("non-null function pointer")(&raw mut al);
            let mut k: tableid_t = 0 as tableid_t;
            while (k as ::core::ffi::c_uint) < (*_feature).u.array.length {
                let mut term: *mut json_value =
                    *(*_feature).u.array.values.offset(k as isize) as *mut json_value;
                if !((*term).type_0 as ::core::ffi::c_uint
                    != json_string as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    let mut item: *mut lookup_hash = ::core::ptr::null_mut::<lookup_hash>();
                    let mut _hf_hashv: ::core::ffi::c_uint = 0;
                    let mut _hj_i: ::core::ffi::c_uint = 0;
                    let mut _hj_j: ::core::ffi::c_uint = 0;
                    let mut _hj_k: ::core::ffi::c_uint = 0;
                    let mut _hj_key: *const ::core::ffi::c_uchar =
                        (*term).u.string.ptr as *const ::core::ffi::c_uchar;
                    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
                    _hj_i = _hj_j;
                    _hj_k = strlen((*term).u.string.ptr) as ::core::ffi::c_uint;
                    while _hj_k >= 12 as ::core::ffi::c_uint {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                    _hf_hashv =
                        _hf_hashv.wrapping_add(strlen((*term).u.string.ptr) as ::core::ffi::c_uint);
                    let mut current_block_54: u64;
                    match _hj_k {
                        11 => {
                            _hf_hashv = _hf_hashv.wrapping_add(
                                (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_54 = 17721430370233415751;
                        }
                        10 => {
                            current_block_54 = 17721430370233415751;
                        }
                        9 => {
                            current_block_54 = 8989441811446665239;
                        }
                        8 => {
                            current_block_54 = 14495838136885722642;
                        }
                        7 => {
                            current_block_54 = 9385799528419063019;
                        }
                        6 => {
                            current_block_54 = 3109151268519014388;
                        }
                        5 => {
                            current_block_54 = 4656740366693513239;
                        }
                        4 => {
                            current_block_54 = 15074489574687793720;
                        }
                        3 => {
                            current_block_54 = 17318996621413929546;
                        }
                        2 => {
                            current_block_54 = 3546546713574234235;
                        }
                        1 => {
                            current_block_54 = 76983250810441849;
                        }
                        _ => {
                            current_block_54 = 10095721787123848864;
                        }
                    }
                    match current_block_54 {
                        17721430370233415751 => {
                            _hf_hashv = _hf_hashv.wrapping_add(
                                (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_54 = 8989441811446665239;
                        }
                        _ => {}
                    }
                    match current_block_54 {
                        8989441811446665239 => {
                            _hf_hashv = _hf_hashv.wrapping_add(
                                (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_54 = 14495838136885722642;
                        }
                        _ => {}
                    }
                    match current_block_54 {
                        14495838136885722642 => {
                            _hj_j = _hj_j.wrapping_add(
                                (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_54 = 9385799528419063019;
                        }
                        _ => {}
                    }
                    match current_block_54 {
                        9385799528419063019 => {
                            _hj_j = _hj_j.wrapping_add(
                                (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_54 = 3109151268519014388;
                        }
                        _ => {}
                    }
                    match current_block_54 {
                        3109151268519014388 => {
                            _hj_j = _hj_j.wrapping_add(
                                (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_54 = 4656740366693513239;
                        }
                        _ => {}
                    }
                    match current_block_54 {
                        4656740366693513239 => {
                            _hj_j = _hj_j
                                .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_54 = 15074489574687793720;
                        }
                        _ => {}
                    }
                    match current_block_54 {
                        15074489574687793720 => {
                            _hj_i = _hj_i.wrapping_add(
                                (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_54 = 17318996621413929546;
                        }
                        _ => {}
                    }
                    match current_block_54 {
                        17318996621413929546 => {
                            _hj_i = _hj_i.wrapping_add(
                                (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_54 = 3546546713574234235;
                        }
                        _ => {}
                    }
                    match current_block_54 {
                        3546546713574234235 => {
                            _hj_i = _hj_i.wrapping_add(
                                (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_54 = 76983250810441849;
                        }
                        _ => {}
                    }
                    match current_block_54 {
                        76983250810441849 => {
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
                    item = ::core::ptr::null_mut::<lookup_hash>();
                    if !lh.is_null() {
                        let mut _hf_bkt: ::core::ffi::c_uint = 0;
                        _hf_bkt = _hf_hashv
                            & (*(*lh).hh.tbl)
                                .num_buckets
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            if !(*(*(*lh).hh.tbl).buckets.offset(_hf_bkt as isize))
                                .hh_head
                                .is_null()
                            {
                                item = ((*(*(*lh).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                                    as *mut ::core::ffi::c_char)
                                    .offset(-(*(*lh).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut lookup_hash
                                    as *mut lookup_hash;
                            } else {
                                item = ::core::ptr::null_mut::<lookup_hash>();
                            }
                            while !item.is_null() {
                                if (*item).hh.hashv == _hf_hashv
                                    && (*item).hh.keylen
                                        == strlen((*term).u.string.ptr) as ::core::ffi::c_uint
                                {
                                    if memcmp(
                                        (*item).hh.key,
                                        (*term).u.string.ptr as *const ::core::ffi::c_void,
                                        strlen((*term).u.string.ptr) as ::core::ffi::c_uint
                                            as usize,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        break;
                                    }
                                }
                                if !(*item).hh.hh_next.is_null() {
                                    item = ((*item).hh.hh_next as *mut ::core::ffi::c_char)
                                        .offset(-(*(*lh).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut lookup_hash
                                        as *mut lookup_hash;
                                } else {
                                    item = ::core::ptr::null_mut::<lookup_hash>();
                                }
                            }
                        }
                    }
                    if !item.is_null() {
                        otl_iLookupRefList.push.expect("non-null function pointer")(
                            &raw mut al,
                            (*item).lookup as otl_LookupRef,
                        );
                    } else {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Lookup assignment %s for feature [%s/%s] is missing or invalid.\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                (*term).u.string.ptr,
                                tag,
                                featureName,
                            ),
                        );
                    }
                }
                k = k.wrapping_add(1);
            }
            if al.length > 0 as usize {
                let mut s: *mut feature_hash = ::core::ptr::null_mut::<feature_hash>();
                let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
                let mut _hj_i_0: ::core::ffi::c_uint = 0;
                let mut _hj_j_0: ::core::ffi::c_uint = 0;
                let mut _hj_k_0: ::core::ffi::c_uint = 0;
                let mut _hj_key_0: *const ::core::ffi::c_uchar =
                    featureName as *const ::core::ffi::c_uchar;
                _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_0 = _hj_j_0;
                _hj_k_0 = strlen(featureName) as ::core::ffi::c_uint;
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
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
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
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                    _hj_i_0 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                    _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
                    _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                    _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                    _hf_hashv_0 ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                    _hj_i_0 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                    _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
                    _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                    _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                    _hf_hashv_0 ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                    _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                    _hj_i_0 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                    _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
                    _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                    _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                    _hf_hashv_0 ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
                    _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
                    _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
                }
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(strlen(featureName) as ::core::ffi::c_uint);
                let mut current_block_174: u64;
                match _hj_k_0 {
                    11 => {
                        _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_174 = 11770592579215200246;
                    }
                    10 => {
                        current_block_174 = 11770592579215200246;
                    }
                    9 => {
                        current_block_174 = 8125531904878226627;
                    }
                    8 => {
                        current_block_174 = 599766223673493735;
                    }
                    7 => {
                        current_block_174 = 14613244978816316141;
                    }
                    6 => {
                        current_block_174 = 4662395312902516269;
                    }
                    5 => {
                        current_block_174 = 965693511569723167;
                    }
                    4 => {
                        current_block_174 = 4667075389960159514;
                    }
                    3 => {
                        current_block_174 = 12015016339672880454;
                    }
                    2 => {
                        current_block_174 = 2956205457714533372;
                    }
                    1 => {
                        current_block_174 = 14454443728975209530;
                    }
                    _ => {
                        current_block_174 = 17727836384662615028;
                    }
                }
                match current_block_174 {
                    11770592579215200246 => {
                        _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_174 = 8125531904878226627;
                    }
                    _ => {}
                }
                match current_block_174 {
                    8125531904878226627 => {
                        _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_174 = 599766223673493735;
                    }
                    _ => {}
                }
                match current_block_174 {
                    599766223673493735 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_174 = 14613244978816316141;
                    }
                    _ => {}
                }
                match current_block_174 {
                    14613244978816316141 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_174 = 4662395312902516269;
                    }
                    _ => {}
                }
                match current_block_174 {
                    4662395312902516269 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_174 = 965693511569723167;
                    }
                    _ => {}
                }
                match current_block_174 {
                    965693511569723167 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_174 = 4667075389960159514;
                    }
                    _ => {}
                }
                match current_block_174 {
                    4667075389960159514 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_174 = 12015016339672880454;
                    }
                    _ => {}
                }
                match current_block_174 {
                    12015016339672880454 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_174 = 2956205457714533372;
                    }
                    _ => {}
                }
                match current_block_174 {
                    2956205457714533372 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_174 = 14454443728975209530;
                    }
                    _ => {}
                }
                match current_block_174 {
                    14454443728975209530 => {
                        _hj_i_0 = _hj_i_0
                            .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                _hj_i_0 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                _hf_hashv_0 ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                _hj_i_0 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                _hf_hashv_0 ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                _hj_i_0 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                _hf_hashv_0 ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
                s = ::core::ptr::null_mut::<feature_hash>();
                if !fh.is_null() {
                    let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
                    _hf_bkt_0 = _hf_hashv_0
                        & (*(*fh).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if !(*(*(*fh).hh.tbl).buckets.offset(_hf_bkt_0 as isize))
                            .hh_head
                            .is_null()
                        {
                            s = ((*(*(*fh).hh.tbl).buckets.offset(_hf_bkt_0 as isize)).hh_head
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(*fh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut feature_hash
                                as *mut feature_hash;
                        } else {
                            s = ::core::ptr::null_mut::<feature_hash>();
                        }
                        while !s.is_null() {
                            if (*s).hh.hashv == _hf_hashv_0
                                && (*s).hh.keylen == strlen(featureName) as ::core::ffi::c_uint
                            {
                                if memcmp(
                                    (*s).hh.key,
                                    featureName as *const ::core::ffi::c_void,
                                    strlen(featureName) as ::core::ffi::c_uint as usize,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if !(*s).hh.hh_next.is_null() {
                                s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                    .offset(-(*(*fh).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut feature_hash
                                    as *mut feature_hash;
                            } else {
                                s = ::core::ptr::null_mut::<feature_hash>();
                            }
                        }
                    }
                }
                if s.is_null() {
                    s = __caryll_allocate_clean(
                        ::core::mem::size_of::<feature_hash>() as usize,
                        194 as ::core::ffi::c_ulong,
                    ) as *mut feature_hash;
                    (*s).name = sdsnew(featureName) as *mut ::core::ffi::c_char;
                    (*s).alias = false;
                    otl_iFeaturePtr.init.expect("non-null function pointer")(&raw mut (*s).feature);
                    (*(*s).feature).name = sdsdup((*s).name as sds);
                    otl_iLookupRefList
                        .replace
                        .expect("non-null function pointer")(
                        &raw mut (*(*s).feature).lookups, al
                    );
                    let mut _ha_hashv: ::core::ffi::c_uint = 0;
                    let mut _hj_i_1: ::core::ffi::c_uint = 0;
                    let mut _hj_j_1: ::core::ffi::c_uint = 0;
                    let mut _hj_k_1: ::core::ffi::c_uint = 0;
                    let mut _hj_key_1: *const ::core::ffi::c_uchar =
                        (*s).name.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_uchar;
                    _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                    _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
                    _hj_i_1 = _hj_j_1;
                    _hj_k_1 = strlen((*s).name) as ::core::ffi::c_uint;
                    while _hj_k_1 >= 12 as ::core::ffi::c_uint {
                        _hj_i_1 = _hj_i_1.wrapping_add(
                            (*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
                        _hj_i_1 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
                        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                        _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
                        _ha_hashv ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
                        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
                        _hj_i_1 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
                        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                        _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
                        _ha_hashv ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
                        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
                        _hj_i_1 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
                        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                        _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
                        _ha_hashv ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
                        _hj_key_1 = _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
                        _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
                    }
                    _ha_hashv = _ha_hashv.wrapping_add(strlen((*s).name) as ::core::ffi::c_uint);
                    let mut current_block_294: u64;
                    match _hj_k_1 {
                        11 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_294 = 1042651977803078134;
                        }
                        10 => {
                            current_block_294 = 1042651977803078134;
                        }
                        9 => {
                            current_block_294 = 3623657252432734002;
                        }
                        8 => {
                            current_block_294 = 12654965358034578036;
                        }
                        7 => {
                            current_block_294 = 5027969324473645360;
                        }
                        6 => {
                            current_block_294 = 13502505478735984184;
                        }
                        5 => {
                            current_block_294 = 1044878420175362827;
                        }
                        4 => {
                            current_block_294 = 10288570686179517515;
                        }
                        3 => {
                            current_block_294 = 16395564228731822224;
                        }
                        2 => {
                            current_block_294 = 1619527839380644891;
                        }
                        1 => {
                            current_block_294 = 16780668966729750739;
                        }
                        _ => {
                            current_block_294 = 6055351187523413397;
                        }
                    }
                    match current_block_294 {
                        1042651977803078134 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_294 = 3623657252432734002;
                        }
                        _ => {}
                    }
                    match current_block_294 {
                        3623657252432734002 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_294 = 12654965358034578036;
                        }
                        _ => {}
                    }
                    match current_block_294 {
                        12654965358034578036 => {
                            _hj_j_1 = _hj_j_1.wrapping_add(
                                (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_294 = 5027969324473645360;
                        }
                        _ => {}
                    }
                    match current_block_294 {
                        5027969324473645360 => {
                            _hj_j_1 = _hj_j_1.wrapping_add(
                                (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_294 = 13502505478735984184;
                        }
                        _ => {}
                    }
                    match current_block_294 {
                        13502505478735984184 => {
                            _hj_j_1 = _hj_j_1.wrapping_add(
                                (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_294 = 1044878420175362827;
                        }
                        _ => {}
                    }
                    match current_block_294 {
                        1044878420175362827 => {
                            _hj_j_1 = _hj_j_1
                                .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_294 = 10288570686179517515;
                        }
                        _ => {}
                    }
                    match current_block_294 {
                        10288570686179517515 => {
                            _hj_i_1 = _hj_i_1.wrapping_add(
                                (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_294 = 16395564228731822224;
                        }
                        _ => {}
                    }
                    match current_block_294 {
                        16395564228731822224 => {
                            _hj_i_1 = _hj_i_1.wrapping_add(
                                (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_294 = 1619527839380644891;
                        }
                        _ => {}
                    }
                    match current_block_294 {
                        1619527839380644891 => {
                            _hj_i_1 = _hj_i_1.wrapping_add(
                                (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_294 = 16780668966729750739;
                        }
                        _ => {}
                    }
                    match current_block_294 {
                        16780668966729750739 => {
                            _hj_i_1 = _hj_i_1
                                .wrapping_add(*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                        }
                        _ => {}
                    }
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                    _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
                    _hj_i_1 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                    _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                    _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
                    _ha_hashv ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                    _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
                    _hj_i_1 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                    _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                    _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
                    _ha_hashv ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                    _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
                    _hj_i_1 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                    _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                    _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
                    _ha_hashv ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
                    (*s).hh.hashv = _ha_hashv;
                    (*s).hh.key = (*s).name.offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void;
                    (*s).hh.keylen = strlen((*s).name) as ::core::ffi::c_uint;
                    if fh.is_null() {
                        (*s).hh.next = NULL;
                        (*s).hh.prev = NULL;
                        (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                            as *mut UT_hash_table
                            as *mut UT_hash_table;
                        if (*s).hh.tbl.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*s).hh.tbl as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                ::core::mem::size_of::<UT_hash_table>() as usize,
                            );
                            (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                            (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                            (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                            (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                                .offset_from(s as *mut ::core::ffi::c_char)
                                as ::core::ffi::c_long
                                as isize;
                            (*(*s).hh.tbl).buckets =
                                malloc((32 as usize).wrapping_mul(::core::mem::size_of::<
                                    UT_hash_bucket,
                                >(
                                )
                                    as usize))
                                    as *mut UT_hash_bucket;
                            (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                            if (*(*s).hh.tbl).buckets.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (32 as usize).wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as usize,
                                    ),
                                );
                            }
                        }
                        fh = s;
                    } else {
                        (*s).hh.tbl = (*fh).hh.tbl;
                        (*s).hh.next = NULL;
                        (*s).hh.prev = ((*(*fh).hh.tbl).tail as *mut ::core::ffi::c_char)
                            .offset(-(*(*fh).hh.tbl).hho)
                            as *mut ::core::ffi::c_void;
                        (*(*(*fh).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                        (*(*fh).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                    }
                    let mut _ha_bkt: ::core::ffi::c_uint = 0;
                    (*(*fh).hh.tbl).num_items = (*(*fh).hh.tbl).num_items.wrapping_add(1);
                    _ha_bkt = _ha_hashv
                        & (*(*fh).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _ha_head: *mut UT_hash_bucket =
                        (*(*fh).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
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
                            (2 as usize)
                                .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        ) as *mut UT_hash_bucket;
                        if _he_new_buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                _he_new_buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (2 as usize)
                                    .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as usize
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
                    (*(*options).logger)
                        .logSDS
                        .expect(
                            "non-null function pointer",
                        )(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        sdscatprintf(
                            sdsempty(),
                            b"[OTFCC-fea] Duplicate feature for [%s/%s]. This feature will be ignored.\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            tag,
                            featureName,
                        ),
                    );
                    otl_iLookupRefList
                        .dispose
                        .expect("non-null function pointer")(&raw mut al);
                }
            } else {
                (*(*options).logger)
                    .logSDS
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[OTFCC-fea] There is no valid lookup assignments for [%s/%s]. This feature will be ignored.\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        tag,
                        featureName,
                    ),
                );
                otl_iLookupRefList
                    .dispose
                    .expect("non-null function pointer")(&raw mut al);
            }
        } else if (*_feature).type_0 as ::core::ffi::c_uint
            == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut s_0: *mut feature_hash = ::core::ptr::null_mut::<feature_hash>();
            let mut target: *mut ::core::ffi::c_char = (*_feature).u.string.ptr;
            let mut _hf_hashv_1: ::core::ffi::c_uint = 0;
            let mut _hj_i_2: ::core::ffi::c_uint = 0;
            let mut _hj_j_2: ::core::ffi::c_uint = 0;
            let mut _hj_k_2: ::core::ffi::c_uint = 0;
            let mut _hj_key_2: *const ::core::ffi::c_uchar = target as *const ::core::ffi::c_uchar;
            _hf_hashv_1 = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_2 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_2 = _hj_j_2;
            _hj_k_2 = strlen(target) as ::core::ffi::c_uint;
            while _hj_k_2 >= 12 as ::core::ffi::c_uint {
                _hj_i_2 = _hj_i_2.wrapping_add(
                    (*_hj_key_2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j_2 = _hj_j_2.wrapping_add(
                    (*_hj_key_2.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                    (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_2.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_1);
                _hj_i_2 ^= _hf_hashv_1 >> 13 as ::core::ffi::c_int;
                _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_1);
                _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                _hj_j_2 ^= _hj_i_2 << 8 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_2);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_2);
                _hf_hashv_1 ^= _hj_j_2 >> 13 as ::core::ffi::c_int;
                _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_1);
                _hj_i_2 ^= _hf_hashv_1 >> 12 as ::core::ffi::c_int;
                _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_1);
                _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                _hj_j_2 ^= _hj_i_2 << 16 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_2);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_2);
                _hf_hashv_1 ^= _hj_j_2 >> 5 as ::core::ffi::c_int;
                _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_1);
                _hj_i_2 ^= _hf_hashv_1 >> 3 as ::core::ffi::c_int;
                _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_1);
                _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                _hj_j_2 ^= _hj_i_2 << 10 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_2);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_2);
                _hf_hashv_1 ^= _hj_j_2 >> 15 as ::core::ffi::c_int;
                _hj_key_2 = _hj_key_2.offset(12 as ::core::ffi::c_int as isize);
                _hj_k_2 = _hj_k_2.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _hf_hashv_1 = _hf_hashv_1.wrapping_add(strlen(target) as ::core::ffi::c_uint);
            let mut current_block_495: u64;
            match _hj_k_2 {
                11 => {
                    _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                        (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_495 = 17747461797571235088;
                }
                10 => {
                    current_block_495 = 17747461797571235088;
                }
                9 => {
                    current_block_495 = 11525807565413539572;
                }
                8 => {
                    current_block_495 = 13355992316862083049;
                }
                7 => {
                    current_block_495 = 9872962151259696607;
                }
                6 => {
                    current_block_495 = 3297121635827725547;
                }
                5 => {
                    current_block_495 = 2720640922275626654;
                }
                4 => {
                    current_block_495 = 18200456723090628504;
                }
                3 => {
                    current_block_495 = 6999931589368679127;
                }
                2 => {
                    current_block_495 = 12727901806336139390;
                }
                1 => {
                    current_block_495 = 4787289807222794095;
                }
                _ => {
                    current_block_495 = 5970770240478038113;
                }
            }
            match current_block_495 {
                17747461797571235088 => {
                    _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                        (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_495 = 11525807565413539572;
                }
                _ => {}
            }
            match current_block_495 {
                11525807565413539572 => {
                    _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                        (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_495 = 13355992316862083049;
                }
                _ => {}
            }
            match current_block_495 {
                13355992316862083049 => {
                    _hj_j_2 = _hj_j_2.wrapping_add(
                        (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_495 = 9872962151259696607;
                }
                _ => {}
            }
            match current_block_495 {
                9872962151259696607 => {
                    _hj_j_2 = _hj_j_2.wrapping_add(
                        (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_495 = 3297121635827725547;
                }
                _ => {}
            }
            match current_block_495 {
                3297121635827725547 => {
                    _hj_j_2 = _hj_j_2.wrapping_add(
                        (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_495 = 2720640922275626654;
                }
                _ => {}
            }
            match current_block_495 {
                2720640922275626654 => {
                    _hj_j_2 = _hj_j_2
                        .wrapping_add(*_hj_key_2.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_495 = 18200456723090628504;
                }
                _ => {}
            }
            match current_block_495 {
                18200456723090628504 => {
                    _hj_i_2 = _hj_i_2.wrapping_add(
                        (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_495 = 6999931589368679127;
                }
                _ => {}
            }
            match current_block_495 {
                6999931589368679127 => {
                    _hj_i_2 = _hj_i_2.wrapping_add(
                        (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_495 = 12727901806336139390;
                }
                _ => {}
            }
            match current_block_495 {
                12727901806336139390 => {
                    _hj_i_2 = _hj_i_2.wrapping_add(
                        (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_495 = 4787289807222794095;
                }
                _ => {}
            }
            match current_block_495 {
                4787289807222794095 => {
                    _hj_i_2 = _hj_i_2
                        .wrapping_add(*_hj_key_2.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                }
                _ => {}
            }
            _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
            _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_1);
            _hj_i_2 ^= _hf_hashv_1 >> 13 as ::core::ffi::c_int;
            _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_1);
            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
            _hj_j_2 ^= _hj_i_2 << 8 as ::core::ffi::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_2);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_2);
            _hf_hashv_1 ^= _hj_j_2 >> 13 as ::core::ffi::c_int;
            _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
            _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_1);
            _hj_i_2 ^= _hf_hashv_1 >> 12 as ::core::ffi::c_int;
            _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_1);
            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
            _hj_j_2 ^= _hj_i_2 << 16 as ::core::ffi::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_2);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_2);
            _hf_hashv_1 ^= _hj_j_2 >> 5 as ::core::ffi::c_int;
            _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
            _hj_i_2 = _hj_i_2.wrapping_sub(_hf_hashv_1);
            _hj_i_2 ^= _hf_hashv_1 >> 3 as ::core::ffi::c_int;
            _hj_j_2 = _hj_j_2.wrapping_sub(_hf_hashv_1);
            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
            _hj_j_2 ^= _hj_i_2 << 10 as ::core::ffi::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_2);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_2);
            _hf_hashv_1 ^= _hj_j_2 >> 15 as ::core::ffi::c_int;
            s_0 = ::core::ptr::null_mut::<feature_hash>();
            if !fh.is_null() {
                let mut _hf_bkt_1: ::core::ffi::c_uint = 0;
                _hf_bkt_1 = _hf_hashv_1
                    & (*(*fh).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(*fh).hh.tbl).buckets.offset(_hf_bkt_1 as isize))
                        .hh_head
                        .is_null()
                    {
                        s_0 = ((*(*(*fh).hh.tbl).buckets.offset(_hf_bkt_1 as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(*fh).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut feature_hash as *mut feature_hash;
                    } else {
                        s_0 = ::core::ptr::null_mut::<feature_hash>();
                    }
                    while !s_0.is_null() {
                        if (*s_0).hh.hashv == _hf_hashv_1
                            && (*s_0).hh.keylen == strlen(target) as ::core::ffi::c_uint
                        {
                            if memcmp(
                                (*s_0).hh.key,
                                target as *const ::core::ffi::c_void,
                                strlen(target) as ::core::ffi::c_uint as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*s_0).hh.hh_next.is_null() {
                            s_0 = ((*s_0).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(*fh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut feature_hash
                                as *mut feature_hash;
                        } else {
                            s_0 = ::core::ptr::null_mut::<feature_hash>();
                        }
                    }
                }
            }
            if !s_0.is_null() {
                let mut dup: *mut feature_hash = ::core::ptr::null_mut::<feature_hash>();
                dup = __caryll_allocate_clean(
                    ::core::mem::size_of::<feature_hash>() as usize,
                    220 as ::core::ffi::c_ulong,
                ) as *mut feature_hash;
                (*dup).alias = true;
                (*dup).name = sdsnew(featureName) as *mut ::core::ffi::c_char;
                (*dup).feature = (*s_0).feature;
                let mut _ha_hashv_0: ::core::ffi::c_uint = 0;
                let mut _hj_i_3: ::core::ffi::c_uint = 0;
                let mut _hj_j_3: ::core::ffi::c_uint = 0;
                let mut _hj_k_3: ::core::ffi::c_uint = 0;
                let mut _hj_key_3: *const ::core::ffi::c_uchar = (*dup)
                    .name
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_uchar;
                _ha_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_3 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_3 = _hj_j_3;
                _hj_k_3 = strlen((*dup).name) as ::core::ffi::c_uint;
                while _hj_k_3 >= 12 as ::core::ffi::c_uint {
                    _hj_i_3 = _hj_i_3.wrapping_add(
                        (*_hj_key_3.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_3.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_j_3 = _hj_j_3.wrapping_add(
                        (*_hj_key_3.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_3.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                        (*_hj_key_3.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_3.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(11 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                    _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_0);
                    _hj_i_3 ^= _ha_hashv_0 >> 13 as ::core::ffi::c_int;
                    _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_0);
                    _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                    _hj_j_3 ^= _hj_i_3 << 8 as ::core::ffi::c_int;
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_3);
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_3);
                    _ha_hashv_0 ^= _hj_j_3 >> 13 as ::core::ffi::c_int;
                    _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                    _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_0);
                    _hj_i_3 ^= _ha_hashv_0 >> 12 as ::core::ffi::c_int;
                    _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_0);
                    _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                    _hj_j_3 ^= _hj_i_3 << 16 as ::core::ffi::c_int;
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_3);
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_3);
                    _ha_hashv_0 ^= _hj_j_3 >> 5 as ::core::ffi::c_int;
                    _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                    _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_0);
                    _hj_i_3 ^= _ha_hashv_0 >> 3 as ::core::ffi::c_int;
                    _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_0);
                    _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                    _hj_j_3 ^= _hj_i_3 << 10 as ::core::ffi::c_int;
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_3);
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_3);
                    _ha_hashv_0 ^= _hj_j_3 >> 15 as ::core::ffi::c_int;
                    _hj_key_3 = _hj_key_3.offset(12 as ::core::ffi::c_int as isize);
                    _hj_k_3 = _hj_k_3.wrapping_sub(12 as ::core::ffi::c_uint);
                }
                _ha_hashv_0 = _ha_hashv_0.wrapping_add(strlen((*dup).name) as ::core::ffi::c_uint);
                let mut current_block_613: u64;
                match _hj_k_3 {
                    11 => {
                        _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                            (*_hj_key_3.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_613 = 6359568825798150134;
                    }
                    10 => {
                        current_block_613 = 6359568825798150134;
                    }
                    9 => {
                        current_block_613 = 12682153168616704965;
                    }
                    8 => {
                        current_block_613 = 14427564677393490829;
                    }
                    7 => {
                        current_block_613 = 10468295484844996031;
                    }
                    6 => {
                        current_block_613 = 5496815784377861184;
                    }
                    5 => {
                        current_block_613 = 17156371565015114949;
                    }
                    4 => {
                        current_block_613 = 17805899884133140060;
                    }
                    3 => {
                        current_block_613 = 5071037075315554844;
                    }
                    2 => {
                        current_block_613 = 16641742881583187241;
                    }
                    1 => {
                        current_block_613 = 17766610632212702295;
                    }
                    _ => {
                        current_block_613 = 6996835929439465836;
                    }
                }
                match current_block_613 {
                    6359568825798150134 => {
                        _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                            (*_hj_key_3.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_613 = 12682153168616704965;
                    }
                    _ => {}
                }
                match current_block_613 {
                    12682153168616704965 => {
                        _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                            (*_hj_key_3.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_613 = 14427564677393490829;
                    }
                    _ => {}
                }
                match current_block_613 {
                    14427564677393490829 => {
                        _hj_j_3 = _hj_j_3.wrapping_add(
                            (*_hj_key_3.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_613 = 10468295484844996031;
                    }
                    _ => {}
                }
                match current_block_613 {
                    10468295484844996031 => {
                        _hj_j_3 = _hj_j_3.wrapping_add(
                            (*_hj_key_3.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_613 = 5496815784377861184;
                    }
                    _ => {}
                }
                match current_block_613 {
                    5496815784377861184 => {
                        _hj_j_3 = _hj_j_3.wrapping_add(
                            (*_hj_key_3.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_613 = 17156371565015114949;
                    }
                    _ => {}
                }
                match current_block_613 {
                    17156371565015114949 => {
                        _hj_j_3 = _hj_j_3
                            .wrapping_add(*_hj_key_3.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_613 = 17805899884133140060;
                    }
                    _ => {}
                }
                match current_block_613 {
                    17805899884133140060 => {
                        _hj_i_3 = _hj_i_3.wrapping_add(
                            (*_hj_key_3.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_613 = 5071037075315554844;
                    }
                    _ => {}
                }
                match current_block_613 {
                    5071037075315554844 => {
                        _hj_i_3 = _hj_i_3.wrapping_add(
                            (*_hj_key_3.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_613 = 16641742881583187241;
                    }
                    _ => {}
                }
                match current_block_613 {
                    16641742881583187241 => {
                        _hj_i_3 = _hj_i_3.wrapping_add(
                            (*_hj_key_3.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_613 = 17766610632212702295;
                    }
                    _ => {}
                }
                match current_block_613 {
                    17766610632212702295 => {
                        _hj_i_3 = _hj_i_3
                            .wrapping_add(*_hj_key_3.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_0);
                _hj_i_3 ^= _ha_hashv_0 >> 13 as ::core::ffi::c_int;
                _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_0);
                _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                _hj_j_3 ^= _hj_i_3 << 8 as ::core::ffi::c_int;
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_3);
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_3);
                _ha_hashv_0 ^= _hj_j_3 >> 13 as ::core::ffi::c_int;
                _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_0);
                _hj_i_3 ^= _ha_hashv_0 >> 12 as ::core::ffi::c_int;
                _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_0);
                _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                _hj_j_3 ^= _hj_i_3 << 16 as ::core::ffi::c_int;
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_3);
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_3);
                _ha_hashv_0 ^= _hj_j_3 >> 5 as ::core::ffi::c_int;
                _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                _hj_i_3 = _hj_i_3.wrapping_sub(_ha_hashv_0);
                _hj_i_3 ^= _ha_hashv_0 >> 3 as ::core::ffi::c_int;
                _hj_j_3 = _hj_j_3.wrapping_sub(_ha_hashv_0);
                _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                _hj_j_3 ^= _hj_i_3 << 10 as ::core::ffi::c_int;
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_3);
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_3);
                _ha_hashv_0 ^= _hj_j_3 >> 15 as ::core::ffi::c_int;
                (*dup).hh.hashv = _ha_hashv_0;
                (*dup).hh.key = (*dup).name.offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void;
                (*dup).hh.keylen = strlen((*dup).name) as ::core::ffi::c_uint;
                if fh.is_null() {
                    (*dup).hh.next = NULL;
                    (*dup).hh.prev = NULL;
                    (*dup).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                        as *mut UT_hash_table
                        as *mut UT_hash_table;
                    if (*dup).hh.tbl.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*dup).hh.tbl as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            ::core::mem::size_of::<UT_hash_table>() as usize,
                        );
                        (*(*dup).hh.tbl).tail = &raw mut (*dup).hh as *mut UT_hash_handle;
                        (*(*dup).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                        (*(*dup).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                        (*(*dup).hh.tbl).hho = (&raw mut (*dup).hh as *mut ::core::ffi::c_char)
                            .offset_from(dup as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long
                            as isize;
                        (*(*dup).hh.tbl).buckets = malloc(
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        ) as *mut UT_hash_bucket;
                        (*(*dup).hh.tbl).signature = HASH_SIGNATURE as u32;
                        if (*(*dup).hh.tbl).buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*(*dup).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (32 as usize).wrapping_mul(
                                    ::core::mem::size_of::<UT_hash_bucket>() as usize,
                                ),
                            );
                        }
                    }
                    fh = dup;
                } else {
                    (*dup).hh.tbl = (*fh).hh.tbl;
                    (*dup).hh.next = NULL;
                    (*dup).hh.prev = ((*(*fh).hh.tbl).tail as *mut ::core::ffi::c_char)
                        .offset(-(*(*fh).hh.tbl).hho)
                        as *mut ::core::ffi::c_void;
                    (*(*(*fh).hh.tbl).tail).next = dup as *mut ::core::ffi::c_void;
                    (*(*fh).hh.tbl).tail = &raw mut (*dup).hh as *mut UT_hash_handle;
                }
                let mut _ha_bkt_0: ::core::ffi::c_uint = 0;
                (*(*fh).hh.tbl).num_items = (*(*fh).hh.tbl).num_items.wrapping_add(1);
                _ha_bkt_0 = _ha_hashv_0
                    & (*(*fh).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head_0: *mut UT_hash_bucket =
                    (*(*fh).hh.tbl).buckets.offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
                (*_ha_head_0).count = (*_ha_head_0).count.wrapping_add(1);
                (*dup).hh.hh_next = (*_ha_head_0).hh_head as *mut UT_hash_handle;
                (*dup).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                if !(*_ha_head_0).hh_head.is_null() {
                    (*(*_ha_head_0).hh_head).hh_prev = &raw mut (*dup).hh as *mut UT_hash_handle;
                }
                (*_ha_head_0).hh_head = &raw mut (*dup).hh as *mut UT_hash_handle;
                if (*_ha_head_0).count
                    >= (*_ha_head_0)
                        .expand_mult
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                    && (*(*dup).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt_0: ::core::ffi::c_uint = 0;
                    let mut _he_bkt_i_0: ::core::ffi::c_uint = 0;
                    let mut _he_thh_0: *mut UT_hash_handle =
                        ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _he_hh_nxt_0: *mut UT_hash_handle =
                        ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _he_new_buckets_0: *mut UT_hash_bucket =
                        ::core::ptr::null_mut::<UT_hash_bucket>();
                    let mut _he_newbkt_0: *mut UT_hash_bucket =
                        ::core::ptr::null_mut::<UT_hash_bucket>();
                    _he_new_buckets_0 = malloc(
                        (2 as usize)
                            .wrapping_mul((*(*dup).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    ) as *mut UT_hash_bucket;
                    if _he_new_buckets_0.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            _he_new_buckets_0 as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (2 as usize)
                                .wrapping_mul((*(*dup).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        );
                        (*(*dup).hh.tbl).ideal_chain_maxlen = ((*(*dup).hh.tbl).num_items
                            >> (*(*dup).hh.tbl)
                                .log2_num_buckets
                                .wrapping_add(1 as ::core::ffi::c_uint))
                        .wrapping_add(
                            if (*(*dup).hh.tbl).num_items
                                & (*(*dup).hh.tbl)
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
                        (*(*dup).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                        _he_bkt_i_0 = 0 as ::core::ffi::c_uint;
                        while _he_bkt_i_0 < (*(*dup).hh.tbl).num_buckets {
                            _he_thh_0 = (*(*(*dup).hh.tbl).buckets.offset(_he_bkt_i_0 as isize))
                                .hh_head
                                as *mut UT_hash_handle;
                            while !_he_thh_0.is_null() {
                                _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                                _he_bkt_0 = (*_he_thh_0).hashv
                                    & (*(*dup).hh.tbl)
                                        .num_buckets
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint);
                                _he_newbkt_0 = _he_new_buckets_0.offset(_he_bkt_0 as isize)
                                    as *mut UT_hash_bucket;
                                (*_he_newbkt_0).count = (*_he_newbkt_0).count.wrapping_add(1);
                                if (*_he_newbkt_0).count > (*(*dup).hh.tbl).ideal_chain_maxlen {
                                    (*(*dup).hh.tbl).nonideal_items =
                                        (*(*dup).hh.tbl).nonideal_items.wrapping_add(1);
                                    (*_he_newbkt_0).expand_mult = (*_he_newbkt_0)
                                        .count
                                        .wrapping_div((*(*dup).hh.tbl).ideal_chain_maxlen);
                                }
                                (*_he_thh_0).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                                (*_he_thh_0).hh_next =
                                    (*_he_newbkt_0).hh_head as *mut UT_hash_handle;
                                if !(*_he_newbkt_0).hh_head.is_null() {
                                    (*(*_he_newbkt_0).hh_head).hh_prev = _he_thh_0;
                                }
                                (*_he_newbkt_0).hh_head = _he_thh_0 as *mut UT_hash_handle;
                                _he_thh_0 = _he_hh_nxt_0;
                            }
                            _he_bkt_i_0 = _he_bkt_i_0.wrapping_add(1);
                        }
                        free((*(*dup).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        (*(*dup).hh.tbl).num_buckets = (*(*dup).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint);
                        (*(*dup).hh.tbl).log2_num_buckets =
                            (*(*dup).hh.tbl).log2_num_buckets.wrapping_add(1);
                        (*(*dup).hh.tbl).buckets = _he_new_buckets_0;
                        (*(*dup).hh.tbl).ineff_expands = if (*(*dup).hh.tbl).nonideal_items
                            > (*(*dup).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                        {
                            (*(*dup).hh.tbl)
                                .ineff_expands
                                .wrapping_add(1 as ::core::ffi::c_uint)
                        } else {
                            0 as ::core::ffi::c_uint
                        };
                        if (*(*dup).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                            (*(*dup).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                        }
                    }
                }
            }
        }
        j = j.wrapping_add(1);
    }
    return fh;
}
#[no_mangle]
pub unsafe extern "C" fn isValidLanguageName(
    mut name: *const ::core::ffi::c_char,
    length: usize,
) -> bool {
    return length == 9 as usize
        && *name.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == SCRIPT_LANGUAGE_SEPARATOR as ::core::ffi::c_int;
}
unsafe extern "C" fn figureOutLanguagesFromJson(
    mut languages: *mut json_value,
    mut fh: *mut feature_hash,
    mut tag: *const ::core::ffi::c_char,
    mut options: *const otfcc_Options,
) -> *mut language_hash {
    let mut sh: *mut language_hash = ::core::ptr::null_mut::<language_hash>();
    let mut j: u32 = 0 as u32;
    while j < (*languages).u.object.length as u32 {
        let mut languageName: *mut ::core::ffi::c_char =
            (*(*languages).u.object.values.offset(j as isize)).name;
        let mut languageNameLen: usize =
            (*(*languages).u.object.values.offset(j as isize)).name_length as usize;
        let mut _language: *mut json_value =
            (*(*languages).u.object.values.offset(j as isize)).value as *mut json_value;
        if isValidLanguageName(languageName, languageNameLen) as ::core::ffi::c_int != 0
            && (*_language).type_0 as ::core::ffi::c_uint
                == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut requiredFeature: *mut otl_Feature = ::core::ptr::null_mut::<otl_Feature>();
            let mut _rf: *mut json_value = json_obj_get_type(
                _language,
                b"requiredFeature\0" as *const u8 as *const ::core::ffi::c_char,
                json_string,
            );
            if !_rf.is_null() {
                let mut rf: *mut feature_hash = ::core::ptr::null_mut::<feature_hash>();
                let mut _hf_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i: ::core::ffi::c_uint = 0;
                let mut _hj_j: ::core::ffi::c_uint = 0;
                let mut _hj_k: ::core::ffi::c_uint = 0;
                let mut _hj_key: *const ::core::ffi::c_uchar =
                    (*_rf).u.string.ptr as *const ::core::ffi::c_uchar;
                _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i = _hj_j;
                _hj_k = strlen((*_rf).u.string.ptr) as ::core::ffi::c_uint;
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
                _hf_hashv =
                    _hf_hashv.wrapping_add(strlen((*_rf).u.string.ptr) as ::core::ffi::c_uint);
                let mut current_block_50: u64;
                match _hj_k {
                    11 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_50 = 8056918467624983594;
                    }
                    10 => {
                        current_block_50 = 8056918467624983594;
                    }
                    9 => {
                        current_block_50 = 14700161889417409860;
                    }
                    8 => {
                        current_block_50 = 10497365183480084116;
                    }
                    7 => {
                        current_block_50 = 18042852036382472658;
                    }
                    6 => {
                        current_block_50 = 550504990770054440;
                    }
                    5 => {
                        current_block_50 = 8837553621540314112;
                    }
                    4 => {
                        current_block_50 = 6773666287664831423;
                    }
                    3 => {
                        current_block_50 = 17404935890793427620;
                    }
                    2 => {
                        current_block_50 = 12280580428087688836;
                    }
                    1 => {
                        current_block_50 = 2905048763656904430;
                    }
                    _ => {
                        current_block_50 = 3160140712158701372;
                    }
                }
                match current_block_50 {
                    8056918467624983594 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_50 = 14700161889417409860;
                    }
                    _ => {}
                }
                match current_block_50 {
                    14700161889417409860 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_50 = 10497365183480084116;
                    }
                    _ => {}
                }
                match current_block_50 {
                    10497365183480084116 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_50 = 18042852036382472658;
                    }
                    _ => {}
                }
                match current_block_50 {
                    18042852036382472658 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_50 = 550504990770054440;
                    }
                    _ => {}
                }
                match current_block_50 {
                    550504990770054440 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_50 = 8837553621540314112;
                    }
                    _ => {}
                }
                match current_block_50 {
                    8837553621540314112 => {
                        _hj_j = _hj_j
                            .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_50 = 6773666287664831423;
                    }
                    _ => {}
                }
                match current_block_50 {
                    6773666287664831423 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_50 = 17404935890793427620;
                    }
                    _ => {}
                }
                match current_block_50 {
                    17404935890793427620 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_50 = 12280580428087688836;
                    }
                    _ => {}
                }
                match current_block_50 {
                    12280580428087688836 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_50 = 2905048763656904430;
                    }
                    _ => {}
                }
                match current_block_50 {
                    2905048763656904430 => {
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
                rf = ::core::ptr::null_mut::<feature_hash>();
                if !fh.is_null() {
                    let mut _hf_bkt: ::core::ffi::c_uint = 0;
                    _hf_bkt = _hf_hashv
                        & (*(*fh).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if !(*(*(*fh).hh.tbl).buckets.offset(_hf_bkt as isize))
                            .hh_head
                            .is_null()
                        {
                            rf = ((*(*(*fh).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(*fh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut feature_hash
                                as *mut feature_hash;
                        } else {
                            rf = ::core::ptr::null_mut::<feature_hash>();
                        }
                        while !rf.is_null() {
                            if (*rf).hh.hashv == _hf_hashv
                                && (*rf).hh.keylen
                                    == strlen((*_rf).u.string.ptr) as ::core::ffi::c_uint
                            {
                                if memcmp(
                                    (*rf).hh.key,
                                    (*_rf).u.string.ptr as *const ::core::ffi::c_void,
                                    strlen((*_rf).u.string.ptr) as ::core::ffi::c_uint as usize,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if !(*rf).hh.hh_next.is_null() {
                                rf = ((*rf).hh.hh_next as *mut ::core::ffi::c_char)
                                    .offset(-(*(*fh).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut feature_hash
                                    as *mut feature_hash;
                            } else {
                                rf = ::core::ptr::null_mut::<feature_hash>();
                            }
                        }
                    }
                }
                if !rf.is_null() {
                    requiredFeature = (*rf).feature;
                }
            }
            let mut af: otl_FeatureRefList = otl_FeatureRefList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<otl_FeatureRef>(),
            };
            otl_iFeatureRefList.init.expect("non-null function pointer")(&raw mut af);
            let mut _features: *mut json_value = json_obj_get_type(
                _language,
                b"features\0" as *const u8 as *const ::core::ffi::c_char,
                json_array,
            );
            if !_features.is_null() {
                let mut k: tableid_t = 0 as tableid_t;
                while (k as ::core::ffi::c_uint) < (*_features).u.array.length {
                    let mut term: *mut json_value =
                        *(*_features).u.array.values.offset(k as isize) as *mut json_value;
                    if (*term).type_0 as ::core::ffi::c_uint
                        == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let mut item: *mut feature_hash = ::core::ptr::null_mut::<feature_hash>();
                        let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
                        let mut _hj_i_0: ::core::ffi::c_uint = 0;
                        let mut _hj_j_0: ::core::ffi::c_uint = 0;
                        let mut _hj_k_0: ::core::ffi::c_uint = 0;
                        let mut _hj_key_0: *const ::core::ffi::c_uchar =
                            (*term).u.string.ptr as *const ::core::ffi::c_uchar;
                        _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
                        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                        _hj_i_0 = _hj_j_0;
                        _hj_k_0 = strlen((*term).u.string.ptr) as ::core::ffi::c_uint;
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
                            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
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
                            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                            _hj_i_0 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
                            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                            _hf_hashv_0 ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
                            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                            _hj_i_0 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
                            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                            _hf_hashv_0 ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
                            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                            _hj_i_0 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
                            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                            _hf_hashv_0 ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
                            _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
                            _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
                        }
                        _hf_hashv_0 = _hf_hashv_0
                            .wrapping_add(strlen((*term).u.string.ptr) as ::core::ffi::c_uint);
                        let mut current_block_170: u64;
                        match _hj_k_0 {
                            11 => {
                                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_170 = 9414534572567306460;
                            }
                            10 => {
                                current_block_170 = 9414534572567306460;
                            }
                            9 => {
                                current_block_170 = 9401706649748148268;
                            }
                            8 => {
                                current_block_170 = 15106656871850413285;
                            }
                            7 => {
                                current_block_170 = 6997942234028788220;
                            }
                            6 => {
                                current_block_170 = 15969188635170351704;
                            }
                            5 => {
                                current_block_170 = 5471895832349958498;
                            }
                            4 => {
                                current_block_170 = 4021922109173273134;
                            }
                            3 => {
                                current_block_170 = 5149161175239843346;
                            }
                            2 => {
                                current_block_170 = 5352833821250328059;
                            }
                            1 => {
                                current_block_170 = 9406273681328019846;
                            }
                            _ => {
                                current_block_170 = 4871270227279186910;
                            }
                        }
                        match current_block_170 {
                            9414534572567306460 => {
                                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_170 = 9401706649748148268;
                            }
                            _ => {}
                        }
                        match current_block_170 {
                            9401706649748148268 => {
                                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_170 = 15106656871850413285;
                            }
                            _ => {}
                        }
                        match current_block_170 {
                            15106656871850413285 => {
                                _hj_j_0 = _hj_j_0.wrapping_add(
                                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_170 = 6997942234028788220;
                            }
                            _ => {}
                        }
                        match current_block_170 {
                            6997942234028788220 => {
                                _hj_j_0 = _hj_j_0.wrapping_add(
                                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_170 = 15969188635170351704;
                            }
                            _ => {}
                        }
                        match current_block_170 {
                            15969188635170351704 => {
                                _hj_j_0 = _hj_j_0.wrapping_add(
                                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_170 = 5471895832349958498;
                            }
                            _ => {}
                        }
                        match current_block_170 {
                            5471895832349958498 => {
                                _hj_j_0 = _hj_j_0.wrapping_add(
                                    *_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint,
                                );
                                current_block_170 = 4021922109173273134;
                            }
                            _ => {}
                        }
                        match current_block_170 {
                            4021922109173273134 => {
                                _hj_i_0 = _hj_i_0.wrapping_add(
                                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_170 = 5149161175239843346;
                            }
                            _ => {}
                        }
                        match current_block_170 {
                            5149161175239843346 => {
                                _hj_i_0 = _hj_i_0.wrapping_add(
                                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_170 = 5352833821250328059;
                            }
                            _ => {}
                        }
                        match current_block_170 {
                            5352833821250328059 => {
                                _hj_i_0 = _hj_i_0.wrapping_add(
                                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_170 = 9406273681328019846;
                            }
                            _ => {}
                        }
                        match current_block_170 {
                            9406273681328019846 => {
                                _hj_i_0 = _hj_i_0.wrapping_add(
                                    *_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint,
                                );
                            }
                            _ => {}
                        }
                        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                        _hj_i_0 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
                        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                        _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                        _hf_hashv_0 ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
                        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                        _hj_i_0 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
                        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                        _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                        _hf_hashv_0 ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
                        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                        _hj_i_0 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
                        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                        _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                        _hf_hashv_0 ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
                        item = ::core::ptr::null_mut::<feature_hash>();
                        if !fh.is_null() {
                            let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
                            _hf_bkt_0 = _hf_hashv_0
                                & (*(*fh).hh.tbl)
                                    .num_buckets
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                if !(*(*(*fh).hh.tbl).buckets.offset(_hf_bkt_0 as isize))
                                    .hh_head
                                    .is_null()
                                {
                                    item = ((*(*(*fh).hh.tbl).buckets.offset(_hf_bkt_0 as isize))
                                        .hh_head
                                        as *mut ::core::ffi::c_char)
                                        .offset(-(*(*fh).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut feature_hash
                                        as *mut feature_hash;
                                } else {
                                    item = ::core::ptr::null_mut::<feature_hash>();
                                }
                                while !item.is_null() {
                                    if (*item).hh.hashv == _hf_hashv_0
                                        && (*item).hh.keylen
                                            == strlen((*term).u.string.ptr) as ::core::ffi::c_uint
                                    {
                                        if memcmp(
                                            (*item).hh.key,
                                            (*term).u.string.ptr as *const ::core::ffi::c_void,
                                            strlen((*term).u.string.ptr) as ::core::ffi::c_uint
                                                as usize,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            break;
                                        }
                                    }
                                    if !(*item).hh.hh_next.is_null() {
                                        item = ((*item).hh.hh_next as *mut ::core::ffi::c_char)
                                            .offset(-(*(*fh).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                            as *mut feature_hash
                                            as *mut feature_hash;
                                    } else {
                                        item = ::core::ptr::null_mut::<feature_hash>();
                                    }
                                }
                            }
                        }
                        if !item.is_null() {
                            otl_iFeatureRefList.push.expect("non-null function pointer")(
                                &raw mut af,
                                (*item).feature as otl_FeatureRef,
                            );
                        }
                    }
                    k = k.wrapping_add(1);
                }
            }
            if !requiredFeature.is_null() || af.length > 0 as usize {
                let mut s: *mut language_hash = ::core::ptr::null_mut::<language_hash>();
                let mut _hf_hashv_1: ::core::ffi::c_uint = 0;
                let mut _hj_i_1: ::core::ffi::c_uint = 0;
                let mut _hj_j_1: ::core::ffi::c_uint = 0;
                let mut _hj_k_1: ::core::ffi::c_uint = 0;
                let mut _hj_key_1: *const ::core::ffi::c_uchar =
                    languageName as *const ::core::ffi::c_uchar;
                _hf_hashv_1 = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_1 = _hj_j_1;
                _hj_k_1 = strlen(languageName) as ::core::ffi::c_uint;
                while _hj_k_1 >= 12 as ::core::ffi::c_uint {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                        (*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                    _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                        (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                    _hj_i_1 ^= _hf_hashv_1 >> 13 as ::core::ffi::c_int;
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                    _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                    _hf_hashv_1 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                    _hj_i_1 ^= _hf_hashv_1 >> 12 as ::core::ffi::c_int;
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                    _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                    _hf_hashv_1 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                    _hj_i_1 ^= _hf_hashv_1 >> 3 as ::core::ffi::c_int;
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                    _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                    _hf_hashv_1 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
                    _hj_key_1 = _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
                    _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
                }
                _hf_hashv_1 = _hf_hashv_1.wrapping_add(strlen(languageName) as ::core::ffi::c_uint);
                let mut current_block_293: u64;
                match _hj_k_1 {
                    11 => {
                        _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                            (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_293 = 9731351165472125272;
                    }
                    10 => {
                        current_block_293 = 9731351165472125272;
                    }
                    9 => {
                        current_block_293 = 18074274781206676365;
                    }
                    8 => {
                        current_block_293 = 2703713326810445295;
                    }
                    7 => {
                        current_block_293 = 2287669382481938074;
                    }
                    6 => {
                        current_block_293 = 53785787823454161;
                    }
                    5 => {
                        current_block_293 = 6602686309234237379;
                    }
                    4 => {
                        current_block_293 = 5710910050460311707;
                    }
                    3 => {
                        current_block_293 = 7949765165458945995;
                    }
                    2 => {
                        current_block_293 = 16157078313657245745;
                    }
                    1 => {
                        current_block_293 = 18435508912408360646;
                    }
                    _ => {
                        current_block_293 = 2627007089909013891;
                    }
                }
                match current_block_293 {
                    9731351165472125272 => {
                        _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                            (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_293 = 18074274781206676365;
                    }
                    _ => {}
                }
                match current_block_293 {
                    18074274781206676365 => {
                        _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                            (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_293 = 2703713326810445295;
                    }
                    _ => {}
                }
                match current_block_293 {
                    2703713326810445295 => {
                        _hj_j_1 = _hj_j_1.wrapping_add(
                            (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_293 = 2287669382481938074;
                    }
                    _ => {}
                }
                match current_block_293 {
                    2287669382481938074 => {
                        _hj_j_1 = _hj_j_1.wrapping_add(
                            (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_293 = 53785787823454161;
                    }
                    _ => {}
                }
                match current_block_293 {
                    53785787823454161 => {
                        _hj_j_1 = _hj_j_1.wrapping_add(
                            (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_293 = 6602686309234237379;
                    }
                    _ => {}
                }
                match current_block_293 {
                    6602686309234237379 => {
                        _hj_j_1 = _hj_j_1
                            .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_293 = 5710910050460311707;
                    }
                    _ => {}
                }
                match current_block_293 {
                    5710910050460311707 => {
                        _hj_i_1 = _hj_i_1.wrapping_add(
                            (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_293 = 7949765165458945995;
                    }
                    _ => {}
                }
                match current_block_293 {
                    7949765165458945995 => {
                        _hj_i_1 = _hj_i_1.wrapping_add(
                            (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_293 = 16157078313657245745;
                    }
                    _ => {}
                }
                match current_block_293 {
                    16157078313657245745 => {
                        _hj_i_1 = _hj_i_1.wrapping_add(
                            (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_293 = 18435508912408360646;
                    }
                    _ => {}
                }
                match current_block_293 {
                    18435508912408360646 => {
                        _hj_i_1 = _hj_i_1
                            .wrapping_add(*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                _hj_i_1 ^= _hf_hashv_1 >> 13 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                _hf_hashv_1 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                _hj_i_1 ^= _hf_hashv_1 >> 12 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                _hf_hashv_1 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                _hj_i_1 ^= _hf_hashv_1 >> 3 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                _hf_hashv_1 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
                s = ::core::ptr::null_mut::<language_hash>();
                if !sh.is_null() {
                    let mut _hf_bkt_1: ::core::ffi::c_uint = 0;
                    _hf_bkt_1 = _hf_hashv_1
                        & (*(*sh).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if !(*(*(*sh).hh.tbl).buckets.offset(_hf_bkt_1 as isize))
                            .hh_head
                            .is_null()
                        {
                            s = ((*(*(*sh).hh.tbl).buckets.offset(_hf_bkt_1 as isize)).hh_head
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(*sh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut language_hash
                                as *mut language_hash;
                        } else {
                            s = ::core::ptr::null_mut::<language_hash>();
                        }
                        while !s.is_null() {
                            if (*s).hh.hashv == _hf_hashv_1
                                && (*s).hh.keylen == strlen(languageName) as ::core::ffi::c_uint
                            {
                                if memcmp(
                                    (*s).hh.key,
                                    languageName as *const ::core::ffi::c_void,
                                    strlen(languageName) as ::core::ffi::c_uint as usize,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if !(*s).hh.hh_next.is_null() {
                                s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                    .offset(-(*(*sh).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut language_hash
                                    as *mut language_hash;
                            } else {
                                s = ::core::ptr::null_mut::<language_hash>();
                            }
                        }
                    }
                }
                if s.is_null() {
                    s = __caryll_allocate_clean(
                        ::core::mem::size_of::<language_hash>() as usize,
                        267 as ::core::ffi::c_ulong,
                    ) as *mut language_hash;
                    (*s).name = sdsnew(languageName) as *mut ::core::ffi::c_char;
                    otl_iLanguageSystem.init.expect("non-null function pointer")(
                        &raw mut (*s).language,
                    );
                    (*(*s).language).name = sdsdup((*s).name as sds);
                    (*(*s).language).requiredFeature = requiredFeature as otl_FeatureRef;
                    otl_iFeatureRefList
                        .replace
                        .expect("non-null function pointer")(
                        &raw mut (*(*s).language).features,
                        af,
                    );
                    let mut _ha_hashv: ::core::ffi::c_uint = 0;
                    let mut _hj_i_2: ::core::ffi::c_uint = 0;
                    let mut _hj_j_2: ::core::ffi::c_uint = 0;
                    let mut _hj_k_2: ::core::ffi::c_uint = 0;
                    let mut _hj_key_2: *const ::core::ffi::c_uchar =
                        (*s).name.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_uchar;
                    _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                    _hj_j_2 = 0x9e3779b9 as ::core::ffi::c_uint;
                    _hj_i_2 = _hj_j_2;
                    _hj_k_2 = strlen((*s).name) as ::core::ffi::c_uint;
                    while _hj_k_2 >= 12 as ::core::ffi::c_uint {
                        _hj_i_2 = _hj_i_2.wrapping_add(
                            (*_hj_key_2.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                ),
                        );
                        _hj_j_2 = _hj_j_2.wrapping_add(
                            (*_hj_key_2.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                ),
                        );
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(11 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                ),
                        );
                        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                        _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                        _hj_i_2 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                        _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                        _hj_j_2 ^= _hj_i_2 << 8 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                        _ha_hashv ^= _hj_j_2 >> 13 as ::core::ffi::c_int;
                        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                        _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                        _hj_i_2 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                        _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                        _hj_j_2 ^= _hj_i_2 << 16 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                        _ha_hashv ^= _hj_j_2 >> 5 as ::core::ffi::c_int;
                        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                        _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                        _hj_i_2 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                        _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                        _hj_j_2 ^= _hj_i_2 << 10 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                        _ha_hashv ^= _hj_j_2 >> 15 as ::core::ffi::c_int;
                        _hj_key_2 = _hj_key_2.offset(12 as ::core::ffi::c_int as isize);
                        _hj_k_2 = _hj_k_2.wrapping_sub(12 as ::core::ffi::c_uint);
                    }
                    _ha_hashv = _ha_hashv.wrapping_add(strlen((*s).name) as ::core::ffi::c_uint);
                    let mut current_block_413: u64;
                    match _hj_k_2 {
                        11 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_413 = 6879240018607126872;
                        }
                        10 => {
                            current_block_413 = 6879240018607126872;
                        }
                        9 => {
                            current_block_413 = 14288784596357532072;
                        }
                        8 => {
                            current_block_413 = 12439072576376712565;
                        }
                        7 => {
                            current_block_413 = 7537225836018668986;
                        }
                        6 => {
                            current_block_413 = 16524316032370654208;
                        }
                        5 => {
                            current_block_413 = 15902484794574381277;
                        }
                        4 => {
                            current_block_413 = 10414648127620005808;
                        }
                        3 => {
                            current_block_413 = 15856974427001039731;
                        }
                        2 => {
                            current_block_413 = 13794339084290703930;
                        }
                        1 => {
                            current_block_413 = 6622003589927843354;
                        }
                        _ => {
                            current_block_413 = 14239264278009762102;
                        }
                    }
                    match current_block_413 {
                        6879240018607126872 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_413 = 14288784596357532072;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        14288784596357532072 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_413 = 12439072576376712565;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        12439072576376712565 => {
                            _hj_j_2 = _hj_j_2.wrapping_add(
                                (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_413 = 7537225836018668986;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        7537225836018668986 => {
                            _hj_j_2 = _hj_j_2.wrapping_add(
                                (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_413 = 16524316032370654208;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        16524316032370654208 => {
                            _hj_j_2 = _hj_j_2.wrapping_add(
                                (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_413 = 15902484794574381277;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        15902484794574381277 => {
                            _hj_j_2 = _hj_j_2
                                .wrapping_add(*_hj_key_2.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_413 = 10414648127620005808;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        10414648127620005808 => {
                            _hj_i_2 = _hj_i_2.wrapping_add(
                                (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_413 = 15856974427001039731;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        15856974427001039731 => {
                            _hj_i_2 = _hj_i_2.wrapping_add(
                                (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_413 = 13794339084290703930;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        13794339084290703930 => {
                            _hj_i_2 = _hj_i_2.wrapping_add(
                                (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_413 = 6622003589927843354;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        6622003589927843354 => {
                            _hj_i_2 = _hj_i_2
                                .wrapping_add(*_hj_key_2.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                        }
                        _ => {}
                    }
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                    _hj_i_2 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 8 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                    _ha_hashv ^= _hj_j_2 >> 13 as ::core::ffi::c_int;
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                    _hj_i_2 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 16 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                    _ha_hashv ^= _hj_j_2 >> 5 as ::core::ffi::c_int;
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                    _hj_i_2 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 10 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                    _ha_hashv ^= _hj_j_2 >> 15 as ::core::ffi::c_int;
                    (*s).hh.hashv = _ha_hashv;
                    (*s).hh.key = (*s).name.offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void;
                    (*s).hh.keylen = strlen((*s).name) as ::core::ffi::c_uint;
                    if sh.is_null() {
                        (*s).hh.next = NULL;
                        (*s).hh.prev = NULL;
                        (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                            as *mut UT_hash_table
                            as *mut UT_hash_table;
                        if (*s).hh.tbl.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*s).hh.tbl as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                ::core::mem::size_of::<UT_hash_table>() as usize,
                            );
                            (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                            (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                            (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                            (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                                .offset_from(s as *mut ::core::ffi::c_char)
                                as ::core::ffi::c_long
                                as isize;
                            (*(*s).hh.tbl).buckets =
                                malloc((32 as usize).wrapping_mul(::core::mem::size_of::<
                                    UT_hash_bucket,
                                >(
                                )
                                    as usize))
                                    as *mut UT_hash_bucket;
                            (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                            if (*(*s).hh.tbl).buckets.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (32 as usize).wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as usize,
                                    ),
                                );
                            }
                        }
                        sh = s;
                    } else {
                        (*s).hh.tbl = (*sh).hh.tbl;
                        (*s).hh.next = NULL;
                        (*s).hh.prev = ((*(*sh).hh.tbl).tail as *mut ::core::ffi::c_char)
                            .offset(-(*(*sh).hh.tbl).hho)
                            as *mut ::core::ffi::c_void;
                        (*(*(*sh).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                        (*(*sh).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                    }
                    let mut _ha_bkt: ::core::ffi::c_uint = 0;
                    (*(*sh).hh.tbl).num_items = (*(*sh).hh.tbl).num_items.wrapping_add(1);
                    _ha_bkt = _ha_hashv
                        & (*(*sh).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _ha_head: *mut UT_hash_bucket =
                        (*(*sh).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
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
                            (2 as usize)
                                .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        ) as *mut UT_hash_bucket;
                        if _he_new_buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                _he_new_buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (2 as usize)
                                    .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as usize
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
                    (*(*options).logger)
                        .logSDS
                        .expect(
                            "non-null function pointer",
                        )(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        sdscatprintf(
                            sdsempty(),
                            b"[OTFCC-fea] Duplicate language item [%s/%s]. This language term will be ignored.\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            tag,
                            languageName,
                        ),
                    );
                    otl_iFeatureRefList
                        .dispose
                        .expect("non-null function pointer")(&raw mut af);
                }
            } else {
                (*(*options).logger)
                    .logSDS
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[OTFCC-fea] There is no valid feature assignments for [%s/%s]. This language term will be ignored.\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        tag,
                        languageName,
                    ),
                );
                otl_iFeatureRefList
                    .dispose
                    .expect("non-null function pointer")(&raw mut af);
            }
        }
        j = j.wrapping_add(1);
    }
    return sh;
}
unsafe extern "C" fn by_lookup_order(
    mut a: *mut lookup_hash,
    mut b: *mut lookup_hash,
) -> ::core::ffi::c_int {
    if (*a).orderType as ::core::ffi::c_uint == (*b).orderType as ::core::ffi::c_uint {
        return (*a).orderVal as ::core::ffi::c_int - (*b).orderVal as ::core::ffi::c_int;
    } else {
        return ((*a).orderType as ::core::ffi::c_uint)
            .wrapping_sub((*b).orderType as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn by_feature_name(
    mut a: *mut feature_hash,
    mut b: *mut feature_hash,
) -> ::core::ffi::c_int {
    return strcmp((*a).name, (*b).name);
}
unsafe extern "C" fn by_language_name(
    mut a: *mut language_hash,
    mut b: *mut language_hash,
) -> ::core::ffi::c_int {
    return strcmp((*a).name, (*b).name);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseOtl(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut table_OTL {
    let mut languages: *mut json_value = ::core::ptr::null_mut::<json_value>();
    let mut features: *mut json_value = ::core::ptr::null_mut::<json_value>();
    let mut lookups: *mut json_value = ::core::ptr::null_mut::<json_value>();
    let mut current_block: u64;
    let mut otl: *mut table_OTL = ::core::ptr::null_mut::<table_OTL>();
    let mut table: *mut json_value = json_obj_get_type(root, tag, json_object);
    if !table.is_null() {
        otl = (
            table_iOTL.create.expect("non-null function pointer"))();
        languages = json_obj_get_type(
            table,
            b"languages\0" as *const u8 as *const ::core::ffi::c_char,
            json_object,
        );
        features = json_obj_get_type(
            table,
            b"features\0" as *const u8 as *const ::core::ffi::c_char,
            json_object,
        );
        lookups = json_obj_get_type(
            table,
            b"lookups\0" as *const u8 as *const ::core::ffi::c_char,
            json_object,
        );
        if !(languages.is_null() || features.is_null() || lookups.is_null()) {
            (*(*options).logger)
                .startSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                sdscatprintf(
                    sdsempty(),
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    tag,
                ),
            );
            let mut ___loggedstep_v: bool = true;
            loop {
                if !___loggedstep_v {
                    current_block = 5279571973604048562;
                    break;
                }
                let mut lh: *mut lookup_hash = figureOutLookupsFromJSON(lookups, options);
                let mut lookupOrder: *mut json_value = json_obj_get_type(
                    table,
                    b"lookupOrder\0" as *const u8 as *const ::core::ffi::c_char,
                    json_array,
                );
                if !lookupOrder.is_null() {
                    let mut j: tableid_t = 0 as tableid_t;
                    while (j as ::core::ffi::c_uint) < (*lookupOrder).u.array.length {
                        let mut _ln: *mut json_value =
                            *(*lookupOrder).u.array.values.offset(j as isize) as *mut json_value;
                        if !_ln.is_null()
                            && (*_ln).type_0 as ::core::ffi::c_uint
                                == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            let mut item: *mut lookup_hash = ::core::ptr::null_mut::<lookup_hash>();
                            let mut _hf_hashv: ::core::ffi::c_uint = 0;
                            let mut _hj_i: ::core::ffi::c_uint = 0;
                            let mut _hj_j: ::core::ffi::c_uint = 0;
                            let mut _hj_k: ::core::ffi::c_uint = 0;
                            let mut _hj_key: *const ::core::ffi::c_uchar =
                                (*_ln).u.string.ptr as *const ::core::ffi::c_uchar;
                            _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                            _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
                            _hj_i = _hj_j;
                            _hj_k = strlen((*_ln).u.string.ptr) as ::core::ffi::c_uint;
                            while _hj_k >= 12 as ::core::ffi::c_uint {
                                _hj_i = _hj_i.wrapping_add(
                                    (*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
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
                                    (*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
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
                                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
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
                            _hf_hashv = _hf_hashv
                                .wrapping_add(strlen((*_ln).u.string.ptr) as ::core::ffi::c_uint);
                            let mut current_block_51: u64;
                            match _hj_k {
                                11 => {
                                    _hf_hashv = _hf_hashv.wrapping_add(
                                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int,
                                    );
                                    current_block_51 = 7087681849025310823;
                                }
                                10 => {
                                    current_block_51 = 7087681849025310823;
                                }
                                9 => {
                                    current_block_51 = 10169226301896485670;
                                }
                                8 => {
                                    current_block_51 = 1542554806440917911;
                                }
                                7 => {
                                    current_block_51 = 17352596825455804027;
                                }
                                6 => {
                                    current_block_51 = 540408258068100013;
                                }
                                5 => {
                                    current_block_51 = 5449811471494522887;
                                }
                                4 => {
                                    current_block_51 = 15238598597864853978;
                                }
                                3 => {
                                    current_block_51 = 4158391151820389001;
                                }
                                2 => {
                                    current_block_51 = 14688179294451066510;
                                }
                                1 => {
                                    current_block_51 = 13670783939593524165;
                                }
                                _ => {
                                    current_block_51 = 11777552016271000781;
                                }
                            }
                            match current_block_51 {
                                7087681849025310823 => {
                                    _hf_hashv = _hf_hashv.wrapping_add(
                                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 16 as ::core::ffi::c_int,
                                    );
                                    current_block_51 = 10169226301896485670;
                                }
                                _ => {}
                            }
                            match current_block_51 {
                                10169226301896485670 => {
                                    _hf_hashv = _hf_hashv.wrapping_add(
                                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 8 as ::core::ffi::c_int,
                                    );
                                    current_block_51 = 1542554806440917911;
                                }
                                _ => {}
                            }
                            match current_block_51 {
                                1542554806440917911 => {
                                    _hj_j = _hj_j.wrapping_add(
                                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int,
                                    );
                                    current_block_51 = 17352596825455804027;
                                }
                                _ => {}
                            }
                            match current_block_51 {
                                17352596825455804027 => {
                                    _hj_j = _hj_j.wrapping_add(
                                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 16 as ::core::ffi::c_int,
                                    );
                                    current_block_51 = 540408258068100013;
                                }
                                _ => {}
                            }
                            match current_block_51 {
                                540408258068100013 => {
                                    _hj_j = _hj_j.wrapping_add(
                                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 8 as ::core::ffi::c_int,
                                    );
                                    current_block_51 = 5449811471494522887;
                                }
                                _ => {}
                            }
                            match current_block_51 {
                                5449811471494522887 => {
                                    _hj_j = _hj_j.wrapping_add(
                                        *_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint,
                                    );
                                    current_block_51 = 15238598597864853978;
                                }
                                _ => {}
                            }
                            match current_block_51 {
                                15238598597864853978 => {
                                    _hj_i = _hj_i.wrapping_add(
                                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int,
                                    );
                                    current_block_51 = 4158391151820389001;
                                }
                                _ => {}
                            }
                            match current_block_51 {
                                4158391151820389001 => {
                                    _hj_i = _hj_i.wrapping_add(
                                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 16 as ::core::ffi::c_int,
                                    );
                                    current_block_51 = 14688179294451066510;
                                }
                                _ => {}
                            }
                            match current_block_51 {
                                14688179294451066510 => {
                                    _hj_i = _hj_i.wrapping_add(
                                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint)
                                            << 8 as ::core::ffi::c_int,
                                    );
                                    current_block_51 = 13670783939593524165;
                                }
                                _ => {}
                            }
                            match current_block_51 {
                                13670783939593524165 => {
                                    _hj_i = _hj_i.wrapping_add(
                                        *_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uint,
                                    );
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
                            item = ::core::ptr::null_mut::<lookup_hash>();
                            if !lh.is_null() {
                                let mut _hf_bkt: ::core::ffi::c_uint = 0;
                                _hf_bkt = _hf_hashv
                                    & (*(*lh).hh.tbl)
                                        .num_buckets
                                        .wrapping_sub(1 as ::core::ffi::c_uint);
                                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                    if !(*(*(*lh).hh.tbl).buckets.offset(_hf_bkt as isize))
                                        .hh_head
                                        .is_null()
                                    {
                                        item = ((*(*(*lh).hh.tbl).buckets.offset(_hf_bkt as isize))
                                            .hh_head
                                            as *mut ::core::ffi::c_char)
                                            .offset(-(*(*lh).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                            as *mut lookup_hash
                                            as *mut lookup_hash;
                                    } else {
                                        item = ::core::ptr::null_mut::<lookup_hash>();
                                    }
                                    while !item.is_null() {
                                        if (*item).hh.hashv == _hf_hashv
                                            && (*item).hh.keylen
                                                == strlen((*_ln).u.string.ptr)
                                                    as ::core::ffi::c_uint
                                        {
                                            if memcmp(
                                                (*item).hh.key,
                                                (*_ln).u.string.ptr as *const ::core::ffi::c_void,
                                                strlen((*_ln).u.string.ptr) as ::core::ffi::c_uint
                                                    as usize,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                break;
                                            }
                                        }
                                        if !(*item).hh.hh_next.is_null() {
                                            item = ((*item).hh.hh_next as *mut ::core::ffi::c_char)
                                                .offset(-(*(*lh).hh.tbl).hho)
                                                as *mut ::core::ffi::c_void
                                                as *mut lookup_hash
                                                as *mut lookup_hash;
                                        } else {
                                            item = ::core::ptr::null_mut::<lookup_hash>();
                                        }
                                    }
                                }
                            }
                            if !item.is_null() {
                                (*item).orderType = LOOKUP_ORDER_FORCE;
                                (*item).orderVal = j as u16;
                            }
                        }
                        j = j.wrapping_add(1);
                    }
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
                if !lh.is_null() {
                    _hs_insize = 1 as ::core::ffi::c_uint;
                    _hs_looping = 1 as ::core::ffi::c_uint;
                    _hs_list = &raw mut (*lh).hh as *mut UT_hash_handle;
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
                                        .offset((*(*lh).hh.tbl).hho)
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
                                            .offset((*(*lh).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                    } else {
                                        ::core::ptr::null_mut::<UT_hash_handle>()
                                    })
                                        as *mut UT_hash_handle;
                                    _hs_qsize = _hs_qsize.wrapping_sub(1);
                                } else if _hs_qsize == 0 as ::core::ffi::c_uint || _hs_q.is_null() {
                                    _hs_e = _hs_p;
                                    if !_hs_p.is_null() {
                                        _hs_p = (if !(*_hs_p).next.is_null() {
                                            ((*_hs_p).next as *mut ::core::ffi::c_char)
                                                .offset((*(*lh).hh.tbl).hho)
                                                as *mut UT_hash_handle
                                        } else {
                                            ::core::ptr::null_mut::<UT_hash_handle>()
                                        })
                                            as *mut UT_hash_handle;
                                    }
                                    _hs_psize = _hs_psize.wrapping_sub(1);
                                } else if by_lookup_order(
                                    (_hs_p as *mut ::core::ffi::c_char)
                                        .offset(-(*(*lh).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut lookup_hash,
                                    (_hs_q as *mut ::core::ffi::c_char)
                                        .offset(-(*(*lh).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut lookup_hash,
                                ) <= 0 as ::core::ffi::c_int
                                {
                                    _hs_e = _hs_p;
                                    if !_hs_p.is_null() {
                                        _hs_p = (if !(*_hs_p).next.is_null() {
                                            ((*_hs_p).next as *mut ::core::ffi::c_char)
                                                .offset((*(*lh).hh.tbl).hho)
                                                as *mut UT_hash_handle
                                        } else {
                                            ::core::ptr::null_mut::<UT_hash_handle>()
                                        })
                                            as *mut UT_hash_handle;
                                    }
                                    _hs_psize = _hs_psize.wrapping_sub(1);
                                } else {
                                    _hs_e = _hs_q;
                                    _hs_q = (if !(*_hs_q).next.is_null() {
                                        ((*_hs_q).next as *mut ::core::ffi::c_char)
                                            .offset((*(*lh).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                    } else {
                                        ::core::ptr::null_mut::<UT_hash_handle>()
                                    })
                                        as *mut UT_hash_handle;
                                    _hs_qsize = _hs_qsize.wrapping_sub(1);
                                }
                                if !_hs_tail.is_null() {
                                    (*_hs_tail).next = if !_hs_e.is_null() {
                                        (_hs_e as *mut ::core::ffi::c_char)
                                            .offset(-(*(*lh).hh.tbl).hho)
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
                                            .offset(-(*(*lh).hh.tbl).hho)
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
                            (*(*lh).hh.tbl).tail = _hs_tail;
                            lh = (_hs_list as *mut ::core::ffi::c_char)
                                .offset(-(*(*lh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut lookup_hash
                                as *mut lookup_hash;
                        }
                        _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
                    }
                }
                let mut fh: *mut feature_hash =
                    figureOutFeaturesFromJSON(features, lh, tag, options);
                let mut _hs_i_0: ::core::ffi::c_uint = 0;
                let mut _hs_looping_0: ::core::ffi::c_uint = 0;
                let mut _hs_nmerges_0: ::core::ffi::c_uint = 0;
                let mut _hs_insize_0: ::core::ffi::c_uint = 0;
                let mut _hs_psize_0: ::core::ffi::c_uint = 0;
                let mut _hs_qsize_0: ::core::ffi::c_uint = 0;
                let mut _hs_p_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hs_q_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hs_e_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hs_list_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hs_tail_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                if !fh.is_null() {
                    _hs_insize_0 = 1 as ::core::ffi::c_uint;
                    _hs_looping_0 = 1 as ::core::ffi::c_uint;
                    _hs_list_0 = &raw mut (*fh).hh as *mut UT_hash_handle;
                    while _hs_looping_0 != 0 as ::core::ffi::c_uint {
                        _hs_p_0 = _hs_list_0;
                        _hs_list_0 = ::core::ptr::null_mut::<UT_hash_handle>();
                        _hs_tail_0 = ::core::ptr::null_mut::<UT_hash_handle>();
                        _hs_nmerges_0 = 0 as ::core::ffi::c_uint;
                        while !_hs_p_0.is_null() {
                            _hs_nmerges_0 = _hs_nmerges_0.wrapping_add(1);
                            _hs_q_0 = _hs_p_0;
                            _hs_psize_0 = 0 as ::core::ffi::c_uint;
                            _hs_i_0 = 0 as ::core::ffi::c_uint;
                            while _hs_i_0 < _hs_insize_0 {
                                _hs_psize_0 = _hs_psize_0.wrapping_add(1);
                                _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                                    ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                        .offset((*(*fh).hh.tbl).hho)
                                        as *mut UT_hash_handle
                                } else {
                                    ::core::ptr::null_mut::<UT_hash_handle>()
                                }) as *mut UT_hash_handle;
                                if _hs_q_0.is_null() {
                                    break;
                                }
                                _hs_i_0 = _hs_i_0.wrapping_add(1);
                            }
                            _hs_qsize_0 = _hs_insize_0;
                            while _hs_psize_0 != 0 as ::core::ffi::c_uint
                                || _hs_qsize_0 != 0 as ::core::ffi::c_uint && !_hs_q_0.is_null()
                            {
                                if _hs_psize_0 == 0 as ::core::ffi::c_uint {
                                    _hs_e_0 = _hs_q_0;
                                    _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                                        ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                            .offset((*(*fh).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                    } else {
                                        ::core::ptr::null_mut::<UT_hash_handle>()
                                    })
                                        as *mut UT_hash_handle;
                                    _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                                } else if _hs_qsize_0 == 0 as ::core::ffi::c_uint
                                    || _hs_q_0.is_null()
                                {
                                    _hs_e_0 = _hs_p_0;
                                    if !_hs_p_0.is_null() {
                                        _hs_p_0 = (if !(*_hs_p_0).next.is_null() {
                                            ((*_hs_p_0).next as *mut ::core::ffi::c_char)
                                                .offset((*(*fh).hh.tbl).hho)
                                                as *mut UT_hash_handle
                                        } else {
                                            ::core::ptr::null_mut::<UT_hash_handle>()
                                        })
                                            as *mut UT_hash_handle;
                                    }
                                    _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                                } else if by_feature_name(
                                    (_hs_p_0 as *mut ::core::ffi::c_char)
                                        .offset(-(*(*fh).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut feature_hash,
                                    (_hs_q_0 as *mut ::core::ffi::c_char)
                                        .offset(-(*(*fh).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut feature_hash,
                                ) <= 0 as ::core::ffi::c_int
                                {
                                    _hs_e_0 = _hs_p_0;
                                    if !_hs_p_0.is_null() {
                                        _hs_p_0 = (if !(*_hs_p_0).next.is_null() {
                                            ((*_hs_p_0).next as *mut ::core::ffi::c_char)
                                                .offset((*(*fh).hh.tbl).hho)
                                                as *mut UT_hash_handle
                                        } else {
                                            ::core::ptr::null_mut::<UT_hash_handle>()
                                        })
                                            as *mut UT_hash_handle;
                                    }
                                    _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                                } else {
                                    _hs_e_0 = _hs_q_0;
                                    _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                                        ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                            .offset((*(*fh).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                    } else {
                                        ::core::ptr::null_mut::<UT_hash_handle>()
                                    })
                                        as *mut UT_hash_handle;
                                    _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                                }
                                if !_hs_tail_0.is_null() {
                                    (*_hs_tail_0).next = if !_hs_e_0.is_null() {
                                        (_hs_e_0 as *mut ::core::ffi::c_char)
                                            .offset(-(*(*fh).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                    } else {
                                        NULL
                                    };
                                } else {
                                    _hs_list_0 = _hs_e_0;
                                }
                                if !_hs_e_0.is_null() {
                                    (*_hs_e_0).prev = if !_hs_tail_0.is_null() {
                                        (_hs_tail_0 as *mut ::core::ffi::c_char)
                                            .offset(-(*(*fh).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                    } else {
                                        NULL
                                    };
                                }
                                _hs_tail_0 = _hs_e_0;
                            }
                            _hs_p_0 = _hs_q_0;
                        }
                        if !_hs_tail_0.is_null() {
                            (*_hs_tail_0).next = NULL;
                        }
                        if _hs_nmerges_0 <= 1 as ::core::ffi::c_uint {
                            _hs_looping_0 = 0 as ::core::ffi::c_uint;
                            (*(*fh).hh.tbl).tail = _hs_tail_0;
                            fh = (_hs_list_0 as *mut ::core::ffi::c_char)
                                .offset(-(*(*fh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut feature_hash
                                as *mut feature_hash;
                        }
                        _hs_insize_0 = _hs_insize_0.wrapping_mul(2 as ::core::ffi::c_uint);
                    }
                }
                let mut sh: *mut language_hash =
                    figureOutLanguagesFromJson(languages, fh, tag, options);
                let mut _hs_i_1: ::core::ffi::c_uint = 0;
                let mut _hs_looping_1: ::core::ffi::c_uint = 0;
                let mut _hs_nmerges_1: ::core::ffi::c_uint = 0;
                let mut _hs_insize_1: ::core::ffi::c_uint = 0;
                let mut _hs_psize_1: ::core::ffi::c_uint = 0;
                let mut _hs_qsize_1: ::core::ffi::c_uint = 0;
                let mut _hs_p_1: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hs_q_1: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hs_e_1: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hs_list_1: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hs_tail_1: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                if !sh.is_null() {
                    _hs_insize_1 = 1 as ::core::ffi::c_uint;
                    _hs_looping_1 = 1 as ::core::ffi::c_uint;
                    _hs_list_1 = &raw mut (*sh).hh as *mut UT_hash_handle;
                    while _hs_looping_1 != 0 as ::core::ffi::c_uint {
                        _hs_p_1 = _hs_list_1;
                        _hs_list_1 = ::core::ptr::null_mut::<UT_hash_handle>();
                        _hs_tail_1 = ::core::ptr::null_mut::<UT_hash_handle>();
                        _hs_nmerges_1 = 0 as ::core::ffi::c_uint;
                        while !_hs_p_1.is_null() {
                            _hs_nmerges_1 = _hs_nmerges_1.wrapping_add(1);
                            _hs_q_1 = _hs_p_1;
                            _hs_psize_1 = 0 as ::core::ffi::c_uint;
                            _hs_i_1 = 0 as ::core::ffi::c_uint;
                            while _hs_i_1 < _hs_insize_1 {
                                _hs_psize_1 = _hs_psize_1.wrapping_add(1);
                                _hs_q_1 = (if !(*_hs_q_1).next.is_null() {
                                    ((*_hs_q_1).next as *mut ::core::ffi::c_char)
                                        .offset((*(*sh).hh.tbl).hho)
                                        as *mut UT_hash_handle
                                } else {
                                    ::core::ptr::null_mut::<UT_hash_handle>()
                                }) as *mut UT_hash_handle;
                                if _hs_q_1.is_null() {
                                    break;
                                }
                                _hs_i_1 = _hs_i_1.wrapping_add(1);
                            }
                            _hs_qsize_1 = _hs_insize_1;
                            while _hs_psize_1 != 0 as ::core::ffi::c_uint
                                || _hs_qsize_1 != 0 as ::core::ffi::c_uint && !_hs_q_1.is_null()
                            {
                                if _hs_psize_1 == 0 as ::core::ffi::c_uint {
                                    _hs_e_1 = _hs_q_1;
                                    _hs_q_1 = (if !(*_hs_q_1).next.is_null() {
                                        ((*_hs_q_1).next as *mut ::core::ffi::c_char)
                                            .offset((*(*sh).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                    } else {
                                        ::core::ptr::null_mut::<UT_hash_handle>()
                                    })
                                        as *mut UT_hash_handle;
                                    _hs_qsize_1 = _hs_qsize_1.wrapping_sub(1);
                                } else if _hs_qsize_1 == 0 as ::core::ffi::c_uint
                                    || _hs_q_1.is_null()
                                {
                                    _hs_e_1 = _hs_p_1;
                                    if !_hs_p_1.is_null() {
                                        _hs_p_1 = (if !(*_hs_p_1).next.is_null() {
                                            ((*_hs_p_1).next as *mut ::core::ffi::c_char)
                                                .offset((*(*sh).hh.tbl).hho)
                                                as *mut UT_hash_handle
                                        } else {
                                            ::core::ptr::null_mut::<UT_hash_handle>()
                                        })
                                            as *mut UT_hash_handle;
                                    }
                                    _hs_psize_1 = _hs_psize_1.wrapping_sub(1);
                                } else if by_language_name(
                                    (_hs_p_1 as *mut ::core::ffi::c_char)
                                        .offset(-(*(*sh).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut language_hash,
                                    (_hs_q_1 as *mut ::core::ffi::c_char)
                                        .offset(-(*(*sh).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut language_hash,
                                ) <= 0 as ::core::ffi::c_int
                                {
                                    _hs_e_1 = _hs_p_1;
                                    if !_hs_p_1.is_null() {
                                        _hs_p_1 = (if !(*_hs_p_1).next.is_null() {
                                            ((*_hs_p_1).next as *mut ::core::ffi::c_char)
                                                .offset((*(*sh).hh.tbl).hho)
                                                as *mut UT_hash_handle
                                        } else {
                                            ::core::ptr::null_mut::<UT_hash_handle>()
                                        })
                                            as *mut UT_hash_handle;
                                    }
                                    _hs_psize_1 = _hs_psize_1.wrapping_sub(1);
                                } else {
                                    _hs_e_1 = _hs_q_1;
                                    _hs_q_1 = (if !(*_hs_q_1).next.is_null() {
                                        ((*_hs_q_1).next as *mut ::core::ffi::c_char)
                                            .offset((*(*sh).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                    } else {
                                        ::core::ptr::null_mut::<UT_hash_handle>()
                                    })
                                        as *mut UT_hash_handle;
                                    _hs_qsize_1 = _hs_qsize_1.wrapping_sub(1);
                                }
                                if !_hs_tail_1.is_null() {
                                    (*_hs_tail_1).next = if !_hs_e_1.is_null() {
                                        (_hs_e_1 as *mut ::core::ffi::c_char)
                                            .offset(-(*(*sh).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                    } else {
                                        NULL
                                    };
                                } else {
                                    _hs_list_1 = _hs_e_1;
                                }
                                if !_hs_e_1.is_null() {
                                    (*_hs_e_1).prev = if !_hs_tail_1.is_null() {
                                        (_hs_tail_1 as *mut ::core::ffi::c_char)
                                            .offset(-(*(*sh).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                    } else {
                                        NULL
                                    };
                                }
                                _hs_tail_1 = _hs_e_1;
                            }
                            _hs_p_1 = _hs_q_1;
                        }
                        if !_hs_tail_1.is_null() {
                            (*_hs_tail_1).next = NULL;
                        }
                        if _hs_nmerges_1 <= 1 as ::core::ffi::c_uint {
                            _hs_looping_1 = 0 as ::core::ffi::c_uint;
                            (*(*sh).hh.tbl).tail = _hs_tail_1;
                            sh = (_hs_list_1 as *mut ::core::ffi::c_char)
                                .offset(-(*(*sh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut language_hash
                                as *mut language_hash;
                        }
                        _hs_insize_1 = _hs_insize_1.wrapping_mul(2 as ::core::ffi::c_uint);
                    }
                }
                if (if !lh.is_null() {
                    (*(*lh).hh.tbl).num_items
                } else {
                    0 as ::core::ffi::c_uint
                }) == 0
                    || (if !fh.is_null() {
                        (*(*fh).hh.tbl).num_items
                    } else {
                        0 as ::core::ffi::c_uint
                    }) == 0
                    || (if !sh.is_null() {
                        (*(*sh).hh.tbl).num_items
                    } else {
                        0 as ::core::ffi::c_uint
                    }) == 0
                {
                    (*(*options).logger)
                        .dedent
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                    );
                    current_block = 12498981253432484999;
                    break;
                } else {
                    let mut s: *mut lookup_hash = ::core::ptr::null_mut::<lookup_hash>();
                    let mut tmp: *mut lookup_hash = ::core::ptr::null_mut::<lookup_hash>();
                    s = lh;
                    tmp = (if !lh.is_null() { (*lh).hh.next } else { NULL }) as *mut lookup_hash
                        as *mut lookup_hash;
                    while !s.is_null() {
                        otl_iLookupList.push.expect("non-null function pointer")(
                            &raw mut (*otl).lookups,
                            (*s).lookup as otl_LookupPtr,
                        );
                        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s).hh;
                        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                            free((*(*lh).hh.tbl).buckets as *mut ::core::ffi::c_void);
                            free((*lh).hh.tbl as *mut ::core::ffi::c_void);
                            lh = ::core::ptr::null_mut::<lookup_hash>();
                        } else {
                            let mut _hd_bkt: ::core::ffi::c_uint = 0;
                            if _hd_hh_del == (*(*lh).hh.tbl).tail {
                                (*(*lh).hh.tbl).tail = ((*_hd_hh_del).prev
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*lh).hh.tbl).hho)
                                    as *mut UT_hash_handle
                                    as *mut UT_hash_handle;
                            }
                            if !(*_hd_hh_del).prev.is_null() {
                                let ref mut fresh0 = (*(((*_hd_hh_del).prev
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*lh).hh.tbl).hho)
                                    as *mut UT_hash_handle))
                                    .next;
                                *fresh0 = (*_hd_hh_del).next;
                            } else {
                                lh = (*_hd_hh_del).next as *mut lookup_hash as *mut lookup_hash;
                            }
                            if !(*_hd_hh_del).next.is_null() {
                                let ref mut fresh1 = (*(((*_hd_hh_del).next
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*lh).hh.tbl).hho)
                                    as *mut UT_hash_handle))
                                    .prev;
                                *fresh1 = (*_hd_hh_del).prev;
                            }
                            _hd_bkt = (*_hd_hh_del).hashv
                                & (*(*lh).hh.tbl)
                                    .num_buckets
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            let mut _hd_head: *mut UT_hash_bucket =
                                (*(*lh).hh.tbl).buckets.offset(_hd_bkt as isize)
                                    as *mut UT_hash_bucket;
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
                            (*(*lh).hh.tbl).num_items = (*(*lh).hh.tbl).num_items.wrapping_sub(1);
                        }
                        sdsfree((*s).name as sds);
                        free(s as *mut ::core::ffi::c_void);
                        s = ::core::ptr::null_mut::<lookup_hash>();
                        s = tmp;
                        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL })
                            as *mut lookup_hash as *mut lookup_hash;
                    }
                    let mut s_0: *mut feature_hash = ::core::ptr::null_mut::<feature_hash>();
                    let mut tmp_0: *mut feature_hash = ::core::ptr::null_mut::<feature_hash>();
                    s_0 = fh;
                    tmp_0 = (if !fh.is_null() { (*fh).hh.next } else { NULL }) as *mut feature_hash
                        as *mut feature_hash;
                    while !s_0.is_null() {
                        if !(*s_0).alias {
                            otl_iFeatureList.push.expect("non-null function pointer")(
                                &raw mut (*otl).features,
                                (*s_0).feature as otl_FeaturePtr,
                            );
                        }
                        let mut _hd_hh_del_0: *mut UT_hash_handle = &raw mut (*s_0).hh;
                        if (*_hd_hh_del_0).prev.is_null() && (*_hd_hh_del_0).next.is_null() {
                            free((*(*fh).hh.tbl).buckets as *mut ::core::ffi::c_void);
                            free((*fh).hh.tbl as *mut ::core::ffi::c_void);
                            fh = ::core::ptr::null_mut::<feature_hash>();
                        } else {
                            let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                            if _hd_hh_del_0 == (*(*fh).hh.tbl).tail {
                                (*(*fh).hh.tbl).tail = ((*_hd_hh_del_0).prev
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*fh).hh.tbl).hho)
                                    as *mut UT_hash_handle
                                    as *mut UT_hash_handle;
                            }
                            if !(*_hd_hh_del_0).prev.is_null() {
                                let ref mut fresh2 = (*(((*_hd_hh_del_0).prev
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*fh).hh.tbl).hho)
                                    as *mut UT_hash_handle))
                                    .next;
                                *fresh2 = (*_hd_hh_del_0).next;
                            } else {
                                fh = (*_hd_hh_del_0).next as *mut feature_hash as *mut feature_hash;
                            }
                            if !(*_hd_hh_del_0).next.is_null() {
                                let ref mut fresh3 = (*(((*_hd_hh_del_0).next
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*fh).hh.tbl).hho)
                                    as *mut UT_hash_handle))
                                    .prev;
                                *fresh3 = (*_hd_hh_del_0).prev;
                            }
                            _hd_bkt_0 = (*_hd_hh_del_0).hashv
                                & (*(*fh).hh.tbl)
                                    .num_buckets
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            let mut _hd_head_0: *mut UT_hash_bucket =
                                (*(*fh).hh.tbl).buckets.offset(_hd_bkt_0 as isize)
                                    as *mut UT_hash_bucket;
                            (*_hd_head_0).count = (*_hd_head_0).count.wrapping_sub(1);
                            if (*_hd_head_0).hh_head == _hd_hh_del_0 {
                                (*_hd_head_0).hh_head =
                                    (*_hd_hh_del_0).hh_next as *mut UT_hash_handle;
                            }
                            if !(*_hd_hh_del_0).hh_prev.is_null() {
                                (*(*_hd_hh_del_0).hh_prev).hh_next = (*_hd_hh_del_0).hh_next;
                            }
                            if !(*_hd_hh_del_0).hh_next.is_null() {
                                (*(*_hd_hh_del_0).hh_next).hh_prev = (*_hd_hh_del_0).hh_prev;
                            }
                            (*(*fh).hh.tbl).num_items = (*(*fh).hh.tbl).num_items.wrapping_sub(1);
                        }
                        sdsfree((*s_0).name as sds);
                        free(s_0 as *mut ::core::ffi::c_void);
                        s_0 = ::core::ptr::null_mut::<feature_hash>();
                        s_0 = tmp_0;
                        tmp_0 = (if !tmp_0.is_null() {
                            (*tmp_0).hh.next
                        } else {
                            NULL
                        }) as *mut feature_hash
                            as *mut feature_hash;
                    }
                    let mut s_1: *mut language_hash = ::core::ptr::null_mut::<language_hash>();
                    let mut tmp_1: *mut language_hash = ::core::ptr::null_mut::<language_hash>();
                    s_1 = sh;
                    tmp_1 = (if !sh.is_null() { (*sh).hh.next } else { NULL }) as *mut language_hash
                        as *mut language_hash;
                    while !s_1.is_null() {
                        otl_iLangSystemList.push.expect("non-null function pointer")(
                            &raw mut (*otl).languages,
                            (*s_1).language as otl_LanguageSystemPtr,
                        );
                        let mut _hd_hh_del_1: *mut UT_hash_handle = &raw mut (*s_1).hh;
                        if (*_hd_hh_del_1).prev.is_null() && (*_hd_hh_del_1).next.is_null() {
                            free((*(*sh).hh.tbl).buckets as *mut ::core::ffi::c_void);
                            free((*sh).hh.tbl as *mut ::core::ffi::c_void);
                            sh = ::core::ptr::null_mut::<language_hash>();
                        } else {
                            let mut _hd_bkt_1: ::core::ffi::c_uint = 0;
                            if _hd_hh_del_1 == (*(*sh).hh.tbl).tail {
                                (*(*sh).hh.tbl).tail = ((*_hd_hh_del_1).prev
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*sh).hh.tbl).hho)
                                    as *mut UT_hash_handle
                                    as *mut UT_hash_handle;
                            }
                            if !(*_hd_hh_del_1).prev.is_null() {
                                let ref mut fresh4 = (*(((*_hd_hh_del_1).prev
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*sh).hh.tbl).hho)
                                    as *mut UT_hash_handle))
                                    .next;
                                *fresh4 = (*_hd_hh_del_1).next;
                            } else {
                                sh = (*_hd_hh_del_1).next as *mut language_hash
                                    as *mut language_hash;
                            }
                            if !(*_hd_hh_del_1).next.is_null() {
                                let ref mut fresh5 = (*(((*_hd_hh_del_1).next
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*sh).hh.tbl).hho)
                                    as *mut UT_hash_handle))
                                    .prev;
                                *fresh5 = (*_hd_hh_del_1).prev;
                            }
                            _hd_bkt_1 = (*_hd_hh_del_1).hashv
                                & (*(*sh).hh.tbl)
                                    .num_buckets
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            let mut _hd_head_1: *mut UT_hash_bucket =
                                (*(*sh).hh.tbl).buckets.offset(_hd_bkt_1 as isize)
                                    as *mut UT_hash_bucket;
                            (*_hd_head_1).count = (*_hd_head_1).count.wrapping_sub(1);
                            if (*_hd_head_1).hh_head == _hd_hh_del_1 {
                                (*_hd_head_1).hh_head =
                                    (*_hd_hh_del_1).hh_next as *mut UT_hash_handle;
                            }
                            if !(*_hd_hh_del_1).hh_prev.is_null() {
                                (*(*_hd_hh_del_1).hh_prev).hh_next = (*_hd_hh_del_1).hh_next;
                            }
                            if !(*_hd_hh_del_1).hh_next.is_null() {
                                (*(*_hd_hh_del_1).hh_next).hh_prev = (*_hd_hh_del_1).hh_prev;
                            }
                            (*(*sh).hh.tbl).num_items = (*(*sh).hh.tbl).num_items.wrapping_sub(1);
                        }
                        sdsfree((*s_1).name as sds);
                        free(s_1 as *mut ::core::ffi::c_void);
                        s_1 = ::core::ptr::null_mut::<language_hash>();
                        s_1 = tmp_1;
                        tmp_1 = (if !tmp_1.is_null() {
                            (*tmp_1).hh.next
                        } else {
                            NULL
                        }) as *mut language_hash
                            as *mut language_hash;
                    }
                    ___loggedstep_v = false;
                    (*(*options).logger)
                        .finish
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                    );
                }
            }
            match current_block {
                12498981253432484999 => {}
                _ => return otl,
            }
        }
    }
    if !otl.is_null() {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            sdscatprintf(
                sdsempty(),
                b"[OTFCC-fea] Ignoring invalid or incomplete OTL table %s.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                tag,
            ),
        );
        table_iOTL.free.expect("non-null function pointer")(otl);
    }
    return ::core::ptr::null_mut::<table_OTL>();
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
