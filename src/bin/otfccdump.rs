use otfcc_rust::logger::ByteStr;
use otfcc_rust::support::options::Options;

use otfcc_rust::font::caryll_font::Font;
use otfcc_rust::font::caryll_sfnt::SplineFontContainer;
use otfcc_rust::support::built_json::BuiltValue;
use otfcc_rust::support::EXIT_FAILURE;

use libc::timespec;
use otfcc_rust::consolidate::consolidate_font;
use otfcc_rust::font::caryll_sfnt::read_sfnt;
use otfcc_rust::json_writer::serialize_to_json;
use otfcc_rust::otf_reader::read_otf;
use otfcc_rust::support::built_json::json_serialize_ex;
use otfcc_rust::support::built_json::{
    JSON_SERIALIZE_MODE_MULTILINE, JSON_SERIALIZE_MODE_PACKED, JsonSerializeOpts,
};
use otfcc_rust::support::cli::getopt::{GetoptItem, LongOpt, getopt_long};
use otfcc_rust::support::cli::{print_version_info, report_getopt_error, start_logging};
use otfcc_rust::support::cstd::strtol::strtol;
use otfcc_rust::support::cli::stopwatch::{log_step_time, time_now};
use std::io::{IsTerminal, Read, Write};
use std::os::unix::ffi::OsStrExt;

// `fprintf(stdout, ...)` -> `print!` -- both of these were pure fixed
// text (the only variadic args are plain integers substituted by
// value, not by reference or pointer), so there was never a genuine
// unsafe operation here, just the c2rust libc-call idiom. `stdout`
// itself stays imported -- it's still needed by the `isatty(fileno(
// stdout))` check elsewhere in this file.
pub fn print_help() {
    print!(
        "\nUsage : otfccdump [OPTIONS] input.[otf|ttf|ttc]\n\n -h, --help              : Display this help message and exit.\n -v, --version           : Display version information and exit.\n -o <file>               : Set output file path to <file>. When absent the dump\n                           will be written to STDOUT.\n -n <n>, --ttc-index <n> : Use the <n>th subfont within the input font.\n --pretty                : Prettify the output JSON.\n --ugly                  : Force uglify the output JSON.\n --verbose               : Show more information when building.\n -q, --quiet             : Be silent when building.\n\n --ignore-glyph-order    : Do not export glyph order information.\n --glyph-name-prefix pfx : Add a prefix to the glyph names.\n --ignore-hints          : Do not export hinting information.\n --decimal-cmap          : Export 'cmap' keys as decimal number.\n --hex-cmap              : Export 'cmap' keys as hex number (U+FFFF).\n --name-by-hash          : Name glyphs using its hash value.\n --name-by-gid           : Name glyphs using its glyph id.\n --add-bom               : Add BOM mark in the output. (It is default on Windows\n                           when redirecting to another program. Use --no-bom to\n                           turn it off.)\n\n"
    );
}
fn run(args: Vec<String>) -> i32 {
    let mut show_help: bool = false;
    let mut show_version: bool = false;
    let mut show_pretty: bool = false;
    let mut show_ugly: bool = false;
    let mut add_bom: bool = false;
    let mut _no_bom: bool = false;
    let mut ttcindex: u32 = 0_u32;
    const OPT_VERSION: i32 = 'v' as i32;
    const OPT_HELP: i32 = 'h' as i32;
    const OPT_PRETTY: i32 = 'p' as i32;
    // `-i`'s direct short match arm and `--ignore-glyph-order`'s long entry
    // already set the same field -- redundant paths, not a bug -- so both
    // get the same dispatch value here, same as `--quiet`/`-q` below.
    const OPT_IGNORE_GLYPH_ORDER: i32 = 'i' as i32;
    const OPT_OUTPUT: i32 = 'o' as i32;
    const OPT_QUIET: i32 = 'q' as i32;
    const OPT_TTC_INDEX: i32 = 'n' as i32;
    const OPT_UGLY: i32 = 256;
    const OPT_TIME: i32 = 257;
    const OPT_IGNORE_HINTS: i32 = 258;
    const OPT_HEX_CMAP: i32 = 259;
    const OPT_DECIMAL_CMAP: i32 = 260;
    const OPT_INSTR_AS_BYTES: i32 = 261;
    const OPT_NAME_BY_HASH: i32 = 262;
    const OPT_NAME_BY_GID: i32 = 263;
    const OPT_GLYPH_NAME_PREFIX: i32 = 264;
    const OPT_VERBOSE: i32 = 265;
    const OPT_ADD_BOM: i32 = 266;
    const OPT_NO_BOM: i32 = 267;
    const OPT_DEBUG_WAIT_ON_START: i32 = 268;
    const LONGOPTS: &[LongOpt] = &[
        LongOpt { name: "version", has_arg: false, val: OPT_VERSION },
        LongOpt { name: "help", has_arg: false, val: OPT_HELP },
        LongOpt { name: "pretty", has_arg: false, val: OPT_PRETTY },
        LongOpt { name: "ugly", has_arg: false, val: OPT_UGLY },
        LongOpt { name: "time", has_arg: false, val: OPT_TIME },
        LongOpt { name: "ignore-glyph-order", has_arg: false, val: OPT_IGNORE_GLYPH_ORDER },
        LongOpt { name: "ignore-hints", has_arg: false, val: OPT_IGNORE_HINTS },
        LongOpt { name: "hex-cmap", has_arg: false, val: OPT_HEX_CMAP },
        LongOpt { name: "decimal-cmap", has_arg: false, val: OPT_DECIMAL_CMAP },
        LongOpt { name: "instr-as-bytes", has_arg: false, val: OPT_INSTR_AS_BYTES },
        LongOpt { name: "name-by-hash", has_arg: false, val: OPT_NAME_BY_HASH },
        LongOpt { name: "name-by-gid", has_arg: false, val: OPT_NAME_BY_GID },
        LongOpt { name: "glyph-name-prefix", has_arg: true, val: OPT_GLYPH_NAME_PREFIX },
        LongOpt { name: "verbose", has_arg: false, val: OPT_VERBOSE },
        LongOpt { name: "quiet", has_arg: false, val: OPT_QUIET },
        LongOpt { name: "add-bom", has_arg: false, val: OPT_ADD_BOM },
        LongOpt { name: "no-bom", has_arg: false, val: OPT_NO_BOM },
        LongOpt { name: "output", has_arg: true, val: OPT_OUTPUT },
        LongOpt { name: "ttc-index", has_arg: true, val: OPT_TTC_INDEX },
        LongOpt { name: "debug-wait-on-start", has_arg: false, val: OPT_DEBUG_WAIT_ON_START },
    ];
    let mut options: Box<Options> = Box::default();
    options.decimal_cmap = true;
    let mut output_path: Option<::std::ffi::CString> = None;
    // Assigned once the positional arguments are known, below.
    let in_path: ::std::ffi::CString;
    let (items, positionals) = getopt_long(&args, "vhqpio:n:", LONGOPTS);
    for item in items {
        match item {
            GetoptItem::Opt { val, arg } => match val {
                OPT_VERSION => show_version = true,
                OPT_HELP => show_help = true,
                OPT_PRETTY => show_pretty = true,
                OPT_IGNORE_GLYPH_ORDER => options.ignore_glyph_order = true,
                OPT_OUTPUT => {
                    output_path = Some(
                        ::std::ffi::CString::new(arg.unwrap())
                            .expect("output path must not contain a NUL byte"),
                    );
                }
                OPT_QUIET => options.quiet = true,
                OPT_TTC_INDEX => {
                    ttcindex = strtol(arg.unwrap().as_bytes(), 10) as u32;
                }
                OPT_UGLY => show_ugly = true,
                OPT_TIME => {}
                OPT_ADD_BOM => add_bom = true,
                OPT_NO_BOM => _no_bom = true,
                OPT_VERBOSE => options.verbose = true,
                OPT_IGNORE_HINTS => options.ignore_hints = true,
                OPT_DECIMAL_CMAP => options.decimal_cmap = true,
                OPT_HEX_CMAP => options.decimal_cmap = false,
                OPT_NAME_BY_HASH => options.name_glyphs_by_hash = true,
                OPT_NAME_BY_GID => options.name_glyphs_by_gid = true,
                OPT_INSTR_AS_BYTES => options.instr_as_bytes = true,
                OPT_GLYPH_NAME_PREFIX => {
                    options.glyph_name_prefix = Some(arg.unwrap().into_bytes());
                }
                OPT_DEBUG_WAIT_ON_START => options.debug_wait_on_start = true,
                _ => {}
            },
            other => report_getopt_error("otfccdump", other),
        }
    }
    if options.debug_wait_on_start {
        // `--debug-wait-on-start` blocks until the user presses a key, so a
        // debugger can attach. Was a `getchar()` shim kept for the C name's
        // sake; the return value was already discarded, and so is a read
        // error (EOF under a pipe means "do not wait", same as before).
        let _ = std::io::stdin().read(&mut [0u8; 1]);
    }
    // Logging starts only now that `--quiet`/`--verbose` are known; nothing
    // is logged before this point (argument errors go straight to stderr).
    let _root_scope = start_logging("otfccdump", &options);
    if show_help {
        print_version_info("otfccdump");
        print_help();
        return 0_i32;
    }
    if show_version {
        print_version_info("otfccdump");
        return 0_i32;
    }
    if let Some(p) = positionals.into_iter().next() {
        in_path =
            ::std::ffi::CString::new(p).expect("input path must not contain a NUL byte");
    } else {
        tracing::error!("Expected argument for input file name.\n");
        print_help();
        return EXIT_FAILURE;
    }
    let mut begin: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    time_now(&mut begin);
    let mut sfnt: Option<SplineFontContainer>;
    let stage = otfcc_rust::logger::stage("Read SFNT");
    {
        tracing::debug!("From file {}", ByteStr(in_path.as_bytes()));
        sfnt = read_sfnt(std::path::Path::new(std::ffi::OsStr::from_bytes(in_path.as_bytes())));
        if sfnt.as_ref().is_none_or(|s| s.count == 0_u32) {
            tracing::error!("Cannot read SFNT file \"{}\". Exit.\n", ByteStr(in_path.as_bytes()));
            return EXIT_FAILURE;
        }
        let subfonts = sfnt.as_ref().unwrap().count;
        if ttcindex >= subfonts {
            tracing::error!("Subfont index {} out of range for \"{}\" (0 -- {}). Exit.\n", ttcindex, ByteStr(in_path.as_bytes()), ByteStr(subfonts.wrapping_sub(1_u32)));
            return EXIT_FAILURE;
        }
        log_step_time(&mut begin);
        stage.finish();
    }
    let mut font: Option<Box<Font>>;
    let stage = otfcc_rust::logger::stage("Read Font");
    {
        font = read_otf(sfnt.as_ref().unwrap(), ttcindex, &options);
        if font.is_none() {
            tracing::error!("Font structure broken or corrupted \"{}\". Exit.\n", ByteStr(in_path.as_bytes()));
            return EXIT_FAILURE;
        }
        drop(sfnt.take());
        log_step_time(&mut begin);
        stage.finish();
    }
    let stage = otfcc_rust::logger::stage("Consolidate");
    {
        consolidate_font(font.as_mut().unwrap(), &options);
        log_step_time(&mut begin);
        stage.finish();
    }
    // Owned now that `serialize_to_json` returns the `BuiltValue` itself
    // rather than a `BuiltValue::into_raw` pointer; `Option` only because
    // the plain block below is what assigns it.
    let mut root: Option<BuiltValue>;
    let stage = otfcc_rust::logger::stage("Dump");
    {
        // The "dump returned null" error path that used to sit here was
        // already dead: the serializer's every exit built a real
        // `BuiltValue`, so the pointer it handed back was never null. With
        // an owned return there is no null to test for at all.
        root = Some(serialize_to_json(font.as_mut().unwrap(), &options));
        log_step_time(&mut begin);
        stage.finish();
    }
    let buf: Vec<u8>;
    let stage = otfcc_rust::logger::stage("Serialize to JSON");
    {
        let mut json_options: JsonSerializeOpts = JsonSerializeOpts {
            mode: 0,
            opts: 0,
            indent_size: 0,
        };
        json_options.mode = JSON_SERIALIZE_MODE_PACKED;
        json_options.opts = 0_i32;
        json_options.indent_size = 4_i32;
        if show_pretty as i32 != 0
            || output_path.is_none() && std::io::stdout().is_terminal()
        {
            json_options.mode = JSON_SERIALIZE_MODE_MULTILINE;
        }
        if show_ugly {
            json_options.mode = JSON_SERIALIZE_MODE_PACKED;
        }
        buf = json_serialize_ex(
            root.as_ref().expect("the Dump step above always assigns root"),
            json_options,
        );
        log_step_time(&mut begin);
        stage.finish();
    }
    let stage = otfcc_rust::logger::stage("Output");
    {
        if let Some(ref output_path) = output_path {
            let os_path = std::ffi::OsStr::from_bytes(output_path.as_bytes());
            let write_result = std::fs::File::create(std::path::Path::new(os_path)).and_then(
                |mut f| {
                    if add_bom {
                        f.write_all(&[0xef, 0xbb, 0xbf])?;
                    }
                    f.write_all(&buf)
                },
            );
            if write_result.is_err() {
                tracing::error!("Cannot write to file \"{}\". Exit.", ByteStr(output_path.as_bytes()));
                return EXIT_FAILURE;
            }
        } else {
            let mut stdout_handle = std::io::stdout();
            if add_bom {
                let _ = stdout_handle.write_all(&[0xef, 0xbb, 0xbf]);
            }
            let _ = stdout_handle.write_all(&buf);
        }
        log_step_time(&mut begin);
        stage.finish();
    }
    let stage = otfcc_rust::logger::stage("Finalize");
    {
        drop(font.take());
        drop(root.take());
        // `in_path`/`output_path` are `CString`/`Option<CString>` now --
        // both drop on their own at the end of this function's scope, no
        // explicit free needed.
        log_step_time(&mut begin);
        stage.finish();
    }
    return 0_i32;
}
pub fn main() -> ::std::process::ExitCode {
    let args: Vec<String> = ::std::env::args().skip(1).collect();
    ::std::process::ExitCode::from(run(args) as u8)
}
