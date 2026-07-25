extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static glyf_iComponentReference: __caryll_elementinterface_glyf_ComponentReference;
    static iVQ: __caryll_vectorinterface_VQ;
    static otfcc_iFont: __caryll_elementinterface_otfcc_Font;
    fn time(__timer: *mut time_t) -> time_t;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{handle_fromIndex, otfcc_Handle_replace, otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle, HANDLE_STATE_EMPTY};
use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __time_t = ::core::ffi::c_long;
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
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
    pub val: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub still: pos_t,
    pub delta: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub type time_t = __time_t;
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
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed_3 = 10;
pub const log_vl_info: C2RustUnnamed_3 = 5;
pub const log_vl_notice: C2RustUnnamed_3 = 2;
pub const log_vl_important: C2RustUnnamed_3 = 1;
pub const log_vl_critical: C2RustUnnamed_3 = 0;
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
pub struct __caryll_elementinterface_glyf_ComponentReference {
    pub init: Option<unsafe extern "C" fn(*mut glyf_ComponentReference) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut glyf_ComponentReference, *const glyf_ComponentReference) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut glyf_ComponentReference, *mut glyf_ComponentReference) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_ComponentReference) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut glyf_ComponentReference, glyf_ComponentReference) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut glyf_ComponentReference, glyf_ComponentReference) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> glyf_ComponentReference>,
    pub dup: Option<unsafe extern "C" fn(glyf_ComponentReference) -> glyf_ComponentReference>,
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
pub type stat_status = ::core::ffi::c_uint;
pub const stat_completed: stat_status = 2;
pub const stat_doing: stat_status = 1;
pub const stat_not_started: stat_status = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POS_MAX: ::core::ffi::c_float = FLT_MAX;
#[no_mangle]
pub unsafe extern "C" fn stat_single_glyph(
    mut table: *mut table_glyf,
    mut gr: *mut glyf_ComponentReference,
    mut stated: *mut stat_status,
    mut depth: uint8_t,
    mut topj: glyphid_t,
    mut options: *const otfcc_Options,
) -> glyf_GlyphStat {
    let mut stat: glyf_GlyphStat = glyf_GlyphStat {
        xMin: 0 as ::core::ffi::c_int as pos_t,
        xMax: 0 as ::core::ffi::c_int as pos_t,
        yMin: 0 as ::core::ffi::c_int as pos_t,
        yMax: 0 as ::core::ffi::c_int as pos_t,
        nestDepth: 0 as uint16_t,
        nPoints: 0 as uint16_t,
        nContours: 0 as uint16_t,
        nCompositePoints: 0 as uint16_t,
        nCompositeContours: 0 as uint16_t,
    };
    let j: glyphid_t = (*gr).glyph.index;
    if depth as ::core::ffi::c_int >= 0xff as ::core::ffi::c_int {
        return stat;
    }
    if *stated.offset(j as isize) == stat_doing {
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
                b"[Stat] Circular glyph reference found in gid %d to gid %d. The reference will be dropped.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                topj as ::core::ffi::c_int,
                j as ::core::ffi::c_int,
            ),
        );
        *stated.offset(j as isize) = stat_completed;
        return stat;
    }
    let g: *mut glyf_Glyph = *(*table).items.offset(j as isize) as *mut glyf_Glyph;
    *stated.offset(j as isize) = stat_doing;
    let mut xmin: pos_t = POS_MAX as pos_t;
    let mut xmax: pos_t = -POS_MAX as pos_t;
    let mut ymin: pos_t = POS_MAX as pos_t;
    let mut ymax: pos_t = -POS_MAX as pos_t;
    let mut nestDepth: uint16_t = 0 as uint16_t;
    let mut nPoints: uint16_t = 0 as uint16_t;
    let mut nCompositePoints: uint16_t = 0 as uint16_t;
    let mut nCompositeContours: uint16_t = 0 as uint16_t;
    for c in 0..(*g).contours.length as shapeid_t {
        let contour = (*g).contours.items.offset(c as isize);
        for pj in 0..(*contour).length as shapeid_t {
            let p: *mut glyf_Point = (*contour).items.offset(pj as isize) as *mut glyf_Point;
            let x: pos_t = round(
                iVQ.getStill.expect("non-null function pointer")((*gr).x) as ::core::ffi::c_double
                    + (*gr).a as ::core::ffi::c_double
                        * iVQ.getStill.expect("non-null function pointer")((*p).x)
                            as ::core::ffi::c_double
                    + (*gr).b as ::core::ffi::c_double
                        * iVQ.getStill.expect("non-null function pointer")((*p).y)
                            as ::core::ffi::c_double,
            ) as pos_t;
            let mut y: pos_t = round(
                iVQ.getStill.expect("non-null function pointer")((*gr).y) as ::core::ffi::c_double
                    + (*gr).c as ::core::ffi::c_double
                        * iVQ.getStill.expect("non-null function pointer")((*p).x)
                            as ::core::ffi::c_double
                    + (*gr).d as ::core::ffi::c_double
                        * iVQ.getStill.expect("non-null function pointer")((*p).y)
                            as ::core::ffi::c_double,
            ) as pos_t;
            if x < xmin {
                xmin = x;
            }
            if x > xmax {
                xmax = x;
            }
            if y < ymin {
                ymin = y;
            }
            if y > ymax {
                ymax = y;
            }
            nPoints = (nPoints as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint16_t;
        }
    }
    nCompositePoints = nPoints;
    nCompositeContours = (*g).contours.length as uint16_t;
    for r in 0..(*g).references.length as shapeid_t {
        let mut ref_0: glyf_ComponentReference = glyf_ComponentReference {
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
            roundToGrid: false,
            useMyMetrics: false,
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            isAnchored: REF_XY,
            inner: 0,
            outer: 0,
        };
        glyf_iComponentReference
            .init
            .expect("non-null function pointer")(&raw mut ref_0);
        let rr: *mut glyf_ComponentReference =
            (*g).references.items.offset(r as isize) as *mut glyf_ComponentReference;
        otfcc_Handle_replace(
            &raw mut ref_0.glyph,
            handle_fromIndex((*rr).glyph.index)
                as otfcc_Handle,
        );
        ref_0.a = (*gr).a * (*rr).a + (*rr).b * (*gr).c;
        ref_0.b = (*rr).a * (*gr).b + (*rr).b * (*gr).d;
        ref_0.c = (*gr).a * (*rr).c + (*gr).c * (*rr).d;
        ref_0.d = (*gr).b * (*rr).c + (*rr).d * (*gr).d;
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.x,
            iVQ.createStill.expect("non-null function pointer")(
                iVQ.getStill.expect("non-null function pointer")((*rr).x)
                    + (*rr).a as pos_t * iVQ.getStill.expect("non-null function pointer")((*gr).x)
                    + (*rr).b as pos_t * iVQ.getStill.expect("non-null function pointer")((*gr).y),
            ) as VQ,
        );
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.y,
            iVQ.createStill.expect("non-null function pointer")(
                iVQ.getStill.expect("non-null function pointer")((*rr).y)
                    + (*rr).c as pos_t * iVQ.getStill.expect("non-null function pointer")((*gr).x)
                    + (*rr).d as pos_t * iVQ.getStill.expect("non-null function pointer")((*gr).y),
            ) as VQ,
        );
        let mut thatstat: glyf_GlyphStat = stat_single_glyph(
            table,
            &raw mut ref_0,
            stated,
            (depth as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
            topj,
            options,
        );
        if thatstat.xMin < xmin {
            xmin = thatstat.xMin;
        }
        if thatstat.xMax > xmax {
            xmax = thatstat.xMax;
        }
        if thatstat.yMin < ymin {
            ymin = thatstat.yMin;
        }
        if thatstat.yMax > ymax {
            ymax = thatstat.yMax;
        }
        if thatstat.nestDepth as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            > nestDepth as ::core::ffi::c_int
        {
            nestDepth =
                (thatstat.nestDepth as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint16_t;
        }
        nCompositePoints = (nCompositePoints as ::core::ffi::c_int
            + thatstat.nCompositePoints as ::core::ffi::c_int)
            as uint16_t;
        nCompositeContours = (nCompositeContours as ::core::ffi::c_int
            + thatstat.nCompositeContours as ::core::ffi::c_int)
            as uint16_t;
    }
    if xmin > xmax {
        xmax = 0 as ::core::ffi::c_int as pos_t;
        xmin = xmax;
    }
    if ymin > ymax {
        ymax = 0 as ::core::ffi::c_int as pos_t;
        ymin = ymax;
    }
    stat.xMin = xmin;
    stat.xMax = xmax;
    stat.yMin = ymin;
    stat.yMax = ymax;
    stat.nestDepth = nestDepth;
    stat.nPoints = nPoints;
    stat.nContours = (*g).contours.length as uint16_t;
    stat.nCompositePoints = nCompositePoints;
    stat.nCompositeContours = nCompositeContours;
    *stated.offset(j as isize) = stat_completed;
    return stat;
}
#[no_mangle]
pub unsafe extern "C" fn statGlyf(mut font: *mut otfcc_Font, mut options: *const otfcc_Options) {
    let mut stated: *mut stat_status = ::core::ptr::null_mut::<stat_status>();
    stated = __caryll_allocate_clean(
        (::core::mem::size_of::<stat_status>() as size_t).wrapping_mul((*(*font).glyf).length),
        99 as ::core::ffi::c_ulong,
    ) as *mut stat_status;
    let mut xmin: pos_t = 0xffffffff as ::core::ffi::c_uint as pos_t;
    let mut xmax: pos_t = (0xffffffff as ::core::ffi::c_uint).wrapping_neg() as pos_t;
    let mut ymin: pos_t = 0xffffffff as ::core::ffi::c_uint as pos_t;
    let mut ymax: pos_t = (0xffffffff as ::core::ffi::c_uint).wrapping_neg() as pos_t;
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let mut gr: glyf_ComponentReference = glyf_ComponentReference {
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
            roundToGrid: false,
            useMyMetrics: false,
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            isAnchored: REF_XY,
            inner: 0,
            outer: 0,
        };
        gr.glyph =
            handle_fromIndex(j) as otfcc_GlyphHandle;
        gr.x =
            iVQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as pos_t);
        gr.y =
            iVQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as pos_t);
        gr.a = 1 as ::core::ffi::c_int as scale_t;
        gr.b = 0 as ::core::ffi::c_int as scale_t;
        gr.c = 0 as ::core::ffi::c_int as scale_t;
        gr.d = 1 as ::core::ffi::c_int as scale_t;
        let ref mut fresh2 = (**(*(*font).glyf).items.offset(j as isize)).stat;
        *fresh2 = stat_single_glyph((*font).glyf, &raw mut gr, stated, 0 as uint8_t, j, options);
        let mut thatstat: glyf_GlyphStat = *fresh2;
        if thatstat.xMin < xmin {
            xmin = thatstat.xMin;
        }
        if thatstat.xMax > xmax {
            xmax = thatstat.xMax;
        }
        if thatstat.yMin < ymin {
            ymin = thatstat.yMin;
        }
        if thatstat.yMax > ymax {
            ymax = thatstat.yMax;
        }
    }
    (*(*font).head).xMin = xmin as int16_t;
    (*(*font).head).xMax = xmax as int16_t;
    (*(*font).head).yMin = ymin as int16_t;
    (*(*font).head).yMax = ymax as int16_t;
    free(stated as *mut ::core::ffi::c_void);
    stated = ::core::ptr::null_mut::<stat_status>();
}
#[no_mangle]
pub unsafe extern "C" fn statMaxp(mut font: *mut otfcc_Font) {
    let mut nestDepth: uint16_t = 0 as uint16_t;
    let mut nPoints: uint16_t = 0 as uint16_t;
    let mut nContours: uint16_t = 0 as uint16_t;
    let mut nComponents: uint16_t = 0 as uint16_t;
    let mut nCompositePoints: uint16_t = 0 as uint16_t;
    let mut nCompositeContours: uint16_t = 0 as uint16_t;
    let mut instSize: uint16_t = 0 as uint16_t;
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph;
        if (*g).contours.length > 0 as size_t {
            if (*g).stat.nPoints as ::core::ffi::c_int > nPoints as ::core::ffi::c_int {
                nPoints = (*g).stat.nPoints;
            }
            if (*g).stat.nContours as ::core::ffi::c_int > nContours as ::core::ffi::c_int {
                nContours = (*g).stat.nContours;
            }
        } else if (*g).references.length > 0 as size_t {
            if (*g).stat.nCompositePoints as ::core::ffi::c_int
                > nCompositePoints as ::core::ffi::c_int
            {
                nCompositePoints = (*g).stat.nCompositePoints;
            }
            if (*g).stat.nCompositeContours as ::core::ffi::c_int
                > nCompositeContours as ::core::ffi::c_int
            {
                nCompositeContours = (*g).stat.nCompositeContours;
            }
            if (*g).stat.nestDepth as ::core::ffi::c_int > nestDepth as ::core::ffi::c_int {
                nestDepth = (*g).stat.nestDepth;
            }
            if (*g).references.length > nComponents as size_t {
                nComponents = (*g).references.length as uint16_t;
            }
        }
        if (*g).instructionsLength as ::core::ffi::c_int > instSize as ::core::ffi::c_int {
            instSize = (*g).instructionsLength;
        }
    }
    (*(*font).maxp).maxPoints = nPoints;
    (*(*font).maxp).maxContours = nContours;
    (*(*font).maxp).maxCompositePoints = nCompositePoints;
    (*(*font).maxp).maxCompositeContours = nCompositeContours;
    (*(*font).maxp).maxComponentDepth = nestDepth;
    (*(*font).maxp).maxComponentElements = nComponents;
    (*(*font).maxp).maxSizeOfInstructions = instSize;
}
unsafe extern "C" fn statHmtx(mut font: *mut otfcc_Font, mut _options: *const otfcc_Options) {
    if (*font).glyf.is_null() {
        return;
    }
    let mut hmtx: *mut table_hmtx = ::core::ptr::null_mut::<table_hmtx>();
    hmtx = __caryll_allocate_clean(
        ::core::mem::size_of::<table_hmtx>() as size_t,
        162 as ::core::ffi::c_ulong,
    ) as *mut table_hmtx;
    let mut count_a: glyphid_t = (*(*font).glyf).length as glyphid_t;
    let mut count_k: glyphid_t = 0 as glyphid_t;
    let mut lsbAtX_0: bool = true;
    if (*font).subtype != FONTTYPE_CFF {
        while count_a as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            && iVQ.getStill.expect("non-null function pointer")(
                (**(*(*font).glyf)
                    .items
                    .offset((count_a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                .advanceWidth,
            ) == iVQ.getStill.expect("non-null function pointer")(
                (**(*(*font).glyf)
                    .items
                    .offset((count_a as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize))
                .advanceWidth,
            )
        {
            count_a = count_a.wrapping_sub(1);
        }
        count_k = (*(*font).glyf).length.wrapping_sub(count_a as size_t) as glyphid_t;
    }
    (*hmtx).metrics = __caryll_allocate_clean(
        (::core::mem::size_of::<horizontal_metric>() as size_t).wrapping_mul(count_a as size_t),
        175 as ::core::ffi::c_ulong,
    ) as *mut horizontal_metric;
    (*hmtx).leftSideBearing = __caryll_allocate_clean(
        (::core::mem::size_of::<pos_t>() as size_t).wrapping_mul(count_k as size_t),
        176 as ::core::ffi::c_ulong,
    ) as *mut pos_t;
    let mut minLSB: pos_t = 0x7fff as ::core::ffi::c_int as pos_t;
    let mut minRSB: pos_t = 0x7fff as ::core::ffi::c_int as pos_t;
    let mut maxExtent: pos_t = -(0x8000 as ::core::ffi::c_int) as pos_t;
    let mut maxWidth: length_t = 0 as ::core::ffi::c_int as length_t;
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph;
        if iVQ.isZero.expect("non-null function pointer")((*g).horizontalOrigin, 1.0f64 / 1000.0f64)
        {
            iVQ.replace.expect("non-null function pointer")(
                &raw mut (*g).horizontalOrigin,
                (
                    iVQ.neutral.expect("non-null function pointer"))() as VQ,
            );
        } else {
            lsbAtX_0 = false;
        }
        let hori: pos_t =
            iVQ.getStill.expect("non-null function pointer")((*g).horizontalOrigin) as pos_t;
        let advw: pos_t =
            iVQ.getStill.expect("non-null function pointer")((*g).advanceWidth) as pos_t;
        let lsb: pos_t = (*g).stat.xMin - hori;
        let rsb: pos_t = advw + hori - (*g).stat.xMax;
        if (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            (*(*hmtx).metrics.offset(j as isize)).advanceWidth = advw as length_t;
            (*(*hmtx).metrics.offset(j as isize)).lsb = lsb;
        } else {
            *(*hmtx)
                .leftSideBearing
                .offset((j as ::core::ffi::c_int - count_a as ::core::ffi::c_int) as isize) = lsb;
        }
        if advw > maxWidth {
            maxWidth = advw as length_t;
        }
        if lsb < minLSB {
            minLSB = lsb;
        }
        if rsb < minRSB {
            minRSB = rsb;
        }
        if (*g).stat.xMax - hori > maxExtent {
            maxExtent = (*g).stat.xMax - hori;
        }
    }
    (*(*font).hhea).numberOfMetrics = count_a as uint16_t;
    (*(*font).hhea).minLeftSideBearing = minLSB as int16_t;
    (*(*font).hhea).minRightSideBearing = minRSB as int16_t;
    (*(*font).hhea).xMaxExtent = maxExtent as int16_t;
    (*(*font).hhea).advanceWidthMax = maxWidth as uint16_t;
    (*font).hmtx = hmtx;
    (*(*font).head).flags = ((*(*font).head).flags as ::core::ffi::c_int
        & !(0x2 as ::core::ffi::c_int)
        | (if lsbAtX_0 { 0x2 as ::core::ffi::c_int } else { 0 as ::core::ffi::c_int }))
        as uint16_t;
}
unsafe extern "C" fn statVmtx(mut font: *mut otfcc_Font, mut options: *const otfcc_Options) {
    if (*font).glyf.is_null() {
        return;
    }
    let mut vmtx: *mut table_vmtx = ::core::ptr::null_mut::<table_vmtx>();
    vmtx = __caryll_allocate_clean(
        ::core::mem::size_of::<table_vmtx>() as size_t,
        218 as ::core::ffi::c_ulong,
    ) as *mut table_vmtx;
    let mut count_a: glyphid_t = (*(*font).glyf).length as glyphid_t;
    let mut count_k: glyphid_t = 0 as glyphid_t;
    if !((*font).subtype == FONTTYPE_CFF && !(*options).cff_short_vmtx) {
        while count_a as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            && iVQ.getStill.expect("non-null function pointer")(
                (**(*(*font).glyf)
                    .items
                    .offset((count_a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                .advanceHeight,
            ) == iVQ.getStill.expect("non-null function pointer")(
                (**(*(*font).glyf)
                    .items
                    .offset((count_a as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize))
                .advanceHeight,
            )
        {
            count_a = count_a.wrapping_sub(1);
        }
        count_k = (*(*font).glyf).length.wrapping_sub(count_a as size_t) as glyphid_t;
    }
    (*vmtx).metrics = __caryll_allocate_clean(
        (::core::mem::size_of::<vertical_metric>() as size_t).wrapping_mul(count_a as size_t),
        230 as ::core::ffi::c_ulong,
    ) as *mut vertical_metric;
    (*vmtx).topSideBearing = __caryll_allocate_clean(
        (::core::mem::size_of::<pos_t>() as size_t).wrapping_mul(count_k as size_t),
        231 as ::core::ffi::c_ulong,
    ) as *mut pos_t;
    let mut minTSB: pos_t = 0x7fff as ::core::ffi::c_int as pos_t;
    let mut minBSB: pos_t = 0x7fff as ::core::ffi::c_int as pos_t;
    let mut maxExtent: pos_t = -(0x8000 as ::core::ffi::c_int) as pos_t;
    let mut maxHeight: length_t = 0 as ::core::ffi::c_int as length_t;
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph;
        let vori: pos_t =
            iVQ.getStill.expect("non-null function pointer")((*g).verticalOrigin) as pos_t;
        let advh: pos_t =
            iVQ.getStill.expect("non-null function pointer")((*g).advanceHeight) as pos_t;
        let tsb: pos_t = vori - (*g).stat.yMax;
        let bsb: pos_t = (*g).stat.yMin - vori + advh;
        if (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            (*(*vmtx).metrics.offset(j as isize)).advanceHeight = advh as length_t;
            (*(*vmtx).metrics.offset(j as isize)).tsb = tsb;
        } else {
            *(*vmtx)
                .topSideBearing
                .offset((j as ::core::ffi::c_int - count_a as ::core::ffi::c_int) as isize) = tsb;
        }
        if advh > maxHeight {
            maxHeight = advh as length_t;
        }
        if tsb < minTSB {
            minTSB = tsb;
        }
        if bsb < minBSB {
            minBSB = bsb;
        }
        if vori - (*g).stat.yMin > maxExtent {
            maxExtent = vori - (*g).stat.yMin;
        }
    }
    (*(*font).vhea).numOfLongVerMetrics = count_a as uint16_t;
    (*(*font).vhea).minTop = minTSB as int16_t;
    (*(*font).vhea).minBottom = minBSB as int16_t;
    (*(*font).vhea).yMaxExtent = maxExtent as int16_t;
    (*(*font).vhea).advanceHeightMax = maxHeight as int16_t;
    (*font).vmtx = vmtx;
}
unsafe extern "C" fn statOS_2UnicodeRanges(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    let mut item: *mut cmap_Entry = ::core::ptr::null_mut::<cmap_Entry>();
    let mut u1: uint32_t = 0 as uint32_t;
    let mut u2: uint32_t = 0 as uint32_t;
    let mut u3: uint32_t = 0 as uint32_t;
    let mut u4: uint32_t = 0 as uint32_t;
    let mut minUnicode: int32_t = 0xffff as int32_t;
    let mut maxUnicode: int32_t = 0 as int32_t;
    item = (*(*font).cmap).unicodes;
    while !item.is_null() {
        let mut u: ::core::ffi::c_int = (*item).unicode;
        if (u as int32_t) < minUnicode {
            minUnicode = u as int32_t;
        }
        if u as int32_t > maxUnicode {
            maxUnicode = u as int32_t;
        }
        if u >= 0 as ::core::ffi::c_int && u <= 0x7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x80 as ::core::ffi::c_int && u <= 0xff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x100 as ::core::ffi::c_int && u <= 0x17f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x180 as ::core::ffi::c_int && u <= 0x24f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x250 as ::core::ffi::c_int && u <= 0x2af as ::core::ffi::c_int
            || u >= 0x1d00 as ::core::ffi::c_int && u <= 0x1d7f as ::core::ffi::c_int
            || u >= 0x1d80 as ::core::ffi::c_int && u <= 0x1dbf as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2b0 as ::core::ffi::c_int && u <= 0x2ff as ::core::ffi::c_int
            || u >= 0xa700 as ::core::ffi::c_int && u <= 0xa71f as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x300 as ::core::ffi::c_int && u <= 0x36f as ::core::ffi::c_int
            || u >= 0x1dc0 as ::core::ffi::c_int && u <= 0x1dff as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x370 as ::core::ffi::c_int && u <= 0x3ff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2c80 as ::core::ffi::c_int && u <= 0x2cff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x400 as ::core::ffi::c_int && u <= 0x4ff as ::core::ffi::c_int
            || u >= 0x500 as ::core::ffi::c_int && u <= 0x52f as ::core::ffi::c_int
            || u >= 0x2de0 as ::core::ffi::c_int && u <= 0x2dff as ::core::ffi::c_int
            || u >= 0xa640 as ::core::ffi::c_int && u <= 0xa69f as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x530 as ::core::ffi::c_int && u <= 0x58f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x590 as ::core::ffi::c_int && u <= 0x5ff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xa500 as ::core::ffi::c_int && u <= 0xa63f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x600 as ::core::ffi::c_int && u <= 0x6ff as ::core::ffi::c_int
            || u >= 0x750 as ::core::ffi::c_int && u <= 0x77f as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x7c0 as ::core::ffi::c_int && u <= 0x7ff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x900 as ::core::ffi::c_int && u <= 0x97f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x980 as ::core::ffi::c_int && u <= 0x9ff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xa00 as ::core::ffi::c_int && u <= 0xa7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xa80 as ::core::ffi::c_int && u <= 0xaff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xb00 as ::core::ffi::c_int && u <= 0xb7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xb80 as ::core::ffi::c_int && u <= 0xbff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xc00 as ::core::ffi::c_int && u <= 0xc7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xc80 as ::core::ffi::c_int && u <= 0xcff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xd00 as ::core::ffi::c_int && u <= 0xd7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xe00 as ::core::ffi::c_int && u <= 0xe7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xe80 as ::core::ffi::c_int && u <= 0xeff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 25 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10a0 as ::core::ffi::c_int && u <= 0x10ff as ::core::ffi::c_int
            || u >= 0x2d00 as ::core::ffi::c_int && u <= 0x2d2f as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 26 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1b00 as ::core::ffi::c_int && u <= 0x1b7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1100 as ::core::ffi::c_int && u <= 0x11ff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1e00 as ::core::ffi::c_int && u <= 0x1eff as ::core::ffi::c_int
            || u >= 0x2c60 as ::core::ffi::c_int && u <= 0x2c7f as ::core::ffi::c_int
            || u >= 0xa720 as ::core::ffi::c_int && u <= 0xa7ff as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1f00 as ::core::ffi::c_int && u <= 0x1fff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2000 as ::core::ffi::c_int && u <= 0x206f as ::core::ffi::c_int
            || u >= 0x2e00 as ::core::ffi::c_int && u <= 0x2e7f as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 31 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2070 as ::core::ffi::c_int && u <= 0x209f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x20a0 as ::core::ffi::c_int && u <= 0x20cf as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x20d0 as ::core::ffi::c_int && u <= 0x20ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2100 as ::core::ffi::c_int && u <= 0x214f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2150 as ::core::ffi::c_int && u <= 0x218f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2190 as ::core::ffi::c_int && u <= 0x21ff as ::core::ffi::c_int
            || u >= 0x27f0 as ::core::ffi::c_int && u <= 0x27ff as ::core::ffi::c_int
            || u >= 0x2900 as ::core::ffi::c_int && u <= 0x297f as ::core::ffi::c_int
            || u >= 0x2b00 as ::core::ffi::c_int && u <= 0x2bff as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2200 as ::core::ffi::c_int && u <= 0x22ff as ::core::ffi::c_int
            || u >= 0x2a00 as ::core::ffi::c_int && u <= 0x2aff as ::core::ffi::c_int
            || u >= 0x27c0 as ::core::ffi::c_int && u <= 0x27ef as ::core::ffi::c_int
            || u >= 0x2980 as ::core::ffi::c_int && u <= 0x29ff as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2300 as ::core::ffi::c_int && u <= 0x23ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2400 as ::core::ffi::c_int && u <= 0x243f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2440 as ::core::ffi::c_int && u <= 0x245f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2460 as ::core::ffi::c_int && u <= 0x24ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2500 as ::core::ffi::c_int && u <= 0x257f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2580 as ::core::ffi::c_int && u <= 0x259f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x25a0 as ::core::ffi::c_int && u <= 0x25ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2600 as ::core::ffi::c_int && u <= 0x26ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2700 as ::core::ffi::c_int && u <= 0x27bf as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x3000 as ::core::ffi::c_int && u <= 0x303f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x3040 as ::core::ffi::c_int && u <= 0x309f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x30a0 as ::core::ffi::c_int && u <= 0x30ff as ::core::ffi::c_int
            || u >= 0x31f0 as ::core::ffi::c_int && u <= 0x31ff as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x3100 as ::core::ffi::c_int && u <= 0x312f as ::core::ffi::c_int
            || u >= 0x31a0 as ::core::ffi::c_int && u <= 0x31bf as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x3130 as ::core::ffi::c_int && u <= 0x318f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xa840 as ::core::ffi::c_int && u <= 0xa87f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x3200 as ::core::ffi::c_int && u <= 0x32ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x3300 as ::core::ffi::c_int && u <= 0x33ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xac00 as ::core::ffi::c_int && u <= 0xd7af as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xd800 as ::core::ffi::c_int && u <= 0xdfff as ::core::ffi::c_int
            || u > 0xffff as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 25 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10900 as ::core::ffi::c_int && u <= 0x1091f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 26 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x4e00 as ::core::ffi::c_int && u <= 0x9fff as ::core::ffi::c_int
            || u >= 0x2e80 as ::core::ffi::c_int && u <= 0x2eff as ::core::ffi::c_int
            || u >= 0x2f00 as ::core::ffi::c_int && u <= 0x2fdf as ::core::ffi::c_int
            || u >= 0x2ff0 as ::core::ffi::c_int && u <= 0x2fff as ::core::ffi::c_int
            || u >= 0x3400 as ::core::ffi::c_int && u <= 0x4dbf as ::core::ffi::c_int
            || u >= 0x20000 as ::core::ffi::c_int && u <= 0x2f7ff as ::core::ffi::c_int
            || u >= 0x3190 as ::core::ffi::c_int && u <= 0x319f as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xe000 as ::core::ffi::c_int && u <= 0xf8ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x31c0 as ::core::ffi::c_int && u <= 0x31ef as ::core::ffi::c_int
            || u >= 0xf900 as ::core::ffi::c_int && u <= 0xfaff as ::core::ffi::c_int
            || u >= 0x2f800 as ::core::ffi::c_int && u <= 0x2fa1f as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xfb00 as ::core::ffi::c_int && u <= 0xfb4f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xfb50 as ::core::ffi::c_int && u <= 0xfdff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 31 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xfe20 as ::core::ffi::c_int && u <= 0xfe2f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xfe10 as ::core::ffi::c_int && u <= 0xfe1f as ::core::ffi::c_int
            || u >= 0xfe30 as ::core::ffi::c_int && u <= 0xfe4f as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xfe50 as ::core::ffi::c_int && u <= 0xfe6f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xfe70 as ::core::ffi::c_int && u <= 0xfeff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xff00 as ::core::ffi::c_int && u <= 0xffef as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xfff0 as ::core::ffi::c_int && u <= 0xffff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xf00 as ::core::ffi::c_int && u <= 0xfff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x700 as ::core::ffi::c_int && u <= 0x74f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x780 as ::core::ffi::c_int && u <= 0x7bf as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xd80 as ::core::ffi::c_int && u <= 0xdff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1000 as ::core::ffi::c_int && u <= 0x109f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1200 as ::core::ffi::c_int && u <= 0x137f as ::core::ffi::c_int
            || u >= 0x1380 as ::core::ffi::c_int && u <= 0x139f as ::core::ffi::c_int
            || u >= 0x2d80 as ::core::ffi::c_int && u <= 0x2ddf as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x13a0 as ::core::ffi::c_int && u <= 0x13ff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1400 as ::core::ffi::c_int && u <= 0x167f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1680 as ::core::ffi::c_int && u <= 0x169f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x16a0 as ::core::ffi::c_int && u <= 0x16ff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1780 as ::core::ffi::c_int && u <= 0x17ff as ::core::ffi::c_int
            || u >= 0x19e0 as ::core::ffi::c_int && u <= 0x19ff as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1800 as ::core::ffi::c_int && u <= 0x18af as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2800 as ::core::ffi::c_int && u <= 0x28ff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xa000 as ::core::ffi::c_int && u <= 0xa48f as ::core::ffi::c_int
            || u >= 0xa490 as ::core::ffi::c_int && u <= 0xa4cf as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1700 as ::core::ffi::c_int && u <= 0x171f as ::core::ffi::c_int
            || u >= 0x1720 as ::core::ffi::c_int && u <= 0x173f as ::core::ffi::c_int
            || u >= 0x1740 as ::core::ffi::c_int && u <= 0x175f as ::core::ffi::c_int
            || u >= 0x1760 as ::core::ffi::c_int && u <= 0x177f as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10300 as ::core::ffi::c_int && u <= 0x1032f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10330 as ::core::ffi::c_int && u <= 0x1034f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10400 as ::core::ffi::c_int && u <= 0x1044f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1d000 as ::core::ffi::c_int && u <= 0x1d0ff as ::core::ffi::c_int
            || u >= 0x1d100 as ::core::ffi::c_int && u <= 0x1d1ff as ::core::ffi::c_int
            || u >= 0x1d200 as ::core::ffi::c_int && u <= 0x1d24f as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1d400 as ::core::ffi::c_int && u <= 0x1d7ff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 25 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xff000 as ::core::ffi::c_int && u <= 0xffffd as ::core::ffi::c_int
            || u >= 0x100000 as ::core::ffi::c_int && u <= 0x10fffd as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 26 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xfe00 as ::core::ffi::c_int && u <= 0xfe0f as ::core::ffi::c_int
            || u >= 0xe0100 as ::core::ffi::c_int && u <= 0xe01ef as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xe0000 as ::core::ffi::c_int && u <= 0xe007f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1900 as ::core::ffi::c_int && u <= 0x194f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1950 as ::core::ffi::c_int && u <= 0x197f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1980 as ::core::ffi::c_int && u <= 0x19df as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 31 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1a00 as ::core::ffi::c_int && u <= 0x1a1f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2c00 as ::core::ffi::c_int && u <= 0x2c5f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x2d30 as ::core::ffi::c_int && u <= 0x2d7f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x4dc0 as ::core::ffi::c_int && u <= 0x4dff as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xa800 as ::core::ffi::c_int && u <= 0xa82f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10000 as ::core::ffi::c_int && u <= 0x1007f as ::core::ffi::c_int
            || u >= 0x10080 as ::core::ffi::c_int && u <= 0x100ff as ::core::ffi::c_int
            || u >= 0x10100 as ::core::ffi::c_int && u <= 0x1013f as ::core::ffi::c_int
        {
            u4 |= ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10140 as ::core::ffi::c_int && u <= 0x1018f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10380 as ::core::ffi::c_int && u <= 0x1039f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x103a0 as ::core::ffi::c_int && u <= 0x103df as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10450 as ::core::ffi::c_int && u <= 0x1047f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10480 as ::core::ffi::c_int && u <= 0x104af as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10800 as ::core::ffi::c_int && u <= 0x1083f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10a00 as ::core::ffi::c_int && u <= 0x10a5f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1d300 as ::core::ffi::c_int && u <= 0x1d35f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x12000 as ::core::ffi::c_int && u <= 0x123ff as ::core::ffi::c_int
            || u >= 0x12400 as ::core::ffi::c_int && u <= 0x1247f as ::core::ffi::c_int
        {
            u4 |= ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1d360 as ::core::ffi::c_int && u <= 0x1d37f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1b80 as ::core::ffi::c_int && u <= 0x1bbf as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1c00 as ::core::ffi::c_int && u <= 0x1c4f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1c50 as ::core::ffi::c_int && u <= 0x1c7f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xa880 as ::core::ffi::c_int && u <= 0xa8df as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xa900 as ::core::ffi::c_int && u <= 0xa92f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xa930 as ::core::ffi::c_int && u <= 0xa95f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0xaa00 as ::core::ffi::c_int && u <= 0xaa5f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x10190 as ::core::ffi::c_int && u <= 0x101cf as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x101d0 as ::core::ffi::c_int && u <= 0x101ff as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x102a0 as ::core::ffi::c_int && u <= 0x102df as ::core::ffi::c_int
            || u >= 0x10280 as ::core::ffi::c_int && u <= 0x1029f as ::core::ffi::c_int
            || u >= 0x10920 as ::core::ffi::c_int && u <= 0x1093f as ::core::ffi::c_int
        {
            u4 |= ((1 as ::core::ffi::c_int) << 25 as ::core::ffi::c_int) as uint32_t;
        }
        if u >= 0x1f030 as ::core::ffi::c_int && u <= 0x1f09f as ::core::ffi::c_int
            || u >= 0x1f000 as ::core::ffi::c_int && u <= 0x1f02f as ::core::ffi::c_int
        {
            u4 |= ((1 as ::core::ffi::c_int) << 26 as ::core::ffi::c_int) as uint32_t;
        }
        item = (*item).hh.next as *mut cmap_Entry;
    }
    if !(*options).keep_unicode_ranges {
        (*(*font).OS_2).ulUnicodeRange1 = u1;
        (*(*font).OS_2).ulUnicodeRange2 = u2;
        (*(*font).OS_2).ulUnicodeRange3 = u3;
        (*(*font).OS_2).ulUnicodeRange4 = u4;
    }
    if minUnicode < 0x10000 as int32_t {
        (*(*font).OS_2).usFirstCharIndex = minUnicode as uint16_t;
    } else {
        (*(*font).OS_2).usFirstCharIndex = 0xffff as uint16_t;
    }
    if maxUnicode < 0x10000 as int32_t {
        (*(*font).OS_2).usLastCharIndex = maxUnicode as uint16_t;
    } else {
        (*(*font).OS_2).usLastCharIndex = 0xffff as uint16_t;
    };
}
unsafe extern "C" fn statOS_2AverageWidth(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    if (*options).keep_average_char_width {
        return;
    }
    let mut totalWidth: uint32_t = 0 as uint32_t;
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let adw: pos_t = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j as isize)).advanceWidth,
        ) as pos_t;
        if adw > 0 as ::core::ffi::c_int as pos_t {
            totalWidth = (totalWidth as pos_t + adw) as uint32_t;
        }
    }
    (*(*font).OS_2).xAvgCharWidth =
        (totalWidth as size_t).wrapping_div((*(*font).glyf).length) as int16_t;
}
unsafe extern "C" fn statMaxContextOTL(table: *const table_OTL) -> uint16_t {
    // c2rust's translation of otfcc's own `foreach(item, vector) { ... }`
    // macro (c/lib/otf-writer/stat.c): the __caryll_index*/keep* variables
    // simulate a single-iteration inner while purely so the macro body can
    // `continue`/`break`; every occurrence here reduces to a plain indexed
    // for loop over the vector, confirmed against the original C source.
    let mut maxc: uint16_t = 1 as uint16_t;
    for i in 0..(*table).lookups.length {
        let lookup: *mut otl_Lookup = *(*table).lookups.items.offset(i as isize);
        match (*lookup).type_0 {
            otl_type_gpos_pair | otl_type_gpos_markToBase | otl_type_gpos_markToLigature
            | otl_type_gpos_markToMark => {
                if (maxc as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                    maxc = 2 as uint16_t;
                }
            }
            otl_type_gsub_ligature => {
                for si in 0..(*lookup).subtables.length {
                    let subtable: *mut subtable_gsub_ligature =
                        *(*lookup).subtables.items.offset(si as isize) as *mut subtable_gsub_ligature;
                    for ei in 0..(*subtable).length {
                        let entry: *mut otl_GsubLigatureEntry = (*subtable).items.offset(ei as isize);
                        if (maxc as ::core::ffi::c_int) < (*(*entry).from).numGlyphs as ::core::ffi::c_int
                        {
                            maxc = (*(*entry).from).numGlyphs as uint16_t;
                        }
                    }
                }
            }
            otl_type_gsub_chaining | otl_type_gpos_chaining => {
                for si in 0..(*lookup).subtables.length {
                    let subtable: *mut subtable_chaining =
                        *(*lookup).subtables.items.offset(si as isize) as *mut subtable_chaining;
                    if (maxc as ::core::ffi::c_int)
                        < (*subtable).c2rust_unnamed.rule.matchCount as ::core::ffi::c_int
                    {
                        maxc = (*subtable).c2rust_unnamed.rule.matchCount as uint16_t;
                    }
                }
            }
            otl_type_gsub_reverse => {
                for si in 0..(*lookup).subtables.length {
                    let subtable: *mut subtable_gsub_reverse =
                        *(*lookup).subtables.items.offset(si as isize) as *mut subtable_gsub_reverse;
                    if (maxc as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
                        maxc = (*subtable).matchCount as uint16_t;
                    }
                }
            }
            _ => {}
        }
    }
    return maxc;
}
unsafe extern "C" fn statMaxContext(mut font: *mut otfcc_Font, mut _options: *const otfcc_Options) {
    let mut maxc: uint16_t = 1 as uint16_t;
    if !(*font).GSUB.is_null() {
        let mut maxc_gsub: uint16_t = statMaxContextOTL((*font).GSUB);
        if maxc_gsub as ::core::ffi::c_int > maxc as ::core::ffi::c_int {
            maxc = maxc_gsub;
        }
    }
    if !(*font).GPOS.is_null() {
        let mut maxc_gpos: uint16_t = statMaxContextOTL((*font).GPOS);
        if maxc_gpos as ::core::ffi::c_int > maxc as ::core::ffi::c_int {
            maxc = maxc_gpos;
        }
    }
    (*(*font).OS_2).usMaxContext = maxc;
}
unsafe extern "C" fn statOS_2(mut font: *mut otfcc_Font, mut options: *const otfcc_Options) {
    statOS_2UnicodeRanges(font, options);
    statOS_2AverageWidth(font, options);
    statMaxContext(font, options);
}
pub const MAX_STAT_METRIC: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe extern "C" fn statCFFWidths(mut font: *mut otfcc_Font) {
    if (*font).glyf.is_null() || (*font).CFF_.is_null() {
        return;
    }
    let mut frequency: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    frequency = __caryll_allocate_clean(
        (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(4096 as size_t),
        524 as ::core::ffi::c_ulong,
    ) as *mut uint32_t;
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let intWidth: uint16_t = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j as isize)).advanceWidth,
        ) as uint16_t;
        if (intWidth as ::core::ffi::c_int) < MAX_STAT_METRIC {
            let fresh1 = frequency.offset(intWidth as isize);
            *fresh1 = (*fresh1).wrapping_add(1 as uint32_t);
        }
    }
    let mut maxfreq: uint16_t = 0 as uint16_t;
    let mut maxj: uint16_t = 0 as uint16_t;
    for j_0 in 0..MAX_STAT_METRIC as uint16_t {
        if *frequency.offset(j_0 as isize) > maxfreq as uint32_t {
            maxfreq = *frequency.offset(j_0 as isize) as uint16_t;
            maxj = j_0;
        }
    }
    let mut nn: uint16_t = 0 as uint16_t;
    let mut nnsum: uint32_t = 0 as uint32_t;
    for j_1 in 0..(*(*font).glyf).length as glyphid_t {
        let adw: pos_t = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j_1 as isize)).advanceWidth,
        ) as pos_t;
        if adw != maxj as ::core::ffi::c_int as pos_t {
            nn = (nn as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint16_t;
            nnsum = (nnsum as pos_t + adw) as uint32_t;
        }
    }
    let mut nominalWidthX: int16_t = 0 as int16_t;
    if nn as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        nominalWidthX = nnsum.wrapping_div(nn as uint32_t) as int16_t;
    }
    if !(*(*font).CFF_).privateDict.is_null() {
        (*(*(*font).CFF_).privateDict).defaultWidthX = maxj as ::core::ffi::c_double;
        if nn as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            (*(*(*font).CFF_).privateDict).nominalWidthX = nominalWidthX as ::core::ffi::c_double;
        }
    }
    if !(*(*font).CFF_).fdArray.is_null() {
        for j_2 in 0..(*(*font).CFF_).fdArrayCount {
            let fd = *(*(*font).CFF_).fdArray.offset(j_2 as isize);
            (*(*fd).privateDict).defaultWidthX = maxj as ::core::ffi::c_double;
            (*(*fd).privateDict).nominalWidthX = nominalWidthX as ::core::ffi::c_double;
        }
    }
    free(frequency as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn statVORG(mut font: *mut otfcc_Font) {
    if (*font).glyf.is_null()
        || (*font).CFF_.is_null()
        || (*font).vhea.is_null()
        || (*font).vmtx.is_null()
    {
        return;
    }
    let mut frequency: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    frequency = __caryll_allocate_clean(
        (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(4096 as size_t),
        562 as ::core::ffi::c_ulong,
    ) as *mut uint32_t;
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let vori: pos_t = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j as isize)).verticalOrigin,
        ) as pos_t;
        if vori >= 0 as ::core::ffi::c_int as pos_t && vori < MAX_STAT_METRIC as pos_t {
            let fresh0 = frequency.offset(vori as uint16_t as isize);
            *fresh0 = (*fresh0).wrapping_add(1 as uint32_t);
        }
    }
    let mut maxfreq: uint32_t = 0 as uint32_t;
    let mut maxj: glyphid_t = 0 as glyphid_t;
    for j_0 in 0..MAX_STAT_METRIC as glyphid_t {
        if *frequency.offset(j_0 as isize) > maxfreq {
            maxfreq = *frequency.offset(j_0 as isize);
            maxj = j_0;
        }
    }
    let mut vorg: *mut table_VORG = ::core::ptr::null_mut::<table_VORG>();
    vorg = __caryll_allocate_clean(
        ::core::mem::size_of::<table_VORG>() as size_t,
        578 as ::core::ffi::c_ulong,
    ) as *mut table_VORG;
    (*vorg).defaultVerticalOrigin = maxj as pos_t;
    let mut nVertOrigs: glyphid_t = 0 as glyphid_t;
    for j_1 in 0..(*(*font).glyf).length as glyphid_t {
        let vori_0: pos_t = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j_1 as isize)).verticalOrigin,
        ) as pos_t;
        if vori_0 != maxj as ::core::ffi::c_int as pos_t {
            nVertOrigs = (nVertOrigs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
        }
    }
    (*vorg).numVertOriginYMetrics = nVertOrigs;
    (*vorg).entries = __caryll_allocate_clean(
        (::core::mem::size_of::<VORG_entry>() as size_t).wrapping_mul(nVertOrigs as size_t),
        587 as ::core::ffi::c_ulong,
    ) as *mut VORG_entry;
    let mut jj: glyphid_t = 0 as glyphid_t;
    for j_2 in 0..(*(*font).glyf).length as glyphid_t {
        let vori_1: pos_t = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j_2 as isize)).verticalOrigin,
        ) as pos_t;
        if vori_1 != maxj as ::core::ffi::c_int as pos_t {
            (*(*vorg).entries.offset(jj as isize)).gid = j_2;
            (*(*vorg).entries.offset(jj as isize)).verticalOrigin = vori_1 as int16_t;
            jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
        }
    }
    free(frequency as *mut ::core::ffi::c_void);
    (*font).VORG = vorg;
}
unsafe extern "C" fn statLTSH(mut font: *mut otfcc_Font) {
    if (*font).glyf.is_null() {
        return;
    }
    let mut needLTSH: bool = false;
    for j in 0..(*(*font).glyf).length as glyphid_t {
        if (**(*(*font).glyf).items.offset(j as isize)).yPel as ::core::ffi::c_int
            > 1 as ::core::ffi::c_int
        {
            needLTSH = true;
        }
    }
    if !needLTSH {
        return;
    }
    let mut ltsh: *mut table_LTSH = ::core::ptr::null_mut::<table_LTSH>();
    ltsh = __caryll_allocate_clean(
        ::core::mem::size_of::<table_LTSH>() as size_t,
        610 as ::core::ffi::c_ulong,
    ) as *mut table_LTSH;
    (*ltsh).numGlyphs = (*(*font).glyf).length as glyphid_t;
    (*ltsh).yPels = __caryll_allocate_clean(
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*ltsh).numGlyphs as size_t),
        612 as ::core::ffi::c_ulong,
    ) as *mut uint8_t;
    for j_0 in 0..(*(*font).glyf).length as glyphid_t {
        *(*ltsh).yPels.offset(j_0 as isize) = (**(*(*font).glyf).items.offset(j_0 as isize)).yPel;
    }
    (*font).LTSH = ltsh;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_statFont(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    if !(*font).glyf.is_null() && !(*font).head.is_null() {
        statGlyf(font, options);
        if !(*options).keep_modified_time {
            (*(*font).head).modified =
                2082844800 as int64_t + time(::core::ptr::null_mut::<time_t>()) as int64_t;
        }
    }
    if !(*font).head.is_null() && !(*font).CFF_.is_null() {
        let mut cff: *mut table_CFF = (*font).CFF_;
        if (*cff).fontBBoxBottom
            > (*(*font).head).yMin as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).fontBBoxBottom = (*(*font).head).yMin as ::core::ffi::c_double;
        }
        if (*cff).fontBBoxTop < (*(*font).head).yMax as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).fontBBoxTop = (*(*font).head).yMax as ::core::ffi::c_double;
        }
        if (*cff).fontBBoxLeft < (*(*font).head).xMin as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).fontBBoxLeft = (*(*font).head).xMin as ::core::ffi::c_double;
        }
        if (*cff).fontBBoxRight
            < (*(*font).head).xMax as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).fontBBoxRight = (*(*font).head).xMax as ::core::ffi::c_double;
        }
        if !(*font).glyf.is_null() && (*cff).isCID {
            (*cff).cidCount = (*(*font).glyf).length as uint32_t;
        }
        if (*cff).isCID {
            if !(*cff).fontMatrix.is_null() {
                iVQ.dispose.expect("non-null function pointer")(&raw mut (*(*cff).fontMatrix).x);
                iVQ.dispose.expect("non-null function pointer")(&raw mut (*(*cff).fontMatrix).y);
                free((*cff).fontMatrix as *mut ::core::ffi::c_void);
                (*cff).fontMatrix = ::core::ptr::null_mut::<cff_FontMatrix>();
            }
            for j in 0..(*cff).fdArrayCount {
                let fd: *mut table_CFF = *(*cff).fdArray.offset(j as isize);
                if !(*fd).fontMatrix.is_null() {
                    iVQ.dispose.expect("non-null function pointer")(&raw mut (*(*fd).fontMatrix).x);
                    iVQ.dispose.expect("non-null function pointer")(&raw mut (*(*fd).fontMatrix).y);
                    free((*fd).fontMatrix as *mut ::core::ffi::c_void);
                    (*fd).fontMatrix = ::core::ptr::null_mut::<cff_FontMatrix>();
                }
                if (*(*font).head).unitsPerEm as ::core::ffi::c_int == 1000 as ::core::ffi::c_int {
                    (*fd).fontMatrix = ::core::ptr::null_mut::<cff_FontMatrix>();
                } else {
                    (*fd).fontMatrix = __caryll_allocate_clean(
                        ::core::mem::size_of::<cff_FontMatrix>() as size_t,
                        651 as ::core::ffi::c_ulong,
                    ) as *mut cff_FontMatrix;
                    (*(*fd).fontMatrix).a = (1.0f64
                        / (*(*font).head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double)
                        as scale_t;
                    (*(*fd).fontMatrix).b = 0.0f64 as scale_t;
                    (*(*fd).fontMatrix).c = 0.0f64 as scale_t;
                    (*(*fd).fontMatrix).d = (1.0f64
                        / (*(*font).head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double)
                        as scale_t;
                    (*(*fd).fontMatrix).x = (
                        iVQ.neutral.expect("non-null function pointer"))();
                    (*(*fd).fontMatrix).y = (
                        iVQ.neutral.expect("non-null function pointer"))();
                }
            }
        } else if (*(*font).head).unitsPerEm as ::core::ffi::c_int == 1000 as ::core::ffi::c_int {
            (*cff).fontMatrix = ::core::ptr::null_mut::<cff_FontMatrix>();
        } else {
            (*cff).fontMatrix = __caryll_allocate_clean(
                ::core::mem::size_of::<cff_FontMatrix>() as size_t,
                664 as ::core::ffi::c_ulong,
            ) as *mut cff_FontMatrix;
            (*(*cff).fontMatrix).a = (1.0f64
                / (*(*font).head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double)
                as scale_t;
            (*(*cff).fontMatrix).b = 0.0f64 as scale_t;
            (*(*cff).fontMatrix).c = 0.0f64 as scale_t;
            (*(*cff).fontMatrix).d = (1.0f64
                / (*(*font).head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double)
                as scale_t;
            (*(*cff).fontMatrix).x = (
                iVQ.neutral.expect("non-null function pointer"))();
            (*(*cff).fontMatrix).y = (
                iVQ.neutral.expect("non-null function pointer"))();
        }
        statCFFWidths(font);
    }
    if !(*font).glyf.is_null() && !(*font).maxp.is_null() {
        (*(*font).maxp).numGlyphs = (*(*font).glyf).length as uint16_t;
    }
    if !(*font).glyf.is_null() && !(*font).post.is_null() {
        (*(*font).post).maxMemType42 = (*(*font).glyf).length as uint32_t;
    }
    if !(*font).glyf.is_null()
        && !(*font).maxp.is_null()
        && (*(*font).maxp).version == 0x10000 as f16dot16
    {
        statMaxp(font);
        if !(*font).fpgm.is_null()
            && (*(*font).fpgm).length > (*(*font).maxp).maxSizeOfInstructions as uint32_t
        {
            (*(*font).maxp).maxSizeOfInstructions = (*(*font).fpgm).length as uint16_t;
        }
        if !(*font).prep.is_null()
            && (*(*font).prep).length > (*(*font).maxp).maxSizeOfInstructions as uint32_t
        {
            (*(*font).maxp).maxSizeOfInstructions = (*(*font).prep).length as uint16_t;
        }
    }
    if !(*font).OS_2.is_null() && !(*font).cmap.is_null() && !(*font).glyf.is_null() {
        statOS_2(font, options);
    }
    if (*font).subtype == FONTTYPE_TTF {
        if !(*font).maxp.is_null() {
            (*(*font).maxp).version = 0x10000 as ::core::ffi::c_int as f16dot16;
        }
    } else if !(*font).maxp.is_null() {
        (*(*font).maxp).version = 0x5000 as ::core::ffi::c_int as f16dot16;
    }
    if !(*font).glyf.is_null() && !(*font).hhea.is_null() {
        statHmtx(font, options);
    }
    if !(*font).glyf.is_null() && !(*font).vhea.is_null() {
        statVmtx(font, options);
        statVORG(font);
    }
    statLTSH(font);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_unstatFont(
    mut font: *mut otfcc_Font,
    mut _options: *const otfcc_Options,
) {
    otfcc_iFont.deleteTable.expect("non-null function pointer")(font, 1751412088i32 as uint32_t);
    otfcc_iFont.deleteTable.expect("non-null function pointer")(font, 1752003704i32 as uint32_t);
    otfcc_iFont.deleteTable.expect("non-null function pointer")(font, 1448038983i32 as uint32_t);
    otfcc_iFont.deleteTable.expect("non-null function pointer")(font, 1986884728i32 as uint32_t);
    otfcc_iFont.deleteTable.expect("non-null function pointer")(font, 1280594760i32 as uint32_t);
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FLT_MAX: ::core::ffi::c_float = __FLT_MAX__;
pub const __FLT_MAX__: ::core::ffi::c_float = 3.40282347e+38f32;
