//! A glyph's vertical origin is its top side bearing (`vmtx`) plus the top
//! of its box. A CFF glyph has no box to read, and otfcc took its top as 0,
//! so a CFF font with `vmtx` but no `VORG` came back with every vertical
//! origin moved down by the glyph's top, and a `VORG` holding the wrong
//! origins. The top now comes from the glyph's curves.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

/// `font` without its `table`: the record is taken out of the table
/// directory, which is all a reader goes by.
fn without_table(font: &[u8], table: &[u8; 4]) -> Vec<u8> {
    let tables = usize::from(u16::from_be_bytes([font[4], font[5]]));
    let records: Vec<&[u8]> = (0..tables)
        .map(|i| &font[12 + 16 * i..28 + 16 * i])
        .filter(|r| &r[..4] != table)
        .collect();
    assert_eq!(records.len(), tables - 1, "the font has the table");
    let mut out = font[..4].to_vec();
    out.extend_from_slice(&(records.len() as u16).to_be_bytes());
    out.extend_from_slice(&font[6..12]);
    for r in records {
        out.extend_from_slice(r);
    }
    // Keep the table data where the remaining offsets point.
    out.extend_from_slice(&[0; 16]);
    out.extend_from_slice(&font[out.len()..]);
    out
}

#[test]
fn cff_vertical_origins_survive_without_vorg() {
    let build = scratch_dir("cff-vertical-origin");
    let base = build.join("base.json");
    run_ok(
        "base dump",
        &otfccdump(),
        &[
            payload("WorkSans-Regular.otf").as_os_str(),
            OsStr::new("-o"),
            base.as_os_str(),
        ],
    );
    // The font's Macintosh names are not UTF-8, which is beside the point.
    let read = |path: &std::path::Path| -> serde_json::Value {
        serde_json::from_str(&String::from_utf8_lossy(&std::fs::read(path).unwrap())).unwrap()
    };
    let mut font = read(&base);
    font["vhea"] = serde_json::json!({
        "version": 1.0625, "ascent": 500, "descent": -500, "lineGap": 0,
        "caretSlopeRise": 0, "caretSlopeRun": 1, "caretOffset": 0,
    });
    for glyph in font["glyf"].as_object_mut().unwrap().values_mut() {
        glyph["advanceHeight"] = 1000.into();
        glyph["verticalOrigin"] = 880.into();
    }
    let input = build.join("vertical.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let with_vorg = build.join("vertical.otf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), with_vorg.as_os_str()],
    );
    // The same font with only `vmtx` to give its vertical origins.
    let otf = build.join("no-vorg.otf");
    std::fs::write(
        &otf,
        without_table(&std::fs::read(&with_vorg).unwrap(), b"VORG"),
    )
    .unwrap();
    let dumped = build.join("no-vorg.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[otf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump = read(&dumped);
    let moved: Vec<&String> = dump["glyf"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, g)| g["verticalOrigin"] != 880)
        .map(|(name, _)| name)
        .collect();
    assert!(moved.is_empty(), "vertical origins moved: {moved:?}");
}
