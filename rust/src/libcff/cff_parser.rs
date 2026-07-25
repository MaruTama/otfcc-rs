extern "C" {
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static cff_iIndex: __caryll_elementinterface_cff_Index;
    static cff_iDict: __caryll_elementinterface_cff_Dict;
    fn cff_close_Charset(cset: cff_Charset);
    fn cff_extract_Charset(
        data: *mut uint8_t,
        offset: int32_t,
        nchars: uint16_t,
        charsets: *mut cff_Charset,
    );
    fn cff_close_FDSelect(fds: cff_FDSelect);
    fn cff_extract_FDSelect(
        data: *mut uint8_t,
        offset: int32_t,
        nchars: uint16_t,
        fdselect: *mut cff_FDSelect,
    );
    fn cff_decodeCS2Token(start: *const uint8_t, val: *mut cff_Value) -> uint32_t;
}

use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type arity_t = uint32_t;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed = 10;
pub const log_vl_info: C2RustUnnamed = 5;
pub const log_vl_notice: C2RustUnnamed = 2;
pub const log_vl_important: C2RustUnnamed = 1;
pub const log_vl_critical: C2RustUnnamed = 0;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const op_FontName: C2RustUnnamed_0 = 3110;
pub const op_FDSelect: C2RustUnnamed_0 = 3109;
pub const op_FDArray: C2RustUnnamed_0 = 3108;
pub const op_UIDBase: C2RustUnnamed_0 = 3107;
pub const op_CIDCount: C2RustUnnamed_0 = 3106;
pub const op_CIDFontType: C2RustUnnamed_0 = 3105;
pub const op_CIDFontRevision: C2RustUnnamed_0 = 3104;
pub const op_CIDFontVersion: C2RustUnnamed_0 = 3103;
pub const op_ROS: C2RustUnnamed_0 = 3102;
pub const op_maxstack: C2RustUnnamed_0 = 25;
pub const op_vstore: C2RustUnnamed_0 = 24;
pub const op_BaseFontBlend: C2RustUnnamed_0 = 3095;
pub const op_blend: C2RustUnnamed_0 = 23;
pub const op_BaseFontName: C2RustUnnamed_0 = 3094;
pub const op_vsindex: C2RustUnnamed_0 = 22;
pub const op_PostScript: C2RustUnnamed_0 = 3093;
pub const op_nominalWidthX: C2RustUnnamed_0 = 21;
pub const op_SyntheicBase: C2RustUnnamed_0 = 3092;
pub const op_defaultWidthX: C2RustUnnamed_0 = 20;
pub const op_initialRandomSeed: C2RustUnnamed_0 = 3091;
pub const op_Subrs: C2RustUnnamed_0 = 19;
pub const op_ExpansionFactor: C2RustUnnamed_0 = 3090;
pub const op_Private: C2RustUnnamed_0 = 18;
pub const op_LanguageGroup: C2RustUnnamed_0 = 3089;
pub const op_CharStrings: C2RustUnnamed_0 = 17;
pub const op_Encoding: C2RustUnnamed_0 = 16;
pub const op_charset: C2RustUnnamed_0 = 15;
pub const op_ForceBold: C2RustUnnamed_0 = 3086;
pub const op_XUID: C2RustUnnamed_0 = 14;
pub const op_StemSnapV: C2RustUnnamed_0 = 3085;
pub const op_UniqueID: C2RustUnnamed_0 = 13;
pub const op_StemSnapH: C2RustUnnamed_0 = 3084;
pub const op_BlueFuzz: C2RustUnnamed_0 = 3083;
pub const op_StdVW: C2RustUnnamed_0 = 11;
pub const op_BlueShift: C2RustUnnamed_0 = 3082;
pub const op_StdHW: C2RustUnnamed_0 = 10;
pub const op_BlueScale: C2RustUnnamed_0 = 3081;
pub const op_FamilyOtherBlues: C2RustUnnamed_0 = 9;
pub const op_StrokeWidth: C2RustUnnamed_0 = 3080;
pub const op_FamilyBlues: C2RustUnnamed_0 = 8;
pub const op_FontMatrix: C2RustUnnamed_0 = 3079;
pub const op_OtherBlues: C2RustUnnamed_0 = 7;
pub const op_CharstringType: C2RustUnnamed_0 = 3078;
pub const op_BlueValues: C2RustUnnamed_0 = 6;
pub const op_PaintType: C2RustUnnamed_0 = 3077;
pub const op_FontBBox: C2RustUnnamed_0 = 5;
pub const op_UnderlineThickness: C2RustUnnamed_0 = 3076;
pub const op_Weight: C2RustUnnamed_0 = 4;
pub const op_UnderlinePosition: C2RustUnnamed_0 = 3075;
pub const op_FamilyName: C2RustUnnamed_0 = 3;
pub const op_ItalicAngle: C2RustUnnamed_0 = 3074;
pub const op_FullName: C2RustUnnamed_0 = 2;
pub const op_isFixedPitch: C2RustUnnamed_0 = 3073;
pub const op_Notice: C2RustUnnamed_0 = 1;
pub const op_Copyright: C2RustUnnamed_0 = 3072;
pub const op_version: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const op_flex1: C2RustUnnamed_1 = 3109;
pub const op_hflex1: C2RustUnnamed_1 = 3108;
pub const op_flex: C2RustUnnamed_1 = 3107;
pub const op_hflex: C2RustUnnamed_1 = 3106;
pub const op_hvcurveto: C2RustUnnamed_1 = 31;
pub const op_roll: C2RustUnnamed_1 = 3102;
pub const op_vhcurveto: C2RustUnnamed_1 = 30;
pub const op_index: C2RustUnnamed_1 = 3101;
pub const op_callgsubr: C2RustUnnamed_1 = 29;
pub const op_exch: C2RustUnnamed_1 = 3100;
pub const op_dup: C2RustUnnamed_1 = 3099;
pub const op_hhcurveto: C2RustUnnamed_1 = 27;
pub const op_sqrt: C2RustUnnamed_1 = 3098;
pub const op_vvcurveto: C2RustUnnamed_1 = 26;
pub const op_rlinecurve: C2RustUnnamed_1 = 25;
pub const op_mul: C2RustUnnamed_1 = 3096;
pub const op_rcurveline: C2RustUnnamed_1 = 24;
pub const op_random: C2RustUnnamed_1 = 3095;
pub const op_vstemhm: C2RustUnnamed_1 = 23;
pub const op_ifelse: C2RustUnnamed_1 = 3094;
pub const op_hmoveto: C2RustUnnamed_1 = 22;
pub const op_get: C2RustUnnamed_1 = 3093;
pub const op_rmoveto: C2RustUnnamed_1 = 21;
pub const op_put: C2RustUnnamed_1 = 3092;
pub const op_cntrmask: C2RustUnnamed_1 = 20;
pub const op_hintmask: C2RustUnnamed_1 = 19;
pub const op_drop: C2RustUnnamed_1 = 3090;
pub const op_hstemhm: C2RustUnnamed_1 = 18;
pub const op_cff2blend: C2RustUnnamed_1 = 16;
pub const op_eq: C2RustUnnamed_1 = 3087;
pub const op_cff2vsidx: C2RustUnnamed_1 = 15;
pub const op_neg: C2RustUnnamed_1 = 3086;
pub const op_endchar: C2RustUnnamed_1 = 14;
pub const op_div: C2RustUnnamed_1 = 3084;
pub const op_sub: C2RustUnnamed_1 = 3083;
pub const op_return: C2RustUnnamed_1 = 11;
pub const op_add: C2RustUnnamed_1 = 3082;
pub const op_callsubr: C2RustUnnamed_1 = 10;
pub const op_abs: C2RustUnnamed_1 = 3081;
pub const op_rrcurveto: C2RustUnnamed_1 = 8;
pub const op_vlineto: C2RustUnnamed_1 = 7;
pub const op_hlineto: C2RustUnnamed_1 = 6;
pub const op_not: C2RustUnnamed_1 = 3077;
pub const op_rlineto: C2RustUnnamed_1 = 5;
pub const op_or: C2RustUnnamed_1 = 3076;
pub const op_vmoveto: C2RustUnnamed_1 = 4;
pub const op_and: C2RustUnnamed_1 = 3075;
pub const op_vstem: C2RustUnnamed_1 = 3;
pub const op_hstem: C2RustUnnamed_1 = 1;
pub type cff_Value_Type = ::core::ffi::c_uint;
pub const CS2_FRACTION: cff_Value_Type = 3;
pub const cff_DOUBLE: cff_Value_Type = 3;
pub const CS2_OPERAND: cff_Value_Type = 2;
pub const cff_INTEGER: cff_Value_Type = 2;
pub const CS2_OPERATOR: cff_Value_Type = 1;
pub const cff_OPERATOR: cff_Value_Type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Value {
    pub t: cff_Value_Type,
    pub c2rust_unnamed: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub i: int32_t,
    pub d: ::core::ffi::c_double,
}
pub type cff_IndexCountType = ::core::ffi::c_uint;
pub const CFF_INDEX_32: cff_IndexCountType = 1;
pub const CFF_INDEX_16: cff_IndexCountType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Index {
    pub countType: cff_IndexCountType,
    pub count: arity_t,
    pub offSize: uint8_t,
    pub offset: *mut uint32_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_cff_Index {
    pub init: Option<unsafe extern "C" fn(*mut cff_Index) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut cff_Index, *const cff_Index) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut cff_Index, *mut cff_Index) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut cff_Index) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut cff_Index, cff_Index) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut cff_Index, cff_Index) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut cff_Index>,
    pub free: Option<unsafe extern "C" fn(*mut cff_Index) -> ()>,
    pub empty: Option<unsafe extern "C" fn(*mut cff_Index) -> ()>,
    pub getLength: Option<unsafe extern "C" fn(*const cff_Index) -> uint32_t>,
    pub parse: Option<unsafe extern "C" fn(*mut uint8_t, uint32_t, *mut cff_Index) -> ()>,
    pub fromCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            uint32_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> *mut caryll_Buffer>,
        ) -> *mut cff_Index,
    >,
    pub build: Option<unsafe extern "C" fn(*const cff_Index) -> *mut caryll_Buffer>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_DictEntry {
    pub op: uint32_t,
    pub cnt: uint32_t,
    pub vals: *mut cff_Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Dict {
    pub count: uint32_t,
    pub ents: *mut cff_DictEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_cff_Dict {
    pub init: Option<unsafe extern "C" fn(*mut cff_Dict) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut cff_Dict, *const cff_Dict) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut cff_Dict, *mut cff_Dict) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut cff_Dict) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut cff_Dict, cff_Dict) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut cff_Dict, cff_Dict) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut cff_Dict>,
    pub free: Option<unsafe extern "C" fn(*mut cff_Dict) -> ()>,
    pub parse: Option<unsafe extern "C" fn(*const uint8_t, uint32_t) -> *mut cff_Dict>,
    pub parseToCallback: Option<
        unsafe extern "C" fn(
            *const uint8_t,
            uint32_t,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    uint32_t,
                    uint8_t,
                    *mut cff_Value,
                    *mut ::core::ffi::c_void,
                ) -> (),
            >,
        ) -> (),
    >,
    pub parseDictKey:
        Option<unsafe extern "C" fn(*const uint8_t, uint32_t, uint32_t, uint32_t) -> cff_Value>,
    pub build: Option<unsafe extern "C" fn(*const cff_Dict) -> *mut caryll_Buffer>,
}
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const cff_CHARSET_FORMAT2: C2RustUnnamed_3 = 5;
pub const cff_CHARSET_FORMAT1: C2RustUnnamed_3 = 4;
pub const cff_CHARSET_FORMAT0: C2RustUnnamed_3 = 3;
pub const cff_CHARSET_EXPERTSUBSET: C2RustUnnamed_3 = 2;
pub const cff_CHARSET_EXPERT: C2RustUnnamed_3 = 1;
pub const cff_CHARSET_UNSPECED: C2RustUnnamed_3 = 0;
pub const cff_CHARSET_ISOADOBE: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat0 {
    pub format: uint8_t,
    pub glyph: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetRangeFormat1 {
    pub first: uint16_t,
    pub nleft: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat1 {
    pub format: uint8_t,
    pub range1: *mut cff_CharsetRangeFormat1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetRangeFormat2 {
    pub first: uint16_t,
    pub nleft: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat2 {
    pub format: uint8_t,
    pub range2: *mut cff_CharsetRangeFormat2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Charset {
    pub t: uint32_t,
    pub s: uint32_t,
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub f0: cff_CharsetFormat0,
    pub f1: cff_CharsetFormat1,
    pub f2: cff_CharsetFormat2,
}
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const cff_FDSELECT_UNSPECED: C2RustUnnamed_5 = 2;
pub const cff_FDSELECT_FORMAT3: C2RustUnnamed_5 = 1;
pub const cff_FDSELECT_FORMAT0: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelectFormat0 {
    pub format: uint8_t,
    pub fds: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelectRangeFormat3 {
    pub first: uint16_t,
    pub fd: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelectFormat3 {
    pub format: uint8_t,
    pub nranges: uint16_t,
    pub range3: *mut cff_FDSelectRangeFormat3,
    pub sentinel: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelect {
    pub t: uint32_t,
    pub s: uint32_t,
    pub c2rust_unnamed: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub f0: cff_FDSelectFormat0,
    pub f3: cff_FDSelectFormat3,
}
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const type2_transient_array: C2RustUnnamed_7 = 32;
pub const type2_max_subrs: C2RustUnnamed_7 = 65300;
pub const type2_charstring_len: C2RustUnnamed_7 = 65535;
pub const type2_subr_nesting: C2RustUnnamed_7 = 10;
pub const type2_stem_hints: C2RustUnnamed_7 = 96;
pub const type2_argument_stack: C2RustUnnamed_7 = 48;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Header {
    pub major: uint8_t,
    pub minor: uint8_t,
    pub hdrSize: uint8_t,
    pub offSize: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingFormat0 {
    pub format: uint8_t,
    pub ncodes: uint8_t,
    pub code: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingRangeFormat1 {
    pub first: uint8_t,
    pub nleft: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingFormat1 {
    pub format: uint8_t,
    pub nranges: uint8_t,
    pub range1: *mut cff_EncodingRangeFormat1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingSupplement {
    pub code: uint8_t,
    pub glyph: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingNS {
    pub nsup: uint8_t,
    pub supplement: *mut cff_EncodingSupplement,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Encoding {
    pub t: uint32_t,
    pub c2rust_unnamed: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub f0: cff_EncodingFormat0,
    pub f1: cff_EncodingFormat1,
    pub ns: cff_EncodingNS,
}
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const cff_ENC_UNSPECED: C2RustUnnamed_9 = 5;
pub const cff_ENC_FORMAT_SUPPLEMENT: C2RustUnnamed_9 = 4;
pub const cff_ENC_FORMAT1: C2RustUnnamed_9 = 3;
pub const cff_ENC_FORMAT0: C2RustUnnamed_9 = 2;
pub const cff_ENC_EXPERT: C2RustUnnamed_9 = 1;
pub const cff_ENC_STANDARD: C2RustUnnamed_9 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Stack {
    pub stack: *mut cff_Value,
    pub transient: [cff_Value; 32],
    pub index: arity_t,
    pub max: arity_t,
    pub stem: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_File {
    pub raw_data: *mut uint8_t,
    pub raw_length: uint32_t,
    pub cnt_glyph: uint16_t,
    pub head: cff_Header,
    pub name: cff_Index,
    pub top_dict: cff_Index,
    pub string: cff_Index,
    pub global_subr: cff_Index,
    pub encodings: cff_Encoding,
    pub charsets: cff_Charset,
    pub fdselect: cff_FDSelect,
    pub char_strings: cff_Index,
    pub font_dict: cff_Index,
    pub local_subr: cff_Index,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_IOutlineBuilder {
    pub setWidth:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> ()>,
    pub newContour: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub lineTo: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub curveTo: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub setHint: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            bool,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub setMask: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> ()>,
    pub getrand: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn gu1(mut s: *mut uint8_t, mut p: uint32_t) -> uint32_t {
    let mut b0: uint32_t = *s.offset(p as isize) as uint32_t;
    return b0;
}
#[inline]
unsafe extern "C" fn gu2(mut s: *mut uint8_t, mut p: uint32_t) -> uint32_t {
    let mut b0: uint32_t =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t;
    let mut b1: uint32_t = *s
        .offset(p as isize)
        .offset(1 as ::core::ffi::c_int as isize) as uint32_t;
    return b0 | b1;
}
unsafe extern "C" fn parse_encoding(
    mut cff: *mut cff_File,
    mut offset: int32_t,
    mut enc: *mut cff_Encoding,
) {
    let mut data: *mut uint8_t = (*cff).raw_data;
    if offset == cff_ENC_STANDARD as ::core::ffi::c_int as int32_t {
        (*enc).t = cff_ENC_STANDARD as ::core::ffi::c_int as uint32_t;
    } else if offset == cff_ENC_EXPERT as ::core::ffi::c_int as int32_t {
        (*enc).t = cff_ENC_EXPERT as ::core::ffi::c_int as uint32_t;
    } else {
        match *data.offset(offset as isize) as ::core::ffi::c_int {
            0 => {
                (*enc).t = cff_ENC_FORMAT0 as ::core::ffi::c_int as uint32_t;
                (*enc).c2rust_unnamed.f0.format = 0 as uint8_t;
                (*enc).c2rust_unnamed.f0.ncodes = *data.offset((offset + 1 as int32_t) as isize);
                (*enc).c2rust_unnamed.f0.code = __caryll_allocate_clean(
                    (::core::mem::size_of::<uint8_t>() as size_t)
                        .wrapping_mul((*enc).c2rust_unnamed.f0.ncodes as size_t),
                    30 as ::core::ffi::c_ulong,
                ) as *mut uint8_t;
                let mut i: uint32_t = 0 as uint32_t;
                while i < (*enc).c2rust_unnamed.f0.ncodes as uint32_t {
                    *(*enc).c2rust_unnamed.f0.code.offset(i as isize) = *data
                        .offset(((offset + 2 as int32_t) as uint32_t).wrapping_add(i) as isize);
                    i = i.wrapping_add(1);
                }
            }
            1 => {
                (*enc).t = cff_ENC_FORMAT1 as ::core::ffi::c_int as uint32_t;
                (*enc).c2rust_unnamed.f1.format = 1 as uint8_t;
                (*enc).c2rust_unnamed.f1.nranges = *data.offset((offset + 1 as int32_t) as isize);
                (*enc).c2rust_unnamed.f1.range1 = __caryll_allocate_clean(
                    (::core::mem::size_of::<cff_EncodingRangeFormat1>() as size_t)
                        .wrapping_mul((*enc).c2rust_unnamed.f1.nranges as size_t),
                    41 as ::core::ffi::c_ulong,
                )
                    as *mut cff_EncodingRangeFormat1;
                let mut i_0: uint32_t = 0 as uint32_t;
                while i_0 < (*enc).c2rust_unnamed.f1.nranges as uint32_t {
                    (*(*enc).c2rust_unnamed.f1.range1.offset(i_0 as isize)).first = *data.offset(
                        ((offset + 2 as int32_t) as uint32_t)
                            .wrapping_add(i_0.wrapping_mul(2 as uint32_t))
                            as isize,
                    );
                    (*(*enc).c2rust_unnamed.f1.range1.offset(i_0 as isize)).nleft = *data.offset(
                        ((offset + 3 as int32_t) as uint32_t)
                            .wrapping_add(i_0.wrapping_mul(2 as uint32_t))
                            as isize,
                    );
                    i_0 = i_0.wrapping_add(1);
                }
            }
            _ => {
                (*enc).t = cff_ENC_FORMAT_SUPPLEMENT as ::core::ffi::c_int as uint32_t;
                (*enc).c2rust_unnamed.ns.nsup = *data.offset(offset as isize);
                (*enc).c2rust_unnamed.ns.supplement = __caryll_allocate_clean(
                    (::core::mem::size_of::<cff_EncodingSupplement>() as size_t)
                        .wrapping_mul((*enc).c2rust_unnamed.ns.nsup as size_t),
                    52 as ::core::ffi::c_ulong,
                )
                    as *mut cff_EncodingSupplement;
                let mut i_1: uint32_t = 0 as uint32_t;
                while i_1 < (*enc).c2rust_unnamed.ns.nsup as uint32_t {
                    (*(*enc).c2rust_unnamed.ns.supplement.offset(i_1 as isize)).code = *data
                        .offset(
                            ((offset + 1 as int32_t) as uint32_t)
                                .wrapping_add(i_1.wrapping_mul(3 as uint32_t))
                                as isize,
                        );
                    (*(*enc).c2rust_unnamed.ns.supplement.offset(i_1 as isize)).glyph = gu2(
                        data,
                        ((offset + 2 as int32_t) as uint32_t)
                            .wrapping_add(i_1.wrapping_mul(3 as uint32_t)),
                    )
                        as uint16_t;
                    i_1 = i_1.wrapping_add(1);
                }
            }
        }
    };
}
unsafe extern "C" fn parse_cff_bytecode(mut cff: *mut cff_File, mut options: *const otfcc_Options) {
    let mut pos: uint32_t = 0;
    let mut offset: int32_t = 0;
    (*cff).head.major = gu1((*cff).raw_data, 0 as uint32_t) as uint8_t;
    (*cff).head.minor = gu1((*cff).raw_data, 1 as uint32_t) as uint8_t;
    (*cff).head.hdrSize = gu1((*cff).raw_data, 2 as uint32_t) as uint8_t;
    (*cff).head.offSize = gu1((*cff).raw_data, 3 as uint32_t) as uint8_t;
    pos = (*cff).head.hdrSize as uint32_t;
    cff_iIndex.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).name,
    );
    pos = (4 as uint32_t).wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
        &raw mut (*cff).name,
    ));
    cff_iIndex.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).top_dict,
    );
    if (*cff).name.count != (*cff).top_dict.count {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as uint8_t,
            log_type_warning,
            sdscatprintf(
                sdsempty(),
                b"[libcff] Bad CFF font: (%d, name), (%d, top_dict).\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*cff).name.count,
                (*cff).top_dict.count,
            ),
        );
    }
    pos = (4 as uint32_t)
        .wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
            &raw mut (*cff).name,
        ))
        .wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
            &raw mut (*cff).top_dict,
        ));
    cff_iIndex.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).string,
    );
    pos = (4 as uint32_t)
        .wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
            &raw mut (*cff).name,
        ))
        .wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
            &raw mut (*cff).top_dict,
        ))
        .wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
            &raw mut (*cff).string,
        ));
    cff_iIndex.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).global_subr,
    );
    if !(*cff).top_dict.data.is_null() {
        let mut offset_0: int32_t = 0;
        offset_0 = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_CharStrings as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as int32_t) {
            cff_iIndex.parse.expect("non-null function pointer")(
                (*cff).raw_data,
                offset_0 as uint32_t,
                &raw mut (*cff).char_strings,
            );
            (*cff).cnt_glyph = (*cff).char_strings.count as uint16_t;
        } else {
            cff_iIndex.empty.expect("non-null function pointer")(&raw mut (*cff).char_strings);
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as uint8_t,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"[libcff] Bad CFF font: no any glyph data.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                ),
            );
        }
        offset_0 = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_Encoding as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as int32_t) {
            parse_encoding(cff, offset_0, &raw mut (*cff).encodings);
        } else {
            (*cff).encodings.t = cff_ENC_UNSPECED as ::core::ffi::c_int as uint32_t;
        }
        offset_0 = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_charset as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as int32_t) {
            cff_extract_Charset(
                (*cff).raw_data,
                offset_0,
                (*cff).char_strings.count as uint16_t,
                &raw mut (*cff).charsets,
            );
        } else {
            (*cff).charsets.t = cff_CHARSET_UNSPECED as ::core::ffi::c_int as uint32_t;
        }
        offset_0 = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_FDSelect as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
        )
        .c2rust_unnamed
        .i;
        if (*cff).char_strings.count != 0 && offset_0 != -(1 as int32_t) {
            cff_extract_FDSelect(
                (*cff).raw_data,
                offset_0,
                (*cff).char_strings.count as uint16_t,
                &raw mut (*cff).fdselect,
            );
        } else {
            (*cff).fdselect.t = cff_FDSELECT_UNSPECED as ::core::ffi::c_int as uint32_t;
        }
        offset_0 = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_FDArray as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as int32_t) {
            cff_iIndex.parse.expect("non-null function pointer")(
                (*cff).raw_data,
                offset_0 as uint32_t,
                &raw mut (*cff).font_dict,
            );
        } else {
            cff_iIndex.empty.expect("non-null function pointer")(&raw mut (*cff).font_dict);
        }
    }
    let mut private_len: int32_t = -(1 as int32_t);
    let mut private_off: int32_t = -(1 as int32_t);
    if !(*cff).top_dict.data.is_null() {
        private_len = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_Private as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
        )
        .c2rust_unnamed
        .i;
        private_off = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_Private as ::core::ffi::c_int as uint32_t,
            1 as uint32_t,
        )
        .c2rust_unnamed
        .i;
    }
    if private_off != -(1 as int32_t) && private_len != -(1 as int32_t) {
        offset = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).raw_data.offset(private_off as isize),
            private_len as uint32_t,
            op_Subrs as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
        )
        .c2rust_unnamed
        .i;
        if offset != -(1 as int32_t) {
            cff_iIndex.parse.expect("non-null function pointer")(
                (*cff).raw_data,
                (private_off + offset) as uint32_t,
                &raw mut (*cff).local_subr,
            );
        } else {
            cff_iIndex.empty.expect("non-null function pointer")(&raw mut (*cff).local_subr);
        }
    } else {
        cff_iIndex.empty.expect("non-null function pointer")(&raw mut (*cff).local_subr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_openStream(
    mut data: *mut uint8_t,
    mut len: uint32_t,
    mut options: *const otfcc_Options,
) -> *mut cff_File {
    let mut file: *mut cff_File = ::core::ptr::null_mut::<cff_File>();
    file = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_File>() as size_t,
        203 as ::core::ffi::c_ulong,
    ) as *mut cff_File;
    (*file).raw_data = __caryll_allocate_clean(
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(len as size_t),
        205 as ::core::ffi::c_ulong,
    ) as *mut uint8_t;
    memcpy(
        (*file).raw_data as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        len as size_t,
    );
    (*file).raw_length = len;
    (*file).cnt_glyph = 0 as uint16_t;
    parse_cff_bytecode(file, options);
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn cff_close(mut file: *mut cff_File) {
    if !file.is_null() {
        if !(*file).raw_data.is_null() {
            free((*file).raw_data as *mut ::core::ffi::c_void);
            (*file).raw_data = ::core::ptr::null_mut::<uint8_t>();
        }
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).name);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).top_dict);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).string);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).global_subr);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).char_strings);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).font_dict);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).local_subr);
        match (*file).encodings.t {
            2 => {
                if !(*file).encodings.c2rust_unnamed.f0.code.is_null() {
                    free((*file).encodings.c2rust_unnamed.f0.code as *mut ::core::ffi::c_void);
                    (*file).encodings.c2rust_unnamed.f0.code = ::core::ptr::null_mut::<uint8_t>();
                }
            }
            3 => {
                if !(*file).encodings.c2rust_unnamed.f1.range1.is_null() {
                    free((*file).encodings.c2rust_unnamed.f1.range1 as *mut ::core::ffi::c_void);
                    (*file).encodings.c2rust_unnamed.f1.range1 =
                        ::core::ptr::null_mut::<cff_EncodingRangeFormat1>();
                }
            }
            4 => {
                if !(*file).encodings.c2rust_unnamed.ns.supplement.is_null() {
                    free(
                        (*file).encodings.c2rust_unnamed.ns.supplement as *mut ::core::ffi::c_void,
                    );
                    (*file).encodings.c2rust_unnamed.ns.supplement =
                        ::core::ptr::null_mut::<cff_EncodingSupplement>();
                }
            }
            0 | 1 | 5 | _ => {}
        }
        cff_close_Charset((*file).charsets);
        cff_close_FDSelect((*file).fdselect);
        free(file as *mut ::core::ffi::c_void);
        file = ::core::ptr::null_mut::<cff_File>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn cff_parseSubr(
    mut idx: uint16_t,
    mut raw: *mut uint8_t,
    mut fdarray: cff_Index,
    mut select: cff_FDSelect,
    mut subr: *mut cff_Index,
) -> uint8_t {
    let mut fd: uint8_t = 0 as uint8_t;
    let mut off_private: int32_t = 0;
    let mut len_private: int32_t = 0;
    let mut off_subr: int32_t = 0;
    match select.t {
        0 => {
            fd = *select.c2rust_unnamed.f0.fds.offset(idx as isize);
        }
        1 => {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < select.c2rust_unnamed.f3.nranges as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
            {
                if idx as ::core::ffi::c_int
                    >= (*select.c2rust_unnamed.f3.range3.offset(i as isize)).first
                        as ::core::ffi::c_int
                    && (idx as ::core::ffi::c_int)
                        < (*select
                            .c2rust_unnamed
                            .f3
                            .range3
                            .offset((i + 1 as ::core::ffi::c_int) as isize))
                        .first as ::core::ffi::c_int
                {
                    fd = (*select.c2rust_unnamed.f3.range3.offset(i as isize)).fd;
                }
                i += 1;
            }
            if idx as ::core::ffi::c_int
                >= (*select.c2rust_unnamed.f3.range3.offset(
                    (select.c2rust_unnamed.f3.nranges as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as isize,
                ))
                .first as ::core::ffi::c_int
                && (idx as ::core::ffi::c_int)
                    < select.c2rust_unnamed.f3.sentinel as ::core::ffi::c_int
            {
                fd = (*select.c2rust_unnamed.f3.range3.offset(
                    (select.c2rust_unnamed.f3.nranges as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as isize,
                ))
                .fd;
            }
        }
        2 => {
            fd = 0 as uint8_t;
        }
        _ => {}
    }
    off_private = cff_iDict.parseDictKey.expect("non-null function pointer")(
        fdarray
            .data
            .offset(*fdarray.offset.offset(fd as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*fdarray
            .offset
            .offset((fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*fdarray.offset.offset(fd as isize)),
        op_Private as ::core::ffi::c_int as uint32_t,
        1 as uint32_t,
    )
    .c2rust_unnamed
    .i;
    len_private = cff_iDict.parseDictKey.expect("non-null function pointer")(
        fdarray
            .data
            .offset(*fdarray.offset.offset(fd as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*fdarray
            .offset
            .offset((fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*fdarray.offset.offset(fd as isize)),
        op_Private as ::core::ffi::c_int as uint32_t,
        0 as uint32_t,
    )
    .c2rust_unnamed
    .i;
    if off_private != -(1 as int32_t) && len_private != -(1 as int32_t) {
        off_subr = cff_iDict.parseDictKey.expect("non-null function pointer")(
            raw.offset(off_private as isize),
            len_private as uint32_t,
            op_Subrs as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
        )
        .c2rust_unnamed
        .i;
        if off_subr != -(1 as int32_t) {
            cff_iIndex.parse.expect("non-null function pointer")(
                raw,
                (off_private + off_subr) as uint32_t,
                subr,
            );
        } else {
            cff_iIndex.empty.expect("non-null function pointer")(subr);
        }
    } else {
        cff_iIndex.empty.expect("non-null function pointer")(subr);
    }
    return fd;
}
#[inline]
unsafe extern "C" fn compute_subr_bias(mut cnt: uint16_t) -> uint16_t {
    if (cnt as ::core::ffi::c_int) < 1240 as ::core::ffi::c_int {
        return 107 as uint16_t;
    } else if (cnt as ::core::ffi::c_int) < 33900 as ::core::ffi::c_int {
        return 1131 as uint16_t;
    } else {
        return 32768 as uint16_t;
    };
}
unsafe extern "C" fn reverseStack(
    mut stack: *mut cff_Stack,
    mut left: uint8_t,
    mut right: uint8_t,
) {
    let mut p1: *mut cff_Value = (*stack).stack.offset(left as ::core::ffi::c_int as isize);
    let mut p2: *mut cff_Value = (*stack).stack.offset(right as ::core::ffi::c_int as isize);
    while p1 < p2 {
        let mut temp: cff_Value = *p1;
        *p1 = *p2;
        *p2 = temp;
        p1 = p1.offset(1);
        p2 = p2.offset(-1);
    }
}
unsafe extern "C" fn callback_nopSetWidth(
    mut _context: *mut ::core::ffi::c_void,
    mut _width: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nopNewContour(mut _context: *mut ::core::ffi::c_void) {}
unsafe extern "C" fn callback_nopLineTo(
    mut _context: *mut ::core::ffi::c_void,
    mut _x1: ::core::ffi::c_double,
    mut _y1: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nopCurveTo(
    mut _context: *mut ::core::ffi::c_void,
    mut _x1: ::core::ffi::c_double,
    mut _y1: ::core::ffi::c_double,
    mut _x2: ::core::ffi::c_double,
    mut _y2: ::core::ffi::c_double,
    mut _x3: ::core::ffi::c_double,
    mut _y3: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nopsetHint(
    mut _context: *mut ::core::ffi::c_void,
    mut _isVertical: bool,
    mut _position: ::core::ffi::c_double,
    mut _width: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nopsetMask(
    mut _context: *mut ::core::ffi::c_void,
    mut _isContourMask: bool,
    mut mask: *mut bool,
) {
    free(mask as *mut ::core::ffi::c_void);
    mask = ::core::ptr::null_mut::<bool>();
}
unsafe extern "C" fn callback_nopgetrand(
    mut _context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_double {
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn cff_parseOutline(
    mut data: *mut uint8_t,
    mut len: uint32_t,
    mut gsubr: cff_Index,
    mut lsubr: cff_Index,
    mut stack: *mut cff_Stack,
    mut outline: *mut ::core::ffi::c_void,
    mut methods: cff_IOutlineBuilder,
    mut options: *const otfcc_Options,
) {
    let mut gsubr_bias: uint16_t = compute_subr_bias(gsubr.count as uint16_t);
    let mut lsubr_bias: uint16_t = compute_subr_bias(lsubr.count as uint16_t);
    let mut start: *mut uint8_t = data;
    let mut advance: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut cnt_bezier: uint32_t = 0;
    let mut val: cff_Value = cff_Value {
        t: 0 as cff_Value_Type,
        c2rust_unnamed: C2RustUnnamed_2 { i: 0 },
    };
    let mut setWidth: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> (),
    > = methods.setWidth;
    let mut newContour: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> =
        methods.newContour;
    let mut lineTo: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    > = methods.lineTo;
    let mut curveTo: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    > = methods.curveTo;
    let mut setHint: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            bool,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    > = methods.setHint;
    let mut setMask: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> ()> =
        methods.setMask;
    let mut getrand: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double,
    > = methods.getrand;
    if setWidth.is_none() {
        setWidth = Some(
            callback_nopSetWidth
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> (),
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> ()>;
    }
    if newContour.is_none() {
        newContour =
            Some(callback_nopNewContour as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    }
    if lineTo.is_none() {
        lineTo = Some(
            callback_nopLineTo
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
            >;
    }
    if curveTo.is_none() {
        curveTo = Some(
            callback_nopCurveTo
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
            >;
    }
    if setHint.is_none() {
        setHint = Some(
            callback_nopsetHint
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    bool,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    bool,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
            >;
    }
    if setMask.is_none() {
        setMask = Some(
            callback_nopsetMask
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> (),
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> ()>;
    }
    if getrand.is_none() {
        getrand = Some(
            callback_nopgetrand
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double,
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double>;
    }
    while start < data.offset(len as isize) {
        advance = cff_decodeCS2Token(start, &raw mut val);
        match val.t as ::core::ffi::c_uint {
            1 => {
                let mut hintBase: ::core::ffi::c_double = 0.;
                match val.c2rust_unnamed.i {
                    1 | 3 | 18 | 23 => {
                        if (*stack).index.wrapping_rem(2 as arity_t) != 0 {
                            setWidth.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                        }
                        (*stack).stem = ((*stack).stem as arity_t)
                            .wrapping_add((*stack).index >> 1 as ::core::ffi::c_int)
                            as uint8_t as uint8_t;
                        hintBase = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut j: uint16_t = (*stack).index.wrapping_rem(2 as arity_t) as uint16_t;
                        while (j as arity_t) < (*stack).index {
                            let mut pos: ::core::ffi::c_double =
                                (*(*stack).stack.offset(j as isize)).c2rust_unnamed.d;
                            let mut width: ::core::ffi::c_double = (*(*stack).stack.offset(
                                (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .c2rust_unnamed
                            .d;
                            setHint.expect("non-null function pointer")(
                                outline,
                                val.c2rust_unnamed.i == op_vstem as ::core::ffi::c_int as int32_t
                                    || val.c2rust_unnamed.i
                                        == op_vstemhm as ::core::ffi::c_int as int32_t,
                                pos + hintBase,
                                width,
                            );
                            hintBase += pos + width;
                            j = (j as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint16_t;
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    19 | 20 => {
                        if (*stack).index.wrapping_rem(2 as arity_t) != 0 {
                            setWidth.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                        }
                        let mut isVertical: bool =
                            (*stack).stem as ::core::ffi::c_int > 0 as ::core::ffi::c_int;
                        (*stack).stem = ((*stack).stem as arity_t)
                            .wrapping_add((*stack).index >> 1 as ::core::ffi::c_int)
                            as uint8_t as uint8_t;
                        let mut hintBase_0: ::core::ffi::c_double =
                            0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut j_0: uint16_t =
                            (*stack).index.wrapping_rem(2 as arity_t) as uint16_t;
                        while (j_0 as arity_t) < (*stack).index {
                            let mut pos_0: ::core::ffi::c_double =
                                (*(*stack).stack.offset(j_0 as isize)).c2rust_unnamed.d;
                            let mut width_0: ::core::ffi::c_double = (*(*stack).stack.offset(
                                (j_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .c2rust_unnamed
                            .d;
                            setHint.expect("non-null function pointer")(
                                outline,
                                isVertical,
                                pos_0 + hintBase_0,
                                width_0,
                            );
                            hintBase_0 += pos_0 + width_0;
                            j_0 = (j_0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint16_t;
                        }
                        let mut maskLength: uint32_t =
                            ((*stack).stem as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int) as uint32_t;
                        let mut mask: *mut bool = ::core::ptr::null_mut::<bool>();
                        mask = __caryll_allocate_clean(
                            (::core::mem::size_of::<bool>() as size_t).wrapping_mul(
                                ((*stack).stem as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
                                    as size_t,
                            ),
                            405 as ::core::ffi::c_ulong,
                        ) as *mut bool;
                        let mut byte: uint32_t = 0 as uint32_t;
                        while byte < maskLength {
                            let mut maskByte: uint8_t =
                                *start.offset(advance.wrapping_add(byte) as isize);
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(0 as uint32_t)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 7 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(1 as uint32_t)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(2 as uint32_t)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 5 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(3 as uint32_t)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(4 as uint32_t)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(5 as uint32_t)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(6 as uint32_t)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(7 as uint32_t)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 0 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            byte = byte.wrapping_add(1);
                        }
                        setMask.expect("non-null function pointer")(
                            outline,
                            val.c2rust_unnamed.i == op_cntrmask as ::core::ffi::c_int as int32_t,
                            mask,
                        );
                        advance = advance.wrapping_add(maskLength);
                        (*stack).index = 0 as arity_t;
                    }
                    4 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_vmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_vmoveto as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            if (*stack).index > 1 as arity_t {
                                setWidth
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            newContour.expect("non-null function pointer")(outline);
                            lineTo.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    21 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_rmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_rmoveto as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            if (*stack).index > 2 as arity_t {
                                setWidth
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            newContour.expect("non-null function pointer")(outline);
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    22 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_hmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_hmoveto as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            if (*stack).index > 1 as arity_t {
                                setWidth
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            newContour.expect("non-null function pointer")(outline);
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    14 => {
                        if (*stack).index > 0 as arity_t {
                            setWidth.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                    }
                    5 => {
                        i = 0 as uint32_t;
                        while i < (*stack).index {
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(1 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            i = i.wrapping_add(2 as uint32_t);
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    7 => {
                        if (*stack).index.wrapping_rem(2 as arity_t) == 1 as arity_t {
                            lineTo.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            i = 1 as uint32_t;
                            while i < (*stack).index {
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                );
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                                i = i.wrapping_add(2 as uint32_t);
                            }
                        } else {
                            i = 0 as uint32_t;
                            while i < (*stack).index {
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                );
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                                i = i.wrapping_add(2 as uint32_t);
                            }
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    6 => {
                        if (*stack).index.wrapping_rem(2 as arity_t) == 1 as arity_t {
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            i = 1 as uint32_t;
                            while i < (*stack).index {
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                );
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                                i = i.wrapping_add(2 as uint32_t);
                            }
                        } else {
                            i = 0 as uint32_t;
                            while i < (*stack).index {
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                );
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                                i = i.wrapping_add(2 as uint32_t);
                            }
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    8 => {
                        i = 0 as uint32_t;
                        while i < (*stack).index {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(1 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(2 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(3 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(4 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(5 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            i = i.wrapping_add(6 as uint32_t);
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    24 => {
                        i = 0 as uint32_t;
                        while i < (*stack).index.wrapping_sub(2 as arity_t) {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(1 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(2 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(3 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(4 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(5 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            i = i.wrapping_add(6 as uint32_t);
                        }
                        lineTo.expect("non-null function pointer")(
                            outline,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                        );
                        (*stack).index = 0 as arity_t;
                    }
                    25 => {
                        i = 0 as uint32_t;
                        while i < (*stack).index.wrapping_sub(6 as arity_t) {
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(1 as uint32_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            i = i.wrapping_add(2 as uint32_t);
                        }
                        curveTo.expect("non-null function pointer")(
                            outline,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(6 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(5 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                        );
                        (*stack).index = 0 as arity_t;
                    }
                    26 => {
                        if (*stack).index.wrapping_rem(4 as arity_t) == 1 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            i = 5 as uint32_t;
                            while i < (*stack).index {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                                i = i.wrapping_add(4 as uint32_t);
                            }
                        } else {
                            i = 0 as uint32_t;
                            while i < (*stack).index {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                                i = i.wrapping_add(4 as uint32_t);
                            }
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    27 => {
                        if (*stack).index.wrapping_rem(4 as arity_t) == 1 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            i = 5 as uint32_t;
                            while i < (*stack).index {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                                i = i.wrapping_add(4 as uint32_t);
                            }
                        } else {
                            i = 0 as uint32_t;
                            while i < (*stack).index {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                                i = i.wrapping_add(4 as uint32_t);
                            }
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    30 => {
                        if (*stack).index.wrapping_rem(4 as arity_t) == 1 as arity_t {
                            cnt_bezier = (*stack)
                                .index
                                .wrapping_sub(5 as arity_t)
                                .wrapping_div(4 as arity_t)
                                as uint32_t;
                        } else {
                            cnt_bezier = (*stack).index.wrapping_div(4 as arity_t) as uint32_t;
                        }
                        i = 0 as uint32_t;
                        while i < (4 as uint32_t).wrapping_mul(cnt_bezier) {
                            if i.wrapping_div(4 as uint32_t).wrapping_rem(2 as uint32_t)
                                == 0 as uint32_t
                            {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                            } else {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                            }
                            i = i.wrapping_add(4 as uint32_t);
                        }
                        if (*stack).index.wrapping_rem(8 as arity_t) == 5 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        if (*stack).index.wrapping_rem(8 as arity_t) == 1 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    31 => {
                        if (*stack).index.wrapping_rem(4 as arity_t) == 1 as arity_t {
                            cnt_bezier = (*stack)
                                .index
                                .wrapping_sub(5 as arity_t)
                                .wrapping_div(4 as arity_t)
                                as uint32_t;
                        } else {
                            cnt_bezier = (*stack).index.wrapping_div(4 as arity_t) as uint32_t;
                        }
                        i = 0 as uint32_t;
                        while i < (4 as uint32_t).wrapping_mul(cnt_bezier) {
                            if i.wrapping_div(4 as uint32_t).wrapping_rem(2 as uint32_t)
                                == 0 as uint32_t
                            {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                            } else {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as uint32_t) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                            }
                            i = i.wrapping_add(4 as uint32_t);
                        }
                        if (*stack).index.wrapping_rem(8 as arity_t) == 5 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        if (*stack).index.wrapping_rem(8 as arity_t) == 1 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    3106 => {
                        if (*stack).index < 7 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_hflex\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_hflex as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                -(*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    3107 => {
                        if (*stack).index < 12 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_flex\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_flex as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(7 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(8 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(9 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(10 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(11 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    3108 => {
                        if (*stack).index < 9 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_hflex1\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_hflex1 as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(7 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(8 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                -((*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d
                                    + (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(7 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d),
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    3109 => {
                        if (*stack).index < 11 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_flex1\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_flex1 as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut dx: ::core::ffi::c_double =
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d
                                    + (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(6 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(8 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d;
                            let mut dy: ::core::ffi::c_double =
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d
                                    + (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(5 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(7 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(9 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d;
                            if fabs(dx) > fabs(dy) {
                                dx = (*(*stack).stack.offset(10 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d;
                                dy = -dy;
                            } else {
                                dx = -dx;
                                dy = (*(*stack).stack.offset(10 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d;
                            }
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(7 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(8 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(9 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                dx,
                                dy,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    3075 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_and\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_and as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if num1 != 0. && num2 != 0. {
                                1.0f64
                            } else {
                                0.0f64
                            };
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3076 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_or\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_or as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num1_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if num1_0 != 0. || num2_0 != 0. {
                                1.0f64
                            } else {
                                0.0f64
                            };
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3077 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_not\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_not as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if num != 0. { 0.0f64 } else { 1.0f64 };
                        }
                    }
                    3081 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_abs\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_abs as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if num_0 < 0.0f64 { -num_0 } else { num_0 };
                        }
                    }
                    3082 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_add\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_add as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num1_1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num1_1 + num2_1;
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3083 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_sub\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_sub as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num1_2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num1_2 - num2_2;
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3084 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_div\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_div as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num1_3: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_3: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num1_3 / num2_3;
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3086 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_neg\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_neg as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num_1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = -num_1;
                        }
                    }
                    3087 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_eq\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_eq as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num1_4: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_4: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if num1_4 == num2_4 { 1.0f64 } else { 0.0f64 };
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3090 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_drop\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_drop as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3092 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_put\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_put as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut val_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut i_0: int32_t = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d as int32_t;
                            (*stack).transient[(i_0
                                % type2_transient_array as ::core::ffi::c_int as int32_t)
                                as usize]
                                .c2rust_unnamed
                                .d = val_0;
                            (*stack).index = (*stack).index.wrapping_sub(2 as arity_t);
                        }
                    }
                    3093 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_get\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_get as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut i_1: int32_t = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d as int32_t;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = (*stack).transient[(i_1
                                % type2_transient_array as ::core::ffi::c_int as int32_t)
                                as usize]
                                .c2rust_unnamed
                                .d;
                        }
                    }
                    3094 => {
                        if (*stack).index < 4 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_ifelse\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_ifelse as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut v2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut v1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut s2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut s1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if v1 <= v2 { s1 } else { s2 };
                            (*stack).index = (*stack).index.wrapping_sub(3 as arity_t);
                        }
                    }
                    3095 => {
                        (*(*stack).stack.offset((*stack).index as isize)).t = cff_DOUBLE;
                        (*(*stack).stack.offset((*stack).index as isize))
                            .c2rust_unnamed
                            .d = getrand.expect("non-null function pointer")(outline);
                        (*stack).index = (*stack).index.wrapping_add(1 as arity_t);
                    }
                    3096 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_mul\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_mul as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num1_5: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_5: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num1_5 * num2_5;
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3098 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_sqrt\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_sqrt as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num_2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = sqrt(num_2);
                        }
                    }
                    3099 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_dup\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_dup as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            *(*stack).stack.offset((*stack).index as isize) = *(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize);
                            (*stack).index = (*stack).index.wrapping_add(1 as arity_t);
                        }
                    }
                    3100 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_exch\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_exch as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut num1_6: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_6: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num2_6;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num1_6;
                        }
                    }
                    3101 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_index\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_index as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut n: uint8_t =
                                (*stack).index.wrapping_sub(1 as arity_t) as uint8_t;
                            let mut j_1: uint8_t = (n as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - (*(*stack).stack.offset(n as isize)).c2rust_unnamed.d as uint8_t
                                    as ::core::ffi::c_int
                                    % n as ::core::ffi::c_int)
                                as uint8_t;
                            *(*stack).stack.offset(n as isize) =
                                *(*stack).stack.offset(j_1 as isize);
                        }
                    }
                    3102 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_roll\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_roll as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            let mut j_2: int32_t = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d as int32_t;
                            let mut n_0: uint32_t = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d as uint32_t;
                            if (*stack).index < (2 as uint32_t).wrapping_add(n_0) {
                                (*(*options).logger)
                                    .logSDS
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*options).logger as *mut otfcc_ILogger,
                                    log_vl_important as ::core::ffi::c_int as uint8_t,
                                    log_type_warning,
                                    sdscatprintf(
                                        sdsempty(),
                                        b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        b"op_roll\0" as *const u8 as *const ::core::ffi::c_char,
                                        op_roll as ::core::ffi::c_int,
                                    ),
                                );
                            } else {
                                j_2 = (-j_2 as uint32_t).wrapping_rem(n_0) as int32_t;
                                if j_2 < 0 as int32_t {
                                    j_2 = (j_2 as uint32_t).wrapping_add(n_0) as int32_t as int32_t;
                                }
                                if !(j_2 == 0) {
                                    let mut last: uint8_t =
                                        (*stack).index.wrapping_sub(3 as arity_t) as uint8_t;
                                    let mut first: uint8_t = (*stack)
                                        .index
                                        .wrapping_sub(2 as arity_t)
                                        .wrapping_sub(n_0 as arity_t)
                                        as uint8_t;
                                    reverseStack(stack, first, last);
                                    reverseStack(
                                        stack,
                                        (last as int32_t - j_2 + 1 as int32_t) as uint8_t,
                                        last,
                                    );
                                    reverseStack(stack, first, (last as int32_t - j_2) as uint8_t);
                                    (*stack).index = (*stack).index.wrapping_sub(2 as arity_t);
                                }
                            }
                        }
                    }
                    11 => return,
                    10 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_callsubr\0" as *const u8 as *const ::core::ffi::c_char,
                                    op_callsubr as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1);
                            let mut subr: uint32_t =
                                (*(*stack).stack.offset((*stack).index as isize))
                                    .c2rust_unnamed
                                    .d as uint32_t;
                            cff_parseOutline(
                                lsubr
                                    .data
                                    .offset(
                                        *lsubr
                                            .offset
                                            .offset((lsubr_bias as uint32_t).wrapping_add(subr)
                                                as isize)
                                            as isize,
                                    )
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                (*lsubr.offset.offset(
                                    (lsubr_bias as uint32_t)
                                        .wrapping_add(subr)
                                        .wrapping_add(1 as uint32_t)
                                        as isize,
                                ))
                                .wrapping_sub(
                                    *lsubr.offset.offset(
                                        (lsubr_bias as uint32_t).wrapping_add(subr) as isize,
                                    ),
                                ),
                                gsubr,
                                lsubr,
                                stack,
                                outline,
                                methods,
                                options,
                            );
                        }
                    }
                    29 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as uint8_t,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for %s (%04x). This operation is ignored.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    b"op_callgsubr\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    op_callgsubr as ::core::ffi::c_int,
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1);
                            let mut subr_0: uint32_t =
                                (*(*stack).stack.offset((*stack).index as isize))
                                    .c2rust_unnamed
                                    .d as uint32_t;
                            cff_parseOutline(
                                gsubr
                                    .data
                                    .offset(*gsubr.offset.offset(
                                        (gsubr_bias as uint32_t).wrapping_add(subr_0) as isize,
                                    ) as isize)
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                (*gsubr.offset.offset(
                                    (gsubr_bias as uint32_t)
                                        .wrapping_add(subr_0)
                                        .wrapping_add(1 as uint32_t)
                                        as isize,
                                ))
                                .wrapping_sub(
                                    *gsubr.offset.offset(
                                        (gsubr_bias as uint32_t).wrapping_add(subr_0) as isize,
                                    ),
                                ),
                                gsubr,
                                lsubr,
                                stack,
                                outline,
                                methods,
                                options,
                            );
                        }
                    }
                    _ => {
                        (*(*options).logger)
                            .logSDS
                            .expect(
                                "non-null function pointer",
                            )(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as uint8_t,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Warning: unknown operator %d occurs in Type 2 CharString. It may caused by file corruption.\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                val.c2rust_unnamed.i,
                            ),
                        );
                        return;
                    }
                }
            }
            2 | 3 => {
                let fresh0 = (*stack).index;
                (*stack).index = (*stack).index.wrapping_add(1);
                *(*stack).stack.offset(fresh0 as isize) = val;
            }
            _ => {}
        }
        start = start.offset(advance as isize);
    }
}
