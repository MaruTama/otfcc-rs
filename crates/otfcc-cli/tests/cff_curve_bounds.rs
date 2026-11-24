//! A CFF glyph's box is that of its curves, not of their control points.
//! otfcc took the control points, so `head`, `hhea` and the left side
//! bearings in `hmtx` came out wider than the outline: KRName's `head.xMax`
//! became 1028 where its curves end at 984.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn cff_glyph_boxes_follow_the_curves() {
    let build = scratch_dir("cff-curve-bounds");
    let base = build.join("base.json");
    run_ok(
        "base dump",
        &otfccdump(),
        &[
            payload("KRName-Regular.otf").as_os_str(),
            OsStr::new("-o"),
            base.as_os_str(),
        ],
    );
    let otf = build.join("bounds.otf");
    run_ok(
        "build",
        &otfccbuild(),
        &[base.as_os_str(), OsStr::new("-o"), otf.as_os_str()],
    );
    let dumped = build.join("bounds.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[otf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    // The values of the original font, whose curves these are.
    let head = &dump["head"];
    assert_eq!(
        [&head["xMin"], &head["yMin"], &head["xMax"], &head["yMax"]],
        [14, -120, 984, 880]
    );
    assert_eq!(dump["hhea"]["xMaxExtent"], 984);
    // The CFF FontBBox keeps the font's own left edge, 0 (which the dump
    // leaves out), outside every glyph: it only grows to take them in.
    assert!(dump["CFF_"].get("fontBBoxLeft").is_none_or(|v| v == 0.0));
    assert_eq!(dump["CFF_"]["fontBBoxRight"], 984.0);
}
