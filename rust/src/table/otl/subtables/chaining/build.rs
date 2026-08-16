#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::table::otl::coverage::Coverage;


use crate::support::buffer::{Buffer};
use crate::support::primitives::{GlyphClass, TableId};


use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};

use crate::table::otl::{ChainingRule, ChainingRuleSet, Lookup, Subtable, SubtablePtr, subtable_at, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GSUB_CHAINING, ChainingSubtable};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::classdef::{ClassDef, build_class_def};
use crate::table::otl::coverage::{build_coverage};
use crate::table::otl::subtables::chaining::common::{chaining_is_classified, chaining_ruleset_const, chaining_rule_mut_from_const};
pub unsafe fn otfcc_chaining_lookup_is_contextual_lookup(
    mut lookup: *const Lookup,
) -> bool {
    if !((*lookup).type_0 == OTL_TYPE_GPOS_CHAINING
        || (*lookup).type_0 == OTL_TYPE_GSUB_CHAINING)
    {
        return false;
    }
    let mut is_contextual: bool = true;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*lookup).subtables.len() {
        let subtable_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, j as usize);
        let Subtable::Chaining(mut_subtable) = &*subtable_ptr else { unreachable!() };
        let subtable: *const ChainingSubtable = mut_subtable;
        if chaining_is_classified(subtable)
        {
            let ruleset: *const ChainingRuleSet = chaining_ruleset_const(subtable);
            let mut k: TableId = 0 as TableId;
            while (k as usize) < (*ruleset).rules.len() {
                let mut rule: *mut ChainingRule = (&(*ruleset).rules)[k as usize]
                    .as_deref()
                    .expect("chaining rule slot should never be None at build time")
                    as *const ChainingRule as *mut ChainingRule;
                let mut n_backtrack: TableId = (*rule).input_begins;
                let mut n_lookahead: TableId = ((*rule).match_count as ::core::ffi::c_int
                    - (*rule).input_ends as ::core::ffi::c_int)
                    as TableId;
                is_contextual =
                    is_contextual as ::core::ffi::c_int != 0 && n_backtrack == 0 && n_lookahead == 0;
                k = k.wrapping_add(1);
            }
        } else {
            let mut rule_0: *mut ChainingRule = chaining_rule_mut_from_const(subtable);
            let mut n_backtrack_0: TableId = (*rule_0).input_begins;
            let mut n_lookahead_0: TableId = ((*rule_0).match_count as ::core::ffi::c_int
                - (*rule_0).input_ends as ::core::ffi::c_int)
                as TableId;
            is_contextual =
                is_contextual as ::core::ffi::c_int != 0 && n_backtrack_0 == 0 && n_lookahead_0 == 0;
        }
        j = j.wrapping_add(1);
    }
    return is_contextual;
}
pub unsafe fn otfcc_build_chaining_coverage(
    mut _subtable: *const ChainingSubtable,
) -> *mut Buffer {
    let subtable: *const ChainingSubtable = _subtable;
    let mut rule: *mut ChainingRule = chaining_rule_mut_from_const(subtable);
    let mut n_backtrack: TableId = (*rule).input_begins;
    let mut n_input: TableId = ((*rule).input_ends as ::core::ffi::c_int
        - (*rule).input_begins as ::core::ffi::c_int) as TableId;
    let mut n_lookahead: TableId = ((*rule).match_count as ::core::ffi::c_int
        - (*rule).input_ends as ::core::ffi::c_int) as TableId;
    let mut n_subst: TableId = (*rule).apply.len() as TableId;
    reverse_backtracks(rule);
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 3 as u32)]);
    bk_push(root, &[bk_int(BkCellType::B16, (n_backtrack as ::core::ffi::c_int) as u32)]);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*rule).input_begins as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_coverage(
                &(&(*rule).match_0)[j as usize] as *const Coverage,
            )))]);
        j = j.wrapping_add(1);
    }
    bk_push(root, &[bk_int(BkCellType::B16, (n_input as ::core::ffi::c_int) as u32)]);
    let mut j_0: TableId = (*rule).input_begins;
    while (j_0 as ::core::ffi::c_int) < (*rule).input_ends as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_coverage(
                &(&(*rule).match_0)[j_0 as usize] as *const Coverage,
            )))]);
        j_0 = j_0.wrapping_add(1);
    }
    bk_push(root, &[bk_int(BkCellType::B16, (n_lookahead as ::core::ffi::c_int) as u32)]);
    let mut j_1: TableId = (*rule).input_ends;
    while (j_1 as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_coverage(
                &(&(*rule).match_0)[j_1 as usize] as *const Coverage,
            )))]);
        j_1 = j_1.wrapping_add(1);
    }
    bk_push(root, &[bk_int(BkCellType::B16, ((*rule).apply.len() as ::core::ffi::c_int) as u32)]);
    let mut j_2: TableId = 0 as TableId;
    while (j_2 as ::core::ffi::c_int) < n_subst as ::core::ffi::c_int {
        bk_push(root, &[bk_int(BkCellType::B16, ((&(*rule).apply)[j_2 as usize].index as ::core::ffi::c_int
                - n_backtrack as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((&(*rule).apply)[j_2 as usize].lookup.index as ::core::ffi::c_int) as u32)]);
        j_2 = j_2.wrapping_add(1);
    }
    return bk_build_block(root);
}
pub unsafe fn otfcc_build_chaining_classes(
    mut _subtable: *const ChainingSubtable,
) -> *mut Buffer {
    let subtable: *const ChainingSubtable = _subtable;
    let ruleset: *const ChainingRuleSet = chaining_ruleset_const(subtable);
    // `.ic` is reached through a `*const ChainingRuleSet` but still needs a
    // `*mut ClassDef` at the one `&raw mut (*ic).glyphs` site below -- the
    // same const-to-mut cast the original C-shaped code already did.
    let ic: *mut ClassDef = (*ruleset).ic.as_deref().unwrap() as *const ClassDef as *mut ClassDef;
    let coverage: *mut Coverage = &raw mut (*ic).glyphs;
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 2 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_coverage(
            coverage,
        ))), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_class_def(
            (*ruleset).bc.as_deref().unwrap(),
        ))), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_class_def(
            ic,
        ))), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_class_def(
            (*ruleset).fc.as_deref().unwrap(),
        ))), bk_int(BkCellType::B16, ((*ic).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as u32)]);
    let mut rcpg: Vec<GlyphClass> = vec![0; ((*ic).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize];
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < (*ruleset).rules.len() {
        let rule_j0: *mut ChainingRule = (&(*ruleset).rules)[j_0 as usize]
            .as_deref()
            .expect("chaining rule slot should never be None at build time")
            as *const ChainingRule as *mut ChainingRule;
        let mut ib: TableId = (*rule_j0).input_begins;
        let mut start_class: TableId = (&(*rule_j0).match_0)[ib as usize][0]
        .index as TableId;
        if start_class as ::core::ffi::c_int
            <= (*ic).maxclass as ::core::ffi::c_int
        {
            let fresh2 = &mut rcpg[start_class as usize];
            *fresh2 = (*fresh2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: GlyphClass = 0 as GlyphClass;
    while j_1 as ::core::ffi::c_int
        <= (*ic).maxclass as ::core::ffi::c_int
    {
        if rcpg[j_1 as usize] != 0 {
            let mut cset: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (rcpg[j_1 as usize] as ::core::ffi::c_int) as u32)]);
            let mut k: TableId = 0 as TableId;
            while (k as usize) < (*ruleset).rules.len() {
                let mut rule: *mut ChainingRule = (&(*ruleset).rules)[k as usize]
                    .as_deref()
                    .expect("chaining rule slot should never be None at build time")
                    as *const ChainingRule as *mut ChainingRule;
                let mut start_class_0: GlyphClass =
                    (&(*rule).match_0)[(*rule).input_begins as usize][0]
                    .index as GlyphClass;
                if !(start_class_0 as ::core::ffi::c_int != j_1 as ::core::ffi::c_int) {
                    reverse_backtracks(rule);
                    let mut n_backtrack: TableId = (*rule).input_begins;
                    let mut n_input: TableId = ((*rule).input_ends as ::core::ffi::c_int
                        - (*rule).input_begins as ::core::ffi::c_int)
                        as TableId;
                    let mut n_lookahead: TableId = ((*rule).match_count as ::core::ffi::c_int
                        - (*rule).input_ends as ::core::ffi::c_int)
                        as TableId;
                    let mut n_subst: TableId = (*rule).apply.len() as TableId;
                    let mut r: *mut BkBlock = bk_new_block(&[]);
                    bk_push(r, &[bk_int(BkCellType::B16, (n_backtrack as ::core::ffi::c_int) as u32)]);
                    let mut m: TableId = 0 as TableId;
                    while (m as ::core::ffi::c_int) < (*rule).input_begins as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((&(*rule).match_0)[m as usize][0]
                            .index as ::core::ffi::c_int) as u32)]);
                        m = m.wrapping_add(1);
                    }
                    bk_push(r, &[bk_int(BkCellType::B16, (n_input as ::core::ffi::c_int) as u32)]);
                    let mut m_0: TableId = ((*rule).input_begins as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as TableId;
                    while (m_0 as ::core::ffi::c_int) < (*rule).input_ends as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((&(*rule).match_0)[m_0 as usize][0]
                            .index as ::core::ffi::c_int) as u32)]);
                        m_0 = m_0.wrapping_add(1);
                    }
                    bk_push(r, &[bk_int(BkCellType::B16, (n_lookahead as ::core::ffi::c_int) as u32)]);
                    let mut m_1: TableId = (*rule).input_ends;
                    while (m_1 as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((&(*rule).match_0)[m_1 as usize][0]
                            .index as ::core::ffi::c_int) as u32)]);
                        m_1 = m_1.wrapping_add(1);
                    }
                    bk_push(r, &[bk_int(BkCellType::B16, (n_subst as ::core::ffi::c_int) as u32)]);
                    let mut m_2: TableId = 0 as TableId;
                    while (m_2 as ::core::ffi::c_int) < n_subst as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((&(*rule).apply)[m_2 as usize].index as ::core::ffi::c_int
                                - n_backtrack as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((&(*rule).apply)[m_2 as usize].lookup.index
                                as ::core::ffi::c_int) as u32)]);
                        m_2 = m_2.wrapping_add(1);
                    }
                    bk_push(cset, &[bk_ptr(BkCellType::P16, r)]);
                }
                k = k.wrapping_add(1);
            }
            bk_push(root, &[bk_ptr(BkCellType::P16, cset)]);
        } else {
            bk_push(root, &[bk_ptr(BkCellType::P16, ::core::ptr::null_mut())]);
        }
        j_1 = j_1.wrapping_add(1);
    }
    return bk_build_block(root);
}
pub unsafe fn otfcc_build_chaining(
    mut _subtable: *const ChainingSubtable,
) -> *mut Buffer {
    if chaining_is_classified(_subtable)
    {
        return otfcc_build_chaining_classes(_subtable);
    } else {
        return otfcc_build_chaining_coverage(_subtable);
    };
}
pub unsafe fn otfcc_build_contextual_coverage(
    mut _subtable: *const ChainingSubtable,
) -> *mut Buffer {
    let subtable: *const ChainingSubtable = _subtable;
    let mut rule: *mut ChainingRule = chaining_rule_mut_from_const(subtable);
    let mut n_input: TableId = ((*rule).input_ends as ::core::ffi::c_int
        - (*rule).input_begins as ::core::ffi::c_int) as TableId;
    let mut n_subst: TableId = (*rule).apply.len() as TableId;
    reverse_backtracks(rule);
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 3 as u32)]);
    bk_push(root, &[bk_int(BkCellType::B16, (n_input as ::core::ffi::c_int) as u32)]);
    bk_push(root, &[bk_int(BkCellType::B16, (n_subst as ::core::ffi::c_int) as u32)]);
    let mut j: TableId = (*rule).input_begins;
    while (j as ::core::ffi::c_int) < (*rule).input_ends as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_coverage(
                &(&(*rule).match_0)[j as usize] as *const Coverage,
            )))]);
        j = j.wrapping_add(1);
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < n_subst as ::core::ffi::c_int {
        bk_push(root, &[bk_int(BkCellType::B16, ((&(*rule).apply)[j_0 as usize].index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((&(*rule).apply)[j_0 as usize].lookup.index as ::core::ffi::c_int) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    return bk_build_block(root);
}
pub unsafe fn otfcc_build_contextual_classes(
    mut _subtable: *const ChainingSubtable,
) -> *mut Buffer {
    let subtable: *const ChainingSubtable = _subtable;
    let ruleset: *const ChainingRuleSet = chaining_ruleset_const(subtable);
    let ic: *mut ClassDef = (*ruleset).ic.as_deref().unwrap() as *const ClassDef as *mut ClassDef;
    let coverage: *mut Coverage = &raw mut (*ic).glyphs;
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 2 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_coverage(
            coverage,
        ))), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(build_class_def(
            ic,
        ))), bk_int(BkCellType::B16, ((*ic).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as u32)]);
    let mut rcpg: Vec<GlyphClass> = vec![0; ((*ic).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize];
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < (*ruleset).rules.len() {
        let rule_j0: *mut ChainingRule = (&(*ruleset).rules)[j_0 as usize]
            .as_deref()
            .expect("chaining rule slot should never be None at build time")
            as *const ChainingRule as *mut ChainingRule;
        let mut ib: TableId = (*rule_j0).input_begins;
        let mut start_class: TableId = (&(*rule_j0).match_0)[ib as usize][0]
        .index as TableId;
        if start_class as ::core::ffi::c_int
            <= (*ic).maxclass as ::core::ffi::c_int
        {
            let fresh3 = &mut rcpg[start_class as usize];
            *fresh3 = (*fresh3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: GlyphClass = 0 as GlyphClass;
    while j_1 as ::core::ffi::c_int
        <= (*ic).maxclass as ::core::ffi::c_int
    {
        if rcpg[j_1 as usize] != 0 {
            let mut cset: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (rcpg[j_1 as usize] as ::core::ffi::c_int) as u32)]);
            let mut k: TableId = 0 as TableId;
            while (k as usize) < (*ruleset).rules.len() {
                let mut rule: *mut ChainingRule = (&(*ruleset).rules)[k as usize]
                    .as_deref()
                    .expect("chaining rule slot should never be None at build time")
                    as *const ChainingRule as *mut ChainingRule;
                let mut start_class_0: GlyphClass =
                    (&(*rule).match_0)[(*rule).input_begins as usize][0]
                    .index as GlyphClass;
                if !(start_class_0 as ::core::ffi::c_int != j_1 as ::core::ffi::c_int) {
                    reverse_backtracks(rule);
                    let mut n_input: TableId = ((*rule).input_ends as ::core::ffi::c_int
                        - (*rule).input_begins as ::core::ffi::c_int)
                        as TableId;
                    let mut n_subst: TableId = (*rule).apply.len() as TableId;
                    let mut r: *mut BkBlock = bk_new_block(&[]);
                    bk_push(r, &[bk_int(BkCellType::B16, (n_input as ::core::ffi::c_int) as u32)]);
                    bk_push(r, &[bk_int(BkCellType::B16, (n_subst as ::core::ffi::c_int) as u32)]);
                    let mut m: TableId = ((*rule).input_begins as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as TableId;
                    while (m as ::core::ffi::c_int) < (*rule).input_ends as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((&(*rule).match_0)[m as usize][0]
                            .index as ::core::ffi::c_int) as u32)]);
                        m = m.wrapping_add(1);
                    }
                    let mut m_0: TableId = 0 as TableId;
                    while (m_0 as ::core::ffi::c_int) < n_subst as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((&(*rule).apply)[m_0 as usize].index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((&(*rule).apply)[m_0 as usize].lookup.index
                                as ::core::ffi::c_int) as u32)]);
                        m_0 = m_0.wrapping_add(1);
                    }
                    bk_push(cset, &[bk_ptr(BkCellType::P16, r)]);
                }
                k = k.wrapping_add(1);
            }
            bk_push(root, &[bk_ptr(BkCellType::P16, cset)]);
        } else {
            bk_push(root, &[bk_ptr(BkCellType::P16, ::core::ptr::null_mut())]);
        }
        j_1 = j_1.wrapping_add(1);
    }
    return bk_build_block(root);
}
pub unsafe fn otfcc_build_contextual(
    mut _subtable: *const ChainingSubtable,
) -> *mut Buffer {
    if chaining_is_classified(_subtable)
    {
        return otfcc_build_contextual_classes(_subtable);
    } else {
        return otfcc_build_contextual_coverage(_subtable);
    };
}
#[inline]
unsafe fn reverse_backtracks(mut rule: *mut ChainingRule) {
    let input_begins = (*rule).input_begins as usize;
    (&mut (*rule).match_0)[..input_begins].reverse();
}
