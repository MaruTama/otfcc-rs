//! Shared helpers for the `cargo test`-native replacements of
//! `rust/scripts/*.sh` (Stage F in the plan doc). Not a test file itself
//! (the `support/` subdirectory convention keeps Cargo from treating it as
//! one) -- each migrated test file does `mod support;` and uses these.
//!
//! Each `tests/*.rs` file is compiled as its own independent binary
//! (standard Cargo integration-test behavior), so this module is
//! recompiled once per caller and only some of its items are used in any
//! one of them -- `dead_code` would otherwise fire in whichever binary
//! doesn't happen to use a given helper.
#![allow(dead_code)]

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// The repository root (one level up from `rust/`, which is
/// `CARGO_MANIFEST_DIR` for every test in this crate).
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR (rust/) should have a parent (the repo root)")
        .to_path_buf()
}

pub fn payload(name: &str) -> PathBuf {
    repo_root().join("tests/payload").join(name)
}

/// A scratch directory under `build/`, created on first use. Each caller
/// passes its own subdirectory name so parallel test binaries (golden.rs,
/// log_output.rs, ...) never share one, the same separation
/// `compare-with-golden.sh`/`compare-log-output.sh` kept via different
/// `BUILD=` directories.
pub fn scratch_dir(subdir: &str) -> PathBuf {
    let dir = repo_root().join("build").join(subdir);
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("failed to create {}: {e}", dir.display()));
    dir
}

/// Path to the just-built `otfccdump`/`otfccbuild` binaries -- Cargo sets
/// `CARGO_BIN_EXE_<name>` automatically for every `[[bin]]` target in the
/// same package, building it first if needed (no separate `build-crate.sh`
/// step, unlike the shell scripts this replaces).
pub fn otfccdump() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_otfccdump"))
}
pub fn otfccbuild() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_otfccbuild"))
}

pub fn sha256_of(path: &Path) -> String {
    let data = std::fs::read(path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    let digest = Sha256::digest(&data);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Parses `tests/golden/checksums.sha256` (`sha256sum`-compatible format:
/// `<64 hex chars><two spaces><label>` per line) into a label -> hash map.
pub fn golden_checksums() -> HashMap<String, String> {
    let checksums_path = repo_root().join("tests/golden/checksums.sha256");
    let text = std::fs::read_to_string(&checksums_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", checksums_path.display()));
    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (hash, label) = line
                .split_once("  ")
                .unwrap_or_else(|| panic!("malformed line in {}: {line:?}", checksums_path.display()));
            (label.to_string(), hash.to_string())
        })
        .collect()
}

/// Hashes `file` and compares against the golden checksum recorded for
/// `label`, returning an `Err` description instead of panicking so callers
/// that check many payloads in one test function can accumulate every
/// failure (matching `compare-with-golden.sh`'s own `fail=1`-and-continue
/// shape) rather than stopping at the first one.
pub fn check_against_golden(file: &Path, label: &str, checksums: &HashMap<String, String>) -> Result<(), String> {
    let Some(want) = checksums.get(label) else {
        return Err(format!(
            "no golden checksum recorded for '{label}' -- run rust/scripts/generate-golden.sh"
        ));
    };
    let got = sha256_of(file);
    if &got == want {
        Ok(())
    } else {
        Err(format!("{label}: does not match golden checksum (file: {})", file.display()))
    }
}

/// Every `Command` this module builds runs with `repo_root()` as its
/// current directory and args relativized against it first (via
/// `relative_arg`) -- matching exactly what the shell scripts this
/// replaces did (`cd "$(dirname "$0")/../.."` up front, then relative
/// paths throughout). Otherwise `otfccdump`/`otfccbuild` would log
/// whatever path string they were given verbatim (e.g. `Read SFNT : From
/// file <path>`), and an absolute path would never match the golden log
/// fixtures, which were captured the same relative-path way.
pub fn relative_arg(path: &Path) -> std::ffi::OsString {
    path.strip_prefix(repo_root()).unwrap_or(path).as_os_str().to_owned()
}

/// Runs `program` (kept absolute -- a relative *executable* path resolves
/// against the parent process's cwd on Unix, not the child's `current_dir`,
/// so relativizing it the way argument paths are below would be wrong)
/// with `args` (each relativized via `relative_arg`), panicking with a
/// clear message (including the label, for use inside a loop over many
/// payloads) if it doesn't exit 0 -- the same "FAIL ...: exited non-zero"
/// check every `compare_payload`/`synth_payload` call in the shell scripts
/// made before hashing output.
pub fn run_ok(label: &str, program: &Path, args: &[&std::ffi::OsStr]) {
    let args: Vec<std::ffi::OsString> = args.iter().map(|a| relative_arg(Path::new(a))).collect();
    let status = std::process::Command::new(program)
        .args(&args)
        .current_dir(repo_root())
        .status()
        .unwrap_or_else(|e| panic!("{label}: failed to spawn {}: {e}", program.display()));
    assert!(status.success(), "{label}: {} exited non-zero ({status})", program.display());
}
