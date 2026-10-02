// Utilities only the two `src/bin/` CLI entry points use -- long-option
// argument parsing, the `--verbose` stopwatch timing log, and the startup
// steps both binaries share -- grouped together since none of it is part of
// the library's own font-reading/-writing vocabulary.
pub mod getopt;
pub mod stopwatch;

use crate::logger::StageGuard;
use crate::support::options::Options;
use crate::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};
use getopt::GetoptItem;

/// Prints the `--version` line, e.g. "This is Polymorphic otfccdump,
/// version 0.10.4."
pub fn print_version_info(program: &str) {
    println!("This is Polymorphic {program}, version {MAIN_VER}.{SECONDARY_VER}.{PATCH_VER}.");
}

/// Reports a command-line problem `getopt_long` found, in the same words GNU
/// getopt uses. Options themselves (`GetoptItem::Opt`) are the caller's to
/// handle and print nothing here.
pub fn report_getopt_error(program: &str, item: GetoptItem) {
    match item {
        GetoptItem::Opt { .. } => {}
        GetoptItem::UnknownLong(s) => {
            eprintln!("{program}: unrecognized option '{s}'");
        }
        GetoptItem::UnknownShort(ch) => {
            eprintln!("{program}: invalid option -- '{ch}'");
        }
        GetoptItem::AmbiguousLong { given, matches } => {
            let possibilities = matches.iter().map(|m| format!("'--{m}'")).collect::<Vec<_>>().join(" ");
            eprintln!("{program}: option '{given}' is ambiguous; possibilities: {possibilities}");
        }
        GetoptItem::MissingArgument(s) => {
            eprintln!("{program}: option '{s}' requires an argument");
        }
    }
}

/// Starts logging to stderr at the verbosity `--quiet`/`--verbose` asked for
/// (errors only / everything / errors and warnings by default) and opens the
/// root scope every later line is indented under. Keep the returned guard
/// alive for the rest of the run.
pub fn start_logging(program: &'static str, options: &Options) -> StageGuard {
    let verbosity: u8 = if options.quiet {
        0
    } else if options.verbose {
        0xff
    } else {
        1
    };
    crate::logger::install_stderr(verbosity);
    crate::logger::indent(program)
}
