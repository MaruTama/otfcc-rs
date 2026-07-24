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
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    static otl_iClassDef: __otfcc_IClassDef;
    static iSubtable_chaining: __caryll_elementinterface_subtable_chaining;
}

use crate::table::otl::classdef::{otl_ClassDef_free, readClassDef, otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage_free, readCoverage, otl_Coverage};
use crate::support::handle::{handle_fromIndex, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle, otfcc_GlyphHandle, otfcc_LookupHandle};
use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
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
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: usize,
    pub size: usize,
    pub free: usize,
    pub data: *mut u8,
}
pub type glyphid_t = u16;
pub type glyphclass_t = u16;
pub type tableid_t = u16;
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
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed_4 = 10;
pub const log_vl_info: C2RustUnnamed_4 = 5;
pub const log_vl_notice: C2RustUnnamed_4 = 2;
pub const log_vl_important: C2RustUnnamed_4 = 1;
pub const log_vl_critical: C2RustUnnamed_4 = 0;
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
pub type font_file_pointer = *mut u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __otfcc_ICoverage {
    pub init: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_Coverage, *const otl_Coverage) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_Coverage) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_Coverage>,
    pub free: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_Coverage, u32) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_Coverage>,
    pub dump: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_Coverage>,
    pub build: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut caryll_Buffer>,
    pub buildFormat:
        Option<unsafe extern "C" fn(*const otl_Coverage, u16) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_Coverage, bool) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_Coverage, otfcc_GlyphHandle) -> ()>,
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
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub rule: otl_ChainingRule,
    pub c2rust_unnamed: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub type CoverageReaderHandler = Option<
    unsafe extern "C" fn(
        font_file_pointer,
        u32,
        u16,
        u32,
        u16,
        glyphid_t,
        *mut ::core::ffi::c_void,
    ) -> *mut otl_Coverage,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct classdefs {
    pub bc: *mut otl_ClassDef,
    pub ic: *mut otl_ClassDef,
    pub fc: *mut otl_ClassDef,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn singleCoverage(
    mut _data: font_file_pointer,
    mut _tableLength: u32,
    mut gid: u16,
    mut _offset: u32,
    mut _kind: u16,
    _maxGlyphs: glyphid_t,
    mut _userdata: *mut ::core::ffi::c_void,
) -> *mut otl_Coverage {
    let mut cov: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    cov = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_Coverage>() as usize,
        14 as ::core::ffi::c_ulong,
    ) as *mut otl_Coverage;
    (*cov).numGlyphs = 1 as glyphid_t;
    (*cov).glyphs = __caryll_allocate_clean(
        ::core::mem::size_of::<otfcc_GlyphHandle>() as usize,
        16 as ::core::ffi::c_ulong,
    ) as *mut otfcc_GlyphHandle;
    *(*cov).glyphs.offset(0 as ::core::ffi::c_int as isize) =
        handle_fromIndex(gid) as otfcc_GlyphHandle;
    return cov;
}
#[no_mangle]
pub unsafe extern "C" fn classCoverage(
    mut _data: font_file_pointer,
    mut _tableLength: u32,
    mut cls: u16,
    mut _offset: u32,
    mut kind: u16,
    maxGlyphs: glyphid_t,
    mut _classdefs: *mut ::core::ffi::c_void,
) -> *mut otl_Coverage {
    let mut defs: *mut classdefs = _classdefs as *mut classdefs;
    let mut cd: *mut otl_ClassDef = if kind as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        (*defs).bc
    } else if kind as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        (*defs).ic
    } else {
        (*defs).fc
    };
    let mut cov: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    cov = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_Coverage>() as usize,
        26 as ::core::ffi::c_ulong,
    ) as *mut otl_Coverage;
    (*cov).numGlyphs = 0 as glyphid_t;
    (*cov).glyphs = ::core::ptr::null_mut::<otfcc_GlyphHandle>();
    let mut count: glyphid_t = 0 as glyphid_t;
    if cls as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut k: glyphid_t = 0 as glyphid_t;
        while (k as ::core::ffi::c_int) < maxGlyphs as ::core::ffi::c_int {
            let mut found: bool = false;
            let mut j: glyphid_t = 0 as glyphid_t;
            while (j as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
                if *(*cd).classes.offset(j as isize) as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && (*(*cd).glyphs.offset(j as isize)).index as ::core::ffi::c_int
                        == k as ::core::ffi::c_int
                {
                    found = true;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
            if !found {
                count = (count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
            }
            k = k.wrapping_add(1);
        }
    } else {
        let mut j_0: glyphid_t = 0 as glyphid_t;
        while (j_0 as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
            if *(*cd).classes.offset(j_0 as isize) as ::core::ffi::c_int
                == cls as ::core::ffi::c_int
            {
                count = count.wrapping_add(1);
            }
            j_0 = j_0.wrapping_add(1);
        }
    }
    if count == 0 {
        return cov;
    }
    (*cov).numGlyphs = count;
    (*cov).glyphs = __caryll_allocate_clean(
        (::core::mem::size_of::<otfcc_GlyphHandle>() as usize).wrapping_mul(count as usize),
        49 as ::core::ffi::c_ulong,
    ) as *mut otfcc_GlyphHandle;
    let mut jj: glyphid_t = 0 as glyphid_t;
    if cls as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut k_0: glyphid_t = 0 as glyphid_t;
        while (k_0 as ::core::ffi::c_int) < maxGlyphs as ::core::ffi::c_int {
            let mut found_0: bool = false;
            let mut j_1: glyphid_t = 0 as glyphid_t;
            while (j_1 as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
                if *(*cd).classes.offset(j_1 as isize) as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                    && (*(*cd).glyphs.offset(j_1 as isize)).index as ::core::ffi::c_int
                        == k_0 as ::core::ffi::c_int
                {
                    found_0 = true;
                    break;
                } else {
                    j_1 = j_1.wrapping_add(1);
                }
            }
            if !found_0 {
                let fresh12 = jj;
                jj = jj.wrapping_add(1);
                *(*cov).glyphs.offset(fresh12 as isize) =
                    handle_fromIndex(k_0)
                        as otfcc_GlyphHandle;
            }
            k_0 = k_0.wrapping_add(1);
        }
    } else {
        let mut j_2: glyphid_t = 0 as glyphid_t;
        while (j_2 as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
            if *(*cd).classes.offset(j_2 as isize) as ::core::ffi::c_int
                == cls as ::core::ffi::c_int
            {
                let fresh13 = jj;
                jj = jj.wrapping_add(1);
                *(*cov).glyphs.offset(fresh13 as isize) =
                    otfcc_Handle_dup(
                        *(*cd).glyphs.offset(j_2 as isize) as otfcc_Handle,
                    ) as otfcc_GlyphHandle;
            }
            j_2 = j_2.wrapping_add(1);
        }
    }
    return cov;
}
#[no_mangle]
pub unsafe extern "C" fn format3Coverage(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut shift: u16,
    mut _offset: u32,
    mut _kind: u16,
    _maxGlyphs: glyphid_t,
    mut _userdata: *mut ::core::ffi::c_void,
) -> *mut otl_Coverage {
    return readCoverage(
        data as *const u8,
        tableLength,
        _offset
            .wrapping_add(shift as u32)
            .wrapping_sub(2 as u32),
    );
}
#[no_mangle]
pub unsafe extern "C" fn GeneralReadContextualRule(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    mut startGID: u16,
    mut minusOne: bool,
    mut fn_0: CoverageReaderHandler,
    maxGlyphs: glyphid_t,
    mut userdata: *mut ::core::ffi::c_void,
) -> *mut otl_ChainingRule {
    let mut nInput: u16 = 0;
    let mut nApply: u16 = 0;
    let mut jj: u16 = 0;
    let mut rule: *mut otl_ChainingRule = ::core::ptr::null_mut::<otl_ChainingRule>();
    rule = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_ChainingRule>() as usize,
        83 as ::core::ffi::c_ulong,
    ) as *mut otl_ChainingRule;
    (*rule).match_0 = ::core::ptr::null_mut::<*mut otl_Coverage>();
    (*rule).apply = ::core::ptr::null_mut::<otl_ChainLookupApplication>();
    let mut minusOneQ: u16 = (if minusOne as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as u16;
    if !(tableLength < offset.wrapping_add(4 as u32)) {
        nInput = read_16u(data.offset(offset as isize) as *const u8);
        nApply = read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        );
        if !(tableLength
            < offset
                .wrapping_add(4 as u32)
                .wrapping_add((2 as ::core::ffi::c_int * nInput as ::core::ffi::c_int) as u32)
                .wrapping_add((4 as ::core::ffi::c_int * nApply as ::core::ffi::c_int) as u32))
        {
            (*rule).matchCount = nInput as tableid_t;
            (*rule).inputBegins = 0 as tableid_t;
            (*rule).inputEnds = nInput as tableid_t;
            (*rule).match_0 = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut otl_Coverage>() as usize)
                    .wrapping_mul((*rule).matchCount as usize),
                98 as ::core::ffi::c_ulong,
            ) as *mut *mut otl_Coverage;
            jj = 0 as u16;
            if minusOne {
                let fresh16 = jj;
                jj = jj.wrapping_add(1);
                let ref mut fresh17 = *(*rule).match_0.offset(fresh16 as isize);
                *fresh17 = fn_0.expect("non-null function pointer")(
                    data,
                    tableLength,
                    startGID,
                    offset,
                    2 as u16,
                    maxGlyphs,
                    userdata,
                );
            }
            let mut j: u16 = 0 as u16;
            while (j as ::core::ffi::c_int)
                < nInput as ::core::ffi::c_int - minusOneQ as ::core::ffi::c_int
            {
                let mut gid: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                let fresh18 = jj;
                jj = jj.wrapping_add(1);
                let ref mut fresh19 = *(*rule).match_0.offset(fresh18 as isize);
                *fresh19 = fn_0.expect("non-null function pointer")(
                    data,
                    tableLength,
                    gid as u16,
                    offset,
                    2 as u16,
                    maxGlyphs,
                    userdata,
                );
                j = j.wrapping_add(1);
            }
            (*rule).applyCount = nApply as tableid_t;
            (*rule).apply = __caryll_allocate_clean(
                (::core::mem::size_of::<otl_ChainLookupApplication>() as usize)
                    .wrapping_mul((*rule).applyCount as usize),
                108 as ::core::ffi::c_ulong,
            ) as *mut otl_ChainLookupApplication;
            let mut j_0: tableid_t = 0 as tableid_t;
            while (j_0 as ::core::ffi::c_int) < nApply as ::core::ffi::c_int {
                (*(*rule).apply.offset(j_0 as isize)).index = ((*rule).inputBegins
                    as ::core::ffi::c_int
                    + read_16u(
                        data.offset(offset as isize)
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * ((*rule).matchCount as ::core::ffi::c_int
                                        - minusOneQ as ::core::ffi::c_int))
                                    as isize,
                            )
                            .offset((j_0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    ) as ::core::ffi::c_int)
                    as tableid_t;
                (*(*rule).apply.offset(j_0 as isize)).lookup =
                    handle_fromIndex(read_16u(
                        data.offset(offset as isize)
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * ((*rule).matchCount as ::core::ffi::c_int
                                        - minusOneQ as ::core::ffi::c_int))
                                    as isize,
                            )
                            .offset((j_0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *const u8,
                    )
                        as glyphid_t) as otfcc_LookupHandle;
                j_0 = j_0.wrapping_add(1);
            }
            reverseBacktracks(rule);
            return rule;
        }
    }
    deleteRule(rule);
    rule = ::core::ptr::null_mut::<otl_ChainingRule>();
    return ::core::ptr::null_mut::<otl_ChainingRule>();
}
unsafe extern "C" fn readContextualFormat1(
    mut subtable: *mut subtable_chaining,
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: glyphid_t,
) -> *mut subtable_chaining {
    let mut covOffset: u16 = 0;
    let mut firstCoverage: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    let mut chainSubRuleSetCount: tableid_t = 0;
    let mut totalRules: tableid_t = 0;
    let mut jj: tableid_t = 0;
    let mut current_block: u64;
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        covOffset = offset.wrapping_add(read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as u32) as u16;
        firstCoverage = readCoverage(
            data as *const u8,
            tableLength,
            covOffset as u32,
        );
        chainSubRuleSetCount = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as tableid_t;
        if !(chainSubRuleSetCount as ::core::ffi::c_int
            != (*firstCoverage).numGlyphs as ::core::ffi::c_int)
        {
            if !(tableLength
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (2 as ::core::ffi::c_int * chainSubRuleSetCount as ::core::ffi::c_int)
                        as u32,
                ))
            {
                totalRules = 0 as tableid_t;
                let mut j: tableid_t = 0 as tableid_t;
                loop {
                    if !((j as ::core::ffi::c_int) < chainSubRuleSetCount as ::core::ffi::c_int) {
                        current_block = 4166486009154926805;
                        break;
                    }
                    let mut srsOffset: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    )
                        as u32);
                    if tableLength < srsOffset.wrapping_add(2 as u32) {
                        current_block = 10321976752019472029;
                        break;
                    }
                    totalRules = (totalRules as ::core::ffi::c_int
                        + read_16u(data.offset(srsOffset as isize) as *const u8)
                            as ::core::ffi::c_int) as tableid_t;
                    if tableLength
                        < srsOffset.wrapping_add(2 as u32).wrapping_add(
                            (2 as ::core::ffi::c_int
                                * read_16u(data.offset(srsOffset as isize) as *const u8)
                                    as ::core::ffi::c_int) as u32,
                        )
                    {
                        current_block = 10321976752019472029;
                        break;
                    }
                    j = j.wrapping_add(1);
                }
                match current_block {
                    10321976752019472029 => {}
                    _ => {
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = totalRules;
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut otl_ChainingRule>() as usize)
                                .wrapping_mul(totalRules as usize),
                            144 as ::core::ffi::c_ulong,
                        )
                            as *mut *mut otl_ChainingRule;
                        jj = 0 as tableid_t;
                        let mut j_0: tableid_t = 0 as tableid_t;
                        while (j_0 as ::core::ffi::c_int)
                            < chainSubRuleSetCount as ::core::ffi::c_int
                        {
                            let mut srsOffset_0: u32 = offset.wrapping_add(read_16u(
                                data.offset(offset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as u32);
                            let mut srsCount: tableid_t =
                                read_16u(data.offset(srsOffset_0 as isize) as *const u8)
                                    as tableid_t;
                            let mut k: tableid_t = 0 as tableid_t;
                            while (k as ::core::ffi::c_int) < srsCount as ::core::ffi::c_int {
                                let mut srOffset: u32 = srsOffset_0.wrapping_add(read_16u(
                                    data.offset(srsOffset_0 as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let ref mut fresh21 = *(*subtable)
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .rules
                                    .offset(jj as isize);
                                *fresh21 = GeneralReadContextualRule(
                                    data,
                                    tableLength,
                                    srOffset,
                                    (*(*firstCoverage).glyphs.offset(j_0 as isize)).index
                                        as u16,
                                    true,
                                    Some(
                                        singleCoverage
                                            as unsafe extern "C" fn(
                                                font_file_pointer,
                                                u32,
                                                u16,
                                                u32,
                                                u16,
                                                glyphid_t,
                                                *mut ::core::ffi::c_void,
                                            )
                                                -> *mut otl_Coverage,
                                    ),
                                    maxGlyphs,
                                    NULL,
                                );
                                jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as tableid_t;
                                k = k.wrapping_add(1);
                            }
                            j_0 = j_0.wrapping_add(1);
                        }
                        otl_Coverage_free(firstCoverage);
                        return subtable;
                    }
                }
            }
        }
    }
    iSubtable_chaining.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<subtable_chaining>();
}
unsafe extern "C" fn readContextualFormat2(
    mut subtable: *mut subtable_chaining,
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: glyphid_t,
) -> *mut subtable_chaining {
    let mut cds: *mut classdefs = ::core::ptr::null_mut::<classdefs>();
    let mut chainSubClassSetCnt: tableid_t = 0;
    let mut totalRules: tableid_t = 0;
    let mut jj: tableid_t = 0;
    if !(tableLength < offset.wrapping_add(8 as u32)) {
        cds = ::core::ptr::null_mut::<classdefs>();
        cds = __caryll_allocate_clean(
            ::core::mem::size_of::<classdefs>() as usize,
            172 as ::core::ffi::c_ulong,
        ) as *mut classdefs;
        (*cds).bc = ::core::ptr::null_mut::<otl_ClassDef>();
        (*cds).ic = readClassDef(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        (*cds).fc = ::core::ptr::null_mut::<otl_ClassDef>();
        chainSubClassSetCnt = read_16u(
            data.offset(offset as isize)
                .offset(6 as ::core::ffi::c_int as isize) as *const u8,
        ) as tableid_t;
        if !(tableLength
            < offset.wrapping_add(12 as u32).wrapping_add(
                (2 as ::core::ffi::c_int * chainSubClassSetCnt as ::core::ffi::c_int) as u32,
            ))
        {
            totalRules = 0 as tableid_t;
            let mut j: tableid_t = 0 as tableid_t;
            while (j as ::core::ffi::c_int) < chainSubClassSetCnt as ::core::ffi::c_int {
                let mut srcOffset: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if srcOffset != 0 {
                    totalRules = (totalRules as ::core::ffi::c_int
                        + read_16u(data.offset(offset as isize).offset(srcOffset as isize)
                            as *const u8) as ::core::ffi::c_int)
                        as tableid_t;
                }
                j = j.wrapping_add(1);
            }
            (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = totalRules;
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut otl_ChainingRule>() as usize)
                    .wrapping_mul(totalRules as usize),
                186 as ::core::ffi::c_ulong,
            )
                as *mut *mut otl_ChainingRule;
            jj = 0 as tableid_t;
            let mut j_0: tableid_t = 0 as tableid_t;
            while (j_0 as ::core::ffi::c_int) < chainSubClassSetCnt as ::core::ffi::c_int {
                let mut srcOffset_0: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if srcOffset_0 != 0 {
                    let mut srsCount: tableid_t =
                        read_16u(data.offset(offset as isize).offset(srcOffset_0 as isize)
                            as *const u8) as tableid_t;
                    let mut k: tableid_t = 0 as tableid_t;
                    while (k as ::core::ffi::c_int) < srsCount as ::core::ffi::c_int {
                        let mut srOffset: u32 = offset.wrapping_add(srcOffset_0).wrapping_add(
                            read_16u(
                                data.offset(offset as isize)
                                    .offset(srcOffset_0 as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32,
                        );
                        let ref mut fresh20 = *(*subtable)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .rules
                            .offset(jj as isize);
                        *fresh20 = GeneralReadContextualRule(
                            data,
                            tableLength,
                            srOffset,
                            j_0 as u16,
                            true,
                            Some(
                                classCoverage
                                    as unsafe extern "C" fn(
                                        font_file_pointer,
                                        u32,
                                        u16,
                                        u32,
                                        u16,
                                        glyphid_t,
                                        *mut ::core::ffi::c_void,
                                    )
                                        -> *mut otl_Coverage,
                            ),
                            maxGlyphs,
                            cds as *mut ::core::ffi::c_void,
                        );
                        jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as tableid_t;
                        k = k.wrapping_add(1);
                    }
                }
                j_0 = j_0.wrapping_add(1);
            }
            if !cds.is_null() {
                if !(*cds).bc.is_null() {
                    otl_ClassDef_free((*cds).bc);
                }
                if !(*cds).ic.is_null() {
                    otl_ClassDef_free((*cds).ic);
                }
                if !(*cds).fc.is_null() {
                    otl_ClassDef_free((*cds).fc);
                }
                free(cds as *mut ::core::ffi::c_void);
                cds = ::core::ptr::null_mut::<classdefs>();
            }
            return subtable;
        }
    }
    iSubtable_chaining.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<subtable_chaining>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_contextual(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: glyphid_t,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut format: u16 = 0 as u16;
    let mut subtable: *mut subtable_chaining =
        (
            iSubtable_chaining
                .create
                .expect("non-null function pointer"))();
    (*subtable).type_0 = otl_chaining_poly;
    if !(tableLength < offset.wrapping_add(2 as u32)) {
        format = read_16u(data.offset(offset as isize) as *const u8);
        if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            return readContextualFormat1(subtable, data, tableLength, offset, maxGlyphs)
                as *mut otl_Subtable;
        } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            return readContextualFormat2(subtable, data, tableLength, offset, maxGlyphs)
                as *mut otl_Subtable;
        } else if format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = 1 as tableid_t;
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut otl_ChainingRule>() as usize)
                    .wrapping_mul(1 as usize),
                231 as ::core::ffi::c_ulong,
            )
                as *mut *mut otl_ChainingRule;
            let ref mut fresh15 = *(*subtable)
                .c2rust_unnamed
                .c2rust_unnamed
                .rules
                .offset(0 as ::core::ffi::c_int as isize);
            *fresh15 = GeneralReadContextualRule(
                data,
                tableLength,
                offset.wrapping_add(2 as u32),
                0 as u16,
                false,
                Some(
                    format3Coverage
                        as unsafe extern "C" fn(
                            font_file_pointer,
                            u32,
                            u16,
                            u32,
                            u16,
                            glyphid_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut otl_Coverage,
                ),
                maxGlyphs,
                NULL,
            );
            return subtable as *mut otl_Subtable;
        }
    }
    (*(*options).logger)
        .logSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        log_vl_important as ::core::ffi::c_int as u8,
        log_type_warning,
        sdscatprintf(
            sdsempty(),
            b"Unsupported format %d.\n\0" as *const u8 as *const ::core::ffi::c_char,
            format as ::core::ffi::c_int,
        ),
    );
    iSubtable_chaining.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn GeneralReadChainingRule(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    mut startGID: u16,
    mut minusOne: bool,
    mut fn_0: CoverageReaderHandler,
    maxGlyphs: glyphid_t,
    mut userdata: *mut ::core::ffi::c_void,
) -> *mut otl_ChainingRule {
    let mut nBack: tableid_t = 0;
    let mut nInput: tableid_t = 0;
    let mut nLookaround: tableid_t = 0;
    let mut nApply: tableid_t = 0;
    let mut jj: tableid_t = 0;
    let mut rule: *mut otl_ChainingRule = ::core::ptr::null_mut::<otl_ChainingRule>();
    rule = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_ChainingRule>() as usize,
        247 as ::core::ffi::c_ulong,
    ) as *mut otl_ChainingRule;
    (*rule).match_0 = ::core::ptr::null_mut::<*mut otl_Coverage>();
    (*rule).apply = ::core::ptr::null_mut::<otl_ChainLookupApplication>();
    let mut minusOneQ: u16 = (if minusOne as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as u16;
    if !(tableLength < offset.wrapping_add(8 as u32)) {
        nBack = read_16u(data.offset(offset as isize) as *const u8) as tableid_t;
        if !(tableLength
            < offset
                .wrapping_add(2 as u32)
                .wrapping_add((2 as ::core::ffi::c_int * nBack as ::core::ffi::c_int) as u32)
                .wrapping_add(2 as u32))
        {
            nInput = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset((2 as ::core::ffi::c_int * nBack as ::core::ffi::c_int) as isize)
                    as *const u8,
            ) as tableid_t;
            if !(tableLength
                < offset
                    .wrapping_add(4 as u32)
                    .wrapping_add(
                        (2 as ::core::ffi::c_int
                            * (nBack as ::core::ffi::c_int + nInput as ::core::ffi::c_int
                                - minusOneQ as ::core::ffi::c_int))
                            as u32,
                    )
                    .wrapping_add(2 as u32))
            {
                nLookaround = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(
                            (2 as ::core::ffi::c_int
                                * (nBack as ::core::ffi::c_int + nInput as ::core::ffi::c_int
                                    - minusOneQ as ::core::ffi::c_int))
                                as isize,
                        ) as *const u8,
                ) as tableid_t;
                if !(tableLength
                    < offset
                        .wrapping_add(6 as u32)
                        .wrapping_add(
                            (2 as ::core::ffi::c_int
                                * (nBack as ::core::ffi::c_int + nInput as ::core::ffi::c_int
                                    - minusOneQ as ::core::ffi::c_int
                                    + nLookaround as ::core::ffi::c_int))
                                as u32,
                        )
                        .wrapping_add(2 as u32))
                {
                    nApply = read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * (nBack as ::core::ffi::c_int + nInput as ::core::ffi::c_int
                                        - minusOneQ as ::core::ffi::c_int
                                        + nLookaround as ::core::ffi::c_int))
                                    as isize,
                            ) as *const u8,
                    ) as tableid_t;
                    if !(tableLength
                        < offset
                            .wrapping_add(8 as u32)
                            .wrapping_add(
                                (2 as ::core::ffi::c_int
                                    * (nBack as ::core::ffi::c_int + nInput as ::core::ffi::c_int
                                        - minusOneQ as ::core::ffi::c_int
                                        + nLookaround as ::core::ffi::c_int))
                                    as u32,
                            )
                            .wrapping_add(
                                (nApply as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                    as u32,
                            ))
                    {
                        (*rule).matchCount = (nBack as ::core::ffi::c_int
                            + nInput as ::core::ffi::c_int
                            + nLookaround as ::core::ffi::c_int)
                            as tableid_t;
                        (*rule).inputBegins = nBack;
                        (*rule).inputEnds = (nBack as ::core::ffi::c_int
                            + nInput as ::core::ffi::c_int)
                            as tableid_t;
                        (*rule).match_0 = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut otl_Coverage>() as usize)
                                .wrapping_mul((*rule).matchCount as usize),
                            267 as ::core::ffi::c_ulong,
                        ) as *mut *mut otl_Coverage;
                        jj = 0 as tableid_t;
                        let mut j: tableid_t = 0 as tableid_t;
                        while (j as ::core::ffi::c_int) < nBack as ::core::ffi::c_int {
                            let mut gid: u32 = read_16u(
                                data.offset(offset as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32;
                            let fresh1 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh2 = *(*rule).match_0.offset(fresh1 as isize);
                            *fresh2 = fn_0.expect("non-null function pointer")(
                                data,
                                tableLength,
                                gid as u16,
                                offset,
                                1 as u16,
                                maxGlyphs,
                                userdata,
                            );
                            j = j.wrapping_add(1);
                        }
                        if minusOne {
                            let fresh3 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh4 = *(*rule).match_0.offset(fresh3 as isize);
                            *fresh4 = fn_0.expect("non-null function pointer")(
                                data,
                                tableLength,
                                startGID,
                                offset,
                                2 as u16,
                                maxGlyphs,
                                userdata,
                            );
                        }
                        let mut j_0: tableid_t = 0 as tableid_t;
                        while (j_0 as ::core::ffi::c_int)
                            < nInput as ::core::ffi::c_int - minusOneQ as ::core::ffi::c_int
                        {
                            let mut gid_0: u32 = read_16u(
                                data.offset(offset as isize)
                                    .offset(4 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (2 as ::core::ffi::c_int
                                            * (*rule).inputBegins as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32;
                            let fresh5 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh6 = *(*rule).match_0.offset(fresh5 as isize);
                            *fresh6 = fn_0.expect("non-null function pointer")(
                                data,
                                tableLength,
                                gid_0 as u16,
                                offset,
                                2 as u16,
                                maxGlyphs,
                                userdata,
                            );
                            j_0 = j_0.wrapping_add(1);
                        }
                        let mut j_1: tableid_t = 0 as tableid_t;
                        while (j_1 as ::core::ffi::c_int) < nLookaround as ::core::ffi::c_int {
                            let mut gid_1: u32 = read_16u(
                                data.offset(offset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (2 as ::core::ffi::c_int
                                            * ((*rule).inputEnds as ::core::ffi::c_int
                                                - minusOneQ as ::core::ffi::c_int))
                                            as isize,
                                    )
                                    .offset(
                                        (j_1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            ) as u32;
                            let fresh7 = jj;
                            jj = jj.wrapping_add(1);
                            let ref mut fresh8 = *(*rule).match_0.offset(fresh7 as isize);
                            *fresh8 = fn_0.expect("non-null function pointer")(
                                data,
                                tableLength,
                                gid_1 as u16,
                                offset,
                                3 as u16,
                                maxGlyphs,
                                userdata,
                            );
                            j_1 = j_1.wrapping_add(1);
                        }
                        (*rule).applyCount = nApply;
                        (*rule).apply = __caryll_allocate_clean(
                            (::core::mem::size_of::<otl_ChainLookupApplication>() as usize)
                                .wrapping_mul((*rule).applyCount as usize),
                            285 as ::core::ffi::c_ulong,
                        )
                            as *mut otl_ChainLookupApplication;
                        let mut j_2: tableid_t = 0 as tableid_t;
                        while (j_2 as ::core::ffi::c_int) < nApply as ::core::ffi::c_int {
                            (*(*rule).apply.offset(j_2 as isize)).index = ((*rule).inputBegins
                                as ::core::ffi::c_int
                                + read_16u(
                                    data.offset(offset as isize)
                                        .offset(8 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int
                                                * ((*rule).matchCount as ::core::ffi::c_int
                                                    - minusOneQ as ::core::ffi::c_int))
                                                as isize,
                                        )
                                        .offset(
                                            (j_2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                ) as ::core::ffi::c_int)
                                as tableid_t;
                            (*(*rule).apply.offset(j_2 as isize)).lookup =
                                handle_fromIndex(
                                    read_16u(
                                        data.offset(offset as isize)
                                            .offset(8 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (2 as ::core::ffi::c_int
                                                    * ((*rule).matchCount as ::core::ffi::c_int
                                                        - minusOneQ as ::core::ffi::c_int))
                                                    as isize,
                                            )
                                            .offset(
                                                (j_2 as ::core::ffi::c_int
                                                    * 4 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8,
                                    ) as glyphid_t,
                                ) as otfcc_LookupHandle;
                            j_2 = j_2.wrapping_add(1);
                        }
                        reverseBacktracks(rule);
                        return rule;
                    }
                }
            }
        }
    }
    deleteRule(rule);
    rule = ::core::ptr::null_mut::<otl_ChainingRule>();
    return ::core::ptr::null_mut::<otl_ChainingRule>();
}
unsafe extern "C" fn readChainingFormat1(
    mut subtable: *mut subtable_chaining,
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: glyphid_t,
) -> *mut subtable_chaining {
    let mut covOffset: u16 = 0;
    let mut firstCoverage: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    let mut chainSubRuleSetCount: tableid_t = 0;
    let mut totalRules: tableid_t = 0;
    let mut jj: tableid_t = 0;
    let mut current_block: u64;
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        covOffset = offset.wrapping_add(read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as u32) as u16;
        firstCoverage = readCoverage(
            data as *const u8,
            tableLength,
            covOffset as u32,
        );
        chainSubRuleSetCount = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as tableid_t;
        if !(chainSubRuleSetCount as ::core::ffi::c_int
            != (*firstCoverage).numGlyphs as ::core::ffi::c_int)
        {
            if !(tableLength
                < offset.wrapping_add(6 as u32).wrapping_add(
                    (2 as ::core::ffi::c_int * chainSubRuleSetCount as ::core::ffi::c_int)
                        as u32,
                ))
            {
                totalRules = 0 as tableid_t;
                let mut j: tableid_t = 0 as tableid_t;
                loop {
                    if !((j as ::core::ffi::c_int) < chainSubRuleSetCount as ::core::ffi::c_int) {
                        current_block = 4166486009154926805;
                        break;
                    }
                    let mut srsOffset: u32 = offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as *const u8,
                    )
                        as u32);
                    if tableLength < srsOffset.wrapping_add(2 as u32) {
                        current_block = 17398460390698728049;
                        break;
                    }
                    totalRules = (totalRules as ::core::ffi::c_int
                        + read_16u(data.offset(srsOffset as isize) as *const u8)
                            as ::core::ffi::c_int) as tableid_t;
                    if tableLength
                        < srsOffset.wrapping_add(2 as u32).wrapping_add(
                            (2 as ::core::ffi::c_int
                                * read_16u(data.offset(srsOffset as isize) as *const u8)
                                    as ::core::ffi::c_int) as u32,
                        )
                    {
                        current_block = 17398460390698728049;
                        break;
                    }
                    j = j.wrapping_add(1);
                }
                match current_block {
                    17398460390698728049 => {}
                    _ => {
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = totalRules;
                        (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut otl_ChainingRule>() as usize)
                                .wrapping_mul(totalRules as usize),
                            321 as ::core::ffi::c_ulong,
                        )
                            as *mut *mut otl_ChainingRule;
                        jj = 0 as tableid_t;
                        let mut j_0: tableid_t = 0 as tableid_t;
                        while (j_0 as ::core::ffi::c_int)
                            < chainSubRuleSetCount as ::core::ffi::c_int
                        {
                            let mut srsOffset_0: u32 = offset.wrapping_add(read_16u(
                                data.offset(offset as isize)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            )
                                as u32);
                            let mut srsCount: tableid_t =
                                read_16u(data.offset(srsOffset_0 as isize) as *const u8)
                                    as tableid_t;
                            let mut k: tableid_t = 0 as tableid_t;
                            while (k as ::core::ffi::c_int) < srsCount as ::core::ffi::c_int {
                                let mut srOffset: u32 = srsOffset_0.wrapping_add(read_16u(
                                    data.offset(srsOffset_0 as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let ref mut fresh14 = *(*subtable)
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .rules
                                    .offset(jj as isize);
                                *fresh14 = GeneralReadChainingRule(
                                    data,
                                    tableLength,
                                    srOffset,
                                    (*(*firstCoverage).glyphs.offset(j_0 as isize)).index
                                        as u16,
                                    true,
                                    Some(
                                        singleCoverage
                                            as unsafe extern "C" fn(
                                                font_file_pointer,
                                                u32,
                                                u16,
                                                u32,
                                                u16,
                                                glyphid_t,
                                                *mut ::core::ffi::c_void,
                                            )
                                                -> *mut otl_Coverage,
                                    ),
                                    maxGlyphs,
                                    NULL,
                                );
                                jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as tableid_t;
                                k = k.wrapping_add(1);
                            }
                            j_0 = j_0.wrapping_add(1);
                        }
                        otl_Coverage_free(firstCoverage);
                        return subtable;
                    }
                }
            }
        }
    }
    iSubtable_chaining.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<subtable_chaining>();
}
unsafe extern "C" fn readChainingFormat2(
    mut subtable: *mut subtable_chaining,
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: glyphid_t,
) -> *mut subtable_chaining {
    let mut cds: *mut classdefs = ::core::ptr::null_mut::<classdefs>();
    let mut chainSubClassSetCnt: tableid_t = 0;
    let mut totalRules: tableid_t = 0;
    let mut jj: tableid_t = 0;
    if !(tableLength < offset.wrapping_add(12 as u32)) {
        cds = ::core::ptr::null_mut::<classdefs>();
        cds = __caryll_allocate_clean(
            ::core::mem::size_of::<classdefs>() as usize,
            349 as ::core::ffi::c_ulong,
        ) as *mut classdefs;
        (*cds).bc = readClassDef(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        (*cds).ic = readClassDef(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        (*cds).fc = readClassDef(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        chainSubClassSetCnt = read_16u(
            data.offset(offset as isize)
                .offset(10 as ::core::ffi::c_int as isize) as *const u8,
        ) as tableid_t;
        if !(tableLength
            < offset.wrapping_add(12 as u32).wrapping_add(
                (2 as ::core::ffi::c_int * chainSubClassSetCnt as ::core::ffi::c_int) as u32,
            ))
        {
            totalRules = 0 as tableid_t;
            let mut j: tableid_t = 0 as tableid_t;
            while (j as ::core::ffi::c_int) < chainSubClassSetCnt as ::core::ffi::c_int {
                let mut srcOffset: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(12 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if srcOffset != 0 {
                    totalRules = (totalRules as ::core::ffi::c_int
                        + read_16u(data.offset(offset as isize).offset(srcOffset as isize)
                            as *const u8) as ::core::ffi::c_int)
                        as tableid_t;
                }
                j = j.wrapping_add(1);
            }
            (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = totalRules;
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut otl_ChainingRule>() as usize)
                    .wrapping_mul(totalRules as usize),
                363 as ::core::ffi::c_ulong,
            )
                as *mut *mut otl_ChainingRule;
            jj = 0 as tableid_t;
            let mut j_0: tableid_t = 0 as tableid_t;
            while (j_0 as ::core::ffi::c_int) < chainSubClassSetCnt as ::core::ffi::c_int {
                let mut srcOffset_0: u32 = read_16u(
                    data.offset(offset as isize)
                        .offset(12 as ::core::ffi::c_int as isize)
                        .offset((j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as *const u8,
                ) as u32;
                if srcOffset_0 != 0 {
                    let mut srsCount: tableid_t =
                        read_16u(data.offset(offset as isize).offset(srcOffset_0 as isize)
                            as *const u8) as tableid_t;
                    let mut k: tableid_t = 0 as tableid_t;
                    while (k as ::core::ffi::c_int) < srsCount as ::core::ffi::c_int {
                        let mut dsrOffset: u32 = read_16u(
                            data.offset(offset as isize)
                                .offset(srcOffset_0 as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(
                                    (k as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        ) as u32;
                        let mut srOffset: u32 =
                            offset.wrapping_add(srcOffset_0).wrapping_add(dsrOffset);
                        let ref mut fresh11 = *(*subtable)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .rules
                            .offset(jj as isize);
                        *fresh11 = GeneralReadChainingRule(
                            data,
                            tableLength,
                            srOffset,
                            j_0 as u16,
                            true,
                            Some(
                                classCoverage
                                    as unsafe extern "C" fn(
                                        font_file_pointer,
                                        u32,
                                        u16,
                                        u32,
                                        u16,
                                        glyphid_t,
                                        *mut ::core::ffi::c_void,
                                    )
                                        -> *mut otl_Coverage,
                            ),
                            maxGlyphs,
                            cds as *mut ::core::ffi::c_void,
                        );
                        jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as tableid_t;
                        k = k.wrapping_add(1);
                    }
                }
                j_0 = j_0.wrapping_add(1);
            }
            if !cds.is_null() {
                if !(*cds).bc.is_null() {
                    otl_ClassDef_free((*cds).bc);
                }
                if !(*cds).ic.is_null() {
                    otl_ClassDef_free((*cds).ic);
                }
                if !(*cds).fc.is_null() {
                    otl_ClassDef_free((*cds).fc);
                }
                free(cds as *mut ::core::ffi::c_void);
                cds = ::core::ptr::null_mut::<classdefs>();
            }
            return subtable;
        }
    }
    iSubtable_chaining.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<subtable_chaining>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_chaining(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    maxGlyphs: glyphid_t,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut format: u16 = 0 as u16;
    let mut subtable: *mut subtable_chaining =
        (
            iSubtable_chaining
                .create
                .expect("non-null function pointer"))();
    (*subtable).type_0 = otl_chaining_poly;
    if !(tableLength < offset.wrapping_add(2 as u32)) {
        format = read_16u(data.offset(offset as isize) as *const u8);
        if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            return readChainingFormat1(subtable, data, tableLength, offset, maxGlyphs)
                as *mut otl_Subtable;
        } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            return readChainingFormat2(subtable, data, tableLength, offset, maxGlyphs)
                as *mut otl_Subtable;
        } else if format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (*subtable).c2rust_unnamed.c2rust_unnamed.rulesCount = 1 as tableid_t;
            (*subtable).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut otl_ChainingRule>() as usize)
                    .wrapping_mul(1 as usize),
                407 as ::core::ffi::c_ulong,
            )
                as *mut *mut otl_ChainingRule;
            let ref mut fresh0 = *(*subtable)
                .c2rust_unnamed
                .c2rust_unnamed
                .rules
                .offset(0 as ::core::ffi::c_int as isize);
            *fresh0 = GeneralReadChainingRule(
                data,
                tableLength,
                offset.wrapping_add(2 as u32),
                0 as u16,
                false,
                Some(
                    format3Coverage
                        as unsafe extern "C" fn(
                            font_file_pointer,
                            u32,
                            u16,
                            u32,
                            u16,
                            glyphid_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut otl_Coverage,
                ),
                maxGlyphs,
                NULL,
            );
            return subtable as *mut otl_Subtable;
        }
    }
    (*(*options).logger)
        .logSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        log_vl_important as ::core::ffi::c_int as u8,
        log_type_warning,
        sdscatprintf(
            sdsempty(),
            b"Unsupported format %d.\n\0" as *const u8 as *const ::core::ffi::c_char,
            format as ::core::ffi::c_int,
        ),
    );
    iSubtable_chaining.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[inline]
unsafe extern "C" fn closeRule(mut rule: *mut otl_ChainingRule) {
    if !rule.is_null()
        && !(*rule).match_0.is_null()
        && (*rule).matchCount as ::core::ffi::c_int != 0
    {
        let mut k: tableid_t = 0 as tableid_t;
        while (k as ::core::ffi::c_int) < (*rule).matchCount as ::core::ffi::c_int {
            otl_Coverage_free(
                *(*rule).match_0.offset(k as isize),
            );
            k = k.wrapping_add(1);
        }
        free((*rule).match_0 as *mut ::core::ffi::c_void);
        (*rule).match_0 = ::core::ptr::null_mut::<*mut otl_Coverage>();
    }
    if !rule.is_null() && !(*rule).apply.is_null() {
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*rule).applyCount as ::core::ffi::c_int {
            otfcc_Handle_dispose(
                &raw mut (*(*rule).apply.offset(j as isize)).lookup,
            );
            j = j.wrapping_add(1);
        }
        free((*rule).apply as *mut ::core::ffi::c_void);
        (*rule).apply = ::core::ptr::null_mut::<otl_ChainLookupApplication>();
    }
}
#[inline]
unsafe extern "C" fn deleteRule(mut rule: *mut otl_ChainingRule) {
    if rule.is_null() {
        return;
    }
    closeRule(rule);
    free(rule as *mut ::core::ffi::c_void);
    rule = ::core::ptr::null_mut::<otl_ChainingRule>();
}
#[inline]
unsafe extern "C" fn reverseBacktracks(mut rule: *mut otl_ChainingRule) {
    if (*rule).inputBegins as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut start: tableid_t = 0 as tableid_t;
        let mut end: tableid_t =
            ((*rule).inputBegins as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as tableid_t;
        while end as ::core::ffi::c_int > start as ::core::ffi::c_int {
            let mut tmp: *mut otl_Coverage = *(*rule).match_0.offset(start as isize);
            let ref mut fresh9 = *(*rule).match_0.offset(start as isize);
            *fresh9 = *(*rule).match_0.offset(end as isize);
            let ref mut fresh10 = *(*rule).match_0.offset(end as isize);
            *fresh10 = tmp;
            end = end.wrapping_sub(1);
            start = start.wrapping_add(1);
        }
    }
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
