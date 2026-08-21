#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};

use crate::table::otl::classdef::{ClassDef, otl_class_def_free, read_class_def};
use crate::table::otl::coverage::{Coverage, coverage_from_raw, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, otfcc_handle_dup, Handle, GlyphHandle, LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::font_reader::{FontReader};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, logger_log_sds};

use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};

use crate::support::{NULL};
use crate::table::otl::{ChainLookupApplication, ChainingRule, ChainingRuleSet, Subtable, ChainingSubtable, subtable_from_raw};
use crate::table::otl::subtables::chaining::common::{subtable_chaining_create, subtable_chaining_free, chaining_ruleset_mut};
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
// Every guard below is expressed as a `FontReader` read or `require_room`
// call in the exact sequence the original's hand-written `table_length <
// ...` checks ran in, so the set of inputs accepted/rejected is unchanged
// (`require_room`'s `checked_mul` cannot itself matter here: every count
// this file reads is a `u16`, so `count * stride` can never overflow
// `usize`). The one behavior change is fidelity, not scope: a `FontReader`
// read only ever demands exactly the bytes the value it is producing
// needs, where a few of the original's guards reserved a handful of extra
// bytes beyond what the following reads actually touched (see
// `read_contextual_format2`'s and `read_chaining_format2`'s "no slop"
// note below) -- always in the safe direction (rejecting strictly less
// than before), documented per-function where it applies.
pub unsafe fn general_read_contextual_rule(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    mut start_gid: u16,
    mut minus_one: bool,
    mut fn_0: CoverageReaderHandler,
    max_glyphs: GlyphId,
    mut userdata: *mut ::core::ffi::c_void,
) -> Option<Box<ChainingRule>> {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let minus_one_q: u16 = minus_one as u16;

    let mut header = FontReader::new(slice).at(offset as usize).ok()?;
    let n_input = header.u16().ok()?;
    let n_apply = header.u16().ok()?;
    // Matches the original's own guard exactly: it reserves `2*n_input`
    // bytes for the input-glyph array even on the `minus_one` path, where
    // only `n_input - minus_one_q` entries are actually read below (one
    // slot more than strictly needed -- preserved as-is, not tightened).
    let needed = (n_input as usize) * 2 + (n_apply as usize) * 4;
    header.require_room(needed, 1).ok()?;

    // `Box` is the allocation, the struct literal is the zero-init the old
    // `__caryll_allocate_clean` provided -- same shape as `new_lookup`/
    // `otfcc_new_glyf_glyph`.
    let mut rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: n_input as TableId,
        input_begins: 0 as TableId,
        input_ends: n_input as TableId,
        match_0: Vec::new(),
        apply: Vec::new(),
    });
    // Filled in order below (the `minus_one` slot first, then the rest
    // sequentially) -- every one of the `match_count` slots is written
    // exactly once, in increasing index order, so `.push()` is the direct
    // replacement for the old `jj`-indexed writes into
    // `__caryll_allocate_clean`'d memory (`jj` itself is gone: it was only
    // ever used as that index).
    rule.match_0 = Vec::with_capacity(rule.match_count as usize);
    if minus_one {
        rule.match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
            data,
            table_length,
            start_gid,
            offset,
            2 as u16,
            max_glyphs,
            userdata,
        )));
    }
    // `n_input - minus_one_q` in the original ran in signed `c_int`
    // arithmetic, so a malformed `n_input < minus_one_q` (possible: the
    // `minus_one` slot above is unconditional, independent of `n_input`'s
    // own value) gave a negative loop bound and simply ran zero
    // iterations. `saturating_sub` reproduces that same "zero iterations"
    // outcome without the panic a plain `u16` subtraction would give here.
    let n_input_read = n_input.saturating_sub(minus_one_q);
    for j in 0..n_input_read {
        let gid = FontReader::new(slice)
            .at(offset as usize + 4 + 2 * j as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
            data,
            table_length,
            gid,
            offset,
            2 as u16,
            max_glyphs,
            userdata,
        )));
    }

    rule.apply = Vec::with_capacity(n_apply as usize);
    let lookup_base = offset as usize + 4 + 2 * n_input_read as usize;
    for j0 in 0..n_apply {
        let mut lr = FontReader::new(slice)
            .at(lookup_base + 4 * j0 as usize)
            .unwrap();
        let seq_index = lr.u16().unwrap();
        let lookup_index = lr.u16().unwrap();
        let index = rule.input_begins.wrapping_add(seq_index);
        let lookup = handle_from_index(lookup_index) as LookupHandle;
        rule.apply.push(ChainLookupApplication { index, lookup });
    }
    reverse_backtracks(&mut *rule as *mut ChainingRule);
    Some(rule)
}
unsafe fn read_contextual_format1(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let mut first_coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();

    let result: Option<()> = 'parse: {
        let Ok(mut header) = FontReader::new(slice).at(offset as usize + 2) else {
            break 'parse None;
        };
        let Ok(cov_rel) = header.u16() else { break 'parse None };
        let Ok(chain_sub_rule_set_count) = header.u16() else {
            break 'parse None;
        };
        let cov_offset = offset.wrapping_add(cov_rel as u32);
        // `read_coverage` always returns a valid (possibly empty) `Coverage`
        // shell, never null, even on malformed input -- see coverage.rs.
        first_coverage = read_coverage(data as *const u8, table_length, cov_offset);
        if chain_sub_rule_set_count as usize != (*first_coverage).len() {
            break 'parse None;
        }
        if header.require_room(chain_sub_rule_set_count as usize, 2).is_err() {
            break 'parse None;
        }

        // First pass: validate every ruleset's own header + rule-offset array.
        let mut total_rules: usize = 0;
        for j in 0..chain_sub_rule_set_count {
            let srs_rel = FontReader::new(slice)
                .at(offset as usize + 6 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            let srs_offset = offset.wrapping_add(srs_rel as u32);
            let Ok(mut srs_header) = FontReader::new(slice).at(srs_offset as usize) else {
                break 'parse None;
            };
            let Ok(srs_count) = srs_header.u16() else {
                break 'parse None;
            };
            if srs_header.require_room(srs_count as usize, 2).is_err() {
                break 'parse None;
            }
            total_rules = total_rules.saturating_add(srs_count as usize);
        }

        // Second pass: build, re-deriving each offset exactly as the first
        // pass did (nothing here is retained across passes, matching the
        // original's own two-pass structure).
        let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
        (*ruleset).rules = Vec::with_capacity(total_rules);
        for j in 0..chain_sub_rule_set_count {
            let srs_rel = FontReader::new(slice)
                .at(offset as usize + 6 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            let srs_offset = offset.wrapping_add(srs_rel as u32);
            let srs_count = FontReader::new(slice)
                .at(srs_offset as usize)
                .unwrap()
                .u16()
                .unwrap();
            for k in 0..srs_count {
                let sr_rel = FontReader::new(slice)
                    .at(srs_offset as usize + 2 + 2 * k as usize)
                    .unwrap()
                    .u16()
                    .unwrap();
                let sr_offset = srs_offset.wrapping_add(sr_rel as u32);
                let rule_ptr = general_read_contextual_rule(
                    data,
                    table_length,
                    sr_offset,
                    (&(*first_coverage))[j as usize].index as u16,
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
            }
        }
        break 'parse Some(());
    };

    // `first_coverage` was leaked on every failure path here (only the
    // success path below ever freed it) -- now freed exactly once,
    // unconditionally, regardless of which branch above bailed out.
    otl_coverage_free(first_coverage);
    if result.is_some() {
        return subtable;
    }
    subtable_chaining_free(subtable);
    ::core::ptr::null_mut::<ChainingSubtable>()
}
unsafe fn read_contextual_format2(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let mut cds: *mut ClassDefs = ::core::ptr::null_mut::<ClassDefs>();

    let result: Option<()> = 'parse: {
        let Ok(mut header) = FontReader::new(slice).at(offset as usize + 4) else {
            break 'parse None;
        };
        let Ok(ic_rel) = header.u16() else { break 'parse None };
        let Ok(chain_sub_class_set_cnt) = header.u16() else {
            break 'parse None;
        };
        // The original reserved 4 extra bytes here (`offset+12+2*count`)
        // beyond what the `classSetOffset` array at `offset+8` actually
        // needs (`offset+8+2*count`) -- always-safe over-conservative slop,
        // now exactly the array's real requirement.
        if header.require_room(chain_sub_class_set_cnt as usize, 2).is_err() {
            break 'parse None;
        }

        cds = __caryll_allocate_clean(
            ::core::mem::size_of::<ClassDefs>() as usize,
            172 as ::core::ffi::c_ulong,
        ) as *mut ClassDefs;
        (*cds).bc = ::core::ptr::null_mut::<ClassDef>();
        (*cds).ic = read_class_def(
            data as *const u8,
            table_length,
            offset.wrapping_add(ic_rel as u32),
        );
        (*cds).fc = ::core::ptr::null_mut::<ClassDef>();

        // First pass: validate every non-empty ClassSet's own header +
        // rule-offset array. The original had NO guard at all here -- every
        // read below (`srs_count` itself, and its rule-offset array) ran
        // straight off `offset + src_offset` with no bounds check, a real
        // out-of-bounds read on a malformed `ChainSubClassSet` offset. Now
        // guarded like every sibling array in this file.
        let mut total_rules: usize = 0;
        for j in 0..chain_sub_class_set_cnt {
            let src_rel = FontReader::new(slice)
                .at(offset as usize + 8 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            if src_rel == 0 {
                continue;
            }
            let Ok(mut cs_header) = FontReader::new(slice).at(offset as usize + src_rel as usize)
            else {
                break 'parse None;
            };
            let Ok(srs_count) = cs_header.u16() else {
                break 'parse None;
            };
            if cs_header.require_room(srs_count as usize, 2).is_err() {
                break 'parse None;
            }
            total_rules = total_rules.saturating_add(srs_count as usize);
        }

        let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
        (*ruleset).rules = Vec::with_capacity(total_rules);
        for j in 0..chain_sub_class_set_cnt {
            let src_rel = FontReader::new(slice)
                .at(offset as usize + 8 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            if src_rel == 0 {
                continue;
            }
            let srs_count = FontReader::new(slice)
                .at(offset as usize + src_rel as usize)
                .unwrap()
                .u16()
                .unwrap();
            for k in 0..srs_count {
                let sr_rel = FontReader::new(slice)
                    .at(offset as usize + src_rel as usize + 2 + 2 * k as usize)
                    .unwrap()
                    .u16()
                    .unwrap();
                let sr_offset = offset
                    .wrapping_add(src_rel as u32)
                    .wrapping_add(sr_rel as u32);
                let rule_ptr = general_read_contextual_rule(
                    data,
                    table_length,
                    sr_offset,
                    j as u16,
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
            }
        }
        break 'parse Some(());
    };

    // `cds` cleanup, run exactly once regardless of outcome -- this is the
    // fallthrough-leak fix that was already present (as a second, duplicated
    // copy of this same block) before this conversion; consolidated into
    // one unconditional cleanup rather than newly discovered here.
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
    }
    if result.is_some() {
        return subtable;
    }
    subtable_chaining_free(subtable);
    ::core::ptr::null_mut::<ChainingSubtable>()
}
pub unsafe fn otl_read_contextual(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
    mut options: &Options,
) -> *mut Subtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let mut subtable: *mut ChainingSubtable = (subtable_chaining_create)();
    // `subtable` is fresh from `create()` (a valid, empty `Canonical`
    // value) -- replace it wholesale with a valid, empty `Poly` ruleset.
    // Every downstream construction path (format1/format2/format3, and the
    // error paths that dispose the subtable without ever reaching one) now
    // sees a valid, possibly-still-empty ruleset from this point on.
    *subtable = ChainingSubtable::Poly(ChainingRuleSet::default());
    let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
    let mut format: u16 = 0 as u16;
    if let Ok(mut r) = FontReader::new(slice).at(offset as usize) {
        if let Ok(f) = r.u16() {
            format = f;
        }
    }
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
    logger_log_sds(
        options.logger,
        LOG_VL_IMPORTANT,
        LoggerType::Warning,
        crate::bytesbuild!(b"Unsupported format ", format as ::core::ffi::c_int, b".\n"),
    );
    subtable_chaining_free(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe fn general_read_chaining_rule(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    mut start_gid: u16,
    mut minus_one: bool,
    mut fn_0: CoverageReaderHandler,
    max_glyphs: GlyphId,
    mut userdata: *mut ::core::ffi::c_void,
) -> Option<Box<ChainingRule>> {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let minus_one_q: u16 = minus_one as u16;

    // Four counts read back-to-back, each immediately followed by a skip
    // over the array it introduces -- `n_back`/`backtrackArray`,
    // `n_input`/`inputArray` (minus the `minus_one` slot), `n_lookaround`/
    // `lookaheadArray`, then `n_apply` itself. This sequence of
    // read-then-`require_room`-then-skip steps enforces exactly the same
    // cumulative byte requirement the original's four incremental
    // `table_length < ...` guards did (each of those checked the running
    // total so far plus room for the next 2-byte count field; here each
    // step's own `u16()`/`require_room` call demands precisely that).
    let mut header = FontReader::new(slice).at(offset as usize).ok()?;
    let n_back = header.u16().ok()?;
    header.require_room(n_back as usize, 2).ok()?;
    header.skip(n_back as usize * 2).ok()?;
    let n_input = header.u16().ok()?;
    let n_input_read = n_input.saturating_sub(minus_one_q);
    header.require_room(n_input_read as usize, 2).ok()?;
    header.skip(n_input_read as usize * 2).ok()?;
    let n_lookaround = header.u16().ok()?;
    header.require_room(n_lookaround as usize, 2).ok()?;
    header.skip(n_lookaround as usize * 2).ok()?;
    let n_apply = header.u16().ok()?;
    header.require_room(n_apply as usize, 4).ok()?;

    // `Box` is the allocation, the struct literal is the zero-init the old
    // `__caryll_allocate_clean` provided -- see `general_read_contextual_rule`.
    // `match_count`/`input_ends` used `c_int` addition then truncated to
    // `TableId` (u16) in the original; `wrapping_add` reproduces that
    // truncation instead of panicking on an overflowing plain `+` (each of
    // `n_back`/`n_input`/`n_lookaround` is individually u16-bounded, but
    // their sum is not).
    let match_count = n_back.wrapping_add(n_input).wrapping_add(n_lookaround);
    let input_ends = n_back.wrapping_add(n_input);
    let mut rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: match_count as TableId,
        input_begins: n_back,
        input_ends: input_ends as TableId,
        match_0: Vec::new(),
        apply: Vec::new(),
    });
    // Filled in order below (backtrack, then the `minus_one` slot, then
    // input, then lookaround) -- every one of the `match_count` slots is
    // written exactly once, in increasing index order, so `.push()` is the
    // direct replacement for the old `jj`-indexed writes (`jj` itself is
    // gone: it was only ever used as that index).
    rule.match_0 = Vec::with_capacity(match_count as usize);
    for j in 0..n_back {
        let gid = FontReader::new(slice)
            .at(offset as usize + 2 + 2 * j as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
            data,
            table_length,
            gid,
            offset,
            1 as u16,
            max_glyphs,
            userdata,
        )));
    }
    if minus_one {
        rule.match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
            data,
            table_length,
            start_gid,
            offset,
            2 as u16,
            max_glyphs,
            userdata,
        )));
    }
    // Array positions derived the same way `header`'s cursor validated
    // them above (cumulative `usize` addition on the *reduced* counts),
    // rather than by re-subtracting `minus_one_q` from `input_ends`/
    // `match_count` the way the original's pointer arithmetic did --
    // avoids a `u16` underflow when a malformed `n_input < minus_one_q`
    // makes those two disagree (see `n_input_read` above), and always
    // agrees with them when they don't.
    let input_base = offset as usize + 4 + 2 * n_back as usize;
    for j0 in 0..n_input_read {
        let gid = FontReader::new(slice)
            .at(input_base + 2 * j0 as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
            data,
            table_length,
            gid,
            offset,
            2 as u16,
            max_glyphs,
            userdata,
        )));
    }
    let lookaround_base = input_base + 2 * n_input_read as usize + 2;
    for j1 in 0..n_lookaround {
        let gid = FontReader::new(slice)
            .at(lookaround_base + 2 * j1 as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.match_0.push(coverage_from_raw(fn_0.expect("non-null function pointer")(
            data,
            table_length,
            gid,
            offset,
            3 as u16,
            max_glyphs,
            userdata,
        )));
    }

    rule.apply = Vec::with_capacity(n_apply as usize);
    let apply_base = lookaround_base + 2 * n_lookaround as usize + 2;
    for j2 in 0..n_apply {
        let mut lr = FontReader::new(slice)
            .at(apply_base + 4 * j2 as usize)
            .unwrap();
        let seq_index = lr.u16().unwrap();
        let lookup_index = lr.u16().unwrap();
        let index = rule.input_begins.wrapping_add(seq_index);
        let lookup = handle_from_index(lookup_index) as LookupHandle;
        rule.apply.push(ChainLookupApplication { index, lookup });
    }
    reverse_backtracks(&mut *rule as *mut ChainingRule);
    Some(rule)
}
unsafe fn read_chaining_format1(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let mut first_coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();

    let result: Option<()> = 'parse: {
        let Ok(mut header) = FontReader::new(slice).at(offset as usize + 2) else {
            break 'parse None;
        };
        let Ok(cov_rel) = header.u16() else { break 'parse None };
        let Ok(chain_sub_rule_set_count) = header.u16() else {
            break 'parse None;
        };
        let cov_offset = offset.wrapping_add(cov_rel as u32);
        // `read_coverage` always returns a valid (possibly empty) `Coverage`
        // shell, never null, even on malformed input -- see coverage.rs.
        first_coverage = read_coverage(data as *const u8, table_length, cov_offset);
        if chain_sub_rule_set_count as usize != (*first_coverage).len() {
            break 'parse None;
        }
        if header.require_room(chain_sub_rule_set_count as usize, 2).is_err() {
            break 'parse None;
        }

        let mut total_rules: usize = 0;
        for j in 0..chain_sub_rule_set_count {
            let srs_rel = FontReader::new(slice)
                .at(offset as usize + 6 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            let srs_offset = offset.wrapping_add(srs_rel as u32);
            let Ok(mut srs_header) = FontReader::new(slice).at(srs_offset as usize) else {
                break 'parse None;
            };
            let Ok(srs_count) = srs_header.u16() else {
                break 'parse None;
            };
            if srs_header.require_room(srs_count as usize, 2).is_err() {
                break 'parse None;
            }
            total_rules = total_rules.saturating_add(srs_count as usize);
        }

        let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
        (*ruleset).rules = Vec::with_capacity(total_rules);
        for j in 0..chain_sub_rule_set_count {
            let srs_rel = FontReader::new(slice)
                .at(offset as usize + 6 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            let srs_offset = offset.wrapping_add(srs_rel as u32);
            let srs_count = FontReader::new(slice)
                .at(srs_offset as usize)
                .unwrap()
                .u16()
                .unwrap();
            for k in 0..srs_count {
                let sr_rel = FontReader::new(slice)
                    .at(srs_offset as usize + 2 + 2 * k as usize)
                    .unwrap()
                    .u16()
                    .unwrap();
                let sr_offset = srs_offset.wrapping_add(sr_rel as u32);
                let rule_ptr = general_read_chaining_rule(
                    data,
                    table_length,
                    sr_offset,
                    (&(*first_coverage))[j as usize].index as u16,
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
            }
        }
        break 'parse Some(());
    };

    // Same fallthrough leak `read_contextual_format1` had: `first_coverage`
    // was only freed on the success path. Freed exactly once here instead.
    otl_coverage_free(first_coverage);
    if result.is_some() {
        return subtable;
    }
    subtable_chaining_free(subtable);
    ::core::ptr::null_mut::<ChainingSubtable>()
}
unsafe fn read_chaining_format2(
    mut subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let mut cds: *mut ClassDefs = ::core::ptr::null_mut::<ClassDefs>();

    let result: Option<()> = 'parse: {
        let Ok(mut header) = FontReader::new(slice).at(offset as usize + 4) else {
            break 'parse None;
        };
        let Ok(bc_rel) = header.u16() else { break 'parse None };
        let Ok(ic_rel) = header.u16() else { break 'parse None };
        let Ok(fc_rel) = header.u16() else { break 'parse None };
        let Ok(chain_sub_class_set_cnt) = header.u16() else {
            break 'parse None;
        };
        if header.require_room(chain_sub_class_set_cnt as usize, 2).is_err() {
            break 'parse None;
        }

        cds = __caryll_allocate_clean(
            ::core::mem::size_of::<ClassDefs>() as usize,
            349 as ::core::ffi::c_ulong,
        ) as *mut ClassDefs;
        (*cds).bc = read_class_def(
            data as *const u8,
            table_length,
            offset.wrapping_add(bc_rel as u32),
        );
        (*cds).ic = read_class_def(
            data as *const u8,
            table_length,
            offset.wrapping_add(ic_rel as u32),
        );
        (*cds).fc = read_class_def(
            data as *const u8,
            table_length,
            offset.wrapping_add(fc_rel as u32),
        );

        // First pass: validate every non-empty ClassSet's own header +
        // rule-offset array. The original had NO guard at all here (same
        // missing-guard shape as `read_contextual_format2`'s ClassSet
        // loop) -- every read below ran straight off `offset + src_offset`
        // with no bounds check. Now guarded like every sibling array.
        let mut total_rules: usize = 0;
        for j in 0..chain_sub_class_set_cnt {
            let src_rel = FontReader::new(slice)
                .at(offset as usize + 12 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            if src_rel == 0 {
                continue;
            }
            let Ok(mut cs_header) = FontReader::new(slice).at(offset as usize + src_rel as usize)
            else {
                break 'parse None;
            };
            let Ok(srs_count) = cs_header.u16() else {
                break 'parse None;
            };
            if cs_header.require_room(srs_count as usize, 2).is_err() {
                break 'parse None;
            }
            total_rules = total_rules.saturating_add(srs_count as usize);
        }

        let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
        (*ruleset).rules = Vec::with_capacity(total_rules);
        for j in 0..chain_sub_class_set_cnt {
            let src_rel = FontReader::new(slice)
                .at(offset as usize + 12 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            if src_rel == 0 {
                continue;
            }
            let srs_count = FontReader::new(slice)
                .at(offset as usize + src_rel as usize)
                .unwrap()
                .u16()
                .unwrap();
            for k in 0..srs_count {
                let dsr_rel = FontReader::new(slice)
                    .at(offset as usize + src_rel as usize + 2 + 2 * k as usize)
                    .unwrap()
                    .u16()
                    .unwrap();
                let sr_offset = offset
                    .wrapping_add(src_rel as u32)
                    .wrapping_add(dsr_rel as u32);
                let rule_ptr = general_read_chaining_rule(
                    data,
                    table_length,
                    sr_offset,
                    j as u16,
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
            }
        }
        break 'parse Some(());
    };

    // `cds` cleanup, run exactly once regardless of outcome -- same
    // consolidation as `read_contextual_format2` (the fallthrough-leak fix
    // was already present as a duplicated block before this conversion).
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
    }
    if result.is_some() {
        return subtable;
    }
    subtable_chaining_free(subtable);
    ::core::ptr::null_mut::<ChainingSubtable>()
}
pub unsafe fn otl_read_chaining(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    max_glyphs: GlyphId,
    mut options: &Options,
) -> *mut Subtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let mut subtable: *mut ChainingSubtable = (subtable_chaining_create)();
    // See the identical comment in `otl_read_contextual`.
    *subtable = ChainingSubtable::Poly(ChainingRuleSet::default());
    let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
    let mut format: u16 = 0 as u16;
    if let Ok(mut r) = FontReader::new(slice).at(offset as usize) {
        if let Ok(f) = r.u16() {
            format = f;
        }
    }
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
    logger_log_sds(
        options.logger,
        LOG_VL_IMPORTANT,
        LoggerType::Warning,
        crate::bytesbuild!(b"Unsupported format ", format as ::core::ffi::c_int, b".\n"),
    );
    subtable_chaining_free(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
#[inline]
// Was a manual meet-in-the-middle index-swapping loop over
// `*mut *mut Coverage` -- exactly `[T]::reverse` on the backtrack
// sub-slice, now that `match_0` is a real `Vec<Coverage>`. `input_begins
// == 0` (nothing to reverse) falls out of slicing an empty range.
unsafe fn reverse_backtracks(mut rule: *mut ChainingRule) {
    let input_begins = (*rule).input_begins as usize;
    (&mut (*rule).match_0)[..input_begins].reverse();
}

#[cfg(test)]
mod chaining_read_tests {
    use super::*;

    fn zeroed_options() -> Options {
        unsafe { ::core::mem::zeroed() }
    }

    unsafe fn glyphs_of(cov: &Coverage) -> Vec<GlyphId> {
        cov.iter().map(|h| h.index).collect()
    }

    #[test]
    fn context_format3_reads_a_single_glyph_based_rule() {
        // format=3, nInput=1, nApply=0, inputArray[0] = coverage shift (10,
        // relative to the format3 rule's own subtable start), coverage
        // table (format1, one glyph) at byte 10.
        let mut data = [0u8; 16];
        data[0..2].copy_from_slice(&3u16.to_be_bytes());
        data[2..4].copy_from_slice(&1u16.to_be_bytes()); // nInput
        data[4..6].copy_from_slice(&0u16.to_be_bytes()); // nApply
        data[6..8].copy_from_slice(&10u16.to_be_bytes()); // shift -> byte 10
        data[10..12].copy_from_slice(&1u16.to_be_bytes()); // coverage format 1
        data[12..14].copy_from_slice(&1u16.to_be_bytes()); // glyphCount
        data[14..16].copy_from_slice(&42u16.to_be_bytes()); // glyph
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::Chaining(sub) = &*boxed else { unreachable!() };
            let ChainingSubtable::Poly(ruleset) = sub else { unreachable!() };
            assert_eq!(ruleset.rules.len(), 1);
            let rule = ruleset.rules[0].as_ref().unwrap();
            assert_eq!(rule.match_0.len(), 1);
            assert_eq!(glyphs_of(&rule.match_0[0]), vec![42]);
            assert!(rule.apply.is_empty());
        }
    }

    #[test]
    fn context_format1_reads_one_rule_set_with_one_rule() {
        // format=1, coverageOffset -> 16 (one glyph, id 5),
        // chainSubRuleSetCount=1, srsOffset[0] -> 8.
        // ChainSubRuleSet at 8: count=1, ruleOffset[0] -> 4 (abs 12).
        // ChainSubRule at 12 (minus_one=true): nInput=1 (the coverage's own
        // glyph fills the one slot), nApply=0.
        let mut data = [0u8; 22];
        data[0..2].copy_from_slice(&1u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&16u16.to_be_bytes()); // coverageOffset
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // chainSubRuleSetCount
        data[6..8].copy_from_slice(&8u16.to_be_bytes()); // srsOffset[0]
        data[8..10].copy_from_slice(&1u16.to_be_bytes()); // srs_count
        data[10..12].copy_from_slice(&4u16.to_be_bytes()); // ruleOffset[0] (rel. to 8 -> 12)
        data[12..14].copy_from_slice(&1u16.to_be_bytes()); // nInput
        data[14..16].copy_from_slice(&0u16.to_be_bytes()); // nApply
        data[16..18].copy_from_slice(&1u16.to_be_bytes()); // coverage format 1
        data[18..20].copy_from_slice(&1u16.to_be_bytes()); // glyphCount
        data[20..22].copy_from_slice(&5u16.to_be_bytes()); // glyph
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::Chaining(sub) = &*boxed else { unreachable!() };
            let ChainingSubtable::Poly(ruleset) = sub else { unreachable!() };
            assert_eq!(ruleset.rules.len(), 1);
            let rule = ruleset.rules[0].as_ref().unwrap();
            assert_eq!(rule.match_count, 1);
            assert_eq!(glyphs_of(&rule.match_0[0]), vec![5]);
        }
    }

    #[test]
    fn context_format1_rule_set_count_mismatched_with_coverage_is_rejected() {
        // chainSubRuleSetCount (2) doesn't match the coverage's glyph
        // count (1) -- the original's own consistency check, preserved.
        let mut data = [0u8; 12];
        data[0..2].copy_from_slice(&1u16.to_be_bytes());
        data[2..4].copy_from_slice(&6u16.to_be_bytes()); // coverageOffset -> 6
        data[4..6].copy_from_slice(&2u16.to_be_bytes()); // count = 2
        data[6..8].copy_from_slice(&1u16.to_be_bytes()); // coverage format 1
        data[8..10].copy_from_slice(&1u16.to_be_bytes()); // glyphCount = 1
        data[10..12].copy_from_slice(&9u16.to_be_bytes());
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(raw.is_null());
        }
    }

    #[test]
    fn context_format2_class_set_offset_past_the_table_end_is_rejected_instead_of_reading_oob() {
        // The original read `srs_count` (and its rule-offset array)
        // straight off `offset + classSetOffset[j]` with no guard at all --
        // a `classSetOffset` pointing past `table_length` read out of
        // bounds. `classSetOffset[0]` here (5000) is far past this
        // 10-byte table.
        let mut data = [0u8; 10];
        data[0..2].copy_from_slice(&2u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&0u16.to_be_bytes()); // unused field
        data[4..6].copy_from_slice(&10u16.to_be_bytes()); // classDefOffset (past end, handled gracefully)
        data[6..8].copy_from_slice(&1u16.to_be_bytes()); // chainSubClassSetCnt
        data[8..10].copy_from_slice(&5000u16.to_be_bytes()); // classSetOffset[0]
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(raw.is_null());
        }
    }

    #[test]
    fn context_format2_zero_class_set_offset_is_skipped() {
        // classSetOffset == 0 is a documented "no ruleset for this class"
        // sentinel, not a real offset -- must not be dereferenced.
        let mut data = [0u8; 10];
        data[0..2].copy_from_slice(&2u16.to_be_bytes());
        data[2..4].copy_from_slice(&0u16.to_be_bytes());
        data[4..6].copy_from_slice(&10u16.to_be_bytes());
        data[6..8].copy_from_slice(&1u16.to_be_bytes());
        data[8..10].copy_from_slice(&0u16.to_be_bytes()); // classSetOffset[0] = 0
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::Chaining(sub) = &*boxed else { unreachable!() };
            let ChainingSubtable::Poly(ruleset) = sub else { unreachable!() };
            assert!(ruleset.rules.is_empty());
        }
    }

    #[test]
    fn chaining_format3_reads_backtrack_input_and_lookahead() {
        // format=3, nBack=1, backtrack shift -> byte 20 (glyph 1),
        // nInput=1, input shift -> byte 26 (glyph 2), nLookaround=1,
        // lookaround shift -> byte 32 (glyph 3), nApply=0.
        let mut data = [0u8; 38];
        data[0..2].copy_from_slice(&3u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&1u16.to_be_bytes()); // nBack
        data[4..6].copy_from_slice(&20u16.to_be_bytes()); // backtrack shift
        data[6..8].copy_from_slice(&1u16.to_be_bytes()); // nInput
        data[8..10].copy_from_slice(&26u16.to_be_bytes()); // input shift
        data[10..12].copy_from_slice(&1u16.to_be_bytes()); // nLookaround
        data[12..14].copy_from_slice(&32u16.to_be_bytes()); // lookaround shift
        data[14..16].copy_from_slice(&0u16.to_be_bytes()); // nApply
        // Coverage tables (format 1, one glyph each) for the three shifts.
        // Each `format3_coverage` call resolves to `offset + shift - 2`
        // where `offset` is `general_read_chaining_rule`'s own offset
        // (the dispatch's `offset + 2`, i.e. `2` here).
        data[20..22].copy_from_slice(&1u16.to_be_bytes());
        data[22..24].copy_from_slice(&1u16.to_be_bytes());
        data[24..26].copy_from_slice(&1u16.to_be_bytes()); // backtrack glyph
        data[26..28].copy_from_slice(&1u16.to_be_bytes());
        data[28..30].copy_from_slice(&1u16.to_be_bytes());
        data[30..32].copy_from_slice(&2u16.to_be_bytes()); // input glyph
        data[32..34].copy_from_slice(&1u16.to_be_bytes());
        data[34..36].copy_from_slice(&1u16.to_be_bytes());
        data[36..38].copy_from_slice(&3u16.to_be_bytes()); // lookaround glyph
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_chaining(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::Chaining(sub) = &*boxed else { unreachable!() };
            let ChainingSubtable::Poly(ruleset) = sub else { unreachable!() };
            assert_eq!(ruleset.rules.len(), 1);
            let rule = ruleset.rules[0].as_ref().unwrap();
            // backtrack is stored reversed; here there's only one entry so
            // the order is unaffected.
            assert_eq!(
                rule.match_0.iter().map(|c| glyphs_of(c)).collect::<Vec<_>>(),
                vec![vec![1], vec![2], vec![3]]
            );
            assert_eq!(rule.input_begins, 1);
            assert_eq!(rule.input_ends, 2);
        }
    }

    #[test]
    fn chaining_rule_with_input_count_below_the_minus_one_slot_does_not_panic() {
        // A malformed `nInput` of 0 while this call site always wants the
        // `minus_one` (coverage-implied) slot filled -- the original
        // computed `nInput - minus_one_q` in signed `c_int` arithmetic and
        // simply ran zero array-read iterations; a naive `u16` port of
        // that subtraction would panic on overflow instead. Reached via
        // `general_read_chaining_rule` directly since `read_chaining_format1`
        // (the real caller of this path) also requires a fully valid,
        // consistent outer coverage/ruleset structure this test isn't
        // trying to build.
        let mut data = [0u8; 10];
        data[0..2].copy_from_slice(&0u16.to_be_bytes()); // nBack
        data[2..4].copy_from_slice(&0u16.to_be_bytes()); // nInput (malformed: 0)
        data[4..6].copy_from_slice(&0u16.to_be_bytes()); // nLookaround
        data[6..8].copy_from_slice(&0u16.to_be_bytes()); // nApply
        unsafe {
            let rule = general_read_chaining_rule(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                7,
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
                        ) -> *mut Coverage,
                ),
                100,
                NULL,
            );
            let rule = rule.unwrap();
            // Only the `minus_one` slot (glyph 7, from `start_gid`) is
            // filled; the (empty) input array contributes nothing.
            assert_eq!(rule.match_0.len(), 1);
            assert_eq!(glyphs_of(&rule.match_0[0]), vec![7]);
        }
    }

    #[test]
    fn unsupported_format_logs_and_returns_null() {
        let data = [0u8, 9]; // format = 9
        let mut options = zeroed_options();
        unsafe {
            options.logger =
                crate::logger::otfcc_new_logger(crate::logger::otfcc_new_empty_target());
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(raw.is_null());
            crate::logger::logger_dispose(options.logger);
        }
    }
}
