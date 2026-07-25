#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
#[allow(unused_imports)]
use ::otfcc_rust;

use otfcc_rust::support::stdio::{stderr, stdin, stdout, FILE};
use libc::{SEEK_SET, exit, fclose, feof, fgets, fopen, fprintf, fread, free, fseek, ftell, fwrite, malloc, realloc, strcmp, strlen, strtol};
extern "C" {
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn json_parse(json: *const ::core::ffi::c_char, length: usize) -> *mut json_value;
    fn json_value_free(_: *mut json_value);
    fn otfcc_newLogger(target: *mut otfcc_ILoggerTarget) -> *mut otfcc_ILogger;
    fn otfcc_newStdErrTarget() -> *mut otfcc_ILoggerTarget;
    fn otfcc_newOptions() -> *mut otfcc_Options;
    fn otfcc_deleteOptions(options: *mut otfcc_Options);
    fn otfcc_Options_optimizeTo(options: *mut otfcc_Options, level: u8);
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn buffree(buf: *mut caryll_Buffer);
    fn buflen(buf: *mut caryll_Buffer) -> usize;
    static otfcc_iFont: __caryll_elementinterface_otfcc_Font;
    fn otfcc_newJsonReader() -> *mut otfcc_IFontBuilder;
    fn otfcc_newOTFWriter() -> *mut otfcc_IFontSerializer;
    fn time_now(tv: *mut timespec);
    fn push_stopwatch(sofar: *mut timespec) -> sds;
}

use otfcc_rust::logger::{log_type_error, log_type_progress, otfcc_ILogger, otfcc_ILoggerTarget};
use otfcc_rust::support::buffer::{caryll_Buffer};
use otfcc_rust::support::options::{otfcc_Options};

use otfcc_rust::vendor::sds::{sds};
use otfcc_rust::vendor::json::{json_value};
use otfcc_rust::font::caryll_font::{__caryll_elementinterface_otfcc_Font, otfcc_Font, otfcc_IFontBuilder, otfcc_IFontSerializer};
use otfcc_rust::logger::{log_vl_critical, log_vl_progress};
use otfcc_rust::support::{EXIT_FAILURE, NULL};
use libc::timespec;
use otfcc_rust::support::getopt::{no_argument, option, required_argument};
use otfcc_rust::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};





































pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn printInfo() {
    fprintf(
        stdout,
        b"This is Polymorphic otfccbuild, version %d.%d.%d.\n\0" as *const u8
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
        b"\nUsage : otfccbuild [OPTIONS] [input.json] -o output.[ttf|otf]\n\n input.json                : Path to input file. When absent the input will be\n                             read from the STDIN.\n\n -h, --help                : Display this help message and exit.\n -v, --version             : Display version information and exit.\n -o <file>                 : Set output file path to <file>.\n -s, --dummy-dsig          : Include an empty DSIG table in the font. For some\n                             Microsoft applications, DSIG is required to enable\n                             OpenType features.\n -O<n>                     : Specify the level for optimization.\n     -O0                     Turn off any optimization.\n     -O1                     Default optimization.\n     -O2                     More aggressive optimizations for web font. In this\n                             level, the following options will be set:\n                               --merge-features\n                               --short-post\n                               --subroutinize\n     -O3                     Most aggressive opptimization strategy will be\n                             used. In this level, these options will be set:\n                               --force-cid\n                               --ignore-glyph-order\n --verbose                 : Show more information when building.\n -q, --quiet               : Be silent when building.\n\n --ignore-hints            : Ignore the hinting information in the input.\n --keep-average-char-width : Keep the OS/2.xAvgCharWidth value from the input\n                             instead of stating the average width of glyphs.\n                             Useful when creating a monospaced font.\n --keep-unicode-ranges     : Keep the OS/2.ulUnicodeRange[1-4] as-is.\n --keep-modified-time      : Keep the head.modified time in the json, instead of\n                             using current time.\n\n --short-post              : Don't export glyph names in the result font.\n --ignore-glyph-order, -i  : Ignore the glyph order information in the input.\n --keep-glyph-order, -k    : Keep the glyph order information in the input.\n                             Use to preserve glyph order under -O2 and -O3.\n --dont-ignore-glyph-order : Same as --keep-glyph-order.\n --merge-features          : Merge duplicate OpenType feature definitions.\n --dont-merge-features     : Keep duplicate OpenType feature definitions.\n --merge-lookups           : Merge duplicate OpenType lookups.\n --dont-merge-lookups      : Keep duplicate OpenType lookups.\n --force-cid               : Convert name-keyed CFF OTF into CID-keyed.\n --subroutinize            : Subroutinize CFF table.\n --stub-cmap4              : Create a stub `cmap` format 4 subtable if format\n                             12 subtable is present.\n\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn readEntireFile(
    mut inPath: *mut ::core::ffi::c_char,
    mut _buffer: *mut *mut ::core::ffi::c_char,
    mut _length: *mut ::core::ffi::c_long,
) {
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut length: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut f: *mut FILE =
        fopen(inPath, b"rb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if f.is_null() {
        fprintf(
            stderr,
            b"Cannot read JSON file \"%s\". Exit.\n\0" as *const u8 as *const ::core::ffi::c_char,
            inPath,
        );
        exit(EXIT_FAILURE);
    }
    fseek(f, 0 as ::core::ffi::c_long, SEEK_END);
    length = ftell(f);
    fseek(f, 0 as ::core::ffi::c_long, SEEK_SET);
    buffer = malloc(length as usize) as *mut ::core::ffi::c_char;
    if !buffer.is_null() {
        fread(
            buffer as *mut ::core::ffi::c_void,
            1 as usize,
            length as usize,
            f,
        );
    }
    fclose(f);
    if buffer.is_null() {
        fprintf(
            stderr,
            b"Cannot read JSON file \"%s\". Exit.\n\0" as *const u8 as *const ::core::ffi::c_char,
            inPath,
        );
        exit(EXIT_FAILURE);
    }
    *_buffer = buffer;
    *_length = length;
}
#[no_mangle]
pub unsafe extern "C" fn readEntireStdin(
    mut _buffer: *mut *mut ::core::ffi::c_char,
    mut _length: *mut ::core::ffi::c_long,
) {
    static mut BUF_SIZE: ::core::ffi::c_long = 0x400000 as ::core::ffi::c_long;
    static mut BUF_MIN: ::core::ffi::c_long = 0x1000 as ::core::ffi::c_long;
    let mut buffer: *mut ::core::ffi::c_char =
        malloc(BUF_SIZE as usize) as *mut ::core::ffi::c_char;
    let mut length: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut remain: ::core::ffi::c_long = BUF_SIZE;
    while feof(stdin) == 0 {
        if remain <= BUF_MIN {
            remain += length >> 1 as ::core::ffi::c_int & 0xffffff as ::core::ffi::c_long;
            buffer = realloc(
                buffer as *mut ::core::ffi::c_void,
                (length + remain) as usize,
            ) as *mut ::core::ffi::c_char;
        }
        fgets(
            buffer.offset(length as isize),
            remain as ::core::ffi::c_int,
            stdin,
        );
        let mut n: ::core::ffi::c_long =
            strlen(buffer.offset(length as isize)) as ::core::ffi::c_long;
        length += n;
        remain -= n;
    }
    *_buffer = buffer;
    *_length = length;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut begin: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    time_now(&raw mut begin);
    let mut show_help: bool = false;
    let mut show_version: bool = false;
    let mut outputPath: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut inPath: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    let mut options: *mut otfcc_Options = otfcc_newOptions();
    (*options).logger = otfcc_newLogger(otfcc_newStdErrTarget());
    (*(*options).logger)
        .indent
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        b"otfccbuild\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_Options_optimizeTo(options, 1 as u8);
    let mut longopts: [option; 25] = [
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
            name: b"keep-glyph-order\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"dont-ignore-glyph-order\0" as *const u8 as *const ::core::ffi::c_char,
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
            name: b"keep-average-char-width\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"keep-unicode-ranges\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"keep-modified-time\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"merge-lookups\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"merge-features\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"dont-merge-lookups\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"dont-merge-features\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"short-post\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"force-cid\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"subroutinize\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"stub-cmap4\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"dummy-dsig\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 's' as i32,
        },
        option {
            name: b"ship\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
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
            name: b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'O' as i32,
        },
        option {
            name: b"output\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'o' as i32,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ];
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            b"vhqskiO:o:\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut longopts as *mut option,
            &raw mut option_index,
        );
        if !(c != -(1 as ::core::ffi::c_int)) {
            break;
        }
        match c {
            0 => {
                if longopts[option_index as usize].flag.is_null() {
                    if !(strcmp(
                        longopts[option_index as usize].name,
                        b"time\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int)
                    {
                        if strcmp(
                            longopts[option_index as usize].name,
                            b"ignore-hints\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).ignore_hints = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"keep-average-char-width\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).keep_average_char_width = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"keep-unicode-ranges\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).keep_unicode_ranges = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"keep-modified-time\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).keep_modified_time = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"merge-features\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).merge_features = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"merge-lookups\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).merge_lookups = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"dont-merge-features\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).merge_features = false;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"dont-merge-lookups\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).merge_lookups = false;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"ignore-glyph-order\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).ignore_glyph_order = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"keep-glyph-order\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).ignore_glyph_order = false;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"dont-keep-glyph-order\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).ignore_glyph_order = false;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"short-post\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).short_post = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"force-cid\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).force_cid = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"subroutinize\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).cff_doSubroutinize = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"stub-cmap4\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).stub_cmap4 = true;
                        } else if strcmp(
                            longopts[option_index as usize].name,
                            b"ship\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*options).ignore_glyph_order = true;
                            (*options).short_post = true;
                            (*options).dummy_DSIG = true;
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
                        }
                    }
                }
            }
            118 => {
                show_version = true;
            }
            104 => {
                show_help = true;
            }
            107 => {
                (*options).ignore_glyph_order = false;
            }
            105 => {
                (*options).ignore_glyph_order = true;
            }
            111 => {
                outputPath = sdsnew(optarg);
            }
            115 => {
                (*options).dummy_DSIG = true;
            }
            113 => {
                (*options).quiet = true;
            }
            79 => {
                otfcc_Options_optimizeTo(options, atoi(optarg) as u8);
            }
            _ => {}
        }
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
        inPath = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        inPath = sdsnew(*argv.offset(optind as isize));
    }
    if outputPath.is_null() {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_critical as ::core::ffi::c_int as u8,
            log_type_error,
            otfcc_rust::sdsbuild!(
                sdsempty(),
                b"Unable to build OpenType font tile : output path not specified. Exit.\n",
            ),
        );
        printHelp();
        exit(EXIT_FAILURE);
    }
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut length: ::core::ffi::c_long = 0;
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        otfcc_rust::sdsbuild!(sdsempty(), b"Load file"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        if !inPath.is_null() {
            (*(*options).logger)
                .startSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                otfcc_rust::sdsbuild!(sdsempty(), b"Load from file ", inPath),
            );
            let mut ___loggedstep_v_0: bool = true;
            while ___loggedstep_v_0 {
                readEntireFile(
                    inPath as *mut ::core::ffi::c_char,
                    &raw mut buffer,
                    &raw mut length,
                );
                sdsfree(inPath);
                ___loggedstep_v_0 = false;
                (*(*options).logger)
                    .finish
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger
                );
            }
        } else {
            (*(*options).logger)
                .startSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                otfcc_rust::sdsbuild!(sdsempty(), b"Load from stdin"),
            );
            let mut ___loggedstep_v_1: bool = true;
            while ___loggedstep_v_1 {
                readEntireStdin(&raw mut buffer, &raw mut length);
                ___loggedstep_v_1 = false;
                (*(*options).logger)
                    .finish
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger
                );
            }
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
    let mut jsonRoot: *mut json_value = ::core::ptr::null_mut::<json_value>();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        otfcc_rust::sdsbuild!(sdsempty(), b"Parse into JSON"),
    );
    let mut ___loggedstep_v_2: bool = true;
    while ___loggedstep_v_2 {
        jsonRoot = json_parse(buffer, length as usize);
        free(buffer as *mut ::core::ffi::c_void);
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            push_stopwatch(&raw mut begin),
        );
        if jsonRoot.is_null() {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_critical as ::core::ffi::c_int as u8,
                log_type_error,
                otfcc_rust::sdsbuild!(
                    sdsempty(),
                    b"Cannot parse JSON file \"",
                    inPath,
                    b"\". Exit.\n",
                ),
            );
            exit(EXIT_FAILURE);
        }
        ___loggedstep_v_2 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    let mut font: *mut otfcc_Font = ::core::ptr::null_mut::<otfcc_Font>();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        otfcc_rust::sdsbuild!(sdsempty(), b"Parse"),
    );
    let mut ___loggedstep_v_3: bool = true;
    while ___loggedstep_v_3 {
        let mut parser: *mut otfcc_IFontBuilder = otfcc_newJsonReader();
        font = (*parser).read.expect("non-null function pointer")(
            jsonRoot as *mut ::core::ffi::c_void,
            0 as u32,
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
                    b"Cannot parse JSON file \"",
                    inPath,
                    b"\" as a font. Exit.\n",
                ),
            );
            exit(EXIT_FAILURE);
        }
        (*parser).free.expect("non-null function pointer")(parser as *mut otfcc_IFontBuilder);
        json_value_free(jsonRoot);
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
        otfcc_rust::sdsbuild!(sdsempty(), b"Consolidate"),
    );
    let mut ___loggedstep_v_4: bool = true;
    while ___loggedstep_v_4 {
        otfcc_iFont.consolidate.expect("non-null function pointer")(font, options);
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
        otfcc_rust::sdsbuild!(sdsempty(), b"Build"),
    );
    let mut ___loggedstep_v_5: bool = true;
    while ___loggedstep_v_5 {
        let mut writer: *mut otfcc_IFontSerializer = otfcc_newOTFWriter();
        let mut otf: *mut caryll_Buffer =
            (*writer).serialize.expect("non-null function pointer")(font, options)
                as *mut caryll_Buffer;
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            otfcc_rust::sdsbuild!(sdsempty(), b"Write to file"),
        );
        let mut ___loggedstep_v_6: bool = true;
        while ___loggedstep_v_6 {
            let mut outfile: *mut FILE = fopen(
                outputPath as *const ::core::ffi::c_char,
                b"wb\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if outfile.is_null() {
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
                        b"\". Exit.\n",
                    ),
                );
                exit(EXIT_FAILURE);
            }
            fwrite(
                (*otf).data as *const ::core::ffi::c_void,
                ::core::mem::size_of::<u8>() as usize,
                buflen(otf),
                outfile,
            );
            fclose(outfile);
            ___loggedstep_v_6 = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as u8,
            log_type_progress,
            push_stopwatch(&raw mut begin),
        );
        buffree(otf);
        (*writer).free.expect("non-null function pointer")(writer as *mut otfcc_IFontSerializer);
        otfcc_iFont.free.expect("non-null function pointer")(font);
        sdsfree(outputPath);
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
