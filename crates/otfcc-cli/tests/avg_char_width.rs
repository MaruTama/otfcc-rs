//! `OS/2.xAvgCharWidth` is the average width of the glyphs whose width is
//! not 0. otfcc divided the sum of the widths by the count of all glyphs,
//! so a font with marks of width 0 got too small a value (Noto Sans
//! Devanagari: 431 instead of 531), and the result was cut, not rounded.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn the_average_width_leaves_out_glyphs_of_width_0() {
    let build = scratch_dir("avg-char-width");
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
    let font: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&base).unwrap()).unwrap();
    let widths: Vec<f64> = font["glyf"]
        .as_object()
        .unwrap()
        .values()
        .map(|g| g["advanceWidth"].as_f64().unwrap_or(0.0))
        .collect();
    let nonzero: Vec<f64> = widths.iter().copied().filter(|&w| w > 0.0).collect();
    assert!(
        nonzero.len() < widths.len(),
        "the font has glyphs of width 0"
    );
    let expected = (nonzero.iter().sum::<f64>() / nonzero.len() as f64).round();

    let ttf = build.join("avg.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[base.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("avg.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );
    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    assert_eq!(dump["OS_2"]["xAvgCharWidth"].as_f64(), Some(expected));
}
