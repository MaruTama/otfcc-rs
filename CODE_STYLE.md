# Code style for readability cleanup

This is the shared guide for the readability work on this repository (see the
"Phase 5" notes in RUST_MIGRATION.md). The code started as a c2rust
transpile of otfcc, so most of what reads badly is the shape of C carried into
Rust. Follow these rules when you touch a file; do not rewrite files only to
apply them.

## Rules for each change

- One theme per PR, one directory at a time. Behavior must not change.
- Every PR keeps `tests/golden` untouched. Never run with `UPDATE_GOLDEN=1`
  in a cleanup PR; a needed update means behavior changed, so stop and find out why.
- Do not rename anything that is exported through the FFI
  (`src/ffi/dll.rs`, `scripts/abi-exports.txt`).
- Do not split a file or function just to lower a line count or a linter score.
  Split only when each part has a name that says what it does.

## Naming

- The module is already the namespace: prefer `index::parse` to
  `libcff::cff_extract_index`. Drop `cff_` / `new_` style C prefixes on
  private and crate-internal items.
- Replace c2rust placeholder names (`in_0`, `x`, `val`) with the name of the thing.
- Keep names from the OpenType / CFF specifications (`charstring`, `FDSelect`,
  `Private DICT`); readers search for them.

## Rust over C shapes

- Output parameters (`&mut T` filled in by the callee) become return values.
- `*_init` / `*_dispose` / `new_empty_*` become `Default` and `Drop`.
- Omit the trailing `return` at the end of a function.
- Use `?` and small helper functions instead of labelled blocks around `Option<()>`.
- Read bytes through `FontReader`, not new local `u16`/`u32` helpers.

## Comments

- A comment says why, or states a constraint the code cannot show
  (for example "a corrupt INDEX is treated as empty").
- Do not narrate history ("this used to be ...", "the plan calls out ...",
  stage numbers). That belongs in the commit message or RUST_MIGRATION.md.
- Do not restate the next line of code.
- Never delete a comment that explains behavior kept for compatibility with
  otfcc; reword it if needed.

## Errors and casts

- Outside tests, no bare `unwrap()`. Use `expect("reason")` for invariants
  and `Result`/`Option` for input from a font file.
- Data from a font file is untrusted: keep the existing bounds checks and
  wrapping arithmetic.
- Do not turn a lossy `as` cast into `try_from` without confirming whether the
  truncation or wrap is relied on; that is a behavior change, not a cleanup.

## Checking a change

```
cargo build --release -p otfcc_rust
cargo test --workspace --release   # includes tests/golden checksums
cargo clippy --workspace --all-targets -- -D warnings
```

`jevlint --ast-only` (local, no API key) can point at long functions and
`unwrap`. Its model-based checks send code to an external service and need
the owner's approval first.
