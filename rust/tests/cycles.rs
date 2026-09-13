//! Rust-native replacement for `rust/scripts/run-cycles.sh` (dump/build
//! cycle stability) and `rust/scripts/compare-roundtrips.js` +
//! `tests/ttf-roundtrip-test.js` (tolerance-based recursive JSON
//! comparison between a payload's first and second dump). See those
//! files' own headers for the full history/rationale.
//!
//! The `otfccdll` (cdylib, ctypes) portion of `run-cycles.sh` is **not**
//! included here -- see the plan doc's Stage F: that's Phase 5, needing
//! `libloading`.
//!
//! One `#[test]`, not several: every cycle must finish writing its
//! `.3.json`/`.5.json` outputs before the roundtrip comparison reads them,
//! the same ordering the two separate script invocations
//! (`run-cycles.sh` then `compare-roundtrips.js`) relied on.

mod support;

use serde_json::Value;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

fn build_dir() -> PathBuf {
    scratch_dir("rust-test-rs")
}

/// `otfccdump` -> `otfccbuild` -> `otfccdump` -> `otfccbuild` -> `otfccdump`,
/// starting from a binary font (`.ttf`/`.otf`) -- same 5-stage shape as
/// run-cycles.sh's `run_ttf_cycle`.
fn run_ttf_cycle(build: &Path, name: &str, ext: &str, input: &Path) {
    let p1 = build.join(format!("{name}.1.json"));
    let p2 = build.join(format!("{name}.2.{ext}"));
    let p3 = build.join(format!("{name}.3.json"));
    let p4 = build.join(format!("{name}.4.{ext}"));
    let p5 = build.join(format!("{name}.5.json"));

    run_ok(&format!("{name} dump 1"), &otfccdump(), &[input.as_os_str(), OsStr::new("-o"), p1.as_os_str(), OsStr::new("--pretty")]);
    run_ok(
        &format!("{name} build 2"),
        &otfccbuild(),
        &[
            p1.as_os_str(),
            OsStr::new("-o"),
            p2.as_os_str(),
            OsStr::new("--keep-average-char-width"),
            OsStr::new("--keep-modified-time"),
        ],
    );
    run_ok(&format!("{name} dump 3"), &otfccdump(), &[p2.as_os_str(), OsStr::new("-o"), p3.as_os_str(), OsStr::new("--pretty")]);
    run_ok(
        &format!("{name} build 4"),
        &otfccbuild(),
        &[
            p3.as_os_str(),
            OsStr::new("-o"),
            p4.as_os_str(),
            OsStr::new("--keep-average-char-width"),
            OsStr::new("--keep-modified-time"),
        ],
    );
    run_ok(&format!("{name} dump 5"), &otfccdump(), &[p4.as_os_str(), OsStr::new("-o"), p5.as_os_str(), OsStr::new("--pretty")]);
}

/// `otfccbuild` -> `otfccdump` -> `otfccbuild` -> `otfccdump`, starting
/// from a JSON payload -- run-cycles.sh's `run_fj_cycle` ("from JSON"),
/// one stage shorter than `run_ttf_cycle` since there's no initial binary
/// to dump. Output files are named `fj-{name}.*`.
fn run_fj_cycle(build: &Path, name: &str, ext: &str, input: &Path) {
    let out_name = format!("fj-{name}");
    let p2 = build.join(format!("{out_name}.2.{ext}"));
    let p3 = build.join(format!("{out_name}.3.json"));
    let p4 = build.join(format!("{out_name}.4.{ext}"));
    let p5 = build.join(format!("{out_name}.5.json"));

    run_ok(
        &format!("{out_name} build 2"),
        &otfccbuild(),
        &[
            input.as_os_str(),
            OsStr::new("-o"),
            p2.as_os_str(),
            OsStr::new("--keep-average-char-width"),
            OsStr::new("--keep-modified-time"),
        ],
    );
    run_ok(&format!("{out_name} dump 3"), &otfccdump(), &[p2.as_os_str(), OsStr::new("-o"), p3.as_os_str(), OsStr::new("--pretty")]);
    run_ok(
        &format!("{out_name} build 4"),
        &otfccbuild(),
        &[
            p3.as_os_str(),
            OsStr::new("-o"),
            p4.as_os_str(),
            OsStr::new("--keep-average-char-width"),
            OsStr::new("--keep-modified-time"),
        ],
    );
    run_ok(&format!("{out_name} dump 5"), &otfccdump(), &[p4.as_os_str(), OsStr::new("-o"), p5.as_os_str(), OsStr::new("--pretty")]);
}

const ROUNDING_ERROR: f64 = 0.001;

/// Port of `tests/ttf-roundtrip-test.js`'s `deepEqual`/`objEquiv`:
/// recursive structural equality, tolerant of floating-point rounding
/// drift in numbers (`%g`-formatted numeric output can shift by a unit in
/// the last place between two dump/build cycles without indicating a real
/// regression). `path` is a breadcrumb used to build a useful failure
/// message; `first_diff` records only the first mismatch found, mirroring
/// the JS version's `shownDiff`-guarded single diff dump.
fn deep_equal(a: &Value, b: &Value, path: &str, first_diff: &mut Option<String>) -> bool {
    match (a, b) {
        (Value::Number(na), Value::Number(nb)) => {
            let fa = na.as_f64().unwrap();
            let fb = nb.as_f64().unwrap();
            let pass = (fa - fb).abs() < ROUNDING_ERROR;
            if !pass && first_diff.is_none() {
                *first_diff = Some(format!("{path}: number mismatch {fa} <> {fb} (difference = {})", (fa - fb).abs()));
            }
            pass
        }
        (Value::Object(oa), Value::Object(ob)) => {
            let mut ka: Vec<&String> = oa.keys().collect();
            let mut kb: Vec<&String> = ob.keys().collect();
            ka.sort();
            kb.sort();
            if ka != kb {
                if first_diff.is_none() {
                    *first_diff = Some(format!("{path}: object key sets differ: {ka:?} <> {kb:?}"));
                }
                return false;
            }
            let mut ok = true;
            for k in ka {
                if !deep_equal(&oa[k], &ob[k], &format!("{path}.{k}"), first_diff) {
                    ok = false;
                }
            }
            ok
        }
        (Value::Array(aa), Value::Array(ab)) => {
            if aa.len() != ab.len() {
                if first_diff.is_none() {
                    *first_diff = Some(format!("{path}: array length differs ({} <> {})", aa.len(), ab.len()));
                }
                return false;
            }
            let mut ok = true;
            for (i, (va, vb)) in aa.iter().zip(ab.iter()).enumerate() {
                if !deep_equal(va, vb, &format!("{path}[{i}]"), first_diff) {
                    ok = false;
                }
            }
            ok
        }
        _ => {
            let pass = a == b;
            if !pass && first_diff.is_none() {
                *first_diff = Some(format!("{path}: value mismatch {a} <> {b}"));
            }
            pass
        }
    }
}

/// Reads `path` as JSON, decoding lossily (invalid UTF-8 bytes become
/// U+FFFD) rather than erroring on non-UTF-8 content: `fs.readFileSync(p,
/// 'utf-8')` in `tests/ttf-roundtrip-test.js` does the same lossy
/// decoding, which matters for at least one payload here
/// (Reinebow-SVGinOT's copyright string carries a raw non-UTF-8 byte that
/// otfccdump preserves byte-for-byte in its JSON output by design -- see
/// the project's other non-UTF-8-name handling). Both sides of every
/// comparison go through the same lossy decoding, so this doesn't change
/// what the check catches.
fn read_json(path: &Path) -> Value {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    let text = String::from_utf8_lossy(&bytes);
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("failed to parse {} as JSON: {e}", path.display()))
}

/// Compares `{base}.5.json` (dump of build of dump of build of the
/// original) against `{base}.3.json` (dump of build of the original) --
/// same pairing `compare-roundtrips.js` checked, confirming the dump/build
/// pipeline reaches a stable fixed point after one round trip rather than
/// drifting further with each cycle.
fn compare_roundtrip(build: &Path, label: &str, base: &str, errors: &mut Vec<String>) {
    let five = build.join(format!("{base}.5.json"));
    let three = build.join(format!("{base}.3.json"));
    let v5 = read_json(&five);
    let v3 = read_json(&three);

    let mut first_diff = None;
    if !deep_equal(&v5, &v3, "$", &mut first_diff) {
        errors.push(format!(
            "{label}: roundtrip mismatch between {} and {}: {}",
            five.display(),
            three.display(),
            first_diff.unwrap_or_default()
        ));
    }
}

#[test]
fn dump_build_cycles_are_stable() {
    let build = build_dir();

    const TTF_PAYLOADS: &[&str] =
        &["NotoNastaliqUrdu-Regular", "iosevka-r", "BungeeColor-Regular_colr_Windows", "Reinebow-SVGinOT", "vtt", "Molengo-Regular"];
    // Cormorant-Medium and WorkSans-Regular.otf are excluded (not present
    // here at all): they crash with a stack overflow in BOTH the C and
    // Rust otfccdump on arm64, a pre-existing bug in the C CFF
    // interpreter predating the Rust translation -- see run-cycles.sh's
    // own comment.
    const CFF_PAYLOADS: &[&str] = &["KRName-Regular"];
    // cid-fdselect-test exercises CffFdSelect::Format3 (glyphs assigned
    // across 3 font dicts with 5 ranges), the only payload in this suite
    // that does -- see run-cycles.sh's own comment. It still runs through
    // the cycle below (crash-freedom check), but compare-roundtrips.js
    // never round-trip-compared it, only WorkSans-Regular and
    // kltf-bugfont1, so this preserves that same omission rather than
    // introducing a new check the original scripts never made.
    const CFF_FJ_PAYLOADS: &[&str] = &["WorkSans-Regular", "kltf-bugfont1", "cid-fdselect-test"];
    const CFF_FJ_ROUNDTRIP_CHECKED: &[&str] = &["WorkSans-Regular", "kltf-bugfont1"];

    for name in TTF_PAYLOADS {
        run_ttf_cycle(&build, name, "ttf", &payload(&format!("{name}.ttf")));
    }
    for name in CFF_PAYLOADS {
        run_ttf_cycle(&build, name, "otf", &payload(&format!("{name}.otf")));
    }
    for name in CFF_FJ_PAYLOADS {
        run_fj_cycle(&build, name, "otf", &payload(&format!("{name}.json")));
    }
    // A frozen fixture (tests/payload/gvar-test.ttf), not regenerated via
    // fontTools here -- see golden.rs's own gvar-test entry for why.
    run_ttf_cycle(&build, "gvar-test", "ttf", &payload("gvar-test.ttf"));

    let mut roundtrip_checked: Vec<&str> = Vec::new();
    roundtrip_checked.extend_from_slice(TTF_PAYLOADS);
    roundtrip_checked.extend_from_slice(CFF_PAYLOADS);
    roundtrip_checked.push("gvar-test");

    let mut errors = Vec::new();
    for name in &roundtrip_checked {
        compare_roundtrip(&build, name, name, &mut errors);
    }
    for name in CFF_FJ_ROUNDTRIP_CHECKED {
        compare_roundtrip(&build, &format!("{name} (fj)"), &format!("fj-{name}"), &mut errors);
    }

    assert!(errors.is_empty(), "{} payload(s) failed the round-trip check:\n{}", errors.len(), errors.join("\n"));
}

/// Exercises `deep_equal` directly against synthetic values: the
/// integration test above regenerates its fixtures from a deterministic
/// pipeline every run, so file-level corruption of `.3.json`/`.5.json`
/// gets silently overwritten before the comparison ever sees it -- this is
/// the only way to confirm the tolerance/mismatch logic itself actually
/// distinguishes a real difference from rounding drift.
#[test]
fn deep_equal_tolerates_rounding_drift_but_catches_real_differences() {
    let mut diff = None;
    assert!(deep_equal(&serde_json::json!(1.0), &serde_json::json!(1.0009), "$", &mut diff));
    assert!(diff.is_none());

    let mut diff = None;
    assert!(!deep_equal(&serde_json::json!(1.0), &serde_json::json!(1.01), "$", &mut diff));
    assert!(diff.unwrap().contains("number mismatch"));

    let mut diff = None;
    let a = serde_json::json!({"a": 1, "b": [1, 2, 3]});
    let b = serde_json::json!({"a": 1, "b": [1, 2, 4]});
    assert!(!deep_equal(&a, &b, "$", &mut diff));
    assert_eq!(diff.unwrap(), "$.b[2]: number mismatch 3 <> 4 (difference = 1)");

    let mut diff = None;
    let c = serde_json::json!({"a": 1});
    let d = serde_json::json!({"a": 1, "extra": 2});
    assert!(!deep_equal(&c, &d, "$", &mut diff));
    assert!(diff.unwrap().contains("key sets differ"));
}
