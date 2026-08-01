#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};

use crate::table::otl::classdef::{ClassDef, otl_class_def_free, read_class_def};
use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, otfcc_handle_dispose, otfcc_handle_dup, Handle, GlyphHandle, LookupHandle};

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
) -> *mut ChainingRule {
    let mut n_input: u16 = 0;
    let mut n_apply: u16 = 0;
    let mut jj: u16 = 0;
    let mut rule: *mut ChainingRule = ::core::ptr::null_mut::<ChainingRule>();
    rule = __caryll_allocate_clean(
        ::core::mem::size_of::<ChainingRule>() as usize,
        83 as ::core::ffi::c_ulong,
    ) as *mut ChainingRule;
    (*rule).match_0 = ::core::ptr::null_mut::<*mut Coverage>();
    (*rule).apply = ::core::ptr::null_mut::<ChainLookupApplication>();
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
            (*rule).match_0 = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut Coverage>() as usize)
                    .wrapping_mul((*rule).match_count as usize),
                98 as ::core::ffi::c_ulong,
            ) as *mut *mut Coverage;
            jj = 0 as u16;
            if minus_one {
                let fresh16 = jj;
                jj = jj.wrapping_add(1);
                let ref mut fresh17 = *(*rule).match_0.offset(fresh16 as isize);
                *fresh17 = fn_0.expect("non-null function pointer")(
                    data,
                    table_length,
                    start_gid,
                    offset,
                    2 as u16,
                    max_glyphs,
                    userdata,
                );
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
                let fresh18 = jj;
                jj = jj.wrapping_add(1);
                let ref mut fresh19 = *(*rule).match_0.offset(fresh18 as isize);
                *fresh19 = fn_0.expect("non-null function pointer")(
                    data,
                    table_length,
                    gid as u16,
                    offset,
                    2 as u16,
                    max_glyphs,
                    userdata,
                );
                j = j.wrapping_add(1);
            }
            (*rule).apply_count = n_apply as TableId;
            (*rule).apply = __caryll_allocate_clean(
                (::core::mem::size_of::<ChainLookupApplication>() as usize)
                    .wrapping_mul((*rule).apply_count as usize),
                108 as ::core::ffi::c_ulong,
            ) as *mut ChainLookupApplication;
            let mut j_0: TableId = 0 as TableId;
            while (j_0 as ::core::ffi::c_int) < n_apply as ::core::ffi::c_int {
                (*(*rule).apply.offset(j_0 as isize)).index = ((*rule).input_begins
                    as ::core::ffi::c_int
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
                (*(*rule).apply.offset(j_0 as isize)).lookup =
                    handle_from_index(read_16u(
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
                j_0 = j_0.wrapping_add(1);
            }
            reverse_backtracks(rule);
            return rule;
        }
    }
    delete_rule(rule);
    rule = ::core::ptr::null_mut::<ChainingRule>();
    return ::core::ptr::null_mut::<ChainingRule>();
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
    let mut jj: TableId = 0;
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
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rules_count = total_rules;
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut ChainingRule>() as usize)
                                .wrapping_mul(total_rules as usize),
                            144 as ::core::ffi::c_ulong,
                        )
                            as *mut *mut ChainingRule;
                        jj = 0 as TableId;
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
                                let ref mut fresh21 = *(*subtable)
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .rules
                                    .offset(jj as isize);
                                *fresh21 = general_read_contextual_rule(
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
                                jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as TableId;
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
    let mut jj: TableId = 0;
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
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules_count = total_rules;
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut ChainingRule>() as usize)
                    .wrapping_mul(total_rules as usize),
                186 as ::core::ffi::c_ulong,
            )
                as *mut *mut ChainingRule;
            jj = 0 as TableId;
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
                        let ref mut fresh20 = *(*subtable)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .rules
                            .offset(jj as isize);
                        *fresh20 = general_read_contextual_rule(
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
                        jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
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
    (*subtable).type_0 = ChainingType::Poly;
    if !(table_length < offset.wrapping_add(2 as u32)) {
        format = read_16u(data.offset(offset as isize) as *const u8);
        if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            return read_contextual_format1(subtable, data, table_length, offset, max_glyphs)
                as *mut Subtable;
        } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            return read_contextual_format2(subtable, data, table_length, offset, max_glyphs)
                as *mut Subtable;
        } else if format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules_count = 1 as TableId;
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
            *fresh15 = general_read_contextual_rule(
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
            return subtable as *mut Subtable;
        }
    }
    (*(*options).logger)
        .log_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        LOG_VL_IMPORTANT,
        LoggerType::Warning,
        crate::sdsbuild!(sdsempty(), b"Unsupported format ", format as ::core::ffi::c_int, b".\n"),
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
) -> *mut ChainingRule {
    let mut n_back: TableId = 0;
    let mut n_input: TableId = 0;
    let mut n_lookaround: TableId = 0;
    let mut n_apply: TableId = 0;
    let mut jj: TableId = 0;
    let mut rule: *mut ChainingRule = ::core::ptr::null_mut::<ChainingRule>();
    rule = __caryll_allocate_clean(
        ::core::mem::size_of::<ChainingRule>() as usize,
        247 as ::core::ffi::c_ulong,
    ) as *mut ChainingRule;
    (*rule).match_0 = ::core::ptr::null_mut::<*mut Coverage>();
    (*rule).apply = ::core::ptr::null_mut::<ChainLookupApplication>();
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
                        (*rule).match_0 = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut Coverage>() as usize)
                                .wrapping_mul((*rule).match_count as usize),
                            267 as ::core::ffi::c_ulong,
                        ) as *mut *mut Coverage;
                        jj = 0 as TableId;
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
                            let fresh1 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh2 = *(*rule).match_0.offset(fresh1 as isize);
                            *fresh2 = fn_0.expect("non-null function pointer")(
                                data,
                                table_length,
                                gid as u16,
                                offset,
                                1 as u16,
                                max_glyphs,
                                userdata,
                            );
                            j = j.wrapping_add(1);
                        }
                        if minus_one {
                            let fresh3 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh4 = *(*rule).match_0.offset(fresh3 as isize);
                            *fresh4 = fn_0.expect("non-null function pointer")(
                                data,
                                table_length,
                                start_gid,
                                offset,
                                2 as u16,
                                max_glyphs,
                                userdata,
                            );
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
                            let fresh5 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh6 = *(*rule).match_0.offset(fresh5 as isize);
                            *fresh6 = fn_0.expect("non-null function pointer")(
                                data,
                                table_length,
                                gid_0 as u16,
                                offset,
                                2 as u16,
                                max_glyphs,
                                userdata,
                            );
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
                            let fresh7 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh8 = *(*rule).match_0.offset(fresh7 as isize);
                            *fresh8 = fn_0.expect("non-null function pointer")(
                                data,
                                table_length,
                                gid_1 as u16,
                                offset,
                                3 as u16,
                                max_glyphs,
                                userdata,
                            );
                            j_1 = j_1.wrapping_add(1);
                        }
                        (*rule).apply_count = n_apply;
                        (*rule).apply = __caryll_allocate_clean(
                            (::core::mem::size_of::<ChainLookupApplication>() as usize)
                                .wrapping_mul((*rule).apply_count as usize),
                            285 as ::core::ffi::c_ulong,
                        )
                            as *mut ChainLookupApplication;
                        let mut j_2: TableId = 0 as TableId;
                        while (j_2 as ::core::ffi::c_int) < n_apply as ::core::ffi::c_int {
                            (*(*rule).apply.offset(j_2 as isize)).index = ((*rule).input_begins
                                as ::core::ffi::c_int
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
                            (*(*rule).apply.offset(j_2 as isize)).lookup =
                                handle_from_index(
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
                            j_2 = j_2.wrapping_add(1);
                        }
                        reverse_backtracks(rule);
                        return rule;
                    }
                }
            }
        }
    }
    delete_rule(rule);
    rule = ::core::ptr::null_mut::<ChainingRule>();
    return ::core::ptr::null_mut::<ChainingRule>();
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
    let mut jj: TableId = 0;
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
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rules_count = total_rules;
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut ChainingRule>() as usize)
                                .wrapping_mul(total_rules as usize),
                            321 as ::core::ffi::c_ulong,
                        )
                            as *mut *mut ChainingRule;
                        jj = 0 as TableId;
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
                                let ref mut fresh14 = *(*subtable)
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .rules
                                    .offset(jj as isize);
                                *fresh14 = general_read_chaining_rule(
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
                                jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as TableId;
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
    let mut jj: TableId = 0;
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
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules_count = total_rules;
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut ChainingRule>() as usize)
                    .wrapping_mul(total_rules as usize),
                363 as ::core::ffi::c_ulong,
            )
                as *mut *mut ChainingRule;
            jj = 0 as TableId;
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
                        let ref mut fresh11 = *(*subtable)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .rules
                            .offset(jj as isize);
                        *fresh11 = general_read_chaining_rule(
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
                        jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
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
    (*subtable).type_0 = ChainingType::Poly;
    if !(table_length < offset.wrapping_add(2 as u32)) {
        format = read_16u(data.offset(offset as isize) as *const u8);
        if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            return read_chaining_format1(subtable, data, table_length, offset, max_glyphs)
                as *mut Subtable;
        } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            return read_chaining_format2(subtable, data, table_length, offset, max_glyphs)
                as *mut Subtable;
        } else if format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules_count = 1 as TableId;
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
            *fresh0 = general_read_chaining_rule(
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
            return subtable as *mut Subtable;
        }
    }
    (*(*options).logger)
        .log_sds
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
unsafe extern "C" fn close_rule(mut rule: *mut ChainingRule) {
    if !rule.is_null()
        && !(*rule).match_0.is_null()
        && (*rule).match_count as ::core::ffi::c_int != 0
    {
        let mut k: TableId = 0 as TableId;
        while (k as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
            otl_coverage_free(
                *(*rule).match_0.offset(k as isize),
            );
            k = k.wrapping_add(1);
        }
        free((*rule).match_0 as *mut ::core::ffi::c_void);
        (*rule).match_0 = ::core::ptr::null_mut::<*mut Coverage>();
    }
    if !rule.is_null() && !(*rule).apply.is_null() {
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < (*rule).apply_count as ::core::ffi::c_int {
            otfcc_handle_dispose(
                &raw mut (*(*rule).apply.offset(j as isize)).lookup,
            );
            j = j.wrapping_add(1);
        }
        free((*rule).apply as *mut ::core::ffi::c_void);
        (*rule).apply = ::core::ptr::null_mut::<ChainLookupApplication>();
    }
}
#[inline]
unsafe extern "C" fn delete_rule(mut rule: *mut ChainingRule) {
    if rule.is_null() {
        return;
    }
    close_rule(rule);
    free(rule as *mut ::core::ffi::c_void);
    rule = ::core::ptr::null_mut::<ChainingRule>();
}
#[inline]
unsafe extern "C" fn reverse_backtracks(mut rule: *mut ChainingRule) {
    if (*rule).input_begins as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut start: TableId = 0 as TableId;
        let mut end: TableId =
            ((*rule).input_begins as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as TableId;
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
