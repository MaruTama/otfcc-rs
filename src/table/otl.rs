pub mod budget;
pub mod build;
pub mod classdef;
pub mod constants;
pub mod coverage;
pub mod dump;
pub mod kind;
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
/// **Deliberately not an `enum`.** The value is read from the font as the
/// lookup type plus the table's base, so anything in `16..=65551` can turn
/// up. An unrecognised type is carried through as-is: `read_otl_subtable`
/// reads nothing for it, and the lookup's generated name puts the raw number
/// in the output as hex (`lookup_0019_3`, from `read.rs`). An enum could not
/// hold such a value, and rejecting it would change the JSON written for a
/// font with an unknown lookup type.
///
/// So this is a newtype over the number, with the known values as named
/// constants, built from the file only through [`LookupType::from_file`].
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
    /// `OTL_TYPE_GPOS_UNKNOWN` for gpos. `raw` comes straight from the file
    /// and is not validated here.
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
    /// The comparisons are `>`, not `>=`: `OTL_TYPE_UNKNOWN` and
    /// `OTL_TYPE_GSUB_UNKNOWN` give 0, while `OTL_TYPE_GPOS_UNKNOWN` (32) is
    /// above gsub's base and so reads as gsub format 16. Only a font
    /// declaring a gpos lookup of format 0 reaches this; the number goes into
    /// the lookup header, so the C behaviour is kept.
    /// `file_format_undoes_the_table_base` pins it.
    pub const fn file_format(self) -> u32 {
        if self.0 > OTL_TYPE_GPOS_UNKNOWN.0 {
            self.0 - OTL_TYPE_GPOS_UNKNOWN.0
        } else { self.0.saturating_sub(OTL_TYPE_GSUB_UNKNOWN.0) }
    }

    /// The name this type has in otfcc's JSON, and the key its lookups are
    /// looked up by when reading JSON back.
    ///
    /// A value outside the known types falls back to index 0's own text,
    /// which keeps the function total without inventing a name.
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
            // An `Extend` normally has its subtable taken by `read.rs` when
            // extensions are resolved; one left here simply drops.
            Subtable::Extend(_) => {}
        }
    }
}
#[derive(Debug)]
pub struct ExtendSubtable {
    pub lookup_type: LookupType,
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
    pub sequence: Vec<Coverage>,
    pub to: Coverage,
}
// A contextual or chaining subtable: a single rule (`Canonical`, format 3),
// or a rule set read from format 1 (`Poly`) or format 2 (`Classified`),
// which `build.rs` writes differently.
#[derive(Debug)]
pub enum ChainingSubtable {
    Canonical(ChainingRule),
    Poly(ChainingRuleSet),
    Classified(ChainingRuleSet),
}
/// A rule set. A rule that fails to read is kept as `None`. `bc`/`ic`/`fc`
/// are the backtrack, input and lookahead class definitions, set only by
/// `classifier.rs` when it classifies a rule set.
#[derive(Default, Debug)]
pub struct ChainingRuleSet {
    pub rules: Vec<Option<Box<ChainingRule>>>,
    pub bc: Option<Box<ClassDef>>,
    pub ic: Option<Box<ClassDef>>,
    pub fc: Option<Box<ClassDef>>,
}
#[derive(Default, Debug)]
pub struct ChainingRule {
    pub match_count: TableId,
    pub input_begins: TableId,
    pub input_ends: TableId,
    pub sequence: Vec<Coverage>,
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
#[derive(Debug)]
pub struct Lookup {
    pub name: Vec<u8>,
    pub lookup_type: LookupType,
    pub _offset: u32,
    pub flags: u16,
    pub subtables: SubtableList,
}
// A lookup's subtables. A slot is `None` when consolidation removed the
// subtable, or extension resolution hit a type mismatch; readers that know
// there are no holes use `iter_subtables`.
pub type SubtableList = Vec<Option<Box<Subtable>>>;
/// Iterate a `SubtableList` as shared references, panicking on the first
/// empty slot reached. Lazy: the panic fires only when iteration gets there.
pub(crate) fn iter_subtables(list: &SubtableList) -> impl Iterator<Item = &Subtable> {
    list.iter().map(|slot| {
        slot.as_deref()
            .expect("subtable slot should not be empty at this point")
    })
}
/// A slot index into `OtlTable.lookups`. Both the binary reader and the JSON
/// parser know the final index when they create one (see
/// `table/otl/parse.rs`'s `PendingLookupId`). Resolve it through
/// `lookup_at`, since consolidation can empty a slot.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LookupIdx(pub u32);
// `None` slots are lookups that consolidation removed: removing one in place
// keeps every other `LookupIdx` valid (see `otl_lookup_list_punch_holes`).
pub type LookupList = Vec<Option<Box<Lookup>>>;
/// Resolve a `LookupIdx` against the `LookupList` it indexes into. Returns
/// `None` both for an out-of-range index and for a punched hole -- callers
/// that reach this point only after construction has already validated the
/// index (every push site guards against an invalid target) should treat a
/// `None` here as "this lookup was pruned by consolidation", not as a bug.
pub(crate) fn lookup_at(list: &LookupList, idx: LookupIdx) -> Option<&Lookup> {
    list.get(idx.0 as usize).and_then(Option::as_deref)
}
// Indices into `OtlTable.lookups`; owns nothing.
pub type LookupRefList = Vec<LookupIdx>;
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
/// Same shape as `LookupIdx`, indexing `OtlTable.features`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FeatureIdx(pub u32);
// `None` slots are features that consolidation removed, as for
// `LookupList`.
pub type FeatureList = Vec<Option<Box<Feature>>>;
/// Same contract as `lookup_at`.
pub(crate) fn feature_at(list: &FeatureList, idx: FeatureIdx) -> Option<&Feature> {
    list.get(idx.0 as usize).and_then(Option::as_deref)
}
// Indices into `OtlTable.features`; owns nothing.
pub type FeatureRefList = Vec<FeatureIdx>;
#[derive(Debug)]
pub struct LanguageSystem {
    pub name: Vec<u8>,
    pub required_feature: Option<FeatureIdx>,
    pub features: FeatureRefList,
}
/// `required_feature` and `features` are indices into `OtlTable.features`.
pub type LangSystemList = Vec<Box<LanguageSystem>>;
#[derive(Debug)]
pub struct OtlTable {
    pub lookups: LookupList,
    pub features: FeatureList,
    pub languages: LangSystemList,
}
#[inline]
pub(crate) fn new_lookup() -> Box<Lookup> {
    Box::new(Lookup {
        name: Vec::new(),
        lookup_type: OTL_TYPE_UNKNOWN,
        _offset: 0,
        flags: 0,
        subtables: Vec::new(),
    })
}
/// Empties, in place, every slot of `arr` whose lookup fails `pred`, so the
/// indices of the others stay valid. Returns whether it emptied any, for
/// the fixed-point loop in `consolidate/otl.rs`.
pub(crate) fn otl_lookup_list_punch_holes(arr: &mut LookupList, mut pred: impl FnMut(&Lookup) -> bool) -> bool {
    let mut punched = false;
    for slot in arr.iter_mut() {
        if let Some(lookup) = slot
            && !pred(lookup) {
                *slot = None;
                punched = true;
            }
    }
    punched
}
// Keeps the indices whose lookup passes `pred`; an index whose slot is
// empty resolves to `None`.
pub(crate) fn otl_lookup_ref_list_filter_env(
    arr: &mut LookupRefList,
    lookups: &LookupList,
    mut pred: impl FnMut(Option<&Lookup>) -> bool,
) {
    arr.retain(|&idx| pred(lookup_at(lookups, idx)));
}
#[inline]
pub(crate) fn new_feature() -> Box<Feature> {
    Box::new(Feature {
        name: Vec::new(),
        lookups: Vec::new(),
    })
}
pub(crate) fn otl_feature_list_punch_holes(arr: &mut FeatureList, mut pred: impl FnMut(&Feature) -> bool) -> bool {
    let mut punched = false;
    for slot in arr.iter_mut() {
        if let Some(feature) = slot
            && !pred(feature) {
                *slot = None;
                punched = true;
            }
    }
    punched
}
pub(crate) fn otl_feature_ref_list_dispose(arr: &mut FeatureRefList) {
    *arr = Vec::new();
}
// Keeps the indices whose feature passes `pred`, as
// `otl_lookup_ref_list_filter_env` does for lookups.
pub(crate) fn otl_feature_ref_list_filter_env(
    arr: &mut FeatureRefList,
    features: &FeatureList,
    mut pred: impl FnMut(Option<&Feature>) -> bool,
) {
    arr.retain(|&idx| pred(feature_at(features, idx)));
}
#[inline]
pub(crate) fn new_language() -> Box<LanguageSystem> {
    Box::new(LanguageSystem {
        name: Vec::new(),
        required_feature: None,
        features: Vec::new(),
    })
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
