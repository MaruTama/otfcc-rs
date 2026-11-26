//! Fields of the metrics tables that came back changed from a build and a
//! dump (#667): `OS/2` version 5's upper optical size was read into the
//! lower one, `post.italicAngle` lost its fraction, `post.maxMemType42` was
//! replaced by the glyph count, and an `LTSH` whose values are all 0 or 1
//! was dropped.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn metrics_fields_survive_a_build_and_a_dump() {
    let build = scratch_dir("metrics-fields");
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
    font["OS_2"]["version"] = 5.into();
    font["OS_2"]["usLowerOpticalPointSize"] = 80.into();
    font["OS_2"]["usUpperOpticalPointSize"] = 120.into();
    font["post"]["italicAngle"] = (-12.5).into();
    font["post"]["maxMemType42"] = 123.into();
    // Every glyph linear at every size: an LTSH of ones.
    for glyph in font["glyf"].as_object_mut().unwrap().values_mut() {
        glyph["LTSH_yPel"] = 1.into();
    }
    let input = build.join("metrics.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("metrics.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("metrics.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    assert_eq!(dump["OS_2"]["usLowerOpticalPointSize"], 80);
    assert_eq!(dump["OS_2"]["usUpperOpticalPointSize"], 120);
    assert_eq!(dump["post"]["italicAngle"], -12.5);
    assert_eq!(dump["post"]["maxMemType42"], 123);
    assert!(
        dump["glyf"]
            .as_object()
            .unwrap()
            .values()
            .all(|g| g["LTSH_yPel"] == 1),
        "the LTSH of ones was dropped"
    );
}

/// A negative `OS/2.version` is no version: it was turned into 0 and the
/// table written as the shortest, version 0 one (#704). Version 1 is written,
/// as for a table without a version, with a warning.
#[test]
fn a_negative_os2_version_is_written_as_version_1() {
    let build = scratch_dir("metrics-fields-os2-version");
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
    font["OS_2"]["version"] = (-1).into();
    let input = build.join("negative.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("negative.ttf");
    let built = std::process::Command::new(otfccbuild())
        .args([input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()])
        .output()
        .expect("otfccbuild runs");
    assert!(built.status.success(), "otfccbuild failed");
    let stderr = String::from_utf8_lossy(&built.stderr);
    assert!(stderr.contains("version -1 is not a version"), "{stderr}");
    let dumped = build.join("negative.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );
    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    assert_eq!(dump["OS_2"]["version"], 1);
}
