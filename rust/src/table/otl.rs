#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod build;
pub mod classdef;
pub mod constants;
pub mod coverage;
pub mod dump;
pub mod parse;
pub mod read;
pub mod subtables;

use libc::{free, malloc, memcpy, memset, qsort};
unsafe extern "C" {
    fn sdsfree(s: sds);
    static iSubtable_gsub_single: __caryll_vectorinterface_subtable_gsub_single;
    static iSubtable_gsub_multi: __caryll_vectorinterface_subtable_gsub_multi;
    static iSubtable_gsub_ligature: __caryll_vectorinterface_subtable_gsub_ligature;
    static iSubtable_chaining: __caryll_elementinterface_subtable_chaining;
    static iSubtable_gsub_reverse: __caryll_elementinterface_subtable_gsub_reverse;
    static iSubtable_gpos_single: __caryll_vectorinterface_subtable_gpos_single;
    static iSubtable_gpos_pair: __caryll_elementinterface_subtable_gpos_pair;
    static iSubtable_gpos_cursive: __caryll_vectorinterface_subtable_gpos_cursive;
    static iSubtable_gpos_markToSingle: __caryll_elementinterface_subtable_gpos_markToSingle;
    static iSubtable_gpos_markToLigature: __caryll_elementinterface_subtable_gpos_markToLigature;
}

use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{otfcc_GlyphHandle, otfcc_LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::primitives::{glyphclass_t, glyphid_t, pos_t, tableid_t};
use crate::vendor::sds::{sds};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::{__compar_fn_t};


/// Which GSUB/GPOS subtable format a lookup is, in otfcc's own numbering: the
/// file's 16-bit format number offset by the table's base, `otl_type_gsub_*`
/// starting at 16 and `otl_type_gpos_*` at 32, so one value names both the
/// table and the format.
///
/// **Deliberately not an `enum`.** The value is read from the font:
/// `otfcc_readOtl_common` does `lookup->type = read_16u(data) + base`, so
/// anything in `16..=65551` can turn up, and C does not clamp it. An
/// unrecognised type is carried through as-is — `otfcc_readOtl_subtable`
/// returns NULL for it, and the lookup's generated name puts the *raw number*
/// in the output as hex (`lookup_0019_3`, from
/// `sdsbuild!(… Hex2(lookup->type) …)` in `read.rs`). A `#[repr(u32)]` enum
/// could not hold such a value, `transmute`ing one in would be UB, and
/// rejecting it would change the JSON that otfcc writes for a font with an
/// unknown lookup type — which no test payload has, so the byte comparison
/// would not have caught it.
///
/// So this is the honest shape: a newtype over the number, with the known
/// values as named constants and the two file-derived construction sites going
/// through [`otl_LookupType::from_file`]. What it buys over the bare `c_uint`
/// c2rust emitted is that the compiler now separates it from every other 32-bit
/// quantity in the OTL code, and that [`otl_LookupType::name`] replaces a
/// 42-entry sparse table of C string pointers.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct otl_LookupType(u32);

pub const otl_type_gpos_extend: otl_LookupType = otl_LookupType(41);
pub const otl_type_gpos_chaining: otl_LookupType = otl_LookupType(40);
pub const otl_type_gpos_context: otl_LookupType = otl_LookupType(39);
pub const otl_type_gpos_markToMark: otl_LookupType = otl_LookupType(38);
pub const otl_type_gpos_markToLigature: otl_LookupType = otl_LookupType(37);
pub const otl_type_gpos_markToBase: otl_LookupType = otl_LookupType(36);
pub const otl_type_gpos_cursive: otl_LookupType = otl_LookupType(35);
pub const otl_type_gpos_pair: otl_LookupType = otl_LookupType(34);
pub const otl_type_gpos_single: otl_LookupType = otl_LookupType(33);
pub const otl_type_gpos_unknown: otl_LookupType = otl_LookupType(32);
pub const otl_type_gsub_reverse: otl_LookupType = otl_LookupType(24);
pub const otl_type_gsub_extend: otl_LookupType = otl_LookupType(23);
pub const otl_type_gsub_chaining: otl_LookupType = otl_LookupType(22);
pub const otl_type_gsub_context: otl_LookupType = otl_LookupType(21);
pub const otl_type_gsub_ligature: otl_LookupType = otl_LookupType(20);
pub const otl_type_gsub_alternate: otl_LookupType = otl_LookupType(19);
pub const otl_type_gsub_multiple: otl_LookupType = otl_LookupType(18);
pub const otl_type_gsub_single: otl_LookupType = otl_LookupType(17);
pub const otl_type_gsub_unknown: otl_LookupType = otl_LookupType(16);
pub const otl_type_unknown: otl_LookupType = otl_LookupType(0);

impl otl_LookupType {
    /// The type of a lookup as the font file spells it: a format number
    /// relative to `base`, which is `otl_type_gsub_unknown` for GSUB and
    /// `otl_type_gpos_unknown` for GPOS. Wrapping, like the C addition it
    /// replaces — `raw` is a full `u16` straight out of the file and is not
    /// validated here, exactly as C does not validate it.
    pub const fn from_file(base: Self, raw: u16) -> Self {
        Self(base.0.wrapping_add(raw as u32))
    }

    /// The number itself. It reaches the output: a lookup with no name gets
    /// `lookup_<this as %04x>_<index>`.
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// The format number to write back into the file — this value with its
    /// table's base taken off again, and 0 for anything at or below GSUB's base.
    ///
    /// The comparisons are `>`, not `>=`, exactly as in C: `otl_type_unknown`
    /// and `otl_type_gsub_unknown` give 0, while `otl_type_gpos_unknown` (32)
    /// is above *GSUB's* base and so reads as GSUB format 16. That is a quirk
    /// of the original, reachable only from a font declaring a GPOS lookup of
    /// format 0; the number reaches the lookup header, so it is reproduced
    /// rather than tidied. `file_format_undoes_the_table_base` pins it.
    pub const fn file_format(self) -> u32 {
        if self.0 > otl_type_gpos_unknown.0 {
            self.0 - otl_type_gpos_unknown.0
        } else if self.0 > otl_type_gsub_unknown.0 {
            self.0 - otl_type_gsub_unknown.0
        } else {
            0
        }
    }

    /// The name this type has in otfcc's JSON, and the key its lookups are
    /// looked up by when reading JSON back.
    ///
    /// This was `tableNames`, a 42-element `static mut` array of C string
    /// pointers indexed by the type — 23 of whose entries were NULL, since the
    /// numbering leaves holes between the two tables. Every one of the 26 uses
    /// indexed it with a constant, so nothing was ever at risk of reading a
    /// hole; the fallback here is index 0's own text, which keeps the function
    /// total without inventing a name.
    pub const fn name(self) -> &'static ::core::ffi::CStr {
        match self.0 {
            17 => c"gsub_single",
            18 => c"gsub_multiple",
            19 => c"gsub_alternate",
            20 => c"gsub_ligature",
            21 => c"gsub_context",
            22 => c"gsub_chaining",
            23 => c"gsub_extend",
            24 => c"gsub_reverse",
            16 => c"gsub_unknown",
            33 => c"gpos_single",
            34 => c"gpos_pair",
            35 => c"gpos_cursive",
            36 => c"gpos_mark_to_base",
            37 => c"gpos_mark_to_ligature",
            38 => c"gpos_mark_to_mark",
            39 => c"gpos_context",
            40 => c"gpos_chaining",
            41 => c"gpos_extend",
            32 => c"gpos_unknown",
            _ => c"unknown",
        }
    }
}
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
    pub c2rust_unnamed: otl_ChainingBody,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union otl_ChainingBody {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: otl_ChainingRuleSet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ChainingRuleSet {
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
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum otl_chaining_type {
    otl_chaining_canonical = 0,
    otl_chaining_poly = 1,
    otl_chaining_classified = 2,
}
pub use otl_chaining_type::*;
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
pub struct __caryll_vectorinterface_subtable_gsub_single {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut subtable_gsub_single, *const subtable_gsub_single) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gsub_single, *mut subtable_gsub_single) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_single, subtable_gsub_single) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_single, subtable_gsub_single) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gsub_single>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut subtable_gsub_single>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut subtable_gsub_single, otl_GsubSingleEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> otl_GsubSingleEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_single,
            Option<
                unsafe extern "C" fn(*const otl_GsubSingleEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_single,
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubSingleEntry,
                    *const otl_GsubSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
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
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut subtable_gsub_ligature>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, otl_GsubLigatureEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> otl_GsubLigatureEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> ()>,
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
pub struct __caryll_elementinterface_subtable_chaining {
    pub init: Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut subtable_chaining, *const subtable_chaining) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut subtable_chaining, *mut subtable_chaining) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut subtable_chaining, subtable_chaining) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut subtable_chaining, subtable_chaining) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_chaining>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_subtable_gpos_single {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, *const subtable_gpos_single) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, *mut subtable_gpos_single) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, subtable_gpos_single) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, subtable_gpos_single) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_single>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut subtable_gpos_single>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut subtable_gpos_single, otl_GposSingleEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> otl_GposSingleEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_single,
            Option<
                unsafe extern "C" fn(*const otl_GposSingleEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_single,
            Option<
                unsafe extern "C" fn(
                    *const otl_GposSingleEntry,
                    *const otl_GposSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_subtable_gpos_pair {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut subtable_gpos_pair, *const subtable_gpos_pair) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gpos_pair, *mut subtable_gpos_pair) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut subtable_gpos_pair, subtable_gpos_pair) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_pair, subtable_gpos_pair) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_pair>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()>,
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
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut subtable_gpos_cursive>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, otl_GposCursiveEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> otl_GposCursiveEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> ()>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_subtable_gpos_markToLigature {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_markToLigature,
            *const subtable_gpos_markToLigature,
        ) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_markToLigature,
            *mut subtable_gpos_markToLigature,
        ) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> ()>,
    pub replace: Option<
        unsafe extern "C" fn(*mut subtable_gpos_markToLigature, subtable_gpos_markToLigature) -> (),
    >,
    pub copyReplace: Option<
        unsafe extern "C" fn(*mut subtable_gpos_markToLigature, subtable_gpos_markToLigature) -> (),
    >,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_markToLigature>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> ()>,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_SubtablePtr {
    pub init: Option<unsafe extern "C" fn(*mut otl_SubtablePtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_SubtablePtr, *const otl_SubtablePtr) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_SubtablePtr, *mut otl_SubtablePtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_SubtablePtr) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_SubtablePtr, otl_SubtablePtr) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_SubtablePtr, otl_SubtablePtr) -> ()>,
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
pub struct __caryll_elementinterface_otl_LookupRef {
    pub init: Option<unsafe extern "C" fn(*mut otl_LookupRef) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LookupRef, *const otl_LookupRef) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LookupRef, *mut otl_LookupRef) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LookupRef) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LookupRef, otl_LookupRef) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LookupRef, otl_LookupRef) -> ()>,
}
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
pub struct __caryll_elementinterface_otl_FeatureRef {
    pub init: Option<unsafe extern "C" fn(*mut otl_FeatureRef) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_FeatureRef, *const otl_FeatureRef) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_FeatureRef, *mut otl_FeatureRef) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_FeatureRef) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_FeatureRef, otl_FeatureRef) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_FeatureRef, otl_FeatureRef) -> ()>,
}
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
#[inline]
unsafe extern "C" fn disposeSubtableDependent(
    mut subtableRef: *mut otl_SubtablePtr,
    mut lookup: *const otl_Lookup,
) {
    match (*lookup).type_0 {
        otl_type_gsub_single => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gsub_single.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gsub_multiple => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gsub_multi.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gsub_alternate => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gsub_multi.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gsub_ligature => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gsub_ligature.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gsub_chaining => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_chaining.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gsub_reverse => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gsub_reverse) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gsub_reverse.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_single => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gpos_single.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_pair => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gpos_pair.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_cursive => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gpos_cursive.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_chaining => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_chaining.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_markToBase => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gpos_markToSingle.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_markToMark => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gpos_markToSingle.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_markToLigature => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> ()>,
                Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>,
            >(iSubtable_gpos_markToLigature.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        _ => {}
    };
}
static otl_iSubtablePtr: __caryll_elementinterface_otl_SubtablePtr =
    __caryll_elementinterface_otl_SubtablePtr {
        init: None,
        copy: None,
        move_0: None,
        dispose: None,
        replace: None,
        copyReplace: None,
    };
#[inline]
unsafe extern "C" fn otl_SubtableList_disposeDependent(
    mut arr: *mut otl_SubtableList,
    mut enclosure: *const otl_Lookup,
) {
    if arr.is_null() {
        return;
    }
    let mut j: usize = (*arr).length;
    loop {
        let fresh0 = j;
        j = j.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        disposeSubtableDependent(
            (*arr).items.offset(j as isize) as *mut otl_SubtablePtr,
            enclosure,
        );
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_SubtablePtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_SubtableList_filterEnv(
    mut arr: *mut otl_SubtableList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_SubtablePtr, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_SubtablePtr,
            env,
        ) {
            if j != k {
                let ref mut fresh1 = *(*arr).items.offset(j as isize);
                *fresh1 = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otl_iSubtablePtr.dispose.is_some() {
                otl_iSubtablePtr.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_SubtablePtr,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_SubtableList_createN(mut n: usize) -> *mut otl_SubtableList {
    let mut t: *mut otl_SubtableList =
        malloc(::core::mem::size_of::<otl_SubtableList>() as usize) as *mut otl_SubtableList;
    otl_SubtableList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_SubtableList_sort(
    mut arr: *mut otl_SubtableList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_SubtablePtr, *const otl_SubtablePtr) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_SubtablePtr>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_SubtablePtr,
                    *const otl_SubtablePtr,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_SubtableList_shrinkToFit(mut arr: *mut otl_SubtableList) {
    otl_SubtableList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_resizeTo(arr: *mut otl_SubtableList, target: usize) {
    cvec_resize_to(otl_SubtableList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_move(dst: *mut otl_SubtableList, src: *mut otl_SubtableList) {
    cvec_move(otl_SubtableList_as_cvec(dst), otl_SubtableList_as_cvec(src));
}
#[inline]
unsafe fn otl_SubtableList_as_cvec(arr: *mut otl_SubtableList) -> *mut CVecRaw<otl_SubtablePtr> {
    arr as *mut CVecRaw<otl_SubtablePtr>
}
#[inline]
unsafe extern "C" fn otl_SubtableList_init(arr: *mut otl_SubtableList) {
    cvec_init(otl_SubtableList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_SubtableList_free(mut x: *mut otl_SubtableList) {
    if x.is_null() {
        return;
    }
    otl_SubtableList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_create() -> *mut otl_SubtableList {
    let mut x: *mut otl_SubtableList =
        malloc(::core::mem::size_of::<otl_SubtableList>() as usize) as *mut otl_SubtableList;
    otl_SubtableList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_SubtableList_fill(mut arr: *mut otl_SubtableList, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_SubtablePtr = ::core::ptr::null_mut::<otl_Subtable>();
        if otl_iSubtablePtr.init.is_some() {
            otl_iSubtablePtr.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_SubtablePtr>() as usize,
            );
        }
        otl_SubtableList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_SubtableList_disposeItem(mut arr: *mut otl_SubtableList, mut n: usize) {
    if otl_iSubtablePtr.dispose.is_some() {
        otl_iSubtablePtr.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_SubtablePtr,
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_SubtableList_push(arr: *mut otl_SubtableList, elem: otl_SubtablePtr) {
    cvec_push(otl_SubtableList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_grow(arr: *mut otl_SubtableList) {
    cvec_grow(otl_SubtableList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_SubtableList_growTo(arr: *mut otl_SubtableList, target: usize) {
    cvec_grow_to(otl_SubtableList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_pop(arr: *mut otl_SubtableList) -> otl_SubtablePtr {
    cvec_pop(otl_SubtableList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_SubtableList_copyReplace(
    mut dst: *mut otl_SubtableList,
    src: otl_SubtableList,
) {
    otl_SubtableList_dispose(dst);
    otl_SubtableList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_copy(
    mut dst: *mut otl_SubtableList,
    mut src: *const otl_SubtableList,
) {
    otl_SubtableList_init(dst);
    otl_SubtableList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iSubtablePtr.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iSubtablePtr.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_SubtablePtr,
                (*src).items.offset(j as isize) as *mut otl_SubtablePtr as *const otl_SubtablePtr,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            let ref mut fresh4 = *(*dst).items.offset(j_0 as isize);
            *fresh4 = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn otl_SubtableList_dispose(mut arr: *mut otl_SubtableList) {
    if arr.is_null() {
        return;
    }
    if otl_iSubtablePtr.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh5 = j;
            j = j.wrapping_sub(1);
            if !(fresh5 != 0) {
                break;
            }
            otl_iSubtablePtr.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_SubtablePtr,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_SubtablePtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_SubtableList_replace(
    mut dst: *mut otl_SubtableList,
    src: otl_SubtableList,
) {
    otl_SubtableList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_SubtableList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_SubtableList_initCapN(mut arr: *mut otl_SubtableList, mut n: usize) {
    otl_SubtableList_init(arr);
    otl_SubtableList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_growToN(arr: *mut otl_SubtableList, target: usize) {
    cvec_grow_to_n(otl_SubtableList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_initN(mut arr: *mut otl_SubtableList, mut n: usize) {
    otl_SubtableList_init(arr);
    otl_SubtableList_growToN(arr, n);
    otl_SubtableList_fill(arr, n);
}
#[unsafe(no_mangle)]
pub static otl_iSubtableList: __caryll_vectorinterface_otl_SubtableList = {
    __caryll_vectorinterface_otl_SubtableList {
        init: Some(otl_SubtableList_init as unsafe extern "C" fn(*mut otl_SubtableList) -> ()),
        copy: Some(
            otl_SubtableList_copy
                as unsafe extern "C" fn(*mut otl_SubtableList, *const otl_SubtableList) -> (),
        ),
        move_0: Some(
            otl_SubtableList_move
                as unsafe extern "C" fn(*mut otl_SubtableList, *mut otl_SubtableList) -> (),
        ),
        dispose: Some(
            otl_SubtableList_dispose as unsafe extern "C" fn(*mut otl_SubtableList) -> (),
        ),
        replace: Some(
            otl_SubtableList_replace
                as unsafe extern "C" fn(*mut otl_SubtableList, otl_SubtableList) -> (),
        ),
        copyReplace: Some(
            otl_SubtableList_copyReplace
                as unsafe extern "C" fn(*mut otl_SubtableList, otl_SubtableList) -> (),
        ),
        create: Some(otl_SubtableList_create),
        free: Some(otl_SubtableList_free as unsafe extern "C" fn(*mut otl_SubtableList) -> ()),
        initN: Some(
            otl_SubtableList_initN as unsafe extern "C" fn(*mut otl_SubtableList, usize) -> (),
        ),
        initCapN: Some(
            otl_SubtableList_initCapN as unsafe extern "C" fn(*mut otl_SubtableList, usize) -> (),
        ),
        createN: Some(
            otl_SubtableList_createN as unsafe extern "C" fn(usize) -> *mut otl_SubtableList,
        ),
        fill: Some(
            otl_SubtableList_fill as unsafe extern "C" fn(*mut otl_SubtableList, usize) -> (),
        ),
        clear: Some(otl_SubtableList_dispose as unsafe extern "C" fn(*mut otl_SubtableList) -> ()),
        push: Some(
            otl_SubtableList_push
                as unsafe extern "C" fn(*mut otl_SubtableList, otl_SubtablePtr) -> (),
        ),
        shrinkToFit: Some(
            otl_SubtableList_shrinkToFit as unsafe extern "C" fn(*mut otl_SubtableList) -> (),
        ),
        pop: Some(
            otl_SubtableList_pop as unsafe extern "C" fn(*mut otl_SubtableList) -> otl_SubtablePtr,
        ),
        disposeItem: Some(
            otl_SubtableList_disposeItem
                as unsafe extern "C" fn(*mut otl_SubtableList, usize) -> (),
        ),
        filterEnv: Some(
            otl_SubtableList_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_SubtableList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_SubtablePtr,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_SubtableList_sort
                as unsafe extern "C" fn(
                    *mut otl_SubtableList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_SubtablePtr,
                            *const otl_SubtablePtr,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
        disposeDependent: Some(
            otl_SubtableList_disposeDependent
                as unsafe extern "C" fn(*mut otl_SubtableList, *const otl_Lookup) -> (),
        ),
    }
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_delete_lookup(mut lookup: *mut otl_Lookup) {
    if lookup.is_null() {
        return;
    }
    otl_iSubtableList
        .disposeDependent
        .expect("non-null function pointer")(&raw mut (*lookup).subtables, lookup);
    sdsfree((*lookup).name);
    free(lookup as *mut ::core::ffi::c_void);
    lookup = ::core::ptr::null_mut::<otl_Lookup>();
}
#[inline]
unsafe extern "C" fn initLookupPtr(mut entry: *mut otl_LookupPtr) {
    *entry = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_Lookup>() as usize,
        47 as ::core::ffi::c_ulong,
    ) as otl_LookupPtr;
    (**entry).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    otl_iSubtableList.init.expect("non-null function pointer")(&raw mut (**entry).subtables);
}
#[inline]
unsafe extern "C" fn disposeLookupPtr(mut entry: *mut otl_LookupPtr) {
    otfcc_delete_lookup(*entry);
}
#[unsafe(no_mangle)]
pub static otl_iLookupPtr: __caryll_elementinterface_otl_LookupPtr = {
    __caryll_elementinterface_otl_LookupPtr {
        init: Some(otl_LookupPtr_init as unsafe extern "C" fn(*mut otl_LookupPtr) -> ()),
        copy: Some(
            otl_LookupPtr_copy
                as unsafe extern "C" fn(*mut otl_LookupPtr, *const otl_LookupPtr) -> (),
        ),
        move_0: Some(
            otl_LookupPtr_move
                as unsafe extern "C" fn(*mut otl_LookupPtr, *mut otl_LookupPtr) -> (),
        ),
        dispose: Some(otl_LookupPtr_dispose as unsafe extern "C" fn(*mut otl_LookupPtr) -> ()),
        replace: Some(
            otl_LookupPtr_replace as unsafe extern "C" fn(*mut otl_LookupPtr, otl_LookupPtr) -> (),
        ),
        copyReplace: Some(
            otl_LookupPtr_copyReplace
                as unsafe extern "C" fn(*mut otl_LookupPtr, otl_LookupPtr) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LookupPtr_dispose(mut x: *mut otl_LookupPtr) {
    disposeLookupPtr(x);
}
#[inline]
unsafe extern "C" fn otl_LookupPtr_copy(
    mut dst: *mut otl_LookupPtr,
    mut src: *const otl_LookupPtr,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LookupPtr>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupPtr_copyReplace(mut dst: *mut otl_LookupPtr, src: otl_LookupPtr) {
    otl_LookupPtr_dispose(dst);
    otl_LookupPtr_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LookupPtr_replace(mut dst: *mut otl_LookupPtr, src: otl_LookupPtr) {
    otl_LookupPtr_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LookupPtr>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupPtr_move(mut dst: *mut otl_LookupPtr, mut src: *mut otl_LookupPtr) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LookupPtr>() as usize,
    );
    otl_LookupPtr_init(src);
}
#[inline]
unsafe extern "C" fn otl_LookupPtr_init(mut x: *mut otl_LookupPtr) {
    initLookupPtr(x);
}
#[inline]
unsafe extern "C" fn otl_LookupList_resizeTo(arr: *mut otl_LookupList, target: usize) {
    cvec_resize_to(otl_LookupList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LookupList_shrinkToFit(mut arr: *mut otl_LookupList) {
    otl_LookupList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_LookupList_move(dst: *mut otl_LookupList, src: *mut otl_LookupList) {
    cvec_move(otl_LookupList_as_cvec(dst), otl_LookupList_as_cvec(src));
}
#[inline]
unsafe fn otl_LookupList_as_cvec(arr: *mut otl_LookupList) -> *mut CVecRaw<otl_LookupPtr> {
    arr as *mut CVecRaw<otl_LookupPtr>
}
#[inline]
unsafe extern "C" fn otl_LookupList_init(arr: *mut otl_LookupList) {
    cvec_init(otl_LookupList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LookupList_filterEnv(
    mut arr: *mut otl_LookupList,
    mut fn_0: Option<unsafe extern "C" fn(*const otl_LookupPtr, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_LookupPtr,
            env,
        ) {
            if j != k {
                let ref mut fresh6 = *(*arr).items.offset(j as isize);
                *fresh6 = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otl_iLookupPtr.dispose.is_some() {
                otl_iLookupPtr.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_LookupPtr,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_LookupList_disposeItem(mut arr: *mut otl_LookupList, mut n: usize) {
    if otl_iLookupPtr.dispose.is_some() {
        otl_iLookupPtr.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_LookupPtr
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_LookupList_sort(
    mut arr: *mut otl_LookupList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_LookupPtr, *const otl_LookupPtr) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_LookupPtr>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_LookupPtr,
                    *const otl_LookupPtr,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_LookupList_fill(mut arr: *mut otl_LookupList, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_LookupPtr = ::core::ptr::null_mut::<otl_Lookup>();
        if otl_iLookupPtr.init.is_some() {
            otl_iLookupPtr.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_LookupPtr>() as usize,
            );
        }
        otl_LookupList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_LookupList_push(arr: *mut otl_LookupList, elem: otl_LookupPtr) {
    cvec_push(otl_LookupList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_LookupList_grow(arr: *mut otl_LookupList) {
    cvec_grow(otl_LookupList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LookupList_growTo(arr: *mut otl_LookupList, target: usize) {
    cvec_grow_to(otl_LookupList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LookupList_pop(arr: *mut otl_LookupList) -> otl_LookupPtr {
    cvec_pop(otl_LookupList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_LookupList_copyReplace(mut dst: *mut otl_LookupList, src: otl_LookupList) {
    otl_LookupList_dispose(dst);
    otl_LookupList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LookupList_copy(
    mut dst: *mut otl_LookupList,
    mut src: *const otl_LookupList,
) {
    otl_LookupList_init(dst);
    otl_LookupList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iLookupPtr.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iLookupPtr.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_LookupPtr,
                (*src).items.offset(j as isize) as *mut otl_LookupPtr as *const otl_LookupPtr,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            let ref mut fresh9 = *(*dst).items.offset(j_0 as isize);
            *fresh9 = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn otl_LookupList_dispose(mut arr: *mut otl_LookupList) {
    if arr.is_null() {
        return;
    }
    if otl_iLookupPtr.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh10 = j;
            j = j.wrapping_sub(1);
            if !(fresh10 != 0) {
                break;
            }
            otl_iLookupPtr.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_LookupPtr,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_LookupPtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_LookupList_replace(mut dst: *mut otl_LookupList, src: otl_LookupList) {
    otl_LookupList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LookupList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupList_initCapN(mut arr: *mut otl_LookupList, mut n: usize) {
    otl_LookupList_init(arr);
    otl_LookupList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LookupList_growToN(arr: *mut otl_LookupList, target: usize) {
    cvec_grow_to_n(otl_LookupList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LookupList_initN(mut arr: *mut otl_LookupList, mut n: usize) {
    otl_LookupList_init(arr);
    otl_LookupList_growToN(arr, n);
    otl_LookupList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LookupList_free(mut x: *mut otl_LookupList) {
    if x.is_null() {
        return;
    }
    otl_LookupList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_LookupList_createN(mut n: usize) -> *mut otl_LookupList {
    let mut t: *mut otl_LookupList =
        malloc(::core::mem::size_of::<otl_LookupList>() as usize) as *mut otl_LookupList;
    otl_LookupList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_LookupList_create() -> *mut otl_LookupList {
    let mut x: *mut otl_LookupList =
        malloc(::core::mem::size_of::<otl_LookupList>() as usize) as *mut otl_LookupList;
    otl_LookupList_init(x);
    return x;
}
#[unsafe(no_mangle)]
pub static otl_iLookupList: __caryll_vectorinterface_otl_LookupList = {
    __caryll_vectorinterface_otl_LookupList {
        init: Some(otl_LookupList_init as unsafe extern "C" fn(*mut otl_LookupList) -> ()),
        copy: Some(
            otl_LookupList_copy
                as unsafe extern "C" fn(*mut otl_LookupList, *const otl_LookupList) -> (),
        ),
        move_0: Some(
            otl_LookupList_move
                as unsafe extern "C" fn(*mut otl_LookupList, *mut otl_LookupList) -> (),
        ),
        dispose: Some(otl_LookupList_dispose as unsafe extern "C" fn(*mut otl_LookupList) -> ()),
        replace: Some(
            otl_LookupList_replace
                as unsafe extern "C" fn(*mut otl_LookupList, otl_LookupList) -> (),
        ),
        copyReplace: Some(
            otl_LookupList_copyReplace
                as unsafe extern "C" fn(*mut otl_LookupList, otl_LookupList) -> (),
        ),
        create: Some(otl_LookupList_create),
        free: Some(otl_LookupList_free as unsafe extern "C" fn(*mut otl_LookupList) -> ()),
        initN: Some(
            otl_LookupList_initN as unsafe extern "C" fn(*mut otl_LookupList, usize) -> (),
        ),
        initCapN: Some(
            otl_LookupList_initCapN as unsafe extern "C" fn(*mut otl_LookupList, usize) -> (),
        ),
        createN: Some(
            otl_LookupList_createN as unsafe extern "C" fn(usize) -> *mut otl_LookupList,
        ),
        fill: Some(otl_LookupList_fill as unsafe extern "C" fn(*mut otl_LookupList, usize) -> ()),
        clear: Some(otl_LookupList_dispose as unsafe extern "C" fn(*mut otl_LookupList) -> ()),
        push: Some(
            otl_LookupList_push as unsafe extern "C" fn(*mut otl_LookupList, otl_LookupPtr) -> (),
        ),
        shrinkToFit: Some(
            otl_LookupList_shrinkToFit as unsafe extern "C" fn(*mut otl_LookupList) -> (),
        ),
        pop: Some(otl_LookupList_pop as unsafe extern "C" fn(*mut otl_LookupList) -> otl_LookupPtr),
        disposeItem: Some(
            otl_LookupList_disposeItem as unsafe extern "C" fn(*mut otl_LookupList, usize) -> (),
        ),
        filterEnv: Some(
            otl_LookupList_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_LookupList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_LookupPtr,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_LookupList_sort
                as unsafe extern "C" fn(
                    *mut otl_LookupList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_LookupPtr,
                            *const otl_LookupPtr,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LookupRef_dispose(mut _x: *mut otl_LookupRef) {}
#[inline]
unsafe extern "C" fn otl_LookupRef_copyReplace(mut dst: *mut otl_LookupRef, src: otl_LookupRef) {
    otl_LookupRef_dispose(dst);
    otl_LookupRef_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LookupRef_move(mut dst: *mut otl_LookupRef, mut src: *mut otl_LookupRef) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LookupRef>() as usize,
    );
    otl_LookupRef_init(src);
}
#[inline]
unsafe extern "C" fn otl_LookupRef_init(mut x: *mut otl_LookupRef) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<otl_LookupRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupRef_copy(
    mut dst: *mut otl_LookupRef,
    mut src: *const otl_LookupRef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LookupRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupRef_replace(mut dst: *mut otl_LookupRef, src: otl_LookupRef) {
    otl_LookupRef_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LookupRef>() as usize,
    );
}
#[unsafe(no_mangle)]
pub static otl_iLookupRef: __caryll_elementinterface_otl_LookupRef = {
    __caryll_elementinterface_otl_LookupRef {
        init: Some(otl_LookupRef_init as unsafe extern "C" fn(*mut otl_LookupRef) -> ()),
        copy: Some(
            otl_LookupRef_copy
                as unsafe extern "C" fn(*mut otl_LookupRef, *const otl_LookupRef) -> (),
        ),
        move_0: Some(
            otl_LookupRef_move
                as unsafe extern "C" fn(*mut otl_LookupRef, *mut otl_LookupRef) -> (),
        ),
        dispose: Some(otl_LookupRef_dispose as unsafe extern "C" fn(*mut otl_LookupRef) -> ()),
        replace: Some(
            otl_LookupRef_replace as unsafe extern "C" fn(*mut otl_LookupRef, otl_LookupRef) -> (),
        ),
        copyReplace: Some(
            otl_LookupRef_copyReplace
                as unsafe extern "C" fn(*mut otl_LookupRef, otl_LookupRef) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LookupRefList_pop(arr: *mut otl_LookupRefList) -> otl_LookupRef {
    cvec_pop(otl_LookupRefList_as_cvec(arr))
}
#[unsafe(no_mangle)]
pub static otl_iLookupRefList: __caryll_vectorinterface_otl_LookupRefList = {
    __caryll_vectorinterface_otl_LookupRefList {
        init: Some(otl_LookupRefList_init as unsafe extern "C" fn(*mut otl_LookupRefList) -> ()),
        copy: Some(
            otl_LookupRefList_copy
                as unsafe extern "C" fn(*mut otl_LookupRefList, *const otl_LookupRefList) -> (),
        ),
        move_0: Some(
            otl_LookupRefList_move
                as unsafe extern "C" fn(*mut otl_LookupRefList, *mut otl_LookupRefList) -> (),
        ),
        dispose: Some(
            otl_LookupRefList_dispose as unsafe extern "C" fn(*mut otl_LookupRefList) -> (),
        ),
        replace: Some(
            otl_LookupRefList_replace
                as unsafe extern "C" fn(*mut otl_LookupRefList, otl_LookupRefList) -> (),
        ),
        copyReplace: Some(
            otl_LookupRefList_copyReplace
                as unsafe extern "C" fn(*mut otl_LookupRefList, otl_LookupRefList) -> (),
        ),
        create: Some(otl_LookupRefList_create),
        free: Some(otl_LookupRefList_free as unsafe extern "C" fn(*mut otl_LookupRefList) -> ()),
        initN: Some(
            otl_LookupRefList_initN as unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> (),
        ),
        initCapN: Some(
            otl_LookupRefList_initCapN
                as unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> (),
        ),
        createN: Some(
            otl_LookupRefList_createN as unsafe extern "C" fn(usize) -> *mut otl_LookupRefList,
        ),
        fill: Some(
            otl_LookupRefList_fill as unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> (),
        ),
        clear: Some(
            otl_LookupRefList_dispose as unsafe extern "C" fn(*mut otl_LookupRefList) -> (),
        ),
        push: Some(
            otl_LookupRefList_push
                as unsafe extern "C" fn(*mut otl_LookupRefList, otl_LookupRef) -> (),
        ),
        shrinkToFit: Some(
            otl_LookupRefList_shrinkToFit as unsafe extern "C" fn(*mut otl_LookupRefList) -> (),
        ),
        pop: Some(
            otl_LookupRefList_pop as unsafe extern "C" fn(*mut otl_LookupRefList) -> otl_LookupRef,
        ),
        disposeItem: Some(
            otl_LookupRefList_disposeItem
                as unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> (),
        ),
        filterEnv: Some(
            otl_LookupRefList_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_LookupRefList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_LookupRef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_LookupRefList_sort
                as unsafe extern "C" fn(
                    *mut otl_LookupRefList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_LookupRef,
                            *const otl_LookupRef,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LookupRefList_shrinkToFit(mut arr: *mut otl_LookupRefList) {
    otl_LookupRefList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_resizeTo(arr: *mut otl_LookupRefList, target: usize) {
    cvec_resize_to(otl_LookupRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_move(dst: *mut otl_LookupRefList, src: *mut otl_LookupRefList) {
    cvec_move(otl_LookupRefList_as_cvec(dst), otl_LookupRefList_as_cvec(src));
}
#[inline]
unsafe fn otl_LookupRefList_as_cvec(arr: *mut otl_LookupRefList) -> *mut CVecRaw<otl_LookupRef> {
    arr as *mut CVecRaw<otl_LookupRef>
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_init(arr: *mut otl_LookupRefList) {
    cvec_init(otl_LookupRefList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_filterEnv(
    mut arr: *mut otl_LookupRefList,
    mut fn_0: Option<unsafe extern "C" fn(*const otl_LookupRef, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_LookupRef,
            env,
        ) {
            if j != k {
                let ref mut fresh11 = *(*arr).items.offset(j as isize);
                *fresh11 = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otl_iLookupRef.dispose.is_some() {
                otl_iLookupRef.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_LookupRef,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_disposeItem(mut arr: *mut otl_LookupRefList, mut n: usize) {
    if otl_iLookupRef.dispose.is_some() {
        otl_iLookupRef.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_LookupRef
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_sort(
    mut arr: *mut otl_LookupRefList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_LookupRef, *const otl_LookupRef) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_LookupRef>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_LookupRef,
                    *const otl_LookupRef,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_fill(mut arr: *mut otl_LookupRefList, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_LookupRef = ::core::ptr::null::<otl_Lookup>();
        if otl_iLookupRef.init.is_some() {
            otl_iLookupRef.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_LookupRef>() as usize,
            );
        }
        otl_LookupRefList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_push(arr: *mut otl_LookupRefList, elem: otl_LookupRef) {
    cvec_push(otl_LookupRefList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_createN(mut n: usize) -> *mut otl_LookupRefList {
    let mut t: *mut otl_LookupRefList =
        malloc(::core::mem::size_of::<otl_LookupRefList>() as usize) as *mut otl_LookupRefList;
    otl_LookupRefList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_grow(arr: *mut otl_LookupRefList) {
    cvec_grow(otl_LookupRefList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_growTo(arr: *mut otl_LookupRefList, target: usize) {
    cvec_grow_to(otl_LookupRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_create() -> *mut otl_LookupRefList {
    let mut x: *mut otl_LookupRefList =
        malloc(::core::mem::size_of::<otl_LookupRefList>() as usize) as *mut otl_LookupRefList;
    otl_LookupRefList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_copyReplace(
    mut dst: *mut otl_LookupRefList,
    src: otl_LookupRefList,
) {
    otl_LookupRefList_dispose(dst);
    otl_LookupRefList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_copy(
    mut dst: *mut otl_LookupRefList,
    mut src: *const otl_LookupRefList,
) {
    otl_LookupRefList_init(dst);
    otl_LookupRefList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iLookupRef.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iLookupRef.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_LookupRef,
                (*src).items.offset(j as isize) as *mut otl_LookupRef as *const otl_LookupRef,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            let ref mut fresh14 = *(*dst).items.offset(j_0 as isize);
            *fresh14 = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_dispose(mut arr: *mut otl_LookupRefList) {
    if arr.is_null() {
        return;
    }
    if otl_iLookupRef.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh15 = j;
            j = j.wrapping_sub(1);
            if !(fresh15 != 0) {
                break;
            }
            otl_iLookupRef.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_LookupRef,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_LookupRef>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_replace(
    mut dst: *mut otl_LookupRefList,
    src: otl_LookupRefList,
) {
    otl_LookupRefList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LookupRefList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_initCapN(mut arr: *mut otl_LookupRefList, mut n: usize) {
    otl_LookupRefList_init(arr);
    otl_LookupRefList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_initN(mut arr: *mut otl_LookupRefList, mut n: usize) {
    otl_LookupRefList_init(arr);
    otl_LookupRefList_growToN(arr, n);
    otl_LookupRefList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_free(mut x: *mut otl_LookupRefList) {
    if x.is_null() {
        return;
    }
    otl_LookupRefList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_growToN(arr: *mut otl_LookupRefList, target: usize) {
    cvec_grow_to_n(otl_LookupRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn initFeaturePtr(mut feature: *mut otl_FeaturePtr) {
    *feature = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_Feature>() as usize,
        61 as ::core::ffi::c_ulong,
    ) as otl_FeaturePtr;
    otl_iLookupRefList.init.expect("non-null function pointer")(&raw mut (**feature).lookups);
}
#[inline]
unsafe extern "C" fn disposeFeaturePtr(mut feature: *mut otl_FeaturePtr) {
    if (*feature).is_null() {
        return;
    }
    if !(**feature).name.is_null() {
        sdsfree((**feature).name);
    }
    otl_iLookupRefList
        .dispose
        .expect("non-null function pointer")(&raw mut (**feature).lookups);
    free(*feature as *mut ::core::ffi::c_void);
    *feature = ::core::ptr::null_mut::<otl_Feature>();
}
#[inline]
unsafe extern "C" fn otl_FeaturePtr_dispose(mut x: *mut otl_FeaturePtr) {
    disposeFeaturePtr(x);
}
#[inline]
unsafe extern "C" fn otl_FeaturePtr_copy(
    mut dst: *mut otl_FeaturePtr,
    mut src: *const otl_FeaturePtr,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_FeaturePtr>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeaturePtr_replace(mut dst: *mut otl_FeaturePtr, src: otl_FeaturePtr) {
    otl_FeaturePtr_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_FeaturePtr>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeaturePtr_move(
    mut dst: *mut otl_FeaturePtr,
    mut src: *mut otl_FeaturePtr,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_FeaturePtr>() as usize,
    );
    otl_FeaturePtr_init(src);
}
#[inline]
unsafe extern "C" fn otl_FeaturePtr_init(mut x: *mut otl_FeaturePtr) {
    initFeaturePtr(x);
}
#[unsafe(no_mangle)]
pub static otl_iFeaturePtr: __caryll_elementinterface_otl_FeaturePtr = {
    __caryll_elementinterface_otl_FeaturePtr {
        init: Some(otl_FeaturePtr_init as unsafe extern "C" fn(*mut otl_FeaturePtr) -> ()),
        copy: Some(
            otl_FeaturePtr_copy
                as unsafe extern "C" fn(*mut otl_FeaturePtr, *const otl_FeaturePtr) -> (),
        ),
        move_0: Some(
            otl_FeaturePtr_move
                as unsafe extern "C" fn(*mut otl_FeaturePtr, *mut otl_FeaturePtr) -> (),
        ),
        dispose: Some(otl_FeaturePtr_dispose as unsafe extern "C" fn(*mut otl_FeaturePtr) -> ()),
        replace: Some(
            otl_FeaturePtr_replace
                as unsafe extern "C" fn(*mut otl_FeaturePtr, otl_FeaturePtr) -> (),
        ),
        copyReplace: Some(
            otl_FeaturePtr_copyReplace
                as unsafe extern "C" fn(*mut otl_FeaturePtr, otl_FeaturePtr) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_FeaturePtr_copyReplace(mut dst: *mut otl_FeaturePtr, src: otl_FeaturePtr) {
    otl_FeaturePtr_dispose(dst);
    otl_FeaturePtr_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_fill(mut arr: *mut otl_FeatureList, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_FeaturePtr = ::core::ptr::null_mut::<otl_Feature>();
        if otl_iFeaturePtr.init.is_some() {
            otl_iFeaturePtr.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_FeaturePtr>() as usize,
            );
        }
        otl_FeatureList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_FeatureList_growTo(arr: *mut otl_FeatureList, target: usize) {
    cvec_grow_to(otl_FeatureList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_growToN(arr: *mut otl_FeatureList, target: usize) {
    cvec_grow_to_n(otl_FeatureList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_initN(mut arr: *mut otl_FeatureList, mut n: usize) {
    otl_FeatureList_init(arr);
    otl_FeatureList_growToN(arr, n);
    otl_FeatureList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_free(mut x: *mut otl_FeatureList) {
    if x.is_null() {
        return;
    }
    otl_FeatureList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_createN(mut n: usize) -> *mut otl_FeatureList {
    let mut t: *mut otl_FeatureList =
        malloc(::core::mem::size_of::<otl_FeatureList>() as usize) as *mut otl_FeatureList;
    otl_FeatureList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_FeatureList_create() -> *mut otl_FeatureList {
    let mut x: *mut otl_FeatureList =
        malloc(::core::mem::size_of::<otl_FeatureList>() as usize) as *mut otl_FeatureList;
    otl_FeatureList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_FeatureList_sort(
    mut arr: *mut otl_FeatureList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_FeaturePtr, *const otl_FeaturePtr) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_FeaturePtr>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_FeaturePtr,
                    *const otl_FeaturePtr,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureList_push(arr: *mut otl_FeatureList, elem: otl_FeaturePtr) {
    cvec_push(otl_FeatureList_as_cvec(arr), elem);
}
#[inline]
unsafe fn otl_FeatureList_as_cvec(arr: *mut otl_FeatureList) -> *mut CVecRaw<otl_FeaturePtr> {
    arr as *mut CVecRaw<otl_FeaturePtr>
}
#[inline]
unsafe extern "C" fn otl_FeatureList_init(arr: *mut otl_FeatureList) {
    cvec_init(otl_FeatureList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_FeatureList_pop(arr: *mut otl_FeatureList) -> otl_FeaturePtr {
    cvec_pop(otl_FeatureList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_FeatureList_copyReplace(
    mut dst: *mut otl_FeatureList,
    src: otl_FeatureList,
) {
    otl_FeatureList_dispose(dst);
    otl_FeatureList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_copy(
    mut dst: *mut otl_FeatureList,
    mut src: *const otl_FeatureList,
) {
    otl_FeatureList_init(dst);
    otl_FeatureList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iFeaturePtr.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iFeaturePtr.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_FeaturePtr,
                (*src).items.offset(j as isize) as *mut otl_FeaturePtr as *const otl_FeaturePtr,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            let ref mut fresh19 = *(*dst).items.offset(j_0 as isize);
            *fresh19 = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn otl_FeatureList_grow(arr: *mut otl_FeatureList) {
    cvec_grow(otl_FeatureList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_FeatureList_dispose(mut arr: *mut otl_FeatureList) {
    if arr.is_null() {
        return;
    }
    if otl_iFeaturePtr.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh20 = j;
            j = j.wrapping_sub(1);
            if !(fresh20 != 0) {
                break;
            }
            otl_iFeaturePtr.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_FeaturePtr,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_FeaturePtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_FeatureList_replace(mut dst: *mut otl_FeatureList, src: otl_FeatureList) {
    otl_FeatureList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_FeatureList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureList_initCapN(mut arr: *mut otl_FeatureList, mut n: usize) {
    otl_FeatureList_init(arr);
    otl_FeatureList_growToN(arr, n);
}
#[unsafe(no_mangle)]
pub static otl_iFeatureList: __caryll_vectorinterface_otl_FeatureList = {
    __caryll_vectorinterface_otl_FeatureList {
        init: Some(otl_FeatureList_init as unsafe extern "C" fn(*mut otl_FeatureList) -> ()),
        copy: Some(
            otl_FeatureList_copy
                as unsafe extern "C" fn(*mut otl_FeatureList, *const otl_FeatureList) -> (),
        ),
        move_0: Some(
            otl_FeatureList_move
                as unsafe extern "C" fn(*mut otl_FeatureList, *mut otl_FeatureList) -> (),
        ),
        dispose: Some(otl_FeatureList_dispose as unsafe extern "C" fn(*mut otl_FeatureList) -> ()),
        replace: Some(
            otl_FeatureList_replace
                as unsafe extern "C" fn(*mut otl_FeatureList, otl_FeatureList) -> (),
        ),
        copyReplace: Some(
            otl_FeatureList_copyReplace
                as unsafe extern "C" fn(*mut otl_FeatureList, otl_FeatureList) -> (),
        ),
        create: Some(otl_FeatureList_create),
        free: Some(otl_FeatureList_free as unsafe extern "C" fn(*mut otl_FeatureList) -> ()),
        initN: Some(
            otl_FeatureList_initN as unsafe extern "C" fn(*mut otl_FeatureList, usize) -> (),
        ),
        initCapN: Some(
            otl_FeatureList_initCapN as unsafe extern "C" fn(*mut otl_FeatureList, usize) -> (),
        ),
        createN: Some(
            otl_FeatureList_createN as unsafe extern "C" fn(usize) -> *mut otl_FeatureList,
        ),
        fill: Some(
            otl_FeatureList_fill as unsafe extern "C" fn(*mut otl_FeatureList, usize) -> (),
        ),
        clear: Some(otl_FeatureList_dispose as unsafe extern "C" fn(*mut otl_FeatureList) -> ()),
        push: Some(
            otl_FeatureList_push
                as unsafe extern "C" fn(*mut otl_FeatureList, otl_FeaturePtr) -> (),
        ),
        shrinkToFit: Some(
            otl_FeatureList_shrinkToFit as unsafe extern "C" fn(*mut otl_FeatureList) -> (),
        ),
        pop: Some(
            otl_FeatureList_pop as unsafe extern "C" fn(*mut otl_FeatureList) -> otl_FeaturePtr,
        ),
        disposeItem: Some(
            otl_FeatureList_disposeItem as unsafe extern "C" fn(*mut otl_FeatureList, usize) -> (),
        ),
        filterEnv: Some(
            otl_FeatureList_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_FeatureList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_FeaturePtr,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_FeatureList_sort
                as unsafe extern "C" fn(
                    *mut otl_FeatureList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_FeaturePtr,
                            *const otl_FeaturePtr,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_FeatureList_shrinkToFit(mut arr: *mut otl_FeatureList) {
    otl_FeatureList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_resizeTo(arr: *mut otl_FeatureList, target: usize) {
    cvec_resize_to(otl_FeatureList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_move(dst: *mut otl_FeatureList, src: *mut otl_FeatureList) {
    cvec_move(otl_FeatureList_as_cvec(dst), otl_FeatureList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_FeatureList_filterEnv(
    mut arr: *mut otl_FeatureList,
    mut fn_0: Option<unsafe extern "C" fn(*const otl_FeaturePtr, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_FeaturePtr,
            env,
        ) {
            if j != k {
                let ref mut fresh16 = *(*arr).items.offset(j as isize);
                *fresh16 = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otl_iFeaturePtr.dispose.is_some() {
                otl_iFeaturePtr.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_FeaturePtr,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_FeatureList_disposeItem(mut arr: *mut otl_FeatureList, mut n: usize) {
    if otl_iFeaturePtr.dispose.is_some() {
        otl_iFeaturePtr.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_FeaturePtr,
        );
    } else {
    };
}
#[unsafe(no_mangle)]
pub static otl_iFeatureRef: __caryll_elementinterface_otl_FeatureRef = {
    __caryll_elementinterface_otl_FeatureRef {
        init: Some(otl_FeatureRef_init as unsafe extern "C" fn(*mut otl_FeatureRef) -> ()),
        copy: Some(
            otl_FeatureRef_copy
                as unsafe extern "C" fn(*mut otl_FeatureRef, *const otl_FeatureRef) -> (),
        ),
        move_0: Some(
            otl_FeatureRef_move
                as unsafe extern "C" fn(*mut otl_FeatureRef, *mut otl_FeatureRef) -> (),
        ),
        dispose: Some(otl_FeatureRef_dispose as unsafe extern "C" fn(*mut otl_FeatureRef) -> ()),
        replace: Some(
            otl_FeatureRef_replace
                as unsafe extern "C" fn(*mut otl_FeatureRef, otl_FeatureRef) -> (),
        ),
        copyReplace: Some(
            otl_FeatureRef_copyReplace
                as unsafe extern "C" fn(*mut otl_FeatureRef, otl_FeatureRef) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_FeatureRef_copyReplace(mut dst: *mut otl_FeatureRef, src: otl_FeatureRef) {
    otl_FeatureRef_dispose(dst);
    otl_FeatureRef_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_FeatureRef_copy(
    mut dst: *mut otl_FeatureRef,
    mut src: *const otl_FeatureRef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_FeatureRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureRef_dispose(mut _x: *mut otl_FeatureRef) {}
#[inline]
unsafe extern "C" fn otl_FeatureRef_replace(mut dst: *mut otl_FeatureRef, src: otl_FeatureRef) {
    otl_FeatureRef_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_FeatureRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureRef_move(
    mut dst: *mut otl_FeatureRef,
    mut src: *mut otl_FeatureRef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_FeatureRef>() as usize,
    );
    otl_FeatureRef_init(src);
}
#[inline]
unsafe extern "C" fn otl_FeatureRef_init(mut x: *mut otl_FeatureRef) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<otl_FeatureRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_fill(mut arr: *mut otl_FeatureRefList, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_FeatureRef = ::core::ptr::null::<otl_Feature>();
        if otl_iFeatureRef.init.is_some() {
            otl_iFeatureRef.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_FeatureRef>() as usize,
            );
        }
        otl_FeatureRefList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_growTo(arr: *mut otl_FeatureRefList, target: usize) {
    cvec_grow_to(otl_FeatureRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_pop(arr: *mut otl_FeatureRefList) -> otl_FeatureRef {
    cvec_pop(otl_FeatureRefList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_copyReplace(
    mut dst: *mut otl_FeatureRefList,
    src: otl_FeatureRefList,
) {
    otl_FeatureRefList_dispose(dst);
    otl_FeatureRefList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_copy(
    mut dst: *mut otl_FeatureRefList,
    mut src: *const otl_FeatureRefList,
) {
    otl_FeatureRefList_init(dst);
    otl_FeatureRefList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iFeatureRef.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iFeatureRef.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_FeatureRef,
                (*src).items.offset(j as isize) as *mut otl_FeatureRef as *const otl_FeatureRef,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            let ref mut fresh24 = *(*dst).items.offset(j_0 as isize);
            *fresh24 = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_dispose(mut arr: *mut otl_FeatureRefList) {
    if arr.is_null() {
        return;
    }
    if otl_iFeatureRef.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh25 = j;
            j = j.wrapping_sub(1);
            if !(fresh25 != 0) {
                break;
            }
            otl_iFeatureRef.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_FeatureRef,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_FeatureRef>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_replace(
    mut dst: *mut otl_FeatureRefList,
    src: otl_FeatureRefList,
) {
    otl_FeatureRefList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_FeatureRefList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_initCapN(mut arr: *mut otl_FeatureRefList, mut n: usize) {
    otl_FeatureRefList_init(arr);
    otl_FeatureRefList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_growToN(arr: *mut otl_FeatureRefList, target: usize) {
    cvec_grow_to_n(otl_FeatureRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_initN(mut arr: *mut otl_FeatureRefList, mut n: usize) {
    otl_FeatureRefList_init(arr);
    otl_FeatureRefList_growToN(arr, n);
    otl_FeatureRefList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_free(mut x: *mut otl_FeatureRefList) {
    if x.is_null() {
        return;
    }
    otl_FeatureRefList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_createN(mut n: usize) -> *mut otl_FeatureRefList {
    let mut t: *mut otl_FeatureRefList =
        malloc(::core::mem::size_of::<otl_FeatureRefList>() as usize) as *mut otl_FeatureRefList;
    otl_FeatureRefList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_create() -> *mut otl_FeatureRefList {
    let mut x: *mut otl_FeatureRefList =
        malloc(::core::mem::size_of::<otl_FeatureRefList>() as usize) as *mut otl_FeatureRefList;
    otl_FeatureRefList_init(x);
    return x;
}
#[unsafe(no_mangle)]
pub static otl_iFeatureRefList: __caryll_vectorinterface_otl_FeatureRefList = {
    __caryll_vectorinterface_otl_FeatureRefList {
        init: Some(otl_FeatureRefList_init as unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()),
        copy: Some(
            otl_FeatureRefList_copy
                as unsafe extern "C" fn(*mut otl_FeatureRefList, *const otl_FeatureRefList) -> (),
        ),
        move_0: Some(
            otl_FeatureRefList_move
                as unsafe extern "C" fn(*mut otl_FeatureRefList, *mut otl_FeatureRefList) -> (),
        ),
        dispose: Some(
            otl_FeatureRefList_dispose as unsafe extern "C" fn(*mut otl_FeatureRefList) -> (),
        ),
        replace: Some(
            otl_FeatureRefList_replace
                as unsafe extern "C" fn(*mut otl_FeatureRefList, otl_FeatureRefList) -> (),
        ),
        copyReplace: Some(
            otl_FeatureRefList_copyReplace
                as unsafe extern "C" fn(*mut otl_FeatureRefList, otl_FeatureRefList) -> (),
        ),
        create: Some(otl_FeatureRefList_create),
        free: Some(otl_FeatureRefList_free as unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()),
        initN: Some(
            otl_FeatureRefList_initN as unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> (),
        ),
        initCapN: Some(
            otl_FeatureRefList_initCapN
                as unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> (),
        ),
        createN: Some(
            otl_FeatureRefList_createN as unsafe extern "C" fn(usize) -> *mut otl_FeatureRefList,
        ),
        fill: Some(
            otl_FeatureRefList_fill as unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> (),
        ),
        clear: Some(
            otl_FeatureRefList_dispose as unsafe extern "C" fn(*mut otl_FeatureRefList) -> (),
        ),
        push: Some(
            otl_FeatureRefList_push
                as unsafe extern "C" fn(*mut otl_FeatureRefList, otl_FeatureRef) -> (),
        ),
        shrinkToFit: Some(
            otl_FeatureRefList_shrinkToFit as unsafe extern "C" fn(*mut otl_FeatureRefList) -> (),
        ),
        pop: Some(
            otl_FeatureRefList_pop
                as unsafe extern "C" fn(*mut otl_FeatureRefList) -> otl_FeatureRef,
        ),
        disposeItem: Some(
            otl_FeatureRefList_disposeItem
                as unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> (),
        ),
        filterEnv: Some(
            otl_FeatureRefList_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_FeatureRefList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_FeatureRef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_FeatureRefList_sort
                as unsafe extern "C" fn(
                    *mut otl_FeatureRefList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_FeatureRef,
                            *const otl_FeatureRef,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_FeatureRefList_shrinkToFit(mut arr: *mut otl_FeatureRefList) {
    otl_FeatureRefList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_resizeTo(arr: *mut otl_FeatureRefList, target: usize) {
    cvec_resize_to(otl_FeatureRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_move(dst: *mut otl_FeatureRefList, src: *mut otl_FeatureRefList) {
    cvec_move(otl_FeatureRefList_as_cvec(dst), otl_FeatureRefList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_filterEnv(
    mut arr: *mut otl_FeatureRefList,
    mut fn_0: Option<unsafe extern "C" fn(*const otl_FeatureRef, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_FeatureRef,
            env,
        ) {
            if j != k {
                let ref mut fresh21 = *(*arr).items.offset(j as isize);
                *fresh21 = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otl_iFeatureRef.dispose.is_some() {
                otl_iFeatureRef.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_FeatureRef,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_sort(
    mut arr: *mut otl_FeatureRefList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_FeatureRef, *const otl_FeatureRef) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_FeatureRef>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_FeatureRef,
                    *const otl_FeatureRef,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_push(arr: *mut otl_FeatureRefList, elem: otl_FeatureRef) {
    cvec_push(otl_FeatureRefList_as_cvec(arr), elem);
}
#[inline]
unsafe fn otl_FeatureRefList_as_cvec(arr: *mut otl_FeatureRefList) -> *mut CVecRaw<otl_FeatureRef> {
    arr as *mut CVecRaw<otl_FeatureRef>
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_init(arr: *mut otl_FeatureRefList) {
    cvec_init(otl_FeatureRefList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_grow(arr: *mut otl_FeatureRefList) {
    cvec_grow(otl_FeatureRefList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_disposeItem(
    mut arr: *mut otl_FeatureRefList,
    mut n: usize,
) {
    if otl_iFeatureRef.dispose.is_some() {
        otl_iFeatureRef.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_FeatureRef,
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn initLanguagePtr(mut language: *mut otl_LanguageSystemPtr) {
    *language = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_LanguageSystem>() as usize,
        77 as ::core::ffi::c_ulong,
    ) as otl_LanguageSystemPtr;
    otl_iFeatureRefList.init.expect("non-null function pointer")(&raw mut (**language).features);
}
#[inline]
unsafe extern "C" fn disposeLanguagePtr(mut language: *mut otl_LanguageSystemPtr) {
    if (*language).is_null() {
        return;
    }
    if !(**language).name.is_null() {
        sdsfree((**language).name);
    }
    otl_iFeatureRefList
        .dispose
        .expect("non-null function pointer")(&raw mut (**language).features);
    free(*language as *mut ::core::ffi::c_void);
    *language = ::core::ptr::null_mut::<otl_LanguageSystem>();
}
#[unsafe(no_mangle)]
pub static otl_iLanguageSystem: __caryll_elementinterface_otl_LanguageSystemPtr = {
    __caryll_elementinterface_otl_LanguageSystemPtr {
        init: Some(initLanguagePtr as unsafe extern "C" fn(*mut otl_LanguageSystemPtr) -> ()),
        copy: None,
        move_0: None,
        dispose: Some(disposeLanguagePtr as unsafe extern "C" fn(*mut otl_LanguageSystemPtr) -> ()),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn otl_LangSystemList_filterEnv(
    mut arr: *mut otl_LangSystemList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_LanguageSystemPtr, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_LanguageSystemPtr,
            env,
        ) {
            if j != k {
                let ref mut fresh26 = *(*arr).items.offset(j as isize);
                *fresh26 = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otl_iLanguageSystem.dispose.is_some() {
                otl_iLanguageSystem
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_LanguageSystemPtr,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn otl_LangSystemList_as_cvec(arr: *mut otl_LangSystemList) -> *mut CVecRaw<otl_LanguageSystemPtr> {
    arr as *mut CVecRaw<otl_LanguageSystemPtr>
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_init(arr: *mut otl_LangSystemList) {
    cvec_init(otl_LangSystemList_as_cvec(arr));
}
#[unsafe(no_mangle)]
pub static otl_iLangSystemList: __caryll_vectorinterface_otl_LangSystemList = {
    __caryll_vectorinterface_otl_LangSystemList {
        init: Some(otl_LangSystemList_init as unsafe extern "C" fn(*mut otl_LangSystemList) -> ()),
        copy: Some(
            otl_LangSystemList_copy
                as unsafe extern "C" fn(*mut otl_LangSystemList, *const otl_LangSystemList) -> (),
        ),
        move_0: Some(
            otl_LangSystemList_move
                as unsafe extern "C" fn(*mut otl_LangSystemList, *mut otl_LangSystemList) -> (),
        ),
        dispose: Some(
            otl_LangSystemList_dispose as unsafe extern "C" fn(*mut otl_LangSystemList) -> (),
        ),
        replace: Some(
            otl_LangSystemList_replace
                as unsafe extern "C" fn(*mut otl_LangSystemList, otl_LangSystemList) -> (),
        ),
        copyReplace: Some(
            otl_LangSystemList_copyReplace
                as unsafe extern "C" fn(*mut otl_LangSystemList, otl_LangSystemList) -> (),
        ),
        create: Some(otl_LangSystemList_create),
        free: Some(otl_LangSystemList_free as unsafe extern "C" fn(*mut otl_LangSystemList) -> ()),
        initN: Some(
            otl_LangSystemList_initN as unsafe extern "C" fn(*mut otl_LangSystemList, usize) -> (),
        ),
        initCapN: Some(
            otl_LangSystemList_initCapN
                as unsafe extern "C" fn(*mut otl_LangSystemList, usize) -> (),
        ),
        createN: Some(
            otl_LangSystemList_createN as unsafe extern "C" fn(usize) -> *mut otl_LangSystemList,
        ),
        fill: Some(
            otl_LangSystemList_fill as unsafe extern "C" fn(*mut otl_LangSystemList, usize) -> (),
        ),
        clear: Some(
            otl_LangSystemList_dispose as unsafe extern "C" fn(*mut otl_LangSystemList) -> (),
        ),
        push: Some(
            otl_LangSystemList_push
                as unsafe extern "C" fn(*mut otl_LangSystemList, otl_LanguageSystemPtr) -> (),
        ),
        shrinkToFit: Some(
            otl_LangSystemList_shrinkToFit as unsafe extern "C" fn(*mut otl_LangSystemList) -> (),
        ),
        pop: Some(
            otl_LangSystemList_pop
                as unsafe extern "C" fn(*mut otl_LangSystemList) -> otl_LanguageSystemPtr,
        ),
        disposeItem: Some(
            otl_LangSystemList_disposeItem
                as unsafe extern "C" fn(*mut otl_LangSystemList, usize) -> (),
        ),
        filterEnv: Some(
            otl_LangSystemList_filterEnv
                as unsafe extern "C" fn(
                    *mut otl_LangSystemList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_LanguageSystemPtr,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_LangSystemList_sort
                as unsafe extern "C" fn(
                    *mut otl_LangSystemList,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_LanguageSystemPtr,
                            *const otl_LanguageSystemPtr,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LangSystemList_disposeItem(
    mut arr: *mut otl_LangSystemList,
    mut n: usize,
) {
    if otl_iLanguageSystem.dispose.is_some() {
        otl_iLanguageSystem
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_LanguageSystemPtr
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_sort(
    mut arr: *mut otl_LangSystemList,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_LanguageSystemPtr,
            *const otl_LanguageSystemPtr,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_LanguageSystemPtr>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_LanguageSystemPtr,
                    *const otl_LanguageSystemPtr,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_fill(mut arr: *mut otl_LangSystemList, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_LanguageSystemPtr = ::core::ptr::null_mut::<otl_LanguageSystem>();
        if otl_iLanguageSystem.init.is_some() {
            otl_iLanguageSystem.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_LanguageSystemPtr>() as usize,
            );
        }
        otl_LangSystemList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_push(arr: *mut otl_LangSystemList, elem: otl_LanguageSystemPtr) {
    cvec_push(otl_LangSystemList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_grow(arr: *mut otl_LangSystemList) {
    cvec_grow(otl_LangSystemList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_growTo(arr: *mut otl_LangSystemList, target: usize) {
    cvec_grow_to(otl_LangSystemList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_pop(arr: *mut otl_LangSystemList) -> otl_LanguageSystemPtr {
    cvec_pop(otl_LangSystemList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_copyReplace(
    mut dst: *mut otl_LangSystemList,
    src: otl_LangSystemList,
) {
    otl_LangSystemList_dispose(dst);
    otl_LangSystemList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_copy(
    mut dst: *mut otl_LangSystemList,
    mut src: *const otl_LangSystemList,
) {
    otl_LangSystemList_init(dst);
    otl_LangSystemList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iLanguageSystem.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iLanguageSystem.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_LanguageSystemPtr,
                (*src).items.offset(j as isize) as *mut otl_LanguageSystemPtr
                    as *const otl_LanguageSystemPtr,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            let ref mut fresh29 = *(*dst).items.offset(j_0 as isize);
            *fresh29 = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_dispose(mut arr: *mut otl_LangSystemList) {
    if arr.is_null() {
        return;
    }
    if otl_iLanguageSystem.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh30 = j;
            j = j.wrapping_sub(1);
            if !(fresh30 != 0) {
                break;
            }
            otl_iLanguageSystem
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_LanguageSystemPtr
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_LanguageSystemPtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_replace(
    mut dst: *mut otl_LangSystemList,
    src: otl_LangSystemList,
) {
    otl_LangSystemList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_LangSystemList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_initCapN(mut arr: *mut otl_LangSystemList, mut n: usize) {
    otl_LangSystemList_init(arr);
    otl_LangSystemList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_growToN(arr: *mut otl_LangSystemList, target: usize) {
    cvec_grow_to_n(otl_LangSystemList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_initN(mut arr: *mut otl_LangSystemList, mut n: usize) {
    otl_LangSystemList_init(arr);
    otl_LangSystemList_growToN(arr, n);
    otl_LangSystemList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_free(mut x: *mut otl_LangSystemList) {
    if x.is_null() {
        return;
    }
    otl_LangSystemList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_createN(mut n: usize) -> *mut otl_LangSystemList {
    let mut t: *mut otl_LangSystemList =
        malloc(::core::mem::size_of::<otl_LangSystemList>() as usize) as *mut otl_LangSystemList;
    otl_LangSystemList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_create() -> *mut otl_LangSystemList {
    let mut x: *mut otl_LangSystemList =
        malloc(::core::mem::size_of::<otl_LangSystemList>() as usize) as *mut otl_LangSystemList;
    otl_LangSystemList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_shrinkToFit(mut arr: *mut otl_LangSystemList) {
    otl_LangSystemList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_resizeTo(arr: *mut otl_LangSystemList, target: usize) {
    cvec_resize_to(otl_LangSystemList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_move(dst: *mut otl_LangSystemList, src: *mut otl_LangSystemList) {
    cvec_move(otl_LangSystemList_as_cvec(dst), otl_LangSystemList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn initOTL(mut table: *mut table_OTL) {
    otl_iLookupList.init.expect("non-null function pointer")(&raw mut (*table).lookups);
    otl_iFeatureList.init.expect("non-null function pointer")(&raw mut (*table).features);
    otl_iLangSystemList.init.expect("non-null function pointer")(&raw mut (*table).languages);
}
#[inline]
unsafe extern "C" fn disposeOTL(mut table: *mut table_OTL) {
    otl_iLookupList.dispose.expect("non-null function pointer")(&raw mut (*table).lookups);
    otl_iFeatureList.dispose.expect("non-null function pointer")(&raw mut (*table).features);
    otl_iLangSystemList
        .dispose
        .expect("non-null function pointer")(&raw mut (*table).languages);
}
#[inline]
unsafe extern "C" fn table_OTL_dispose(mut x: *mut table_OTL) {
    disposeOTL(x);
}
#[inline]
unsafe extern "C" fn table_OTL_copyReplace(mut dst: *mut table_OTL, src: table_OTL) {
    table_OTL_dispose(dst);
    table_OTL_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_OTL_free(mut x: *mut table_OTL) {
    if x.is_null() {
        return;
    }
    table_OTL_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_OTL_create() -> *mut table_OTL {
    let mut x: *mut table_OTL =
        malloc(::core::mem::size_of::<table_OTL>() as usize) as *mut table_OTL;
    table_OTL_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_OTL_init(mut x: *mut table_OTL) {
    initOTL(x);
}
#[unsafe(no_mangle)]
pub static table_iOTL: __caryll_elementinterface_table_OTL = {
    __caryll_elementinterface_table_OTL {
        init: Some(table_OTL_init as unsafe extern "C" fn(*mut table_OTL) -> ()),
        copy: Some(table_OTL_copy as unsafe extern "C" fn(*mut table_OTL, *const table_OTL) -> ()),
        move_0: Some(table_OTL_move as unsafe extern "C" fn(*mut table_OTL, *mut table_OTL) -> ()),
        dispose: Some(table_OTL_dispose as unsafe extern "C" fn(*mut table_OTL) -> ()),
        replace: Some(table_OTL_replace as unsafe extern "C" fn(*mut table_OTL, table_OTL) -> ()),
        copyReplace: Some(
            table_OTL_copyReplace as unsafe extern "C" fn(*mut table_OTL, table_OTL) -> (),
        ),
        create: Some(table_OTL_create),
        free: Some(table_OTL_free as unsafe extern "C" fn(*mut table_OTL) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_OTL_copy(mut dst: *mut table_OTL, mut src: *const table_OTL) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_OTL>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_OTL_replace(mut dst: *mut table_OTL, src: table_OTL) {
    table_OTL_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_OTL>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_OTL_move(mut dst: *mut table_OTL, mut src: *mut table_OTL) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_OTL>() as usize,
    );
    table_OTL_init(src);
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
pub struct __caryll_vectorinterface_otl_LigatureArray {
    pub init: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LigatureArray, *const otl_LigatureArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LigatureArray, *mut otl_LigatureArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LigatureArray, otl_LigatureArray) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LigatureArray, otl_LigatureArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_LigatureArray>,
    pub free: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_LigatureArray>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LigatureArray, otl_LigatureBaseRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> otl_LigatureBaseRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_LigatureArray,
            Option<
                unsafe extern "C" fn(
                    *const otl_LigatureBaseRecord,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_LigatureArray,
            Option<
                unsafe extern "C" fn(
                    *const otl_LigatureBaseRecord,
                    *const otl_LigatureBaseRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}

#[cfg(test)]
mod tests {
    use super::*;

    // These are not internal labels: `name()` supplies the `"type"` string
    // otfccdump writes for every lookup, and the key otfccbuild matches a
    // lookup against when reading the JSON back. They are also *not* the
    // constants' own spelling -- the JSON says `gpos_mark_to_base` where the
    // constant is `otl_type_gpos_markToBase` -- so they were copied from the
    // `tableNames` table this replaced and are pinned here rather than derived.
    #[test]
    fn lookup_type_names_are_the_json_strings() {
        for (t, name) in [
            (otl_type_unknown, c"unknown"),
            (otl_type_gsub_unknown, c"gsub_unknown"),
            (otl_type_gsub_single, c"gsub_single"),
            (otl_type_gsub_multiple, c"gsub_multiple"),
            (otl_type_gsub_alternate, c"gsub_alternate"),
            (otl_type_gsub_ligature, c"gsub_ligature"),
            (otl_type_gsub_context, c"gsub_context"),
            (otl_type_gsub_chaining, c"gsub_chaining"),
            (otl_type_gsub_extend, c"gsub_extend"),
            (otl_type_gsub_reverse, c"gsub_reverse"),
            (otl_type_gpos_unknown, c"gpos_unknown"),
            (otl_type_gpos_single, c"gpos_single"),
            (otl_type_gpos_pair, c"gpos_pair"),
            (otl_type_gpos_cursive, c"gpos_cursive"),
            (otl_type_gpos_markToBase, c"gpos_mark_to_base"),
            (otl_type_gpos_markToLigature, c"gpos_mark_to_ligature"),
            (otl_type_gpos_markToMark, c"gpos_mark_to_mark"),
            (otl_type_gpos_context, c"gpos_context"),
            (otl_type_gpos_chaining, c"gpos_chaining"),
            (otl_type_gpos_extend, c"gpos_extend"),
        ] {
            assert_eq!(t.name(), name, "name for {t:?}");
        }
    }

    // The numbering is otfcc's own: the file's format number plus 16 for GSUB or
    // 32 for GPOS. `file_format` has to undo exactly that, because its result is
    // written straight into the lookup header.
    #[test]
    fn file_format_undoes_the_table_base() {
        assert_eq!(otl_type_gsub_single.file_format(), 1);
        assert_eq!(otl_type_gsub_reverse.file_format(), 8);
        assert_eq!(otl_type_gsub_extend.file_format(), 7);
        assert_eq!(otl_type_gpos_single.file_format(), 1);
        assert_eq!(otl_type_gpos_extend.file_format(), 9);
        // The bases themselves are *not* above their own base -- C compares with
        // `>`, not `>=` -- so they carry no format number.
        assert_eq!(otl_type_unknown.file_format(), 0);
        assert_eq!(otl_type_gsub_unknown.file_format(), 0);
        // Except `gpos_unknown`, and this one is a quirk kept on purpose: 32 is
        // not above GPOS's base but it *is* above GSUB's, so C's nested
        // comparisons read it as GSUB format 16. Reachable only from a font
        // declaring a GPOS lookup of format 0, which no version of the spec has
        // -- but the number would go straight into the lookup header, so it is
        // reproduced rather than tidied.
        assert_eq!(otl_type_gpos_unknown.file_format(), 16);
    }

    // A lookup type comes out of the font as a 16-bit number added to a base,
    // and C keeps whatever that gives -- including values no variant names,
    // which is why this type is not an enum. The raw value is observable: an
    // unnamed lookup is called `lookup_<raw as %04x>_<index>` in the JSON.
    #[test]
    fn from_file_keeps_unnamed_types() {
        assert_eq!(
            otl_LookupType::from_file(otl_type_gsub_unknown, 1),
            otl_type_gsub_single
        );
        assert_eq!(
            otl_LookupType::from_file(otl_type_gpos_unknown, 9),
            otl_type_gpos_extend
        );
        // GSUB format 9 exists in no version of the spec otfcc knows; it stays
        // 25, gets no subtable, and reaches the output as `lookup_0019_…`.
        let unnamed = otl_LookupType::from_file(otl_type_gsub_unknown, 9);
        assert_eq!(unnamed.raw(), 25);
        assert_eq!(unnamed.name(), c"unknown");
        assert_eq!(otl_LookupType::from_file(otl_type_gsub_unknown, 0xffff).raw(), 65551);
        assert_eq!(::core::mem::size_of::<otl_LookupType>(), 4);
    }
}
