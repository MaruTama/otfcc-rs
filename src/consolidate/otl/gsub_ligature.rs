use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::table::otl::coverage::shrink_coverage;

use crate::support::options::Options;

use crate::support::glyph_order::GlyphOrder;

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::support::glyph_order::otfcc_gord_consolidate_handle;
use crate::table::otl::subtables::gsub_ligature::subtable_gsub_ligature_replace;
use crate::table::otl::{GsubLigatureEntry, GsubLigatureSubtable, Subtable};

pub fn consolidate_gsub_ligature(
    glyph_order: &GlyphOrder,
    _subtable: &mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::GsubLigature(subtable) = _subtable else {
        unreachable!()
    };
    let mut nt: GsubLigatureSubtable = Vec::new();
    for entry in subtable.iter_mut() {
        // Guaranteed `Some`: `consolidate_otl` (and hence this function)
        // only ever runs when `glyf` is present, and `otfcc_consolidate_font`
        // always populates `glyph_order` before that, whenever `glyf` is
        // present.
        if !otfcc_gord_consolidate_handle(glyph_order, &mut entry.to) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[Consolidate] Ignored missing glyph /", &entry.to.name, b".\n",),
            );
        } else {
            fontop_consolidate_coverage(glyph_order, &mut entry.from, options);
            shrink_coverage(&mut entry.from, false);
            if entry.from.is_empty() {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[Consolidate] Ignoring empty ligature substitution to glyph /",
                        &entry.to.name,
                        b".\n",
                    ),
                );
            } else {
                nt.push(GsubLigatureEntry {
                    from: ::core::mem::take(&mut entry.from),
                    to: entry.to.clone(),
                });
            }
        }
    }
    subtable_gsub_ligature_replace(subtable, nt);
    subtable.is_empty()
}
