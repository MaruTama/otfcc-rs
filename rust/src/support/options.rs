#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{ILogger};


#[derive(Copy, Clone)]
#[repr(C)]
pub struct Options {
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
    pub logger: *mut ILogger,
}
pub unsafe extern "C" fn otfcc_new_options() -> *mut Options {
    let mut options: *mut Options = ::core::ptr::null_mut::<Options>();
    options = __caryll_allocate_clean(
        ::core::mem::size_of::<Options>() as usize,
        6 as ::core::ffi::c_ulong,
    ) as *mut Options;
    return options;
}
pub unsafe extern "C" fn otfcc_delete_options(mut options: *mut Options) {
    if !options.is_null() {
        free((*options).glyph_name_prefix as *mut ::core::ffi::c_void);
        (*options).glyph_name_prefix = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*options).logger.is_null() {
            (*(*options).logger)
                .dispose
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    free(options as *mut ::core::ffi::c_void);
    options = ::core::ptr::null_mut::<Options>();
}
pub unsafe extern "C" fn otfcc_options_optimize_to(
    mut options: *mut Options,
    mut level: u8,
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
