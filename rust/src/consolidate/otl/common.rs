#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::table::otl::coverage::{Coverage};
use crate::support::handle::{GlyphHandle, Handle, otfcc_handle_dispose};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphClass, GlyphId};
use crate::font::caryll_font::{Font};

























use crate::table::otl::classdef::{ClassDef};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::vendor::sds::{sdsempty};









pub unsafe extern "C" fn fontop_consolidate_coverage(
    mut font: *mut Font,
    mut coverage: *mut Coverage,
    mut options: *const Options,
) {
    if coverage.is_null() {
        return;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*coverage).len() {
        let mut h: *mut GlyphHandle = &raw mut (&mut (*coverage))[j as usize];
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order, h as *mut GlyphHandle
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
                    (*h).name,
                    b".\n",
                ),
            );
            otfcc_handle_dispose(h as *mut Handle);
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe extern "C" fn fontop_consolidate_class_def(
    mut font: *mut Font,
    mut cd: *mut ClassDef,
    mut options: *const Options,
) {
    if cd.is_null() {
        return;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*cd).glyphs.len() {
        let mut h: *mut GlyphHandle = &raw mut (&mut (*cd).glyphs)[j as usize];
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order, h as *mut GlyphHandle
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
                    (*h).name,
                    b".\n",
                ),
            );
            otfcc_handle_dispose(h as *mut Handle);
            (&mut (*cd).classes)[j as usize] = 0 as GlyphClass;
        }
        j = j.wrapping_add(1);
    }
}
