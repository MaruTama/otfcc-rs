use libc::{free, malloc, memcpy, strcmp};
extern "C" {
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}

use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage_free, readCoverage, otl_Coverage};
use crate::support::handle::{handle_fromIndex, otfcc_GlyphHandle, otfcc_LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
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
pub type font_file_pointer = *mut u8;
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
pub struct __caryll_elementinterface_subtable_gsub_reverse {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gsub_reverse) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut subtable_gsub_reverse, *const subtable_gsub_reverse) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gsub_reverse, *mut subtable_gsub_reverse) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gsub_reverse) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_reverse, subtable_gsub_reverse) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_reverse, subtable_gsub_reverse) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gsub_reverse>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gsub_reverse) -> ()>,
}
pub type otl_BuildHeuristics = ::core::ffi::c_uint;
pub const OTL_BH_GSUB_VERT: otl_BuildHeuristics = 1;
pub const OTL_BH_NORMAL: otl_BuildHeuristics = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
    return fallback;
}
#[inline]
unsafe extern "C" fn initGsubReverse(mut subtable: *mut subtable_gsub_reverse) {
    (*subtable).match_0 = ::core::ptr::null_mut::<*mut otl_Coverage>();
    (*subtable).to = ::core::ptr::null_mut::<otl_Coverage>();
}
#[inline]
unsafe extern "C" fn disposeGsubReverse(mut subtable: *mut subtable_gsub_reverse) {
    if !(*subtable).match_0.is_null() {
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
            otl_Coverage_free(
                *(*subtable).match_0.offset(j as isize),
            );
            j = j.wrapping_add(1);
        }
    }
    if !(*subtable).to.is_null() {
        otl_Coverage_free((*subtable).to);
    }
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_dispose(mut x: *mut subtable_gsub_reverse) {
    disposeGsubReverse(x);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_free(mut x: *mut subtable_gsub_reverse) {
    if x.is_null() {
        return;
    }
    subtable_gsub_reverse_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub static mut iSubtable_gsub_reverse: __caryll_elementinterface_subtable_gsub_reverse = {
    __caryll_elementinterface_subtable_gsub_reverse {
        init: Some(
            subtable_gsub_reverse_init as unsafe extern "C" fn(*mut subtable_gsub_reverse) -> (),
        ),
        copy: Some(
            subtable_gsub_reverse_copy
                as unsafe extern "C" fn(
                    *mut subtable_gsub_reverse,
                    *const subtable_gsub_reverse,
                ) -> (),
        ),
        move_0: Some(
            subtable_gsub_reverse_move
                as unsafe extern "C" fn(
                    *mut subtable_gsub_reverse,
                    *mut subtable_gsub_reverse,
                ) -> (),
        ),
        dispose: Some(
            subtable_gsub_reverse_dispose as unsafe extern "C" fn(*mut subtable_gsub_reverse) -> (),
        ),
        replace: Some(
            subtable_gsub_reverse_replace
                as unsafe extern "C" fn(*mut subtable_gsub_reverse, subtable_gsub_reverse) -> (),
        ),
        copyReplace: Some(
            subtable_gsub_reverse_copyReplace
                as unsafe extern "C" fn(*mut subtable_gsub_reverse, subtable_gsub_reverse) -> (),
        ),
        create: Some(subtable_gsub_reverse_create),
        free: Some(
            subtable_gsub_reverse_free as unsafe extern "C" fn(*mut subtable_gsub_reverse) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_create() -> *mut subtable_gsub_reverse {
    let mut x: *mut subtable_gsub_reverse =
        malloc(::core::mem::size_of::<subtable_gsub_reverse>() as usize)
            as *mut subtable_gsub_reverse;
    subtable_gsub_reverse_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_init(mut x: *mut subtable_gsub_reverse) {
    initGsubReverse(x);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_copy(
    mut dst: *mut subtable_gsub_reverse,
    mut src: *const subtable_gsub_reverse,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_reverse>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_copyReplace(
    mut dst: *mut subtable_gsub_reverse,
    src: subtable_gsub_reverse,
) {
    subtable_gsub_reverse_dispose(dst);
    subtable_gsub_reverse_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_move(
    mut dst: *mut subtable_gsub_reverse,
    mut src: *mut subtable_gsub_reverse,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_reverse>() as usize,
    );
    subtable_gsub_reverse_init(src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_reverse_replace(
    mut dst: *mut subtable_gsub_reverse,
    src: subtable_gsub_reverse,
) {
    subtable_gsub_reverse_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_reverse>() as usize,
    );
}
unsafe extern "C" fn reverseBacktracks(
    mut match_0: *mut *mut otl_Coverage,
    mut inputIndex: tableid_t,
) {
    if inputIndex as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut start: tableid_t = 0 as tableid_t;
        let mut end: tableid_t =
            (inputIndex as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as tableid_t;
        while end as ::core::ffi::c_int > start as ::core::ffi::c_int {
            let mut tmp: *mut otl_Coverage = *match_0.offset(start as isize);
            let ref mut fresh3 = *match_0.offset(start as isize);
            *fresh3 = *match_0.offset(end as isize);
            let ref mut fresh4 = *match_0.offset(end as isize);
            *fresh4 = tmp;
            end = end.wrapping_sub(1);
            start = start.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gsub_reverse(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut nBacktrack: tableid_t = 0;
    let mut nForward: tableid_t = 0;
    let mut nReplacement: tableid_t = 0;
    let mut subtable: *mut subtable_gsub_reverse =
        (
            iSubtable_gsub_reverse
                .create
                .expect("non-null function pointer"))();
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        nBacktrack = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as tableid_t;
        if !(tableLength
            < offset.wrapping_add(6 as u32).wrapping_add(
                (nBacktrack as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
            ))
        {
            nForward = read_16u(
                data.offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset((nBacktrack as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                    as *const u8,
            ) as tableid_t;
            if !(tableLength
                < offset.wrapping_add(8 as u32).wrapping_add(
                    ((nBacktrack as ::core::ffi::c_int + nForward as ::core::ffi::c_int)
                        * 2 as ::core::ffi::c_int) as u32,
                ))
            {
                nReplacement = read_16u(
                    data.offset(offset as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset(
                            ((nBacktrack as ::core::ffi::c_int + nForward as ::core::ffi::c_int)
                                * 2 as ::core::ffi::c_int) as isize,
                        ) as *const u8,
                ) as tableid_t;
                if !(tableLength
                    < offset.wrapping_add(10 as u32).wrapping_add(
                        ((nBacktrack as ::core::ffi::c_int
                            + nForward as ::core::ffi::c_int
                            + nReplacement as ::core::ffi::c_int)
                            * 2 as ::core::ffi::c_int) as u32,
                    ))
                {
                    (*subtable).matchCount = (nBacktrack as ::core::ffi::c_int
                        + nForward as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as tableid_t;
                    (*subtable).match_0 = __caryll_allocate_clean(
                        (::core::mem::size_of::<*mut otl_Coverage>() as usize)
                            .wrapping_mul((*subtable).matchCount as usize),
                        47 as ::core::ffi::c_ulong,
                    ) as *mut *mut otl_Coverage;
                    (*subtable).inputIndex = nBacktrack;
                    let mut j: tableid_t = 0 as tableid_t;
                    while (j as ::core::ffi::c_int) < nBacktrack as ::core::ffi::c_int {
                        let mut covOffset: u32 = offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        let ref mut fresh0 = *(*subtable).match_0.offset(j as isize);
                        *fresh0 = readCoverage(
                            data as *const u8,
                            tableLength,
                            covOffset,
                        );
                        j = j.wrapping_add(1);
                    }
                    let mut covOffset_0: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *const u8,
                    )
                        as u32);
                    let ref mut fresh1 =
                        *(*subtable).match_0.offset((*subtable).inputIndex as isize);
                    *fresh1 = readCoverage(
                        data as *const u8,
                        tableLength,
                        covOffset_0,
                    );
                    if !(nReplacement as ::core::ffi::c_int
                        != (**(*subtable).match_0.offset((*subtable).inputIndex as isize)).numGlyphs
                            as ::core::ffi::c_int)
                    {
                        let mut j_0: tableid_t = 0 as tableid_t;
                        while (j_0 as ::core::ffi::c_int) < nForward as ::core::ffi::c_int {
                            let mut covOffset_1: u32 = offset.wrapping_add(read_16u(
                                data.offset(offset as isize)
                                    .offset(8 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (nBacktrack as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as u32);
                            let ref mut fresh2 = *(*subtable).match_0.offset(
                                (nBacktrack as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + j_0 as ::core::ffi::c_int)
                                    as isize,
                            );
                            *fresh2 = readCoverage(
                                data as *const u8,
                                tableLength,
                                covOffset_1,
                            );
                            j_0 = j_0.wrapping_add(1);
                        }
                        (*subtable).to = __caryll_allocate_clean(
                            ::core::mem::size_of::<otl_Coverage>() as usize,
                            64 as ::core::ffi::c_ulong,
                        ) as *mut otl_Coverage;
                        (*(*subtable).to).numGlyphs = nReplacement as glyphid_t;
                        (*(*subtable).to).glyphs = __caryll_allocate_clean(
                            (::core::mem::size_of::<otfcc_GlyphHandle>() as usize)
                                .wrapping_mul(nReplacement as usize),
                            66 as ::core::ffi::c_ulong,
                        )
                            as *mut otfcc_GlyphHandle;
                        let mut j_1: tableid_t = 0 as tableid_t;
                        while (j_1 as ::core::ffi::c_int) < nReplacement as ::core::ffi::c_int {
                            *(*(*subtable).to).glyphs.offset(j_1 as isize) =
                                handle_fromIndex(
                                    read_16u(
                                        data.offset(offset as isize)
                                            .offset(10 as ::core::ffi::c_int as isize)
                                            .offset(
                                                ((nBacktrack as ::core::ffi::c_int
                                                    + nForward as ::core::ffi::c_int
                                                    + j_1 as ::core::ffi::c_int)
                                                    * 2 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            as *const u8,
                                    ) as glyphid_t,
                                ) as otfcc_GlyphHandle;
                            j_1 = j_1.wrapping_add(1);
                        }
                        reverseBacktracks((*subtable).match_0, (*subtable).inputIndex);
                        return subtable as *mut otl_Subtable;
                    }
                }
            }
        }
    }
    iSubtable_gsub_reverse
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_dump_reverse(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gsub_reverse = &raw const (*_subtable).gsub_reverse;
    let mut _st: *mut json_value = json_object_new(3 as usize);
    let mut _match: *mut json_value = json_array_new((*subtable).matchCount as usize);
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
        json_array_push(
            _match,
            otl_iCoverage.dump.expect("non-null function pointer")(
                *(*subtable).match_0.offset(j as isize),
            ),
        );
        j = j.wrapping_add(1);
    }
    json_object_push(
        _st,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        _match,
    );
    json_object_push(
        _st,
        b"to\0" as *const u8 as *const ::core::ffi::c_char,
        otl_iCoverage.dump.expect("non-null function pointer")((*subtable).to),
    );
    json_object_push(
        _st,
        b"inputIndex\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*subtable).inputIndex as i64),
    );
    return _st;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_parse_reverse(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut _match: *mut json_value = json_obj_get_type(
        _subtable,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    let mut _to: *mut json_value = json_obj_get_type(
        _subtable,
        b"to\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if _match.is_null() || _to.is_null() {
        return ::core::ptr::null_mut::<otl_Subtable>();
    }
    let mut subtable: *mut subtable_gsub_reverse =
        (
            iSubtable_gsub_reverse
                .create
                .expect("non-null function pointer"))();
    (*subtable).matchCount = (*_match).u.array.length as tableid_t;
    (*subtable).match_0 = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut otl_Coverage>() as usize)
            .wrapping_mul((*subtable).matchCount as usize),
        100 as ::core::ffi::c_ulong,
    ) as *mut *mut otl_Coverage;
    (*subtable).inputIndex = json_obj_getnum_fallback(
        _subtable,
        b"inputIndex\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as tableid_t;
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
        let ref mut fresh5 = *(*subtable).match_0.offset(j as isize);
        *fresh5 = otl_iCoverage.parse.expect("non-null function pointer")(
            *(*_match).u.array.values.offset(j as isize),
        );
        j = j.wrapping_add(1);
    }
    (*subtable).to = otl_iCoverage.parse.expect("non-null function pointer")(_to);
    return subtable as *mut otl_Subtable;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gsub_reverse(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gsub_reverse = &raw const (*_subtable).gsub_reverse;
    reverseBacktracks((*subtable).match_0, (*subtable).inputIndex);
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            *(*subtable).match_0.offset((*subtable).inputIndex as isize),
        )),
        bkover as ::core::ffi::c_int,
    );
    bk_push(
        root,
        b16 as ::core::ffi::c_int,
        (*subtable).inputIndex as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*subtable).inputIndex as ::core::ffi::c_int {
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*subtable).match_0.offset(j as isize),
            )),
            bkover as ::core::ffi::c_int,
        );
        j = j.wrapping_add(1);
    }
    bk_push(
        root,
        b16 as ::core::ffi::c_int,
        (*subtable).matchCount as ::core::ffi::c_int
            - (*subtable).inputIndex as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j_0: tableid_t =
        ((*subtable).inputIndex as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as tableid_t;
    while (j_0 as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*subtable).match_0.offset(j_0 as isize),
            )),
            bkover as ::core::ffi::c_int,
        );
        j_0 = j_0.wrapping_add(1);
    }
    bk_push(
        root,
        b16 as ::core::ffi::c_int,
        (*(*subtable).to).numGlyphs as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j_1: tableid_t = 0 as tableid_t;
    while (j_1 as ::core::ffi::c_int) < (*(*subtable).to).numGlyphs as ::core::ffi::c_int {
        bk_push(
            root,
            b16 as ::core::ffi::c_int,
            (*(*(*subtable).to).glyphs.offset(j_1 as isize)).index as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        j_1 = j_1.wrapping_add(1);
    }
    return bk_build_Block(root);
}
