#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md


use crate::support::buffer::{Buffer};
use crate::support::options::{Options};


use crate::support::parsed_json::{ParsedValue};
use crate::font::caryll_font::{Font};
use crate::font::caryll_font::{otfcc_font_free};
use crate::consolidate::{otfcc_consolidate_font};
use crate::json_reader::{read_json};
use crate::logger::{otfcc_new_empty_target, otfcc_new_logger, logger_indent};
use crate::otf_writer::{serialize_to_otf};
use crate::support::buffer::{buffree};
use crate::support::options::{otfcc_options_optimize_to, otfcc_new_options};
use crate::support::parsed_json::{json_parse, json_value_free};





































#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfccbuild_json_otf(
    mut inlen: u32,
    mut injson: *const ::core::ffi::c_char,
    mut olevel: u8,
    mut for_webfont: bool,
) -> *mut Buffer {
    let mut options: *mut Options = otfcc_new_options();
    (*options).logger = otfcc_new_logger(otfcc_new_empty_target());
    logger_indent(
        (*options).logger,
        b"otfccbuild\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_options_optimize_to(options, olevel);
    if for_webfont {
        (*options).ignore_glyph_order = true;
        (*options).force_cid = true;
    }
    let mut json_root: *mut ParsedValue = json_parse(injson, inlen as usize);
    if json_root.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut font: *mut Font = read_json(json_root as *mut ::core::ffi::c_void, 0 as u32, options);
    json_value_free(json_root);
    if font.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    otfcc_consolidate_font(font, options);
    let mut otf: *mut Buffer = serialize_to_otf(font, options) as *mut Buffer;
    otfcc_font_free(font);
    return otf;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_get_buf_len(mut buf: *mut Buffer) -> usize {
    return (*buf).size;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_get_buf_data(mut buf: *mut Buffer) -> *mut u8 {
    return (*buf).data;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfccbuild_free_otfbuf(mut buf: *mut Buffer) {
    buffree(buf);
}
