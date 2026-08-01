#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{strcmp};
use crate::table::otl::coverage::shrink_coverage;
use crate::support::handle::{HandleState, handle_consolidate_to, Handle, otfcc_handle_dispose, LookupHandle};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, TableId};

use crate::font::caryll_font::{Font};


























use crate::table::otl::{ChainingRule, Subtable, ChainingSubtable, OtlTable};
use crate::consolidate::otl::common::{fontop_consolidate_coverage};
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
            .log_sds
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
    while (j as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
        fontop_consolidate_coverage(font, *(*rule).match_0.offset(j as isize), options);
        shrink_coverage(
            *(*rule).match_0.offset(j as isize),
            true,
        );
        possible = possible as ::core::ffi::c_int != 0
            && (**(*rule).match_0.offset(j as isize)).len() as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int;
        j = j.wrapping_add(1);
    }
    if (*rule).input_begins as ::core::ffi::c_int > (*rule).match_count as ::core::ffi::c_int {
        (*rule).input_begins = (*rule).match_count;
    }
    if (*rule).input_ends as ::core::ffi::c_int > (*rule).match_count as ::core::ffi::c_int {
        (*rule).input_ends = (*rule).match_count;
    }
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < (*rule).apply_count as ::core::ffi::c_int {
        let mut found_lookup: bool = false;
        let mut h: *mut LookupHandle = &raw mut (*(*rule).apply.offset(j_0 as isize)).lookup;
        if !(*h).name.is_null() {
            let mut k: TableId = 0 as TableId;
            while (k as usize) < (*table).lookups.len() {
                if !(&(*table).lookups)[k as usize].is_null() {
                    if !((*(&(*table).lookups)[k as usize])
                        .subtables
                        .is_empty())
                    {
                        if !(strcmp(
                            (*(&(*table).lookups)[k as usize]).name
                                as *const ::core::ffi::c_char,
                            (*h).name as *const ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int)
                        {
                            found_lookup = true;
                            handle_consolidate_to(
                                h as *mut Handle,
                                k as GlyphId,
                                (*(&(*table).lookups)[k as usize]).name,
                            );
                        }
                    }
                }
                k = k.wrapping_add(1);
            }
            if !found_lookup && !(*(*rule).apply.offset(j_0 as isize)).lookup.name.is_null() {
                (*(*options).logger)
                    .log_sds
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
                otfcc_handle_dispose(
                    &raw mut (*(*rule).apply.offset(j_0 as isize)).lookup,
                );
            }
        } else if (*h).state == HandleState::Index
        {
            if (*h).index as usize >= (*table).lookups.len() {
                (*(*options).logger)
                    .log_sds
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
            handle_consolidate_to(
                h as *mut Handle,
                (*h).index,
                (*(&(*table).lookups)[(*h).index as usize]).name,
            );
        }
        j_0 = j_0.wrapping_add(1);
    }
    if (*rule).apply_count != 0 {
        let mut k_0: TableId = 0 as TableId;
        let mut j_1: TableId = 0 as TableId;
        while (j_1 as ::core::ffi::c_int) < (*rule).apply_count as ::core::ffi::c_int {
            if !(*(*rule).apply.offset(j_1 as isize)).lookup.name.is_null() {
                let fresh0 = k_0;
                k_0 = k_0.wrapping_add(1);
                *(*rule).apply.offset(fresh0 as isize) = (*(*rule).apply.offset(j_1 as isize)).clone();
            }
            j_1 = j_1.wrapping_add(1);
        }
        (*rule).apply_count = k_0;
        if (*rule).apply_count == 0 {
            return true;
        }
    }
    return !possible;
}
