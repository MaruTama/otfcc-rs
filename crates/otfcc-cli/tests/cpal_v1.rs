//! CPAL version 1 adds palette types, palette labels and entry labels.
//! otfcc read their offsets 4 bytes too far on, so each array came from the
//! next one's offset, and it read a palette's label back from `"type"`.
//! Both are fixed: a v1 CPAL comes back from a build and a dump as it was.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn cpal_v1_arrays_survive_a_build_and_a_dump() {
    let build = scratch_dir("cpal-v1");
    let base = build.join("base.json");
    run_ok(
        "base dump",
        &otfccdump(),
        &[
            payload("BungeeColor-Regular_colr_Windows.ttf").as_os_str(),
            OsStr::new("-o"),
            base.as_os_str(),
        ],
    );
    let mut font: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&base).unwrap()).unwrap();
    // Two palettes of the font's own colors, with a type and a label each,
    // and a label for every entry.
    let colors: Vec<serde_json::Value> = font["CPAL"]["palettes"][0]["colors"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(j, c)| {
            let mut c = c.clone();
            c["label"] = (300 + j as u16).into();
            c
        })
        .collect();
    let cpal = serde_json::json!({
        "version": 1,
        "palettes": [
            { "type": 1, "label": 256, "colors": colors },
            { "type": 2, "label": 257, "colors": colors },
        ],
    });
    font["CPAL"] = cpal.clone();
    let input = build.join("v1.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("v1.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("v1.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    assert_eq!(dump["CPAL"], cpal);
}
