extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: uint32_t);
    fn bufwrite64b(buf: *mut caryll_Buffer, x: uint64_t);
    fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
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

use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u, read_32s, read_64u};
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
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type f16dot16 = int32_t;
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
pub type font_file_pointer = *mut uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn initHead(mut head: *mut table_head) {
    memset(
        head as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_head>() as size_t,
    );
    (*head).magicNumber = 0x5f0f3cf5 as uint32_t;
    (*head).unitsPerEm = 1000 as uint16_t;
}
#[inline]
unsafe extern "C" fn disposeHead(mut _head: *mut table_head) {}
#[inline]
unsafe extern "C" fn table_head_replace(mut dst: *mut table_head, src: table_head) {
    table_head_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_head>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_head_free(mut x: *mut table_head) {
    if x.is_null() {
        return;
    }
    table_head_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_head_copyReplace(mut dst: *mut table_head, src: table_head) {
    table_head_dispose(dst);
    table_head_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_head_copy(mut dst: *mut table_head, mut src: *const table_head) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_head>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_head_dispose(mut x: *mut table_head) {
    disposeHead(x);
}
#[inline]
unsafe extern "C" fn table_head_move(mut dst: *mut table_head, mut src: *mut table_head) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_head>() as size_t,
    );
    table_head_init(src);
}
#[no_mangle]
pub static mut table_iHead: __caryll_elementinterface_table_head = {
    __caryll_elementinterface_table_head {
        init: Some(table_head_init as unsafe extern "C" fn(*mut table_head) -> ()),
        copy: Some(
            table_head_copy as unsafe extern "C" fn(*mut table_head, *const table_head) -> (),
        ),
        move_0: Some(
            table_head_move as unsafe extern "C" fn(*mut table_head, *mut table_head) -> (),
        ),
        dispose: Some(table_head_dispose as unsafe extern "C" fn(*mut table_head) -> ()),
        replace: Some(
            table_head_replace as unsafe extern "C" fn(*mut table_head, table_head) -> (),
        ),
        copyReplace: Some(
            table_head_copyReplace as unsafe extern "C" fn(*mut table_head, table_head) -> (),
        ),
        create: Some(table_head_create),
        free: Some(table_head_free as unsafe extern "C" fn(*mut table_head) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_head_create() -> *mut table_head {
    let mut x: *mut table_head =
        malloc(::core::mem::size_of::<table_head>() as size_t) as *mut table_head;
    table_head_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_head_init(mut x: *mut table_head) {
    initHead(x);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readHead(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_head {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1751474532i32 as uint32_t {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: uint32_t = table.length;
                    if length < 54 as uint32_t {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as uint8_t,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"table 'head' corrupted.\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                    } else {
                        let mut head: *mut table_head = ::core::ptr::null_mut::<table_head>();
                        head = __caryll_allocate_clean(
                            ::core::mem::size_of::<table_head>() as size_t,
                            24 as ::core::ffi::c_ulong,
                        ) as *mut table_head;
                        (*head).version = read_32s(data as *const uint8_t) as f16dot16;
                        (*head).fontRevision = read_32u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const uint8_t,
                        );
                        (*head).checkSumAdjustment = read_32u(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const uint8_t,
                        );
                        (*head).magicNumber = read_32u(
                            data.offset(12 as ::core::ffi::c_int as isize) as *const uint8_t,
                        );
                        (*head).flags = read_16u(
                            data.offset(16 as ::core::ffi::c_int as isize) as *const uint8_t
                        );
                        (*head).unitsPerEm = read_16u(
                            data.offset(18 as ::core::ffi::c_int as isize) as *const uint8_t,
                        );
                        (*head).created = read_64u(
                            data.offset(20 as ::core::ffi::c_int as isize) as *const uint8_t
                        ) as int64_t;
                        (*head).modified = read_64u(
                            data.offset(28 as ::core::ffi::c_int as isize) as *const uint8_t
                        ) as int64_t;
                        (*head).xMin = read_16u(
                            data.offset(36 as ::core::ffi::c_int as isize) as *const uint8_t
                        ) as int16_t;
                        (*head).yMin = read_16u(
                            data.offset(38 as ::core::ffi::c_int as isize) as *const uint8_t
                        ) as int16_t;
                        (*head).xMax = read_16u(
                            data.offset(40 as ::core::ffi::c_int as isize) as *const uint8_t
                        ) as int16_t;
                        (*head).yMax = read_16u(
                            data.offset(42 as ::core::ffi::c_int as isize) as *const uint8_t
                        ) as int16_t;
                        (*head).macStyle = read_16u(
                            data.offset(44 as ::core::ffi::c_int as isize) as *const uint8_t
                        );
                        (*head).lowestRecPPEM = read_16u(
                            data.offset(46 as ::core::ffi::c_int as isize) as *const uint8_t,
                        );
                        (*head).fontDirectoryHint = read_16u(
                            data.offset(48 as ::core::ffi::c_int as isize) as *const uint8_t,
                        ) as int16_t;
                        (*head).indexToLocFormat = read_16u(
                            data.offset(50 as ::core::ffi::c_int as isize) as *const uint8_t,
                        ) as int16_t;
                        (*head).glyphDataFormat = read_16u(
                            data.offset(52 as ::core::ffi::c_int as isize) as *const uint8_t,
                        ) as int16_t;
                        return head;
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
    return ::core::ptr::null_mut::<table_head>();
}
static mut headFlagsLabels: [*const ::core::ffi::c_char; 16] = [
    b"baselineAtY_0\0" as *const u8 as *const ::core::ffi::c_char,
    b"lsbAtX_0\0" as *const u8 as *const ::core::ffi::c_char,
    b"instrMayDependOnPointSize\0" as *const u8 as *const ::core::ffi::c_char,
    b"alwaysUseIntegerSize\0" as *const u8 as *const ::core::ffi::c_char,
    b"instrMayAlterAdvanceWidth\0" as *const u8 as *const ::core::ffi::c_char,
    b"designedForVertical\0" as *const u8 as *const ::core::ffi::c_char,
    b"_reserved1\0" as *const u8 as *const ::core::ffi::c_char,
    b"designedForComplexScript\0" as *const u8 as *const ::core::ffi::c_char,
    b"hasMetamorphosisEffects\0" as *const u8 as *const ::core::ffi::c_char,
    b"containsStrongRTL\0" as *const u8 as *const ::core::ffi::c_char,
    b"containsIndicRearrangement\0" as *const u8 as *const ::core::ffi::c_char,
    b"fontIsLossless\0" as *const u8 as *const ::core::ffi::c_char,
    b"fontIsConverted\0" as *const u8 as *const ::core::ffi::c_char,
    b"optimizedForCleartype\0" as *const u8 as *const ::core::ffi::c_char,
    b"lastResortFont\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut macStyleLabels: [*const ::core::ffi::c_char; 8] = [
    b"bold\0" as *const u8 as *const ::core::ffi::c_char,
    b"italic\0" as *const u8 as *const ::core::ffi::c_char,
    b"underline\0" as *const u8 as *const ::core::ffi::c_char,
    b"outline\0" as *const u8 as *const ::core::ffi::c_char,
    b"shadow\0" as *const u8 as *const ::core::ffi::c_char,
    b"condensed\0" as *const u8 as *const ::core::ffi::c_char,
    b"extended\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpHead(
    mut table: *const table_head,
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
            b"head\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut head: *mut json_value = json_object_new(15 as size_t);
        json_object_push(
            head,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            head,
            b"fontRevision\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).fontRevision as f16dot16)),
        );
        json_object_push(
            head,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).flags as ::core::ffi::c_int,
                &raw mut headFlagsLabels as *mut *const ::core::ffi::c_char,
            ),
        );
        json_object_push(
            head,
            b"unitsPerEm\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).unitsPerEm as int64_t),
        );
        json_object_push(
            head,
            b"created\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).created),
        );
        json_object_push(
            head,
            b"modified\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).modified),
        );
        json_object_push(
            head,
            b"xMin\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).xMin as int64_t),
        );
        json_object_push(
            head,
            b"xMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).xMax as int64_t),
        );
        json_object_push(
            head,
            b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).yMin as int64_t),
        );
        json_object_push(
            head,
            b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).yMax as int64_t),
        );
        json_object_push(
            head,
            b"macStyle\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).macStyle as ::core::ffi::c_int,
                &raw mut macStyleLabels as *mut *const ::core::ffi::c_char,
            ),
        );
        json_object_push(
            head,
            b"lowestRecPPEM\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).lowestRecPPEM as int64_t),
        );
        json_object_push(
            head,
            b"fontDirectoryHint\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).fontDirectoryHint as int64_t),
        );
        json_object_push(
            head,
            b"indexToLocFormat\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).indexToLocFormat as int64_t),
        );
        json_object_push(
            head,
            b"glyphDataFormat\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).glyphDataFormat as int64_t),
        );
        json_object_push(
            root,
            b"head\0" as *const u8 as *const ::core::ffi::c_char,
            head,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseHead(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_head {
    let mut head: *mut table_head = (
        table_iHead.create.expect("non-null function pointer"))();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"head\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"head\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*head).version = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ));
            (*head).fontRevision = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"fontRevision\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            )) as uint32_t;
            (*head).flags = otfcc_parse_flags(
                json_obj_get(table, b"flags\0" as *const u8 as *const ::core::ffi::c_char),
                &raw mut headFlagsLabels as *mut *const ::core::ffi::c_char,
            ) as uint16_t;
            (*head).unitsPerEm = json_obj_getnum_fallback(
                table,
                b"unitsPerEm\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as uint16_t;
            (*head).created = json_obj_getnum_fallback(
                table,
                b"created\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as int64_t;
            (*head).modified = json_obj_getnum_fallback(
                table,
                b"modified\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as int64_t;
            (*head).xMin = json_obj_getnum_fallback(
                table,
                b"xMin\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as int16_t;
            (*head).xMax = json_obj_getnum_fallback(
                table,
                b"xMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as int16_t;
            (*head).yMin = json_obj_getnum_fallback(
                table,
                b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as int16_t;
            (*head).yMax = json_obj_getnum_fallback(
                table,
                b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as int16_t;
            (*head).macStyle = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"macStyle\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &raw mut macStyleLabels as *mut *const ::core::ffi::c_char,
            ) as uint16_t;
            (*head).lowestRecPPEM = json_obj_getnum_fallback(
                table,
                b"lowestRecPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as uint16_t;
            (*head).fontDirectoryHint = json_obj_getnum_fallback(
                table,
                b"fontDirectoryHint\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as int16_t;
            (*head).indexToLocFormat = json_obj_getnum_fallback(
                table,
                b"indexToLocFormat\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as int16_t;
            (*head).glyphDataFormat = json_obj_getnum_fallback(
                table,
                b"glyphDataFormat\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as int16_t;
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return head;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildHead(
    mut head: *const table_head,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if head.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite32b(buf, (*head).version as uint32_t);
    bufwrite32b(buf, (*head).fontRevision);
    bufwrite32b(buf, (*head).checkSumAdjustment);
    bufwrite32b(buf, (*head).magicNumber);
    bufwrite16b(buf, (*head).flags);
    bufwrite16b(buf, (*head).unitsPerEm);
    bufwrite64b(buf, (*head).created as uint64_t);
    bufwrite64b(buf, (*head).modified as uint64_t);
    bufwrite16b(buf, (*head).xMin as uint16_t);
    bufwrite16b(buf, (*head).yMin as uint16_t);
    bufwrite16b(buf, (*head).xMax as uint16_t);
    bufwrite16b(buf, (*head).yMax as uint16_t);
    bufwrite16b(buf, (*head).macStyle);
    bufwrite16b(buf, (*head).lowestRecPPEM);
    bufwrite16b(buf, (*head).fontDirectoryHint as uint16_t);
    bufwrite16b(buf, (*head).indexToLocFormat as uint16_t);
    bufwrite16b(buf, (*head).glyphDataFormat as uint16_t);
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
unsafe extern "C" fn json_obj_getnum_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return fallback;
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
    return fallback;
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
unsafe extern "C" fn otfcc_parse_flags(
    mut v: *const json_value,
    mut labels: *mut *const ::core::ffi::c_char,
) -> uint32_t {
    if v.is_null() {
        return 0 as uint32_t;
    }
    if (*v).type_0 as ::core::ffi::c_uint
        == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*v).u.integer as uint32_t;
    } else if (*v).type_0 as ::core::ffi::c_uint
        == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*v).u.dbl as uint32_t;
    } else if (*v).type_0 as ::core::ffi::c_uint
        == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut flags: uint32_t = 0 as uint32_t;
        let mut j: uint16_t = 0 as uint16_t;
        while !(*labels.offset(j as isize)).is_null() {
            if json_obj_getbool(v, *labels.offset(j as isize)) {
                flags |= ((1 as ::core::ffi::c_int) << j as ::core::ffi::c_int) as uint32_t;
            }
            j = j.wrapping_add(1);
        }
        return flags;
    } else {
        return 0 as uint32_t;
    };
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
