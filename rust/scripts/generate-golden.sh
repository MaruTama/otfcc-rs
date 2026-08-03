#!/usr/bin/env bash
# (Re)generates tests/golden/checksums.sha256 from the currently built Rust
# crate. rust/scripts/compare-with-golden.sh checks against it instead of
# rebuilding C from source on every run.
#
# Stores SHA-256 checksums, not the dump JSON / build output files
# themselves -- a hash is exactly as good at detecting "this changed" as
# the full bytes are, at a tiny fraction of the size (the alternative was
# ~28MB of committed fixtures, dominated by one payload's 14MB pretty-
# printed JSON dump). The one thing a hash can't do is show you *what*
# changed in a `git diff` -- for that, use rust/scripts/compare-with-c.sh
# (which does keep full output around, transiently, under build/) or
# otfccdump the payload yourself and diff by hand.
#
# The one file kept as an actual committed fixture, not a hash, is
# tests/golden/dll-test.otf: the otfccdll cdylib API doesn't take
# --keep-modified-time, so its output legitimately varies by a handful of
# timestamp bytes even between two correct builds -- compare-with-golden.sh
# needs the real bytes to run a tolerance-based `cmp -l`, not an
# all-or-nothing hash match.
#
# Only run this deliberately, when a change *legitimately* alters output
# (a real bug fix, a new feature) -- never to make a failing comparison
# pass without understanding why it failed. Before regenerating:
#
#   1. Confirm the new output is actually correct (by hand, or by building
#      the C toolchain and running rust/scripts/compare-with-c.sh, if the
#      change is meant to keep matching C's behavior).
#   2. Run this script.
#   3. `git diff tests/golden/checksums.sha256` and review which labels
#      moved -- an unexpectedly large or unrelated set of changed lines is
#      a sign something other than the intended change affected output.
#   4. Commit the updated fixtures in the SAME commit as the change that
#      motivated them, so the diff explains itself.
#
# Must run AFTER the Rust crate has been built (cargo build --release).
#
# Invoke as: ./rust/scripts/generate-golden.sh
set -euo pipefail
cd "$(dirname "$0")/../.."
. rust/scripts/sha256-of.sh

BIN=rust/target/release
if [ ! -x "${BIN}/otfccdump" ] || [ ! -x "${BIN}/otfccbuild" ]; then
	echo "ERROR: ${BIN}/{otfccdump,otfccbuild} not found; build the Rust crate first." >&2
	exit 1
fi

GOLDEN=tests/golden
SCRATCH=build/golden-gen
mkdir -p "${GOLDEN}" "${SCRATCH}"
CHECKSUMS="${GOLDEN}/checksums.sha256"
: > "${CHECKSUMS}"

record() {
	# record <file> <label> -- appends "<sha256>  <label>" to the manifest.
	printf '%s  %s\n' "$(sha256_of "$1")" "$2" >>"${CHECKSUMS}"
}

gen_pair() {
	local name="$1" ext="$2" in="$3"
	"${BIN}/otfccdump" "${in}" -o "${SCRATCH}/${name}.json" --pretty
	record "${SCRATCH}/${name}.json" "${name}.json"
	"${BIN}/otfccbuild" "${SCRATCH}/${name}.json" -o "${SCRATCH}/${name}.${ext}" --keep-average-char-width --keep-modified-time
	record "${SCRATCH}/${name}.${ext}" "${name}.${ext}"
	echo "  ${name}"
}

echo "==> Regenerating tests/golden/checksums.sha256 from the built Rust crate"
gen_pair NotoNastaliqUrdu-Regular ttf tests/payload/NotoNastaliqUrdu-Regular.ttf
gen_pair iosevka-r ttf tests/payload/iosevka-r.ttf
gen_pair BungeeColor-Regular_colr_Windows ttf tests/payload/BungeeColor-Regular_colr_Windows.ttf
gen_pair Reinebow-SVGinOT ttf tests/payload/Reinebow-SVGinOT.ttf
gen_pair vtt ttf tests/payload/vtt.ttf
gen_pair Molengo-Regular ttf tests/payload/Molengo-Regular.ttf
gen_pair KRName-Regular otf tests/payload/KRName-Regular.otf
# tests/payload/gvar-test.ttf is itself a frozen fixture -- see the comment
# in compare-with-golden.sh for why it is not regenerated via fontTools here.
gen_pair gvar-test ttf tests/payload/gvar-test.ttf

echo "  unknown-lookup (dump only)"
python3 rust/scripts/make-test-unknown-lookup.py tests/payload/iosevka-r.ttf "${SCRATCH}/unknown-lookup-src.ttf"
"${BIN}/otfccdump" "${SCRATCH}/unknown-lookup-src.ttf" -o "${SCRATCH}/unknown-lookup.json" --pretty
record "${SCRATCH}/unknown-lookup.json" "unknown-lookup.json"

synth_pair() {
	local name="$1" maker="$2"
	python3 "${maker}" "${SCRATCH}/iosevka-r.json" "${SCRATCH}/${name}-input.json"
	"${BIN}/otfccbuild" "${SCRATCH}/${name}-input.json" -o "${SCRATCH}/${name}.ttf" --keep-average-char-width --keep-modified-time
	record "${SCRATCH}/${name}.ttf" "${name}.ttf"
	"${BIN}/otfccdump" "${SCRATCH}/${name}.ttf" -o "${SCRATCH}/${name}.dump.json" --pretty
	record "${SCRATCH}/${name}.dump.json" "${name}.dump.json"
	echo "  ${name}"
}
synth_pair meta-test rust/scripts/make-test-meta.py
synth_pair vdmx-test rust/scripts/make-test-vdmx.py
synth_pair gsub-multi-dedup rust/scripts/make-test-gsub-multi-dedup.py

echo "  dll-test.otf (kept as a real committed file, not a hash -- see header comment)"
SO_EXT="so"
[ "$(uname)" = "Darwin" ] && SO_EXT="dylib"
python3 rust/scripts/test-dll.py "${BIN}/libotfcc_rust.${SO_EXT}" "${SCRATCH}/Molengo-Regular.json" "${GOLDEN}/dll-test.otf"

sort -k2 -o "${CHECKSUMS}" "${CHECKSUMS}"
rm -rf "${SCRATCH}"
echo "==> Done. Review with: git diff tests/golden/checksums.sha256"
