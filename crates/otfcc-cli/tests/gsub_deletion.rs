//! A multiple substitution to no glyphs deletes its glyph. The OpenType
//! specification prohibits it, but fonts ship it (Noto Sans Javanese) and
//! HarfBuzz applies it, so otfcc keeps one the font has: dropping it changed
//! how the rebuilt font shapes. A sequence that only becomes empty because
//! its glyphs do not exist is still dropped, and so is an empty alternate
//! set, which does nothing.

mod support;

use std::ffi::OsStr;
use std::process::Command;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn a_multiple_substitution_to_no_glyphs_is_kept() {
    let build = scratch_dir("gsub-deletion");
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
    let (a, b, c) = (&glyphs[10], &glyphs[11], &glyphs[12]);
    font["GSUB"] = serde_json::json!({
        "languages": { "DFLT_DFLT": { "features": ["ccmp_00000", "salt_00001"] } },
        "features": { "ccmp_00000": ["deletes"], "salt_00001": ["alternates"] },
        "lookups": {
            "deletes": {
                "type": "gsub_multiple",
                "flags": {},
                "subtables": [{ (a.clone()): [], (b.clone()): ["no-such-glyph"], (c.clone()): [a] }],
            },
            "alternates": {
                "type": "gsub_alternate",
                "flags": {},
                "subtables": [{ (a.clone()): [], (b.clone()): [c] }],
            }
        },
        "lookupOrder": ["deletes", "alternates"],
    });
    let input = build.join("deletion.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("deletion.ttf");
    let built = Command::new(otfccbuild())
        .args([input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()])
        .output()
        .expect("otfccbuild runs");
    assert!(built.status.success(), "otfccbuild failed");
    let stderr = String::from_utf8_lossy(&built.stderr);
    assert!(
        stderr.contains(&format!("glyph /{a} to no glyphs is against the OpenType")),
        "no warning for the deletion: {stderr}"
    );
    let dumped = build.join("deletion.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    let lookups = dump["GSUB"]["lookups"].as_object().unwrap();
    let subtable_of = |kind: &str| {
        lookups
            .values()
            .find(|l| l["type"] == kind)
            .map(|l| l["subtables"][0].clone())
            .unwrap_or_else(|| panic!("no {kind} lookup"))
    };
    // The deletion is kept; the sequence of a missing glyph is not.
    assert_eq!(
        subtable_of("gsub_multiple"),
        serde_json::json!({ (a.clone()): [], (c.clone()): [a] })
    );
    // An empty alternate set is dropped.
    assert_eq!(
        subtable_of("gsub_alternate"),
        serde_json::json!({ (b.clone()): [c] })
    );
}
