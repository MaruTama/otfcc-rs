//! Inputs a reader must survive without a panic: a CFF font whose `vmtx`,
//! sized by `maxp`, covers fewer glyphs than its CFF table has, and a
//! chaining rule in JSON whose input is empty.

mod support;

use std::ffi::OsStr;
use std::process::Command;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

/// The offset of `table` in `font`.
fn table_offset(font: &[u8], table: &[u8; 4]) -> usize {
    let tables = usize::from(u16::from_be_bytes([font[4], font[5]]));
    (0..tables)
        .map(|i| 12 + 16 * i)
        .find(|&r| &font[r..r + 4] == table)
        .map(|r| {
            u32::from_be_bytes([font[r + 8], font[r + 9], font[r + 10], font[r + 11]]) as usize
        })
        .expect("the font has the table")
}

#[test]
fn a_cff_font_whose_maxp_counts_fewer_glyphs_dumps() {
    let build = scratch_dir("robust-inputs");
    let mut font = std::fs::read(payload("KRName-Regular.otf")).unwrap();
    assert!(font.windows(4).any(|w| w == b"vmtx"), "the font has vmtx");
    // maxp.numGlyphs = 2: vmtx then covers two glyphs of the CFF table's 38.
    let maxp = table_offset(&font, b"maxp");
    font[maxp + 4..maxp + 6].copy_from_slice(&2u16.to_be_bytes());
    let otf = build.join("short-maxp.otf");
    std::fs::write(&otf, &font).unwrap();
    run_ok(
        "dump",
        &otfccdump(),
        &[
            otf.as_os_str(),
            OsStr::new("-o"),
            build.join("short-maxp.json").as_os_str(),
        ],
    );
}

#[test]
fn a_chaining_rule_with_an_empty_input_is_dropped() {
    let build = scratch_dir("robust-inputs");
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
    let order: Vec<String> = font["glyph_order"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| g.as_str().unwrap().to_string())
        .collect();
    // Rules whose `inputEnds` is at `inputBegins`: no input glyph at all.
    let rules: Vec<serde_json::Value> = (0..6)
        .map(|i| {
            serde_json::json!({
                "match": [[order[10 + i]], [order[30 + i]]],
                "apply": [],
                "inputBegins": 0,
                "inputEnds": 0,
            })
        })
        .collect();
    font["GSUB"]["lookups"]["empty_input"] =
        serde_json::json!({ "type": "gsub_chaining", "flags": {}, "subtables": rules });
    font["GSUB"]["lookupOrder"]
        .as_array_mut()
        .unwrap()
        .push("empty_input".into());
    let input = build.join("empty-input.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let output = Command::new(otfccbuild())
        .args([
            input.as_os_str(),
            OsStr::new("-o"),
            build.join("empty-input.ttf").as_os_str(),
        ])
        .output()
        .expect("otfccbuild runs");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("Ignored empty subtable"), "{stderr}");
}
