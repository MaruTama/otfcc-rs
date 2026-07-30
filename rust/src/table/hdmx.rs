#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u};

use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::maxp::{MaxpTable};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceRecord {
    pub pixelSize: u8,
    pub maxWidth: u8,
    pub widths: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdmxTable {
    pub version: u16,
    pub numRecords: u16,
    pub sizeDeviceRecord: u32,
    pub records: *mut DeviceRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HdmxTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut HdmxTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut HdmxTable, *const HdmxTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut HdmxTable, *mut HdmxTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut HdmxTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut HdmxTable, HdmxTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut HdmxTable, HdmxTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut HdmxTable>,
    pub free: Option<unsafe extern "C" fn(*mut HdmxTable) -> ()>,
}
#[inline]
unsafe extern "C" fn dispose_hdmx(mut table: *mut HdmxTable) {
    if (*table).records.is_null() {
        return;
    }
    let mut i: u32 = 0 as u32;
    while i < (*table).numRecords as u32 {
        if !(*(*table).records.offset(i as isize)).widths.is_null() {
            free((*(*table).records.offset(i as isize)).widths as *mut ::core::ffi::c_void);
            let ref mut fresh0 = (*(*table).records.offset(i as isize)).widths;
            *fresh0 = ::core::ptr::null_mut::<u8>();
        }
        i = i.wrapping_add(1);
    }
    free((*table).records as *mut ::core::ffi::c_void);
    (*table).records = ::core::ptr::null_mut::<DeviceRecord>();
}
pub static TABLE_I_HDMX: HdmxTableElementInterface = {
    HdmxTableElementInterface {
        init: Some(table_hdmx_init as unsafe extern "C" fn(*mut HdmxTable) -> ()),
        copy: Some(
            table_hdmx_copy as unsafe extern "C" fn(*mut HdmxTable, *const HdmxTable) -> (),
        ),
        move_0: Some(
            table_hdmx_move as unsafe extern "C" fn(*mut HdmxTable, *mut HdmxTable) -> (),
        ),
        dispose: Some(table_hdmx_dispose as unsafe extern "C" fn(*mut HdmxTable) -> ()),
        replace: Some(
            table_hdmx_replace as unsafe extern "C" fn(*mut HdmxTable, HdmxTable) -> (),
        ),
        copyReplace: Some(
            table_hdmx_copy_replace as unsafe extern "C" fn(*mut HdmxTable, HdmxTable) -> (),
        ),
        create: Some(table_hdmx_create),
        free: Some(table_hdmx_free as unsafe extern "C" fn(*mut HdmxTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_hdmx_dispose(mut x: *mut HdmxTable) {
    dispose_hdmx(x);
}
#[inline]
unsafe extern "C" fn table_hdmx_free(mut x: *mut HdmxTable) {
    if x.is_null() {
        return;
    }
    table_hdmx_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_hdmx_copy_replace(mut dst: *mut HdmxTable, src: HdmxTable) {
    table_hdmx_dispose(dst);
    table_hdmx_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_hdmx_copy(mut dst: *mut HdmxTable, mut src: *const HdmxTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HdmxTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hdmx_replace(mut dst: *mut HdmxTable, src: HdmxTable) {
    table_hdmx_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HdmxTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hdmx_create() -> *mut HdmxTable {
    let mut x: *mut HdmxTable =
        malloc(::core::mem::size_of::<HdmxTable>() as usize) as *mut HdmxTable;
    table_hdmx_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_hdmx_move(mut dst: *mut HdmxTable, mut src: *mut HdmxTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HdmxTable>() as usize,
    );
    table_hdmx_init(src);
}
#[inline]
unsafe extern "C" fn table_hdmx_init(mut x: *mut HdmxTable) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<HdmxTable>() as usize,
    );
}
pub unsafe extern "C" fn otfcc_read_hdmx(
    mut packet: Packet,
    mut _options: *const Options,
    mut maxp: *mut MaxpTable,
) -> *mut HdmxTable {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1751412088i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut hdmx: *mut HdmxTable = ::core::ptr::null_mut::<HdmxTable>();
                    hdmx = __caryll_allocate_clean(
                        ::core::mem::size_of::<HdmxTable>() as usize,
                        20 as ::core::ffi::c_ulong,
                    ) as *mut HdmxTable;
                    (*hdmx).version = read_16u(data as *const u8);
                    (*hdmx).numRecords =
                        read_16u(data.offset(2 as ::core::ffi::c_int as isize) as *const u8);
                    (*hdmx).sizeDeviceRecord =
                        read_32u(data.offset(4 as ::core::ffi::c_int as isize) as *const u8);
                    (*hdmx).records = __caryll_allocate_clean(
                        (::core::mem::size_of::<DeviceRecord>() as usize)
                            .wrapping_mul((*hdmx).numRecords as usize),
                        24 as ::core::ffi::c_ulong,
                    ) as *mut DeviceRecord;
                    let mut i: u32 = 0 as u32;
                    while i < (*hdmx).numRecords as u32 {
                        (*(*hdmx).records.offset(i as isize)).pixelSize = *data
                            .offset(8 as ::core::ffi::c_int as isize)
                            .offset(i.wrapping_mul(
                                (2 as ::core::ffi::c_int + (*maxp).numGlyphs as ::core::ffi::c_int)
                                    as u32,
                            ) as isize);
                        (*(*hdmx).records.offset(i as isize)).maxWidth = *data
                            .offset(8 as ::core::ffi::c_int as isize)
                            .offset(i.wrapping_mul(
                                (2 as ::core::ffi::c_int + (*maxp).numGlyphs as ::core::ffi::c_int)
                                    as u32,
                            ) as isize)
                            .offset(1 as ::core::ffi::c_int as isize);
                        let ref mut fresh1 = (*(*hdmx).records.offset(i as isize)).widths;
                        *fresh1 = __caryll_allocate_clean(
                            (::core::mem::size_of::<u8>() as usize)
                                .wrapping_mul((*maxp).numGlyphs as usize),
                            29 as ::core::ffi::c_ulong,
                        ) as *mut u8;
                        memcpy(
                            (*(*hdmx).records.offset(i as isize)).widths
                                as *mut ::core::ffi::c_void,
                            data.offset(8 as ::core::ffi::c_int as isize)
                                .offset(i.wrapping_mul(
                                    (2 as ::core::ffi::c_int
                                        + (*maxp).numGlyphs as ::core::ffi::c_int)
                                        as u32,
                                ) as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            (*maxp).numGlyphs as usize,
                        );
                        i = i.wrapping_add(1);
                    }
                    return hdmx;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<HdmxTable>();
}
