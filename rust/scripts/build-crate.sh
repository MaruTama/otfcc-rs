#!/usr/bin/env bash
# Builds the transpiled crate (release) with the pinned stable toolchain in
# rust-toolchain.toml. Requires only rustup + cargo — no c2rust/Docker needed;
# works on any architecture.
#
# Invoke as: ./rust/scripts/build-crate.sh
set -euo pipefail
cd "$(dirname "$0")/../.."

CRATE=rust
if [ ! -d "${CRATE}" ]; then
	echo "ERROR: ${CRATE} not found (run this from the repo root)." >&2
	exit 1
fi

echo "==> Building the crate (release)"
# Cargo.lock is committed and used as-is (--locked): the crate has a real
# dependency now, so an unpinned resolve would make "byte-identical to C" quietly
# depend on whatever libc version happened to be current. It used to be deleted
# here, from when the crate had no dependencies at all and the lock file was
# nothing but c2rust output that went stale.
( cd "${CRATE}" && cargo build --release --locked )

echo "==> Running cargo test"
( cd "${CRATE}" && cargo test --release --locked )
