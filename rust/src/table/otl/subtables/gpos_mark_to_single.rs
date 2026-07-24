use libc::{free, malloc, memcmp, memcpy, memset, qsort, strcmp, strlen};
extern "C" {
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_object_push_length(
        object: *mut json_value,
        name_length: ::core::ffi::c_uint,
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
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
    static otl_iMarkArray: __caryll_vectorinterface_otl_MarkArray;
    fn otl_anchor_absent() -> otl_Anchor;
    fn otl_read_anchor(
        data: font_file_pointer,
        tableLength: u32,
        offset: u32,
    ) -> otl_Anchor;
    fn otl_parse_anchor(v: *mut json_value) -> otl_Anchor;
    fn bkFromAnchor(a: otl_Anchor) -> *mut bk_Block;
    fn otl_readMarkArray(
        array: *mut otl_MarkArray,
        cov: *mut otl_Coverage,
        data: font_file_pointer,
        tableLength: u32,
        offset: u32,
    );
    fn otl_parseMarkArray(
        _marks: *mut json_value,
        array: *mut otl_MarkArray,
        h: *mut *mut otl_ClassnameHash,
        options: *const otfcc_Options,
    );
}

use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage, otl_Coverage};
use crate::support::handle::{handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle, HANDLE_STATE_EMPTY};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::logger::{log_type_warning, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphclass_t, glyphid_t, pos_t, tableid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_object, json_pre_serialized, json_type, json_value};
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_serialize_opts {
    pub mode: ::core::ffi::c_int,
    pub opts: ::core::ffi::c_int,
    pub indent_size: ::core::ffi::c_int,
}
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
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed_4 = 10;
pub const log_vl_info: C2RustUnnamed_4 = 5;
pub const log_vl_notice: C2RustUnnamed_4 = 2;
pub const log_vl_important: C2RustUnnamed_4 = 1;
pub const log_vl_critical: C2RustUnnamed_4 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
    pub c2rust_unnamed: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub struct __caryll_vectorinterface_otl_MarkArray {
    pub init: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_MarkArray, *const otl_MarkArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_MarkArray, *mut otl_MarkArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkArray) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_MarkArray>,
    pub free: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_MarkArray, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_MarkArray, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_MarkArray>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_MarkArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> otl_MarkRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_MarkArray, usize) -> ()>,
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
pub struct __caryll_vectorinterface_otl_BaseArray {
    pub init: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_BaseArray, *const otl_BaseArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_BaseArray, *mut otl_BaseArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_BaseArray, otl_BaseArray) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_BaseArray, otl_BaseArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_BaseArray>,
    pub free: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_BaseArray, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_BaseArray, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_BaseArray>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_BaseArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_BaseArray, otl_BaseRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> otl_BaseRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_BaseArray, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_BaseArray,
            Option<unsafe extern "C" fn(*const otl_BaseRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_BaseArray,
            Option<
                unsafe extern "C" fn(
                    *const otl_BaseRecord,
                    *const otl_BaseRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_BaseRecord {
    pub init: Option<unsafe extern "C" fn(*mut otl_BaseRecord) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_BaseRecord, *const otl_BaseRecord) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_BaseRecord, *mut otl_BaseRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_BaseRecord) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_BaseRecord, otl_BaseRecord) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_BaseRecord, otl_BaseRecord) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_subtable_gpos_markToSingle {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_markToSingle,
            *const subtable_gpos_markToSingle,
        ) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_markToSingle,
            *mut subtable_gpos_markToSingle,
        ) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
    pub replace: Option<
        unsafe extern "C" fn(*mut subtable_gpos_markToSingle, subtable_gpos_markToSingle) -> (),
    >,
    pub copyReplace: Option<
        unsafe extern "C" fn(*mut subtable_gpos_markToSingle, subtable_gpos_markToSingle) -> (),
    >,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_markToSingle>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
}
pub type otl_BuildHeuristics = ::core::ffi::c_uint;
pub const OTL_BH_GSUB_VERT: otl_BuildHeuristics = 1;
pub const OTL_BH_NORMAL: otl_BuildHeuristics = 0;
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
unsafe extern "C" fn deleteBaseArrayItem(mut entry: *mut otl_BaseRecord) {
    otfcc_Handle_dispose(&raw mut (*entry).glyph);
    free((*entry).anchors as *mut ::core::ffi::c_void);
    (*entry).anchors = ::core::ptr::null_mut::<otl_Anchor>();
}
static mut ba_typeinfo: __caryll_elementinterface_otl_BaseRecord = {
    __caryll_elementinterface_otl_BaseRecord {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(deleteBaseArrayItem as unsafe extern "C" fn(*mut otl_BaseRecord) -> ()),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn otl_BaseArray_createN(mut n: usize) -> *mut otl_BaseArray {
    let mut t: *mut otl_BaseArray =
        malloc(::core::mem::size_of::<otl_BaseArray>() as usize) as *mut otl_BaseArray;
    otl_BaseArray_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_BaseArray_push(arr: *mut otl_BaseArray, elem: otl_BaseRecord) {
    cvec_push(otl_BaseArray_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_BaseArray_grow(arr: *mut otl_BaseArray) {
    cvec_grow(otl_BaseArray_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_BaseArray_growTo(arr: *mut otl_BaseArray, target: usize) {
    cvec_grow_to(otl_BaseArray_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_BaseArray_pop(arr: *mut otl_BaseArray) -> otl_BaseRecord {
    cvec_pop(otl_BaseArray_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_BaseArray_copyReplace(mut dst: *mut otl_BaseArray, src: otl_BaseArray) {
    otl_BaseArray_dispose(dst);
    otl_BaseArray_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_BaseArray_copy(
    mut dst: *mut otl_BaseArray,
    mut src: *const otl_BaseArray,
) {
    otl_BaseArray_init(dst);
    otl_BaseArray_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if ba_typeinfo.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            ba_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_BaseRecord,
                (*src).items.offset(j as isize) as *mut otl_BaseRecord as *const otl_BaseRecord,
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
unsafe extern "C" fn otl_BaseArray_dispose(mut arr: *mut otl_BaseArray) {
    if arr.is_null() {
        return;
    }
    if ba_typeinfo.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            ba_typeinfo.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_BaseRecord,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_BaseRecord>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_BaseArray_replace(mut dst: *mut otl_BaseArray, src: otl_BaseArray) {
    otl_BaseArray_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_BaseArray>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_BaseArray_initCapN(mut arr: *mut otl_BaseArray, mut n: usize) {
    otl_BaseArray_init(arr);
    otl_BaseArray_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_BaseArray_growToN(arr: *mut otl_BaseArray, target: usize) {
    cvec_grow_to_n(otl_BaseArray_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_BaseArray_initN(mut arr: *mut otl_BaseArray, mut n: usize) {
    otl_BaseArray_init(arr);
    otl_BaseArray_growToN(arr, n);
    otl_BaseArray_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_BaseArray_free(mut x: *mut otl_BaseArray) {
    if x.is_null() {
        return;
    }
    otl_BaseArray_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe fn otl_BaseArray_as_cvec(arr: *mut otl_BaseArray) -> *mut CVecRaw<otl_BaseRecord> {
    arr as *mut CVecRaw<otl_BaseRecord>
}
#[inline]
unsafe extern "C" fn otl_BaseArray_init(arr: *mut otl_BaseArray) {
    cvec_init(otl_BaseArray_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_BaseArray_create() -> *mut otl_BaseArray {
    let mut x: *mut otl_BaseArray =
        malloc(::core::mem::size_of::<otl_BaseArray>() as usize) as *mut otl_BaseArray;
    otl_BaseArray_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_BaseArray_filterEnv(
    mut arr: *mut otl_BaseArray,
    mut fn_0: Option<unsafe extern "C" fn(*const otl_BaseRecord, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_BaseRecord,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if ba_typeinfo.dispose.is_some() {
                ba_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_BaseRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[no_mangle]
pub static mut otl_iBaseArray: __caryll_vectorinterface_otl_BaseArray = {
    __caryll_vectorinterface_otl_BaseArray {
        init: Some(otl_BaseArray_init as unsafe extern "C" fn(*mut otl_BaseArray) -> ()),
        copy: Some(
            otl_BaseArray_copy
                as unsafe extern "C" fn(*mut otl_BaseArray, *const otl_BaseArray) -> (),
        ),
        move_0: Some(
            otl_BaseArray_move
                as unsafe extern "C" fn(*mut otl_BaseArray, *mut otl_BaseArray) -> (),
        ),
        dispose: Some(otl_BaseArray_dispose as unsafe extern "C" fn(*mut otl_BaseArray) -> ()),
        replace: Some(
            otl_BaseArray_replace as unsafe extern "C" fn(*mut otl_BaseArray, otl_BaseArray) -> (),
        ),
        copyReplace: Some(
            otl_BaseArray_copyReplace
                as unsafe extern "C" fn(*mut otl_BaseArray, otl_BaseArray) -> (),
        ),
        create: Some(otl_BaseArray_create),
        free: Some(otl_BaseArray_free as unsafe extern "C" fn(*mut otl_BaseArray) -> ()),
        initN: Some(otl_BaseArray_initN as unsafe extern "C" fn(*mut otl_BaseArray, usize) -> ()),
        initCapN: Some(
            otl_BaseArray_initCapN as unsafe extern "C" fn(*mut otl_BaseArray, usize) -> (),
        ),
        createN: Some(otl_BaseArray_createN as unsafe extern "C" fn(usize) -> *mut otl_BaseArray),
        fill: Some(otl_BaseArray_fill as unsafe extern "C" fn(*mut otl_BaseArray, usize) -> ()),
        clear: Some(otl_BaseArray_dispose as unsafe extern "C" fn(*mut otl_BaseArray) -> ()),
        push: Some(
            otl_BaseArray_push as unsafe extern "C" fn(*mut otl_BaseArray, otl_BaseRecord) -> (),
        ),
        shrinkToFit: Some(
            otl_BaseArray_shrinkToFit as unsafe extern "C" fn(*mut otl_BaseArray) -> (),
        ),
        pop: Some(otl_BaseArray_pop as unsafe extern "C" fn(*mut otl_BaseArray) -> otl_BaseRecord),
        disposeItem: Some(
            otl_BaseArray_disposeItem as unsafe extern "C" fn(*mut otl_BaseArray, usize) -> (),
        ),
        filterEnv: Some(
            otl_BaseArray_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_BaseArray,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_BaseRecord,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_BaseArray_sort
                as unsafe extern "C" fn(
                    *mut otl_BaseArray,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_BaseRecord,
                            *const otl_BaseRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_BaseArray_shrinkToFit(mut arr: *mut otl_BaseArray) {
    otl_BaseArray_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_BaseArray_move(dst: *mut otl_BaseArray, src: *mut otl_BaseArray) {
    cvec_move(otl_BaseArray_as_cvec(dst), otl_BaseArray_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_BaseArray_resizeTo(arr: *mut otl_BaseArray, target: usize) {
    cvec_resize_to(otl_BaseArray_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_BaseArray_disposeItem(mut arr: *mut otl_BaseArray, mut n: usize) {
    if ba_typeinfo.dispose.is_some() {
        ba_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_BaseRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_BaseArray_sort(
    mut arr: *mut otl_BaseArray,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_BaseRecord, *const otl_BaseRecord) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_BaseRecord>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_BaseRecord,
                    *const otl_BaseRecord,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_BaseArray_fill(mut arr: *mut otl_BaseArray, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_BaseRecord = otl_BaseRecord {
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            anchors: ::core::ptr::null_mut::<otl_Anchor>(),
        };
        if ba_typeinfo.init.is_some() {
            ba_typeinfo.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_BaseRecord>() as usize,
            );
        }
        otl_BaseArray_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn initMarkToSingle(mut subtable: *mut subtable_gpos_markToSingle) {
    otl_iMarkArray.init.expect("non-null function pointer")(&raw mut (*subtable).markArray);
    otl_iBaseArray.init.expect("non-null function pointer")(&raw mut (*subtable).baseArray);
}
#[inline]
unsafe extern "C" fn disposeMarkToSingle(mut subtable: *mut subtable_gpos_markToSingle) {
    otl_iMarkArray.dispose.expect("non-null function pointer")(&raw mut (*subtable).markArray);
    otl_iBaseArray.dispose.expect("non-null function pointer")(&raw mut (*subtable).baseArray);
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToSingle_copyReplace(
    mut dst: *mut subtable_gpos_markToSingle,
    src: subtable_gpos_markToSingle,
) {
    subtable_gpos_markToSingle_dispose(dst);
    subtable_gpos_markToSingle_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToSingle_copy(
    mut dst: *mut subtable_gpos_markToSingle,
    mut src: *const subtable_gpos_markToSingle,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_markToSingle>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToSingle_replace(
    mut dst: *mut subtable_gpos_markToSingle,
    src: subtable_gpos_markToSingle,
) {
    subtable_gpos_markToSingle_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_markToSingle>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToSingle_move(
    mut dst: *mut subtable_gpos_markToSingle,
    mut src: *mut subtable_gpos_markToSingle,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_markToSingle>() as usize,
    );
    subtable_gpos_markToSingle_init(src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToSingle_free(mut x: *mut subtable_gpos_markToSingle) {
    if x.is_null() {
        return;
    }
    subtable_gpos_markToSingle_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToSingle_init(mut x: *mut subtable_gpos_markToSingle) {
    initMarkToSingle(x);
}
#[inline]
unsafe extern "C" fn subtable_gpos_markToSingle_dispose(mut x: *mut subtable_gpos_markToSingle) {
    disposeMarkToSingle(x);
}
#[no_mangle]
pub static mut iSubtable_gpos_markToSingle: __caryll_elementinterface_subtable_gpos_markToSingle = {
    __caryll_elementinterface_subtable_gpos_markToSingle {
        init: Some(
            subtable_gpos_markToSingle_init
                as unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> (),
        ),
        copy: Some(
            subtable_gpos_markToSingle_copy
                as unsafe extern "C" fn(
                    *mut subtable_gpos_markToSingle,
                    *const subtable_gpos_markToSingle,
                ) -> (),
        ),
        move_0: Some(
            subtable_gpos_markToSingle_move
                as unsafe extern "C" fn(
                    *mut subtable_gpos_markToSingle,
                    *mut subtable_gpos_markToSingle,
                ) -> (),
        ),
        dispose: Some(
            subtable_gpos_markToSingle_dispose
                as unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> (),
        ),
        replace: Some(
            subtable_gpos_markToSingle_replace
                as unsafe extern "C" fn(
                    *mut subtable_gpos_markToSingle,
                    subtable_gpos_markToSingle,
                ) -> (),
        ),
        copyReplace: Some(
            subtable_gpos_markToSingle_copyReplace
                as unsafe extern "C" fn(
                    *mut subtable_gpos_markToSingle,
                    subtable_gpos_markToSingle,
                ) -> (),
        ),
        create: Some(subtable_gpos_markToSingle_create),
        free: Some(
            subtable_gpos_markToSingle_free
                as unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_markToSingle_create() -> *mut subtable_gpos_markToSingle {
    let mut x: *mut subtable_gpos_markToSingle =
        malloc(::core::mem::size_of::<subtable_gpos_markToSingle>() as usize)
            as *mut subtable_gpos_markToSingle;
    subtable_gpos_markToSingle_init(x);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gpos_markToSingle(
    data: font_file_pointer,
    mut tableLength: u32,
    mut subtableOffset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut markArrayOffset: u32 = 0;
    let mut baseArrayOffset: u32 = 0;
    let mut _offset: u32 = 0;
    let mut subtable: *mut subtable_gpos_markToSingle =
        (
            iSubtable_gpos_markToSingle
                .create
                .expect("non-null function pointer"))();
    let mut marks: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    let mut bases: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    if !(tableLength < subtableOffset.wrapping_add(12 as u32)) {
        marks = readCoverage(
            data as *const u8,
            tableLength,
            subtableOffset.wrapping_add(read_16u(
                data.offset(subtableOffset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        bases = readCoverage(
            data as *const u8,
            tableLength,
            subtableOffset.wrapping_add(read_16u(
                data.offset(subtableOffset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !(marks.is_null()
            || (*marks).numGlyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || bases.is_null()
            || (*bases).numGlyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            (*subtable).classCount = read_16u(
                data.offset(subtableOffset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            ) as glyphclass_t;
            markArrayOffset = subtableOffset.wrapping_add(read_16u(
                data.offset(subtableOffset as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32);
            otl_readMarkArray(
                &raw mut (*subtable).markArray,
                marks,
                data,
                tableLength,
                markArrayOffset,
            );
            baseArrayOffset = subtableOffset.wrapping_add(read_16u(
                data.offset(subtableOffset as isize)
                    .offset(10 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32);
            if !(tableLength
                < baseArrayOffset.wrapping_add(2 as u32).wrapping_add(
                    (2 as ::core::ffi::c_int
                        * (*bases).numGlyphs as ::core::ffi::c_int
                        * (*subtable).classCount as ::core::ffi::c_int)
                        as u32,
                ))
            {
                if !(read_16u(data.offset(baseArrayOffset as isize) as *const u8)
                    as ::core::ffi::c_int
                    != (*bases).numGlyphs as ::core::ffi::c_int)
                {
                    _offset = baseArrayOffset.wrapping_add(2 as u32);
                    let mut j: glyphid_t = 0 as glyphid_t;
                    while (j as ::core::ffi::c_int) < (*bases).numGlyphs as ::core::ffi::c_int {
                        let mut baseAnchors: *mut otl_Anchor =
                            ::core::ptr::null_mut::<otl_Anchor>();
                        baseAnchors = __caryll_allocate_clean(
                            (::core::mem::size_of::<otl_Anchor>() as usize)
                                .wrapping_mul((*subtable).classCount as usize),
                            49 as ::core::ffi::c_ulong,
                        ) as *mut otl_Anchor;
                        let mut k: glyphclass_t = 0 as glyphclass_t;
                        while (k as ::core::ffi::c_int)
                            < (*subtable).classCount as ::core::ffi::c_int
                        {
                            if read_16u(data.offset(_offset as isize) as *const u8) != 0 {
                                *baseAnchors.offset(k as isize) = otl_read_anchor(
                                    data,
                                    tableLength,
                                    baseArrayOffset.wrapping_add(read_16u(
                                        data.offset(_offset as isize) as *const u8,
                                    )
                                        as u32),
                                );
                            } else {
                                *baseAnchors.offset(k as isize) = otl_anchor_absent();
                            }
                            _offset = _offset.wrapping_add(2 as u32);
                            k = k.wrapping_add(1);
                        }
                        otl_iBaseArray.push.expect("non-null function pointer")(
                            &raw mut (*subtable).baseArray,
                            otl_BaseRecord {
                                glyph: otfcc_Handle_dup(
                                    *(*bases).glyphs.offset(j as isize) as otfcc_Handle,
                                ) as otfcc_GlyphHandle,
                                anchors: baseAnchors,
                            },
                        );
                        j = j.wrapping_add(1);
                    }
                    if !marks.is_null() {
                        otl_Coverage_free(marks);
                    }
                    if !bases.is_null() {
                        otl_Coverage_free(bases);
                    }
                    return subtable as *mut otl_Subtable;
                }
            }
        }
    }
    iSubtable_gpos_markToSingle
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_dump_markToSingle(
    mut st: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gpos_markToSingle = &raw const (*st).gpos_markToSingle;
    let mut _subtable: *mut json_value = json_object_new(3 as usize);
    let mut _marks: *mut json_value = json_object_new((*subtable).markArray.length);
    let mut _bases: *mut json_value = json_object_new((*subtable).baseArray.length);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*subtable).markArray.length {
        let mut _mark: *mut json_value = json_object_new(3 as usize);
        let mut markClassName: sds = sdscatfmt(
            sdsempty(),
            b"anchor%i\0" as *const u8 as *const ::core::ffi::c_char,
            (*(*subtable).markArray.items.offset(j as isize)).markClass as ::core::ffi::c_int,
        );
        json_object_push(
            _mark,
            b"class\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new_length(
                sdslen(markClassName) as ::core::ffi::c_uint,
                markClassName as *const ::core::ffi::c_char,
            ),
        );
        sdsfree(markClassName);
        json_object_push(
            _mark,
            b"x\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*(*subtable).markArray.items.offset(j as isize)).anchor.x as i64),
        );
        json_object_push(
            _mark,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*(*subtable).markArray.items.offset(j as isize)).anchor.y as i64),
        );
        json_object_push(
            _marks,
            (*(*subtable).markArray.items.offset(j as isize)).glyph.name
                as *const ::core::ffi::c_char,
            preserialize(_mark),
        );
        j = j.wrapping_add(1);
    }
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as usize) < (*subtable).baseArray.length {
        let mut _base: *mut json_value = json_object_new((*subtable).classCount as usize);
        let mut k: glyphclass_t = 0 as glyphclass_t;
        while (k as ::core::ffi::c_int) < (*subtable).classCount as ::core::ffi::c_int {
            if (*(*(*subtable).baseArray.items.offset(j_0 as isize))
                .anchors
                .offset(k as isize))
            .present
            {
                let mut _anchor: *mut json_value = json_object_new(2 as usize);
                json_object_push(
                    _anchor,
                    b"x\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (*(*(*subtable).baseArray.items.offset(j_0 as isize))
                            .anchors
                            .offset(k as isize))
                        .x as i64,
                    ),
                );
                json_object_push(
                    _anchor,
                    b"y\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new(
                        (*(*(*subtable).baseArray.items.offset(j_0 as isize))
                            .anchors
                            .offset(k as isize))
                        .y as i64,
                    ),
                );
                let mut markClassName_0: sds = sdscatfmt(
                    sdsempty(),
                    b"anchor%i\0" as *const u8 as *const ::core::ffi::c_char,
                    k as ::core::ffi::c_int,
                );
                json_object_push_length(
                    _base,
                    sdslen(markClassName_0) as ::core::ffi::c_uint,
                    markClassName_0 as *const ::core::ffi::c_char,
                    _anchor,
                );
                sdsfree(markClassName_0);
            }
            k = k.wrapping_add(1);
        }
        json_object_push(
            _bases,
            (*(*subtable).baseArray.items.offset(j_0 as isize))
                .glyph
                .name as *const ::core::ffi::c_char,
            preserialize(_base),
        );
        j_0 = j_0.wrapping_add(1);
    }
    json_object_push(
        _subtable,
        b"marks\0" as *const u8 as *const ::core::ffi::c_char,
        _marks,
    );
    json_object_push(
        _subtable,
        b"bases\0" as *const u8 as *const ::core::ffi::c_char,
        _bases,
    );
    return _subtable;
}
unsafe extern "C" fn parseBases(
    mut _bases: *mut json_value,
    mut subtable: *mut subtable_gpos_markToSingle,
    mut h: *mut *mut otl_ClassnameHash,
    mut options: *const otfcc_Options,
) {
    let mut classCount: glyphclass_t = (if !(*h).is_null() {
        (*(**h).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) as glyphclass_t;
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*_bases).u.object.length {
        let mut gname: *mut ::core::ffi::c_char =
            (*(*_bases).u.object.values.offset(j as isize)).name;
        let mut base: otl_BaseRecord = otl_BaseRecord {
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            anchors: ::core::ptr::null_mut::<otl_Anchor>(),
        };
        base.glyph = handle_fromName(sdsnewlen(
            gname as *const ::core::ffi::c_void,
            (*(*_bases).u.object.values.offset(j as isize)).name_length as usize,
        )) as otfcc_GlyphHandle;
        base.anchors = __caryll_allocate_clean(
            (::core::mem::size_of::<otl_Anchor>() as usize).wrapping_mul(classCount as usize),
            116 as ::core::ffi::c_ulong,
        ) as *mut otl_Anchor;
        let mut k: glyphclass_t = 0 as glyphclass_t;
        while (k as ::core::ffi::c_int) < classCount as ::core::ffi::c_int {
            *base.anchors.offset(k as isize) = otl_anchor_absent();
            k = k.wrapping_add(1);
        }
        let mut baseRecord: *mut json_value =
            (*(*_bases).u.object.values.offset(j as isize)).value as *mut json_value;
        if baseRecord.is_null()
            || (*baseRecord).type_0 as ::core::ffi::c_uint
                != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            otl_iBaseArray.push.expect("non-null function pointer")(
                &raw mut (*subtable).baseArray,
                base,
            );
        } else {
            let mut k_0: glyphclass_t = 0 as glyphclass_t;
            while (k_0 as ::core::ffi::c_uint) < (*baseRecord).u.object.length {
                let mut className: sds = sdsnewlen(
                    (*(*baseRecord).u.object.values.offset(k_0 as isize)).name
                        as *const ::core::ffi::c_void,
                    (*(*baseRecord).u.object.values.offset(k_0 as isize)).name_length as usize,
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
                let mut current_block_56: u64;
                match _hj_k {
                    11 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_56 = 14536411282452634839;
                    }
                    10 => {
                        current_block_56 = 14536411282452634839;
                    }
                    9 => {
                        current_block_56 = 9913348930486913067;
                    }
                    8 => {
                        current_block_56 = 1505195771936801158;
                    }
                    7 => {
                        current_block_56 = 15021600489117130768;
                    }
                    6 => {
                        current_block_56 = 8233865231112875104;
                    }
                    5 => {
                        current_block_56 = 3771526520438017190;
                    }
                    4 => {
                        current_block_56 = 6788034837040873263;
                    }
                    3 => {
                        current_block_56 = 17257476062468164659;
                    }
                    2 => {
                        current_block_56 = 16976244951184097103;
                    }
                    1 => {
                        current_block_56 = 14519719227392997025;
                    }
                    _ => {
                        current_block_56 = 8151474771948790331;
                    }
                }
                match current_block_56 {
                    14536411282452634839 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_56 = 9913348930486913067;
                    }
                    _ => {}
                }
                match current_block_56 {
                    9913348930486913067 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_56 = 1505195771936801158;
                    }
                    _ => {}
                }
                match current_block_56 {
                    1505195771936801158 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_56 = 15021600489117130768;
                    }
                    _ => {}
                }
                match current_block_56 {
                    15021600489117130768 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_56 = 8233865231112875104;
                    }
                    _ => {}
                }
                match current_block_56 {
                    8233865231112875104 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_56 = 3771526520438017190;
                    }
                    _ => {}
                }
                match current_block_56 {
                    3771526520438017190 => {
                        _hj_j = _hj_j
                            .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_56 = 6788034837040873263;
                    }
                    _ => {}
                }
                match current_block_56 {
                    6788034837040873263 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_56 = 17257476062468164659;
                    }
                    _ => {}
                }
                match current_block_56 {
                    17257476062468164659 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_56 = 16976244951184097103;
                    }
                    _ => {}
                }
                match current_block_56 {
                    16976244951184097103 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_56 = 14519719227392997025;
                    }
                    _ => {}
                }
                match current_block_56 {
                    14519719227392997025 => {
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
                                        as usize,
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
                            b"[OTFCC-fea] Invalid anchor class name <%s> for /%s. This base anchor is ignored.\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            className,
                            gname,
                        ),
                    );
                } else {
                    *base.anchors.offset((*s).classID as isize) = otl_parse_anchor(
                        (*(*baseRecord).u.object.values.offset(k_0 as isize)).value
                            as *mut json_value,
                    );
                }
                sdsfree(className);
                k_0 = k_0.wrapping_add(1);
            }
            otl_iBaseArray.push.expect("non-null function pointer")(
                &raw mut (*subtable).baseArray,
                base,
            );
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_parse_markToSingle(
    mut _subtable: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut _marks: *mut json_value = json_obj_get_type(
        _subtable,
        b"marks\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    let mut _bases: *mut json_value = json_obj_get_type(
        _subtable,
        b"bases\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if _marks.is_null() || _bases.is_null() {
        return ::core::ptr::null_mut::<otl_Subtable>();
    }
    let mut st: *mut subtable_gpos_markToSingle =
        (
            iSubtable_gpos_markToSingle
                .create
                .expect("non-null function pointer"))();
    let mut h: *mut otl_ClassnameHash = ::core::ptr::null_mut::<otl_ClassnameHash>();
    otl_parseMarkArray(_marks, &raw mut (*st).markArray, &raw mut h, options);
    (*st).classCount = (if !h.is_null() {
        (*(*h).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) as glyphclass_t;
    parseBases(_bases, st, &raw mut h, options);
    let mut s: *mut otl_ClassnameHash = ::core::ptr::null_mut::<otl_ClassnameHash>();
    let mut tmp: *mut otl_ClassnameHash = ::core::ptr::null_mut::<otl_ClassnameHash>();
    s = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut otl_ClassnameHash
        as *mut otl_ClassnameHash;
    while !s.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<otl_ClassnameHash>();
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
                h = (*_hd_hh_del).next as *mut otl_ClassnameHash as *mut otl_ClassnameHash;
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
        sdsfree((*s).className);
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<otl_ClassnameHash>();
        s = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut otl_ClassnameHash
            as *mut otl_ClassnameHash;
    }
    return st as *mut otl_Subtable;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gpos_markToSingle(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gpos_markToSingle = &raw const (*_subtable).gpos_markToSingle;
    let mut marks: *mut otl_Coverage = otl_Coverage_create();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*subtable).markArray.length {
        pushToCoverage(
            marks,
            otfcc_Handle_dup(
                (*(*subtable).markArray.items.offset(j as isize)).glyph as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
        j = j.wrapping_add(1);
    }
    let mut bases: *mut otl_Coverage = otl_Coverage_create();
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as usize) < (*subtable).baseArray.length {
        pushToCoverage(
            bases,
            otfcc_Handle_dup(
                (*(*subtable).baseArray.items.offset(j_0 as isize)).glyph as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            marks,
        )),
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            bases,
        )),
        b16 as ::core::ffi::c_int,
        (*subtable).classCount as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut markArray: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        (*subtable).markArray.length,
        bkover as ::core::ffi::c_int,
    );
    let mut j_1: glyphid_t = 0 as glyphid_t;
    while (j_1 as usize) < (*subtable).markArray.length {
        bk_push(
            markArray,
            b16 as ::core::ffi::c_int,
            (*(*subtable).markArray.items.offset(j_1 as isize)).markClass as ::core::ffi::c_int,
            p16 as ::core::ffi::c_int,
            bkFromAnchor((*(*subtable).markArray.items.offset(j_1 as isize)).anchor),
            bkover as ::core::ffi::c_int,
        );
        j_1 = j_1.wrapping_add(1);
    }
    let mut baseArray: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        (*subtable).baseArray.length,
        bkover as ::core::ffi::c_int,
    );
    let mut j_2: glyphid_t = 0 as glyphid_t;
    while (j_2 as usize) < (*subtable).baseArray.length {
        let mut k: glyphclass_t = 0 as glyphclass_t;
        while (k as ::core::ffi::c_int) < (*subtable).classCount as ::core::ffi::c_int {
            bk_push(
                baseArray,
                p16 as ::core::ffi::c_int,
                bkFromAnchor(
                    *(*(*subtable).baseArray.items.offset(j_2 as isize))
                        .anchors
                        .offset(k as isize),
                ),
                bkover as ::core::ffi::c_int,
            );
            k = k.wrapping_add(1);
        }
        j_2 = j_2.wrapping_add(1);
    }
    bk_push(
        root,
        p16 as ::core::ffi::c_int,
        markArray,
        p16 as ::core::ffi::c_int,
        baseArray,
        bkover as ::core::ffi::c_int,
    );
    otl_Coverage_free(marks);
    otl_Coverage_free(bases);
    return bk_build_Block(root);
}
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
