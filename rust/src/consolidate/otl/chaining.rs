#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::table::otl::coverage::{Coverage, shrink_coverage};
use crate::support::handle::{HandleState, handle_name_eq_bytes, Handle, otfcc_handle_dispose, LookupHandle};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, TableId};

use crate::font::caryll_font::{Font};




use crate::table::otl::{ChainingRule, Subtable, ChainingSubtable, OtlTable};
use crate::consolidate::otl::common::{fontop_consolidate_coverage};

pub unsafe extern "C" fn consolidate_chaining(
    mut font: *mut Font,
    mut table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let Subtable::Chaining(mut_subtable) = &mut *_subtable else { unreachable!() };
    let subtable: *mut ChainingSubtable = mut_subtable;
    if (*subtable).type_0 as u64 != 0 {
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"[Consolidate] Ignoring non-canonical chaining subtable."),
        );
        return false;
    }
    let mut rule: *mut ChainingRule = &raw mut (*subtable).c2rust_unnamed.rule as *mut ChainingRule;
    let mut possible: bool = true;
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
        fontop_consolidate_coverage(
            font,
            &mut (&mut (*rule).match_0)[j as usize] as *mut Coverage,
            options,
        );
        shrink_coverage(
            &mut (&mut (*rule).match_0)[j as usize] as *mut Coverage,
            true,
        );
        possible = possible as ::core::ffi::c_int != 0
            && (&(*rule).match_0)[j as usize].len() as ::core::ffi::c_int
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
    while (j_0 as usize) < (*rule).apply.len() {
        let mut found_lookup: bool = false;
        let mut h: *mut LookupHandle = &raw mut (&mut (*rule).apply)[j_0 as usize].lookup;
        if !(*h).name.is_empty() {
            let mut k: TableId = 0 as TableId;
            while (k as usize) < (*table).lookups.len() {
                // Every element is a `Box<Lookup>` now, never null, so the
                // old null check is gone -- everything else here is plain
                // field access through the `Box`, unchanged.
                if !((*(&(*table).lookups)[k as usize])
                    .subtables
                    .is_empty())
                {
                    if handle_name_eq_bytes(
                        &(*h).name,
                        &(*(&(*table).lookups)[k as usize]).name,
                    ) {
                        found_lookup = true;
                        *h = Handle {
                            state: HandleState::Consolidated,
                            index: k as GlyphId,
                            name: (*(&(*table).lookups)[k as usize]).name.clone(),
                        } as LookupHandle;
                    }
                }
                k = k.wrapping_add(1);
            }
            if !found_lookup && !(&(*rule).apply)[j_0 as usize].lookup.name.is_empty() {
                (*(*options).logger)
                    .log_sds
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(b"[Consolidate] Quoting an invalid lookup ",
                        &(&(*rule).apply)[j_0 as usize].lookup.name,
                        b". This lookup application is ignored.",
                    ),
                );
                otfcc_handle_dispose(
                    &raw mut (&mut (*rule).apply)[j_0 as usize].lookup,
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
                    crate::bytesbuild!(b"[Consolidate] Quoting an invalid lookup #",
                        (*h).index as ::core::ffi::c_int,
                        b".",
                    ),
                );
                (*h).index = 0 as GlyphId;
            }
            let idx = (*h).index;
            *h = Handle {
                state: HandleState::Consolidated,
                index: idx,
                name: (*(&(*table).lookups)[idx as usize]).name.clone(),
            } as LookupHandle;
        }
        j_0 = j_0.wrapping_add(1);
    }
    if !(*rule).apply.is_empty() {
        // Was a manual compact-in-place loop over `apply_count` before
        // `.apply` became a `Vec` -- `retain` is the direct translation.
        (*rule).apply.retain(|app| !app.lookup.name.is_empty());
        if (*rule).apply.is_empty() {
            return true;
        }
    }
    return !possible;
}
