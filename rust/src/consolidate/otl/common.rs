#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::handle::{GlyphHandle, Handle, otfcc_handle_dispose};
use crate::table::otl::coverage::Coverage;

use crate::font::caryll_font::Font;
use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use crate::support::primitives::{GlyphClass, GlyphId};

use crate::support::glyph_order::otfcc_gord_consolidate_handle;
use crate::table::otl::classdef::ClassDef;

pub unsafe fn fontop_consolidate_coverage(
    font: *mut Font,
    coverage: *mut Coverage,
    options: &Options,
) {
    if coverage.is_null() {
        return;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*coverage).len() {
        let h: *mut GlyphHandle = &raw mut (&mut (*coverage))[j as usize];
        if !otfcc_gord_consolidate_handle(
            (*font)
                .glyph_order
                .as_deref_mut()
                .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder),
            h as *mut GlyphHandle,
        ) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[Consolidate] Ignored missing glyph /", &(*h).name, b".\n",),
            );
            otfcc_handle_dispose(h as *mut Handle);
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe fn fontop_consolidate_class_def(
    font: *mut Font,
    cd: *mut ClassDef,
    options: &Options,
) {
    if cd.is_null() {
        return;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*cd).glyphs.len() {
        let h: *mut GlyphHandle = &raw mut (&mut (*cd).glyphs)[j as usize];
        if !otfcc_gord_consolidate_handle(
            (*font)
                .glyph_order
                .as_deref_mut()
                .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder),
            h as *mut GlyphHandle,
        ) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[Consolidate] Ignored missing glyph /", &(*h).name, b".\n",),
            );
            otfcc_handle_dispose(h as *mut Handle);
            (&mut (*cd).classes)[j as usize] = 0 as GlyphClass;
        }
        j = j.wrapping_add(1);
    }
}
