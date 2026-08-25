#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{GlyphHandle, Handle, HandleState};
use crate::table::otl::coverage::Coverage;

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};

use crate::font::caryll_font::Font;
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, TableId};

use crate::table::otl::{GsubReverseSubtable, OtlTable, Subtable};

use crate::consolidate::otl::common::fontop_consolidate_coverage;

pub unsafe fn consolidate_gsub_reverse(
    font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::GsubReverse(mut_subtable) = &mut *_subtable else {
        unreachable!()
    };
    let subtable: *mut GsubReverseSubtable = mut_subtable;
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*subtable).match_count as ::core::ffi::c_int {
        fontop_consolidate_coverage(
            font,
            &mut (&mut (*subtable).match_0)[j as usize] as *mut Coverage,
            options,
        );
        j = j.wrapping_add(1);
    }
    fontop_consolidate_coverage(font, &mut (*subtable).to as *mut Coverage, options);
    if (*subtable).input_index as ::core::ffi::c_int
        >= (*subtable).match_count as ::core::ffi::c_int
    {
        (*subtable).input_index =
            ((*subtable).match_count as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as TableId;
    }
    let from: *mut Coverage =
        &mut (&mut (*subtable).match_0)[(*subtable).input_index as usize] as *mut Coverage;
    let to: *mut Coverage = &mut (*subtable).to as *mut Coverage;
    // Deduplicates by `from`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`,
    // not `IndexMap`: the original also did a HASH_SORT by that same id
    // right before reading entries back out. Same overall shape as
    // `consolidate_gsub_single`'s uthash -> `BTreeMap` rewrite (both share
    // the same C-side dedup-hash node), but names are `sdsdup`'d into the
    // map up front here rather than aliasing `from`/`to`'s existing
    // `SdsRaw` pointers the way the original uthash node did. The original
    // then truncated `from`/`to` to the survivor count *before* reading
    // those aliases back out -- harmless in C (truncating a length field
    // frees nothing), but `from`/`to` are real `Vec<GlyphHandle>` now and
    // `Handle` owns its name (`Drop` frees it): truncating first can drop
    // (and free) a survivor whose original index landed past the new
    // length, leaving a still-pending alias dangling before it's read.
    // Confirmed empirically (not just by inspection) with a synthetic
    // duplicate placed away from the end of `from`, which reproduces the
    // exact use-after-free ordering; a build with intervening allocations
    // happened not to visibly corrupt the output, but the read is still of
    // freed memory. Owned copies collected up front, with `from`/`to`
    // rebuilt from scratch afterward, sidestep the ordering hazard
    // entirely instead of preserving it.
    let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, i32, Vec<u8>)> =
        std::collections::BTreeMap::new();
    let n: usize = (*from).len().min((*to).len());
    let mut k: usize = 0;
    while k < n {
        let fromid: i32 = (&(*from))[k].index as i32;
        if seen.contains_key(&fromid) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Double-mapping a glyph in a reverse substitution /",
                    &(&(*from))[k].name,
                    b".\n",
                ),
            );
        } else {
            let toid: i32 = (&(*to))[k].index as i32;
            let fromname: Vec<u8> = (&(*from))[k].name.clone();
            let toname: Vec<u8> = (&(*to))[k].name.clone();
            seen.insert(fromid, (fromname, toid, toname));
        }
        k = k.wrapping_add(1);
    }
    let count: usize = seen.len();
    if count != (*from).len() || count != (*to).len() {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"[Consolidate] In this reverse subsitution lookup, some mappings are ignored.\n",
            ),
        );
    }
    *from = Vec::new();
    *to = Vec::new();
    for (fromid, (fromname, toid, toname)) in seen {
        (*from).push(Handle {
            state: HandleState::Consolidated,
            index: fromid as GlyphId,
            name: fromname,
        } as GlyphHandle);
        (*to).push(Handle {
            state: HandleState::Consolidated,
            index: toid as GlyphId,
            name: toname,
        } as GlyphHandle);
    }
    return false;
}
