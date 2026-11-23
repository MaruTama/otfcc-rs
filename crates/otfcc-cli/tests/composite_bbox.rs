//! A composite glyph's bounding box is that of its flattened outline. otfcc
//! applied a component's 2x2 transform transposed (`b` and `c` swapped
//! against the specification's x' = a·x + c·y, y' = b·x + d·y) and composed
//! nested components in the wrong order, so a rotated component, or a
//! flipped component placed by another composite (Source Serif 4's
//! `exclamdown`), got a wrong box in the glyf header and in `head`.

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
fn composite_bounding_boxes_follow_the_flattened_outline() {
    let build = scratch_dir("composite-bbox");
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
    // A simple glyph that is not symmetric, so a swapped term shows.
    let inner = "acutecomb";
    let points: Vec<(f64, f64)> = font["glyf"][inner]["contours"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|c| c.as_array().unwrap().clone())
        .map(|p| (p["x"].as_f64().unwrap(), p["y"].as_f64().unwrap()))
        .collect();
    assert!(!points.is_empty());

    let reference = |glyph: &str, x: f64, y: f64, [a, b, c, d]: [f64; 4]| serde_json::json!({ "glyph": glyph, "x": x, "y": y, "a": a, "b": b, "c": c, "d": d });
    let composite =
        |r: serde_json::Value| serde_json::json!({ "advanceWidth": 500, "references": [r] });
    // Rotated a quarter turn: x' = -y, y' = x.
    let quarter = [0., 1., -1., 0.];
    let flipped = [-1., 0., 0., -1.];
    let glyphs = [
        (
            "bbox.rotated",
            composite(reference(inner, 10., 20., quarter)),
        ),
        (
            "bbox.flipped",
            composite(reference(inner, 289., 673., flipped)),
        ),
        // The flipped glyph placed by another composite, as Source Serif 4
        // places it in `exclamdown`.
        (
            "bbox.nested",
            composite(reference("bbox.flipped", 0., -181., [1., 0., 0., 1.])),
        ),
        // A rotation of the rotated glyph: the transforms compose.
        (
            "bbox.twice",
            composite(reference("bbox.rotated", 5., 0., quarter)),
        ),
    ];
    let first = font["glyph_order"].as_array().unwrap().len();
    for (name, glyph) in &glyphs {
        font["glyph_order"]
            .as_array_mut()
            .unwrap()
            .push((*name).into());
        font["glyf"][*name] = glyph.clone();
    }
    let input = build.join("bbox.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("bbox.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let built = std::fs::read(&ttf).unwrap();

    // Each glyph as a map from the inner glyph's points, per the specification.
    type Map = fn((f64, f64)) -> (f64, f64);
    let maps: [Map; 4] = [
        |(x, y)| (10. - y, 20. + x),
        |(x, y)| (289. - x, 673. - y),
        |(x, y)| (289. - x, 673. - y - 181.),
        |(x, y)| (5. - (20. + x), -y + 10.),
    ];
    for (i, map) in maps.iter().enumerate() {
        let moved: Vec<(f64, f64)> = points.iter().map(|&p| map(p)).collect();
        let lo = |f: fn(&(f64, f64)) -> f64| moved.iter().map(f).fold(f64::MAX, f64::min) as i16;
        let hi = |f: fn(&(f64, f64)) -> f64| moved.iter().map(f).fold(f64::MIN, f64::max) as i16;
        let expected = [lo(|p| p.0), lo(|p| p.1), hi(|p| p.0), hi(|p| p.1)];
        assert_eq!(
            stored_bbox(&built, first + i),
            expected,
            "bounding box of {}",
            glyphs[i].0
        );
    }
}
