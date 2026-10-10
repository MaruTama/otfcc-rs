//! What each kind of lookup does at every stage of the pipeline, in one place.
//!
//! A lookup's subtables are stored as the [`Subtable`] enum, but what to do
//! with them depends on the lookup's [`LookupType`], and several types share
//! a variant (multiple and alternate substitution both hold `GsubMulti`, the
//! two chaining types both hold `Chaining`, mark-to-base and mark-to-mark both
//! hold `GposMarkToSingle`). Each kind implements [`LookupKind`] once --
//! binary read, JSON parse, JSON dump, binary build, consolidation,
//! unconsolidation and the `usMaxContext` statistic -- and [`LOOKUP_KINDS`]
//! lists them; adding a kind means implementing the trait and adding it to
//! the list.
//!
//! The context and extension types are not here. Both only exist while a
//! binary table is being read: a context subtable is read straight into the
//! chaining representation, and an extension subtable is unwrapped into the
//! subtable it points at. `read_otl_subtable` handles them itself.
use crate::consolidate::otl::chaining::consolidate_chaining;
use crate::consolidate::otl::gpos_cursive::consolidate_gpos_cursive;
use crate::consolidate::otl::gpos_pair::consolidate_gpos_pair;
use crate::consolidate::otl::gpos_single::consolidate_gpos_single;
use crate::consolidate::otl::gsub_ligature::consolidate_gsub_ligature;
use crate::consolidate::otl::gsub_multi::consolidate_gsub_multi;
use crate::consolidate::otl::gsub_reverse::consolidate_gsub_reverse;
use crate::consolidate::otl::gsub_single::consolidate_gsub_single;
use crate::consolidate::otl::mark::{consolidate_mark_to_ligature, consolidate_mark_to_single};
use crate::otf_reader::unconsolidate::unconsolidate_chaining;
use otfcc_json::BuiltValue;
use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use otfcc_json::ParsedValue;
use crate::support::primitives::{GlyphId, TableId};
use crate::table::otl::budget::OtlReadBudget;
use crate::table::otl::build::{LookupWriteCtx, write_each_subtable, write_each_subtable_split};
use crate::table::otl::subtables::chaining::classifier::classified_build_chaining;
use crate::table::otl::subtables::chaining::common::chaining_rule_const;
use crate::table::otl::subtables::chaining::dump::otl_dump_chaining;
use crate::table::otl::subtables::chaining::parse::otl_parse_chaining;
use crate::table::otl::subtables::chaining::read::otl_read_chaining;
use crate::table::otl::subtables::gpos_cursive::{
    build_gpos_cursive, otl_gpos_dump_cursive, otl_gpos_parse_cursive, otl_read_gpos_cursive,
};
use crate::table::otl::subtables::gpos_mark_to_ligature::{
    build_gpos_mark_to_ligature, otl_gpos_dump_mark_to_ligature, otl_gpos_parse_mark_to_ligature,
    otl_read_gpos_mark_to_ligature,
};
use crate::table::otl::subtables::gpos_mark_to_single::{
    build_gpos_mark_to_single, otl_gpos_dump_mark_to_single, otl_gpos_parse_mark_to_single,
    otl_read_gpos_mark_to_single,
};
use crate::table::otl::subtables::gpos_pair::{
    build_gpos_pair, otl_gpos_dump_pair, otl_gpos_parse_pair, otl_read_gpos_pair,
};
use crate::table::otl::subtables::gpos_single::{
    build_gpos_single, otl_gpos_dump_single, otl_gpos_parse_single, otl_read_gpos_single,
};
use crate::table::otl::subtables::gsub_ligature::{
    build_gsub_ligature_subtable, otl_gsub_dump_ligature, otl_gsub_parse_ligature,
    otl_read_gsub_ligature,
};
use crate::table::otl::subtables::gsub_multi::{
    build_gsub_multi_subtable_split, otl_gsub_dump_multi, otl_gsub_parse_multi, otl_read_gsub_multi,
};
use crate::table::otl::subtables::gsub_reverse::{
    build_gsub_reverse, otl_gsub_dump_reverse, otl_gsub_parse_reverse, otl_read_gsub_reverse,
};
use crate::table::otl::subtables::gsub_single::{
    build_gsub_single_subtable, otl_gsub_dump_single, otl_gsub_parse_single, otl_read_gsub_single,
};
use crate::table::otl::{
    Lookup, LookupList, LookupType, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE,
    OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK,
    OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING,
    OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE,
    Subtable, iter_subtables,
};

/// What consolidating one subtable may need besides the subtable itself.
/// Only chaining subtables use more than the glyph order: they resolve the
/// lookups their rules apply, by name or index, against the whole table.
/// See `consolidate_otl_table` for why the lookup being consolidated is
/// passed as `self_index`/`self_name` instead of being read from `lookups`.
pub struct LookupConsolidateCtx<'a> {
    pub glyph_order: &'a GlyphOrder,
    pub lookups: &'a LookupList,
    pub self_index: TableId,
    pub self_name: &'a [u8],
    pub options: &'a Options,
}

pub trait LookupKind: Sync {
    fn lookup_type(&self) -> LookupType;
    /// Reads one subtable at `offset` in a GSUB or GPOS table.
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable>;
    /// Reads one subtable from otfcc's JSON.
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable>;
    /// Writes one subtable as otfcc's JSON.
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue;
    /// Builds every subtable of `lookup` into `ctx.subtables` and returns how
    /// many there are. Builds a whole lookup rather than one subtable because
    /// chaining lookups classify their subtables together.
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId;
    /// Consolidates one subtable. Returns true when nothing is left of it and
    /// it should be removed.
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool;
    /// Raises `maxc` to the longest glyph sequence `lookup` looks at, for
    /// OS/2 `usMaxContext`. Kinds that only ever look at one glyph leave it.
    fn raise_max_context(&self, _lookup: &Lookup, _maxc: &mut u16) {}
    /// Turns a lookup read from a binary font into the shape its JSON form
    /// has, before it is dumped.
    fn unconsolidate(&self, _lookup: &mut Lookup) {}
}

struct GsubSingle;
impl LookupKind for GsubSingle {
    fn lookup_type(&self) -> LookupType {
        OTL_TYPE_GSUB_SINGLE
    }
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable> {
        otl_read_gsub_single(data, offset, max_glyphs, budget)
    }
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable> {
        otl_gsub_parse_single(json)
    }
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue {
        otl_gsub_dump_single(subtable)
    }
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId {
        write_each_subtable(lookup, ctx, build_gsub_single_subtable)
    }
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool {
        consolidate_gsub_single(ctx.glyph_order, subtable)
    }
}

/// Multiple and alternate substitution: one glyph to a sequence, or to a set
/// to choose from. Both are stored, read and written the same way.
struct GsubMulti(LookupType);
impl LookupKind for GsubMulti {
    fn lookup_type(&self) -> LookupType {
        self.0
    }
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable> {
        otl_read_gsub_multi(data, offset, max_glyphs, budget)
    }
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable> {
        otl_gsub_parse_multi(json)
    }
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue {
        otl_gsub_dump_multi(subtable)
    }
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId {
        write_each_subtable_split(lookup, ctx, build_gsub_multi_subtable_split)
    }
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool {
        consolidate_gsub_multi(ctx.glyph_order, subtable)
    }
}

struct GsubLigature;
impl LookupKind for GsubLigature {
    fn lookup_type(&self) -> LookupType {
        OTL_TYPE_GSUB_LIGATURE
    }
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable> {
        otl_read_gsub_ligature(data, offset, max_glyphs, budget)
    }
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable> {
        otl_gsub_parse_ligature(json)
    }
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue {
        otl_gsub_dump_ligature(subtable)
    }
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId {
        write_each_subtable(lookup, ctx, build_gsub_ligature_subtable)
    }
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool {
        consolidate_gsub_ligature(ctx.glyph_order, subtable)
    }
    fn raise_max_context(&self, lookup: &Lookup, maxc: &mut u16) {
        for subtable in iter_subtables(&lookup.subtables) {
            let Subtable::GsubLigature(entries) = subtable else {
                unreachable!()
            };
            for entry in entries {
                if (*maxc as usize) < entry.from.len() {
                    *maxc = entry.from.len() as u16;
                }
            }
        }
    }
}

/// Chaining substitution or positioning. Context lookups read from a binary
/// font end up here too, under the chaining type.
struct Chaining(LookupType);
impl LookupKind for Chaining {
    fn lookup_type(&self) -> LookupType {
        self.0
    }
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable> {
        otl_read_chaining(data, offset, max_glyphs, budget)
    }
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable> {
        otl_parse_chaining(json)
    }
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue {
        otl_dump_chaining(subtable)
    }
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId {
        classified_build_chaining(lookup, ctx.subtables, ctx.last_offset)
    }
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool {
        consolidate_chaining(
            ctx.glyph_order,
            ctx.lookups,
            ctx.self_index,
            ctx.self_name,
            subtable,
            ctx.options,
        )
    }
    fn raise_max_context(&self, lookup: &Lookup, maxc: &mut u16) {
        for subtable in iter_subtables(&lookup.subtables) {
            let Subtable::Chaining(subtable) = subtable else {
                unreachable!()
            };
            let match_count = chaining_rule_const(subtable).match_count;
            if *maxc < match_count {
                *maxc = match_count;
            }
        }
    }
    fn unconsolidate(&self, lookup: &mut Lookup) {
        unconsolidate_chaining(lookup);
    }
}

struct GsubReverse;
impl LookupKind for GsubReverse {
    fn lookup_type(&self) -> LookupType {
        OTL_TYPE_GSUB_REVERSE
    }
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable> {
        otl_read_gsub_reverse(data, offset, max_glyphs, budget)
    }
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable> {
        otl_gsub_parse_reverse(json)
    }
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue {
        otl_gsub_dump_reverse(subtable)
    }
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId {
        write_each_subtable(lookup, ctx, build_gsub_reverse)
    }
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool {
        consolidate_gsub_reverse(ctx.glyph_order, subtable)
    }
    fn raise_max_context(&self, lookup: &Lookup, maxc: &mut u16) {
        for subtable in iter_subtables(&lookup.subtables) {
            let Subtable::GsubReverse(subtable) = subtable else {
                unreachable!()
            };
            if *maxc < subtable.match_count {
                *maxc = subtable.match_count;
            }
        }
    }
}

struct GposSingle;
impl LookupKind for GposSingle {
    fn lookup_type(&self) -> LookupType {
        OTL_TYPE_GPOS_SINGLE
    }
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable> {
        otl_read_gpos_single(data, offset, max_glyphs, budget)
    }
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable> {
        otl_gpos_parse_single(json)
    }
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue {
        otl_gpos_dump_single(subtable)
    }
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId {
        write_each_subtable(lookup, ctx, build_gpos_single)
    }
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool {
        consolidate_gpos_single(ctx.glyph_order, subtable)
    }
}

/// Raises `maxc` to 2, for kinds that always look at a pair of glyphs.
fn raise_to_pair(maxc: &mut u16) {
    if *maxc < 2 {
        *maxc = 2;
    }
}

struct GposPair;
impl LookupKind for GposPair {
    fn lookup_type(&self) -> LookupType {
        OTL_TYPE_GPOS_PAIR
    }
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable> {
        otl_read_gpos_pair(data, offset, max_glyphs, budget)
    }
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable> {
        otl_gpos_parse_pair(json)
    }
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue {
        otl_gpos_dump_pair(subtable)
    }
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId {
        write_each_subtable(lookup, ctx, build_gpos_pair)
    }
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool {
        consolidate_gpos_pair(ctx.glyph_order, subtable)
    }
    fn raise_max_context(&self, _lookup: &Lookup, maxc: &mut u16) {
        raise_to_pair(maxc);
    }
}

struct GposCursive;
impl LookupKind for GposCursive {
    fn lookup_type(&self) -> LookupType {
        OTL_TYPE_GPOS_CURSIVE
    }
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable> {
        otl_read_gpos_cursive(data, offset, max_glyphs, budget)
    }
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable> {
        otl_gpos_parse_cursive(json)
    }
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue {
        otl_gpos_dump_cursive(subtable)
    }
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId {
        write_each_subtable(lookup, ctx, build_gpos_cursive)
    }
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool {
        consolidate_gpos_cursive(ctx.glyph_order, subtable)
    }
}

/// Mark-to-base and mark-to-mark: both attach a mark to one other glyph, and
/// are stored, read and written the same way.
struct GposMarkToSingle(LookupType);
impl LookupKind for GposMarkToSingle {
    fn lookup_type(&self) -> LookupType {
        self.0
    }
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable> {
        otl_read_gpos_mark_to_single(data, offset, max_glyphs, budget)
    }
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable> {
        otl_gpos_parse_mark_to_single(json)
    }
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue {
        otl_gpos_dump_mark_to_single(subtable)
    }
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId {
        write_each_subtable(lookup, ctx, build_gpos_mark_to_single)
    }
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool {
        consolidate_mark_to_single(ctx.glyph_order, subtable)
    }
    fn raise_max_context(&self, _lookup: &Lookup, maxc: &mut u16) {
        raise_to_pair(maxc);
    }
}

struct GposMarkToLigature;
impl LookupKind for GposMarkToLigature {
    fn lookup_type(&self) -> LookupType {
        OTL_TYPE_GPOS_MARK_TO_LIGATURE
    }
    fn read_subtable(
        &self,
        data: &[u8],
        offset: u32,
        max_glyphs: GlyphId,
        budget: &mut OtlReadBudget,
    ) -> Option<Subtable> {
        otl_read_gpos_mark_to_ligature(data, offset, max_glyphs, budget)
    }
    fn parse_subtable(&self, json: Option<&ParsedValue>) -> Option<Subtable> {
        otl_gpos_parse_mark_to_ligature(json)
    }
    fn dump_subtable(&self, subtable: &Subtable) -> BuiltValue {
        otl_gpos_dump_mark_to_ligature(subtable)
    }
    fn build_lookup(&self, lookup: &Lookup, ctx: &mut LookupWriteCtx) -> TableId {
        write_each_subtable(lookup, ctx, build_gpos_mark_to_ligature)
    }
    fn consolidate_subtable(&self, subtable: &mut Subtable, ctx: &LookupConsolidateCtx) -> bool {
        consolidate_mark_to_ligature(ctx.glyph_order, subtable)
    }
    fn raise_max_context(&self, _lookup: &Lookup, maxc: &mut u16) {
        raise_to_pair(maxc);
    }
}

/// Every kind of lookup that outlives reading a binary font.
///
/// The order is the order a JSON lookup's `type` is tried in. A lookup whose
/// `type` is missing gets one warning per kind tried, so the order and the
/// length of this list both show in the log.
pub static LOOKUP_KINDS: [&dyn LookupKind; 13] = [
    &GsubSingle,
    &GsubMulti(OTL_TYPE_GSUB_MULTIPLE),
    &GsubMulti(OTL_TYPE_GSUB_ALTERNATE),
    &GsubLigature,
    &Chaining(OTL_TYPE_GSUB_CHAINING),
    &GsubReverse,
    &GposSingle,
    &GposPair,
    &GposCursive,
    &Chaining(OTL_TYPE_GPOS_CHAINING),
    &GposMarkToSingle(OTL_TYPE_GPOS_MARK_TO_BASE),
    &GposMarkToSingle(OTL_TYPE_GPOS_MARK_TO_MARK),
    &GposMarkToLigature,
];

/// The kind of a lookup of type `lookup_type`, if it is one of
/// [`LOOKUP_KINDS`].
pub fn lookup_kind(lookup_type: LookupType) -> Option<&'static dyn LookupKind> {
    LOOKUP_KINDS
        .iter()
        .copied()
        .find(|kind| kind.lookup_type() == lookup_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_has_its_own_type_and_name() {
        for (i, a) in LOOKUP_KINDS.iter().enumerate() {
            assert_ne!(a.lookup_type().name(), "unknown");
            for b in &LOOKUP_KINDS[..i] {
                assert_ne!(a.lookup_type(), b.lookup_type());
            }
            assert_eq!(
                lookup_kind(a.lookup_type()).unwrap().lookup_type(),
                a.lookup_type()
            );
        }
    }

    #[test]
    fn context_and_extension_types_have_no_kind() {
        use crate::table::otl::{
            OTL_TYPE_GPOS_CONTEXT, OTL_TYPE_GPOS_EXTEND, OTL_TYPE_GSUB_CONTEXT,
            OTL_TYPE_GSUB_EXTEND, OTL_TYPE_UNKNOWN,
        };
        for t in [
            OTL_TYPE_GSUB_CONTEXT,
            OTL_TYPE_GPOS_CONTEXT,
            OTL_TYPE_GSUB_EXTEND,
            OTL_TYPE_GPOS_EXTEND,
            OTL_TYPE_UNKNOWN,
        ] {
            assert!(lookup_kind(t).is_none());
        }
    }
}
