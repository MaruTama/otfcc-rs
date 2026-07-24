use libc::{free, malloc, memcpy, memset, strcmp};
extern "C" {
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32s};
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
pub type f16dot16 = i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_PacketPiece {
    pub tag: u32,
    pub checkSum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Packet {
    pub sfnt_version: u32,
    pub numTables: u16,
    pub searchRange: u16,
    pub entrySelector: u16,
    pub rangeShift: u16,
    pub pieces: *mut otfcc_PacketPiece,
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
pub struct __caryll_elementinterface_table_hhea {
    pub init: Option<unsafe extern "C" fn(*mut table_hhea) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_hhea, *const table_hhea) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_hhea, *mut table_hhea) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_hhea) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_hhea, table_hhea) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_hhea, table_hhea) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_hhea>,
    pub free: Option<unsafe extern "C" fn(*mut table_hhea) -> ()>,
}
pub type font_file_pointer = *mut u8;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn initHhea(mut hhea: *mut table_hhea) {
    memset(
        hhea as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_hhea>() as usize,
    );
    (*hhea).version = 0x10000 as ::core::ffi::c_int as f16dot16;
}
#[inline]
unsafe extern "C" fn disposeHhea(mut _hhea: *mut table_hhea) {}
#[inline]
unsafe extern "C" fn table_hhea_free(mut x: *mut table_hhea) {
    if x.is_null() {
        return;
    }
    table_hhea_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub static mut table_iHhea: __caryll_elementinterface_table_hhea = {
    __caryll_elementinterface_table_hhea {
        init: Some(table_hhea_init as unsafe extern "C" fn(*mut table_hhea) -> ()),
        copy: Some(
            table_hhea_copy as unsafe extern "C" fn(*mut table_hhea, *const table_hhea) -> (),
        ),
        move_0: Some(
            table_hhea_move as unsafe extern "C" fn(*mut table_hhea, *mut table_hhea) -> (),
        ),
        dispose: Some(table_hhea_dispose as unsafe extern "C" fn(*mut table_hhea) -> ()),
        replace: Some(
            table_hhea_replace as unsafe extern "C" fn(*mut table_hhea, table_hhea) -> (),
        ),
        copyReplace: Some(
            table_hhea_copyReplace as unsafe extern "C" fn(*mut table_hhea, table_hhea) -> (),
        ),
        create: Some(table_hhea_create),
        free: Some(table_hhea_free as unsafe extern "C" fn(*mut table_hhea) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_hhea_dispose(mut x: *mut table_hhea) {
    disposeHhea(x);
}
#[inline]
unsafe extern "C" fn table_hhea_create() -> *mut table_hhea {
    let mut x: *mut table_hhea =
        malloc(::core::mem::size_of::<table_hhea>() as usize) as *mut table_hhea;
    table_hhea_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_hhea_init(mut x: *mut table_hhea) {
    initHhea(x);
}
#[inline]
unsafe extern "C" fn table_hhea_replace(mut dst: *mut table_hhea, src: table_hhea) {
    table_hhea_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hhea>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hhea_copy(mut dst: *mut table_hhea, mut src: *const table_hhea) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hhea>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hhea_move(mut dst: *mut table_hhea, mut src: *mut table_hhea) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hhea>() as usize,
    );
    table_hhea_init(src);
}
#[inline]
unsafe extern "C" fn table_hhea_copyReplace(mut dst: *mut table_hhea, src: table_hhea) {
    table_hhea_dispose(dst);
    table_hhea_copy(dst, &raw const src);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readHhea(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_hhea {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1751672161i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    if length < 36 as u32 {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"table 'hhea' corrupted.\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                    } else {
                        let mut hhea: *mut table_hhea = ::core::ptr::null_mut::<table_hhea>();
                        hhea = __caryll_allocate_clean(
                            ::core::mem::size_of::<table_hhea>() as usize,
                            23 as ::core::ffi::c_ulong,
                        ) as *mut table_hhea;
                        (*hhea).version = read_32s(data as *const u8) as f16dot16;
                        (*hhea).ascender = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*hhea).descender = read_16u(
                            data.offset(6 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*hhea).lineGap = read_16u(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*hhea).advanceWidthMax = read_16u(
                            data.offset(10 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*hhea).minLeftSideBearing = read_16u(
                            data.offset(12 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).minRightSideBearing = read_16u(
                            data.offset(14 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).xMaxExtent = read_16u(
                            data.offset(16 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).caretSlopeRise = read_16u(
                            data.offset(18 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).caretSlopeRun = read_16u(
                            data.offset(20 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).caretOffset = read_16u(
                            data.offset(22 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).reserved[0 as ::core::ffi::c_int as usize] = read_16u(
                            data.offset(24 as ::core::ffi::c_int as isize) as *const u8,
                        )
                            as i16;
                        (*hhea).reserved[1 as ::core::ffi::c_int as usize] = read_16u(
                            data.offset(26 as ::core::ffi::c_int as isize) as *const u8,
                        )
                            as i16;
                        (*hhea).reserved[2 as ::core::ffi::c_int as usize] = read_16u(
                            data.offset(28 as ::core::ffi::c_int as isize) as *const u8,
                        )
                            as i16;
                        (*hhea).reserved[3 as ::core::ffi::c_int as usize] = read_16u(
                            data.offset(30 as ::core::ffi::c_int as isize) as *const u8,
                        )
                            as i16;
                        (*hhea).metricDataFormat = read_16u(
                            data.offset(32 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*hhea).numberOfMetrics = read_16u(
                            data.offset(34 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        return hhea;
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
    return ::core::ptr::null_mut::<table_hhea>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpHhea(
    mut table: *const table_hhea,
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
            b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut hhea: *mut json_value = json_object_new(13 as usize);
        json_object_push(
            hhea,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            hhea,
            b"ascender\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ascender as i64),
        );
        json_object_push(
            hhea,
            b"descender\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).descender as i64),
        );
        json_object_push(
            hhea,
            b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).lineGap as i64),
        );
        json_object_push(
            hhea,
            b"advanceWidthMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).advanceWidthMax as i64),
        );
        json_object_push(
            hhea,
            b"minLeftSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).minLeftSideBearing as i64),
        );
        json_object_push(
            hhea,
            b"minRightSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).minRightSideBearing as i64),
        );
        json_object_push(
            hhea,
            b"xMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).xMaxExtent as i64),
        );
        json_object_push(
            hhea,
            b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caretSlopeRise as i64),
        );
        json_object_push(
            hhea,
            b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caretSlopeRun as i64),
        );
        json_object_push(
            hhea,
            b"caretOffset\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).caretOffset as i64),
        );
        json_object_push(
            root,
            b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
            hhea,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseHhea(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_hhea {
    let mut hhea: *mut table_hhea = (
        table_iHhea.create.expect("non-null function pointer"))();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*hhea).version = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ));
            (*hhea).ascender = json_obj_getnum_fallback(
                table,
                b"ascender\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).descender = json_obj_getnum_fallback(
                table,
                b"descender\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).lineGap = json_obj_getnum_fallback(
                table,
                b"lineGap\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).advanceWidthMax = json_obj_getnum_fallback(
                table,
                b"advanceWidthMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*hhea).minLeftSideBearing = json_obj_getnum_fallback(
                table,
                b"minLeftSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).minRightSideBearing = json_obj_getnum_fallback(
                table,
                b"minRightSideBearing\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).xMaxExtent = json_obj_getnum_fallback(
                table,
                b"xMaxExtent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caretSlopeRise = json_obj_getnum_fallback(
                table,
                b"caretSlopeRise\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caretSlopeRun = json_obj_getnum_fallback(
                table,
                b"caretSlopeRun\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*hhea).caretOffset = json_obj_getnum_fallback(
                table,
                b"caretOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return hhea;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildHhea(
    mut hhea: *const table_hhea,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if hhea.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite32b(buf, (*hhea).version as u32);
    bufwrite16b(buf, (*hhea).ascender as u16);
    bufwrite16b(buf, (*hhea).descender as u16);
    bufwrite16b(buf, (*hhea).lineGap as u16);
    bufwrite16b(buf, (*hhea).advanceWidthMax);
    bufwrite16b(buf, (*hhea).minLeftSideBearing as u16);
    bufwrite16b(buf, (*hhea).minRightSideBearing as u16);
    bufwrite16b(buf, (*hhea).xMaxExtent as u16);
    bufwrite16b(buf, (*hhea).caretSlopeRise as u16);
    bufwrite16b(buf, (*hhea).caretSlopeRun as u16);
    bufwrite16b(buf, (*hhea).caretOffset as u16);
    bufwrite16b(
        buf,
        (*hhea).reserved[0 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(
        buf,
        (*hhea).reserved[1 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(
        buf,
        (*hhea).reserved[2 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(
        buf,
        (*hhea).reserved[3 as ::core::ffi::c_int as usize] as u16,
    );
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, (*hhea).numberOfMetrics);
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
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
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
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
