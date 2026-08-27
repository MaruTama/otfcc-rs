#![no_main]

// Fuzzes the full otfccdump pipeline: otfcc_read_sfnt -> read_otf ->
// otfcc_consolidate_font -> serialize_to_json, in that order -- exactly
// otfccdump.rs's own Read Font / Consolidate / Dump sequence (src/bin/
// otfccdump.rs).
//
// `otf_parse` (this crate's other target) stops after `read_otf`, so it
// never exercises consolidation or JSON serialization at all. That gap is
// not hypothetical: a manual (non-fuzz-harness) investigation found a real
// heap-use-after-free that `otf_parse` could never have found, because the
// dangling read only happens during `otfcc_dump_otl` (called from
// `serialize_to_json`), well past where `otf_parse` already returned. The
// bug -- `LanguageSystem.required_feature`, a lone borrowed `*const
// Feature`, was never revisited when `consolidate_otl_table` dropped the
// `Feature` it pointed at -- is fixed in `consolidate.rs`; this target
// exists so the *next* bug in this same family (consolidate/dump, not
// parse) has a chance of turning up here instead of needing another manual
// hunt.
//
// Same per-iteration cleanup discipline as `otf_parse`: this harness runs
// thousands of iterations in one process, so every path (bad file, bad
// font, consolidate/dump failure, success) explicitly frees sfnt/font/
// root/options before returning, unlike the CLI's process-exit-on-failure
// shortcut.

use libfuzzer_sys::fuzz_target;
use otfcc_rust::consolidate::otfcc_consolidate_font;
use otfcc_rust::font::caryll_font::otfcc_font_free;
use otfcc_rust::font::caryll_sfnt::{otfcc_delete_sfnt, otfcc_read_sfnt_from_reader};
use otfcc_rust::json_writer::serialize_to_json;
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

        // Subfont index 0 always exists once `count > 0` -- see otf_parse's
        // own comment on why this target does not also fuzz the TTC index.
        let font = read_otf(sfnt as *mut libc::c_void, 0, &*options);
        otfcc_delete_sfnt(sfnt);

        if !font.is_null() {
            otfcc_consolidate_font(font, &*options);
            let root = serialize_to_json(font, &*options);
            if !root.is_null() {
                drop(Box::from_raw(root));
            }
            otfcc_font_free(font);
        }
        otfcc_delete_options(options);
    }
});
