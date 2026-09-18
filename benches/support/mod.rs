//! Shared helpers for the `criterion` benchmarks in `benches/`. Not a
//! benchmark itself -- each `benches/*.rs` file does `mod support;` and uses
//! these.
//!
//! Deliberately duplicates (rather than shares) `tests/support/mod.rs`'s
//! tiny `repo_root()`/`payload()` helpers: Cargo compiles `tests/` and
//! `benches/` as separate, unrelated sets of target crates, so there is no
//! straightforward way to reuse one directory's `mod support;` from the
//! other without fighting that separation -- and the duplicated part is a
//! handful of lines, cheaper to repeat than to plumb around.
//!
//! Every `unsafe` call in the crate's own c2rust-shaped dump/build pipeline
//! (`*mut Font`/`*mut SplineFontContainer`/`*mut BuiltValue`/
//! `*mut ParsedValue`/`*mut Buffer`) is confined to this file: each
//! `benches/*.rs` file calls only [`dump_to_json`]/[`build_to_otf`], which
//! return plain `Vec<u8>`, so the benchmark files themselves stay free of
//! raw-pointer noise.
#![allow(dead_code)]

use otfcc_rust::consolidate::otfcc_consolidate_font;
use otfcc_rust::font::caryll_font::otfcc_font_free;
use otfcc_rust::font::caryll_sfnt::{otfcc_delete_sfnt, otfcc_read_sfnt_from_reader};
use otfcc_rust::json_reader::read_json;
use otfcc_rust::json_writer::serialize_to_json;
use otfcc_rust::logger::{Logger, otfcc_new_empty_target};
use otfcc_rust::otf_reader::read_otf;
use otfcc_rust::otf_writer::serialize_to_otf;
use otfcc_rust::support::built_json::{JSON_SERIALIZE_MODE_PACKED, JsonSerializeOpts, json_serialize_ex};
use otfcc_rust::support::options::{Options, otfcc_delete_options, otfcc_new_options, otfcc_options_optimize_to};
use otfcc_rust::support::parsed_json::{json_parse, json_value_free};
use std::cell::RefCell;
use std::io::Cursor;
use std::path::{Path, PathBuf};

pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

pub fn payload(name: &str) -> PathBuf {
    repo_root().join("tests/payload").join(name)
}

pub fn payload_bytes(name: &str) -> Vec<u8> {
    let path = payload(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
}

/// A fresh, silent `Options` (an empty logger target -- no stderr output
/// competing with `criterion`'s own progress printing), at the default
/// optimization level.
pub fn quiet_options() -> *mut Options {
    unsafe {
        let options = otfcc_new_options();
        (*options).logger = RefCell::new(Logger::new(otfcc_new_empty_target()));
        options
    }
}

/// [`quiet_options`], with `-O2`'s exact effect applied
/// (`short_post`/`cff_do_subroutinize`/`merge_features`) -- matching what
/// `otfccbuild -O2` itself sets, not a hand-picked subset of it.
pub fn quiet_options_o2() -> *mut Options {
    unsafe {
        let options = quiet_options();
        otfcc_options_optimize_to(&mut *options, 2);
        options
    }
}

pub fn free_options(options: *mut Options) {
    unsafe { otfcc_delete_options(options) };
}

/// The dump pipeline (`otfccdump.rs`'s own steps, in-process): SFNT bytes
/// in, pretty-printed JSON bytes out.
pub fn dump_to_json(sfnt_bytes: &[u8], options: *const Options) -> Vec<u8> {
    unsafe {
        let sfnt = otfcc_read_sfnt_from_reader(&mut Cursor::new(sfnt_bytes));
        assert!(!sfnt.is_null(), "otfcc_read_sfnt_from_reader returned NULL");

        let font = read_otf(&*sfnt, 0, &*options);
        assert!(!font.is_null(), "read_otf returned NULL");
        otfcc_delete_sfnt(sfnt);

        otfcc_consolidate_font(&mut *font, &*options);

        let root = serialize_to_json(&mut *font, &*options);
        otfcc_font_free(font);

        let json_options = JsonSerializeOpts { mode: JSON_SERIALIZE_MODE_PACKED, opts: 0, indent_size: 4 };
        json_serialize_ex(&root, json_options)
    }
}

/// The build pipeline (`otfccbuild.rs`'s own steps, in-process): JSON bytes
/// in, built OTF/TTF bytes out.
pub fn build_to_otf(json_bytes: &[u8], options: *const Options) -> Vec<u8> {
    unsafe {
        let json_root = json_parse(json_bytes.as_ptr() as *const ::core::ffi::c_char, json_bytes.len());
        assert!(!json_root.is_null(), "json_parse returned NULL");

        let font = read_json(&*json_root, &*options);
        assert!(!font.is_null(), "read_json returned NULL");
        json_value_free(json_root);

        otfcc_consolidate_font(&mut *font, &*options);

        let otf = serialize_to_otf(&mut *font, &*options);
        otfcc_font_free(font);

        otf.data
    }
}
