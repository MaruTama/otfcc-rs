//! Rust-native replacement for `scripts/test-lookup-alias.sh` (see
//! that script's own header, and `scripts/make-test-lookup-alias.py`,
//! for the full GSUB lookup-alias double-push bug writeup). Rust-only:
//! the underlying bug exists in the original C source too, and the fix
//! here (`LookupHash.alias: bool`, mirroring `FeatureHash`'s existing
//! field) was a Rust-only design decision, so there is no C binary to
//! byte-compare against. Instead this asserts, against the Rust binaries
//! alone: build determinism, the alias not inflating the lookup count,
//! and dump -> build -> dump round-trip stability.

mod support;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use support::{otfccbuild, otfccdump, payload, repo_root, run_ok, scratch_dir};

/// `true` (and prints why) if `python3` isn't on PATH -- same
/// python3-presence check `golden.rs` makes for its own Python-dependent
/// fixture generators; `make-test-lookup-alias.py` needs it too.
fn skip_if_no_python3(check_name: &str) -> bool {
    let has_python3 = std::process::Command::new("python3")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !has_python3 {
        eprintln!("  (skipping {check_name}: python3 not found)");
    }
    !has_python3
}

fn gsub_lookup_count(json_path: &Path) -> usize {
    let text = std::fs::read_to_string(json_path).unwrap_or_else(|e| panic!("failed to read {}: {e}", json_path.display()));
    let value: serde_json::Value =
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("failed to parse {} as JSON: {e}", json_path.display()));
    value["GSUB"]["lookups"]
        .as_object()
        .unwrap_or_else(|| panic!("{}: GSUB.lookups is not a JSON object", json_path.display()))
        .len()
}

#[test]
fn gsub_lookup_alias_does_not_duplicate_or_destabilize() {
    if skip_if_no_python3("lookup-alias regression") {
        return;
    }

    let build = scratch_dir("lookup-alias-test-rs");
    let src_json = payload("kltf-bugfont1.json");
    let alias_json = build.join("alias.json");

    run_ok(
        "make-test-lookup-alias.py",
        Path::new("python3"),
        &[repo_root().join("scripts/make-test-lookup-alias.py").as_os_str(), src_json.as_os_str(), alias_json.as_os_str()],
    );

    // Baseline lookup count from the un-aliased source: the aliased build
    // must report the same count, not one more -- the whole point of the
    // alias fix is that the alias's `.lookup` push is skipped, so it must
    // resolve to the SAME underlying Lookup.
    let expect_count = gsub_lookup_count(&src_json);

    // 3x build determinism: the same kind of check that caught the gasp
    // uninitialized-Vec bug, which only otfccbuild's own nondeterministic
    // crash pattern revealed.
    let outs: Vec<PathBuf> = (1..=3).map(|i| build.join(format!("out.{i}.otf"))).collect();
    for out in &outs {
        run_ok(
            "otfccbuild (determinism check)",
            &otfccbuild(),
            &[
                alias_json.as_os_str(),
                OsStr::new("-o"),
                out.as_os_str(),
                OsStr::new("--keep-average-char-width"),
                OsStr::new("--keep-modified-time"),
            ],
        );
    }
    let read = |p: &Path| std::fs::read(p).unwrap_or_else(|e| panic!("failed to read {}: {e}", p.display()));
    let (b1, b2, b3) = (read(&outs[0]), read(&outs[1]), read(&outs[2]));
    assert!(b1 == b2 && b1 == b3, "otfccbuild produced different output across repeated runs on the same input");

    // The alias must have resolved to the existing lookup, not a duplicate.
    let dump1 = build.join("dump.1.json");
    run_ok("otfccdump (for lookup count)", &otfccdump(), &[outs[0].as_os_str(), OsStr::new("-o"), dump1.as_os_str(), OsStr::new("--pretty")]);
    let got_count = gsub_lookup_count(&dump1);
    assert_eq!(got_count, expect_count, "lookup count changed by the alias: expected {expect_count} (unchanged), got {got_count}");

    // dump -> build -> dump round-trip stability.
    let out_2nd = build.join("out.2nd.otf");
    run_ok(
        "otfccbuild (round-trip)",
        &otfccbuild(),
        &[
            dump1.as_os_str(),
            OsStr::new("-o"),
            out_2nd.as_os_str(),
            OsStr::new("--keep-average-char-width"),
            OsStr::new("--keep-modified-time"),
        ],
    );
    let dump2 = build.join("dump.2.json");
    run_ok("otfccdump (round-trip)", &otfccdump(), &[out_2nd.as_os_str(), OsStr::new("-o"), dump2.as_os_str(), OsStr::new("--pretty")]);
    let (d1, d2) = (read(&dump1), read(&dump2));
    assert_eq!(d1, d2, "dump -> build -> dump is not stable for the lookup-alias payload");
}
