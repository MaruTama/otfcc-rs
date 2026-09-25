use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::handle::{Handle, HandleState, LookupHandle, handle_name_eq_bytes};
use crate::table::otl::coverage::shrink_coverage;

use crate::support::options::Options;
use crate::support::primitives::{GlyphId, TableId};

use crate::support::glyph_order::GlyphOrder;

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::table::otl::subtables::chaining::common::{chaining_is_canonical, chaining_rule_mut};
use crate::table::otl::{ChainingRule, LookupList, Subtable};

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
        logger_log_sds(
            &mut options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"[Consolidate] Ignoring non-canonical chaining subtable."),
        );
        return false;
    }
    let rule: &mut ChainingRule = chaining_rule_mut(subtable);
    // Guaranteed `Some`: `consolidate_otl` (and hence every caller that
    // reaches here) only ever runs when `glyf` is present, and
    // `otfcc_consolidate_font` always populates `glyph_order` before
    // that, whenever `glyf` is present.
    let mut possible: bool = true;
    let match_count = rule.match_count as usize;
    for cov in rule.match_0.iter_mut().take(match_count) {
        fontop_consolidate_coverage(glyph_order, cov, options);
        shrink_coverage(cov, true);
        possible = possible && !cov.is_empty();
    }
    if rule.input_begins as i32 > rule.match_count as i32 {
        rule.input_begins = rule.match_count;
    }
    if rule.input_ends as i32 > rule.match_count as i32 {
        rule.input_ends = rule.match_count;
    }
    for app in rule.apply.iter_mut() {
        let mut found_lookup: bool = false;
        if !app.lookup.name.is_empty() {
            // Deliberately no early exit on the first match: a later `k`
            // matching the same name overwrites `app.lookup` again, same
            // as the original's own unconditional overwrite -- "last
            // matching index wins" for a font with duplicate lookup names,
            // preserved exactly rather than "fixed" to first-match.
            for (k, slot) in lookups.iter().enumerate() {
                let k = k as TableId;
                // `k == self_index` is answered from `self_name` instead
                // of `slot` (a `None` hole left by the caller's `take()`,
                // see this function's own doc comment) -- see also why
                // "exists" doesn't need checking for that case: reaching
                // this call at all means `self_index`'s own subtable list
                // still has at least one slot (`__declare_otl_consolidation`
                // bails out before ever calling in here otherwise, and the
                // `Vec`'s length -- as opposed to its slots' contents --
                // never shrinks mid-pass, only at that function's trailing
                // `retain()`), so "self" is unconditionally present.
                let (exists, name) = if k == self_index {
                    (true, self_name)
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
                    app.lookup = Handle {
                        state: HandleState::Consolidated,
                        index: k,
                        name: name.to_vec(),
                    } as LookupHandle;
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
                let budget = options.consolidate_warning_budget.get();
                if budget > 0 {
                    options.consolidate_warning_budget.set(budget - 1);
                    logger_log_sds(
                        &mut options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[Consolidate] Quoting an invalid lookup ",
                            &app.lookup.name,
                            b". This lookup application is ignored.",
                        ),
                    );
                }
                app.lookup = Handle::default();
            }
        } else if app.lookup.state == HandleState::Index {
            // Invalid now covers both "out of range" (unchanged) and
            // "in range but a hole" (new -- see the `None`-slot comment
            // above): either way there is no real `Lookup` at this index
            // to resolve against. Same self-reference treatment as the
            // name-based branch above: `idx == self_index` never actually
            // reads `lookups[self_index]` (a `None` hole), answering from
            // `self_name` instead.
            let target_exists = if app.lookup.index == self_index {
                true
            } else {
                lookups
                    .get(app.lookup.index as usize)
                    .and_then(Option::as_deref)
                    .is_some()
            };
            if !target_exists {
                let budget = options.consolidate_warning_budget.get();
                if budget > 0 {
                    options.consolidate_warning_budget.set(budget - 1);
                    logger_log_sds(
                        &mut options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[Consolidate] Quoting an invalid lookup #",
                            app.lookup.index as i32,
                            b".",
                        ),
                    );
                }
                app.lookup.index = 0 as GlyphId;
            }
            let idx = app.lookup.index;
            let name = if idx == self_index {
                self_name.to_vec()
            } else {
                lookups
                    .get(idx as usize)
                    .and_then(Option::as_deref)
                    .map_or_else(Vec::new, |lookup| lookup.name.clone())
            };
            app.lookup = Handle {
                state: HandleState::Consolidated,
                index: idx,
                name,
            } as LookupHandle;
        }
    }
    if !rule.apply.is_empty() {
        // Was a manual compact-in-place loop over `apply_count` before
        // `.apply` became a `Vec` -- `retain` is the direct translation.
        rule.apply.retain(|app| !app.lookup.name.is_empty());
        if rule.apply.is_empty() {
            return true;
        }
    }
    !possible
}
