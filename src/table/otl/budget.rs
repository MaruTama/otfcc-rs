//! Table-wide work budgets for reading one GSUB, GPOS or GDEF table.
//!
//! Each table read creates its own `OtlReadBudget` and passes it down by
//! `&mut`, so concurrent reads (for example two unit tests on different
//! threads) never drain each other's budgets.
//!
//! Every limit keeps its value, and its doc comment explaining why, next to
//! the code that spends it:
//! - `coverage_entries`: `coverage::MAX_TOTAL_COVERAGE_ENTRY_BUILDS_PER_TABLE`
//! - `class_zero_glyphs`, `class_coverage_calls`, `rules`, `rule_positions`:
//!   `chaining::read`'s `MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS`,
//!   `MAX_TOTAL_CLASS_COVERAGE_CALLS`, `MAX_TOTAL_RULES_PER_TABLE` and
//!   `MAX_TOTAL_RULE_POSITIONS_PER_TABLE`
//! - `mark_attach_anchors`: `gpos_mark_common::MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE`
//! - `pair_grid_cells`: `gpos_pair::MAX_TOTAL_GPOS_PAIR_GRID_CELLS_PER_TABLE`
//! - `feature_param_characters`:
//!   `feature_params::MAX_TOTAL_FEATURE_PARAM_CHARACTERS_PER_TABLE`
//! - `subtables`, `feature_lookup_refs`: `read`'s
//!   `MAX_TOTAL_SUBTABLES_PER_TABLE` and `MAX_TOTAL_FEATURE_LOOKUP_REFS_PER_TABLE`
//!
//! A limit exists to stop crafted input from taking unbounded time or
//! memory, and no specification has one, so reaching it means data of a
//! possibly valid font is dropped. The first time each limit is reached in
//! a table, a warning says so (`OtlReadBudget::reached`).

use crate::table::otl::coverage::MAX_TOTAL_COVERAGE_ENTRY_BUILDS_PER_TABLE;
use crate::table::otl::feature_params::MAX_TOTAL_FEATURE_PARAM_CHARACTERS_PER_TABLE;
use crate::table::otl::read::{
    MAX_TOTAL_FEATURE_LOOKUP_REFS_PER_TABLE, MAX_TOTAL_SUBTABLES_PER_TABLE,
};
use crate::table::otl::subtables::chaining::read::{
    MAX_TOTAL_CLASS_COVERAGE_CALLS, MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS,
    MAX_TOTAL_RULE_POSITIONS_PER_TABLE, MAX_TOTAL_RULES_PER_TABLE,
};
use crate::table::otl::subtables::gpos_mark_common::MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE;
use crate::table::otl::subtables::gpos_pair::MAX_TOTAL_GPOS_PAIR_GRID_CELLS_PER_TABLE;

/// A limit on how much of one table is read. Each has a warning text: what
/// there is more of than otfcc reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Limit {
    CoverageEntries,
    ClassZeroGlyphs,
    ClassCoverageCalls,
    Rules,
    AllRulePositions,
    MarkAttachAnchors,
    PairGridCells,
    Subtables,
    FeatureLookupRefs,
    FeatureParamCharacters,
    /// `gpos_pair`'s per-subtable `MAX_TOTAL_GPOS_PAIR_CLASS_CELLS`.
    PairSubtableCells,
    /// `chaining::read`'s `MAX_POSITIONS_PER_RULE`.
    RulePositions,
    /// `chaining::read`'s `MAX_APPLY_PER_RULE`.
    RuleApplications,
}

impl Limit {
    fn what(self) -> &'static str {
        match self {
            Limit::CoverageEntries => "coverage glyphs",
            Limit::ClassZeroGlyphs => "glyphs in contextual class 0",
            Limit::ClassCoverageCalls => "contextual class references",
            Limit::Rules => "contextual rules",
            Limit::AllRulePositions => "glyph positions in contextual rules",
            Limit::MarkAttachAnchors => "mark attachment anchors",
            Limit::PairGridCells => "pair positioning cells",
            Limit::Subtables => "lookup subtables",
            Limit::FeatureLookupRefs => "lookup references from features",
            Limit::FeatureParamCharacters => "characters in character variant parameters",
            Limit::PairSubtableCells => "pair positioning cells in one subtable",
            Limit::RulePositions => "glyph positions in one contextual rule",
            Limit::RuleApplications => "lookup applications in one contextual rule",
        }
    }
    fn bit(self) -> u16 {
        1 << self as u16
    }
}

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
    /// Glyph positions of those rules.
    pub(crate) rule_positions: u32,
    /// Anchor slots allocated by the mark-attachment readers.
    pub(crate) mark_attach_anchors: u32,
    /// Cells of the pair-positioning value grids (`gpos_pair`).
    pub(crate) pair_grid_cells: u32,
    /// Lookup subtables, summed over every lookup of the table.
    pub(crate) subtables: u32,
    /// Lookup indices in the features' lookup lists, summed over every
    /// feature.
    pub(crate) feature_lookup_refs: u32,
    /// Characters of `cv` feature parameters.
    pub(crate) feature_param_characters: u32,
    /// The table's tag, for the warnings.
    pub(crate) table: &'static str,
    /// The limits already warned about, one bit per [`Limit`].
    pub(crate) warned: u16,
}

impl OtlReadBudget {
    /// A full budget, for the start of one table read.
    pub fn new() -> Self {
        Self::for_table("OTL")
    }

    /// A full budget for reading the table `table` (`"GSUB"`, ...).
    pub fn for_table(table: &'static str) -> Self {
        OtlReadBudget {
            coverage_entries: MAX_TOTAL_COVERAGE_ENTRY_BUILDS_PER_TABLE,
            class_zero_glyphs: MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS,
            class_coverage_calls: MAX_TOTAL_CLASS_COVERAGE_CALLS,
            rules: MAX_TOTAL_RULES_PER_TABLE,
            rule_positions: MAX_TOTAL_RULE_POSITIONS_PER_TABLE,
            mark_attach_anchors: MAX_TOTAL_MARK_ATTACH_ANCHORS_PER_TABLE,
            pair_grid_cells: MAX_TOTAL_GPOS_PAIR_GRID_CELLS_PER_TABLE,
            subtables: MAX_TOTAL_SUBTABLES_PER_TABLE,
            feature_lookup_refs: MAX_TOTAL_FEATURE_LOOKUP_REFS_PER_TABLE,
            feature_param_characters: MAX_TOTAL_FEATURE_PARAM_CHARACTERS_PER_TABLE,
            table,
            warned: 0,
        }
    }

    /// Records that `limit` stopped part of the table from being read, and
    /// warns about it the first time.
    pub(crate) fn reached(&mut self, limit: Limit) {
        if self.warned & limit.bit() == 0 {
            self.warned |= limit.bit();
            tracing::warn!(
                "[otl] {} has more {} than otfcc reads; the rest are ignored.\n",
                self.table,
                limit.what()
            );
        }
    }

    /// Whether the limit `limit` has been reached.
    #[cfg(test)]
    pub(crate) fn has_reached(&self, limit: Limit) -> bool {
        self.warned & limit.bit() != 0
    }

    /// The allowance `limit` counts down, for the limits that have one.
    fn field(&mut self, limit: Limit) -> &mut u32 {
        match limit {
            Limit::CoverageEntries => &mut self.coverage_entries,
            Limit::ClassZeroGlyphs => &mut self.class_zero_glyphs,
            Limit::ClassCoverageCalls => &mut self.class_coverage_calls,
            Limit::Rules => &mut self.rules,
            Limit::AllRulePositions => &mut self.rule_positions,
            Limit::MarkAttachAnchors => &mut self.mark_attach_anchors,
            Limit::PairGridCells => &mut self.pair_grid_cells,
            Limit::Subtables => &mut self.subtables,
            Limit::FeatureLookupRefs => &mut self.feature_lookup_refs,
            Limit::FeatureParamCharacters => &mut self.feature_param_characters,
            Limit::PairSubtableCells | Limit::RulePositions | Limit::RuleApplications => {
                unreachable!("{limit:?} is a fixed cap, not an allowance")
            }
        }
    }

    /// Spends `n` units of `limit` if all of them are available. Returns
    /// `false`, spends nothing and warns otherwise, so "the budget refused"
    /// always means "this call changed nothing".
    fn try_spend(&mut self, limit: Limit, n: u32) -> bool {
        let field = self.field(limit);
        match field.checked_sub(n) {
            Some(left) => {
                *field = left;
                true
            }
            None => {
                self.reached(limit);
                false
            }
        }
    }

    /// Spends as many as possible of `n` units of `limit`, and returns how
    /// many it spent; warns when that is fewer than `n`.
    fn spend_up_to(&mut self, limit: Limit, n: u32) -> u32 {
        let field = self.field(limit);
        let granted = n.min(*field);
        *field -= granted;
        if granted < n {
            self.reached(limit);
        }
        granted
    }

    /// Spends one coverage entry; `false` once the table's allowance is gone.
    pub(crate) fn take_coverage_entry(&mut self) -> bool {
        self.try_spend(Limit::CoverageEntries, 1)
    }

    /// Spends one `class_coverage` call; `false` once the allowance is gone.
    pub(crate) fn take_class_coverage_call(&mut self) -> bool {
        self.try_spend(Limit::ClassCoverageCalls, 1)
    }

    /// Spends up to `n` subtables of one lookup, and returns how many of
    /// them may be read.
    pub(crate) fn take_subtables(&mut self, n: u16) -> u16 {
        // At most `n`, so the result fits in a `u16`.
        self.spend_up_to(Limit::Subtables, u32::from(n)) as u16
    }

    /// Spends the `n` characters of one feature's parameters if all are
    /// available.
    pub(crate) fn try_spend_feature_param_characters(&mut self, n: u32) -> bool {
        self.try_spend(Limit::FeatureParamCharacters, n)
    }

    /// Spends one lookup index of a feature; `false` once the allowance is
    /// gone.
    pub(crate) fn take_feature_lookup_ref(&mut self) -> bool {
        self.try_spend(Limit::FeatureLookupRefs, 1)
    }

    /// Whether `class_coverage` may scan another glyph; warns the first
    /// time it may not.
    pub(crate) fn class_zero_left(&mut self) -> bool {
        if self.class_zero_glyphs > 0 {
            true
        } else {
            self.reached(Limit::ClassZeroGlyphs);
            false
        }
    }

    /// Charges one scanned glyph to `class_coverage`'s allowance. Callers
    /// check `class_zero_left` first; an empty allowance stays at zero.
    pub(crate) fn charge_class_zero(&mut self) {
        self.class_zero_glyphs = self.class_zero_glyphs.saturating_sub(1);
    }

    /// Spends one contextual or chaining rule; `false` once the allowance is
    /// gone.
    pub(crate) fn take_rule(&mut self) -> bool {
        self.try_spend(Limit::Rules, 1)
    }

    /// Spends the `n` glyph positions of one rule if all are available;
    /// `false` once they are not.
    pub(crate) fn try_spend_rule_positions(&mut self, n: u32) -> bool {
        self.try_spend(Limit::AllRulePositions, n)
    }

    /// Spends `n` anchor slots if all of them are available.
    pub(crate) fn try_spend_mark_attach_anchors(&mut self, n: usize) -> bool {
        let Ok(n) = u32::try_from(n) else {
            self.reached(Limit::MarkAttachAnchors);
            return false;
        };
        self.try_spend(Limit::MarkAttachAnchors, n)
    }

    /// Spends `n` cells of the pair-positioning grids if all are available.
    pub(crate) fn try_spend_pair_grid_cells(&mut self, n: usize) -> bool {
        let Ok(n) = u32::try_from(n) else {
            self.reached(Limit::PairGridCells);
            return false;
        };
        self.try_spend(Limit::PairGridCells, n)
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
        let mut budget = OtlReadBudget {
            mark_attach_anchors: 3,
            ..OtlReadBudget::new()
        };
        assert!(!budget.try_spend_mark_attach_anchors(4));
        assert_eq!(budget.mark_attach_anchors, 3);
        assert!(budget.try_spend_mark_attach_anchors(3));
        assert_eq!(budget.mark_attach_anchors, 0);
        assert!(!budget.try_spend_mark_attach_anchors(1));
    }

    #[test]
    fn pair_grid_cells_are_a_table_wide_allowance() {
        let mut budget = OtlReadBudget {
            pair_grid_cells: 10,
            ..OtlReadBudget::new()
        };
        assert!(budget.try_spend_pair_grid_cells(6));
        assert!(!budget.try_spend_pair_grid_cells(5));
        assert_eq!(budget.pair_grid_cells, 4);
        assert!(budget.try_spend_pair_grid_cells(4));
    }

    #[test]
    fn single_unit_budgets_stop_at_zero() {
        let mut budget = OtlReadBudget {
            rules: 1,
            coverage_entries: 1,
            class_coverage_calls: 1,
            ..OtlReadBudget::new()
        };
        assert!(budget.take_rule() && !budget.take_rule());
        assert!(budget.take_coverage_entry() && !budget.take_coverage_entry());
        assert!(budget.take_class_coverage_call() && !budget.take_class_coverage_call());
        assert_eq!(
            (
                budget.rules,
                budget.coverage_entries,
                budget.class_coverage_calls
            ),
            (0, 0, 0)
        );
    }
}
