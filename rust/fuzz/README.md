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
cp ../../tests/payload/*.{ttf,otf} corpus/otf_parse/   # see exclusion below
echo '{}' > corpus/json_build/empty.json
```

**Exclude `Cormorant-Medium.otf` and `WorkSans-Regular.otf` from
`otf_parse`'s seed corpus.** Both trigger the same pre-existing, already-
documented stack-overflow in the CFF outline interpreter's unbounded
recursion (`rust/README.md`, "Status: Phase 1 complete" — confirmed to crash
the *C* toolchain too, not a migration regression). Fuzzing rediscovers this
same crash class within seconds of mutation from almost any CFF-bearing
seed, which is expected and not a new finding — but seeding directly from
either file means the very first run reports a crash before the fuzzer has
explored anything else.

## Known findings, not yet fixed (Phase 5 plan, Stages 7-1/7-3)

Fuzzing found real bugs within the first ~20 seconds of running each target
against the seed corpus above. None are fixed in the PR that added this
infrastructure — see that PR's rationale for why (production-code changes
belong to the specific plan stage that already scoped them, not to
infrastructure setup) — but they're documented here and reproducible from
`tests/fuzz-corpus/known-issues/` so Stage 7-1/7-3 has a concrete starting
point instead of having to rediscover them:

- **`tests/fuzz-corpus/known-issues/json-build-cff-charset-null-glyf.bin`**
  (13 bytes, `{"CFF_": {}}`) — `table/cff.rs:2302`, `cff_make_charset`
  dereferences `glyf: *mut GlyfTable` without checking it for null.
  `"CFF_"` present with no corresponding glyph data reaches this with a null
  `glyf` pointer. Under this fuzz build's `-C debug-assertions` (which
  enables Rust's newer null/alignment checks on raw-pointer-to-reference
  conversions), this panics with "null reference produced"; in an ordinary
  `cargo build --release` (no debug-assertions), the same input is a real
  null-pointer dereference — undefined behavior, not just a panic.
  Reproduce: `cargo fuzz run json_build ../../tests/fuzz-corpus/known-issues/json-build-cff-charset-null-glyf.bin`

- **`tests/fuzz-corpus/known-issues/json-build-advance-width-subtract-overflow.bin`**
  (a `cid-fdselect-test.json`-derived payload with an absurd `advanceWidth`,
  e.g. `50188...` repeated to ~90 digits) — `libcff/charstring_il.rs:405`,
  `glyph_adw_const as c_int - nominal_width as c_int` panics with "attempt
  to subtract with overflow". `glyph_adw_const` is an `f64` from
  attacker-controlled JSON; casting a huge float `as c_int` *saturates* to
  `i32::MAX` (this is correct, checked Rust behavior, not a bug on its own),
  but the subsequent subtraction against `nominal_width` can then itself
  overflow `i32`. This specific panic only fires under debug-assertions (a
  plain `--release` build would silently wrap, producing a nonsensically
  wrapped advance-width delta in the built font instead of crashing) — a
  second, quieter bug hiding behind the loud one.
  Reproduce: `cargo fuzz run json_build ../../tests/fuzz-corpus/known-issues/json-build-advance-width-subtract-overflow.bin`

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

- **`otf_parse` cannot yet run a long, unattended fuzzing campaign**, because
  a short/truncated input very quickly reaches `otfcc_get16u`/`otfcc_get32u`
  (`font/caryll_sfnt.rs:220,239`) calling **`libc::exit(EXIT_FAILURE)`
  directly from library code** on a short read. libFuzzer's contract
  requires the harness function to return, not terminate the process — a
  target that calls `exit()` makes `cargo fuzz run` itself report `ERROR:
  libFuzzer: fuzz target exited` and stop, rather than continuing to fuzz.
  This is precisely the class of bug the Phase 5 plan's Stage 7-3 already
  scoped for removal ("ライブラリ内の `exit()` 4箇所...を修正" —
  `support/alloc.rs`'s two OOM handlers and these two short-read handlers);
  this finding is independent confirmation that fixing it has concrete
  payoff beyond API cleanliness — it's currently a hard ceiling on how much
  of `otf_parse`'s state space a fuzzing campaign can actually reach.

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
