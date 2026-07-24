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
pub type glyphid_t = u16;
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
pub struct VORG_entry {
    pub gid: glyphid_t,
    pub verticalOrigin: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_VORG {
    pub numVertOriginYMetrics: glyphid_t,
    pub defaultVerticalOrigin: pos_t,
    pub entries: *mut VORG_entry,
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
pub type font_file_pointer = *mut u8;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn disposeVORG(mut vorg: *mut table_VORG) {
    free((*vorg).entries as *mut ::core::ffi::c_void);
    (*vorg).entries = ::core::ptr::null_mut::<VORG_entry>();
}
#[no_mangle]
pub static mut table_iVORG: __caryll_elementinterface_table_VORG = {
    __caryll_elementinterface_table_VORG {
        init: Some(table_VORG_init as unsafe extern "C" fn(*mut table_VORG) -> ()),
        copy: Some(
            table_VORG_copy as unsafe extern "C" fn(*mut table_VORG, *const table_VORG) -> (),
        ),
        move_0: Some(
            table_VORG_move as unsafe extern "C" fn(*mut table_VORG, *mut table_VORG) -> (),
        ),
        dispose: Some(table_VORG_dispose as unsafe extern "C" fn(*mut table_VORG) -> ()),
        replace: Some(
            table_VORG_replace as unsafe extern "C" fn(*mut table_VORG, table_VORG) -> (),
        ),
        copyReplace: Some(
            table_VORG_copyReplace as unsafe extern "C" fn(*mut table_VORG, table_VORG) -> (),
        ),
        create: Some(table_VORG_create),
        free: Some(table_VORG_free as unsafe extern "C" fn(*mut table_VORG) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_VORG_free(mut x: *mut table_VORG) {
    if x.is_null() {
        return;
    }
    table_VORG_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_VORG_dispose(mut x: *mut table_VORG) {
    disposeVORG(x);
}
#[inline]
unsafe extern "C" fn table_VORG_init(mut x: *mut table_VORG) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_VORG>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_VORG_copyReplace(mut dst: *mut table_VORG, src: table_VORG) {
    table_VORG_dispose(dst);
    table_VORG_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_VORG_copy(mut dst: *mut table_VORG, mut src: *const table_VORG) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_VORG>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_VORG_replace(mut dst: *mut table_VORG, src: table_VORG) {
    table_VORG_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_VORG>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_VORG_move(mut dst: *mut table_VORG, mut src: *mut table_VORG) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_VORG>() as usize,
    );
    table_VORG_init(src);
}
#[inline]
unsafe extern "C" fn table_VORG_create() -> *mut table_VORG {
    let mut x: *mut table_VORG =
        malloc(::core::mem::size_of::<table_VORG>() as usize) as *mut table_VORG;
    table_VORG_init(x);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readVORG(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_VORG {
    let mut numVertOriginYMetrics: u16 = 0;
    let mut vorg: *mut table_VORG = ::core::ptr::null_mut::<table_VORG>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1448038983i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    if !(length < 8 as u32) {
                        numVertOriginYMetrics = read_16u(
                            data.offset(6 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        if !(length
                            < (8 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                    * numVertOriginYMetrics as ::core::ffi::c_int)
                                as u32)
                        {
                            vorg = (
                                table_iVORG.create.expect("non-null function pointer"))();
                            (*vorg).defaultVerticalOrigin = read_16s(
                                data.offset(4 as ::core::ffi::c_int as isize) as *const u8,
                            ) as pos_t;
                            (*vorg).numVertOriginYMetrics = numVertOriginYMetrics as glyphid_t;
                            (*vorg).entries = __caryll_allocate_clean(
                                (::core::mem::size_of::<VORG_entry>() as usize)
                                    .wrapping_mul(numVertOriginYMetrics as usize),
                                22 as ::core::ffi::c_ulong,
                            ) as *mut VORG_entry;
                            let mut j: u16 = 0 as u16;
                            while (j as ::core::ffi::c_int)
                                < numVertOriginYMetrics as ::core::ffi::c_int
                            {
                                (*(*vorg).entries.offset(j as isize)).gid = read_16u(
                                    data.offset(8 as ::core::ffi::c_int as isize).offset(
                                        (4 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                                )
                                    as glyphid_t;
                                (*(*vorg).entries.offset(j as isize)).verticalOrigin = read_16s(
                                    data.offset(8 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (4 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                j = j.wrapping_add(1);
                            }
                            return vorg;
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
                            b"Table 'VORG' corrupted.\0" as *const u8 as *const ::core::ffi::c_char,
                        ),
                    );
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_VORG>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildVORG(
    mut table: *const table_VORG,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if table.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite16b(buf, 1 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, pos_to_u16((*table).defaultVerticalOrigin));
    bufwrite16b(buf, (*table).numVertOriginYMetrics as u16);
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < (*table).numVertOriginYMetrics as ::core::ffi::c_int {
        bufwrite16b(buf, (*(*table).entries.offset(j as isize)).gid as u16);
        bufwrite16b(
            buf,
            (*(*table).entries.offset(j as isize)).verticalOrigin as u16,
        );
        j = j.wrapping_add(1);
    }
    return buf;
}
