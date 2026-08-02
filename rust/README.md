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
- **The two CFF operator tables want to be newtypes.** `cff_DictOperator` and
  `cff_CharstringOperator` are `i32` aliases now, which is honest about what
  they are — numbers, not a closed set — and cost 155 casts to say so. What it
  does not buy is the one check worth having: **38 of the numbers mean one thing
  in a DICT and something else in a CharString** (`op_Notice` and `op_hstem` are
  both 1; `op_FDArray` and `op_hflex1` are both 3108). Nothing is wrong today —
  the 105 names never collide and the two sets are read by disjoint code — but
  the compiler cannot see the distinction through an alias.
  `the_two_operator_tables_share_numbers` records the overlap. Making them
  newtypes means giving `cffdict_input_*` and `il_push_op` the specific type
  instead of a bare integer, which is a change to the CharString interpreter's
  plumbing and belongs in its own PR.
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
  `tests/golden/` and hand `compare-with-c.sh`'s job over to it — otherwise
  removing C removes the safety net that makes all of this checkable.
