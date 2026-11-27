//! `OS/2` version 0 ends at `usWinDescent`. otfcc raised every table to
//! version 1, adding code page ranges of 0, so a version 0 table did not
//! come back as it was. It is kept now; a table with no version is still
//! version 1.

// A test file: its helper functions may `unwrap()` like its tests do.
#![allow(clippy::unwrap_used)]

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

/// The length of the `OS/2` table in the font at `font`.
fn os2_length(font: &[u8]) -> u32 {
    let tables = usize::from(u16::from_be_bytes([font[4], font[5]]));
    (0..tables)
        .map(|i| 12 + 16 * i)
        .find(|&r| &font[r..r + 4] == b"OS/2")
        .map(|r| u32::from_be_bytes([font[r + 12], font[r + 13], font[r + 14], font[r + 15]]))
        .expect("an OS/2 table")
}

/// Builds `font` and returns the built bytes and their dump.
fn build_and_dump(name: &str, font: &serde_json::Value) -> (Vec<u8>, serde_json::Value) {
    let build = scratch_dir("os2-version-0");
    let input = build.join(format!("{name}.json"));
    std::fs::write(&input, serde_json::to_vec(font).unwrap()).unwrap();
    let ttf = build.join(format!("{name}.ttf"));
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join(format!("{name}.dump.json"));
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );
    (
        std::fs::read(&ttf).unwrap(),
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap(),
    )
}

#[test]
fn an_os2_version_0_table_is_kept() {
    let build = scratch_dir("os2-version-0");
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

    font["OS_2"]["version"] = 0.into();
    let (built, dump) = build_and_dump("v0", &font);
    assert_eq!(dump["OS_2"]["version"], 0);
    assert_eq!(os2_length(&built), 78, "version 0 ends at usWinDescent");

    font["OS_2"].as_object_mut().unwrap().remove("version");
    let (built, dump) = build_and_dump("no-version", &font);
    assert_eq!(dump["OS_2"]["version"], 1);
    assert_eq!(os2_length(&built), 86);
}
