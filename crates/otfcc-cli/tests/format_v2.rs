//! `otfccdump --format v2`: a directory with `font.json` and one file per
//! glyph under `glyf/`, contour points as `[x, y]` arrays. Building from it
//! must give the same font, byte for byte, as building from the v1 dump of
//! the same input, and `otfccbuild` must not follow a glyph file path out of
//! the directory.

mod support;

use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

/// `extra` (options given as strings) followed by `args`.
fn with_extra<'a>(extra: &'a [&'a str], args: &[&'a OsStr]) -> Vec<&'a OsStr> {
    extra
        .iter()
        .map(OsStr::new)
        .chain(args.iter().copied())
        .collect()
}

fn dump_and_build(build: &Path, name: &str, input: &Path, extra: &[&str]) -> (Vec<u8>, Vec<u8>) {
    let v1_json = build.join(format!("{name}.v1.json"));
    let v2_dir = build.join(format!("{name}.v2"));
    let _ = std::fs::remove_dir_all(&v2_dir);
    run_ok(
        &format!("{name} v1 dump"),
        &otfccdump(),
        &with_extra(
            extra,
            &[input.as_os_str(), OsStr::new("-o"), v1_json.as_os_str()],
        ),
    );
    run_ok(
        &format!("{name} v2 dump"),
        &otfccdump(),
        &with_extra(
            extra,
            &[
                OsStr::new("--format"),
                OsStr::new("v2"),
                input.as_os_str(),
                OsStr::new("-o"),
                v2_dir.as_os_str(),
            ],
        ),
    );
    let mut built = Vec::new();
    for (label, source) in [("v1", v1_json.as_path()), ("v2", v2_dir.as_path())] {
        let out = build.join(format!("{name}.{label}.bin"));
        run_ok(
            &format!("{name} {label} build"),
            &otfccbuild(),
            &[
                source.as_os_str(),
                OsStr::new("-o"),
                out.as_os_str(),
                OsStr::new("--keep-modified-time"),
            ],
        );
        built.push(std::fs::read(&out).expect("the build wrote its output"));
    }
    let v2 = built.pop().expect("two builds");
    let v1 = built.pop().expect("two builds");
    (v1, v2)
}

#[test]
fn a_v2_dump_builds_the_same_font_as_a_v1_dump() {
    let build = scratch_dir("format-v2");
    // TrueType with hints, CFF, a variable font, and one whose glyph
    // order comes from the JSON order alone.
    let cases: &[(&str, &str, &[&str])] = &[
        ("iosevka-r", "iosevka-r.ttf", &[]),
        ("WorkSans-Regular", "WorkSans-Regular.otf", &[]),
        ("gvar-test", "gvar-test.ttf", &[]),
        (
            "Molengo-no-order",
            "Molengo-Regular.ttf",
            &["--ignore-glyph-order"],
        ),
    ];
    for (name, file, extra) in cases {
        let (v1, v2) = dump_and_build(&build, name, &payload(file), extra);
        assert!(
            v1 == v2,
            "{name}: the v2 dump built a different font from the v1 dump"
        );
    }
}

#[test]
fn a_v2_dump_needs_an_empty_output_directory() {
    let build = scratch_dir("format-v2-occupied");
    let dir = build.join("occupied");
    std::fs::create_dir_all(&dir).expect("scratch directory");
    std::fs::write(dir.join("left-over.json"), b"{}").expect("scratch file");
    let status = Command::new(otfccdump())
        .args([OsStr::new("--format"), OsStr::new("v2")])
        .arg(payload("iosevka-r.ttf"))
        .arg("-o")
        .arg(&dir)
        .status()
        .expect("otfccdump runs");
    assert!(
        !status.success(),
        "dumping into a non-empty directory must fail"
    );
}

#[test]
fn a_glyph_path_outside_the_glyph_directory_is_refused() {
    let build = scratch_dir("format-v2-escape");
    let dir = build.join("escape");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("glyf")).expect("scratch directory");
    std::fs::write(build.join("outside.json"), br#"{"advanceWidth": 1}"#).expect("scratch file");
    std::fs::write(
        dir.join("font.json"),
        br#"{"glyf": {".notdef": "glyf/../../outside.json"}}"#,
    )
    .expect("scratch file");
    let out = build.join("escape.ttf");
    let status = Command::new(otfccbuild())
        .arg(&dir)
        .arg("-o")
        .arg(&out)
        .status()
        .expect("otfccbuild runs");
    assert!(!status.success(), "a glyph path with .. must be refused");
}
