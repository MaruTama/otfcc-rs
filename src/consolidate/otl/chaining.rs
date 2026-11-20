use crate::logger::ByteStr;
use crate::support::handle::{Handle, HandleState, LookupHandle, handle_name_eq_bytes};
use crate::table::otl::coverage::shrink_coverage;

use crate::support::options::Options;
use crate::support::primitives::{GlyphId, TableId};

use crate::support::glyph_order::GlyphOrder;

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::table::otl::subtables::chaining::common::{chaining_is_canonical, chaining_rule_mut};
use crate::table::otl::{ChainLookupApplication, ChainingRule, LookupList, Subtable};

/// See `Options::consolidate_warning_budget`'s own doc comment: bounds the
/// total "invalid lookup reference" warnings this function will log across
/// one whole font, since the per-subtable/per-rule caps upstream
/// (`chaining/read.rs`) bound each factor individually but not their
/// product. Generous: real fonts essentially never hit even one such
/// warning, let alone tens of thousands.
pub(crate) const CONSOLIDATE_WARNING_BUDGET: u32 = 10_000;

pub(crate) fn consolidate_chaining(
    glyph_order: &GlyphOrder,
    // `lookups` is the *whole* table's lookup list, but the caller
    // (`consolidate_otl_table`) physically removed `lookups[self_index]`
    // (via `Option::take()`) before handing this out -- otherwise a
    // shared `&LookupList` spanning the entire `Vec` would alias the
    // `&mut Lookup`/`&mut Subtable` the caller is holding into that exact
    // slot, a genuine borrow-checker conflict, not just a Stacked-Borrows
    // one. That means `lookups[self_index]` reads back as `None` here --
    // never actually read for that reason: every comparison against the
    // *current* lookup (a chaining rule naming its own containing lookup,
    // `k == self_index`) is answered from `self_name` instead of from
    // `lookups`, which is exactly the data that slot would have held.
    lookups: &LookupList,
    self_index: TableId,
    self_name: &[u8],
    _subtable: &mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::Chaining(subtable) = _subtable else {
        unreachable!()
    };
    if !chaining_is_canonical(subtable) {
        tracing::warn!("[Consolidate] Ignoring non-canonical chaining subtable.");
        return false;
    }
    let rule: &mut ChainingRule = chaining_rule_mut(subtable);
    let possible = consolidate_match_sequence(glyph_order, rule);
    let self_lookup = SelfLookup {
        index: self_index,
        name: self_name,
    };
    for app in rule.apply.iter_mut() {
        if !app.lookup.name.is_empty() {
            resolve_apply_by_name(app, lookups, &self_lookup, options);
        } else if app.lookup.state == HandleState::Index {
            resolve_apply_by_index(app, lookups, &self_lookup, options);
        }
    }
    if !rule.apply.is_empty() {
        // Drop applications whose lookup did not resolve.
        rule.apply.retain(|app| !app.lookup.name.is_empty());
        if rule.apply.is_empty() {
            return true;
        }
    }
    !possible
}

/// The lookup that owns the rule being consolidated. Its slot in the lookup
/// list is a `None` hole (see `consolidate_chaining`'s `lookups` comment), so
/// anything that refers to it is answered from this instead.
struct SelfLookup<'a> {
    index: TableId,
    name: &'a [u8],
}

/// Consolidates the match (backtrack/input/lookahead) coverages and clamps the
/// input range to the match count. Returns `false` when some coverage ended up
/// empty, i.e. the rule can never match.
fn consolidate_match_sequence(glyph_order: &GlyphOrder, rule: &mut ChainingRule) -> bool {
    let mut possible: bool = true;
    let match_count = rule.match_count as usize;
    for cov in rule.sequence.iter_mut().take(match_count) {
        fontop_consolidate_coverage(glyph_order, cov);
        shrink_coverage(cov, true);
        possible = possible && !cov.is_empty();
    }
    if i32::from(rule.input_begins) > i32::from(rule.match_count) {
        rule.input_begins = rule.match_count;
    }
    if i32::from(rule.input_ends) > i32::from(rule.match_count) {
        rule.input_ends = rule.match_count;
    }
    possible
}

/// Spends one unit of the whole-font warning budget; `true` when the caller
/// may still log. See `CONSOLIDATE_WARNING_BUDGET`.
fn take_warning_budget(options: &Options) -> bool {
    let budget = options.consolidate_warning_budget.get();
    if budget > 0 {
        options.consolidate_warning_budget.set(budget - 1);
        true
    } else {
        false
    }
}

/// Resolves an apply entry that names its lookup. An unresolvable name drops
/// the application (its handle is reset, and `retain` removes it afterwards).
fn resolve_apply_by_name(
    app: &mut ChainLookupApplication,
    lookups: &LookupList,
    self_lookup: &SelfLookup<'_>,
    options: &Options,
) {
    let mut found_lookup: bool = false;
    // Deliberately no early exit on the first match: a later `k`
    // matching the same name overwrites `app.lookup` again -- "last
    // matching index wins" for a font with duplicate lookup names,
    // as upstream otfcc does.
    for (k, slot) in lookups.iter().enumerate() {
        let k = k as TableId;
        // `k == self_index` is answered from `self_name` instead of `slot`.
        // "exists" needn't be checked for that case: reaching this call at
        // all means the own subtable list still has at least one slot
        // (`consolidate_lookup` bails out before calling in here otherwise,
        // and the `Vec`'s length never shrinks mid-pass, only at that
        // function's trailing `retain()`), so "self" is unconditionally
        // present.
        let (exists, name) = if k == self_lookup.index {
            (true, self_lookup.name)
        } else {
            // A `None` slot here is a hole an earlier iteration of
            // the caller's own fixed-point loop already punched
            // (see `consolidate_otl_table`) -- nothing to match
            // against.
            match slot.as_deref() {
                Some(lookup) if !lookup.subtables.is_empty() => (true, lookup.name.as_slice()),
                _ => (false, [].as_slice()),
            }
        };
        if exists && handle_name_eq_bytes(&app.lookup.name, name) {
            found_lookup = true;
            app.lookup = Handle::new(HandleState::Consolidated, k, name.to_vec()) as LookupHandle;
        }
    }
    if !found_lookup {
        // See `CONSOLIDATE_WARNING_BUDGET`'s doc comment: a font
        // whose rules apply thousands of unresolvable lookups can
        // still reach this point despite the per-rule/per-subtable/
        // per-lookup caps upstream, since those caps bound each
        // factor individually, not their product -- this budget
        // bounds the actual cost (heap-allocating log calls), while
        // the dispose below (which the warning exists to explain)
        // still always runs.
        if take_warning_budget(options) {
            tracing::warn!(
                "[Consolidate] Quoting an invalid lookup {}. This lookup application is ignored.",
                ByteStr(&app.lookup.name)
            );
        }
        app.lookup = Handle::default();
    }
}

/// Resolves an apply entry that refers to its lookup by index and fills in the
/// lookup's name. An invalid index (out of range, or a hole) falls back to
/// index 0.
fn resolve_apply_by_index(
    app: &mut ChainLookupApplication,
    lookups: &LookupList,
    self_lookup: &SelfLookup<'_>,
    options: &Options,
) {
    // Invalid covers both "out of range" and "in range but a hole": either
    // way there is no real `Lookup` at this index to resolve against.
    // `idx == self_index` never reads `lookups[self_index]` (a `None` hole);
    // it is answered from `self_name` instead.
    let target_exists = if app.lookup.index == self_lookup.index {
        true
    } else {
        lookups
            .get(app.lookup.index as usize)
            .and_then(Option::as_deref)
            .is_some()
    };
    if !target_exists {
        if take_warning_budget(options) {
            tracing::warn!(
                "[Consolidate] Quoting an invalid lookup #{}.",
                i32::from(app.lookup.index)
            );
        }
        app.lookup.index = 0 as GlyphId;
    }
    let idx = app.lookup.index;
    let name = if idx == self_lookup.index {
        self_lookup.name.to_vec()
    } else {
        lookups
            .get(idx as usize)
            .and_then(Option::as_deref)
            .map_or_else(Vec::new, |lookup| lookup.name.clone())
    };
    app.lookup = Handle::new(HandleState::Consolidated, idx, name) as LookupHandle;
}
