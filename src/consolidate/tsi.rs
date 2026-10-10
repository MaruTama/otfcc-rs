use crate::logger::ByteStr;
use crate::support::glyph_order::GlyphOrder;
use crate::support::glyph_order::gord_consolidate_handle;
use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_index,
};
use crate::support::primitives::GlyphId;
use crate::table::glyf::GlyfTable;
use crate::table::tsi::tsi_entry_dup;
use crate::table::tsi::{TsiEntry, TsiEntryType, TsiTable};

// Takes `glyf`/`glyph_order` as separate borrowed pieces (rather than
// `font: &mut Font` alongside `tsi: &mut Option<TsiTable>`, which would
// be two simultaneous borrows of the same `Font` whenever a caller
// passes `&mut font.tsi_01`/`&mut font.tsi_23`) -- the caller
// (`consolidate_font`) borrows `font.glyf`/`font.glyph_order`
// (shared) and `font.tsi_01`/`font.tsi_23` (mutable, one at a time) as
// disjoint fields directly off `font`, which Rust allows even though a
// single `&Font`/`&mut Font` funneled through this function's own
// parameter list would not.
pub(crate) fn consolidate_tsi(glyf: &GlyfTable, glyph_order: &GlyphOrder, tsi: &mut Option<TsiTable>) {
    if tsi.is_none() {
        return;
    }
    let mut consolidated: TsiTable = Vec::new();
    // `Option<Vec<u8>>` per slot preserves the old null/non-null
    // distinction (`None` = "no entry yet for this GID", `Some` = has
    // content, even if empty) that the raw `*mut SdsRaw` array's
    // `is_null()` checks relied on -- a plain assignment below correctly
    // drops whatever was there before, so the old explicit
    // free-before-overwrite is now implicit.
    let mut gid_entries: Vec<Option<Vec<u8>>> = vec![None; glyf.len()];
    let entries: &mut Vec<TsiEntry> = tsi.as_mut().unwrap();
    for entry in entries.iter_mut() {
        if entry.kind == TsiEntryType::Glyph {
            if gord_consolidate_handle(glyph_order, &mut entry.glyph) {
                gid_entries[entry.glyph.index as usize] =
                    Some(::core::mem::take(&mut entry.content));
            } else {
                tracing::warn!("[Consolidate] Ignored missing glyph of /{}", ByteStr(&entry.glyph.name));
            }
        } else {
            // `tsi_entry_dup` is a safe fn now that this stack includes
            // #364's `tsi.rs` conversion.
            consolidated.push(tsi_entry_dup(entry));
        }
    }
    for (j, entry) in gid_entries.iter_mut().enumerate() {
        let mut e_0: TsiEntry = TsiEntry {
            kind: TsiEntryType::Glyph,
            glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
            content: Vec::new(),
        };
        e_0.kind = TsiEntryType::Glyph;
        e_0.glyph = handle_from_index(j as GlyphId) as GlyphHandle;
        gord_consolidate_handle(glyph_order, &mut e_0.glyph);
        e_0.content = entry.take().unwrap_or_default();
        consolidated.push(e_0);
    }
    consolidated.sort_by(|a, b| {
        (a.kind as u32)
            .cmp(&(b.kind as u32))
            .then(a.glyph.index.cmp(&b.glyph.index))
    });
    // Old `tsi` (the previous value) drops naturally here, when this
    // assignment overwrites it -- no explicit `table_tsi_free` needed.
    *tsi = Some(consolidated);
}
