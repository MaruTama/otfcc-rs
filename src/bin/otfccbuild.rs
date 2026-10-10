use otfcc_rust::logger::ByteStr;
use otfcc_binary::Buffer;
use otfcc_rust::support::options::Options;

use otfcc_rust::consolidate::consolidate_font;
use otfcc_rust::font::model::Font;
use otfcc_rust::json_reader::read_json;
use otfcc_rust::otf_writer::serialize_to_otf;
use otfcc_rust::support::cli::getopt::{GetoptItem, LongOpt, getopt_long};
use otfcc_rust::support::cli::{print_version_info, report_getopt_error, start_logging};
use otfcc_rust::support::options::options_optimize_to;
use otfcc_json::ParsedValue;
use otfcc_json::parse_json;
use otfcc_rust::support::cli::stopwatch::log_step_time;
use otfcc_rust::support::EXIT_FAILURE;
use std::io::Read;
use std::os::unix::ffi::OsStrExt;

pub fn print_help() {
    print!(
        "\nUsage : otfccbuild [OPTIONS] [input.json] -o output.[ttf|otf]\n\n input.json                : Path to input file. When absent the input will be\n                             read from the STDIN.\n\n -h, --help                : Display this help message and exit.\n -v, --version             : Display version information and exit.\n -o <file>                 : Set output file path to <file>.\n -s, --dummy-dsig          : Include an empty DSIG table in the font. For some\n                             Microsoft applications, DSIG is required to enable\n                             OpenType features.\n -O<n>                     : Specify the level for optimization.\n     -O0                     Turn off any optimization.\n     -O1                     Default optimization.\n     -O2                     More aggressive optimizations for web font. In this\n                             level, the following options will be set:\n                               --merge-features\n                               --short-post\n                               --subroutinize\n     -O3                     Most aggressive opptimization strategy will be\n                             used. In this level, these options will be set:\n                               --force-cid\n                               --ignore-glyph-order\n --verbose                 : Show more information when building.\n -q, --quiet               : Be silent when building.\n\n --ignore-hints            : Ignore the hinting information in the input.\n --keep-average-char-width : Keep the OS/2.xAvgCharWidth value from the input\n                             instead of stating the average width of glyphs.\n                             Useful when creating a monospaced font.\n --keep-unicode-ranges     : Keep the OS/2.ulUnicodeRange[1-4] as-is.\n --keep-modified-time      : Keep the head.modified time in the json, instead of\n                             using current time.\n\n --short-post              : Don't export glyph names in the result font.\n --ignore-glyph-order, -i  : Ignore the glyph order information in the input.\n --keep-glyph-order, -k    : Keep the glyph order information in the input.\n                             Use to preserve glyph order under -O2 and -O3.\n --dont-ignore-glyph-order : Same as --keep-glyph-order.\n --merge-features          : Merge duplicate OpenType feature definitions.\n --dont-merge-features     : Keep duplicate OpenType feature definitions.\n --merge-lookups           : Merge duplicate OpenType lookups.\n --dont-merge-lookups      : Keep duplicate OpenType lookups.\n --force-cid               : Convert name-keyed CFF OTF into CID-keyed.\n --subroutinize            : Subroutinize CFF table.\n --stub-cmap4              : Create a stub `cmap` format 4 subtable if format\n                             12 subtable is present.\n\n"
    );
}
// Reads the whole input file; `None` if it cannot be opened or read.
pub fn read_entire_file(in_path: &::core::ffi::CStr) -> Option<Vec<u8>> {
    let os_path = std::ffi::OsStr::from_bytes(in_path.to_bytes());
    let Ok(bytes) = std::fs::read(std::path::Path::new(os_path)) else {
        // Written as raw bytes, not through `eprint!`/`format!`: `in_path` is
        // an OS path and need not be UTF-8, which a `str` formatter would
        // either reject or mangle into U+FFFD. `%s` printed the bytes as-is.
        use std::io::Write;
        let mut msg = b"Cannot read JSON file \"".to_vec();
        msg.extend_from_slice(in_path.to_bytes());
        msg.extend_from_slice(b"\". Exit.\n");
        let _ = std::io::stderr().write_all(&msg);
        return None;
    };
    Some(bytes)
}
// Reads all of stdin, embedded NUL bytes included.
pub fn read_entire_stdin() -> Vec<u8> {
    let mut bytes = Vec::new();
    let _ = std::io::stdin().lock().read_to_end(&mut bytes);
    bytes
}
fn run(args: Vec<String>) -> i32 {
    let mut begin = std::time::Instant::now();
    let mut show_help: bool = false;
    let mut show_version: bool = false;
    let mut invalid_argument = false;
    let mut output_path: Option<::std::ffi::CString> = None;
    let mut options: Box<Options> = Box::default();
    options_optimize_to(&mut options, 1_u8);
    const OPT_VERSION: i32 = 'v' as i32;
    const OPT_HELP: i32 = 'h' as i32;
    // `--keep-glyph-order` and `--dont-ignore-glyph-order` are synonyms, so
    // both long options share this value.
    const OPT_KEEP_GLYPH_ORDER: i32 = 'k' as i32;
    const OPT_IGNORE_GLYPH_ORDER: i32 = 'i' as i32;
    const OPT_OUTPUT: i32 = 'o' as i32;
    const OPT_DUMMY_DSIG: i32 = 's' as i32;
    const OPT_QUIET: i32 = 'q' as i32;
    const OPT_OPTIMIZE: i32 = 'O' as i32;
    const OPT_TIME: i32 = 256;
    const OPT_IGNORE_HINTS: i32 = 257;
    const OPT_KEEP_AVERAGE_CHAR_WIDTH: i32 = 258;
    const OPT_KEEP_UNICODE_RANGES: i32 = 259;
    const OPT_KEEP_MODIFIED_TIME: i32 = 260;
    const OPT_MERGE_LOOKUPS: i32 = 261;
    const OPT_MERGE_FEATURES: i32 = 262;
    const OPT_DONT_MERGE_LOOKUPS: i32 = 263;
    const OPT_DONT_MERGE_FEATURES: i32 = 264;
    const OPT_SHORT_POST: i32 = 265;
    const OPT_FORCE_CID: i32 = 266;
    const OPT_SUBROUTINIZE: i32 = 267;
    const OPT_STUB_CMAP4: i32 = 268;
    const OPT_SHIP: i32 = 269;
    const OPT_VERBOSE: i32 = 270;
    const LONGOPTS: &[LongOpt] = &[
        LongOpt { name: "version", has_arg: false, val: OPT_VERSION },
        LongOpt { name: "help", has_arg: false, val: OPT_HELP },
        LongOpt { name: "time", has_arg: false, val: OPT_TIME },
        LongOpt { name: "ignore-glyph-order", has_arg: false, val: OPT_IGNORE_GLYPH_ORDER },
        LongOpt { name: "keep-glyph-order", has_arg: false, val: OPT_KEEP_GLYPH_ORDER },
        LongOpt { name: "dont-ignore-glyph-order", has_arg: false, val: OPT_KEEP_GLYPH_ORDER },
        LongOpt { name: "ignore-hints", has_arg: false, val: OPT_IGNORE_HINTS },
        LongOpt {
            name: "keep-average-char-width",
            has_arg: false,
            val: OPT_KEEP_AVERAGE_CHAR_WIDTH,
        },
        LongOpt { name: "keep-unicode-ranges", has_arg: false, val: OPT_KEEP_UNICODE_RANGES },
        LongOpt { name: "keep-modified-time", has_arg: false, val: OPT_KEEP_MODIFIED_TIME },
        LongOpt { name: "merge-lookups", has_arg: false, val: OPT_MERGE_LOOKUPS },
        LongOpt { name: "merge-features", has_arg: false, val: OPT_MERGE_FEATURES },
        LongOpt { name: "dont-merge-lookups", has_arg: false, val: OPT_DONT_MERGE_LOOKUPS },
        LongOpt { name: "dont-merge-features", has_arg: false, val: OPT_DONT_MERGE_FEATURES },
        LongOpt { name: "short-post", has_arg: false, val: OPT_SHORT_POST },
        LongOpt { name: "force-cid", has_arg: false, val: OPT_FORCE_CID },
        LongOpt { name: "subroutinize", has_arg: false, val: OPT_SUBROUTINIZE },
        LongOpt { name: "stub-cmap4", has_arg: false, val: OPT_STUB_CMAP4 },
        LongOpt { name: "dummy-dsig", has_arg: false, val: OPT_DUMMY_DSIG },
        LongOpt { name: "ship", has_arg: false, val: OPT_SHIP },
        LongOpt { name: "verbose", has_arg: false, val: OPT_VERBOSE },
        LongOpt { name: "quiet", has_arg: false, val: OPT_QUIET },
        LongOpt { name: "optimize", has_arg: true, val: OPT_OPTIMIZE },
        LongOpt { name: "output", has_arg: true, val: OPT_OUTPUT },
    ];
    let (items, positionals) = getopt_long(&args, "vhqskiO:o:", LONGOPTS);
    for item in items {
        match item {
            GetoptItem::Opt { val, arg } => match val {
                OPT_VERSION => show_version = true,
                OPT_HELP => show_help = true,
                OPT_KEEP_GLYPH_ORDER => options.ignore_glyph_order = false,
                OPT_IGNORE_GLYPH_ORDER => options.ignore_glyph_order = true,
                OPT_OUTPUT => {
                    output_path = Some(
                        ::std::ffi::CString::new(arg.unwrap())
                            .expect("output path must not contain a NUL byte"),
                    );
                }
                OPT_DUMMY_DSIG => options.dummy_dsig = true,
                OPT_QUIET => options.quiet = true,
                OPT_OPTIMIZE => {
                    let arg = arg.unwrap();
                    match arg.parse::<u8>() {
                        Ok(level) => options_optimize_to(&mut options, level),
                        Err(_) => {
                            eprintln!("otfccbuild: invalid optimization level '{arg}'");
                            invalid_argument = true;
                        }
                    }
                }
                OPT_TIME => {}
                OPT_IGNORE_HINTS => options.ignore_hints = true,
                OPT_KEEP_AVERAGE_CHAR_WIDTH => options.keep_average_char_width = true,
                OPT_KEEP_UNICODE_RANGES => options.keep_unicode_ranges = true,
                OPT_KEEP_MODIFIED_TIME => options.keep_modified_time = true,
                OPT_MERGE_LOOKUPS => options.merge_lookups = true,
                OPT_MERGE_FEATURES => options.merge_features = true,
                OPT_DONT_MERGE_LOOKUPS => options.merge_lookups = false,
                OPT_DONT_MERGE_FEATURES => options.merge_features = false,
                OPT_SHORT_POST => options.short_post = true,
                OPT_FORCE_CID => options.force_cid = true,
                OPT_SUBROUTINIZE => options.cff_do_subroutinize = true,
                OPT_STUB_CMAP4 => options.stub_cmap4 = true,
                OPT_SHIP => {
                    options.ignore_glyph_order = true;
                    options.short_post = true;
                    options.dummy_dsig = true;
                }
                OPT_VERBOSE => options.verbose = true,
                _ => {}
            },
            other => report_getopt_error("otfccbuild", other),
        }
    }
    // Logging starts only now that `--quiet`/`--verbose` are known; nothing
    // is logged before this point (argument errors go straight to stderr).
    let _root_scope = start_logging("otfccbuild", &options);
    if show_help {
        print_version_info("otfccbuild");
        print_help();
        return 0_i32;
    }
    if show_version {
        print_version_info("otfccbuild");
        return 0_i32;
    }
    if invalid_argument {
        return EXIT_FAILURE;
    }
    let in_path: Option<::std::ffi::CString> = positionals.into_iter().next().map(|p| {
        ::std::ffi::CString::new(p).expect("input path must not contain a NUL byte")
    });
    if output_path.is_none() {
        tracing::error!("Unable to build OpenType font tile : output path not specified. Exit.\n");
        print_help();
        return EXIT_FAILURE;
    }
    let buffer: Vec<u8>;
    let stage = otfcc_rust::logger::stage("Load file");
    {
        if let Some(ref in_path) = in_path {
            let substage = otfcc_rust::logger::stage(format_args!("Load from file {}", ByteStr(in_path.as_bytes())));
            {
                let Some(b) = read_entire_file(in_path.as_c_str()) else {
                    return EXIT_FAILURE;
                };
                buffer = b;
                substage.finish();
            }
        } else {
            let substage = otfcc_rust::logger::stage("Load from stdin");
            {
                buffer = read_entire_stdin();
                substage.finish();
            }
        }
        log_step_time(&mut begin);
        stage.finish();
    }
    let mut json_root: Option<ParsedValue>;
    let stage = otfcc_rust::logger::stage("Parse into JSON");
    {
        json_root = parse_json(&buffer);
        log_step_time(&mut begin);
        if json_root.is_none() {
            tracing::error!("Cannot parse JSON file \"{}\". Exit.\n", ByteStr(in_path.as_deref().map(::std::ffi::CStr::to_bytes).unwrap_or(b"")));
            return EXIT_FAILURE;
        }
        stage.finish();
    }
    let mut font: Option<Box<Font>>;
    let stage = otfcc_rust::logger::stage("Parse");
    {
        font = read_json(json_root.as_mut().unwrap(), &options);
        if font.is_none() {
            tracing::error!("Cannot parse JSON file \"{}\" as a font. Exit.\n", ByteStr(in_path.as_deref().map(::std::ffi::CStr::to_bytes).unwrap_or(b"")));
            return EXIT_FAILURE;
        }
        drop(json_root.take());
        log_step_time(&mut begin);
        stage.finish();
    }
    let stage = otfcc_rust::logger::stage("Consolidate");
    {
        consolidate_font(font.as_mut().unwrap(), &options);
        log_step_time(&mut begin);
        stage.finish();
    }
    let stage = otfcc_rust::logger::stage("Build");
    {
        let otf: Buffer = serialize_to_otf(font.as_mut().unwrap(), &options);
        let substage = otfcc_rust::logger::stage("Write to file");
        {
            // Always `Some` here -- the `output_path.is_none()` branch
            // above already exited.
            let output_path = output_path.as_ref().unwrap();
            let os_path = std::ffi::OsStr::from_bytes(output_path.as_bytes());
            if std::fs::write(std::path::Path::new(os_path), &otf.data).is_err() {
                tracing::error!("Cannot write to file \"{}\". Exit.\n", ByteStr(output_path.as_bytes()));
                return EXIT_FAILURE;
            }
            substage.finish();
        }
        log_step_time(&mut begin);
        drop(font.take());
        // `in_path`/`output_path` are `Option<CString>` now -- both drop on
        // their own at the end of this function's scope, no explicit
        // free needed.
        stage.finish();
    }
    return 0_i32;
}
pub fn main() -> ::std::process::ExitCode {
    let args: Vec<String> = ::std::env::args().skip(1).collect();
    ::std::process::ExitCode::from(run(args) as u8)
}
