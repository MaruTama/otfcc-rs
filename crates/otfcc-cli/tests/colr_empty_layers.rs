//! A COLR v0 base glyph record may have no layers: the glyph is then drawn
//! as nothing in color. otfcc dropped such a record, so the glyph was drawn
//! with its plain outline instead. One the font has is kept now; a record
//! that only becomes empty because its layer glyphs do not exist is still
//! dropped.

mod support;

use std::ffi::OsStr;
use std::process::Command;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn a_color_glyph_with_no_layers_is_kept() {
    let build = scratch_dir("colr-empty-layers");
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
    let mappings = font["COLR"].as_array_mut().unwrap();
    assert!(mappings.len() >= 3);
    let empty = mappings[0]["from"].clone();
    let missing = mappings[1]["from"].clone();
    let kept = mappings[2].clone();
    mappings[0]["to"] = serde_json::json!([]);
    mappings[1]["to"] = serde_json::json!([{ "layer": "no-such-glyph", "paletteIndex": 0 }]);
    let input = build.join("empty.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("empty.ttf");
    let built = Command::new(otfccbuild())
        .args([input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()])
        .output()
        .expect("otfccbuild runs");
    assert!(built.status.success(), "otfccbuild failed");
    let stderr = String::from_utf8_lossy(&built.stderr);
    assert!(
        stderr.contains(&format!(
            "COLR decomposition for /{} is empty",
            missing.as_str().unwrap()
        )),
        "no warning for the dropped record: {stderr}"
    );
    let dumped = build.join("empty.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    let back = dump["COLR"].as_array().unwrap();
    let find = |from: &serde_json::Value| back.iter().find(|m| &m["from"] == from);
    assert_eq!(find(&empty).unwrap()["to"], serde_json::json!([]));
    assert!(
        find(&missing).is_none(),
        "a record of missing layers is dropped"
    );
    assert_eq!(find(&kept["from"]).unwrap(), &kept);
}
