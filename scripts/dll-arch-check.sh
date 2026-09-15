#!/usr/bin/env bash
# Sourceable helper: can python3 actually dlopen the crate's cdylib?
#
# The otfccdll checks load the cdylib through python3/ctypes, so the two must
# share an architecture. Normally they do — rust-toolchain.toml's `channel`
# resolves to rustup's own host triple, which matches the system python3.
#
# What breaks it on an Apple Silicon Mac is a *Rosetta rustup*: an
# x86_64-apple-darwin rustup resolves `1.97.1` to the x86_64 toolchain, so cargo
# emits an x86_64 dylib while python3 is arm64, and there is no Rosetta python3
# to load it with (`arch -x86_64 python3` -> "Bad CPU type"). Installing the
# native toolchain alongside it fixes that:
#
#   rustup toolchain install 1.97.1-aarch64-apple-darwin --force-non-host
#   export PATH="$HOME/.rustup/toolchains/1.97.1-aarch64-apple-darwin/bin:$PATH"
#
# Callers use this to SKIP the ctypes check with an explicit reason instead of
# dying on an OSError that has nothing to do with the code under test. The same
# check runs for real in the arch-matched Linux container and in CI.
#
# Usage:
#   . "$(dirname "$0")/dll-arch-check.sh"
#   reason="$(dll_arch_skip_reason "path/to/libotfcc_rust.dylib")"
#   [ -n "${reason}" ] && echo "skipping: ${reason}"

# Echoes a human-readable reason if python3 cannot load ${1}; echoes nothing
# when the architectures match (or when the situation can't arise / can't be
# determined, in which case the caller should just try it).
dll_arch_skip_reason() {
	local lib="$1"
	[ "$(uname -s)" = "Darwin" ] || return 0
	[ -f "${lib}" ] || return 0
	command -v lipo >/dev/null 2>&1 || return 0

	local dll_arch py_arch
	dll_arch="$(lipo -archs "${lib}" 2>/dev/null || true)"
	py_arch="$(python3 -c 'import platform; print(platform.machine())' 2>/dev/null || true)"
	[ -n "${dll_arch}" ] && [ -n "${py_arch}" ] || return 0

	# dll_arch may list several architectures for a fat binary; a match on any
	# one of them is enough.
	if ! printf '%s\n' ${dll_arch} | grep -qx "${py_arch}"; then
		echo "cdylib is ${dll_arch}, python3 is ${py_arch} — cannot dlopen across architectures"
	fi
}
