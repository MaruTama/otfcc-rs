//! The font's bounding box in `head` and the side bearing minimums and
//! extent in `hhea` are over the glyphs with an outline. otfcc counted an
//! empty glyph (a space) as the box (0, 0, 0, 0), so `xMin` and
//! `minLeftSideBearing` could never be above 0.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn empty_glyphs_do_not_count_in_the_font_bounds() {
    let build = scratch_dir("empty-glyph-bounds");
    let base = build.join("base.json");
    // Every outlined glyph of this font starts at x = 50 or later, and it
    // has an empty `.notdef` and `space`.
    run_ok(
        "base dump",
        &otfccdump(),
        &[
            payload("gvar-test.ttf").as_os_str(),
            OsStr::new("-o"),
            base.as_os_str(),
        ],
    );
    // The font keeps a left side bearing of 0 for a glyph that starts at
    // 50 (its `horizontalOrigin`). Without it the bearing is the glyph's
    // xMin, which is what this test measures the minimum of.
    let mut font: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&base).unwrap()).unwrap();
    for glyph in font["glyf"].as_object_mut().unwrap().values_mut() {
        glyph.as_object_mut().unwrap().remove("horizontalOrigin");
    }
    let input = build.join("bounds.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("bounds.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("bounds.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    assert_eq!(dump["head"]["xMin"], 50);
    assert_eq!(dump["hhea"]["minLeftSideBearing"], 50);
}
