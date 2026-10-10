//! Table-wide work budgets for reading one GSUB, GPOS or GDEF table.
//!
//! Each table read creates its own `OtlReadBudget` and passes it down by
//! `&mut`, so concurrent reads (for example two unit tests on different
//! threads) never drain each other's budgets.
//!
//! Every limit keeps its value, and its doc comment explaining why, next to
//! the code that spends it:
//! - `coverage_entries`: `coverage::MAX_TOTAL_COVERAGE_ENTRY_BUILDS_PER_TABLE`
//! - `class_zero_glyphs`, `class_coverage_calls`, `rules`:
//!   `chaining::read`'s `MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS`,
//!   `MAX_TOTAL_CLASS_COVERAGE_CALLS` and `MAX_TOTAL_RULES_PER_TABLE`
//! - `mark_attach_anchors`: `gpos_common::MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE`

use crate::table::otl::coverage::MAX_TOTAL_COVERAGE_ENTRY_BUILDS_PER_TABLE;
use crate::table::otl::subtables::chaining::read::{
    MAX_TOTAL_CLASS_COVERAGE_CALLS, MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS, MAX_TOTAL_RULES_PER_TABLE,
};
use crate::table::otl::subtables::gpos_common::MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE;

/// The remaining allowance of each kind of work for the table being read.
/// Every field counts down from its limit and never wraps below zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtlReadBudget {
    /// Glyphs added while building a `Coverage` (`read_coverage`).
    pub(crate) coverage_entries: u32,
    /// Glyphs scanned by `class_coverage`.
    pub(crate) class_zero_glyphs: u32,
    /// Calls to `class_coverage` itself.
    pub(crate) class_coverage_calls: u32,
    /// Contextual and chaining rules built.
    pub(crate) rules: u32,
    /// Anchor slots allocated by the mark-attachment readers.
    pub(crate) mark_attach_anchors: u32,
}

impl OtlReadBudget {
    /// A full budget, for the start of one table read.
    pub fn new() -> Self {
        OtlReadBudget {
            coverage_entries: MAX_TOTAL_COVERAGE_ENTRY_BUILDS_PER_TABLE,
            class_zero_glyphs: MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS,
            class_coverage_calls: MAX_TOTAL_CLASS_COVERAGE_CALLS,
            rules: MAX_TOTAL_RULES_PER_TABLE,
            mark_attach_anchors: MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE,
        }
    }

    /// Spends `n` units of `field` if all of them are available. Returns
    /// `false` and spends nothing otherwise, so "the budget refused" always
    /// means "this call changed nothing".
    fn try_spend(field: &mut u32, n: u32) -> bool {
        match field.checked_sub(n) {
            Some(left) => {
                *field = left;
                true
            }
            None => false,
        }
    }

    /// Spends one coverage entry; `false` once the table's allowance is gone.
    pub(crate) fn take_coverage_entry(&mut self) -> bool {
        Self::try_spend(&mut self.coverage_entries, 1)
    }

    /// Spends one `class_coverage` call; `false` once the allowance is gone.
    pub(crate) fn take_class_coverage_call(&mut self) -> bool {
        Self::try_spend(&mut self.class_coverage_calls, 1)
    }

    /// Whether `class_coverage` may scan another glyph.
    pub(crate) fn class_zero_left(&self) -> bool {
        self.class_zero_glyphs > 0
    }

    /// Charges one scanned glyph to `class_coverage`'s allowance. Callers
    /// check `class_zero_left` first; an empty allowance stays at zero.
    pub(crate) fn charge_class_zero(&mut self) {
        self.class_zero_glyphs = self.class_zero_glyphs.saturating_sub(1);
    }

    /// Spends one contextual or chaining rule; `false` once the allowance is
    /// gone.
    pub(crate) fn take_rule(&mut self) -> bool {
        Self::try_spend(&mut self.rules, 1)
    }

    /// Spends `n` anchor slots if all of them are available.
    pub(crate) fn try_spend_mark_attach_anchors(&mut self, n: usize) -> bool {
        let Ok(n) = u32::try_from(n) else { return false };
        Self::try_spend(&mut self.mark_attach_anchors, n)
    }
}

impl Default for OtlReadBudget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_refused_spend_changes_nothing() {
        let mut budget = OtlReadBudget { mark_attach_anchors: 3, ..OtlReadBudget::new() };
        assert!(!budget.try_spend_mark_attach_anchors(4));
        assert_eq!(budget.mark_attach_anchors, 3);
        assert!(budget.try_spend_mark_attach_anchors(3));
        assert_eq!(budget.mark_attach_anchors, 0);
        assert!(!budget.try_spend_mark_attach_anchors(1));
    }

    #[test]
    fn single_unit_budgets_stop_at_zero() {
        let mut budget = OtlReadBudget { rules: 1, coverage_entries: 1, class_coverage_calls: 1, ..OtlReadBudget::new() };
        assert!(budget.take_rule() && !budget.take_rule());
        assert!(budget.take_coverage_entry() && !budget.take_coverage_entry());
        assert!(budget.take_class_coverage_call() && !budget.take_class_coverage_call());
        assert_eq!((budget.rules, budget.coverage_entries, budget.class_coverage_calls), (0, 0, 0));
    }
}
