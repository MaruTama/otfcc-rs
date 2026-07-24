extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: size_t) -> sds;
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscat(s: sds, t: *const ::core::ffi::c_char) -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufninit(n: uint32_t, ...) -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn bufwrite_sds(buf: *mut caryll_Buffer, str: sds);
    fn bufwrite_bufdel(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
    static iVQ: __caryll_vectorinterface_VQ;
    static glyf_iPoint: __caryll_elementinterface_glyf_Point;
    static glyf_iContour: __caryll_vectorinterface_glyf_Contour;
    static glyf_iContourList: __caryll_vectorinterface_glyf_ContourList;
    static glyf_iStemDefList: __caryll_vectorinterface_glyf_StemDefList;
    static glyf_iMaskList: __caryll_vectorinterface_glyf_MaskList;
    static table_iGlyf: __caryll_vectorinterface_table_glyf;
    fn otfcc_newGlyf_glyph() -> *mut glyf_Glyph;
    fn json_array_new(length: size_t) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: size_t) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: int64_t) -> *mut json_value;
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn json_new_VQ(z: VQ, fvar: *const table_fvar) -> *mut json_value;
    fn cffnum(v: cff_Value) -> ::core::ffi::c_double;
    static cff_iIndex: __caryll_elementinterface_cff_Index;
    static cff_iDict: __caryll_elementinterface_cff_Dict;
    fn cff_build_Charset(cset: cff_Charset) -> *mut caryll_Buffer;
    fn cff_close_FDSelect(fds: cff_FDSelect);
    fn cff_build_FDSelect(fd: cff_FDSelect) -> *mut caryll_Buffer;
    fn sdsget_cff_sid(idx: uint16_t, str: cff_Index) -> sds;
    fn cff_encodeCffOperator(val: int32_t) -> *mut caryll_Buffer;
    fn cff_buildOffset(val: int32_t) -> *mut caryll_Buffer;
    fn cff_buildHeader() -> *mut caryll_Buffer;
    fn cff_parseSubr(
        idx: uint16_t,
        raw: *mut uint8_t,
        fdarray: cff_Index,
        select: cff_FDSelect,
        subr: *mut cff_Index,
    ) -> uint8_t;
    fn cff_parseOutline(
        data: *mut uint8_t,
        len: uint32_t,
        gsubr: cff_Index,
        lsubr: cff_Index,
        stack: *mut cff_Stack,
        outline: *mut ::core::ffi::c_void,
        methods: cff_IOutlineBuilder,
        options: *const otfcc_Options,
    );
    fn cff_openStream(
        data: *mut uint8_t,
        len: uint32_t,
        options: *const otfcc_Options,
    ) -> *mut cff_File;
    fn cff_close(file: *mut cff_File);
    fn cff_compileGlyphToIL(
        g: *mut glyf_Glyph,
        defaultWidth: uint16_t,
        nominalWidth: uint16_t,
    ) -> *mut cff_CharstringIL;
    fn cff_optimizeIL(il: *mut cff_CharstringIL, options: *const otfcc_Options);
    static cff_iSubrGraph: __caryll_elementinterface_cff_SubrGraph;
    fn cff_insertILToGraph(g: *mut cff_SubrGraph, il: *mut cff_CharstringIL);
    fn cff_ilGraphToBuffers(
        g: *mut cff_SubrGraph,
        s: *mut *mut caryll_Buffer,
        gs: *mut *mut caryll_Buffer,
        ls: *mut *mut caryll_Buffer,
        options: *const otfcc_Options,
    );
}

use crate::support::handle::{handle_fromIndex, otfcc_Handle, otfcc_GlyphHandle};
use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut ::core::ffi::c_void,
    pub __gr_top: *mut ::core::ffi::c_void,
    pub __vr_top: *mut ::core::ffi::c_void,
    pub __gr_offs: ::core::ffi::c_int,
    pub __vr_offs: ::core::ffi::c_int,
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type ptrdiff_t = isize;
pub type f16dot16 = int32_t;
pub type glyphid_t = uint16_t;
pub type tableid_t = uint16_t;
pub type shapeid_t = uint16_t;
pub type cffsid_t = uint16_t;
pub type arity_t = uint32_t;
pub type pos_t = ::core::ffi::c_double;
pub type scale_t = ::core::ffi::c_double;
pub type otfcc_FDHandle = otfcc_Handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: ::core::ffi::c_uint,
    pub expand_mult: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut ::core::ffi::c_void,
    pub next: *mut ::core::ffi::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut ::core::ffi::c_void,
    pub keylen: ::core::ffi::c_uint,
    pub hashv: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: ::core::ffi::c_uint,
    pub log2_num_buckets: ::core::ffi::c_uint,
    pub num_items: ::core::ffi::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: ::core::ffi::c_uint,
    pub nonideal_items: ::core::ffi::c_uint,
    pub ineff_expands: ::core::ffi::c_uint,
    pub noexpand: ::core::ffi::c_uint,
    pub signature: uint32_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_PacketPiece {
    pub tag: uint32_t,
    pub checkSum: uint32_t,
    pub offset: uint32_t,
    pub length: uint32_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Packet {
    pub sfnt_version: uint32_t,
    pub numTables: uint16_t,
    pub searchRange: uint16_t,
    pub entrySelector: uint16_t,
    pub rangeShift: uint16_t,
    pub pieces: *mut otfcc_PacketPiece,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VV {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_AxisSpan {
    pub start: pos_t,
    pub peak: pos_t,
    pub end: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_Region {
    pub dimensions: shapeid_t,
    pub spans: [vq_AxisSpan; 0],
}
pub type VQSegType = ::core::ffi::c_uint;
pub const VQ_DELTA: VQSegType = 1;
pub const VQ_STILL: VQSegType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_Segment {
    pub type_0: VQSegType,
    pub val: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub still: pos_t,
    pub delta: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub quantity: pos_t,
    pub touched: bool,
    pub region: *const vq_Region,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_SegList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut vq_Segment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VQ {
    pub kernel: pos_t,
    pub shift: vq_SegList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_VQ {
    pub init: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VQ, *const VQ) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VQ, *mut VQ) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> VQ>,
    pub dup: Option<unsafe extern "C" fn(VQ) -> VQ>,
    pub neutral: Option<unsafe extern "C" fn() -> VQ>,
    pub plus: Option<unsafe extern "C" fn(VQ, VQ) -> VQ>,
    pub inplacePlus: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub inplaceNegate: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub negate: Option<unsafe extern "C" fn(VQ) -> VQ>,
    pub inplaceMinus: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub minus: Option<unsafe extern "C" fn(VQ, VQ) -> VQ>,
    pub inplaceScale: Option<unsafe extern "C" fn(*mut VQ, scale_t) -> ()>,
    pub inplacePlusScale: Option<unsafe extern "C" fn(*mut VQ, scale_t, VQ) -> ()>,
    pub scale: Option<unsafe extern "C" fn(VQ, scale_t) -> VQ>,
    pub equal: Option<unsafe extern "C" fn(VQ, VQ) -> bool>,
    pub compare: Option<unsafe extern "C" fn(VQ, VQ) -> ::core::ffi::c_int>,
    pub compareRef: Option<unsafe extern "C" fn(*const VQ, *const VQ) -> ::core::ffi::c_int>,
    pub show: Option<unsafe extern "C" fn(VQ) -> ()>,
    pub getStill: Option<unsafe extern "C" fn(VQ) -> pos_t>,
    pub createStill: Option<unsafe extern "C" fn(pos_t) -> VQ>,
    pub isStill: Option<unsafe extern "C" fn(VQ) -> bool>,
    pub isZero: Option<unsafe extern "C" fn(VQ, pos_t) -> bool>,
    pub pointLinearTfm: Option<unsafe extern "C" fn(VQ, pos_t, VQ, pos_t, VQ) -> VQ>,
    pub addDelta: Option<unsafe extern "C" fn(*mut VQ, bool, *const vq_Region, pos_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_head {
    pub version: f16dot16,
    pub fontRevision: uint32_t,
    pub checkSumAdjustment: uint32_t,
    pub magicNumber: uint32_t,
    pub flags: uint16_t,
    pub unitsPerEm: uint16_t,
    pub created: int64_t,
    pub modified: int64_t,
    pub xMin: int16_t,
    pub yMin: int16_t,
    pub xMax: int16_t,
    pub yMax: int16_t,
    pub macStyle: uint16_t,
    pub lowestRecPPEM: uint16_t,
    pub fontDirectoryHint: int16_t,
    pub indexToLocFormat: int16_t,
    pub glyphDataFormat: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vf_Axis {
    pub tag: uint32_t,
    pub minValue: pos_t,
    pub defaultValue: pos_t,
    pub maxValue: pos_t,
    pub flags: uint16_t,
    pub axisNameID: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vf_Axes {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut vf_Axis,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fvar_Instance {
    pub subfamilyNameID: uint16_t,
    pub flags: uint16_t,
    pub coordinates: VV,
    pub postScriptNameID: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fvar_InstanceList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut fvar_Instance,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fvar_Master {
    pub name: sds,
    pub region: *mut vq_Region,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_fvar {
    pub majorVersion: uint16_t,
    pub minorVersion: uint16_t,
    pub axes: vf_Axes,
    pub instances: fvar_InstanceList,
    pub masters: *mut fvar_Master,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Point {
    pub x: VQ,
    pub y: VQ,
    pub onCurve: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_glyf_Point {
    pub init: Option<unsafe extern "C" fn(*mut glyf_Point) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_Point, *const glyf_Point) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_Point, *mut glyf_Point) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_Point) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_Point, glyf_Point) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_Point, glyf_Point) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> glyf_Point>,
    pub dup: Option<unsafe extern "C" fn(glyf_Point) -> glyf_Point>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Contour {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_Point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_glyf_Contour {
    pub init: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_Contour, *const glyf_Contour) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_Contour, *mut glyf_Contour) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_Contour, glyf_Contour) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_Contour, glyf_Contour) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_Contour>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_Contour, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_Contour, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut glyf_Contour>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_Contour, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_Contour, glyf_Point) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_Contour) -> glyf_Point>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_Contour, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_Contour,
            Option<unsafe extern "C" fn(*const glyf_Point, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_Contour,
            Option<unsafe extern "C" fn(*const glyf_Point, *const glyf_Point) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ContourList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_Contour,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_glyf_ContourList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_ContourList, *const glyf_ContourList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_ContourList, *mut glyf_ContourList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_ContourList, glyf_ContourList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_ContourList, glyf_ContourList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_ContourList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_ContourList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_ContourList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut glyf_ContourList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_ContourList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_ContourList, glyf_Contour) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> glyf_Contour>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_ContourList, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_ContourList,
            Option<unsafe extern "C" fn(*const glyf_Contour, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_ContourList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_Contour,
                    *const glyf_Contour,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PostscriptStemDef {
    pub position: pos_t,
    pub width: pos_t,
    pub map: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_StemDefList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_PostscriptStemDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_glyf_StemDefList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_StemDefList, *const glyf_StemDefList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_StemDefList, *mut glyf_StemDefList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_StemDefList, glyf_StemDefList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_StemDefList, glyf_StemDefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_StemDefList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_StemDefList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_StemDefList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut glyf_StemDefList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_StemDefList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_StemDefList, glyf_PostscriptStemDef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> glyf_PostscriptStemDef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_StemDefList, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_StemDefList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_PostscriptStemDef,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_StemDefList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_PostscriptStemDef,
                    *const glyf_PostscriptStemDef,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PostscriptHintMask {
    pub pointsBefore: uint16_t,
    pub contoursBefore: uint16_t,
    pub maskH: [bool; 256],
    pub maskV: [bool; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_MaskList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_PostscriptHintMask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_glyf_MaskList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_MaskList, *const glyf_MaskList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_MaskList, *mut glyf_MaskList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_MaskList, glyf_MaskList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_MaskList, glyf_MaskList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_MaskList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_MaskList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_MaskList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut glyf_MaskList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_MaskList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_MaskList, glyf_PostscriptHintMask) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> glyf_PostscriptHintMask>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_MaskList, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_MaskList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_PostscriptHintMask,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_MaskList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_PostscriptHintMask,
                    *const glyf_PostscriptHintMask,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
pub type RefAnchorStatus = ::core::ffi::c_uint;
pub const REF_ANCHOR_CONSOLIDATING_XY: RefAnchorStatus = 5;
pub const REF_ANCHOR_CONSOLIDATING_ANCHOR: RefAnchorStatus = 4;
pub const REF_ANCHOR_CONSOLIDATED: RefAnchorStatus = 3;
pub const REF_ANCHOR_XY: RefAnchorStatus = 2;
pub const REF_ANCHOR_ANCHOR: RefAnchorStatus = 1;
pub const REF_XY: RefAnchorStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ComponentReference {
    pub x: VQ,
    pub y: VQ,
    pub roundToGrid: bool,
    pub useMyMetrics: bool,
    pub glyph: otfcc_GlyphHandle,
    pub a: scale_t,
    pub b: scale_t,
    pub c: scale_t,
    pub d: scale_t,
    pub isAnchored: RefAnchorStatus,
    pub inner: shapeid_t,
    pub outer: shapeid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ReferenceList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_ComponentReference,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_GlyphStat {
    pub xMin: pos_t,
    pub xMax: pos_t,
    pub yMin: pos_t,
    pub yMax: pos_t,
    pub nestDepth: uint16_t,
    pub nPoints: uint16_t,
    pub nContours: uint16_t,
    pub nCompositePoints: uint16_t,
    pub nCompositeContours: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Glyph {
    pub name: sds,
    pub horizontalOrigin: VQ,
    pub advanceWidth: VQ,
    pub verticalOrigin: VQ,
    pub advanceHeight: VQ,
    pub contours: glyf_ContourList,
    pub references: glyf_ReferenceList,
    pub stemH: glyf_StemDefList,
    pub stemV: glyf_StemDefList,
    pub hintMasks: glyf_MaskList,
    pub contourMasks: glyf_MaskList,
    pub instructionsLength: uint16_t,
    pub instructions: *mut uint8_t,
    pub yPel: uint8_t,
    pub fdSelect: otfcc_FDHandle,
    pub cid: glyphid_t,
    pub stat: glyf_GlyphStat,
}
pub type glyf_GlyphPtr = *mut glyf_Glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_glyf {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_GlyphPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_table_glyf {
    pub init: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_glyf, *const table_glyf) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_glyf, *mut table_glyf) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_glyf, table_glyf) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_glyf, table_glyf) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_glyf>,
    pub free: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_glyf, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_glyf, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut table_glyf>,
    pub fill: Option<unsafe extern "C" fn(*mut table_glyf, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_glyf, glyf_GlyphPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_glyf) -> glyf_GlyphPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_glyf, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_glyf,
            Option<unsafe extern "C" fn(*const glyf_GlyphPtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_glyf,
            Option<
                unsafe extern "C" fn(
                    *const glyf_GlyphPtr,
                    *const glyf_GlyphPtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FontMatrix {
    pub a: scale_t,
    pub b: scale_t,
    pub c: scale_t,
    pub d: scale_t,
    pub x: VQ,
    pub y: VQ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_PrivateDict {
    pub blueValuesCount: arity_t,
    pub blueValues: *mut ::core::ffi::c_double,
    pub otherBluesCount: arity_t,
    pub otherBlues: *mut ::core::ffi::c_double,
    pub familyBluesCount: arity_t,
    pub familyBlues: *mut ::core::ffi::c_double,
    pub familyOtherBluesCount: arity_t,
    pub familyOtherBlues: *mut ::core::ffi::c_double,
    pub blueScale: ::core::ffi::c_double,
    pub blueShift: ::core::ffi::c_double,
    pub blueFuzz: ::core::ffi::c_double,
    pub stdHW: ::core::ffi::c_double,
    pub stdVW: ::core::ffi::c_double,
    pub stemSnapHCount: arity_t,
    pub stemSnapH: *mut ::core::ffi::c_double,
    pub stemSnapVCount: arity_t,
    pub stemSnapV: *mut ::core::ffi::c_double,
    pub forceBold: bool,
    pub languageGroup: uint32_t,
    pub expansionFactor: ::core::ffi::c_double,
    pub initialRandomSeed: ::core::ffi::c_double,
    pub defaultWidthX: ::core::ffi::c_double,
    pub nominalWidthX: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _table_CFF {
    pub fontName: sds,
    pub isCID: bool,
    pub version: sds,
    pub notice: sds,
    pub copyright: sds,
    pub fullName: sds,
    pub familyName: sds,
    pub weight: sds,
    pub isFixedPitch: bool,
    pub italicAngle: ::core::ffi::c_double,
    pub underlinePosition: ::core::ffi::c_double,
    pub underlineThickness: ::core::ffi::c_double,
    pub fontBBoxTop: ::core::ffi::c_double,
    pub fontBBoxBottom: ::core::ffi::c_double,
    pub fontBBoxLeft: ::core::ffi::c_double,
    pub fontBBoxRight: ::core::ffi::c_double,
    pub strokeWidth: ::core::ffi::c_double,
    pub privateDict: *mut cff_PrivateDict,
    pub fontMatrix: *mut cff_FontMatrix,
    pub cidRegistry: sds,
    pub cidOrdering: sds,
    pub cidSupplement: uint32_t,
    pub cidFontVersion: ::core::ffi::c_double,
    pub cidFontRevision: ::core::ffi::c_double,
    pub cidCount: uint32_t,
    pub UIDBase: uint32_t,
    pub fdArrayCount: tableid_t,
    pub fdArray: *mut *mut table_CFF,
}
pub type table_CFF = _table_CFF;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_CFF {
    pub init: Option<unsafe extern "C" fn(*mut table_CFF) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_CFF, *const table_CFF) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_CFF, *mut table_CFF) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_CFF) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_CFF, table_CFF) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_CFF, table_CFF) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_CFF>,
    pub free: Option<unsafe extern "C" fn(*mut table_CFF) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_CFFAndGlyf {
    pub meta: *mut table_CFF,
    pub glyphs: *mut table_glyf,
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
pub struct cff_Index {
    pub countType: cff_IndexCountType,
    pub count: arity_t,
    pub offSize: uint8_t,
    pub offset: *mut uint32_t,
    pub data: *mut uint8_t,
}
pub type cff_IndexCountType = ::core::ffi::c_uint;
pub const CFF_INDEX_32: cff_IndexCountType = 1;
pub const CFF_INDEX_16: cff_IndexCountType = 0;
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
pub struct cff_FDSelectRangeFormat3 {
    pub first: uint16_t,
    pub fd: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelectFormat0 {
    pub format: uint8_t,
    pub fds: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Charset {
    pub t: uint32_t,
    pub s: uint32_t,
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub f0: cff_CharsetFormat0,
    pub f1: cff_CharsetFormat1,
    pub f2: cff_CharsetFormat2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat2 {
    pub format: uint8_t,
    pub range2: *mut cff_CharsetRangeFormat2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetRangeFormat2 {
    pub first: uint16_t,
    pub nleft: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat1 {
    pub format: uint8_t,
    pub range1: *mut cff_CharsetRangeFormat1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetRangeFormat1 {
    pub first: uint16_t,
    pub nleft: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat0 {
    pub format: uint8_t,
    pub glyph: *mut uint16_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingNS {
    pub nsup: uint8_t,
    pub supplement: *mut cff_EncodingSupplement,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingSupplement {
    pub code: uint8_t,
    pub glyph: uint16_t,
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
pub struct cff_EncodingRangeFormat1 {
    pub first: uint8_t,
    pub nleft: uint8_t,
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
pub struct cff_Header {
    pub major: uint8_t,
    pub minor: uint8_t,
    pub hdrSize: uint8_t,
    pub offSize: uint8_t,
}
pub type font_file_pointer = *mut uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_extract_context {
    pub fdArrayIndex: int32_t,
    pub meta: *mut table_CFF,
    pub glyphs: *mut table_glyf,
    pub cffFile: *mut cff_File,
    pub seed: uint64_t,
}
pub const cff_CHARSET_FORMAT2: C2RustUnnamed_13 = 5;
pub const cff_CHARSET_FORMAT1: C2RustUnnamed_13 = 4;
pub const cff_CHARSET_FORMAT0: C2RustUnnamed_13 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outline_builder_context {
    pub g: *mut glyf_Glyph,
    pub jContour: shapeid_t,
    pub jPoint: shapeid_t,
    pub defaultWidthX: ::core::ffi::c_double,
    pub nominalWidthX: ::core::ffi::c_double,
    pub definedHStems: uint8_t,
    pub definedVStems: uint8_t,
    pub definedHintMasks: uint8_t,
    pub definedContourMasks: uint8_t,
    pub randx: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Value {
    pub t: cff_Value_Type,
    pub c2rust_unnamed: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub i: int32_t,
    pub d: ::core::ffi::c_double,
}
pub type cff_Value_Type = ::core::ffi::c_uint;
pub const CS2_FRACTION: cff_Value_Type = 3;
pub const cff_DOUBLE: cff_Value_Type = 3;
pub const CS2_OPERAND: cff_Value_Type = 2;
pub const cff_INTEGER: cff_Value_Type = 2;
pub const CS2_OPERATOR: cff_Value_Type = 1;
pub const cff_OPERATOR: cff_Value_Type = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub u: uint64_t,
    pub d: ::core::ffi::c_double,
}
pub const cff_FDSELECT_UNSPECED: C2RustUnnamed_14 = 2;
pub const op_ROS: C2RustUnnamed_12 = 3102;
pub const op_nominalWidthX: C2RustUnnamed_12 = 21;
pub const op_defaultWidthX: C2RustUnnamed_12 = 20;
pub const op_initialRandomSeed: C2RustUnnamed_12 = 3091;
pub const op_ExpansionFactor: C2RustUnnamed_12 = 3090;
pub const op_LanguageGroup: C2RustUnnamed_12 = 3089;
pub const op_ForceBold: C2RustUnnamed_12 = 3086;
pub const op_StdVW: C2RustUnnamed_12 = 11;
pub const op_StdHW: C2RustUnnamed_12 = 10;
pub const op_BlueFuzz: C2RustUnnamed_12 = 3083;
pub const op_BlueShift: C2RustUnnamed_12 = 3082;
pub const op_BlueScale: C2RustUnnamed_12 = 3081;
pub const op_StemSnapV: C2RustUnnamed_12 = 3085;
pub const op_StemSnapH: C2RustUnnamed_12 = 3084;
pub const op_FamilyOtherBlues: C2RustUnnamed_12 = 9;
pub const op_FamilyBlues: C2RustUnnamed_12 = 8;
pub const op_OtherBlues: C2RustUnnamed_12 = 7;
pub const op_BlueValues: C2RustUnnamed_12 = 6;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Dict {
    pub count: uint32_t,
    pub ents: *mut cff_DictEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_DictEntry {
    pub op: uint32_t,
    pub cnt: uint32_t,
    pub vals: *mut cff_Value,
}
pub const op_Private: C2RustUnnamed_12 = 18;
pub const op_StrokeWidth: C2RustUnnamed_12 = 3080;
pub const op_UnderlineThickness: C2RustUnnamed_12 = 3076;
pub const op_UnderlinePosition: C2RustUnnamed_12 = 3075;
pub const op_ItalicAngle: C2RustUnnamed_12 = 3074;
pub const op_isFixedPitch: C2RustUnnamed_12 = 3073;
pub const op_FontMatrix: C2RustUnnamed_12 = 3079;
pub const op_FontBBox: C2RustUnnamed_12 = 5;
pub const op_Weight: C2RustUnnamed_12 = 4;
pub const op_FamilyName: C2RustUnnamed_12 = 3;
pub const op_FullName: C2RustUnnamed_12 = 2;
pub const op_FontName: C2RustUnnamed_12 = 3110;
pub const op_Copyright: C2RustUnnamed_12 = 3072;
pub const op_Notice: C2RustUnnamed_12 = 1;
pub const op_version: C2RustUnnamed_12 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_sid_entry {
    pub sid: ::core::ffi::c_int,
    pub str_0: *mut ::core::ffi::c_char,
    pub hh: UT_hash_handle,
}
pub const op_UIDBase: C2RustUnnamed_12 = 3107;
pub const op_CIDCount: C2RustUnnamed_12 = 3106;
pub const op_CIDFontRevision: C2RustUnnamed_12 = 3104;
pub const op_CIDFontVersion: C2RustUnnamed_12 = 3103;
pub const op_Subrs: C2RustUnnamed_12 = 19;
pub const cff_FDSELECT_FORMAT3: C2RustUnnamed_14 = 1;
pub const cff_CHARSET_ISOADOBE: C2RustUnnamed_13 = 0;
pub const op_FDArray: C2RustUnnamed_12 = 3108;
pub const op_CharStrings: C2RustUnnamed_12 = 17;
pub const op_FDSelect: C2RustUnnamed_12 = 3109;
pub const op_charset: C2RustUnnamed_12 = 15;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_SubrGraph {
    pub root: *mut cff_SubrRule,
    pub last: *mut cff_SubrRule,
    pub diagramIndex: *mut cff_SubrDiagramIndex,
    pub totalRules: uint32_t,
    pub totalCharStrings: uint32_t,
    pub doSubroutinize: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_SubrDiagramIndex {
    pub arity: uint8_t,
    pub key: *mut uint8_t,
    pub start: *mut cff_SubrNode,
    pub hh: UT_hash_handle,
}
pub type cff_SubrNode = __cff_SubrNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __cff_SubrNode {
    pub prev: *mut cff_SubrNode,
    pub rule: *mut cff_SubrRule,
    pub next: *mut cff_SubrNode,
    pub terminal: *mut caryll_Buffer,
    pub hard: bool,
    pub guard: bool,
    pub last: bool,
}
pub type cff_SubrRule = __cff_SubrRule;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __cff_SubrRule {
    pub printed: bool,
    pub numbered: bool,
    pub number: uint32_t,
    pub height: uint32_t,
    pub uniqueIndex: uint32_t,
    pub cffIndex: uint16_t,
    pub refcount: uint32_t,
    pub effectiveLength: uint32_t,
    pub guard: *mut cff_SubrNode,
    pub next: *mut cff_SubrRule,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_charstring_builder_context {
    pub glyf: *mut table_glyf,
    pub defaultWidth: uint16_t,
    pub nominalWidthX: uint16_t,
    pub options: *const otfcc_Options,
    pub graph: cff_SubrGraph,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_cff_SubrGraph {
    pub init: Option<unsafe extern "C" fn(*mut cff_SubrGraph) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut cff_SubrGraph, *const cff_SubrGraph) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut cff_SubrGraph, *mut cff_SubrGraph) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut cff_SubrGraph) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut cff_SubrGraph, cff_SubrGraph) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut cff_SubrGraph, cff_SubrGraph) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut cff_SubrGraph>,
    pub free: Option<unsafe extern "C" fn(*mut cff_SubrGraph) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharstringIL {
    pub length: uint32_t,
    pub free: uint32_t,
    pub instr: *mut cff_CharstringInstruction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharstringInstruction {
    pub type_0: cff_InstructionType,
    pub arity: arity_t,
    pub c2rust_unnamed: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub d: ::core::ffi::c_double,
    pub i: int32_t,
}
pub type cff_InstructionType = ::core::ffi::c_uint;
pub const IL_ITEM_PHANTOM_OPERAND: cff_InstructionType = 4;
pub const IL_ITEM_PHANTOM_OPERATOR: cff_InstructionType = 3;
pub const IL_ITEM_SPECIAL: cff_InstructionType = 2;
pub const IL_ITEM_OPERATOR: cff_InstructionType = 1;
pub const IL_ITEM_OPERAND: cff_InstructionType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdarray_compile_context {
    pub fdArray: *mut *mut table_CFF,
    pub stringHash: *mut *mut cff_sid_entry,
}
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const op_CIDFontType: C2RustUnnamed_12 = 3105;
pub const op_maxstack: C2RustUnnamed_12 = 25;
pub const op_vstore: C2RustUnnamed_12 = 24;
pub const op_BaseFontBlend: C2RustUnnamed_12 = 3095;
pub const op_blend: C2RustUnnamed_12 = 23;
pub const op_BaseFontName: C2RustUnnamed_12 = 3094;
pub const op_vsindex: C2RustUnnamed_12 = 22;
pub const op_PostScript: C2RustUnnamed_12 = 3093;
pub const op_SyntheicBase: C2RustUnnamed_12 = 3092;
pub const op_Encoding: C2RustUnnamed_12 = 16;
pub const op_XUID: C2RustUnnamed_12 = 14;
pub const op_UniqueID: C2RustUnnamed_12 = 13;
pub const op_CharstringType: C2RustUnnamed_12 = 3078;
pub const op_PaintType: C2RustUnnamed_12 = 3077;
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const cff_CHARSET_EXPERTSUBSET: C2RustUnnamed_13 = 2;
pub const cff_CHARSET_EXPERT: C2RustUnnamed_13 = 1;
pub const cff_CHARSET_UNSPECED: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const cff_FDSELECT_FORMAT0: C2RustUnnamed_14 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as size_t,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as size_t;
        }
        _ => {}
    }
    return 0 as size_t;
}
pub const HASH_INITIAL_NUM_BUCKETS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const HASH_INITIAL_NUM_BUCKETS_LOG2: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const HASH_BKT_CAPACITY_THRESH: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
pub const HASH_SIGNATURE: ::core::ffi::c_uint = 0xa0111fe1 as ::core::ffi::c_uint;
#[no_mangle]
pub static mut DEFAULT_BLUE_SCALE: ::core::ffi::c_double = 0.039625f64;
#[no_mangle]
pub static mut DEFAULT_BLUE_SHIFT: ::core::ffi::c_double =
    7 as ::core::ffi::c_int as ::core::ffi::c_double;
#[no_mangle]
pub static mut DEFAULT_BLUE_FUZZ: ::core::ffi::c_double =
    1 as ::core::ffi::c_int as ::core::ffi::c_double;
#[no_mangle]
pub static mut DEFAULT_EXPANSION_FACTOR: ::core::ffi::c_double = 0.06f64;
unsafe extern "C" fn otfcc_newCff_private() -> *mut cff_PrivateDict {
    let mut pd: *mut cff_PrivateDict = ::core::ptr::null_mut::<cff_PrivateDict>();
    pd = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_PrivateDict>() as size_t,
        15 as ::core::ffi::c_ulong,
    ) as *mut cff_PrivateDict;
    (*pd).blueFuzz = DEFAULT_BLUE_FUZZ;
    (*pd).blueScale = DEFAULT_BLUE_SCALE;
    (*pd).blueShift = DEFAULT_BLUE_SHIFT;
    (*pd).expansionFactor = DEFAULT_EXPANSION_FACTOR;
    return pd;
}
#[inline]
unsafe extern "C" fn initFD(mut fd: *mut table_CFF) {
    memset(
        fd as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_CFF>() as size_t,
    );
    (*fd).underlinePosition = -(100 as ::core::ffi::c_int) as ::core::ffi::c_double;
    (*fd).underlineThickness = 50 as ::core::ffi::c_int as ::core::ffi::c_double;
}
unsafe extern "C" fn otfcc_delete_privatedict(mut priv_0: *mut cff_PrivateDict) {
    if priv_0.is_null() {
        return;
    }
    free((*priv_0).blueValues as *mut ::core::ffi::c_void);
    (*priv_0).blueValues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((*priv_0).otherBlues as *mut ::core::ffi::c_void);
    (*priv_0).otherBlues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((*priv_0).familyBlues as *mut ::core::ffi::c_void);
    (*priv_0).familyBlues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((*priv_0).familyOtherBlues as *mut ::core::ffi::c_void);
    (*priv_0).familyOtherBlues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((*priv_0).stemSnapH as *mut ::core::ffi::c_void);
    (*priv_0).stemSnapH = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((*priv_0).stemSnapV as *mut ::core::ffi::c_void);
    (*priv_0).stemSnapV = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free(priv_0 as *mut ::core::ffi::c_void);
    priv_0 = ::core::ptr::null_mut::<cff_PrivateDict>();
}
#[inline]
unsafe extern "C" fn disposeFontMatrix(mut fm: *mut cff_FontMatrix) {
    if fm.is_null() {
        return;
    }
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*fm).x);
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*fm).y);
}
#[inline]
unsafe extern "C" fn disposeFD(mut fd: *mut table_CFF) {
    sdsfree((*fd).version);
    sdsfree((*fd).notice);
    sdsfree((*fd).copyright);
    sdsfree((*fd).fullName);
    sdsfree((*fd).familyName);
    sdsfree((*fd).weight);
    sdsfree((*fd).fontName);
    sdsfree((*fd).cidRegistry);
    sdsfree((*fd).cidOrdering);
    disposeFontMatrix((*fd).fontMatrix);
    free((*fd).fontMatrix as *mut ::core::ffi::c_void);
    (*fd).fontMatrix = ::core::ptr::null_mut::<cff_FontMatrix>();
    otfcc_delete_privatedict((*fd).privateDict);
    if !(*fd).fdArray.is_null() {
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*fd).fdArrayCount as ::core::ffi::c_int {
            table_iCFF.free.expect("non-null function pointer")(*(*fd).fdArray.offset(j as isize));
            j = j.wrapping_add(1);
        }
        free((*fd).fdArray as *mut ::core::ffi::c_void);
        (*fd).fdArray = ::core::ptr::null_mut::<*mut table_CFF>();
    }
}
#[inline]
unsafe extern "C" fn table_CFF_free(mut x: *mut table_CFF) {
    if x.is_null() {
        return;
    }
    table_CFF_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_CFF_replace(mut dst: *mut table_CFF, src: table_CFF) {
    table_CFF_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_CFF>() as size_t,
    );
}
#[no_mangle]
pub static mut table_iCFF: __caryll_elementinterface_table_CFF = {
    __caryll_elementinterface_table_CFF {
        init: Some(table_CFF_init as unsafe extern "C" fn(*mut table_CFF) -> ()),
        copy: Some(table_CFF_copy as unsafe extern "C" fn(*mut table_CFF, *const table_CFF) -> ()),
        move_0: Some(table_CFF_move as unsafe extern "C" fn(*mut table_CFF, *mut table_CFF) -> ()),
        dispose: Some(table_CFF_dispose as unsafe extern "C" fn(*mut table_CFF) -> ()),
        replace: Some(table_CFF_replace as unsafe extern "C" fn(*mut table_CFF, table_CFF) -> ()),
        copyReplace: Some(
            table_CFF_copyReplace as unsafe extern "C" fn(*mut table_CFF, table_CFF) -> (),
        ),
        create: Some(table_CFF_create),
        free: Some(table_CFF_free as unsafe extern "C" fn(*mut table_CFF) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_CFF_move(mut dst: *mut table_CFF, mut src: *mut table_CFF) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_CFF>() as size_t,
    );
    table_CFF_init(src);
}
#[inline]
unsafe extern "C" fn table_CFF_create() -> *mut table_CFF {
    let mut x: *mut table_CFF =
        malloc(::core::mem::size_of::<table_CFF>() as size_t) as *mut table_CFF;
    table_CFF_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_CFF_init(mut x: *mut table_CFF) {
    initFD(x);
}
#[inline]
unsafe extern "C" fn table_CFF_copyReplace(mut dst: *mut table_CFF, src: table_CFF) {
    table_CFF_dispose(dst);
    table_CFF_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_CFF_dispose(mut x: *mut table_CFF) {
    disposeFD(x);
}
#[inline]
unsafe extern "C" fn table_CFF_copy(mut dst: *mut table_CFF, mut src: *const table_CFF) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_CFF>() as size_t,
    );
}
unsafe extern "C" fn callback_extract_private(
    mut op: uint32_t,
    mut top: uint8_t,
    mut stack: *mut cff_Value,
    mut _context: *mut ::core::ffi::c_void,
) {
    let mut context: *mut cff_extract_context = _context as *mut cff_extract_context;
    let mut meta: *mut table_CFF = (*context).meta;
    if (*context).fdArrayIndex >= 0 as int32_t
        && (*context).fdArrayIndex < (*meta).fdArrayCount as int32_t
    {
        meta = *(*meta).fdArray.offset((*context).fdArrayIndex as isize);
    }
    let mut pd: *mut cff_PrivateDict = (*meta).privateDict;
    match op {
        6 => {
            (*pd).blueValuesCount = top as arity_t;
            (*pd).blueValues = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                    .wrapping_mul((*pd).blueValuesCount as size_t),
                86 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j: arity_t = 0 as arity_t;
            while j < (*pd).blueValuesCount {
                *(*pd).blueValues.offset(j as isize) = cffnum(*stack.offset(j as isize));
                j = j.wrapping_add(1);
            }
        }
        7 => {
            (*pd).otherBluesCount = top as arity_t;
            (*pd).otherBlues = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                    .wrapping_mul((*pd).otherBluesCount as size_t),
                94 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j_0: arity_t = 0 as arity_t;
            while j_0 < (*pd).otherBluesCount {
                *(*pd).otherBlues.offset(j_0 as isize) = cffnum(*stack.offset(j_0 as isize));
                j_0 = j_0.wrapping_add(1);
            }
        }
        8 => {
            (*pd).familyBluesCount = top as arity_t;
            (*pd).familyBlues = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                    .wrapping_mul((*pd).familyBluesCount as size_t),
                102 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j_1: arity_t = 0 as arity_t;
            while j_1 < (*pd).familyBluesCount {
                *(*pd).familyBlues.offset(j_1 as isize) = cffnum(*stack.offset(j_1 as isize));
                j_1 = j_1.wrapping_add(1);
            }
        }
        9 => {
            (*pd).familyOtherBluesCount = top as arity_t;
            (*pd).familyOtherBlues = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                    .wrapping_mul((*pd).familyOtherBluesCount as size_t),
                110 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j_2: arity_t = 0 as arity_t;
            while j_2 < (*pd).familyOtherBluesCount {
                *(*pd).familyOtherBlues.offset(j_2 as isize) = cffnum(*stack.offset(j_2 as isize));
                j_2 = j_2.wrapping_add(1);
            }
        }
        3084 => {
            (*pd).stemSnapHCount = top as arity_t;
            (*pd).stemSnapH = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                    .wrapping_mul((*pd).stemSnapHCount as size_t),
                118 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j_3: arity_t = 0 as arity_t;
            while j_3 < (*pd).stemSnapHCount {
                *(*pd).stemSnapH.offset(j_3 as isize) = cffnum(*stack.offset(j_3 as isize));
                j_3 = j_3.wrapping_add(1);
            }
        }
        3085 => {
            (*pd).stemSnapVCount = top as arity_t;
            (*pd).stemSnapV = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                    .wrapping_mul((*pd).stemSnapVCount as size_t),
                126 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j_4: arity_t = 0 as arity_t;
            while j_4 < (*pd).stemSnapVCount {
                *(*pd).stemSnapV.offset(j_4 as isize) = cffnum(*stack.offset(j_4 as isize));
                j_4 = j_4.wrapping_add(1);
            }
        }
        3081 => {
            if top != 0 {
                (*pd).blueScale = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3082 => {
            if top != 0 {
                (*pd).blueShift = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3083 => {
            if top != 0 {
                (*pd).blueFuzz = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        10 => {
            if top != 0 {
                (*pd).stdHW = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        11 => {
            if top != 0 {
                (*pd).stdVW = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3086 => {
            if top != 0 {
                (*pd).forceBold = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) != 0.;
            }
        }
        3089 => {
            if top != 0 {
                (*pd).languageGroup = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) as uint32_t;
            }
        }
        3090 => {
            if top != 0 {
                (*pd).expansionFactor = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3091 => {
            if top != 0 {
                (*pd).initialRandomSeed = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        20 => {
            if top != 0 {
                (*pd).defaultWidthX = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        21 => {
            if top != 0 {
                (*pd).nominalWidthX = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn callback_extract_fd(
    mut op: uint32_t,
    mut top: uint8_t,
    mut stack: *mut cff_Value,
    mut _context: *mut ::core::ffi::c_void,
) {
    let mut context: *mut cff_extract_context = _context as *mut cff_extract_context;
    let mut file: *mut cff_File = (*context).cffFile;
    let mut meta: *mut table_CFF = (*context).meta;
    if (*context).fdArrayIndex >= 0 as int32_t
        && (*context).fdArrayIndex < (*meta).fdArrayCount as int32_t
    {
        meta = *(*meta).fdArray.offset((*context).fdArrayIndex as isize);
    }
    match op {
        0 => {
            if top != 0 {
                (*meta).version = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as uint16_t,
                    (*file).string,
                );
            }
        }
        1 => {
            if top != 0 {
                (*meta).notice = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as uint16_t,
                    (*file).string,
                );
            }
        }
        3072 => {
            if top != 0 {
                (*meta).copyright = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as uint16_t,
                    (*file).string,
                );
            }
        }
        3110 => {
            if top != 0 {
                (*meta).fontName = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as uint16_t,
                    (*file).string,
                );
            }
        }
        2 => {
            if top != 0 {
                (*meta).fullName = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as uint16_t,
                    (*file).string,
                );
            }
        }
        3 => {
            if top != 0 {
                (*meta).familyName = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as uint16_t,
                    (*file).string,
                );
            }
        }
        4 => {
            if top != 0 {
                (*meta).weight = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as uint16_t,
                    (*file).string,
                );
            }
        }
        5 => {
            if top as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                (*meta).fontBBoxLeft = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize),
                );
                (*meta).fontBBoxBottom = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize),
                );
                (*meta).fontBBoxRight = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize),
                );
                (*meta).fontBBoxTop = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3079 => {
            if top as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                (*meta).fontMatrix = __caryll_allocate_clean(
                    ::core::mem::size_of::<cff_FontMatrix>() as size_t,
                    208 as ::core::ffi::c_ulong,
                ) as *mut cff_FontMatrix;
                (*(*meta).fontMatrix).a = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize),
                ) as scale_t;
                (*(*meta).fontMatrix).b = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize),
                ) as scale_t;
                (*(*meta).fontMatrix).c = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize),
                ) as scale_t;
                (*(*meta).fontMatrix).d = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize),
                ) as scale_t;
                (*(*meta).fontMatrix).x = iVQ.createStill.expect("non-null function pointer")(
                    cffnum(
                        *stack
                            .offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize),
                    ) as pos_t,
                );
                (*(*meta).fontMatrix).y = iVQ.createStill.expect("non-null function pointer")(
                    cffnum(
                        *stack
                            .offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                    ) as pos_t,
                );
            }
        }
        3073 => {
            if top != 0 {
                (*meta).isFixedPitch = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) != 0.;
            }
        }
        3074 => {
            if top != 0 {
                (*meta).italicAngle = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3075 => {
            if top != 0 {
                (*meta).underlinePosition = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3076 => {
            if top != 0 {
                (*meta).underlineThickness = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3080 => {
            if top != 0 {
                (*meta).strokeWidth = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        18 => {
            if top as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
                let mut privateLength: uint32_t = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize),
                ) as uint32_t;
                let mut privateOffset: uint32_t = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) as uint32_t;
                (*meta).privateDict = otfcc_newCff_private();
                cff_iDict
                    .parseToCallback
                    .expect("non-null function pointer")(
                    (*file).raw_data.offset(privateOffset as isize),
                    privateLength,
                    context as *mut ::core::ffi::c_void,
                    Some(
                        callback_extract_private
                            as unsafe extern "C" fn(
                                uint32_t,
                                uint8_t,
                                *mut cff_Value,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                );
            }
        }
        3102 => {
            if top as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                (*meta).isCID = true;
                (*meta).cidRegistry = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as uint16_t,
                    (*file).string,
                );
                (*meta).cidOrdering = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as uint16_t,
                    (*file).string,
                );
                (*meta).cidSupplement = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) as uint32_t;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn callback_draw_setwidth(
    mut _context: *mut ::core::ffi::c_void,
    mut width: ::core::ffi::c_double,
) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    iVQ.replace.expect("non-null function pointer")(
        &raw mut (*(*context).g).advanceWidth,
        iVQ.createStill.expect("non-null function pointer")(
            width as pos_t + (*context).nominalWidthX as pos_t,
        ) as VQ,
    );
}
unsafe extern "C" fn callback_draw_next_contour(mut _context: *mut ::core::ffi::c_void) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    let mut c: glyf_Contour = glyf_Contour {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<glyf_Point>(),
    };
    glyf_iContour.init.expect("non-null function pointer")(&raw mut c);
    glyf_iContourList.push.expect("non-null function pointer")(
        &raw mut (*(*context).g).contours,
        c,
    );
    (*context).jContour = (*(*context).g).contours.length as shapeid_t;
    (*context).jPoint = 0 as shapeid_t;
}
unsafe extern "C" fn callback_draw_lineto(
    mut _context: *mut ::core::ffi::c_void,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    if (*context).jContour != 0 {
        let mut contour: *mut glyf_Contour =
            (*(*context).g).contours.items.offset(
                ((*context).jContour as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
            ) as *mut glyf_Contour;
        let mut z: glyf_Point = glyf_Point {
            x: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            onCurve: 0,
        };
        glyf_iPoint.init.expect("non-null function pointer")(&raw mut z);
        z.onCurve = true_0 as int8_t;
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z.x,
            iVQ.createStill.expect("non-null function pointer")(x1 as pos_t) as VQ,
        );
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z.y,
            iVQ.createStill.expect("non-null function pointer")(y1 as pos_t) as VQ,
        );
        glyf_iContour.push.expect("non-null function pointer")(contour, z);
        (*context).jPoint =
            ((*context).jPoint as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
    }
}
unsafe extern "C" fn callback_draw_curveto(
    mut _context: *mut ::core::ffi::c_void,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
    mut x3: ::core::ffi::c_double,
    mut y3: ::core::ffi::c_double,
) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    if (*context).jContour != 0 {
        let mut contour: *mut glyf_Contour =
            (*(*context).g).contours.items.offset(
                ((*context).jContour as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
            ) as *mut glyf_Contour;
        let mut z: glyf_Point = glyf_Point {
            x: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            onCurve: 0,
        };
        glyf_iPoint.init.expect("non-null function pointer")(&raw mut z);
        z.onCurve = false_0 as int8_t;
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z.x,
            iVQ.createStill.expect("non-null function pointer")(x1 as pos_t) as VQ,
        );
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z.y,
            iVQ.createStill.expect("non-null function pointer")(y1 as pos_t) as VQ,
        );
        glyf_iContour.push.expect("non-null function pointer")(contour, z);
        let mut z_0: glyf_Point = glyf_Point {
            x: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            onCurve: 0,
        };
        glyf_iPoint.init.expect("non-null function pointer")(&raw mut z_0);
        z_0.onCurve = false_0 as int8_t;
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z_0.x,
            iVQ.createStill.expect("non-null function pointer")(x2 as pos_t) as VQ,
        );
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z_0.y,
            iVQ.createStill.expect("non-null function pointer")(y2 as pos_t) as VQ,
        );
        glyf_iContour.push.expect("non-null function pointer")(contour, z_0);
        let mut z_1: glyf_Point = glyf_Point {
            x: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            onCurve: 0,
        };
        glyf_iPoint.init.expect("non-null function pointer")(&raw mut z_1);
        z_1.onCurve = true_0 as int8_t;
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z_1.x,
            iVQ.createStill.expect("non-null function pointer")(x3 as pos_t) as VQ,
        );
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z_1.y,
            iVQ.createStill.expect("non-null function pointer")(y3 as pos_t) as VQ,
        );
        glyf_iContour.push.expect("non-null function pointer")(contour, z_1);
        (*context).jPoint =
            ((*context).jPoint as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as shapeid_t;
    }
}
unsafe extern "C" fn callback_draw_sethint(
    mut _context: *mut ::core::ffi::c_void,
    mut isVertical: bool,
    mut position: ::core::ffi::c_double,
    mut width: ::core::ffi::c_double,
) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    glyf_iStemDefList.push.expect("non-null function pointer")(
        if isVertical as ::core::ffi::c_int != 0 {
            &raw mut (*(*context).g).stemV
        } else {
            &raw mut (*(*context).g).stemH
        },
        glyf_PostscriptStemDef {
            position: position as pos_t,
            width: width as pos_t,
            map: 0,
        },
    );
}
unsafe extern "C" fn callback_draw_setmask(
    mut _context: *mut ::core::ffi::c_void,
    mut isContourMask: bool,
    mut maskArray: *mut bool,
) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    let mut maskList: *mut glyf_MaskList = if isContourMask as ::core::ffi::c_int != 0 {
        &raw mut (*(*context).g).contourMasks
    } else {
        &raw mut (*(*context).g).hintMasks
    };
    let mut mask: glyf_PostscriptHintMask = glyf_PostscriptHintMask {
        pointsBefore: 0,
        contoursBefore: 0,
        maskH: [false; 256],
        maskV: [false; 256],
    };
    if (*context).jContour != 0 {
        mask.contoursBefore =
            ((*context).jContour as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint16_t;
    } else {
        mask.contoursBefore = 0 as uint16_t;
    }
    mask.pointsBefore = (*context).jPoint as uint16_t;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int {
        mask.maskH[j as usize] = if (j as size_t) < (*(*context).g).stemH.length {
            *maskArray.offset(j as isize) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != 0;
        mask.maskV[j as usize] = if (j as size_t) < (*(*context).g).stemV.length {
            *maskArray.offset((j as size_t).wrapping_add((*(*context).g).stemH.length) as isize)
                as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != 0;
        j = j.wrapping_add(1);
    }
    free(maskArray as *mut ::core::ffi::c_void);
    maskArray = ::core::ptr::null_mut::<bool>();
    if (*maskList).length > 0 as size_t
        && (*(*maskList)
            .items
            .offset((*maskList).length.wrapping_sub(1 as size_t) as isize))
        .contoursBefore as ::core::ffi::c_int
            == mask.contoursBefore as ::core::ffi::c_int
        && (*(*maskList)
            .items
            .offset((*maskList).length.wrapping_sub(1 as size_t) as isize))
        .pointsBefore as ::core::ffi::c_int
            == mask.pointsBefore as ::core::ffi::c_int
    {
        let mut j_0: shapeid_t = 0 as shapeid_t;
        while (j_0 as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int {
            (*(*maskList)
                .items
                .offset((*maskList).length.wrapping_sub(1 as size_t) as isize))
            .maskH[j_0 as usize] = mask.maskH[j_0 as usize];
            (*(*maskList)
                .items
                .offset((*maskList).length.wrapping_sub(1 as size_t) as isize))
            .maskV[j_0 as usize] = mask.maskV[j_0 as usize];
            j_0 = j_0.wrapping_add(1);
        }
    } else {
        glyf_iMaskList.push.expect("non-null function pointer")(maskList, mask);
        if isContourMask {
            (*context).definedContourMasks = ((*context).definedContourMasks as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint8_t;
        } else {
            (*context).definedHintMasks = ((*context).definedHintMasks as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint8_t;
        }
    };
}
unsafe extern "C" fn callback_draw_getrand(
    mut _context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_double {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    let mut x: uint64_t = (*context).randx;
    x ^= x >> 12 as ::core::ffi::c_int;
    x ^= x << 25 as ::core::ffi::c_int;
    x ^= x >> 27 as ::core::ffi::c_int;
    (*context).randx = x;
    let mut a: C2RustUnnamed_10 = C2RustUnnamed_10 { u: 0 };
    a.u = x.wrapping_mul(2685821657736338717 as uint64_t);
    a.u = a.u >> 12 as ::core::ffi::c_int | 0x3ff0000000000000 as uint64_t;
    let mut q: ::core::ffi::c_double = if a.u & 2048 as uint64_t != 0 {
        1.0f64 - 2.2204460492503131E-16f64 / 2.0f64
    } else {
        1.0f64
    };
    return a.d - q;
}
static mut drawPass: cff_IOutlineBuilder = {
    cff_IOutlineBuilder {
        setWidth: Some(
            callback_draw_setwidth
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> (),
        ),
        newContour: Some(
            callback_draw_next_contour as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        lineTo: Some(
            callback_draw_lineto
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        ),
        curveTo: Some(
            callback_draw_curveto
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        ),
        setHint: Some(
            callback_draw_sethint
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    bool,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        ),
        setMask: Some(
            callback_draw_setmask
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> (),
        ),
        getrand: Some(
            callback_draw_getrand
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double,
        ),
    }
};
unsafe extern "C" fn buildOutline(
    mut i: glyphid_t,
    mut context: *mut cff_extract_context,
    mut options: *const otfcc_Options,
) {
    let mut f: *mut cff_File = (*context).cffFile;
    let mut g: *mut glyf_Glyph = otfcc_newGlyf_glyph();
    let ref mut fresh8 = *(*(*context).glyphs).items.offset(i as isize);
    *fresh8 = g as glyf_GlyphPtr;
    let mut seed: uint64_t = (*context).seed;
    let mut localSubrs: cff_Index = cff_Index {
        countType: CFF_INDEX_16,
        count: 0,
        offSize: 0,
        offset: ::core::ptr::null_mut::<uint32_t>(),
        data: ::core::ptr::null_mut::<uint8_t>(),
    };
    cff_iIndex.init.expect("non-null function pointer")(&raw mut localSubrs);
    let mut stack: cff_Stack = cff_Stack {
        stack: ::core::ptr::null_mut::<cff_Value>(),
        transient: [cff_Value {
            t: 0 as cff_Value_Type,
            c2rust_unnamed: C2RustUnnamed_9 { i: 0 },
        }; 32],
        index: 0,
        max: 0,
        stem: 0,
    };
    stack.max = 0x10000 as arity_t;
    stack.stack = __caryll_allocate_clean(
        (::core::mem::size_of::<cff_Value>() as size_t).wrapping_mul(stack.max as size_t),
        407 as ::core::ffi::c_ulong,
    ) as *mut cff_Value;
    stack.index = 0 as arity_t;
    stack.stem = 0 as uint8_t;
    let mut bc: outline_builder_context = outline_builder_context {
        g: g,
        jContour: 0 as shapeid_t,
        jPoint: 0 as shapeid_t,
        defaultWidthX: 0.0f64,
        nominalWidthX: 0.0f64,
        definedHStems: 0 as uint8_t,
        definedVStems: 0 as uint8_t,
        definedHintMasks: 0 as uint8_t,
        definedContourMasks: 0 as uint8_t,
        randx: 0 as uint64_t,
    };
    let mut fd: uint8_t = 0 as uint8_t;
    if (*f).fdselect.t != cff_FDSELECT_UNSPECED as ::core::ffi::c_int as uint32_t {
        fd = cff_parseSubr(
            i as uint16_t,
            (*f).raw_data,
            (*f).font_dict,
            (*f).fdselect,
            &raw mut localSubrs,
        );
    } else {
        fd = cff_parseSubr(
            i as uint16_t,
            (*f).raw_data,
            (*f).top_dict,
            (*f).fdselect,
            &raw mut localSubrs,
        );
    }
    (*g).fdSelect = handle_fromIndex(fd as glyphid_t)
        as otfcc_FDHandle;
    if !(*(*context).meta).fdArray.is_null()
        && (fd as ::core::ffi::c_int) < (*(*context).meta).fdArrayCount as ::core::ffi::c_int
        && !(**(*(*context).meta).fdArray.offset(fd as isize))
            .privateDict
            .is_null()
    {
        bc.defaultWidthX =
            (*(**(*(*context).meta).fdArray.offset(fd as isize)).privateDict).defaultWidthX;
        bc.nominalWidthX =
            (*(**(*(*context).meta).fdArray.offset(fd as isize)).privateDict).nominalWidthX;
    } else if !(*(*context).meta).privateDict.is_null() {
        bc.defaultWidthX = (*(*(*context).meta).privateDict).defaultWidthX;
        bc.nominalWidthX = (*(*(*context).meta).privateDict).nominalWidthX;
    }
    iVQ.replace.expect("non-null function pointer")(
        &raw mut (*g).advanceWidth,
        iVQ.createStill.expect("non-null function pointer")(bc.defaultWidthX as pos_t) as VQ,
    );
    let mut charStringPtr: *mut uint8_t = (*f)
        .char_strings
        .data
        .offset(*(*f).char_strings.offset.offset(i as isize) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut charStringLength: uint32_t = (*(*f)
        .char_strings
        .offset
        .offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
    .wrapping_sub(*(*f).char_strings.offset.offset(i as isize));
    stack.index = 0 as arity_t;
    stack.stem = 0 as uint8_t;
    bc.jContour = 0 as shapeid_t;
    bc.jPoint = 0 as shapeid_t;
    bc.randx = seed;
    cff_parseOutline(
        charStringPtr,
        charStringLength,
        (*f).global_subr,
        localSubrs,
        &raw mut stack,
        &raw mut bc as *mut ::core::ffi::c_void,
        drawPass,
        options,
    );
    let mut cx: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut cy: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as size_t) < (*g).contours.length {
        let mut contour: *mut glyf_Contour =
            (*g).contours.items.offset(j as isize) as *mut glyf_Contour;
        let mut k: shapeid_t = 0 as shapeid_t;
        while (k as size_t) < (*contour).length {
            let mut z: *mut glyf_Point = (*contour).items.offset(k as isize) as *mut glyf_Point;
            iVQ.inplacePlus.expect("non-null function pointer")(&raw mut cx, (*z).x);
            iVQ.inplacePlus.expect("non-null function pointer")(&raw mut cy, (*z).y);
            iVQ.copyReplace.expect("non-null function pointer")(&raw mut (*z).x, cx);
            iVQ.copyReplace.expect("non-null function pointer")(&raw mut (*z).y, cy);
            k = k.wrapping_add(1);
        }
        if iVQ.compare.expect("non-null function pointer")(
            (*(*contour).items.offset(0 as ::core::ffi::c_int as isize)).x,
            (*(*contour)
                .items
                .offset((*contour).length.wrapping_sub(1 as size_t) as isize))
            .x,
        ) == 0
            && iVQ.compare.expect("non-null function pointer")(
                (*(*contour).items.offset(0 as ::core::ffi::c_int as isize)).y,
                (*(*contour)
                    .items
                    .offset((*contour).length.wrapping_sub(1 as size_t) as isize))
                .y,
            ) == 0
            && ((*(*contour).items.offset(0 as ::core::ffi::c_int as isize)).onCurve
                as ::core::ffi::c_int
                != 0
                && (*(*contour)
                    .items
                    .offset((*contour).length.wrapping_sub(1 as size_t) as isize))
                .onCurve as ::core::ffi::c_int
                    != 0)
        {
            glyf_iContour.pop.expect("non-null function pointer")(contour);
        }
        glyf_iContour
            .shrinkToFit
            .expect("non-null function pointer")(contour);
        j = j.wrapping_add(1);
    }
    glyf_iContourList
        .shrinkToFit
        .expect("non-null function pointer")(&raw mut (*g).contours);
    iVQ.dispose.expect("non-null function pointer")(&raw mut cx);
    iVQ.dispose.expect("non-null function pointer")(&raw mut cy);
    cff_iIndex.dispose.expect("non-null function pointer")(&raw mut localSubrs);
    free(stack.stack as *mut ::core::ffi::c_void);
    stack.stack = ::core::ptr::null_mut::<cff_Value>();
    (*context).seed = bc.randx;
}
unsafe extern "C" fn formCIDString(mut cid: cffsid_t) -> sds {
    return sdscatprintf(
        sdsnew(b"CID\0" as *const u8 as *const ::core::ffi::c_char),
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        cid as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn nameGlyphsAccordingToCFF(mut context: *mut cff_extract_context) {
    let mut cffFile: *mut cff_File = (*context).cffFile;
    let mut glyphs: *mut table_glyf = (*context).glyphs;
    let mut charset: *mut cff_Charset = &raw mut (*cffFile).charsets;
    if (*(*context).meta).isCID {
        match (*charset).t {
            3 => {
                let mut j: glyphid_t = 0 as glyphid_t;
                while (j as uint32_t) < (*charset).s {
                    let mut sid: cffsid_t =
                        *(*charset).c2rust_unnamed.f0.glyph.offset(j as isize) as cffsid_t;
                    let mut glyphname: sds = sdsget_cff_sid(sid as uint16_t, (*cffFile).string);
                    if !glyphname.is_null() {
                        let ref mut fresh2 = (**(*glyphs)
                            .items
                            .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .name;
                        *fresh2 = glyphname;
                        (**(*glyphs).items.offset(
                            (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .cid = sid as glyphid_t;
                    }
                    j = j.wrapping_add(1);
                }
            }
            4 => {
                let mut glyphsNamedSofar: uint32_t = 1 as uint32_t;
                let mut j_0: glyphid_t = 0 as glyphid_t;
                while (j_0 as uint32_t) < (*charset).s {
                    let mut first: cffsid_t =
                        (*(*charset).c2rust_unnamed.f1.range1.offset(j_0 as isize)).first
                            as cffsid_t;
                    let mut k: glyphid_t = 0 as glyphid_t;
                    while k as ::core::ffi::c_int
                        <= (*(*charset).c2rust_unnamed.f1.range1.offset(j_0 as isize)).nleft
                            as ::core::ffi::c_int
                    {
                        let mut sid_0: cffsid_t =
                            (first as ::core::ffi::c_int + k as ::core::ffi::c_int) as cffsid_t;
                        let mut glyphname_0: sds = formCIDString(sid_0);
                        if (glyphsNamedSofar as size_t) < (*glyphs).length && !glyphname_0.is_null()
                        {
                            let ref mut fresh3 =
                                (**(*glyphs).items.offset(glyphsNamedSofar as isize)).name;
                            *fresh3 = glyphname_0;
                            (**(*glyphs).items.offset(glyphsNamedSofar as isize)).cid =
                                sid_0 as glyphid_t;
                        }
                        glyphsNamedSofar = glyphsNamedSofar.wrapping_add(1);
                        k = k.wrapping_add(1);
                    }
                    j_0 = j_0.wrapping_add(1);
                }
            }
            5 => {
                let mut glyphsNamedSofar_0: uint32_t = 1 as uint32_t;
                let mut j_1: glyphid_t = 0 as glyphid_t;
                while (j_1 as uint32_t) < (*charset).s {
                    let mut first_0: cffsid_t =
                        (*(*charset).c2rust_unnamed.f2.range2.offset(j_1 as isize)).first
                            as cffsid_t;
                    let mut k_0: glyphid_t = 0 as glyphid_t;
                    while k_0 as ::core::ffi::c_int
                        <= (*(*charset).c2rust_unnamed.f2.range2.offset(j_1 as isize)).nleft
                            as ::core::ffi::c_int
                    {
                        let mut sid_1: cffsid_t =
                            (first_0 as ::core::ffi::c_int + k_0 as ::core::ffi::c_int) as cffsid_t;
                        let mut glyphname_1: sds = formCIDString(sid_1);
                        if (glyphsNamedSofar_0 as size_t) < (*glyphs).length
                            && !glyphname_1.is_null()
                        {
                            let ref mut fresh4 =
                                (**(*glyphs).items.offset(glyphsNamedSofar_0 as isize)).name;
                            *fresh4 = glyphname_1;
                            (**(*glyphs).items.offset(glyphsNamedSofar_0 as isize)).cid =
                                sid_1 as glyphid_t;
                        }
                        glyphsNamedSofar_0 = glyphsNamedSofar_0.wrapping_add(1);
                        k_0 = k_0.wrapping_add(1);
                    }
                    j_1 = j_1.wrapping_add(1);
                }
            }
            _ => {}
        }
    } else {
        match (*charset).t {
            3 => {
                let mut j_2: glyphid_t = 0 as glyphid_t;
                while (j_2 as uint32_t) < (*charset).s {
                    let mut sid_2: cffsid_t =
                        *(*charset).c2rust_unnamed.f0.glyph.offset(j_2 as isize) as cffsid_t;
                    let mut glyphname_2: sds = sdsget_cff_sid(sid_2 as uint16_t, (*cffFile).string);
                    if !glyphname_2.is_null() {
                        let ref mut fresh5 = (**(*glyphs).items.offset(
                            (j_2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .name;
                        *fresh5 = glyphname_2;
                    }
                    j_2 = j_2.wrapping_add(1);
                }
            }
            4 => {
                let mut glyphsNamedSofar_1: uint32_t = 1 as uint32_t;
                let mut j_3: glyphid_t = 0 as glyphid_t;
                while (j_3 as uint32_t) < (*charset).s {
                    let mut first_1: glyphid_t =
                        (*(*charset).c2rust_unnamed.f1.range1.offset(j_3 as isize)).first
                            as glyphid_t;
                    let mut k_1: glyphid_t = 0 as glyphid_t;
                    while k_1 as ::core::ffi::c_int
                        <= (*(*charset).c2rust_unnamed.f1.range1.offset(j_3 as isize)).nleft
                            as ::core::ffi::c_int
                    {
                        let mut sid_3: cffsid_t =
                            (first_1 as ::core::ffi::c_int + k_1 as ::core::ffi::c_int) as cffsid_t;
                        let mut glyphname_3: sds =
                            sdsget_cff_sid(sid_3 as uint16_t, (*cffFile).string);
                        if (glyphsNamedSofar_1 as size_t) < (*glyphs).length
                            && !glyphname_3.is_null()
                        {
                            let ref mut fresh6 =
                                (**(*glyphs).items.offset(glyphsNamedSofar_1 as isize)).name;
                            *fresh6 = glyphname_3;
                        }
                        glyphsNamedSofar_1 = glyphsNamedSofar_1.wrapping_add(1);
                        k_1 = k_1.wrapping_add(1);
                    }
                    j_3 = j_3.wrapping_add(1);
                }
            }
            5 => {
                let mut glyphsNamedSofar_2: uint32_t = 1 as uint32_t;
                let mut j_4: glyphid_t = 0 as glyphid_t;
                while (j_4 as uint32_t) < (*charset).s {
                    let mut first_2: glyphid_t =
                        (*(*charset).c2rust_unnamed.f2.range2.offset(j_4 as isize)).first
                            as glyphid_t;
                    let mut k_2: glyphid_t = 0 as glyphid_t;
                    while k_2 as ::core::ffi::c_int
                        <= (*(*charset).c2rust_unnamed.f2.range2.offset(j_4 as isize)).nleft
                            as ::core::ffi::c_int
                    {
                        let mut sid_4: cffsid_t =
                            (first_2 as ::core::ffi::c_int + k_2 as ::core::ffi::c_int) as cffsid_t;
                        let mut glyphname_4: sds =
                            sdsget_cff_sid(sid_4 as uint16_t, (*cffFile).string);
                        if (glyphsNamedSofar_2 as size_t) < (*glyphs).length
                            && !glyphname_4.is_null()
                        {
                            let ref mut fresh7 =
                                (**(*glyphs).items.offset(glyphsNamedSofar_2 as isize)).name;
                            *fresh7 = glyphname_4;
                        }
                        glyphsNamedSofar_2 = glyphsNamedSofar_2.wrapping_add(1);
                        k_2 = k_2.wrapping_add(1);
                    }
                    j_4 = j_4.wrapping_add(1);
                }
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn qround(x: ::core::ffi::c_double) -> ::core::ffi::c_double {
    return otfcc_from_fixed(otfcc_to_fixed(x));
}
unsafe extern "C" fn applyCffMatrix(
    mut CFF_: *mut table_CFF,
    mut glyf: *mut table_glyf,
    mut head: *const table_head,
) {
    let mut jj: glyphid_t = 0 as glyphid_t;
    while (jj as size_t) < (*glyf).length {
        let mut g: *mut glyf_Glyph = *(*glyf).items.offset(jj as isize) as *mut glyf_Glyph;
        let mut fd: *mut table_CFF = CFF_;
        if !(*fd).fdArray.is_null()
            && ((*g).fdSelect.index as ::core::ffi::c_int)
                < (*fd).fdArrayCount as ::core::ffi::c_int
        {
            fd = *(*fd).fdArray.offset((*g).fdSelect.index as isize);
        }
        if !(*fd).fontMatrix.is_null() {
            let mut a: scale_t = qround(
                (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    * (*(*fd).fontMatrix).a as ::core::ffi::c_double,
            ) as scale_t;
            let mut b: scale_t = qround(
                (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    * (*(*fd).fontMatrix).b as ::core::ffi::c_double,
            ) as scale_t;
            let mut c: scale_t = qround(
                (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    * (*(*fd).fontMatrix).c as ::core::ffi::c_double,
            ) as scale_t;
            let mut d: scale_t = qround(
                (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    * (*(*fd).fontMatrix).d as ::core::ffi::c_double,
            ) as scale_t;
            let mut x: VQ = iVQ.scale.expect("non-null function pointer")(
                (*(*fd).fontMatrix).x,
                (*head).unitsPerEm as scale_t,
            );
            x.kernel = qround(x.kernel as ::core::ffi::c_double) as pos_t;
            let mut y: VQ = iVQ.scale.expect("non-null function pointer")(
                (*(*fd).fontMatrix).y,
                (*head).unitsPerEm as scale_t,
            );
            y.kernel = qround(y.kernel as ::core::ffi::c_double) as pos_t;
            let mut j: shapeid_t = 0 as shapeid_t;
            while (j as size_t) < (*g).contours.length {
                let mut contour: *mut glyf_Contour =
                    (*g).contours.items.offset(j as isize) as *mut glyf_Contour;
                let mut k: shapeid_t = 0 as shapeid_t;
                while (k as size_t) < (*contour).length {
                    let mut zx: VQ = iVQ.dup.expect("non-null function pointer")(
                        (*(*contour).items.offset(k as isize)).x,
                    );
                    let mut zy: VQ = iVQ.dup.expect("non-null function pointer")(
                        (*(*contour).items.offset(k as isize)).y,
                    );
                    iVQ.replace.expect("non-null function pointer")(
                        &raw mut (*(*contour).items.offset(k as isize)).x,
                        iVQ.pointLinearTfm.expect("non-null function pointer")(
                            x, a as pos_t, zx, b as pos_t, zy,
                        ) as VQ,
                    );
                    iVQ.replace.expect("non-null function pointer")(
                        &raw mut (*(*contour).items.offset(k as isize)).y,
                        iVQ.pointLinearTfm.expect("non-null function pointer")(
                            y, c as pos_t, zx, d as pos_t, zy,
                        ) as VQ,
                    );
                    iVQ.dispose.expect("non-null function pointer")(&raw mut zx);
                    iVQ.dispose.expect("non-null function pointer")(&raw mut zy);
                    k = k.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
            iVQ.dispose.expect("non-null function pointer")(&raw mut x);
            iVQ.dispose.expect("non-null function pointer")(&raw mut y);
        }
        jj = jj.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readCFFAndGlyfTables(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
    mut head: *const table_head,
) -> table_CFFAndGlyf {
    let mut ret: table_CFFAndGlyf = table_CFFAndGlyf {
        meta: ::core::ptr::null_mut::<table_CFF>(),
        glyphs: ::core::ptr::null_mut::<table_glyf>(),
    };
    ret.meta = ::core::ptr::null_mut::<table_CFF>();
    ret.glyphs = ::core::ptr::null_mut::<table_glyf>();
    let mut context: cff_extract_context = cff_extract_context {
        fdArrayIndex: 0,
        meta: ::core::ptr::null_mut::<table_CFF>(),
        glyphs: ::core::ptr::null_mut::<table_glyf>(),
        cffFile: ::core::ptr::null_mut::<cff_File>(),
        seed: 0,
    };
    context.fdArrayIndex = -(1 as ::core::ffi::c_int) as int32_t;
    context.meta = ::core::ptr::null_mut::<table_CFF>();
    context.glyphs = ::core::ptr::null_mut::<table_glyf>();
    context.cffFile = ::core::ptr::null_mut::<cff_File>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1128678944i32 as uint32_t {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: uint32_t = table.length;
                    let mut cffFile: *mut cff_File =
                        cff_openStream(data as *mut uint8_t, length, options);
                    context.cffFile = cffFile;
                    context.meta = (
                        table_iCFF.create.expect("non-null function pointer"))();
                    cff_iDict
                        .parseToCallback
                        .expect("non-null function pointer")(
                        (*cffFile).top_dict.data,
                        (*(*cffFile)
                            .top_dict
                            .offset
                            .offset(1 as ::core::ffi::c_int as isize))
                        .wrapping_sub(
                            *(*cffFile)
                                .top_dict
                                .offset
                                .offset(0 as ::core::ffi::c_int as isize),
                        ),
                        &raw mut context as *mut ::core::ffi::c_void,
                        Some(
                            callback_extract_fd
                                as unsafe extern "C" fn(
                                    uint32_t,
                                    uint8_t,
                                    *mut cff_Value,
                                    *mut ::core::ffi::c_void,
                                ) -> (),
                        ),
                    );
                    if (*context.meta).fontName.is_null() {
                        (*context.meta).fontName = sdsget_cff_sid(391 as uint16_t, (*cffFile).name);
                    }
                    if (*cffFile).font_dict.count != 0 {
                        (*context.meta).fdArrayCount = (*cffFile).font_dict.count as tableid_t;
                        (*context.meta).fdArray = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut table_CFF>() as size_t)
                                .wrapping_mul((*context.meta).fdArrayCount as size_t),
                            637 as ::core::ffi::c_ulong,
                        ) as *mut *mut table_CFF;
                        let mut j: tableid_t = 0 as tableid_t;
                        while (j as ::core::ffi::c_int)
                            < (*context.meta).fdArrayCount as ::core::ffi::c_int
                        {
                            let ref mut fresh0 = *(*context.meta).fdArray.offset(j as isize);
                            *fresh0 = (
                                table_iCFF.create.expect("non-null function pointer"))();
                            context.fdArrayIndex = j as int32_t;
                            cff_iDict
                                .parseToCallback
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*cffFile)
                                    .font_dict
                                    .data
                                    .offset(
                                        *(*cffFile).font_dict.offset.offset(j as isize) as isize,
                                    )
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                (*(*cffFile)
                                    .font_dict
                                    .offset
                                    .offset(
                                        (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                    ))
                                    .wrapping_sub(
                                        *(*cffFile).font_dict.offset.offset(j as isize),
                                    ),
                                &raw mut context as *mut ::core::ffi::c_void,
                                Some(
                                    callback_extract_fd
                                        as unsafe extern "C" fn(
                                            uint32_t,
                                            uint8_t,
                                            *mut cff_Value,
                                            *mut ::core::ffi::c_void,
                                        ) -> (),
                                ),
                            );
                            if (**(*context.meta).fdArray.offset(j as isize))
                                .fontName
                                .is_null()
                            {
                                let ref mut fresh1 =
                                    (**(*context.meta).fdArray.offset(j as isize)).fontName;
                                *fresh1 = sdscatprintf(
                                    sdsempty(),
                                    b"_Subfont%d\0" as *const u8 as *const ::core::ffi::c_char,
                                    j as ::core::ffi::c_int,
                                );
                            }
                            j = j.wrapping_add(1);
                        }
                    }
                    ret.meta = context.meta;
                    context.seed = 0x1234567887654321 as uint64_t;
                    if !(*context.meta).privateDict.is_null() {
                        context.seed = (*(*context.meta).privateDict).initialRandomSeed as uint64_t
                            ^ 0x1234567887654321 as uint64_t;
                    }
                    let mut glyphs: *mut table_glyf =
                        table_iGlyf.createN.expect("non-null function pointer")(
                            (*cffFile).char_strings.count as size_t,
                        );
                    context.glyphs = glyphs;
                    let mut j_0: glyphid_t = 0 as glyphid_t;
                    while (j_0 as size_t) < (*glyphs).length {
                        buildOutline(j_0, &raw mut context, options);
                        j_0 = j_0.wrapping_add(1);
                    }
                    applyCffMatrix(context.meta, context.glyphs, head);
                    nameGlyphsAccordingToCFF(&raw mut context);
                    ret.glyphs = context.glyphs;
                    cff_close(cffFile);
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ret;
}
unsafe extern "C" fn pdDeltaToJson(
    mut target: *mut json_value,
    mut field: *const ::core::ffi::c_char,
    mut count: arity_t,
    mut values: *mut ::core::ffi::c_double,
) {
    if count == 0 || values.is_null() {
        return;
    }
    let mut a: *mut json_value = json_array_new(count as size_t);
    let mut j: arity_t = 0 as arity_t;
    while j < count {
        json_array_push(a, json_double_new(*values.offset(j as isize)));
        j = j.wrapping_add(1);
    }
    json_object_push(target, field, a);
}
unsafe extern "C" fn pdToJson(mut pd: *const cff_PrivateDict) -> *mut json_value {
    let mut _pd: *mut json_value = json_object_new(24 as size_t);
    pdDeltaToJson(
        _pd,
        b"blueValues\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).blueValuesCount,
        (*pd).blueValues,
    );
    pdDeltaToJson(
        _pd,
        b"otherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).otherBluesCount,
        (*pd).otherBlues,
    );
    pdDeltaToJson(
        _pd,
        b"familyBlues\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).familyBluesCount,
        (*pd).familyBlues,
    );
    pdDeltaToJson(
        _pd,
        b"familyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).familyOtherBluesCount,
        (*pd).familyOtherBlues,
    );
    pdDeltaToJson(
        _pd,
        b"stemSnapH\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).stemSnapHCount,
        (*pd).stemSnapH,
    );
    pdDeltaToJson(
        _pd,
        b"stemSnapV\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).stemSnapVCount,
        (*pd).stemSnapV,
    );
    if (*pd).blueScale != DEFAULT_BLUE_SCALE {
        json_object_push(
            _pd,
            b"blueScale\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).blueScale),
        );
    }
    if (*pd).blueShift != DEFAULT_BLUE_SHIFT {
        json_object_push(
            _pd,
            b"blueShift\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).blueShift),
        );
    }
    if (*pd).blueFuzz != DEFAULT_BLUE_FUZZ {
        json_object_push(
            _pd,
            b"blueFuzz\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).blueFuzz),
        );
    }
    if (*pd).stdHW != 0. {
        json_object_push(
            _pd,
            b"stdHW\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).stdHW),
        );
    }
    if (*pd).stdVW != 0. {
        json_object_push(
            _pd,
            b"stdVW\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).stdVW),
        );
    }
    if (*pd).forceBold {
        json_object_push(
            _pd,
            b"forceBold\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*pd).forceBold as ::core::ffi::c_int),
        );
    }
    if (*pd).languageGroup != 0 {
        json_object_push(
            _pd,
            b"languageGroup\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).languageGroup as ::core::ffi::c_double),
        );
    }
    if (*pd).expansionFactor != DEFAULT_EXPANSION_FACTOR {
        json_object_push(
            _pd,
            b"expansionFactor\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).expansionFactor),
        );
    }
    if (*pd).initialRandomSeed != 0. {
        json_object_push(
            _pd,
            b"initialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).initialRandomSeed),
        );
    }
    if (*pd).defaultWidthX != 0. {
        json_object_push(
            _pd,
            b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).defaultWidthX),
        );
    }
    if (*pd).nominalWidthX != 0. {
        json_object_push(
            _pd,
            b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).nominalWidthX),
        );
    }
    return _pd;
}
unsafe extern "C" fn fdToJson(mut table: *const table_CFF) -> *mut json_value {
    let mut _CFF_: *mut json_value = json_object_new(24 as size_t);
    if (*table).isCID {
        json_object_push(
            _CFF_,
            b"isCID\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*table).isCID as ::core::ffi::c_int),
        );
    }
    if !(*table).version.is_null() {
        json_object_push(
            _CFF_,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).version),
        );
    }
    if !(*table).notice.is_null() {
        json_object_push(
            _CFF_,
            b"notice\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).notice),
        );
    }
    if !(*table).copyright.is_null() {
        json_object_push(
            _CFF_,
            b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).copyright),
        );
    }
    if !(*table).fontName.is_null() {
        json_object_push(
            _CFF_,
            b"fontName\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).fontName),
        );
    }
    if !(*table).fullName.is_null() {
        json_object_push(
            _CFF_,
            b"fullName\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).fullName),
        );
    }
    if !(*table).familyName.is_null() {
        json_object_push(
            _CFF_,
            b"familyName\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).familyName),
        );
    }
    if !(*table).weight.is_null() {
        json_object_push(
            _CFF_,
            b"weight\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).weight),
        );
    }
    if (*table).isFixedPitch {
        json_object_push(
            _CFF_,
            b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*table).isFixedPitch as ::core::ffi::c_int),
        );
    }
    if (*table).italicAngle != 0. {
        json_object_push(
            _CFF_,
            b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).italicAngle),
        );
    }
    if (*table).underlinePosition != -(100 as ::core::ffi::c_int) as ::core::ffi::c_double {
        json_object_push(
            _CFF_,
            b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).underlinePosition),
        );
    }
    if (*table).underlineThickness != 50 as ::core::ffi::c_int as ::core::ffi::c_double {
        json_object_push(
            _CFF_,
            b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).underlineThickness),
        );
    }
    if (*table).strokeWidth != 0. {
        json_object_push(
            _CFF_,
            b"strokeWidth\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).strokeWidth),
        );
    }
    if (*table).fontBBoxLeft != 0. {
        json_object_push(
            _CFF_,
            b"fontBBoxLeft\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).fontBBoxLeft),
        );
    }
    if (*table).fontBBoxBottom != 0. {
        json_object_push(
            _CFF_,
            b"fontBBoxBottom\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).fontBBoxBottom),
        );
    }
    if (*table).fontBBoxRight != 0. {
        json_object_push(
            _CFF_,
            b"fontBBoxRight\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).fontBBoxRight),
        );
    }
    if (*table).fontBBoxTop != 0. {
        json_object_push(
            _CFF_,
            b"fontBBoxTop\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).fontBBoxTop),
        );
    }
    if !(*table).fontMatrix.is_null() {
        let mut _fontMatrix: *mut json_value = json_object_new(6 as size_t);
        json_object_push(
            _fontMatrix,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*(*table).fontMatrix).a as ::core::ffi::c_double),
        );
        json_object_push(
            _fontMatrix,
            b"b\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*(*table).fontMatrix).b as ::core::ffi::c_double),
        );
        json_object_push(
            _fontMatrix,
            b"c\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*(*table).fontMatrix).c as ::core::ffi::c_double),
        );
        json_object_push(
            _fontMatrix,
            b"d\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*(*table).fontMatrix).d as ::core::ffi::c_double),
        );
        json_object_push(
            _fontMatrix,
            b"x\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_VQ((*(*table).fontMatrix).x, ::core::ptr::null::<table_fvar>()),
        );
        json_object_push(
            _fontMatrix,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_VQ((*(*table).fontMatrix).y, ::core::ptr::null::<table_fvar>()),
        );
        json_object_push(
            _CFF_,
            b"fontMatrix\0" as *const u8 as *const ::core::ffi::c_char,
            _fontMatrix,
        );
    }
    if !(*table).privateDict.is_null() {
        json_object_push(
            _CFF_,
            b"privates\0" as *const u8 as *const ::core::ffi::c_char,
            pdToJson((*table).privateDict),
        );
    }
    if !(*table).cidRegistry.is_null() && !(*table).cidOrdering.is_null() {
        json_object_push(
            _CFF_,
            b"cidRegistry\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).cidRegistry),
        );
        json_object_push(
            _CFF_,
            b"cidOrdering\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).cidOrdering),
        );
        json_object_push(
            _CFF_,
            b"cidSupplement\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).cidSupplement as int64_t),
        );
    }
    if !(*table).fdArray.is_null() {
        let mut _fdArray: *mut json_value = json_object_new((*table).fdArrayCount as size_t);
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*table).fdArrayCount as ::core::ffi::c_int {
            let mut name: sds = (**(*table).fdArray.offset(j as isize)).fontName;
            let ref mut fresh9 = (**(*table).fdArray.offset(j as isize)).fontName;
            *fresh9 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            json_object_push(
                _fdArray,
                name as *const ::core::ffi::c_char,
                fdToJson(*(*table).fdArray.offset(j as isize)),
            );
            let ref mut fresh10 = (**(*table).fdArray.offset(j as isize)).fontName;
            *fresh10 = name;
            j = j.wrapping_add(1);
        }
        json_object_push(
            _CFF_,
            b"fdArray\0" as *const u8 as *const ::core::ffi::c_char,
            _fdArray,
        );
    }
    return _CFF_;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpCFF(
    mut table: *const table_CFF,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"CFF\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        json_object_push(
            root,
            b"CFF_\0" as *const u8 as *const ::core::ffi::c_char,
            fdToJson(table),
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
unsafe extern "C" fn pdDeltaFromJson(
    mut dump: *mut json_value,
    mut count: *mut arity_t,
    mut array: *mut *mut ::core::ffi::c_double,
) {
    if dump.is_null()
        || (*dump).type_0 as ::core::ffi::c_uint
            != json_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    *count = (*dump).u.array.length as arity_t;
    *array = __caryll_allocate_clean(
        (::core::mem::size_of::<::core::ffi::c_double>() as size_t).wrapping_mul(*count as size_t),
        785 as ::core::ffi::c_ulong,
    ) as *mut ::core::ffi::c_double;
    let mut j: arity_t = 0 as arity_t;
    while j < *count {
        *(*array).offset(j as isize) = json_numof(*(*dump).u.array.values.offset(j as isize));
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn pdFromJson(mut dump: *mut json_value) -> *mut cff_PrivateDict {
    if dump.is_null()
        || (*dump).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<cff_PrivateDict>();
    }
    let mut pd: *mut cff_PrivateDict = otfcc_newCff_private();
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"blueValues\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).blueValuesCount,
        &raw mut (*pd).blueValues,
    );
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"otherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).otherBluesCount,
        &raw mut (*pd).otherBlues,
    );
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"familyBlues\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).familyBluesCount,
        &raw mut (*pd).familyBlues,
    );
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"familyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).familyOtherBluesCount,
        &raw mut (*pd).familyOtherBlues,
    );
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"stemSnapH\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).stemSnapHCount,
        &raw mut (*pd).stemSnapH,
    );
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"stemSnapV\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).stemSnapVCount,
        &raw mut (*pd).stemSnapV,
    );
    (*pd).blueScale = json_obj_getnum_fallback(
        dump,
        b"blueScale\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_BLUE_SCALE,
    );
    (*pd).blueShift = json_obj_getnum_fallback(
        dump,
        b"blueShift\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_BLUE_SHIFT,
    );
    (*pd).blueFuzz = json_obj_getnum_fallback(
        dump,
        b"blueFuzz\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_BLUE_FUZZ,
    );
    (*pd).stdHW = json_obj_getnum(dump, b"stdHW\0" as *const u8 as *const ::core::ffi::c_char);
    (*pd).stdVW = json_obj_getnum(dump, b"stdVW\0" as *const u8 as *const ::core::ffi::c_char);
    (*pd).forceBold = json_obj_getbool(
        dump,
        b"forceBold\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*pd).languageGroup = json_obj_getnum(
        dump,
        b"languageGroup\0" as *const u8 as *const ::core::ffi::c_char,
    ) as uint32_t;
    (*pd).expansionFactor = json_obj_getnum_fallback(
        dump,
        b"expansionFactor\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_EXPANSION_FACTOR,
    );
    (*pd).initialRandomSeed = json_obj_getnum(
        dump,
        b"initialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return pd;
}
unsafe extern "C" fn fdFromJson(
    mut dump: *const json_value,
    mut options: *const otfcc_Options,
    mut topLevel: bool,
) -> *mut table_CFF {
    let mut table: *mut table_CFF = (
        table_iCFF.create.expect("non-null function pointer"))();
    if dump.is_null()
        || (*dump).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return table;
    }
    (*table).version = json_obj_getsds(
        dump,
        b"version\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).notice = json_obj_getsds(dump, b"notice\0" as *const u8 as *const ::core::ffi::c_char);
    (*table).copyright = json_obj_getsds(
        dump,
        b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fontName = json_obj_getsds(
        dump,
        b"fontName\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fullName = json_obj_getsds(
        dump,
        b"fullName\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).familyName = json_obj_getsds(
        dump,
        b"familyName\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).weight = json_obj_getsds(dump, b"weight\0" as *const u8 as *const ::core::ffi::c_char);
    (*table).isFixedPitch = json_obj_getbool(
        dump,
        b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).italicAngle = json_obj_getnum(
        dump,
        b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).underlinePosition = json_obj_getnum_fallback(
        dump,
        b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
        -100.0f64,
    );
    (*table).underlineThickness = json_obj_getnum_fallback(
        dump,
        b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
        50.0f64,
    );
    (*table).strokeWidth = json_obj_getnum(
        dump,
        b"strokeWidth\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fontBBoxLeft = json_obj_getnum(
        dump,
        b"fontBBoxLeft\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fontBBoxBottom = json_obj_getnum(
        dump,
        b"fontBBoxBottom\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fontBBoxRight = json_obj_getnum(
        dump,
        b"fontBBoxRight\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fontBBoxTop = json_obj_getnum(
        dump,
        b"fontBBoxTop\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).privateDict = pdFromJson(json_obj_get_type(
        dump,
        b"privates\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    ));
    (*table).cidRegistry = json_obj_getsds(
        dump,
        b"cidRegistry\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).cidOrdering = json_obj_getsds(
        dump,
        b"cidOrdering\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).cidSupplement = json_obj_getint(
        dump,
        b"cidSupplement\0" as *const u8 as *const ::core::ffi::c_char,
    ) as uint32_t;
    (*table).UIDBase = json_obj_getint(
        dump,
        b"UIDBase\0" as *const u8 as *const ::core::ffi::c_char,
    ) as uint32_t;
    (*table).cidCount = json_obj_getint(
        dump,
        b"cidCount\0" as *const u8 as *const ::core::ffi::c_char,
    ) as uint32_t;
    (*table).cidFontVersion = json_obj_getnum(
        dump,
        b"cidFontVersion\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).cidFontRevision = json_obj_getnum(
        dump,
        b"cidFontRevision\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut fdarraydump: *mut json_value = json_obj_get_type(
        dump,
        b"fdArray\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !fdarraydump.is_null() {
        (*table).isCID = true;
        (*table).fdArrayCount = (*fdarraydump).u.object.length as tableid_t;
        (*table).fdArray = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut table_CFF>() as size_t)
                .wrapping_mul((*table).fdArrayCount as size_t),
            872 as ::core::ffi::c_ulong,
        ) as *mut *mut table_CFF;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*table).fdArrayCount as ::core::ffi::c_int {
            let ref mut fresh11 = *(*table).fdArray.offset(j as isize);
            *fresh11 = fdFromJson(
                (*(*fdarraydump).u.object.values.offset(j as isize)).value,
                options,
                false,
            );
            if !(**(*table).fdArray.offset(j as isize)).fontName.is_null() {
                sdsfree((**(*table).fdArray.offset(j as isize)).fontName);
            }
            let ref mut fresh12 = (**(*table).fdArray.offset(j as isize)).fontName;
            *fresh12 = sdsnewlen(
                (*(*fdarraydump).u.object.values.offset(j as isize)).name
                    as *const ::core::ffi::c_void,
                (*(*fdarraydump).u.object.values.offset(j as isize)).name_length as size_t,
            );
            j = j.wrapping_add(1);
        }
    }
    if (*table).fontName.is_null() {
        (*table).fontName = sdsnew(b"CARYLL_CFFFONT\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*table).privateDict.is_null() {
        (*table).privateDict = otfcc_newCff_private();
    }
    if topLevel as ::core::ffi::c_int != 0
        && (*options).force_cid as ::core::ffi::c_int != 0
        && (*table).fdArray.is_null()
    {
        (*table).fdArrayCount = 1 as tableid_t;
        (*table).fdArray = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut table_CFF>() as size_t)
                .wrapping_mul((*table).fdArrayCount as size_t),
            885 as ::core::ffi::c_ulong,
        ) as *mut *mut table_CFF;
        let ref mut fresh13 = *(*table).fdArray.offset(0 as ::core::ffi::c_int as isize);
        *fresh13 = (
            table_iCFF.create.expect("non-null function pointer"))();
        let mut fd0: *mut table_CFF = *(*table).fdArray.offset(0 as ::core::ffi::c_int as isize);
        (*fd0).privateDict = (*table).privateDict;
        (*table).privateDict = otfcc_newCff_private();
        (*fd0).fontName = sdscat(
            sdsdup((*table).fontName),
            b"-subfont0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*table).isCID = true;
    }
    if (*table).isCID as ::core::ffi::c_int != 0 && (*table).cidRegistry.is_null() {
        (*table).cidRegistry = sdsnew(b"CARYLL\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*table).isCID as ::core::ffi::c_int != 0 && (*table).cidOrdering.is_null() {
        (*table).cidOrdering = sdsnew(b"OTFCCAUTOCID\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseCFF(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_CFF {
    let mut dump: *mut json_value = json_obj_get_type(
        root,
        b"CFF_\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if dump.is_null() {
        return ::core::ptr::null_mut::<table_CFF>();
    } else {
        let mut cff: *mut table_CFF = ::core::ptr::null_mut::<table_CFF>();
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"CFF\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            cff = fdFromJson(dump, options, true);
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        return cff;
    };
}
unsafe extern "C" fn cff_make_charstrings(
    mut context: *mut cff_charstring_builder_context,
    mut s: *mut *mut caryll_Buffer,
    mut gs: *mut *mut caryll_Buffer,
    mut ls: *mut *mut caryll_Buffer,
) {
    if (*(*context).glyf).length == 0 as size_t {
        return;
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as size_t) < (*(*context).glyf).length {
        let mut il: *mut cff_CharstringIL = cff_compileGlyphToIL(
            *(*(*context).glyf).items.offset(j as isize) as *mut glyf_Glyph,
            (*context).defaultWidth,
            (*context).nominalWidthX,
        );
        cff_optimizeIL(il, (*context).options);
        cff_insertILToGraph(&raw mut (*context).graph, il);
        free((*il).instr as *mut ::core::ffi::c_void);
        (*il).instr = ::core::ptr::null_mut::<cff_CharstringInstruction>();
        free(il as *mut ::core::ffi::c_void);
        il = ::core::ptr::null_mut::<cff_CharstringIL>();
        j = j.wrapping_add(1);
    }
    cff_ilGraphToBuffers(&raw mut (*context).graph, s, gs, ls, (*context).options);
}
unsafe extern "C" fn sidof(mut h: *mut *mut cff_sid_entry, mut s: sds) -> ::core::ffi::c_int {
    let mut item: *mut cff_sid_entry = ::core::ptr::null_mut::<cff_sid_entry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = s as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
    while _hj_k >= 12 as ::core::ffi::c_uint {
        _hj_i = _hj_i.wrapping_add(
            (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hj_j = _hj_j.wrapping_add(
            (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hf_hashv = _hf_hashv.wrapping_add(
            (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
        _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
    }
    _hf_hashv =
        _hf_hashv.wrapping_add(strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 2136187349163929158;
        }
        10 => {
            current_block_50 = 2136187349163929158;
        }
        9 => {
            current_block_50 = 15000087230028316373;
        }
        8 => {
            current_block_50 = 12618485878637048149;
        }
        7 => {
            current_block_50 = 10172284732124562867;
        }
        6 => {
            current_block_50 = 10320808845489400712;
        }
        5 => {
            current_block_50 = 17757077319915322283;
        }
        4 => {
            current_block_50 = 3097490847089818784;
        }
        3 => {
            current_block_50 = 6116987625208566775;
        }
        2 => {
            current_block_50 = 13858715045951221004;
        }
        1 => {
            current_block_50 = 18086712884960296808;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        2136187349163929158 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 15000087230028316373;
        }
        _ => {}
    }
    match current_block_50 {
        15000087230028316373 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 12618485878637048149;
        }
        _ => {}
    }
    match current_block_50 {
        12618485878637048149 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 10172284732124562867;
        }
        _ => {}
    }
    match current_block_50 {
        10172284732124562867 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 10320808845489400712;
        }
        _ => {}
    }
    match current_block_50 {
        10320808845489400712 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 17757077319915322283;
        }
        _ => {}
    }
    match current_block_50 {
        17757077319915322283 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 3097490847089818784;
        }
        _ => {}
    }
    match current_block_50 {
        3097490847089818784 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 6116987625208566775;
        }
        _ => {}
    }
    match current_block_50 {
        6116987625208566775 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 13858715045951221004;
        }
        _ => {}
    }
    match current_block_50 {
        13858715045951221004 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 18086712884960296808;
        }
        _ => {}
    }
    match current_block_50 {
        18086712884960296808 => {
            _hj_i = _hj_i.wrapping_add(
                *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
        }
        _ => {}
    }
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
    item = ::core::ptr::null_mut::<cff_sid_entry>();
    if !(*h).is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(**h).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                item = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(**h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut cff_sid_entry
                    as *mut cff_sid_entry;
            } else {
                item = ::core::ptr::null_mut::<cff_sid_entry>();
            }
            while !item.is_null() {
                if (*item).hh.hashv == _hf_hashv
                    && (*item).hh.keylen
                        == strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_uint
                {
                    if memcmp(
                        (*item).hh.key,
                        s as *const ::core::ffi::c_void,
                        strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_uint as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*item).hh.hh_next.is_null() {
                    item = ((*item).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(**h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut cff_sid_entry
                        as *mut cff_sid_entry;
                } else {
                    item = ::core::ptr::null_mut::<cff_sid_entry>();
                }
            }
        }
    }
    if !item.is_null() {
        return 391 as ::core::ffi::c_int + (*item).sid;
    } else {
        item = __caryll_allocate_clean(
            ::core::mem::size_of::<cff_sid_entry>() as size_t,
            949 as ::core::ffi::c_ulong,
        ) as *mut cff_sid_entry;
        (*item).sid = (if !(*h).is_null() {
            (*(**h).hh.tbl).num_items
        } else {
            0 as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
        (*item).str_0 = sdsdup(s) as *mut ::core::ffi::c_char;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            (*item).str_0.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = strlen((*item).str_0) as ::core::ffi::c_uint;
        while _hj_k_0 >= 12 as ::core::ffi::c_uint {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j_0 = _hj_j_0.wrapping_add(
                (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _ha_hashv = _ha_hashv.wrapping_add(
                (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv = _ha_hashv.wrapping_add(strlen((*item).str_0) as ::core::ffi::c_uint);
        let mut current_block_169: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 8249353659195211371;
            }
            10 => {
                current_block_169 = 8249353659195211371;
            }
            9 => {
                current_block_169 = 3890748899046558515;
            }
            8 => {
                current_block_169 = 15908325233356615362;
            }
            7 => {
                current_block_169 = 4798082187389923100;
            }
            6 => {
                current_block_169 = 10664847837258368038;
            }
            5 => {
                current_block_169 = 13472887685141297691;
            }
            4 => {
                current_block_169 = 17259833882719531241;
            }
            3 => {
                current_block_169 = 441531399728294563;
            }
            2 => {
                current_block_169 = 8298588265412241393;
            }
            1 => {
                current_block_169 = 17576006425236317873;
            }
            _ => {
                current_block_169 = 16835199615365683821;
            }
        }
        match current_block_169 {
            8249353659195211371 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 3890748899046558515;
            }
            _ => {}
        }
        match current_block_169 {
            3890748899046558515 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 15908325233356615362;
            }
            _ => {}
        }
        match current_block_169 {
            15908325233356615362 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 4798082187389923100;
            }
            _ => {}
        }
        match current_block_169 {
            4798082187389923100 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 10664847837258368038;
            }
            _ => {}
        }
        match current_block_169 {
            10664847837258368038 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 13472887685141297691;
            }
            _ => {}
        }
        match current_block_169 {
            13472887685141297691 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_169 = 17259833882719531241;
            }
            _ => {}
        }
        match current_block_169 {
            17259833882719531241 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 441531399728294563;
            }
            _ => {}
        }
        match current_block_169 {
            441531399728294563 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 8298588265412241393;
            }
            _ => {}
        }
        match current_block_169 {
            8298588265412241393 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 17576006425236317873;
            }
            _ => {}
        }
        match current_block_169 {
            17576006425236317873 => {
                _hj_i_0 =
                    _hj_i_0
                        .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
        (*item).hh.hashv = _ha_hashv;
        (*item).hh.key = (*item).str_0.offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*item).hh.keylen = strlen((*item).str_0) as ::core::ffi::c_uint;
        if (*h).is_null() {
            (*item).hh.next = NULL;
            (*item).hh.prev = NULL;
            (*item).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*item).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*item).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as size_t,
                );
                (*(*item).hh.tbl).tail = &raw mut (*item).hh as *mut UT_hash_handle;
                (*(*item).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*item).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*item).hh.tbl).hho = (&raw mut (*item).hh as *mut ::core::ffi::c_char)
                    .offset_from(item as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as ptrdiff_t;
                (*(*item).hh.tbl).buckets = malloc(
                    (32 as size_t).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                ) as *mut UT_hash_bucket;
                (*(*item).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                if (*(*item).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as size_t)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                    );
                }
            }
            *h = item;
        } else {
            (*item).hh.tbl = (**h).hh.tbl;
            (*item).hh.next = NULL;
            (*item).hh.prev = ((*(**h).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(**h).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(**h).hh.tbl).tail).next = item as *mut ::core::ffi::c_void;
            (*(**h).hh.tbl).tail = &raw mut (*item).hh as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(**h).hh.tbl).num_items = (*(**h).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(**h).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket =
            (*(**h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*item).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*item).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*item).hh as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*item).hh as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*item).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: ::core::ffi::c_uint = 0;
            let mut _he_bkt_i: ::core::ffi::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_hh_nxt: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_new_buckets: *mut UT_hash_bucket =
                ::core::ptr::null_mut::<UT_hash_bucket>();
            let mut _he_newbkt: *mut UT_hash_bucket = ::core::ptr::null_mut::<UT_hash_bucket>();
            _he_new_buckets = malloc(
                (2 as size_t)
                    .wrapping_mul((*(*item).hh.tbl).num_buckets as size_t)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as size_t)
                        .wrapping_mul((*(*item).hh.tbl).num_buckets as size_t)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                );
                (*(*item).hh.tbl).ideal_chain_maxlen = ((*(*item).hh.tbl).num_items
                    >> (*(*item).hh.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*item).hh.tbl).num_items
                        & (*(*item).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint)
                            .wrapping_sub(1 as ::core::ffi::c_uint)
                        != 0 as ::core::ffi::c_uint
                    {
                        1 as ::core::ffi::c_uint
                    } else {
                        0 as ::core::ffi::c_uint
                    },
                );
                (*(*item).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*item).hh.tbl).num_buckets {
                    _he_thh = (*(*(*item).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*item).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*item).hh.tbl).ideal_chain_maxlen {
                            (*(*item).hh.tbl).nonideal_items =
                                (*(*item).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*item).hh.tbl).ideal_chain_maxlen);
                        }
                        (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                        (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UT_hash_handle;
                        if !(*_he_newbkt).hh_head.is_null() {
                            (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                        }
                        (*_he_newbkt).hh_head = _he_thh as *mut UT_hash_handle;
                        _he_thh = _he_hh_nxt;
                    }
                    _he_bkt_i = _he_bkt_i.wrapping_add(1);
                }
                free((*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*item).hh.tbl).num_buckets = (*(*item).hh.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*item).hh.tbl).log2_num_buckets =
                    (*(*item).hh.tbl).log2_num_buckets.wrapping_add(1);
                (*(*item).hh.tbl).buckets = _he_new_buckets;
                (*(*item).hh.tbl).ineff_expands = if (*(*item).hh.tbl).nonideal_items
                    > (*(*item).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*item).hh.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*item).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*item).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        return 391 as ::core::ffi::c_int + (*item).sid;
    };
}
unsafe extern "C" fn cffdict_givemeablank(mut dict: *mut cff_Dict) -> *mut cff_DictEntry {
    (*dict).count = (*dict).count.wrapping_add(1);
    (*dict).ents = __caryll_reallocate(
        (*dict).ents as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<cff_DictEntry>() as size_t).wrapping_mul((*dict).count as size_t),
        959 as ::core::ffi::c_ulong,
    ) as *mut cff_DictEntry;
    return (*dict)
        .ents
        .offset((*dict).count.wrapping_sub(1 as uint32_t) as isize)
        as *mut cff_DictEntry;
}
unsafe extern "C" fn cffdict_input(
    mut dict: *mut cff_Dict,
    mut op: uint32_t,
    mut t: cff_Value_Type,
    mut arity: arity_t,
    mut args: ...
) {
    let mut last: *mut cff_DictEntry = cffdict_givemeablank(dict);
    (*last).op = op;
    (*last).cnt = arity as uint32_t;
    (*last).vals = __caryll_allocate_clean(
        (::core::mem::size_of::<cff_Value>() as size_t).wrapping_mul(arity as size_t),
        966 as ::core::ffi::c_ulong,
    ) as *mut cff_Value;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut j: arity_t = 0 as arity_t;
    while j < arity {
        if t as ::core::ffi::c_uint == cff_DOUBLE as ::core::ffi::c_int as ::core::ffi::c_uint {
            let mut x: ::core::ffi::c_double = ap.arg::<::core::ffi::c_double>();
            if x == round(x) {
                (*(*last).vals.offset(j as isize)).t = cff_INTEGER;
                (*(*last).vals.offset(j as isize)).c2rust_unnamed.i = round(x) as int32_t;
            } else {
                (*(*last).vals.offset(j as isize)).t = cff_DOUBLE;
                (*(*last).vals.offset(j as isize)).c2rust_unnamed.d = x;
            }
        } else {
            let mut x_0: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
            (*(*last).vals.offset(j as isize)).t = t;
            (*(*last).vals.offset(j as isize)).c2rust_unnamed.i = x_0 as int32_t;
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn cffdict_input_array(
    mut dict: *mut cff_Dict,
    mut op: uint32_t,
    mut t: cff_Value_Type,
    mut arity: arity_t,
    mut arr: *mut ::core::ffi::c_double,
) {
    if arity == 0 || arr.is_null() {
        return;
    }
    let mut last: *mut cff_DictEntry = cffdict_givemeablank(dict);
    (*last).op = op;
    (*last).cnt = arity as uint32_t;
    (*last).vals = __caryll_allocate_clean(
        (::core::mem::size_of::<cff_Value>() as size_t).wrapping_mul(arity as size_t),
        994 as ::core::ffi::c_ulong,
    ) as *mut cff_Value;
    let mut j: arity_t = 0 as arity_t;
    while j < arity {
        let mut x: ::core::ffi::c_double = *arr.offset(j as isize);
        if t as ::core::ffi::c_uint == cff_DOUBLE as ::core::ffi::c_int as ::core::ffi::c_uint {
            if x == round(x) {
                (*(*last).vals.offset(j as isize)).t = cff_INTEGER;
                (*(*last).vals.offset(j as isize)).c2rust_unnamed.i = round(x) as int32_t;
            } else {
                (*(*last).vals.offset(j as isize)).t = cff_DOUBLE;
                (*(*last).vals.offset(j as isize)).c2rust_unnamed.d = x;
            }
        } else {
            (*(*last).vals.offset(j as isize)).t = t;
            (*(*last).vals.offset(j as isize)).c2rust_unnamed.i = round(x) as int32_t;
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn cff_make_fd_dict(
    mut fd: *mut table_CFF,
    mut h: *mut *mut cff_sid_entry,
) -> *mut cff_Dict {
    let mut dict: *mut cff_Dict = (
        cff_iDict.create.expect("non-null function pointer"))();
    if !(*fd).cidRegistry.is_null() && !(*fd).cidOrdering.is_null() {
        cffdict_input(
            dict,
            op_ROS as ::core::ffi::c_int as uint32_t,
            cff_INTEGER,
            3 as arity_t,
            sidof(h, (*fd).cidRegistry),
            sidof(h, (*fd).cidOrdering),
            (*fd).cidSupplement,
        );
    }
    if !(*fd).version.is_null() {
        cffdict_input(
            dict,
            op_version as ::core::ffi::c_int as uint32_t,
            cff_INTEGER,
            1 as arity_t,
            sidof(h, (*fd).version),
        );
    }
    if !(*fd).notice.is_null() {
        cffdict_input(
            dict,
            op_Notice as ::core::ffi::c_int as uint32_t,
            cff_INTEGER,
            1 as arity_t,
            sidof(h, (*fd).notice),
        );
    }
    if !(*fd).copyright.is_null() {
        cffdict_input(
            dict,
            op_Copyright as ::core::ffi::c_int as uint32_t,
            cff_INTEGER,
            1 as arity_t,
            sidof(h, (*fd).copyright),
        );
    }
    if !(*fd).fullName.is_null() {
        cffdict_input(
            dict,
            op_FullName as ::core::ffi::c_int as uint32_t,
            cff_INTEGER,
            1 as arity_t,
            sidof(h, (*fd).fullName),
        );
    }
    if !(*fd).familyName.is_null() {
        cffdict_input(
            dict,
            op_FamilyName as ::core::ffi::c_int as uint32_t,
            cff_INTEGER,
            1 as arity_t,
            sidof(h, (*fd).familyName),
        );
    }
    if !(*fd).weight.is_null() {
        cffdict_input(
            dict,
            op_Weight as ::core::ffi::c_int as uint32_t,
            cff_INTEGER,
            1 as arity_t,
            sidof(h, (*fd).weight),
        );
    }
    cffdict_input(
        dict,
        op_FontBBox as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        4 as arity_t,
        (*fd).fontBBoxLeft,
        (*fd).fontBBoxBottom,
        (*fd).fontBBoxRight,
        (*fd).fontBBoxTop,
    );
    cffdict_input(
        dict,
        op_isFixedPitch as ::core::ffi::c_int as uint32_t,
        cff_INTEGER,
        1 as arity_t,
        (*fd).isFixedPitch as ::core::ffi::c_int,
    );
    cffdict_input(
        dict,
        op_ItalicAngle as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*fd).italicAngle,
    );
    cffdict_input(
        dict,
        op_UnderlinePosition as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*fd).underlinePosition,
    );
    cffdict_input(
        dict,
        op_UnderlineThickness as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*fd).underlineThickness,
    );
    cffdict_input(
        dict,
        op_StrokeWidth as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*fd).strokeWidth,
    );
    if !(*fd).fontMatrix.is_null() {
        cffdict_input(
            dict,
            op_FontMatrix as ::core::ffi::c_int as uint32_t,
            cff_DOUBLE,
            6 as arity_t,
            (*(*fd).fontMatrix).a,
            (*(*fd).fontMatrix).b,
            (*(*fd).fontMatrix).c,
            (*(*fd).fontMatrix).d,
            iVQ.getStill.expect("non-null function pointer")((*(*fd).fontMatrix).x),
            iVQ.getStill.expect("non-null function pointer")((*(*fd).fontMatrix).y),
        );
    }
    if !(*fd).fontName.is_null() {
        cffdict_input(
            dict,
            op_FontName as ::core::ffi::c_int as uint32_t,
            cff_INTEGER,
            1 as arity_t,
            sidof(h, (*fd).fontName),
        );
    }
    if (*fd).cidFontVersion != 0. {
        cffdict_input(
            dict,
            op_CIDFontVersion as ::core::ffi::c_int as uint32_t,
            cff_DOUBLE,
            1 as arity_t,
            (*fd).cidFontVersion,
        );
    }
    if (*fd).cidFontRevision != 0. {
        cffdict_input(
            dict,
            op_CIDFontRevision as ::core::ffi::c_int as uint32_t,
            cff_DOUBLE,
            1 as arity_t,
            (*fd).cidFontRevision,
        );
    }
    if (*fd).cidCount != 0 {
        cffdict_input(
            dict,
            op_CIDCount as ::core::ffi::c_int as uint32_t,
            cff_INTEGER,
            1 as arity_t,
            (*fd).cidCount,
        );
    }
    if (*fd).UIDBase != 0 {
        cffdict_input(
            dict,
            op_UIDBase as ::core::ffi::c_int as uint32_t,
            cff_INTEGER,
            1 as arity_t,
            (*fd).UIDBase,
        );
    }
    return dict;
}
unsafe extern "C" fn cff_make_private_dict(mut pd: *mut cff_PrivateDict) -> *mut cff_Dict {
    let mut dict: *mut cff_Dict = ::core::ptr::null_mut::<cff_Dict>();
    dict = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_Dict>() as size_t,
        1057 as ::core::ffi::c_ulong,
    ) as *mut cff_Dict;
    if pd.is_null() {
        return dict;
    }
    cffdict_input_array(
        dict,
        op_BlueValues as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        (*pd).blueValuesCount,
        (*pd).blueValues,
    );
    cffdict_input_array(
        dict,
        op_OtherBlues as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        (*pd).otherBluesCount,
        (*pd).otherBlues,
    );
    cffdict_input_array(
        dict,
        op_FamilyBlues as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        (*pd).familyBluesCount,
        (*pd).familyBlues,
    );
    cffdict_input_array(
        dict,
        op_FamilyOtherBlues as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        (*pd).familyOtherBluesCount,
        (*pd).familyOtherBlues,
    );
    cffdict_input_array(
        dict,
        op_StemSnapH as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        (*pd).stemSnapHCount,
        (*pd).stemSnapH,
    );
    cffdict_input_array(
        dict,
        op_StemSnapV as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        (*pd).stemSnapVCount,
        (*pd).stemSnapV,
    );
    cffdict_input(
        dict,
        op_BlueScale as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*pd).blueScale,
    );
    cffdict_input(
        dict,
        op_BlueShift as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*pd).blueShift,
    );
    cffdict_input(
        dict,
        op_BlueFuzz as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*pd).blueFuzz,
    );
    cffdict_input(
        dict,
        op_StdHW as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*pd).stdHW,
    );
    cffdict_input(
        dict,
        op_StdVW as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*pd).stdVW,
    );
    cffdict_input(
        dict,
        op_ForceBold as ::core::ffi::c_int as uint32_t,
        cff_INTEGER,
        1 as arity_t,
        (*pd).forceBold as ::core::ffi::c_int,
    );
    cffdict_input(
        dict,
        op_LanguageGroup as ::core::ffi::c_int as uint32_t,
        cff_INTEGER,
        1 as arity_t,
        (*pd).languageGroup,
    );
    cffdict_input(
        dict,
        op_ExpansionFactor as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*pd).expansionFactor,
    );
    cffdict_input(
        dict,
        op_initialRandomSeed as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*pd).initialRandomSeed,
    );
    cffdict_input(
        dict,
        op_defaultWidthX as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*pd).defaultWidthX,
    );
    cffdict_input(
        dict,
        op_nominalWidthX as ::core::ffi::c_int as uint32_t,
        cff_DOUBLE,
        1 as arity_t,
        (*pd).nominalWidthX,
    );
    return dict;
}
unsafe extern "C" fn by_sid(
    mut a: *mut cff_sid_entry,
    mut b: *mut cff_sid_entry,
) -> ::core::ffi::c_int {
    return (*a).sid - (*b).sid;
}
unsafe extern "C" fn callback_makestringindex(
    mut context: *mut ::core::ffi::c_void,
    mut i: uint32_t,
) -> *mut caryll_Buffer {
    let mut blobs: *mut *mut caryll_Buffer = context as *mut *mut caryll_Buffer;
    return *blobs.offset(i as isize);
}
unsafe extern "C" fn cffstrings_to_indexblob(mut h: *mut *mut cff_sid_entry) -> *mut caryll_Buffer {
    let mut _hs_i: ::core::ffi::c_uint = 0;
    let mut _hs_looping: ::core::ffi::c_uint = 0;
    let mut _hs_nmerges: ::core::ffi::c_uint = 0;
    let mut _hs_insize: ::core::ffi::c_uint = 0;
    let mut _hs_psize: ::core::ffi::c_uint = 0;
    let mut _hs_qsize: ::core::ffi::c_uint = 0;
    let mut _hs_p: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_q: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_e: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_list: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_tail: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    if !(*h).is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (**h).hh as *mut UT_hash_handle;
        while _hs_looping != 0 as ::core::ffi::c_uint {
            _hs_p = _hs_list;
            _hs_list = ::core::ptr::null_mut::<UT_hash_handle>();
            _hs_tail = ::core::ptr::null_mut::<UT_hash_handle>();
            _hs_nmerges = 0 as ::core::ffi::c_uint;
            while !_hs_p.is_null() {
                _hs_nmerges = _hs_nmerges.wrapping_add(1);
                _hs_q = _hs_p;
                _hs_psize = 0 as ::core::ffi::c_uint;
                _hs_i = 0 as ::core::ffi::c_uint;
                while _hs_i < _hs_insize {
                    _hs_psize = _hs_psize.wrapping_add(1);
                    _hs_q = (if !(*_hs_q).next.is_null() {
                        ((*_hs_q).next as *mut ::core::ffi::c_char)
                            .offset((*(**h).hh.tbl).hho)
                            as *mut UT_hash_handle
                    } else {
                        ::core::ptr::null_mut::<UT_hash_handle>()
                    }) as *mut UT_hash_handle;
                    if _hs_q.is_null() {
                        break;
                    }
                    _hs_i = _hs_i.wrapping_add(1);
                }
                _hs_qsize = _hs_insize;
                while _hs_psize != 0 as ::core::ffi::c_uint
                    || _hs_qsize != 0 as ::core::ffi::c_uint && !_hs_q.is_null()
                {
                    if _hs_psize == 0 as ::core::ffi::c_uint {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(**h).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    } else if _hs_qsize == 0 as ::core::ffi::c_uint || _hs_q.is_null() {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(**h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if by_sid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut cff_sid_entry,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut cff_sid_entry,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(**h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(**h).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    } else {
                        _hs_list = _hs_e;
                    }
                    if !_hs_e.is_null() {
                        (*_hs_e).prev = if !_hs_tail.is_null() {
                            (_hs_tail as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    }
                    _hs_tail = _hs_e;
                }
                _hs_p = _hs_q;
            }
            if !_hs_tail.is_null() {
                (*_hs_tail).next = NULL;
            }
            if _hs_nmerges <= 1 as ::core::ffi::c_uint {
                _hs_looping = 0 as ::core::ffi::c_uint;
                (*(**h).hh.tbl).tail = _hs_tail;
                *h = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut cff_sid_entry
                    as *mut cff_sid_entry;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut blobs: *mut *mut caryll_Buffer = ::core::ptr::null_mut::<*mut caryll_Buffer>();
    let mut n: uint32_t = if !(*h).is_null() {
        (*(**h).hh.tbl).num_items as uint32_t
    } else {
        0 as uint32_t
    };
    blobs = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut caryll_Buffer>() as size_t).wrapping_mul(n as size_t),
        1097 as ::core::ffi::c_ulong,
    ) as *mut *mut caryll_Buffer;
    let mut j: uint32_t = 0 as uint32_t;
    let mut item: *mut cff_sid_entry = ::core::ptr::null_mut::<cff_sid_entry>();
    let mut tmp: *mut cff_sid_entry = ::core::ptr::null_mut::<cff_sid_entry>();
    item = *h;
    tmp = (if !(*h).is_null() { (**h).hh.next } else { NULL }) as *mut cff_sid_entry
        as *mut cff_sid_entry;
    while !item.is_null() {
        let ref mut fresh15 = *blobs.offset(j as isize);
        *fresh15 = bufnew();
        bufwrite_sds(*blobs.offset(j as isize), (*item).str_0 as sds);
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*item).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(**h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((**h).hh.tbl as *mut ::core::ffi::c_void);
            *h = ::core::ptr::null_mut::<cff_sid_entry>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(**h).hh.tbl).tail {
                (*(**h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(**h).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh16 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(**h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh16 = (*_hd_hh_del).next;
            } else {
                *h = (*_hd_hh_del).next as *mut cff_sid_entry as *mut cff_sid_entry;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh17 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(**h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh17 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(**h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(**h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(**h).hh.tbl).num_items = (*(**h).hh.tbl).num_items.wrapping_sub(1);
        }
        sdsfree((*item).str_0 as sds);
        free(item as *mut ::core::ffi::c_void);
        item = ::core::ptr::null_mut::<cff_sid_entry>();
        j = j.wrapping_add(1);
        item = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut cff_sid_entry
            as *mut cff_sid_entry;
    }
    let mut strings: *mut cff_Index = cff_iIndex.fromCallback.expect("non-null function pointer")(
        blobs as *mut ::core::ffi::c_void,
        n,
        Some(
            callback_makestringindex
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> *mut caryll_Buffer,
        ),
    );
    free(blobs as *mut ::core::ffi::c_void);
    blobs = ::core::ptr::null_mut::<*mut caryll_Buffer>();
    let mut final_blob: *mut caryll_Buffer =
        cff_iIndex.build.expect("non-null function pointer")(strings);
    cff_iIndex.free.expect("non-null function pointer")(strings);
    (*final_blob).cursor = (*final_blob).size;
    return final_blob;
}
unsafe extern "C" fn cff_compile_nameindex(mut cff: *mut table_CFF) -> *mut caryll_Buffer {
    let mut nameIndex: *mut cff_Index = (
        cff_iIndex.create.expect("non-null function pointer"))();
    (*nameIndex).count = 1 as arity_t;
    (*nameIndex).offSize = 4 as uint8_t;
    (*nameIndex).offset = __caryll_allocate_clean(
        (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(2 as size_t),
        1121 as ::core::ffi::c_ulong,
    ) as *mut uint32_t;
    if (*cff).fontName.is_null() {
        (*cff).fontName = sdsnew(b"Caryll-CFF-FONT\0" as *const u8 as *const ::core::ffi::c_char);
    }
    *(*nameIndex).offset.offset(0 as ::core::ffi::c_int as isize) = 1 as uint32_t;
    *(*nameIndex).offset.offset(1 as ::core::ffi::c_int as isize) =
        sdslen((*cff).fontName).wrapping_add(1 as size_t) as uint32_t;
    (*nameIndex).data = __caryll_allocate_clean(
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul((1 as size_t).wrapping_add(sdslen((*cff).fontName))),
        1125 as ::core::ffi::c_ulong,
    ) as *mut uint8_t;
    memcpy(
        (*nameIndex).data as *mut ::core::ffi::c_void,
        (*cff).fontName as *const ::core::ffi::c_void,
        sdslen((*cff).fontName),
    );
    let mut buf: *mut caryll_Buffer =
        cff_iIndex.build.expect("non-null function pointer")(nameIndex);
    cff_iIndex.free.expect("non-null function pointer")(nameIndex);
    if !(*cff).fontName.is_null() {
        sdsfree((*cff).fontName);
        (*cff).fontName = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return buf;
}
unsafe extern "C" fn cff_make_charset(
    mut cff: *mut table_CFF,
    mut glyf: *mut table_glyf,
    mut stringHash: *mut *mut cff_sid_entry,
) -> *mut caryll_Buffer {
    let mut charset: *mut cff_Charset = ::core::ptr::null_mut::<cff_Charset>();
    charset = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_Charset>() as size_t,
        1140 as ::core::ffi::c_ulong,
    ) as *mut cff_Charset;
    if (*glyf).length > 1 as size_t {
        (*charset).t = cff_CHARSET_FORMAT2 as ::core::ffi::c_int as uint32_t;
        (*charset).s = 1 as uint32_t;
        (*charset).c2rust_unnamed.f2.format = 2 as uint8_t;
        (*charset).c2rust_unnamed.f2.range2 = __caryll_allocate_clean(
            ::core::mem::size_of::<cff_CharsetRangeFormat2>() as size_t,
            1145 as ::core::ffi::c_ulong,
        ) as *mut cff_CharsetRangeFormat2;
        if (*cff).isCID {
            (*(*charset)
                .c2rust_unnamed
                .f2
                .range2
                .offset(0 as ::core::ffi::c_int as isize))
            .first = 1 as uint16_t;
            (*(*charset)
                .c2rust_unnamed
                .f2
                .range2
                .offset(0 as ::core::ffi::c_int as isize))
            .nleft = (*glyf).length.wrapping_sub(2 as size_t) as uint16_t;
        } else {
            let mut j: glyphid_t = 1 as glyphid_t;
            while (j as size_t) < (*glyf).length {
                sidof(stringHash, (**(*glyf).items.offset(j as isize)).name);
                j = j.wrapping_add(1);
            }
            (*(*charset)
                .c2rust_unnamed
                .f2
                .range2
                .offset(0 as ::core::ffi::c_int as isize))
            .first = sidof(
                stringHash,
                (**(*glyf).items.offset(1 as ::core::ffi::c_int as isize)).name,
            ) as uint16_t;
            (*(*charset)
                .c2rust_unnamed
                .f2
                .range2
                .offset(0 as ::core::ffi::c_int as isize))
            .nleft = (*glyf).length.wrapping_sub(2 as size_t) as uint16_t;
        }
    } else {
        (*charset).t = cff_CHARSET_ISOADOBE as ::core::ffi::c_int as uint32_t;
    }
    let mut c: *mut caryll_Buffer = cff_build_Charset(*charset);
    if (*charset).t == cff_CHARSET_FORMAT2 as ::core::ffi::c_int as uint32_t {
        free((*charset).c2rust_unnamed.f2.range2 as *mut ::core::ffi::c_void);
        (*charset).c2rust_unnamed.f2.range2 = ::core::ptr::null_mut::<cff_CharsetRangeFormat2>();
    }
    free(charset as *mut ::core::ffi::c_void);
    charset = ::core::ptr::null_mut::<cff_Charset>();
    return c;
}
unsafe extern "C" fn cff_make_fdselect(
    mut cff: *mut table_CFF,
    mut glyf: *mut table_glyf,
) -> *mut caryll_Buffer {
    let mut fdi0: uint8_t = 0;
    if !(*cff).isCID {
        return bufnew();
    }
    let mut ranges: uint32_t = 1 as uint32_t;
    let mut current: uint8_t = 0 as uint8_t;
    let mut fds: *mut cff_FDSelect = ::core::ptr::null_mut::<cff_FDSelect>();
    fds = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_FDSelect>() as size_t,
        1171 as ::core::ffi::c_ulong,
    ) as *mut cff_FDSelect;
    (*fds).t = cff_FDSELECT_UNSPECED as ::core::ffi::c_int as uint32_t;
    if !((*glyf).length == 0) {
        fdi0 = (**(*glyf).items.offset(0 as ::core::ffi::c_int as isize))
            .fdSelect
            .index as uint8_t;
        if fdi0 as ::core::ffi::c_int > (*cff).fdArrayCount as ::core::ffi::c_int {
            fdi0 = 0 as uint8_t;
        }
        current = fdi0;
        let mut j: glyphid_t = 1 as glyphid_t;
        while (j as size_t) < (*glyf).length {
            let mut fdi: uint8_t = (**(*glyf).items.offset(j as isize)).fdSelect.index as uint8_t;
            if fdi as ::core::ffi::c_int > (*cff).fdArrayCount as ::core::ffi::c_int {
                fdi = 0 as uint8_t;
            }
            if fdi as ::core::ffi::c_int != current as ::core::ffi::c_int {
                current = fdi;
                ranges = ranges.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        (*fds).c2rust_unnamed.f3.range3 = __caryll_allocate_clean(
            (::core::mem::size_of::<cff_FDSelectRangeFormat3>() as size_t)
                .wrapping_mul(ranges as size_t),
            1185 as ::core::ffi::c_ulong,
        ) as *mut cff_FDSelectRangeFormat3;
        (*(*fds)
            .c2rust_unnamed
            .f3
            .range3
            .offset(0 as ::core::ffi::c_int as isize))
        .first = 0 as uint16_t;
        current = fdi0;
        (*(*fds)
            .c2rust_unnamed
            .f3
            .range3
            .offset(0 as ::core::ffi::c_int as isize))
        .fd = current;
        let mut j_0: glyphid_t = 1 as glyphid_t;
        while (j_0 as size_t) < (*glyf).length {
            let mut fdi_0: uint8_t =
                (**(*glyf).items.offset(j_0 as isize)).fdSelect.index as uint8_t;
            if fdi_0 as ::core::ffi::c_int > (*cff).fdArrayCount as ::core::ffi::c_int {
                fdi_0 = 0 as uint8_t;
            }
            if (**(*glyf).items.offset(j_0 as isize)).fdSelect.index as ::core::ffi::c_int
                != current as ::core::ffi::c_int
            {
                current = fdi_0;
                (*fds).s = (*fds).s.wrapping_add(1);
                (*(*fds).c2rust_unnamed.f3.range3.offset((*fds).s as isize)).first =
                    j_0 as uint16_t;
                (*(*fds).c2rust_unnamed.f3.range3.offset((*fds).s as isize)).fd = current;
            }
            j_0 = j_0.wrapping_add(1);
        }
        (*fds).t = cff_FDSELECT_FORMAT3 as ::core::ffi::c_int as uint32_t;
        (*fds).s = ranges;
        (*fds).c2rust_unnamed.f3.format = 3 as uint8_t;
        (*fds).c2rust_unnamed.f3.nranges = ranges as uint16_t;
        (*fds).c2rust_unnamed.f3.sentinel = (*glyf).length as uint16_t;
    }
    let mut e: *mut caryll_Buffer = cff_build_FDSelect(*fds);
    cff_close_FDSelect(*fds);
    free(fds as *mut ::core::ffi::c_void);
    fds = ::core::ptr::null_mut::<cff_FDSelect>();
    return e;
}
unsafe extern "C" fn callback_makefd(
    mut _context: *mut ::core::ffi::c_void,
    mut i: uint32_t,
) -> *mut caryll_Buffer {
    let mut context: *mut fdarray_compile_context = _context as *mut fdarray_compile_context;
    let mut fd: *mut cff_Dict = cff_make_fd_dict(
        *(*context).fdArray.offset(i as isize),
        (*context).stringHash,
    );
    let mut blob: *mut caryll_Buffer = cff_iDict.build.expect("non-null function pointer")(fd);
    bufwrite_bufdel(
        blob,
        cff_buildOffset(0xeeeeeeee as ::core::ffi::c_uint as int32_t),
    );
    bufwrite_bufdel(
        blob,
        cff_buildOffset(0xffffffff as ::core::ffi::c_uint as int32_t),
    );
    bufwrite_bufdel(
        blob,
        cff_encodeCffOperator(op_Private as ::core::ffi::c_int as int32_t),
    );
    cff_iDict.build.expect("non-null function pointer")(fd);
    return blob;
}
unsafe extern "C" fn cff_make_fdarray(
    mut fdArrayCount: tableid_t,
    mut fdArray: *mut *mut table_CFF,
    mut stringHash: *mut *mut cff_sid_entry,
) -> *mut cff_Index {
    let mut context: fdarray_compile_context = fdarray_compile_context {
        fdArray: ::core::ptr::null_mut::<*mut table_CFF>(),
        stringHash: ::core::ptr::null_mut::<*mut cff_sid_entry>(),
    };
    context.fdArray = fdArray;
    context.stringHash = stringHash;
    return cff_iIndex.fromCallback.expect("non-null function pointer")(
        &raw mut context as *mut ::core::ffi::c_void,
        fdArrayCount as uint32_t,
        Some(
            callback_makefd
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> *mut caryll_Buffer,
        ),
    );
}
unsafe extern "C" fn writecff_CIDKeyed(
    mut cff: *mut table_CFF,
    mut glyf: *mut table_glyf,
    mut options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    let mut blob: *mut caryll_Buffer = bufnew();
    let mut stringHash: *mut cff_sid_entry = ::core::ptr::null_mut::<cff_sid_entry>();
    let mut h: *mut caryll_Buffer = cff_buildHeader();
    let mut n: *mut caryll_Buffer = cff_compile_nameindex(cff);
    let mut top: *mut cff_Dict = cff_make_fd_dict(cff, &raw mut stringHash);
    let mut t: *mut caryll_Buffer = cff_iDict.build.expect("non-null function pointer")(top);
    cff_iDict.free.expect("non-null function pointer")(top);
    let mut top_pd: *mut cff_Dict = cff_make_private_dict((*cff).privateDict);
    let mut p: *mut caryll_Buffer = cff_iDict.build.expect("non-null function pointer")(top_pd);
    bufwrite_bufdel(
        p,
        cff_buildOffset(0xffffffff as ::core::ffi::c_uint as int32_t),
    );
    bufwrite_bufdel(
        p,
        cff_encodeCffOperator(op_Subrs as ::core::ffi::c_int as int32_t),
    );
    cff_iDict.free.expect("non-null function pointer")(top_pd);
    let mut e: *mut caryll_Buffer = cff_make_fdselect(cff, glyf);
    let mut fdArrayIndex: *mut cff_Index = ::core::ptr::null_mut::<cff_Index>();
    let mut r: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    if (*cff).isCID {
        fdArrayIndex = cff_make_fdarray((*cff).fdArrayCount, (*cff).fdArray, &raw mut stringHash);
        r = cff_iIndex.build.expect("non-null function pointer")(fdArrayIndex);
    } else {
        r = __caryll_allocate_clean(
            ::core::mem::size_of::<caryll_Buffer>() as size_t,
            1265 as ::core::ffi::c_ulong,
        ) as *mut caryll_Buffer;
    }
    let mut c: *mut caryll_Buffer = cff_make_charset(cff, glyf, &raw mut stringHash);
    let mut i: *mut caryll_Buffer = cffstrings_to_indexblob(&raw mut stringHash);
    let mut s: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    let mut gs: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    let mut ls: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    let mut g2cContext: cff_charstring_builder_context = cff_charstring_builder_context {
        glyf: ::core::ptr::null_mut::<table_glyf>(),
        defaultWidth: 0,
        nominalWidthX: 0,
        options: ::core::ptr::null::<otfcc_Options>(),
        graph: cff_SubrGraph {
            root: ::core::ptr::null_mut::<cff_SubrRule>(),
            last: ::core::ptr::null_mut::<cff_SubrRule>(),
            diagramIndex: ::core::ptr::null_mut::<cff_SubrDiagramIndex>(),
            totalRules: 0,
            totalCharStrings: 0,
            doSubroutinize: false,
        },
    };
    g2cContext.glyf = glyf;
    g2cContext.defaultWidth = (*(*cff).privateDict).defaultWidthX as uint16_t;
    g2cContext.nominalWidthX = (*(*cff).privateDict).nominalWidthX as uint16_t;
    g2cContext.options = options;
    cff_iSubrGraph.init.expect("non-null function pointer")(&raw mut g2cContext.graph);
    g2cContext.graph.doSubroutinize = (*options).cff_doSubroutinize;
    cff_make_charstrings(&raw mut g2cContext, &raw mut s, &raw mut gs, &raw mut ls);
    cff_iSubrGraph.dispose.expect("non-null function pointer")(&raw mut g2cContext.graph);
    let mut additionalTopDictOpsSize: uint32_t = 0 as uint32_t;
    let mut off: uint32_t = (*h)
        .size
        .wrapping_add((*n).size)
        .wrapping_add(11 as size_t)
        .wrapping_add((*t).size) as uint32_t;
    if (*c).size != 0 as size_t {
        additionalTopDictOpsSize = additionalTopDictOpsSize.wrapping_add(6 as uint32_t);
    }
    if (*e).size != 0 as size_t {
        additionalTopDictOpsSize = additionalTopDictOpsSize.wrapping_add(7 as uint32_t);
    }
    if (*s).size != 0 as size_t {
        additionalTopDictOpsSize = additionalTopDictOpsSize.wrapping_add(6 as uint32_t);
    }
    if (*p).size != 0 as size_t {
        additionalTopDictOpsSize = additionalTopDictOpsSize.wrapping_add(11 as uint32_t);
    }
    if (*r).size != 0 as size_t {
        additionalTopDictOpsSize = additionalTopDictOpsSize.wrapping_add(7 as uint32_t);
    }
    bufwrite_bufdel(blob, h);
    bufwrite_bufdel(blob, n);
    let mut delta_size: int32_t = (*t)
        .size
        .wrapping_add(additionalTopDictOpsSize as size_t)
        .wrapping_add(1 as size_t) as uint32_t as int32_t;
    bufwrite_bufdel(
        blob,
        bufninit(
            11 as uint32_t,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            delta_size >> 24 as ::core::ffi::c_int & 0xff as int32_t,
            delta_size >> 16 as ::core::ffi::c_int & 0xff as int32_t,
            delta_size >> 8 as ::core::ffi::c_int & 0xff as int32_t,
            delta_size & 0xff as int32_t,
        ),
    );
    bufwrite_bufdel(blob, t);
    off = (off as size_t).wrapping_add(
        (additionalTopDictOpsSize as size_t)
            .wrapping_add((*i).size)
            .wrapping_add((*gs).size),
    ) as uint32_t as uint32_t;
    if (*c).size != 0 as size_t {
        bufwrite_bufdel(blob, cff_buildOffset(off as int32_t));
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator(op_charset as ::core::ffi::c_int as int32_t),
        );
        off = (off as size_t).wrapping_add((*c).size) as uint32_t as uint32_t;
    }
    if (*e).size != 0 as size_t {
        bufwrite_bufdel(blob, cff_buildOffset(off as int32_t));
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator(op_FDSelect as ::core::ffi::c_int as int32_t),
        );
        off = (off as size_t).wrapping_add((*e).size) as uint32_t as uint32_t;
    }
    if (*s).size != 0 as size_t {
        bufwrite_bufdel(blob, cff_buildOffset(off as int32_t));
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator(op_CharStrings as ::core::ffi::c_int as int32_t),
        );
        off = (off as size_t).wrapping_add((*s).size) as uint32_t as uint32_t;
    }
    if (*p).size != 0 as size_t {
        bufwrite_bufdel(blob, cff_buildOffset((*p).size as uint32_t as int32_t));
        bufwrite_bufdel(blob, cff_buildOffset(off as int32_t));
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator(op_Private as ::core::ffi::c_int as int32_t),
        );
        off = (off as size_t).wrapping_add((*p).size) as uint32_t as uint32_t;
    }
    if (*r).size != 0 as size_t {
        bufwrite_bufdel(blob, cff_buildOffset(off as int32_t));
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator(op_FDArray as ::core::ffi::c_int as int32_t),
        );
        off = (off as size_t).wrapping_add((*r).size) as uint32_t as uint32_t;
    }
    bufwrite_bufdel(blob, i);
    bufwrite_bufdel(blob, gs);
    bufwrite_bufdel(blob, c);
    bufwrite_bufdel(blob, e);
    bufwrite_bufdel(blob, s);
    let mut startingPositionOfPrivates: *mut size_t = ::core::ptr::null_mut::<size_t>();
    startingPositionOfPrivates = __caryll_allocate_clean(
        (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(
            (1 as ::core::ffi::c_int + (*cff).fdArrayCount as ::core::ffi::c_int) as size_t,
        ),
        1350 as ::core::ffi::c_ulong,
    ) as *mut size_t;
    *startingPositionOfPrivates.offset(0 as ::core::ffi::c_int as isize) = (*blob).cursor;
    bufwrite_bufdel(blob, p);
    let mut endingPositionOfPrivates: *mut size_t = ::core::ptr::null_mut::<size_t>();
    endingPositionOfPrivates = __caryll_allocate_clean(
        (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(
            (1 as ::core::ffi::c_int + (*cff).fdArrayCount as ::core::ffi::c_int) as size_t,
        ),
        1354 as ::core::ffi::c_ulong,
    ) as *mut size_t;
    *endingPositionOfPrivates.offset(0 as ::core::ffi::c_int as isize) = (*blob).cursor;
    if (*cff).isCID {
        let mut fdArrayPrivatesStartOffset: uint32_t = off;
        let mut fdArrayPrivates: *mut *mut caryll_Buffer =
            ::core::ptr::null_mut::<*mut caryll_Buffer>();
        fdArrayPrivates = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut caryll_Buffer>() as size_t)
                .wrapping_mul((*cff).fdArrayCount as size_t),
            1359 as ::core::ffi::c_ulong,
        ) as *mut *mut caryll_Buffer;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*cff).fdArrayCount as ::core::ffi::c_int {
            let mut pd: *mut cff_Dict =
                cff_make_private_dict((**(*cff).fdArray.offset(j as isize)).privateDict);
            let mut p_0: *mut caryll_Buffer =
                cff_iDict.build.expect("non-null function pointer")(pd);
            bufwrite_bufdel(
                p_0,
                cff_buildOffset(0xffffffff as ::core::ffi::c_uint as int32_t),
            );
            bufwrite_bufdel(
                p_0,
                cff_encodeCffOperator(op_Subrs as ::core::ffi::c_int as int32_t),
            );
            cff_iDict.free.expect("non-null function pointer")(pd);
            let ref mut fresh14 = *fdArrayPrivates.offset(j as isize);
            *fresh14 = p_0;
            let mut privateLengthPtr: *mut uint8_t = (*fdArrayIndex).data.offset(
                (*(*fdArrayIndex)
                    .offset
                    .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(11 as uint32_t) as isize,
            ) as *mut uint8_t;
            *privateLengthPtr.offset(0 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 24 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
            *privateLengthPtr.offset(1 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 16 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
            *privateLengthPtr.offset(2 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 8 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
            *privateLengthPtr.offset(3 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 0 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
            let mut privateOffsetPtr: *mut uint8_t = (*fdArrayIndex).data.offset(
                (*(*fdArrayIndex)
                    .offset
                    .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(6 as uint32_t) as isize,
            ) as *mut uint8_t;
            *privateOffsetPtr.offset(0 as ::core::ffi::c_int as isize) =
                (fdArrayPrivatesStartOffset >> 24 as ::core::ffi::c_int & 0xff as uint32_t)
                    as uint8_t;
            *privateOffsetPtr.offset(1 as ::core::ffi::c_int as isize) =
                (fdArrayPrivatesStartOffset >> 16 as ::core::ffi::c_int & 0xff as uint32_t)
                    as uint8_t;
            *privateOffsetPtr.offset(2 as ::core::ffi::c_int as isize) =
                (fdArrayPrivatesStartOffset >> 8 as ::core::ffi::c_int & 0xff as uint32_t)
                    as uint8_t;
            *privateOffsetPtr.offset(3 as ::core::ffi::c_int as isize) =
                (fdArrayPrivatesStartOffset >> 0 as ::core::ffi::c_int & 0xff as uint32_t)
                    as uint8_t;
            fdArrayPrivatesStartOffset = (fdArrayPrivatesStartOffset as size_t)
                .wrapping_add((*p_0).size) as uint32_t
                as uint32_t;
            j = j.wrapping_add(1);
        }
        buffree(r);
        r = cff_iIndex.build.expect("non-null function pointer")(fdArrayIndex);
        cff_iIndex.free.expect("non-null function pointer")(fdArrayIndex);
        bufwrite_bufdel(blob, r);
        let mut j_0: tableid_t = 0 as tableid_t;
        while (j_0 as ::core::ffi::c_int) < (*cff).fdArrayCount as ::core::ffi::c_int {
            *startingPositionOfPrivates
                .offset((j_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (*blob).cursor;
            bufwrite_bufdel(blob, *fdArrayPrivates.offset(j_0 as isize));
            *endingPositionOfPrivates
                .offset((j_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (*blob).cursor;
            j_0 = j_0.wrapping_add(1);
        }
        free(fdArrayPrivates as *mut ::core::ffi::c_void);
        fdArrayPrivates = ::core::ptr::null_mut::<*mut caryll_Buffer>();
    } else {
        bufwrite_bufdel(blob, r);
    }
    let mut positionOfLocalSubroutines: size_t = (*blob).cursor;
    bufwrite_bufdel(blob, ls);
    let mut j_1: tableid_t = 0 as tableid_t;
    while (j_1 as ::core::ffi::c_int)
        < (*cff).fdArrayCount as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    {
        let mut lsOffset: size_t = positionOfLocalSubroutines
            .wrapping_sub(*startingPositionOfPrivates.offset(j_1 as isize));
        let mut ptr: *mut uint8_t = (*blob).data.offset(
            (*endingPositionOfPrivates.offset(j_1 as isize)).wrapping_sub(5 as size_t) as isize,
        ) as *mut uint8_t;
        *ptr.offset(0 as ::core::ffi::c_int as isize) =
            (lsOffset >> 24 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
        *ptr.offset(1 as ::core::ffi::c_int as isize) =
            (lsOffset >> 16 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
        *ptr.offset(2 as ::core::ffi::c_int as isize) =
            (lsOffset >> 8 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
        *ptr.offset(3 as ::core::ffi::c_int as isize) =
            (lsOffset >> 0 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
        j_1 = j_1.wrapping_add(1);
    }
    free(startingPositionOfPrivates as *mut ::core::ffi::c_void);
    startingPositionOfPrivates = ::core::ptr::null_mut::<size_t>();
    free(endingPositionOfPrivates as *mut ::core::ffi::c_void);
    endingPositionOfPrivates = ::core::ptr::null_mut::<size_t>();
    return blob;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildCFF(
    cffAndGlyf: table_CFFAndGlyf,
    mut options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    return writecff_CIDKeyed(cffAndGlyf.meta, cffAndGlyf.glyphs, options);
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_getsds(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> sds {
    let mut v: *mut json_value = json_obj_get_type(obj, key, json_string);
    if v.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        return sdsnewlen(
            (*v).u.string.ptr as *const ::core::ffi::c_void,
            (*v).u.string.length as size_t,
        );
    };
}
#[inline]
unsafe extern "C" fn json_numof(mut cv: *const json_value) -> ::core::ffi::c_double {
    if !cv.is_null()
        && (*cv).type_0 as ::core::ffi::c_uint
            == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*cv).u.integer as ::core::ffi::c_double;
    }
    if !cv.is_null()
        && (*cv).type_0 as ::core::ffi::c_uint
            == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*cv).u.dbl;
    }
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}
#[inline]
unsafe extern "C" fn json_obj_getnum(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0.0f64;
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0.0f64;
}
#[inline]
unsafe extern "C" fn json_obj_getint(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> int32_t {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as int32_t;
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as int32_t;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl as int32_t;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0 as int32_t;
}
#[inline]
unsafe extern "C" fn json_obj_getnum_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return fallback;
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return fallback;
}
#[inline]
unsafe extern "C" fn json_obj_getbool(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> bool {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false;
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_boolean as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.boolean != 0;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return false;
}
#[inline]
unsafe extern "C" fn json_from_sds(str: sds) -> *mut json_value {
    return json_string_new_length(
        sdslen(str) as ::core::ffi::c_uint,
        str as *const ::core::ffi::c_char,
    );
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
