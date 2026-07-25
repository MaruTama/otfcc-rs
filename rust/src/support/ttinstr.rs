#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, snprintf, strlen, strtol};
unsafe extern "C" {
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_string_new(_: *const ::core::ffi::c_char) -> *mut json_value;
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsfree(s: sds);
    fn base64_encode(src: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
    fn base64_decode(src: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
    #[cfg(not(target_os = "macos"))]
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    #[cfg(not(target_os = "macos"))]
    fn __ctype_tolower_loc() -> *mut *const i32;
}

#[cfg(target_os = "macos")]
use crate::support::ctype_compat::{__ctype_b_loc, __ctype_tolower_loc};

use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};

use crate::support::options::{otfcc_Options};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_array, json_integer, json_pre_serialized, json_string, json_value};

use crate::support::ctype_compat::{_ISdigit};
use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};
pub const ttf_pushw: ttf_instructions = 184;
pub const ttf_pushb: ttf_instructions = 176;
pub const ttf_npushw: ttf_instructions = 65;
pub const ttf_npushb: ttf_instructions = 64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instrdata {
    pub instrs: *mut u8,
    pub instr_cnt: u32,
    pub bts: *mut u8,
}
pub const bt_byte: byte_types = 2;
pub const bt_cnt: byte_types = 1;
pub const bt_wordhi: byte_types = 3;
pub const bt_impliedreturn: byte_types = 5;
pub const bt_wordlo: byte_types = 4;
pub const bt_instr: byte_types = 0;
pub type ttf_instructions = ::core::ffi::c_uint;
pub const ttf_getvariation: ttf_instructions = 145;
pub const ttf_ws: ttf_instructions = 66;
pub const ttf_wcvtp: ttf_instructions = 68;
pub const ttf_wcvtf: ttf_instructions = 112;
pub const ttf_utp: ttf_instructions = 41;
pub const ttf_szps: ttf_instructions = 22;
pub const ttf_szp2: ttf_instructions = 21;
pub const ttf_szp1: ttf_instructions = 20;
pub const ttf_szp0: ttf_instructions = 19;
pub const ttf_swap: ttf_instructions = 35;
pub const ttf_svtca: ttf_instructions = 0;
pub const ttf_sub: ttf_instructions = 97;
pub const ttf_sswci: ttf_instructions = 30;
pub const ttf_ssw: ttf_instructions = 31;
pub const ttf_srp2: ttf_instructions = 18;
pub const ttf_srp1: ttf_instructions = 17;
pub const ttf_srp0: ttf_instructions = 16;
pub const ttf_sround: ttf_instructions = 118;
pub const ttf_spvtl: ttf_instructions = 6;
pub const ttf_spvtca: ttf_instructions = 2;
pub const ttf_spvfs: ttf_instructions = 10;
pub const ttf_smd: ttf_instructions = 26;
pub const ttf_sloop: ttf_instructions = 23;
pub const ttf_shz: ttf_instructions = 54;
pub const ttf_shpix: ttf_instructions = 56;
pub const ttf_shp: ttf_instructions = 50;
pub const ttf_shc: ttf_instructions = 52;
pub const ttf_sfvtpv: ttf_instructions = 14;
pub const ttf_sfvtl: ttf_instructions = 8;
pub const ttf_sfvtca: ttf_instructions = 4;
pub const ttf_sfvfs: ttf_instructions = 11;
pub const ttf_sds: ttf_instructions = 95;
pub const ttf_sdpvtl: ttf_instructions = 134;
pub const ttf_sdb: ttf_instructions = 94;
pub const ttf_scvtci: ttf_instructions = 29;
pub const ttf_scfs: ttf_instructions = 72;
pub const ttf_scantype: ttf_instructions = 141;
pub const ttf_scanctrl: ttf_instructions = 133;
pub const ttf_sangw: ttf_instructions = 126;
pub const ttf_s45round: ttf_instructions = 119;
pub const ttf_rutg: ttf_instructions = 124;
pub const ttf_rthg: ttf_instructions = 25;
pub const ttf_rtg: ttf_instructions = 24;
pub const ttf_rtdg: ttf_instructions = 61;
pub const ttf_rs: ttf_instructions = 67;
pub const ttf_round: ttf_instructions = 104;
pub const ttf_roll: ttf_instructions = 138;
pub const ttf_roff: ttf_instructions = 122;
pub const ttf_rdtg: ttf_instructions = 125;
pub const ttf_rcvt: ttf_instructions = 69;
pub const ttf_pop: ttf_instructions = 33;
pub const ttf_or: ttf_instructions = 91;
pub const ttf_odd: ttf_instructions = 86;
pub const ttf_nround: ttf_instructions = 108;
pub const ttf_not: ttf_instructions = 92;
pub const ttf_neq: ttf_instructions = 85;
pub const ttf_neg: ttf_instructions = 101;
pub const ttf_mul: ttf_instructions = 99;
pub const ttf_msirp: ttf_instructions = 58;
pub const ttf_mps: ttf_instructions = 76;
pub const ttf_mppem: ttf_instructions = 75;
pub const ttf_mirp: ttf_instructions = 224;
pub const ttf_mindex: ttf_instructions = 38;
pub const ttf_min: ttf_instructions = 140;
pub const ttf_miap: ttf_instructions = 62;
pub const ttf_mdrp: ttf_instructions = 192;
pub const ttf_mdap: ttf_instructions = 46;
pub const ttf_md: ttf_instructions = 73;
pub const ttf_max: ttf_instructions = 139;
pub const ttf_lteq: ttf_instructions = 81;
pub const ttf_lt: ttf_instructions = 80;
pub const ttf_loopcall: ttf_instructions = 42;
pub const ttf_jrot: ttf_instructions = 120;
pub const ttf_jrof: ttf_instructions = 121;
pub const ttf_jmpr: ttf_instructions = 28;
pub const ttf_iup: ttf_instructions = 48;
pub const ttf_isect: ttf_instructions = 15;
pub const ttf_ip: ttf_instructions = 57;
pub const ttf_instctrl: ttf_instructions = 142;
pub const ttf_if: ttf_instructions = 88;
pub const ttf_idef: ttf_instructions = 137;
pub const ttf_gteq: ttf_instructions = 83;
pub const ttf_gt: ttf_instructions = 82;
pub const ttf_gpv: ttf_instructions = 12;
pub const ttf_gfv: ttf_instructions = 13;
pub const ttf_getinfo: ttf_instructions = 136;
pub const ttf_gc: ttf_instructions = 70;
pub const ttf_floor: ttf_instructions = 102;
pub const ttf_fliprgon: ttf_instructions = 129;
pub const ttf_fliprgoff: ttf_instructions = 130;
pub const ttf_flippt: ttf_instructions = 128;
pub const ttf_flipon: ttf_instructions = 77;
pub const ttf_flipoff: ttf_instructions = 78;
pub const ttf_fdef: ttf_instructions = 44;
pub const ttf_even: ttf_instructions = 87;
pub const ttf_eq: ttf_instructions = 84;
pub const ttf_endf: ttf_instructions = 45;
pub const ttf_else: ttf_instructions = 27;
pub const ttf_eif: ttf_instructions = 89;
pub const ttf_dup: ttf_instructions = 32;
pub const ttf_div: ttf_instructions = 98;
pub const ttf_depth: ttf_instructions = 36;
pub const ttf_deltap3: ttf_instructions = 114;
pub const ttf_deltap2: ttf_instructions = 113;
pub const ttf_deltap1: ttf_instructions = 93;
pub const ttf_deltac3: ttf_instructions = 117;
pub const ttf_deltac2: ttf_instructions = 116;
pub const ttf_deltac1: ttf_instructions = 115;
pub const ttf_debug: ttf_instructions = 79;
pub const ttf_clear: ttf_instructions = 34;
pub const ttf_cindex: ttf_instructions = 37;
pub const ttf_ceiling: ttf_instructions = 103;
pub const ttf_call: ttf_instructions = 43;
pub const ttf_and: ttf_instructions = 90;
pub const ttf_alignrp: ttf_instructions = 60;
pub const ttf_alignpts: ttf_instructions = 39;
pub const ttf_add: ttf_instructions = 96;
pub const ttf_abs: ttf_instructions = 100;
pub const ttf_aa: ttf_instructions = 127;
pub type byte_types = ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn preserialize(mut x: *mut json_value) -> *mut json_value {
    let mut opts: json_serialize_opts = json_serialize_opts {
        mode: json_serialize_mode_packed,
        opts: 0,
        indent_size: 0,
    };
    let mut preserialize_len: usize = json_measure_ex(x, opts);
    let mut buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let mut xx: *mut json_value = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as usize) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = json_pre_serialized;
    return xx;
}
pub static ff_ttf_instrnames: [&::core::ffi::CStr; 256] = [
    c"SVTCA[y-axis]",
    c"SVTCA[x-axis]",
    c"SPVTCA[y-axis]",
    c"SPVTCA[x-axis]",
    c"SFVTCA[y-axis]",
    c"SFVTCA[x-axis]",
    c"SPVTL[parallel]",
    c"SPVTL[orthog]",
    c"SFVTL[parallel]",
    c"SFVTL[orthog]",
    c"SPVFS",
    c"SFVFS",
    c"GPV",
    c"GFV",
    c"SFVTPV",
    c"ISECT",
    c"SRP0",
    c"SRP1",
    c"SRP2",
    c"SZP0",
    c"SZP1",
    c"SZP2",
    c"SZPS",
    c"SLOOP",
    c"RTG",
    c"RTHG",
    c"SMD",
    c"ELSE",
    c"JMPR",
    c"SCVTCI",
    c"SSWCI",
    c"SSW",
    c"DUP",
    c"POP",
    c"CLEAR",
    c"SWAP",
    c"DEPTH",
    c"CINDEX",
    c"MINDEX",
    c"ALIGNPTS",
    c"Unknown28",
    c"UTP",
    c"LOOPCALL",
    c"CALL",
    c"FDEF",
    c"ENDF",
    c"MDAP[no-rnd]",
    c"MDAP[rnd]",
    c"IUP[y]",
    c"IUP[x]",
    c"SHP[rp2]",
    c"SHP[rp1]",
    c"SHC[rp2]",
    c"SHC[rp1]",
    c"SHZ[rp2]",
    c"SHZ[rp1]",
    c"SHPIX",
    c"IP",
    c"MSIRP[no-rp0]",
    c"MSIRP[rp0]",
    c"ALIGNRP",
    c"RTDG",
    c"MIAP[no-rnd]",
    c"MIAP[rnd]",
    c"NPUSHB",
    c"NPUSHW",
    c"WS",
    c"RS",
    c"WCVTP",
    c"RCVT",
    c"GC[cur]",
    c"GC[orig]",
    c"SCFS",
    c"MD[grid]",
    c"MD[orig]",
    c"MPPEM",
    c"MPS",
    c"FLIPON",
    c"FLIPOFF",
    c"DEBUG",
    c"LT",
    c"LTEQ",
    c"GT",
    c"GTEQ",
    c"EQ",
    c"NEQ",
    c"ODD",
    c"EVEN",
    c"IF",
    c"EIF",
    c"AND",
    c"OR",
    c"NOT",
    c"DELTAP1",
    c"SDB",
    c"SDS",
    c"ADD",
    c"SUB",
    c"DIV",
    c"MUL",
    c"ABS",
    c"NEG",
    c"FLOOR",
    c"CEILING",
    c"ROUND[Grey]",
    c"ROUND[Black]",
    c"ROUND[White]",
    c"ROUND[Undef4]",
    c"NROUND[Grey]",
    c"NROUND[Black]",
    c"NROUND[White]",
    c"NROUND[Undef4]",
    c"WCVTF",
    c"DELTAP2",
    c"DELTAP3",
    c"DELTAC1",
    c"DELTAC2",
    c"DELTAC3",
    c"SROUND",
    c"S45ROUND",
    c"JROT",
    c"JROF",
    c"ROFF",
    c"Unknown7B",
    c"RUTG",
    c"RDTG",
    c"SANGW",
    c"AA",
    c"FLIPPT",
    c"FLIPRGON",
    c"FLIPRGOFF",
    c"Unknown83",
    c"Unknown84",
    c"SCANCTRL",
    c"SDPVTL[parallel]",
    c"SDPVTL[orthog]",
    c"GETINFO",
    c"IDEF",
    c"ROLL",
    c"MAX",
    c"MIN",
    c"SCANTYPE",
    c"INSTCTRL",
    c"Unknown8F",
    c"Unknown90",
    c"GETVARIATION",
    c"Unknown92",
    c"Unknown93",
    c"Unknown94",
    c"Unknown95",
    c"Unknown96",
    c"Unknown97",
    c"Unknown98",
    c"Unknown99",
    c"Unknown9A",
    c"Unknown9B",
    c"Unknown9C",
    c"Unknown9D",
    c"Unknown9E",
    c"Unknown9F",
    c"UnknownA0",
    c"UnknownA1",
    c"UnknownA2",
    c"UnknownA3",
    c"UnknownA4",
    c"UnknownA5",
    c"UnknownA6",
    c"UnknownA7",
    c"UnknownA8",
    c"UnknownA9",
    c"UnknownAA",
    c"UnknownAB",
    c"UnknownAC",
    c"UnknownAD",
    c"UnknownAE",
    c"UnknownAF",
    c"PUSHB_1",
    c"PUSHB_2",
    c"PUSHB_3",
    c"PUSHB_4",
    c"PUSHB_5",
    c"PUSHB_6",
    c"PUSHB_7",
    c"PUSHB_8",
    c"PUSHW_1",
    c"PUSHW_2",
    c"PUSHW_3",
    c"PUSHW_4",
    c"PUSHW_5",
    c"PUSHW_6",
    c"PUSHW_7",
    c"PUSHW_8",
    c"MDRP[grey]",
    c"MDRP[black]",
    c"MDRP[white]",
    c"MDRP03",
    c"MDRP[rnd,grey]",
    c"MDRP[rnd,black]",
    c"MDRP[rnd,white]",
    c"MDRP07",
    c"MDRP[min,grey]",
    c"MDRP[min,black]",
    c"MDRP[min,white]",
    c"MDRP0b",
    c"MDRP[min,rnd,grey]",
    c"MDRP[min,rnd,black]",
    c"MDRP[min,rnd,white]",
    c"MDRP0f",
    c"MDRP[rp0,grey]",
    c"MDRP[rp0,black]",
    c"MDRP[rp0,white]",
    c"MDRP13",
    c"MDRP[rp0,rnd,grey]",
    c"MDRP[rp0,rnd,black]",
    c"MDRP[rp0,rnd,white]",
    c"MDRP17",
    c"MDRP[rp0,min,grey]",
    c"MDRP[rp0,min,black]",
    c"MDRP[rp0,min,white]",
    c"MDRP1b",
    c"MDRP[rp0,min,rnd,grey]",
    c"MDRP[rp0,min,rnd,black]",
    c"MDRP[rp0,min,rnd,white]",
    c"MDRP1f",
    c"MIRP[grey]",
    c"MIRP[black]",
    c"MIRP[white]",
    c"MIRP03",
    c"MIRP[rnd,grey]",
    c"MIRP[rnd,black]",
    c"MIRP[rnd,white]",
    c"MIRP07",
    c"MIRP[min,grey]",
    c"MIRP[min,black]",
    c"MIRP[min,white]",
    c"MIRP0b",
    c"MIRP[min,rnd,grey]",
    c"MIRP[min,rnd,black]",
    c"MIRP[min,rnd,white]",
    c"MIRP0f",
    c"MIRP[rp0,grey]",
    c"MIRP[rp0,black]",
    c"MIRP[rp0,white]",
    c"MIRP13",
    c"MIRP[rp0,rnd,grey]",
    c"MIRP[rp0,rnd,black]",
    c"MIRP[rp0,rnd,white]",
    c"MIRP17",
    c"MIRP[rp0,min,grey]",
    c"MIRP[rp0,min,black]",
    c"MIRP[rp0,min,white]",
    c"MIRP1b",
    c"MIRP[rp0,min,rnd,grey]",
    c"MIRP[rp0,min,rnd,black]",
    c"MIRP[rp0,min,rnd,white]",
    c"MIRP1f",
];
unsafe extern "C" fn strnmatch(
    mut str1: *const ::core::ffi::c_char,
    mut str2: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ch1: ::core::ffi::c_int = 0;
    let mut ch2: ::core::ffi::c_int = 0;
    loop {
        let fresh19 = n;
        n = n - 1;
        if !(fresh19 > 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh20 = str1;
        str1 = str1.offset(1);
        ch1 = *fresh20 as ::core::ffi::c_int;
        let fresh21 = str2;
        str2 = str2.offset(1);
        ch2 = *fresh21 as ::core::ffi::c_int;
        ch1 = {
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_int>() > 1_usize {
                if 0 != 0 {
                    let mut __c: ::core::ffi::c_int = ch1;
                    __res =
                        (if __c < -(128 as ::core::ffi::c_int) || __c > 255 as ::core::ffi::c_int {
                            __c as i32
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                } else {
                    __res = tolower(ch1);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(ch1 as isize) as ::core::ffi::c_int;
            }
            __res
        };
        ch2 = {
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_int>() > 1_usize {
                if 0 != 0 {
                    let mut __c: ::core::ffi::c_int = ch2;
                    __res =
                        (if __c < -(128 as ::core::ffi::c_int) || __c > 255 as ::core::ffi::c_int {
                            __c as i32
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                } else {
                    __res = tolower(ch2);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(ch2 as isize) as ::core::ffi::c_int;
            }
            __res
        };
        if ch1 != ch2 || ch1 == '\0' as i32 {
            return ch1 - ch2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_instrs(
    mut text: *mut ::core::ffi::c_char,
    mut len: *mut ::core::ffi::c_int,
    mut context: *mut ::core::ffi::c_void,
    mut IVError: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> (),
    >,
) -> *mut u8 {
    let mut numberstack: [::core::ffi::c_short; 256] = [0; 256];
    let mut npos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nread: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut push_left: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut push_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut brack: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut icnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut imax: ::core::ffi::c_int = strlen(text) as ::core::ffi::c_int;
    let mut val: ::core::ffi::c_int = 0;
    let mut instrs: *mut u8 = ::core::ptr::null_mut::<u8>();
    instrs = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul(imax as usize),
        444 as ::core::ffi::c_ulong,
    ) as *mut u8;
    pt = text;
    while *pt != 0 {
        npos = 0 as ::core::ffi::c_int;
        while npos < 256 as ::core::ffi::c_int {
            while *pt as ::core::ffi::c_int == ' ' as i32
                || *pt as ::core::ffi::c_int == '\t' as i32
            {
                pt = pt.offset(1);
            }
            if !(*(*__ctype_b_loc()).offset(*pt as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
                || *pt as ::core::ffi::c_int == '-' as i32)
            {
                break;
            }
            val = strtol(pt, &raw mut end, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if val > 32767 as ::core::ffi::c_int || val < -(32768 as ::core::ffi::c_int) {
                IVError.expect("non-null function pointer")(
                    context,
                    b"A value must be between [-32768,32767]\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
                return ::core::ptr::null_mut::<u8>();
            }
            pt = end;
            let fresh0 = npos;
            npos = npos + 1;
            numberstack[fresh0 as usize] = val as ::core::ffi::c_short;
        }
        while *pt as ::core::ffi::c_int == ' ' as i32 || *pt as ::core::ffi::c_int == '\t' as i32 {
            pt = pt.offset(1);
        }
        if !(npos == 0 as ::core::ffi::c_int
            && (*pt as ::core::ffi::c_int == '\r' as i32
                || *pt as ::core::ffi::c_int == '\n' as i32
                || *pt as ::core::ffi::c_int == '\0' as i32))
        {
            nread = 0 as ::core::ffi::c_int;
            if push_left == -(1 as ::core::ffi::c_int) {
                if npos == 0 as ::core::ffi::c_int {
                    IVError.expect("non-null function pointer")(
                        context,
                        b"Expected a number for a push count\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                    );
                } else if numberstack[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    > 255 as ::core::ffi::c_int
                    || numberstack[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        <= 0 as ::core::ffi::c_int
                {
                    IVError.expect("non-null function pointer")(
                        context,
                        b"The push count must be a number between 0 and 255\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                    );
                    return ::core::ptr::null_mut::<u8>();
                } else {
                    nread = 1 as ::core::ffi::c_int;
                    let fresh1 = icnt;
                    icnt = icnt + 1;
                    *instrs.offset(fresh1 as isize) =
                        numberstack[0 as ::core::ffi::c_int as usize] as u8;
                    push_left = numberstack[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                }
            }
            if push_left != 0 as ::core::ffi::c_int
                && push_left < npos - nread
                && (*pt as ::core::ffi::c_int == '\r' as i32
                    || *pt as ::core::ffi::c_int == '\n' as i32
                    || *pt as ::core::ffi::c_int == '\0' as i32)
            {
                IVError.expect("non-null function pointer")(
                    context,
                    b"More pushes specified than needed\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
                return ::core::ptr::null_mut::<u8>();
            }
            while push_left > 0 as ::core::ffi::c_int && nread < npos {
                if push_size == 2 as ::core::ffi::c_int {
                    let fresh2 = icnt;
                    icnt = icnt + 1;
                    *instrs.offset(fresh2 as isize) =
                        (numberstack[nread as usize] as ::core::ffi::c_int
                            >> 8 as ::core::ffi::c_int) as u8;
                    let fresh3 = nread;
                    nread = nread + 1;
                    let fresh4 = icnt;
                    icnt = icnt + 1;
                    *instrs.offset(fresh4 as isize) =
                        (numberstack[fresh3 as usize] as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int) as u8;
                } else if numberstack[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    > 255 as ::core::ffi::c_int
                    || (numberstack[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                {
                    IVError.expect("non-null function pointer")(
                        context,
                        b"A value to be pushed by a byte push must be between 0 and 255\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                    );
                    return ::core::ptr::null_mut::<u8>();
                } else {
                    let fresh5 = nread;
                    nread = nread + 1;
                    let fresh6 = icnt;
                    icnt = icnt + 1;
                    *instrs.offset(fresh6 as isize) = numberstack[fresh5 as usize] as u8;
                }
                push_left -= 1;
            }
            if nread < npos
                && push_left == 0 as ::core::ffi::c_int
                && (*pt as ::core::ffi::c_int == '\r' as i32
                    || *pt as ::core::ffi::c_int == '\n' as i32
                    || *pt as ::core::ffi::c_int == '\0' as i32)
            {
                IVError.expect("non-null function pointer")(
                    context,
                    b"Unexpected number\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
                return ::core::ptr::null_mut::<u8>();
            }
            if !(*pt as ::core::ffi::c_int == '\r' as i32
                || *pt as ::core::ffi::c_int == '\n' as i32
                || *pt as ::core::ffi::c_int == '\0' as i32)
            {
                if push_left > 0 as ::core::ffi::c_int {
                    IVError.expect("non-null function pointer")(
                        context,
                        b"Missing pushes\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                    );
                    return ::core::ptr::null_mut::<u8>();
                }
                while nread < npos {
                    i = nread;
                    if numberstack[nread as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                        && numberstack[nread as usize] as ::core::ffi::c_int
                            <= 255 as ::core::ffi::c_int
                    {
                        while i < npos
                            && numberstack[i as usize] as ::core::ffi::c_int
                                >= 0 as ::core::ffi::c_int
                            && numberstack[i as usize] as ::core::ffi::c_int
                                <= 255 as ::core::ffi::c_int
                        {
                            i += 1;
                        }
                        if i - nread <= 8 as ::core::ffi::c_int {
                            let fresh7 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh7 as isize) = (ttf_pushb as ::core::ffi::c_int
                                + (i - nread)
                                - 1 as ::core::ffi::c_int)
                                as u8;
                        } else {
                            let fresh8 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh8 as isize) =
                                ttf_npushb as ::core::ffi::c_int as u8;
                            let fresh9 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh9 as isize) = (i - nread) as u8;
                        }
                        while nread < i {
                            let fresh10 = nread;
                            nread = nread + 1;
                            let fresh11 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh11 as isize) =
                                numberstack[fresh10 as usize] as u8;
                        }
                    } else {
                        while i < npos
                            && ((numberstack[i as usize] as ::core::ffi::c_int)
                                < 0 as ::core::ffi::c_int
                                || numberstack[i as usize] as ::core::ffi::c_int
                                    > 255 as ::core::ffi::c_int)
                        {
                            i += 1;
                        }
                        if i - nread <= 8 as ::core::ffi::c_int {
                            let fresh12 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh12 as isize) = (ttf_pushw as ::core::ffi::c_int
                                + (i - nread)
                                - 1 as ::core::ffi::c_int)
                                as u8;
                        } else {
                            let fresh13 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh13 as isize) =
                                ttf_npushw as ::core::ffi::c_int as u8;
                            let fresh14 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh14 as isize) = (i - nread) as u8;
                        }
                        while nread < i {
                            let fresh15 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh15 as isize) = (numberstack[nread as usize]
                                as ::core::ffi::c_int
                                >> 8 as ::core::ffi::c_int)
                                as u8;
                            let fresh16 = nread;
                            nread = nread + 1;
                            let fresh17 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh17 as isize) = (numberstack[fresh16 as usize]
                                as ::core::ffi::c_int
                                & 0xff as ::core::ffi::c_int)
                                as u8;
                        }
                    }
                }
                brack = ::core::ptr::null_mut::<::core::ffi::c_char>();
                end = pt;
                while *end as ::core::ffi::c_int != '\r' as i32
                    && *end as ::core::ffi::c_int != '\n' as i32
                    && *end as ::core::ffi::c_int != ' ' as i32
                    && *end as ::core::ffi::c_int != '\0' as i32
                {
                    if *end as ::core::ffi::c_int == '[' as i32
                        || *end as ::core::ffi::c_int == '_' as i32
                    {
                        brack = end;
                    }
                    end = end.offset(1);
                }
                i = 0 as ::core::ffi::c_int;
                while i < 256 as ::core::ffi::c_int {
                    if strnmatch(
                        pt,
                        ff_ttf_instrnames[i as usize].as_ptr(),
                        end.offset_from(pt) as ::core::ffi::c_long as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int
                        && ::core::mem::size_of::<::core::ffi::c_char>()
                            .wrapping_mul(end.offset_from(pt) as ::core::ffi::c_long as usize)
                            == ff_ttf_instrnames[i as usize].count_bytes()
                    {
                        break;
                    }
                    i += 1;
                }
                if i == 256 as ::core::ffi::c_int && !brack.is_null() {
                    i = 0 as ::core::ffi::c_int;
                    while i < 256 as ::core::ffi::c_int {
                        if strnmatch(
                            pt,
                            ff_ttf_instrnames[i as usize].as_ptr(),
                            (brack.offset_from(pt) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long)
                                as ::core::ffi::c_int,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        i += 1;
                    }
                    val = strtol(
                        brack.offset(1 as ::core::ffi::c_int as isize),
                        &raw mut bend,
                        2 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    while *bend as ::core::ffi::c_int == ' ' as i32
                        || *bend as ::core::ffi::c_int == '\t' as i32
                    {
                        bend = bend.offset(1);
                    }
                    if *bend as ::core::ffi::c_int != ']' as i32 {
                        IVError.expect("non-null function pointer")(
                            context,
                            b"Missing right bracket in command (or bad binary value in bracket)\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                        );
                        return ::core::ptr::null_mut::<u8>();
                    }
                    if val >= 32 as ::core::ffi::c_int {
                        IVError.expect("non-null function pointer")(
                            context,
                            b"Bracketted value is too large\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                        );
                        return ::core::ptr::null_mut::<u8>();
                    }
                    i += val;
                }
                pt = end;
                let fresh18 = icnt;
                icnt = icnt + 1;
                *instrs.offset(fresh18 as isize) = i as u8;
                if i == ttf_npushb as ::core::ffi::c_int
                    || i == ttf_npushw as ::core::ffi::c_int
                    || i >= ttf_pushb as ::core::ffi::c_int
                        && i <= ttf_pushw as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                {
                    push_size = if i == ttf_npushb as ::core::ffi::c_int
                        || i >= ttf_pushb as ::core::ffi::c_int
                            && i <= ttf_pushb as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    };
                    if i == ttf_npushb as ::core::ffi::c_int
                        || i == ttf_npushw as ::core::ffi::c_int
                    {
                        push_left = -(1 as ::core::ffi::c_int);
                    } else if i >= ttf_pushb as ::core::ffi::c_int
                        && i <= ttf_pushb as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                    {
                        push_left = i - ttf_pushb as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                    } else {
                        push_left = i - ttf_pushw as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                    }
                }
                if *pt as ::core::ffi::c_int == '\0' as i32 {
                    break;
                }
            }
        }
        pt = pt.offset(1);
    }
    *len = icnt;
    instrs = __caryll_reallocate(
        instrs as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<u8>() as usize).wrapping_mul(
            (if icnt == 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                icnt
            }) as usize,
        ),
        573 as ::core::ffi::c_ulong,
    ) as *mut u8;
    return instrs;
}
unsafe extern "C" fn instr_typify(mut id: *mut instrdata) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = (*id).instr_cnt as ::core::ffi::c_int;
    let mut cnt: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut lh: ::core::ffi::c_int = 0;
    let mut instrs: *mut u8 = (*id).instrs;
    let mut bts: *mut u8 = ::core::ptr::null_mut::<u8>();
    if (*id).bts.is_null() {
        (*id).bts = __caryll_allocate_clean(
            (::core::mem::size_of::<u8>() as usize)
                .wrapping_mul((len + 1 as ::core::ffi::c_int) as usize),
            582 as ::core::ffi::c_ulong,
        ) as *mut u8;
    }
    bts = (*id).bts;
    lh = 0 as ::core::ffi::c_int;
    i = lh;
    while i < len {
        *bts.offset(i as isize) = bt_instr as ::core::ffi::c_int as u8;
        lh += 1;
        if *instrs.offset(i as isize) as ::core::ffi::c_int == ttf_npushb as ::core::ffi::c_int {
            i += 1;
            *bts.offset(i as isize) = bt_cnt as ::core::ffi::c_int as u8;
            cnt = *instrs.offset(i as isize) as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < cnt {
                i += 1;
                *bts.offset(i as isize) = bt_byte as ::core::ffi::c_int as u8;
                j += 1;
            }
            lh += 1 as ::core::ffi::c_int + cnt;
        } else if *instrs.offset(i as isize) as ::core::ffi::c_int
            == ttf_npushw as ::core::ffi::c_int
        {
            i += 1;
            *bts.offset(i as isize) = bt_cnt as ::core::ffi::c_int as u8;
            lh += 1;
            cnt = *instrs.offset(i as isize) as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < cnt {
                i += 1;
                *bts.offset(i as isize) = bt_wordhi as ::core::ffi::c_int as u8;
                i += 1;
                *bts.offset(i as isize) = bt_wordlo as ::core::ffi::c_int as u8;
                j += 1;
            }
            lh += 1 as ::core::ffi::c_int + cnt;
        } else if *instrs.offset(i as isize) as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
            == 0xb0 as ::core::ffi::c_int
        {
            cnt = (*instrs.offset(i as isize) as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < cnt {
                i += 1;
                *bts.offset(i as isize) = bt_byte as ::core::ffi::c_int as u8;
                j += 1;
            }
            lh += cnt;
        } else if *instrs.offset(i as isize) as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
            == 0xb8 as ::core::ffi::c_int
        {
            cnt = (*instrs.offset(i as isize) as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < cnt {
                i += 1;
                *bts.offset(i as isize) = bt_wordhi as ::core::ffi::c_int as u8;
                i += 1;
                *bts.offset(i as isize) = bt_wordlo as ::core::ffi::c_int as u8;
                j += 1;
            }
            lh += cnt;
        }
        i += 1;
    }
    *bts.offset(i as isize) = bt_impliedreturn as ::core::ffi::c_int as u8;
    return lh;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_ttinstr(
    mut instructions: *mut u8,
    mut length: u32,
    mut options: *const otfcc_Options,
) -> *mut json_value {
    if (*options).instr_as_bytes {
        let mut len: usize = 0 as usize;
        let mut buf: *mut u8 = base64_encode(instructions, length as usize, &raw mut len);
        return json_string_new_length(len as ::core::ffi::c_uint, buf as *mut ::core::ffi::c_char);
    } else {
        let mut id: instrdata = instrdata {
            instrs: ::core::ptr::null_mut::<u8>(),
            instr_cnt: 0,
            bts: ::core::ptr::null_mut::<u8>(),
        };
        memset(
            &raw mut id as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<instrdata>() as usize,
        );
        id.instr_cnt = length;
        id.instrs = instructions;
        instr_typify(&raw mut id);
        let mut ret: *mut json_value = json_array_new(id.instr_cnt as usize);
        let mut i: u32 = 0 as u32;
        while i < id.instr_cnt {
            if *id.bts.offset(i as isize) as ::core::ffi::c_int == bt_wordhi as ::core::ffi::c_int {
                json_array_push(
                    ret,
                    json_integer_new(
                        ((*id.instrs.offset(i as isize) as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *id.instrs.offset(i.wrapping_add(1 as u32) as isize)
                                as ::core::ffi::c_int) as i16
                            as i64,
                    ),
                );
                i = i.wrapping_add(1);
            } else if *id.bts.offset(i as isize) as ::core::ffi::c_int
                == bt_cnt as ::core::ffi::c_int
                || *id.bts.offset(i as isize) as ::core::ffi::c_int == bt_byte as ::core::ffi::c_int
            {
                json_array_push(
                    ret,
                    json_integer_new(*id.instrs.offset(i as isize) as i64),
                );
            } else {
                json_array_push(
                    ret,
                    json_string_new(ff_ttf_instrnames[*id.instrs.offset(i as isize) as usize].as_ptr()),
                );
            }
            i = i.wrapping_add(1);
        }
        free(id.bts as *mut ::core::ffi::c_void);
        id.bts = ::core::ptr::null_mut::<u8>();
        return preserialize(ret);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_ttinstr(
    mut col: *mut json_value,
    mut context: *mut ::core::ffi::c_void,
    mut Make: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut u8, u32) -> ()>,
    mut Wrong: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> (),
    >,
) {
    if col.is_null() {
        Make.expect("non-null function pointer")(
            context,
            ::core::ptr::null_mut::<u8>(),
            0 as u32,
        );
    } else if (*col).type_0 as ::core::ffi::c_uint
        == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut instrlen: usize = 0;
        let mut instructions: *mut u8 = base64_decode(
            (*col).u.string.ptr as *mut u8,
            (*col).u.string.length as usize,
            &raw mut instrlen,
        );
        Make.expect("non-null function pointer")(context, instructions, instrlen as u32);
    } else if (*col).type_0 as ::core::ffi::c_uint
        == json_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut istrlen: usize = 0 as usize;
        let mut j: u32 = 0 as u32;
        while j < (*col).u.array.length as u32 {
            let mut record: *mut json_value =
                *(*col).u.array.values.offset(j as isize) as *mut json_value;
            if (*record).type_0 as ::core::ffi::c_uint
                == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                istrlen = istrlen.wrapping_add(
                    (*record)
                        .u
                        .string
                        .length
                        .wrapping_add(1 as ::core::ffi::c_uint) as usize,
                );
            } else if (*record).type_0 as ::core::ffi::c_uint
                == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                istrlen = istrlen
                    .wrapping_add((1 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as usize);
            } else {
                Make.expect("non-null function pointer")(
                    context,
                    ::core::ptr::null_mut::<u8>(),
                    0 as u32,
                );
                return;
            }
            j = j.wrapping_add(1);
        }
        let mut instrString: sds = sdsnewlen(
            ::core::ptr::null::<::core::ffi::c_void>(),
            istrlen.wrapping_add(1 as usize),
        );
        let mut head: *mut ::core::ffi::c_char = instrString as *mut ::core::ffi::c_char;
        let mut j_0: u32 = 0 as u32;
        while j_0 < (*col).u.array.length as u32 {
            let mut record_0: *mut json_value =
                *(*col).u.array.values.offset(j_0 as isize) as *mut json_value;
            if (*record_0).type_0 as ::core::ffi::c_uint
                == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                memcpy(
                    head as *mut ::core::ffi::c_void,
                    (*record_0).u.string.ptr as *const ::core::ffi::c_void,
                    (::core::mem::size_of::<::core::ffi::c_char>() as usize)
                        .wrapping_mul((*record_0).u.string.length as usize),
                );
                head = head.offset((*record_0).u.string.length as isize);
            } else if (*record_0).type_0 as ::core::ffi::c_uint
                == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut n: ::core::ffi::c_int = snprintf(
                    head,
                    20 as usize,
                    b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                    (*record_0).u.integer as ::core::ffi::c_int,
                );
                head = head.offset(n as isize);
            }
            *head = '\n' as i32 as ::core::ffi::c_char;
            head = head.offset(1);
            j_0 = j_0.wrapping_add(1);
        }
        let mut instrLength: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut instructions_0: *mut u8 = parse_instrs(
            instrString as *mut ::core::ffi::c_char,
            &raw mut instrLength,
            context,
            Wrong,
        );
        sdsfree(instrString);
        if !instructions_0.is_null() && instrLength != 0 {
            Make.expect("non-null function pointer")(
                context,
                instructions_0,
                instrLength as u32,
            );
        } else {
            Make.expect("non-null function pointer")(
                context,
                ::core::ptr::null_mut::<u8>(),
                0 as u32,
            );
        }
    } else {
        Make.expect("non-null function pointer")(
            context,
            ::core::ptr::null_mut::<u8>(),
            0 as u32,
        );
    };
}
#[inline]
unsafe extern "C" fn tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
