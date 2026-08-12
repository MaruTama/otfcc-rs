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
re-run c2rust, and — since `tests/golden/` was frozen (see below) — do
**not** need `c/` present, built, or even checked out either:

```bash
./rust/scripts/build-crate.sh        # cargo build --release + cargo test
./rust/scripts/check-abi.sh          # the exported C ABI surface is unchanged
./rust/scripts/compare-with-golden.sh # compare byte-for-byte against tests/golden/
./rust/scripts/run-cycles.sh         # dump/build cycles against the Rust binaries
node rust/scripts/compare-roundtrips.js
```

(`./rust/scripts/test.sh` = `build-crate.sh` + `check-abi.sh` +
`compare-with-golden.sh` + `run-cycles.sh`, for convenience.) None of this
needs Docker, c2rust, a C compiler, or a specific architecture — plain
`rustup`/`cargo`.

If you're changing behavior in a way that's meant to keep matching C (as
opposed to a deliberate, intentional divergence), `compare-with-c.sh` is
still there for that: it builds C with clang and compares byte-for-byte
against the live Rust build, the same check `compare-with-golden.sh` now
runs against a frozen snapshot instead. Confirm with it, then run
`generate-golden.sh` to refresh `tests/golden/` and commit the result
alongside the change that motivated it. See "CI decoupled from C" further
down for the full story.

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
