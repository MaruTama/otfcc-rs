use crate::logger::ByteStr;
use crate::support::handle::{GlyphHandle, Handle, HandleState};
use crate::table::otl::coverage::{Coverage, shrink_coverage};


use crate::support::primitives::GlyphId;

use crate::support::glyph_order::GlyphOrder;

use crate::table::otl::{GsubMultiEntry, Subtable};

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::support::glyph_order::gord_consolidate_handle;
use crate::table::otl::subtables::gsub_multi::dispose_gsub_multi_subtable;

pub fn consolidate_gsub_multi(glyph_order: &GlyphOrder, _subtable: &mut Subtable) -> bool {
    let Subtable::GsubMulti(subtable) = _subtable else {
        unreachable!()
    };
    // Deduplicates by `from.index`, first occurrence wins -- a later
    // duplicate's already-consolidated `to` coverage is simply dropped along
    // with the rest of the pre-dedup `subtable` when it's disposed below.
    // `BTreeMap` so the result is ascending by glyph id, not insertion
    // order.
    let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, Coverage)> =
        std::collections::BTreeMap::new();
    for entry in subtable.iter_mut() {
        // Guaranteed `Some`: `consolidate_otl` (and hence this function)
        // only ever runs when `glyf` is present, and `consolidate_font`
        // always populates `glyph_order` before that, whenever `glyf` is
        // present.
        if !gord_consolidate_handle(glyph_order, &mut entry.from) {
            tracing::warn!("[Consolidate] Ignored missing glyph /{}.\n", ByteStr(&entry.from.name));
        } else {
            fontop_consolidate_coverage(glyph_order, &mut entry.to);
            shrink_coverage(&mut entry.to, false);
            if entry.to.is_empty() {
                tracing::warn!("[Consolidate] Ignoring empty one-to-many / alternative substitution for glyph /{}.\n", ByteStr(&entry.from.name));
            } else {
                let fromid: i32 = entry.from.index as i32;
                if let std::collections::btree_map::Entry::Vacant(e) = seen.entry(fromid) {
                    let fromname: Vec<u8> = entry.from.name.clone();
                    let to: Coverage = ::core::mem::take(&mut entry.to);
                    e.insert((fromname, to));
                }
            }
        }
    }
    dispose_gsub_multi_subtable(subtable);
    for (fromid, (fromname, to)) in seen {
        subtable.push(GsubMultiEntry {
            from: Handle::new(HandleState::Consolidated, fromid as GlyphId, fromname) as GlyphHandle,
            to,
        });
    }
    subtable.is_empty()
}
