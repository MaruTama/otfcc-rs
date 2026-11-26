//! `--name-by-hash` names a glyph by a hash of what it draws. Two composite
//! glyphs that differed only in a component's `scaledOffset`, or only in
//! the points an anchored component joins, drew differently but got the
//! same hash (#703).

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn how_a_component_is_placed_changes_its_hash() {
    let build = scratch_dir("name-by-hash");
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
    let on = |x: i32, y: i32| serde_json::json!({ "x": x, "y": y, "on": true });
    let scaled = |scaled: bool| {
        serde_json::json!([{
            "glyph": "nh.A", "x": 10, "y": 20, "a": 0.5, "d": 0.5, "scaledOffset": scaled,
        }])
    };
    let anchored = |outer: u16| {
        serde_json::json!([
            { "glyph": "nh.A", "x": 0, "y": 0 },
            { "glyph": "nh.A", "x": 0, "y": 0, "isAnchored": true, "outer": outer, "inner": 0 },
        ])
    };
    let composites = [
        ("nh.unscaled", scaled(false)),
        ("nh.scaled", scaled(true)),
        ("nh.anchored1", anchored(1)),
        ("nh.anchored2", anchored(2)),
    ];
    let first = font["glyph_order"].as_array().unwrap().len();
    for (name, references) in &composites {
        font["glyph_order"]
            .as_array_mut()
            .unwrap()
            .push((*name).into());
        font["glyf"][*name] = serde_json::json!({ "advanceWidth": 500, "references": references });
    }
    font["glyph_order"]
        .as_array_mut()
        .unwrap()
        .push("nh.A".into());
    font["glyf"]["nh.A"] = serde_json::json!({
        "advanceWidth": 500,
        "contours": [[on(100, 100), on(300, 100), on(300, 500), on(100, 500)]],
    });
    let input = build.join("hash.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("hash.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("hash.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[
            ttf.as_os_str(),
            OsStr::new("--name-by-hash"),
            OsStr::new("-o"),
            dumped.as_os_str(),
        ],
    );
    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    let order = dump["glyph_order"].as_array().unwrap();
    let names: Vec<&str> = (first..first + composites.len())
        .map(|g| order[g].as_str().unwrap())
        .collect();
    // A hash is five groups of hex digits; a name taken already gets a
    // `-2` suffix, so compare the hashes themselves.
    let hash = |name: &str| name.split('-').take(5).collect::<Vec<_>>().join("-");
    assert_ne!(hash(names[0]), hash(names[1]), "{names:?}");
    assert_ne!(hash(names[2]), hash(names[3]), "{names:?}");
}
