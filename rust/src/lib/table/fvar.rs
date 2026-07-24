extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatsds(s: sds, t: sds) -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn sdsfromlonglong(value: ::core::ffi::c_longlong) -> sds;
    fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double;
    static iVV: __caryll_vectorinterface_VV;
    fn vq_AxisSpanIsOne(a: *const vq_AxisSpan) -> bool;
    fn vq_deleteRegion(region: *mut vq_Region);
    static iVQ: __caryll_vectorinterface_VQ;
    static vf_iAxes: __caryll_vectorinterface_vf_Axes;
    fn json_array_new(length: size_t) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: size_t) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_object_push_length(
        object: *mut json_value,
        name_length: ::core::ffi::c_uint,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
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
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> size_t;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::src::lib::support::stdio::FILE;
use crate::src::lib::support::alloc::{__caryll_allocate_clean};
use crate::src::lib::support::cvec::{
    cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push,
    cvec_resize_to, CVecRaw,
};
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub type ptrdiff_t = isize;
pub type f16dot16 = int32_t;
pub type tableid_t = uint16_t;
pub type shapeid_t = uint16_t;
pub type pos_t = ::core::ffi::c_double;
pub type scale_t = ::core::ffi::c_double;
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
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed_4 = 10;
pub const log_vl_info: C2RustUnnamed_4 = 5;
pub const log_vl_notice: C2RustUnnamed_4 = 2;
pub const log_vl_important: C2RustUnnamed_4 = 1;
pub const log_vl_critical: C2RustUnnamed_4 = 0;
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
pub struct __caryll_vectorinterface_VV {
    pub init: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VV, *const VV) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VV, *mut VV) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VV, VV) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VV, VV) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VV>,
    pub free: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut VV, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut VV, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut VV>,
    pub fill: Option<unsafe extern "C" fn(*mut VV, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VV, pos_t) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut VV) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut VV) -> pos_t>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut VV, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut VV,
            Option<unsafe extern "C" fn(*const pos_t, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut VV,
            Option<unsafe extern "C" fn(*const pos_t, *const pos_t) -> ::core::ffi::c_int>,
        ) -> (),
    >,
    pub neutral: Option<unsafe extern "C" fn(tableid_t) -> VV>,
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
    pub val: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub still: pos_t,
    pub delta: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub struct __caryll_vectorinterface_vf_Axes {
    pub init: Option<unsafe extern "C" fn(*mut vf_Axes) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut vf_Axes, *const vf_Axes) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut vf_Axes, *mut vf_Axes) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vf_Axes) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vf_Axes, vf_Axes) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut vf_Axes, vf_Axes) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut vf_Axes>,
    pub free: Option<unsafe extern "C" fn(*mut vf_Axes) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut vf_Axes, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut vf_Axes, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut vf_Axes>,
    pub fill: Option<unsafe extern "C" fn(*mut vf_Axes, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut vf_Axes) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut vf_Axes, vf_Axis) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut vf_Axes) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut vf_Axes) -> vf_Axis>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut vf_Axes, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut vf_Axes,
            Option<unsafe extern "C" fn(*const vf_Axis, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut vf_Axes,
            Option<unsafe extern "C" fn(*const vf_Axis, *const vf_Axis) -> ::core::ffi::c_int>,
        ) -> (),
    >,
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
pub struct __caryll_elementinterface_fvar_Instance {
    pub init: Option<unsafe extern "C" fn(*mut fvar_Instance) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut fvar_Instance, *const fvar_Instance) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut fvar_Instance, *mut fvar_Instance) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut fvar_Instance) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut fvar_Instance, fvar_Instance) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut fvar_Instance, fvar_Instance) -> ()>,
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
pub struct __caryll_vectorinterface_fvar_InstanceList {
    pub init: Option<unsafe extern "C" fn(*mut fvar_InstanceList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut fvar_InstanceList, *const fvar_InstanceList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut fvar_InstanceList, *mut fvar_InstanceList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut fvar_InstanceList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut fvar_InstanceList, fvar_InstanceList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut fvar_InstanceList, fvar_InstanceList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut fvar_InstanceList>,
    pub free: Option<unsafe extern "C" fn(*mut fvar_InstanceList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut fvar_InstanceList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut fvar_InstanceList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut fvar_InstanceList>,
    pub fill: Option<unsafe extern "C" fn(*mut fvar_InstanceList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut fvar_InstanceList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut fvar_InstanceList, fvar_Instance) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut fvar_InstanceList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut fvar_InstanceList) -> fvar_Instance>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut fvar_InstanceList, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut fvar_InstanceList,
            Option<unsafe extern "C" fn(*const fvar_Instance, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut fvar_InstanceList,
            Option<
                unsafe extern "C" fn(
                    *const fvar_Instance,
                    *const fvar_Instance,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
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
pub struct __caryll_elementinterface_table_fvar {
    pub init: Option<unsafe extern "C" fn(*mut table_fvar) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_fvar, *const table_fvar) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_fvar, *mut table_fvar) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_fvar) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_fvar, table_fvar) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_fvar, table_fvar) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_fvar>,
    pub free: Option<unsafe extern "C" fn(*mut table_fvar) -> ()>,
    pub registerRegion:
        Option<unsafe extern "C" fn(*mut table_fvar, *mut vq_Region) -> *const vq_Region>,
    pub findMasterByRegion:
        Option<unsafe extern "C" fn(*const table_fvar, *const vq_Region) -> *const fvar_Master>,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct InstanceRecord {
    pub subfamilyNameID: uint16_t,
    pub flags: uint16_t,
    pub coordinates: [f16dot16; 0],
}
pub type font_file_pointer = *mut uint8_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct FVARHeader {
    pub majorVersion: uint16_t,
    pub minorVersion: uint16_t,
    pub axesArrayOffset: uint16_t,
    pub reserved1: uint16_t,
    pub axisCount: uint16_t,
    pub axisSize: uint16_t,
    pub instanceCount: uint16_t,
    pub instanceSize: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct VariationAxisRecord {
    pub axisTag: uint32_t,
    pub minValue: f16dot16,
    pub defaultValue: f16dot16,
    pub maxValue: f16dot16,
    pub flags: uint16_t,
    pub axisNameID: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_serialize_opts {
    pub mode: ::core::ffi::c_int,
    pub opts: ::core::ffi::c_int,
    pub indent_size: ::core::ffi::c_int,
}
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
#[inline]
unsafe extern "C" fn initFvarInstance(mut inst: *mut fvar_Instance) {
    memset(
        inst as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<fvar_Instance>() as size_t,
    );
    iVV.init.expect("non-null function pointer")(&raw mut (*inst).coordinates);
}
#[inline]
unsafe extern "C" fn disposeFvarInstance(mut inst: *mut fvar_Instance) {
    iVV.dispose.expect("non-null function pointer")(&raw mut (*inst).coordinates);
}
#[no_mangle]
pub static mut fvar_iInstance: __caryll_elementinterface_fvar_Instance = {
    __caryll_elementinterface_fvar_Instance {
        init: Some(fvar_Instance_init as unsafe extern "C" fn(*mut fvar_Instance) -> ()),
        copy: Some(
            fvar_Instance_copy
                as unsafe extern "C" fn(*mut fvar_Instance, *const fvar_Instance) -> (),
        ),
        move_0: Some(
            fvar_Instance_move
                as unsafe extern "C" fn(*mut fvar_Instance, *mut fvar_Instance) -> (),
        ),
        dispose: Some(fvar_Instance_dispose as unsafe extern "C" fn(*mut fvar_Instance) -> ()),
        replace: Some(
            fvar_Instance_replace as unsafe extern "C" fn(*mut fvar_Instance, fvar_Instance) -> (),
        ),
        copyReplace: Some(
            fvar_Instance_copyReplace
                as unsafe extern "C" fn(*mut fvar_Instance, fvar_Instance) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn fvar_Instance_copyReplace(mut dst: *mut fvar_Instance, src: fvar_Instance) {
    fvar_Instance_dispose(dst);
    fvar_Instance_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn fvar_Instance_dispose(mut x: *mut fvar_Instance) {
    disposeFvarInstance(x);
}
#[inline]
unsafe extern "C" fn fvar_Instance_move(mut dst: *mut fvar_Instance, mut src: *mut fvar_Instance) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<fvar_Instance>() as size_t,
    );
    fvar_Instance_init(src);
}
#[inline]
unsafe extern "C" fn fvar_Instance_init(mut x: *mut fvar_Instance) {
    initFvarInstance(x);
}
#[inline]
unsafe extern "C" fn fvar_Instance_copy(
    mut dst: *mut fvar_Instance,
    mut src: *const fvar_Instance,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<fvar_Instance>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn fvar_Instance_replace(mut dst: *mut fvar_Instance, src: fvar_Instance) {
    fvar_Instance_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<fvar_Instance>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_free(mut x: *mut fvar_InstanceList) {
    if x.is_null() {
        return;
    }
    fvar_InstanceList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_resizeTo(arr: *mut fvar_InstanceList, target: size_t) {
    cvec_resize_to(fvar_InstanceList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_createN(mut n: size_t) -> *mut fvar_InstanceList {
    let mut t: *mut fvar_InstanceList =
        malloc(::core::mem::size_of::<fvar_InstanceList>() as size_t) as *mut fvar_InstanceList;
    fvar_InstanceList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_create() -> *mut fvar_InstanceList {
    let mut x: *mut fvar_InstanceList =
        malloc(::core::mem::size_of::<fvar_InstanceList>() as size_t) as *mut fvar_InstanceList;
    fvar_InstanceList_init(x);
    return x;
}
#[inline]
unsafe fn fvar_InstanceList_as_cvec(arr: *mut fvar_InstanceList) -> *mut CVecRaw<fvar_Instance> {
    arr as *mut CVecRaw<fvar_Instance>
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_init(arr: *mut fvar_InstanceList) {
    cvec_init(fvar_InstanceList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_move(dst: *mut fvar_InstanceList, src: *mut fvar_InstanceList) {
    cvec_move(fvar_InstanceList_as_cvec(dst), fvar_InstanceList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_filterEnv(
    mut arr: *mut fvar_InstanceList,
    mut fn_0: Option<unsafe extern "C" fn(*const fvar_Instance, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: size_t = 0 as size_t;
    let mut k: size_t = 0 as size_t;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut fvar_Instance,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if fvar_iInstance.dispose.is_some() {
                fvar_iInstance.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut fvar_Instance,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_disposeItem(mut arr: *mut fvar_InstanceList, mut n: size_t) {
    if fvar_iInstance.dispose.is_some() {
        fvar_iInstance.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut fvar_Instance
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_sort(
    mut arr: *mut fvar_InstanceList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const fvar_Instance, *const fvar_Instance) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<fvar_Instance>() as size_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const fvar_Instance,
                    *const fvar_Instance,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_fill(mut arr: *mut fvar_InstanceList, mut n: size_t) {
    while (*arr).length < n {
        let mut x: fvar_Instance = fvar_Instance {
            subfamilyNameID: 0,
            flags: 0,
            coordinates: VV {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<pos_t>(),
            },
            postScriptNameID: 0,
        };
        if fvar_iInstance.init.is_some() {
            fvar_iInstance.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<fvar_Instance>() as size_t,
            );
        }
        fvar_InstanceList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_push(arr: *mut fvar_InstanceList, elem: fvar_Instance) {
    cvec_push(fvar_InstanceList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_grow(arr: *mut fvar_InstanceList) {
    cvec_grow(fvar_InstanceList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_growTo(arr: *mut fvar_InstanceList, target: size_t) {
    cvec_grow_to(fvar_InstanceList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_pop(arr: *mut fvar_InstanceList) -> fvar_Instance {
    cvec_pop(fvar_InstanceList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_copyReplace(
    mut dst: *mut fvar_InstanceList,
    src: fvar_InstanceList,
) {
    fvar_InstanceList_dispose(dst);
    fvar_InstanceList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_copy(
    mut dst: *mut fvar_InstanceList,
    mut src: *const fvar_InstanceList,
) {
    fvar_InstanceList_init(dst);
    fvar_InstanceList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if fvar_iInstance.copy.is_some() {
        let mut j: size_t = 0 as size_t;
        while j < (*src).length {
            fvar_iInstance.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut fvar_Instance,
                (*src).items.offset(j as isize) as *mut fvar_Instance as *const fvar_Instance,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: size_t = 0 as size_t;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_dispose(mut arr: *mut fvar_InstanceList) {
    if arr.is_null() {
        return;
    }
    if fvar_iInstance.dispose.is_some() {
        let mut j: size_t = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            fvar_iInstance.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut fvar_Instance,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<fvar_Instance>();
    (*arr).length = 0 as size_t;
    (*arr).capacity = 0 as size_t;
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_replace(
    mut dst: *mut fvar_InstanceList,
    src: fvar_InstanceList,
) {
    fvar_InstanceList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<fvar_InstanceList>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_initCapN(mut arr: *mut fvar_InstanceList, mut n: size_t) {
    fvar_InstanceList_init(arr);
    fvar_InstanceList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_growToN(arr: *mut fvar_InstanceList, target: size_t) {
    cvec_grow_to_n(fvar_InstanceList_as_cvec(arr), target);
}
#[no_mangle]
pub static mut fvar_iInstanceList: __caryll_vectorinterface_fvar_InstanceList = {
    __caryll_vectorinterface_fvar_InstanceList {
        init: Some(fvar_InstanceList_init as unsafe extern "C" fn(*mut fvar_InstanceList) -> ()),
        copy: Some(
            fvar_InstanceList_copy
                as unsafe extern "C" fn(*mut fvar_InstanceList, *const fvar_InstanceList) -> (),
        ),
        move_0: Some(
            fvar_InstanceList_move
                as unsafe extern "C" fn(*mut fvar_InstanceList, *mut fvar_InstanceList) -> (),
        ),
        dispose: Some(
            fvar_InstanceList_dispose as unsafe extern "C" fn(*mut fvar_InstanceList) -> (),
        ),
        replace: Some(
            fvar_InstanceList_replace
                as unsafe extern "C" fn(*mut fvar_InstanceList, fvar_InstanceList) -> (),
        ),
        copyReplace: Some(
            fvar_InstanceList_copyReplace
                as unsafe extern "C" fn(*mut fvar_InstanceList, fvar_InstanceList) -> (),
        ),
        create: Some(fvar_InstanceList_create),
        free: Some(fvar_InstanceList_free as unsafe extern "C" fn(*mut fvar_InstanceList) -> ()),
        initN: Some(
            fvar_InstanceList_initN as unsafe extern "C" fn(*mut fvar_InstanceList, size_t) -> (),
        ),
        initCapN: Some(
            fvar_InstanceList_initCapN
                as unsafe extern "C" fn(*mut fvar_InstanceList, size_t) -> (),
        ),
        createN: Some(
            fvar_InstanceList_createN as unsafe extern "C" fn(size_t) -> *mut fvar_InstanceList,
        ),
        fill: Some(
            fvar_InstanceList_fill as unsafe extern "C" fn(*mut fvar_InstanceList, size_t) -> (),
        ),
        clear: Some(
            fvar_InstanceList_dispose as unsafe extern "C" fn(*mut fvar_InstanceList) -> (),
        ),
        push: Some(
            fvar_InstanceList_push
                as unsafe extern "C" fn(*mut fvar_InstanceList, fvar_Instance) -> (),
        ),
        shrinkToFit: Some(
            fvar_InstanceList_shrinkToFit as unsafe extern "C" fn(*mut fvar_InstanceList) -> (),
        ),
        pop: Some(
            fvar_InstanceList_pop as unsafe extern "C" fn(*mut fvar_InstanceList) -> fvar_Instance,
        ),
        disposeItem: Some(
            fvar_InstanceList_disposeItem
                as unsafe extern "C" fn(*mut fvar_InstanceList, size_t) -> (),
        ),
        filterEnv: Some(
            fvar_InstanceList_filterEnv
                as unsafe extern "C" fn(
                    *mut fvar_InstanceList,
                    Option<
                        unsafe extern "C" fn(
                            *const fvar_Instance,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            fvar_InstanceList_sort
                as unsafe extern "C" fn(
                    *mut fvar_InstanceList,
                    Option<
                        unsafe extern "C" fn(
                            *const fvar_Instance,
                            *const fvar_Instance,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn fvar_InstanceList_initN(mut arr: *mut fvar_InstanceList, mut n: size_t) {
    fvar_InstanceList_init(arr);
    fvar_InstanceList_growToN(arr, n);
    fvar_InstanceList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn fvar_InstanceList_shrinkToFit(mut arr: *mut fvar_InstanceList) {
    fvar_InstanceList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn disposeFvarMaster(mut m: *mut fvar_Master) {
    sdsfree((*m).name);
    vq_deleteRegion((*m).region);
}
#[inline]
unsafe extern "C" fn initFvar(mut fvar: *mut table_fvar) {
    memset(
        fvar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_fvar>() as size_t,
    );
    vf_iAxes.init.expect("non-null function pointer")(&raw mut (*fvar).axes);
    fvar_iInstanceList.init.expect("non-null function pointer")(&raw mut (*fvar).instances);
}
#[inline]
unsafe extern "C" fn disposeFvar(mut fvar: *mut table_fvar) {
    vf_iAxes.dispose.expect("non-null function pointer")(&raw mut (*fvar).axes);
    fvar_iInstanceList
        .dispose
        .expect("non-null function pointer")(&raw mut (*fvar).instances);
    let mut current: *mut fvar_Master = ::core::ptr::null_mut::<fvar_Master>();
    let mut tmp: *mut fvar_Master = ::core::ptr::null_mut::<fvar_Master>();
    current = (*fvar).masters;
    tmp = (if !(*fvar).masters.is_null() {
        (*(*fvar).masters).hh.next
    } else {
        NULL
    }) as *mut fvar_Master as *mut fvar_Master;
    while !current.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*current).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*fvar).masters).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*fvar).masters).hh.tbl as *mut ::core::ffi::c_void);
            (*fvar).masters = ::core::ptr::null_mut::<fvar_Master>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*fvar).masters).hh.tbl).tail {
                (*(*(*fvar).masters).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*fvar).masters).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*fvar).masters).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh2 = (*_hd_hh_del).next;
            } else {
                (*fvar).masters = (*_hd_hh_del).next as *mut fvar_Master as *mut fvar_Master;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*fvar).masters).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh3 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*fvar).masters).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = (*(*(*fvar).masters).hh.tbl)
                .buckets
                .offset(_hd_bkt as isize)
                as *mut UT_hash_bucket;
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
            (*(*(*fvar).masters).hh.tbl).num_items =
                (*(*(*fvar).masters).hh.tbl).num_items.wrapping_sub(1);
        }
        disposeFvarMaster(current);
        free(current as *mut ::core::ffi::c_void);
        current = ::core::ptr::null_mut::<fvar_Master>();
        current = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut fvar_Master
            as *mut fvar_Master;
    }
}
unsafe extern "C" fn fvar_registerRegion(
    mut fvar: *mut table_fvar,
    mut region: *mut vq_Region,
) -> *const vq_Region {
    let mut m: *mut fvar_Master = ::core::ptr::null_mut::<fvar_Master>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = region as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::core::mem::size_of::<vq_Region>().wrapping_add(
        ::core::mem::size_of::<vq_AxisSpan>()
            .wrapping_mul((*region).dimensions as usize),
    ) as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(
        ::core::mem::size_of::<vq_Region>().wrapping_add(
            ::core::mem::size_of::<vq_AxisSpan>()
                .wrapping_mul((*region).dimensions as usize),
        ) as ::core::ffi::c_uint,
    );
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 11098432890987736715;
        }
        10 => {
            current_block_50 = 11098432890987736715;
        }
        9 => {
            current_block_50 = 7788850179560822105;
        }
        8 => {
            current_block_50 = 2013626843157172960;
        }
        7 => {
            current_block_50 = 7680992524440278500;
        }
        6 => {
            current_block_50 = 14601631620111087220;
        }
        5 => {
            current_block_50 = 11029710244996856751;
        }
        4 => {
            current_block_50 = 16753638405504927854;
        }
        3 => {
            current_block_50 = 13847968192452473061;
        }
        2 => {
            current_block_50 = 13091112611283870258;
        }
        1 => {
            current_block_50 = 18027894311151487420;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        11098432890987736715 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 7788850179560822105;
        }
        _ => {}
    }
    match current_block_50 {
        7788850179560822105 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 2013626843157172960;
        }
        _ => {}
    }
    match current_block_50 {
        2013626843157172960 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 7680992524440278500;
        }
        _ => {}
    }
    match current_block_50 {
        7680992524440278500 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 14601631620111087220;
        }
        _ => {}
    }
    match current_block_50 {
        14601631620111087220 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 11029710244996856751;
        }
        _ => {}
    }
    match current_block_50 {
        11029710244996856751 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 16753638405504927854;
        }
        _ => {}
    }
    match current_block_50 {
        16753638405504927854 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 13847968192452473061;
        }
        _ => {}
    }
    match current_block_50 {
        13847968192452473061 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 13091112611283870258;
        }
        _ => {}
    }
    match current_block_50 {
        13091112611283870258 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 18027894311151487420;
        }
        _ => {}
    }
    match current_block_50 {
        18027894311151487420 => {
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
    m = ::core::ptr::null_mut::<fvar_Master>();
    if !(*fvar).masters.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*fvar).masters).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*fvar).masters).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                m = ((*(*(*(*fvar).masters).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*fvar).masters).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut fvar_Master
                    as *mut fvar_Master;
            } else {
                m = ::core::ptr::null_mut::<fvar_Master>();
            }
            while !m.is_null() {
                if (*m).hh.hashv == _hf_hashv
                    && (*m).hh.keylen as usize
                        == ::core::mem::size_of::<vq_Region>().wrapping_add(
                            ::core::mem::size_of::<vq_AxisSpan>()
                                .wrapping_mul((*region).dimensions as usize),
                        )
                {
                    if memcmp(
                        (*m).hh.key,
                        region as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<vq_Region>() as size_t).wrapping_add(
                            (::core::mem::size_of::<vq_AxisSpan>() as size_t)
                                .wrapping_mul((*region).dimensions as size_t),
                        ),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*m).hh.hh_next.is_null() {
                    m = ((*m).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*fvar).masters).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut fvar_Master
                        as *mut fvar_Master;
                } else {
                    m = ::core::ptr::null_mut::<fvar_Master>();
                }
            }
        }
    }
    if !m.is_null() {
        vq_deleteRegion(region);
        return (*m).region;
    } else {
        m = __caryll_allocate_clean(
            ::core::mem::size_of::<fvar_Master>() as size_t,
            47 as ::core::ffi::c_ulong,
        ) as *mut fvar_Master;
        let mut sMasterID: sds = sdsfromlonglong((1 as ::core::ffi::c_uint).wrapping_add(
            if !(*fvar).masters.is_null() {
                (*(*(*fvar).masters).hh.tbl).num_items
            } else {
                0 as ::core::ffi::c_uint
            },
        ) as ::core::ffi::c_longlong);
        (*m).name = sdscatsds(
            sdsnew(b"m\0" as *const u8 as *const ::core::ffi::c_char),
            sMasterID,
        );
        sdsfree(sMasterID);
        (*m).region = region;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar = (*m).region as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = ::core::mem::size_of::<vq_Region>().wrapping_add(
            ::core::mem::size_of::<vq_AxisSpan>()
                .wrapping_mul((*region).dimensions as usize),
        ) as ::core::ffi::c_uint;
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
        _ha_hashv = _ha_hashv.wrapping_add(
            ::core::mem::size_of::<vq_Region>().wrapping_add(
                ::core::mem::size_of::<vq_AxisSpan>()
                    .wrapping_mul((*region).dimensions as usize),
            ) as ::core::ffi::c_uint,
        );
        let mut current_block_171: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_171 = 6827241531168806533;
            }
            10 => {
                current_block_171 = 6827241531168806533;
            }
            9 => {
                current_block_171 = 7490234768345424691;
            }
            8 => {
                current_block_171 = 2571479547849027551;
            }
            7 => {
                current_block_171 = 5065576992453236399;
            }
            6 => {
                current_block_171 = 2708817167913782276;
            }
            5 => {
                current_block_171 = 9658771359317796075;
            }
            4 => {
                current_block_171 = 16102792977521885693;
            }
            3 => {
                current_block_171 = 6851027814222055606;
            }
            2 => {
                current_block_171 = 7597280631034036803;
            }
            1 => {
                current_block_171 = 5043988931478781221;
            }
            _ => {
                current_block_171 = 9587810615301548814;
            }
        }
        match current_block_171 {
            6827241531168806533 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_171 = 7490234768345424691;
            }
            _ => {}
        }
        match current_block_171 {
            7490234768345424691 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_171 = 2571479547849027551;
            }
            _ => {}
        }
        match current_block_171 {
            2571479547849027551 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_171 = 5065576992453236399;
            }
            _ => {}
        }
        match current_block_171 {
            5065576992453236399 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_171 = 2708817167913782276;
            }
            _ => {}
        }
        match current_block_171 {
            2708817167913782276 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_171 = 9658771359317796075;
            }
            _ => {}
        }
        match current_block_171 {
            9658771359317796075 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_171 = 16102792977521885693;
            }
            _ => {}
        }
        match current_block_171 {
            16102792977521885693 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_171 = 6851027814222055606;
            }
            _ => {}
        }
        match current_block_171 {
            6851027814222055606 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_171 = 7597280631034036803;
            }
            _ => {}
        }
        match current_block_171 {
            7597280631034036803 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_171 = 5043988931478781221;
            }
            _ => {}
        }
        match current_block_171 {
            5043988931478781221 => {
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
        (*m).hh.hashv = _ha_hashv;
        (*m).hh.key = (*m).region as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*m).hh.keylen = ::core::mem::size_of::<vq_Region>().wrapping_add(
            ::core::mem::size_of::<vq_AxisSpan>()
                .wrapping_mul((*region).dimensions as usize),
        ) as ::core::ffi::c_uint;
        if (*fvar).masters.is_null() {
            (*m).hh.next = NULL;
            (*m).hh.prev = NULL;
            (*m).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*m).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*m).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as size_t,
                );
                (*(*m).hh.tbl).tail = &raw mut (*m).hh as *mut UT_hash_handle;
                (*(*m).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*m).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*m).hh.tbl).hho = (&raw mut (*m).hh as *mut ::core::ffi::c_char)
                    .offset_from(m as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as ptrdiff_t;
                (*(*m).hh.tbl).buckets = malloc(
                    (32 as size_t).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                ) as *mut UT_hash_bucket;
                (*(*m).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                if (*(*m).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*m).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as size_t)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                    );
                }
            }
            (*fvar).masters = m;
        } else {
            (*m).hh.tbl = (*(*fvar).masters).hh.tbl;
            (*m).hh.next = NULL;
            (*m).hh.prev = ((*(*(*fvar).masters).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*fvar).masters).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*fvar).masters).hh.tbl).tail).next = m as *mut ::core::ffi::c_void;
            (*(*(*fvar).masters).hh.tbl).tail = &raw mut (*m).hh as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*fvar).masters).hh.tbl).num_items =
            (*(*(*fvar).masters).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*fvar).masters).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = (*(*(*fvar).masters).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*m).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*m).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*m).hh as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*m).hh as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*m).hh.tbl).noexpand == 0
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
                    .wrapping_mul((*(*m).hh.tbl).num_buckets as size_t)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as size_t)
                        .wrapping_mul((*(*m).hh.tbl).num_buckets as size_t)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                );
                (*(*m).hh.tbl).ideal_chain_maxlen = ((*(*m).hh.tbl).num_items
                    >> (*(*m).hh.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*m).hh.tbl).num_items
                        & (*(*m).hh.tbl)
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
                (*(*m).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*m).hh.tbl).num_buckets {
                    _he_thh = (*(*(*m).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*m).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*m).hh.tbl).ideal_chain_maxlen {
                            (*(*m).hh.tbl).nonideal_items =
                                (*(*m).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*m).hh.tbl).ideal_chain_maxlen);
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
                free((*(*m).hh.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*m).hh.tbl).num_buckets = (*(*m).hh.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*m).hh.tbl).log2_num_buckets = (*(*m).hh.tbl).log2_num_buckets.wrapping_add(1);
                (*(*m).hh.tbl).buckets = _he_new_buckets;
                (*(*m).hh.tbl).ineff_expands = if (*(*m).hh.tbl).nonideal_items
                    > (*(*m).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*m).hh.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*m).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*m).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        return (*m).region;
    };
}
unsafe extern "C" fn fvar_findMasterByRegion(
    mut fvar: *const table_fvar,
    mut region: *const vq_Region,
) -> *const fvar_Master {
    let mut m: *mut fvar_Master = ::core::ptr::null_mut::<fvar_Master>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = region as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::core::mem::size_of::<vq_Region>().wrapping_add(
        ::core::mem::size_of::<vq_AxisSpan>()
            .wrapping_mul((*region).dimensions as usize),
    ) as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(
        ::core::mem::size_of::<vq_Region>().wrapping_add(
            ::core::mem::size_of::<vq_AxisSpan>()
                .wrapping_mul((*region).dimensions as usize),
        ) as ::core::ffi::c_uint,
    );
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 16983614438056130870;
        }
        10 => {
            current_block_50 = 16983614438056130870;
        }
        9 => {
            current_block_50 = 15525165297982684156;
        }
        8 => {
            current_block_50 = 17129624834029794688;
        }
        7 => {
            current_block_50 = 18376437513952032856;
        }
        6 => {
            current_block_50 = 6454216577031963914;
        }
        5 => {
            current_block_50 = 6870917165266285974;
        }
        4 => {
            current_block_50 = 26157140621613139;
        }
        3 => {
            current_block_50 = 7257937163290155083;
        }
        2 => {
            current_block_50 = 8009893845190326358;
        }
        1 => {
            current_block_50 = 11128669157540593563;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        16983614438056130870 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 15525165297982684156;
        }
        _ => {}
    }
    match current_block_50 {
        15525165297982684156 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 17129624834029794688;
        }
        _ => {}
    }
    match current_block_50 {
        17129624834029794688 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 18376437513952032856;
        }
        _ => {}
    }
    match current_block_50 {
        18376437513952032856 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 6454216577031963914;
        }
        _ => {}
    }
    match current_block_50 {
        6454216577031963914 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 6870917165266285974;
        }
        _ => {}
    }
    match current_block_50 {
        6870917165266285974 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 26157140621613139;
        }
        _ => {}
    }
    match current_block_50 {
        26157140621613139 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 7257937163290155083;
        }
        _ => {}
    }
    match current_block_50 {
        7257937163290155083 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 8009893845190326358;
        }
        _ => {}
    }
    match current_block_50 {
        8009893845190326358 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 11128669157540593563;
        }
        _ => {}
    }
    match current_block_50 {
        11128669157540593563 => {
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
    m = ::core::ptr::null_mut::<fvar_Master>();
    if !(*fvar).masters.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*fvar).masters).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*fvar).masters).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                m = ((*(*(*(*fvar).masters).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*fvar).masters).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut fvar_Master
                    as *mut fvar_Master;
            } else {
                m = ::core::ptr::null_mut::<fvar_Master>();
            }
            while !m.is_null() {
                if (*m).hh.hashv == _hf_hashv
                    && (*m).hh.keylen as usize
                        == ::core::mem::size_of::<vq_Region>().wrapping_add(
                            ::core::mem::size_of::<vq_AxisSpan>()
                                .wrapping_mul((*region).dimensions as usize),
                        )
                {
                    if memcmp(
                        (*m).hh.key,
                        region as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<vq_Region>() as size_t).wrapping_add(
                            (::core::mem::size_of::<vq_AxisSpan>() as size_t)
                                .wrapping_mul((*region).dimensions as size_t),
                        ),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*m).hh.hh_next.is_null() {
                    m = ((*m).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*fvar).masters).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut fvar_Master
                        as *mut fvar_Master;
                } else {
                    m = ::core::ptr::null_mut::<fvar_Master>();
                }
            }
        }
    }
    return m;
}
#[inline]
unsafe extern "C" fn table_fvar_free(mut x: *mut table_fvar) {
    if x.is_null() {
        return;
    }
    table_fvar_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_fvar_dispose(mut x: *mut table_fvar) {
    disposeFvar(x);
}
#[inline]
unsafe extern "C" fn table_fvar_init(mut x: *mut table_fvar) {
    initFvar(x);
}
#[inline]
unsafe extern "C" fn table_fvar_create() -> *mut table_fvar {
    let mut x: *mut table_fvar =
        malloc(::core::mem::size_of::<table_fvar>() as size_t) as *mut table_fvar;
    table_fvar_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_fvar_copyReplace(mut dst: *mut table_fvar, src: table_fvar) {
    table_fvar_dispose(dst);
    table_fvar_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_fvar_copy(mut dst: *mut table_fvar, mut src: *const table_fvar) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_fvar>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_fvar_replace(mut dst: *mut table_fvar, src: table_fvar) {
    table_fvar_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_fvar>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_fvar_move(mut dst: *mut table_fvar, mut src: *mut table_fvar) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_fvar>() as size_t,
    );
    table_fvar_init(src);
}
#[no_mangle]
pub static mut table_iFvar: __caryll_elementinterface_table_fvar = {
    __caryll_elementinterface_table_fvar {
        init: Some(table_fvar_init as unsafe extern "C" fn(*mut table_fvar) -> ()),
        copy: Some(
            table_fvar_copy as unsafe extern "C" fn(*mut table_fvar, *const table_fvar) -> (),
        ),
        move_0: Some(
            table_fvar_move as unsafe extern "C" fn(*mut table_fvar, *mut table_fvar) -> (),
        ),
        dispose: Some(table_fvar_dispose as unsafe extern "C" fn(*mut table_fvar) -> ()),
        replace: Some(
            table_fvar_replace as unsafe extern "C" fn(*mut table_fvar, table_fvar) -> (),
        ),
        copyReplace: Some(
            table_fvar_copyReplace as unsafe extern "C" fn(*mut table_fvar, table_fvar) -> (),
        ),
        create: Some(table_fvar_create),
        free: Some(table_fvar_free as unsafe extern "C" fn(*mut table_fvar) -> ()),
        registerRegion: Some(
            fvar_registerRegion
                as unsafe extern "C" fn(*mut table_fvar, *mut vq_Region) -> *const vq_Region,
        ),
        findMasterByRegion: Some(
            fvar_findMasterByRegion
                as unsafe extern "C" fn(*const table_fvar, *const vq_Region) -> *const fvar_Master,
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn otfcc_readFvar(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_fvar {
    let mut header: *mut FVARHeader = ::core::ptr::null_mut::<FVARHeader>();
    let mut nAxes: uint16_t = 0;
    let mut instanceSizeWithoutPSNID: uint16_t = 0;
    let mut instanceSizeWithPSNID: uint16_t = 0;
    let mut axisRecord: *mut VariationAxisRecord = ::core::ptr::null_mut::<VariationAxisRecord>();
    let mut nInstances: uint16_t = 0;
    let mut hasPostscriptNameID: bool = false;
    let mut instance: *mut InstanceRecord = ::core::ptr::null_mut::<InstanceRecord>();
    let mut fvar: *mut table_fvar = ::core::ptr::null_mut::<table_fvar>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1719034226i32 as uint32_t {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    if !((table.length as usize) < ::core::mem::size_of::<FVARHeader>()) {
                        header = data as *mut FVARHeader;
                        if !(be16((*header).majorVersion) as ::core::ffi::c_int
                            != 1 as ::core::ffi::c_int)
                        {
                            if !(be16((*header).minorVersion) as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int)
                            {
                                if !(be16((*header).axesArrayOffset) as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int)
                                {
                                    if !(be16((*header).axisCount) as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int)
                                    {
                                        if !(be16((*header).axisSize) as usize
                                            != ::core::mem::size_of::<VariationAxisRecord>())
                                        {
                                            nAxes = be16((*header).axisCount);
                                            instanceSizeWithoutPSNID = 4_usize.wrapping_add(
                                                (nAxes as usize).wrapping_mul(
                                                    ::core::mem::size_of::<f16dot16>(),
                                                ),
                                            )
                                                as uint16_t;
                                            instanceSizeWithPSNID = (2 as ::core::ffi::c_int
                                                + instanceSizeWithoutPSNID as ::core::ffi::c_int)
                                                as uint16_t;
                                            if !(be16((*header).instanceSize) as ::core::ffi::c_int
                                                != instanceSizeWithoutPSNID as ::core::ffi::c_int
                                                && be16((*header).instanceSize)
                                                    as ::core::ffi::c_int
                                                    != instanceSizeWithPSNID as ::core::ffi::c_int)
                                            {
                                                if !((table.length as usize)
                                                    < (be16((*header).axesArrayOffset) as usize)
                                                        .wrapping_add(
                                                            ::core::mem::size_of::<
                                                                VariationAxisRecord,
                                                            >(
                                                            )
                                                                .wrapping_mul(nAxes as usize),
                                                        )
                                                        .wrapping_add(
                                                            (be16((*header).instanceSize)
                                                                as ::core::ffi::c_int
                                                                * be16((*header).instanceCount)
                                                                    as ::core::ffi::c_int)
                                                                as usize,
                                                        ))
                                                {
                                                    fvar = ::core::mem::transmute::<
                                                        _,
                                                        fn() -> *mut table_fvar,
                                                    >(
                                                        table_iFvar
                                                            .create
                                                            .expect("non-null function pointer"),
                                                    )(
                                                    );
                                                    axisRecord =
                                                        data.offset(be16((*header).axesArrayOffset)
                                                            as ::core::ffi::c_int
                                                            as isize)
                                                            as *mut VariationAxisRecord;
                                                    let mut j: uint16_t = 0 as uint16_t;
                                                    while (j as ::core::ffi::c_int)
                                                        < nAxes as ::core::ffi::c_int
                                                    {
                                                        let mut axis: vf_Axis = vf_Axis {
                                                            tag: be32((*axisRecord).axisTag),
                                                            minValue: otfcc_from_fixed(be32(
                                                                (*axisRecord).minValue as uint32_t,
                                                            )
                                                                as f16dot16)
                                                                as pos_t,
                                                            defaultValue: otfcc_from_fixed(be32(
                                                                (*axisRecord).defaultValue
                                                                    as uint32_t,
                                                            )
                                                                as f16dot16)
                                                                as pos_t,
                                                            maxValue: otfcc_from_fixed(be32(
                                                                (*axisRecord).maxValue as uint32_t,
                                                            )
                                                                as f16dot16)
                                                                as pos_t,
                                                            flags: be16((*axisRecord).flags),
                                                            axisNameID: be16(
                                                                (*axisRecord).axisNameID,
                                                            ),
                                                        };
                                                        vf_iAxes
                                                            .push
                                                            .expect("non-null function pointer")(
                                                            &raw mut (*fvar).axes,
                                                            axis,
                                                        );
                                                        axisRecord = axisRecord.offset(1);
                                                        j = j.wrapping_add(1);
                                                    }
                                                    nInstances = be16((*header).instanceCount);
                                                    hasPostscriptNameID =
                                                        be16((*header).instanceSize)
                                                            as ::core::ffi::c_int
                                                            == instanceSizeWithPSNID
                                                                as ::core::ffi::c_int;
                                                    instance = axisRecord as *mut InstanceRecord;
                                                    let mut j_0: uint16_t = 0 as uint16_t;
                                                    while (j_0 as ::core::ffi::c_int)
                                                        < nInstances as ::core::ffi::c_int
                                                    {
                                                        let mut inst: fvar_Instance =
                                                            fvar_Instance {
                                                                subfamilyNameID: 0,
                                                                flags: 0,
                                                                coordinates: VV {
                                                                    length: 0,
                                                                    capacity: 0,
                                                                    items: ::core::ptr::null_mut::<
                                                                        pos_t,
                                                                    >(
                                                                    ),
                                                                },
                                                                postScriptNameID: 0,
                                                            };
                                                        fvar_iInstance
                                                            .init
                                                            .expect("non-null function pointer")(
                                                            &raw mut inst,
                                                        );
                                                        inst.subfamilyNameID =
                                                            be16((*instance).subfamilyNameID);
                                                        inst.flags = be16((*instance).flags);
                                                        let mut k: uint16_t = 0 as uint16_t;
                                                        while (k as ::core::ffi::c_int)
                                                            < nAxes as ::core::ffi::c_int
                                                        {
                                                            iVV.push.expect(
                                                                "non-null function pointer",
                                                            )(
                                                                &raw mut inst.coordinates,
                                                                otfcc_from_fixed(be32(
                                                                    *(&raw mut (*instance)
                                                                        .coordinates
                                                                        as *mut f16dot16)
                                                                        .offset(k as isize)
                                                                        as uint32_t,
                                                                )
                                                                    as f16dot16)
                                                                    as pos_t,
                                                            );
                                                            k = k.wrapping_add(1);
                                                        }
                                                        iVV.shrinkToFit
                                                            .expect("non-null function pointer")(
                                                            &raw mut inst.coordinates,
                                                        );
                                                        if hasPostscriptNameID {
                                                            inst.postScriptNameID = be16(
                                                                *((instance as font_file_pointer)
                                                                    .offset(
                                                                        instanceSizeWithoutPSNID
                                                                            as ::core::ffi::c_int
                                                                            as isize,
                                                                    )
                                                                    as *mut uint16_t),
                                                            );
                                                        }
                                                        fvar_iInstanceList
                                                            .push
                                                            .expect("non-null function pointer")(
                                                            &raw mut (*fvar).instances,
                                                            inst,
                                                        );
                                                        instance = (instance as font_file_pointer)
                                                            .offset(be16((*header).instanceSize)
                                                                as ::core::ffi::c_int
                                                                as isize)
                                                            as *mut InstanceRecord;
                                                        j_0 = j_0.wrapping_add(1);
                                                    }
                                                    vf_iAxes
                                                        .shrinkToFit
                                                        .expect("non-null function pointer")(
                                                        &raw mut (*fvar).axes,
                                                    );
                                                    fvar_iInstanceList
                                                        .shrinkToFit
                                                        .expect("non-null function pointer")(
                                                        &raw mut (*fvar).instances,
                                                    );
                                                    return fvar;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as uint8_t,
                        log_type_warning,
                        sdscatprintf(
                            sdsempty(),
                            b"table 'fvar' corrupted.\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        ),
                    );
                    table_iFvar.free.expect("non-null function pointer")(fvar);
                    fvar = ::core::ptr::null_mut::<table_fvar>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_fvar>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpFvar(
    mut table: *const table_fvar,
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
            b"fvar\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut t: *mut json_value = json_object_new(2 as size_t);
        let mut _axes: *mut json_value = json_object_new((*table).axes.length);
        let mut __caryll_index: size_t = 0 as size_t;
        let mut keep: size_t = 1 as size_t;
        while keep != 0 && __caryll_index < (*table).axes.length {
            let mut axis: *mut vf_Axis = (*table).axes.items.offset(__caryll_index as isize);
            while keep != 0 {
                let mut _axis: *mut json_value = json_object_new(5 as size_t);
                json_object_push(
                    _axis,
                    b"minValue\0" as *const u8 as *const ::core::ffi::c_char,
                    json_double_new((*axis).minValue as ::core::ffi::c_double),
                );
                json_object_push(
                    _axis,
                    b"defaultValue\0" as *const u8 as *const ::core::ffi::c_char,
                    json_double_new((*axis).defaultValue as ::core::ffi::c_double),
                );
                json_object_push(
                    _axis,
                    b"maxValue\0" as *const u8 as *const ::core::ffi::c_char,
                    json_double_new((*axis).maxValue as ::core::ffi::c_double),
                );
                json_object_push(
                    _axis,
                    b"flags\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*axis).flags as int64_t),
                );
                json_object_push(
                    _axis,
                    b"axisNameID\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*axis).axisNameID as int64_t),
                );
                json_object_push_tag(_axes, (*axis).tag, _axis);
                keep = (keep == 0) as ::core::ffi::c_int as size_t;
            }
            keep = (keep == 0) as ::core::ffi::c_int as size_t;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            t,
            b"axes\0" as *const u8 as *const ::core::ffi::c_char,
            _axes,
        );
        let mut _instances: *mut json_value = json_array_new((*table).instances.length);
        let mut __caryll_index_0: size_t = 0 as size_t;
        let mut keep_0: size_t = 1 as size_t;
        while keep_0 != 0 && __caryll_index_0 < (*table).instances.length {
            let mut instance: *mut fvar_Instance =
                (*table).instances.items.offset(__caryll_index_0 as isize);
            while keep_0 != 0 {
                let mut _instance: *mut json_value = json_object_new(4 as size_t);
                json_object_push(
                    _instance,
                    b"subfamilyNameID\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*instance).subfamilyNameID as int64_t),
                );
                if (*instance).postScriptNameID != 0 {
                    json_object_push(
                        _instance,
                        b"postScriptNameID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer_new((*instance).postScriptNameID as int64_t),
                    );
                }
                json_object_push(
                    _instance,
                    b"flags\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*instance).flags as int64_t),
                );
                json_object_push(
                    _instance,
                    b"coordinates\0" as *const u8 as *const ::core::ffi::c_char,
                    json_new_VVp(&raw mut (*instance).coordinates, table),
                );
                json_array_push(_instances, _instance);
                keep_0 = (keep_0 == 0) as ::core::ffi::c_int as size_t;
            }
            keep_0 = (keep_0 == 0) as ::core::ffi::c_int as size_t;
            __caryll_index_0 = __caryll_index_0.wrapping_add(1);
        }
        json_object_push(
            t,
            b"instances\0" as *const u8 as *const ::core::ffi::c_char,
            _instances,
        );
        let mut _masters: *mut json_value = json_object_new(
            (if !(*table).masters.is_null() {
                (*(*(*table).masters).hh.tbl).num_items
            } else {
                0 as ::core::ffi::c_uint
            }) as size_t,
        );
        let mut current: *mut fvar_Master = ::core::ptr::null_mut::<fvar_Master>();
        let mut tmp: *mut fvar_Master = ::core::ptr::null_mut::<fvar_Master>();
        current = (*table).masters;
        tmp = (if !(*table).masters.is_null() {
            (*(*table).masters).hh.next
        } else {
            NULL
        }) as *mut fvar_Master as *mut fvar_Master;
        while !current.is_null() {
            json_object_push(
                _masters,
                (*current).name as *const ::core::ffi::c_char,
                preserialize(json_new_VQRegion_Explicit((*current).region, table)),
            );
            current = tmp;
            tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut fvar_Master
                as *mut fvar_Master;
        }
        json_object_push(
            t,
            b"masters\0" as *const u8 as *const ::core::ffi::c_char,
            _masters,
        );
        json_object_push(
            root,
            b"fvar\0" as *const u8 as *const ::core::ffi::c_char,
            t,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn json_new_VQSegment(
    mut s: *const vq_Segment,
    mut fvar: *const table_fvar,
) -> *mut json_value {
    let mut d: *mut json_value = ::core::ptr::null_mut::<json_value>();
    match (*s).type_0 as ::core::ffi::c_uint {
        0 => return json_new_position((*s).val.still),
        1 => {
            d = json_object_new(3 as size_t);
            json_object_push(
                d,
                b"delta\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_position((*s).val.delta.quantity),
            );
            if !(*s).val.delta.touched {
                json_object_push(
                    d,
                    b"implicit\0" as *const u8 as *const ::core::ffi::c_char,
                    json_boolean_new(!(*s).val.delta.touched as ::core::ffi::c_int),
                );
            }
            json_object_push(
                d,
                b"on\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_VQRegion((*s).val.delta.region, fvar),
            );
            return d;
        }
        _ => return json_integer_new(0 as int64_t),
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_new_VQ(z: VQ, mut fvar: *const table_fvar) -> *mut json_value {
    if z.shift.length == 0 {
        return preserialize(json_new_position(iVQ
            .getStill
            .expect("non-null function pointer")(
            z
        )));
    } else {
        let mut a: *mut json_value = json_array_new(z.shift.length.wrapping_add(1 as size_t));
        json_array_push(a, json_new_position(z.kernel));
        let mut j: size_t = 0 as size_t;
        while j < z.shift.length {
            json_array_push(
                a,
                json_new_VQSegment(z.shift.items.offset(j as isize) as *mut vq_Segment, fvar),
            );
            j = j.wrapping_add(1);
        }
        return preserialize(a);
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_new_VV(x: VV, mut fvar: *const table_fvar) -> *mut json_value {
    let mut axes: *const vf_Axes = &raw const (*fvar).axes;
    if !axes.is_null() && (*axes).length == x.length {
        let mut _coord: *mut json_value = json_object_new((*axes).length);
        let mut m: size_t = 0 as size_t;
        while m < x.length {
            let mut axis: *mut vf_Axis = (*axes).items.offset(m as isize) as *mut vf_Axis;
            let mut tag: [::core::ffi::c_char; 4] = [
                (((*axis).tag & 0xff000000 as uint32_t) >> 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                (((*axis).tag & 0xff0000 as uint32_t) >> 16 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                (((*axis).tag & 0xff00 as uint32_t) >> 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                ((*axis).tag & 0xff as uint32_t) as ::core::ffi::c_char,
            ];
            json_object_push_length(
                _coord,
                4 as ::core::ffi::c_uint,
                &raw mut tag as *mut ::core::ffi::c_char,
                json_new_position(*x.items.offset(m as isize)),
            );
            m = m.wrapping_add(1);
        }
        return preserialize(_coord);
    } else {
        let mut _coord_0: *mut json_value = json_array_new(x.length);
        let mut m_0: size_t = 0 as size_t;
        while m_0 < x.length {
            json_array_push(_coord_0, json_new_position(*x.items.offset(m_0 as isize)));
            m_0 = m_0.wrapping_add(1);
        }
        return preserialize(_coord_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_new_VVp(
    mut x: *const VV,
    mut fvar: *const table_fvar,
) -> *mut json_value {
    let mut axes: *const vf_Axes = &raw const (*fvar).axes;
    if !axes.is_null() && (*axes).length == (*x).length {
        let mut _coord: *mut json_value = json_object_new((*axes).length);
        let mut m: size_t = 0 as size_t;
        while m < (*x).length {
            let mut axis: *mut vf_Axis = (*axes).items.offset(m as isize) as *mut vf_Axis;
            let mut tag: [::core::ffi::c_char; 4] = [
                (((*axis).tag & 0xff000000 as uint32_t) >> 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                (((*axis).tag & 0xff0000 as uint32_t) >> 16 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                (((*axis).tag & 0xff00 as uint32_t) >> 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                ((*axis).tag & 0xff as uint32_t) as ::core::ffi::c_char,
            ];
            json_object_push_length(
                _coord,
                4 as ::core::ffi::c_uint,
                &raw mut tag as *mut ::core::ffi::c_char,
                json_new_position(*(*x).items.offset(m as isize)),
            );
            m = m.wrapping_add(1);
        }
        return preserialize(_coord);
    } else {
        let mut _coord_0: *mut json_value = json_array_new((*x).length);
        let mut m_0: size_t = 0 as size_t;
        while m_0 < (*x).length {
            json_array_push(
                _coord_0,
                json_new_position(*(*x).items.offset(m_0 as isize)),
            );
            m_0 = m_0.wrapping_add(1);
        }
        return preserialize(_coord_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_vqOf(mut cv: *const json_value, mut _fvar: *const table_fvar) -> VQ {
    return iVQ.createStill.expect("non-null function pointer")(json_numof(cv) as pos_t);
}
#[no_mangle]
pub unsafe extern "C" fn json_new_VQAxisSpan(mut s: *const vq_AxisSpan) -> *mut json_value {
    if vq_AxisSpanIsOne(s) {
        return json_string_new(b"*\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        let mut a: *mut json_value = json_object_new(3 as size_t);
        json_object_push(
            a,
            b"start\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*s).start),
        );
        json_object_push(
            a,
            b"peak\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*s).peak),
        );
        json_object_push(
            a,
            b"end\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*s).end),
        );
        return a;
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_new_VQRegion_Explicit(
    mut rs: *const vq_Region,
    mut fvar: *const table_fvar,
) -> *mut json_value {
    let mut axes: *const vf_Axes = &raw const (*fvar).axes;
    if !axes.is_null() && (*axes).length == (*rs).dimensions as size_t {
        let mut r: *mut json_value = json_object_new((*rs).dimensions as size_t);
        let mut j: size_t = 0 as size_t;
        while j < (*rs).dimensions as size_t {
            json_object_push_tag(
                r,
                (*(*axes).items.offset(j as isize)).tag,
                json_new_VQAxisSpan(
                    (&raw const (*rs).spans as *const vq_AxisSpan).offset(j as isize)
                        as *const vq_AxisSpan,
                ),
            );
            j = j.wrapping_add(1);
        }
        return r;
    } else {
        let mut r_0: *mut json_value = json_array_new((*rs).dimensions as size_t);
        let mut j_0: size_t = 0 as size_t;
        while j_0 < (*rs).dimensions as size_t {
            json_array_push(
                r_0,
                json_new_VQAxisSpan(
                    (&raw const (*rs).spans as *const vq_AxisSpan).offset(j_0 as isize)
                        as *const vq_AxisSpan,
                ),
            );
            j_0 = j_0.wrapping_add(1);
        }
        return r_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_new_VQRegion(
    mut rs: *const vq_Region,
    mut fvar: *const table_fvar,
) -> *mut json_value {
    let mut m: *const fvar_Master = table_iFvar
        .findMasterByRegion
        .expect("non-null function pointer")(fvar, rs);
    if !m.is_null() && !(*m).name.is_null() {
        return json_string_new_length(
            sdslen((*m).name) as ::core::ffi::c_uint,
            (*m).name as *const ::core::ffi::c_char,
        );
    } else {
        return json_new_VQRegion_Explicit(rs, fvar);
    };
}
pub const json_serialize_mode_packed: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn be16(mut x: uint16_t) -> uint16_t {
    return ((x as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | (x as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int)
        as uint16_t;
}
#[inline]
unsafe extern "C" fn be32(mut x: uint32_t) -> uint32_t {
    return (x & 0xff as uint32_t) << 24 as ::core::ffi::c_int
        | (x & 0xff00 as uint32_t) << 8 as ::core::ffi::c_int
        | (x & 0xff0000 as uint32_t) >> 8 as ::core::ffi::c_int
        | (x & 0xff000000 as uint32_t) >> 24 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn json_object_push_tag(
    mut a: *mut json_value,
    mut tag: uint32_t,
    mut b: *mut json_value,
) -> *mut json_value {
    let mut tags: [::core::ffi::c_char; 4] = [
        ((tag & 0xff000000 as uint32_t) >> 24 as ::core::ffi::c_int) as ::core::ffi::c_char,
        ((tag & 0xff0000 as uint32_t) >> 16 as ::core::ffi::c_int) as ::core::ffi::c_char,
        ((tag & 0xff00 as uint32_t) >> 8 as ::core::ffi::c_int) as ::core::ffi::c_char,
        (tag & 0xff as uint32_t) as ::core::ffi::c_char,
    ];
    return json_object_push_length(
        a,
        4 as ::core::ffi::c_uint,
        &raw mut tags as *mut ::core::ffi::c_char,
        b,
    );
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
unsafe extern "C" fn json_new_position(mut z: pos_t) -> *mut json_value {
    if round(z as ::core::ffi::c_double) == z {
        return json_integer_new(z as int64_t);
    } else {
        return json_double_new(z as ::core::ffi::c_double);
    };
}
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
