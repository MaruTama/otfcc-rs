use crate::logger::ByteStr;
use crate::support::handle::{GlyphHandle, Handle, HandleState};


use crate::support::glyph_order::GlyphOrder;
use crate::support::primitives::GlyphId;

use crate::table::otl::{Anchor, GposCursiveEntry, Subtable};

use crate::support::glyph_order::otfcc_gord_consolidate_handle;
use crate::table::otl::subtables::gpos_cursive::dispose_gpos_cursive_subtable;

pub fn consolidate_gpos_cursive(
    glyph_order: &GlyphOrder,
    _subtable: &mut Subtable,
) -> bool {
    let Subtable::GposCursive(subtable) = _subtable else {
        unreachable!()
    };
    // Deduplicates by `target`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`,
    // not `IndexMap`: the original also did a HASH_SORT by that same id
    // right before reading entries back out, so the final order is
    // ascending by glyph id, not insertion order -- a `BTreeMap`'s
    // iteration order already is that, for free. Same shape as
    // `consolidate_gpos_single`'s uthash -> `BTreeMap` rewrite
    // (RUST_MIGRATION.md), with `(enter, exit): (Anchor, Anchor)` in place of
    // that one's single `PositionValue`.
    let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, Anchor, Anchor)> =
        std::collections::BTreeMap::new();
    for entry in subtable.iter_mut() {
        // Guaranteed `Some`: `consolidate_otl` (and hence this function)
        // only ever runs when `glyf` is present, and `otfcc_consolidate_font`
        // always populates `glyph_order` before that, whenever `glyf` is
        // present.
        if !otfcc_gord_consolidate_handle(glyph_order, &mut entry.target) {
            tracing::warn!("[Consolidate] Ignored missing glyph /{}.\n", ByteStr(&entry.target.name));
        } else {
            let fromid: i32 = entry.target.index as i32;
            if let std::collections::btree_map::Entry::Vacant(e) = seen.entry(fromid) {
                let fromname: Vec<u8> = entry.target.name.clone();
                let enter: Anchor = entry.enter;
                let exit: Anchor = entry.exit;
                e.insert((fromname, enter, exit));
            } else {
                tracing::warn!("[Consolidate] Double-mapping a glyph in a cursive positioning /{}.\n", ByteStr(&entry.target.name));
            }
        }
    }
    dispose_gpos_cursive_subtable(subtable);
    for (fromid, (fromname, enter, exit)) in seen {
        subtable.push(GposCursiveEntry {
            target: Handle::new(HandleState::Consolidated, fromid as GlyphId, fromname) as GlyphHandle,
            enter,
            exit,
        });
    }
    subtable.is_empty()
}
