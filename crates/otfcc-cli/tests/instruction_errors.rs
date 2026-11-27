//! Instructions in `fpgm` or `prep` that cannot be parsed left the table
//! empty without a word, where a glyph's are warned about: one mistyped
//! value lost the whole program silently. Both are reported now.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn unparsable_fpgm_instructions_are_reported() {
    let build = scratch_dir("instruction-errors");
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
    // 300 does not fit the byte a PUSHB pushes.
    font["fpgm"] = serde_json::json!(["PUSHB_2", "5 300", "DUP"]);
    let input = build.join("fpgm.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("fpgm.ttf");
    let built = std::process::Command::new(otfccbuild())
        .args([input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()])
        .output()
        .expect("otfccbuild runs");
    assert!(built.status.success(), "otfccbuild failed");
    let stderr = String::from_utf8_lossy(&built.stderr);
    assert!(
        stderr.contains("TrueType instructions parse error") && stderr.contains("in fpgm"),
        "{stderr}"
    );
}
