#!/usr/bin/env bash
# Compares otfccdump/otfccbuild's *log* output (everything written to stderr
# via the Logger -- indent/start/log/dedent/finish) against the frozen
# fixtures in tests/golden/log/, instead of rebuilding the C toolchain from
# source and diffing against it on every run.
#
# The golden log fixtures were captured from the Rust build at a point where
# this script (in its earlier form) had just confirmed it byte-identical to
# C's log output on every case below; they are C's approval, frozen -- the
# same move compare-with-golden.sh already made for dump/build output. See
# rust/scripts/generate-log-golden.sh to refresh them after a legitimate
# change, and rust/README.md for when this switched over.
#
# Until the original version of this script existed, nothing compared logger
# output at all: every other comparison in this directory (compare-with-c.sh,
# compare-with-golden.sh, run-cycles.sh) only checks the *produced font/JSON
# files*, which never touch the logger. That left the entire Stage 6-2 Logger
# vtable retype (ILogger/ILoggerTarget's Vec<u8>-typed slots, Logger.indents
# as Vec<Vec<u8>>, the indent-guide rendering in logger_log_sds) with zero
# coverage from the rest of the suite -- a change to indentation width, the
# " | "/" |-" guide characters, message wording, or which messages get
# demoted below --quiet/promoted under --verbose could regress silently.
# That risk is exactly the same for the ongoing Stage 7-2 Logger-ownership
# refactor, which is why this check is worth keeping (and now keeping in CI,
# since it no longer needs the C toolchain to run).
#
# Must run AFTER the Rust crate has been built (cargo build --release).
#
# Invoke as: ./rust/scripts/compare-log-output.sh
set -euo pipefail
cd "$(dirname "$0")/../.."

RUST_BIN=rust/target/release
if [ ! -x "${RUST_BIN}/otfccdump" ] || [ ! -x "${RUST_BIN}/otfccbuild" ]; then
	echo "ERROR: ${RUST_BIN}/{otfccdump,otfccbuild} not found; build the Rust crate first." >&2
	exit 1
fi

GOLDEN=tests/golden/log
BUILD=build/compare-log-output
mkdir -p "${BUILD}"

fail=0

# See generate-log-golden.sh: the golden fixtures already have this applied
# (including blanking out the scratch-directory path, since that script and
# this one use differently-named scratch dirs), so the freshly captured
# output needs the same normalization to compare equal.
normalize() {
	sed -E \
		-e 's/Step time = [0-9.eE+-]+s\./Step time = <T>s./g' \
		-e 's#build/[A-Za-z0-9_-]+/#build/<SCRATCH>/#g' \
		"$1"
}

compare_log() {
	local label="$1"
	shift
	local raw_log="${BUILD}/${label}.raw.log"
	local norm_log="${BUILD}/${label}.norm.log"
	local golden_log="${GOLDEN}/${label}.log"

	if [ ! -f "${golden_log}" ]; then
		echo "FAIL  ${label}: no golden fixture at ${golden_log}" >&2
		fail=1
		return
	fi

	"$@" >"${raw_log}.stdout" 2>"${raw_log}" || true
	normalize "${raw_log}" >"${norm_log}"

	if cmp -s "${norm_log}" "${golden_log}"; then
		echo "PASS  ${label}: log output matches golden (modulo elapsed-time numbers)"
	else
		echo "FAIL  ${label}: log output differs from golden"
		diff -u "${golden_log}" "${norm_log}" | head -40 || true
		fail=1
	fi
}

dump_verbose() {
	"${RUST_BIN}/otfccdump" tests/payload/iosevka-r.ttf -o "${BUILD}/iosevka-r.json" --pretty --verbose
}
dump_quiet() {
	"${RUST_BIN}/otfccdump" tests/payload/iosevka-r.ttf -o "${BUILD}/iosevka-r-q.json" --pretty --quiet
}
dump_cff_verbose() {
	"${RUST_BIN}/otfccdump" tests/payload/KRName-Regular.otf -o "${BUILD}/KRName-Regular.json" --pretty --verbose
}
build_verbose() {
	"${RUST_BIN}/otfccbuild" "${BUILD}/iosevka-r.json" -o "${BUILD}/iosevka-r.ttf" --keep-average-char-width --keep-modified-time --verbose
}
build_quiet() {
	"${RUST_BIN}/otfccbuild" "${BUILD}/iosevka-r.json" -o "${BUILD}/iosevka-r-q.ttf" --keep-average-char-width --keep-modified-time --quiet
}
dump_missing_file() {
	"${RUST_BIN}/otfccdump" "${BUILD}/does-not-exist.ttf" -o "${BUILD}/does-not-exist.json" --verbose
}

echo "==> Comparing Rust log output (stderr) against tests/golden/log/, byte-for-byte modulo timing"

# --verbose is the interesting case: it's the only flag that exercises
# indent/dedent nesting (LOG_VL_PROGRESS-level "Begin"/"Finish" pairs) and
# the continuation-guide rendering in logger_log_sds, i.e. everything the
# Logger.indents: Vec<Vec<u8>> retype could get wrong.
#
# dump-verbose must run before build-verbose/build-quiet: they consume the
# iosevka-r.json that dump-verbose produces.
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
	echo "==> FAILED: at least one payload's Rust log output differs from the golden fixtures" >&2
	exit 1
fi
echo "==> All log output matches tests/golden/log/ (modulo elapsed-time numbers)"
