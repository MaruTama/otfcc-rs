#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};
unsafe extern "C" {
    static otl_iCoverage: __otfcc_ICoverage;
    static otl_iClassDef: __otfcc_IClassDef;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}

use crate::table::otl::classdef::{__otfcc_IClassDef};
use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{caryll_Buffer};
use crate::support::primitives::{glyphclass_t, tableid_t};


use crate::bk::bkblock::{b16, bk_Block, bk_int, bk_new_Block, bk_ptr, bk_push, p16};

use crate::table::otl::{otl_ChainingRule, otl_Lookup, otl_Subtable, otl_chaining_classified, otl_type_gpos_chaining, otl_type_gsub_chaining, subtable_chaining};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_chainingLookupIsContextualLookup(
    mut lookup: *const otl_Lookup,
) -> bool {
    if !((*lookup).type_0 == otl_type_gpos_chaining
        || (*lookup).type_0 == otl_type_gsub_chaining)
    {
        return false;
    }
    let mut isContextual: bool = true;
    let mut j: tableid_t = 0 as tableid_t;
    while (j as usize) < (*lookup).subtables.length {
        let mut subtable: *const subtable_chaining =
            &raw mut (**(*lookup).subtables.items.offset(j as isize)).chaining;
        if (*subtable).type_0 == otl_chaining_classified
        {
            let mut k: tableid_t = 0 as tableid_t;
            while (k as ::core::ffi::c_int)
                < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
            {
                let mut rule: *mut otl_ChainingRule = *(*subtable)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let mut nBacktrack: tableid_t = (*rule).inputBegins;
                let mut nLookahead: tableid_t = ((*rule).matchCount as ::core::ffi::c_int
                    - (*rule).inputEnds as ::core::ffi::c_int)
                    as tableid_t;
                isContextual =
                    isContextual as ::core::ffi::c_int != 0 && nBacktrack == 0 && nLookahead == 0;
                k = k.wrapping_add(1);
            }
        } else {
            let mut rule_0: *mut otl_ChainingRule =
                &raw const (*subtable).c2rust_unnamed.rule as *mut otl_ChainingRule;
            let mut nBacktrack_0: tableid_t = (*rule_0).inputBegins;
            let mut nLookahead_0: tableid_t = ((*rule_0).matchCount as ::core::ffi::c_int
                - (*rule_0).inputEnds as ::core::ffi::c_int)
                as tableid_t;
            isContextual =
                isContextual as ::core::ffi::c_int != 0 && nBacktrack_0 == 0 && nLookahead_0 == 0;
        }
        j = j.wrapping_add(1);
    }
    return isContextual;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_build_chaining_coverage(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_chaining = &raw const (*_subtable).chaining;
    let mut rule: *mut otl_ChainingRule =
        &raw const (*subtable).c2rust_unnamed.rule as *mut otl_ChainingRule;
    let mut nBacktrack: tableid_t = (*rule).inputBegins;
    let mut nInput: tableid_t = ((*rule).inputEnds as ::core::ffi::c_int
        - (*rule).inputBegins as ::core::ffi::c_int) as tableid_t;
    let mut nLookahead: tableid_t = ((*rule).matchCount as ::core::ffi::c_int
        - (*rule).inputEnds as ::core::ffi::c_int) as tableid_t;
    let mut nSubst: tableid_t = (*rule).applyCount;
    reverseBacktracks(rule);
    let mut root: *mut bk_Block = bk_new_Block(&[bk_int(b16, 3 as u32)]);
    bk_push(root, &[bk_int(b16, (nBacktrack as ::core::ffi::c_int) as u32)]);
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*rule).inputBegins as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j as isize),
            )))]);
        j = j.wrapping_add(1);
    }
    bk_push(root, &[bk_int(b16, (nInput as ::core::ffi::c_int) as u32)]);
    let mut j_0: tableid_t = (*rule).inputBegins;
    while (j_0 as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j_0 as isize),
            )))]);
        j_0 = j_0.wrapping_add(1);
    }
    bk_push(root, &[bk_int(b16, (nLookahead as ::core::ffi::c_int) as u32)]);
    let mut j_1: tableid_t = (*rule).inputEnds;
    while (j_1 as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j_1 as isize),
            )))]);
        j_1 = j_1.wrapping_add(1);
    }
    bk_push(root, &[bk_int(b16, ((*rule).applyCount as ::core::ffi::c_int) as u32)]);
    let mut j_2: tableid_t = 0 as tableid_t;
    while (j_2 as ::core::ffi::c_int) < nSubst as ::core::ffi::c_int {
        bk_push(root, &[bk_int(b16, ((*(*rule).apply.offset(j_2 as isize)).index as ::core::ffi::c_int
                - nBacktrack as ::core::ffi::c_int) as u32), bk_int(b16, ((*(*rule).apply.offset(j_2 as isize)).lookup.index as ::core::ffi::c_int) as u32)]);
        j_2 = j_2.wrapping_add(1);
    }
    return bk_build_Block(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_build_chaining_classes(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_chaining = &raw const (*_subtable).chaining;
    let mut coverage: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    coverage = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_Coverage>() as usize,
        67 as ::core::ffi::c_ulong,
    ) as *mut otl_Coverage;
    (*coverage).numGlyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).numGlyphs;
    (*coverage).glyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).glyphs;
    let mut root: *mut bk_Block = bk_new_Block(&[bk_int(b16, 2 as u32), bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            coverage,
        ))), bk_ptr(p16, bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.bc,
        ))), bk_ptr(p16, bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.ic,
        ))), bk_ptr(p16, bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.fc,
        ))), bk_int(b16, ((*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as u32)]);
    let mut rcpg: *mut glyphclass_t = ::core::ptr::null_mut::<glyphclass_t>();
    rcpg = __caryll_allocate_clean(
        (::core::mem::size_of::<glyphclass_t>() as usize).wrapping_mul(
            ((*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as usize,
        ),
        81 as ::core::ffi::c_ulong,
    ) as *mut glyphclass_t;
    let mut j: glyphclass_t = 0 as glyphclass_t;
    while j as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        *rcpg.offset(j as isize) = 0 as glyphclass_t;
        j = j.wrapping_add(1);
    }
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int)
        < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
    {
        let mut ib: tableid_t = (**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .inputBegins;
        let mut startClass: tableid_t = (*(**(**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .match_0
        .offset(ib as isize))
        .glyphs
        .offset(0 as ::core::ffi::c_int as isize))
        .index as tableid_t;
        if startClass as ::core::ffi::c_int
            <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
        {
            let ref mut fresh2 = *rcpg.offset(startClass as isize);
            *fresh2 = (*fresh2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphclass_t;
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: glyphclass_t = 0 as glyphclass_t;
    while j_1 as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        if *rcpg.offset(j_1 as isize) != 0 {
            let mut cset: *mut bk_Block = bk_new_Block(&[bk_int(b16, (*rcpg.offset(j_1 as isize) as ::core::ffi::c_int) as u32)]);
            let mut k: tableid_t = 0 as tableid_t;
            while (k as ::core::ffi::c_int)
                < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
            {
                let mut rule: *mut otl_ChainingRule = *(*subtable)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let mut startClass_0: glyphclass_t =
                    (*(**(*rule).match_0.offset((*rule).inputBegins as isize))
                        .glyphs
                        .offset(0 as ::core::ffi::c_int as isize))
                    .index as glyphclass_t;
                if !(startClass_0 as ::core::ffi::c_int != j_1 as ::core::ffi::c_int) {
                    reverseBacktracks(rule);
                    let mut nBacktrack: tableid_t = (*rule).inputBegins;
                    let mut nInput: tableid_t = ((*rule).inputEnds as ::core::ffi::c_int
                        - (*rule).inputBegins as ::core::ffi::c_int)
                        as tableid_t;
                    let mut nLookahead: tableid_t = ((*rule).matchCount as ::core::ffi::c_int
                        - (*rule).inputEnds as ::core::ffi::c_int)
                        as tableid_t;
                    let mut nSubst: tableid_t = (*rule).applyCount;
                    let mut r: *mut bk_Block = bk_new_Block(&[]);
                    bk_push(r, &[bk_int(b16, (nBacktrack as ::core::ffi::c_int) as u32)]);
                    let mut m: tableid_t = 0 as tableid_t;
                    while (m as ::core::ffi::c_int) < (*rule).inputBegins as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(b16, ((*(**(*rule).match_0.offset(m as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int) as u32)]);
                        m = m.wrapping_add(1);
                    }
                    bk_push(r, &[bk_int(b16, (nInput as ::core::ffi::c_int) as u32)]);
                    let mut m_0: tableid_t = ((*rule).inputBegins as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as tableid_t;
                    while (m_0 as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(b16, ((*(**(*rule).match_0.offset(m_0 as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int) as u32)]);
                        m_0 = m_0.wrapping_add(1);
                    }
                    bk_push(r, &[bk_int(b16, (nLookahead as ::core::ffi::c_int) as u32)]);
                    let mut m_1: tableid_t = (*rule).inputEnds;
                    while (m_1 as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(b16, ((*(**(*rule).match_0.offset(m_1 as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int) as u32)]);
                        m_1 = m_1.wrapping_add(1);
                    }
                    bk_push(r, &[bk_int(b16, (nSubst as ::core::ffi::c_int) as u32)]);
                    let mut m_2: tableid_t = 0 as tableid_t;
                    while (m_2 as ::core::ffi::c_int) < nSubst as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(b16, ((*(*rule).apply.offset(m_2 as isize)).index as ::core::ffi::c_int
                                - nBacktrack as ::core::ffi::c_int) as u32), bk_int(b16, ((*(*rule).apply.offset(m_2 as isize)).lookup.index
                                as ::core::ffi::c_int) as u32)]);
                        m_2 = m_2.wrapping_add(1);
                    }
                    bk_push(cset, &[bk_ptr(p16, r)]);
                }
                k = k.wrapping_add(1);
            }
            bk_push(root, &[bk_ptr(p16, cset)]);
        } else {
            bk_push(root, &[bk_ptr(p16, ::core::ptr::null_mut())]);
        }
        j_1 = j_1.wrapping_add(1);
    }
    free(coverage as *mut ::core::ffi::c_void);
    coverage = ::core::ptr::null_mut::<otl_Coverage>();
    free(rcpg as *mut ::core::ffi::c_void);
    rcpg = ::core::ptr::null_mut::<glyphclass_t>();
    return bk_build_Block(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_build_chaining(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    if (*_subtable).chaining.type_0 == otl_chaining_classified
    {
        return otfcc_build_chaining_classes(_subtable);
    } else {
        return otfcc_build_chaining_coverage(_subtable);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_build_contextual_coverage(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_chaining = &raw const (*_subtable).chaining;
    let mut rule: *mut otl_ChainingRule =
        &raw const (*subtable).c2rust_unnamed.rule as *mut otl_ChainingRule;
    let mut nInput: tableid_t = ((*rule).inputEnds as ::core::ffi::c_int
        - (*rule).inputBegins as ::core::ffi::c_int) as tableid_t;
    let mut nSubst: tableid_t = (*rule).applyCount;
    reverseBacktracks(rule);
    let mut root: *mut bk_Block = bk_new_Block(&[bk_int(b16, 3 as u32)]);
    bk_push(root, &[bk_int(b16, (nInput as ::core::ffi::c_int) as u32)]);
    bk_push(root, &[bk_int(b16, (nSubst as ::core::ffi::c_int) as u32)]);
    let mut j: tableid_t = (*rule).inputBegins;
    while (j as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
                *(*rule).match_0.offset(j as isize),
            )))]);
        j = j.wrapping_add(1);
    }
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int) < nSubst as ::core::ffi::c_int {
        bk_push(root, &[bk_int(b16, ((*(*rule).apply.offset(j_0 as isize)).index as ::core::ffi::c_int) as u32), bk_int(b16, ((*(*rule).apply.offset(j_0 as isize)).lookup.index as ::core::ffi::c_int) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    return bk_build_Block(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_build_contextual_classes(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_chaining = &raw const (*_subtable).chaining;
    let mut coverage: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    coverage = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_Coverage>() as usize,
        174 as ::core::ffi::c_ulong,
    ) as *mut otl_Coverage;
    (*coverage).numGlyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).numGlyphs;
    (*coverage).glyphs = (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).glyphs;
    let mut root: *mut bk_Block = bk_new_Block(&[bk_int(b16, 2 as u32), bk_ptr(p16, bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(
            coverage,
        ))), bk_ptr(p16, bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
            (*subtable).c2rust_unnamed.c2rust_unnamed.ic,
        ))), bk_int(b16, ((*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as u32)]);
    let mut rcpg: *mut glyphclass_t = ::core::ptr::null_mut::<glyphclass_t>();
    rcpg = __caryll_allocate_clean(
        (::core::mem::size_of::<glyphclass_t>() as usize).wrapping_mul(
            ((*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as usize,
        ),
        186 as ::core::ffi::c_ulong,
    ) as *mut glyphclass_t;
    let mut j: glyphclass_t = 0 as glyphclass_t;
    while j as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        *rcpg.offset(j as isize) = 0 as glyphclass_t;
        j = j.wrapping_add(1);
    }
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int)
        < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
    {
        let mut ib: tableid_t = (**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .inputBegins;
        let mut startClass: tableid_t = (*(**(**(*subtable)
            .c2rust_unnamed
            .c2rust_unnamed
            .rules
            .offset(j_0 as isize))
        .match_0
        .offset(ib as isize))
        .glyphs
        .offset(0 as ::core::ffi::c_int as isize))
        .index as tableid_t;
        if startClass as ::core::ffi::c_int
            <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
        {
            let ref mut fresh3 = *rcpg.offset(startClass as isize);
            *fresh3 = (*fresh3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphclass_t;
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: glyphclass_t = 0 as glyphclass_t;
    while j_1 as ::core::ffi::c_int
        <= (*(*subtable).c2rust_unnamed.c2rust_unnamed.ic).maxclass as ::core::ffi::c_int
    {
        if *rcpg.offset(j_1 as isize) != 0 {
            let mut cset: *mut bk_Block = bk_new_Block(&[bk_int(b16, (*rcpg.offset(j_1 as isize) as ::core::ffi::c_int) as u32)]);
            let mut k: tableid_t = 0 as tableid_t;
            while (k as ::core::ffi::c_int)
                < (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount as ::core::ffi::c_int
            {
                let mut rule: *mut otl_ChainingRule = *(*subtable)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let mut startClass_0: glyphclass_t =
                    (*(**(*rule).match_0.offset((*rule).inputBegins as isize))
                        .glyphs
                        .offset(0 as ::core::ffi::c_int as isize))
                    .index as glyphclass_t;
                if !(startClass_0 as ::core::ffi::c_int != j_1 as ::core::ffi::c_int) {
                    reverseBacktracks(rule);
                    let mut nInput: tableid_t = ((*rule).inputEnds as ::core::ffi::c_int
                        - (*rule).inputBegins as ::core::ffi::c_int)
                        as tableid_t;
                    let mut nSubst: tableid_t = (*rule).applyCount;
                    let mut r: *mut bk_Block = bk_new_Block(&[]);
                    bk_push(r, &[bk_int(b16, (nInput as ::core::ffi::c_int) as u32)]);
                    bk_push(r, &[bk_int(b16, (nSubst as ::core::ffi::c_int) as u32)]);
                    let mut m: tableid_t = ((*rule).inputBegins as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as tableid_t;
                    while (m as ::core::ffi::c_int) < (*rule).inputEnds as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(b16, ((*(**(*rule).match_0.offset(m as isize))
                                .glyphs
                                .offset(0 as ::core::ffi::c_int as isize))
                            .index as ::core::ffi::c_int) as u32)]);
                        m = m.wrapping_add(1);
                    }
                    let mut m_0: tableid_t = 0 as tableid_t;
                    while (m_0 as ::core::ffi::c_int) < nSubst as ::core::ffi::c_int {
                        bk_push(r, &[bk_int(b16, ((*(*rule).apply.offset(m_0 as isize)).index as ::core::ffi::c_int) as u32), bk_int(b16, ((*(*rule).apply.offset(m_0 as isize)).lookup.index
                                as ::core::ffi::c_int) as u32)]);
                        m_0 = m_0.wrapping_add(1);
                    }
                    bk_push(cset, &[bk_ptr(p16, r)]);
                }
                k = k.wrapping_add(1);
            }
            bk_push(root, &[bk_ptr(p16, cset)]);
        } else {
            bk_push(root, &[bk_ptr(p16, ::core::ptr::null_mut())]);
        }
        j_1 = j_1.wrapping_add(1);
    }
    free(coverage as *mut ::core::ffi::c_void);
    coverage = ::core::ptr::null_mut::<otl_Coverage>();
    free(rcpg as *mut ::core::ffi::c_void);
    rcpg = ::core::ptr::null_mut::<glyphclass_t>();
    return bk_build_Block(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_build_contextual(
    mut _subtable: *const otl_Subtable,
) -> *mut caryll_Buffer {
    if (*_subtable).chaining.type_0 == otl_chaining_classified
    {
        return otfcc_build_contextual_classes(_subtable);
    } else {
        return otfcc_build_contextual_coverage(_subtable);
    };
}
#[inline]
unsafe extern "C" fn reverseBacktracks(mut rule: *mut otl_ChainingRule) {
    if (*rule).inputBegins as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut start: tableid_t = 0 as tableid_t;
        let mut end: tableid_t =
            ((*rule).inputBegins as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as tableid_t;
        while end as ::core::ffi::c_int > start as ::core::ffi::c_int {
            let mut tmp: *mut otl_Coverage = *(*rule).match_0.offset(start as isize);
            let ref mut fresh0 = *(*rule).match_0.offset(start as isize);
            *fresh0 = *(*rule).match_0.offset(end as isize);
            let ref mut fresh1 = *(*rule).match_0.offset(end as isize);
            *fresh1 = tmp;
            end = end.wrapping_sub(1);
            start = start.wrapping_add(1);
        }
    }
}
