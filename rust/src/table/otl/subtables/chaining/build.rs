extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    static otl_iCoverage: __otfcc_ICoverage;
    static otl_iClassDef: __otfcc_IClassDef;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}

use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{otfcc_GlyphHandle, otfcc_LookupHandle};
use crate::support::stdio::FILE;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn otfcc_chainingLookupIsContextualLookup(
    mut lookup: *const otl_Lookup,
) -> bool {
    if !((*lookup).type_0 as ::core::ffi::c_uint
        == otl_type_gpos_chaining as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*lookup).type_0 as ::core::ffi::c_uint
            == otl_type_gsub_chaining as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        return false;
    }
    let mut isContextual: bool = true;
    let mut j: tableid_t = 0 as tableid_t;
    while (j as usize) < (*lookup).subtables.length {
        let mut subtable: *const subtable_chaining =
            &raw mut (**(*lookup).subtables.items.offset(j as isize)).chaining;
        if (*subtable).type_0 as ::core::ffi::c_uint
            == otl_chaining_classified as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut k: tableid_t = 0 as tableid_t;
            while (k as ::core::ffi::c_int)
                < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
            {
                let mut rule: *mut otl_ChainingRule = *(*subtable)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let mut nBacktrack: tableid_t = (*rule).inputBegins;
                let mut nLookahead: tableid_t = ((*rule).matchCount as ::core::ffi::c_int
                    - (*rule).inputEnds as ::core::ffi::c_int)
                    as tableid_t;
                isContextual =
                    isContextual as ::core::ffi::c_int != 0 && nBacktrack == 0 && nLookahead == 0;
                k = k.wrapping_add(1);
            }
        } else {
            let mut rule_0: *mut otl_ChainingRule =
                &raw const (*subtable).c2rust_unnamed.rule as *mut otl_ChainingRule;
            let mut nBacktrack_0: tableid_t = (*rule_0).inputBegins;
            let mut nLookahead_0: tableid_t = ((*rule_0).matchCount as ::core::ffi::c_int
                - (*rule_0).inputEnds as ::core::ffi::c_int)
                as tableid_t;
            isContextual =
                isContextual as ::core::ffi::c_int != 0 && nBacktrack_0 == 0 && nLookahead_0 == 0;
        }
        j = j.wrapping_add(1);
    }
    return isContextual;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_chaining_coverage(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_chaining = &raw const (*_subtable).chaining;
    let mut rule: *mut otl_ChainingRule =
        &raw const (*subtable).c2rust_unnamed.rule as *mut otl_ChainingRule;
    let mut nBacktrack: tableid_t = (*rule).inputBegins;
    let mut nInput: tableid_t = ((*rule).inputEnds as ::core::ffi::c_int
        - (*rule).inputBegins as ::core::ffi::c_int) as tableid_t;
    let mut nLookahead: tableid_t = ((*rule).matchCount as ::core::ffi::c_int
        - (*rule).inputEnds as ::core::ffi::c_int) as tableid_t;
    let mut nSubst: tableid_t = (*rule).applyCount;
    reverseBacktracks(rule);
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    bk_push(
        root,
        b16 as ::core::ffi::c_int,
        nBacktrack as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*rule).inputBegins as ::core::ffi::c_int {
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j as isize),
            )),
            bkover as ::core::ffi::c_int,
        );
        j = j.wrapping_add(1);
    }
    bk_push(
        root,
        b16 as ::core::ffi::c_int,
        nInput as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j_0: tableid_t = (*rule).inputBegins;
    while (j_0 as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j_0 as isize),
            )),
            bkover as ::core::ffi::c_int,
        );
        j_0 = j_0.wrapping_add(1);
    }
    bk_push(
        root,
        b16 as ::core::ffi::c_int,
        nLookahead as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j_1: tableid_t = (*rule).inputEnds;
    while (j_1 as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j_1 as isize),
            )),
            bkover as ::core::ffi::c_int,
        );
        j_1 = j_1.wrapping_add(1);
    }
    bk_push(
        root,
        b16 as ::core::ffi::c_int,
        (*rule).applyCount as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j_2: tableid_t = 0 as tableid_t;
    while (j_2 as ::core::ffi::c_int) < nSubst as ::core::ffi::c_int {
        bk_push(
            root,
            b16 as ::core::ffi::c_int,
            (*(*rule).apply.offset(j_2 as isize)).index as ::core::ffi::c_int
                - nBacktrack as ::core::ffi::c_int,
            b16 as ::core::ffi::c_int,
            (*(*rule).apply.offset(j_2 as isize)).lookup.index as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        j_2 = j_2.wrapping_add(1);
    }
    return bk_build_Block(root);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_chaining_classes(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_chaining = &raw const (*_subtable).chaining;
    let mut coverage: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    coverage = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_Coverage>() as usize,
        67 as ::core::ffi::c_ulong,
    ) as *mut otl_Coverage;
    (*coverage).numGlyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).numGlyphs;
    (*coverage).glyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).glyphs;
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            coverage,
        )),
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.bc,
        )),
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.ic,
        )),
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.fc,
        )),
        b16 as ::core::ffi::c_int,
        (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut rcpg: *mut glyphclass_t = ::core::ptr::null_mut::<glyphclass_t>();
    rcpg = __caryll_allocate_clean(
        (::core::mem::size_of::<glyphclass_t>() as usize).wrapping_mul(
            ((*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as usize,
        ),
        81 as ::core::ffi::c_ulong,
    ) as *mut glyphclass_t;
    let mut j: glyphclass_t = 0 as glyphclass_t;
    while j as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        *rcpg.offset(j as isize) = 0 as glyphclass_t;
        j = j.wrapping_add(1);
    }
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int)
        < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
    {
        let mut ib: tableid_t = (**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .inputBegins;
        let mut startClass: tableid_t = (*(**(**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .match_0
        .offset(ib as isize))
        .glyphs
        .offset(0 as ::core::ffi::c_int as isize))
        .index as tableid_t;
        if startClass as ::core::ffi::c_int
            <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
        {
            let ref mut fresh2 = *rcpg.offset(startClass as isize);
            *fresh2 = (*fresh2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphclass_t;
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: glyphclass_t = 0 as glyphclass_t;
    while j_1 as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        if *rcpg.offset(j_1 as isize) != 0 {
            let mut cset: *mut bk_Block = bk_new_Block(
                b16 as ::core::ffi::c_int,
                *rcpg.offset(j_1 as isize) as ::core::ffi::c_int,
                bkover as ::core::ffi::c_int,
            );
            let mut k: tableid_t = 0 as tableid_t;
            while (k as ::core::ffi::c_int)
                < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
            {
                let mut rule: *mut otl_ChainingRule = *(*subtable)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let mut startClass_0: glyphclass_t =
                    (*(**(*rule).match_0.offset((*rule).inputBegins as isize))
                        .glyphs
                        .offset(0 as ::core::ffi::c_int as isize))
                    .index as glyphclass_t;
                if !(startClass_0 as ::core::ffi::c_int != j_1 as ::core::ffi::c_int) {
                    reverseBacktracks(rule);
                    let mut nBacktrack: tableid_t = (*rule).inputBegins;
                    let mut nInput: tableid_t = ((*rule).inputEnds as ::core::ffi::c_int
                        - (*rule).inputBegins as ::core::ffi::c_int)
                        as tableid_t;
                    let mut nLookahead: tableid_t = ((*rule).matchCount as ::core::ffi::c_int
                        - (*rule).inputEnds as ::core::ffi::c_int)
                        as tableid_t;
                    let mut nSubst: tableid_t = (*rule).applyCount;
                    let mut r: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
                    bk_push(
                        r,
                        b16 as ::core::ffi::c_int,
                        nBacktrack as ::core::ffi::c_int,
                        bkover as ::core::ffi::c_int,
                    );
                    let mut m: tableid_t = 0 as tableid_t;
                    while (m as ::core::ffi::c_int) < (*rule).inputBegins as ::core::ffi::c_int {
                        bk_push(
                            r,
                            b16 as ::core::ffi::c_int,
                            (*(**(*rule).match_0.offset(m as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int,
                            bkover as ::core::ffi::c_int,
                        );
                        m = m.wrapping_add(1);
                    }
                    bk_push(
                        r,
                        b16 as ::core::ffi::c_int,
                        nInput as ::core::ffi::c_int,
                        bkover as ::core::ffi::c_int,
                    );
                    let mut m_0: tableid_t = ((*rule).inputBegins as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as tableid_t;
                    while (m_0 as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
                        bk_push(
                            r,
                            b16 as ::core::ffi::c_int,
                            (*(**(*rule).match_0.offset(m_0 as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int,
                            bkover as ::core::ffi::c_int,
                        );
                        m_0 = m_0.wrapping_add(1);
                    }
                    bk_push(
                        r,
                        b16 as ::core::ffi::c_int,
                        nLookahead as ::core::ffi::c_int,
                        bkover as ::core::ffi::c_int,
                    );
                    let mut m_1: tableid_t = (*rule).inputEnds;
                    while (m_1 as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
                        bk_push(
                            r,
                            b16 as ::core::ffi::c_int,
                            (*(**(*rule).match_0.offset(m_1 as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int,
                            bkover as ::core::ffi::c_int,
                        );
                        m_1 = m_1.wrapping_add(1);
                    }
                    bk_push(
                        r,
                        b16 as ::core::ffi::c_int,
                        nSubst as ::core::ffi::c_int,
                        bkover as ::core::ffi::c_int,
                    );
                    let mut m_2: tableid_t = 0 as tableid_t;
                    while (m_2 as ::core::ffi::c_int) < nSubst as ::core::ffi::c_int {
                        bk_push(
                            r,
                            b16 as ::core::ffi::c_int,
                            (*(*rule).apply.offset(m_2 as isize)).index as ::core::ffi::c_int
                                - nBacktrack as ::core::ffi::c_int,
                            b16 as ::core::ffi::c_int,
                            (*(*rule).apply.offset(m_2 as isize)).lookup.index
                                as ::core::ffi::c_int,
                            bkover as ::core::ffi::c_int,
                        );
                        m_2 = m_2.wrapping_add(1);
                    }
                    bk_push(
                        cset,
                        p16 as ::core::ffi::c_int,
                        r,
                        bkover as ::core::ffi::c_int,
                    );
                }
                k = k.wrapping_add(1);
            }
            bk_push(
                root,
                p16 as ::core::ffi::c_int,
                cset,
                bkover as ::core::ffi::c_int,
            );
        } else {
            bk_push(
                root,
                p16 as ::core::ffi::c_int,
                NULL,
                bkover as ::core::ffi::c_int,
            );
        }
        j_1 = j_1.wrapping_add(1);
    }
    free(coverage as *mut ::core::ffi::c_void);
    coverage = ::core::ptr::null_mut::<otl_Coverage>();
    free(rcpg as *mut ::core::ffi::c_void);
    rcpg = ::core::ptr::null_mut::<glyphclass_t>();
    return bk_build_Block(root);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_chaining(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    if (*_subtable).chaining.type_0 as ::core::ffi::c_uint
        == otl_chaining_classified as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return otfcc_build_chaining_classes(_subtable);
    } else {
        return otfcc_build_chaining_coverage(_subtable);
    };
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_contextual_coverage(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_chaining = &raw const (*_subtable).chaining;
    let mut rule: *mut otl_ChainingRule =
        &raw const (*subtable).c2rust_unnamed.rule as *mut otl_ChainingRule;
    let mut nInput: tableid_t = ((*rule).inputEnds as ::core::ffi::c_int
        - (*rule).inputBegins as ::core::ffi::c_int) as tableid_t;
    let mut nSubst: tableid_t = (*rule).applyCount;
    reverseBacktracks(rule);
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    bk_push(
        root,
        b16 as ::core::ffi::c_int,
        nInput as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    bk_push(
        root,
        b16 as ::core::ffi::c_int,
        nSubst as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j: tableid_t = (*rule).inputBegins;
    while (j as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j as isize),
            )),
            bkover as ::core::ffi::c_int,
        );
        j = j.wrapping_add(1);
    }
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int) < nSubst as ::core::ffi::c_int {
        bk_push(
            root,
            b16 as ::core::ffi::c_int,
            (*(*rule).apply.offset(j_0 as isize)).index as ::core::ffi::c_int,
            b16 as ::core::ffi::c_int,
            (*(*rule).apply.offset(j_0 as isize)).lookup.index as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        j_0 = j_0.wrapping_add(1);
    }
    return bk_build_Block(root);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_contextual_classes(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_chaining = &raw const (*_subtable).chaining;
    let mut coverage: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    coverage = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_Coverage>() as usize,
        174 as ::core::ffi::c_ulong,
    ) as *mut otl_Coverage;
    (*coverage).numGlyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).numGlyphs;
    (*coverage).glyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).glyphs;
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            coverage,
        )),
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.ic,
        )),
        b16 as ::core::ffi::c_int,
        (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut rcpg: *mut glyphclass_t = ::core::ptr::null_mut::<glyphclass_t>();
    rcpg = __caryll_allocate_clean(
        (::core::mem::size_of::<glyphclass_t>() as usize).wrapping_mul(
            ((*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as usize,
        ),
        186 as ::core::ffi::c_ulong,
    ) as *mut glyphclass_t;
    let mut j: glyphclass_t = 0 as glyphclass_t;
    while j as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        *rcpg.offset(j as isize) = 0 as glyphclass_t;
        j = j.wrapping_add(1);
    }
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int)
        < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
    {
        let mut ib: tableid_t = (**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .inputBegins;
        let mut startClass: tableid_t = (*(**(**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .match_0
        .offset(ib as isize))
        .glyphs
        .offset(0 as ::core::ffi::c_int as isize))
        .index as tableid_t;
        if startClass as ::core::ffi::c_int
            <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
        {
            let ref mut fresh3 = *rcpg.offset(startClass as isize);
            *fresh3 = (*fresh3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphclass_t;
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: glyphclass_t = 0 as glyphclass_t;
    while j_1 as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        if *rcpg.offset(j_1 as isize) != 0 {
            let mut cset: *mut bk_Block = bk_new_Block(
                b16 as ::core::ffi::c_int,
                *rcpg.offset(j_1 as isize) as ::core::ffi::c_int,
                bkover as ::core::ffi::c_int,
            );
            let mut k: tableid_t = 0 as tableid_t;
            while (k as ::core::ffi::c_int)
                < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
            {
                let mut rule: *mut otl_ChainingRule = *(*subtable)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let mut startClass_0: glyphclass_t =
                    (*(**(*rule).match_0.offset((*rule).inputBegins as isize))
                        .glyphs
                        .offset(0 as ::core::ffi::c_int as isize))
                    .index as glyphclass_t;
                if !(startClass_0 as ::core::ffi::c_int != j_1 as ::core::ffi::c_int) {
                    reverseBacktracks(rule);
                    let mut nInput: tableid_t = ((*rule).inputEnds as ::core::ffi::c_int
                        - (*rule).inputBegins as ::core::ffi::c_int)
                        as tableid_t;
                    let mut nSubst: tableid_t = (*rule).applyCount;
                    let mut r: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
                    bk_push(
                        r,
                        b16 as ::core::ffi::c_int,
                        nInput as ::core::ffi::c_int,
                        bkover as ::core::ffi::c_int,
                    );
                    bk_push(
                        r,
                        b16 as ::core::ffi::c_int,
                        nSubst as ::core::ffi::c_int,
                        bkover as ::core::ffi::c_int,
                    );
                    let mut m: tableid_t = ((*rule).inputBegins as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as tableid_t;
                    while (m as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
                        bk_push(
                            r,
                            b16 as ::core::ffi::c_int,
                            (*(**(*rule).match_0.offset(m as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int,
                            bkover as ::core::ffi::c_int,
                        );
                        m = m.wrapping_add(1);
                    }
                    let mut m_0: tableid_t = 0 as tableid_t;
                    while (m_0 as ::core::ffi::c_int) < nSubst as ::core::ffi::c_int {
                        bk_push(
                            r,
                            b16 as ::core::ffi::c_int,
                            (*(*rule).apply.offset(m_0 as isize)).index as ::core::ffi::c_int,
                            b16 as ::core::ffi::c_int,
                            (*(*rule).apply.offset(m_0 as isize)).lookup.index
                                as ::core::ffi::c_int,
                            bkover as ::core::ffi::c_int,
                        );
                        m_0 = m_0.wrapping_add(1);
                    }
                    bk_push(
                        cset,
                        p16 as ::core::ffi::c_int,
                        r,
                        bkover as ::core::ffi::c_int,
                    );
                }
                k = k.wrapping_add(1);
            }
            bk_push(
                root,
                p16 as ::core::ffi::c_int,
                cset,
                bkover as ::core::ffi::c_int,
            );
        } else {
            bk_push(
                root,
                p16 as ::core::ffi::c_int,
                NULL,
                bkover as ::core::ffi::c_int,
            );
        }
        j_1 = j_1.wrapping_add(1);
    }
    free(coverage as *mut ::core::ffi::c_void);
    coverage = ::core::ptr::null_mut::<otl_Coverage>();
    free(rcpg as *mut ::core::ffi::c_void);
    rcpg = ::core::ptr::null_mut::<glyphclass_t>();
    return bk_build_Block(root);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_contextual(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    if (*_subtable).chaining.type_0 as ::core::ffi::c_uint
        == otl_chaining_classified as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return otfcc_build_contextual_classes(_subtable);
    } else {
        return otfcc_build_contextual_coverage(_subtable);
    };
}
#[inline]
unsafe extern "C" fn reverseBacktracks(mut rule: *mut otl_ChainingRule) {
    if (*rule).inputBegins as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut start: tableid_t = 0 as tableid_t;
        let mut end: tableid_t =
            ((*rule).inputBegins as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as tableid_t;
        while end as ::core::ffi::c_int > start as ::core::ffi::c_int {
            let mut tmp: *mut otl_Coverage = *(*rule).match_0.offset(start as isize);
            let ref mut fresh0 = *(*rule).match_0.offset(start as isize);
            *fresh0 = *(*rule).match_0.offset(end as isize);
            let ref mut fresh1 = *(*rule).match_0.offset(end as isize);
            *fresh1 = tmp;
            end = end.wrapping_sub(1);
            start = start.wrapping_add(1);
        }
    }
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
