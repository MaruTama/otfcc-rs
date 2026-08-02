#!/usr/bin/env bash
# Convenience wrapper: build-crate.sh + run-cycles.sh. Runs the project's own
# round-trip stability tests against the Rust binaries, mirroring quick.make's
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
./rust/scripts/check-abi.sh
./rust/scripts/compare-with-golden.sh
./rust/scripts/run-cycles.sh
./rust/scripts/test-lookup-alias.sh
