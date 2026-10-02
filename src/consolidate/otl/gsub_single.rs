use crate::logger::ByteStr;
use crate::support::handle::{GlyphHandle, Handle, HandleState};


use crate::support::primitives::GlyphId;

use crate::table::otl::{GsubSingleEntry, Subtable};

use crate::support::glyph_order::{GlyphOrder, gord_consolidate_handle};
use crate::table::otl::subtables::gsub_single::dispose_gsub_single_subtable;

pub fn consolidate_gsub_single(
    glyph_order: &GlyphOrder,
    _subtable: &mut Subtable,
) -> bool {
    // Guaranteed `Some`: `consolidate_otl` (and hence this function) only
    // ever runs when `glyf` is present, and `consolidate_font`
    // always populates `glyph_order` before that, whenever `glyf` is
    // present.
    let Subtable::GsubSingle(subtable) = _subtable else {
        unreachable!()
    };
    // Deduplicates by `from`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`,
    // not `IndexMap`: the original also did a HASH_SORT by that same id
    // right before reading entries back out. Same shape as
    // `consolidate_gpos_single`'s uthash -> `BTreeMap` rewrite
    // (RUST_MIGRATION.md), with `to`'s `(id, name)` in place of a single
    // `PositionValue`.
    let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, i32, Vec<u8>)> =
        std::collections::BTreeMap::new();
    for entry in subtable.iter_mut() {
        if !gord_consolidate_handle(glyph_order, &mut entry.from) {
            tracing::warn!("[Consolidate] Ignored missing glyph /{}.\n", ByteStr(&entry.from.name));
        } else if !gord_consolidate_handle(glyph_order, &mut entry.to) {
            tracing::warn!("[Consolidate] Ignored missing glyph /{}.\n", ByteStr(&entry.to.name));
        } else {
            let fromid: i32 = entry.from.index as i32;
            if let std::collections::btree_map::Entry::Vacant(e) = seen.entry(fromid) {
                let toid: i32 = entry.to.index as i32;
                let fromname: Vec<u8> = entry.from.name.clone();
                let toname: Vec<u8> = entry.to.name.clone();
                e.insert((fromname, toid, toname));
            } else {
                tracing::warn!("[Consolidate] Double-mapping a glyph in a single substitution /{}.\n", ByteStr(&entry.from.name));
            }
        }
    }
    if seen.len() != subtable.len() {
        tracing::warn!("[Consolidate] In this lookup, some mappings are ignored.\n");
    }
    dispose_gsub_single_subtable(subtable);
    for (fromid, (fromname, toid, toname)) in seen {
        subtable.push(GsubSingleEntry {
            from: Handle::new(HandleState::Consolidated, fromid as GlyphId, fromname) as GlyphHandle,
            to: Handle::new(HandleState::Consolidated, toid as GlyphId, toname) as GlyphHandle,
        });
    }
    subtable.is_empty()
}
