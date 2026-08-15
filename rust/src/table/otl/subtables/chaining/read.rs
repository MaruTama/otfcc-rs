#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};

use crate::table::otl::classdef::{ClassDef, otl_class_def_free, read_class_def};
use crate::table::otl::coverage::{Coverage, coverage_from_raw, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, otfcc_handle_dup, Handle, GlyphHandle, LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};

use crate::support::{NULL};
use crate::table::otl::{ChainLookupApplication, ChainingRule, ChainingRuleSet, Subtable, ChainingSubtable, subtable_from_raw};
use crate::table::otl::subtables::chaining::common::{I_SUBTABLE_CHAINING, chaining_ruleset_mut};
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
pub unsafe extern "C" fn single_coverage(
    mut _data: FontFilePointer,
    mut _table_length: u32,
    mut gid: u16,
    mut _offset: u32,
    mut _kind: u16,
    _max_glyphs: GlyphId,
    mut _userdata: *mut ::core::ffi::c_void,
) -> *mut Coverage {
    let cov: *mut Coverage = otl_coverage_create();
    push_to_coverage(cov, handle_from_index(gid) as GlyphHandle);
    return cov;
}
pub unsafe extern "C" fn class_coverage(
    mut _data: FontFilePointer,
    mut _table_length: u32,
    mut cls: u16,
    mut _offset: u32,
    mut kind: u16,
    max_glyphs: GlyphId,
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
    let cov: *mut Coverage = otl_coverage_create();
    let mut count: GlyphId = 0 as GlyphId;
    if cls as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int) < max_glyphs as ::core::ffi::c_int {
            let mut found: bool = false;
            let mut j: GlyphId = 0 as GlyphId;
            while (j as usize) < (*cd).glyphs.len() {
                if (&(*cd).classes)[j as usize] as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && (&(*cd).glyphs)[j as usize].index as ::core::ffi::c_int
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
        while (j_0 as usize) < (*cd).glyphs.len() {
            if (&(*cd).classes)[j_0 as usize] as ::core::ffi::c_int
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
    if cls as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut k_0: GlyphId = 0 as GlyphId;
        while (k_0 as ::core::ffi::c_int) < max_glyphs as ::core::ffi::c_int {
            let mut found_0: bool = false;
            let mut j_1: GlyphId = 0 as GlyphId;
            while (j_1 as usize) < (*cd).glyphs.len() {
                if (&(*cd).classes)[j_1 as usize] as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                    && (&(*cd).glyphs)[j_1 as usize].index as ::core::ffi::c_int
                        == k_0 as ::core::ffi::c_int
                {
                    found_0 = true;
                    break;
                } else {
                    j_1 = j_1.wrapping_add(1);
                }
            }
            if !found_0 {
                push_to_coverage(cov, handle_from_index(k_0) as GlyphHandle);
            }
            k_0 = k_0.wrapping_add(1);
        }
    } else {
        let mut j_2: GlyphId = 0 as GlyphId;
        while (j_2 as usize) < (*cd).glyphs.len() {
            if (&(*cd).classes)[j_2 as usize] as ::core::ffi::c_int
                == cls as ::core::ffi::c_int
            {
                push_to_coverage(
                    cov,
                    otfcc_handle_dup((&(*cd).glyphs)[j_2 as usize].clone() as Handle) as GlyphHandle,
                );
            }
            j_2 = j_2.wrapping_add(1);
        }
    }
    return cov;
}
pub unsafe extern "C" fn format3_coverage(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut shift: u16,
    mut _offset: u32,
    mut _kind: u16,
    _max_glyphs: GlyphId,
    mut _userdata: *mut ::core::ffi::c_void,
) -> *mut Coverage {
    return read_coverage(
        data as *const u8,
        table_length,
        _offset
            .wrapping_add(shift as u32)
            .wrapping_sub(2 as u32),
    );
}
pub unsafe extern "C" fn general_read_contextual_rule(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    mut start_gid: u16,
    mut minus_one: bool,
    mut fn_0: CoverageReaderHandler,
    max_glyphs: GlyphId,
    mut userdata: *mut ::core::ffi::c_void,
) -> Option<Box<ChainingRule>> {
    let mut n_input: u16 = 0;
    let mut n_apply: u16 = 0;
    // `Box` is the allocation, the struct literal is the zero-init the old
    // `__caryll_allocate_clean` provided -- same shape as `new_lookup`/
    // `otfcc_new_glyf_glyph`. Every `(*rule).field` access below still
    // works unchanged through the `Box`'s `Deref`/`DerefMut`.
    let mut rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: 0 as TableId,
        input_begins: 0 as TableId,
        input_ends: 0 as TableId,
        match_0: Vec::new(),
        apply: Vec::new(),
    });
    let mut minus_one_q: u16 = (if minus_one as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as u16;
    if !(table_length < offset.wrapping_add(4 as u32)) {
        n_input = read_16u(data.offset(offset as isize) as *const u8);
        n_apply = read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        );
        if !(table_length
            < offset
                .wrapping_add(4 as u32)
                .wrapping_add((2 as ::core::ffi::c_int * n_input as ::core::ffi::c_int) as u32)
                .wrapping_add((4 as ::core::ffi::c_int * n_apply as ::core::ffi::c_int) as u32))
        {
            (*rule).match_count = n_input as TableId;
            (*rule).input_begins = 0 as TableId;
            (*rule).input_ends = n_input as TableId;
            // Filled in order below (the `minus_one` slot first, then the
            // rest sequentially) -- every one of the `match_count` slots is
            // written exactly once, in increasing index order, so `.push()`
            // is the direct replacement for the old `jj`-indexed writes into
            // `__caryll_allocate_clean`'d memory (`jj` itself is gone: it
            // was only ever used as that index).
            (*rule).match_0 = Vec::with_capacity((*rule).match_count as usize);
            if minus_one {
                (*rule).match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                    data,
                    table_length,
                    start_gid,
                    offset,
                    2 as u16,
                    max_glyphs,
                    userdata,
                )));
            }
            let mut j: u16 = 0 as u16;
            while (j as ::core::ffi::c_int)
                < n_input as ::core::ffi::c_int - minus_one_q as ::core::ffi::c_int
            {
                let mut gid: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                (*rule).match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                    data,
                    table_length,
                    gid as u16,
                    offset,
                    2 as u16,
                    max_glyphs,
                    userdata,
                )));
                j = j.wrapping_add(1);
            }
            (*rule).apply = Vec::with_capacity(n_apply as usize);
            let mut j_0: TableId = 0 as TableId;
            while (j_0 as ::core::ffi::c_int) < n_apply as ::core::ffi::c_int {
                let index = ((*rule).input_begins as ::core::ffi::c_int
                    + read_16u(
                        data.offset(offset as isize)
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * ((*rule).match_count as ::core::ffi::c_int
                                        - minus_one_q as ::core::ffi::c_int))
                                    as isize,
                            )
                            .offset((j_0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    ) as ::core::ffi::c_int)
                    as TableId;
                let lookup = handle_from_index(read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(
                            (2 as ::core::ffi::c_int
                                * ((*rule).match_count as ::core::ffi::c_int
                                    - minus_one_q as ::core::ffi::c_int))
                                as isize,
                        )
                        .offset((j_0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const u8,
                )
                    as GlyphId) as LookupHandle;
                (*rule).apply.push(ChainLookupApplication { index, lookup });
                j_0 = j_0.wrapping_add(1);
            }
            reverse_backtracks(&mut *rule as *mut ChainingRule);
            return Some(rule);
        }
    }
    // `rule` (whatever partial state it reached) drops here automatically --
    // both fields self-drop now, no manual `delete_rule` call needed.
    return None;
}
unsafe extern "C" fn read_contextual_format1(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let mut cov_offset: u16 = 0;
    let mut first_coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut chain_sub_rule_set_count: TableId = 0;
    let mut total_rules: TableId = 0;
    let mut current_block: u64;
    if !(table_length < offset.wrapping_add(6 as u32)) {
        cov_offset = offset.wrapping_add(read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as u32) as u16;
        first_coverage = read_coverage(
            data as *const u8,
            table_length,
            cov_offset as u32,
        );
        chain_sub_rule_set_count = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if !(chain_sub_rule_set_count as ::core::ffi::c_int
            != (*first_coverage).len() as ::core::ffi::c_int)
        {
            if !(table_length
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (2 as ::core::ffi::c_int * chain_sub_rule_set_count as ::core::ffi::c_int)
                        as u32,
                ))
            {
                total_rules = 0 as TableId;
                let mut j: TableId = 0 as TableId;
                loop {
                    if !((j as ::core::ffi::c_int) < chain_sub_rule_set_count as ::core::ffi::c_int) {
                        current_block = 4166486009154926805;
                        break;
                    }
                    let mut srs_offset: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    )
                        as u32);
                    if table_length < srs_offset.wrapping_add(2 as u32) {
                        current_block = 10321976752019472029;
                        break;
                    }
                    total_rules = (total_rules as ::core::ffi::c_int
                        + read_16u(data.offset(srs_offset as isize) as *const u8)
                            as ::core::ffi::c_int) as TableId;
                    if table_length
                        < srs_offset.wrapping_add(2 as u32).wrapping_add(
                            (2 as ::core::ffi::c_int
                                * read_16u(data.offset(srs_offset as isize) as *const u8)
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
                        let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
                        (*ruleset).rules = Vec::with_capacity(total_rules as usize);
                        let mut j_0: TableId = 0 as TableId;
                        while (j_0 as ::core::ffi::c_int)
                            < chain_sub_rule_set_count as ::core::ffi::c_int
                        {
                            let mut srs_offset_0: u32 = offset.wrapping_add(read_16u(
                                data.offset(offset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as u32);
                            let mut srs_count: TableId =
                                read_16u(data.offset(srs_offset_0 as isize) as *const u8)
                                    as TableId;
                            let mut k: TableId = 0 as TableId;
                            while (k as ::core::ffi::c_int) < srs_count as ::core::ffi::c_int {
                                let mut sr_offset: u32 = srs_offset_0.wrapping_add(read_16u(
                                    data.offset(srs_offset_0 as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let rule_ptr = general_read_contextual_rule(
                                    data,
                                    table_length,
                                    sr_offset,
                                    (&(*first_coverage))[j_0 as usize].index
                                        as u16,
                                    true,
                                    Some(
                                        single_coverage
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
                                    max_glyphs,
                                    NULL,
                                );
                                (*ruleset).rules.push(rule_ptr);
                                k = k.wrapping_add(1);
                            }
                            j_0 = j_0.wrapping_add(1);
                        }
                        otl_coverage_free(first_coverage);
                        return subtable;
                    }
                }
            }
        }
    }
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<ChainingSubtable>();
}
unsafe extern "C" fn read_contextual_format2(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let mut cds: *mut ClassDefs = ::core::ptr::null_mut::<ClassDefs>();
    let mut chain_sub_class_set_cnt: TableId = 0;
    let mut total_rules: TableId = 0;
    if !(table_length < offset.wrapping_add(8 as u32)) {
        cds = ::core::ptr::null_mut::<ClassDefs>();
        cds = __caryll_allocate_clean(
            ::core::mem::size_of::<ClassDefs>() as usize,
            172 as ::core::ffi::c_ulong,
        ) as *mut ClassDefs;
        (*cds).bc = ::core::ptr::null_mut::<ClassDef>();
        (*cds).ic = read_class_def(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        (*cds).fc = ::core::ptr::null_mut::<ClassDef>();
        chain_sub_class_set_cnt = read_16u(
            data.offset(offset as isize)
                .offset(6 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if !(table_length
            < offset.wrapping_add(12 as u32).wrapping_add(
                (2 as ::core::ffi::c_int * chain_sub_class_set_cnt as ::core::ffi::c_int) as u32,
            ))
        {
            total_rules = 0 as TableId;
            let mut j: TableId = 0 as TableId;
            while (j as ::core::ffi::c_int) < chain_sub_class_set_cnt as ::core::ffi::c_int {
                let mut src_offset: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if src_offset != 0 {
                    total_rules = (total_rules as ::core::ffi::c_int
                        + read_16u(data.offset(offset as isize).offset(src_offset as isize)
                            as *const u8) as ::core::ffi::c_int)
                        as TableId;
                }
                j = j.wrapping_add(1);
            }
            let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
            (*ruleset).rules = Vec::with_capacity(total_rules as usize);
            let mut j_0: TableId = 0 as TableId;
            while (j_0 as ::core::ffi::c_int) < chain_sub_class_set_cnt as ::core::ffi::c_int {
                let mut src_offset_0: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if src_offset_0 != 0 {
                    let mut srs_count: TableId =
                        read_16u(data.offset(offset as isize).offset(src_offset_0 as isize)
                            as *const u8) as TableId;
                    let mut k: TableId = 0 as TableId;
                    while (k as ::core::ffi::c_int) < srs_count as ::core::ffi::c_int {
                        let mut sr_offset: u32 = offset.wrapping_add(src_offset_0).wrapping_add(
                            read_16u(
                                data.offset(offset as isize)
                                    .offset(src_offset_0 as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32,
                        );
                        let rule_ptr = general_read_contextual_rule(
                            data,
                            table_length,
                            sr_offset,
                            j_0 as u16,
                            true,
                            Some(
                                class_coverage
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
                            max_glyphs,
                            cds as *mut ::core::ffi::c_void,
                        );
                        (*ruleset).rules.push(rule_ptr);
                        k = k.wrapping_add(1);
                    }
                }
                j_0 = j_0.wrapping_add(1);
            }
            if !cds.is_null() {
                if !(*cds).bc.is_null() {
                    otl_class_def_free((*cds).bc);
                }
                if !(*cds).ic.is_null() {
                    otl_class_def_free((*cds).ic);
                }
                if !(*cds).fc.is_null() {
                    otl_class_def_free((*cds).fc);
                }
                free(cds as *mut ::core::ffi::c_void);
                cds = ::core::ptr::null_mut::<ClassDefs>();
            }
            return subtable;
        }
    }
    // `cds` (and its populated `.ic`, from the first length check passing
    // above) leaked here on this malformed-input path: falling through to
    // the failure return below skipped the same cleanup the success path
    // just above already does. Same fix as `read_chaining_format2`'s
    // sibling leak.
    if !cds.is_null() {
        if !(*cds).bc.is_null() {
            otl_class_def_free((*cds).bc);
        }
        if !(*cds).ic.is_null() {
            otl_class_def_free((*cds).ic);
        }
        if !(*cds).fc.is_null() {
            otl_class_def_free((*cds).fc);
        }
        free(cds as *mut ::core::ffi::c_void);
        cds = ::core::ptr::null_mut::<ClassDefs>();
    }
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<ChainingSubtable>();
}
pub unsafe extern "C" fn otl_read_contextual(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
    mut options: *const Options,
) -> *mut Subtable {
    let mut format: u16 = 0 as u16;
    let mut subtable: *mut ChainingSubtable =
        (
            I_SUBTABLE_CHAINING
                .create
                .expect("non-null function pointer"))();
    // `subtable` is fresh from `create()` (a valid, empty `Canonical`
    // value) -- replace it wholesale with a valid, empty `Poly` ruleset.
    // Every downstream construction path (format1/format2/format3, and the
    // error paths that dispose the subtable without ever reaching one) now
    // sees a valid, possibly-still-empty ruleset from this point on.
    *subtable = ChainingSubtable::Poly(ChainingRuleSet::default());
    let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
    if !(table_length < offset.wrapping_add(2 as u32)) {
        format = read_16u(data.offset(offset as isize) as *const u8);
        if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            return subtable_from_raw(
                read_contextual_format1(subtable, data, table_length, offset, max_glyphs),
                Subtable::Chaining,
            );
        } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            return subtable_from_raw(
                read_contextual_format2(subtable, data, table_length, offset, max_glyphs),
                Subtable::Chaining,
            );
        } else if format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            let rule_ptr = general_read_contextual_rule(
                data,
                table_length,
                offset.wrapping_add(2 as u32),
                0 as u16,
                false,
                Some(
                    format3_coverage
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
                max_glyphs,
                NULL,
            );
            (*ruleset).rules.push(rule_ptr);
            return subtable_from_raw(subtable, Subtable::Chaining);
        }
    }
    (*(*options).logger)
        .log_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        LOG_VL_IMPORTANT,
        LoggerType::Warning,
        crate::bytesbuild!(b"Unsupported format ", format as ::core::ffi::c_int, b".\n"),
    );
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn general_read_chaining_rule(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    mut start_gid: u16,
    mut minus_one: bool,
    mut fn_0: CoverageReaderHandler,
    max_glyphs: GlyphId,
    mut userdata: *mut ::core::ffi::c_void,
) -> Option<Box<ChainingRule>> {
    let mut n_back: TableId = 0;
    let mut n_input: TableId = 0;
    let mut n_lookaround: TableId = 0;
    let mut n_apply: TableId = 0;
    // `Box` is the allocation, the struct literal is the zero-init the old
    // `__caryll_allocate_clean` provided -- see `general_read_contextual_rule`.
    let mut rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: 0 as TableId,
        input_begins: 0 as TableId,
        input_ends: 0 as TableId,
        match_0: Vec::new(),
        apply: Vec::new(),
    });
    let mut minus_one_q: u16 = (if minus_one as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as u16;
    if !(table_length < offset.wrapping_add(8 as u32)) {
        n_back = read_16u(data.offset(offset as isize) as *const u8) as TableId;
        if !(table_length
            < offset
                .wrapping_add(2 as u32)
                .wrapping_add((2 as ::core::ffi::c_int * n_back as ::core::ffi::c_int) as u32)
                .wrapping_add(2 as u32))
        {
            n_input = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset((2 as ::core::ffi::c_int * n_back as ::core::ffi::c_int) as isize)
                    as *const u8,
            ) as TableId;
            if !(table_length
                < offset
                    .wrapping_add(4 as u32)
                    .wrapping_add(
                        (2 as ::core::ffi::c_int
                            * (n_back as ::core::ffi::c_int + n_input as ::core::ffi::c_int
                                - minus_one_q as ::core::ffi::c_int))
                            as u32,
                    )
                    .wrapping_add(2 as u32))
            {
                n_lookaround = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(
                            (2 as ::core::ffi::c_int
                                * (n_back as ::core::ffi::c_int + n_input as ::core::ffi::c_int
                                    - minus_one_q as ::core::ffi::c_int))
                                as isize,
                        ) as *const u8,
                ) as TableId;
                if !(table_length
                    < offset
                        .wrapping_add(6 as u32)
                        .wrapping_add(
                            (2 as ::core::ffi::c_int
                                * (n_back as ::core::ffi::c_int + n_input as ::core::ffi::c_int
                                    - minus_one_q as ::core::ffi::c_int
                                    + n_lookaround as ::core::ffi::c_int))
                                as u32,
                        )
                        .wrapping_add(2 as u32))
                {
                    n_apply = read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * (n_back as ::core::ffi::c_int + n_input as ::core::ffi::c_int
                                        - minus_one_q as ::core::ffi::c_int
                                        + n_lookaround as ::core::ffi::c_int))
                                    as isize,
                            ) as *const u8,
                    ) as TableId;
                    if !(table_length
                        < offset
                            .wrapping_add(8 as u32)
                            .wrapping_add(
                                (2 as ::core::ffi::c_int
                                    * (n_back as ::core::ffi::c_int + n_input as ::core::ffi::c_int
                                        - minus_one_q as ::core::ffi::c_int
                                        + n_lookaround as ::core::ffi::c_int))
                                    as u32,
                            )
                            .wrapping_add(
                                (n_apply as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                    as u32,
                            ))
                    {
                        (*rule).match_count = (n_back as ::core::ffi::c_int
                            + n_input as ::core::ffi::c_int
                            + n_lookaround as ::core::ffi::c_int)
                            as TableId;
                        (*rule).input_begins = n_back;
                        (*rule).input_ends = (n_back as ::core::ffi::c_int
                            + n_input as ::core::ffi::c_int)
                            as TableId;
                        // Filled in order below (backtrack, then the
                        // `minus_one` slot, then input, then lookaround) --
                        // every one of the `match_count` slots is written
                        // exactly once, in increasing index order, so
                        // `.push()` is the direct replacement for the old
                        // `jj`-indexed writes (`jj` itself is gone: it was
                        // only ever used as that index).
                        (*rule).match_0 = Vec::with_capacity((*rule).match_count as usize);
                        let mut j: TableId = 0 as TableId;
                        while (j as ::core::ffi::c_int) < n_back as ::core::ffi::c_int {
                            let mut gid: u32 = read_16u(
                                data.offset(offset as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32;
                            (*rule).match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                                data,
                                table_length,
                                gid as u16,
                                offset,
                                1 as u16,
                                max_glyphs,
                                userdata,
                            )));
                            j = j.wrapping_add(1);
                        }
                        if minus_one {
                            (*rule).match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                                data,
                                table_length,
                                start_gid,
                                offset,
                                2 as u16,
                                max_glyphs,
                                userdata,
                            )));
                        }
                        let mut j_0: TableId = 0 as TableId;
                        while (j_0 as ::core::ffi::c_int)
                            < n_input as ::core::ffi::c_int - minus_one_q as ::core::ffi::c_int
                        {
                            let mut gid_0: u32 = read_16u(
                                data.offset(offset as isize)
                                    .offset(4 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (2 as ::core::ffi::c_int
                                            * (*rule).input_begins as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32;
                            (*rule).match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                                data,
                                table_length,
                                gid_0 as u16,
                                offset,
                                2 as u16,
                                max_glyphs,
                                userdata,
                            )));
                            j_0 = j_0.wrapping_add(1);
                        }
                        let mut j_1: TableId = 0 as TableId;
                        while (j_1 as ::core::ffi::c_int) < n_lookaround as ::core::ffi::c_int {
                            let mut gid_1: u32 = read_16u(
                                data.offset(offset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (2 as ::core::ffi::c_int
                                            * ((*rule).input_ends as ::core::ffi::c_int
                                                - minus_one_q as ::core::ffi::c_int))
                                            as isize,
                                    )
                                    .offset(
                                        (j_1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32;
                            (*rule).match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                                data,
                                table_length,
                                gid_1 as u16,
                                offset,
                                3 as u16,
                                max_glyphs,
                                userdata,
                            )));
                            j_1 = j_1.wrapping_add(1);
                        }
                        (*rule).apply = Vec::with_capacity(n_apply as usize);
                        let mut j_2: TableId = 0 as TableId;
                        while (j_2 as ::core::ffi::c_int) < n_apply as ::core::ffi::c_int {
                            let index = ((*rule).input_begins as ::core::ffi::c_int
                                + read_16u(
                                    data.offset(offset as isize)
                                        .offset(8 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int
                                                * ((*rule).match_count as ::core::ffi::c_int
                                                    - minus_one_q as ::core::ffi::c_int))
                                                as isize,
                                        )
                                        .offset(
                                            (j_2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                ) as ::core::ffi::c_int)
                                as TableId;
                            let lookup = handle_from_index(
                                read_16u(
                                    data.offset(offset as isize)
                                        .offset(8 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int
                                                * ((*rule).match_count as ::core::ffi::c_int
                                                    - minus_one_q as ::core::ffi::c_int))
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
                            (*rule).apply.push(ChainLookupApplication { index, lookup });
                            j_2 = j_2.wrapping_add(1);
                        }
                        reverse_backtracks(&mut *rule as *mut ChainingRule);
                        return Some(rule);
                    }
                }
            }
        }
    }
    // `rule` drops here automatically -- see `general_read_contextual_rule`.
    return None;
}
unsafe extern "C" fn read_chaining_format1(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let mut cov_offset: u16 = 0;
    let mut first_coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut chain_sub_rule_set_count: TableId = 0;
    let mut total_rules: TableId = 0;
    let mut current_block: u64;
    if !(table_length < offset.wrapping_add(6 as u32)) {
        cov_offset = offset.wrapping_add(read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as u32) as u16;
        first_coverage = read_coverage(
            data as *const u8,
            table_length,
            cov_offset as u32,
        );
        chain_sub_rule_set_count = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if !(chain_sub_rule_set_count as ::core::ffi::c_int
            != (*first_coverage).len() as ::core::ffi::c_int)
        {
            if !(table_length
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (2 as ::core::ffi::c_int * chain_sub_rule_set_count as ::core::ffi::c_int)
                        as u32,
                ))
            {
                total_rules = 0 as TableId;
                let mut j: TableId = 0 as TableId;
                loop {
                    if !((j as ::core::ffi::c_int) < chain_sub_rule_set_count as ::core::ffi::c_int) {
                        current_block = 4166486009154926805;
                        break;
                    }
                    let mut srs_offset: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    )
                        as u32);
                    if table_length < srs_offset.wrapping_add(2 as u32) {
                        current_block = 17398460390698728049;
                        break;
                    }
                    total_rules = (total_rules as ::core::ffi::c_int
                        + read_16u(data.offset(srs_offset as isize) as *const u8)
                            as ::core::ffi::c_int) as TableId;
                    if table_length
                        < srs_offset.wrapping_add(2 as u32).wrapping_add(
                            (2 as ::core::ffi::c_int
                                * read_16u(data.offset(srs_offset as isize) as *const u8)
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
                        let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
                        (*ruleset).rules = Vec::with_capacity(total_rules as usize);
                        let mut j_0: TableId = 0 as TableId;
                        while (j_0 as ::core::ffi::c_int)
                            < chain_sub_rule_set_count as ::core::ffi::c_int
                        {
                            let mut srs_offset_0: u32 = offset.wrapping_add(read_16u(
                                data.offset(offset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as u32);
                            let mut srs_count: TableId =
                                read_16u(data.offset(srs_offset_0 as isize) as *const u8)
                                    as TableId;
                            let mut k: TableId = 0 as TableId;
                            while (k as ::core::ffi::c_int) < srs_count as ::core::ffi::c_int {
                                let mut sr_offset: u32 = srs_offset_0.wrapping_add(read_16u(
                                    data.offset(srs_offset_0 as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let rule_ptr = general_read_chaining_rule(
                                    data,
                                    table_length,
                                    sr_offset,
                                    (&(*first_coverage))[j_0 as usize].index
                                        as u16,
                                    true,
                                    Some(
                                        single_coverage
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
                                    max_glyphs,
                                    NULL,
                                );
                                (*ruleset).rules.push(rule_ptr);
                                k = k.wrapping_add(1);
                            }
                            j_0 = j_0.wrapping_add(1);
                        }
                        otl_coverage_free(first_coverage);
                        return subtable;
                    }
                }
            }
        }
    }
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<ChainingSubtable>();
}
unsafe extern "C" fn read_chaining_format2(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let mut cds: *mut ClassDefs = ::core::ptr::null_mut::<ClassDefs>();
    let mut chain_sub_class_set_cnt: TableId = 0;
    let mut total_rules: TableId = 0;
    if !(table_length < offset.wrapping_add(12 as u32)) {
        cds = ::core::ptr::null_mut::<ClassDefs>();
        cds = __caryll_allocate_clean(
            ::core::mem::size_of::<ClassDefs>() as usize,
            349 as ::core::ffi::c_ulong,
        ) as *mut ClassDefs;
        (*cds).bc = read_class_def(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        (*cds).ic = read_class_def(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        (*cds).fc = read_class_def(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        chain_sub_class_set_cnt = read_16u(
            data.offset(offset as isize)
                .offset(10 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if !(table_length
            < offset.wrapping_add(12 as u32).wrapping_add(
                (2 as ::core::ffi::c_int * chain_sub_class_set_cnt as ::core::ffi::c_int) as u32,
            ))
        {
            total_rules = 0 as TableId;
            let mut j: TableId = 0 as TableId;
            while (j as ::core::ffi::c_int) < chain_sub_class_set_cnt as ::core::ffi::c_int {
                let mut src_offset: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(12 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if src_offset != 0 {
                    total_rules = (total_rules as ::core::ffi::c_int
                        + read_16u(data.offset(offset as isize).offset(src_offset as isize)
                            as *const u8) as ::core::ffi::c_int)
                        as TableId;
                }
                j = j.wrapping_add(1);
            }
            let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
            (*ruleset).rules = Vec::with_capacity(total_rules as usize);
            let mut j_0: TableId = 0 as TableId;
            while (j_0 as ::core::ffi::c_int) < chain_sub_class_set_cnt as ::core::ffi::c_int {
                let mut src_offset_0: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(12 as ::core::ffi::c_int as isize)
                        .offset((j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if src_offset_0 != 0 {
                    let mut srs_count: TableId =
                        read_16u(data.offset(offset as isize).offset(src_offset_0 as isize)
                            as *const u8) as TableId;
                    let mut k: TableId = 0 as TableId;
                    while (k as ::core::ffi::c_int) < srs_count as ::core::ffi::c_int {
                        let mut dsr_offset: u32 = read_16u(
                            data.offset(offset as isize)
                                .offset(src_offset_0 as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(
                                    (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        ) as u32;
                        let mut sr_offset: u32 =
                            offset.wrapping_add(src_offset_0).wrapping_add(dsr_offset);
                        let rule_ptr = general_read_chaining_rule(
                            data,
                            table_length,
                            sr_offset,
                            j_0 as u16,
                            true,
                            Some(
                                class_coverage
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
                            max_glyphs,
                            cds as *mut ::core::ffi::c_void,
                        );
                        (*ruleset).rules.push(rule_ptr);
                        k = k.wrapping_add(1);
                    }
                }
                j_0 = j_0.wrapping_add(1);
            }
            if !cds.is_null() {
                if !(*cds).bc.is_null() {
                    otl_class_def_free((*cds).bc);
                }
                if !(*cds).ic.is_null() {
                    otl_class_def_free((*cds).ic);
                }
                if !(*cds).fc.is_null() {
                    otl_class_def_free((*cds).fc);
                }
                free(cds as *mut ::core::ffi::c_void);
                cds = ::core::ptr::null_mut::<ClassDefs>();
            }
            return subtable;
        }
    }
    // Same fallthrough leak `read_contextual_format2` had: `cds` (and its
    // populated `.bc`/`.ic`/`.fc`, from the first length check passing
    // above) was never freed on this malformed-input path.
    if !cds.is_null() {
        if !(*cds).bc.is_null() {
            otl_class_def_free((*cds).bc);
        }
        if !(*cds).ic.is_null() {
            otl_class_def_free((*cds).ic);
        }
        if !(*cds).fc.is_null() {
            otl_class_def_free((*cds).fc);
        }
        free(cds as *mut ::core::ffi::c_void);
        cds = ::core::ptr::null_mut::<ClassDefs>();
    }
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<ChainingSubtable>();
}
pub unsafe extern "C" fn otl_read_chaining(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
    mut options: *const Options,
) -> *mut Subtable {
    let mut format: u16 = 0 as u16;
    let mut subtable: *mut ChainingSubtable =
        (
            I_SUBTABLE_CHAINING
                .create
                .expect("non-null function pointer"))();
    // See the identical comment in `otl_read_contextual`.
    *subtable = ChainingSubtable::Poly(ChainingRuleSet::default());
    let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
    if !(table_length < offset.wrapping_add(2 as u32)) {
        format = read_16u(data.offset(offset as isize) as *const u8);
        if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            return subtable_from_raw(
                read_chaining_format1(subtable, data, table_length, offset, max_glyphs),
                Subtable::Chaining,
            );
        } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            return subtable_from_raw(
                read_chaining_format2(subtable, data, table_length, offset, max_glyphs),
                Subtable::Chaining,
            );
        } else if format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            let rule_ptr = general_read_chaining_rule(
                data,
                table_length,
                offset.wrapping_add(2 as u32),
                0 as u16,
                false,
                Some(
                    format3_coverage
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
                max_glyphs,
                NULL,
            );
            (*ruleset).rules.push(rule_ptr);
            return subtable_from_raw(subtable, Subtable::Chaining);
        }
    }
    (*(*options).logger)
        .log_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        LOG_VL_IMPORTANT,
        LoggerType::Warning,
        crate::bytesbuild!(b"Unsupported format ", format as ::core::ffi::c_int, b".\n"),
    );
    I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
#[inline]
// Was a manual meet-in-the-middle index-swapping loop over
// `*mut *mut Coverage` -- exactly `[T]::reverse` on the backtrack
// sub-slice, now that `match_0` is a real `Vec<Coverage>`. `input_begins
// == 0` (nothing to reverse) falls out of slicing an empty range.
unsafe extern "C" fn reverse_backtracks(mut rule: *mut ChainingRule) {
    let input_begins = (*rule).input_begins as usize;
    (&mut (*rule).match_0)[..input_begins].reverse();
}
