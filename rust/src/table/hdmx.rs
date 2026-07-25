use libc::{free, malloc, memcpy, memset};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u};

use crate::support::options::{otfcc_Options};
use crate::support::primitives::{f16dot16, font_file_pointer};

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
pub struct device_record {
    pub pixelSize: u8,
    pub maxWidth: u8,
    pub widths: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_hdmx {
    pub version: u16,
    pub numRecords: u16,
    pub sizeDeviceRecord: u32,
    pub records: *mut device_record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_hdmx {
    pub init: Option<unsafe extern "C" fn(*mut table_hdmx) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_hdmx, *const table_hdmx) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_hdmx, *mut table_hdmx) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_hdmx) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_hdmx, table_hdmx) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_hdmx, table_hdmx) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_hdmx>,
    pub free: Option<unsafe extern "C" fn(*mut table_hdmx) -> ()>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn disposeHdmx(mut table: *mut table_hdmx) {
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
    (*table).records = ::core::ptr::null_mut::<device_record>();
}
#[no_mangle]
pub static mut table_iHdmx: __caryll_elementinterface_table_hdmx = {
    __caryll_elementinterface_table_hdmx {
        init: Some(table_hdmx_init as unsafe extern "C" fn(*mut table_hdmx) -> ()),
        copy: Some(
            table_hdmx_copy as unsafe extern "C" fn(*mut table_hdmx, *const table_hdmx) -> (),
        ),
        move_0: Some(
            table_hdmx_move as unsafe extern "C" fn(*mut table_hdmx, *mut table_hdmx) -> (),
        ),
        dispose: Some(table_hdmx_dispose as unsafe extern "C" fn(*mut table_hdmx) -> ()),
        replace: Some(
            table_hdmx_replace as unsafe extern "C" fn(*mut table_hdmx, table_hdmx) -> (),
        ),
        copyReplace: Some(
            table_hdmx_copyReplace as unsafe extern "C" fn(*mut table_hdmx, table_hdmx) -> (),
        ),
        create: Some(table_hdmx_create),
        free: Some(table_hdmx_free as unsafe extern "C" fn(*mut table_hdmx) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_hdmx_dispose(mut x: *mut table_hdmx) {
    disposeHdmx(x);
}
#[inline]
unsafe extern "C" fn table_hdmx_free(mut x: *mut table_hdmx) {
    if x.is_null() {
        return;
    }
    table_hdmx_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_hdmx_copyReplace(mut dst: *mut table_hdmx, src: table_hdmx) {
    table_hdmx_dispose(dst);
    table_hdmx_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_hdmx_copy(mut dst: *mut table_hdmx, mut src: *const table_hdmx) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hdmx>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hdmx_replace(mut dst: *mut table_hdmx, src: table_hdmx) {
    table_hdmx_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hdmx>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hdmx_create() -> *mut table_hdmx {
    let mut x: *mut table_hdmx =
        malloc(::core::mem::size_of::<table_hdmx>() as usize) as *mut table_hdmx;
    table_hdmx_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_hdmx_move(mut dst: *mut table_hdmx, mut src: *mut table_hdmx) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hdmx>() as usize,
    );
    table_hdmx_init(src);
}
#[inline]
unsafe extern "C" fn table_hdmx_init(mut x: *mut table_hdmx) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_hdmx>() as usize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readHdmx(
    mut packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
    mut maxp: *mut table_maxp,
) -> *mut table_hdmx {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1751412088i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut hdmx: *mut table_hdmx = ::core::ptr::null_mut::<table_hdmx>();
                    hdmx = __caryll_allocate_clean(
                        ::core::mem::size_of::<table_hdmx>() as usize,
                        20 as ::core::ffi::c_ulong,
                    ) as *mut table_hdmx;
                    (*hdmx).version = read_16u(data as *const u8);
                    (*hdmx).numRecords =
                        read_16u(data.offset(2 as ::core::ffi::c_int as isize) as *const u8);
                    (*hdmx).sizeDeviceRecord =
                        read_32u(data.offset(4 as ::core::ffi::c_int as isize) as *const u8);
                    (*hdmx).records = __caryll_allocate_clean(
                        (::core::mem::size_of::<device_record>() as usize)
                            .wrapping_mul((*hdmx).numRecords as usize),
                        24 as ::core::ffi::c_ulong,
                    ) as *mut device_record;
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
    return ::core::ptr::null_mut::<table_hdmx>();
}
