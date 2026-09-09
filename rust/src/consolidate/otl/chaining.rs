use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::handle::{
    Handle, HandleState, LookupHandle, handle_name_eq_bytes, otfcc_handle_dispose,
};
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
    let mut j: TableId = 0 as TableId;
    while (j as i32) < rule.match_count as i32 {
        fontop_consolidate_coverage(glyph_order, &mut rule.match_0[j as usize], options);
        shrink_coverage(&mut rule.match_0[j as usize], true);
        possible = possible as i32 != 0 && rule.match_0[j as usize].len() as i32 > 0_i32;
        j = j.wrapping_add(1);
    }
    if rule.input_begins as i32 > rule.match_count as i32 {
        rule.input_begins = rule.match_count;
    }
    if rule.input_ends as i32 > rule.match_count as i32 {
        rule.input_ends = rule.match_count;
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < rule.apply.len() {
        let mut found_lookup: bool = false;
        if !rule.apply[j_0 as usize].lookup.name.is_empty() {
            let mut k: TableId = 0 as TableId;
            while (k as usize) < unsafe { (*table).lookups.len() } {
                // Every element is a `Box<Lookup>` now, never null, so the
                // old null check is gone -- everything else here is plain
                // field access through the `Box`, unchanged.
                let matched = unsafe {
                    !(&(*table).lookups)[k as usize].subtables.is_empty()
                        && handle_name_eq_bytes(
                            &rule.apply[j_0 as usize].lookup.name,
                            &(&(*table).lookups)[k as usize].name,
                        )
                };
                if matched {
                    found_lookup = true;
                    rule.apply[j_0 as usize].lookup = Handle {
                        state: HandleState::Consolidated,
                        index: k as GlyphId,
                        name: unsafe { (&(*table).lookups)[k as usize].name.clone() },
                    } as LookupHandle;
                }
                k = k.wrapping_add(1);
            }
            if !found_lookup && !rule.apply[j_0 as usize].lookup.name.is_empty() {
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
                            &rule.apply[j_0 as usize].lookup.name,
                            b". This lookup application is ignored.",
                        ),
                    );
                }
                otfcc_handle_dispose(&mut rule.apply[j_0 as usize].lookup);
            }
        } else if rule.apply[j_0 as usize].lookup.state == HandleState::Index {
            if rule.apply[j_0 as usize].lookup.index as usize >= unsafe { (*table).lookups.len() }
            {
                let budget = options.consolidate_warning_budget.get();
                if budget > 0 {
                    options.consolidate_warning_budget.set(budget - 1);
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[Consolidate] Quoting an invalid lookup #",
                            rule.apply[j_0 as usize].lookup.index as i32,
                            b".",
                        ),
                    );
                }
                rule.apply[j_0 as usize].lookup.index = 0 as GlyphId;
            }
            let idx = rule.apply[j_0 as usize].lookup.index;
            rule.apply[j_0 as usize].lookup = Handle {
                state: HandleState::Consolidated,
                index: idx,
                name: unsafe { (&(*table).lookups)[idx as usize].name.clone() },
            } as LookupHandle;
        }
        j_0 = j_0.wrapping_add(1);
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
