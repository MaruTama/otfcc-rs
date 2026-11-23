//! A CFF font's charstrings carry advance widths, but OpenType layout takes
//! them from `hmtx`, which a CFF font must have. otfcc read a CFF font's
//! widths from its charstrings and never read its `hmtx`, so a font whose
//! two disagree came back with the charstring widths.

mod support;

use std::ffi::OsStr;
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
fn cff_advance_widths_come_from_hmtx() {
    let build = scratch_dir("cff-hmtx-widths");
    let base = build.join("base.otf");
    // A CFF font whose `hmtx` gives glyph 1 a width its charstring does not.
    let mut font = std::fs::read(payload("WorkSans-Regular.otf")).unwrap();
    let hmtx = table_offset(&font, b"hmtx");
    // Glyph 1's long metric: advanceWidth, then lsb.
    font[hmtx + 4..hmtx + 6].copy_from_slice(&777u16.to_be_bytes());
    std::fs::write(&base, &font).unwrap();

    let dumped = build.join("base.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[base.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );
    // The font's Macintosh names are not UTF-8, which is beside the point.
    let read = |path: &std::path::Path| -> serde_json::Value {
        serde_json::from_str(&String::from_utf8_lossy(&std::fs::read(path).unwrap())).unwrap()
    };
    let dump = read(&dumped);
    let glyph1 = dump["glyph_order"][1].as_str().unwrap().to_string();
    assert_eq!(dump["glyf"][&glyph1]["advanceWidth"], 777);

    // And the rebuilt font writes it to both.
    let otf = build.join("rebuilt.otf");
    run_ok(
        "build",
        &otfccbuild(),
        &[dumped.as_os_str(), OsStr::new("-o"), otf.as_os_str()],
    );
    let rebuilt = std::fs::read(&otf).unwrap();
    let hmtx = table_offset(&rebuilt, b"hmtx");
    assert_eq!(
        u16::from_be_bytes([rebuilt[hmtx + 4], rebuilt[hmtx + 5]]),
        777
    );
}
