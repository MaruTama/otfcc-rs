# Retired: the c2rust transpile pipeline, and everything that needed `c/`

Nothing in this directory is part of the build or of CI, and **none of it
should be run without extra setup first.** It is kept only as an audit
trail, for two unrelated reasons that happen to share a folder:

## The c2rust transpile pipeline

`Dockerfile`, `transpile.sh`, `fix-transmute-abi.py`, `fix-float-narrowing.py`
produced the initial Rust source in Phase 1 (see
[issue #2](https://github.com/MaruTama/otfcc-rs/issues/2)). Two reasons this
part can't be used any more:

- `transpile.sh` does `rm -rf rust/src` and copies fresh c2rust output over
  it. Since Phase 2 that discards every hand-idiomatized file.
- Phase 3 moved the crate to the standard cargo layout
  (`src/lib.rs`, `src/bin/`, `src/support/…`). c2rust emits its own
  `src/lib/…` / `src/dep/extern/…` / `src/src/…` tree, which no longer maps
  onto this crate at all.

A C-side change is now ported to the Rust side by hand, mirroring whatever
the equivalent C diff does.

## Everything that needs `c/`

`compare-with-c.sh`, `gen-compile-commands.sh`, and `filter-compdb.js` moved
here when `c/` (the original C sources this crate was transpiled from) was
deleted from the repo — see the "Next steps" entry in `rust/README.md` for
when and why. `c/` was kept side by side with `rust/` for exactly as long as
it served as the byte-comparison oracle; once `tests/golden/` (dump/build
output) and `tests/golden/log/` (stderr log output) were both frozen from a
confirmed-matching build, nothing in the build or in CI needed it checked
out any more.

These three scripts still work, but only after restoring `c/` from git
history first — e.g. `git show <pre-deletion-commit>:c` (checked out to the
repo root as `c/`) or checking out a tag/commit that predates the deletion.
`compare-with-c.sh` needs `dll-arch-check.sh` and `test-dll.py` too, which
did **not** move: both stayed in `rust/scripts/` because
`compare-with-golden.sh` and `run-cycles.sh` still use them for the
Rust-only cdylib arch check, independent of C.

## Why keep it

These files are the documentation of record for why parts of the crate look
the way they do — the kind of thing that gets "cleaned up" by someone who
doesn't know the history:

- `fix-float-narrowing.py` lists the exact `bufwriteNNb()` call sites where a
  deliberate `EXPR as int16_t as uint16_t` double cast is required. Dropping
  the intermediate signed cast silently zeroes negative `hmtx.lsb` /
  `vmtx.tsb` / `VORG.defaultVerticalOrigin`, because Rust's float→unsigned
  conversion saturates where C's reinterprets the bits.
- `fix-transmute-abi.py` documents a c2rust bug that corrupted every
  struct-by-value return through a function-pointer field.
- `transpile.sh` records the `kPow10` bounds clamp in the vendored dtoa (a
  latent OOB read in the C source that Rust's bounds checks turn into a
  panic) and why the compile database has to live on the bind mount.
- `Dockerfile` records the toolchain that actually worked: native arm64,
  Ubuntu 24.04 / clang-17, c2rust 0.22.1 — each of those pinned for a
  specific failure documented in its header comments.

`gen-compile-commands.sh` and `filter-compdb.js` were the transpiler's input
and originally stayed outside this directory on the reasoning that a
`compile_commands.json` is still useful on its own for pointing clangd at
the C sources — but that reasoning depended on `c/` being present, so once
it was deleted these two joined `compare-with-c.sh` here instead (see
"Everything that needs `c/`" above).
