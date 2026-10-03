//! Consolidation: resolving every glyph reference in a font against its
//! glyph order, and dropping what does not resolve, before the font is
//! written. `consolidate_font` builds the glyph order if the font has none
//! and then consolidates each table in `CONSOLIDATE_ORDER`; the per-table
//! work lives in one submodule per table (`otl` holds the lookups' and
//! GDEF's, one file per lookup kind).
pub mod cmap;
pub mod colr;
pub mod glyf;
pub mod otl;
pub mod tsi;

use crate::font::model::Font;
use crate::font::table_registry::CONSOLIDATE_ORDER;
use crate::logger::ByteStr;
use crate::support::glyph_order::GlyphOrder;
use crate::support::glyph_order::set_glyph_order_by_name;
use crate::support::options::Options;
use crate::support::primitives::GlyphId;

pub fn consolidate_font(font: &mut Font, options: &Options) {
    // See `Options::consolidate_warning_budget`'s own doc comment: reset
    // once per font here, not just once at `Options` creation, since one
    // `Options` can drive many conversions over its life.
    options
        .consolidate_warning_budget
        .set(crate::consolidate::otl::chaining::CONSOLIDATE_WARNING_BUDGET);
    if font.glyph_order.is_none()
        && let Some(glyf) = font.glyf.as_mut()
    {
        // Built directly via `Box::new`, not `OTFCC_PKG_GLYPH_ORDER.create`
        // (`malloc`) + `Box::from_raw` -- `Box::from_raw` requires the
        // pointer to have come from Rust's global allocator, which a bare
        // libc `malloc` is not guaranteed to match. `go` borrows `go_box`
        // for the rest of this block (unchanged from here down), matching
        // the `GaspTable`/`CmapTable` "accumulator is `Option<Box<X>>`/
        // `Box<X>` from the start" idiom.
        let mut go_box: Box<GlyphOrder> = Box::new(GlyphOrder {
            entries: Vec::new(),
            by_gid: ::std::collections::BTreeMap::new(),
            by_name: ::std::collections::HashMap::new(),
        });
        let go: &mut GlyphOrder = go_box.as_mut();
        for (gid, slot) in glyf.iter_mut().enumerate() {
            let g = slot.as_mut().unwrap();
            let gid = gid as GlyphId;
            let name: Vec<u8> = if g.name.is_empty() {
                let name = crate::bytesbuild!(b"$$gid", gid as i32);
                g.name = name.clone();
                name
            } else {
                g.name.clone()
            };
            // `.clone()`, not a move: `set_glyph_order_by_name` always
            // consumes its own copy (no ownership contract to track any
            // more -- see its doc comment), but `name` is still needed
            // below regardless of whether this call succeeds or fails, for
            // the log message and/or the retry loop.
            if !set_glyph_order_by_name(go, name.clone(), gid) {
                tracing::warn!("[Consolidate] Glyph name {} is already in use.", ByteStr(&name));
                let mut suffix: u32 = 2_u32;
                let mut success: bool;
                loop {
                    let newname: Vec<u8> = crate::bytesbuild!(&name, b"_", suffix);
                    success = set_glyph_order_by_name(go, newname.clone(), gid);
                    if !success {
                        suffix = suffix.wrapping_add(1_u32);
                    } else {
                        tracing::warn!("[Consolidate] Glyph {} is renamed into {}.", ByteStr(&name), ByteStr(&newname));
                        g.name = newname;
                    }
                    if success {
                        break;
                    }
                }
            }
        }
        font.glyph_order = Some(go_box);
    }
    for table in CONSOLIDATE_ORDER {
        table.consolidate(font, options);
    }
}

