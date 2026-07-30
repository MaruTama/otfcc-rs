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
use crate::table::otl::subtables::chaining::common::{I_SUBTABLE_CHAINING};
use crate::table::otl::subtables::gpos_cursive::{I_SUBTABLE_GPOS_CURSIVE};
use crate::table::otl::subtables::gpos_mark_to_ligature::{I_SUBTABLE_GPOS_MARK_TO_LIGATURE};
use crate::table::otl::subtables::gpos_mark_to_single::{I_SUBTABLE_GPOS_MARK_TO_SINGLE};
use crate::table::otl::subtables::gpos_pair::{I_SUBTABLE_GPOS_PAIR};
use crate::table::otl::subtables::gpos_single::{I_SUBTABLE_GPOS_SINGLE};
use crate::table::otl::subtables::gsub_ligature::{I_SUBTABLE_GSUB_LIGATURE};
use crate::table::otl::subtables::gsub_multi::{I_SUBTABLE_GSUB_MULTI};
use crate::table::otl::subtables::gsub_reverse::{I_SUBTABLE_GSUB_REVERSE};
use crate::table::otl::subtables::gsub_single::{I_SUBTABLE_GSUB_SINGLE};
use crate::vendor::sds::{sdsfree};


/// Which gsub/gpos subtable format a lookup is, in otfcc's own numbering: the
/// file's 16-bit format number offset by the table's base, `otl_type_gsub_*`
/// starting at 16 and `otl_type_gpos_*` at 32, so one value names both the
/// table and the format.
///
/// **Deliberately not an `enum`.** The value is read from the font:
/// `otfcc_read_otl_common` does `lookup->type = read_16u(data) + base`, so
/// anything in `16..=65551` can turn up, and C does not clamp it. An
/// unrecognised type is carried through as-is — `otfcc_read_otl_subtable`
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

pub const OTL_TYPE_GPOS_EXTEND: LookupType = LookupType(41);
pub const OTL_TYPE_GPOS_CHAINING: LookupType = LookupType(40);
pub const OTL_TYPE_GPOS_CONTEXT: LookupType = LookupType(39);
pub const OTL_TYPE_GPOS_MARK_TO_MARK: LookupType = LookupType(38);
pub const OTL_TYPE_GPOS_MARK_TO_LIGATURE: LookupType = LookupType(37);
pub const OTL_TYPE_GPOS_MARK_TO_BASE: LookupType = LookupType(36);
pub const OTL_TYPE_GPOS_CURSIVE: LookupType = LookupType(35);
pub const OTL_TYPE_GPOS_PAIR: LookupType = LookupType(34);
pub const OTL_TYPE_GPOS_SINGLE: LookupType = LookupType(33);
pub const OTL_TYPE_GPOS_UNKNOWN: LookupType = LookupType(32);
pub const OTL_TYPE_GSUB_REVERSE: LookupType = LookupType(24);
pub const OTL_TYPE_GSUB_EXTEND: LookupType = LookupType(23);
pub const OTL_TYPE_GSUB_CHAINING: LookupType = LookupType(22);
pub const OTL_TYPE_GSUB_CONTEXT: LookupType = LookupType(21);
pub const OTL_TYPE_GSUB_LIGATURE: LookupType = LookupType(20);
pub const OTL_TYPE_GSUB_ALTERNATE: LookupType = LookupType(19);
pub const OTL_TYPE_GSUB_MULTIPLE: LookupType = LookupType(18);
pub const OTL_TYPE_GSUB_SINGLE: LookupType = LookupType(17);
pub const OTL_TYPE_GSUB_UNKNOWN: LookupType = LookupType(16);
pub const OTL_TYPE_UNKNOWN: LookupType = LookupType(0);

impl LookupType {
    /// The type of a lookup as the font file spells it: a format number
    /// relative to `base`, which is `OTL_TYPE_GSUB_UNKNOWN` for gsub and
    /// `OTL_TYPE_GPOS_UNKNOWN` for gpos. Wrapping, like the C addition it
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
    /// table's base taken off again, and 0 for anything at or below gsub's base.
    ///
    /// The comparisons are `>`, not `>=`, exactly as in C: `OTL_TYPE_UNKNOWN`
    /// and `OTL_TYPE_GSUB_UNKNOWN` give 0, while `OTL_TYPE_GPOS_UNKNOWN` (32)
    /// is above *gsub's* base and so reads as gsub format 16. That is a quirk
    /// of the original, reachable only from a font declaring a gpos lookup of
    /// format 0; the number reaches the lookup header, so it is reproduced
    /// rather than tidied. `file_format_undoes_the_table_base` pins it.
    pub const fn file_format(self) -> u32 {
        if self.0 > OTL_TYPE_GPOS_UNKNOWN.0 {
            self.0 - OTL_TYPE_GPOS_UNKNOWN.0
        } else if self.0 > OTL_TYPE_GSUB_UNKNOWN.0 {
            self.0 - OTL_TYPE_GSUB_UNKNOWN.0
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
    pub gpos_mark_to_single: GposMarkToSingleSubtable,
    pub gpos_mark_to_ligature: GposMarkToLigatureSubtable,
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
    pub class_count: GlyphClass,
    pub mark_array: MarkArray,
    pub lig_array: LigatureArray,
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
    pub component_count: GlyphId,
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
    pub mark_class: GlyphClass,
    pub anchor: Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposMarkToSingleSubtable {
    pub class_count: GlyphClass,
    pub mark_array: MarkArray,
    pub base_array: BaseArray,
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
    pub first_values: *mut *mut PositionValue,
    pub second_values: *mut *mut PositionValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PositionValue {
    pub dx: Pos,
    pub dy: Pos,
    pub d_width: Pos,
    pub d_height: Pos,
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
    pub match_count: TableId,
    pub input_index: TableId,
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
    pub rules_count: TableId,
    pub rules: *mut *mut ChainingRule,
    pub bc: *mut ClassDef,
    pub ic: *mut ClassDef,
    pub fc: *mut ClassDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainingRule {
    pub match_count: TableId,
    pub input_begins: TableId,
    pub input_ends: TableId,
    pub match_0: *mut *mut Coverage,
    pub apply_count: TableId,
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
    Canonical = 0,
    Poly = 1,
    Classified = 2,
}
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
    pub copy_replace:
        Option<unsafe extern "C" fn(*mut GsubSingleSubtable, GsubSingleSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GsubSingleSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut GsubSingleSubtable, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut GsubSingleSubtable, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut GsubSingleSubtable>,
    pub fill: Option<unsafe extern "C" fn(*mut GsubSingleSubtable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut GsubSingleSubtable, GsubSingleEntry) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> GsubSingleEntry>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut GsubSingleSubtable, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace:
        Option<unsafe extern "C" fn(*mut GsubMultiSubtable, GsubMultiSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GsubMultiSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut GsubMultiSubtable>,
    pub fill: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, GsubMultiEntry) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> GsubMultiEntry>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut GsubMultiSubtable, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace:
        Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, GsubLigatureSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GsubLigatureSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut GsubLigatureSubtable>,
    pub fill: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, GsubLigatureEntry) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> GsubLigatureEntry>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut GsubLigatureSubtable, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut ChainingSubtable, ChainingSubtable) -> ()>,
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
    pub copy_replace:
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
    pub copy_replace:
        Option<unsafe extern "C" fn(*mut GposSingleSubtable, GposSingleSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GposSingleSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut GposSingleSubtable>,
    pub fill: Option<unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut GposSingleSubtable, GposSingleEntry) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> GposSingleEntry>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut GposSingleSubtable, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace:
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
    pub copy_replace:
        Option<unsafe extern "C" fn(*mut GposCursiveSubtable, GposCursiveSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GposCursiveSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut GposCursiveSubtable>,
    pub fill: Option<unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut GposCursiveSubtable, GposCursiveEntry) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> GposCursiveEntry>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut GposCursiveSubtable, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace: Option<
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
    pub copy_replace: Option<
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut SubtableList, SubtableList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut SubtableList>,
    pub free: Option<unsafe extern "C" fn(*mut SubtableList) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut SubtableList, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut SubtableList, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut SubtableList>,
    pub fill: Option<unsafe extern "C" fn(*mut SubtableList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut SubtableList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut SubtableList, SubtablePtr) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut SubtableList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut SubtableList) -> SubtablePtr>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut SubtableList, usize) -> ()>,
    pub filter_env: Option<
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
    pub dispose_dependent:
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut SubtablePtr, SubtablePtr) -> ()>,
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut LookupPtr, LookupPtr) -> ()>,
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut LookupList, LookupList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LookupList>,
    pub free: Option<unsafe extern "C" fn(*mut LookupList) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut LookupList, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut LookupList, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut LookupList>,
    pub fill: Option<unsafe extern "C" fn(*mut LookupList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut LookupList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut LookupList, LookupPtr) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut LookupList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut LookupList) -> LookupPtr>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut LookupList, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut LookupRef, LookupRef) -> ()>,
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut LookupRefList, LookupRefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LookupRefList>,
    pub free: Option<unsafe extern "C" fn(*mut LookupRefList) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut LookupRefList, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut LookupRefList, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut LookupRefList>,
    pub fill: Option<unsafe extern "C" fn(*mut LookupRefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut LookupRefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut LookupRefList, LookupRef) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut LookupRefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut LookupRefList) -> LookupRef>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut LookupRefList, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut FeaturePtr, FeaturePtr) -> ()>,
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut FeatureList, FeatureList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut FeatureList>,
    pub free: Option<unsafe extern "C" fn(*mut FeatureList) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut FeatureList, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut FeatureList, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut FeatureList>,
    pub fill: Option<unsafe extern "C" fn(*mut FeatureList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut FeatureList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut FeatureList, FeaturePtr) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut FeatureList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut FeatureList) -> FeaturePtr>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut FeatureList, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut FeatureRef, FeatureRef) -> ()>,
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
    pub copy_replace:
        Option<unsafe extern "C" fn(*mut FeatureRefList, FeatureRefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut FeatureRefList>,
    pub free: Option<unsafe extern "C" fn(*mut FeatureRefList) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut FeatureRefList, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut FeatureRefList, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut FeatureRefList>,
    pub fill: Option<unsafe extern "C" fn(*mut FeatureRefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut FeatureRefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut FeatureRefList, FeatureRef) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut FeatureRefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut FeatureRefList) -> FeatureRef>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut FeatureRefList, usize) -> ()>,
    pub filter_env: Option<
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
    pub required_feature: FeatureRef,
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
    pub copy_replace:
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
    pub copy_replace:
        Option<unsafe extern "C" fn(*mut LangSystemList, LangSystemList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LangSystemList>,
    pub free: Option<unsafe extern "C" fn(*mut LangSystemList) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut LangSystemList, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut LangSystemList, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut LangSystemList>,
    pub fill: Option<unsafe extern "C" fn(*mut LangSystemList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut LangSystemList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut LangSystemList, LanguageSystemPtr) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut LangSystemList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut LangSystemList) -> LanguageSystemPtr>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut LangSystemList, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut OtlTable, OtlTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut OtlTable>,
    pub free: Option<unsafe extern "C" fn(*mut OtlTable) -> ()>,
}
#[inline]
unsafe extern "C" fn dispose_subtable_dependent(
    mut subtable_ref: *mut SubtablePtr,
    mut lookup: *const Lookup,
) {
    match (*lookup).type_0 {
        OTL_TYPE_GSUB_SINGLE => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubSingleSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GSUB_SINGLE.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GSUB_MULTIPLE => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GSUB_MULTI.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GSUB_ALTERNATE => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubMultiSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GSUB_MULTI.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GSUB_LIGATURE => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubLigatureSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GSUB_LIGATURE.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GSUB_CHAINING => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_CHAINING.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GSUB_REVERSE => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubReverseSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GSUB_REVERSE.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GPOS_SINGLE => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposSingleSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GPOS_SINGLE.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GPOS_PAIR => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposPairSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GPOS_PAIR.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GPOS_CURSIVE => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposCursiveSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GPOS_CURSIVE.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GPOS_CHAINING => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_CHAINING.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GPOS_MARK_TO_BASE => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GPOS_MARK_TO_SINGLE.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GPOS_MARK_TO_MARK => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposMarkToSingleSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GPOS_MARK_TO_SINGLE.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GPOS_MARK_TO_LIGATURE => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposMarkToLigatureSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GPOS_MARK_TO_LIGATURE.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        _ => {}
    };
}
static OTL_I_SUBTABLE_PTR: SubtablePtrElementInterface =
    SubtablePtrElementInterface {
        init: None,
        copy: None,
        move_0: None,
        dispose: None,
        replace: None,
        copy_replace: None,
    };
#[inline]
unsafe extern "C" fn otl_subtable_list_dispose_dependent(
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
        dispose_subtable_dependent(
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
unsafe extern "C" fn otl_subtable_list_filter_env(
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
            if OTL_I_SUBTABLE_PTR.dispose.is_some() {
                OTL_I_SUBTABLE_PTR.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_subtable_list_create_n(mut n: usize) -> *mut SubtableList {
    let mut t: *mut SubtableList =
        malloc(::core::mem::size_of::<SubtableList>() as usize) as *mut SubtableList;
    otl_subtable_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_subtable_list_sort(
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
unsafe extern "C" fn otl_subtable_list_shrink_to_fit(mut arr: *mut SubtableList) {
    otl_subtable_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_subtable_list_resize_to(arr: *mut SubtableList, target: usize) {
    cvec_resize_to(otl_subtable_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_subtable_list_move(dst: *mut SubtableList, src: *mut SubtableList) {
    cvec_move(otl_subtable_list_as_cvec(dst), otl_subtable_list_as_cvec(src));
}
#[inline]
unsafe fn otl_subtable_list_as_cvec(arr: *mut SubtableList) -> *mut CVecRaw<SubtablePtr> {
    arr as *mut CVecRaw<SubtablePtr>
}
#[inline]
unsafe extern "C" fn otl_subtable_list_init(arr: *mut SubtableList) {
    cvec_init(otl_subtable_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_subtable_list_free(mut x: *mut SubtableList) {
    if x.is_null() {
        return;
    }
    otl_subtable_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_subtable_list_create() -> *mut SubtableList {
    let mut x: *mut SubtableList =
        malloc(::core::mem::size_of::<SubtableList>() as usize) as *mut SubtableList;
    otl_subtable_list_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_subtable_list_fill(mut arr: *mut SubtableList, mut n: usize) {
    while (*arr).length < n {
        let mut x: SubtablePtr = ::core::ptr::null_mut::<Subtable>();
        if OTL_I_SUBTABLE_PTR.init.is_some() {
            OTL_I_SUBTABLE_PTR.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<SubtablePtr>() as usize,
            );
        }
        otl_subtable_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_subtable_list_dispose_item(mut arr: *mut SubtableList, mut n: usize) {
    if OTL_I_SUBTABLE_PTR.dispose.is_some() {
        OTL_I_SUBTABLE_PTR.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut SubtablePtr,
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_subtable_list_push(arr: *mut SubtableList, elem: SubtablePtr) {
    cvec_push(otl_subtable_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_subtable_list_grow(arr: *mut SubtableList) {
    cvec_grow(otl_subtable_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_subtable_list_grow_to(arr: *mut SubtableList, target: usize) {
    cvec_grow_to(otl_subtable_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_subtable_list_pop(arr: *mut SubtableList) -> SubtablePtr {
    cvec_pop(otl_subtable_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_subtable_list_copy_replace(
    mut dst: *mut SubtableList,
    src: SubtableList,
) {
    otl_subtable_list_dispose(dst);
    otl_subtable_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_subtable_list_copy(
    mut dst: *mut SubtableList,
    mut src: *const SubtableList,
) {
    otl_subtable_list_init(dst);
    otl_subtable_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if OTL_I_SUBTABLE_PTR.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            OTL_I_SUBTABLE_PTR.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_subtable_list_dispose(mut arr: *mut SubtableList) {
    if arr.is_null() {
        return;
    }
    if OTL_I_SUBTABLE_PTR.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh5 = j;
            j = j.wrapping_sub(1);
            if !(fresh5 != 0) {
                break;
            }
            OTL_I_SUBTABLE_PTR.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_subtable_list_replace(
    mut dst: *mut SubtableList,
    src: SubtableList,
) {
    otl_subtable_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<SubtableList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_subtable_list_init_cap_n(mut arr: *mut SubtableList, mut n: usize) {
    otl_subtable_list_init(arr);
    otl_subtable_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn otl_subtable_list_grow_to_n(arr: *mut SubtableList, target: usize) {
    cvec_grow_to_n(otl_subtable_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_subtable_list_init_n(mut arr: *mut SubtableList, mut n: usize) {
    otl_subtable_list_init(arr);
    otl_subtable_list_grow_to_n(arr, n);
    otl_subtable_list_fill(arr, n);
}
pub static OTL_I_SUBTABLE_LIST: SubtableListVectorInterface = {
    SubtableListVectorInterface {
        init: Some(otl_subtable_list_init as unsafe extern "C" fn(*mut SubtableList) -> ()),
        copy: Some(
            otl_subtable_list_copy
                as unsafe extern "C" fn(*mut SubtableList, *const SubtableList) -> (),
        ),
        move_0: Some(
            otl_subtable_list_move
                as unsafe extern "C" fn(*mut SubtableList, *mut SubtableList) -> (),
        ),
        dispose: Some(
            otl_subtable_list_dispose as unsafe extern "C" fn(*mut SubtableList) -> (),
        ),
        replace: Some(
            otl_subtable_list_replace
                as unsafe extern "C" fn(*mut SubtableList, SubtableList) -> (),
        ),
        copy_replace: Some(
            otl_subtable_list_copy_replace
                as unsafe extern "C" fn(*mut SubtableList, SubtableList) -> (),
        ),
        create: Some(otl_subtable_list_create),
        free: Some(otl_subtable_list_free as unsafe extern "C" fn(*mut SubtableList) -> ()),
        init_n: Some(
            otl_subtable_list_init_n as unsafe extern "C" fn(*mut SubtableList, usize) -> (),
        ),
        init_cap_n: Some(
            otl_subtable_list_init_cap_n as unsafe extern "C" fn(*mut SubtableList, usize) -> (),
        ),
        create_n: Some(
            otl_subtable_list_create_n as unsafe extern "C" fn(usize) -> *mut SubtableList,
        ),
        fill: Some(
            otl_subtable_list_fill as unsafe extern "C" fn(*mut SubtableList, usize) -> (),
        ),
        clear: Some(otl_subtable_list_dispose as unsafe extern "C" fn(*mut SubtableList) -> ()),
        push: Some(
            otl_subtable_list_push
                as unsafe extern "C" fn(*mut SubtableList, SubtablePtr) -> (),
        ),
        shrink_to_fit: Some(
            otl_subtable_list_shrink_to_fit as unsafe extern "C" fn(*mut SubtableList) -> (),
        ),
        pop: Some(
            otl_subtable_list_pop as unsafe extern "C" fn(*mut SubtableList) -> SubtablePtr,
        ),
        dispose_item: Some(
            otl_subtable_list_dispose_item
                as unsafe extern "C" fn(*mut SubtableList, usize) -> (),
        ),
        filter_env: Some(
            otl_subtable_list_filter_env
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
            otl_subtable_list_sort
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
        dispose_dependent: Some(
            otl_subtable_list_dispose_dependent
                as unsafe extern "C" fn(*mut SubtableList, *const Lookup) -> (),
        ),
    }
};
pub unsafe extern "C" fn otfcc_delete_lookup(mut lookup: *mut Lookup) {
    if lookup.is_null() {
        return;
    }
    OTL_I_SUBTABLE_LIST
        .dispose_dependent
        .expect("non-null function pointer")(&raw mut (*lookup).subtables, lookup);
    sdsfree((*lookup).name);
    free(lookup as *mut ::core::ffi::c_void);
    lookup = ::core::ptr::null_mut::<Lookup>();
}
#[inline]
unsafe extern "C" fn init_lookup_ptr(mut entry: *mut LookupPtr) {
    *entry = __caryll_allocate_clean(
        ::core::mem::size_of::<Lookup>() as usize,
        47 as ::core::ffi::c_ulong,
    ) as LookupPtr;
    (**entry).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    OTL_I_SUBTABLE_LIST.init.expect("non-null function pointer")(&raw mut (**entry).subtables);
}
#[inline]
unsafe extern "C" fn dispose_lookup_ptr(mut entry: *mut LookupPtr) {
    otfcc_delete_lookup(*entry);
}
pub static OTL_I_LOOKUP_PTR: LookupPtrElementInterface = {
    LookupPtrElementInterface {
        init: Some(otl_lookup_ptr_init as unsafe extern "C" fn(*mut LookupPtr) -> ()),
        copy: Some(
            otl_lookup_ptr_copy
                as unsafe extern "C" fn(*mut LookupPtr, *const LookupPtr) -> (),
        ),
        move_0: Some(
            otl_lookup_ptr_move
                as unsafe extern "C" fn(*mut LookupPtr, *mut LookupPtr) -> (),
        ),
        dispose: Some(otl_lookup_ptr_dispose as unsafe extern "C" fn(*mut LookupPtr) -> ()),
        replace: Some(
            otl_lookup_ptr_replace as unsafe extern "C" fn(*mut LookupPtr, LookupPtr) -> (),
        ),
        copy_replace: Some(
            otl_lookup_ptr_copy_replace
                as unsafe extern "C" fn(*mut LookupPtr, LookupPtr) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_lookup_ptr_dispose(mut x: *mut LookupPtr) {
    dispose_lookup_ptr(x);
}
#[inline]
unsafe extern "C" fn otl_lookup_ptr_copy(
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
unsafe extern "C" fn otl_lookup_ptr_copy_replace(mut dst: *mut LookupPtr, src: LookupPtr) {
    otl_lookup_ptr_dispose(dst);
    otl_lookup_ptr_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_lookup_ptr_replace(mut dst: *mut LookupPtr, src: LookupPtr) {
    otl_lookup_ptr_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupPtr>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_lookup_ptr_move(mut dst: *mut LookupPtr, mut src: *mut LookupPtr) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupPtr>() as usize,
    );
    otl_lookup_ptr_init(src);
}
#[inline]
unsafe extern "C" fn otl_lookup_ptr_init(mut x: *mut LookupPtr) {
    init_lookup_ptr(x);
}
#[inline]
unsafe extern "C" fn otl_lookup_list_resize_to(arr: *mut LookupList, target: usize) {
    cvec_resize_to(otl_lookup_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_lookup_list_shrink_to_fit(mut arr: *mut LookupList) {
    otl_lookup_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_lookup_list_move(dst: *mut LookupList, src: *mut LookupList) {
    cvec_move(otl_lookup_list_as_cvec(dst), otl_lookup_list_as_cvec(src));
}
#[inline]
unsafe fn otl_lookup_list_as_cvec(arr: *mut LookupList) -> *mut CVecRaw<LookupPtr> {
    arr as *mut CVecRaw<LookupPtr>
}
#[inline]
unsafe extern "C" fn otl_lookup_list_init(arr: *mut LookupList) {
    cvec_init(otl_lookup_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_lookup_list_filter_env(
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
            if OTL_I_LOOKUP_PTR.dispose.is_some() {
                OTL_I_LOOKUP_PTR.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_lookup_list_dispose_item(mut arr: *mut LookupList, mut n: usize) {
    if OTL_I_LOOKUP_PTR.dispose.is_some() {
        OTL_I_LOOKUP_PTR.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut LookupPtr
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_lookup_list_sort(
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
unsafe extern "C" fn otl_lookup_list_fill(mut arr: *mut LookupList, mut n: usize) {
    while (*arr).length < n {
        let mut x: LookupPtr = ::core::ptr::null_mut::<Lookup>();
        if OTL_I_LOOKUP_PTR.init.is_some() {
            OTL_I_LOOKUP_PTR.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<LookupPtr>() as usize,
            );
        }
        otl_lookup_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_lookup_list_push(arr: *mut LookupList, elem: LookupPtr) {
    cvec_push(otl_lookup_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_lookup_list_grow(arr: *mut LookupList) {
    cvec_grow(otl_lookup_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_lookup_list_grow_to(arr: *mut LookupList, target: usize) {
    cvec_grow_to(otl_lookup_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_lookup_list_pop(arr: *mut LookupList) -> LookupPtr {
    cvec_pop(otl_lookup_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_lookup_list_copy_replace(mut dst: *mut LookupList, src: LookupList) {
    otl_lookup_list_dispose(dst);
    otl_lookup_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_lookup_list_copy(
    mut dst: *mut LookupList,
    mut src: *const LookupList,
) {
    otl_lookup_list_init(dst);
    otl_lookup_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if OTL_I_LOOKUP_PTR.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            OTL_I_LOOKUP_PTR.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_lookup_list_dispose(mut arr: *mut LookupList) {
    if arr.is_null() {
        return;
    }
    if OTL_I_LOOKUP_PTR.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh10 = j;
            j = j.wrapping_sub(1);
            if !(fresh10 != 0) {
                break;
            }
            OTL_I_LOOKUP_PTR.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_lookup_list_replace(mut dst: *mut LookupList, src: LookupList) {
    otl_lookup_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_lookup_list_init_cap_n(mut arr: *mut LookupList, mut n: usize) {
    otl_lookup_list_init(arr);
    otl_lookup_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn otl_lookup_list_grow_to_n(arr: *mut LookupList, target: usize) {
    cvec_grow_to_n(otl_lookup_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_lookup_list_init_n(mut arr: *mut LookupList, mut n: usize) {
    otl_lookup_list_init(arr);
    otl_lookup_list_grow_to_n(arr, n);
    otl_lookup_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_lookup_list_free(mut x: *mut LookupList) {
    if x.is_null() {
        return;
    }
    otl_lookup_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_lookup_list_create_n(mut n: usize) -> *mut LookupList {
    let mut t: *mut LookupList =
        malloc(::core::mem::size_of::<LookupList>() as usize) as *mut LookupList;
    otl_lookup_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_lookup_list_create() -> *mut LookupList {
    let mut x: *mut LookupList =
        malloc(::core::mem::size_of::<LookupList>() as usize) as *mut LookupList;
    otl_lookup_list_init(x);
    return x;
}
pub static OTL_I_LOOKUP_LIST: LookupListVectorInterface = {
    LookupListVectorInterface {
        init: Some(otl_lookup_list_init as unsafe extern "C" fn(*mut LookupList) -> ()),
        copy: Some(
            otl_lookup_list_copy
                as unsafe extern "C" fn(*mut LookupList, *const LookupList) -> (),
        ),
        move_0: Some(
            otl_lookup_list_move
                as unsafe extern "C" fn(*mut LookupList, *mut LookupList) -> (),
        ),
        dispose: Some(otl_lookup_list_dispose as unsafe extern "C" fn(*mut LookupList) -> ()),
        replace: Some(
            otl_lookup_list_replace
                as unsafe extern "C" fn(*mut LookupList, LookupList) -> (),
        ),
        copy_replace: Some(
            otl_lookup_list_copy_replace
                as unsafe extern "C" fn(*mut LookupList, LookupList) -> (),
        ),
        create: Some(otl_lookup_list_create),
        free: Some(otl_lookup_list_free as unsafe extern "C" fn(*mut LookupList) -> ()),
        init_n: Some(
            otl_lookup_list_init_n as unsafe extern "C" fn(*mut LookupList, usize) -> (),
        ),
        init_cap_n: Some(
            otl_lookup_list_init_cap_n as unsafe extern "C" fn(*mut LookupList, usize) -> (),
        ),
        create_n: Some(
            otl_lookup_list_create_n as unsafe extern "C" fn(usize) -> *mut LookupList,
        ),
        fill: Some(otl_lookup_list_fill as unsafe extern "C" fn(*mut LookupList, usize) -> ()),
        clear: Some(otl_lookup_list_dispose as unsafe extern "C" fn(*mut LookupList) -> ()),
        push: Some(
            otl_lookup_list_push as unsafe extern "C" fn(*mut LookupList, LookupPtr) -> (),
        ),
        shrink_to_fit: Some(
            otl_lookup_list_shrink_to_fit as unsafe extern "C" fn(*mut LookupList) -> (),
        ),
        pop: Some(otl_lookup_list_pop as unsafe extern "C" fn(*mut LookupList) -> LookupPtr),
        dispose_item: Some(
            otl_lookup_list_dispose_item as unsafe extern "C" fn(*mut LookupList, usize) -> (),
        ),
        filter_env: Some(
            otl_lookup_list_filter_env
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
            otl_lookup_list_sort
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
unsafe extern "C" fn otl_lookup_ref_dispose(mut _x: *mut LookupRef) {}
#[inline]
unsafe extern "C" fn otl_lookup_ref_copy_replace(mut dst: *mut LookupRef, src: LookupRef) {
    otl_lookup_ref_dispose(dst);
    otl_lookup_ref_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_move(mut dst: *mut LookupRef, mut src: *mut LookupRef) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupRef>() as usize,
    );
    otl_lookup_ref_init(src);
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_init(mut x: *mut LookupRef) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<LookupRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_copy(
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
unsafe extern "C" fn otl_lookup_ref_replace(mut dst: *mut LookupRef, src: LookupRef) {
    otl_lookup_ref_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupRef>() as usize,
    );
}
pub static OTL_I_LOOKUP_REF: LookupRefElementInterface = {
    LookupRefElementInterface {
        init: Some(otl_lookup_ref_init as unsafe extern "C" fn(*mut LookupRef) -> ()),
        copy: Some(
            otl_lookup_ref_copy
                as unsafe extern "C" fn(*mut LookupRef, *const LookupRef) -> (),
        ),
        move_0: Some(
            otl_lookup_ref_move
                as unsafe extern "C" fn(*mut LookupRef, *mut LookupRef) -> (),
        ),
        dispose: Some(otl_lookup_ref_dispose as unsafe extern "C" fn(*mut LookupRef) -> ()),
        replace: Some(
            otl_lookup_ref_replace as unsafe extern "C" fn(*mut LookupRef, LookupRef) -> (),
        ),
        copy_replace: Some(
            otl_lookup_ref_copy_replace
                as unsafe extern "C" fn(*mut LookupRef, LookupRef) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_pop(arr: *mut LookupRefList) -> LookupRef {
    cvec_pop(otl_lookup_ref_list_as_cvec(arr))
}
pub static OTL_I_LOOKUP_REF_LIST: LookupRefListVectorInterface = {
    LookupRefListVectorInterface {
        init: Some(otl_lookup_ref_list_init as unsafe extern "C" fn(*mut LookupRefList) -> ()),
        copy: Some(
            otl_lookup_ref_list_copy
                as unsafe extern "C" fn(*mut LookupRefList, *const LookupRefList) -> (),
        ),
        move_0: Some(
            otl_lookup_ref_list_move
                as unsafe extern "C" fn(*mut LookupRefList, *mut LookupRefList) -> (),
        ),
        dispose: Some(
            otl_lookup_ref_list_dispose as unsafe extern "C" fn(*mut LookupRefList) -> (),
        ),
        replace: Some(
            otl_lookup_ref_list_replace
                as unsafe extern "C" fn(*mut LookupRefList, LookupRefList) -> (),
        ),
        copy_replace: Some(
            otl_lookup_ref_list_copy_replace
                as unsafe extern "C" fn(*mut LookupRefList, LookupRefList) -> (),
        ),
        create: Some(otl_lookup_ref_list_create),
        free: Some(otl_lookup_ref_list_free as unsafe extern "C" fn(*mut LookupRefList) -> ()),
        init_n: Some(
            otl_lookup_ref_list_init_n as unsafe extern "C" fn(*mut LookupRefList, usize) -> (),
        ),
        init_cap_n: Some(
            otl_lookup_ref_list_init_cap_n
                as unsafe extern "C" fn(*mut LookupRefList, usize) -> (),
        ),
        create_n: Some(
            otl_lookup_ref_list_create_n as unsafe extern "C" fn(usize) -> *mut LookupRefList,
        ),
        fill: Some(
            otl_lookup_ref_list_fill as unsafe extern "C" fn(*mut LookupRefList, usize) -> (),
        ),
        clear: Some(
            otl_lookup_ref_list_dispose as unsafe extern "C" fn(*mut LookupRefList) -> (),
        ),
        push: Some(
            otl_lookup_ref_list_push
                as unsafe extern "C" fn(*mut LookupRefList, LookupRef) -> (),
        ),
        shrink_to_fit: Some(
            otl_lookup_ref_list_shrink_to_fit as unsafe extern "C" fn(*mut LookupRefList) -> (),
        ),
        pop: Some(
            otl_lookup_ref_list_pop as unsafe extern "C" fn(*mut LookupRefList) -> LookupRef,
        ),
        dispose_item: Some(
            otl_lookup_ref_list_dispose_item
                as unsafe extern "C" fn(*mut LookupRefList, usize) -> (),
        ),
        filter_env: Some(
            otl_lookup_ref_list_filter_env
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
            otl_lookup_ref_list_sort
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
unsafe extern "C" fn otl_lookup_ref_list_shrink_to_fit(mut arr: *mut LookupRefList) {
    otl_lookup_ref_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_resize_to(arr: *mut LookupRefList, target: usize) {
    cvec_resize_to(otl_lookup_ref_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_move(dst: *mut LookupRefList, src: *mut LookupRefList) {
    cvec_move(otl_lookup_ref_list_as_cvec(dst), otl_lookup_ref_list_as_cvec(src));
}
#[inline]
unsafe fn otl_lookup_ref_list_as_cvec(arr: *mut LookupRefList) -> *mut CVecRaw<LookupRef> {
    arr as *mut CVecRaw<LookupRef>
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_init(arr: *mut LookupRefList) {
    cvec_init(otl_lookup_ref_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_filter_env(
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
            if OTL_I_LOOKUP_REF.dispose.is_some() {
                OTL_I_LOOKUP_REF.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_lookup_ref_list_dispose_item(mut arr: *mut LookupRefList, mut n: usize) {
    if OTL_I_LOOKUP_REF.dispose.is_some() {
        OTL_I_LOOKUP_REF.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut LookupRef
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_sort(
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
unsafe extern "C" fn otl_lookup_ref_list_fill(mut arr: *mut LookupRefList, mut n: usize) {
    while (*arr).length < n {
        let mut x: LookupRef = ::core::ptr::null::<Lookup>();
        if OTL_I_LOOKUP_REF.init.is_some() {
            OTL_I_LOOKUP_REF.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<LookupRef>() as usize,
            );
        }
        otl_lookup_ref_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_push(arr: *mut LookupRefList, elem: LookupRef) {
    cvec_push(otl_lookup_ref_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_create_n(mut n: usize) -> *mut LookupRefList {
    let mut t: *mut LookupRefList =
        malloc(::core::mem::size_of::<LookupRefList>() as usize) as *mut LookupRefList;
    otl_lookup_ref_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_grow(arr: *mut LookupRefList) {
    cvec_grow(otl_lookup_ref_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_grow_to(arr: *mut LookupRefList, target: usize) {
    cvec_grow_to(otl_lookup_ref_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_create() -> *mut LookupRefList {
    let mut x: *mut LookupRefList =
        malloc(::core::mem::size_of::<LookupRefList>() as usize) as *mut LookupRefList;
    otl_lookup_ref_list_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_copy_replace(
    mut dst: *mut LookupRefList,
    src: LookupRefList,
) {
    otl_lookup_ref_list_dispose(dst);
    otl_lookup_ref_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_copy(
    mut dst: *mut LookupRefList,
    mut src: *const LookupRefList,
) {
    otl_lookup_ref_list_init(dst);
    otl_lookup_ref_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if OTL_I_LOOKUP_REF.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            OTL_I_LOOKUP_REF.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_lookup_ref_list_dispose(mut arr: *mut LookupRefList) {
    if arr.is_null() {
        return;
    }
    if OTL_I_LOOKUP_REF.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh15 = j;
            j = j.wrapping_sub(1);
            if !(fresh15 != 0) {
                break;
            }
            OTL_I_LOOKUP_REF.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_lookup_ref_list_replace(
    mut dst: *mut LookupRefList,
    src: LookupRefList,
) {
    otl_lookup_ref_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LookupRefList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_init_cap_n(mut arr: *mut LookupRefList, mut n: usize) {
    otl_lookup_ref_list_init(arr);
    otl_lookup_ref_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_init_n(mut arr: *mut LookupRefList, mut n: usize) {
    otl_lookup_ref_list_init(arr);
    otl_lookup_ref_list_grow_to_n(arr, n);
    otl_lookup_ref_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_free(mut x: *mut LookupRefList) {
    if x.is_null() {
        return;
    }
    otl_lookup_ref_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_lookup_ref_list_grow_to_n(arr: *mut LookupRefList, target: usize) {
    cvec_grow_to_n(otl_lookup_ref_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn init_feature_ptr(mut feature: *mut FeaturePtr) {
    *feature = __caryll_allocate_clean(
        ::core::mem::size_of::<Feature>() as usize,
        61 as ::core::ffi::c_ulong,
    ) as FeaturePtr;
    OTL_I_LOOKUP_REF_LIST.init.expect("non-null function pointer")(&raw mut (**feature).lookups);
}
#[inline]
unsafe extern "C" fn dispose_feature_ptr(mut feature: *mut FeaturePtr) {
    if (*feature).is_null() {
        return;
    }
    if !(**feature).name.is_null() {
        sdsfree((**feature).name);
    }
    OTL_I_LOOKUP_REF_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (**feature).lookups);
    free(*feature as *mut ::core::ffi::c_void);
    *feature = ::core::ptr::null_mut::<Feature>();
}
#[inline]
unsafe extern "C" fn otl_feature_ptr_dispose(mut x: *mut FeaturePtr) {
    dispose_feature_ptr(x);
}
#[inline]
unsafe extern "C" fn otl_feature_ptr_copy(
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
unsafe extern "C" fn otl_feature_ptr_replace(mut dst: *mut FeaturePtr, src: FeaturePtr) {
    otl_feature_ptr_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeaturePtr>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_feature_ptr_move(
    mut dst: *mut FeaturePtr,
    mut src: *mut FeaturePtr,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeaturePtr>() as usize,
    );
    otl_feature_ptr_init(src);
}
#[inline]
unsafe extern "C" fn otl_feature_ptr_init(mut x: *mut FeaturePtr) {
    init_feature_ptr(x);
}
pub static OTL_I_FEATURE_PTR: FeaturePtrElementInterface = {
    FeaturePtrElementInterface {
        init: Some(otl_feature_ptr_init as unsafe extern "C" fn(*mut FeaturePtr) -> ()),
        copy: Some(
            otl_feature_ptr_copy
                as unsafe extern "C" fn(*mut FeaturePtr, *const FeaturePtr) -> (),
        ),
        move_0: Some(
            otl_feature_ptr_move
                as unsafe extern "C" fn(*mut FeaturePtr, *mut FeaturePtr) -> (),
        ),
        dispose: Some(otl_feature_ptr_dispose as unsafe extern "C" fn(*mut FeaturePtr) -> ()),
        replace: Some(
            otl_feature_ptr_replace
                as unsafe extern "C" fn(*mut FeaturePtr, FeaturePtr) -> (),
        ),
        copy_replace: Some(
            otl_feature_ptr_copy_replace
                as unsafe extern "C" fn(*mut FeaturePtr, FeaturePtr) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_feature_ptr_copy_replace(mut dst: *mut FeaturePtr, src: FeaturePtr) {
    otl_feature_ptr_dispose(dst);
    otl_feature_ptr_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_feature_list_fill(mut arr: *mut FeatureList, mut n: usize) {
    while (*arr).length < n {
        let mut x: FeaturePtr = ::core::ptr::null_mut::<Feature>();
        if OTL_I_FEATURE_PTR.init.is_some() {
            OTL_I_FEATURE_PTR.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<FeaturePtr>() as usize,
            );
        }
        otl_feature_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_feature_list_grow_to(arr: *mut FeatureList, target: usize) {
    cvec_grow_to(otl_feature_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_feature_list_grow_to_n(arr: *mut FeatureList, target: usize) {
    cvec_grow_to_n(otl_feature_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_feature_list_init_n(mut arr: *mut FeatureList, mut n: usize) {
    otl_feature_list_init(arr);
    otl_feature_list_grow_to_n(arr, n);
    otl_feature_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_feature_list_free(mut x: *mut FeatureList) {
    if x.is_null() {
        return;
    }
    otl_feature_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_feature_list_create_n(mut n: usize) -> *mut FeatureList {
    let mut t: *mut FeatureList =
        malloc(::core::mem::size_of::<FeatureList>() as usize) as *mut FeatureList;
    otl_feature_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_feature_list_create() -> *mut FeatureList {
    let mut x: *mut FeatureList =
        malloc(::core::mem::size_of::<FeatureList>() as usize) as *mut FeatureList;
    otl_feature_list_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_feature_list_sort(
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
unsafe extern "C" fn otl_feature_list_push(arr: *mut FeatureList, elem: FeaturePtr) {
    cvec_push(otl_feature_list_as_cvec(arr), elem);
}
#[inline]
unsafe fn otl_feature_list_as_cvec(arr: *mut FeatureList) -> *mut CVecRaw<FeaturePtr> {
    arr as *mut CVecRaw<FeaturePtr>
}
#[inline]
unsafe extern "C" fn otl_feature_list_init(arr: *mut FeatureList) {
    cvec_init(otl_feature_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_feature_list_pop(arr: *mut FeatureList) -> FeaturePtr {
    cvec_pop(otl_feature_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_feature_list_copy_replace(
    mut dst: *mut FeatureList,
    src: FeatureList,
) {
    otl_feature_list_dispose(dst);
    otl_feature_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_feature_list_copy(
    mut dst: *mut FeatureList,
    mut src: *const FeatureList,
) {
    otl_feature_list_init(dst);
    otl_feature_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if OTL_I_FEATURE_PTR.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            OTL_I_FEATURE_PTR.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_feature_list_grow(arr: *mut FeatureList) {
    cvec_grow(otl_feature_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_feature_list_dispose(mut arr: *mut FeatureList) {
    if arr.is_null() {
        return;
    }
    if OTL_I_FEATURE_PTR.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh20 = j;
            j = j.wrapping_sub(1);
            if !(fresh20 != 0) {
                break;
            }
            OTL_I_FEATURE_PTR.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_feature_list_replace(mut dst: *mut FeatureList, src: FeatureList) {
    otl_feature_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeatureList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_feature_list_init_cap_n(mut arr: *mut FeatureList, mut n: usize) {
    otl_feature_list_init(arr);
    otl_feature_list_grow_to_n(arr, n);
}
pub static OTL_I_FEATURE_LIST: FeatureListVectorInterface = {
    FeatureListVectorInterface {
        init: Some(otl_feature_list_init as unsafe extern "C" fn(*mut FeatureList) -> ()),
        copy: Some(
            otl_feature_list_copy
                as unsafe extern "C" fn(*mut FeatureList, *const FeatureList) -> (),
        ),
        move_0: Some(
            otl_feature_list_move
                as unsafe extern "C" fn(*mut FeatureList, *mut FeatureList) -> (),
        ),
        dispose: Some(otl_feature_list_dispose as unsafe extern "C" fn(*mut FeatureList) -> ()),
        replace: Some(
            otl_feature_list_replace
                as unsafe extern "C" fn(*mut FeatureList, FeatureList) -> (),
        ),
        copy_replace: Some(
            otl_feature_list_copy_replace
                as unsafe extern "C" fn(*mut FeatureList, FeatureList) -> (),
        ),
        create: Some(otl_feature_list_create),
        free: Some(otl_feature_list_free as unsafe extern "C" fn(*mut FeatureList) -> ()),
        init_n: Some(
            otl_feature_list_init_n as unsafe extern "C" fn(*mut FeatureList, usize) -> (),
        ),
        init_cap_n: Some(
            otl_feature_list_init_cap_n as unsafe extern "C" fn(*mut FeatureList, usize) -> (),
        ),
        create_n: Some(
            otl_feature_list_create_n as unsafe extern "C" fn(usize) -> *mut FeatureList,
        ),
        fill: Some(
            otl_feature_list_fill as unsafe extern "C" fn(*mut FeatureList, usize) -> (),
        ),
        clear: Some(otl_feature_list_dispose as unsafe extern "C" fn(*mut FeatureList) -> ()),
        push: Some(
            otl_feature_list_push
                as unsafe extern "C" fn(*mut FeatureList, FeaturePtr) -> (),
        ),
        shrink_to_fit: Some(
            otl_feature_list_shrink_to_fit as unsafe extern "C" fn(*mut FeatureList) -> (),
        ),
        pop: Some(
            otl_feature_list_pop as unsafe extern "C" fn(*mut FeatureList) -> FeaturePtr,
        ),
        dispose_item: Some(
            otl_feature_list_dispose_item as unsafe extern "C" fn(*mut FeatureList, usize) -> (),
        ),
        filter_env: Some(
            otl_feature_list_filter_env
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
            otl_feature_list_sort
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
unsafe extern "C" fn otl_feature_list_shrink_to_fit(mut arr: *mut FeatureList) {
    otl_feature_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_feature_list_resize_to(arr: *mut FeatureList, target: usize) {
    cvec_resize_to(otl_feature_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_feature_list_move(dst: *mut FeatureList, src: *mut FeatureList) {
    cvec_move(otl_feature_list_as_cvec(dst), otl_feature_list_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_feature_list_filter_env(
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
            if OTL_I_FEATURE_PTR.dispose.is_some() {
                OTL_I_FEATURE_PTR.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_feature_list_dispose_item(mut arr: *mut FeatureList, mut n: usize) {
    if OTL_I_FEATURE_PTR.dispose.is_some() {
        OTL_I_FEATURE_PTR.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut FeaturePtr,
        );
    } else {
    };
}
pub static OTL_I_FEATURE_REF: FeatureRefElementInterface = {
    FeatureRefElementInterface {
        init: Some(otl_feature_ref_init as unsafe extern "C" fn(*mut FeatureRef) -> ()),
        copy: Some(
            otl_feature_ref_copy
                as unsafe extern "C" fn(*mut FeatureRef, *const FeatureRef) -> (),
        ),
        move_0: Some(
            otl_feature_ref_move
                as unsafe extern "C" fn(*mut FeatureRef, *mut FeatureRef) -> (),
        ),
        dispose: Some(otl_feature_ref_dispose as unsafe extern "C" fn(*mut FeatureRef) -> ()),
        replace: Some(
            otl_feature_ref_replace
                as unsafe extern "C" fn(*mut FeatureRef, FeatureRef) -> (),
        ),
        copy_replace: Some(
            otl_feature_ref_copy_replace
                as unsafe extern "C" fn(*mut FeatureRef, FeatureRef) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otl_feature_ref_copy_replace(mut dst: *mut FeatureRef, src: FeatureRef) {
    otl_feature_ref_dispose(dst);
    otl_feature_ref_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_feature_ref_copy(
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
unsafe extern "C" fn otl_feature_ref_dispose(mut _x: *mut FeatureRef) {}
#[inline]
unsafe extern "C" fn otl_feature_ref_replace(mut dst: *mut FeatureRef, src: FeatureRef) {
    otl_feature_ref_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeatureRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_feature_ref_move(
    mut dst: *mut FeatureRef,
    mut src: *mut FeatureRef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeatureRef>() as usize,
    );
    otl_feature_ref_init(src);
}
#[inline]
unsafe extern "C" fn otl_feature_ref_init(mut x: *mut FeatureRef) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<FeatureRef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_fill(mut arr: *mut FeatureRefList, mut n: usize) {
    while (*arr).length < n {
        let mut x: FeatureRef = ::core::ptr::null::<Feature>();
        if OTL_I_FEATURE_REF.init.is_some() {
            OTL_I_FEATURE_REF.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<FeatureRef>() as usize,
            );
        }
        otl_feature_ref_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_grow_to(arr: *mut FeatureRefList, target: usize) {
    cvec_grow_to(otl_feature_ref_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_pop(arr: *mut FeatureRefList) -> FeatureRef {
    cvec_pop(otl_feature_ref_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_copy_replace(
    mut dst: *mut FeatureRefList,
    src: FeatureRefList,
) {
    otl_feature_ref_list_dispose(dst);
    otl_feature_ref_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_copy(
    mut dst: *mut FeatureRefList,
    mut src: *const FeatureRefList,
) {
    otl_feature_ref_list_init(dst);
    otl_feature_ref_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if OTL_I_FEATURE_REF.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            OTL_I_FEATURE_REF.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_feature_ref_list_dispose(mut arr: *mut FeatureRefList) {
    if arr.is_null() {
        return;
    }
    if OTL_I_FEATURE_REF.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh25 = j;
            j = j.wrapping_sub(1);
            if !(fresh25 != 0) {
                break;
            }
            OTL_I_FEATURE_REF.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_feature_ref_list_replace(
    mut dst: *mut FeatureRefList,
    src: FeatureRefList,
) {
    otl_feature_ref_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FeatureRefList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_init_cap_n(mut arr: *mut FeatureRefList, mut n: usize) {
    otl_feature_ref_list_init(arr);
    otl_feature_ref_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_grow_to_n(arr: *mut FeatureRefList, target: usize) {
    cvec_grow_to_n(otl_feature_ref_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_init_n(mut arr: *mut FeatureRefList, mut n: usize) {
    otl_feature_ref_list_init(arr);
    otl_feature_ref_list_grow_to_n(arr, n);
    otl_feature_ref_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_free(mut x: *mut FeatureRefList) {
    if x.is_null() {
        return;
    }
    otl_feature_ref_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_create_n(mut n: usize) -> *mut FeatureRefList {
    let mut t: *mut FeatureRefList =
        malloc(::core::mem::size_of::<FeatureRefList>() as usize) as *mut FeatureRefList;
    otl_feature_ref_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_create() -> *mut FeatureRefList {
    let mut x: *mut FeatureRefList =
        malloc(::core::mem::size_of::<FeatureRefList>() as usize) as *mut FeatureRefList;
    otl_feature_ref_list_init(x);
    return x;
}
pub static OTL_I_FEATURE_REF_LIST: FeatureRefListVectorInterface = {
    FeatureRefListVectorInterface {
        init: Some(otl_feature_ref_list_init as unsafe extern "C" fn(*mut FeatureRefList) -> ()),
        copy: Some(
            otl_feature_ref_list_copy
                as unsafe extern "C" fn(*mut FeatureRefList, *const FeatureRefList) -> (),
        ),
        move_0: Some(
            otl_feature_ref_list_move
                as unsafe extern "C" fn(*mut FeatureRefList, *mut FeatureRefList) -> (),
        ),
        dispose: Some(
            otl_feature_ref_list_dispose as unsafe extern "C" fn(*mut FeatureRefList) -> (),
        ),
        replace: Some(
            otl_feature_ref_list_replace
                as unsafe extern "C" fn(*mut FeatureRefList, FeatureRefList) -> (),
        ),
        copy_replace: Some(
            otl_feature_ref_list_copy_replace
                as unsafe extern "C" fn(*mut FeatureRefList, FeatureRefList) -> (),
        ),
        create: Some(otl_feature_ref_list_create),
        free: Some(otl_feature_ref_list_free as unsafe extern "C" fn(*mut FeatureRefList) -> ()),
        init_n: Some(
            otl_feature_ref_list_init_n as unsafe extern "C" fn(*mut FeatureRefList, usize) -> (),
        ),
        init_cap_n: Some(
            otl_feature_ref_list_init_cap_n
                as unsafe extern "C" fn(*mut FeatureRefList, usize) -> (),
        ),
        create_n: Some(
            otl_feature_ref_list_create_n as unsafe extern "C" fn(usize) -> *mut FeatureRefList,
        ),
        fill: Some(
            otl_feature_ref_list_fill as unsafe extern "C" fn(*mut FeatureRefList, usize) -> (),
        ),
        clear: Some(
            otl_feature_ref_list_dispose as unsafe extern "C" fn(*mut FeatureRefList) -> (),
        ),
        push: Some(
            otl_feature_ref_list_push
                as unsafe extern "C" fn(*mut FeatureRefList, FeatureRef) -> (),
        ),
        shrink_to_fit: Some(
            otl_feature_ref_list_shrink_to_fit as unsafe extern "C" fn(*mut FeatureRefList) -> (),
        ),
        pop: Some(
            otl_feature_ref_list_pop
                as unsafe extern "C" fn(*mut FeatureRefList) -> FeatureRef,
        ),
        dispose_item: Some(
            otl_feature_ref_list_dispose_item
                as unsafe extern "C" fn(*mut FeatureRefList, usize) -> (),
        ),
        filter_env: Some(
            otl_feature_ref_list_filter_env
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
            otl_feature_ref_list_sort
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
unsafe extern "C" fn otl_feature_ref_list_shrink_to_fit(mut arr: *mut FeatureRefList) {
    otl_feature_ref_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_resize_to(arr: *mut FeatureRefList, target: usize) {
    cvec_resize_to(otl_feature_ref_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_move(dst: *mut FeatureRefList, src: *mut FeatureRefList) {
    cvec_move(otl_feature_ref_list_as_cvec(dst), otl_feature_ref_list_as_cvec(src));
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_filter_env(
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
            if OTL_I_FEATURE_REF.dispose.is_some() {
                OTL_I_FEATURE_REF.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_feature_ref_list_sort(
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
unsafe extern "C" fn otl_feature_ref_list_push(arr: *mut FeatureRefList, elem: FeatureRef) {
    cvec_push(otl_feature_ref_list_as_cvec(arr), elem);
}
#[inline]
unsafe fn otl_feature_ref_list_as_cvec(arr: *mut FeatureRefList) -> *mut CVecRaw<FeatureRef> {
    arr as *mut CVecRaw<FeatureRef>
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_init(arr: *mut FeatureRefList) {
    cvec_init(otl_feature_ref_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_grow(arr: *mut FeatureRefList) {
    cvec_grow(otl_feature_ref_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_feature_ref_list_dispose_item(
    mut arr: *mut FeatureRefList,
    mut n: usize,
) {
    if OTL_I_FEATURE_REF.dispose.is_some() {
        OTL_I_FEATURE_REF.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut FeatureRef,
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn init_language_ptr(mut language: *mut LanguageSystemPtr) {
    *language = __caryll_allocate_clean(
        ::core::mem::size_of::<LanguageSystem>() as usize,
        77 as ::core::ffi::c_ulong,
    ) as LanguageSystemPtr;
    OTL_I_FEATURE_REF_LIST.init.expect("non-null function pointer")(&raw mut (**language).features);
}
#[inline]
unsafe extern "C" fn dispose_language_ptr(mut language: *mut LanguageSystemPtr) {
    if (*language).is_null() {
        return;
    }
    if !(**language).name.is_null() {
        sdsfree((**language).name);
    }
    OTL_I_FEATURE_REF_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (**language).features);
    free(*language as *mut ::core::ffi::c_void);
    *language = ::core::ptr::null_mut::<LanguageSystem>();
}
pub static OTL_I_LANGUAGE_SYSTEM: LanguageSystemPtrElementInterface = {
    LanguageSystemPtrElementInterface {
        init: Some(init_language_ptr as unsafe extern "C" fn(*mut LanguageSystemPtr) -> ()),
        copy: None,
        move_0: None,
        dispose: Some(dispose_language_ptr as unsafe extern "C" fn(*mut LanguageSystemPtr) -> ()),
        replace: None,
        copy_replace: None,
    }
};
#[inline]
unsafe extern "C" fn otl_lang_system_list_filter_env(
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
            if OTL_I_LANGUAGE_SYSTEM.dispose.is_some() {
                OTL_I_LANGUAGE_SYSTEM
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
unsafe fn otl_lang_system_list_as_cvec(arr: *mut LangSystemList) -> *mut CVecRaw<LanguageSystemPtr> {
    arr as *mut CVecRaw<LanguageSystemPtr>
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_init(arr: *mut LangSystemList) {
    cvec_init(otl_lang_system_list_as_cvec(arr));
}
pub static OTL_I_LANG_SYSTEM_LIST: LangSystemListVectorInterface = {
    LangSystemListVectorInterface {
        init: Some(otl_lang_system_list_init as unsafe extern "C" fn(*mut LangSystemList) -> ()),
        copy: Some(
            otl_lang_system_list_copy
                as unsafe extern "C" fn(*mut LangSystemList, *const LangSystemList) -> (),
        ),
        move_0: Some(
            otl_lang_system_list_move
                as unsafe extern "C" fn(*mut LangSystemList, *mut LangSystemList) -> (),
        ),
        dispose: Some(
            otl_lang_system_list_dispose as unsafe extern "C" fn(*mut LangSystemList) -> (),
        ),
        replace: Some(
            otl_lang_system_list_replace
                as unsafe extern "C" fn(*mut LangSystemList, LangSystemList) -> (),
        ),
        copy_replace: Some(
            otl_lang_system_list_copy_replace
                as unsafe extern "C" fn(*mut LangSystemList, LangSystemList) -> (),
        ),
        create: Some(otl_lang_system_list_create),
        free: Some(otl_lang_system_list_free as unsafe extern "C" fn(*mut LangSystemList) -> ()),
        init_n: Some(
            otl_lang_system_list_init_n as unsafe extern "C" fn(*mut LangSystemList, usize) -> (),
        ),
        init_cap_n: Some(
            otl_lang_system_list_init_cap_n
                as unsafe extern "C" fn(*mut LangSystemList, usize) -> (),
        ),
        create_n: Some(
            otl_lang_system_list_create_n as unsafe extern "C" fn(usize) -> *mut LangSystemList,
        ),
        fill: Some(
            otl_lang_system_list_fill as unsafe extern "C" fn(*mut LangSystemList, usize) -> (),
        ),
        clear: Some(
            otl_lang_system_list_dispose as unsafe extern "C" fn(*mut LangSystemList) -> (),
        ),
        push: Some(
            otl_lang_system_list_push
                as unsafe extern "C" fn(*mut LangSystemList, LanguageSystemPtr) -> (),
        ),
        shrink_to_fit: Some(
            otl_lang_system_list_shrink_to_fit as unsafe extern "C" fn(*mut LangSystemList) -> (),
        ),
        pop: Some(
            otl_lang_system_list_pop
                as unsafe extern "C" fn(*mut LangSystemList) -> LanguageSystemPtr,
        ),
        dispose_item: Some(
            otl_lang_system_list_dispose_item
                as unsafe extern "C" fn(*mut LangSystemList, usize) -> (),
        ),
        filter_env: Some(
            otl_lang_system_list_filter_env
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
            otl_lang_system_list_sort
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
unsafe extern "C" fn otl_lang_system_list_dispose_item(
    mut arr: *mut LangSystemList,
    mut n: usize,
) {
    if OTL_I_LANGUAGE_SYSTEM.dispose.is_some() {
        OTL_I_LANGUAGE_SYSTEM
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut LanguageSystemPtr
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_sort(
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
unsafe extern "C" fn otl_lang_system_list_fill(mut arr: *mut LangSystemList, mut n: usize) {
    while (*arr).length < n {
        let mut x: LanguageSystemPtr = ::core::ptr::null_mut::<LanguageSystem>();
        if OTL_I_LANGUAGE_SYSTEM.init.is_some() {
            OTL_I_LANGUAGE_SYSTEM.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<LanguageSystemPtr>() as usize,
            );
        }
        otl_lang_system_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_push(arr: *mut LangSystemList, elem: LanguageSystemPtr) {
    cvec_push(otl_lang_system_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_grow(arr: *mut LangSystemList) {
    cvec_grow(otl_lang_system_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_grow_to(arr: *mut LangSystemList, target: usize) {
    cvec_grow_to(otl_lang_system_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_pop(arr: *mut LangSystemList) -> LanguageSystemPtr {
    cvec_pop(otl_lang_system_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_copy_replace(
    mut dst: *mut LangSystemList,
    src: LangSystemList,
) {
    otl_lang_system_list_dispose(dst);
    otl_lang_system_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_copy(
    mut dst: *mut LangSystemList,
    mut src: *const LangSystemList,
) {
    otl_lang_system_list_init(dst);
    otl_lang_system_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if OTL_I_LANGUAGE_SYSTEM.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            OTL_I_LANGUAGE_SYSTEM.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn otl_lang_system_list_dispose(mut arr: *mut LangSystemList) {
    if arr.is_null() {
        return;
    }
    if OTL_I_LANGUAGE_SYSTEM.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh30 = j;
            j = j.wrapping_sub(1);
            if !(fresh30 != 0) {
                break;
            }
            OTL_I_LANGUAGE_SYSTEM
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
unsafe extern "C" fn otl_lang_system_list_replace(
    mut dst: *mut LangSystemList,
    src: LangSystemList,
) {
    otl_lang_system_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LangSystemList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_init_cap_n(mut arr: *mut LangSystemList, mut n: usize) {
    otl_lang_system_list_init(arr);
    otl_lang_system_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_grow_to_n(arr: *mut LangSystemList, target: usize) {
    cvec_grow_to_n(otl_lang_system_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_init_n(mut arr: *mut LangSystemList, mut n: usize) {
    otl_lang_system_list_init(arr);
    otl_lang_system_list_grow_to_n(arr, n);
    otl_lang_system_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_free(mut x: *mut LangSystemList) {
    if x.is_null() {
        return;
    }
    otl_lang_system_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_create_n(mut n: usize) -> *mut LangSystemList {
    let mut t: *mut LangSystemList =
        malloc(::core::mem::size_of::<LangSystemList>() as usize) as *mut LangSystemList;
    otl_lang_system_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_create() -> *mut LangSystemList {
    let mut x: *mut LangSystemList =
        malloc(::core::mem::size_of::<LangSystemList>() as usize) as *mut LangSystemList;
    otl_lang_system_list_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_shrink_to_fit(mut arr: *mut LangSystemList) {
    otl_lang_system_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_resize_to(arr: *mut LangSystemList, target: usize) {
    cvec_resize_to(otl_lang_system_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn otl_lang_system_list_move(dst: *mut LangSystemList, src: *mut LangSystemList) {
    cvec_move(otl_lang_system_list_as_cvec(dst), otl_lang_system_list_as_cvec(src));
}
#[inline]
unsafe extern "C" fn init_otl(mut table: *mut OtlTable) {
    OTL_I_LOOKUP_LIST.init.expect("non-null function pointer")(&raw mut (*table).lookups);
    OTL_I_FEATURE_LIST.init.expect("non-null function pointer")(&raw mut (*table).features);
    OTL_I_LANG_SYSTEM_LIST.init.expect("non-null function pointer")(&raw mut (*table).languages);
}
#[inline]
unsafe extern "C" fn dispose_otl(mut table: *mut OtlTable) {
    OTL_I_LOOKUP_LIST.dispose.expect("non-null function pointer")(&raw mut (*table).lookups);
    OTL_I_FEATURE_LIST.dispose.expect("non-null function pointer")(&raw mut (*table).features);
    OTL_I_LANG_SYSTEM_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*table).languages);
}
#[inline]
unsafe extern "C" fn table_otl_dispose(mut x: *mut OtlTable) {
    dispose_otl(x);
}
#[inline]
unsafe extern "C" fn table_otl_copy_replace(mut dst: *mut OtlTable, src: OtlTable) {
    table_otl_dispose(dst);
    table_otl_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_otl_free(mut x: *mut OtlTable) {
    if x.is_null() {
        return;
    }
    table_otl_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_otl_create() -> *mut OtlTable {
    let mut x: *mut OtlTable =
        malloc(::core::mem::size_of::<OtlTable>() as usize) as *mut OtlTable;
    table_otl_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_otl_init(mut x: *mut OtlTable) {
    init_otl(x);
}
pub static TABLE_I_OTL: OtlTableElementInterface = {
    OtlTableElementInterface {
        init: Some(table_otl_init as unsafe extern "C" fn(*mut OtlTable) -> ()),
        copy: Some(table_otl_copy as unsafe extern "C" fn(*mut OtlTable, *const OtlTable) -> ()),
        move_0: Some(table_otl_move as unsafe extern "C" fn(*mut OtlTable, *mut OtlTable) -> ()),
        dispose: Some(table_otl_dispose as unsafe extern "C" fn(*mut OtlTable) -> ()),
        replace: Some(table_otl_replace as unsafe extern "C" fn(*mut OtlTable, OtlTable) -> ()),
        copy_replace: Some(
            table_otl_copy_replace as unsafe extern "C" fn(*mut OtlTable, OtlTable) -> (),
        ),
        create: Some(table_otl_create),
        free: Some(table_otl_free as unsafe extern "C" fn(*mut OtlTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_otl_copy(mut dst: *mut OtlTable, mut src: *const OtlTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<OtlTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_otl_replace(mut dst: *mut OtlTable, src: OtlTable) {
    table_otl_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<OtlTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_otl_move(mut dst: *mut OtlTable, mut src: *mut OtlTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<OtlTable>() as usize,
    );
    table_otl_init(src);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MarkArrayVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut MarkArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut MarkArray, *const MarkArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut MarkArray, *mut MarkArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut MarkArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut MarkArray, MarkArray) -> ()>,
    pub copy_replace: Option<unsafe extern "C" fn(*mut MarkArray, MarkArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut MarkArray>,
    pub free: Option<unsafe extern "C" fn(*mut MarkArray) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut MarkArray, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut MarkArray, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut MarkArray>,
    pub fill: Option<unsafe extern "C" fn(*mut MarkArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut MarkArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut MarkArray, MarkRecord) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut MarkArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut MarkArray) -> MarkRecord>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut MarkArray, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut BaseArray, BaseArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut BaseArray>,
    pub free: Option<unsafe extern "C" fn(*mut BaseArray) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut BaseArray, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut BaseArray, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut BaseArray>,
    pub fill: Option<unsafe extern "C" fn(*mut BaseArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut BaseArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut BaseArray, BaseRecord) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut BaseArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut BaseArray) -> BaseRecord>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut BaseArray, usize) -> ()>,
    pub filter_env: Option<
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut LigatureArray, LigatureArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LigatureArray>,
    pub free: Option<unsafe extern "C" fn(*mut LigatureArray) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut LigatureArray, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut LigatureArray, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut LigatureArray>,
    pub fill: Option<unsafe extern "C" fn(*mut LigatureArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut LigatureArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut LigatureArray, LigatureBaseRecord) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut LigatureArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut LigatureArray) -> LigatureBaseRecord>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut LigatureArray, usize) -> ()>,
    pub filter_env: Option<
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
    // constant is `OTL_TYPE_GPOS_MARK_TO_BASE` -- so they were copied from the
    // `tableNames` table this replaced and are pinned here rather than derived.
    #[test]
    fn lookup_type_names_are_the_json_strings() {
        for (t, name) in [
            (OTL_TYPE_UNKNOWN, c"unknown"),
            (OTL_TYPE_GSUB_UNKNOWN, c"gsub_unknown"),
            (OTL_TYPE_GSUB_SINGLE, c"gsub_single"),
            (OTL_TYPE_GSUB_MULTIPLE, c"gsub_multiple"),
            (OTL_TYPE_GSUB_ALTERNATE, c"gsub_alternate"),
            (OTL_TYPE_GSUB_LIGATURE, c"gsub_ligature"),
            (OTL_TYPE_GSUB_CONTEXT, c"gsub_context"),
            (OTL_TYPE_GSUB_CHAINING, c"gsub_chaining"),
            (OTL_TYPE_GSUB_EXTEND, c"gsub_extend"),
            (OTL_TYPE_GSUB_REVERSE, c"gsub_reverse"),
            (OTL_TYPE_GPOS_UNKNOWN, c"gpos_unknown"),
            (OTL_TYPE_GPOS_SINGLE, c"gpos_single"),
            (OTL_TYPE_GPOS_PAIR, c"gpos_pair"),
            (OTL_TYPE_GPOS_CURSIVE, c"gpos_cursive"),
            (OTL_TYPE_GPOS_MARK_TO_BASE, c"gpos_mark_to_base"),
            (OTL_TYPE_GPOS_MARK_TO_LIGATURE, c"gpos_mark_to_ligature"),
            (OTL_TYPE_GPOS_MARK_TO_MARK, c"gpos_mark_to_mark"),
            (OTL_TYPE_GPOS_CONTEXT, c"gpos_context"),
            (OTL_TYPE_GPOS_CHAINING, c"gpos_chaining"),
            (OTL_TYPE_GPOS_EXTEND, c"gpos_extend"),
        ] {
            assert_eq!(t.name(), name, "name for {t:?}");
        }
    }

    // The numbering is otfcc's own: the file's format number plus 16 for gsub or
    // 32 for gpos. `file_format` has to undo exactly that, because its result is
    // written straight into the lookup header.
    #[test]
    fn file_format_undoes_the_table_base() {
        assert_eq!(OTL_TYPE_GSUB_SINGLE.file_format(), 1);
        assert_eq!(OTL_TYPE_GSUB_REVERSE.file_format(), 8);
        assert_eq!(OTL_TYPE_GSUB_EXTEND.file_format(), 7);
        assert_eq!(OTL_TYPE_GPOS_SINGLE.file_format(), 1);
        assert_eq!(OTL_TYPE_GPOS_EXTEND.file_format(), 9);
        // The bases themselves are *not* above their own base -- C compares with
        // `>`, not `>=` -- so they carry no format number.
        assert_eq!(OTL_TYPE_UNKNOWN.file_format(), 0);
        assert_eq!(OTL_TYPE_GSUB_UNKNOWN.file_format(), 0);
        // Except `gpos_unknown`, and this one is a quirk kept on purpose: 32 is
        // not above gpos's base but it *is* above gsub's, so C's nested
        // comparisons read it as gsub format 16. Reachable only from a font
        // declaring a gpos lookup of format 0, which no version of the spec has
        // -- but the number would go straight into the lookup header, so it is
        // reproduced rather than tidied.
        assert_eq!(OTL_TYPE_GPOS_UNKNOWN.file_format(), 16);
    }

    // A lookup type comes out of the font as a 16-bit number added to a base,
    // and C keeps whatever that gives -- including values no variant names,
    // which is why this type is not an enum. The raw value is observable: an
    // unnamed lookup is called `lookup_<raw as %04x>_<index>` in the JSON.
    #[test]
    fn from_file_keeps_unnamed_types() {
        assert_eq!(
            LookupType::from_file(OTL_TYPE_GSUB_UNKNOWN, 1),
            OTL_TYPE_GSUB_SINGLE
        );
        assert_eq!(
            LookupType::from_file(OTL_TYPE_GPOS_UNKNOWN, 9),
            OTL_TYPE_GPOS_EXTEND
        );
        // gsub format 9 exists in no version of the spec otfcc knows; it stays
        // 25, gets no subtable, and reaches the output as `lookup_0019_…`.
        let unnamed = LookupType::from_file(OTL_TYPE_GSUB_UNKNOWN, 9);
        assert_eq!(unnamed.raw(), 25);
        assert_eq!(unnamed.name(), c"unknown");
        assert_eq!(LookupType::from_file(OTL_TYPE_GSUB_UNKNOWN, 0xffff).raw(), 65551);
        assert_eq!(::core::mem::size_of::<LookupType>(), 4);
    }
}
