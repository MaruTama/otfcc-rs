#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, raw_ref_op)]
#[allow(unused_imports)]
use ::otfcc_rust;
use otfcc_rust::table::otl::coverage::{otl_Coverage};
use otfcc_rust::support::stdio::{stdin, stdout, FILE};
extern "C" {
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> size_t;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn otfcc_readSFNT(file: *mut FILE) -> *mut otfcc_SplineFontContainer;
    fn otfcc_deleteSFNT(font: *mut otfcc_SplineFontContainer);
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static otfcc_iFont: __caryll_elementinterface_otfcc_Font;
    fn otfcc_newOTFReader() -> *mut otfcc_IFontBuilder;
    fn otfcc_newJsonWriter() -> *mut otfcc_IFontSerializer;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn time_now(tv: *mut timespec);
    fn push_stopwatch(sofar: *mut timespec) -> sds;
}
use otfcc_rust::support::handle::{otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle};
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub struct otfcc_SplineFontContainer {
    pub type_0: uint32_t,
    pub count: uint32_t,
    pub offsets: *mut uint32_t,
    pub packets: *mut otfcc_Packet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _caryll_font {
    pub subtype: otfcc_font_subtype,
    pub fvar: *mut table_fvar,
    pub head: *mut table_head,
    pub hhea: *mut table_hhea,
    pub maxp: *mut table_maxp,
    pub OS_2: *mut table_OS_2,
    pub hmtx: *mut table_hmtx,
    pub post: *mut table_post,
    pub hdmx: *mut table_hdmx,
    pub vhea: *mut table_vhea,
    pub vmtx: *mut table_vmtx,
    pub VORG: *mut table_VORG,
    pub CFF_: *mut table_CFF,
    pub glyf: *mut table_glyf,
    pub cmap: *mut table_cmap,
    pub name: *mut table_name,
    pub meta: *mut table_meta,
    pub fpgm: *mut table_fpgm_prep,
    pub prep: *mut table_fpgm_prep,
    pub cvt_: *mut table_cvt,
    pub gasp: *mut table_gasp,
    pub VDMX: *mut table_VDMX,
    pub LTSH: *mut table_LTSH,
    pub GSUB: *mut table_OTL,
    pub GPOS: *mut table_OTL,
    pub GDEF: *mut table_GDEF,
    pub BASE: *mut table_BASE,
    pub CPAL: *mut table_CPAL,
    pub COLR: *mut table_COLR,
    pub SVG_: *mut table_SVG,
    pub TSI_01: *mut table_TSI,
    pub TSI_23: *mut table_TSI,
    pub TSI5: *mut table_TSI5,
    pub glyph_order: *mut otfcc_GlyphOrder,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_GlyphOrder {
    pub byGID: *mut otfcc_GlyphOrderEntry,
    pub byName: *mut otfcc_GlyphOrderEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_GlyphOrderEntry {
    pub gid: glyphid_t,
    pub name: sds,
    pub orderType: uint8_t,
    pub orderEntry: uint32_t,
    pub hhID: UT_hash_handle,
    pub hhName: UT_hash_handle,
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
pub type ptrdiff_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: ::core::ffi::c_uint,
    pub expand_mult: ::core::ffi::c_uint,
}
pub type sds = *mut ::core::ffi::c_char;
pub type glyphid_t = uint16_t;
pub type otl_ClassDef = table_TSI5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_TSI5 {
    pub numGlyphs: glyphid_t,
    pub capacity: uint32_t,
    pub maxclass: glyphclass_t,
    pub glyphs: *mut otfcc_GlyphHandle,
    pub classes: *mut glyphclass_t,
}
pub type glyphclass_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_TSI {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut tsi_Entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tsi_Entry {
    pub type_0: tsi_EntryType,
    pub glyph: otfcc_GlyphHandle,
    pub content: sds,
}
pub type tsi_EntryType = ::core::ffi::c_uint;
pub const TSI_RESERVED_FFFC: tsi_EntryType = 4;
pub const TSI_CVT: tsi_EntryType = 3;
pub const TSI_PREP: tsi_EntryType = 2;
pub const TSI_FPGM: tsi_EntryType = 1;
pub const TSI_GLYPH: tsi_EntryType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_SVG {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut svg_Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svg_Assignment {
    pub start: glyphid_t,
    pub end: glyphid_t,
    pub document: *mut caryll_Buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_COLR {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut colr_Mapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colr_Mapping {
    pub glyph: otfcc_GlyphHandle,
    pub layers: colr_LayerList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colr_LayerList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut colr_Layer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colr_Layer {
    pub glyph: otfcc_GlyphHandle,
    pub paletteIndex: colorid_t,
}
pub type colorid_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_CPAL {
    pub version: uint16_t,
    pub palettes: cpal_PaletteSet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_PaletteSet {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut cpal_Palette,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_Palette {
    pub colorset: cpal_ColorSet,
    pub type_0: uint32_t,
    pub label: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_ColorSet {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut cpal_Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_Color {
    pub red: uint8_t,
    pub green: uint8_t,
    pub blue: uint8_t,
    pub alpha: uint8_t,
    pub label: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_BASE {
    pub horizontal: *mut otl_BaseAxis,
    pub vertical: *mut otl_BaseAxis,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseAxis {
    pub scriptCount: tableid_t,
    pub entries: *mut otl_BaseScriptEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseScriptEntry {
    pub tag: uint32_t,
    pub defaultBaselineTag: uint32_t,
    pub baseValuesCount: tableid_t,
    pub baseValues: *mut otl_BaseValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseValue {
    pub tag: uint32_t,
    pub coordinate: pos_t,
}
pub type pos_t = ::core::ffi::c_double;
pub type tableid_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_GDEF {
    pub glyphClassDef: *mut otl_ClassDef,
    pub markAttachClassDef: *mut otl_ClassDef,
    pub ligCarets: otl_LigCaretTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LigCaretTable {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_CaretValueRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_CaretValueRecord {
    pub glyph: otfcc_GlyphHandle,
    pub carets: otl_CaretValueList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_CaretValueList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_CaretValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_CaretValue {
    pub format: int8_t,
    pub coordiante: pos_t,
    pub pointIndex: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_OTL {
    pub lookups: otl_LookupList,
    pub features: otl_FeatureList,
    pub languages: otl_LangSystemList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LangSystemList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LanguageSystemPtr,
}
pub type otl_LanguageSystemPtr = *mut otl_LanguageSystem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LanguageSystem {
    pub name: sds,
    pub requiredFeature: otl_FeatureRef,
    pub features: otl_FeatureRefList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_FeatureRefList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_FeatureRef,
}
pub type otl_FeatureRef = *const otl_Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_Feature {
    pub name: sds,
    pub lookups: otl_LookupRefList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LookupRefList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LookupRef,
}
pub type otl_LookupRef = *const otl_Lookup;
pub type otl_Lookup = _otl_lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _otl_lookup {
    pub name: sds,
    pub type_0: otl_LookupType,
    pub _offset: uint32_t,
    pub flags: uint16_t,
    pub subtables: otl_SubtableList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_SubtableList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_SubtablePtr,
}
pub type otl_SubtablePtr = *mut otl_Subtable;
pub type otl_Subtable = _otl_subtable;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _otl_subtable {
    pub gsub_single: subtable_gsub_single,
    pub gsub_multi: subtable_gsub_multi,
    pub gsub_ligature: subtable_gsub_ligature,
    pub chaining: subtable_chaining,
    pub gsub_reverse: subtable_gsub_reverse,
    pub gpos_single: subtable_gpos_single,
    pub gpos_pair: subtable_gpos_pair,
    pub gpos_cursive: subtable_gpos_cursive,
    pub gpos_markToSingle: subtable_gpos_markToSingle,
    pub gpos_markToLigature: subtable_gpos_markToLigature,
    pub extend: subtable_extend,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_extend {
    pub type_0: otl_LookupType,
    pub subtable: *mut otl_Subtable,
}
pub type otl_LookupType = ::core::ffi::c_uint;
pub const otl_type_gpos_extend: otl_LookupType = 41;
pub const otl_type_gpos_chaining: otl_LookupType = 40;
pub const otl_type_gpos_context: otl_LookupType = 39;
pub const otl_type_gpos_markToMark: otl_LookupType = 38;
pub const otl_type_gpos_markToLigature: otl_LookupType = 37;
pub const otl_type_gpos_markToBase: otl_LookupType = 36;
pub const otl_type_gpos_cursive: otl_LookupType = 35;
pub const otl_type_gpos_pair: otl_LookupType = 34;
pub const otl_type_gpos_single: otl_LookupType = 33;
pub const otl_type_gpos_unknown: otl_LookupType = 32;
pub const otl_type_gsub_reverse: otl_LookupType = 24;
pub const otl_type_gsub_extend: otl_LookupType = 23;
pub const otl_type_gsub_chaining: otl_LookupType = 22;
pub const otl_type_gsub_context: otl_LookupType = 21;
pub const otl_type_gsub_ligature: otl_LookupType = 20;
pub const otl_type_gsub_alternate: otl_LookupType = 19;
pub const otl_type_gsub_multiple: otl_LookupType = 18;
pub const otl_type_gsub_single: otl_LookupType = 17;
pub const otl_type_gsub_unknown: otl_LookupType = 16;
pub const otl_type_unknown: otl_LookupType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_markToLigature {
    pub classCount: glyphclass_t,
    pub markArray: otl_MarkArray,
    pub ligArray: otl_LigatureArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LigatureArray {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LigatureBaseRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LigatureBaseRecord {
    pub glyph: otfcc_GlyphHandle,
    pub componentCount: glyphid_t,
    pub anchors: *mut *mut otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_Anchor {
    pub present: bool,
    pub x: pos_t,
    pub y: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_MarkArray {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_MarkRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_MarkRecord {
    pub glyph: otfcc_GlyphHandle,
    pub markClass: glyphclass_t,
    pub anchor: otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_markToSingle {
    pub classCount: glyphclass_t,
    pub markArray: otl_MarkArray,
    pub baseArray: otl_BaseArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseArray {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_BaseRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseRecord {
    pub glyph: otfcc_GlyphHandle,
    pub anchors: *mut otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_cursive {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GposCursiveEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GposCursiveEntry {
    pub target: otfcc_GlyphHandle,
    pub enter: otl_Anchor,
    pub exit: otl_Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_pair {
    pub first: *mut otl_ClassDef,
    pub second: *mut otl_ClassDef,
    pub firstValues: *mut *mut otl_PositionValue,
    pub secondValues: *mut *mut otl_PositionValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_PositionValue {
    pub dx: pos_t,
    pub dy: pos_t,
    pub dWidth: pos_t,
    pub dHeight: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gpos_single {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GposSingleEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GposSingleEntry {
    pub target: otfcc_GlyphHandle,
    pub value: otl_PositionValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_reverse {
    pub matchCount: tableid_t,
    pub inputIndex: tableid_t,
    pub match_0: *mut *mut otl_Coverage,
    pub to: *mut otl_Coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_chaining {
    pub type_0: otl_chaining_type,
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub rulesCount: tableid_t,
    pub rules: *mut *mut otl_ChainingRule,
    pub bc: *mut otl_ClassDef,
    pub ic: *mut otl_ClassDef,
    pub fc: *mut otl_ClassDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ChainingRule {
    pub matchCount: tableid_t,
    pub inputBegins: tableid_t,
    pub inputEnds: tableid_t,
    pub match_0: *mut *mut otl_Coverage,
    pub applyCount: tableid_t,
    pub apply: *mut otl_ChainLookupApplication,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ChainLookupApplication {
    pub index: tableid_t,
    pub lookup: otfcc_LookupHandle,
}
pub type otl_chaining_type = ::core::ffi::c_uint;
pub const otl_chaining_classified: otl_chaining_type = 2;
pub const otl_chaining_poly: otl_chaining_type = 1;
pub const otl_chaining_canonical: otl_chaining_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_ligature {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GsubLigatureEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GsubLigatureEntry {
    pub from: *mut otl_Coverage,
    pub to: otfcc_GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_multi {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GsubMultiEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GsubMultiEntry {
    pub from: otfcc_GlyphHandle,
    pub to: *mut otl_Coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtable_gsub_single {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_GsubSingleEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_GsubSingleEntry {
    pub from: otfcc_GlyphHandle,
    pub to: otfcc_GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_FeatureList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_FeaturePtr,
}
pub type otl_FeaturePtr = *mut otl_Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LookupList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LookupPtr,
}
pub type otl_LookupPtr = *mut otl_Lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_LTSH {
    pub version: uint16_t,
    pub numGlyphs: glyphid_t,
    pub yPels: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_VDMX {
    pub version: uint16_t,
    pub ratios: vdmx_RatioRagneList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_RatioRagneList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut vdmx_RatioRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_RatioRange {
    pub bCharset: uint8_t,
    pub xRatio: uint8_t,
    pub yStartRatio: uint8_t,
    pub yEndRatio: uint8_t,
    pub records: vdmx_Group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_Group {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut vdmx_Record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_Record {
    pub yPelHeight: uint16_t,
    pub yMax: int16_t,
    pub yMin: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_gasp {
    pub version: uint16_t,
    pub records: gasp_RecordList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gasp_RecordList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut gasp_Record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gasp_Record {
    pub rangeMaxPPEM: glyphsize_t,
    pub dogray: bool,
    pub gridfit: bool,
    pub symmetric_smoothing: bool,
    pub symmetric_gridfit: bool,
}
pub type glyphsize_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_cvt {
    pub length: uint32_t,
    pub words: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_fpgm_prep {
    pub tag: sds,
    pub length: uint32_t,
    pub bytes: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_meta {
    pub version: uint32_t,
    pub flags: uint32_t,
    pub entries: meta_Entries,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meta_Entries {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut meta_Entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meta_Entry {
    pub tag: uint32_t,
    pub data: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_name {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otfcc_NameRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_NameRecord {
    pub platformID: uint16_t,
    pub encodingID: uint16_t,
    pub languageID: uint16_t,
    pub nameID: uint16_t,
    pub nameString: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_cmap {
    pub unicodes: *mut cmap_Entry,
    pub uvs: *mut cmap_UVS_Entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap_UVS_Entry {
    pub hh: UT_hash_handle,
    pub key: cmap_UVS_key,
    pub glyph: otfcc_GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap_UVS_key {
    pub unicode: uint32_t,
    pub selector: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap_Entry {
    pub hh: UT_hash_handle,
    pub unicode: ::core::ffi::c_int,
    pub glyph: otfcc_GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_glyf {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_GlyphPtr,
}
pub type glyf_GlyphPtr = *mut glyf_Glyph;
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
pub type otfcc_FDHandle = otfcc_Handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_MaskList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_PostscriptHintMask,
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
pub struct glyf_StemDefList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_PostscriptStemDef,
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
pub struct glyf_ReferenceList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_ComponentReference,
}
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
pub type shapeid_t = uint16_t;
pub type RefAnchorStatus = ::core::ffi::c_uint;
pub const REF_ANCHOR_CONSOLIDATING_XY: RefAnchorStatus = 5;
pub const REF_ANCHOR_CONSOLIDATING_ANCHOR: RefAnchorStatus = 4;
pub const REF_ANCHOR_CONSOLIDATED: RefAnchorStatus = 3;
pub const REF_ANCHOR_XY: RefAnchorStatus = 2;
pub const REF_ANCHOR_ANCHOR: RefAnchorStatus = 1;
pub const REF_XY: RefAnchorStatus = 0;
pub type scale_t = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VQ {
    pub kernel: pos_t,
    pub shift: vq_SegList,
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
pub struct vq_Segment {
    pub type_0: VQSegType,
    pub val: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub still: pos_t,
    pub delta: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub quantity: pos_t,
    pub touched: bool,
    pub region: *const vq_Region,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_Region {
    pub dimensions: shapeid_t,
    pub spans: [vq_AxisSpan; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_AxisSpan {
    pub start: pos_t,
    pub peak: pos_t,
    pub end: pos_t,
}
pub type VQSegType = ::core::ffi::c_uint;
pub const VQ_DELTA: VQSegType = 1;
pub const VQ_STILL: VQSegType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ContourList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_Contour,
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
pub struct glyf_Point {
    pub x: VQ,
    pub y: VQ,
    pub onCurve: int8_t,
}
pub type table_CFF = _table_CFF;
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
pub type arity_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_VORG {
    pub numVertOriginYMetrics: glyphid_t,
    pub defaultVerticalOrigin: pos_t,
    pub entries: *mut VORG_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VORG_entry {
    pub gid: glyphid_t,
    pub verticalOrigin: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_vmtx {
    pub metrics: *mut vertical_metric,
    pub topSideBearing: *mut pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vertical_metric {
    pub advanceHeight: length_t,
    pub tsb: pos_t,
}
pub type length_t = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_vhea {
    pub version: f16dot16,
    pub ascent: int16_t,
    pub descent: int16_t,
    pub lineGap: int16_t,
    pub advanceHeightMax: int16_t,
    pub minTop: int16_t,
    pub minBottom: int16_t,
    pub yMaxExtent: int16_t,
    pub caretSlopeRise: int16_t,
    pub caretSlopeRun: int16_t,
    pub caretOffset: int16_t,
    pub dummy0: int16_t,
    pub dummy1: int16_t,
    pub dummy2: int16_t,
    pub dummy3: int16_t,
    pub metricDataFormat: int16_t,
    pub numOfLongVerMetrics: uint16_t,
}
pub type f16dot16 = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_hdmx {
    pub version: uint16_t,
    pub numRecords: uint16_t,
    pub sizeDeviceRecord: uint32_t,
    pub records: *mut device_record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device_record {
    pub pixelSize: uint8_t,
    pub maxWidth: uint8_t,
    pub widths: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_post {
    pub version: f16dot16,
    pub italicAngle: f16dot16,
    pub underlinePosition: int16_t,
    pub underlineThickness: int16_t,
    pub isFixedPitch: uint32_t,
    pub minMemType42: uint32_t,
    pub maxMemType42: uint32_t,
    pub minMemType1: uint32_t,
    pub maxMemType1: uint32_t,
    pub post_name_map: *mut otfcc_GlyphOrder,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_hmtx {
    pub metrics: *mut horizontal_metric,
    pub leftSideBearing: *mut pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct horizontal_metric {
    pub advanceWidth: length_t,
    pub lsb: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_OS_2 {
    pub version: uint16_t,
    pub xAvgCharWidth: int16_t,
    pub usWeightClass: uint16_t,
    pub usWidthClass: uint16_t,
    pub fsType: uint16_t,
    pub ySubscriptXSize: int16_t,
    pub ySubscriptYSize: int16_t,
    pub ySubscriptXOffset: int16_t,
    pub ySubscriptYOffset: int16_t,
    pub ySupscriptXSize: int16_t,
    pub ySupscriptYSize: int16_t,
    pub ySupscriptXOffset: int16_t,
    pub ySupscriptYOffset: int16_t,
    pub yStrikeoutSize: int16_t,
    pub yStrikeoutPosition: int16_t,
    pub sFamilyClass: int16_t,
    pub panose: [uint8_t; 10],
    pub ulUnicodeRange1: uint32_t,
    pub ulUnicodeRange2: uint32_t,
    pub ulUnicodeRange3: uint32_t,
    pub ulUnicodeRange4: uint32_t,
    pub achVendID: [uint8_t; 4],
    pub fsSelection: uint16_t,
    pub usFirstCharIndex: uint16_t,
    pub usLastCharIndex: uint16_t,
    pub sTypoAscender: int16_t,
    pub sTypoDescender: int16_t,
    pub sTypoLineGap: int16_t,
    pub usWinAscent: uint16_t,
    pub usWinDescent: uint16_t,
    pub ulCodePageRange1: uint32_t,
    pub ulCodePageRange2: uint32_t,
    pub sxHeight: int16_t,
    pub sCapHeight: int16_t,
    pub usDefaultChar: uint16_t,
    pub usBreakChar: uint16_t,
    pub usMaxContext: uint16_t,
    pub usLowerOpticalPointSize: uint16_t,
    pub usUpperOpticalPointSize: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_maxp {
    pub version: f16dot16,
    pub numGlyphs: uint16_t,
    pub maxPoints: uint16_t,
    pub maxContours: uint16_t,
    pub maxCompositePoints: uint16_t,
    pub maxCompositeContours: uint16_t,
    pub maxZones: uint16_t,
    pub maxTwilightPoints: uint16_t,
    pub maxStorage: uint16_t,
    pub maxFunctionDefs: uint16_t,
    pub maxInstructionDefs: uint16_t,
    pub maxStackElements: uint16_t,
    pub maxSizeOfInstructions: uint16_t,
    pub maxComponentElements: uint16_t,
    pub maxComponentDepth: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_hhea {
    pub version: f16dot16,
    pub ascender: int16_t,
    pub descender: int16_t,
    pub lineGap: int16_t,
    pub advanceWidthMax: uint16_t,
    pub minLeftSideBearing: int16_t,
    pub minRightSideBearing: int16_t,
    pub xMaxExtent: int16_t,
    pub caretSlopeRise: int16_t,
    pub caretSlopeRun: int16_t,
    pub caretOffset: int16_t,
    pub reserved: [int16_t; 4],
    pub metricDataFormat: int16_t,
    pub numberOfMetrics: uint16_t,
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
pub struct table_fvar {
    pub majorVersion: uint16_t,
    pub minorVersion: uint16_t,
    pub axes: vf_Axes,
    pub instances: fvar_InstanceList,
    pub masters: *mut fvar_Master,
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
pub struct fvar_InstanceList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut fvar_Instance,
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
pub struct VV {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut pos_t,
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
pub struct vf_Axis {
    pub tag: uint32_t,
    pub minValue: pos_t,
    pub defaultValue: pos_t,
    pub maxValue: pos_t,
    pub flags: uint16_t,
    pub axisNameID: uint16_t,
}
pub type otfcc_font_subtype = ::core::ffi::c_uint;
pub const FONTTYPE_CFF: otfcc_font_subtype = 1;
pub const FONTTYPE_TTF: otfcc_font_subtype = 0;
pub type otfcc_Font = _caryll_font;
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
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed_8 = 10;
pub const log_vl_info: C2RustUnnamed_8 = 5;
pub const log_vl_notice: C2RustUnnamed_8 = 2;
pub const log_vl_important: C2RustUnnamed_8 = 1;
pub const log_vl_critical: C2RustUnnamed_8 = 0;
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
pub struct __caryll_elementinterface_otfcc_Font {
    pub init: Option<unsafe extern "C" fn(*mut otfcc_Font) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Font) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otfcc_Font, *mut otfcc_Font) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_Font) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otfcc_Font, otfcc_Font) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otfcc_Font, otfcc_Font) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otfcc_Font>,
    pub free: Option<unsafe extern "C" fn(*mut otfcc_Font) -> ()>,
    pub consolidate: Option<unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Options) -> ()>,
    pub createTable:
        Option<unsafe extern "C" fn(*mut otfcc_Font, uint32_t) -> *mut ::core::ffi::c_void>,
    pub deleteTable: Option<unsafe extern "C" fn(*mut otfcc_Font, uint32_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_IFontBuilder {
    pub read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            uint32_t,
            *const otfcc_Options,
        ) -> *mut otfcc_Font,
    >,
    pub free: Option<unsafe extern "C" fn(*mut otfcc_IFontBuilder) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_IFontSerializer {
    pub serialize: Option<
        unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Options) -> *mut ::core::ffi::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut otfcc_IFontSerializer) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const json_serialize_mode_multiline: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const json_serialize_mode_packed: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn getchar() -> ::core::ffi::c_int {
    return getc(stdin);
}
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
    let mut ttcindex: uint32_t = 0 as uint32_t;
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
                ttcindex = atoi(optarg) as uint32_t;
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
        }) as uint8_t,
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
            log_vl_critical as ::core::ffi::c_int as uint8_t,
            log_type_error,
            sdscatprintf(
                sdsempty(),
                b"Expected argument for input file name.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            ),
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
        sdscatprintf(
            sdsempty(),
            b"Read SFNT\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as uint8_t,
            log_type_progress,
            sdscatprintf(
                sdsempty(),
                b"From file %s\0" as *const u8 as *const ::core::ffi::c_char,
                inPath,
            ),
        );
        let mut file: *mut FILE = fopen(
            inPath as *const ::core::ffi::c_char,
            b"rb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        sfnt = otfcc_readSFNT(file);
        if sfnt.is_null() || (*sfnt).count == 0 as uint32_t {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_critical as ::core::ffi::c_int as uint8_t,
                log_type_error,
                sdscatprintf(
                    sdsempty(),
                    b"Cannot read SFNT file \"%s\". Exit.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    inPath,
                ),
            );
            exit(EXIT_FAILURE);
        }
        if ttcindex >= (*sfnt).count {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_critical as ::core::ffi::c_int as uint8_t,
                log_type_error,
                sdscatprintf(
                    sdsempty(),
                    b"Subfont index %d out of range for \"%s\" (0 -- %d). Exit.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    ttcindex,
                    inPath,
                    (*sfnt).count.wrapping_sub(1 as uint32_t),
                ),
            );
            exit(EXIT_FAILURE);
        }
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as uint8_t,
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
        sdscatprintf(
            sdsempty(),
            b"Read Font\0" as *const u8 as *const ::core::ffi::c_char,
        ),
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
                log_vl_critical as ::core::ffi::c_int as uint8_t,
                log_type_error,
                sdscatprintf(
                    sdsempty(),
                    b"Font structure broken or corrupted \"%s\". Exit.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    inPath,
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
            log_vl_progress as ::core::ffi::c_int as uint8_t,
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
        sdscatprintf(
            sdsempty(),
            b"Consolidate\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        otfcc_iFont.consolidate.expect("non-null function pointer")(font, options);
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as uint8_t,
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
        sdscatprintf(
            sdsempty(),
            b"Dump\0" as *const u8 as *const ::core::ffi::c_char,
        ),
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
                log_vl_critical as ::core::ffi::c_int as uint8_t,
                log_type_error,
                sdscatprintf(
                    sdsempty(),
                    b"Font structure broken or corrupted \"%s\". Exit.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    inPath,
                ),
            );
            exit(EXIT_FAILURE);
        }
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as uint8_t,
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
    let mut buflen: size_t = 0;
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"Serialize to JSON\0" as *const u8 as *const ::core::ffi::c_char,
        ),
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
        buf = calloc(1 as size_t, buflen) as *mut ::core::ffi::c_char;
        json_serialize_ex(buf, root, jsonOptions);
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress as ::core::ffi::c_int as uint8_t,
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
        sdscatprintf(
            sdsempty(),
            b"Output\0" as *const u8 as *const ::core::ffi::c_char,
        ),
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
                    log_vl_critical as ::core::ffi::c_int as uint8_t,
                    log_type_error,
                    sdscatprintf(
                        sdsempty(),
                        b"Cannot write to file \"%s\". Exit.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        outputPath,
                    ),
                );
                exit(EXIT_FAILURE);
            }
            if add_bom {
                fputc(0xef as ::core::ffi::c_int, outputFile);
                fputc(0xbb as ::core::ffi::c_int, outputFile);
                fputc(0xbf as ::core::ffi::c_int, outputFile);
            }
            let mut actualLen: size_t = buflen.wrapping_sub(1 as size_t);
            while *buf.offset(actualLen as isize) == 0 {
                actualLen = actualLen.wrapping_sub(1 as size_t);
            }
            fwrite(
                buf as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
                actualLen.wrapping_add(1 as size_t),
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
            log_vl_progress as ::core::ffi::c_int as uint8_t,
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
        sdscatprintf(
            sdsempty(),
            b"Finalize\0" as *const u8 as *const ::core::ffi::c_char,
        ),
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
            log_vl_progress as ::core::ffi::c_int as uint8_t,
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
pub const MAIN_VER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SECONDARY_VER: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PATCH_VER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
