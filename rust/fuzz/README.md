# Fuzzing (cargo-fuzz)

Two libFuzzer targets over the two attacker-reachable byte-stream boundaries
in this crate:

- **`otf_parse`** — sfnt/OTF binary parsing: `otfcc_read_sfnt` (table
  directory + table data) then `read_otf` (the per-table readers: `head`,
  `cmap`, `glyf`, `otl`, CFF, ...). This is the path
  `rust/README.md`'s Phase 5 plan identified as having real, C-inherited
  memory-safety gaps on malformed input — unvalidated offsets in
  `cmap.rs`/`name.rs`/`post.rs`, glyph readers with no length parameter at
  all, a CFF INDEX overflow that computes a ~4GB `memcpy` — and as never
  having been fuzzed before this target existed.
- **`json_build`** — the JSON-to-font builder, fuzzed through the actual
  public FFI entry point (`otfccbuild_json_otf`), the one boundary a real
  embedder calls. No internal reflection needed for this one.

## Running

```bash
cd rust/fuzz
cargo fuzz run otf_parse -- -max_total_time=60
cargo fuzz run json_build -- -max_total_time=60
```

This subdirectory is its own cargo workspace (`fuzz/Cargo.toml`'s
`[workspace] members = ["."]`) with its own `rust-toolchain.toml` pinned to a
dated **nightly** — the sole exception to "stable only" anywhere in this repo
(see that file's comment for why: cargo-fuzz needs `-Z sanitizer=address`,
which stable rustc refuses). Nothing about `rust/`'s own `cargo
build`/`cargo test`/`cargo clippy` sees this toolchain file or this
workspace; they're isolated on purpose.

Seed corpora aren't committed (`fuzz/corpus/` is gitignored, matching
cargo-fuzz's own default) to avoid duplicating the payloads already checked
into `tests/payload/`. Seed from there before a real run:

```bash
mkdir -p corpus/otf_parse corpus/json_build
cp ../../tests/payload/*.{ttf,otf} corpus/otf_parse/
echo '{}' > corpus/json_build/empty.json
```

`Cormorant-Medium.otf` and `WorkSans-Regular.otf` no longer need excluding
from `otf_parse`'s seed corpus. Both used to trigger a stack-overflow from
the CFF outline interpreter's unbounded `callsubr`/`callgsubr` recursion
(confirmed to crash the *C* toolchain too, not a migration regression) —
fixed by capping subroutine call nesting at the Type 2 Charstring spec's
own limit of 10 (`libcff/cff_parser.rs`'s `MAX_SUBR_CALL_DEPTH`, `rust/
README.md`'s Stage 7-4 file-I/O-adjacent CFF fix). Both files now parse
cleanly through `otf_parse`.

## Known findings, not yet fixed (Phase 5 plan, Stages 7-1/7-3)

Fuzzing found real bugs within the first ~20 seconds of running each target
against the seed corpus above. None are fixed in the PR that added this
infrastructure — see that PR's rationale for why (production-code changes
belong to the specific plan stage that already scoped them, not to
infrastructure setup) — but they're documented here and reproducible from
`tests/fuzz-corpus/known-issues/` so Stage 7-1/7-3 has a concrete starting
point instead of having to rediscover them:

- ~~`tests/fuzz-corpus/known-issues/json-build-cff-charset-null-glyf.bin`
  (13 bytes, `{"CFF_": {}}`) — `table/cff.rs`'s `cff_make_charset` (and, one
  level up, `cff_make_fdselect` and `cff_make_charstrings`) dereferenced
  `glyf: *mut GlyfTable` without checking it for null~~ — **fixed**: all
  three shared one caller, `writecff_cid_keyed` (`otfcc_build_cff`'s only
  path), so the null check moved there instead of being duplicated three
  times — a null `glyf` is now substituted with a local empty `GlyfTable`
  before any of the three run. Fixing that alone still panicked one level
  further in: `cff_make_charstrings`'s own "0 glyphs" early return left its
  three `*mut Buffer` out-params (`s`/`gs`/`ls`) at the null the caller
  pre-initialized them to, and the caller dereferences all three
  unconditionally right after the call returns — fixed by having that
  early return populate them with empty (not null) `Buffer`s instead.

- ~~`tests/fuzz-corpus/known-issues/json-build-advance-width-subtract-overflow.bin`
  (a `cid-fdselect-test.json`-derived payload with an absurd `advanceWidth`,
  e.g. `50188...` repeated to ~90 digits) — `libcff/charstring_il.rs`,
  `glyph_adw_const as c_int - nominal_width as c_int` panicked with
  "attempt to subtract with overflow"~~ — **fixed**: `glyph_adw_const as
  c_int` already saturates a huge-magnitude `f64` to `i32::MIN`/`MAX`
  rather than wrapping (correct, checked Rust behavior on its own), but the
  subsequent plain `-` could still underflow past `i32::MIN` once
  `nominal_width` was subtracted from a saturated `i32::MIN`. Switched to
  `saturating_sub`, so the extreme case clamps instead of panicking (under
  debug-assertions) or silently wrapping to a nonsensical advance-width
  delta (in an ordinary release build, the quieter bug that was hiding
  behind the loud one).

- **`otf_parse` leaks 478 bytes across 9 allocations on `tests/payload/gvar-test.ttf`** —
  a real, valid, already-extensively-tested payload, not a malformed or
  synthetic one, so this is a genuine pre-existing production leak in
  normal operation, not just an edge-case robustness gap. Confirmed
  reproducible locally with symbols (`cargo fuzz run otf_parse
  ../../tests/payload/gvar-test.ttf`, `ASAN_OPTIONS=detect_leaks=1`). Two
  distinct allocation sites in the trace: a `calloc` inside
  `table/glyf/read.rs::otfcc_read_glyf` (32 bytes), and a `format!()`
  call inside `table/fvar.rs::fvar_register_region` (the `FvarMaster.name`
  field, 2 bytes plus its `Vec` backing) reached through the same
  function. Both are downstream of `VqRegion` — the C flexible-array-member
  gvar/fvar tuple-variation-region type the Phase 5 plan's Stage 6-4
  section already flagged as one of the "difficult" remaining raw-pointer
  types (`FvarMaster.region: *mut VqRegion`, "owned, but `VqRegion` is a
  C flexible array member so it can't be a plain `Box` until `VqRegion`
  itself is `Vec`-ified"). Likely root cause: some region-deduplication or
  error path frees/reassigns `region` without going through the drop chain
  that would also free the `FvarMaster` entry pointing at it, orphaning
  the entry's own heap fields. Not investigated further or fixed here —
  belongs to the same Stage 6-4 ownership work as the rest of `VqRegion`.
  This is why `otf_parse`'s CI step doesn't use `-detect_leaks=0`: leaving
  leak detection on is what caught this, and disabling it to get a
  "clean" advisory job would just hide this class of finding going
  forward too.

- ~~`otf_parse` cannot yet run a long, unattended fuzzing campaign because a
  short/truncated input reaches `otfcc_get16u`/`otfcc_get32u` calling
  `libc::exit(EXIT_FAILURE)` directly from library code~~ — **fixed**: Stage
  7-3 and Stage 7-4's `font/caryll_sfnt.rs` rewrite moved both functions to
  `Option`-returning, `std::io`-based reads with no `exit()` anywhere in the
  read path.

- ~~`otf_parse` can be made to `malloc()` an attacker-chosen, multi-gigabyte
  allocation from a single small input — `font/caryll_sfnt.rs`,
  `otfcc_read_packets` allocated `vec![0u8; length as usize]` for every
  table directory entry using that entry's raw, unvalidated `length` field
  *before* checking the entry's bytes actually exist in the file~~ —
  **fixed**: the function now gets the file's real length once up front
  and checks each entry's `offset + length` against it before allocating.
  Reproducer (`tests/fuzz-corpus/known-issues/otf-parse-table-length-oom.bin`,
  a 961-byte input that used to request a 3.7GB allocation) confirmed
  fixed, now returning null in ~1ms.

## CI

The workflow runs both targets for a short, fixed time budget on every push
(not exhaustive fuzzing — that would need a much longer-running, separately-
scheduled job) as an **advisory, non-blocking** step
(`continue-on-error: true`), the same treatment `cargo miri test` gets and
for the same reason: this is new dynamic-analysis tooling whose current
findings (above) are real but not yet triaged into fixes, so treating it as
a hard merge gate today would just permanently red the build for reasons
unrelated to whatever change a given PR is actually making. Revisit once
Stage 7-1/7-3 have addressed the findings above — a clean fuzzing run *then*
is worth gating on.
