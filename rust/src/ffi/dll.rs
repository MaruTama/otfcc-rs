#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::buffer::Buffer;
use crate::support::options::Options;

use crate::consolidate::otfcc_consolidate_font;
use crate::font::caryll_font::Font;
use crate::font::caryll_font::otfcc_font_free;
use crate::json_reader::read_json;
use crate::logger::{Logger, logger_indent, otfcc_new_empty_target};
use crate::otf_writer::serialize_to_otf;
use crate::support::buffer::buffree;
use crate::support::options::{otfcc_delete_options, otfcc_new_options, otfcc_options_optimize_to};
use crate::support::parsed_json::ParsedValue;
use crate::support::parsed_json::{json_parse, json_value_free};
use std::cell::RefCell;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfccbuild_json_otf(
    inlen: u32,
    injson: *const ::core::ffi::c_char,
    olevel: u8,
    for_webfont: bool,
) -> *mut Buffer {
    let options: *mut Options = otfcc_new_options();
    (*options).logger = RefCell::new(Logger::new(otfcc_new_empty_target()));
    logger_indent(
        &mut *(*options).logger.borrow_mut(),
        b"otfccbuild\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_options_optimize_to(options, olevel);
    if for_webfont {
        (*options).ignore_glyph_order = true;
        (*options).force_cid = true;
    }
    let json_root: *mut ParsedValue = json_parse(injson, inlen as usize);
    if json_root.is_null() {
        otfcc_delete_options(options);
        return ::core::ptr::null_mut::<Buffer>();
    }
    let font: *mut Font = read_json(json_root as *mut ::core::ffi::c_void, 0 as u32, &*options);
    json_value_free(json_root);
    if font.is_null() {
        otfcc_delete_options(options);
        return ::core::ptr::null_mut::<Buffer>();
    }
    otfcc_consolidate_font(font, &*options);
    let otf: *mut Buffer = serialize_to_otf(font, &*options) as *mut Buffer;
    otfcc_font_free(font);
    otfcc_delete_options(options);
    return otf;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_get_buf_len(buf: *mut Buffer) -> usize {
    return (*buf).data.len();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_get_buf_data(buf: *mut Buffer) -> *mut u8 {
    return (*buf).data.as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfccbuild_free_otfbuf(buf: *mut Buffer) {
    buffree(buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    // otfccbuild_json_otf used to leak `options` (and the Logger it owns)
    // on every one of its three return paths -- discovered while writing
    // the JSON-fuzz target in fuzz/fuzz_targets/json_build.rs, which would
    // otherwise have reported this exact leak on its very first input and
    // stayed stuck on it. cargo test alone can't detect a leak (no
    // instrumentation) -- these exist to catch a crash or use-after-free
    // regression in the cleanup path this fix touches, and to document the
    // bug for anyone reading this file. See rust/fuzz/README.md for
    // finding this class of bug directly, under a leak sanitizer.
    //
    // Only the `json_parse` failure path (invalid JSON syntax) is
    // reachable through this entry point today: `read_json` walks its
    // input permissively (missing/wrong-shaped keys fall back to defaults
    // rather than failing, the same `json_obj_getnum`-style fallback
    // documented in rust/README.md), so every one of `{}`, `[]`, `null`,
    // `123`, `"x"` -- confirmed by hand while writing this test -- builds
    // a valid, if nearly empty, font instead of returning null. That
    // makes `font.is_null()` at the second early return dead in practice,
    // not just untested; the fix still needed to cover it; a future
    // change that makes `read_json` fail for real should add a case here.
    unsafe fn build(json: &[u8]) -> *mut Buffer {
        otfccbuild_json_otf(
            json.len() as u32,
            json.as_ptr() as *const ::core::ffi::c_char,
            0,
            false,
        )
    }

    #[test]
    fn invalid_json_returns_null_without_crashing() {
        unsafe {
            let buf = build(b"not json");
            assert!(buf.is_null());
        }
    }

    #[test]
    // Constructs a real `Font` via `otfcc_font_create`, then `read_json`
    // populates it field-by-field, then serializes to OTF -- which used to
    // hit two separate, now-fixed UBs under miri: `Font`-construction
    // (Stage 7-2-d's `Font` Box化) and `font/caryll_sfnt_builder.rs`'s
    // checksum computation reading a `Vec<u8>` through a misaligned `*mut
    // u32` (fixed by reading big-endian bytes via `chunks_exact`/
    // `from_be_bytes` instead of a pointer cast). No longer miri-ignored.
    fn minimal_json_builds_and_frees_cleanly() {
        unsafe {
            // Exercises the success-path `otfcc_delete_options` call this
            // fix adds -- `read_json` on `{}` yields a fully-defaulted,
            // zero-glyph font (see the module doc comment above), which
            // otfcc_consolidate_font/serialize_to_otf still happily turn
            // into a (tiny but valid) OTF Buffer.
            let buf = build(b"{}");
            assert!(!buf.is_null());
            assert!(otfcc_get_buf_len(buf) > 0);
            assert!(!otfcc_get_buf_data(buf).is_null());
            otfccbuild_free_otfbuf(buf);
        }
    }

    #[test]
    // Same reason as minimal_json_builds_and_frees_cleanly above: this also
    // builds a real Font and serializes it on every iteration. Both UBs it
    // used to hit under miri are fixed; no longer miri-ignored.
    fn repeated_calls_do_not_crash() {
        // Not a leak check (needs a sanitizer for that -- see fuzz/), just
        // confirming the cleanup paths added by the fix above are safe to
        // hit many times in one process, the way both otfccdump/otfccbuild
        // (many payloads per process in run-cycles.sh) and the fuzz
        // targets (thousands of iterations per process) actually call
        // this function.
        unsafe {
            for _ in 0..100 {
                assert!(build(b"not json").is_null());
                let buf = build(b"{}");
                assert!(!buf.is_null());
                otfccbuild_free_otfbuf(buf);
            }
        }
    }
}
