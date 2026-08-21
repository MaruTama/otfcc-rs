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

use otfcc_rust::support::stdio::{stdin, stdout, FILE};
use libc::{exit, fclose, fgetc, fileno, fopen, fprintf, fputc, fwrite, isatty, strcmp, strdup, strtol};
// `otfcc_read_sfnt` and friends are this crate's own functions, still reached
// through `extern "C"` rather than `use otfcc_rust::…` because the binary also
// carries its own copies of the types in their signatures. Once those types
// are unified the declarations go away and so does this allow, which is only
// needed because `libc::FILE` is deliberately opaque.
#[allow(improper_ctypes)]
unsafe extern "C" {
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const LongOption,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

use otfcc_rust::logger::{LoggerType, logger_finish, logger_indent, logger_log_sds, logger_set_verbosity, logger_start_sds};

use otfcc_rust::support::options::{Options};

use otfcc_rust::support::built_json::BuiltValue;
use otfcc_rust::font::caryll_font::{Font};
use otfcc_rust::font::caryll_sfnt::{SplineFontContainer};
use otfcc_rust::logger::{LOG_VL_CRITICAL, LOG_VL_PROGRESS};
use otfcc_rust::support::{EXIT_FAILURE, NULL};
































use otfcc_rust::support::built_json::{JSON_SERIALIZE_MODE_MULTILINE, JSON_SERIALIZE_MODE_PACKED, JsonSerializeOpts};
use libc::timespec;
use otfcc_rust::support::getopt::{NO_ARGUMENT, LongOption, REQUIRED_ARGUMENT};
use otfcc_rust::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};
use otfcc_rust::font::caryll_font::{otfcc_font_free};
use otfcc_rust::consolidate::{otfcc_consolidate_font};
use otfcc_rust::font::caryll_sfnt::{otfcc_delete_sfnt, otfcc_read_sfnt};
use otfcc_rust::json_writer::{serialize_to_json};
use otfcc_rust::logger::{Logger, otfcc_new_std_err_target};
use std::cell::RefCell;
use otfcc_rust::otf_reader::{read_otf};
use otfcc_rust::support::options::{otfcc_delete_options, otfcc_new_options};
use otfcc_rust::support::stopwatch::{push_stopwatch, time_now};
use otfcc_rust::support::built_json::json_serialize_ex;





#[inline]
unsafe fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[inline]
unsafe fn getchar() -> ::core::ffi::c_int {
    return fgetc(stdin);
}
pub unsafe fn printInfo() {
    fprintf(
        stdout,
        b"This is Polymorphic otfccdump, version %d.%d.%d.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        MAIN_VER,
        SECONDARY_VER,
        PATCH_VER,
    );
}
pub unsafe fn printHelp() {
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
    let mut longopts: [LongOption; 21] = [
        LongOption {
            name: b"version\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'v' as i32,
        },
        LongOption {
            name: b"help\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'h' as i32,
        },
        LongOption {
            name: b"pretty\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'p' as i32,
        },
        LongOption {
            name: b"ugly\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"time\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"ignore-glyph-order\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"ignore-hints\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"hex-cmap\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"decimal-cmap\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"instr-as-bytes\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"name-by-hash\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"name-by-gid\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"glyph-name-prefix\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: REQUIRED_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"quiet\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"add-bom\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"no-bom\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: b"output\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: REQUIRED_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'o' as i32,
        },
        LongOption {
            name: b"ttc-index\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: REQUIRED_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'n' as i32,
        },
        LongOption {
            name: b"debug-wait-on-start\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: NO_ARGUMENT,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
        LongOption {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut options: *mut Options = otfcc_new_options();
    (*options).logger = RefCell::new(Logger::new(otfcc_new_std_err_target()));
    logger_indent(
        &mut *(*options).logger.borrow_mut(),
        b"otfccdump\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*options).decimal_cmap = true;
    let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    let mut outputPath: Option<::std::ffi::CString> = None;
    // Placeholder, unconditionally overwritten below before any real use
    // (the only path that skips the assignment calls `exit()`).
    let mut inPath: ::std::ffi::CString = ::std::ffi::CString::default();
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            b"vhqpio:n:\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut longopts as *mut LongOption,
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
                outputPath = Some(::std::ffi::CStr::from_ptr(optarg).to_owned());
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
    if optind >= argc {
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_CRITICAL,
            LoggerType::Error,
            otfcc_rust::bytesbuild!(b"Expected argument for input file name.\n"),
        );
        printHelp();
        exit(EXIT_FAILURE);
    } else {
        inPath = ::std::ffi::CStr::from_ptr(*argv.offset(optind as isize)).to_owned();
    }
    let mut begin: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    time_now(&raw mut begin);
    let mut sfnt: *mut SplineFontContainer =
        ::core::ptr::null_mut::<SplineFontContainer>();
    logger_start_sds(
        &mut *(*options).logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Read SFNT"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            otfcc_rust::bytesbuild!(b"From file ", inPath.as_bytes()),
        );
        let mut file: *mut FILE = fopen(
            inPath.as_ptr(),
            b"rb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        sfnt = otfcc_read_sfnt(file);
        if sfnt.is_null() || (*sfnt).count == 0 as u32 {
            logger_log_sds(
                &mut *(*options).logger.borrow_mut(),
                LOG_VL_CRITICAL,
                LoggerType::Error,
                otfcc_rust::bytesbuild!(b"Cannot read SFNT file \"",
                    inPath.as_bytes(),
                    b"\". Exit.\n",
                ),
            );
            exit(EXIT_FAILURE);
        }
        if ttcindex >= (*sfnt).count {
            logger_log_sds(
                &mut *(*options).logger.borrow_mut(),
                LOG_VL_CRITICAL,
                LoggerType::Error,
                otfcc_rust::bytesbuild!(b"Subfont index ",
                    ttcindex,
                    b" out of range for \"",
                    inPath.as_bytes(),
                    b"\" (0 -- ",
                    (*sfnt).count.wrapping_sub(1 as u32),
                    b"). Exit.\n",
                ),
            );
            exit(EXIT_FAILURE);
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
    let mut font: *mut Font = ::core::ptr::null_mut::<Font>();
    logger_start_sds(
        &mut *(*options).logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Read Font"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        font = read_otf(sfnt as *mut ::core::ffi::c_void, ttcindex, &*options);
        if font.is_null() {
            logger_log_sds(
                &mut *(*options).logger.borrow_mut(),
                LOG_VL_CRITICAL,
                LoggerType::Error,
                otfcc_rust::bytesbuild!(b"Font structure broken or corrupted \"",
                    inPath.as_bytes(),
                    b"\". Exit.\n",
                ),
            );
            exit(EXIT_FAILURE);
        }
        if !sfnt.is_null() {
            otfcc_delete_sfnt(sfnt);
        }
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_0 = false;
        logger_finish(&mut *(*options).logger.borrow_mut());
    }
    logger_start_sds(
        &mut *(*options).logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Consolidate"),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        otfcc_consolidate_font(font, &*options);
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_1 = false;
        logger_finish(&mut *(*options).logger.borrow_mut());
    }
    let mut root: *mut BuiltValue = ::core::ptr::null_mut::<BuiltValue>();
    logger_start_sds(
        &mut *(*options).logger.borrow_mut(),
        otfcc_rust::bytesbuild!(b"Dump"),
    );
    let mut ___loggedstep_v_2: bool = true;
    while ___loggedstep_v_2 {
        root = serialize_to_json(font, &*options) as *mut BuiltValue;
        if root.is_null() {
            logger_log_sds(
                &mut *(*options).logger.borrow_mut(),
                LOG_VL_CRITICAL,
                LoggerType::Error,
                otfcc_rust::bytesbuild!(b"Font structure broken or corrupted \"",
                    inPath.as_bytes(),
                    b"\". Exit.\n",
                ),
            );
            exit(EXIT_FAILURE);
        }
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_2 = false;
        logger_finish(&mut *(*options).logger.borrow_mut());
    }
    let mut buf: Vec<u8> = Vec::new();
    logger_start_sds(
        &mut *(*options).logger.borrow_mut(),
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
        jsonOptions.opts = 0 as ::core::ffi::c_int;
        jsonOptions.indent_size = 4 as ::core::ffi::c_int;
        if show_pretty as ::core::ffi::c_int != 0
            || outputPath.is_none() && isatty(fileno(stdout)) != 0
        {
            jsonOptions.mode = JSON_SERIALIZE_MODE_MULTILINE;
        }
        if show_ugly {
            jsonOptions.mode = JSON_SERIALIZE_MODE_PACKED;
        }
        buf = json_serialize_ex(&*root, jsonOptions);
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
        otfcc_rust::bytesbuild!(b"Output"),
    );
    let mut ___loggedstep_v_4: bool = true;
    while ___loggedstep_v_4 {
        if let Some(ref output_path) = outputPath {
            let mut outputFile: *mut FILE = fopen(
                output_path.as_ptr(),
                b"wb\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if outputFile.is_null() {
                logger_log_sds(
                    &mut *(*options).logger.borrow_mut(),
                    LOG_VL_CRITICAL,
                    LoggerType::Error,
                    otfcc_rust::bytesbuild!(b"Cannot write to file \"",
                        output_path.as_bytes(),
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
            fwrite(
                buf.as_ptr() as *const ::core::ffi::c_void,
                ::core::mem::size_of::<u8>() as usize,
                buf.len(),
                outputFile,
            );
            fclose(outputFile);
        } else {
            if add_bom {
                fputc(0xef as ::core::ffi::c_int, stdout);
                fputc(0xbb as ::core::ffi::c_int, stdout);
                fputc(0xbf as ::core::ffi::c_int, stdout);
            }
            fwrite(
                buf.as_ptr() as *const ::core::ffi::c_void,
                ::core::mem::size_of::<u8>() as usize,
                buf.len(),
                stdout,
            );
        }
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
        otfcc_rust::bytesbuild!(b"Finalize"),
    );
    let mut ___loggedstep_v_5: bool = true;
    while ___loggedstep_v_5 {
        if !font.is_null() {
            otfcc_font_free(font);
        }
        if !root.is_null() {
            drop(Box::from_raw(root));
        }
        // `inPath`/`outputPath` are `CString`/`Option<CString>` now --
        // both drop on their own at the end of this function's scope, no
        // explicit free needed.
        logger_log_sds(
            &mut *(*options).logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            push_stopwatch(&raw mut begin),
        );
        ___loggedstep_v_5 = false;
        logger_finish(&mut *(*options).logger.borrow_mut());
    }
    otfcc_delete_options(options);
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
