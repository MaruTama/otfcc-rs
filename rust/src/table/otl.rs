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

use crate::table::otl::classdef::{ClassDef};
use crate::table::otl::coverage::{Coverage};
use crate::support::handle::{GlyphHandle, LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::primitives::{GlyphClass, GlyphId, Pos, TableId};
use crate::vendor::sds::{SdsRaw};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::{ComparFn};
use crate::table::otl::subtables::chaining::common::{iSubtable_chaining};
use crate::table::otl::subtables::gpos_cursive::{iSubtable_gpos_cursive};
use crate::table::otl::subtables::gpos_mark_to_ligature::{iSubtable_gpos_markToLigature};
use crate::table::otl::subtables::gpos_mark_to_single::{iSubtable_gpos_markToSingle};
use crate::table::otl::subtables::gpos_pair::{iSubtable_gpos_pair};
use crate::table::otl::subtables::gpos_single::{iSubtable_gpos_single};
use crate::table::otl::subtables::gsub_ligature::{iSubtable_gsub_ligature};
use crate::table::otl::subtables::gsub_multi::{iSubtable_gsub_multi};
use crate::table::otl::subtables::gsub_reverse::{iSubtable_gsub_reverse};
use crate::table::otl::subtables::gsub_single::{iSubtable_gsub_single};
use crate::vendor::sds::{sdsfree};


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
/// through [`LookupType::from_file`]. What it buys over the bare `c_uint`
/// c2rust emitted is that the compiler now separates it from every other 32-bit
/// quantity in the OTL code, and that [`LookupType::name`] replaces a
/// 42-entry sparse table of C string pointers.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct LookupType(u32);

pub const otl_type_gpos_extend: LookupType = LookupType(41);
pub const otl_type_gpos_chaining: LookupType = LookupType(40);
pub const otl_type_gpos_context: LookupType = LookupType(39);
pub const otl_type_gpos_markToMark: LookupType = LookupType(38);
pub const otl_type_gpos_markToLigature: LookupType = LookupType(37);
pub const otl_type_gpos_markToBase: LookupType = LookupType(36);
pub const otl_type_gpos_cursive: LookupType = LookupType(35);
pub const otl_type_gpos_pair: LookupType = LookupType(34);
pub const otl_type_gpos_single: LookupType = LookupType(33);
pub const otl_type_gpos_unknown: LookupType = LookupType(32);
pub const otl_type_gsub_reverse: LookupType = LookupType(24);
pub const otl_type_gsub_extend: LookupType = LookupType(23);
pub const otl_type_gsub_chaining: LookupType = LookupType(22);
pub const otl_type_gsub_context: LookupType = LookupType(21);
pub const otl_type_gsub_ligature: LookupType = LookupType(20);
pub const otl_type_gsub_alternate: LookupType = LookupType(19);
pub const otl_type_gsub_multiple: LookupType = LookupType(18);
pub const otl_type_gsub_single: LookupType = LookupType(17);
pub const otl_type_gsub_unknown: LookupType = LookupType(16);
pub const otl_type_unknown: LookupType = LookupType(0);

impl LookupType {
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
pub union Subtable {
    pub gsub_single: GsubSingleSubtable,
    pub gsub_multi: GsubMultiSubtable,
    pub gsub_ligature: GsubLigatureSubtable,
    pub chaining: ChainingSubtable,
    pub gsub_reverse: GsubReverseSubtable,
    pub gpos_single: GposSingleSubtable,
    pub gpos_pair: GposPairSubtable,
    pub gpos_cursive: GposCursiveSubtable,
    pub gpos_markToSingle: GposMarkToSingleSubtable,
    pub gpos_markToLigature: GposMarkToLigatureSubtable,
    pub extend: ExtendSubtable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtendSubtable {
    pub type_0: LookupType,
    pub subtable: *mut Subtable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposMarkToLigatureSubtable {
    pub classCount: GlyphClass,
    pub markArray: MarkArray,
    pub ligArray: LigatureArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LigatureArray {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut LigatureBaseRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LigatureBaseRecord {
    pub glyph: GlyphHandle,
    pub componentCount: GlyphId,
    pub anchors: *mut *mut Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Anchor {
    pub present: bool,
    pub x: Pos,
    pub y: Pos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MarkArray {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut MarkRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MarkRecord {
    pub glyph: GlyphHandle,
    pub markClass: GlyphClass,
    pub anchor: Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposMarkToSingleSubtable {
    pub classCount: GlyphClass,
    pub markArray: MarkArray,
    pub baseArray: BaseArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseArray {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut BaseRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseRecord {
    pub glyph: GlyphHandle,
    pub anchors: *mut Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposCursiveSubtable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut GposCursiveEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposCursiveEntry {
    pub target: GlyphHandle,
    pub enter: Anchor,
    pub exit: Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposPairSubtable {
    pub first: *mut ClassDef,
    pub second: *mut ClassDef,
    pub firstValues: *mut *mut PositionValue,
    pub secondValues: *mut *mut PositionValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PositionValue {
    pub dx: Pos,
    pub dy: Pos,
    pub dWidth: Pos,
    pub dHeight: Pos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposSingleSubtable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut GposSingleEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposSingleEntry {
    pub target: GlyphHandle,
    pub value: PositionValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubReverseSubtable {
    pub matchCount: TableId,
    pub inputIndex: TableId,
    pub match_0: *mut *mut Coverage,
    pub to: *mut Coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainingSubtable {
    pub type_0: ChainingType,
    pub c2rust_unnamed: ChainingBody,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ChainingBody {
    pub rule: ChainingRule,
    pub c2rust_unnamed: ChainingRuleSet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainingRuleSet {
    pub rulesCount: TableId,
    pub rules: *mut *mut ChainingRule,
    pub bc: *mut ClassDef,
    pub ic: *mut ClassDef,
    pub fc: *mut ClassDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainingRule {
    pub matchCount: TableId,
    pub inputBegins: TableId,
    pub inputEnds: TableId,
    pub match_0: *mut *mut Coverage,
    pub applyCount: TableId,
    pub apply: *mut ChainLookupApplication,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainLookupApplication {
    pub index: TableId,
    pub lookup: LookupHandle,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ChainingType {
    otl_chaining_canonical = 0,
    otl_chaining_poly = 1,
    otl_chaining_classified = 2,
}
pub use ChainingType::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubLigatureSubtable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut GsubLigatureEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubLigatureEntry {
    pub from: *mut Coverage,
    pub to: GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubMultiSubtable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut GsubMultiEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubMultiEntry {
    pub from: GlyphHandle,
    pub to: *mut Coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubSingleSubtable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut GsubSingleEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubSingleEntry {
    pub from: GlyphHandle,
    pub to: GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lookup {
    pub name: SdsRaw,
    pub type_0: LookupType,
    pub _offset: u32,
    pub flags: u16,
    pub subtables: SubtableList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubtableList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut SubtablePtr,
}
pub type SubtablePtr = *mut Subtable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubSingleSubtableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut GsubSingleSubtable, *const GsubSingleSubtable) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut GsubSingleSubtable, *mut GsubSingleSubtable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut GsubSingleSubtable, GsubSingleSubtable) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut GsubSingleSubtable, GsubSingleSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GsubSingleSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut GsubSingleSubtable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut GsubSingleSubtable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut GsubSingleSubtable>,
    pub fill: Option<unsafe extern "C" fn(*mut GsubSingleSubtable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut GsubSingleSubtable, GsubSingleEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> GsubSingleEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut GsubSingleSubtable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut GsubSingleSubtable,
            Option<
                unsafe extern "C" fn(*const GsubSingleEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut GsubSingleSubtable,
            Option<
                unsafe extern "C" fn(
                    *const GsubSingleEntry,
                    *const GsubSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubMultiSubtableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut GsubMultiSubtable, *const GsubMultiSubtable) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut GsubMultiSubtable, *mut GsubMultiSubtable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, GsubMultiSubtable) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut GsubMultiSubtable, GsubMultiSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GsubMultiSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut GsubMultiSubtable>,
    pub fill: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, GsubMultiEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> GsubMultiEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut GsubMultiSubtable,
            Option<
                unsafe extern "C" fn(*const GsubMultiEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut GsubMultiSubtable,
            Option<
                unsafe extern "C" fn(
                    *const GsubMultiEntry,
                    *const GsubMultiEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubLigatureSubtableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut GsubLigatureSubtable, *const GsubLigatureSubtable) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut GsubLigatureSubtable, *mut GsubLigatureSubtable) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, GsubLigatureSubtable) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, GsubLigatureSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GsubLigatureSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut GsubLigatureSubtable>,
    pub fill: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, GsubLigatureEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> GsubLigatureEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut GsubLigatureSubtable,
            Option<
                unsafe extern "C" fn(
                    *const GsubLigatureEntry,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut GsubLigatureSubtable,
            Option<
                unsafe extern "C" fn(
                    *const GsubLigatureEntry,
                    *const GsubLigatureEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainingSubtableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut ChainingSubtable, *const ChainingSubtable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut ChainingSubtable, *mut ChainingSubtable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut ChainingSubtable, ChainingSubtable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut ChainingSubtable, ChainingSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ChainingSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubReverseSubtableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GsubReverseSubtable) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut GsubReverseSubtable, *const GsubReverseSubtable) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut GsubReverseSubtable, *mut GsubReverseSubtable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GsubReverseSubtable) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut GsubReverseSubtable, GsubReverseSubtable) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut GsubReverseSubtable, GsubReverseSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GsubReverseSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GsubReverseSubtable) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposSingleSubtableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut GposSingleSubtable, *const GposSingleSubtable) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut GposSingleSubtable, *mut GposSingleSubtable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut GposSingleSubtable, GposSingleSubtable) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut GposSingleSubtable, GposSingleSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GposSingleSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut GposSingleSubtable>,
    pub fill: Option<unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut GposSingleSubtable, GposSingleEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> GposSingleEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut GposSingleSubtable,
            Option<
                unsafe extern "C" fn(*const GposSingleEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut GposSingleSubtable,
            Option<
                unsafe extern "C" fn(
                    *const GposSingleEntry,
                    *const GposSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposPairSubtableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GposPairSubtable) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut GposPairSubtable, *const GposPairSubtable) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut GposPairSubtable, *mut GposPairSubtable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GposPairSubtable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut GposPairSubtable, GposPairSubtable) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut GposPairSubtable, GposPairSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GposPairSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GposPairSubtable) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposCursiveSubtableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut GposCursiveSubtable, *const GposCursiveSubtable) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut GposCursiveSubtable, *mut GposCursiveSubtable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut GposCursiveSubtable, GposCursiveSubtable) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut GposCursiveSubtable, GposCursiveSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GposCursiveSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut GposCursiveSubtable>,
    pub fill: Option<unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut GposCursiveSubtable, GposCursiveEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> GposCursiveEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut GposCursiveSubtable,
            Option<
                unsafe extern "C" fn(*const GposCursiveEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut GposCursiveSubtable,
            Option<
                unsafe extern "C" fn(
                    *const GposCursiveEntry,
                    *const GposCursiveEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposMarkToSingleSubtableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut GposMarkToSingleSubtable,
            *const GposMarkToSingleSubtable,
        ) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(
            *mut GposMarkToSingleSubtable,
            *mut GposMarkToSingleSubtable,
        ) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> ()>,
    pub replace: Option<
        unsafe extern "C" fn(*mut GposMarkToSingleSubtable, GposMarkToSingleSubtable) -> (),
    >,
    pub copyReplace: Option<
        unsafe extern "C" fn(*mut GposMarkToSingleSubtable, GposMarkToSingleSubtable) -> (),
    >,
    pub create: Option<unsafe extern "C" fn() -> *mut GposMarkToSingleSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposMarkToLigatureSubtableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GposMarkToLigatureSubtable) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut GposMarkToLigatureSubtable,
            *const GposMarkToLigatureSubtable,
        ) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(
            *mut GposMarkToLigatureSubtable,
            *mut GposMarkToLigatureSubtable,
        ) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut GposMarkToLigatureSubtable) -> ()>,
    pub replace: Option<
        unsafe extern "C" fn(*mut GposMarkToLigatureSubtable, GposMarkToLigatureSubtable) -> (),
    >,
    pub copyReplace: Option<
        unsafe extern "C" fn(*mut GposMarkToLigatureSubtable, GposMarkToLigatureSubtable) -> (),
    >,
    pub create: Option<unsafe extern "C" fn() -> *mut GposMarkToLigatureSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GposMarkToLigatureSubtable) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubtableListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut SubtableList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut SubtableList, *const SubtableList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut SubtableList, *mut SubtableList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut SubtableList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut SubtableList, SubtableList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut SubtableList, SubtableList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut SubtableList>,
    pub free: Option<unsafe extern "C" fn(*mut SubtableList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut SubtableList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut SubtableList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut SubtableList>,
    pub fill: Option<unsafe extern "C" fn(*mut SubtableList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut SubtableList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut SubtableList, SubtablePtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut SubtableList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut SubtableList) -> SubtablePtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut SubtableList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut SubtableList,
            Option<unsafe extern "C" fn(*const SubtablePtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut SubtableList,
            Option<
                unsafe extern "C" fn(
                    *const SubtablePtr,
                    *const SubtablePtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
    pub disposeDependent:
        Option<unsafe extern "C" fn(*mut SubtableList, *const Lookup) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubtablePtrElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut SubtablePtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut SubtablePtr, *const SubtablePtr) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut SubtablePtr, *mut SubtablePtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut SubtablePtr) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut SubtablePtr, SubtablePtr) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut SubtablePtr, SubtablePtr) -> ()>,
}
pub type LookupPtr = *mut Lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookupPtrElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut LookupPtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut LookupPtr, *const LookupPtr) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut LookupPtr, *mut LookupPtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut LookupPtr) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut LookupPtr, LookupPtr) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut LookupPtr, LookupPtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookupList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut LookupPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookupListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut LookupList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut LookupList, *const LookupList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut LookupList, *mut LookupList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut LookupList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut LookupList, LookupList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut LookupList, LookupList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LookupList>,
    pub free: Option<unsafe extern "C" fn(*mut LookupList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut LookupList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut LookupList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut LookupList>,
    pub fill: Option<unsafe extern "C" fn(*mut LookupList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut LookupList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut LookupList, LookupPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut LookupList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut LookupList) -> LookupPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut LookupList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut LookupList,
            Option<unsafe extern "C" fn(*const LookupPtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut LookupList,
            Option<
                unsafe extern "C" fn(
                    *const LookupPtr,
                    *const LookupPtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
pub type LookupRef = *const Lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookupRefElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut LookupRef) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut LookupRef, *const LookupRef) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut LookupRef, *mut LookupRef) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut LookupRef) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut LookupRef, LookupRef) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut LookupRef, LookupRef) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookupRefList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut LookupRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookupRefListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut LookupRefList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut LookupRefList, *const LookupRefList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut LookupRefList, *mut LookupRefList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut LookupRefList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut LookupRefList, LookupRefList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut LookupRefList, LookupRefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LookupRefList>,
    pub free: Option<unsafe extern "C" fn(*mut LookupRefList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut LookupRefList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut LookupRefList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut LookupRefList>,
    pub fill: Option<unsafe extern "C" fn(*mut LookupRefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut LookupRefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut LookupRefList, LookupRef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut LookupRefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut LookupRefList) -> LookupRef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut LookupRefList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut LookupRefList,
            Option<unsafe extern "C" fn(*const LookupRef, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut LookupRefList,
            Option<
                unsafe extern "C" fn(
                    *const LookupRef,
                    *const LookupRef,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Feature {
    pub name: SdsRaw,
    pub lookups: LookupRefList,
}
pub type FeaturePtr = *mut Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FeaturePtrElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut FeaturePtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut FeaturePtr, *const FeaturePtr) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut FeaturePtr, *mut FeaturePtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut FeaturePtr) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut FeaturePtr, FeaturePtr) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut FeaturePtr, FeaturePtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FeatureList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut FeaturePtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FeatureListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut FeatureList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut FeatureList, *const FeatureList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut FeatureList, *mut FeatureList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut FeatureList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut FeatureList, FeatureList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut FeatureList, FeatureList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut FeatureList>,
    pub free: Option<unsafe extern "C" fn(*mut FeatureList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut FeatureList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut FeatureList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut FeatureList>,
    pub fill: Option<unsafe extern "C" fn(*mut FeatureList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut FeatureList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut FeatureList, FeaturePtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut FeatureList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut FeatureList) -> FeaturePtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut FeatureList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut FeatureList,
            Option<unsafe extern "C" fn(*const FeaturePtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut FeatureList,
            Option<
                unsafe extern "C" fn(
                    *const FeaturePtr,
                    *const FeaturePtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
pub type FeatureRef = *const Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FeatureRefElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut FeatureRef) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut FeatureRef, *const FeatureRef) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut FeatureRef, *mut FeatureRef) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut FeatureRef) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut FeatureRef, FeatureRef) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut FeatureRef, FeatureRef) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FeatureRefList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut FeatureRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FeatureRefListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut FeatureRefList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut FeatureRefList, *const FeatureRefList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut FeatureRefList, *mut FeatureRefList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut FeatureRefList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut FeatureRefList, FeatureRefList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut FeatureRefList, FeatureRefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut FeatureRefList>,
    pub free: Option<unsafe extern "C" fn(*mut FeatureRefList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut FeatureRefList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut FeatureRefList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut FeatureRefList>,
    pub fill: Option<unsafe extern "C" fn(*mut FeatureRefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut FeatureRefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut FeatureRefList, FeatureRef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut FeatureRefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut FeatureRefList) -> FeatureRef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut FeatureRefList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut FeatureRefList,
            Option<unsafe extern "C" fn(*const FeatureRef, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut FeatureRefList,
            Option<
                unsafe extern "C" fn(
                    *const FeatureRef,
                    *const FeatureRef,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LanguageSystem {
    pub name: SdsRaw,
    pub requiredFeature: FeatureRef,
    pub features: FeatureRefList,
}
pub type LanguageSystemPtr = *mut LanguageSystem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LanguageSystemPtrElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut LanguageSystemPtr) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut LanguageSystemPtr, *const LanguageSystemPtr) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut LanguageSystemPtr, *mut LanguageSystemPtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut LanguageSystemPtr) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut LanguageSystemPtr, LanguageSystemPtr) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut LanguageSystemPtr, LanguageSystemPtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LangSystemList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut LanguageSystemPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LangSystemListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut LangSystemList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut LangSystemList, *const LangSystemList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut LangSystemList, *mut LangSystemList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut LangSystemList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut LangSystemList, LangSystemList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut LangSystemList, LangSystemList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LangSystemList>,
    pub free: Option<unsafe extern "C" fn(*mut LangSystemList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut LangSystemList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut LangSystemList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut LangSystemList>,
    pub fill: Option<unsafe extern "C" fn(*mut LangSystemList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut LangSystemList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut LangSystemList, LanguageSystemPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut LangSystemList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut LangSystemList) -> LanguageSystemPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut LangSystemList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut LangSystemList,
            Option<
                unsafe extern "C" fn(
                    *const LanguageSystemPtr,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut LangSystemList,
            Option<
                unsafe extern "C" fn(
                    *const LanguageSystemPtr,
                    *const LanguageSystemPtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OtlTable {
    pub lookups: LookupList,
    pub features: FeatureList,
    pub languages: LangSystemList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OtlTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut OtlTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut OtlTable, *const OtlTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut OtlTable, *mut OtlTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut OtlTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut OtlTable, OtlTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut OtlTable, OtlTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut OtlTable>,
    pub free: Option<unsafe extern "C" fn(*mut OtlTable) -> ()>,
}
#[inline]
unsafe extern "C" fn disposeSubtableDependent(
    mut subtableRef: *mut SubtablePtr,
    mut lookup: *const Lookup,
) {
    match (*lookup).type_0 {
        otl_type_gsub_single => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gsub_single.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gsub_multiple => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gsub_multi.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gsub_alternate => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gsub_multi.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gsub_ligature => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gsub_ligature.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gsub_chaining => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_chaining.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gsub_reverse => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubReverseSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gsub_reverse.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_single => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gpos_single.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_pair => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposPairSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gpos_pair.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_cursive => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gpos_cursive.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_chaining => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_chaining.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_markToBase => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gpos_markToSingle.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_markToMark => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gpos_markToSingle.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        otl_type_gpos_markToLigature => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposMarkToLigatureSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(iSubtable_gpos_markToLigature.free)
            .expect("non-null function pointer")(*subtableRef);
        }
        _ => {}
    };
}
static otl_iSubtablePtr: SubtablePtrElementInterface =
    SubtablePtrElementInterface {
        init: None,
        copy: None,
        move_0: None,
        dispose: None,
        replace: None,
        copyReplace: None,
    };
#[inline]
unsafe extern "C" fn otl_SubtableList_disposeDependent(
    mut arr: *mut SubtableList,
    mut enclosure: *const Lookup,
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
            (*arr).items.offset(j as isize) as *mut SubtablePtr,
            enclosure,
        );
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<SubtablePtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_SubtableList_filterEnv(
    mut arr: *mut SubtableList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const SubtablePtr, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut SubtablePtr,
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
                    (*arr).items.offset(k as isize) as *mut SubtablePtr,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_SubtableList_createN(mut n: usize) -> *mut SubtableList {
    let mut t: *mut SubtableList =
        malloc(::core::mem::size_of::<SubtableList>() as usize) as *mut SubtableList;
    otl_SubtableList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_SubtableList_sort(
    mut arr: *mut SubtableList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const SubtablePtr, *const SubtablePtr) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<SubtablePtr>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const SubtablePtr,
                    *const SubtablePtr,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_SubtableList_shrinkToFit(mut arr: *mut SubtableList) {
    otl_SubtableList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_resizeTo(arr: *mut SubtableList, target: usize) {
    cvec_resize_to(otl_SubtableList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_move(dst: *mut SubtableList, src: *mut SubtableList) {
    cvec_move(otl_SubtableList_as_cvec(dst), otl_SubtableList_as_cvec(src));
}
#[inline]
unsafe fn otl_SubtableList_as_cvec(arr: *mut SubtableList) -> *mut CVecRaw<SubtablePtr> {
    arr as *mut CVecRaw<SubtablePtr>
}
#[inline]
unsafe extern "C" fn otl_SubtableList_init(arr: *mut SubtableList) {
    cvec_init(otl_SubtableList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_SubtableList_free(mut x: *mut SubtableList) {
    if x.is_null() {
        return;
    }
    otl_SubtableList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_create() -> *mut SubtableList {
    let mut x: *mut SubtableList =
        malloc(::core::mem::size_of::<SubtableList>() as usize) as *mut SubtableList;
    otl_SubtableList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_SubtableList_fill(mut arr: *mut SubtableList, mut n: usize) {
    while (*arr).length < n {
        let mut x: SubtablePtr = ::core::ptr::null_mut::<Subtable>();
        if otl_iSubtablePtr.init.is_some() {
            otl_iSubtablePtr.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<SubtablePtr>() as usize,
            );
        }
        otl_SubtableList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_SubtableList_disposeItem(mut arr: *mut SubtableList, mut n: usize) {
    if otl_iSubtablePtr.dispose.is_some() {
        otl_iSubtablePtr.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut SubtablePtr,
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_SubtableList_push(arr: *mut SubtableList, elem: SubtablePtr) {
    cvec_push(otl_SubtableList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_grow(arr: *mut SubtableList) {
    cvec_grow(otl_SubtableList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_SubtableList_growTo(arr: *mut SubtableList, target: usize) {
    cvec_grow_to(otl_SubtableList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_pop(arr: *mut SubtableList) -> SubtablePtr {
    cvec_pop(otl_SubtableList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_SubtableList_copyReplace(
    mut dst: *mut SubtableList,
    src: SubtableList,
) {
    otl_SubtableList_dispose(dst);
    otl_SubtableList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_copy(
    mut dst: *mut SubtableList,
    mut src: *const SubtableList,
) {
    otl_SubtableList_init(dst);
    otl_SubtableList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iSubtablePtr.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iSubtablePtr.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut SubtablePtr,
                (*src).items.offset(j as isize) as *mut SubtablePtr as *const SubtablePtr,
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
unsafe extern "C" fn otl_SubtableList_dispose(mut arr: *mut SubtableList) {
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
                (*arr).items.offset(j as isize) as *mut SubtablePtr,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<SubtablePtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_SubtableList_replace(
    mut dst: *mut SubtableList,
    src: SubtableList,
) {
    otl_SubtableList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<SubtableList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_SubtableList_initCapN(mut arr: *mut SubtableList, mut n: usize) {
    otl_SubtableList_init(arr);
    otl_SubtableList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_growToN(arr: *mut SubtableList, target: usize) {
    cvec_grow_to_n(otl_SubtableList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_SubtableList_initN(mut arr: *mut SubtableList, mut n: usize) {
    otl_SubtableList_init(arr);
    otl_SubtableList_growToN(arr, n);
    otl_SubtableList_fill(arr, n);
}
pub static otl_iSubtableList: SubtableListVectorInterface = {
    SubtableListVectorInterface {
        init: Some(otl_SubtableList_init as unsafe extern "C" fn(*mut SubtableList) -> ()),
        copy: Some(
            otl_SubtableList_copy
                as unsafe extern "C" fn(*mut SubtableList, *const SubtableList) -> (),
        ),
        move_0: Some(
            otl_SubtableList_move
                as unsafe extern "C" fn(*mut SubtableList, *mut SubtableList) -> (),
        ),
        dispose: Some(
            otl_SubtableList_dispose as unsafe extern "C" fn(*mut SubtableList) -> (),
        ),
        replace: Some(
            otl_SubtableList_replace
                as unsafe extern "C" fn(*mut SubtableList, SubtableList) -> (),
        ),
        copyReplace: Some(
            otl_SubtableList_copyReplace
                as unsafe extern "C" fn(*mut SubtableList, SubtableList) -> (),
        ),
        create: Some(otl_SubtableList_create),
        free: Some(otl_SubtableList_free as unsafe extern "C" fn(*mut SubtableList) -> ()),
        initN: Some(
            otl_SubtableList_initN as unsafe extern "C" fn(*mut SubtableList, usize) -> (),
        ),
        initCapN: Some(
            otl_SubtableList_initCapN as unsafe extern "C" fn(*mut SubtableList, usize) -> (),
        ),
        createN: Some(
            otl_SubtableList_createN as unsafe extern "C" fn(usize) -> *mut SubtableList,
        ),
        fill: Some(
            otl_SubtableList_fill as unsafe extern "C" fn(*mut SubtableList, usize) -> (),
        ),
        clear: Some(otl_SubtableList_dispose as unsafe extern "C" fn(*mut SubtableList) -> ()),
        push: Some(
            otl_SubtableList_push
                as unsafe extern "C" fn(*mut SubtableList, SubtablePtr) -> (),
        ),
        shrinkToFit: Some(
            otl_SubtableList_shrinkToFit as unsafe extern "C" fn(*mut SubtableList) -> (),
        ),
        pop: Some(
            otl_SubtableList_pop as unsafe extern "C" fn(*mut SubtableList) -> SubtablePtr,
        ),
        disposeItem: Some(
            otl_SubtableList_disposeItem
                as unsafe extern "C" fn(*mut SubtableList, usize) -> (),
        ),
        filterEnv: Some(
            otl_SubtableList_filterEnv
                as unsafe extern "C" fn(
                    *mut SubtableList,
                    Option<
                        unsafe extern "C" fn(
                            *const SubtablePtr,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_SubtableList_sort
                as unsafe extern "C" fn(
                    *mut SubtableList,
                    Option<
                        unsafe extern "C" fn(
                            *const SubtablePtr,
                            *const SubtablePtr,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
        disposeDependent: Some(
            otl_SubtableList_disposeDependent
                as unsafe extern "C" fn(*mut SubtableList, *const Lookup) -> (),
        ),
    }
};
pub unsafe extern "C" fn otfcc_delete_lookup(mut lookup: *mut Lookup) {
    if lookup.is_null() {
        return;
    }
    otl_iSubtableList
        .disposeDependent
        .expect("non-null function pointer")(&raw mut (*lookup).subtables, lookup);
    sdsfree((*lookup).name);
    free(lookup as *mut ::core::ffi::c_void);
    lookup = ::core::ptr::null_mut::<Lookup>();
}
#[inline]
unsafe extern "C" fn initLookupPtr(mut entry: *mut LookupPtr) {
    *entry = __caryll_allocate_clean(
        ::core::mem::size_of::<Lookup>() as usize,
        47 as ::core::ffi::c_ulong,
    ) as LookupPtr;
    (**entry).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    otl_iSubtableList.init.expect("non-null function pointer")(&raw mut (**entry).subtables);
}
#[inline]
unsafe extern "C" fn disposeLookupPtr(mut entry: *mut LookupPtr) {
    otfcc_delete_lookup(*entry);
}
pub static otl_iLookupPtr: LookupPtrElementInterface = {
    LookupPtrElementInterface {
        init: Some(otl_LookupPtr_init as unsafe extern "C" fn(*mut LookupPtr) -> ()),
        copy: Some(
            otl_LookupPtr_copy
                as unsafe extern "C" fn(*mut LookupPtr, *const LookupPtr) -> (),
        ),
        move_0: Some(
            otl_LookupPtr_move
                as unsafe extern "C" fn(*mut LookupPtr, *mut LookupPtr) -> (),
        ),
        dispose: Some(otl_LookupPtr_dispose as unsafe extern "C" fn(*mut LookupPtr) -> ()),
        replace: Some(
            otl_LookupPtr_replace as unsafe extern "C" fn(*mut LookupPtr, LookupPtr) -> (),
        ),
        copyReplace: Some(
            otl_LookupPtr_copyReplace
                as unsafe extern "C" fn(*mut LookupPtr, LookupPtr) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LookupPtr_dispose(mut x: *mut LookupPtr) {
    disposeLookupPtr(x);
}
#[inline]
unsafe extern "C" fn otl_LookupPtr_copy(
    mut dst: *mut LookupPtr,
    mut src: *const LookupPtr,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupPtr>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupPtr_copyReplace(mut dst: *mut LookupPtr, src: LookupPtr) {
    otl_LookupPtr_dispose(dst);
    otl_LookupPtr_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LookupPtr_replace(mut dst: *mut LookupPtr, src: LookupPtr) {
    otl_LookupPtr_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupPtr>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupPtr_move(mut dst: *mut LookupPtr, mut src: *mut LookupPtr) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupPtr>() as usize,
    );
    otl_LookupPtr_init(src);
}
#[inline]
unsafe extern "C" fn otl_LookupPtr_init(mut x: *mut LookupPtr) {
    initLookupPtr(x);
}
#[inline]
unsafe extern "C" fn otl_LookupList_resizeTo(arr: *mut LookupList, target: usize) {
    cvec_resize_to(otl_LookupList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LookupList_shrinkToFit(mut arr: *mut LookupList) {
    otl_LookupList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_LookupList_move(dst: *mut LookupList, src: *mut LookupList) {
    cvec_move(otl_LookupList_as_cvec(dst), otl_LookupList_as_cvec(src));
}
#[inline]
unsafe fn otl_LookupList_as_cvec(arr: *mut LookupList) -> *mut CVecRaw<LookupPtr> {
    arr as *mut CVecRaw<LookupPtr>
}
#[inline]
unsafe extern "C" fn otl_LookupList_init(arr: *mut LookupList) {
    cvec_init(otl_LookupList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LookupList_filterEnv(
    mut arr: *mut LookupList,
    mut fn_0: Option<unsafe extern "C" fn(*const LookupPtr, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut LookupPtr,
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
                    (*arr).items.offset(k as isize) as *mut LookupPtr,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_LookupList_disposeItem(mut arr: *mut LookupList, mut n: usize) {
    if otl_iLookupPtr.dispose.is_some() {
        otl_iLookupPtr.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut LookupPtr
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_LookupList_sort(
    mut arr: *mut LookupList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const LookupPtr, *const LookupPtr) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<LookupPtr>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const LookupPtr,
                    *const LookupPtr,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_LookupList_fill(mut arr: *mut LookupList, mut n: usize) {
    while (*arr).length < n {
        let mut x: LookupPtr = ::core::ptr::null_mut::<Lookup>();
        if otl_iLookupPtr.init.is_some() {
            otl_iLookupPtr.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<LookupPtr>() as usize,
            );
        }
        otl_LookupList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_LookupList_push(arr: *mut LookupList, elem: LookupPtr) {
    cvec_push(otl_LookupList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_LookupList_grow(arr: *mut LookupList) {
    cvec_grow(otl_LookupList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LookupList_growTo(arr: *mut LookupList, target: usize) {
    cvec_grow_to(otl_LookupList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LookupList_pop(arr: *mut LookupList) -> LookupPtr {
    cvec_pop(otl_LookupList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_LookupList_copyReplace(mut dst: *mut LookupList, src: LookupList) {
    otl_LookupList_dispose(dst);
    otl_LookupList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LookupList_copy(
    mut dst: *mut LookupList,
    mut src: *const LookupList,
) {
    otl_LookupList_init(dst);
    otl_LookupList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iLookupPtr.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iLookupPtr.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut LookupPtr,
                (*src).items.offset(j as isize) as *mut LookupPtr as *const LookupPtr,
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
unsafe extern "C" fn otl_LookupList_dispose(mut arr: *mut LookupList) {
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
                (*arr).items.offset(j as isize) as *mut LookupPtr,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<LookupPtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_LookupList_replace(mut dst: *mut LookupList, src: LookupList) {
    otl_LookupList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupList_initCapN(mut arr: *mut LookupList, mut n: usize) {
    otl_LookupList_init(arr);
    otl_LookupList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LookupList_growToN(arr: *mut LookupList, target: usize) {
    cvec_grow_to_n(otl_LookupList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LookupList_initN(mut arr: *mut LookupList, mut n: usize) {
    otl_LookupList_init(arr);
    otl_LookupList_growToN(arr, n);
    otl_LookupList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LookupList_free(mut x: *mut LookupList) {
    if x.is_null() {
        return;
    }
    otl_LookupList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_LookupList_createN(mut n: usize) -> *mut LookupList {
    let mut t: *mut LookupList =
        malloc(::core::mem::size_of::<LookupList>() as usize) as *mut LookupList;
    otl_LookupList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_LookupList_create() -> *mut LookupList {
    let mut x: *mut LookupList =
        malloc(::core::mem::size_of::<LookupList>() as usize) as *mut LookupList;
    otl_LookupList_init(x);
    return x;
}
pub static otl_iLookupList: LookupListVectorInterface = {
    LookupListVectorInterface {
        init: Some(otl_LookupList_init as unsafe extern "C" fn(*mut LookupList) -> ()),
        copy: Some(
            otl_LookupList_copy
                as unsafe extern "C" fn(*mut LookupList, *const LookupList) -> (),
        ),
        move_0: Some(
            otl_LookupList_move
                as unsafe extern "C" fn(*mut LookupList, *mut LookupList) -> (),
        ),
        dispose: Some(otl_LookupList_dispose as unsafe extern "C" fn(*mut LookupList) -> ()),
        replace: Some(
            otl_LookupList_replace
                as unsafe extern "C" fn(*mut LookupList, LookupList) -> (),
        ),
        copyReplace: Some(
            otl_LookupList_copyReplace
                as unsafe extern "C" fn(*mut LookupList, LookupList) -> (),
        ),
        create: Some(otl_LookupList_create),
        free: Some(otl_LookupList_free as unsafe extern "C" fn(*mut LookupList) -> ()),
        initN: Some(
            otl_LookupList_initN as unsafe extern "C" fn(*mut LookupList, usize) -> (),
        ),
        initCapN: Some(
            otl_LookupList_initCapN as unsafe extern "C" fn(*mut LookupList, usize) -> (),
        ),
        createN: Some(
            otl_LookupList_createN as unsafe extern "C" fn(usize) -> *mut LookupList,
        ),
        fill: Some(otl_LookupList_fill as unsafe extern "C" fn(*mut LookupList, usize) -> ()),
        clear: Some(otl_LookupList_dispose as unsafe extern "C" fn(*mut LookupList) -> ()),
        push: Some(
            otl_LookupList_push as unsafe extern "C" fn(*mut LookupList, LookupPtr) -> (),
        ),
        shrinkToFit: Some(
            otl_LookupList_shrinkToFit as unsafe extern "C" fn(*mut LookupList) -> (),
        ),
        pop: Some(otl_LookupList_pop as unsafe extern "C" fn(*mut LookupList) -> LookupPtr),
        disposeItem: Some(
            otl_LookupList_disposeItem as unsafe extern "C" fn(*mut LookupList, usize) -> (),
        ),
        filterEnv: Some(
            otl_LookupList_filterEnv
                as unsafe extern "C" fn(
                    *mut LookupList,
                    Option<
                        unsafe extern "C" fn(
                            *const LookupPtr,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_LookupList_sort
                as unsafe extern "C" fn(
                    *mut LookupList,
                    Option<
                        unsafe extern "C" fn(
                            *const LookupPtr,
                            *const LookupPtr,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LookupRef_dispose(mut _x: *mut LookupRef) {}
#[inline]
unsafe extern "C" fn otl_LookupRef_copyReplace(mut dst: *mut LookupRef, src: LookupRef) {
    otl_LookupRef_dispose(dst);
    otl_LookupRef_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LookupRef_move(mut dst: *mut LookupRef, mut src: *mut LookupRef) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupRef>() as usize,
    );
    otl_LookupRef_init(src);
}
#[inline]
unsafe extern "C" fn otl_LookupRef_init(mut x: *mut LookupRef) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<LookupRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupRef_copy(
    mut dst: *mut LookupRef,
    mut src: *const LookupRef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupRef_replace(mut dst: *mut LookupRef, src: LookupRef) {
    otl_LookupRef_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupRef>() as usize,
    );
}
pub static otl_iLookupRef: LookupRefElementInterface = {
    LookupRefElementInterface {
        init: Some(otl_LookupRef_init as unsafe extern "C" fn(*mut LookupRef) -> ()),
        copy: Some(
            otl_LookupRef_copy
                as unsafe extern "C" fn(*mut LookupRef, *const LookupRef) -> (),
        ),
        move_0: Some(
            otl_LookupRef_move
                as unsafe extern "C" fn(*mut LookupRef, *mut LookupRef) -> (),
        ),
        dispose: Some(otl_LookupRef_dispose as unsafe extern "C" fn(*mut LookupRef) -> ()),
        replace: Some(
            otl_LookupRef_replace as unsafe extern "C" fn(*mut LookupRef, LookupRef) -> (),
        ),
        copyReplace: Some(
            otl_LookupRef_copyReplace
                as unsafe extern "C" fn(*mut LookupRef, LookupRef) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LookupRefList_pop(arr: *mut LookupRefList) -> LookupRef {
    cvec_pop(otl_LookupRefList_as_cvec(arr))
}
pub static otl_iLookupRefList: LookupRefListVectorInterface = {
    LookupRefListVectorInterface {
        init: Some(otl_LookupRefList_init as unsafe extern "C" fn(*mut LookupRefList) -> ()),
        copy: Some(
            otl_LookupRefList_copy
                as unsafe extern "C" fn(*mut LookupRefList, *const LookupRefList) -> (),
        ),
        move_0: Some(
            otl_LookupRefList_move
                as unsafe extern "C" fn(*mut LookupRefList, *mut LookupRefList) -> (),
        ),
        dispose: Some(
            otl_LookupRefList_dispose as unsafe extern "C" fn(*mut LookupRefList) -> (),
        ),
        replace: Some(
            otl_LookupRefList_replace
                as unsafe extern "C" fn(*mut LookupRefList, LookupRefList) -> (),
        ),
        copyReplace: Some(
            otl_LookupRefList_copyReplace
                as unsafe extern "C" fn(*mut LookupRefList, LookupRefList) -> (),
        ),
        create: Some(otl_LookupRefList_create),
        free: Some(otl_LookupRefList_free as unsafe extern "C" fn(*mut LookupRefList) -> ()),
        initN: Some(
            otl_LookupRefList_initN as unsafe extern "C" fn(*mut LookupRefList, usize) -> (),
        ),
        initCapN: Some(
            otl_LookupRefList_initCapN
                as unsafe extern "C" fn(*mut LookupRefList, usize) -> (),
        ),
        createN: Some(
            otl_LookupRefList_createN as unsafe extern "C" fn(usize) -> *mut LookupRefList,
        ),
        fill: Some(
            otl_LookupRefList_fill as unsafe extern "C" fn(*mut LookupRefList, usize) -> (),
        ),
        clear: Some(
            otl_LookupRefList_dispose as unsafe extern "C" fn(*mut LookupRefList) -> (),
        ),
        push: Some(
            otl_LookupRefList_push
                as unsafe extern "C" fn(*mut LookupRefList, LookupRef) -> (),
        ),
        shrinkToFit: Some(
            otl_LookupRefList_shrinkToFit as unsafe extern "C" fn(*mut LookupRefList) -> (),
        ),
        pop: Some(
            otl_LookupRefList_pop as unsafe extern "C" fn(*mut LookupRefList) -> LookupRef,
        ),
        disposeItem: Some(
            otl_LookupRefList_disposeItem
                as unsafe extern "C" fn(*mut LookupRefList, usize) -> (),
        ),
        filterEnv: Some(
            otl_LookupRefList_filterEnv
                as unsafe extern "C" fn(
                    *mut LookupRefList,
                    Option<
                        unsafe extern "C" fn(
                            *const LookupRef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_LookupRefList_sort
                as unsafe extern "C" fn(
                    *mut LookupRefList,
                    Option<
                        unsafe extern "C" fn(
                            *const LookupRef,
                            *const LookupRef,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LookupRefList_shrinkToFit(mut arr: *mut LookupRefList) {
    otl_LookupRefList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_resizeTo(arr: *mut LookupRefList, target: usize) {
    cvec_resize_to(otl_LookupRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_move(dst: *mut LookupRefList, src: *mut LookupRefList) {
    cvec_move(otl_LookupRefList_as_cvec(dst), otl_LookupRefList_as_cvec(src));
}
#[inline]
unsafe fn otl_LookupRefList_as_cvec(arr: *mut LookupRefList) -> *mut CVecRaw<LookupRef> {
    arr as *mut CVecRaw<LookupRef>
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_init(arr: *mut LookupRefList) {
    cvec_init(otl_LookupRefList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_filterEnv(
    mut arr: *mut LookupRefList,
    mut fn_0: Option<unsafe extern "C" fn(*const LookupRef, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut LookupRef,
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
                    (*arr).items.offset(k as isize) as *mut LookupRef,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_disposeItem(mut arr: *mut LookupRefList, mut n: usize) {
    if otl_iLookupRef.dispose.is_some() {
        otl_iLookupRef.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut LookupRef
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_sort(
    mut arr: *mut LookupRefList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const LookupRef, *const LookupRef) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<LookupRef>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const LookupRef,
                    *const LookupRef,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_fill(mut arr: *mut LookupRefList, mut n: usize) {
    while (*arr).length < n {
        let mut x: LookupRef = ::core::ptr::null::<Lookup>();
        if otl_iLookupRef.init.is_some() {
            otl_iLookupRef.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<LookupRef>() as usize,
            );
        }
        otl_LookupRefList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_push(arr: *mut LookupRefList, elem: LookupRef) {
    cvec_push(otl_LookupRefList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_createN(mut n: usize) -> *mut LookupRefList {
    let mut t: *mut LookupRefList =
        malloc(::core::mem::size_of::<LookupRefList>() as usize) as *mut LookupRefList;
    otl_LookupRefList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_grow(arr: *mut LookupRefList) {
    cvec_grow(otl_LookupRefList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_growTo(arr: *mut LookupRefList, target: usize) {
    cvec_grow_to(otl_LookupRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_create() -> *mut LookupRefList {
    let mut x: *mut LookupRefList =
        malloc(::core::mem::size_of::<LookupRefList>() as usize) as *mut LookupRefList;
    otl_LookupRefList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_copyReplace(
    mut dst: *mut LookupRefList,
    src: LookupRefList,
) {
    otl_LookupRefList_dispose(dst);
    otl_LookupRefList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_copy(
    mut dst: *mut LookupRefList,
    mut src: *const LookupRefList,
) {
    otl_LookupRefList_init(dst);
    otl_LookupRefList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iLookupRef.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iLookupRef.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut LookupRef,
                (*src).items.offset(j as isize) as *mut LookupRef as *const LookupRef,
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
unsafe extern "C" fn otl_LookupRefList_dispose(mut arr: *mut LookupRefList) {
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
                (*arr).items.offset(j as isize) as *mut LookupRef,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<LookupRef>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_replace(
    mut dst: *mut LookupRefList,
    src: LookupRefList,
) {
    otl_LookupRefList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupRefList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_initCapN(mut arr: *mut LookupRefList, mut n: usize) {
    otl_LookupRefList_init(arr);
    otl_LookupRefList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_initN(mut arr: *mut LookupRefList, mut n: usize) {
    otl_LookupRefList_init(arr);
    otl_LookupRefList_growToN(arr, n);
    otl_LookupRefList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_free(mut x: *mut LookupRefList) {
    if x.is_null() {
        return;
    }
    otl_LookupRefList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_LookupRefList_growToN(arr: *mut LookupRefList, target: usize) {
    cvec_grow_to_n(otl_LookupRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn initFeaturePtr(mut feature: *mut FeaturePtr) {
    *feature = __caryll_allocate_clean(
        ::core::mem::size_of::<Feature>() as usize,
        61 as ::core::ffi::c_ulong,
    ) as FeaturePtr;
    otl_iLookupRefList.init.expect("non-null function pointer")(&raw mut (**feature).lookups);
}
#[inline]
unsafe extern "C" fn disposeFeaturePtr(mut feature: *mut FeaturePtr) {
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
    *feature = ::core::ptr::null_mut::<Feature>();
}
#[inline]
unsafe extern "C" fn otl_FeaturePtr_dispose(mut x: *mut FeaturePtr) {
    disposeFeaturePtr(x);
}
#[inline]
unsafe extern "C" fn otl_FeaturePtr_copy(
    mut dst: *mut FeaturePtr,
    mut src: *const FeaturePtr,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeaturePtr>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeaturePtr_replace(mut dst: *mut FeaturePtr, src: FeaturePtr) {
    otl_FeaturePtr_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeaturePtr>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeaturePtr_move(
    mut dst: *mut FeaturePtr,
    mut src: *mut FeaturePtr,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeaturePtr>() as usize,
    );
    otl_FeaturePtr_init(src);
}
#[inline]
unsafe extern "C" fn otl_FeaturePtr_init(mut x: *mut FeaturePtr) {
    initFeaturePtr(x);
}
pub static otl_iFeaturePtr: FeaturePtrElementInterface = {
    FeaturePtrElementInterface {
        init: Some(otl_FeaturePtr_init as unsafe extern "C" fn(*mut FeaturePtr) -> ()),
        copy: Some(
            otl_FeaturePtr_copy
                as unsafe extern "C" fn(*mut FeaturePtr, *const FeaturePtr) -> (),
        ),
        move_0: Some(
            otl_FeaturePtr_move
                as unsafe extern "C" fn(*mut FeaturePtr, *mut FeaturePtr) -> (),
        ),
        dispose: Some(otl_FeaturePtr_dispose as unsafe extern "C" fn(*mut FeaturePtr) -> ()),
        replace: Some(
            otl_FeaturePtr_replace
                as unsafe extern "C" fn(*mut FeaturePtr, FeaturePtr) -> (),
        ),
        copyReplace: Some(
            otl_FeaturePtr_copyReplace
                as unsafe extern "C" fn(*mut FeaturePtr, FeaturePtr) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_FeaturePtr_copyReplace(mut dst: *mut FeaturePtr, src: FeaturePtr) {
    otl_FeaturePtr_dispose(dst);
    otl_FeaturePtr_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_fill(mut arr: *mut FeatureList, mut n: usize) {
    while (*arr).length < n {
        let mut x: FeaturePtr = ::core::ptr::null_mut::<Feature>();
        if otl_iFeaturePtr.init.is_some() {
            otl_iFeaturePtr.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<FeaturePtr>() as usize,
            );
        }
        otl_FeatureList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_FeatureList_growTo(arr: *mut FeatureList, target: usize) {
    cvec_grow_to(otl_FeatureList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_growToN(arr: *mut FeatureList, target: usize) {
    cvec_grow_to_n(otl_FeatureList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_initN(mut arr: *mut FeatureList, mut n: usize) {
    otl_FeatureList_init(arr);
    otl_FeatureList_growToN(arr, n);
    otl_FeatureList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_free(mut x: *mut FeatureList) {
    if x.is_null() {
        return;
    }
    otl_FeatureList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_createN(mut n: usize) -> *mut FeatureList {
    let mut t: *mut FeatureList =
        malloc(::core::mem::size_of::<FeatureList>() as usize) as *mut FeatureList;
    otl_FeatureList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_FeatureList_create() -> *mut FeatureList {
    let mut x: *mut FeatureList =
        malloc(::core::mem::size_of::<FeatureList>() as usize) as *mut FeatureList;
    otl_FeatureList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_FeatureList_sort(
    mut arr: *mut FeatureList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const FeaturePtr, *const FeaturePtr) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<FeaturePtr>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const FeaturePtr,
                    *const FeaturePtr,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureList_push(arr: *mut FeatureList, elem: FeaturePtr) {
    cvec_push(otl_FeatureList_as_cvec(arr), elem);
}
#[inline]
unsafe fn otl_FeatureList_as_cvec(arr: *mut FeatureList) -> *mut CVecRaw<FeaturePtr> {
    arr as *mut CVecRaw<FeaturePtr>
}
#[inline]
unsafe extern "C" fn otl_FeatureList_init(arr: *mut FeatureList) {
    cvec_init(otl_FeatureList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_FeatureList_pop(arr: *mut FeatureList) -> FeaturePtr {
    cvec_pop(otl_FeatureList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_FeatureList_copyReplace(
    mut dst: *mut FeatureList,
    src: FeatureList,
) {
    otl_FeatureList_dispose(dst);
    otl_FeatureList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_copy(
    mut dst: *mut FeatureList,
    mut src: *const FeatureList,
) {
    otl_FeatureList_init(dst);
    otl_FeatureList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iFeaturePtr.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iFeaturePtr.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut FeaturePtr,
                (*src).items.offset(j as isize) as *mut FeaturePtr as *const FeaturePtr,
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
unsafe extern "C" fn otl_FeatureList_grow(arr: *mut FeatureList) {
    cvec_grow(otl_FeatureList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_FeatureList_dispose(mut arr: *mut FeatureList) {
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
                (*arr).items.offset(j as isize) as *mut FeaturePtr,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<FeaturePtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_FeatureList_replace(mut dst: *mut FeatureList, src: FeatureList) {
    otl_FeatureList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeatureList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureList_initCapN(mut arr: *mut FeatureList, mut n: usize) {
    otl_FeatureList_init(arr);
    otl_FeatureList_growToN(arr, n);
}
pub static otl_iFeatureList: FeatureListVectorInterface = {
    FeatureListVectorInterface {
        init: Some(otl_FeatureList_init as unsafe extern "C" fn(*mut FeatureList) -> ()),
        copy: Some(
            otl_FeatureList_copy
                as unsafe extern "C" fn(*mut FeatureList, *const FeatureList) -> (),
        ),
        move_0: Some(
            otl_FeatureList_move
                as unsafe extern "C" fn(*mut FeatureList, *mut FeatureList) -> (),
        ),
        dispose: Some(otl_FeatureList_dispose as unsafe extern "C" fn(*mut FeatureList) -> ()),
        replace: Some(
            otl_FeatureList_replace
                as unsafe extern "C" fn(*mut FeatureList, FeatureList) -> (),
        ),
        copyReplace: Some(
            otl_FeatureList_copyReplace
                as unsafe extern "C" fn(*mut FeatureList, FeatureList) -> (),
        ),
        create: Some(otl_FeatureList_create),
        free: Some(otl_FeatureList_free as unsafe extern "C" fn(*mut FeatureList) -> ()),
        initN: Some(
            otl_FeatureList_initN as unsafe extern "C" fn(*mut FeatureList, usize) -> (),
        ),
        initCapN: Some(
            otl_FeatureList_initCapN as unsafe extern "C" fn(*mut FeatureList, usize) -> (),
        ),
        createN: Some(
            otl_FeatureList_createN as unsafe extern "C" fn(usize) -> *mut FeatureList,
        ),
        fill: Some(
            otl_FeatureList_fill as unsafe extern "C" fn(*mut FeatureList, usize) -> (),
        ),
        clear: Some(otl_FeatureList_dispose as unsafe extern "C" fn(*mut FeatureList) -> ()),
        push: Some(
            otl_FeatureList_push
                as unsafe extern "C" fn(*mut FeatureList, FeaturePtr) -> (),
        ),
        shrinkToFit: Some(
            otl_FeatureList_shrinkToFit as unsafe extern "C" fn(*mut FeatureList) -> (),
        ),
        pop: Some(
            otl_FeatureList_pop as unsafe extern "C" fn(*mut FeatureList) -> FeaturePtr,
        ),
        disposeItem: Some(
            otl_FeatureList_disposeItem as unsafe extern "C" fn(*mut FeatureList, usize) -> (),
        ),
        filterEnv: Some(
            otl_FeatureList_filterEnv
                as unsafe extern "C" fn(
                    *mut FeatureList,
                    Option<
                        unsafe extern "C" fn(
                            *const FeaturePtr,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_FeatureList_sort
                as unsafe extern "C" fn(
                    *mut FeatureList,
                    Option<
                        unsafe extern "C" fn(
                            *const FeaturePtr,
                            *const FeaturePtr,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_FeatureList_shrinkToFit(mut arr: *mut FeatureList) {
    otl_FeatureList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_resizeTo(arr: *mut FeatureList, target: usize) {
    cvec_resize_to(otl_FeatureList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureList_move(dst: *mut FeatureList, src: *mut FeatureList) {
    cvec_move(otl_FeatureList_as_cvec(dst), otl_FeatureList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_FeatureList_filterEnv(
    mut arr: *mut FeatureList,
    mut fn_0: Option<unsafe extern "C" fn(*const FeaturePtr, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut FeaturePtr,
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
                    (*arr).items.offset(k as isize) as *mut FeaturePtr,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn otl_FeatureList_disposeItem(mut arr: *mut FeatureList, mut n: usize) {
    if otl_iFeaturePtr.dispose.is_some() {
        otl_iFeaturePtr.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut FeaturePtr,
        );
    } else {
    };
}
pub static otl_iFeatureRef: FeatureRefElementInterface = {
    FeatureRefElementInterface {
        init: Some(otl_FeatureRef_init as unsafe extern "C" fn(*mut FeatureRef) -> ()),
        copy: Some(
            otl_FeatureRef_copy
                as unsafe extern "C" fn(*mut FeatureRef, *const FeatureRef) -> (),
        ),
        move_0: Some(
            otl_FeatureRef_move
                as unsafe extern "C" fn(*mut FeatureRef, *mut FeatureRef) -> (),
        ),
        dispose: Some(otl_FeatureRef_dispose as unsafe extern "C" fn(*mut FeatureRef) -> ()),
        replace: Some(
            otl_FeatureRef_replace
                as unsafe extern "C" fn(*mut FeatureRef, FeatureRef) -> (),
        ),
        copyReplace: Some(
            otl_FeatureRef_copyReplace
                as unsafe extern "C" fn(*mut FeatureRef, FeatureRef) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_FeatureRef_copyReplace(mut dst: *mut FeatureRef, src: FeatureRef) {
    otl_FeatureRef_dispose(dst);
    otl_FeatureRef_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_FeatureRef_copy(
    mut dst: *mut FeatureRef,
    mut src: *const FeatureRef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeatureRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureRef_dispose(mut _x: *mut FeatureRef) {}
#[inline]
unsafe extern "C" fn otl_FeatureRef_replace(mut dst: *mut FeatureRef, src: FeatureRef) {
    otl_FeatureRef_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeatureRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureRef_move(
    mut dst: *mut FeatureRef,
    mut src: *mut FeatureRef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeatureRef>() as usize,
    );
    otl_FeatureRef_init(src);
}
#[inline]
unsafe extern "C" fn otl_FeatureRef_init(mut x: *mut FeatureRef) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<FeatureRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_fill(mut arr: *mut FeatureRefList, mut n: usize) {
    while (*arr).length < n {
        let mut x: FeatureRef = ::core::ptr::null::<Feature>();
        if otl_iFeatureRef.init.is_some() {
            otl_iFeatureRef.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<FeatureRef>() as usize,
            );
        }
        otl_FeatureRefList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_growTo(arr: *mut FeatureRefList, target: usize) {
    cvec_grow_to(otl_FeatureRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_pop(arr: *mut FeatureRefList) -> FeatureRef {
    cvec_pop(otl_FeatureRefList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_copyReplace(
    mut dst: *mut FeatureRefList,
    src: FeatureRefList,
) {
    otl_FeatureRefList_dispose(dst);
    otl_FeatureRefList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_copy(
    mut dst: *mut FeatureRefList,
    mut src: *const FeatureRefList,
) {
    otl_FeatureRefList_init(dst);
    otl_FeatureRefList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iFeatureRef.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iFeatureRef.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut FeatureRef,
                (*src).items.offset(j as isize) as *mut FeatureRef as *const FeatureRef,
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
unsafe extern "C" fn otl_FeatureRefList_dispose(mut arr: *mut FeatureRefList) {
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
                (*arr).items.offset(j as isize) as *mut FeatureRef,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<FeatureRef>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_replace(
    mut dst: *mut FeatureRefList,
    src: FeatureRefList,
) {
    otl_FeatureRefList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeatureRefList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_initCapN(mut arr: *mut FeatureRefList, mut n: usize) {
    otl_FeatureRefList_init(arr);
    otl_FeatureRefList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_growToN(arr: *mut FeatureRefList, target: usize) {
    cvec_grow_to_n(otl_FeatureRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_initN(mut arr: *mut FeatureRefList, mut n: usize) {
    otl_FeatureRefList_init(arr);
    otl_FeatureRefList_growToN(arr, n);
    otl_FeatureRefList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_free(mut x: *mut FeatureRefList) {
    if x.is_null() {
        return;
    }
    otl_FeatureRefList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_createN(mut n: usize) -> *mut FeatureRefList {
    let mut t: *mut FeatureRefList =
        malloc(::core::mem::size_of::<FeatureRefList>() as usize) as *mut FeatureRefList;
    otl_FeatureRefList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_create() -> *mut FeatureRefList {
    let mut x: *mut FeatureRefList =
        malloc(::core::mem::size_of::<FeatureRefList>() as usize) as *mut FeatureRefList;
    otl_FeatureRefList_init(x);
    return x;
}
pub static otl_iFeatureRefList: FeatureRefListVectorInterface = {
    FeatureRefListVectorInterface {
        init: Some(otl_FeatureRefList_init as unsafe extern "C" fn(*mut FeatureRefList) -> ()),
        copy: Some(
            otl_FeatureRefList_copy
                as unsafe extern "C" fn(*mut FeatureRefList, *const FeatureRefList) -> (),
        ),
        move_0: Some(
            otl_FeatureRefList_move
                as unsafe extern "C" fn(*mut FeatureRefList, *mut FeatureRefList) -> (),
        ),
        dispose: Some(
            otl_FeatureRefList_dispose as unsafe extern "C" fn(*mut FeatureRefList) -> (),
        ),
        replace: Some(
            otl_FeatureRefList_replace
                as unsafe extern "C" fn(*mut FeatureRefList, FeatureRefList) -> (),
        ),
        copyReplace: Some(
            otl_FeatureRefList_copyReplace
                as unsafe extern "C" fn(*mut FeatureRefList, FeatureRefList) -> (),
        ),
        create: Some(otl_FeatureRefList_create),
        free: Some(otl_FeatureRefList_free as unsafe extern "C" fn(*mut FeatureRefList) -> ()),
        initN: Some(
            otl_FeatureRefList_initN as unsafe extern "C" fn(*mut FeatureRefList, usize) -> (),
        ),
        initCapN: Some(
            otl_FeatureRefList_initCapN
                as unsafe extern "C" fn(*mut FeatureRefList, usize) -> (),
        ),
        createN: Some(
            otl_FeatureRefList_createN as unsafe extern "C" fn(usize) -> *mut FeatureRefList,
        ),
        fill: Some(
            otl_FeatureRefList_fill as unsafe extern "C" fn(*mut FeatureRefList, usize) -> (),
        ),
        clear: Some(
            otl_FeatureRefList_dispose as unsafe extern "C" fn(*mut FeatureRefList) -> (),
        ),
        push: Some(
            otl_FeatureRefList_push
                as unsafe extern "C" fn(*mut FeatureRefList, FeatureRef) -> (),
        ),
        shrinkToFit: Some(
            otl_FeatureRefList_shrinkToFit as unsafe extern "C" fn(*mut FeatureRefList) -> (),
        ),
        pop: Some(
            otl_FeatureRefList_pop
                as unsafe extern "C" fn(*mut FeatureRefList) -> FeatureRef,
        ),
        disposeItem: Some(
            otl_FeatureRefList_disposeItem
                as unsafe extern "C" fn(*mut FeatureRefList, usize) -> (),
        ),
        filterEnv: Some(
            otl_FeatureRefList_filterEnv
                as unsafe extern "C" fn(
                    *mut FeatureRefList,
                    Option<
                        unsafe extern "C" fn(
                            *const FeatureRef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_FeatureRefList_sort
                as unsafe extern "C" fn(
                    *mut FeatureRefList,
                    Option<
                        unsafe extern "C" fn(
                            *const FeatureRef,
                            *const FeatureRef,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_FeatureRefList_shrinkToFit(mut arr: *mut FeatureRefList) {
    otl_FeatureRefList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_resizeTo(arr: *mut FeatureRefList, target: usize) {
    cvec_resize_to(otl_FeatureRefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_move(dst: *mut FeatureRefList, src: *mut FeatureRefList) {
    cvec_move(otl_FeatureRefList_as_cvec(dst), otl_FeatureRefList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_filterEnv(
    mut arr: *mut FeatureRefList,
    mut fn_0: Option<unsafe extern "C" fn(*const FeatureRef, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut FeatureRef,
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
                    (*arr).items.offset(k as isize) as *mut FeatureRef,
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
    mut arr: *mut FeatureRefList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const FeatureRef, *const FeatureRef) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<FeatureRef>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const FeatureRef,
                    *const FeatureRef,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_push(arr: *mut FeatureRefList, elem: FeatureRef) {
    cvec_push(otl_FeatureRefList_as_cvec(arr), elem);
}
#[inline]
unsafe fn otl_FeatureRefList_as_cvec(arr: *mut FeatureRefList) -> *mut CVecRaw<FeatureRef> {
    arr as *mut CVecRaw<FeatureRef>
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_init(arr: *mut FeatureRefList) {
    cvec_init(otl_FeatureRefList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_grow(arr: *mut FeatureRefList) {
    cvec_grow(otl_FeatureRefList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_FeatureRefList_disposeItem(
    mut arr: *mut FeatureRefList,
    mut n: usize,
) {
    if otl_iFeatureRef.dispose.is_some() {
        otl_iFeatureRef.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut FeatureRef,
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn initLanguagePtr(mut language: *mut LanguageSystemPtr) {
    *language = __caryll_allocate_clean(
        ::core::mem::size_of::<LanguageSystem>() as usize,
        77 as ::core::ffi::c_ulong,
    ) as LanguageSystemPtr;
    otl_iFeatureRefList.init.expect("non-null function pointer")(&raw mut (**language).features);
}
#[inline]
unsafe extern "C" fn disposeLanguagePtr(mut language: *mut LanguageSystemPtr) {
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
    *language = ::core::ptr::null_mut::<LanguageSystem>();
}
pub static otl_iLanguageSystem: LanguageSystemPtrElementInterface = {
    LanguageSystemPtrElementInterface {
        init: Some(initLanguagePtr as unsafe extern "C" fn(*mut LanguageSystemPtr) -> ()),
        copy: None,
        move_0: None,
        dispose: Some(disposeLanguagePtr as unsafe extern "C" fn(*mut LanguageSystemPtr) -> ()),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn otl_LangSystemList_filterEnv(
    mut arr: *mut LangSystemList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const LanguageSystemPtr, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut LanguageSystemPtr,
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
                    (*arr).items.offset(k as isize) as *mut LanguageSystemPtr,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn otl_LangSystemList_as_cvec(arr: *mut LangSystemList) -> *mut CVecRaw<LanguageSystemPtr> {
    arr as *mut CVecRaw<LanguageSystemPtr>
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_init(arr: *mut LangSystemList) {
    cvec_init(otl_LangSystemList_as_cvec(arr));
}
pub static otl_iLangSystemList: LangSystemListVectorInterface = {
    LangSystemListVectorInterface {
        init: Some(otl_LangSystemList_init as unsafe extern "C" fn(*mut LangSystemList) -> ()),
        copy: Some(
            otl_LangSystemList_copy
                as unsafe extern "C" fn(*mut LangSystemList, *const LangSystemList) -> (),
        ),
        move_0: Some(
            otl_LangSystemList_move
                as unsafe extern "C" fn(*mut LangSystemList, *mut LangSystemList) -> (),
        ),
        dispose: Some(
            otl_LangSystemList_dispose as unsafe extern "C" fn(*mut LangSystemList) -> (),
        ),
        replace: Some(
            otl_LangSystemList_replace
                as unsafe extern "C" fn(*mut LangSystemList, LangSystemList) -> (),
        ),
        copyReplace: Some(
            otl_LangSystemList_copyReplace
                as unsafe extern "C" fn(*mut LangSystemList, LangSystemList) -> (),
        ),
        create: Some(otl_LangSystemList_create),
        free: Some(otl_LangSystemList_free as unsafe extern "C" fn(*mut LangSystemList) -> ()),
        initN: Some(
            otl_LangSystemList_initN as unsafe extern "C" fn(*mut LangSystemList, usize) -> (),
        ),
        initCapN: Some(
            otl_LangSystemList_initCapN
                as unsafe extern "C" fn(*mut LangSystemList, usize) -> (),
        ),
        createN: Some(
            otl_LangSystemList_createN as unsafe extern "C" fn(usize) -> *mut LangSystemList,
        ),
        fill: Some(
            otl_LangSystemList_fill as unsafe extern "C" fn(*mut LangSystemList, usize) -> (),
        ),
        clear: Some(
            otl_LangSystemList_dispose as unsafe extern "C" fn(*mut LangSystemList) -> (),
        ),
        push: Some(
            otl_LangSystemList_push
                as unsafe extern "C" fn(*mut LangSystemList, LanguageSystemPtr) -> (),
        ),
        shrinkToFit: Some(
            otl_LangSystemList_shrinkToFit as unsafe extern "C" fn(*mut LangSystemList) -> (),
        ),
        pop: Some(
            otl_LangSystemList_pop
                as unsafe extern "C" fn(*mut LangSystemList) -> LanguageSystemPtr,
        ),
        disposeItem: Some(
            otl_LangSystemList_disposeItem
                as unsafe extern "C" fn(*mut LangSystemList, usize) -> (),
        ),
        filterEnv: Some(
            otl_LangSystemList_filterEnv
                as unsafe extern "C" fn(
                    *mut LangSystemList,
                    Option<
                        unsafe extern "C" fn(
                            *const LanguageSystemPtr,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            otl_LangSystemList_sort
                as unsafe extern "C" fn(
                    *mut LangSystemList,
                    Option<
                        unsafe extern "C" fn(
                            *const LanguageSystemPtr,
                            *const LanguageSystemPtr,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_LangSystemList_disposeItem(
    mut arr: *mut LangSystemList,
    mut n: usize,
) {
    if otl_iLanguageSystem.dispose.is_some() {
        otl_iLanguageSystem
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut LanguageSystemPtr
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_sort(
    mut arr: *mut LangSystemList,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const LanguageSystemPtr,
            *const LanguageSystemPtr,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<LanguageSystemPtr>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const LanguageSystemPtr,
                    *const LanguageSystemPtr,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_fill(mut arr: *mut LangSystemList, mut n: usize) {
    while (*arr).length < n {
        let mut x: LanguageSystemPtr = ::core::ptr::null_mut::<LanguageSystem>();
        if otl_iLanguageSystem.init.is_some() {
            otl_iLanguageSystem.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<LanguageSystemPtr>() as usize,
            );
        }
        otl_LangSystemList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_push(arr: *mut LangSystemList, elem: LanguageSystemPtr) {
    cvec_push(otl_LangSystemList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_grow(arr: *mut LangSystemList) {
    cvec_grow(otl_LangSystemList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_growTo(arr: *mut LangSystemList, target: usize) {
    cvec_grow_to(otl_LangSystemList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_pop(arr: *mut LangSystemList) -> LanguageSystemPtr {
    cvec_pop(otl_LangSystemList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_copyReplace(
    mut dst: *mut LangSystemList,
    src: LangSystemList,
) {
    otl_LangSystemList_dispose(dst);
    otl_LangSystemList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_copy(
    mut dst: *mut LangSystemList,
    mut src: *const LangSystemList,
) {
    otl_LangSystemList_init(dst);
    otl_LangSystemList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otl_iLanguageSystem.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otl_iLanguageSystem.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut LanguageSystemPtr,
                (*src).items.offset(j as isize) as *mut LanguageSystemPtr
                    as *const LanguageSystemPtr,
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
unsafe extern "C" fn otl_LangSystemList_dispose(mut arr: *mut LangSystemList) {
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
                (*arr).items.offset(j as isize) as *mut LanguageSystemPtr
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<LanguageSystemPtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_replace(
    mut dst: *mut LangSystemList,
    src: LangSystemList,
) {
    otl_LangSystemList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LangSystemList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_initCapN(mut arr: *mut LangSystemList, mut n: usize) {
    otl_LangSystemList_init(arr);
    otl_LangSystemList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_growToN(arr: *mut LangSystemList, target: usize) {
    cvec_grow_to_n(otl_LangSystemList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_initN(mut arr: *mut LangSystemList, mut n: usize) {
    otl_LangSystemList_init(arr);
    otl_LangSystemList_growToN(arr, n);
    otl_LangSystemList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_free(mut x: *mut LangSystemList) {
    if x.is_null() {
        return;
    }
    otl_LangSystemList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_createN(mut n: usize) -> *mut LangSystemList {
    let mut t: *mut LangSystemList =
        malloc(::core::mem::size_of::<LangSystemList>() as usize) as *mut LangSystemList;
    otl_LangSystemList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_create() -> *mut LangSystemList {
    let mut x: *mut LangSystemList =
        malloc(::core::mem::size_of::<LangSystemList>() as usize) as *mut LangSystemList;
    otl_LangSystemList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_shrinkToFit(mut arr: *mut LangSystemList) {
    otl_LangSystemList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_resizeTo(arr: *mut LangSystemList, target: usize) {
    cvec_resize_to(otl_LangSystemList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_LangSystemList_move(dst: *mut LangSystemList, src: *mut LangSystemList) {
    cvec_move(otl_LangSystemList_as_cvec(dst), otl_LangSystemList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn initOTL(mut table: *mut OtlTable) {
    otl_iLookupList.init.expect("non-null function pointer")(&raw mut (*table).lookups);
    otl_iFeatureList.init.expect("non-null function pointer")(&raw mut (*table).features);
    otl_iLangSystemList.init.expect("non-null function pointer")(&raw mut (*table).languages);
}
#[inline]
unsafe extern "C" fn disposeOTL(mut table: *mut OtlTable) {
    otl_iLookupList.dispose.expect("non-null function pointer")(&raw mut (*table).lookups);
    otl_iFeatureList.dispose.expect("non-null function pointer")(&raw mut (*table).features);
    otl_iLangSystemList
        .dispose
        .expect("non-null function pointer")(&raw mut (*table).languages);
}
#[inline]
unsafe extern "C" fn table_OTL_dispose(mut x: *mut OtlTable) {
    disposeOTL(x);
}
#[inline]
unsafe extern "C" fn table_OTL_copyReplace(mut dst: *mut OtlTable, src: OtlTable) {
    table_OTL_dispose(dst);
    table_OTL_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_OTL_free(mut x: *mut OtlTable) {
    if x.is_null() {
        return;
    }
    table_OTL_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_OTL_create() -> *mut OtlTable {
    let mut x: *mut OtlTable =
        malloc(::core::mem::size_of::<OtlTable>() as usize) as *mut OtlTable;
    table_OTL_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_OTL_init(mut x: *mut OtlTable) {
    initOTL(x);
}
pub static table_iOTL: OtlTableElementInterface = {
    OtlTableElementInterface {
        init: Some(table_OTL_init as unsafe extern "C" fn(*mut OtlTable) -> ()),
        copy: Some(table_OTL_copy as unsafe extern "C" fn(*mut OtlTable, *const OtlTable) -> ()),
        move_0: Some(table_OTL_move as unsafe extern "C" fn(*mut OtlTable, *mut OtlTable) -> ()),
        dispose: Some(table_OTL_dispose as unsafe extern "C" fn(*mut OtlTable) -> ()),
        replace: Some(table_OTL_replace as unsafe extern "C" fn(*mut OtlTable, OtlTable) -> ()),
        copyReplace: Some(
            table_OTL_copyReplace as unsafe extern "C" fn(*mut OtlTable, OtlTable) -> (),
        ),
        create: Some(table_OTL_create),
        free: Some(table_OTL_free as unsafe extern "C" fn(*mut OtlTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_OTL_copy(mut dst: *mut OtlTable, mut src: *const OtlTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<OtlTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_OTL_replace(mut dst: *mut OtlTable, src: OtlTable) {
    table_OTL_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<OtlTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_OTL_move(mut dst: *mut OtlTable, mut src: *mut OtlTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<OtlTable>() as usize,
    );
    table_OTL_init(src);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MarkArrayVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut MarkArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut MarkArray, *const MarkArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut MarkArray, *mut MarkArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut MarkArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut MarkArray, MarkArray) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut MarkArray, MarkArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut MarkArray>,
    pub free: Option<unsafe extern "C" fn(*mut MarkArray) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut MarkArray, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut MarkArray, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut MarkArray>,
    pub fill: Option<unsafe extern "C" fn(*mut MarkArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut MarkArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut MarkArray, MarkRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut MarkArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut MarkArray) -> MarkRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut MarkArray, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut MarkArray,
            Option<unsafe extern "C" fn(*const MarkRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut MarkArray,
            Option<
                unsafe extern "C" fn(
                    *const MarkRecord,
                    *const MarkRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseArrayVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut BaseArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut BaseArray, *const BaseArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut BaseArray, *mut BaseArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut BaseArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut BaseArray, BaseArray) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut BaseArray, BaseArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut BaseArray>,
    pub free: Option<unsafe extern "C" fn(*mut BaseArray) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut BaseArray, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut BaseArray, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut BaseArray>,
    pub fill: Option<unsafe extern "C" fn(*mut BaseArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut BaseArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut BaseArray, BaseRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut BaseArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut BaseArray) -> BaseRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut BaseArray, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut BaseArray,
            Option<unsafe extern "C" fn(*const BaseRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut BaseArray,
            Option<
                unsafe extern "C" fn(
                    *const BaseRecord,
                    *const BaseRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LigatureArrayVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut LigatureArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut LigatureArray, *const LigatureArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut LigatureArray, *mut LigatureArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut LigatureArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut LigatureArray, LigatureArray) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut LigatureArray, LigatureArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LigatureArray>,
    pub free: Option<unsafe extern "C" fn(*mut LigatureArray) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut LigatureArray, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut LigatureArray, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut LigatureArray>,
    pub fill: Option<unsafe extern "C" fn(*mut LigatureArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut LigatureArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut LigatureArray, LigatureBaseRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut LigatureArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut LigatureArray) -> LigatureBaseRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut LigatureArray, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut LigatureArray,
            Option<
                unsafe extern "C" fn(
                    *const LigatureBaseRecord,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut LigatureArray,
            Option<
                unsafe extern "C" fn(
                    *const LigatureBaseRecord,
                    *const LigatureBaseRecord,
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
            LookupType::from_file(otl_type_gsub_unknown, 1),
            otl_type_gsub_single
        );
        assert_eq!(
            LookupType::from_file(otl_type_gpos_unknown, 9),
            otl_type_gpos_extend
        );
        // GSUB format 9 exists in no version of the spec otfcc knows; it stays
        // 25, gets no subtable, and reaches the output as `lookup_0019_…`.
        let unnamed = LookupType::from_file(otl_type_gsub_unknown, 9);
        assert_eq!(unnamed.raw(), 25);
        assert_eq!(unnamed.name(), c"unknown");
        assert_eq!(LookupType::from_file(otl_type_gsub_unknown, 0xffff).raw(), 65551);
        assert_eq!(::core::mem::size_of::<LookupType>(), 4);
    }
}
