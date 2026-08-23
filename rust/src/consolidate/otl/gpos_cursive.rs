#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{GlyphHandle, Handle, HandleState};

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};

use crate::font::caryll_font::Font;
use crate::support::options::Options;
use crate::support::primitives::GlyphId;

use crate::table::otl::{Anchor, GposCursiveEntry, GposCursiveSubtable, OtlTable, Subtable};

use crate::support::glyph_order::{GlyphOrder, otfcc_gord_consolidate_handle};
use crate::table::otl::subtables::gpos_cursive::dispose_gpos_cursive_subtable;

pub unsafe fn consolidate_gpos_cursive(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: &Options,
) -> bool {
    let Subtable::GposCursive(mut_subtable) = &mut *_subtable else {
        unreachable!()
    };
    let subtable: *mut GposCursiveSubtable = mut_subtable;
    // Deduplicates by `target`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`,
    // not `IndexMap`: the original also did a HASH_SORT by that same id
    // right before reading entries back out, so the final order is
    // ascending by glyph id, not insertion order -- a `BTreeMap`'s
    // iteration order already is that, for free. Same shape as
    // `consolidate_gpos_single`'s uthash -> `BTreeMap` rewrite
    // (rust/README.md), with `(enter, exit): (Anchor, Anchor)` in place of
    // that one's single `PositionValue`.
    let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, Anchor, Anchor)> =
        std::collections::BTreeMap::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*subtable).len() {
        if !otfcc_gord_consolidate_handle(
            (*font)
                .glyph_order
                .as_deref_mut()
                .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder),
            &raw mut (&mut (*subtable))[k as usize].target,
        ) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Ignored missing glyph /",
                    &(&(*subtable))[k as usize].target.name,
                    b".\n",
                ),
            );
        } else {
            let fromid: i32 = (&(*subtable))[k as usize].target.index as i32;
            if seen.contains_key(&fromid) {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[Consolidate] Double-mapping a glyph in a cursive positioning /",
                        &(&(*subtable))[k as usize].target.name,
                        b".\n",
                    ),
                );
            } else {
                let fromname: Vec<u8> = (&(*subtable))[k as usize].target.name.clone();
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
            target: Handle {
                state: HandleState::Consolidated,
                index: fromid as GlyphId,
                name: fromname,
            } as GlyphHandle,
            enter,
            exit,
        });
    }
    return (*subtable).len() == 0 as usize;
}
