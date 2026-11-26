//! Composite glyph boxes that came out wrong (#703):
//!
//! - an anchored component of a composite that is itself a component was
//!   resolved where the outer composite places it, so the outer offset
//!   leaked into the stored one (`x: 810` for 410);
//! - a component without points (a space) joined the box as (0, 0, 0, 0);
//! - hand-written x/y beside an anchor, or `scaledOffset` with one, were
//!   used for the box though the font draws the component by its anchor;
//! - a scaled point at exactly -34.5 was rounded to -35, where fontTools
//!   and the original fonts round half up to -34 (a point of the font is
//!   still taken as the glyf writer stores it, rounded half away from
//!   zero).
//!
//! Every expected box below is the one fontTools computes.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

fn be16(data: &[u8], at: usize) -> u16 {
    u16::from_be_bytes([data[at], data[at + 1]])
}
fn be32(data: &[u8], at: usize) -> u32 {
    u32::from_be_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]])
}

/// The bounding box (xMin, yMin, xMax, yMax) the glyf header of glyph `gid` records.
fn stored_bbox(font: &[u8], gid: usize) -> [i16; 4] {
    let tables = be16(font, 4) as usize;
    let table = |tag: &[u8; 4]| {
        (0..tables)
            .map(|i| 12 + 16 * i)
            .find(|&r| &font[r..r + 4] == tag)
            .map(|r| be32(font, r + 8) as usize)
            .unwrap_or_else(|| panic!("no {} table", String::from_utf8_lossy(tag)))
    };
    let (head, loca, glyf) = (table(b"head"), table(b"loca"), table(b"glyf"));
    let start = if be16(font, head + 50) == 0 {
        2 * be16(font, loca + 2 * gid) as usize
    } else {
        be32(font, loca + 4 * gid) as usize
    };
    let at = glyf + start;
    [2, 4, 6, 8].map(|o| be16(font, at + o) as i16)
}

#[test]
fn composite_boxes_follow_the_components_as_drawn() {
    let build = scratch_dir("composite-components");
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
    let rotated = |outer: u16, inner: u16, scaled: bool| {
        // Hand-written x/y of 0 beside the anchor: wrong, as the build warns.
        serde_json::json!({
            "glyph": "cc.A", "x": 0, "y": 0, "a": 0, "b": 1, "c": -1, "d": 0,
            "isAnchored": true, "outer": outer, "inner": inner, "scaledOffset": scaled,
        })
    };
    let reference =
        |glyph: &str, x: i32, y: i32| serde_json::json!({ "glyph": glyph, "x": x, "y": y });
    // The outer composites come first in the glyph order, so they are
    // consolidated before the composite they place.
    let glyphs = [
        (
            "cc.P",
            serde_json::json!([reference("cc.H", 400, 50), {
                "glyph": "cc.A", "x": 0, "y": 0, "isAnchored": true, "outer": 6, "inner": 1,
            }]),
            [110, 150, 710, 1150],
        ),
        (
            "cc.Q",
            serde_json::json!([reference("cc.H", 400, 50)]),
            [310, 150, 710, 750],
        ),
        (
            "cc.S",
            serde_json::json!([reference("cc.A", 10, 0), rotated(2, 0, true)]),
            [-90, 100, 310, 700],
        ),
        (
            "cc.H",
            serde_json::json!([reference("cc.A", 10, 0), rotated(2, 0, false)]),
            [-90, 100, 310, 700],
        ),
        (
            "cc.B",
            serde_json::json!([reference("cc.A", 0, 0), reference("cc.space", 900, 900)]),
            [100, 100, 300, 500],
        ),
        (
            "cc.half",
            serde_json::json!([{ "glyph": "cc.odd", "x": -85, "y": 0, "a": 0.5, "d": 0.5 }]),
            [-34, 51, 66, 251],
        ),
    ];
    let first = font["glyph_order"].as_array().unwrap().len();
    for (name, references, _) in &glyphs {
        font["glyph_order"]
            .as_array_mut()
            .unwrap()
            .push((*name).into());
        font["glyf"][*name] = serde_json::json!({ "advanceWidth": 500, "references": references });
    }
    let simple = [
        (
            "cc.A",
            serde_json::json!([[on(100, 100), on(300, 100), on(300, 500), on(100, 500)]]),
        ),
        (
            "cc.odd",
            serde_json::json!([[on(101, 101), on(301, 101), on(301, 501)]]),
        ),
    ];
    for (name, contours) in &simple {
        font["glyph_order"]
            .as_array_mut()
            .unwrap()
            .push((*name).into());
        font["glyf"][*name] = serde_json::json!({ "advanceWidth": 500, "contours": contours });
    }
    // A point at -10.5 is stored as -11 (rounded half away from zero), and
    // the box is that of the stored point.
    let fraction = font["glyph_order"].as_array().unwrap().len();
    font["glyph_order"]
        .as_array_mut()
        .unwrap()
        .push("cc.fraction".into());
    font["glyf"]["cc.fraction"] = serde_json::json!({
        "advanceWidth": 500,
        "contours": [[{ "x": -10.5, "y": 0, "on": true }, on(100, 0), on(100, 100)]],
    });
    font["glyph_order"]
        .as_array_mut()
        .unwrap()
        .push("cc.space".into());
    font["glyf"]["cc.space"] = serde_json::json!({ "advanceWidth": 500 });
    // Keep the names, so the dump below can find the glyphs again.
    font["post"]["version"] = 2.into();

    let check = |ttf: &std::path::Path, what: &str| {
        let bytes = std::fs::read(ttf).unwrap();
        for (i, (name, _, expected)) in glyphs.iter().enumerate() {
            assert_eq!(stored_bbox(&bytes, first + i), *expected, "{name}, {what}");
        }
    };
    let input = build.join("components.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("components.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    check(&ttf, "built from hand-written JSON");
    assert_eq!(
        stored_bbox(&std::fs::read(&ttf).unwrap(), fraction),
        [-11, 0, 100, 100],
        "a point at -10.5"
    );

    // Dumped, the anchored component of the nested H keeps H's own offset,
    // and built again the boxes are the same.
    let dumped = build.join("components.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );
    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    let anchored = &dump["glyf"]["cc.H"]["references"][1];
    assert_eq!(
        (anchored["x"].clone(), anchored["y"].clone()),
        (410.into(), 400.into())
    );
    let rebuilt = build.join("rebuilt.ttf");
    run_ok(
        "rebuild",
        &otfccbuild(),
        &[dumped.as_os_str(), OsStr::new("-o"), rebuilt.as_os_str()],
    );
    check(&rebuilt, "dumped and built again");
}
