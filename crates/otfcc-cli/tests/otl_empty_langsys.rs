//! A language system with no features is valid, and differs from having no
//! language system: the language then uses no feature at all, instead of
//! falling back to the script's default. otfcc dumped one as
//! `"features": []` but dropped it when building, so the language took the
//! default's features (Source Serif 4 crafted with an empty Turkish system
//! formed the `fi` ligature after a build). One that is empty only because
//! its features do not exist is still dropped.

mod support;

use std::ffi::OsStr;
use support::{otfccbuild, otfccdump, payload, run_ok, scratch_dir};

#[test]
fn a_language_system_with_no_features_is_kept() {
    let build = scratch_dir("otl-empty-langsys");
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
    let languages = font["GSUB"]["languages"].as_object_mut().unwrap();
    languages.insert("latn_TRK ".into(), serde_json::json!({ "features": [] }));
    languages.insert(
        "latn_AZE ".into(),
        serde_json::json!({ "features": ["no-such-feature"] }),
    );
    let input = build.join("empty.json");
    std::fs::write(&input, serde_json::to_vec(&font).unwrap()).unwrap();
    let ttf = build.join("empty.ttf");
    run_ok(
        "build",
        &otfccbuild(),
        &[input.as_os_str(), OsStr::new("-o"), ttf.as_os_str()],
    );
    let dumped = build.join("empty.dump.json");
    run_ok(
        "dump",
        &otfccdump(),
        &[ttf.as_os_str(), OsStr::new("-o"), dumped.as_os_str()],
    );

    let dump: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&dumped).unwrap()).unwrap();
    let languages = dump["GSUB"]["languages"].as_object().unwrap();
    assert_eq!(
        languages.get("latn_TRK "),
        Some(&serde_json::json!({ "features": [] }))
    );
    assert!(languages.get("latn_AZE ").is_none());
}
