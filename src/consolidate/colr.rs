use crate::font::model::Font;
use crate::logger::ByteStr;
use crate::support::glyph_order::GlyphOrder;
use crate::support::glyph_order::gord_consolidate_handle;
use crate::table::colr::{ColrMapping, ColrTable};

pub(crate) fn consolidate_colr(font: &mut Font) {
    if font.colr.is_none() || font.glyph_order.is_none() {
        return;
    }
    // Guaranteed `Some` by the early return above.
    let glyph_order: &GlyphOrder = font
        .glyph_order
        .as_deref()
        .expect("the early return above checked glyph_order");
    let mut consolidated: ColrTable = Vec::new();
    let source: &mut Vec<ColrMapping> = font
        .colr
        .as_mut()
        .expect("the early return above checked colr");
    for mapping in source.iter_mut() {
        if !gord_consolidate_handle(glyph_order, &mut mapping.glyph) {
            tracing::warn!(
                "[Consolidate] Ignored missing glyph of /{}",
                ByteStr(&mapping.glyph.name)
            );
        } else {
            let mut m: ColrMapping = ColrMapping {
                glyph: mapping.glyph.clone(),
                layers: Vec::new(),
            };
            for layer in mapping.layers.iter_mut() {
                if !gord_consolidate_handle(glyph_order, &mut layer.glyph) {
                    tracing::warn!(
                        "[Consolidate] Ignored missing glyph of /{}",
                        ByteStr(&layer.glyph.name)
                    );
                } else {
                    m.layers.push(layer.clone());
                }
            }
            // A base glyph with no layers is drawn as nothing in color, which
            // the specification allows, so one the font has is kept. One that
            // only becomes empty because its layer glyphs do not exist is
            // dropped, so the glyph keeps its plain outline.
            if !m.layers.is_empty() || mapping.layers.is_empty() {
                consolidated.push(m);
            } else {
                tracing::warn!(
                    "[Consolidate] COLR decomposition for /{} is empty",
                    ByteStr(&mapping.glyph.name)
                );
            }
        }
    }
    font.colr = Some(consolidated);
}
