//! The OpenType layout reader stops at limits that guard against crafted
//! input, and no specification has them. A font with more lookups,
//! features or lookups per feature than the old fixed caps (300, 500 and
//! 300) must come back whole from a dump, without a warning.

mod support;

use std::ffi::OsStr;
use std::process::Command;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

const LOOKUPS: usize = 400;
const FEATURES: usize = 600;

/// `iosevka-r`'s dump with its GSUB replaced by `LOOKUPS` single
/// substitutions, `FEATURES` features of one or two lookups each, and one
/// more feature listing every lookup.
fn many_lookups_and_features(json: &str) -> serde_json::Value {
    let mut font: serde_json::Value = serde_json::from_str(json).expect("dump is JSON");
    let glyphs: Vec<String> = font["glyph_order"]
        .as_array()
        .expect("glyph_order")
        .iter()
        .map(|g| g.as_str().expect("glyph name").to_string())
        .collect();
    let lookup_name = |i: usize| format!("lookup_{i:03}");
    let mut lookups = serde_json::Map::new();
    let mut order = Vec::new();
    for i in 0..LOOKUPS {
        // A different pair of glyphs for each lookup, so that none of them
        // is merged into another.
        let mut subtable = serde_json::Map::new();
        subtable.insert(
            glyphs[10 + i].clone(),
            serde_json::Value::String(glyphs[10 + LOOKUPS + i].clone()),
        );
        lookups.insert(
            lookup_name(i),
            serde_json::json!({ "type": "gsub_single", "flags": {}, "subtables": [subtable] }),
        );
        order.push(lookup_name(i));
    }
    let mut features = serde_json::Map::new();
    let mut names = Vec::new();
    for j in 0..FEATURES {
        // Features past `LOOKUPS` get a second lookup, so that no two
        // features list the same lookups.
        let mut list = vec![lookup_name(j % LOOKUPS)];
        if j >= LOOKUPS {
            list.push(lookup_name((j + 7) % LOOKUPS));
        }
        let name = format!("ss{:02}_{j:05}", j % 20 + 1);
        features.insert(name.clone(), serde_json::json!(list));
        names.push(name);
    }
    let all = format!("aalt_{FEATURES:05}");
    features.insert(all.clone(), serde_json::json!(order));
    names.push(all);
    font["GSUB"] = serde_json::json!({
        "languages": { "DFLT_DFLT": { "features": names } },
        "features": features,
        "lookups": lookups,
        "lookupOrder": order,
    });
    font
}

#[test]
fn a_dump_keeps_every_lookup_and_feature() {
    let build = scratch_dir("otl-limits");
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
    let font = many_lookups_and_features(&std::fs::read_to_string(&base).unwrap());
    let input = build.join("many.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("many.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("many.dump.json");
    let output = Command::new(otfccdump())
        .args([ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()])
        .output()
        .expect("otfccdump runs");
    assert!(output.status.success(), "otfccdump failed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("[otl]"), "unexpected warning: {stderr}");

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    let gsub = &dump["GSUB"];
    assert_eq!(gsub["lookups"].as_object().unwrap().len(), LOOKUPS);
    let features = gsub["features"].as_object().unwrap();
    assert_eq!(features.len(), FEATURES + 1);
    let longest = features
        .values()
        .map(|lookups| lookups.as_array().unwrap().len())
        .max();
    assert_eq!(longest, Some(LOOKUPS));
    let language = gsub["languages"]["DFLT_DFLT"]["features"]
        .as_array()
        .unwrap();
    assert_eq!(language.len(), FEATURES + 1);
}
