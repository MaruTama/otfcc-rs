extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: usize) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: usize,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: usize,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: usize,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> usize;
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
    fn otfcc_build_gsub_single_subtable(
        _subtable: *const otl_Subtable,
        heuristics: otl_BuildHeuristics,
    ) -> *mut caryll_Buffer;
    fn otfcc_build_gsub_multi_subtable_split(
        _subtable: *const otl_Subtable,
        heuristics: otl_BuildHeuristics,
        count: *mut tableid_t,
    ) -> *mut *mut caryll_Buffer;
    fn otfcc_build_gsub_ligature_subtable(
        _subtable: *const otl_Subtable,
        heuristics: otl_BuildHeuristics,
    ) -> *mut caryll_Buffer;
    fn otfcc_build_gsub_reverse(
        _subtable: *const otl_Subtable,
        heuristics: otl_BuildHeuristics,
    ) -> *mut caryll_Buffer;
    fn otfcc_build_gpos_single(
        _subtable: *const otl_Subtable,
        heuristics: otl_BuildHeuristics,
    ) -> *mut caryll_Buffer;
    fn otfcc_build_gpos_cursive(
        _subtable: *const otl_Subtable,
        heuristics: otl_BuildHeuristics,
    ) -> *mut caryll_Buffer;
    fn otfcc_build_gpos_markToSingle(
        _subtable: *const otl_Subtable,
        heuristics: otl_BuildHeuristics,
    ) -> *mut caryll_Buffer;
    fn otfcc_build_gpos_markToLigature(
        _subtable: *const otl_Subtable,
        heuristics: otl_BuildHeuristics,
    ) -> *mut caryll_Buffer;
    fn otfcc_classifiedBuildChaining(
        lookup: *const otl_Lookup,
        subtableBuffers: *mut *mut *mut caryll_Buffer,
        lastOffset: *mut usize,
    ) -> tableid_t;
    fn otfcc_chainingLookupIsContextualLookup(lookup: *const otl_Lookup) -> bool;
    fn otfcc_build_gpos_pair(
        _subtable: *const otl_Subtable,
        heuristics: otl_BuildHeuristics,
    ) -> *mut caryll_Buffer;
}

use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{otfcc_GlyphHandle, otfcc_LookupHandle};
use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: u8,
    pub alloc: u8,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: u16,
    pub alloc: u16,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: u32,
    pub alloc: u32,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: u64,
    pub alloc: u64,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed = 10;
pub const log_vl_info: C2RustUnnamed = 5;
pub const log_vl_notice: C2RustUnnamed = 2;
pub const log_vl_important: C2RustUnnamed = 1;
pub const log_vl_critical: C2RustUnnamed = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
    pub c2rust_unnamed: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub type otl_LookupPtr = *mut otl_Lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LookupList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_LookupPtr,
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
pub struct otl_Feature {
    pub name: sds,
    pub lookups: otl_LookupRefList,
}
pub type otl_FeaturePtr = *mut otl_Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_FeatureList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_FeaturePtr,
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
pub struct otl_LanguageSystem {
    pub name: sds,
    pub requiredFeature: otl_FeatureRef,
    pub features: otl_FeatureRefList,
}
pub type otl_LanguageSystemPtr = *mut otl_LanguageSystem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LangSystemList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_LanguageSystemPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_OTL {
    pub lookups: otl_LookupList,
    pub features: otl_FeatureList,
    pub languages: otl_LangSystemList,
}
pub type otl_BuildHeuristics = ::core::ffi::c_uint;
pub const OTL_BH_GSUB_VERT: otl_BuildHeuristics = 1;
pub const OTL_BH_NORMAL: otl_BuildHeuristics = 0;
pub type _otl_Builder =
    Option<unsafe extern "C" fn(*const otl_Subtable, otl_BuildHeuristics) -> *mut caryll_Buffer>;
pub type _otl_SplitBuilder = Option<
    unsafe extern "C" fn(
        *const otl_Subtable,
        otl_BuildHeuristics,
        *mut tableid_t,
    ) -> *mut *mut caryll_Buffer,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct script_stat_hash {
    pub tag: sds,
    pub lc: u16,
    pub dl: *mut otl_LanguageSystem,
    pub ll: *mut *mut otl_LanguageSystem,
    pub hh: UT_hash_handle,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
pub const HASH_INITIAL_NUM_BUCKETS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const HASH_INITIAL_NUM_BUCKETS_LOG2: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const HASH_BKT_CAPACITY_THRESH: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
pub const HASH_SIGNATURE: ::core::ffi::c_uint = 0xa0111fe1 as ::core::ffi::c_uint;
pub const LARGE_SUBTABLE_LIMIT: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe extern "C" fn featureNameToTag(name: sds) -> u32 {
    let mut tag: u32 = 0 as u32;
    if sdslen(name) > 0 as usize {
        tag |= ((*name.offset(0 as ::core::ffi::c_int as isize) as u8 as ::core::ffi::c_int)
            << 24 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as u32;
    }
    if sdslen(name) > 1 as usize {
        tag |= ((*name.offset(1 as ::core::ffi::c_int as isize) as u8 as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32;
    }
    if sdslen(name) > 2 as usize {
        tag |= ((*name.offset(2 as ::core::ffi::c_int as isize) as u8 as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
    }
    if sdslen(name) > 3 as usize {
        tag |= ((*name.offset(3 as ::core::ffi::c_int as isize) as u8 as ::core::ffi::c_int)
            << 0 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as u32;
    }
    return tag;
}
unsafe extern "C" fn _declare_lookup_writer(
    mut type_0: otl_LookupType,
    mut fn_0: _otl_Builder,
    mut lookup: *const otl_Lookup,
    mut subtables: *mut *mut *mut caryll_Buffer,
    mut lastOffset: *mut usize,
    mut preferExtensionForThisLUT: *mut bool,
    mut heuristics: otl_BuildHeuristics,
) -> tableid_t {
    if (*lookup).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        *subtables = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut caryll_Buffer>() as usize)
                .wrapping_mul((*lookup).subtables.length),
            38 as ::core::ffi::c_ulong,
        ) as *mut *mut caryll_Buffer;
        let mut totalBufSizeShort: usize = 0 as usize;
        let mut totalBufSizeExt: usize = 0 as usize;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as usize) < (*lookup).subtables.length {
            let mut buf: *mut caryll_Buffer = fn_0.expect("non-null function pointer")(
                *(*lookup).subtables.items.offset(j as isize) as *const otl_Subtable,
                heuristics,
            );
            let ref mut fresh1 = *(*subtables).offset(j as isize);
            *fresh1 = buf;
            totalBufSizeShort = totalBufSizeShort.wrapping_add((*buf).size);
            totalBufSizeExt = totalBufSizeExt.wrapping_add(8 as usize);
            j = j.wrapping_add(1);
        }
        if totalBufSizeShort > LARGE_SUBTABLE_LIMIT as usize {
            *lastOffset = (*lastOffset).wrapping_add(totalBufSizeExt);
            *preferExtensionForThisLUT = true;
        } else {
            *lastOffset = (*lastOffset).wrapping_add(totalBufSizeShort);
            *preferExtensionForThisLUT = false;
        }
        return (*lookup).subtables.length as tableid_t;
    }
    return 0 as tableid_t;
}
unsafe extern "C" fn _declare_lookup_writer_split(
    mut type_0: otl_LookupType,
    mut fn_0: _otl_SplitBuilder,
    mut lookup: *const otl_Lookup,
    mut subtables: *mut *mut *mut caryll_Buffer,
    mut lastOffset: *mut usize,
    mut preferExtensionForThisLUT: *mut bool,
    mut heuristics: otl_BuildHeuristics,
) -> tableid_t {
    if (*lookup).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        let mut buffers: *mut *mut caryll_Buffer = ::core::ptr::null_mut::<*mut caryll_Buffer>();
        let mut total: tableid_t = 0 as tableid_t;
        let mut totalBufSizeShort: usize = 0 as usize;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as usize) < (*lookup).subtables.length {
            let mut nPart: tableid_t = 0 as tableid_t;
            let mut part: *mut *mut caryll_Buffer = fn_0.expect("non-null function pointer")(
                *(*lookup).subtables.items.offset(j as isize) as *const otl_Subtable,
                heuristics,
                &raw mut nPart,
            );
            let mut k: tableid_t = 0 as tableid_t;
            while (k as ::core::ffi::c_int) < nPart as ::core::ffi::c_int {
                buffers = __caryll_reallocate(
                    buffers as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<*mut caryll_Buffer>() as usize).wrapping_mul(
                        (total as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
                    ),
                    81 as ::core::ffi::c_ulong,
                ) as *mut *mut caryll_Buffer;
                let ref mut fresh2 = *buffers.offset(total as isize);
                *fresh2 = *part.offset(k as isize);
                totalBufSizeShort =
                    totalBufSizeShort.wrapping_add((**part.offset(k as isize)).size);
                total = total.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            free(part as *mut ::core::ffi::c_void);
            part = ::core::ptr::null_mut::<*mut caryll_Buffer>();
            j = j.wrapping_add(1);
        }
        *subtables = buffers;
        if totalBufSizeShort > LARGE_SUBTABLE_LIMIT as usize {
            *lastOffset = (*lastOffset)
                .wrapping_add((8 as ::core::ffi::c_int * total as ::core::ffi::c_int) as usize);
            *preferExtensionForThisLUT = true;
        } else {
            *lastOffset = (*lastOffset).wrapping_add(totalBufSizeShort);
            *preferExtensionForThisLUT = false;
        }
        return total;
    }
    return 0 as tableid_t;
}
unsafe extern "C" fn _build_lookup(
    mut lookup: *const otl_Lookup,
    mut subtables: *mut *mut *mut caryll_Buffer,
    mut lastOffset: *mut usize,
    mut preferExtensionForThisLUT: *mut bool,
    mut heuristics: otl_BuildHeuristics,
) -> tableid_t {
    if (*lookup).type_0 as ::core::ffi::c_uint
        == otl_type_gpos_chaining as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*lookup).type_0 as ::core::ffi::c_uint
            == otl_type_gsub_chaining as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return otfcc_classifiedBuildChaining(lookup, subtables, lastOffset);
    }
    let mut written: tableid_t = 0 as tableid_t;
    if written == 0 {
        written = _declare_lookup_writer(
            otl_type_gsub_single,
            Some(
                otfcc_build_gsub_single_subtable
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                    ) -> *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer_split(
            otl_type_gsub_multiple,
            Some(
                otfcc_build_gsub_multi_subtable_split
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                        *mut tableid_t,
                    ) -> *mut *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer_split(
            otl_type_gsub_alternate,
            Some(
                otfcc_build_gsub_multi_subtable_split
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                        *mut tableid_t,
                    ) -> *mut *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            otl_type_gsub_ligature,
            Some(
                otfcc_build_gsub_ligature_subtable
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                    ) -> *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            otl_type_gsub_reverse,
            Some(
                otfcc_build_gsub_reverse
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                    ) -> *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            otl_type_gpos_single,
            Some(
                otfcc_build_gpos_single
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                    ) -> *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            otl_type_gpos_pair,
            Some(
                otfcc_build_gpos_pair
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                    ) -> *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            otl_type_gpos_cursive,
            Some(
                otfcc_build_gpos_cursive
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                    ) -> *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            otl_type_gpos_markToBase,
            Some(
                otfcc_build_gpos_markToSingle
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                    ) -> *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            otl_type_gpos_markToMark,
            Some(
                otfcc_build_gpos_markToSingle
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                    ) -> *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            otl_type_gpos_markToLigature,
            Some(
                otfcc_build_gpos_markToLigature
                    as unsafe extern "C" fn(
                        *const otl_Subtable,
                        otl_BuildHeuristics,
                    ) -> *mut caryll_Buffer,
            ),
            lookup,
            subtables,
            lastOffset,
            preferExtensionForThisLUT,
            heuristics,
        );
    }
    return written;
}
unsafe extern "C" fn getLookupHeuristics(
    mut table: *const table_OTL,
    mut lut: *const otl_Lookup,
) -> otl_BuildHeuristics {
    let mut heu: otl_BuildHeuristics = OTL_BH_NORMAL;
    if (*lut).type_0 as ::core::ffi::c_uint
        == otl_type_gsub_single as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut j: tableid_t = 0 as tableid_t;
        while (j as usize) < (*table).features.length {
            let mut fea: *const otl_Feature =
                *(*table).features.items.offset(j as isize) as *const otl_Feature;
            if !(featureNameToTag((*fea).name) != 1986359924i32 as u32) {
                let mut k: tableid_t = 0 as tableid_t;
                while (k as usize) < (*fea).lookups.length {
                    if *(*fea).lookups.items.offset(k as isize) == lut {
                        heu = ::core::mem::transmute::<::core::ffi::c_uint, otl_BuildHeuristics>(
                            heu as ::core::ffi::c_uint
                                | OTL_BH_GSUB_VERT as ::core::ffi::c_int as ::core::ffi::c_uint,
                        );
                    }
                    k = k.wrapping_add(1);
                }
            }
            j = j.wrapping_add(1);
        }
    }
    return heu;
}
unsafe extern "C" fn writeOTLLookups(
    mut table: *const table_OTL,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut bk_Block {
    let mut subtables: *mut *mut *mut caryll_Buffer =
        ::core::ptr::null_mut::<*mut *mut caryll_Buffer>();
    subtables = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut *mut caryll_Buffer>() as usize)
            .wrapping_mul((*table).lookups.length),
        150 as ::core::ffi::c_ulong,
    ) as *mut *mut *mut caryll_Buffer;
    let mut preferExtForThisLut: *mut bool = ::core::ptr::null_mut::<bool>();
    let mut subtableQuantity: *mut tableid_t = ::core::ptr::null_mut::<tableid_t>();
    subtableQuantity = __caryll_allocate_clean(
        (::core::mem::size_of::<tableid_t>() as usize).wrapping_mul((*table).lookups.length),
        153 as ::core::ffi::c_ulong,
    ) as *mut tableid_t;
    preferExtForThisLut = __caryll_allocate_clean(
        (::core::mem::size_of::<bool>() as usize).wrapping_mul((*table).lookups.length),
        154 as ::core::ffi::c_ulong,
    ) as *mut bool;
    let mut lastOffset: usize = 0 as usize;
    let mut j: tableid_t = 0 as tableid_t;
    while (j as usize) < (*table).lookups.length {
        let mut lookup: *mut otl_Lookup =
            *(*table).lookups.items.offset(j as isize) as *mut otl_Lookup;
        let mut heu: otl_BuildHeuristics = getLookupHeuristics(table, lookup);
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            sdscatprintf(
                sdsempty(),
                b"Building lookup %s (%u/%u)\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*lookup).name,
                j as ::core::ffi::c_int,
                (*table).lookups.length as u32,
            ),
        );
        *subtableQuantity.offset(j as isize) = _build_lookup(
            lookup,
            subtables.offset(j as isize) as *mut *mut *mut caryll_Buffer,
            &raw mut lastOffset,
            preferExtForThisLut.offset(j as isize) as *mut bool,
            heu,
        );
        j = j.wrapping_add(1);
    }
    let mut headerSize: usize =
        (2 as usize).wrapping_add((2 as usize).wrapping_mul((*table).lookups.length));
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as usize) < (*table).lookups.length {
        if *subtableQuantity.offset(j_0 as isize) != 0 {
            headerSize = headerSize.wrapping_add(
                (6 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *subtableQuantity.offset(j_0 as isize) as ::core::ffi::c_int)
                    as usize,
            );
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut useExtended: bool = lastOffset >= (0xff00 as usize).wrapping_sub(headerSize);
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        (*table).lookups.length,
        bkover as ::core::ffi::c_int,
    );
    let mut j_1: tableid_t = 0 as tableid_t;
    while (j_1 as usize) < (*table).lookups.length {
        if *subtableQuantity.offset(j_1 as isize) == 0 {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_notice as ::core::ffi::c_int as u8,
                log_type_info,
                sdscatprintf(
                    sdsempty(),
                    b"Lookup %s is empty.\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (**(*table).lookups.items.offset(j_1 as isize)).name,
                ),
            );
        }
        let mut lookup_0: *mut otl_Lookup =
            *(*table).lookups.items.offset(j_1 as isize) as *mut otl_Lookup;
        let canBeContextual: bool = otfcc_chainingLookupIsContextualLookup(lookup_0);
        let useExtendedForIt: bool = useExtended as ::core::ffi::c_int != 0
            || *preferExtForThisLut.offset(j_1 as isize) as ::core::ffi::c_int != 0;
        if useExtendedForIt {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_notice as ::core::ffi::c_int as u8,
                log_type_info,
                sdscatprintf(
                    sdsempty(),
                    b"[OTFCC-fea] Using extended OpenType table layout for %s/%s.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    tag,
                    (*lookup_0).name,
                ),
            );
        }
        let mut lookupType: u16 = (if useExtendedForIt as ::core::ffi::c_int != 0 {
            (if (*lookup_0).type_0 as ::core::ffi::c_uint
                > otl_type_gpos_unknown as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                otl_type_gpos_extend as ::core::ffi::c_int
                    - otl_type_gpos_unknown as ::core::ffi::c_int
            } else if (*lookup_0).type_0 as ::core::ffi::c_uint
                > otl_type_gsub_unknown as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                otl_type_gsub_extend as ::core::ffi::c_int
                    - otl_type_gsub_unknown as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint
        } else {
            (if (*lookup_0).type_0 as ::core::ffi::c_uint
                > otl_type_gpos_unknown as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                ((*lookup_0).type_0 as ::core::ffi::c_uint).wrapping_sub(
                    otl_type_gpos_unknown as ::core::ffi::c_int as ::core::ffi::c_uint,
                )
            } else {
                if (*lookup_0).type_0 as ::core::ffi::c_uint
                    > otl_type_gsub_unknown as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    ((*lookup_0).type_0 as ::core::ffi::c_uint).wrapping_sub(
                        otl_type_gsub_unknown as ::core::ffi::c_int as ::core::ffi::c_uint,
                    )
                } else {
                    0 as ::core::ffi::c_uint
                }
            })
            .wrapping_sub(
                (if canBeContextual as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            )
        }) as u16;
        let mut blk: *mut bk_Block = bk_new_Block(
            b16 as ::core::ffi::c_int,
            lookupType as ::core::ffi::c_int,
            b16 as ::core::ffi::c_int,
            (*lookup_0).flags as ::core::ffi::c_int,
            b16 as ::core::ffi::c_int,
            *subtableQuantity.offset(j_1 as isize) as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        let mut k: tableid_t = 0 as tableid_t;
        while (k as ::core::ffi::c_int)
            < *subtableQuantity.offset(j_1 as isize) as ::core::ffi::c_int
        {
            if useExtendedForIt {
                let mut extensionLookupType: u16 = (if (*lookup_0).type_0
                    as ::core::ffi::c_uint
                    > otl_type_gpos_unknown as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    ((*lookup_0).type_0 as ::core::ffi::c_uint).wrapping_sub(
                        otl_type_gpos_unknown as ::core::ffi::c_int as ::core::ffi::c_uint,
                    )
                } else {
                    if (*lookup_0).type_0 as ::core::ffi::c_uint
                        > otl_type_gsub_unknown as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        ((*lookup_0).type_0 as ::core::ffi::c_uint).wrapping_sub(
                            otl_type_gsub_unknown as ::core::ffi::c_int as ::core::ffi::c_uint,
                        )
                    } else {
                        0 as ::core::ffi::c_uint
                    }
                })
                .wrapping_sub(
                    (if canBeContextual as ::core::ffi::c_int != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ) as u16;
                let mut stub: *mut bk_Block = bk_new_Block(
                    b16 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    b16 as ::core::ffi::c_int,
                    extensionLookupType as ::core::ffi::c_int,
                    p32 as ::core::ffi::c_int,
                    bk_newBlockFromBuffer(*(*subtables.offset(j_1 as isize)).offset(k as isize)),
                    bkover as ::core::ffi::c_int,
                );
                bk_push(
                    blk,
                    p16 as ::core::ffi::c_int,
                    stub,
                    bkover as ::core::ffi::c_int,
                );
            } else {
                bk_push(
                    blk,
                    p16 as ::core::ffi::c_int,
                    bk_newBlockFromBuffer(*(*subtables.offset(j_1 as isize)).offset(k as isize)),
                    bkover as ::core::ffi::c_int,
                );
            }
            k = k.wrapping_add(1);
        }
        bk_push(
            blk,
            b16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            blk,
            bkover as ::core::ffi::c_int,
        );
        free(*subtables.offset(j_1 as isize) as *mut ::core::ffi::c_void);
        let ref mut fresh0 = *subtables.offset(j_1 as isize);
        *fresh0 = ::core::ptr::null_mut::<*mut caryll_Buffer>();
        j_1 = j_1.wrapping_add(1);
    }
    free(subtables as *mut ::core::ffi::c_void);
    subtables = ::core::ptr::null_mut::<*mut *mut caryll_Buffer>();
    free(subtableQuantity as *mut ::core::ffi::c_void);
    subtableQuantity = ::core::ptr::null_mut::<tableid_t>();
    free(preferExtForThisLut as *mut ::core::ffi::c_void);
    preferExtForThisLut = ::core::ptr::null_mut::<bool>();
    return root;
}
unsafe extern "C" fn writeOTLFeatures(
    mut table: *const table_OTL,
    mut _options: *const otfcc_Options,
) -> *mut bk_Block {
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        (*table).features.length,
        bkover as ::core::ffi::c_int,
    );
    let mut j: tableid_t = 0 as tableid_t;
    while (j as usize) < (*table).features.length {
        let mut fea: *mut bk_Block = bk_new_Block(
            p16 as ::core::ffi::c_int,
            NULL,
            b16 as ::core::ffi::c_int,
            (**(*table).features.items.offset(j as isize))
                .lookups
                .length,
            bkover as ::core::ffi::c_int,
        );
        let mut k: tableid_t = 0 as tableid_t;
        while (k as usize)
            < (**(*table).features.items.offset(j as isize))
                .lookups
                .length
        {
            let mut l: tableid_t = 0 as tableid_t;
            while (l as usize) < (*table).lookups.length {
                if *(**(*table).features.items.offset(j as isize))
                    .lookups
                    .items
                    .offset(k as isize)
                    == *(*table).lookups.items.offset(l as isize) as otl_LookupRef
                {
                    bk_push(
                        fea,
                        b16 as ::core::ffi::c_int,
                        l as ::core::ffi::c_int,
                        bkover as ::core::ffi::c_int,
                    );
                    break;
                } else {
                    l = l.wrapping_add(1);
                }
            }
            k = k.wrapping_add(1);
        }
        bk_push(
            root,
            b32 as ::core::ffi::c_int,
            featureNameToTag((**(*table).features.items.offset(j as isize)).name),
            p16 as ::core::ffi::c_int,
            fea,
            bkover as ::core::ffi::c_int,
        );
        j = j.wrapping_add(1);
    }
    return root;
}
unsafe extern "C" fn featureIndex(
    mut feature: *const otl_Feature,
    mut table: *const table_OTL,
) -> tableid_t {
    let mut j: tableid_t = 0 as tableid_t;
    while (j as usize) < (*table).features.length {
        if *(*table).features.items.offset(j as isize) == feature as otl_FeaturePtr {
            return j;
        }
        j = j.wrapping_add(1);
    }
    return 0xffff as tableid_t;
}
unsafe extern "C" fn writeLanguage(
    mut lang: *mut otl_LanguageSystem,
    mut table: *const table_OTL,
) -> *mut bk_Block {
    if lang.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let mut root: *mut bk_Block = bk_new_Block(
        p16 as ::core::ffi::c_int,
        NULL,
        b16 as ::core::ffi::c_int,
        featureIndex((*lang).requiredFeature as *const otl_Feature, table) as ::core::ffi::c_int,
        b16 as ::core::ffi::c_int,
        (*lang).features.length,
        bkover as ::core::ffi::c_int,
    );
    let mut k: tableid_t = 0 as tableid_t;
    while (k as usize) < (*lang).features.length {
        bk_push(
            root,
            b16 as ::core::ffi::c_int,
            featureIndex(
                *(*lang).features.items.offset(k as isize) as *const otl_Feature,
                table,
            ) as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        k = k.wrapping_add(1);
    }
    return root;
}
unsafe extern "C" fn writeScript(
    mut script: *mut script_stat_hash,
    mut table: *const table_OTL,
) -> *mut bk_Block {
    let mut root: *mut bk_Block = bk_new_Block(
        p16 as ::core::ffi::c_int,
        writeLanguage((*script).dl, table),
        b16 as ::core::ffi::c_int,
        (*script).lc as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*script).lc as ::core::ffi::c_int {
        let mut tag: sds = sdsnewlen(
            (**(*script).ll.offset(j as isize))
                .name
                .offset(5 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            4 as usize,
        );
        bk_push(
            root,
            b32 as ::core::ffi::c_int,
            featureNameToTag(tag),
            p16 as ::core::ffi::c_int,
            writeLanguage(*(*script).ll.offset(j as isize), table),
            bkover as ::core::ffi::c_int,
        );
        sdsfree(tag);
        j = j.wrapping_add(1);
    }
    return root;
}
unsafe extern "C" fn writeOTLScriptAndLanguages(
    mut table: *const table_OTL,
    mut _options: *const otfcc_Options,
) -> *mut bk_Block {
    let mut h: *mut script_stat_hash = ::core::ptr::null_mut::<script_stat_hash>();
    let mut j: tableid_t = 0 as tableid_t;
    while (j as usize) < (*table).languages.length {
        let mut language: *mut otl_LanguageSystem =
            *(*table).languages.items.offset(j as isize) as *mut otl_LanguageSystem;
        let mut scriptTag: sds =
            sdsnewlen((*language).name as *const ::core::ffi::c_void, 4 as usize);
        let mut isDefault: bool = strncmp(
            (*language).name.offset(5 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            b"DFLT\0" as *const u8 as *const ::core::ffi::c_char,
            4 as usize,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                (*language).name.offset(5 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
                b"dflt\0" as *const u8 as *const ::core::ffi::c_char,
                4 as usize,
            ) == 0 as ::core::ffi::c_int;
        let mut s: *mut script_stat_hash = ::core::ptr::null_mut::<script_stat_hash>();
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key: *const ::core::ffi::c_uchar = scriptTag as *const ::core::ffi::c_uchar;
        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = strlen(scriptTag as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
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
            .wrapping_add(strlen(scriptTag as *const ::core::ffi::c_char) as ::core::ffi::c_uint);
        let mut current_block_50: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 14823687005023999748;
            }
            10 => {
                current_block_50 = 14823687005023999748;
            }
            9 => {
                current_block_50 = 2331656302760911566;
            }
            8 => {
                current_block_50 = 17352930546238167574;
            }
            7 => {
                current_block_50 = 13740184123479751145;
            }
            6 => {
                current_block_50 = 14860613234516215618;
            }
            5 => {
                current_block_50 = 13669816829141938816;
            }
            4 => {
                current_block_50 = 7711255570521815756;
            }
            3 => {
                current_block_50 = 8659757378588889400;
            }
            2 => {
                current_block_50 = 16517549058459909004;
            }
            1 => {
                current_block_50 = 640113823387602610;
            }
            _ => {
                current_block_50 = 15004371738079956865;
            }
        }
        match current_block_50 {
            14823687005023999748 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 2331656302760911566;
            }
            _ => {}
        }
        match current_block_50 {
            2331656302760911566 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 17352930546238167574;
            }
            _ => {}
        }
        match current_block_50 {
            17352930546238167574 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 13740184123479751145;
            }
            _ => {}
        }
        match current_block_50 {
            13740184123479751145 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 14860613234516215618;
            }
            _ => {}
        }
        match current_block_50 {
            14860613234516215618 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 13669816829141938816;
            }
            _ => {}
        }
        match current_block_50 {
            13669816829141938816 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_50 = 7711255570521815756;
            }
            _ => {}
        }
        match current_block_50 {
            7711255570521815756 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 8659757378588889400;
            }
            _ => {}
        }
        match current_block_50 {
            8659757378588889400 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 16517549058459909004;
            }
            _ => {}
        }
        match current_block_50 {
            16517549058459909004 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 640113823387602610;
            }
            _ => {}
        }
        match current_block_50 {
            640113823387602610 => {
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
        s = ::core::ptr::null_mut::<script_stat_hash>();
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
                        as *mut ::core::ffi::c_void as *mut script_stat_hash
                        as *mut script_stat_hash;
                } else {
                    s = ::core::ptr::null_mut::<script_stat_hash>();
                }
                while !s.is_null() {
                    if (*s).hh.hashv == _hf_hashv
                        && (*s).hh.keylen
                            == strlen(scriptTag as *const ::core::ffi::c_char)
                                as ::core::ffi::c_uint
                    {
                        if memcmp(
                            (*s).hh.key,
                            scriptTag as *const ::core::ffi::c_void,
                            strlen(scriptTag as *const ::core::ffi::c_char) as ::core::ffi::c_uint
                                as usize,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*s).hh.hh_next.is_null() {
                        s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut script_stat_hash
                            as *mut script_stat_hash;
                    } else {
                        s = ::core::ptr::null_mut::<script_stat_hash>();
                    }
                }
            }
        }
        if !s.is_null() {
            if isDefault {
                (*s).dl = language;
            } else {
                (*s).lc = ((*s).lc as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
                let ref mut fresh3 = *(*s)
                    .ll
                    .offset(((*s).lc as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
                *fresh3 = language;
            }
            sdsfree(scriptTag);
        } else {
            s = __caryll_allocate_clean(
                ::core::mem::size_of::<script_stat_hash>() as usize,
                316 as ::core::ffi::c_ulong,
            ) as *mut script_stat_hash;
            (*s).tag = scriptTag;
            (*s).dl = ::core::ptr::null_mut::<otl_LanguageSystem>();
            (*s).ll = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut otl_LanguageSystem>() as usize)
                    .wrapping_mul((*table).languages.length),
                319 as ::core::ffi::c_ulong,
            ) as *mut *mut otl_LanguageSystem;
            if isDefault {
                (*s).dl = language;
                (*s).lc = 0 as u16;
            } else {
                (*s).lc = 1 as u16;
                let ref mut fresh4 = *(*s)
                    .ll
                    .offset(((*s).lc as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
                *fresh4 = language;
            }
            let mut _ha_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i_0: ::core::ffi::c_uint = 0;
            let mut _hj_j_0: ::core::ffi::c_uint = 0;
            let mut _hj_k_0: ::core::ffi::c_uint = 0;
            let mut _hj_key_0: *const ::core::ffi::c_uchar =
                (*s).tag.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_uchar;
            _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_0 = _hj_j_0;
            _hj_k_0 = strlen((*s).tag as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
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
            _ha_hashv =
                _ha_hashv.wrapping_add(
                    strlen((*s).tag as *const ::core::ffi::c_char) as ::core::ffi::c_uint
                );
            let mut current_block_183: u64;
            match _hj_k_0 {
                11 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_183 = 9646444951891790190;
                }
                10 => {
                    current_block_183 = 9646444951891790190;
                }
                9 => {
                    current_block_183 = 3334602948526645343;
                }
                8 => {
                    current_block_183 = 2212965138932432936;
                }
                7 => {
                    current_block_183 = 10274451560863855454;
                }
                6 => {
                    current_block_183 = 14269574886101392281;
                }
                5 => {
                    current_block_183 = 12841885000613847604;
                }
                4 => {
                    current_block_183 = 9712482253163437245;
                }
                3 => {
                    current_block_183 = 3099469230490028345;
                }
                2 => {
                    current_block_183 = 10402200969996848048;
                }
                1 => {
                    current_block_183 = 3799609926439758155;
                }
                _ => {
                    current_block_183 = 5832582820025303349;
                }
            }
            match current_block_183 {
                9646444951891790190 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_183 = 3334602948526645343;
                }
                _ => {}
            }
            match current_block_183 {
                3334602948526645343 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_183 = 2212965138932432936;
                }
                _ => {}
            }
            match current_block_183 {
                2212965138932432936 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_183 = 10274451560863855454;
                }
                _ => {}
            }
            match current_block_183 {
                10274451560863855454 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_183 = 14269574886101392281;
                }
                _ => {}
            }
            match current_block_183 {
                14269574886101392281 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_183 = 12841885000613847604;
                }
                _ => {}
            }
            match current_block_183 {
                12841885000613847604 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_183 = 9712482253163437245;
                }
                _ => {}
            }
            match current_block_183 {
                9712482253163437245 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_183 = 3099469230490028345;
                }
                _ => {}
            }
            match current_block_183 {
                3099469230490028345 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_183 = 10402200969996848048;
                }
                _ => {}
            }
            match current_block_183 {
                10402200969996848048 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_183 = 3799609926439758155;
                }
                _ => {}
            }
            match current_block_183 {
                3799609926439758155 => {
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
            (*s).hh.key = (*s).tag.offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            (*s).hh.keylen = strlen((*s).tag as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
            if h.is_null() {
                (*s).hh.next = NULL;
                (*s).hh.prev = NULL;
                (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                    as *mut UT_hash_table as *mut UT_hash_table;
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
                    (*(*s).hh.tbl).buckets = malloc(
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    ) as *mut UT_hash_bucket;
                    (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                    if (*(*s).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
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
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
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
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        if !h.is_null() {
            (*(*h).hh.tbl).num_items
        } else {
            0 as ::core::ffi::c_uint
        },
        bkover as ::core::ffi::c_int,
    );
    let mut s_0: *mut script_stat_hash = ::core::ptr::null_mut::<script_stat_hash>();
    let mut tmp: *mut script_stat_hash = ::core::ptr::null_mut::<script_stat_hash>();
    s_0 = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut script_stat_hash
        as *mut script_stat_hash;
    while !s_0.is_null() {
        bk_push(
            root,
            b32 as ::core::ffi::c_int,
            featureNameToTag((*s_0).tag),
            p16 as ::core::ffi::c_int,
            writeScript(s_0, table),
            bkover as ::core::ffi::c_int,
        );
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s_0).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<script_stat_hash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh5 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh5 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut script_stat_hash as *mut script_stat_hash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh6 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh6 = (*_hd_hh_del).prev;
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
        sdsfree((*s_0).tag);
        free((*s_0).ll as *mut ::core::ffi::c_void);
        (*s_0).ll = ::core::ptr::null_mut::<*mut otl_LanguageSystem>();
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<script_stat_hash>();
        s_0 = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut script_stat_hash
            as *mut script_stat_hash;
    }
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildOtl(
    mut table: *const table_OTL,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut caryll_Buffer {
    if table.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
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
    while ___loggedstep_v {
        let mut lookups: *mut bk_Block = writeOTLLookups(table, options, tag);
        let mut features: *mut bk_Block = writeOTLFeatures(table, options);
        let mut languages: *mut bk_Block = writeOTLScriptAndLanguages(table, options);
        let mut root: *mut bk_Block = bk_new_Block(
            b32 as ::core::ffi::c_int,
            0x10000 as ::core::ffi::c_int,
            p16 as ::core::ffi::c_int,
            languages,
            p16 as ::core::ffi::c_int,
            features,
            p16 as ::core::ffi::c_int,
            lookups,
            bkover as ::core::ffi::c_int,
        );
        buf = bk_build_Block(root);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    return buf;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
