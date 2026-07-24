extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
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
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static otl_iClassDef: __otfcc_IClassDef;
    static otl_iCaretValueList: __caryll_vectorinterface_otl_CaretValueList;
    static otl_iLigCaretTable: __caryll_vectorinterface_otl_LigCaretTable;
    fn fontop_consolidateClassDef(
        font: *mut otfcc_Font,
        cd: *mut otl_ClassDef,
        options: *const otfcc_Options,
    );
}

use crate::src::lib::table::otl::coverage::{otl_Coverage};
use crate::src::lib::support::handle::{handle_fromConsolidated, otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle};
use crate::src::lib::support::stdio::FILE;
use crate::src::lib::support::alloc::{__caryll_allocate_clean};
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
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
    pub u: C2RustUnnamed_4,
    pub _reserved: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub next_alloc: *mut _json_value,
    pub object_mem: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub boolean: ::core::ffi::c_int,
    pub integer: int64_t,
    pub dbl: ::core::ffi::c_double,
    pub string: C2RustUnnamed_7,
    pub object: C2RustUnnamed_6,
    pub array: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub struct C2RustUnnamed_7 {
    pub length: ::core::ffi::c_uint,
    pub ptr: *mut ::core::ffi::c_char,
}
pub type json_value = _json_value;
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
    pub read: Option<unsafe extern "C" fn(*const uint8_t, uint32_t, uint32_t) -> *mut otl_ClassDef>,
    pub expand:
        Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_ClassDef) -> *mut otl_ClassDef>,
    pub dump: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_ClassDef>,
    pub build: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_CaretValueList {
    pub init: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_CaretValueList, *const otl_CaretValueList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_CaretValueList, *mut otl_CaretValueList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_CaretValueList, otl_CaretValueList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_CaretValueList, otl_CaretValueList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_CaretValueList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_CaretValueList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_CaretValueList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut otl_CaretValueList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_CaretValueList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_CaretValueList, otl_CaretValue) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_CaretValueList) -> otl_CaretValue>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_CaretValueList, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_CaretValueList,
            Option<unsafe extern "C" fn(*const otl_CaretValue, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_CaretValueList,
            Option<
                unsafe extern "C" fn(
                    *const otl_CaretValue,
                    *const otl_CaretValue,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_LigCaretTable {
    pub init: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, *const otl_LigCaretTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, *mut otl_LigCaretTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, otl_LigCaretTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, otl_LigCaretTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_LigCaretTable>,
    pub free: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut otl_LigCaretTable>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, otl_CaretValueRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LigCaretTable) -> otl_CaretValueRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LigCaretTable, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_LigCaretTable,
            Option<
                unsafe extern "C" fn(*const otl_CaretValueRecord, *mut ::core::ffi::c_void) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_LigCaretTable,
            Option<
                unsafe extern "C" fn(
                    *const otl_CaretValueRecord,
                    *const otl_CaretValueRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GDEF_ligcaret_hash {
    pub gid: ::core::ffi::c_int,
    pub name: sds,
    pub carets: otl_CaretValueList,
    pub hh: UT_hash_handle,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HASH_INITIAL_NUM_BUCKETS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const HASH_INITIAL_NUM_BUCKETS_LOG2: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const HASH_BKT_CAPACITY_THRESH: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
pub const HASH_SIGNATURE: ::core::ffi::c_uint = 0xa0111fe1 as ::core::ffi::c_uint;
unsafe extern "C" fn by_gid(
    mut a: *mut GDEF_ligcaret_hash,
    mut b: *mut GDEF_ligcaret_hash,
) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
#[no_mangle]
pub unsafe extern "C" fn consolidate_GDEF(
    mut font: *mut otfcc_Font,
    mut gdef: *mut table_GDEF,
    mut options: *const otfcc_Options,
) {
    if font.is_null() || (*font).glyph_order.is_null() || gdef.is_null() {
        return;
    }
    if !(*gdef).glyphClassDef.is_null() {
        fontop_consolidateClassDef(font, (*gdef).glyphClassDef, options);
        otl_iClassDef.shrink.expect("non-null function pointer")((*gdef).glyphClassDef);
        if (*(*gdef).glyphClassDef).numGlyphs == 0 {
            otl_iClassDef.free.expect("non-null function pointer")((*gdef).glyphClassDef);
            (*gdef).glyphClassDef = ::core::ptr::null_mut::<otl_ClassDef>();
        }
    }
    if !(*gdef).markAttachClassDef.is_null() {
        fontop_consolidateClassDef(font, (*gdef).markAttachClassDef, options);
        otl_iClassDef.shrink.expect("non-null function pointer")((*gdef).markAttachClassDef);
        if (*(*gdef).markAttachClassDef).numGlyphs == 0 {
            otl_iClassDef.free.expect("non-null function pointer")((*gdef).markAttachClassDef);
            (*gdef).markAttachClassDef = ::core::ptr::null_mut::<otl_ClassDef>();
        }
    }
    if (*gdef).ligCarets.length != 0 {
        let mut h: *mut GDEF_ligcaret_hash = ::core::ptr::null_mut::<GDEF_ligcaret_hash>();
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as size_t) < (*gdef).ligCarets.length {
            let mut s: *mut GDEF_ligcaret_hash = ::core::ptr::null_mut::<GDEF_ligcaret_hash>();
            if otfcc_pkgGlyphOrder
                .consolidateHandle
                .expect("non-null function pointer")(
                (*font).glyph_order,
                &raw mut (*(*gdef).ligCarets.items.offset(j as isize)).glyph,
            ) {
                let mut gid: ::core::ffi::c_int =
                    (*(*gdef).ligCarets.items.offset(j as isize)).glyph.index as ::core::ffi::c_int;
                let mut gname: sds =
                    sdsdup((*(*gdef).ligCarets.items.offset(j as isize)).glyph.name);
                if !gname.is_null() {
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
                            (*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                    _hf_hashv = _hf_hashv.wrapping_add(
                        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint,
                    );
                    let mut current_block_68: u64;
                    match _hj_k {
                        11 => {
                            _hf_hashv = _hf_hashv.wrapping_add(
                                (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_68 = 7251305533897705623;
                        }
                        10 => {
                            current_block_68 = 7251305533897705623;
                        }
                        9 => {
                            current_block_68 = 645677152134562946;
                        }
                        8 => {
                            current_block_68 = 242026537404353796;
                        }
                        7 => {
                            current_block_68 = 15639471673938373060;
                        }
                        6 => {
                            current_block_68 = 7893160634709431737;
                        }
                        5 => {
                            current_block_68 = 14659199081944334378;
                        }
                        4 => {
                            current_block_68 = 5943912363581501881;
                        }
                        3 => {
                            current_block_68 = 3435367719625643249;
                        }
                        2 => {
                            current_block_68 = 7929974425468050909;
                        }
                        1 => {
                            current_block_68 = 7266796945765332121;
                        }
                        _ => {
                            current_block_68 = 6545907279487748450;
                        }
                    }
                    match current_block_68 {
                        7251305533897705623 => {
                            _hf_hashv = _hf_hashv.wrapping_add(
                                (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_68 = 645677152134562946;
                        }
                        _ => {}
                    }
                    match current_block_68 {
                        645677152134562946 => {
                            _hf_hashv = _hf_hashv.wrapping_add(
                                (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_68 = 242026537404353796;
                        }
                        _ => {}
                    }
                    match current_block_68 {
                        242026537404353796 => {
                            _hj_j = _hj_j.wrapping_add(
                                (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_68 = 15639471673938373060;
                        }
                        _ => {}
                    }
                    match current_block_68 {
                        15639471673938373060 => {
                            _hj_j = _hj_j.wrapping_add(
                                (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_68 = 7893160634709431737;
                        }
                        _ => {}
                    }
                    match current_block_68 {
                        7893160634709431737 => {
                            _hj_j = _hj_j.wrapping_add(
                                (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_68 = 14659199081944334378;
                        }
                        _ => {}
                    }
                    match current_block_68 {
                        14659199081944334378 => {
                            _hj_j = _hj_j
                                .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_68 = 5943912363581501881;
                        }
                        _ => {}
                    }
                    match current_block_68 {
                        5943912363581501881 => {
                            _hj_i = _hj_i.wrapping_add(
                                (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_68 = 3435367719625643249;
                        }
                        _ => {}
                    }
                    match current_block_68 {
                        3435367719625643249 => {
                            _hj_i = _hj_i.wrapping_add(
                                (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_68 = 7929974425468050909;
                        }
                        _ => {}
                    }
                    match current_block_68 {
                        7929974425468050909 => {
                            _hj_i = _hj_i.wrapping_add(
                                (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_68 = 7266796945765332121;
                        }
                        _ => {}
                    }
                    match current_block_68 {
                        7266796945765332121 => {
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
                    s = ::core::ptr::null_mut::<GDEF_ligcaret_hash>();
                    if !h.is_null() {
                        let mut _hf_bkt: ::core::ffi::c_uint = 0;
                        _hf_bkt = _hf_hashv
                            & (*(*h).hh.tbl)
                                .num_buckets
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            if !(*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize))
                                .hh_head
                                .is_null()
                            {
                                s = ((*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                                    as *mut ::core::ffi::c_char)
                                    .offset(-(*(*h).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut GDEF_ligcaret_hash
                                    as *mut GDEF_ligcaret_hash;
                            } else {
                                s = ::core::ptr::null_mut::<GDEF_ligcaret_hash>();
                            }
                            while !s.is_null() {
                                if (*s).hh.hashv == _hf_hashv
                                    && (*s).hh.keylen as usize
                                        == ::core::mem::size_of::<::core::ffi::c_int>()
                                {
                                    if memcmp(
                                        (*s).hh.key,
                                        &raw mut gid as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        break;
                                    }
                                }
                                if !(*s).hh.hh_next.is_null() {
                                    s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                        .offset(-(*(*h).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut GDEF_ligcaret_hash
                                        as *mut GDEF_ligcaret_hash;
                                } else {
                                    s = ::core::ptr::null_mut::<GDEF_ligcaret_hash>();
                                }
                            }
                        }
                    }
                    if s.is_null() {
                        s = __caryll_allocate_clean(
                            ::core::mem::size_of::<GDEF_ligcaret_hash>() as size_t,
                            42 as ::core::ffi::c_ulong,
                        ) as *mut GDEF_ligcaret_hash;
                        (*s).gid = gid;
                        (*s).name = gname;
                        otl_iCaretValueList
                            .move_0
                            .expect("non-null function pointer")(
                            &raw mut (*s).carets,
                            &raw mut (*(*gdef).ligCarets.items.offset(j as isize)).carets,
                        );
                        let mut _ha_hashv: ::core::ffi::c_uint = 0;
                        let mut _hj_i_0: ::core::ffi::c_uint = 0;
                        let mut _hj_j_0: ::core::ffi::c_uint = 0;
                        let mut _hj_k_0: ::core::ffi::c_uint = 0;
                        let mut _hj_key_0: *const ::core::ffi::c_uchar =
                            &raw mut (*s).gid as *const ::core::ffi::c_uchar;
                        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                        _hj_i_0 = _hj_j_0;
                        _hj_k_0 =
                            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
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
                        _ha_hashv =
                            _ha_hashv
                                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>()
                                    as ::core::ffi::c_uint);
                        let mut current_block_186: u64;
                        match _hj_k_0 {
                            11 => {
                                _ha_hashv = _ha_hashv.wrapping_add(
                                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_186 = 14836544894546935381;
                            }
                            10 => {
                                current_block_186 = 14836544894546935381;
                            }
                            9 => {
                                current_block_186 = 7836834032572359738;
                            }
                            8 => {
                                current_block_186 = 9666289021101186870;
                            }
                            7 => {
                                current_block_186 = 1426904578044511456;
                            }
                            6 => {
                                current_block_186 = 5209490962234335381;
                            }
                            5 => {
                                current_block_186 = 3762873336238896710;
                            }
                            4 => {
                                current_block_186 = 14086196355212951910;
                            }
                            3 => {
                                current_block_186 = 11371703062444718518;
                            }
                            2 => {
                                current_block_186 = 13035048525369125056;
                            }
                            1 => {
                                current_block_186 = 11994227959910167019;
                            }
                            _ => {
                                current_block_186 = 13665239467142187023;
                            }
                        }
                        match current_block_186 {
                            14836544894546935381 => {
                                _ha_hashv = _ha_hashv.wrapping_add(
                                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_186 = 7836834032572359738;
                            }
                            _ => {}
                        }
                        match current_block_186 {
                            7836834032572359738 => {
                                _ha_hashv = _ha_hashv.wrapping_add(
                                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_186 = 9666289021101186870;
                            }
                            _ => {}
                        }
                        match current_block_186 {
                            9666289021101186870 => {
                                _hj_j_0 = _hj_j_0.wrapping_add(
                                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_186 = 1426904578044511456;
                            }
                            _ => {}
                        }
                        match current_block_186 {
                            1426904578044511456 => {
                                _hj_j_0 = _hj_j_0.wrapping_add(
                                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_186 = 5209490962234335381;
                            }
                            _ => {}
                        }
                        match current_block_186 {
                            5209490962234335381 => {
                                _hj_j_0 = _hj_j_0.wrapping_add(
                                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_186 = 3762873336238896710;
                            }
                            _ => {}
                        }
                        match current_block_186 {
                            3762873336238896710 => {
                                _hj_j_0 = _hj_j_0.wrapping_add(
                                    *_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint,
                                );
                                current_block_186 = 14086196355212951910;
                            }
                            _ => {}
                        }
                        match current_block_186 {
                            14086196355212951910 => {
                                _hj_i_0 = _hj_i_0.wrapping_add(
                                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_186 = 11371703062444718518;
                            }
                            _ => {}
                        }
                        match current_block_186 {
                            11371703062444718518 => {
                                _hj_i_0 = _hj_i_0.wrapping_add(
                                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_186 = 13035048525369125056;
                            }
                            _ => {}
                        }
                        match current_block_186 {
                            13035048525369125056 => {
                                _hj_i_0 = _hj_i_0.wrapping_add(
                                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_186 = 11994227959910167019;
                            }
                            _ => {}
                        }
                        match current_block_186 {
                            11994227959910167019 => {
                                _hj_i_0 = _hj_i_0.wrapping_add(
                                    *_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint,
                                );
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
                        (*s).hh.key = &raw mut (*s).gid as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void;
                        (*s).hh.keylen =
                            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                        if h.is_null() {
                            (*s).hh.next = NULL;
                            (*s).hh.prev = NULL;
                            (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                                as *mut UT_hash_table
                                as *mut UT_hash_table;
                            if (*s).hh.tbl.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*s).hh.tbl as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    ::core::mem::size_of::<UT_hash_table>() as size_t,
                                );
                                (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                                (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                                (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                                (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                                    .offset_from(s as *mut ::core::ffi::c_char)
                                    as ::core::ffi::c_long
                                    as ptrdiff_t;
                                (*(*s).hh.tbl).buckets =
                                    malloc((32 as size_t).wrapping_mul(::core::mem::size_of::<
                                        UT_hash_bucket,
                                    >(
                                    )
                                        as size_t))
                                        as *mut UT_hash_bucket;
                                (*(*s).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                                if (*(*s).hh.tbl).buckets.is_null() {
                                    exit(-(1 as ::core::ffi::c_int));
                                } else {
                                    memset(
                                        (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                        '\0' as i32,
                                        (32 as size_t).wrapping_mul(::core::mem::size_of::<
                                            UT_hash_bucket,
                                        >(
                                        )
                                            as size_t),
                                    );
                                }
                            }
                            h = s;
                        } else {
                            (*s).hh.tbl = (*h).hh.tbl;
                            (*s).hh.next = NULL;
                            (*s).hh.prev = ((*(*h).hh.tbl).tail as *mut ::core::ffi::c_char)
                                .offset(-(*(*h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void;
                            (*(*(*h).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                            (*(*h).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                        }
                        let mut _ha_bkt: ::core::ffi::c_uint = 0;
                        (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_add(1);
                        _ha_bkt = _ha_hashv
                            & (*(*h).hh.tbl)
                                .num_buckets
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        let mut _ha_head: *mut UT_hash_bucket =
                            (*(*h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                        (*s).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
                        (*s).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                        if !(*_ha_head).hh_head.is_null() {
                            (*(*_ha_head).hh_head).hh_prev =
                                &raw mut (*s).hh as *mut UT_hash_handle;
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
                                (2 as size_t)
                                    .wrapping_mul((*(*s).hh.tbl).num_buckets as size_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as size_t
                                    ),
                            ) as *mut UT_hash_bucket;
                            if _he_new_buckets.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    _he_new_buckets as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (2 as size_t)
                                        .wrapping_mul((*(*s).hh.tbl).num_buckets as size_t)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<UT_hash_bucket>() as size_t
                                        ),
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
                                    _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize))
                                        .hh_head
                                        as *mut UT_hash_handle;
                                    while !_he_thh.is_null() {
                                        _he_hh_nxt = (*_he_thh).hh_next;
                                        _he_bkt = (*_he_thh).hashv
                                            & (*(*s).hh.tbl)
                                                .num_buckets
                                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                                .wrapping_sub(1 as ::core::ffi::c_uint);
                                        _he_newbkt = _he_new_buckets.offset(_he_bkt as isize)
                                            as *mut UT_hash_bucket;
                                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                        if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                            (*(*s).hh.tbl).nonideal_items =
                                                (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                                .count
                                                .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
                                        }
                                        (*_he_thh).hh_prev =
                                            ::core::ptr::null_mut::<UT_hash_handle>();
                                        (*_he_thh).hh_next =
                                            (*_he_newbkt).hh_head as *mut UT_hash_handle;
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
                            log_vl_important as ::core::ffi::c_int as uint8_t,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"[Consolidate] Detected caret value double-mapping about glyph %s\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                gname,
                            ),
                        );
                    }
                }
            }
            j = j.wrapping_add(1);
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
        if !h.is_null() {
            _hs_insize = 1 as ::core::ffi::c_uint;
            _hs_looping = 1 as ::core::ffi::c_uint;
            _hs_list = &raw mut (*h).hh as *mut UT_hash_handle;
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
                                .offset((*(*h).hh.tbl).hho)
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
                                    .offset((*(*h).hh.tbl).hho)
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
                                        .offset((*(*h).hh.tbl).hho)
                                        as *mut UT_hash_handle
                                } else {
                                    ::core::ptr::null_mut::<UT_hash_handle>()
                                }) as *mut UT_hash_handle;
                            }
                            _hs_psize = _hs_psize.wrapping_sub(1);
                        } else if by_gid(
                            (_hs_p as *mut ::core::ffi::c_char)
                                .offset(-(*(*h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut GDEF_ligcaret_hash,
                            (_hs_q as *mut ::core::ffi::c_char)
                                .offset(-(*(*h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut GDEF_ligcaret_hash,
                        ) <= 0 as ::core::ffi::c_int
                        {
                            _hs_e = _hs_p;
                            if !_hs_p.is_null() {
                                _hs_p = (if !(*_hs_p).next.is_null() {
                                    ((*_hs_p).next as *mut ::core::ffi::c_char)
                                        .offset((*(*h).hh.tbl).hho)
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
                                    .offset((*(*h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                            _hs_qsize = _hs_qsize.wrapping_sub(1);
                        }
                        if !_hs_tail.is_null() {
                            (*_hs_tail).next = if !_hs_e.is_null() {
                                (_hs_e as *mut ::core::ffi::c_char)
                                    .offset(-(*(*h).hh.tbl).hho)
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
                                    .offset(-(*(*h).hh.tbl).hho)
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
                    (*(*h).hh.tbl).tail = _hs_tail;
                    h = (_hs_list as *mut ::core::ffi::c_char)
                        .offset(-(*(*h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut GDEF_ligcaret_hash
                        as *mut GDEF_ligcaret_hash;
                }
                _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
            }
        }
        otl_iLigCaretTable.clear.expect("non-null function pointer")(&raw mut (*gdef).ligCarets);
        let mut s_0: *mut GDEF_ligcaret_hash = ::core::ptr::null_mut::<GDEF_ligcaret_hash>();
        let mut tmp: *mut GDEF_ligcaret_hash = ::core::ptr::null_mut::<GDEF_ligcaret_hash>();
        s_0 = h;
        tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut GDEF_ligcaret_hash
            as *mut GDEF_ligcaret_hash;
        while !s_0.is_null() {
            let mut v: otl_CaretValueRecord = otl_CaretValueRecord {
                glyph: handle_fromConsolidated(
                    (*s_0).gid as glyphid_t, (*s_0).name
                ) as otfcc_GlyphHandle,
                carets: otl_CaretValueList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<otl_CaretValue>(),
                },
            };
            otl_iCaretValueList
                .move_0
                .expect("non-null function pointer")(
                &raw mut v.carets, &raw mut (*s_0).carets
            );
            otl_iLigCaretTable.push.expect("non-null function pointer")(
                &raw mut (*gdef).ligCarets,
                v,
            );
            sdsfree((*s_0).name);
            let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s_0).hh;
            if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
                free((*h).hh.tbl as *mut ::core::ffi::c_void);
                h = ::core::ptr::null_mut::<GDEF_ligcaret_hash>();
            } else {
                let mut _hd_bkt: ::core::ffi::c_uint = 0;
                if _hd_hh_del == (*(*h).hh.tbl).tail {
                    (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*h).hh.tbl).hho)
                        as *mut UT_hash_handle
                        as *mut UT_hash_handle;
                }
                if !(*_hd_hh_del).prev.is_null() {
                    let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*h).hh.tbl).hho)
                        as *mut UT_hash_handle))
                        .next;
                    *fresh0 = (*_hd_hh_del).next;
                } else {
                    h = (*_hd_hh_del).next as *mut GDEF_ligcaret_hash as *mut GDEF_ligcaret_hash;
                }
                if !(*_hd_hh_del).next.is_null() {
                    let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                        .offset((*(*h).hh.tbl).hho)
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh1 = (*_hd_hh_del).prev;
                }
                _hd_bkt = (*_hd_hh_del).hashv
                    & (*(*h).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _hd_head: *mut UT_hash_bucket =
                    (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UT_hash_bucket;
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
                (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_sub(1);
            }
            free(s_0 as *mut ::core::ffi::c_void);
            s_0 = ::core::ptr::null_mut::<GDEF_ligcaret_hash>();
            s_0 = tmp;
            tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut GDEF_ligcaret_hash
                as *mut GDEF_ligcaret_hash;
        }
    }
}
