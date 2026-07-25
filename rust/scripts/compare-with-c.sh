#!/usr/bin/env bash
# Builds the C toolchain natively (Linux or macOS — the premake/ninja
# toolchain binaries and the quick.make target are selected from `uname`) and
# compares its output against the already-built Rust crate
# (rust/target/release/), byte-for-byte, on the same canonical input JSON for
# each payload.
#
# Must run AFTER the Rust crate has been built (cargo build --release) and
# on the SAME architecture as that build, so both binaries' outputs are
# directly comparable without any cross-arch ambiguity.
#
# Invoke as: ./rust/scripts/compare-with-c.sh
set -euo pipefail
cd "$(dirname "$0")/../.."

RUST_BIN=rust/target/release
if [ ! -x "${RUST_BIN}/otfccdump" ] || [ ! -x "${RUST_BIN}/otfccbuild" ]; then
	echo "ERROR: ${RUST_BIN}/{otfccdump,otfccbuild} not found; build the Rust crate first." >&2
	exit 1
fi

echo "==> Building the C toolchain (native)"
# Unlike gen-compile-commands.sh (which invokes ninja directly after its own
# `cd`), this uses quick.make's own `{linux,macosx}-release-x64` target, which
# internally does `cd build/ninja && ../../$(BD_NINJA) ...` — BD_NINJA must
# stay a repo-root-relative path for that "../../" prefix to resolve.
#
# The premake/ninja toolchain binaries and the quick.make target are both
# per-OS. Getting this wrong is not a subtle failure but it *looks* like one:
# `build/` and `bin/` are shared (the Linux verification container bind-mounts
# the repo at its host path), so a stale cross-OS object tree makes the linker
# report "file format not recognized", or — worse — leaves a working binary for
# the *other* OS in place, and every payload then reports a byte mismatch that
# has nothing to do with the Rust code. Hence: detect the OS, and if the
# existing tree was built for the other one, wipe it first.
if [ "$(uname -s)" = "Darwin" ]; then
	export PREMAKE5="c/dep/bin-osx/premake5"
	export BD_NINJA="c/dep/bin-osx/ninja"
	MAKE_TARGET="macosx-release-x64"
	EXPECT_FORMAT="Mach-O"
else
	export PREMAKE5="c/dep/bin-linux/premake5"
	export BD_NINJA="c/dep/bin-linux/ninja"
	MAKE_TARGET="linux-release-x64"
	EXPECT_FORMAT="ELF"
fi
chmod +x "${PREMAKE5}" "${BD_NINJA}"
if command -v file >/dev/null 2>&1 && [ -f bin/release-x64/otfccdump ] &&
	! file -b bin/release-x64/otfccdump | grep -q "${EXPECT_FORMAT}"; then
	echo "  (existing build/ and bin/ are from another OS — clearing)"
	rm -rf build/ninja build/obj bin/release-x64 bin/x64
fi
# quick.make's mf-ninja-linux passes --cc=$(CC) to premake5; Make's built-in
# default ($(CC) = "cc") isn't a valid compiler name for it. Default to
# clang, not gcc: c2rust's transpile is based on parsing with clang's AST, and
# gcc vs clang produce measurably different floating-point rounding in this
# codebase (verified: a gcc-built otfccbuild differs byte-for-byte from a
# clang-built one on the SAME source and SAME machine, while clang builds
# match across architectures/OSes) — using gcc here would flag that
# gcc/clang difference as a false Rust-vs-C mismatch.
export CC="${CC:-clang}"
if [ "${CC}" = "cc" ]; then export CC=clang; fi
make -f c/quick.make "${MAKE_TARGET}"
C_BIN=bin/release-x64

BUILD=build/compare-with-c
mkdir -p "${BUILD}"

TTF_PAYLOADS="NotoNastaliqUrdu-Regular iosevka-r BungeeColor-Regular_colr_Windows Reinebow-SVGinOT vtt Molengo-Regular"
CFF_PAYLOADS="KRName-Regular"
# Cormorant-Medium / WorkSans-Regular.otf are excluded: both the C and Rust
# otfccdump stack-overflow on them (a pre-existing bug in the C CFF
# interpreter — see rust/README.md), unrelated to this comparison.

# Optional: the gvar (variable-font) payload from make-test-variable-font.py.
# Needs fontTools, so it's generated as a separate CI step rather than always
# required; skip if it wasn't generated.
GVAR_PAYLOAD="build/gvar-test.ttf"

fail=0

compare_payload() {
	local name="$1" ext="$2" in="$3"
	local out="${BUILD}/${name}"

	# Canonical input JSON, dumped once with the C toolchain so both builds
	# start from byte-identical input.
	"${C_BIN}/otfccdump" "${in}" -o "${out}.json" --pretty

	# ...which also makes it the expectation for the Rust dumper. Until this
	# check existed, nothing compared otfccdump's output between the two
	# implementations: the JSON both builds consume comes from C, so a
	# divergence in the dump direction only showed up if it happened to change
	# the *build* result too. Anything that writes JSON keys or number
	# formatting (flag label tables, for instance) needs this half.
	rm -f "${out}.rust.json"
	if ! "${RUST_BIN}/otfccdump" "${in}" -o "${out}.rust.json" --pretty; then
		echo "FAIL  ${name} dump: Rust otfccdump exited non-zero"
		fail=1
	elif cmp -s "${out}.json" "${out}.rust.json"; then
		echo "PASS  ${name} dump: byte-identical"
	else
		echo "FAIL  ${name} dump: differs ($(cmp -l "${out}.json" "${out}.rust.json" 2>/dev/null | wc -l) bytes)"
		fail=1
	fi

	# Check both builds actually succeeded. Under `set -e` a crash would abort
	# the whole script with nothing but the shell's own "Abort trap: 6", which
	# reads like a harness problem rather than a failing payload; and a stale
	# output file from an earlier run would still be sitting there. Report it as
	# a FAIL for this payload and carry on to the rest.
	rm -f "${out}.c.${ext}" "${out}.rust.${ext}"
	if ! "${C_BIN}/otfccbuild" "${out}.json" -o "${out}.c.${ext}" --keep-average-char-width --keep-modified-time; then
		echo "FAIL  ${name}.${ext}: C otfccbuild exited non-zero"
		fail=1
		return
	fi
	if ! "${RUST_BIN}/otfccbuild" "${out}.json" -o "${out}.rust.${ext}" --keep-average-char-width --keep-modified-time; then
		echo "FAIL  ${name}.${ext}: Rust otfccbuild exited non-zero"
		fail=1
		return
	fi

	if cmp -s "${out}.c.${ext}" "${out}.rust.${ext}"; then
		echo "PASS  ${name}.${ext}: byte-identical"
	else
		echo "FAIL  ${name}.${ext}: differs ($(cmp -l "${out}.c.${ext}" "${out}.rust.${ext}" 2>/dev/null | wc -l) bytes)"
		fail=1
	fi
}

echo "==> Comparing C vs Rust otfccbuild output, byte-for-byte"
for f in ${TTF_PAYLOADS}; do
	compare_payload "${f}" ttf "tests/payload/${f}.ttf"
done
for f in ${CFF_PAYLOADS}; do
	compare_payload "${f}" otf "tests/payload/${f}.otf"
done
if [ -f "${GVAR_PAYLOAD}" ]; then
	compare_payload "gvar-test" ttf "${GVAR_PAYLOAD}"
else
	echo "  (skipping gvar-test.ttf: not found; run rust/scripts/make-test-variable-font.py first)"
fi

# A lookup type otfcc does not recognise is *kept*, not clamped: the reader does
# `type = read_16u(data) + base` and hands the result on, so such a lookup gets
# no subtable, dumps as `{}`, and — with no name from the feature list — is named
# after the raw number in hex. That is why `otl_LookupType` is a newtype over
# `u32` rather than an `enum`, and none of the payloads above has one, so the
# comparison that would notice a change there is this one.
#
# Dump only. Both toolchains *refuse* to build the resulting JSON ("Lookup … does
# not have a valid 'type' field"), which is itself matching behaviour but not
# something compare_payload can express — it treats a non-zero otfccbuild as a
# failure, correctly, for every payload that is supposed to build.
if command -v python3 >/dev/null 2>&1; then
	UNKNOWN_LOOKUP="${BUILD}/unknown-lookup.ttf"
	python3 rust/scripts/make-test-unknown-lookup.py tests/payload/iosevka-r.ttf "${UNKNOWN_LOOKUP}"
	rm -f "${BUILD}/unknown-lookup.c.json" "${BUILD}/unknown-lookup.rust.json"
	"${C_BIN}/otfccdump" "${UNKNOWN_LOOKUP}" -o "${BUILD}/unknown-lookup.c.json" --pretty
	if ! "${RUST_BIN}/otfccdump" "${UNKNOWN_LOOKUP}" -o "${BUILD}/unknown-lookup.rust.json" --pretty; then
		echo "FAIL  unknown-lookup dump: Rust otfccdump exited non-zero"
		fail=1
	elif cmp -s "${BUILD}/unknown-lookup.c.json" "${BUILD}/unknown-lookup.rust.json"; then
		echo "PASS  unknown-lookup dump: byte-identical"
	else
		echo "FAIL  unknown-lookup dump: differs ($(cmp -l "${BUILD}/unknown-lookup.c.json" "${BUILD}/unknown-lookup.rust.json" 2>/dev/null | wc -l) bytes)"
		fail=1
	fi
else
	echo "  (skipping unknown-lookup: python3 not found)"
fi

echo "==> Comparing C vs Rust otfccdll (cdylib) output, byte-for-byte"
DLL_C="${C_BIN}/libotfccdll.so"
[ "$(uname)" = "Darwin" ] && DLL_C="${C_BIN}/libotfccdll.dylib"
RUST_SO_EXT="so"
[ "$(uname)" = "Darwin" ] && RUST_SO_EXT="dylib"
DLL_RUST="${RUST_BIN}/libotfcc_rust.${RUST_SO_EXT}"

# Skip with an explicit reason (rather than dying on an unrelated ctypes
# OSError) when python3 can't load the cdylib at all — see dll-arch-check.sh.
# Any *other* test-dll.py failure stays fatal.
. "$(dirname "$0")/dll-arch-check.sh"
DLL_ARCH_SKIP="$(dll_arch_skip_reason "${DLL_RUST}")"

if [ -n "${DLL_ARCH_SKIP}" ]; then
	echo "  (SKIP otfccdll comparison: ${DLL_ARCH_SKIP};"
	echo "   this check runs for real in the Linux container and in CI)"
elif [ -f "${DLL_C}" ] && [ -f "${DLL_RUST}" ]; then
	DLL_JSON="${BUILD}/Molengo-Regular.json"
	python3 "$(dirname "$0")/test-dll.py" "${DLL_C}" "${DLL_JSON}" "${BUILD}/dll-c.otf"
	python3 "$(dirname "$0")/test-dll.py" "${DLL_RUST}" "${DLL_JSON}" "${BUILD}/dll-rust.otf"
	# The DLL API doesn't take --keep-modified-time, so head.created/modified/
	# checkSumAdjustment legitimately vary run to run (see README) — even two
	# C-only invocations differ at those bytes. Diff byte count against that
	# same-library baseline instead of expecting a plain cmp to pass.
	python3 "$(dirname "$0")/test-dll.py" "${DLL_C}" "${DLL_JSON}" "${BUILD}/dll-c-2.otf"
	# cmp -l exits non-zero whenever the files differ, which they legitimately
	# do here (see comment above) — under `set -e`, that would abort the whole
	# script at the very assignment meant to *measure* that expected diff, so
	# tolerate cmp's exit status explicitly with `|| true`.
	baseline_diff=$( (cmp -l "${BUILD}/dll-c.otf" "${BUILD}/dll-c-2.otf" 2>/dev/null || true) | wc -l | tr -d ' ')
	cross_diff=$( (cmp -l "${BUILD}/dll-c.otf" "${BUILD}/dll-rust.otf" 2>/dev/null || true) | wc -l | tr -d ' ')
	if [ "${cross_diff}" -le "${baseline_diff}" ]; then
		echo "PASS  otfccdll: Rust matches C (differs in ${cross_diff} bytes, same as the ${baseline_diff}-byte run-to-run timestamp variance)"
	else
		echo "FAIL  otfccdll: Rust differs from C in ${cross_diff} bytes (run-to-run baseline is only ${baseline_diff})"
		fail=1
	fi
else
	echo "  (skipping otfccdll comparison: ${DLL_C} or ${DLL_RUST} not built)"
fi

if [ "${fail}" -ne 0 ]; then
	echo "==> FAILED: at least one payload's Rust output differs from C" >&2
	exit 1
fi
echo "==> All payloads byte-identical between C and Rust"
