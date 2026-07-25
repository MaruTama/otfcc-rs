extern "C" {
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
}
use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{otfcc_GlyphHandle, otfcc_Handle, otfcc_Handle_dispose};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};

use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphclass_t, glyphid_t};
use crate::vendor::sds::{sds};
use crate::font::caryll_font::{otfcc_Font};
use crate::support::glyph_order::{glyph_handle, otfcc_GlyphOrderPackage};

























use crate::table::otl::classdef::{otl_ClassDef};









#[no_mangle]
pub unsafe extern "C" fn fontop_consolidateCoverage(
    mut font: *mut otfcc_Font,
    mut coverage: *mut otl_Coverage,
    mut options: *const otfcc_Options,
) {
    if coverage.is_null() {
        return;
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*coverage).numGlyphs as ::core::ffi::c_int {
        let mut h: *mut glyph_handle = (*coverage).glyphs.offset(j as isize) as *mut glyph_handle;
        if !otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order, h as *mut otfcc_GlyphHandle
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
                    (*h).name,
                    b".\n",
                ),
            );
            otfcc_Handle_dispose(h as *mut otfcc_Handle);
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fontop_consolidateClassDef(
    mut font: *mut otfcc_Font,
    mut cd: *mut otl_ClassDef,
    mut options: *const otfcc_Options,
) {
    if cd.is_null() {
        return;
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
        let mut h: *mut glyph_handle = (*cd).glyphs.offset(j as isize) as *mut glyph_handle;
        if !otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order, h as *mut otfcc_GlyphHandle
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
                    (*h).name,
                    b".\n",
                ),
            );
            otfcc_Handle_dispose(h as *mut otfcc_Handle);
            *(*cd).classes.offset(j as isize) = 0 as glyphclass_t;
        }
        j = j.wrapping_add(1);
    }
}
