extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn json_array_new(length: size_t) -> *mut json_value;
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
    fn json_integer_new(_: int64_t) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> size_t;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: size_t) -> sds;
    fn sdsfree(s: sds);
    fn base64_encode(src: *const uint8_t, len: size_t, out_len: *mut size_t) -> *mut uint8_t;
    fn base64_decode(src: *const uint8_t, len: size_t, out_len: *mut size_t) -> *mut uint8_t;
    #[cfg(not(target_os = "macos"))]
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    #[cfg(not(target_os = "macos"))]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
use crate::src::lib::support::stdio::FILE;
#[cfg(target_os = "macos")]
use crate::src::lib::support::ctype_compat::{__ctype_b_loc, __ctype_tolower_loc};

use crate::src::lib::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int16_t = __int16_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type json_type = ::core::ffi::c_uint;
pub const json_pre_serialized: json_type = 8;
pub const json_null: json_type = 7;
pub const json_boolean: json_type = 6;
pub const json_string: json_type = 5;
pub const json_double: json_type = 4;
pub const json_integer: json_type = 3;
pub const json_array: json_type = 2;
pub const json_object: json_type = 1;
pub const json_none: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_value {
    pub parent: *mut _json_value,
    pub type_0: json_type,
    pub u: C2RustUnnamed_0,
    pub _reserved: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub next_alloc: *mut _json_value,
    pub object_mem: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub boolean: ::core::ffi::c_int,
    pub integer: int64_t,
    pub dbl: ::core::ffi::c_double,
    pub string: C2RustUnnamed_3,
    pub object: C2RustUnnamed_2,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut json_object_entry,
}
pub type json_object_entry = _json_object_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_object_entry {
    pub name: *mut ::core::ffi::c_char,
    pub name_length: ::core::ffi::c_uint,
    pub value: *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub length: ::core::ffi::c_uint,
    pub ptr: *mut ::core::ffi::c_char,
}
pub type json_value = _json_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_serialize_opts {
    pub mode: ::core::ffi::c_int,
    pub opts: ::core::ffi::c_int,
    pub indent_size: ::core::ffi::c_int,
}
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILoggerTarget {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()>,
}
pub type otfcc_LoggerType = ::core::ffi::c_uint;
pub const log_type_progress: otfcc_LoggerType = 3;
pub const log_type_info: otfcc_LoggerType = 2;
pub const log_type_warning: otfcc_LoggerType = 1;
pub const log_type_error: otfcc_LoggerType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILogger {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub indent: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub indentSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub start: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub startSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub log: Option<
        unsafe extern "C" fn(
            *mut otfcc_ILogger,
            uint8_t,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t) -> ()>,
    pub getTarget: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Options {
    pub debug_wait_on_start: bool,
    pub ignore_glyph_order: bool,
    pub ignore_hints: bool,
    pub has_vertical_metrics: bool,
    pub export_fdselect: bool,
    pub keep_average_char_width: bool,
    pub keep_unicode_ranges: bool,
    pub short_post: bool,
    pub dummy_DSIG: bool,
    pub keep_modified_time: bool,
    pub instr_as_bytes: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub cff_short_vmtx: bool,
    pub merge_lookups: bool,
    pub merge_features: bool,
    pub force_cid: bool,
    pub cff_rollCharString: bool,
    pub cff_doSubroutinize: bool,
    pub stub_cmap4: bool,
    pub decimal_cmap: bool,
    pub name_glyphs_by_hash: bool,
    pub name_glyphs_by_gid: bool,
    pub glyph_name_prefix: *mut ::core::ffi::c_char,
    pub logger: *mut otfcc_ILogger,
}
pub const ttf_pushw: ttf_instructions = 184;
pub const ttf_pushb: ttf_instructions = 176;
pub const ttf_npushw: ttf_instructions = 65;
pub const ttf_npushb: ttf_instructions = 64;
pub const _ISdigit: C2RustUnnamed_4 = 2048;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instrdata {
    pub instrs: *mut uint8_t,
    pub instr_cnt: uint32_t,
    pub bts: *mut uint8_t,
}
pub const bt_byte: byte_types = 2;
pub const bt_cnt: byte_types = 1;
pub const bt_wordhi: byte_types = 3;
pub const bt_impliedreturn: byte_types = 5;
pub const bt_wordlo: byte_types = 4;
pub const bt_instr: byte_types = 0;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _IScntrl: C2RustUnnamed_4 = 2;
pub const _ISblank: C2RustUnnamed_4 = 1;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _ISupper: C2RustUnnamed_4 = 256;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const json_serialize_mode_packed: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn preserialize(mut x: *mut json_value) -> *mut json_value {
    let mut opts: json_serialize_opts = json_serialize_opts {
        mode: json_serialize_mode_packed,
        opts: 0,
        indent_size: 0,
    };
    let mut preserialize_len: size_t = json_measure_ex(x, opts);
    let mut buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let mut xx: *mut json_value = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as size_t) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = json_pre_serialized;
    return xx;
}
#[no_mangle]
pub static mut ff_ttf_instrnames: [*const ::core::ffi::c_char; 256] = [
    b"SVTCA[y-axis]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SVTCA[x-axis]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SPVTCA[y-axis]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SPVTCA[x-axis]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SFVTCA[y-axis]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SFVTCA[x-axis]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SPVTL[parallel]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SPVTL[orthog]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SFVTL[parallel]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SFVTL[orthog]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SPVFS\0" as *const u8 as *const ::core::ffi::c_char,
    b"SFVFS\0" as *const u8 as *const ::core::ffi::c_char,
    b"GPV\0" as *const u8 as *const ::core::ffi::c_char,
    b"GFV\0" as *const u8 as *const ::core::ffi::c_char,
    b"SFVTPV\0" as *const u8 as *const ::core::ffi::c_char,
    b"ISECT\0" as *const u8 as *const ::core::ffi::c_char,
    b"SRP0\0" as *const u8 as *const ::core::ffi::c_char,
    b"SRP1\0" as *const u8 as *const ::core::ffi::c_char,
    b"SRP2\0" as *const u8 as *const ::core::ffi::c_char,
    b"SZP0\0" as *const u8 as *const ::core::ffi::c_char,
    b"SZP1\0" as *const u8 as *const ::core::ffi::c_char,
    b"SZP2\0" as *const u8 as *const ::core::ffi::c_char,
    b"SZPS\0" as *const u8 as *const ::core::ffi::c_char,
    b"SLOOP\0" as *const u8 as *const ::core::ffi::c_char,
    b"RTG\0" as *const u8 as *const ::core::ffi::c_char,
    b"RTHG\0" as *const u8 as *const ::core::ffi::c_char,
    b"SMD\0" as *const u8 as *const ::core::ffi::c_char,
    b"ELSE\0" as *const u8 as *const ::core::ffi::c_char,
    b"JMPR\0" as *const u8 as *const ::core::ffi::c_char,
    b"SCVTCI\0" as *const u8 as *const ::core::ffi::c_char,
    b"SSWCI\0" as *const u8 as *const ::core::ffi::c_char,
    b"SSW\0" as *const u8 as *const ::core::ffi::c_char,
    b"DUP\0" as *const u8 as *const ::core::ffi::c_char,
    b"POP\0" as *const u8 as *const ::core::ffi::c_char,
    b"CLEAR\0" as *const u8 as *const ::core::ffi::c_char,
    b"SWAP\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEPTH\0" as *const u8 as *const ::core::ffi::c_char,
    b"CINDEX\0" as *const u8 as *const ::core::ffi::c_char,
    b"MINDEX\0" as *const u8 as *const ::core::ffi::c_char,
    b"ALIGNPTS\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown28\0" as *const u8 as *const ::core::ffi::c_char,
    b"UTP\0" as *const u8 as *const ::core::ffi::c_char,
    b"LOOPCALL\0" as *const u8 as *const ::core::ffi::c_char,
    b"CALL\0" as *const u8 as *const ::core::ffi::c_char,
    b"FDEF\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENDF\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDAP[no-rnd]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDAP[rnd]\0" as *const u8 as *const ::core::ffi::c_char,
    b"IUP[y]\0" as *const u8 as *const ::core::ffi::c_char,
    b"IUP[x]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SHP[rp2]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SHP[rp1]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SHC[rp2]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SHC[rp1]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SHZ[rp2]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SHZ[rp1]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SHPIX\0" as *const u8 as *const ::core::ffi::c_char,
    b"IP\0" as *const u8 as *const ::core::ffi::c_char,
    b"MSIRP[no-rp0]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MSIRP[rp0]\0" as *const u8 as *const ::core::ffi::c_char,
    b"ALIGNRP\0" as *const u8 as *const ::core::ffi::c_char,
    b"RTDG\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIAP[no-rnd]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIAP[rnd]\0" as *const u8 as *const ::core::ffi::c_char,
    b"NPUSHB\0" as *const u8 as *const ::core::ffi::c_char,
    b"NPUSHW\0" as *const u8 as *const ::core::ffi::c_char,
    b"WS\0" as *const u8 as *const ::core::ffi::c_char,
    b"RS\0" as *const u8 as *const ::core::ffi::c_char,
    b"WCVTP\0" as *const u8 as *const ::core::ffi::c_char,
    b"RCVT\0" as *const u8 as *const ::core::ffi::c_char,
    b"GC[cur]\0" as *const u8 as *const ::core::ffi::c_char,
    b"GC[orig]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SCFS\0" as *const u8 as *const ::core::ffi::c_char,
    b"MD[grid]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MD[orig]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MPPEM\0" as *const u8 as *const ::core::ffi::c_char,
    b"MPS\0" as *const u8 as *const ::core::ffi::c_char,
    b"FLIPON\0" as *const u8 as *const ::core::ffi::c_char,
    b"FLIPOFF\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEBUG\0" as *const u8 as *const ::core::ffi::c_char,
    b"LT\0" as *const u8 as *const ::core::ffi::c_char,
    b"LTEQ\0" as *const u8 as *const ::core::ffi::c_char,
    b"GT\0" as *const u8 as *const ::core::ffi::c_char,
    b"GTEQ\0" as *const u8 as *const ::core::ffi::c_char,
    b"EQ\0" as *const u8 as *const ::core::ffi::c_char,
    b"NEQ\0" as *const u8 as *const ::core::ffi::c_char,
    b"ODD\0" as *const u8 as *const ::core::ffi::c_char,
    b"EVEN\0" as *const u8 as *const ::core::ffi::c_char,
    b"IF\0" as *const u8 as *const ::core::ffi::c_char,
    b"EIF\0" as *const u8 as *const ::core::ffi::c_char,
    b"AND\0" as *const u8 as *const ::core::ffi::c_char,
    b"OR\0" as *const u8 as *const ::core::ffi::c_char,
    b"NOT\0" as *const u8 as *const ::core::ffi::c_char,
    b"DELTAP1\0" as *const u8 as *const ::core::ffi::c_char,
    b"SDB\0" as *const u8 as *const ::core::ffi::c_char,
    b"SDS\0" as *const u8 as *const ::core::ffi::c_char,
    b"ADD\0" as *const u8 as *const ::core::ffi::c_char,
    b"SUB\0" as *const u8 as *const ::core::ffi::c_char,
    b"DIV\0" as *const u8 as *const ::core::ffi::c_char,
    b"MUL\0" as *const u8 as *const ::core::ffi::c_char,
    b"ABS\0" as *const u8 as *const ::core::ffi::c_char,
    b"NEG\0" as *const u8 as *const ::core::ffi::c_char,
    b"FLOOR\0" as *const u8 as *const ::core::ffi::c_char,
    b"CEILING\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND[Grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND[Black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND[White]\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND[Undef4]\0" as *const u8 as *const ::core::ffi::c_char,
    b"NROUND[Grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"NROUND[Black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"NROUND[White]\0" as *const u8 as *const ::core::ffi::c_char,
    b"NROUND[Undef4]\0" as *const u8 as *const ::core::ffi::c_char,
    b"WCVTF\0" as *const u8 as *const ::core::ffi::c_char,
    b"DELTAP2\0" as *const u8 as *const ::core::ffi::c_char,
    b"DELTAP3\0" as *const u8 as *const ::core::ffi::c_char,
    b"DELTAC1\0" as *const u8 as *const ::core::ffi::c_char,
    b"DELTAC2\0" as *const u8 as *const ::core::ffi::c_char,
    b"DELTAC3\0" as *const u8 as *const ::core::ffi::c_char,
    b"SROUND\0" as *const u8 as *const ::core::ffi::c_char,
    b"S45ROUND\0" as *const u8 as *const ::core::ffi::c_char,
    b"JROT\0" as *const u8 as *const ::core::ffi::c_char,
    b"JROF\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROFF\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown7B\0" as *const u8 as *const ::core::ffi::c_char,
    b"RUTG\0" as *const u8 as *const ::core::ffi::c_char,
    b"RDTG\0" as *const u8 as *const ::core::ffi::c_char,
    b"SANGW\0" as *const u8 as *const ::core::ffi::c_char,
    b"AA\0" as *const u8 as *const ::core::ffi::c_char,
    b"FLIPPT\0" as *const u8 as *const ::core::ffi::c_char,
    b"FLIPRGON\0" as *const u8 as *const ::core::ffi::c_char,
    b"FLIPRGOFF\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown83\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown84\0" as *const u8 as *const ::core::ffi::c_char,
    b"SCANCTRL\0" as *const u8 as *const ::core::ffi::c_char,
    b"SDPVTL[parallel]\0" as *const u8 as *const ::core::ffi::c_char,
    b"SDPVTL[orthog]\0" as *const u8 as *const ::core::ffi::c_char,
    b"GETINFO\0" as *const u8 as *const ::core::ffi::c_char,
    b"IDEF\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROLL\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIN\0" as *const u8 as *const ::core::ffi::c_char,
    b"SCANTYPE\0" as *const u8 as *const ::core::ffi::c_char,
    b"INSTCTRL\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown8F\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown90\0" as *const u8 as *const ::core::ffi::c_char,
    b"GETVARIATION\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown92\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown93\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown94\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown95\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown96\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown97\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown98\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown99\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown9A\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown9B\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown9C\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown9D\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown9E\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown9F\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownA0\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownA1\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownA2\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownA3\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownA4\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownA5\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownA6\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownA7\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownA8\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownA9\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownAA\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownAB\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownAC\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownAD\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownAE\0" as *const u8 as *const ::core::ffi::c_char,
    b"UnknownAF\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHB_1\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHB_2\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHB_3\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHB_4\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHB_5\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHB_6\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHB_7\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHB_8\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHW_1\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHW_2\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHW_3\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHW_4\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHW_5\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHW_6\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHW_7\0" as *const u8 as *const ::core::ffi::c_char,
    b"PUSHW_8\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP03\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rnd,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rnd,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rnd,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP07\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[min,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[min,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[min,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP0b\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[min,rnd,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[min,rnd,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[min,rnd,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP0f\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP13\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,rnd,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,rnd,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,rnd,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP17\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,min,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,min,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,min,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP1b\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,min,rnd,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,min,rnd,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP[rp0,min,rnd,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MDRP1f\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP03\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rnd,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rnd,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rnd,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP07\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[min,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[min,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[min,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP0b\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[min,rnd,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[min,rnd,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[min,rnd,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP0f\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP13\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,rnd,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,rnd,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,rnd,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP17\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,min,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,min,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,min,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP1b\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,min,rnd,grey]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,min,rnd,black]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP[rp0,min,rnd,white]\0" as *const u8 as *const ::core::ffi::c_char,
    b"MIRP1f\0" as *const u8 as *const ::core::ffi::c_char,
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
                            __c as __int32_t
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
                            __c as __int32_t
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
) -> *mut uint8_t {
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
    let mut instrs: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    instrs = __caryll_allocate_clean(
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(imax as size_t),
        444 as ::core::ffi::c_ulong,
    ) as *mut uint8_t;
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
                return ::core::ptr::null_mut::<uint8_t>();
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
                    return ::core::ptr::null_mut::<uint8_t>();
                } else {
                    nread = 1 as ::core::ffi::c_int;
                    let fresh1 = icnt;
                    icnt = icnt + 1;
                    *instrs.offset(fresh1 as isize) =
                        numberstack[0 as ::core::ffi::c_int as usize] as uint8_t;
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
                return ::core::ptr::null_mut::<uint8_t>();
            }
            while push_left > 0 as ::core::ffi::c_int && nread < npos {
                if push_size == 2 as ::core::ffi::c_int {
                    let fresh2 = icnt;
                    icnt = icnt + 1;
                    *instrs.offset(fresh2 as isize) =
                        (numberstack[nread as usize] as ::core::ffi::c_int
                            >> 8 as ::core::ffi::c_int) as uint8_t;
                    let fresh3 = nread;
                    nread = nread + 1;
                    let fresh4 = icnt;
                    icnt = icnt + 1;
                    *instrs.offset(fresh4 as isize) =
                        (numberstack[fresh3 as usize] as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int) as uint8_t;
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
                    return ::core::ptr::null_mut::<uint8_t>();
                } else {
                    let fresh5 = nread;
                    nread = nread + 1;
                    let fresh6 = icnt;
                    icnt = icnt + 1;
                    *instrs.offset(fresh6 as isize) = numberstack[fresh5 as usize] as uint8_t;
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
                return ::core::ptr::null_mut::<uint8_t>();
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
                    return ::core::ptr::null_mut::<uint8_t>();
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
                                as uint8_t;
                        } else {
                            let fresh8 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh8 as isize) =
                                ttf_npushb as ::core::ffi::c_int as uint8_t;
                            let fresh9 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh9 as isize) = (i - nread) as uint8_t;
                        }
                        while nread < i {
                            let fresh10 = nread;
                            nread = nread + 1;
                            let fresh11 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh11 as isize) =
                                numberstack[fresh10 as usize] as uint8_t;
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
                                as uint8_t;
                        } else {
                            let fresh13 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh13 as isize) =
                                ttf_npushw as ::core::ffi::c_int as uint8_t;
                            let fresh14 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh14 as isize) = (i - nread) as uint8_t;
                        }
                        while nread < i {
                            let fresh15 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh15 as isize) = (numberstack[nread as usize]
                                as ::core::ffi::c_int
                                >> 8 as ::core::ffi::c_int)
                                as uint8_t;
                            let fresh16 = nread;
                            nread = nread + 1;
                            let fresh17 = icnt;
                            icnt = icnt + 1;
                            *instrs.offset(fresh17 as isize) = (numberstack[fresh16 as usize]
                                as ::core::ffi::c_int
                                & 0xff as ::core::ffi::c_int)
                                as uint8_t;
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
                        ff_ttf_instrnames[i as usize],
                        end.offset_from(pt) as ::core::ffi::c_long as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int
                        && ::core::mem::size_of::<::core::ffi::c_char>()
                            .wrapping_mul(end.offset_from(pt) as ::core::ffi::c_long as usize)
                            == strlen(ff_ttf_instrnames[i as usize])
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
                            ff_ttf_instrnames[i as usize],
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
                        return ::core::ptr::null_mut::<uint8_t>();
                    }
                    if val >= 32 as ::core::ffi::c_int {
                        IVError.expect("non-null function pointer")(
                            context,
                            b"Bracketted value is too large\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                        );
                        return ::core::ptr::null_mut::<uint8_t>();
                    }
                    i += val;
                }
                pt = end;
                let fresh18 = icnt;
                icnt = icnt + 1;
                *instrs.offset(fresh18 as isize) = i as uint8_t;
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
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(
            (if icnt == 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                icnt
            }) as size_t,
        ),
        573 as ::core::ffi::c_ulong,
    ) as *mut uint8_t;
    return instrs;
}
unsafe extern "C" fn instr_typify(mut id: *mut instrdata) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = (*id).instr_cnt as ::core::ffi::c_int;
    let mut cnt: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut lh: ::core::ffi::c_int = 0;
    let mut instrs: *mut uint8_t = (*id).instrs;
    let mut bts: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if (*id).bts.is_null() {
        (*id).bts = __caryll_allocate_clean(
            (::core::mem::size_of::<uint8_t>() as size_t)
                .wrapping_mul((len + 1 as ::core::ffi::c_int) as size_t),
            582 as ::core::ffi::c_ulong,
        ) as *mut uint8_t;
    }
    bts = (*id).bts;
    lh = 0 as ::core::ffi::c_int;
    i = lh;
    while i < len {
        *bts.offset(i as isize) = bt_instr as ::core::ffi::c_int as uint8_t;
        lh += 1;
        if *instrs.offset(i as isize) as ::core::ffi::c_int == ttf_npushb as ::core::ffi::c_int {
            i += 1;
            *bts.offset(i as isize) = bt_cnt as ::core::ffi::c_int as uint8_t;
            cnt = *instrs.offset(i as isize) as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < cnt {
                i += 1;
                *bts.offset(i as isize) = bt_byte as ::core::ffi::c_int as uint8_t;
                j += 1;
            }
            lh += 1 as ::core::ffi::c_int + cnt;
        } else if *instrs.offset(i as isize) as ::core::ffi::c_int
            == ttf_npushw as ::core::ffi::c_int
        {
            i += 1;
            *bts.offset(i as isize) = bt_cnt as ::core::ffi::c_int as uint8_t;
            lh += 1;
            cnt = *instrs.offset(i as isize) as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < cnt {
                i += 1;
                *bts.offset(i as isize) = bt_wordhi as ::core::ffi::c_int as uint8_t;
                i += 1;
                *bts.offset(i as isize) = bt_wordlo as ::core::ffi::c_int as uint8_t;
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
                *bts.offset(i as isize) = bt_byte as ::core::ffi::c_int as uint8_t;
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
                *bts.offset(i as isize) = bt_wordhi as ::core::ffi::c_int as uint8_t;
                i += 1;
                *bts.offset(i as isize) = bt_wordlo as ::core::ffi::c_int as uint8_t;
                j += 1;
            }
            lh += cnt;
        }
        i += 1;
    }
    *bts.offset(i as isize) = bt_impliedreturn as ::core::ffi::c_int as uint8_t;
    return lh;
}
#[no_mangle]
pub unsafe extern "C" fn dump_ttinstr(
    mut instructions: *mut uint8_t,
    mut length: uint32_t,
    mut options: *const otfcc_Options,
) -> *mut json_value {
    if (*options).instr_as_bytes {
        let mut len: size_t = 0 as size_t;
        let mut buf: *mut uint8_t = base64_encode(instructions, length as size_t, &raw mut len);
        return json_string_new_length(len as ::core::ffi::c_uint, buf as *mut ::core::ffi::c_char);
    } else {
        let mut id: instrdata = instrdata {
            instrs: ::core::ptr::null_mut::<uint8_t>(),
            instr_cnt: 0,
            bts: ::core::ptr::null_mut::<uint8_t>(),
        };
        memset(
            &raw mut id as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<instrdata>() as size_t,
        );
        id.instr_cnt = length;
        id.instrs = instructions;
        instr_typify(&raw mut id);
        let mut ret: *mut json_value = json_array_new(id.instr_cnt as size_t);
        let mut i: uint32_t = 0 as uint32_t;
        while i < id.instr_cnt {
            if *id.bts.offset(i as isize) as ::core::ffi::c_int == bt_wordhi as ::core::ffi::c_int {
                json_array_push(
                    ret,
                    json_integer_new(
                        ((*id.instrs.offset(i as isize) as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *id.instrs.offset(i.wrapping_add(1 as uint32_t) as isize)
                                as ::core::ffi::c_int) as int16_t
                            as int64_t,
                    ),
                );
                i = i.wrapping_add(1);
            } else if *id.bts.offset(i as isize) as ::core::ffi::c_int
                == bt_cnt as ::core::ffi::c_int
                || *id.bts.offset(i as isize) as ::core::ffi::c_int == bt_byte as ::core::ffi::c_int
            {
                json_array_push(
                    ret,
                    json_integer_new(*id.instrs.offset(i as isize) as int64_t),
                );
            } else {
                json_array_push(
                    ret,
                    json_string_new(ff_ttf_instrnames[*id.instrs.offset(i as isize) as usize]),
                );
            }
            i = i.wrapping_add(1);
        }
        free(id.bts as *mut ::core::ffi::c_void);
        id.bts = ::core::ptr::null_mut::<uint8_t>();
        return preserialize(ret);
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_ttinstr(
    mut col: *mut json_value,
    mut context: *mut ::core::ffi::c_void,
    mut Make: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t, uint32_t) -> ()>,
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
            ::core::ptr::null_mut::<uint8_t>(),
            0 as uint32_t,
        );
    } else if (*col).type_0 as ::core::ffi::c_uint
        == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut instrlen: size_t = 0;
        let mut instructions: *mut uint8_t = base64_decode(
            (*col).u.string.ptr as *mut uint8_t,
            (*col).u.string.length as size_t,
            &raw mut instrlen,
        );
        Make.expect("non-null function pointer")(context, instructions, instrlen as uint32_t);
    } else if (*col).type_0 as ::core::ffi::c_uint
        == json_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut istrlen: size_t = 0 as size_t;
        let mut j: uint32_t = 0 as uint32_t;
        while j < (*col).u.array.length as uint32_t {
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
                        .wrapping_add(1 as ::core::ffi::c_uint) as size_t,
                );
            } else if (*record).type_0 as ::core::ffi::c_uint
                == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                istrlen = istrlen
                    .wrapping_add((1 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as size_t);
            } else {
                Make.expect("non-null function pointer")(
                    context,
                    ::core::ptr::null_mut::<uint8_t>(),
                    0 as uint32_t,
                );
                return;
            }
            j = j.wrapping_add(1);
        }
        let mut instrString: sds = sdsnewlen(
            ::core::ptr::null::<::core::ffi::c_void>(),
            istrlen.wrapping_add(1 as size_t),
        );
        let mut head: *mut ::core::ffi::c_char = instrString as *mut ::core::ffi::c_char;
        let mut j_0: uint32_t = 0 as uint32_t;
        while j_0 < (*col).u.array.length as uint32_t {
            let mut record_0: *mut json_value =
                *(*col).u.array.values.offset(j_0 as isize) as *mut json_value;
            if (*record_0).type_0 as ::core::ffi::c_uint
                == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                memcpy(
                    head as *mut ::core::ffi::c_void,
                    (*record_0).u.string.ptr as *const ::core::ffi::c_void,
                    (::core::mem::size_of::<::core::ffi::c_char>() as size_t)
                        .wrapping_mul((*record_0).u.string.length as size_t),
                );
                head = head.offset((*record_0).u.string.length as isize);
            } else if (*record_0).type_0 as ::core::ffi::c_uint
                == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut n: ::core::ffi::c_int = snprintf(
                    head,
                    20 as size_t,
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
        let mut instructions_0: *mut uint8_t = parse_instrs(
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
                instrLength as uint32_t,
            );
        } else {
            Make.expect("non-null function pointer")(
                context,
                ::core::ptr::null_mut::<uint8_t>(),
                0 as uint32_t,
            );
        }
    } else {
        Make.expect("non-null function pointer")(
            context,
            ::core::ptr::null_mut::<uint8_t>(),
            0 as uint32_t,
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
