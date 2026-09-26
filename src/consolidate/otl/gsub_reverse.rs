use crate::support::handle::{GlyphHandle, Handle, HandleState};

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};

use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, TableId};

use crate::table::otl::Subtable;

use crate::consolidate::otl::common::fontop_consolidate_coverage;

pub fn consolidate_gsub_reverse(
    glyph_order: &GlyphOrder,
    _subtable: &mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::GsubReverse(subtable) = _subtable else {
        unreachable!()
    };
    // Guaranteed `Some`: `consolidate_otl` (and hence every caller that
    // reaches here) only ever runs when `glyf` is present, and
    // `otfcc_consolidate_font` always populates `glyph_order` before
    // that, whenever `glyf` is present.
    let match_count = subtable.match_count as usize;
    for cov in subtable.match_0.iter_mut().take(match_count) {
        fontop_consolidate_coverage(glyph_order, cov, options);
    }
    fontop_consolidate_coverage(glyph_order, &mut subtable.to, options);
    if subtable.input_index as i32 >= subtable.match_count as i32 {
        subtable.input_index = (subtable.match_count as i32 - 1_i32) as TableId;
    }
    let input_index = subtable.input_index as usize;
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
    // `.zip()` stops at the shorter side on its own, the same bound `n =
    // min(...)` computed by hand.
    for (from, to) in subtable.match_0[input_index].iter().zip(subtable.to.iter()) {
        let fromid: i32 = from.index as i32;
        if let std::collections::btree_map::Entry::Vacant(e) = seen.entry(fromid) {
            let toid: i32 = to.index as i32;
            let fromname: Vec<u8> = from.name.clone();
            let toname: Vec<u8> = to.name.clone();
            e.insert((fromname, toid, toname));
        } else {
            logger_log_sds(
                &mut options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Double-mapping a glyph in a reverse substitution /",
                    &from.name,
                    b".\n",
                ),
            );
        }
    }
    let count: usize = seen.len();
    if count != subtable.match_0[input_index].len() || count != subtable.to.len() {
        logger_log_sds(
            &mut options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"[Consolidate] In this reverse subsitution lookup, some mappings are ignored.\n",
            ),
        );
    }
    subtable.match_0[input_index] = Vec::new();
    subtable.to = Vec::new();
    for (fromid, (fromname, toid, toname)) in seen {
        subtable.match_0[input_index].push(Handle {
            state: HandleState::Consolidated,
            index: fromid as GlyphId,
            name: fromname,
        } as GlyphHandle);
        subtable.to.push(Handle {
            state: HandleState::Consolidated,
            index: toid as GlyphId,
            name: toname,
        } as GlyphHandle);
    }
    return false;
}
