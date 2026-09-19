#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see RUST_MIGRATION.md
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[allow(unused_imports)]
use ::otfcc_rust;

use libc::{fileno, fprintf, isatty};
use otfcc_rust::support::stdio::{stderr, stdout};

use otfcc_rust::logger::{
    LoggerType, logger_finish, logger_indent_sds, logger_log_sds, logger_set_verbosity,
    logger_start_sds,
};

use otfcc_rust::support::options::Options;

use otfcc_rust::font::caryll_font::Font;
use otfcc_rust::font::caryll_sfnt::SplineFontContainer;
use otfcc_rust::logger::{LOG_VL_CRITICAL, LOG_VL_PROGRESS};
use otfcc_rust::support::built_json::BuiltValue;
use otfcc_rust::support::EXIT_FAILURE;

use libc::timespec;
use otfcc_rust::consolidate::otfcc_consolidate_font;
use otfcc_rust::font::caryll_sfnt::otfcc_read_sfnt;
use otfcc_rust::json_writer::serialize_to_json;
use otfcc_rust::logger::{Logger, otfcc_new_std_err_target};
use otfcc_rust::otf_reader::read_otf;
use otfcc_rust::support::built_json::json_serialize_ex;
use otfcc_rust::support::built_json::{
    JSON_SERIALIZE_MODE_MULTILINE, JSON_SERIALIZE_MODE_PACKED, JsonSerializeOpts,
};
use otfcc_rust::support::getopt::{GetoptItem, LongOpt, getopt_long};
use otfcc_rust::support::strtol::strtol;
use otfcc_rust::support::stopwatch::{push_stopwatch, time_now};
use otfcc_rust::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};
use std::cell::RefCell;
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;

// `fprintf(stdout, ...)` -> `print!` -- both of these were pure fixed
// text (the only variadic args are plain integers substituted by
// value, not by reference or pointer), so there was never a genuine
// unsafe operation here, just the c2rust libc-call idiom. `stdout`
// itself stays imported -- it's still needed by the `isatty(fileno(
// stdout))` check elsewhere in this file.
pub fn printInfo() {
    println!(
        "This is Polymorphic otfccdump, version {}.{}.{}.",
        MAIN_VER, SECONDARY_VER, PATCH_VER,
    );
}
pub fn printHelp() {
    print!(
        "\nUsage : otfccdump [OPTIONS] input.[otf|ttf|ttc]\n\n -h, --help              : Display this help message and exit.\n -v, --version           : Display version information and exit.\n -o <file>               : Set output file path to <file>. When absent the dump\n                           will be written to STDOUT.\n -n <n>, --ttc-index <n> : Use the <n>th subfont within the input font.\n --pretty                : Prettify the output JSON.\n --ugly                  : Force uglify the output JSON.\n --verbose               : Show more information when building.\n -q, --quiet             : Be silent when building.\n\n --ignore-glyph-order    : Do not export glyph order information.\n --glyph-name-prefix pfx : Add a prefix to the glyph names.\n --ignore-hints          : Do not export hinting information.\n --decimal-cmap          : Export 'cmap' keys as decimal number.\n --hex-cmap              : Export 'cmap' keys as hex number (U+FFFF).\n --name-by-hash          : Name glyphs using its hash value.\n --name-by-gid           : Name glyphs using its glyph id.\n --add-bom               : Add BOM mark in the output. (It is default on Windows\n                           when redirecting to another program. Use --no-bom to\n                           turn it off.)\n\n"
    );
}
unsafe fn main_0(args: Vec<String>) -> i32 {
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
    options.logger = RefCell::new(Logger::new(otfcc_new_std_err_target()));
    logger_indent_sds(&mut *options.logger.borrow_mut(), b"otfccdump".to_vec());
    options.decimal_cmap = true;
    let mut outputPath: Option<::std::ffi::CString> = None;
    // Placeholder, unconditionally overwritten below before any real use
    // (the only path that skips the assignment calls `exit()`).
    let mut inPath: ::std::ffi::CString = ::std::ffi::CString::default();
    let (items, positionals) = getopt_long(&args, "vhqpio:n:", LONGOPTS);
    for item in items {
        match item {
            GetoptItem::Opt { val, arg } => match val {
                OPT_VERSION => show_version = true,
                OPT_HELP => show_help = true,
                OPT_PRETTY => show_pretty = true,
                OPT_IGNORE_GLYPH_ORDER => options.ignore_glyph_order = true,
                OPT_OUTPUT => {
                    outputPath = Some(
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
            GetoptItem::UnknownLong(s) => {
                let c = ::std::ffi::CString::new(format!("otfccdump: unrecognized option '{s}'\n"))
                    .unwrap();
                fprintf(stderr, b"%s\0" as *const u8 as *const ::core::ffi::c_char, c.as_ptr());
            }
            GetoptItem::UnknownShort(ch) => {
                let c = ::std::ffi::CString::new(format!("otfccdump: invalid option -- '{ch}'\n"))
                    .unwrap();
                fprintf(stderr, b"%s\0" as *const u8 as *const ::core::ffi::c_char, c.as_ptr());
            }
            GetoptItem::AmbiguousLong { given, matches } => {
                let possibilities =
                    matches.iter().map(|m| format!("'--{m}'")).collect::<Vec<_>>().join(" ");
                let c = ::std::ffi::CString::new(format!(
                    "otfccdump: option '{given}' is ambiguous; possibilities: {possibilities}\n"
                ))
                .unwrap();
                fprintf(stderr, b"%s\0" as *const u8 as *const ::core::ffi::c_char, c.as_ptr());
            }
            GetoptItem::MissingArgument(s) => {
                let c = ::std::ffi::CString::new(format!(
                    "otfccdump: option '{s}' requires an argument\n"
                ))
                .unwrap();
                fprintf(stderr, b"%s\0" as *const u8 as *const ::core::ffi::c_char, c.as_ptr());
            }
        }
    }
    if options.debug_wait_on_start {
        // `--debug-wait-on-start` blocks until the user presses a key, so a
        // debugger can attach. Was a `getchar()` shim kept for the C name's
        // sake; the return value was already discarded, and so is a read
        // error (EOF under a pipe means "do not wait", same as before).
        let _ = std::io::stdin().read(&mut [0u8; 1]);
    }
    logger_set_verbosity(
        &mut *options.logger.borrow_mut(),
        (if options.quiet as i32 != 0 {
            0_i32
        } else if options.verbose as i32 != 0 {
            0xff_i32
        } else {
            1_i32
        }) as u8,
    );
    if show_help {
        printInfo();
        printHelp();
        return 0_i32;
    }
    if show_version {
        printInfo();
        return 0_i32;
    }
    if let Some(p) = positionals.into_iter().next() {
        inPath =
            ::std::ffi::CString::new(p).expect("input path must not contain a NUL byte");
    } else {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_CRITICAL,
            LoggerType::Error,
            otfcc_rust::bytesbuild!(b"Expected argument for input file name.\n"),
        );
        printHelp();
        return EXIT_FAILURE;
    }
    let mut begin: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    time_now(&raw mut begin);
    let mut sfnt: Option<SplineFontContainer> = None;
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Read SFNT"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            otfcc_rust::bytesbuild!(b"From file ", inPath.as_bytes()),
        );
        sfnt = otfcc_read_sfnt(std::path::Path::new(std::ffi::OsStr::from_bytes(inPath.as_bytes())));
        if sfnt.as_ref().is_none_or(|s| s.count == 0_u32) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_CRITICAL,
                LoggerType::Error,
                otfcc_rust::bytesbuild!(
                    b"Cannot read SFNT file \"",
                    inPath.as_bytes(),
                    b"\". Exit.\n",
                ),
            );
            return EXIT_FAILURE;
        }
        let subfonts = sfnt.as_ref().unwrap().count;
        if ttcindex >= subfonts {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_CRITICAL,
                LoggerType::Error,
                otfcc_rust::bytesbuild!(
                    b"Subfont index ",
                    ttcindex,
                    b" out of range for \"",
                    inPath.as_bytes(),
                    b"\" (0 -- ",
                    subfonts.wrapping_sub(1_u32),
                    b"). Exit.\n",
                ),
            );
            return EXIT_FAILURE;
        }
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    let mut font: Option<Box<Font>> = None;
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Read Font"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        font = read_otf(sfnt.as_ref().unwrap(), ttcindex, &*options);
        if font.is_none() {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_CRITICAL,
                LoggerType::Error,
                otfcc_rust::bytesbuild!(
                    b"Font structure broken or corrupted \"",
                    inPath.as_bytes(),
                    b"\". Exit.\n",
                ),
            );
            return EXIT_FAILURE;
        }
        drop(sfnt.take());
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_0 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Consolidate"),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        otfcc_consolidate_font(font.as_mut().unwrap(), &*options);
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_1 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    // Owned now that `serialize_to_json` returns the `BuiltValue` itself
    // rather than a `BuiltValue::into_raw` pointer; `Option` only because
    // the goto-emulating `while ___loggedstep_v` block below is what
    // assigns it.
    let mut root: Option<BuiltValue> = None;
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Dump"),
    );
    let mut ___loggedstep_v_2: bool = true;
    while ___loggedstep_v_2 {
        // The "dump returned null" error path that used to sit here was
        // already dead: the serializer's every exit built a real
        // `BuiltValue`, so the pointer it handed back was never null. With
        // an owned return there is no null to test for at all.
        root = Some(serialize_to_json(font.as_mut().unwrap(), &*options));
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_2 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    let mut buf: Vec<u8> = Vec::new();
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Serialize to JSON"),
    );
    let mut ___loggedstep_v_3: bool = true;
    while ___loggedstep_v_3 {
        let mut jsonOptions: JsonSerializeOpts = JsonSerializeOpts {
            mode: 0,
            opts: 0,
            indent_size: 0,
        };
        jsonOptions.mode = JSON_SERIALIZE_MODE_PACKED;
        jsonOptions.opts = 0_i32;
        jsonOptions.indent_size = 4_i32;
        if show_pretty as i32 != 0
            || outputPath.is_none() && isatty(fileno(stdout)) != 0
        {
            jsonOptions.mode = JSON_SERIALIZE_MODE_MULTILINE;
        }
        if show_ugly {
            jsonOptions.mode = JSON_SERIALIZE_MODE_PACKED;
        }
        buf = json_serialize_ex(
            root.as_ref().expect("the Dump step above always assigns root"),
            jsonOptions,
        );
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_3 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Output"),
    );
    let mut ___loggedstep_v_4: bool = true;
    while ___loggedstep_v_4 {
        if let Some(ref output_path) = outputPath {
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
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_CRITICAL,
                    LoggerType::Error,
                    otfcc_rust::bytesbuild!(
                        b"Cannot write to file \"",
                        output_path.as_bytes(),
                        b"\". Exit.",
                    ),
                );
                return EXIT_FAILURE;
            }
        } else {
            let mut stdout_handle = std::io::stdout();
            if add_bom {
                let _ = stdout_handle.write_all(&[0xef, 0xbb, 0xbf]);
            }
            let _ = stdout_handle.write_all(&buf);
        }
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_4 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Finalize"),
    );
    let mut ___loggedstep_v_5: bool = true;
    while ___loggedstep_v_5 {
        drop(font.take());
        drop(root.take());
        // `inPath`/`outputPath` are `CString`/`Option<CString>` now --
        // both drop on their own at the end of this function's scope, no
        // explicit free needed.
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_5 = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    return 0_i32;
}
pub fn main() -> ::std::process::ExitCode {
    let args: Vec<String> = ::std::env::args().skip(1).collect();
    unsafe { ::std::process::ExitCode::from(main_0(args) as u8) }
}
