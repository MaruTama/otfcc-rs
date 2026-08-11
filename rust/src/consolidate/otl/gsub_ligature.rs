#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::table::otl::coverage::{Coverage, shrink_coverage};
use crate::support::handle::{GlyphHandle, Handle, otfcc_handle_dup};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};

use crate::font::caryll_font::{Font};

























use crate::table::otl::{GsubLigatureEntry, Subtable, GsubLigatureSubtable, OtlTable};
use crate::consolidate::otl::common::{fontop_consolidate_coverage};
use crate::support::glyph_order::{GlyphOrder, OTFCC_PKG_GLYPH_ORDER};
use crate::table::otl::subtables::gsub_ligature::{subtable_gsub_ligature_replace};
use crate::vendor::sds::{sdsempty};










pub unsafe extern "C" fn consolidate_gsub_ligature(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let Subtable::GsubLigature(mut_subtable) = &mut *_subtable else { unreachable!() };
    let subtable: *mut GsubLigatureSubtable = mut_subtable;
    let mut nt: GsubLigatureSubtable = Vec::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*subtable).len() {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order.as_deref_mut().map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder),
            &raw mut (&mut (*subtable))[k as usize].to,
        ) {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored missing glyph /",
                    &(&(*subtable))[k as usize].to.name,
                    b".\n",
                ),
            );
        } else {
            fontop_consolidate_coverage(
                font,
                &mut (&mut (*subtable))[k as usize].from as *mut Coverage,
                options,
            );
            shrink_coverage(
                &mut (&mut (*subtable))[k as usize].from as *mut Coverage,
                false,
            );
            if (&(*subtable))[k as usize].from.is_empty() {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignoring empty ligature substitution to glyph /",
                        &(&(*subtable))[k as usize].to.name,
                        b".\n",
                    ),
                );
            } else {
                nt.push(
                    GsubLigatureEntry {
                        from: ::core::mem::take(&mut (&mut (*subtable))[k as usize].from),
                        to: otfcc_handle_dup(
                            (&(*subtable))[k as usize].to.clone() as Handle,
                        ) as GlyphHandle,
                    },
                );
            }
        }
        k = k.wrapping_add(1);
    }
    subtable_gsub_ligature_replace(subtable, nt);
    return (*subtable).len() == 0 as usize;
}
