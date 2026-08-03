#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{handle_from_consolidated, GlyphHandle};

use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::font::caryll_font::{Font};

use crate::table::otl::{GsubSingleEntry, Subtable, GsubSingleSubtable, OtlTable};

use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::table::otl::subtables::gsub_single::{dispose_gsub_single_subtable};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree, SdsRaw};

pub unsafe extern "C" fn consolidate_gsub_single(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GsubSingleSubtable = &raw mut (*_subtable).gsub_single as *mut GsubSingleSubtable;
    // Deduplicates by `from`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`,
    // not `IndexMap`: the original also did a HASH_SORT by that same id
    // right before reading entries back out. Same shape as
    // `consolidate_gpos_single`'s uthash -> `BTreeMap` rewrite
    // (rust/README.md), with `to`'s `(id, name)` in place of a single
    // `PositionValue`.
    let mut seen: std::collections::BTreeMap<i32, (SdsRaw, i32, SdsRaw)> =
        std::collections::BTreeMap::new();
    let mut k: usize = 0 as usize;
    while k < (*subtable).len() {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (&mut (*subtable))[k as usize].from,
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
                    (&(*subtable))[k as usize].from.name,
                    b".\n",
                ),
            );
        } else if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (&mut (*subtable))[k as usize].to,
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
                    (&(*subtable))[k as usize].to.name,
                    b".\n",
                ),
            );
        } else {
            let fromid: i32 = (&(*subtable))[k as usize].from.index as i32;
            if seen.contains_key(&fromid) {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Double-mapping a glyph in a single substitution /",
                        (&(*subtable))[k as usize].from.name,
                        b".\n",
                    ),
                );
            } else {
                let toid: i32 = (&(*subtable))[k as usize].to.index as i32;
                let fromname: SdsRaw = sdsdup((&(*subtable))[k as usize].from.name);
                let toname: SdsRaw = sdsdup((&(*subtable))[k as usize].to.name);
                seen.insert(fromid, (fromname, toid, toname));
            }
        }
        k = k.wrapping_add(1);
    }
    if seen.len() != (*subtable).len() {
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::sdsbuild!(
                sdsempty(),
                b"[Consolidate] In this lookup, some mappings are ignored.\n",
            ),
        );
    }
    dispose_gsub_single_subtable(subtable);
    for (fromid, (fromname, toid, toname)) in seen {
        (*subtable).push(GsubSingleEntry {
            from: handle_from_consolidated(fromid as GlyphId, fromname) as GlyphHandle,
            to: handle_from_consolidated(toid as GlyphId, toname) as GlyphHandle,
        });
        sdsfree(fromname);
        sdsfree(toname);
    }
    return (*subtable).len() == 0 as usize;
}
