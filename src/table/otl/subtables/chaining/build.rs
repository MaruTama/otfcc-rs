use crate::table::otl::coverage::Coverage;

use crate::support::primitives::{GlyphClass, TableId};
use otfcc_binary::Buffer;

use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::otl::classdef::{ClassDef, build_class_def};
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::chaining::common::{
    chaining_is_classified, chaining_rule_const, chaining_ruleset_const, chaining_subtable_ref,
};
use crate::table::otl::{
    ChainingRule, ChainingRuleSet, ChainingSubtable, Lookup, OTL_TYPE_GPOS_CHAINING,
    OTL_TYPE_GSUB_CHAINING,
};
use otfcc_binary::bk::block::bk_new_block_from_buffer;
use otfcc_binary::bk::graph::bk_build_block;
/// Whether a chaining lookup has no backtrack and no lookahead in any rule,
/// so it can be written as a plain contextual lookup (GSUB 5, GPOS 7).
pub fn chaining_lookup_is_contextual_lookup(lookup: &Lookup) -> bool {
    if !(lookup.lookup_type == OTL_TYPE_GPOS_CHAINING
        || lookup.lookup_type == OTL_TYPE_GSUB_CHAINING)
    {
        return false;
    }
    let mut is_contextual = true;
    for slot in lookup.subtables.iter() {
        let subtable = chaining_subtable_ref(slot);
        if chaining_is_classified(subtable) {
            let ruleset = chaining_ruleset_const(subtable);
            for rule_slot in ruleset.rules.iter() {
                let rule = rule_slot
                    .as_deref()
                    .expect("chaining rule slot should never be None at build time");
                let n_backtrack: TableId = rule.input_begins;
                let n_lookahead: TableId =
                    (i32::from(rule.match_count) - i32::from(rule.input_ends)) as TableId;
                is_contextual = is_contextual && n_backtrack == 0 && n_lookahead == 0;
            }
        } else {
            let rule = chaining_rule_const(subtable);
            let n_backtrack: TableId = rule.input_begins;
            let n_lookahead: TableId =
                (i32::from(rule.match_count) - i32::from(rule.input_ends)) as TableId;
            is_contextual = is_contextual && n_backtrack == 0 && n_lookahead == 0;
        }
    }
    is_contextual
}
/// Writes one rule as a chaining subtable of format 3 (one coverage per
/// position).
pub fn build_chaining_coverage(_subtable: &ChainingSubtable) -> Buffer {
    let ChainingSubtable::Canonical(rule) = _subtable else {
        unreachable!()
    };
    let n_backtrack: TableId = rule.input_begins;
    let n_input: TableId = (i32::from(rule.input_ends) - i32::from(rule.input_begins)) as TableId;
    let n_lookahead: TableId =
        (i32::from(rule.match_count) - i32::from(rule.input_ends)) as TableId;
    // The backtrack portion (indices [0, input_begins)) is written in wire
    // order, the reverse of `sequence`'s storage order. Clone just that
    // slice and reverse the clone; every read below of a backtrack-region
    // index goes through `backtrack` instead.
    let mut backtrack: Vec<Coverage> = rule.sequence[..rule.input_begins as usize].to_vec();
    backtrack.reverse();
    let mut root: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, 3)]);
    bk_push(
        &mut root,
        vec![bk_int(BkCellType::B16, i32::from(n_backtrack) as u32)],
    );
    for cov in backtrack.iter() {
        bk_push(
            &mut root,
            vec![bk_ptr(
                BkCellType::P16,
                bk_new_block_from_buffer(Some(build_coverage(cov))),
            )],
        );
    }
    bk_push(
        &mut root,
        vec![bk_int(BkCellType::B16, i32::from(n_input) as u32)],
    );
    for cov in &rule.sequence[rule.input_begins as usize..rule.input_ends as usize] {
        bk_push(
            &mut root,
            vec![bk_ptr(
                BkCellType::P16,
                bk_new_block_from_buffer(Some(build_coverage(cov))),
            )],
        );
    }
    bk_push(
        &mut root,
        vec![bk_int(BkCellType::B16, i32::from(n_lookahead) as u32)],
    );
    for cov in &rule.sequence[rule.input_ends as usize..rule.match_count as usize] {
        bk_push(
            &mut root,
            vec![bk_ptr(
                BkCellType::P16,
                bk_new_block_from_buffer(Some(build_coverage(cov))),
            )],
        );
    }
    bk_push(
        &mut root,
        vec![bk_int(BkCellType::B16, (rule.apply.len() as i32) as u32)],
    );
    for app in rule.apply.iter() {
        bk_push(
            &mut root,
            vec![
                bk_int(
                    BkCellType::B16,
                    (i32::from(app.index) - i32::from(n_backtrack)) as u32,
                ),
                bk_int(BkCellType::B16, u32::from(app.lookup.index)),
            ],
        );
    }
    bk_build_block(root)
}
const CLASSIFIED_HAS_ALL_CLASS_DEFS: &str =
    "only the classifier builds a Classified ruleset, and it sets bc, ic and fc";
/// Writes a class-based rule set as a chaining subtable of format 2.
pub fn build_chaining_classes(_subtable: &ChainingSubtable) -> Buffer {
    let (ChainingSubtable::Poly(ruleset) | ChainingSubtable::Classified(ruleset)) = _subtable
    else {
        unreachable!()
    };
    let ic: &ClassDef = ruleset.ic.as_deref().expect(CLASSIFIED_HAS_ALL_CLASS_DEFS);
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 2),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&ic.glyphs))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_class_def(
                ruleset.bc.as_deref().expect(CLASSIFIED_HAS_ALL_CLASS_DEFS),
            ))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_class_def(ic))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_class_def(
                ruleset.fc.as_deref().expect(CLASSIFIED_HAS_ALL_CLASS_DEFS),
            ))),
        ),
        bk_int(BkCellType::B16, (i32::from(ic.maxclass) + 1) as u32),
    ]);
    let rcpg = count_rules_per_start_class(ruleset, ic.maxclass);
    for (j_1, &count) in rcpg.iter().enumerate() {
        if count != 0 {
            let mut cset: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, u32::from(count))]);
            for slot in ruleset.rules.iter() {
                let rule: &ChainingRule = slot
                    .as_deref()
                    .expect("chaining rule slot should never be None at build time");
                let class_of_first_input: GlyphClass =
                    rule.sequence[rule.input_begins as usize][0].index as GlyphClass;
                if class_of_first_input as usize == j_1 {
                    bk_push(
                        &mut cset,
                        vec![bk_ptr(BkCellType::P16, Some(build_class_rule(rule)))],
                    );
                }
            }
            bk_push(&mut root, vec![bk_ptr(BkCellType::P16, Some(cset))]);
        } else {
            bk_push(&mut root, vec![bk_ptr(BkCellType::P16, None)]);
        }
    }
    bk_build_block(root)
}

/// For each input class, how many rules start with it (the number of rules in
/// that class's rule set). A start class above `maxclass` is not counted.
fn count_rules_per_start_class(ruleset: &ChainingRuleSet, maxclass: GlyphClass) -> Vec<GlyphClass> {
    let mut rcpg: Vec<GlyphClass> = vec![0; (i32::from(maxclass) + 1) as usize];
    for slot in ruleset.rules.iter() {
        let rule_j0: &ChainingRule = slot
            .as_deref()
            .expect("chaining rule slot should never be None at build time");
        let ib: TableId = rule_j0.input_begins;
        let start_class: TableId = rule_j0.sequence[ib as usize][0].index as TableId;
        if i32::from(start_class) <= i32::from(maxclass) {
            rcpg[start_class as usize] = rcpg[start_class as usize].wrapping_add(1);
        }
    }
    rcpg
}

/// One class sequence rule: backtrack classes (nearest first), the input
/// classes after the first, the lookahead classes, then the lookup records.
/// The first input class is not stored: it selects the rule set.
fn build_class_rule(rule: &ChainingRule) -> BkBlock {
    // Same clone-then-reverse-locally treatment as `build_chaining_coverage`
    // above.
    let mut backtrack: Vec<Coverage> = rule.sequence[..rule.input_begins as usize].to_vec();
    backtrack.reverse();
    let n_backtrack: TableId = rule.input_begins;
    let n_input: TableId = (i32::from(rule.input_ends) - i32::from(rule.input_begins)) as TableId;
    let n_lookahead: TableId =
        (i32::from(rule.match_count) - i32::from(rule.input_ends)) as TableId;
    let mut r: BkBlock = bk_new_block(Vec::new());
    bk_push(
        &mut r,
        vec![bk_int(BkCellType::B16, i32::from(n_backtrack) as u32)],
    );
    for cov in backtrack.iter() {
        bk_push(
            &mut r,
            vec![bk_int(BkCellType::B16, u32::from(cov[0].index))],
        );
    }
    bk_push(
        &mut r,
        vec![bk_int(BkCellType::B16, i32::from(n_input) as u32)],
    );
    let m_0_start = (i32::from(rule.input_begins) + 1) as usize;
    for cov in &rule.sequence[m_0_start..rule.input_ends as usize] {
        bk_push(
            &mut r,
            vec![bk_int(BkCellType::B16, u32::from(cov[0].index))],
        );
    }
    bk_push(
        &mut r,
        vec![bk_int(BkCellType::B16, i32::from(n_lookahead) as u32)],
    );
    for cov in &rule.sequence[rule.input_ends as usize..rule.match_count as usize] {
        bk_push(
            &mut r,
            vec![bk_int(BkCellType::B16, u32::from(cov[0].index))],
        );
    }
    bk_push(
        &mut r,
        vec![bk_int(BkCellType::B16, (rule.apply.len() as i32) as u32)],
    );
    for app in rule.apply.iter() {
        bk_push(
            &mut r,
            vec![
                bk_int(
                    BkCellType::B16,
                    (i32::from(app.index) - i32::from(n_backtrack)) as u32,
                ),
                bk_int(BkCellType::B16, u32::from(app.lookup.index)),
            ],
        );
    }
    r
}
/// Writes a chaining subtable: format 2 for a class-based rule set, format 3
/// for a single rule.
pub fn build_chaining(_subtable: &ChainingSubtable) -> Buffer {
    if chaining_is_classified(_subtable) {
        build_chaining_classes(_subtable)
    } else {
        build_chaining_coverage(_subtable)
    }
}
/// Writes one rule as a contextual subtable of format 3.
pub fn build_contextual_coverage(_subtable: &ChainingSubtable) -> Buffer {
    let ChainingSubtable::Canonical(rule) = _subtable else {
        unreachable!()
    };
    let n_input: TableId = (i32::from(rule.input_ends) - i32::from(rule.input_begins)) as TableId;
    let n_subst: TableId = rule.apply.len() as TableId;
    // Unlike `build_chaining_coverage`, this function never reads any
    // backtrack-region index (the `j` loop below starts at `input_begins`,
    // not 0) -- the equivalent `reverse_backtracks` call the C-shaped code
    // made here had no observable effect on this function's output and is
    // simply not needed, rather than needing the clone-then-reverse
    // treatment `build_chaining_coverage` needs.
    let mut root: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, 3)]);
    bk_push(
        &mut root,
        vec![bk_int(BkCellType::B16, i32::from(n_input) as u32)],
    );
    bk_push(
        &mut root,
        vec![bk_int(BkCellType::B16, i32::from(n_subst) as u32)],
    );
    for cov in &rule.sequence[rule.input_begins as usize..rule.input_ends as usize] {
        bk_push(
            &mut root,
            vec![bk_ptr(
                BkCellType::P16,
                bk_new_block_from_buffer(Some(build_coverage(cov))),
            )],
        );
    }
    for app in rule.apply.iter() {
        bk_push(
            &mut root,
            vec![
                bk_int(BkCellType::B16, u32::from(app.index)),
                bk_int(BkCellType::B16, u32::from(app.lookup.index)),
            ],
        );
    }
    bk_build_block(root)
}
/// Writes a class-based rule set as a contextual subtable of format 2.
pub fn build_contextual_classes(_subtable: &ChainingSubtable) -> Buffer {
    let (ChainingSubtable::Poly(ruleset) | ChainingSubtable::Classified(ruleset)) = _subtable
    else {
        unreachable!()
    };
    let ic: &ClassDef = ruleset.ic.as_deref().expect(CLASSIFIED_HAS_ALL_CLASS_DEFS);
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 2),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&ic.glyphs))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_class_def(ic))),
        ),
        bk_int(BkCellType::B16, (i32::from(ic.maxclass) + 1) as u32),
    ]);
    let mut rcpg: Vec<GlyphClass> = vec![0; (i32::from(ic.maxclass) + 1) as usize];
    for slot in ruleset.rules.iter() {
        let rule_j0: &ChainingRule = slot
            .as_deref()
            .expect("chaining rule slot should never be None at build time");
        let ib: TableId = rule_j0.input_begins;
        let start_class: TableId = rule_j0.sequence[ib as usize][0].index as TableId;
        if i32::from(start_class) <= i32::from(ic.maxclass) {
            rcpg[start_class as usize] = rcpg[start_class as usize].wrapping_add(1);
        }
    }
    for (j_1, &count) in rcpg.iter().enumerate() {
        if count != 0 {
            let mut cset: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, u32::from(count))]);
            for slot in ruleset.rules.iter() {
                let rule: &ChainingRule = slot
                    .as_deref()
                    .expect("chaining rule slot should never be None at build time");
                let class_of_first_input: GlyphClass =
                    rule.sequence[rule.input_begins as usize][0].index as GlyphClass;
                if class_of_first_input as usize == j_1 {
                    // Same "no observable effect" reasoning as
                    // `build_contextual_coverage` -- the loop below
                    // starts at `input_begins + 1`, never reading a
                    // backtrack-region index, so no reversal is needed.
                    let n_input: TableId =
                        (i32::from(rule.input_ends) - i32::from(rule.input_begins)) as TableId;
                    let mut r: BkBlock = bk_new_block(Vec::new());
                    bk_push(
                        &mut r,
                        vec![bk_int(BkCellType::B16, i32::from(n_input) as u32)],
                    );
                    bk_push(
                        &mut r,
                        vec![bk_int(BkCellType::B16, (rule.apply.len() as i32) as u32)],
                    );
                    let m_start = (i32::from(rule.input_begins) + 1) as usize;
                    for cov in &rule.sequence[m_start..rule.input_ends as usize] {
                        bk_push(
                            &mut r,
                            vec![bk_int(BkCellType::B16, u32::from(cov[0].index))],
                        );
                    }
                    for app in rule.apply.iter() {
                        bk_push(
                            &mut r,
                            vec![
                                bk_int(BkCellType::B16, u32::from(app.index)),
                                bk_int(BkCellType::B16, u32::from(app.lookup.index)),
                            ],
                        );
                    }
                    bk_push(&mut cset, vec![bk_ptr(BkCellType::P16, Some(r))]);
                }
            }
            bk_push(&mut root, vec![bk_ptr(BkCellType::P16, Some(cset))]);
        } else {
            bk_push(&mut root, vec![bk_ptr(BkCellType::P16, None)]);
        }
    }
    bk_build_block(root)
}
/// Writes a contextual subtable: format 2 for a class-based rule set, format
/// 3 for a single rule.
pub fn build_contextual(_subtable: &ChainingSubtable) -> Buffer {
    if chaining_is_classified(_subtable) {
        build_contextual_classes(_subtable)
    } else {
        build_contextual_coverage(_subtable)
    }
}
