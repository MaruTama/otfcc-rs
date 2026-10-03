use crate::logger::ByteStr;
use crate::table::otl::coverage::shrink_coverage;


use crate::support::glyph_order::GlyphOrder;

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::support::glyph_order::gord_consolidate_handle;
use crate::table::otl::subtables::gsub_ligature::subtable_gsub_ligature_replace;
use crate::table::otl::{GsubLigatureEntry, GsubLigatureSubtable, Subtable};

pub fn consolidate_gsub_ligature(
    glyph_order: &GlyphOrder,
    _subtable: &mut Subtable,
) -> bool {
    let Subtable::GsubLigature(subtable) = _subtable else {
        unreachable!()
    };
    let mut nt: GsubLigatureSubtable = Vec::new();
    for entry in subtable.iter_mut() {
        // Guaranteed `Some`: `consolidate_otl` (and hence this function)
        // only ever runs when `glyf` is present, and `consolidate_font`
        // always populates `glyph_order` before that, whenever `glyf` is
        // present.
        if !gord_consolidate_handle(glyph_order, &mut entry.to) {
            tracing::warn!("[Consolidate] Ignored missing glyph /{}.\n", ByteStr(&entry.to.name));
        } else {
            fontop_consolidate_coverage(glyph_order, &mut entry.from);
            shrink_coverage(&mut entry.from, false);
            if entry.from.is_empty() {
                tracing::warn!("[Consolidate] Ignoring empty ligature substitution to glyph /{}.\n", ByteStr(&entry.to.name));
            } else {
                nt.push(GsubLigatureEntry {
                    from: ::core::mem::take(&mut entry.from),
                    to: entry.to.clone(),
                });
            }
        }
    }
    subtable_gsub_ligature_replace(subtable, nt);
    subtable.is_empty()
}
