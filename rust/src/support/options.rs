extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}

use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILoggerTarget {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()>,
}
pub type otfcc_LoggerType = ::core::ffi::c_uint;
pub const log_type_progress: otfcc_LoggerType = 3;
pub const log_type_info: otfcc_LoggerType = 2;
pub const log_type_warning: otfcc_LoggerType = 1;
pub const log_type_error: otfcc_LoggerType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILogger {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub indent: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub indentSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub start: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub startSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub log: Option<
        unsafe extern "C" fn(
            *mut otfcc_ILogger,
            uint8_t,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t) -> ()>,
    pub getTarget: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Options {
    pub debug_wait_on_start: bool,
    pub ignore_glyph_order: bool,
    pub ignore_hints: bool,
    pub has_vertical_metrics: bool,
    pub export_fdselect: bool,
    pub keep_average_char_width: bool,
    pub keep_unicode_ranges: bool,
    pub short_post: bool,
    pub dummy_DSIG: bool,
    pub keep_modified_time: bool,
    pub instr_as_bytes: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub cff_short_vmtx: bool,
    pub merge_lookups: bool,
    pub merge_features: bool,
    pub force_cid: bool,
    pub cff_rollCharString: bool,
    pub cff_doSubroutinize: bool,
    pub stub_cmap4: bool,
    pub decimal_cmap: bool,
    pub name_glyphs_by_hash: bool,
    pub name_glyphs_by_gid: bool,
    pub glyph_name_prefix: *mut ::core::ffi::c_char,
    pub logger: *mut otfcc_ILogger,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn otfcc_newOptions() -> *mut otfcc_Options {
    let mut options: *mut otfcc_Options = ::core::ptr::null_mut::<otfcc_Options>();
    options = __caryll_allocate_clean(
        ::core::mem::size_of::<otfcc_Options>() as size_t,
        6 as ::core::ffi::c_ulong,
    ) as *mut otfcc_Options;
    return options;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_deleteOptions(mut options: *mut otfcc_Options) {
    if !options.is_null() {
        free((*options).glyph_name_prefix as *mut ::core::ffi::c_void);
        (*options).glyph_name_prefix = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*options).logger.is_null() {
            (*(*options).logger)
                .dispose
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    free(options as *mut ::core::ffi::c_void);
    options = ::core::ptr::null_mut::<otfcc_Options>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_Options_optimizeTo(
    mut options: *mut otfcc_Options,
    mut level: uint8_t,
) {
    (*options).cff_rollCharString = false;
    (*options).short_post = false;
    (*options).ignore_glyph_order = false;
    (*options).cff_short_vmtx = false;
    (*options).merge_features = false;
    (*options).force_cid = false;
    (*options).cff_doSubroutinize = false;
    if level as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
        (*options).cff_rollCharString = true;
        (*options).cff_short_vmtx = true;
    }
    if level as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
        (*options).short_post = true;
        (*options).cff_doSubroutinize = true;
        (*options).merge_features = true;
    }
    if level as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
        (*options).ignore_glyph_order = true;
        (*options).force_cid = true;
    }
}
