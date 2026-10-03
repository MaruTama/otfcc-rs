pub mod chaining;
pub mod common;
pub mod gdef;
pub mod gpos_cursive;
pub mod gpos_pair;
pub mod gpos_single;
pub mod gsub_ligature;
pub mod gsub_multi;
pub mod gsub_reverse;
pub mod gsub_single;
pub mod mark;

use crate::logger::ByteStr;
use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use crate::support::primitives::TableId;
use crate::table::otl::{
    otl_feature_list_punch_holes, otl_feature_ref_list_filter_env, otl_lookup_list_punch_holes,
    otl_lookup_ref_list_filter_env,
};
use crate::table::otl::kind::{LookupConsolidateCtx, lookup_kind};
use crate::table::otl::{Lookup, LookupList, OtlTable};

pub fn consolidate_lookup(
    glyph_order: &GlyphOrder,
    lookups: &LookupList,
    self_index: TableId,
    self_name: &[u8],
    lookup: &mut Lookup,
    options: &Options,
) {
    if lookup.subtables.is_empty() {
        return;
    }
    let Some(kind) = lookup_kind(lookup.type_0) else {
        return;
    };
    let ctx = LookupConsolidateCtx { glyph_order, lookups, self_index, self_name, options };
    let stage = crate::logger::stage(ByteStr(&lookup.name));
    // The "Ignored empty subtable" warning below can fire up to 300,000
    // times for a font whose subtables mostly fail to parse (a lookup can
    // hold up to `MAX_TOTAL_SUBTABLES_PER_LOOKUP` (1,000) subtables and a
    // table up to `MAX_TOTAL_LOOKUPS_PER_TABLE` (300) lookups); CI fuzz
    // found exactly this shape. `tracing::warn!` formats nothing when
    // warnings are not being printed, so that volume costs nothing then.
    for (j, slot) in lookup.subtables.iter_mut().enumerate() {
        if slot.is_none() {
            tracing::warn!("[Consolidate] Ignored empty subtable {} of lookup {}.\n", j as i32, ByteStr(&lookup.name));
        } else {
            let sub = slot.as_deref_mut().unwrap();
            let subtable_removed = kind.consolidate_subtable(sub, &ctx);
            if subtable_removed {
                // Was a `fndel: SubtableRemover` parameter, one
                // `LookupType`-keyed function pointer per call site
                // below, each `transmute`d from `*mut ConcreteType` to
                // `*mut Subtable` -- sound only because `Subtable` used
                // to be a union with no discriminant to misinterpret.
                // Now that it is an enum, `Subtable`'s own `Drop` does
                // this dispatch, self-describing off the enum's tag, so
                // setting the slot to `None` (dropping the `Box` in
                // place) is all that is needed -- no per-type function
                // pointer, no separate explicit `Box::from_raw`.
                *slot = None;
                tracing::warn!("[Consolidate] Ignored empty subtable {} of lookup {}.\n", j as i32, ByteStr(&lookup.name));
            }
        }
    }
    // `Vec::retain` drops every discarded `Box<Subtable>` in place (its
    // `Drop` runs as part of the retain-internal shift), the same thing
    // the old manual `.take()`-then-`truncate()` two-pass compaction did
    // by hand -- no risk of the double-owned-`Box` hazard that reasoning
    // used to warn about, since `retain` never leaves two slots pointing
    // at the same allocation to begin with.
    lookup.subtables.retain(|s| s.is_some());
    if lookup.subtables.is_empty() {
        tracing::warn!("[Consolidate] Lookup {} is empty and will be removed.\n", ByteStr(&lookup.name));
    }
    stage.finish();
}
// Stage L-7: `table` is a real `&mut OtlTable` now, not a raw pointer --
// closing the aliasing hazard the plan doc flagged this stage for. The one
// wrinkle: `consolidate_lookup`'s call into `consolidate_chaining`
// still needs read access to *every* lookup, including the very one whose
// `&mut Lookup` this loop is holding at the time (a chaining rule can name
// its own containing lookup -- `k == self_index` below). A blanket
// `&table.lookups` alongside a live `&mut Lookup` borrowed from inside
// that same `Vec` is a real, ordinary (not just Stacked-Borrows-flavored)
// borrow-checker conflict -- there is no way around it by index alone.
// `Option::take()` resolves it: physically remove the lookup being
// processed from its slot (leaving a plain `None` there, not a dangling
// borrow) before handing out `&table.lookups`, then put it back
// afterwards. A naive version of this (deliberately *not* what this does)
// would silently break self-reference -- with the lookup missing from the
// list, a name/index scan that includes itself would come up empty, and
// `consolidate_chaining` would treat a real self-reference as an invalid
// lookup and discard it, a genuine output regression. `self_index`/
// `self_name` (the latter cloned *before* the `take`, since it borrows
// from the very value about to be reborrowed mutably) are threaded down
// so `consolidate_chaining` can special-case exactly that slot instead of
// reading it (as `None`) from `lookups`.
pub(crate) fn consolidate_otl_table(glyph_order: Option<&GlyphOrder>, table: Option<&mut OtlTable>, options: &Options) {
    // Every lookup consolidator below reads exactly one thing from the font:
    // its glyph order (checked across `consolidate/otl/` -- nothing else).
    // So this takes `glyph_order`, not the `Font`, which is what lets the
    // caller borrow `font.glyph_order` and `font.gsub`/`.gpos` (disjoint
    // fields) at the same time without a raw pointer.
    let Some(glyph_order) = glyph_order else {
        return;
    };
    let Some(table) = table else {
        return;
    };
    loop {
        for j in 0..table.lookups.len() {
            // A hole here (`None`) means a previous iteration of this
            // same fixed-point loop already punched it -- nothing left
            // to consolidate at this slot.
            let mut current = table.lookups[j].take();
            if let Some(lookup) = current.as_deref_mut() {
                let self_name = lookup.name.clone();
                consolidate_lookup(
                    glyph_order,
                    &table.lookups,
                    j as TableId,
                    &self_name,
                    lookup,
                    options,
                );
            }
            table.lookups[j] = current;
        }
        for feature in table.features.iter_mut().flatten() {
            otl_lookup_ref_list_filter_env(&mut feature.lookups, &table.lookups, |lut| {
                lut.is_some_and(|l| !l.subtables.is_empty())
            });
        }
        for lang in table.languages.iter_mut() {
            // `required_feature` is a single borrowed `Option<FeatureIdx>`,
            // not a list element `otl_feature_ref_list_filter_env` (below)
            // ever touches -- it was set once at parse time and otherwise
            // never revisited. Below, this same pass drops every `Feature`
            // whose `.lookups` is empty from `table.features` (punching a
            // hole where its `Box` used to live); a `required_feature`
            // still pointing at one of those becomes a dangling read the
            // very next time this language is dumped or built.
            // `feature_ref_is_not_empty`'s check (`.lookups.is_empty()`) is
            // applied here too, so a `required_feature` is cleared in the
            // same pass, by the same rule, as every other reference to
            // that feature -- closing a real fuzzer-found use-after-free
            // (heap-use-after-free reading a freed `Feature`'s `name` from
            // `dump_otl`, ASan-confirmed). `feature_at` resolving to
            // `None` (an out-of-range index, never expected here, or a
            // hole punched by an *earlier* iteration of this same loop)
            // is treated the same as "empty": either way, nothing valid to
            // require.
            if let Some(rf) = lang.required_feature {
                let target_empty = crate::table::otl::feature_at(&table.features, rf)
                    .is_none_or(|f| f.lookups.is_empty());
                if target_empty {
                    lang.required_feature = None;
                }
            }
            otl_feature_ref_list_filter_env(&mut lang.features, &table.features, |feat| {
                feat.is_some_and(|f| !f.lookups.is_empty())
            });
        }
        // A hole-preserving `Vec` never shrinks, unlike the old
        // `Vec::retain`-based compaction this replaces -- `punched_lookups`/
        // `punched_features` are the explicit "did this pass change
        // anything" signal the fixed-point loop below now watches instead
        // of `.len()`.
        let punched_lookups =
            otl_lookup_list_punch_holes(&mut table.lookups, |lut| !lut.subtables.is_empty());
        let punched_features =
            otl_feature_list_punch_holes(&mut table.features, |feat| !feat.lookups.is_empty());
        if !punched_lookups && !punched_features {
            break;
        }
    }
}

#[cfg(test)]
mod consolidate_otl_table_tests {
    use super::*;
    use crate::font::model::Font;
    use crate::support::handle::{Handle, HandleState, LookupHandle};
    use crate::table::otl::{
        ChainLookupApplication, ChainingRule, ChainingSubtable, FeatureIdx, LookupIdx, LookupType,
        OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GSUB_CHAINING, Subtable, new_feature, new_language,
        new_lookup,
    };

    fn empty_font_with_glyph_order() -> Box<Font> {
        Box::new(Font {
            subtype: crate::font::model::FontSubtype::Ttf,
            fvar: None,
            head: None,
            hhea: None,
            maxp: None,
            os_2: None,
            hmtx: None,
            post: None,
            vhea: None,
            vmtx: None,
            vorg: None,
            cff: None,
            glyf: None,
            cmap: None,
            name: None,
            meta: None,
            fpgm: None,
            prep: None,
            cvt_: None,
            gasp: None,
            vdmx: None,
            ltsh: None,
            gsub: None,
            gpos: None,
            gdef: None,
            base: None,
            cpal: None,
            colr: None,
            svg: None,
            tsi_01: None,
            tsi_23: None,
            tsi5: None,
            glyph_order: Some(Box::new(GlyphOrder {
                entries: Vec::new(),
                by_gid: Default::default(),
                by_name: Default::default(),
            })),
        })
    }

    // The fuzzer-found, ASan-confirmed bug this pins down: a
    // `LanguageSystem.required_feature` is a lone borrowed `*const Feature`
    // into `table.features` that nothing here used to revisit once set at
    // parse time. When the `Feature` it points at ends up with no valid
    // lookups (every lookup referencing it turned out to have zero usable
    // subtables) and gets dropped from `table.features` by this same
    // consolidation pass, `required_feature` was left dangling -- read
    // later by `dump_otl`/the build path, an actual heap-use-after-
    // free (confirmed via a debug-std ASan build: `AddressSanitizer:
    // heap-use-after-free ... freed by ... otl_feature_list_filter_env ...
    // READ of size 8 ... in dump_otl`). `lang.features` (the *list* of
    // borrowed feature refs) was already correctly pruned in this same
    // pass; `required_feature` (the lone one) was not.
    #[test]
    fn required_feature_pointing_at_a_lookup_with_no_valid_subtables_is_cleared_not_left_dangling() {
        // `LookupIdx(0)`/`FeatureIdx(0)` reference the one lookup/feature
        // slot below directly -- no raw pointers or `unsafe` needed to
        // build this fixture anymore, now that the cross-references are
        // plain indices rather than borrows into a `Box` this test would
        // otherwise have to keep pinned in place.
        let mut table = Box::new(OtlTable {
            lookups: vec![Some(new_lookup())], // subtables empty -- "no valid subtables"
            features: Vec::new(),
            languages: Vec::new(),
        });

        let mut feature = new_feature();
        feature.lookups.push(LookupIdx(0));
        table.features.push(Some(feature));

        let mut lang = new_language();
        lang.required_feature = Some(FeatureIdx(0));
        lang.features.push(FeatureIdx(0));
        table.languages.push(lang);

        let font = empty_font_with_glyph_order();
        let options = Options::default();

        consolidate_otl_table(font.glyph_order.as_deref(), Some(table.as_mut()), &options);

        assert!(table.languages[0].required_feature.is_none());
        // Consolidation now punches holes instead of compacting -- an
        // emptied-out `table.features`/`.lookups` still has one slot each,
        // just `None` rather than removed outright.
        assert!(table.features.iter().all(Option::is_none));
        assert!(table.lookups.iter().all(Option::is_none));
    }

    // `consolidate_otl_table` takes the font's glyph order alone (the only
    // thing any lookup consolidator reads), and returns without touching the
    // table when there is none. `consolidate_font` cannot actually
    // reach it that way today -- it errors out earlier for `glyf` without a
    // glyph order -- so no fixture exercises the guard, which is exactly why
    // it is pinned here: an emptied-out lookup would be punched away below
    // if the early return were ever lost.
    #[test]
    fn without_a_glyph_order_the_otl_table_is_left_untouched() {
        let mut table = Box::new(OtlTable {
            lookups: vec![Some(new_lookup())], // no subtables: would be punched if visited
            features: Vec::new(),
            languages: Vec::new(),
        });
        let options = Options::default();

        consolidate_otl_table(None, Some(table.as_mut()), &options);

        assert!(table.lookups[0].is_some());
    }

    fn self_referencing_chaining_lookup(lookup_type: LookupType, app_lookup: LookupHandle) -> Box<Lookup> {
        let mut lookup = new_lookup();
        lookup.name = b"self_ref_lookup".to_vec();
        lookup.type_0 = lookup_type;
        lookup.subtables.push(Some(Box::new(Subtable::Chaining(
            ChainingSubtable::Canonical(ChainingRule {
                match_count: 0,
                input_begins: 0,
                input_ends: 0,
                match_0: Vec::new(),
                apply: vec![ChainLookupApplication {
                    index: 0,
                    lookup: app_lookup,
                }],
            }),
        ))));
        lookup
    }

    // Stage L-7's own reason for existing: `consolidate_otl_table` now
    // `take()`s the lookup being processed out of `table.lookups` before
    // handing `consolidate_chaining` a shared `&LookupList` (so that
    // shared borrow can't alias the `&mut Subtable` also being threaded
    // through), which means a naive scan for "does lookup k exist" would
    // see the current lookup's own slot as an empty hole. A chaining rule
    // whose one lookup application names its own containing lookup (a
    // real OpenType idiom, e.g. an iterative contextual substitution) is
    // exactly the case that would silently break: without the `self_index`/
    // `self_name` special-casing this test pins down, the self-reference
    // would be misdiagnosed as an invalid lookup and discarded.
    #[test]
    fn chaining_rule_naming_its_own_lookup_by_name_resolves_instead_of_being_invalidated() {
        let lookup = self_referencing_chaining_lookup(
            OTL_TYPE_GSUB_CHAINING,
            Handle::new(HandleState::Name, 0, b"self_ref_lookup".to_vec()),
        );
        let mut table = Box::new(OtlTable {
            lookups: vec![Some(lookup)],
            features: Vec::new(),
            languages: Vec::new(),
        });
        let font = empty_font_with_glyph_order();
        let options = Options::default();

        consolidate_otl_table(font.glyph_order.as_deref(), Some(table.as_mut()), &options);

        let resolved = table.lookups[0]
            .as_deref()
            .expect("the self-referencing lookup itself must survive consolidation");
        let Subtable::Chaining(ChainingSubtable::Canonical(rule)) =
            resolved.subtables[0].as_deref().unwrap()
        else {
            unreachable!()
        };
        assert_eq!(
            rule.apply.len(),
            1,
            "the self-referencing apply entry must not have been dropped as invalid"
        );
        assert_eq!(rule.apply[0].lookup.state, HandleState::Consolidated);
        assert_eq!(rule.apply[0].lookup.index, 0);
        assert_eq!(rule.apply[0].lookup.name, b"self_ref_lookup");
    }

    // Same self-reference case as above, but through the index-based
    // resolution branch (`HandleState::Index`) instead of the name-based
    // one -- both branches independently special-case `self_index`. The
    // self-referencing lookup is deliberately at index 1, not 0: the
    // "unresolvable index" fallback also resets to index 0, so a
    // self-index of 0 would make a broken self-index special case
    // indistinguishable from a correctly-handled one (both end up
    // pointing at index 0) -- this placement is what actually exercises
    // the bug this test exists to catch.
    #[test]
    fn chaining_rule_naming_its_own_lookup_by_index_resolves_instead_of_being_invalidated() {
        let mut lookup = self_referencing_chaining_lookup(
            OTL_TYPE_GPOS_CHAINING,
            Handle::new(HandleState::Index, 1, Vec::new()),
        );
        lookup.subtables[0]
            .as_deref_mut()
            .map(|s| {
                let Subtable::Chaining(ChainingSubtable::Canonical(rule)) = s else {
                    unreachable!()
                };
                rule.apply[0].index = 1;
            })
            .unwrap();
        let mut table = Box::new(OtlTable {
            lookups: vec![Some(new_lookup()), Some(lookup)],
            features: Vec::new(),
            languages: Vec::new(),
        });
        let font = empty_font_with_glyph_order();
        let options = Options::default();

        consolidate_otl_table(font.glyph_order.as_deref(), Some(table.as_mut()), &options);

        let resolved = table.lookups[1]
            .as_deref()
            .expect("the self-referencing lookup itself must survive consolidation");
        let Subtable::Chaining(ChainingSubtable::Canonical(rule)) =
            resolved.subtables[0].as_deref().unwrap()
        else {
            unreachable!()
        };
        assert_eq!(rule.apply.len(), 1);
        assert_eq!(rule.apply[0].lookup.state, HandleState::Consolidated);
        assert_eq!(rule.apply[0].lookup.index, 1);
        assert_eq!(rule.apply[0].lookup.name, b"self_ref_lookup");
    }
}
