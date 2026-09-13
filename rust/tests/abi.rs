//! Rust-native replacement for `rust/scripts/check-abi.sh` (see that
//! script's own header comment for the full history/rationale, still kept
//! as reference until Stage F's migration finishes and the script is
//! deleted). Guards the crate's exported C ABI surface: only four symbols
//! are the real public API (`rust/src/ffi/dll.rs`) that any out-of-process
//! consumer can reach through the cdylib, and this asserts the cdylib
//! exports exactly those four plus whatever `scripts/abi-exports.txt`
//! separately records (nothing added without updating the snapshot,
//! nothing recorded silently disappearing).
//!
//! To refresh the snapshot after an intentional change to the exported
//! surface, run this test with `UPDATE_ABI_SNAPSHOT=1` set -- see
//! `cdylib_exports_exactly_the_recorded_abi_surface`'s own body.

use object::Object;
use std::path::{Path, PathBuf};

const REQUIRED: &[&str] = &[
    "otfccbuild_json_otf",
    "otfcc_get_buf_len",
    "otfcc_get_buf_data",
    "otfccbuild_free_otfbuf",
];

const SNAPSHOT_RELATIVE_PATH: &str = "scripts/abi-exports.txt";

/// Mirrors `CARGO_BIN_EXE_<name>`'s trick for binaries, which Cargo has no
/// direct equivalent for cdylib artifacts: derive the profile directory
/// from this test binary's own path (`.../target/<profile>/deps/<test>`)
/// and look for the cdylib as a sibling of `deps/`, not inside it -- Cargo
/// places the primary (unhashed) artifact directly in `<profile>/`, and
/// building it happens automatically as part of building this test
/// binary's own `[lib]` dependency (confirmed: a plain `cargo build`/
/// `cargo test`, no separate step, already produces `libotfcc_rust.dylib`
/// in `target/debug/` alongside the two CLI binaries).
fn cdylib_path() -> PathBuf {
    let exe = std::env::current_exe().expect("current_exe should resolve for a running test binary");
    let profile_dir = exe
        .parent() // deps/
        .and_then(Path::parent) // <profile>/
        .expect("test binary should live under target/<profile>/deps/");
    let name = if cfg!(target_os = "macos") {
        "libotfcc_rust.dylib"
    } else if cfg!(target_os = "windows") {
        "otfcc_rust.dll"
    } else {
        "libotfcc_rust.so"
    };
    profile_dir.join(name)
}

fn manifest_path(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// Extracts the cdylib's exported symbol names, normalized the same way
/// `check-abi.sh`'s own `extract_symbols` was: Rust-mangled names (legacy
/// `_ZN...` / v0 `_R...`) and OS linker artifacts filtered out, sorted and
/// deduped so the result is directly comparable across platforms.
fn exported_symbols(lib_path: &Path) -> Vec<String> {
    let data =
        std::fs::read(lib_path).unwrap_or_else(|e| panic!("failed to read {}: {e}", lib_path.display()));
    let file = object::File::parse(&*data)
        .unwrap_or_else(|e| panic!("failed to parse {} as an object file: {e}", lib_path.display()));
    let mut names: Vec<String> = file
        .exports()
        .expect("reading the export table should not fail for a well-formed cdylib")
        .filter_map(|export| {
            let export = export.expect("export table entries should parse individually");
            let object::read::NameOrOrdinal::Name(name_bytes) = export.name() else {
                return None; // PE-only ordinal export, never applies to a cdylib.
            };
            let raw = String::from_utf8_lossy(name_bytes).into_owned();
            // macOS Mach-O export names keep their leading underscore (C
            // name mangling); `nm -gU`'s own output does too, but
            // check-abi.sh stripped it with `sed 's/^_//'` so both
            // platforms compare on the same footing here.
            let name = raw.strip_prefix('_').map(str::to_owned).unwrap_or(raw);
            if name.starts_with("_ZN") || name.starts_with("_R") {
                return None; // Rust-mangled, never ABI.
            }
            if matches!(
                name.as_str(),
                "_init" | "_fini" | "__bss_start" | "_edata" | "_end" | "_IO_stdin_used"
            ) {
                return None; // ELF linker artifacts, not ours.
            }
            Some(name)
        })
        .collect();
    names.sort();
    names.dedup();
    names
}

#[test]
fn cdylib_exports_exactly_the_recorded_abi_surface() {
    let lib_path = cdylib_path();
    assert!(
        lib_path.exists(),
        "{} not found -- it should have been built automatically as part of this test \
         binary's own build; something is wrong with the build if it's missing",
        lib_path.display()
    );
    let current = exported_symbols(&lib_path);

    for required in REQUIRED {
        assert!(
            current.iter().any(|s| s == required),
            "required public ABI symbol '{required}' is NOT exported by {}",
            lib_path.display()
        );
    }

    let snapshot_path = manifest_path(SNAPSHOT_RELATIVE_PATH);

    if std::env::var_os("UPDATE_ABI_SNAPSHOT").is_some() {
        std::fs::write(&snapshot_path, current.join("\n") + "\n")
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", snapshot_path.display()));
        eprintln!(
            "==> Updated {} ({} symbols)",
            snapshot_path.display(),
            current.len()
        );
        return;
    }

    let snapshot_text = std::fs::read_to_string(&snapshot_path).unwrap_or_else(|e| {
        panic!(
            "failed to read {}: {e} -- create it by re-running this test with \
             UPDATE_ABI_SNAPSHOT=1 set",
            snapshot_path.display()
        )
    });
    let mut recorded: Vec<&str> = snapshot_text.lines().filter(|l| !l.is_empty()).collect();
    recorded.sort();
    recorded.dedup();

    let current_refs: Vec<&str> = current.iter().map(String::as_str).collect();

    let added: Vec<&str> = current_refs.iter().filter(|s| !recorded.contains(s)).copied().collect();
    let removed: Vec<&str> = recorded.iter().filter(|s| !current_refs.contains(s)).copied().collect();

    assert!(
        added.is_empty(),
        "{} symbol(s) newly exported but not recorded in {}: {added:?}\n\
         If making these public is intentional, re-run this test with UPDATE_ABI_SNAPSHOT=1 set.",
        added.len(),
        snapshot_path.display(),
    );
    assert!(
        removed.is_empty(),
        "{} recorded symbol(s) in {} are no longer exported: {removed:?}\n\
         Confirm none of the required ABI functions is among them, then re-run this test \
         with UPDATE_ABI_SNAPSHOT=1 set.",
        removed.len(),
        snapshot_path.display(),
    );
}
