#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::table::otl::coverage::{Coverage, shrink_coverage};
use crate::support::handle::{GlyphHandle, Handle, otfcc_handle_dup};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};

use crate::font::caryll_font::{Font};

























use crate::table::otl::{GsubLigatureEntry, Subtable, GsubLigatureSubtable, OtlTable};
use crate::consolidate::otl::common::{fontop_consolidate_coverage};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::table::otl::subtables::gsub_ligature::{subtable_gsub_ligature_replace};
use crate::vendor::sds::{sdsempty};










pub unsafe extern "C" fn consolidate_gsub_ligature(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GsubLigatureSubtable = &raw mut (*_subtable).gsub_ligature as *mut GsubLigatureSubtable;
    let mut nt: GsubLigatureSubtable = Vec::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*subtable).len() {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order,
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
                    (&(*subtable))[k as usize].to.name,
                    b".\n",
                ),
            );
        } else {
            fontop_consolidate_coverage(font, (&(*subtable))[k as usize].from, options);
            shrink_coverage(
                (&(*subtable))[k as usize].from,
                false,
            );
            if (*(&(*subtable))[k as usize].from).num_glyphs == 0 {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignoring empty ligature substitution to glyph /",
                        (&(*subtable))[k as usize].to.name,
                        b".\n",
                    ),
                );
            } else {
                nt.push(
                    GsubLigatureEntry {
                        from: (&(*subtable))[k as usize].from,
                        to: otfcc_handle_dup(
                            (&(*subtable))[k as usize].to.clone() as Handle,
                        ) as GlyphHandle,
                    },
                );
                let ref mut fresh0 = (&mut (*subtable))[k as usize].from;
                *fresh0 = ::core::ptr::null_mut::<Coverage>();
            }
        }
        k = k.wrapping_add(1);
    }
    subtable_gsub_ligature_replace(subtable, nt);
    return (*subtable).len() == 0 as usize;
}
