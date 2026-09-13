use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::handle::{Handle, HandleState, LookupHandle, handle_name_eq_bytes};
use crate::table::otl::coverage::shrink_coverage;

use crate::support::options::Options;
use crate::support::primitives::{GlyphId, TableId};

use crate::font::caryll_font::Font;

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::table::otl::subtables::chaining::common::{chaining_is_canonical, chaining_rule_mut};
use crate::table::otl::{ChainingRule, OtlTable, Subtable};

/// See `Options::consolidate_warning_budget`'s own doc comment: bounds the
/// total "invalid lookup reference" warnings this function will log across
/// one whole font, since the per-subtable/per-rule caps upstream
/// (`chaining/read.rs`) bound each factor individually but not their
/// product. Generous: real fonts essentially never hit even one such
/// warning, let alone tens of thousands.
pub(crate) const CONSOLIDATE_WARNING_BUDGET: u32 = 10_000;

pub(crate) fn consolidate_chaining(
    font: &Font,
    // Stays a raw pointer, never a `&OtlTable`: `table.lookups[j]` (the
    // lookup this subtable belongs to) is exactly the `&mut Subtable`
    // this function already mutates through `rule` below, so a blanket
    // shared `&OtlTable` covering that same memory alongside `rule`'s
    // live `&mut` would be a genuine Stacked-Borrows violation (confirmed
    // by miri). Each read below is a narrow, one-off `unsafe {}` bridge
    // instead (`vqs_compare` pattern) -- for any index `k != j` this is
    // trivially sound (a different `Box<Lookup>` allocation entirely);
    // for the self-referencing `k == j` case (a chaining rule naming its
    // own lookup) this mirrors the original c2rust raw-pointer code's
    // permissiveness exactly, rather than introducing a new restriction.
    table: *const OtlTable,
    _subtable: &mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::Chaining(subtable) = _subtable else {
        unreachable!()
    };
    if !chaining_is_canonical(subtable) {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"[Consolidate] Ignoring non-canonical chaining subtable."),
        );
        return false;
    }
    // `chaining_rule_mut` is a safe fn but still returns a raw pointer (see
    // its own doc comment) -- this is the one narrow bridge this function
    // needs, matching the `vqs_compare` pattern used throughout this
    // migration.
    let rule: &mut ChainingRule = unsafe { &mut *chaining_rule_mut(subtable) };
    // Guaranteed `Some`: `consolidate_otl` (and hence every caller that
    // reaches here) only ever runs when `glyf` is present, and
    // `otfcc_consolidate_font` always populates `glyph_order` before
    // that, whenever `glyf` is present.
    let glyph_order = font.glyph_order.as_deref().unwrap();
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
            let mut k: TableId = 0 as TableId;
            while (k as usize) < unsafe { (*table).lookups.len() } {
                // A `None` slot here is a hole an earlier iteration of the
                // caller's own fixed-point loop already punched (see
                // `consolidate_otl_table`) -- nothing to match against.
                let matched = unsafe { (&(*table).lookups)[k as usize].as_deref() }.is_some_and(
                    |lookup| !lookup.subtables.is_empty() && handle_name_eq_bytes(&app.lookup.name, &lookup.name),
                );
                if matched {
                    found_lookup = true;
                    app.lookup = Handle {
                        state: HandleState::Consolidated,
                        index: k as GlyphId,
                        name: unsafe { (&(*table).lookups)[k as usize].as_deref() }
                            .unwrap()
                            .name
                            .clone(),
                    } as LookupHandle;
                }
                k = k.wrapping_add(1);
            }
            if !found_lookup && !app.lookup.name.is_empty() {
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
                        &mut *options.logger.borrow_mut(),
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
            // to resolve against.
            let lookups = unsafe { &(*table).lookups };
            let target = lookups.get(app.lookup.index as usize).and_then(Option::as_deref);
            if target.is_none() {
                let budget = options.consolidate_warning_budget.get();
                if budget > 0 {
                    options.consolidate_warning_budget.set(budget - 1);
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
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
            let name = lookups
                .get(idx as usize)
                .and_then(Option::as_deref)
                .map_or_else(Vec::new, |lookup| lookup.name.clone());
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
