use libc::{strcmp};
extern "C" {
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    fn fontop_consolidateCoverage(
        font: *mut otfcc_Font,
        coverage: *mut otl_Coverage,
        options: *const otfcc_Options,
    );
}
use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, shrinkCoverage};
use crate::support::handle::{HANDLE_STATE_INDEX, handle_consolidateTo, otfcc_Handle, otfcc_Handle_dispose, otfcc_LookupHandle};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};

use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphid_t, tableid_t};
use crate::vendor::sds::{sds};

use crate::font::caryll_font::{otfcc_Font};


























use crate::table::otl::{otl_ChainingRule, otl_Subtable, subtable_chaining, table_OTL};










pub type lookup_handle = otfcc_LookupHandle;
#[no_mangle]
pub unsafe extern "C" fn consolidate_chaining(
    mut font: *mut otfcc_Font,
    mut table: *mut table_OTL,
    mut _subtable: *mut otl_Subtable,
    mut options: *const otfcc_Options,
) -> bool {
    let mut subtable: *mut subtable_chaining = &raw mut (*_subtable).chaining;
    if (*subtable).type_0 as u64 != 0 {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            crate::sdsbuild!(sdsempty(), b"[Consolidate] Ignoring non-canonical chaining subtable."),
        );
        return false;
    }
    let mut rule: *mut otl_ChainingRule = &raw mut (*subtable).c2rust_unnamed.rule;
    let mut possible: bool = true;
    let mut j: tableid_t = 0 as tableid_t;
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
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int) < (*rule).applyCount as ::core::ffi::c_int {
        let mut foundLookup: bool = false;
        let mut h: *mut lookup_handle = &raw mut (*(*rule).apply.offset(j_0 as isize)).lookup;
        if !(*h).name.is_null() {
            let mut k: tableid_t = 0 as tableid_t;
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
                            foundLookup = true;
                            handle_consolidateTo(
                                h as *mut otfcc_Handle,
                                k as glyphid_t,
                                (**(*table).lookups.items.offset(k as isize)).name,
                            );
                        }
                    }
                }
                k = k.wrapping_add(1);
            }
            if !foundLookup && !(*(*rule).apply.offset(j_0 as isize)).lookup.name.is_null() {
                (*(*options).logger)
                    .logSDS
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
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
        } else if (*h).state == HANDLE_STATE_INDEX
        {
            if (*h).index as usize >= (*table).lookups.length {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Quoting an invalid lookup #",
                        (*h).index as ::core::ffi::c_int,
                        b".",
                    ),
                );
                (*h).index = 0 as glyphid_t;
            }
            handle_consolidateTo(
                h as *mut otfcc_Handle,
                (*h).index,
                (**(*table).lookups.items.offset((*h).index as isize)).name,
            );
        }
        j_0 = j_0.wrapping_add(1);
    }
    if (*rule).applyCount != 0 {
        let mut k_0: tableid_t = 0 as tableid_t;
        let mut j_1: tableid_t = 0 as tableid_t;
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
