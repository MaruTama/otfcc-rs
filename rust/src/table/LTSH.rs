use libc::{free, malloc, memcpy, memset};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite8(buf: *mut caryll_Buffer, byte: u8);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
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
pub struct table_LTSH {
    pub version: u16,
    pub numGlyphs: glyphid_t,
    pub yPels: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_LTSH {
    pub init: Option<unsafe extern "C" fn(*mut table_LTSH) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_LTSH, *const table_LTSH) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_LTSH, *mut table_LTSH) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_LTSH) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_LTSH, table_LTSH) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_LTSH, table_LTSH) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_LTSH>,
    pub free: Option<unsafe extern "C" fn(*mut table_LTSH) -> ()>,
}
pub type font_file_pointer = *mut u8;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn disposeLTSH(mut ltsh: *mut table_LTSH) {
    if !ltsh.is_null() {
        free((*ltsh).yPels as *mut ::core::ffi::c_void);
        (*ltsh).yPels = ::core::ptr::null_mut::<u8>();
    }
}
#[inline]
unsafe extern "C" fn table_LTSH_free(mut x: *mut table_LTSH) {
    if x.is_null() {
        return;
    }
    table_LTSH_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub static mut table_iLTSH: __caryll_elementinterface_table_LTSH = {
    __caryll_elementinterface_table_LTSH {
        init: Some(table_LTSH_init as unsafe extern "C" fn(*mut table_LTSH) -> ()),
        copy: Some(
            table_LTSH_copy as unsafe extern "C" fn(*mut table_LTSH, *const table_LTSH) -> (),
        ),
        move_0: Some(
            table_LTSH_move as unsafe extern "C" fn(*mut table_LTSH, *mut table_LTSH) -> (),
        ),
        dispose: Some(table_LTSH_dispose as unsafe extern "C" fn(*mut table_LTSH) -> ()),
        replace: Some(
            table_LTSH_replace as unsafe extern "C" fn(*mut table_LTSH, table_LTSH) -> (),
        ),
        copyReplace: Some(
            table_LTSH_copyReplace as unsafe extern "C" fn(*mut table_LTSH, table_LTSH) -> (),
        ),
        create: Some(table_LTSH_create),
        free: Some(table_LTSH_free as unsafe extern "C" fn(*mut table_LTSH) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_LTSH_dispose(mut x: *mut table_LTSH) {
    disposeLTSH(x);
}
#[inline]
unsafe extern "C" fn table_LTSH_create() -> *mut table_LTSH {
    let mut x: *mut table_LTSH =
        malloc(::core::mem::size_of::<table_LTSH>() as usize) as *mut table_LTSH;
    table_LTSH_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_LTSH_init(mut x: *mut table_LTSH) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_LTSH>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_LTSH_copy(mut dst: *mut table_LTSH, mut src: *const table_LTSH) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_LTSH>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_LTSH_copyReplace(mut dst: *mut table_LTSH, src: table_LTSH) {
    table_LTSH_dispose(dst);
    table_LTSH_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_LTSH_move(mut dst: *mut table_LTSH, mut src: *mut table_LTSH) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_LTSH>() as usize,
    );
    table_LTSH_init(src);
}
#[inline]
unsafe extern "C" fn table_LTSH_replace(mut dst: *mut table_LTSH, src: table_LTSH) {
    table_LTSH_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_LTSH>() as usize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readLTSH(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
) -> *mut table_LTSH {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1280594760i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut LTSH: *mut table_LTSH = ::core::ptr::null_mut::<table_LTSH>();
                    LTSH = __caryll_allocate_clean(
                        ::core::mem::size_of::<table_LTSH>() as usize,
                        15 as ::core::ffi::c_ulong,
                    ) as *mut table_LTSH;
                    (*LTSH).version = read_16u(data as *const u8);
                    (*LTSH).numGlyphs =
                        read_16u(data.offset(2 as ::core::ffi::c_int as isize) as *const u8)
                            as glyphid_t;
                    (*LTSH).yPels = __caryll_allocate_clean(
                        (::core::mem::size_of::<u8>() as usize)
                            .wrapping_mul((*LTSH).numGlyphs as usize),
                        18 as ::core::ffi::c_ulong,
                    ) as *mut u8;
                    memcpy(
                        (*LTSH).yPels as *mut ::core::ffi::c_void,
                        data.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                        (*LTSH).numGlyphs as usize,
                    );
                    return LTSH;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_LTSH>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildLTSH(
    mut ltsh: *const table_LTSH,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if ltsh.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, (*ltsh).numGlyphs as u16);
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < (*ltsh).numGlyphs as ::core::ffi::c_int {
        bufwrite8(buf, *(*ltsh).yPels.offset(j as isize));
        j = j.wrapping_add(1);
    }
    return buf;
}
