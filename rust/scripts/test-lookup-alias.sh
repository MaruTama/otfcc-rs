#!/usr/bin/env bash
# Rust-only regression test for the GSUB lookup-alias double-push bug (see
# rust/scripts/make-test-lookup-alias.py and rust/README.md for the full
# writeup). Unlike every other make-test-*.py payload, this one is NOT run
# through compare-with-c.sh: the bug is present in the original C source too,
# and the fix here is Rust-only by explicit decision, so C still crashes on
# this input and there is nothing to byte-compare against.
#
# Instead this asserts, against the Rust binaries alone:
#   1. otfccbuild exits 0 (does not crash / hang) on the alias payload.
#   2. the build is deterministic (byte-identical across repeated runs --
#      the same kind of check that caught the gasp uninitialized-Vec bug,
#      which only otfccbuild's own nondeterministic crash pattern revealed).
#   3. the alias does not inflate the lookup count (the whole point of the
#      `LookupHash.alias` fix is that the alias's `.lookup` push is skipped,
#      so it must resolve to the SAME underlying Lookup, not a duplicate).
#   4. dump -> build -> dump is stable (round-trip idempotence).
#
# Invoke as: ./rust/scripts/test-lookup-alias.sh
set -euo pipefail
cd "$(dirname "$0")/../.."

BIN=rust/target/release
if [ ! -x "${BIN}/otfccdump" ] || [ ! -x "${BIN}/otfccbuild" ]; then
	echo "ERROR: ${BIN}/{otfccdump,otfccbuild} not found; run build-crate.sh first." >&2
	exit 1
fi

BUILD=build/lookup-alias-test
mkdir -p "${BUILD}"

SRC_JSON="tests/payload/kltf-bugfont1.json"
ALIAS_JSON="${BUILD}/alias.json"
python3 rust/scripts/make-test-lookup-alias.py "${SRC_JSON}" "${ALIAS_JSON}"

# Baseline lookup count (from the un-aliased source) -- the aliased build
# must report the same count, not one more.
EXPECT_COUNT=$(python3 -c "import json; print(len(json.load(open('${SRC_JSON}'))['GSUB']['lookups']))")

echo "==> Building the lookup-alias payload 3x, checking exit code and determinism"
for i in 1 2 3; do
	"${BIN}/otfccbuild" "${ALIAS_JSON}" -o "${BUILD}/out.${i}.otf" \
		--keep-average-char-width --keep-modified-time
done
if ! cmp -s "${BUILD}/out.1.otf" "${BUILD}/out.2.otf" || ! cmp -s "${BUILD}/out.1.otf" "${BUILD}/out.3.otf"; then
	echo "FAIL: otfccbuild produced different output across repeated runs on the same input" >&2
	exit 1
fi
echo "PASS: 3 runs exited 0 and produced byte-identical output"

echo "==> Checking the alias resolved to the existing lookup, not a duplicate"
"${BIN}/otfccdump" "${BUILD}/out.1.otf" -o "${BUILD}/dump.1.json" --pretty
GOT_COUNT=$(python3 -c "import json; print(len(json.load(open('${BUILD}/dump.1.json'))['GSUB']['lookups']))")
if [ "${GOT_COUNT}" != "${EXPECT_COUNT}" ]; then
	echo "FAIL: expected ${EXPECT_COUNT} lookups (unchanged by the alias), got ${GOT_COUNT}" >&2
	exit 1
fi
echo "PASS: lookup count unchanged by the alias (${GOT_COUNT})"

echo "==> Checking dump -> build -> dump round-trip stability"
"${BIN}/otfccbuild" "${BUILD}/dump.1.json" -o "${BUILD}/out.2nd.otf" \
	--keep-average-char-width --keep-modified-time
"${BIN}/otfccdump" "${BUILD}/out.2nd.otf" -o "${BUILD}/dump.2.json" --pretty
if ! cmp -s "${BUILD}/dump.1.json" "${BUILD}/dump.2.json"; then
	echo "FAIL: dump -> build -> dump is not stable for the lookup-alias payload" >&2
	exit 1
fi
echo "PASS: round-trip stable"

echo "==> lookup-alias regression test passed"
