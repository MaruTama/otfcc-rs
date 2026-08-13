#!/usr/bin/env bash
# Compares otfccdump/otfccbuild's *log* output (everything written to stderr
# via the ILogger vtable -- indent/start/log/dedent/finish) between the C and
# Rust builds, byte-for-byte modulo elapsed-time numbers.
#
# Until this script existed, nothing compared logger output at all: every
# other comparison in this directory (compare-with-c.sh, compare-with-golden.sh,
# run-cycles.sh) only checks the *produced font/JSON files*, which never touch
# the logger. That left the entire Stage 6-2 Logger vtable retype (ILogger /
# ILoggerTarget's Vec<u8>-typed slots, Logger.indents as Vec<Vec<u8>>, the
# indent-guide rendering in logger_log_sds) with zero coverage from the rest
# of the suite -- a change to indentation width, the " | "/" |-" guide
# characters, message wording, or which messages get demoted below
# --quiet/promoted under --verbose could regress silently.
#
# Must run AFTER the Rust crate has been built (cargo build --release) and on
# the SAME architecture as that build, for the same reason as
# compare-with-c.sh (byte comparison across a cross-arch build is meaningless).
#
# Invoke as: ./rust/scripts/compare-log-output.sh
set -euo pipefail
cd "$(dirname "$0")/../.."

RUST_BIN=rust/target/release
if [ ! -x "${RUST_BIN}/otfccdump" ] || [ ! -x "${RUST_BIN}/otfccbuild" ]; then
	echo "ERROR: ${RUST_BIN}/{otfccdump,otfccbuild} not found; build the Rust crate first." >&2
	exit 1
fi

echo "==> Building the C toolchain (native)"
# Same OS-detection / stale-tree-wipe / compiler-pinning logic as
# compare-with-c.sh -- see that script for the full rationale on each of
# these choices (clang over gcc for float-rounding parity, wiping build/ and
# bin/ when they're from the other OS, etc).
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
export CC="${CC:-clang}"
if [ "${CC}" = "cc" ]; then export CC=clang; fi
make -f c/quick.make "${MAKE_TARGET}"
C_BIN=bin/release-x64

BUILD=build/compare-log-output
mkdir -p "${BUILD}"

fail=0

# `push_stopwatch` (rust/src/support/stopwatch.rs) formats real elapsed time
# with libc's "%g", e.g. "Step time = 0.000123s.\n" -- the one piece of log
# output that can never match byte-for-byte between two separate process
# runs, let alone two different implementations. Blank it out before
# comparing so the check covers everything else (message text, indentation,
# the " | " / " |-" continuation guides, which messages appear at all) without
# being defeated by a number that is expected to differ.
normalize() {
	sed -E 's/Step time = [0-9.eE+-]+s\./Step time = <T>s./g' "$1"
}

compare_log() {
	local label="$1"
	shift
	local c_log="${BUILD}/${label}.c.log"
	local rust_log="${BUILD}/${label}.rust.log"
	local c_norm="${BUILD}/${label}.c.norm.log"
	local rust_norm="${BUILD}/${label}.rust.norm.log"

	"$@" "${C_BIN}" >"${c_log}.stdout" 2>"${c_log}" || true
	"$@" "${RUST_BIN}" >"${rust_log}.stdout" 2>"${rust_log}" || true

	normalize "${c_log}" >"${c_norm}"
	normalize "${rust_log}" >"${rust_norm}"

	if cmp -s "${c_norm}" "${rust_norm}"; then
		echo "PASS  ${label}: log output matches (modulo elapsed-time numbers)"
	else
		echo "FAIL  ${label}: log output differs"
		diff -u "${c_norm}" "${rust_norm}" | head -40 || true
		fail=1
	fi
}

# Each helper takes the binary directory as its one argument, so the same
# closure runs against both C_BIN and RUST_BIN inside compare_log.
dump_verbose() {
	"$1/otfccdump" tests/payload/iosevka-r.ttf -o "${BUILD}/iosevka-r.json" --pretty --verbose
}
dump_quiet() {
	"$1/otfccdump" tests/payload/iosevka-r.ttf -o "${BUILD}/iosevka-r-q.json" --pretty --quiet
}
dump_cff_verbose() {
	"$1/otfccdump" tests/payload/KRName-Regular.otf -o "${BUILD}/KRName-Regular.json" --pretty --verbose
}
build_verbose() {
	"$1/otfccbuild" "${BUILD}/iosevka-r.json" -o "${BUILD}/iosevka-r.ttf" --keep-average-char-width --keep-modified-time --verbose
}
build_quiet() {
	"$1/otfccbuild" "${BUILD}/iosevka-r.json" -o "${BUILD}/iosevka-r-q.ttf" --keep-average-char-width --keep-modified-time --quiet
}
dump_missing_file() {
	"$1/otfccdump" "${BUILD}/does-not-exist.ttf" -o "${BUILD}/does-not-exist.json" --verbose
}

echo "==> Comparing C vs Rust log output (stderr), byte-for-byte modulo timing"

# --verbose is the interesting case: it's the only flag that exercises
# indent/dedent nesting (LOG_VL_PROGRESS-level "Begin"/"Finish" pairs) and
# the continuation-guide rendering in logger_log_sds, i.e. everything the
# Logger.indents: Vec<Vec<u8>> retype could get wrong.
compare_log "dump-verbose" dump_verbose
# --quiet raises the verbosity floor, so this exercises the OTHER half of
# set_verbosity/verbosity_limit filtering: confirms nothing at all reaches
# the target when it shouldn't.
compare_log "dump-quiet" dump_quiet
# A CFF (.otf) payload takes a different code path (libcff's own logging in
# addition to the shared reader/writer log calls), so it needs its own check.
compare_log "dump-cff-verbose" dump_cff_verbose
compare_log "build-verbose" build_verbose
compare_log "build-quiet" build_quiet
# The LOG_VL_CRITICAL / LoggerType::Error path (logger_log_sds's
# OTFCC_LOGGER_TYPE_NAMES prefix) is otherwise never reached by any payload
# above, since they all succeed.
compare_log "dump-missing-file" dump_missing_file

if [ "${fail}" -ne 0 ]; then
	echo "==> FAILED: at least one payload's Rust log output differs from C" >&2
	exit 1
fi
echo "==> All log output matches between C and Rust (modulo elapsed-time numbers)"
