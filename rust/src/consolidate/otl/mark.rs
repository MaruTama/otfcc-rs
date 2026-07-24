use libc::{exit, free, malloc, memcmp, memset};
extern "C" {
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static otl_iMarkArray: __caryll_vectorinterface_otl_MarkArray;
    static otl_iBaseArray: __caryll_vectorinterface_otl_BaseArray;
    static otl_iLigatureArray: __caryll_vectorinterface_otl_LigatureArray;
}

use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{handle_fromConsolidated, otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{log_type_warning, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{arity_t, colorid_t, f16dot16, glyphclass_t, glyphid_t, glyphsize_t, length_t, pos_t, scale_t, shapeid_t, tableid_t};
use crate::vendor::sds::{sds};
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
    pub orderType: u8,
    pub orderEntry: u32,
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
    pub hho: isize,
    pub ideal_chain_maxlen: ::core::ffi::c_uint,
    pub nonideal_items: ::core::ffi::c_uint,
    pub ineff_expands: ::core::ffi::c_uint,
    pub noexpand: ::core::ffi::c_uint,
    pub signature: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: ::core::ffi::c_uint,
    pub expand_mult: ::core::ffi::c_uint,
}
pub type otl_ClassDef = table_TSI5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_TSI5 {
    pub numGlyphs: glyphid_t,
    pub capacity: u32,
    pub maxclass: glyphclass_t,
    pub glyphs: *mut otfcc_GlyphHandle,
    pub classes: *mut glyphclass_t,
}
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
pub type otfcc_FDHandle = otfcc_Handle;
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
pub struct VQ {
    pub kernel: pos_t,
    pub shift: vq_SegList,
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
pub struct fvar_Master {
    pub name: sds,
    pub region: *mut vq_Region,
    pub hh: UT_hash_handle,
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
pub struct fvar_Instance {
    pub subfamilyNameID: u16,
    pub flags: u16,
    pub coordinates: VV,
    pub postScriptNameID: u16,
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
pub struct vf_Axes {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut vf_Axis,
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
pub type otfcc_font_subtype = ::core::ffi::c_uint;
pub const FONTTYPE_CFF: otfcc_font_subtype = 1;
pub const FONTTYPE_TTF: otfcc_font_subtype = 0;
pub type otfcc_Font = _caryll_font;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed_3 = 10;
pub const log_vl_info: C2RustUnnamed_3 = 5;
pub const log_vl_notice: C2RustUnnamed_3 = 2;
pub const log_vl_important: C2RustUnnamed_3 = 1;
pub const log_vl_critical: C2RustUnnamed_3 = 0;
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
pub struct __caryll_vectorinterface_otl_MarkArray {
    pub init: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_MarkArray, *const otl_MarkArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_MarkArray, *mut otl_MarkArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkArray) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_MarkArray>,
    pub free: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_MarkArray, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_MarkArray, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_MarkArray>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_MarkArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_MarkArray, otl_MarkRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_MarkArray) -> otl_MarkRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_MarkArray, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_MarkArray,
            Option<unsafe extern "C" fn(*const otl_MarkRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_MarkArray,
            Option<
                unsafe extern "C" fn(
                    *const otl_MarkRecord,
                    *const otl_MarkRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_BaseArray {
    pub init: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_BaseArray, *const otl_BaseArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_BaseArray, *mut otl_BaseArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_BaseArray, otl_BaseArray) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_BaseArray, otl_BaseArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_BaseArray>,
    pub free: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_BaseArray, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_BaseArray, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_BaseArray>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_BaseArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_BaseArray, otl_BaseRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_BaseArray) -> otl_BaseRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_BaseArray, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_BaseArray,
            Option<unsafe extern "C" fn(*const otl_BaseRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_BaseArray,
            Option<
                unsafe extern "C" fn(
                    *const otl_BaseRecord,
                    *const otl_BaseRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_LigatureArray {
    pub init: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LigatureArray, *const otl_LigatureArray) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LigatureArray, *mut otl_LigatureArray) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LigatureArray, otl_LigatureArray) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LigatureArray, otl_LigatureArray) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_LigatureArray>,
    pub free: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_LigatureArray>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LigatureArray, otl_LigatureBaseRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LigatureArray) -> otl_LigatureBaseRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LigatureArray, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_LigatureArray,
            Option<
                unsafe extern "C" fn(
                    *const otl_LigatureBaseRecord,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_LigatureArray,
            Option<
                unsafe extern "C" fn(
                    *const otl_LigatureBaseRecord,
                    *const otl_LigatureBaseRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base_hash {
    pub gid: ::core::ffi::c_int,
    pub name: sds,
    pub anchors: *mut otl_Anchor,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mark_hash {
    pub gid: ::core::ffi::c_int,
    pub name: sds,
    pub markClass: glyphclass_t,
    pub anchor: otl_Anchor,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lig_hash {
    pub gid: ::core::ffi::c_int,
    pub name: sds,
    pub componentCount: glyphid_t,
    pub anchors: *mut *mut otl_Anchor,
    pub hh: UT_hash_handle,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HASH_INITIAL_NUM_BUCKETS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const HASH_INITIAL_NUM_BUCKETS_LOG2: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const HASH_BKT_CAPACITY_THRESH: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
pub const HASH_SIGNATURE: ::core::ffi::c_uint = 0xa0111fe1 as ::core::ffi::c_uint;
unsafe extern "C" fn mark_by_gid(
    mut a: *mut mark_hash,
    mut b: *mut mark_hash,
) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
unsafe extern "C" fn base_by_gid(
    mut a: *mut base_hash,
    mut b: *mut base_hash,
) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
unsafe extern "C" fn lig_by_gid(mut a: *mut lig_hash, mut b: *mut lig_hash) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
unsafe extern "C" fn consolidateMarkArray(
    mut font: *mut otfcc_Font,
    mut _table: *mut table_OTL,
    mut options: *const otfcc_Options,
    mut markArray: *mut otl_MarkArray,
    mut classCount: glyphclass_t,
) {
    let mut hm: *mut mark_hash = ::core::ptr::null_mut::<mark_hash>();
    let mut k: glyphid_t = 0 as glyphid_t;
    while (k as usize) < (*markArray).length {
        if !otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*markArray).items.offset(k as isize)).glyph,
        ) {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"[Consolidate] Ignored unknown glyph name %s.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*(*markArray).items.offset(k as isize)).glyph.name,
                ),
            );
        } else {
            let mut s: *mut mark_hash = ::core::ptr::null_mut::<mark_hash>();
            let mut gid: ::core::ffi::c_int =
                (*(*markArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
            let mut _hf_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i: ::core::ffi::c_uint = 0;
            let mut _hj_j: ::core::ffi::c_uint = 0;
            let mut _hj_k: ::core::ffi::c_uint = 0;
            let mut _hj_key: *const ::core::ffi::c_uchar =
                &raw mut gid as *const ::core::ffi::c_uchar;
            _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i = _hj_j;
            _hj_k = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            while _hj_k >= 12 as ::core::ffi::c_uint {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
            _hf_hashv = _hf_hashv
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_52: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 15766119939431011442;
                }
                10 => {
                    current_block_52 = 15766119939431011442;
                }
                9 => {
                    current_block_52 = 16082293127231038334;
                }
                8 => {
                    current_block_52 = 6924315704091482277;
                }
                7 => {
                    current_block_52 = 8817668411986532499;
                }
                6 => {
                    current_block_52 = 17613857163787856897;
                }
                5 => {
                    current_block_52 = 7171273293905213987;
                }
                4 => {
                    current_block_52 = 4496227623580412362;
                }
                3 => {
                    current_block_52 = 16130385434440591865;
                }
                2 => {
                    current_block_52 = 13809408577757465348;
                }
                1 => {
                    current_block_52 = 6834373174349270986;
                }
                _ => {
                    current_block_52 = 1345366029464561491;
                }
            }
            match current_block_52 {
                15766119939431011442 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 16082293127231038334;
                }
                _ => {}
            }
            match current_block_52 {
                16082293127231038334 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 6924315704091482277;
                }
                _ => {}
            }
            match current_block_52 {
                6924315704091482277 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 8817668411986532499;
                }
                _ => {}
            }
            match current_block_52 {
                8817668411986532499 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 17613857163787856897;
                }
                _ => {}
            }
            match current_block_52 {
                17613857163787856897 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 7171273293905213987;
                }
                _ => {}
            }
            match current_block_52 {
                7171273293905213987 => {
                    _hj_j = _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_52 = 4496227623580412362;
                }
                _ => {}
            }
            match current_block_52 {
                4496227623580412362 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 16130385434440591865;
                }
                _ => {}
            }
            match current_block_52 {
                16130385434440591865 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 13809408577757465348;
                }
                _ => {}
            }
            match current_block_52 {
                13809408577757465348 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 6834373174349270986;
                }
                _ => {}
            }
            match current_block_52 {
                6834373174349270986 => {
                    _hj_i = _hj_i
                        .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
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
            s = ::core::ptr::null_mut::<mark_hash>();
            if !hm.is_null() {
                let mut _hf_bkt: ::core::ffi::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize))
                        .hh_head
                        .is_null()
                    {
                        s = ((*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut mark_hash as *mut mark_hash;
                    } else {
                        s = ::core::ptr::null_mut::<mark_hash>();
                    }
                    while !s.is_null() {
                        if (*s).hh.hashv == _hf_hashv
                            && (*s).hh.keylen as usize
                                == ::core::mem::size_of::<::core::ffi::c_int>()
                        {
                            if memcmp(
                                (*s).hh.key,
                                &raw mut gid as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*s).hh.hh_next.is_null() {
                            s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(*hm).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut mark_hash as *mut mark_hash;
                        } else {
                            s = ::core::ptr::null_mut::<mark_hash>();
                        }
                    }
                }
            }
            if s.is_null()
                && (*(*markArray).items.offset(k as isize)).anchor.present as ::core::ffi::c_int
                    != 0
                && ((*(*markArray).items.offset(k as isize)).markClass as ::core::ffi::c_int)
                    < classCount as ::core::ffi::c_int
            {
                s = __caryll_allocate_clean(
                    ::core::mem::size_of::<mark_hash>() as usize,
                    47 as ::core::ffi::c_ulong,
                ) as *mut mark_hash;
                (*s).gid =
                    (*(*markArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
                (*s).name = sdsdup((*(*markArray).items.offset(k as isize)).glyph.name);
                (*s).markClass = (*(*markArray).items.offset(k as isize)).markClass;
                (*s).anchor = (*(*markArray).items.offset(k as isize)).anchor;
                let mut _ha_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i_0: ::core::ffi::c_uint = 0;
                let mut _hj_j_0: ::core::ffi::c_uint = 0;
                let mut _hj_k_0: ::core::ffi::c_uint = 0;
                let mut _hj_key_0: *const ::core::ffi::c_uchar =
                    &raw mut (*s).gid as *const ::core::ffi::c_uchar;
                _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_0 = _hj_j_0;
                _hj_k_0 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                        (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint
                );
                let mut current_block_171: u64;
                match _hj_k_0 {
                    11 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 9869338346707858197;
                    }
                    10 => {
                        current_block_171 = 9869338346707858197;
                    }
                    9 => {
                        current_block_171 = 7158800297742905591;
                    }
                    8 => {
                        current_block_171 = 17374360098714674690;
                    }
                    7 => {
                        current_block_171 = 15108445819848477191;
                    }
                    6 => {
                        current_block_171 = 7080490894523740831;
                    }
                    5 => {
                        current_block_171 = 706379200111713019;
                    }
                    4 => {
                        current_block_171 = 14540267986305250866;
                    }
                    3 => {
                        current_block_171 = 11423875456617891677;
                    }
                    2 => {
                        current_block_171 = 11721289334896627849;
                    }
                    1 => {
                        current_block_171 = 3913562009144861594;
                    }
                    _ => {
                        current_block_171 = 7315983924538012637;
                    }
                }
                match current_block_171 {
                    9869338346707858197 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 7158800297742905591;
                    }
                    _ => {}
                }
                match current_block_171 {
                    7158800297742905591 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 17374360098714674690;
                    }
                    _ => {}
                }
                match current_block_171 {
                    17374360098714674690 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 15108445819848477191;
                    }
                    _ => {}
                }
                match current_block_171 {
                    15108445819848477191 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 7080490894523740831;
                    }
                    _ => {}
                }
                match current_block_171 {
                    7080490894523740831 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 706379200111713019;
                    }
                    _ => {}
                }
                match current_block_171 {
                    706379200111713019 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_171 = 14540267986305250866;
                    }
                    _ => {}
                }
                match current_block_171 {
                    14540267986305250866 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 11423875456617891677;
                    }
                    _ => {}
                }
                match current_block_171 {
                    11423875456617891677 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 11721289334896627849;
                    }
                    _ => {}
                }
                match current_block_171 {
                    11721289334896627849 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 3913562009144861594;
                    }
                    _ => {}
                }
                match current_block_171 {
                    3913562009144861594 => {
                        _hj_i_0 = _hj_i_0
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
                (*s).hh.hashv = _ha_hashv;
                (*s).hh.key =
                    &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
                (*s).hh.keylen =
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                if hm.is_null() {
                    (*s).hh.next = NULL;
                    (*s).hh.prev = NULL;
                    (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                        as *mut UT_hash_table
                        as *mut UT_hash_table;
                    if (*s).hh.tbl.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*s).hh.tbl as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            ::core::mem::size_of::<UT_hash_table>() as usize,
                        );
                        (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                        (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                        (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                        (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                            .offset_from(s as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long
                            as isize;
                        (*(*s).hh.tbl).buckets = malloc(
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        ) as *mut UT_hash_bucket;
                        (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                        if (*(*s).hh.tbl).buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (32 as usize).wrapping_mul(
                                    ::core::mem::size_of::<UT_hash_bucket>() as usize,
                                ),
                            );
                        }
                    }
                    hm = s;
                } else {
                    (*s).hh.tbl = (*hm).hh.tbl;
                    (*s).hh.next = NULL;
                    (*s).hh.prev = ((*(*hm).hh.tbl).tail as *mut ::core::ffi::c_char)
                        .offset(-(*(*hm).hh.tbl).hho)
                        as *mut ::core::ffi::c_void;
                    (*(*(*hm).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                    (*(*hm).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                }
                let mut _ha_bkt: ::core::ffi::c_uint = 0;
                (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_add(1);
                _ha_bkt = _ha_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head: *mut UT_hash_bucket =
                    (*(*hm).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                (*s).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
                (*s).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                if !(*_ha_head).hh_head.is_null() {
                    (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UT_hash_handle;
                }
                (*_ha_head).hh_head = &raw mut (*s).hh as *mut UT_hash_handle;
                if (*_ha_head).count
                    >= (*_ha_head)
                        .expand_mult
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                    && (*(*s).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt: ::core::ffi::c_uint = 0;
                    let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                    let mut _he_thh: *mut UT_hash_handle =
                        ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _he_hh_nxt: *mut UT_hash_handle =
                        ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _he_new_buckets: *mut UT_hash_bucket =
                        ::core::ptr::null_mut::<UT_hash_bucket>();
                    let mut _he_newbkt: *mut UT_hash_bucket =
                        ::core::ptr::null_mut::<UT_hash_bucket>();
                    _he_new_buckets = malloc(
                        (2 as usize)
                            .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    ) as *mut UT_hash_bucket;
                    if _he_new_buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            _he_new_buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (2 as usize)
                                .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        );
                        (*(*s).hh.tbl).ideal_chain_maxlen = ((*(*s).hh.tbl).num_items
                            >> (*(*s).hh.tbl)
                                .log2_num_buckets
                                .wrapping_add(1 as ::core::ffi::c_uint))
                        .wrapping_add(
                            if (*(*s).hh.tbl).num_items
                                & (*(*s).hh.tbl)
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
                        (*(*s).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                        _he_bkt_i = 0 as ::core::ffi::c_uint;
                        while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                            _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                                as *mut UT_hash_handle;
                            while !_he_thh.is_null() {
                                _he_hh_nxt = (*_he_thh).hh_next;
                                _he_bkt = (*_he_thh).hashv
                                    & (*(*s).hh.tbl)
                                        .num_buckets
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint);
                                _he_newbkt =
                                    _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                                (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                    (*(*s).hh.tbl).nonideal_items =
                                        (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                    (*_he_newbkt).expand_mult = (*_he_newbkt)
                                        .count
                                        .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
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
                        free((*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        (*(*s).hh.tbl).num_buckets = (*(*s).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint);
                        (*(*s).hh.tbl).log2_num_buckets =
                            (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
                        (*(*s).hh.tbl).buckets = _he_new_buckets;
                        (*(*s).hh.tbl).ineff_expands = if (*(*s).hh.tbl).nonideal_items
                            > (*(*s).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                        {
                            (*(*s).hh.tbl)
                                .ineff_expands
                                .wrapping_add(1 as ::core::ffi::c_uint)
                        } else {
                            0 as ::core::ffi::c_uint
                        };
                        if (*(*s).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                            (*(*s).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                        }
                    }
                }
            } else {
                (*(*options).logger)
                    .logSDS
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[Consolidate] Ignored invalid or double-mapping mark definition for /%s.\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        (*(*markArray).items.offset(k as isize)).glyph.name,
                    ),
                );
            }
        }
        k = k.wrapping_add(1);
    }
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
    if !hm.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*hm).hh as *mut UT_hash_handle;
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
                            .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
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
                                    .offset((*(*hm).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if mark_by_gid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut mark_hash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut mark_hash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*hm).hh.tbl).hho)
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
                                .offset(-(*(*hm).hh.tbl).hho)
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
                (*(*hm).hh.tbl).tail = _hs_tail;
                hm = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut mark_hash
                    as *mut mark_hash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    otl_iMarkArray.clear.expect("non-null function pointer")(markArray);
    let mut s_0: *mut mark_hash = ::core::ptr::null_mut::<mark_hash>();
    let mut tmp: *mut mark_hash = ::core::ptr::null_mut::<mark_hash>();
    s_0 = hm;
    tmp = (if !hm.is_null() { (*hm).hh.next } else { NULL }) as *mut mark_hash as *mut mark_hash;
    while !s_0.is_null() {
        otl_iMarkArray.push.expect("non-null function pointer")(
            markArray,
            otl_MarkRecord {
                glyph: handle_fromConsolidated(
                    (*s_0).gid as glyphid_t, (*s_0).name
                ) as otfcc_GlyphHandle,
                markClass: (*s_0).markClass,
                anchor: (*s_0).anchor,
            },
        );
        sdsfree((*s_0).name);
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s_0).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*hm).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*hm).hh.tbl as *mut ::core::ffi::c_void);
            hm = ::core::ptr::null_mut::<mark_hash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*hm).hh.tbl).tail {
                (*(*hm).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh3 = (*_hd_hh_del).next;
            } else {
                hm = (*_hd_hh_del).next as *mut mark_hash as *mut mark_hash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh4 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh4 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*hm).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*hm).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UT_hash_bucket;
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
            (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<mark_hash>();
        s_0 = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut mark_hash
            as *mut mark_hash;
    }
}
unsafe extern "C" fn consolidateBaseArray(
    mut font: *mut otfcc_Font,
    mut _table: *mut table_OTL,
    mut options: *const otfcc_Options,
    mut baseArray: *mut otl_BaseArray,
) {
    let mut hm: *mut base_hash = ::core::ptr::null_mut::<base_hash>();
    let mut k: glyphid_t = 0 as glyphid_t;
    while (k as usize) < (*baseArray).length {
        if !otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*baseArray).items.offset(k as isize)).glyph,
        ) {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"[Consolidate] Ignored unknown glyph name %s.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*(*baseArray).items.offset(k as isize)).glyph.name,
                ),
            );
        } else {
            let mut s: *mut base_hash = ::core::ptr::null_mut::<base_hash>();
            let mut gid: ::core::ffi::c_int =
                (*(*baseArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
            let mut _hf_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i: ::core::ffi::c_uint = 0;
            let mut _hj_j: ::core::ffi::c_uint = 0;
            let mut _hj_k: ::core::ffi::c_uint = 0;
            let mut _hj_key: *const ::core::ffi::c_uchar =
                &raw mut gid as *const ::core::ffi::c_uchar;
            _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i = _hj_j;
            _hj_k = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            while _hj_k >= 12 as ::core::ffi::c_uint {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
            _hf_hashv = _hf_hashv
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_52: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 7016587201154547590;
                }
                10 => {
                    current_block_52 = 7016587201154547590;
                }
                9 => {
                    current_block_52 = 3980250441984174877;
                }
                8 => {
                    current_block_52 = 6148045088452986653;
                }
                7 => {
                    current_block_52 = 18186708060314969588;
                }
                6 => {
                    current_block_52 = 18271855001797298824;
                }
                5 => {
                    current_block_52 = 12249008867511416487;
                }
                4 => {
                    current_block_52 = 5550823838973015271;
                }
                3 => {
                    current_block_52 = 7202498605496425099;
                }
                2 => {
                    current_block_52 = 15113091096903121241;
                }
                1 => {
                    current_block_52 = 3727798181405880336;
                }
                _ => {
                    current_block_52 = 1345366029464561491;
                }
            }
            match current_block_52 {
                7016587201154547590 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 3980250441984174877;
                }
                _ => {}
            }
            match current_block_52 {
                3980250441984174877 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 6148045088452986653;
                }
                _ => {}
            }
            match current_block_52 {
                6148045088452986653 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 18186708060314969588;
                }
                _ => {}
            }
            match current_block_52 {
                18186708060314969588 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 18271855001797298824;
                }
                _ => {}
            }
            match current_block_52 {
                18271855001797298824 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 12249008867511416487;
                }
                _ => {}
            }
            match current_block_52 {
                12249008867511416487 => {
                    _hj_j = _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_52 = 5550823838973015271;
                }
                _ => {}
            }
            match current_block_52 {
                5550823838973015271 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 7202498605496425099;
                }
                _ => {}
            }
            match current_block_52 {
                7202498605496425099 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 15113091096903121241;
                }
                _ => {}
            }
            match current_block_52 {
                15113091096903121241 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 3727798181405880336;
                }
                _ => {}
            }
            match current_block_52 {
                3727798181405880336 => {
                    _hj_i = _hj_i
                        .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
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
            s = ::core::ptr::null_mut::<base_hash>();
            if !hm.is_null() {
                let mut _hf_bkt: ::core::ffi::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize))
                        .hh_head
                        .is_null()
                    {
                        s = ((*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut base_hash as *mut base_hash;
                    } else {
                        s = ::core::ptr::null_mut::<base_hash>();
                    }
                    while !s.is_null() {
                        if (*s).hh.hashv == _hf_hashv
                            && (*s).hh.keylen as usize
                                == ::core::mem::size_of::<::core::ffi::c_int>()
                        {
                            if memcmp(
                                (*s).hh.key,
                                &raw mut gid as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*s).hh.hh_next.is_null() {
                            s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(*hm).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut base_hash as *mut base_hash;
                        } else {
                            s = ::core::ptr::null_mut::<base_hash>();
                        }
                    }
                }
            }
            if s.is_null() {
                s = __caryll_allocate_clean(
                    ::core::mem::size_of::<base_hash>() as usize,
                    87 as ::core::ffi::c_ulong,
                ) as *mut base_hash;
                (*s).gid =
                    (*(*baseArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
                (*s).name = sdsdup((*(*baseArray).items.offset(k as isize)).glyph.name);
                (*s).anchors = (*(*baseArray).items.offset(k as isize)).anchors;
                let ref mut fresh0 = (*(*baseArray).items.offset(k as isize)).anchors;
                *fresh0 = ::core::ptr::null_mut::<otl_Anchor>();
                let mut _ha_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i_0: ::core::ffi::c_uint = 0;
                let mut _hj_j_0: ::core::ffi::c_uint = 0;
                let mut _hj_k_0: ::core::ffi::c_uint = 0;
                let mut _hj_key_0: *const ::core::ffi::c_uchar =
                    &raw mut (*s).gid as *const ::core::ffi::c_uchar;
                _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_0 = _hj_j_0;
                _hj_k_0 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                        (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint
                );
                let mut current_block_171: u64;
                match _hj_k_0 {
                    11 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 17956245827096646122;
                    }
                    10 => {
                        current_block_171 = 17956245827096646122;
                    }
                    9 => {
                        current_block_171 = 17451773831962767405;
                    }
                    8 => {
                        current_block_171 = 2555747926156542244;
                    }
                    7 => {
                        current_block_171 = 3671894898333869379;
                    }
                    6 => {
                        current_block_171 = 18122161107652318248;
                    }
                    5 => {
                        current_block_171 = 10637280720788854375;
                    }
                    4 => {
                        current_block_171 = 3528141965437604235;
                    }
                    3 => {
                        current_block_171 = 15534641122025353471;
                    }
                    2 => {
                        current_block_171 = 7633517610621306592;
                    }
                    1 => {
                        current_block_171 = 387405325351757541;
                    }
                    _ => {
                        current_block_171 = 7315983924538012637;
                    }
                }
                match current_block_171 {
                    17956245827096646122 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 17451773831962767405;
                    }
                    _ => {}
                }
                match current_block_171 {
                    17451773831962767405 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 2555747926156542244;
                    }
                    _ => {}
                }
                match current_block_171 {
                    2555747926156542244 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 3671894898333869379;
                    }
                    _ => {}
                }
                match current_block_171 {
                    3671894898333869379 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 18122161107652318248;
                    }
                    _ => {}
                }
                match current_block_171 {
                    18122161107652318248 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 10637280720788854375;
                    }
                    _ => {}
                }
                match current_block_171 {
                    10637280720788854375 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_171 = 3528141965437604235;
                    }
                    _ => {}
                }
                match current_block_171 {
                    3528141965437604235 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 15534641122025353471;
                    }
                    _ => {}
                }
                match current_block_171 {
                    15534641122025353471 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 7633517610621306592;
                    }
                    _ => {}
                }
                match current_block_171 {
                    7633517610621306592 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 387405325351757541;
                    }
                    _ => {}
                }
                match current_block_171 {
                    387405325351757541 => {
                        _hj_i_0 = _hj_i_0
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
                (*s).hh.hashv = _ha_hashv;
                (*s).hh.key =
                    &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
                (*s).hh.keylen =
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                if hm.is_null() {
                    (*s).hh.next = NULL;
                    (*s).hh.prev = NULL;
                    (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                        as *mut UT_hash_table
                        as *mut UT_hash_table;
                    if (*s).hh.tbl.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*s).hh.tbl as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            ::core::mem::size_of::<UT_hash_table>() as usize,
                        );
                        (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                        (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                        (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                        (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                            .offset_from(s as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long
                            as isize;
                        (*(*s).hh.tbl).buckets = malloc(
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        ) as *mut UT_hash_bucket;
                        (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                        if (*(*s).hh.tbl).buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (32 as usize).wrapping_mul(
                                    ::core::mem::size_of::<UT_hash_bucket>() as usize,
                                ),
                            );
                        }
                    }
                    hm = s;
                } else {
                    (*s).hh.tbl = (*hm).hh.tbl;
                    (*s).hh.next = NULL;
                    (*s).hh.prev = ((*(*hm).hh.tbl).tail as *mut ::core::ffi::c_char)
                        .offset(-(*(*hm).hh.tbl).hho)
                        as *mut ::core::ffi::c_void;
                    (*(*(*hm).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                    (*(*hm).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                }
                let mut _ha_bkt: ::core::ffi::c_uint = 0;
                (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_add(1);
                _ha_bkt = _ha_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head: *mut UT_hash_bucket =
                    (*(*hm).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                (*s).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
                (*s).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                if !(*_ha_head).hh_head.is_null() {
                    (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UT_hash_handle;
                }
                (*_ha_head).hh_head = &raw mut (*s).hh as *mut UT_hash_handle;
                if (*_ha_head).count
                    >= (*_ha_head)
                        .expand_mult
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                    && (*(*s).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt: ::core::ffi::c_uint = 0;
                    let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                    let mut _he_thh: *mut UT_hash_handle =
                        ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _he_hh_nxt: *mut UT_hash_handle =
                        ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _he_new_buckets: *mut UT_hash_bucket =
                        ::core::ptr::null_mut::<UT_hash_bucket>();
                    let mut _he_newbkt: *mut UT_hash_bucket =
                        ::core::ptr::null_mut::<UT_hash_bucket>();
                    _he_new_buckets = malloc(
                        (2 as usize)
                            .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    ) as *mut UT_hash_bucket;
                    if _he_new_buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            _he_new_buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (2 as usize)
                                .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        );
                        (*(*s).hh.tbl).ideal_chain_maxlen = ((*(*s).hh.tbl).num_items
                            >> (*(*s).hh.tbl)
                                .log2_num_buckets
                                .wrapping_add(1 as ::core::ffi::c_uint))
                        .wrapping_add(
                            if (*(*s).hh.tbl).num_items
                                & (*(*s).hh.tbl)
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
                        (*(*s).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                        _he_bkt_i = 0 as ::core::ffi::c_uint;
                        while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                            _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                                as *mut UT_hash_handle;
                            while !_he_thh.is_null() {
                                _he_hh_nxt = (*_he_thh).hh_next;
                                _he_bkt = (*_he_thh).hashv
                                    & (*(*s).hh.tbl)
                                        .num_buckets
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint);
                                _he_newbkt =
                                    _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                                (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                    (*(*s).hh.tbl).nonideal_items =
                                        (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                    (*_he_newbkt).expand_mult = (*_he_newbkt)
                                        .count
                                        .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
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
                        free((*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        (*(*s).hh.tbl).num_buckets = (*(*s).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint);
                        (*(*s).hh.tbl).log2_num_buckets =
                            (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
                        (*(*s).hh.tbl).buckets = _he_new_buckets;
                        (*(*s).hh.tbl).ineff_expands = if (*(*s).hh.tbl).nonideal_items
                            > (*(*s).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                        {
                            (*(*s).hh.tbl)
                                .ineff_expands
                                .wrapping_add(1 as ::core::ffi::c_uint)
                        } else {
                            0 as ::core::ffi::c_uint
                        };
                        if (*(*s).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                            (*(*s).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                        }
                    }
                }
            } else {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[Consolidate] Ignored anchor double-definition for /%s.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*(*baseArray).items.offset(k as isize)).glyph.name,
                    ),
                );
            }
        }
        k = k.wrapping_add(1);
    }
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
    if !hm.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*hm).hh as *mut UT_hash_handle;
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
                            .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
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
                                    .offset((*(*hm).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if base_by_gid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut base_hash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut base_hash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*hm).hh.tbl).hho)
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
                                .offset(-(*(*hm).hh.tbl).hho)
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
                (*(*hm).hh.tbl).tail = _hs_tail;
                hm = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut base_hash
                    as *mut base_hash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    otl_iBaseArray.clear.expect("non-null function pointer")(baseArray);
    let mut s_0: *mut base_hash = ::core::ptr::null_mut::<base_hash>();
    let mut tmp: *mut base_hash = ::core::ptr::null_mut::<base_hash>();
    s_0 = hm;
    tmp = (if !hm.is_null() { (*hm).hh.next } else { NULL }) as *mut base_hash as *mut base_hash;
    while !s_0.is_null() {
        otl_iBaseArray.push.expect("non-null function pointer")(
            baseArray,
            otl_BaseRecord {
                glyph: handle_fromConsolidated(
                    (*s_0).gid as glyphid_t, (*s_0).name
                ) as otfcc_GlyphHandle,
                anchors: (*s_0).anchors,
            },
        );
        sdsfree((*s_0).name);
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s_0).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*hm).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*hm).hh.tbl as *mut ::core::ffi::c_void);
            hm = ::core::ptr::null_mut::<base_hash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*hm).hh.tbl).tail {
                (*(*hm).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh1 = (*_hd_hh_del).next;
            } else {
                hm = (*_hd_hh_del).next as *mut base_hash as *mut base_hash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh2 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*hm).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*hm).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UT_hash_bucket;
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
            (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<base_hash>();
        s_0 = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut base_hash
            as *mut base_hash;
    }
}
unsafe extern "C" fn consolidateLigArray(
    mut font: *mut otfcc_Font,
    mut _table: *mut table_OTL,
    mut options: *const otfcc_Options,
    mut ligArray: *mut otl_LigatureArray,
) {
    let mut hm: *mut lig_hash = ::core::ptr::null_mut::<lig_hash>();
    let mut k: glyphid_t = 0 as glyphid_t;
    while (k as usize) < (*ligArray).length {
        if !otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*ligArray).items.offset(k as isize)).glyph,
        ) {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"[Consolidate] Ignored unknown glyph name %s.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*(*ligArray).items.offset(k as isize)).glyph.name,
                ),
            );
        } else {
            let mut s: *mut lig_hash = ::core::ptr::null_mut::<lig_hash>();
            let mut gid: ::core::ffi::c_int =
                (*(*ligArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
            let mut _hf_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i: ::core::ffi::c_uint = 0;
            let mut _hj_j: ::core::ffi::c_uint = 0;
            let mut _hj_k: ::core::ffi::c_uint = 0;
            let mut _hj_key: *const ::core::ffi::c_uchar =
                &raw mut gid as *const ::core::ffi::c_uchar;
            _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i = _hj_j;
            _hj_k = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            while _hj_k >= 12 as ::core::ffi::c_uint {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
            _hf_hashv = _hf_hashv
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_52: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 671631640629127466;
                }
                10 => {
                    current_block_52 = 671631640629127466;
                }
                9 => {
                    current_block_52 = 2507948425875615653;
                }
                8 => {
                    current_block_52 = 11781834747162053735;
                }
                7 => {
                    current_block_52 = 4976633191839108100;
                }
                6 => {
                    current_block_52 = 874960838666993744;
                }
                5 => {
                    current_block_52 = 16560270646560938773;
                }
                4 => {
                    current_block_52 = 4552817509067871589;
                }
                3 => {
                    current_block_52 = 10488484428681960669;
                }
                2 => {
                    current_block_52 = 10475785614112867771;
                }
                1 => {
                    current_block_52 = 9334142406440380371;
                }
                _ => {
                    current_block_52 = 1345366029464561491;
                }
            }
            match current_block_52 {
                671631640629127466 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 2507948425875615653;
                }
                _ => {}
            }
            match current_block_52 {
                2507948425875615653 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 11781834747162053735;
                }
                _ => {}
            }
            match current_block_52 {
                11781834747162053735 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 4976633191839108100;
                }
                _ => {}
            }
            match current_block_52 {
                4976633191839108100 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 874960838666993744;
                }
                _ => {}
            }
            match current_block_52 {
                874960838666993744 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 16560270646560938773;
                }
                _ => {}
            }
            match current_block_52 {
                16560270646560938773 => {
                    _hj_j = _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_52 = 4552817509067871589;
                }
                _ => {}
            }
            match current_block_52 {
                4552817509067871589 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 10488484428681960669;
                }
                _ => {}
            }
            match current_block_52 {
                10488484428681960669 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 10475785614112867771;
                }
                _ => {}
            }
            match current_block_52 {
                10475785614112867771 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 9334142406440380371;
                }
                _ => {}
            }
            match current_block_52 {
                9334142406440380371 => {
                    _hj_i = _hj_i
                        .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
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
            s = ::core::ptr::null_mut::<lig_hash>();
            if !hm.is_null() {
                let mut _hf_bkt: ::core::ffi::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize))
                        .hh_head
                        .is_null()
                    {
                        s = ((*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut lig_hash
                            as *mut lig_hash;
                    } else {
                        s = ::core::ptr::null_mut::<lig_hash>();
                    }
                    while !s.is_null() {
                        if (*s).hh.hashv == _hf_hashv
                            && (*s).hh.keylen as usize
                                == ::core::mem::size_of::<::core::ffi::c_int>()
                        {
                            if memcmp(
                                (*s).hh.key,
                                &raw mut gid as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*s).hh.hh_next.is_null() {
                            s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(*hm).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut lig_hash as *mut lig_hash;
                        } else {
                            s = ::core::ptr::null_mut::<lig_hash>();
                        }
                    }
                }
            }
            if s.is_null() {
                s = __caryll_allocate_clean(
                    ::core::mem::size_of::<lig_hash>() as usize,
                    125 as ::core::ffi::c_ulong,
                ) as *mut lig_hash;
                (*s).gid =
                    (*(*ligArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
                (*s).name = sdsdup((*(*ligArray).items.offset(k as isize)).glyph.name);
                (*s).componentCount = (*(*ligArray).items.offset(k as isize)).componentCount;
                (*s).anchors = (*(*ligArray).items.offset(k as isize)).anchors;
                let ref mut fresh5 = (*(*ligArray).items.offset(k as isize)).anchors;
                *fresh5 = ::core::ptr::null_mut::<*mut otl_Anchor>();
                let mut _ha_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i_0: ::core::ffi::c_uint = 0;
                let mut _hj_j_0: ::core::ffi::c_uint = 0;
                let mut _hj_k_0: ::core::ffi::c_uint = 0;
                let mut _hj_key_0: *const ::core::ffi::c_uchar =
                    &raw mut (*s).gid as *const ::core::ffi::c_uchar;
                _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_0 = _hj_j_0;
                _hj_k_0 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                        (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
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
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint
                );
                let mut current_block_172: u64;
                match _hj_k_0 {
                    11 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_172 = 6992567993738996300;
                    }
                    10 => {
                        current_block_172 = 6992567993738996300;
                    }
                    9 => {
                        current_block_172 = 10765054353708945446;
                    }
                    8 => {
                        current_block_172 = 8954403608661305380;
                    }
                    7 => {
                        current_block_172 = 1261730542405161010;
                    }
                    6 => {
                        current_block_172 = 4662756573773047525;
                    }
                    5 => {
                        current_block_172 = 5898464050339554315;
                    }
                    4 => {
                        current_block_172 = 4359070003714544862;
                    }
                    3 => {
                        current_block_172 = 10064774401823359594;
                    }
                    2 => {
                        current_block_172 = 114102700035871186;
                    }
                    1 => {
                        current_block_172 = 6262880948454574332;
                    }
                    _ => {
                        current_block_172 = 939350892795860671;
                    }
                }
                match current_block_172 {
                    6992567993738996300 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_172 = 10765054353708945446;
                    }
                    _ => {}
                }
                match current_block_172 {
                    10765054353708945446 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_172 = 8954403608661305380;
                    }
                    _ => {}
                }
                match current_block_172 {
                    8954403608661305380 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_172 = 1261730542405161010;
                    }
                    _ => {}
                }
                match current_block_172 {
                    1261730542405161010 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_172 = 4662756573773047525;
                    }
                    _ => {}
                }
                match current_block_172 {
                    4662756573773047525 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_172 = 5898464050339554315;
                    }
                    _ => {}
                }
                match current_block_172 {
                    5898464050339554315 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_172 = 4359070003714544862;
                    }
                    _ => {}
                }
                match current_block_172 {
                    4359070003714544862 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_172 = 10064774401823359594;
                    }
                    _ => {}
                }
                match current_block_172 {
                    10064774401823359594 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_172 = 114102700035871186;
                    }
                    _ => {}
                }
                match current_block_172 {
                    114102700035871186 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_172 = 6262880948454574332;
                    }
                    _ => {}
                }
                match current_block_172 {
                    6262880948454574332 => {
                        _hj_i_0 = _hj_i_0
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
                (*s).hh.hashv = _ha_hashv;
                (*s).hh.key =
                    &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
                (*s).hh.keylen =
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                if hm.is_null() {
                    (*s).hh.next = NULL;
                    (*s).hh.prev = NULL;
                    (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                        as *mut UT_hash_table
                        as *mut UT_hash_table;
                    if (*s).hh.tbl.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*s).hh.tbl as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            ::core::mem::size_of::<UT_hash_table>() as usize,
                        );
                        (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                        (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                        (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                        (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                            .offset_from(s as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long
                            as isize;
                        (*(*s).hh.tbl).buckets = malloc(
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        ) as *mut UT_hash_bucket;
                        (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                        if (*(*s).hh.tbl).buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (32 as usize).wrapping_mul(
                                    ::core::mem::size_of::<UT_hash_bucket>() as usize,
                                ),
                            );
                        }
                    }
                    hm = s;
                } else {
                    (*s).hh.tbl = (*hm).hh.tbl;
                    (*s).hh.next = NULL;
                    (*s).hh.prev = ((*(*hm).hh.tbl).tail as *mut ::core::ffi::c_char)
                        .offset(-(*(*hm).hh.tbl).hho)
                        as *mut ::core::ffi::c_void;
                    (*(*(*hm).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                    (*(*hm).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                }
                let mut _ha_bkt: ::core::ffi::c_uint = 0;
                (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_add(1);
                _ha_bkt = _ha_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head: *mut UT_hash_bucket =
                    (*(*hm).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                (*s).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
                (*s).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                if !(*_ha_head).hh_head.is_null() {
                    (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UT_hash_handle;
                }
                (*_ha_head).hh_head = &raw mut (*s).hh as *mut UT_hash_handle;
                if (*_ha_head).count
                    >= (*_ha_head)
                        .expand_mult
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                    && (*(*s).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt: ::core::ffi::c_uint = 0;
                    let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                    let mut _he_thh: *mut UT_hash_handle =
                        ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _he_hh_nxt: *mut UT_hash_handle =
                        ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _he_new_buckets: *mut UT_hash_bucket =
                        ::core::ptr::null_mut::<UT_hash_bucket>();
                    let mut _he_newbkt: *mut UT_hash_bucket =
                        ::core::ptr::null_mut::<UT_hash_bucket>();
                    _he_new_buckets = malloc(
                        (2 as usize)
                            .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    ) as *mut UT_hash_bucket;
                    if _he_new_buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            _he_new_buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (2 as usize)
                                .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        );
                        (*(*s).hh.tbl).ideal_chain_maxlen = ((*(*s).hh.tbl).num_items
                            >> (*(*s).hh.tbl)
                                .log2_num_buckets
                                .wrapping_add(1 as ::core::ffi::c_uint))
                        .wrapping_add(
                            if (*(*s).hh.tbl).num_items
                                & (*(*s).hh.tbl)
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
                        (*(*s).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                        _he_bkt_i = 0 as ::core::ffi::c_uint;
                        while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                            _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                                as *mut UT_hash_handle;
                            while !_he_thh.is_null() {
                                _he_hh_nxt = (*_he_thh).hh_next;
                                _he_bkt = (*_he_thh).hashv
                                    & (*(*s).hh.tbl)
                                        .num_buckets
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint);
                                _he_newbkt =
                                    _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                                (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                    (*(*s).hh.tbl).nonideal_items =
                                        (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                    (*_he_newbkt).expand_mult = (*_he_newbkt)
                                        .count
                                        .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
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
                        free((*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        (*(*s).hh.tbl).num_buckets = (*(*s).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint);
                        (*(*s).hh.tbl).log2_num_buckets =
                            (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
                        (*(*s).hh.tbl).buckets = _he_new_buckets;
                        (*(*s).hh.tbl).ineff_expands = if (*(*s).hh.tbl).nonideal_items
                            > (*(*s).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                        {
                            (*(*s).hh.tbl)
                                .ineff_expands
                                .wrapping_add(1 as ::core::ffi::c_uint)
                        } else {
                            0 as ::core::ffi::c_uint
                        };
                        if (*(*s).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                            (*(*s).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                        }
                    }
                }
            } else {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[Consolidate] Ignored anchor double-definition for /%s.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*(*ligArray).items.offset(k as isize)).glyph.name,
                    ),
                );
            }
        }
        k = k.wrapping_add(1);
    }
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
    if !hm.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*hm).hh as *mut UT_hash_handle;
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
                            .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
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
                                    .offset((*(*hm).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if lig_by_gid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut lig_hash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut lig_hash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*hm).hh.tbl).hho)
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
                                .offset(-(*(*hm).hh.tbl).hho)
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
                (*(*hm).hh.tbl).tail = _hs_tail;
                hm = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut lig_hash
                    as *mut lig_hash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    otl_iLigatureArray.clear.expect("non-null function pointer")(ligArray);
    let mut s_0: *mut lig_hash = ::core::ptr::null_mut::<lig_hash>();
    let mut tmp: *mut lig_hash = ::core::ptr::null_mut::<lig_hash>();
    s_0 = hm;
    tmp = (if !hm.is_null() { (*hm).hh.next } else { NULL }) as *mut lig_hash as *mut lig_hash;
    while !s_0.is_null() {
        otl_iLigatureArray.push.expect("non-null function pointer")(
            ligArray,
            otl_LigatureBaseRecord {
                glyph: handle_fromConsolidated(
                    (*s_0).gid as glyphid_t, (*s_0).name
                ) as otfcc_GlyphHandle,
                componentCount: (*s_0).componentCount,
                anchors: (*s_0).anchors,
            },
        );
        sdsfree((*s_0).name);
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s_0).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*hm).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*hm).hh.tbl as *mut ::core::ffi::c_void);
            hm = ::core::ptr::null_mut::<lig_hash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*hm).hh.tbl).tail {
                (*(*hm).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh6 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh6 = (*_hd_hh_del).next;
            } else {
                hm = (*_hd_hh_del).next as *mut lig_hash as *mut lig_hash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh7 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh7 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*hm).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*hm).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UT_hash_bucket;
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
            (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<lig_hash>();
        s_0 = tmp;
        tmp =
            (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut lig_hash as *mut lig_hash;
    }
}
#[no_mangle]
pub unsafe extern "C" fn consolidate_mark_to_single(
    mut font: *mut otfcc_Font,
    mut table: *mut table_OTL,
    mut _subtable: *mut otl_Subtable,
    mut options: *const otfcc_Options,
) -> bool {
    let mut subtable: *mut subtable_gpos_markToSingle = &raw mut (*_subtable).gpos_markToSingle;
    consolidateMarkArray(
        font,
        table,
        options,
        &raw mut (*subtable).markArray,
        (*subtable).classCount,
    );
    consolidateBaseArray(font, table, options, &raw mut (*subtable).baseArray);
    return (*subtable).markArray.length == 0 as usize
        || (*subtable).baseArray.length == 0 as usize;
}
#[no_mangle]
pub unsafe extern "C" fn consolidate_mark_to_ligature(
    mut font: *mut otfcc_Font,
    mut table: *mut table_OTL,
    mut _subtable: *mut otl_Subtable,
    mut options: *const otfcc_Options,
) -> bool {
    let mut subtable: *mut subtable_gpos_markToLigature = &raw mut (*_subtable).gpos_markToLigature;
    consolidateMarkArray(
        font,
        table,
        options,
        &raw mut (*subtable).markArray,
        (*subtable).classCount,
    );
    consolidateLigArray(font, table, options, &raw mut (*subtable).ligArray);
    return (*subtable).markArray.length == 0 as usize
        || (*subtable).ligArray.length == 0 as usize;
}
