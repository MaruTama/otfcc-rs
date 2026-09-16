//! Rust-native replacement for the `otfccdll` (cdylib) FFI-boundary check:
//! `scripts/test-dll.py`, `scripts/dll-arch-check.sh`, and the
//! `otfccdll` sections of `scripts/compare-with-golden.sh` and
//! `scripts/run-cycles.sh` that invoke them. Loads the four public
//! ABI functions (`src/ffi/dll.rs`) from the just-built cdylib via
//! `libloading` and calls them the same way an out-of-process consumer
//! would, then compares the built OTF against `tests/golden/dll-test.otf`
//! within a fixed byte tolerance (timestamp fields legitimately vary run
//! to run -- see the comment below).
//!
//! Unlike `test-dll.py`, which loads the cdylib via python3/ctypes and can
//! fail to even start on an Apple Silicon Mac with a Rosetta rustup (a
//! python3-vs-cdylib architecture mismatch -- the whole reason
//! `dll-arch-check.sh` exists), this runs in the SAME process the cdylib
//! itself was just built for: there is no separate interpreter, and so no
//! second architecture to mismatch. The structural fragility
//! `dll-arch-check.sh` works around doesn't apply here, which is why this
//! file has no equivalent skip logic.

mod support;

use std::ffi::{c_char, c_void, OsStr};
use std::path::{Path, PathBuf};
use support::{otfccdump, payload, repo_root, run_ok, scratch_dir};

/// Same derivation `abi.rs`'s own `cdylib_path` uses -- see its doc
/// comment for why Cargo has no `CARGO_BIN_EXE_<name>`-style env var for
/// cdylib artifacts.
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

type BuildFn = unsafe extern "C" fn(u32, *const c_char, u8, bool) -> *mut c_void;
type GetLenFn = unsafe extern "C" fn(*mut c_void) -> usize;
type GetDataFn = unsafe extern "C" fn(*mut c_void) -> *mut u8;
type FreeFn = unsafe extern "C" fn(*mut c_void);

/// Same shape as `cmp -l a b | wc -l`: counts differing bytes over the
/// shared prefix, plus every trailing byte past the shorter file's length
/// (a real length mismatch is a structural difference, not a timestamp
/// tolerance case, so it should count fully against the tolerance rather
/// than being silently ignored).
fn byte_diff_count(a: &[u8], b: &[u8]) -> usize {
    let common = a.len().min(b.len());
    let mismatched = (0..common).filter(|&i| a[i] != b[i]).count();
    mismatched + a.len().abs_diff(b.len())
}

#[test]
fn otfccdll_build_matches_golden_within_timestamp_tolerance() {
    let build = scratch_dir("dll-abi-test-rs");

    // A fresh dump of the same payload compare-with-golden.sh's own
    // otfccdll section feeds test-dll.py, produced independently here
    // rather than reused from golden.rs's own build dir -- separate test
    // binaries can't reliably share a scratch file across processes (see
    // golden.rs's own iosevka-r.json race between two of its test fns).
    let input_json = build.join("Molengo-Regular.json");
    run_ok(
        "Molengo-Regular dump (for otfccdll)",
        &otfccdump(),
        &[payload("Molengo-Regular.ttf").as_os_str(), OsStr::new("-o"), input_json.as_os_str(), OsStr::new("--pretty")],
    );
    let injson = std::fs::read(&input_json).unwrap_or_else(|e| panic!("failed to read {}: {e}", input_json.display()));

    let lib_path = cdylib_path();
    assert!(
        lib_path.exists(),
        "{} not found -- it should have been built automatically as part of this test binary's own build",
        lib_path.display()
    );

    let output: Vec<u8> = unsafe {
        let lib = libloading::Library::new(&lib_path).unwrap_or_else(|e| panic!("failed to dlopen {}: {e}", lib_path.display()));
        let build_fn: libloading::Symbol<BuildFn> = lib.get(b"otfccbuild_json_otf\0").expect("otfccbuild_json_otf should be exported");
        let get_len: libloading::Symbol<GetLenFn> = lib.get(b"otfcc_get_buf_len\0").expect("otfcc_get_buf_len should be exported");
        let get_data: libloading::Symbol<GetDataFn> = lib.get(b"otfcc_get_buf_data\0").expect("otfcc_get_buf_data should be exported");
        let free_fn: libloading::Symbol<FreeFn> = lib.get(b"otfccbuild_free_otfbuf\0").expect("otfccbuild_free_otfbuf should be exported");

        let buf = build_fn(injson.len() as u32, injson.as_ptr() as *const c_char, 0, false);
        assert!(!buf.is_null(), "otfccbuild_json_otf returned NULL");

        let len = get_len(buf);
        let data_ptr = get_data(buf);
        assert!(!data_ptr.is_null(), "otfcc_get_buf_data returned NULL for a non-empty buffer");
        let data = std::slice::from_raw_parts(data_ptr, len).to_vec();

        free_fn(buf);
        data
    };

    let out_path = build.join("dll-rust.otf");
    std::fs::write(&out_path, &output).unwrap_or_else(|e| panic!("failed to write {}: {e}", out_path.display()));

    let golden_path = repo_root().join("tests/golden/dll-test.otf");
    if support::update_golden() {
        std::fs::write(&golden_path, &output).unwrap_or_else(|e| panic!("failed to write {}: {e}", golden_path.display()));
        eprintln!("  updated tests/golden/dll-test.otf");
        return;
    }

    // The DLL API doesn't take --keep-modified-time, so
    // head.created/modified/checkSumAdjustment legitimately vary run to
    // run -- see compare-with-golden.sh's own comment on this same check.
    // 32 bytes is generous enough for every timestamp/checksum field in
    // the format to differ, far too small for any real structural
    // difference.
    const TOLERANCE: usize = 32;
    let golden = std::fs::read(&golden_path).unwrap_or_else(|e| panic!("failed to read {}: {e}", golden_path.display()));

    let diff = byte_diff_count(&golden, &output);
    assert!(
        diff <= TOLERANCE,
        "otfccdll output ({}) differs from golden ({}) in {diff} byte(s) (expected at most {TOLERANCE}, timestamp-only)",
        out_path.display(),
        golden_path.display(),
    );
}
