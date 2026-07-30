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
use crate::table::otl::subtables::gsub_ligature::{I_SUBTABLE_GSUB_LIGATURE};
use crate::vendor::sds::{sdsempty};










pub unsafe extern "C" fn consolidate_gsub_ligature(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GsubLigatureSubtable = &raw mut (*_subtable).gsub_ligature;
    let mut nt: GsubLigatureSubtable = GsubLigatureSubtable {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<GsubLigatureEntry>(),
    };
    I_SUBTABLE_GSUB_LIGATURE
        .init
        .expect("non-null function pointer")(&raw mut nt);
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*subtable).length {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*subtable).items.offset(k as isize)).to,
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
                    (*(*subtable).items.offset(k as isize)).to.name,
                    b".\n",
                ),
            );
        } else {
            fontop_consolidate_coverage(font, (*(*subtable).items.offset(k as isize)).from, options);
            shrink_coverage(
                (*(*subtable).items.offset(k as isize)).from,
                false,
            );
            if (*(*(*subtable).items.offset(k as isize)).from).num_glyphs == 0 {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignoring empty ligature substitution to glyph /",
                        (*(*subtable).items.offset(k as isize)).to.name,
                        b".\n",
                    ),
                );
            } else {
                I_SUBTABLE_GSUB_LIGATURE
                    .push
                    .expect("non-null function pointer")(
                    &raw mut nt,
                    GsubLigatureEntry {
                        from: (*(*subtable).items.offset(k as isize)).from,
                        to: otfcc_handle_dup(
                            (*(*subtable).items.offset(k as isize)).to as Handle,
                        ) as GlyphHandle,
                    },
                );
                let ref mut fresh0 = (*(*subtable).items.offset(k as isize)).from;
                *fresh0 = ::core::ptr::null_mut::<Coverage>();
            }
        }
        k = k.wrapping_add(1);
    }
    I_SUBTABLE_GSUB_LIGATURE
        .replace
        .expect("non-null function pointer")(subtable, nt);
    return (*subtable).length == 0 as usize;
}
