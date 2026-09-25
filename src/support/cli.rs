// Utilities only the two `src/bin/` CLI entry points use -- long-option
// argument parsing and the `--verbose` stopwatch timing log, grouped
// together since neither is part of the library's own font-reading/
// -writing vocabulary.
pub mod getopt;
pub mod stopwatch;
