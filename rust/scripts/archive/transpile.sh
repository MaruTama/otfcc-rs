#!/usr/bin/env bash
# Transpiles otfcc to Rust with c2rust. Runs inside the c2rust image with the
# repo mounted at its *host* path, so the absolute paths in compile_commands.json
# (generated on the host) resolve unchanged. No premake/ninja/node needed here.
#
# Invoke as:
#   docker run --rm -v "$PWD":"$PWD" -w "$PWD" \
#       --entrypoint bash otfcc-c2rust rust/scripts/transpile.sh
set -euo pipefail

DEST="rust"
# CRITICAL: the compile database must live on the *mounted* filesystem, not on
# the container's tmpfs. With the DB on /tmp, c2rust panics reliably and
# reproducibly ("Type conversion not implemented for TagTypeUnknown") partway
# through; with the byte-identical DB on the bind mount it always succeeds.
# (Likely c2rust's clang AST-exporter writes intermediates relative to the DB.)
DB="rust/.compile_commands.native.json"
WORK=/tmp/otfcc-rust

if [ ! -f c/compile_commands.json ]; then
	echo "ERROR: c/compile_commands.json not found in $(pwd)." >&2
	echo "Generate it on the host first (see rust/README.md)." >&2
	exit 1
fi

# The host database targets an x86_64 config (-m64). This image is native arm64
# and c2rust ignores --target/--sysroot, so it parses the native aarch64 headers;
# strip -m64 to let clang target the native arch. The image already neutralizes
# aarch64's SIMD <bits/math-vector.h>, which c2rust cannot represent. x86_64 and
# arm64 Linux are both LP64, so the transpiled Rust is equivalent.
sed 's/ -m64//g' c/compile_commands.json > "${DB}"

echo "==> Transpiling $(grep -c '"file"' "${DB}") translation units"
rm -rf "${WORK}"
# --emit-build-files writes Cargo.toml/build.rs; -b marks binary entrypoints.
c2rust transpile "${DB}" \
	--emit-build-files \
	--output-dir "${WORK}" \
	-b otfccdump \
	-b otfccbuild
# Note: otfccdll.c is the shared-library entry (no main), so it is NOT a -b
# binary target; it is compiled into the library. The C build (premake5.lua)
# builds it as a SharedLib, so add "cdylib" alongside c2rust's default
# staticlib+rlib crate-type — otfccdll.c's #[no_mangle] extern "C" fns
# (otfccbuild_json_otf / otfcc_get_buf_len / otfcc_get_buf_data /
# otfccbuild_free_otfbuf) are already in lib.rs and just need a cdylib
# target to be callable from outside the crate.
sed -i 's/crate-type = \["staticlib", "rlib"\]/crate-type = ["staticlib", "rlib", "cdylib"]/' \
	"${WORK}/Cargo.toml"

# --- Post-transpile fixups so the crate compiles (build with the pinned
# nightly in rust-toolchain.toml; c2rust output needs release / overflow-checks
# off at runtime) ---
# c2rust emits `let ref mut freshN = (*packed).len;` which takes a reference to
# a packed-struct field — a hard error (E0793). Rewrite to a raw pointer.
sed -i 's/let ref mut \(fresh[0-9]*\) =/let \1 = \&raw mut/' \
	"${WORK}/src/dep/extern/sds.rs"

# Runtime: the vendored dtoa indexes kPow10 (a [u32; 10]) with -kappa, which can
# reach 12 for high-precision doubles. In C that reads adjacent memory (a large
# value that makes GrisuRound a no-op); Rust bounds-checks and panics. Clamp the
# index to the array bounds, reproducing C's effective (no-op-rounding) behavior.
sed -i 's/kPow10\[-kappa as usize\]/kPow10[(-kappa as usize).min(9)]/; s/kPow10\[kappa as usize\]/kPow10[(kappa as usize).min(9)]/' \
	"${WORK}/src/dep/extern/emyg_dtoa/emyg_dtoa.rs"

# For zero-arg, struct-returning function-pointer-field calls that get an
# outer typedef-alias cast (e.g. `Handle.empty()` returning otfcc_Handle,
# assigned to an otfcc_GlyphHandle field), c2rust sometimes emits
#   ::core::mem::transmute::<_, fn(..) -> T>( fnptr_after_.expect(..) )( )
# which silently drops the `unsafe extern "C"` ABI, corrupting every such call
# (observed: struct-by-value returns come back with garbage fields — e.g. a
# glyph handle's `name` field ends up holding an unrelated function's address,
# leading to `free(): invalid pointer` aborts). The wrapped expression already
# has the correct `unsafe extern "C" fn(..) -> T` type, so the transmute is
# both needless and the actual bug; strip it, calling the function pointer
# directly. See rust/scripts/fix-transmute-abi.py for the implementation.
python3 "$(dirname "$0")/fix-transmute-abi.py" "${WORK}"

# Runtime: c2rust mistranslates the IMPLICIT `pos_t` (double) -> `uintN_t`
# narrowing conversion at bufwriteNNb() call sites that have no explicit
# intermediate cast in the C source (e.g. `bufwrite16b(buf,
# hmtx->metrics[j].lsb)`). It emits a direct `lsb as uint16_t`, which uses
# Rust's SATURATING float->unsigned semantics (any negative value becomes 0),
# whereas C's actual runtime behavior converts through a signed integer first
# then reinterprets the bits as unsigned (-41.0 -> 0xFFD7). Observed impact:
# negative side-bearing / vertical-origin values silently corrupt to 0 in the
# built binary. See rust/scripts/fix-float-narrowing.py for the target list.
python3 "$(dirname "$0")/fix-float-narrowing.py" "${WORK}"

echo "==> Copying finished crate to ${DEST}"
# NOTE: ${DEST} (rust/) also holds hand-maintained tooling (this script,
# compare-with-c.sh, the Dockerfile, README.md, ...) since the crate root and
# the migration scripts share one flattened directory. Only replace the
# c2rust-owned crate files, not the whole directory.
rm -rf "${DEST}/src"
rm -f "${DEST}/Cargo.toml" "${DEST}/Cargo.lock" "${DEST}/build.rs" "${DEST}/lib.rs"
cp -r "${WORK}/src" "${WORK}/Cargo.toml" "${WORK}/Cargo.lock" "${WORK}/build.rs" "${WORK}/lib.rs" "${DEST}/"
rm -f "${DB}"

echo "==> Transpile complete. Crate at ${DEST}"
echo "    modules: $(find "${DEST}/src" -name '*.rs' | wc -l)"
