#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::handle::{
    Handle, HandleState, LookupHandle, handle_name_eq_bytes, otfcc_handle_dispose,
};
use crate::table::otl::coverage::{Coverage, shrink_coverage};

use crate::support::options::Options;
use crate::support::primitives::{GlyphId, TableId};

use crate::font::caryll_font::Font;

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::table::otl::subtables::chaining::common::{chaining_is_canonical, chaining_rule_mut};
use crate::table::otl::{ChainingRule, ChainingSubtable, OtlTable, Subtable};

/// See `Options::consolidate_warning_budget`'s own doc comment: bounds the
/// total "invalid lookup reference" warnings this function will log across
/// one whole font, since the per-subtable/per-rule caps upstream
/// (`chaining/read.rs`) bound each factor individually but not their
/// product. Generous: real fonts essentially never hit even one such
/// warning, let alone tens of thousands.
pub(crate) const CONSOLIDATE_WARNING_BUDGET: u32 = 10_000;

pub unsafe fn consolidate_chaining(
    font: *mut Font,
    table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::Chaining(mut_subtable) = &mut *_subtable else {
        unreachable!()
    };
    let subtable: *mut ChainingSubtable = mut_subtable;
    if !chaining_is_canonical(subtable) {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"[Consolidate] Ignoring non-canonical chaining subtable."),
        );
        return false;
    }
    let rule: *mut ChainingRule = chaining_rule_mut(subtable);
    let mut possible: bool = true;
    let mut j: TableId = 0 as TableId;
    while (j as i32) < (*rule).match_count as i32 {
        fontop_consolidate_coverage(
            font,
            &mut (&mut (*rule).match_0)[j as usize] as *mut Coverage,
            options,
        );
        shrink_coverage(
            &mut (&mut (*rule).match_0)[j as usize] as *mut Coverage,
            true,
        );
        possible = possible as i32 != 0
            && (&(*rule).match_0)[j as usize].len() as i32 > 0_i32;
        j = j.wrapping_add(1);
    }
    if (*rule).input_begins as i32 > (*rule).match_count as i32 {
        (*rule).input_begins = (*rule).match_count;
    }
    if (*rule).input_ends as i32 > (*rule).match_count as i32 {
        (*rule).input_ends = (*rule).match_count;
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < (*rule).apply.len() {
        let mut found_lookup: bool = false;
        let h: *mut LookupHandle = &raw mut (&mut (*rule).apply)[j_0 as usize].lookup;
        if !(*h).name.is_empty() {
            let mut k: TableId = 0 as TableId;
            while (k as usize) < (*table).lookups.len() {
                // Every element is a `Box<Lookup>` now, never null, so the
                // old null check is gone -- everything else here is plain
                // field access through the `Box`, unchanged.
                if !((*(&(*table).lookups)[k as usize]).subtables.is_empty()) {
                    if handle_name_eq_bytes(&(*h).name, &(*(&(*table).lookups)[k as usize]).name) {
                        found_lookup = true;
                        *h = Handle {
                            state: HandleState::Consolidated,
                            index: k as GlyphId,
                            name: (*(&(*table).lookups)[k as usize]).name.clone(),
                        } as LookupHandle;
                    }
                }
                k = k.wrapping_add(1);
            }
            if !found_lookup && !(&(*rule).apply)[j_0 as usize].lookup.name.is_empty() {
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
                            &(&(*rule).apply)[j_0 as usize].lookup.name,
                            b". This lookup application is ignored.",
                        ),
                    );
                }
                otfcc_handle_dispose(&raw mut (&mut (*rule).apply)[j_0 as usize].lookup);
            }
        } else if (*h).state == HandleState::Index {
            if (*h).index as usize >= (*table).lookups.len() {
                let budget = options.consolidate_warning_budget.get();
                if budget > 0 {
                    options.consolidate_warning_budget.set(budget - 1);
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[Consolidate] Quoting an invalid lookup #",
                            (*h).index as i32,
                            b".",
                        ),
                    );
                }
                (*h).index = 0 as GlyphId;
            }
            let idx = (*h).index;
            *h = Handle {
                state: HandleState::Consolidated,
                index: idx,
                name: (*(&(*table).lookups)[idx as usize]).name.clone(),
            } as LookupHandle;
        }
        j_0 = j_0.wrapping_add(1);
    }
    if !(*rule).apply.is_empty() {
        // Was a manual compact-in-place loop over `apply_count` before
        // `.apply` became a `Vec` -- `retain` is the direct translation.
        (*rule).apply.retain(|app| !app.lookup.name.is_empty());
        if (*rule).apply.is_empty() {
            return true;
        }
    }
    return !possible;
}
