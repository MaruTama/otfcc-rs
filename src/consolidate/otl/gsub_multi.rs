use crate::logger::ByteStr;
use crate::support::handle::{GlyphHandle, Handle, HandleState};
use crate::table::otl::coverage::{Coverage, shrink_coverage};

use crate::support::primitives::GlyphId;

use crate::support::glyph_order::GlyphOrder;

use crate::table::otl::{GsubMultiEntry, Subtable};

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::support::glyph_order::gord_consolidate_handle;

/// Resolves the glyphs of a multiple (`deletions_allowed`) or alternate
/// substitution subtable and drops what does not resolve.
///
/// A multiple substitution to no glyphs deletes its glyph. The OpenType
/// specification prohibits it, but fonts use it (Noto Sans Javanese) and
/// HarfBuzz applies it, so one the font already has is kept, with a warning.
/// A sequence emptied here, because none of its glyphs exists, is dropped
/// as before rather than turned into a deletion. An empty alternate set
/// does nothing and is dropped.
pub fn consolidate_gsub_multi(
    glyph_order: &GlyphOrder,
    _subtable: &mut Subtable,
    deletions_allowed: bool,
) -> bool {
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
            tracing::warn!(
                "[Consolidate] Ignored missing glyph /{}.\n",
                ByteStr(&entry.from.name)
            );
        } else {
            let deletion = deletions_allowed && entry.to.is_empty();
            fontop_consolidate_coverage(glyph_order, &mut entry.to);
            shrink_coverage(&mut entry.to, false);
            if deletion {
                tracing::warn!(
                    "[Consolidate] Multiple substitution of glyph /{} to no glyphs is against the OpenType specification; it is kept.\n",
                    ByteStr(&entry.from.name)
                );
            }
            if entry.to.is_empty() && !deletion {
                tracing::warn!(
                    "[Consolidate] Ignoring empty one-to-many / alternative substitution for glyph /{}.\n",
                    ByteStr(&entry.from.name)
                );
            } else {
                let fromid: i32 = i32::from(entry.from.index);
                if let std::collections::btree_map::Entry::Vacant(e) = seen.entry(fromid) {
                    let fromname: Vec<u8> = entry.from.name.clone();
                    let to: Coverage = ::core::mem::take(&mut entry.to);
                    e.insert((fromname, to));
                }
            }
        }
    }
    subtable.clear();
    for (fromid, (fromname, to)) in seen {
        subtable.push(GsubMultiEntry {
            from: Handle::new(HandleState::Consolidated, fromid as GlyphId, fromname)
                as GlyphHandle,
            to,
        });
    }
    subtable.is_empty()
}
