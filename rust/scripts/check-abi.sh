#!/usr/bin/env bash
# Guards the crate's exported C ABI surface.
#
# WHY THIS EXISTS
#
# Only FOUR symbols are actually part of otfcc's public C ABI — the otfccdll
# API that rust/scripts/test-dll.py drives through ctypes, and the only entry
# points any out-of-process consumer can reach:
#
#     otfccbuild_json_otf  otfcc_get_buf_len  otfcc_get_buf_data
#     otfccbuild_free_otfbuf
#
# Everything else the cdylib currently exports (see abi-exports.txt) is there
# only because c2rust marked every non-`static` C function `#[no_mangle]`.
# Those are internal cross-module calls, not API. Nothing links against them:
# compare-with-c.sh and test-dll.py run the C and Rust implementations as
# SEPARATE processes/libraries and compare their OUTPUT — no C code and Rust
# code ever share a struct in one process.
#
# That distinction is what licenses the Phase 3 refactor to change internal
# struct layouts, drop `#[repr(C)]`, and move to Vec/String/Box: byte-identical
# output is the real invariant, not ABI-compatible internals. This script keeps
# that reasoning honest by making the ABI surface an explicit, reviewed artifact
# instead of an assumption:
#
#   - the four required symbols must always be exported (hard failure);
#   - no NEW symbol may appear without being recorded (catches an internal
#     helper accidentally becoming public);
#   - a symbol that DISAPPEARS is also a failure until the snapshot is
#     refreshed with `--update`, so every batch of internalized symbols shows
#     up as a reviewable diff in abi-exports.txt rather than passing silently.
#
# Usage:
#   ./rust/scripts/check-abi.sh            # verify (needs a release build)
#   ./rust/scripts/check-abi.sh --update   # refresh the recorded snapshot
set -euo pipefail
cd "$(dirname "$0")/../.."

SNAPSHOT=rust/scripts/abi-exports.txt

REQUIRED=(
	otfccbuild_json_otf
	otfcc_get_buf_len
	otfcc_get_buf_data
	otfccbuild_free_otfbuf
)

case "$(uname -s)" in
Darwin) LIB=rust/target/release/libotfcc_rust.dylib ;;
*) LIB=rust/target/release/libotfcc_rust.so ;;
esac

if [ ! -f "${LIB}" ]; then
	echo "ERROR: ${LIB} not found; run ./rust/scripts/build-crate.sh first." >&2
	exit 1
fi

# Extract exported symbol names, normalized so the list is comparable across
# platforms:
#   - macOS `nm -gU` prints external defined symbols with a leading underscore
#     (Mach-O's C name mangling); Linux `nm -D --defined-only` prints dynamic
#     defined symbols without one.
#   - Rust-mangled symbols (_ZN.../_R...) are never ABI; a cdylib normally
#     hides them, but filter defensively.
#   - _init/_fini/__bss_start/_edata/_end/_IO_stdin_used are ELF linker
#     artifacts, not ours.
extract_symbols() {
	if [ "$(uname -s)" = "Darwin" ]; then
		nm -gU "${LIB}" | awk '{print $NF}' | sed 's/^_//'
	else
		nm -D --defined-only "${LIB}" | awk '{print $NF}'
	fi |
		grep -Ev '^(_ZN|_R)' |
		grep -Ev '^(_init|_fini|__bss_start|_edata|_end|_IO_stdin_used)$' |
		LC_ALL=C sort -u
}

CURRENT="$(extract_symbols)"

if [ "${1:-}" = "--update" ]; then
	printf '%s\n' "${CURRENT}" >"${SNAPSHOT}"
	echo "==> Updated ${SNAPSHOT} ($(printf '%s\n' "${CURRENT}" | wc -l | tr -d ' ') symbols)"
	exit 0
fi

status=0

# 1. The actual public ABI must be intact.
for sym in "${REQUIRED[@]}"; do
	if ! printf '%s\n' "${CURRENT}" | grep -qx "${sym}"; then
		echo "FAIL: required public ABI symbol '${sym}' is NOT exported by ${LIB}" >&2
		status=1
	fi
done

# 2. The rest of the surface must match what's recorded.
if [ ! -f "${SNAPSHOT}" ]; then
	echo "ERROR: ${SNAPSHOT} not found; create it with: $0 --update" >&2
	exit 1
fi

added="$(comm -23 <(printf '%s\n' "${CURRENT}") <(LC_ALL=C sort -u "${SNAPSHOT}"))"
removed="$(comm -13 <(printf '%s\n' "${CURRENT}") <(LC_ALL=C sort -u "${SNAPSHOT}"))"

if [ -n "${added}" ]; then
	echo "FAIL: $(printf '%s\n' "${added}" | wc -l | tr -d ' ') symbol(s) newly exported but not recorded in ${SNAPSHOT}:" >&2
	printf '  + %s\n' ${added} >&2
	echo "  If making these public is intentional, run: $0 --update" >&2
	status=1
fi

if [ -n "${removed}" ]; then
	echo "FAIL: $(printf '%s\n' "${removed}" | wc -l | tr -d ' ') recorded symbol(s) are no longer exported:" >&2
	printf '  - %s\n' ${removed} >&2
	echo "  This is expected while internalizing symbols during Phase 3 — confirm none" >&2
	echo "  of the four public ABI functions is among them, then run: $0 --update" >&2
	status=1
fi

if [ "${status}" -eq 0 ]; then
	echo "==> ABI OK: 4 required public symbols present; $(printf '%s\n' "${CURRENT}" | wc -l | tr -d ' ') exports match ${SNAPSHOT}"
fi

exit "${status}"
