use crate::logger::ByteStr;
use crate::support::handle::{GlyphHandle, Handle, HandleState};


use crate::support::primitives::GlyphId;

use crate::support::glyph_order::GlyphOrder;

use crate::table::gdef::{CaretValueList, CaretValueRecord, GdefTable, clear_lig_carets};

use crate::consolidate::otl::common::fontop_consolidate_class_def;
use crate::support::glyph_order::gord_consolidate_handle;
use crate::table::otl::classdef::shrink_class_def;

pub fn consolidate_gdef(glyph_order: Option<&GlyphOrder>, gdef: Option<&mut GdefTable>) {
    let (Some(gdef), Some(glyph_order)) = (gdef, glyph_order) else {
        return;
    };
    if let Some(cd) = gdef.glyph_class_def.as_deref_mut() {
        fontop_consolidate_class_def(Some(glyph_order), Some(cd));
        let cd = gdef.glyph_class_def.as_deref_mut().unwrap();
        shrink_class_def(cd);
        if cd.glyphs.is_empty() {
            gdef.glyph_class_def = None;
        }
    }
    if let Some(cd) = gdef.mark_attach_class_def.as_deref_mut() {
        fontop_consolidate_class_def(Some(glyph_order), Some(cd));
        let cd = gdef.mark_attach_class_def.as_deref_mut().unwrap();
        shrink_class_def(cd);
        if cd.glyphs.is_empty() {
            gdef.mark_attach_class_def = None;
        }
    }
    if !gdef.lig_carets.is_empty() {
        let lig_carets: &mut Vec<CaretValueRecord> = &mut gdef.lig_carets;
        // Deduplicates by glyph id, first occurrence wins -- a later
        // duplicate is logged as a warning and dropped (its own caret list
        // simply stays behind in `lig_carets` and gets freed when that
        // `Vec` is cleared below, since it was never taken out).
        // `BTreeMap` so the result is ascending by glyph id, not insertion
        // order. Each `CaretValueList` is moved out via `mem::take`.
        //
        // Unlike the other dedup passes, a glyph handle that fails to
        // resolve is silently skipped here, with no "[Consolidate] Ignored
        // missing glyph" warning.
        let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, CaretValueList)> =
            std::collections::BTreeMap::new();
        for rec in lig_carets.iter_mut() {
            if gord_consolidate_handle(glyph_order, &mut rec.glyph) {
                let gid: i32 = rec.glyph.index as i32;
                if let std::collections::btree_map::Entry::Vacant(e) = seen.entry(gid) {
                    let gname: Vec<u8> = rec.glyph.name.clone();
                    if !gname.is_empty() {
                        let carets: CaretValueList = ::core::mem::take(&mut rec.carets);
                        e.insert((gname, carets));
                    }
                } else {
                    tracing::warn!("[Consolidate] Detected caret value double-mapping about glyph {}", ByteStr(&rec.glyph.name));
                }
            }
        }
        clear_lig_carets(&mut gdef.lig_carets);
        for (gid, (gname, carets)) in seen {
            gdef.lig_carets.push(CaretValueRecord {
                glyph: Handle::new(HandleState::Consolidated, gid as GlyphId, gname) as GlyphHandle,
                carets,
            });
        }
    }
}
