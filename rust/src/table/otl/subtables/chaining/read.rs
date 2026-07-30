#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};

use crate::table::otl::classdef::{ClassDef, otl_ClassDef_free, readClassDef};
use crate::table::otl::coverage::{Coverage, otl_Coverage_free, readCoverage};
use crate::support::handle::{handle_fromIndex, otfcc_Handle_dispose, otfcc_Handle_dup, Handle, GlyphHandle, LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};

use crate::support::{NULL};
use crate::table::otl::{ChainLookupApplication, ChainingRule, Subtable, ChainingType, ChainingSubtable};
use crate::table::otl::subtables::chaining::common::{I_SUBTABLE_CHAINING};
use crate::vendor::sds::{sdsempty};
pub type CoverageReaderHandler = Option<
    unsafe extern "C" fn(
        FontFilePointer,
        u32,
        u16,
        u32,
        u16,
        GlyphId,
        *mut ::core::ffi::c_void,
    ) -> *mut Coverage,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClassDefs {
    pub bc: *mut ClassDef,
    pub ic: *mut ClassDef,
    pub fc: *mut ClassDef,
}
pub unsafe extern "C" fn singleCoverage(
    mut _data: FontFilePointer,
    mut _tableLength: u32,
    mut gid: u16,
    mut _offset: u32,
    mut _kind: u16,
    _maxGlyphs: GlyphId,
    mut _userdata: *mut ::core::ffi::c_void,
) -> *mut Coverage {
    let mut cov: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    cov = __caryll_allocate_clean(
        ::core::mem::size_of::<Coverage>() as usize,
        14 as ::core::ffi::c_ulong,
    ) as *mut Coverage;
    (*cov).numGlyphs = 1 as GlyphId;
    (*cov).glyphs = __caryll_allocate_clean(
        ::core::mem::size_of::<GlyphHandle>() as usize,
        16 as ::core::ffi::c_ulong,
    ) as *mut GlyphHandle;
    *(*cov).glyphs.offset(0 as ::core::ffi::c_int as isize) =
        handle_fromIndex(gid) as GlyphHandle;
    return cov;
}
pub unsafe extern "C" fn classCoverage(
    mut _data: FontFilePointer,
    mut _tableLength: u32,
    mut cls: u16,
    mut _offset: u32,
    mut kind: u16,
    maxGlyphs: GlyphId,
    mut _classdefs: *mut ::core::ffi::c_void,
) -> *mut Coverage {
    let mut defs: *mut ClassDefs = _classdefs as *mut ClassDefs;
    let mut cd: *mut ClassDef = if kind as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        (*defs).bc
    } else if kind as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        (*defs).ic
    } else {
        (*defs).fc
    };
    let mut cov: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    cov = __caryll_allocate_clean(
        ::core::mem::size_of::<Coverage>() as usize,
        26 as ::core::ffi::c_ulong,
    ) as *mut Coverage;
    (*cov).numGlyphs = 0 as GlyphId;
    (*cov).glyphs = ::core::ptr::null_mut::<GlyphHandle>();
    let mut count: GlyphId = 0 as GlyphId;
    if cls as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int) < maxGlyphs as ::core::ffi::c_int {
            let mut found: bool = false;
            let mut j: GlyphId = 0 as GlyphId;
            while (j as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
                if *(*cd).classes.offset(j as isize) as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && (*(*cd).glyphs.offset(j as isize)).index as ::core::ffi::c_int
                        == k as ::core::ffi::c_int
                {
                    found = true;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
            if !found {
                count = (count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
            }
            k = k.wrapping_add(1);
        }
    } else {
        let mut j_0: GlyphId = 0 as GlyphId;
        while (j_0 as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
            if *(*cd).classes.offset(j_0 as isize) as ::core::ffi::c_int
                == cls as ::core::ffi::c_int
            {
                count = count.wrapping_add(1);
            }
            j_0 = j_0.wrapping_add(1);
        }
    }
    if count == 0 {
        return cov;
    }
    (*cov).numGlyphs = count;
    (*cov).glyphs = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphHandle>() as usize).wrapping_mul(count as usize),
        49 as ::core::ffi::c_ulong,
    ) as *mut GlyphHandle;
    let mut jj: GlyphId = 0 as GlyphId;
    if cls as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut k_0: GlyphId = 0 as GlyphId;
        while (k_0 as ::core::ffi::c_int) < maxGlyphs as ::core::ffi::c_int {
            let mut found_0: bool = false;
            let mut j_1: GlyphId = 0 as GlyphId;
            while (j_1 as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
                if *(*cd).classes.offset(j_1 as isize) as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                    && (*(*cd).glyphs.offset(j_1 as isize)).index as ::core::ffi::c_int
                        == k_0 as ::core::ffi::c_int
                {
                    found_0 = true;
                    break;
                } else {
                    j_1 = j_1.wrapping_add(1);
                }
            }
            if !found_0 {
                let fresh12 = jj;
                jj = jj.wrapping_add(1);
                *(*cov).glyphs.offset(fresh12 as isize) =
                    handle_fromIndex(k_0)
                        as GlyphHandle;
            }
            k_0 = k_0.wrapping_add(1);
        }
    } else {
        let mut j_2: GlyphId = 0 as GlyphId;
        while (j_2 as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
            if *(*cd).classes.offset(j_2 as isize) as ::core::ffi::c_int
                == cls as ::core::ffi::c_int
            {
                let fresh13 = jj;
                jj = jj.wrapping_add(1);
                *(*cov).glyphs.offset(fresh13 as isize) =
                    otfcc_Handle_dup(
                        *(*cd).glyphs.offset(j_2 as isize) as Handle,
                    ) as GlyphHandle;
            }
            j_2 = j_2.wrapping_add(1);
        }
    }
    return cov;
}
pub unsafe extern "C" fn format3Coverage(
    mut data: FontFilePointer,
    mut tableLength: u32,
    mut shift: u16,
    mut _offset: u32,
    mut _kind: u16,
    _maxGlyphs: GlyphId,
    mut _userdata: *mut ::core::ffi::c_void,
) -> *mut Coverage {
    return readCoverage(
        data as *const u8,
        tableLength,
        _offset
            .wrapping_add(shift as u32)
            .wrapping_sub(2 as u32),
    );
}
pub unsafe extern "C" fn GeneralReadContextualRule(
    mut data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    mut startGID: u16,
    mut minusOne: bool,
    mut fn_0: CoverageReaderHandler,
    maxGlyphs: GlyphId,
    mut userdata: *mut ::core::ffi::c_void,
) -> *mut ChainingRule {
    let mut nInput: u16 = 0;
    let mut nApply: u16 = 0;
    let mut jj: u16 = 0;
    let mut rule: *mut ChainingRule = ::core::ptr::null_mut::<ChainingRule>();
    rule = __caryll_allocate_clean(
        ::core::mem::size_of::<ChainingRule>() as usize,
        83 as ::core::ffi::c_ulong,
    ) as *mut ChainingRule;
    (*rule).match_0 = ::core::ptr::null_mut::<*mut Coverage>();
    (*rule).apply = ::core::ptr::null_mut::<ChainLookupApplication>();
    let mut minusOneQ: u16 = (if minusOne as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as u16;
    if !(tableLength < offset.wrapping_add(4 as u32)) {
        nInput = read_16u(data.offset(offset as isize) as *const u8);
        nApply = read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        );
        if !(tableLength
            < offset
                .wrapping_add(4 as u32)
                .wrapping_add((2 as ::core::ffi::c_int * nInput as ::core::ffi::c_int) as u32)
                .wrapping_add((4 as ::core::ffi::c_int * nApply as ::core::ffi::c_int) as u32))
        {
            (*rule).matchCount = nInput as TableId;
            (*rule).inputBegins = 0 as TableId;
            (*rule).inputEnds = nInput as TableId;
            (*rule).match_0 = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut Coverage>() as usize)
                    .wrapping_mul((*rule).matchCount as usize),
                98 as ::core::ffi::c_ulong,
            ) as *mut *mut Coverage;
            jj = 0 as u16;
            if minusOne {
                let fresh16 = jj;
                jj = jj.wrapping_add(1);
                let ref mut fresh17 = *(*rule).match_0.offset(fresh16 as isize);
                *fresh17 = fn_0.expect("non-null function pointer")(
                    data,
                    tableLength,
                    startGID,
                    offset,
                    2 as u16,
                    maxGlyphs,
                    userdata,
                );
            }
            let mut j: u16 = 0 as u16;
            while (j as ::core::ffi::c_int)
                < nInput as ::core::ffi::c_int - minusOneQ as ::core::ffi::c_int
            {
                let mut gid: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                let fresh18 = jj;
                jj = jj.wrapping_add(1);
                let ref mut fresh19 = *(*rule).match_0.offset(fresh18 as isize);
                *fresh19 = fn_0.expect("non-null function pointer")(
                    data,
                    tableLength,
                    gid as u16,
                    offset,
                    2 as u16,
                    maxGlyphs,
                    userdata,
                );
                j = j.wrapping_add(1);
            }
            (*rule).applyCount = nApply as TableId;
            (*rule).apply = __caryll_allocate_clean(
                (::core::mem::size_of::<ChainLookupApplication>() as usize)
                    .wrapping_mul((*rule).applyCount as usize),
                108 as ::core::ffi::c_ulong,
            ) as *mut ChainLookupApplication;
            let mut j_0: TableId = 0 as TableId;
            while (j_0 as ::core::ffi::c_int) < nApply as ::core::ffi::c_int {
                (*(*rule).apply.offset(j_0 as isize)).index = ((*rule).inputBegins
                    as ::core::ffi::c_int
                    + read_16u(
                        data.offset(offset as isize)
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * ((*rule).matchCount as ::core::ffi::c_int
                                        - minusOneQ as ::core::ffi::c_int))
                                    as isize,
                            )
                            .offset((j_0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    ) as ::core::ffi::c_int)
                    as TableId;
                (*(*rule).apply.offset(j_0 as isize)).lookup =
                    handle_fromIndex(read_16u(
                        data.offset(offset as isize)
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * ((*rule).matchCount as ::core::ffi::c_int
                                        - minusOneQ as ::core::ffi::c_int))
                                    as isize,
                            )
                            .offset((j_0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *const u8,
                    )
                        as GlyphId) as LookupHandle;
                j_0 = j_0.wrapping_add(1);
            }
            reverseBacktracks(rule);
            return rule;
        }
    }
    deleteRule(rule);
    rule = ::core::ptr::null_mut::<ChainingRule>();
    return ::core::ptr::null_mut::<ChainingRule>();
}
unsafe extern "C" fn readContextualFormat1(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: GlyphId,
) -> *mut ChainingSubtable {
    let mut covOffset: u16 = 0;
    let mut firstCoverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut chainSubRuleSetCount: TableId = 0;
    let mut totalRules: TableId = 0;
    let mut jj: TableId = 0;
    let mut current_block: u64;
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        covOffset = offset.wrapping_add(read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as u32) as u16;
        firstCoverage = readCoverage(
            data as *const u8,
            tableLength,
            covOffset as u32,
        );
        chainSubRuleSetCount = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if !(chainSubRuleSetCount as ::core::ffi::c_int
            != (*firstCoverage).numGlyphs as ::core::ffi::c_int)
        {
            if !(tableLength
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (2 as ::core::ffi::c_int * chainSubRuleSetCount as ::core::ffi::c_int)
                        as u32,
                ))
            {
                totalRules = 0 as TableId;
                let mut j: TableId = 0 as TableId;
                loop {
                    if !((j as ::core::ffi::c_int) < chainSubRuleSetCount as ::core::ffi::c_int) {
                        current_block = 4166486009154926805;
                        break;
                    }
                    let mut srsOffset: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    )
                        as u32);
                    if tableLength < srsOffset.wrapping_add(2 as u32) {
                        current_block = 10321976752019472029;
                        break;
                    }
                    totalRules = (totalRules as ::core::ffi::c_int
                        + read_16u(data.offset(srsOffset as isize) as *const u8)
                            as ::core::ffi::c_int) as TableId;
                    if tableLength
                        < srsOffset.wrapping_add(2 as u32).wrapping_add(
                            (2 as ::core::ffi::c_int
                                * read_16u(data.offset(srsOffset as isize) as *const u8)
                                    as ::core::ffi::c_int) as u32,
                        )
                    {
                        current_block = 10321976752019472029;
                        break;
                    }
                    j = j.wrapping_add(1);
                }
                match current_block {
                    10321976752019472029 => {}
                    _ => {
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = totalRules;
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut ChainingRule>() as usize)
                                .wrapping_mul(totalRules as usize),
                            144 as ::core::ffi::c_ulong,
                        )
                            as *mut *mut ChainingRule;
                        jj = 0 as TableId;
                        let mut j_0: TableId = 0 as TableId;
                        while (j_0 as ::core::ffi::c_int)
                            < chainSubRuleSetCount as ::core::ffi::c_int
                        {
                            let mut srsOffset_0: u32 = offset.wrapping_add(read_16u(
                                data.offset(offset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as u32);
                            let mut srsCount: TableId =
                                read_16u(data.offset(srsOffset_0 as isize) as *const u8)
                                    as TableId;
                            let mut k: TableId = 0 as TableId;
                            while (k as ::core::ffi::c_int) < srsCount as ::core::ffi::c_int {
                                let mut srOffset: u32 = srsOffset_0.wrapping_add(read_16u(
                                    data.offset(srsOffset_0 as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let ref mut fresh21 = *(*subtable)
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .rules
                                    .offset(jj as isize);
                                *fresh21 = GeneralReadContextualRule(
                                    data,
                                    tableLength,
                                    srOffset,
                                    (*(*firstCoverage).glyphs.offset(j_0 as isize)).index
                                        as u16,
                                    true,
                                    Some(
                                        singleCoverage
                                            as unsafe extern "C" fn(
                                                FontFilePointer,
                                                u32,
                                                u16,
                                                u32,
                                                u16,
                                                GlyphId,
                                                *mut ::core::ffi::c_void,
                                            )
                                                -> *mut Coverage,
                                    ),
                                    maxGlyphs,
                                    NULL,
                                );
                                jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as TableId;
                                k = k.wrapping_add(1);
                            }
                            j_0 = j_0.wrapping_add(1);
                        }
                        otl_Coverage_free(firstCoverage);
                        return subtable;
                    }
                }
            }
        }
    }
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<ChainingSubtable>();
}
unsafe extern "C" fn readContextualFormat2(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: GlyphId,
) -> *mut ChainingSubtable {
    let mut cds: *mut ClassDefs = ::core::ptr::null_mut::<ClassDefs>();
    let mut chainSubClassSetCnt: TableId = 0;
    let mut totalRules: TableId = 0;
    let mut jj: TableId = 0;
    if !(tableLength < offset.wrapping_add(8 as u32)) {
        cds = ::core::ptr::null_mut::<ClassDefs>();
        cds = __caryll_allocate_clean(
            ::core::mem::size_of::<ClassDefs>() as usize,
            172 as ::core::ffi::c_ulong,
        ) as *mut ClassDefs;
        (*cds).bc = ::core::ptr::null_mut::<ClassDef>();
        (*cds).ic = readClassDef(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        (*cds).fc = ::core::ptr::null_mut::<ClassDef>();
        chainSubClassSetCnt = read_16u(
            data.offset(offset as isize)
                .offset(6 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if !(tableLength
            < offset.wrapping_add(12 as u32).wrapping_add(
                (2 as ::core::ffi::c_int * chainSubClassSetCnt as ::core::ffi::c_int) as u32,
            ))
        {
            totalRules = 0 as TableId;
            let mut j: TableId = 0 as TableId;
            while (j as ::core::ffi::c_int) < chainSubClassSetCnt as ::core::ffi::c_int {
                let mut srcOffset: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if srcOffset != 0 {
                    totalRules = (totalRules as ::core::ffi::c_int
                        + read_16u(data.offset(offset as isize).offset(srcOffset as isize)
                            as *const u8) as ::core::ffi::c_int)
                        as TableId;
                }
                j = j.wrapping_add(1);
            }
            (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = totalRules;
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut ChainingRule>() as usize)
                    .wrapping_mul(totalRules as usize),
                186 as ::core::ffi::c_ulong,
            )
                as *mut *mut ChainingRule;
            jj = 0 as TableId;
            let mut j_0: TableId = 0 as TableId;
            while (j_0 as ::core::ffi::c_int) < chainSubClassSetCnt as ::core::ffi::c_int {
                let mut srcOffset_0: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if srcOffset_0 != 0 {
                    let mut srsCount: TableId =
                        read_16u(data.offset(offset as isize).offset(srcOffset_0 as isize)
                            as *const u8) as TableId;
                    let mut k: TableId = 0 as TableId;
                    while (k as ::core::ffi::c_int) < srsCount as ::core::ffi::c_int {
                        let mut srOffset: u32 = offset.wrapping_add(srcOffset_0).wrapping_add(
                            read_16u(
                                data.offset(offset as isize)
                                    .offset(srcOffset_0 as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32,
                        );
                        let ref mut fresh20 = *(*subtable)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .rules
                            .offset(jj as isize);
                        *fresh20 = GeneralReadContextualRule(
                            data,
                            tableLength,
                            srOffset,
                            j_0 as u16,
                            true,
                            Some(
                                classCoverage
                                    as unsafe extern "C" fn(
                                        FontFilePointer,
                                        u32,
                                        u16,
                                        u32,
                                        u16,
                                        GlyphId,
                                        *mut ::core::ffi::c_void,
                                    )
                                        -> *mut Coverage,
                            ),
                            maxGlyphs,
                            cds as *mut ::core::ffi::c_void,
                        );
                        jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
                        k = k.wrapping_add(1);
                    }
                }
                j_0 = j_0.wrapping_add(1);
            }
            if !cds.is_null() {
                if !(*cds).bc.is_null() {
                    otl_ClassDef_free((*cds).bc);
                }
                if !(*cds).ic.is_null() {
                    otl_ClassDef_free((*cds).ic);
                }
                if !(*cds).fc.is_null() {
                    otl_ClassDef_free((*cds).fc);
                }
                free(cds as *mut ::core::ffi::c_void);
                cds = ::core::ptr::null_mut::<ClassDefs>();
            }
            return subtable;
        }
    }
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<ChainingSubtable>();
}
pub unsafe extern "C" fn otl_read_contextual(
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: GlyphId,
    mut options: *const Options,
) -> *mut Subtable {
    let mut format: u16 = 0 as u16;
    let mut subtable: *mut ChainingSubtable =
        (
            I_SUBTABLE_CHAINING
                .create
                .expect("non-null function pointer"))();
    (*subtable).type_0 = ChainingType::Poly;
    if !(tableLength < offset.wrapping_add(2 as u32)) {
        format = read_16u(data.offset(offset as isize) as *const u8);
        if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            return readContextualFormat1(subtable, data, tableLength, offset, maxGlyphs)
                as *mut Subtable;
        } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            return readContextualFormat2(subtable, data, tableLength, offset, maxGlyphs)
                as *mut Subtable;
        } else if format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = 1 as TableId;
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut ChainingRule>() as usize)
                    .wrapping_mul(1 as usize),
                231 as ::core::ffi::c_ulong,
            )
                as *mut *mut ChainingRule;
            let ref mut fresh15 = *(*subtable)
                .c2rust_unnamed
                .c2rust_unnamed
                .rules
                .offset(0 as ::core::ffi::c_int as isize);
            *fresh15 = GeneralReadContextualRule(
                data,
                tableLength,
                offset.wrapping_add(2 as u32),
                0 as u16,
                false,
                Some(
                    format3Coverage
                        as unsafe extern "C" fn(
                            FontFilePointer,
                            u32,
                            u16,
                            u32,
                            u16,
                            GlyphId,
                            *mut ::core::ffi::c_void,
                        ) -> *mut Coverage,
                ),
                maxGlyphs,
                NULL,
            );
            return subtable as *mut Subtable;
        }
    }
    (*(*options).logger)
        .logSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        LOG_VL_IMPORTANT,
        LoggerType::Warning,
        crate::sdsbuild!(sdsempty(), b"Unsupported format ", format as ::core::ffi::c_int, b".\n"),
    );
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn GeneralReadChainingRule(
    mut data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    mut startGID: u16,
    mut minusOne: bool,
    mut fn_0: CoverageReaderHandler,
    maxGlyphs: GlyphId,
    mut userdata: *mut ::core::ffi::c_void,
) -> *mut ChainingRule {
    let mut nBack: TableId = 0;
    let mut nInput: TableId = 0;
    let mut nLookaround: TableId = 0;
    let mut nApply: TableId = 0;
    let mut jj: TableId = 0;
    let mut rule: *mut ChainingRule = ::core::ptr::null_mut::<ChainingRule>();
    rule = __caryll_allocate_clean(
        ::core::mem::size_of::<ChainingRule>() as usize,
        247 as ::core::ffi::c_ulong,
    ) as *mut ChainingRule;
    (*rule).match_0 = ::core::ptr::null_mut::<*mut Coverage>();
    (*rule).apply = ::core::ptr::null_mut::<ChainLookupApplication>();
    let mut minusOneQ: u16 = (if minusOne as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as u16;
    if !(tableLength < offset.wrapping_add(8 as u32)) {
        nBack = read_16u(data.offset(offset as isize) as *const u8) as TableId;
        if !(tableLength
            < offset
                .wrapping_add(2 as u32)
                .wrapping_add((2 as ::core::ffi::c_int * nBack as ::core::ffi::c_int) as u32)
                .wrapping_add(2 as u32))
        {
            nInput = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset((2 as ::core::ffi::c_int * nBack as ::core::ffi::c_int) as isize)
                    as *const u8,
            ) as TableId;
            if !(tableLength
                < offset
                    .wrapping_add(4 as u32)
                    .wrapping_add(
                        (2 as ::core::ffi::c_int
                            * (nBack as ::core::ffi::c_int + nInput as ::core::ffi::c_int
                                - minusOneQ as ::core::ffi::c_int))
                            as u32,
                    )
                    .wrapping_add(2 as u32))
            {
                nLookaround = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(
                            (2 as ::core::ffi::c_int
                                * (nBack as ::core::ffi::c_int + nInput as ::core::ffi::c_int
                                    - minusOneQ as ::core::ffi::c_int))
                                as isize,
                        ) as *const u8,
                ) as TableId;
                if !(tableLength
                    < offset
                        .wrapping_add(6 as u32)
                        .wrapping_add(
                            (2 as ::core::ffi::c_int
                                * (nBack as ::core::ffi::c_int + nInput as ::core::ffi::c_int
                                    - minusOneQ as ::core::ffi::c_int
                                    + nLookaround as ::core::ffi::c_int))
                                as u32,
                        )
                        .wrapping_add(2 as u32))
                {
                    nApply = read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * (nBack as ::core::ffi::c_int + nInput as ::core::ffi::c_int
                                        - minusOneQ as ::core::ffi::c_int
                                        + nLookaround as ::core::ffi::c_int))
                                    as isize,
                            ) as *const u8,
                    ) as TableId;
                    if !(tableLength
                        < offset
                            .wrapping_add(8 as u32)
                            .wrapping_add(
                                (2 as ::core::ffi::c_int
                                    * (nBack as ::core::ffi::c_int + nInput as ::core::ffi::c_int
                                        - minusOneQ as ::core::ffi::c_int
                                        + nLookaround as ::core::ffi::c_int))
                                    as u32,
                            )
                            .wrapping_add(
                                (nApply as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                    as u32,
                            ))
                    {
                        (*rule).matchCount = (nBack as ::core::ffi::c_int
                            + nInput as ::core::ffi::c_int
                            + nLookaround as ::core::ffi::c_int)
                            as TableId;
                        (*rule).inputBegins = nBack;
                        (*rule).inputEnds = (nBack as ::core::ffi::c_int
                            + nInput as ::core::ffi::c_int)
                            as TableId;
                        (*rule).match_0 = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut Coverage>() as usize)
                                .wrapping_mul((*rule).matchCount as usize),
                            267 as ::core::ffi::c_ulong,
                        ) as *mut *mut Coverage;
                        jj = 0 as TableId;
                        let mut j: TableId = 0 as TableId;
                        while (j as ::core::ffi::c_int) < nBack as ::core::ffi::c_int {
                            let mut gid: u32 = read_16u(
                                data.offset(offset as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32;
                            let fresh1 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh2 = *(*rule).match_0.offset(fresh1 as isize);
                            *fresh2 = fn_0.expect("non-null function pointer")(
                                data,
                                tableLength,
                                gid as u16,
                                offset,
                                1 as u16,
                                maxGlyphs,
                                userdata,
                            );
                            j = j.wrapping_add(1);
                        }
                        if minusOne {
                            let fresh3 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh4 = *(*rule).match_0.offset(fresh3 as isize);
                            *fresh4 = fn_0.expect("non-null function pointer")(
                                data,
                                tableLength,
                                startGID,
                                offset,
                                2 as u16,
                                maxGlyphs,
                                userdata,
                            );
                        }
                        let mut j_0: TableId = 0 as TableId;
                        while (j_0 as ::core::ffi::c_int)
                            < nInput as ::core::ffi::c_int - minusOneQ as ::core::ffi::c_int
                        {
                            let mut gid_0: u32 = read_16u(
                                data.offset(offset as isize)
                                    .offset(4 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (2 as ::core::ffi::c_int
                                            * (*rule).inputBegins as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32;
                            let fresh5 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh6 = *(*rule).match_0.offset(fresh5 as isize);
                            *fresh6 = fn_0.expect("non-null function pointer")(
                                data,
                                tableLength,
                                gid_0 as u16,
                                offset,
                                2 as u16,
                                maxGlyphs,
                                userdata,
                            );
                            j_0 = j_0.wrapping_add(1);
                        }
                        let mut j_1: TableId = 0 as TableId;
                        while (j_1 as ::core::ffi::c_int) < nLookaround as ::core::ffi::c_int {
                            let mut gid_1: u32 = read_16u(
                                data.offset(offset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (2 as ::core::ffi::c_int
                                            * ((*rule).inputEnds as ::core::ffi::c_int
                                                - minusOneQ as ::core::ffi::c_int))
                                            as isize,
                                    )
                                    .offset(
                                        (j_1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32;
                            let fresh7 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh8 = *(*rule).match_0.offset(fresh7 as isize);
                            *fresh8 = fn_0.expect("non-null function pointer")(
                                data,
                                tableLength,
                                gid_1 as u16,
                                offset,
                                3 as u16,
                                maxGlyphs,
                                userdata,
                            );
                            j_1 = j_1.wrapping_add(1);
                        }
                        (*rule).applyCount = nApply;
                        (*rule).apply = __caryll_allocate_clean(
                            (::core::mem::size_of::<ChainLookupApplication>() as usize)
                                .wrapping_mul((*rule).applyCount as usize),
                            285 as ::core::ffi::c_ulong,
                        )
                            as *mut ChainLookupApplication;
                        let mut j_2: TableId = 0 as TableId;
                        while (j_2 as ::core::ffi::c_int) < nApply as ::core::ffi::c_int {
                            (*(*rule).apply.offset(j_2 as isize)).index = ((*rule).inputBegins
                                as ::core::ffi::c_int
                                + read_16u(
                                    data.offset(offset as isize)
                                        .offset(8 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int
                                                * ((*rule).matchCount as ::core::ffi::c_int
                                                    - minusOneQ as ::core::ffi::c_int))
                                                as isize,
                                        )
                                        .offset(
                                            (j_2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                ) as ::core::ffi::c_int)
                                as TableId;
                            (*(*rule).apply.offset(j_2 as isize)).lookup =
                                handle_fromIndex(
                                    read_16u(
                                        data.offset(offset as isize)
                                            .offset(8 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (2 as ::core::ffi::c_int
                                                    * ((*rule).matchCount as ::core::ffi::c_int
                                                        - minusOneQ as ::core::ffi::c_int))
                                                    as isize,
                                            )
                                            .offset(
                                                (j_2 as ::core::ffi::c_int
                                                    * 4 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8,
                                    ) as GlyphId,
                                ) as LookupHandle;
                            j_2 = j_2.wrapping_add(1);
                        }
                        reverseBacktracks(rule);
                        return rule;
                    }
                }
            }
        }
    }
    deleteRule(rule);
    rule = ::core::ptr::null_mut::<ChainingRule>();
    return ::core::ptr::null_mut::<ChainingRule>();
}
unsafe extern "C" fn readChainingFormat1(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: GlyphId,
) -> *mut ChainingSubtable {
    let mut covOffset: u16 = 0;
    let mut firstCoverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut chainSubRuleSetCount: TableId = 0;
    let mut totalRules: TableId = 0;
    let mut jj: TableId = 0;
    let mut current_block: u64;
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        covOffset = offset.wrapping_add(read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as u32) as u16;
        firstCoverage = readCoverage(
            data as *const u8,
            tableLength,
            covOffset as u32,
        );
        chainSubRuleSetCount = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if !(chainSubRuleSetCount as ::core::ffi::c_int
            != (*firstCoverage).numGlyphs as ::core::ffi::c_int)
        {
            if !(tableLength
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (2 as ::core::ffi::c_int * chainSubRuleSetCount as ::core::ffi::c_int)
                        as u32,
                ))
            {
                totalRules = 0 as TableId;
                let mut j: TableId = 0 as TableId;
                loop {
                    if !((j as ::core::ffi::c_int) < chainSubRuleSetCount as ::core::ffi::c_int) {
                        current_block = 4166486009154926805;
                        break;
                    }
                    let mut srsOffset: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    )
                        as u32);
                    if tableLength < srsOffset.wrapping_add(2 as u32) {
                        current_block = 17398460390698728049;
                        break;
                    }
                    totalRules = (totalRules as ::core::ffi::c_int
                        + read_16u(data.offset(srsOffset as isize) as *const u8)
                            as ::core::ffi::c_int) as TableId;
                    if tableLength
                        < srsOffset.wrapping_add(2 as u32).wrapping_add(
                            (2 as ::core::ffi::c_int
                                * read_16u(data.offset(srsOffset as isize) as *const u8)
                                    as ::core::ffi::c_int) as u32,
                        )
                    {
                        current_block = 17398460390698728049;
                        break;
                    }
                    j = j.wrapping_add(1);
                }
                match current_block {
                    17398460390698728049 => {}
                    _ => {
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = totalRules;
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut ChainingRule>() as usize)
                                .wrapping_mul(totalRules as usize),
                            321 as ::core::ffi::c_ulong,
                        )
                            as *mut *mut ChainingRule;
                        jj = 0 as TableId;
                        let mut j_0: TableId = 0 as TableId;
                        while (j_0 as ::core::ffi::c_int)
                            < chainSubRuleSetCount as ::core::ffi::c_int
                        {
                            let mut srsOffset_0: u32 = offset.wrapping_add(read_16u(
                                data.offset(offset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as u32);
                            let mut srsCount: TableId =
                                read_16u(data.offset(srsOffset_0 as isize) as *const u8)
                                    as TableId;
                            let mut k: TableId = 0 as TableId;
                            while (k as ::core::ffi::c_int) < srsCount as ::core::ffi::c_int {
                                let mut srOffset: u32 = srsOffset_0.wrapping_add(read_16u(
                                    data.offset(srsOffset_0 as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let ref mut fresh14 = *(*subtable)
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .rules
                                    .offset(jj as isize);
                                *fresh14 = GeneralReadChainingRule(
                                    data,
                                    tableLength,
                                    srOffset,
                                    (*(*firstCoverage).glyphs.offset(j_0 as isize)).index
                                        as u16,
                                    true,
                                    Some(
                                        singleCoverage
                                            as unsafe extern "C" fn(
                                                FontFilePointer,
                                                u32,
                                                u16,
                                                u32,
                                                u16,
                                                GlyphId,
                                                *mut ::core::ffi::c_void,
                                            )
                                                -> *mut Coverage,
                                    ),
                                    maxGlyphs,
                                    NULL,
                                );
                                jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as TableId;
                                k = k.wrapping_add(1);
                            }
                            j_0 = j_0.wrapping_add(1);
                        }
                        otl_Coverage_free(firstCoverage);
                        return subtable;
                    }
                }
            }
        }
    }
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<ChainingSubtable>();
}
unsafe extern "C" fn readChainingFormat2(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: GlyphId,
) -> *mut ChainingSubtable {
    let mut cds: *mut ClassDefs = ::core::ptr::null_mut::<ClassDefs>();
    let mut chainSubClassSetCnt: TableId = 0;
    let mut totalRules: TableId = 0;
    let mut jj: TableId = 0;
    if !(tableLength < offset.wrapping_add(12 as u32)) {
        cds = ::core::ptr::null_mut::<ClassDefs>();
        cds = __caryll_allocate_clean(
            ::core::mem::size_of::<ClassDefs>() as usize,
            349 as ::core::ffi::c_ulong,
        ) as *mut ClassDefs;
        (*cds).bc = readClassDef(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        (*cds).ic = readClassDef(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        (*cds).fc = readClassDef(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        chainSubClassSetCnt = read_16u(
            data.offset(offset as isize)
                .offset(10 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if !(tableLength
            < offset.wrapping_add(12 as u32).wrapping_add(
                (2 as ::core::ffi::c_int * chainSubClassSetCnt as ::core::ffi::c_int) as u32,
            ))
        {
            totalRules = 0 as TableId;
            let mut j: TableId = 0 as TableId;
            while (j as ::core::ffi::c_int) < chainSubClassSetCnt as ::core::ffi::c_int {
                let mut srcOffset: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(12 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if srcOffset != 0 {
                    totalRules = (totalRules as ::core::ffi::c_int
                        + read_16u(data.offset(offset as isize).offset(srcOffset as isize)
                            as *const u8) as ::core::ffi::c_int)
                        as TableId;
                }
                j = j.wrapping_add(1);
            }
            (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = totalRules;
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut ChainingRule>() as usize)
                    .wrapping_mul(totalRules as usize),
                363 as ::core::ffi::c_ulong,
            )
                as *mut *mut ChainingRule;
            jj = 0 as TableId;
            let mut j_0: TableId = 0 as TableId;
            while (j_0 as ::core::ffi::c_int) < chainSubClassSetCnt as ::core::ffi::c_int {
                let mut srcOffset_0: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(12 as ::core::ffi::c_int as isize)
                        .offset((j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if srcOffset_0 != 0 {
                    let mut srsCount: TableId =
                        read_16u(data.offset(offset as isize).offset(srcOffset_0 as isize)
                            as *const u8) as TableId;
                    let mut k: TableId = 0 as TableId;
                    while (k as ::core::ffi::c_int) < srsCount as ::core::ffi::c_int {
                        let mut dsrOffset: u32 = read_16u(
                            data.offset(offset as isize)
                                .offset(srcOffset_0 as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(
                                    (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        ) as u32;
                        let mut srOffset: u32 =
                            offset.wrapping_add(srcOffset_0).wrapping_add(dsrOffset);
                        let ref mut fresh11 = *(*subtable)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .rules
                            .offset(jj as isize);
                        *fresh11 = GeneralReadChainingRule(
                            data,
                            tableLength,
                            srOffset,
                            j_0 as u16,
                            true,
                            Some(
                                classCoverage
                                    as unsafe extern "C" fn(
                                        FontFilePointer,
                                        u32,
                                        u16,
                                        u32,
                                        u16,
                                        GlyphId,
                                        *mut ::core::ffi::c_void,
                                    )
                                        -> *mut Coverage,
                            ),
                            maxGlyphs,
                            cds as *mut ::core::ffi::c_void,
                        );
                        jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
                        k = k.wrapping_add(1);
                    }
                }
                j_0 = j_0.wrapping_add(1);
            }
            if !cds.is_null() {
                if !(*cds).bc.is_null() {
                    otl_ClassDef_free((*cds).bc);
                }
                if !(*cds).ic.is_null() {
                    otl_ClassDef_free((*cds).ic);
                }
                if !(*cds).fc.is_null() {
                    otl_ClassDef_free((*cds).fc);
                }
                free(cds as *mut ::core::ffi::c_void);
                cds = ::core::ptr::null_mut::<ClassDefs>();
            }
            return subtable;
        }
    }
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<ChainingSubtable>();
}
pub unsafe extern "C" fn otl_read_chaining(
    data: FontFilePointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: GlyphId,
    mut options: *const Options,
) -> *mut Subtable {
    let mut format: u16 = 0 as u16;
    let mut subtable: *mut ChainingSubtable =
        (
            I_SUBTABLE_CHAINING
                .create
                .expect("non-null function pointer"))();
    (*subtable).type_0 = ChainingType::Poly;
    if !(tableLength < offset.wrapping_add(2 as u32)) {
        format = read_16u(data.offset(offset as isize) as *const u8);
        if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            return readChainingFormat1(subtable, data, tableLength, offset, maxGlyphs)
                as *mut Subtable;
        } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            return readChainingFormat2(subtable, data, tableLength, offset, maxGlyphs)
                as *mut Subtable;
        } else if format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = 1 as TableId;
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut ChainingRule>() as usize)
                    .wrapping_mul(1 as usize),
                407 as ::core::ffi::c_ulong,
            )
                as *mut *mut ChainingRule;
            let ref mut fresh0 = *(*subtable)
                .c2rust_unnamed
                .c2rust_unnamed
                .rules
                .offset(0 as ::core::ffi::c_int as isize);
            *fresh0 = GeneralReadChainingRule(
                data,
                tableLength,
                offset.wrapping_add(2 as u32),
                0 as u16,
                false,
                Some(
                    format3Coverage
                        as unsafe extern "C" fn(
                            FontFilePointer,
                            u32,
                            u16,
                            u32,
                            u16,
                            GlyphId,
                            *mut ::core::ffi::c_void,
                        ) -> *mut Coverage,
                ),
                maxGlyphs,
                NULL,
            );
            return subtable as *mut Subtable;
        }
    }
    (*(*options).logger)
        .logSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        LOG_VL_IMPORTANT,
        LoggerType::Warning,
        crate::sdsbuild!(sdsempty(), b"Unsupported format ", format as ::core::ffi::c_int, b".\n"),
    );
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
#[inline]
unsafe extern "C" fn closeRule(mut rule: *mut ChainingRule) {
    if !rule.is_null()
        && !(*rule).match_0.is_null()
        && (*rule).matchCount as ::core::ffi::c_int != 0
    {
        let mut k: TableId = 0 as TableId;
        while (k as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
            otl_Coverage_free(
                *(*rule).match_0.offset(k as isize),
            );
            k = k.wrapping_add(1);
        }
        free((*rule).match_0 as *mut ::core::ffi::c_void);
        (*rule).match_0 = ::core::ptr::null_mut::<*mut Coverage>();
    }
    if !rule.is_null() && !(*rule).apply.is_null() {
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < (*rule).applyCount as ::core::ffi::c_int {
            otfcc_Handle_dispose(
                &raw mut (*(*rule).apply.offset(j as isize)).lookup,
            );
            j = j.wrapping_add(1);
        }
        free((*rule).apply as *mut ::core::ffi::c_void);
        (*rule).apply = ::core::ptr::null_mut::<ChainLookupApplication>();
    }
}
#[inline]
unsafe extern "C" fn deleteRule(mut rule: *mut ChainingRule) {
    if rule.is_null() {
        return;
    }
    closeRule(rule);
    free(rule as *mut ::core::ffi::c_void);
    rule = ::core::ptr::null_mut::<ChainingRule>();
}
#[inline]
unsafe extern "C" fn reverseBacktracks(mut rule: *mut ChainingRule) {
    if (*rule).inputBegins as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut start: TableId = 0 as TableId;
        let mut end: TableId =
            ((*rule).inputBegins as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as TableId;
        while end as ::core::ffi::c_int > start as ::core::ffi::c_int {
            let mut tmp: *mut Coverage = *(*rule).match_0.offset(start as isize);
            let ref mut fresh9 = *(*rule).match_0.offset(start as isize);
            *fresh9 = *(*rule).match_0.offset(end as isize);
            let ref mut fresh10 = *(*rule).match_0.offset(end as isize);
            *fresh10 = tmp;
            end = end.wrapping_sub(1);
            start = start.wrapping_add(1);
        }
    }
}
