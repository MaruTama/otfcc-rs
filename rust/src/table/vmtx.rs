use libc::{free, malloc, memcpy, memset};
extern "C" {
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{pos_to_u16, read_16u, read_16s};
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
pub type glyphid_t = u16;
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
pub struct vertical_metric {
    pub advanceHeight: length_t,
    pub tsb: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_vmtx {
    pub metrics: *mut vertical_metric,
    pub topSideBearing: *mut pos_t,
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
pub type font_file_pointer = *mut u8;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn disposeVmtx(mut table: *mut table_vmtx) {
    if !(*table).metrics.is_null() {
        free((*table).metrics as *mut ::core::ffi::c_void);
        (*table).metrics = ::core::ptr::null_mut::<vertical_metric>();
    }
    if !(*table).topSideBearing.is_null() {
        free((*table).topSideBearing as *mut ::core::ffi::c_void);
        (*table).topSideBearing = ::core::ptr::null_mut::<pos_t>();
    }
}
#[inline]
unsafe extern "C" fn table_vmtx_dispose(mut x: *mut table_vmtx) {
    disposeVmtx(x);
}
#[inline]
unsafe extern "C" fn table_vmtx_copy(mut dst: *mut table_vmtx, mut src: *const table_vmtx) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_vmtx>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_vmtx_create() -> *mut table_vmtx {
    let mut x: *mut table_vmtx =
        malloc(::core::mem::size_of::<table_vmtx>() as usize) as *mut table_vmtx;
    table_vmtx_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_vmtx_init(mut x: *mut table_vmtx) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_vmtx>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_vmtx_copyReplace(mut dst: *mut table_vmtx, src: table_vmtx) {
    table_vmtx_dispose(dst);
    table_vmtx_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_vmtx_move(mut dst: *mut table_vmtx, mut src: *mut table_vmtx) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_vmtx>() as usize,
    );
    table_vmtx_init(src);
}
#[inline]
unsafe extern "C" fn table_vmtx_replace(mut dst: *mut table_vmtx, src: table_vmtx) {
    table_vmtx_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_vmtx>() as usize,
    );
}
#[no_mangle]
pub static mut table_iVmtx: __caryll_elementinterface_table_vmtx = {
    __caryll_elementinterface_table_vmtx {
        init: Some(table_vmtx_init as unsafe extern "C" fn(*mut table_vmtx) -> ()),
        copy: Some(
            table_vmtx_copy as unsafe extern "C" fn(*mut table_vmtx, *const table_vmtx) -> (),
        ),
        move_0: Some(
            table_vmtx_move as unsafe extern "C" fn(*mut table_vmtx, *mut table_vmtx) -> (),
        ),
        dispose: Some(table_vmtx_dispose as unsafe extern "C" fn(*mut table_vmtx) -> ()),
        replace: Some(
            table_vmtx_replace as unsafe extern "C" fn(*mut table_vmtx, table_vmtx) -> (),
        ),
        copyReplace: Some(
            table_vmtx_copyReplace as unsafe extern "C" fn(*mut table_vmtx, table_vmtx) -> (),
        ),
        create: Some(table_vmtx_create),
        free: Some(table_vmtx_free as unsafe extern "C" fn(*mut table_vmtx) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_vmtx_free(mut x: *mut table_vmtx) {
    if x.is_null() {
        return;
    }
    table_vmtx_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readVmtx(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
    mut vhea: *mut table_vhea,
    mut maxp: *mut table_maxp,
) -> *mut table_vmtx {
    if vhea.is_null()
        || maxp.is_null()
        || (*vhea).numOfLongVerMetrics as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || ((*maxp).numGlyphs as ::core::ffi::c_int)
            < (*vhea).numOfLongVerMetrics as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<table_vmtx>();
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
            if table.tag == 1986884728i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    let mut vmtx: *mut table_vmtx = ::core::ptr::null_mut::<table_vmtx>();
                    let mut count_a: glyphid_t = (*vhea).numOfLongVerMetrics as glyphid_t;
                    let mut count_k: glyphid_t = ((*maxp).numGlyphs as ::core::ffi::c_int
                        - (*vhea).numOfLongVerMetrics as ::core::ffi::c_int)
                        as glyphid_t;
                    if length
                        < (count_a as ::core::ffi::c_int * 4 as ::core::ffi::c_int
                            + count_k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as u32
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Table 'vmtx' corrupted.\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                        if !vmtx.is_null() {
                            table_iVmtx.free.expect("non-null function pointer")(vmtx);
                            vmtx = ::core::ptr::null_mut::<table_vmtx>();
                        }
                    } else {
                        vmtx = __caryll_allocate_clean(
                            ::core::mem::size_of::<table_vmtx>() as usize,
                            27 as ::core::ffi::c_ulong,
                        ) as *mut table_vmtx;
                        (*vmtx).metrics = __caryll_allocate_clean(
                            (::core::mem::size_of::<vertical_metric>() as usize)
                                .wrapping_mul(count_a as usize),
                            28 as ::core::ffi::c_ulong,
                        ) as *mut vertical_metric;
                        (*vmtx).topSideBearing = __caryll_allocate_clean(
                            (::core::mem::size_of::<pos_t>() as usize)
                                .wrapping_mul(count_k as usize),
                            29 as ::core::ffi::c_ulong,
                        ) as *mut pos_t;
                        let mut ia: glyphid_t = 0 as glyphid_t;
                        while (ia as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
                            (*(*vmtx).metrics.offset(ia as isize)).advanceHeight =
                                read_16u(data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                ) as *const u8) as length_t;
                            (*(*vmtx).metrics.offset(ia as isize)).tsb = read_16s(
                                data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                )
                                .offset(2 as ::core::ffi::c_int as isize)
                                    as *const u8,
                            )
                                as pos_t;
                            ia = ia.wrapping_add(1);
                        }
                        let mut ik: glyphid_t = 0 as glyphid_t;
                        while (ik as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
                            *(*vmtx).topSideBearing.offset(ik as isize) = read_16s(
                                data.offset(
                                    (count_a as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                .offset(
                                    (ik as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                            )
                                as pos_t;
                            ik = ik.wrapping_add(1);
                        }
                        return vmtx;
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
    return ::core::ptr::null_mut::<table_vmtx>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildVmtx(
    mut vmtx: *const table_vmtx,
    mut count_a: glyphid_t,
    mut count_k: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if vmtx.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    if !(*vmtx).metrics.is_null() {
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            bufwrite16b(
                buf,
                (*(*vmtx).metrics.offset(j as isize)).advanceHeight as u16,
            );
            bufwrite16b(buf, pos_to_u16((*(*vmtx).metrics.offset(j as isize)).tsb));
            j = j.wrapping_add(1);
        }
    }
    if !(*vmtx).topSideBearing.is_null() {
        let mut j_0: glyphid_t = 0 as glyphid_t;
        while (j_0 as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
            bufwrite16b(
                buf,
                pos_to_u16(*(*vmtx).topSideBearing.offset(j_0 as isize)),
            );
            j_0 = j_0.wrapping_add(1);
        }
    }
    return buf;
}
