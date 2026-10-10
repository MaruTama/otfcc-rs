
use otfcc_binary::Buffer;
use crate::support::options::Options;

use crate::consolidate::consolidate_font;
use crate::json_reader::read_json;
use crate::otf_writer::serialize_to_otf;
use crate::support::options::options_optimize_to;
use otfcc_json::parse_json;

/// # Safety
/// `injson` must be non-null and point to at least `inlen` readable bytes
/// (they need not be NUL-terminated -- `inlen` alone bounds the slice this
/// builds from them) for the duration of this call; the memory it points
/// to must not be mutated concurrently. The returned pointer, when
/// non-null, is an owned `Buffer` the caller must eventually pass to
/// [`otfccbuild_free_otfbuf`] exactly once to avoid leaking it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfccbuild_json_otf(
    inlen: u32,
    injson: *const ::core::ffi::c_char,
    olevel: u8,
    for_webfont: bool,
) -> *mut Buffer {
    let mut options: Box<Options> = Box::default();
    let _root_scope = crate::logger::indent("otfccbuild");
    options_optimize_to(&mut options, olevel);
    if for_webfont {
        options.ignore_glyph_order = true;
        options.force_cid = true;
    }
    // The one place the raw `(pointer, length)` pair from the C caller is
    // turned into a slice; everything from here on is safe.
    // SAFETY: the caller guarantees `injson` points to `inlen` readable bytes.
    let input = unsafe { ::core::slice::from_raw_parts(injson as *const u8, inlen as usize) };
    let Some(mut json_root) = parse_json(input)
    else {
        return ::core::ptr::null_mut::<Buffer>();
    };
    let Some(mut font) = read_json(&mut json_root, &options) else {
        return ::core::ptr::null_mut::<Buffer>();
    };
    drop(json_root);
    consolidate_font(&mut font, &options);
    // This is the one genuine `extern "C"` boundary in the crate, so it is
    // also the one place that still needs to hand a `Buffer` back as a raw
    // pointer -- `serialize_to_otf` returns the `Buffer` itself now.
    let otf: *mut Buffer = Box::into_raw(Box::new(serialize_to_otf(&mut font, &options)));
    drop(font);
    return otf;
}
/// # Safety
/// `buf` must be non-null and point to a live `Buffer` obtained from
/// [`otfccbuild_json_otf`] and not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_get_buf_len(buf: *mut Buffer) -> usize {
    // SAFETY: the caller guarantees `buf` is a live `Buffer` from this library.
    return unsafe { (*buf).data.len() };
}
/// # Safety
/// Same contract as [`otfcc_get_buf_len`]: `buf` must be non-null and
/// point to a live, not-yet-freed `Buffer`. The returned pointer aliases
/// `buf`'s own data and is only valid as long as `buf` itself is (and
/// only until the next call that could reallocate its data), so it must
/// not be used after `buf` is freed via [`otfccbuild_free_otfbuf`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_get_buf_data(buf: *mut Buffer) -> *mut u8 {
    // SAFETY: as for `otfcc_get_buf_len`.
    return unsafe { (*buf).data.as_mut_ptr() };
}
/// # Safety
/// `buf` must either be null or point to a live `Buffer` obtained from
/// [`otfccbuild_json_otf`] and not already freed. Calling this
/// twice on the same pointer, or using `buf` (or any pointer previously
/// returned by [`otfcc_get_buf_data`] for it) afterward, is a
/// use-after-free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfccbuild_free_otfbuf(buf: *mut Buffer) {
    if !buf.is_null() {
        // SAFETY: a non-null `buf` came from `Box::into_raw` in
        // `otfccbuild_json_otf` and has not been freed.
        drop(unsafe { Box::from_raw(buf) });
    }
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
    // bug for anyone reading this file. See fuzz/README.md for
    // finding this class of bug directly, under a leak sanitizer.
    //
    // Only the `json_parse` failure path (invalid JSON syntax) is
    // reachable through this entry point today: `read_json` walks its
    // input permissively (missing/wrong-shaped keys fall back to defaults
    // rather than failing, the same `json_obj_getnum`-style fallback
    // documented in RUST_MIGRATION.md), so every one of `{}`, `[]`, `null`,
    // `123`, `"x"` -- confirmed by hand while writing this test -- builds
    // a valid, if nearly empty, font instead of returning null. That
    // makes `font.is_null()` at the second early return dead in practice,
    // not just untested; the fix still needed to cover it; a future
    // change that makes `read_json` fail for real should add a case here.
    unsafe fn build(json: &[u8]) -> *mut Buffer {
        // SAFETY: `json` is a live slice of `json.len()` bytes.
        unsafe {
            otfccbuild_json_otf(
                json.len() as u32,
                json.as_ptr() as *const ::core::ffi::c_char,
                0,
                false,
            )
        }
    }

    #[test]
    fn invalid_json_returns_null_without_crashing() {
        unsafe {
            let buf = build(b"not json");
            assert!(buf.is_null());
        }
    }

    #[test]
    // `read_json` builds a `Font` from `{}`, then serializes it to OTF --
    // run under Miri too, covering `Font` construction and
    // `font/sfnt_builder.rs`'s checksum computation.
    fn minimal_json_builds_and_frees_cleanly() {
        unsafe {
            // Exercises the success path -- `read_json` on `{}` yields a fully-defaulted,
            // zero-glyph font (see the module doc comment above), which
            // consolidate_font/serialize_to_otf still happily turn
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
        // this function. 100 reps under native `cargo test` (fast); under
        // Miri (interpreted, ~0.9s/rep, this crate's second-slowest test
        // at the full count) a state-not-reset-between-calls bug would
        // reproduce on the 2nd or 3rd call, not specifically need 100 --
        // each iteration is the same deterministic operation, not a fuzzed
        // input, so more reps past the first few don't add real UB-
        // detection confidence, just wall-clock cost.
        let reps = if cfg!(miri) { 5 } else { 100 };
        unsafe {
            for _ in 0..reps {
                assert!(build(b"not json").is_null());
                let buf = build(b"{}");
                assert!(!buf.is_null());
                otfccbuild_free_otfbuf(buf);
            }
        }
    }
}
