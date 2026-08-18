#![no_main]

// Fuzzes the JSON-to-font builder path through the actual public FFI entry
// point (otfccbuild_json_otf), which is the one boundary a real embedder
// calls -- unlike otf_parse.rs, this target needs no internal reflection at
// all. Covers ParsedValue's own parser (support/parsed_json.rs) plus
// json_reader.rs's font-shape assembly, otfcc_consolidate_font, and every
// table's build/serialize path, all from attacker-controlled bytes.
//
// json_reader.rs walks its input permissively -- missing or wrong-shaped
// keys fall back to defaults rather than failing (see rust/src/ffi/dll.rs's
// own test module for how thoroughly: `{}`, `[]`, `null`, `123`, `"x"` all
// build a valid font). That makes a crash here more likely to come from a
// numeric field taken to an extreme (glyph/lookup counts, table-building
// arithmetic) than from a rejected shape, which is exactly the kind of bug
// category unchecked-arithmetic UB fuzzing is good at finding.

use libfuzzer_sys::fuzz_target;
use otfcc_rust::ffi::dll::{otfcc_get_buf_data, otfcc_get_buf_len, otfccbuild_free_otfbuf, otfccbuild_json_otf};

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }
    unsafe {
        let buf = otfccbuild_json_otf(
            data.len() as u32,
            data.as_ptr() as *const ::core::ffi::c_char,
            0,
            false,
        );
        if buf.is_null() {
            return;
        }
        // Touch the returned bytes so a corrupt Buffer (wrong size vs.
        // actual allocation) shows up as an ASan read, not silently.
        let len = otfcc_get_buf_len(buf);
        let data_ptr = otfcc_get_buf_data(buf);
        if len > 0 && !data_ptr.is_null() {
            std::hint::black_box(std::slice::from_raw_parts(data_ptr, len));
        }
        otfccbuild_free_otfbuf(buf);
    }
});
