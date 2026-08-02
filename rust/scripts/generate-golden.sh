#!/usr/bin/env bash
# (Re)generates the frozen fixtures under tests/golden/ from the currently
# built Rust crate. rust/scripts/compare-with-golden.sh checks against
# these instead of rebuilding C from source on every run.
#
# Only run this deliberately, when a change *legitimately* alters output
# (a real bug fix, a new feature) -- never to make a failing comparison
# pass without understanding why it failed. Before regenerating:
#
#   1. Confirm the new output is actually correct (by hand, or by building
#      the C toolchain and running rust/scripts/compare-with-c.sh, if the
#      change is meant to keep matching C's behavior).
#   2. Run this script.
#   3. `git diff --stat tests/golden/` and review which fixtures moved --
#      an unexpectedly large or unrelated diff is a sign something other
#      than the intended change affected output.
#   4. Commit the updated fixtures in the SAME commit as the change that
#      motivated them, so the diff explains itself.
#
# Must run AFTER the Rust crate has been built (cargo build --release).
#
# Invoke as: ./rust/scripts/generate-golden.sh
set -euo pipefail
cd "$(dirname "$0")/../.."

BIN=rust/target/release
if [ ! -x "${BIN}/otfccdump" ] || [ ! -x "${BIN}/otfccbuild" ]; then
	echo "ERROR: ${BIN}/{otfccdump,otfccbuild} not found; build the Rust crate first." >&2
	exit 1
fi

GOLDEN=tests/golden
mkdir -p "${GOLDEN}"

gen_pair() {
	local name="$1" ext="$2" in="$3"
	"${BIN}/otfccdump" "${in}" -o "${GOLDEN}/${name}.json" --pretty
	"${BIN}/otfccbuild" "${GOLDEN}/${name}.json" -o "${GOLDEN}/${name}.${ext}" --keep-average-char-width --keep-modified-time
	echo "  ${name}"
}

echo "==> Regenerating tests/golden/ from the built Rust crate"
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
python3 rust/scripts/make-test-unknown-lookup.py tests/payload/iosevka-r.ttf build/golden-gen-unknown-lookup.ttf
"${BIN}/otfccdump" build/golden-gen-unknown-lookup.ttf -o "${GOLDEN}/unknown-lookup.json" --pretty

synth_pair() {
	local name="$1" maker="$2"
	python3 "${maker}" "${GOLDEN}/iosevka-r.json" "build/golden-gen-${name}-input.json"
	"${BIN}/otfccbuild" "build/golden-gen-${name}-input.json" -o "${GOLDEN}/${name}.ttf" --keep-average-char-width --keep-modified-time
	"${BIN}/otfccdump" "${GOLDEN}/${name}.ttf" -o "${GOLDEN}/${name}.dump.json" --pretty
	echo "  ${name}"
}
synth_pair meta-test rust/scripts/make-test-meta.py
synth_pair vdmx-test rust/scripts/make-test-vdmx.py

echo "  dll-test.otf"
SO_EXT="so"
[ "$(uname)" = "Darwin" ] && SO_EXT="dylib"
python3 rust/scripts/test-dll.py "${BIN}/libotfcc_rust.${SO_EXT}" "${GOLDEN}/Molengo-Regular.json" "${GOLDEN}/dll-test.otf"

echo "==> Done. Review with: git diff --stat tests/golden/"
