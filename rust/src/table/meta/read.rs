extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static meta_iEntries: __caryll_vectorinterface_meta_Entries;
    static table_iMeta: __caryll_elementinterface_table_meta;
}
use crate::support::binio::{read_32u};
use crate::logger::{log_type_warning, otfcc_ILogger};
use crate::support::options::{otfcc_Options};
use crate::vendor::sds::{sds};
pub type otfcc_LoggerVerbosity = ::core::ffi::c_uint;
pub const log_vl_progress: otfcc_LoggerVerbosity = 10;
pub const log_vl_info: otfcc_LoggerVerbosity = 5;
pub const log_vl_notice: otfcc_LoggerVerbosity = 2;
pub const log_vl_important: otfcc_LoggerVerbosity = 1;
pub const log_vl_critical: otfcc_LoggerVerbosity = 0;
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
pub struct meta_Entry {
    pub tag: u32,
    pub data: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meta_Entries {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut meta_Entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_meta_Entries {
    pub init: Option<unsafe extern "C" fn(*mut meta_Entries) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut meta_Entries, *const meta_Entries) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut meta_Entries, *mut meta_Entries) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut meta_Entries) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut meta_Entries, meta_Entries) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut meta_Entries, meta_Entries) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut meta_Entries>,
    pub free: Option<unsafe extern "C" fn(*mut meta_Entries) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut meta_Entries, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut meta_Entries, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut meta_Entries>,
    pub fill: Option<unsafe extern "C" fn(*mut meta_Entries, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut meta_Entries) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut meta_Entries, meta_Entry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut meta_Entries) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut meta_Entries) -> meta_Entry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut meta_Entries, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut meta_Entries,
            Option<unsafe extern "C" fn(*const meta_Entry, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut meta_Entries,
            Option<unsafe extern "C" fn(*const meta_Entry, *const meta_Entry) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_meta {
    pub version: u32,
    pub flags: u32,
    pub entries: meta_Entries,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_meta {
    pub init: Option<unsafe extern "C" fn(*mut table_meta) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_meta, *const table_meta) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_meta, *mut table_meta) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_meta) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_meta, table_meta) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_meta, table_meta) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_meta>,
    pub free: Option<unsafe extern "C" fn(*mut table_meta) -> ()>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn otfcc_readMeta(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_meta {
    let mut version: u32 = 0;
    let mut flags: u32 = 0;
    let mut dataMapsCount: u32 = 0;
    let mut meta: *mut table_meta = ::core::ptr::null_mut::<table_meta>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1835365473i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    if !(table.length < 16 as u32) {
                        version = read_32u(table.data.offset(0 as ::core::ffi::c_int as isize));
                        flags = read_32u(table.data.offset(4 as ::core::ffi::c_int as isize));
                        dataMapsCount =
                            read_32u(table.data.offset(12 as ::core::ffi::c_int as isize));
                        if !(table.length
                            < (16 as u32)
                                .wrapping_add((12 as u32).wrapping_mul(dataMapsCount)))
                        {
                            meta = (
                                table_iMeta.create.expect("non-null function pointer"))();
                            (*meta).version = version;
                            (*meta).flags = flags;
                            let mut j: u32 = 0 as u32;
                            while j < dataMapsCount {
                                let mut tag: u32 = read_32u(
                                    table
                                        .data
                                        .offset(16 as ::core::ffi::c_int as isize)
                                        .offset((12 as u32).wrapping_mul(j) as isize)
                                        .offset(0 as ::core::ffi::c_int as isize),
                                );
                                let mut offset: u32 = read_32u(
                                    table
                                        .data
                                        .offset(16 as ::core::ffi::c_int as isize)
                                        .offset((12 as u32).wrapping_mul(j) as isize)
                                        .offset(4 as ::core::ffi::c_int as isize),
                                );
                                let mut length: u32 = read_32u(
                                    table
                                        .data
                                        .offset(16 as ::core::ffi::c_int as isize)
                                        .offset((12 as u32).wrapping_mul(j) as isize)
                                        .offset(8 as ::core::ffi::c_int as isize),
                                );
                                if !(table.length < offset.wrapping_add(length)) {
                                    meta_iEntries.push.expect("non-null function pointer")(
                                        &raw mut (*meta).entries,
                                        meta_Entry {
                                            tag: tag,
                                            data: sdsnewlen(
                                                table.data.offset(offset as isize)
                                                    as *mut ::core::ffi::c_char
                                                    as *const ::core::ffi::c_void,
                                                length as usize,
                                            ),
                                        },
                                    );
                                }
                                j = j.wrapping_add(1);
                            }
                            return meta;
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
                            b"Table 'meta' corrupted.\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        ),
                    );
                    table_iMeta.free.expect("non-null function pointer")(meta);
                    meta = ::core::ptr::null_mut::<table_meta>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return meta;
}
