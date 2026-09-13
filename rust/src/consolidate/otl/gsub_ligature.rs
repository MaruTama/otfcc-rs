use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::handle::{GlyphHandle, Handle, otfcc_handle_dup};
use crate::table::otl::coverage::shrink_coverage;

use crate::support::options::Options;
use crate::support::primitives::GlyphId;

use crate::font::caryll_font::Font;

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::support::glyph_order::otfcc_gord_consolidate_handle;
use crate::table::otl::subtables::gsub_ligature::subtable_gsub_ligature_replace;
use crate::table::otl::{GsubLigatureEntry, GsubLigatureSubtable, OtlTable, Subtable};

pub fn consolidate_gsub_ligature(
    font: &Font,
    _table: *const OtlTable,
    _subtable: &mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::GsubLigature(subtable) = _subtable else {
        unreachable!()
    };
    let mut nt: GsubLigatureSubtable = Vec::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < subtable.len() {
        // Guaranteed `Some`: `consolidate_otl` (and hence this function)
        // only ever runs when `glyf` is present, and `otfcc_consolidate_font`
        // always populates `glyph_order` before that, whenever `glyf` is
        // present.
        if !otfcc_gord_consolidate_handle(
            font.glyph_order.as_deref().unwrap(),
            &mut subtable[k as usize].to,
        ) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Ignored missing glyph /",
                    &subtable[k as usize].to.name,
                    b".\n",
                ),
            );
        } else {
            fontop_consolidate_coverage(
                font.glyph_order.as_deref().unwrap(),
                &mut subtable[k as usize].from,
                options,
            );
            shrink_coverage(&mut subtable[k as usize].from, false);
            if subtable[k as usize].from.is_empty() {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[Consolidate] Ignoring empty ligature substitution to glyph /",
                        &subtable[k as usize].to.name,
                        b".\n",
                    ),
                );
            } else {
                nt.push(GsubLigatureEntry {
                    from: ::core::mem::take(&mut subtable[k as usize].from),
                    to: otfcc_handle_dup(subtable[k as usize].to.clone() as Handle) as GlyphHandle,
                });
            }
        }
        k = k.wrapping_add(1);
    }
    subtable_gsub_ligature_replace(subtable, nt);
    subtable.len() == 0_usize
}
