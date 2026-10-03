use crate::font::model::Font;
use crate::logger::ByteStr;
use crate::support::glyph_order::GlyphOrder;
use crate::support::glyph_order::gord_consolidate_handle;
use crate::support::handle::Handle;

pub fn consolidate_cmap(font: &mut Font) {
    let glyph_order: Option<&GlyphOrder> = font.glyph_order.as_deref();
    if let Some(glyph_order) = glyph_order.filter(|_| font.cmap.is_some()) {
        // A failed resolution disposes the entry's `Handle` in place
        // (leaving it in the map with an empty name) rather than
        // removing the entry -- `dump_cmap`'s "skip if name is null"
        // check is what actually hides it later.
        for (&unicode, glyph) in font.cmap.as_mut().unwrap().unicodes.iter_mut() {
            if !gord_consolidate_handle(glyph_order, glyph) {
                tracing::warn!("[Consolidate] Ignored mapping U+{:04X} to non-existent glyph /{}.\n", unicode as u32, ByteStr(&glyph.name));
                *glyph = Handle::default();
            }
        }
    }
    if let Some(glyph_order) = glyph_order.filter(|_| font.cmap.is_some()) {
        for (key, glyph) in font.cmap.as_mut().unwrap().uvs.iter_mut() {
            if !gord_consolidate_handle(glyph_order, glyph) {
                tracing::warn!("[Consolidate] Ignored UVS mapping [U+{:04X} U+{:04X}] to non-existent glyph /{}.\n", key.unicode, key.selector, ByteStr(&glyph.name));
                *glyph = Handle::default();
            }
        }
    }
}
