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
    let glyph_order: &GlyphOrder = font.glyph_order.as_deref().unwrap();
    let mut consolidated: ColrTable = Vec::new();
    let source: &mut Vec<ColrMapping> = font.colr.as_mut().unwrap();
    for mapping in source.iter_mut() {
        if !gord_consolidate_handle(glyph_order, &mut mapping.glyph) {
            tracing::warn!("[Consolidate] Ignored missing glyph of /{}", ByteStr(&mapping.glyph.name));
        } else {
            let mut m: ColrMapping = ColrMapping {
                glyph: mapping.glyph.clone(),
                layers: Vec::new(),
            };
            for layer in mapping.layers.iter_mut() {
                if !gord_consolidate_handle(glyph_order, &mut layer.glyph) {
                    tracing::warn!("[Consolidate] Ignored missing glyph of /{}", ByteStr(&layer.glyph.name));
                } else {
                    m.layers.push(layer.clone());
                }
            }
            if !mapping.layers.is_empty() {
                consolidated.push(m);
            } else {
                tracing::warn!("[Consolidate] COLR decomposition for /{} is empth", ByteStr(&mapping.glyph.name));
                // `m` is dropped here (its `Handle` and `layers: Vec<ColrLayer>`
                // freed by their own compiler-generated drop glue) rather than
                // pushed into `consolidated` -- no manual dispose call needed.
            }
        }
    }
    font.colr = Some(consolidated);
}
