//! Numeric CLI arguments are read with `str::parse`, so a value that is not
//! a number is an error rather than being read as 0 or as its leading digits.

mod support;

use std::process::Command;
use support::{otfccbuild, otfccdump, payload, scratch_dir};

#[test]
fn a_non_numeric_ttc_index_is_rejected() {
    let out = Command::new(otfccdump())
        .args(["--ttc-index", "1x"])
        .arg(payload("Molengo-Regular.ttf"))
        .output()
        .expect("otfccdump runs");
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("invalid subfont index '1x'"));
    assert!(out.stdout.is_empty());
}

#[test]
fn a_non_numeric_optimization_level_is_rejected() {
    let out_path = scratch_dir("cli-args-rs").join("unused.otf");
    let out = Command::new(otfccbuild())
        .args(["-Ox", "-o"])
        .arg(&out_path)
        .arg(payload("iosevka-r.json"))
        .output()
        .expect("otfccbuild runs");
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("invalid optimization level 'x'"));
    assert!(!out_path.exists());
}
