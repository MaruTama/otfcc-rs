extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn otfcc_to_f2dot14(x: ::core::ffi::c_double) -> i16;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
    static iVQ: __caryll_vectorinterface_VQ;
    static table_iVmtx: __caryll_elementinterface_table_vmtx;
    static table_iVORG: __caryll_elementinterface_table_VORG;
    static table_iHmtx: __caryll_elementinterface_table_hmtx;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static otl_iSubtableList: __caryll_vectorinterface_otl_SubtableList;
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn buflen(buf: *mut caryll_Buffer) -> usize;
    fn bufwrite8(buf: *mut caryll_Buffer, byte: u8);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn bufwrite_bytes(buf: *mut caryll_Buffer, size: usize, str: *const u8);
    fn aglfn_setupNames(map: *mut otfcc_GlyphOrder);
    fn sha1_init(ctx: *mut SHA1_CTX);
    fn sha1_update(ctx: *mut SHA1_CTX, data: *const BYTE, len: usize);
    fn sha1_final(ctx: *mut SHA1_CTX, hash: *mut BYTE);
}

use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle};
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
pub struct __caryll_vectorinterface_otl_SubtableList {
    pub init: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_SubtableList, *const otl_SubtableList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_SubtableList, *mut otl_SubtableList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_SubtableList, otl_SubtableList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_SubtableList, otl_SubtableList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_SubtableList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_SubtableList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_SubtableList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut otl_SubtableList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_SubtableList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_SubtableList, otl_SubtablePtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> otl_SubtablePtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_SubtableList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_SubtableList,
            Option<unsafe extern "C" fn(*const otl_SubtablePtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_SubtableList,
            Option<
                unsafe extern "C" fn(
                    *const otl_SubtablePtr,
                    *const otl_SubtablePtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
    pub disposeDependent:
        Option<unsafe extern "C" fn(*mut otl_SubtableList, *const otl_Lookup) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphHash {
    pub hash: [u8; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1_CTX {
    pub data: [BYTE; 64],
    pub datalen: WORD,
    pub bitlen: ::core::ffi::c_ulonglong,
    pub state: [WORD; 5],
    pub k: [WORD; 4],
}
pub type WORD = ::core::ffi::c_uint;
pub type BYTE = ::core::ffi::c_uchar;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn hashVQS(buf: *mut caryll_Buffer, s: vq_Segment) {
    bufwrite8(buf, s.type_0 as u8);
    match s.type_0 {
        VQ_STILL => {
            bufwrite32b(
                buf,
                otfcc_to_fixed(s.val.still as ::core::ffi::c_double) as u32,
            );
        }
        VQ_DELTA => {
            bufwrite32b(
                buf,
                otfcc_to_fixed(s.val.delta.quantity as ::core::ffi::c_double) as u32,
            );
            bufwrite32b(buf, (*s.val.delta.region).dimensions as u32);
            for j in 0..(*s.val.delta.region).dimensions as usize {
                let span: *const vq_AxisSpan =
                    (&raw const (*s.val.delta.region).spans as *const vq_AxisSpan)
                        .offset(j as isize);
                bufwrite32b(
                    buf,
                    otfcc_to_f2dot14((*span).start as ::core::ffi::c_double) as u32,
                );
                bufwrite32b(
                    buf,
                    otfcc_to_f2dot14((*span).peak as ::core::ffi::c_double) as u32,
                );
                bufwrite32b(
                    buf,
                    otfcc_to_f2dot14((*span).end as ::core::ffi::c_double) as u32,
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn hashVQ(buf: *mut caryll_Buffer, x: VQ) {
    bufwrite32b(
        buf,
        otfcc_to_fixed(x.kernel as ::core::ffi::c_double) as u32,
    );
    bufwrite32b(buf, x.shift.length as u32);
    for j in 0..x.shift.length {
        hashVQS(buf, *x.shift.items.offset(j as isize));
    }
}
#[no_mangle]
pub unsafe extern "C" fn nameGlyphByHash(
    mut g: *mut glyf_Glyph,
    mut glyf: *mut table_glyf,
) -> GlyphHash {
    let buf: *mut caryll_Buffer = bufnew();
    bufwrite8(buf, 'H' as i32 as u8);
    hashVQ(buf, (*g).advanceWidth);
    bufwrite8(buf, 'h' as i32 as u8);
    hashVQ(buf, (*g).horizontalOrigin);
    bufwrite8(buf, 'V' as i32 as u8);
    hashVQ(buf, (*g).advanceHeight);
    bufwrite8(buf, 'v' as i32 as u8);
    hashVQ(buf, (*g).verticalOrigin);
    bufwrite8(buf, 'C' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).contours.length {
        bufwrite8(buf, '(' as i32 as u8);
        let c: *mut glyf_Contour = (*g).contours.items.offset(j as isize) as *mut glyf_Contour;
        for k in 0..(*c).length {
            let point = (*c).items.offset(k as isize);
            hashVQ(buf, (*point).x);
            hashVQ(buf, (*point).y);
            bufwrite8(buf, ((*point).onCurve != 0) as u8);
        }
        bufwrite8(buf, ')' as i32 as u8);
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'R' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).references.length {
        let r: *mut glyf_ComponentReference =
            (*g).references.items.offset(j as isize) as *mut glyf_ComponentReference;
        let mut h: GlyphHash = nameGlyphByHash(
            *(*glyf).items.offset((*r).glyph.index as isize) as *mut glyf_Glyph,
            glyf,
        );
        bufwrite_bytes(
            buf,
            SHA1_BLOCK_SIZE as usize,
            &raw mut h.hash as *mut u8,
        );
        hashVQ(buf, (*r).x);
        hashVQ(buf, (*r).y);
        bufwrite32b(
            buf,
            otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u32,
        );
        bufwrite32b(
            buf,
            otfcc_to_f2dot14((*r).b as ::core::ffi::c_double) as u32,
        );
        bufwrite32b(
            buf,
            otfcc_to_f2dot14((*r).c as ::core::ffi::c_double) as u32,
        );
        bufwrite32b(
            buf,
            otfcc_to_f2dot14((*r).d as ::core::ffi::c_double) as u32,
        );
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 's' as i32 as u8);
    bufwrite8(buf, 'H' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).stemH.length {
        let stem = (*g).stemH.items.offset(j as isize);
        bufwrite32b(buf, otfcc_to_fixed((*stem).position as ::core::ffi::c_double) as u32);
        bufwrite32b(buf, otfcc_to_fixed((*stem).width as ::core::ffi::c_double) as u32);
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 's' as i32 as u8);
    bufwrite8(buf, 'V' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).stemV.length {
        let stem = (*g).stemV.items.offset(j as isize);
        bufwrite32b(buf, otfcc_to_fixed((*stem).position as ::core::ffi::c_double) as u32);
        bufwrite32b(buf, otfcc_to_fixed((*stem).width as ::core::ffi::c_double) as u32);
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'm' as i32 as u8);
    bufwrite8(buf, 'H' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).hintMasks.length {
        let mask = (*g).hintMasks.items.offset(j as isize);
        bufwrite16b(buf, (*mask).contoursBefore);
        bufwrite16b(buf, (*mask).pointsBefore);
        for k in 0..(*g).stemH.length {
            bufwrite8(buf, (*mask).maskH[k] as u8);
        }
        for k in 0..(*g).stemV.length {
            bufwrite8(buf, (*mask).maskV[k] as u8);
        }
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'm' as i32 as u8);
    bufwrite8(buf, 'C' as i32 as u8);
    bufwrite8(buf, '(' as i32 as u8);
    for j in 0..(*g).contourMasks.length {
        let mask = (*g).contourMasks.items.offset(j as isize);
        bufwrite16b(buf, (*mask).contoursBefore);
        bufwrite16b(buf, (*mask).pointsBefore);
        for k in 0..(*g).stemH.length {
            bufwrite8(buf, (*mask).maskH[k] as u8);
        }
        for k in 0..(*g).stemV.length {
            bufwrite8(buf, (*mask).maskV[k] as u8);
        }
    }
    bufwrite8(buf, ')' as i32 as u8);
    bufwrite8(buf, 'I' as i32 as u8);
    bufwrite32b(buf, (*g).instructionsLength as u32);
    bufwrite_bytes(buf, (*g).instructionsLength as usize, (*g).instructions);
    let mut ctx: SHA1_CTX = SHA1_CTX {
        data: [0; 64],
        datalen: 0,
        bitlen: 0,
        state: [0; 5],
        k: [0; 4],
    };
    let mut hash: [u8; 20] = [0; 20];
    sha1_init(&raw mut ctx);
    sha1_update(&raw mut ctx, (*buf).data as *const BYTE, buflen(buf));
    sha1_final(&raw mut ctx, &raw mut hash as *mut BYTE);
    let mut h_0: GlyphHash = GlyphHash { hash: [0; 20] };
    for j in 0..SHA1_BLOCK_SIZE as usize {
        h_0.hash[j] = hash[j];
    }
    buffree(buf);
    return h_0;
}
unsafe extern "C" fn createGlyphOrder(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) -> *mut otfcc_GlyphOrder {
    let mut glyph_order: *mut otfcc_GlyphOrder =
        (
            otfcc_pkgGlyphOrder
                .create
                .expect("non-null function pointer"))();
    let mut numGlyphs: glyphid_t = (*(*font).glyf).length as glyphid_t;
    let mut prefix: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*options).glyph_name_prefix.is_null() {
        prefix = sdsnew((*options).glyph_name_prefix);
    } else {
        prefix = sdsempty();
    }
    for j in 0..numGlyphs {
        let mut g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph;
        if (*options).name_glyphs_by_hash {
            let h: GlyphHash = nameGlyphByHash(g, (*font).glyf);
            let mut gname: sds = sdsempty();
            for j_0 in 0..SHA1_BLOCK_SIZE as u16 {
                if j_0 % 4 == 0 && j_0 / 4 != 0 {
                    gname = sdscatprintf(
                        gname,
                        b"-%02X\0" as *const u8 as *const ::core::ffi::c_char,
                        h.hash[j_0 as usize] as ::core::ffi::c_int,
                    );
                } else {
                    gname = sdscatprintf(
                        gname,
                        b"%02X\0" as *const u8 as *const ::core::ffi::c_char,
                        h.hash[j_0 as usize] as ::core::ffi::c_int,
                    );
                }
            }
            if otfcc_pkgGlyphOrder
                .lookupName
                .expect("non-null function pointer")(glyph_order, gname)
            {
                let mut n: glyphid_t = 2 as glyphid_t;
                let mut stillIn: bool = false;
                loop {
                    if stillIn {
                        n = (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
                    }
                    let mut newname: sds = sdscatprintf(
                        sdsempty(),
                        b"%s-%s%d\0" as *const u8 as *const ::core::ffi::c_char,
                        gname,
                        prefix,
                        n as ::core::ffi::c_int,
                    );
                    stillIn = otfcc_pkgGlyphOrder
                        .lookupName
                        .expect("non-null function pointer")(
                        glyph_order, newname
                    );
                    sdsfree(newname);
                    if !stillIn {
                        break;
                    }
                }
                let mut newname_0: sds = sdscatprintf(
                    sdsempty(),
                    b"%s-%s%d\0" as *const u8 as *const ::core::ffi::c_char,
                    gname,
                    prefix,
                    n as ::core::ffi::c_int,
                );
                let mut sharedName: sds = otfcc_pkgGlyphOrder
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, j, newname_0
                );
                if !(*g).name.is_null() {
                    sdsfree((*g).name);
                }
                (*g).name = sdsdup(sharedName);
                sdsfree(gname);
            } else {
                let mut sharedName_0: sds = otfcc_pkgGlyphOrder
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, j, gname
                );
                if !(*g).name.is_null() {
                    sdsfree((*g).name);
                }
                (*g).name = sdsdup(sharedName_0);
            }
        } else if !((*options).ignore_glyph_order || (*options).name_glyphs_by_gid) {
            if !(*g).name.is_null() {
                let mut gname_0: sds = sdscatprintf(
                    sdsempty(),
                    b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    prefix,
                    (*g).name,
                );
                let sharedName_1: sds = otfcc_pkgGlyphOrder
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, j, gname_0
                );
                if !(*g).name.is_null() {
                    sdsfree((*g).name);
                }
                (*g).name = sdsdup(sharedName_1);
            }
        }
    }
    if !(*font).post.is_null()
        && !(*(*font).post).post_name_map.is_null()
        && !(*options).ignore_glyph_order
        && !(*options).name_glyphs_by_gid
    {
        let mut s: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
        let mut tmp: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
        s = (*(*(*font).post).post_name_map).byGID;
        tmp = (if !(*(*(*font).post).post_name_map).byGID.is_null() {
            (*(*(*(*font).post).post_name_map).byGID).hhID.next
        } else {
            NULL
        }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        while !s.is_null() {
            let mut gname_1: sds = sdscatprintf(
                sdsempty(),
                b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                prefix,
                (*s).name,
            );
            otfcc_pkgGlyphOrder
                .setByGID
                .expect("non-null function pointer")(glyph_order, (*s).gid, gname_1);
            s = tmp;
            tmp = (if !tmp.is_null() {
                (*tmp).hhID.next
            } else {
                NULL
            }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        }
    }
    if !(*font).cmap.is_null() && !(*options).name_glyphs_by_gid {
        let mut aglfn: *mut otfcc_GlyphOrder =
            (
                otfcc_pkgGlyphOrder
                    .create
                    .expect("non-null function pointer"))();
        aglfn_setupNames(aglfn);
        let mut s_0: *mut cmap_Entry = ::core::ptr::null_mut::<cmap_Entry>();
        s_0 = (*(*font).cmap).unicodes;
        while !s_0.is_null() {
            if (*s_0).glyph.index as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let mut name: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if (*s_0).unicode > 0 as ::core::ffi::c_int
                    && (*s_0).unicode < 0xffff as ::core::ffi::c_int
                {
                    otfcc_pkgGlyphOrder
                        .nameAField_Shared
                        .expect("non-null function pointer")(
                        aglfn,
                        (*s_0).unicode as glyphid_t,
                        &raw mut name,
                    );
                }
                if name.is_null() {
                    name = sdscatprintf(
                        sdsempty(),
                        b"%suni%04X\0" as *const u8 as *const ::core::ffi::c_char,
                        prefix,
                        (*s_0).unicode,
                    );
                } else {
                    name = sdscatprintf(
                        sdsempty(),
                        b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                        prefix,
                        name,
                    );
                }
                otfcc_pkgGlyphOrder
                    .setByGID
                    .expect("non-null function pointer")(
                    glyph_order, (*s_0).glyph.index, name
                );
            }
            s_0 = (*s_0).hh.next as *mut cmap_Entry;
        }
        otfcc_pkgGlyphOrder.free.expect("non-null function pointer")(aglfn);
    }
    for j_1 in 0..numGlyphs {
        let mut name_0: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if j_1 > 1 {
            name_0 = sdscatfmt(
                sdsempty(),
                b"%sglyph%u\0" as *const u8 as *const ::core::ffi::c_char,
                prefix,
                j_1 as ::core::ffi::c_int,
            );
        } else if j_1 == 1 {
            if !(*(*(*font).glyf)
                .items
                .offset(1 as ::core::ffi::c_int as isize))
            .is_null()
                && (**(*(*font).glyf)
                    .items
                    .offset(1 as ::core::ffi::c_int as isize))
                .contours
                .length
                    == 0
                && (**(*(*font).glyf)
                    .items
                    .offset(1 as ::core::ffi::c_int as isize))
                .references
                .length
                    == 0
            {
                name_0 = sdscatfmt(
                    sdsempty(),
                    b"%s.null\0" as *const u8 as *const ::core::ffi::c_char,
                    prefix,
                );
            } else {
                name_0 = sdscatfmt(
                    sdsempty(),
                    b"%sglyph%u\0" as *const u8 as *const ::core::ffi::c_char,
                    prefix,
                    j_1 as ::core::ffi::c_int,
                );
            }
        } else {
            name_0 = sdscatfmt(
                sdsempty(),
                b"%s.notdef\0" as *const u8 as *const ::core::ffi::c_char,
                prefix,
            );
        }
        otfcc_pkgGlyphOrder
            .setByGID
            .expect("non-null function pointer")(glyph_order, j_1, name_0);
    }
    sdsfree(prefix);
    return glyph_order;
}
unsafe extern "C" fn nameGlyphs(mut font: *mut otfcc_Font, mut gord: *mut otfcc_GlyphOrder) {
    if gord.is_null() {
        return;
    }
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph;
        let mut glyphName: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
        otfcc_pkgGlyphOrder
            .nameAField_Shared
            .expect("non-null function pointer")(gord, j, &raw mut glyphName);
        if !(*g).name.is_null() {
            sdsfree((*g).name);
        }
        (*g).name = sdsdup(glyphName);
    }
}
unsafe extern "C" fn unconsolidate_chaining(
    _font: *mut otfcc_Font,
    lookup: *mut otl_Lookup,
    _table: *mut table_OTL,
) {
    // The original C (c/lib/otf-reader/unconsolidate.c) computes a
    // `totalRules` count in a first pass over the subtables and never uses
    // it afterward (no capacity-reservation call, no other reference) --
    // genuinely dead code upstream, not a c2rust artifact. Confirmed by
    // inspection: the loop body only reads subtable fields into a local
    // accumulator with no other side effects. Omitted here.
    let mut newsts: otl_SubtableList = otl_SubtableList {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<otl_SubtablePtr>(),
    };
    otl_iSubtableList.init.expect("non-null function pointer")(&raw mut newsts);
    for j in 0..(*lookup).subtables.length as tableid_t {
        let slot = (*lookup).subtables.items.offset(j as isize);
        if (*slot).is_null() {
            continue;
        }
        let sub: otl_SubtablePtr = *slot;
        if (*sub).chaining.type_0 == otl_chaining_poly {
            let rules_count = (*sub).chaining.c2rust_unnamed.c2rust_unnamed.rulesCount;
            for k in 0..rules_count as ::core::ffi::c_int {
                let rule_slot = (*sub)
                    .chaining
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(k as isize);
                let st: *mut otl_Subtable = __caryll_allocate_clean(
                    ::core::mem::size_of::<otl_Subtable>() as usize,
                    278 as ::core::ffi::c_ulong,
                ) as *mut otl_Subtable;
                (*st).chaining.type_0 = otl_chaining_canonical;
                // Transfer ownership of the rule out of *rule_slot.
                (*st).chaining.c2rust_unnamed.rule = **rule_slot;
                free(*rule_slot as *mut ::core::ffi::c_void);
                *rule_slot = ::core::ptr::null_mut::<otl_ChainingRule>();
                otl_iSubtableList.push.expect("non-null function pointer")(
                    &raw mut newsts,
                    st as otl_SubtablePtr,
                );
            }
            free((*sub).chaining.c2rust_unnamed.c2rust_unnamed.rules as *mut ::core::ffi::c_void);
            (*sub).chaining.c2rust_unnamed.c2rust_unnamed.rules =
                ::core::ptr::null_mut::<*mut otl_ChainingRule>();
            free(sub as *mut ::core::ffi::c_void);
            *slot = ::core::ptr::null_mut::<otl_Subtable>();
        } else if (*sub).chaining.type_0 == otl_chaining_canonical {
            let st_0: *mut otl_Subtable = __caryll_allocate_clean(
                ::core::mem::size_of::<otl_Subtable>() as usize,
                289 as ::core::ffi::c_ulong,
            ) as *mut otl_Subtable;
            (*st_0).chaining.type_0 = otl_chaining_canonical;
            (*st_0).chaining.c2rust_unnamed.rule = (*sub).chaining.c2rust_unnamed.rule;
            otl_iSubtableList.push.expect("non-null function pointer")(
                &raw mut newsts,
                st_0 as otl_SubtablePtr,
            );
            *slot = ::core::ptr::null_mut::<otl_Subtable>();
        }
    }
    otl_iSubtableList
        .disposeDependent
        .expect("non-null function pointer")(&raw mut (*lookup).subtables, lookup);
    (*lookup).subtables = newsts;
}
unsafe extern "C" fn expandChain(font: *mut otfcc_Font, lookup: *mut otl_Lookup, table: *mut table_OTL) {
    match (*lookup).type_0 {
        otl_type_gsub_chaining | otl_type_gpos_chaining => {
            unconsolidate_chaining(font, lookup, table);
        }
        _ => {}
    };
}
unsafe extern "C" fn expandChainingLookups(font: *mut otfcc_Font) {
    if !(*font).GSUB.is_null() {
        for j in 0..(*(*font).GSUB).lookups.length {
            let lookup: *mut otl_Lookup = *(*(*font).GSUB).lookups.items.offset(j as isize) as *mut otl_Lookup;
            expandChain(font, lookup, (*font).GSUB);
        }
    }
    if !(*font).GPOS.is_null() {
        for j in 0..(*(*font).GPOS).lookups.length {
            let lookup: *mut otl_Lookup = *(*(*font).GPOS).lookups.items.offset(j as isize) as *mut otl_Lookup;
            expandChain(font, lookup, (*font).GPOS);
        }
    }
}
unsafe extern "C" fn mergeHmtx(font: *mut otfcc_Font) {
    if !(!(*font).hhea.is_null() && !(*font).hmtx.is_null() && !(*font).glyf.is_null()) {
        return;
    }
    let count_a: u32 = (*(*font).hhea).numberOfMetrics as u32;
    for j in 0..(*(*font).glyf).length as glyphid_t {
        let g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut glyf_Glyph;
        let adw: pos_t = (*(*(*font).hmtx).metrics.offset(
            (if (j as u32) < count_a {
                j as u32
            } else {
                count_a.wrapping_sub(1 as u32)
            }) as isize,
        ))
        .advanceWidth as pos_t;
        let lsb: pos_t = if (j as u32) < count_a {
            (*(*(*font).hmtx).metrics.offset(j as isize)).lsb
        } else {
            *(*(*font).hmtx)
                .leftSideBearing
                .offset((j as u32).wrapping_sub(count_a) as isize)
        };
        iVQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).advanceWidth,
            iVQ.createStill.expect("non-null function pointer")(adw) as VQ,
        );
        iVQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).horizontalOrigin,
            iVQ.createStill.expect("non-null function pointer")(-lsb + (*g).stat.xMin) as VQ,
        );
    }
    table_iHmtx.free.expect("non-null function pointer")((*font).hmtx);
    (*font).hmtx = ::core::ptr::null_mut::<table_hmtx>();
}
unsafe extern "C" fn mergeVmtx(font: *mut otfcc_Font) {
    if !(!(*font).vhea.is_null() && !(*font).vmtx.is_null() && !(*font).glyf.is_null()) {
        return;
    }
    let count_a: u32 = (*(*font).vhea).numOfLongVerMetrics as u32;
    let mut vorgs: *mut pos_t = ::core::ptr::null_mut::<pos_t>();
    if !(*font).VORG.is_null() {
        vorgs = __caryll_allocate_clean(
            (::core::mem::size_of::<pos_t>() as usize).wrapping_mul((*(*font).glyf).length),
            351 as ::core::ffi::c_ulong,
        ) as *mut pos_t;
        for j in 0..(*(*font).glyf).length as glyphid_t {
            *vorgs.offset(j as isize) = (*(*font).VORG).defaultVerticalOrigin;
        }
        for j_0 in 0..(*(*font).VORG).numVertOriginYMetrics as glyphid_t {
            if ((*(*(*font).VORG).entries.offset(j_0 as isize)).gid as usize)
                < (*(*font).glyf).length
            {
                *vorgs.offset((*(*(*font).VORG).entries.offset(j_0 as isize)).gid as isize) =
                    (*(*(*font).VORG).entries.offset(j_0 as isize)).verticalOrigin as pos_t;
            }
        }
        table_iVORG.free.expect("non-null function pointer")((*font).VORG);
        (*font).VORG = ::core::ptr::null_mut::<table_VORG>();
    }
    for j_1 in 0..(*(*font).glyf).length as glyphid_t {
        let g: *mut glyf_Glyph = *(*(*font).glyf).items.offset(j_1 as isize) as *mut glyf_Glyph;
        let adh: pos_t = (*(*(*font).vmtx).metrics.offset(
            (if (j_1 as u32) < count_a {
                j_1 as u32
            } else {
                count_a.wrapping_sub(1 as u32)
            }) as isize,
        ))
        .advanceHeight as pos_t;
        let tsb: pos_t = if (j_1 as u32) < count_a {
            (*(*(*font).vmtx).metrics.offset(j_1 as isize)).tsb
        } else {
            *(*(*font).vmtx)
                .topSideBearing
                .offset((j_1 as u32).wrapping_sub(count_a) as isize)
        };
        iVQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).advanceHeight,
            iVQ.createStill.expect("non-null function pointer")(adh) as VQ,
        );
        iVQ.inplacePlus.expect("non-null function pointer")(
            &raw mut (*g).verticalOrigin,
            iVQ.createStill.expect("non-null function pointer")(if !vorgs.is_null() {
                *vorgs.offset(j_1 as isize)
            } else {
                tsb + (*g).stat.yMax
            }) as VQ,
        );
    }
    if !vorgs.is_null() {
        free(vorgs as *mut ::core::ffi::c_void);
        vorgs = ::core::ptr::null_mut::<pos_t>();
    }
    table_iVmtx.free.expect("non-null function pointer")((*font).vmtx);
    (*font).vmtx = ::core::ptr::null_mut::<table_vmtx>();
}
unsafe extern "C" fn mergeLTSH(font: *mut otfcc_Font) {
    if !(*font).glyf.is_null() && !(*font).LTSH.is_null() {
        let n = ((*(*font).glyf).length as glyphid_t).min((*(*font).LTSH).numGlyphs);
        for j in 0..n {
            (**(*(*font).glyf).items.offset(j as isize)).yPel =
                *(*(*font).LTSH).yPels.offset(j as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_unconsolidateFont(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) {
    mergeHmtx(font);
    mergeVmtx(font);
    mergeLTSH(font);
    expandChainingLookups(font);
    if !(*font).glyf.is_null() {
        let mut gord: *mut otfcc_GlyphOrder = createGlyphOrder(font, options);
        nameGlyphs(font, gord);
        otfcc_pkgGlyphOrder.free.expect("non-null function pointer")(gord);
    }
}
pub const SHA1_BLOCK_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
