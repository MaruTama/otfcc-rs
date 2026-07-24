pub mod otl;

extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static tsi_iEntry: __caryll_elementinterface_tsi_Entry;
    static table_iTSI: __caryll_vectorinterface_table_TSI;
    static glyf_iContourList: __caryll_vectorinterface_glyf_ContourList;
    static glyf_iStemDefList: __caryll_vectorinterface_glyf_StemDefList;
    static glyf_iMaskList: __caryll_vectorinterface_glyf_MaskList;
    static glyf_iComponentReference: __caryll_elementinterface_glyf_ComponentReference;
    static glyf_iReferenceList: __caryll_vectorinterface_glyf_ReferenceList;
    static iVQ: __caryll_vectorinterface_VQ;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static iSubtable_gsub_single: __caryll_vectorinterface_subtable_gsub_single;
    static iSubtable_gsub_multi: __caryll_vectorinterface_subtable_gsub_multi;
    static iSubtable_gsub_ligature: __caryll_vectorinterface_subtable_gsub_ligature;
    static iSubtable_chaining: __caryll_elementinterface_subtable_chaining;
    static iSubtable_gsub_reverse: __caryll_elementinterface_subtable_gsub_reverse;
    static iSubtable_gpos_single: __caryll_vectorinterface_subtable_gpos_single;
    static iSubtable_gpos_pair: __caryll_elementinterface_subtable_gpos_pair;
    static iSubtable_gpos_cursive: __caryll_vectorinterface_subtable_gpos_cursive;
    static iSubtable_gpos_markToSingle: __caryll_elementinterface_subtable_gpos_markToSingle;
    static iSubtable_gpos_markToLigature: __caryll_elementinterface_subtable_gpos_markToLigature;
    static otl_iLookupList: __caryll_vectorinterface_otl_LookupList;
    static otl_iLookupRefList: __caryll_vectorinterface_otl_LookupRefList;
    static otl_iFeatureList: __caryll_vectorinterface_otl_FeatureList;
    static otl_iFeatureRefList: __caryll_vectorinterface_otl_FeatureRefList;
    static colr_iLayer: __caryll_elementinterface_colr_Layer;
    static colr_iLayerList: __caryll_vectorinterface_colr_LayerList;
    static colr_iMapping: __caryll_elementinterface_colr_Mapping;
    static table_iCOLR: __caryll_vectorinterface_table_COLR;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn otfcc_newGlyf_glyph() -> *mut glyf_Glyph;
    fn fontop_consolidateClassDef(
        font: *mut otfcc_Font,
        cd: *mut otl_ClassDef,
        options: *const otfcc_Options,
    );
    fn consolidate_gsub_single(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gsub_multi(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gsub_alternative(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gsub_ligature(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gsub_reverse(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gpos_single(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gpos_pair(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_gpos_cursive(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_chaining(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_mark_to_single(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_mark_to_ligature(
        font: *mut otfcc_Font,
        table: *mut table_OTL,
        _subtable: *mut otl_Subtable,
        options: *const otfcc_Options,
    ) -> bool;
    fn consolidate_GDEF(
        font: *mut otfcc_Font,
        gdef: *mut table_GDEF,
        options: *const otfcc_Options,
    );
}

use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{handle_consolidateTo, handle_fromIndex, otfcc_Handle_copy, otfcc_Handle_dispose, otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle, HANDLE_STATE_INDEX, HANDLE_STATE_EMPTY};
use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
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
pub type sds = *mut ::core::ffi::c_char;
pub type glyphid_t = u16;
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
pub type glyphclass_t = u16;
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
pub struct caryll_Buffer {
    pub cursor: usize,
    pub size: usize,
    pub free: usize,
    pub data: *mut u8,
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
pub type colorid_t = u16;
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
pub type pos_t = ::core::ffi::c_double;
pub type tableid_t = u16;
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
pub type glyphsize_t = u16;
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
pub type shapeid_t = u16;
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
pub type arity_t = u32;
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
pub type length_t = ::core::ffi::c_double;
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
pub type f16dot16 = i32;
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
            u8,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, u8, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, u8) -> ()>,
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
pub struct __caryll_vectorinterface_glyf_ContourList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_ContourList, *const glyf_ContourList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_ContourList, *mut glyf_ContourList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_ContourList, glyf_ContourList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_ContourList, glyf_ContourList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_ContourList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_ContourList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_ContourList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut glyf_ContourList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_ContourList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_ContourList, glyf_Contour) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> glyf_Contour>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_ContourList, usize) -> ()>,
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
pub struct __caryll_vectorinterface_glyf_StemDefList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_StemDefList, *const glyf_StemDefList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_StemDefList, *mut glyf_StemDefList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_StemDefList, glyf_StemDefList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_StemDefList, glyf_StemDefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_StemDefList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut glyf_StemDefList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_StemDefList, glyf_PostscriptStemDef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> glyf_PostscriptStemDef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> ()>,
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
pub struct __caryll_vectorinterface_glyf_MaskList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_MaskList, *const glyf_MaskList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_MaskList, *mut glyf_MaskList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_MaskList, glyf_MaskList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_MaskList, glyf_MaskList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_MaskList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_MaskList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_MaskList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut glyf_MaskList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_MaskList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_MaskList, glyf_PostscriptHintMask) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> glyf_PostscriptHintMask>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_MaskList, usize) -> ()>,
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
pub struct __caryll_vectorinterface_glyf_ReferenceList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut glyf_ReferenceList, *const glyf_ReferenceList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut glyf_ReferenceList, *mut glyf_ReferenceList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, glyf_ReferenceList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut glyf_ReferenceList, glyf_ReferenceList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_ReferenceList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut glyf_ReferenceList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, glyf_ComponentReference) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> glyf_ComponentReference>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_ReferenceList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_ComponentReference,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_ReferenceList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_ComponentReference,
                    *const glyf_ComponentReference,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_subtable_gsub_single {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut subtable_gsub_single, *const subtable_gsub_single) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gsub_single, *mut subtable_gsub_single) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_single, subtable_gsub_single) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_single, subtable_gsub_single) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gsub_single>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut subtable_gsub_single>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut subtable_gsub_single, otl_GsubSingleEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> otl_GsubSingleEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gsub_single, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_single,
            Option<
                unsafe extern "C" fn(*const otl_GsubSingleEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_single,
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubSingleEntry,
                    *const otl_GsubSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_subtable_gsub_multi {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut subtable_gsub_multi, *const subtable_gsub_multi) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gsub_multi, *mut subtable_gsub_multi) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, subtable_gsub_multi) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_multi, subtable_gsub_multi) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gsub_multi>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut subtable_gsub_multi>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, otl_GsubMultiEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> otl_GsubMultiEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gsub_multi, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_multi,
            Option<
                unsafe extern "C" fn(*const otl_GsubMultiEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_multi,
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubMultiEntry,
                    *const otl_GsubMultiEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_subtable_gsub_ligature {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut subtable_gsub_ligature, *const subtable_gsub_ligature) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut subtable_gsub_ligature, *mut subtable_gsub_ligature) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, subtable_gsub_ligature) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, subtable_gsub_ligature) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gsub_ligature>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut subtable_gsub_ligature>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, otl_GsubLigatureEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> otl_GsubLigatureEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gsub_ligature, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_ligature,
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubLigatureEntry,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gsub_ligature,
            Option<
                unsafe extern "C" fn(
                    *const otl_GsubLigatureEntry,
                    *const otl_GsubLigatureEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_subtable_chaining {
    pub init: Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut subtable_chaining, *const subtable_chaining) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut subtable_chaining, *mut subtable_chaining) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut subtable_chaining, subtable_chaining) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut subtable_chaining, subtable_chaining) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_chaining>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_subtable_gsub_reverse {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gsub_reverse) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut subtable_gsub_reverse, *const subtable_gsub_reverse) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gsub_reverse, *mut subtable_gsub_reverse) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gsub_reverse) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_reverse, subtable_gsub_reverse) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gsub_reverse, subtable_gsub_reverse) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gsub_reverse>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gsub_reverse) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_subtable_gpos_single {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, *const subtable_gpos_single) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, *mut subtable_gpos_single) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, subtable_gpos_single) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_single, subtable_gpos_single) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_single>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut subtable_gpos_single>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut subtable_gpos_single, otl_GposSingleEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> otl_GposSingleEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_single,
            Option<
                unsafe extern "C" fn(*const otl_GposSingleEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_single,
            Option<
                unsafe extern "C" fn(
                    *const otl_GposSingleEntry,
                    *const otl_GposSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_subtable_gpos_pair {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut subtable_gpos_pair, *const subtable_gpos_pair) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gpos_pair, *mut subtable_gpos_pair) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut subtable_gpos_pair, subtable_gpos_pair) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_pair, subtable_gpos_pair) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_pair>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_subtable_gpos_cursive {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut subtable_gpos_cursive, *const subtable_gpos_cursive) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, *mut subtable_gpos_cursive) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, subtable_gpos_cursive) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, subtable_gpos_cursive) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_cursive>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut subtable_gpos_cursive>,
    pub fill: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, otl_GposCursiveEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> otl_GposCursiveEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut subtable_gpos_cursive, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_cursive,
            Option<
                unsafe extern "C" fn(*const otl_GposCursiveEntry, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_cursive,
            Option<
                unsafe extern "C" fn(
                    *const otl_GposCursiveEntry,
                    *const otl_GposCursiveEntry,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_subtable_gpos_markToSingle {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_markToSingle,
            *const subtable_gpos_markToSingle,
        ) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_markToSingle,
            *mut subtable_gpos_markToSingle,
        ) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
    pub replace: Option<
        unsafe extern "C" fn(*mut subtable_gpos_markToSingle, subtable_gpos_markToSingle) -> (),
    >,
    pub copyReplace: Option<
        unsafe extern "C" fn(*mut subtable_gpos_markToSingle, subtable_gpos_markToSingle) -> (),
    >,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_markToSingle>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_subtable_gpos_markToLigature {
    pub init: Option<unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_markToLigature,
            *const subtable_gpos_markToLigature,
        ) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(
            *mut subtable_gpos_markToLigature,
            *mut subtable_gpos_markToLigature,
        ) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> ()>,
    pub replace: Option<
        unsafe extern "C" fn(*mut subtable_gpos_markToLigature, subtable_gpos_markToLigature) -> (),
    >,
    pub copyReplace: Option<
        unsafe extern "C" fn(*mut subtable_gpos_markToLigature, subtable_gpos_markToLigature) -> (),
    >,
    pub create: Option<unsafe extern "C" fn() -> *mut subtable_gpos_markToLigature>,
    pub free: Option<unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_LookupList {
    pub init: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LookupList, *const otl_LookupList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LookupList, *mut otl_LookupList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LookupList, otl_LookupList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LookupList, otl_LookupList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_LookupList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_LookupList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LookupList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_LookupList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LookupList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LookupList, otl_LookupPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LookupList) -> otl_LookupPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LookupList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_LookupList,
            Option<unsafe extern "C" fn(*const otl_LookupPtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_LookupList,
            Option<
                unsafe extern "C" fn(
                    *const otl_LookupPtr,
                    *const otl_LookupPtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_LookupRefList {
    pub init: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LookupRefList, *const otl_LookupRefList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LookupRefList, *mut otl_LookupRefList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LookupRefList, otl_LookupRefList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LookupRefList, otl_LookupRefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_LookupRefList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_LookupRefList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LookupRefList, otl_LookupRef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> otl_LookupRef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LookupRefList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_LookupRefList,
            Option<unsafe extern "C" fn(*const otl_LookupRef, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_LookupRefList,
            Option<
                unsafe extern "C" fn(
                    *const otl_LookupRef,
                    *const otl_LookupRef,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_FeatureList {
    pub init: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_FeatureList, *const otl_FeatureList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_FeatureList, *mut otl_FeatureList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_FeatureList, otl_FeatureList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_FeatureList, otl_FeatureList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_FeatureList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_FeatureList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_FeatureList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_FeatureList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_FeatureList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_FeatureList, otl_FeaturePtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> otl_FeaturePtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_FeatureList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_FeatureList,
            Option<unsafe extern "C" fn(*const otl_FeaturePtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_FeatureList,
            Option<
                unsafe extern "C" fn(
                    *const otl_FeaturePtr,
                    *const otl_FeaturePtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_FeatureRefList {
    pub init: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_FeatureRefList, *const otl_FeatureRefList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_FeatureRefList, *mut otl_FeatureRefList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, otl_FeatureRefList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_FeatureRefList, otl_FeatureRefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_FeatureRefList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_FeatureRefList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, otl_FeatureRef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> otl_FeatureRef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_FeatureRefList,
            Option<unsafe extern "C" fn(*const otl_FeatureRef, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_FeatureRefList,
            Option<
                unsafe extern "C" fn(
                    *const otl_FeatureRef,
                    *const otl_FeatureRef,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_colr_Layer {
    pub init: Option<unsafe extern "C" fn(*mut colr_Layer) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut colr_Layer, *const colr_Layer) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut colr_Layer, *mut colr_Layer) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut colr_Layer) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut colr_Layer, colr_Layer) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut colr_Layer, colr_Layer) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_colr_LayerList {
    pub init: Option<unsafe extern "C" fn(*mut colr_LayerList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut colr_LayerList, *const colr_LayerList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut colr_LayerList, *mut colr_LayerList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut colr_LayerList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut colr_LayerList, colr_LayerList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut colr_LayerList, colr_LayerList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut colr_LayerList>,
    pub free: Option<unsafe extern "C" fn(*mut colr_LayerList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut colr_LayerList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut colr_LayerList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut colr_LayerList>,
    pub fill: Option<unsafe extern "C" fn(*mut colr_LayerList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut colr_LayerList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut colr_LayerList, colr_Layer) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut colr_LayerList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut colr_LayerList) -> colr_Layer>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut colr_LayerList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut colr_LayerList,
            Option<unsafe extern "C" fn(*const colr_Layer, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut colr_LayerList,
            Option<unsafe extern "C" fn(*const colr_Layer, *const colr_Layer) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_colr_Mapping {
    pub init: Option<unsafe extern "C" fn(*mut colr_Mapping) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut colr_Mapping, *const colr_Mapping) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut colr_Mapping, *mut colr_Mapping) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut colr_Mapping) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut colr_Mapping, colr_Mapping) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut colr_Mapping, colr_Mapping) -> ()>,
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
pub struct __caryll_elementinterface_tsi_Entry {
    pub init: Option<unsafe extern "C" fn(*mut tsi_Entry) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut tsi_Entry, *const tsi_Entry) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut tsi_Entry, *mut tsi_Entry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut tsi_Entry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut tsi_Entry, tsi_Entry) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut tsi_Entry, tsi_Entry) -> ()>,
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
pub type subtable_remover = Option<unsafe extern "C" fn(*mut otl_Subtable) -> ()>;
pub type otl_consolidation_function = Option<
    unsafe extern "C" fn(
        *mut otfcc_Font,
        *mut table_OTL,
        *mut otl_Subtable,
        *const otfcc_Options,
    ) -> bool,
>;
pub type fd_handle = otfcc_FDHandle;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn by_stem_pos(
    mut a: *const glyf_PostscriptStemDef,
    mut b: *const glyf_PostscriptStemDef,
) -> ::core::ffi::c_int {
    if (*a).position == (*b).position {
        return (*a).map as ::core::ffi::c_int - (*b).map as ::core::ffi::c_int;
    } else if (*a).position > (*b).position {
        return 1 as ::core::ffi::c_int;
    } else {
        return -(1 as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn by_mask_pointindex(
    mut a: *const glyf_PostscriptHintMask,
    mut b: *const glyf_PostscriptHintMask,
) -> ::core::ffi::c_int {
    return if (*a).contoursBefore as ::core::ffi::c_int == (*b).contoursBefore as ::core::ffi::c_int
    {
        (*a).pointsBefore as ::core::ffi::c_int - (*b).pointsBefore as ::core::ffi::c_int
    } else {
        (*a).contoursBefore as ::core::ffi::c_int - (*b).contoursBefore as ::core::ffi::c_int
    };
}
unsafe extern "C" fn consolidateGlyphContours(
    mut g: *mut glyf_Glyph,
    mut options: *const otfcc_Options,
) {
    let mut nContoursConsolidated: shapeid_t = 0 as shapeid_t;
    let mut skip: shapeid_t = 0 as shapeid_t;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as usize) < (*g).contours.length {
        if (*(*g).contours.items.offset(j as isize)).length != 0 {
            *(*g)
                .contours
                .items
                .offset((j as ::core::ffi::c_int - skip as ::core::ffi::c_int) as isize) =
                *(*g).contours.items.offset(j as isize);
            nContoursConsolidated = (nContoursConsolidated as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as shapeid_t;
        } else {
            glyf_iContourList
                .disposeItem
                .expect("non-null function pointer")(
                &raw mut (*g).contours, j as usize
            );
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"[Consolidate] Removed empty contour #%d in glyph %s.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    j as ::core::ffi::c_int,
                    (*g).name,
                ),
            );
            skip = (skip as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
        }
        j = j.wrapping_add(1);
    }
    (*g).contours.length = nContoursConsolidated as usize;
}
unsafe extern "C" fn consolidateGlyphReferences(
    mut g: *mut glyf_Glyph,
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    let mut nReferencesConsolidated: shapeid_t = 0 as shapeid_t;
    let mut skip: shapeid_t = 0 as shapeid_t;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as usize) < (*g).references.length {
        if !otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*g).references.items.offset(j as isize)).glyph,
        ) {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"[Consolidate] Ignored absent glyph component reference /%s within /%s.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (*(*g).references.items.offset(j as isize)).glyph.name,
                    (*g).name,
                ),
            );
            glyf_iReferenceList
                .disposeItem
                .expect("non-null function pointer")(
                &raw mut (*g).references, j as usize
            );
            skip = (skip as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
        } else {
            *(*g)
                .references
                .items
                .offset((j as ::core::ffi::c_int - skip as ::core::ffi::c_int) as isize) =
                *(*g).references.items.offset(j as isize);
            nReferencesConsolidated = (nReferencesConsolidated as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as shapeid_t;
        }
        j = j.wrapping_add(1);
    }
    (*g).references.length = nReferencesConsolidated as usize;
}
unsafe extern "C" fn consolidateGlyphHints(
    mut g: *mut glyf_Glyph,
    mut _options: *const otfcc_Options,
) {
    if (*g).stemH.length != 0 {
        let mut j: shapeid_t = 0 as shapeid_t;
        while (j as usize) < (*g).stemH.length {
            (*(*g).stemH.items.offset(j as isize)).map = j as u16;
            j = j.wrapping_add(1);
        }
        glyf_iStemDefList.sort.expect("non-null function pointer")(
            &raw mut (*g).stemH,
            Some(
                by_stem_pos
                    as unsafe extern "C" fn(
                        *const glyf_PostscriptStemDef,
                        *const glyf_PostscriptStemDef,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    if (*g).stemV.length != 0 {
        let mut j_0: shapeid_t = 0 as shapeid_t;
        while (j_0 as usize) < (*g).stemV.length {
            (*(*g).stemV.items.offset(j_0 as isize)).map = j_0 as u16;
            j_0 = j_0.wrapping_add(1);
        }
        glyf_iStemDefList.sort.expect("non-null function pointer")(
            &raw mut (*g).stemV,
            Some(
                by_stem_pos
                    as unsafe extern "C" fn(
                        *const glyf_PostscriptStemDef,
                        *const glyf_PostscriptStemDef,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    let mut hmap: *mut shapeid_t = ::core::ptr::null_mut::<shapeid_t>();
    hmap = __caryll_allocate_clean(
        (::core::mem::size_of::<shapeid_t>() as usize).wrapping_mul((*g).stemH.length),
        80 as ::core::ffi::c_ulong,
    ) as *mut shapeid_t;
    let mut vmap: *mut shapeid_t = ::core::ptr::null_mut::<shapeid_t>();
    vmap = __caryll_allocate_clean(
        (::core::mem::size_of::<shapeid_t>() as usize).wrapping_mul((*g).stemV.length),
        82 as ::core::ffi::c_ulong,
    ) as *mut shapeid_t;
    let mut j_1: shapeid_t = 0 as shapeid_t;
    while (j_1 as usize) < (*g).stemH.length {
        *hmap.offset((*(*g).stemH.items.offset(j_1 as isize)).map as isize) = j_1;
        j_1 = j_1.wrapping_add(1);
    }
    let mut j_2: shapeid_t = 0 as shapeid_t;
    while (j_2 as usize) < (*g).stemV.length {
        *vmap.offset((*(*g).stemV.items.offset(j_2 as isize)).map as isize) = j_2;
        j_2 = j_2.wrapping_add(1);
    }
    if (*g).hintMasks.length != 0 {
        glyf_iMaskList.sort.expect("non-null function pointer")(
            &raw mut (*g).hintMasks,
            Some(
                by_mask_pointindex
                    as unsafe extern "C" fn(
                        *const glyf_PostscriptHintMask,
                        *const glyf_PostscriptHintMask,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut j_3: shapeid_t = 0 as shapeid_t;
        while (j_3 as usize) < (*g).hintMasks.length {
            let mut oldmask: glyf_PostscriptHintMask = *(*g).hintMasks.items.offset(j_3 as isize);
            let mut k: shapeid_t = 0 as shapeid_t;
            while (k as usize) < (*g).stemH.length {
                (*(*g).hintMasks.items.offset(j_3 as isize)).maskH[k as usize] =
                    oldmask.maskH[*hmap.offset(k as isize) as usize];
                k = k.wrapping_add(1);
            }
            let mut k_0: shapeid_t = 0 as shapeid_t;
            while (k_0 as usize) < (*g).stemV.length {
                (*(*g).hintMasks.items.offset(j_3 as isize)).maskV[k_0 as usize] =
                    oldmask.maskV[*vmap.offset(k_0 as isize) as usize];
                k_0 = k_0.wrapping_add(1);
            }
            j_3 = j_3.wrapping_add(1);
        }
    }
    if (*g).contourMasks.length != 0 {
        glyf_iMaskList.sort.expect("non-null function pointer")(
            &raw mut (*g).contourMasks,
            Some(
                by_mask_pointindex
                    as unsafe extern "C" fn(
                        *const glyf_PostscriptHintMask,
                        *const glyf_PostscriptHintMask,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut j_4: shapeid_t = 0 as shapeid_t;
        while (j_4 as usize) < (*g).contourMasks.length {
            let mut oldmask_0: glyf_PostscriptHintMask =
                *(*g).contourMasks.items.offset(j_4 as isize);
            let mut k_1: shapeid_t = 0 as shapeid_t;
            while (k_1 as usize) < (*g).stemH.length {
                (*(*g).contourMasks.items.offset(j_4 as isize)).maskH[k_1 as usize] =
                    oldmask_0.maskH[*hmap.offset(k_1 as isize) as usize];
                k_1 = k_1.wrapping_add(1);
            }
            let mut k_2: shapeid_t = 0 as shapeid_t;
            while (k_2 as usize) < (*g).stemV.length {
                (*(*g).contourMasks.items.offset(j_4 as isize)).maskV[k_2 as usize] =
                    oldmask_0.maskV[*vmap.offset(k_2 as isize) as usize];
                k_2 = k_2.wrapping_add(1);
            }
            j_4 = j_4.wrapping_add(1);
        }
    }
    free(hmap as *mut ::core::ffi::c_void);
    hmap = ::core::ptr::null_mut::<shapeid_t>();
    free(vmap as *mut ::core::ffi::c_void);
    vmap = ::core::ptr::null_mut::<shapeid_t>();
}
unsafe extern "C" fn consolidateFDSelect(
    mut h: *mut fd_handle,
    mut cff: *mut table_CFF,
    mut options: *const otfcc_Options,
    gname: sds,
) {
    if cff.is_null() || (*cff).fdArray.is_null() || (*cff).fdArrayCount == 0 {
        return;
    }
    if (*h).state as ::core::ffi::c_uint
        == HANDLE_STATE_INDEX as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*h).index as ::core::ffi::c_int >= (*cff).fdArrayCount as ::core::ffi::c_int {
            (*h).index = 0 as glyphid_t;
        }
        handle_consolidateTo(
            h as *mut otfcc_Handle,
            (*h).index,
            (**(*cff).fdArray.offset((*h).index as isize)).fontName,
        );
    } else if !(*h).name.is_null() {
        let mut found: bool = false;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*cff).fdArrayCount as ::core::ffi::c_int {
            if strcmp(
                (*h).name as *const ::core::ffi::c_char,
                (**(*cff).fdArray.offset(j as isize)).fontName as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                found = true;
                handle_consolidateTo(
                    h as *mut otfcc_Handle,
                    j as glyphid_t,
                    (**(*cff).fdArray.offset(j as isize)).fontName,
                );
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        if !found {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"[Consolidate] CID Subfont %s is not defined. (in glyph /%s).\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*h).name,
                    gname,
                ),
            );
            otfcc_Handle_dispose(h as *mut otfcc_Handle);
        }
    } else if !(*h).name.is_null() {
        otfcc_Handle_dispose(h as *mut otfcc_Handle);
    }
}
#[no_mangle]
pub unsafe extern "C" fn consolidateGlyph(
    mut g: *mut glyf_Glyph,
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    consolidateGlyphContours(g, options);
    consolidateGlyphReferences(g, font, options);
    consolidateGlyphHints(g, options);
    consolidateFDSelect(&raw mut (*g).fdSelect, (*font).CFF_, options, (*g).name);
}
#[no_mangle]
pub unsafe extern "C" fn getPointCoordinates(
    mut table: *mut table_glyf,
    mut gr: *mut glyf_ComponentReference,
    mut n: shapeid_t,
    mut stated: *mut shapeid_t,
    mut x: *mut VQ,
    mut y: *mut VQ,
    mut options: *const otfcc_Options,
) -> bool {
    let mut j: glyphid_t = (*gr).glyph.index;
    let mut g: *mut glyf_Glyph = *(*table).items.offset(j as isize) as *mut glyf_Glyph;
    let mut c: shapeid_t = 0 as shapeid_t;
    while (c as usize) < (*g).contours.length {
        let mut pj: shapeid_t = 0 as shapeid_t;
        while (pj as usize) < (*(*g).contours.items.offset(c as isize)).length {
            if *stated as ::core::ffi::c_int == n as ::core::ffi::c_int {
                let mut p: *mut glyf_Point = (*(*g).contours.items.offset(c as isize))
                    .items
                    .offset(pj as isize)
                    as *mut glyf_Point;
                iVQ.replace.expect("non-null function pointer")(
                    x,
                    iVQ.pointLinearTfm.expect("non-null function pointer")(
                        (*gr).x,
                        (*gr).a as pos_t,
                        (*p).x,
                        (*gr).b as pos_t,
                        (*p).y,
                    ) as VQ,
                );
                iVQ.replace.expect("non-null function pointer")(
                    y,
                    iVQ.pointLinearTfm.expect("non-null function pointer")(
                        (*gr).y,
                        (*gr).c as pos_t,
                        (*p).x,
                        (*gr).d as pos_t,
                        (*p).y,
                    ) as VQ,
                );
                return true;
            }
            *stated = (*stated as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
            pj = pj.wrapping_add(1);
        }
        c = c.wrapping_add(1);
    }
    let mut r: shapeid_t = 0 as shapeid_t;
    while (r as usize) < (*g).references.length {
        let mut rr: *mut glyf_ComponentReference =
            (*g).references.items.offset(r as isize) as *mut glyf_ComponentReference;
        consolidateAnchorRef(table, gr, rr, options);
        let mut ref_0: glyf_ComponentReference =
            (
                glyf_iComponentReference
                    .empty
                    .expect("non-null function pointer"))();
        ref_0.glyph = handle_fromIndex(
            (*(*g).references.items.offset(r as isize)).glyph.index,
        ) as otfcc_GlyphHandle;
        ref_0.a = (*gr).a * (*rr).a + (*rr).b * (*gr).c;
        ref_0.b = (*rr).a * (*gr).b + (*rr).b * (*gr).d;
        ref_0.c = (*gr).a * (*rr).c + (*gr).c * (*rr).d;
        ref_0.d = (*gr).b * (*rr).c + (*rr).d * (*gr).d;
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.x,
            iVQ.pointLinearTfm.expect("non-null function pointer")(
                (*rr).x,
                (*rr).a as pos_t,
                (*gr).x,
                (*rr).b as pos_t,
                (*gr).y,
            ) as VQ,
        );
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.y,
            iVQ.pointLinearTfm.expect("non-null function pointer")(
                (*rr).y,
                (*rr).c as pos_t,
                (*gr).x,
                (*rr).d as pos_t,
                (*gr).y,
            ) as VQ,
        );
        let mut success: bool =
            getPointCoordinates(table, &raw mut ref_0, n, stated, x, y, options);
        glyf_iComponentReference
            .dispose
            .expect("non-null function pointer")(&raw mut ref_0);
        if success {
            return true;
        }
        r = r.wrapping_add(1);
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn consolidateAnchorRef(
    mut table: *mut table_glyf,
    mut gr: *mut glyf_ComponentReference,
    mut rr: *mut glyf_ComponentReference,
    mut options: *const otfcc_Options,
) -> bool {
    if (*rr).isAnchored as ::core::ffi::c_uint
        == REF_ANCHOR_CONSOLIDATED as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*rr).isAnchored as ::core::ffi::c_uint
            == REF_XY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true;
    }
    if (*rr).isAnchored as ::core::ffi::c_uint
        == REF_ANCHOR_CONSOLIDATING_ANCHOR as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*rr).isAnchored as ::core::ffi::c_uint
            == REF_ANCHOR_CONSOLIDATING_XY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            sdscatprintf(
                sdsempty(),
                b"Found circular reference of out-of-range point reference in anchored reference.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        (*rr).isAnchored = REF_XY;
        return false;
    }
    if (*rr).isAnchored as ::core::ffi::c_uint
        == REF_ANCHOR_ANCHOR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*rr).isAnchored = REF_ANCHOR_CONSOLIDATING_ANCHOR;
    } else {
        (*rr).isAnchored = REF_ANCHOR_CONSOLIDATING_XY;
    }
    let mut innerX: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut outerX: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut innerY: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut outerY: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut innerCounter: shapeid_t = 0 as shapeid_t;
    let mut outerCounter: shapeid_t = 0 as shapeid_t;
    let mut rr1: glyf_ComponentReference =
        (
            glyf_iComponentReference
                .empty
                .expect("non-null function pointer"))();
    rr1.glyph = handle_fromIndex((*rr).glyph.index)
        as otfcc_GlyphHandle;
    let mut s1: bool = getPointCoordinates(
        table,
        gr,
        (*rr).outer,
        &raw mut outerCounter,
        &raw mut outerX,
        &raw mut outerY,
        options,
    );
    let mut s2: bool = getPointCoordinates(
        table,
        &raw mut rr1,
        (*rr).inner,
        &raw mut innerCounter,
        &raw mut innerX,
        &raw mut innerY,
        options,
    );
    if !s1 {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            sdscatprintf(
                sdsempty(),
                b"Failed to access point %d in outer glyph.\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*rr).outer as ::core::ffi::c_int,
            ),
        );
    }
    if !s2 {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            sdscatprintf(
                sdsempty(),
                b"Failed to access point %d in reference to %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*rr).outer as ::core::ffi::c_int,
                (*rr).glyph.name,
            ),
        );
    }
    let mut rrx: VQ = iVQ.pointLinearTfm.expect("non-null function pointer")(
        outerX,
        -((*rr).a as pos_t),
        innerX,
        -((*rr).b as pos_t),
        innerY,
    );
    let mut rry: VQ = iVQ.pointLinearTfm.expect("non-null function pointer")(
        outerY,
        -((*rr).c as pos_t),
        innerX,
        -((*rr).d as pos_t),
        innerY,
    );
    if (*rr).isAnchored as ::core::ffi::c_uint
        == REF_ANCHOR_CONSOLIDATING_ANCHOR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        iVQ.replace.expect("non-null function pointer")(&raw mut (*rr).x, rrx);
        iVQ.replace.expect("non-null function pointer")(&raw mut (*rr).y, rry);
        (*rr).isAnchored = REF_ANCHOR_CONSOLIDATED;
    } else {
        if fabs(
            iVQ.getStill.expect("non-null function pointer")((*rr).x) as ::core::ffi::c_double
                - iVQ.getStill.expect("non-null function pointer")(rrx) as ::core::ffi::c_double,
        ) > 0.5f64
            && fabs(
                iVQ.getStill.expect("non-null function pointer")((*rr).y) as ::core::ffi::c_double
                    - iVQ.getStill.expect("non-null function pointer")(rry)
                        as ::core::ffi::c_double,
            ) > 0.5f64
        {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"Anchored reference to %s does not match its X/Y offset data.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*rr).glyph.name,
                ),
            );
        }
        (*rr).isAnchored = REF_ANCHOR_CONSOLIDATED;
        iVQ.dispose.expect("non-null function pointer")(&raw mut rrx);
        iVQ.dispose.expect("non-null function pointer")(&raw mut rry);
    }
    glyf_iComponentReference
        .dispose
        .expect("non-null function pointer")(&raw mut rr1);
    iVQ.dispose.expect("non-null function pointer")(&raw mut innerX);
    iVQ.dispose.expect("non-null function pointer")(&raw mut innerY);
    iVQ.dispose.expect("non-null function pointer")(&raw mut outerX);
    iVQ.dispose.expect("non-null function pointer")(&raw mut outerY);
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn consolidateGlyf(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    if (*font).glyph_order.is_null() || (*font).glyf.is_null() {
        return;
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*(*font).glyf).length {
        if !(*(*(*font).glyf).items.offset(j as isize)).is_null() {
            consolidateGlyph(
                *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph,
                font,
                options,
            );
        } else {
            let ref mut fresh6 = *(*(*font).glyf).items.offset(j as isize);
            *fresh6 = otfcc_newGlyf_glyph() as glyf_GlyphPtr;
        }
        j = j.wrapping_add(1);
    }
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as usize) < (*(*font).glyf).length {
        let mut g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j_0 as isize) as *mut glyf_Glyph;
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*g).name,
            ),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut gr: glyf_ComponentReference =
                (
                    glyf_iComponentReference
                        .empty
                        .expect("non-null function pointer"))();
            gr.glyph = handle_fromIndex(j_0)
                as otfcc_GlyphHandle;
            let mut r: shapeid_t = 0 as shapeid_t;
            while (r as usize) < (*g).references.length {
                let mut rr: *mut glyf_ComponentReference =
                    (*g).references.items.offset(r as isize) as *mut glyf_ComponentReference;
                consolidateAnchorRef((*font).glyf, &raw mut gr, rr, options);
                r = r.wrapping_add(1);
            }
            glyf_iComponentReference
                .dispose
                .expect("non-null function pointer")(&raw mut gr);
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        j_0 = j_0.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn consolidateCmap(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    if !(*font).glyph_order.is_null() && !(*font).cmap.is_null() {
        let mut item: *mut cmap_Entry = ::core::ptr::null_mut::<cmap_Entry>();
        item = (*(*font).cmap).unicodes;
        while !item.is_null() {
            if !otfcc_pkgGlyphOrder
                .consolidateHandle
                .expect("non-null function pointer")(
                (*font).glyph_order, &raw mut (*item).glyph
            ) {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[Consolidate] Ignored mapping U+%04X to non-existent glyph /%s.\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        (*item).unicode,
                        (*item).glyph.name,
                    ),
                );
                otfcc_Handle_dispose(&raw mut (*item).glyph);
            }
            item = (*item).hh.next as *mut cmap_Entry;
        }
    }
    if !(*font).glyph_order.is_null() && !(*font).cmap.is_null() {
        let mut item_0: *mut cmap_UVS_Entry = ::core::ptr::null_mut::<cmap_UVS_Entry>();
        item_0 = (*(*font).cmap).uvs;
        while !item_0.is_null() {
            if !otfcc_pkgGlyphOrder
                .consolidateHandle
                .expect("non-null function pointer")(
                (*font).glyph_order, &raw mut (*item_0).glyph
            ) {
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
                        b"[Consolidate] Ignored UVS mapping [U+%04X U+%04X] to non-existent glyph /%s.\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        (*item_0).key.unicode,
                        (*item_0).key.selector,
                        (*item_0).glyph.name,
                    ),
                );
                otfcc_Handle_dispose(&raw mut (*item_0).glyph);
            }
            item_0 = (*item_0).hh.next as *mut cmap_UVS_Entry;
        }
    }
}
unsafe extern "C" fn __declare_otl_consolidation(
    mut type_0: otl_LookupType,
    mut fn_0: otl_consolidation_function,
    mut fndel: subtable_remover,
    mut font: *mut otfcc_Font,
    mut table: *mut table_OTL,
    mut lookup: *mut otl_Lookup,
    mut options: *const otfcc_Options,
) {
    if lookup.is_null()
        || (*lookup).subtables.length == 0
        || (*lookup).type_0 as ::core::ffi::c_uint != type_0 as ::core::ffi::c_uint
    {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            (*lookup).name,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: tableid_t = 0 as tableid_t;
        while (j as usize) < (*lookup).subtables.length {
            if (*(*lookup).subtables.items.offset(j as isize)).is_null() {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[Consolidate] Ignored empty subtable %d of lookup %s.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        j as ::core::ffi::c_int,
                        (*lookup).name,
                    ),
                );
            } else {
                let mut subtableRemoved: bool = false;
                subtableRemoved = fn_0.expect("non-null function pointer")(
                    font,
                    table,
                    *(*lookup).subtables.items.offset(j as isize) as *mut otl_Subtable,
                    options,
                );
                if subtableRemoved {
                    fndel.expect("non-null function pointer")(
                        *(*lookup).subtables.items.offset(j as isize) as *mut otl_Subtable,
                    );
                    let ref mut fresh3 = *(*lookup).subtables.items.offset(j as isize);
                    *fresh3 = ::core::ptr::null_mut::<otl_Subtable>();
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        sdscatprintf(
                            sdsempty(),
                            b"[Consolidate] Ignored empty subtable %d of lookup %s.\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            j as ::core::ffi::c_int,
                            (*lookup).name,
                        ),
                    );
                }
            }
            j = j.wrapping_add(1);
        }
        let mut k: tableid_t = 0 as tableid_t;
        let mut j_0: tableid_t = 0 as tableid_t;
        while (j_0 as usize) < (*lookup).subtables.length {
            if !(*(*lookup).subtables.items.offset(j_0 as isize)).is_null() {
                let fresh4 = k;
                k = k.wrapping_add(1);
                let ref mut fresh5 = *(*lookup).subtables.items.offset(fresh4 as isize);
                *fresh5 = *(*lookup).subtables.items.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
        }
        (*lookup).subtables.length = k as usize;
        if k == 0 {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"[Consolidate] Lookup %s is empty and will be removed.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*lookup).name,
                ),
            );
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_consolidate_lookup(
    mut font: *mut otfcc_Font,
    mut table: *mut table_OTL,
    mut lookup: *mut otl_Lookup,
    mut options: *const otfcc_Options,
) {
    __declare_otl_consolidation(
        otl_type_gsub_single,
        Some(
            consolidate_gsub_single
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gsub_single) -> ()>,
            subtable_remover,
        >(iSubtable_gsub_single.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gsub_multiple,
        Some(
            consolidate_gsub_multi
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
            subtable_remover,
        >(iSubtable_gsub_multi.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gsub_alternate,
        Some(
            consolidate_gsub_alternative
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gsub_multi) -> ()>,
            subtable_remover,
        >(iSubtable_gsub_multi.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gsub_ligature,
        Some(
            consolidate_gsub_ligature
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gsub_ligature) -> ()>,
            subtable_remover,
        >(iSubtable_gsub_ligature.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gsub_chaining,
        Some(
            consolidate_chaining
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
            subtable_remover,
        >(iSubtable_chaining.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gsub_reverse,
        Some(
            consolidate_gsub_reverse
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gsub_reverse) -> ()>,
            subtable_remover,
        >(iSubtable_gsub_reverse.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_single,
        Some(
            consolidate_gpos_single
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_single) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_single.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_pair,
        Some(
            consolidate_gpos_pair
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_pair.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_cursive,
        Some(
            consolidate_gpos_cursive
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_cursive) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_cursive.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_chaining,
        Some(
            consolidate_chaining
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_chaining) -> ()>,
            subtable_remover,
        >(iSubtable_chaining.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_markToBase,
        Some(
            consolidate_mark_to_single
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_markToSingle.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_markToMark,
        Some(
            consolidate_mark_to_single
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_markToSingle) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_markToSingle.free),
        font,
        table,
        lookup,
        options,
    );
    __declare_otl_consolidation(
        otl_type_gpos_markToLigature,
        Some(
            consolidate_mark_to_ligature
                as unsafe extern "C" fn(
                    *mut otfcc_Font,
                    *mut table_OTL,
                    *mut otl_Subtable,
                    *const otfcc_Options,
                ) -> bool,
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut subtable_gpos_markToLigature) -> ()>,
            subtable_remover,
        >(iSubtable_gpos_markToLigature.free),
        font,
        table,
        lookup,
        options,
    );
}
unsafe extern "C" fn lookupRefIsNotEmpty(
    mut rLut: *const otl_LookupRef,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !rLut.is_null() && !(*rLut).is_null() && (**rLut).subtables.length > 0 as usize;
}
unsafe extern "C" fn featureRefIsNotEmpty(
    mut rFeat: *const otl_FeatureRef,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !rFeat.is_null() && !(*rFeat).is_null() && (**rFeat).lookups.length > 0 as usize;
}
unsafe extern "C" fn lookupIsNotEmpty(
    mut rLut: *const otl_LookupPtr,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !rLut.is_null() && !(*rLut).is_null() && (**rLut).subtables.length > 0 as usize;
}
unsafe extern "C" fn featureIsNotEmpty(
    mut rFeat: *const otl_FeaturePtr,
    mut _env: *mut ::core::ffi::c_void,
) -> bool {
    return !rFeat.is_null() && !(*rFeat).is_null() && (**rFeat).lookups.length > 0 as usize;
}
unsafe extern "C" fn consolidateOTLTable(
    mut font: *mut otfcc_Font,
    mut table: *mut table_OTL,
    mut options: *const otfcc_Options,
) {
    if (*font).glyph_order.is_null() || table.is_null() {
        return;
    }
    loop {
        let mut featN: tableid_t = (*table).features.length as tableid_t;
        let mut lutN: tableid_t = (*table).lookups.length as tableid_t;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as usize) < (*table).lookups.length {
            otfcc_consolidate_lookup(
                font,
                table,
                *(*table).lookups.items.offset(j as isize) as *mut otl_Lookup,
                options,
            );
            j = j.wrapping_add(1);
        }
        let mut j_0: tableid_t = 0 as tableid_t;
        while (j_0 as usize) < (*table).features.length {
            let mut feature: *mut otl_Feature =
                *(*table).features.items.offset(j_0 as isize) as *mut otl_Feature;
            otl_iLookupRefList
                .filterEnv
                .expect("non-null function pointer")(
                &raw mut (*feature).lookups,
                Some(
                    lookupRefIsNotEmpty
                        as unsafe extern "C" fn(
                            *const otl_LookupRef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                ),
                NULL,
            );
            j_0 = j_0.wrapping_add(1);
        }
        let mut j_1: tableid_t = 0 as tableid_t;
        while (j_1 as usize) < (*table).languages.length {
            let mut lang: *mut otl_LanguageSystem =
                *(*table).languages.items.offset(j_1 as isize) as *mut otl_LanguageSystem;
            otl_iFeatureRefList
                .filterEnv
                .expect("non-null function pointer")(
                &raw mut (*lang).features,
                Some(
                    featureRefIsNotEmpty
                        as unsafe extern "C" fn(
                            *const otl_FeatureRef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                ),
                NULL,
            );
            j_1 = j_1.wrapping_add(1);
        }
        otl_iLookupList
            .filterEnv
            .expect("non-null function pointer")(
            &raw mut (*table).lookups,
            Some(
                lookupIsNotEmpty
                    as unsafe extern "C" fn(*const otl_LookupPtr, *mut ::core::ffi::c_void) -> bool,
            ),
            NULL,
        );
        otl_iFeatureList
            .filterEnv
            .expect("non-null function pointer")(
            &raw mut (*table).features,
            Some(
                featureIsNotEmpty
                    as unsafe extern "C" fn(
                        *const otl_FeaturePtr,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
            ),
            NULL,
        );
        let mut featN1: tableid_t = (*table).features.length as tableid_t;
        let mut lutN1: tableid_t = (*table).lookups.length as tableid_t;
        if featN1 as ::core::ffi::c_int >= featN as ::core::ffi::c_int
            && lutN1 as ::core::ffi::c_int >= lutN as ::core::ffi::c_int
        {
            break;
        }
    }
}
unsafe extern "C" fn consolidateOTL(mut font: *mut otfcc_Font, mut options: *const otfcc_Options) {
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        consolidateOTLTable(font, (*font).GSUB, options);
        ___loggedstep_v = false;
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
            b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        consolidateOTLTable(font, (*font).GPOS, options);
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
            b"GDEF\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        consolidate_GDEF(font, (*font).GDEF, options);
        ___loggedstep_v_1 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
unsafe extern "C" fn consolidateCOLR(mut font: *mut otfcc_Font, mut options: *const otfcc_Options) {
    if font.is_null() || (*font).COLR.is_null() || (*font).glyph_order.is_null() {
        return;
    }
    let mut consolidated: *mut table_COLR = (
        table_iCOLR.create.expect("non-null function pointer"))();
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*(*font).COLR).length {
        let mut mapping: *mut colr_Mapping = (*(*font).COLR).items.offset(__caryll_index as isize);
        while keep != 0 {
            if !otfcc_pkgGlyphOrder
                .consolidateHandle
                .expect("non-null function pointer")(
                (*font).glyph_order, &raw mut (*mapping).glyph
            ) {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[Consolidate] Ignored missing glyph of /%s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*mapping).glyph.name,
                    ),
                );
            } else {
                let mut m: colr_Mapping = colr_Mapping {
                    glyph: otfcc_Handle {
                        state: HANDLE_STATE_EMPTY,
                        index: 0,
                        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    },
                    layers: colr_LayerList {
                        length: 0,
                        capacity: 0,
                        items: ::core::ptr::null_mut::<colr_Layer>(),
                    },
                };
                otfcc_Handle_copy(
                    &raw mut m.glyph,
                    &raw mut (*mapping).glyph,
                );
                colr_iLayerList.init.expect("non-null function pointer")(&raw mut m.layers);
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < (*mapping).layers.length {
                    let mut layer: *mut colr_Layer =
                        (*mapping).layers.items.offset(__caryll_index_0 as isize);
                    while keep_0 != 0 {
                        if !otfcc_pkgGlyphOrder
                            .consolidateHandle
                            .expect("non-null function pointer")(
                            (*font).glyph_order,
                            &raw mut (*layer).glyph,
                        ) {
                            (*(*options).logger)
                                .logSDS
                                .expect("non-null function pointer")(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                sdscatprintf(
                                    sdsempty(),
                                    b"[Consolidate] Ignored missing glyph of /%s\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    (*layer).glyph.name,
                                ),
                            );
                        } else {
                            let mut layer1: colr_Layer = colr_Layer {
                                glyph: otfcc_Handle {
                                    state: HANDLE_STATE_EMPTY,
                                    index: 0,
                                    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                },
                                paletteIndex: 0,
                            };
                            colr_iLayer.copy.expect("non-null function pointer")(
                                &raw mut layer1,
                                layer,
                            );
                            colr_iLayerList.push.expect("non-null function pointer")(
                                &raw mut m.layers,
                                layer1,
                            );
                        }
                        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    }
                    keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    __caryll_index_0 = __caryll_index_0.wrapping_add(1);
                }
                if (*mapping).layers.length != 0 {
                    table_iCOLR.push.expect("non-null function pointer")(consolidated, m);
                } else {
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        sdscatprintf(
                            sdsempty(),
                            b"[Consolidate] COLR decomposition for /%s is empth\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*mapping).glyph.name,
                        ),
                    );
                    colr_iMapping.dispose.expect("non-null function pointer")(&raw mut m);
                }
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    table_iCOLR.free.expect("non-null function pointer")((*font).COLR);
    (*font).COLR = consolidated;
}
unsafe extern "C" fn compareTSIEntry(
    mut a: *const tsi_Entry,
    mut b: *const tsi_Entry,
) -> ::core::ffi::c_int {
    if (*a).type_0 as ::core::ffi::c_uint != (*b).type_0 as ::core::ffi::c_uint {
        return ((*a).type_0 as ::core::ffi::c_uint).wrapping_sub((*b).type_0 as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    return (*a).glyph.index as ::core::ffi::c_int - (*b).glyph.index as ::core::ffi::c_int;
}
unsafe extern "C" fn consolidateTSI(
    mut font: *mut otfcc_Font,
    mut _tsi: *mut *mut table_TSI,
    mut options: *const otfcc_Options,
) {
    let mut tsi: *mut table_TSI = *_tsi;
    if font.is_null() || (*font).glyf.is_null() || tsi.is_null() || (*font).glyph_order.is_null() {
        return;
    }
    let mut consolidated: *mut table_TSI = (
        table_iTSI.create.expect("non-null function pointer"))();
    let mut gidEntries: *mut sds = ::core::ptr::null_mut::<sds>();
    gidEntries = __caryll_allocate_clean(
        (::core::mem::size_of::<sds>() as usize).wrapping_mul((*(*font).glyf).length),
        448 as ::core::ffi::c_ulong,
    ) as *mut sds;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*tsi).length {
        let mut entry: *mut tsi_Entry = (*tsi).items.offset(__caryll_index as isize);
        while keep != 0 {
            if (*entry).type_0 as ::core::ffi::c_uint
                == TSI_GLYPH as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if otfcc_pkgGlyphOrder
                    .consolidateHandle
                    .expect("non-null function pointer")(
                    (*font).glyph_order,
                    &raw mut (*entry).glyph,
                ) {
                    if !(*gidEntries.offset((*entry).glyph.index as isize)).is_null() {
                        sdsfree(*gidEntries.offset((*entry).glyph.index as isize));
                    }
                    let ref mut fresh2 = *gidEntries.offset((*entry).glyph.index as isize);
                    *fresh2 = (*entry).content;
                    (*entry).content = ::core::ptr::null_mut::<::core::ffi::c_char>();
                } else {
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        sdscatprintf(
                            sdsempty(),
                            b"[Consolidate] Ignored missing glyph of /%s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*entry).glyph.name,
                        ),
                    );
                }
            } else {
                let mut e: tsi_Entry = tsi_Entry {
                    type_0: TSI_GLYPH,
                    glyph: otfcc_Handle {
                        state: HANDLE_STATE_EMPTY,
                        index: 0,
                        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    },
                    content: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                };
                tsi_iEntry.copy.expect("non-null function pointer")(&raw mut e, entry);
                table_iTSI.push.expect("non-null function pointer")(consolidated, e);
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*(*font).glyf).length {
        let mut e_0: tsi_Entry = tsi_Entry {
            type_0: TSI_GLYPH,
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            content: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        e_0.type_0 = TSI_GLYPH;
        e_0.glyph =
            handle_fromIndex(j) as otfcc_GlyphHandle;
        otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")((*font).glyph_order, &raw mut e_0.glyph);
        e_0.content = if !(*gidEntries.offset(j as isize)).is_null() {
            *gidEntries.offset(j as isize)
        } else {
            sdsempty()
        };
        table_iTSI.push.expect("non-null function pointer")(consolidated, e_0);
        j = j.wrapping_add(1);
    }
    table_iTSI.free.expect("non-null function pointer")(tsi);
    free(gidEntries as *mut ::core::ffi::c_void);
    gidEntries = ::core::ptr::null_mut::<sds>();
    table_iTSI.sort.expect("non-null function pointer")(
        consolidated,
        Some(
            compareTSIEntry
                as unsafe extern "C" fn(*const tsi_Entry, *const tsi_Entry) -> ::core::ffi::c_int,
        ),
    );
    *_tsi = consolidated;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_consolidateFont(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    if !(*font).glyf.is_null() && (*font).glyph_order.is_null() {
        let mut go: *mut otfcc_GlyphOrder =
            (
                otfcc_pkgGlyphOrder
                    .create
                    .expect("non-null function pointer"))();
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as usize) < (*(*font).glyf).length {
            let mut name: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut glyfName: sds = (**(*(*font).glyf).items.offset(j as isize)).name;
            if !glyfName.is_null() {
                name = sdsdup(glyfName);
            } else {
                name = sdscatprintf(
                    sdsempty(),
                    b"$$gid%d\0" as *const u8 as *const ::core::ffi::c_char,
                    j as ::core::ffi::c_int,
                );
                let ref mut fresh0 = (**(*(*font).glyf).items.offset(j as isize)).name;
                *fresh0 = sdsdup(name);
            }
            if !otfcc_pkgGlyphOrder
                .setByName
                .expect("non-null function pointer")(go, name, j)
            {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[Consolidate] Glyph name %s is already in use.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        name,
                    ),
                );
                let mut suffix: u32 = 2 as u32;
                let mut success: bool = false;
                loop {
                    let mut newname: sds = sdscatfmt(
                        sdsempty(),
                        b"%s_%u\0" as *const u8 as *const ::core::ffi::c_char,
                        name,
                        suffix,
                    );
                    success = otfcc_pkgGlyphOrder
                        .setByName
                        .expect("non-null function pointer")(
                        go, newname, j
                    );
                    if !success {
                        sdsfree(newname);
                        suffix = suffix.wrapping_add(1 as u32);
                    } else {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"[Consolidate] Glyph %s is renamed into %s.\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                name,
                                newname,
                            ),
                        );
                        sdsfree((**(*(*font).glyf).items.offset(j as isize)).name);
                        let ref mut fresh1 = (**(*(*font).glyf).items.offset(j as isize)).name;
                        *fresh1 = sdsdup(newname);
                    }
                    if success {
                        break;
                    }
                }
                sdsfree(name);
            }
            j = j.wrapping_add(1);
        }
        (*font).glyph_order = go;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        consolidateGlyf(font, options);
        ___loggedstep_v = false;
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
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        consolidateCmap(font, options);
        ___loggedstep_v_0 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    if !(*font).glyf.is_null() {
        consolidateOTL(font, options);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"COLR\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v_1: bool = true;
    while ___loggedstep_v_1 {
        consolidateCOLR(font, options);
        ___loggedstep_v_1 = false;
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
            b"TSI_01\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v_2: bool = true;
    while ___loggedstep_v_2 {
        consolidateTSI(font, &raw mut (*font).TSI_01, options);
        ___loggedstep_v_2 = false;
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
            b"TSI_23\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v_3: bool = true;
    while ___loggedstep_v_3 {
        consolidateTSI(font, &raw mut (*font).TSI_23, options);
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
            b"TSI5\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v_4: bool = true;
    while ___loggedstep_v_4 {
        fontop_consolidateClassDef(font, (*font).TSI5 as *mut otl_ClassDef, options);
        ___loggedstep_v_4 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
