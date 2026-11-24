//! A symbol font maps its glyphs under Windows Symbol (3/0), not Unicode.
//! otfcc did not read 3/0, so a font with no other subtable came back with
//! no `cmap`. Its mappings are read now, the encoding is dumped as
//! `"cmap_encoding": "symbol"`, and the build files the format 4 subtable
//! under 3/0 again. A font with a Windows Unicode subtable as well keeps
//! that one alone.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

/// The offset of the `cmap` table in `font`.
fn cmap_offset(font: &[u8]) -> usize {
    let tables = usize::from(u16::from_be_bytes([font[4], font[5]]));
    (0..tables)
        .map(|i| 12 + 16 * i)
        .find(|&r| &font[r..r + 4] == b"cmap")
        .map(|r| {
            u32::from_be_bytes([font[r + 8], font[r + 9], font[r + 10], font[r + 11]]) as usize
        })
        .expect("a cmap table")
}

/// The (platform, encoding) of each `cmap` encoding record of `font`.
fn encodings(font: &[u8]) -> Vec<(u16, u16)> {
    let cmap = cmap_offset(font);
    let records = usize::from(u16::from_be_bytes([font[cmap + 2], font[cmap + 3]]));
    (0..records)
        .map(|i| cmap + 4 + 8 * i)
        .map(|at| {
            (
                u16::from_be_bytes([font[at], font[at + 1]]),
                u16::from_be_bytes([font[at + 2], font[at + 3]]),
            )
        })
        .collect()
}

fn dump(font: &std::path::Path, out: &std::path::Path) -> serde_json::Value {
    run_ok(
        "dump",
        &otfccdump(),
        &[font.as_os_str(), OsStr::new("-o"), out.as_os_str()],
    );
    serde_json::from_str(&std::fs::read_to_string(out).unwrap()).unwrap()
}

#[test]
fn a_symbol_cmap_survives_a_dump_and_a_build() {
    let build = scratch_dir("cmap-symbol");
    let original = std::fs::read(payload("iosevka-r.ttf")).unwrap();
    let unicode = dump(&payload("iosevka-r.ttf"), &build.join("unicode.json"));
    assert!(unicode.get("cmap_encoding").is_none());

    // The same subtables, every record filed under Windows Symbol.
    let mut symbol = original.clone();
    let cmap = cmap_offset(&symbol);
    for i in 0..encodings(&original).len() {
        let at = cmap + 4 + 8 * i;
        symbol[at..at + 4].copy_from_slice(&[0, 3, 0, 0]);
    }
    let symbol_ttf = build.join("symbol.ttf");
    std::fs::write(&symbol_ttf, &symbol).unwrap();
    let dumped = dump(&symbol_ttf, &build.join("symbol.json"));
    assert_eq!(dumped["cmap_encoding"], "symbol");
    assert_eq!(dumped["cmap"], unicode["cmap"]);

    // Built again, the format 4 subtable is under 3/0 alone.
    let rebuilt = build.join("rebuilt.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[
            build.join("symbol.json").as_os_str(),
            OsStr::new("-o"),
            rebuilt.as_os_str(),
        ],
    );
    let rebuilt_bytes = std::fs::read(&rebuilt).unwrap();
    let records = encodings(&rebuilt_bytes);
    assert!(records.contains(&(3, 0)), "{records:?}");
    assert!(
        !records.contains(&(3, 1)) && !records.contains(&(0, 3)),
        "{records:?}"
    );
    let again = dump(&rebuilt, &build.join("rebuilt.json"));
    assert_eq!(again["cmap_encoding"], "symbol");
    assert_eq!(again["cmap"], unicode["cmap"]);

    // A font with a Windows Unicode subtable besides 3/0 stays Unicode.
    let mut both = original.clone();
    let first = cmap_offset(&both) + 4;
    both[first..first + 4].copy_from_slice(&[0, 3, 0, 0]);
    assert!(encodings(&both).contains(&(3, 1)) || encodings(&both).contains(&(3, 10)));
    let both_ttf = build.join("both.ttf");
    std::fs::write(&both_ttf, &both).unwrap();
    assert!(
        dump(&both_ttf, &build.join("both.json"))
            .get("cmap_encoding")
            .is_none()
    );
}
