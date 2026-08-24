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
// file by path via otfcc_read_sfnt and process-exits on failure, so its
// error paths never have to worry about freeing what came before -- this
// harness runs thousands of iterations in one process, so it can't take
// that shortcut: every path here (bad file, bad ttcindex, bad font, good
// font) explicitly frees sfnt/font/options before returning, unlike the CLI.
//
// otfcc_read_sfnt itself is path-based (Stage 7-4 moved it off a `FILE*`
// onto `std::fs`/`std::io`), so this uses its `Read + Seek`-generic sibling
// `otfcc_read_sfnt_from_reader` with a `Cursor` over the fuzzer-provided
// bytes instead -- the same "wrap this byte slice, no real file on disk"
// shape `fmemopen` gave the old `FILE*`-based reader, without needing a
// real temp file written to disk on every one of this target's
// thousands-per-process iterations.

use libfuzzer_sys::fuzz_target;
use otfcc_rust::font::caryll_font::otfcc_font_free;
use otfcc_rust::font::caryll_sfnt::{otfcc_delete_sfnt, otfcc_read_sfnt_from_reader};
use otfcc_rust::logger::{Logger, otfcc_new_empty_target};
use otfcc_rust::otf_reader::read_otf;
use otfcc_rust::support::options::{otfcc_delete_options, otfcc_new_options};
use std::cell::RefCell;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    unsafe {
        let sfnt = otfcc_read_sfnt_from_reader(&mut Cursor::new(data));

        if sfnt.is_null() || (*sfnt).count == 0 {
            if !sfnt.is_null() {
                otfcc_delete_sfnt(sfnt);
            }
            return;
        }

        let options = otfcc_new_options();
        (*options).logger = RefCell::new(Logger::new(otfcc_new_empty_target()));

        // Subfont index 0 always exists once `count > 0` -- fuzzing which
        // TTC subfont gets selected would mostly re-exercise the same
        // per-table readers this target already drives, at the cost of a
        // second dimension in the corpus.
        let font = read_otf(sfnt as *mut libc::c_void, 0, &*options);
        otfcc_delete_sfnt(sfnt);

        if !font.is_null() {
            otfcc_font_free(font);
        }
        otfcc_delete_options(options);
    }
});
