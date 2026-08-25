#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{GlyphHandle, Handle, HandleState};

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};

use crate::font::caryll_font::Font;
use crate::support::options::Options;
use crate::support::primitives::GlyphId;

use crate::table::otl::{GposSingleEntry, GposSingleSubtable, OtlTable, PositionValue, Subtable};

use crate::support::glyph_order::{GlyphOrder, otfcc_gord_consolidate_handle};
use crate::table::otl::subtables::gpos_single::dispose_gpos_single_subtable;

pub unsafe fn consolidate_gpos_single(
    font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::GposSingle(mut_subtable) = &mut *_subtable else {
        unreachable!()
    };
    let subtable: *mut GposSingleSubtable = mut_subtable;
    // Deduplicates by `target`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`,
    // not `IndexMap`: the original also did a HASH_SORT by that same id
    // right before reading entries back out, so the final order is
    // ascending by glyph id, not insertion order -- a `BTreeMap`'s
    // iteration order already is that, for free. Same shape as
    // `consolidate_gsub_multi`'s uthash -> `BTreeMap` rewrite
    // (rust/README.md), minus that one's coverage-consolidation step (a
    // `PositionValue` is a plain `Copy` struct, nothing to consolidate).
    let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, PositionValue)> =
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
                        b"[Consolidate] Detected glyph double-mapping about /",
                        &(&(*subtable))[k as usize].target.name,
                        b".\n",
                    ),
                );
            } else {
                let fromname: Vec<u8> = (&(*subtable))[k as usize].target.name.clone();
                let v: PositionValue = (&(*subtable))[k as usize].value;
                seen.insert(fromid, (fromname, v));
            }
        }
        k = k.wrapping_add(1);
    }
    dispose_gpos_single_subtable(subtable);
    for (fromid, (fromname, v)) in seen {
        (*subtable).push(GposSingleEntry {
            target: Handle {
                state: HandleState::Consolidated,
                index: fromid as GlyphId,
                name: fromname,
            } as GlyphHandle,
            value: v,
        });
    }
    return (*subtable).len() == 0 as usize;
}
