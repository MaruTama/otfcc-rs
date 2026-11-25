use crate::font::model::Font;
use crate::logger::ByteStr;
use crate::support::glyph_order::GlyphOrder;
use crate::support::glyph_order::gord_consolidate_handle;
use crate::support::handle::Handle;

pub fn consolidate_cmap(font: &mut Font) {
    let glyph_order: Option<&GlyphOrder> = font.glyph_order.as_deref();
    if let Some(glyph_order) = glyph_order.filter(|_| font.cmap.is_some()) {
        // A mapping to a glyph the font does not have is dropped, and so is
        // one to glyph 0, the missing glyph: a code mapped to it is not
        // mapped, and the font reads back without it.
        font.cmap
            .as_mut()
            .expect("the filter above checked cmap")
            .unicodes
            .retain(|&unicode, glyph| {
                if !gord_consolidate_handle(glyph_order, glyph) {
                    tracing::warn!(
                        "[Consolidate] Ignored mapping U+{:04X} to non-existent glyph /{}.\n",
                        unicode as u32,
                        ByteStr(&glyph.name)
                    );
                    return false;
                }
                glyph.index != 0
            });
    }
    if let Some(glyph_order) = glyph_order.filter(|_| font.cmap.is_some()) {
        for (key, glyph) in font
            .cmap
            .as_mut()
            .expect("the filter above checked cmap")
            .uvs
            .iter_mut()
        {
            if !gord_consolidate_handle(glyph_order, glyph) {
                tracing::warn!(
                    "[Consolidate] Ignored UVS mapping [U+{:04X} U+{:04X}] to non-existent glyph /{}.\n",
                    key.unicode,
                    key.selector,
                    ByteStr(&glyph.name)
                );
                *glyph = Handle::default();
            }
        }
    }
}
