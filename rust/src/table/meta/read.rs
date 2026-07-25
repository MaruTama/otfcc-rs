extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static meta_iEntries: __caryll_vectorinterface_meta_Entries;
    static table_iMeta: __caryll_elementinterface_table_meta;
}
use crate::support::binio::{read_32u};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::options::{otfcc_Options};
use crate::vendor::sds::{sds};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

use crate::table::meta::types::{__caryll_elementinterface_table_meta, __caryll_vectorinterface_meta_Entries, meta_Entry, table_meta};
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
