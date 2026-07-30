#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};

use crate::table::otl::coverage::Coverage;


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{Buffer};
use crate::support::primitives::{GlyphClass, TableId};


use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_Block, bk_ptr, bk_push};

use crate::table::otl::{ChainingRule, Lookup, Subtable, ChainingType, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GSUB_CHAINING, ChainingSubtable};
use crate::bk::bkblock::{bk_newBlockFromBuffer};
use crate::bk::bkgraph::{bk_build_Block};
use crate::table::otl::classdef::{OTL_I_CLASS_DEF};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
pub unsafe extern "C" fn otfcc_chainingLookupIsContextualLookup(
    mut lookup: *const Lookup,
) -> bool {
    if !((*lookup).type_0 == OTL_TYPE_GPOS_CHAINING
        || (*lookup).type_0 == OTL_TYPE_GSUB_CHAINING)
    {
        return false;
    }
    let mut is_contextual: bool = true;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*lookup).subtables.length {
        let mut subtable: *const ChainingSubtable =
            &raw mut (**(*lookup).subtables.items.offset(j as isize)).chaining;
        if (*subtable).type_0 == ChainingType::Classified
        {
            let mut k: TableId = 0 as TableId;
            while (k as ::core::ffi::c_int)
                < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
            {
                let mut rule: *mut ChainingRule = *(*subtable)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let mut n_backtrack: TableId = (*rule).inputBegins;
                let mut n_lookahead: TableId = ((*rule).matchCount as ::core::ffi::c_int
                    - (*rule).inputEnds as ::core::ffi::c_int)
                    as TableId;
                is_contextual =
                    is_contextual as ::core::ffi::c_int != 0 && n_backtrack == 0 && n_lookahead == 0;
                k = k.wrapping_add(1);
            }
        } else {
            let mut rule_0: *mut ChainingRule =
                &raw const (*subtable).c2rust_unnamed.rule as *mut ChainingRule;
            let mut n_backtrack_0: TableId = (*rule_0).inputBegins;
            let mut n_lookahead_0: TableId = ((*rule_0).matchCount as ::core::ffi::c_int
                - (*rule_0).inputEnds as ::core::ffi::c_int)
                as TableId;
            is_contextual =
                is_contextual as ::core::ffi::c_int != 0 && n_backtrack_0 == 0 && n_lookahead_0 == 0;
        }
        j = j.wrapping_add(1);
    }
    return is_contextual;
}
pub unsafe extern "C" fn otfcc_build_chaining_coverage(
    mut _subtable: *const Subtable,
) -> *mut Buffer {
    let mut subtable: *const ChainingSubtable = &raw const (*_subtable).chaining;
    let mut rule: *mut ChainingRule =
        &raw const (*subtable).c2rust_unnamed.rule as *mut ChainingRule;
    let mut n_backtrack: TableId = (*rule).inputBegins;
    let mut n_input: TableId = ((*rule).inputEnds as ::core::ffi::c_int
        - (*rule).inputBegins as ::core::ffi::c_int) as TableId;
    let mut n_lookahead: TableId = ((*rule).matchCount as ::core::ffi::c_int
        - (*rule).inputEnds as ::core::ffi::c_int) as TableId;
    let mut n_subst: TableId = (*rule).applyCount;
    reverseBacktracks(rule);
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, 3 as u32)]);
    bk_push(root, &[bk_int(BkCellType::B16, (n_backtrack as ::core::ffi::c_int) as u32)]);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*rule).inputBegins as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j as isize),
            )))]);
        j = j.wrapping_add(1);
    }
    bk_push(root, &[bk_int(BkCellType::B16, (n_input as ::core::ffi::c_int) as u32)]);
    let mut j_0: TableId = (*rule).inputBegins;
    while (j_0 as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j_0 as isize),
            )))]);
        j_0 = j_0.wrapping_add(1);
    }
    bk_push(root, &[bk_int(BkCellType::B16, (n_lookahead as ::core::ffi::c_int) as u32)]);
    let mut j_1: TableId = (*rule).inputEnds;
    while (j_1 as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j_1 as isize),
            )))]);
        j_1 = j_1.wrapping_add(1);
    }
    bk_push(root, &[bk_int(BkCellType::B16, ((*rule).applyCount as ::core::ffi::c_int) as u32)]);
    let mut j_2: TableId = 0 as TableId;
    while (j_2 as ::core::ffi::c_int) < n_subst as ::core::ffi::c_int {
        bk_push(root, &[bk_int(BkCellType::B16, ((*(*rule).apply.offset(j_2 as isize)).index as ::core::ffi::c_int
                - n_backtrack as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*(*rule).apply.offset(j_2 as isize)).lookup.index as ::core::ffi::c_int) as u32)]);
        j_2 = j_2.wrapping_add(1);
    }
    return bk_build_Block(root);
}
pub unsafe extern "C" fn otfcc_build_chaining_classes(
    mut _subtable: *const Subtable,
) -> *mut Buffer {
    let mut subtable: *const ChainingSubtable = &raw const (*_subtable).chaining;
    let mut coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    coverage = __caryll_allocate_clean(
        ::core::mem::size_of::<Coverage>() as usize,
        67 as ::core::ffi::c_ulong,
    ) as *mut Coverage;
    (*coverage).numGlyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).numGlyphs;
    (*coverage).glyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).glyphs;
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, 2 as u32), bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            coverage,
        ))), bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_CLASS_DEF.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.bc,
        ))), bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_CLASS_DEF.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.ic,
        ))), bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_CLASS_DEF.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.fc,
        ))), bk_int(BkCellType::B16, ((*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as u32)]);
    let mut rcpg: *mut GlyphClass = ::core::ptr::null_mut::<GlyphClass>();
    rcpg = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphClass>() as usize).wrapping_mul(
            ((*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as usize,
        ),
        81 as ::core::ffi::c_ulong,
    ) as *mut GlyphClass;
    let mut j: GlyphClass = 0 as GlyphClass;
    while j as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        *rcpg.offset(j as isize) = 0 as GlyphClass;
        j = j.wrapping_add(1);
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int)
        < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
    {
        let mut ib: TableId = (**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .inputBegins;
        let mut start_class: TableId = (*(**(**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .match_0
        .offset(ib as isize))
        .glyphs
        .offset(0 as ::core::ffi::c_int as isize))
        .index as TableId;
        if start_class as ::core::ffi::c_int
            <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
        {
            let ref mut fresh2 = *rcpg.offset(start_class as isize);
            *fresh2 = (*fresh2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: GlyphClass = 0 as GlyphClass;
    while j_1 as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        if *rcpg.offset(j_1 as isize) != 0 {
            let mut cset: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, (*rcpg.offset(j_1 as isize) as ::core::ffi::c_int) as u32)]);
            let mut k: TableId = 0 as TableId;
            while (k as ::core::ffi::c_int)
                < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
            {
                let mut rule: *mut ChainingRule = *(*subtable)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let mut start_class_0: GlyphClass =
                    (*(**(*rule).match_0.offset((*rule).inputBegins as isize))
                        .glyphs
                        .offset(0 as ::core::ffi::c_int as isize))
                    .index as GlyphClass;
                if !(start_class_0 as ::core::ffi::c_int != j_1 as ::core::ffi::c_int) {
                    reverseBacktracks(rule);
                    let mut n_backtrack: TableId = (*rule).inputBegins;
                    let mut n_input: TableId = ((*rule).inputEnds as ::core::ffi::c_int
                        - (*rule).inputBegins as ::core::ffi::c_int)
                        as TableId;
                    let mut n_lookahead: TableId = ((*rule).matchCount as ::core::ffi::c_int
                        - (*rule).inputEnds as ::core::ffi::c_int)
                        as TableId;
                    let mut n_subst: TableId = (*rule).applyCount;
                    let mut r: *mut BkBlock = bk_new_Block(&[]);
                    bk_push(r, &[bk_int(BkCellType::B16, (n_backtrack as ::core::ffi::c_int) as u32)]);
                    let mut m: TableId = 0 as TableId;
                    while (m as ::core::ffi::c_int) < (*rule).inputBegins as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((*(**(*rule).match_0.offset(m as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int) as u32)]);
                        m = m.wrapping_add(1);
                    }
                    bk_push(r, &[bk_int(BkCellType::B16, (n_input as ::core::ffi::c_int) as u32)]);
                    let mut m_0: TableId = ((*rule).inputBegins as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as TableId;
                    while (m_0 as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((*(**(*rule).match_0.offset(m_0 as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int) as u32)]);
                        m_0 = m_0.wrapping_add(1);
                    }
                    bk_push(r, &[bk_int(BkCellType::B16, (n_lookahead as ::core::ffi::c_int) as u32)]);
                    let mut m_1: TableId = (*rule).inputEnds;
                    while (m_1 as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((*(**(*rule).match_0.offset(m_1 as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int) as u32)]);
                        m_1 = m_1.wrapping_add(1);
                    }
                    bk_push(r, &[bk_int(BkCellType::B16, (n_subst as ::core::ffi::c_int) as u32)]);
                    let mut m_2: TableId = 0 as TableId;
                    while (m_2 as ::core::ffi::c_int) < n_subst as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((*(*rule).apply.offset(m_2 as isize)).index as ::core::ffi::c_int
                                - n_backtrack as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*(*rule).apply.offset(m_2 as isize)).lookup.index
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
    free(coverage as *mut ::core::ffi::c_void);
    coverage = ::core::ptr::null_mut::<Coverage>();
    free(rcpg as *mut ::core::ffi::c_void);
    rcpg = ::core::ptr::null_mut::<GlyphClass>();
    return bk_build_Block(root);
}
pub unsafe extern "C" fn otfcc_build_chaining(
    mut _subtable: *const Subtable,
) -> *mut Buffer {
    if (*_subtable).chaining.type_0 == ChainingType::Classified
    {
        return otfcc_build_chaining_classes(_subtable);
    } else {
        return otfcc_build_chaining_coverage(_subtable);
    };
}
pub unsafe extern "C" fn otfcc_build_contextual_coverage(
    mut _subtable: *const Subtable,
) -> *mut Buffer {
    let mut subtable: *const ChainingSubtable = &raw const (*_subtable).chaining;
    let mut rule: *mut ChainingRule =
        &raw const (*subtable).c2rust_unnamed.rule as *mut ChainingRule;
    let mut n_input: TableId = ((*rule).inputEnds as ::core::ffi::c_int
        - (*rule).inputBegins as ::core::ffi::c_int) as TableId;
    let mut n_subst: TableId = (*rule).applyCount;
    reverseBacktracks(rule);
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, 3 as u32)]);
    bk_push(root, &[bk_int(BkCellType::B16, (n_input as ::core::ffi::c_int) as u32)]);
    bk_push(root, &[bk_int(BkCellType::B16, (n_subst as ::core::ffi::c_int) as u32)]);
    let mut j: TableId = (*rule).inputBegins;
    while (j as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j as isize),
            )))]);
        j = j.wrapping_add(1);
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < n_subst as ::core::ffi::c_int {
        bk_push(root, &[bk_int(BkCellType::B16, ((*(*rule).apply.offset(j_0 as isize)).index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*(*rule).apply.offset(j_0 as isize)).lookup.index as ::core::ffi::c_int) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    return bk_build_Block(root);
}
pub unsafe extern "C" fn otfcc_build_contextual_classes(
    mut _subtable: *const Subtable,
) -> *mut Buffer {
    let mut subtable: *const ChainingSubtable = &raw const (*_subtable).chaining;
    let mut coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    coverage = __caryll_allocate_clean(
        ::core::mem::size_of::<Coverage>() as usize,
        174 as ::core::ffi::c_ulong,
    ) as *mut Coverage;
    (*coverage).numGlyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).numGlyphs;
    (*coverage).glyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).glyphs;
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, 2 as u32), bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            coverage,
        ))), bk_ptr(BkCellType::P16, bk_newBlockFromBuffer(OTL_I_CLASS_DEF.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.ic,
        ))), bk_int(BkCellType::B16, ((*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as u32)]);
    let mut rcpg: *mut GlyphClass = ::core::ptr::null_mut::<GlyphClass>();
    rcpg = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphClass>() as usize).wrapping_mul(
            ((*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as usize,
        ),
        186 as ::core::ffi::c_ulong,
    ) as *mut GlyphClass;
    let mut j: GlyphClass = 0 as GlyphClass;
    while j as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        *rcpg.offset(j as isize) = 0 as GlyphClass;
        j = j.wrapping_add(1);
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int)
        < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
    {
        let mut ib: TableId = (**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .inputBegins;
        let mut start_class: TableId = (*(**(**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .match_0
        .offset(ib as isize))
        .glyphs
        .offset(0 as ::core::ffi::c_int as isize))
        .index as TableId;
        if start_class as ::core::ffi::c_int
            <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
        {
            let ref mut fresh3 = *rcpg.offset(start_class as isize);
            *fresh3 = (*fresh3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphClass;
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: GlyphClass = 0 as GlyphClass;
    while j_1 as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        if *rcpg.offset(j_1 as isize) != 0 {
            let mut cset: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, (*rcpg.offset(j_1 as isize) as ::core::ffi::c_int) as u32)]);
            let mut k: TableId = 0 as TableId;
            while (k as ::core::ffi::c_int)
                < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
            {
                let mut rule: *mut ChainingRule = *(*subtable)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let mut start_class_0: GlyphClass =
                    (*(**(*rule).match_0.offset((*rule).inputBegins as isize))
                        .glyphs
                        .offset(0 as ::core::ffi::c_int as isize))
                    .index as GlyphClass;
                if !(start_class_0 as ::core::ffi::c_int != j_1 as ::core::ffi::c_int) {
                    reverseBacktracks(rule);
                    let mut n_input: TableId = ((*rule).inputEnds as ::core::ffi::c_int
                        - (*rule).inputBegins as ::core::ffi::c_int)
                        as TableId;
                    let mut n_subst: TableId = (*rule).applyCount;
                    let mut r: *mut BkBlock = bk_new_Block(&[]);
                    bk_push(r, &[bk_int(BkCellType::B16, (n_input as ::core::ffi::c_int) as u32)]);
                    bk_push(r, &[bk_int(BkCellType::B16, (n_subst as ::core::ffi::c_int) as u32)]);
                    let mut m: TableId = ((*rule).inputBegins as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as TableId;
                    while (m as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((*(**(*rule).match_0.offset(m as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int) as u32)]);
                        m = m.wrapping_add(1);
                    }
                    let mut m_0: TableId = 0 as TableId;
                    while (m_0 as ::core::ffi::c_int) < n_subst as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(BkCellType::B16, ((*(*rule).apply.offset(m_0 as isize)).index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*(*rule).apply.offset(m_0 as isize)).lookup.index
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
    free(coverage as *mut ::core::ffi::c_void);
    coverage = ::core::ptr::null_mut::<Coverage>();
    free(rcpg as *mut ::core::ffi::c_void);
    rcpg = ::core::ptr::null_mut::<GlyphClass>();
    return bk_build_Block(root);
}
pub unsafe extern "C" fn otfcc_build_contextual(
    mut _subtable: *const Subtable,
) -> *mut Buffer {
    if (*_subtable).chaining.type_0 == ChainingType::Classified
    {
        return otfcc_build_contextual_classes(_subtable);
    } else {
        return otfcc_build_contextual_coverage(_subtable);
    };
}
#[inline]
unsafe extern "C" fn reverseBacktracks(mut rule: *mut ChainingRule) {
    if (*rule).inputBegins as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut start: TableId = 0 as TableId;
        let mut end: TableId =
            ((*rule).inputBegins as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as TableId;
        while end as ::core::ffi::c_int > start as ::core::ffi::c_int {
            let mut tmp: *mut Coverage = *(*rule).match_0.offset(start as isize);
            let ref mut fresh0 = *(*rule).match_0.offset(start as isize);
            *fresh0 = *(*rule).match_0.offset(end as isize);
            let ref mut fresh1 = *(*rule).match_0.offset(end as isize);
            *fresh1 = tmp;
            end = end.wrapping_sub(1);
            start = start.wrapping_add(1);
        }
    }
}
