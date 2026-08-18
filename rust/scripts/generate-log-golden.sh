#!/usr/bin/env bash
# (Re)generates tests/golden/log/*.log from the currently built Rust crate's
# stderr output. rust/scripts/compare-log-output.sh checks against these
# instead of rebuilding the C toolchain and diffing against it on every run
# -- the same "freeze C's approval, then compare against the freeze" move
# generate-golden.sh/compare-with-golden.sh already made for dump/build
# output. See that pair's header comments for the full rationale.
#
# Real committed text files, not hashes (unlike checksums.sha256): the
# largest of the six is ~300KB, log text is exactly the kind of content
# where `git diff` against the fixture is the point -- a change to message
# wording, indentation, or which messages appear at which verbosity should
# show up as a reviewable diff, not just a pass/fail.
#
# Only run this deliberately, when a change *legitimately* alters log output
# (new/reworded message, changed verbosity threshold, indent-guide rendering
# fix) -- never to make a failing comparison pass without understanding why
# it failed. Before regenerating:
#
#   1. Confirm the new output is actually correct (by hand, or by building
#      the C toolchain and running rust/scripts/compare-log-output.sh in its
#      C-comparison form -- see that script's git history -- if the change
#      is meant to keep matching C's behavior).
#   2. Run this script.
#   3. `git diff tests/golden/log/` and review which files/lines moved -- an
#      unexpectedly large or unrelated diff is a sign something other than
#      the intended change affected log output.
#   4. Commit the updated fixtures in the SAME commit as the change that
#      motivated them, so the diff explains itself.
#
# Must run AFTER the Rust crate has been built (cargo build --release).
#
# Invoke as: ./rust/scripts/generate-log-golden.sh
set -euo pipefail
cd "$(dirname "$0")/../.."

RUST_BIN=rust/target/release
if [ ! -x "${RUST_BIN}/otfccdump" ] || [ ! -x "${RUST_BIN}/otfccbuild" ]; then
	echo "ERROR: ${RUST_BIN}/{otfccdump,otfccbuild} not found; build the Rust crate first." >&2
	exit 1
fi

GOLDEN=tests/golden/log
SCRATCH=build/log-golden-gen
mkdir -p "${GOLDEN}" "${SCRATCH}"

# Same normalization as compare-log-output.sh: push_stopwatch's "%g"-formatted
# elapsed time is the one piece of log output that can never be identical
# between two separate process runs, so it's blanked out before freezing.
# The scratch-directory path is also blanked out: this script and
# compare-log-output.sh use differently-named scratch dirs
# (build/log-golden-gen vs build/compare-log-output), and "From file
# <path>"/error messages otherwise embed that path verbatim -- without this,
# every run would show a spurious diff having nothing to do with log content.
normalize() {
	sed -E \
		-e 's/Step time = [0-9.eE+-]+s\./Step time = <T>s./g' \
		-e 's#build/[A-Za-z0-9_-]+/#build/<SCRATCH>/#g' \
		"$1"
}

gen() {
	local label="$1"
	shift
	"$@" >"${SCRATCH}/${label}.stdout" 2>"${SCRATCH}/${label}.raw.log" || true
	normalize "${SCRATCH}/${label}.raw.log" >"${GOLDEN}/${label}.log"
	echo "  ${label}"
}

dump_verbose() {
	"${RUST_BIN}/otfccdump" tests/payload/iosevka-r.ttf -o "${SCRATCH}/iosevka-r.json" --pretty --verbose
}
dump_quiet() {
	"${RUST_BIN}/otfccdump" tests/payload/iosevka-r.ttf -o "${SCRATCH}/iosevka-r-q.json" --pretty --quiet
}
dump_cff_verbose() {
	"${RUST_BIN}/otfccdump" tests/payload/KRName-Regular.otf -o "${SCRATCH}/KRName-Regular.json" --pretty --verbose
}
build_verbose() {
	"${RUST_BIN}/otfccbuild" "${SCRATCH}/iosevka-r.json" -o "${SCRATCH}/iosevka-r.ttf" --keep-average-char-width --keep-modified-time --verbose
}
build_quiet() {
	"${RUST_BIN}/otfccbuild" "${SCRATCH}/iosevka-r.json" -o "${SCRATCH}/iosevka-r-q.ttf" --keep-average-char-width --keep-modified-time --quiet
}
dump_missing_file() {
	"${RUST_BIN}/otfccdump" "${SCRATCH}/does-not-exist.ttf" -o "${SCRATCH}/does-not-exist.json" --verbose
}

echo "==> Regenerating tests/golden/log/ from the built Rust crate"
# dump_verbose must run before build_verbose/build_quiet: they consume
# iosevka-r.json, which dump_verbose produces.
gen dump-verbose dump_verbose
gen dump-quiet dump_quiet
gen dump-cff-verbose dump_cff_verbose
gen build-verbose build_verbose
gen build-quiet build_quiet
gen dump-missing-file dump_missing_file

rm -rf "${SCRATCH}"
echo "==> Done. Review with: git diff tests/golden/log/"
