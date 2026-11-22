//! A lookup's mark filtering set (`lookupFlag` bit `USE_MARK_FILTERING_SET`
//! and its `markFilteringSet`) and the GDEF mark glyph sets it names must
//! come back from a build and a dump. They used to be dropped, which changed
//! which marks such a lookup skips.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn mark_filtering_sets_survive_a_build_and_a_dump() {
    let build = scratch_dir("mark-filtering");
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
    let (a, b, c, d) = (&glyphs[10], &glyphs[11], &glyphs[12], &glyphs[13]);
    // In glyph order, which is how a set is written back.
    let sets = serde_json::json!([[a, b], [c]]);
    font["GDEF"]["markGlyphSets"] = sets.clone();
    font["GSUB"] = serde_json::json!({
        "languages": { "DFLT_DFLT": { "features": ["ccmp_00000"] } },
        "features": { "ccmp_00000": ["filtered"] },
        "lookups": {
            "filtered": {
                "type": "gsub_single",
                "flags": { "ignoreBases": true },
                "markFilteringSet": 1,
                "subtables": [{ (a.clone()): d }],
            }
        },
        "lookupOrder": ["filtered"],
    });
    let input = build.join("filtered.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("filtered.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("filtered.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    assert_eq!(dump["GDEF"]["markGlyphSets"], sets);
    // A dump names lookups after their feature, so take the only one.
    let lookups = dump["GSUB"]["lookups"].as_object().unwrap();
    assert_eq!(lookups.len(), 1);
    let lookup = lookups.values().next().unwrap();
    assert_eq!(lookup["markFilteringSet"], 1);
    // The other flags are kept, and the set's own bit is not a flag label.
    assert_eq!(lookup["flags"], serde_json::json!({ "ignoreBases": true }));
}
