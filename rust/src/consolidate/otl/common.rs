#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::handle::{GlyphHandle, otfcc_handle_dispose};
use crate::table::otl::coverage::Coverage;

use crate::font::caryll_font::Font;
use crate::support::options::Options;
use crate::support::primitives::{GlyphClass, GlyphId};

use crate::support::glyph_order::otfcc_gord_consolidate_handle;
use crate::table::otl::classdef::ClassDef;

pub unsafe fn fontop_consolidate_coverage(
    font: *mut Font,
    coverage: *mut Coverage,
    options: &Options,
) {
    if coverage.is_null() {
        return;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*coverage).len() {
        let h: *mut GlyphHandle = &raw mut (&mut (*coverage))[j as usize];
        // Guaranteed `Some`: `consolidate_otl` (and hence every caller that
        // reaches here) only ever runs when `glyf` is present, and
        // `otfcc_consolidate_font` always populates `glyph_order` before
        // that, whenever `glyf` is present.
        if !otfcc_gord_consolidate_handle((*font).glyph_order.as_deref().unwrap(), &mut *h) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[Consolidate] Ignored missing glyph /", &(*h).name, b".\n",),
            );
            otfcc_handle_dispose(&mut *h);
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe fn fontop_consolidate_class_def(
    font: *mut Font,
    cd: *mut ClassDef,
    options: &Options,
) {
    if cd.is_null() {
        return;
    }
    // Unlike this file's other caller chain (OTL lookup consolidation,
    // always glyf-gated by the time it reaches here), `consolidate.rs`
    // also calls this directly for TSI5, which is NOT glyf-gated -- a
    // JSON font can declare a `TSI5` table with no `glyf` table at all, in
    // which case `glyph_order` is never created. The original C (and this
    // function's own body below) unconditionally dereferenced `go` with no
    // null check, so that combination was a null-pointer segfault; this
    // early return is a genuine fix uncovered by requiring a real
    // `&GlyphOrder` here; every glyph reference in the table below simply
    // stays unresolved, matching how an unresolved reference is already
    // handled elsewhere in this file.
    let Some(glyph_order) = (*font).glyph_order.as_deref() else {
        return;
    };
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*cd).glyphs.len() {
        let h: *mut GlyphHandle = &raw mut (&mut (*cd).glyphs)[j as usize];
        if !otfcc_gord_consolidate_handle(glyph_order, &mut *h) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[Consolidate] Ignored missing glyph /", &(*h).name, b".\n",),
            );
            otfcc_handle_dispose(&mut *h);
            (&mut (*cd).classes)[j as usize] = 0 as GlyphClass;
        }
        j = j.wrapping_add(1);
    }
}
