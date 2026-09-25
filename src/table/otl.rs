#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see RUST_MIGRATION.md
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
    pub const fn name(self) -> &'static str {
        match self.0 {
            17 => "gsub_single",
            18 => "gsub_multiple",
            19 => "gsub_alternate",
            20 => "gsub_ligature",
            21 => "gsub_context",
            22 => "gsub_chaining",
            23 => "gsub_extend",
            24 => "gsub_reverse",
            16 => "gsub_unknown",
            33 => "gpos_single",
            34 => "gpos_pair",
            35 => "gpos_cursive",
            36 => "gpos_mark_to_base",
            37 => "gpos_mark_to_ligature",
            38 => "gpos_mark_to_mark",
            39 => "gpos_context",
            40 => "gpos_chaining",
            41 => "gpos_extend",
            32 => "gpos_unknown",
            _ => "unknown",
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
#[derive(Debug)]
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
        match self {
            Subtable::GsubSingle(x) => dispose_gsub_single_subtable(x),
            Subtable::GsubMulti(x) => dispose_gsub_multi_subtable(x),
            Subtable::GsubLigature(x) => dispose_gsub_ligature_subtable(x),
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
            Subtable::GposSingle(x) => dispose_gpos_single_subtable(x),
            // `first`/`second: Option<Box<ClassDef>>` and
            // `first_values`/`second_values: Vec<Vec<PositionValue>>`
            // all self-drop now -- no manual dispose left to call, same
            // reasoning as `GposMarkToSingle` above.
            Subtable::GposPair(_) => {}
            Subtable::GposCursive(x) => dispose_gpos_cursive_subtable(x),
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
            // `subtable: Option<Box<Subtable>>` (a real owning field, not
            // a raw pointer) self-drops -- no manual dispose needed, and
            // unlike the raw-pointer shape this replaced, forgetting to
            // `.take()` it before an `Extend` value drops is no longer a
            // leak: Rust's own per-field drop glue frees whatever is
            // still `Some` here automatically. `otl/read.rs`'s
            // extend-expansion usually does take it first (to resolve
            // every `Extend` placeholder to its nested subtable, or, on
            // a mismatched-type error path, to a scratch `Lookup` that
            // takes over `.subtable` and drops it itself), but nothing
            // relies on that happening for correctness any more.
            Subtable::Extend(_) => {}
        }
    }
}
#[derive(Debug)]
pub struct ExtendSubtable {
    pub type_0: LookupType,
    pub subtable: Option<Box<Subtable>>,
}
// Embedded by value in `Subtable::GposMarkToLigature` -- no `Copy`/`Clone`
// needed once `mark_array`/`lig_array` own `Vec`s.
#[derive(Debug)]
pub struct GposMarkToLigatureSubtable {
    pub class_count: GlyphClass,
    pub mark_array: MarkArray,
    pub lig_array: LigatureArray,
}
/// Embedded by value in both `GposMarkToSingleSubtable` and
/// `GposMarkToLigatureSubtable`, not a `Subtable` union field itself.
pub type LigatureArray = Vec<LigatureBaseRecord>;
#[derive(Clone, Debug)]
pub struct LigatureBaseRecord {
    pub glyph: GlyphHandle,
    pub component_count: GlyphId,
    pub anchors: Vec<Vec<Anchor>>,
}
#[derive(Copy, Clone, Debug)]
pub struct Anchor {
    pub present: bool,
    pub x: Pos,
    pub y: Pos,
}
/// Embedded by value in both `GposMarkToSingleSubtable` and
/// `GposMarkToLigatureSubtable`, not a `Subtable` union field itself.
pub type MarkArray = Vec<MarkRecord>;
#[derive(Clone, Debug)]
pub struct MarkRecord {
    pub glyph: GlyphHandle,
    pub mark_class: GlyphClass,
    pub anchor: Anchor,
}
// Embedded by value in `Subtable::GposMarkToSingle` -- no `Copy`/`Clone`
// needed once `mark_array`/`base_array` own `Vec`s.
#[derive(Debug)]
pub struct GposMarkToSingleSubtable {
    pub class_count: GlyphClass,
    pub mark_array: MarkArray,
    pub base_array: BaseArray,
}
/// Embedded by value in `GposMarkToSingleSubtable`, not a `Subtable` union
/// field itself.
pub type BaseArray = Vec<BaseRecord>;
#[derive(Clone, Debug)]
pub struct BaseRecord {
    pub glyph: GlyphHandle,
    pub anchors: Vec<Anchor>,
}
pub type GposCursiveSubtable = Vec<GposCursiveEntry>;
#[derive(Clone, Debug)]
pub struct GposCursiveEntry {
    pub target: GlyphHandle,
    pub enter: Anchor,
    pub exit: Anchor,
}
// `Copy` dropped: `first`/`second`/`first_values`/`second_values` all own
// heap allocations now.
#[derive(Clone, Debug)]
pub struct GposPairSubtable {
    pub first: Option<Box<ClassDef>>,
    pub second: Option<Box<ClassDef>>,
    pub first_values: Vec<Vec<PositionValue>>,
    pub second_values: Vec<Vec<PositionValue>>,
}
#[derive(Copy, Clone, Debug)]
pub struct PositionValue {
    pub dx: Pos,
    pub dy: Pos,
    pub d_width: Pos,
    pub d_height: Pos,
}
pub type GposSingleSubtable = Vec<GposSingleEntry>;
#[derive(Clone, Debug)]
pub struct GposSingleEntry {
    pub target: GlyphHandle,
    pub value: PositionValue,
}
// `Copy` dropped: `match_0`/`to` own `Vec`s now.
#[derive(Clone, Debug)]
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
#[derive(Debug)]
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
#[derive(Default, Debug)]
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
#[derive(Default, Debug)]
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
#[derive(Clone, Debug)]
pub struct ChainLookupApplication {
    pub index: TableId,
    pub lookup: LookupHandle,
}
pub type GsubLigatureSubtable = Vec<GsubLigatureEntry>;
#[derive(Clone, Debug)]
pub struct GsubLigatureEntry {
    pub from: Coverage,
    pub to: GlyphHandle,
}
pub type GsubMultiSubtable = Vec<GsubMultiEntry>;
#[derive(Clone, Debug)]
pub struct GsubMultiEntry {
    pub from: GlyphHandle,
    pub to: Coverage,
}
pub type GsubSingleSubtable = Vec<GsubSingleEntry>;
#[derive(Clone, Debug)]
pub struct GsubSingleEntry {
    pub from: GlyphHandle,
    pub to: GlyphHandle,
}
// `subtables: SubtableList`(`Vec<Option<Box<Subtable>>>`)を値で持つため
// `Copy` を落とす。常に `*mut`/`*const` 経由でしか触られない（値渡し・値
// コピーの箇所は無い）。
#[derive(Debug)]
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
// 所有する `Box` 配列。各要素は `None` にもなり得る（consolidate 中の一時的な
// 「取り除かれた」穴、または extend 展開の型不一致エラー経路で残る穴）。
// `stat.rs`（唯一の呼び出し元。下の doc comment 参照）など、読み取り時点で
// 穴が無いと分かっている箇所は `subtable_at`（下記）で `.expect()` して
// 安全な参照に戻す。
pub type SubtableList = Vec<Option<Box<Subtable>>>;
/// Read a `SubtableList` element as a shared reference, panicking if the
/// slot is empty. Every caller already assumed a slot could not be empty at
/// the point it reads one -- before `Box` made a hole `None` instead of a
/// dangling pointer, that assumption being wrong meant a silent
/// out-of-bounds-shaped dereference. Now it is a clean panic.
///
/// This used to return `SubtablePtr` (`*mut Subtable`), forcing every call
/// site to wrap its use in `unsafe {}` even though none of them ever wrote
/// through it. Re-checked fresh (the previous doc comment's "build.rs/
/// dump.rs/.../the chaining classifier all read... " no longer matches: a
/// grep for `subtable_at` across `src/` turns up exactly one caller left,
/// `otf_writer/stat.rs`, and only for reads) -- `&Subtable` says exactly
/// what every remaining caller needs, with no raw-pointer boundary left to
/// preserve.
pub(crate) fn subtable_at(list: &SubtableList, idx: usize) -> &Subtable {
    list[idx]
        .as_deref()
        .expect("subtable slot should not be empty at this point")
}
pub type LookupPtr = *mut Lookup;
/// A stable slot index into `OtlTable.lookups`, replacing the old borrowed
/// `*const Lookup` cross-reference (`LookupRef`). Every construction site
/// (both the binary-read and the JSON-parse path) already knows the exact
/// target index at push time -- the binary path because `OtlTable.lookups`
/// is fully built, in final order, before any `Feature`/`LanguageSystem` is
/// parsed; the JSON path via an explicit remap step run right after its own
/// (pre-final-order) name resolution finishes, see `table/otl/parse.rs`'s
/// `PendingLookupId`. Resolve through `lookup_at`, never by indexing
/// `OtlTable.lookups` directly, since consolidation can punch a hole (see
/// `LookupList` below) at any index after construction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LookupIdx(pub u32);
// Stage 6-4, third of the group -- see `LangSystemList`/`FeatureList` for
// the shape. `Lookup`'s own `Drop` (above) now does the type-dispatched
// `SubtableList` teardown `SubtableList` itself still can't do on its own.
// `Option` (not plain `Box`) for the same reason `SubtableList` already
// needed it: `consolidate_otl_table`'s fixed-point pruning loop removes a
// `Lookup` whose `subtables` end up empty, but other lookups/features
// already hold a `LookupIdx` pointing at *other*, still-live slots by
// position -- compacting the `Vec` (the old `Vec::retain`-based
// `otl_lookup_list_filter_env`) would silently shift every index after the
// removed one, so removal now punches a `None` hole in place instead (see
// `otl_lookup_list_punch_holes`).
pub type LookupList = Vec<Option<Box<Lookup>>>;
/// Resolve a `LookupIdx` against the `LookupList` it indexes into. Returns
/// `None` both for an out-of-range index and for a punched hole -- callers
/// that reach this point only after construction has already validated the
/// index (every push site guards against an invalid target) should treat a
/// `None` here as "this lookup was pruned by consolidation", not as a bug.
pub(crate) fn lookup_at(list: &LookupList, idx: LookupIdx) -> Option<&Lookup> {
    list.get(idx.0 as usize).and_then(Option::as_deref)
}
// 所有しない参照配列（`LookupList` の要素をインデックスで指すだけ）。分類その3。
pub type LookupRefList = Vec<LookupIdx>;
// `lookups: LookupRefList`(`Vec<LookupIdx>`)を値で持つため `Copy` を落とす。
#[derive(Debug)]
pub struct Feature {
    pub name: Vec<u8>,
    pub lookups: LookupRefList,
}
/// `lookups: LookupRefList` (`Vec<LookupIdx>`) needs no help -- it holds
/// only *borrowed* indices into `OtlTable.lookups`, so its own drop glue is
/// enough. `name` (a `Vec<u8>` since the `sds` sweep reached this field) now
/// also tears down for free, so `Feature` needs no manual `Drop` impl at all
/// anymore.
pub type FeaturePtr = *mut Feature;
/// Same shape as `LookupIdx`, indexing `OtlTable.features`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FeatureIdx(pub u32);
// Stage 6-4, second of the "owned pointer array" group -- see
// `LangSystemList`/`new_language` for the shape and rationale. `Option`
// wrapping added for the same reason as `LookupList` above -- `Feature`s
// referenced by a still-live `LanguageSystem.features`/`.required_feature`
// must keep their position when a *different* feature is pruned.
pub type FeatureList = Vec<Option<Box<Feature>>>;
/// Same contract as `lookup_at`.
pub(crate) fn feature_at(list: &FeatureList, idx: FeatureIdx) -> Option<&Feature> {
    list.get(idx.0 as usize).and_then(Option::as_deref)
}
// 所有しない参照配列（`FeatureList` の要素をインデックスで指すだけ）。
pub type FeatureRefList = Vec<FeatureIdx>;
// `required_feature`はインデックスなので無関係、`features: FeatureRefList`
// (`Vec<FeatureIdx>`)を値で持つため `Copy` を落とす。
#[derive(Debug)]
pub struct LanguageSystem {
    pub name: Vec<u8>,
    pub required_feature: Option<FeatureIdx>,
    pub features: FeatureRefList,
}
/// `required_feature` and `features` both hold *borrowed* indices into
/// `OtlTable`'s own `features` list, so nothing there needs freeing --
/// `features`'s backing `Vec` drops itself, and `name` (a `Vec<u8>` since
/// the `sds` sweep reached this field) now also tears down for free, so
/// `LanguageSystem` needs no manual `Drop` impl at all anymore.
// Stage 6-4 pilot for the "owned pointer array" shape (plan classification
// その3): the elements are `Box`es now, not raw `*mut`, so the `Vec`'s own
// drop glue frees every element -- see RUST_MIGRATION.md.
pub type LangSystemList = Vec<Box<LanguageSystem>>;
// 3つとも値でVecを持つため `Copy` を落とす。`Font.gsub`/`Font.gpos` は
// `*mut OtlTable` フィールドで、crate全体を通じて常にポインタ経由。
#[derive(Debug)]
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
// The not-yet-owned scratch/rejection cases in `table/otl/{read,parse}.rs`
// that used to call `otfcc_delete_lookup` on a `Lookup` not yet pushed into
// a `LookupList` now just let the local `Box<Lookup>` drop naturally
// instead -- `otfcc_delete_lookup` itself (`drop(Box::from_raw(lookup))`)
// had no other callers left once both sites were converted, confirmed by
// grep before deleting it.
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
// Was a `*mut c_void` context pointer + `Option<unsafe fn(...)>` predicate,
// type-erasing the two callers' concrete predicates behind a shared shape
// purely so one function pointer type could stand in for both -- the same
// "type erasure that was never actually needed" pattern Stage 9 Phase 9
// found in `libcff/cff_index.rs`'s `new_index_by_callback` (resolved there
// via `impl Iterator`) and `libcff/cff_dict.rs`'s `parse_to_callback`
// (resolved via `impl FnMut`). The sole caller already knows its predicate
// at compile time, so a generic `impl FnMut(&Lookup) -> bool` carries the
// same information with no unsafe function-pointer cast and no env
// pointer -- every element here is a live `Box<Lookup>`, never null, so
// there is no nullable-pointer case for the predicate to handle either.
/// Replaces the old `Vec::retain`-based `otl_lookup_list_filter_env`:
/// `retain` compacts, shifting every surviving element after a removed one
/// down by one slot -- fatal now that `Feature.lookups`/other `LookupIdx`
/// values reference `OtlTable.lookups` *by position*. This punches a `None`
/// hole in place instead, so every index that was valid before a call
/// (other than one pointing at a just-rejected slot) is still valid after
/// it. Returns whether anything was actually punched, for the fixed-point
/// loop in `consolidate.rs` that used to watch `.len()` shrink -- a
/// hole-preserving `Vec` never shrinks, so "did this pass change anything"
/// has to be signalled explicitly instead.
pub(crate) fn otl_lookup_list_punch_holes(arr: &mut LookupList, mut pred: impl FnMut(&Lookup) -> bool) -> bool {
    let mut punched = false;
    for slot in arr.iter_mut() {
        if let Some(lookup) = slot {
            if !pred(lookup) {
                *slot = None;
                punched = true;
            }
        }
    }
    punched
}
// `LookupIdx`単体の要素インターフェース(旧`otl_lookup_ref_init`/`_copy`/
// `_dispose`)は`LookupRefList`自体の死んだ`.copy`スロットからしか呼ばれて
// おらず削除——`.dispose`(旧`otl_lookup_ref_dispose`)・`.replace`
// (旧`otl_lookup_ref_list_dispose`/`_replace`)も含め、`LookupIdx`は
// 所有物を持たない（`LookupList`が指し先の`Lookup`を所有する）ため、
// これらは元から`*arr = Vec::new()`/`*dst = src`という素のVec操作でしか
// なかった——呼び出し元(`table/otl/parse.rs`)がこのPRで
// `PendingLookups`/`PendingFeatures`の直接構築に置き換わったため、
// ラッパー自体も削除。
// Same closure-based de-type-erasure as `otl_lookup_list_punch_holes`
// above, but `LookupRefList`'s elements are themselves non-owning
// borrowed indices (`LookupIdx`, into `OtlTable.lookups`) rather than
// owned `Box`es -- unlike that list, an index here really can resolve to
// a hole (`lookup_at` returns `None` for one, same as an out-of-range
// index), so the predicate takes `Option<&Lookup>` and this helper needs
// the owning `LookupList` to resolve each index through.
pub(crate) fn otl_lookup_ref_list_filter_env(
    arr: &mut LookupRefList,
    lookups: &LookupList,
    mut pred: impl FnMut(Option<&Lookup>) -> bool,
) {
    arr.retain(|&idx| pred(lookup_at(lookups, idx)));
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
// Same hole-punching shape as `otl_lookup_list_punch_holes`.
pub(crate) fn otl_feature_list_punch_holes(arr: &mut FeatureList, mut pred: impl FnMut(&Feature) -> bool) -> bool {
    let mut punched = false;
    for slot in arr.iter_mut() {
        if let Some(feature) = slot {
            if !pred(feature) {
                *slot = None;
                punched = true;
            }
        }
    }
    punched
}
// `FeatureIdx`単体の要素インターフェースは`FeatureRefList`の死んだ`.copy`
// からしか呼ばれておらず削除。`FeatureIdx`は所有物を持たない
// （`FeatureList`が指し先の`Feature`を所有する）。`.replace`
// (旧`otl_feature_ref_list_replace`)も`LookupRefList`と同じ理由で削除
// ——呼び出し元が`PendingFeatures`の直接構築に置き換わった。
// `LookupRefList`と同じく所有物を持たない要素の配列。
pub(crate) fn otl_feature_ref_list_dispose(arr: &mut FeatureRefList) {
    *arr = Vec::new();
}
// Same closure-based de-type-erasure as `otl_lookup_ref_list_filter_env`
// (`FeatureRefList`'s elements are likewise non-owning borrowed indices,
// into `OtlTable.features`).
pub(crate) fn otl_feature_ref_list_filter_env(
    arr: &mut FeatureRefList,
    features: &FeatureList,
    mut pred: impl FnMut(Option<&Feature>) -> bool,
) {
    arr.retain(|&idx| pred(feature_at(features, idx)));
}
/// Replaces the old `__caryll_allocate_clean`-into-a-`*mut`-out-parameter
/// constructor: `Box` is the allocation, and the struct literal is the
/// zero-initialization the `calloc` used to provide.
#[inline]
pub(crate) fn new_language() -> Box<LanguageSystem> {
    Box::new(LanguageSystem {
        name: Vec::new(),
        required_feature: None,
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
            (OTL_TYPE_UNKNOWN, "unknown"),
            (OTL_TYPE_GSUB_UNKNOWN, "gsub_unknown"),
            (OTL_TYPE_GSUB_SINGLE, "gsub_single"),
            (OTL_TYPE_GSUB_MULTIPLE, "gsub_multiple"),
            (OTL_TYPE_GSUB_ALTERNATE, "gsub_alternate"),
            (OTL_TYPE_GSUB_LIGATURE, "gsub_ligature"),
            (OTL_TYPE_GSUB_CONTEXT, "gsub_context"),
            (OTL_TYPE_GSUB_CHAINING, "gsub_chaining"),
            (OTL_TYPE_GSUB_EXTEND, "gsub_extend"),
            (OTL_TYPE_GSUB_REVERSE, "gsub_reverse"),
            (OTL_TYPE_GPOS_UNKNOWN, "gpos_unknown"),
            (OTL_TYPE_GPOS_SINGLE, "gpos_single"),
            (OTL_TYPE_GPOS_PAIR, "gpos_pair"),
            (OTL_TYPE_GPOS_CURSIVE, "gpos_cursive"),
            (OTL_TYPE_GPOS_MARK_TO_BASE, "gpos_mark_to_base"),
            (OTL_TYPE_GPOS_MARK_TO_LIGATURE, "gpos_mark_to_ligature"),
            (OTL_TYPE_GPOS_MARK_TO_MARK, "gpos_mark_to_mark"),
            (OTL_TYPE_GPOS_CONTEXT, "gpos_context"),
            (OTL_TYPE_GPOS_CHAINING, "gpos_chaining"),
            (OTL_TYPE_GPOS_EXTEND, "gpos_extend"),
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
        assert_eq!(unnamed.name(), "unknown");
        assert_eq!(
            LookupType::from_file(OTL_TYPE_GSUB_UNKNOWN, 0xffff).raw(),
            65551
        );
        assert_eq!(::core::mem::size_of::<LookupType>(), 4);
    }
}
