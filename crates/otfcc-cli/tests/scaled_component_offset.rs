//! A composite glyph's component can say its offset goes through its
//! transform (`SCALED_COMPONENT_OFFSET`). otfcc always wrote
//! `UNSCALED_COMPONENT_OFFSET`, which moves a scaled component; it now keeps
//! the flag, written in JSON as `"scaledOffset": true`.

// A test file: its helper functions may `unwrap()` like its tests do.
#![allow(clippy::unwrap_used)]

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

/// The first composite glyph of `font` and its first reference.
fn first_reference(font: &serde_json::Value) -> (String, serde_json::Value) {
    let glyphs = font["glyf"].as_object().unwrap();
    let (name, glyph) = glyphs
        .iter()
        .find(|(_, g)| g["references"].as_array().is_some_and(|r| !r.is_empty()))
        .expect("a composite glyph");
    (name.clone(), glyph["references"][0].clone())
}

#[test]
fn a_scaled_component_offset_survives_a_build_and_a_dump() {
    let build = scratch_dir("scaled-component-offset");
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
    let (name, mut reference) = first_reference(&font);
    reference["a"] = 0.5.into();
    reference["d"] = 0.5.into();
    reference["x"] = 200.into();
    reference["scaledOffset"] = true.into();
    font["glyf"][&name]["references"][0] = reference.clone();
    let input = build.join("scaled.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("scaled.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("scaled.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    let back = &dump["glyf"][&name]["references"][0];
    assert_eq!(back["scaledOffset"], true);
    assert_eq!(back["x"], 200);
    // The other components keep the default, which is not written.
    let others = dump["glyf"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(n, _)| **n != name)
        .flat_map(|(_, g)| g["references"].as_array().cloned().unwrap_or_default())
        .filter(|r| r.get("scaledOffset").is_some())
        .count();
    assert_eq!(others, 0);
}
