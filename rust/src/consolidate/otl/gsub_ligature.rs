#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
unsafe extern "C" {
    fn sdsempty() -> sds;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static iSubtable_gsub_ligature: __caryll_vectorinterface_subtable_gsub_ligature;
    static otl_iCoverage: __otfcc_ICoverage;
    fn fontop_consolidateCoverage(
        font: *mut otfcc_Font,
        coverage: *mut otl_Coverage,
        options: *const otfcc_Options,
    );
}
use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, shrinkCoverage};
use crate::support::handle::{otfcc_GlyphHandle, otfcc_Handle, otfcc_Handle_dup};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};

use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphid_t};
use crate::vendor::sds::{sds};

use crate::font::caryll_font::{otfcc_Font};

use crate::support::glyph_order::{otfcc_GlyphOrderPackage};
























use crate::table::otl::{__caryll_vectorinterface_subtable_gsub_ligature, otl_GsubLigatureEntry, otl_Subtable, subtable_gsub_ligature, table_OTL};










#[unsafe(no_mangle)]
pub unsafe extern "C" fn consolidate_gsub_ligature(
    mut font: *mut otfcc_Font,
    mut _table: *mut table_OTL,
    mut _subtable: *mut otl_Subtable,
    mut options: *const otfcc_Options,
) -> bool {
    let mut subtable: *mut subtable_gsub_ligature = &raw mut (*_subtable).gsub_ligature;
    let mut nt: subtable_gsub_ligature = subtable_gsub_ligature {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<otl_GsubLigatureEntry>(),
    };
    iSubtable_gsub_ligature
        .init
        .expect("non-null function pointer")(&raw mut nt);
    let mut k: glyphid_t = 0 as glyphid_t;
    while (k as usize) < (*subtable).length {
        if !otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*subtable).items.offset(k as isize)).to,
        ) {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored missing glyph /",
                    (*(*subtable).items.offset(k as isize)).to.name,
                    b".\n",
                ),
            );
        } else {
            fontop_consolidateCoverage(font, (*(*subtable).items.offset(k as isize)).from, options);
            shrinkCoverage(
                (*(*subtable).items.offset(k as isize)).from,
                false,
            );
            if (*(*(*subtable).items.offset(k as isize)).from).numGlyphs == 0 {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignoring empty ligature substitution to glyph /",
                        (*(*subtable).items.offset(k as isize)).to.name,
                        b".\n",
                    ),
                );
            } else {
                iSubtable_gsub_ligature
                    .push
                    .expect("non-null function pointer")(
                    &raw mut nt,
                    otl_GsubLigatureEntry {
                        from: (*(*subtable).items.offset(k as isize)).from,
                        to: otfcc_Handle_dup(
                            (*(*subtable).items.offset(k as isize)).to as otfcc_Handle,
                        ) as otfcc_GlyphHandle,
                    },
                );
                let ref mut fresh0 = (*(*subtable).items.offset(k as isize)).from;
                *fresh0 = ::core::ptr::null_mut::<otl_Coverage>();
            }
        }
        k = k.wrapping_add(1);
    }
    iSubtable_gsub_ligature
        .replace
        .expect("non-null function pointer")(subtable, nt);
    return (*subtable).length == 0 as usize;
}
