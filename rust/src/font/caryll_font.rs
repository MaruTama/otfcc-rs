use libc::{free, malloc, memcpy, memset};
extern "C" {
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static otl_iClassDef: __otfcc_IClassDef;
    static table_iHead: __caryll_elementinterface_table_head;
    static table_iHhea: __caryll_elementinterface_table_hhea;
    static table_iMaxp: __caryll_elementinterface_table_maxp;
    static table_iOS_2: __caryll_elementinterface_table_OS_2;
    static table_iHmtx: __caryll_elementinterface_table_hmtx;
    static iTable_post: __caryll_elementinterface_table_post;
    static table_iVhea: __caryll_elementinterface_table_vhea;
    static table_iVORG: __caryll_elementinterface_table_VORG;
    static table_iGasp: __caryll_elementinterface_table_gasp;
    static table_iVmtx: __caryll_elementinterface_table_vmtx;
    static table_iGlyf: __caryll_vectorinterface_table_glyf;
    static table_iName: __caryll_vectorinterface_table_name;
    static table_iMeta: __caryll_elementinterface_table_meta;
    static table_iFpgm_prep: __caryll_elementinterface_table_fpgm_prep;
    static table_iCFF: __caryll_elementinterface_table_CFF;
    static table_iCmap: __caryll_elementinterface_table_cmap;
    static table_iOTL: __caryll_elementinterface_table_OTL;
    static table_iGDEF: __caryll_elementinterface_table_GDEF;
    static table_iLTSH: __caryll_elementinterface_table_LTSH;
    static table_iCPAL: __caryll_elementinterface_table_CPAL;
    static table_iBASE: __caryll_elementinterface_table_BASE;
    static table_iCOLR: __caryll_vectorinterface_table_COLR;
    static table_iSVG: __caryll_vectorinterface_table_SVG;
    static table_iTSI: __caryll_vectorinterface_table_TSI;
    static table_iCvt: __caryll_elementinterface_table_cvt;
    fn otfcc_consolidateFont(font: *mut otfcc_Font, options: *const otfcc_Options);
}
use crate::table::otl::classdef::{otl_ClassDef_free, otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{arity_t, colorid_t, f16dot16, glyphclass_t, glyphid_t, glyphsize_t, length_t, pos_t, scale_t, shapeid_t, tableid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_value};
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
pub struct __otfcc_IClassDef {
    pub init: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_ClassDef, *const otl_ClassDef) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_ClassDef, *mut otl_ClassDef) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_ClassDef>,
    pub free: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut otl_ClassDef, otfcc_GlyphHandle, glyphclass_t) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_ClassDef>,
    pub expand:
        Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_ClassDef) -> *mut otl_ClassDef>,
    pub dump: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_ClassDef>,
    pub build: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
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
    pub c2rust_unnamed: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub struct __caryll_elementinterface_table_head {
    pub init: Option<unsafe extern "C" fn(*mut table_head) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_head, *const table_head) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_head, *mut table_head) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_head) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_head, table_head) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_head, table_head) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_head>,
    pub free: Option<unsafe extern "C" fn(*mut table_head) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_maxp {
    pub init: Option<unsafe extern "C" fn(*mut table_maxp) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_maxp, *const table_maxp) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_maxp, *mut table_maxp) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_maxp) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_maxp, table_maxp) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_maxp, table_maxp) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_maxp>,
    pub free: Option<unsafe extern "C" fn(*mut table_maxp) -> ()>,
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
    pub initN: Option<unsafe extern "C" fn(*mut table_glyf, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_glyf, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut table_glyf>,
    pub fill: Option<unsafe extern "C" fn(*mut table_glyf, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_glyf, glyf_GlyphPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_glyf) -> glyf_GlyphPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_glyf, usize) -> ()>,
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
pub struct __caryll_elementinterface_table_hhea {
    pub init: Option<unsafe extern "C" fn(*mut table_hhea) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_hhea, *const table_hhea) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_hhea, *mut table_hhea) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_hhea) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_hhea, table_hhea) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_hhea, table_hhea) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_hhea>,
    pub free: Option<unsafe extern "C" fn(*mut table_hhea) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_hmtx {
    pub init: Option<unsafe extern "C" fn(*mut table_hmtx) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_hmtx, *const table_hmtx) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_hmtx, *mut table_hmtx) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_hmtx) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_hmtx, table_hmtx) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_hmtx, table_hmtx) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_hmtx>,
    pub free: Option<unsafe extern "C" fn(*mut table_hmtx) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_vhea {
    pub init: Option<unsafe extern "C" fn(*mut table_vhea) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_vhea, *const table_vhea) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_vhea, *mut table_vhea) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_vhea) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_vhea, table_vhea) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_vhea, table_vhea) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_vhea>,
    pub free: Option<unsafe extern "C" fn(*mut table_vhea) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_vmtx {
    pub init: Option<unsafe extern "C" fn(*mut table_vmtx) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_vmtx, *const table_vmtx) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_vmtx, *mut table_vmtx) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_vmtx) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_vmtx, table_vmtx) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_vmtx, table_vmtx) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_vmtx>,
    pub free: Option<unsafe extern "C" fn(*mut table_vmtx) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_OS_2 {
    pub init: Option<unsafe extern "C" fn(*mut table_OS_2) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_OS_2, *const table_OS_2) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_OS_2, *mut table_OS_2) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_OS_2) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_OS_2, table_OS_2) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_OS_2, table_OS_2) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_OS_2>,
    pub free: Option<unsafe extern "C" fn(*mut table_OS_2) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_post {
    pub init: Option<unsafe extern "C" fn(*mut table_post) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_post, *const table_post) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_post, *mut table_post) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_post) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_post, table_post) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_post, table_post) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_post>,
    pub free: Option<unsafe extern "C" fn(*mut table_post) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_table_name {
    pub init: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_name, *const table_name) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_name, *mut table_name) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_name, table_name) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_name, table_name) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_name>,
    pub free: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_name, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_name, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut table_name>,
    pub fill: Option<unsafe extern "C" fn(*mut table_name, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_name, otfcc_NameRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_name) -> otfcc_NameRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_name, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_name,
            Option<unsafe extern "C" fn(*const otfcc_NameRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_name,
            Option<
                unsafe extern "C" fn(
                    *const otfcc_NameRecord,
                    *const otfcc_NameRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_meta {
    pub init: Option<unsafe extern "C" fn(*mut table_meta) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_meta, *const table_meta) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_meta, *mut table_meta) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_meta) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_meta, table_meta) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_meta, table_meta) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_meta>,
    pub free: Option<unsafe extern "C" fn(*mut table_meta) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_cmap {
    pub init: Option<unsafe extern "C" fn(*mut table_cmap) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_cmap, *const table_cmap) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_cmap, *mut table_cmap) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_cmap) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_cmap, table_cmap) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_cmap, table_cmap) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_cmap>,
    pub free: Option<unsafe extern "C" fn(*mut table_cmap) -> ()>,
    pub encodeByIndex:
        Option<unsafe extern "C" fn(*mut table_cmap, ::core::ffi::c_int, u16) -> bool>,
    pub encodeByName:
        Option<unsafe extern "C" fn(*mut table_cmap, ::core::ffi::c_int, sds) -> bool>,
    pub unmap: Option<unsafe extern "C" fn(*mut table_cmap, ::core::ffi::c_int) -> bool>,
    pub lookup: Option<
        unsafe extern "C" fn(*const table_cmap, ::core::ffi::c_int) -> *mut otfcc_GlyphHandle,
    >,
    pub encodeUVSByIndex:
        Option<unsafe extern "C" fn(*mut table_cmap, cmap_UVS_key, u16) -> bool>,
    pub encodeUVSByName: Option<unsafe extern "C" fn(*mut table_cmap, cmap_UVS_key, sds) -> bool>,
    pub unmapUVS: Option<unsafe extern "C" fn(*mut table_cmap, cmap_UVS_key) -> bool>,
    pub lookupUVS:
        Option<unsafe extern "C" fn(*const table_cmap, cmap_UVS_key) -> *mut otfcc_GlyphHandle>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_cvt {
    pub init: Option<unsafe extern "C" fn(*mut table_cvt) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_cvt, *const table_cvt) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_cvt, *mut table_cvt) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_cvt) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_cvt, table_cvt) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_cvt, table_cvt) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_cvt>,
    pub free: Option<unsafe extern "C" fn(*mut table_cvt) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_fpgm_prep {
    pub init: Option<unsafe extern "C" fn(*mut table_fpgm_prep) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_fpgm_prep, *const table_fpgm_prep) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_fpgm_prep, *mut table_fpgm_prep) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_fpgm_prep) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_fpgm_prep, table_fpgm_prep) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_fpgm_prep, table_fpgm_prep) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_fpgm_prep>,
    pub free: Option<unsafe extern "C" fn(*mut table_fpgm_prep) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_gasp {
    pub init: Option<unsafe extern "C" fn(*mut table_gasp) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_gasp, *const table_gasp) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_gasp, *mut table_gasp) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_gasp) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_gasp, table_gasp) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_gasp, table_gasp) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_gasp>,
    pub free: Option<unsafe extern "C" fn(*mut table_gasp) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_LTSH {
    pub init: Option<unsafe extern "C" fn(*mut table_LTSH) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_LTSH, *const table_LTSH) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_LTSH, *mut table_LTSH) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_LTSH) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_LTSH, table_LTSH) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_LTSH, table_LTSH) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_LTSH>,
    pub free: Option<unsafe extern "C" fn(*mut table_LTSH) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_VORG {
    pub init: Option<unsafe extern "C" fn(*mut table_VORG) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_VORG, *const table_VORG) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_VORG, *mut table_VORG) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_VORG) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_VORG, table_VORG) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_VORG, table_VORG) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_VORG>,
    pub free: Option<unsafe extern "C" fn(*mut table_VORG) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_OTL {
    pub init: Option<unsafe extern "C" fn(*mut table_OTL) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_OTL, *const table_OTL) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_OTL, *mut table_OTL) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_OTL) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_OTL, table_OTL) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_OTL, table_OTL) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_OTL>,
    pub free: Option<unsafe extern "C" fn(*mut table_OTL) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_GDEF {
    pub init: Option<unsafe extern "C" fn(*mut table_GDEF) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_GDEF, *const table_GDEF) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_GDEF, *mut table_GDEF) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_GDEF) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_GDEF, table_GDEF) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_GDEF, table_GDEF) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_GDEF>,
    pub free: Option<unsafe extern "C" fn(*mut table_GDEF) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_BASE {
    pub init: Option<unsafe extern "C" fn(*mut table_BASE) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_BASE, *const table_BASE) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_BASE, *mut table_BASE) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_BASE) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_BASE, table_BASE) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_BASE, table_BASE) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_BASE>,
    pub free: Option<unsafe extern "C" fn(*mut table_BASE) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_CPAL {
    pub init: Option<unsafe extern "C" fn(*mut table_CPAL) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_CPAL, *const table_CPAL) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_CPAL, *mut table_CPAL) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_CPAL) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_CPAL, table_CPAL) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_CPAL, table_CPAL) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_CPAL>,
    pub free: Option<unsafe extern "C" fn(*mut table_CPAL) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_table_COLR {
    pub init: Option<unsafe extern "C" fn(*mut table_COLR) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_COLR, *const table_COLR) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_COLR, *mut table_COLR) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_COLR) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_COLR, table_COLR) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_COLR, table_COLR) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_COLR>,
    pub free: Option<unsafe extern "C" fn(*mut table_COLR) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_COLR, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_COLR, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut table_COLR>,
    pub fill: Option<unsafe extern "C" fn(*mut table_COLR, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_COLR) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_COLR, colr_Mapping) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_COLR) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_COLR) -> colr_Mapping>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_COLR, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_COLR,
            Option<unsafe extern "C" fn(*const colr_Mapping, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_COLR,
            Option<
                unsafe extern "C" fn(
                    *const colr_Mapping,
                    *const colr_Mapping,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_table_SVG {
    pub init: Option<unsafe extern "C" fn(*mut table_SVG) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_SVG, *const table_SVG) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_SVG, *mut table_SVG) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_SVG) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_SVG, table_SVG) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_SVG, table_SVG) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_SVG>,
    pub free: Option<unsafe extern "C" fn(*mut table_SVG) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_SVG, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_SVG, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut table_SVG>,
    pub fill: Option<unsafe extern "C" fn(*mut table_SVG, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_SVG) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_SVG, svg_Assignment) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_SVG) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_SVG) -> svg_Assignment>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_SVG, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_SVG,
            Option<unsafe extern "C" fn(*const svg_Assignment, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_SVG,
            Option<
                unsafe extern "C" fn(
                    *const svg_Assignment,
                    *const svg_Assignment,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_table_TSI {
    pub init: Option<unsafe extern "C" fn(*mut table_TSI) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_TSI, *const table_TSI) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_TSI, *mut table_TSI) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_TSI) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_TSI, table_TSI) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_TSI, table_TSI) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_TSI>,
    pub free: Option<unsafe extern "C" fn(*mut table_TSI) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_TSI, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_TSI, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut table_TSI>,
    pub fill: Option<unsafe extern "C" fn(*mut table_TSI, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_TSI) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_TSI, tsi_Entry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_TSI) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_TSI) -> tsi_Entry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_TSI, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_TSI,
            Option<unsafe extern "C" fn(*const tsi_Entry, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_TSI,
            Option<unsafe extern "C" fn(*const tsi_Entry, *const tsi_Entry) -> ::core::ffi::c_int>,
        ) -> (),
    >,
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
        Option<unsafe extern "C" fn(*mut otfcc_Font, u32) -> *mut ::core::ffi::c_void>,
    pub deleteTable: Option<unsafe extern "C" fn(*mut otfcc_Font, u32) -> ()>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn createFontTable(
    mut _font: *mut otfcc_Font,
    tag: u32,
) -> *mut ::core::ffi::c_void {
    match tag {
        1851878757 => {
            return (
                table_iName.create.expect("non-null function pointer"))() as *mut ::core::ffi::c_void;
        }
        1196643650 | 1196445523 => {
            return (
                table_iOTL.create.expect("non-null function pointer"))() as *mut ::core::ffi::c_void;
        }
        _ => return NULL,
    };
}
unsafe extern "C" fn deleteFontTable(mut font: *mut otfcc_Font, tag: u32) {
    match tag {
        1751474532 => {
            if !(*font).head.is_null() {
                table_iHead.free.expect("non-null function pointer")((*font).head);
                (*font).head = ::core::ptr::null_mut::<table_head>();
            }
            return;
        }
        1751672161 => {
            if !(*font).hhea.is_null() {
                table_iHhea.free.expect("non-null function pointer")((*font).hhea);
                (*font).hhea = ::core::ptr::null_mut::<table_hhea>();
            }
            return;
        }
        1835104368 => {
            if !(*font).maxp.is_null() {
                table_iMaxp.free.expect("non-null function pointer")((*font).maxp);
                (*font).maxp = ::core::ptr::null_mut::<table_maxp>();
            }
            return;
        }
        1330863922 | 1330851634 => {
            if !(*font).OS_2.is_null() {
                table_iOS_2.free.expect("non-null function pointer")((*font).OS_2);
                (*font).OS_2 = ::core::ptr::null_mut::<table_OS_2>();
            }
            return;
        }
        1851878757 => {
            if !(*font).name.is_null() {
                table_iName.free.expect("non-null function pointer")((*font).name);
                (*font).name = ::core::ptr::null_mut::<table_name>();
            }
            return;
        }
        1835365473 => {
            if !(*font).meta.is_null() {
                table_iMeta.free.expect("non-null function pointer")((*font).meta);
                (*font).meta = ::core::ptr::null_mut::<table_meta>();
            }
            return;
        }
        1752003704 => {
            if !(*font).hmtx.is_null() {
                table_iHmtx.free.expect("non-null function pointer")((*font).hmtx);
                (*font).hmtx = ::core::ptr::null_mut::<table_hmtx>();
            }
            return;
        }
        1986884728 => {
            if !(*font).vmtx.is_null() {
                table_iVmtx.free.expect("non-null function pointer")((*font).vmtx);
                (*font).vmtx = ::core::ptr::null_mut::<table_vmtx>();
            }
            return;
        }
        1886352244 => {
            if !(*font).post.is_null() {
                iTable_post.free.expect("non-null function pointer")((*font).post);
                (*font).post = ::core::ptr::null_mut::<table_post>();
            }
            return;
        }
        1986553185 => {
            if !(*font).vhea.is_null() {
                table_iVhea.free.expect("non-null function pointer")((*font).vhea);
                (*font).vhea = ::core::ptr::null_mut::<table_vhea>();
            }
            return;
        }
        1718642541 => {
            if !(*font).fpgm.is_null() {
                table_iFpgm_prep.free.expect("non-null function pointer")((*font).fpgm);
                (*font).fpgm = ::core::ptr::null_mut::<table_fpgm_prep>();
            }
            return;
        }
        1886545264 => {
            if !(*font).prep.is_null() {
                table_iFpgm_prep.free.expect("non-null function pointer")((*font).prep);
                (*font).prep = ::core::ptr::null_mut::<table_fpgm_prep>();
            }
            return;
        }
        1668707423 | 1668707360 => {
            if !(*font).cvt_.is_null() {
                table_iCvt.free.expect("non-null function pointer")((*font).cvt_);
                (*font).cvt_ = ::core::ptr::null_mut::<table_cvt>();
            }
            return;
        }
        1734439792 => {
            if !(*font).gasp.is_null() {
                table_iGasp.free.expect("non-null function pointer")((*font).gasp);
                (*font).gasp = ::core::ptr::null_mut::<table_gasp>();
            }
            return;
        }
        1128679007 | 1128678944 => {
            if !(*font).CFF_.is_null() {
                table_iCFF.free.expect("non-null function pointer")((*font).CFF_);
                (*font).CFF_ = ::core::ptr::null_mut::<table_CFF>();
            }
            return;
        }
        1735162214 => {
            if !(*font).glyf.is_null() {
                table_iGlyf.free.expect("non-null function pointer")((*font).glyf);
                (*font).glyf = ::core::ptr::null_mut::<table_glyf>();
            }
            return;
        }
        1668112752 => {
            if !(*font).cmap.is_null() {
                table_iCmap.free.expect("non-null function pointer")((*font).cmap);
                (*font).cmap = ::core::ptr::null_mut::<table_cmap>();
            }
            return;
        }
        1280594760 => {
            if !(*font).LTSH.is_null() {
                table_iLTSH.free.expect("non-null function pointer")((*font).LTSH);
                (*font).LTSH = ::core::ptr::null_mut::<table_LTSH>();
            }
            return;
        }
        1196643650 => {
            if !(*font).GSUB.is_null() {
                table_iOTL.free.expect("non-null function pointer")((*font).GSUB);
                (*font).GSUB = ::core::ptr::null_mut::<table_OTL>();
            }
            return;
        }
        1196445523 => {
            if !(*font).GPOS.is_null() {
                table_iOTL.free.expect("non-null function pointer")((*font).GPOS);
                (*font).GPOS = ::core::ptr::null_mut::<table_OTL>();
            }
            return;
        }
        1195656518 => {
            if !(*font).GDEF.is_null() {
                table_iGDEF.free.expect("non-null function pointer")((*font).GDEF);
                (*font).GDEF = ::core::ptr::null_mut::<table_GDEF>();
            }
            return;
        }
        1111577413 => {
            if !(*font).BASE.is_null() {
                table_iBASE.free.expect("non-null function pointer")((*font).BASE);
                (*font).BASE = ::core::ptr::null_mut::<table_BASE>();
            }
            return;
        }
        1448038983 => {
            if !(*font).VORG.is_null() {
                table_iVORG.free.expect("non-null function pointer")((*font).VORG);
                (*font).VORG = ::core::ptr::null_mut::<table_VORG>();
            }
            return;
        }
        1129333068 => {
            if !(*font).CPAL.is_null() {
                table_iCPAL.free.expect("non-null function pointer")((*font).CPAL);
                (*font).CPAL = ::core::ptr::null_mut::<table_CPAL>();
            }
            return;
        }
        1129270354 => {
            if !(*font).COLR.is_null() {
                table_iCOLR.free.expect("non-null function pointer")((*font).COLR);
                (*font).COLR = ::core::ptr::null_mut::<table_COLR>();
            }
            return;
        }
        1398163232 | 1398163295 => {
            if !(*font).SVG_.is_null() {
                table_iSVG.free.expect("non-null function pointer")((*font).SVG_);
                (*font).SVG_ = ::core::ptr::null_mut::<table_SVG>();
            }
            return;
        }
        1414744368 | 1414744369 => {
            if !(*font).TSI_01.is_null() {
                table_iTSI.free.expect("non-null function pointer")((*font).TSI_01);
                (*font).TSI_01 = ::core::ptr::null_mut::<table_TSI>();
            }
            return;
        }
        1414744370 | 1414744371 => {
            if !(*font).TSI_23.is_null() {
                table_iTSI.free.expect("non-null function pointer")((*font).TSI_23);
                (*font).TSI_23 = ::core::ptr::null_mut::<table_TSI>();
            }
            return;
        }
        1414744373 => {
            if !(*font).TSI5.is_null() {
                otl_ClassDef_free(
                    (*font).TSI5 as *mut otl_ClassDef,
                );
                (*font).TSI5 = ::core::ptr::null_mut::<table_TSI5>();
            }
            return;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn initFont(mut font: *mut otfcc_Font) {
    memset(
        font as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<otfcc_Font>() as usize,
    );
}
#[inline]
unsafe extern "C" fn disposeFont(mut font: *mut otfcc_Font) {
    deleteFontTable(font, 1751474532i32 as u32);
    deleteFontTable(font, 1751672161i32 as u32);
    deleteFontTable(font, 1835104368i32 as u32);
    deleteFontTable(font, 1330863922i32 as u32);
    deleteFontTable(font, 1851878757i32 as u32);
    deleteFontTable(font, 1835365473i32 as u32);
    deleteFontTable(font, 1752003704i32 as u32);
    deleteFontTable(font, 1986884728i32 as u32);
    deleteFontTable(font, 1886352244i32 as u32);
    deleteFontTable(font, 1751412088i32 as u32);
    deleteFontTable(font, 1986553185i32 as u32);
    deleteFontTable(font, 1718642541i32 as u32);
    deleteFontTable(font, 1886545264i32 as u32);
    deleteFontTable(font, 1668707423i32 as u32);
    deleteFontTable(font, 1734439792i32 as u32);
    deleteFontTable(font, 1128679007i32 as u32);
    deleteFontTable(font, 1735162214i32 as u32);
    deleteFontTable(font, 1668112752i32 as u32);
    deleteFontTable(font, 1280594760i32 as u32);
    deleteFontTable(font, 1196643650i32 as u32);
    deleteFontTable(font, 1196445523i32 as u32);
    deleteFontTable(font, 1195656518i32 as u32);
    deleteFontTable(font, 1111577413i32 as u32);
    deleteFontTable(font, 1448038983i32 as u32);
    deleteFontTable(font, 1129333068i32 as u32);
    deleteFontTable(font, 1129270354i32 as u32);
    deleteFontTable(font, 1398163295i32 as u32);
    deleteFontTable(font, 1414744368i32 as u32);
    deleteFontTable(font, 1414744370i32 as u32);
    deleteFontTable(font, 1414744373i32 as u32);
    otfcc_pkgGlyphOrder.free.expect("non-null function pointer")((*font).glyph_order);
}
#[inline]
unsafe extern "C" fn otfcc_Font_dispose(mut x: *mut otfcc_Font) {
    disposeFont(x);
}
#[inline]
unsafe extern "C" fn otfcc_Font_create() -> *mut otfcc_Font {
    let mut x: *mut otfcc_Font =
        malloc(::core::mem::size_of::<otfcc_Font>() as usize) as *mut otfcc_Font;
    otfcc_Font_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otfcc_Font_init(mut x: *mut otfcc_Font) {
    initFont(x);
}
#[inline]
unsafe extern "C" fn otfcc_Font_free(mut x: *mut otfcc_Font) {
    if x.is_null() {
        return;
    }
    otfcc_Font_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otfcc_Font_copyReplace(mut dst: *mut otfcc_Font, src: otfcc_Font) {
    otfcc_Font_dispose(dst);
    otfcc_Font_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otfcc_Font_copy(mut dst: *mut otfcc_Font, mut src: *const otfcc_Font) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_Font>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_Font_replace(mut dst: *mut otfcc_Font, src: otfcc_Font) {
    otfcc_Font_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_Font>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_Font_move(mut dst: *mut otfcc_Font, mut src: *mut otfcc_Font) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_Font>() as usize,
    );
    otfcc_Font_init(src);
}
#[no_mangle]
pub static mut otfcc_iFont: __caryll_elementinterface_otfcc_Font = {
    __caryll_elementinterface_otfcc_Font {
        init: Some(otfcc_Font_init as unsafe extern "C" fn(*mut otfcc_Font) -> ()),
        copy: Some(
            otfcc_Font_copy as unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Font) -> (),
        ),
        move_0: Some(
            otfcc_Font_move as unsafe extern "C" fn(*mut otfcc_Font, *mut otfcc_Font) -> (),
        ),
        dispose: Some(otfcc_Font_dispose as unsafe extern "C" fn(*mut otfcc_Font) -> ()),
        replace: Some(
            otfcc_Font_replace as unsafe extern "C" fn(*mut otfcc_Font, otfcc_Font) -> (),
        ),
        copyReplace: Some(
            otfcc_Font_copyReplace as unsafe extern "C" fn(*mut otfcc_Font, otfcc_Font) -> (),
        ),
        create: Some(otfcc_Font_create),
        free: Some(otfcc_Font_free as unsafe extern "C" fn(*mut otfcc_Font) -> ()),
        consolidate: Some(
            otfcc_consolidateFont
                as unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Options) -> (),
        ),
        createTable: Some(
            createFontTable
                as unsafe extern "C" fn(*mut otfcc_Font, u32) -> *mut ::core::ffi::c_void,
        ),
        deleteTable: Some(deleteFontTable as unsafe extern "C" fn(*mut otfcc_Font, u32) -> ()),
    }
};
