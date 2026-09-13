//! Rust-native replacement for `rust/scripts/compare-log-output.sh` (see
//! that script's own header for the full history/rationale). Compares
//! `otfccdump`/`otfccbuild`'s stderr (everything written through the
//! Logger) against the frozen fixtures in `tests/golden/log/`,
//! byte-for-byte modulo elapsed-time numbers and the scratch directory's
//! own path.
//!
//! A single `#[test]` running every case in order, not six independent
//! ones: `build-verbose`/`build-quiet` consume the `iosevka-r.json` that
//! `dump-verbose` produces, a real ordering dependency the original shell
//! script's sequential execution relied on -- `cargo test`'s default
//! parallel harness would race that if these were separate test fns.

mod support;

use regex::Regex;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use support::{otfccbuild, otfccdump, payload, relative_arg, repo_root, scratch_dir};

fn golden_log_dir() -> PathBuf {
    repo_root().join("tests/golden/log")
}

/// Same two substitutions `compare-log-output.sh`'s own `normalize()`
/// made: the golden fixtures already have both applied (including
/// blanking out the scratch-directory path, since the shell script and
/// this one use differently-named scratch dirs), so freshly captured
/// output needs the same normalization to compare equal.
fn normalize(raw: &str) -> String {
    let step_time = Regex::new(r"Step time = [0-9.eE+-]+s\.").unwrap();
    let scratch = Regex::new(r"build/[A-Za-z0-9_-]+/").unwrap();
    let normalized = step_time.replace_all(raw, "Step time = <T>s.");
    scratch.replace_all(&normalized, "build/<SCRATCH>/").into_owned()
}

/// Runs `program` with `args`, capturing stderr (ignoring exit status --
/// `dump_missing_file` is expected to fail, and this check is only about
/// what it logs), and returns the normalized text.
fn captured_normalized_stderr(program: &std::path::Path, args: &[&std::ffi::OsStr]) -> String {
    // Args relativized against repo_root() (current_dir below), same
    // reasoning as support::run_ok's own doc comment: otfccdump/otfccbuild
    // log whatever path string they're given verbatim, and the golden log
    // fixtures were captured with relative paths.
    let args: Vec<std::ffi::OsString> = args.iter().map(|a| relative_arg(std::path::Path::new(a))).collect();
    let output = Command::new(program)
        .args(&args)
        .current_dir(repo_root())
        .stdout(Stdio::null())
        .output()
        .unwrap_or_else(|e| panic!("failed to spawn {}: {e}", program.display()));
    normalize(&String::from_utf8_lossy(&output.stderr))
}

fn compare_log(label: &str, got_normalized: &str, errors: &mut Vec<String>) {
    let golden_path = golden_log_dir().join(format!("{label}.log"));
    let Ok(want) = std::fs::read_to_string(&golden_path) else {
        errors.push(format!("{label}: no golden fixture at {}", golden_path.display()));
        return;
    };
    if got_normalized != want {
        let mut msg = format!("{label}: log output differs from golden\n");
        for diff in diff_lines(&want, got_normalized).into_iter().take(40) {
            msg.push_str(&diff);
            msg.push('\n');
        }
        errors.push(msg);
    }
}

/// A minimal unified-diff-flavored line list (want vs. got), good enough
/// for a failure message to point at what changed without pulling in a
/// diff crate for a test-only comparison.
fn diff_lines(want: &str, got: &str) -> Vec<String> {
    let want_lines: Vec<&str> = want.lines().collect();
    let got_lines: Vec<&str> = got.lines().collect();
    let mut out = Vec::new();
    for i in 0..want_lines.len().max(got_lines.len()) {
        match (want_lines.get(i), got_lines.get(i)) {
            (Some(w), Some(g)) if w == g => {}
            (Some(w), Some(g)) => {
                out.push(format!("-{w}"));
                out.push(format!("+{g}"));
            }
            (Some(w), None) => out.push(format!("-{w}")),
            (None, Some(g)) => out.push(format!("+{g}")),
            (None, None) => {}
        }
    }
    out
}

#[test]
fn log_output_matches_golden() {
    let build = scratch_dir("compare-log-output-rs");
    let mut errors = Vec::new();

    // --verbose is the interesting case: it's the only flag that exercises
    // indent/dedent nesting (LOG_VL_PROGRESS-level "Begin"/"Finish" pairs)
    // and the continuation-guide rendering in logger_log_sds, i.e.
    // everything the Logger.indents: Vec<Vec<u8>> retype could get wrong.
    let iosevka_json = build.join("iosevka-r.json");
    let dump_verbose = captured_normalized_stderr(
        &otfccdump(),
        &[
            payload("iosevka-r.ttf").as_os_str(),
            std::ffi::OsStr::new("-o"),
            iosevka_json.as_os_str(),
            std::ffi::OsStr::new("--pretty"),
            std::ffi::OsStr::new("--verbose"),
        ],
    );
    compare_log("dump-verbose", &dump_verbose, &mut errors);

    // --quiet raises the verbosity floor, so this exercises the OTHER half
    // of set_verbosity/verbosity_limit filtering: confirms nothing at all
    // reaches the target when it shouldn't.
    let dump_quiet = captured_normalized_stderr(
        &otfccdump(),
        &[
            payload("iosevka-r.ttf").as_os_str(),
            std::ffi::OsStr::new("-o"),
            build.join("iosevka-r-q.json").as_os_str(),
            std::ffi::OsStr::new("--pretty"),
            std::ffi::OsStr::new("--quiet"),
        ],
    );
    compare_log("dump-quiet", &dump_quiet, &mut errors);

    // A CFF (.otf) payload takes a different code path (libcff's own
    // logging in addition to the shared reader/writer log calls), so it
    // needs its own check.
    let dump_cff_verbose = captured_normalized_stderr(
        &otfccdump(),
        &[
            payload("KRName-Regular.otf").as_os_str(),
            std::ffi::OsStr::new("-o"),
            build.join("KRName-Regular.json").as_os_str(),
            std::ffi::OsStr::new("--pretty"),
            std::ffi::OsStr::new("--verbose"),
        ],
    );
    compare_log("dump-cff-verbose", &dump_cff_verbose, &mut errors);

    // Must run after dump-verbose above: consumes the iosevka-r.json it
    // just produced.
    let build_verbose = captured_normalized_stderr(
        &otfccbuild(),
        &[
            iosevka_json.as_os_str(),
            std::ffi::OsStr::new("-o"),
            build.join("iosevka-r.ttf").as_os_str(),
            std::ffi::OsStr::new("--keep-average-char-width"),
            std::ffi::OsStr::new("--keep-modified-time"),
            std::ffi::OsStr::new("--verbose"),
        ],
    );
    compare_log("build-verbose", &build_verbose, &mut errors);

    let build_quiet = captured_normalized_stderr(
        &otfccbuild(),
        &[
            iosevka_json.as_os_str(),
            std::ffi::OsStr::new("-o"),
            build.join("iosevka-r-q.ttf").as_os_str(),
            std::ffi::OsStr::new("--keep-average-char-width"),
            std::ffi::OsStr::new("--keep-modified-time"),
            std::ffi::OsStr::new("--quiet"),
        ],
    );
    compare_log("build-quiet", &build_quiet, &mut errors);

    // The LOG_VL_CRITICAL / LoggerType::Error path (logger_log_sds's
    // OTFCC_LOGGER_TYPE_NAMES prefix) is otherwise never reached by any
    // payload above, since they all succeed.
    let dump_missing_file = captured_normalized_stderr(
        &otfccdump(),
        &[
            build.join("does-not-exist.ttf").as_os_str(),
            std::ffi::OsStr::new("-o"),
            build.join("does-not-exist.json").as_os_str(),
            std::ffi::OsStr::new("--verbose"),
        ],
    );
    compare_log("dump-missing-file", &dump_missing_file, &mut errors);

    assert!(errors.is_empty(), "{} log comparison(s) failed:\n{}", errors.len(), errors.join("\n"));
}
