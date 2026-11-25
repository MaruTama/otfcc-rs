//! A CFF charset gives each glyph its CID in a CID-keyed font, or the SID
//! of its name. otfcc wrote it as one range from the first glyph, assuming
//! the ids follow one another, so a subset CID font had its CIDs replaced
//! by glyph ids, and a name sharing an earlier string's SID shifted every
//! name after it. A format 0 charset was read as SIDs in a CID font too,
//! and CIDCount was replaced by the glyph count.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

fn read(path: &std::path::Path) -> serde_json::Value {
    // The fonts' Macintosh names are not UTF-8, which is beside the point.
    serde_json::from_str(&String::from_utf8_lossy(&std::fs::read(path).unwrap())).unwrap()
}

/// Builds `font` and dumps the result.
fn round_trip(name: &str, font: &serde_json::Value) -> serde_json::Value {
    let build = scratch_dir("cff-charset");
    let input = build.join(format!("{name}.json"));
    std::fs::write(&input, serde_json::to_vec(font).unwrap()).unwrap();
    let otf = build.join(format!("{name}.otf"));
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), otf.as_os_str()],
    );
    let dumped = build.join(format!("{name}.dump.json"));
    run_ok(
        "dump",
        &otfccdump(),
        &[otf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );
    read(&dumped)
}

fn base(payload_name: &str) -> serde_json::Value {
    let build = scratch_dir("cff-charset");
    let base = build.join(format!("{payload_name}.json"));
    run_ok(
        "base dump",
        &otfccdump(),
        &[
            payload(payload_name).as_os_str(),
            OsStr::new("-o"),
            base.as_os_str(),
        ],
    );
    read(&base)
}

#[test]
fn a_subset_cid_font_keeps_its_cids_and_cid_count() {
    let mut font = base("KRName-Regular.otf");
    let order: Vec<String> = font["glyph_order"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| g.as_str().unwrap().to_string())
        .collect();
    // Scattered CIDs, none following another: a format 0 charset.
    let cid_of = |gid: usize| 3 * gid + 2;
    for (gid, name) in order.iter().enumerate().skip(1) {
        font["glyf"][name]["CFF_CID"] = cid_of(gid).into();
    }
    font["CFF_"]["cidCount"] = 2000.into();
    let dump = round_trip("cids", &font);
    let back: Vec<String> = dump["glyph_order"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| g.as_str().unwrap().to_string())
        .collect();
    for (gid, name) in back.iter().enumerate().skip(1) {
        assert_eq!(
            dump["glyf"][name]["CFF_CID"],
            cid_of(gid),
            "CID of glyph {gid}"
        );
        assert_eq!(name, &format!("CID{}", cid_of(gid)));
    }
    assert_eq!(dump["CFF_"]["cidCount"], 2000);

    // A CIDCount below the largest CID is raised to cover it.
    font["CFF_"]["cidCount"] = 4.into();
    let dump = round_trip("small-count", &font);
    assert_eq!(dump["CFF_"]["cidCount"], cid_of(order.len() - 1) + 1);
}

#[test]
fn a_glyph_name_sharing_another_string_keeps_every_name() {
    let mut font = base("WorkSans-Regular.otf");
    let order: Vec<String> = font["glyph_order"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| g.as_str().unwrap().to_string())
        .collect();
    // The Weight string is glyph 2's name, so that name's SID comes first.
    font["CFF_"]["weight"] = order[2].clone().into();
    let dump = round_trip("weight", &font);
    assert_eq!(dump["glyph_order"], font["glyph_order"]);
}
