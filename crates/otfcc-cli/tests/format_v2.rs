//! `otfccdump --format v2`: a directory with `font.json` and one file per
//! glyph under `glyf/`, contour points as `[x, y]` arrays, and glyph lists
//! that several chaining rules share named in their lookup. Building from it
//! must give the same font, byte for byte, as building from the v1 dump of
//! the same input, and `otfccbuild` must not follow a glyph file path out of
//! the directory, nor read more than the directory holds.

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
        // Class-based chaining lookups, written with named glyph classes.
        ("NotoNastaliqUrdu", "NotoNastaliqUrdu-Regular.ttf", &[]),
    ];
    for (name, file, extra) in cases {
        let (v1, v2) = dump_and_build(&build, name, &payload(file), extra);
        assert!(
            v1 == v2,
            "{name}: the v2 dump built a different font from the v1 dump"
        );
    }
    let noto = std::fs::read_to_string(build.join("NotoNastaliqUrdu.v2/font.json"))
        .expect("the v2 dump wrote font.json");
    assert!(
        noto.contains(r#""classes": {"#) && noto.contains(r#""@c1""#),
        "the v2 dump names the glyph lists that chaining rules share"
    );
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

/// Builds the v2 directory `dir` and returns otfccbuild's stderr, after
/// checking that the build failed.
fn build_fails(dir: &Path, out: &Path, why: &str) -> String {
    let output = Command::new(otfccbuild())
        .arg(dir)
        .arg("-o")
        .arg(out)
        .output()
        .expect("otfccbuild runs");
    assert!(!output.status.success(), "{why}");
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// A v2 directory whose `font.json` is `index`, with an empty `glyf/`.
fn v2_dir(build: &Path, name: &str, index: &str) -> std::path::PathBuf {
    let dir = build.join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("glyf")).expect("scratch directory");
    std::fs::write(dir.join("font.json"), index).expect("scratch file");
    dir
}

// One glyph file named by many glyphs was read and parsed once per glyph,
// so a small directory could make the build take any amount of memory. A
// dump never names one file twice.
#[test]
fn a_glyph_file_named_by_two_glyphs_is_refused() {
    let build = scratch_dir("format-v2-shared-file");
    let dir = v2_dir(
        &build,
        "shared",
        r#"{"glyf": {".notdef": "glyf/a.json", "a": "glyf/a.json"}}"#,
    );
    std::fs::write(dir.join("glyf/a.json"), br#"{"advanceWidth": 1}"#).expect("scratch file");
    let stderr = build_fails(
        &dir,
        &build.join("shared.ttf"),
        "a shared file must be refused",
    );
    assert!(stderr.contains("named by more than one glyph"), "{stderr}");
}

// An index of more glyphs than a font can have is refused before any file
// is read.
#[test]
fn an_index_of_too_many_glyphs_is_refused_before_reading() {
    let build = scratch_dir("format-v2-too-many");
    let entries: Vec<String> = (0..=65_535)
        .map(|j| format!("\"g{j}\": \"glyf/g{j}.json\""))
        .collect();
    let dir = v2_dir(
        &build,
        "many",
        &format!("{{\"glyf\": {{{}}}}}", entries.join(",")),
    );
    let stderr = build_fails(
        &dir,
        &build.join("many.ttf"),
        "too many glyphs must be refused",
    );
    assert!(stderr.contains("more than a font can have"), "{stderr}");
}

// A symbolic link under `glyf/` could point at any file, a FIFO would hang
// the build, and a device such as /dev/zero would never end: only regular
// files are read.
#[cfg(unix)]
#[test]
fn a_glyph_file_that_is_not_a_regular_file_is_refused() {
    let build = scratch_dir("format-v2-not-a-file");
    std::fs::write(build.join("outside.json"), br#"{"advanceWidth": 1234}"#).expect("scratch file");
    let dir = v2_dir(&build, "link", r#"{"glyf": {".notdef": "glyf/a.json"}}"#);
    std::os::unix::fs::symlink(build.join("outside.json"), dir.join("glyf/a.json"))
        .expect("symlink");
    let stderr = build_fails(
        &dir,
        &build.join("link.ttf"),
        "a symbolic link must be refused",
    );
    assert!(stderr.contains("not a regular file"), "{stderr}");

    let dir = v2_dir(&build, "fifo", r#"{"glyf": {".notdef": "glyf/a.json"}}"#);
    let status = Command::new("mkfifo")
        .arg(dir.join("glyf/a.json"))
        .status()
        .expect("mkfifo runs");
    assert!(status.success());
    let stderr = build_fails(&dir, &build.join("fifo.ttf"), "a FIFO must be refused");
    assert!(stderr.contains("not a regular file"), "{stderr}");
}
