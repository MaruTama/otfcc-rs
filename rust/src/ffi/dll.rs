#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md


use crate::logger::ILogger;
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};


use crate::vendor::json::{JsonValue};
use crate::font::caryll_font::{Font, IFontBuilder, IFontSerializer};
use crate::font::caryll_font::{otfcc_iFont};
use crate::json_reader::{otfcc_newJsonReader};
use crate::logger::{otfcc_newEmptyTarget, otfcc_newLogger};
use crate::otf_writer::{otfcc_newOTFWriter};
use crate::support::buffer::{buffree};
use crate::support::options::{otfcc_Options_optimizeTo, otfcc_newOptions};
use crate::vendor::json::{json_parse, json_value_free};





































#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfccbuild_json_otf(
    mut inlen: u32,
    mut injson: *const ::core::ffi::c_char,
    mut olevel: u8,
    mut for_webfont: bool,
) -> *mut Buffer {
    let mut options: *mut Options = otfcc_newOptions();
    (*options).logger = otfcc_newLogger(otfcc_newEmptyTarget());
    (*(*options).logger)
        .indent
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        b"otfccbuild\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_Options_optimizeTo(options, olevel);
    if for_webfont {
        (*options).ignore_glyph_order = true;
        (*options).force_cid = true;
    }
    let mut jsonRoot: *mut JsonValue = json_parse(injson, inlen as usize);
    if jsonRoot.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut parser: *mut IFontBuilder = otfcc_newJsonReader();
    let mut font: *mut Font = (*parser).read.expect("non-null function pointer")(
        jsonRoot as *mut ::core::ffi::c_void,
        0 as u32,
        options,
    );
    (*parser).free.expect("non-null function pointer")(parser as *mut IFontBuilder);
    json_value_free(jsonRoot);
    if font.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    otfcc_iFont.consolidate.expect("non-null function pointer")(font, options);
    let mut writer: *mut IFontSerializer = otfcc_newOTFWriter();
    let mut otf: *mut Buffer =
        (*writer).serialize.expect("non-null function pointer")(font, options)
            as *mut Buffer;
    (*writer).free.expect("non-null function pointer")(writer as *mut IFontSerializer);
    otfcc_iFont.free.expect("non-null function pointer")(font);
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
