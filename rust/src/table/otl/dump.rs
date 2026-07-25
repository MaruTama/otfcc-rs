extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn json_array_new(length: size_t) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: size_t) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new(_: *const ::core::ffi::c_char) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: int64_t) -> *mut json_value;
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> size_t;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn otl_gsub_dump_single(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gsub_dump_multi(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gsub_dump_ligature(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gsub_dump_reverse(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gpos_dump_single(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gpos_dump_cursive(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gpos_dump_markToSingle(st: *const otl_Subtable) -> *mut json_value;
    fn otl_gpos_dump_markToLigature(st: *const otl_Subtable) -> *mut json_value;
    fn otl_dump_chaining(_subtable: *const otl_Subtable) -> *mut json_value;
    static mut lookupFlagsLabels: [*const ::core::ffi::c_char; 0];
    static mut tableNames: [*const ::core::ffi::c_char; 0];
    fn otl_gpos_dump_pair(_subtable: *const otl_Subtable) -> *mut json_value;
}
use crate::table::otl::classdef::{otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{otfcc_GlyphHandle, otfcc_LookupHandle};
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
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
pub type otl_LookupPtr = *mut otl_Lookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LookupList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LookupPtr,
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
pub struct otl_Feature {
    pub name: sds,
    pub lookups: otl_LookupRefList,
}
pub type otl_FeaturePtr = *mut otl_Feature;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_FeatureList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_FeaturePtr,
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
pub struct otl_LanguageSystem {
    pub name: sds,
    pub requiredFeature: otl_FeatureRef,
    pub features: otl_FeatureRefList,
}
pub type otl_LanguageSystemPtr = *mut otl_LanguageSystem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_LangSystemList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut otl_LanguageSystemPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_OTL {
    pub lookups: otl_LookupList,
    pub features: otl_FeatureList,
    pub languages: otl_LangSystemList,
}
pub const json_serialize_mode_packed: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn otfcc_dump_flags(
    mut flags: ::core::ffi::c_int,
    mut labels: *mut *const ::core::ffi::c_char,
) -> *mut json_value {
    let mut v: *mut json_value = json_object_new(0 as size_t);
    let mut j: uint16_t = 0 as uint16_t;
    while !(*labels.offset(j as isize)).is_null() {
        if flags & (1 as ::core::ffi::c_int) << j as ::core::ffi::c_int != 0 {
            json_object_push(v, *labels.offset(j as isize), json_boolean_new(true_0));
        }
        j = j.wrapping_add(1);
    }
    return v;
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
unsafe extern "C" fn _declare_lookup_dumper(
    mut llt: otl_LookupType,
    mut lt: *const ::core::ffi::c_char,
    mut dumper: Option<unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value>,
    mut lookup: *mut otl_Lookup,
    mut dump: *mut json_value,
) {
    if (*lookup).type_0 as ::core::ffi::c_uint == llt as ::core::ffi::c_uint {
        json_object_push(
            dump,
            b"type\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new(lt),
        );
        json_object_push(
            dump,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*lookup).flags as ::core::ffi::c_int,
                &raw mut lookupFlagsLabels as *mut *const ::core::ffi::c_char,
            ),
        );
        if (*lookup).flags as ::core::ffi::c_int >> 8 as ::core::ffi::c_int != 0 {
            json_object_push(
                dump,
                b"markAttachmentType\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new(
                    ((*lookup).flags as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as int64_t,
                ),
            );
        }
        let mut subtables: *mut json_value = json_array_new((*lookup).subtables.length);
        let mut j: tableid_t = 0 as tableid_t;
        while (j as size_t) < (*lookup).subtables.length {
            if !(*(*lookup).subtables.items.offset(j as isize)).is_null() {
                json_array_push(
                    subtables,
                    dumper.expect("non-null function pointer")(
                        *(*lookup).subtables.items.offset(j as isize) as *const otl_Subtable,
                    ),
                );
            }
            j = j.wrapping_add(1);
        }
        json_object_push(
            dump,
            b"subtables\0" as *const u8 as *const ::core::ffi::c_char,
            subtables,
        );
    }
}
unsafe extern "C" fn _dump_lookup(mut lookup: *mut otl_Lookup, mut dump: *mut json_value) {
    _declare_lookup_dumper(
        otl_type_gsub_single,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_single as ::core::ffi::c_int as isize),
        Some(otl_gsub_dump_single as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gsub_multiple,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_multiple as ::core::ffi::c_int as isize),
        Some(otl_gsub_dump_multi as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gsub_alternate,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_alternate as ::core::ffi::c_int as isize),
        Some(otl_gsub_dump_multi as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gsub_ligature,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_ligature as ::core::ffi::c_int as isize),
        Some(
            otl_gsub_dump_ligature as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gsub_chaining,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_chaining as ::core::ffi::c_int as isize),
        Some(otl_dump_chaining as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gsub_reverse,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_reverse as ::core::ffi::c_int as isize),
        Some(otl_gsub_dump_reverse as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_chaining,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_chaining as ::core::ffi::c_int as isize),
        Some(otl_dump_chaining as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_single,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_single as ::core::ffi::c_int as isize),
        Some(otl_gpos_dump_single as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_pair,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_pair as ::core::ffi::c_int as isize),
        Some(otl_gpos_dump_pair as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_cursive,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_cursive as ::core::ffi::c_int as isize),
        Some(otl_gpos_dump_cursive as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_markToBase,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_markToBase as ::core::ffi::c_int as isize),
        Some(
            otl_gpos_dump_markToSingle
                as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_markToMark,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_markToMark as ::core::ffi::c_int as isize),
        Some(
            otl_gpos_dump_markToSingle
                as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_markToLigature,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_markToLigature as ::core::ffi::c_int as isize),
        Some(
            otl_gpos_dump_markToLigature
                as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value,
        ),
        lookup,
        dump,
    );
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpOtl(
    mut table: *const table_OTL,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) {
    if table.is_null()
        || (*table).languages.length == 0
        || (*table).lookups.length == 0
        || (*table).features.length == 0
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
            tag,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut otl: *mut json_value = json_object_new(3 as size_t);
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"Languages\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v_0: bool = true;
        while ___loggedstep_v_0 {
            let mut languages: *mut json_value = json_object_new((*table).languages.length);
            let mut j: tableid_t = 0 as tableid_t;
            while (j as size_t) < (*table).languages.length {
                let mut _lang: *mut json_value = json_object_new(5 as size_t);
                let mut lang: *mut otl_LanguageSystem =
                    *(*table).languages.items.offset(j as isize) as *mut otl_LanguageSystem;
                if !(*lang).requiredFeature.is_null() {
                    json_object_push(
                        _lang,
                        b"requiredFeature\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new(
                            (*(*lang).requiredFeature).name as *const ::core::ffi::c_char,
                        ),
                    );
                }
                let mut features: *mut json_value = json_array_new((*lang).features.length);
                let mut k: tableid_t = 0 as tableid_t;
                while (k as size_t) < (*lang).features.length {
                    if !(*(*lang).features.items.offset(k as isize)).is_null() {
                        json_array_push(
                            features,
                            json_string_new(
                                (**(*lang).features.items.offset(k as isize)).name
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                    }
                    k = k.wrapping_add(1);
                }
                json_object_push(
                    _lang,
                    b"features\0" as *const u8 as *const ::core::ffi::c_char,
                    preserialize(features),
                );
                json_object_push(languages, (*lang).name as *const ::core::ffi::c_char, _lang);
                j = j.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"languages\0" as *const u8 as *const ::core::ffi::c_char,
                languages,
            );
            ___loggedstep_v_0 = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"Features\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v_1: bool = true;
        while ___loggedstep_v_1 {
            let mut features_0: *mut json_value = json_object_new((*table).features.length);
            let mut j_0: tableid_t = 0 as tableid_t;
            while (j_0 as size_t) < (*table).features.length {
                let mut feature: *mut otl_Feature =
                    *(*table).features.items.offset(j_0 as isize) as *mut otl_Feature;
                let mut _feature: *mut json_value = json_array_new((*feature).lookups.length);
                let mut k_0: tableid_t = 0 as tableid_t;
                while (k_0 as size_t) < (*feature).lookups.length {
                    if !(*(*feature).lookups.items.offset(k_0 as isize)).is_null() {
                        json_array_push(
                            _feature,
                            json_string_new(
                                (**(*feature).lookups.items.offset(k_0 as isize)).name
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                    }
                    k_0 = k_0.wrapping_add(1);
                }
                json_object_push(
                    features_0,
                    (*feature).name as *const ::core::ffi::c_char,
                    preserialize(_feature),
                );
                j_0 = j_0.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"features\0" as *const u8 as *const ::core::ffi::c_char,
                features_0,
            );
            ___loggedstep_v_1 = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"Lookups\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v_2: bool = true;
        while ___loggedstep_v_2 {
            let mut lookups: *mut json_value = json_object_new((*table).lookups.length);
            let mut lookupOrder: *mut json_value = json_array_new((*table).lookups.length);
            let mut j_1: tableid_t = 0 as tableid_t;
            while (j_1 as size_t) < (*table).lookups.length {
                let mut _lookup: *mut json_value = json_object_new(5 as size_t);
                let mut lookup: *mut otl_Lookup =
                    *(*table).lookups.items.offset(j_1 as isize) as *mut otl_Lookup;
                _dump_lookup(lookup, _lookup);
                json_object_push(
                    lookups,
                    (*lookup).name as *const ::core::ffi::c_char,
                    _lookup,
                );
                json_array_push(
                    lookupOrder,
                    json_string_new((*lookup).name as *const ::core::ffi::c_char),
                );
                j_1 = j_1.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"lookups\0" as *const u8 as *const ::core::ffi::c_char,
                lookups,
            );
            json_object_push(
                otl,
                b"lookupOrder\0" as *const u8 as *const ::core::ffi::c_char,
                lookupOrder,
            );
            ___loggedstep_v_2 = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        json_object_push(root, tag, otl);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
