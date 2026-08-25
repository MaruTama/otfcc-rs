#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{GlyphHandle, Handle, HandleState};

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};

use crate::font::caryll_font::Font;
use crate::support::options::Options;
use crate::support::primitives::GlyphId;

use crate::table::otl::{GsubSingleEntry, GsubSingleSubtable, OtlTable, Subtable};

use crate::support::glyph_order::{GlyphOrder, otfcc_gord_consolidate_handle};
use crate::table::otl::subtables::gsub_single::dispose_gsub_single_subtable;

pub unsafe fn consolidate_gsub_single(
    font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    options: &Options,
) -> bool {
    let glyph_order: *mut GlyphOrder = (*font)
        .glyph_order
        .as_deref_mut()
        .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder);
    let Subtable::GsubSingle(mut_subtable) = &mut *_subtable else {
        unreachable!()
    };
    let subtable: *mut GsubSingleSubtable = mut_subtable;
    // Deduplicates by `from`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`,
    // not `IndexMap`: the original also did a HASH_SORT by that same id
    // right before reading entries back out. Same shape as
    // `consolidate_gpos_single`'s uthash -> `BTreeMap` rewrite
    // (rust/README.md), with `to`'s `(id, name)` in place of a single
    // `PositionValue`.
    let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, i32, Vec<u8>)> =
        std::collections::BTreeMap::new();
    let mut k: usize = 0 as usize;
    while k < (*subtable).len() {
        if !otfcc_gord_consolidate_handle(glyph_order, &raw mut (&mut (*subtable))[k as usize].from)
        {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Ignored missing glyph /",
                    &(&(*subtable))[k as usize].from.name,
                    b".\n",
                ),
            );
        } else if !otfcc_gord_consolidate_handle(
            glyph_order,
            &raw mut (&mut (*subtable))[k as usize].to,
        ) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Ignored missing glyph /",
                    &(&(*subtable))[k as usize].to.name,
                    b".\n",
                ),
            );
        } else {
            let fromid: i32 = (&(*subtable))[k as usize].from.index as i32;
            if seen.contains_key(&fromid) {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[Consolidate] Double-mapping a glyph in a single substitution /",
                        &(&(*subtable))[k as usize].from.name,
                        b".\n",
                    ),
                );
            } else {
                let toid: i32 = (&(*subtable))[k as usize].to.index as i32;
                let fromname: Vec<u8> = (&(*subtable))[k as usize].from.name.clone();
                let toname: Vec<u8> = (&(*subtable))[k as usize].to.name.clone();
                seen.insert(fromid, (fromname, toid, toname));
            }
        }
        k = k.wrapping_add(1);
    }
    if seen.len() != (*subtable).len() {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"[Consolidate] In this lookup, some mappings are ignored.\n",),
        );
    }
    dispose_gsub_single_subtable(subtable);
    for (fromid, (fromname, toid, toname)) in seen {
        (*subtable).push(GsubSingleEntry {
            from: Handle {
                state: HandleState::Consolidated,
                index: fromid as GlyphId,
                name: fromname,
            } as GlyphHandle,
            to: Handle {
                state: HandleState::Consolidated,
                index: toid as GlyphId,
                name: toname,
            } as GlyphHandle,
        });
    }
    return (*subtable).len() == 0 as usize;
}
