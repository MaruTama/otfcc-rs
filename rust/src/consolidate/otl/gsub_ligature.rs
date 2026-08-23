#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::handle::{GlyphHandle, Handle, otfcc_handle_dup};
use crate::table::otl::coverage::{Coverage, shrink_coverage};

use crate::support::options::Options;
use crate::support::primitives::GlyphId;

use crate::font::caryll_font::Font;

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::support::glyph_order::{GlyphOrder, otfcc_gord_consolidate_handle};
use crate::table::otl::subtables::gsub_ligature::subtable_gsub_ligature_replace;
use crate::table::otl::{GsubLigatureEntry, GsubLigatureSubtable, OtlTable, Subtable};

pub unsafe extern "C" fn consolidate_gsub_ligature(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let Subtable::GsubLigature(mut_subtable) = &mut *_subtable else {
        unreachable!()
    };
    let subtable: *mut GsubLigatureSubtable = mut_subtable;
    let mut nt: GsubLigatureSubtable = Vec::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*subtable).len() {
        if !otfcc_gord_consolidate_handle(
            (*font)
                .glyph_order
                .as_deref_mut()
                .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder),
            &raw mut (&mut (*subtable))[k as usize].to,
        ) {
            logger_log_sds(
                &mut *(*options).logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Ignored missing glyph /",
                    &(&(*subtable))[k as usize].to.name,
                    b".\n",
                ),
            );
        } else {
            fontop_consolidate_coverage(
                font,
                &mut (&mut (*subtable))[k as usize].from as *mut Coverage,
                &*options,
            );
            shrink_coverage(
                &mut (&mut (*subtable))[k as usize].from as *mut Coverage,
                false,
            );
            if (&(*subtable))[k as usize].from.is_empty() {
                logger_log_sds(
                    &mut *(*options).logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[Consolidate] Ignoring empty ligature substitution to glyph /",
                        &(&(*subtable))[k as usize].to.name,
                        b".\n",
                    ),
                );
            } else {
                nt.push(GsubLigatureEntry {
                    from: ::core::mem::take(&mut (&mut (*subtable))[k as usize].from),
                    to: otfcc_handle_dup((&(*subtable))[k as usize].to.clone() as Handle)
                        as GlyphHandle,
                });
            }
        }
        k = k.wrapping_add(1);
    }
    subtable_gsub_ligature_replace(subtable, nt);
    return (*subtable).len() == 0 as usize;
}
