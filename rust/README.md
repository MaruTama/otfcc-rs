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
- **`sdslen`**, the last duplicated `static inline` now that `json-funcs.h` is
  done: 20 copies, one per file that measures an `sds`. The rest of the `sds.h`
  inlines (`sdsavail`, `sdssetlen`, `sdsalloc`, …) are already single, so this is
  one name — but it needs a `pub sdslen` in `vendor/sds.rs`, which is also what
  `json_from_sds` is waiting for.
- **Rust naming for the whole crate is done** (types, enum variants,
  constants, statics, locals, functions, struct fields and modules — see
  each above) and all three naming `allow`s are gone from `lib.rs`. Stage 4
  is complete.
- **Then safe Rust, type by type**: `CVecRaw<T>` → `Vec<T>` first (one
  implementation backs ~37 container types), then `sds` → `String`,
  `caryll_Buffer` → `Vec<u8>`, `malloc`/`dispose` → `Box` + `Drop`, and the
  7,682 `.offset()` calls into slices and iterators. Each of those PRs should
  end by deleting its files' `allow(unsafe_op_in_unsafe_fn)`: 120 files carry
  one today, and that count is the honest measure of how much of this crate is
  still C.
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
  `tests/golden/` and hand `compare-with-c.sh`'s job over to it — otherwise
  removing C removes the safety net that makes all of this checkable.
