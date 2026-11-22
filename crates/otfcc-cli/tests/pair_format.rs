//! Which pairs a PairPos subtable matches depends on its format, and a
//! matching pair ends the lookup even when it moves nothing. So a pair
//! subtable keeps its format through a dump and a build, and a format 1
//! pair listed with no adjustment (`{}` in the matrix) stays listed. Only a
//! subtable whose JSON gives no format is written in the smaller format.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn pair_subtables_keep_their_format_and_listed_pairs() {
    let build = scratch_dir("pair-format");
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
    // The same sparse matrix three times: as format 2, which format 1 would
    // write smaller; with no format, which is left to the build; and as
    // format 1 with the pair (a, b) listed with no adjustment.
    let classes = serde_json::json!({
        "first": { (a.clone()): 1 },
        "second": { (b.clone()): 1, (c.clone()): 2 },
    });
    let with = |format: Option<u16>, matrix: serde_json::Value| {
        let mut st = classes.clone();
        if let Some(format) = format {
            st["format"] = format.into();
        }
        st["matrix"] = matrix;
        st
    };
    let sparse = serde_json::json!([[0, 0, 0], [0, 0, 30]]);
    let listed = serde_json::json!([[0, 0, 0], [0, {}, 30]]);
    font["GPOS"] = serde_json::json!({
        "languages": { "DFLT_DFLT": { "features": ["kern_00000"] } },
        "features": { "kern_00000": ["pairs"] },
        "lookups": {
            "pairs": {
                "type": "gpos_pair",
                "flags": {},
                "subtables": [
                    with(Some(2), sparse.clone()),
                    with(None, sparse),
                    with(Some(1), listed),
                ],
            }
        },
        "lookupOrder": ["pairs"],
    });
    let input = build.join("pairs.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("pairs.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("pairs.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    let lookup = dump["GPOS"]["lookups"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()
        .clone();
    let subtables = lookup["subtables"].as_array().unwrap();
    let formats: Vec<&serde_json::Value> = subtables.iter().map(|st| &st["format"]).collect();
    assert_eq!(formats, [2, 1, 1]);
    // Read back from format 1, the listed pair is the `{}` cell.
    let cells: Vec<&serde_json::Value> = subtables[2]["matrix"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|row| row.as_array().unwrap())
        .collect();
    assert_eq!(
        cells
            .iter()
            .filter(|c| **c == &serde_json::json!({}))
            .count(),
        1
    );
    assert_eq!(
        subtables[1]["matrix"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|row| row.as_array().unwrap())
            .filter(|c| c.is_object() && c.as_object().unwrap().is_empty())
            .count(),
        0,
        "a subtable without listed pairs gets none"
    );
}
