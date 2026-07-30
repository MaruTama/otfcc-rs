#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{strcmp};
use crate::table::otl::coverage::shrinkCoverage;
use crate::support::handle::{HandleState, handle_consolidateTo, Handle, otfcc_Handle_dispose, LookupHandle};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, TableId};

use crate::font::caryll_font::{Font};


























use crate::table::otl::{ChainingRule, Subtable, ChainingSubtable, OtlTable};
use crate::consolidate::otl::common::{fontop_consolidateCoverage};
use crate::vendor::sds::{sdsempty};










pub unsafe extern "C" fn consolidate_chaining(
    mut font: *mut Font,
    mut table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut ChainingSubtable = &raw mut (*_subtable).chaining;
    if (*subtable).type_0 as u64 != 0 {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::sdsbuild!(sdsempty(), b"[Consolidate] Ignoring non-canonical chaining subtable."),
        );
        return false;
    }
    let mut rule: *mut ChainingRule = &raw mut (*subtable).c2rust_unnamed.rule;
    let mut possible: bool = true;
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
        fontop_consolidateCoverage(font, *(*rule).match_0.offset(j as isize), options);
        shrinkCoverage(
            *(*rule).match_0.offset(j as isize),
            true,
        );
        possible = possible as ::core::ffi::c_int != 0
            && (**(*rule).match_0.offset(j as isize)).numGlyphs as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int;
        j = j.wrapping_add(1);
    }
    if (*rule).inputBegins as ::core::ffi::c_int > (*rule).matchCount as ::core::ffi::c_int {
        (*rule).inputBegins = (*rule).matchCount;
    }
    if (*rule).inputEnds as ::core::ffi::c_int > (*rule).matchCount as ::core::ffi::c_int {
        (*rule).inputEnds = (*rule).matchCount;
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < (*rule).applyCount as ::core::ffi::c_int {
        let mut found_lookup: bool = false;
        let mut h: *mut LookupHandle = &raw mut (*(*rule).apply.offset(j_0 as isize)).lookup;
        if !(*h).name.is_null() {
            let mut k: TableId = 0 as TableId;
            while (k as usize) < (*table).lookups.length {
                if !(*(*table).lookups.items.offset(k as isize)).is_null() {
                    if !((**(*table).lookups.items.offset(k as isize))
                        .subtables
                        .length
                        == 0)
                    {
                        if !(strcmp(
                            (**(*table).lookups.items.offset(k as isize)).name
                                as *const ::core::ffi::c_char,
                            (*h).name as *const ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int)
                        {
                            found_lookup = true;
                            handle_consolidateTo(
                                h as *mut Handle,
                                k as GlyphId,
                                (**(*table).lookups.items.offset(k as isize)).name,
                            );
                        }
                    }
                }
                k = k.wrapping_add(1);
            }
            if !found_lookup && !(*(*rule).apply.offset(j_0 as isize)).lookup.name.is_null() {
                (*(*options).logger)
                    .logSDS
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Quoting an invalid lookup ",
                        (*(*rule).apply.offset(j_0 as isize)).lookup.name,
                        b". This lookup application is ignored.",
                    ),
                );
                otfcc_Handle_dispose(
                    &raw mut (*(*rule).apply.offset(j_0 as isize)).lookup,
                );
            }
        } else if (*h).state == HandleState::Index
        {
            if (*h).index as usize >= (*table).lookups.length {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Quoting an invalid lookup #",
                        (*h).index as ::core::ffi::c_int,
                        b".",
                    ),
                );
                (*h).index = 0 as GlyphId;
            }
            handle_consolidateTo(
                h as *mut Handle,
                (*h).index,
                (**(*table).lookups.items.offset((*h).index as isize)).name,
            );
        }
        j_0 = j_0.wrapping_add(1);
    }
    if (*rule).applyCount != 0 {
        let mut k_0: TableId = 0 as TableId;
        let mut j_1: TableId = 0 as TableId;
        while (j_1 as ::core::ffi::c_int) < (*rule).applyCount as ::core::ffi::c_int {
            if !(*(*rule).apply.offset(j_1 as isize)).lookup.name.is_null() {
                let fresh0 = k_0;
                k_0 = k_0.wrapping_add(1);
                *(*rule).apply.offset(fresh0 as isize) = *(*rule).apply.offset(j_1 as isize);
            }
            j_1 = j_1.wrapping_add(1);
        }
        (*rule).applyCount = k_0;
        if (*rule).applyCount == 0 {
            return true;
        }
    }
    return !possible;
}
