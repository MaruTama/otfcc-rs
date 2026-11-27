//! The Unicode platform's encodings 0/0, 0/1 and 0/2 are deprecated but
//! defined, and their subtables map Unicode code points like 0/3 does.
//! otfcc read only 0/3 to 0/5, 3/1 and 3/10, so a font whose subtables were
//! all filed under 0/1 came back with no `cmap` at all.

// A test file: its helper functions may `unwrap()` like its tests do.
#![allow(clippy::unwrap_used)]

mod support;

use std::ffi::OsStr;
use support::{otfccdump, payload, run_ok, scratch_dir};

/// `font` with every `cmap` encoding record renamed to
/// `platform`/`encoding`, pointing at the same subtables.
fn with_cmap_encoding(mut font: Vec<u8>, platform: u16, encoding: u16) -> Vec<u8> {
    let tables = usize::from(u16::from_be_bytes([font[4], font[5]]));
    let cmap = (0..tables)
        .map(|i| 12 + 16 * i)
        .find(|&r| &font[r..r + 4] == b"cmap")
        .map(|r| {
            u32::from_be_bytes([font[r + 8], font[r + 9], font[r + 10], font[r + 11]]) as usize
        })
        .expect("a cmap table");
    let records = usize::from(u16::from_be_bytes([font[cmap + 2], font[cmap + 3]]));
    for i in 0..records {
        let at = cmap + 4 + 8 * i;
        font[at..at + 2].copy_from_slice(&platform.to_be_bytes());
        font[at + 2..at + 4].copy_from_slice(&encoding.to_be_bytes());
    }
    font
}

fn dump(name: &str, font: &[u8]) -> serde_json::Value {
    let build = scratch_dir("cmap-legacy-unicode");
    let ttf = build.join(format!("{name}.ttf"));
    std::fs::write(&ttf, font).unwrap();
    let dumped = build.join(format!("{name}.json"));
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );
    serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap()
}

#[test]
fn subtables_of_the_deprecated_unicode_encodings_are_read() {
    let original = std::fs::read(payload("iosevka-r.ttf")).unwrap();
    let expected = dump("original", &original)["cmap"].clone();
    assert!(expected.as_object().is_some_and(|m| !m.is_empty()));
    for encoding in 0..=2 {
        let cmap = dump(
            &format!("unicode-{encoding}"),
            &with_cmap_encoding(original.clone(), 0, encoding),
        )["cmap"]
            .clone();
        assert_eq!(cmap, expected, "cmap read from 0/{encoding}");
    }
}
