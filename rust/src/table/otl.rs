#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod build;
pub mod classdef;
pub mod constants;
pub mod coverage;
pub mod dump;
pub mod parse;
pub mod read;
pub mod subtables;

use crate::support::handle::{GlyphHandle, LookupHandle};
use crate::table::otl::classdef::ClassDef;
use crate::table::otl::coverage::Coverage;

use crate::support::primitives::{GlyphClass, GlyphId, Pos, TableId};
use crate::table::otl::subtables::gpos_cursive::dispose_gpos_cursive_subtable;
use crate::table::otl::subtables::gpos_single::dispose_gpos_single_subtable;
use crate::table::otl::subtables::gsub_ligature::dispose_gsub_ligature_subtable;
use crate::table::otl::subtables::gsub_multi::dispose_gsub_multi_subtable;
use crate::table::otl::subtables::gsub_single::dispose_gsub_single_subtable;

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
// Never copied or moved by value anywhere in the crate -- every use is
// `*mut Subtable`/`*const Subtable`/`Box<Subtable>`, so `Subtable` itself
// needs neither `Copy` nor `Clone`.
//
// Was a `union` with the discriminant living outside it, in `Lookup.type_0`
// -- every read of a variant was a pointer-cast (`&raw const/mut
// (*subtable).field as *const/mut T`), sound only because a union's fields
// all start at offset 0 and `LookupType` was trusted to say which one was
// live. Two consequences of that shape turned out to be unsound in a way
// nothing in this crate exercised: `dispose_subtable_dependent`'s
// `LookupType`-dispatch free functions and `consolidate.rs`'s
// `SubtableRemover` both `transmute`d a `*mut ConcreteType`-typed function
// pointer to `*mut Subtable` and called it directly on a `*mut Subtable` --
// which only worked because the union had no tag to misinterpret. Neither
// of those sites survived the enum: `dispose_subtable_dependent` (its
// dispatch is now `Drop`, below) and `SubtableRemover`
// (`__declare_otl_consolidation` in `consolidate.rs`) are both gone.
//
// As an enum, the discriminant is self-describing -- `Drop` (below) replaces
// both `LookupType`-keyed free-function tables, and no variant needs
// `ManuallyDrop` any more (that was purely a union restriction: a union
// can't auto-drop a field because it doesn't know which one is live).
pub enum Subtable {
    GsubSingle(GsubSingleSubtable),
    GsubMulti(GsubMultiSubtable),
    GsubLigature(GsubLigatureSubtable),
    Chaining(ChainingSubtable),
    GsubReverse(GsubReverseSubtable),
    GposSingle(GposSingleSubtable),
    GposPair(GposPairSubtable),
    GposCursive(GposCursiveSubtable),
    GposMarkToSingle(GposMarkToSingleSubtable),
    GposMarkToLigature(GposMarkToLigatureSubtable),
    Extend(ExtendSubtable),
}
impl Drop for Subtable {
    fn drop(&mut self) {
        unsafe {
            match self {
                Subtable::GsubSingle(x) => {
                    dispose_gsub_single_subtable(x as *mut GsubSingleSubtable)
                }
                Subtable::GsubMulti(x) => dispose_gsub_multi_subtable(x as *mut GsubMultiSubtable),
                Subtable::GsubLigature(x) => {
                    dispose_gsub_ligature_subtable(x as *mut GsubLigatureSubtable)
                }
                // `ChainingRule`'s and `ChainingRuleSet`'s fields (including
                // `bc`/`ic`/`fc: Option<Box<ClassDef>>`, converted alongside
                // this enum) all self-drop now -- no manual dispose left to
                // call, same reasoning as `GposPair`/`GposMarkToSingle`
                // above.
                Subtable::Chaining(_) => {}
                // `match_0: Vec<Coverage>` and `to: Coverage` both self-drop
                // -- no manual dispose left to call, same reasoning as the
                // `GposMarkTo*` arms above. The not-yet-adopted-into-the-enum
                // intermediate a `*mut GsubReverseSubtable` is between
                // `_create()` and `subtable_from_raw` no longer needs its own
                // `dispose_gsub_reverse` either (Stage 7-2-d): `_create()` now
                // allocates via `Box::into_raw`, so `subtable_gsub_reverse_
                // free`'s `Box::from_raw` runs this same enum-field drop glue
                // directly, and a raw `free()` there would have skipped it.
                Subtable::GsubReverse(_) => {}
                Subtable::GposSingle(x) => {
                    dispose_gpos_single_subtable(x as *mut GposSingleSubtable)
                }
                // `first`/`second: Option<Box<ClassDef>>` and
                // `first_values`/`second_values: Vec<Vec<PositionValue>>`
                // all self-drop now -- no manual dispose left to call, same
                // reasoning as `GposMarkToSingle` above.
                Subtable::GposPair(_) => {}
                Subtable::GposCursive(x) => {
                    dispose_gpos_cursive_subtable(x as *mut GposCursiveSubtable)
                }
                // `mark_array: MarkArray` and `base_array: BaseArray`
                // (`Vec<BaseRecord>`, `BaseRecord.anchors` now a plain
                // `Vec<Anchor>`) both self-drop -- no manual dispose left to
                // call, same reasoning as `Extend` below.
                Subtable::GposMarkToSingle(_) => {}
                // `mark_array: MarkArray` and `lig_array: LigatureArray`
                // (`Vec<LigatureBaseRecord>`, `LigatureBaseRecord.anchors`
                // now a plain `Vec<Vec<Anchor>>`) both self-drop -- no
                // manual dispose left to call, same reasoning as
                // `GposMarkToSingle` above.
                Subtable::GposMarkToLigature(_) => {}
                // `subtable: *mut Subtable`'s ownership is always taken (via
                // `.subtable`) before an `Extend` value is legitimately
                // dropped -- `otl/read.rs`'s extend-expansion resolves every
                // `Extend` placeholder to its nested subtable (or, on a
                // mismatched-type error path, to a scratch `Lookup` that
                // takes over `.subtable` and drops it itself) before the
                // shell holding it is ever freed. Matches the old
                // `dispose_subtable_dependent`'s behavior exactly: `EXTEND`
                // had no arm there either, falling through its `_ => {}`.
                Subtable::Extend(_) => {}
            }
        }
    }
}
/// Adopt a vtable-`create()`d raw pointer into a heap-allocated `Subtable`.
///
/// Every subtable's own `_create()` now allocates with `Box::into_raw(Box::
/// new(..))` instead of `malloc` (Stage 7-2-d), so `raw` -- whether it comes
/// straight back from `_create()` or via one of the `ChainingSubtable`
/// read-helpers that thread the same pointer through in place
/// (`read_contextual_format1`/`2`, `read_chaining_format1`/`2`) -- is always
/// that same Rust allocation. `Box::from_raw` reclaims it directly (no more
/// `ptr::read`-then-`free` shell dance: that was only ever needed to avoid
/// mixing a `malloc`'d shell with a `Box`'s own drop glue), and the moved-out
/// value is wrapped into the specific variant by `wrap` (a tuple-variant
/// constructor, e.g. `Subtable::GsubSingle`) and boxed. This only changes
/// what happens to `_create()`'s result at the point each read/parse
/// function used to just cast it `as *mut Subtable`, which relied on
/// `Subtable` being a union with no discriminant to disturb; that cast is
/// unsound now that it is an enum. Null-safe -- several callers' result can
/// still be null on a read error (`table_length` too short partway through,
/// or a `ChainingSubtable` helper freeing it and returning null), and the
/// old `as *mut Subtable` cast propagated a null exactly the same way.
pub(crate) unsafe fn subtable_from_raw<T>(raw: *mut T, wrap: fn(T) -> Subtable) -> *mut Subtable {
    if raw.is_null() {
        return ::core::ptr::null_mut();
    }
    let value = *Box::from_raw(raw);
    Box::into_raw(Box::new(wrap(value)))
}
/// Adopt an already-heap-allocated `*mut Subtable` (e.g. a
/// `subtable_from_raw`/`Box::into_raw` result, or a `.subtable` field read
/// out of an `ExtendSubtable`) into a `SubtableList` slot. Several read/parse
/// entry points can legitimately return null (unrecognised lookup format,
/// truncated data), and a null in a `SubtableList` slot was always a valid
/// "hole" even before this migration -- `Box::from_raw` on a null pointer
/// would be UB, so this null check is required, not defensive.
pub(crate) unsafe fn subtable_list_slot(raw: SubtablePtr) -> Option<Box<Subtable>> {
    if raw.is_null() {
        None
    } else {
        Some(Box::from_raw(raw))
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtendSubtable {
    pub type_0: LookupType,
    pub subtable: *mut Subtable,
}
// Embedded by value in `Subtable::GposMarkToLigature` -- no `Copy`/`Clone`
// needed once `mark_array`/`lig_array` own `Vec`s.
#[repr(C)]
pub struct GposMarkToLigatureSubtable {
    pub class_count: GlyphClass,
    pub mark_array: MarkArray,
    pub lig_array: LigatureArray,
}
/// Embedded by value in both `GposMarkToSingleSubtable` and
/// `GposMarkToLigatureSubtable`, not a `Subtable` union field itself.
pub type LigatureArray = Vec<LigatureBaseRecord>;
#[derive(Clone)]
#[repr(C)]
pub struct LigatureBaseRecord {
    pub glyph: GlyphHandle,
    pub component_count: GlyphId,
    pub anchors: Vec<Vec<Anchor>>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Anchor {
    pub present: bool,
    pub x: Pos,
    pub y: Pos,
}
/// Embedded by value in both `GposMarkToSingleSubtable` and
/// `GposMarkToLigatureSubtable`, not a `Subtable` union field itself.
pub type MarkArray = Vec<MarkRecord>;
#[derive(Clone)]
#[repr(C)]
pub struct MarkRecord {
    pub glyph: GlyphHandle,
    pub mark_class: GlyphClass,
    pub anchor: Anchor,
}
// Embedded by value in `Subtable::GposMarkToSingle` -- no `Copy`/`Clone`
// needed once `mark_array`/`base_array` own `Vec`s.
#[repr(C)]
pub struct GposMarkToSingleSubtable {
    pub class_count: GlyphClass,
    pub mark_array: MarkArray,
    pub base_array: BaseArray,
}
/// Embedded by value in `GposMarkToSingleSubtable`, not a `Subtable` union
/// field itself.
pub type BaseArray = Vec<BaseRecord>;
#[derive(Clone)]
#[repr(C)]
pub struct BaseRecord {
    pub glyph: GlyphHandle,
    pub anchors: Vec<Anchor>,
}
pub type GposCursiveSubtable = Vec<GposCursiveEntry>;
#[derive(Clone)]
#[repr(C)]
pub struct GposCursiveEntry {
    pub target: GlyphHandle,
    pub enter: Anchor,
    pub exit: Anchor,
}
// `Copy` dropped: `first`/`second`/`first_values`/`second_values` all own
// heap allocations now.
#[derive(Clone)]
#[repr(C)]
pub struct GposPairSubtable {
    pub first: Option<Box<ClassDef>>,
    pub second: Option<Box<ClassDef>>,
    pub first_values: Vec<Vec<PositionValue>>,
    pub second_values: Vec<Vec<PositionValue>>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PositionValue {
    pub dx: Pos,
    pub dy: Pos,
    pub d_width: Pos,
    pub d_height: Pos,
}
pub type GposSingleSubtable = Vec<GposSingleEntry>;
#[derive(Clone)]
#[repr(C)]
pub struct GposSingleEntry {
    pub target: GlyphHandle,
    pub value: PositionValue,
}
// `Copy` dropped: `match_0`/`to` own `Vec`s now.
#[derive(Clone)]
#[repr(C)]
pub struct GsubReverseSubtable {
    pub match_count: TableId,
    pub input_index: TableId,
    pub match_0: Vec<Coverage>,
    pub to: Coverage,
}
// Was a C-shaped `struct { type_0: ChainingType, c2rust_unnamed: union {
// rule: ManuallyDrop<ChainingRule>, c2rust_unnamed: ManuallyDrop<
// ChainingRuleSet> } }` -- the same "tag fully determines the live union
// arm" shape already converted for `CffEncoding`/`CffCharset`/`CffFdSelect`/
// `Subtable` itself, just one level deeper (the outer `Subtable` enum's own
// conversion left this nested union in place). `Poly` and `Classified` both
// carry a `ChainingRuleSet` -- they shared the same union arm before, and
// still share the same payload type now; only their outer discriminant
// differs, which build.rs's `type_0 == ChainingType::Classified` checks
// still need to distinguish (see `chaining_is_classified`, common.rs).
// `ChainingType` itself is gone -- the enum's own discriminant replaces it,
// and nothing else in the crate read a bare `ChainingType` value.
//
// Derives neither `Copy` nor `Clone`: `ChainingRuleSet.bc`/`.ic`/`.fc` are
// `Option<Box<ClassDef>>` (converted alongside this enum, matching
// `GposPairSubtable.first`/`.second`), so an automatic bitwise `Clone`
// would double-free. Nothing calls `.clone()` on this type -- the vtable's
// `.copy` slot (`subtable_chaining_copy`) is confirmed dead (never called
// outside its own static initializer) and is now a loud `unreachable!()`
// instead of a `memcpy` that would be unsound over owned `Vec`/`Box` data.
pub enum ChainingSubtable {
    Canonical(ChainingRule),
    Poly(ChainingRuleSet),
    Classified(ChainingRuleSet),
}
/// `rules: *mut *mut ChainingRule` (the `Poly`/`Classified` shape) becomes
/// `Vec<Option<Box<ChainingRule>>>` -- joining the `LangSystemList`/
/// `FeatureList`/`LookupList`/`GlyfTable` "owned pointer array" group.
/// `Option` (not plain `Box`) because `general_read_contextual_rule`/
/// `general_read_chaining_rule` can fail on truncated/malformed font data
/// and the pre-`Vec` code pushed a null element in that case with no
/// downstream null check -- `None` reproduces that shape exactly, and
/// every consumption site treats a `None` as unreachable-in-practice
/// (`.expect(...)`, which panics instead of the old null-pointer-deref UB
/// if that latent path is ever actually hit). `rules_count` is gone;
/// every read site now uses `rules.len()`.
///
/// `bc`/`ic`/`fc: *mut ClassDef` become `Option<Box<ClassDef>>`, matching
/// `GposPairSubtable.first`/`.second` exactly -- both are populated only
/// by `classifier.rs`'s later classification pass (never by the raw binary
/// read, which always leaves a `Poly` ruleset's class defs as `None`/null),
/// consumed the same way (`.as_deref()`/`.as_deref_mut()` at the `OTL_I_
/// CLASS_DEF.build`/`.parse` call sites), and now self-drop with the rest
/// of the struct.
#[derive(Default)]
pub struct ChainingRuleSet {
    pub rules: Vec<Option<Box<ChainingRule>>>,
    pub bc: Option<Box<ClassDef>>,
    pub ic: Option<Box<ClassDef>>,
    pub fc: Option<Box<ClassDef>>,
}
/// Replaces the calloc'd raw array `apply: *mut ChainLookupApplication` +
/// `apply_count: TableId` -- the last remaining "leaf type owns a `Handle`
/// but its container isn't `Vec`-backed yet" gap in this crate. No
/// `apply_count` field survives: every read site now uses `apply.len()`.
///
/// `match_0` (`*mut *mut Coverage` -> `Vec<Coverage>`) was the last field
/// needing a custom teardown, so no manual `Drop` impl remains: both fields
/// now self-drop, and the compiler-generated glue tears down a
/// `ChainingRule` correctly whether it's reached via `.rules: Vec<Option<
/// Box<ChainingRule>>>` or via the `ChainingSubtable::Canonical` variant
/// directly (an ordinary enum payload, no longer a `ManuallyDrop` union
/// field needing a separate explicit drop step).
#[derive(Default)]
#[repr(C)]
pub struct ChainingRule {
    pub match_count: TableId,
    pub input_begins: TableId,
    pub input_ends: TableId,
    pub match_0: Vec<Coverage>,
    pub apply: Vec<ChainLookupApplication>,
}
/// `lookup: LookupHandle` (= `Handle`) already has a real `Drop`/`Clone`
/// impl (the Handle pilot), so `Vec<ChainLookupApplication>`'s own drop
/// glue disposes every element correctly with no extra `Drop` impl here.
#[derive(Clone)]
#[repr(C)]
pub struct ChainLookupApplication {
    pub index: TableId,
    pub lookup: LookupHandle,
}
pub type GsubLigatureSubtable = Vec<GsubLigatureEntry>;
#[derive(Clone)]
#[repr(C)]
pub struct GsubLigatureEntry {
    pub from: Coverage,
    pub to: GlyphHandle,
}
pub type GsubMultiSubtable = Vec<GsubMultiEntry>;
#[derive(Clone)]
#[repr(C)]
pub struct GsubMultiEntry {
    pub from: GlyphHandle,
    pub to: Coverage,
}
pub type GsubSingleSubtable = Vec<GsubSingleEntry>;
#[derive(Clone)]
#[repr(C)]
pub struct GsubSingleEntry {
    pub from: GlyphHandle,
    pub to: GlyphHandle,
}
// `subtables: SubtableList`(`Vec<Option<Box<Subtable>>>`)を値で持つため
// `Copy` を落とす。常に `*mut`/`*const` 経由でしか触られない（値渡し・値
// コピーの箇所は無い）。
#[repr(C)]
pub struct Lookup {
    pub name: Vec<u8>,
    pub type_0: LookupType,
    pub _offset: u32,
    pub flags: u16,
    pub subtables: SubtableList,
}
// `Lookup` needed no `Drop` impl of its own even before this: once
// `SubtableList` became `Vec<Option<Box<Subtable>>>`, both fields it owns
// (`name: Vec<u8>`, `subtables`) tear down through ordinary compiler-
// generated field-by-field drop glue, recursively -- `Subtable`'s own
// `Drop` (above) runs for every `Some` element, `None` holes cost nothing.
// The custom impl this comment used to describe called
// `otl_subtable_list_dispose_dependent`, which existed only because
// `SubtableList`'s elements were raw `*mut Subtable` -- deleted along with
// that function once `Box` made the ownership self-describing.
pub type SubtablePtr = *mut Subtable;
// 所有する `Box` 配列。各要素は `None` にもなり得る（consolidate 中の一時的な
// 「取り除かれた」穴、または extend 展開の型不一致エラー経路で残る穴）。
// `build`/`dump`/`stat` など、読み取り時点で穴が無いと分かっている箇所は
// `subtable_at`（下記）で `.expect()` して `SubtablePtr` に戻す。
pub type SubtableList = Vec<Option<Box<Subtable>>>;
/// Read a `SubtableList` element as a raw pointer, panicking if the slot is
/// empty. Every caller of this helper already assumed a slot could not be
/// empty at the point it reads one (`build.rs`/`dump.rs`/`stat.rs`/the
/// chaining classifier all read post-consolidation lists with no null
/// check) -- before `Box` made a hole `None` instead of a dangling
/// `*mut Subtable`, that assumption being wrong meant a silent
/// out-of-bounds-shaped dereference. Now it is a clean panic.
pub(crate) unsafe fn subtable_at(list: &SubtableList, idx: usize) -> SubtablePtr {
    list[idx]
        .as_deref()
        .expect("subtable slot should not be empty at this point") as *const Subtable
        as *mut Subtable
}
pub type LookupPtr = *mut Lookup;
// Stage 6-4, third of the group -- see `LangSystemList`/`FeatureList` for
// the shape. `Lookup`'s own `Drop` (above) now does the type-dispatched
// `SubtableList` teardown `SubtableList` itself still can't do on its own.
pub type LookupList = Vec<Box<Lookup>>;
pub type LookupRef = *const Lookup;
// 所有しない参照配列（`LookupList` の要素を指すだけ）。分類その3。
pub type LookupRefList = Vec<LookupRef>;
// `lookups: LookupRefList`(`Vec<LookupRef>`)を値で持つため `Copy` を落とす。
// `Lookup` と同じく常に `*mut`/`*const` 経由。
#[repr(C)]
pub struct Feature {
    pub name: Vec<u8>,
    pub lookups: LookupRefList,
}
/// `lookups: LookupRefList` (`Vec<LookupRef>`) needs no help -- it holds
/// only *borrowed* `*const Lookup`s into `OtlTable.lookups`, so its own drop
/// glue is enough. `name` (a `Vec<u8>` since the `sds` sweep reached this
/// field) now also tears down for free, so `Feature` needs no manual `Drop`
/// impl at all anymore.
pub type FeaturePtr = *mut Feature;
// Stage 6-4, second of the "owned pointer array" group -- see
// `LangSystemList`/`new_language` for the shape and rationale.
pub type FeatureList = Vec<Box<Feature>>;
pub type FeatureRef = *const Feature;
// 所有しない参照配列（`FeatureList` の要素を指すだけ）。
pub type FeatureRefList = Vec<FeatureRef>;
// `required_feature`はポインタなので無関係、`features: FeatureRefList`
// (`Vec<FeatureRef>`)を値で持つため `Copy` を落とす。
#[repr(C)]
pub struct LanguageSystem {
    pub name: Vec<u8>,
    pub required_feature: FeatureRef,
    pub features: FeatureRefList,
}
/// `required_feature` and `features` both hold *borrowed* `*const
/// Feature`s into `OtlTable`'s own `features` list, so nothing there needs
/// freeing -- `features`'s backing `Vec` drops itself, and `name` (a
/// `Vec<u8>` since the `sds` sweep reached this field) now also tears down
/// for free, so `LanguageSystem` needs no manual `Drop` impl at all
/// anymore.
// Stage 6-4 pilot for the "owned pointer array" shape (plan classification
// その3): the elements are `Box`es now, not raw `*mut`, so the `Vec`'s own
// drop glue frees every element -- see rust/README.md.
pub type LangSystemList = Vec<Box<LanguageSystem>>;
// 3つとも値でVecを持つため `Copy` を落とす。`Font.gsub`/`Font.gpos` は
// `*mut OtlTable` フィールドで、crate全体を通じて常にポインタ経由。
#[repr(C)]
pub struct OtlTable {
    pub lookups: LookupList,
    pub features: FeatureList,
    pub languages: LangSystemList,
}
// `dispose_subtable_dependent`/`otl_subtable_list_dispose_dependent` (a
// 13-arm `LookupType`-keyed free-function dispatch, then later a `Box`-
// reclaiming loop) are gone: with `SubtableList` now `Vec<Option<Box
// <Subtable>>>`, disposing a list is exactly what dropping it already does
// -- `Subtable`'s own `Drop` runs for every `Some` element -- so every
// former call site (`Lookup`'s own now-deleted `Drop` impl,
// `otf_reader/unconsolidate.rs`, `table/otl/read.rs`) either needed no
// replacement at all or shrank to a plain assignment that lets the old
// value drop itself.
// Only ever called on a `Lookup` that hasn't been pushed into a `LookupList`
// yet -- the not-yet-owned scratch/rejection cases in `table/otl/{read,
// parse}.rs`. Reclaims the `Box` `new_lookup`/`Box::into_raw` produced and
// drops it, which now does the subtable teardown and `name` free that this
// function's body used to spell out directly.
pub unsafe fn otfcc_delete_lookup(lookup: *mut Lookup) {
    if lookup.is_null() {
        return;
    }
    drop(Box::from_raw(lookup));
}
/// Same shape as `new_language`/`new_feature`: `Box` is the allocation, the
/// struct literal is the zero-init the old `__caryll_allocate_clean`
/// provided.
#[inline]
pub(crate) fn new_lookup() -> Box<Lookup> {
    Box::new(Lookup {
        name: Vec::new(),
        type_0: OTL_TYPE_UNKNOWN,
        _offset: 0,
        flags: 0,
        subtables: Vec::new(),
    })
}
// `LookupPtr`単体の`.copy`（`otl_lookup_ptr_copy`、生ポインタのmemcpy）は
// `LookupList`自体の`.copy`（テーブル全体クローン、死んでいる）からしか
// 呼ばれておらず削除。
// テーブル全体の `.copy`（`otl_lookup_list_copy`、生存していた `LookupPtr`
// 単体copyと同じく死んでいる）は削除。
//
// `otl_lookup_list_dispose`（旧`table_otl_free`専用の全ドロップヘルパ）も
// 削除: `Font.gsub`/`Font.gpos`のBox化で`table_otl_free`自体が死んだため、
// この関数を呼ぶ場所が無くなった。`Option<Box<OtlTable>>`が`None`になる
// （または単に破棄される）だけで`LookupList`（`Vec<Box<Lookup>>`)は
// 自動的にフルドロップされる——`Lookup::drop`がtype-dispatchedな
// `SubtableList`の破棄と`name`の解放をやる。
// 元の「スワップして末尾を切り詰め」ループを`Vec::retain`に素直に置き換え。
pub(crate) unsafe fn otl_lookup_list_filter_env(
    arr: *mut LookupList,
    fn_0: Option<unsafe extern "C" fn(*const Lookup, *mut ::core::ffi::c_void) -> bool>,
    env: *mut ::core::ffi::c_void,
) {
    (*arr).retain(|item| {
        if fn_0.expect("non-null function pointer")(&raw const **item, env) {
            true
        } else {
            // Rejected: `retain` drops `*item` (a `Box<Lookup>`) itself,
            // running the same teardown `dispose_lookup_ptr` used to do
            // manually -- no explicit call needed.
            false
        }
    });
}
// `LookupRef`単体の要素インターフェース(`otl_lookup_ref_init`/`_copy`/
// `_dispose`)は`LookupRefList`自体の死んだ`.copy`スロットからしか呼ばれて
// おらず削除——`.dispose`(`otl_lookup_ref_dispose`)も含め、`LookupRef`は
// 所有物を持たない（`LookupList`が指し先の`Lookup`を所有する）ため、
// disposeは何もしない。`Vec<LookupRef>`自身の`Drop`だけで十分。
// `LookupRefList`は所有物を持たない要素の配列。disposeはバッキング配列を
// 解放するだけ（要素そのものへの処理は不要）。`.copy`（テーブル全体クローン）
// は死んでいたため削除。
pub(crate) unsafe fn otl_lookup_ref_list_dispose(arr: *mut LookupRefList) {
    if arr.is_null() {
        return;
    }
    *arr = Vec::new();
}
// 元のスワップ&切り詰めループを`Vec::retain`に。要素のdisposeは無いので
// 述語の結果をそのまま`retain`の判定に使うだけで済む。
pub(crate) unsafe fn otl_lookup_ref_list_filter_env(
    arr: *mut LookupRefList,
    fn_0: Option<unsafe extern "C" fn(*const LookupRef, *mut ::core::ffi::c_void) -> bool>,
    env: *mut ::core::ffi::c_void,
) {
    (*arr).retain(|&item| fn_0.expect("non-null function pointer")(&item as *const LookupRef, env));
}
// `.replace`の唯一の呼び出し箇所(`table/otl/parse.rs`)は毎回、直前に
// `new_feature`で作った空のdestに対して呼ばれる——単純な move-assign
// で置き換え可能（旧`dispose`+`memcpy`と等価、Rustの代入が古い値を
// 正しくドロップする）。
pub(crate) unsafe fn otl_lookup_ref_list_replace(dst: *mut LookupRefList, src: LookupRefList) {
    *dst = src;
}
/// Same shape as `new_language`: `Box` is the allocation, the struct
/// literal is the zero-init the old `__caryll_allocate_clean` provided.
#[inline]
pub(crate) fn new_feature() -> Box<Feature> {
    Box::new(Feature {
        name: Vec::new(),
        lookups: Vec::new(),
    })
}
// `FeaturePtr`単体の`.copy`(生ポインタmemcpy)は`FeatureList`の死んだ
// `.copy`からしか呼ばれておらず削除。
// テーブル全体の`.copy`（死んでいる）は削除。`otl_feature_list_dispose`
// （旧`table_otl_free`専用ヘルパ）も同じ理由で削除——`LookupList`と同じく
// `FeatureList`（`Vec<Box<Feature>>`）は`OtlTable`ごと破棄されれば
// 自動的にフルドロップされる。
pub(crate) unsafe fn otl_feature_list_filter_env(
    arr: *mut FeatureList,
    fn_0: Option<unsafe extern "C" fn(*const Feature, *mut ::core::ffi::c_void) -> bool>,
    env: *mut ::core::ffi::c_void,
) {
    (*arr).retain(|item| {
        if fn_0.expect("non-null function pointer")(&raw const **item, env) {
            true
        } else {
            // Rejected: `retain` drops `*item` itself (a `Box<Feature>`),
            // which frees `name` -- no explicit dispose call needed.
            false
        }
    });
}
// `FeatureRef`単体の要素インターフェースは`FeatureRefList`の死んだ`.copy`
// からしか呼ばれておらず削除。`FeatureRef`は所有物を持たない
// （`FeatureList`が指し先の`Feature`を所有する）。
// `.replace`の唯一の呼び出し箇所(`table/otl/parse.rs`)は`new_language`
// で作った空のdestに対して呼ばれる——move-assignで置き換え可能。
pub(crate) unsafe fn otl_feature_ref_list_replace(dst: *mut FeatureRefList, src: FeatureRefList) {
    *dst = src;
}
// `LookupRefList`と同じく所有物を持たない要素の配列。
pub(crate) unsafe fn otl_feature_ref_list_dispose(arr: *mut FeatureRefList) {
    if arr.is_null() {
        return;
    }
    *arr = Vec::new();
}
pub(crate) unsafe fn otl_feature_ref_list_filter_env(
    arr: *mut FeatureRefList,
    fn_0: Option<unsafe extern "C" fn(*const FeatureRef, *mut ::core::ffi::c_void) -> bool>,
    env: *mut ::core::ffi::c_void,
) {
    (*arr)
        .retain(|&item| fn_0.expect("non-null function pointer")(&item as *const FeatureRef, env));
}
/// Replaces the old `__caryll_allocate_clean`-into-a-`*mut`-out-parameter
/// constructor: `Box` is the allocation, and the struct literal is the
/// zero-initialization the `calloc` used to provide.
#[inline]
pub(crate) fn new_language() -> Box<LanguageSystem> {
    Box::new(LanguageSystem {
        name: Vec::new(),
        required_feature: ::core::ptr::null::<Feature>(),
        features: Vec::new(),
    })
}
// テーブル全体の`.copy`（死んでいる）は削除。`otl_lang_system_list_dispose`
// （旧`table_otl_free`専用ヘルパ）も同じ理由で削除。このコンテナだけ
// `.filter_env`スロットが元から無い——言語システム自体は間引かれず、
// `.features`(FeatureRefList)だけが間引かれる。
//
// `Font.gsub`/`Font.gpos`が`Option<Box<OtlTable>>`になったので
// `table_otl_free`自体が不要になった（`Option`の破棄／再代入で
// `LookupList`/`FeatureList`/`LangSystemList`が自動的にフルドロップされる）。
// `table_otl_create`/`init_otl` deleted, not converted: their sole caller
// was `create_font_table`'s `create_table` vtable slot, and grepping
// every `FontElementInterface` field found `.create_table` itself is
// never read anywhere in the crate -- `create_font_table` and its other
// callee `table_name_create` (`table/name.rs`) are dead for the same
// reason, deleted alongside this.
// テーブル全体の `.copy`（`table_otl_copy`、生ポインタのmemcpy）は
// crate全体で一度も呼ばれておらず削除——Vec所有下でのmemcpyは
// 3つの内側リストすべての二重解放になるため、`.clone()`への移植も不要。

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
        assert_eq!(
            LookupType::from_file(OTL_TYPE_GSUB_UNKNOWN, 0xffff).raw(),
            65551
        );
        assert_eq!(::core::mem::size_of::<LookupType>(), 4);
    }
}
