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

**The C implementation is gone.** `c/` (`lib/`, `src/`, `dep/`, `include/`,
`premake5.lua`, `quick.make`, `_vc*.bat`) held the original C sources this
crate was transpiled from, and was deliberately kept side by side with
`rust/` for exactly as long as it was needed as the byte-comparison oracle
(`compare-with-c.sh`). Once `tests/golden/` was frozen from a confirmed-
matching build — dump/build output via `checksums.sha256`
(`compare-with-golden.sh`), and stderr log output via `tests/golden/log/`
(`compare-log-output.sh`) — nothing in the build or in CI needed `c/` present,
built, or even checked out any more, and it was deleted. It is still in git
history (tag/commit predating the deletion) for anyone who needs to diff
against the original C source or re-run `compare-with-c.sh` by hand; see
`rust/scripts/archive/README.md`. `tests/`, `build/`, and `bin/` stay at the
repo root — the Rust binaries build against and are verified against those
same fixtures/outputs. This directory is flattened: the crate root
(`Cargo.toml`, `src/`) and the migration tooling (`scripts/`, this README)
live side by side directly in `rust/` — there is no separate `transpiled/`
subdirectory.

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
re-run c2rust, and — since `tests/golden/` was frozen (see below) and `c/`
has been deleted — do not need it present, built, or checked out either:

```bash
./rust/scripts/build-crate.sh          # cargo build --release + cargo test
(cd rust && cargo clippy --release --all-targets --locked -- -D warnings)
./rust/scripts/check-abi.sh            # the exported C ABI surface is unchanged
./rust/scripts/compare-with-golden.sh  # compare byte-for-byte against tests/golden/
./rust/scripts/compare-log-output.sh   # compare stderr output against tests/golden/log/
./rust/scripts/run-cycles.sh           # dump/build cycles against the Rust binaries
node rust/scripts/compare-roundtrips.js
```

(Clippy has no dedicated wrapper script; `[lints.clippy]` in `rust/Cargo.toml`
is the allow-list of c2rust-transpile-shaped lint categories deferred to a
later Phase 5 stage — see "Next steps" below — so anything not on that list
is a hard failure under `-D warnings`.)

(`./rust/scripts/test.sh` = `build-crate.sh` + `check-abi.sh` +
`compare-with-golden.sh` + `compare-log-output.sh` + `run-cycles.sh`, for
convenience.) None of this needs Docker, c2rust, a C compiler, or a specific
architecture — plain `rustup`/`cargo`.

Two more checks exist but aren't part of `test.sh` and aren't merge gates —
`rust/fuzz/` (cargo-fuzz; needs a pinned nightly, its own toolchain file, see
`rust/fuzz/README.md`) and `cargo +nightly-2026-08-17 miri test --lib --
--test-threads=1`. Both are wired into CI as advisory
(`continue-on-error: true`) jobs; see "Next steps" below for why and what
they've already found.

If you're changing behavior in a way that's meant to keep matching what C
used to do (as opposed to a deliberate, intentional divergence — which is
explicitly permitted from Phase 5 onward, see the plan linked from the
issue), `scripts/archive/compare-with-c.sh` still exists for that, but needs
`c/` restored from git history to run (`git show <pre-deletion-commit>:c` or
checking out a tag before the deletion) — see `scripts/archive/README.md`.
Confirm with it, then run `generate-golden.sh`/`generate-log-golden.sh` to
refresh `tests/golden/` and commit the result alongside the change that
motivated it. See "CI decoupled from C" further down for the full story of
how the dump/build half of this moved; the log-output half moved the same
way, later (see "Next steps").

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

plus the two CLI binaries. And those four are now the *only* symbols the cdylib
exports — `scripts/abi-exports.txt` is four lines long.

It used to be 553. c2rust marked every non-`static` C function `#[no_mangle]`,
and the crate then found **1,086 of its own items — 895 functions and 191
vtable statics — through those C symbol names** rather than through Rust paths:
one file wrote `#[unsafe(no_mangle)] pub unsafe extern "C" fn bufwrite8`, and 40
others wrote `extern "C" { fn bufwrite8(...); }` and called it. The crate was
linking to itself. Those declarations are now `use` imports and `#[no_mangle]`
is gone from everything but the four, so the export table *is* the API.

Two things followed from that, beyond the smaller surface. The declared
signatures were never checked against the definitions — an `extern "C"` block is
a promise to the linker, not to the type checker — so 1,086 boundaries had no
verification at all; converting them made rustc check every one (all matched).
And 305 of the declarations were for items the declaring file never used, dead
text that could not fail to compile because nothing was being resolved.

This is worth stating explicitly because it defines the boundary of what the
idiomatization is allowed to change. `compare-with-golden.sh`/`test-dll.py`
today (originally `archive/compare-with-c.sh`, before `tests/golden/` was
frozen and `c/` deleted — see "CI decoupled from C" below) run the checked
implementation as a **separate process / separate shared library** from
whatever it's being verified against and compare their *output*; at no point
did C code and Rust code share a struct inside one process. So:

> **Byte-identical output is the invariant. ABI-compatible internals are
> not.** Internal struct layouts, `#[repr(C)]`, field order, and the
> `#[no_mangle]` attribute on internal functions can all change freely, as
> long as the four symbols above keep working and the built fonts stay
> byte-for-byte identical.

That is what makes the remaining work (replacing the C-style containers,
`sds` strings, and `malloc`/`free` ownership with `Vec`/`String`/`Box`)
possible at all, rather than being blocked by an imagined ABI contract. Note
that the internal functions keep their `extern "C"` calling convention for now,
even without `#[no_mangle]`: many of them are stored as `extern "C" fn` pointers
in the vtable statics, so the convention comes off with those, not before.

`check-abi.sh` keeps this honest: it fails if any of the four goes missing,
fails if a *new* symbol appears un-recorded, and fails if a recorded symbol
*disappears* until the snapshot is refreshed with `check-abi.sh --update`. With
the snapshot down to four lines, "a new symbol" now means any accidental
re-export whatsoever — a `#[unsafe(no_mangle)]` added out of habit, or a
`pub extern "C"` item that escapes. The snapshot needs no per-platform exceptions:
the last one was three `__ctype_*_loc` shims this crate exported on macOS to
stand in for glibc internals the transpiled code called by name, and those are
gone — `support/ctype_compat.rs` now provides the five C-locale functions
(`c_isdigit`, `c_isspace`, `c_isprint`, `c_tolower`, `c_toupper`) that the call
sites were reaching for through the tables, so nothing is `#[cfg]`-dependent
and the exported surface is identical on both platforms. Nine of the twelve
`_IS*` classes turned out to be tested by nothing at all. The two Rust
functions that look like drop-in replacements are not:
`is_ascii_whitespace` omits `\v`, and `is_ascii_graphic` omits the space that
`isprint` includes — `ctype_matches_libc` checks all 384 possible inputs
against the platform's own libc rather than trusting the reading.

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

1. Restore `c/` from git history (e.g. `git show <pre-deletion-commit>:c`),
   then generate the compilation database (macOS shown; `OS=linux` on
   Linux):

   ```bash
   ./rust/scripts/archive/gen-compile-commands.sh
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

- `archive/{Dockerfile,transpile.sh,fix-transmute-abi.py,fix-float-narrowing.py}`
  — the retired c2rust pipeline. Do not run; see the section above.
- `archive/{gen-compile-commands.sh,filter-compdb.js}` — host-side: premake →
  `ninja -t compdb cc` → reduce to the single release-x64 C config
  (118 translation units). Was the input to the transpiler; moved to
  `archive/` along with `compare-with-c.sh` (below) when `c/` was deleted
  from the tree, and needs it restored from git history to run; see
  `archive/README.md`.
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
  `compare-with-golden.sh` + `compare-log-output.sh` + `run-cycles.sh`.
- `compare-with-golden.sh` / `generate-golden.sh` — compare the built crate's
  dump/build output against `tests/golden/checksums.sha256`, and refresh
  that snapshot after a legitimate output-changing change. See "CI decoupled
  from C" below.
- `compare-log-output.sh` / `generate-log-golden.sh` — the same freeze-then-
  compare move as the pair above, but for stderr log output against
  `tests/golden/log/`. See "Next steps" below for how this one came later.
- `compare-roundtrips.js` — runs `tests/ttf-roundtrip-test.js` over every
  payload produced and reports a single pass/fail summary.
- `archive/compare-with-c.sh` — **moved to `archive/`, needs `c/` restored
  from git history to run** (see `archive/README.md`). Historically: builds
  the C toolchain **with clang** and compares its output against an
  already-built Rust crate byte-for-byte, on the same machine. Both
  directions: `otfccdump`'s JSON, and the font that `otfccbuild`
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
- `dll-arch-check.sh` — sourced by `run-cycles.sh`/`compare-with-golden.sh`/
  `archive/compare-with-c.sh` to detect when python3 cannot `dlopen` the
  crate's cdylib at all, so the ctypes check is skipped with a stated reason
  instead of failing. Normally the
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

**CI checks five things** (true as of Phase 1; see "CI decoupled from C" and
"Next steps" below for how the mechanism — not the coverage — changed since):
the crate builds and `cargo test` passes (c2rust
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
  enough to read: 574 at the start, **4 now** — the surface is exactly the API.
- **The crate stopped linking to itself.** 1,086 of its own items (895 functions,
  191 vtable statics) were reached through C symbol names via
  `extern "C" { … }` declarations; they are `use` imports now, and
  `#[unsafe(no_mangle)]` is gone from all but the four public functions. Beyond
  the export table, this is what makes renaming possible at all: a symbol-name
  edge is invisible to the type checker, so renaming a definition would have
  been a *link* error at best, and a silent bind to a same-named function at
  worst. It also put 1,086 previously unchecked signatures under rustc (all
  matched) and removed 305 declarations of items the declaring file never used.
  `extern "C"` itself stays for now: many of these functions are stored as
  `extern "C" fn` pointers in the vtable statics, so the calling convention
  comes off together with those.
- **The type namespace is Rust-shaped.** 419 of the 453 type definitions still
  had their C spelling; they are UpperCamelCase now (22,459 occurrences over
  124 files), and the 90 enum variants are written `Enum::Variant`.
  `#![allow(non_camel_case_types)]` is gone. The naming rule is that a prefix
  the crate or the module already supplies comes off — `otfcc_Options` →
  `Options`, `otl_Coverage` → `Coverage` — unless the bare remainder is too
  generic to read at an unqualified use site, where it stays and is CamelCased
  (`bk_Cell` → `BkCell`, `json_value` → `JsonValue`). `table_head` →
  `HeadTable` and `subtable_gsub_single` → `GsubSingleSubtable` put the noun
  first. **Note that the entries above this one use the pre-rename names**,
  because they describe what those PRs did at the time.
  Three things worth knowing:
  - **`non_camel_case_types` could not drive the pass, and still cannot check
    most of it.** rustc exempts `#[repr(C)]` types from that lint — such a type
    is presumed to mirror a C declaration — and 352 of the crate's 400
    struct/enum/union definitions are `#[repr(C)]`. The lint saw 160 of the
    419. The rest were found by inventorying the source, and deleting the
    `allow` proves only that the 48 non-`repr(C)` types and the 90 variants
    stay conformant; the other 352 are unchecked until Stage 6 drops
    `#[repr(C)]`, at which point they must already be correct or the build
    breaks. That is the honest reading of that deleted line.
  - **Qualifying the variants removed a hazard rather than avoiding it.** Each
    defining module carried a `pub use ThatEnum::*;` glob so use sites could
    name variants bare; all 20 globs are gone. A bare variant name in a pattern
    position is not an error if it is not in scope — it is a catch-all binding
    that eats the arms after it, which is how PR #34 lost a `match`.
    `Enum::Variant` cannot be read that way. The globs were also shadowing the
    prelude: `JsonType`'s variants are `None` and `String`, so with the glob in
    place `mem_alloc: None` in `vendor/json.rs` meant `JsonType::None`. Twelve
    type errors appeared when the glob came out, all of them latent before.
  - **Fourteen `typedef struct _foo foo;` pairs resolved to one Rust name**, so
    the redundant alias is gone: `_caryll_font`/`otfcc_Font` → `Font`,
    `bk_Item` (an alias of `bk_Cell`) → `BkCell`, and the six names for one
    handle type collapse to `Handle`, `GlyphHandle`, `LookupHandle`,
    `FdHandle`. A module name and a type name can also be the same identifier:
    `sds` was both `pub type sds` and `pub mod sds`, and a whole-identifier
    rename rewrote 123 path segments along with the type. The build caught it,
    and the rename tool now refuses any name that is also a module name.
- **Constants and statics are `SCREAMING_SNAKE_CASE`.** 322 unique names were
  still C-cased (366 definition sites — the plan's 339 was a stale count from
  before PR #43 removed `no_mangle` from 350 more items, the same effect that
  hit the type pass) — 3,452 occurrences over 104 files.
  `#![allow(non_upper_case_globals)]` is gone, and this time the lint really
  does drive and check the whole thing: unlike `non_camel_case_types`, nothing
  here is `repr(C)`-exempt. `f16dot16_negativeIntinity` gets its typo fixed
  (`F16DOT16_NEGATIVE_INFINITY`) on the way, the same call as the two type-name
  typos in PR #44. Digits stay attached to the word they're already touching
  rather than getting their own segment — `f16dot16_precision` (the type is
  `F16Dot16`) becomes `F16DOT16_PRECISION`, not `F_16_DOT_16_PRECISION`, and
  `type2_max_subrs` becomes `TYPE2_MAX_SUBRS` — because a naive case-transition
  splitter fragments digit-bearing names the source doesn't actually treat as
  multi-word.
- **Local variables and parameters are `snake_case`** — 421 names, 2,792
  occurrences over 62 files. `non_snake_case` covers four different things
  (variables, functions, struct fields, modules) with one lint, so
  `#![allow(non_snake_case)]` stays on `lib.rs` until all four are done; only
  the "variable" diagnostics (707 sites, 480 unique names) belong to this PR.
  Two things made this pass riskier than renaming types or constants, because
  locals are scoped and a whole-crate textual rename by exact name isn't:
  - **58 names were left for the field and function passes instead**, because
    their identifier text is shared across namespaces. `className` is both a
    local in `gpos_common.rs` *and* a struct field with the same spelling
    elsewhere; renaming it here would rename the field too, ahead of the
    field pass's per-site JSON-key check. Anything whose name (stripped of a
    leading `_` or trailing `_N`) collided with a flagged field or function
    name was deferred, so the eventual field/function rename lands on both
    at once.
  - **A leading underscore is not decoration.** `_maxGlyphs` and `maxGlyphs`
    are two *different* parameters in two different function signatures that
    implement the same callback shape — one ignores the argument, hence the
    Rust unused-parameter convention. An early version of the case converter
    stripped the underscore before recasing, which collapsed both onto
    `max_glyphs` and would have silently turned that warning suppression
    off; the fix keeps the prefix. One name needed a human instead of the
    tool: `src/vendor/emyg_dtoa.rs`'s `K` (the binary exponent, an out
    parameter) and `k` (the decimal digit count, a local) are both standard
    notation from the Grisu2 paper and coexist in the same function —
    `to_snake("K") == "k"` would have shadowed the real `k`. `K` stays
    C-cased for now.
- **Functions are `snake_case`** — 1,727 names, 5,561 occurrences over 107
  files. 3 more names (`byGID`, `parseDictKey`, `parseToCallback`) deferred to
  the field pass for the same collide-with-a-field reason as the locals pass.
  One file needed surgery first: `src/vf/vq.rs` had **8 pairs of functions
  that would have collided once recased** — a `VQ_x` wrapper and an `x`
  (lowercase) implementation, both still real, separate functions
  (`VQ_compare`/`vqCompare`, `VQ_copy`/`vqCopy`, …). Each pair was checked
  individually rather than assumed identical: `vqInit`/`vqCopy`/`vqDispose`
  turned out to have no caller besides their own `VQ_` wrapper, so the
  wrapper absorbed the body and the duplicate was deleted; `vqCompare`,
  `vqInplacePlus`, `vqInplaceScale`, `vqInplaceNegate` and `vqNeutral` are
  each called from *other* functions too (`vqCompare` also backs
  `VQ_compareRef` and `VQ_equal`), so there the trivial one-line `VQ_`
  wrapper was deleted instead and its call sites redirected to the real
  implementation. Net effect: 8 redundant forwarding functions gone — dead
  weight from a C source that had both an inline helper and a wrapper for it,
  which c2rust carried over verbatim. All-payload byte comparison covered
  this: `vf/vq.rs` backs variable-font interpolation, one of the tested
  payloads.
- **Struct fields are `snake_case`** — 344 names, 6,038 occurrences over 97
  files: 337 fields plus 7 locals (`_className`, `_fdArray`, `_fontMatrix`,
  `_className_0`, `className_0`, `nameString_0`, `_CFF_`) that were held back
  from the locals/functions passes for colliding with one of these fields.
  This was meant to be the careful one, and it earned it:
  - **Two fields landing on the same name inside one struct** would be a
    compile error at best — or, if the crate used field-init shorthand or
    positional literals anywhere, a silent field swap. It doesn't (every
    struct literal here is `Struct { field: value, ... }`), and a per-struct
    check found no same-struct collisions regardless.
  - **JSON keys had to come out untouched**, which meant first confirming
    nothing in the crate ties a Rust identifier to its JSON spelling by
    reflection — no `stringify!`, no macro doing it implicitly. Every key is
    its own string or byte-string literal sitting next to the field it
    serializes (`b"numGlyphs\0"` beside a `numGlyphs` field), so the rename
    tool's existing literal-skip guard was already sufficient; confirmed by
    an all-payload byte comparison of `otfccdump`'s JSON output, not just
    argued from reading the code.
  - **8 fields — `BASE`, `COLR`, `CPAL`, `GDEF`, `LTSH`, `OS_2`, `TSI5`,
    `VORG` — spell exactly the module that owns the matching table**
    (`mod BASE;`, soon `mod base;`). `record.BASE` and `crate::table::BASE`
    are unambiguous to the compiler, but the rename tool is textual and
    can't tell a field access from a path segment — the same hazard as `sds`
    the type vs. `sds` the module in PR #44. Deferred to the module pass,
    which has to touch those files anyway.
- **Modules are `snake_case`, and `#![allow(non_snake_case)]` is gone —
  Stage 4 naming is complete.** 11 modules kept their C-tag spelling for
  both the file and the `mod` declaration: `BASE`, `CFF`, `COLR`, `CPAL`,
  `GDEF` (twice — `table::GDEF` and `consolidate::otl::GDEF` are two
  different modules that happen to share a name, both renamed the same way
  since they don't collide with each other), `LTSH`, `OS_2`, `SVG`, `TSI5`,
  `VORG`, `_TSI`. 188 occurrences over 17 files — mod declarations, path
  segments, `use` statements — plus the 8 fields and the 1 local variable
  deferred from the fields pass, all sharing this exact spelling with the
  module next to them, renamed together in the same commit so a field-only
  rename never ran ahead of the file move that makes it safe. One holdout
  needed a human: `emyg_dtoa.rs`'s `K` (a `*mut c_int` out-parameter for the
  binary exponent, five function signatures) becomes `k_out` — plain `k` was
  already the name of the decimal-digit-count local it coexists with in the
  same function, the same shadowing risk the locals pass hit and worked
  around the same way. With modules done, the third and last naming
  `#![allow(...)]` comes off `lib.rs` — after `non_camel_case_types` (PR
  #44) and `non_upper_case_globals` (PR #45) — and unlike the type lint,
  `non_snake_case` has no `repr(C)` exemption, so this one really does mean
  every variable, function, field and module in the crate is conformant,
  checked by the compiler on every build from here on.
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
  Both counts are historical: the edition made the two claims audible, and the
  audit that followed found almost nothing foreign behind either of them — 4
  `#[unsafe(no_mangle)]` and 13 genuinely external names survive (see "The
  public ABI is four functions").

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

  **The whole header followed.** All 17 of its helpers were `static inline`, and
  c2rust had re-emitted each into every translation unit that used one — 32
  copies of `json_obj_get`, 30 of `json_obj_get_type`, 16 of `preserialize`,
  **139 definitions in total**. There is now one of each, in
  `support/json_funcs.rs`, at −2,469/+357 lines across 39 files. Every name's
  copies were checked textually identical before any was deleted, and none was
  ever `#[no_mangle]`, so the ABI was untouched (553 exports at the time).

  One of the seventeen looks redundant and is not: `json_obj_getnum` walks the
  object itself rather than calling `json_obj_get`, because on a name match whose
  value has the wrong type it **keeps looking**, where
  `json_numof(json_obj_get(obj, key))` would stop at the first match and return
  0. The two differ whenever a key appears twice with different value types,
  which the parser permits — it keeps duplicate members. `json_obj_getnum` and
  `json_obj_getint`, which were the `_fallback` bodies with the fallback spelled
  0, do now delegate; the loops themselves stay as C wrote them.

  The one helper left where c2rust put it is `json_from_sds`, in `table/CFF.rs`:
  it needs `sdslen`, itself a `static inline` from `sds.h` with 20 copies of its
  own, and that family is a pass of its own. It only ever had one copy here, so
  there is nothing to collapse.

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

- **Bit sets use `bitflags`, and one of them needs `#[repr(transparent)]`.**
  `glyf_PointFlags` (the flag byte before each outline point),
  `glyf_ComponentFlags` (the flag word before each composite component) and
  `otl_BuildHeuristics` are genuine bit algebra — `|=` to build, `&` to test —
  so they are `bitflags` structs over the width the wire uses (`u8`, `u16`,
  `u32`), read with `from_bits_retain` so a bit otfcc does not know about is
  carried rather than dropped, and written with `.bits()`.

  `otl_BuildHeuristics` is a parameter of the `extern "C"` subtable builders,
  and `bitflags`' generated struct is **not FFI-safe by default**: without
  `#[repr(transparent)]` inside the macro, `improper_ctypes` fires at nine
  declarations, which `warnings = "deny"` turns into a failed build. The
  attribute is load-bearing, not decoration — verified by removing it.

  Two of the five things the plan called bit sets were not:
  - `MASK_ON_CURVE` is one bit, on a field (`glyf_Point::onCurve`) that is not a
    flag set. It is a plain `i8` const now, typed as the field it masks.
  - `json_GlyphOrderPass` is an ordered *priority*, not a set: the lowest pass
    wins, because `setOrderByName` escalates only downwards and `_byOrder`
    sorts ascending. So it is an `enum` with derived `Ord`, like `bk_CellType` —
    and, like `cff_Value_Type`, it needed a zero variant this port had to name.
    C keeps the `enum` inside `json-reader.c` while `glyph-order.h` declares the
    field as a bare `uint8_t`, which lets the OTF path leave it at whatever
    `calloc` gave it: `otfcc_setGlyphOrderByGID` allocates an entry and sets only
    `gid` and `name`. `ORD_UNSET = 0` is that state, and it is meaningful —
    zero outranks every named pass, so an entry placed by GID can never be
    escalated by one. The type therefore lives in `support/glyph_order.rs` next
    to the field it types, not in `json_reader.rs` where C has it.

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

- **`table/cff.rs`'s `build_outline` no longer allocates a fresh
  0x10000-entry `Vec<CffValue>` charstring-interpreter operand stack for
  every single glyph.** Found while verifying an unrelated PR
  ([#260](https://github.com/MaruTama/otfcc-rs/pull/260), `table/fvar.rs`'s
  `FontReader` migration): that PR's `otf_parse` fuzz job hung for 30+
  seconds and exceeded the fuzzer's 2GB memory limit on a mutated CID-keyed
  CFF table whose `CharStrings` count had been pushed to 65535 (the
  corpus's max `u16`).
  - **Confirmed unrelated to PR #260 before touching anything**: replayed
    the exact same input against `master` from *before* that PR and got
    the identical hang -- a pre-existing bug the fuzz job happened to
    surface while checking a different change, not a regression from it.
  - **Isolated to the CFF table specifically** by zeroing every other
    table in the crash input and rerunning (only the CFF table's presence
    changed the outcome), then to this one allocation site by
    `sample`-profiling the hung process (allocator/`RawVecInner::
    with_capacity_in` frames dominated the samples) and confirming that
    zeroing just the `CharStrings` count field collapsed the runtime from
    30+ seconds to under one.
  - **The bug**: `build_outline` runs once per glyph
    (`otfcc_read_cff_and_glyf_tables`'s per-glyph loop), and used to
    construct its own fresh `CffStack` locally on every call --
    `libcff.rs`'s own doc comment on `CffStack.stack` already noted the
    `0x10000`-entry sizing is "generous", "never actually approached" by
    a real charstring (the Type 2 spec caps operand stack depth at 48), so
    for an ordinary font this per-glyph reallocation was wasted but cheap
    work relative to everything else `build_outline` does. At 65535
    glyphs it dominates: every glyph paid for allocating and zero-filling
    a ~1MB `Vec` it barely touches, and that per-glyph churn is exactly
    the pattern ASan's allocator instrumentation (always on for `cargo
    fuzz` builds) amplifies most.
  - **The fix**: the stack is now allocated once in
    `otfcc_read_cff_and_glyf_tables`, before the per-glyph loop, and
    passed into `build_outline` by pointer; only the cheap fields
    (`index`/`stem`/`transient`, none of them heap-backed) are reset per
    glyph. Safe to reuse as-is: every push/pop in the interpreter already
    stays within `[0, index)`, so stale bytes from a previous glyph past
    the new glyph's own `index` are never read.
  - **Reproducer** kept as a fuzz-corpus regression pin
    (`tests/fuzz-corpus/known-issues/
    otf-parse-cff-per-glyph-stack-realloc-hang.bin`, 492KB -- large for
    that directory, because reaching 65535 glyphs with distinct,
    well-formed-enough `CharStrings`/`FDSelect`/`FDArray` structure
    doesn't compress smaller) plus a new unit test,
    `otf_reader.rs`'s `cff_font_with_huge_glyph_count_parses_promptly`,
    which pins it as a **wall-clock budget** (must parse in well under
    10s) rather than a correctness assertion, since the bug was about
    time/memory, not a wrong answer or a crash.
  - **Verification**: full pipeline green -- build and clippy clean at
    `-D warnings`, 301/301 tests (300 prior + 1 new), ABI unchanged,
    golden bytes unchanged (`KRName-Regular.otf`/`-O2.otf`, the payloads
    that actually exercise `build_outline`, byte-identical) and log output
    unchanged, cycles/lookup-alias/10-payload round-trips all clean, the
    original crash input now parses in ~4s under `cargo fuzz`'s ASan
    build (down from a 30+s timeout) and a fresh 60-second `cargo fuzz
    run otf_parse` afterward found nothing, both fuzz targets `cargo
    check`-clean, `cargo miri test` clean (278 passed, 0 failed, 23
    ignored -- the wall-clock regression test is itself `#[cfg_attr(miri,
    ignore)]`, both because Miri needs `-Zmiri-disable-isolation` for
    filesystem access and because its interpretation overhead would make
    a timing-based assertion meaningless).

- **Stage 7-1 mop-up: the last 8 tables/helpers still on the pre-`FontReader`
  c2rust idiom (`.offset()` raw-pointer arithmetic, `__fortable_*`/
  `current_block` foreach/goto emulation, `binio::read_*`) are migrated --
  `table/vorg.rs`, `table/base.rs`, `table/colr.rs`, `table/cpal.rs`,
  `table/gdef.rs`, `table/svg.rs`, `table/otl/subtables/extend.rs`, and
  `table/otl/subtables/gpos_common.rs`'s `otl_read_mark_array`/
  `otl_read_anchor`/`read_gpos_value`.** These were found by re-running
  `survey-unsafe.sh`-style greps for `__fortable_`/`.offset(` after the
  `getopt_long` PR and discovering Stage 7-1's flagship named bugs (post/
  name/cmap/glyf/gvar/ltsh/hdmx/CFF, all already fixed in earlier PRs) were
  not actually the *entire* list of unmigrated files -- these 8 had never
  been named individually in the plan and so were never swept up.
  - **Four newly-found, genuinely exploitable bugs, all the same shape**:
    an offset or length read straight from the file as a raw `u32` (full
    attacker control, up to `u32::MAX`), guarded with `x.wrapping_add(n)`
    or `x.wrapping_add(y).wrapping_add(z)` in **32-bit** arithmetic -- a
    value close enough to `u32::MAX` wraps the sum back down to something
    small, so a length check that should reject an out-of-range offset
    passes instead, and the subsequent `.offset()` reads from (or copies
    out of) memory nowhere near the table. Distinct from the *shape* of
    bug `table/gdef.rs`'s/`table/svg.rs`'s/`otl/subtables/gpos_common.rs`'s
    *other* offsets have (sums of a couple `u16` fields, which can never
    reach anywhere near `u32::MAX` regardless of arithmetic width) --
    these four are directly attacker-supplied 32-bit values:
    - `table/cpal.rs`: **all four** of `offsetFirstColorRecord`,
      `offsetPaletteTypeArray`, `offsetPaletteLabelArray` and
      `offsetPaletteEntryLabelArray`.
    - `table/svg.rs`: the per-record document span, guarded by
      `offset_to_svg_doc_index.wrapping_add(docstart).wrapping_add(doclen)`
      -- three raw `u32`s chained, any pair of which can wrap it.
    - `table/otl/subtables/extend.rs`: the Extension mechanism's whole
      reason for existing is carrying a real 32-bit subtable offset (every
      other GSUB/GPOS lookup type is limited to Offset16) -- `subtable_
      offset.wrapping_add(extensionOffset)` combined a bounded `u16`-scale
      offset with that fully attacker-controlled `extensionOffset`.
    All four fixed by `FontReader`'s `checked_add`/`checked_mul` (or, for
    `extend.rs`, `u32::checked_add` directly, since it doesn't need a full
    `FontReader` conversion) instead of `wrapping_add`.
  - **Two more bugs found along the way, different shape**:
    - `table/otl/subtables/gpos_common.rs`'s `otl_read_mark_array` had *no*
      room check at all for its `mark_count`-driven record loop (only the
      2-byte `MarkCount` field itself was bounds-checked) -- a large
      `mark_count` against a short table read straight past the buffer's
      end. Also indexed the sibling Coverage table's `Vec` by `mark_count`
      with no check against the Coverage's own length -- two independent,
      both attacker-controlled counts a well-formed font keeps equal but
      nothing enforced, so a mismatched `MarkCount` panicked on `Vec`
      index out of bounds (a crash, not a memory-safety bug, but still
      DoS-reachable from a crafted GPOS MarkToBase/MarkToLigature table).
      Fixed by `require_room` plus capping the loop at `cov.len()`.
    - `table/gdef.rs`'s `otfcc_read_gdef`: the LigCaretList's `Coverage`
      (`cov`, allocated via `read_coverage`) was never freed on either of
      its two "this table is corrupted" abort paths -- only the success
      path called `otl_coverage_free`. A real pre-existing leak on any
      malformed LigCaretList, fixed as a natural side effect of replacing
      the `current_block` goto-emulation with early returns (every new
      return path frees `cov` first).
  - **`table/base.rs`'s own narrower version of the same overflow-defeats-
    guard bug, via truncation instead of wraparound**: the original
    computed nested `BaseScript`/`BaseValues` offsets as `(x as c_int +
    offset as c_int) as u16` -- safe 32-bit addition, but then truncating
    the *result* back down to `u16`, silently wrapping whenever the real
    combined offset exceeded 65535. Every derived offset now stays `usize`
    instead of being narrowed back to `u16`.
  - **Everything else migrated (`table/vorg.rs`, `table/colr.rs`, and the
    non-buggy parts of the other five) had no live bug** -- their offsets
    are sums of one or two `u16`-scale fields (a few hundred thousand at
    most), nowhere near the `u32::MAX` a wraparound needs, so this part of
    the work is pure modernization: `.offset()`/`FontFilePointer`/
    `__fortable_*`/`current_block` gone, `FontReader` in their place, same
    reasoning `otl/coverage.rs`'s own `read_coverage` doc comment already
    laid out for exactly this class of bug when *it* was migrated.
  - **Tests came first for every fix**: each of the six confirmed bugs
    above got its own regression test reproducing the exact wraparound/
    missing-check/leak shape (`table/cpal.rs`'s 2 near-`u32::MAX` tests,
    `table/svg.rs`'s 1, `table/otl/subtables/extend.rs`'s 1,
    `table/otl/subtables/gpos_common.rs`'s 2 for `otl_read_mark_array`),
    plus well-formed/truncated/malformed-input coverage for every other
    migrated function (39 new tests total across the 8 files).
  - **Verification**: full pipeline green -- build and clippy clean at
    `-D warnings`, 300/300 tests (261 prior + 39 new), ABI unchanged,
    golden bytes unchanged (including `gdef-ligcaret-dedup` and
    `mark-consolidate-dedup`, which exercise exactly the GDEF lig-caret and
    GPOS mark-array paths touched here) and log output unchanged, cycles/
    lookup-alias/10-payload round-trips all clean, `cargo fuzz run
    otf_parse -- -max_total_time=60` found nothing beyond expected
    "Undefined Byte in CFF" warning noise, both fuzz targets `cargo
    check`-clean, `cargo miri test` clean (278 passed, 0 failed, 22
    ignored, matching baseline + the 39 new tests). `survey-unsafe.sh`
    deltas (measured against this branch's parent): `unsafe fn` 889->884,
    raw pointer types 5352->5287, `.offset(` calls 745->613,
    `__fortable_*` 117->33, `current_block` 18->12, `as ::core::ffi::c_int`
    casts 4957->4650, while loops 670->634. `unsafe blocks` rose 276->299
    and `Result<` usage rose 33->38 -- expected, not a regression: each
    file went from one giant `unsafe fn` body to several small explicit
    `unsafe {}` blocks around the handful of calls (`otfcc_handle_dup`,
    `read_coverage`, `otl_coverage_free`, ...) that still need it, plus a
    `Result`-returning `parse_*` helper per file, matching every prior
    Stage 7-1 migration's shape. `files with allow(unsafe_op_in_unsafe_fn)`
    unchanged at 101/142, same as every prior Stage 7-1 file: the public
    `otfcc_read_*`/`otfcc_build_*` wrappers still call other `unsafe fn`s
    without individually wrapping every call, so the file-level allow
    stays -- removing it crate-wide is Stage 7-4 scope, not this migration's.
  - **Deliberately out of scope**: the `__fortable_*` table-lookup loops
    still present in `cff.rs`/`glyf/read.rs`/`fvar.rs`/`cvt.rs`/
    `otf_reader.rs` are a different, lower-value category -- plain
    "find this table by tag in `packet.pieces`" iteration, not attacker-
    facing bounds-unsafe parsing, and already partially interleaved with
    those files' already-migrated per-record readers. Left for whenever
    those files get their own dedicated pass.

- **`getopt_long`'s libc FFI dependency is gone -- Stage 7-4's last named
  item.** `support/getopt.rs` held only an FFI mirror of libc's `struct
  option` (`LongOption`); the actual `getopt_long` symbol was reached
  through a raw `unsafe extern "C"` block in each binary, which was already
  a lie about portability -- `libc` declares `getopt_long` for the BSDs,
  Apple, Solaris, Android and Hurd, but **not** for
  `*-unknown-linux-gnu`, this crate's own CI target. Replaced with a
  hand-rolled, `std::env::args`-based parser reimplementing the two
  behaviors that made `getopt_long` worth having over a plain positional
  scan: permuting recognized options in front of positional arguments
  regardless of where either appears in argv, and matching an unambiguous
  prefix of a long option's name. Deliberately narrower than the real
  thing (no `-W longopt`, no `optstring` `+`/`-` mode switches, no
  `POSIXLY_CORRECT`) -- neither binary used any of those, and glibc's own
  docs call them rare corners even in C code.
  - **Tests came first, on purpose.** The plan flagged that the existing
    suite pinned neither GNU getopt's argv permutation nor long-option
    abbreviation -- only the `-O2` concatenated form. `support/getopt.rs`'s
    14 unit tests (permutation, unambiguous-prefix match, exact-match-wins-
    even-as-a-prefix, ambiguous-prefix reporting, long/short required-arg
    handling both attached and separate, bundled short flags, a
    value-taking short mid-bundle consuming the rest of its token, `--`
    ending option parsing, unknown long/short options, missing required
    arguments, and a lone `-` as positional) were written and passing
    against the new parser before either binary was touched.
  - **A platform-specific quirk, deliberately not reproduced**: empirically,
    macOS's system `getopt_long` silently drops an ambiguous long-option
    prefix (e.g. `--merge-`, matching both `--merge-lookups` and
    `--merge-features`) instead of erroring -- a real divergence from
    documented glibc/GNU behavior. Implemented the standard glibc semantics
    (hard, reported ambiguity) instead, since this crate's CI runs both
    platforms and an undocumented macOS-only quirk isn't a contract worth
    porting.
  - **A genuine pre-existing bug, fixed as a side effect of the redesign**:
    `otfccbuild.rs`'s `--dont-ignore-glyph-order` was registered in
    `longopts` and documented in `printHelp()` as a synonym for
    `--keep-glyph-order`, but the old `strcmp` chain checked for the wrong
    string (`"dont-keep-glyph-order"`, never actually registered under that
    spelling) -- so the documented flag silently no-op'd. The new dispatch
    design gives every option a unique `i32` match value directly (no more
    `option_index` indirection into `longopts`, since there's no C-side
    `flag`/`val` split left to mirror once this isn't crossing an FFI
    boundary), so `--dont-ignore-glyph-order` and `--keep-glyph-order` now
    share one value by construction -- there is no string left to typo.
  - **A deliberate, documented behavior change**: both binaries' `main()`
    now build argv via `std::env::args()` (`String`, panics on non-UTF-8)
    instead of the old `CString`-per-arg conversion, which was already
    panic-on-non-UTF-8 in practice (`CString::new(arg).expect(...)`) --
    so this is a no-op in practice, just one less manual conversion layer.
    Argument values consumed by an option (`-o`, `--optimize`,
    `--glyph-name-prefix`, `--ttc-index`) go through the same
    `CString::new(...).expect(...)` panic-on-embedded-NUL path the rest of
    this migration already uses for CLI strings.
  - **Verification**: full pipeline green -- build and clippy clean at `-D
    warnings`, 261/261 tests (247 prior + 14 new getopt unit tests), ABI
    unchanged, golden bytes and log output unchanged, cycles/lookup-alias/
    10-payload round-trips all clean, both fuzz targets `cargo check`-clean,
    `cargo miri test` clean (239 passed, 0 failed, 22 ignored, matching
    baseline + the 14 new tests). Golden coverage only exercises the
    concatenated/single-spelling flag forms already in use, so argv
    permutation, prefix abbreviation, the `--dont-ignore-glyph-order` fix,
    and the ambiguous/unknown/missing-argument diagnostics were additionally
    checked empirically against both built binaries with real payloads
    (`tests/payload/iosevka-r.json`/`.ttf`) -- permuted and non-permuted
    invocations byte-identical, `--dont-ignore-glyph-order` output
    byte-identical to `--keep-glyph-order`, prefix abbreviations
    byte-identical to full spellings. `survey-unsafe.sh` deltas (measured
    against this branch's parent, `e34a73f5`): raw pointer types
    5528->5352, `is_null()` calls 363->361, `as ::core::ffi::c_int` casts
    5037->4957 -- `support/getopt.rs`'s old `#[repr(C)]` `LongOption`
    array-literal boilerplate (46 struct literals across both binaries'
    `longopts` tables, each with a `flag: null_mut()` and 1-2 `as
    *const c_char`/`as c_int` casts) accounted for most of it. `unsafe fn`
    count is unchanged (889) -- `main_0` and the option-parsing code were
    already `unsafe fn`/inside `unsafe` blocks before this PR, for reasons
    unrelated to `getopt_long` itself (raw `Options`/`Logger` pointers).

- **`lib.rs`'s three crate-root `#![allow(dead_code, unused_assignments,
  unused_mut)]` are gone -- the last item Stage 7-4 named, scoped to be
  done last since "most of it should fall out naturally from the earlier
  stages."** Removing them cold surfaced 1,626 individual violations
  across ~48 files -- large-sounding, but the actual composition made it
  far more tractable than that number suggests:
  - **1,477 `unused_mut` (91%)**: c2rust marks every C-style local `mut`
    regardless of whether it's ever reassigned; rustc's own lint finds
    exactly the ones that aren't. Purely mechanical, fixed in one
    `cargo fix --lib --release --allow-dirty --broken-code` pass, zero
    manual review needed -- removing `mut` from a binding never
    reassigned cannot change behavior.
  - **128 more, after that**: `unused_assignments` on `let mut x: T =
    dummy;` declarations where every real code path overwrites `x`
    before ever reading it (c2rust's other standard dummy-init habit).
    Fixed by a script that stripped each flagged initializer down to
    `let mut x: T;` (deferred initialization) and rebuilt to let rustc's
    own definite-assignment analysis be the actual safety check -- it
    would have refused to compile any site where a path could still read
    `x` before a real assignment reached it. Zero did; all 128 were
    genuinely dead every time. A second `cargo fix` pass then dropped the
    `mut` this revealed as no-longer-needed on some of them (a dummy
    initializer is itself a write, so removing it can turn a two-write
    binding into a one-write, no-`mut`-needed one).
  - **The last 15** needed actual eyes: 6 were genuinely dead functions
    (`binio.rs`'s `read_8s`/`read_24u`/`read_32s`/`read_64u` -- Stage
    7-1's `FontReader` conversion outgrew them, though `read_8u`/`16u`/
    `16s`/`32u` still have real callers; `handle.rs`'s
    `otfcc_handle_copy_replace`/`handle_name_eq_cstr`, both redundant
    with a sibling already doing the same thing), deleted outright. The
    remaining 15 dead-store sites (11 "free then null out" or "reassign
    then immediately return" writes right before the variable goes out
    of scope; one `break`-before-the-check-it-fed; one loop's final
    increment with no read after it; one write on a path that already
    falls through to a literal `return None` further down that never
    reads it) were each individually confirmed to have no reader on any
    path before being deleted.
  - **Verification**: full pipeline green -- build with *zero* warnings
    (not just errors) at `-D warnings`, 247/247 tests, clippy clean, ABI
    unchanged, golden bytes and log output unchanged (every one of these
    was a dead-code/dead-store removal or a `mut` drop, never a behavior
    change), round-trips 10/10, lookup-alias regression clean, both fuzz
    targets `cargo check`-clean, `cargo miri test` clean (225 passed, 0
    failed, 22 ignored, matching baseline). `survey-unsafe.sh` deltas:
    `unsafe fn` 895->889, raw pointer types 5534->5528, `.offset(` calls
    756->747 (the 6 deleted functions' own bodies).

- **Two more real crashes found by fuzzing while investigating the
  `gvar-test.ttf` leak below -- both in the CFF *read* path, unrelated to
  each other and to that investigation.** Neither was reachable from the
  seed corpus; both surfaced within minutes of local mutation-based
  fuzzing (`cargo fuzz run otf_parse -- -max_total_time=...`).
  - **`libcff/cff_parser.rs`'s `cff_parse_subr`: `fd` (a glyph's font-dict
    index, read straight from the FDSelect table -- attacker-controlled)
    indexed `fdarray.offset` via raw, unchecked `.offset()` arithmetic
    with nothing above bounding it against `fdarray`'s actual size.** An
    `fd` past `fdarray.count` read arbitrary memory past the `Vec`'s
    allocation -- a real SEGV. Fixed with the same fallback the function
    already uses when a well-formed `fd`'s FDArray entry just doesn't
    declare a Private dict: treat an out-of-range `fd` as "no private
    dict for this glyph" and fall back to `empty_index`. Pinned by a new
    unit test constructing exactly this shape (`fdarray.count: 1`,
    `CffFdSelect::Format0(vec![99])`) -- direct enough to test without a
    fuzz-corpus reproducer.
  - **`table/cff.rs`'s `otfcc_read_cff_and_glyf_tables`: a CFF table
    whose Top DICT INDEX declares `count: 0` (no entries at all) indexed
    `top_dict.offset[0]`/`[1]` unconditionally, panicking ("index out of
    bounds: the len is 0") since `extract_index` only populates `offset`
    when `count > 0`.** Same guard shape as the `font_dict.count != 0`
    check a few lines below it for the FDArray INDEX -- wrapped the
    whole top-dict-dependent block in `if (*cff_file).top_dict.count !=
    0`, leaving `ret` at its already-null defaults (matching how this
    function already signals "nothing usable found here") when it's 0.
    Pinned by `tests/fuzz-corpus/known-issues/
    otf-parse-empty-top-dict-index-panic.bin`, not a unit test --
    reaching this code needs a real `Packet`/`CffFile`, heavier to
    hand-construct than the `cff_parse_subr` case above.
  - **Verification**: full pipeline green -- build, 247/247 tests (the
    new `cff_parse_subr` test included), clippy clean, ABI unchanged,
    golden bytes and log output unchanged (both fixes only change
    malformed-input behavior; every payload with real CFF content,
    including the CID-keyed ones, stayed byte-identical), round-trips
    10/10, lookup-alias regression clean, both fuzz targets `cargo
    check`-clean, `cargo miri test` clean (225 passed, 0 failed, 22
    ignored). Both minimized reproducers confirmed fixed locally (exit
    cleanly instead of crashing); a 3-minute local `cargo fuzz run
    otf_parse` and a 1-minute `json_build` run afterward found nothing
    else.

- **`rust/fuzz/README.md`'s last "known finding" (the `gvar-test.ttf`
  leak, Stage 6-4/`VqRegion` scope) no longer reproduces -- confirmed,
  not newly fixed.** Investigated as the next item after the OOM fix
  below; the two allocation sites the finding named (a `calloc` in
  `table/glyf/read.rs::otfcc_read_glyf`, a `format!()` in
  `table/fvar.rs::fvar_register_region` building `FvarMaster.name`) had
  both already been rewritten to plain owned `Vec`s as part of Stage
  7-1's `glyf`/`gvar` work, done earlier in this same effort but before
  the finding was last rechecked -- a side effect of that ownership
  rewrite covering the exact fields this leak was rooted in, not a fix
  landed for this finding specifically. `leaks --atExit` (macOS's own
  leak detector -- LeakSanitizer itself isn't supported on macOS at all,
  a real LLVM/Darwin limitation already noted for the `callback_makefd`
  leak above) finds 0 leaks now on `gvar-test.ttf` and every other
  `otf_parse` corpus file. With this, every finding `rust/fuzz/README.md`'s
  "Known findings" section ever listed is resolved -- until fuzzing
  turned up the two new, unrelated ones above while re-verifying that.

- **`font/caryll_sfnt.rs`: fixed the last `fuzz`-`README.md`-documented
  finding, `otfcc_read_packets`'s multi-gigabyte-allocation OOM.** A table
  directory entry's raw, unvalidated `length` field (up to `u32::MAX`) sized
  `vec![0u8; length as usize]` before the function ever tried to read a
  single byte of that table -- a 961-byte crafted input could make it
  request a 3.7GB allocation. Fixed by getting the file's real length once
  up front (`file.seek(SeekFrom::End(0))` -- doesn't disturb anything after
  it, every read below starts with its own absolute `SeekFrom::Start`) and
  checking each entry's `offset + length` against it before allocating,
  the same "fail before doing unbounded work" shape `read_exact`'s own
  short-read failure already gave the byte-copying step right below it.
  This was scoped in `rust/fuzz/README.md` as "exactly what Stage 7-1's
  planned `FontReader` is designed to close crate-wide" and deliberately
  deferred there -- reconsidered and fixed directly here instead, since
  the actual change needed turned out to be self-contained to this one
  function rather than needing the full `FontReader` abstraction first.
  Added `table_length_far_past_file_end_fails_without_allocating_it`
  (declares a length one byte short of `u32::MAX` in a few dozen actual
  bytes; if the check regressed, this test would hang or OOM instead of
  finishing instantly) alongside the existing short-read test.
  - **Verification**: full pipeline green -- build, 246/246 tests (the new
    one included), clippy clean, ABI unchanged, golden bytes and log
    output unchanged, round-trips 10/10, lookup-alias regression clean,
    both fuzz targets `cargo check`-clean, `cargo miri test` clean (224
    passed, 0 failed, 22 ignored -- one more than baseline, the new test
    itself touches a real file so it's `#[cfg_attr(miri, ignore = "...")]`
    like its neighbors). The known-issues reproducer
    (`otf-parse-table-length-oom.bin`) confirmed fixed locally, running in
    1ms instead of requesting a multi-gigabyte allocation; a 60-second
    local `cargo fuzz run otf_parse` against the full corpus afterward
    found nothing else.

- **`table/cff.rs`/`libcff/charstring_il.rs`: fixed both of the two
  `json_build` bugs `rust/fuzz/README.md`'s "Known findings" had documented
  as deliberately deferred to Stage 7-1/7-3.** Found while confirming the
  CFF-recursion and leak fixes below actually turned the `fuzz` job green
  -- they didn't either, and the reason had nothing to do with either fix:
  the `fuzz` job's "Confirm the known findings still reproduce" step runs
  `cargo fuzz run` against these two bugs' minimized inputs and *expects*
  them to crash (that's what "still reproduce" verified) -- so `fuzz` had
  been showing failed on every PR since Stage 7-0-c introduced this
  infrastructure, regardless of whether `otf_parse`/`json_build`'s own
  exploratory runs (the only steps actually able to catch a *new*
  regression) were clean.
  - **`{"CFF_": {}}` (no `glyf` table at all) null-pointer-dereferenced**
    in `table/cff.rs`: `cff_make_charset`, `cff_make_fdselect`, and
    `cff_make_charstrings` all took the same `glyf: *mut GlyfTable` and
    dereferenced it unconditionally. All three share one caller,
    `writecff_cid_keyed` (`otfcc_build_cff`'s only path), so the null
    check moved there instead of being duplicated three times -- a null
    `glyf` is substituted with a local empty `GlyfTable` before any of the
    three run. That alone still panicked one level further in:
    `cff_make_charstrings`'s own "0 glyphs" early return left its three
    `*mut Buffer` out-params (`s`/`gs`/`ls`) at the null the caller
    pre-initializes them to, and the caller dereferences all three right
    after the call returns regardless -- fixed by having that early
    return populate them with empty (not null) `Buffer`s instead.
  - **An absurd JSON `advanceWidth` panicked** in
    `libcff/charstring_il.rs`: `glyph_adw_const as c_int` already
    saturates a huge-magnitude `f64` to `i32::MIN`/`MAX` rather than
    wrapping (correct, checked Rust behavior on its own), but the
    subsequent plain `-` against `nominal_width` could still underflow
    past `i32::MIN`. Switched to `saturating_sub`, so the extreme case
    clamps instead of panicking (under debug-assertions) or silently
    wrapping to a nonsensical advance-width delta (an ordinary release
    build's quieter version of the same bug).
  - **`.github/workflows/rust.yml`**: with both bugs fixed, that step's two
    `cargo fuzz run` commands now exit 0 -- renamed to "Regression-test the
    fixed findings" and repurposed as a plain positive regression test
    (these two specific inputs must not start crashing again), rather than
    a check that they still do.
  - **Verification**: full pipeline green -- build, 245/245 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged, round-
    trips 10/10, lookup-alias regression clean, both fuzz targets `cargo
    check`-clean, `cargo miri test` clean at baseline. Both minimized
    reproducers confirmed exit 0 locally; a 60-second local `cargo fuzz
    run` against each target's full corpus afterward found nothing else
    new (`otf_parse` still reproduces the already-documented, still-open
    `caryll_sfnt.rs:105` OOM finding, unrelated to any of this).

- **`table/cff.rs`: fixed a real leak in `callback_makefd`** (CID-keyed CFF
  FDArray compilation), found while confirming the CFF-recursion fix above
  actually turned the `fuzz` job green -- it didn't, a second, unrelated
  bug in `json_build`'s path was still failing it. `build_dict(fd)` was
  called twice: once correctly, whose `Buffer` became the function's
  return value, and a second time with the result silently discarded --
  every FD dict compiled that way leaked the second `Buffer`'s heap
  allocation. Separately, `fd` (the `CffDict` from `cff_make_fd_dict`) was
  never freed at all -- every other dict in this file (`top`, `top_pd`,
  `pd`) is `cff_dict_free`'d after use, this one just wasn't. Both are
  plain omissions, not behavior changes: `build_dict` takes `*const
  CffDict` (read-only) so calling it twice was memory-safe, just wasteful,
  and `cff_dict_free` only releases memory, it doesn't touch the already-
  returned `blob`. Fixed by deleting the redundant call and adding the
  missing `cff_dict_free(fd)`.
  - **Not caught locally beforehand**: LeakSanitizer isn't supported on
    macOS at all (a real LLVM/Darwin allocator limitation, not a project
    gap), so this crate's fuzz job only ever exercises it in CI (Linux).
    Diagnosed here via macOS's own `leaks --atExit` tool against a plain
    (non-ASan) `cargo build --release` binary run directly against each
    `rust/fuzz/corpus/json_build/` seed file -- `cid-fdselect-test.json`
    (a CID-keyed CFF payload, the one seed that actually exercises
    `callback_makefd`) reproduced it immediately, with full symbol names
    pointing straight at the two lines above.
  - **Verification**: full pipeline green -- build, 245/245 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged (confirms
    the fix is a pure leak fix, not a behavior change), round-trips 10/10,
    lookup-alias regression clean, both fuzz targets `cargo check`-clean,
    `cargo miri test` clean at baseline. `leaks --atExit` against all 5
    `json_build` seed files: 0 leaks now (`cid-fdselect-test.json` showed
    several distinct leak reports, all rooted at `cff_make_fd_dict`/
    `build_dict`, before the fix). A 60-second local `cargo fuzz run
    json_build` against the full corpus afterward found nothing else.

- **`libcff/cff_parser.rs`: fixed the crash behind the `fuzz` CI job's
  persistent (advisory, `continue-on-error`) failure on every recent PR --
  the CFF Type 2 CharString interpreter had no limit on `callsubr`/
  `callgsubr` recursion depth, and no bounds check on the operand stack
  (fixed-capacity `Vec<CffValue>`, 65536 slots) before pushing onto it, nor
  on four operators' (`rcurveline`/`rlinecurve`/`vhcurveto`/`hvcurveto`)
  unchecked-subtraction operand-count math -- all four are the same root
  shape (an unsigned subtraction/loop-bound computed without first checking
  there were enough operands, wrapping to a huge value on malformed input
  the same way an unsigned integer underflow does in C). None of this was
  reachable from the migration itself; it's C-inherited (confirmed: the
  pre-Stage-7-0-a C source had the identical unguarded recursion and
  identical unguarded subtractions).
  - **`MAX_SUBR_CALL_DEPTH = 10`** (the Type 2 Charstring spec's own limit)
    caps `cff_parse_outline`'s recursion; exceeding it logs a warning and
    stops parsing that outline instead of overflowing the native stack.
    This alone also resolved the *other*, previously-documented CFF
    stack-overflow that made `rust/fuzz/README.md` exclude
    `Cormorant-Medium.otf`/`WorkSans-Regular.otf` from the fuzz seed corpus
    -- same root cause, confirmed by hand (both now parse cleanly); that
    exclusion is removed from both the workflow and the fuzz README.
  - **The operand-stack push sites** (plain operand tokens, `random`,
    `dup` -- the only three that write before checking capacity) now log a
    warning and stop parsing that outline if the stack is already full,
    instead of writing one `CffValue` past the end of its `Vec`.
  - **`rcurveline`/`rlinecurve`/`vhcurveto`/`hvcurveto`** now check for the
    minimum operand count (2, 6, and "not exactly 1 leftover coordinate"
    respectively) before doing arithmetic that assumed it, instead of
    computing a huge loop bound via unsigned wraparound and walking far
    past the stack's buffer.
  - **Found while verifying this fix**: `font/caryll_sfnt.rs:105`
    (`otfcc_read_packets`) allocates `vec![0u8; length as usize]` for each
    table using that table's raw, unvalidated declared length -- a small
    crafted input can request a multi-gigabyte allocation. Predates this
    PR (and Stage 7-4's file-I/O work) by over a week; not fixed here to
    keep this PR to one theme. Documented with a reproducer in
    `rust/fuzz/README.md`'s "Known findings" and
    `tests/fuzz-corpus/known-issues/otf-parse-table-length-oom.bin` --
    exactly the class of bug Stage 7-1's planned `FontReader` (checked
    offsets/lengths before any read or allocation) is meant to close
    crate-wide.
  - **Verification**: full pipeline green -- build, 245/245 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged (these
    fixes only change malformed-input behavior), round-trips 10/10,
    lookup-alias regression clean, both fuzz targets `cargo check`-clean,
    `cargo miri test` clean at baseline. Confirmed by hand: the original
    CI-failing seed (`FDArrayTest257.otf`) and both newly-unexcluded seeds
    (`Cormorant-Medium.otf`, `WorkSans-Regular.otf`) all parse cleanly
    now; a 60-second local `cargo fuzz run otf_parse` against the full
    corpus found no other crash besides the pre-existing, now-documented
    OOM above.

- **Stage 7-4: the file-I/O item's last two spots -- `bin/otfccbuild.rs`'s
  `readEntireStdin` and `bin/otfccdump.rs`'s `getchar` -- moved off
  `libc::fgets`/`fgetc`/`realloc` onto `std::io`, finishing the item
  `font/caryll_sfnt.rs` and the previous file-I/O PR started.** `fopen`/
  `fclose`/`fread`/`fwrite`/`fputc`/`fgets`/`fgetc` are now gone from
  `rust/src` entirely.
  - **`readEntireStdin`** (feeds `otfccbuild`'s no-input-file/piped-JSON
    path) used a `malloc`/`realloc`-growing buffer filled by `fgets` in a
    loop, measuring each chunk with `strlen` -- which stops at the first
    embedded NUL byte, so any stdin content after one silently vanished
    from `length` (and thus from the JSON text handed to `json_parse`)
    instead of erroring or being kept. `Read::read_to_end` copies exactly
    the bytes it receives with no such assumption, closing that class of
    bug structurally, the same way `readEntireFile`'s `std::fs::read`
    closed the short-read class of bug in the previous PR. The out-param
    stays a `malloc`'d `*mut c_char` for the same reason as
    `readEntireFile`'s: it's copied out of a `Vec<u8>` into the same
    `buffer`/`length` locals `readEntireFile` also writes, freed
    uniformly downstream with `free()`.
  - **`getchar`** (only used to block on a keypress for
    `--debug-wait-on-start`) moved from `fgetc(stdin)` to a single-byte
    `std::io::stdin().read(...)`, keeping its libc `getchar()`-style
    return convention (the byte read, or `-1` on EOF/error) since nothing
    about its one call site needed to change.
  - **No existing script exercises stdin input** (`compare-with-golden.sh`
    and friends always pass a file path), so this was verified by hand:
    piping each of the four `tests/payload/*.json` fixtures through
    `otfccbuild -q -o ... <` and diffing the result against the same
    fixture built from a path -- byte-identical in all four cases.
  - **Verification**: full pipeline green -- build, 245/245 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged,
    round-trips 10/10, lookup-alias regression clean, both fuzz targets
    `cargo check`-clean, `cargo miri test` clean with no new ignores, plus
    the by-hand stdin-vs-file byte comparison above.

- **Stage 7-4: the remaining file I/O -- `bin/otfccbuild.rs`'s
  `readEntireFile`, both binaries' output writes, and `logger.rs`'s stderr
  write -- moved off `libc::fopen`/`fread`/`fwrite`/`fclose`/`fputc` onto
  `std::fs`/`std::io`, completing the item `font/caryll_sfnt.rs` started.**
  - **`readEntireFile` had the same bug just fixed in `caryll_sfnt.rs`**:
    `fseek`/`ftell` determined a claimed length, then `fread`'s return
    value was discarded, so a read that returned fewer bytes than that
    claimed length (a race with concurrent truncation, or any other short
    read) left the `malloc`'d buffer's tail as uninitialized memory that
    still got treated as `length` valid bytes and fed to `json_parse`.
    `std::fs::read` reads to actual EOF into a `Vec<u8>` whose length is
    exactly what was read, structurally closing the class of bug rather
    than adding a check for it. The out-param stays a `malloc`'d
    `*mut c_char`, copied out of the `Vec` -- its caller's `buffer`/
    `length` locals are shared with `readEntireStdin` (still
    `malloc`/`realloc`/`fgets`-based, a separate, harder conversion, left
    for its own PR) and freed uniformly downstream with `free()`, so only
    the reading changed, not the buffer's ownership shape.
  - **Both binaries' output writes** (`otfccbuild.rs`'s OTF write,
    `otfccdump.rs`'s JSON write with its optional UTF-8 BOM, for both the
    `-o <path>` and stdout branches) moved to `std::fs::write`/
    `std::fs::File::create` + `Write::write_all`/`std::io::stdout()`. The
    file-output paths now surface a write failure as the same
    "Cannot write to file" error the old code only produced for an
    `fopen` failure -- the old `fwrite` return value was discarded, same
    shape as the `readEntireFile` bug but on the write side.
  - **`logger.rs`'s `LoggerTarget::Stderr` push** (the one function behind
    every one of this crate's ~425 logger calls) moved from
    `fwrite`/`fprintf(stderr, ...)` to `std::io::stderr().write_all(...)`.
  - **Verification**: full pipeline green -- build, 245/245 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged (exercises
    both the `otfccbuild` write path and every logger call),
    round-trips 10/10, lookup-alias regression clean, both fuzz targets
    `cargo check`-clean, `cargo miri test` clean with no new ignores (none
    of these three files' tests touch a real file directly -- golden/
    cycles/roundtrip scripts already exercise the JSON-input build path
    `readEntireFile` sits on, and log-output comparison exercises every
    `LoggerTarget::Stderr` call).

- **Stage 7-4: `font/caryll_sfnt.rs` moved off `libc::fopen`/`fread`/`fseek`
  onto `std::fs`/`std::io`, fixing the file-I/O bug the plan named as this
  item's motivation.** `otfcc_read_packets`'s second loop (reading each
  table's actual bytes) used to call `fread` and discard its return value
  -- a table whose declared `length` ran past the end of a truncated file
  was silently zero-padded instead of the read failing. `Read::read_exact`
  fails on a short read on its own, so this needed no separate byte-count
  check, the same way the header/directory fields already worked once
  Stage 7-3 fixed their own `exit()`-on-failure. Verified against a real
  truncated font by hand, not just by the new unit tests: `otfccdump` on a
  font truncated mid-table now cleanly reports `Cannot read SFNT file
  "...". Exit.` (exit 1) instead of building a font with that table's
  tail silently zeroed.
  - **`otfcc_read_sfnt` changed shape**: it used to take an already-`fopen`'d
    `*mut FILE`, with `otfccdump.rs`'s one caller doing the `fopen` itself;
    it now takes the path directly and opens the file internally (`std::fs::
    File::open`), so the caller no longer needs a `libc::fopen`/`FILE*` of
    its own for this at all. The reading functions
    (`otfcc_read_packets`/`otfcc_read_sfnt_body`/`otfcc_get16u`/`32`) are
    generic over `Read`/`Read + Seek` rather than hardcoded to `std::fs::
    File`, so a new sibling entry point, `otfcc_read_sfnt_from_reader`,
    can take any reader.
  - **Found while updating this function's one real caller**: the
    `otf_parse` fuzz target (`rust/fuzz/`, a separate cargo workspace the
    standard build/clippy/test pipeline never compiles -- caught only by
    explicitly `cargo check`ing it, per this migration's established
    habit) turned out to be a second caller, missed by grepping just
    `rust/src/`. It used to wrap the fuzzer-provided byte slice with
    `fmemopen` as a `FILE*` specifically because `otfcc_read_sfnt` was
    `FILE*`-shaped -- its own comment even named this as something
    "Stage 7-4 of the plan will eventually replace". Switched to
    `otfcc_read_sfnt_from_reader` with a `std::io::Cursor<&[u8]>`, keeping
    the same "wrap this byte slice, no real file on disk" shape without
    writing a real temp file on every one of its thousands-per-process
    iterations.
  - **Verification**: full pipeline green -- build, 245/245 tests (4 new,
    including one pinning the truncation bug directly), clippy clean, ABI
    unchanged, golden bytes and log output unchanged (`dump-missing-file`
    exercises the new open-failure path), round-trips 10/10, lookup-alias
    regression clean, both fuzz targets `cargo check`-clean (`otf_parse`
    needed real code changes, not just a recompile, confirmed by hand --
    this is exactly the kind of signature change that check exists to
    catch before CI does). `cargo miri test`: 224 passed, 0 failed, 21
    ignored (3 more than baseline) -- the 3 new tests that touch a real
    file (`nonexistent_path_returns_null`,
    `table_length_past_truncated_file_end_fails_instead_of_zero_padding`,
    `well_formed_single_table_font_reads_its_bytes_back`) hit Miri's
    filesystem isolation (`error: unsupported operation: 'open' not
    available when isolation is enabled`) and are marked
    `#[cfg_attr(miri, ignore = "...")]`, matching this crate's existing
    convention for Miri-incompatible tests; all 4 new tests run normally
    under `cargo test`.

- **Stage 7-4: `vendor/sds.rs` (1,368 lines, a transpiled port of redis's
  `sds` string type) removed entirely, replaced by a ~275-line
  `support/fmt.rs`.** Confirmed by grep before touching anything, not
  assumed: every `sds*()` function in the file (25+ of them --
  `sdsnewlen`/`sdscatlen`/`sdsdup`/`sdstrim`/`sdssplitargs`/etc.) had *zero*
  real call sites anywhere else in the crate. More surprising, so was
  `sdsbuild!` itself (the `SdsRaw`-returning macro `SdsPart::append_to`
  existed to serve) -- only its `Vec<u8>`-targeting sibling `bytesbuild!`
  (`SdsPart::append_to_vec`) is what any real code calls today. The whole
  redis-derived string reimplementation (`SdsHeader`, `sds_header`,
  `SDS_MAX_PREALLOC`, `Sds`, `SdsRaw`, `sdsbuild!`, and every `sds*()`
  function) was dead weight kept alive only because the file's own test
  suite built through it to cross-check formatting against libc's
  `snprintf` -- rewriting those tests to call `bytesbuild!` directly (which
  already returns an owned `Vec<u8>`, so no `sdslen`/`sdsfree`/raw-pointer
  read-back needed either) removed that last dependency. What's left --
  the `SdsPart` trait (now just `append_to_vec`), the formatting newtypes
  (`Byte`/`Hex4`/`Hex4Upper`/`Hex2`/`Hex2Upper`/`Dec5`), and `bytesbuild!`
  itself -- moved to `support/fmt.rs`, since none of it is "vendored" C
  code anymore.
  - **3 of the file's 10 tests don't have an equivalent in the new file,
    and shouldn't**: `newlen_from_null_init_is_zero_filled`,
    `cat_grows_and_keeps_embedded_nul`, `dup_is_independent_of_the_original`
    tested the `sds` header/growth/dup mechanics directly -- code that no
    longer exists, since `Vec<u8>`'s growth and `Clone` are the standard
    library's problem to test, not this crate's. One test
    (`sds_part_keeps_embedded_nul_but_c_string_does_not`, contrasting `%S`
    vs. `%s`) was rewritten rather than dropped, since the "byte slice
    keeps embedded NULs, C string stops at the first one" distinction it
    pinned still holds between `&[u8]` and `*const c_char` -- just not
    via `Sds` anymore, which is gone. Total library test count: 244 → 241,
    all three drops accounted for above, nothing live lost coverage.
  - `sdsget_cff_sid` (`libcff/cff_string.rs`, unrelated to this file but
    named after it) renamed to `get_cff_sid` -- it already returned
    `Option<Vec<u8>>`, not an `sds`, so the prefix was a pure holdover;
    15 call sites in `table/cff.rs` updated.
  - **Verification**: full pipeline green -- build, 241/241 tests
    (244 minus the 3 explained above), clippy clean, ABI unchanged, golden
    bytes and log output unchanged (log/JSON text is exactly what
    `bytesbuild!` builds, crate-wide), round-trips 10/10, lookup-alias
    regression clean, `cargo miri test` identical to baseline, both fuzz
    targets `cargo check`-clean.

- **Stage 7-4: `EndianProbe16`/`EndianProbe32` (`support/binio.rs`),
  `CffDoubleBits` (`table/cff.rs`), and `DoubleBits`
  (`vendor/emyg_dtoa.rs`) removed -- all four were the same
  bit-reinterpretation shape `ComponentArg` was, just for different types.**
  - `EndianProbe16`/`32` were a runtime host-endianness probe (`font/
    caryll_sfnt.rs`'s `otfcc_check_endian`) plus a conditional byte-swap
    (`otfcc_endian_convert16`/`32`) applied to a value `fread` had just
    copied a file's big-endian bytes into byte-for-byte -- exactly what
    `u16::from_be`/`u32::from_be` do, built into the standard library
    without a union or a runtime probe. (This is the same fix `font/
    caryll_sfnt_builder.rs`'s checksum computation already got, in an
    earlier Stage 7-3 PR, for its own copy of this pattern -- `caryll_sfnt.
    rs`'s was the last one left, confirmed by grep before starting.) Both
    functions, `otfcc_check_endian`, and the two `EndianProbe*` unions
    (zero other users crate-wide) are gone.
  - `CffDoubleBits` (`callback_draw_getrand`, the CFF outline builder's
    xorshift-based random-double generator) and `DoubleBits`
    (`diy_fp_from_double`, `emyg_dtoa`'s float-to-shortest-decimal
    conversion) each had exactly one construction site, immediately
    followed by reading the *other* field -- a `u64` written, an `f64`
    read (or vice versa), the classic "reinterpret this float's bits as an
    integer" trick. `f64::from_bits`/`f64::to_bits` are that exact
    operation without a union.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged (CFF
    hinting and `otfccdump`'s missing-file path both exercise code this
    touched), round-trips 10/10, lookup-alias regression clean, `cargo
    miri test` identical to baseline, both fuzz targets `cargo
    check`-clean.

- **Stage 7-4: `table/glyf/build.rs`'s `ComponentArg` union removed.**
  `union { pointid: u16, coord: i16 }` turned out to be a pure same-size
  bit-reinterpretation trick, not a real tagged value: `arg1`/`arg2` are
  written as whichever type a composite glyph's arguments actually are
  (point indices via `.pointid`, coordinate deltas via `.coord`), then
  always read back as `.pointid` a few lines later regardless of which one
  was written, relying on the union's overlapping storage to reinterpret
  an `i16`'s bits as `u16` for `bufwrite16b`/`bufwrite8`. A plain `as u16`
  cast on the same-width integer does the identical bit-preserving
  reinterpretation, so `arg1`/`arg2` are now just `u16` locals produced by
  an `if`/`else` expression (one arm building them from `u16` point
  indices, the other from an `i16` coordinate cast to `u16` at the point
  of construction) instead of a union write/reinterpret-read pair.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged
    (composite-glyph payloads exercise this function directly),
    round-trips 10/10, lookup-alias regression clean, `cargo miri test`
    identical to baseline, both fuzz targets `cargo check`-clean.

- **Stage 7-4: `CffValueBody` union → `CffValue` enum, the largest single
  item in this stage.** 281 of the crate's 290 `c2rust_unnamed` (tagged-
  union field access) occurrences belonged to this one type, 231 of those
  concentrated in a single file (`libcff/cff_parser.rs`). A focused survey
  first (see prior "Next steps" entries' pattern) found the real shape
  before writing any code: `CffValue` becomes
  ```rust
  pub enum CffValue { Unset, Operator(i32), Integer(i32), Double(f64) }
  pub fn cffnum(val: CffValue) -> f64 { /* Integer/Double -> f64, else 0.0 */ }
  ```
  `Operator` genuinely carries an opcode (confirmed via both write sites in
  `libcff/cff_codecs.rs`'s decoders and read sites in `libcff/cff_dict.rs`/
  `cff_parser.rs`'s dispatch), so -- unlike `Unset` -- it isn't a bare unit
  variant.
  - **A real, previously-silent bug found while designing the fix, not
    just while surveying it**: `libcff/cff_dict.rs`'s `parse_dict_key`
    used to encode "key not found" as `t: Unset, c2rust_unnamed.i: -1` and
    every one of 11 call sites in `cff_parser.rs` read `.c2rust_unnamed.i`
    unconditionally, trusting that `-1` sentinel without ever checking
    `.t` -- meaning a crafted font that encoded a DICT operand as a *real
    number* instead of an integer would have its `f64` bit pattern
    silently misread as an `i32` offset/length. A first-pass fix
    (`cffnum(val) as i32`, matching a pattern this migration's earlier
    surveys called reasonable in the abstract) would have **changed the
    sentinel value from -1 to 0** for the legitimate "not found" case,
    since `cffnum(Unset) == 0.0` -- a real behavior regression caught only
    by reading the 11 call sites' actual `if offset_0 != -1 { ... }` usage
    before implementing, not by the type signature alone. Fixed instead
    with a dedicated `parse_dict_key_int(...) -> i32` that matches the
    variant explicitly and returns `-1` for `Unset`/`Operator`, preserving
    the exact existing "not found" convention for the common case while
    still converting a genuinely-`Double` DICT value correctly (`d as
    i32`) instead of reading raw union bytes.
  - **`table/cff.rs` had the same shape, 9 more sites**: `.c2rust_unnamed.i
    as u16` SID/CID-field reads in `callback_extract_fd`/
    `callback_extract_private`, guarded only by an operand-*count* check,
    never a `.t` check. These use `cffnum(val) as u16` instead (not
    `parse_dict_key_int`'s `-1` convention, since these read `CffValue`s
    directly off a CharString/DICT operand stack mid-callback, not through
    `parse_dict_key`'s "not found" API) -- unifying them with the
    `cffnum()`-based reads a few lines away in the same file that were
    already written correctly.
  - **A second dead branch found while converting, not before**:
    `table/cff.rs`'s `cffdict_input_array` took a runtime `CffValueType`
    parameter to choose between a `Double` and an `Integer` encoding path,
    but all 6 call sites (`cff_make_private_dict`) passed `Double` --
    confirmed by grep, matching what the function's own pre-existing doc
    comment had already suspected ("the runtime branch on `t` is really
    two functions") without anyone following through and checking. The
    `Integer` branch and the `t` parameter are gone; the function is now a
    thin empty-check wrapper around `cffdict_input_doubles`.
  - **Zero external-to-CFF blast radius**: confirmed by grep before
    starting that `CffValue`/`CffValueBody`/`CffValueType` are used only
    in `libcff/cff_value.rs`, `cff_codecs.rs`, `cff_dict.rs`,
    `cff_parser.rs`, and `table/cff.rs` -- no other subsystem touches
    them.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged
    (**including `KRName-Regular-O2.otf` (CFF subroutinize), the one
    payload that exercises the DICT/CharString stack machinery this PR
    rewrote most heavily**), round-trips 10/10, lookup-alias regression
    clean, `cargo miri test` identical to baseline, both fuzz targets
    `cargo check`-clean.

- **Stage 7-3 closes: `bin/otfccdump.rs`/`bin/otfccbuild.rs`'s `exit()` calls
  → `main() -> ExitCode`.** The last named Stage 7-3 item. Both binaries
  already had a `main_0(argc, argv) -> c_int` doing all the real work,
  with `pub fn main()` just forwarding its return value to `std::process::
  exit()` -- so the fix is narrower than it sounds: convert the 12
  `exit(EXIT_FAILURE)` call sites scattered through each `main_0` into
  plain `return EXIT_FAILURE;` (both already return the same `c_int`
  type, and `EXIT_FAILURE`'s value, `1`, is unchanged), and change each
  `main()` from `std::process::exit(main_0(...))` to `std::process::
  ExitCode::from(main_0(...) as u8)`.
  - **One site per file needed more than a substitution**:
    `otfccbuild.rs`'s `readEntireFile` -- a helper `main_0` calls, not
    `main_0` itself -- had 2 of its own `exit()` calls. Converted its
    signature from `-> ()` (writing through two out-parameters) to `->
    bool`, its two `exit()`s to `return false;`, and its one call site in
    `main_0` to check the result and `return EXIT_FAILURE;` itself on
    failure -- the same "propagate a failure signal up to the one place
    that already owns process-exit semantics" shape `font/caryll_sfnt.rs`'s
    `otfcc_get16u`/`otfcc_get32u` -> `Option` conversion used earlier in
    this stage. Every other `exit()` site in both files (10 of the 12) was
    already directly inside `main_0`, confirmed by checking each site's
    enclosing function before assuming a plain substitution would work.
  - **Verified by hand, not just by inspection**: built both binaries and
    ran a success path and a failure path through each --
    `otfccdump`/`otfccbuild --help` exit 0, `otfccdump` on a missing file
    and `otfccbuild` on unparseable JSON both exit 1 with their existing
    error messages intact (`otfccbuild : Parse into JSON : [ERROR] Cannot
    parse JSON file "...". Exit.`).
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged (the
    `dump-missing-file` golden log case exercises one of these exit
    paths directly), round-trips 10/10, lookup-alias regression clean,
    `cargo miri test` identical to baseline, both fuzz targets `cargo
    check`-clean.

- **Stage 7-3: `table/_tsi.rs`'s c2rust-residue `panic!` removed.**
  `propergid` had a numeric `switch` over `TsiEntryType as c_uint` with a
  fallthrough `panic!("Reached end of non-void function without
  returning")` for "no case matched" -- but `TsiEntryType` is a closed
  5-variant enum (`Glyph=0`, `Fpgm=1`, `Prep=2`, `Cvt=3`,
  `ReservedFffc=4`) and the switch already covered all five numerically,
  so that arm was unreachable, not a real error path. Rewritten to match
  on the enum directly instead of its numeric cast, so the exhaustiveness
  is compiler-checked (a future variant added without updating this match
  is now a build error, not a runtime panic reachable in production) and
  the `panic!` arm falls away with it, needing no `_ => unreachable!()`
  replacement.
  - **Found and documented, not fixed, while reading the surrounding
    control flow**: `push_tsi_entries` calls `propergid` with a null
    `entry` from its own `min_n`-padding loop -- sound only because it's
    never called with `type_0 == TsiEntryType::Glyph` (the one arm that
    dereferences `entry`) and a nonzero `min_n` at the same time
    (`otfcc_build_tsi`'s `TsiEntryType::Glyph` call site always pairs it
    with `min_n = 0`). A future call site could violate that pairing;
    left as a documented invariant rather than an added runtime check,
    since it isn't the dead code this PR set out to remove and changing
    it needs its own look at whether a genuine fix or just an assert is
    warranted.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged
    (`tsi5`/`_tsi`-table payloads exercise `propergid` directly),
    round-trips 10/10, lookup-alias regression clean, `cargo miri test`
    identical to baseline, both fuzz targets `cargo check`-clean.

- **Stage 7-3: `font/caryll_sfnt.rs`'s short-read `exit()`s removed.**
  `otfcc_get16u`/`otfcc_get32u` (the sfnt header/table-directory readers)
  called `libc::exit(EXIT_FAILURE)` straight from a raw `fprintf` to
  `stderr` on a truncated file -- bypassing the `Logger` and whatever
  caller was mid-read. They now return `Option<u16>`/`Option<u32>`
  (`None` on a short `fread`), propagated with `let Some(x) = ... else {
  return false; }` through a new `otfcc_read_sfnt_body` (split out of
  `otfcc_read_sfnt` so the "free the partial `font`, return null" cleanup
  lives in one place) and `otfcc_read_packets` (now `-> bool`), up to
  `otfcc_read_sfnt` returning null on failure instead of aborting the
  process mid-read.
  - **No caller changes needed.** `otfcc_read_sfnt`'s only caller
    (`bin/otfccdump.rs`) already null-checks its return (`sfnt.is_null()
    || (*sfnt).count == 0`) and logs a clean `"Cannot read SFNT file
    \"...\". Exit."` through the normal `Logger` channel before exiting --
    this fix just reuses that existing path instead of adding a new one,
    confirmed by hand (`head -c 2`/`head -c 20` truncations of a real
    font both now produce that message and exit 1, and the old raw
    `"File corruption of terminated unexpectedly.\n"` string -- note the
    pre-existing grammar bug, which is now gone entirely rather than
    fixed in place -- no longer appears in the built binary).
  - **Scope note, checked before implementing**: the plan named 4
    `exit()`-related Stage 7-3 items. `support/alloc.rs`'s OOM `exit()`s
    were the first (previous entry below); `ffi/dll.rs`'s `Options` leak
    turned out already fixed; this is the third. `table/_tsi.rs:381`'s
    c2rust-residue `panic!` remains for a follow-up PR, along with the 15
    `exit()`/`std::process::exit` call sites across `bin/*.rs`.
  - **Deliberately left alone**: `otfcc_read_packets`'s second `fread`
    (reading each table's actual bytes) still discards its return value --
    a truncated file there silently zero-pads instead of failing. This is
    a real, already-known bug, but the plan puts it in Stage 7-4 (`ファイ
    ル I/O 22箇所 → std::fs/std::io`) alongside 21 other same-shaped call
    sites, not here -- fixing just this one now would be inconsistent
    with how the rest get fixed later, and this PR's `Option`-based fix
    is unrelated to that one's cause (a discarded return value, not a
    process-exit).
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged,
    round-trips 10/10, lookup-alias regression clean, `cargo miri test`
    identical to baseline, both fuzz targets `cargo check`-clean.

- **Stage 7-3 begins: `support/alloc.rs`'s OOM `exit()` → `handle_alloc_error`.**
  `__caryll_allocate_clean`/`__caryll_reallocate` (the crate's shared
  calloc/realloc wrappers) printed a custom `"[<line>]Out of memory(N
  bytes)"` message and called `libc::exit(EXIT_FAILURE)` on a null
  return from `calloc`/`realloc`. Replaced with `std::alloc::
  handle_alloc_error(Layout::from_size_align(n, 1).unwrap())`, the
  standard-library idiom for exactly this situation (a manual allocation
  that failed and can't be recovered from) -- same "log then abort,
  can't continue" behavior, minus the per-call-site line number in the
  message (now reports the failed allocation's size instead, via the
  standard "memory allocation of N bytes failed" abort message).
  - Checked first, per this migration's discipline of verifying a
    plan item is still accurate before touching it: the plan named 3 more
    `exit()` sites for this stage (`ffi/dll.rs`'s `Options` leak,
    `font/caryll_sfnt.rs:229,248`'s short-read `exit()`s,
    `table/_tsi.rs:381`'s c2rust-residue `panic!`) -- the `ffi/dll.rs`
    leak turned out already fixed (in earlier, pre-Stage-7 work, per its
    own in-file comment), so this PR covers only the still-open
    `support/alloc.rs` item; the other two remain for follow-up PRs.
  - `line: c_ulong` kept as a parameter (renamed `_line`) rather than
    removed from every one of the ~50 call sites across the crate that
    still pass one -- the same choice `bk/bkgraph.rs`'s
    `compute_block_offsets` made for its own now-vestigial `_line`.
  - **Verification**: full pipeline green -- build, 244/244 tests,
    clippy clean, ABI unchanged, golden bytes and log output unchanged,
    round-trips 10/10, lookup-alias regression clean, `cargo miri test`
    identical to baseline, both fuzz targets `cargo check`-clean.

- **Stage 7-2-f, the last item: `BkBlock`/`BkCellValue::Ptr` Box化.** A
  focused survey (`bk/bkblock.rs`, `bk/bkgraph.rs`, and every `table/*.rs`
  file touching `*mut BkBlock` -- 21 files, ~128 occurrences) found this was
  actually smaller and lower-risk than the already-completed `libcff/subr.rs`
  arena conversion, contrary to earlier surveys' assumption. Every `BkBlock`
  is single-parent-owned: real call sites always finish building a child
  completely (via `bk_new_block`/`bk_push`) before splicing it into a
  parent, no `BkBlock` is ever shared across two `BkGraph`s (`gpos_pair.rs`'s
  two-graph case builds fully independent trees and always deletes one
  entirely before touching the other), no cycle is constructible (there's no
  API for a child to reference an ancestor while the ancestor is still being
  built), and teardown is centralized to exactly two places
  (`bk_delete_graph`'s flat walk over an already-built tree, and
  `bkpushitems`'s `Embed` arm, which frees a block immediately after copying
  its cells). That ownership discipline means `BkCellValue::Ptr(*mut
  BkBlock)` **stays a raw pointer** rather than becoming an arena index --
  `libcff/subr.rs`'s tombstoned-arena template is only needed where slots
  get deleted and revisited mid-algorithm, which never happens here. Instead:
  `BkBlock.cells: *mut BkCell` (hand-managed with `length`/`free` slack
  bookkeeping, grown via raw `realloc`) becomes `Vec<BkCell>`, and `BkBlock`
  itself moves from `__caryll_allocate_clean`/raw `free` to
  `Box::into_raw`/`Box::from_raw`, the same malloc-shell removal every other
  `_create()` in this migration has had.
  - **Zero external files needed changes.** Every one of the 21 files
    touching `*mut BkBlock` outside `bk/` (`table/base.rs`, `table/cmap.rs`,
    `table/colr.rs`, `table/cpal.rs`, `table/gdef.rs`, `table/svg.rs`,
    `table/otl/build.rs`, `table/vdmx/funcs.rs`, `table/meta/build.rs`, 9
    `table/otl/subtables/*.rs` files, `table/otl/subtables/chaining/
    build.rs`) only ever calls the opaque `bk_new_block`/`bk_push`/`bk_ptr`/
    `bk_int`/`bk_build_block`/etc. API and passes `*mut BkBlock`/`*mut
    BkGraph` around as an opaque handle -- confirmed by grep before writing
    any code, not assumed. `bk_cell_is_pointer` changed from `*mut BkCell`
    to `&BkCell` (no longer even needs `unsafe`) since nothing outside `bk/`
    called it either. This PR is confined to `bk/bkblock.rs` and
    `bk/bkgraph.rs`.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean (including the `dangerous_implicit_autorefs` lint on `Vec`-index
    writes through a raw-pointer field, fixed with an explicit `&mut
    Vec<BkCell>` local the same way `support/buffer.rs`/`table/glyf/
    build.rs` needed it earlier), ABI unchanged, golden bytes and log output
    unchanged (exercises `bk_untangle_graph`'s Sp16/Sp32 promotion path,
    the part of this rewrite most likely to have an off-by-one), round-trips
    10/10, lookup-alias regression clean, `cargo miri test` 226/0/18
    identical to baseline, both fuzz targets `cargo check`-clean.
  - **The plan's Stage 7-2-f also named two more raw pointers besides
    `BkCellValue::Ptr`: `VqSegmentDelta.region`/`RegionKey.0`
    (`(*const VqRegion)`).** Traced concretely rather than assumed (prompted
    by a status question after this PR opened): both already have the same
    "borrowed pointer into a longer-lived, individually `Box`-owned value,
    freed exactly once, never revisited mid-algorithm" shape `Feature.
    lookups`/`LanguageSystem.features` (`table/otl.rs`) already document and
    rely on -- `VqSegmentDelta.region` always ends up holding the canonical
    pointer `fvar_register_region` returns, which lives inside `FvarTable.
    masters` (`vf/region.rs`'s `vq_create_region` gives each `VqRegion` its
    own individual `Box`) and is disposed exactly once, by `FvarTable`'s own
    `Drop`, at final `Font` teardown; a content-duplicate region is freed
    immediately during registration, before its pointer is ever handed to a
    `VqSegmentDelta`. No code change needed -- doc comments added on both
    types recording the reasoning, so this doesn't need re-deriving later.
    With this, every raw pointer Stage 7-2-f named is now resolved (either
    converted, in `BkCellValue::Ptr`'s case, or confirmed already-sound and
    documented as such), closing out Stage 7-2 entirely.

- **Fixed the `caryll_sfnt_builder.rs` checksum alignment UB surfaced by the
  `Font` Box化 fix above.** `buf_checksum`/`create_segment` cast a
  `Buffer.data: Vec<u8>` pointer (1-byte aligned) to `*mut u32` and
  dereferenced it to sum a table's bytes as big-endian u32 words --
  undefined behaviour whenever the cast pointer isn't 4-byte aligned,
  independent of whether the table's length was a multiple of 4 (`Vec<u8>`
  makes no alignment promise beyond 1). `otfcc_check_endian`/
  `otfcc_endian_convert32` (a runtime host-endianness probe via a union,
  used only to interpret each loaded native-endian `u32` as big-endian) are
  removed from this file entirely -- both call sites now go through a new
  `buf_checksum_bytes(&[u8]) -> u32` helper that reads each 4-byte window
  with `chunks_exact(4)` + `u32::from_be_bytes`, which is both alignment-safe
  and endianness-portable without a runtime check. `buflongalign` (called by
  both callers immediately before) always pads `data` to a multiple of 4, so
  `chunks_exact(4)` covers every byte with no remainder. The sibling
  `otfcc_check_endian`/`otfcc_endian_convert32`/`otfcc_endian_convert16` in
  `font/caryll_sfnt.rs` are untouched -- those read through `fread` into an
  aligned stack local, not a pointer cast onto `Vec<u8>`, so they were never
  affected.
  - The two `ffi::dll::tests` that had been re-ignored under miri for this
    bug (`minimal_json_builds_and_frees_cleanly`, `repeated_calls_do_not_
    crash`) are un-ignored; both pass under `cargo miri test` now.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged (checksum
    values themselves were arithmetically correct despite the UB, so output
    is byte-identical to before), round-trips 10/10, lookup-alias regression
    clean, `cargo miri test` clean including both previously-masked tests,
    both fuzz targets `cargo check`-clean.

- **Stage 7-2-d, the last `_create()`: `otfcc_font_create` (`Font` itself),
  Box化.** A focused survey found this was not the cleanup it looked like --
  it's a live, miri-confirmed UB bug. `Font` has 6 `Option<Vec<T>>` fields
  (`glyf`, `name`, `colr`, `svg`, `tsi_01`, `tsi_23`); unlike `Option<Box<T>>`,
  `Option<Vec<T>>` does not niche-optimize an all-zero bit pattern to `None`
  (`Vec`'s niche lives in `Cap`'s valid-range restriction, not in "pointer ==
  0"), so `otfcc_font_create`'s `malloc`+`memset`-zero followed by
  `json_reader.rs`/`otf_reader.rs`'s plain `(*font).glyf = ...` assignments
  ran the compiler-generated drop-the-old-value glue on a bogus `Some`,
  which is exactly the `Options.logger` calloc-trap (see
  `otfcc-vec-field-assign-needs-calloc`), just recurring on `Font` itself.
  Reproduced directly with `cargo miri test -- --ignored
  ffi::dll::tests::minimal_json_builds_and_frees_cleanly` before the fix:
  `constructing invalid value of type std::ptr::Unique<u8>`, inside
  `Vec::drop` reached through `Option<GlyfTable>`'s drop glue.
  - **Fix**: `otfcc_font_create` is now `Box::into_raw(Box::new(Font {
    subtype: FontSubtype::Ttf, fvar: None, ..., glyph_order: None }))` --
    every one of the 33 non-`subtype` fields defaults to the literal `None`,
    no per-type default-value judgment calls needed (lower risk than
    `CffTable`'s 24-field literal). `otfcc_font_free` is now `drop(Box::
    from_raw(x))`. `init_font`/`dispose_font`/`otfcc_font_dispose`/
    `otfcc_font_init` all removed as dead code; `delete_font_table` (used
    elsewhere for selective single-table clearing, e.g. `otf_writer/stat.rs`)
    is untouched.
  - **Two pre-existing leaks fixed as a side effect**: the old `dispose_font`
    explicitly null'd 31 of `Font`'s 33 table fields before the struct's
    memory was `free()`'d raw; `fvar` and `vdmx` were the two fields it
    missed, so whatever they pointed to was never reclaimed on font
    teardown (raw `free()` runs no field `Drop` glue). `drop(Box::from_raw
    (x))` now drops every field through its own `Drop` impl regardless of
    whether the old hand-written list covered it. Stale comment in
    `table/fvar.rs` documenting the `fvar` leak as deliberately-preserved
    corrected in place.
  - **A second, unrelated, pre-existing bug surfaced by unblocking miri
    further**, not fixed here: `font/caryll_sfnt_builder.rs`'s
    `create_segment`/`otfcc_checksum` cast a `Buffer.data: Vec<u8>` pointer
    (1-byte aligned) to `*mut u32` and dereference it -- an alignment
    violation whenever a table's byte length from an aligned start isn't a
    multiple of 4. The two `ffi::dll::tests` that build a real `Font` used
    to be miri-ignored for the `Font` bug above and never got far enough
    under miri to hit this one; they're now miri-ignored again with an
    updated reason pointing at this bug instead. Left for a dedicated PR --
    out of scope for a `Font` Box化 change, and deserves its own look at
    whether `create_segment`/`otfcc_checksum` should read via
    `u32::from_ne_bytes` on a 4-byte slice instead of a pointer cast.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged, round-trips
    10/10, lookup-alias regression clean, `cargo miri test` 224/0/20
    identical to baseline (with the alignment bug now masked again by the
    updated ignore reasons rather than the old Font one), both fuzz targets
    `cargo check`-clean.

- **Stage 7-2-h (`repr(C)` removal): 112 of 128 `#[repr(C)]` struct
  attributes removed.** A prior survey found 116 of 118 struct/enum types
  carrying `#[repr(C)]` were non-load-bearing and every name already
  `non_camel_case_types`-conformant (so removal would need zero renames);
  actually implementing it added its own per-type safety pass beyond
  naming -- checking every candidate for a still-`extern "C"` by-value
  crossing, a `transmute`/union reinterpretation, and finally just
  building under `-D warnings` as the real safety net (`improper_ctypes`
  would have caught anything the greps missed).
  - **3 more types turned out load-bearing, beyond the 2 the survey
    already knew about** (`support/getopt.rs`'s `LongOption`, `vendor/
    sds.rs`'s `SdsHeader`): `table/fvar.rs`'s `FVARHeader`, `InstanceRecord`,
    `VariationAxisRecord`. `otfcc_read_fvar` does `data as *mut FVARHeader`
    directly on raw on-disk font bytes (`data` comes straight from
    `table.data.as_ptr()`), then reads fields through that pointer --
    exactly the same "byte-exact overlay onto foreign bytes" shape
    `SdsHeader` has, just missed by the earlier naming-only pass since it
    required tracing where the pointer that gets cast to the type
    actually comes from, not just grepping for `transmute`. All three
    already carried `#[repr(C, packed)]`; left the whole attribute
    untouched rather than guessing whether `packed` alone would have been
    safe to keep without `C`.
  - **112 removed cleanly** — every payload still parses/dumps/builds
    byte-identical, confirming these were genuinely representation-only
    attributes with no code anywhere depending on C field ordering.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean (including zero new `non_camel_case_types`/`improper_ctypes`
    warnings, confirming the "no renames needed" finding held), ABI
    unchanged, golden bytes and log output unchanged, round-trips 10/10,
    lookup-alias regression clean, `cargo miri test` 224/0/20 identical to
    baseline, both fuzz targets `cargo check`-clean. `repr(C)` census: 11
    real attributes remain (down from the prior 118 struct/enum count plus
    6 unions, i.e. 112 removed of 118 struct/enum candidates) -- the 5
    load-bearing structs above, plus the 6 unions this pass deliberately
    left untouched (`CffValueBody`, `EndianProbe32`, `EndianProbe16`,
    `CffDoubleBits`, `DoubleBits`, `ComponentArg`, all still Stage 7-4
    work).

- **Three quick wins landed together after a fresh feasibility survey found
  each of them tractable now, contrary to earlier deferrals**: the last 3
  `_create()` malloc shells (Stage 7-2-d) and `VqRegion`'s C flexible-
  array-member shape (part of Stage 7-2-f). A real type-confusion bug was
  also found and fixed along the way.
  - **Stage 7-2-d, the 3 remaining `_create()`s**: `otfcc_glyph_order_create`
    (`support/glyph_order.rs`), `table_glyf_create_n` (`table/glyf.rs`),
    `table_cff_create` (`table/cff.rs`) all converted to `Box::into_raw(
    Box::new(...))`, same shape as the 10 OTL subtable `_create()`s
    converted earlier. Two of the three deferral reasons had gone stale
    since they were written: `table_glyf_create_n`'s comment said it
    couldn't convert because `Font.cff` "isn't Box化'd yet" -- it already
    was (`Option<Box<CffTable>>`, from earlier Stage 6-4 work); `table_cff_
    create`'s comment said converting it would force every recursive
    `fd_array` call site to become `Box`-aware -- `fd_array` was already
    `Vec<Box<CffTable>>` (from Stage 7-2-c). Both stale comments were
    corrected in place, not just the code. `otfcc_font_create` (`Font`
    itself) stays deferred -- `*mut Font` still appears in ~87 places
    including the FFI boundary, genuinely large work -- but a smaller,
    real first step was identified for later: `json_reader.rs`/`otf_
    reader.rs`'s `(*font).field = ...` writes onto freshly-calloc'd memory
    could move to `ptr::write` field-by-field, the same fix already applied
    to `Options`'s own calloc'd construction, without touching `Font`'s
    allocator or its `*mut Font` type at all.
  - **`VqRegion`** (`vf/region.rs`): was a C flexible-array-member struct
    (`spans: [VqAxisSpan; 0]`, allocated as one `dimensions`-header-plus-
    trailing-spans block, indexed everywhere via pointer arithmetic past
    the struct's own address) -- now `{ dimensions: ShapeId, spans: Vec<
    VqAxisSpan> }`. `vq_compare_region` moved from a `strncmp` byte-
    identity comparison over the whole block to a structural `dimensions`-
    then-`spans` comparison (`VqAxisSpan` gained `PartialEq`/`PartialOrd`);
    it's only ever consumed as an ordering/equality signal, never for
    byte-identity, so this is behavior-preserving. `RegionKey` (`table/
    fvar.rs`), which DOES need byte-identity (it's an `IndexMap` dedup
    key mirroring the original uthash table's `memcmp` semantics), now
    hashes/compares `dimensions` and `spans` as two separate byte views
    instead of one contiguous range -- incidentally more correct than
    before, since the old single-range view also swept in a few bytes of
    zeroed alignment padding between the two fields.
  - **`VqSegmentDelta.region`/`RegionKey.0` deliberately stayed raw
    pointers**, not converted to indices into `FvarTable.masters`'s
    `IndexMap`, per the plan's own Stage 7-2-f section. An `IndexMap`
    insert can still reallocate its own backing array and move the
    `FvarMaster` values it stores, but `VqRegion` itself now lives in
    stable `Box`-owned heap storage -- the pointee address doesn't move
    just because the map reallocates, so an index buys nothing here that
    a raw pointer doesn't already have, at the cost of needing a back-
    reference to resolve it (`VqSegmentDelta` currently needs none).
  - **A real bug found and fixed**: `otf_reader/unconsolidate.rs`'s
    `hash_vqs` (used for duplicate-glyph detection when naming glyphs by
    content hash) read `VqRegion.spans` via the old raw pointer-offset
    trick, which stayed *syntactically* valid after the `Vec` conversion
    (pointer-to-pointer casts always compile) but became a genuine type
    confusion: for axis index 0 it read `Vec<VqAxisSpan>`'s own internal
    (ptr, len, capacity) representation as if it were span data, and for
    index ≥ 1 it read out of bounds past the `Vec`'s own struct. None of
    the 244 lib tests exercise this path (it only fires for a font with
    real gvar tuple-variation deltas being name-hashed), so it wasn't
    caught by the type checker or the test suite -- only by tracing every
    read site of the field being converted, the same discipline this
    migration has applied to every risky conversion. Fixed to index
    `spans` directly; `KRName-Regular`/`gvar-test.ttf`'s golden bytes
    (which do exercise gvar deltas) stayed byte-identical after the fix,
    consistent with (though not conclusive proof of) correctness --
    genuinely wrong hash bytes would very likely have produced a
    different, detectable checksum.
  - **Deferred, per the same survey**: `BkCellValue::Ptr`/`BkGraphNode.
    block` (`bk/bkblock.rs`/`bk/bkgraph.rs`) would need an arena for
    individually-`malloc`'d `BkBlock`s, similar in shape to the completed
    `libcff/subr.rs` conversion but larger -- `*mut BkBlock` appears
    ~128 times across 21 otherwise-unrelated table builder files (vs.
    `subr.rs`'s single self-contained file), so it needs its own dedicated
    effort, not a fold-in to this batch. `Feature.lookups`/`LanguageSystem.
    features` (weak `*const Lookup`/`*const Feature` refs into `OtlTable`'s
    `Vec<Box<Lookup>>`/`Vec<Box<Feature>>`) turned out not to need
    conversion at all: since the referents are `Box`-owned, `Vec::retain`
    compaction reorders the *handles*, not the heap-allocated values
    themselves, so existing raw-pointer refs already survive compaction
    correctly -- the plan's "index into the Vec" framing for this pair was
    an overgeneralization from `subr.rs`'s different (plain `Vec<T>`, no
    `Box` indirection) hazard shape.
  - `repr(C)` removal (Stage 7-2-h's other half) was surveyed too: of 118
    struct/enum types carrying `#[repr(C)]`, only 2 remain load-bearing
    (`LongOption`, `SdsHeader`) and every one of the other 116 already has
    a fully `UpperCamelCase`-conformant name -- removing `#[repr(C)]`
    would be a zero-risk, single-pass mechanical change with no rename
    sub-batch needed. Not done in this PR (scoped to the 4 items above),
    but ready to execute as the next step.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes byte-identical on every payload
    (including `gvar-test.ttf`, the one exercising the `hash_vqs` fix) and
    log output unchanged, round-trips 10/10, lookup-alias regression
    clean, `cargo miri test` 224/0/20 identical to baseline, both fuzz
    targets `cargo check`-clean. `survey-unsafe.sh`: raw pointer types
    5774→5746, `.offset(` calls 843→838.

- **Stage 7-2-e: `Buffer` internally owns its bytes via `Vec<u8>` instead
  of manual `malloc`/`realloc`/`free`.** The single largest remaining
  conversion in the whole migration -- `bufwrite*` alone has ~510 call
  sites across 58 files -- landed as one PR by keeping every public
  function in `support/buffer.rs` at its existing `*mut Buffer` signature
  (`bufnew`, `buffree`, `bufwrite8`/`16b`/`32b`/..., `buflen`, `bufpos`,
  `bufseek`, `bufwrite_bytes`, `bufwrite_buf`, the `bufping`/`bufpong`
  offset-backpatching family, ...): the ~42 files that only ever call
  through those wrappers needed **zero changes**, verified by the build
  itself once `buffer.rs` compiled clean. The real work concentrated in
  `support/buffer.rs` itself and the ~19 files that poke `.cursor`/
  `.size`/`.data` directly.
  - **`Buffer`'s shape**: `{ cursor: usize, size: usize, free: usize, data:
    *mut u8 }` → `{ cursor: usize, data: Vec<u8> }`. `size` is gone,
    collapsed into `.data.len()` at every read site (mechanical, ~90
    substitutions). `free` -- the hand-tracked spare-capacity bookkeeping a
    manual `realloc` strategy needed, including a 16 MiB per-reallocation
    growth cap -- has no replacement at all: `Vec`'s own capacity
    management replaces the whole mechanism, and nothing outside this file
    ever read `.free` (confirmed by grep before starting). `bufnew()` is
    `Box::into_raw(Box::new(...))` now, not `__caryll_allocate_clean`'d
    (an all-zero `Buffer` stopped being a valid bit pattern the instant
    `data` became a `Vec`, the same calloc hazard this migration has hit
    repeatedly); `buffree()` is `drop(Box::from_raw(buf))`.
  - **The write path**: `buf_push_bytes` (every fixed-width `bufwriteNN`
    reduces to this) can seek backward and overwrite already-written bytes
    in place -- the offset-backpatching idiom `bufping16b`/`bufpong` build
    on -- so it isn't a plain `Vec::extend`. If the write fits inside the
    already-written region it's an in-place slice overwrite; otherwise
    `resize` grows first (zero-filling any gap between the old length and
    the cursor, matching what a fresh `realloc` over calloc'd memory used
    to leave there), then the same slice-copy runs either way.
  - **`Copy` dropped, `Clone` kept**: a `Vec`-owning struct can't be `Copy`.
    Audited first (only two real by-value `Buffer` usages existed anywhere
    in the crate): `libcff/subr.rs`'s `vec![zero_buffer; n]` scratch arrays
    (three of them) still work under `Clone` alone -- cloning an empty
    `Vec::new()` is free -- with one wrinkle: `vec![x; n]` only needs
    `Clone`, but reusing the same `x` across more than one `vec![x; n]`
    call needs an explicit `.clone()` at every use but the last, since the
    macro moves its argument on each call. `table/svg.rs`'s
    `otfcc_build_svg` used to build a transient stack-local `Buffer` that
    borrowed another buffer's bytes for one call into
    `bk_new_block_from_buffer_copy`; that trick needed a real `.clone()`
    now instead of a raw-pointer alias -- correctness-preserving and cheap
    (once per SVG assignment during build, not a hot path). A prior
    survey's assumption that `bk_new_block_from_buffer_copy` had a single
    caller (and could be re-signed to take `&[u8]` instead) turned out to
    be wrong -- `table/cmap.rs` calls it too -- so its `*const Buffer`
    signature stayed, and the transient-view approach was kept rather than
    threading a signature change through a second, unrelated caller.
  - **The four "hard" hand-rolled blob builders the plan itself flagged**
    (`libcff/cff_charset.rs`, `cff_fdselect.rs`, `cff_index.rs`'s
    `build_index`, and one site in `table/cff.rs`, plus `cff_codecs.rs`'s
    `cff_encode_cff_float`, found during this conversion, not in the
    original survey) all shared one property that made them tractable:
    every one calloc's an exact-sized buffer up front and then writes into
    it at **strictly increasing offsets**, format-header byte(s) first,
    then a homogeneous array of fixed-or-computed-width entries, in order.
    That's exactly what a plain sequence of `bufwrite8` calls already
    produces -- no bespoke `Vec` manipulation code was needed in any of
    them, just replacing "calloc the exact size, then poke every offset by
    hand" with "start from `bufnew()` and push the same bytes in the same
    order". `cff_index.rs`'s `build_index` additionally trades a raw
    `memcpy` for `bufwrite_bytes`, and `libcff/subr.rs`'s `ident_node`
    (comparing two terminal nodes' bytes for equality via `strncmp`) and
    `get_singlet_hash_key`/`get_doublet_hash_key` (hashing a terminal's
    bytes) both simplify from raw-pointer slice reconstruction to a direct
    `&Vec<u8>` borrow, now that `.data` already *is* the slice.
  - **One new deny-by-default lint surfaced by this conversion**:
    `dangerous_implicit_autorefs` -- indexing or slicing a field through an
    implicit reference materialized from a raw-pointer dereference
    (`(*buf).data[i]`, `(*buf).data[a..b].copy_from_slice(...)`) is now a
    hard error, not just discouraged style. Every site that needed
    mutable, in-place access to already-written bytes (`buf_push_bytes`
    itself, and `table/glyf/build.rs`'s RLE flag-compression `shrink_flags`,
    which reads back and increments a previously-written repeat-count byte)
    now binds an explicit `&Vec<u8>`/`&mut Vec<u8>` local first and indexes
    that, rather than indexing through the raw pointer directly.
  - **The public ABI boundary**: `ffi/dll.rs`'s `otfcc_get_buf_len`/
    `otfcc_get_buf_data` (2 of the 4 exported symbols) keep their exact
    `usize`/`*mut u8` return types -- callers across the FFI boundary see
    no change at all, only the internal read moved to `.data.len()`/
    `.data.as_mut_ptr()`. `check-abi.sh` confirms all 4 exports unchanged.
  - **Verification**: full pipeline green -- build (lib *and* both
    binaries, which surfaced one more call site `cargo build --lib` alone
    doesn't reach), 244/244 tests, clippy clean, ABI unchanged, golden
    bytes byte-identical on every payload including `KRName-Regular-O2.otf`
    (the one payload exercising `subr.rs`'s CFF subroutinizer, the most
    Buffer-intensive path in the crate) and log output unchanged, round-
    trips 10/10, lookup-alias regression clean, `cargo miri test`
    224/0/20 identical to baseline (the strongest evidence against any
    allocator-mismatch, double-free, or aliasing bug in a conversion this
    size), both fuzz targets `cargo check`-clean. `survey-unsafe.sh`: raw
    pointer types 5797→5774, `.offset(` calls 892→843, `is_null()`
    387→383, `__caryll_allocate_clean` census 89→73.
  - **Deferred, out of this PR's scope**: the ~12 files that only *read*
    `Buffer` fields (offset backpatching, hashing, magic-byte sniffing) got
    swept up automatically by the mechanical `.size`→`.data.len()`
    substitution above and needed no further design work, so nothing
    remains split out by risk tier the way an earlier survey anticipated --
    this PR turned out to cover the whole stage in one pass rather than
    the originally-planned five-way A–E split.

- **Stage 7-2-h (partial): 24 dispatch-pattern functions dropped `extern
  "C"`, unlocking `&Options` for the ~21 of them Stage 7-2-a had to leave
  on `*const Options`.** A survey of "dispatch tables" discovered during
  Stage 7-2-a found they aren't persistent static tables at all -- every
  one is a same-call pattern (`fn_ptr.expect(...)( args )`, called
  immediately after being cast, never stored beyond one statement). Nothing
  about that pattern requires C ABI linkage; it only requires *some*
  `fn(...)` pointer type. So the `"C"` keyword itself, not the
  function-pointer-value pattern, was the actual constraint 7-2-a hit.
  - **Three families, 24 functions total**: `table/otl/parse.rs`'s
    `_declare_lookup_parser` dispatch (10 lookup-subtable parsers -- the 8
    part 1 already knew about, plus `otl_gpos_parse_mark_to_ligature`/
    `_mark_to_single`, found later); `consolidate.rs`'s
    `__declare_otl_consolidation` dispatch (11 functions, including
    `consolidate_gsub_single` in `consolidate/otl/gsub_single.rs`); and
    `table/otl/subtables/chaining/read.rs`'s `CoverageReaderHandler` type
    (3 functions -- no `Options` parameter, pure ABI-keyword drop).
  - Each function's own definition lost `extern "C"`; each dispatch
    family's shared function-pointer type (`OtlConsolidationFunction` in
    `consolidate.rs`, `CoverageReaderHandler` in `chaining/read.rs`, and
    `_declare_lookup_parser`'s own inline parameter type) was updated to
    match; every cast site (`NAME as unsafe extern "C" fn(...)` →
    `unsafe fn(...)`) followed. For the ~21 with an `Options` parameter,
    `*const Options` became `&Options` at the same time, collapsing the
    `&*options` bridging expressions those call sites had carried since
    7-2-a back into plain `options` -- mechanical once the type-signature
    change landed, no logic touched.
  - **What's still deferred in Stage 7-2-h**: `repr(C)` removal for the
    ~121 non-load-bearing types (only `support/getopt.rs`'s `LongOption`
    and `vendor/sds.rs`'s `SdsHeader` are genuinely layout-dependent) is a
    separate, larger task -- removing `repr(C)` re-activates `non_camel_
    case_types` and friends for every affected type for the first time,
    and this codebase's `-D warnings` policy means any type whose name
    doesn't already conform would break the build the moment its `repr(C)`
    exemption disappears. That needs its own naming-violation inventory
    before starting, not a blind attribute removal. The ~44 other
    `extern "C" fn`s in the crate (real FFI-boundary functions, and
    callback shapes like `support/ttinstr.rs`'s `make`/`wrong` pair that a
    prior stage already partly converted) are untouched -- this batch was
    scoped to exactly the 24 same-call dispatch functions, nothing else.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged, round-
    trips 10/10, lookup-alias regression clean, `cargo miri test`
    224/0/20 identical to baseline, both fuzz targets `cargo check`-clean.
    `survey-unsafe.sh`: raw pointer types 5847→5797 (the `&Options`
    conversions); `unsafe fn` count going *up* (945→969) is a text-match
    artifact, not a regression -- the script counts the literal substring
    `"unsafe fn"`, which `unsafe extern "C" fn` never matched but plain
    `unsafe fn` does, so converting 24 functions mechanically adds 24 new
    matches for a strictly less-unsafe signature.

- **Stage 7-2-c (batch 1 of 2): 5 of the plan's 10 "outer `Box` + inner raw
  array" types converted to `Vec<T>`/`Option<Box<T>>`, their manual `impl
  Drop` deleted.** Stage 6-4 had already `Box`/`Vec`-ified the *outer*
  container for every one of the plan's 10 candidates; this stage finishes
  the job on the *inner* raw-pointer field each one still owned.
  - **`CvtTable.words`** (`table/cvt.rs`): `*mut u16` → `Vec<u16>`. The
    `length` field is gone too -- it always equaled `words.len()` at every
    construction site, so it was pure redundancy once the storage itself
    carries its own length.
  - **`LtshTable.y_pels`** (`table/ltsh.rs`) and **`VorgTable.entries`**
    (`table/vorg.rs`): `*mut u8`/`*mut VorgEntry` → `Vec<u8>`/
    `Vec<VorgEntry>`. Unlike `CvtTable.length`, `num_glyphs`/
    `num_vert_origin_y_metrics` stay as real fields -- both are read
    independently at `otf_reader/unconsolidate.rs`'s `merge_ltsh`/
    `merge_vmtx` (a `.min()` clamp bound, not just a length), so collapsing
    them into `.len()` would have been a real (if probably harmless)
    behavior risk, not just a cleanup. Extra call sites beyond the two
    owning files: `otf_writer/stat.rs`'s `stat_ltsh`/`stat_vorg` (the build-
    direction constructors) and `otf_reader/unconsolidate.rs`'s own reads.
  - **`GdefTable.{glyph_class_def,mark_attach_class_def}`** (`table/
    gdef.rs`): `*mut ClassDef` → `Option<Box<ClassDef>>` each, following an
    exact precedent already in the tree (`table/otl.rs`'s
    `ChainingRuleSet.bc`/`.ic`/`.fc`, same field shape for the same
    `ClassDef` type). Verified before converting that this is safe: `ClassDef`
    has no manual `Drop` of its own (a plain `Vec`-holding struct that
    already self-drops), and `otl_class_def_free` -- the function `GdefTable`'s
    old `Drop` used to call -- turned out to be exactly `drop(Box::from_raw(
    x))`, i.e. precisely what `Option<Box<ClassDef>>`'s own drop glue does.
    `impl Drop for GdefTable` deleted outright, not ported. A third call
    site outside `gdef.rs` needed the same treatment:
    `consolidate/otl/gdef.rs`'s `consolidate_gdef`, which shrinks a class
    def post-consolidation and frees it if it ended up empty -- the manual
    `otl_class_def_free` + null-out became a plain `= None`.
  - **`table/otl/subtables/chaining/read.rs`'s `ClassDefs`** (a private,
    transient parse-time scratch struct -- *not* the same type as
    `ChainingRuleSet` above, despite sharing a field shape and the
    `ClassDef` element type): same `Option<Box<ClassDef>>` conversion, but
    with two added wrinkles this type's siblings didn't have. First, it used
    to `#[derive(Copy, Clone)]`, incompatible with owning a `Box`; auditing
    every touch site in the file first confirmed nothing ever actually
    copied it by value (every access already went through a raw pointer),
    so dropping the derive changed no call site's shape. Second, it never
    had an `impl Drop` at all -- its raw pointers were freed by hand at a
    specific point in `read_contextual_format2`/`read_chaining_format2`
    (`otl_class_def_free` × 3 + a final `free`), now replaced by a single
    `drop(Box::from_raw(cds))` once the whole struct is genuinely
    `Box`-owned. Verified this introduces no double-free: the struct's only
    other reader, `class_coverage` (invoked as a C-callback-shaped `fn_0`
    with the struct as `void*` userdata), only ever *reads* through the raw
    pointer during the parse loop and never takes ownership, so there is
    exactly one owner to drop, at exactly the point the original code freed
    it. One behavior-adjacent change worth flagging: `class_coverage` used
    to read `bc`/`ic`/`fc` as a possibly-null `*mut ClassDef` and
    dereference it unconditionally (a latent null-deref for any `kind` whose
    field the calling parse path left unpopulated -- `read_contextual_
    format2` deliberately leaves `bc`/`fc` as `None`, only ever populating
    `ic`); this now calls `.expect()` on the `Option` instead, panicking
    loudly rather than reading through null. Confirmed this can never
    actually fire: traced every `class_coverage` call site by hand and
    `general_read_contextual_rule` (the only caller reachable from the
    `bc`/`fc`-leaves-`None` path) always passes `kind == 2` (`ic`), never
    `1`/`3` -- so the `None` branches were already dead in practice, just
    silently so. Same "UB becomes a panic, not silent" idiom used
    throughout this migration.
  - **What's deferred to a second batch, and why**: `FpgmPrepTable.bytes`
    (coupled to a C-callback-shaped `extern "C" fn` `support/ttinstr.rs`
    calls into), `Glyph.instructions` (~15 scattered touch sites across
    `glyf.rs`/`glyf/build.rs`), `SvgAssignment.document` (owns a `*mut
    Buffer`; `Buffer` itself is still the old C-shaped struct, better paired
    with the separate Stage 7-2-e `Buffer`→`Vec<u8>` work), and
    `PostTable.post_name_map` (coupled to a not-yet-decided `Font.
    glyph_order`/`GlyphOrder` representation question). `FvarTable`'s
    `Drop` (walks `FvarMaster.region: *mut VqRegion`) and `Subtable`'s
    enum-dispatch `Drop` surfaced by a fresh grep are a different shape
    entirely (not "inner raw array"), out of this stage's scope by
    definition.
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged, round-
    trips 10/10, lookup-alias regression clean, `cargo miri test` 224/0/20
    identical to baseline, both fuzz targets `cargo check`-clean.
    `survey-unsafe.sh`: raw pointer types 5924→5906, `.offset(` calls
    934→915, `is_null()` 411→394, `impl Drop for` blocks 10→6 (`Subtable`,
    `FvarTable`, `PostTable`, `FpgmPrepTable`, `SvgAssignment`, `Glyph`
    remain -- the latter 4 are exactly this stage's deferred batch 2).

- **Stage 7-2-c (batch 2 of 2): the 4 deferred types converted too --
  Stage 7-2-c is now fully done, all 10 of the plan's candidates cleared.**
  A follow-up survey found batch 1's caution about these 4 was mostly
  overcautious; only `FpgmPrepTable.bytes`/`Glyph.instructions` were
  genuinely coupled (to each other, not to anything blocking).
  - **`FpgmPrepTable.bytes`** (`table/fpgm_prep.rs`) and
    **`Glyph.instructions`** (`table/glyf.rs`): both `*mut u8` → `Vec<u8>`,
    converted together because both are populated via callbacks
    (`make_fpgm_prep_instr`/`make_instrs_for_glyph`) invoked by the same
    shared parser, `parse_ttinstr`/`parse_instrs` in `support/ttinstr.rs`.
    `parse_instrs` used to allocate an upper-bound-sized buffer, write into
    it in strictly increasing order (push semantics, never reading back),
    then do one final `realloc` shrink to the exact size -- exactly what
    `Vec::with_capacity` + `.push()` already does, no shrink step needed
    since `Vec::len()` already reflects what was pushed. The `make`
    callback now takes an owned `Vec<u8>` by value instead of `*mut u8,
    u32`, and drops its `extern "C"` linkage: nothing about a callback
    invoked only from inside `parse_ttinstr`, itself only called from two
    internal-Rust sites, needs C ABI linkage. `Glyph` kept `#[derive(Clone)]`
    through this conversion, and it's sound for the first time -- previously
    it shallow-copied the raw `instructions` pointer alongside a real
    `Drop`, a live double-free hazard if `Glyph` were ever cloned wholesale
    (no call site did, but the bug class is now categorically gone).
  - **`SvgAssignment.document`** (`table/svg.rs`): `*mut Buffer` →
    `Vec<u8>` directly, skipping `Buffer` entirely rather than waiting for
    Stage 7-2-e -- safe because every construction site writes `document`
    exactly once (never incrementally appended) and no reader ever touches
    `.cursor`/`.free`, only `.data`/`.size`. `bk_new_block_from_buffer_copy`
    (`bk/bkblock.rs`) turned out to have other callers (`table/cmap.rs`),
    contradicting the batch-1-adjacent survey's single-caller assumption --
    so its signature stayed `*const Buffer`, and `otfcc_build_svg` instead
    builds a stack-local `Buffer` that borrows `a.document`'s pointer/len
    for the one call (the function only ever reads `.size`/`.data`, never
    frees or retains the pointer, so this is a safe zero-copy view).
  - **`PostTable.post_name_map`** (`table/post.rs`): `*mut GlyphOrder` →
    `Option<Box<GlyphOrder>>`. The earlier "coupled to `Font.glyph_order`'s
    representation" caution turned out to be stale: `Font.glyph_order` is
    already `Option<Box<GlyphOrder>>`, built via `Box::new(GlyphOrder{...})`
    struct literals in `consolidate.rs`/`json_reader.rs` that completely
    bypass `otfcc_glyph_order_create`/`_free` -- so `post_name_map` never
    actually shared a constructor/destructor with it, just the element
    type. `otfcc_read_post` now builds the `GlyphOrder` the same way those
    two sites already do; `otfcc_glyph_order_create`/`_free` themselves stay
    untouched for their other callers (`unconsolidate.rs`'s local `aglfn`/
    `gord` scratch values).
  - **Verification**: full pipeline green -- build, 244/244 tests, clippy
    clean, ABI unchanged, golden bytes and log output unchanged, round-
    trips 10/10, lookup-alias regression clean, `cargo miri test` 224/0/20
    identical to baseline, both fuzz targets `cargo check`-clean.
    `survey-unsafe.sh`: raw pointer types 5906→5847, `.offset(` calls
    915→892, `is_null()` 394→387. `impl Drop for` blocks 6→2 -- only
    `FvarTable` and `Subtable` remain, neither an "inner raw array" case
    (see batch 1's note on why they were always out of this stage's scope).

- **Stage 7-2-d (easy wins): 10 OTL subtable `_create()`s converted from
  malloc/calloc shells to direct `Box` construction.** A survey of the
  plan's "7-2-d" list found the named 9 `_create()`s (really 10 -- the
  plan's list missed `gpos_mark_to_ligature`/`gpos_mark_to_single`) were
  low-risk, well-precedented wins (`otl_class_def_create` already did this
  exact conversion earlier in the migration), while the plan's other 4
  named targets (`Font`/`GlyphOrder`'s own top-level `_create()`s,
  `table/cff.rs`, `table/glyf.rs`) are genuinely coupled to larger,
  still-unconverted structures (recursive CFF FD-array ownership, `Font`'s
  crate-wide calloc-then-assign construction pattern) and stay deferred.
  - Every one of the 10 now builds its struct with a plain Rust literal and
    returns via `Box::into_raw`, instead of `malloc`/`__caryll_allocate_
    clean` + manual field writes. `subtable_from_raw` (`table/otl.rs`) --
    the shared helper every one of these feeds into -- changed from
    `ptr::read`+`libc::free` to `*Box::from_raw(raw)`, since every input now
    genuinely originates from a `Box` allocation; it kept its generic
    `*mut T` signature rather than narrowing to `Box<T>`, since some
    callers (chaining) still need a `*mut T`-shaped intermediate threaded
    through several helper functions before it reaches this adoption point.
  - **One real hazard found and avoided**: `chaining/common.rs`'s
    `subtable_chaining_create`/`_free` stayed genuinely dual-mode rather
    than fully converting, because `subtable_chaining_free` is *also* used
    by `chaining/classifier.rs`'s `try_classify_around`, which builds its
    own separate `ChainingSubtable` via a still-`__caryll_allocate_clean`'d
    allocation (out of this stage's scope) and must keep matching it with
    `libc::free`. Mixing that calloc'd pointer with `Box::from_raw` (or the
    reverse) would be a genuine allocator-mismatch hazard -- so
    `subtable_chaining_create`'s own `Box`-allocated results are instead
    reclaimed via direct `drop(Box::from_raw(x))` calls at their specific
    call sites in `chaining/read.rs`, each commented with which allocation
    origin it is and why it must not go through `subtable_chaining_free`.
  - A handful of now-redundant `_init` helpers with no other callers
    (`init_gsub_reverse`, `init_gpos_pair`, `init_mark_to_ligature`,
    `init_mark_to_single`, `subtable_chaining_init`) were removed along
    with their matching `_dispose` counterparts where applicable.
  - **Verification**: same full pipeline as above, all green (run once,
    combined with the Stage 7-2-c batch 2 commits above since both landed
    on the same branch/PR). `__caryll_allocate_clean` census: 93→89.

- **Stage 7-2-b: `Options` owns its `Logger` inline, via `RefCell`, instead
  of pointing at a second heap allocation.** `Options.logger` was `*mut
  Logger`, built by a separate `otfcc_new_logger` allocation and freed by
  a separate `logger_dispose` -- both removed; `Logger::new(target)` now
  just builds the value, and disposal is whatever `Drop` already does.
  - **The design fork this stage didn't have before Stage 7-2-a**: the
    plan wrote this stage assuming `Options` was still passed as a raw
    pointer everywhere, where getting `*mut Logger` out of a `*const
    Options` is free (raw pointers ignore aliasing rules). Once 7-2-a
    converted essentially every call site to a real `&Options`, "just own
    `Logger` by value" would have forced every logging call site back to
    needing `&mut Options` -- undoing 7-2-a's entire conversion, since
    Rust's aliasing rules don't let you get `&mut` access to one field
    while the surrounding struct is only borrowed shared. `RefCell<Logger>`
    is the standard answer to exactly this shape of problem (shared
    read-mostly context, one field needs interior mutability) in a
    single-threaded crate (`Mutex` would add locking overhead for nothing;
    `Cell` doesn't work since `Logger` isn't `Copy`). Every existing call
    site already logs with one short-lived `borrow_mut()` per statement,
    immediately released, never nested into another borrow of the same
    `Logger` -- confirmed by the full pipeline and Miri both staying green
    (a real double-borrow would panic, not silently misbehave).
  - **The calloc hazard, generalized further**: `otfcc_new_options` calloc's
    the whole `Options` struct in one shot (`__caryll_allocate_clean`), same
    as `otfcc_new_logger` used to for `Logger` alone. An all-zero `RefCell<
    Logger>` is exactly as invalid as an all-zero `Logger` was (`indents:
    Vec<Vec<u8>>` needs a dangling non-null sentinel, not a null one) --
    same `ptr::write`-not-`=` treatment applies, now at the `Options` level
    instead of the `Logger` level. The new wrinkle: `otfcc_delete_options`
    still does a raw `free()`, not `Box`-based drop (that's Stage 7-2-d's
    job, not this stage's), so raw `free()` alone would leak `Logger.
    indents`' heap buffer, exactly like `glyph_name_prefix` always needed
    explicit freeing for the same reason. Fixed the same way: `mem::replace`
    the field with a fresh, non-allocating empty `Logger` and let the
    replaced value's `Drop` run, before the raw `free()` reclaims the
    struct's own memory.
  - **`Options` lost `#[derive(Copy, Clone)]`**: `RefCell` is never `Copy`
    regardless of its contents. Audited first (grep for `: Options` and
    `*options` used as a value rather than through `&`/`*const`/`*mut`) --
    the only genuine by-value use anywhere was test code already using
    `mem::zeroed()`, itself made obsolete by this same stage: `Options`
    gained `#[derive(Default)]` instead (`Logger: Default` delegates to
    `Logger::new(LoggerTarget::Empty)`), used by 5 `mem::zeroed()`/
    `zeroed_options()`-named test helpers across `cff_parser.rs`, `subr.rs`,
    `glyf/read.rs`, `otl/read.rs`, `chaining/read.rs`. Two of those files
    (`subr.rs`, `chaining/read.rs`) had gone further and manually installed
    a real `otfcc_new_logger`-built `Logger` after zeroing, because the
    function under test logs unconditionally and a null `*mut Logger`
    would have segfaulted -- `Options::default()`'s `logger` is already a
    real (if `LoggerTarget::Empty`, i.e. no-op-push) `Logger`, so that
    workaround construction and its matching `logger_dispose` call both
    just disappeared.
  - **Mechanical ripple**: ~314 call sites across 54 files change from
    `options.logger`/`(*options).logger` (a `Copy` raw-pointer field read)
    to `&mut *options.logger.borrow_mut()`/`&mut *(*options).logger.
    borrow_mut()`. Done as one uniform, script-driven text substitution
    across the whole tree rather than by hand or by per-directory agent
    batches (unlike Stage 7-2-a) -- the pattern has no legitimate
    variation to reason about per call site. The one place a blind
    substitution went wrong: `otfcc_new_options`'s `ptr::write(&raw mut
    (*options).logger, ...)` and `otfcc_delete_options`'s `mem::replace(
    &mut (*options).logger, ...)` are *place* expressions (the destination
    being written/swapped), not value reads -- the naive regex rewrote
    both into nonsense (`&raw mut &mut *(*options).logger.borrow_mut()`,
    ill-typed and caught immediately by `cargo build`) before being hand-
    fixed back to the plain field place. A useful reminder that "every
    occurrence of this exact token sequence means the same thing" is a
    hypothesis to verify against the build, not something to trust from
    the shape of the match alone.
  - **`rust/fuzz/fuzz_targets/otf_parse.rs` needed the same treatment**
    as its two other `Options`-touching lines already got in Stage 7-2-a's
    tail. `rust/fuzz/` is a separate cargo workspace member the normal
    `cargo build`/`test`/`clippy` pipeline never compiles (see Stage
    7-2-a's entry below for how that bit there); checked proactively this
    time with `cd rust/fuzz && cargo check --bin otf_parse --bin
    json_build` before this stage's verification pipeline ran, rather than
    discovered only after a CI-only build failure.
  - **Verification**: full pipeline green -- build, 244/244 tests (no
    `RefCell` double-borrow panics anywhere in the suite), clippy clean,
    ABI unchanged, golden bytes and log output unchanged byte-for-byte
    (the strongest evidence this is a pure representation change, not a
    behavior change -- log *content* would shift immediately if borrow
    timing or drop order changed anything observable), round-trips 10/10,
    lookup-alias regression clean, `cargo miri test` 224/0/20 identical to
    baseline, both fuzz targets `cargo check`-clean. `survey-unsafe.sh`:
    raw pointer types 5935→5924, `unsafe` blocks 289→286, `is_null()`
    413→411.

- **Stage 7-2-a (part 2): `*const Options` → `&Options`, the ~153
  actually-used parameters part 1 (below) left alone.** Done as 4 batches
  by subsystem rather than one large PR, since a reference conversion can
  in principle hit borrow-checker fallout that a plain deletion can't —
  smaller batches keep each one's risk legible. All 4 landed on the same
  branch/PR to keep review to a single pass.
  - **Batch 1 — CFF** (`libcff/cff_parser.rs`, `libcff/charstring_il.rs`,
    `libcff/subr.rs`, `table/cff.rs`, 12 functions incl.
    `cff_parse_outline`, the ~1,900-line Type2 charstring interpreter —
    the riskiest function by sheer size even though the change itself is
    mechanical). Fully mechanical: no borrow-checker conflicts, no
    dispatch-table constraints in these 4 files.
    - **One deliberate exception, structural not accidental**:
      `CffCharstringBuilderContext.options` (`table/cff.rs`) is a *struct
      field*, not a function parameter, so it stays `*const Options` —
      the struct is a stack-local scratch context threaded through one
      call chain, and re-deriving the raw pointer from the incoming
      `&Options` at that one field assignment is simpler than threading
      a lifetime parameter through the whole struct.
    - **One test fix, not a behavior change**: `cff_parser.rs`'s
      truncated-header test used to pass a null `*const Options` down a
      path that provably never dereferences it. A real `&Options` can't
      be null, so it now uses a `mem::zeroed()` `Options` local — sound
      because `Options` (`support/options.rs`) is `Copy`/`Clone` over
      only `bool` and raw-pointer fields (no `Vec`/`Box`/`HashMap`), so
      the all-zero bit pattern is a valid value, unlike the
      `otfcc-vec-field-assign-needs-calloc`-shaped traps found earlier.
  - **Batch 2 — OTL** (`table/otl/{build,dump,parse,read}.rs`,
    `table/otl/subtables/{chaining/read,extend,gpos_mark_to_ligature,
    gpos_mark_to_single}.rs`, `consolidate/otl/*.rs`, 33 functions).
    Surfaced two `extern "C" fn`-typed dispatch tables where a function's
    `Options` parameter is load-bearing because its *address* is taken
    and stored as a value, not just called directly:
    - `table/otl/parse.rs`'s `_declare_lookup_parser` table already had 8
      known members (part 1's unused-parameter pass left them alone for
      the same reason). Two more turned out to be in that table with a
      genuinely *used* `Options` parameter — `otl_gpos_parse_mark_to_
      ligature`/`_mark_to_single` — so instead of leaving the whole
      function alone, the real work moved into a private `parse_bases(
      ..., &Options)` helper, called as `parse_bases(..., &*options)`
      from the still-`*const Options` dispatch entry point.
    - `consolidate.rs` has a second, independent dispatch table
      (`extern "C" fn(*mut Font, *mut OtlTable, *mut Subtable, *const
      Options) -> bool`) constraining 10 more functions the same way
      (`consolidate_gpos_cursive`/`_single`/`_pair`, `consolidate_gsub_
      ligature`/`_reverse`/`_multi`/`_alternative`, `consolidate_
      chaining`, `consolidate_mark_to_single`/`_to_ligature`) — each
      keeps `*const Options` and bridges `&*options` into the
      newly-`&Options` helpers it calls.
  - **Batch 3 — other `table/*.rs`** (every remaining `table/*.rs` file:
    `_tsi`, `base`, `cmap`, `colr`, `cpal`, `cvt`, `fpgm_prep`, `fvar`,
    `gasp`, `gdef`, `glyf`/`glyf/read`, `head`, `hhea`, `hmtx`, `ltsh`,
    `maxp`, `meta/{dump,parse,read}`, `name`, `os_2`, `post`, `svg`,
    `vdmx/funcs`, `vhea`, `vmtx`, `vorg` — 68 functions, the largest
    batch). No dispatch-table constraints found; fully mechanical.
    `fpgm_prep.rs`/`glyf.rs` each kept one `options as *const Options`
    pass-through into `support/ttinstr.rs`'s not-yet-converted
    `dump_ttinstr`, cleaned up in batch 4.
  - **Batch 4 — top-level orchestration** (`consolidate.rs`'s
    non-dispatch-table functions, `font/caryll_sfnt_builder.rs`,
    `json_reader.rs`/`json_writer.rs`, `otf_reader.rs`/`otf_reader/
    unconsolidate.rs`, `otf_writer.rs`/`otf_writer/stat.rs`,
    `support/ttinstr.rs` — 33 functions). This is where the outer
    `read_otf`/`read_json`/`serialize_to_json`/`serialize_to_otf`
    functions and their `FontBuilder`/`FontSerializer` trait-impl bodies
    finally moved off `*const Options`, which let ~70 accumulated
    `&*options` bridging expressions (added by batches 1–3 for callers
    that hadn't converted yet) collapse back into plain `options`.
    - `FontBuilder`/`FontSerializer` themselves stayed untouched: both
      traits are already erased to `*const c_void` at the boundary (for
      dynamic dispatch reasons predating this change), so each impl body
      does one `&*(options as *const Options)` reconstruction up front
      and uses a real `&Options` from there on — no trait signature or
      cross-implementor change needed.
    - `SfntBuilder.options` (`font/caryll_sfnt_builder.rs`) is a struct
      field, same reasoning as `CffCharstringBuilderContext.options` in
      batch 1 — left as `*const Options`.
    - The 3 legitimate ownership/construction sites this whole part was
      never going to touch — `support/options.rs`'s `otfcc_new_options`/
      `otfcc_delete_options`/`otfcc_options_optimize_to`, and the `*mut
      Options` locals in `bin/otfccbuild.rs`, `bin/otfccdump.rs`,
      `ffi/dll.rs` — got their call sites into now-`&Options` functions
      adjusted to `&*options`, nothing else.
  - **What's left `*const Options` after all 4 batches, confirmed by
    grep**: exactly the 21 dispatch-table-constrained function
    declarations found across batches 2 and 4 (10 lookup-subtable
    parsers + 11 `consolidate.rs` dispatch functions — one more than
    the original plan's estimate of 20, since `consolidate_gsub_single`
    turned out to be address-taken the same way), the 2 struct fields
    (`table/cff.rs`, `font/caryll_sfnt_builder.rs`), and the
    trait-boundary `*const c_void` bridging casts in batch 4's 4 files.
    Everything else in `rust/src/` that ever took a used `Options`
    parameter now takes `&Options`.
  - **Verification** (run once at the end, after all 4 batches):
    full pipeline green — build, 244/244 tests, clippy clean, ABI
    unchanged, golden bytes and log output unchanged (including
    `KRName-Regular-O2.otf`'s subroutine-count line), round-trips 10/10,
    `cargo miri test` 224/0/20, identical to baseline throughout. `survey-
    unsafe.sh`: raw pointer types 6088 (start of part 2, i.e. after part
    1's deletions below) → 5935.

- **Stage 7-2-a (part 1 of 2): removed 51 dead `_options: *const Options`
  parameters.** The plan flagged ~62 of these as pure C-signature inertia
  (never read, never dereferenced) across `rust/src/`; a fresh survey found
  59 actually left in the tree (some were incidentally cleaned up by earlier
  Stage 7-1 work). 51 were dropped from their function signatures, with
  every call site (including cross-file callers and `#[cfg(test)]` call
  sites) updated to match; a couple of now-dead `zeroed_options()` test
  helpers and unused `use ... Options;` imports went with them.
  - **8 left alone, deliberately**: every `otl_g{sub,pos}_parse_*`/
    `otl_parse_chaining` function whose `_options` parameter is unused in
    the body is *also* cast to `unsafe extern "C" fn(*const ParsedValue,
    *const Options) -> *mut Subtable` and stored as a function-pointer
    value in `table/otl/parse.rs`'s `_declare_lookup_parser` dispatch
    table. Dropping the parameter there would change the function's type
    and break the pointer-table match even though the value is never
    used at the call site — a case where "unused" isn't the same question
    as "safe to delete", because the signature itself is load-bearing.
    These 8 stay as `*const Options` for now; they'll need to be
    revisited together with that dispatch table, likely in Stage 7-2-h's
    `extern "C"` residue cleanup rather than here.
  - **Scope boundary**: this pass only *deletes* unused parameters. It does
    not convert any of the remaining, actually-used `*const Options`
    parameters to `&Options` — that conversion (part 2 of Stage 7-2-a) is
    a separate PR, since "delete a parameter no one reads" and "change a
    parameter's type and add borrow-checker constraints at every call
    site" are different kinds of risk and shouldn't be reviewed together.
  - **Verification**: full pipeline green (build, 244/244 tests, clippy
    `-D warnings` clean, ABI unchanged at 4 exports, golden bytes and log
    output unchanged, round-trips 10/10, `cargo miri test` 224 passed / 0
    failed / 20 ignored — identical to the pre-PR baseline). `survey-
    unsafe.sh`: raw pointer types 6163→6088, `unsafe` blocks 298→289,
    everything else unchanged (deleting a parameter doesn't change a
    function's own `unsafe fn`-ness).

- **Stage 7-2-g: `libcff/subr.rs`'s intrusive doubly-linked-list CFF
  subroutinizer, converted to an arena + index.** The plan's own single
  hardest-flagged piece in the whole migration. `CffSubrNode`/
  `CffSubrRule`'s `*mut`-typed `prev`/`next`/`rule`/`guard` fields are
  gone, replaced by `NodeId`/`RuleId` (bare `usize` newtypes) indexing
  into `CffSubrGraph.nodes: Vec<CffSubrNode>`/`.rules: Vec<CffSubrRule>`.
  - **Why an index and not just "a safe pointer"**: this graph's core
    operation is deleting a node mid-algorithm (`expand_call`,
    `remove_node_from_graph`) while other structures -- sibling nodes'
    own `prev`/`next`, and `diagram_index`'s weak `start` references --
    can still describe where it used to be. A naive arena that *recycles*
    a freed slot would let a stale reference silently start resolving to
    a totally unrelated later node the instant that slot gets reused --
    worse than the raw-pointer use-after-free it replaces, since a
    dangling pointer tends to crash and a reused index just corrupts the
    graph quietly. `CffSubrGraph::delete_node` never reuses a slot:
    deleted nodes are tombstoned (a `dead: bool` flag) and left in place
    for the rest of the graph's lifetime -- exactly one CFF table's
    subroutinize build pass, not something long-lived, so the wasted
    space is bounded and cheap. Rules never need this at all: nothing
    ever removes one mid-algorithm (only `dispose` tears them all down at
    once at the very end), so plain, always-valid indices sufficed there
    from the start.
  - **Verification strategy, given the algorithm's own subtlety**: rather
    than trust a mechanical pointer-to-index transliteration, every
    single function was diffed line-by-line against the pre-conversion
    original (kept on hand via `git show`) after the initial rewrite.
    This caught two real transcription mistakes before they ever reached
    a test run:
    - `process_match_doublet`'s "is this rule already fully surrounded by
      its own guard" check was accidentally written against `n`'s
      neighbors instead of `m`'s -- the original checks `(*m).prev` and
      `(*(*m).next).next`, not `n`'s equivalents. Silent logic bug, not a
      type error, so the compiler had no way to catch it.
    - `cff_insert_il_to_graph` gained an unplanned `buffree()` for the
      case where the accumulated blob is still empty after the main loop
      (reachable only for a charstring IL with zero instructions). The
      *original* leaks that one small `Buffer` in this case -- a real,
      pre-existing, low-severity bug, but fixing it wasn't this PR's
      purpose, and silently doing so as a side effect of an unrelated
      structural conversion would have conflated two different kinds of
      change in one diff. Reverted to match the leak exactly; noted here
      instead as a discovered-but-deliberately-untouched issue, the same
      posture taken elsewhere in this migration (e.g. `parse_point_
      numbers`'s missing byte-swap in `table/glyf/read.rs`).
  - **The strongest evidence this is correct**: `KRName-Regular-O2.otf`
    (the CFF subroutinize build, the *only* payload that exercises this
    algorithm at all) stays byte-identical on golden, including the
    `[libcff] Total N subroutines extracted.` progress log line
    (`compare-log-output.sh` checks stderr byte-for-byte too, so the
    exact subroutine count -- not just the final bytes -- matches).
  - **Miri coverage is partial, same limitation the prerequisite PR
    already noted**: `cff_merge_cs2_operand` calls `libc::modf`,
    unsupported on macOS under Miri, so 4 of the 5 existing tests (any
    that encode real glyph content) stay Miri-ignored; only the
    empty-graph test runs there, confirming the `CffSubrGraph` init/
    dispose lifecycle itself (allocation and tombstoning included) is
    clean. The rest of the correctness case rests on the line-by-line
    diff and the golden `-O2` match above, not Miri.
  - `table/cff.rs`'s one construction site for `CffSubrGraph` -- previously
    a raw struct literal naming `*mut CffSubrRule` directly for `root`/
    `last` -- now calls `CffSubrGraph::default()` instead, since
    `NodeId`/`RuleId` are private to `subr.rs` and callers outside it were
    never meant to need to name them.
  - `rust/scripts/survey-unsafe.sh` deltas: raw pointer types 6261 → 6163
    (-98), `is_null()` calls 440 → 413 (-27), `as ::core::ffi::c_int`
    5276 → 5266 (-10).
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 244 tests (unchanged -- same 5 `subr_graph_tests` from the
    prerequisite PR, now exercising the new implementation), clippy
    clean, ABI export guard, all golden fixtures byte-identical (dump/
    build and log output, zero exceptions, `KRName-Regular-O2.otf`
    specifically confirmed), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes, `cargo miri test` clean locally
    before pushing (224 passed, 0 failed, 20 pre-existing ignores).
  - This closes Stage 7-2-g. What's left in `libcff/`'s C-isms (the
    `#[repr(C)]`/`extern "C"` residue on `CffSubrRule` and the handful of
    C-shaped signatures this file's callbacks still use) belongs to
    Stage 7-2-h, not this PR.

- **Stage 7-2-g prerequisite: `libcff/subr.rs` unit tests, no production
  code changed.** The plan flags `subr.rs`'s intrusive doubly-linked-list
  CFF subroutinizer (`CffSubrNode`/`CffSubrRule`/`CffSubrGraph`, headed
  for an arena + index conversion) as the single hardest remaining piece,
  and says explicitly to add fuzz/unit test coverage *before* starting --
  the `-O2` subroutinize build path was previously exercised by exactly
  one payload (`KRName-Regular-O2.otf`'s golden checksum), end-to-end
  only, with no isolated coverage of the graph algorithm itself. This PR
  is that prerequisite: 5 new tests against the two entry points
  `table/cff.rs` actually calls (`cff_insert_il_to_graph`,
  `cff_il_graph_to_buffers`), independent of how the graph ends up
  represented internally, so they keep pinning behavior across the
  upcoming conversion rather than needing to be rewritten alongside it.
  - **Coverage**: an empty graph produces empty indexes; one glyph with
    subroutinize off produces exactly one char string and no subroutines;
    two glyphs with an identical repeated `[rmoveto, hlineto]` doublet
    and subroutinize on get it extracted into one shared subroutine
    (confirmed both by a direct subroutine-count assertion and by the
    char-strings index shrinking relative to the unsubroutinized case);
    two glyphs with *different* content extract no subroutine (no false
    positives).
  - **One real bug found, in the test harness, not in `subr.rs`
    itself**: the first version of this suite segfaulted on every test,
    including the trivial empty-graph one. Root cause was a mismatch
    between two already-known hazards from earlier in this migration,
    stacked on each other in a new test file: `cff_il_graph_to_buffers`
    logs an unconditional progress message, so `Options.logger` must be a
    real logger, not the null one `mem::zeroed()` gives -- exactly the
    hazard `chaining/read.rs`'s own "unsupported format" test already
    had to work around, just missed here on the first pass. A second,
    separate near-miss caught before it became a Miri failure: the test
    harness's own `CffSubrGraph` construction initially used
    `mem::zeroed()` too, which is invalid for its `HashMap` field --
    `otfcc-vec-field-assign-needs-calloc`'s exact hazard, this time on a
    `HashMap` rather than a `Vec`. Fixed by building the struct as a
    proper Rust literal (`diagram_index: HashMap::new()` set directly),
    matching how `table/cff.rs` itself already constructs this same
    struct.
  - Miri coverage is partial: `cff_merge_cs2_operand` calls `libc::modf`
    (an unsupported foreign function on macOS under Miri, the same
    category as `strtod`/`snprintf` already excluded elsewhere in this
    crate), so the 4 tests that encode any glyph content are `#[cfg_attr(
    miri, ignore)]`'d; only the empty-graph test runs under Miri today.
    Still real coverage: it exercises the full init/dispose lifecycle of
    `CffSubrGraph` (including the `HashMap` construction this PR's own
    near-miss was about) cleanly.
  - No behavior change: this PR touches no production code at all, only
    adds a `#[cfg(test)]` module.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 244 tests (was 239 -- 5 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (unaffected, as expected
    for a test-only change), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes, `cargo miri test` clean locally
    before pushing (224 passed, 0 failed, 20 ignored -- 16 pre-existing
    plus this PR's 4 `modf` ignores).
  - Next: the actual arena + index conversion of `CffSubrNode`/
    `CffSubrRule`/`CffSubrGraph`'s intrusive linked lists, now that this
    safety net is in place.

- **Stage 7-1 follow-up: `libcff/cff_parser.rs`'s `callsubr`/`callgsubr`
  subroutine index bounds -- the other concrete gap flagged when
  `cff_parse_outline` was first scoped out.** The subroutine index these
  two CharString operators use is an entirely attacker-controlled Type2
  operand (a stack value, popped and cast to `u32`). The original indexed
  `gsubr`/`lsubr`'s own offset array with it via raw, unchecked
  `.offset()` arithmetic, then used the (possibly garbage) result to
  derive a *pointer and length* for a *recursive* `cff_parse_outline`
  call -- a malformed subroutine index could recurse into arbitrary
  memory. Fixed with one new helper, `locate_subr`, shared by both
  operators.
  - **`locate_subr` re-validates more than just "in bounds"**:
    `extract_index` (`libcff/cff_index.rs`, an earlier PR in this same
    push) only validates an INDEX's *last* offset entry, to close the
    4GB-`memcpy` wraparound bug -- intermediate entries were never
    checked at all, and can still be zero or non-monotonic. So beyond
    `offset.get(idx)`/`offset.get(idx+1)` succeeding, `locate_subr` also
    checks both entries are `>= 1` (the CFF INDEX spec's own 1-based
    offset requirement), that the pair is non-decreasing, and that the
    resulting byte range actually fits within the INDEX's own `data`.
    Each of the three is independently pinned by its own test
    (`a_zero_intermediate_offset_is_rejected`,
    `a_non_monotonic_offset_pair_is_rejected`,
    `a_range_past_the_actual_data_length_is_rejected_instead_of_reading_
    oob`).
  - **Deliberately not touched**: unbounded recursion depth (a
    charstring's subroutines calling each other, even validly within
    these new bounds) is a stack-overflow DoS, not a memory-safety bug,
    and it's already a known, pre-existing, documented issue in this
    exact interpreter -- `Cormorant-Medium.otf`/`WorkSans-Regular.otf`
    are excluded from the fuzz corpus specifically because they trigger
    it, and it reproduces in the original C toolchain too. Out of this
    PR's scope; unrelated to the bug fixed here.
  - **No behavior change on any committed payload**: `KRName-Regular-
    O2.otf` (the CFF subroutinize variant, which exercises `callsubr`/
    `callgsubr` far more heavily than the plain payload) stays
    byte-identical on build, dump, and round-trip. 6 new unit tests on
    `locate_subr` directly.
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 942 → 934 (-8),
    `as ::core::ffi::c_int` 5278 → 5276 (-2).
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 239 tests (was 233 -- 6 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions, CFF payloads specifically confirmed), all
    round-trip payloads stable, issue #1's lookup-alias regression still
    passes, `cargo miri test` clean locally before pushing (223 passed, 0
    failed, 16 pre-existing ignores).
  - Between this PR and the previous one, both concrete gaps flagged when
    `cff_parse_outline` was scoped out of Stage 7-1 proper are now
    closed. What's left in this area -- `libcff/subr.rs`'s intrusive
    linked list (the CFF subroutinize *build* path, Stage 7-2-g's own
    hardest, separately-scoped piece) and the pre-existing unbounded-
    recursion DoS noted above -- are both a different shape of task than
    "add bounds checking to a raw-pointer reader."

- **Stage 7-1 follow-up: `libcff/cff_codecs.rs`'s Type2/DICT token
  decoders -- the boundary gap the previous PR deliberately left open
  in `cff_parse_outline`.** That PR noted `cff_parse_outline` already
  bounds its own top-level token loop against a real `len`, unlike every
  other reader in this stage, but that only checks *before* decoding a
  token, not that the token *itself* stays within bounds -- the actual
  per-token byte reads live one level down, in `cff_decode_cs2_token`
  (CharStrings) and `cff_decode_cff_token`/`cff_dec_i`/`cff_dec_r`/
  `cff_dec_o`/`cff_dec_e` (DICTs, shared by `cff_dict.rs`'s
  `parse_to_callback` -- the DICT-key lookup every `cff_parser.rs` Top
  DICT read goes through). All of these read up to 5 bytes unconditionally
  based only on the first byte's value, with zero awareness of how many
  bytes actually remained. This turned out to be a well-defined, tractable
  fix rather than the open-ended gap it looked like -- not the
  `cff_parse_outline`/`subr.rs` mega-effort originally floated as the
  alternative next step.
  - **`cff_dec_r`'s nibble scan (DICT real numbers) had no bound
    whatsoever** -- not "checked against the wrong thing," genuinely
    unbounded: it scanned forward one nibble at a time looking for a
    `0xF` terminator with no length parameter anywhere in the function.
    A malformed DICT real number that never has one read arbitrarily far
    past the buffer -- the least-guarded read found anywhere in this
    whole migration. It also built the decoded digit string with `strcat`
    into a fixed 72-byte stack buffer with no check that the (also
    attacker-controlled) nibble count fit, a second, independent bug (a
    stack buffer overflow, not just an overread) hiding behind the first.
    Rewritten to scan through a bounds-checked slice and build the text
    into a growable `Vec<u8>` instead of the fixed buffer; `atof`/
    `strtod` still does the actual parsing, unchanged, so number-format
    fidelity is untouched (confirmed by golden: `KRName-Regular-O2.otf`,
    the one payload whose CFF subroutinize path exercises DICT parsing
    most heavily, stays byte-identical).
  - **Every other decoder had the ordinary "reads N bytes based on the
    first byte, no check that N are actually available" gap**: `cff_dec_i`
    (DICT integers, 1-5 bytes), `cff_dec_o` (DICT operators, 1-2 bytes),
    `cff_decode_cs2_token` (CharString tokens, 1-5 bytes depending on
    format). All six decoders now take `remaining: usize` and return
    `Option<u32>` instead of an unconditional `u32`; the `DE_T2` dispatch
    table (256 entries, one per possible first byte) and its two callers
    (`cff_parse_outline`'s CharString loop, `parse_to_callback`'s DICT
    loop) updated to match -- both now compute `remaining` via
    `.offset_from()` each iteration and `break` cleanly on `None` instead
    of reading on.
  - **No behavior change on any committed payload**: the CFF payloads
    (`KRName-Regular.otf` and its `-O2` subroutinize variant, which
    exercises both the CharString interpreter and DICT parsing far more
    heavily than the plain payload) stay byte-identical on build, dump,
    and round-trip. 8 new unit tests (1 ignored under Miri --
    `libc::strtod` is unsupported there on macOS, the same category as
    the pre-existing `printf`/`snprintf` ignores in `vendor/sds.rs`).
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 961 → 942 (-19),
    `as ::core::ffi::c_int` 5443 → 5278 (-165, mostly the 256-entry
    dispatch table's casts), `Option<` usage 279 → 543 (+264, the same
    table now returning `Option<u32>`).
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 233 tests (was 225 -- 8 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions, CFF payloads specifically confirmed), all
    round-trip payloads stable, issue #1's lookup-alias regression still
    passes, `cargo miri test` clean locally before pushing (217 passed, 0
    failed, 16 ignored -- 15 pre-existing plus this PR's one `strtod`
    ignore).
  - With this PR, the boundary gap the previous PR deliberately left open
    is closed. What remains genuinely out of Stage 7-1's scope in this
    area is `libcff/subr.rs`'s intrusive linked list (the CFF
    subroutinize *build* path, already flagged in the plan as Stage
    7-2-g's own hardest, separately-scoped piece) and a full line-by-line
    audit of `cff_parse_outline`'s ~1,900 lines of stack-machine
    interpretation (subroutine call depth/indexing into `gsubr`/`lsubr`,
    etc.) -- both real future work, but a different shape of task than
    "add bounds checking to a raw-pointer reader," which is what Stage
    7-1 set out to do.

- **Stage 7-1: `libcff/cff_parser.rs`, part 1 -- the CFF header and
  Encoding table, third piece of `libcff/`'s remaining scope.** Scoped
  deliberately narrowly: `gu1`/`gu2`, `parse_encoding` (the Encoding table
  reader -- same shape as the previous PR's `cff_extract_charset`/
  `cff_extract_fd_select`), and the 4 fixed header-byte reads at the top
  of `parse_cff_bytecode`. **Not** `cff_parse_outline` (~1,940 of this
  file's 2,490 lines) -- the CFF Type2 charstring bytecode interpreter,
  which already takes a real `len: u32` and bounds its own top-level token
  loop against it (`while start < data.offset(len as isize)`), unlike
  every other reader converted in this stage. What bounds-safety work it
  still needs (individual token decoding near the boundary, subroutine
  call depth/indexing) is a different shape of problem than "no length
  parameter at all," closer in character to `libcff/subr.rs`'s intrusive
  linked list (already called out in the plan as its own hardest,
  separately-scoped piece) than to this stage's other targets -- left as
  a deliberate, explicit gap rather than folded in here.
  - **The 4 header-byte reads had no length check at all**: a `raw_length`
    shorter than 4 read past the allocation `cff_open_stream` copied the
    font's CFF table into. Each field now defaults to 0 on a bounds
    failure instead of bailing out of the function entirely --
    `extract_index` below is already bounds-checked regardless of what
    `pos` (derived from `head.hdr_size`) it ends up given, so a garbage
    header degrades the same way a garbage DICT offset already does,
    rather than needing a new early-return path.
  - **`parse_encoding` had the same shape of bugs `cff_extract_charset`/
    `cff_extract_fd_select` did**: a negative `offset` (reachable from a
    malformed DICT key, only checked against `-1` at the call site) moved
    the read pointer before the buffer, and every one of the three
    formats' arrays was completely unguarded. All now go through one
    sequential `FontReader` -- all three formats lay their count field and
    array immediately after the format byte with no gaps, so a single
    reader walking forward covers the whole record, same as
    `cff_extract_fd_select`'s equivalent conversion.
  - **No behavior change on any committed payload**: the CFF payloads
    stay byte-identical. 4 new unit tests (a minimal `CffFile` built
    directly in safe Rust, not `__caryll_allocate_clean`, to avoid needing
    every field to be a valid calloc'd bit pattern).
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 973 → 961 (-12),
    `as ::core::ffi::c_int` 5447 → 5443 (-4), `while` loops 689 → 686 (-3).
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 225 tests (was 221 -- 4 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions, CFF payloads specifically confirmed), all
    round-trip payloads stable, issue #1's lookup-alias regression still
    passes, `cargo miri test` clean locally before pushing (210 passed, 0
    failed, 15 pre-existing ignores).
  - Next: `cff_parse_outline` remains explicitly out of scope for Stage
    7-1 (see above) -- with this PR, Stage 7-1's originally-scoped work
    (parse-boundary safety across `otl/`, `glyf`/`gvar`, and the
    `libcff/` header/INDEX/Encoding/Charset/FdSelect readers) is
    complete. Revisit `cff_parse_outline`/`charstring_il.rs`/`subr.rs`
    together as their own effort before moving to Stage 7-2.

- **Stage 7-1: `libcff/cff_charset.rs`/`cff_fdselect.rs` -- second piece
  of `libcff/`'s remaining scope.** Same shape as `cff_index.rs`'s own
  conversion: each file's `gu1`/`gu2` and its one `cff_extract_*` reader
  (`cff_extract_charset`, `cff_extract_fd_select`) converted, plus
  threading `CffFile.raw_length` through their two call sites in
  `cff_parser.rs` -- not a conversion of `cff_parser.rs` itself, still the
  larger remaining follow-up.
  - **A negative `offset` moved the read pointer before the buffer in
    both functions.** `offset: i32` comes from a DICT key lookup that
    only checks it against `-1` ("not present") at the call site --
    any other negative value reached `data.offset(offset as isize)`
    directly, walking backward off the start of the allocation. Both
    functions now check `offset < 0` up front and fall back to the same
    value they already used for an unrecognized format byte
    (`IsoAdobe`/`Unspecified`) -- the original drew no distinction
    between "malformed" and "recognized fallback" to begin with.
  - **`cff_extract_charset`'s Format0 had the same wraparound-to-huge-
    allocation shape as `cff_index.rs`'s 4GB `memcpy` bug**: `count =
    nchars as c_int - 1` for `nchars == 0` went negative in `c_int`
    arithmetic and was cast straight to `u32`, producing `0xFFFFFFFF` and
    an immediate `Vec::with_capacity` abort. `.saturating_sub(1)` closes
    it. Pinned by `format0_nchars_zero_does_not_attempt_a_huge_allocation`.
  - **Every array read in both functions was otherwise completely
    unguarded**: Format0's glyph/fd arrays, Format1/Format2's two-pass
    range-counting walk (the same "read a count from every entry as you
    scan forward, no bound on how far" shape used in `otl/subtables/
    chaining/read.rs`'s ClassSet arrays), and Format3's `nranges`-driven
    range array plus its trailing sentinel. All now go through
    `FontReader`; `cff_extract_fd_select` in particular collapses to one
    sequential reader for the whole function, since format0's array and
    format3's range array + sentinel are laid out with no gaps.
  - **No behavior change on any committed payload**: the same CFF
    payloads (`KRName-Regular.otf` and its `-O2` variant) stay
    byte-identical. 11 new unit tests split across the two files.
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 982 → 973 (-9),
    `as ::core::ffi::c_int` 5457 → 5447 (-10), `while` loops 694 → 689
    (-5).
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 221 tests (was 210 -- 11 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions, CFF payloads specifically confirmed), all
    round-trip payloads stable, issue #1's lookup-alias regression still
    passes, `cargo miri test` clean locally before pushing (206 passed, 0
    failed, 15 pre-existing ignores).
  - Next: `cff_parser.rs` itself -- the last and largest remaining piece
    of Stage 7-1's `libcff/` scope, and of Stage 7-1 as a whole.

- **Stage 7-1: `libcff/cff_index.rs` -- the CFF INDEX reader, first piece
  of the last remaining Stage 7-1 scope.** The plan names this file's
  4GB-`memcpy` bug specifically; this PR is scoped to `cff_index.rs`
  itself (`gu1`-`gu4`, `extract_index`) plus the minimal, mechanical
  call-site threading needed to give it a real length -- not a full
  conversion of `cff_parser.rs` (2,479 lines, its own much larger
  follow-up) or the other three files that duplicate `gu1`/`gu2`
  (`cff_charset.rs`, `cff_fdselect.rs`, also follow-ups).
  - **The headline bug, fixed exactly as the plan describes it**:
    `extract_index` computed the INDEX's data-block length as
    `offset[count].wrapping_sub(1)`. A malformed INDEX whose last offset
    entry is 0 (invalid per spec -- offsets are 1-based and
    non-decreasing, so a well-formed INDEX's last offset is always >= 1)
    wrapped that subtraction to `0xFFFFFFFF`, and the `memcpy` that
    followed copied up to ~4GB from wherever `data` happened to point.
    `FontReader`'s bounds checking closes this by construction -- a
    `data_len` that large can never fit in a real table, so the read
    simply fails -- without needing a separate special case for the
    wraparound itself; a `checked_sub` still guards the subtraction
    directly, since a `data_len` of exactly 0 wrapping to `usize::MAX` on
    a 32-bit target would otherwise pass a 64-bit length check it
    shouldn't. Pinned by `last_offset_of_zero_is_rejected_instead_of_a_
    4gb_memcpy`.
  - **The rest of `extract_index` was equally unguarded**: `count`/
    `off_size` and the whole `offset[]` array were read off a bare
    `*mut u8` with no length parameter anywhere in the call chain,
    and an out-of-range `off_size` (not 1-4) silently produced an
    all-zero offset array -- just another path to the same wraparound
    bug above. All now guarded via `FontReader`, with dedicated tests for
    a truncated offset array, a data block longer than the table, and
    the invalid-`off_size` case.
  - **Threading a real length required minimal, mechanical touches
    outside this file**: `CffFile.raw_length` (already tracked alongside
    `raw_data`, just never passed to `extract_index`) is now passed at
    all 8 call sites in `cff_parser.rs`, and one of them
    (`cff_parse_subr`, called from `table/cff.rs`) needed a new
    `raw_length: u32` parameter threaded through its own two call sites
    to reach it. Nothing else in `cff_parser.rs` or `table/cff.rs`
    changed -- their own internal unguarded reads (including
    `cff_parser.rs`'s own separate `gu1`/`gu2` copies) are out of this
    PR's scope.
  - **No behavior change on any committed payload**: `KRName-Regular.otf`
    and its `-O2` (CFF subroutinize) variant -- the only CFF-bearing
    payloads -- stay byte-identical on both build and dump. 6 new unit
    tests cover the fixed bugs plus well-formed empty and non-empty INDEX
    reads.
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 1001 → 982 (-19),
    `as ::core::ffi::c_int` 5477 → 5457 (-20).
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 210 tests (was 204 -- 6 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions, CFF payloads specifically confirmed), all
    round-trip payloads stable, issue #1's lookup-alias regression still
    passes, `cargo miri test` clean locally before pushing (195 passed, 0
    failed, 15 pre-existing ignores).
  - Next: `cff_charset.rs`/`cff_fdselect.rs` (their own smaller `gu1`/
    `gu2` copies), then `cff_parser.rs` itself (the large remaining
    piece) -- the rest of Stage 7-1's `libcff/` scope.

- **Stage 7-1: `table/glyf/read.rs`, part 2 -- the gvar tuple-variation
  applicator** (`polymorphize`, `polymorphize_glyph`, `next_tvh` ->
  `next_tvh_offset`, `parse_point_numbers`, `read_packed_delta`,
  `create_region_from_tuples`). Follow-up to the previous entry's glyf
  outline readers, now converting the other half of this file: a
  self-describing binary format with no length of its own anywhere in the
  chain -- a `TupleVariationHeader` array's own end (and so where the next
  one starts) is only known after reading *that* header's own flags, and a
  `GlyphVariationData`'s point/delta streams are only bounded by their own
  declared `variationDataSize` accumulating across the tuple loop. Every
  one of these used to be walked with bare `*mut T`/`FontFilePointer`
  pointer arithmetic and zero length awareness.
  - **The core move: every raw pointer through this whole call chain
    becomes an absolute `usize` byte offset into one `gvar: &[u8]` slice**
    (the whole `gvar` table, built once in `polymorphize` from the
    `PacketPiece`'s own `Vec<u8>`/length and threaded down unchanged --
    same discipline as `data`/`table_length` pairs everywhere else in this
    migration), with every read going through `FontReader::new(gvar).
    at(offset)` instead of `.offset()`. This eliminated the need for the
    `#[repr(C, packed)]` `GVARHeader`/`TupleVariationHeader`/
    `GlyphVariationData` structs entirely (nothing else in the crate used
    them -- confirmed by grep before deleting), along with the `be16`/
    `be32` manual byte-swaps their native-endian pointer casts needed
    (`FontReader`'s reads are big-endian by construction). `next_tvh` ->
    `next_tvh_offset` returns the next tuple header's offset instead of a
    pointer, and `TuplePolymorphizerCtx.shared_tuples: *mut F2Dot14`
    becomes `shared_tuples_offset: usize` into the same slice.
  - **Two real, previously-undocumented "zero guard" bugs in
    `polymorphize`'s own top level**, the same class
    `read_contextual_format2`'s `ChainSubClassSet` array had
    (`otl/subtables/chaining/read.rs`, previous stage): the per-glyph
    `glyphVariationDataOffsets` array read `data.offset(sizeof(GVARHeader)
    + j*stride)` for every glyph with *no* check that the table was long
    enough to actually hold that many entries, and the
    `glyphVariationDataArrayOffset + glyphVariationDataOffset` sum that
    follows was never checked against the table's own length either. Both
    now go through `FontReader`; a glyph whose entry doesn't fit is
    skipped (no variation applied to it) rather than reading past the
    table.
  - **Two more of the same class inside `polymorphize_glyph`/
    `create_region_from_tuples`**: the embedded-peak/intermediate-region
    `F2Dot14` reads inside a tuple header, and the whole per-tuple header
    walk itself (`next_tvh_offset`'s bump, `tsd`'s accumulation via each
    tuple's own `variationDataSize`) had no bounds checking against `gvar`
    at all. `create_region_from_tuples` is the one function here that
    allocates before it can fail (`vq_create_region`, up front, needed to
    write each dimension's span into as it goes) -- a bounds failure
    partway through now frees that allocation before returning `None`
    instead of leaking it, verified by `cargo miri test` (see below).
  - **A discovered-but-not-fixed correctness bug, deliberately left as
    found**: `parse_point_numbers`'s wide (`POINTS_ARE_WORDS`) point-number
    read did `*(data as *mut u16)` with no byte-swap at all, where its
    sibling `read_packed_delta`'s wide-delta case correctly calls `be16()`
    first. Every other 16-bit read in this codebase (`binio::read_16u`,
    every `FontReader::u16()`) is big-endian; this one path reads
    native-endian, which is wrong on this crate's own convention and the
    OpenType spec's. This is a correctness bug, not a parse-boundary
    safety issue, and out of this PR's scope to fix -- fixing it could
    change output for a real, well-formed font whose gvar data actually
    exercises a wide point-number run (none of the committed payloads do,
    checked both ways), which would violate the "well-formed output stays
    byte-identical" invariant this whole stage runs under. Preserved
    exactly (a comment at the call site explains why); a good target for
    a small, separately-scoped follow-up.
  - `polymorphize`'s own `__fortable_*`/`current_block` (goto emulation)
    -> the same `.iter().find()` idiom every other migrated reader uses,
    same as `otfcc_read_glyf`'s conversion in the previous entry.
  - **No behavior change on any committed payload**: `gvar-test.ttf`
    (the only payload exercising this code path at all) stays
    byte-identical on both `compare-with-golden.sh` (build and dump) and
    `compare-roundtrips.js`. 8 new unit tests on the lower-level functions
    (`next_tvh_offset`, `create_region_from_tuples`, `parse_point_numbers`,
    `read_packed_delta`) cover both the truncated-input rejection paths
    and a well-formed read each -- `polymorphize`'s own top-level per-glyph
    guard isn't separately unit-tested (constructing a full `Packet`/
    `FvarTable`/`GlyfTable` harness for it would be disproportionate to
    the marginal coverage over what `gvar-test.ttf`'s end-to-end golden/
    round-trip coverage and the lower-level tests already pin).
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 1029 → 1001 (-28),
    `__fortable_*` 131 → 118 (-13), raw pointer types 6275 → 6248 (-27),
    `as ::core::ffi::c_int` 5536 → 5477 (-59), `while` loops 702 → 695
    (-7).
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 204 tests (was 196 -- 8 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions, `gvar-test.ttf` specifically confirmed), all
    round-trip payloads stable, issue #1's lookup-alias regression still
    passes, `cargo miri test` clean locally before pushing (189 passed, 0
    failed, 15 pre-existing ignores -- confirms `create_region_from_
    tuples`'s new failure-path cleanup doesn't leak).
  - Next: `libcff/` (`gu1`-`gu4` duplicated readers, the CFF INDEX 4GB
    `memcpy` overflow) -- the last remaining Stage 7-1 scope per the plan.
    `table/glyf/read.rs` is now fully converted.

- **Stage 7-1: `table/glyf/read.rs` -- the core glyf outline readers
  (`otfcc_read_glyph`, `otfcc_read_simple_glyph`, `otfcc_read_composite_
  glyph`, `otfcc_read_glyf`).** The plan flagged this file by name as the
  hardest remaining piece in Stage 7-1: unlike every other reader
  converted so far, the three glyph-body readers took **no length
  parameter at all** -- just a raw `start: FontFilePointer` -- and walked
  forward on nothing but what the wire format itself implied (a simple
  glyph's flag/coordinate streams ran until they'd produced
  `endPtsOfContours`-implied many points; a composite glyph's component
  chain ran until it saw the `MORE_COMPONENTS` bit clear). Split into two
  PRs: this one converts the glyf/loca-driven outline readers (the direct
  attacker-reachable parse boundary); the gvar tuple-variation applicator
  (`polymorphize`/`polymorphize_glyph`/`next_tvh`/`parse_point_numbers`/
  `read_packed_delta`, ~670 more lines in the same file) is next, as its
  own PR -- it's a large, independent binary format (F2Dot14 tuples,
  shared/private point-number runs, packed deltas) that only ever runs as
  a post-processing pass over glyphs this PR already finished reading.
  - **`otfcc_read_glyph`/`otfcc_read_simple_glyph`/`otfcc_read_composite_
    glyph` now take (and are bounds-checked against) an actual byte range,
    derived from this glyph's own `loca` entries.** `otfcc_read_glyf`
    already validates that `loca`'s offsets are monotonic non-decreasing
    and fit within the `glyf` table's own length before ever reading a
    glyph, so `offsets[gid+1] - offsets[gid]` is always a real, in-bounds
    byte count -- passed down and used to scope a `FontReader` per glyph.
    All three readers now return `Option<Box<Glyph>>`; a glyph whose data
    doesn't fit its own declared range degrades to an empty glyph (same
    fallback already used for a zero-length `loca` range) instead of
    reading adjacent bytes.
  - **Four real, previously-undocumented bugs**, all only reachable
    through genuinely malformed glyph data (every committed payload stays
    byte-identical -- see below):
    - The fixed 10-byte glyph header (`numberOfContours` + bbox) was read
      with no guard at all; a `loca`-implied glyph shorter than 10 bytes
      read straight into whatever followed it in the `glyf` table's
      buffer (or past the end, for the last glyph).
    - A composite glyph's component chain had no bound *except*
      `MORE_COMPONENTS`; one that never clears it (or is truncated
      mid-record) read forever past its own bytes. Now every field read
      goes through `FontReader`, so running out of bytes fails the read
      and rejects the glyph -- pinned by
      `composite_glyph_more_components_never_cleared_terminates_and_is_
      rejected`.
    - A non-monotonic `endPtsOfContours` (a later entry smaller than the
      previous one) computed this contour's point count in signed
      arithmetic, same as the original, but then cast that possibly-
      negative result straight to `usize` for `glyf_contour_fill`'s fill
      count -- turning a small malformed input into an attempted
      near-`usize::MAX`-point allocation (a DoS, not a memory-safety
      bug, but a real one). Now rejected before the cast can go negative.
      Pinned by `non_monotonic_end_points_of_contours_is_rejected_not_a_
      huge_allocation`.
    - A flag run whose `REPEAT` count overruns the glyph's own declared
      point count indexed a fixed-size buffer with no bounds check --
      silently past the end in the original C, a Rust `Vec` index panic
      if ported verbatim. Now rejected before the write. Pinned by
      `repeat_run_overrunning_the_declared_point_count_is_rejected_not_a_
      panic`.
  - **`otfcc_read_glyf`'s own `loca` guard didn't account for
    `loca_is_long`.** Its upfront length check (`length < 2*num_glyphs+2`)
    used the *short*-format byte-per-entry count unconditionally, so a
    long-format `loca` table (4 bytes/entry) only half as long as it
    needed to be still passed the guard, and the `read_32u` calls that
    followed read past the table's actual end. `FontReader`'s per-read
    checking makes the separate upfront guard unnecessary -- removed
    rather than duplicated, in favor of each `u16()`/`u32()` call failing
    on its own.
  - `__fortable_*`/`current_block` (goto emulation) in `otfcc_read_glyf`
    -> the same `packet.pieces.iter().find(|p| p.tag == ...)` idiom every
    other migrated table reader in this crate already uses.
    `support/binio.rs`'s `read_8u`/`read_8s`/`read_16u`/`read_16s`/
    `read_32u` import is gone from this file entirely (nothing left in
    the converted functions uses it; the untouched gvar code below never
    did -- it reads through `#[repr(C, packed)]` struct field access
    instead).
  - **No behavior change on any committed payload**: every payload stayed
    byte-identical, including glyf-heavy fonts and `gvar-test.ttf`
    (exercises both the outline readers and the still-untouched
    `polymorphize` pass downstream of them), all verified end-to-end by
    `compare-with-golden.sh`/`run-cycles.sh`/`compare-roundtrips.js`. 7
    new unit tests cover all four fixed bugs plus a well-formed simple
    and composite glyph.
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 1080 → 1029 (-51),
    `__fortable_*` 158 → 131 (-27), `current_block` 29 → 23 (-6), raw
    pointer types 6299 → 6275 (-24), `as ::core::ffi::c_int` 5632 → 5536
    (-96), `while` loops 710 → 702 (-8).
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 196 tests (was 189 -- 7 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes, `cargo miri test` clean locally
    before pushing (181 passed, 0 failed, 15 pre-existing ignores).
  - Next: the gvar tuple-variation applicator in this same file
    (`polymorphize` and friends), as its own PR -- then `libcff/` (`gu1`-
    `gu4` duplicated readers, the CFF INDEX 4GB `memcpy` overflow), the
    remaining Stage 7-1 scope per the plan.

- **Stage 7-1: `otl/subtables/chaining/read.rs` (1,246 lines) -- the
  largest single file converted in the whole `otl/` family, and the last
  piece of Stage 7-1's `otl` scope.** Two nearly-parallel implementations
  live in this one file: "Context" (`general_read_contextual_rule`,
  `read_contextual_format1/2`, `otl_read_contextual`) has no
  backtrack/lookahead, "ChainContext" (`general_read_chaining_rule`,
  `read_chaining_format1/2`, `otl_read_chaining`) has both -- each with
  three binary formats (glyph-sequence, class-sequence, coverage-list).
  `single_coverage`/`class_coverage`/`format3_coverage` (the
  `CoverageReaderHandler` callbacks) and `reverse_backtracks` are
  unchanged: the first two never touch raw font bytes, and
  `format3_coverage` already passes its raw pointer/length straight
  through to `read_coverage` unchanged, matching this file family's
  established boundary discipline.
  - **Three real, previously-undocumented bugs, plus one already-fixed
    leak now consolidated.**
    - **`read_contextual_format1`/`read_chaining_format1`: `first_coverage`
      leaked on every failure path.** `read_coverage` always returns a
      valid (never-null) shell even on malformed input, so this was never
      a null-deref -- but the original only ever freed `first_coverage`
      on the success `return subtable;` path; every guard failure below
      it (count mismatch, truncated ruleset array, truncated per-rule
      array) fell through to `subtable_chaining_free(subtable); return
      null;` without touching it. Same shape as `gpos_mark_to_single.rs`/
      `gsub_ligature.rs`'s Coverage leak, fixed the same way: a labeled
      block evaluating to `Option<()>`, with `otl_coverage_free` run
      exactly once, unconditionally, after it.
    - **`read_contextual_format2`/`read_chaining_format2`: the
      `ChainSubClassSet` array had *no guard at all*.** Unlike every
      other array in this file family, the original read `srs_count` (and
      then its own rule-offset array) straight off `offset +
      classSetOffset[j]` with zero bounds checking -- a `classSetOffset`
      pointing past `table_length` read out of bounds outright, not
      merely past an overflowed guard. This is the same "no guard
      whatsoever" shape `gsub_multi.rs`'s Sequence subtable and
      `otl/read.rs`'s `langSysRecords`/`featureIndex` arrays had earlier
      in this stage. Now guarded like every sibling array here; a
      dedicated test (`context_format2_class_set_offset_past_the_table_
      end_is_rejected_instead_of_reading_oob`) pins a `classSetOffset` of
      5000 against a 10-byte table.
    - **`general_read_chaining_rule`: a malformed `nInput < minus_one_q`
      (a font can set `nInput = 0` while this call site always wants its
      one coverage-implied slot filled) would underflow a `u16`
      subtraction and panic**, where the original's signed `c_int`
      arithmetic just ran zero array-read iterations. `general_read_
      contextual_rule` has the analogous case. Fixed by computing the
      reduced count once with `saturating_sub` (for loop bounds) and by
      deriving every array's start position through cumulative `usize`
      addition on that already-validated count, rather than re-deriving
      it via `match_count - minus_one_q`/`input_ends - minus_one_q`
      subtraction the way the original's pointer arithmetic did -- the
      two disagree exactly in this degenerate case, and the `usize`
      version is both panic-free and the one that actually matches where
      `FontReader`'s own guards validated. Pinned by
      `chaining_rule_with_input_count_below_the_minus_one_slot_does_not_
      panic`.
  - **The already-present `cds` (`ClassDefs`) leak-on-failure fix in
    `read_contextual_format2`/`read_chaining_format2` is preserved, not
    newly found here.** Both functions already had a duplicated cleanup
    block on the fallthrough failure path (with a comment explaining the
    leak it closes) from an earlier, pre-`FontReader` pass. Consolidated
    into one unconditional cleanup after the labeled block instead of two
    copies of the same code, but the fix itself predates this PR.
  - **Guard-threshold notes (behavior changes only on malformed input,
    always in the safe/stricter-or-equal direction).**
    `general_read_contextual_rule`'s guard is preserved exactly as
    written (it reserves one array slot more than strictly needed even on
    the `minus_one` path -- over-conservative, not tightened, to avoid
    second-guessing a byte-accounting quirk that was already safe).
    `general_read_chaining_rule`'s four sequential `FontReader` reads
    reproduce the original's four incremental guards exactly (each
    demands precisely the next field/array, matching the original's
    running-total-plus-next-field check). `read_contextual_format2`'s
    `chainSubClassSet` array guard is 4 bytes tighter than the original's
    (`offset+8+2*count` vs. the original's `offset+12+2*count`, which
    reserved 4 bytes beyond the array's own real requirement) -- always
    safe, since a `FontReader` read only ever demands the bytes the value
    it produces actually needs.
  - ClassDefs stays a `#[repr(C)]` struct of three raw `*mut ClassDef`
    fields (not `Vec`/`Box`), so unlike the earlier `init_mark_to_*`
    calloc'd-struct bug this migration found, plain `=` assignment onto
    its `__caryll_allocate_clean`'d fields is not UB (no niche-optimized
    type involved) -- confirmed, not changed.
  - **No behavior change on any committed payload**: every payload stayed
    byte-identical, including the 45 `gsub_chaining` and 1 `gpos_chaining`
    lookup instances across the payload corpus (confirmed by grepping
    `otfccdump` output), all exercised end-to-end by
    `compare-with-golden.sh`/`run-cycles.sh`/`compare-roundtrips.js`. 8
    new unit tests cover both fixed bugs, the underflow-panic fix, the
    zero-classSetOffset sentinel, and a well-formed case for Context
    format1/format3 and ChainContext format3.
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 1199 → 1080 (-119),
    `current_block` 39 → 29 (-10), raw pointer types 6340 → 6299 (-41),
    `as ::core::ffi::c_int` 5835 → 5632 (-203), `while` loops 725 → 710
    (-15). `#![allow(unsafe_op_in_unsafe_fn)]` file count unchanged
    (102/142): this file still defines `single_coverage`/`class_coverage`/
    `format3_coverage` as raw-pointer `unsafe extern "C" fn`s and a
    `#[repr(C)] ClassDefs` of raw pointers, none of which this PR's scope
    (parse-boundary safety, not the C-ism removal in Stage 7-2/7-4)
    touches.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 189 tests (was 181 -- 8 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes, `cargo miri test` clean locally
    before pushing (174 passed, 0 failed, 15 ignored -- the ignored ones
    are the pre-existing `libc::snprintf`-under-Miri exclusions, unrelated
    to this file).
  - Next: `glyf`/`gvar` (no length parameter at all, per the plan) and
    `libcff/` (`gu1`-`gu4` duplicated readers, the CFF INDEX 4GB `memcpy`
    overflow) -- Stage 7-1's remaining scope after `otl/` is now fully
    converted.

- **Stage 7-1: `otl/subtables/gpos_pair.rs` -- the last "regular" (non-
  chaining) subtable reader, and the most structurally complex file in
  this stage so far.** Two very different binary formats live in one
  function (Format 1: per-glyph-pair PairSets, with a synthesized
  `second` ClassDef the wire format doesn't carry explicitly; Format 2: an
  exhaustive class1×class2 value matrix), each fully rewritten with the
  same boundary discipline as the rest of this file family.
  - **Two real, previously-undocumented bugs, one per format.** Format
    1's `coverageOffset` (and every header field after it) used to be
    read with *no* guard beyond the very first `table_length < offset +
    2` -- just the 2-byte format field itself -- so a table only a few
    bytes long claiming format 1 read straight past its own end before
    any of the format's own per-field guards further down ever ran.
    Sequential `FontReader` reads close this by construction: every
    field is checked, not just the ones the original happened to guard
    explicitly. Format 2's final byte-length guard --
    `class1_count * class2_count * (len1+len2)` -- is the same
    overflow-defeats-guard shape as `gpos_mark_to_ligature.rs`'s
    `component_count * class_count`: both `class1_count`/`class2_count`
    are independently unbounded `u16` fields, so the product can reach
    65535*65535*16 (~68.7 billion). Fixed with two chained
    `checked_mul`s (count*count, then that product against the per-cell
    stride via `require_room`) -- the same "checked step, then checked
    step" shape `name.rs`'s two-tier guards used.
  - **No other bugs found.** Unlike several files in this stage, this
    one already freed its intermediate `Coverage`s (`cov`/`cov_0`)
    unconditionally right after consuming them, on every path -- no
    leak to fix here.
  - **No behavior change on any committed payload**: every payload
    stayed byte-identical, including the two committed payloads
    (`BungeeColor-Regular_colr_Windows.ttf`, `Molengo-Regular.ttf`,
    confirmed by grep) that actually carry a `gpos_pair` lookup and are
    exercised end-to-end by `run-cycles.sh`/`compare-roundtrips.js`. 4
    new unit tests cover both fixed bugs plus a well-formed case for
    each format.
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 1240 → 1199, raw
    pointer types 6362 → 6340, `current_block` 44 → 39.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 181 tests (was 177 -- 4 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log
    output, zero exceptions), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes, `cargo miri test` clean
    locally before pushing.
  - Next: `chaining/read.rs` (1,246 lines) and its supporting files in
    `otl/subtables/chaining/` -- the largest and most structurally
    complex file left in the whole `otl/` family, and the last piece of
    Stage 7-1's `otl`/`glyf` scope before moving to `glyf`/`gvar` (which
    have no length parameter at all, per the plan) and `libcff/`.

- **Stage 7-1: `otl/subtables/{gsub_ligature,gpos_mark_to_ligature}.rs`
  -- third batch of individual subtable readers.** Same boundary
  discipline as the previous two batches.
  - **`gpos_mark_to_ligature.rs`: the same overflow-defeats-guard bug as
    `gpos_mark_to_single.rs`'s, but sharper.** The LigatureAttach's
    byte-length guard is `2 * component_count * class_count` -- unlike
    `gpos_mark_to_single.rs`'s `bases.len() * class_count` (where one
    factor is at least bounded by the actual glyph count), *both*
    `component_count` and `class_count` here are independently unbounded
    `u16` fields read straight from the file, so the product can reach
    65535*65535*2 (~8.6 billion) from a much smaller, more plausible
    crafted input. Fixed the same way: `checked_mul` on `usize`.
  - **`init_mark_to_ligature`: the exact same calloc'd-struct
    plain-assignment UB that CI's `miri` job caught in the previous PR's
    `gpos_mark_to_single.rs`.** Found by inspection this time, before
    pushing, rather than by CI -- once one instance of this pattern turns
    up in one `init_*` function in this file family, it's worth checking
    every sibling `init_*` right away rather than waiting for `miri` to
    find each one independently. Fixed with the same `.write()` pattern.
  - **`gsub_ligature.rs`: the same pre-existing Coverage leak as
    `gpos_mark_to_single.rs`'s, from the previous batch** --
    `start_coverage` (always-allocated by `read_coverage`) was only freed
    on the success path in the original; every failure guard reached
    after it was read leaked it. Fixed by freeing it on both the success
    and failure paths uniformly, matching `gsub_ligature.rs`'s own
    `current_block`-eliminated structure. Also dropped `ligature_count`,
    an accumulator the original computed (summing each LigatureSet's own
    ligature count as an incidental side effect of validating it) but,
    per a full grep, never actually read afterward -- the same kind of
    vestigial validation-only computation `otl/read.rs`'s
    `n_language_combinations` turned out to be.
  - **No behavior change on any committed payload**: `KRName-Regular.otf`
    (a font with real ligature substitutions) and
    `mark-consolidate-dedup.ttf` (built specifically to exercise
    `gpos_mark_to_base`/`gpos_mark_to_ligature`) both stayed
    byte-identical. 5 new unit tests cover the fixed bugs plus
    well-formed/mismatched-count cases. Ran `cargo miri test` locally
    before pushing this time, learning from the previous PR's CI-caught
    finding.
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 1278 → 1240, raw
    pointer types 6380 → 6362, `current_block` 58 → 44 -- eliminated in
    both files this batch.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 177 tests (was 172 -- 5 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log
    output, zero exceptions), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes, `cargo miri test` clean
    locally.
  - Next: `gpos_pair.rs` (865 lines, the last "regular" subtable reader
    left) and `chaining/read.rs` (1,246 lines, the largest and most
    structurally complex file in this family) -- likely each its own PR
    given their size.

- **Stage 7-1: `otl/subtables/{gpos_single,gsub_reverse,gpos_mark_to_single}.rs`
  -- second batch of individual subtable readers.** Same boundary
  discipline as the previous batch: raw-pointer public signatures
  unchanged, `FontReader` used internally, labeled blocks
  (`'parse: { ...; break 'parse; }`) replacing `current_block`
  goto-emulation where a header parse either fully succeeds or bails to
  shared cleanup.
  - **`gpos_mark_to_single.rs`: three real, previously-undocumented bugs
    -- the third caught by CI itself, on this PR.** First, the
    BaseArray's byte-length guard --
    `2 * bases.len() * class_count` -- is the same overflow-defeats-guard
    shape as `cmap.rs`'s `n_groups`, just reached by two independently
    large factors instead of one: `bases.len()` can be as large as the
    glyph count (up to 65535) and `class_count` is an unrelated,
    unbounded `u16` read straight from the file, so their product can
    exceed `i32::MAX` (65535*65535*2 is ~8.6 billion) well within
    plausible crafted input. Fixed with `checked_mul` on `usize` before
    ever calling `require_room` -- `usize` is 64-bit on every target this
    crate builds for, so the product of two u16-bounded values can never
    itself overflow the check. Second, a genuine (if minor) memory leak:
    the original only freed `marks`/`bases` (both always-allocated by
    `read_coverage`, even for an empty result) on the success path --
    every failure guard reached *after* they were read fell through to
    `subtable_gpos_mark_to_single_free(subtable)` without freeing either.
    Restructured so the labeled block evaluates to an `Option<*mut
    Subtable>` and cleanup runs exactly once, after the parse attempt, on
    every path -- covered indirectly by the `miri`/`fuzz` CI jobs rather
    than a dedicated unit test (a leak isn't observable through a normal
    assertion).

    Third: `init_mark_to_single` initialized a fresh, `calloc`'d
    (`__caryll_allocate_clean`) `GposMarkToSingleSubtable` with plain `=`
    assignments (`(*subtable).mark_array = Vec::new()`) -- the exact
    "constructing invalid value... encountered 0" UB this crate already
    has a name for (`otfcc-vec-field-assign-needs-calloc` in this repo's
    memory notes, and `logger.rs`'s `otfcc_new_logger` fix from Stage
    7-0-c): the implicit drop of the "already there" all-zero `Vec`
    before the new one is written is UB the instant it runs, whether or
    not it has any observable runtime effect. This one predates this PR
    entirely -- `gsub_reverse.rs`'s equivalent `init_gsub_reverse`
    already used `.write()` correctly, but this file's copy didn't -- and
    had simply never been exercised by `cargo miri test` before, since no
    existing test constructed a `GposMarkToSingleSubtable` at all. CI's
    `miri` job (advisory, but never previously red on this crate for a
    *new* finding) caught it the moment this PR's own new tests started
    calling `otl_read_gpos_mark_to_single`. Fixed the same way
    `otfcc_new_logger` was: `(&raw mut (*subtable).mark_array).write(...)`.
  - **`gsub_reverse.rs`: one real, previously-undocumented bug, a
    different shape from the others in this stage.** `match_count`
    (`n_backtrack + n_forward + 1`, a `TableId`/u16 field) is a sum of
    two independently u16-bounded values plus one, which can itself
    exceed `u16::MAX` even though the original's own array-length guards
    (correctly, unlike the two bugs above) never overflow. The original's
    implicit `as TableId` cast on that sum silently truncated it -- which
    wouldn't have been a bounds-check bypass exactly, but would have gone
    on to index `match_0` (sized to the *truncated* count) with the
    *real*, larger `n_backtrack` a few lines later and panic out of
    bounds, trading a clean "corrupted table" outcome for an abort.
    Rejected instead via `checked_add`. Covered by
    `match_count_overflow_is_rejected_instead_of_panicking` (constructs a
    ~128KB synthetic subtable to actually reach `n_backtrack =
    u16::MAX`, since the guard needs a real backing array that size to
    get past the earlier length checks).
  - **`gpos_single.rs` was already fully guarded** (its `value_count *
    position_format_length` guard's multiplier is bounded to at most 16,
    so no overflow risk) -- a mechanical conversion, not a bug fix.
  - **No behavior change on any committed payload**: the golden set's
    `gpos-single-dedup.ttf` and `gsub-reverse-dedup.ttf` (dedicated to
    these exact readers) plus `mark-consolidate-dedup.ttf` (whose
    `gpos_mark_to_base` lookup routes through `gpos_mark_to_single.rs` --
    `MarkBasePos` and `MarkToSingle` share one binary format, hence the
    shared reader) all stayed byte-identical. 7 new unit tests cover the
    two fixed bugs plus well-formed/mismatched-count cases for all three
    readers.
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 1320 → 1278, raw
    pointer types 6398 → 6380, `current_block` 64 → 58.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 172 tests (was 165 -- 7 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log
    output, zero exceptions), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes.
  - Next: the remaining `otl/subtables/*` readers (gpos_pair,
    gsub_ligature, gpos_mark_to_ligature, `chaining/read.rs`), same
    boundary pattern -- `chaining/read.rs` (1,246 lines) is the largest
    and most structurally complex file left in this family.

- **Stage 7-1: `otl/subtables/{gsub_single,gsub_multi,gpos_cursive}.rs`
  -- first batch of individual subtable readers.** Chosen as the
  smallest, self-contained single-format readers to start this next
  round with. Each takes `otl_read_otl_subtable`'s original raw-pointer
  `(data: FontFilePointer, table_length: u32, subtable_offset: u32, ...)`
  signature unchanged (their caller in `otl/read.rs` isn't touched
  again, and neither are `Coverage`/`otl_read_anchor` in
  `coverage.rs`/`gpos_common.rs`, both still raw-pointer-shaped) --
  internally, each reconstructs a `&[u8]` via `slice::from_raw_parts`
  and uses `FontReader` for its own header fields, same boundary
  discipline as `coverage.rs`/`classdef.rs`'s PR.
  - **Where the original had `current_block` goto-emulation for a
    header parse that either fully succeeds or bails to shared cleanup**
    (freeing whatever coverage tables were already read, freeing the
    subtable shell), the rewrite uses a labeled block
    (`'parse: { ...; break 'parse; }`) instead -- the same shape
    `otl/read.rs`'s `?`-propagation used, but suited to these functions
    because they interleave `FontReader` header reads with raw-pointer
    `Coverage`/`Anchor` calls that a pure `Result` chain can't cleanly
    wrap without more churn than this batch's scope justifies.
  - **`gsub_multi.rs`: a real, previously-undocumented bug, same shape
    as `otl/read.rs`'s `langSysRecords` one.** Each Sequence subtable
    (reached via a per-entry `sequenceOffsets[]` array) had *no* length
    guard at all -- neither `seq_offset` itself nor the Sequence's own
    `glyphCount` (a full attacker-controlled u16) were checked against
    the table's actual length before reading that many glyph IDs.
    `FontReader::at`/`require_room` close both, independently: covered
    by `sequence_offset_past_the_table_end_is_rejected_instead_of_reading_oob`
    and
    `sequence_glyph_count_larger_than_available_is_rejected_instead_of_reading_oob`.
  - **`gsub_single.rs` and `gpos_cursive.rs` were already fully
    guarded** in the original (their `toglyphs`/`value_count` array
    guards use a `u16`-bounded multiplier, so no overflow risk either) --
    both are mechanical conversions, not bug fixes.
  - **No behavior change on any committed payload**: the golden set's
    three dedicated payloads for these exact readers
    (`gsub-single-dedup.ttf`, `gsub-multi-dedup.ttf`,
    `gpos-cursive-dedup.ttf`) plus `unknown-lookup.ttf` (51 GSUB
    lookups) all stayed byte-identical. 8 new unit tests cover the one
    fixed bug plus well-formed/mismatched-count cases for all three
    readers, since no committed payload happens to be malformed here --
    caught two of my own test-data offset mistakes this way (a header
    length miscount and a stale coverage offset) before they could hide
    a real bug behind a coincidentally-passing assertion.
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 1352 → 1320, raw
    pointer types 6407 → 6398, `current_block` 68 → 64.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 165 tests (was 157 -- 8 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log
    output, zero exceptions), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes.
  - Next: the remaining `otl/subtables/*` readers
    (gpos_single/pair/mark_to_*, gsub_ligature/reverse,
    `chaining/read.rs`), same boundary pattern.

- **Stage 7-1: `otl/read.rs` -- the top-level GSUB/GPOS/GDEF
  script/feature/lookup-list parser.** The single biggest structural
  change in this stage so far: `otfcc_read_otl_common` was five levels of
  nested `if`/`current_block` goto-emulation (the c2rust idiom for a C
  function with multiple `goto fail;` labels), because every guard
  failure, at any depth, converged on the same `return None;` at the very
  bottom -- discarding the whole in-progress `OtlTable` (every lookup/
  feature/language already pushed, included). That "any failure discards
  everything, uniformly" shape is exactly what `?`-propagation on a
  `Result` gives for free, so the rewrite (`parse_otl_common`) flattens
  entirely: no `current_block`, no goto targets, half the nesting depth.
  - **Two real, previously-undocumented bugs**, both unrelated to the
    `wrapping_add`-overflow class already fixed elsewhere in this stage
    -- these had *no length guard whatsoever*, not even a defeatable one:
    - `langSysRecords[]` in the script-list walk: `lang_sys_count` is a
      full attacker-controlled `u16`, and the original read that many
      6-byte records completely unconditionally. `require_room` before
      the loop closes it. Covered by
      `lang_sys_records_array_larger_than_declared_is_rejected_instead_of_reading_oob`.
    - `featureIndex[]` in `parse_language` (a LangSys's referenced-
      feature list): same shape, `feature_count` unbounded. Unlike the
      one above, a failure here was already recoverable in the original
      (it clears just that one language's `required_feature`/`features`
      rather than aborting): `require_room` is added to the same
      try-this-language block, so a failure there falls back the same
      way a truncated 6-byte header already did. Covered by
      `feature_index_array_larger_than_declared_falls_back_per_language_not_the_whole_table`.
  - **A verified, deliberate simplification**: the original's script-list
    walk was two passes over every script -- a validation-only first pass
    (computing a `n_language_combinations` count that, per a full grep,
    is never read again anywhere) whose real job was checking every
    `script_offset` up front, *before* the second pass started mutating
    `table.languages`/`table.features` -- because with no per-read bounds
    checking, an OOB read discovered mid-way through the second pass
    would have left partially-built state with no clean way to unwind.
    `parse_otl_common` doesn't need that split: since every failure here
    already discards the whole `OtlTable` via `?`, a bounds failure
    partway through a single merged pass is no different from one at the
    very start -- both end in the same `Err` and the same fully-discarded
    `table_box`. Confirmed there's no other reader anywhere in `otl/`
    that reaches into a partially-populated `OtlTable` after a failed
    parse (`otfcc_read_otl` only ever sees `Ok`/`Err` from this function,
    never a partial value).
  - **`otfcc_read_otl_subtable` (the per-lookup-type-format dispatcher)
    and everything below it in `subtables/*` are untouched** -- still
    raw-pointer-shaped, called from `otfcc_read_otl_lookup` via
    `data.as_ptr()`/`data.len()` derived from the same `&[u8]` this PR
    threads everywhere else, exactly the boundary `coverage.rs`/
    `classdef.rs` established their previous PR.
  - **No behavior change on any committed payload**: every payload with
    GSUB/GPOS/GDEF (most of them, including the golden set's
    `unknown-lookup.ttf` with 51 GSUB + 4 GPOS lookups and every
    `-dedup` payload) stayed byte-identical -- this function builds every
    lookup/feature name and language-system linkage the golden output
    depends on, so this is strong end-to-end confirmation of the
    rewrite, not just unit-level. Issue #1's lookup-alias regression
    (which specifically exercises lookup naming/aliasing) still passes.
    5 new unit tests cover the two fixed bugs plus a well-formed
    round-trip through `parse_otl_common`/`otfcc_read_otl_lookup`
    directly (constructing a minimal synthetic GSUB-shaped table, since
    no committed payload happens to be malformed here).
  - `rust/scripts/survey-unsafe.sh` deltas: `__fortable_*` 172 → 158,
    `.offset(` 1412 → 1352, raw pointer types 6425 → 6407,
    `current_block` (goto emulation) 78 → 68 -- the largest single-PR
    drop in `current_block` count so far, from eliminating this file's
    goto-emulation entirely.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 157 tests (was 152 -- 5 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log
    output, zero exceptions), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes.
  - Next: the individual subtable readers in `otl/subtables/*`
    (gpos_single/pair/cursive/mark_to_*, gsub_single/multi/ligature/
    reverse, `chaining/read.rs`) -- all confirmed (by this and the
    previous PR) to receive the same trustworthy, non-shrinking
    `table_length`/`(data, offset)` pairing, so each can adopt
    `FontReader` independently.

- **Stage 7-1, start of the `otl`/ family: `otl/coverage.rs` and
  `otl/classdef.rs`.** These two are the shared building blocks
  `read_coverage`/`read_class_def` that every GSUB/GPOS/GDEF subtable
  reader calls to resolve a Coverage or ClassDef offset, so they're the
  natural entry point into this next, much larger file family (10,834
  lines across `otl/` and `otl/subtables/`).
  - **Verified before touching anything**: whether it's actually safe to
    convert these two files' *internals* in isolation, ahead of their
    many not-yet-converted callers. Both take `(data: *const u8,
    table_length: u32, offset: u32)`, and the concern was whether
    `table_length` might, at some point in the call chain above these
    functions, get recomputed as a shrinking "remaining budget" via
    subtraction -- the exact shape that caused `cmap.rs`'s underflow bug.
    Traced the whole chain: `otfcc_read_otl`/`otfcc_read_gdef` read
    `table.length` once from the `PacketPiece` and thread that single
    value unchanged through every layer (`otfcc_read_otl_lookup` ->
    `otfcc_read_otl_subtable` -> every per-format reader); only `offset`
    grows via `wrapping_add` as recursion descends. No reassignment or
    subtraction of `table_length` exists anywhere in `otl/`. This means
    `slice::from_raw_parts(data, table_length as usize)` inside these two
    functions really does describe the same allocation the top-level
    table reader validated -- converting bottom-up here doesn't create a
    false sense of safety the way it would have if `table_length` were
    untrustworthy.
  - **Real bug, same class as `cmap.rs`'s but via addition instead of
    multiplication**: both functions' guards used `offset.wrapping_add(N)`
    where `offset` is a `u32` read from the file with no upper bound of
    its own. An `offset` close to `u32::MAX` wraps the whole comparison
    back down to something small, passing a `table_length < ...` guard
    that should have failed. `FontReader::at`/`require_room`'s
    `checked_add`/`checked_mul` close this the same way they did in
    `cmap.rs`. Covered by both files'
    `offset_near_u32_max_does_not_wrap_the_guard` tests.
  - **Public signatures unchanged** (`*const u8`/`u32`/`u32` in, raw
    `*mut Coverage`/`*mut ClassDef` out) -- every other file in `otl/`
    still calls these two exactly as before; only the internals moved to
    `FontReader`. `Coverage`/`ClassDef` themselves, and the surrounding
    `dump`/`parse`/`build`/`shrink` functions (which never touch
    untrusted file bytes), are unchanged.
  - **No behavior change on any committed payload**: every payload with
    GSUB/GPOS lookups (most of them) stayed byte-identical -- the golden
    set specifically includes several `-dedup` payloads built to exercise
    coverage/class-def-heavy consolidation paths. 8 new unit tests are
    the only coverage of the fixed bug itself.
  - `rust/scripts/survey-unsafe.sh` deltas: `.offset(` 1452 → 1412, raw
    pointer types 6427 → 6425 (`Coverage`/`ClassDef` themselves still
    return raw pointers to their many not-yet-converted callers, so the
    drop here is smaller than a full-file conversion's).
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 152 tests (was 144 -- 8 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes.
  - Next: `otl/read.rs` (the top-level GSUB/GPOS/GDEF lookup/subtable
    dispatcher), then the individual subtable readers in
    `otl/subtables/*` (gpos_single/pair/cursive/mark_to_*,
    gsub_single/multi/ligature/reverse, `chaining/read.rs`) -- all of
    which already receive the same trustworthy, non-shrinking
    `table_length` this PR confirmed, so each can adopt `FontReader`
    independently without repeating that provenance check.

- **Stage 7-1: `table/cmap.rs` -- the file the plan's original survey wrote
  up as the primary motivating example for this whole stage.** Both
  headline bugs from that writeup are real and are now fixed.
  - **The `wrapping_sub` underflow.** Every subtable reader used to take
    `(start: pointer, length_limit: u32)`, and `length_limit` was computed
    *once*, by the caller, as `length.wrapping_sub(table_offset)` --
    `table_offset` is a `u32` read straight from the cmap directory, never
    checked against `length` first. A `table_offset` larger than the
    table's own `length` wrapped that subtraction to a number near
    `u32::MAX`, so every downstream `length_limit < ...` guard in whatever
    format reader ran next passed vacuously against a budget that no
    longer meant anything. The exact same shape existed one level down,
    too: `read_format14` computed each `read_uvs_default`/
    `read_uvs_non_default` call's budget the same way, from
    `default_uvs_offset`/`non_default_uvs_offset` fields that are equally
    unchecked. The fix isn't a patched guard -- it's removing the
    `length_limit` concept entirely. Every reader now takes `(data: &[u8],
    offset: usize)`, where `data` is always the *whole* cmap table's bytes
    and `offset` is an absolute, unvalidated position into it;
    `FontReader::at(offset)` rejects `offset > data.len()` before any
    arithmetic on it happens, so there is nothing left to underflow, at
    any recursion depth. Covered by
    `parse_cmap_directory_entry_pointing_past_the_table_end_skips_just_that_subtable`
    and `format14_default_uvs_offset_past_the_table_end_is_a_noop_not_oob`.
  - **The overflowing guard expression.** `read_format12`'s `n_groups`,
    `read_uvs_default`'s `num_unicode_value_ranges`,
    `read_uvs_non_default`'s `num_uvs_mappings`, and `read_format14`'s own
    `n_groups` are all full 32-bit fields read straight from the file,
    each guarded by a `length_limit < K.wrapping_add(S.wrapping_mul(count))`
    check that can itself overflow for a large enough `count` (the
    `12 * n_groups` example the plan's writeup used verbatim). All four
    now use `FontReader::require_room`'s `checked_mul`/`checked_add`,
    matching the fix already applied to `name.rs`'s record array and
    `meta.rs`'s entry array. Covered by
    `format12_n_groups_large_enough_to_overflow_the_multiplication_is_a_noop`,
    `uvs_default_num_ranges_overflow_is_a_noop_not_oob`,
    `uvs_non_default_num_mappings_overflow_is_a_noop_not_oob`, and
    `format14_n_groups_overflow_is_a_noop_not_oob`.
  - **`read_format4` needed no guard fix** -- its own header-level
    `length_limit < 16 + segments_count*8` check is safe as-is, since
    `segments_count` is derived from a `u16` field (bounded to 32767
    after the `/2`), so `segments_count*8` can't overflow. Converted for
    consistency and to drop the raw-pointer array-offset arithmetic
    (`idRangeOffset`'s "byte distance from the entry's own array
    position" indirection, in particular, is exactly the kind of pointer
    math this migration is removing crate-wide). Covered by
    `format4_direct_delta_segment_maps_one_codepoint` and
    `format4_indirect_segment_follows_id_range_offset_into_the_glyph_array`.
  - **This PR only touches the reading half** (`read_format4`/`12`/`14`,
    `read_uvs_default`/`_non_default`, `read_cmap_mapping_table`/`_uvs`,
    `otfcc_read_cmap`). `otfcc_dump_cmap`/`otfcc_parse_cmap`/
    `otfcc_build_cmap*` work entirely on the in-memory `CmapTable` (a
    `BTreeMap`, already safe) or write fresh output buffers -- no
    untrusted file bytes reach them, so they're out of this PR's scope.
  - **No behavior change on any committed payload**: every payload with a
    `cmap` table (which is most of them) stayed byte-identical --
    `compare-with-golden.sh` and `run-cycles.sh` both exercise real
    format4/format12 subtables end to end. 12 new unit tests are the only
    coverage of the fixed bugs themselves, since no committed payload has
    a malformed `cmap`.
  - `rust/scripts/survey-unsafe.sh` deltas: `__fortable_*` 186 → 172,
    `.offset(` 1505 → 1452, raw pointer types 6448 → 6427.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 144 tests (was 132 -- 12 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes.
  - Next: the `otl`/`glyf` families, per the plan.

- **Stage 7-1, seventh batch and last of batch 2: `os_2`/`gasp`/`meta`/`vdmx`.**
  Closes out batch 2 (fixed-length-header-centric files). Unlike the sixth
  batch, two of these four files had real unchecked-offset bugs, matching
  what the plan's original survey expected from this file group.
  - **`table/meta/read.rs`: the same overflow-defeats-guard bug class as
    `cmap.rs`, this time in `meta`.** The header guard was
    `table.length < 16.wrapping_add(12.wrapping_mul(data_maps_count))` --
    a `data_maps_count` large enough to overflow `12 * count` (e.g.
    `0x1555_5556`) wraps the sum back down to something small, so the
    guard passes even though the real entry array is nowhere near that
    short, and the loop then read each entry's `tag`/`offset`/`length`
    straight past the table's actual end. `require_room`'s
    `checked_mul`/`checked_add` closes this the same way it already did
    for `name.rs`'s record array. Each entry's own data span
    (`offset..offset+length`) had the identical wrapping-arithmetic gap
    (`table.length < offset.wrapping_add(length)`); `FontReader::sub`'s
    `checked_add` replaces it. Unlike the header-level guard, a single
    entry failing this check does not drop the whole table -- matching
    the original, which silently skipped just that one entry and kept
    going (covered by
    `entry_whose_span_overflows_offset_plus_length_is_dropped_not_the_whole_table`
    and `entry_span_past_the_table_end_is_dropped_not_the_whole_table`).
  - **`table/vdmx/funcs.rs`: `group_offset` had no bounds check at all --
    not even the wrapping-arithmetic-defeated kind, just nothing.** Each
    ratio range's offset table entry (`group_offset`, a raw `u16` read
    straight from the file) was handed directly to `data.offset()` to
    locate that ratio's VDMX group -- no comparison against the table's
    actual length anywhere before dereferencing it. A crafted
    `group_offset` (or a `recs` count implying group entries past the
    table) could read arbitrarily far out of bounds. Every offset in this
    reader (ratio range, offset table, group) now goes through
    `FontReader::at`, so an out-of-range offset fails the read instead of
    dereferencing it. Unlike `meta`, the original had no existing
    per-entry skip logic to preserve here, so a bad `group_offset` now
    fails the whole table (the same "corrupted" warning the header-level
    guards already used) rather than reading garbage.
  - **`table/os_2.rs`: mechanical, but the most structurally intricate
    file in this batch.** Five nested `version >= N && length < M` gates
    decide, tier by tier, whether the whole table is rejected outright
    (not merely truncated) when a table claims a higher OS/2 version than
    its actual length supports. Ported verbatim -- including the
    pre-existing bug where the version-5 tier's second field write reuses
    `us_lower_optical_point_size` instead of
    `us_upper_optical_point_size`, so the latter is never actually
    populated from the file. Fixing that is a correctness change outside
    this migration's parse-bounds-safety scope, called out in the code
    and covered (not fixed) by
    `version_5_table_leaves_upper_optical_point_size_at_zero`. Sequential
    `FontReader` reads land on the same fixed byte offsets the original's
    explicit `data.offset(N)` calls used, because each version tier's
    length threshold (68 < 78 < 86 < 96 < 100) is strictly increasing and
    every version-gated read only happens once its own threshold has
    already passed -- verified both by the field-by-field comments in the
    code and by `version_1_table_shorter_than_86_is_rejected_even_though_base_fields_parsed`.
  - **`table/gasp.rs`: mechanical**, already had a sound (non-overflowing,
    since `num_ranges` is bounded to a `u16`) length guard.
  - **No behavior change on any committed payload**: `meta-test.ttf` and
    `vdmx-test.ttf` -- the two golden payloads built specifically to
    exercise these readers -- both stayed byte-identical, alongside the
    rest of the 9-payload set with zero exceptions.
  - 19 new unit tests across the four files.
  - `rust/scripts/survey-unsafe.sh` deltas: `__fortable_*` 242 → 186,
    `.offset(` 1585 → 1505, raw pointer types 6497 → 6448.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 132 tests (was 113 -- 19 new), clippy clean, ABI export
    guard, all golden fixtures byte-identical (dump/build and log output,
    zero exceptions), all round-trip payloads stable, issue #1's
    lookup-alias regression still passes.
  - This closes out Stage 7-1 batch 2. Next: `cmap` -- the file the
    plan's original survey wrote up as the primary motivating example for
    this whole stage (`table_offset` read unchecked and used in a
    `wrapping_sub` that underflows past every downstream guard; a
    `12 * n_groups` guard expression that itself overflows) -- then the
    `otl`/`glyf` families.

- **Stage 7-1, sixth batch (start of batch 2): the six fixed-length
  metrics-header tables — `head`/`hhea`/`maxp`/`hmtx`/`vhea`/`vmtx`.**
  Unlike batch 1's files, every one of these six already had a length guard
  before this PR (a lesson from the plan's original survey: `head`/`hhea`
  reject `length < 54`/`36`; `maxp` requires `length == 32 || length == 6`
  exactly; `hmtx`/`vmtx` check `length < count_a*4 + count_k*2`; `vhea`
  requires `length >= 36`), so this batch is mechanical `FontReader`
  conversion rather than bug-fixing — the `__fortable_*`/`.offset()`/raw
  `FontFilePointer` scaffolding around an already-correct guard.
  - **One real behavior change, found while converting `maxp`**: the
    original guard only checks `length == 32 || length == 6`, then branches
    on the *version field itself* (`if version == 0x10000`) to decide
    whether to read the 13 version-1.0-only fields. A table that is exactly
    the 6-byte short form but whose first 4 bytes happen to spell version
    `0x00010000` would take that branch anyway and read the 26
    version-1.0-only bytes straight past a 6-byte buffer's actual end — the
    length check and the version check are independent conditions, and
    satisfying one doesn't imply anything about the other. `parse_maxp`
    keeps the same `data.len() == 32 || data.len() == 6` guard verbatim,
    but the version-1.0 field reads now go through the same `FontReader`
    as everything else, so they fail cleanly (whole table dropped, same
    "corrupted" warning) instead of reading OOB. Covered by
    `six_byte_table_claiming_version_1_0_is_rejected_instead_of_reading_oob`.
  - **No behavior change on any committed payload**: all 9 TTF/OTF payloads
    have well-formed `head`/`hhea`/`maxp` tables, and every payload with
    vertical metrics has well-formed `vhea`/`vmtx` — `compare-with-golden.sh`
    stayed byte-identical across every payload with zero exceptions.
  - 14 new unit tests across the six files (well-formed-parses-every-field
    and one-byte-short-is-rejected for each of `head`/`hhea`/`vhea`; the
    two `maxp` version branches, a length strictly between 6 and 32, and
    the 6-byte-but-claims-1.0 case; a full-metrics-plus-trailing-LSB case
    and a too-short-table case for each of `hmtx`/`vmtx`).
  - `rust/scripts/survey-unsafe.sh` deltas: `__fortable_*` 326 → 242,
    `.offset(` 1652 → 1585, raw pointer types 6572 → 6497.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 113 tests (was 99 — 14 new), clippy clean, ABI export guard,
    all golden fixtures byte-identical (dump/build and log output, zero
    exceptions), all round-trip payloads stable, issue #1's lookup-alias
    regression still passes.
  - Next in batch 2: `os_2`/`gasp`/`meta`/`vdmx` — `meta` and `vdmx` both
    have a real unchecked-offset bug in the current survey (`meta`'s
    `data_maps_count`-driven guard can overflow the same way `cmap`'s does;
    `vdmx`'s `group_offset` is used with no bounds check against the table
    length at all), so that PR is not purely mechanical.

- **Stage 7-1, fifth batch and last of the original batch-1 list:
  `table/name.rs::otfcc_read_name`.** The file the plan's original survey
  flagged as the single worst offender, and the last of the "0–1 guard"
  files from that first pass. Confirms the survey's judgment exactly: the
  record *array* (12 bytes/record starting at offset 6) already had a
  guard (`length < 6 + 12 * count`), but each record's *string* — read from
  `string_offset + offset` for a declared `length` bytes, through one of
  three different decoders depending on the record's platform/encoding
  (`should_decode_as_bytes`'s raw copy, `should_decode_as_utf16`'s
  UTF-16BE conversion, or a `base64_encode` fallback for anything else) —
  had no bounds check at all. Any of the three could read arbitrarily far
  past the table.
  - **Fixed once, for all three decoders**, by resolving the whole string
    span through `FontReader::at` + `peek_bytes` *before* branching on
    which decoder to use — the three `unsafe` legacy helpers
    (`utf16be_to_utf8`/`base64_encode`, still raw-pointer-shaped; Stage
    7-4's concern, not this one) only ever see a byte range already
    confirmed to fit. A record whose string span doesn't fit keeps its
    `platform_id`/`encoding_id`/`language_id`/`name_id` (still meaningful
    metadata) but gets an empty `name_string` instead of the
    out-of-bounds read — the same "keep the record, drop only what
    doesn't fit" choice `table/post.rs::parse_post` made for an
    out-of-range `glyphNameIndex` two PRs ago.
  - **The record-array bound itself is now `checked_mul`/`checked_add`**
    (via `FontReader::require_room`) instead of the original's
    `wrapping_add`/`wrapping_mul`, closing the same "guard expression
    overflows and passes vacuously" gap already fixed in `table/cmap.rs`'s
    guards elsewhere in this crate (documented in this plan's Stage 7-1
    writeup) — an 0xFFFF `count` can no longer wrap `12 * count` back into
    range.
  - **No behavior change on any committed payload**: every one of the 9
    TTF/OTF payloads has a `name` table with real records across all
    three decode paths (Mac-Roman bytes, Windows UTF-16BE, and at least
    one payload exercising the base64 fallback), so this is the first
    file in this stage where the happy path's golden coverage is total,
    not partial — `compare-with-golden.sh` stayed byte-identical across
    every payload with zero exceptions. The malformed-input fix itself
    (like every other in this stage) has no golden payload to exercise it,
    so the 8 new unit tests — Mac-Roman decode, UTF-16BE decode (checked
    against the actual converted UTF-8 bytes, not just "no crash"), a
    truncated header, a record array shorter than its declared count, an
    overflowing count, the string-span-past-the-end case (metadata kept,
    string dropped), a `string_offset` itself past the table end, and a
    zero-length string — are its only coverage.
  - This closes out the plan's original Stage 7-1 batch-1 file list
    (`post`/`ltsh`/`hdmx`/`cvt`/`fpgm_prep`/`_tsi`/`tsi5`/`name`, across
    five PRs). Next: batch 2 (fixed-length-header-centric files:
    `head`/`hhea`/`maxp`/`hmtx`/`vhea`/`vmtx`/`os_2`/`gasp`/`meta`/`vdmx`),
    then `cmap` (where the plan's motivating real bugs live), then the
    `otl`/`glyf` families.
  - `rust/scripts/survey-unsafe.sh` deltas: `__fortable_*` 340 → 326,
    `.offset(` 1677 → 1652 (the largest single-file drop in this stage so
    far), raw pointer types 6583 → 6572.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 99 tests (was 91 — 8 new), clippy clean, ABI export guard,
    all golden fixtures byte-identical (dump/build and log output, with
    zero exceptions this time), all round-trip payloads stable, issue #1's
    regression test green.

- **Stage 7-1, fourth batch: `table/_tsi.rs::otfcc_read_tsi` — the dual-
  table cross-referencing reader deferred out of the third batch.** This
  is the most involved single-function migration in Stage 7-1 so far: it
  reads an "index" table (`TSI0`/`TSI2`, 8-byte records: gid + declared
  text length + text offset) that describes spans into a separate "text"
  table (`TSI1`/`TSI3`), including a forward scan across the index for a
  `>= 0x8000` "compute the length from the next entry's offset" sentinel.
  Three real bugs, all now fixed:
  - **The actual overread this table exists to guard against**: a
    *non-sentinel* declared `text_length` (the common case — anything
    below `0x8000`) was used to slice `text_part` with no check that
    `text_offset + text_length` actually fit inside it. The existing
    `text_offset >= text_part.length` guard caught an out-of-range
    *start*, but said nothing about the *span* — a `text_length` of, say,
    100 against a 2-byte `text_part` read 98 bytes past the end. Fixed by
    resolving the whole span through `FontReader::at` + `peek_bytes`
    before touching any byte; an index entry whose declared length doesn't
    actually fit is dropped rather than read out of bounds — no golden
    payload has ever exercised this table (see below), so there is no
    "matches C" behavior to preserve here, only "don't read past the
    buffer" to establish for the first time.
  - **Two independent instances of the `tsi5.rs`-style off-by-one**: the
    main scan over index records (`j * 8 < index_part.length`) and the
    inner forward-scan for the sentinel's replacement length (`k * 8 <
    index_part.length`) both admitted a final *partial* 8-byte record.
    Both replaced by a shared `read_tsi_index_entry` helper built on
    `FontReader`, which only succeeds when the full record is present —
    the same fix shape as `table/tsi5.rs::otfcc_read_tsi5` two PRs ago,
    applied twice here since the bug pattern appeared twice.
  - **No behavior change for any input this table can correctly parse**:
    the sentinel-length-prediction logic (find the next entry with a
    larger, in-range offset; fall back to "rest of the buffer" if none
    exists) is preserved exactly, including cross-entry state carried
    across loop iterations — verified by a dedicated test
    (`sentinel_length_is_predicted_from_the_next_entrys_offset`) that
    pins the exact byte arithmetic. No committed payload has a TSI0/TSI1
    (or TSI2/TSI3) pair at all, so this table's *only* coverage, both of
    the fix and of the three bugs it replaces, is the 9 new unit tests —
    happy path, the sentinel prediction (with and without a following
    entry), both off-by-one shapes, the real overread, the pre-existing
    out-of-range-offset skip, a zero-length skip, and the
    `Prep`/`Cvt`/`Fpgm` reserved-gid type mapping.
  - `rust/scripts/survey-unsafe.sh` deltas: `__fortable_*` 368 → 340,
    `.offset(` 1686 → 1677, raw pointer types unchanged (this function had
    none of its own to begin with — it borrowed `&PacketPiece`s already).
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 91 tests (was 82 — 9 new), clippy clean, ABI export guard,
    all golden fixtures byte-identical (dump/build and log output), all
    round-trip payloads stable, issue #1's regression test green.

- **Stage 7-1, third batch: `table/fpgm_prep.rs` + `table/tsi5.rs`.**
  `table/_tsi.rs` and `table/name.rs` — the two real-bug-dense files left in
  the original batch-1 list — turned out to need enough of their own
  design work (dual-table cross-referencing for `_tsi.rs`, a string heap
  for `name.rs`) that they're better served as their own focused PRs;
  this one stays small.
  - **`table/fpgm_prep.rs::otfcc_read_fpgm_prep` was already memory-safe**,
    the same shape as `table/cvt.rs::otfcc_read_cvt` two PRs ago: it copies
    the table's own `PacketPiece.data` verbatim, and that buffer is always
    exactly `length` bytes long (an invariant `font/caryll_sfnt.rs`
    establishes when it builds the packet), so there's no
    declared-length-vs-actual-data mismatch to exploit — and no field
    structure to parse, so `FontReader` itself has nothing to add here.
    Still dropped `__fortable_*` and the raw-pointer/`memcpy` pair for
    `table.data.len()` + `copy_nonoverlapping`, for consistency. Real
    payloads (`fpgm`/`prep` are present in 3 of the 9 TTFs) confirm no
    output change.
  - **`table/tsi5.rs::otfcc_read_tsi5` had a real off-by-one overread** —
    the loop condition `j * 2 < table.length` admits one extra 2-byte read
    whenever `table.length` is odd (a 1-byte table satisfies `0 < 1`, then
    reads bytes `[0, 1]`, the second of which doesn't exist). Rewritten as
    `while let Ok(class) = r.u16()`, which requires both bytes to actually
    be present, so an odd-length table now drops its trailing byte instead
    of reading past the end — even-length tables parse identically. No
    committed payload has a TSI5 table, so the 3 new unit tests (even
    length, the odd-length off-by-one specifically, and the empty-table
    case) are this table's only coverage; they build a synthetic `Packet`/
    `PacketPiece` directly and call `otfcc_read_tsi5` end to end rather
    than testing an extracted `parse_*` helper, since this table's
    "parsing" *is* the incremental `push_class_def` construction — there
    was nothing to cleanly separate the way `post`/`ltsh`/`hdmx` could.
  - `rust/scripts/survey-unsafe.sh` deltas: `__fortable_*` 394 → 368,
    `.offset(` 1687 → 1686 (`fpgm_prep.rs`'s single `memcpy` call had
    already collapsed most of its own `.offset()` arithmetic away before
    this batch), raw pointer types 6585 → 6583.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 82 tests (was 79 — 3 new), clippy clean, ABI export guard,
    all golden fixtures byte-identical (dump/build and log output), all
    round-trip payloads stable, issue #1's regression test green.

- **Stage 7-1, second batch: `table/{ltsh,hdmx,cvt}.rs` onto `FontReader`.**
  Same shape as the `post.rs` pilot — each reader's fallible parsing moved
  into a small `parse_*(&[u8]) -> Result<...>` helper that builds nothing
  until every read has already succeeded, `__fortable_*`/`.offset()` gone,
  a `parse_*_tests` module added directly beside each.
  - **`table/ltsh.rs::otfcc_read_ltsh` had no length check at all** — read a
    4-byte fixed header, then `memcpy`'d `num_glyphs` bytes regardless of
    whether the table actually had that many. `y_pels` stays a raw
    `*mut u8` (Stage 7-2's concern, not this one) — `parse_ltsh` now
    resolves the pel slice first and only `__caryll_allocate_clean`s +
    `copy_nonoverlapping`s it in once parsing succeeds, so a rejected table
    never allocates at all. No committed payload has an LTSH table (checked
    by hand against every `tests/payload/*.ttf`), so the 3 new unit tests
    are this table's only coverage of either the fix or the original bug.
  - **`table/hdmx.rs::otfcc_read_hdmx` had the same missing-length-check
    shape** — `record_base` was computed from an untrusted `i * stride`
    with nothing checking the whole records array actually fit the table.
    Confirmed (again) that this function is unreachable from anywhere in
    the crate (`otf_reader.rs` never calls it, matching the module's own
    existing dead-code comment) — fixed anyway, for the same reason the
    file's earlier `Vec`-conversion comment gives ("converted anyway for
    consistency… this was inert, but…"), and because dead code has a way of
    not staying dead. No logger call on the error path, matching this
    function's pre-existing unused `_options` parameter. 2 new unit tests.
  - **`table/cvt.rs::otfcc_read_cvt` turned out to already be memory-safe**,
    unlike the other two — `table_length` is derived directly from the
    table's own declared length (`length / 2`), and the read loop is
    bounded by that exact same value, so `2 * table_length <= length`
    always holds by construction. The plan's original file-level survey
    (a text-pattern heuristic counting explicit guard clauses) flagged this
    as a "0-guard" file; reading the actual arithmetic shows it never
    needed one. Migrated to `FontReader` anyway for consistency with the
    rest of this batch (and because the read genuinely cannot fail here,
    `.expect()` with an explanation of why, rather than a `Result` return
    with no real error path) — no behavior or safety change, pure
    modernization. Left `otfcc_parse_cvt` (the JSON-side reader, including
    its base64-decode path) and `otfcc_build_cvt` untouched — out of this
    stage's scope (OTF binary parsing, not JSON parsing or serialization).
  - **No behavior change on any committed payload**: `cvt_` is exercised by
    5 of the 9 TTF payloads and stayed byte-identical; LTSH/hdmx are
    exercised by none, so their fix is provable only via the new unit
    tests and by the absence of any golden regression.
  - `rust/scripts/survey-unsafe.sh` deltas from this batch: `__fortable_*`
    432 → 394, `.offset(` 1695 → 1687, raw pointer types 6594 → 6585.
  - Verified with the standard pipeline on macOS (arm64 native): 0
    warnings, 79 tests (was 74 — 5 new), clippy clean, ABI export guard,
    all golden fixtures byte-identical (dump/build and log output), all
    round-trip payloads stable, issue #1's regression test green.

- **Phase 5, Stage 7-1 begins: `support/font_reader.rs` (`FontReader`), and
  `table/post.rs`'s reader is the pilot migration.** Stage 7-0 (verification
  infrastructure: golden log/`c/` deletion, clippy, fuzz+miri) is complete
  and merged; this is the first PR of the actual parse-boundary-safety work
  those tools were built to check. `support/binio.rs`'s `read_*` family (465
  call sites across 41 files, per the plan's own survey) takes a bare
  `*const u8` with no length and trusts the caller — `FontReader` replaces
  it with bounds-checked reads over a `&[u8]` that return `Result` instead
  of reading past the end. Nine new unit tests directly on the reader
  (`support/font_reader.rs`): big/little-endian round-trips, signed
  two's-complement, `u24`, past-the-end errors that don't move the cursor,
  `require_room`'s `checked_mul` guard against the "count large enough to
  overflow the multiplication itself" class the plan's own `cmap.rs`
  writeup documents, and `at`/`sub` for following an in-table offset
  without losing the outer buffer's bounds.
  - **`table/post.rs::otfcc_read_post` had zero length checks at all before
    this** — the worst offender the plan's survey found: a 4-byte `post`
    table used to overread 28+ bytes reading its own fixed header, and a
    version-2.0 table's Pascal-string name heap was bounded only by the
    *offset* of each entry's length byte, never `offset + 1 + len`, so a
    single corrupt length byte could read arbitrarily far past the table.
    Split into a new pure, fully-safe `parse_post(&[u8]) -> Result<...>` (no
    `unsafe` anywhere in it) that parses the fixed header and, for a
    version-2.0 table, the entire name-index/name-heap structure into plain
    owned Rust values — and only *then*, once every read has already
    succeeded, builds the real `GlyphOrder` via
    `otfcc_glyph_order_create`/`otfcc_set_glyph_order_by_gid`. That ordering
    is deliberate: it means a read failure can never leave a partially-built
    `GlyphOrder` needing cleanup, so there was nothing to get wrong on the
    error path. `otfcc_read_post` itself shrank to "find the table, call
    `parse_post`, log-and-return-`None` on `Err`, otherwise build the
    `PostTable`" — the `__fortable_*`/`current_block` scaffolding is gone,
    replaced with `packet.pieces.iter().find(|p| p.tag == TAG_POST)`.
  - **A second, quieter bug**: `glyphNameIndex[j] - 258` indexing straight
    into `pending_names` with no bounds check — the original C read
    whatever bytes sat past that allocation; a naive Rust port of the same
    index expression would have turned that into a panic instead (progress,
    but still not acceptable for untrusted input). Falls back to an empty
    name instead of either.
  - **Seven new unit tests on `parse_post` directly**, each pinning one
    specific case: a valid version-1.0 header, a version-2.0 header with
    real pending-name resolution, a standard-Mac-name index that correctly
    never touches the heap, the truncated-header case, the exact
    Pascal-string-length-past-the-end overread described above, the
    out-of-range `glyphNameIndex` case, and a `number_glyphs` large enough
    to overflow `2 * number_glyphs` in the index-array bounds check.
  - **No behavior change on any committed payload**: 9 of the project's TTF
    payloads have a real version-2.0 `post` table with real pending names
    (checked by hand via `otfccdump`), so the happy path was already
    exercised by real data, not just the new synthetic tests — confirmed by
    `compare-with-golden.sh` staying byte-identical across every payload.
    Only genuinely malformed input (none of which exists in the committed
    corpus) takes a different path now: log a warning and skip the table,
    same as every other already-migrated reader's existing length checks,
    instead of reading adjacent memory.
  - **New `rust/scripts/survey-unsafe.sh`**, read-only, reports the
    residual-work counters the plan tracks across Stage 7-1/7-2/7-4 (allow-list
    file count, `unsafe fn`/`.offset()`/`is_null()`/`__fortable_*`/
    `current_block` counts, `Result`/`Option` adoption) in one place, the
    same way `grep -rc "allow(unsafe_op_in_unsafe_fn)"` already serves the
    burn-down. Every future PR touching this area should paste its output.
  - Verified with the standard pipeline on macOS (arm64 native): 0 warnings,
    74 tests (was 57 — 9 new on `FontReader`, 8 new on `parse_post`), clippy
    clean, ABI export guard, all golden fixtures byte-identical (dump/build
    and log output), all round-trip payloads stable, issue #1's regression
    test green.

- **Phase 5, third PR: `cargo-fuzz` and `cargo miri test`, both wired into CI
  as advisory (`continue-on-error: true`), not merge gates.** Last piece of
  the verification-infrastructure trio the plan opens with. Unlike the first
  two PRs, this one **does** touch production code — twice, and only because
  the newly-added tooling would otherwise have been immediately useless (see
  each fix below for why) — everything else found is deliberately left
  unfixed and tracked here for Stage 7-1/7-3 to pick up.
  - **`rust/fuzz/`** (new): two libFuzzer targets. `otf_parse` fuzzes the
    sfnt/OTF binary-parsing path (`otfcc_read_sfnt` + `read_otf`) — the exact
    path this plan's Stage 7-1 was written around (unvalidated offsets in
    `cmap.rs`/`name.rs`/`post.rs`, glyph readers with no length parameter,
    the CFF INDEX overflow) and which had never been fuzzed before.
    `libc::fmemopen` wraps the fuzzer's byte slice as the `FILE*`
    `otfcc_read_sfnt`'s C-inherited signature expects. `json_build` fuzzes
    the JSON→font builder through the actual public FFI entry point
    (`otfccbuild_json_otf`) — no internal reflection needed for that one.
    This subdirectory is its own cargo workspace with its own
    `rust-toolchain.toml` pinned to a dated **nightly** (cargo-fuzz needs
    `-Z sanitizer=address`, stable-only everywhere else in this repo) —
    isolated so `cargo build`/`test`/`clippy` from `rust/` never see it.
  - **Found three real, unfixed bugs within the first ~20 seconds of
    running each target**, none fixed here (see `rust/fuzz/README.md` for
    the full detail, minimized repros committed at
    `tests/fuzz-corpus/known-issues/`):
    - `libcff/charstring_il.rs:405` — `glyph_adw_const as c_int - nominal_width
      as c_int` panics ("attempt to subtract with overflow") when an
      attacker-controlled JSON `advanceWidth` is astronomically large; the
      float→int cast itself saturates correctly, the *subsequent*
      subtraction is what overflows. Silent (wrong) wrapping in an ordinary
      release build, not just a fuzz-build panic.
    - `table/cff.rs:2302` (`cff_make_charset`) — dereferences
      `glyf: *mut GlyfTable` without a null check; `{"CFF_": {}}` (13 bytes)
      reaches it with `glyf` null. Real undefined behavior in a plain
      `cargo build --release`; only panics ("null reference produced")
      under this fuzz build's extra validity checks.
    - `font/caryll_sfnt.rs:220,239` (`otfcc_get16u`/`otfcc_get32u`) call
      `libc::exit(EXIT_FAILURE)` directly from **library** code on a short
      read — exactly the four library-internal `exit()` calls Stage 7-3
      already planned to remove, confirmed here to have a concrete cost
      beyond API cleanliness: it kills the whole `otf_parse` fuzzing
      process on any truncated file, which caps how much of that target's
      state space a campaign can actually explore until it's fixed.
  - **`ffi/dll.rs`'s `otfccbuild_json_otf` leaked its `Options` (and the
    `Logger` it owns) on all three return paths — fixed here.** Found while
    writing the `json_build` fuzz harness; not fixing it would have made
    that target report the identical already-known leak on its very first
    input and stay stuck there forever, which is a different situation from
    "a bug fuzzing found" (the actual point of the tooling) — the same
    "the new infra is worthless without this one fix" reasoning as the
    logger fix below. `ffi/dll.rs` also gained its first unit tests (3 new
    ones — this file had none): the two early-return leak paths, and the
    success path (`otfcc_delete_options` on the third, previously-also-
    leaking return). Along the way, confirmed `read_json` is fully
    permissive — `{}`, `[]`, `null`, `123`, `"x"` all build a valid,
    if nearly empty, font rather than failing, so the `font.is_null()`
    early return is dead code today, not just untested; documented in the
    test module for whoever eventually makes `read_json` fail for real.
  - **`cargo miri test` found a crate-wide bug on its first run — one
    instance fixed here, dozens more tracked, not fixed.** A `calloc`'d
    struct's pointer-niche fields (`Vec<T>`, `Box<T>`, `Option<Vec<T>>`,
    `Option<Box<T>>`, ...) are not a valid bit pattern, so a plain `(*ptr).
    field = value;` first-write is undefined behavior **the instant the
    implicit "drop the old value" step constructs a typed value from the
    all-zero bytes** — independent of whether the drop body ever goes on to
    read or free anything, which is exactly why this was never caught by
    ~57 unit tests, the full golden byte-comparison suite, or any round-trip
    test before Miri: it's usually harmless in practice on today's codegen.
    See the corrected entry in project memory
    (`otfcc-vec-field-assign-needs-calloc`, if consulting these notes
    outside this repo's own memory system) — the earlier belief that calloc
    alone made this pattern "safe by convention" was wrong; only
    `ptr::write` (a placement store that skips the drop entirely) is
    actually sound. `logger.rs`'s `otfcc_new_logger` is fixed (blocks
    essentially every test that touches a `Logger`, which is most of them).
    `font/caryll_font.rs:init_font` (`memset`-zeroes the whole `Font`
    struct, then `json_reader.rs`/`otf_reader.rs` plain-assign `glyf`/
    `cff`/`head`/dozens more fields) has the identical pattern on
    essentially every field and is **not** fixed here — real, substantial,
    crate-wide work belonging to a new Stage 6-4 sub-task (audit every
    `_create()`/`init_*()` pair), not to this infrastructure PR. Tests that
    construct a real `Font` are `#[cfg_attr(miri, ignore = "...")]`'d with a
    pointer to this paragraph.
  - **14 of 57 tests are `#[cfg_attr(miri, ignore)]`'d in total** — the two
    Font-construction ones above, plus 12 that hit genuine Miri
    *limitations*, not bugs (each with its own comment explaining which):
    libc `memmove`/`snprintf`/`strcmp`/ctype functions unsupported as
    foreign functions on the macOS target; `std::fs::read_dir` needing
    `-Zmiri-disable-isolation`; and `parse_number`'s `10.0f64.powf()`
    (JSON exponent handling) — a transcendental function IEEE754 doesn't
    require to be correctly-rounded, confirmed to disagree with native
    libm *and with itself between separate Miri runs* on the identical
    input, i.e. actually non-deterministic under Miri specifically, not a
    parsing bug reachable on any real target. The remaining 43 pass clean.
  - **CI**: `cargo clippy`'s and `warnings = "deny"`'s reasoning (a pinned
    exact toolchain means a new release can't invent a failure) does not
    extend to fuzzing or Miri — both are new dynamic-analysis tools whose
    *current* findings are real but untriaged, so both CI jobs are
    `continue-on-error: true` (advisory), matching how the plan already
    anticipated treating Miri and now treats fuzzing the same way. The fuzz
    job seeds its corpora from `tests/payload/` at CI time rather than
    committing them (avoids duplicating already-committed font payloads);
    the Miri job runs `--test-threads=1` since Miri's concurrency emulation
    is limited.
  - Verified with the standard pipeline on macOS (arm64 native, Rosetta
    rustup): 0 warnings, 57 tests (was 54 — the 3 new `ffi::dll` tests),
    clippy clean, ABI export guard, all golden fixtures byte-identical
    (dump/build and log output unaffected by the `ffi/dll.rs`/`logger.rs`
    fixes), all round-trip payloads stable, issue #1's regression test
    green. `cargo miri test --lib -- --test-threads=1`: 43 passed, 0
    failed, 14 ignored (see above). Both fuzz targets smoke-tested locally
    (short runs plus the two committed crash reproducers), confirmed to
    behave exactly as documented.
  - **CI itself then caught three things local testing missed** — two are
    fixed in a follow-up commit before merge, the third is a genuine
    production finding, documented (not fixed) the same as the fuzzing
    findings above:
    - **`string_edge_cases` also hit the `10.0f64.powf()` non-determinism**
      above — not a flaky multi-threading artifact as first guessed, the
      *same* root cause as `number_edge_cases`, just the fraction-digit
      `powf` call (`1.5`'s fractional part) instead of the exponent one,
      and it happened to round correctly on this Mac's local Miri run by
      chance. Confirmed non-deterministic across platforms, not just
      across runs on one: passed reliably locally on macOS, failed
      reliably on the Linux CI runner. Now `#[cfg_attr(miri, ignore)]`'d
      with the same reasoning.
    - **The `otf_parse` fuzz target's own harness had a real double-close
      bug**, caught by Linux + ASan on the very first (unmutated)
      seed-corpus file, `tests/payload/gvar-test.ttf` — a real, valid,
      already-extensively-tested payload, not a malformed one. Root cause:
      `otfcc_read_sfnt` (`font/caryll_sfnt.rs:172`) takes ownership of the
      `FILE*` passed in and `fclose`s it internally on every path — the
      harness *also* called `libc::fclose` on it afterward, a genuine
      double-close/double-free of the `fmemopen`'d handle. This is a bug
      in the fuzz harness `rust/fuzz/fuzz_targets/otf_parse.rs` introduced
      in this same PR, not a production bug: `bin/otfccdump.rs`'s own
      `otfcc_read_sfnt` call site (the pattern the harness was supposed to
      mirror) never calls `fclose` on the file it opened either, for
      exactly this reason. Confirmed local, un-fixed reproduction against
      the exact byte-identical failing input did *not* crash on this
      Mac's libc (double-`fclose` is implementation-defined behavior, and
      apparently harmless here) — glibc's ASan build is what caught it.
      Removing the redundant `fclose` call fixed it; re-ran the fuzz
      target afterward and the only *crashing* finding left is the
      already-documented, already out-of-scope CFF stack-overflow (same
      crash hash as before, not new) — see the next bullet for what fixing
      the double-close uncovered underneath it.
    - **With the double-close gone, `otf_parse` now runs far enough into
      `gvar-test.ttf` (the same seed file) to surface a real leak**: 478
      bytes across 9 allocations, confirmed reproducible locally with
      symbols. Two sites in the trace: a `calloc` inside
      `table/glyf/read.rs::otfcc_read_glyf`, and a `format!()` allocation
      (`FvarMaster.name`) inside `table/fvar.rs::fvar_register_region`
      reached through it. Both are downstream of `VqRegion` — already
      flagged in this same entry's Stage 6-4 discussion as one of the
      "difficult" remaining raw-pointer types
      (`FvarMaster.region: *mut VqRegion`, blocked on `VqRegion` itself
      becoming `Vec`-shaped) — likely a region-dedup or error path that
      frees/reassigns `region` without going through the drop chain that
      would also free the `FvarMaster` entry pointing at it. Not
      investigated further or fixed here; full detail and reproduction
      steps in `rust/fuzz/README.md`. Left leak detection **on** rather
      than passing `-detect_leaks=0` to get a clean advisory job — that
      would just hide this class of finding (and any future one) instead
      of surfacing it, which defeats the entire point of adding this
      tooling.

- **Phase 5, second PR: clippy wired in with an allow-list ratchet, and CI
  gained a native macOS (arm64) job.** Second piece of the verification-
  infrastructure trio the plan opens with (log-output golden + `c/` deletion
  was the first; fuzz + miri come next), still zero production-code changes.
  - **`cargo clippy --release --all-targets` reported 6,485 warnings across
    30 lint categories on first run — all style/complexity, zero
    `clippy::correctness`.** Every single one is c2rust-transpile shape
    (`unnecessary_cast` 3,435 — matches the `as ::core::ffi::c_int` noise
    already tallied in the plan; `needless_return` 1,426 — c2rust's explicit
    `return expr;` at every function's end; `missing_safety_doc` 924 — every
    `pub unsafe fn` lacking a `# Safety` section; `explicit_auto_deref` 286;
    `ptr_offset_with_cast` 105 — every `.offset()` call; and 25 smaller
    categories down to single digits). None of it was fixed in this PR:
    every category is already scoped to a specific later Phase 5 stage that
    removes it as a side effect of rewriting those exact lines
    (`ptr_offset_with_cast` → Stage 7-1's `FontReader`;
    `missing_safety_doc` → Stages 7-1–7-3, which delete most of the crate's
    961 `unsafe fn`s outright rather than document contracts on functions
    that are about to stop existing; the rest → Stage 7-4's mechanical
    cleanup). Fixing any of it here would be rework once those stages land,
    and would touch files they already plan to touch — the exact
    "container replacement and element ownership in the same PR" mistake
    the `VqSegList`→`Vec` pilot (Stage 6) made and the plan explicitly
    guards against repeating.
  - **`[lints.clippy]` in `Cargo.toml`** lists all 30 categories as
    `"allow"`, each with the count from this PR's clippy run and which
    stage removes it — the same ratchet `unsafe_op_in_unsafe_fn`'s per-file
    `allow` already is (`grep -rc "allow(unsafe_op_in_unsafe_fn)"` as the
    remaining-work count), just crate-wide in `Cargo.toml` instead of
    per-file, since clippy has no per-file granularity to match. CI runs
    `cargo clippy --all-targets -- -D warnings`, so anything not on that
    list — including any *new* lint category introduced by future code —
    is a hard failure. Safe to make fatal for the same reason
    `[lints.rust] warnings = "deny"` already is: the toolchain
    (`rust-toolchain.toml`) is pinned to an exact version, so a new stable
    or clippy release can't invent a warning that breaks this build out
    from under anyone; a deliberate version bump is where that gets dealt
    with.
  - **CI's one `ubuntu-latest` job became a `strategy.matrix` over
    `[ubuntu-latest, macos-latest]`**, running the identical full pipeline
    (build, `cargo test`, clippy, ABI guard, golden dump/build comparison,
    golden log comparison, dump/build cycles, issue #1 regression,
    `compare-roundtrips.js`) on both. `rust/README.md` has said "verified on
    both OSes" for every PR in this log, but that macOS/arm64 half only ever
    happened by hand, locally — this closes that gap.
    `docs.github.com`'s hosted `macos-latest` runners are native Apple
    Silicon (not x86_64-under-Rosetta), so this is also expected to close a
    long-standing coverage hole for free: the `otfccdll` ctypes comparison
    (`dll-arch-check.sh`) SKIPs whenever the built cdylib's architecture
    doesn't match the runner's python3 — which is exactly the failure mode
    this Mac's non-native rustup hits locally (`cdylib is x86_64, python3 is
    arm64`, SKIPped throughout this PR's own local verification runs) — and
    a native arm64 toolchain building on a native arm64 runner with a
    native arm64 python3 shouldn't hit that mismatch at all. Not confirmed
    from a local run (that's precisely the gap this closes); confirm by
    reading the macOS job's log after this PR's CI run.
  - Verified with the standard pipeline on macOS (arm64 native, Rosetta
    rustup — see above for the one expected SKIP): clippy clean under
    `-D warnings`, 0 rustc warnings, 54 tests, ABI export guard, all golden
    fixtures byte-identical (dump/build and log output), all round-trip
    payloads stable, issue #1's regression test green. The matrix's actual
    Linux+native-macOS split is what CI itself verifies from here.

- **Phase 5 kickoff: log output frozen as a golden fixture, then `c/` deleted.**
  First PR of the Phase 5 plan (see the plan file linked from the issue —
  "unsafe を整理する" から "unsafe を無くす" へ). The plan's first move is
  building out the verification net *before* touching any production code,
  because the rest of Phase 5 (parse-boundary safety, `Options`/`Logger`
  ownership, `Result`-typed errors) all needs something to check output
  against once `c/` is gone.
  - **What was missing**: `tests/golden/checksums.sha256` (PR #78, see "CI
    decoupled from C" above) already froze *dump/build output* as the oracle
    for `compare-with-golden.sh`, but `compare-log-output.sh` — the only
    check covering the 425 logger call sites and the `Logger.indents`
    indent-guide rendering (Stage 6-2's `ILogger`/`ILoggerTarget` retype,
    later collapsed to a plain enum in the Phase 4 PRs) — was never migrated
    alongside it. It still rebuilt `c/` from source on every invocation and
    was consequently never added to CI or `test.sh`. Confirmed it still
    passed against C one more time before freezing anything (`PASS` on all
    six cases: `dump-verbose`, `dump-quiet`, `dump-cff-verbose`,
    `build-verbose`, `build-quiet`, `dump-missing-file`).
  - **`rust/scripts/generate-log-golden.sh`** (new) captures the same six
    cases' normalized stderr into `tests/golden/log/*.log` — real committed
    text files, not hashes, since the largest is ~300KB and a `git diff`
    against a log fixture is directly useful for review (unlike the ~28MB of
    dump/build output `checksums.sha256` avoided committing).
    `compare-log-output.sh` was rewritten to diff freshly captured,
    normalized output against those fixtures instead of building C. One
    extra normalization was needed beyond the existing "Step time = …s."
    blanking: the golden-generation script and the comparison script use
    differently-named scratch directories (`build/log-golden-gen` vs
    `build/compare-log-output`), and that path leaks into log text via
    "From file …" and error messages — a bare `cmp` would have failed on
    every run for a reason unrelated to log content. Both scripts' `normalize()`
    now also collapse `build/<anything>/` to a placeholder.
  - **`compare-log-output.sh` is now wired into `test.sh` and CI**
    (`.github/workflows/rust.yml`), immediately after
    `compare-with-golden.sh` — it no longer needs the C toolchain, so
    nothing was stopping it from running on every push/PR any more.
  - **`c/` (8.0MB: `lib/`, `src/`, `dep/`, `include/`, `premake5.lua`,
    `quick.make`, `_vc*.bat`) is deleted.** Nothing in the build or in CI
    referenced it after the above (CI's `paths:` trigger already dropped
    `c/**` back in PR #78). It remains available from git history for
    anyone who needs to diff against the original C source.
  - **`compare-with-c.sh`, `gen-compile-commands.sh`, and
    `filter-compdb.js` moved to `scripts/archive/`.** All three require
    `c/` to run (the first builds and compares against it directly; the
    other two produce a `compile_commands.json` for `c/`'s sources) and
    cannot function once it's gone. They join `transpile.sh` and friends as
    audit trail rather than being deleted outright, consistent with how
    this directory already treats retired-but-documentation-worthy
    tooling — `archive/README.md` explains why and notes they need `c/`
    restored from git history (e.g. `git show <pre-deletion-commit>:c`) to
    run again. `dll-arch-check.sh` was **not** moved: `compare-with-golden.sh`
    and `run-cycles.sh` both still source it for the Rust-only cdylib arch
    check, independent of C.
  - Verified with the full pipeline on macOS (arm64 native): 0 warnings, 54
    tests, ABI export guard, all golden fixtures byte-identical (dump/build
    output and now log output), all round-trip payloads stable, issue #1's
    regression test green.

- **`unsafe_op_in_unsafe_fn` burn-down, sixteenth batch: 12 files, 114 → 102.**
  Resumed the burn-down after the `unsafe extern "C"` removal plan (Phase
  0-4) took priority for a while -- picked off the smallest remaining
  candidates first. Two shapes showed up:
  - **Files that no longer needed the allow at all.** `table/meta/types.rs`
    and `table/vdmx/types.rs` are pure struct definitions with zero
    `unsafe fn` bodies -- the allow was stale weight left over from
    whichever earlier pass first touched the file. Just deleted.
  - **A genuinely root-cause fix, not just wrapping.** `table/meta/build.rs`'s
    `otfcc_build_meta` only needed to be treated as unsafe because it cast a
    perfectly good `&MetaEntry` reference to a raw pointer (`let mut e: *const
    MetaEntry = &entries[...]`) purely to dereference it three lines later
    -- keeping it as a reference and reading `e.tag`/`e.data.len()`/
    `e.data.as_ptr()` directly removes the only unsafe operation in the
    function's own body outright (the remaining `unsafe` blocks are for
    calling into `bk_new_block`/`bk_push`/`bk_build_block`, which are
    genuinely unsafe FFI-shaped functions, not something this function's
    logic caused).
  - **The rest were mechanical `unsafe {}` wrapping**, no redesign needed
    since the operations really are irreducibly unsafe (`support/alloc.rs`'s
    `calloc`/`realloc`/`free`/`exit`/`fprintf` calls, `support/stopwatch.rs`'s
    `clock_gettime`/`snprintf`/raw `timespec` pointer arithmetic,
    `consolidate/otl/gpos_pair.rs`'s subtable pointer walk -- kept its
    `extern "C"` signature untouched, since it's one of the genuine
    per-lookup-type dispatch functions `consolidate.rs`'s
    `__declare_otl_consolidation` registers by address, the kind of case the
    `unsafe extern "C"` removal plan explicitly carved out as real dispatch,
    not vestigial -- and `libcff/cff_writer.rs`'s CFF Type-2 operand-encoding
    arithmetic). Two of these (`table/otl/subtables/chaining/dump.rs`,
    `libcff/cff_writer.rs`) have unsafe operations threaded through nearly
    every line of the function body, so the whole body went in one `unsafe {
    }` block rather than wrapping each call individually -- the lint only
    requires *some* enclosing block, not the smallest possible one, and
    fragmenting a function that's unsafe almost end-to-end into a dozen tiny
    blocks would have made it harder to read, not safer.
  - **Four more picked off in a second pass**, same batch/branch/PR (kept
    it one PR rather than stacking a second one on top of an unmerged
    first):
    - `support/options.rs` -- mechanical wrapping. `otfcc_new_options`'s
      `__caryll_allocate_clean` call, `otfcc_delete_options`'s `free`/
      `logger_dispose` calls, and `otfcc_options_optimize_to`'s field
      writes through the raw `*mut Options` all wrapped in `unsafe {}`.
      Left `Options` as a raw-pointer-accessed struct -- it's threaded
      through the whole crate and four public FFI functions, well outside
      this batch's scope.
    - `table/otl/subtables/extend.rs` -- whole-body wrap of
      `_caryll_read_otl_extend` (raw pointer offsets reading the subtable
      header) plus its two public callers `otfcc_read_otl_gsub_extend`/
      `otfcc_read_otl_gpos_extend`, which do nothing but forward into it.
    - `libcff/cff_value.rs` -- **narrower fix than the file's shape
      invites.** `CffValue`/`CffValueBody` is a C-style tagged union
      (`CffValueType` discriminant + `{ i: i32, d: f64 }` union) that could
      become a Rust enum the same way `CffEncoding`/`ChainingSubtable`/etc.
      already did elsewhere in this crate -- but that conversion ripples
      into `cff_dict.rs`, `cff_parser.rs`, `table/cff.rs`, `libcff.rs`, and
      `cff_codecs.rs`, which is a separate, larger unit of work on its own.
      For this batch, just wrapped the two actually-unsafe operations in
      `cffnum()` (reading `val.c2rust_unnamed.i`/`.d` out of the union) in
      individual `unsafe {}` blocks and dropped the file-level allow.
      Confirmed *constructing* a union value (`CffValueBody { i: 0 }`,
      used at ~8 other call sites in this file and elsewhere) is not
      itself unsafe in Rust -- only reading a union field is -- so nothing
      else in the file needed touching.
    - `table/meta/read.rs` -- whole-body wrap of `otfcc_read_meta`, which
      is almost entirely raw offset reads into the table's byte buffer
      inside the c2rust "single-iteration fortable" `while`-loop idiom.
    - **Found a real `unused_unsafe`/macro-nesting interaction while
      doing this file.** Wrapping the whole function body in `unsafe {}`
      made the `bytesbuild!(b"Table 'meta' corrupted.\n")` call inside it
      fail to build: `bytesbuild!` has its own internal `unsafe { ...
      SdsPart::append_to_vec(...) }`, and once that's nested inside an
      *explicit* enclosing `unsafe {}` block (as opposed to the implicit
      unsafety of an `unsafe fn` body under
      `allow(unsafe_op_in_unsafe_fn)`, which doesn't trigger this), rustc's
      `unused_unsafe` lint correctly calls the inner block redundant --
      and `-D warnings` turns that into a hard error. Considered hoisting
      the `bytesbuild!` call out of the (triply-nested-loop) error path to
      dodge the nesting, but that risks Rust's "value moved here, in
      previous iteration of loop" borrow-checker error, since the
      checker's CFG-based analysis can't see that the loop only runs once
      -- it doesn't reason about the actual runtime truth of the loop
      condition. Fixed it at the source instead: added
      `#[allow(unused_unsafe)]` directly to `bytesbuild!`'s internal
      `unsafe {}` block in `vendor/sds.rs`, with a comment explaining why.
      This is a one-time fix at the macro definition that pre-empts the
      identical conflict at every future burn-down file that (a) wraps its
      whole body in one `unsafe {}` and (b) calls `bytesbuild!` -- which,
      per the macro's own doc comment, is used at ~50+ call sites
      crate-wide, so this exact shape will keep coming up. Confirmed the
      sibling `sdsbuild!` macro has no equivalent internal `unsafe {}` and
      so isn't affected.
  - Verified with the standard full pipeline on both platforms: 54 unit
    tests green (0 warnings under `warnings = "deny"`), every payload
    byte-identical in both directions including the `otfccdll` cdylib
    (`meta-test.ttf`/`vdmx-test.ttf` specifically exercise the touched type
    files), all 10 round-trip payloads stable, issue #1's large-lookup
    regression test green, `compare-log-output.sh` green.
    102 of 141 files still carry the allow -- most of the remaining ones are
    substantially larger and will need the file-by-file, sometimes
    data-structure-redesigning treatment the fifteen batches before this one
    used, not this batch's quick wins.

- **Delete the dead `ComparFn` type alias (`support.rs`).** Noted as a
  byproduct of the Phase 4 planning work but explicitly left out of that
  plan's scope: every `qsort` call site in the crate had already been
  converted to `sort_by`/`sort_unstable_by` in earlier Vec-conversion
  passes, so `ComparFn` (the named type those call sites used to
  `transmute` a concrete comparator into) had zero remaining references
  anywhere -- confirmed by a crate-wide grep for both `ComparFn` and
  `qsort(` before deleting. One-line removal, no behavior change.
  Verified with the standard full pipeline on both platforms: 54 unit
  tests green, every payload byte-identical in both directions including
  the `otfccdll` cdylib, all 10 round-trip payloads stable, issue #1's
  large-lookup regression test green, `compare-log-output.sh` green.

- **Phase 4 sub-phase C (final): collapse `ILogger`, closing out the
  `unsafe extern "C"` removal plan entirely.** `ILogger` (12 fields, one
  implementation, `VTABLE_LOGGER`) was structurally different from every
  other vtable this migration collapsed: dispatch went through an
  *instance* pointer -- `Options.logger: *mut ILogger` -- rather than a
  bare `pub static`, so the field-access shape at every call site was
  `(*(*options).logger).field.expect(...)(logger_ptr, args)` instead of
  `VTABLE.field.expect(...)(args)`. Same single-implementation shape
  underneath, just one more level of indirection to see through. With
  ~312 call sites across 54 files (`.log_sds` alone: 149; `.start_sds`/
  `.finish`: 78 each; `.indent`: 3; `.set_verbosity`: 2; `.dispose`/
  `.dedent`: 1 each), this is the largest single mechanical sweep in the
  whole cleanup after `OTFCC_PKG_GLYPH_ORDER`'s ~628. The existing
  `collapse_vtable_calls.pl` couldn't be reused as-is -- its mapping
  format splits on the first `.` in the left-hand side, which breaks when
  the "vtable expression" itself contains dots (`(*(*options).logger)`)
  -- so a sibling script, `collapse_ilogger_calls.pl`, uses a
  pipe-separated mapping (`VTABLE_EXPR|field=real_fn`) instead. Every one
  of the 312 sites converted cleanly on the first pass; the only manual
  follow-up was stripping the now-dangling `as *mut ILogger` casts every
  call site's `(*options).logger` argument carried (312 more sites,
  handled with a single crate-wide `s/ as \*mut ILogger//g` once
  `Options.logger` was retyped to `*mut Logger` directly) and adding the
  newly-direct `logger_*` names to each of the ~55 files' `use
  crate::logger::{...}` imports (`ILogger` swapped out, whichever
  `logger_*` functions that file actually calls swapped in) -- both bin/
  crates (`otfccbuild.rs`/`otfccdump.rs`) needed the same treatment via
  their `otfcc_rust::logger::` import path, since they're separate crate
  targets.

  Two internal fields had zero call sites anywhere, external or
  internal, once traced (`logger_start`, the `*const c_char`-taking
  sibling of the always-used `logger_start_sds`; and `logger_log`, same
  relationship to `logger_log_sds`) -- both deleted outright, matching
  the "confirmed dead, no call site anywhere in the crate" pattern used
  throughout this project. `logger_indent_sds` and `logger_dedent` had no
  *external* call sites either, but survived: `logger_indent`/
  `logger_start_sds` call the former directly by name, and
  `logger_finish` calls the latter directly by name, once those internal
  vtable-style dispatches collapsed too -- the same "trace the internal
  call chain before declaring something dead" discipline the
  `VQ_I_SEGMENT`/`I_VQ` batches established.

  `ILogger` and `VTABLE_LOGGER` are gone; `Logger.vtable: ILogger`
  (the embedded copy every `Logger` instance carried) is gone too --
  `Options.logger`/`otfcc_new_logger`'s return type are `*mut Logger`
  now, not `*mut ILogger`. With the struct gone, the crate-level
  `#![allow(improper_ctypes_definitions)]` in `logger.rs` -- there since
  the `sds` sweep gave these vtable slots `Vec<u8>` parameters --
  finally came off clean too, confirmed by a full rebuild under
  `warnings = "deny"`.

  Verified with the standard full pipeline on both platforms: 54 unit
  tests green, every payload byte-identical in both directions including
  the `otfccdll` cdylib, all 10 round-trip payloads stable, issue #1's
  large-lookup regression test green, and `compare-log-output.sh` green
  -- particularly load-bearing here, since every one of the 312 rewritten
  call sites feeds this crate's logging output, and this is the one
  check in the pipeline that actually exercises it byte-for-byte against
  the C original.

  **This closes out the original `unsafe extern "C"` removal plan in
  full** -- all 17 Phase 3 vtables plus all 3 Phase 4 exceptions
  (`IFontBuilder`/`IFontSerializer`, `ILoggerTarget`, `ILogger`) are now
  collapsed or converted to idiomatic Rust. The only `extern "C"` left
  anywhere in `rust/src` is the crate's genuine 4-function FFI boundary
  in `ffi/dll.rs`.

- **Phase 4 sub-phases A+B, bundled into one PR: `IFontBuilder`/
  `IFontSerializer` collapse, and `ILoggerTarget` → `LoggerTarget` enum.**
  The original Phase 4 scoping (README, previously) assumed
  `IFontBuilder`/`IFontSerializer` were genuine runtime dispatch needing a
  `FontFormat` enum -- investigation found that was a misclassification.
  All 3 call sites (`ffi/dll.rs`'s `otfccbuild_json_otf`,
  `bin/otfccbuild.rs`, `bin/otfccdump.rs`) construct a *fixed* pair at
  compile time (JSON reader → OTF writer, twice, and OTF reader → JSON
  writer, once) -- no code path ever branches between the 4 possible
  pairings at runtime. So this collapsed exactly like the 17 vestigial
  vtables from Phase 3: `read_otf`/`read_json`/`serialize_to_otf`/
  `serialize_to_json` (already plain functions one layer under the vtable
  wrapper, each just forwarding to a `FontBuilder`/`FontSerializer` trait
  method) went from module-private `unsafe extern "C" fn` to `pub unsafe
  fn` (the `bin/` targets are separate crates, same visibility gotcha as
  `OTFCC_I_FONT` in Phase 3 batch 3), the 3 call sites now call them
  directly, and `IFontBuilder`/`IFontSerializer`, their 4
  `__caryll_allocate_clean`-based constructors, and their 4 identical
  `free(self as *mut c_void)` shells are gone.

  `ILoggerTarget` (stderr output vs no-op) turned out to be the one
  genuinely polymorphic case in the original Phase 4 trio -- two
  implementations, chosen once per call path at `Logger` construction
  (CLI binaries always get stderr, the `ffi/dll.rs` library path always
  gets no-op) and never switched afterward. That shape doesn't fit the
  "collapse to a direct call" mechanical pattern the other cleanups used;
  it needed the crate's other recurring idiom instead -- converting a
  small closed set of C-shaped implementations into a Rust `enum` +
  `match` (same treatment `CffEncoding`/`VqSegment`/`Subtable`/
  `ChainingSubtable`/`CffCharset`/`CffFdSelect` already got). `logger.rs`
  gained `pub enum LoggerTarget { Stderr, Empty }` with a `push` method
  matching on self; `Logger.target` changed from `*mut c_void` (a second,
  separately `__caryll_allocate_clean`'d `StderrTarget` shell) to a plain
  inline `LoggerTarget` field -- one fewer heap allocation per `Logger`.
  `ILoggerTarget`, `StderrTarget`, `VTABLE_STDERR_TARGET`/
  `VTABLE_EMPTY_TARGET`, and their `extern "C"` wrapper functions are
  gone; `logger_get_target` (whose only purpose was fetching the vtable
  pointer to dispatch through) is gone too, since `logger_log_sds` and
  `logger_dispose` now just read `(*self_0).target` directly -- and
  `logger_dispose` no longer needs to dispose the target at all, since an
  inline enum with no heap payload has nothing to free beyond what
  `free()`-ing the `Logger` struct itself already reclaims.
  `otfcc_new_std_err_target`/`otfcc_new_empty_target` stayed as thin
  functions returning `LoggerTarget::Stderr`/`LoggerTarget::Empty`, so the
  3 existing call sites (`ffi/dll.rs`, `bin/otfccbuild.rs`,
  `bin/otfccdump.rs`) needed no changes at all -- `otfcc_new_logger`'s
  parameter type flowed through unchanged from their perspective.

  `ILogger` itself (12 fields, single implementation `VTABLE_LOGGER`, but
  dispatched through an *instance* pointer -- `Options.logger: *mut
  ILogger` -- rather than a bare `pub static`, so the mechanical Phase 3
  pattern didn't apply as-is) is deliberately **not** touched by this PR:
  its ~321 call sites are next, as their own PR, once `Logger.target`'s
  shape was settled here first.

  Verified with the standard full pipeline on both platforms: 54 unit
  tests green (0 warnings under `warnings = "deny"`), every payload
  byte-identical in both directions including the `otfccdll` cdylib, all
  10 round-trip payloads stable, issue #1's large-lookup regression test
  green, and `compare-log-output.sh` green -- particularly load-bearing
  here, since it is the pipeline's only check that actually exercises the
  stderr-vs-no-op branch this PR rewrote.

- **`unsafe extern "C"` removal, Phase 3 batch 7 (final): `VqVectorInterface`/
  `I_VQ` (27 fields, `vf/vq.rs`) -- the largest vtable in the original
  17-vtable inventory, and the last one. 18 of the 27 fields turned out to
  have a real caller somewhere in the crate (`get_still`/`create_still`/
  `replace` alone account for the bulk of the ~170 call sites, across 9
  files: `consolidate.rs`, `libcff/charstring_il.rs`,
  `otf_reader/unconsolidate.rs`, `table/cff.rs`, `table/fvar.rs`,
  `table/glyf/build.rs`, `table/glyf/read.rs`, `table/glyf.rs`,
  `otf_writer/stat.rs`); those 18 functions were widened from
  module-private to `pub(crate)` and every caller's `use` import rewritten
  to name them directly. The remaining 9 fields (`empty`, `plus`,
  `inplace_negate`, `negate`, `inplace_minus`, `equal`, `compare_ref`,
  `show`) needed a closer look than a flat "only-in-the-static" check:
  `vq_inplace_negate`/`vq_negate`/`vq_inplace_minus` looked dead by that
  test alone, but `vq_minus` (alive, real caller) calls `vq_inplace_minus`
  directly by name, which calls `vq_negate` directly, which calls
  `vq_inplace_negate` directly -- a direct-call chain entirely separate
  from the vtable, so all three survive and stayed module-private. The
  other 5 (`vq_empty`, `vq_plus`, `vq_equal`, `vq_compare_ref`, `vq_show`)
  really were only ever reachable through the vtable's own static
  initializer, so they were deleted; deleting `vq_show` cascaded further,
  since its only caller was `show_vq`, which drops out too, and that in
  turn was the only caller of `vq_segment_show`/`show_vqs` in the same
  file (left behind, unconverted, from the `VQ_I_SEGMENT` collapse in
  batch 6 -- that batch's `.show` field had a real caller at the time,
  `show_vq`, so `vq_segment_show` correctly survived that pass; it only
  went dead once `show_vq` itself did, here) and `vq_show_region` in
  `vf/region.rs` (an already-stubbed-out no-op, `pub` and thus invisible to
  the usual unused-function lint, whose only call site was inside
  `show_vqs`). Also extended `collapse_vtable_calls.pl`'s regex once more
  to tolerate the trailing-comma `.expect(...)`-with-comma shape (now
  three sightings of this pattern: batch 4, batch 6, and here). With every
  `vq_*` function's `extern "C"` gone, the crate-level
  `#![allow(improper_ctypes_definitions)]` in `vf/vq.rs` and the four
  files that had copied the same allow alongside their own `VqVectorInterface`
  call sites (`libcff/charstring_il.rs`, `otf_reader/unconsolidate.rs`,
  `table/fvar.rs`, `table/glyf.rs`) all turned out to be safely removable --
  confirmed by a clean rebuild under `warnings = "deny"` after deleting
  each one. Verified with the standard full pipeline on both platforms: 54
  unit tests green, every payload byte-identical in both directions
  including the `otfccdll` cdylib, all 10 round-trip payloads stable,
  issue #1's large-lookup regression test green, `compare-log-output.sh`
  green. **This closes out the entire 17-vtable Phase 3 inventory from the
  original `unsafe extern "C"` removal plan.** What remains from the
  broader plan is Phase 4 (`ILoggerTarget`/`ILogger`/`IFontBuilder`/
  `IFontSerializer`), explicitly scoped out as real runtime dispatch
  needing a bigger redesign (`FontFormat` enum, etc.), to be picked up as
  its own separate thread only if asked.

- **`unsafe extern "C"` removal, Phase 3 batch 6: `VqSegmentElementInterface`/
  `VQ_I_SEGMENT` (11 fields) and `GlyphOrderPackage`/`OTFCC_PKG_GLYPH_ORDER`
  (9 fields), bundled into one PR.** `VQ_I_SEGMENT` turned out to be mostly
  dead: only 3 of its 11 fields (`.dispose`, `.copy`, `.show`) have any real
  caller anywhere in the crate; the other 8 (`vqs_create_still`,
  `vqs_create_delta`, `vq_segment_empty`, `vq_segment_dup`,
  `vq_segment_compare`, `vq_segment_compare_ref`, `vq_segment_equal`,
  `vq_segment_init`) are reachable only through the vtable's own static
  initializer or through each other, so they went away with it -- one of
  the eight (`vqs_create_delta`) already carried a comment from an earlier
  migration pass confirming it was dead. The 3 survivors
  (`vq_segment_dispose`/`vq_segment_copy`/`vq_segment_show`) stayed
  module-private, since every real call site lives in `vf/vq.rs` itself.
  `OTFCC_PKG_GLYPH_ORDER` was the opposite shape: 7 of its 9 non-dead
  fields are genuinely called from 13 other files (`.set_by_gid` alone
  accounts for ~594 of the ~628 call sites, almost all the repetitive
  `aglfn.rs` pattern the plan flagged), so collapsing it meant widening
  7 functions from module-private to `pub(crate)` in `support/glyph_order.rs`
  and rewriting each caller file's `use` import to name the concrete
  functions instead of the vtable. `.init`/`.dispose` had no external
  callers (already called by name internally) and stayed private. Both
  vtables' call sites were rewritten mechanically with the same
  `collapse_vtable_calls.pl` script used since Phase 3 batch 1 -- extended
  this round to also tolerate a trailing comma inside
  `.expect("non-null function pointer",\n)`, a shape the script had missed
  once before (Phase 3 batch 4) and that showed up again here. Verified
  with the standard full pipeline on both platforms: 54 unit tests green
  (0 warnings under `warnings = "deny"`), every payload byte-identical in
  both directions including the `otfccdll` cdylib, all 10 round-trip
  payloads stable, issue #1's large-lookup regression test green,
  `compare-log-output.sh` green. Only `VqVectorInterface`/`I_VQ` (27
  fields, ~178 call sites) remains from the original 17-vtable inventory --
  the largest, planned last, as its own PR per the original plan.

- **`unsafe extern "C"` removal, Phase 3 batch 5: `CffIOutlineBuilder`/
  `DRAW_PASS`, the value-passed outlier the plan flagged as needing a
  different collapse mechanism than the other 16 vtables.** Every other
  vtable was a `pub static` referenced through a global field-access chain
  (`VTABLE.field.expect(...)(args)`); `DRAW_PASS` was instead passed
  *by value* as a whole-struct parameter (`methods: CffIOutlineBuilder`)
  into `cff_parse_outline`, which then copied each field into a local
  `Option<fn ptr>` and dispatched through those -- same degenerate
  single-implementation shape, just structured as a function argument
  instead of a static. Collapsing it meant: dropping the `methods`
  parameter from `cff_parse_outline` entirely; replacing all ~51 scattered
  `local_var.expect("non-null function pointer")(args)` call sites
  throughout its ~1300-line body with direct calls to the corresponding
  `callback_draw_*` function; deleting the per-field `.is_none()`
  fallback-to-`callback_nop_*` blocks (dead the moment `methods` stopped
  existing -- they were already unreachable in practice, since the crate's
  one call site always passed fully-populated `DRAW_PASS`); and deleting
  the now-fully-dead `callback_nop_*` functions and the `CffIOutlineBuilder`
  struct itself. `cff_parse_outline` also recurses into itself for CFF
  subroutine calls (`OP_CALLSUBR`/`OP_CALLGSUBR`), and both recursive call
  sites were still passing `methods` forward -- easy to miss since they
  don't match the vtable-field-access pattern the mechanical sweep looks
  for, caught immediately by `cargo build`'s "cannot find value" once the
  parameter was gone. The `callback_draw_*` functions live in
  `table/cff.rs` but are now called directly from `cff_parse_outline` in
  `libcff/cff_parser.rs`, so they became `pub(crate)` and gained a new
  cross-module `use` -- the two files already had a dependency edge in the
  other direction (`cff_parser.rs`'s `cff_parse_outline` was already
  imported into `table/cff.rs`), so this closes the loop rather than
  opening a new one. Verified with the standard full pipeline on both
  platforms (macOS arm64 and the Linux container): 54 unit tests green (0
  warnings under `warnings = "deny"`), every payload byte-identical in
  both directions including the `otfccdll` cdylib (CFF charstring parsing
  runs on every CFF payload in the suite, so this collapse's correctness
  is exercised heavily, not just compiled), all 10 round-trip payloads
  stable, issue #1's large-lookup regression test green,
  `compare-log-output.sh` green. 3 of the 17 vtables from the original
  inventory remain: `VqSegmentElementInterface`/`VQ_I_SEGMENT` (11),
  `GlyphOrderPackage`/`OTFCC_PKG_GLYPH_ORDER` (10), and
  `VqVectorInterface`/`I_VQ` (27, planned last).

- **`unsafe extern "C"` removal, Phase 3 batch 4: `CffIndexElementInterface`/
  `CFF_I_INDEX` (10 fields) and `CffDictElementInterface`/`CFF_I_DICT` (8).**
  Same collapse pattern, but both vtables carry the individual-case
  callback-parameter shape the plan flagged separately from the
  fake-polymorphism vtables themselves: `CFF_I_INDEX.from_callback` and
  `CFF_I_DICT.parse_to_callback` each take a nested
  `Option<unsafe extern "C" fn(...)>` callback argument, and the concrete
  functions callers pass there (`from_array`, `callback_makestringindex`,
  `callback_makefd`, `callback_extract_private`, `callback_extract_fd`)
  stay `extern "C"` -- only the two vtable wrapper functions themselves
  (`new_index_by_callback`, `parse_to_callback`) lost the annotation, their
  callback *parameter's* type didn't.
  `cff_index_copy`/`cff_dict_copy` were both already-confirmed-dead
  `unreachable!()` stubs (fifth and sixth instances of that shape) and got
  deleted alongside their vtables. `CFF_I_DICT` had two more dead fields
  `CFF_I_INDEX` didn't: `parse_dict` (the whole-buffer parser, superseded
  everywhere by `parse_to_callback`) had zero callers anywhere, and
  `cff_dict_init` had zero callers *even internally* -- unlike every other
  `_init` companion, `cff_dict_create` builds its `CffDict` with a direct
  `Box::new` literal rather than calling `cff_dict_init`, so the init
  function was already fully inert before this PR, not just
  vtable-unreachable. One mechanical-sweep gap surfaced by the build (not
  the address-taken kind from Phase 2, a different regex miss): one
  `.expect("non-null function pointer")` call in `table/cff.rs` had a
  trailing comma after its multi-line string argument
  (`.expect(\n    "...",\n)`), which the collapse script's pattern didn't
  anticipate -- caught immediately by `cargo build`'s "cannot find value"
  error and fixed by hand. Purely mechanical beyond that -- no behavior
  change; verified with the standard full pipeline on both platforms
  (macOS arm64 and the Linux container): 54 unit tests green (0 warnings
  under `warnings = "deny"`), every payload byte-identical in both
  directions including the `otfccdll` cdylib, all 10 round-trip payloads
  stable, issue #1's large-lookup regression test green,
  `compare-log-output.sh` green. 4 of the 17 vtables from the original
  inventory remain: `CffIOutlineBuilder`/`DRAW_PASS` (8, value-passed),
  `VqSegmentElementInterface`/`VQ_I_SEGMENT` (11),
  `GlyphOrderPackage`/`OTFCC_PKG_GLYPH_ORDER` (10), and
  `VqVectorInterface`/`I_VQ` (27, planned last).

- **`unsafe extern "C"` removal, Phase 3 batch 3: `ChainingSubtableElementInterface`/
  `I_SUBTABLE_CHAINING` (5 fields), `ICoverage`/`OTL_I_COVERAGE` (4), and
  `FontElementInterface`/`OTFCC_I_FONT` (7) -- the last one handled with the
  extra care the plan called for, since it's the vtable closest to the
  crate's real FFI boundary.** Same collapse pattern as the prior two
  batches. `I_SUBTABLE_CHAINING.copy`'s target (`subtable_chaining_copy`)
  was the third instance of the by-now-familiar shape: a comment already
  declared it dead (`ChainingSubtable::copy is dead code and unsound for
  owned Vec/Box data`), confirmed by grep before deleting it alongside the
  vtable. `OTFCC_I_FONT.copy`'s target (`otfcc_font_copy`) was the fourth --
  same `unreachable!()` treatment from the low-priority-bundle PR, same
  confirmation, same deletion.
  `OTFCC_I_FONT` needed the most care of any collapse so far because three
  of its four real calls happen inside `ffi/dll.rs` itself
  (`.consolidate`/`.free`), the one file whose `#[unsafe(no_mangle)] pub
  unsafe extern "C" fn` signatures must never change. The fix only ever
  touched call sites *inside* those functions' bodies -- their own
  signatures were untouched -- confirmed directly: `check-abi.sh` still
  reports exactly the same 4 exported symbols, and a hash of `ffi/dll.rs`
  taken before the mechanical rewrite matched after it ran (the script only
  found `OTFCC_I_FONT.field.expect(...)` patterns, which don't appear
  anywhere near those signatures). This also surfaced a real crate-boundary
  wrinkle the earlier batches hadn't hit: `otfcc_font_create`/
  `otfcc_font_free` are called from `bin/otfccdump.rs`/`bin/otfccbuild.rs`,
  which are *separate crates* that depend on the library crate by name
  (Cargo auto-discovers `src/bin/*.rs` as independent binary targets) --
  `pub(crate)` visibility, which sufficed for every other collapsed
  function so far, doesn't reach across that boundary, so those two needed
  full `pub` instead. `delete_font_table` and `otfcc_consolidate_font`
  stayed at their narrower visibility (`pub(crate)` and `pub` respectively,
  matching what each one's actual callers needed). Purely mechanical
  beyond that -- no behavior change; verified with the standard full
  pipeline on both platforms (macOS arm64 and the Linux container): 54
  unit tests green (0 warnings under `warnings = "deny"`), `check-abi.sh`
  green (4/4 exports unchanged), every payload byte-identical in both
  directions including the `otfccdll` cdylib, all 10 round-trip payloads
  stable, issue #1's large-lookup regression test green,
  `compare-log-output.sh` green. 6 of the 17 vtables from the original
  inventory remain: `CffIOutlineBuilder`/`DRAW_PASS` (8, the value-passed
  outlier), `VQ_I_SEGMENT` (11), `CFF_I_INDEX` (10), `CFF_I_DICT` (8),
  `OTFCC_PKG_GLYPH_ORDER` (10), and `I_VQ` (27, planned last).

- **`unsafe extern "C"` removal, Phase 3 batch 2: `CffTableElementInterface`/
  `TABLE_I_CFF` (5 fields), `GposPairSubtableElementInterface`/
  `I_SUBTABLE_GPOS_PAIR` (5), and `IClassDef`/`OTL_I_CLASS_DEF` (5).**
  Same collapse pattern as batch 1, but two of these three arrived
  pre-thinned by earlier work: `TABLE_I_CFF` already had `.copy`/
  `.dispose`/`.free` set to `None` (deleted in a prior pass once their
  targets went fully dead), and `I_SUBTABLE_GPOS_PAIR.copy`'s target
  (`subtable_gpos_pair_copy`) was already flagged dead in a code comment
  referencing the `otfcc-stage6-vtable-copy-move-mostly-dead` memory --
  this PR is what finally deletes it, along with the two now-unreachable
  `None` fields' comments and the vtable shells around all three. Only
  `OTL_I_CLASS_DEF` had every field genuinely in use (`.free`/`.dump`/
  `.parse`/`.build`/`.shrink`, all real) -- its 24 call sites span 6 files
  (`consolidate/otl/gdef.rs`, `consolidate/otl/gpos_pair.rs`,
  `table/tsi5.rs`, `table/gdef.rs`, `table/otl/subtables/gpos_pair.rs`,
  `table/otl/subtables/chaining/build.rs`), the widest spread of any
  vtable collapsed so far. Its five backing functions were already
  `pub(crate)`, so no visibility widening was needed there, just updating
  each file's `use` import to name the specific functions it calls instead
  of the vtable. Purely mechanical beyond that -- no behavior change;
  verified with the standard full pipeline on both platforms (macOS arm64
  and the Linux container): 54 unit tests green (0 warnings under
  `warnings = "deny"`), every payload byte-identical in both directions
  including the `otfccdll` cdylib, all 10 round-trip payloads stable,
  issue #1's large-lookup regression test green, `compare-log-output.sh`
  green. 9 of the 17 vtables from the original inventory remain, including
  `I_VQ` (27 fields, planned last).

- **`unsafe extern "C"` removal, Phase 3 begins: the first 5 vtable
  collapses, batched into one PR.** `CffSubrGraphElementInterface`/
  `CFF_I_SUBR_GRAPH` (2 fields), `FvarTableElementInterface`/
  `TABLE_I_FVAR` (2), `PointElementInterface`/`GLYF_I_POINT` (2),
  `ComponentReferenceElementInterface`/`GLYF_I_COMPONENT_REFERENCE` (2),
  and `GsubReverseSubtableElementInterface`/`I_SUBTABLE_GSUB_REVERSE` (5)
  are gone -- each was a single-static struct-of-function-pointers with no
  real runtime dispatch (a C habit of grouping related functions, not
  polymorphism), so every `VTABLE.field.expect("non-null function
  pointer")(args)` call site became a direct `real_fn_name(args)` call,
  the struct definition and its one static instance were deleted, and the
  functions they used to dispatch through dropped `extern "C"` now that
  nothing takes their address. One knock-on cleanup fell out along the
  way: `I_SUBTABLE_GSUB_REVERSE.copy`'s target
  (`subtable_gsub_reverse_copy`) had zero callers anywhere outside its own
  vtable entry -- once the vtable was gone it was genuinely dead code, so
  it (and the now-unused `memcpy` import) were deleted outright rather
  than kept as an orphaned function nobody calls. `TABLE_I_FVAR.
  register_region`'s target and both `GLYF_I_POINT`/
  `GLYF_I_COMPONENT_REFERENCE` targets needed to go from module-private to
  `pub` (the vtable's `pub static` used to be the only public path to
  them) and their cross-file `use` imports updated accordingly. Purely
  mechanical beyond that -- no behavior change; verified with the standard
  full pipeline on both platforms (macOS arm64 and the Linux container):
  54 unit tests green (0 warnings under `warnings = "deny"`), every
  payload byte-identical in both directions including the `otfccdll`
  cdylib, all 10 round-trip payloads stable, issue #1's large-lookup
  regression test green, `compare-log-output.sh` green. 12 of the 17
  vtables from the original inventory remain, including `I_VQ` (27
  fields, the largest, planned last).

- **`unsafe extern "C"` removal, Phase 2 complete: the remaining
  349 directly-called functions across 56 files, in one combined PR.**
  Covers everything the first three slices hadn't reached:
  `table/otl/` (including all of `table/otl/subtables/`, `chaining/`, and
  `build.rs`'s `OtlBuilder`/`OtlSplitBuilder` dispatch machinery),
  `table/glyf/build.rs`+`read.rs`, `table/meta/`, `table/vdmx/funcs.rs`,
  `consolidate.rs`+`consolidate/otl/`, `otf_reader.rs`+`unconsolidate.rs`,
  `otf_writer.rs`+`stat.rs`, `json_reader.rs`, `json_writer.rs`,
  `font/caryll_sfnt.rs`+`caryll_sfnt_builder.rs`, all of `support/`,
  `vendor/emyg_dtoa.rs`+`sds.rs`, and one function Phase 1 had missed in
  `bin/otfccdump.rs` (`getchar`, a thin `fgetc(stdin)` wrapper, called only
  by name). At this scale a single crate-wide exclusion-set computation was
  used again (same address-taken grep as the second and third slices) and
  applied across every file in one pass rather than one PR per directory,
  since the finding from the third slice held: the same list protects
  every vtable and individual case without per-file lookups. It also
  deliberately swept `logger.rs` and `font/caryll_font.rs` -- both carry
  vtables (`ILogger`/`ILoggerTarget`, `IFontBuilder`/`IFontSerializer`)
  explicitly deferred to a future Phase 4 as genuine runtime dispatch, not
  fake polymorphism -- but only their few non-vtable helper functions
  (`otfcc_new_logger`, `otfcc_new_std_err_target`, `otfcc_new_empty_target`,
  `dispose_font`, `init_font`) matched nothing address-taken and were
  swept; every vtable-backed function in both files stayed untouched, so
  the Phase 4 boundary was respected without needing a manual file
  exclusion. One real gap surfaced by `cargo build` itself: three
  functions (`subtable_chaining_create`, `subtable_gpos_pair_create`,
  `subtable_gsub_reverse_create`) are address-taken via a bare
  `Some(name)` with no `as` cast, and a stale intermediate file from an
  earlier step in the exclusion-set computation had silently dropped them
  from the address-taken set despite matching the same detection regex --
  the build's three "expected \"C\" fn, found \"Rust\" fn" errors caught
  it immediately, they were reverted by hand, and a clean rebuild afterward
  confirmed nothing else had slipped through. This is exactly the safety
  net the original plan described: the mechanical sweep doesn't need a
  perfect exclusion list up front, because a wrong one fails to compile
  rather than silently corrupting behavior. Verified with the standard
  full pipeline on both platforms (macOS arm64 and the Linux container):
  54 unit tests green (0 warnings under `warnings = "deny"`), every
  payload byte-identical in both directions including the `otfccdll`
  cdylib, all 10 round-trip payloads stable, issue #1's large-lookup
  regression test green, `compare-log-output.sh` green. Phase 2 (the
  ~734-function mechanical sweep) is now fully done; what remains is
  Phase 3's 17 vtable collapses and Phase 4's three genuine-dispatch
  exceptions.

- **`unsafe extern "C"` removal, Phase 2 (third slice): `table/`'s top-level
  files, 192 directly-called functions across all 27 files directly under
  `table/` (excluding the `table/otl/` and `table/glyf/` subdirectories,
  saved for later batches).** This slice covers more vtables and individual
  address-taken cases at once than any prior one: `table/fvar.rs`'s
  `FvarTableElementInterface`/`TABLE_I_FVAR`, `table/glyf.rs`'s
  `PointElementInterface`/`GLYF_I_POINT` and
  `ComponentReferenceElementInterface`/`GLYF_I_COMPONENT_REFERENCE`,
  `table/cff.rs`'s `CffTableElementInterface`/`TABLE_I_CFF` and the local
  `CffIOutlineBuilder`/`DRAW_PASS` callback methods (`callback_draw_*`) and
  the `CFF_I_INDEX.from_callback` individual-case callbacks
  (`callback_makestringindex`/`callback_makefd`), and both real call sites
  of `support/ttinstr.rs`'s `parse_ttinstr` callback pair
  (`table/fpgm_prep.rs`'s `make_fpgm_prep_instr`/`wrong_fpgm_prep_instr` and
  `table/glyf.rs`'s `make_instrs_for_glyph`/`wrong_instrs_for_glyph`).
  Rather than hunt each of these down file by file, the exclusion set was
  computed once, crate-wide (every identifier address-taken via
  `as unsafe extern "C" fn` or a resolvable bare `Some(...)`, from the same
  script used for the two prior slices), then applied unmodified across all
  27 files -- the same list correctly protected every one of these cases
  without a single per-file lookup, and a clean `cargo build` afterward
  confirmed nothing address-taken had been missed. 34 declarations stayed
  untouched this way; the other 192 dropped `extern "C"` with zero
  call-site changes. Purely calling-convention annotation removal, no
  behavior change; verified with the standard full pipeline on both
  platforms (macOS arm64 and the Linux container): 54 unit tests green (0
  warnings under `warnings = "deny"`), every payload byte-identical in both
  directions including the `otfccdll` cdylib, all 10 round-trip payloads
  stable, issue #1's large-lookup regression test green,
  `compare-log-output.sh` green.

- **`unsafe extern "C"` removal, Phase 2 (second slice): `vf/` + `bk/`'s 34
  directly-called functions.** `vf/vq.rs` alone holds 46 `unsafe extern "C"
  fn` declarations, but 38 of them back the two largest vtables in the whole
  crate -- `VqVectorInterface`/`I_VQ` (27 fields) and
  `VqSegmentElementInterface`/`VQ_I_SEGMENT` (11 fields), both single-static
  and both deferred to Phase 3 -- leaving only 8 sweepable. Cross-checked by
  computing the field counts against the crate-wide address-taken grep
  (27 + 11 = 38, an exact match), rather than trusting either number alone.
  `vf/region.rs` (8), `bk/bkblock.rs` (10), and `bk/bkgraph.rs` (8) had no
  vtable involvement and swept in full. Purely calling-convention annotation
  removal, no behavior change; verified with the standard full pipeline on
  both platforms (macOS arm64 and the Linux container): 54 unit tests green
  (0 warnings under `warnings = "deny"`), every payload byte-identical in
  both directions including the `otfccdll` cdylib, all 10 round-trip
  payloads stable, issue #1's large-lookup regression test green,
  `compare-log-output.sh` green.

- **`unsafe extern "C"` removal, Phase 2 (first slice): `libcff/`'s ~92
  directly-called functions.** The mechanical sweep continues into the
  ~734-function batch, directory by directory, starting with `libcff/`. This
  slice was more involved than Phase 1's `bin/` sweep: `libcff/` also holds
  four of the vtable structs Phase 3 will collapse (`CffSubrGraphElementInterface`/
  `CFF_I_SUBR_GRAPH`, `CffIndexElementInterface`/`CFF_I_INDEX`,
  `CffDictElementInterface`/`CFF_I_DICT`, and `CffIOutlineBuilder`, whose
  single static `DRAW_PASS` instance lives in `table/cff.rs`) plus a handful
  of individually address-taken callback functions, so a per-function
  exclusion list was required rather than a blanket per-file skip. Built the
  list by grepping the *entire* crate (not just `libcff/`) for every
  identifier immediately preceding `as unsafe extern "C" fn` or wrapped in a
  bare `Some(...)` that resolves to a real function -- this caught one thing
  the original planning pass missed: `cff_parser.rs`'s six `callback_nop_*`
  functions, which back-fill missing fields on a caller-supplied
  `CffIOutlineBuilder` and are address-taken the same way `DRAW_PASS`'s
  fields are, even though `cff_parser.rs` itself holds no vtable. 34 of the
  122 `unsafe extern "C" fn` declarations in `libcff/` matched this
  crate-wide address-taken set and stayed untouched (deferred to Phase 3 or
  the individual-case cleanup); the other 88 dropped `extern "C"` with zero
  call-site changes. Also handled `cff_codecs.rs`'s `DE_T2`, the 256-entry
  CFF Type-2-opcode jump table flagged in the plan as a real dispatch
  mechanism rather than a fake-polymorphism vtable: stripped `extern "C"`
  from the array's element type and from the four functions
  (`cff_dec_e`/`_i`/`_o`/`_r`) it references, while leaving the array and its
  `DE_T2[opcode].expect(...)()` dispatch untouched. Purely calling-convention
  annotation removal, no behavior change; verified with the standard full
  pipeline on both platforms (macOS arm64 and the Linux container): 54 unit
  tests green (0 warnings under `warnings = "deny"`), every payload
  byte-identical in both directions including the `otfccdll` cdylib, all 10
  round-trip payloads stable, issue #1's large-lookup regression test green,
  `compare-log-output.sh` green.

- **`unsafe extern "C"` removal, Phase 1 of a new multi-phase cleanup: the
  8 spurious `extern "C"` internal helpers in `bin/`.** A fresh audit (three
  parallel agents plus a planning pass) found that of the crate's 889
  `unsafe extern "C" fn` declarations left over from the c2rust transpile,
  the real public FFI boundary is exactly 4 functions in `rust/src/ffi/dll.rs`
  (the only ones both `#[unsafe(no_mangle)]` and reachable through the
  `cdylib` crate-type -- `extern "C"` alone exports nothing). The other 885
  are internal-only artifacts of the transpiler always attaching `extern "C"`
  regardless of whether a function's address is ever taken: ~734 are called
  by plain name and can drop `extern "C"` with zero call-site changes (Rust's
  call syntax doesn't encode calling convention), ~155 have their address
  taken and stored in one of 17 single-static "vtable" structs that are a
  pure C habit of grouping related functions rather than genuine runtime
  polymorphism, and a handful of individual address-taken cases (callback
  parameters, dispatch-table type aliases, and one real 256-entry CFF
  Type-2-opcode jump table) need atomic per-case handling. This first unit
  tackles the safest slice: `otfccdump.rs`/`otfccbuild.rs`'s internal
  `atoi`/`printInfo`/`printHelp` (both files) and `readEntireFile`/
  `readEntireStdin` (`otfccbuild.rs` only) -- none `#[no_mangle]`, and
  binaries aren't part of the `cdylib` ABI at all, so there's no exposure
  here regardless. Purely a calling-convention annotation removed, no
  behavior change; verified with the standard full pipeline on both
  platforms (macOS arm64 and the Linux container): 54 unit tests green (0
  warnings under `warnings = "deny"`), every payload byte-identical in both
  directions including the `otfccdll` cdylib, all 10 round-trip payloads
  stable, issue #1's large-lookup regression test green, `compare-log-output.sh`
  green. Later phases will work through the ~734 directly-called functions
  (grouped by directory, ~6 PRs) and then the 17 vtable structs one at a
  time (smallest first, `I_VQ`'s 27 fields last); three genuinely
  runtime-dispatched exceptions (`ILogger`/`ILoggerTarget`,
  `IFontBuilder`/`IFontSerializer`) are out of scope for this cleanup and
  deferred to a separate future redesign.

- **The three lower-priority items the same final audit found -- two
  confirmed-dead-but-armed spots and one pre-existing-in-C leak -- closed
  out too, at the user's request, rather than left as accepted debt.**
  - **`font/caryll_font.rs`'s `otfcc_font_copy`** (`OTFCC_I_FONT.copy`, zero
    call sites anywhere in the crate) becomes `unreachable!()` instead of
    its old `memcpy`-of-the-whole-`Font`-struct body -- the same "confirmed
    dead, loud failure instead of silently reintroducing the risk if it
    ever gets wired up" treatment already given to `cff_index_copy`/
    `cff_dict_copy`/`subtable_gpos_pair_copy`, just missed when `Font`'s
    ~25 table fields were converted to `Option<Box<_>>`/`Vec`.
  - **`table/hdmx.rs`'s `HdmxTable.records`/`DeviceRecord.widths`** convert
    to `Vec<DeviceRecord>`/`Vec<u8>`, and `DeviceRecord` drops `Copy` --
    the exact "`Copy` struct owning heap data behind a raw pointer" smell
    the audit was built to catch, matching `Packet`/`PacketPiece`'s shape
    even though `HdmxTable` itself is confirmed entirely dead code (no
    wired build/dump path in this crate at all). The manual `Drop for
    HdmxTable` impl this replaces is deleted outright -- `Vec`'s own drop
    glue reaches every level now, nothing left to write by hand.
  - **`table/otl/subtables/chaining/read.rs`'s `read_contextual_format2`/
    `read_chaining_format2`** leaked their `cds: *mut ClassDefs` (and its
    populated `ClassDef`s, each owning `Vec`-backed glyph data) whenever a
    malformed Format-2 chaining-context lookup passed the first length
    check but failed the second: control fell through to the shared
    failure return without running the same cleanup the success path just
    above it already does. This exact leak exists in the original C too
    (its `goto FAIL` skips the equivalent free), so it was left alone in
    every earlier pass out of C-parity caution -- but a leak has no
    observable effect on this crate's byte-for-byte output comparisons,
    so fixing it doesn't risk that parity. Both functions gained the same
    cleanup block, duplicated verbatim, right before their fallback
    `I_SUBTABLE_CHAINING.free`/return.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every payload byte-identical in both directions
    including the `otfccdll` cdylib; all 10 round-trip payloads stable;
    issue #1's large-lookup regression test green; `compare-log-output.sh`
    green. (None of the three payloads exercise a malformed Format-2
    chaining table or `HdmxTable`/`otfcc_font_copy` at all -- these three
    fixes are unreachable on every payload in the standard suite, verified
    by code review rather than by the comparison pipeline, same caveat as
    the two bugs fixed in the previous unit.)

- **Two real bugs, found by a final full-crate audit for anything still
  not memory-safe or not idiomatic, fixed in one small PR.** With every
  item from the original full-crate audit's Vec-conversion list now
  landed (the two units above), three parallel agents re-swept the entire
  crate (`table/`, `libcff/`, everything else) looking specifically for
  what earlier passes might have missed -- genuinely live gaps, not
  re-litigating already-accepted patterns like the vtable/`*ElementInterface`
  dispatch shape or the documented "outer struct shell stays a raw pointer"
  fold points. Five candidates came back; three were dead code or already
  provably harmless (a `Copy`-deriving `DeviceRecord` in the confirmed-dead
  `HdmxTable`; a pre-existing leak in `otl/subtables/chaining/read.rs`'s
  Format-2 malformed-input path that already exists in the original C, left
  alone for C-parity; a `memcpy`-based whole-`Font` struct copy with zero
  call sites, deferred rather than fixed since it's inert). Two were real:
  - **`table/cff.rs`'s `cff_make_private_dict` zero-initialized a `CffDict`
    via `__caryll_allocate_clean` (calloc) instead of `CFF_I_DICT.create()`**
    -- unsound now that `CffDict` owns `ents: Vec<CffDictEntry>` (an all-zero
    bit pattern is not a valid `Vec`), and the calloc'd memory was later
    reclaimed via `Box::from_raw` in `CFF_I_DICT.free`, an allocator
    mismatch on top of the invalid-`Vec` UB. Live on every CFF-flavored font
    build (both the plain and CID/FD-array private-dict paths) --
    `cff_make_fd_dict`, two functions above in the same file, already did
    this correctly, confirming the miss was an oversight, not a deliberate
    choice, when `CffDict`'s outer shell was converted to `Box`.
  - **`support/ttinstr.rs`'s `dump_ttinstr` leaked the buffer
    `base64_encode` allocates** when `options.instr_as_bytes` is set --
    `json_string_new_length` copies the bytes into a fresh `Vec` rather than
    taking ownership of the buffer passed to it, so the buffer itself was
    never freed. `table/name.rs`'s three sibling `base64_encode`/
    `base64_decode` call sites already free correctly; this one was missed.
  - Both fixes are small and behaviorally invisible to the standard
    byte-comparison pipeline (neither changes any output, only what gets
    freed/how a struct is initialized) -- verified by code review plus the
    standard full pipeline on both platforms: 54 unit tests green (0
    warnings under `warnings = "deny"`); every payload byte-identical in
    both directions including the `otfccdll` cdylib, with particular
    attention to the CFF-heavy payloads (`KRName-Regular-O2.otf`
    subroutinize, `cid-fdselect-test.otf` CID/FDSelect) that exercise
    `cff_make_private_dict`; all 10 round-trip payloads stable; issue #1's
    large-lookup regression test green; `compare-log-output.sh` green.

- **`font/caryll_sfnt.rs`'s `Packet`/`PacketPiece` finally convert to `Vec`,
  and `Packet` drops `Copy` -- the one item the full-crate audit's Vec-
  conversion list left unfinished, split out of the twelve-item bundle above
  because of its size.** `SplineFontContainer.offsets`/`.packets`,
  `Packet.pieces`, and `PacketPiece.data` were all still `__caryll_allocate_
  clean`'d/`free`'d raw arrays; `PacketPiece.data` in particular is sized
  straight from the SFNT table directory's per-table `length` field --
  untrusted font bytes, the same risk class `CffIndex`/`CffDict` closed.
  `Packet` used to derive `Copy` purely so it could be passed by value
  through borrow-checker friction it didn't actually need: every one of the
  ~30 `table/*.rs` parsers, and `otf_reader.rs`'s `otfcc_read_sfnt` (which
  reuses one `packet` across ~20 sequential calls), only ever *read* it.
  Every one of those call sites now takes `&Packet` instead -- a `Copy`
  struct standing in for shared, uncounted, manually-freed heap data was
  the tell that the ownership model didn't match what the code actually
  did with it.
  - `SplineFontContainer.offsets: Vec<u32>`, `.packets: Vec<Packet>`;
    `Packet.pieces: Vec<PacketPiece>`; `PacketPiece.data: Vec<u8>`. None of
    the four still derive `Copy` (`PacketPiece`/`Packet` drop the derive
    outright; `SplineFontContainer` was never copied as a whole).
  - `otfcc_read_sfnt` becomes `Box::into_raw(Box::new(SplineFontContainer
    { .. }))` instead of `__caryll_allocate_clean`; `otfcc_delete_sfnt`
    collapses to a single `Box::from_raw` drop -- no per-packet, per-piece
    manual free loop needed any more, `Vec`'s own drop glue reaches every
    level.
  - `otfcc_read_packets` (the SFNT-table-directory parse loop, the one
    building `PacketPiece.data` from untrusted `length` values) preserves
    one exact quirk from the original C rather than "fixing" it: the
    per-table byte-reading pass is bounded by `packets[0].num_tables`, not
    the current packet's own `num_tables` -- harmless for ordinary single-
    SFNT files (where packet 0 is the only packet) but a real difference
    for TTC collections with differently-sized packets. This is a
    mechanical ownership conversion, not a behavior change, so the quirk
    ships unchanged (called out with an inline comment at the site).
  - Every `table/*.rs` parser's signature changes `packet: Packet` to
    `packet: &Packet`; the internal copy-out `let mut table: PacketPiece =
    *packet.pieces.offset(N as isize);` becomes a borrow, `let table:
    &PacketPiece = &packet.pieces[N as usize];`; every subsequent
    `table.data as FontFilePointer` (and any further `.data.offset(...)`
    chain) gains a `.as_ptr()` first, same shape as every other `Vec`-
    reading conversion this crate has made. `table/glyf/read.rs`'s and
    `otf_reader.rs`'s inline copies of the same pattern (not behind a named
    parser function) convert identically.
  - `table/_tsi.rs`'s `otfcc_read_tsi` needed a real restructure, not just
    field-type mechanics: `text_part`/`index_part` were plain `PacketPiece`
    values with a tag-0 sentinel standing in for "no matching table found
    yet," reassigned by value once a scan turned up a match. With
    `PacketPiece` no longer `Copy`, these become `Option<&PacketPiece>` --
    `None` is the same "not found" signal the tag-0 sentinel used to carry,
    and a found match is a borrow, not a copy.
  - `Packet.pieces.offset()`'s only two remaining raw-pointer-arithmetic
    consumers -- `otfcc_read_sfnt`'s allocation-shape parity comment and the
    `packet_0_num_tables` quirk above -- route through the established
    `let field = &(*ptr).field;` local-binding idiom to dodge the
    `dangerous_implicit_autorefs` lint on indexing through a raw-pointer
    dereference, same as every other struct-behind-a-raw-pointer conversion
    this crate has made.
  - `bin/otfccdump.rs` needed no changes at all: it only ever holds
    `*mut SplineFontContainer` as an opaque handle across the
    `otfcc_read_sfnt`/`otfcc_delete_sfnt` boundary, never touching `Packet`/
    `PacketPiece` directly.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib, with particular attention to
    `meta-test.ttf`/`vdmx-test.ttf` (the two payloads whose parsers needed
    the extra `.data.offset()` fixups beyond the mechanical sweep); all 10
    round-trip payloads stable; issue #1's large-lookup regression test
    green; `compare-log-output.sh` green.

- **Twelve more malloc'd scratch/output arrays convert to `Vec`, bundled into
  one PR -- the medium-priority half of the full-crate audit's Vec-conversion
  list, everything left except `CffIndex`/`CffDict` (PR #180, the highest-
  priority item, shipped separately) and `caryll_sfnt.rs`'s `Packet`/
  `PacketPiece` (investigated and split out into its own future PR -- see
  below).** Every item is a self-contained local scratch buffer or an
  output array with no external touch points beyond its own file (confirmed
  by grep before converting each one), so this is a wide but shallow sweep:
  same "calloc a fixed-size buffer up front, fill it, free it at the end"
  shape throughout, all converting to the same `Vec::with_capacity`/`vec![]`
  + indexing (or `.push()`) pattern already established by every prior unit.
  - `libcff/charstring_il.rs`: `CffCharstringIl.instr`/`.length`/`.free`
    collapse into a single `Vec<CffCharstringInstruction>` field -- the
    hand-rolled 256-instruction-block growth (`ensure_there_is_space`) is
    exactly what `Vec::push` already does, so the three `il_push_*` helpers
    shrink to one push call each. The ~50 `.offset()` read/mutate sites
    across `zroll`/`opop_roll`/`hvlineto_roll`/`hvvhcurve_roll`/
    `hhvvcurve_roll`/`il_matchtype`/`il_matchop`/`nextstop` (plus `subr.rs`'s
    `cff_insert_il_to_graph`) keep their exact shape via `.as_mut_ptr()`,
    same idiom as `CffStack`/`cff_parser.rs`. `cff_build_il`/`cff_shrink_il`/
    `cff_i_lmerge_il`/`cff_il_equal`/`instruction_eq` are converted too even
    though grep confirms zero call sites for any of them (dead since before
    this migration) -- kept compiling rather than deleted, since removing
    live-looking code is out of scope for a mechanical Vec sweep.
  - `bk/bkgraph.rs`: `compute_block_offsets`'s return value becomes
    `Vec<usize>`, with `getoffset`/`getoffset_untangle`/`otfcc_build_bkblock`/
    `try_untabgle_block` taking `&[usize]` instead of `*mut usize` -- all
    four only ever read it.
  - `support/unicodeconv.rs`: `utf8toutf16be` returns `Vec<u8>` instead of a
    `malloc`'d `*mut u8` plus an `out_bytes` out-param, matching its sibling
    `utf16be_to_utf8`'s shape (converted several units ago); its one caller
    (`table/name.rs`) drops the matching `free`.
  - `consolidate.rs`: `consolidate_glyph_hints`'s `hmap`/`vmap` permutation
    scratch arrays.
  - `otf_reader/unconsolidate.rs`: `merge_vmtx`'s `vorgs` becomes
    `Option<Vec<Pos>>` (the `Option` replacing the old null-pointer-as-
    "vorg table absent" signal), same shape as the already-converted
    `stat_glyf` frequency array.
  - `support/ttinstr.rs`: only `InstrData.bts` converts, *not* `.instrs` --
    `.bts` is allocated, filled (`instr_typify`), and freed entirely within
    this file, but `.instrs` is always a borrowed alias into a caller-owned
    buffer (`Glyph.instructions`/`FpgmPrepTable.bytes`), and those two
    fields are themselves a deliberate Stage 6-4 "outer struct Box'd, inner
    array stays a manually freed raw pointer" case (documented above,
    "Post-transpile fixups"). Converting `.instrs` would mean changing what
    `Glyph.instructions` is, which is out of scope here. Also deleted a
    `memset`-then-immediately-overwrite-with-a-real-literal dead line in
    `dump_ttinstr` that would have been unsound once `InstrData` owns a
    `Vec` (zeroing over a live `Vec`'s pointer/len/cap is UB, even though
    the values it zeroed were already about to be set to the same thing by
    the surrounding struct literal).
  - `table/glyf/build.rs`: `otfcc_build_glyf`'s `loca` offset table.
  - `table/otl/subtables/chaining/build.rs`: `otfcc_build_chaining_classes`'s
    `rcpg` class-population-count scratch, converted identically in both of
    its two near-duplicate copies (`otfcc_build_chaining_classes` proper and
    the equivalent block in `otfcc_build_contextual`'s classified branch).
  - `table/base.rs`: `axis_to_bk`'s `BaseTagList.items` -- was a hand-rolled
    "linear-search for an existing tag, grow-by-one-and-append if not
    found" loop, now `Vec::contains`/`Vec::push`; the `qsort`+`by_tag`
    comparator (now with zero remaining callers, deleted) becomes
    `Vec::sort` since ordinary integer ordering needs no custom comparator.
  - `libcff/cff_codecs.rs`: `cff_encode_cff_float`'s BCD nibble-array
    scratch.
  - `table/cff.rs`: `cffstrings_to_indexblob`'s `blobs` pointer array (now
    `Vec<*mut Buffer>` -- dropping it still only deallocates the pointer
    array itself, never the `Buffer`s it points to, matching the original
    `free`'s exact scope) and `writecff_cid_keyed`'s three CID-FDArray
    bookkeeping arrays (`starting_position_of_privates`,
    `ending_position_of_privates`, `fd_array_privates`). (`cffdict_input_
    doubles`/`_ints`/`_array` were already converted in the `CffDict` PR
    above.)
  - `table/tsi5.rs`: `otfcc_build_tsi5`'s `tsi5cls` per-glyph class table.
  - **Investigated but split out, not included here: `font/caryll_sfnt.rs`'s
    `SplineFontContainer.offsets`/`.packets`/`Packet.pieces`/`PacketPiece.
    data`.** This one is a different shape from the twelve above: `Packet`
    is passed *by value* to all ~33 `table/*.rs` parse functions (relying on
    its `Copy` derive), reused across ~20 sequential calls in
    `otf_reader.rs`'s `otfcc_read_sfnt`. Converting `Packet.pieces`/
    `PacketPiece.data` to `Vec` loses `Copy`, which would require changing
    every one of those ~33 function signatures to `&Packet` and every call
    site to match, plus a more involved rewrite of `table/_tsi.rs`'s
    found/not-found placeholder pattern (`text_part`/`index_part` currently
    reassigned by value, would need `Option<&PacketPiece>` instead). Scope
    comparable to or larger than `CffIndex`/`CffDict`, so left for its own
    dedicated PR rather than folded into this bundle.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib; all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green;
    `compare-log-output.sh` green.

- **`CffIndex`/`CffDict` convert their malloc'd arrays to `Vec` -- the two
  remaining "counted array sized from untrusted CFF bytes" structures the
  full-crate audit flagged, same risk class `CffStack` closed for the
  Type 2 CharString interpreter's operand stack.** `CffIndex.offset`/`.data`
  are sized from a font-byte-derived `count` in `extract_index` (the parse
  path, hit by every CFF-flavored font); `CffDictEntry.vals`/`CffDict.ents`
  are sized from operand/entry counts read out of untrusted DICT bytes in
  `parse_dict`. Both were correctly paired malloc/free before, but a
  counting mistake anywhere in the parse logic would have been an immediate
  OOB write -- `Vec` removes that risk class structurally, the same
  motivation as every prior "untrusted-input malloc array" conversion this
  crate has made. `CffIndexCountType`'s implicit off-count and
  `CffDictEntry.cnt`/`CffDict.count` are dropped entirely as write-only
  duplicates of `.len()`, matching the `CffStack.max` precedent.
  - `cff_index_copy`/`cff_dict_copy` (both `memcpy`-based, confirmed by grep
    to have zero call sites beyond their own vtable static initializer)
    become `unreachable!()` -- a bitwise copy of owned `Vec` fields would be
    an immediate double-free, same treatment as `subtable_gpos_pair_copy`
    and the other confirmed-dead vtable copy slots.
  - `cff_index_init`/`cff_dict_init` (zero-init vtable slots for a stack-
    local scratch `CffIndex`/`CffDict`) become `ptr::write` of an explicit
    empty value -- no all-zero bit pattern is a valid `Vec`-owning struct
    any more, same shape as `otl_init_chaining`.
  - `CffIndex`/`CffDict` lose their `Copy` derive along with the raw
    pointers, so `cff_parse_outline`, `cff_parse_subr`, and
    `sdsget_cff_sid` -- all previously taking a `CffIndex` by value purely
    to read from it recursively (subroutine calls) or repeatedly (charstring
    interpretation) -- switch to `&CffIndex`/`&CffDict` parameters, with
    call sites updated to pass a reference instead of a copy.
  - `libcff/cff_parser.rs`'s many `.offset.offset(N)`/`.data.offset(N)` raw-
    pointer-arithmetic chains keep their exact existing shape, just reading
    through `.offset.as_ptr()`/`.data.as_ptr()` first -- the same "leave the
    body untouched, convert only the allocation site" idiom `CffStack` used,
    applied here with a verified perl multi-line substitution instead of
    line-by-line editing (26 and 4 sites respectively, match count checked
    against plain `grep` before applying).
  - `new_index_by_callback` (the compile/build-side `CffIndex` constructor,
    growth-with-slack) keeps `offset`/`data` as local `Vec`s for the entire
    loop, `.resize()`-ing in place of `__caryll_reallocate` and
    `.truncate()`-ing unused slack at the end, only moving them into the
    struct once fully built -- avoiding the `dangerous_implicit_autorefs`
    lint by construction rather than patching around it, same as
    `extract_index`'s parse-side counterpart.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib, with extra attention to the
    CFF-heaviest payloads (`KRName-Regular-O2.otf` subroutinize,
    `cid-fdselect-test.otf` CID/FDSelect); all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green;
    `compare-log-output.sh` green.

- **Three more tag+union pairs become enums, bundled into one PR:
  `VqSegment`/`VqSegmentValue`, `BkCell`/`BkCellValue`, and
  `CffCharstringInstruction`/`CffCharstringArgument`.** The last of the
  union→enum candidates the full-crate audit turned up, closing out that
  theme. Unlike `ChainingSubtable`, none of these three own heap data --
  every union arm is `Copy` (`Pos`/`f64`, `i32`, `u32`, a borrowed raw
  pointer) -- so all three new enums stay `Copy` too, with no `Drop`/
  ownership bookkeeping at all.
  - `VqSegment` (`vf/vq.rs`, ~20 call sites plus 3 in `table/fvar.rs`/
    `table/glyf/read.rs`/`otf_reader/unconsolidate.rs`) becomes `enum
    VqSegment { Still(Pos), Delta(VqSegmentDelta) }`. `hash_vqs`
    (`unconsolidate.rs`) writes the variant as a byte into the glyph hash,
    byte-for-byte -- now via an explicit `discriminant_byte()` method (a
    plain `as` cast can't extract a discriminant from a data-carrying
    variant), pinned 0/1 by the existing `vqsegtype_discriminants_are_the_
    hashed_values` test, rewritten to match. `copy_vq_segment` needed the
    most care: the original only copied `.quantity`/`.region` into an
    existing `Delta` slot, leaving `.touched`'s bits whatever they already
    were -- meaningful when overwriting an existing `Delta` (preserved
    exactly), undefined when the destination was fresh-`Still` (every real
    call site), where `false` now replaces the old uninitialized read with
    a defined, safe value. Traced every `.touched` reader to confirm this
    is safe: it does affect real output (`table/fvar.rs`'s `json_new_vq_
    segment` reads it for gvar's `"implicit"` JSON field), but only the
    values `table/glyf/read.rs`'s `apply_coords` sets explicitly -- the
    values flowing through `copy_vq_segment`'s "fresh `Still` dst" case
    never reach that dump path in practice, confirmed by the full pipeline
    (`gvar-test.ttf` byte-identical both directions on both platforms).
  - `BkCell` (`bk/bkblock.rs` + `bk/bkgraph.rs`, ~25 call sites) becomes
    `struct BkCell { t: BkCellType, value: BkCellValue }` with `enum
    BkCellValue { Int(u32), Ptr(*mut BkBlock) }`. Unlike the other two,
    `BkCellType`'s ten values don't map 1:1 onto the union's two arms
    (`B8`/`B16`/`B32` all share `.z`, six more variants all share `.p`), so
    `t` stays a separate field -- `bk_cell_is_pointer`'s existing `t >=
    BkCellType::P16` still decides which one a given `t` implies. This is
    the crate's central binary-serialization primitive (every table's
    build path funnels through it), so its exercise by the standard
    verification suite is about as thorough as this crate gets.
  - `CffCharstringInstruction` (`libcff/charstring_il.rs`, 39 call sites,
    plus 4 in `libcff/subr.rs`) becomes `struct { type_0: CffInstructionType,
    arity: Arity, arg: CffCharstringArgument }` with `enum
    CffCharstringArgument { D(f64), I(i32) }` -- `type_0`'s five values
    similarly don't map 1:1 (`Operand`/`PhantomOperand` share `.d`,
    the other three share `.i`), so it also stays separate. Found one real
    ordering bug while converting, not just a mechanical rename:
    `hvlineto_roll` computed its `checkdelta` value (reading `.i()`)
    *before* confirming the instruction at that position was actually an
    operator -- harmless under the old union (a stray read of the wrong
    arm's bits, discarded moments later since the surrounding condition
    never used it if the check failed) but a real panic under the new
    enum. Fixed by moving the read after the operator check that already
    guarded every other read in this function -- the value was always
    discarded on the failing path anyway, so this changes nothing
    observable, confirmed by both platforms' full pipelines (the panic
    reproduced on `cid-fdselect-test.otf` before the fix, and disappeared
    after with byte-identical output).
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib; all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green;
    `compare-log-output.sh` green.

- **`ChainingSubtable`/`ChainingBody` become a real enum -- the one place the
  `Subtable` union-to-enum migration (B-1/B-2/B-3) left a nested C-shaped
  union in place.** The full-crate audit that turned up the two cmap/COLR
  leaks also flagged this: `ChainingSubtable { type_0: ChainingType,
  c2rust_unnamed: union { rule: ManuallyDrop<ChainingRule>, c2rust_unnamed:
  ManuallyDrop<ChainingRuleSet> } }` is exactly the "tag fully determines the
  live union arm" shape already converted for `CffEncoding`/`CffCharset`/
  `CffFdSelect`/`Subtable` itself -- just one level deeper, inside
  `Subtable::Chaining`'s own payload. `Poly` and `Classified` both carry a
  `ChainingRuleSet` (they shared a union arm before too), so the new shape is
  `enum ChainingSubtable { Canonical(ChainingRule), Poly(ChainingRuleSet),
  Classified(ChainingRuleSet) }`; `ChainingType` itself is gone, replaced by
  the enum's own discriminant, with `chaining_is_classified`/
  `chaining_is_canonical` (`subtables/chaining/common.rs`) covering the two
  places that used to compare it directly.
  - `ChainingRuleSet.bc`/`.ic`/`.fc: *mut ClassDef` become `Option<Box<
    ClassDef>>`, matching `GposPairSubtable.first`/`.second` exactly -- both
    are populated only by `classifier.rs`'s later classification pass (the
    raw binary read always leaves a `Poly` ruleset's class defs `None`), and
    now self-drop with the rest of the struct.
  - This is a large, cross-file change (`table/otl.rs`'s type definitions,
    all of `table/otl/subtables/chaining/{common,parse,dump,build,
    classifier,read}.rs`, plus three call sites the initial grep survey
    missed and the compiler caught instead: `consolidate/otl/chaining.rs`,
    `otf_writer/stat.rs`, and `otf_reader/unconsolidate.rs`) but a
    mechanical one: every function keeps its existing `*mut`/`*const
    ChainingSubtable` signature and control flow untouched, with small
    shared accessors (`chaining_rule_mut`/`_const`, `chaining_ruleset_mut`/
    `_const`, `chaining_rule_mut_from_const` for `build.rs`'s const-to-mut
    cast sites) replacing the raw `&raw mut/const (*subtable).c2rust_unnamed
    .{rule,c2rust_unnamed}` field access one-for-one.
  - `otl_init_chaining`/`otl_dispose_chaining` (the vtable's `.init`/
    `.dispose`) shrink to a single `ptr::write`/`drop_in_place` each --
    no all-zero bit pattern is a valid `ChainingSubtable` (it owns `Vec`
    fields through every variant), so `.init` places a valid empty
    `Canonical` value directly instead of `memset`; `.dispose` just runs
    the enum's own `Drop`, which now correctly tears down whichever variant
    is live without the old tag-gated free logic (removed, along with the
    `close_rule` helper it needed). The vtable's `.copy` slot
    (`subtable_chaining_copy`) was already confirmed dead before this
    change (never called outside its own static initializer); its `memcpy`
    body is now `unreachable!()` instead of something that would be
    actively unsound over owned `Vec`/`Box` data if it were ever wired up.
  - `otf_reader/unconsolidate.rs`'s `unconsolidate_chaining` -- which
    explodes a `Poly` ruleset read from binary into one `Canonical`
    subtable per rule for JSON dump -- got simpler and lost a real,
    previously-flagged leak as a side effect: the pre-enum version's
    `Canonical` branch deliberately left the original subtable's outer
    allocation leaked (documented in its own comment) because working
    around `Subtable`'s `Drop` impl to move a union-embedded rule out
    safely wasn't practical with a union in the way. With a real enum,
    `Subtable::drop`'s move restriction still applies (can't pattern-match
    a value out of a `Drop` type), but `ChainingRule`/`ChainingRuleSet`
    themselves have no custom `Drop`, so `mem::take` through a `&mut`
    borrow moves the payload out cleanly, and the emptied original just
    drops normally at the end of the loop iteration -- no leak, no raw
    pointer surgery.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib. This exercises the change
    unusually thoroughly -- `NotoNastaliqUrdu-Regular.ttf` alone carries 41
    `gsub_chaining` and 1 `gpos_chaining` lookups round-tripped
    byte-identically, and all 10 `run-cycles.sh` payloads survive a
    dump-build-dump-build cycle stable, which exercises
    `classifier.rs`'s `Poly`-to-`Classified` promotion (the one path that
    constructs a brand-new `ChainingSubtable` value from scratch rather
    than just mutating an existing one) on every one of them; issue #1's
    large-lookup regression test green; `compare-log-output.sh` green.

- **Full audit of the `unsafe_op_in_unsafe_fn` burn-down, and two real memory
  leaks it turned up.** After fifteen units of converting specific dangerous
  ownership/union patterns, we asked whether anything had been missed --
  three parallel audits covered all 114 files still carrying the allow
  attribute (`table/`+`table/otl/`, `libcff/`, and everything else). The
  answer was yes, and not just style: `table/cmap.rs`'s
  `otfcc_build_cmap_format14` allocated a `0x110001`-entry `valid_selectors`
  array with `__caryll_allocate_clean` and never freed it on any path (~1.1MB
  per call); `table/colr.rs`'s `otfcc_read_colr` allocated `gids`/`colors`
  arrays sized to `num_layer_records`, used them to build the returned
  `ColrTable`, and likewise never freed them. Both are real leaks in code
  paths that run on every `cmap` format-14 build and every `COLR`-table
  parse, not just readability debt.
  - Both were converted to plain `Vec`s at the same call sites that used to
    hold the raw allocation -- `valid_selectors: Vec<bool> = vec![false;
    MAX_UNICODE as usize]`, `gids`/`colors` built with `Vec::with_capacity`
    + `.push()` in the same loop that used to write via `.offset(j)`. No
    `free()` call needed removing because there wasn't one to begin with;
    that absence was the bug. `table/colr.rs`'s now-unused
    `__caryll_allocate_clean` import was dropped.
  - The audit's other findings (a `ChainingSubtable`/`ChainingBody` tag+union
    that the `Subtable` enum conversion left one level too shallow, three
    more untouched tag+union pairs in `libcff/charstring_il.rs`,
    `bk/bkblock.rs`, and `vf/vq.rs`, and roughly a dozen mechanical
    "malloc+indexed-write array, never quite converted" sites of varying
    priority) are recorded for follow-up units; none of those are known bugs
    the way these two leaks were, so they're being picked off individually
    rather than bundled into this fix.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- `BungeeColor-
    Regular_colr_Windows.ttf` exercises `otfcc_read_colr` directly, and
    every payload with a `cmap` table exercises `otfcc_build_cmap_format14`;
    all 10 round-trip payloads stable; issue #1's large-lookup regression
    test green; `compare-log-output.sh` green. The existing byte-comparison
    pipeline can't detect a leak directly (output is unchanged either way),
    so the leak fix itself was verified by code inspection rather than by
    the automated pipeline -- noting this limitation explicitly rather than
    overstating what the byte-identical result proves here.

- **Fifteenth unit of the `unsafe_op_in_unsafe_fn` burn-down: `CffStack`'s
  operand array becomes a `Vec`.** `CffStack` is the Type 2 CharString
  interpreter's operand stack -- the structure `cff_parse_outline`
  (`libcff/cff_parser.rs`'s ~1000-line CharString bytecode interpreter,
  covering every arithmetic/stack operator plus all the curve and flex
  variants) reads and writes on nearly every operator. Unlike the recent
  small, self-contained units, this one is a large *mechanical* sweep: 220
  call sites across a single file, converted with the same "leave the body
  untouched, convert only the allocation site" idiom used for
  `stat_single_glyph`'s recursion and `subr.rs`'s `serialize_node_to_
  buffer` -- every `.stack.offset(N)` site became `.stack.as_mut_ptr()
  .offset(N)` verbatim, with no restructuring of the interpreter's control
  flow. The risk this posed was pure volume, not design difficulty; a
  smaller, more surgical diff wasn't available because every operator
  branch in `cff_parse_outline` touches the operand array somewhere.
  - The sweep was applied with `perl -0777 -pi -e 's/\.stack(\s*)\.offset\(/
    .stack.as_mut_ptr()$1.offset(/g'` (slurp mode, so `.stack` and
    `.offset(` split across lines -- common in this c2rust-generated file --
    are matched too), after confirming with `grep -c` that the substitution
    pattern and the plain `.stack` occurrence count matched exactly (220
    both ways), i.e. every `.stack` reference in the file is followed by
    `.offset(` with no other usage shape to miss.
  - `CffStack.max` -- a field set once at construction (`0x10000`, the
    same generous fixed capacity `__caryll_allocate_clean` used to
    pre-allocate) and never read anywhere in the interpreter -- is dropped
    entirely, exactly duplicating `stack.capacity()`.
  - Construction in `table/cff.rs`'s `build_outline` becomes a single
    `vec![CffValue { .. }; 0x10000]` in place of `__caryll_allocate_clean` +
    a separate `.max`/`.stack` assignment; disposal drops the matching
    `free(stack.stack ...)` call, since `Vec` self-drops.
  - `reverse_stack` (the `roll` operator's small helper) picks up the same
    `.as_mut_ptr()` treatment at its two sites; the `p1 < p2` pointer
    comparison between the two resulting `*mut CffValue`s is unaffected --
    `.as_mut_ptr()` still yields a raw pointer of the same type.
  - `CffValue`'s own internal union (`CffValueBody`) is untouched by this
    change -- only the *container* around `CffValue` moved from a raw
    allocation to `Vec`; the tagged-union-of-value-types representation
    inside each element is a separate, much larger concern out of scope
    here.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- every CFF-containing
    payload (`KRName-Regular.otf`, `KRName-Regular-O2.otf`'s subroutinize
    path, `cid-fdselect-test.otf`'s CID/FDSelect path) exercises
    `cff_parse_outline` directly, giving this specific change unusually
    high verification signal; all 10 round-trip payloads stable; issue #1's
    large-lookup regression test green; `compare-log-output.sh` green.

- **Fourteenth unit of the `unsafe_op_in_unsafe_fn` burn-down: `HmtxTable`/
  `VmtxTable`'s inner arrays become `Vec`s.** The cross-file theme the
  thirteenth unit (`otf_writer/stat.rs`'s local scratch buffers)
  deliberately left out: `HmtxTable.metrics`/`.left_side_bearing` and
  `VmtxTable.metrics`/`.top_side_bearing` were the last raw-pointer arrays
  standing after Stage 6-4's Box化 already wrapped both tables themselves
  in `Option<Box<>>`. Investigating turned out smaller than expected --
  neither table ever appears in JSON dump/parse (glyph-level metrics live
  on `Glyph.advance_width`/`.horizontal_origin` etc. instead; `HmtxTable`/
  `VmtxTable` exist purely as `hmtx`/`vmtx`-binary-serialization
  intermediates), so the whole theme closed in three files: `table/
  hmtx.rs`, `table/vmtx.rs`, and their two touch points elsewhere in the
  crate.
  - Both structs drop their custom `Drop` impl entirely -- `Vec`'s own
    drop glue now reaches both arrays, the same simplification `CffTable`
    itself reached at the end of Stage 6-4's Box化.
  - `otfcc_read_hmtx`/`otfcc_read_vmtx` (parse) build both `Vec`s directly
    with `.push()` instead of `__caryll_allocate_clean` + indexed writes.
    `otfcc_build_hmtx`/`otfcc_build_vmtx` (binary write) switch from
    `.offset(j)` to `metrics[j]`/`left_side_bearing[j]` indexing --
    confirmed safe by tracing `count_a`/`count_k` back through both the
    read-then-build and stat-then-build pipelines: both always match each
    `Vec`'s own length exactly, since every one of the three sites
    (`stat_hmtx`/`stat_vmtx`, `otfcc_read_hmtx`/`otfcc_read_vmtx`, and the
    caller in `otf_writer.rs` that recomputes `count_a`/`count_k` before
    calling `otfcc_build_hmtx`/`otfcc_build_vmtx`) derives them the same
    way, from `hhea.number_of_metrics`/`vhea.num_of_long_ver_metrics` and
    `maxp.num_glyphs`.
  - `otf_writer/stat.rs`'s `stat_hmtx`/`stat_vmtx` (construction) fill
    *both* `Vec`s from the same single loop over every glyph -- glyphs
    before the split point push onto `metrics`, glyphs after it push onto
    `left_side_bearing`/`top_side_bearing` -- so the pre-sized, index-
    written arrays become two plain `Vec`s built with conditional
    `.push()`, no pre-sizing needed.
  - `otf_reader/unconsolidate.rs`'s `merge_hmtx`/`merge_vmtx` -- found only
    by the compiler, not by the initial grep survey (they reach the fields
    through `(*font).hmtx.take().unwrap()`'s inferred type rather than
    spelling `HmtxTable` anywhere in the file) -- switch their `.offset(j)`
    reads to the same `metrics[j]` indexing.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- every payload with a
    `glyf` table exercises `otfcc_read_hmtx`/`otfcc_read_vmtx`,
    `merge_hmtx`/`merge_vmtx`, `stat_hmtx`/`stat_vmtx`, and `otfcc_build_
    hmtx`/`otfcc_build_vmtx` on both the dump and build paths; all 10
    round-trip payloads stable; issue #1's large-lookup regression test
    green; `compare-log-output.sh` green.

- **Thirteenth unit of the `unsafe_op_in_unsafe_fn` burn-down: `otf_writer/
  stat.rs`'s local scratch buffers become `Vec`s.** `stat.rs` (the pass
  that computes derived font statistics -- bounding boxes, metrics,
  histograms -- before the binary tables get built) had three
  `__caryll_allocate_clean`/`free` scratch buffers that never escape the
  function that allocates them:
  - `stat_glyf`'s `stated: *mut StatStatus`, a per-glyph cycle-detection
    array threaded through `stat_single_glyph`'s composite-glyph recursion
    (`stat_single_glyph` itself keeps its `*mut StatStatus` parameter type
    unchanged -- including the recursive call passing it straight
    through -- since only the allocation site needed to know the buffer
    moved to a `Vec`, matching the same "leave the raw-pointer body
    untouched, convert only where it's allocated/freed" idiom used for
    `subr.rs`'s `serialize_node_to_buffer` earlier in this migration).
  - `stat_cff_widths` and `stat_vorg` each build a fixed 4096-entry `u32`
    frequency histogram (advance-width and vertical-origin distributions,
    respectively) to find the most common value for `CFF`'s default/
    nominal width and `VORG`'s default vertical origin. Both become
    `vec![0u32; MAX_STAT_METRIC as usize]`.
  - Deliberately **not** touched: `HmtxTable`/`VmtxTable`/`VorgTable`/
    `LtshTable` themselves still hold raw-pointer arrays (`metrics`/
    `left_side_bearing`, `metrics`/`top_side_bearing`, `entries`, `y_pels`)
    that this same file constructs and hands off via `Some(Box::new(...))`.
    Those arrays *do* escape into a struct field, so converting them means
    touching each table's own `table/*.rs` (parse/dump) and wherever the
    binary table gets serialized -- a separate, larger, cross-file theme
    left for its own dedicated unit(s), not folded in here.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- every build-direction
    payload exercises `stat_glyf`/`stat_cff_widths`/`stat_vorg` directly,
    since `otfcc_stat_font` runs unconditionally before any binary table is
    written; all 10 round-trip payloads stable; issue #1's large-lookup
    regression test green; `compare-log-output.sh` green.

- **Twelfth unit of the `unsafe_op_in_unsafe_fn` burn-down: `CffFdSelect`
  becomes a tagged Rust enum.** The natural follow-up to `CffCharset`:
  `CffFdSelect` (`libcff/cff_fdselect.rs`) was the same `t`-discriminant-
  plus-`c2rust_unnamed`-union shape, one raw-pointer array per format
  (`f0`/`f3`) recording which font dict a CID font's glyphs draw their
  Private DICT from.
  - `CffFdSelect` is now `enum CffFdSelect { Unspecified, Format0(Vec<u8>),
    Format3 { range3: Vec<CffFdSelectRangeFormat3>, sentinel: u16 } }`, no
    `#[repr(C)]`. `s`/`nranges` are gone, replaced by `range3`'s own
    `.len()` -- but `sentinel` (the one-past-the-last glyph index Format3's
    final range extends to) stays as a named field, since unlike the
    counts it's a genuine data value with no `Vec` to derive it from.
  - `cff_extract_fd_select` returns by value instead of writing through an
    out-param, and `cff_build_fd_select` takes `&CffFdSelect` rather than
    by value, both for the same reasons as their `CffCharset` equivalents.
    `cff_parse_subr` (`cff_parser.rs`) needed the same by-value-to-borrow
    switch for a sharper reason than style: its two call sites in `table/
    cff.rs`'s `build_outline` both read `(*f).fdselect` off a `CffFile`
    that's shared across every glyph in the font, called once per glyph --
    taking `CffFdSelect` by value would have moved it out on the *first*
    glyph, leaving every subsequent glyph's lookup reading a moved-from
    value. Passing `&(*f).fdselect` sidesteps the question entirely.
    `cff_close_fd_select` is deleted, matching `cff_close_charset`.
  - `table/cff.rs`'s builder (`cff_make_fdselect`) had the most interesting
    wrinkle: its `.s` field served two roles across the same function --
    first a live write cursor advanced through the glyph-scanning loop
    (`(*fds).s = (*fds).s.wrapping_add(1)`, indexing into `range3` as it
    went), then overwritten with the final range count right after. Since
    neither role survives past this one function, both collapse into a
    single sequential `Vec::push()` per format transition -- which also
    let the function drop its separate first pass that pre-counted ranges
    just to size the old `__caryll_allocate_clean` allocation, the same
    "`Vec` absorbs the counting pass" simplification already used for
    `Coverage`/`ClassDef`/`gpos_pair.rs`.
  - **Known gap, called out explicitly rather than silently accepted**: no
    CID-keyed CFF payload exists anywhere in this repo's test
    infrastructure (`tests/payload/`, golden fixtures, or the synthetic-
    payload scripts) -- so `CffFdSelect::Format3`, the only variant with
    real internal structure, is never exercised by `compare-with-c.sh` or
    any other verification script in this migration. `FDArrayTest257.otf`/
    `FDArrayTest65535.otf` sit in `tests/payload/` unreferenced by any
    script and were confirmed (before this PR's changes, against unmodified
    `origin/master`) to already segfault identically to the pre-existing,
    already-excluded `Cormorant-Medium.otf`/`WorkSans-Regular.otf` crashes
    -- a separate, unrelated bug, not something this PR could have
    introduced or fixed. Confidence in this conversion instead comes from
    the diff itself: every arithmetic expression, byte offset, and div/mod
    split in `cff_build_fd_select`'s Format3 branch and `cff_extract_fd_
    select`'s Format3 parsing carried over unchanged, only the storage
    mechanism (raw pointer offsetting -> `Vec` push/iterate/index) moved.
    A CID test payload remains a real, open gap in this crate's
    verification coverage -- worth fixing independently of any specific
    union conversion, either by sourcing a small CID `.otf` or hand-
    authoring a minimal CID JSON fixture for `otfccbuild`.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib; all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green; `compare-log-
    output.sh` green. (`CffFdSelect::Format0` -- the non-CID-count path --
    and the `Unspecified` variant are exercised by every non-CID payload
    above; `Format3` is not, per the gap noted above.)

- **Eleventh unit of the `unsafe_op_in_unsafe_fn` burn-down: `CffCharset`
  becomes a tagged Rust enum.** Investigating `table/cff.rs`'s remaining
  themes (the realloc-grown CharString interpreter stack) turned up a
  different, better-scoped candidate first: `CffCharset` (`libcff/cff_
  charset.rs`) was a `t: CffCharsetType` discriminant plus a `c2rust_
  unnamed: CffCharsetBody` union (`f0`/`f1`/`f2`, one raw-pointer array each
  for the CFF Top DICT's three possible Charset representations -- a
  glyph-SID-per-glyph table, or one of two range-based encodings) -- the
  exact same shape `CffEncoding` had (PR #167), and the same fix applies
  verbatim. (The interpreter stack itself was ruled out: it's a single
  `CffStack` value in `table/cff.rs`, but the hundreds of `.stack.offset(j)`
  sites that actually push/pop/index it live in `cff_parser.rs`'s ~2800-line
  CharString VM -- far outside this burn-down's usual unit size, and left
  for its own dedicated effort if ever attempted.)
  - `CffCharset` is now `enum CffCharset { IsoAdobe, Expert, ExpertSubset,
    Format0(Vec<u16>), Format1(Vec<CffCharsetRangeFormat1>),
    Format2(Vec<CffCharsetRangeFormat2>) }`, no `#[repr(C)]` (same
    reasoning as `CffEncoding`: never crosses the crate's four real FFI
    symbols). `s` (the entry count) is gone, replaced by each `Vec`'s own
    `.len()` -- write-only, like `CffEncoding`'s `format`/`ncodes`/etc.
    `CffCharsetType`'s double duty (both the enum discriminant *and* the
    three magic Top-DICT-offset sentinels 0/1/2 for the predefined
    charsets) is split the same way `CffEncodingType` was: the discriminant
    role disappears into the new enum, and the offset-sentinel role becomes
    three named consts (`CFF_CHARSET_OFFSET_ISO_ADOBE`/`_EXPERT`/
    `_EXPERT_SUBSET`), mirroring `CFF_STANDARD_ENCODING_OFFSET`/`CFF_EXPERT_
    ENCODING_OFFSET`.
  - `cff_extract_charset` returns `CffCharset` by value instead of writing
    through a `*mut CffCharset` out-param (the usual "unwrap_X_table"
    shape), and drops `extern "C"` for the same reason `parse_encoding` did
    -- a data-carrying enum has no C spelling, and the function is only
    ever called from within `cff_parser.rs`. `cff_build_charset` -- which
    only ever reads the charset to serialize it, never mutates or frees
    anything -- switches from taking `CffCharset` by value to `&CffCharset`,
    sidestepping the question of moving a non-`Copy` value out of a raw
    pointer deref entirely. `cff_close_charset` is deleted outright: once
    disposal is just "drop the owned `Vec`", the two calling sites became a
    plain reassignment (`(*file).charsets = CffCharset::IsoAdobe;` in
    `cff_parser.rs`'s `cff_close`, matching the `CffEncoding`/`GlyphOrder`
    "reset the field to run Drop glue before the struct's own bare `free()`"
    pattern) or disappeared into the new by-value construction (`table/
    cff.rs`'s `cff_make_charset`, the builder).
  - `table/cff.rs`'s `name_glyphs_according_to_cff` -- the largest
    consumption site, two symmetric six-arm matches (CID vs. non-CID glyph
    naming) reading `.s`/`.c2rust_unnamed.fN...` through a `*mut CffCharset`
    -- converts to matching `&*charset` directly, each arm binding the
    variant's owned `Vec` by reference and iterating it instead of indexing
    by hand.
  - A test in `cff_charset.rs` pinning `CFF_CHARSET_UNSPECED` and
    `CffCharsetType::IsoAdobe` as "the same state under two names" is
    deleted along with both those items -- the premise (Rust can't give one
    value two variant names) is exactly what the enum conversion resolves
    directly, so there is nothing left to pin.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 54 unit tests green (0 warnings under
    `warnings = "deny"`, one fewer than before from the deleted now-moot
    test) -- every standard payload byte-identical in both directions
    including the `otfccdll` cdylib -- `KRName-Regular.otf`/`KRName-
    Regular-O2.otf` exercise `CffCharset` directly on both the parse and
    build sides; all 10 round-trip payloads stable; issue #1's large-lookup
    regression test green; `compare-log-output.sh` green.

- **Tenth unit of the `unsafe_op_in_unsafe_fn` burn-down: `table/cff.rs`'s
  `CffPrivateDict` hint arrays become `Vec<f64>`.** `table/cff.rs` was
  flagged in the original survey as too large and heterogeneous for a single
  PR (~3000 lines, ~48 malloc/free-ish sites spanning a realloc-grown
  CharString interpreter stack, a 3-way union charset/FdSelect builder, an
  FDArray-private-dict-patching loop, and more) -- this unit picks the one
  piece that was already flagged, in a comment left by an earlier PR, as the
  clean next step: `CffPrivateDict`'s six parallel `(count: Arity, *mut
  f64)` pairs (`blueValues`/`otherBlues`/`familyBlues`/`familyOtherBlues`/
  `stemSnapH`/`stemSnapV`, the CFF Private DICT's hint-replacement delta
  arrays) become six plain `Vec<f64>` fields, with the `_count` fields
  dropped entirely (replaced by each `Vec`'s own `.len()`).
  - The custom `impl Drop for CffPrivateDict` (which freed each of the six
    arrays by hand) is deleted outright -- `Vec`'s own drop glue now reaches
    every allocation, the same shape `CffTable` itself reached at the end of
    the earlier Box化 PR that left these six fields as the one remaining
    raw-pointer exception.
  - Three call sites needed matching signature changes, all in this same
    file: `callback_extract_private`'s six DICT-operator match arms (parse
    side) build each `Vec` directly with `(0..top).map(|j| cffnum(*stack.
    offset(j))).collect()` instead of an `__caryll_allocate_clean`+loop
    pair; `pd_delta_to_json`/`pd_delta_from_json` (dump/parse-from-JSON)
    drop their `count`/out-param parameters for a plain `&[f64]` borrow and
    a `Vec<f64>` return value respectively, matching the "unwrap_X_table"
    return-by-value shape used throughout this migration; `cffdict_input_
    array` (DICT-entry serialization) drops its `arity: Arity` parameter for
    the slice's own `.len()`, joining `cffdict_input_doubles`/`cffdict_
    input_ints` (already slice-taking from an earlier PR) as the third and
    last `cffdict_input_*` helper to make that switch.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- `KRName-Regular.otf`/
    `KRName-Regular-O2.otf` in particular exercise `CffPrivateDict` directly
    on both the parse and build sides; all 10 round-trip payloads stable;
    issue #1's large-lookup regression test green; `compare-log-output.sh`
    green.

- **Ninth unit of the `unsafe_op_in_unsafe_fn` burn-down: `libcff/subr.rs`'s
  `char_strings`/`gsubrs`/`lsubrs` scratch arrays become `Vec<Buffer>`.**
  `subr.rs` implements CFF subroutinization (the Larsson-Moffat-style
  dictionary-compression pass that finds repeated charstring fragments and
  factors them into callable subroutines) as an intrusive doubly-linked-list
  graph (`CffSubrRule`/`CffSubrNode`, joined via a circular sentinel `guard`
  node per rule, cross-referenced by raw pointers with manual refcounting).
  That graph is *not* part of this PR -- unlike `Subtable` or `BkGraph`, an
  intrusive doubly-linked list with back-pointers and reference counting has
  no established safe-Rust shape in this migration (arena+index, the pattern
  used for `GlyphOrder`'s aliased pointers, doesn't fit a structure that's
  actively spliced/merged/substituted node-by-node throughout the algorithm)
  and forcing one in for its own sake would be a much larger, much riskier
  PR than this burn-down's usual unit. What *is* self-contained: `cff_il_
  graph_to_buffers`, the function that turns the finished graph into output
  bytes, allocates three `__caryll_allocate_clean`'d arrays of `Buffer`
  structs (not pointers -- the structs themselves, written into via
  `serialize_node_to_buffer`, then read back through `CFF_I_INDEX.from_
  callback`), each freed field-by-field (every `.data`) and then as a whole
  -- the same "malloc'd scratch array of plain structs" shape already
  converted to `Vec` for `table/glyf/read.rs`'s six scratch buffers earlier
  in this migration. `Buffer` is `Copy`/`repr(C)`, and its zeroed state is
  exactly what `bufnew()` itself produces (`__caryll_allocate_clean` zeroes,
  then `bufnew` only re-asserts `free`/`size` are 0), so `vec![zero_buffer;
  n]` starts every slot in the same state a freshly-`bufnew`'d buffer would.
  `serialize_node_to_buffer` itself (and the graph functions it walks) keeps
  its `*mut Buffer` signature unchanged -- callers now pass `.as_mut_ptr()`
  instead of the raw allocation, so the graph-side code doesn't need to know
  its scratch buffers moved. The three manual `free(array)` calls at the end
  are simply gone; `Vec`'s own drop glue covers them once every `.data` has
  already been freed by the existing per-entry loops (unchanged apart from
  being `Vec` iteration instead of pointer offsetting).
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- `KRName-Regular-O2.otf`
    (CFF subroutinize) in particular exercises `cff_il_graph_to_buffers`
    directly; all 10 round-trip payloads stable; issue #1's large-lookup
    regression test green; `compare-log-output.sh` green.

- **Eighth unit of the `unsafe_op_in_unsafe_fn` burn-down: `bk/bkgraph.rs`'s
  `BkGraph.entries` becomes a `Vec`, replacing its hand-rolled
  realloc-with-slack growth scheme.** `BkGraph` is the block-deduplication/
  offset-resolution graph every table builder in the crate goes through
  (`bk_build_block` is called from `table/base.rs`, `table/colr.rs`,
  `table/cpal.rs`, `table/cmap.rs`, `table/gdef.rs`, `table/svg.rs`, every
  GSUB/GPOS subtable builder, and more) -- `entries: *mut BkGraphNode` was
  grown by hand via `_bkgraph_grow`, which tracked a separate `free: u32`
  slack count and called `__caryll_reallocate` whenever it ran out, doubling
  roughly like a manual `Vec` would but without `Vec`'s bookkeeping.
  - `BkGraph` is now `{ entries: Vec<BkGraphNode> }` -- `length`/`free` are
    both gone, replaced by `entries.len()` and `Vec`'s own amortized growth.
    `_bkgraph_grow`'s two call sites (`dfs_insert_cells`, `try_untabgle_
    block`) become plain `.push()`s of a fully-formed `BkGraphNode`, and the
    outer shell (`bk_new_graph_from_root_block`'s allocation, `bk_delete_
    graph`'s final free) moves from `__caryll_allocate_clean`/`free` to
    `Box::into_raw`/`Box::from_raw` -- the same "outer shell only" split
    already used for `CffIndex`/`CffDict`: `BkGraphNode.block: *mut BkBlock`
    stays a raw pointer, since `bk_delete_graph` still owns and frees the
    `BkBlock` tree the graph was built from, and converting *that* recursive
    structure is a separate, unscoped piece of work. `BkGraph` carries no
    `Copy`/`Clone`/`repr(C)` -- confirmed by grep that the only other file
    touching it (`gpos_pair.rs`) always goes through the `*mut BkGraph`
    `bk_new_graph_from_root_block` returns, never by value.
  - The two `qsort` comparators (`_by_height`, `_by_order`) become `sort_by`
    closures returning `Ordering`, the same conservative swap already made
    for `Coverage`/`ClassDef`/`gpos_pair.rs`'s own qsort-scratch-buffer
    conversions (`qsort` isn't guaranteed stable; `sort_by` is). Both
    comparators' final tiebreaker is each entry's insertion order, which is
    unique per entry, so this was already a strict total order with no ties
    for stability to matter on -- the swap is behavior-preserving either way.
  - Every other `(*f).length`/`(*f).entries.offset(j)` pair through the file
    becomes `(*f).entries.len()`/indexing. A newer rustc lint, `implicit_
    autoref` (autoref through a raw-pointer deref implicitly manufacturing a
    reference), rejects plain `(*f).entries[j]` bracket-indexing on a `Vec`
    field reached through `*mut BkGraph`; every such site is written as the
    compiler's own suggested fix, `(&(*f).entries)[j]` / `(&mut (*f).
    entries)[j]`, each a fresh, short-lived reborrow rather than one held
    across other calls -- deliberately, since holding a `&mut` across a call
    that (through the same raw pointer, from a different function) reborrows
    the same `Vec` would be exactly the kind of aliasing violation this
    lint-driven rewrite exists to avoid introducing.
  - **Caught by the verification pipeline, not by review**: the first
    version of this conversion inlined `_bkgraph_grow`'s two call sites
    incompletely and dropped `(*b)._visitstate = BkCellVisitState::Black;`
    from `dfs_insert_cells` -- the line that marks a `BkBlock` as already
    inserted into the graph. Every other payload stayed byte-identical (most
    fixtures' block graphs don't share blocks across parents), but
    `BungeeColor-Regular_colr_Windows.ttf` -- a COLR font whose per-glyph
    layer lists reuse the same blocks from multiple parents -- came out 40
    bytes larger, because shared blocks that should have been visited once
    and reused were instead being re-inserted as duplicates every time a new
    parent referenced them. `compare-with-c.sh` caught this immediately as a
    single non-byte-identical payload with a real size delta (not just a
    checksum/offset shuffle), which is what pointed at "fewer merges
    happened" rather than a purely cosmetic difference. Restoring the
    `_visitstate` write fixed it; recorded here since it's exactly the
    failure mode this crate's "always verify against a payload that
    exercises block sharing, not just tree-shaped output" pipeline exists to
    catch.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- `BungeeColor-Regular_colr_
    Windows.ttf` in particular exercises the block-sharing path this PR's
    regression lived in; all 10 round-trip payloads stable; issue #1's
    large-lookup regression test green; `compare-log-output.sh` green.

- **Seventh unit of the `unsafe_op_in_unsafe_fn` burn-down: `libcff/
  cff_parser.rs`'s `CffEncoding` union becomes a tagged Rust enum.**
  `CffEncoding` was a `t: CffEncodingType` discriminant plus a `c2rust_
  unnamed: CffEncodingBody` union (`f0`/`f1`/`ns`, one raw-pointer array
  each for the CFF Top DICT's three possible Encoding representations:
  a code-per-glyph table, a range table, or a supplement list layered on
  a predefined encoding) -- the same shape `Subtable` had, and the same
  fix: a single enum, discriminant and payload together, so the compiler
  enforces that only the payload matching the current variant is ever
  read.
  - `CffEncoding` is now `enum CffEncoding { Standard, Expert,
    Format0(Vec<u8>), Format1(Vec<CffEncodingRangeFormat1>),
    FormatSupplement(Vec<CffEncodingSupplement>), Unspecified }`, with no
    `#[repr(C)]` -- matching `Subtable`'s own precedent, since this type
    never crosses the crate's four real FFI symbols (`ffi/dll.rs`,
    verified with `check-abi.sh`). The `format`/`ncodes`/`nranges`/`nsup`
    fields that used to sit alongside each raw array are gone entirely:
    all four were write-only (set once while parsing, never read again
    anywhere in the crate) and exactly duplicated information the
    corresponding `Vec`'s own `.len()` (or the enum variant itself, for
    `format`) already carries.
  - `parse_encoding` used to fill an already-allocated `*mut CffEncoding`
    out-param; it now returns `CffEncoding` by value, the same
    "unwrap_X_table" shape used throughout this migration. It also drops
    `extern "C"`: a data-carrying enum has no C spelling
    (`improper_ctypes_definitions`), and the function is only ever called
    from within this same file, not part of the crate's public ABI --
    the same reasoning already applied to `bufninit` earlier in the
    migration.
  - `CffFile.encodings: CffEncoding` now owns `Vec`s on three of its six
    variants, but `CffFile` itself is still a manually `__caryll_
    allocate_clean`'d / raw-`free()`'d struct (not `Box`-managed) --
    so `cff_close` explicitly resets `(*file).encodings =
    CffEncoding::Unspecified` before the struct's own `free()`, running
    the field's Drop glue first. A bare `libc::free()` on the struct's
    memory would not otherwise invoke it -- the same pattern already
    established for `dispose_glyph_order`. `#[derive(Copy, Clone)]` was
    removed from `CffFile` accordingly, after confirming by grep that the
    struct is never used by value anywhere in the crate (always through
    `*mut CffFile`/`*const CffFile`) -- the derive was vestigial.
  - A broad `.c2rust_unnamed` grep across this file initially looked like
    a much larger task (300+ hits) before it became clear that most
    belonged to two unrelated unions sharing the same c2rust-generated
    field name in the same file: `CffValue`'s `i`/`d` union (used
    pervasively across ~2000 lines of charstring/DICT-value parsing) and
    `CffFdSelect`'s `f0`/`f3` union. Filtering to only `CffEncoding`-
    specific access patterns isolated the true scope at roughly 30 sites,
    matching the original survey's estimate for this theme.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings
    under `warnings = "deny"`); every standard payload byte-identical in
    both directions including the `otfccdll` cdylib; all 10 round-trip
    payloads stable; issue #1's large-lookup regression test green (run
    against `iosevka-r.ttf`, which has enough glyphs to exceed the 64KB
    subtable threshold); `compare-log-output.sh` green.

- **Sixth unit of the `unsafe_op_in_unsafe_fn` burn-down, batched at the
  user's request: `table/base.rs`, `table/otl/build.rs`, and `table/glyf/
  read.rs` in one PR.** Each had been surveyed and individually scoped as
  tractable but not yet started; rather than three separate PR cycles,
  they landed together.
  - **`table/base.rs`**: `BaseAxis`/`BaseScriptEntry`/`BaseValue`/
    `BaseTagList`'s remaining raw arrays (`entries`, `base_values`) become
    `Vec`s, and `BaseTable.horizontal`/`.vertical` become `Option<Box<
    BaseAxis>>` -- the layer inside the `horizontal`/`vertical` Box化 an
    earlier PR left as raw pointers. The custom `Drop for BaseTable` impl
    (and `delete_base_axis`, its only caller) is deleted entirely --
    `Option`/`Box`/`Vec`'s own drop glue reaches every allocation now.
    This closes a *documented* pre-existing leak by construction, not by
    an explicit fix: the earlier PR's comment on this file recorded that
    `delete_base_axis` never freed `axis` itself, only its `entries` --
    true in the original C too. A raw `*mut BaseAxis` freed by a hand-
    written dispose function can leak that way; a `Box<BaseAxis>` cannot,
    the same shape as the `otfccbuild.rs` use-after-free fixed by
    construction earlier in this migration. `read_base_script`/
    `base_script_from_json` (the two functions that used to fill a
    `*mut BaseScriptEntry` slot through an out-param, resetting it to
    empty on any of several validation failures) now return `(u32,
    Vec<BaseValue>)` by value instead -- every original failure branch
    reset the entry to the same `(0, empty)` state regardless of what had
    been partially written along the way, so the two representations
    agree on every observable outcome; this version just never writes
    the intermediate values that were always going to be discarded.
    Confirmed before starting that `BaseAxis`/`BaseScriptEntry`/
    `BaseValue`/`BaseTagList` are referenced nowhere outside this file.
  - **`table/otl/build.rs`**: the `subtables`/`subtable_quantity`/
    `prefer_ext_for_this_lut` triple of parallel, `__caryll_allocate_
    clean`'d arrays (one `*mut *mut *mut Buffer`, sized to `lookups.
    len()`, each slot itself another dynamically-sized array of per-
    lookup subtable buffers) become `Vec<Vec<*mut Buffer>>`/`Vec<TableId>`/
    `Vec<bool>`. `_declare_lookup_writer`/`_declare_lookup_writer_split`/
    `_build_lookup` take `&mut Vec<*mut Buffer>` instead of writing
    through a `*mut *mut *mut Buffer` out-param; Rust's implicit
    reborrowing lets `_build_lookup`'s dozen sequential, mutually-
    exclusive `_declare_lookup_writer(_split)` calls keep passing the
    same `&mut` reference without any explicit `&mut *x` at each site.
    `otfcc_classified_build_chaining` (`chaining/classifier.rs`, `_build_
    lookup`'s other, chaining-specific dispatch target -- its only call
    site anywhere in the crate) needed the identical signature change to
    stay consistent; confirmed no other caller before touching it.
  - **`table/glyf/read.rs`**: six independent malloc/free scratch
    buffers -- `flags`, `nudges`, `glyph_refs`, `delta_x`/`delta_y`,
    `offsets` -- become local `Vec`s, each dropping itself instead of
    needing a matching `free()` at every one of the function's several
    exit points (`offsets` alone had three). `fill_the_gaps`/
    `apply_coords` take `&mut [VqSegment]`/`&[*mut Point]` borrows now
    instead of raw pointers they never owned in the first place.
    `parse_point_numbers` returns `(FontFilePointer, Vec<ShapeId>)`
    instead of writing through two out-params (a data pointer and a
    count that is now just `.len()`). The trickiest piece was `point_
    indeces`/`shared_point_indeces`: a GVAR tuple's point-index array is
    a genuine borrowed-vs-owned split, not just a leftover raw pointer --
    it normally *borrows* `shared_point_indeces` (parsed once per glyph,
    outliving every tuple that reads it) and only *owns* a fresh
    allocation when that one tuple's `PRIVATE_POINT_NUMBERS` bit is set,
    freed within that same loop iteration rather than at the end. This
    is modeled directly with `std::borrow::Cow<[ShapeId]>` --
    `Cow::Borrowed(&shared_point_indeces)` in the common case, `Cow::
    Owned(private_point_numbers)` when private -- rather than force-
    fitting it into a single owned `Vec` (which would have meant either
    cloning the shared array on every tuple that doesn't need a private
    one, or leaving the aliasing as raw pointers and converting nothing).
  - **Explicitly not attempted**: `font/caryll_sfnt.rs`, the fourth file
    from the same round of surveying, turned out to need far more than
    its own malloc/free sites suggested. `Packet` (`SplineFontContainer`'s
    per-table-directory-entry struct, containing `pieces: *mut
    PacketPiece`) is `Copy` and passed *by value* into every single OTF
    table reader in the crate -- 30 files' `otfcc_read_*` entry points all
    take `packet: Packet` this way. Converting `pieces` to `Vec<
    PacketPiece>` would make `Packet` no longer `Copy`, which would in
    turn require changing all 30 signatures (and every call site that
    currently relies on the implicit copy) in the same PR -- an order of
    magnitude bigger than the "many access-site edits across ~200 lines"
    the original survey estimated for this file alone, discovered only
    by tracing where `Packet` actually flows rather than just counting
    this file's own allocation sites. Left for a dedicated future PR,
    not folded into this batch.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- `gvar-test.ttf`
    exercises the `point_indeces`/`Cow` design directly, `unknown-
    lookup.ttf` (51 GSUB lookups, 4 GPOS lookups forced to format 10)
    exercises `table/otl/build.rs`'s lookup-splitting logic, and `base-
    test.ttf` exercises `table/base.rs` directly; all 10 round-trip
    payloads stable; issue #1's large-lookup regression test green;
    `compare-log-output.sh` green.

- **Fifth unit of the `unsafe_op_in_unsafe_fn` burn-down: `CffIndex`'s and
  `CffDict`'s outer create/free shells become `Box::into_raw`/`Box::
  from_raw`, mirroring `Coverage`/`ClassDef` -- but their inner arrays
  (`CffIndex.offset`/`.data`, `CffDict.ents`/`CffDictEntry.vals`) are
  deliberately left as raw pointers this time.** Both files were next in
  the priority survey. Reading each closely to plan the full conversion
  found the same shape of problem `gpos_pair.rs`'s `subtable_from_raw`
  coupling did: `CffIndex.offset`/`.data` are read and written directly
  (not through any `cff_index.rs` function) by `table/cff.rs`'s
  `cff_compile_nameindex` and its FDArray-private-dict-patching loop, and
  `CffDict.ents`'s only growth function, `cffdict_givemeablank`, lives in
  `table/cff.rs` too, not `cff_dict.rs`. `table/cff.rs` was already
  flagged in the original survey as too large and heterogeneous for a
  single PR (a realloc-grown interpreter stack, a 3-way C union charset,
  several unrelated allocation shapes in one ~3000-line file) -- so
  converting either inner array now would mean reaching into that file
  as a side effect of what was meant to be two small, self-contained
  units, the same scope-creep this burn-down has consistently avoided.
  - What *is* self-contained: the outer shells. `cff_index_create`/
    `cff_dict_create` switch from `malloc` + a `memset`-based `_init`
    helper to `Box::new` of an explicit all-zero struct literal;
    `cff_index_free`/`cff_dict_free` switch from `free()` to `Box::
    from_raw` after the existing `_dispose` call (which still frees
    `offset`/`data`/`ents`/`vals` exactly as before -- only the shell's
    own allocator changed). Confirmed by grep before touching either:
    every `CFF_I_INDEX.create`/`.free` and `CFF_I_DICT.create`/`.free`
    call site pairs consistently through the same vtable functions, with
    no `subtable_from_raw`-style generic adapter reclaiming either type
    any other way.
  - `cff_index_init` survives even though `cff_index_create` no longer
    calls it: `CFF_I_INDEX.init` has one other call site (`table/cff.rs`,
    zero-initializing a stack-local `CffIndex` that was never a `malloc`/
    `Box` allocation to begin with) -- caught by checking every caller
    before deleting, not just the one this PR happened to be touching.
    `cff_dict_init` similarly stays defined (for `CFF_I_DICT.init`'s
    struct-literal slot), even though grepping found that slot itself has
    no call site anywhere -- the same "present but unreachable" shape as
    `subtable_gpos_pair_copy` elsewhere in this migration, not something
    this PR's narrow scope should go delete.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- `KRName-Regular-O2.otf`
    (CFF subroutinize) in particular exercises `CffIndex`/`CffDict`
    heavily; all 10 round-trip payloads stable; issue #1's large-lookup
    regression test green; `compare-log-output.sh` green.

- **Fourth unit of the `unsafe_op_in_unsafe_fn` burn-down: `GlyphOrder`'s
  dual `by_gid`/`by_name` index is redesigned as an arena + `usize`
  indices, replacing the individually-heap-allocated, raw-pointer-aliased
  `GlyphOrderEntry`s the uthash port left behind.** `support/glyph_order.
  rs` was next in the priority survey, flagged there as a genuine alias
  (both maps held `*mut GlyphOrderEntry` pointing at the same
  allocations, with `by_gid` informally treated as "the owner" for
  disposal purposes). Reading it closely to plan the fix found the
  aliasing was more than cosmetic: `json_reader.rs`'s `set_order_by_name`/
  `order_glyphs` pair shows a JSON-driven glyph-order entry can exist in
  `by_name` *alone* for a while, with a placeholder `gid`, before
  `order_glyphs` sorts and inserts it into `by_gid` with a real one -- so
  "`by_gid` always owns" (which the old disposal code, walking only
  `by_gid`, implicitly assumed) isn't reliably true partway through that
  process. An `Rc<RefCell<_>>` per entry would have worked but doesn't
  fit any existing idiom in this crate; the chosen fix instead gives
  `GlyphOrder` a third field, `entries: Vec<GlyphOrderEntry>`, as the
  single real owner (a plain growing arena nothing is ever removed from),
  with `by_gid`/`by_name` reduced to `BTreeMap<GlyphId, usize>`/
  `HashMap<Vec<u8>, usize>` -- non-owning indices into it, valid for
  `GlyphOrder`'s whole lifetime and immune to the dangling-pointer
  failure mode raw aliasing had, since a plain `usize` can't outlive what
  it doesn't own in the first place.
  - The custom `Drop for GlyphOrder` impl is deleted outright: `entries`'
    own `Vec` drop glue already frees every entry (and its `name: Vec<u8>`)
    on the way down, so there's nothing left for a hand-written impl to
    do. `dispose_glyph_order`'s manual `by_gid`-walk-and-`free()` loop
    collapses to three plain field reassignments for the same reason.
  - The raw-pointer leak reached three files beyond `glyph_order.rs`
    itself: `json_reader.rs` (`set_order_by_name`/`order_glyphs`, the
    transient by-name-only window described above), `table/post.rs`
    (`build_post_2_0`'s name-table walk), and `table/glyf.rs`
    (`otfcc_glyf_parse_glyph`'s lookup, read-only -- its `*mut
    GlyphOrderEntry` parameter becomes `&GlyphOrderEntry`). Every site
    already only ever read-and-cloned or freshly-inserted through an
    entry, never held one across an unrelated mutation, so swapping "raw
    pointer obtained from a map" for "index looked up, then indexed into
    `entries`" changed no observable behavior at any of them.
  - The outer `*mut GlyphOrder` shell itself (`otfcc_glyph_order_create`/
    `.free`, and `Font.glyph_order`'s separate direct-`Box::new`
    construction, already established before this PR) is untouched --
    this PR is purely about what a `GlyphOrder` owns internally, not how
    the struct itself is allocated.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- the `fj-*` payloads in
    particular build their glyph order entirely from JSON, exercising
    `set_order_by_name`/`order_glyphs`'s transient-entry window directly;
    all 10 round-trip payloads stable; issue #1's large-lookup regression
    test green; `compare-log-output.sh` green.

- **Third unit of the `unsafe_op_in_unsafe_fn` burn-down: `table/name.rs`'s
  `malloc` site wasn't a conversion target at all -- it, and the two
  functions upstream of it, turned out to be entirely dead code.**
  Investigating `table_name_create` (`NameTable`'s outer create shell,
  the next candidate after `gpos_pair.rs` per the original priority
  survey) to plan its `Box` conversion found instead that its only
  caller, `create_font_table`, is itself unreachable: grepping every read
  of `FontElementInterface`'s fields across the crate found `.create`,
  `.consolidate`, `.free`, and `.delete_table` all genuinely called, but
  `.create_table` -- the field `create_font_table` is assigned to in
  `OTFCC_I_FONT` -- never read anywhere. `table_otl_create` (`table/
  otl.rs`) was `create_font_table`'s other callee and dead for the exact
  same reason; a previous pass had already left a comment on it (in
  Japanese, unusually for this file) noting the same finding without
  acting on it. All three functions, `init_otl` (only called from
  `table_otl_create`), and the `create_table` field itself (removed from
  `FontElementInterface` and its one literal) are deleted rather than
  converted -- there was nothing here to give a `Box`, since nothing
  live was ever calling it in the first place.
  - `table/name.rs`'s other three `free()` sites (`base64_encode`,
    `utf8toutf16be`, `base64_decode`'s temporary buffers) are left as-is:
    each is a genuinely single-owner, immediately-consumed scratch buffer
    from a different file's C-shaped helper, with no aliasing and nothing
    blocking a future conversion -- just out of scope for a `table/
    name.rs`-focused PR, the same discipline as leaving `gpos_pair.rs`'s
    `subtable_from_raw` coupling for its own later unit.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib; all 10 round-trip
    payloads stable; issue #1's large-lookup regression test green;
    `compare-log-output.sh` green.

- **Second unit of the `unsafe_op_in_unsafe_fn` burn-down: `table/otl/
  subtables/gpos_pair.rs`'s two `qsort` scratch buffers become plain
  `Vec`s, same as `Coverage`/`ClassDef` -- but its outer subtable create/
  free shell is deliberately left untouched this time.** Picked as the
  natural follow-up to the `Coverage`/`ClassDef` PR since it directly
  consumes both of their raw-pointer APIs, but reading it closely
  surfaced a coupling the initial survey hadn't caught: unlike `Coverage`/
  `ClassDef`, `GposPairSubtable`'s create/free (`subtable_gpos_pair_
  create`/`subtable_gpos_pair_free`) isn't the only thing that ever
  reclaims that allocation -- every read/parse entry point across all 11
  `Subtable` variants also routes its freshly-`_create()`d value through
  one shared, generic `subtable_from_raw::<T>` (`table/otl.rs`), which
  does its own `ptr::read` + a bare `free()` before rewrapping the value
  into the `Subtable` enum. Converting `subtable_gpos_pair_create`/`_free`
  alone to `Box::into_raw`/`Box::from_raw` (the `Coverage`/`ClassDef`
  pattern) would leave `subtable_from_raw`'s `free()` call freeing a
  `Box`-allocated pointer for this one variant only -- the exact
  mismatched-allocator hazard that whole conversion exists to close off,
  and not one this PR can fix in isolation without touching `subtable_
  from_raw` and, in turn, the other ten variants' own `_create()`
  functions (all of which still allocate via plain `malloc`) in the same
  change. Left for a later, appropriately-scoped unit -- this PR only
  converts what's genuinely self-contained.
  - `otfcc_build_gpos_pair_individual`'s `pair_counts: *mut GlyphId`
    (`__caryll_allocate_clean`-zeroed, incremented, read, freed once)
    becomes `vec![0; ...]`. Its `pairs: *mut IndividualGposPair` (freshly
    allocated *inside* the per-coverage-glyph loop, filled, `qsort`'d,
    read, freed every iteration before the next one reallocates) becomes
    a `Vec` built with `.push()` and `.sort_by_key(...)` -- **stable**,
    the same deliberately conservative choice as the `Coverage`/
    `ClassDef` PR, since `qsort` itself carries no stability guarantee.
    No pre-sizing needed for `pairs`: the same predicate the counting
    pass used decides what gets pushed, so the `Vec` always ends up
    exactly `current_pair_count` entries long by construction, the same
    property that let `pair_counts` size it precisely before. Deletes the
    `by_pair_second_glyph` C-ABI comparator `qsort` needed, along with
    the now-unused `qsort`/`__caryll_allocate_clean` imports.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- `unknown-lookup.ttf`
    (4 GPOS lookups forced to format 10) in particular exercises pair
    positioning directly and confirmed the stable-sort substitution
    matches the real `qsort`'s output; all 10 round-trip payloads stable;
    issue #1's large-lookup regression test green; `compare-log-output.sh`
    green.

- **First unit of the `unsafe_op_in_unsafe_fn` burn-down proper (Stage 6-4
  continued): `Coverage`/`ClassDef`'s malloc/free shell becomes `Box::
  into_raw`/`Box::from_raw`, and their `qsort` scratch buffers become
  plain sorted `Vec`s.** With the `sds` sweep closed out, a survey of the
  114 files still carrying `#![allow(unsafe_op_in_unsafe_fn)]` (of 141
  total) found their `malloc`/`calloc`/`realloc`/`free` call sites --
  205 in total, concentrated in `table/` and `libcff/` -- as the next
  tractable seam: each site is a C-shaped manual allocation Rust's
  ownership types can absorb. `table/otl/coverage.rs` and
  `table/otl/classdef.rs` were picked as the first, smallest, safest unit
  after a closer read of the top candidates: both already store their
  actual payload as a `Vec` (`Coverage = Vec<GlyphHandle>`, `ClassDef.
  {glyphs,classes}: Vec<_>`, both landed in earlier Stage 6-4 PRs) and the
  remaining allocation sites are just the outer create/free shell around
  that `Vec` plus a `qsort`-driven scratch buffer -- no aliasing, no
  unions, no cross-file ownership transfer, unlike several other
  candidates surveyed alongside them (see below).
  - `otl_coverage_create`/`otl_class_def_create` switch from `malloc` +
    placement `.write()` to `Box::into_raw(Box::new(...))`; `otl_coverage_
    free`/`otl_class_def_free` switch from a `free()` call to `drop(Box::
    from_raw(x))`. The pointer type callers see (`*mut Coverage`/`*mut
    ClassDef`) is unchanged, so none of the ~20 files calling these
    functions needed to change -- only how the memory behind the pointer
    is obtained and released.
  - `coverage_from_raw`/`classdef_from_raw` (the "unwrap_X_table" idiom
    Stage 6-4 established for adopting a raw vtable-produced pointer into
    an owned value) simplify to a single `Box::from_raw` each, since the
    allocation being reclaimed already *is* a `Box` now -- no more
    `ptr::read` the value out, `free` the shell, `Box::new` a fresh copy.
    `table/tsi5.rs`'s `unwrap_class_def` (a second, independent
    implementation of the same idiom, predating `classdef_from_raw`)
    needed the identical fix: it used to `ptr::read` + a bare libc
    `free()`, which must be updated in lockstep with `otl_class_def_
    create`'s allocator -- freeing a `Box::into_raw` allocation with raw
    `free()` instead of `Box::from_raw` is exactly the mismatched-
    allocator hazard this whole conversion exists to close off, even
    though the two happen to coincide in practice under the default
    system allocator.
  - `otl_coverage_dispose` and `otl_class_def_dispose` (plus classdef's
    `dispose_class_def` helper) are deleted outright: both existed only
    to empty the `Vec` fields before the shell was freed, which is now
    redundant -- dropping the reclaimed `Box` drops its `Vec` fields on
    the way to deallocating the shell, in one step, and grep confirmed
    neither had any caller beyond its own file's `_free` function.
  - The three `qsort` scratch-buffer sites (`build_coverage_format`'s
    `r: *mut GlyphId`, `shrink_coverage`'s in-place sort of `Coverage`
    itself, `build_class_def`'s `r: *mut ClassDefSortRecord`) become a
    local `Vec` built with `.push()`/`.collect()` and sorted with `.sort_
    by_key(...)` -- **stable**, not `sort_unstable_by_key`, a deliberately
    conservative choice: `qsort` itself carries no stability guarantee,
    and while none of the three sites appeared to depend on tie-breaking
    order by inspection, a stable sort is the closest available match to
    "whatever glibc's qsort actually did on the tested platforms" without
    having to prove no payload can produce a tie. This also deletes the
    `by_gid`/`by_handle_gid` C-ABI comparator functions `qsort` needed,
    along with the `__caryll_allocate_clean`/`malloc`/`free`/`qsort`
    imports both files no longer use.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib -- `unknown-lookup.ttf` in
    particular exercises coverage/class-def tables heavily and confirmed
    the stable-sort substitution reproduces the real qsort's output
    exactly on every tested payload; all 10 round-trip payloads stable;
    issue #1's large-lookup regression test green; `compare-log-output.sh`
    green.
  - **Surveyed but explicitly deferred, for later units of this same
    burn-down**: `table/base.rs` (its `delete_base_axis` has a
    *documented* pre-existing leak -- never frees `BaseAxis` itself, only
    its `entries` -- that a straightforward `Box` conversion of the next
    layer in would silently fix by construction, the same shape as the
    `otfccbuild.rs` UAF fix above but not yet a decision this sweep made);
    `table/glyf/read.rs` (`point_indeces` aliases a shared buffer by
    default and only owns a private allocation, freed only on that one
    branch, under a bitflag -- tractable once read closely, but a real
    borrowed-vs-owned design choice, not a mechanical swap); `libcff/
    cff_parser.rs` (`CffFile.encodings`' payload is a genuine C union
    needing the same union-to-enum treatment the `Subtable` conversion
    already proved out at scale, confined to one file and 58 sites --
    likely tractable, just bigger); `libcff/subr.rs` (`CffSubrGraph` is a
    manually refcounted circular doubly-linked list with a sentinel node
    -- does not map onto `Box`/`Vec` at all, and likely needs either an
    arena-plus-indices redesign or a deliberate decision to leave it
    unsafe rather than force-fitting it into this burn-down's usual shape).

- **The `sds` sweep's true finale: `table/otl/read.rs`'s 12 name-building
  sites and `table/cff.rs`'s last `_Subfont` site, closing out every real
  `sds`/`SdsRaw` call site in application code.** A post-merge grep after
  the previous PR turned up this pocket -- never previously scoped in the
  sweep -- as GSUB/GPOS feature/lookup/langsys name construction and one
  CFF FDArray subfont name, all in the exact `let tmp =
  crate::sdsbuild!(sdsempty(), ...); X.name = sds_to_vec(tmp);
  sdsfree(tmp);` shape the Logger vtable PR's `bytesbuild!` macro already
  exists to replace directly (no `sdsempty()` seed, returns `Vec<u8>`
  outright) -- so all 13 sites collapsed to a single `bytesbuild!(...)`
  call each, purely mechanical, no behavior change.
  - `support/handle.rs`'s `sds_to_vec` helper, whose only callers were
    these 13 sites, is deleted along with it. Its neighbor `sds_into_vec`
    turned out to already have zero callers anywhere in the crate --
    genuinely dead code, deleted alongside it rather than left to rot.
  - Confirmed by grep that this really is the finale: excluding
    `vendor/sds.rs` itself (the vendored library, its macro definitions,
    and its own tests, which stay), there is no longer a single real
    `sds`/`SdsRaw`/`sdsbuild!`/`sdsempty()` call site left anywhere in
    the crate's application code.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib (the CFF-subroutinize
    payload in particular exercises the `_Subfont` naming path directly);
    all 10 round-trip payloads stable; issue #1's large-lookup regression
    test green; `compare-log-output.sh` green.

- **Stage 6-2's `sds` sweep finale: `table/glyf.rs`'s remaining leftover,
  `support/unicodeconv.rs`, `support/ttinstr.rs`, and both `bin/` CLI entry
  points, closing the explicitly-agreed sub-theme list out.** Chosen over
  fixing only the small `glyf.rs` leftover, and over pausing the sweep here,
  after a weighted priority pass had ranked these three as the lowest-value
  remaining pockets -- picked together because none of the four shares any
  code with the others, so batching was a scheduling choice only.
  - **`table/glyf.rs`**: the JSON-glyph loop's `gname: SdsRaw` (built via
    `sdsnewlen` off `json_obj_key_at`/`json_obj_key_len_at`, then copied
    into a `Vec<u8>` anyway before the `SdsRaw` was freed) collapses to a
    single `json_obj_key_bytes_at` call, removing a pointless copy-then-
    free-of-the-copy round trip.
  - **`support/unicodeconv.rs`**: `utf16le_to_utf8` deleted outright --
    zero callers anywhere in the crate. `utf16be_to_utf8` returns
    `Vec<u8>` instead of `SdsRaw`, with the output pass rewritten from
    raw-pointer writes into an `sdsnewlen`-allocated buffer to
    `Vec::with_capacity` + `.push()`; the `bytes_needed` sizing pass is
    unchanged. `utf8toutf16be` now takes `&[u8]` instead of `SdsRaw`: an
    earlier pass through this file had left a comment explicitly declining
    this conversion ("its internals walk raw pointers derived from
    `sdslen`, complex enough that widening it to `&[u8]` isn't worth the
    risk for its one call site") -- re-examined here and reversed, since
    the null check it was worried about is only reachable via a null
    `SdsRaw` (moot for a slice reference, which can't be null) and
    `sdslen` trivially becomes `.len()`. `table/name.rs`'s two call sites
    (decode and encode) simplified to match, dropping an `sdsnewlen`/
    `sdsfree` round trip on the encode side.
  - **`support/ttinstr.rs`**: the TrueType-instructions-from-JSON-array
    parser's scratch buffer (`sdsnewlen(NULL, istrlen + 1)`, a
    zero-filled buffer whose trailing zero byte serves as `parse_instrs`'s
    NUL terminator -- `parse_instrs` reads it with `strlen`) becomes
    `vec![0u8; istrlen + 1]`. The fill loop and the untouched
    `parse_instrs` function both keep working unmodified, since a
    zero-filled `Vec<u8>` gives the exact same size and the exact same
    trailing-NUL guarantee `sdsnewlen(NULL, ...)` did.
  - **`bin/otfccdump.rs` and `bin/otfccbuild.rs`**: `inPath`/`outputPath`
    (originally `SdsRaw`, built from `argv`/`optarg` via `sdsnew`) become
    `CString`/`Option<CString>` (mandatory in `otfccdump.rs`; genuinely
    optional in `otfccbuild.rs`, whose `inPath` falls back to stdin).
    **`otfccbuild.rs`'s conversion surfaced a genuine pre-existing
    use-after-free**, not introduced by this PR: the C-derived code freed
    `inPath` (`sdsfree(inPath)`) immediately after `readEntireFile`
    consumed it, but `inPath` was read again afterward, in two later
    `bytesbuild!`-built "Cannot parse JSON file ..." error messages on the
    parse-failure path. Converting to `Option<CString>` and simply
    removing that early free -- letting the value live for the function's
    natural scope and drop at the end -- fixes this by construction; no
    explicit bug-specific logic was needed, Rust's ownership model just
    doesn't have room for the buggy access pattern any more. Manually
    verified with a malformed JSON file and `--verbose` on both platforms:
    C prints `Cannot parse JSON file "". Exit.` (reading through the freed
    pointer recovers an empty string), while Rust correctly prints
    `Cannot parse JSON file "build/manual-uaf-check/broken.json". Exit.`
    -- a deliberate, documented, correctness-improving divergence from
    C's exact byte-for-byte output on this one error path, the first such
    intentional divergence found in this migration so far. The two error
    messages themselves handle `inPath`'s possible `None` (stdin fallback)
    with a null-safe raw-pointer fallback that reuses the existing
    `*const c_char` `SdsPart` impl's `"(null)"` rendering, so the no-path
    case still renders exactly as before.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib; all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green;
    `compare-log-output.sh` green; plus the manual use-after-free check
    above, run on both platforms.
  - **Not in scope for this PR**: a full-crate grep for remaining `sds`
    call sites turned up a pocket never previously scoped in this sweep --
    `table/otl/read.rs` has 24 real (non-comment) sites parsing GSUB/GPOS
    feature/lookup/langsys names from raw binary tag data -- plus 2
    remaining sites in `table/cff.rs` (the `tmp_subfont` site, flagged as
    out-of-scope back during the Logger vtable PR and never picked up
    since). Left untouched here, consistent with not silently expanding
    scope beyond what was explicitly agreed going in; to be raised
    separately.

- **Stage 6-2's seventh `sds` sub-theme: `table/post.rs`'s `pending_names`
  array and `json_reader.rs`'s glyph-order construction, batched into one
  PR.** Picked together per the weighted priority ranking (impact/future-
  value/readability) worked out after the `cmap.rs` PR -- both scored just
  behind `cmap.rs` and ahead of the remaining pockets (`unicodeconv.rs`,
  `ttinstr.rs`, the `bin/` CLI paths), and neither shares any code with the
  other, so batching them was purely a scheduling choice, not a dependency.
  - **`table/post.rs`**: the format-2.0 `post` table parser's
    `pending_names: [SdsRaw; 65536]` fixed stack array (pointer-sized *
    65536 = 512KB, plus a `memset` call that turned out to be fully
    redundant -- the Rust array literal `[null; 65536]` already
    zero-initializes, so the C-derived `memset` right after it was
    reinitializing already-zeroed memory) becomes a growable
    `Vec<Vec<u8>>`. The trailing manual `sdsfree` cleanup loop is gone
    entirely -- `Vec<Vec<u8>>` drops its own contents. Same shape as the
    Logger vtable PR's `Logger.indents` conversion: a fixed-capacity
    C-shaped array with hand-rolled bookkeeping becomes a plain growable
    container with none.
  - **`json_reader.rs`**: `set_order_by_name` and
    `escalate_glyph_order_by_name` (the two functions every glyph-order
    JSON source -- `glyf`, `cmap`, the explicit `glyph_order` array --
    funnels through) retyped off `SdsRaw`. `set_order_by_name` takes owned
    `Vec<u8>` (it conditionally stores the name), which **closes a
    documented pre-existing leak by construction**: the duplicate-name
    path used to leave the old `SdsRaw` `name` deliberately un-freed (the
    code comment said as much, calling it a leak inherited from the C
    original that "none of this function's callers free either") -- an
    unused `Vec<u8>` just drops instead, no explicit fix needed.
    `escalate_glyph_order_by_name` takes `&[u8]` instead, since it only
    ever reads for a lookup and never stores -- there was no ownership to
    plumb through in the first place, so borrowing is both simpler and
    more honest than following `set_order_by_name`'s owned shape out of
    misplaced consistency.
  - `place_order_entries_from_glyf`'s two `strcmp(gname, ".notdef"/
    ".null")` comparisons became plain `gname.as_slice() == b".notdef"`
    slice equality once `gname` was a `Vec<u8>` -- no NUL-terminated
    C-string detour needed at all.
  - `place_order_entries_from_cmap` inlines the exact same U+XXXX-or-
    decimal parse `table/cmap.rs`'s `parse_unicode` has (duplicated code,
    not shared) -- given the identical shape, applied the identical fix:
    borrow `json_obj_key_at`'s pointer directly instead of copying it into
    an owned `sds` first (every JSON object key is already NUL-terminated
    in `ParsedValue`'s own storage), so `strlen` sees the same length
    `sdslen` used to on the copy, with no `sdsnewlen`+`sdsfree` pair
    needed around the call any more.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib (every payload's `post`
    table and JSON-driven glyph ordering exercise both halves of this PR
    directly, no manual edge-case construction needed this time); all 10
    round-trip payloads stable; issue #1's large-lookup regression test
    green; `compare-log-output.sh` green.

- **Stage 6-2's sixth `sds` sub-theme: `table/cmap.rs`'s remaining
  `SdsRaw` usage -- both the dump-side key building and the parse-side
  `parse_unicode`/`parse_uvs_key` pair -- is retyped away entirely,
  closing out the `otfcc_encode_cmap_by_name`/`_uvs_by_name` adapter
  shim the `handle_from_name` PR deliberately left `SdsRaw`-in.** Picked
  first among the remaining themes (over `post.rs`'s `pending_names`
  array, `json_reader.rs`'s glyph-order construction) for weighing
  highest on impact/future-value/readability: cmap is a core table every
  font has, and finishing it cleans up residue from already-landed work
  rather than opening a new isolated pocket.
  - **Dump side**: `sdsbuild!`/`sdsfromlonglong`+`sdsfree`-built cmap and
    cmap_uvs JSON keys (`U+XXXX` hex or plain decimal, depending on
    `--decimal-cmap`/`--hex-cmap`) become `bytesbuild!` +
    `json_object_push_bytes_key`, the same shape used throughout this
    sweep. No behavior change: `c_int`/`c_uint`'s `SdsPart` decimal
    formatting already matches `sdsfromlonglong`'s output exactly for
    every value these keys can hold.
  - **`parse_unicode`/`parse_uvs_key` keep calling libc `strtol`/`atoi`
    unchanged (deliberately not reimplemented in safe Rust)** -- their
    real complexity was never the parsing logic itself, just the `SdsRaw`
    plumbing around it. Both now borrow `json_obj_key_at`'s pointer
    directly instead of copying it into an owned `sds` first: every JSON
    object key is already NUL-terminated in `ParsedValue`'s own storage
    (a `ParsedValue`-level guarantee, not an `sds`-level one), so `strlen`
    sees exactly the length `sdslen` used to on the copy, and neither call
    site needs an `sdsnewlen`+`sdsfree` pair around the call any more.
  - **`otfcc_encode_cmap_by_name`/`_uvs_by_name` finally drop their
    `SdsRaw`-in adapter shim**, now that the `gname` feeding them is a
    plain `Vec<u8>` (`json_str_bytes`, added by the `handle_from_name`
    PR). Both callers pass `gname.clone()` and keep their own copy for the
    log-warning message that follows on the "already mapped" path -- the
    same clone-at-the-call-site pattern used for `GlyphOrderPackage`'s
    `set_by_name`. Bonus: the pre-existing leak on that path (the old
    `SdsRaw` `name` was never freed when a codepoint/UVS pair was already
    taken) is gone by construction, not by an explicit fix -- an unused
    `Vec<u8>` just drops.
  - **Verification note**: no standard payload happens to have two cmap
    or cmap_uvs entries mapping to the same codepoint, so the "already
    mapped" warning path (and the `Vec<u8>`-drops-the-leak change) had
    zero coverage from the existing scripts. Manually constructed both
    collisions (a decimal and a zero-padded-decimal key parsing to the
    same codepoint; a UVS pair likewise) and confirmed byte-identical
    output *and* log text against the C build on both platforms, plus a
    `--hex-cmap` dump/build pass (the non-default key format) for the
    same reason.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib (`KRName-Regular`
    exercises `cmap_uvs` specifically); all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green;
    `compare-log-output.sh` green; plus the manual collision/hex-cmap
    checks above.

- **Stage 6-2's fifth `sds` sub-theme: `handle_from_name` (the
  `Handle`/`GlyphHandle`/`LookupHandle` constructor every parse-side glyph
  name reference goes through) is retyped from `SdsRaw` to
  `Option<Vec<u8>>`, closing out all 21 real call sites across 15 files.**
  Picked as the natural next step after the vtable-shaped and small-batch
  PRs, since several of the *other* remaining `sds` sub-themes (the
  `table/cmap.rs` parse side, `json_reader.rs`'s glyph-order construction)
  build on `handle_from_name` and would have needed this conversion
  eventually anyway.
  - **Why `Option<Vec<u8>>`, not a bare `Vec<u8>`.** The old `SdsRaw`
    signature let a caller pass a null pointer to mean "no name" while a
    non-null-but-empty `sds` string meant "named, and the name happens to
    be empty" -- two different `Handle` states
    (`HandleState::Empty`/`HandleState::Name`). Collapsing that to
    `v.is_empty()` on a bare `Vec<u8>` would have quietly merged those two
    states for the one call site that can legitimately hit it
    (`table/glyf.rs`'s CFF `fd_select`, sourced from `json_obj_getsds`,
    which already returns `None` when the JSON key is simply absent).
    `Option<Vec<u8>>` preserves the distinction exactly, and that same
    call site now passes its `json_obj_getsds(...)` result straight
    through with no wrapping at all -- both already speak `Option<Vec<u8>>`.
  - **New accessors added to `support/parsed_json.rs`:** `json_str_bytes`
    and `json_obj_key_bytes_at`, the owned-`Vec<u8>` combination of the
    existing `json_str_ptr`+`json_str_len` / `json_obj_key_at`+
    `json_obj_key_len_at` pairs every other call site used to feed straight
    into `sdsnewlen`. Returning an owned copy (not a borrowed `&[u8]`) sidesteps
    any question of what lifetime a slice reconstructed from a raw pointer
    could honestly claim -- every call site wanted an owned copy anyway,
    either for a `Handle.name` field or as `handle_from_name`'s argument.
  - **Two functions deleted outright:** `handle_from_consolidated` and
    `handle_consolidate_to` had zero real callers left (grep-confirmed;
    only comments still named them, explaining what had already replaced
    each call site during earlier `consolidate/otl/*.rs` and
    `support/glyph_order.rs` work) -- no need to convert their signatures
    at all.
  - **`table/cmap.rs`'s two wrapper functions kept their `SdsRaw`-in
    signature on purpose.** `otfcc_encode_cmap_by_name`/
    `_uvs_by_name` forward straight to `handle_from_name`, but their own
    two callers (`parse_cmap_unicodes`/`parse_cmap_uvs`) build the name via
    hand-rolled pointer scanning that belongs to a separate, not-yet-scoped
    `sds` theme -- converting these two wrappers' signatures would have
    dragged that theme's scope into this PR. Adapted internally instead
    (copy the bytes for `handle_from_name`, then free the original `SdsRaw`
    only on the path that used to consume it), leaving both wrappers'
    callers untouched and their existing "the Occupied/duplicate path never
    frees `name`" pre-existing behavior exactly as it was (not this PR's
    job to fix).
  - Every other call site's `sdsnewlen(json_str_ptr(v), json_str_len(v))`/
    `sdsnewlen(json_obj_key_at(...), json_obj_key_len_at(...))` shape
    collapsed to a single `Some(json_str_bytes(v))`/
    `Some(json_obj_key_bytes_at(obj, i))` call, with the (rare) raw-pointer
    local that used to hold the intermediate `SdsRaw` -- when nothing else
    in the function still needed it -- removed entirely rather than kept
    around unused.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib (every payload's GSUB/GPOS
    lookups, coverage tables, class defs, and GDEF caret lists exercise
    `handle_from_name` extensively); all 10 round-trip payloads stable;
    issue #1's large-lookup regression test green; `compare-log-output.sh`
    green.

- **Stage 6-2's fourth `sds` sub-theme: four small, self-contained
  `sds`/`SdsRaw` sites unrelated to any vtable, batched into one PR.**
  Chosen as a deliberate change of pace after the vtable-shaped Logger and
  `GlyphOrderPackage` PRs -- these four don't share any infrastructure or
  call site with each other, so they're grouped only by being independently
  small.
  - **GPOS mark-class JSON keys** (`table/otl/subtables/
    gpos_mark_to_single.rs`, `gpos_mark_to_ligature.rs`): the
    `"anchor0"`/`"ac_3"`-style keys built for each mark class during dump,
    2 sites per file. `sdsbuild!(sdsempty(), b"anchor", k)` +
    `sdslen`/`sdsfree` became `bytesbuild!(b"anchor", k)` handed straight to
    `json_string_new_from_bytes`/`json_object_push_bytes_key` -- both
    already existed (built for the C-3/Logger work), so this needed no new
    infrastructure, only wiring up already-scoped-out call sites. Verified
    against `NotoNastaliqUrdu-Regular`, the one standard payload with real
    `gpos_mark_to_base`/`gpos_mark_to_ligature` content (345 `"class"` keys
    across its GPOS table) -- byte-identical.
  - **`table/os_2.rs`'s `achVendID`**: a fixed 4-byte field
    (`CffTable.ach_vend_id: [u8; 4]`) that used to go through
    `sdsnewlen`+`json_string_new`+`sdsfree` for no reason beyond that being
    the only string constructor available at transpile time -- now a
    direct `json_string_new_from_bytes(&(*table).ach_vend_id)` (array-to-
    slice coercion, no allocation-shaped detour at all).
  - **`table/name.rs`'s copyright string**: replaced the version-string
    build (`sdsbuild!` piecing together `"-- By OTFCC ", MAIN_VER, ".",
    ...`) and its trailing `sdsgrowzero`-to-`COPYRIGHT_LEN` pad with
    `bytesbuild!` + `Vec::resize`. Bonus: removes a documented-but-never-
    triggered latent use-after-free class from the original C
    (`sdsgrowzero` may reallocate and the C source dropped the result) --
    `Vec::resize` has no equivalent hazard to begin with, so the comment
    explaining why the bug never fired could be deleted along with the bug
    class itself.
  - **Deliberately left alone**: `gpos_cursive.rs`/`gpos_single.rs` also
    have `SdsRaw` locals named `gname`, but those are `handle_from_name`
    call sites (JSON *key* → `Handle`, parse-side), not
    `mark_class_name`-shaped constructions -- a different, not-yet-scoped
    theme (`handle_from_name`'s 21 call sites across 10 files). Caught
    during scoping by checking each file's actual code shape rather than
    trusting the file name alone.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib; all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green;
    `compare-log-output.sh` green.

- **Stage 6-2's third `sds` sub-theme: the `GlyphOrderPackage` vtable
  (`set_by_gid`/`set_by_name`/`lookup_name`) is retyped from `SdsRaw` to
  `Vec<u8>`.** The largest remaining piece of the sweep by call-site count
  (598, versus Logger's 227), chosen over the smaller `handle_from_name`
  pilot specifically because it's the other vtable-shaped theme, the same
  reason the Logger PR was scoped ahead of the smaller pilots.
  `support/glyph_order.rs`'s `GlyphOrderEntry.name` and the
  `name_a_field_shared` slot were already `Vec<u8>` from earlier work; this
  PR closes out the 3 slots that still took a raw, `sdsfree`-requiring
  `SdsRaw` as input.
  - `otfcc_set_glyph_order_by_gid`/`otfcc_set_glyph_order_by_name`/
    `gord_lookup_name` all lost their internal `sds_to_vec`/`sdsfree` calls
    entirely -- `Vec<u8>` drops on its own in every branch, where the old
    code needed an explicit free matched to each return path by hand.
  - **The one real ownership wrinkle, resolved by cloning at the call
    site rather than restructuring the vtable further.**
    `otfcc_set_glyph_order_by_name` has a conditional-ownership contract:
    it consumes `name` on the two paths that store it (new entry, or --
    via `set_by_gid` -- an existing GID), but on the "name already taken"
    path it never touches `name` at all, leaving the *caller* responsible
    (documented in the function's own comment, previously "deliberately
    left un-freed"). `Vec<u8>`'s move semantics can't express "maybe
    consumed, maybe not" the way a raw pointer convention could -- so
    `consolidate.rs`'s two calls now pass `name.clone()`/`newname.clone()`
    and keep their own copy for the log message and retry loop that follow
    regardless of which path the callee took. This trades a small `Vec`
    clone (once per glyph during consolidate, not a hot path) for making
    the "which branch frees this" question -- the exact hazard class
    flagged when this theme was scoped -- structurally impossible to get
    wrong again.
  - `support/aglfn.rs`'s 586 call sites (the Adobe Glyph List name table,
    100% uniform `sdsnew(b"Name\0" as *const u8 as *const c_char)` →
    `set_by_gid`) converted with a scripted regex pass to
    `b"Name".to_vec()`, exactly as scoped -- no hand-editing 586 lines.
  - `otf_reader/unconsolidate.rs`'s `create_glyph_order` -- the function
    that actually names every glyph when unconsolidating a font (by hash,
    by cmap/AGLFN lookup, by prefix, or by position) -- was the one
    genuinely busy site: every local `SdsRaw` (`prefix`, `gname`, the
    per-attempt `newname` in the hash-collision retry loop, the cmap/post
    name-building locals) became `Vec<u8>`, and the hash-naming loop's
    branchy `sdsbuild!` pair (append "-" then the hex byte, or just the hex
    byte) simplified to an unconditional `Hex2Upper(...).append_to_vec(&mut
    gname)` with the "-" pushed first when the branch condition holds --
    same output, less duplication. The one `lookup_name` call whose result
    feeds a later branch (deciding whether the hash name needs a numbered
    suffix) passes `gname.clone()`, matching the same pattern as
    `consolidate.rs`'s `set_by_name` sites and for the same reason.
  - `table/post.rs`'s 2 `set_by_gid` sites (mapping the `post` table's
    name index back to glyph names) needed no local restructuring --
    `sdsdup(pending_names[i])` became `sds_to_vec(pending_names[i])` (a
    copy, not a free-then-realloc; `pending_names` itself is still the
    `[SdsRaw; 65536]` stack array from a separate, not-yet-scoped part of
    the sweep, and is freed exactly as before) and
    `sdsnew(STANDARD_MAC_NAMES[i].as_ptr())` became
    `STANDARD_MAC_NAMES[i].to_bytes().to_vec()`.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib; all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green;
    `compare-log-output.sh` green. Additionally, since no existing script
    exercises `--name-by-hash` (the SHA1-hash-naming path through the
    collision-retry loop this PR restructured the most), manually ran
    `otfccdump --name-by-hash` + `otfccbuild` through both the C and Rust
    binaries on both platforms and confirmed byte-identical output there
    too.

- **Stage 6-2's second `sds` sub-theme: `json_obj_getsds`, the CFF
  string-lookup pair (`sdsget_cff_sid`/`form_cid_string`), and CFF's
  string-dedup map are all retyped away from `sds`/`SdsRaw`.** Chosen
  (over the vtable-shaped `GlyphOrderPackage` theme, or scoping the
  smaller `handle_from_name` pilot first) as a self-contained, no-vtable
  slice of the theme surveyed after the Logger PR landed.
  - `support/parsed_json.rs`'s `json_obj_getsds` returns `Option<Vec<u8>>`
    instead of a possibly-null `SdsRaw` the caller had to `sdsfree`. Its 11
    callers split three ways: `table/cff.rs`'s 9 sites (already wrapped in
    `sds_into_vec`, so they collapse to `.unwrap_or_default()`);
    `table/glyf.rs`'s 1 site, which still has to hand a raw `SdsRaw` to
    `handle_from_name` (left untouched — out of scope for this PR) so it
    rebuilds one from the `Option<Vec<u8>>` with `sdsnewlen`; and
    `table/svg.rs`'s 1 site (the `document` field, both the plain and
    base64-encoded branches), which now writes straight from the `Vec<u8>`
    into `bufwrite_bytes` instead of round-tripping through `sdslen`.
  - `libcff/cff_string.rs`'s `sdsget_cff_sid` returns `Option<Vec<u8>>`
    (`None` for a SID with no matching entry in the CFF string INDEX);
    `table/cff.rs`'s private `form_cid_string` returns a never-null
    `Vec<u8>` built with `bytesbuild!`. 14 call sites across
    `table/cff.rs`, all already null-checking before use, so the
    conversion is a mechanical `if let Some(...)`/`.unwrap_or_default()`
    swap throughout.
  - **`CffTable.string_hash`** (the CFF string-dedup table built while
    writing a CFF font, `sid → string` insertion-ordered by
    `IndexMap`) moves from `IndexMap<Vec<u8>, SdsRaw>` to
    `IndexMap<Vec<u8>, Vec<u8>>`. `sidof`'s insert becomes a plain
    `s.to_vec()`; `cffstrings_to_indexblob`'s drain loop calls the
    existing `bufnwrite8` instead of `bufwrite_sds`+`sdsfree`, which left
    `bufwrite_sds` itself with zero remaining callers crate-wide —
    deleted, along with its now-unused `SdsRaw`/`sdslen` imports in
    `support/buffer.rs`.
  - No storage-shape surprises this time (unlike Logger's `Logger.indents`
    or C-3's attach-before-populate hazard) — every site here was already
    either null-checked before use or paired 1:1 with its own
    `sdsfree`/`sds_to_vec`, so the conversion only had to swap the type,
    not restructure ownership.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib (`KRName-Regular` and
    `Reinebow-SVGinOT` specifically exercise the CFF string and SVG
    `document` paths touched here); all 10 round-trip payloads stable;
    issue #1's large-lookup regression test green; `compare-log-output.sh`
    green on both platforms too (unaffected by this PR, but re-run as part
    of the standard pipeline going forward).

- **Stage 6-2's Logger vtable is retyped from `sds`/`SdsRaw` to `Vec<u8>`,
  the first sub-theme of the `sds` → `Vec<u8>`/`String` sweep.** `ILogger`'s
  `indent_sds`/`start_sds`/`log_sds` and `ILoggerTarget::push` now pass
  `Vec<u8>` instead of the old manually-`sdsfree`'d `SdsRaw`; `Logger.indents`
  (the per-nesting-level indent-segment stack) is `Vec<Vec<u8>>` instead of a
  hand-rolled malloc/realloc'd `*mut SdsRaw` array with a separate
  `level_cap` field tracking its capacity — `level_cap` is gone entirely,
  replaced by `.push()`/`.pop()`. Shipped as one PR covering both the
  infrastructure and the full call-site sweep (unlike the JSON work's
  "ship unwired, then wire up" split): there's only one type here, so there
  was no unwired-shim stage to ship separately.
  - **New:** `SdsPart::append_to_vec(self, v: &mut Vec<u8>)`, a sibling to
    the existing `SdsPart::append_to(self, s: SdsRaw) -> SdsRaw`, implemented
    on every existing `SdsPart` impl (`&[u8]`, C strings, `Sds`, `Byte`,
    the `Dec5`/`Hex4`/`Hex2` number-formatting wrappers, etc.) with the exact
    same rendering logic, writing into a growable `Vec<u8>` instead of an
    `sds` buffer. `bytesbuild!(...)`, a `sdsbuild!` sibling with no leading
    `sdsempty()` seed argument (a fresh `Vec::new()` needs no allocator
    call), replaces every logger-bound `sdsbuild!(sdsempty(), ...)` call —
    about 190 sites across ~51 files (`consolidate.rs` and its `otl/*.rs`
    submodules, every `table/*.rs` leaf dumper, `libcff/{cff_parser,subr}.rs`,
    `json_reader.rs`, `font/caryll_sfnt_builder.rs`, `otf_writer/stat.rs`, and
    both `bin/{otfccdump,otfccbuild}.rs`), found by iterating
    `cargo build --lib --bins`'s exact diagnostic spans rather than a
    blind file-wide regex — several files (`table/cmap.rs`'s JSON-key
    building, `consolidate.rs`'s glyph-name dedup) mix logger and
    non-logger `sdsbuild!` calls in the same file, so only the
    logger-bound ones convert.
  - `push_stopwatch` (`support/stopwatch.rs`) — the one place a `bytesbuild!`
    result flows through a variable instead of a direct vtable-call
    argument — returns `Vec<u8>` now too; its 12 call sites (7 in
    `otfccdump.rs`, 5 in `otfccbuild.rs`) needed no changes, since they
    already just forward the return value straight into `log_sds`.
  - `Logger.indents = Vec::new()` is set explicitly in `otfcc_new_logger`
    after the `calloc`-equivalent allocation — the zeroed bytes a fresh
    `Logger` starts with are not a valid `Vec` bit pattern (`Vec`'s empty
    state is `NonNull::dangling()`, not a null pointer), the same hazard
    recorded for every other malloc'd-struct-gets-a-`Vec`-field conversion
    this migration has done.
  - **New verification script:** `rust/scripts/compare-log-output.sh`.
    Every existing comparison script (`compare-with-c.sh`,
    `compare-with-golden.sh`, `run-cycles.sh`) only checks the produced
    font/JSON files — none of them touch stderr, so nothing was actually
    exercising the Logger vtable end to end before this PR. The new script
    runs `otfccdump`/`otfccbuild` under `--verbose`, `--quiet`, and a
    missing-input-file error case for both the C and Rust builds, and
    diffs stderr byte-for-byte after blanking out `push_stopwatch`'s
    `Step time = N.NNNNNNs.` numbers (the one piece of log output that can
    never match between two separate process runs). Confirmed the script
    actually detects a divergence (not just trivially passing on empty
    output) by corrupting one captured log and checking the comparison
    fails before restoring it.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib; all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green; and the new
    `compare-log-output.sh` green on both platforms.

- **Stage 6-2.5's Theme C is complete: the old vendored `JsonValue`-based
  JSON parser and builder are deleted entirely (C-4), closing out the
  theme C-1 through C-3 spent four PRs building toward.** With C-2
  (`ParsedValue`) and C-3 (`BuiltValue`) both fully wired, grep confirmed
  `support/json_funcs.rs`'s whole accessor layer, `vendor/json.rs`'s
  parser (`json_parse_ex` and its state machine, `JsonValue`/
  `JsonObjectEntry`/the union payload, `JsonSettings`/`JsonState`), and
  `vendor/json_builder.rs`'s builder (`JsonBuilderValue`, `builderize`,
  every `json_*_new`/`json_*_push` constructor, `json_measure_ex`/
  `json_serialize_ex`, `json_builder_free`) had zero remaining callers in
  production — deleted outright rather than kept as dead weight. Net
  change: **~4,900 lines removed** across the three files (~301 added,
  mostly test rewrites below).
  - **What survived, and why.** `vendor/json.rs` now holds only the
    `JsonType` enum (12 lines) — `parsed_json`/`built_json`'s own APIs
    still use it as a plain type-tag argument
    (`json_obj_get_type`/`json_type_of` and friends), independent of the
    parser that used to sit alongside it. `vendor/json_builder.rs` now
    holds only `JsonSerializeOpts` and the `JSON_SERIALIZE_MODE_*`/
    `JSON_SERIALIZE_OPT_*` constants (~30 lines) — real, live imports:
    `built_json.rs` re-exports them, and `bin/otfccdump.rs` constructs
    them directly. `support/json_funcs.rs` had nothing left to keep and
    was deleted as a whole file.
  - **The judgment call, made explicitly rather than silently.** Both
    `parsed_json.rs`'s and `built_json.rs`'s own differential test suites
    used the old parser/builder as their correctness oracle (comparing the
    new implementation's output against the old one's, byte-for-byte or
    tree-for-tree) — deleting the vendored code meant losing that "matches
    legacy C behavior forever" signal unless the tests were rewritten
    first. Rewritten as fixed assertions instead: `built_json.rs`'s
    `packed_matches_the_known_good_fixture`/`multiline_matches_the_known_
    good_fixture` assert against byte fixtures captured from the new
    serializer's own output (after confirming that output matched the old
    builder one last time); `parsed_json.rs`'s number/string/leniency/
    malformed-input tests assert directly against `parse_json`'s parsed
    structure instead of a second parser's tree. Test count is unchanged
    (55 before and after); the payload-directory smoke test
    (`every_committed_payload_json_parses`) lost its structural-equality
    check but keeps its "doesn't fail to parse" coverage over every real
    `.json` payload in the tree.
  - **A second latent bug found while rewriting the fixture tests, this
    time in the test code itself, not the vendored library.**
    `built_json.rs`'s differential-test tree builder
    (`build_sample_tree`) passed hardcoded lengths to
    `json_object_push_length` that didn't match several of its own C
    string literals' actual lengths (`"ints"` passed as length 6 instead
    of 4, one shared length `7` used for six keys of different actual
    lengths, `"emptyarr"` passed as length 9 instead of 8) — reading past
    the literal into whatever followed in the binary's rodata, or (for the
    exact-`len+1` case) capturing the string's own NUL terminator as
    content. Undetected until now because the *same* wrong length was fed
    to both the old and new builder in the comparison, so both sides
    produced the identically-corrupted key and the differential assertion
    still passed — a textbook case of a bug two implementations share
    surviving a differential test unnoticed. Fixed before capturing the
    new fixtures (using `strlen`-based pushes or a computed length instead
    of a hand-counted literal, removing the whole bug class), confirmed
    still matching the old builder one more time before it was deleted.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib (as expected — pure
    dead-code deletion, no behavior change); all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green on both
    platforms too.

- **All ~40 build/dump-side files now construct `BuiltValue` instead of
  `vendor::json::JsonValue` — Stage 6-2.5 C-3, part 2, wiring up the
  representation the previous PR shipped unwired.** Every `otfcc_dump_X`/
  `otl_dump_X`/`dump_class_def`/`dump_coverage`-family function's signature
  moved from `*mut JsonValue` to `*mut BuiltValue`, from the two build-side
  entry points (`json_writer.rs`'s dispatcher, and `bin/otfccdump.rs`'s one
  `IFontSerializer`/`*mut c_void` boundary crossing) down through every leaf
  table dumper. Shipped as one PR, same reasoning as C-2 part 2: return-type
  propagation through a shared tree root is just as viral as parameter-type
  propagation was on the parse side — confirmed by converting the
  dispatcher alone and watching the compiler error trail point at exactly
  the still-unconverted callees, the same tracking strategy C-2 used.
  - **A real, silent-corruption bug found and fixed during this wiring, not
    present in C-2's parse-side conversion.** Several dump functions built
    an empty array/object, pushed it into its parent immediately, and only
    *afterward* populated it in a loop — sound under the old `JsonValue`
    API because `json_object_push`/`json_array_push` store a raw pointer,
    so later mutations through the original handle stayed visible via the
    alias. `BuiltValue`'s push *moves* the value in by ownership, so that
    same ordering silently discards everything pushed after the handoff —
    the parent ends up holding the empty snapshot. Caught by
    `compare-with-c.sh`'s byte-for-byte check (`meta-test`/`vdmx-test`
    dumps came back with truncated/empty arrays), not by the type checker,
    since both old and new APIs return the same pointer type and compile
    equally well either way. Fixed in `table/meta/dump.rs` (one
    array) and `table/vdmx/funcs.rs` (nested three levels deep — ratios,
    each ratio's records, this needed reordering at every level) by
    restructuring each container to be fully populated with its own
    children *before* being pushed into its parent. Swept every other
    converted file for the same shape with a small script (validated
    against these two known-buggy files first, to confirm it actually
    catches the pattern before trusting a clean sweep elsewhere) — no
    further instances found.
  - Also converted: `support/ttinstr.rs`'s `dump_ttinstr` (the one
    parse/build pair living outside `table/`, mirroring `parse_ttinstr`'s
    own C-2 move), and `table/otl/{classdef,coverage}.rs`'s `IClassDef.dump`/
    `ICoverage.dump` vtable fields (the return-type equivalent of C-2's
    `.parse` field conversion).
  - **The last 4 build-side helpers still living in `support/json_funcs.rs`
    (`otfcc_dump_flags`, `json_object_push_tag`, `json_new_position`,
    `preserialize`) moved to `support/built_json.rs`, then were deleted
    from `json_funcs.rs`** once grep confirmed zero remaining callers
    (mirroring C-2's `json_ident.rs` deletion) — along with their
    now-unused imports (`round`'s `extern "C"` declaration is gone too;
    `json_new_position` uses `f64::round` directly, identical rounding
    behavior, no libm binding needed).
  - `bin/otfccdump.rs`'s buffer handling simplified along with the
    `json_measure_ex` removal C-3 part 1 already flagged: `json_serialize_ex`
    now returns an exact `Vec<u8>`, so the old "scan backward over the
    over-sized buffer's trailing zero padding to find the real end" step
    is gone, and both the file-output and stdout-output paths write the
    exact byte count via `fwrite` uniformly (the old stdout path used
    `fputs`, relying on NUL-termination that a `Vec<u8>` doesn't carry).
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib (both platforms — the real
    dlopen check runs in the Linux container); all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green on both
    platforms too.

- **A new safe Rust representation for the JSON *build/dump* side exists now
  (`support::built_json::{BuiltValue, ...}`) — Stage 6-2.5 C-3, part 1. It
  is not wired into anything yet.** Mirrors C-2's own two-stage shape: this
  PR ships `BuiltValue` and a constructor/serializer layer that mirrors
  `vendor::json_builder`'s API name-for-name, plus a differential test
  suite proving `json_serialize_ex` matches `vendor::json_builder::
  json_measure_ex`+`json_serialize_ex` byte-for-byte (packed and multiline
  modes, every escape case, `PreSerialized` splicing, empty containers);
  the next PR(s) will switch the ~40 `table/*.rs` dump-side functions
  (currently calling `vendor::json_builder` directly) over to it.
  - **Real-world scope turned out narrower than C-2's, once actually
    surveyed** (same shape of finding as C-2's own `json_parse_ex`
    narrowing): `builderize()` — the "upgrade a bare `JsonValue` the
    *parser* produced into a builder value in place" escape hatch — never
    fires in this codebase. Every value ever passed to a `json_*_push`
    call was itself produced by a `json_*_new` call; now that C-2 already
    split `ParsedValue` (parse) from `JsonValue` (build) into distinct
    Rust types, a parsed value reaching this API would be a compile error,
    not a runtime "maybe". `json_object_sort`/`json_object_merge` have
    zero callers anywhere in the crate (confirmed by grep) and were
    dropped rather than ported. `.parent`/`length_iterated` — bookkeeping
    that let the old measure/serialize/free walk the tree *iteratively*
    instead of recursively, a pure C-stack-frugality concern — have no
    equivalent at all; `BuiltValue`'s serializer is ordinary recursion over
    `&BuiltValue`, and there's no `BuiltValue`-side `free` function
    (`Drop` does it).
  - **`json_measure_ex` itself has no replacement, and that's deliberate,
    not an oversight.** Read closely, it exists purely to pre-size a
    `calloc`'d C buffer before `json_serialize_ex` fills it in a second
    pass — and it deliberately *over*-estimates (its arithmetic adds
    `newlines * indent_size` *and* the actual summed indent depth, not
    either alone). `bin/otfccdump.rs` has to scan backward over the
    resulting buffer's trailing zero padding to find where the real
    content actually ends before writing it out — a workaround for the
    over-estimate that a `Vec<u8>`-returning serializer makes unnecessary.
    `built_json::json_serialize_ex` returns the exact bytes directly, no
    upfront size pass, no trailing-zero scan; wiring this in (the next PR)
    will delete that scan from `bin/otfccdump.rs` as dead weight along
    with the measure step itself.
  - **One more latent bug found and flagged, not fixed here, mirroring
    C-2's integer-overflow finding on the parse side.** `json_serialize_ex`'s
    integer arm does `integer = -integer` on a negative `i64` with no
    overflow guard — UB-but-wraps in C, but a checked-negation panic in a
    debug-mode Rust build when the value is `i64::MIN`. Found while
    building this PR's differential test's integer sample set (`i64::MIN`
    had to be excluded to avoid crashing the *old* builder under test);
    `BuiltValue`'s own integer serialization uses `i64::to_string()`
    instead, which has no such edge case. Flagged as a follow-up task
    rather than fixed in the vendored code, since C-3's own wiring PR will
    delete that code path entirely once it lands.
  - `BuiltValue` needs no NUL-termination convention on `Str`/keys, unlike
    `ParsedValue` — nothing on the build side ever reads a constructed
    value back out through a C-string accessor; it's only ever
    constructed, then serialized.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 55 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib (as expected — nothing is
    wired yet, so existing dump behavior is provably unchanged).

- **All ~48 parse-side consumer files now read through `ParsedValue`
  instead of `vendor::json::JsonValue` — Stage 6-2.5 C-2, part 2, wiring up
  the parser this theme's previous PR shipped unwired.** Every
  `otfcc_parse_X`/`otl_parse_X`/`otl_gsub_parse_X`/`otl_gpos_parse_X`
  function's signature moved from `*const JsonValue` to
  `*const ParsedValue`, all the way from the two top-level entry points
  (`bin/otfccbuild.rs`, `ffi/dll.rs`, which now call
  `support::parsed_json::{json_parse, json_value_free}` instead of the
  vendored parser) down through `json_reader.rs`'s dispatcher to every leaf
  table parser. Shipped as one PR, not split: unlike the B-3/C-1 themes,
  these are function *signatures* forming one connected call graph — a
  callee's signature can't move without every caller moving too, confirmed
  by converting a callee alone and watching the compiler error trail point
  at exactly its still-unconverted callers. That compile-error trail is
  what drove the file-by-file order below.
  - **`support::parsed_json`'s accessor layer mirrors `support::json_funcs`
    name-for-name** (both the original 16 helpers and C-1's 11 additions),
    targeting `*const ParsedValue` instead — so a converted file's call
    *expressions* stay textually identical; only its `use` line and its own
    signatures change. Four of those accessors (`json_obj_get`,
    `json_obj_get_type`, `json_obj_val_at`, `json_arr_at`) return
    `*mut ParsedValue`, not `*const` — matching `json_funcs`'s own
    surprising choice (a "read" accessor returning a mutable pointer)
    exactly, discovered when the first `*const`-returning attempt broke
    call sites that legitimately need to write back through the result
    (see the mutation bullet below). `*mut T` coerces to `*const T`
    implicitly, so this widened return type cost nothing at any read-only
    call site.
  - **Two new primitives that don't mirror anything in `json_funcs`,
    because the old union design had no equivalent.** `json_parse`/
    `json_value_free` are thin `*const c_char`/`usize` → `*mut ParsedValue`
    wrappers around `parse_json`/`Box`, matching `vendor::json::json_parse`'s
    own raw-pointer contract exactly so the two FFI entry points didn't need
    reshaping. `json_obj_set_val_at`/`json_obj_null_out_val_at` replace two
    C-side patterns that used to reach into `JsonValueReserved`/`.parent`
    fields by hand: `glyf.rs` frees each glyph's parse subtree as soon as
    it's consumed (bounding peak memory on huge fonts) by splicing in a
    fresh `json_null_new()`, and `table/otl/parse.rs`'s duplicate-feature/
    lookup merger turns a structurally-identical duplicate definition into
    an alias string the same way. Both collapse into a single
    `ParsedValue`-owning assignment now — `*v = ParsedValue::Null` (or
    `Str(..)`) drops the old subtree for free, no `.parent` bookkeeping
    left to get wrong.
  - **The one hand-reasoned bug-compatibility case: `json_ident`'s call
    site.** `table/otl/parse.rs`'s duplicate merger used to call a
    hand-written deep-equality walk (`support/json_ident.rs`) because
    `JsonValue` had no derived `PartialEq`. `ParsedValue` does (`#[derive(
    ..., PartialEq)]`, added when the type was first defined) — so
    `json_ident(jthis, jthat)` became `*jthis == *jthat` directly, and
    `json_ident.rs` was deleted as dead code (confirmed zero remaining
    callers by grep before removal).
  - **`json_vq_of` (`table/fvar.rs`) is the one parse-direction function
    living in an otherwise dump-only file** — every other `fvar.rs`
    function only *builds* `JsonValue`s for the variable-font dump path,
    but this one reads a coordinate back out of parsed JSON for `glyf.rs`'s
    variable-point parsing. Easy to miss on a file-level "is this
    dump-only?" pass; caught because it's `glyf.rs`'s only remaining
    `JsonValue` reference once the rest of that file converts, and the
    compiler pointed straight at it.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 51 unit tests green (0 warnings under
    `warnings = "deny"`); every standard payload byte-identical in both
    directions including the `otfccdll` cdylib (both platforms — the real
    dlopen check runs in the Linux container); all 10 round-trip payloads
    stable; issue #1's large-lookup regression test green on both
    platforms too.

- **A new safe, single-pass JSON parser exists now (`support::parsed_json::
  {ParsedValue, parse_json}`) — the first piece of C-2 of Stage 6-2.5,
  "make `JsonValue` safe Rust". It is not wired into anything yet.** This
  PR ships the parser and a differential test suite proving it matches
  `vendor::json::json_parse` byte-for-byte; the next PR(s) will switch the
  ~48 `table/*/parse.rs`-family files (currently reading through C-1's
  accessor layer) over to it.
  - **Why a genuinely separate type, not a representation change to
    `JsonValue`.** `JsonValue` is shared today by both the parse side
    (`vendor::json`) and the build side (`vendor::json_builder`, every
    `table/*/dump.rs`, the writer). Investigated before writing any code:
    the two object graphs never actually intersect at runtime (the whole
    parse tree is freed via `json_value_free` before any build tree is
    constructed), and no dump-side code ever reads *through* an existing
    value the way parse-side code does — the only `json_funcs` calls from
    `dump.rs` files are `preserialize`/`otfcc_dump_flags`, both pure
    build-side constructors. That means introducing a distinct
    `ParsedValue` type for the parse side is sound and requires touching
    zero build-side code, so `vendor::json::JsonValue` and
    `vendor::json_builder` are both untouched by this PR.
  - **The real contract turned out to be much narrower than
    `json_parse_ex`'s general-purpose signature suggests.** `json_parse_ex`
    takes a `JsonSettings` (custom allocator, `max_memory`, a comment-
    syntax flag) and an `error_buf` for diagnostic text — but the only
    entry point actually used anywhere in this crate is `json_parse`
    (2 call sites: `bin/otfccbuild.rs`, `ffi/dll.rs`), which always passes
    `error_buf = null` and `settings.settings = 0`. Parse-error text and
    position have never been surfaced anywhere in this codebase, and
    comment support (`JSON_ENABLE_COMMENTS`) is dead code in this crate's
    actual usage. `parse_json` therefore only needs to answer "parses, or
    doesn't" — no line/column tracking, no comment syntax, no allocator
    hookup (moot once `Vec`/`Box` own the memory instead of the vendored
    parser's two-pass measure-then-fill C algorithm).
  - **Two genuine leniency quirks in the vendored parser, found by testing
    against it rather than assumed from reading 3000 lines of state
    machine, then matched exactly.** (1) A single trailing comma before
    `]`/`}` is silently tolerated (`json_parse(b"[1,2,]")` succeeds and
    produces a 2-element array) — but a second consecutive comma, or a
    comma before the first element, is still rejected. (2) A bare `-` with
    no digits after it is accepted as `Int(0)`, not rejected — the
    vendored parser's number state has an explicit "expected digit"
    check after `.` and after `e`/`E`, but not after the leading sign,
    and the value's `.integer` field starts calloc'd-zero. Both were
    confirmed with a throwaway probe against the real `json_parse` before
    being written into `parse_json` and into the permanent differential
    test suite (`parser_leniency_quirks_match`), not inferred from source
    reading alone.
  - **Number parsing replicates the vendored parser's exact arithmetic
    assembly, not a generic string-to-f64 parse.** Integer vs `Double` is
    a syntactic decision (presence of `.`/`e`/`E` in the literal, not
    magnitude), matching `FLAG_NUM_E`'s role in `json.rs`. Double values
    are assembled the same way: `int_part as f64`, then
    `+= fraction / 10^fraction_digits`, then `*= 10^exponent` — via
    `f64::powf` (the same libm `pow` C calls), not `str::parse::<f64>()`,
    so rounding matches bit-for-bit rather than just numerically. Leading
    zeros (`01`, `00`) are rejected, matching the vendored parser's
    `Unexpected '0' before ...` error. Integer accumulation wraps silently
    on overflow via `wrapping_mul`/`wrapping_add`, matching the vendored
    parser's unchecked `integer = integer * 10 + digit` in the (C, or a
    Rust release build) sense — **a differential-test run against
    `json_parse` directly with an out-of-`i64`-range integer literal was
    deliberately excluded from the test suite** after it was found to
    panic-abort the *vendored* parser itself in a debug build (Rust's
    default overflow checks catch the same unchecked multiply/add that's
    silent UB-but-wraps in C); this is a pre-existing latent bug in
    `vendor/json.rs`, unrelated to and not reproduced by the new parser,
    flagged separately rather than fixed here.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 44 pre-existing unit tests plus 6 new
    ones (`every_committed_payload_json_matches` — every `.json` payload
    in `tests/payload/` and `build/` parsed and structurally compared
    against `vendor::json::json_parse`'s tree, key order and duplicate
    keys included; `number_edge_cases_match`; `number_syntax_errors_match`;
    `string_edge_cases_match`, including `\uXXXX` surrogate-pair decoding
    and raw non-UTF-8 bytes; `parser_leniency_quirks_match`;
    `malformed_input_rejected_the_same_way`), all green; every standard
    payload byte-identical in both directions including the `otfccdll`
    cdylib; all 10 round-trip payloads stable; issue #1's regression test
    green — expected, since nothing in the crate calls `parse_json` yet.

- **`JsonValue`'s union payload is no longer read directly outside
  `vendor/json.rs`/`vendor/json_builder.rs` — C-1 of Stage 6-2.5, the
  "make `JsonValue` safe Rust" theme, and the first PR to touch that theme
  at all.** `JsonValue`'s payload is still a raw `union JsonValuePayload
  { boolean, integer, dbl, string, object, array }` reached through `.u.*`
  field access on a `*const`/`*mut JsonValue` — this PR does not change
  that representation at all, only *who's allowed to read it directly*.
  11 new self-guarding accessor functions were added to
  `support/json_funcs.rs` (`json_obj_len`/`json_obj_key_at`/
  `json_obj_key_len_at`/`json_obj_val_at`, `json_arr_len`/`json_arr_at`,
  `json_str_ptr`/`json_str_len`, `json_int_val`/`json_dbl_val`/
  `json_bool_val`), and all 433 direct union-field reads across 33 parse-
  side consumer files were replaced with calls to them — a pure,
  behavior-preserving textual substitution, not a restructuring. The
  point: once `JsonValue`'s representation itself becomes a safe enum
  (C-2), only these 11 functions and `vendor/json.rs` need to change —
  every one of the 33 consumer files converted here is already finished
  with respect to that future step.
  - **Why this had to come before the representation change, not after.**
    A prior investigation (recorded in this same "Next steps" section
    before this PR) established the crate's decisive split: every
    `dump.rs` in the tree goes through the builder API in
    `vendor/json_builder.rs` and never touches the union directly, while
    every direct union read lives on the parse side. That split makes it
    possible to convert parse-side *access* first, independent of
    *representation*, and defer the representation change (and the
    build-side equivalent) to later, smaller, more isolated PRs — the
    same reasoning the B-3 series used to convert one field at a time
    instead of one giant `Subtable` PR.
  - **The accessors are thin and self-guarding, not zero-cost
    reinterpretations.** Each one repeats the same null/type check this
    crate's existing accessors (`json_obj_getbool`, `json_numof`, etc.,
    already in `json_funcs.rs`) already used — `json_obj_len` returns 0
    for a null or non-`Object` value rather than reading garbage,
    `json_str_ptr`/`json_str_len` accept `String` *and* `PreSerialized`
    (the same payload, retagged — see `preserialize`) — so a few sites
    that used to skip a redundant type check some helper already made
    are now technically doing that check twice. Free once inlined, and
    it means the accessors are *safer* than the raw reads they replace,
    not just differently spelled: `table/vdmx/funcs.rs`'s ratio-array
    loop, for instance, used to dereference `(*_ratios).u.array.length`
    with no null check on `_ratios` at all (a latent null-deref on a
    missing `"ratios"` key) — `json_arr_len`'s self-guard fixes that
    incidentally, not as this PR's goal but as a side effect of routing
    through a helper that already had to handle it.
  - **`json_funcs.rs`'s own pre-existing helpers were rewritten to use
    the new primitives too**, not just left as a second population of
    direct union readers sitting next to the new ones — `json_obj_getbool`,
    `json_obj_get`, `json_obj_getsds`, `json_obj_getstr_share`,
    `json_numof`, `json_boolof`, `json_obj_getnum_fallback`,
    `json_obj_getint_fallback`, and `otfcc_parse_flags` all now call the
    new accessors internally. The result: **every** direct `.u.*` read
    in the entire crate outside `vendor/json.rs`/`vendor/json_builder.rs`
    lives in exactly 11 functions, all in this one file.
  - **One non-mechanical fix, caught during conversion, not assumed
    away.** `table/otl/subtables/gsub_ligature.rs`'s legacy (no
    `"substitutions"` key) parse branch reads `(*_subtable).u.array.length`
    on a value whose real type is `Object` — valid C because
    `JsonArrayValue.length` and `JsonObjectValue.length` share the same
    offset and type, so the union aliases correctly by accident. A blind
    text substitution to `json_arr_len` would have returned 0 always
    (wrong type check) and silently dropped every font using that legacy
    JSON shape; it was converted to `json_obj_len` instead, matching the
    value's actual type and the object-style access the very next lines
    already use on the same value.
  - **Two destructive free-and-replace sites, deliberately left alone.**
    `table/glyf.rs` (glyph-by-glyph parse) and `table/otl/parse.rs`
    (`feature_merger_activate`'s duplicate-merge pass) each free a JSON
    subtree mid-walk and write a fresh value back into the same
    `JsonObjectEntry` slot in place. The *read* side of each site (the
    key/value being examined before the free) was converted normally;
    the raw pointer arithmetic that locates the slot being *written to*
    was left untouched, since no accessor exists yet for mutating a slot
    in place — that's C-2/C-3 territory, once the representation itself
    can express "replace this object member" safely.
  - Verified with the standard full pipeline on both platforms (macOS
    arm64 and the Linux container): 44/44 unit tests, ABI at exactly 4
    symbols, every standard payload byte-identical in both directions
    including the `otfccdll` cdylib, all 10 round-trip payloads stable,
    and issue #1's regression test green — expected, since this PR
    changes zero bytes of representation or logic, only which function
    reads which union field.

- **`GposPairSubtable` is fully Box-ified now — `first`/`second: *mut ClassDef`
  become `Option<Box<ClassDef>>`, `first_values`/`second_values: *mut *mut
  PositionValue` become `Vec<Vec<PositionValue>>` (B-3-7 of the
  remaining-three-themes plan, the seventh and last of 7 raw-pointer fields).
  This closes out the whole B-3 theme: every `XxxSubtable`/`XxxEntry` field
  that used to hold a raw pointer into its own heap allocation now owns it
  through `Vec`/`Box`/`Option`.** The largest of the 7 by every measure —
  4 fields, ~130 sites in `table/otl/subtables/gpos_pair.rs` alone (987
  lines) plus `consolidate/otl/gpos_pair.rs` — and the first to introduce a
  new bridging idiom rather than reusing `coverage_from_raw`.
  - **`classdef_from_raw`, `coverage_from_raw`'s sibling.** `ClassDef` (used
    for the outer `first`/`second` glyph-class tables) is a plain struct, not
    a `Vec` type alias like `Coverage`, and — unlike every prior B-3 field —
    is legitimately `Option`al: `parse_class_def` returns null on a
    non-object JSON value, and callers already checked `.is_null()` for it.
    `classdef_from_raw(raw: *mut ClassDef) -> Option<Box<ClassDef>>` added
    to `classdef.rs` right after `otl_class_def_create`, same `ptr::read` +
    `free()` shape as `coverage_from_raw` (never `Box::from_raw` on a
    `malloc`'d pointer — the same allocator-mismatch hazard this migration
    has avoided throughout). `ChainingRuleSet.bc`/`.ic`/`.fc: *mut ClassDef`
    stay untouched raw pointers — this PR only converts the two `ClassDef`
    fields living inside `GposPairSubtable` itself, not every raw-pointer
    `ClassDef` in the crate.
  - **The reverse bridge doesn't exist, on purpose.** `expand_class_def`
    consumes a `*mut ClassDef` it's handed and internally `free()`s it before
    returning a new one — feeding it a `Box`-derived pointer would `free()`
    Rust-allocator memory through libc, exactly the mismatch `coverage_from_
    raw` was designed to avoid. `otl_read_gpos_pair`'s Format 2 branch (which
    calls `expand_class_def`) keeps `first` as a plain local raw pointer
    (`first_raw`) through that one consuming call, and only wraps it with
    `classdef_from_raw` into `(*subtable).first` once settled — never
    constructing a `Box` that a C-side `free()` could reach. Format 1 (which
    never calls a consuming function) adopts immediately after construction,
    matching every earlier field's timing.
  - **Adoption timing had to match the original's exactly, not just produce
    the same end state.** Both read functions have several early-exit paths
    (length-check failures) that fall through to `I_SUBTABLE_GPOS_PAIR.free`
    without reaching the success return. The original assigned `(*subtable)
    .first` immediately at construction, so that free correctly disposed a
    half-built subtable on every exit path. Deferring adoption to a purely
    local variable until the function's end (the simpler-looking option)
    would have leaked `first`/`second`/both value grids on exactly those
    paths — so both fields are adopted into `(*subtable)` as soon as they're
    fully constructed, with a raw-pointer alias (`first_cd`, `second_cd`)
    derived once via `.as_deref_mut().unwrap()` for every read/write after
    that, the same "derive an alias once, mutate through it" idiom used for
    every Box-reached field so far in this migration.
  - **`first_values`/`second_values` collapse from a manual 2D
    `__caryll_allocate_clean` grid to one `vec![vec![..]; ..]` line where the
    fill is out-of-order, or to `.push()`ed rows where it's exhaustive and
    sequential.** Format 1's real-value pass indexes by `cid` from an
    `IndexSet` lookup (not sequential), so the grid is pre-sized with
    `position_zero()` placeholders and index-assigned — the same "pre-size
    for out-of-order `Copy` fills" shape as B-3-1/B-3-5. Format 2 and
    `otl_gpos_parse_pair`'s JSON-matrix fill are both exhaustive nested loops
    in strict row-then-column order, so their grids are built directly with
    `.push()`, no placeholder pass needed at all — simpler than either of
    Format 1's two passes.
  - **`IndividualGposPair.fv`/`.sv` changed from `*mut PositionValue` to
    `PositionValue`.** These used to point *into* the `first_values`/
    `second_values` grid (an address-of-a-cell, valid only as long as the
    grid outlives the pointer, true within a single build pass). Since
    `PositionValue` is `Copy` and the grid is a real `Vec<Vec<..>>` now, the
    matched cell's value is just copied out at collection time instead —
    removing the last per-element pointer in the file, and removing the only
    place where the grid's address needed to stay stable across the sort.
  - **The vtable's `.copy` slot (`subtable_gpos_pair_copy`) was a whole-struct
    `memcpy` — unsound the moment any field owns a `Vec`/`Box` (aliases two
    owners onto one allocation).** Confirmed via the same file-boundary
    reachability check as `otfcc-stage6-vtable-copy-move-mostly-dead`:
    `I_SUBTABLE_GPOS_PAIR.copy` has no callers anywhere in the crate, so
    rather than preserving memcpy semantics behind a hazard nothing
    exercises, it was rewritten to the simplest correct body — a field-wise
    `.clone()` (`ClassDef` already derives `Clone`; `Vec<Vec<PositionValue>>`
    clones directly since `PositionValue` is `Copy`).
  - `cov_from_cd`'s parameter tightened from `*mut ClassDef` to `*const
    ClassDef` (it never mutates its argument) to avoid a `*const` -> `*mut`
    cast at both of its call sites now that `first_cd`/`second_cd` are
    naturally `*const ClassDef` in the two build functions.
  - `#[derive(Copy, Clone)]` on `GposPairSubtable` dropped to `#[derive(
    Clone)]`, the same change every other B-3 field's enclosing struct
    needed once it stopped being trivially copyable.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 44/44 unit tests, ABI at exactly 4 symbols,
    every standard payload byte-identical in both directions including the
    `otfccdll` cdylib, all 10 round-trip payloads stable, and issue #1's
    regression test green. `gpos_pair` has real coverage in the standard
    payload set (`BungeeColor-Regular_colr_Windows`, `Molengo-Regular` both
    carry real `gpos_pair` lookups), and `otfcc_build_gpos_pair` always
    exercises both `otfcc_build_gpos_pair_individual` and `_classes` on
    every build regardless of which wire format a subtable was originally
    read from, so both code paths this PR touches are exercised by the
    existing payload set rather than needing a dedicated forced-dedup
    payload.

- **`ChainingRule.match_0` is `Vec<Coverage>` now, not `*mut *mut Coverage`
  (B-3-6 of the remaining-three-themes plan, the sixth of 7 raw-pointer
  fields) — and its custom `Drop` impl is gone entirely.** Spread across 7
  files (`table/otl.rs`, `chaining/{read,parse,build,classifier,dump}.rs`,
  `consolidate/otl/chaining.rs`), the widest-reaching of the 7 fields so far
  by file count, though every site was one of the same few mechanical shapes
  established in B-3-1..5.
  - **The biggest `Drop` deletion in the B-3 series.** `ChainingRule` used to
    need a hand-written `impl Drop` solely to free `match_0`'s per-element
    `Coverage`s and its own backing array (`apply` already self-dropped from
    an earlier PR). With `match_0` now a real `Vec<Coverage>`, both fields
    self-drop and the whole `impl Drop for ChainingRule` block is deleted —
    compiler-generated glue takes over completely. `close_rule` (used for
    the `Canonical` variant's `ManuallyDrop<ChainingRule>`, never
    `Box`-owned) needed no change at all: it already ran the type's `Drop`
    glue via `ptr::drop_in_place`, whatever that glue happens to be.
  - **Two construction shapes, same helper.** `chaining/read.rs`'s two rule
    builders (`general_read_contextual_rule`, `general_read_chaining_rule`)
    fill `match_0` in strict sequential order, so both switched from
    `__caryll_allocate_clean` + offset-indexed writes to
    `Vec::with_capacity` + `.push(coverage_from_raw(...))` — reusing B-3-2's
    helper again — which made the `jj`/`j`-counter variables in both
    functions fully redundant (confirmed via grep: never read for anything
    but indexing) and let them be deleted outright.
    `chaining/classifier.rs`'s `build_rule` is the same sequential shape,
    filling from already-in-memory data rather than parsed JSON.
  - **The `malloc`-not-`calloc` trap, recognized and handled the same way as
    B-3-5, twice in one function.** `chaining/parse.rs`'s
    `otl_parse_chaining` constructs `rule` as a raw pointer into a
    `ManuallyDrop<ChainingRule>` union arm inside a `malloc`+`memset`-zeroed
    `ChainingSubtable`. The existing code already used
    `::core::ptr::write(&raw mut (*rule).apply, ...)` for the adjacent
    `apply` field for exactly this reason (memset-zeroed bytes aren't a
    valid `Vec` bit pattern, so plain `=` would try to drop garbage first);
    `match_0` got the identical treatment,
    `::core::ptr::write(&raw mut (*rule).match_0, Vec::with_capacity(...))`,
    right next to it.
  - **Two independent `reverse_backtracks` functions, same file-local name,
    same simplification.** `chaining/read.rs` and `chaining/build.rs` each
    define their own `reverse_backtracks(rule: *mut ChainingRule)` (2 and 2
    call sites respectively, none crossing the file boundary) — both were
    manual meet-in-the-middle index-swap loops over `*mut *mut Coverage`,
    both collapsed to a one-line slice reversal now that `match_0` is a
    `Vec`. Neither needed a signature change, unlike B-3-5's version (which
    took the array pointer directly rather than the owning struct).
    `dangerous_implicit_autorefs` required an explicit `&mut` around the
    field before slicing — `(&mut (*rule).match_0)[..input_begins]
    .reverse()` — the same shape used everywhere else in this migration when
    a `Vec` is reached through a raw-pointer deref.
  - **`chaining/build.rs` and `chaining/classifier.rs`'s `(**(*rule)
    .match_0.offset(n)))[0]` reads collapse to plain double-indexing.**
    `match_0: *mut *mut Coverage` meant reaching an element needed two
    dereferences past the offset; `Vec<Coverage>` (`Coverage = Vec
    <GlyphHandle>`) makes the same read `(&(*rule).match_0)[n as usize][0]`
    — one indexing operation per dimension, no pointer arithmetic. The
    `OTL_I_COVERAGE.build`/`.dump` vtable calls (which take `*const
    Coverage`) and the `class_compatible`/`fontop_consolidate_coverage`/
    `shrink_coverage` calls (which take `*mut Coverage`) both needed a
    single element reference derived explicitly (`&(&(*rule).match_0)[n] as
    *const Coverage` / `&mut (&mut (*rule).match_0)[n] as *mut Coverage`),
    the same explicit-reference-before-raw-cast idiom used throughout this
    series to satisfy `dangerous_implicit_autorefs`.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 44/44 unit tests, ABI at exactly 4 symbols,
    every standard payload byte-identical in both directions including the
    `otfccdll` cdylib, all 10 round-trip payloads stable, and issue #1's
    regression test green. No dedicated `chaining` dedup payload exists
    among `rust/scripts/make-test-*-dedup.py`, but real coverage isn't
    thin here — `NotoNastaliqUrdu-Regular` and `iosevka-r` both carry
    `gsub_chaining`/`gpos_chaining` lookups in the standard payload set,
    exercised byte-identically by `compare-with-c.sh` on both platforms.

- **`GsubReverseSubtable.match_0` is `Vec<Coverage>` now (was `*mut *mut
  Coverage`) and `.to` is `Coverage` (was `*mut Coverage`) — B-3-5 of the
  remaining-three-themes plan, the fifth of 7 raw-pointer fields.** Reused
  `coverage_from_raw` (B-3-2) for every construction site; `match_0`'s slots
  are filled out of sequential order (backtrack positions, then the input
  glyph at `input_index`, then forward positions), so it's pre-sized with
  placeholder empty `Coverage`s and index-assigned, the same shape B-3-1's
  `parse_bases` needed.
  - **`reverse_backtracks` collapsed to one line.** Its manual
    meet-in-the-middle index-swapping loop over `*mut *mut Coverage` was
    always exactly `[T]::reverse` on the backtrack sub-slice — `Vec<Coverage>`
    makes that literal: `match_0[..input_index as usize].reverse()`. Retyped
    its parameter from a raw pointer to `&mut [Coverage]`, the only one of
    the 7 fields' helper functions in this series whose *signature* changed
    rather than just its body, since nothing else calls it (2 sites, both in
    this file) and the safe-slice version is strictly simpler than any raw
    pointer arithmetic version would be.
  - **The `malloc`-not-`calloc` trap, hit for real this time.**
    `subtable_gsub_reverse_create` mallocs (not calloc's) the shell, and
    `init_gsub_reverse` used to write null pointers into `match_0`/`to` —
    a plain field assignment, harmless for raw pointers. With `Vec` fields,
    an `=` assignment into that same uninitialized memory would first try to
    *drop* whatever garbage bytes were already sitting there — this is
    exactly the `otfcc-vec-field-assign-needs-calloc` hazard from earlier in
    this migration, caught before shipping rather than after: `init_gsub_reverse`
    uses `.write()` (placement construction, no drop of the old
    non-existent value) instead of `=`, matching the pattern
    `otl_coverage_create`'s own comment already documented for exactly this
    reason.
  - **Same cascade as B-3-1, but this time it reaches the top.**
    `GsubReverseSubtable` has exactly two fields that used to need manual
    disposal, and both now self-drop — nothing else in the struct owns
    anything. `Subtable::drop`'s `GsubReverse` arm becomes a no-op.
    `dispose_gsub_reverse` itself stays (its body simplified from a manual
    per-element free loop to two `= Vec::new()` assignments) because it
    still has a real job: freeing the not-yet-adopted-into-the-enum
    malloc'd intermediate between `_create()` and `subtable_from_raw`, where
    a raw `free()` would skip `Vec`'s drop glue entirely. `#[derive(Copy,
    Clone)]` dropped to `#[derive(Clone)]` (a union-embeddable struct's old
    requirement; `Copy` isn't possible once two fields own `Vec`s).
  - `otfcc_build_gsub_reverse` takes `*const Subtable` but calls
    `reverse_backtracks` (which mutates `match_0` in place, sorting
    backtracks into wire order) — pre-existing behavior, unchanged by this
    field's type. Cast away constness for just that one call, the same
    `fd_to_json`-style const-cast this migration has used before when a
    build pass is documented to be the only thing touching a value.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 44/44 unit tests, ABI at exactly 4 symbols,
    every standard payload byte-identical in both directions including the
    `otfccdll` cdylib, all 10 round-trip payloads stable, issue #1's
    regression test green, and the `gsub-reverse` dedup payload (no
    committed font payload has a `gsub_reverse` lookup at all, and this one
    also exercises `reverse_backtracks` through a real build pass)
    byte-identical against the C reference on both platforms.

- **`LigatureBaseRecord.anchors` is `Vec<Vec<Anchor>>` now, not `*mut *mut
  Anchor` (B-3-4 of the remaining-three-themes plan) — the first genuinely
  2D field of the 7.** `anchors[component][class]`: outer dimension sized by
  `component_count`, inner by the enclosing subtable's `class_count`.
  `Anchor` is `Copy` (as established in B-3-1), so both dimensions are plain
  `Vec`s with no nested pointers or per-element free once built — `Vec<Vec
  <Anchor>>` maps the shape directly, no `Option` needed at either level
  (an absent outer array is just an empty `Vec`, exactly what
  `component_count == 0` already meant).
  - **Same cascade as B-3-1, one level up.** `anchors` and `glyph:
    GlyphHandle` both self-drop, so `delete_lig_array_item` is deleted
    outright and `dispose_lig_array` collapses to `*arr = Vec::new()`
    (kept — `consolidate/otl/mark.rs`'s dedup pass still clears an in-place
    array mid-function, the same real caller B-3-1's `dispose_base_array`
    had). One level up, `mark_array` already self-dropped before this PR, so
    with `lig_array` now self-dropping too, `dispose_mark_to_ligature`
    becomes pure dead weight — deleted, `Subtable::drop`'s
    `GposMarkToLigature` arm becomes a no-op, `subtable_gpos_mark_to_ligature_free`
    switches to the `ptr::read` + `drop` + `free` idiom. `component_count`
    itself was left as a separate field rather than folded into
    `.anchors.len()` — same call as B-3-1's `class_count`, and here doubly
    so: the read path sets `component_count` *before* `anchors` is built, so
    collapsing them would mean restructuring construction order, not just
    swapping a field for a method call.
  - **Two sites needed pre-sizing instead of `.push()`**, same reason as
    B-3-1's: `parse_bases`'s JSON-driven construction fills each component's
    class slots by `class_id` out of key order, so each inner `Vec` is built
    via `vec![otl_anchor_absent(); class_count]` up front, then indexed —
    not appended to.
  - The dump function's nested `.offset(k).offset(m)` reads needed a `&Vec
    <Vec<Anchor>>` hoisted once per outer loop iteration (`dangerous_implicit_autorefs`,
    same lint hit repeatedly in the CFF fd_array work) rather than an inline
    `(*base).anchors[k][m]` — indexing through a raw-pointer deref twice in
    one expression is exactly what that lint exists to catch.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 44/44 unit tests, ABI at exactly 4 symbols,
    every standard payload byte-identical in both directions including the
    `otfccdll` cdylib, all 10 round-trip payloads stable, issue #1's
    regression test green, and the `mark-consolidate` dedup payload (which
    exercises both the mark-to-base *and* mark-to-ligature dedup paths in
    one forged font — the log output for this field literally reads
    "Ignored anchor double-definition for /uni0302") byte-identical against
    the C reference on both platforms.

- **`GsubLigatureEntry.from` is `Coverage` now, not `*mut Coverage` (B-3-3 of
  the remaining-three-themes plan) — same shape as B-3-2's `GsubMultiEntry.to`,
  in the sibling GSUB subtable.** Reused `coverage_from_raw` (added in B-3-2)
  unchanged; `from`/`to` both self-drop now, so `delete_gsub_ligature_entry`
  is deleted outright and `dispose_gsub_ligature_subtable` collapses to `*arr
  = Vec::new()` — the same cascade as B-3-1/B-3-2.
  - **`subtable_gsub_ligature_replace`, the one live `.replace` slot among
    the `Subtable`-union-blocked containers, needed no change at all**: it
    already disposed the old array and move-assigned a fresh one wholesale,
    which is exactly as correct for `Vec<GsubLigatureEntry>` as it was for
    the raw-pointer-holding version.
  - **`consolidate/otl/gsub_ligature.rs`'s filter pass** (drop entries whose
    glyph is missing or whose shrunk coverage ends up empty, keep the rest)
    used the same "copy the pointer, null the source" idiom the other two
    B-3 PRs replaced with `mem::take` — same fix, same reason.
  - The build function (`otfcc_build_gsub_ligature_subtable`) had the most
    call sites of the three GSUB fields so far — index-0 access twice,
    `.len()` twice, an indexed inner loop — all mechanical `(*(&expr))[..]`
    -> `expr[..]` / `(*(&expr)).len()` -> `expr.len()` unwraps, since `.from`
    is a place now instead of a pointer to dereference.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 44/44 unit tests, ABI at exactly 4 symbols,
    every standard payload byte-identical in both directions including the
    `otfccdll` cdylib, all 10 round-trip payloads stable, and issue #1's
    regression test green. Unlike B-3-2's field, this one has real coverage
    in the standard payload set (`NotoNastaliqUrdu-Regular`/`iosevka-r` both
    have `gsub_ligature` lookups), so no forged dedup payload was needed
    beyond the standard set.

- **`GsubMultiEntry.to` is `Coverage` now, not `*mut Coverage` (B-3-2 of the
  remaining-three-themes plan).** `Coverage` is itself already a bare
  `Vec<GlyphHandle>` (Stage 6-1 legacy), so this field needed no internal
  restructuring at all — only the outer pointer went away. The field is
  never legitimately absent (`parse_coverage` always returns at least an
  empty `Coverage`, never null), so `Coverage` by value, not
  `Option<Coverage>`.
  - **New helper, `coverage_from_raw` (`table/otl/coverage.rs`)**, alongside
    `otl_coverage_create`/`otl_coverage_free`: adopts a
    `otl_coverage_create()`/vtable-`.parse()`-built raw `*mut Coverage` into
    an owned value via the same `ptr::read` + `free` "unwrap_X_table" idiom
    used throughout Stage 6-4. `otl_coverage_create`/`push_to_coverage`
    themselves are untouched — they're generic building blocks used all over
    the OTL code for local coverage construction, not specific to this one
    field, so only the *storage* boundary changed, matching the same
    narrow-scoping this migration used for `Subtable` construction
    (`subtable_from_raw`) in B-1.
  - **Cascaded the same way B-3-1 did.** `to: Coverage` and `from:
    GlyphHandle` both self-drop, so `GsubMultiSubtable` (`Vec<GsubMultiEntry>`)
    needs no per-element destructor at all — `delete_gsub_multi_entry` is
    deleted outright, `dispose_gsub_multi_subtable` collapses to `*arr =
    Vec::new()`.
  - **`consolidate/otl/gsub_multi.rs`'s dedup pass** (first-occurrence-wins
    by `from.index`) used the same "copy the pointer out, null the source"
    two-step B-3-1 replaced with `mem::take` — same fix here, same reason:
    a `Vec` isn't `Copy`, so the naive port would leave two entries owning
    the same buffer.
  - Everywhere else was a mechanical `(*x.to)` -> `x.to` / `x.to as *const
    Coverage` swap, since the shared coverage-table helpers
    (`fontop_consolidate_coverage`, `shrink_coverage`, `OTL_I_COVERAGE`'s
    vtable) still take raw pointers — deriving one at each call site, body
    otherwise unchanged.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 44/44 unit tests, ABI at exactly 4 symbols,
    every standard payload byte-identical in both directions including the
    `otfccdll` cdylib, all 10 round-trip payloads stable, issue #1's
    regression test green, and the `gsub-multi` dedup payload (no committed
    font payload has a `gsub_multiple`/`gsub_alternate` lookup at all, so
    this forged one is the only coverage this field's dedup path has)
    byte-identical against the C reference on both platforms.

- **`BaseRecord.anchors` is `Vec<Anchor>` now, not `*mut Anchor` (B-3-1 of
  the remaining-three-themes plan — first of 7 raw-pointer fields left
  inside individual `XxxSubtable` structs after B-1/B-2 finished the outer
  `Subtable`/`SubtableList` conversion).** `Anchor` (`{ present: bool, x:
  Pos, y: Pos }`) is a small `Copy` struct, and the field was always a
  malloc'd array sized by the enclosing subtable's `class_count` — the
  simplest of the 7 remaining fields, with no nested pointers and no
  per-element free.
  - **Cascaded further than the one field.** Once `anchors` self-drops,
    `BaseRecord` needs no destructor at all (`glyph: GlyphHandle` already had
    one from the Handle pilot) — so `delete_base_array_item` is deleted
    outright. `dispose_base_array` keeps existing (still has a real,
    non-destructor caller: `consolidate/otl/mark.rs`'s dedup pass clears an
    in-place array mid-function, not just at end of scope) but its body
    collapses to `*arr = Vec::new()`, the same one-liner shape established
    elsewhere in this migration. One level up, `GposMarkToSingleSubtable`'s
    other field (`mark_array: MarkArray`) already self-dropped before this
    PR (`dispose_mark_array` was already a no-op `*arr = Vec::new()`), so
    with both fields now self-dropping, `dispose_mark_to_single` itself
    becomes pure dead weight — deleted, with `Subtable::drop`'s
    `GposMarkToSingle` arm becoming a no-op (`Subtable::GposMarkToSingle(_)
    => {}`), the same shape `Extend` already had from B-1.
    `subtable_gpos_mark_to_single_free` (still needed — called from the read
    function's own error path, unrelated to `Subtable::drop`) switches from
    calling the now-deleted dispose function to the `ptr::read` +
    `drop` + `free` "unwrap_X_table" idiom used throughout Stage 6-4.
  - **`consolidate/otl/mark.rs`'s dedup pass** used to copy `anchors`'
    pointer value out of a `BaseRecord` and separately null the source field
    — two independent steps, harmless for a bare pointer. `Vec<Anchor>`
    needs the one atomic operation that's actually correct here:
    `mem::take`, matching the pattern established for exactly this shape in
    B-2.
  - Two sites indexed by a JSON class name rather than filled sequentially
    (`parse_bases`'s random-access write, keyed by `class_id` out of key
    order) needed pre-sizing (`vec![otl_anchor_absent(); class_count]`) rather
    than `.push()` — the one place this conversion wasn't a pure
    `.offset(k) -> [k]` mechanical swap.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 44/44 unit tests, ABI at exactly 4 symbols,
    every standard payload byte-identical in both directions including the
    `otfccdll` cdylib, all 10 round-trip payloads stable, issue #1's
    regression test green, and the `mark-consolidate` dedup payload (which
    exercises this exact field's dedup path — the log output literally reads
    "Ignored anchor double-definition") byte-identical against the C
    reference on both platforms.

- **`SubtableList` is `Vec<Option<Box<Subtable>>>` now, not `Vec<*mut Subtable>`
  (B-2 of the remaining-three-themes plan, following B-1's enum conversion).**
  Every element used to be a bare raw pointer, nullable to represent a
  "removed"/"not yet expanded" hole (consolidation drops a subtable by nulling
  its slot then compacting; `extend`-lookup expansion can leave a hole on its
  rare mismatched-lookup-type error path). `Option<Box<Subtable>>` says the
  same thing in the type: `None` is the hole, `Some` owns the value. `Lookup`
  needed no `Drop` impl of its own even before this — once `SubtableList`
  became this shape, both fields it owns (`name: Vec<u8>`, `subtables`) tear
  down through ordinary compiler-generated field-by-field drop glue,
  recursively; the custom impl this had (calling
  `otl_subtable_list_dispose_dependent`) is deleted along with that function
  and `dispose_subtable_dependent` — both existed only because the elements
  were raw pointers with no self-describing ownership. `SubtablePtr = *mut
  Subtable` stays: individual read/parse/consolidate functions still take and
  return it unchanged, matching the C-derived vtable shapes this migration has
  consistently preserved elsewhere. Only the *container*'s element type
  changed.
  - **New helpers, both in `table/otl.rs`**: `subtable_at(list, idx) ->
    SubtablePtr` reads a slot and panics if it's empty, for the many
    read-only call sites (`build.rs`, `stat.rs`, the chaining classifier) that
    already assumed a slot could never be empty at the point they read it —
    before `Box` made a hole `None` instead of a dangling `*mut Subtable`,
    that assumption being wrong was a silent out-of-bounds-shaped dereference;
    now it's a clean panic. `subtable_list_slot(ptr) -> Option<Box<Subtable>>`
    is the inverse, for the several construction paths whose result can
    legitimately be null (an unrecognised lookup format, truncated data) —
    `Box::from_raw` on a null pointer would itself be UB, so this null check
    is required, not defensive.
  - **`otl/read.rs`'s extend-expansion loop needed `.take()`, not a
    straight port.** The old code read a slot's raw pointer, then separately
    wrote `null` into it — two independent steps, harmless when the element
    type has no destructor. With `Option<Box<Subtable>>`, `.take()` collapses
    that into the one operation that's actually correct: it moves the `Box`
    out and leaves `None` behind atomically, which matters because a plain
    read-then-null (or, worse, a copy without nulling) risks two slots owning
    the same `Box` — a double free the instant either one drops. The
    mismatched-lookup-type branch's scratch `Lookup` — built only to reuse its
    `Drop`-driven teardown on one rejected subtable, then immediately dropped
    — needed no other change.
  - **`consolidate.rs`'s `__declare_otl_consolidation` compaction loop had the
    same hazard**, more subtly: it moves surviving elements toward the front
    of the list by index (`subtables[fresh] = subtables[j]`), then
    `truncate()`s away the tail. On the old `Vec<*mut Subtable>` this was a
    harmless pointer copy — truncated raw pointers were never dropped, so a
    stale duplicate past the truncation point was inert. On
    `Vec<Option<Box<Subtable>>>`, `Vec::truncate` *does* run `Drop` on
    everything it removes — so a naive copy-assign would leave the
    soon-to-be-truncated source slot owning the same `Box` as its new home,
    and truncation would free it out from under the element that's supposed
    to survive. `.take()` at the source avoids this the same way it does in
    `read.rs`.
  - **`unconsolidate_chaining`'s rebuild got simpler, not just retyped.**
    Its own `newsts.push(Box::into_raw(Box::new(Subtable::Chaining(..))))` —
    B-1's construction idiom, needed then because the list still held raw
    pointers — collapses to `newsts.push(Some(Box::new(Subtable::Chaining
    (..))))` once the list holds `Box` directly, dropping the
    into_raw/from_raw round trip entirely. Its final `
    otl_subtable_list_dispose_dependent(..); (*lookup).subtables = newsts;`
    shrinks to the plain assignment alone: replacing a `Vec<Option<Box
    <Subtable>>>` already drops whatever the old one held (correctly
    disposing anything this loop's `Poly`/`Canonical` handling didn't touch,
    e.g. a `Classified` subtable, exactly as the explicit call used to).
  - **`table/otl/dump.rs` keeps its own explicit hole check** (`if let
    Some(sub) = ..`) rather than switching to `subtable_at` — unlike
    `build`/`stat`, it's expected to encounter holes (an `Extend` mismatch can
    leave one) and already skipped them by design; `subtable_at`'s panic
    would be the wrong behavior here, not a tightening of it.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 44/44 unit tests, ABI at exactly 4 symbols,
    every standard payload byte-identical in both directions including the
    `otfccdll` cdylib, all 10 round-trip payloads stable, issue #1's
    regression test green, the lookup-alias regression script green, and all
    six `make-test-*-dedup.py` forged payloads (the same set B-1 used, since
    they cover the exact `SubtableList`-holding lookup types this PR's
    ownership change touches) byte-identical against the C reference.

- **`Subtable` is a tagged `enum` now, not a `union`: done (B-1 of the
  remaining-three-themes plan).** Was an 11-variant `union` with the
  discriminant living *outside* it, in `Lookup.type_0` — every read was a
  pointer-cast (`&raw const/mut (*subtable).field as *const/mut T`), sound
  only because a union's fields all start at offset 0 and `LookupType` was
  trusted to say which one was live. As an enum, the tag lives in the value
  itself, `Drop` (below) replaces two separate `LookupType`-keyed free-function
  tables with one self-describing dispatch, and no variant needs
  `ManuallyDrop` any more (that was purely a union restriction).
  - **Scope grew twice past the mechanical 92-site count the investigation
    found**, both times because the union's "read through a differently-typed
    pointer" trick had spread further than a grep for `.field_name` accesses
    could see:
    - `dispose_subtable_dependent`'s 13-arm `LookupType` match and
      `consolidate.rs`'s `SubtableRemover` (a `*mut Subtable`-typed function
      pointer, registered once per `LookupType`, at 13 call sites in
      `otfcc_consolidate_lookup`) each `transmute`d a `*mut ConcreteType`-typed
      free function to `*mut Subtable` and called it directly — a cast a
      union tolerates and an enum does not. Both are gone: `Subtable::drop`
      dispatches off the enum's own discriminant, so freeing an element is
      `Box::from_raw(ptr)` and nothing else. `SubtableRemover` itself is
      deleted, along with the 13 `transmute` blocks that fed it.
    - **`otf_writer/stat.rs`'s `stat_max_context_otl` was missed by the
      investigation entirely** (three sites: `OTL_TYPE_GSUB_LIGATURE`,
      `OTL_TYPE_GSUB_CHAINING`/`OTL_TYPE_GPOS_CHAINING`,
      `OTL_TYPE_GSUB_REVERSE`), because it reinterprets a `SubtablePtr`
      directly — `(&(*lookup).subtables)[si] as *mut GsubLigatureSubtable` —
      with no `&raw` in sight, a different textual shape than every other
      consumption site in `table/otl/subtables/*.rs`. Confirmed by evidence,
      not inspection: `cargo build` didn't catch it (the cast type-checks —
      it's just wrong), the standard payload set didn't catch it either (its
      three lookup types are all rare enough that none of the eight fonts in
      `compare-with-c.sh`'s payload set has `x_avg_char_width`'s max-context
      computation actually walk one), but `rust/scripts/compare-with-c.sh`
      run against `rust/target/release` (not the stale `bin/release-x64` copy
      of the *C* binary a prior debugging session had left behind, which
      silently never exercised the Rust code at all) segfaulted on
      `iosevka-r.ttf`, `meta-test.ttf`, `vdmx-test.ttf`, and `base-test.ttf`
      — reading a `Vec`'s length out of what was actually an enum
      discriminant. Fixed the same way as every other consumption site: match
      on the variant, don't reinterpret the pointer.
  - **Construction sites split into three shapes**, not one:
    - The common case (most variants): a vtable `_create()` mallocs the
      concrete type, the read/parse function fills it in through
      `(*subtable).field = ...` exactly as before, and only the final `return
      subtable as *mut Subtable;` changes — to `subtable_from_raw(subtable,
      Subtable::Variant)`, a new helper (`table/otl.rs`) that reuses this
      migration's established "unwrap_X_table" idiom (`ptr::read` moves the
      value out, `free` releases just the now-empty shell, the result is
      boxed) with the destination variant supplied as a tuple-constructor
      function value. Null-safe, since a few of these can still fail
      partway through a read and return null — the old cast propagated that
      exactly the same way.
    - `extend.rs`'s `_caryll_read_otl_extend` and `unconsolidate.rs`'s
      `unconsolidate_chaining` (two sites) allocated a whole `Subtable`-sized
      block directly and wrote into `.extend`/`.chaining` *in place* — no
      concrete-type intermediate to adopt. Rewritten to build the
      `ExtendSubtable`/`ChainingSubtable` value as a local first, then
      `Box::into_raw(Box::new(Subtable::Variant(value)))`, matching the
      shape every other construction site now has.
    - `unconsolidate_chaining`'s `Poly`-splitting branch also surfaced a
      genuine (if harmless) pre-existing leak: the old code was
      `free(sub as *mut c_void)` — a raw block deallocation that never called
      `sub`'s own dispose, so its `ChainingRuleSet`'s `bc`/`ic`/`fc` classdefs
      were never freed (only `.rules` was, via an explicit `mem::take` two
      lines earlier). `Box::from_raw(sub)` now runs `Subtable::Chaining`'s
      `Drop` — `otl_dispose_chaining` — which does free them. Output-invisible
      either way (freed memory cannot appear in what `otfccdump` prints, so
      untouched by the byte-comparison tests), a leak fix rather than a
      behavior change worth preserving.
  - **`ExtendSubtable.subtable: *mut Subtable`'s ownership is always taken
    before an `Extend` value is legitimately dropped** — `otl/read.rs`'s
    extend-expansion resolves every `Extend` placeholder to its nested
    subtable (or, on the rare mismatched-lookup-type error path, to a scratch
    `Lookup` that takes over `.subtable` and drops it itself) before the
    shell holding it is ever freed. `Subtable::Extend`'s own `Drop` arm is
    therefore a no-op, matching the old `dispose_subtable_dependent`'s
    behavior exactly (`OTL_TYPE_GSUB_EXTEND`/`OTL_TYPE_GPOS_EXTEND` had no arm
    there either, falling through its `_ => {}`).
  - **`otfcc_build_chaining`/`otfcc_build_contextual` (and the four
    `_classes`/`_coverage` functions under them) were retyped from `*const
    Subtable` to `*const ChainingSubtable`** rather than given the usual
    enum-unwrap line. Their only caller (`chaining/classifier.rs`, both call
    sites) never actually has a `Subtable` — only a `*mut ChainingSubtable`
    that `try_classify_around` may have swapped in for the original — so the
    old `as *mut Subtable` cast at the call site was itself relying on the
    union trick with nothing on either end to unwrap.
  - `SubtableList` stays `Vec<*mut Subtable>` in this PR; giving each element
    a `Box` (dropping the now-redundant manual free calls this PR's `Drop`
    impl already makes correct but not yet exclusively relied on) is
    Stage 6-4's next piece.
  - Verified with the standard full pipeline on both platforms (macOS arm64
    and the Linux container): 44/44 unit tests, ABI at exactly 4 symbols,
    every standard payload byte-identical in both directions including the
    `otfccdll` cdylib, all 10 round-trip payloads stable, issue #1's
    regression test green, and the lookup-alias regression script green.
    Beyond the standard set: all six `make-test-*-dedup.py` forged payloads
    (`gsub-reverse`, `gsub-single`, `gsub-multi`, `gpos-single`,
    `gpos-cursive`, `mark-consolidate` — covering the exact variants this PR
    restructured, several of which no committed font payload exercises at
    all) built byte-identical against the C reference on both platforms.

- **Types out of `pub type X = c_uint`: done, except `ctype_class_bits`.** All
  31 of the classifications c2rust left as integer aliases now have the shape
  they earned, and the shapes are not uniform — which was the point. Twenty are
  real `enum`s (`handle_state`, the ten the crate generates itself,
  `bk_CellType`, `tsi_EntryType`, `json_type`, `byte_types`, the three CFF
  format bytes, `cff_Value_Type`, `json_GlyphOrderPass`); one is a newtype
  (`otl_LookupType`); three are `bitflags` (`glyf_PointFlags`,
  `glyf_ComponentFlags`, `otl_BuildHeuristics`); `ttf_instructions` was deleted
  outright; and the rest turned out not to be classifications at all
  (`MASK_ON_CURVE`, `cff_Type2Limits`, the two operator tables,
  `otfcc_LoggerVerbosity`, `WORD`, `json_uchar` — plain integers of the right
  width). Three aliases are left: `WORD` and `json_uchar`, which are honest
  typedefs, and `ctype_class_bits`, whose twelve constants are the bitmasks
  c2rust's expansion of `<ctype.h>` tests against — nine of them tested by
  nothing at all — so it wants deleting rather than typing.
- **The two CFF operator tables are newtypes: done.** `CffDictOperator` and
  `CffCharstringOperator` were `i32` aliases, which was honest about what they
  are — numbers, not a closed set — but bought none of the one check worth
  having: **38 of the numbers mean one thing in a DICT and something else in a
  CharString** (`OP_NOTICE` and `OP_HSTEM` are both 1; `OP_FD_ARRAY` and
  `OP_HFLEX1` are both 3108). Nothing was ever wrong — the 105 names never
  collide and the two sets are read by disjoint code — but the compiler could
  not see the distinction through an alias, and `cffdict_input_ints(dict,
  OP_HSTEM, ..)` would have compiled. Now it does not. Both are
  `#[repr(transparent)]` newtypes, not `enum`s, for the same reason
  `LookupType` is one: an operator otfcc does not know has to travel through
  unchanged rather than fail to construct.
  - **The inner widths differ, and that is deliberate.** `CffDictOperator(u32)`
    matches the width the dict machinery already used everywhere
    (`CffDictEntry.op`, `parse_dict_key`, `CffGetKeyContext.op`, both extract
    callbacks), so wrapping it **removed** 47 `as u32` casts rather than adding
    any. `CffCharstringOperator(i32)` matches the `i32` arm of
    `CffCharstringInstruction`'s argument union.
  - **The CharString storage field stays a bare `i32`.** That `.i` arm is shared:
    `CffInstructionType::Special` puts non-operator bytes in the very same
    field, so typing the field would have broken the `Special` path while
    looking correct. What the newtype covers is everything *flowing into* it —
    `il_push_op`, `il_matchop`, `zroll`, `opop_roll`, `_il_push_maskgroup`,
    `_il_push_stemgroup`, `cff_get_standard_arity`, `cff_merge_cs2_operator` —
    which is exactly where a DICT operator could have been passed by mistake.
    The three places that read an operator back out of the arm now wrap it
    explicitly at the point where the surrounding `match` has already proved it
    is an operator.
  - **The 105 constant declarations and every `match` body were left alone.**
    Arms stayed bare integer literals; the functions that dispatch on an
    operator (`op_cff_name`, `op_cs2_name`, `cff_get_standard_arity`, both
    extract callbacks) match on `op.0`. Rewriting ~200 arms into
    `CffDictOperator(n) =>` would have been a much larger diff for a table that
    reads worse.
  - **Two functions turned out to be dead**: `op_cff_name` and `op_cs2_name`
    have no callers anywhere in the crate and are not among the four exported
    symbols. Kept rather than deleted — they are libcff's own API, and they are
    the clearest illustration of the split, being the two halves of the name
    table sitting side by side in one file. They now carry the specific types.
  - `the_two_operator_tables_share_numbers` no longer compiles as written:
    `assert_eq!(OP_NOTICE, OP_HSTEM)` is a type error now, which is the check
    this PR bought. Rewritten to reach for `.0` on each side, so the overlap is
    still recorded but saying it takes an explicit unwrap.
  - Verified with the standard full pipeline on both platforms (macOS arm64 and
    the Linux container): 44/44 unit tests, ABI at exactly 4 symbols, every
    payload byte-identical in both directions including the `otfccdll` cdylib,
    all 10 round-trip payloads stable, and issue #1's regression test green.
    Beyond the standard set, `tests/cffdump-opcode-check.js` was run against all
    18 `tests/payload/cffspecial/*.otf` fonts — one per Type 2 arithmetic
    operator (`abs`, `add`, `and`, `div`, `drop`, `dup`, `eq`, `exch`,
    `ifelse`, `index`, `mul`, `neg`, `not`, `or`, `put`/`get`, `roll`, `sqrt`,
    `sub`) — because those exercise the CharString interpreter arms this PR
    retyped and the standard payloads do not reach all of them.
- **`sdslen` is consolidated: done.** The last duplicated `static inline` from
  `sds.h` — 20 identical copies (confirmed byte-for-byte after whitespace
  normalization before deleting any of them), one per file that measured an
  `sds`. `vendor/sds.rs`'s own copy is now `pub(crate)`; the other 19 became a
  `use crate::vendor::sds::{sdslen};` import (merged into each file's existing
  `sdsXxx` import line where one existed). Each file's now-orphaned
  `SDS_TYPE_*`/`SdsHdr*` import (the only reason those names were imported at
  all) was dropped, and `table/cff.rs`'s `json_from_sds` — flagged above as
  blocked on this — now resolves for free. Byte-identical on both platforms,
  no call-site behavior changed (`vendor::sds` was already the sole real
  implementation; this PR only removed the copies).
- **Stage 6-2 phase 1: `sds`'s internal header is a single fixed shape,
  not five variable ones.** The original (redis-derived) `sds` picked among
  `SDS_TYPE_5`/`8`/`16`/`32`/`64` — `#[repr(C, packed)]` header structs of
  increasing width, selected by a tag byte packed into the byte immediately
  before the string data — to save memory across millions of tiny database
  keys. otfcc handles a few thousand strings per font at most, so that
  micro-optimization was pure complexity here with no payoff. Replaced with
  one `#[repr(C)] struct SdsHeader { len: usize, cap: usize }`, unconditionally
  allocated, removing the whole `SDS_TYPE_*` dispatch — `sds_hdr_size`/
  `sds_req_type` and all five header structs are gone, and `sdslen`/`sdsavail`/
  `sdssetlen`/`sdsalloc`/`sdssetalloc`/`sdsnewlen`/`sdsfree`/
  `sds_make_room_for`/`sds_remove_free_space` each dropped their `match flags
  & SDS_TYPE_MASK { ... }` branch in favor of one direct field access.
  `sds_make_room_for`'s "different type, need a fresh allocation and a
  header-byte copy" branch disappears entirely: with one header shape, growth
  is always a plain `realloc`.
  - **Why this is possible without touching any of the ~670 call sites**:
    `SdsRaw` stays `*mut c_char`, still pointing directly at byte 0 of the
    string content, header immediately before it, exactly as before — only
    what's *at* that offset changed shape. This only works because the
    previous `sdslen` dedup (above) had already confirmed, crate-wide, that
    no file outside `vendor/sds.rs` does its own header-relative pointer
    arithmetic; every other file only reaches an `sds`'s metadata through
    this module's functions. A **true** `Vec<u8>`-backed representation
    turned out not to be achievable at this phase for the same reason in
    reverse: a `Vec<u8>`'s own `(ptr, len, cap)` triple doesn't live at a
    fixed offset before its buffer, so making `Sds` a real `Vec<u8>` would
    mean `SdsRaw` could no longer be a bare pointer to the data — which is
    exactly the shape every `#[repr(C)]` struct field typed `SdsRaw` and
    every direct-dereference call site (`.name as *const c_char` passed to a
    C string function, `*s.offset(j)`, `memcmp(s1 as *const c_void, ...)`)
    still assumes. Getting to a genuine `Vec<u8>`/`String` is Stage 6-2's
    later phase, once call sites move off the raw-pointer API entirely and
    `SdsRaw` fields in owning structs become real `Vec<u8>`/`String` fields —
    at which point the header-before-data trick disappears on its own,
    because ownership metadata lives in the struct field itself.
  - **Three more functions turned out to be dead** by the same grep-for-
    callers-both-internal-and-external discipline used throughout this
    migration: `sds_incr_len`, `sds_alloc_ptr`, `sds_alloc_size` (0 callers
    anywhere, including within `vendor/sds.rs` itself) — deleted rather than
    reimplemented against the new header, since rewriting unreachable code
    just to keep it unreachable serves nobody.
  - **New unit tests target exactly what the rewrite could get wrong**:
    zero-fill on a null `init`, embedded-NUL survival across repeated growth
    (`sdscatlen` called 20× to force multiple reallocations), and that
    `sdsdup` doesn't alias the original. The existing `SdsPart`/`sdsbuild!`
    byte-for-byte-vs-`snprintf` tests all still pass unchanged, since they
    only observe `sdslen`/content, never header shape.
  - **Verification carried extra weight here**: this is the first change in
    the migration to touch how *every* string in the crate is stored in
    memory, not just one container type. Full pipeline (byte comparison
    including `NotoNastaliqUrdu-Regular.ttf`, 10-payload round trips — which
    exercise repeated dup/cat/free cycles — and the issue #1 golden test) came
    back clean on both platforms, with no output byte or observable-capacity
    behavior change anywhere.
- **Stage 6-4 pilot: `Handle` owns its name for real, and drops `Copy`.**
  `Handle.name: SdsRaw` was always disposed/duplicated through explicit
  `otfcc_handle_dispose`/`otfcc_handle_dup` calls — the crate never trusted
  `Copy`'s bitwise semantics to do the right thing with it, `Copy` was only
  there so a `Handle` could be read out of a raw pointer without the compiler
  objecting. `Handle` now implements `Drop` (frees `name` if non-null) and
  `Clone` (deep-copies `name` via `sdsdup`, replacing the old `copy_handle`),
  and the `#[derive(Copy, Clone)]` vtable-package (`HandlePackage`/
  `OTFCC_I_HANDLE`) that used to hold C-shaped wrappers around all of this is
  deleted outright — it had zero callers, confirmed the same way as every
  other dead vtable in this migration.
  - **Every struct embedding `Handle`/`GlyphHandle`/`LookupHandle` loses
    `Copy` simultaneously** — there is only one `Handle`, so the moment it
    stops being `Copy`, so does everything containing it. 13 structs across
    6 files (`table/otl.rs` ×9, `table/_tsi.rs`, `table/colr.rs`,
    `table/cmap.rs` ×2) had their `#[derive(Copy, Clone)]` reduced to
    `#[derive(Clone)]`. Two more (`ComponentReference`/`Glyph` in
    `table/glyf.rs`) had already lost `Copy` in the `VqSegList` pilot and
    needed no change; two more still (`ColrMapping`/`CaretValueRecord`)
    already weren't `Copy` from their own earlier PRs.
  - **The compiler turned this from an unbounded audit into a checklist.**
    Removing `Copy` produced exactly 32 `E0507` ("cannot move out of ...")
    errors, all one shape: a `Handle` read out of a raw pointer or a `Vec`
    index to build a *new*, independent entry elsewhere (`otfcc_handle_dup`'s
    universal call pattern) — genuine duplication, not a hidden move. `cargo
    fix --broken-code` applied rustc's own `.clone()` suggestions at all 32
    sites across 17 files mechanically; a second `cargo build` came back at
    zero errors, zero warnings, first try. This is the same "let the compiler
    enumerate every break" method the `VqSegList` pilot used for the same
    reason: raw-pointer reads aren't borrow-checked, but *moving a non-`Copy`
    value out of one* is still rejected at compile time, so the danger this
    conversion carries (an accidental implicit move creating two owners of one
    `name` allocation) can't compile silently — every instance surfaces as an
    error with its own fix already suggested.
  - **What the compiler can't see: raw-pointer *writes* into memory that was
    never a valid `Handle` to begin with.** This is the one category `E0507`
    doesn't catch, because it's not a move-out, it's a move-*in* to
    already-there-but-garbage memory — and it produced a real, if
    intermittent, crash (`___BUG_IN_CLIENT_OF_LIBMALLOC_POINTER_BEING_FREED_
    WAS_NOT_ALLOCATED`, caught by `compare-with-c.sh`, not by `cargo
    test`) that took a macOS crash-report stack trace to pin down, since it
    reproduced reliably inside the verification script but not under a
    directly-invoked debugger. `push_to_coverage`/`push_class_def`
    (`table/otl/{coverage,classdef}.rs`) grow `Coverage.glyphs`/
    `ClassDef.glyphs` — hand-managed `*mut GlyphHandle` arrays, not `Vec`s,
    predating this migration's container work (plan classification "その3") —
    via `__caryll_reallocate`, then wrote the new element with plain
    assignment: `*ptr.offset(new_len - 1) = h;`. Assignment through a
    dereferenced pointer always drops the *old* value at that address before
    writing the new one; for a `realloc`-grown, never-yet-initialized slot,
    that "old value" is uninitialized or leftover bytes from a previous
    allocation, and `Handle::drop` reading its `.name` field as a `SdsRaw`
    and calling `sdsfree` on whatever garbage pointer results in is exactly
    the crash. The fix is `ptr::write` in both places — the same
    placement-construction idiom already used at container-*creation* time
    throughout this migration (`x.write(Vec::new())`, the `GaspTable` calloc
    fix), just newly needed at element-*push* time too, because this is the
    first `Vec`-shaped write pattern applied to a hand-rolled growable array
    of a non-`Copy` element. Grepped for every other `*mut GlyphHandle`/
    `*mut LookupHandle`/`*mut Handle`-typed *field* crate-wide after fixing
    these two (not just usage — the struct fields that own an allocation) and
    confirmed no third instance exists.
  - **The "manual per-element dispose loop" hazard flagged before starting
    this turned out to be a non-issue, for a reason specific to how
    `otfcc_handle_dispose` was already written**: it doesn't just `sdsfree` —
    it also resets `name` to null afterward (`*x = Handle::default()`).
    Every existing container dispose loop (`dispose_mark_array`,
    `dispose_gpos_cursive_subtable`, `table_tsi_dispose`, …) already calls it
    per element before the container itself resets to `Vec::new()`. Once
    every element's `.name` is null, the *automatic* `Drop` that fires later
    when `Vec::new()`'s assignment drops the old backing storage sees a null
    pointer and no-ops — dispose-then-null is inherently double-drop-safe,
    the same property that makes `Option::take()` safe in ordinary Rust. No
    container dispose function needed to change. (They are now *redundant*
    rather than required, since `Vec`'s own drop glue would free everything
    correctly even without the manual loop — but redundant-and-correct isn't
    worth chasing down for its own sake here.)
  - **Verified past the point of trusting a single clean run**: the first
    `compare-with-c.sh` pass after the `E0507` fixes was clean; the *next*
    pass (same binaries, same payloads) crashed on 5 of 10 — the
    non-determinism that's the signature of heap corruption rather than a
    logic bug, and the reason this got a macOS crash report pulled rather
    than accepted as a fluke. After the `ptr::write` fix, `compare-with-c.sh`
    was re-run 3× in a row on macOS and 3× on Linux with no failures, on top
    of the standard single-pass pipeline (byte comparison, 10-payload 5-cycle
    round trips, issue #1 golden test) on both platforms.
  - **Net diff is small — 20 files, +121/−155** — because the `E0507`
    fallout was mechanical and the container dispose functions needed no
    changes at all. The size of *this* PR is not representative of what full
    `Handle` disposal-loop cleanup (removing the now-redundant manual loops)
    or a wider Stage 6-4 sweep (`Coverage`/`ClassDef`/`Glyph` unified onto
    `Box`/`Vec` ownership rather than hand-rolled `malloc`/`realloc` arrays)
    would cost — those remain open, larger follow-ups.
- **Stage 6-4: `Coverage`/`ClassDef`'s hand-rolled `glyphs` arrays become
  `Vec<GlyphHandle>` — the follow-up flagged two entries up.** `Coverage`
  (`num_glyphs`/`capacity`/`glyphs: *mut GlyphHandle`) had no field beyond the
  array itself, so it collapses to `pub type Coverage = Vec<GlyphHandle>`,
  the same "C-native vector shape becomes a bare `pub type`" call as
  `ColrTable`/`TsiTable`. `ClassDef` keeps `maxclass: GlyphClass` as a real
  third field alongside `glyphs: Vec<GlyphHandle>`/`classes: Vec<GlyphClass>`
  — the two arrays always grow together (checked every call site before
  starting), never independently, so one struct with two `Vec`s is the
  honest shape, not two containers plus a length invariant to maintain by
  hand.
  - **Vtable trimming, same two-level check as `Handle`'s Drop/Clone PR
    before it**: `ICoverage` 16 fields → 4 (`dump`/`parse`/`build`/
    `build_format`), `IClassDef` 15 → 5 (adds `shrink`). A slot survives only
    if it's dispatched *through the vtable* somewhere (`OTL_I_COVERAGE.dump.
    expect(...)(...)`) — a backing function can be very much alive by direct
    call (`shrink_coverage(...)`) while its vtable field is completely dead,
    and the two questions have different answers per slot.
  - **`push_to_coverage`/`push_class_def` collapse to `.push()`, which
    removes the `ptr::write` workaround from the `Handle` Drop/Clone PR at
    its root rather than papering over it again.** That PR's crash was a
    `realloc`-grown-but-uninitialized slot receiving a plain `*ptr = h`
    assignment, which drops whatever garbage was already there. `Vec::push`
    never has that failure mode — it manages its own growth and always
    writes into memory it knows is uninitialized — so the class of bug
    disappears rather than needing a second manual fix.
  - **`otl_coverage_create`/`otl_class_def_create` use `malloc` +
    `.write(...)` placement construction, not `calloc`.** Placement-writing
    a whole `Coverage`/`ClassDef` value never reads the destination first, so
    the `GaspTable` calloc lesson (a field *assignment* onto `malloc`'d
    memory drops uninitialized garbage) doesn't apply here — there's no
    field assignment, only a single whole-value write.
  - **`shrink_coverage`/`shrink_class_def` rewritten around `Vec::truncate`,
    which is not just safe but a real (if minor) bug fix.** The original
    compacts live entries to the front and then just decrements the length
    field — the physically-still-allocated tail slots past the new length
    are never explicitly freed, a leak for any entry whose `Handle` was
    filtered out during compaction. `.truncate(k)` runs `Drop` on every
    element beyond `k`, so the same compaction now frees what the C
    original silently kept allocated. Verified this doesn't double-free
    anything by hand-tracing every case truncate's target slot can be in
    (self-assigned, previously-nulled, or a superseded-but-still-valid
    original) before relying on it. `shrink_coverage`'s sort stays a direct
    `libc::qsort` call over the `Vec`'s own buffer (`.as_mut_ptr()`/`.len()`)
    with the unchanged `by_handle_gid` comparator, rather than switching to
    `.sort_by()` — `qsort` is unstable and `.sort_by()` is stable, and this
    sidesteps any risk of an observable ordering difference on duplicate
    keys.
  - **Found and fixed the "manual raw-alloc-then-populate `Coverage`"
    anti-pattern in five places** (`gsub_single.rs`,
    `gsub_reverse.rs`, `chaining/classifier.rs`'s `build_rule`,
    `chaining/read.rs`'s `single_coverage`/`class_coverage`) — code that
    bypassed the container API entirely, calling `__caryll_allocate_clean`
    directly for both the `Coverage` struct and its backing array and
    writing elements by raw offset. Every instance reduces to the same fix:
    `otl_coverage_create()` once, then a loop of `push_to_coverage(...)`
    calls — sidestepping any calloc-vs-malloc placement question by reusing
    the already-correct constructor instead of re-deriving allocation safety
    at each call site.
  - **The "ownership-steal move" flagged before starting
    (`table/otl/subtables/gpos_pair.rs`, format-1 pair adjustment: a freshly
    read `Coverage`'s glyph array is repurposed as a brand-new `ClassDef`'s
    `glyphs`, then the emptied `Coverage` husk is freed) became
    `std::mem::take(&mut *cov)` moving the whole `Vec` into the new
    `ClassDef`'s field, followed by `otl_coverage_free(cov)` on the
    now-empty husk** — `take` leaves a `Vec::new()` behind, so freeing the
    husk afterward is a correct empty-`Vec` drop rather than the double-free
    the naive read would suggest.
  - **The "deliberate read-only aliasing" hazard flagged before starting
    (`table/otl/subtables/chaining/build.rs`, two build functions that
    reuse `OTL_I_COVERAGE.build` to serialize a `ClassDef`'s glyph list by
    constructing a throwaway `Coverage` that aliases its `.glyphs` pointer,
    then `free()`s only the throwaway struct) resolved without constructing
    anything**: `OTL_I_COVERAGE.build`'s backing function takes `*const
    Coverage`, read-only, and `Coverage`/`Vec<GlyphHandle>` are the same
    type now, so `&raw mut (*ic).glyphs as *mut Coverage` — a raw pointer
    straight into the existing field — serves the same purpose with nothing
    to allocate or free afterward. The two `free(coverage as *mut c_void)`
    calls that used to release the throwaway struct's own allocation were
    deleted along with it; keeping either one would `free()` a pointer that
    was never separately `malloc`'d.
  - **`dangerous_implicit_autorefs` fired at a scale this PR didn't expect
    going in — 172 machine-applied edits across 18 files** (`&raw mut/const
    (*ptr)[idx]`-shaped expressions, where indexing through a raw-pointer
    deref needs an explicit reference to avoid the implicit-autoref lint).
    Unlike the `glyf`/`Subtable` PRs, where this was anticipated and budgeted
    for, here it only surfaced once the crate reached zero hard `E0xxx`
    errors — every file this PR touched had compiled clean against `.len()`/
    `[idx]` substitutions individually, so the lint pass across the whole
    crate hadn't run to completion until the very last mechanical fix landed.
    Same fix as before: `cargo build --message-format=json`, extract each
    diagnostic's `suggested_replacement` spans, apply by byte offset
    descending per file so earlier splices don't shift later ones — no
    manual fixups needed this time, unlike the `glyf` PR's double-span
    mismatched-paren cases.
  - **No synthetic payload needed** — every payload already in
    `compare-with-c.sh` exercises `Coverage`/`ClassDef` through GSUB/GPOS
    lookups (single, multi, ligature, reverse chaining, pair, chaining
    classifiers). Full pipeline — build, 44 tests, ABI (4 exports),
    byte-for-byte comparison (re-run 4× on macOS and 3× on Linux, following
    the `Handle` PR's precedent of not trusting a single clean pass for a
    container-lifecycle change), 10-payload round trips, issue #1 golden
    test — came back clean on both platforms every time.
- **Small follow-up: six now-redundant manual `Handle` dispose loops
  retired.** Flagged in the `Coverage`/`ClassDef` PR above as open work —
  every container converted to `Vec<T>` so far that disposes its elements by
  looping and calling `otfcc_handle_dispose(&raw mut (*e).FIELD)` on each one
  before resetting the `Vec` is doing something the `Vec`'s own drop glue
  already does automatically, now that `Handle` has a real `Drop` (the
  `Handle` pilot PR). Scoped narrowly to the container/element pairs where
  the *entire* per-element dtor was Handle disposal and nothing else —
  `GposCursiveEntry`/`GposSingleEntry`/`GsubSingleEntry`/`MarkRecord`
  (single or double `GlyphHandle` field, otherwise plain values) and
  `CaretValueRecord` (`GlyphHandle` + a `Vec<CaretValue>`, which auto-drops
  on its own too) — each collapses to `*arr = Vec::new()` or `.clear()`,
  with the per-element dtor function deleted outright. `TsiEntry` is a
  partial case, kept in scope since the fix is a one-line removal: it also
  owns a raw `content: SdsRaw` that has no automatic drop glue (it isn't a
  `Handle`), so `dispose_tsi_entry` keeps its `sdsfree` call and only drops
  the now-redundant `Handle` line.
  - **Explicitly did not touch every `otfcc_handle_dispose` call site** —
    only ones inside a "loop over a `Vec`, dispose each element, reset the
    container" shape, verified element-type-by-element-type that no other
    field needs a manual free (`BaseRecord`/`LigatureBaseRecord`'s
    `anchors: *mut Anchor`/`*mut *mut Anchor`, `GsubMultiEntry`/
    `GsubLigatureEntry`'s `*mut Coverage` field, `ColrLayer`/`ColrMapping`,
    `ComponentReference`'s `VQ` fields dispatched through `I_VQ.dispose`) are
    all real raw-pointer or vtable-dispatched frees with no automatic drop
    glue yet — none of those were touched. `ColrMapping` in particular looks
    like it should qualify (its `.glyph` and `.layers: Vec<ColrLayer>` would
    both now auto-drop), but its dispose function has a call site in
    `consolidate.rs` outside any container-loop shape that wasn't traced
    before this PR's time budget ran out; left for a future look rather than
    guessed at.
  - **Also fixed the comments this PR's own finding made stale**: several
    files still said a leaf type "stays `Copy` … until Stage 6-4" or that
    skipping a manual dispose loop "would leak" — both written before the
    `Handle` pilot PR actually landed Stage 6-4's `Handle` half. Reworded in
    place rather than deleted, since the underlying point (why no `Clone`
    derive, why the dup function is written by hand) is still correct and
    worth keeping.
  - Zero behavior change, verified with the standard full pipeline (build,
    44 tests, ABI, byte comparison, round trips, issue #1 golden test) on
    both platforms.
- **Follow-up to the follow-up: `ColrMapping`/`ColrLayer`'s dispose functions
  turned out to be fully redundant too, not just the leaf field.** The
  previous PR left this one specifically unaudited (its one call site outside
  `colr.rs`'s own container teardown, `consolidate.rs`'s COLR-decomposition
  path, wasn't traced in time). Traced it here: `m: ColrMapping` at that call
  site is a plain, fully-owned local (built with a struct literal, never
  moved out in the branch that calls `dispose_colr_mapping`), so it was
  already going to be dropped automatically — by the same compiler-generated
  glue that made the `Handle` field itself redundant — the instant the `else`
  block it's in exits. `dispose_colr_layer`/`dispose_colr_mapping` are
  deleted outright (not just their `Handle` line, unlike the leaf-only cases
  in the previous PR): `ColrMapping` holds nothing but a `Handle` and a
  `Vec<ColrLayer>` of the same shape, so *nothing* in either function's body
  survives the same argument. `dispose_colr_table`'s per-mapping loop
  (`colr.rs`'s own teardown path) collapses the same way. The `consolidate.rs`
  call site's fix is a straight deletion, not a substitution — the value
  being manually disposed there was never going to leak or double-free
  either way, since manual dispose followed immediately by scope-end drop
  was always double-drop-safe (same "dispose-then-null" property this whole
  line of cleanup rests on), it was just redundant work.
  - Verified with `BungeeColor-Regular_colr_Windows.ttf` specifically in
    mind — the one existing payload that exercises this exact
    `consolidate_colr` decomposition path, byte-identical across the
    standard full pipeline on both platforms.
- **Same finding, one field over: `ComponentReference`'s `VQ`/`Handle`
  dispose calls are redundant too, not just `Handle`'s.** `VqSegList`'s own
  `Vec` conversion (the `VQ` pilot, much earlier in this migration) already
  gave `VQ` real drop glue, the same way the `Handle` pilot later did for
  `Handle` -- this PR is the first to actually retire a manual dispose call
  built on that fact, because `ComponentReference` is the one struct that
  embeds both. `GLYF_I_COMPONENT_REFERENCE`'s `.dispose` field/backing
  function is deleted outright (its only two callers were a container-loop
  and a `retain_mut` closure, both fixed the same way as the `Handle`
  cleanup PRs), and three more call sites turned up in `consolidate.rs`'s
  glyph-reference-anchoring code once the vtable field was gone and the
  compiler pointed at every remaining `.dispose` reference: each one disposes
  a plain, never-moved owned local (`ref_0`, `rr1`, `gr`, plus the adjacent
  `inner_x`/`inner_y`/`outer_x`/`outer_y`/`rrx`/`rry` `VQ` locals in the same
  function) immediately before the function returns or the enclosing block
  ends -- the same "already about to auto-drop" shape as `ColrMapping`'s
  `consolidate.rs` call site.
  - **Deliberately did not chase this further.** `I_VQ.dispose` has ~20 more
    call sites across `cff.rs`, `libcff/charstring_il.rs`, `glyf/read.rs`,
    and `otf_writer/stat.rs` that were not traced here — some may turn out to
    be the same pattern, some may be genuine reset-and-reuse-in-place calls
    (which a `VQ`'s own drop glue cannot replace, since reusing the same
    binding after "dropping" it isn't valid Rust without reassigning it).
    Each needs the same one-call-site-at-a-time ownership trace as the ones
    fixed here, not a blanket search-and-delete.
  - Also caught: `PointElementInterface`/`ComponentReferenceElementInterface`
    both still declared an `empty`/`dup` pair alongside `init`/`copy` that
    look, from a first pass, like they might be similarly overbuilt — not
    investigated here, since that's a vtable-reachability question (PR
    #50/#51's method), a different kind of check than the dispose-redundancy
    one this PR chain has been doing.
  - Zero behavior change, verified with the standard full pipeline (build,
    44 tests, ABI, byte comparison — 3× on both platforms — round trips,
    issue #1 golden test).
- **Audited the ~20 remaining `I_VQ.dispose` call sites flagged in the
  previous PR, one ownership trace at a time.** The split turned out close
  to even, and confirms the rule the two previous PRs established: a manual
  dispose call is redundant exactly when the value it disposes is a plain
  Rust-owned local or `Vec` element that's about to auto-drop anyway (scope
  exit, function return, or a container reset via `Vec::new()`/`.clear()`),
  and it's still load-bearing when the value is reached through a raw
  pointer into `malloc`'d/`__caryll_allocate_clean`'d memory that gets freed
  with a bare `free()` call — `free()` doesn't run Rust destructors, so
  nothing else would ever free that `VQ`'s backing `Vec`.
  - **Six sites across four files were redundant** (`libcff/charstring_il.rs`,
    two spots in `table/cff.rs`, `table/glyf/read.rs`): all locals declared
    with `let mut x: VQ = …` (or similar), never moved out, disposed
    immediately before the enclosing function returns or the enclosing block
    ends. Each pair collapses to a one-line comment; no logic changed.
  - **Three sites are genuine and untouched**: `table/cff.rs`'s
    `dispose_font_matrix` and `table/glyf.rs`'s `otfcc_delete_glyf_glyph`
    both operate on a `*mut CffFontMatrix`/`*mut Glyph` that a bare `free()`
    call follows a few lines later; `otf_writer/stat.rs`'s two call sites are
    the same `CffFontMatrix` shape. All three are exactly the "raw pointer +
    manual `free()`" case the rule above predicts stays necessary.
  - **`dispose_point`/`glyf_point_dispose` turned out to be simpler than
    either category — plain dead code**, not merely redundant: grepping for
    `GLYF_I_POINT.dispose` and the two function names found no caller
    anywhere (the `Contour = Vec<Point>` conversion, well before this PR
    chain started, already noted in its own comment that no dispose loop was
    needed for `Point`, since it owns no `Handle` — this vtable slot was
    apparently never wired up to begin with). Deleted outright, along with
    the `.dispose` field on `PointElementInterface` and its static
    initializer entry — this is a vtable-reachability finding (PR #50/#51's
    check), the different kind of thing the previous PR's `empty`/`dup` note
    was about, turning up as a side effect of reading through every call site
    rather than a deliberate audit of its own.
  - Zero behavior change, verified with the standard full pipeline (build,
    44 tests, ABI, byte comparison — 3× on both platforms — round trips,
    issue #1 golden test).
- **Followed up on the previous PR's `empty`/`dup` note: `PointElementInterface`
  and `ComponentReferenceElementInterface` both had two dead vtable fields
  each.** Same method as PR #50/#51 — grep every `GLYF_I_POINT.<field>`/
  `GLYF_I_COMPONENT_REFERENCE.<field>` call site (including the split-across-
  lines form the naive same-line grep in the previous PR's investigation
  would have missed, now checked with a small Python scan instead of `grep`)
  and see which fields are ever actually dispatched *through the vtable*,
  independent of whether the backing function is still called directly.
  - **`PointElementInterface`**: `.init`/`.dup` are live (dispatched from
    `table/cff.rs`, `table/glyf.rs`, `libcff/charstring_il.rs`); `.copy`/
    `.empty` are not. `glyf_point_empty` has no caller at all (direct or
    vtable) — deleted outright. `glyf_point_copy`/`copy_point` are *not*
    deleted despite `.copy` being dead, because `glyf_point_dup` (very much
    alive) calls `glyf_point_copy` directly — the same "field dead, backing
    function alive by direct call" split this whole PR series keeps finding.
  - **`ComponentReferenceElementInterface`**: `.init`/`.empty` are live
    (`otf_writer/stat.rs`, `consolidate.rs`, `table/glyf/read.rs`); `.copy`/
    `.dup` are not, and — unlike `Point` — nothing calls
    `glyf_component_reference_dup` directly either, so the whole chain
    (`glyf_component_reference_dup` → `glyf_component_reference_copy` →
    `copy_glyf_reference`) is genuinely dead and all three were deleted, not
    just the vtable fields.
  - Zero behavior change, verified with the standard full pipeline (build,
    44 tests, ABI, byte comparison — 3× on both platforms — round trips,
    issue #1 golden test).
- **Stage 6-4 proper begins: `LangSystemList` → `Vec<Box<LanguageSystem>>`,
  the pilot for the "owned pointer array" shape.** Stage 6-1 deliberately
  stopped at `Vec<*mut T>` for the seven containers in plan classification
  「その3」, deferring the pointee's `Box`-ification to Stage 6-4 on the
  don't-move-three-things-at-once rule `VQ` taught. This is the first of
  those to actually get its elements boxed, chosen as the pilot because it
  is by far the smallest: 3 push sites, 4 read sites, one constructor, one
  dispose function.
  - **`LanguageSystem` gets a real `Drop`** that `sdsfree`s its `name` — the
    only allocation it owns. `required_feature` and `features` both hold
    *borrowed* `*const Feature`s into the same table's `features` list, so
    the `Vec<FeatureRef>` needs no help beyond its own drop glue. With that,
    `Vec<Box<LanguageSystem>>` frees everything by itself:
    `otl_lang_system_list_dispose` collapses to `*arr = Vec::new()` and
    `dispose_language_ptr` is deleted outright.
  - **The `__caryll_allocate_clean`-into-an-out-parameter constructor is
    gone.** `init_language_ptr(&raw mut lang)` becomes `new_language() ->
    Box<LanguageSystem>` — `Box` is the allocation and the struct literal is
    the zero-init the `calloc` used to provide, so the whole
    "calloc-vs-malloc placement" question this migration keeps running into
    stops applying to this type.
  - **The uthash node in `parse.rs` is the interesting case**: it is a
    *transient owner* — `LanguageHash.language` holds the `LanguageSystem`
    while the node is alive, then hands it to `(*otl).languages` before the
    node itself is `free()`d. Kept as a raw pointer (a `Box` field inside a
    `__caryll_allocate_clean`'d node freed with bare `free()` would never
    drop), with `Box::into_raw` at the construction site and `Box::from_raw`
    at the push site making the transfer explicit. Verified first that every
    node reaches that push — the loop that drains the hash pushes each
    `.language` unconditionally before freeing the node, and nothing else in
    the file frees a `LanguageSystem`.
  - **`ScriptStatHash`'s `dl`/`ll` fields became `*const`** (`build.rs`).
    They borrow `LanguageSystem`s out of the table and never free them (only
    `free((*s).ll)`, the pointer *array*), so `*const` is the honest type now
    that the `Vec` owns `Box`es rather than raw pointers — the compiler asked
    for this, and it is the right answer rather than a cast to silence it.
    Element addresses stay stable across `Vec` growth exactly as before,
    since it is the `Box` that moves, not the `LanguageSystem`.
  - Zero behavior change, verified with the standard full pipeline (build,
    44 tests, ABI, byte comparison — 3× on both platforms — round trips,
    issue #1 golden test). `NotoNastaliqUrdu-Regular.ttf` drives script/
    language-system reading, consolidation, dumping and building end to end,
    so no synthetic payload was needed.
  - **Remaining in this group** (`SubtableList`/`LookupList`/`FeatureList`,
    plus the two non-owning `*const` reference lists that need no change, and
    `GlyfTable`): each is bigger than this one and, unlike `LanguageSystem`,
    their elements own further containers — so they want the same
    one-container-at-a-time treatment rather than a sweep.
- **`FeatureList` → `Vec<Box<Feature>>`, second of the group.** Chosen next
  because `Feature` turned out to be structurally identical to
  `LanguageSystem` — one owned `sds` name plus a `Vec` of *borrowed*
  references (`LookupRefList` here, `FeatureRefList` there) — so the same
  `Drop` impl, the same `new_T() -> Box<T>` constructor shape, and the same
  `otl_T_list_dispose` → `*arr = Vec::new()` collapse all applied unchanged.
  - **One new wrinkle `LanguageSystem` didn't have: feature *aliasing*.**
    `parse.rs`'s hash-dedup pass can register the same `Feature` allocation
    under two different JSON keys — a real entry (`alias: false`) and an
    alias entry (`alias: true`) that copies the same `*mut Feature` into a
    second `FeatureHash` node without taking ownership. Verified before
    touching anything that the push loop only transfers ownership
    (`Box::from_raw`) for the non-alias node, and that neither node's
    teardown path ever touches `.feature` directly (only `.name`/the node
    itself) — so the alias node's copy is safely discarded, never freed
    twice. Same transient-owner treatment as `LanguageHash`: `.feature`
    stays a raw `*mut Feature`, `Box::into_raw` at construction,
    `Box::from_raw` at the one push site.
  - **`feature_is_not_empty`'s signature drops a level of indirection**
    (`*const FeaturePtr` → `*const Feature`) along with the null check that
    went with it — a `Box<Feature>` inside the `Vec` can never be null, so
    the check `!(*r_feat).is_null()` (testing the *pointee*-of-the-pointee
    for null) had nothing left to test.
  - **`feature_index`'s pointer-identity comparison stays valid** across the
    `Vec<*mut Feature>` → `Vec<Box<Feature>>` change: a `Box`'s heap address
    doesn't move when the `Vec` reallocates (only the `Box` handle itself
    does), so `&raw const *element == feature` compares the same addresses
    the old raw-pointer equality did.
  - **A pre-existing leak on three early-return paths disappears for free**:
    `otfcc_read_otl_common`'s feature-parsing loop constructs a `feature`
    before the length checks that can still `break` out of the loop early
    without reaching the `.push()` — in C (and in the untouched `*mut`
    version), that allocation was never freed. With `feature: Box<Feature>`,
    an early `break` now drops it automatically at scope exit. Unobservable
    in output bytes (leaks never are), same category as `shrink_coverage`'s
    `Vec::truncate` fix earlier in this migration.
  - Zero behavior change otherwise, verified with the standard full pipeline
    (build, 44 tests, ABI, byte comparison — 3× on both platforms — round
    trips, issue #1 golden test). `NotoNastaliqUrdu-Regular.ttf` drives
    feature/lookup reading, consolidation, dumping and building end to end,
    so no synthetic payload was needed.
- **`LookupList` → `Vec<Box<Lookup>>`, third of the group — and it surfaced a
  genuine pre-existing double-free bug in the original C source, fixed
  Rust-only by explicit decision.**
  - **`Lookup` can't get the same one-line `Drop` as `LanguageSystem`/
    `Feature`.** Its `subtables: SubtableList` (`Vec<*mut Subtable>`) is a
    genuinely owned container of a `#[repr(C)] union` with no discriminant
    of its own — the live variant is only known via the *enclosing*
    `Lookup.type_0`. Rust's automatic drop glue can't type-dispatch a union,
    so `impl Drop for Lookup` calls the pre-existing
    `otl_subtable_list_dispose_dependent(&raw mut self.subtables, self)`
    helper (which already did exactly this type dispatch for the old manual
    `otfcc_delete_lookup`) before `sdsfree`ing `name`. Everything else —
    `new_lookup() -> Box<Lookup>`, the transient-owner `LookupHash.lookup`
    treatment, `otl_lookup_list_dispose` collapsing to `*arr = Vec::new()` —
    followed the `LanguageSystem`/`Feature` template unchanged.
    `otfcc_delete_lookup` itself (still called directly elsewhere) became
    `if lookup.is_null() { return; } drop(Box::from_raw(lookup));`.
  - **Found while porting the alias handling `Feature` had already taught to
    expect: `LookupHash` has no `alias` field, in the C original, not just
    the Rust translation.** `FeatureHash` (`c/lib/table/otl/parse.c` and its
    Rust translation both) has an `alias: bool` that the final drain loop
    checks (`if (!s->alias) { push }`) before transferring ownership —
    exactly the guard the `FeatureList` PR above relied on to prove no
    double-free. `LookupHash` has no such field in *either* language, and
    its drain loop pushes every node's `.lookup` unconditionally, alias
    copies included. A JSON `"lookups"` object where one entry is a string
    (an alias to an existing lookup, the same shape `"features"` supports)
    therefore pushes the same `Lookup` pointer twice — a double-free on
    disposal in C, and in a naive `Vec<Box<Lookup>>` translation, two
    `Box::from_raw` calls on the same allocation.
  - **Verified empirically, not just by reading, before deciding how to
    fix it**: `rust/scripts/make-test-lookup-alias.py` (new) derives a
    payload from `tests/payload/kltf-bugfont1.json` by turning one
    `GSUB.lookups` entry into a string-valued alias of another. Built with
    the pre-fix baseline (`git stash`) on both implementations: **C
    segfaults (exit 139)**; **Rust hangs at 100% CPU** in an infinite loop
    (the corrupted allocator free list from the double `Box::from_raw`
    never terminates the internal search), had to be killed with `pkill -9`.
  - **Two scope decisions were the user's, not mine, both confirmed via
    `AskUserQuestion` before writing any fix**: (1) investigate and fix
    correctly with a synthetic payload, rather than skip `LookupList` for
    now or add a minimal double-free guard without the full `Box`
    conversion; (2) fix Rust only, *not* the matching C bug — unlike issue
    #1's dual-fix precedent, `c/lib/table/otl/parse.c` is intentionally
    left untouched here.
  - **The fix**: `LookupHash` gained an `alias: bool` field (with an
    in-source comment recording this exact asymmetry and the empirical
    evidence above), set `false` at the real-definition construction site
    and `true` at the alias-construction site, mirroring `FeatureHash`
    exactly. The drain loop's push becomes conditional on `!(*s).alias`,
    same as the feature loop already was. Rebuilt from the pre-fix baseline
    and reran the same payload: **exits 0, deterministic across repeated
    runs, and the alias resolves to the same underlying `Lookup`** (a
    feature that references both the real name and its alias now lists the
    same lookup twice by name, not a phantom duplicate — lookup count in
    the output is unchanged from the un-aliased source).
  - **Because C still crashes on this input, it cannot go through
    `compare-with-c.sh`'s byte-for-byte comparison** (there is nothing
    correct for C to produce to compare against) — the usual
    `make-test-*.py` pattern doesn't apply as-is. Added
    `rust/scripts/test-lookup-alias.sh` instead: a Rust-only regression
    test asserting (a) `otfccbuild` exits 0 on the alias payload, (b) three
    repeated builds are byte-identical (the same kind of determinism check
    that caught the `gasp` uninitialized-`Vec` bug earlier in this
    migration — a corrupt-but-not-crashing build would slip past a single
    run), (c) the lookup count is unchanged by the alias (proving the push
    was actually skipped, not just that nothing crashed), and (d) a
    dump → build → dump round trip is stable. Wired into `test.sh` and into
    CI as its own step, alongside (not inside) `run-cycles.sh`.
  - **Every other call site touched by `Vec<Box<Lookup>>` was the
    established mechanical pattern**: `(&(*container))[idx]` now yields
    `&Box<Lookup>` rather than a `Copy`-able raw pointer, so read-only
    consumers (`otf_writer/stat.rs`'s `stat_max_context_otl`,
    `table/otl/dump.rs`, two sites in `table/otl/build.rs`) take
    `*const Lookup` via `&raw const *(&(*container))[idx]`, matching
    whichever raw-pointer-ness their own `table`/`lookup` parameter already
    had (a `*const OtlTable` parameter cannot yield `&mut`, so those follow
    through as `*const` end to end); mutating consumers use `&raw mut`
    through `&mut`. `_dump_lookup`/`_declare_lookup_dumper` in `dump.rs`
    were retyped from `*mut Lookup` to `*const Lookup` to match — they were
    always read-only despite the C-translated `mut` qualifier.
    `feature_index`'s pointer-identity comparison pattern
    (`&raw const *element == target`) reappears once more, in
    `write_otl_features`'s search for which lookup index a feature's
    `LookupRef` corresponds to (a bare `as LookupRef` cast on a `Box`
    doesn't compile; comparing the two `*const Lookup`s directly does).
  - Zero behavior change otherwise, verified with the standard full
    pipeline (build, 44 tests, ABI, byte comparison, round trips, issue #1
    golden test) plus the new lookup-alias regression test, on both macOS
    and Linux. Merged as PR #77.
- **`GlyfTable` → `Vec<Option<Box<Glyph>>>`, last of the group.** The other
  three (`LangSystemList`/`FeatureList`/`LookupList`) are always
  push-populated, so plain `Vec<Box<T>>` was enough. `GlyfTable` is
  different: `table_glyf_create_n` pre-sizes the table to the glyph count
  up front, and callers (CFF extraction, TrueType `glyf` parsing) fill each
  GID in afterward by index — meaning a slot can legitimately be observed
  unset partway through, and `consolidate_glyf` explicitly patches any GID
  that never got filled with a fresh empty glyph. `Box<Glyph>` can never be
  null, so the element type is `Option<Box<Glyph>>`, not `Box<Glyph>` —
  the one place in this whole "owned pointer array" group where the
  container shape itself had to change, not just the pointee.
  - **`Glyph`'s `Drop` is smaller than `Lookup`'s, for the opposite reason
    `Handle` made `Feature`/`LanguageSystem`'s small**: only `name` (sds)
    and `instructions` (a raw `*mut u8` byte buffer) are manually torn
    down. Everything else auto-drops correctly already —
    `horizontal_origin`/`advance_width`/`vertical_origin`/`advance_height`
    (`VQ`, whose own `I_VQ.dispose` is just `shift = Vec::new()`, exactly
    what a `VQ` field drop already does for free), the four `Vec` fields
    (`contours`/`references`/`stem_h`/`stem_v`/`hint_masks`/
    `contour_masks`, matching the existing `Contour`/`ReferenceList`
    comments from the earlier `glyf` PR), and `fd_select`
    (a `Handle`, which has had a real `Drop` since the crate-wide `Handle`
    conversion — see the Stage 6-2/6-4 investigation notes below).
  - **The construction site keeps its C name.** Unlike `new_lookup`/
    `new_feature`/`new_language` (all newly-private, never called outside
    their own conversion), `otfcc_new_glyf_glyph` is called from three
    other files (`consolidate.rs`, `table/cff.rs`, `table/glyf/read.rs`),
    so it kept its existing name and just changed its return type from
    `*mut Glyph` to `Box<Glyph>` — renaming would have been a pure
    surface-area increase for no benefit.
  - **The "returns `Box`, but the caller keeps mutating through a raw
    pointer for the rest of the function" pattern appears three times**
    (`table/cff.rs`'s `build_outline`, and the two `otfcc_read_*_glyph`
    functions in `table/glyf/read.rs` that just return the finished
    `Box<Glyph>` directly and need no split at all). `build_outline` is the
    interesting one: it stores the glyph into the table *before* the CFF
    charstring interpreter has finished filling it in (the interpreter
    context holds a `*mut Glyph` it mutates incrementally as it walks the
    charstring), so the `Box` is moved into `Some(...)` in the `Vec` slot
    immediately, and a `*mut Glyph` taken from the same local *before* that
    move keeps pointing at the same heap allocation afterward — moving a
    `Box` moves only the handle, never the allocation, the same guarantee
    `feature_index`'s pointer-identity comparison already relied on twice
    in this migration. Assigning into an already-correctly-sized `Vec`
    slot (`table_glyf_create_n` pre-sized it) doesn't reallocate, so this
    is not a dangling-pointer hazard the way it would be if the `Vec`
    itself could still grow.
  - **A second, more subtle case of the same shape survives *inside*
    `get_point_coordinates`/`consolidate_anchor_ref`** (`consolidate.rs`):
    these two mutually-recursive functions resolve anchored component
    references by walking into *other* glyphs' `references` through the
    table and mutating `ComponentReference.is_anchored` in place (cycle
    detection during resolution) — so, despite reading far more than they
    write, `table: *mut GlyfTable` could not be narrowed to `*const`
    the way `otf_writer/stat.rs`'s read-only glyph-stat functions could.
    Confirmed by grepping the full body of both functions for `(*rr).field
    =`-shaped assignments before deciding either way, the same
    read-vs-write audit this whole migration has relied on at every
    `*mut`/`*const` boundary.
  - **`*const`/`*mut` retyping happened at roughly the same scale as the
    `LookupList` PR**, spread across `table/glyf.rs` (dump/parse helpers —
    `glyf_dump_glyph`, `glyf_glyph_dump_contours`/`_references`/
    `_stemdefs`/`_maskdefs` all turned out to take `*mut` only because
    c2rust never distinguishes const-correctness, not because anything
    mutates), `table/glyf/build.rs`, `libcff/charstring_il.rs` (three
    charstring-compiling functions, likewise read-only), `table/cff.rs`,
    `consolidate.rs`, `otf_reader/unconsolidate.rs`, and
    `otf_writer/stat.rs`. Each site's mutability was decided by grepping
    that specific function's body for an actual field assignment through
    the pointer, not assumed from the parameter's original C-translated
    `mut` qualifier — `stat_single_glyph` and `name_glyph_by_hash` in
    particular looked mutable at a glance (both take a `*mut` and touch a
    `ComponentReference` obtained from the glyph table) but turned out to
    only ever read through it.
  - Zero behavior change, verified with the standard full pipeline (build,
    44 tests, ABI, byte comparison — twice on macOS, once on Linux — round
    trips, issue #1 golden test, plus the lookup-alias regression test)
    on both macOS and Linux. `KRName-Regular.otf`'s CFF extraction and the
    TrueType payloads' `glyf` read/build/consolidate paths already drive
    every branch touched here end to end, so no synthetic payload was
    needed. **This completes the "owned pointer array" group** (`LangSystemList`
    → `FeatureList` → `LookupList` → `GlyfTable`) begun after PR #74;
    `SubtableList` stays `Vec<*mut Subtable>` permanently rather than
    joining this group, since `Subtable`'s `#[repr(C)] union` has no
    discriminant of its own and can't be `Box`-owned without the
    `ManuallyDrop` treatment Stage 6-1 already gave it.
- **uthash → real Rust containers: first instance done, real scope of the
  rest measured.** c2rust inlined every `HASH_ADD`/`HASH_FIND`/`HASH_ITER`
  call site's expansion textually rather than sharing one implementation, so
  `vendor/uthash.rs` itself is tiny (44 lines, just the three handle/table/
  bucket structs) while ~24 distinct hash-table *instances* (structs with
  their own `hh: UtHashHandle` field) are scattered across ~19 files, each
  with its own full copy of uthash's insert/find/grow/sort/delete logic
  (the Bob Jenkins hash alone is ~90 lines, repeated at every insert and
  find site).
  - **Investigated as a candidate for a from-scratch `Subtable` union → enum
    conversion first, and set that aside.** The union's per-variant fields
    are allocated at their own trimmed size (`malloc(size_of::<FieldType>())`,
    not `size_of::<Subtable>()`) and the resulting pointer is reinterpret-cast
    to `*mut Subtable` — safe today because access always goes through the
    one field a `Lookup.type_0`-driven caller already knows is live, but
    incompatible with a real `enum` (which needs the *whole* enum's size,
    tag included, to construct any variant at all). Fixing that touches
    all 11 subtables' `_create` functions, not just the ~58 field-access
    call sites `union` → `enum` looks like on the surface — a materially
    bigger job than it first appeared, so it's parked, not attempted here.
  - **Picked up uthash → real containers instead, and the same
    bigger-than-it-looks lesson applied again, for a different reason.**
    This is not a mechanical "same values, different container" swap the
    way `CVecRaw<T>` → `Vec<T>` (Stage 6-1) mostly was: uthash's `HASH_SORT`
    and its "insert or find-and-skip" idiom carry real *behavioral* logic
    (deduplication rules, output ordering) that has to be read and
    understood per instance before it can be preserved, not just
    transcribed. Confirmed by fully tracing one instance
    (`consolidate_gsub_multi`/`GsubMultiHash`,
    `consolidate/otl/gsub_multi.rs`) end to end before writing a line of
    replacement code: it deduplicates by `from`-glyph id (first occurrence
    wins, a later duplicate's already-consolidated `to` coverage is simply
    dropped along with the pre-dedup input `Vec` when it's disposed — not
    merged, not an error), then reads the surviving entries back out
    **sorted ascending by that same id** (a `HASH_SORT` call right before
    the `HASH_ITER`, easy to miss skimming the macro expansion, fatal to
    miss when picking a replacement type).
  - **That last point is why the replacement is `BTreeMap`, not
    `IndexMap`**, despite `IndexMap` being the container this whole
    investigation set out expecting to use (its insertion-order iteration
    is what most of the other ~23 instances' `HASH_ITER`-without-`HASH_SORT`
    call sites actually need — matching the read/dump/build order the JSON
    and binary formats care about, per the `577 order-dependent sites`
    figure in the Stage 6-1 finale notes above). This one instance sorts
    before reading back, so its natural replacement is whichever container
    is *already* sorted by key — a `BTreeMap`, needing no separate sort
    step and no comparator function at all. **The concrete lesson for the
    rest of this theme: check for a `HASH_SORT` before picking a
    replacement container, every time — it will not be the same answer for
    every instance.**
  - **The rewrite is a ~1050-line function collapsing to ~70.** Almost the
    entire original was the inlined Bob Jenkins hash computation, manual
    bucket table creation/growth, and a hand-rolled merge sort — all of
    which simply do not exist as source once `BTreeMap` does the hashing
    (well, ordering) and iteration order internally.
    `GsubMultiHash`/`by_from_id_multi` (the struct and sort comparator) are
    deleted along with it; `consolidate_gsub_alternative` (a one-line
    wrapper calling this same function, since `GSUB_ALTERNATE` reuses
    `GSUB_MULTIPLE`'s subtable shape) needed no changes.
  - **No committed payload has a `gsub_multiple` or `gsub_alternate` lookup
    at all** (confirmed by grepping every payload's dumped lookup `"type"`
    values before starting — the same check that has caught every prior
    coverage gap in this migration), let alone one that exercises the
    duplicate-`from` path specifically. `rust/scripts/
    make-test-gsub-multi-dedup.py` (new) forges one: a single subtable with
    **two JSON object members sharing the same key** ("from" glyph name).
    A genuine JSON object can't have that — Python's `json` module (and
    every conforming parser) silently collapses duplicate keys — but
    otfcc's own vendored parser deliberately does not (`table/otl/parse.rs`
    already relies on this for a different reason, see the "not redundant"
    note on `json_obj_getnum` in the Stage 3 table above), and the
    subtable reader iterates every raw member by index rather than by key
    lookup, so the hand-written duplicate survives parsing as two separate
    entries — the same shape two distinct rules for one input glyph would
    take arriving from a binary font's rule array. Built with the pre-fix
    (`git stash`) baseline and the post-fix crate side by side on the exact
    same generated input: **byte-identical build output and byte-identical
    re-dump**, both keeping the first rule's target and dropping the
    second's, both on the same lookup name the two independently arrive at
    through unrelated auto-renaming. Wired into `compare-with-golden.sh`/
    `generate-golden.sh` alongside `meta-test`/`vdmx-test`.
  - **Also fixed in passing**: `GsubSingleMapHash`
    (`table/otl/subtables/gsub_single.rs`) looked identical to the dead
    vtable-slot pattern this migration has repeatedly found and deleted —
    defined but with zero references in its own file. It survived a closer
    check: `consolidate/otl/{gsub_single,gsub_reverse}.rs` both import and
    genuinely use it (a c2rust dedup artifact — two unrelated C translation
    units apparently declared byte-identical local hash-node structs, and
    the Stage 2 type-dedup pass folded them into one shared definition
    living in a third, unrelated file). Left in place, now with a comment
    explaining why a struct with no local users isn't dead here.
  - Zero behavior change otherwise, verified with the standard full
    pipeline (build, 44 tests, ABI, golden-checksum comparison, round
    trips, issue #1 golden test, lookup-alias regression test) on both
    macOS and Linux, plus the pre-fix-vs-post-fix baseline diff above.
    **This is the first of ~23 remaining uthash instances** — each needs
    the same "read the whole function, check for `HASH_SORT`, verify
    against a synthetic duplicate/edge-case payload where the existing
    corpus doesn't cover one" treatment individually; there is no
    mechanical shortcut across instances the way `CVecRaw<T>` → `Vec<T>`
    mostly was.
- **uthash → `BTreeMap`, second instance: `GposSingleHash`
  (`consolidate_gpos_single`, `consolidate/otl/gpos_single.rs`).** Same
  overall shape as `GsubMultiHash` (dedup by glyph id, `HASH_SORT` before
  reading entries back out, `BTreeMap` for the same reason), confirmed by
  the same full read-the-function-first process rather than assumed from
  the similarity — worth doing anyway, because this instance's
  found-a-duplicate branch is *not* the same as the first one's:
  - **`GsubMultiHash` silently drops a later duplicate; `GposSingleHash`
    drops it too, but logs a warning first** (`"[Consolidate] Detected
    glyph double-mapping about /<name>."`) — a real behavioral difference
    between two instances that looked interchangeable from their shape
    alone, and exactly the kind of thing "read every instance individually,
    no mechanical shortcut" (the closing note on the first instance) was
    warning about. Preserved verbatim: the warning fires from the same
    `if seen.contains_key(&fromid)` branch that used to be uthash's
    `HASH_FIND`-succeeded case.
  - **`NotoNastaliqUrdu-Regular.ttf`** (already in `tests/golden/`) **has
    real `gpos_single` lookups**, so the ordinary (no-duplicate) path was
    already exercised by the existing golden-checksum comparison before
    this PR touched anything — confirmed by running it against the
    unmodified crate first. The *duplicate*-target path, the entire reason
    this uthash table existed, was not: no payload has two rules
    positioning the same glyph within one subtable.
    `rust/scripts/make-test-gpos-single-dedup.py` (new) forges one the same
    way as the `gsub_multiple` script — a hand-written JSON object with two
    members sharing the same key, which otfcc's vendored parser preserves
    (member-by-index iteration) where a conforming parser would collapse
    them. Verified byte-identical build output, re-dump, **and warning
    message** against the pre-fix baseline on this payload before wiring it
    into `compare-with-golden.sh`/`generate-golden.sh` alongside the first
    instance.
  - Zero behavior change otherwise, verified with the standard full
    pipeline on both macOS and Linux, plus the baseline diff above.
    **~22 uthash instances remain.**
- **uthash → `BTreeMap`, third instance: `GposCursiveHash`
  (`consolidate_gpos_cursive`, `consolidate/otl/gpos_cursive.rs`).** Same
  shape again (dedup by glyph id, `HASH_SORT` before reading entries back
  out, `BTreeMap`), confirmed the same way — full read before writing any
  replacement code, not assumed from the previous two instances' shape.
  This one's found-a-duplicate branch logs a warning too, but with its own
  distinct message (`"[Consolidate] Double-mapping a glyph in a cursive
  positioning /<name>."`, not `GposSingleHash`'s `"Detected glyph
  double-mapping"`) — a third instance, a third message, reinforcing that
  each has to be checked rather than assumed to match the last one found.
  `(enter, exit): (Anchor, Anchor)` (both `Copy`) stands in for
  `GposSingleHash`'s single `PositionValue`.
  - `NotoNastaliqUrdu-Regular.ttf` has real `gpos_cursive` lookups, so the
    ordinary path was already covered; `rust/scripts/
    make-test-gpos-cursive-dedup.py` (new, same duplicate-JSON-key
    technique as the first two) covers the duplicate-target path, verified
    byte-identical build output, re-dump, and warning message against the
    pre-fix baseline before joining the golden-checksum pipeline.
  - Zero behavior change otherwise, verified with the standard full
    pipeline on both macOS and Linux. **~21 uthash instances remain.**
- **uthash → `BTreeMap`, fourth instance: `GdefLigCaretHash`
  (`consolidate_gdef`, `consolidate/otl/gdef.rs`).** Only the
  ligature-caret half of `consolidate_gdef` used uthash — the
  `glyph_class_def`/`mark_attach_class_def` consolidation earlier in the
  same function is untouched. Same overall shape (dedup by glyph id,
  `HASH_SORT` before reading entries back out, `BTreeMap`), confirmed by
  the same full read-first process, which turned up two genuine
  differences from the previous three instances rather than the shape
  alone:
  - **No "missing glyph" warning.** All three previous instances log
    `"[Consolidate] Ignored missing glyph /<name>."` when
    `consolidate_handle` fails to resolve a glyph; this one silently skips
    the entry instead. Preserved as-is — confirmed by reading the original
    function, not assumed from the pattern.
  - **A pre-existing leak on the duplicate path disappears for free.** The
    original unconditionally `sdsdup`s the glyph name *before* checking
    for a duplicate, leaking that copy whenever the entry turns out to be
    a duplicate. The rewrite only dups the name when actually inserting
    (first occurrence), reading the un-duplicated name directly for the
    warning message instead — same category of incidental fix as the
    `FeatureList` leak earlier in this migration, invisible in output
    bytes and not chased down as a goal in itself.
  - Duplicate found → logs `"[Consolidate] Detected caret value
    double-mapping about glyph <name>"` (no trailing period, unlike the
    other three instances' messages) then drops, not merges.
  - **`NotoNastaliqUrdu-Regular.ttf`** has real (14-entry) `ligCarets`, so
    the ordinary path was already covered by the existing golden-checksum
    comparison. `rust/scripts/make-test-gdef-ligcaret-dedup.py` (new, same
    duplicate-JSON-key technique) covers the duplicate-glyph path —
    `ligCarets` is a JSON object keyed by glyph name, so the forged input
    gives one glyph two member entries with different caret positions.
    Verified byte-identical build output, re-dump, and warning message
    against the pre-fix baseline before joining the golden-checksum
    pipeline.
  - Zero behavior change otherwise, verified with the standard full
    pipeline on both macOS and Linux. **~20 uthash instances remain.**
- **uthash → `BTreeMap`, fifth and sixth instances: `GsubSingleMapHash`,
  shared by `consolidate_gsub_single` (`consolidate/otl/gsub_single.rs`)
  and `consolidate_gsub_reverse` (`consolidate/otl/gsub_reverse.rs`).**
  Converted together in one PR since both share the same C-side dedup-hash
  node type (a genuine Stage 2 dedup artifact, not a copy-paste — see the
  `GsubSingleMapHash` note on the second instance above) and deleting it
  requires converting both call sites. Read both in full before writing
  either replacement, which is what caught the two differences below —
  they do not actually behave alike despite sharing a hash node type.
  - **`consolidate_gsub_single` is the closest match yet to the shape of
    `handle_from_consolidated` seen so far**: dedup by `from`'s glyph id
    (`BTreeMap`, `HASH_SORT`-equivalent ordering), with a
    `"[Consolidate] Ignored missing glyph /<name>."` warning on either an
    unresolved `from` or `to` handle (checked in that order, matching the
    original), and `"[Consolidate] Double-mapping a glyph in a single
    substitution /<name>."` on a duplicate `from`. **New element not seen
    in any prior instance**: after the loop, if the survivor count is less
    than the original entry count (some entries were dropped to a missing
    glyph or a duplicate), a `"[Consolidate] In this lookup, some mappings
    are ignored.\n"` warning fires — preserved by comparing
    `seen.len() != subtable.len()` post-loop, the `BTreeMap` equivalent of
    the original's post-loop `HASH_COUNT` check.
  - **`consolidate_gsub_reverse` turned up a genuine, if narrow,
    pre-existing memory-safety hazard — not just a behavioral quirk.** Its
    dedup hash node aliased the *original* `SdsRaw` name pointers straight
    out of `from`/`to` (`Vec<GlyphHandle>`, i.e. `Coverage`) rather than
    `sdsdup`-ing them the way `consolidate_gsub_single`'s node does; the
    original C code then truncated `from`/`to` to the survivor count
    *before* reading those aliases back out to build the final handles.
    That ordering was harmless in C (a length-field truncation frees
    nothing), but `Coverage` became a real `Vec<GlyphHandle>` earlier in
    this migration and `Handle` now owns its name via `Drop`
    (`otfcc-stage6-vtable-copy-move-mostly-dead`-adjacent work) —
    truncating first can drop, and free, a *surviving* entry whose
    original index happens to land past the new (shorter) length, leaving
    a still-pending alias in the hash table dangling before the
    write-back loop reads it. Caught by fully reading the function rather
    than assumed from `consolidate_gsub_single`'s shape, then confirmed
    empirically: `rust/scripts/make-test-gsub-reverse-dedup.py` places its
    duplicate glyph at index 0 and 2 of a 4-entry `match[0]` array (not at
    the tail), so the surviving 4th entry is exactly the one truncation
    would have dropped first. Running that payload through the *pre-fix*
    binary did not visibly corrupt the output on this machine — a classic
    non-deterministic UB outcome, plausible here because `free()` doesn't
    clear memory and the two `sdsdup` calls between the truncation and the
    dangling read happened not to reuse that exact block — but it is a
    genuine use-after-free regardless of whether one particular run shows
    it. The rewrite avoids the hazard by construction rather than
    preserving the ordering: `sdsdup` every survivor's name into the
    `BTreeMap` up front (independent, owned copies), then replace
    `from`/`to` wholesale with freshly built `Vec`s from the map, instead
    of truncating the originals in place and writing back into them.
    `consolidate_gsub_reverse` also returns `false` unconditionally
    (unlike every other instance's `subtable.len() == 0`) — preserved
    exactly, a real quirk of the original rather than an oversight.
  - Neither function has an existing payload exercising its *ordinary*
    path, let alone its dedup path — confirmed by grepping every
    committed payload's dumped lookup `"type"` values, zero
    `gsub_reverse` lookups anywhere and zero `gsub_single` payloads with a
    duplicate `from`. `rust/scripts/make-test-gsub-single-dedup.py` (new,
    the established duplicate-JSON-key technique, since `gsub_single`'s
    subtable is a JSON object) and `rust/scripts/
    make-test-gsub-reverse-dedup.py` (new; `gsub_reverse`'s `match`/`to`
    are plain JSON arrays, so no key-uniqueness trick is needed — a
    hand-written array can just repeat a glyph name) both verified
    byte-identical build output, re-dump, and warning text against the
    pre-fix baseline before joining the golden-checksum pipeline.
  - `GsubSingleMapHash` (`table/otl/subtables/gsub_single.rs`) is now
    unused by either consumer and is deleted, along with each file's own
    copy of the `by_from_id` comparator.
  - Verified with the standard full pipeline on both macOS and Linux.
    **~18 uthash instances remain.**
- **uthash → `Vec`, seventh instance: `ScriptStatHash`
  (`write_otl_script_and_languages`, `table/otl/build.rs`) — the first
  instance in this migration where the right replacement is a plain `Vec`,
  not `BTreeMap`.** Self-contained to one file (unlike the `ClassNameHash`
  instances in `table/otl/subtables/gpos_common.rs` and the two
  `gpos_mark_to_{single,ligature}.rs` files, which were surveyed as the
  next candidate and set aside — that hash table is built in one file,
  looked up read-only in two others, and disposed in each of those two,
  a materially bigger unit of work than a single self-contained instance).
  - This function groups a font's languages by script tag (the first 4
    bytes of `language.name`) while writing the OTL `Script`/`LangSys`
    binary tables — not a dedup pass. Reading it in full turned up the
    reason `BTreeMap` (used for every uthash instance so far) would have
    been the *wrong* container here: **the original C never calls
    `HASH_SORT` before its `HASH_ITER`** over this table, unlike every
    prior instance. Output order is insertion order, not tag order.
    `BTreeMap`'s iteration order is always key order, so using it here
    would have silently reordered a font's `Script` table entries whenever
    the source data's own script order isn't already alphabetical.
  - Replaced with a plain `Vec<ScriptGroup>` and a linear "already seen"
    scan by tag instead of `IndexMap` (the container this migration's
    plan names for the ~577 other insertion-order-dependent uthash/output
    sites, per the "genuine library win" survey in this file). The number
    of distinct scripts in a real font is small — typically single
    digits — so an O(n) scan over an O(n)-sized `Vec` costs nothing
    observable, and pulling in a new dependency for a handful of entries
    isn't warranted. `IndexMap` remains the intended tool for the larger
    order-dependent tables (glyph order, lookup order, coverage order)
    noted elsewhere in this file.
  - No warnings anywhere in the original function (confirmed by grepping
    for logger calls in the range) — the simplest instance yet to verify
    for behavior preservation, since there's no message text to match.
  - A later language sharing a script tag with an existing entry, whose
    name is *also* dflt/DFLT, silently overwrites that script's recorded
    default language — the original never guarded against a second
    default and the rewrite doesn't either; not a case worth warning
    about, just preserved as-is.
  - **No new synthetic payload needed.** `iosevka-r.ttf` (already in
    `tests/golden/`) has four distinct scripts (`DFLT`/`cyrl`/`grek`/
    `latn`) in both its `GSUB` and `GPOS` tables, and its `GSUB` table's
    `cyrl` script has *two* languages — `cyrl_DFLT` (the default) and
    `cyrl_SRB ` (a non-default) — exercising both the multi-script
    grouping and the default-plus-others-within-one-script path for real,
    not synthetically. `tests/golden/checksums.sha256` needed no
    regeneration at all for this instance: every existing payload's
    output, including `iosevka-r`, stayed byte-identical to the frozen
    baseline on the first build after the rewrite.
  - `ScriptStatHash` is deleted along with the `use crate::vendor::uthash`
    import and five now-unused `libc` imports (`exit`/`malloc`/`memset`/
    `strlen`, plus `NULL`) that were used only inside the removed
    resize/hash-compute boilerplate; `free`/`memcmp`/`strncmp` remain used
    elsewhere in the file and are kept.
  - Verified with the standard full pipeline on both macOS and Linux.
    **~17 uthash instances remain.**
- **uthash → `HashMap`, eighth instance: `JsonObjEntry`
  (`compare_json_objects`, `support/json_ident.rs`) — the first instance
  in this migration where the right replacement is `HashMap`, not
  `BTreeMap`/`Vec`/`IndexMap`.** `compare_json_objects` (called by
  `json_ident`, which recursively compares two `JsonValue` trees for
  structural equality) never produces ordered output of any kind — it
  only returns a `bool` — so none of the three ordering-sensitive
  containers used for prior instances applies. Values built from this
  object can be arbitrarily large (a glyph-order-sized object, for
  instance), so a linear scan the way `ScriptStatHash` used (small,
  bounded script counts) would be a real algorithmic regression; `HashMap`
  keeps the O(1)-average lookup the original uthash table had.
  - **A key finding changed the whole design, found only by reading the
    insertion loop rather than assuming "duplicate found" always means
    "warn and drop" the way the six `consolidate/otl/*.rs` instances did**:
    this table's insert is guarded by a `HASH_FIND`-before-add with no
    `else` branch — if the key is already present, the loop does nothing
    at all (no warning, no second node, nothing) and moves on. A later
    member of `a` sharing an earlier member's key is silently and
    completely ignored, not merely deduplicated into a second linked
    node the way the consolidate-side instances kept (and warned about)
    duplicates. `HashMap::entry(key).or_insert(...)` reproduces "first
    occurrence wins, later duplicates vanish outright" exactly.
  - Confirmed the "every distinct key must be matched" requirement by
    reading through to the original's disposal loop, which also doubles
    as the final correctness check (`allcheck = allcheck && (*e).check`
    while freeing each node) — replaced by `seen.values().all(|(_,
    checked)| *checked)` after the second loop, with no dispose step
    needed since the `HashMap` (and the `&CStr` keys borrowed directly
    from `a`'s own `JsonValue` tree, not `strdup`'d copies) are dropped
    automatically.
  - **Consequence worth recording, not fixed**: two JSON objects with the
    same length but a *different* multiset of duplicate keys can compare
    `json_ident`-equal to each other, as long as their sets of *distinct*
    keys line up with matching values — because `compare_json_objects`
    only ever tracks one entry per distinct key regardless of how many
    times `a` repeats it. This was true of the original uthash version
    too (a direct consequence of the no-`else`-branch insert just
    described) and is preserved exactly, not tightened.
  - **Real functional coverage, not synthetic**: `json_ident`'s only
    other caller in the crate, `feature_merger_activate`
    (`table/otl/parse.rs`), uses it to detect when two lookups being
    built are structurally identical so one can alias the other instead
    of duplicating it — exactly what `rust/scripts/test.sh`'s
    "lookup-alias regression test" (`GSUB.lookups.lookup_alias_test`
    aliasing `lookup_calt_0`) exercises end to end. That test passing is
    direct evidence this rewrite's equality logic is correct for the
    crate's one real call site; no new synthetic payload was needed.
  - `JsonObjEntry` and the `use crate::vendor::uthash` import are deleted,
    along with `__caryll_allocate_clean`, `NULL`, and the seven `libc`
    symbols `exit`/`free`/`malloc`/`memcmp`/`memset`/`strdup`/`strlen`
    that were used only inside the removed hash boilerplate — nine
    now-unused imports in total; `strcmp` remains used in `json_ident`'s
    string-value
    comparison and is kept.
  - Verified with the standard full pipeline on both macOS and Linux.
    **~16 uthash instances remain.**
- **uthash → `IndexMap`, ninth instance: `FvarMaster`
  (`table/fvar.rs`) — the first genuine use of `indexmap` in this
  migration, and the first instance where an incomplete crate-wide `grep`
  briefly led to the wrong conclusion.** Reading only within `fvar.rs`,
  `.register_region` (the vtable slot that populates this table) appeared
  to have zero call sites — matching the exact shape of a dead vtable
  slot this migration has repeatedly found and deleted (`ScriptStatHash`'s
  `.copy`, the `otl.rs`/glyf `.copy`/`.move_0` sweeps, etc.) — and a first
  pass deleted the whole mechanism on that basis. The build immediately
  failed: `table/glyf/read.rs` calls `TABLE_I_FVAR.register_region` while
  parsing `gvar` tuple-variation headers, a call site outside `fvar.rs`
  that a `fvar.rs`-scoped `grep` never surfaces. Reverted before
  committing; **the fix here is to `grep` the whole crate for a vtable
  slot's call sites, not just the file that defines it** — this migration
  had already grepped crate-wide for every *type* name in each prior
  instance, but this is the first time the search needed to cover a
  *vtable slot field name* too.
  - With that corrected, this table genuinely deduplicates by a
    `VqRegion`'s full byte content (a fixed `dimensions` header plus a
    variable-length trailing `spans` array, allocated as one contiguous
    C "flexible array member" block — `vf/region.rs`, itself not yet
    `Vec`-ified, deliberately left untouched here) rather than by a
    simple key: `glyf/read.rs` registers a freshly built region for every
    `gvar` tuple-variation header, and identical regions (very common —
    many glyphs share the same variation axis span) collapse onto one
    canonical, named "master" (`m1`, `m2`, ... in registration order),
    which `json_new_vq_region` (during dump) and `otfcc_dump_fvar`'s
    `"masters"` object (during dump) both read back out.
  - **No `HASH_SORT` anywhere in this table either** (confirmed by
    `grep`, the same check that mattered for `ScriptStatHash`) — output
    order is registration order, since a master's very name is derived
    from `(*fvar).masters.len()` at insert time. Unlike `ScriptStatHash`,
    though, the number of masters here isn't bounded to a handful: a
    `gvar` table can register a fresh region for every tuple-variation
    header across every glyph, so a linear "already seen" scan (fine for
    a font's small script count) would be a real algorithmic regression
    here. `IndexMap<RegionKey, FvarMaster>` keeps insertion-order
    iteration *and* O(1)-average lookup — the first time this migration's
    long-flagged "`IndexMap` for the larger insertion-order-dependent
    uthash tables" candidate is actually used. `RegionKey` is a thin
    `*const VqRegion` wrapper with a hand-written `Hash`/`Eq` that reads
    the same variable-length byte range the original `memcmp`-based key
    did, so `VqRegion`'s C layout didn't need to change to make this
    work.
  - **A new class of calloc-safety question, resolved conservatively**:
    every prior "malloc → calloc" fix in this migration concerned a
    direct field assignment (`= Vec::new()`) reading calloc-zeroed
    garbage as a `Vec`, which is documented-safe because a zeroed `Vec`
    bit pattern *is* `Vec::new()`. `IndexMap` (a third-party type) has no
    such documented guarantee, and likely holds a `NonNull` internally
    that would be null (an invariant violation) if read from zeroed
    memory. `init_fvar` was already correct here (`table_fvar_create`
    already used `calloc`, inherited from `.axes`/`.instances`), but this
    migration hadn't previously needed to ask whether a *new* field's
    zero-representation was safe to construct-then-drop — `masters` gets
    `::core::ptr::write(&raw mut (*fvar).masters, IndexMap::new())`
    instead of a plain assignment, sidestepping the question entirely by
    never reading (or dropping) whatever calloc left there.
  - **Real functional coverage, not synthetic**: `gvar-test.ttf` (already
    in `tests/golden/`) registers the same region from multiple tuple-
    variation headers — confirmed empirically by counting `"m1"`
    references in its dump (6, one being the master's own key, five
    being `"on": "m1"` reuses) — so both the "new region" and "duplicate
    region, dedup and free" branches run for real on this payload, not
    just the first-registration path. Output stayed byte-identical to
    the frozen golden checksum.
  - `#[derive(Copy, Clone)]` and the `hh: UtHashHandle` field are dropped
    from `FvarMaster` (never copied by value anywhere, only ever reached
    through `*mut`/`*const FvarMaster`); `dispose_fvar`'s manual
    linked-list walk collapses to iterating the drained `IndexMap` and
    calling the same per-entry `dispose_fvar_master` (`sdsfree` the name,
    `vq_delete_region` the region) each entry always needed.
  - This is the crate's first dependency on `indexmap` (added to
    `rust/Cargo.toml`, pulling in `hashbrown`/`equivalent` transitively).
  - Verified with the standard full pipeline on both macOS and Linux.
    **~15 uthash instances remain.**
- **uthash → `BTreeMap`, tenth instance: `SfntTableEntry`
  (`otfcc_sfnt_builder_*`, `font/caryll_sfnt_builder.rs`) — back to
  `BTreeMap` after two straight instances that needed something else.**
  This table collects a font's top-level SFNT tables (`glyf`, `GSUB`,
  `head`, ...) keyed by 4-byte tag while the binary file is being
  assembled. Unlike `ScriptStatHash`/`FvarMaster`, this one's `by_tag`
  comparator **is** fed into a `HASH_SORT` before the table directory is
  written — confirmed by reading `otfcc_sfnt_builder_serialize` in full,
  not assumed from the two immediately-prior instances' shape (which
  would have pointed the wrong way) — and the sort isn't a stylistic
  choice: the SFNT format itself requires the table directory sorted
  ascending by tag. `BTreeMap<i32, SfntTableEntry>`'s always-sorted
  iteration matches this exactly, the same shape as the six
  `consolidate/otl/*.rs` instances earlier in this migration.
  - Deduplicates by tag, first registration wins — a later
    `otfcc_sfnt_builder_push_table` call for an already-present tag just
    frees the newly-passed buffer and returns, **silently, no warning
    logged** (confirmed by reading the insert branch's `else`, not
    assumed) — a third distinct "duplicate" behavior in this migration,
    after "warn and drop" (most `consolidate/otl/*.rs` instances) and
    "drop with no warning at all" (`JsonObjEntry`).
  - `create_segment` returns `SfntTableEntry` by value now instead of a
    separately `malloc`'d node, since entries live directly inside the
    `BTreeMap`.
  - `SfntBuilder.tables` gets `ptr::write`, not a plain assignment, for
    the same reason established for `IndexMap` in the previous instance:
    `BTreeMap` is a `std` type but still has no documented guarantee
    that reading calloc-zeroed bytes as a live, droppable instance is
    safe, and `SfntBuilder` itself is `__caryll_allocate_clean`
    (calloc)-allocated.
  - **Real functional coverage, not synthetic**: every single payload in
    this crate's test suite builds a binary font, so every one of them
    exercises this table's insert-many/sort/serialize path, including
    the `head` table's special-cased `checksumAdjustment` write-back —
    and every one stayed byte-identical to its frozen golden checksum,
    including the `otfccdll` cdylib comparison. No synthetic payload
    needed; this is the most heavily-exercised instance converted so
    far.
  - Verified with the standard full pipeline on both macOS and Linux.
    **~14 uthash instances remain.**
- **uthash → `BTreeSet`, eleventh instance: `LigatureAggregator`
  (`otfcc_build_gsub_ligature_subtable`, `table/otl/subtables/
  gsub_ligature.rs`) — the first instance that isn't a map at all.**
  Reading the whole ~1000-line function (rather than assuming the shape
  from `by_gid` looking like every prior comparator) turned up that the
  `LigatureAggregator` struct carries no data beyond the `gid` it's keyed
  by — no companion value is ever attached to a node after it's found or
  inserted. Its only two uses are (1) building a sorted, deduplicated
  Coverage of "first glyphs" for the ligature lookup, and (2) re-scanning
  the *entire* original subtable, per distinct gid, once to count and
  once to emit each matching ligature rule into that glyph's
  `LigatureSet` — an O(n²) two-pass filter over the original array, not
  a lookup into stored per-gid data. `by_gid` is fed into a `HASH_SORT`,
  so output order is ascending gid, the same shape as the six
  `consolidate/otl/*.rs` instances and `SfntTableEntry`. Since there is
  no value to carry, this isn't a map at all: `std::collections::
  BTreeSet<i32>` (not `BTreeMap`) reproduces "sorted, deduplicated set of
  glyph ids" directly, and the two re-scan passes are kept exactly as
  in the original rather than restructured to look up stored data (doing
  so would be a bigger, riskier rewrite than the uthash removal itself
  called for).
  - No warnings, no dedup-behavior question here at all — a `gid`
    appearing on multiple ligature rules is the *entire point* of the
    grouping (every rule with that starting glyph ends up in its
    `LigatureSet`), unlike every previous instance's "duplicate key"
    question.
  - **Real functional coverage, not synthetic**: `NotoNastaliqUrdu-
    Regular.ttf` (already in `tests/golden/`) has 5 `gsub_ligature`
    lookups and `iosevka-r.ttf` has 1; both stayed byte-identical to
    their frozen golden checksums. No synthetic payload needed.
  - Hit the `dangerous_implicit_autorefs` lint on `(*(&(*subtable))
    [j].from)[0]`-style expressions (`from: *mut Coverage`, so indexing
    through the dereferenced pointer without an explicit intervening
    `&` autorefs across a raw-pointer deref) — same lint, same fix
    (accepting rustc's suggested explicit `&`) as the `otl` pointer-list
    and `glyf` `Vec`-conversion PRs earlier in Stage 6-1.
  - Verified with the standard full pipeline on both macOS and Linux.
    **~13 uthash instances remain.**
- **uthash → `IndexMap`, twelfth instance: `CffSidEntry`
  (`sidof`/`cffstrings_to_indexblob`, `table/cff.rs`) — the second use of
  `indexmap` in this migration, for the same reason as `FvarMaster`.**
  `sidof` interns a string into the CFF string table, returning its SID
  (391 + registration index) — deduplicating identical strings, assigning
  each new one the next sequential SID. Used for both FD-dict metadata
  strings (CID-keyed CFF: `cidRegistry`/`version`/`notice`/... per FD) and,
  for non-CID CFF, **every glyph name in the font** via `cff_make_charset`
  — so, like `FvarMaster`'s regions, the number of distinct strings here
  is not a small bounded count the way `ScriptStatHash`'s script count
  was; a linear scan would be a real algorithmic regression for a
  large-glyph-count non-CID font. `by_sid` is fed a `HASH_SORT`, but
  registration order *is* SID order by construction (`sid = HASH_COUNT`
  at insert time), so `IndexMap`'s native insertion-order iteration needs
  no separate sort step, the same shape as the `FvarMaster` PR.
  - **A new fidelity question, resolved conservatively rather than
    simplified away**: the original's dedup key is `strlen`-bounded (the
    Bob Jenkins hash and the `memcmp` are both driven by `strlen(s)`),
    but the *stored* value is the full sds-length `sdsdup`. Two strings
    identical only up to their first NUL byte would therefore be treated
    as the same string by the original, with only the first's full
    content (NUL and beyond) surviving to the output. This crate's own
    prior notes on `%s`/non-UTF-8 glyph names establish that arbitrary
    byte content in a glyph name is a real, anticipated case here, not a
    hypothetical -- so this rewrite preserves the `strlen`-bounded key
    exactly (`CStr::from_ptr(s).to_bytes()`) rather than switching to the
    full sds length, even though the latter would have been simpler and
    the difference is very unlikely to ever be observed in a real font.
  - `CffSidEntry` and `by_sid` are deleted; `FdArrayCompileContext
    .string_hash` and the `sidof`/`cff_make_fd_dict`/`cff_make_fdarray`/
    `cff_make_charset`/`cffstrings_to_indexblob` signatures thread
    `*mut indexmap::IndexMap<Vec<u8>, SdsRaw>` through unchanged in
    shape (still `&raw mut string_hash` at every call site) -- only the
    pointee type changed, so the whole call chain across five functions
    compiled correctly on the first attempt after the type change, with
    only unused-import errors left to clean up.
  - **Real functional coverage across both branches, not synthetic**:
    `KRName-Regular.otf`'s CFF is CID-keyed (confirmed via its own dump's
    `isCID: true`) and stayed byte-identical to its frozen golden
    checksum, exercising the FD-dict-string path. `WorkSans-Regular.json`
    (the "fj" — from-JSON — payload already in `rust/scripts/
    run-cycles.sh`/`compare-roundtrips.js`, excluded only from dump-side
    testing because the *matching* `.otf` triggers a pre-existing,
    unrelated CFF-interpreter stack overflow on read) is non-CID with
    786 glyphs, exercising `sidof` for hundreds of real glyph names
    through the exact large-scale path `IndexMap` was chosen for; its
    round-trip stability check passed. No new synthetic payload needed
    for either branch.
  - Verified with the standard full pipeline on both macOS and Linux.
    **~12 uthash instances remain.**
- **uthash → `IndexSet`, thirteenth instance: `PairClassifierHash`
  (`otl_read_gpos_pair`'s Format 1 branch, `table/otl/subtables/
  gpos_pair.rs`) — the largest single-function rewrite in this migration
  so far, and the first instance used across three separate phases of
  one function rather than build-then-iterate-once.** GPOS `PairPos`
  Format 1 (individual glyph pairs, no explicit classes on the wire)
  gets converted into the same `first`/`second`/`matrix` class-grid
  representation Format 2 already carries explicitly — so reading Format
  1 has to *synthesize* a class def for the "second" glyph of every
  pair. `PairClassifierHash` is that synthesis: keyed by gid, deduped,
  each distinct gid assigned the next sequential class id (1-based —
  class 0 means "not covered") at insertion.
  - The same hash instance is threaded through three phases without
    ever being rebuilt: **(1) build** — read every pair once, insert
    each "second" gid; **(2) size and allocate** — `(*subtable).second`'s
    `glyphs`/`classes` and the `first_values`/`second_values` grids are
    sized from the hash's final item count; **(3) look up (not
    insert)** — re-read the *same* pairs a second time, look up each
    "second" gid's already-assigned class id to know where in the grid
    to place its position value, silently skipping (not panicking) on
    an unmatched lookup, matching the original's `if !s_0.is_null()`
    guard exactly; **(4) finalize** — walk the set once more to populate
    `(*subtable).second`'s `glyphs`/`classes` before returning. No
    `HASH_SORT` is used here, but since cid is assigned as
    `num_items + 1` at insert time, insertion order and cid-ascending
    order are the same order by construction — the same shape as
    `SfntTableEntry`'s `HASH_SORT`-free-but-still-ordered case, reached
    by a different route.
  - No value beyond presence needs to be carried once cid is derived
    from position, so this is `indexmap::IndexSet<i32>`, not a map — the
    same simplification as `LigatureAggregator`'s `BTreeSet`, extended
    to a case that also needs `IndexMap`'s O(1) average lookup (a
    `Vec`-based linear scan would have been fine for `LigatureAggregator`
    given the shape data set there but not guaranteed for every font's
    kerning table). `get_index_of` doubles as both the phase-1
    "already inserted?" check (via `insert_full`, one line replacing
    roughly 950 lines of find-then-insert-then-resize boilerplate) and
    the phase-3 lookup.
  - Read the whole ~1500-line span before writing any replacement code,
    per this migration's standing discipline — worth calling out here
    specifically because the temptation to assume "found → some
    per-node payload, not-found → insert" (the shape of every
    `consolidate/otl/*.rs` instance) would have been wrong: there is no
    per-node payload at all, only presence and position.
  - **Real, non-synthetic coverage confirmed at the binary level, not
    inferred from the JSON dump shape** (which looks identical for
    Format 1 and Format 2 sources, since both converge on the same
    `first`/`second`/`matrix` output): a small script reading each
    payload's raw `GPOS` table directly confirmed
    `BungeeColor-Regular_colr_Windows.ttf` has one Format 1 `PairPos`
    subtable, and its dumped `"second"` map has 19 distinct glyphs each
    assigned a unique sequential class 1–19 with no two glyphs sharing a
    class — exactly this rewrite's dedup shape, not Format 2's
    author-chosen class grouping (visible elsewhere in the same file's
    other subtable, where several glyphs share one class). That payload
    stayed byte-identical to its frozen golden checksum. No synthetic
    payload needed.
  - `PairClassifierHash` is deleted; `by_pair_second_glyph` (a
    different, unrelated comparator used on the *build* side of this
    file) is untouched.
  - Verified with the standard full pipeline on both macOS and Linux.
    **~11 uthash instances remain.**
- **uthash → `BTreeMap`, fourteenth instance: `ClassNameHash`
  (`otl_parse_mark_array`, `table/otl/subtables/gpos_common.rs`) — the
  first instance spanning three files, set aside once already earlier in
  this migration as bigger scope than a single self-contained instance,
  revisited after enough of the larger multi-phase/multi-file patterns
  (`SfntTableEntry`, `CffSidEntry`, `PairClassifierHash`) had been done
  to make it tractable.** This table interns GPOS mark/base anchor class
  *names* (JSON-authored strings like `"top"`/`"bottom"`, not the numeric
  classes the binary format uses) into sequential numeric class ids
  while parsing `gpos_mark_to_single`/`gpos_mark_to_ligature` JSON.
  Built once in `otl_parse_mark_array` (`gpos_common.rs`), then handed by
  pointer to `parse_bases` in each of the other two files for read-only
  lookup, then disposed by each of those files' top-level entry function
  — a genuinely cross-file container, not just a cross-file *type*.
  - **The renumbering is alphabetical by class name, not insertion
    order** — confirmed by finding `compare_class_hash` (a plain
    `strcmp`) fed into a `HASH_SORT`, and finding *two* separate
    `class_id`-assignment sites: an insert-time placeholder
    (`HASH_COUNT`, i.e. arbitrary/insertion-order) at the first site,
    silently overwritten at the second by a sequential counter walked
    over the post-sort order. Missing this distinction was exactly the
    trap `FvarMaster`/`ScriptStatHash`/`SfntTableEntry` earlier in this
    migration would have set up an intuition for ("if in doubt, assume
    insertion order") — this instance breaks that intuition, and only
    reading both assignment sites (not just the first one found by
    grepping `class_id =`) caught it.
  - `BTreeMap<Vec<u8>, GlyphClass>` reproduces the alphabetical order for
    free (`Vec<u8>`'s `Ord` matches `strcmp`'s byte-wise comparison on
    the NUL-free byte sequences both use), so the whole `HASH_SORT` step
    collapses to `for (rank, id) in map.values_mut().enumerate() { *id =
    rank as GlyphClass; }` after every mark has been registered — no
    separate sort algorithm needed, the same win as `SfntTableEntry`.
  - **A mark's `mark_class` is written twice, once with a placeholder,
    once for real** — the original's first pass sets it from the
    insert-time (pre-renumbering) id, immediately stale once the
    alphabetical renumbering runs; a second pass re-derives each
    anchor-bearing mark's class name from the *same* JSON node
    (mirrored exactly here — marks don't carry their own class-name
    string, so both passes independently re-read `_marks`) and
    overwrites `mark_class` with the final id. Since the first value is
    always overwritten for every mark that gets one at all, this
    rewrite skips computing it in the first pass entirely (just
    registers presence via `.entry(...).or_insert(0)`) rather than
    reproducing a lookup whose result is guaranteed to be thrown away.
  - **`strlen`-bounded dedup key, not the JSON string's own tracked
    length** — same fidelity question as `CffSidEntry`, resolved the
    same way and for the same reason (anchor class names are
    user-authored JSON strings, not guaranteed free of embedded NULs).
    Confirmed by checking `sdsbuild!`'s `SdsPart` impl for `SdsRaw`/`*mut
    c_char`: it `strlen`s regardless of the sds's own tracked length, so
    the original's warning message (`"[OTFCC-fea] Invalid anchor class
    name <...>"`, built from an `SdsRaw`) was *already* silently
    `strlen`-truncated the same way — meaning this rewrite's warning,
    built directly from the raw JSON name pointer, matches byte-for-byte
    with no extra allocation needed.
  - **Real, substantial coverage, not synthetic**:
    `NotoNastaliqUrdu-Regular.ttf` (already in `tests/golden/`) has 11
    real `gpos_mark_to_base` lookups and 1 `gpos_mark_to_ligature`
    lookup — an Arabic script font, where diacritic mark positioning
    onto base letters and onto ligatures is central, not incidental.
    Its dump-then-rebuild round trip drives both files'
    `otl_parse_mark_array`/`parse_bases` rewrites through real,
    multi-class alphabetical renumbering, and the rebuilt binary stayed
    byte-identical to its frozen golden checksum. No synthetic payload
    needed.
  - `ClassNameHash` and `compare_class_hash` are deleted from
    `gpos_common.rs`; both consumer files' disposal loops (walking and
    freeing each hash node) are gone entirely, since a `BTreeMap<Vec<u8>,
    _>` with no raw-pointer values needs no manual cleanup.
  - Verified with the standard full pipeline on both macOS and Linux.
    **~10 uthash instances remain.**
- **uthash → `BTreeMap`, fifteenth instance: `BaseHash`/`MarkHash`/`LigHash`
  (`rust/src/consolidate/otl/mark.rs`)**. Three separate uthash tables in
  one file, all used the same way: `consolidate_mark_array`,
  `consolidate_base_array`, and `consolidate_lig_array` each build a
  table keyed by *resolved glyph id* (not name — `HASH_ADD_INT`, hashing
  the raw `c_int` gid, not a string), feed it through `HASH_SORT` with a
  `by_gid` comparator (`(*a).gid - (*b).gid`, i.e. plain ascending
  numeric order) before the final `HASH_ITER`, then throw the table away
  once its contents are pushed back into a freshly emptied `MarkArray`/
  `BaseArray`/`LigatureArray`. `GlyphId` (`u16`) is `Ord` in exactly that
  order, so `BTreeMap<GlyphId, _>` reproduces `HASH_SORT` for free, same
  as every earlier gid/index-keyed instance in this migration.
  - **Two different "value already present" behaviors, both preserved
    exactly.** `consolidate_mark_array` only inserts when the entry is
    vacant **and** `anchor.present` **and** `mark_class < class_count`;
    any of the three failing (including "already present") logs the
    *same* "Ignored invalid or double-mapping mark definition" warning —
    expressed as `Entry::Vacant(v) if <extra condition> => { v.insert(...) }
    _ => { warn(...) }`, so a guard failure on `Vacant` falls through to
    the same `_` arm as `Occupied` without a second lookup.
    `consolidate_base_array`/`consolidate_lig_array` have no extra
    condition — `Entry::Vacant`/`Entry::Occupied` is the whole check —
    but log a *different* message ("Ignored anchor double-definition").
  - **Ownership transfer, not a copy, for the two anchor-array fields.**
    Unlike the mark case (`name: sdsdup(...)`, a real copy — the original
    `MarkRecord`'s own `Handle` survives untouched and gets disposed
    normally later), `consolidate_base_array`/`consolidate_lig_array`
    *steal* the `anchors`/`anchors` (component-indexed, `*mut *mut
    Anchor`) pointer straight out of the source array's element and null
    the source field out — `let ref mut fresh0 = (&mut
    (*base_array))[k].anchors; *fresh0 = null_mut();` — so that the
    subsequent `dispose_base_array(base_array)` (which unconditionally
    frees every element's `anchors`) doesn't double-free the pointer that
    now lives in the map. When an entry is instead rejected as a
    duplicate, its `anchors` pointer is deliberately left in place, so
    that same `dispose_*_array` call is what frees it — the "leak
    prevention" for the rejected duplicate is not bespoke cleanup code in
    the hash-table path at all, it's simply *not stealing*, and letting
    the ordinary end-of-function disposal do its normal job.
  - No `name`/gid stored in the map values beyond what's needed to
    rebuild — `MarkHashValue { name, mark_class, anchor }`, `BaseHashValue
    { name, anchors }`, `LigHashValue { name, component_count, anchors }`
    — `name` is an `SdsRaw` `sdsdup`'d copy in all three cases (sds itself
    is untouched by this migration, per the established scope boundary),
    freed with `sdsfree` right after each output push, same as the
    original's post-`HASH_ITER` cleanup.
  - **No coverage gap needed real chasing this time, but no free ride
    either.** `NotoNastaliqUrdu-Regular.ttf` (11 `gpos_mark_to_base` + 1
    `gpos_mark_to_ligature` lookups) exercises the *ordinary* insert path
    for all three tables through the existing golden checksum, but no
    committed payload has a *duplicate* glyph within one subtable's
    `marks`/`bases` JSON object, so the dedup branches — the entire
    reason these three uthash tables existed — had never actually run
    under test. Closed with `rust/scripts/make-test-mark-consolidate-dedup.py`
    (same raw-JSON-duplicate-key technique as `make-test-gpos-single-dedup.py`
    et al.): one forged `gpos_mark_to_base` lookup with a duplicate
    `marks` entry and a duplicate `bases` entry, and one forged
    `gpos_mark_to_ligature` lookup with a duplicate ligature `bases`
    entry, each pair sharing a glyph name so the *second* JSON member
    parses into a second array element that collides by gid with the
    first at consolidation time. Confirmed by hand (before freezing the
    golden checksum) that all three warnings fire and the surviving
    values are the *first* occurrence's, not the second's — then wired
    into `generate-golden.sh`/`compare-with-golden.sh` as
    `mark-consolidate-dedup`, an additions-only change to
    `tests/golden/checksums.sha256`.
  - `BaseHash`/`MarkHash`/`LigHash` and their `base_by_gid`/`mark_by_gid`/
    `lig_by_gid` comparators are deleted; no manual `HASH_DEL`/`free`
    node-walk survives — dropping the `BTreeMap` (or just letting
    `into_iter()` drain it) is enough, since map values own nothing the
    map itself needs to know about (the `anchors` pointers and `name`
    are moved out to the pushed record / freed explicitly, same as
    before).
  - Verified with the standard full pipeline on both macOS and Linux.
    **~9 uthash instances remain.**
- **uthash → `IndexSet`/`IndexMap`, sixteenth instance: `CoverageEntry`
  (`rust/src/table/otl/coverage.rs`'s `read_coverage`, plus a second,
  cross-file copy of the *same* struct in `rust/src/table/otl/classdef.rs`
  reused by `read_class_def` and `expand_class_def`)**. Flagged early in
  this migration's plan as "shared with classdef.rs" and set aside for
  exactly that reason — the two files import the identical `CoverageEntry`
  type (`gid: c_int`, `cov_index: c_int`, `hh: UtHashHandle`), so neither
  side could be converted independently without breaking the other's
  `use`. Turned out to be four separate hash usages, not one, each with
  its own dedup-key-vs-sort-key relationship — **the actual finding of
  this instance is that identical uthash-boilerplate shapes can still
  need different container strategies**, contradicting the assumption
  (built up over the previous fifteen instances) that "same struct,
  same comparator name" meant "same rewrite".
  - **`read_coverage` Format 1 (glyph list)**: dedups by gid, sorts by
    `covIndex` — but `covIndex` was assigned `j` (the loop's own
    position) *only at insertion time* (skipped duplicates never
    advance it), so the sequence of inserted `covIndex` values is
    already strictly increasing. Sorting by it is provably a no-op:
    `indexmap::IndexSet<GlyphId>` (dedups on `.insert()`, iterates in
    insertion order) reproduces the exact final order with no sort step
    and no need to track `covIndex` at all — the value doesn't matter,
    only presence and position, same finding as `PairClassifierHash`.
  - **`read_coverage` Format 2 (gid ranges) is the opposite case**: here
    `covIndex` is `startCoverageIndex + k` where `k` is the *absolute
    gid*, confirmed against `c/lib/table/otl/coverage.c` line 89 to rule
    out a transpilation artifact — not `startCoverageIndex + (k -
    start)`, which is what a "position within the whole coverage table"
    would actually need. Whether this is a pre-existing upstream quirk
    or intentional wasn't investigated (out of scope — preserve, don't
    fix), but its consequence is decisive: within one range `covIndex`
    is monotonic with `k`, but *across* ranges it is not generally
    monotonic with insertion order once ranges are out of file order or
    overlapping. Reproduced with `indexmap::IndexMap<GlyphId, i32>`
    (dedup by gid, first occurrence wins, insertion order preserved for
    tie-breaking) collected into a `Vec` and stable-sorted by the stored
    `covIndex` — `Vec::sort_by_key` is stable, matching `HASH_SORT`'s
    documented mergesort stability.
  - **`classdef.rs`'s `read_class_def` Format 2 repurposes the same
    field for a different, dump-visible purpose**: `covIndex` here holds
    the *class value*, not a position, so `HASH_SORT`-by-it orders the
    resulting `ClassDef` by ascending class value, not by gid — visible
    directly in `dump_class_def`'s walk order. Same
    `IndexMap`-then-stable-sort shape as coverage Format 2, sorting by
    the stored class instead.
  - **`expand_class_def` has no `HASH_SORT` call at all** — confirmed by
    grep before writing any replacement code (the only `by_cov_index(`
    call site in the file belongs to `read_class_def`, well before this
    function starts), so its final walk is plain insertion order, which
    `IndexMap` gives for free with no sort step. The function reuses one
    hash table across two phases sharing the same map, the same
    multi-phase shape as `PairClassifierHash`: phase 1 inserts every
    `(gid, class)` from the old, partial `ClassDef` (`ocd`, first
    occurrence wins); phase 2 walks the target `Coverage` and inserts
    any glyph not already present with class 0. The rewrite collapsed
    from roughly 1600 lines (two ~700-line find-then-insert blocks) to
    about 20.
  - No warning/log call exists anywhere in either file's uthash paths —
    confirmed by grep before starting — so, unlike the `consolidate/otl`
    instances, there was no message fidelity to preserve, only ordering.
  - **Real, non-synthetic coverage for every path touched**:
    `BungeeColor-Regular_colr_Windows.ttf` (already in `tests/golden/`,
    per the `PairClassifierHash` bullet above) has a Format 2
    class-based `GPOS` pair subtable in the same file as its Format 1
    one, which drives `read_class_def`'s Format 2 branch and
    `expand_class_def` on every build; every other payload's ordinary
    `Coverage` reads drive `read_coverage` Format 1 (and Format 2 where
    present). The genuinely adversarial case for Format 2 — out-of-order
    or overlapping ranges, where the sort-vs-insertion-order distinction
    would actually produce different output — isn't reachable through
    any JSON→build round trip (the builder always emits well-formed,
    non-overlapping, ascending ranges), so it was verified by direct
    reasoning about the formula rather than a synthetic payload, the
    same bar as `read_coverage` Format 1's provably-a-no-op sort. All
    payloads stayed byte-identical to their frozen golden checksums; no
    checksum regeneration needed.
  - `CoverageEntry` and both files' `by_cov_index` comparators are
    deleted entirely (the struct's only two importers are these two
    files, confirmed by a crate-wide grep before deletion). `by_gid`/
    `by_handle_gid`/`ClassDefSortRecord` (the `qsort`-based *build*-side
    comparators, unrelated to uthash) are untouched.
  - Verified with the standard full pipeline on both macOS and Linux.
    **~8 uthash instances remain.**
- **uthash → `BTreeMap`, seventeenth instance: `ClassifierHash`
  (`rust/src/table/otl/subtables/chaining/classifier.rs`)**, the OTL
  chaining/contextual-substitution classifier — the algorithm that
  decides whether several adjacent per-glyph chaining rules can be
  merged into one class-based (`ChainDef` format 2) subtable. The
  smallest remaining uthash-macro count (77) of the untouched files,
  but the densest in logic: one shared hash table read and written
  across three functions (`class_compatible`, `build_rule`, `to_class`),
  itself instantiated three times per subtable (`hb`/`hi`/`hf`, one
  each for the backtrack/input/lookahead match positions).
  - **The struct's own comparator (`by_gid_clsh`) sorts by the same gid
    that's already the dedup key** — the same relationship as
    `CoverageEntry`'s Format 1, so `BTreeMap<GlyphId, ClassifierValue>`
    reproduces `to_class`'s `HASH_SORT`-then-walk with no separate sort
    step, and `by_gid_clsh` itself is gone, fully subsumed by the
    container.
  - **A second, throwaway hash inside `class_compatible`'s "already
    classified" branch turned out to need no value at all.** When
    `cov`'s first glyph is already in `h` under some class `cls`, the
    original builds a *second*, temporary hash (`revh`) of `cov`'s own
    glyph ids purely to answer one question: does `h`'s *entire*
    existing membership of class `cls` exactly equal `cov`'s glyph set
    (not just "is `cov` a subset of it")? Read fully before writing any
    code, since the shape looked at first like it might carry a payload
    the way `h` itself does — it doesn't; every write to `revh`'s
    `gname`/`cls` fields is dead, only presence is ever read back. A
    bare `HashSet<GlyphId>`, collected directly from `cov`, replaces it;
    the "every member of class `cls` in `h` must be in `cov`'s set"
    check becomes a one-line `.iter().filter(...).all(...)`.
  - **`gname` in both `h` and the old `revh` is an alias, not an owned
    copy** — confirmed by grep: no `sdsfree` call exists anywhere in
    this file, and the field is assigned directly from
    `cov`'s own `GlyphHandle.name` (`gname: (&(*cov))[j].name`, no
    `sdsdup`). It's a borrow into memory the `Lookup`/`Coverage`
    structures already own and keep alive for the duration of
    classification; `to_class`'s later `handle_from_consolidated` is
    what actually duplicates it into the new `ClassDef`'s own `Handle`.
    Preserved as a plain pointer copy in `ClassifierValue`, matching the
    original exactly — no risk of a double free, since `SdsRaw` has no
    `Drop` impl to begin with.
  - **`to_class` never disposed the hash in the original either** —
    confirmed by grep before touching `try_classify_around` (the only
    caller): the three manual `HASH_ITER`+`HASH_DEL`+`free` drain loops
    at the end of that function were *always* the thing that freed
    `hb`/`hi`/`hf`, regardless of whether `to_class` had been called on
    them earlier in the same call (it only sorts and reads). Changing
    `hb`/`hi`/`hf` to owned `BTreeMap`s and `to_class`/`build_rule` to
    borrow them (`&BTreeMap`, not consuming) lets Rust's ordinary
    scope-exit `Drop` do exactly that job — the three ~60-line drain
    loops are deleted outright, not replaced with anything.
  - **One `None` arm in `build_rule` mirrors a null-pointer dereference
    in the original that is unreachable in practice, not merely
    convenient to assume so** — by the time `build_rule` runs for a
    given match position, `class_compatible` has already run for that
    exact `h` and glyph and never returns success without the glyph
    being present in `h` afterward, so the original's unconditional
    `(*s).cls` (no null check at all) never actually reads garbage in
    a real invocation. Written as `match h.get(&gid) { Some(v) => ...,
    None => 0 }` (falling back to the same "class 0" the empty-coverage
    branch uses) rather than `.unwrap()`, matching this migration's
    established handling of such algorithm-invariant-guaranteed `None`
    arms (see `ClassNameHash`).
  - **Real, substantial coverage confirmed at the binary level, not
    assumed from the dump JSON's shape**: whether the classification
    *merge* actually happens (`compatible_count > 1` in
    `try_classify_around`, the branch that exercises `to_class`'s sort
    and `class_compatible`'s `revset`/`allcheck` logic) depends on
    whether adjacent rules happen to share compatible glyph classes, so
    a small script parsing raw `GSUB`/`GPOS` bytes was used to check
    for format-2 (class-based) `ChainContextSubst` subtables directly —
    format 1 and 3 alone wouldn't prove anything, since those are what
    the *unclassified* per-glyph-coverage path also produces. Rebuilding
    `iosevka-r.ttf` from its own dump produces multiple format-2
    subtables (`GSUB` lookups 25 and 50), confirming the merge path
    genuinely runs, and the rebuild stayed byte-identical to its frozen
    golden checksum. No synthetic payload needed.
  - `ClassifierHash` and `by_gid_clsh` are deleted entirely.
  - Verified with the standard full pipeline on both macOS and Linux.
    **~7 uthash instances remain.**
- **uthash → `Vec`, eighteenth instance: `LookupHash`
  (`rust/src/table/otl/parse.rs`)**. `parse.rs` (6,692 lines) holds three
  name-keyed hashes — `LookupHash`, `FeatureHash`, `LanguageHash` — big
  enough that this PR converts only the first; the other two are left
  for follow-up PRs, each getting the same care rather than three at
  once. Chosen over `FeatureHash`/`LanguageHash` because it already
  carried a documented Rust-only bugfix (the `alias` flag, see below),
  making it the instance most worth getting exactly right first.
  - **Not a dedup map — the reason this can't be `BTreeMap`/`IndexMap`
    like every previous name-keyed instance.** `_declare_lookup_parser`
    (the real-entry path) does check "does this name already exist"
    before inserting, rejecting the declaration with a warning if so —
    but the *alias* path (a JSON `"lookups"` string value) only checks
    that its *target* name exists; it never checks whether its own new
    name collides with something already there. So two entries *can*
    legitimately share a name, and uthash's bucket-prepend insertion
    means `HASH_FIND` on a duplicated key always resolves to the
    *most-recently-inserted* match. `LookupEntry` stays a plain `Vec`
    (append-only, preserving insertion order for exactly this reason),
    and every "look up by name" call site — alias-target resolution,
    the `lookupOrder` Force-promotion pass in `otfcc_parse_otl`, and a
    feature's lookup-name resolution in `figure_out_features_from_json`
    (which receives this same `lh`, now `&Vec<LookupEntry>`) — became
    `.iter().rev().find(...)` (or `.iter_mut().rev()` for the one
    mutating case), reproducing "most recent wins" exactly without a
    second index structure; alias resolution is rare enough that an
    O(n) reverse scan needs no justification beyond that.
  - **The sort key (`by_lookup_order`: `order_type` then `order_val`)
    is unrelated to the dedup question above** (moot here since there
    is no dedup step) **but still doesn't match insertion order once
    `lookupOrder` promotes a subset of entries to `Force`** — the same
    "sort key can outlive insertion order" shape as `CoverageEntry`
    Format 2. Resolved the same way: nothing between the `lookupOrder`
    loop and the final drain needs `lh` in sorted order (`HASH_FIND`
    doesn't care about the `HASH_SORT`-maintained iteration list, only
    the untouched bucket chains), so the original's early
    `HASH_SORT(lh, by_lookup_order)` call is deleted outright and
    replaced with a single `lh.sort_by(...)` immediately before the
    drain — `by_lookup_order` itself is gone, subsumed by adding
    `PartialOrd, Ord` to `LookupOrderType`'s derive (its two variants'
    declaration order already matches the comparator's numeric
    comparison) and `Vec::sort_by`'s stability matching `HASH_SORT`'s
    documented mergesort stability.
  - **The final drain already only pushed non-alias entries' `.lookup`
    into `otl.lookups`** (`Box::from_raw`, matching the alias flag's
    original fix) **but always freed every entry's `.name`, alias or
    not** — because `.name` was its own independent `sdsnew`-allocated
    copy on the hash node, never shared, unlike `.lookup`. `Vec<u8>`
    replacing that field needs no explicit free at all, matching every
    other container conversion in this migration; the manual
    `HASH_ITER`+`HASH_DEL`+`free` walk is gone, replaced by
    `lh.into_iter()` and Rust's ordinary per-element drop.
  - **Real, direct coverage for both mechanisms this instance touches**:
    `rust/scripts/make-test-lookup-alias.py` (already part of every
    `test.sh` run) exists specifically to regression-test the alias
    fix — it segfaults C's otfccbuild and hung this crate's pre-`Box`
    baseline, so a clean pass here is a strong signal the `alias`
    semantics survived the rewrite intact. `lookupOrder` (the `Force`
    promotion path) is present in `iosevka-r.json`, `WorkSans-Regular.json`,
    and `kltf-bugfont1.json` already, so the sort-deferral change is
    exercised by the standard payload set, not a synthetic addition.
    All payloads stayed byte-identical to golden; no regeneration
    needed.
  - Verified with the standard full pipeline on both macOS and Linux.
    **`FeatureHash`/`LanguageHash` in this same file remain for
    follow-up PRs — the uthash-instance counter isn't decremented for
    this one, since it's the first of three sharing one file, matching
    how `CoverageEntry` and `ClassifierHash` were only counted once
    each file's full set of instances was done.**
- **uthash → `Vec`, nineteenth instance: `FeatureHash`
  (`rust/src/table/otl/parse.rs`)**, the second of the three name-keyed
  hashes in this file. `LanguageHash` remains for a follow-up PR.
  - **Same asymmetric shape as `LookupHash`, confirmed by re-checking
    rather than assumed from the pattern**: `figure_out_features_from_json`
    rejects a real (array-valued) feature declaration outright if its
    name already exists (`[OTFCC-fea] Duplicate feature`, disposing the
    freshly-built `LookupRefList` without merging), but an alias entry
    (a JSON string value under `"features"`) only checks that its
    *target* name exists — never its own — so, same as `LookupHash`,
    two entries can legitimately share a name and this stays a `Vec`
    with reverse (most-recent-wins) search, not a dedup map.
  - **The sort key this time genuinely does equal the would-be dedup
    key** (`by_feature_name`: byte-wise `strcmp` on `name`) — unlike
    `LookupHash`'s `order_type`/`order_val`, which came apart from
    insertion order once `lookupOrder` promoted a subset of entries.
    That equality doesn't make `BTreeMap` usable here either, for the
    same reason it couldn't for `LookupHash`: a `BTreeMap` can't hold
    two entries under one key, and aliases mean it must be able to.
    So this is `Vec` for the same structural reason as `LookupHash`,
    just sorted by a different key at drain time — `fh.sort_by(|a, b|
    a.name.cmp(&b.name))`, `Vec<u8>`'s byte-wise `Ord` matching `strcmp`
    on NUL-free byte sequences exactly, the same equivalence this
    migration has relied on since `ClassNameHash`. `by_feature_name`
    itself is gone, subsumed by the container.
  - **This `fh` is itself consumed by two more functions that had to
    change in step**: `figure_out_features_from_json` receives `lh`
    (now `&Vec<LookupEntry>`, from the `LookupHash` PR) to resolve a
    feature's lookup-name array; `figure_out_languages_from_json`
    receives this PR's `fh` (now `&Vec<FeatureEntry>`) to resolve a
    language's `requiredFeature` and `features` array the same way.
    Both became `.iter().rev().find(...)` at their respective call
    sites, matching every other by-name lookup in this file so far.
  - **The final drain's ownership split matches `LookupHash` exactly**:
    `.feature` is only taken back via `Box::from_raw` for non-alias
    entries (an alias's copy of the same pointer is never freed on its
    own), while `.name` was always freed regardless of `alias` (its own
    independent `sdsnew`-allocated copy, never shared) — `Vec<u8>`
    replacing that field needs no explicit free at all, so the manual
    `HASH_ITER`+`HASH_DEL`+`free` walk for `fh` is gone entirely, same
    as it was for `lh`.
  - **Real coverage**: every payload with any GSUB/GPOS feature at all
    exercises the ordinary (non-alias, non-duplicate) path through
    `figure_out_features_from_json`, which is all of them. The alias
    and duplicate-rejection branches aren't separately exercised by any
    committed payload, but their logic is unchanged from `LookupHash`'s
    already-verified shape (the lookup-alias regression test covers the
    identical mechanism one type up), so no new synthetic payload was
    added for this PR specifically. All payloads stayed byte-identical
    to golden; no regeneration needed.
  - Verified with the standard full pipeline on both macOS and Linux.
    **`LanguageHash` in this same file remains for a follow-up PR — the
    uthash-instance counter isn't decremented yet.**
- **uthash → `BTreeMap`, twentieth instance: `LanguageHash`
  (`rust/src/table/otl/parse.rs`)**, the third and last of this file's
  name-keyed hashes.
  - **The one genuinely simple case of the three, confirmed rather than
    assumed**: grep before touching any code confirmed `"languages"`
    has no string-value (alias) case at all — `figure_out_languages_from_json`
    only ever branches on `JsonType::Object`, never `JsonType::String`,
    and `LanguageHash` itself has no `alias` field to begin with. With
    no alias mechanism, a duplicate name is unconditionally rejected
    (`[OTFCC-fea] Duplicate language item`, disposing the freshly-built
    `FeatureRefList`) with no bypass, so every name really is unique —
    the first name-keyed hash in this file where the dedup key and the
    `HASH_SORT` key (`by_language_name`, byte-wise `strcmp` on `name`,
    same as `by_feature_name`) coincide *and* stay usable, unlike
    `LookupHash`/`FeatureHash` where aliasing ruled a map out despite
    the same coincidence. `BTreeMap<Vec<u8>, *mut LanguageSystem>`
    applies directly, no wrapper value struct needed since the only
    payload is the one pointer — same reasoning as `PairClassifierHash`
    (IndexSet, not a map) for "don't carry a value the map doesn't
    need," just landing on a different container here since a name
    *is* needed as the map's own key. `by_language_name` is gone,
    subsumed by the container's natural `Ord`, and no explicit sort
    call is needed at drain time either (unlike `LookupHash`/
    `FeatureHash`'s deferred `sort_by`) since `BTreeMap` iterates
    presorted.
  - **This is the last of the three, so the whole uthash-boilerplate
    import list for this file finally goes**: `exit`/`free`/`malloc`/
    `memcmp`/`memset`/`strlen` from `libc`, `__caryll_allocate_clean`,
    `NULL`, the entire `vendor::uthash` import line, and `sdsdup`/
    `sdsfree` — all unused the moment this PR's `cargo build` ran,
    confirming no other code in this 6,700-line file depended on any
    of them. `strcmp`/`strncmp` remain (used by the lookup-type/
    feature-merge string comparisons, unrelated to hashing).
  - **Real coverage**: every payload with any `"languages"` entry at
    all exercises the ordinary path (all of them, since a font with
    OTL features always declares at least `DFLT`); the duplicate-name
    rejection isn't separately exercised by any committed payload, but
    its logic is unchanged from the already-verified `LookupHash`/
    `FeatureHash` "reject and dispose" shape one level up, so no new
    synthetic payload was added. All payloads stayed byte-identical to
    golden; no regeneration needed.
  - Verified with the standard full pipeline on both macOS and Linux.
    **All three of `parse.rs`'s name-keyed hashes are now converted.
    ~6 uthash instances remain.**
- **uthash → `BTreeMap`, twenty-first instance (first half): `CmapTable.unicodes`
  (`rust/src/table/cmap.rs`)**, converting the `CmapEntry` hash to
  `BTreeMap<c_int, GlyphHandle>`. The sibling `.uvs`/`CmapUvsEntry` field is
  deferred to a follow-up PR, so the uthash-instance counter isn't
  decremented yet.
  - **The first *persistent* hash instance in this migration.** Every prior
    instance was a transient, build-then-drain scratch structure created
    and consumed within one function or a small cooperating cluster.
    `unicodes` is a long-lived field of `CmapTable` itself — read, written,
    and iterated by many functions across the font's whole lifetime
    (encode/unmap/lookup during parse and JSON encode, sorted iteration
    during dump and binary build) — so this is closer to a Stage-6-1
    "Vec-ify a persistent field" conversion than the usual
    "build-then-drain" pattern.
  - **Sort key == dedup key, same shape as `LanguageHash`**: `by_unicode`
    (`HASH_SORT`'s comparator) sorts by the same `unicode` field that keys
    the hash, so `BTreeMap<c_int, GlyphHandle>` needs no explicit sort at
    drain time and `by_unicode` itself is gone.
  - **`table_cmap_copy` deleted outright, confirmed dead first**: a raw
    `memcpy` of the whole `CmapTable`, which would have had to `memcpy` a
    `BTreeMap` — impossible without double-freeing its backing allocation.
    Crate-wide grep confirmed no caller of `.copy` exists anywhere (only
    `caryll_font.rs`'s `TABLE_I_CMAP.free` is called from outside this
    file), so the slot and its vtable field were both removed rather than
    worked around.
  - **A cross-file consumer discovery not seen in any prior (transient)
    instance**: `CmapEntry` turned out to be imported and walked directly
    by three other files — `consolidate.rs`'s `consolidate_cmap`,
    `otf_reader/unconsolidate.rs`'s AGLFN-naming walk, and
    `otf_writer/stat.rs`'s `stat_os_2_unicode_ranges` — none of which showed
    up from reading cmap.rs alone; all three surfaced only once `cargo
    build` reported `unresolved import` after the struct was deleted. Each
    was converted to `for (&unicode, glyph) in (*cmap).unicodes.iter()` (or
    `.iter_mut()` for `consolidate_cmap`, which mutates `glyph` in place on
    a failed name resolution), preserving each function's exact original
    semantics (`stat.rs`'s ~450-line body of Unicode-block range checks was
    left untouched by keeping the loop variable named `u`).
  - **Found and fixed a pre-existing type-confusion bug inherited from the
    original C** (`c/lib/table/cmap.c:109,122`): `otfcc_unmap_cmap_uvs` and
    `otfcc_cmap_lookup_uvs` both declared their walk variable as
    `cmap_Entry *s` instead of `cmap_UVS_Entry *s` (compare the correctly
    typed sibling `otfcc_encodeCmapUVSByName`, which does use
    `cmap_UVS_Entry *s`). uthash's `HASH_FIND`/`HASH_DEL` still worked
    correctly regardless — they only dereference `&s->hh`, and `hh` is the
    first field of both structs — but `&s->glyph` read from the wrong byte
    offset (`CmapEntry.glyph` sits after a 4-byte `int unicode`;
    `CmapUvsEntry.glyph` sits after an 8-byte `CmapUvsKey`), producing
    garbage `Handle` data. Deleting `CmapEntry` broke both functions'
    compilation, which is how this surfaced. `otfcc_unmap_cmap_uvs` is
    fully dead (no callers anywhere in the crate), so retyping it is
    behavior-neutral; `otfcc_cmap_lookup_uvs` **is** reachable (called
    once, in the UVS parse path), so fixing its type annotation is a
    deliberate, incidental memory-safety fix — same category as the
    `LookupHash.alias` fix earlier in this migration. Both were retyped to
    the correct `CmapUvsEntry` throughout.
  - **Real coverage**: every payload with a `cmap` table exercises
    encode/unmap/lookup/dump/build for `.unicodes` (all of them);
    `NotoNastaliqUrdu-Regular.ttf` and `iosevka-r.ttf` in particular carry
    large, non-trivial Unicode ranges that exercise the format-4/format-12
    contiguous-run logic in `otfcc_build_cmap_format4`/`_format12`. No new
    synthetic payload was needed. All payloads stayed byte-identical to
    golden; no regeneration needed.
  - Verified with the standard full pipeline on both macOS and Linux.
    **`.uvs`/`CmapUvsEntry` in this same file remains for a follow-up PR —
    the uthash-instance counter isn't decremented yet.**
- **uthash → `BTreeMap`, twenty-second instance (second half):
  `CmapTable.uvs` (`rust/src/table/cmap.rs`)**, converting the
  `CmapUvsEntry` hash to `BTreeMap<CmapUvsKey, GlyphHandle>` and completing
  this file — the uthash-instance counter is decremented now.
  - **Same shape as `.unicodes`, confirmed rather than assumed**: `by_uvs_key`
    (`HASH_SORT`'s comparator) compares `(unicode, then selector)`, which is
    exactly `CmapUvsKey`'s derived `Ord` (its two fields, `unicode` then
    `selector`, compared in declaration order), and `HASH_FIND`'s key
    equality is the same two-field comparison. Sort key, dedup key and
    derived `Ord` all agree, so `BTreeMap<CmapUvsKey, GlyphHandle>` needs no
    wrapper struct (the key carries both fields directly) and no explicit
    sort at drain time — `by_uvs_key` itself is gone, subsumed by the
    container.
  - **`CmapUvsKey` gained `PartialEq`/`Eq`/`PartialOrd`/`Ord` derives**
    (previously only `Copy`/`Clone`, since uthash did its own hashing and
    `memcmp`); `CmapUvsEntry` itself — the wrapper struct holding `hh: UtHashHandle`,
    `key: CmapUvsKey` and `glyph: GlyphHandle` — is gone entirely, same as
    `CmapEntry` before it.
  - **The four public accessor functions collapsed the same way `.unicodes`'s
    did**: `otfcc_encode_cmap_uvs_by_index`/`_by_name` are a `.entry(c)` match
    on `Vacant`/`Occupied` (insert-if-absent, no overwrite, no warning);
    `otfcc_unmap_cmap_uvs` is `.remove(&c).is_some()`; `otfcc_cmap_lookup_uvs`
    is `.get(&c)` plus a pointer cast. Each collapsed from ~250–450 lines of
    Jenkins-hash-and-bucket-chain boilerplate to 4–9 lines.
  - **`dispose_cmap` needed no explicit per-entry disposal at all**: with
    both `.unicodes` and `.uvs` now `BTreeMap`s of `Handle`-owning values,
    dropping each map (via assignment) runs every entry's `Handle::drop` in
    turn — the two manual `HASH_ITER`+`HASH_DEL`+`free` walks this replaced
    are both gone, and the function is four lines.
  - **Cross-file consumer, one this time (not three)**: `consolidate.rs`'s
    `consolidate_cmap` also walked `CmapUvsEntry` directly for its UVS half
    (`otf_reader/unconsolidate.rs` and `otf_writer/stat.rs` only ever touched
    `.unicodes`, confirmed by grep before starting). Converted to
    `for (key, glyph) in (*(*font).cmap).uvs.iter_mut() { ... }`, matching
    the `.unicodes` half converted in the previous PR.
  - **Real coverage**: `KRName-Regular.otf` carries a real `cmap_uvs` table
    (confirmed in its golden dump) and already exercises the full
    read/parse/dump/build path for `.uvs` in every run of
    `compare-with-c.sh`/`compare-with-golden.sh`; format-14's
    default/non-default range logic in `build_format14_for_selector` is
    driven by that same payload. No new synthetic payload was needed. All
    payloads stayed byte-identical to golden; no regeneration needed.
  - Verified with the standard full pipeline on both macOS and Linux.
    **`cmap.rs` is now fully converted (both `.unicodes` and `.uvs`).
    ~5 uthash instances remain.**
- **uthash → `HashMap`, twenty-third instance: `CffSubrGraph.diagram_index`
  (`rust/src/libcff/subr.rs`)**, converting the `CffSubrDiagramIndex` hash
  (variable-length byte-string keys) to `HashMap<Vec<u8>, CffSubrDiagramIndexEntry>`.
  - **The first genuinely *pervasive* single-file hash in this migration**:
    unlike every prior instance (built and drained within one function or a
    small cooperating cluster), this table is read, written, and deleted
    from across six different functions spread over the whole file
    (`unlink_node`, `add_doublet`, `add_singlet`, `check_doublet_match`,
    `check_singlet_match`, plus disposal) — the CFF subroutine-graph
    deduplication algorithm's shared state for the whole file.
  - **One table, two arities, sharing one keyspace on purpose**: entries for
    "singlet" (one-node) and "doublet" (two-node) charstring patterns live
    in the same hash, keyed by a variable-length byte fingerprint
    (`get_singlet_hash_key`/`get_doublet_hash_key`) whose leading byte
    (`'1'` vs `'2'`) keeps the two arities from ever colliding — confirmed
    by reading both key-builders rather than assumed, since a collision
    would have been silently wrong instead of a crash. No `HASH_SORT`
    anywhere and the only whole-table walk is disposal, so order never
    matters: `HashMap`, not `BTreeMap`.
  - **`key: *mut u8` and `hh: UtHashHandle` both vanish from the value
    struct** (`CffSubrDiagramIndex` → `CffSubrDiagramIndexEntry { arity,
    start }`) — grepped first to confirm `.key` is never read for anything
    but the hash itself (only ever set on insert, freed on delete), so
    `HashMap`'s own key fully subsumes it, matching every by-name/by-key
    hash converted so far in this migration.
  - **Three different call shapes for the same table, each translated on
    its own merits rather than by analogy**: `add_doublet`/`add_singlet`
    are unconditional upserts (`HashMap::insert` replacing the whole value
    on a duplicate key reproduces the original's "found → overwrite
    `.start`, drop the new key; not found → allocate and insert" exactly,
    since the found branch never touched `.arity` and a doublet-keyed
    entry's arity is always 2 regardless). `check_doublet_match`/
    `check_singlet_match` are search-with-conditional-insert — an
    `.entry()` match on `Vacant`/`Occupied`, where the vacant arm inserts
    and returns one bool while the occupied arm reads `.arity`/`.start` to
    decide whether to fire `process_match_doublet`/`process_match_singlet`
    and returns a *different* bool per arity (doublet's "no match" arm
    returns `true`, singlet's returns `false` — confirmed by reading each
    function's tail rather than assuming symmetry between the two, since
    they are not symmetric). `unlink_node` is search-with-conditional-delete,
    once for each arity, deleting an entry only if it is still pointing at
    the node being unlinked (some other node may have since claimed that
    key's slot) — `.get()` then a conditional `.remove()`.
  - **`get_singlet_hash_key`/`get_doublet_hash_key` rewritten to build
    `Vec<u8>` directly** (`.push`/`.extend_from_slice`) instead of
    `__caryll_allocate_clean` + manual `memcpy` at computed offsets — same
    byte layout (header bytes, payload, trailing NUL), so keys built here
    compare equal to the old `memcmp`-compared keys. Dropped `extern "C"`
    from both (confirmed by grep: all 6 call sites are internal to this
    file, none through an FFI/vtable boundary) since `Vec<u8>` isn't
    FFI-safe.
  - **`CffSubrGraph.diagram_index`'s disposal collapsed to a bare field
    reassignment**: the entries own nothing (`arity: u8` and
    `start: *mut CffSubrNode` are both `Copy`), so dropping the map is the
    whole disposal — the manual `HASH_ITER`+`HASH_DEL`+`free` walk is gone.
  - **`.copy`/`.create`/`.free` deleted from `CffSubrGraphElementInterface`,
    confirmed dead first**: crate-wide grep found only `.init`/`.dispose`
    called from outside this file (`table/cff.rs`), and the backing
    functions (`cff_subr_graph_copy`'s raw `memcpy`, `cff_subr_graph_create`,
    `cff_subr_graph_free`) were referenced only by the vtable's own static
    initializer — never called, matching the `table_cmap_copy`/
    `TABLE_I_COLR`-precedent pattern. `CffSubrGraph` and its one embedding
    struct (`table/cff.rs`'s `CffCharstringBuilderContext`) both drop
    `Copy`/`Clone` as a result; the sole call site already constructs and
    uses both purely by pointer, so this was a clean removal.
  - **Found and closed a real coverage gap, not a synthetic-payload
    substitute**: CFF subroutinization (`-O2`, `--subroutinize`) was never
    exercised anywhere in this test suite before this PR — no script built
    with `-O2`, so `add_doublet`/`add_singlet`/`check_doublet_match`/
    `check_singlet_match` (this whole conversion's actual logic) had zero
    coverage. Rather than write a synthetic payload, reused
    `KRName-Regular.otf`'s already-dumped JSON (it has enough repeated
    charstring structure that `-O2` measurably shrinks it: 21572 bytes at
    `-O0` vs 18464 at `-O2`) and added a `-O2 -k` build step to
    `compare-with-c.sh`, `compare-with-golden.sh`, and
    `generate-golden.sh`, labeled `KRName-Regular-O2.otf`. Confirmed
    byte-identical to a freshly built C toolchain on both macOS and Linux
    before freezing it as a new golden checksum entry — this is the
    highest-risk single change in this migration to date (the first
    genuinely new algorithmic path this port has had to prove correct
    against C from scratch, not just re-shape existing verified behavior),
    so it got its own from-scratch C comparison rather than relying on the
    frozen golden alone.
  - Verified with the standard full pipeline on both macOS and Linux, plus
    a fresh `compare-with-c.sh` run on both (not just golden) given the
    above. **`libcff/subr.rs` is now fully converted.
    ~4 uthash instances remain.**
- **uthash → `BTreeMap`/`HashMap`, twenty-fourth instance:
  `GlyphOrder.by_gid`/`by_name` (`rust/src/support/glyph_order.rs`)**, the
  largest and structurally most novel instance in this migration — a
  genuine dual index, not a pair of independent hashes like `CmapTable`'s
  `.unicodes`/`.uvs`. Every `GlyphOrderEntry` carried two simultaneous
  uthash handles (`hh_id`, `hh_name`), threading the *same* heap-allocated
  entry into two separate tables at once. Spans five files:
  `support/glyph_order.rs`, `json_reader.rs`, `table/post.rs`,
  `otf_reader/unconsolidate.rs`, `table/glyf.rs` — discovered by grep
  *before* starting, not by build-error surprise this time.
  - **Design confirmed before writing any code** (see the session's
    discussion): entries stay individually heap-allocated and raw-pointer-
    referenced, ownership model unchanged (deferred to Stage 6-4, matching
    every prior instance) — `by_gid`/`by_name` are non-owning indices over
    that one set of allocations, not owners in their own right. `by_gid:
    BTreeMap<GlyphId, *mut GlyphOrderEntry>` — no `HASH_SORT` ever existed
    on it, but `order_glyphs` (json_reader.rs) rebuilds it from scratch by
    inserting gids 0, 1, 2, ... in ascending order after sorting
    `by_name`, and the OTF-read path (`otfcc_set_glyph_order_by_gid`)
    inserts in the gid order its callers already iterate in — a
    `BTreeMap` reproduces the original's effective iteration order
    exactly. `by_name: HashMap<Vec<u8>, *mut GlyphOrderEntry>` — only ever
    point-looked-up by name day to day; the one place needing a different
    order (`order_glyphs`, sorting by `(order_type, order_entry)`, *not*
    alphabetically) already does its own explicit sort at the point of
    use, the same "sort key != dedup key, defer to drain time" shape as
    `LookupHash`/`FeatureHash`.
  - **Confirmed the two tables can never diverge in membership before
    touching the design**: every insertion path (`otfcc_set_glyph_order_by_gid`,
    `otfcc_set_glyph_order_by_name`) inserts into both tables in the same
    call; `set_order_by_name` (json_reader.rs, the JSON-parse-time
    registration path) touches `by_name` only and leaves `by_gid` untouched
    until `order_glyphs` rebuilds it wholesale at the end — so `by_gid`
    never accumulates placeholder/colliding gids that a `BTreeMap` could
    silently lose, the one scenario that would have broken this design.
  - **`dispose_glyph_order` walks `by_gid` once, frees each entry, then
    clears both maps** — matching the original's single-`HASH_ITER`-frees-
    both-indices shape, but simpler: `by_name` never owned anything of its
    own, so clearing it needs no walk at all.
  - **`.copy`/`.move_0`/`.replace`/`.copy_replace` deleted from
    `GlyphOrderPackage`, confirmed dead first**: all four were raw
    `memcpy`s of the whole `GlyphOrder` (incompatible with owned
    containers regardless), and crate-wide grep found none of the four
    called anywhere outside the vtable's own static initializer and each
    other — same shape as `table_cmap_copy`/`CFF_I_SUBR_GRAPH`'s dead
    slots. `.init`/`.dispose` kept (reachable indirectly through
    `.create`/`.free`, which *are* called externally, 594 times for
    `.set_by_gid` alone via `support/aglfn.rs`'s AGLFN static table).
  - **`otfcc_glyph_order_create`'s `malloc` needed no `calloc` fix** (the
    gasp/CFF-subr-graph trap): `init_glyph_order` was rewritten to
    `ptr::write` (placement-construct) rather than a plain field
    assignment, since its one live caller always hands it fresh,
    uninitialized memory — matching `cmap.rs`'s `init_cmap` precedent, and
    sidestepping the malloc/calloc question entirely rather than auditing
    for it.
  - **Found and fixed a second pre-existing C bug this session** (first
    was `cmap.c`'s UVS type confusion): `otfcc_gordConsolidateHandle`
    (`c/lib/support/glyph-order.c:83`) reads
    `HASH_FIND(hhName, go->byGID, &(h->index), sizeof(glyphid_t), t)` —
    the `hhName` selector on a search of `byGID` by a *gid* key. Since a
    glyph name is essentially never exactly `sizeof(glyphid_t)` (2) bytes
    long, this fallback search (meant to recover a stale `Consolidated`-
    state handle by index when its name no longer resolves) could never
    find anything — confirmed by finding the exact analogous, *correctly*
    typed search one branch down for `HANDLE_STATE_INDEX`
    (`otfcc_gordNameAFieldShared`, `hhID`/`byGID`), which is what this
    line was clearly meant to call. Fixed to a proper `by_gid.get(&h.index)`
    lookup. Confirmed via a full `compare-with-c.sh` run on both platforms
    that no committed payload exercises the diverging path (byte-identical
    to the still-buggy C throughout), so this is a correctness fix with no
    observable effect on any current payload — same category as the
    `LookupHash.alias`/`cmap.c` UVS fixes earlier in this migration.
  - **`table/post.rs`'s format-2.0 name table required rethinking one
    walk, not just retyping it**: `otfcc_build_post` originally walked
    `by_name`'s uthash chain directly for output, relying on that chain
    still being in the `(order_type, order_entry)` order `order_glyphs`
    had sorted it into earlier in the pipeline (in a different file) — an
    order this migration's `by_name: HashMap` does not and cannot
    preserve. Since that effective order is, by construction, the same as
    ascending gid order (gids were assigned 0, 1, 2, ... in exactly that
    traversal), rewrote the walk over `by_gid.iter()` instead, which is
    definitionally gid-ascending and reproduces the same byte sequence
    without leaning on `by_name`'s vanished implicit ordering.
    `otf_reader/unconsolidate.rs`'s AGLFN-derived-name registration loop
    (walking a *different* `GlyphOrder`, `post_name_map.by_gid`, to
    register names into the font's real `glyph_order`) needed no such
    rethinking — its two tables' entries have distinct, non-interacting
    gid spaces, so plain `.iter()` order was never load-bearing there.
  - **Real coverage**: every payload with any named glyphs exercises both
    tables end to end (`otfcc_set_glyph_order_by_name`/`_by_gid`,
    `order_glyphs`, `otfcc_gord_consolidate_handle`'s `Name`/`Index`
    branches, `otfcc_build_post`'s format-2.0 walk for `KRName-Regular.otf`
    specifically, since it's the one payload with a real `post` v2 table)
    — all of them, so no new synthetic payload was needed. All payloads
    stayed byte-identical to golden *and* to a freshly built C toolchain
    on both platforms; no golden regeneration needed.
  - Verified with the standard full pipeline on both macOS and Linux, plus
    a fresh `compare-with-c.sh` run on both given the scope and the
    deliberate bug fix. **`support/glyph_order.rs` is now fully converted.**
- **uthash migration complete.** A crate-wide grep right after this PR
  merged (`grep -rl "UtHashHandle\|vendor::uthash" src/`) turned up only
  `vendor/uthash.rs` itself — the running "~N uthash instances remain"
  counter threaded through this section had drifted low by a few over the
  course of the migration (an estimate re-derived from memory at each
  step, not a script-verified count), so the true final instance was this
  one, not a separately-numbered twenty-fifth. **Follow-up cleanup PR**
  deleted the now-fully-dead `vendor/uthash.rs` (44 lines: `UtHashHandle`/
  `UtHashTable`/`UtHashBucket` struct defs and the `HASH_*` size/threshold
  consts — the `HASH_ADD`/`HASH_FIND`/`HASH_SORT`/`HASH_ITER` *macros*
  were never here in the first place, since c2rust had already expanded
  every invocation inline at its call site during the original transpile)
  and the `pub mod uthash;` line in `vendor.rs`. Verified with the
  standard full pipeline on both macOS and Linux (no logic changed, so no
  `compare-with-c.sh` re-run needed — this was confirmed dead code
  removal, not a behavioral change).
- **Rust naming for the whole crate is done** (types, enum variants,
  constants, statics, locals, functions, struct fields and modules — see
  each above) and all three naming `allow`s are gone from `lib.rs`. Stage 4
  is complete.
- **Then safe Rust, type by type. Stage 6-1 (`CVecRaw<T>` → `Vec<T>`,
  ~37 container types) is done** — `support/cvec.rs` is deleted, its last
  consumer converted below. Next: `sds` → `String`, `caryll_Buffer` →
  `Vec<u8>`, `malloc`/`dispose` → `Box` + `Drop`, and the 7,682 `.offset()`
  calls into slices and iterators. Each of those PRs should end by deleting
  its files' `allow(unsafe_op_in_unsafe_fn)`: 120 files carry one today, and
  that count is the honest measure of how much of this crate is still C.
  - **First, though: the `Vec<T>` conversion's real obstacle wasn't the
    generic arithmetic in `cvec.rs`, it was that every container-owning
    struct is `#[derive(Copy, Clone)]` and gets memcpy'd wholesale by its
    own `move`/`replace`/`copy_replace` — a struct containing a real `Vec`
    can't be `Copy`, and a raw memcpy of one would alias two owners onto
    one heap allocation. Measured which of these were actually reachable
    (`grep`-checking `INSTANCE\s*\.\s*SLOT` outside each vtable's own
    initializer, across the whole crate) before touching anything: of 286
    non-`VQ` `move_0`/`replace`/`copy_replace` slots, only 4 are ever
    called from outside their own file, and `VQ`'s own `move_0` is dead
    too — every other one of the 1,185 total vtable slots exists only
    because c2rust's vtable-package macro stamped out the full C interface
    unconditionally (same cause as PR #43's 305 dead `extern "C"`
    declarations). A first same-line-only version of that grep undercounted
    by 4 real call sites, wrapped across lines by long identifiers
    (`OTL_I_CARET_VALUE_LIST\n    .move_0`); the compiler caught all 4
    before anything shipped, which is the whole reason this class of
    mistake is survivable here and division-by-repr(C) mistakes in the
    naming stage weren't as cheap to catch. Deleted the 284 confirmed-dead
    slots, then re-measured with `--force-warn dead_code` (not assumed)
    to find and delete the 311 functions that became unreachable as a
    result — converged in one pass, no cascade. `I_VQ`'s live
    `.replace`/`.copy_replace` (variable-font Pos blending) are the only
    survivors of this category; converting `VqSegList` to a real `Vec`
    is real, careful work — everything else in this list can just use
    `Vec`'s own `Clone`/`Drop`.
  - **Second pass, same method, next ten slot names**:
    `pop`/`init_n`/`init_cap_n`/`create_n`/`fill`/`dispose_item`/
    `filter_env`/`shrink_to_fit`/`sort`/`clear`. 339 of 370 instances
    across these ten are dead by the same reachability check; the 31
    real survivors are concentrated in `glyf` (`Contour`/`ContourList`
    get sorted, filtered, popped and shrunk during consolidation — the
    one table where these operations are actually exercised), a few
    `filter_env` calls pruning unreferenced OTL lookups/features, and a
    handful of `.sort` calls that fix dump order. Deleted the 339 dead
    entries, then 373 functions the compiler confirmed unreachable as a
    result (some second-order — a `_grow_to_n` helper that existed only
    to back the `_init_n` wrapper just deleted also goes), converging in
    one pass again. `init`/`dispose`/`copy`/`create`/`free` are left
    alone here: mostly live (56/63/38/35/40 of ~96), so auditing them
    down to their dead minority isn't worth it the way it was for slots
    that are 80–100% dead. What's left of the vtable surface after these
    two PRs is either genuinely load-bearing or small enough that a
    per-type `Vec<T>` conversion PR can absorb whatever's left of it
    directly.
  - **`VqSegList` → `Vec<VqSegment>` — the pilot.** `VQ.shift` (the vtable
    slots preserved above, `VqSegListVectorInterface`/`VQ_I_SEG_LIST`, and
    every `vq_seg_list_*` function) is gone; `VQ` is `struct { kernel: Pos,
    shift: Vec<VqSegment> }` and can no longer be `Copy` — this is the change
    every other container conversion in this list will also make, and it
    ripples further than the container itself:
    - `VQ` embeds by value inside `Point`, `ComponentReference`, `Glyph`
      (`table/glyf.rs`) and `CffFontMatrix` (`table/cff.rs`); all four lose
      `Copy` too. Nothing that holds these behind a pointer needs to
      change — a `*mut Point` is `Copy` regardless of what `Point` is — the
      loss only propagates through fields holding the struct *by value*.
    - `support/cvec.rs`'s `cvec_push`/`cvec_pop` required `T: Copy` (a plain
      dereference-assignment doesn't work for a type it can't just
      duplicate). Both are `ptr::write`/`ptr::read` now, which move the
      value instead of copying it — byte-identical for the ~36 element
      types that are still `Copy`, and it's what let `Contour`/
      `ReferenceList` keep holding non-`Copy` `Point`/`ComponentReference`
      without becoming part of this PR's scope.
    - Every call site that read a `VQ` field through a raw pointer as a
      plain expression (`(*gr).x`, `b.shift[p]`, …) stopped compiling —
      Rust won't silently duplicate a `Vec` the way it would a `Copy`
      struct. 139 sites across 6 files needed `.clone()` (`consolidate.rs`,
      `libcff/charstring_il.rs`, `table/cff.rs`, `table/glyf.rs`,
      `table/glyf/read.rs`, `otf_writer/stat.rs`) — the compiler's own
      `E0507`/`E0382` suggestions, checked rather than trusted: an earlier
      pass collected *every* suggested fix mechanically and briefly
      corrupted several files, because a handful of sites had a
      *different*, more invasive suggestion (hoist into a `let mut value =
      …;` binding) alongside the simple one, and both landed. Redone by
      filtering to pure, short insertions only; the complex suggestions
      turned out to be unnecessary; the simple `.clone()` at the same site
      was sufficient everywhere.
    - `vq_replace`'s `memcpy` of the whole struct — flagged as the reason
      this container needed care in the first place — becomes `*dst =
      src;`. Confirmed safe rather than assumed: every real call site
      passes `I_VQ.replace` a fresh temporary (the direct return of
      `point_linear_tfm(...)`), never a binding the caller reuses
      afterward, so the move this now performs matches what the memcpy
      already did in practice.
    - Two functions removed from `#[repr(C)]`/`extern "C"` VQ now trip
      `improper_ctypes_definitions` (`VQ` has no defined C layout once it
      owns a `Vec`) and `dangerous_implicit_autorefs` (indexing `(*x).shift`
      through a raw pointer implicitly creates a reference to the
      pointee). The first is `#[allow]`'d per file with the same rationale
      as `unsafe_op_in_unsafe_fn` — these `extern "C"` functions are vtable
      dispatch, never real FFI, and go away together in a later pass; the
      second was fixed properly, by binding `let shift = &mut (*x).shift;`
      once and indexing through that instead of re-deriving the reference
      from the raw pointer at every access.
  - **`GaspRecordList` → `Vec<GaspRecord>`.** Chosen as the next target by
    the same criteria used to pick `VQ`, applied up front instead of
    discovered mid-PR: `GaspRecord` is already `Copy` with no owned/nested
    pointers (no cascade), `GaspRecordList`/`GaspTable` have zero references
    outside `table/gasp.rs` (zero blast radius), and `GaspTable` is only ever
    passed by pointer, never by value (no `improper_ctypes_definitions`
    risk expected, and none appeared). The vtable-package boilerplate
    (`GaspRecordElementInterface`, `GaspRecordList`,
    `GaspRecordListVectorInterface`, and every `gasp_record_*`/
    `gasp_record_list_*` function) is gone; `GaspTable.records` is a plain
    `Vec<GaspRecord>`, and `GaspTable` itself drops `Copy` (it was never
    embedded by value anywhere, so nothing else loses `Copy` this time —
    no cascade, unlike `VQ`).
    - Hit the same `dangerous_implicit_autorefs` lint as `VQ`, fixed the
      same way: `let records: &Vec<GaspRecord> = &(*table).records;` bound
      once per function, then indexed/`.len()`'d through that.
    - Found a new failure mode `VQ` didn't have, because `VQ` never happened
      to trigger it: `table_gasp_create` allocated with `malloc`, and
      `init_gasp` assigns straight into the fresh struct (`(*gasp).records =
      Vec::new();`). That assignment drops whatever was already in
      `records` *first* — on `malloc`'s uninitialized bytes, that's a
      `Vec::drop` reading a garbage capacity and attempting to deallocate
      through a garbage pointer. Caught by `compare-with-c.sh`, not by the
      build: it compiled cleanly, then corrupted the heap on every payload
      that actually has a `gasp` table (crashed `otfccbuild` outright on
      three of them, silently produced a 12MB-divergent dump on a fourth).
      `VQ`'s own `vq_init` does the identical field-assignment, but every
      caller allocates through `__caryll_allocate_clean` (`calloc`), so the
      to-be-dropped bytes are zeroed and `Vec::drop` is a documented no-op
      at capacity 0 — safe by convention, not by construction. Fixed by
      allocating `table_gasp_create` with `calloc` instead of `malloc`, to
      match. Added to the per-type conversion checklist: any `_create` that
      `malloc`s its struct and then field-assigns a `Vec` into it needs the
      same fix, and the compiler cannot catch this — it compiles either way
      and only misbehaves at runtime, on payloads that exercise the table.
      Caught here only because `compare-with-c.sh` runs every payload, not
      because anything about the code looked wrong.
  - **`MetaEntries` → `Vec<MetaEntry>`.** Same selection criteria, same
    result: `MetaEntry` is `Copy` (it holds an `SdsRaw` pointer, but that's
    still a plain `Copy` field to Rust — the codebase doesn't yet know it's
    owned), `MetaEntries`/`MetaTable` are referenced nowhere outside
    `table/meta/`, and `MetaTable` is only ever behind a pointer. Applied the
    `table_gasp_create`-style checklist item up front this time — `libc::calloc`
    from the start, not discovered by a crash — and it was in fact needed:
    `init_meta_table` has the same `(*t).entries = Vec::new();` field
    assignment `init_gasp` did. One thing `GaspRecord` didn't have: each
    `MetaEntry` owns an `sds` string (`dispose_meta_entry` frees it via
    `sdsfree`), so `Vec<MetaEntry>`'s own `Drop` isn't enough — dropping the
    `Vec` only frees the array, not what each element's raw pointer points to.
    `dispose_meta_table` still walks the vector and disposes each entry
    first, same as before, just over `.iter_mut()` instead of the old
    `.items.offset(j)` loop. **Found a real, permanent coverage gap while
    checking this, not a bug**: no committed payload has a `meta` table, so
    every path this PR touched — `read`/`parse`'s `.push`, `build`/`dump`'s
    iteration, the `calloc`-vs-`malloc` fix itself — had never actually run
    under `compare-with-c.sh`. Same shape of gap as PR #36's unrecognised-
    lookup-type payload, so fixed it the same way: `make-test-meta.py`
    injects a synthetic `meta` block (one entry on each of the two
    known string tags, `dlng`/`slng`, plus one non-string tag through the
    base64 path) into an existing canonical payload JSON, and
    `compare-with-c.sh` now builds and dumps it every run — byte-identical
    both directions, on both platforms, closing the gap for future PRs too.
  - **Surveyed all 34 remaining containers at once** (`scripts/survey-containers.py`,
    committed here since the next ~10 PRs all need it) instead of re-deriving
    difficulty per type. The three questions that turned out to matter, checked
    for every container in one pass: does the element own a resource (`sds`,
    another container, a non-`Copy` type), is the container ever embedded *by
    value* outside its own file, and is it (or its element) ever passed by
    value through a function signature. Findings: `table/otl.rs` alone holds 14
    of the 34 (so PRs from here on are file-scoped, not type-scoped, except
    where a file mixes trivial and hard containers — `glyf.rs` does, `Contour`/
    `ContourList` next to `MaskList`/`StemDefList`); 9 are gasp-shaped
    (straightforward); several nest (`ColrLayerList` inside `ColrTable`,
    `CaretValueList` inside `LigCaretTable`, …) and have to go inside-out; 7
    hold `*mut T`/`*const T` elements and get converted to `Vec<*mut T>` only —
    `Vec<Box<T>>` is Stage 6-4's job, not this one, on the same
    don't-move-three-things-at-once lesson `VQ` taught; and `VV` is the
    counter-example that owning nothing doesn't mean easy — its element is a
    bare `c_double`, but it's embedded by value in `FvarInstance` and passed by
    value through an `extern "C" fn`, so it cascades exactly like `VQ` did.
  - **`VdmxGroup`/`VdmxRatioRangeList` → `Vec<VdmxRecord>`/`Vec<VdmxRatioRange>`
    — first nested pair, converted inside-out in one PR** (not two, per the
    plan's "no wasted intermediate PR" call for nesting pairs). Both `VdmxTable`
    and `VdmxRatioRange` were free of the by-value-elsewhere and
    passed-by-value hazards, so no cascade and no new lint. `table_vdmx_create`
    got the `calloc` fix up front, same as `MetaEntries`. Nothing here owns a
    resource, so — unlike `MetaEntries` — the whole per-element vtable
    (`VdmxRecordElementInterface`/`VDMX_I_RECORD`, `VdmxRatioRangeElementInterface`/
    `VDMX_I_RATIO_RANGE`) turned out to be pure boilerplate once the container
    itself became a `Vec`: every `.init`/`.dispose` call on a `VdmxRatioRange`
    in `funcs.rs` was invoked immediately after (init) or immediately before
    the value went out of scope with nothing pushed into it (dispose) — i.e.
    already redundant with the struct literal that constructed it, or with
    Rust's own drop glue. Deleted rather than translated; verified by checking
    every one of the three call sites by hand, not by assuming "no owned
    resources" implies "safe to delete" in general. Same coverage gap as
    `MetaEntries`: no payload has a VDMX table, so `make-test-vdmx.py` +
    `compare-with-c.sh` closes it the same way (two ratio ranges, one exercising
    the min/max scan over several size records).
  - **`ColrLayerList`/`ColrTable` → `Vec<ColrLayer>`/`Vec<ColrMapping>` —
    the first nested pair with an owning element, and the first with real,
    live `.copy` slots.** Three things distinguish it from every conversion so
    far, and any one of them alone would have made this a bigger PR than
    `VdmxGroup`'s:
    - `ColrLayer.glyph`/`ColrMapping.glyph` are `Handle`, and `Handle` owns an
      `sds` name string while staying `#[derive(Copy, Clone)]` (a crate-wide
      convention untouched by Stage 6 so far — fixing it is Stage 6-4's job).
      That means a *derived* `Clone` on anything containing a `Handle` would
      only alias the name pointer, not duplicate it — the same shape of trap
      `MetaEntry`'s `sds` field was, but this time it bites on the *copy* path,
      not just disposal. `ColrLayer`/`ColrMapping` are deliberately given no
      `#[derive(Clone)]` at all; `colr_layer_dup`/`colr_mapping_dup` do the
      real deep copy explicitly, through `otfcc_handle_dup` (already crate-
      provided), matching the crate's existing convention of never leaning on
      implicit Copy/Clone for a `Handle`-holding type.
    - Unlike every previous target, `TABLE_I_COLR.copy`/`.sort` are **not**
      dead — `otfcc_build_colr` uses both, on every build, to work on a sorted
      copy without mutating the caller's table. Converting them wasn't
      optional; `.sort` became `colr.sort_by(|a, b| a.glyph.index.cmp(&b.glyph.index))`
      (replacing `qsort`), `.copy` became `.iter().map(colr_mapping_dup).collect()`
      (the deep copy above, not `.clone()`).
    - `consolidate.rs`'s `consolidate_colr` — a second, separate file that
      reaches directly into `ColrTable`/`ColrMapping`/`ColrLayer`, not just
      `table/colr.rs` — mutates each mapping's and each layer's `Handle` in
      place (resolving glyph names to indices) while building a fresh,
      independently-owned consolidated table alongside it. Converted to index
      through `&mut Vec<ColrMapping>` / `&mut Vec<ColrLayer>` rather than
      `.items.offset(j)`, with `colr_layer_dup` reused for the new table's
      copies (no shortcuts specific to this file).
    - `ColrTable` itself is unlike every prior "table": in the C-derived
      shape it already *was* the vector (`length`/`capacity`/`items` directly,
      no wrapping struct with other fields), so it becomes
      `pub type ColrTable = Vec<ColrMapping>;` outright — no
      `ColrTableElementInterface` survives at all, just three plain functions
      (`table_colr_create`/`_free`, called from `consolidate.rs` and
      `caryll_font.rs` too) and `Vec`'s own methods everywhere else.
      `table_colr_create` uses `malloc` + `x.write(Vec::new())` — placement
      construction, not a field assignment — which sidesteps the
      `GaspTable`-style `calloc` requirement entirely: `.write()` never reads
      or drops the destination, so it doesn't matter what `malloc` left there.
    - Verified against real, live data, not just a synthetic payload: the
      only committed font with a COLR table
      (`BungeeColor-Regular_colr_Windows.ttf`) round-trips through
      `consolidate_colr` on every build, and the byte comparison covers it
      already — no new coverage gap to close here, unlike `MetaEntries`/`VDMX`.
  - **`CpalColorSet`/`CpalPaletteSet` → `Vec<CpalColor>`/`Vec<CpalPalette>` —
    second nested pair, back to `VdmxGroup`'s shape** (no owned resources,
    `.copy` dead) rather than `ColrLayerList`'s. Confirmed rather than assumed:
    `TABLE_I_CPAL.copy`/`CPAL_I_PALETTE_SET.copy`/`CPAL_I_COLOR_SET.copy` are
    referenced only inside their own vtables' static initializers, never
    called — same shape as every dead `.copy` slot before `ColrLayerList`,
    just checked again rather than pattern-matched on "it's a nested pair, so
    it must be like `VdmxGroup`." `table_cpal_create` got the `calloc` fix.
    One genuine behavioral wrinkle, found by reading `init_palette` field by
    field rather than assuming the vtable-removal was a no-op: it set
    `.label = 0xffff`, not the `0` the surrounding struct literal in
    `otfcc_read_cpal` used — and nothing between `.init()` and the palette's
    final push overwrote `.label` in that one call site (unlike the sibling
    call in `otfcc_parse_cpal`, where `.label` *is* unconditionally
    overwritten right after, making that `.init()` fully redundant). Deleting
    the `.init()` call there without noticing would have silently changed
    every palette's default label from `0xffff` (`no label`) to `0` (a real
    label index) whenever a font omits the CPAL palette-label array — the
    same "check every call site, don't assume `.init()` is always redundant
    once its container is already literal-initialized" lesson `VdmxGroup`
    taught, but this time the assumption would have been *wrong* instead of
    right. Verified against the same live BungeeColor payload as `ColrLayerList`
    (it has both COLR and CPAL) — no new coverage gap.
  - **`survey-containers.py`'s "straightforward" bucket had a blind spot of its
    own, caught before it steered a PR wrong: it never checked for `Handle`/
    `GlyphHandle` embedding**, only `SdsRaw`/`VQ`/known containers/raw
    pointers — so it silently missed that `Handle` (staying `Copy` while
    owning an `sds` name, same as `ColrLayer`) is exactly what all four of the
    `otl.rs` candidates it had called simple (`GposCursiveSubtable`,
    `GposSingleSubtable`, `GsubSingleSubtable`, `MarkArray`) actually embed —
    a plain regex on the element's own field list, one level deep, can't see
    a resource owned two levels down. Fixed by recursing into any
    capitalised, unrecognised field type instead of just special-casing a
    fixed list of names; every one of those four containers reclassified as
    `GlyphHandle`-owning the moment the fix landed. Second, independent gap in
    the same script: it has no notion of *which vtable slots are actually
    called* — the question PR #50/#51's grep-based reachability check
    answers — so it can call a container "straightforward" while missing that
    its `.sort` is live from a third file. That's exactly what happened to
    `MaskList`/`StemDefList` below: flagged simple by all three of the
    script's axes, but only found to have a live `.sort` (called from
    `consolidate.rs`, not `glyf.rs` itself) by grepping for it separately.
    Net effect: **after `GposCursiveSubtable`/`GposSingleSubtable`/
    `GsubSingleSubtable`/`MarkArray`'s reclassification and
    `MaskList`/`StemDefList`'s conversion, the "straightforward" bucket among
    the 26 remaining containers is empty** — every one of them needs the
    fuller checklist (element ownership two levels deep, vtable-slot
    reachability, other files touching the type directly) from the start now,
    not just the three original axes.
  - **`MaskList`/`StemDefList` → `Vec<PostscriptHintMask>`/`Vec<PostscriptStemDef>`
    — the widest-reaching container conversion yet, five files, no
    `Handle`-cascade** (the elements are plain data — fixed-size arrays and
    numbers — so unlike `ColrLayerList` this one stayed close to `VdmxGroup`
    in complexity once the file count is set aside). `.init`/`.dispose`/
    `.push`/`.sort` are all live; `.copy`/`.create`/`.free` confirmed dead the
    same way as every prior target and deleted. Both types are directly
    vector-shaped in the C-derived source (no wrapping struct), so — like
    `ColrTable` — they become `pub type StemDefList = Vec<PostscriptStemDef>;`/
    `pub type MaskList = Vec<PostscriptHintMask>;` outright.
    - `table/glyf.rs` itself: `otfcc_new_glyf_glyph` allocates via
      `__caryll_allocate_clean` (already calloc'd, same as `VQ`'s fields, so
      the plain `(*g).stem_h = Vec::new();` field assignments are safe by the
      same convention, not the `GaspTable`-style bug) and
      `otfcc_delete_glyf_glyph`/the dump and parse helpers convert the same
      way every prior container's dump/parse side has.
    - `consolidate.rs`'s `consolidate_glyph_hints` was the real center of
      gravity: it mutates each stem's `.map` in place, sorts, then builds a
      position-remap table (`hmap`/`vmap`) used to reorder every mask's bit
      array to match. The two `qsort`-based comparators (`by_stem_pos`,
      `by_mask_pointindex`) were kept as-is and wrapped in a `.sort_by`
      closure comparing their `c_int` result against `0`, rather than
      re-derived as native Rust comparisons — deliberately the lower-risk
      choice given how much else in this one function changed at once.
    - `table/cff.rs`'s `callback_draw_sethint`/`callback_draw_setmask` (the
      CFF-hint-parsing callbacks) and `libcff/charstring_il.rs`'s
      `_il_push_maskgroup`/`il_push_masks`/`_il_push_stemgroup`/
      `il_push_stems` (CFF charstring emission, the reverse direction) round
      out the five files — found only by grepping for the field names
      directly, not by trusting any single file's import list.
    - Verified against real, live data on both platforms:
      `KRName-Regular.otf` (the CFF payload) has `stemH`/`stemV`/`hintMasks`
      on its very first glyph, so both the parse-to-charstring and
      read-from-charstring directions, plus `consolidate_glyph_hints`'s
      remap logic, ran for real on every `compare-with-c.sh` and
      `run-cycles.sh` invocation — no synthetic payload needed here either.
  - **The reclassified `otl.rs` four turned out to be blocked by something the
    survey script has no axis for at all: a `#[derive(Copy, Clone)] union`.**
    Investigated as the next candidate after `MaskList`/`StemDefList`, since
    the script (post-fix) said all four (`GposCursiveSubtable`,
    `GposSingleSubtable`, `GsubSingleSubtable`, `MarkArray`) were merely
    `Handle`-owning, same shape as `ColrLayer`/`CaretValueRecord`. But
    `otl.rs`'s `Subtable` is `#[derive(Copy, Clone)] pub union Subtable { pub
    gsub_single: GsubSingleSubtable, pub gpos_cursive: GposCursiveSubtable,
    pub gpos_mark_to_single: GposMarkToSingleSubtable, … }` — every one of its
    ten variants must stay `Copy` for the union itself to derive `Copy`/
    `Clone` (a Rust union can't hold a non-`Copy` field without
    `ManuallyDrop` and losing the derive entirely). `MarkArray` is embedded
    by value inside `GposMarkToSingleSubtable`, which is itself a variant of
    `Subtable` — so `Vec`-ifying `MarkArray` alone would cascade into
    un-deriving the union, which cascades into every other variant type
    across the file, which is a different and much larger undertaking than a
    single-container conversion (likely turning `Subtable` into a tagged
    enum eventually, but that's its own project). `GposCursiveSubtable`/
    `GposSingleSubtable`/`GsubSingleSubtable` are variants too, so they're
    blocked the same way. None of the three axes in `survey-containers.py`
    (element ownership, byval-outside-file embedding, byval call signatures)
    look inside same-file unions, so this was invisible until read by hand.
    **Parked until `Subtable` itself is tackled** — not attempted this round.
  - **`CaretValueList`/`LigCaretTable` → `Vec<CaretValue>`/
    `Vec<CaretValueRecord>` (`table/gdef.rs`), picked instead once the
    `otl.rs` four turned out blocked.** Chosen in part *because* `GdefTable`
    (the file's outer table struct, embedding `LigCaretTable` by value) is
    only ever touched through `*mut`/`*const` pointers crate-wide, confirmed
    by grep before starting — no union, no by-value embedding anywhere
    outside `table/gdef.rs` and `consolidate/otl/gdef.rs`.
    - `CaretValue` (the innermost element) owns nothing, so `CaretValueList`
      becomes `pub type CaretValueList = Vec<CaretValue>;` outright, same as
      `ColrTable`/`MaskList`. `CaretValueRecord` embeds `GlyphHandle` (owns an
      `sds` name, stays `Copy` crate-wide pending Stage 6-4) exactly like
      `ColrMapping`, so it drops the derived `Clone`/`Copy` — but *unlike*
      `ColrMapping`, no explicit dup function was needed: every reachability
      check (`.copy`, `.replace`) came back dead, and the two live element
      producers (`otfcc_read_gdef`, `lig_caret_from_json`) always build a
      fresh `CaretValueRecord` and move it, never duplicate one.
    - `OTL_I_CARET_VALUE`'s `init`/`copy`/`dispose` were all `None` in the
      static initializer itself — not merely unreachable through some other
      path, but *literally never `Some` anywhere in the file* — so the
      per-element callback branches in the old list-copy/dispose functions
      were dead in a stronger sense than usual. `.clear` on
      `LigCaretTable`'s vtable turned out to share its function pointer with
      `.dispose` (`clear: Some(otl_lig_caret_table_dispose as …)`), and
      `.clear` *is* live from `consolidate_gdef` — so the dispose logic
      (walk every record, dispose its `Handle`, only *then* truncate) had to
      survive even though `.dispose` itself, called as its own field, is
      dead. Ported as one shared `clear_lig_carets` helper used by both
      `dispose_gdef` (whole-table teardown) and `consolidate_gdef`'s rebuild.
    - `table_gdef_create` was still using `malloc`, and `init_gdef` assigns
      straight into `(*gdef).lig_carets` (`= Vec::new()`) — the exact
      `GaspTable` hazard (rust/README.md), caught and fixed proactively
      *before* building this time by switching to `calloc`, not discovered
      by a crash afterward.
    - `table_gdef_copy`'s `memcpy`-based body was confirmed dead by the same
      grep-for-the-vtable-field-call method as every prior target (only ever
      assigned into the vtable static, never invoked through it or by name)
      and deleted outright rather than ported to `.clone()` — a bitwise copy
      would double-free `lig_carets` now that it owns a `Vec`.
    - `consolidate/otl/gdef.rs`'s `GdefLigCaretHash` (a uthash-style
      intermediate node used only while re-keying carets by consolidated
      glyph ID) embeds `CaretValueList` by value too, and is always
      allocated via `__caryll_allocate_clean` (calloc) — so the `.move_0`
      calls that swap a `CaretValueList` in and out of it convert cleanly to
      `std::mem::take`, same safe-by-convention story as `GaspTable`/
      `CpalTable`'s calloc'd `_create` functions, not a new bug. Dropped its
      `#[derive(Copy, Clone)]` (verified by grep: always touched through
      `*mut`/`*const`, never copied by value).
    - Following the `ColrTable`/`CpalTable` precedent of not stopping at the
      element level: `GdefTableElementInterface`/`TABLE_I_GDEF` (the whole
      outer vtable) was deleted too, even though its `.create`/`.free` were
      live and called cross-file from `font/caryll_font.rs` — replaced with
      plain `pub(crate)` `table_gdef_create`/`table_gdef_free` functions,
      matching how `caryll_font.rs` already calls `table_colr_free`/
      `table_cpal_free`/`table_vdmx_free` directly instead of through a
      vtable.
    - `read_lig_caret_record` returns `CaretValueRecord` by value from an
      `extern "C" fn` — not real FFI (only ever called from
      `otfcc_read_gdef` in the same crate), but the now-non-`repr(C)`,
      `Vec`-owning return type trips `improper_ctypes_definitions` the same
      way `VqSegList`'s vtable dispatch did in `vf/vq.rs` (PR #52). Silenced
      with a function-level `#[allow(improper_ctypes_definitions)]`, not a
      file-level one — this is the only site in the file that needs it.
    - Verified against real, live data on both platforms:
      `NotoNastaliqUrdu-Regular.ttf` genuinely has a non-empty `"ligCarets"`
      block, so `otfcc_read_gdef`/`otfcc_dump_gdef`/`otfcc_parse_gdef`/
      `otfcc_build_gdef` *and* `consolidate_gdef`'s hash-merge/mergesort path
      (which is what actually exercises `GdefLigCaretHash` and the
      `mem::take` moves) all ran for real on every `compare-with-c.sh` and
      `run-cycles.sh` invocation — no synthetic payload needed.
  - **`NameTable`/`TsiTable` → `Vec<NameRecord>`/`Vec<TsiEntry>` (`table/name.rs`,
    `table/_tsi.rs`, `consolidate.rs`, `font/caryll_font.rs`) — two containers
    done together in one PR** (the user explicitly OK'd a bigger, batched PR
    at this point rather than the usual one-container-at-a-time cadence).
    Chosen after the `otl.rs` four turned out parked behind the `Subtable`
    union: both are directly vector-shaped in the C-derived source (`length`/
    `capacity`/`items`, no wrapper struct), both have sds-owning elements with
    no `Handle`-of-`Handle` nesting, and both are single-table-file-scoped
    (`NameTable` touches only `name.rs` + `caryll_font.rs`; `TsiTable` touches
    `_tsi.rs` + `caryll_font.rs` + `consolidate.rs`) — no union embedding in
    either file, checked by hand before starting given the `otl.rs` lesson.
    - **`NameRecord` and `TsiEntry` both stay `#[derive(Copy, Clone)]`**, same
      as `MetaEntry` before them — an owned `sds` field alone doesn't block
      `Copy` in this crate's convention (only a `Handle` embedding forces the
      no-derive-plus-explicit-dup pattern, and `TsiEntry`'s embedded
      `GlyphHandle` still doesn't, for the same "leaf stays `Copy` until
      Stage 6-4" reason `CaretValueRecord`/`ColrLayer` do). This is safe
      specifically *because* each container's own whole-table `.copy` vtable
      slot (`table_name_copy`/`table_tsi_copy`, both `memcpy`- or
      elementwise-based) was confirmed dead by the same grep-the-vtable-field
      method as every prior target, and got deleted outright rather than
      ported — so nothing ever relies on `Vec<NameRecord>: Clone` or
      `Vec<TsiEntry>: Clone` doing a deep copy that would otherwise alias
      every record's owned pointers.
    - **`TsiEntry` needed a real duplicate function where `CaretValueRecord`
      didn't** — unlike the gdef.rs pair (PR #59), where every `.copy`/
      `.replace` path turned out dead, `TSI_I_ENTRY.copy` (the per-element,
      not whole-table, copy) is genuinely called once, directly, from
      `consolidate_tsi` in `consolidate.rs` — duplicating a non-`Glyph`-type
      entry (`Fpgm`/`Prep`/`Cvt`/`ReservedFffc`) while the `Glyph`-type
      entries in the same loop are handled by content-*move* instead (steal
      `.content`, null the source) rather than duplication. Ported as
      `tsi_entry_dup`, calling `otfcc_handle_dup` + `sdsdup` exactly like the
      original `copy_tsi_entry` did — checking every call site rather than
      assuming "no cascade means no dup function needed" (`CaretValueRecord`'s
      lesson) would have missed this one.
    - **Both `.sort` slots were live** (`NameTable`'s from
      `otfcc_parse_name`, `TsiTable`'s from `consolidate_tsi`) and both
      comparators are simple lexicographic tie-breaks over small integer
      fields (`platform_id`/`encoding_id`/`language_id`/`name_id` for names;
      `type_0`/`glyph.index` for TSI entries) with no floating point and no
      aliasing concerns — ported as native `.sort_by` chains of `.cmp().then(…)`
      rather than wrapping the old `qsort`-calling comparator (unlike
      `consolidate_glyph_hints`'s stem sort in PR #58, which wrapped
      instead). Accepted the theoretical risk that libc's `qsort` (unstable)
      and Rust's `slice::sort_by` (stable) could permute *fully tied* records
      differently — judged acceptable because every one of the two sort
      call sites is exercised by nearly every payload already in
      `compare-with-c.sh` (every font has a `name` table; `vtt.ttf` has real
      `TSI_01`/`TSI_23` data), and all of them came back byte-identical.
    - `table_name_create`/`table_tsi_create` both switched to the
      `malloc` + `.write(Vec::new())` placement-construction pattern
      (`ColrTable`/`MaskList`'s style, not `GaspTable`'s `calloc`) since both
      types are bare `Vec` aliases with no wrapper struct — no field
      assignment ever reads what `malloc` left behind, so the `calloc` fix
      doesn't apply and wasn't needed.
    - Verified against real, live data on both platforms: every payload's
      `name` table exercises `otfcc_parse_name`'s sort (build direction);
      `vtt.ttf` (a VTT-hinted font) has genuine `TSI_01`/`TSI_23` data,
      exercising `otfcc_read_tsi`/`otfcc_dump_tsi`/`otfcc_parse_tsi`/
      `otfcc_build_tsi` *and* `consolidate_tsi`'s hash-based gid remap +
      sort — no synthetic payload needed for either container.
  - **Re-measured after PR #60: the plan's "4 containers frozen behind
    `Subtable`" was an undercount — it's actually 8.** `BaseArray`
    (`GposMarkToSingleSubtable.base_array`) and `LigatureArray`
    (`GposMarkToLigatureSubtable.lig_array`) are embedded *by value inside a
    union variant's own struct*, one level deeper than `MarkArray`'s direct
    embedding, so the earlier hand-audit (which only checked variant types
    themselves, not their fields) missed them; `GsubLigatureSubtable`/
    `GsubMultiSubtable` are union variants directly and were miscounted the
    same way `GposCursiveSubtable`/`GposSingleSubtable`/`GsubSingleSubtable`
    were before. All 8 need `Subtable`'s `#[derive(Copy, Clone)] union` to
    stop requiring every variant to be `Copy` before any of them can move to
    `Vec`. But `Subtable` itself turned out to have **zero by-value uses**
    anywhere in the crate (grepped every occurrence: all `*mut Subtable`/
    `*const Subtable`/`size_of::<Subtable>()`/`null_mut::<Subtable>()`) — so
    the fix isn't the full tagged-`enum` rewrite the plan assumed, just
    wrapping the non-`Copy` variant types in `ManuallyDrop`, deferred to
    after the other 14 working containers (svg/fvar-vf/otl-pointer-arrays/
    glyf) so the audit trail for *why* each of those 14 doesn't touch the
    union exists before touching the union itself. Three `memcpy(...,
    size_of::<Subtable>())` sites (`otf_reader/unconsolidate.rs:482,502`,
    `table/otl/subtables/extend.rs:28`) will need individual porting once a
    variant becomes `Vec`-owning, since a bitwise union copy would then
    double-free.
  - **`SvgTable` → `Vec<SvgAssignment>`** (`table/svg.rs`,
    `font/caryll_font.rs`) — the first of the 14 containers no longer
    blocked by the union, picked because it was already directly
    vector-shaped in the C-derived source (no wrapper struct, like
    `ColrTable`) and touches only two files.
    - The element's owned resource is a `*mut Buffer` (`buffree`), not an
      `sds`/`Handle` — a shape not seen before in this series. `Copy` stays
      on `SvgAssignment` (copying the pointer bytes is fine; nothing reads
      that copy as a deep clone), but `otfcc_build_svg`'s sorted-copy path
      needs a *real* duplicate, so `svg_assignment_dup` does the same
      `bufnew()` + `bufwrite_buf()` the old `copy_svg_assigment` did —
      `.clone()` alone would alias the same `Buffer`.
    - Unlike every prior candidate in this series, **both `TABLE_I_SVG.copy`
      and `.sort` were live**, not dead vtable slots: `otfcc_build_svg` calls
      `.copy` to get an owned, disposable copy before sorting it by
      `start` glyph ID and disposing it again, without touching the
      caller's original table — the same "build a sorted copy, leave the
      original alone" shape `ColrTable`/`otfcc_build_colr` had. Ported as
      `(*_svg).iter().map(svg_assignment_dup).collect()` +
      `.sort_by(|a, b| a.start.cmp(&b.start))`, replacing the `qsort`-driven
      `by_start_gid` comparator with a native comparison (first done for
      `NameTable`/`TsiTable` in PR #60) since it's a single-field integer
      tie-break with no aliasing concerns.
    - `table_svg_create` was already `malloc` + `.write(Vec::new())`
      placement construction, not a field assignment — no `calloc` fix
      needed, same reasoning as `ColrTable`/`NameTable`/`TsiTable`.
    - Verified against real, live data on both platforms:
      `Reinebow-SVGinOT.ttf` (already in `compare-with-c.sh`/`run-cycles.sh`)
      has a genuine `SVG ` table, exercising `otfcc_read_svg`/
      `otfcc_dump_svg`/`otfcc_parse_svg`/`otfcc_build_svg` end to end — no
      synthetic payload needed.
  - **`VfAxes`/`VV`/`FvarInstanceList` → `Vec<VfAxis>`/`Vec<Pos>`/
    `Vec<FvarInstance>`** (`vf/axis.rs`, `vf/vv.rs`, `vf/vq.rs`,
    `vf/region.rs`, `table/fvar.rs`, `table/glyf/read.rs`) — the second
    container group, a nested/non-nested mix converted in dependency order:
    `VfAxes` (standalone) → `VV` (the innermost, embedded in `FvarInstance`)
    → `FvarInstanceList` (the outer table).
    - `VfAxes`: as expected, `VfAxis` owns nothing (the old
      `vf_axis_dispose` was already an empty function). The live slots
      (`.init`/`.dispose`/`.push`/`.shrink_to_fit`, all called from
      `otfcc_read_fvar`) went straight to `Vec` methods; the whole-table
      `.copy` was confirmed dead (never called via the vtable field) and
      dropped along with `VfAxisElementInterface`/`VfAxesVectorInterface`.
    - `VV`: turned out to be the same `Copy`-cascade shape as `VQ`
      (embedded by value in `FvarInstance`, plus a by-value call site,
      `json_new_vv(x: VV, …)`) — but simpler in the end, because the
      element (`Pos`, an `f64`) owns nothing at all. That let the *entire*
      `vf/vq.rs` VV apparatus (`PosElementInterface`, `VQ_I_POS_T`,
      `vv_init`/`vv_push`/`vv_copy`/`vv_dispose`/`vv_shrink_to_fit`/
      `vv_create`/`vv_free`/`vv_init_n`/`vv_fill`/`create_neutral_vv`, the
      `I_VV` static) disappear outright — every live call site
      (`.init`/`.push`/`.shrink_to_fit`/`.dispose`, all in `table/fvar.rs`)
      converts directly to a `Vec<Pos>` method call, no wrapper functions
      needed. **`json_new_vv` (the by-value sibling of the live
      `json_new_v_vp`) turned out to be dead** — never called anywhere,
      confirmed by grep, not assumed from "it has a by-value signature so
      it must be live" (the plan's own worry going in). Deleted rather than
      ported; `json_new_v_vp` (and `vq_region_get_weight` in
      `vf/region.rs`, and `polymorphize`/`json_new_vq_region_explicit` in
      `table/glyf/read.rs`/`table/fvar.rs`, all genuinely live) had their
      `.length`/`.items.offset(...)` reads converted to `Vec` `.len()`/
      indexing, binding a `&Vec<T>` once per function first as usual for
      `dangerous_implicit_autorefs`.
    - `FvarInstanceList`: `fvar_instance_list_create` used `malloc` and
      `table_fvar_create` (the *outer* `FvarTable`, which owns `axes`/
      `instances` by value) did too — both switched to `calloc` before
      building, the `GaspTable` fix applied proactively this time. **New
      wrinkle, not seen in earlier containers**: `FvarInstance` only owns
      another `Vec` (`coordinates: Vec<Pos>`), not a raw pointer, so unlike
      `SvgAssignment`/`NameRecord`/`TsiEntry` it needs **no per-element
      dispose function at all** — `Vec<FvarInstance>`'s own `Drop` already
      recurses into every instance's `coordinates` for free, since Rust
      generates that drop glue automatically for a struct holding a `Vec`
      field. `dispose_fvar` shrank to two plain reassignments
      (`(*fvar).axes = Vec::new(); (*fvar).instances = Vec::new();`).
      `FVAR_I_INSTANCE.copy` (the old `memcpy`-based `FvarInstance` copy)
      was confirmed dead by walking one level up, not just grepping its own
      name — the same "a caller existing in text isn't proof of being
      called" check `VdmxRatioRangeList` needed. `FvarTable` itself lost
      `#[derive(Copy, Clone)]` entirely (not even `Clone`): it's always
      reached through `*mut`/`*const` (`Font.fvar: *mut FvarTable`), never
      embedded or passed by value anywhere in the crate.
    - **The coverage gap the plan predicted was real**: `make-test-
      variable-font.py`'s single-axis (`wght`) gvar-test font had no named
      instances, so `FvarInstanceList`/`VV`'s read+dump path had never run
      in `compare-with-c.sh`/`run-cycles.sh`. Fixed the same way as
      `meta`/`VDMX`/unknown-lookup: added two `InstanceDescriptor`s
      (Regular/Bold) to the designspace document, confirmed with
      `fontTools.ttLib` that the built font actually carries 2 instances,
      then re-ran the full suite — `gvar-test` stayed byte-identical in
      both directions on both platforms.
  - **The `otl.rs` pointer-array group → `Vec<*mut T>`/`Vec<*const T>`**
    (`SubtableList`, `LookupList`, `FeatureList`, `LangSystemList`,
    `LookupRefList`, `FeatureRefList` — `table/otl.rs` plus 11 consuming
    files, 234 call sites) — the third container group, and the widest one
    yet by file count. Deliberately stopped at `Vec<*mut T>`/`Vec<*const T>`
    (plan classification "その3"): the pointee `Box`-ification is Stage 6-4's
    job, kept separate from this pass on purpose — moving both the container
    shape and the element ownership at once is exactly what made the `VQ`
    conversion (PR #52) the riskiest one in this series.
    - Four owning structs (`Lookup.subtables`, `Feature.lookups`,
      `LanguageSystem.features`, `OtlTable.{lookups,features,languages}`)
      all lost `#[derive(Copy, Clone)]`; none is ever constructed as a bare
      value or passed by value anywhere in the crate, so no `Clone` was
      needed either.
    - **`SubtableList`'s disposal can't be an ordinary `Drop`.** `Subtable`
      is a bare `#[repr(C)] union` with no discriminant of its own — the
      tag that says which variant is live lives one level up, on the
      *enclosing* `Lookup.type_0`. Freeing a subtable correctly (walking
      and freeing that variant's own nested allocations) needs the lookup's
      type, not just the pointer, which is exactly why the existing
      `dispose_subtable_dependent(subtable_ref, lookup)` helper takes both
      — its signature didn't need to change at all (it already operated on
      a single element pointer, independent of container shape), only the
      three callers walking `SubtableList` needed updating.
    - **`.clear()` vs. full reassignment, made concrete for the first
      time**: several of this group's dispose paths (`otfcc_delete_lookup`,
      `dispose_otl`/`table_otl_free`) are followed immediately by the
      caller raw-`free()`-ing the *enclosing* struct. `Vec::clear()` only
      sets `len = 0` — it leaves the backing allocation (a separate heap
      block from the enclosing struct) untouched, and a raw `libc::free()`
      of the enclosing block never runs `Drop`, so `.clear()` here would
      leak the backing array every time. Used `*arr = Vec::new()` instead
      at every "final dispose before the enclosing block is freed" site —
      it drops (and actually deallocates) the old `Vec` before the empty
      replacement moves in. `.clear()` stays correct only where a container
      is genuinely reused in place afterward, the way `consolidate_gdef`
      already did with `clear_lig_carets`.
    - `LookupRefList`/`FeatureRefList` (the two non-owning ones, holding
      `*const Lookup`/`*const Feature` borrowed from the owning
      `LookupList`/`FeatureList`) needed no per-element dispose at all — a
      grep of every push site confirmed each one copies a pointer *value*
      already owned elsewhere, never a stack address or an address into the
      owning container's own backing buffer, so disposal is just dropping
      the backing array. Their `.replace` slots (found live for the first
      time in this series — PR #50 counted them among the 4 genuinely-live
      `.replace`/`.move_0` survivors) turned out to always target a
      freshly-`init_feature_ptr`/`init_language_ptr`'d, still-empty
      destination at their one call site each, so `otl_lookup_ref_list_replace`/
      `otl_feature_ref_list_replace` reduce to a plain `*dst = src;`
      move-assignment rather than anything more involved.
    - The old index-swap-and-truncate compaction loops (pruning null'd-out
      subtables in `consolidate.rs`, filtering dead lookups/features/refs
      in `consolidate_otl_table`) ported two ways depending on shape: the
      `LookupRefList`/`FeatureRefList`/`LookupList`/`FeatureList` ones
      became plain `Vec::retain`/`.retain_mut` closures (no per-element
      cleanup needed beyond calling the existing dispose helper inside the
      closure); `consolidate.rs`'s own subtable-pruning loop kept its
      original two-pass shape (null out, then compact) rather than being
      re-derived, matching this series' standing preference for wrapping
      over re-deriving in an already-large change.
    - **`dangerous_implicit_autorefs` fires on `[idx]` indexing through a
      raw-pointer-derived place, but not on plain method calls like
      `.len()`/`.push()`** on the same place — a distinction this series
      hadn't needed to draw before, because `Index`/`IndexMut` operator
      dispatch is the specific thing the lint targets, not method calls in
      general. At this file count (234 call sites across 12 files) fixing
      each by hand would've been the real risk; instead, `cargo build
      --message-format=json` was used to extract every diagnostic's
      structured `suggested_replacement` (rustc already computes the
      exact right fix per call site, correctly distinguishing `&` for reads
      from `&mut` for write targets — including the `a[i] = a[j]` case,
      where the RHS index needs `&` and the LHS needs `&mut` in the same
      statement), and a small script applied them by line/column offset.
      (Plain `cargo fix --broken-code` was tried first and left every one
      of these unapplied across five passes — the suggestions carry
      `Applicability::MaybeIncorrect`, which `cargo fix` intentionally
      skips; only the JSON-diagnostics route got the machine-computed fixes
      applied.)
    - Verified against real, live data on both platforms: every payload's
      GSUB/GPOS tables exercise the full read/build/dump/parse/consolidate
      paths through these six containers (`vtt.ttf`, `NotoNastaliqUrdu-
      Regular.ttf`, and `iosevka-r.ttf`'s issue #1 golden test all touch
      `Lookup`/`Feature`/`LanguageSystem` heavily already) — no synthetic
      payload needed.
    - **Found, not fixed, in this PR**: while working through the "dispose
      right before the enclosing block is freed" pattern above, the same
      `.clear()`-then-raw-`free()` shape turned up in a few already-merged
      containers (`SvgTable`, `NameTable`/`TsiTable`, `GdefTable`'s lig-caret
      path) — a real backing-array leak, invisible to every test here since
      leaks don't change output bytes. Flagged separately rather than fixed
      as part of this PR's scope.
  - **`Contour`/`ContourList`/`ReferenceList`/`GlyfTable` → `Vec<Point>`/
    `Vec<Contour>`/`Vec<ComponentReference>`/`Vec<*mut Glyph>` (`table/glyf.rs`
    and its `read`/`build` submodules, `table/cff.rs`, `libcff/charstring_il.rs`,
    `consolidate.rs`, `otf_writer/stat.rs`, `otf_reader/unconsolidate.rs`,
    `font/caryll_font.rs`) — the last of the four groups from the "作業可能な
    14型" plan, done last on purpose since it's the deepest one.**
    - **The same container asymmetry as `ColrLayerList`/`NameTable` shows up
      again, but split across two of the four types instead of one**:
      `Contour`'s only element (`Point`) owns nothing but further `VQ` `Vec`s,
      so `Contour`/`ContourList`'s disposal is fully automatic — no manual
      dispose function survives at all, `(*g).contours = Vec::new();` at a
      teardown site is the whole story. `ReferenceList`'s element
      (`ComponentReference`) embeds a `GlyphHandle`, which stays `Copy` and
      un-auto-dropped by this crate's convention, so it keeps one small
      helper, `dispose_reference_list`, that loops calling the untouched
      `GLYF_I_COMPONENT_REFERENCE.dispose` before replacing the `Vec`.
    - **`GlyfTable` is the "その3" pointer-array shape** (`Vec<*mut Glyph>`,
      same treatment as the six `otl.rs` containers in the previous PR) —
      `Glyph` itself stays behind a raw pointer, so `table_glyf_free` still
      calls `otfcc_delete_glyf_glyph` per slot (nulls tolerated, that
      function already no-ops on null) before dropping the pointer `Vec`.
      `table_glyf_create`/`table_glyf_create_n` switched to the `malloc` +
      `.write(...)` placement-construction pattern (`ColrTable`'s style) —
      `create_n` places `vec![ptr::null_mut(); n]` directly, replacing three
      separate helper functions (`table_glyf_init_n`/`_grow_to_n`/`_fill`).
    - **`consolidate_glyph_contours`/`consolidate_glyph_references` ported to
      `Vec::retain`/`.retain_mut` rather than translating the original
      index-shift-and-truncate compaction loops literally** — same choice as
      the `LookupList`/`FeatureList` pair in the previous PR, and it lands
      differently for the two functions here precisely because of the
      dispose asymmetry above: the contours closure needs nothing beyond the
      existing warning-log call for a dropped element (Rust's own drop glue
      handles the rest), while the references closure has to call
      `GLYF_I_COMPONENT_REFERENCE.dispose` on the element explicitly before
      returning `false`, because `retain`'s internal compaction drops
      rejected elements the normal way and Rust doesn't know a `Handle`
      needs `otfcc_handle_dispose`. `retain_mut` (not `retain`) was needed
      for the references side specifically because `consolidate_handle`
      mutates the candidate's `.glyph` field in place before deciding
      keep/drop.
    - **The scratch `Contour` array in `libcff/charstring_il.rs`'s
      `cff_compile_glyph_to_il`** (a `__caryll_allocate_clean`-allocated
      `*mut Contour` used as a fixed-size working copy of a glyph's
      contours, disposed at the end of the same function) is calloc'd, so
      the same "field assignment onto zeroed memory is safe because a
      zero-capacity `Vec`'s drop never touches its pointer field" reasoning
      already used for `vq_init` applies again — `*newcontour = Vec::new();`
      needs no `calloc`-vs-`malloc` fix, unlike `GaspTable`'s original bug.
      The matching disposal loop switched from a vtable dispose call to
      `ptr::drop_in_place`, since each slot is by then a genuine
      placement-constructed `Vec<Point>`, not calloc garbage.
    - **`dangerous_implicit_autorefs` at similar scale to the previous PR**
      (258 machine-applied edits across 8 files) — the JSON-diagnostics
      extraction script from the `otl` PR wasn't committed anywhere, so it
      was rewritten from scratch rather than recovered; same approach
      (`cargo build --message-format=json`, apply `suggested_replacement`
      spans by line/column, sort descending per file so earlier splices
      don't shift later offsets). One new wrinkle this size didn't hit
      before: rustc sometimes emits *two* separate spans for a single
      doubly-indexed expression like `(*g).contours[c][pj]` (one wrapping
      the outer index, one the inner), and applying both blindly by naive
      column offset produced a handful of `)[` mismatched-paren splices —
      caught immediately by the next build (unbalanced delimiters, not a
      silent wrong-output-shape hazard) and fixed by hand, three call sites
      total.
    - **No synthetic payload needed** — `glyf` is the one table nearly every
      payload here already exercises through every code path this PR
      touches: `KRName-Regular.otf`'s CFF outline extraction has real
      `stemH`/`stemV` and multi-point contours, `iosevka-r.ttf`/`vtt.ttf`
      drive the TrueType `glyf` read/build/dump/parse round trip at scale,
      and `consolidate_glyph_contours`/`_references`'s empty-contour and
      dangling-reference branches are hit by the existing corpus. All of
      `compare-with-c.sh`/`run-cycles.sh`/the roundtrip comparisons/the
      issue #1 golden test came back clean on both platforms.
  - **`Subtable`'s `ManuallyDrop` wrap unblocks the last 8 containers**
    (`MarkArray`/`BaseArray`/`LigatureArray`/`GposCursiveSubtable`/
    `GposSingleSubtable`/`GsubSingleSubtable`/`GsubLigatureSubtable`/
    `GsubMultiSubtable` → `Vec<T>`) — the fix flagged two entries up as
    "deferred until the other 14 working containers exist", now applied.
    **This finishes Stage 6-1**: `support/cvec.rs`'s `CVecRaw<T>` had zero
    remaining consumers once these 8 landed, so the file (and its `mod
    cvec;` line, and the five unit tests inside it) is deleted outright
    rather than left as dead code.
    - **The mechanism is exactly what the earlier note predicted, and
      cheaper than it looked.** A Rust union can't hold a non-`Copy` field
      without `ManuallyDrop<T>` wrapping it — wrapping just the 7 affected
      variants (`chaining`/`gsub_reverse`/`gpos_pair`/`extend` stay bare)
      and dropping `#[derive(Copy, Clone)]` from the union itself was the
      whole fix. `ManuallyDrop<T>` is `#[repr(transparent)]`, so every
      extraction site keeps its existing `&raw mut/const
      (*subtable).field` syntax and only needs one appended cast — `as
      *mut/*const ActualType` — to recover the real type; nothing
      downstream (indexing, `.push`, `.len()`, …) changes. That kept the
      touched surface to ~22 union-field extraction sites across 14 files,
      not the 32-file/364-site figure the plan had estimated for a full
      tagged-`enum` rewrite — because a `ManuallyDrop` wrap doesn't change
      how the union is accessed, only what's inside each field.
    - **`GposMarkToSingleSubtable`/`GposMarkToLigatureSubtable` lost their
      own `#[derive(Copy, Clone)]` too**, for the same reason as the union:
      once `mark_array`/`base_array`/`lig_array` are real `Vec`s, the host
      structs can't be `Copy` either. Both were confirmed never passed or
      embedded by value outside their own files first.
    - **The `.copy` vtable slots on both `mark_to_single`/`mark_to_ligature`
      element interfaces were dead** (grepped for the interface-static name
      being called, not just the function name — the file's own `.expect()`
      call inside the vtable initializer doesn't count as a caller), so the
      two `memcpy`-based `subtable_gpos_mark_to_X_copy` functions were
      deleted outright rather than ported to `.clone()`, same as every
      earlier dead-`.copy` case in this migration.
    - **`GsubLigatureSubtable` is the one type with a live `.replace`**
      (`consolidate_gsub_ligature` builds a filter-and-transfer-ownership
      `nt: GsubLigatureSubtable` and swaps it in for the old subtable).
      `subtable_gsub_ligature_replace(dst, src)` went from `dispose(dst);
      memcpy(dst, &src, size_of::<X>())` to `dispose_gsub_ligature_subtable
      (dst); *dst = src;` — a safe move-assign, since every call site passes
      a fresh, never-reused local. Taking `src: GsubLigatureSubtable` by
      value in an `extern "C" fn` trips `improper_ctypes_definitions` (a
      `Vec` has unspecified layout) exactly the way `read_lig_caret_record`
      did in the `CaretValueList` PR — same fix, a function-level
      `#[allow(improper_ctypes_definitions)]`, since the function is never
      actually called across a real FFI boundary, only used for
      vtable-shaped internal dispatch.
    - **The consolidate-side call sites were more work than the plan's
      hand-audit had scoped.** The plan's "re-measured after PR #60" note
      (two entries up) only tracked the `Subtable`-union extraction casts;
      it missed that `consolidate/otl/{gpos_cursive,gpos_single,gsub_single,
      gsub_multi,gsub_ligature,mark}.rs` each run a full uthash-based
      dedup-and-rebuild pass directly against these containers'
      `.length`/`.items.offset(...)` shape, with their own
      `I_SUBTABLE_X.clear`/`.push` vtable calls at the end (`mark.rs` alone
      has three such passes, for `mark_array`/`base_array`/`lig_array`).
      Every `dispose_X_subtable`/`dispose_mark_array`/`dispose_base_array`/
      `dispose_lig_array` helper written for the `table/otl/subtables/*.rs`
      side had to be made `pub(crate)` and imported here too, since the old
      `.clear` vtable slot was always an alias for the full dispose (grepped
      each vtable's own static initializer to confirm before relying on
      that), not a capacity-preserving truncate — the established "`.clear`
      that's actually a full dispose must not become `Vec::clear()`" rule
      from earlier PRs applied seven more times.
    - **The two `malloc`-based host-struct `_create` functions were
      switched to `__caryll_allocate_clean` (calloc) up front**, applying
      the `GaspTable` lesson proactively rather than discovering it via a
      crash: both `subtable_gpos_mark_to_single_create` and
      `subtable_gpos_mark_to_ligature_create` assign `Vec::new()` into two
      fields of a freshly allocated struct, and that assignment implicitly
      drops whatever was already there — safe only when the memory started
      zeroed (a zero-capacity `Vec`'s drop is a no-op regardless of what its
      pointer field holds).
    - **`dangerous_implicit_autorefs` at the same scale as the previous two
      PRs, but with none of their manual-fixup wrinkles** — 290
      machine-applied edits across 15 files (the JSON-diagnostics script
      from the `glyf` PR, rewritten from scratch there since it was never
      committed, needed no changes this time); no double-span mismatched-
      paren case turned up.
    - **No synthetic payload needed**: `NotoNastaliqUrdu-Regular.ttf`
      already exercises all 8 of these lookup types end to end (confirmed
      by tabulating every payload's lookup `"type"` values via
      `otfccdump` before starting), and it was already in
      `compare-with-c.sh`'s payload list. All of
      `compare-with-c.sh`/`run-cycles.sh`/the roundtrip comparisons/the
      issue #1 golden test came back clean on both platforms, both before
      and after deleting `cvec.rs`.
    - **The three `memcpy(…, size_of::<Subtable>())` sites the plan had
      flagged as risky turned out to be irrelevant to this PR** —
      `otf_reader/unconsolidate.rs:482,502` and
      `table/otl/subtables/extend.rs:28` are `__caryll_allocate_clean`
      (calloc) calls sized to the *whole* union for the `chaining`/`extend`
      variants, which stayed bare (not `ManuallyDrop`-wrapped) and were
      never touched here.
- **CI decoupled from C: `tests/golden/` now carries `compare-with-c.sh`'s
  job.** (This bullet fixes a stray, incomplete sentence fragment that used
  to sit here — "`tests/golden/` and hand `compare-with-c.sh`'s job over to
  it — otherwise removing C removes the safety net that makes all of this
  checkable" — orphaned from whatever it was originally attached to; the
  work it was gesturing at is what this bullet actually describes.)
  Explicit instruction: it is fine for the Rust crate to diverge from C
  from here on, but CI must keep confirming the *build output* stays
  correct — so this captures C's approval as a frozen snapshot rather than
  dropping the check.
  - **What moved**: for every payload `compare-with-c.sh` already covered
    (the 6 TTF + 1 CFF payloads, the gvar variable-font payload, the
    synthetic `unknown-lookup`/`meta-test`/`vdmx-test` payloads, and the
    `otfccdll` cdylib), the dump JSON and build output that `compare-with-c.sh`
    had just finished confirming byte-identical to C (PR #78's full-pipeline
    run, both platforms) are hashed (SHA-256) into
    `tests/golden/checksums.sha256`. `rust/scripts/compare-with-golden.sh`
    reproduces the exact same dump→build comparisons `compare-with-c.sh`
    did, hashing its own freshly-produced output and checking it against
    the recorded checksum, instead of `cmp`-ing against a freshly built C
    binary — CI no longer installs clang, builds `c/`, or needs `c/`
    checked out at all. The CI workflow's `paths:` trigger dropped `c/**`
    accordingly.
  - **Checksums, not committed files, after a first pass committed the
    actual output and the size didn't sit right.** The first version of
    this PR committed the dump JSON and build output directly (the same
    shape `tests/payload/*.json` already uses for fixtures) — correct, but
    ~28MB, almost all of it one payload's pretty-printed dump
    (`NotoNastaliqUrdu-Regular.json` alone was 14MB). A hash is exactly as
    good at detecting "this changed" as the full bytes are: nothing here
    needs the *content* of a stored golden file, only whether a freshly
    produced one matches it, and equality-of-hash is exactly that
    question. Re-thought after being asked directly whether the committed
    files were actually necessary — they weren't; `tests/golden/` is 68KB
    now. Two follow-on effects of dropping the content: (1) the *build*
    step needs real input bytes, not a hash, so `compare-with-golden.sh`
    re-dumps the payload itself (already hash-checked a moment earlier)
    and builds from that, rather than reading a golden JSON file that no
    longer exists; (2) `generate-golden.sh`'s review step changed from
    `git diff --stat tests/golden/` (see *which files* moved) to
    `git diff tests/golden/checksums.sha256` (see *which labels'* hashes
    moved) — a hash diff can't show *what* changed the way a content diff
    could, so the header comment now points at `compare-with-c.sh` (still
    around, see below) for that when it's actually needed.
  - **One committed file survives on purpose**: `tests/golden/dll-test.otf`
    stays real bytes, not a hash, because the `otfccdll` comparison below
    needs an actual byte-level diff count to apply its tolerance against —
    "hash equal or not" can't express "close enough". At 62KB it isn't
    what made the size worth reconsidering.
  - **`compare-with-c.sh` itself is not deleted** — it still builds C and
    compares against the live Rust build exactly as before, kept as a
    manual, on-demand tool for the one time it is still needed: re-confirming
    a legitimately output-changing fix still matches C's behavior before
    running the new `rust/scripts/generate-golden.sh` to refresh
    `tests/golden/` and committing the result alongside the change that
    motivated it.
  - **One payload had to become a real fixture instead of a build-time
    generated one**: `gvar-test.ttf` (the variable-font payload) was
    previously produced fresh every CI run by
    `make-test-variable-font.py` via fontTools, which stamps
    `head.created`/`head.modified` with the current wall-clock time when
    they are not set explicitly. A freshly-regenerated copy would carry a
    different embedded timestamp than whatever the frozen golden dump JSON
    recorded, failing the comparison for a reason with nothing to do with
    correctness. Committed the already-generated, already-verified
    `build/gvar-test.ttf` as `tests/payload/gvar-test.ttf` instead — a
    static fixture like every other payload in that directory —
    and dropped the `pip install fonttools` / generation step from CI
    entirely. `run-cycles.sh` and `compare-with-c.sh` both updated to read
    it from the new fixed path, unconditionally (no more "skip if not
    generated" branch).
  - **The `otfccdll` comparison needed a different tolerance mechanism, not
    just a different reference file.** `compare-with-c.sh`'s version
    tolerated the cdylib API's lack of `--keep-modified-time` by diffing
    two *fresh, same-run* builds against each other and requiring the
    cross-implementation diff to be no larger than that self-diff — sound
    when both sides are built moments apart, but not against a golden
    fixture captured at an arbitrary earlier time: two fresh builds can
    land in the same wall-clock second and diff by 0 bytes, which would
    make *any* nonzero drift from the older golden file a spurious failure
    (hit exactly this while writing the script — a real 0-vs-5-byte
    failure on a byte-identical build). Replaced the dynamic baseline with
    a fixed, generous tolerance (32 bytes — comfortably more than the
    `created`/`modified` `LONGDATETIME`s and `checkSumAdjustment` combined
    could ever contribute, far too small for any real structural
    difference) instead.
  - **Verified the new script against the just-generated fixtures on both
    platforms** before wiring it into CI: `compare-with-golden.sh` (run
    twice on macOS for determinism, once on Linux) reports every payload
    byte-identical, alongside the standard `cargo test`, `check-abi.sh`,
    `run-cycles.sh` + round trips, the issue #1 golden test, and
    `test-lookup-alias.sh` — all still green.
  - **`c/` itself is untouched** — this decouples CI's dependency on it,
    it does not remove or archive the C source. Deleting `c/` outright
    remains a separate, later decision; `tests/golden/` existing is what
    would make that decision safe if it's ever made, not a claim that it
    has been made.
- **Stage 6-4: `ChainingRule.apply` (`*mut ChainLookupApplication` +
  `apply_count`) → `Vec<ChainLookupApplication>`.** The last remaining
  "leaf type owns a `Handle` but its container isn't `Vec`-backed yet" gap
  from the Stage 6-4 survey — closing it means every `Handle`-owning leaf
  in the crate now sits in a real container. `apply_count` is gone
  entirely; every read site uses `.apply.len()`.
  - **The union cascade turned out to be real, and one size bigger than the
    plan estimated.** `ChainLookupApplication` losing `Copy` (it now holds
    a `Vec`) forces `ChainingRule` off `Copy`, which — because
    `ChainingBody` is a union with `rule: ChainingRule` as one of its two
    variants — forces `rule` into `ManuallyDrop<ChainingRule>` (a union
    can't hold a non-`Copy` field any other way). That in turn forces
    `ChainingSubtable` (the struct wrapping `ChainingBody`) off `Copy`,
    which forces `Subtable.chaining` — the *outer* union's variant — into
    `ManuallyDrop<ChainingSubtable>` too: the eighth variant to get this
    treatment, joining the seven from the earlier `Subtable`
    `ManuallyDrop` PR. `ChainingRuleSet` (the `Poly`/`Classified` shape,
    `ChainingBody`'s other variant) is untouched and stays `Copy` — `.rules`
    remains an unconverted `*mut *mut ChainingRule`, deferred below.
  - **Neither `ChainingSubtable` nor `ChainingBody` derive `Clone` anymore**
    (previously they did, matching most `ManuallyDrop`-wrapped host
    structs) — deriving `Clone` on a union requires the union to
    implement `Copy` (`rustc`'s own diagnostic: "the trait bound
    `ChainingBody: Copy` is not satisfied"), which is exactly what just
    became impossible. Confirmed nothing calls `.clone()` on either type
    before dropping the derive — the vtable's `.copy` slot
    (`subtable_chaining_copy`) is a raw `memcpy`, not `Clone::clone`, and
    like every other `ManuallyDrop`-wrapped variant's `.copy` slot in this
    crate, is confirmed dead (never called outside its own static
    initializer).
  - **Plain field access through a `ManuallyDrop` union field needs help
    in two different ways, and only one of them was anticipated.** Reads
    that stop at the wrapped field itself (`&raw const (*ptr).chaining as
    *const ChainingSubtable`) were already the established idiom from the
    `Subtable` `ManuallyDrop` PR. What wasn't anticipated: `rustc` also
    denies *plain field writes* one level through the wrapper
    (`(*st).chaining.type_0 = …` — "not automatically applying `DerefMut`
    on `ManuallyDrop` union field... writing to this reference calls the
    destructor for the old value") and denies `dangerous_implicit_autorefs`
    on *plain field reads* that chain through the wrapper without an
    explicit pointer cast (`(*_subtable).chaining.type_0 == …` in
    `otfcc_build_chaining`/`otfcc_build_contextual` and
    `(*subtable).c2rust_unnamed.rule.match_count` in
    `otf_writer/stat.rs`, a tenth file this PR touched that no file-scoped
    `grep` for `chaining/` turned up — only `cargo build`'s own errors
    found it). Both were fixed the same way as the established
    `ManuallyDrop`-extraction idiom: hoist to an explicitly-cast local
    pointer once (`let subtable_chaining: *mut ChainingSubtable = &raw mut
    (*ptr).chaining as *mut ChainingSubtable;`) before projecting further,
    rather than chaining through the raw union-field access inline.
  - **`.apply[idx]` indexing needed the same `dangerous_implicit_autorefs`
    treatment already used for the `otl` pointer-list PR, at every read
    site** — `Index`/`IndexMut` dispatch through a raw pointer's
    dereferenced field autorefs a `&`/`&mut Vec`, which the lint denies
    without an explicit reference. ~20 sites across
    `chaining/{build,classifier,dump}.rs` and `consolidate/otl/chaining.rs`
    were mechanically rewritten from `(*rule).apply[i]` to
    `(&(*rule).apply)[i]` (or `(&mut (*rule).apply)[i]` at the one write
    site, forming `h: *mut LookupHandle` in `consolidate_chaining`) — no
    `suggested_replacement` came back in `cargo build
    --message-format=json` this time (unlike the `otl` pointer-list PR),
    so this pass was done by hand against the plain diagnostic text
    instead of machine-applied.
  - **`consolidate/otl/chaining.rs`'s manual compaction loop becomes
    `Vec::retain`, with the early-return guarded exactly as before**: the
    original only compacted (and potentially returned `true` for "drop
    this rule") when `apply_count != 0` to start; translated as `if
    !(*rule).apply.is_empty() { (*rule).apply.retain(|app|
    !app.lookup.name.is_null()); if (*rule).apply.is_empty() { return
    true; } }` rather than an unconditional `retain` + empty-check, which
    would have (incorrectly) started returning `true` for rules that never
    had any applies at all.
  - **`otf_reader/unconsolidate.rs`'s `unconsolidate_chaining` was the
    genuinely risky file** — the only one with real struct-move semantics,
    not just mechanical retyping. Both `Poly`- and `Canonical`-branch
    struct-copy assignments (`(*st).chaining.c2rust_unnamed.rule =
    **rule_slot;` and `(*st_0)....rule = (*sub)....rule;`) relied on
    `ChainingRule: Copy` to express "move the rule's ownership into the
    new canonical subtable." Rewritten as explicit
    `ptr::write(&raw mut (*st_chaining).c2rust_unnamed.rule,
    ManuallyDrop::new(ptr::read(...)))` pairs — `ptr::read` performs the
    same bitwise move without invoking `Vec`'s drop glue on the
    moved-from bytes, and the source allocation is freed (`Poly` branch)
    or intentionally left alone (`Canonical` branch) exactly as before.
    **The `Canonical` branch's pre-existing leak — `sub`, the whole outer
    `Subtable` block, is never `free()`'d on that path — was preserved
    byte-for-byte, not fixed**: fixing it would have been a behavioral
    change riding along with a mechanical conversion, exactly the kind of
    scope creep this migration's methodology avoids. `ptr::read`'s
    semantics make this preservation automatic — the moved-from bytes in
    `sub`'s allocation are left untouched and never dropped, so the leak
    persists in the same shape it always had.
  - **`ChainingRuleSet.rules` (the `Poly`/`Classified` shape) stays
    deferred, but its elements needed fixing anyway** — a subtlety the
    plan's file-scoping got right for the wrong reason. `.rules: *mut *mut
    ChainingRule` is an unconverted raw array of pointers to the *same*
    `ChainingRule` type, so every `.apply`/`.apply_count` read reached
    through it (in `otfcc_build_chaining_classes`,
    `otfcc_build_contextual_classes`, `otfcc_chaining_lookup_is_contextual_lookup`)
    needed the identical `.len()`/indexing conversion as the `.rule`
    single-instance path, even though `ChainingRuleSet` itself never
    changes shape. The type change ripples through every consumer of
    `ChainingRule`, regardless of which union variant reaches it.
  - **Real coverage, no synthetic payload needed**: `NotoNastaliqUrdu-Regular.ttf`
    (41 GSUB + 1 GPOS chaining lookups), `iosevka-r.ttf` (2, also the
    issue #1 golden payload), and `Molengo-Regular.ttf` (1) all exercise
    the `.apply`/`Canonical` shape through read/parse/build/dump — checked
    with `otfccdump` against every payload before scoping the PR, and
    confirmed none of them use the `Poly`/`Classified` shape (`.rules`),
    which is why deferring it is safe today.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    including the chaining-heavy ones above, all 10 payloads' round trips,
    and the issue #1 golden test. `rust/target` had to be wiped alongside
    the usual `build/ninja build/obj bin/release-x64 bin/x64` when
    switching platforms this time — it had accumulated macOS arm64
    `cargo build --release` binaries from earlier in the session, which
    the Linux container then tried (and failed) to execute directly
    ("Exec format error"), a new instance of the same cross-platform
    build-artifact contamination class as the existing `build/`/`bin/`
    reset rule.
- **Stage 6-4 follow-up: `ChainingRuleSet.rules` (`*mut *mut ChainingRule` +
  `rules_count`) → `Vec<*mut ChainingRule>`.** The deferred half of the
  `ChainingRule.apply` PR above — container only, matching the `otl`
  pointer-list precedent (`LookupList`/`FeatureList`): pointees stay
  individually heap-allocated raw pointers, `Box`-ification stays Stage
  6-4's job for a later PR. `rules_count` is gone; every read site now uses
  `.rules.len()`. 5 files: `table/otl.rs`,
  `table/otl/subtables/chaining/{common,classifier,read,build}.rs`,
  `otf_reader/unconsolidate.rs`.
  - **The `ChainingBody` union needed `ManuallyDrop` on *both* variants,
    not just `.rule`** — `rules: Vec<*mut ChainingRule>` losing `Copy`
    means the union's other field (`c2rust_unnamed: ChainingRuleSet`) now
    also needs wrapping, regardless of which variant is "logically active"
    at a given moment; a union simply cannot hold a non-`Copy` field any
    other way. This roughly doubled the extraction-site cast/hoist work
    predicted by the earlier PR's "~30 more call sites" estimate turned
    out accurate (51 `.c2rust_unnamed.c2rust_unnamed.*` sites across 5
    files) — every one now goes through a single hoisted
    `let ruleset: *mut ChainingRuleSet = &raw mut
    (*subtable).c2rust_unnamed.c2rust_unnamed as *mut ChainingRuleSet;`
    per function, then plain `(*ruleset).rules`/`.bc`/`.ic`/`.fc` from
    there — the same idiom as the `.rule` extractions, just for the
    sibling variant.
  - **A landmine specific to this field, not seen with `.apply`:
    `type_0` can be set to `Poly` before `.rules` is ever populated.**
    `otl_read_contextual`/`otl_read_chaining` set `type_0 = Poly`
    immediately after `create()` (freshly `memset`-zeroed memory), *then*
    dispatch to a format handler that populates `.rules` — but the
    unrecognised-format and too-short-table error paths free the subtable
    in between, with `type_0 == Poly` but `.rules` still raw zeroed bytes.
    Under the old raw-pointer field this was safe (`rules.is_null()` on a
    zeroed pointer correctly reads as "nothing to walk"); under `Vec` it
    would be immediate UB (a zeroed byte pattern is never a valid `Vec` —
    calling anything on it, even a length check, reads invalid state).
    Fixed by moving the placement-construct up to happen in the same
    breath as the `type_0 = Poly` assignment (`ptr::write(&raw mut
    (*ruleset).rules, Vec::new())`, immediately after, before any format
    dispatch) rather than leaving it to whichever format handler runs —
    every path from that point on, including the disposal-only error
    paths, now sees a valid (possibly still-empty) `Vec`. `classifier.rs`'s
    `try_classify_around` sets `type_0 = Classified` in the opposite order
    (after `.rules` is already fully built), so it needed no such fix — a
    fresh `calloc`'d `ChainingSubtable`'s `type_0` field reads as
    `Canonical` (0) for the whole span before the explicit assignment at
    the end, and nothing external observes that transient local value in
    between. This asymmetry — placement-construct-with-the-flag vs.
    populate-then-set-the-flag — is now the deciding question to ask
    before touching any future "tagged union with a late discriminant
    write" conversion in this crate.
  - **`otl_dispose_chaining` got simpler, not more complex, once the
    above held**: no more `.rules.is_null()` guard at all — by the time
    `type_0 != Canonical` is observable there, `.rules` is *always* a
    valid `Vec` (empty or not), so disposal is just "iterate, dispose
    each pointee, `= Vec::new()`" unconditionally.
  - **`unconsolidate_chaining`'s `Poly`-branch consumption loop** (the
    inverse of construction: splitting a `ChainingRuleSet` back out into
    individual `Canonical` subtables) uses `mem::take(&mut
    (*ruleset).rules)` to get an owned `Vec<*mut ChainingRule>` to iterate
    by value — this both hands over the pointer list and leaves a valid
    empty `Vec` behind in the same expression, so the raw `free(sub as
    *mut c_void)` right after (which does not run any Rust drop glue) has
    nothing left to leak. Simpler than the manual "read a slot, null it,
    free the whole array afterward" dance the old raw-pointer version
    needed.
  - **`compatible_count > 1` requires *three* mutually-compatible adjacent
    subtables, not two** — `try_classify_around` only increments
    `compatible_count` for subtables *after* the first one being tried, so
    grouping needs the first plus two more to pass `class_compatible`.
    Relevant because it changes what a synthetic test payload would need
    to look like, and turned out to matter for a different reason below.
  - **The "no current payload exercises the classifier" premise from the
    original scoping PR was wrong, and it was checked before writing any
    conversion code this time.** Built a synthetic 3-subtable
    classifiable-group payload (`make-test-classified-chaining.py`,
    modeled on `make-test-vdmx.py`) to fill the assumed gap, then
    instrumented `try_classify_around`'s success branch with a temporary
    `eprintln!` before wiring the script in, specifically to confirm the
    synthetic payload was doing real work rather than silently no-op'ing.
    It fired — but so did the *unmodified* `tests/payload/iosevka-r.ttf`,
    three times per build (9/5/5 rules grouped), with no synthetic
    payload involved at all: `iosevka-r`'s own `lookup_ccmp_1`/`lookup_calt_0`
    chaining lookups already contain naturally-compatible adjacent
    subtable runs, and this path was already being byte-compared on every
    `compare-with-c.sh` run throughout this crate's history. Deleted the
    synthetic payload script and the debug instrumentation rather than
    ship a redundant test — real, already-wired coverage existed the
    whole time, the earlier PR's assumption was simply never rechecked
    against the actual data. Lesson for the next "is this exercised"
    question: check with instrumentation before writing a synthetic
    fixture, not after — a byte-identical `compare-with-c.sh` pass alone
    cannot distinguish "this path ran and matched" from "this path never
    ran," but a one-line `eprintln!` in the success branch can, cheaply,
    and should be the first move.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including `iosevka-r.ttf`, now confirmed to genuinely exercise the
    `Classified` build path), all 10 payloads' round trips, and the issue
    #1 golden test. `rust/target` wiped alongside `build/`/`bin/` on every
    platform switch this time, per the lesson recorded above.
  - **`ChainingBody`/`ChainingRuleSet`/`ChainingRule` are now fully
    `Vec`-backed on every field that can own memory.** The
    `ChainingRule.apply`/`ChainingRuleSet.rules` pair was the last "leaf
    type owns a `Handle` but its container isn't `Vec`-backed yet" gap
    from the Stage 6-4 survey — that theme is now complete.
- **`ChainingRuleSet.rules` → `Vec<Option<Box<ChainingRule>>>`, joining the
  `LangSystemList`/`FeatureList`/`LookupList`/`GlyfTable` "owned pointer
  array" group** (that group's README entry called itself closed after
  `GlyfTable`, before this type existed as a candidate — this reopens it
  for one more member, the only remaining `Vec<*mut T>` container left
  anywhere in `table/`). `Option`, not plain `Box`, because
  `general_read_contextual_rule`/`general_read_chaining_rule` can fail on
  truncated/malformed font data and the pre-`Box` code pushed a null
  element in that case with no downstream null check — asked the user
  which shape to use given that wrinkle, since it's a real design choice,
  not a mechanical translation; `Vec<Option<Box<T>>>` (matching the
  `GlyfTable` precedent) was the answer. Every consumption site treats a
  `None` as unreachable in practice (`.expect(...)`, which panics instead
  of the old null-pointer-deref UB on the one hand if that latent path is
  ever actually hit — no current payload does).
  - **`ChainingRule` gained a real `Drop` impl** (frees `.match_0`'s array
    and each `Coverage` it points to; `.apply`'s `Vec` already disposes
    itself) — this is what makes `Box<ChainingRule>` safe to drop
    automatically instead of needing the old manual `close_rule`/
    `delete_rule` dispose pair. `close_rule` (still needed for the
    `Canonical` variant, `ChainingBody.rule: ManuallyDrop<ChainingRule>`,
    which is never `Box`-owned) shrank to a one-line
    `ptr::drop_in_place(rule)` that runs the same `Drop` impl through a
    raw pointer instead. `delete_rule` — used only to tear down a
    `Vec<*mut ChainingRule>` element — disappeared entirely, in both its
    copies (`common.rs`'s and the c2rust-duplicated one in `read.rs`):
    `otl_dispose_chaining`'s `Poly`/`Classified` branch is now just
    `(*ruleset).rules = Vec::new();`, no per-element loop, since dropping
    the `Vec` now drops every `Some(Box<ChainingRule>)` correctly on its
    own. Dropped `#[derive(Clone)]` from `ChainingRule` in the same
    change — a derived `Clone` would shallow-copy `match_0`, aliasing two
    rules onto one heap array that `Drop` then frees twice; nothing in
    the crate ever called `.clone()` on it, so this closes a latent
    footgun rather than breaking anything live.
  - **The two `read.rs` binary-format constructors
    (`general_read_contextual_rule`/`general_read_chaining_rule`) needed
    real restructuring, not just a signature change** — matching the
    established `Box::new` + struct-literal pattern (`new_lookup`,
    `otfcc_new_glyf_glyph`) rather than the shortcut of wrapping an
    existing `__caryll_allocate_clean` (libc `calloc`) pointer with
    `Box::from_raw`. Checked first whether that shortcut had any
    precedent in the crate: every existing `Box::from_raw` call site
    (`otfcc_delete_lookup`, three sites in `table/otl/parse.rs`)
    reconstitutes a pointer that started life as `Box::new` and was
    handed out via `Box::into_raw` for C-shaped passing — never a
    genuinely `malloc`'d pointer. Wrapping a `calloc`'d pointer directly
    would "work" today (the `System` allocator is backed by the platform
    `malloc`/`free` on both targets this crate verifies), but it isn't
    the pattern anywhere else in the crate and would rely on that
    allocator-equivalence holding rather than being guaranteed by the
    language — not worth the shortcut. Once each function starts from a
    `Box::new(ChainingRule { ...zeroed... })`, almost the entire body is
    untouched: `(*rule).field = ...` reads exactly the same through
    `Box`'s `Deref`/`DerefMut` as it did through the raw pointer, so only
    the declaration and the two return points (`Some(rule)` on success,
    `None` on every early-exit path) needed to change. The failure paths
    got strictly simpler, not more complex: the old code called
    `delete_rule(rule)` explicitly before returning null; the new code
    just returns `None`, and `rule`'s `Drop` (now real, via `ChainingRule`)
    runs automatically as the local variable goes out of scope, correctly
    tearing down whatever partial state (`.match_0` array, `.apply`
    entries) construction had reached by that point.
  - **`classifier.rs`'s `build_rule` (the reverse direction — building a
    `Classified` subtable's rules from already-consolidated JSON-sourced
    data during BUILD, not parsing binary) got the same `Box::new`
    treatment, but returns plain `Box<ChainingRule>`, not `Option`** —
    unlike the `read.rs` constructors, this never fails; it transforms
    already-valid in-memory structures, with no truncated-data escape
    hatch to represent. Its one `Vec::with_capacity` reassignment for
    `.apply` also lost its `ptr::write` placement-construct: since
    `Box::new`'s struct literal already gives `.apply` a valid (if empty)
    `Vec`, a plain `=` correctly drops that empty `Vec` first before
    replacing it — no landmine, unlike the calloc'd-memory sites
    elsewhere in this migration where a plain assignment would try to
    drop garbage bytes.
  - **`unconsolidate_chaining`'s `Poly`-branch move got simpler, not more
    complex, converting from raw pointers to `Box`.** The previous PR's
    `ptr::read(rule_ptr)` + `ManuallyDrop::new(...)` dance — needed
    because a raw `*mut ChainingRule` has no compiler-tracked ownership to
    move out of — is replaced by `*boxed_rule`: moving a value out of a
    `Box<T>` is one of the few operations the compiler special-cases to
    allow directly (unlike moving out of an arbitrary raw pointer or
    reference), so `ManuallyDrop::new(*boxed_rule)` reads as what it is —
    take the rule out of its box, hand it to the union's `ManuallyDrop`
    slot — with no unsafe `ptr::read` needed at all. The explicit
    `free(rule_ptr as *mut c_void)` call is also gone: moving out of the
    `Box` deallocates its own heap slot automatically, through the same
    allocator that `general_read_contextual_rule`/`general_read_chaining_rule`
    used to create it (`Box::new`, i.e. Rust's global allocator, not
    `libc::free` — consistent throughout, since nothing here mixes
    allocators the way a `Box::from_raw`-on-`calloc` shortcut would have).
  - **Every read-side consumption of `.rules[idx]` in `build.rs`
    (5 sites across `otfcc_chaining_lookup_is_contextual_lookup`,
    `otfcc_build_chaining_classes`, `otfcc_build_contextual_classes`)
    became `.as_deref().expect(...)  as *const ChainingRule as *mut
    ChainingRule`** — reads a `&ChainingRule` out of the `Option<Box<_>>`
    slot without taking ownership (these functions only ever read),
    then casts to match the raw-pointer-based signatures
    (`reverse_backtracks`, etc.) every other function in this file
    already uses for `ChainingRule` regardless of how it's owned. `None`
    is provably unreachable at these specific call sites even more
    strongly than at the `read.rs`/`unconsolidate.rs` sites above: every
    `.rules` a build-direction function ever sees was *just* constructed
    fresh by `classifier.rs`'s `build_rule` (`Box`, never `Option`,
    wrapped in `Some` at the one push site) moments earlier in the same
    call — parsing failure, the only source of `None`, cannot occur on
    that path at all.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test. Also ran
    three repeated dump→build cycles of `NotoNastaliqUrdu-Regular.ttf`
    (the heaviest chaining payload) on macOS and diffed each output
    byte-for-byte against the next, given this PR changes allocator and
    `Drop` behavior more than most container-only `Vec` conversions in
    this migration — all three identical, no leak/use-after-free-shaped
    corruption. No memory checker (valgrind/ASan) available in either
    verification environment; determinism-under-repetition plus the
    standard byte-comparison suite is this crate's established bar
    absent one, consistent with every earlier PR in this migration.
- **Stage 6-2 phase 2 (re-scoped): `TsiEntry.content` → `Vec<u8>`, the
  first field-level `sds` unwind since the phase-2 investigation stalled
  in the earlier session.** That investigation's blocker (`Handle`'s
  `Copy`-ness cascading crate-wide) was resolved by the `Handle` pilot
  (PR #68) and confirmed clear again before starting: `TsiEntry` already
  derives only `Clone`, not `Copy`, so this field's type change has zero
  ripple onto sibling leaf types. Picked as the pilot for continuing the
  ~1,084-call-site `sds*()` sweep (of which 586 are `support/aglfn.rs`'s
  one-shape-repeated static-table `sdsnew` calls, not representative of
  the interesting remainder) specifically because its blast radius is
  fully contained to two files (`table/_tsi.rs`, `consolidate.rs`) — no
  other file references `TsiEntry.content` directly.
  - **`TsiEntry` needs no `Drop` impl of its own** — both fields
    (`glyph: GlyphHandle`, already real `Drop`; `content: Vec<u8>`, now
    real `Drop`) tear themselves down correctly on their own, so a plain
    `#[derive(Clone)]` struct with no manual destructor is enough. This
    made `dispose_tsi_entry`/`table_tsi_dispose`'s per-element loop fully
    redundant, same as `ChainingRule` the PR before this one:
    `table_tsi_free` shrank to `drop_in_place` + `free`, no manual walk.
  - **`consolidate_tsi`'s `gid_entries` scratch array converts alongside
    `.content`, not separately** — it's a `*mut SdsRaw` array (one slot
    per glyph) that exists purely to redistribute `.content` values by
    GID, so its element type is coupled to `.content`'s by construction.
    Went with `Vec<Option<Vec<u8>>>` (replacing the `__caryll_allocate_clean`
    calloc + manual `free`) specifically to preserve a real semantic
    distinction the raw-pointer version encoded: `is_null()` meant "no
    entry yet for this GID", which is not the same state as "entry
    exists with zero-length content" (`sdsempty()`'s fallback case) —
    `None` vs. `Some(Vec::new())` reproduces that distinction exactly,
    where a bare `Vec<Vec<u8>>` could not have. The explicit
    free-before-overwrite (`if !is_null() { sdsfree(...) }`) disappears
    without replacement: a plain `gid_entries[idx] = Some(...)` in Rust
    always drops whatever was there first, automatically.
  - **`mem::take`/`.take()` replace two of this migration's now-familiar
    "move out, leave the source neutralized" idioms** — `mem::take(&mut
    (*entry).content)` (moves `.content` out of the source `TsiEntry`
    into the scratch array slot, leaving a valid empty `Vec` behind
    exactly where the old code left a null pointer) and
    `gid_entries[j].take().unwrap_or_default()` (moves the scratch
    value back out into the freshly-consolidated entry, `None` → empty
    `Vec` matching the old `sdsempty()` fallback). Neither needed
    `unsafe`, unlike the raw-pointer-based moves earlier in this
    migration (`ptr::read`/`ManuallyDrop`) — ordinary safe-Rust
    ownership transfer was enough once both sides were real Rust values.
  - **Byte-copying helpers substitute directly for their `sds` counterparts
    with no semantic gap**: `sdsnewlen(ptr, len)` (parse/read directions)
    → `slice::from_raw_parts(ptr, len).to_vec()`; `sdslen`/cast-to-`sds`
    (dump direction, for `json_string_new_length`'s `(len, ptr)` pair) →
    `.len()`/`.as_ptr()`; `bufwrite_sds` (build direction) →
    `bufnwrite8`, an existing `&[u8]`-based buffer-write helper from the
    `c_variadic` removal PR (#29) that this conversion is the first to
    reuse for a genuinely `Vec<u8>`-backed field rather than a temporary
    slice.
  - **Real coverage, no synthetic payload needed**: `vtt.ttf` carries
    real `TSI_01`/`TSI_23` data (confirmed by the `NameTable`/`TsiTable`
    Vec-ification PR, #60) and already drives read/parse/build/dump plus
    `consolidate_tsi`'s GID-redistribution path (including the
    `None`/`Some` distinction above, since `vtt.ttf` has both `Glyph`-type
    entries needing redistribution and `Fpgm`/`Prep`/`Cvt` entries that
    don't) through the standard `compare-with-c.sh` run.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    including `vtt.ttf`, all 10 payloads' round trips, and the issue #1
    golden test.
  - **Remaining `sds*()` call sites after this PR**: still roughly 1,050
    (crate-wide sweep continues field by field / file by file — no
    single next target chosen yet). `support/aglfn.rs`'s 586 mechanical
    `sdsnew` calls stay as `sds` until `GlyphOrderEntry`'s own name field
    is converted (a much larger, `GlyphOrder`-wide change, not attempted
    here); `Handle.name` itself (the single highest-leverage remaining
    field, touching nearly every other leaf type in the crate) is a
    separate, larger future PR given its crate-wide fan-out.
- **Stage 6-2 phase 2, second field: `FpgmPrepTable.tag` → `Vec<u8>`.**
  Picked as the next `sds` sweep target for the same reason as
  `TsiEntry.content` — small, provably self-contained blast radius (one
  file, `table/fpgm_prep.rs`, 250 lines) — but this one turned out to be
  a different shape entirely: `.tag` is **write-only**. Grepped every
  read of `(*table).tag`/`.tag` in this file and its two external
  callers (`font/caryll_font.rs`, only via `.free`) before converting —
  found none. `table_dump_table_fpgm_prep`/`otfcc_build_fpgm_prep` both
  already have the same string available as their own `tag: *const
  c_char` parameter and never reach for the field. Converted the type
  anyway rather than deleting the field outright — removing a field is
  a scope-creeping cleanup riding along with a type-only PR, and this
  crate's methodology has consistently kept those separate (confirm a
  field or slot is dead, note it, but don't fold its removal into an
  unrelated conversion unless the two are inseparable, as `ChainingRule`'s
  `Drop` impl was to `Vec<Box<_>>` two PRs ago).
  - **`FpgmPrepTable` loses `Copy`/`Clone` outright, not just `Copy`** —
    unlike every leaf type converted so far in this sweep, nothing calls
    `.clone()` on this one either (it's a single-instance table behind
    `*mut FpgmPrepTable`, `font.fpgm`/`font.prep`, never stored in an
    array), so there was no reason to keep a `Clone` impl it would never
    exercise.
  - **`.copy` (`table_fpgm_prep_copy`, a raw `memcpy`) was already dead**
    before this PR and would have become unsound after it (a `Vec` field
    can't survive a byte-for-byte struct copy without aliasing) — the
    same "grep every call site, walk up one level if the immediate
    caller is itself unreached" check from every earlier vtable-slot
    audit in this migration, confirmed via `font/caryll_font.rs` only
    ever calling `.free` on this table's vtable. Deleted the function and
    set the vtable slot to `None`, matching the established pattern for
    confirmed-dead slots elsewhere (`OTL_I_CARET_VALUE`'s `init`/`copy`/
    `dispose`, this migration's `GdefTable` PR) rather than leaving a
    function that would silently corrupt memory if some future change
    ever wired it back up.
  - **Both construction sites (`otfcc_read_fpgm_prep`, `otfcc_parse_fpgm_prep`)
    needed `ptr::write`, not plain assignment** — `table_fpgm_prep_create`
    is still `malloc` + `memset`-zero (`table_fpgm_prep_init`), so `.tag`
    starts as invalid `Vec` bytes immediately after `create()`, same
    landmine class as every other "first write onto freshly calloc'd
    memory" site in this migration. The parse-direction site additionally
    switched from `sdsnew(tag)` to the crate's established
    `CStr::from_ptr(tag).to_bytes().to_vec()` idiom (already used in
    `table/otl/parse.rs`/`table/cff.rs`) for turning a borrowed C string
    parameter into an owned byte vector.
  - **Real coverage, no synthetic payload needed**: `Molengo-Regular.ttf`,
    `iosevka-r.ttf`, and `vtt.ttf` all carry real `fpgm`/`prep` tables
    (confirmed via `otfccdump` against every payload before starting),
    driving both the read and parse construction paths — and therefore
    both `ptr::write` sites — through the standard `compare-with-c.sh`
    run.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test.
- **Stage 6-2 phase 2, third field: `MetaEntry.data` → `Vec<u8>`.** Same
  selection criterion as the two before it — self-contained blast radius,
  this time the whole `table/meta/{types,dump,build,parse,read}.rs`
  module (5 files, no external references to `MetaEntry` beyond a
  documentation comment in `table/name.rs`).
  - **`.data: Vec<u8>`, not `String`** — `meta` table entries can be either
    UTF-8 tag strings or arbitrary base64-decoded binary (the `dlng`/`slng`
    string tags vs. e.g. a raw `appl` blob), so `Vec<u8>` is the only
    honest representation, matching `TsiEntry.content`'s reasoning.
  - **`table_meta_copy` (the `.copy` vtable slot, a field-by-field
    `.entries.clone()`) was the only place in the entire module that
    called `.clone()` on anything meta-related** — confirmed dead first
    (only `.create`/`.free` are ever called from outside this module,
    via `table/meta/{parse,read}.rs`'s own `TABLE_I_META.create` and
    `font/caryll_font.rs`'s `.free`), then deleted along with the
    `#[derive(Clone)]` on both `MetaEntry` and `MetaTable` that only
    existed to support it — the same "confirm dead, delete rather than
    make unsound" pattern as `FpgmPrepTable.copy` the PR before this one.
  - **`dispose_meta_table`'s per-element `dispose_meta_entry` loop
    disappeared** once `.data: Vec<u8>` had real drop glue — `.entries =
    Vec::new();` alone now tears everything down correctly, same as
    `TsiTable`'s and `ChainingRule`'s containers.
  - **`parse_meta_data` returns `Option<Vec<u8>>`, not a plain `Vec<u8>`**
    — unlike `TsiEntry.content`'s parse path, this one can genuinely
    return "no data" (neither a `string` key, a `base64` key, nor a bare
    JSON string matched), and the caller already checked for that with
    an `is_null()` guard before pushing. `if let Some(data) = ...`
    replaces that guard directly. Needed a function-level
    `#[allow(improper_ctypes_definitions)]` — `Option<Vec<u8>>` isn't
    FFI-safe and this function's `extern "C"` is a c2rust artifact from
    the original signature, never a real ABI boundary (only called from
    `otfcc_parse_meta` in the same file) — same reasoning already used
    elsewhere in this migration for non-FFI-safe return types on
    internal-only `extern "C"` functions.
  - **Real coverage already existed, no new synthetic payload needed**:
    `make-test-meta.py` (added when `MetaEntry` was first `Vec`-ified,
    several PRs before this migration reached `sds`) already injects both
    a string-tag and a base64-tag entry and is wired into
    `compare-with-c.sh` as `meta-test.ttf`/`meta-test dump` — both
    directions (the `is_string_tag` dump branch and the `base64_encode`/
    `base64_decode` branch) confirmed byte-identical on both platforms.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    including `meta-test.ttf`, all 10 payloads' round trips, and the
    issue #1 golden test.
- **`Handle.name: SdsRaw` → `Vec<u8>`.** The last leaf-type `sds` field in
  the crate, and much larger in touch-point volume than the three fields
  before it — `Handle` is embedded (directly or via `GlyphHandle`/
  `LookupHandle`) throughout `table/**`, `consolidate/**`, and
  `otf_reader/otf_writer`, so the fallout from changing its `name` field's
  storage type reached ~30 files. Scoped up front with a foreground
  Explore-agent inventory (100 call sites / 30 files / 4 categories:
  raw-pointer passthrough, construction, comparison, other) before writing
  any code, rather than discovering scope reactively through compiler
  errors alone — the compiler was still used as the final authoritative
  check, per this crate's established methodology, but starting from a
  categorized map made the ~30-file sweep tractable instead of a blind
  whack-a-mole.
  - **New `sds_to_vec`/`handle_name_eq_cstr` helpers in `support/handle.rs`**.
    `handle_from_name`/`handle_from_consolidated`/`handle_consolidate_to`
    keep their existing `SdsRaw`-in public signatures (their ~40 call
    sites across the crate still pass an owned or borrowed `sds`), so only
    their internals changed: `sds_to_vec(s: SdsRaw) -> Vec<u8>` is the one
    place that reads `sdslen`+`slice::from_raw_parts` to copy the bytes
    out. `handle_name_eq_cstr(name: &[u8], other: *const c_char) -> bool`
    replicates `strcmp(a.name, b.name) == 0` for the one comparison call
    site (`chaining.rs`'s lookup-name match) that used to rely on `strcmp`
    directly against the old `SdsRaw` field.
  - **`impl SdsPart for &Vec<u8>` added to `vendor/sds.rs`**, matching the
    *`*mut c_char`/`*const c_char`* impl's strlen/NUL-truncating semantics
    — not the `&[u8]` impl's full-length semantics — specifically because
    every existing `sdsbuild!` call site passing a `Handle.name` relied on
    the old `SdsRaw`-typed field's NUL-truncating behavior. Using `&[u8]`
    semantics instead would silently change output for any name containing
    an embedded NUL (rare, but this crate has stayed deliberately paranoid
    about byte-exact preservation for exactly this class of edge case).
    Every `sdsbuild!` site touching a `Handle.name` needed an explicit `&`
    added — the macro's `SdsPart` dispatch is UFCS-style (`Trait::method(x,
    ...)`), not `x.method()`, so there is no automatic deref coercion from
    `Vec<u8>` to `&Vec<u8>` to paper over a missing `&`.
  - **New `json_string_new_from_bytes`/`json_object_push_bytes_key` helpers
    in `vendor/json_builder.rs`**, for the ~15 call sites across
    `table/**` that used to hand a `Handle.name`-typed `SdsRaw` straight
    into `json_string_new`/`json_object_push` (both strlen-based). Same
    NUL-truncation rationale as the `SdsPart` impl above, just for the
    JSON-value and object-key positions instead of the `sdsbuild!` macro.
  - **Simplification pattern, reused from the `ChainingRule.apply` PR**:
    every consolidate-phase hash-map value struct that used to carry a
    `sdsdup`'d `SdsRaw` name (later fed into `handle_from_consolidated` +
    `sdsfree`) now just carries the `Vec<u8>` (via `.clone()`) and
    constructs the final `Handle { state: HandleState::Consolidated,
    index, name }` struct literal directly — the `handle_from_consolidated`
    round trip through a byte copy it would only immediately re-copy is
    gone. Applied throughout `consolidate.rs` and all of
    `consolidate/otl/{mark,gdef,chaining,gpos_cursive,gpos_single,
    gsub_ligature,gsub_multi,gsub_reverse,gsub_single}.rs`, plus
    `table/otl/subtables/chaining/classifier.rs`'s `ClassifierValue.gname`
    (converted from `SdsRaw` to `Vec<u8>` for the same reason — it's a
    scratch value sourced from a `Handle.name` and fed right back into one).
  - **Two structurally-dead-looking sites that were not**: `otf_writer/
    stat.rs` builds two scratch `ComponentReference { glyph: Handle {
    name: ... } }` literals that are immediately overwritten field-by-field
    a few lines later — `name: Vec::new()` is correct there for the same
    reason the crate's earlier `null_mut()` placeholders were, just spelled
    differently for the new field type. `table/glyf.rs`'s
    `glyf_parse_reference` has a genuine plain-field *write* (`ref_0.glyph.
    name = Vec::new();`, not a struct literal) onto a `ComponentReference`
    that was already validly constructed via `GLYF_I_COMPONENT_REFERENCE
    .empty()` — confirmed that constructor already places a valid empty
    `Vec` there before trusting the plain assignment (dropping an already-
    empty `Vec` and replacing it with another is a safe no-op, unlike the
    "malloc'd struct, first-ever write" landmine class this migration has
    hit repeatedly elsewhere).
  - **`support/glyph_order.rs`'s `otfcc_gord_consolidate_handle`** was the
    single highest-value simplification the earlier scoping pass had
    flagged: it used to hand-roll exactly what `sds_to_vec` now does
    (`slice::from_raw_parts((*h).name as *const u8, sdslen((*h).name))
    .to_vec()`) twice, to build a `Vec<u8>` lookup key from a `Handle.name`
    that was *already* a `Vec<u8>` once the type changed — both sites
    collapsed to `(*h).name.clone()`. `GlyphOrderEntry.name` itself is a
    separate, still-`SdsRaw` field (a different struct, not `Handle`) and
    was intentionally left alone.
  - **Two `push_class_def`/`push_to_coverage` functions needed
    function-level `#[allow(improper_ctypes_definitions)]`**, and
    `support/handle.rs` needed a file-level one — passing a `Handle`/
    `GlyphHandle` by value through an `extern "C" fn` signature is no
    longer FFI-safe now that it owns a `Vec`. None of the affected
    functions are `#[no_mangle]` (this crate's only real FFI surface is
    the 4 symbols in `ffi/dll.rs`); every `extern "C"` here is c2rust's
    calling-convention residue from the original C signature, not a real
    ABI boundary — same reasoning already used for `CaretValueRecord`/
    `GsubLigatureSubtable` earlier in this migration.
  - **No new synthetic payloads needed** — `Handle.name` is exercised by
    essentially every existing payload (it's the glyph/lookup name field
    itself), so `compare-with-c.sh`'s existing corpus already drives every
    construction/comparison/dump path touched by this PR, including the
    `meta-test`/`vdmx-test`/`unknown-lookup` synthetic payloads added by
    earlier PRs.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test.
  - With this PR, every `Handle`/`GlyphHandle`/`LookupHandle`-typed field
    the crate ever hung an `SdsRaw` name off has moved to `Vec<u8>`. The
    remaining ~1,050 other `sds*()` call sites (dominated by
    `support/aglfn.rs`'s 586 mechanical `sdsnew` calls, which stay `sds`
    until `GlyphOrderEntry`'s own `name` field is converted) remain a much
    larger, not-yet-scoped future theme — no commitment made to pursue
    further after this PR.
- **`FvarMaster.name: SdsRaw` → `Vec<u8>`.** A small, single-file follow-up
  in the same `sds` sweep — `FvarMaster` (a variation-region "master" name
  like `"m1"`, `"m2"`, ..., stored in `FvarTable.masters: IndexMap<RegionKey,
  FvarMaster>`) is entirely private to `table/fvar.rs`, with no `Copy`/
  `Clone` derive to lose (unlike every `Handle`-adjacent leaf type converted
  so far, there was no `Copy` cascade to worry about here).
  - **The name is crate-generated, not font-derived** — `fvar_register_region`
    synthesizes it from an internal counter (`"m" + (masters.len() + 1)`)
    rather than copying anything read from the font file or JSON input, so
    unlike the `sdscatprintf`/`sdscatfmt` replacement sites elsewhere in
    this migration (which stay byte-oriented specifically because they
    carry font-derived, possibly-non-UTF-8 glyph names), this one site can
    safely use `format!("m{}", ...).into_bytes()` — matching the existing
    precedent in `support/ttinstr.rs`'s synthesized `PUSHB_N`/`PUSHW_N`
    labels and `vendor/sds.rs`'s own `SdsPart` integer-formatter impls.
  - **`dispose_fvar_master` loses its `sdsfree(m.name)` call** but keeps
    the function (and its `vq_delete_region(m.region)` call) — same
    pattern as every earlier leaf-type conversion: the `Vec<u8>` is torn
    down for free when the owning `FvarMaster` is moved out of the
    `IndexMap` and dropped at the end of `dispose_fvar`'s loop iteration,
    but `region: *mut VqRegion` is still a raw pointer needing an explicit
    call.
  - **Real coverage, no synthetic payload needed**: `gvar-test.ttf` (the
    existing variable-font payload) already drives both the master-naming
    path (`fvar_register_region`, on every distinct tuple-variation region)
    and the two dump-direction reads (`otfcc_dump_fvar`'s masters object,
    `json_new_vq_region`'s named-region shortcut).
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test.
- **`Glyph.name: SdsRaw` → `Vec<u8>`.** The glyph's own name field
  (distinct from the `Handle`-based glyph/lookup *references* the earlier
  `Handle.name` PR converted). Larger than `FvarMaster` — four files
  (`table/glyf.rs`, `consolidate.rs`, `otf_reader/unconsolidate.rs`,
  `table/cff.rs`) rather than one — because `Glyph.name` is populated from
  three independent directions (JSON/glyph-order parsing, the
  consolidate-phase glyph-order registration/uniqueness pass, and CFF
  charset-derived naming) and read back out by two (JSON dump, CFF
  charset-building).
  - **`Glyph` already had a real `Drop` impl** (from the earlier "glyf
    残り4型" `Box<Glyph>` PR), so this conversion is a pure simplification
    there: the `if !self.name.is_null() { sdsfree(...) }` block disappears
    entirely, leaving only `instructions`' manual `free()`.
  - **`support/handle.rs`'s `sds_to_vec` helper (added for the `Handle.name`
    PR) does essentially all of the heavy lifting here** — every site that
    used to `sdsdup` a shared/borrowed `SdsRaw` into `Glyph.name` (the
    consolidate-phase glyph-order registration in both `consolidate.rs` and
    `otf_reader/unconsolidate.rs`, and CFF charset-derived naming in
    `table/cff.rs`) now calls `sds_to_vec` instead — same "copy the bytes,
    don't touch the source's lifetime" contract, and the old paired
    `if !name.is_null() { sdsfree(name); }` teardown before each
    reassignment is now redundant (a plain `Vec<u8>` assignment already
    drops the old value).
  - **`table/cff.rs`'s CFF-charset-derived naming needed one more step
    than a straight `sds_to_vec` swap**: `sdsget_cff_sid`/`form_cid_string`
    return a *freshly allocated* `sds` with no other owner, so after
    copying its bytes into `Glyph.name` via `sds_to_vec`, the now-redundant
    `sds` needs an explicit `sdsfree` — unlike the "borrowed, shared, owned
    elsewhere" sources `sds_to_vec` was originally written for. Six call
    sites (one per charset-format branch, CID and non-CID) follow this
    same `sds_to_vec` + `sdsfree` pair.
  - **`sidof` (the CFF string-interning helper in `table/cff.rs`) was
    deliberately left untouched** — it's shared between the two
    `Glyph.name`-reading call sites (in `cff_make_charset`, building the
    non-CID charset's glyph-name SIDs) and eight other call sites passing
    `CffFontInfo`'s still-`SdsRaw` fields (`cid_registry`, `version`,
    `notice`, `copyright`, `full_name`, `family_name`, `weight`,
    `font_name` — a separate, not-yet-scoped `sds` theme). Widening
    `sidof`'s signature to `&[u8]` would have forced touching all eight of
    those unrelated call sites in the same PR, so instead the two
    `Glyph.name` sites build a short-lived temporary `sds` via `sdsnewlen`
    right before calling `sidof`, and free it immediately after — keeping
    the two themes cleanly separable, at the cost of a small, contained
    round trip.
  - **One genuinely diagnostic-only site**: `wrong_instrs_for_glyph`
    (`table/glyf.rs`) passes the glyph name into an `fprintf(stderr, "...%s...")`
    call on a TrueType-instruction parse error. `fprintf`'s `%s` needs a
    NUL-terminated buffer, which a bare `Vec<u8>` isn't — a byte-copy with
    an appended NUL is built locally for this one call. Never part of
    dumped/built output, so this doesn't need the NUL-truncation care the
    crate's other glyph-name-to-JSON sites take (matching the existing
    "(null)"-fallback carve-out already recorded for `SdsPart`).
  - **Real coverage, no synthetic payload needed**: every existing payload
    exercises `Glyph.name` (it's the glyph's own name), and `KRName-Regular.otf`/
    `KRName-Regular-O2.otf` (CID CFF, non-CID CFF, and CFF subroutinized)
    specifically drive `table/cff.rs`'s CID and non-CID charset-naming
    branches in both directions.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test.
- **The `sds` sweep's last four leaf fields, batched into one PR** at the
  user's request ("まとめてsdsスイープを解消して" — resolve the remaining `sds`
  sweep all together) rather than as four separate PRs. Each is scoped and
  verified independently below, but landed as a single commit/PR since none
  of the four touch the same struct.
  - **`GlyphOrderEntry.name: SdsRaw` → `Vec<u8>`** (`support/glyph_order.rs`,
    `json_reader.rs`, `otf_reader/unconsolidate.rs`, `table/post.rs`,
    `table/glyf.rs`). The trickiest of the four, for a reason specific to
    this type: `otfcc_set_glyph_order_by_gid`'s original `SdsRaw` return
    value was a *borrowed* pointer into the entry's own storage that every
    caller either discarded (~590 fire-and-forget registrations in
    `support/aglfn.rs`/`table/post.rs`) or copied via `sds_to_vec` without
    ever freeing. A `Vec<u8>` can't be borrowed out through an `extern "C"
    fn` return the same way, so the return type changed to an owned
    `Vec<u8>` clone of the canonical name — the ~590 discarding call sites
    needed zero code changes (an unused return value just drops), and the
    few call sites that captured it got simpler (no more `sds_to_vec`
    step, the return is already the right type). Same treatment for
    `otfcc_gord_name_a_field_shared`'s out-parameter (`*mut SdsRaw` →
    `*mut Vec<u8>`). `otfcc_gord_consolidate_handle`'s three
    `handle_consolidate_to` calls became direct `Handle{...}` construction
    (the established simplification, since the name is already the right
    type). New `sds_into_vec` helper in `support/handle.rs` (distinct from
    `sds_to_vec`): takes ownership of *and frees* a possibly-null `SdsRaw`,
    treating null as an empty `Vec` — for `otfcc_set_glyph_order_by_name`'s
    JSON-parse caller, where the old code just stored a possibly-null
    pointer directly. `dispose_glyph_order`'s per-entry loop needed an
    explicit `(*entry).name = Vec::new();` before the raw `libc::free` of
    each entry — same "manual drop-in-place before a raw free" requirement
    as every other `*mut T`-owning, non-`Box`-allocated struct in this
    migration (`libc::free` has no idea about `Vec`'s drop glue).
  - **`Lookup.name`/`Feature.name`/`LanguageSystem.name: SdsRaw` → `Vec<u8>`**
    (`table/otl.rs`, `table/otl/{build,parse,read,dump}.rs`,
    `consolidate/otl/chaining.rs`). All three already had a real `Drop`
    impl (or, for `Feature`/`LanguageSystem`, one whose *only* job was
    freeing `name`) from the earlier `Box<Lookup>`/`Box<Feature>`/
    `Box<LanguageSystem>` Stage-6-4 PR — `Lookup`'s `Drop` simplifies to
    just its type-dispatched `SubtableList` teardown, and `Feature`/
    `LanguageSystem` need no manual `Drop` impl at all anymore. New
    `handle_name_eq_bytes` helper in `support/handle.rs` (alongside the
    existing `handle_name_eq_cstr`): the same NUL-truncating comparison,
    but for two `Vec<u8>`-shaped names now that `Lookup.name` moved off
    `sds` too (`consolidate/otl/chaining.rs`'s lookup-by-name search used
    to compare a `Handle.name` against a `Lookup.name` via `strcmp`-through-
    `CStr`; both sides are `Vec<u8>` now). `table/otl/build.rs`'s
    `feature_name_to_tag` (reads a tag's first 4 bytes) and its
    `write_otl_script_and_languages`' local `ScriptGroup.tag` both widened
    from `SdsRaw`/temporary-`sds` to `&[u8]`/`Vec<u8>` directly, since both
    are entirely internal to this file. `table/otl/read.rs`'s ~6
    `crate::sdsbuild!`-into-a-field sites (synthesizing default lookup/
    feature/language names when the font omits one) needed a `let tmp =
    sdsbuild!(...); field = sds_to_vec(tmp); sdsfree(tmp);` round trip
    each, since `sdsbuild!` still only produces `SdsRaw`.
  - **`CffTable`'s nine font-info fields → `Vec<u8>`** (`font_name`,
    `version`, `notice`, `copyright`, `full_name`, `family_name`, `weight`,
    `cid_registry`, `cid_ordering`; all in `table/cff.rs`). `CffTable`
    loses `#[derive(Copy, Clone)]` (nine `Vec<u8>` fields forces it
    regardless of any `Handle`-embedding concern) — confirmed safe first:
    every use of the type is behind `*mut CffTable`/`*const CffTable`
    (including `fd_array: *mut *mut CffTable`, itself a pointer array, not
    a value array), so there was no cascade. `table_cff_copy` (the `.copy`
    vtable slot, a raw `memcpy`) was already unreachable from any live
    call site and is deleted outright (matching this migration's
    established pattern for confirmed-dead slots) rather than left in
    place, where it would have become unsound (a `memcpy`'d `Vec<u8>`
    aliases its heap buffer). `dispose_fd` (called from `table_cff_free`
    right before a raw `libc::free` of the whole struct) needed the same
    "explicit drop-in-place before the raw free" treatment as
    `GlyphOrderEntry` above, for all nine fields. `sidof` (the CFF
    string-interning helper used to build the CFF string INDEX) had been
    left taking `SdsRaw` in the `Glyph.name` PR specifically *because* it
    was shared with these nine still-`sds` fields — now that they're
    `Vec<u8>` too, `sidof` finally widens to `&[u8]` directly, and the two
    `Glyph.name` call sites in `cff_make_charset` drop the temporary-`sds`
    round trip that PR had to add as a workaround.
  - **`NameRecord.name_string: SdsRaw` → `Vec<u8>`** (`table/name.rs`
    only). `NameRecord` loses `Copy` (keeps `Clone`) for the same reason
    as `CffTable` above. `table_name_dispose`'s explicit per-record
    `dispose_name_record` loop is gone — `Vec::clear()` on
    `Vec<NameRecord>` already drops each element in place, which now runs
    `NameRecord`'s own (derived) drop glue and frees `name_string`'s
    buffer for free, the same simplification every other leaf-type
    conversion in this sweep got. `support/unicodeconv.rs`'s
    `utf8toutf16be` (used once, in this file's `otfcc_build_name`) was
    deliberately *not* widened to `&[u8]` — its internals are a non-trivial
    hand-rolled UTF-8 decoder walking raw pointers derived from `sdslen`,
    complex enough that touching it for a single call site wasn't worth
    the risk, so a temporary `sds` copy is round-tripped at that one call
    site instead (same reasoning as `sidof` before this PR widened it: a
    helper's internals only get touched when it's cheap and safe to do
    so, otherwise the caller absorbs a round trip).
  - **No new synthetic payloads needed for any of the four** — glyph
    naming, OTL lookup/feature/language naming, CFF font info, and the
    `name` table are all exercised by essentially every existing payload;
    `unknown-lookup.ttf` (added for an earlier PR) specifically drives the
    default-name-synthesis paths in `table/otl/read.rs` (51 GSUB lookups,
    many needing synthesized names), and `KRName-Regular.otf`/
    `KRName-Regular-O2.otf` drive the CFF font-info read/dump/build paths.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test.
  - **This closes the `sds` sweep for leaf-type struct fields entirely.**
    The only remaining `sds` usage in the crate is `vendor/sds.rs`'s own
    API surface (by design, permanent) and `support/aglfn.rs`'s ~586
    mechanical `sdsnew` calls feeding `set_by_gid` (already `Vec<u8>`-safe
    on the receiving end; converting the call sites themselves was never
    in scope and remains not worth doing — they're already about as
    simple as C-string literals get).
- **Stage 6-4 "Box化" pilot: `Font.ltsh: *mut LtshTable` → `Option<Box<LtshTable>>`**,
  the first of `Font`'s ~32 `*mut X`-typed table fields to make this move.
  Chosen as the pilot for being the smallest/simplest: `LtshTable` owns
  exactly one nested allocation (`y_pels: *mut u8`) and had no other
  structural complications.
  - **The entire `LtshTableElementInterface` vtable is gone** —
    `table_ltsh_create`/`_init`/`_free`/`_dispose`/`_copy` all deleted.
    Grepping confirmed only `.free` was ever called from outside
    `table/ltsh.rs` (from `font/caryll_font.rs`'s `delete_font_table`);
    `.init`/`.create`/`.copy`/`.dispose` were never called at all —
    `otfcc_read_ltsh`/`stat_ltsh` already built the struct directly via
    `__caryll_allocate_clean`, never through the vtable's `.create`. A
    `Drop for LtshTable` impl (frees `y_pels` if non-null) replaces the
    vtable's `.free`/`.dispose` pair; `Box::new` construction replaces
    `.create`; `Option`'s null-pointer optimization replaces the old
    null-`*mut LtshTable`-means-absent convention, for free, since `Font`
    itself is `malloc`+`memset(0)`'d (calloc-equivalent) and an all-zero
    `Option<Box<T>>` is a valid `None`.
  - **`Box::new` construction, not `Box::from_raw`-wrapping-`calloc`** —
    reconfirmed the crate's only sound pattern for introducing a new
    `Box<T>`: build the value directly (`Box::new(LtshTable { version,
    num_glyphs, y_pels })`), never `calloc`/`__caryll_allocate_clean` the
    struct itself and then `Box::from_raw` it (unprecedented in this
    crate, and technically violates `Box::from_raw`'s allocator-matching
    contract). The nested `y_pels: *mut u8` array is still built via
    `__caryll_allocate_clean` — it's never itself wrapped in a `Box`, only
    manually freed by the `Drop` impl.
  - **`LtshTable` dropped `#[derive(Copy, Clone)]`** — a `Drop` impl and
    `Copy` are mutually exclusive, and `y_pels` needing single ownership
    means `Copy` was already semantically wrong before this PR, just
    unenforced (no call site ever relied on copying it by value).
  - **The cascade landed on `Font` itself, not just `LtshTable`.** `Font`
    embeds `ltsh` directly (not behind a pointer), so `Option<Box<LtshTable>>`
    being non-`Copy` meant `Font` could no longer derive `Copy, Clone`
    either (`E0204`/`E0277` on build). Grepped every use of `Font` in the
    crate — it's accessed exclusively via `*mut Font`/`(*font).field`,
    never returned by value, never constructed as a value literal outside
    its own `otfcc_font_create`, never `.clone()`'d — so dropping the
    derive outright was safe, the same check already applied to
    `CffTable`/`NameRecord`/`GlyphOrderEntry` earlier in this migration.
    This is expected to recur for every subsequent `Font` field this
    theme converts, until the last raw-pointer field is gone and `Font`'s
    `Copy`/`Clone` removal only needs doing once (this PR pays that cost).
  - `otfcc_build_ltsh`'s parameter widened from `*const LtshTable` to
    `Option<&LtshTable>` (internal-only call, never crosses the real FFI
    boundary — matches the crate's established `#[allow(improper_ctypes_definitions)]`
    rationale). `otf_writer.rs`'s one call site adapted via `.as_deref()`.
  - `otf_reader/unconsolidate.rs`'s `merge_ltsh` and `otf_writer/stat.rs`'s
    `stat_ltsh` (the read-merge and write-synthesize call sites) adapted to
    `if let Some(ltsh) = &(*font).ltsh` / `Some(Box::new(LtshTable { .. }))`
    respectively; no behavioral change.
  - **No new synthetic payload needed** — LTSH is a TrueType-only table
    (`FontSubtype::Ttf` gate in `otf_writer.rs`), and every existing
    TrueType payload (`iosevka-r.ttf`, `vtt.ttf`, `Molengo-Regular.ttf`,
    `NotoNastaliqUrdu-Regular.ttf`) already carries one, already exercised
    by `compare-with-c.sh`/`run-cycles.sh`.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test.
  - **This is the opening PR of Stage 6-4's "Box化" theme proper**
    (converting `Font`'s remaining ~31 `*mut X` table fields to
    `Option<Box<X>>` one at a time, per the user's explicit direction to
    continue this theme after the `sds` sweep closed). The pattern this
    PR establishes — delete the vtable, `Box::new` construction, `Drop`
    for any nested owned pointers, `Option`'s null-pointer optimization,
    and (once, here) the `Font: Copy/Clone` removal — is expected to
    repeat with decreasing marginal cost for the simpler remaining fields,
    and increasing care for fields whose tables embed `Vec`-backed
    children (already non-`Copy`) or further nested pointers.
- **Stage 6-4 "Box化", second field: `Font.vorg: *mut VorgTable` →
  `Option<Box<VorgTable>>`.** Same shape as the `LtshTable` pilot —
  `VorgTable` owns exactly one nested allocation (`entries: *mut VorgEntry`)
  and its vtable had the identical dead-slot profile (only `.free` called
  from outside `table/vorg.rs`, from `caryll_font.rs`'s table disposal and
  `unconsolidate.rs`'s merge step; `.init`/`.copy`/`.create`/`.dispose`
  never called at all). `VorgTableElementInterface` deleted entirely,
  `Drop for VorgTable` frees `entries`, `Copy`/`Clone` dropped (same
  "already semantically wrong, just unenforced" reasoning).
  - **`Font` did not need its `Copy`/`Clone` removal repeated** — already
    gone since the `LtshTable` pilot. Confirms the "one-time cost" framing
    from that PR: subsequent fields in this theme pay only their own
    per-type cost, not a recurring `Font`-wide one.
  - `otf_reader/unconsolidate.rs`'s `merge_vmtx` (the one call site reading
    `.vorg`'s contents before consuming it) switched from repeated
    `(*(*font).vorg).field` dereferences to `if let Some(vorg) =
    (*font).vorg.take()`, matching the "take, use the owned value, let it
    drop" pattern already established for `Handle`/`LtshTable`; no separate
    `TABLE_I_VORG.free` call needed afterward since the `Box` drops on its
    own once `vorg` goes out of scope.
  - `otf_writer/stat.rs`'s `stat_vorg` (the write-synthesize call site)
    rebuilt to accumulate `default_vertical_origin`/`entries` as locals and
    construct via `Box::new(VorgTable { .. })` at the end, replacing the
    `__caryll_allocate_clean`-then-field-assign construction of the table
    struct itself (the nested `entries` array is still built via
    `__caryll_allocate_clean`, same as `LtshTable`'s `y_pels` — it's never
    itself `Box`-wrapped, only freed by the `Drop` impl).
  - `otfcc_build_vorg`'s parameter widened from `*const VorgTable` to
    `Option<&VorgTable>` (`#[allow(improper_ctypes_definitions)]`, same
    internal-only-call rationale as `otfcc_build_ltsh`).
  - **No new synthetic payload needed** — VORG is exercised by every
    payload with a `glyf` table and non-default vertical origins; already
    covered by `compare-with-c.sh`/`run-cycles.sh`.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test.
- **Stage 6-4 "Box化", batched: 9 more table types (10 `Font` fields) in
  one PR**, at the user's explicit request to batch this round rather
  than continue the one-field-per-PR cadence. Before implementing,
  surveyed every remaining `Font.*: *mut X` field's call-site count
  (`grep -c` across `otf_reader.rs`/`otf_writer.rs`/`json_reader.rs`/
  `json_writer.rs`/`otf_writer/stat.rs`/`otf_reader/unconsolidate.rs`/
  `font/caryll_font.rs`) — this is the real cost driver, not structural
  complexity: `head`/`maxp`/`vhea`/`hhea`/`os_2`/`cmap` are "simple
  struct, zero owned pointers" (the easiest `Drop`-impl case in the
  abstract) but are read/written 15-35 times each inside
  `otf_writer/stat.rs`'s per-field summary computation, while the 10
  fields converted here all had ≤15 touch sites, matching the
  `LtshTable`/`VorgTable` pilots' actual risk profile. Converted this
  round: `hdmx`, `vdmx`, `cvt_`, `meta`, `gasp`, `cpal`, `fpgm`/`prep`
  (share `FpgmPrepTable`), `hmtx`, `vmtx`.
  - **`HdmxTable` (`hdmx`) turned out to be entirely dead** — grepping
    the whole crate outside `table/hdmx.rs` and `Font`'s own field list
    found zero references: `otfcc_read_hdmx` is never called from
    `otf_reader.rs`, there's no `otfcc_build_hdmx` at all, and
    `caryll_font.rs`'s disposal switch has no arm for it. HDMX (an
    optional, rarely-used hinting table) was apparently never wired into
    this crate's read/build pipeline even in the original C. Converted
    anyway for `Font`-field consistency, at effectively zero risk since
    there are no call sites to break.
  - **`VdmxTable`/`CpalTable`/`GaspTable`/`MetaTable` needed no `Drop`
    impl at all** — every field they (transitively) own was already a
    `Vec`/scalar from earlier Vec化 PRs (`vdmx.ratios: Vec<VdmxRatioRange>`,
    `cpal.palettes: Vec<CpalPalette>`, etc.), so `Box::new` construction
    plus the derived drop glue is sufficient; the vtables existed purely
    to manage a `calloc`d/`malloc`d wrapper struct around already-safe
    contents. `CpalTable` didn't even have a full `XTableElementInterface`
    struct, just free-standing `table_cpal_*` helper functions — same
    "only `.free` (and here, implicitly `.create`) ever called from
    outside the file" pattern applied regardless of the vtable's shape.
    `table_cpal_copy` was confirmed dead (never called anywhere, not even
    self-referentially) and deleted outright, matching `TABLE_I_VDMX`'s
    and others' precedent.
  - **`HdmxTable`/`CvtTable`/`FpgmPrepTable`/`HmtxTable`/`VmtxTable`**
    match the `LtshTable`/`VorgTable` pattern exactly (one or two owned
    raw arrays, real `Drop` impl needed). `HmtxTable`/`VmtxTable` each
    own two independent arrays (`metrics`/`left_side_bearing` and
    `metrics`/`top_side_bearing` respectively) — both freed in the same
    `Drop` impl, matching `HdmxTable`'s two-level array precedent from
    this same batch.
  - **`FpgmPrepTable`'s `otfcc_parse_fpgm_prep` needed a different
    construction shape than every other type converted so far**: it
    builds the table by handing a raw `*mut c_void` to `parse_ttinstr`,
    which invokes a callback (`make_fpgm_prep_instr`) that writes
    `.length`/`.bytes` back through that pointer — a callback-driven
    construction, not a linear "build fields, then `Box::new`" one.
    Solved by allocating the `Box` first with placeholder field values,
    then passing `boxed.as_mut() as *mut FpgmPrepTable as *mut c_void`
    into `parse_ttinstr` so the callback writes directly into the
    already-boxed storage — `Box`'s heap address is stable once
    allocated, so this is sound (the callback never sees or moves the
    `Box` itself, only a raw pointer derived from it, exactly like every
    other `*mut c_void`-callback pattern already in this crate).
  - **`merge_hmtx`/`merge_vmtx`/`stat_vorg`'s guard conditions**
    (`otf_reader/unconsolidate.rs`, `otf_writer/stat.rs`) needed
    `.is_null()`/`.is_none()` swaps at each of the ~10 non-owning
    touch sites across these two files, plus `(*font).X.take()` at the
    point where the table is actually consumed and then implicitly
    disposed — same "take, use the owned value, let it drop" pattern
    established by the `LtshTable` pilot, now applied at higher volume.
  - **No new synthetic payloads needed for any of the nine** — every
    converted table (except the confirmed-dead `HdmxTable`) is exercised
    by existing payloads already wired into `compare-with-c.sh`
    (`vdmx-test.ttf`, `meta-test.ttf`, `BungeeColor-Regular_colr_Windows.ttf`
    for CPAL, and every TrueType payload for gasp/cvt/fpgm/prep/hmtx/vmtx).
  - **Deferred, with reasons recorded for the next round**: `head`/
    `maxp`/`vhea`/`hhea`/`os_2`/`cmap` (15-35 touch sites each,
    concentrated in `otf_writer/stat.rs`'s summary computation — each
    deserves focused attention rather than being rushed into a batch);
    `fvar` (low direct touch count but threaded pervasively through
    dump/parse "context" structs as a borrowed `ctx.fvar` — the true
    blast radius needs auditing those context-construction sites, not
    just direct `Font.fvar` accesses); the six already-`Vec`-backed table
    types (`glyf`/`name`/`colr`/`svg`/`tsi_01`/`tsi_23`/`tsi5`) — need a
    design decision (`Option<Vec<T>>`, which gets the null-pointer niche
    for free since `Vec` is already non-null, vs `Option<Box<Vec<T>>>`)
    before converting; `gdef`/`base` (nested owned types `ClassDef`/
    `BaseAxis` whose own Drop/ownership readiness wasn't checked this
    round); `post` (`post_name_map: *mut GlyphOrder`'s ownership
    relationship to `Font.glyph_order` is unclear); `glyph_order` itself
    (foundational, high blast radius); `gsub`/`gpos` (`OtlTable`, 871
    lines, union-heavy); `cff` (`CffTable`, 3081 lines, by far the
    largest and most structurally complex table).
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test.
- **Stage 6-4 "Box化": `Font.cmap: *mut CmapTable` → `Option<Box<CmapTable>>`**,
  the next field down the deferred-by-touch-count list (15 sites, lowest
  of the six remaining "simple struct" candidates). Unlike every other
  table converted in this theme so far, `CmapTable`'s fields
  (`unicodes`/`uvs`) are already `BTreeMap<_, GlyphHandle>` from an
  earlier uthash-removal PR, so — like `VdmxTable`/`CpalTable`/
  `GaspTable`/`MetaTable` before it — no manual `Drop` impl was needed,
  just `Box::new` construction.
  - **A vtable-deletion grep mistake, caught by the compiler**: this
    file's `CmapTableElementInterface` has four extra "method" slots
    beyond the usual `init`/`dispose`/`create`/`free`
    (`.encode_by_index`, `.lookup`, `.encode_uvs_by_index`, etc.) that
    every other converted table's vtable lacks. A first-pass
    `grep -n "TABLE_I_CMAP\."` (single-line anchored) found only the
    `.create` call sites and concluded the method slots were "never
    called through the vtable" — wrong: `read_uvs_default`/
    `read_uvs_non_default`/`otfcc_build_cmap_format14` call
    `TABLE_I_CMAP.lookup`/`.encode_uvs_by_index` with the method name on
    its own line (`TABLE_I_CMAP\n    .lookup\n    .expect(...)`), which
    a single-line-anchored `\.` pattern never matches. Deleting the
    vtable anyway produced four `E0425: cannot find value TABLE_I_CMAP`
    compile errors immediately — the mistake cost nothing beyond one
    extra build/fix cycle, but it's a lesson for every future
    vtable-deletion grep in this crate: **search for the bare
    identifier** (`grep -n "TABLE_I_X"`, no anchored dot) so multi-line
    method-call syntax can't hide a live call site. Fixed by replacing
    the four call sites with direct calls to the vtable slots' backing
    functions (`otfcc_cmap_lookup`, `otfcc_encode_cmap_uvs_by_index`),
    which were already exported, ordinary functions — same behavior, no
    vtable indirection needed. `.unmap`/`.unmap_uvs`/`.encode_by_name`/
    the other two "by_name" slots were genuinely dead in vtable form
    (confirmed via the corrected grep); kept as ordinary functions
    rather than deleted, since removing live-looking public API during a
    type-only conversion would be scope creep.
  - **`otfcc_read_cmap`/`otfcc_parse_cmap` build the table incrementally
    through helper functions that take a raw `*mut CmapTable`**
    (`read_cmap_mapping_table`, `parse_cmap_unicodes`, etc.) — these
    internal-only signatures were left unchanged; each entry point
    constructs a local `Box<CmapTable>` up front and derives a
    `*mut CmapTable` from it (`cmap_box.as_deref_mut().unwrap() as *mut
    CmapTable` / `cmap_box.as_mut() as *mut CmapTable`) to hand to the
    helpers, then returns the `Box` itself at the end. Matches the same
    "helpers keep taking raw pointers, only the public entry/exit points
    change type" pattern used for `VdmxTable`/`CpalTable`'s internal
    build helpers.
  - **`otfcc_read_cmap`'s corrupted-table branch previously did a raw
    `free(cmap as *mut c_void)`**, bypassing `CmapTable`'s (implicit,
    now-derived) drop glue — safe only because the two `BTreeMap`s were
    still empty at that point (no subtable had been read yet when the
    length check fails), so there was nothing to leak either way.
    Replaced with `cmap_box = None;`, which is both simpler and now
    actually runs the drop glue (a no-op on empty maps, but the
    principled choice going forward).
  - Four call sites outside `table/cmap.rs` needed the usual
    `.is_null()`/`.is_some()` swap: `otf_reader/unconsolidate.rs`
    (glyph-order AGLFN naming pass), `consolidate.rs`'s
    `consolidate_cmap` (twice, for `.unicodes`/`.uvs`), and
    `otf_writer/stat.rs`'s `stat_os_2` guard — the last of these wasn't
    caught by the original call-site survey (a `grep -c` count of
    `(*font).cmap` in `stat.rs` reported 2, but one of the two hits was
    inside `stat_os_2_unicode_ranges`'s own body, which is only reached
    through this guard, and the guard itself was on a `use`-adjacent
    line the count still should have caught — recorded here as a
    reminder that touch-count surveys are a planning aid, not a
    replacement for actually building and reading every compiler error).
  - **No new synthetic payload needed** — cmap is exercised by every
    payload with any Unicode-mapped glyphs, already covered by
    `compare-with-c.sh`/`run-cycles.sh`.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test.
- **Stage 6-4 "Box化": `Font.os_2: *mut Os2Table` → `Option<Box<Os2Table>>`**,
  the next field down the deferred list (18 sites). `Os2Table` is a pure
  scalar/fixed-size-array struct (39 fields, no owned pointers at all) —
  the first genuinely `Copy`-safe struct converted in this theme, so
  `#[derive(Copy, Clone)]` was left untouched on the struct itself (no
  `Drop` impl, no aliasing hazard, nothing to remove).
  - **Construction via `mem::zeroed()`, not a 39-field struct literal**:
    the old `init_os2` did `memset` to all-zero then set `.version = 4`.
    Since every field is an integer or fixed-size byte array (`u16`/
    `i16`/`u32`/`[u8; 10]`/`[u8; 4]`), an all-zero bit pattern is valid
    for all of them, so `let mut v: Os2Table = mem::zeroed(); v.version =
    4; Box::new(v)` reproduces the exact same initial state as the old
    `create()` without writing out every field by hand.
  - **`otfcc_parse_os_2`'s old `if os_2.is_null() { return null; }`
    guard became structurally dead** — it existed because the old
    `table_os_2_create()`'s `malloc` could in principle fail and return
    null; `Box::new` cannot return null (it aborts the process on
    allocation failure instead), so there is no longer anything to
    check. Removed rather than translated into an `Option` check that
    could never fire.
  - **`stat_os_2_unicode_ranges`/`stat_os_2_average_width`/
    `stat_max_context`** (`otf_writer/stat.rs`) each hoist a single
    `let os_2: *mut Os2Table = (*font).os_2.as_deref_mut().unwrap() as
    *mut Os2Table;` at the top (all three are only reachable through
    `stat_os_2`, itself only called after a `(*font).os_2.is_some()`
    guard at the call site), then every `(*(*font).os_2).field`
    dereference in the body mechanically became `(*os_2).field` — same
    pattern as the `LtshTable`/`VorgTable` pilots' "hoist once per
    function" rule, just applied across three call sites sharing one
    guard instead of one.
  - **No new synthetic payload needed** — OS/2 is present in every
    payload, already covered by `compare-with-c.sh`/`run-cycles.sh`.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload,
    all 10 payloads' round trips, and the issue #1 golden test.
- **Stage 6-4 "Box化", batched: `head`/`hhea`/`maxp`/`vhea` (4 `Font`
  fields) in one PR**, per the user's explicit "Box化をまとめて対応して"
  instruction to batch conversions rather than ship one field per PR.
  A call-site-count survey across `otf_reader.rs`/`otf_writer.rs`/
  `json_reader.rs`/`json_writer.rs`/`otf_writer/stat.rs`/
  `otf_reader/unconsolidate.rs` showed `head`/`maxp` are the *most*
  expensive remaining fields to convert (33/35 touches) despite having
  the structurally simplest layout (zero owned pointers) — the cost
  comes entirely from `otf_writer/stat.rs`'s per-field summary
  computation, not from the type itself. `hhea`/`vhea` are cheap (≤15
  touches) and share the same `mem::zeroed()` construction shape, so
  all four were grouped into one PR rather than four.
  - All four structs kept `#[derive(Copy, Clone)]` unchanged — no
    owned pointers, same as `Os2Table`.
  - **`mem::zeroed()` construction, field-by-field audited for which
    defaults must survive**: each of the four `otfcc_parse_X` functions
    was checked field-by-field for whether the old C `init_X`'s
    non-zero default is unconditionally overwritten later in the same
    function body (safe to leave zeroed) or only overwritten if the
    corresponding JSON key is present (must be explicitly preserved).
    `HeadTable.magic_number = 0x5f0f3cf5` and `.units_per_em = 1000`,
    `MaxpTable.version = 0x10000`, `HheaTable.version = 0x10000` are
    never touched elsewhere in their parse functions when the JSON key
    is absent, so they're set explicitly on the zeroed value before
    boxing; every other field in all four structs is unconditionally
    written later in the same function, so the zeroed default is
    discarded either way and needed no explicit handling.
  - **New pattern: "raw-pointer alias, derived once" for
    `otfcc_stat_font`** (`otf_writer/stat.rs`) — this ~140-line
    orchestrator touches `(*(*font).head)`/`(*(*font).maxp)` across
    roughly 35 call sites spread over multiple independently-guarded
    sections. Rather than converting every individual deref to
    `Option`-aware syntax, two raw pointers are derived once at
    function entry — `let head: *mut HeadTable =
    (*font).head.as_deref_mut().map_or(ptr::null_mut(), |h| h as *mut
    HeadTable);` (valid even when the field is `None`, giving a null
    pointer matching old semantics exactly) — then every
    `(*(*font).head)`/`(*(*font).maxp)` mechanically becomes
    `(*head)`/`(*maxp)` and every `!(*font).head.is_null()` becomes
    `!head.is_null()`, preserving every existing null-check-based
    control-flow branch byte-for-byte. Applied via a Python script
    targeting the exact line range rather than hand-editing 35 sites.
  - **"Hoist once via `.unwrap()`" reapplied** for `stat_glyf`/
    `stat_maxp`/`stat_hmtx`/`stat_vmtx` — each is only ever called
    under a confirmed-`Some` guard at its call site (same rule as the
    `Os2Table` `stat_os_2_*` helpers), so each hoists its table pointer
    once via `.unwrap()` at function entry. `stat_hmtx` additionally
    hoists `head` via `.map_or(null_mut(), ...)` (not `.unwrap()`)
    since `.head.flags` is touched unconditionally there without an
    explicit guard in the original code.
  - Vtable deletion for all four types confirmed via bare-identifier
    grep (`grep -n "TABLE_I_HEAD"`, no anchored dot) — the lesson from
    the `CmapTable` PR, where an anchored `grep -n "TABLE_I_X\."` missed
    a method call wrapped onto its own line.
  - **No new synthetic payload needed** — all four tables are present
    in every payload, already covered by `compare-with-c.sh`/
    `run-cycles.sh`.
  - Verified with the standard full pipeline on both macOS (arm64
    native) and Linux (`otfcc-stage0-verify` container) — 0 warnings,
    44/44 tests, ABI export guard, `compare-with-c.sh` byte-identical
    on every payload (including the `otfccdll` cdylib comparison), all
    10 payloads' round trips, and the issue #1 golden test.
- **Stage 6-4 "Box化", batched: `base`/`gdef` (2 `Font` fields) in one PR**,
  the first two fields with nested owned raw pointers (`BaseAxis`,
  `ClassDef`) rather than pure scalars, grouped together because both
  needed the same "leave the nested owner as a raw pointer, add a `Drop`
  impl on the top-level struct" treatment established by `VorgTable`.
  - **`BaseTable`**: `horizontal`/`vertical: *mut BaseAxis` stay raw
    pointers (their own Vec化 is a separate future task); `Drop` calls the
    same `delete_base_axis` helper `dispose_base` always called. `Copy`/
    `Clone` dropped (mutually exclusive with `Drop`, and was already
    semantically wrong — `BaseAxis` ownership can't be duplicated by
    value). Deleted the entire `BaseTableElementInterface` vtable
    (`table_base_init`/`_dispose`/`_create`/`_copy`/`_free`,
    `TABLE_I_BASE`) — construction now goes through `Box::new` directly in
    `otfcc_read_base`/`otfcc_parse_base`.
  - **Preserves an existing leak, not introduced by this PR**:
    `delete_base_axis` only ever freed `(*axis).entries` (and each entry's
    `base_values`), never `axis` itself — true in the pre-Box化
    translation too (`dispose_base` never called `free()` on `horizontal`/
    `vertical`). Not fixed here, same discipline as the `unconsolidate.rs`
    move preserved as-is in the `ChainingRule.apply` PR: byte-for-byte
    disposal parity takes priority over opportunistic bug fixes inside a
    Box化 PR.
  - **`GdefTable`** was already non-`Copy` (its `lig_carets` field is
    already a `Vec<CaretValueRecord>` from an earlier Vec化 pass), so no
    derive to remove — just added a `Drop` impl covering
    `glyph_class_def`/`mark_attach_class_def` (left as raw `*mut ClassDef`,
    freed via the existing `otl_class_def_free`) plus
    `self.lig_carets.clear()` (its own drop glue already frees each
    record's `Handle` name and caret `Vec`). Deleted
    `init_gdef`/`dispose_gdef`/`table_gdef_init`/`_dispose`/`_create`/
    `_free`; `otfcc_read_gdef`/`otfcc_parse_gdef` build via
    `Some(Box::new(GdefTable { .. }))` up front and mutate through
    `gdef.as_mut().unwrap().field`, matching the `GaspTable` PR's
    "accumulator variable is `Option<Box<X>>` from the start" idiom rather
    than building through a raw pointer and wrapping at the end (the
    latter would need `Box::from_raw` on a `calloc`-allocated pointer,
    which isn't a Rust-global-allocator allocation).
  - `consolidate.rs`'s `consolidate_gdef(font, (*font).gdef, options)` call
    (in-place hash consolidation, unrelated to build/dump) updated to
    `(*font).gdef.as_deref_mut().map_or(ptr::null_mut(), |g| g as *mut
    GdefTable)`; `consolidate_gdef` already null-checked its `gdef`
    parameter, so `None` passes through safely.
  - **Verification gap found and closed**: no existing payload has a
    `BASE` table (`GDEF` is exercised by `NotoNastaliqUrdu-Regular.ttf`,
    including its `ligCarets`, so that one had no gap). Added
    `rust/scripts/make-test-base.py` (two scripts, one with a default
    baseline and shared/unique baseline tags across scripts to exercise
    `axis_to_bk`'s tag-dedup path, one with a single baseline and no
    default) and wired it into `compare-with-c.sh` the same way as the
    `vdmx-test`/`meta-test` gaps — both build and dump directions
    byte-identical against C on both platforms.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including the new `base-test` and the `otfccdll` cdylib comparison),
    all 10 payloads' round trips, and the issue #1 golden test.
- **Stage 6-4 "Box化": `Font.post: *mut PostTable` → `Option<Box<PostTable>>`**.
  `PostTable` owns one allocation, `post_name_map: *mut GlyphOrder`
  (populated only when reading a version-2.0 `post` table from an OTF
  file) — left as a raw pointer (its own Box化 depends on how
  `Font.glyph_order`, the same `GlyphOrder` type, is eventually
  represented) and freed via a new `Drop` impl calling the same
  `OTFCC_PKG_GLYPH_ORDER.free` `dispose_post` always called. `Copy`/`Clone`
  dropped. Deleted `PostTableElementInterface`/`init_post`/`dispose_post`/
  `table_post_init`/`_dispose`/`_create`/`_copy`/`_free`/`I_TABLE_POST` —
  grepping (bare identifier, not anchored) confirmed only `.create`/`.free`
  were ever called from outside `table/post.rs`.
  - **Investigated and resolved the ownership ambiguity this field was
    deferred for**: `post_name_map` is a standalone allocation, not an
    alias into `Font.glyph_order` — `otf_reader/unconsolidate.rs`'s
    `create_glyph_order` only ever *reads* from it (to backfill glyph names
    from a version-2.0 `post` table) via `.by_gid.iter()` and never stores
    the pointer anywhere else, so there's no other owner to worry about
    aliasing with. `otfcc_build_post` takes `Font.glyph_order` as a
    *second*, separate parameter (unrelated to `post_name_map`), confirming
    the two are always distinct `GlyphOrder` allocations.
  - **`otfcc_read_post`**: every field is unconditionally overwritten from
    file data in this function's own body, so `mem::zeroed()`'s default is
    never observed — construction builds a local `PostTable` value directly
    (mirroring the internal `map: *mut GlyphOrder` construction, unchanged)
    and wraps it in `Some(Box::new(..))` at the return.
  - **`otfcc_parse_post`**: unlike `otfcc_read_post`, `.version`'s old
    `0x30000` default *does* need to survive — it's inside the
    `if !table.is_null()` guard, so it's only overwritten when the JSON
    has a "post" key. Uses the `Os2Table`/`HeadTable` pattern: `mem::zeroed()`
    + explicit `.version = 0x30000` + `Box::new` + a raw-pointer alias for
    the rest of the (unconditional-construction, JSON-optional-fields) body.
    Also notable: `otfcc_parse_post` unconditionally returns `Some(..)`
    even when no "post" JSON key exists (matching the old always-allocate
    C behavior) — `Font.post` is only ever `None` via the OTF-read path
    finding no `post` table in the packet, never via JSON parsing.
  - `otf_writer/stat.rs`'s single touch point (no hoist needed, unlike
    `head`/`maxp`'s ~35-site orchestrator) became `if !(*font).glyf.is_null()
    && (*font).post.is_some() { (*font).post.as_deref_mut().unwrap()
    .max_mem_type42 = ...; }`. `otf_reader/unconsolidate.rs`'s read-only
    `post_name_map` access became a single `.as_deref().map_or(null_mut(),
    |p| p.post_name_map)` alias, mirroring the "raw-pointer alias, derived
    once" pattern at a much smaller scale.
  - **No new synthetic payload needed** — several existing payloads
    (`BungeeColor-Regular_colr_Windows`, `Molengo-Regular`,
    `NotoNastaliqUrdu-Regular`, `gvar-test`, `vtt`) have `post.version ==
    2.0`, exercising `post_name_map` on both the read and unconsolidate
    paths, already covered by `compare-with-c.sh`.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including the `otfccdll` cdylib comparison), all 10 payloads' round
    trips, and the issue #1 golden test.
- **Stage 6-4 "Box化": `Font.glyph_order: *mut GlyphOrder` →
  `Option<Box<GlyphOrder>>`**, the highest-blast-radius field in this theme
  (17 files, ~26 touch sites) since `GlyphOrder` is used pervasively as a
  bare raw pointer well beyond `Font.glyph_order` itself.
  - **`GlyphOrder` stays a raw-pointer-constructible type everywhere except
    `Font.glyph_order`**: `PostTable.post_name_map`, the `aglfn`/`gord`
    locals in `otf_reader/unconsolidate.rs`'s `create_glyph_order`/
    `otfcc_unconsolidate_font`, all keep going through
    `OTFCC_PKG_GLYPH_ORDER.create`/`.free` unchanged — none of that vtable
    was touched or deleted (unlike every previous Box化 PR in this theme).
    A new `impl Drop for GlyphOrder` (walking `by_gid`, dropping each
    entry's `name: Vec<u8>`, freeing the entry) coexists safely with this:
    a raw pointer being `free()`'d never invokes a type's `Drop` impl, so
    the two ownership styles don't interfere — `Drop::drop` only ever fires
    for a `Box<GlyphOrder>` going out of scope, which after this PR is only
    ever `Font.glyph_order`.
  - **Two construction sites, one per pipeline**, both switched from
    "`.create()` (malloc) then thread the raw pointer through population
    calls" to "`Box::new(GlyphOrder { by_gid: BTreeMap::new(), by_name:
    HashMap::new() })` up front, alias a raw pointer for the *unchanged*
    population code, wrap in `Some(..)` at the return" — the
    `GaspTable`/`CmapTable` "accumulator is `Box<X>` from the start" idiom.
    Not `OTFCC_PKG_GLYPH_ORDER.create()` + `Box::from_raw`: `Box::from_raw`
    requires the pointer to have come from Rust's global allocator, which a
    bare libc `malloc` is not guaranteed to match.
    - `consolidate.rs`'s `otfcc_consolidate_font` (OTF-read pipeline): only
      constructs when `Font.glyph_order` is still `None` after
      `otf_reader/unconsolidate.rs` has already named every glyph (that
      earlier pass builds and discards its own *temporary*, unrelated
      `GlyphOrder` just to backfill `Glyph.name`s, never touching
      `Font.glyph_order`).
    - `json_reader.rs`'s `parse_glyph_order` (JSON-parse pipeline):
      unconditionally constructs and returns `Some(..)` even when the JSON
      has no relevant keys (matching the old always-allocate C behavior,
      same shape as the `post` PR's `otfcc_parse_post`).
  - **~20 consumption sites**, each in a different, mostly single-touch
    function taking `font: *mut Font` — no orchestrator-scale hoisting
    needed. Functions with just a guard (`consolidate_glyf`/
    `consolidate_otl_table`) became `.is_none()`/`.is_some()` checks;
    functions with a guard *and* further use in the body
    (`consolidate_cmap` 4 sites, `consolidate_colr` 3, `consolidate_tsi` 3,
    `consolidate_gdef`/`consolidate_gsub_single` 2 each) each hoist a
    single `let glyph_order: *mut GlyphOrder = (*font).glyph_order
    .as_deref_mut().map_or(ptr::null_mut(), |g| g as *mut GlyphOrder);` at
    function entry; the remaining single-touch call sites (`common.rs`,
    `gsub_multi.rs`, `gpos_single.rs`, `gpos_cursive.rs`,
    `gsub_ligature.rs`, `mark.rs` ×3, `otf_writer.rs`, `json_reader.rs`'s
    `otfcc_parse_glyf` call) inline the same `.as_deref_mut().map_or(..)`
    expression directly at the call.
  - **No new synthetic payload needed** — every payload with a `glyf`
    table already exercises `Font.glyph_order` on both pipelines, already
    covered by `compare-with-c.sh`.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including the `otfccdll` cdylib comparison), all 10 payloads' round
    trips, and the issue #1 golden test.
- **Stage 6-4 "Box化": `Font.fvar: *mut FvarTable` → `Option<Box<FvarTable>>`**.
  Despite the plan's "pervasive via context structs" warning, `Font.fvar`
  itself turned out to have only 3 real touch points — the "pervasiveness"
  is entirely about `*mut FvarTable` being threaded through
  `GlyfIOContext.fvar` (a `#[derive(Copy, Clone)]`, freshly-constructed-
  per-call context struct, unrelated to `Font`'s own ownership) and a
  handful of `::core::ptr::null::<FvarTable>()` literals in `table/cff.rs`/
  `table/glyf.rs` for the non-variable-font case — none of that needed to
  change.
  - **`FvarTable`**: `masters: IndexMap<RegionKey, FvarMaster>`'s values own
    a raw pointer (`region: *mut VqRegion`) — a new `Drop` impl walks
    `masters` and calls the existing `dispose_fvar_master` per entry, same
    shape `dispose_fvar` already had. `Copy`/`Clone` were already absent.
    Deleted `.init`/`.copy`/`.dispose`/`.create`/`.free` from
    `FvarTableElementInterface` and their backing functions
    (`init_fvar`/`dispose_fvar`/`table_fvar_init`/`_dispose`/`_create`/
    `_free`) — confirmed dead outside the file the same way as every prior
    target. **Kept `.register_region`/`.find_master_by_region`**: these
    operate on a bare `*mut FvarTable`/`*const FvarTable` from
    `table/glyf/read.rs`'s gvar tuple-variation parsing, unrelated to
    `Font`'s ownership of the table — `FvarTableElementInterface` shrinks
    to just those two slots rather than being deleted outright, since (also
    unlike every prior target) callers outside `table/fvar.rs` still
    genuinely need it.
  - **`otfcc_read_fvar`**: the old accumulator variable (`fvar: *mut
    FvarTable`) was only ever assigned once, deep inside a long chain of
    nested corruption-guards, immediately followed by unconditional
    `axes`/`instances` population and an immediate `return` — no path
    exists where the corrupted-table branch runs with a non-null `fvar`.
    That let the conversion be simpler than usual: the `Box::new(FvarTable
    { .. })` construction (with a raw-pointer alias for the unchanged
    population loops) replaces the `.create()` call at that same deep
    point, and the corrupted-table branch's `TABLE_I_FVAR.free(fvar)` call
    is deleted outright (dead: there is never anything to free there)
    rather than translated.
  - **Pre-existing leak found, not fixed**: `caryll_font.rs`'s
    `dispose_font` has never had a disposal call for `Font.fvar` — the SFNT
    tag for `fvar` (1719034226) doesn't appear anywhere in
    `delete_font_table`'s match, and no other cleanup path frees it either.
    This predates this PR; converting the field type doesn't change it
    (`Option<Box<FvarTable>>` is simply never set back to `None`, so the
    `Box` — and by extension the new `Drop` impl — never runs for
    `Font.fvar` specifically, matching the old always-leaked behavior
    exactly). Not fixed here, same discipline as `BaseTable`'s
    `delete_base_axis` leak and `otf_reader/unconsolidate.rs`'s
    `unconsolidate_chaining` move earlier in this migration — preserving
    byte-for-byte behavior takes priority over opportunistic fixes inside a
    Box化-only PR.
  - Two consumption sites needed `.as_deref_mut()`/`.as_deref()` updates:
    `otf_reader.rs`/`json_writer.rs`'s `GlyfIOContext { fvar: .., .. }`
    constructions, and `json_writer.rs`'s `otfcc_dump_fvar` call (whose
    signature also moved to the standard `Option<&FvarTable>` +
    `#[allow(improper_ctypes_definitions)]` shape used by every other
    `otfcc_dump_X` in this migration).
  - **No new synthetic payload needed** — `gvar-test.ttf` already exercises
    `fvar`'s axes/instances/region-registration machinery end to end,
    already covered by `compare-with-c.sh`.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including the `otfccdll` cdylib comparison), all 10 payloads' round
    trips, and the issue #1 golden test.
- **Stage 6-4 "Box化", batched: `name`/`colr`/`svg`/`tsi_01`/`tsi_23`/`tsi5`
  (5 already-Vec-backed `Font` fields) in one PR** — `glyf` (the 6th field
  in this group) deferred to its own PR: its construction/disposal is
  woven through `table/glyf.rs` (1400+ lines) plus `glyf/read.rs` and
  `glyf/build.rs`, not a single top-level site like the other five.
  - **Design decision for the plan's open question**: `Font.name`/`.colr`/
    `.svg`/`.tsi_01`/`.tsi_23` are `Option<Vec<T>>`, **not**
    `Option<Box<Vec<T>>>` — `NameTable`/`ColrTable`/`SvgTable`/`TsiTable`
    are already bare `pub type X = Vec<T>` aliases (no wrapper struct), and
    `Vec<T>` already owns its own heap buffer; a second `Box` layer would
    be pure indirection overhead with no benefit. `Option<Vec<T>>` is also
    the exact same size as the old raw pointer (niche optimization on
    `Vec`'s internal `NonNull`), so this is a like-for-like representation
    of the old nullable-pointer-to-heap-vec semantics. `extern "C" fn`s
    returning `Option<Vec<T>>` need `#[allow(improper_ctypes_definitions)]`
    — unlike `Option<Box<T>>`/`Option<&T>`, rustc's FFI-safety lint doesn't
    special-case `Option<Vec<T>>` even though the same niche optimization
    applies at the ABI level.
  - **`NameRecord`/`ColrMapping`/`ColrLayer`/`TsiEntry` needed no new
    `Drop` impl** — all either own only `Vec<u8>` fields directly, or a
    `GlyphHandle`/`Handle`, both already fully self-dropping from earlier
    passes in this migration (the `Handle` pilot, the sds sweep). A plain
    `Vec<T>`'s own `Drop` already recurses through everything.
  - **`SvgAssignment` was the one exception, and got the fix**: its
    `document: *mut Buffer` had no `Drop` — table-level code always walked
    and disposed it manually (`dispose_svg_assignment`/`table_svg_dispose`).
    Rather than keep that manual-walk pattern working for
    `Option<Vec<SvgAssignment>>` (fragile: every future drop site would
    need to remember to walk first), added a real `impl Drop for
    SvgAssignment` (frees `document` via `buffree`) and removed `Copy`/
    `Clone` from the struct — matching the `Handle` pilot's playbook,
    scoped down to a type used only within `table/svg.rs`. Nothing in the
    file relied on `SvgAssignment: Copy`; duplication always went through
    the existing `svg_assignment_dup` deep-copy helper.
  - **`Tsi5Table = ClassDef`(a struct, not a bare `Vec`) is
    `Option<Box<Tsi5Table>>`**, not `Option<Vec<...>>` — different shape
    from the other five. `ClassDef` itself stays a raw-pointer-
    constructible type everywhere else in the crate (`GdefTable`'s two
    `ClassDef` fields, the `OTL_I_CLASS_DEF` package used throughout
    `otl`/`gdef` consolidation); widening `otl_class_def_create`/
    `OTL_I_CLASS_DEF.parse` themselves to return `Box<ClassDef>` would
    ripple well beyond this one field. Instead, a new `unwrap_class_def`
    helper "adopts" the malloc'd value into a genuine `Box`: `ptr::read`
    moves the `ClassDef` value out (only the 3-word `Vec` descriptors are
    copied, not the heap buffers — exactly what a normal Rust move does),
    then the now-empty outer allocation is released with a bare `free`
    (not `otl_class_def_free`, which would incorrectly try to drop the
    `Vec`s a second time).
  - **Construction pattern for the five `Vec`-shaped fields**: all five
    `otfcc_read_X`/`otfcc_parse_X` functions had the same shape already
    established in this theme — an accumulator only ever constructed deep
    inside nested corruption-guards, immediately followed by an
    unconditional loop and `return`. Converted straightforwardly: replace
    the old `table_X_create()` (malloc + placement `Vec::new()`) call with
    a bare local `let mut x: XTable = Vec::new();`, mechanically replace
    `(*x).push(..)` with `x.push(..)`, wrap the return in `Some(..)`. The
    now-dead corrupted-table branches' `table_X_free(x)` calls were
    deleted outright (never reachable with `x` non-null, confirmed the
    same way as `BaseTable`/`FvarTable` before them) rather than adapted.
  - **`consolidate_colr`/`consolidate_tsi`** (`consolidate.rs`) directly
    rebuild `Font.colr`/`Font.tsi_01`/`Font.tsi_23` in place (not through
    read/dump/parse/build) — `consolidate_tsi`'s signature changed from
    `_tsi: *mut *mut TsiTable` to `_tsi: *mut Option<TsiTable>` (still a
    raw pointer to the `Font` field itself, just to the new field type);
    both functions' manual `table_X_free`/`table_X_create` calls around
    the rebuild became a plain `(*font).colr = Some(consolidated);`-style
    assignment, since overwriting the old `Option<Vec<T>>` value already
    drops it.
  - **Every whole-table vtable/create/free helper deleted**: none of
    `table_name_create/_free/_dispose`, `table_colr_create/_free/
    _dispose`, `table_svg_create/_free` (already gone from the prior PR),
    `table_tsi_create/_free` survive outside their own now-removed call
    sites — confirmed via crate-wide grep before deletion, same discipline
    as every previous target in this theme. `table_name_create` is the one
    exception, kept only because `create_font_table`'s long-dead
    `create_table` vtable slot (never called from anywhere, confirmed by
    grep) still references it — deleting that dead branch is out of scope
    here.
  - **No new synthetic payloads needed** — every one of the five fields is
    already exercised by an existing payload (`BungeeColor-Regular_colr_
    Windows.ttf` for `colr`, `Reinebow-SVGinOT.ttf` for `svg`, `vtt.ttf`
    for `tsi_01`/`tsi_23`/`tsi5`, every payload for `name`).
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including the `otfccdll` cdylib comparison), all 10 payloads' round
    trips, and the issue #1 golden test.
- **Stage 6-4 "Box化": `Font.glyf: *mut GlyfTable` → `Option<GlyfTable>`**
  — the 6th and last field of the "already-Vec-backed" batch, deferred
  from the previous PR because its construction/disposal is woven through
  `table/glyf.rs` (1400+ lines) plus `glyf/read.rs`/`glyf/build.rs`,
  rather than a single top-level site.
  - **`Option<Vec<Option<Box<Glyph>>>>`, not `Option<Box<Vec<...>>>`** —
    same reasoning as the previous batch (`GlyfTable` is already a bare
    `pub type GlyfTable = Vec<Option<Box<Glyph>>>` alias; the elements are
    already fully self-owning from an earlier pass in this migration, so
    no new `Drop` impl was needed anywhere for this PR).
  - **New pattern: `unwrap_glyf_table`, reusing the `unwrap_class_def`
    idiom from `Tsi5Table`** — `table/cff.rs`'s CFF glyph extraction still
    builds a `GlyfTable` through `table_glyf_create_n` as a bare
    `*mut GlyfTable` (a separate, much larger conversion: `Font.cff` isn't
    Box化'd yet, so widening that constructor to return an owned `Vec` was
    out of scope). `unwrap_glyf_table` "adopts" that raw pointer at the
    one point it actually needs to become `Font.glyf`
    (`otf_reader.rs`'s CFF-path assignment): `ptr::read` moves the `Vec`
    descriptor out, then the now-empty outer shell is released with a bare
    `free` (not `table_glyf_free`, which would incorrectly try to drop the
    `Vec` a second time). `table/cff.rs` and `table/otl.rs` themselves
    needed **zero changes** — confirmed via grep that both only ever touch
    `GlyfTable` as a type for their own internal scratch space, never
    `Font.glyf` directly.
  - **Producer functions converted directly**: `otfcc_parse_glyf`/
    `otfcc_read_glyf` now return `Option<GlyfTable>` (accumulator-only-
    constructed-under-nested-guards shape, same as every previous target —
    the now-dead corrupted-table-branch frees were deleted, not adapted,
    after confirming reachability the usual way). `otfcc_dump_glyf`/
    `otfcc_build_glyf` take `Option<&GlyfTable>` and derive a raw pointer
    once at function entry (`#[allow(improper_ctypes_definitions)]`, same
    as the `Vec`-shaped fields from the previous PR).
  - **By far the largest consumer surface of any field in this theme**:
    101 compile errors on the first crate-wide build after converting the
    field and the four producer functions, spread across 8 files. The
    overwhelming majority (~60) were in `otf_writer/stat.rs`, which has
    eight separate functions touching `(*font).glyf` with no local null
    check of their own — each relies on a guard several call-levels up
    (`otfcc_stat_font`'s own body, or transitively through `stat_os_2`).
    Every one of them got the same "raw-pointer alias, derived once, with
    a comment naming which caller's guard makes the `.unwrap()` safe"
    treatment already established for `stat_maxp` in an earlier PR;
    `otfcc_stat_font` itself got a third hoisted alias (`glyf`, alongside
    the pre-existing `head`/`maxp`) plus a mechanical whole-function
    `(*(*font).glyf)` → `(*glyf)` replace.
  - **`consolidate.rs`, `otf_reader/unconsolidate.rs`, `otf_reader.rs`,
    `otf_writer.rs`, `json_reader.rs`, `json_writer.rs`** all needed the
    same per-function hoist treatment for their own `(*font).glyf` touch
    sites — `create_glyph_order`/`name_glyphs` in `unconsolidate.rs` and
    `merge_hmtx`/`merge_vmtx`/`merge_ltsh` each documented which caller's
    guard justifies the `.unwrap()`/`.as_mut()`, matching the `stat.rs`
    discipline throughout.
  - **No new synthetic payloads needed** — `glyf` is exercised by nearly
    every existing payload on both the TrueType and CFF-extraction paths
    (`KRName-Regular.otf`'s CFF glyph extraction, `iosevka-r.ttf`/
    `vtt.ttf`'s large-scale TrueType `glyf` read/build, plus the
    `stat.rs`/`consolidate.rs` code paths they all drive).
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including the `otfccdll` cdylib comparison), all 10 payloads' round
    trips, and the issue #1 golden test.
- **Stage 6-4 "Box化": `Font.gsub`/`Font.gpos: *mut OtlTable` →
  `Option<Box<OtlTable>>`** (both fields in one PR — they share the same
  `OtlTable` type and every touch site handles them symmetrically).
  - **No new `Drop` impl needed** — `OtlTable`'s three fields
    (`lookups: Vec<Box<Lookup>>`, `features: Vec<Box<Feature>>`,
    `languages: Vec<Box<LanguageSystem>>`) were already fully
    self-dropping from the Stage 6-1 Vec化 pass (`Lookup` has its own
    `Drop` for the type-dispatched `SubtableList` teardown; `Feature`/
    `LanguageSystem` need none). A plain `Option<Box<OtlTable>>` going to
    `None` (or out of scope) already does everything `table_otl_free` used
    to do by hand, so `table_otl_free` and the three now-pointless
    `otl_{lookup,feature,lang_system}_list_dispose` wrapper functions it
    called (each already just `*arr = Vec::new()`) all became fully dead
    and were deleted — confirmed via crate-wide grep before removal, same
    discipline as every previous target in this theme.
  - **`table_otl_create` (`calloc`-based) stays**, unlike `table_otl_free`
    — it's still referenced by `create_font_table`'s long-dead
    `create_table` vtable slot (never called from anywhere, confirmed by
    grep — the same reason `table_name_create` was kept unconverted in the
    `name`/`colr`/`svg` batch). The two live producer functions
    (`otfcc_read_otl_common`, `otfcc_parse_otl`) no longer call it at all;
    they build via `Box::new(OtlTable { .. })` directly instead, following
    the `FvarTable`/`go_box` "accumulator is `Option<Box<X>>`/`Box<X>`
    from the start" idiom — a raw-pointer alias derived once right after
    construction lets the rest of each function's existing `(*table)`
    body (hundreds of lines, deeply nested) stay completely unchanged.
  - **`otfcc_read_otl_common`'s old `if !table.is_null() { .. }
    table_otl_free(table); return null;` shape collapses cleanly**: since
    `Box::new` can't yield null, the derived alias is checked exactly as
    before (harmless — never actually false, but zero-diff to keep), and
    the corrupted-table tail simply becomes `return None;` — the boxed
    accumulator was never moved into `Some(..)`, so it drops correctly on
    its own without an explicit free call. `otfcc_parse_otl` follows the
    same shape, with the pre-existing "only log the warning if a table was
    actually allocated" guard (`if !otl.is_null()`) becoming
    `if otl_box.is_some()`.
  - **`otfcc_dump_otl`/`otfcc_build_otl`** take `Option<&OtlTable>` and
    derive a `*const OtlTable` once at function entry, matching
    `otfcc_dump_glyf`'s pattern from the previous PR (no
    `improper_ctypes_definitions` needed here, unlike the `Option<Vec<T>>`
    fields — `Option<&T>` already gets rustc's FFI-safety niche-
    optimization exemption).
  - **Consumer surface was much smaller than `glyf`'s**: only 5 files
    beyond the table's own read/parse/dump/build (`caryll_font.rs`,
    `consolidate.rs`, `otf_writer/stat.rs`, `otf_writer.rs`,
    `json_writer.rs`, `otf_reader/unconsolidate.rs`), and within those,
    only a handful of touch points each — `consolidate_otl_table`/
    `stat_max_context_otl`/`expand_chain` all already took bare
    `*mut`/`*const OtlTable`, so every call site just needed a derived
    raw-pointer alias (`.as_deref()`/`.as_deref_mut()`/`.as_mut()`), not a
    signature change.
  - **No new synthetic payloads needed** — every existing payload with
    `GSUB`/`GPOS` tables (`NotoNastaliqUrdu-Regular.ttf`,
    `iosevka-r.ttf`, the synthetic `unknown-lookup.ttf`, etc.) exercises
    both fields' full read/parse/dump/build/consolidate paths already.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including the `otfccdll` cdylib comparison), all 10 payloads' round
    trips, and the issue #1 golden test.
- **Stage 6-4 "Box化": `Font.cff: *mut CffTable` → `Option<Box<CffTable>>`**
  — the last field of the Box化 theme, and the largest/most structurally
  complex table in the crate (3081 lines). **Scope was deliberately
  narrow**: only `Font.cff` itself (the top-level table) becomes
  `Option<Box<CffTable>>`. `CffTable.fd_array: *mut *mut CffTable` is
  self-referential (a CID-keyed CFF font's per-Font-DICT tables, each a
  full `CffTable`) and stays raw-pointer-based, along with
  `private_dict`/`font_matrix` — matching the `GdefTable`/`ClassDef`
  precedent ("only the `Font`-owned top slot gets boxed, the type itself
  stays raw-pointer-friendly for internal recursive/shared use").
  Widening `fd_array` to `Vec<Box<CffTable>>` is a separate future task.
  - **`impl Drop for CffTable` reuses `dispose_fd` unchanged** rather than
    duplicating its logic — `dispose_fd` already frees exactly the three
    raw-pointer fields (`private_dict`, `font_matrix`, `fd_array`,
    recursing into `fd_array` children via the still-live
    `TABLE_I_CFF.free`) that a derived `Drop` wouldn't otherwise know how
    to release; the `Vec<u8>` font-info fields it also resets to
    `Vec::new()` get harmlessly re-dropped (as empty, so a no-op) by
    Rust's own field-drop after the custom `drop()` body returns. This
    keeps the manual recursive-free path (still used for `fd_array`
    children) and the new automatic top-level path doing identically the
    same teardown.
  - **New `unwrap_cff_table` helper, reusing the `unwrap_glyf_table`/
    `unwrap_class_def` idiom**: `table_cff_create` (`malloc`-based) and
    `fd_from_json` are shared between the top-level table and `fd_array`
    children (both built the very same way), so neither producer function
    could switch to constructing via `Box::new` directly without also
    making the recursive FD-array construction `Box`-aware — out of scope
    here. `unwrap_cff_table` instead adopts the malloc'd top-level pointer
    at the one point it needs to become `Font.cff`: `ptr::read` moves the
    value out (shallow — the `Vec<u8>` heap buffers and the three raw
    pointer fields all move with it, nothing duplicated), the now-empty
    shell is released with a bare `free` (not `table_cff_free`, which
    would incorrectly re-drop everything), and the value is placed into a
    fresh `Box::new`.
  - **Producer functions**: `otfcc_parse_cff` now returns
    `Option<Box<CffTable>>`, calling the unchanged `fd_from_json(dump,
    options, true)` internally and wrapping the result through
    `unwrap_cff_table`. `otfcc_read_cff_and_glyf_tables` (the TrueType/CFF
    dual-path SFNT reader) stays fully unchanged, still returning
    `CffAndGlyf { meta: *mut CffTable, .. }` — its single consumer
    (`otf_reader.rs`) applies `unwrap_cff_table(cffpr.meta)` at the
    assignment into `Font.cff`, the same shape as the `glyf` field's
    `unwrap_glyf_table(cffpr.glyphs)` from the previous PR.
  - **`otfcc_dump_cff`** takes `Option<&CffTable>` and derives a raw
    pointer once at entry, matching `otfcc_dump_otl`/`otfcc_dump_glyf`.
    **`otfcc_build_cff`/`CffAndGlyf`/`CffExtractContext`/
    `FdArrayCompileContext` all stay unchanged** — they already only ever
    carry `*mut CffTable`, so callers just needed a derived raw-pointer
    alias (`.as_deref()`/`.as_deref_mut()`) at existing construction
    sites, not a signature change.
  - **Consumer surface**: 6 files beyond the table's own read/parse/dump/
    build (`caryll_font.rs`, `consolidate.rs`, `otf_writer/stat.rs`,
    `otf_writer.rs`, `json_writer.rs`, `otf_reader.rs`), each needing only
    a handful of touch points — `consolidate_fd_select`/
    `stat_cff_widths`/`otfcc_stat_font` already took bare `*mut CffTable`.
  - **No new synthetic payloads needed** — `KRName-Regular.otf`/
    `KRName-Regular-O2.otf` are CID-keyed CFF fonts that already drive the
    full `fd_array` recursion (multiple Font DICTs) through read, dump,
    parse, build, and — critically for this PR — repeated dump/build
    cycles in `run-cycles.sh`, which would surface a double-free or leak
    in the recursive teardown path if the `Drop`/`unwrap_cff_table` split
    were wrong.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including the `otfccdll` cdylib comparison), all 10 payloads' round
    trips, and the issue #1 golden test.
- **Box化 theme complete.** Every `Font` field that was a raw
  `*mut X`/nullable pointer at the start of Stage 6-4 is now either
  `Option<Box<X>>`, `Option<Vec<T>>`, or (for `glyf`) the direct
  `Option<GlyfTable>` shape — see the "残り32型の棚卸し"/"Stage 6-1"
  sections above for the full field-by-field history. Deeper structural
  work remains inside individual tables (`fd_array`'s self-reference,
  `private_dict`/`font_matrix`'s own Box化, `Subtable`'s eventual tagged-
  enum conversion, etc.) but those are separate future tasks, not part of
  this theme's own scope.
- **Stage 6-4 follow-up: `CffTable.private_dict`/`font_matrix` →
  `Option<Box<CffPrivateDict>>`/`Option<Box<CffFontMatrix>>`** — the
  first of the three follow-ups the `cff` Box化 PR flagged as future
  work, chosen first for being the smallest: unlike `Font.cff` itself,
  these two fields belong to `CffTable` *itself*, so the type change
  applies uniformly whether the enclosing `CffTable` is the boxed
  top-level `Font.cff` or a still-raw-pointer `fd_array` child — no
  "shared constructor between boxed and raw" complication like
  `unwrap_cff_table` needed to solve.
  - **`CffFontMatrix` needed no `Drop` impl at all** — its `x`/`y: VQ`
    fields were already fully self-dropping from the Stage 6-1 `VQ`
    pilot (`Vec<VqSegment>`). The old `dispose_font_matrix` helper (which
    called `I_VQ.dispose` on `x`/`y` before the caller `free()`'d the
    struct) became pure redundancy once `font_matrix` owns its allocation
    through `Box` — deleted outright, not adapted.
  - **`CffPrivateDict` did need a real `Drop`** — its six `*mut c_double`
    blue-value/stem-snap arrays stay raw pointers (their own Box化/Vec化
    is separate future work), so `impl Drop for CffPrivateDict` frees
    exactly those six arrays, replacing `otfcc_delete_privatedict`
    one-for-one. `#[derive(Copy, Clone)]` had to come off (Rust forbids
    deriving `Copy` on a type with a custom `Drop`); confirmed via grep
    that nothing relied on cloning it.
  - **`otfcc_new_cff_private()` construction simplified**: it used to
    `__caryll_allocate_clean` (calloc) then set 4 non-zero-default fields;
    now it's a plain safe `fn otfcc_new_cff_private() -> Box<CffPrivateDict>`
    (no `unsafe`, no `extern "C"` — confirmed via grep it's never called
    through a vtable slot) building the whole struct literal in one
    `Box::new`, matching the `new_lookup`/`new_feature` idiom. The one
    genuinely delicate transfer site — `fd_from_json`'s force-CID path,
    which hands the top-level table's already-populated `private_dict`
    off to the newly-created `fd_array[0]` and gives the top-level table a
    fresh one — became `(*fd0).private_dict = (*table).private_dict.take();
    (*table).private_dict = Some(otfcc_new_cff_private());`, the `.take()`
    idiom doing exactly what the old raw-pointer reassignment did.
  - **`otf_writer/stat.rs`'s `otfcc_stat_font` font_matrix-recompute
    block** (the largest single touch point) rebuilds `font_matrix` from
    scratch for both the top-level table and every `fd_array` child based
    on `head.units_per_em`; the old dispose-then-allocate-then-fill shape
    collapsed to a plain `(*cff).font_matrix = None;` (old value drops
    for free) followed by `Some(Box::new(CffFontMatrix { .. }))` built in
    one literal — no more `__caryll_allocate_clean` + six sequential field
    assignments through a raw pointer.
  - **No new synthetic payloads needed** — the same `KRName-Regular.otf`/
    `KRName-Regular-O2.otf` CID-keyed CFF payloads that covered the
    `Font.cff` PR's `fd_array` recursion also drive `private_dict`'s
    parse/read/dump/build paths and (`-O2`, via `run-cycles.sh`'s repeated
    dump/build) `font_matrix`'s scale-recompute path on every cycle.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including the `otfccdll` cdylib comparison), all 10 payloads' round
    trips, and the issue #1 golden test.
- **Stage 6-4 follow-up: `CffTable.fd_array: *mut *mut CffTable` →
  `Vec<Box<CffTable>>`** — the second of the three follow-ups the `cff`
  Box化 PR flagged, and the deeper one: `fd_array` is self-referential
  (a CID-keyed CFF font's per-Font-DICT tables, each itself a full
  `CffTable`), and unlike `private_dict`/`font_matrix` it's accessed
  through a **recursive, index-based, partially-constructed** pattern
  during binary parsing, not just read/written as a single value.
  - **The core problem**: `callback_extract_fd`/`callback_extract_private`
    re-derive which `CffTable` they're populating via
    `context.fd_array_index` + `(*meta).fd_array[index]`, and this lookup
    has to resolve correctly *while that element is still being filled
    in* (the recursive DICT parser writes fields into it across many
    callback invocations). The fix: push each element (adopted via
    `unwrap_cff_table`, same idiom as `Font.cff` itself) into the `Vec`
    **before** starting its recursive parse, not after — a `Box`'s heap
    address never moves even if a later `push` reallocates the `Vec`'s
    own backing buffer of `Box` pointers, so indexing into an
    already-pushed-but-not-yet-fully-populated element is completely
    safe. The JSON-parse path (`fd_from_json`) doesn't have this
    complication — each recursive call returns a fully-built `CffTable`
    in one shot, so those elements just get adopted and pushed normally.
  - **`fd_array_count: TableId` field removed entirely**, replaced by
    `.fd_array.len()` everywhere (matching the `ChainingRule.apply`
    precedent) — one less place for the count and the container to drift
    apart.
  - **`CffTable` needs no custom `Drop` impl at all anymore.** With
    `private_dict`/`font_matrix` already `Option<Box<>>` (previous PR)
    and now `fd_array: Vec<Box<CffTable>>`, every field is a genuinely
    owned Rust type that self-drops through ordinary compiler-generated
    field-by-field drop glue — recursively, since each `fd_array`
    element is itself a full `CffTable`. The previous `impl Drop for
    CffTable` (which called `dispose_fd`, whose only remaining real work
    was the raw `fd_array` recursive-free loop) is deleted outright,
    along with `dispose_fd`/`table_cff_dispose`/`table_cff_free` (all
    fully dead once that loop is gone — confirmed via crate-wide grep).
    `table_cff_create`/`table_cff_init`/`init_fd` (`malloc`-based) stay:
    every `CffTable` value, top-level or `fd_array` child, is still
    constructed this way before being adopted into a `Box`.
  - **`FdArrayCompileContext.fd_array`** (the separate context struct
    `cff_make_fdarray`/`callback_makefd` use to iterate an
    already-complete `fd_array` on the *build* side) changes from
    `*mut *mut CffTable` to `*const Vec<Box<CffTable>>` — a raw pointer
    to the owning `CffTable`'s own `fd_array` field, since this side only
    ever needs read access to already-fully-built elements.
  - **`dangerous_implicit_autorefs` hit again** (12 sites this time) —
    the same lint the `otl` pointer-list and `glyf` PRs hit: indexing a
    `Vec` reached through a raw pointer deref needs an explicit
    `(&(*ptr).field)[idx]`/`(&mut (*ptr).field)[idx]` wrap, or a
    hoisted `&Vec<..>`/`&mut Vec<..>` local, rather than
    `(*ptr).field[idx]` directly.
  - **No new synthetic payloads needed** — the same CID-keyed CFF
    payloads (`KRName-Regular.otf`/`-O2.otf`) that covered the previous
    two `CffTable` PRs already drive the full binary-parse recursive
    construction, the JSON-parse construction, the dump path, and (via
    `run-cycles.sh`'s repeated cycles) the build path, on every run — a
    double-free or an off-by-one in the "push before populate" ordering
    would have shown up as a crash or a byte mismatch in this same
    existing coverage.
  - Verified with the standard full pipeline on both macOS (arm64 native)
    and Linux (`otfcc-stage0-verify` container) — 0 warnings, 44/44 tests,
    ABI export guard, `compare-with-c.sh` byte-identical on every payload
    (including the `otfccdll` cdylib comparison), all 10 payloads' round
    trips, and the issue #1 golden test.
