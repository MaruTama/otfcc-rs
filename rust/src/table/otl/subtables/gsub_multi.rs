use libc::{free, malloc, memcpy, memset, qsort};
extern "C" {
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}

use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage, otl_Coverage};
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle, HANDLE_STATE_EMPTY};

use crate::support::alloc::__caryll_reallocate;
use crate::support::binio::{read_16u};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphclass_t, glyphid_t, pos_t, tableid_t};
use crate::vendor::sds::{sds};
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
pub struct __caryll_vectorinterface_subtable_gsub_multi {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut subtable_gsub_multi, *const subtable_gsub_multi) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gsub_multi, *mut subtable_gsub_multi) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, subtable_gsub_multi) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_multi, subtable_gsub_multi) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gsub_multi>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut subtable_gsub_multi>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, otl_GsubMultiEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> otl_GsubMultiEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_multi,
            Option<
                unsafe extern "C" fn(*const otl_GsubMultiEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_multi,
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubMultiEntry,
                    *const otl_GsubMultiEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_GsubMultiEntry {
    pub init: Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry, *const otl_GsubMultiEntry) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry, *mut otl_GsubMultiEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry, otl_GsubMultiEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_GsubMultiEntry, otl_GsubMultiEntry) -> ()>,
}
pub type otl_BuildHeuristics = ::core::ffi::c_uint;
pub const OTL_BH_GSUB_VERT: otl_BuildHeuristics = 1;
pub const OTL_BH_NORMAL: otl_BuildHeuristics = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn deleteGsubMultiEntry(mut entry: *mut otl_GsubMultiEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).from);
    otl_Coverage_free((*entry).to);
    (*entry).to = ::core::ptr::null_mut::<otl_Coverage>();
}
static mut gsm_typeinfo: __caryll_elementinterface_otl_GsubMultiEntry = {
    __caryll_elementinterface_otl_GsubMultiEntry {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(deleteGsubMultiEntry as unsafe extern "C" fn(*mut otl_GsubMultiEntry) -> ()),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe fn as_cvec(arr: *mut subtable_gsub_multi) -> *mut CVecRaw<otl_GsubMultiEntry> {
    arr as *mut CVecRaw<otl_GsubMultiEntry>
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_growTo(arr: *mut subtable_gsub_multi, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
#[no_mangle]
pub static mut iSubtable_gsub_multi: __caryll_vectorinterface_subtable_gsub_multi = {
    __caryll_vectorinterface_subtable_gsub_multi {
        init: Some(
            subtable_gsub_multi_init as unsafe extern "C" fn(*mut subtable_gsub_multi) -> (),
        ),
        copy: Some(
            subtable_gsub_multi_copy
                as unsafe extern "C" fn(*mut subtable_gsub_multi, *const subtable_gsub_multi) -> (),
        ),
        move_0: Some(
            subtable_gsub_multi_move
                as unsafe extern "C" fn(*mut subtable_gsub_multi, *mut subtable_gsub_multi) -> (),
        ),
        dispose: Some(
            subtable_gsub_multi_dispose as unsafe extern "C" fn(*mut subtable_gsub_multi) -> (),
        ),
        replace: Some(
            subtable_gsub_multi_replace
                as unsafe extern "C" fn(*mut subtable_gsub_multi, subtable_gsub_multi) -> (),
        ),
        copyReplace: Some(
            subtable_gsub_multi_copyReplace
                as unsafe extern "C" fn(*mut subtable_gsub_multi, subtable_gsub_multi) -> (),
        ),
        create: Some(subtable_gsub_multi_create),
        free: Some(
            subtable_gsub_multi_free as unsafe extern "C" fn(*mut subtable_gsub_multi) -> (),
        ),
        initN: Some(
            subtable_gsub_multi_initN
                as unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> (),
        ),
        initCapN: Some(
            subtable_gsub_multi_initCapN
                as unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> (),
        ),
        createN: Some(
            subtable_gsub_multi_createN as unsafe extern "C" fn(usize) -> *mut subtable_gsub_multi,
        ),
        fill: Some(
            subtable_gsub_multi_fill
                as unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> (),
        ),
        clear: Some(
            subtable_gsub_multi_dispose as unsafe extern "C" fn(*mut subtable_gsub_multi) -> (),
        ),
        push: Some(
            subtable_gsub_multi_push
                as unsafe extern "C" fn(*mut subtable_gsub_multi, otl_GsubMultiEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gsub_multi_shrinkToFit as unsafe extern "C" fn(*mut subtable_gsub_multi) -> (),
        ),
        pop: Some(
            subtable_gsub_multi_pop
                as unsafe extern "C" fn(*mut subtable_gsub_multi) -> otl_GsubMultiEntry,
        ),
        disposeItem: Some(
            subtable_gsub_multi_disposeItem
                as unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> (),
        ),
        filterEnv: Some(
            subtable_gsub_multi_filterEnv
                as unsafe extern "C" fn(
                    *mut subtable_gsub_multi,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GsubMultiEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gsub_multi_sort
                as unsafe extern "C" fn(
                    *mut subtable_gsub_multi,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GsubMultiEntry,
                            *const otl_GsubMultiEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gsub_multi_shrinkToFit(mut arr: *mut subtable_gsub_multi) {
    subtable_gsub_multi_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_resizeTo(arr: *mut subtable_gsub_multi, target: usize) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_move(
    dst: *mut subtable_gsub_multi,
    src: *mut subtable_gsub_multi,
) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_disposeItem(
    mut arr: *mut subtable_gsub_multi,
    mut n: usize,
) {
    if gsm_typeinfo.dispose.is_some() {
        gsm_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_GsubMultiEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_sort(
    mut arr: *mut subtable_gsub_multi,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_GsubMultiEntry,
            *const otl_GsubMultiEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_GsubMultiEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubMultiEntry,
                    *const otl_GsubMultiEntry,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_fill(mut arr: *mut subtable_gsub_multi, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_GsubMultiEntry = otl_GsubMultiEntry {
            from: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            to: ::core::ptr::null_mut::<otl_Coverage>(),
        };
        if gsm_typeinfo.init.is_some() {
            gsm_typeinfo.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_GsubMultiEntry>() as usize,
            );
        }
        subtable_gsub_multi_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_push(arr: *mut subtable_gsub_multi, elem: otl_GsubMultiEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_grow(arr: *mut subtable_gsub_multi) {
    cvec_grow(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_init(arr: *mut subtable_gsub_multi) {
    cvec_init(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_copyReplace(
    mut dst: *mut subtable_gsub_multi,
    src: subtable_gsub_multi,
) {
    subtable_gsub_multi_dispose(dst);
    subtable_gsub_multi_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_pop(arr: *mut subtable_gsub_multi) -> otl_GsubMultiEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_copy(
    mut dst: *mut subtable_gsub_multi,
    mut src: *const subtable_gsub_multi,
) {
    subtable_gsub_multi_init(dst);
    subtable_gsub_multi_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gsm_typeinfo.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            gsm_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_GsubMultiEntry,
                (*src).items.offset(j as isize) as *mut otl_GsubMultiEntry
                    as *const otl_GsubMultiEntry,
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
unsafe extern "C" fn subtable_gsub_multi_dispose(mut arr: *mut subtable_gsub_multi) {
    if arr.is_null() {
        return;
    }
    if gsm_typeinfo.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            gsm_typeinfo.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_GsubMultiEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_GsubMultiEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_replace(
    mut dst: *mut subtable_gsub_multi,
    src: subtable_gsub_multi,
) {
    subtable_gsub_multi_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gsub_multi>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_initCapN(
    mut arr: *mut subtable_gsub_multi,
    mut n: usize,
) {
    subtable_gsub_multi_init(arr);
    subtable_gsub_multi_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_growToN(arr: *mut subtable_gsub_multi, target: usize) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_initN(mut arr: *mut subtable_gsub_multi, mut n: usize) {
    subtable_gsub_multi_init(arr);
    subtable_gsub_multi_growToN(arr, n);
    subtable_gsub_multi_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_free(mut x: *mut subtable_gsub_multi) {
    if x.is_null() {
        return;
    }
    subtable_gsub_multi_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_createN(mut n: usize) -> *mut subtable_gsub_multi {
    let mut t: *mut subtable_gsub_multi =
        malloc(::core::mem::size_of::<subtable_gsub_multi>() as usize) as *mut subtable_gsub_multi;
    subtable_gsub_multi_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_create() -> *mut subtable_gsub_multi {
    let mut x: *mut subtable_gsub_multi =
        malloc(::core::mem::size_of::<subtable_gsub_multi>() as usize) as *mut subtable_gsub_multi;
    subtable_gsub_multi_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gsub_multi_filterEnv(
    mut arr: *mut subtable_gsub_multi,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_GsubMultiEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_GsubMultiEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gsm_typeinfo.dispose.is_some() {
                gsm_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_GsubMultiEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gsub_multi(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut seqCount: glyphid_t = 0;
    let subtable: *mut subtable_gsub_multi =
        (
            iSubtable_gsub_multi
                .create
                .expect("non-null function pointer"))();
    let mut from: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        from = readCoverage(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        seqCount = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as glyphid_t;
        if seqCount as ::core::ffi::c_int == (*from).numGlyphs as ::core::ffi::c_int {
            if !(tableLength
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (seqCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                ))
            {
                for j in 0..seqCount {
                    let seqOffset: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    )
                        as u32);
                    let cov: *mut otl_Coverage =
                        otl_Coverage_create();
                    let n: glyphid_t =
                        read_16u(data.offset(seqOffset as isize) as *const u8) as glyphid_t;
                    for k in 0..n {
                        pushToCoverage(
                            cov,
                            handle_fromIndex(read_16u(
                                data.offset(seqOffset as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as glyphid_t) as otfcc_GlyphHandle,
                        );
                    }
                    iSubtable_gsub_multi
                        .push
                        .expect("non-null function pointer")(
                        subtable,
                        otl_GsubMultiEntry {
                            from: otfcc_Handle_dup(
                                *(*from).glyphs.offset(j as isize) as otfcc_Handle,
                            ) as otfcc_GlyphHandle,
                            to: cov,
                        },
                    );
                }
                otl_Coverage_free(from);
                return subtable as *mut otl_Subtable;
            }
        }
    }
    if !from.is_null() {
        otl_Coverage_free(from);
    }
    iSubtable_gsub_multi
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_dump_multi(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let subtable: *const subtable_gsub_multi = &raw const (*_subtable).gsub_multi;
    let st: *mut json_value = json_object_new((*subtable).length);
    for j in 0..(*subtable).length as glyphid_t {
        let entry = (*subtable).items.offset(j as isize);
        json_object_push(
            st,
            (*entry).from.name as *const ::core::ffi::c_char,
            otl_iCoverage.dump.expect("non-null function pointer")((*entry).to),
        );
    }
    return st;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_parse_multi(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let st: *mut subtable_gsub_multi =
        (
            iSubtable_gsub_multi
                .create
                .expect("non-null function pointer"))();
    for k in 0..(*_subtable).u.object.length as glyphid_t {
        let entry = (*_subtable).u.object.values.offset(k as isize);
        let _to: *mut json_value = (*entry).value as *mut json_value;
        if !_to.is_null() && (*_to).type_0 == json_array {
            iSubtable_gsub_multi
                .push
                .expect("non-null function pointer")(
                st,
                otl_GsubMultiEntry {
                    from: handle_fromName(sdsnewlen(
                        (*entry).name as *const ::core::ffi::c_void,
                        (*entry).name_length as usize,
                    )) as otfcc_GlyphHandle,
                    to: otl_iCoverage.parse.expect("non-null function pointer")(_to),
                },
            );
        }
    }
    return st as *mut otl_Subtable;
}
unsafe extern "C" fn buildGsubMultiSubtableRange(
    subtable: *const subtable_gsub_multi,
    start: glyphid_t,
    end: glyphid_t,
) -> *mut caryll_Buffer {
    let cov: *mut otl_Coverage = otl_Coverage_create();
    for j in start..end {
        pushToCoverage(
            cov,
            otfcc_Handle_dup(
                (*(*subtable).items.offset(j as isize)).from as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
    }
    let root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(cov)),
        b16 as ::core::ffi::c_int,
        end as ::core::ffi::c_int - start as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    for j_0 in start..end {
        let to = (*(*subtable).items.offset(j_0 as isize)).to;
        let b: *mut bk_Block = bk_new_Block(
            b16 as ::core::ffi::c_int,
            (*to).numGlyphs as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        for k in 0..(*to).numGlyphs {
            bk_push(
                b,
                b16 as ::core::ffi::c_int,
                (*(*to).glyphs.offset(k as isize)).index as ::core::ffi::c_int,
                bkover as ::core::ffi::c_int,
            );
        }
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            b,
            bkover as ::core::ffi::c_int,
        );
    }
    otl_Coverage_free(cov);
    return bk_build_Block(root);
}
pub const GSUB_MULTI_SUBTABLE_SIZE_LIMIT: ::core::ffi::c_int = 0xff00 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gsub_multi_subtable_split(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
    mut count: *mut tableid_t,
) -> *mut *mut caryll_Buffer {
    let mut subtable: *const subtable_gsub_multi = &raw const (*_subtable).gsub_multi;
    let mut parts: *mut *mut caryll_Buffer = ::core::ptr::null_mut::<*mut caryll_Buffer>();
    let mut nParts: tableid_t = 0 as tableid_t;
    let mut start: glyphid_t = 0 as glyphid_t;
    while (start as usize) < (*subtable).length {
        let mut size: usize = (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize;
        let mut end: glyphid_t = start;
        while (end as usize) < (*subtable).length {
            let mut entrySize: usize = ((2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize)
                .wrapping_add(
                    ((*(*(*subtable).items.offset(end as isize)).to).numGlyphs as usize)
                        .wrapping_mul(2 as usize),
                );
            if end as ::core::ffi::c_int > start as ::core::ffi::c_int
                && size.wrapping_add(entrySize) > GSUB_MULTI_SUBTABLE_SIZE_LIMIT as usize
            {
                break;
            }
            size = size.wrapping_add(entrySize);
            end = end.wrapping_add(1);
        }
        parts = __caryll_reallocate(
            parts as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut caryll_Buffer>() as usize)
                .wrapping_mul((nParts as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize),
            125 as ::core::ffi::c_ulong,
        ) as *mut *mut caryll_Buffer;
        let ref mut fresh2 = *parts.offset(nParts as isize);
        *fresh2 = buildGsubMultiSubtableRange(subtable, start, end);
        nParts = nParts.wrapping_add(1);
        start = end;
    }
    if nParts == 0 {
        parts = __caryll_reallocate(
            parts as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut caryll_Buffer>() as usize).wrapping_mul(1 as usize),
            132 as ::core::ffi::c_ulong,
        ) as *mut *mut caryll_Buffer;
        let ref mut fresh3 = *parts.offset(0 as ::core::ffi::c_int as isize);
        *fresh3 = buildGsubMultiSubtableRange(subtable, 0 as glyphid_t, 0 as glyphid_t);
        nParts = 1 as tableid_t;
    }
    *count = nParts;
    return parts;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gsub_multi_subtable(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gsub_multi = &raw const (*_subtable).gsub_multi;
    return buildGsubMultiSubtableRange(subtable, 0 as glyphid_t, (*subtable).length as glyphid_t);
}
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
