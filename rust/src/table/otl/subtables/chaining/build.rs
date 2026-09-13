#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::table::otl::coverage::Coverage;

use crate::support::buffer::Buffer;
use crate::support::primitives::{GlyphClass, TableId};

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkgraph::bk_build_block;
use crate::table::otl::classdef::{ClassDef, build_class_def};
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::chaining::common::{
    chaining_is_classified, chaining_rule_mut_from_const, chaining_ruleset_const,
};
use crate::table::otl::{
    ChainingRule, ChainingRuleSet, ChainingSubtable, Lookup, OTL_TYPE_GPOS_CHAINING,
    OTL_TYPE_GSUB_CHAINING, Subtable, SubtablePtr, subtable_at,
};
pub unsafe fn otfcc_chaining_lookup_is_contextual_lookup(lookup: *const Lookup) -> bool {
    if !((*lookup).type_0 == OTL_TYPE_GPOS_CHAINING || (*lookup).type_0 == OTL_TYPE_GSUB_CHAINING) {
        return false;
    }
    let mut is_contextual: bool = true;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*lookup).subtables.len() {
        let subtable_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, j as usize);
        let Subtable::Chaining(mut_subtable) = &*subtable_ptr else {
            unreachable!()
        };
        let subtable: *const ChainingSubtable = mut_subtable;
        if chaining_is_classified(&*subtable) {
            let ruleset: *const ChainingRuleSet = chaining_ruleset_const(&*subtable);
            let mut k: TableId = 0 as TableId;
            while (k as usize) < (*ruleset).rules.len() {
                let rule: *mut ChainingRule = (&(*ruleset).rules)[k as usize]
                    .as_deref()
                    .expect("chaining rule slot should never be None at build time")
                    as *const ChainingRule
                    as *mut ChainingRule;
                let n_backtrack: TableId = (*rule).input_begins;
                let n_lookahead: TableId = ((*rule).match_count as i32
                    - (*rule).input_ends as i32)
                    as TableId;
                is_contextual = is_contextual as i32 != 0
                    && n_backtrack == 0
                    && n_lookahead == 0;
                k = k.wrapping_add(1);
            }
        } else {
            let rule_0: *mut ChainingRule = chaining_rule_mut_from_const(subtable);
            let n_backtrack_0: TableId = (*rule_0).input_begins;
            let n_lookahead_0: TableId = ((*rule_0).match_count as i32
                - (*rule_0).input_ends as i32)
                as TableId;
            is_contextual = is_contextual as i32 != 0
                && n_backtrack_0 == 0
                && n_lookahead_0 == 0;
        }
        j = j.wrapping_add(1);
    }
    return is_contextual;
}
pub fn otfcc_build_chaining_coverage(_subtable: &ChainingSubtable) -> Buffer {
    let ChainingSubtable::Canonical(rule) = _subtable else {
        unreachable!()
    };
    let n_backtrack: TableId = rule.input_begins;
    let n_input: TableId = (rule.input_ends as i32 - rule.input_begins as i32) as TableId;
    let n_lookahead: TableId = (rule.match_count as i32 - rule.input_ends as i32) as TableId;
    let n_subst: TableId = rule.apply.len() as TableId;
    // The backtrack portion (indices [0, input_begins)) needs to be read in
    // wire order, the reverse of `match_0`'s storage order. Clone just that
    // slice and reverse the clone rather than sorting `match_0` in place
    // (which used to need a const-to-mut cast, unsound now that this
    // function takes a genuine shared `&ChainingSubtable`) -- every read
    // below of a backtrack-region index goes through `backtrack` instead.
    let mut backtrack: Vec<Coverage> = rule.match_0[..rule.input_begins as usize].to_vec();
    backtrack.reverse();
    let mut root: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, 3_u32)]);
    bk_push(
        &mut root,
        vec![bk_int(
            BkCellType::B16,
            (n_backtrack as i32) as u32,
        )],
    );
    let mut j: TableId = 0 as TableId;
    while (j as i32) < rule.input_begins as i32 {
        bk_push(
            &mut root,
            vec![bk_ptr(
                BkCellType::P16,
                bk_new_block_from_buffer(Some(build_coverage(&backtrack[j as usize]))),
            )],
        );
        j = j.wrapping_add(1);
    }
    bk_push(
        &mut root,
        vec![bk_int(
            BkCellType::B16,
            (n_input as i32) as u32,
        )],
    );
    let mut j_0: TableId = rule.input_begins;
    while (j_0 as i32) < rule.input_ends as i32 {
        bk_push(
            &mut root,
            vec![bk_ptr(
                BkCellType::P16,
                bk_new_block_from_buffer(Some(build_coverage(
                    &rule.match_0[j_0 as usize],
                ))),
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    bk_push(
        &mut root,
        vec![bk_int(
            BkCellType::B16,
            (n_lookahead as i32) as u32,
        )],
    );
    let mut j_1: TableId = rule.input_ends;
    while (j_1 as i32) < rule.match_count as i32 {
        bk_push(
            &mut root,
            vec![bk_ptr(
                BkCellType::P16,
                bk_new_block_from_buffer(Some(build_coverage(
                    &rule.match_0[j_1 as usize],
                ))),
            )],
        );
        j_1 = j_1.wrapping_add(1);
    }
    bk_push(
        &mut root,
        vec![bk_int(
            BkCellType::B16,
            (rule.apply.len() as i32) as u32,
        )],
    );
    let mut j_2: TableId = 0 as TableId;
    while (j_2 as i32) < n_subst as i32 {
        bk_push(
            &mut root,
            vec![
                bk_int(
                    BkCellType::B16,
                    (rule.apply[j_2 as usize].index as i32
                        - n_backtrack as i32) as u32,
                ),
                bk_int(
                    BkCellType::B16,
                    (rule.apply[j_2 as usize].lookup.index as i32) as u32,
                ),
            ],
        );
        j_2 = j_2.wrapping_add(1);
    }
    return bk_build_block(root);
}
pub fn otfcc_build_chaining_classes(_subtable: &ChainingSubtable) -> Buffer {
    let (ChainingSubtable::Poly(ruleset) | ChainingSubtable::Classified(ruleset)) = _subtable
    else {
        unreachable!()
    };
    let ic: &ClassDef = ruleset.ic.as_deref().unwrap();
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 2_u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&ic.glyphs))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_class_def(ruleset.bc.as_deref().unwrap()))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_class_def(ic))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_class_def(ruleset.fc.as_deref().unwrap()))),
        ),
        bk_int(
            BkCellType::B16,
            (ic.maxclass as i32 + 1_i32) as u32,
        ),
    ]);
    let mut rcpg: Vec<GlyphClass> =
        vec![0; (ic.maxclass as i32 + 1_i32) as usize];
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < ruleset.rules.len() {
        let rule_j0: &ChainingRule = ruleset.rules[j_0 as usize]
            .as_deref()
            .expect("chaining rule slot should never be None at build time");
        let ib: TableId = rule_j0.input_begins;
        let start_class: TableId = rule_j0.match_0[ib as usize][0].index as TableId;
        if start_class as i32 <= ic.maxclass as i32 {
            rcpg[start_class as usize] = rcpg[start_class as usize].wrapping_add(1);
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: GlyphClass = 0 as GlyphClass;
    while j_1 as i32 <= ic.maxclass as i32 {
        if rcpg[j_1 as usize] != 0 {
            let mut cset: BkBlock = bk_new_block(vec![bk_int(
                BkCellType::B16,
                (rcpg[j_1 as usize] as i32) as u32,
            )]);
            let mut k: TableId = 0 as TableId;
            while (k as usize) < ruleset.rules.len() {
                let rule: &ChainingRule = ruleset.rules[k as usize]
                    .as_deref()
                    .expect("chaining rule slot should never be None at build time");
                let start_class_0: GlyphClass =
                    rule.match_0[rule.input_begins as usize][0].index as GlyphClass;
                if !(start_class_0 as i32 != j_1 as i32) {
                    // Same clone-then-reverse-locally treatment as
                    // `otfcc_build_chaining_coverage` above.
                    let mut backtrack: Vec<Coverage> =
                        rule.match_0[..rule.input_begins as usize].to_vec();
                    backtrack.reverse();
                    let n_backtrack: TableId = rule.input_begins;
                    let n_input: TableId = (rule.input_ends as i32
                        - rule.input_begins as i32)
                        as TableId;
                    let n_lookahead: TableId = (rule.match_count as i32
                        - rule.input_ends as i32)
                        as TableId;
                    let n_subst: TableId = rule.apply.len() as TableId;
                    let mut r: BkBlock = bk_new_block(Vec::new());
                    bk_push(
                        &mut r,
                        vec![bk_int(
                            BkCellType::B16,
                            (n_backtrack as i32) as u32,
                        )],
                    );
                    let mut m: TableId = 0 as TableId;
                    while (m as i32) < rule.input_begins as i32 {
                        bk_push(
                            &mut r,
                            vec![bk_int(
                                BkCellType::B16,
                                (backtrack[m as usize][0].index as i32)
                                    as u32,
                            )],
                        );
                        m = m.wrapping_add(1);
                    }
                    bk_push(
                        &mut r,
                        vec![bk_int(
                            BkCellType::B16,
                            (n_input as i32) as u32,
                        )],
                    );
                    let mut m_0: TableId = (rule.input_begins as i32
                        + 1_i32)
                        as TableId;
                    while (m_0 as i32) < rule.input_ends as i32 {
                        bk_push(
                            &mut r,
                            vec![bk_int(
                                BkCellType::B16,
                                (rule.match_0[m_0 as usize][0].index as i32)
                                    as u32,
                            )],
                        );
                        m_0 = m_0.wrapping_add(1);
                    }
                    bk_push(
                        &mut r,
                        vec![bk_int(
                            BkCellType::B16,
                            (n_lookahead as i32) as u32,
                        )],
                    );
                    let mut m_1: TableId = rule.input_ends;
                    while (m_1 as i32) < rule.match_count as i32 {
                        bk_push(
                            &mut r,
                            vec![bk_int(
                                BkCellType::B16,
                                (rule.match_0[m_1 as usize][0].index as i32)
                                    as u32,
                            )],
                        );
                        m_1 = m_1.wrapping_add(1);
                    }
                    bk_push(
                        &mut r,
                        vec![bk_int(
                            BkCellType::B16,
                            (n_subst as i32) as u32,
                        )],
                    );
                    let mut m_2: TableId = 0 as TableId;
                    while (m_2 as i32) < n_subst as i32 {
                        bk_push(
                            &mut r,
                            vec![
                                bk_int(
                                    BkCellType::B16,
                                    (rule.apply[m_2 as usize].index as i32
                                        - n_backtrack as i32)
                                        as u32,
                                ),
                                bk_int(
                                    BkCellType::B16,
                                    (rule.apply[m_2 as usize].lookup.index
                                        as i32)
                                        as u32,
                                ),
                            ],
                        );
                        m_2 = m_2.wrapping_add(1);
                    }
                    bk_push(&mut cset, vec![bk_ptr(BkCellType::P16, Some(r))]);
                }
                k = k.wrapping_add(1);
            }
            bk_push(&mut root, vec![bk_ptr(BkCellType::P16, Some(cset))]);
        } else {
            bk_push(&mut root, vec![bk_ptr(BkCellType::P16, None)]);
        }
        j_1 = j_1.wrapping_add(1);
    }
    return bk_build_block(root);
}
pub fn otfcc_build_chaining(_subtable: &ChainingSubtable) -> Buffer {
    if chaining_is_classified(_subtable) {
        return otfcc_build_chaining_classes(_subtable);
    } else {
        return otfcc_build_chaining_coverage(_subtable);
    };
}
pub fn otfcc_build_contextual_coverage(_subtable: &ChainingSubtable) -> Buffer {
    let ChainingSubtable::Canonical(rule) = _subtable else {
        unreachable!()
    };
    let n_input: TableId = (rule.input_ends as i32 - rule.input_begins as i32) as TableId;
    let n_subst: TableId = rule.apply.len() as TableId;
    // Unlike `otfcc_build_chaining_coverage`, this function never reads any
    // backtrack-region index (the `j` loop below starts at `input_begins`,
    // not 0) -- the equivalent `reverse_backtracks` call the C-shaped code
    // made here had no observable effect on this function's output and is
    // simply not needed, rather than needing the clone-then-reverse
    // treatment `otfcc_build_chaining_coverage` needs.
    let mut root: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, 3_u32)]);
    bk_push(
        &mut root,
        vec![bk_int(
            BkCellType::B16,
            (n_input as i32) as u32,
        )],
    );
    bk_push(
        &mut root,
        vec![bk_int(
            BkCellType::B16,
            (n_subst as i32) as u32,
        )],
    );
    let mut j: TableId = rule.input_begins;
    while (j as i32) < rule.input_ends as i32 {
        bk_push(
            &mut root,
            vec![bk_ptr(
                BkCellType::P16,
                bk_new_block_from_buffer(Some(build_coverage(
                    &rule.match_0[j as usize],
                ))),
            )],
        );
        j = j.wrapping_add(1);
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as i32) < n_subst as i32 {
        bk_push(
            &mut root,
            vec![
                bk_int(
                    BkCellType::B16,
                    (rule.apply[j_0 as usize].index as i32) as u32,
                ),
                bk_int(
                    BkCellType::B16,
                    (rule.apply[j_0 as usize].lookup.index as i32) as u32,
                ),
            ],
        );
        j_0 = j_0.wrapping_add(1);
    }
    return bk_build_block(root);
}
pub fn otfcc_build_contextual_classes(_subtable: &ChainingSubtable) -> Buffer {
    let (ChainingSubtable::Poly(ruleset) | ChainingSubtable::Classified(ruleset)) = _subtable
    else {
        unreachable!()
    };
    let ic: &ClassDef = ruleset.ic.as_deref().unwrap();
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 2_u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&ic.glyphs))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_class_def(ic))),
        ),
        bk_int(
            BkCellType::B16,
            (ic.maxclass as i32 + 1_i32) as u32,
        ),
    ]);
    let mut rcpg: Vec<GlyphClass> =
        vec![0; (ic.maxclass as i32 + 1_i32) as usize];
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < ruleset.rules.len() {
        let rule_j0: &ChainingRule = ruleset.rules[j_0 as usize]
            .as_deref()
            .expect("chaining rule slot should never be None at build time");
        let ib: TableId = rule_j0.input_begins;
        let start_class: TableId = rule_j0.match_0[ib as usize][0].index as TableId;
        if start_class as i32 <= ic.maxclass as i32 {
            rcpg[start_class as usize] = rcpg[start_class as usize].wrapping_add(1);
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: GlyphClass = 0 as GlyphClass;
    while j_1 as i32 <= ic.maxclass as i32 {
        if rcpg[j_1 as usize] != 0 {
            let mut cset: BkBlock = bk_new_block(vec![bk_int(
                BkCellType::B16,
                (rcpg[j_1 as usize] as i32) as u32,
            )]);
            let mut k: TableId = 0 as TableId;
            while (k as usize) < ruleset.rules.len() {
                let rule: &ChainingRule = ruleset.rules[k as usize]
                    .as_deref()
                    .expect("chaining rule slot should never be None at build time");
                let start_class_0: GlyphClass =
                    rule.match_0[rule.input_begins as usize][0].index as GlyphClass;
                if !(start_class_0 as i32 != j_1 as i32) {
                    // Same "no observable effect" reasoning as
                    // `otfcc_build_contextual_coverage` -- the `m` loop
                    // below starts at `input_begins + 1`, never reading a
                    // backtrack-region index, so the reversal this rule
                    // used to get is dropped rather than reproduced.
                    let n_input: TableId = (rule.input_ends as i32
                        - rule.input_begins as i32)
                        as TableId;
                    let n_subst: TableId = rule.apply.len() as TableId;
                    let mut r: BkBlock = bk_new_block(Vec::new());
                    bk_push(
                        &mut r,
                        vec![bk_int(
                            BkCellType::B16,
                            (n_input as i32) as u32,
                        )],
                    );
                    bk_push(
                        &mut r,
                        vec![bk_int(
                            BkCellType::B16,
                            (n_subst as i32) as u32,
                        )],
                    );
                    let mut m: TableId = (rule.input_begins as i32
                        + 1_i32)
                        as TableId;
                    while (m as i32) < rule.input_ends as i32 {
                        bk_push(
                            &mut r,
                            vec![bk_int(
                                BkCellType::B16,
                                (rule.match_0[m as usize][0].index as i32)
                                    as u32,
                            )],
                        );
                        m = m.wrapping_add(1);
                    }
                    let mut m_0: TableId = 0 as TableId;
                    while (m_0 as i32) < n_subst as i32 {
                        bk_push(
                            &mut r,
                            vec![
                                bk_int(
                                    BkCellType::B16,
                                    (rule.apply[m_0 as usize].index as i32)
                                        as u32,
                                ),
                                bk_int(
                                    BkCellType::B16,
                                    (rule.apply[m_0 as usize].lookup.index
                                        as i32)
                                        as u32,
                                ),
                            ],
                        );
                        m_0 = m_0.wrapping_add(1);
                    }
                    bk_push(&mut cset, vec![bk_ptr(BkCellType::P16, Some(r))]);
                }
                k = k.wrapping_add(1);
            }
            bk_push(&mut root, vec![bk_ptr(BkCellType::P16, Some(cset))]);
        } else {
            bk_push(&mut root, vec![bk_ptr(BkCellType::P16, None)]);
        }
        j_1 = j_1.wrapping_add(1);
    }
    return bk_build_block(root);
}
pub fn otfcc_build_contextual(_subtable: &ChainingSubtable) -> Buffer {
    if chaining_is_classified(_subtable) {
        return otfcc_build_contextual_classes(_subtable);
    } else {
        return otfcc_build_contextual_coverage(_subtable);
    };
}
