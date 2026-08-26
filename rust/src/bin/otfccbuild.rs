#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
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

use libc::{fprintf, free, malloc, strtol};
use otfcc_rust::support::stdio::{stderr, stdout};

use otfcc_rust::logger::{
    LoggerType, logger_finish, logger_indent, logger_log_sds, logger_set_verbosity,
    logger_start_sds,
};
use otfcc_rust::support::buffer::Buffer;
use otfcc_rust::support::options::Options;

use libc::timespec;
use otfcc_rust::consolidate::otfcc_consolidate_font;
use otfcc_rust::font::caryll_font::Font;
use otfcc_rust::font::caryll_font::otfcc_font_free;
use otfcc_rust::json_reader::read_json;
use otfcc_rust::logger::{LOG_VL_CRITICAL, LOG_VL_PROGRESS};
use otfcc_rust::logger::{Logger, otfcc_new_std_err_target};
use otfcc_rust::otf_writer::serialize_to_otf;
use otfcc_rust::support::buffer::buffree;
use otfcc_rust::support::getopt::{GetoptItem, LongOpt, getopt_long};
use otfcc_rust::support::options::{
    otfcc_delete_options, otfcc_new_options, otfcc_options_optimize_to,
};
use otfcc_rust::support::parsed_json::ParsedValue;
use otfcc_rust::support::parsed_json::{json_parse, json_value_free};
use otfcc_rust::support::stopwatch::{push_stopwatch, time_now};
use otfcc_rust::support::{EXIT_FAILURE, NULL};
use otfcc_rust::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};
use std::cell::RefCell;
use std::io::Read;
use std::os::unix::ffi::OsStrExt;

#[inline]
unsafe fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub unsafe fn printInfo() {
    fprintf(
        stdout,
        b"This is Polymorphic otfccbuild, version %d.%d.%d.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        MAIN_VER,
        SECONDARY_VER,
        PATCH_VER,
    );
}
pub unsafe fn printHelp() {
    fprintf(
        stdout,
        b"\nUsage : otfccbuild [OPTIONS] [input.json] -o output.[ttf|otf]\n\n input.json                : Path to input file. When absent the input will be\n                             read from the STDIN.\n\n -h, --help                : Display this help message and exit.\n -v, --version             : Display version information and exit.\n -o <file>                 : Set output file path to <file>.\n -s, --dummy-dsig          : Include an empty DSIG table in the font. For some\n                             Microsoft applications, DSIG is required to enable\n                             OpenType features.\n -O<n>                     : Specify the level for optimization.\n     -O0                     Turn off any optimization.\n     -O1                     Default optimization.\n     -O2                     More aggressive optimizations for web font. In this\n                             level, the following options will be set:\n                               --merge-features\n                               --short-post\n                               --subroutinize\n     -O3                     Most aggressive opptimization strategy will be\n                             used. In this level, these options will be set:\n                               --force-cid\n                               --ignore-glyph-order\n --verbose                 : Show more information when building.\n -q, --quiet               : Be silent when building.\n\n --ignore-hints            : Ignore the hinting information in the input.\n --keep-average-char-width : Keep the OS/2.xAvgCharWidth value from the input\n                             instead of stating the average width of glyphs.\n                             Useful when creating a monospaced font.\n --keep-unicode-ranges     : Keep the OS/2.ulUnicodeRange[1-4] as-is.\n --keep-modified-time      : Keep the head.modified time in the json, instead of\n                             using current time.\n\n --short-post              : Don't export glyph names in the result font.\n --ignore-glyph-order, -i  : Ignore the glyph order information in the input.\n --keep-glyph-order, -k    : Keep the glyph order information in the input.\n                             Use to preserve glyph order under -O2 and -O3.\n --dont-ignore-glyph-order : Same as --keep-glyph-order.\n --merge-features          : Merge duplicate OpenType feature definitions.\n --dont-merge-features     : Keep duplicate OpenType feature definitions.\n --merge-lookups           : Merge duplicate OpenType lookups.\n --dont-merge-lookups      : Keep duplicate OpenType lookups.\n --force-cid               : Convert name-keyed CFF OTF into CID-keyed.\n --subroutinize            : Subroutinize CFF table.\n --stub-cmap4              : Create a stub `cmap` format 4 subtable if format\n                             12 subtable is present.\n\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
}
// `false` means the file couldn't be opened or read -- the caller
// (`main_0`) returns `EXIT_FAILURE` itself instead of this function
// calling `exit()` deep inside a helper, the same "propagate a failure
// signal up to the one place that already owns process-exit semantics"
// shape `font/caryll_sfnt.rs`'s `otfcc_get16u`/`otfcc_get32u` -> `Option`
// conversion used.
//
// The bug this fixes: the old `fseek`/`ftell`/`fread` version discarded
// `fread`'s return value, so a read that returned fewer bytes than
// `length` (a race with concurrent truncation, or any other short read)
// left the malloc'd buffer's tail as uninitialized memory that still got
// treated as `length` valid bytes and fed to `json_parse`. `std::fs::read`
// reads to actual EOF into a `Vec<u8>` whose length is exactly what was
// read, so there is no way for a short read to go unnoticed. `_buffer`
// stays a `malloc`'d `*mut c_char` (the caller's shared `buffer`/`length`
// locals are also written by `readEntireStdin`, unconverted, and both are
// freed uniformly downstream with `free()` and read with `json_parse`) --
// only the reading, not the buffer's ownership shape, changes here.
pub unsafe fn readEntireFile(
    inPath: *mut ::core::ffi::c_char,
    _buffer: *mut *mut ::core::ffi::c_char,
    _length: *mut ::core::ffi::c_long,
) -> bool {
    let path_bytes = unsafe { ::core::ffi::CStr::from_ptr(inPath) }.to_bytes();
    let os_path = std::ffi::OsStr::from_bytes(path_bytes);
    let Ok(bytes) = std::fs::read(std::path::Path::new(os_path)) else {
        fprintf(
            stderr,
            b"Cannot read JSON file \"%s\". Exit.\n\0" as *const u8 as *const ::core::ffi::c_char,
            inPath,
        );
        return false;
    };
    let buffer = malloc(bytes.len()) as *mut ::core::ffi::c_char;
    if buffer.is_null() {
        fprintf(
            stderr,
            b"Cannot read JSON file \"%s\". Exit.\n\0" as *const u8 as *const ::core::ffi::c_char,
            inPath,
        );
        return false;
    }
    unsafe {
        ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, bytes.len());
    }
    *_buffer = buffer;
    *_length = bytes.len() as ::core::ffi::c_long;
    true
}
// The old `fgets`/`strlen` loop measured each chunk it read with `strlen`,
// which stops at the first embedded NUL byte -- any stdin content after an
// embedded NUL silently vanished from `length` (and thus from the JSON
// text handed to `json_parse`) instead of erroring or being kept.
// `Read::read_to_end` copies exactly the bytes it receives with no such
// assumption, closing that class of bug structurally, the same way
// `readEntireFile`'s `std::fs::read` closed the short-read class of bug.
pub unsafe fn readEntireStdin(
    _buffer: *mut *mut ::core::ffi::c_char,
    _length: *mut ::core::ffi::c_long,
) {
    let mut bytes = Vec::new();
    let _ = std::io::stdin().lock().read_to_end(&mut bytes);
    let buffer = malloc(bytes.len().max(1)) as *mut ::core::ffi::c_char;
    unsafe {
        ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, bytes.len());
    }
    *_buffer = buffer;
    *_length = bytes.len() as ::core::ffi::c_long;
}
unsafe fn main_0(args: Vec<String>) -> ::core::ffi::c_int {
    let mut begin: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    time_now(&raw mut begin);
    let mut show_help: bool = false;
    let mut show_version: bool = false;
    let mut outputPath: Option<::std::ffi::CString> = None;
    let mut inPath: Option<::std::ffi::CString> = None;
    let mut options: *mut Options = otfcc_new_options();
    (*options).logger = RefCell::new(Logger::new(otfcc_new_std_err_target()));
    logger_indent(
        &mut *(*options).logger.borrow_mut(),
        b"otfccbuild\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_options_optimize_to(options, 1 as u8);
    const OPT_VERSION: i32 = 'v' as i32;
    const OPT_HELP: i32 = 'h' as i32;
    // `--keep-glyph-order` and `--dont-ignore-glyph-order` are documented as
    // synonyms (see `printHelp` above) and always had identical intended
    // effect. The old c2rust match block checked the long option's name via
    // `strcmp(..., "dont-keep-glyph-order")` -- a string that was never
    // actually registered in `longopts` (which spelled it
    // `dont-ignore-glyph-order`) -- so `--dont-ignore-glyph-order` silently
    // no-op'd instead of clearing `ignore_glyph_order`. Giving both entries
    // the same dispatch value fixes that bug structurally: there is no
    // string to typo anymore.
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
                OPT_KEEP_GLYPH_ORDER => (*options).ignore_glyph_order = false,
                OPT_IGNORE_GLYPH_ORDER => (*options).ignore_glyph_order = true,
                OPT_OUTPUT => {
                    outputPath = Some(
                        ::std::ffi::CString::new(arg.unwrap())
                            .expect("output path must not contain a NUL byte"),
                    );
                }
                OPT_DUMMY_DSIG => (*options).dummy_dsig = true,
                OPT_QUIET => (*options).quiet = true,
                OPT_OPTIMIZE => {
                    let carg = ::std::ffi::CString::new(arg.unwrap())
                        .expect("optimize level must not contain a NUL byte");
                    otfcc_options_optimize_to(options, atoi(carg.as_ptr()) as u8);
                }
                OPT_TIME => {}
                OPT_IGNORE_HINTS => (*options).ignore_hints = true,
                OPT_KEEP_AVERAGE_CHAR_WIDTH => (*options).keep_average_char_width = true,
                OPT_KEEP_UNICODE_RANGES => (*options).keep_unicode_ranges = true,
                OPT_KEEP_MODIFIED_TIME => (*options).keep_modified_time = true,
                OPT_MERGE_LOOKUPS => (*options).merge_lookups = true,
                OPT_MERGE_FEATURES => (*options).merge_features = true,
                OPT_DONT_MERGE_LOOKUPS => (*options).merge_lookups = false,
                OPT_DONT_MERGE_FEATURES => (*options).merge_features = false,
                OPT_SHORT_POST => (*options).short_post = true,
                OPT_FORCE_CID => (*options).force_cid = true,
                OPT_SUBROUTINIZE => (*options).cff_do_subroutinize = true,
                OPT_STUB_CMAP4 => (*options).stub_cmap4 = true,
                OPT_SHIP => {
                    (*options).ignore_glyph_order = true;
                    (*options).short_post = true;
                    (*options).dummy_dsig = true;
                }
                OPT_VERBOSE => (*options).verbose = true,
                _ => {}
            },
            GetoptItem::UnknownLong(s) => {
                let c = ::std::ffi::CString::new(format!("otfccbuild: unrecognized option '{s}'\n"))
                    .unwrap();
                fprintf(stderr, b"%s\0" as *const u8 as *const ::core::ffi::c_char, c.as_ptr());
            }
            GetoptItem::UnknownShort(ch) => {
                let c = ::std::ffi::CString::new(format!("otfccbuild: invalid option -- '{ch}'\n"))
                    .unwrap();
                fprintf(stderr, b"%s\0" as *const u8 as *const ::core::ffi::c_char, c.as_ptr());
            }
            GetoptItem::AmbiguousLong { given, matches } => {
                let possibilities =
                    matches.iter().map(|m| format!("'--{m}'")).collect::<Vec<_>>().join(" ");
                let c = ::std::ffi::CString::new(format!(
                    "otfccbuild: option '{given}' is ambiguous; possibilities: {possibilities}\n"
                ))
                .unwrap();
                fprintf(stderr, b"%s\0" as *const u8 as *const ::core::ffi::c_char, c.as_ptr());
            }
            GetoptItem::MissingArgument(s) => {
                let c = ::std::ffi::CString::new(format!(
                    "otfccbuild: option '{s}' requires an argument\n"
                ))
                .unwrap();
                fprintf(stderr, b"%s\0" as *const u8 as *const ::core::ffi::c_char, c.as_ptr());
            }
        }
    }
    logger_set_verbosity(
        &mut *(*options).logger.borrow_mut(),
        (if (*options).quiet as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else if (*options).verbose as ::core::ffi::c_int != 0 {
            0xff as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as u8,
    );
    if show_help {
        printInfo();
        printHelp();
        return 0 as ::core::ffi::c_int;
    }
    if show_version {
        printInfo();
        return 0 as ::core::ffi::c_int;
    }
    inPath = positionals.into_iter().next().map(|p| {
        ::std::ffi::CString::new(p).expect("input path must not contain a NUL byte")
    });
    if outputPath.is_none() {
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_CRITICAL,
            LoggerType::Error,
            otfcc_rust::bytesbuild!(
                b"Unable to build OpenType font tile : output path not specified. Exit.\n",
            ),
        );
        printHelp();
        return EXIT_FAILURE;
    }
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut length: ::core::ffi::c_long = 0;
    logger_start_sds(
        &mut *(*options).logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Load file"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        if let Some(ref in_path) = inPath {
            logger_start_sds(
                &mut *(*options).logger.borrow_mut(),
                otfcc_rust::bytesbuild!(b"Load from file ", in_path.as_bytes()),
            );
            let mut ___loggedstep_v_0: bool = true;
            while ___loggedstep_v_0 {
                if !readEntireFile(
                    in_path.as_ptr() as *mut ::core::ffi::c_char,
                    &raw mut buffer,
                    &raw mut length,
                ) {
                    return EXIT_FAILURE;
                }
                // No longer freed here (was: `sdsfree(inPath)`) -- doing
                // so used to leave a dangling pointer that the two later
                // "Cannot parse JSON file" error messages below still
                // read from (`bytesbuild!(..., inPath, ...)`), a genuine
                // pre-existing use-after-free. `inPath` now just lives
                // for the rest of the function and drops naturally at
                // the end, which is exactly what those later reads
                // needed all along.
                ___loggedstep_v_0 = false;
                logger_finish(&mut *(*options).logger.borrow_mut());
            }
        } else {
            logger_start_sds(
                &mut *(*options).logger.borrow_mut(),
                otfcc_rust::bytesbuild!(b"Load from stdin"),
            );
            let mut ___loggedstep_v_1: bool = true;
            while ___loggedstep_v_1 {
                readEntireStdin(&raw mut buffer, &raw mut length);
                ___loggedstep_v_1 = false;
                logger_finish(&mut *(*options).logger.borrow_mut());
            }
        }
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v = false;
        logger_finish(&mut *(*options).logger.borrow_mut());
    }
    let mut json_root: *mut ParsedValue = ::core::ptr::null_mut::<ParsedValue>();
    logger_start_sds(
        &mut *(*options).logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Parse into JSON"),
    );
    let mut ___loggedstep_v_2: bool = true;
    while ___loggedstep_v_2 {
        json_root = json_parse(buffer, length as usize);
        free(buffer as *mut ::core::ffi::c_void);
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        if json_root.is_null() {
            logger_log_sds(
                &mut *(*options).logger.borrow_mut(),
                LOG_VL_CRITICAL,
                LoggerType::Error,
                otfcc_rust::bytesbuild!(
                    b"Cannot parse JSON file \"",
                    inPath.as_ref().map_or(::core::ptr::null(), |p| p.as_ptr()),
                    b"\". Exit.\n",
                ),
            );
            return EXIT_FAILURE;
        }
        ___loggedstep_v_2 = false;
        logger_finish(&mut *(*options).logger.borrow_mut());
    }
    let mut font: *mut Font = ::core::ptr::null_mut::<Font>();
    logger_start_sds(
        &mut *(*options).logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Parse"),
    );
    let mut ___loggedstep_v_3: bool = true;
    while ___loggedstep_v_3 {
        font = read_json(json_root as *mut ::core::ffi::c_void, 0 as u32, &*options);
        if font.is_null() {
            logger_log_sds(
                &mut *(*options).logger.borrow_mut(),
                LOG_VL_CRITICAL,
                LoggerType::Error,
                otfcc_rust::bytesbuild!(
                    b"Cannot parse JSON file \"",
                    inPath.as_ref().map_or(::core::ptr::null(), |p| p.as_ptr()),
                    b"\" as a font. Exit.\n",
                ),
            );
            return EXIT_FAILURE;
        }
        json_value_free(json_root);
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_3 = false;
        logger_finish(&mut *(*options).logger.borrow_mut());
    }
    logger_start_sds(
        &mut *(*options).logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Consolidate"),
    );
    let mut ___loggedstep_v_4: bool = true;
    while ___loggedstep_v_4 {
        otfcc_consolidate_font(font, &*options);
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_4 = false;
        logger_finish(&mut *(*options).logger.borrow_mut());
    }
    logger_start_sds(
        &mut *(*options).logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Build"),
    );
    let mut ___loggedstep_v_5: bool = true;
    while ___loggedstep_v_5 {
        let mut otf: *mut Buffer = serialize_to_otf(font, &*options) as *mut Buffer;
        logger_start_sds(
            &mut *(*options).logger.borrow_mut(),
            otfcc_rust::bytesbuild!(b"Write to file"),
        );
        let mut ___loggedstep_v_6: bool = true;
        while ___loggedstep_v_6 {
            // Always `Some` here -- the `outputPath.is_none()` branch
            // above already exited.
            let output_path = outputPath.as_ref().unwrap();
            let os_path = std::ffi::OsStr::from_bytes(output_path.as_bytes());
            if std::fs::write(std::path::Path::new(os_path), &(*otf).data).is_err() {
                logger_log_sds(
                    &mut *(*options).logger.borrow_mut(),
                    LOG_VL_CRITICAL,
                    LoggerType::Error,
                    otfcc_rust::bytesbuild!(
                        b"Cannot write to file \"",
                        output_path.as_bytes(),
                        b"\". Exit.\n",
                    ),
                );
                return EXIT_FAILURE;
            }
            ___loggedstep_v_6 = false;
            logger_finish(&mut *(*options).logger.borrow_mut());
        }
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        buffree(otf);
        otfcc_font_free(font);
        // `inPath`/`outputPath` are `Option<CString>` now -- both drop on
        // their own at the end of this function's scope, no explicit
        // free needed.
        ___loggedstep_v_5 = false;
        logger_finish(&mut *(*options).logger.borrow_mut());
    }
    otfcc_delete_options(options);
    return 0 as ::core::ffi::c_int;
}
pub fn main() -> ::std::process::ExitCode {
    let args: Vec<String> = ::std::env::args().skip(1).collect();
    unsafe { ::std::process::ExitCode::from(main_0(args) as u8) }
}
