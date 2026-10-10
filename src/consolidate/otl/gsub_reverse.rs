use crate::logger::ByteStr;
use crate::support::handle::{GlyphHandle, Handle, HandleState};


use crate::support::glyph_order::GlyphOrder;
use crate::support::primitives::{GlyphId, TableId};

use crate::table::otl::Subtable;

use crate::consolidate::otl::common::fontop_consolidate_coverage;

pub fn consolidate_gsub_reverse(
    glyph_order: &GlyphOrder,
    _subtable: &mut Subtable,
) -> bool {
    let Subtable::GsubReverse(subtable) = _subtable else {
        unreachable!()
    };
    // Guaranteed `Some`: `consolidate_otl` (and hence every caller that
    // reaches here) only ever runs when `glyf` is present, and
    // `consolidate_font` always populates `glyph_order` before
    // that, whenever `glyf` is present.
    let match_count = subtable.match_count as usize;
    for cov in subtable.sequence.iter_mut().take(match_count) {
        fontop_consolidate_coverage(glyph_order, cov);
    }
    fontop_consolidate_coverage(glyph_order, &mut subtable.to);
    if subtable.input_index as i32 >= subtable.match_count as i32 {
        subtable.input_index = (subtable.match_count as i32 - 1_i32) as TableId;
    }
    let input_index = subtable.input_index as usize;
    // Deduplicates by `from`'s glyph id, first occurrence wins -- a later
    // duplicate is logged as a warning and dropped, not merged. `BTreeMap`
    // so the survivors come back out sorted by that id. Names are copied
    // into the map up front and `from`/`to` are rebuilt from scratch
    // afterward, so no survivor is read after truncation could have
    // dropped it.
    let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, i32, Vec<u8>)> =
        std::collections::BTreeMap::new();
    // `.zip()` stops at the shorter side on its own, the same bound `n =
    // min(...)` computed by hand.
    for (from, to) in subtable.sequence[input_index].iter().zip(subtable.to.iter()) {
        let fromid: i32 = from.index as i32;
        if let std::collections::btree_map::Entry::Vacant(e) = seen.entry(fromid) {
            let toid: i32 = to.index as i32;
            let fromname: Vec<u8> = from.name.clone();
            let toname: Vec<u8> = to.name.clone();
            e.insert((fromname, toid, toname));
        } else {
            tracing::warn!("[Consolidate] Double-mapping a glyph in a reverse substitution /{}.\n", ByteStr(&from.name));
        }
    }
    let count: usize = seen.len();
    if count != subtable.sequence[input_index].len() || count != subtable.to.len() {
        tracing::warn!("[Consolidate] In this reverse subsitution lookup, some mappings are ignored.\n");
    }
    subtable.sequence[input_index] = Vec::new();
    subtable.to = Vec::new();
    for (fromid, (fromname, toid, toname)) in seen {
        subtable.sequence[input_index].push(Handle::new(HandleState::Consolidated, fromid as GlyphId, fromname) as GlyphHandle);
        subtable.to.push(Handle::new(HandleState::Consolidated, toid as GlyphId, toname) as GlyphHandle);
    }
    return false;
}
