#!/usr/bin/env bash
# Convenience wrapper: build-crate.sh + clippy + check-abi.sh +
# compare-with-golden.sh + compare-log-output.sh + run-cycles.sh +
# test-lookup-alias.sh -- the same steps CI runs, minus the round-trip
# stability tests (see below). Mirrors quick.make's
# ttfroundtriptest/cffroundtriptest targets.
#
# Invoke as:
#   ./rust/scripts/test.sh
#
# The round-trip comparisons need `node`, so this script only builds the crate
# and produces the dump/build cycle artifacts under build/; run
# `node rust/scripts/compare-roundtrips.js` afterward (see the loop this
# script prints at the end).
set -euo pipefail
cd "$(dirname "$0")/../.."

./rust/scripts/build-crate.sh
( cd rust && cargo clippy --release --all-targets --locked -- -D warnings )
./rust/scripts/check-abi.sh
./rust/scripts/compare-with-golden.sh
./rust/scripts/compare-log-output.sh
./rust/scripts/run-cycles.sh
./rust/scripts/test-lookup-alias.sh
