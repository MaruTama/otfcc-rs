//! A code mapped to glyph 0, the missing glyph, is not mapped. otfcc read
//! such a mapping from a format 4 segment's idDelta but not from its glyph
//! ID array, so `"0": ".notdef"` could vanish between two dumps, and the
//! rebuilt `OS/2.usFirstCharIndex` (0) disagreed with the first mapping the
//! font reads back with. It is dropped both ways now, and when building.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn a_mapping_to_the_missing_glyph_is_dropped() {
    let build = scratch_dir("cmap-missing-glyph");
    let base = build.join("base.json");
    run_ok(
        "base dump",
        &otfccdump(),
        &[
            payload("iosevka-r.ttf").as_os_str(),
            OsStr::new("-o"),
            base.as_os_str(),
        ],
    );
    let mut font: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&base).unwrap()).unwrap();
    assert!(font["cmap"].get("0").is_none());
    let notdef = font["glyph_order"][0].clone();
    font["cmap"] = serde_json::json!({ "0": notdef, "1": "B", "2": "A", "65": "A" });
    let input = build.join("missing.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("missing.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("missing.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );
    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    // The font has no glyph names, so the glyphs come back renamed.
    let codes: Vec<&String> = dump["cmap"].as_object().unwrap().keys().collect();
    assert_eq!(codes, ["1", "2", "65"]);
    assert_eq!(dump["OS_2"]["usFirstCharIndex"], 1);
}
