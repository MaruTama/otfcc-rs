use libc::{exit, free, malloc, memcmp, memset, strcmp, strtol};
extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static otfcc_iFont: __caryll_elementinterface_otfcc_Font;
    fn otfcc_parseHead(root: *const json_value, options: *const otfcc_Options) -> *mut table_head;
    fn otfcc_parseGlyf(
        root: *const json_value,
        glyph_order: *mut otfcc_GlyphOrder,
        options: *const otfcc_Options,
    ) -> *mut table_glyf;
    fn otfcc_parseCFF(root: *const json_value, options: *const otfcc_Options) -> *mut table_CFF;
    fn otfcc_parseMaxp(root: *const json_value, options: *const otfcc_Options) -> *mut table_maxp;
    fn otfcc_parseHhea(root: *const json_value, options: *const otfcc_Options) -> *mut table_hhea;
    fn otfcc_parseVhea(root: *const json_value, options: *const otfcc_Options) -> *mut table_vhea;
    fn otfcc_parseOS_2(root: *const json_value, options: *const otfcc_Options) -> *mut table_OS_2;
    fn otfcc_parsePost(root: *const json_value, options: *const otfcc_Options) -> *mut table_post;
    fn otfcc_parseName(root: *const json_value, options: *const otfcc_Options) -> *mut table_name;
    fn otfcc_parseMeta(root: *const json_value, options: *const otfcc_Options) -> *mut table_meta;
    fn otfcc_parseCmap(root: *const json_value, options: *const otfcc_Options) -> *mut table_cmap;
    fn otfcc_parseCvt(
        root: *const json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    ) -> *mut table_cvt;
    fn otfcc_parseFpgmPrep(
        root: *const json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    ) -> *mut table_fpgm_prep;
    fn otfcc_parseGasp(root: *const json_value, options: *const otfcc_Options) -> *mut table_gasp;
    fn otfcc_parseVDMX(root: *const json_value, options: *const otfcc_Options) -> *mut table_VDMX;
    fn otfcc_parseGDEF(root: *const json_value, options: *const otfcc_Options) -> *mut table_GDEF;
    fn otfcc_parseBASE(root: *const json_value, options: *const otfcc_Options) -> *mut table_BASE;
    fn otfcc_parseOtl(
        root: *const json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    ) -> *mut table_OTL;
    fn otfcc_parseCPAL(root: *const json_value, options: *const otfcc_Options) -> *mut table_CPAL;
    fn otfcc_parseCOLR(root: *const json_value, options: *const otfcc_Options) -> *mut table_COLR;
    fn otfcc_parseSVG(root: *const json_value, options: *const otfcc_Options) -> *mut table_SVG;
    fn otfcc_parseTSI(
        root: *const json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    ) -> *mut table_TSI;
    fn otfcc_parseTSI5(root: *const json_value, options: *const otfcc_Options) -> *mut table_TSI5;
}

use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::otf_reader::FontBuilder;
use crate::logger::{log_type_info, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{arity_t, colorid_t, f16dot16, glyphclass_t, glyphid_t, glyphsize_t, length_t, pos_t, scale_t, shapeid_t, tableid_t};
use crate::vendor::sds::{sds};
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
    pub integer: i64,
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
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: u8,
    pub alloc: u8,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: u16,
    pub alloc: u16,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: u32,
    pub alloc: u32,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: u64,
    pub alloc: u64,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
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
    pub hho: isize,
    pub ideal_chain_maxlen: ::core::ffi::c_uint,
    pub nonideal_items: ::core::ffi::c_uint,
    pub ineff_expands: ::core::ffi::c_uint,
    pub noexpand: ::core::ffi::c_uint,
    pub signature: u32,
}
pub type otfcc_FDHandle = otfcc_Handle;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed_4 = 10;
pub const log_vl_info: C2RustUnnamed_4 = 5;
pub const log_vl_notice: C2RustUnnamed_4 = 2;
pub const log_vl_important: C2RustUnnamed_4 = 1;
pub const log_vl_critical: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_GlyphOrderEntry {
    pub gid: glyphid_t,
    pub name: sds,
    pub orderType: u8,
    pub orderEntry: u32,
    pub hhID: UT_hash_handle,
    pub hhName: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_GlyphOrder {
    pub byGID: *mut otfcc_GlyphOrderEntry,
    pub byName: *mut otfcc_GlyphOrderEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_GlyphOrderPackage {
    pub init: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, *const otfcc_GlyphOrder) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, *mut otfcc_GlyphOrder) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, otfcc_GlyphOrder) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, otfcc_GlyphOrder) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otfcc_GlyphOrder>,
    pub free: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder) -> ()>,
    pub setByGID: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, glyphid_t, sds) -> sds>,
    pub setByName: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, sds, glyphid_t) -> bool>,
    pub nameAField_Shared:
        Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, glyphid_t, *mut sds) -> bool>,
    pub consolidateHandle:
        Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, *mut otfcc_GlyphHandle) -> bool>,
    pub lookupName: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, sds) -> bool>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VV {
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
pub struct vf_Axis {
    pub tag: u32,
    pub minValue: pos_t,
    pub defaultValue: pos_t,
    pub maxValue: pos_t,
    pub flags: u16,
    pub axisNameID: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vf_Axes {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut vf_Axis,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fvar_Instance {
    pub subfamilyNameID: u16,
    pub flags: u16,
    pub coordinates: VV,
    pub postScriptNameID: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fvar_InstanceList {
    pub length: usize,
    pub capacity: usize,
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
    pub majorVersion: u16,
    pub minorVersion: u16,
    pub axes: vf_Axes,
    pub instances: fvar_InstanceList,
    pub masters: *mut fvar_Master,
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
pub type table_TSI5 = otl_ClassDef;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_TSI {
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
pub struct table_COLR {
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
    pub items: *mut colr_Layer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colr_Layer {
    pub glyph: otfcc_GlyphHandle,
    pub paletteIndex: colorid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_CPAL {
    pub version: u16,
    pub palettes: cpal_PaletteSet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_PaletteSet {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut cpal_Palette,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_Palette {
    pub colorset: cpal_ColorSet,
    pub type_0: u32,
    pub label: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_ColorSet {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut cpal_Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub label: u16,
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
    pub tag: u32,
    pub defaultBaselineTag: u32,
    pub baseValuesCount: tableid_t,
    pub baseValues: *mut otl_BaseValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseValue {
    pub tag: u32,
    pub coordinate: pos_t,
}
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_CaretValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_CaretValue {
    pub format: i8,
    pub coordiante: pos_t,
    pub pointIndex: i16,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_LookupRef,
}
pub type otl_LookupRef = *const otl_Lookup;
pub type otl_Lookup = _otl_lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _otl_lookup {
    pub name: sds,
    pub type_0: otl_LookupType,
    pub _offset: u32,
    pub flags: u16,
    pub subtables: otl_SubtableList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_SubtableList {
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
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
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_FeaturePtr,
}
pub type otl_FeaturePtr = *mut otl_Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LookupList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otl_LookupPtr,
}
pub type otl_LookupPtr = *mut otl_Lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_LTSH {
    pub version: u16,
    pub numGlyphs: glyphid_t,
    pub yPels: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_VDMX {
    pub version: u16,
    pub ratios: vdmx_RatioRagneList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_RatioRagneList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut vdmx_RatioRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_RatioRange {
    pub bCharset: u8,
    pub xRatio: u8,
    pub yStartRatio: u8,
    pub yEndRatio: u8,
    pub records: vdmx_Group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_Group {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut vdmx_Record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_Record {
    pub yPelHeight: u16,
    pub yMax: i16,
    pub yMin: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_gasp {
    pub version: u16,
    pub records: gasp_RecordList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gasp_RecordList {
    pub length: usize,
    pub capacity: usize,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_cvt {
    pub length: u32,
    pub words: *mut u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_fpgm_prep {
    pub tag: sds,
    pub length: u32,
    pub bytes: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_meta {
    pub version: u32,
    pub flags: u32,
    pub entries: meta_Entries,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meta_Entries {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut meta_Entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meta_Entry {
    pub tag: u32,
    pub data: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_name {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otfcc_NameRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_NameRecord {
    pub platformID: u16,
    pub encodingID: u16,
    pub languageID: u16,
    pub nameID: u16,
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
    pub unicode: u32,
    pub selector: u32,
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
    pub length: usize,
    pub capacity: usize,
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
    pub instructionsLength: u16,
    pub instructions: *mut u8,
    pub yPel: u8,
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
    pub nestDepth: u16,
    pub nPoints: u16,
    pub nContours: u16,
    pub nCompositePoints: u16,
    pub nCompositeContours: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_MaskList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_PostscriptHintMask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PostscriptHintMask {
    pub pointsBefore: u16,
    pub contoursBefore: u16,
    pub maskH: [bool; 256],
    pub maskV: [bool; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_StemDefList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_PostscriptStemDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PostscriptStemDef {
    pub position: pos_t,
    pub width: pos_t,
    pub map: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ReferenceList {
    pub length: usize,
    pub capacity: usize,
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
pub type RefAnchorStatus = ::core::ffi::c_uint;
pub const REF_ANCHOR_CONSOLIDATING_XY: RefAnchorStatus = 5;
pub const REF_ANCHOR_CONSOLIDATING_ANCHOR: RefAnchorStatus = 4;
pub const REF_ANCHOR_CONSOLIDATED: RefAnchorStatus = 3;
pub const REF_ANCHOR_XY: RefAnchorStatus = 2;
pub const REF_ANCHOR_ANCHOR: RefAnchorStatus = 1;
pub const REF_XY: RefAnchorStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ContourList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_Contour,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Contour {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_Point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Point {
    pub x: VQ,
    pub y: VQ,
    pub onCurve: i8,
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
    pub cidSupplement: u32,
    pub cidFontVersion: ::core::ffi::c_double,
    pub cidFontRevision: ::core::ffi::c_double,
    pub cidCount: u32,
    pub UIDBase: u32,
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
    pub languageGroup: u32,
    pub expansionFactor: ::core::ffi::c_double,
    pub initialRandomSeed: ::core::ffi::c_double,
    pub defaultWidthX: ::core::ffi::c_double,
    pub nominalWidthX: ::core::ffi::c_double,
}
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
    pub verticalOrigin: i16,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_vhea {
    pub version: f16dot16,
    pub ascent: i16,
    pub descent: i16,
    pub lineGap: i16,
    pub advanceHeightMax: i16,
    pub minTop: i16,
    pub minBottom: i16,
    pub yMaxExtent: i16,
    pub caretSlopeRise: i16,
    pub caretSlopeRun: i16,
    pub caretOffset: i16,
    pub dummy0: i16,
    pub dummy1: i16,
    pub dummy2: i16,
    pub dummy3: i16,
    pub metricDataFormat: i16,
    pub numOfLongVerMetrics: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_hdmx {
    pub version: u16,
    pub numRecords: u16,
    pub sizeDeviceRecord: u32,
    pub records: *mut device_record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device_record {
    pub pixelSize: u8,
    pub maxWidth: u8,
    pub widths: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_post {
    pub version: f16dot16,
    pub italicAngle: f16dot16,
    pub underlinePosition: i16,
    pub underlineThickness: i16,
    pub isFixedPitch: u32,
    pub minMemType42: u32,
    pub maxMemType42: u32,
    pub minMemType1: u32,
    pub maxMemType1: u32,
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
    pub version: u16,
    pub xAvgCharWidth: i16,
    pub usWeightClass: u16,
    pub usWidthClass: u16,
    pub fsType: u16,
    pub ySubscriptXSize: i16,
    pub ySubscriptYSize: i16,
    pub ySubscriptXOffset: i16,
    pub ySubscriptYOffset: i16,
    pub ySupscriptXSize: i16,
    pub ySupscriptYSize: i16,
    pub ySupscriptXOffset: i16,
    pub ySupscriptYOffset: i16,
    pub yStrikeoutSize: i16,
    pub yStrikeoutPosition: i16,
    pub sFamilyClass: i16,
    pub panose: [u8; 10],
    pub ulUnicodeRange1: u32,
    pub ulUnicodeRange2: u32,
    pub ulUnicodeRange3: u32,
    pub ulUnicodeRange4: u32,
    pub achVendID: [u8; 4],
    pub fsSelection: u16,
    pub usFirstCharIndex: u16,
    pub usLastCharIndex: u16,
    pub sTypoAscender: i16,
    pub sTypoDescender: i16,
    pub sTypoLineGap: i16,
    pub usWinAscent: u16,
    pub usWinDescent: u16,
    pub ulCodePageRange1: u32,
    pub ulCodePageRange2: u32,
    pub sxHeight: i16,
    pub sCapHeight: i16,
    pub usDefaultChar: u16,
    pub usBreakChar: u16,
    pub usMaxContext: u16,
    pub usLowerOpticalPointSize: u16,
    pub usUpperOpticalPointSize: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_maxp {
    pub version: f16dot16,
    pub numGlyphs: u16,
    pub maxPoints: u16,
    pub maxContours: u16,
    pub maxCompositePoints: u16,
    pub maxCompositeContours: u16,
    pub maxZones: u16,
    pub maxTwilightPoints: u16,
    pub maxStorage: u16,
    pub maxFunctionDefs: u16,
    pub maxInstructionDefs: u16,
    pub maxStackElements: u16,
    pub maxSizeOfInstructions: u16,
    pub maxComponentElements: u16,
    pub maxComponentDepth: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_hhea {
    pub version: f16dot16,
    pub ascender: i16,
    pub descender: i16,
    pub lineGap: i16,
    pub advanceWidthMax: u16,
    pub minLeftSideBearing: i16,
    pub minRightSideBearing: i16,
    pub xMaxExtent: i16,
    pub caretSlopeRise: i16,
    pub caretSlopeRun: i16,
    pub caretOffset: i16,
    pub reserved: [i16; 4],
    pub metricDataFormat: i16,
    pub numberOfMetrics: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_head {
    pub version: f16dot16,
    pub fontRevision: u32,
    pub checkSumAdjustment: u32,
    pub magicNumber: u32,
    pub flags: u16,
    pub unitsPerEm: u16,
    pub created: i64,
    pub modified: i64,
    pub xMin: i16,
    pub yMin: i16,
    pub xMax: i16,
    pub yMax: i16,
    pub macStyle: u16,
    pub lowestRecPPEM: u16,
    pub fontDirectoryHint: i16,
    pub indexToLocFormat: i16,
    pub glyphDataFormat: i16,
}
pub type otfcc_font_subtype = ::core::ffi::c_uint;
pub const FONTTYPE_CFF: otfcc_font_subtype = 1;
pub const FONTTYPE_TTF: otfcc_font_subtype = 0;
pub type otfcc_Font = _caryll_font;
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
        Option<unsafe extern "C" fn(*mut otfcc_Font, u32) -> *mut ::core::ffi::c_void>,
    pub deleteTable: Option<unsafe extern "C" fn(*mut otfcc_Font, u32) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_IFontBuilder {
    pub read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            u32,
            *const otfcc_Options,
        ) -> *mut otfcc_Font,
    >,
    pub free: Option<unsafe extern "C" fn(*mut otfcc_IFontBuilder) -> ()>,
}
pub const ORD_GLYPHORDER: C2RustUnnamed_9 = 1;
pub const ORD_CMAP: C2RustUnnamed_9 = 3;
pub const ORD_GLYF: C2RustUnnamed_9 = 4;
pub const ORD_NOTDEF: C2RustUnnamed_9 = 2;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
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
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
pub const HASH_INITIAL_NUM_BUCKETS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const HASH_INITIAL_NUM_BUCKETS_LOG2: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const HASH_BKT_CAPACITY_THRESH: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
pub const HASH_SIGNATURE: ::core::ffi::c_uint = 0xa0111fe1 as ::core::ffi::c_uint;
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
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
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
unsafe extern "C" fn otfcc_decideFontSubtypeFromJson(
    mut root: *const json_value,
) -> otfcc_font_subtype {
    if !json_obj_get_type(
        root,
        b"CFF_\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    )
    .is_null()
    {
        return FONTTYPE_CFF;
    } else {
        return FONTTYPE_TTF;
    };
}
unsafe extern "C" fn setOrderByName(
    mut go: *mut otfcc_GlyphOrder,
    mut name: sds,
    mut orderType: u8,
    mut orderEntry: u32,
) {
    let mut s: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = name as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = sdslen(name) as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(sdslen(name) as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 17804697211240665320;
        }
        10 => {
            current_block_50 = 17804697211240665320;
        }
        9 => {
            current_block_50 = 10934104523405478302;
        }
        8 => {
            current_block_50 = 18021056235773049229;
        }
        7 => {
            current_block_50 = 1545999744855823442;
        }
        6 => {
            current_block_50 = 5785022371778852742;
        }
        5 => {
            current_block_50 = 852838572082876989;
        }
        4 => {
            current_block_50 = 4171976486760612607;
        }
        3 => {
            current_block_50 = 8689672022250159862;
        }
        2 => {
            current_block_50 = 5254886831261953223;
        }
        1 => {
            current_block_50 = 380073568100959624;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        17804697211240665320 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 10934104523405478302;
        }
        _ => {}
    }
    match current_block_50 {
        10934104523405478302 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 18021056235773049229;
        }
        _ => {}
    }
    match current_block_50 {
        18021056235773049229 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 1545999744855823442;
        }
        _ => {}
    }
    match current_block_50 {
        1545999744855823442 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 5785022371778852742;
        }
        _ => {}
    }
    match current_block_50 {
        5785022371778852742 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 852838572082876989;
        }
        _ => {}
    }
    match current_block_50 {
        852838572082876989 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 4171976486760612607;
        }
        _ => {}
    }
    match current_block_50 {
        4171976486760612607 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 8689672022250159862;
        }
        _ => {}
    }
    match current_block_50 {
        8689672022250159862 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 5254886831261953223;
        }
        _ => {}
    }
    match current_block_50 {
        5254886831261953223 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 380073568100959624;
        }
        _ => {}
    }
    match current_block_50 {
        380073568100959624 => {
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
    s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
    if !(*go).byName.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*go).byName).hhName.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*go).byName).hhName.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                s = ((*(*(*(*go).byName).hhName.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).byName).hhName.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut otfcc_GlyphOrderEntry
                    as *mut otfcc_GlyphOrderEntry;
            } else {
                s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
            }
            while !s.is_null() {
                if (*s).hhName.hashv == _hf_hashv && (*s).hhName.keylen as usize == sdslen(name) {
                    if memcmp(
                        (*s).hhName.key,
                        name as *const ::core::ffi::c_void,
                        sdslen(name),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hhName.hh_next.is_null() {
                    s = ((*s).hhName.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byName).hhName.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut otfcc_GlyphOrderEntry
                        as *mut otfcc_GlyphOrderEntry;
                } else {
                    s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
                }
            }
        }
    }
    if s.is_null() {
        s = __caryll_allocate_clean(
            ::core::mem::size_of::<otfcc_GlyphOrderEntry>() as usize,
            21 as ::core::ffi::c_ulong,
        ) as *mut otfcc_GlyphOrderEntry;
        (*s).gid = -(1 as ::core::ffi::c_int) as glyphid_t;
        (*s).name = name;
        (*s).orderType = orderType;
        (*s).orderEntry = orderEntry;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            (*s).name.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = sdslen((*s).name) as ::core::ffi::c_uint;
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
        _ha_hashv = _ha_hashv.wrapping_add(sdslen((*s).name) as ::core::ffi::c_uint);
        let mut current_block_169: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 12766349834733685609;
            }
            10 => {
                current_block_169 = 12766349834733685609;
            }
            9 => {
                current_block_169 = 921014338214990409;
            }
            8 => {
                current_block_169 = 10918256127192299627;
            }
            7 => {
                current_block_169 = 9578982516296139066;
            }
            6 => {
                current_block_169 = 4934683382580054635;
            }
            5 => {
                current_block_169 = 9066128494507323840;
            }
            4 => {
                current_block_169 = 11126302601059242051;
            }
            3 => {
                current_block_169 = 552284807196668465;
            }
            2 => {
                current_block_169 = 13163467591269913071;
            }
            1 => {
                current_block_169 = 9704386208929738663;
            }
            _ => {
                current_block_169 = 14714495436747744489;
            }
        }
        match current_block_169 {
            12766349834733685609 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 921014338214990409;
            }
            _ => {}
        }
        match current_block_169 {
            921014338214990409 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 10918256127192299627;
            }
            _ => {}
        }
        match current_block_169 {
            10918256127192299627 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 9578982516296139066;
            }
            _ => {}
        }
        match current_block_169 {
            9578982516296139066 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 4934683382580054635;
            }
            _ => {}
        }
        match current_block_169 {
            4934683382580054635 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 9066128494507323840;
            }
            _ => {}
        }
        match current_block_169 {
            9066128494507323840 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_169 = 11126302601059242051;
            }
            _ => {}
        }
        match current_block_169 {
            11126302601059242051 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 552284807196668465;
            }
            _ => {}
        }
        match current_block_169 {
            552284807196668465 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 13163467591269913071;
            }
            _ => {}
        }
        match current_block_169 {
            13163467591269913071 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 9704386208929738663;
            }
            _ => {}
        }
        match current_block_169 {
            9704386208929738663 => {
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
        (*s).hhName.hashv = _ha_hashv;
        (*s).hhName.key = (*s).name.offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hhName.keylen = sdslen((*s).name) as ::core::ffi::c_uint;
        if (*go).byName.is_null() {
            (*s).hhName.next = NULL_0;
            (*s).hhName.prev = NULL_0;
            (*s).hhName.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*s).hhName.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*s).hhName.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as usize,
                );
                (*(*s).hhName.tbl).tail = &raw mut (*s).hhName as *mut UT_hash_handle;
                (*(*s).hhName.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*s).hhName.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*s).hhName.tbl).hho = (&raw mut (*s).hhName as *mut ::core::ffi::c_char)
                    .offset_from(s as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*s).hhName.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                (*(*s).hhName.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*s).hhName.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*s).hhName.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    );
                }
            }
            (*go).byName = s;
        } else {
            (*s).hhName.tbl = (*(*go).byName).hhName.tbl;
            (*s).hhName.next = NULL_0;
            (*s).hhName.prev = ((*(*(*go).byName).hhName.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*go).byName).hhName.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*go).byName).hhName.tbl).tail).next = s as *mut ::core::ffi::c_void;
            (*(*(*go).byName).hhName.tbl).tail = &raw mut (*s).hhName as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*go).byName).hhName.tbl).num_items =
            (*(*(*go).byName).hhName.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*go).byName).hhName.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = (*(*(*go).byName).hhName.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*s).hhName.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*s).hhName.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hhName as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*s).hhName as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*s).hhName.tbl).noexpand == 0
        {
            let mut _he_bkt: ::core::ffi::c_uint = 0;
            let mut _he_bkt_i: ::core::ffi::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_hh_nxt: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_new_buckets: *mut UT_hash_bucket =
                ::core::ptr::null_mut::<UT_hash_bucket>();
            let mut _he_newbkt: *mut UT_hash_bucket = ::core::ptr::null_mut::<UT_hash_bucket>();
            _he_new_buckets = malloc(
                (2 as usize)
                    .wrapping_mul((*(*s).hhName.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*s).hhName.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                );
                (*(*s).hhName.tbl).ideal_chain_maxlen = ((*(*s).hhName.tbl).num_items
                    >> (*(*s).hhName.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*s).hhName.tbl).num_items
                        & (*(*s).hhName.tbl)
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
                (*(*s).hhName.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*s).hhName.tbl).num_buckets {
                    _he_thh = (*(*(*s).hhName.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*s).hhName.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*s).hhName.tbl).ideal_chain_maxlen {
                            (*(*s).hhName.tbl).nonideal_items =
                                (*(*s).hhName.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*s).hhName.tbl).ideal_chain_maxlen);
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
                free((*(*s).hhName.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*s).hhName.tbl).num_buckets = (*(*s).hhName.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*s).hhName.tbl).log2_num_buckets =
                    (*(*s).hhName.tbl).log2_num_buckets.wrapping_add(1);
                (*(*s).hhName.tbl).buckets = _he_new_buckets;
                (*(*s).hhName.tbl).ineff_expands = if (*(*s).hhName.tbl).nonideal_items
                    > (*(*s).hhName.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*s).hhName.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*s).hhName.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*s).hhName.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
    } else if (*s).orderType as ::core::ffi::c_int > orderType as ::core::ffi::c_int {
        (*s).orderType = orderType;
        (*s).orderEntry = orderEntry;
    }
}
unsafe extern "C" fn _byOrder(
    mut a: *mut otfcc_GlyphOrderEntry,
    mut b: *mut otfcc_GlyphOrderEntry,
) -> ::core::ffi::c_int {
    if ((*a).orderType as ::core::ffi::c_int) < (*b).orderType as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if (*a).orderType as ::core::ffi::c_int > (*b).orderType as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if (*a).orderEntry < (*b).orderEntry {
        return -(1 as ::core::ffi::c_int);
    }
    if (*a).orderEntry > (*b).orderEntry {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn orderGlyphs(mut go: *mut otfcc_GlyphOrder) {
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
    if !(*go).byName.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*(*go).byName).hhName as *mut UT_hash_handle;
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
                            .offset((*(*(*go).byName).hhName.tbl).hho)
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
                                .offset((*(*(*go).byName).hhName.tbl).hho)
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
                                    .offset((*(*(*go).byName).hhName.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if _byOrder(
                        (_hs_p as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*go).byName).hhName.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut otfcc_GlyphOrderEntry,
                        (_hs_q as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*go).byName).hhName.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut otfcc_GlyphOrderEntry,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*(*go).byName).hhName.tbl).hho)
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
                                .offset((*(*(*go).byName).hhName.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*(*go).byName).hhName.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL_0
                        };
                    } else {
                        _hs_list = _hs_e;
                    }
                    if !_hs_e.is_null() {
                        (*_hs_e).prev = if !_hs_tail.is_null() {
                            (_hs_tail as *mut ::core::ffi::c_char)
                                .offset(-(*(*(*go).byName).hhName.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL_0
                        };
                    }
                    _hs_tail = _hs_e;
                }
                _hs_p = _hs_q;
            }
            if !_hs_tail.is_null() {
                (*_hs_tail).next = NULL_0;
            }
            if _hs_nmerges <= 1 as ::core::ffi::c_uint {
                _hs_looping = 0 as ::core::ffi::c_uint;
                (*(*(*go).byName).hhName.tbl).tail = _hs_tail;
                (*go).byName = (_hs_list as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).byName).hhName.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut otfcc_GlyphOrderEntry
                    as *mut otfcc_GlyphOrderEntry;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut current: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
    let mut temp: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
    let mut gid: glyphid_t = 0 as glyphid_t;
    current = (*go).byName;
    temp = (if !(*go).byName.is_null() {
        (*(*go).byName).hhName.next
    } else {
        NULL_0
    }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
    while !current.is_null() {
        (*current).gid = gid;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key: *const ::core::ffi::c_uchar =
            &raw mut (*current).gid as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::core::mem::size_of::<glyphid_t>() as ::core::ffi::c_uint;
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
            _ha_hashv = _ha_hashv.wrapping_add(
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
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv =
            _ha_hashv.wrapping_add(::core::mem::size_of::<glyphid_t>() as ::core::ffi::c_uint);
        let mut current_block_122: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_122 = 6107916429970634913;
            }
            10 => {
                current_block_122 = 6107916429970634913;
            }
            9 => {
                current_block_122 = 16824043896303109816;
            }
            8 => {
                current_block_122 = 8056315725412367711;
            }
            7 => {
                current_block_122 = 3473396220534792688;
            }
            6 => {
                current_block_122 = 6187632812825568025;
            }
            5 => {
                current_block_122 = 15028834309073582064;
            }
            4 => {
                current_block_122 = 4789897784655735301;
            }
            3 => {
                current_block_122 = 16804586237124906222;
            }
            2 => {
                current_block_122 = 1446524650436596843;
            }
            1 => {
                current_block_122 = 3046610167049820456;
            }
            _ => {
                current_block_122 = 15622658527355336244;
            }
        }
        match current_block_122 {
            6107916429970634913 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_122 = 16824043896303109816;
            }
            _ => {}
        }
        match current_block_122 {
            16824043896303109816 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_122 = 8056315725412367711;
            }
            _ => {}
        }
        match current_block_122 {
            8056315725412367711 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_122 = 3473396220534792688;
            }
            _ => {}
        }
        match current_block_122 {
            3473396220534792688 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_122 = 6187632812825568025;
            }
            _ => {}
        }
        match current_block_122 {
            6187632812825568025 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_122 = 15028834309073582064;
            }
            _ => {}
        }
        match current_block_122 {
            15028834309073582064 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_122 = 4789897784655735301;
            }
            _ => {}
        }
        match current_block_122 {
            4789897784655735301 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_122 = 16804586237124906222;
            }
            _ => {}
        }
        match current_block_122 {
            16804586237124906222 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_122 = 1446524650436596843;
            }
            _ => {}
        }
        match current_block_122 {
            1446524650436596843 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_122 = 3046610167049820456;
            }
            _ => {}
        }
        match current_block_122 {
            3046610167049820456 => {
                _hj_i =
                    _hj_i
                        .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        (*current).hhID.hashv = _ha_hashv;
        (*current).hhID.key =
            &raw mut (*current).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*current).hhID.keylen = ::core::mem::size_of::<glyphid_t>() as ::core::ffi::c_uint;
        if (*go).byGID.is_null() {
            (*current).hhID.next = NULL_0;
            (*current).hhID.prev = NULL_0;
            (*current).hhID.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*current).hhID.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*current).hhID.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as usize,
                );
                (*(*current).hhID.tbl).tail = &raw mut (*current).hhID as *mut UT_hash_handle;
                (*(*current).hhID.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*current).hhID.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*current).hhID.tbl).hho = (&raw mut (*current).hhID as *mut ::core::ffi::c_char)
                    .offset_from(current as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long
                    as isize;
                (*(*current).hhID.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                (*(*current).hhID.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*current).hhID.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*current).hhID.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    );
                }
            }
            (*go).byGID = current;
        } else {
            (*current).hhID.tbl = (*(*go).byGID).hhID.tbl;
            (*current).hhID.next = NULL_0;
            (*current).hhID.prev = ((*(*(*go).byGID).hhID.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*go).byGID).hhID.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*go).byGID).hhID.tbl).tail).next = current as *mut ::core::ffi::c_void;
            (*(*(*go).byGID).hhID.tbl).tail = &raw mut (*current).hhID as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*go).byGID).hhID.tbl).num_items = (*(*(*go).byGID).hhID.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*go).byGID).hhID.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket =
            (*(*(*go).byGID).hhID.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*current).hhID.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*current).hhID.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*current).hhID as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*current).hhID as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*current).hhID.tbl).noexpand == 0
        {
            let mut _he_bkt: ::core::ffi::c_uint = 0;
            let mut _he_bkt_i: ::core::ffi::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_hh_nxt: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_new_buckets: *mut UT_hash_bucket =
                ::core::ptr::null_mut::<UT_hash_bucket>();
            let mut _he_newbkt: *mut UT_hash_bucket = ::core::ptr::null_mut::<UT_hash_bucket>();
            _he_new_buckets = malloc(
                (2 as usize)
                    .wrapping_mul((*(*current).hhID.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*current).hhID.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                );
                (*(*current).hhID.tbl).ideal_chain_maxlen = ((*(*current).hhID.tbl).num_items
                    >> (*(*current).hhID.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*current).hhID.tbl).num_items
                        & (*(*current).hhID.tbl)
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
                (*(*current).hhID.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*current).hhID.tbl).num_buckets {
                    _he_thh = (*(*(*current).hhID.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*current).hhID.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*current).hhID.tbl).ideal_chain_maxlen {
                            (*(*current).hhID.tbl).nonideal_items =
                                (*(*current).hhID.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*current).hhID.tbl).ideal_chain_maxlen);
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
                free((*(*current).hhID.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*current).hhID.tbl).num_buckets = (*(*current).hhID.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*current).hhID.tbl).log2_num_buckets =
                    (*(*current).hhID.tbl).log2_num_buckets.wrapping_add(1);
                (*(*current).hhID.tbl).buckets = _he_new_buckets;
                (*(*current).hhID.tbl).ineff_expands = if (*(*current).hhID.tbl).nonideal_items
                    > (*(*current).hhID.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*current).hhID.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*current).hhID.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*current).hhID.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        gid = (gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
        current = temp;
        temp = (if !temp.is_null() {
            (*temp).hhName.next
        } else {
            NULL_0
        }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
    }
}
unsafe extern "C" fn escalateGlyphOrderByName(
    mut go: *mut otfcc_GlyphOrder,
    mut name: sds,
    mut orderType: u8,
    mut orderEntry: u32,
) {
    let mut s: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = name as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = sdslen(name) as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(sdslen(name) as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 16638419120543210315;
        }
        10 => {
            current_block_50 = 16638419120543210315;
        }
        9 => {
            current_block_50 = 552162828606560255;
        }
        8 => {
            current_block_50 = 2647375570052691271;
        }
        7 => {
            current_block_50 = 12476858771624613021;
        }
        6 => {
            current_block_50 = 13420836126193784560;
        }
        5 => {
            current_block_50 = 6204429805193992324;
        }
        4 => {
            current_block_50 = 6265671356496406540;
        }
        3 => {
            current_block_50 = 14904062521666713051;
        }
        2 => {
            current_block_50 = 4518118397577342293;
        }
        1 => {
            current_block_50 = 6707521803593403316;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        16638419120543210315 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 552162828606560255;
        }
        _ => {}
    }
    match current_block_50 {
        552162828606560255 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 2647375570052691271;
        }
        _ => {}
    }
    match current_block_50 {
        2647375570052691271 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 12476858771624613021;
        }
        _ => {}
    }
    match current_block_50 {
        12476858771624613021 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 13420836126193784560;
        }
        _ => {}
    }
    match current_block_50 {
        13420836126193784560 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 6204429805193992324;
        }
        _ => {}
    }
    match current_block_50 {
        6204429805193992324 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 6265671356496406540;
        }
        _ => {}
    }
    match current_block_50 {
        6265671356496406540 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 14904062521666713051;
        }
        _ => {}
    }
    match current_block_50 {
        14904062521666713051 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 4518118397577342293;
        }
        _ => {}
    }
    match current_block_50 {
        4518118397577342293 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 6707521803593403316;
        }
        _ => {}
    }
    match current_block_50 {
        6707521803593403316 => {
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
    s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
    if !(*go).byName.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*go).byName).hhName.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*go).byName).hhName.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                s = ((*(*(*(*go).byName).hhName.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).byName).hhName.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut otfcc_GlyphOrderEntry
                    as *mut otfcc_GlyphOrderEntry;
            } else {
                s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
            }
            while !s.is_null() {
                if (*s).hhName.hashv == _hf_hashv && (*s).hhName.keylen as usize == sdslen(name) {
                    if memcmp(
                        (*s).hhName.key,
                        name as *const ::core::ffi::c_void,
                        sdslen(name),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hhName.hh_next.is_null() {
                    s = ((*s).hhName.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byName).hhName.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut otfcc_GlyphOrderEntry
                        as *mut otfcc_GlyphOrderEntry;
                } else {
                    s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
                }
            }
        }
    }
    if !s.is_null() && (*s).orderType as ::core::ffi::c_int > orderType as ::core::ffi::c_int {
        (*s).orderType = orderType;
        (*s).orderEntry = orderEntry;
    }
}
unsafe extern "C" fn placeOrderEntriesFromGlyf(
    mut table: *mut json_value,
    mut go: *mut otfcc_GlyphOrder,
) {
    let mut j: u32 = 0 as u32;
    while j < (*table).u.object.length as u32 {
        let mut gname: sds = sdsnewlen(
            (*(*table).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*table).u.object.values.offset(j as isize)).name_length as usize,
        );
        if strcmp(
            gname as *const ::core::ffi::c_char,
            b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            setOrderByName(
                go,
                gname,
                ORD_NOTDEF as ::core::ffi::c_int as u8,
                0 as u32,
            );
        } else if strcmp(
            gname as *const ::core::ffi::c_char,
            b".null\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            setOrderByName(
                go,
                gname,
                ORD_NOTDEF as ::core::ffi::c_int as u8,
                1 as u32,
            );
        } else {
            setOrderByName(go, gname, ORD_GLYF as ::core::ffi::c_int as u8, j);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn placeOrderEntriesFromCmap(
    mut table: *mut json_value,
    mut go: *mut otfcc_GlyphOrder,
) {
    let mut j: u32 = 0 as u32;
    while j < (*table).u.object.length as u32 {
        let mut unicodeStr: sds = sdsnewlen(
            (*(*table).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*table).u.object.values.offset(j as isize)).name_length as usize,
        );
        let mut item: *mut json_value =
            (*(*table).u.object.values.offset(j as isize)).value as *mut json_value;
        let mut unicode: i32 = 0;
        if sdslen(unicodeStr) > 2 as usize
            && *unicodeStr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'U' as i32
            && *unicodeStr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '+' as i32
        {
            unicode = strtol(
                unicodeStr.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                16 as ::core::ffi::c_int,
            ) as i32;
        } else {
            unicode = atoi(unicodeStr as *const ::core::ffi::c_char) as i32;
        }
        sdsfree(unicodeStr);
        if (*item).type_0 as ::core::ffi::c_uint
            == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
            && unicode > 0 as i32
            && unicode <= 0x10ffff as i32
        {
            let mut gname: sds = sdsnewlen(
                (*item).u.string.ptr as *const ::core::ffi::c_void,
                (*item).u.string.length as usize,
            );
            escalateGlyphOrderByName(
                go,
                gname,
                ORD_CMAP as ::core::ffi::c_int as u8,
                unicode as u32,
            );
            sdsfree(gname);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn placeOrderEntriesFromSubtable(
    mut table: *mut json_value,
    mut go: *mut otfcc_GlyphOrder,
    mut zeroOnly: bool,
) {
    let mut uplimit: u32 = (*table).u.array.length as u32;
    if uplimit >= 1 as u32 && zeroOnly as ::core::ffi::c_int != 0 {
        uplimit = 1 as u32;
    }
    let mut j: u32 = 0 as u32;
    while j < uplimit {
        let mut item: *mut json_value =
            *(*table).u.array.values.offset(j as isize) as *mut json_value;
        if (*item).type_0 as ::core::ffi::c_uint
            == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut gname: sds = sdsnewlen(
                (*item).u.string.ptr as *const ::core::ffi::c_void,
                (*item).u.string.length as usize,
            );
            escalateGlyphOrderByName(
                go,
                gname,
                ORD_GLYPHORDER as ::core::ffi::c_int as u8,
                j,
            );
            sdsfree(gname);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn parseGlyphOrder(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut otfcc_GlyphOrder {
    let mut go: *mut otfcc_GlyphOrder = (
        otfcc_pkgGlyphOrder
            .create
            .expect("non-null function pointer"))();
    if (*root).type_0 as ::core::ffi::c_uint
        != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return go;
    }
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        placeOrderEntriesFromGlyf(table, go);
        table = json_obj_get_type(
            root,
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
            json_object,
        );
        if !table.is_null() {
            placeOrderEntriesFromCmap(table, go);
        }
        table = json_obj_get_type(
            root,
            b"glyph_order\0" as *const u8 as *const ::core::ffi::c_char,
            json_array,
        );
        if !table.is_null() {
            let mut ignoreGlyphOrder: bool = (*options).ignore_glyph_order;
            if ignoreGlyphOrder as ::core::ffi::c_int != 0
                && !json_obj_get_type(
                    root,
                    b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
                    json_array,
                )
                .is_null()
            {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_notice as ::core::ffi::c_int as u8,
                    log_type_info,
                    sdscatprintf(
                        sdsempty(),
                        b"OpenType SVG table detected. Glyph order is preserved.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                );
                ignoreGlyphOrder = false;
            }
            placeOrderEntriesFromSubtable(table, go, ignoreGlyphOrder);
        }
    }
    orderGlyphs(go);
    return go;
}
struct JsonReader;
impl FontBuilder for JsonReader {
    unsafe fn read(
    mut _root: *mut ::core::ffi::c_void,
    mut _index: u32,
    options: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let options = options as *const otfcc_Options;
    let mut root: *const json_value = _root as *mut json_value;
    let mut font: *mut otfcc_Font = (
        otfcc_iFont.create.expect("non-null function pointer"))();
    if font.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*font).subtype = otfcc_decideFontSubtypeFromJson(root);
    (*font).glyph_order = parseGlyphOrder(root, options);
    (*font).glyf = otfcc_parseGlyf(root, (*font).glyph_order, options);
    (*font).CFF_ = otfcc_parseCFF(root, options);
    (*font).head = otfcc_parseHead(root, options);
    (*font).hhea = otfcc_parseHhea(root, options);
    (*font).OS_2 = otfcc_parseOS_2(root, options);
    (*font).maxp = otfcc_parseMaxp(root, options);
    (*font).post = otfcc_parsePost(root, options);
    (*font).name = otfcc_parseName(root, options);
    (*font).meta = otfcc_parseMeta(root, options);
    (*font).cmap = otfcc_parseCmap(root, options);
    if !(*options).ignore_hints {
        (*font).fpgm = otfcc_parseFpgmPrep(
            root,
            options,
            b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).prep = otfcc_parseFpgmPrep(
            root,
            options,
            b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).cvt_ = otfcc_parseCvt(
            root,
            options,
            b"cvt_\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).gasp = otfcc_parseGasp(root, options);
    }
    (*font).VDMX = otfcc_parseVDMX(root, options);
    (*font).vhea = otfcc_parseVhea(root, options);
    if !(*font).glyf.is_null() {
        (*font).GSUB = otfcc_parseOtl(
            root,
            options,
            b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).GPOS = otfcc_parseOtl(
            root,
            options,
            b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).GDEF = otfcc_parseGDEF(root, options);
    }
    (*font).BASE = otfcc_parseBASE(root, options);
    (*font).CPAL = otfcc_parseCPAL(root, options);
    (*font).COLR = otfcc_parseCOLR(root, options);
    (*font).SVG_ = otfcc_parseSVG(root, options);
    (*font).TSI_01 = otfcc_parseTSI(
        root,
        options,
        b"TSI_01\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*font).TSI_23 = otfcc_parseTSI(
        root,
        options,
        b"TSI_23\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*font).TSI5 = otfcc_parseTSI5(root, options);
    return font as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn readJson(
    mut _root: *mut ::core::ffi::c_void,
    mut _index: u32,
    mut options: *const otfcc_Options,
) -> *mut otfcc_Font {
    <JsonReader as FontBuilder>::read(_root, _index, options as *const ::core::ffi::c_void)
        as *mut otfcc_Font
}
#[inline]
unsafe extern "C" fn freeReader(mut self_0: *mut otfcc_IFontBuilder) {
    free(self_0 as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_newJsonReader() -> *mut otfcc_IFontBuilder {
    let mut reader: *mut otfcc_IFontBuilder = ::core::ptr::null_mut::<otfcc_IFontBuilder>();
    reader = __caryll_allocate_clean(
        ::core::mem::size_of::<otfcc_IFontBuilder>() as usize,
        177 as ::core::ffi::c_ulong,
    ) as *mut otfcc_IFontBuilder;
    (*reader).read = Some(
        readJson
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                u32,
                *const otfcc_Options,
            ) -> *mut otfcc_Font,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                u32,
                *const otfcc_Options,
            ) -> *mut otfcc_Font,
        >;
    (*reader).free = Some(freeReader as unsafe extern "C" fn(*mut otfcc_IFontBuilder) -> ())
        as Option<unsafe extern "C" fn(*mut otfcc_IFontBuilder) -> ()>;
    return reader;
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
