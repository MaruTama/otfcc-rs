use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::handle::otfcc_handle_dispose;
use crate::table::otl::coverage::Coverage;

use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use crate::support::primitives::{GlyphClass, GlyphId};

use crate::support::glyph_order::otfcc_gord_consolidate_handle;
use crate::table::otl::classdef::ClassDef;

// Takes `glyph_order: &GlyphOrder` directly rather than `font: &Font`
// (same reason as `consolidate_tsi`'s split `glyf`/`glyph_order`
// parameters, above in `consolidate.rs`): the one caller that reaches
// this from a `&mut Font` (TSI5 consolidation) needs to mutably borrow a
// *different* field (`font.tsi5`) in the same call, which a `font: &Font`
// parameter here would block by requiring the whole struct immutably.
pub fn fontop_consolidate_coverage(
    glyph_order: &GlyphOrder,
    coverage: &mut Coverage,
    options: &Options,
) {
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < coverage.len() {
        if !otfcc_gord_consolidate_handle(glyph_order, &mut coverage[j as usize]) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Ignored missing glyph /",
                    &coverage[j as usize].name,
                    b".\n",
                ),
            );
            otfcc_handle_dispose(&mut coverage[j as usize]);
        }
        j = j.wrapping_add(1);
    }
}
pub fn fontop_consolidate_class_def(
    glyph_order: Option<&GlyphOrder>,
    cd: Option<&mut ClassDef>,
    options: &Options,
) {
    let Some(cd) = cd else {
        return;
    };
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
    let Some(glyph_order) = glyph_order else {
        return;
    };
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < cd.glyphs.len() {
        if !otfcc_gord_consolidate_handle(glyph_order, &mut cd.glyphs[j as usize]) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Ignored missing glyph /",
                    &cd.glyphs[j as usize].name,
                    b".\n",
                ),
            );
            otfcc_handle_dispose(&mut cd.glyphs[j as usize]);
            cd.classes[j as usize] = 0 as GlyphClass;
        }
        j = j.wrapping_add(1);
    }
}
