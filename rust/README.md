# otfcc → Rust migration (c2rust)

This directory holds the Rust port of otfcc, produced by transpiling the C
sources with [c2rust](https://c2rust.com/) and tracked in
[issue #2](https://github.com/MaruTama/otfcc/issues/2).

**`rust/` is committed source, not a build artifact.**
The plan is to eventually delete the C implementation and let this Rust code
be the real, standalone implementation — so unlike a typical generated-code
directory, it's checked into the repo and (from Phase 2 onward) will be
hand-edited directly. Only `rust/target/` (the actual
compiled binaries) is gitignored.

The C-side fix for [issue #1](https://github.com/MaruTama/otfcc/issues/1)
(large `gsub_alternate` corruption) landed separately and is carried into the
committed Rust source, since it was part of the C sources at transpile time.

**Repository layout**: the C implementation (`lib/`, `src/`, `dep/`,
`include/`, `premake5.lua`, `quick.make`, `_vc*.bat`) lives under `c/` at the
repo root, mirroring how the Rust port is self-contained under
`rust/`. This is deliberate: once Rust is trusted as the sole
implementation, retiring C is a single `rm -rf c/` (see "Next steps" below).
`tests/`, `build/`, and `bin/` stay at the repo root — both sides build
against and are verified against those same shared fixtures/outputs, so
splitting them would mean duplicating the font payloads. `c/premake5.lua`
and `c/quick.make` are written to still produce `build/` and `bin/` at the
repo root (not nested under `c/`), so nothing downstream (this directory's
scripts, CI) had to change its output-path assumptions. This directory is
flattened: the crate root (`Cargo.toml`, `src/`) and the migration tooling
(`scripts/`, this README) live side by side directly in `rust/` — there is no
separate `transpiled/` subdirectory.

## Crate layout

The crate uses the standard cargo layout. c2rust originally emitted every
module under a `src::lib::` / `src::dep::r#extern::` / `src::src::` tree that
mirrored the C source directories, so paths read
`crate::src::lib::support::stdio::FILE` and each module lived in a directory
named after itself (`src/lib/support/buffer/buffer.rs`). That is now:

```
rust/Cargo.toml
rust/src/lib.rs                  crate root: a flat list of `pub mod`
rust/src/bin/{otfccdump,otfccbuild}.rs
rust/src/ffi/dll.rs              the four public extern "C" functions
rust/src/vendor/{sds,json,json_builder,emyg_dtoa,uthash}.rs  third-party C
rust/src/version.rs              MAIN_VER / SECONDARY_VER / PATCH_VER
rust/src/{bk,consolidate,font,json_reader,json_writer,libcff,logger,
          otf_reader,otf_writer,support,table,vf}[.rs|/]
```

Every directory has a sibling module file (`src/support.rs` for
`src/support/`), the 2018-style layout with no `mod.rs`, so paths are now
`crate::support::stdio::FILE`. `push_stopwatch` lives in
`src/support/stopwatch.rs` rather than `src/bin/` — the two binaries link
against it as a library symbol, and anything directly under `src/bin/` would
be treated as a third binary target. `table/meta/type.rs` and
`table/vdmx/type.rs` became `types.rs`, which removes the `r#type` escaping
from those paths.

## Everyday use: just build and test

CI (`.github/workflows/rust.yml`) and local development do **not**
re-run c2rust. They just build the committed Rust source and check it:

```bash
pip install fonttools && python3 rust/scripts/make-test-variable-font.py
                                   # optional: adds the gvar payload below
./rust/scripts/build-crate.sh   # cargo build --release + cargo test
./rust/scripts/check-abi.sh     # the exported C ABI surface is unchanged
./rust/scripts/compare-with-c.sh # build C with clang, compare byte-for-byte
./rust/scripts/run-cycles.sh    # dump/build cycles against the Rust binaries
node rust/scripts/compare-roundtrips.js
```

(`./rust/scripts/test.sh` = `build-crate.sh` + `check-abi.sh` +
`run-cycles.sh`, for convenience.) None of this needs Docker, c2rust, or a
specific architecture — plain `rustup`/`cargo` plus a C compiler.

The toolchain is **stable Rust, pinned** (`rust-toolchain.toml`: 1.97.1,
edition 2024); rustup installs it on first build. The pin is deliberate, for the
same reason `Cargo.lock` is committed and used with `--locked`: "byte-identical
to C" must not depend on whichever compiler happened to be current. It is also
what makes `[lints.rust] warnings = "deny"` safe — the crate has claimed 0
warnings since Phase 2, and now the build enforces it, without a future stable
being able to break the build by inventing a lint. Bumping the pin is where new
warnings get dealt with, on purpose.

## The public ABI is four functions — and why that matters

otfcc's real public C ABI, as far as anything outside this crate can observe,
is exactly four symbols — the `otfccdll` API that `test-dll.py` drives through
ctypes:

```
otfccbuild_json_otf   otfcc_get_buf_len   otfcc_get_buf_data
otfccbuild_free_otfbuf
```

plus the two CLI binaries. Everything *else* the cdylib exports (574 symbols
at the time of writing, listed in `scripts/abi-exports.txt`) is exported only
because c2rust marked every non-`static` C function `#[no_mangle]`. Those are
internal cross-module calls that happen to have external linkage; no consumer
links against them.

This is worth stating explicitly because it defines the boundary of what the
idiomatization is allowed to change. `compare-with-c.sh` and `test-dll.py` run
the C and the Rust implementations as **separate processes / separate shared
libraries** and compare their *output*; at no point do C code and Rust code
share a struct inside one process. So:

> **Byte-identical output is the invariant. ABI-compatible internals are
> not.** Internal struct layouts, `#[repr(C)]`, field order, and the
> `#[no_mangle]` attribute on internal functions can all change freely, as
> long as the four symbols above keep working and the built fonts stay
> byte-for-byte identical.

That is what makes the remaining work (replacing the C-style containers,
`sds` strings, and `malloc`/`free` ownership with `Vec`/`String`/`Box`)
possible at all, rather than being blocked by an imagined ABI contract.

`check-abi.sh` keeps this honest: it fails if any of the four goes missing,
fails if a *new* symbol appears un-recorded (an internal helper accidentally
becoming public), and fails if a recorded symbol *disappears* until the
snapshot is refreshed with `check-abi.sh --update`. So each batch of newly
internalized symbols shows up as a reviewable diff of `abi-exports.txt`
instead of passing unnoticed. The three `__ctype_*_loc` shims are excluded
from the snapshot because they are `#[cfg(target_os = "macos")]`-only (on
glibc they come from libc), making them the one genuinely platform-dependent
part of the surface.

## Regenerating the Rust source — retired, kept for the audit trail

**The transpile pipeline now lives in `scripts/archive/` and must not be
run.** It was already destructive when Phase 2 began (`transpile.sh` does
`rm -rf rust/src` and copies fresh c2rust output over it, discarding every
hand-idiomatized file); the standard cargo layout adopted in Phase 3 means its
output no longer even maps onto this crate's directory structure. A C-side
change is ported to Rust by hand from here on, mirroring whatever the
equivalent C diff does.

It stays in the repository rather than being deleted because it documents
*why* parts of this crate look the way they do — in particular
`fix-float-narrowing.py`'s call-site list, which is the reason for the
deliberate `EXPR as int16_t as uint16_t` double casts that must never be
"simplified" away.

<details>
<summary>The original procedure (do not run)</summary>

1. Generate the compilation database (macOS shown; `OS=linux` on Linux):

   ```bash
   ./rust/scripts/gen-compile-commands.sh
   ```

2. Build the transpiler image once (native arm64; slow — it compiles c2rust
   from source):

   ```bash
   docker build -t otfcc-c2rust -f rust/scripts/archive/Dockerfile rust/scripts/archive/
   ```

3. Transpile. The repo is mounted at its **host path** so the absolute paths
   in `c/compile_commands.json` resolve unchanged. This overwrites the crate
   files under `rust/` (not its hand-maintained scripts) — review the diff
   before committing.

   ```bash
   docker run --rm -v "$PWD":"$PWD" -w "$PWD" \
       --entrypoint bash otfcc-c2rust rust/scripts/archive/transpile.sh
   ```

4. Verify it still builds and matches C (see "Everyday use" above), then
   commit the diff.

### Why native arm64, and this base image

c2rust is best supported on Linux with a known LLVM, but building it from a
stock, emulated `linux/amd64` image proved brittle: an amd64 image under QEMU
hits a fatal GNU Make jobserver bug ("write jobserver: Bad file descriptor")
building c2rust's vendored tinycbor. The image therefore targets the host's
**native arm64** instead, which avoids emulation entirely.

c2rust also cannot represent the SIMD vector-math declarations in aarch64
glibc's `<bits/math-vector.h>` (`_ZGVnN..` libm variants) and panics ("Could
not find CTypeId … in TypedAstContext"); c2rust's frontend ignores
`--target`/`--sysroot`, so it always parses the native aarch64 headers. otfcc
never uses vectorized libm, so the image replaces that header with glibc's
empty SIMD-decl stubs (`#include <bits/libm-simd-decl-stubs.h>`), which still
defines the macros `mathcalls.h` needs.

The image is Ubuntu 24.04 (LLVM/Clang 17) with **c2rust 0.22.1** — clang-18
tripped an unrelated c2rust ast-exporter bug on some ordinary code, clang-17
didn't.

Once transpiled, though, the resulting Rust source is **not**
architecture-specific: building the arm64-transpiled source on amd64
(verified) produces output byte-identical to a same-machine C build. Only the
transpile step itself needs arm64.

> If the sandboxed shell makes `docker` hang on "load metadata" (credential
> helper), export `DOCKER_HOST=unix://$HOME/.docker/run/docker.sock` and
> `DOCKER_CONFIG=<dir with config.json = {}>` to bypass the helper for
> anonymous pulls.

### Post-transpile fixups (applied automatically by `transpile.sh`)

- `fix-transmute-abi.py` — c2rust sometimes wraps a zero-arg,
  struct-returning function-pointer-field call in a `transmute` that drops
  the `unsafe extern "C"` ABI when the C code assigns the result through an
  outer typedef-alias cast. This corrupted every such struct-by-value return
  (observed: a Handle's `name` field ending up holding an unrelated
  function's address → `free(): invalid pointer`). Strips the transmute,
  calling the function pointer directly.
- `fix-float-narrowing.py` — c2rust mistranslates C's *implicit*
  `pos_t` (f64) → `uintN_t` narrowing conversion at `bufwriteNNb()` call
  sites lacking an explicit intermediate cast in the C source. It emits
  `x as uint16_t`, using Rust's *saturating* float→unsigned semantics
  (negative → 0) instead of C's actual behavior (convert through a signed
  int, reinterpret the bits: `-41.0 → 0xFFD7`, decodes back to -41). This
  silently zeroed negative `hmtx.lsb` / `vmtx.tsb` /
  `VORG.defaultVerticalOrigin` in the built font. Fixed at the 5 confirmed
  call sites.
- Also adds `cdylib` to the crate's `crate-type` (the C build makes
  `otfccdll.c` a `SharedLib`; c2rust's default `staticlib`+`rlib` alone can't
  be linked against as a shared library) and clamps the vendored dtoa's
  `kPow10` index (a latent OOB in the C source that Rust's bounds checks
  catch — see the comment in `transpile.sh` for the full mechanism).

</details>

## Pipeline pieces

- `gen-compile-commands.sh` / `filter-compdb.js` — host-side: premake →
  `ninja -t compdb cc` → reduce to the single release-x64 C config
  (118 translation units). Was the input to the transpiler; still useful on
  its own for pointing clangd at the C sources.
- `archive/{Dockerfile,transpile.sh,fix-transmute-abi.py,fix-float-narrowing.py}`
  — the retired c2rust pipeline. Do not run; see the section above.
- `build-crate.sh` — builds the committed crate (release) and runs
  `cargo test`. Needs only rustup + cargo (the pinned stable toolchain in
  `rust-toolchain.toml`) — no c2rust/Docker, works on any architecture.
- `run-cycles.sh` — runs the same dump/build cycles as `c/quick.make`'s
  round-trip targets against an already-built crate, for every payload the C
  test suite covers (minus two fonts that crash both C and Rust with a stack
  overflow — see Status below), plus the `otfccdll` cdylib test if built.
- `check-abi.sh` — verifies the cdylib's exported C ABI surface against
  `abi-exports.txt` (see "The public ABI is four functions" above).
  `--update` refreshes the snapshot.
- `test.sh` — convenience wrapper: `build-crate.sh` + `check-abi.sh` +
  `run-cycles.sh`.
- `compare-roundtrips.js` — runs `tests/ttf-roundtrip-test.js` over every
  payload produced and reports a single pass/fail summary.
- `compare-with-c.sh` — builds the C toolchain **with clang** and compares
  its output against an already-built Rust crate byte-for-byte, on the same
  machine. Both directions: `otfccdump`'s JSON, and the font that `otfccbuild`
  produces from the C dump. Defaults to clang, not gcc: c2rust's transpile is based on parsing
  with clang's AST, and gcc vs clang produce measurably different
  floating-point rounding in this codebase (verified: a gcc build differs
  byte-for-byte from a clang build of the *identical* source on the
  *identical* machine, while clang builds match byte-for-byte across
  architectures and OSes — arm64 Linux, amd64 Linux, and arm64 macOS all
  agree). Using gcc here would flag that gcc/clang difference as a false
  Rust-vs-C mismatch. Picks the premake/ninja binaries and the `quick.make`
  target from `uname`, so it works natively on macOS as well as Linux — and
  because `build/`/`bin/` are shared with the Linux verification container
  (which bind-mounts the repo at its host path), it first checks whether the
  existing object tree was built for the *other* OS and clears it if so. A
  stale cross-OS tree otherwise shows up either as "file format not
  recognized" from the linker or, worse, as *every* payload mismatching —
  which looks exactly like a Rust regression and isn't one. It also compares
  one *forged* payload, dump only: `make-test-unknown-lookup.py` rewrites every
  GSUB/GPOS lookup in `iosevka-r.ttf` to a format number neither table defines,
  because no real payload has one and the unknown-type path is otherwise
  unchecked (see `otl_LookupType` below). Both toolchains then refuse to
  *build* the resulting JSON, identically, which is why that half is skipped.
- `make-test-unknown-lookup.py` — generates the payload above from a committed
  one. Standard library only, unlike `make-test-variable-font.py`.
- `dll-arch-check.sh` — sourced by `run-cycles.sh`/`compare-with-c.sh` to
  detect when python3 cannot `dlopen` the crate's cdylib at all, so the
  ctypes check is skipped with a stated reason instead of failing. Normally the
  two match, since `rust-toolchain.toml`'s `channel` resolves to rustup's own
  host triple. What breaks it is a *Rosetta rustup* on an Apple Silicon Mac: an
  `x86_64-apple-darwin` rustup emits an x86_64 dylib while python3 is arm64, and
  no Rosetta python3 exists to load it. Installing the native toolchain
  alongside it (the command is in that script's header) fixes it; the check also
  runs for real in the arch-matched Linux container and in CI.
- `make-test-variable-font.py` — builds a minimal, self-contained variable
  font (fvar + gvar, one `wght` axis, two masters, via fontTools APIs — no
  external download) to exercise the gvar delta-application path, which none
  of `tests/payload/*.ttf` has. Needs `fontTools` (`pip install fonttools`);
  writes `build/gvar-test.ttf`. CI generates this before every run; locally,
  `compare-with-c.sh`/`run-cycles.sh` pick it up automatically if present and
  skip it (with a message) otherwise.
- `test-dll.py` — exercises the `otfccdll` C API (`otfccbuild_json_otf` /
  `otfcc_get_buf_len` / `otfcc_get_buf_data` / `otfccbuild_free_otfbuf`) via
  `ctypes`, against either the C `libotfccdll.{dylib,so}` or the Rust
  `cdylib`, to compare output byte-for-byte. `compare-with-c.sh` runs this
  against both libraries on the same JSON input and diffs the result.

## Status: Phase 1 complete

The committed crate (`Cargo.toml`, `src/lib.rs`, `build.rs`, the
`otfccdump`/`otfccbuild` binaries, `otfccdll` compiled into the lib) **builds
and its round-trips are byte-for-byte correct**:

- The Rust binaries pass `tests/ttf-roundtrip-test.js` on all 6 TTF payloads,
  the CFF payload `KRName-Regular`, both from-JSON CFF payloads, and the
  generated gvar payload (see below) — 10 payloads total.
- Building the *same* input JSON with the C toolchain and the Rust toolchain
  produces `.ttf`/`.otf` files that are **byte-identical** (`cmp` shows 0
  differing bytes) for all 8 directly-comparable payloads.
- **Variable-font (gvar) coverage**: none of `tests/payload/*.ttf` has an
  `fvar` table, so the gvar delta-application path (`applyPolymorphism` in
  `c/lib/table/glyf/read.c`) was untested by the payload matrix above.
  `make-test-variable-font.py` closes that gap — CI generates the font fresh
  every run and it's part of the regular byte-comparison and round-trip
  matrix. C and Rust are byte-identical at every stage of a full two-cycle
  round trip (original dump, build 1, post-build dump 1, build 2, post-build
  dump 2). This also surfaced a pre-existing otfcc limitation (`otfccbuild`
  doesn't reconstruct `fvar`/`gvar` from JSON with delta-annotated
  coordinates), but it reproduces identically in C and Rust, confirming it's
  an existing gap in otfcc's build-side variable-font support, not a
  migration regression.

Two fonts (`Cormorant-Medium.otf`, `WorkSans-Regular.otf`) crash both the C
*and* Rust `otfccdump` with a stack overflow — a pre-existing bug in the C
CFF interpreter (verified: the C binary also exits SIGSEGV on them), not
something the Rust translation introduced or needs to fix here.

**CI checks five things**: the crate builds and `cargo test` passes (c2rust
generated no tests; hand-written coverage now starts with the `cvec` capacity
arithmetic and the `caryll_Buffer` byte-order/cursor contract, and grows with
each idiomatized module), the exported C ABI surface is unchanged
(`check-abi.sh`), its output (including the gvar payload and the `otfccdll`
cdylib) is byte-identical to the C toolchain's (`compare-with-c.sh`), and the
round-trip stability tests pass (`compare-roundtrips.js`). It's a single
`ubuntu-latest` job, no Docker.

**`otfccdll` (cdylib) coverage**: `compare-with-c.sh` calls
`otfccbuild_json_otf` through both the Rust `.so` and the C `.so`/`.dylib`
(via `test-dll.py`/ctypes) on the same JSON input and diffs the results. The
DLL API doesn't accept `--keep-modified-time`, so
`head.created`/`modified`/`checkSumAdjustment` legitimately vary run to run
even between two C-only invocations — the check compares the Rust-vs-C byte
diff against that same-library run-to-run baseline instead of requiring a
plain `cmp` pass.

## Status: Phase 2 in progress (idiomatization)

The crate went from raw c2rust output (317 compiler warnings, exclusively
`unsafe extern "C" fn`s full of manual pointer-offset loops and redundant
casts) to a **0-warning build** with `support/buffer.rs`,
`bk/bkblock.rs`, and `bk/bkgraph.rs` (the "support/low-level I/O"
layer the original plan called out as the correct starting point — it's the
most-depended-on code and the least entangled with the rest of the crate)
rewritten to idiomatic bodies:

- **Crate-wide, byte-preserving mechanical sweep**: `cargo fix` removed 151
  unused-variable and 50 unnecessary-parens warnings; a script-driven pass
  stripped 113 redundant `unsafe` blocks (struct-of-fn-pointer vtable
  initializers that never needed the keyword); another replaced all 434
  `(true|false)_0 != 0` occurrences (c2rust's rename of C's `true`/`false`
  macros, since they collide with Rust keywords) with plain `true`/`false`.
- **buffer.rs**: the eight `bufwrite16l/b`..`bufwrite64l/b` functions'
  manual per-byte shift/mask/store expansions became
  `x.to_le_bytes()`/`to_be_bytes()` through one shared `buf_push_bytes()`
  helper; `bufwrite_sds/_str/_bytes/_buf/_bufdel` build a slice and go
  through the same helper instead of a raw `memcpy` call; index `while`
  loops became `for` loops throughout.
- **bkblock.rs** / **bkgraph.rs**: triple-cast type comparisons
  (`x as c_uint == Y as c_int as c_uint`, where both sides were already the
  same type) became `match`/`==` on the named `bk_CellType` consts directly;
  every index `while` loop became a `for` loop, including
  `dfs_attract_cells`'s reverse-iteration underflow-sentinel trick
  (`j = length; loop { let fresh = j; j -= 1; if fresh == 0 { break } ...
  }`), which became `for j in (0..length).rev()`; the offsets-prefix-sum
  computation duplicated identically in three functions was factored into
  one private `compute_block_offsets()`. **The actual offset-overflow
  decision logic — `getoffset()`, `getoffset_untangle()`, and
  `try_untabgle_block`'s bounds check — was deliberately left untouched
  beyond an equivalent-by-construction rewrite** (`offset < 0 || offset >
  0xffff` → `!(0..=0xffff).contains(&offset)`), since this module is where
  issue #1's fix lives; verified with `tests/gsub-alternate-large-test.js`
  (the dedicated issue #1 regression test) run directly against the Rust
  build, not just the standard payload matrix.
- **alloc.rs** (new): `__caryll_allocate_clean`/`__caryll_reallocate` were
  duplicated byte-for-byte in every file that used them (c2rust's per-
  translation-unit expansion of a `static inline` C header); factored out to
  one shared module and wired into the three files above. Neither helper was
  ever `#[no_mangle]`, so this changes no ABI. The ~47 other files with their
  own copy (and the still-per-file `read_8u/16u/24u/32u` family, unused in
  support/bk) are unchanged — rolling this out further is a separate,
  larger, more carefully-reviewed pass.

- **`otf_reader`/`otf_writer`** (`otf_reader.rs`, `unconsolidate.rs`,
  `otf_writer.rs`, `stat.rs`): the second module idiomatized, after
  support/bk. Two functions (`decideFontSubtypeOTF`,
  `statMaxContextOTL`) turned out to be c2rust's translation of otfcc's own
  `foreach(item, vector) { ... }` macro — `__fortable_*`/`__caryll_index*`/
  `keep*` variables simulating a single-iteration inner loop purely so the
  macro body can `break`/`continue`; traced by hand against the original C
  source and collapsed to plain `for` loops (`statMaxContextOTL` had 4
  nested occurrences). `unconsolidate_chaining` (expands a lookup's
  multi-rule "poly" chaining representation into one canonical subtable per
  rule — real ownership-transferring memory management, ~90 lines) had a
  first pass computing a `totalRules` count that's **never read afterward,
  confirmed dead in the original C too** (`c/lib/otf-reader/unconsolidate.c`);
  removed. Every other bounded loop across the four files became `for`;
  linked-list-style hash-table traversals (`while !p.is_null() { ...; p =
  next }`) were left as `while`, consistent with the rest of this pass.
  Deliberately skipped: `statOS_2UnicodeRanges` (~450 lines, but not a loop
  — one `if unicode-in-range { set bit N }` per OpenType-spec Unicode block,
  already flat; retyping ~150 bit positions by hand is high risk for a
  change that's mostly whitespace). Caught one real mistake mid-rewrite:
  `onCurve` is `int8_t`, not `bool` — a naive `onCurve as uint8_t` would NOT
  reproduce the original's "normalize any nonzero value to exactly 1"
  behavior; fixed to `(onCurve != 0) as uint8_t` before it was ever
  committed. Verified real coverage before touching
  `unconsolidate_chaining`: dumped all 6 TTF payloads and confirmed
  NotoNastaliqUrdu-Regular/iosevka-r/BungeeColor-Regular_colr_Windows/
  Molengo-Regular all have `gsub_chaining` (and NotoNastaliqUrdu also
  `gpos_chaining`) lookups, so the standard byte-comparison suite actually
  exercises the rewritten function.

Every commit in this pass was verified against the full byte-comparison
matrix (`compare-with-c.sh`), all round-trip payloads
(`compare-roundtrips.js`), and — for the bk and unconsolidate_chaining
changes specifically — the issue #1 golden regression test, before moving
to the next file.

## Status: Phase 3 in progress (structure, then safe Rust)

Phase 2 idiomatized function *bodies*. Phase 3 attacks the thing that made
that work harder than it should have been: this was still 123 independent C
translation units rather than one crate. c2rust processes one `.c` file at a
time and has no way to say "that type is declared over there", so it
redeclared every type in every file that touched it — 8,090 declarations of
458 distinct types, 30.8% of the crate's lines. Because Rust types are
*nominal*, those were 8,090 genuinely distinct types, which is why PR #17's
trait work had to erase its boundaries to `*mut c_void`.

Landed so far, each commit verified against the full matrix on macOS and in
the CI-matching Linux container:

- **The public ABI is guarded** (`check-abi.sh`, see above). Each commit reports
  the same export list as the one before it, which is what makes "this changed
  nothing observable" a checked claim rather than an assertion. The count only
  moves when a symbol is deliberately internalized, and then the diff is small
  enough to read: 574 at the start, 554 now.
- **Standard cargo layout**: `src/lib.rs` + `src/bin/` + `src/ffi/` +
  `src/vendor/`, replacing c2rust's `src::lib::` / `src::dep::r#extern::` /
  `src::src::` scaffolding. See "Crate layout" above.
- **C's integer typedefs are gone**: 1,227 declarations of `uint16_t`,
  `size_t`, `__int32_t` and friends replaced by `u16`/`usize`/`i32`.
  `c_int`/`c_long`/`c_char`/`c_void` stay — their width really is
  platform-dependent, and issue #14 (Windows, LLP64) depends on that.
- **The C library comes from `libc`**: 648 duplicated declarations delegated,
  and `FILE` is now `libc::FILE` (opaque) instead of a hand-copied glibc
  `_IO_FILE` struct that didn't describe macOS at all. That removed the
  crate's last `extern type`, so **`#![feature(extern_types)]` is gone** — the
  first of the three nightly features to go, and the crate is on stable now
  (see below).
- **The base types have one declaration each**: `sds` (was 95 copies),
  `caryll_Buffer` (80), `otfcc_ILogger`/`ILoggerTarget`/`LoggerType` (79),
  `otfcc_Options` (78), and otfcc's scalar vocabulary — `glyphid_t`, `pos_t`,
  `tableid_t`, … — collected into `support/primitives.rs` along with the
  explanatory comments from `c/include/otfcc/primitives.h`.
- **The vendored JSON types too** (59 copies each), including the five
  anonymous struct/unions inside `_json_value`. Those could not be deduped by
  name: c2rust numbers anonymous types per translation unit, so the same type
  is `C2RustUnnamed_0` in 52 files and `C2RustUnnamed_4` in 7 others, while
  those names mean unrelated things in the 46 files that don't touch json.
  They are matched by field list and renamed (`json_value_payload`,
  `json_array_value`, `json_object_value`, `json_string_value`,
  `json_value_reserved`).
- **All 30 remaining anonymous types have real names**, identified the same
  way but *recursively*, because they nest: the `vq_Segment.val` union's
  `delta` member is itself anonymous, so two copies of that union differ
  textually while being one type. Named from the C declaration that produced
  them — `otl_ChainingBody`/`otl_ChainingRuleSet`, `vq_SegmentValue`/
  `vq_SegmentDelta`, `bk_CellValue`, the `cff_*Body` format unions,
  `glyf_PackedPointRun`. The anonymous *enums* needed their constants folded
  into the signature: `pub type C2RustUnnamed_N = c_uint` is identical text
  for all 41 of them, so the logger's verbosity levels and CFF's operator
  tables would otherwise have hashed alike.
- **Every domain type has one declaration**, in the module matching the C
  header that declares it: `otl.h` → `table::otl` (47 types, 1,735 copies),
  `uthash.h` → `vendor::uthash`, `font.h` → `font::caryll_font`. 7,747
  declarations removed; the crate went from 178k lines to 137k.
- **The platform's own types come from `libc`**, which is where the last 58
  duplicated declarations went: `timespec` (c2rust had copied glibc's struct
  and its private `__time_t`/`__syscall_slong_t` typedefs into every file that
  timed anything — the same mistake as the hand-copied `_IO_FILE`), plus
  `time_t` and `SEEK_SET`. `getopt_long`'s `struct option` is the exception and
  stays in `support::getopt`: libc declares it for the BSDs, Apple, Solaris and
  Android but **not** for `*-unknown-linux-gnu`, so delegating it would break
  CI. The whole `va_list` chain turned out to be dead — 19 declarations whose
  only references were each other, and carrying the *AArch64* register-save
  layout because the transpile ran on arm64 while the crate builds x86_64.

- **The C variadics are gone, and with them the nightly requirement.** All
  eight were replaced: `bufninit`/`bufnwrite8` take a `&[u8]`,
  `bk_new_Block`/`bk_push` take a `&[bk_Item]`, `cffdict_input` split into the
  two functions its `t` argument was already selecting between, `zroll` takes
  `&[bool]`, and `sdscatprintf`/`sdscatfmt` became the `sdsbuild!` macro over a
  `SdsPart` trait. Each of these had passed a count *and* a list, or a type tag
  *and* a value, with nothing checking that the two agreed — `arity` against the
  number of operands, `n` against the number of bytes, `%d` against what was
  actually pushed. All 252 `sdscatprintf`/`sdscatfmt` call sites turned out to
  use a literal format string, so nothing needed a run-time `printf` at all.
  `#![feature(c_variadic)]` is gone and `raw_ref_op` has been stable since
  1.82, which is what unblocked the toolchain move below.

  Two things not to undo there. Text is appended as **bytes**: `%s` arguments
  are C strings out of a font file, so `format!` would mean a `CStr` → `str`
  conversion and a glyph name that isn't valid UTF-8 would come out changed
  (U+FFFD) — passing every payload in the test set and failing on the first
  Latin-1 glyph name. And `support/stopwatch.rs` still calls libc's `snprintf`
  for its one `%g`, because Rust has no `%g` and this crate deliberately does not
  format floats itself (JSON numbers go through the vendored `emyg_dtoa` so their
  spelling matches the C build).

  This pass also exposed a latent use-after-free in `name.c:188`:
  `sdsgrowzero(copyright, COPYRIGHT_LEN)` discards the pointer it returns, and
  had got away with it only because `sdscatprintf` over-allocated by 2× — a
  two-byte margin. Building the same string in pieces allocates less, the growth
  reallocates, and the stale pointer aborts in libmalloc. Intermittently, since
  whether `realloc` moves a block depends on the heap.

- **Almost nothing is `static mut` any more**: 158 of the 199 became plain
  `static`. Exactly *two* were ever assigned to; the rest are C's file-scope
  constants — 33 vtable packages, lookup tables, single numeric limits — and
  c2rust had no way to tell, because C doesn't either. Which ones could change
  was decided by the compiler, not by reading the types: convert them all, then
  revert what rustc rejects. A struct of fn pointers is `Sync` and the same
  struct with one raw pointer field is not, and the two declarations look
  identical. The 19 that stayed held raw pointers — `[*const c_char; N]`
  tables of JSON key names — and are now `[&CStr; N]`, which *is* `Sync`; see
  the next bullet. Also gone: the
  `.init_array` / `.CRT$XIB` / `__DATA,__mod_init_func` hack that existed to
  assign `json_builder_extra` before `main`, which in Rust is a `const`
  (`size_of` is a compile-time question).

- **Stable Rust, edition 2024.** Two of the edition's changes were mechanical and
  both are about honesty at the FFI boundary: `#[no_mangle]` →
  `#[unsafe(no_mangle)]` (575 sites), because a chosen symbol name can silently
  collide with another object's; and `extern "C" { … }` →
  `unsafe extern "C" { … }` (107 blocks), because writing down a foreign
  signature is the unsafe act and getting it wrong is UB at every call site.

- **The label tables are `&CStr` slices.** Sixteen `static mut
  [*const c_char; N]` tables — eleven naming the bits of a flag field, five
  indexed by a code (CFF standard strings, Macintosh glyph names, TrueType
  instruction mnemonics) — became `static [&CStr; N]`. The flag tables were
  NUL-terminated so that `otfcc_dump_flags`/`otfcc_parse_flags` could walk them
  without being told a length; every one was dense up to a single trailing null
  (checked entry by entry), so a slice carries the same information and the
  sentinel is gone.

  Those two helpers are `static inline` in `json-funcs.h`, so there were three
  copies of each plus seven of the `json_obj_getbool` they call, all textually
  identical; they live in `support/json_funcs.rs` now. Two copies had been
  reaching another module's tables by declaring them
  `extern "C" { static mut X: [*const c_char; 0] }` — a length-0 array
  declaration typechecks against anything, and it went through the linker
  rather than the module system.

  This is also where `compare-with-c.sh` grew a check it should always have
  had: the canonical JSON both builds consume is dumped by the *C* tool, so
  nothing compared `otfccdump`'s own output between implementations. A changed
  JSON key would only have been caught if it also changed the build result.
  It is compared now, and matches byte-for-byte on all eight payloads.

- **`otl_LookupType` is a newtype, not an `enum`** — the one place where the
  obvious modernization is the wrong one, so it is written down here.

  A lookup's type is read as `read_16u(data) + base`, where `base` is 16 for
  GSUB and 32 for GPOS, giving one number that names both the table and the
  format. otfcc does not validate it. A type it does not recognise gets no
  subtable reader (`otfcc_readOtl_subtable` returns NULL), the lookup dumps as
  `{}`, and if the feature list gave it no name it is named after **the raw
  number in hex** — `lookup_1a_23`. So the value reaches the output, and
  anything in `16..=65551` can appear. A `#[repr(u32)]` enum cannot hold that;
  `transmute`ing it in is UB; and rejecting it at the boundary would change
  what otfcc writes for such a font.

  What the newtype does buy: the compiler now tells a lookup type apart from
  every other 32-bit quantity in the OTL code, the two file-derived
  construction sites go through `from_file`, and `tableNames` — a 42-entry
  `static mut` array of C string pointers with 23 NULL holes, indexed by the
  type — is a `name()` method. Its 26 uses were all *constant* indices sitting
  next to the same constant as an argument, so the two dispatch helpers
  (`_declare_lookup_dumper`, `_declareLookupParser`) each lost a parameter that
  was a function of another one. Those strings are the JSON's `"type"` values,
  and they are not the constants' own spelling (`gpos_mark_to_base` vs
  `otl_type_gpos_markToBase`), so a test pins all twenty.

  No payload has an unknown lookup type, which is why
  `make-test-unknown-lookup.py` now forges one and `compare-with-c.sh` compares
  the dump of it. Byte-identical — 55 lookups, all `{}`, all named after the
  forged number.

  The third, `unsafe_op_in_unsafe_fn`, fires **48,000 times**, and it is the one
  worth having: it separates "this function is unsafe to call" from "this line is
  the unsafe part". But the only mechanical fix is wrapping 2,300 function bodies
  in `unsafe {}` — a 190,000-line reindentation that would bury every real change
  for the rest of the migration. So it is allowed per *file*, in the 120 files
  that need one. Not once at the crate root: lint levels inherit into child
  modules, so one line in `lib.rs` would silently cover the 21 files that are
  already clean. This way
  `grep -rc "allow(unsafe_op_in_unsafe_fn)" rust/src` is the remaining-work
  count, each Stage 6 PR deletes the line from the file it finishes, and the
  compiler keeps that file honest from then on.

  A side effect worth as much as the toolchain change: this Mac can build the
  crate natively, so the `otfccdll` ctypes comparison — SKIPped on Apple Silicon
  since PR #17 and checked only in the Linux container and in CI — now runs on
  macOS too, and matches the C dylib to the byte.

**Nothing in the crate is now declared twice**: 896 declarations, 0 duplicated,
down from 8,090 declarations of 458 types.

Every dedup step verifies that all copies are textually identical before
deleting any of them; a name whose copies disagree is reported and left alone,
since "same name, different type" is exactly the failure a mechanical pass
could otherwise introduce in silence. Two names did disagree and both were
real: `table_TSI5`/`otl_ClassDef`, where 17 files carried the inverse of
`TSI5.h`'s `typedef otl_ClassDef table_TSI5` because c2rust made whichever
name it met first the struct; and `otfcc_IFontBuilder`/`otfcc_IFontSerializer`,
still live after PR #17's trait work because the two binaries are separate
crates that reach the readers and writers through `extern "C"`.

A second trap, from turning `match x { 2 => … }` into `match x { json_array =>
… }`: an enum variant in a *pattern* only means the variant if that name is in
scope. If it is not imported, Rust reads it as a fresh binding that matches
anything — so a single missing name in a `use` list silently turns one arm into
a catch-all and makes every later arm dead. Here rustc caught it
(`bindings_with_variant_name` is deny-by-default, and the dead arms tripped
`unreachable_patterns` under `warnings = "deny"`), but only because the match
had arms after the mis-resolved one. Add the variant to the file's `use` before
putting it in a pattern.

One trap worth knowing before the next mechanical pass: `vendor/sds.rs`
imports `__ctype_b_loc` and friends under `#[cfg(target_os = "macos")]`, since
on Linux they come from glibc. Folding an unconditional import into that gated
line compiles perfectly on macOS and leaves the names undefined on Linux —
which is also why `cargo fix`'s unused-import removals have to be re-checked
on the other platform before a commit is trusted.

## Next steps

- **Real `enum`s, the rest.** Fifteen are done — `handle_state`, the ten whose
  values the crate generates itself, `bk_CellType`, `tsi_EntryType`,
  `json_type`, `byte_types` — plus `otl_LookupType` as a newtype and
  `ttf_instructions` deleted outright. Sixteen `pub type X = c_uint` aliases are
  left, and what remains is everything a plain `enum` cannot express:
  - values that come **out of a font file**: the CFF operator and format-byte
    tables. Read what C does with an unrecognised value *before* choosing the
    shape — if it keeps the value and that value can reach the output, the
    answer is a newtype, as it was for `otl_LookupType` (see above). Never
    `transmute` — an out-of-range discriminant is instant UB.
  - **bit sets** (`glyf_PointFlags`, `glyf_ComponentFlags`,
    `glyf_OnCurveMask`, `json_GlyphOrderPass`, `ctype_class_bits`) stay bit
    sets — a newtype or `bitflags`, not an enum.
  - **one number, two names**: `cff_Value_Type` spells the same value as both
    a DICT and a CharString operator, `cff_CharsetType` has
    `UNSPECED` == `ISOADOBE` == 0. Rust says that with a variant plus an
    associated constant.
  - not enumerations at all: `cff_Type2Limits` is a table of capacity
    ceilings, `otfcc_LoggerVerbosity` an ordered threshold compared with `<=`,
    and `WORD`/`json_uchar` are plain typedefs.
- **The rest of `json-funcs.h`**: `json_obj_get` still has 32 identical
  copies, and `json_obj_getnum`/`json_obj_getint`/`…_fallback` nine each. Same
  consolidation as the flag helpers, just more of it.
- **Rust naming**, once each type exists in one place: `otfcc_Options` →
  `Options`, functions → inherent methods, fields → snake_case. Then the
  crate-level `allow(non_camel_case_types)`/`non_snake_case`/
  `non_upper_case_globals` come out, which is what turns "this is idiomatic
  now" from an opinion into something the compiler checks.
- **Then safe Rust, type by type**: `CVecRaw<T>` → `Vec<T>` first (one
  implementation backs ~37 container types), then `sds` → `String`,
  `caryll_Buffer` → `Vec<u8>`, `malloc`/`dispose` → `Box` + `Drop`, and the
  7,682 `.offset()` calls into slices and iterators. Each of those PRs should
  end by deleting its files' `allow(unsafe_op_in_unsafe_fn)`: 120 files carry
  one today, and that count is the honest measure of how much of this crate is
  still C.
- Before `c/` can be deleted, freeze each payload's expected output into
  `tests/golden/` and hand `compare-with-c.sh`'s job over to it — otherwise
  removing C removes the safety net that makes all of this checkable.
