use crate::logger::ByteStr;
use crate::support::handle::{GlyphHandle, Handle, HandleState};


use crate::support::glyph_order::GlyphOrder;
use crate::support::primitives::GlyphId;

use crate::table::otl::{GposSingleEntry, PositionValue, Subtable};

use crate::support::glyph_order::gord_consolidate_handle;
use crate::table::otl::subtables::gpos_single::dispose_gpos_single_subtable;

pub fn consolidate_gpos_single(
    glyph_order: &GlyphOrder,
    _subtable: &mut Subtable,
) -> bool {
    let Subtable::GposSingle(subtable) = _subtable else {
        unreachable!()
    };
    // Deduplicates by `target`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`,
    // not `IndexMap`: the original also did a HASH_SORT by that same id
    // right before reading entries back out, so the final order is
    // ascending by glyph id, not insertion order -- a `BTreeMap`'s
    // iteration order already is that, for free. Same shape as
    // `consolidate_gsub_multi`'s uthash -> `BTreeMap` rewrite
    // (RUST_MIGRATION.md), minus that one's coverage-consolidation step (a
    // `PositionValue` is a plain `Copy` struct, nothing to consolidate).
    let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, PositionValue)> =
        std::collections::BTreeMap::new();
    for entry in subtable.iter_mut() {
        // Guaranteed `Some`: `consolidate_otl` (and hence this function)
        // only ever runs when `glyf` is present, and `consolidate_font`
        // always populates `glyph_order` before that, whenever `glyf` is
        // present.
        if !gord_consolidate_handle(glyph_order, &mut entry.target) {
            tracing::warn!("[Consolidate] Ignored missing glyph /{}.\n", ByteStr(&entry.target.name));
        } else {
            let fromid: i32 = entry.target.index as i32;
            if let std::collections::btree_map::Entry::Vacant(e) = seen.entry(fromid) {
                let fromname: Vec<u8> = entry.target.name.clone();
                let v: PositionValue = entry.value;
                e.insert((fromname, v));
            } else {
                tracing::warn!("[Consolidate] Detected glyph double-mapping about /{}.\n", ByteStr(&entry.target.name));
            }
        }
    }
    dispose_gpos_single_subtable(subtable);
    for (fromid, (fromname, v)) in seen {
        subtable.push(GposSingleEntry {
            target: Handle::new(HandleState::Consolidated, fromid as GlyphId, fromname) as GlyphHandle,
            value: v,
        });
    }
    subtable.is_empty()
}
