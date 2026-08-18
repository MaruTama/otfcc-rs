#![no_main]

// Fuzzes the sfnt/OTF binary-parsing path: otfcc_read_sfnt (table directory
// + table data) followed by read_otf (per-table readers -- head, cmap, glyf,
// otl, CFF, ...). This is the path rust/README.md's Phase 5 plan calls out
// as having real, C-inherited memory-safety bugs on malformed input (see
// "Stage 7-1" -- unvalidated offsets in cmap.rs/name.rs/post.rs, glyph
// readers with no length parameter at all, a CFF INDEX overflow that
// computes a ~4GB memcpy) and as never having been fuzzed before. This
// target is the regression net for that work: every crash this finds and
// every one Stage 7-1 fixes should turn into a corpus entry that stays
// crash-free forever after.
//
// The otfccdump binary's equivalent flow (src/bin/otfccdump.rs) reads a real
// file with fopen/otfcc_read_sfnt and process-exits on failure, so its error
// paths never have to worry about freeing what came before -- this harness
// runs thousands of iterations in one process, so it can't take that
// shortcut: every path here (bad file, bad ttcindex, bad font, good font)
// explicitly frees sfnt/font/options before returning, unlike the CLI.
//
// fmemopen wraps the fuzzer-provided byte slice as a FILE* without a real
// file on disk, since otfcc_read_sfnt's signature is FILE*-shaped (inherited
// from the C fread/fseek-based reader) rather than slice-based -- the same
// FILE*-at-the-boundary shape Stage 7-4 of the plan will eventually replace
// with std::fs/std::io.

use libfuzzer_sys::fuzz_target;
use otfcc_rust::font::caryll_font::otfcc_font_free;
use otfcc_rust::font::caryll_sfnt::{otfcc_delete_sfnt, otfcc_read_sfnt};
use otfcc_rust::logger::{otfcc_new_empty_target, otfcc_new_logger};
use otfcc_rust::otf_reader::read_otf;
use otfcc_rust::support::options::{otfcc_delete_options, otfcc_new_options};

fuzz_target!(|data: &[u8]| {
    // fmemopen requires a non-empty buffer on some libc implementations
    // (glibc accepts size 0, macOS's libc does not); nothing interesting is
    // reachable from an empty file either way.
    if data.is_empty() {
        return;
    }

    unsafe {
        // fmemopen does not copy: it reads/writes through this exact
        // buffer for the FILE*'s lifetime, so `buf` must outlive `file`.
        let mut buf = data.to_vec();
        let mode = c"rb";
        let file = libc::fmemopen(
            buf.as_mut_ptr() as *mut libc::c_void,
            buf.len(),
            mode.as_ptr(),
        );
        if file.is_null() {
            return;
        }

        let sfnt = otfcc_read_sfnt(file as *mut otfcc_rust::support::stdio::FILE);
        // otfcc_read_sfnt does not take ownership of `file` on any path
        // (mirrors src/bin/otfccdump.rs, which never fcloses it either) --
        // only the fmemopen'd FILE* itself needs closing here.
        libc::fclose(file);

        if sfnt.is_null() || (*sfnt).count == 0 {
            if !sfnt.is_null() {
                otfcc_delete_sfnt(sfnt);
            }
            return;
        }

        let options = otfcc_new_options();
        (*options).logger = otfcc_new_logger(otfcc_new_empty_target());

        // Subfont index 0 always exists once `count > 0` -- fuzzing which
        // TTC subfont gets selected would mostly re-exercise the same
        // per-table readers this target already drives, at the cost of a
        // second dimension in the corpus.
        let font = read_otf(sfnt as *mut libc::c_void, 0, options);
        otfcc_delete_sfnt(sfnt);

        if !font.is_null() {
            otfcc_font_free(font);
        }
        otfcc_delete_options(options);
    }
});
