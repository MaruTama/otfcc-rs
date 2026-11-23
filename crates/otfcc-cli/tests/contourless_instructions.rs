//! A simple glyph can have no contours and still have instructions (the
//! specification lets `numberOfContours` be 0). otfcc read such a glyph as
//! a composite and wrote a glyph without an outline as zero bytes, so its
//! instructions were lost both ways. They are kept now; a glyph with
//! nothing at all is still written with zero length.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn instructions_of_a_glyph_without_contours_survive() {
    let build = scratch_dir("contourless-instructions");
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
    let instructions = serde_json::json!(["PUSHB_1", 0, "SRP0"]);
    font["glyph_order"]
        .as_array_mut()
        .unwrap()
        .extend(["hinted.empty".into(), "plain.empty".into()]);
    font["glyf"]["hinted.empty"] =
        serde_json::json!({ "advanceWidth": 500, "instructions": instructions });
    font["glyf"]["plain.empty"] = serde_json::json!({ "advanceWidth": 500 });
    let input = build.join("empty.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("empty.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("empty.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    // The font has no glyph names, so the two glyphs come back renamed:
    // find them by their place in the glyph order.
    let order = dump["glyph_order"].as_array().unwrap();
    let glyph = |back: usize| &dump["glyf"][order[order.len() - back].as_str().unwrap()];
    let hinted = glyph(2);
    assert_eq!(hinted["instructions"], instructions);
    assert!(hinted.get("contours").is_none() && hinted.get("references").is_none());
    assert!(glyph(1).get("instructions").is_none());
}
