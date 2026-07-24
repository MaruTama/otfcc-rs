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
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t);
}

use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{pos_to_u16, read_16u, read_16s};
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
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
pub type glyphid_t = uint16_t;
pub type pos_t = ::core::ffi::c_double;
pub type length_t = ::core::ffi::c_double;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed = 10;
pub const log_vl_info: C2RustUnnamed = 5;
pub const log_vl_notice: C2RustUnnamed = 2;
pub const log_vl_important: C2RustUnnamed = 1;
pub const log_vl_critical: C2RustUnnamed = 0;
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
pub struct horizontal_metric {
    pub advanceWidth: length_t,
    pub lsb: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_hmtx {
    pub metrics: *mut horizontal_metric,
    pub leftSideBearing: *mut pos_t,
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
pub type font_file_pointer = *mut uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn disposeHmtx(mut table: *mut table_hmtx) {
    if !(*table).metrics.is_null() {
        free((*table).metrics as *mut ::core::ffi::c_void);
        (*table).metrics = ::core::ptr::null_mut::<horizontal_metric>();
    }
    if !(*table).leftSideBearing.is_null() {
        free((*table).leftSideBearing as *mut ::core::ffi::c_void);
        (*table).leftSideBearing = ::core::ptr::null_mut::<pos_t>();
    }
}
#[inline]
unsafe extern "C" fn table_hmtx_dispose(mut x: *mut table_hmtx) {
    disposeHmtx(x);
}
#[inline]
unsafe extern "C" fn table_hmtx_copy(mut dst: *mut table_hmtx, mut src: *const table_hmtx) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hmtx>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_hmtx_create() -> *mut table_hmtx {
    let mut x: *mut table_hmtx =
        malloc(::core::mem::size_of::<table_hmtx>() as size_t) as *mut table_hmtx;
    table_hmtx_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_hmtx_init(mut x: *mut table_hmtx) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_hmtx>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn table_hmtx_copyReplace(mut dst: *mut table_hmtx, src: table_hmtx) {
    table_hmtx_dispose(dst);
    table_hmtx_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_hmtx_move(mut dst: *mut table_hmtx, mut src: *mut table_hmtx) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hmtx>() as size_t,
    );
    table_hmtx_init(src);
}
#[inline]
unsafe extern "C" fn table_hmtx_replace(mut dst: *mut table_hmtx, src: table_hmtx) {
    table_hmtx_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hmtx>() as size_t,
    );
}
#[no_mangle]
pub static mut table_iHmtx: __caryll_elementinterface_table_hmtx = {
    __caryll_elementinterface_table_hmtx {
        init: Some(table_hmtx_init as unsafe extern "C" fn(*mut table_hmtx) -> ()),
        copy: Some(
            table_hmtx_copy as unsafe extern "C" fn(*mut table_hmtx, *const table_hmtx) -> (),
        ),
        move_0: Some(
            table_hmtx_move as unsafe extern "C" fn(*mut table_hmtx, *mut table_hmtx) -> (),
        ),
        dispose: Some(table_hmtx_dispose as unsafe extern "C" fn(*mut table_hmtx) -> ()),
        replace: Some(
            table_hmtx_replace as unsafe extern "C" fn(*mut table_hmtx, table_hmtx) -> (),
        ),
        copyReplace: Some(
            table_hmtx_copyReplace as unsafe extern "C" fn(*mut table_hmtx, table_hmtx) -> (),
        ),
        create: Some(table_hmtx_create),
        free: Some(table_hmtx_free as unsafe extern "C" fn(*mut table_hmtx) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_hmtx_free(mut x: *mut table_hmtx) {
    if x.is_null() {
        return;
    }
    table_hmtx_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readHmtx(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
    mut hhea: *mut table_hhea,
    mut maxp: *mut table_maxp,
) -> *mut table_hmtx {
    if hhea.is_null()
        || maxp.is_null()
        || (*hhea).numberOfMetrics == 0
        || ((*maxp).numGlyphs as ::core::ffi::c_int) < (*hhea).numberOfMetrics as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<table_hmtx>();
    }
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1752003704i32 as uint32_t {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: uint32_t = table.length;
                    let mut hmtx: *mut table_hmtx = ::core::ptr::null_mut::<table_hmtx>();
                    let mut count_a: glyphid_t = (*hhea).numberOfMetrics as glyphid_t;
                    let mut count_k: glyphid_t = ((*maxp).numGlyphs as ::core::ffi::c_int
                        - (*hhea).numberOfMetrics as ::core::ffi::c_int)
                        as glyphid_t;
                    if length
                        < (count_a as ::core::ffi::c_int * 4 as ::core::ffi::c_int
                            + count_k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as uint32_t
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as uint8_t,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Table 'hmtx' corrupted.\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                        if !hmtx.is_null() {
                            table_iHmtx.free.expect("non-null function pointer")(hmtx);
                            hmtx = ::core::ptr::null_mut::<table_hmtx>();
                        }
                    } else {
                        hmtx = __caryll_allocate_clean(
                            ::core::mem::size_of::<table_hmtx>() as size_t,
                            27 as ::core::ffi::c_ulong,
                        ) as *mut table_hmtx;
                        (*hmtx).metrics = __caryll_allocate_clean(
                            (::core::mem::size_of::<horizontal_metric>() as size_t)
                                .wrapping_mul(count_a as size_t),
                            28 as ::core::ffi::c_ulong,
                        ) as *mut horizontal_metric;
                        (*hmtx).leftSideBearing = __caryll_allocate_clean(
                            (::core::mem::size_of::<pos_t>() as size_t)
                                .wrapping_mul(count_k as size_t),
                            29 as ::core::ffi::c_ulong,
                        ) as *mut pos_t;
                        let mut ia: glyphid_t = 0 as glyphid_t;
                        while (ia as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
                            (*(*hmtx).metrics.offset(ia as isize)).advanceWidth =
                                read_16u(data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                ) as *const uint8_t) as length_t;
                            (*(*hmtx).metrics.offset(ia as isize)).lsb = read_16s(
                                data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                )
                                .offset(2 as ::core::ffi::c_int as isize)
                                    as *const uint8_t,
                            )
                                as pos_t;
                            ia = ia.wrapping_add(1);
                        }
                        let mut ik: glyphid_t = 0 as glyphid_t;
                        while (ik as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
                            *(*hmtx).leftSideBearing.offset(ik as isize) = read_16s(
                                data.offset(
                                    (count_a as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                .offset(
                                    (ik as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const uint8_t,
                            )
                                as pos_t;
                            ik = ik.wrapping_add(1);
                        }
                        return hmtx;
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
    return ::core::ptr::null_mut::<table_hmtx>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildHmtx(
    mut hmtx: *const table_hmtx,
    mut count_a: glyphid_t,
    mut count_k: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    let mut buf: *mut caryll_Buffer = bufnew();
    if hmtx.is_null() {
        return buf;
    }
    if !(*hmtx).metrics.is_null() {
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            bufwrite16b(
                buf,
                (*(*hmtx).metrics.offset(j as isize)).advanceWidth as uint16_t,
            );
            bufwrite16b(buf, pos_to_u16((*(*hmtx).metrics.offset(j as isize)).lsb));
            j = j.wrapping_add(1);
        }
    }
    if !(*hmtx).leftSideBearing.is_null() {
        let mut j_0: glyphid_t = 0 as glyphid_t;
        while (j_0 as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
            bufwrite16b(
                buf,
                pos_to_u16(*(*hmtx).leftSideBearing.offset(j_0 as isize)),
            );
            j_0 = j_0.wrapping_add(1);
        }
    }
    return buf;
}
