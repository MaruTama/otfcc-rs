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

use otfcc_rust::support::stdio::{stdin, stdout, FILE};
use libc::{calloc, exit, fclose, fgetc, fileno, fopen, fprintf, fputc, fputs, free, fwrite, isatty, strcmp, strdup, strtol};
// `otfcc_readSFNT` and friends are this crate's own functions, still reached
// through `extern "C"` rather than `use otfcc_rust::…` because the binary also
// carries its own copies of the types in their signatures. Once those types
// are unified the declarations go away and so does this allow, which is only
// needed because `libc::FILE` is deliberately opaque.
#[allow(improper_ctypes)]
extern "C" {
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn otfcc_readSFNT(file: *mut FILE) -> *mut otfcc_SplineFontContainer;
    fn otfcc_deleteSFNT(font: *mut otfcc_SplineFontContainer);
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn otfcc_newLogger(target: *mut otfcc_ILoggerTarget) -> *mut otfcc_ILogger;
    fn otfcc_newStdErrTarget() -> *mut otfcc_ILoggerTarget;
    fn otfcc_newOptions() -> *mut otfcc_Options;
    fn otfcc_deleteOptions(options: *mut otfcc_Options);
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static otfcc_iFont: __caryll_elementinterface_otfcc_Font;
    fn otfcc_newOTFReader() -> *mut otfcc_IFontBuilder;
    fn otfcc_newJsonWriter() -> *mut otfcc_IFontSerializer;
    fn time_now(tv: *mut timespec);
    fn push_stopwatch(sofar: *mut timespec) -> sds;
}

use otfcc_rust::logger::{log_type_error, log_type_progress, otfcc_ILogger, otfcc_ILoggerTarget};

use otfcc_rust::support::options::{otfcc_Options};

use otfcc_rust::vendor::sds::{sds};
use otfcc_rust::vendor::json::{json_value};
use otfcc_rust::font::caryll_font::{__caryll_elementinterface_otfcc_Font, otfcc_Font, otfcc_IFontBuilder, otfcc_IFontSerializer};
use otfcc_rust::font::caryll_sfnt::{otfcc_SplineFontContainer};
use otfcc_rust::logger::{log_vl_critical, log_vl_progress};
use otfcc_rust::support::{EXIT_FAILURE, NULL};
































use otfcc_rust::vendor::json_builder::{json_serialize_mode_multiline, json_serialize_mode_packed, json_serialize_opts};
use libc::timespec;
use otfcc_rust::support::getopt::{no_argument, option, required_argument};
use otfcc_rust::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};





#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn getchar() -> ::core::ffi::c_int {
    return fgetc(stdin);
}
#[no_mangle]
pub unsafe extern "C" fn printInfo() {
    fprintf(
        stdout,
        b"This is Polymorphic otfccdump, version %d.%d.%d.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        MAIN_VER,
        SECONDARY_VER,
        PATCH_VER,
    );
}
#[no_mangle]
pub unsafe extern "C" fn printHelp() {
    fprintf(
        stdout,
        b"\nUsage : otfccdump [OPTIONS] input.[otf|ttf|ttc]\n\n -h, --help              : Display this help message and exit.\n -v, --version           : Display version information and exit.\n -o <file>               : Set output file path to <file>. When absent the dump\n                           will be written to STDOUT.\n -n <n>, --ttc-index <n> : Use the <n>th subfont within the input font.\n --pretty                : Prettify the output JSON.\n --ugly                  : Force uglify the output JSON.\n --verbose               : Show more information when building.\n -q, --quiet             : Be silent when building.\n\n --ignore-glyph-order    : Do not export glyph order information.\n --glyph-name-prefix pfx : Add a prefix to the glyph names.\n --ignore-hints          : Do not export hinting information.\n --decimal-cmap          : Export 'cmap' keys as decimal number.\n --hex-cmap              : Export 'cmap' keys as hex number (U+FFFF).\n --name-by-hash          : Name glyphs using its hash value.\n --name-by-gid           : Name glyphs using its glyph id.\n --add-bom               : Add BOM mark in the output. (It is default on Windows\n                           when redirecting to another program. Use --no-bom to\n                           turn it off.)\n\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut show_help: bool = false;
    let mut show_version: bool = false;
    let mut show_pretty: bool = false;
    let mut show_ugly: bool = false;
    let mut add_bom: bool = false;
    let mut _no_bom: bool = false;
    let mut ttcindex: u32 = 0 as u32;
    let mut longopts: [option; 21] = [
        option {
            name: b"version\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'v' as i32,
        },
        option {
            name: b"help\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'h' as i32,
        },
        option {
            name: b"pretty\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'p' as i32,
        },
        option {
            name: b"ugly\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"time\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"ignore-glyph-order\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"ignore-hints\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"hex-cmap\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"decimal-cmap\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"instr-as-bytes\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"name-by-hash\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"name-by-gid\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"glyph-name-prefix\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"quiet\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"add-bom\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"no-bom\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"output\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'o' as i32,
        },
        option {
            name: b"ttc-index\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'n' as i32,
        },
        option {
            name: b"debug-wait-on-start\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut options: *mut otfcc_Options = otfcc_newOptions();
    (*options).logger = otfcc_newLogger(otfcc_newStdErrTarget());
    (*(*options).logger)
        .indent
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        b"otfccdump\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*options).decimal_cmap = true;
    let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    let mut outputPath: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut inPath: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            b"vhqpio:n:\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut longopts as *mut option,
            &raw mut option_index,
        );
        if !(c != -(1 as ::core::ffi::c_int)) {
            break;
        }
        match c {
            0 => {
                if longopts[option_index as usize].flag.is_null() {
                    if strcmp(
                        longopts[option_index as usize].name,
                        b"ugly\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        show_ugly = true;
                    } else if !(strcmp(
                        longopts[option_index as usize].name,
                        b"time\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int)
                    {
                        if strcmp(
                            longopts[option_index as usize].name,
                            b"add-bom\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            add_bom = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"no-bom\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            _no_bom = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"ignore-glyph-order\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).ignore_glyph_order = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).verbose = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"quiet\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).quiet = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"ignore-hints\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).ignore_hints = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"decimal-cmap\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).decimal_cmap = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"hex-cmap\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).decimal_cmap = false;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"name-by-hash\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).name_glyphs_by_hash = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"name-by-gid\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).name_glyphs_by_gid = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"instr-as-bytes\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).instr_as_bytes = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"glyph-name-prefix\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).glyph_name_prefix = strdup(optarg);
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"debug-wait-on-start\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).debug_wait_on_start = true;
                        }
                    }
                }
            }
            118 => {
                show_version = true;
            }
            105 => {
                (*options).ignore_glyph_order = true;
            }
            104 => {
                show_help = true;
            }
            112 => {
                show_pretty = true;
            }
            111 => {
                outputPath = sdsnew(optarg);
            }
            113 => {
                (*options).quiet = true;
            }
            110 => {
                ttcindex = atoi(optarg) as u32;
            }
            _ => {}
        }
    }
    if (*options).debug_wait_on_start {
        getchar();
    }
    (*(*options).logger)
        .setVerbosity
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
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
    if optind >= argc {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_critical as ::core::ffi::c_int as u8,
            log_type_error,
            otfcc_rust::sdsbuild!(sdsempty(), b"Expected argument for input file name.\n"),
        );
        printHelp();
        exit(EXIT_FAILURE);
    } else {
        inPath = sdsnew(*argv.offset(optind as isize));
    }
    let mut begin: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    time_now(&raw mut begin);
    let mut sfnt: *mut otfcc_SplineFontContainer =
        ::core::ptr::null_mut::<otfcc_SplineFontContainer>();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        otfcc_rust::sdsbuild!(sdsempty(), b"Read SFNT"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            otfcc_rust::sdsbuild!(sdsempty(), b"From file ", inPath),
        );
        let mut file: *mut FILE = fopen(
            inPath as *const ::core::ffi::c_char,
            b"rb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        sfnt = otfcc_readSFNT(file);
        if sfnt.is_null() || (*sfnt).count == 0 as u32 {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_critical as ::core::ffi::c_int as u8,
                log_type_error,
                otfcc_rust::sdsbuild!(
                    sdsempty(),
                    b"Cannot read SFNT file \"",
                    inPath,
                    b"\". Exit.\n",
                ),
            );
            exit(EXIT_FAILURE);
        }
        if ttcindex >= (*sfnt).count {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_critical as ::core::ffi::c_int as u8,
                log_type_error,
                otfcc_rust::sdsbuild!(
                    sdsempty(),
                    b"Subfont index ",
                    ttcindex,
                    b" out of range for \"",
                    inPath,
                    b"\" (0 -- ",
                    (*sfnt).count.wrapping_sub(1 as u32),
                    b"). Exit.\n",
                ),
            );
            exit(EXIT_FAILURE);
        }
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    let mut font: *mut otfcc_Font = ::core::ptr::null_mut::<otfcc_Font>();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        otfcc_rust::sdsbuild!(sdsempty(), b"Read Font"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        let mut reader: *mut otfcc_IFontBuilder = otfcc_newOTFReader();
        font = (*reader).read.expect("non-null function pointer")(
            sfnt as *mut ::core::ffi::c_void,
            ttcindex,
            options,
        );
        if font.is_null() {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_critical as ::core::ffi::c_int as u8,
                log_type_error,
                otfcc_rust::sdsbuild!(
                    sdsempty(),
                    b"Font structure broken or corrupted \"",
                    inPath,
                    b"\". Exit.\n",
                ),
            );
            exit(EXIT_FAILURE);
        }
        (*reader).free.expect("non-null function pointer")(reader as *mut otfcc_IFontBuilder);
        if !sfnt.is_null() {
            otfcc_deleteSFNT(sfnt);
        }
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_0 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        otfcc_rust::sdsbuild!(sdsempty(), b"Consolidate"),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        otfcc_iFont.consolidate.expect("non-null function pointer")(font, options);
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_1 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    let mut root: *mut json_value = ::core::ptr::null_mut::<json_value>();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        otfcc_rust::sdsbuild!(sdsempty(), b"Dump"),
    );
    let mut ___loggedstep_v_2: bool = true;
    while ___loggedstep_v_2 {
        let mut dumper: *mut otfcc_IFontSerializer = otfcc_newJsonWriter();
        root = (*dumper).serialize.expect("non-null function pointer")(font, options)
            as *mut json_value;
        if root.is_null() {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_critical as ::core::ffi::c_int as u8,
                log_type_error,
                otfcc_rust::sdsbuild!(
                    sdsempty(),
                    b"Font structure broken or corrupted \"",
                    inPath,
                    b"\". Exit.\n",
                ),
            );
            exit(EXIT_FAILURE);
        }
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            push_stopwatch(&raw mut begin),
        );
        (*dumper).free.expect("non-null function pointer")(dumper as *mut otfcc_IFontSerializer);
        ___loggedstep_v_2 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buflen: usize = 0;
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        otfcc_rust::sdsbuild!(sdsempty(), b"Serialize to JSON"),
    );
    let mut ___loggedstep_v_3: bool = true;
    while ___loggedstep_v_3 {
        let mut jsonOptions: json_serialize_opts = json_serialize_opts {
            mode: 0,
            opts: 0,
            indent_size: 0,
        };
        jsonOptions.mode = json_serialize_mode_packed;
        jsonOptions.opts = 0 as ::core::ffi::c_int;
        jsonOptions.indent_size = 4 as ::core::ffi::c_int;
        if show_pretty as ::core::ffi::c_int != 0
            || outputPath.is_null() && isatty(fileno(stdout)) != 0
        {
            jsonOptions.mode = json_serialize_mode_multiline;
        }
        if show_ugly {
            jsonOptions.mode = json_serialize_mode_packed;
        }
        buflen = json_measure_ex(root, jsonOptions);
        buf = calloc(1 as usize, buflen) as *mut ::core::ffi::c_char;
        json_serialize_ex(buf, root, jsonOptions);
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_3 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        otfcc_rust::sdsbuild!(sdsempty(), b"Output"),
    );
    let mut ___loggedstep_v_4: bool = true;
    while ___loggedstep_v_4 {
        if !outputPath.is_null() {
            let mut outputFile: *mut FILE = fopen(
                outputPath as *const ::core::ffi::c_char,
                b"wb\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if outputFile.is_null() {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_critical as ::core::ffi::c_int as u8,
                    log_type_error,
                    otfcc_rust::sdsbuild!(
                        sdsempty(),
                        b"Cannot write to file \"",
                        outputPath,
                        b"\". Exit.",
                    ),
                );
                exit(EXIT_FAILURE);
            }
            if add_bom {
                fputc(0xef as ::core::ffi::c_int, outputFile);
                fputc(0xbb as ::core::ffi::c_int, outputFile);
                fputc(0xbf as ::core::ffi::c_int, outputFile);
            }
            let mut actualLen: usize = buflen.wrapping_sub(1 as usize);
            while *buf.offset(actualLen as isize) == 0 {
                actualLen = actualLen.wrapping_sub(1 as usize);
            }
            fwrite(
                buf as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                actualLen.wrapping_add(1 as usize),
                outputFile,
            );
            fclose(outputFile);
        } else {
            if add_bom {
                fputc(0xef as ::core::ffi::c_int, stdout);
                fputc(0xbb as ::core::ffi::c_int, stdout);
                fputc(0xbf as ::core::ffi::c_int, stdout);
            }
            fputs(buf, stdout);
        }
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_4 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        otfcc_rust::sdsbuild!(sdsempty(), b"Finalize"),
    );
    let mut ___loggedstep_v_5: bool = true;
    while ___loggedstep_v_5 {
        free(buf as *mut ::core::ffi::c_void);
        if !font.is_null() {
            otfcc_iFont.free.expect("non-null function pointer")(font);
        }
        if !root.is_null() {
            json_builder_free(root);
        }
        if !inPath.is_null() {
            sdsfree(inPath);
        }
        if !outputPath.is_null() {
            sdsfree(outputPath);
        }
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_5 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    otfcc_deleteOptions(options);
    return 0 as ::core::ffi::c_int;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ))
    }
}
