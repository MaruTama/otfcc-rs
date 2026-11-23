//! Feature parameters (`featureParamsOffset`) carry the names a user sees
//! for stylistic sets and character variants, and the `size` feature's
//! design size. They used to be dropped, and a `size` feature, which must
//! have no lookups, was dropped whole. A feature with parameters is written
//! as `{"lookups": [...], "params": {...}}`.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn feature_parameters_survive_a_build_and_a_dump() {
    let build = scratch_dir("feature-params");
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
    let glyphs: Vec<String> = font["glyph_order"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| g.as_str().unwrap().to_string())
        .collect();
    let (a, b) = (&glyphs[10], &glyphs[11]);
    let ss01 = serde_json::json!({ "uiLabel": 276 });
    let cv01 = serde_json::json!({
        "uiLabel": 277, "tooltip": 278, "paramLabels": { "first": 279, "count": 2 },
        "characters": [97, 98],
    });
    let size = serde_json::json!({
        "designSize": 18, "subfamily": 3, "subfamilyName": 280, "rangeStart": 13.9, "rangeEnd": 24,
    });
    font["GSUB"] = serde_json::json!({
        "languages": { "DFLT_DFLT": { "features": ["ss01_00000", "cv01_00001", "liga_00002"] } },
        "features": {
            "ss01_00000": { "lookups": ["alt"], "params": ss01 },
            "cv01_00001": { "lookups": ["alt"], "params": cv01 },
            "liga_00002": ["alt"],
        },
        "lookups": {
            "alt": { "type": "gsub_single", "flags": {}, "subtables": [{ (a.clone()): b }] }
        },
        "lookupOrder": ["alt"],
    });
    // `size` has no lookups at all, as the specification requires.
    font["GPOS"] = serde_json::json!({
        "languages": { "DFLT_DFLT": { "features": ["size_00000"] } },
        "features": { "size_00000": { "params": size } },
        "lookups": {
            "unused": { "type": "gpos_single", "flags": {}, "subtables": [{ (a.clone()): { "dx": 1 } }] }
        },
        "lookupOrder": ["unused"],
    });
    let input = build.join("params.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("params.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("params.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    let feature = |table: &str, tag: &str| {
        dump[table]["features"]
            .as_object()
            .unwrap()
            .iter()
            .find(|(name, _)| name.starts_with(tag))
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| panic!("no {tag} in {table}"))
    };
    assert_eq!(feature("GSUB", "ss01")["params"], ss01);
    assert_eq!(feature("GSUB", "cv01")["params"], cv01);
    // A feature without parameters is still a bare lookup list.
    assert!(feature("GSUB", "liga").is_array());
    let size_feature = feature("GPOS", "size");
    assert_eq!(size_feature["params"], size);
    assert_eq!(size_feature["lookups"], serde_json::json!([]));
}
