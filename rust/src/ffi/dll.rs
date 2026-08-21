#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md


use crate::support::buffer::{Buffer};
use crate::support::options::{Options};


use crate::support::parsed_json::{ParsedValue};
use crate::font::caryll_font::{Font};
use crate::font::caryll_font::{otfcc_font_free};
use crate::consolidate::{otfcc_consolidate_font};
use crate::json_reader::{read_json};
use crate::logger::{otfcc_new_empty_target, logger_indent, Logger};
use std::cell::RefCell;
use crate::otf_writer::{serialize_to_otf};
use crate::support::buffer::{buffree};
use crate::support::options::{otfcc_options_optimize_to, otfcc_new_options, otfcc_delete_options};
use crate::support::parsed_json::{json_parse, json_value_free};





































#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfccbuild_json_otf(
    mut inlen: u32,
    mut injson: *const ::core::ffi::c_char,
    mut olevel: u8,
    mut for_webfont: bool,
) -> *mut Buffer {
    let mut options: *mut Options = otfcc_new_options();
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
    let mut json_root: *mut ParsedValue = json_parse(injson, inlen as usize);
    if json_root.is_null() {
        otfcc_delete_options(options);
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut font: *mut Font = read_json(json_root as *mut ::core::ffi::c_void, 0 as u32, &*options);
    json_value_free(json_root);
    if font.is_null() {
        otfcc_delete_options(options);
        return ::core::ptr::null_mut::<Buffer>();
    }
    otfcc_consolidate_font(font, &*options);
    let mut otf: *mut Buffer = serialize_to_otf(font, &*options) as *mut Buffer;
    otfcc_font_free(font);
    otfcc_delete_options(options);
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
    // Constructs a real `Font` (`otfcc_font_create` -> `malloc` +
    // `memset`-zero, `font/caryll_font.rs:init_font`), and `read_json`'s
    // field-by-field `(*font).x = ...` writes over that zeroed memory are
    // the same "drop-before-write on an invalid niche-optimized bit
    // pattern" UB class documented on `otfcc_new_options`
    // (`support/options.rs:otfcc_new_options`) -- except `Font` has this on
    // essentially every pointer-containing field, not just one, discovered
    // while wiring up `cargo miri test` in rust/fuzz's sibling
    // infrastructure PR. Fixing `Font`'s construction path is real,
    // substantial, crate-wide work (every `otfcc_parse_*`/`otfcc_read_*`
    // call site that populates a freshly-created `Font` or table struct),
    // out of scope for that PR; tracked in rust/README.md's "Next steps".
    #[cfg_attr(miri, ignore = "Font construction has pre-existing calloc+assign UB, see rust/README.md Next steps")]
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
    // builds a real Font on every iteration.
    #[cfg_attr(miri, ignore = "Font construction has pre-existing calloc+assign UB, see rust/README.md Next steps")]
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

