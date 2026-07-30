#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::binio::{read_32u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::options::{Options};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::meta::types::{MetaEntry, MetaTable};
use crate::table::meta::types::{META_I_ENTRIES, TABLE_I_META};
use crate::vendor::sds::{sdsempty, sdsnewlen};
pub unsafe extern "C" fn otfcc_readMeta(
    packet: Packet,
    mut options: *const Options,
) -> *mut MetaTable {
    let mut version: u32 = 0;
    let mut flags: u32 = 0;
    let mut dataMapsCount: u32 = 0;
    let mut meta: *mut MetaTable = ::core::ptr::null_mut::<MetaTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
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
                                TABLE_I_META.create.expect("non-null function pointer"))();
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
                                    META_I_ENTRIES.push.expect("non-null function pointer")(
                                        &raw mut (*meta).entries,
                                        MetaEntry {
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
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(sdsempty(), b"Table 'meta' corrupted.\n"),
                    );
                    TABLE_I_META.free.expect("non-null function pointer")(meta);
                    meta = ::core::ptr::null_mut::<MetaTable>();
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
