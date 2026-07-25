extern "C" {
    fn json_parse(json: *const ::core::ffi::c_char, length: usize) -> *mut json_value;
    fn json_value_free(_: *mut json_value);
    fn otfcc_newLogger(target: *mut otfcc_ILoggerTarget) -> *mut otfcc_ILogger;
    fn otfcc_newEmptyTarget() -> *mut otfcc_ILoggerTarget;
    fn otfcc_newOptions() -> *mut otfcc_Options;
    fn otfcc_Options_optimizeTo(options: *mut otfcc_Options, level: u8);
    fn buffree(buf: *mut caryll_Buffer);
    static otfcc_iFont: __caryll_elementinterface_otfcc_Font;
    fn otfcc_newJsonReader() -> *mut otfcc_IFontBuilder;
    fn otfcc_newOTFWriter() -> *mut otfcc_IFontSerializer;
}


use crate::logger::{otfcc_ILogger, otfcc_ILoggerTarget};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};


use crate::vendor::json::{json_value};
use crate::font::caryll_font::{__caryll_elementinterface_otfcc_Font, otfcc_Font, otfcc_IFontBuilder, otfcc_IFontSerializer};





































#[no_mangle]
pub unsafe extern "C" fn otfccbuild_json_otf(
    mut inlen: u32,
    mut injson: *const ::core::ffi::c_char,
    mut olevel: u8,
    mut for_webfont: bool,
) -> *mut caryll_Buffer {
    let mut options: *mut otfcc_Options = otfcc_newOptions();
    (*options).logger = otfcc_newLogger(otfcc_newEmptyTarget());
    (*(*options).logger)
        .indent
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        b"otfccbuild\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_Options_optimizeTo(options, olevel);
    if for_webfont {
        (*options).ignore_glyph_order = true;
        (*options).force_cid = true;
    }
    let mut jsonRoot: *mut json_value = json_parse(injson, inlen as usize);
    if jsonRoot.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut parser: *mut otfcc_IFontBuilder = otfcc_newJsonReader();
    let mut font: *mut otfcc_Font = (*parser).read.expect("non-null function pointer")(
        jsonRoot as *mut ::core::ffi::c_void,
        0 as u32,
        options,
    );
    (*parser).free.expect("non-null function pointer")(parser as *mut otfcc_IFontBuilder);
    json_value_free(jsonRoot);
    if font.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    otfcc_iFont.consolidate.expect("non-null function pointer")(font, options);
    let mut writer: *mut otfcc_IFontSerializer = otfcc_newOTFWriter();
    let mut otf: *mut caryll_Buffer =
        (*writer).serialize.expect("non-null function pointer")(font, options)
            as *mut caryll_Buffer;
    (*writer).free.expect("non-null function pointer")(writer as *mut otfcc_IFontSerializer);
    otfcc_iFont.free.expect("non-null function pointer")(font);
    return otf;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_get_buf_len(mut buf: *mut caryll_Buffer) -> usize {
    return (*buf).size;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_get_buf_data(mut buf: *mut caryll_Buffer) -> *mut u8 {
    return (*buf).data;
}
#[no_mangle]
pub unsafe extern "C" fn otfccbuild_free_otfbuf(mut buf: *mut caryll_Buffer) {
    buffree(buf);
}
