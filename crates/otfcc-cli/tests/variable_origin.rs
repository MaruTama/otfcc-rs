//! A glyph's horizontal origin (xMin minus the left side bearing) was
//! dumped only when it did not vary. In a variable font the phantom points
//! move it, so the key was left out, and the build put the left side
//! bearing back on xMin (#725). It is written whenever it varies or is not
//! 0, with its deltas, as the advance width is.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

fn dump(font: &std::path::Path, out: &std::path::Path) -> serde_json::Value {
    run_ok(
        "dump",
        &otfccdump(),
        &[font.as_os_str(), OsStr::new("-o"), out.as_os_str()],
    );
    serde_json::from_str(&std::fs::read_to_string(out).expect("the dump is written"))
        .expect("the dump is JSON")
}

#[test]
fn a_varying_origin_survives_a_dump_and_a_build() {
    let build = scratch_dir("variable-origin");
    // Glyph `A` starts at x = 50 with a left side bearing of 0, and its
    // phantom points vary.
    let first = dump(&payload("gvar-test.ttf"), &build.join("first.json"));
    let origin = &first["glyf"]["A"]["horizontalOrigin"];
    assert_eq!(
        origin[0], 50,
        "the origin at the default instance: {origin}"
    );
    assert!(origin[1].get("delta").is_some(), "its deltas: {origin}");

    let ttf = build.join("rebuilt.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[
            build.join("first.json").as_os_str(),
            OsStr::new("-o"),
            ttf.as_os_str(),
        ],
    );
    // The rebuilt font has no gvar, so the origin is a plain number now:
    // still 50, where it used to come back as 0 (the key was absent).
    let second = dump(&ttf, &build.join("second.json"));
    assert_eq!(second["glyf"]["A"]["horizontalOrigin"], 50);
}
