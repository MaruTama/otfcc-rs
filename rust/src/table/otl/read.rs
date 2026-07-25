extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static otl_iSubtableList: __caryll_vectorinterface_otl_SubtableList;
    static otl_iLookupPtr: __caryll_elementinterface_otl_LookupPtr;
    static otl_iLookupList: __caryll_vectorinterface_otl_LookupList;
    static otl_iLookupRefList: __caryll_vectorinterface_otl_LookupRefList;
    static otl_iFeaturePtr: __caryll_elementinterface_otl_FeaturePtr;
    static otl_iFeatureList: __caryll_vectorinterface_otl_FeatureList;
    static otl_iFeatureRefList: __caryll_vectorinterface_otl_FeatureRefList;
    static otl_iLanguageSystem: __caryll_elementinterface_otl_LanguageSystemPtr;
    static otl_iLangSystemList: __caryll_vectorinterface_otl_LangSystemList;
    static table_iOTL: __caryll_elementinterface_table_OTL;
    fn otl_read_gsub_single(
        data: font_file_pointer,
        tableLength: uint32_t,
        subtableOffset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_read_gsub_multi(
        data: font_file_pointer,
        tableLength: uint32_t,
        subtableOffset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_read_gsub_ligature(
        data: font_file_pointer,
        tableLength: uint32_t,
        subtableOffset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_read_gsub_reverse(
        data: font_file_pointer,
        tableLength: uint32_t,
        offset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_read_gpos_pair(
        data: font_file_pointer,
        tableLength: uint32_t,
        offset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_read_gpos_cursive(
        data: font_file_pointer,
        tableLength: uint32_t,
        subtableOffset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_read_gpos_markToSingle(
        data: font_file_pointer,
        tableLength: uint32_t,
        subtableOffset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_read_gpos_markToLigature(
        data: font_file_pointer,
        tableLength: uint32_t,
        subtableOffset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_read_chaining(
        data: font_file_pointer,
        tableLength: uint32_t,
        offset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otl_read_contextual(
        data: font_file_pointer,
        tableLength: uint32_t,
        offset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otfcc_readOtl_gsub_extend(
        data: font_file_pointer,
        tableLength: uint32_t,
        subtableOffset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    fn otfcc_readOtl_gpos_extend(
        data: font_file_pointer,
        tableLength: uint32_t,
        subtableOffset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
    static SCRIPT_LANGUAGE_SEPARATOR: ::core::ffi::c_char;
    fn otfcc_delete_lookup(lookup: *mut otl_Lookup);
    fn otl_read_gpos_single(
        data: font_file_pointer,
        tableLength: uint32_t,
        subtableOffset: uint32_t,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
}
use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{otfcc_GlyphHandle, otfcc_LookupHandle};
use crate::support::binio::{read_16u, read_32u};
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type sds = *mut ::core::ffi::c_char;
pub type glyphid_t = uint16_t;
pub type glyphclass_t = uint16_t;
pub type tableid_t = uint16_t;
pub type pos_t = ::core::ffi::c_double;
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
pub type font_file_pointer = *mut uint8_t;
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
pub type otl_Subtable = _otl_subtable;
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
pub type otl_Lookup = _otl_lookup;
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
    pub initN: Option<unsafe extern "C" fn(*mut otl_SubtableList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_SubtableList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut otl_SubtableList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_SubtableList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_SubtableList, otl_SubtablePtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_SubtableList) -> otl_SubtablePtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_SubtableList, size_t) -> ()>,
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
pub type otl_LookupPtr = *mut otl_Lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_LookupPtr {
    pub init: Option<unsafe extern "C" fn(*mut otl_LookupPtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_LookupPtr, *const otl_LookupPtr) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_LookupPtr, *mut otl_LookupPtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LookupPtr) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LookupPtr, otl_LookupPtr) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_LookupPtr, otl_LookupPtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LookupList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LookupPtr,
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
    pub initN: Option<unsafe extern "C" fn(*mut otl_LookupList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LookupList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut otl_LookupList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LookupList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LookupList, otl_LookupPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LookupList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LookupList) -> otl_LookupPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LookupList, size_t) -> ()>,
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
pub type otl_LookupRef = *const otl_Lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LookupRefList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LookupRef,
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
    pub initN: Option<unsafe extern "C" fn(*mut otl_LookupRefList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LookupRefList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut otl_LookupRefList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LookupRefList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LookupRefList, otl_LookupRef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LookupRefList) -> otl_LookupRef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LookupRefList, size_t) -> ()>,
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
pub struct otl_Feature {
    pub name: sds,
    pub lookups: otl_LookupRefList,
}
pub type otl_FeaturePtr = *mut otl_Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_FeaturePtr {
    pub init: Option<unsafe extern "C" fn(*mut otl_FeaturePtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_FeaturePtr, *const otl_FeaturePtr) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_FeaturePtr, *mut otl_FeaturePtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_FeaturePtr) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_FeaturePtr, otl_FeaturePtr) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_FeaturePtr, otl_FeaturePtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_FeatureList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_FeaturePtr,
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
    pub initN: Option<unsafe extern "C" fn(*mut otl_FeatureList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_FeatureList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut otl_FeatureList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_FeatureList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_FeatureList, otl_FeaturePtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_FeatureList) -> otl_FeaturePtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_FeatureList, size_t) -> ()>,
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
pub type otl_FeatureRef = *const otl_Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_FeatureRefList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_FeatureRef,
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
    pub initN: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut otl_FeatureRefList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, otl_FeatureRef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_FeatureRefList) -> otl_FeatureRef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_FeatureRefList, size_t) -> ()>,
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
pub struct otl_LanguageSystem {
    pub name: sds,
    pub requiredFeature: otl_FeatureRef,
    pub features: otl_FeatureRefList,
}
pub type otl_LanguageSystemPtr = *mut otl_LanguageSystem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_LanguageSystemPtr {
    pub init: Option<unsafe extern "C" fn(*mut otl_LanguageSystemPtr) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut otl_LanguageSystemPtr, *const otl_LanguageSystemPtr) -> (),
    >,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_LanguageSystemPtr, *mut otl_LanguageSystemPtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LanguageSystemPtr) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut otl_LanguageSystemPtr, otl_LanguageSystemPtr) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_LanguageSystemPtr, otl_LanguageSystemPtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LangSystemList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LanguageSystemPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_otl_LangSystemList {
    pub init: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_LangSystemList, *const otl_LangSystemList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_LangSystemList, *mut otl_LangSystemList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_LangSystemList, otl_LangSystemList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_LangSystemList, otl_LangSystemList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_LangSystemList>,
    pub free: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut otl_LangSystemList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut otl_LangSystemList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut otl_LangSystemList>,
    pub fill: Option<unsafe extern "C" fn(*mut otl_LangSystemList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_LangSystemList, otl_LanguageSystemPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut otl_LangSystemList) -> otl_LanguageSystemPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut otl_LangSystemList, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut otl_LangSystemList,
            Option<
                unsafe extern "C" fn(
                    *const otl_LanguageSystemPtr,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut otl_LangSystemList,
            Option<
                unsafe extern "C" fn(
                    *const otl_LanguageSystemPtr,
                    *const otl_LanguageSystemPtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn otfcc_readOtl_subtable(
    mut data: font_file_pointer,
    mut tableLength: uint32_t,
    mut subtableOffset: uint32_t,
    mut lookupType: otl_LookupType,
    maxGlyphs: glyphid_t,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    match lookupType as ::core::ffi::c_uint {
        17 => {
            return otl_read_gsub_single(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        18 => {
            return otl_read_gsub_multi(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        19 => {
            return otl_read_gsub_multi(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        20 => {
            return otl_read_gsub_ligature(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        22 => {
            return otl_read_chaining(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        24 => {
            return otl_read_gsub_reverse(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        40 => {
            return otl_read_chaining(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        21 => {
            return otl_read_contextual(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        39 => {
            return otl_read_contextual(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        33 => {
            return otl_read_gpos_single(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        34 => {
            return otl_read_gpos_pair(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        35 => {
            return otl_read_gpos_cursive(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        36 => {
            return otl_read_gpos_markToSingle(
                data,
                tableLength,
                subtableOffset,
                maxGlyphs,
                options,
            );
        }
        38 => {
            return otl_read_gpos_markToSingle(
                data,
                tableLength,
                subtableOffset,
                maxGlyphs,
                options,
            );
        }
        37 => {
            return otl_read_gpos_markToLigature(
                data,
                tableLength,
                subtableOffset,
                maxGlyphs,
                options,
            );
        }
        23 => {
            return otfcc_readOtl_gsub_extend(
                data,
                tableLength,
                subtableOffset,
                maxGlyphs,
                options,
            );
        }
        41 => {
            return otfcc_readOtl_gpos_extend(
                data,
                tableLength,
                subtableOffset,
                maxGlyphs,
                options,
            );
        }
        _ => return ::core::ptr::null_mut::<otl_Subtable>(),
    };
}
unsafe extern "C" fn parseLanguage(
    mut data: font_file_pointer,
    mut tableLength: uint32_t,
    mut base: uint32_t,
    mut lang: *mut otl_LanguageSystem,
    mut features: *mut otl_FeatureList,
) {
    let mut rid: tableid_t = 0;
    let mut featureCount: tableid_t = 0;
    if tableLength < base.wrapping_add(6 as uint32_t) {
        otl_iFeatureRefList
            .dispose
            .expect("non-null function pointer")(&raw mut (*lang).features);
        (*lang).requiredFeature = ::core::ptr::null::<otl_Feature>();
        return;
    } else {
        rid = read_16u(
            data.offset(base as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const uint8_t,
        ) as tableid_t;
        if (rid as size_t) < (*features).length {
            (*lang).requiredFeature = *(*features).items.offset(rid as isize) as otl_FeatureRef;
        } else {
            (*lang).requiredFeature = ::core::ptr::null::<otl_Feature>();
        }
        featureCount = read_16u(
            data.offset(base as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const uint8_t,
        ) as tableid_t;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < featureCount as ::core::ffi::c_int {
            let mut featureIndex: tableid_t = read_16u(
                data.offset(base as isize)
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset((2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize)
                    as *const uint8_t,
            ) as tableid_t;
            if (featureIndex as size_t) < (*features).length {
                otl_iFeatureRefList.push.expect("non-null function pointer")(
                    &raw mut (*lang).features,
                    *(*features).items.offset(featureIndex as isize) as otl_FeatureRef,
                );
            }
            j = j.wrapping_add(1);
        }
        return;
    };
}
unsafe extern "C" fn otfcc_readOtl_common(
    mut data: font_file_pointer,
    mut tableLength: uint32_t,
    mut lookup_type_base: otl_LookupType,
    mut options: *const otfcc_Options,
) -> *mut table_OTL {
    let mut scriptListOffset: uint32_t = 0;
    let mut featureListOffset: uint32_t = 0;
    let mut lookupListOffset: uint32_t = 0;
    let mut current_block: u64;
    let mut table: *mut table_OTL = (
        table_iOTL.create.expect("non-null function pointer"))();
    if !table.is_null() {
        if !(tableLength < 10 as uint32_t) {
            scriptListOffset =
                read_16u(data.offset(4 as ::core::ffi::c_int as isize) as *const uint8_t)
                    as uint32_t;
            if !(tableLength < scriptListOffset.wrapping_add(2 as uint32_t)) {
                featureListOffset =
                    read_16u(data.offset(6 as ::core::ffi::c_int as isize) as *const uint8_t)
                        as uint32_t;
                if !(tableLength < featureListOffset.wrapping_add(2 as uint32_t)) {
                    lookupListOffset =
                        read_16u(data.offset(8 as ::core::ffi::c_int as isize) as *const uint8_t)
                            as uint32_t;
                    if !(tableLength < lookupListOffset.wrapping_add(2 as uint32_t)) {
                        let mut lookupCount: tableid_t =
                            read_16u(data.offset(lookupListOffset as isize) as *const uint8_t)
                                as tableid_t;
                        if !(tableLength
                            < lookupListOffset.wrapping_add(2 as uint32_t).wrapping_add(
                                (lookupCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as uint32_t,
                            ))
                        {
                            let mut j: tableid_t = 0 as tableid_t;
                            loop {
                                if !((j as ::core::ffi::c_int) < lookupCount as ::core::ffi::c_int)
                                {
                                    current_block = 12147880666119273379;
                                    break;
                                }
                                let mut lookup: *mut otl_Lookup =
                                    ::core::ptr::null_mut::<otl_Lookup>();
                                otl_iLookupPtr.init.expect("non-null function pointer")(
                                    &raw mut lookup,
                                );
                                (*lookup)._offset = lookupListOffset.wrapping_add(read_16u(
                                    data.offset(lookupListOffset as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const uint8_t,
                                )
                                    as uint32_t);
                                if tableLength < (*lookup)._offset.wrapping_add(6 as uint32_t) {
                                    current_block = 2510049428056405458;
                                    break;
                                }
                                (*lookup).type_0 = (read_16u(
                                    data.offset((*lookup)._offset as isize) as *const uint8_t,
                                )
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(lookup_type_base as ::core::ffi::c_uint)
                                    as otl_LookupType;
                                otl_iLookupList.push.expect("non-null function pointer")(
                                    &raw mut (*table).lookups,
                                    lookup as otl_LookupPtr,
                                );
                                j = j.wrapping_add(1);
                            }
                            match current_block {
                                2510049428056405458 => {}
                                _ => {
                                    let mut featureCount: tableid_t =
                                        read_16u(data.offset(featureListOffset as isize)
                                            as *const uint8_t)
                                            as tableid_t;
                                    if !(tableLength
                                        < featureListOffset
                                            .wrapping_add(2 as uint32_t)
                                            .wrapping_add(
                                                (featureCount as ::core::ffi::c_int
                                                    * 6 as ::core::ffi::c_int)
                                                    as uint32_t,
                                            ))
                                    {
                                        let mut lnk: tableid_t = 0 as tableid_t;
                                        let mut j_0: tableid_t = 0 as tableid_t;
                                        loop {
                                            if !((j_0 as ::core::ffi::c_int)
                                                < featureCount as ::core::ffi::c_int)
                                            {
                                                current_block = 13460095289871124136;
                                                break;
                                            }
                                            let mut feature: *mut otl_Feature =
                                                ::core::ptr::null_mut::<otl_Feature>();
                                            otl_iFeaturePtr
                                                .init
                                                .expect("non-null function pointer")(
                                                &raw mut feature,
                                            );
                                            let mut tag: uint32_t = read_32u(
                                                data.offset(featureListOffset as isize)
                                                    .offset(2 as ::core::ffi::c_int as isize)
                                                    .offset(
                                                        (j_0 as ::core::ffi::c_int
                                                            * 6 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    as *const uint8_t,
                                            );
                                            if !(*options).glyph_name_prefix.is_null() {
                                                (*feature).name = sdscatprintf(
                                                    sdsempty(),
                                                    b"%c%c%c%c_%s_%05d\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    tag >> 24 as ::core::ffi::c_int
                                                        & 0xff as uint32_t,
                                                    tag >> 16 as ::core::ffi::c_int
                                                        & 0xff as uint32_t,
                                                    tag >> 8 as ::core::ffi::c_int
                                                        & 0xff as uint32_t,
                                                    tag & 0xff as uint32_t,
                                                    (*options).glyph_name_prefix,
                                                    j_0 as ::core::ffi::c_int,
                                                );
                                            } else {
                                                (*feature).name = sdscatprintf(
                                                    sdsempty(),
                                                    b"%c%c%c%c_%05d\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    tag >> 24 as ::core::ffi::c_int
                                                        & 0xff as uint32_t,
                                                    tag >> 16 as ::core::ffi::c_int
                                                        & 0xff as uint32_t,
                                                    tag >> 8 as ::core::ffi::c_int
                                                        & 0xff as uint32_t,
                                                    tag & 0xff as uint32_t,
                                                    j_0 as ::core::ffi::c_int,
                                                );
                                            }
                                            let mut featureOffset: uint32_t = featureListOffset
                                                .wrapping_add(read_16u(
                                                    data.offset(featureListOffset as isize)
                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (j_0 as ::core::ffi::c_int
                                                                * 6 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        .offset(4 as ::core::ffi::c_int as isize)
                                                        as *const uint8_t,
                                                )
                                                    as uint32_t);
                                            if tableLength
                                                < featureOffset.wrapping_add(4 as uint32_t)
                                            {
                                                current_block = 2510049428056405458;
                                                break;
                                            }
                                            let mut lookupCount_0: tableid_t = read_16u(
                                                data.offset(featureOffset as isize)
                                                    .offset(2 as ::core::ffi::c_int as isize)
                                                    as *const uint8_t,
                                            )
                                                as tableid_t;
                                            if tableLength
                                                < featureOffset
                                                    .wrapping_add(4 as uint32_t)
                                                    .wrapping_add(
                                                        (lookupCount_0 as ::core::ffi::c_int
                                                            * 2 as ::core::ffi::c_int)
                                                            as uint32_t,
                                                    )
                                            {
                                                current_block = 2510049428056405458;
                                                break;
                                            }
                                            let mut k: tableid_t = 0 as tableid_t;
                                            while (k as ::core::ffi::c_int)
                                                < lookupCount_0 as ::core::ffi::c_int
                                            {
                                                let mut lookupid: tableid_t = read_16u(
                                                    data.offset(featureOffset as isize)
                                                        .offset(4 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (k as ::core::ffi::c_int
                                                                * 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        as *const uint8_t,
                                                )
                                                    as tableid_t;
                                                if (lookupid as size_t) < (*table).lookups.length {
                                                    let mut lookup_0: *mut otl_Lookup = *(*table)
                                                        .lookups
                                                        .items
                                                        .offset(lookupid as isize)
                                                        as *mut otl_Lookup;
                                                    if (*lookup_0).name.is_null() {
                                                        if !(*options).glyph_name_prefix.is_null() {
                                                            let fresh3 = lnk;
                                                            lnk = lnk.wrapping_add(1);
                                                            (*lookup_0).name = sdscatprintf(
                                                                sdsempty(),
                                                                b"lookup_%s_%c%c%c%c_%d\0"
                                                                    as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                (*options).glyph_name_prefix,
                                                                tag >> 24 as ::core::ffi::c_int
                                                                    & 0xff as uint32_t,
                                                                tag >> 16 as ::core::ffi::c_int
                                                                    & 0xff as uint32_t,
                                                                tag >> 8 as ::core::ffi::c_int
                                                                    & 0xff as uint32_t,
                                                                tag & 0xff as uint32_t,
                                                                fresh3 as ::core::ffi::c_int,
                                                            );
                                                        } else {
                                                            let fresh4 = lnk;
                                                            lnk = lnk.wrapping_add(1);
                                                            (*lookup_0).name = sdscatprintf(
                                                                sdsempty(),
                                                                b"lookup_%c%c%c%c_%d\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                tag >> 24 as ::core::ffi::c_int
                                                                    & 0xff as uint32_t,
                                                                tag >> 16 as ::core::ffi::c_int
                                                                    & 0xff as uint32_t,
                                                                tag >> 8 as ::core::ffi::c_int
                                                                    & 0xff as uint32_t,
                                                                tag & 0xff as uint32_t,
                                                                fresh4 as ::core::ffi::c_int,
                                                            );
                                                        }
                                                    }
                                                    otl_iLookupRefList
                                                        .push
                                                        .expect("non-null function pointer")(
                                                        &raw mut (*feature).lookups,
                                                        lookup_0 as otl_LookupRef,
                                                    );
                                                }
                                                k = k.wrapping_add(1);
                                            }
                                            otl_iFeatureList
                                                .push
                                                .expect("non-null function pointer")(
                                                &raw mut (*table).features,
                                                feature as otl_FeaturePtr,
                                            );
                                            j_0 = j_0.wrapping_add(1);
                                        }
                                        match current_block {
                                            2510049428056405458 => {}
                                            _ => {
                                                let mut scriptCount: tableid_t =
                                                    read_16u(data.offset(scriptListOffset as isize)
                                                        as *const uint8_t)
                                                        as tableid_t;
                                                if !(tableLength
                                                    < scriptListOffset
                                                        .wrapping_add(2 as uint32_t)
                                                        .wrapping_add(
                                                            (6 as ::core::ffi::c_int
                                                                * scriptCount as ::core::ffi::c_int)
                                                                as uint32_t,
                                                        ))
                                                {
                                                    let mut nLanguageCombinations: uint32_t =
                                                        0 as uint32_t;
                                                    let mut j_1: tableid_t = 0 as tableid_t;
                                                    loop {
                                                        if !((j_1 as ::core::ffi::c_int)
                                                            < scriptCount as ::core::ffi::c_int)
                                                        {
                                                            current_block = 6528285054092551010;
                                                            break;
                                                        }
                                                        let mut scriptOffset: uint32_t =
                                                            scriptListOffset
                                                                .wrapping_add(read_16u(
                                                                data.offset(
                                                                    scriptListOffset as isize,
                                                                )
                                                                .offset(
                                                                    2 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                .offset(
                                                                    (6 as ::core::ffi::c_int
                                                                        * j_1 as ::core::ffi::c_int)
                                                                        as isize,
                                                                )
                                                                .offset(
                                                                    4 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                    as *const uint8_t,
                                                            )
                                                                as uint32_t);
                                                        if tableLength
                                                            < scriptOffset
                                                                .wrapping_add(4 as uint32_t)
                                                        {
                                                            current_block = 2510049428056405458;
                                                            break;
                                                        }
                                                        let mut defaultLangSystem: tableid_t =
                                                            read_16u(
                                                                data.offset(scriptOffset as isize)
                                                                    as *const uint8_t,
                                                            )
                                                                as tableid_t;
                                                        nLanguageCombinations =
                                                            nLanguageCombinations.wrapping_add(
                                                                ((if defaultLangSystem
                                                                    as ::core::ffi::c_int
                                                                    != 0
                                                                {
                                                                    1 as ::core::ffi::c_int
                                                                } else {
                                                                    0 as ::core::ffi::c_int
                                                                }) + read_16u(
                                                                    data.offset(
                                                                        scriptOffset as isize,
                                                                    )
                                                                    .offset(
                                                                        2 as ::core::ffi::c_int
                                                                            as isize,
                                                                    )
                                                                        as *const uint8_t,
                                                                )
                                                                    as ::core::ffi::c_int)
                                                                    as uint32_t,
                                                            );
                                                        j_1 = j_1.wrapping_add(1);
                                                    }
                                                    match current_block {
                                                        2510049428056405458 => {}
                                                        _ => {
                                                            let mut j_2: tableid_t = 0 as tableid_t;
                                                            while (j_2 as ::core::ffi::c_int)
                                                                < scriptCount as ::core::ffi::c_int
                                                            {
                                                                let mut tag_0: uint32_t = read_32u(
                                                                    data
                                                                        .offset(scriptListOffset as isize)
                                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                                        .offset(
                                                                            (6 as ::core::ffi::c_int * j_2 as ::core::ffi::c_int)
                                                                                as isize,
                                                                        ) as *const uint8_t,
                                                                );
                                                                let mut scriptOffset_0: uint32_t = scriptListOffset
                                                                    .wrapping_add(
                                                                        read_16u(
                                                                            data
                                                                                .offset(scriptListOffset as isize)
                                                                                .offset(2 as ::core::ffi::c_int as isize)
                                                                                .offset(
                                                                                    (6 as ::core::ffi::c_int * j_2 as ::core::ffi::c_int)
                                                                                        as isize,
                                                                                )
                                                                                .offset(4 as ::core::ffi::c_int as isize) as *const uint8_t,
                                                                        ) as uint32_t,
                                                                    );
                                                                let mut defaultLangSystem_0: tableid_t = read_16u(
                                                                    data.offset(scriptOffset_0 as isize) as *const uint8_t,
                                                                ) as tableid_t;
                                                                if defaultLangSystem_0 != 0 {
                                                                    let mut lang: *mut otl_LanguageSystem = ::core::ptr::null_mut::<
                                                                        otl_LanguageSystem,
                                                                    >();
                                                                    otl_iLanguageSystem
                                                                        .init
                                                                        .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        &raw mut lang
                                                                    );
                                                                    (*lang).name = sdscatprintf(
                                                                        sdsempty(),
                                                                        b"%c%c%c%c%cDFLT\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        tag_0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                        tag_0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                        tag_0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                        tag_0 & 0xff as uint32_t,
                                                                        SCRIPT_LANGUAGE_SEPARATOR as ::core::ffi::c_int,
                                                                    );
                                                                    parseLanguage(
                                                                        data,
                                                                        tableLength,
                                                                        scriptOffset_0
                                                                            .wrapping_add(
                                                                                defaultLangSystem_0
                                                                                    as uint32_t,
                                                                            ),
                                                                        lang,
                                                                        &raw mut (*table).features,
                                                                    );
                                                                    otl_iLangSystemList
                                                                        .push
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(
                                                                        &raw mut (*table).languages,
                                                                        lang as otl_LanguageSystemPtr,
                                                                    );
                                                                }
                                                                let mut langSysCount: tableid_t =
                                                                    read_16u(
                                                                        data.offset(
                                                                            scriptOffset_0 as isize,
                                                                        )
                                                                        .offset(
                                                                            2 as ::core::ffi::c_int
                                                                                as isize,
                                                                        )
                                                                            as *const uint8_t,
                                                                    )
                                                                        as tableid_t;
                                                                let mut k_0: tableid_t =
                                                                    0 as tableid_t;
                                                                while (k_0 as ::core::ffi::c_int)
                                                                    < langSysCount
                                                                        as ::core::ffi::c_int
                                                                {
                                                                    let mut langTag: uint32_t = read_32u(
                                                                        data
                                                                            .offset(scriptOffset_0 as isize)
                                                                            .offset(4 as ::core::ffi::c_int as isize)
                                                                            .offset(
                                                                                (6 as ::core::ffi::c_int * k_0 as ::core::ffi::c_int)
                                                                                    as isize,
                                                                            ) as *const uint8_t,
                                                                    );
                                                                    let mut langSys: tableid_t = read_16u(
                                                                        data
                                                                            .offset(scriptOffset_0 as isize)
                                                                            .offset(4 as ::core::ffi::c_int as isize)
                                                                            .offset(
                                                                                (6 as ::core::ffi::c_int * k_0 as ::core::ffi::c_int)
                                                                                    as isize,
                                                                            )
                                                                            .offset(4 as ::core::ffi::c_int as isize) as *const uint8_t,
                                                                    ) as tableid_t;
                                                                    let mut lang_0: *mut otl_LanguageSystem = ::core::ptr::null_mut::<
                                                                        otl_LanguageSystem,
                                                                    >();
                                                                    otl_iLanguageSystem
                                                                        .init
                                                                        .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        &raw mut lang_0
                                                                    );
                                                                    (*lang_0).name = sdscatprintf(
                                                                        sdsempty(),
                                                                        b"%c%c%c%c%c%c%c%c%c\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        tag_0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                        tag_0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                        tag_0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                        tag_0 & 0xff as uint32_t,
                                                                        SCRIPT_LANGUAGE_SEPARATOR as ::core::ffi::c_int,
                                                                        langTag >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                        langTag >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                        langTag >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                        langTag & 0xff as uint32_t,
                                                                    );
                                                                    parseLanguage(
                                                                        data,
                                                                        tableLength,
                                                                        scriptOffset_0
                                                                            .wrapping_add(
                                                                                langSys as uint32_t,
                                                                            ),
                                                                        lang_0,
                                                                        &raw mut (*table).features,
                                                                    );
                                                                    otl_iLangSystemList
                                                                        .push
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(
                                                                        &raw mut (*table).languages,
                                                                        lang_0 as otl_LanguageSystemPtr,
                                                                    );
                                                                    k_0 = k_0.wrapping_add(1);
                                                                }
                                                                j_2 = j_2.wrapping_add(1);
                                                            }
                                                            let mut j_3: tableid_t = 0 as tableid_t;
                                                            while (j_3 as size_t)
                                                                < (*table).lookups.length
                                                            {
                                                                if (**(*table)
                                                                    .lookups
                                                                    .items
                                                                    .offset(j_3 as isize))
                                                                .name
                                                                .is_null()
                                                                {
                                                                    if !(*options)
                                                                        .glyph_name_prefix
                                                                        .is_null()
                                                                    {
                                                                        let ref mut fresh5 =
                                                                            (**(*table)
                                                                                .lookups
                                                                                .items
                                                                                .offset(
                                                                                    j_3 as isize,
                                                                                ))
                                                                            .name;
                                                                        *fresh5 = sdscatprintf(
                                                                            sdsempty(),
                                                                            b"lookup_%s_%02x_%d\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            (*options).glyph_name_prefix,
                                                                            (**(*table).lookups.items.offset(j_3 as isize)).type_0
                                                                                as ::core::ffi::c_uint,
                                                                            j_3 as ::core::ffi::c_int,
                                                                        );
                                                                    } else {
                                                                        let ref mut fresh6 =
                                                                            (**(*table)
                                                                                .lookups
                                                                                .items
                                                                                .offset(
                                                                                    j_3 as isize,
                                                                                ))
                                                                            .name;
                                                                        *fresh6 = sdscatprintf(
                                                                            sdsempty(),
                                                                            b"lookup_%02x_%d\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            (**(*table).lookups.items.offset(j_3 as isize)).type_0
                                                                                as ::core::ffi::c_uint,
                                                                            j_3 as ::core::ffi::c_int,
                                                                        );
                                                                    }
                                                                }
                                                                j_3 = j_3.wrapping_add(1);
                                                            }
                                                            return table;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !table.is_null() {
        table_iOTL.free.expect("non-null function pointer")(table);
    }
    return ::core::ptr::null_mut::<table_OTL>();
}
unsafe extern "C" fn otfcc_readOtl_lookup(
    mut data: font_file_pointer,
    mut tableLength: uint32_t,
    mut lookup: *mut otl_Lookup,
    mut maxGlyphs: glyphid_t,
    mut options: *const otfcc_Options,
) {
    (*lookup).flags = read_16u(
        data.offset((*lookup)._offset as isize)
            .offset(2 as ::core::ffi::c_int as isize) as *const uint8_t,
    );
    let mut subtableCount: tableid_t = read_16u(
        data.offset((*lookup)._offset as isize)
            .offset(4 as ::core::ffi::c_int as isize) as *const uint8_t,
    ) as tableid_t;
    if subtableCount == 0
        || tableLength
            < (*lookup)._offset.wrapping_add(6 as uint32_t).wrapping_add(
                (2 as ::core::ffi::c_int * subtableCount as ::core::ffi::c_int) as uint32_t,
            )
    {
        (*lookup).type_0 = otl_type_unknown;
        return;
    }
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < subtableCount as ::core::ffi::c_int {
        let mut subtableOffset: uint32_t = (*lookup)._offset.wrapping_add(read_16u(
            data.offset((*lookup)._offset as isize)
                .offset(6 as ::core::ffi::c_int as isize)
                .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                as *const uint8_t,
        ) as uint32_t);
        let mut subtable: *mut otl_Subtable = otfcc_readOtl_subtable(
            data,
            tableLength,
            subtableOffset,
            (*lookup).type_0,
            maxGlyphs,
            options,
        );
        otl_iSubtableList.push.expect("non-null function pointer")(
            &raw mut (*lookup).subtables,
            subtable as otl_SubtablePtr,
        );
        j = j.wrapping_add(1);
    }
    if (*lookup).type_0 as ::core::ffi::c_uint
        == otl_type_gsub_extend as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*lookup).type_0 as ::core::ffi::c_uint
            == otl_type_gpos_extend as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*lookup).type_0 = otl_type_unknown;
        let mut j_0: tableid_t = 0 as tableid_t;
        while (j_0 as size_t) < (*lookup).subtables.length {
            if !(*(*lookup).subtables.items.offset(j_0 as isize)).is_null() {
                (*lookup).type_0 = (**(*lookup).subtables.items.offset(j_0 as isize))
                    .extend
                    .type_0;
                break;
            } else {
                j_0 = j_0.wrapping_add(1);
            }
        }
        if (*lookup).type_0 as u64 != 0 {
            let mut j_1: tableid_t = 0 as tableid_t;
            while (j_1 as size_t) < (*lookup).subtables.length {
                if !(*(*lookup).subtables.items.offset(j_1 as isize)).is_null()
                    && (**(*lookup).subtables.items.offset(j_1 as isize))
                        .extend
                        .type_0 as ::core::ffi::c_uint
                        == (*lookup).type_0 as ::core::ffi::c_uint
                {
                    let mut st: *mut otl_Subtable =
                        (**(*lookup).subtables.items.offset(j_1 as isize))
                            .extend
                            .subtable as *mut otl_Subtable;
                    free(
                        *(*lookup).subtables.items.offset(j_1 as isize) as *mut ::core::ffi::c_void
                    );
                    let ref mut fresh0 = *(*lookup).subtables.items.offset(j_1 as isize);
                    *fresh0 = ::core::ptr::null_mut::<otl_Subtable>();
                    let ref mut fresh1 = *(*lookup).subtables.items.offset(j_1 as isize);
                    *fresh1 = st as otl_SubtablePtr;
                } else if !(*(*lookup).subtables.items.offset(j_1 as isize)).is_null() {
                    let mut temp: *mut otl_Lookup = ::core::ptr::null_mut::<otl_Lookup>();
                    otl_iLookupPtr.init.expect("non-null function pointer")(&raw mut temp);
                    (*temp).type_0 = (**(*lookup).subtables.items.offset(j_1 as isize))
                        .extend
                        .type_0;
                    otl_iSubtableList.push.expect("non-null function pointer")(
                        &raw mut (*temp).subtables,
                        (**(*lookup).subtables.items.offset(j_1 as isize))
                            .extend
                            .subtable as otl_SubtablePtr,
                    );
                    otfcc_delete_lookup(temp);
                    temp = ::core::ptr::null_mut::<otl_Lookup>();
                    free(
                        *(*lookup).subtables.items.offset(j_1 as isize) as *mut ::core::ffi::c_void
                    );
                    let ref mut fresh2 = *(*lookup).subtables.items.offset(j_1 as isize);
                    *fresh2 = ::core::ptr::null_mut::<otl_Subtable>();
                }
                j_1 = j_1.wrapping_add(1);
            }
        } else {
            otl_iSubtableList
                .disposeDependent
                .expect("non-null function pointer")(
                &raw mut (*lookup).subtables, lookup
            );
            return;
        }
    }
    if (*lookup).type_0 as ::core::ffi::c_uint
        == otl_type_gsub_context as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*lookup).type_0 = otl_type_gsub_chaining;
    }
    if (*lookup).type_0 as ::core::ffi::c_uint
        == otl_type_gpos_context as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*lookup).type_0 = otl_type_gpos_chaining;
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readOtl(
    mut packet: otfcc_Packet,
    mut options: *const otfcc_Options,
    mut tag: uint32_t,
    mut maxGlyphs: glyphid_t,
) -> *mut table_OTL {
    let mut otl: *mut table_OTL = ::core::ptr::null_mut::<table_OTL>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == tag {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: uint32_t = table.length;
                    otl = otfcc_readOtl_common(
                        data,
                        length,
                        (if tag == 1196643650i32 as uint32_t {
                            otl_type_gsub_unknown as ::core::ffi::c_int
                        } else if tag == 1196445523i32 as uint32_t {
                            otl_type_gpos_unknown as ::core::ffi::c_int
                        } else {
                            otl_type_unknown as ::core::ffi::c_int
                        }) as otl_LookupType,
                        options,
                    );
                    if otl.is_null() {
                        if !otl.is_null() {
                            table_iOTL.free.expect("non-null function pointer")(otl);
                        }
                        otl = ::core::ptr::null_mut::<table_OTL>();
                    } else {
                        let mut j: tableid_t = 0 as tableid_t;
                        while (j as size_t) < (*otl).lookups.length {
                            otfcc_readOtl_lookup(
                                data,
                                length,
                                *(*otl).lookups.items.offset(j as isize) as *mut otl_Lookup,
                                maxGlyphs,
                                options,
                            );
                            j = j.wrapping_add(1);
                        }
                        return otl;
                    }
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_OTL>();
}
