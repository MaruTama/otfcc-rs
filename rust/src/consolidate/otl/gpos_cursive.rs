#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{handle_from_consolidated, GlyphHandle};

use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::font::caryll_font::{Font};
























use crate::table::otl::{Anchor, GposCursiveEntry, Subtable, GposCursiveSubtable, OtlTable};





use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::table::otl::subtables::gpos_cursive::{dispose_gpos_cursive_subtable};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree};




pub unsafe extern "C" fn consolidate_gpos_cursive(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GposCursiveSubtable = &raw mut (*_subtable).gpos_cursive as *mut GposCursiveSubtable;
    // Deduplicates by `target`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`,
    // not `IndexMap`: the original also did a HASH_SORT by that same id
    // right before reading entries back out, so the final order is
    // ascending by glyph id, not insertion order -- a `BTreeMap`'s
    // iteration order already is that, for free. Same shape as
    // `consolidate_gpos_single`'s uthash -> `BTreeMap` rewrite
    // (rust/README.md), with `(enter, exit): (Anchor, Anchor)` in place of
    // that one's single `PositionValue`.
    let mut seen: std::collections::BTreeMap<i32, (SdsRaw, Anchor, Anchor)> =
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
                        b"[Consolidate] Double-mapping a glyph in a cursive positioning /",
                        (&(*subtable))[k as usize].target.name,
                        b".\n",
                    ),
                );
            } else {
                let fromname: SdsRaw = sdsdup((&(*subtable))[k as usize].target.name);
                let enter: Anchor = (&(*subtable))[k as usize].enter;
                let exit: Anchor = (&(*subtable))[k as usize].exit;
                seen.insert(fromid, (fromname, enter, exit));
            }
        }
        k = k.wrapping_add(1);
    }
    dispose_gpos_cursive_subtable(subtable);
    for (fromid, (fromname, enter, exit)) in seen {
        (*subtable).push(GposCursiveEntry {
            target: handle_from_consolidated(fromid as GlyphId, fromname) as GlyphHandle,
            enter,
            exit,
        });
        sdsfree(fromname);
    }
    return (*subtable).len() == 0 as usize;
}
