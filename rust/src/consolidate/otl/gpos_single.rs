#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{handle_from_consolidated, GlyphHandle};

use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::font::caryll_font::{Font};
























use crate::table::otl::{GposSingleEntry, PositionValue, Subtable, GposSingleSubtable, OtlTable};





use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::table::otl::subtables::gpos_single::{dispose_gpos_single_subtable};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree};




pub unsafe extern "C" fn consolidate_gpos_single(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GposSingleSubtable = &raw mut (*_subtable).gpos_single as *mut GposSingleSubtable;
    // Deduplicates by `target`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`,
    // not `IndexMap`: the original also did a HASH_SORT by that same id
    // right before reading entries back out, so the final order is
    // ascending by glyph id, not insertion order -- a `BTreeMap`'s
    // iteration order already is that, for free. Same shape as
    // `consolidate_gsub_multi`'s uthash -> `BTreeMap` rewrite
    // (rust/README.md), minus that one's coverage-consolidation step (a
    // `PositionValue` is a plain `Copy` struct, nothing to consolidate).
    let mut seen: std::collections::BTreeMap<i32, (SdsRaw, PositionValue)> =
        std::collections::BTreeMap::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*subtable).len() {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (&mut (*subtable))[k as usize].target,
        ) {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored missing glyph /",
                    (&(*subtable))[k as usize].target.name,
                    b".\n",
                ),
            );
        } else {
            let fromid: i32 = (&(*subtable))[k as usize].target.index as i32;
            if seen.contains_key(&fromid) {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Detected glyph double-mapping about /",
                        (&(*subtable))[k as usize].target.name,
                        b".\n",
                    ),
                );
            } else {
                let fromname: SdsRaw = sdsdup((&(*subtable))[k as usize].target.name);
                let v: PositionValue = (&(*subtable))[k as usize].value;
                seen.insert(fromid, (fromname, v));
            }
        }
        k = k.wrapping_add(1);
    }
    dispose_gpos_single_subtable(subtable);
    for (fromid, (fromname, v)) in seen {
        (*subtable).push(GposSingleEntry {
            target: handle_from_consolidated(fromid as GlyphId, fromname) as GlyphHandle,
            value: v,
        });
        sdsfree(fromname);
    }
    return (*subtable).len() == 0 as usize;
}
