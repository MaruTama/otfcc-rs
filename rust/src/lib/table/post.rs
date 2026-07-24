extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: size_t) -> sds;
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite8(buf: *mut caryll_Buffer, byte: uint8_t);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: uint32_t);
    fn bufwrite_sds(buf: *mut caryll_Buffer, str: sds);
    fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    fn json_object_new(length: size_t) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: int64_t) -> *mut json_value;
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
}
use crate::src::lib::support::handle::{otfcc_GlyphHandle};
use crate::src::lib::support::binio::{read_16u, read_32u, read_32s};
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type ptrdiff_t = isize;
pub type f16dot16 = int32_t;
pub type glyphid_t = uint16_t;
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
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: ::core::ffi::c_uint,
    pub nonideal_items: ::core::ffi::c_uint,
    pub ineff_expands: ::core::ffi::c_uint,
    pub noexpand: ::core::ffi::c_uint,
    pub signature: uint32_t,
}
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
pub type font_file_pointer = *mut uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as size_t,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as size_t;
        }
        _ => {}
    }
    return 0 as size_t;
}
static mut standardMacNames: [*const ::core::ffi::c_char; 258] = [
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".null\0" as *const u8 as *const ::core::ffi::c_char,
    b"nonmarkingreturn\0" as *const u8 as *const ::core::ffi::c_char,
    b"space\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclam\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"numbersign\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollar\0" as *const u8 as *const ::core::ffi::c_char,
    b"percent\0" as *const u8 as *const ::core::ffi::c_char,
    b"ampersand\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asterisk\0" as *const u8 as *const ::core::ffi::c_char,
    b"plus\0" as *const u8 as *const ::core::ffi::c_char,
    b"comma\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
    b"period\0" as *const u8 as *const ::core::ffi::c_char,
    b"slash\0" as *const u8 as *const ::core::ffi::c_char,
    b"zero\0" as *const u8 as *const ::core::ffi::c_char,
    b"one\0" as *const u8 as *const ::core::ffi::c_char,
    b"two\0" as *const u8 as *const ::core::ffi::c_char,
    b"three\0" as *const u8 as *const ::core::ffi::c_char,
    b"four\0" as *const u8 as *const ::core::ffi::c_char,
    b"five\0" as *const u8 as *const ::core::ffi::c_char,
    b"six\0" as *const u8 as *const ::core::ffi::c_char,
    b"seven\0" as *const u8 as *const ::core::ffi::c_char,
    b"eight\0" as *const u8 as *const ::core::ffi::c_char,
    b"nine\0" as *const u8 as *const ::core::ffi::c_char,
    b"colon\0" as *const u8 as *const ::core::ffi::c_char,
    b"semicolon\0" as *const u8 as *const ::core::ffi::c_char,
    b"less\0" as *const u8 as *const ::core::ffi::c_char,
    b"equal\0" as *const u8 as *const ::core::ffi::c_char,
    b"greater\0" as *const u8 as *const ::core::ffi::c_char,
    b"question\0" as *const u8 as *const ::core::ffi::c_char,
    b"at\0" as *const u8 as *const ::core::ffi::c_char,
    b"A\0" as *const u8 as *const ::core::ffi::c_char,
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"E\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"I\0" as *const u8 as *const ::core::ffi::c_char,
    b"J\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"O\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Q\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"U\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
    b"X\0" as *const u8 as *const ::core::ffi::c_char,
    b"Y\0" as *const u8 as *const ::core::ffi::c_char,
    b"Z\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"backslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char,
    b"underscore\0" as *const u8 as *const ::core::ffi::c_char,
    b"grave\0" as *const u8 as *const ::core::ffi::c_char,
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"b\0" as *const u8 as *const ::core::ffi::c_char,
    b"c\0" as *const u8 as *const ::core::ffi::c_char,
    b"d\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"f\0" as *const u8 as *const ::core::ffi::c_char,
    b"g\0" as *const u8 as *const ::core::ffi::c_char,
    b"h\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"j\0" as *const u8 as *const ::core::ffi::c_char,
    b"k\0" as *const u8 as *const ::core::ffi::c_char,
    b"l\0" as *const u8 as *const ::core::ffi::c_char,
    b"m\0" as *const u8 as *const ::core::ffi::c_char,
    b"n\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"p\0" as *const u8 as *const ::core::ffi::c_char,
    b"q\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\0" as *const u8 as *const ::core::ffi::c_char,
    b"s\0" as *const u8 as *const ::core::ffi::c_char,
    b"t\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"v\0" as *const u8 as *const ::core::ffi::c_char,
    b"w\0" as *const u8 as *const ::core::ffi::c_char,
    b"x\0" as *const u8 as *const ::core::ffi::c_char,
    b"y\0" as *const u8 as *const ::core::ffi::c_char,
    b"z\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"bar\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"dagger\0" as *const u8 as *const ::core::ffi::c_char,
    b"degree\0" as *const u8 as *const ::core::ffi::c_char,
    b"cent\0" as *const u8 as *const ::core::ffi::c_char,
    b"sterling\0" as *const u8 as *const ::core::ffi::c_char,
    b"section\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
    b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
    b"registered\0" as *const u8 as *const ::core::ffi::c_char,
    b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
    b"trademark\0" as *const u8 as *const ::core::ffi::c_char,
    b"acute\0" as *const u8 as *const ::core::ffi::c_char,
    b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"notequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"AE\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"infinity\0" as *const u8 as *const ::core::ffi::c_char,
    b"plusminus\0" as *const u8 as *const ::core::ffi::c_char,
    b"lessequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"greaterequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"yen\0" as *const u8 as *const ::core::ffi::c_char,
    b"mu\0" as *const u8 as *const ::core::ffi::c_char,
    b"partialdiff\0" as *const u8 as *const ::core::ffi::c_char,
    b"summation\0" as *const u8 as *const ::core::ffi::c_char,
    b"product\0" as *const u8 as *const ::core::ffi::c_char,
    b"pi\0" as *const u8 as *const ::core::ffi::c_char,
    b"integral\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
    b"Omega\0" as *const u8 as *const ::core::ffi::c_char,
    b"ae\0" as *const u8 as *const ::core::ffi::c_char,
    b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
    b"logicalnot\0" as *const u8 as *const ::core::ffi::c_char,
    b"radical\0" as *const u8 as *const ::core::ffi::c_char,
    b"florin\0" as *const u8 as *const ::core::ffi::c_char,
    b"approxequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Delta\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
    b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char,
    b"nonbreakingspace\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"OE\0" as *const u8 as *const ::core::ffi::c_char,
    b"oe\0" as *const u8 as *const ::core::ffi::c_char,
    b"endash\0" as *const u8 as *const ::core::ffi::c_char,
    b"emdash\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteright\0" as *const u8 as *const ::core::ffi::c_char,
    b"divide\0" as *const u8 as *const ::core::ffi::c_char,
    b"lozenge\0" as *const u8 as *const ::core::ffi::c_char,
    b"ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"fraction\0" as *const u8 as *const ::core::ffi::c_char,
    b"currency\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char,
    b"fi\0" as *const u8 as *const ::core::ffi::c_char,
    b"fl\0" as *const u8 as *const ::core::ffi::c_char,
    b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"perthousand\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"apple\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"Uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char,
    b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"breve\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"ring\0" as *const u8 as *const ::core::ffi::c_char,
    b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
    b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
    b"caron\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"lslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"Scaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"scaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"Zcaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"zcaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"brokenbar\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"Yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"minus\0" as *const u8 as *const ::core::ffi::c_char,
    b"multiply\0" as *const u8 as *const ::core::ffi::c_char,
    b"onesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"twosuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"threesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"onehalf\0" as *const u8 as *const ::core::ffi::c_char,
    b"onequarter\0" as *const u8 as *const ::core::ffi::c_char,
    b"threequarters\0" as *const u8 as *const ::core::ffi::c_char,
    b"franc\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gbreve\0" as *const u8 as *const ::core::ffi::c_char,
    b"gbreve\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"Scedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"scedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"cacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"ccaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"dcroat\0" as *const u8 as *const ::core::ffi::c_char,
];
#[inline]
unsafe extern "C" fn initPost(mut post: *mut table_post) {
    memset(
        post as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_post>() as size_t,
    );
    (*post).version = 0x30000 as ::core::ffi::c_int as f16dot16;
}
#[inline]
unsafe extern "C" fn disposePost(mut post: *mut table_post) {
    if !(*post).post_name_map.is_null() {
        otfcc_pkgGlyphOrder.free.expect("non-null function pointer")((*post).post_name_map);
    }
}
#[inline]
unsafe extern "C" fn table_post_dispose(mut x: *mut table_post) {
    disposePost(x);
}
#[inline]
unsafe extern "C" fn table_post_free(mut x: *mut table_post) {
    if x.is_null() {
        return;
    }
    table_post_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_post_create() -> *mut table_post {
    let mut x: *mut table_post =
        malloc(::core::mem::size_of::<table_post>() as size_t) as *mut table_post;
    table_post_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_post_init(mut x: *mut table_post) {
    initPost(x);
}
#[inline]
unsafe extern "C" fn table_post_copy(mut dst: *mut table_post, mut src: *const table_post) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_post>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_post_copyReplace(mut dst: *mut table_post, src: table_post) {
    table_post_dispose(dst);
    table_post_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_post_move(mut dst: *mut table_post, mut src: *mut table_post) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_post>() as size_t,
    );
    table_post_init(src);
}
#[inline]
unsafe extern "C" fn table_post_replace(mut dst: *mut table_post, src: table_post) {
    table_post_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_post>() as size_t,
    );
}
#[no_mangle]
pub static mut iTable_post: __caryll_elementinterface_table_post = {
    __caryll_elementinterface_table_post {
        init: Some(table_post_init as unsafe extern "C" fn(*mut table_post) -> ()),
        copy: Some(
            table_post_copy as unsafe extern "C" fn(*mut table_post, *const table_post) -> (),
        ),
        move_0: Some(
            table_post_move as unsafe extern "C" fn(*mut table_post, *mut table_post) -> (),
        ),
        dispose: Some(table_post_dispose as unsafe extern "C" fn(*mut table_post) -> ()),
        replace: Some(
            table_post_replace as unsafe extern "C" fn(*mut table_post, table_post) -> (),
        ),
        copyReplace: Some(
            table_post_copyReplace as unsafe extern "C" fn(*mut table_post, table_post) -> (),
        ),
        create: Some(table_post_create),
        free: Some(table_post_free as unsafe extern "C" fn(*mut table_post) -> ()),
    }
};
#[no_mangle]
pub unsafe extern "C" fn otfcc_readPost(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
) -> *mut table_post {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1886352244i32 as uint32_t {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut post: *mut table_post =
                        (
                            iTable_post.create.expect("non-null function pointer"))();
                    (*post).version = read_32s(data as *const uint8_t) as f16dot16;
                    (*post).italicAngle =
                        read_32u(data.offset(4 as ::core::ffi::c_int as isize) as *const uint8_t)
                            as f16dot16;
                    (*post).underlinePosition =
                        read_16u(data.offset(8 as ::core::ffi::c_int as isize) as *const uint8_t)
                            as int16_t;
                    (*post).underlineThickness =
                        read_16u(data.offset(10 as ::core::ffi::c_int as isize) as *const uint8_t)
                            as int16_t;
                    (*post).isFixedPitch =
                        read_32u(data.offset(12 as ::core::ffi::c_int as isize) as *const uint8_t);
                    (*post).minMemType42 =
                        read_32u(data.offset(16 as ::core::ffi::c_int as isize) as *const uint8_t);
                    (*post).maxMemType42 =
                        read_32u(data.offset(20 as ::core::ffi::c_int as isize) as *const uint8_t);
                    (*post).minMemType1 =
                        read_32u(data.offset(24 as ::core::ffi::c_int as isize) as *const uint8_t);
                    (*post).maxMemType1 =
                        read_32u(data.offset(28 as ::core::ffi::c_int as isize) as *const uint8_t);
                    (*post).post_name_map = ::core::ptr::null_mut::<otfcc_GlyphOrder>();
                    if (*post).version == 0x20000 as f16dot16 {
                        let mut map: *mut otfcc_GlyphOrder =
                            (
                                otfcc_pkgGlyphOrder
                                    .create
                                    .expect("non-null function pointer"))();
                        let mut pendingNames: [sds; 65536] =
                            [::core::ptr::null_mut::<::core::ffi::c_char>(); 65536];
                        memset(
                            &raw mut pendingNames as *mut sds as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<[sds; 65536]>() as size_t,
                        );
                        let mut numberGlyphs: uint16_t = read_16u(
                            data.offset(32 as ::core::ffi::c_int as isize) as *const uint8_t,
                        );
                        let mut offset: uint32_t = (34 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * numberGlyphs as ::core::ffi::c_int)
                            as uint32_t;
                        let mut pendingNameIndex: uint16_t = 0 as uint16_t;
                        while pendingNameIndex as ::core::ffi::c_int <= 0xffff as ::core::ffi::c_int
                            && offset < table.length
                        {
                            let mut len: uint8_t = *data.offset(offset as isize);
                            let mut s: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            if len as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                s = sdsnewlen(
                                    data.offset(offset as isize)
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as *const ::core::ffi::c_void,
                                    len as size_t,
                                );
                            } else {
                                s = sdsempty();
                            }
                            offset = offset.wrapping_add(
                                (len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
                            );
                            pendingNames[pendingNameIndex as usize] = s;
                            pendingNameIndex = (pendingNameIndex as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as uint16_t;
                        }
                        let mut j: uint16_t = 0 as uint16_t;
                        while (j as ::core::ffi::c_int) < numberGlyphs as ::core::ffi::c_int {
                            let mut nameMap: uint16_t =
                                read_16u(data.offset(34 as ::core::ffi::c_int as isize).offset(
                                    (2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                                ) as *const uint8_t);
                            if nameMap as ::core::ffi::c_int >= 258 as ::core::ffi::c_int {
                                otfcc_pkgGlyphOrder
                                    .setByGID
                                    .expect("non-null function pointer")(
                                    map,
                                    j as glyphid_t,
                                    sdsdup(
                                        pendingNames[(nameMap as ::core::ffi::c_int
                                            - 258 as ::core::ffi::c_int)
                                            as usize],
                                    ),
                                );
                            } else {
                                otfcc_pkgGlyphOrder
                                    .setByGID
                                    .expect("non-null function pointer")(
                                    map,
                                    j as glyphid_t,
                                    sdsnew(standardMacNames[nameMap as usize]),
                                );
                            }
                            j = j.wrapping_add(1);
                        }
                        let mut j_0: uint32_t = 0 as uint32_t;
                        while j_0 < pendingNameIndex as uint32_t {
                            sdsfree(pendingNames[j_0 as usize]);
                            j_0 = j_0.wrapping_add(1);
                        }
                        (*post).post_name_map = map;
                    }
                    return post;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_post>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpPost(
    mut table: *const table_post,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"post\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut post: *mut json_value = json_object_new(10 as size_t);
        json_object_push(
            post,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            post,
            b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new(otfcc_from_fixed((*table).italicAngle) as int64_t),
        );
        json_object_push(
            post,
            b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).underlinePosition as int64_t),
        );
        json_object_push(
            post,
            b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).underlineThickness as int64_t),
        );
        json_object_push(
            post,
            b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*table).isFixedPitch as ::core::ffi::c_int),
        );
        json_object_push(
            post,
            b"minMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).minMemType42 as int64_t),
        );
        json_object_push(
            post,
            b"maxMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxMemType42 as int64_t),
        );
        json_object_push(
            post,
            b"minMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).minMemType1 as int64_t),
        );
        json_object_push(
            post,
            b"maxMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxMemType1 as int64_t),
        );
        json_object_push(
            root,
            b"post\0" as *const u8 as *const ::core::ffi::c_char,
            post,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parsePost(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_post {
    let mut post: *mut table_post = (
        iTable_post.create.expect("non-null function pointer"))();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"post\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"post\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            if (*options).short_post {
                (*post).version = 0x30000 as ::core::ffi::c_int as f16dot16;
            } else {
                (*post).version = otfcc_to_fixed(json_obj_getnum(
                    table,
                    b"version\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            }
            (*post).italicAngle = otfcc_to_fixed(json_obj_getnum(
                table,
                b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            (*post).underlinePosition = json_obj_getnum(
                table,
                b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            ) as int16_t;
            (*post).underlineThickness = json_obj_getnum(
                table,
                b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            ) as int16_t;
            (*post).isFixedPitch = json_obj_getbool(
                table,
                b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            ) as uint32_t;
            (*post).minMemType42 = json_obj_getnum(
                table,
                b"minMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            ) as uint32_t;
            (*post).maxMemType42 = json_obj_getnum(
                table,
                b"maxMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            ) as uint32_t;
            (*post).minMemType1 = json_obj_getnum(
                table,
                b"minMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            ) as uint32_t;
            (*post).maxMemType1 = json_obj_getnum(
                table,
                b"maxMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            ) as uint32_t;
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return post;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildPost(
    mut post: *const table_post,
    mut glyphorder: *mut otfcc_GlyphOrder,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if post.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite32b(buf, (*post).version as uint32_t);
    bufwrite32b(buf, (*post).italicAngle as uint32_t);
    bufwrite16b(buf, (*post).underlinePosition as uint16_t);
    bufwrite16b(buf, (*post).underlineThickness as uint16_t);
    bufwrite32b(buf, (*post).isFixedPitch);
    bufwrite32b(buf, (*post).minMemType42);
    bufwrite32b(buf, (*post).maxMemType42);
    bufwrite32b(buf, (*post).minMemType1);
    bufwrite32b(buf, (*post).maxMemType1);
    if (*post).version == 0x20000 as f16dot16 {
        bufwrite16b(
            buf,
            (if !(*glyphorder).byName.is_null() {
                (*(*(*glyphorder).byName).hhName.tbl).num_items
            } else {
                0 as ::core::ffi::c_uint
            }) as uint16_t,
        );
        let mut s: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
        let mut tmp: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
        s = (*glyphorder).byName;
        tmp = (if !(*glyphorder).byName.is_null() {
            (*(*glyphorder).byName).hhName.next
        } else {
            NULL
        }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        while !s.is_null() {
            bufwrite16b(
                buf,
                (258 as ::core::ffi::c_int + (*s).gid as ::core::ffi::c_int) as uint16_t,
            );
            s = tmp;
            tmp = (if !tmp.is_null() {
                (*tmp).hhName.next
            } else {
                NULL
            }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        }
        s = (*glyphorder).byName;
        tmp = (if !(*glyphorder).byName.is_null() {
            (*(*glyphorder).byName).hhName.next
        } else {
            NULL
        }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        while !s.is_null() {
            bufwrite8(buf, sdslen((*s).name) as uint8_t);
            bufwrite_sds(buf, (*s).name);
            s = tmp;
            tmp = (if !tmp.is_null() {
                (*tmp).hhName.next
            } else {
                NULL
            }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        }
    }
    return buf;
}
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
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
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
#[inline]
unsafe extern "C" fn json_obj_getnum(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0.0f64;
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0.0f64;
}
#[inline]
unsafe extern "C" fn json_obj_getbool(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> bool {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false;
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_boolean as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.boolean != 0;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return false;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
