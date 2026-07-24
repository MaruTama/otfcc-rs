extern "C" {
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
}

use crate::support::stdio::{FILE, stderr};
use crate::support::alloc::{__caryll_allocate_clean};
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
pub struct otfcc_SplineFontContainer {
    pub type_0: u32,
    pub count: u32,
    pub offsets: *mut u32,
    pub packets: *mut otfcc_Packet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i1: [u8; 4],
    pub i4: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub i1: [u8; 2],
    pub i2: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i1: [u8; 2],
    pub i2: u16,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn otfcc_read_packets(
    mut font: *mut otfcc_SplineFontContainer,
    mut file: *mut FILE,
) {
    let mut count: u32 = 0 as u32;
    while count < (*font).count {
        fseek(
            file,
            *(*font).offsets.offset(count as isize) as ::core::ffi::c_long,
            SEEK_SET,
        );
        (*(*font).packets.offset(count as isize)).sfnt_version = otfcc_get32u(file);
        (*(*font).packets.offset(count as isize)).numTables = otfcc_get16u(file);
        (*(*font).packets.offset(count as isize)).searchRange = otfcc_get16u(file);
        (*(*font).packets.offset(count as isize)).entrySelector = otfcc_get16u(file);
        (*(*font).packets.offset(count as isize)).rangeShift = otfcc_get16u(file);
        let ref mut fresh0 = (*(*font).packets.offset(count as isize)).pieces;
        *fresh0 = __caryll_allocate_clean(
            (::core::mem::size_of::<otfcc_PacketPiece>() as usize)
                .wrapping_mul((*(*font).packets.offset(count as isize)).numTables as usize),
            13 as ::core::ffi::c_ulong,
        ) as *mut otfcc_PacketPiece;
        let mut i: u32 = 0 as u32;
        while i < (*(*font).packets.offset(count as isize)).numTables as u32 {
            (*(*(*font).packets.offset(count as isize))
                .pieces
                .offset(i as isize))
            .tag = otfcc_get32u(file);
            (*(*(*font).packets.offset(count as isize))
                .pieces
                .offset(i as isize))
            .checkSum = otfcc_get32u(file);
            (*(*(*font).packets.offset(count as isize))
                .pieces
                .offset(i as isize))
            .offset = otfcc_get32u(file);
            (*(*(*font).packets.offset(count as isize))
                .pieces
                .offset(i as isize))
            .length = otfcc_get32u(file);
            let ref mut fresh1 = (*(*(*font).packets.offset(count as isize))
                .pieces
                .offset(i as isize))
            .data;
            *fresh1 = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul(
                    (*(*(*font).packets.offset(count as isize))
                        .pieces
                        .offset(i as isize))
                    .length as usize,
                ),
                20 as ::core::ffi::c_ulong,
            ) as *mut u8;
            i = i.wrapping_add(1);
        }
        let mut i_0: u32 = 0 as u32;
        while i_0
            < (*(*font).packets.offset(0 as ::core::ffi::c_int as isize)).numTables as u32
        {
            fseek(
                file,
                (*(*(*font).packets.offset(count as isize))
                    .pieces
                    .offset(i_0 as isize))
                .offset as ::core::ffi::c_long,
                SEEK_SET,
            );
            fread(
                (*(*(*font).packets.offset(count as isize))
                    .pieces
                    .offset(i_0 as isize))
                .data as *mut ::core::ffi::c_void,
                (*(*(*font).packets.offset(count as isize))
                    .pieces
                    .offset(i_0 as isize))
                .length as usize,
                1 as usize,
                file,
            );
            i_0 = i_0.wrapping_add(1);
        }
        count = count.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readSFNT(mut file: *mut FILE) -> *mut otfcc_SplineFontContainer {
    if file.is_null() {
        return ::core::ptr::null_mut::<otfcc_SplineFontContainer>();
    }
    let mut font: *mut otfcc_SplineFontContainer =
        ::core::ptr::null_mut::<otfcc_SplineFontContainer>();
    font = __caryll_allocate_clean(
        ::core::mem::size_of::<otfcc_SplineFontContainer>() as usize,
        34 as ::core::ffi::c_ulong,
    ) as *mut otfcc_SplineFontContainer;
    (*font).type_0 = otfcc_get32u(file);
    match (*font).type_0 {
        1330926671 | 65536 | 1953658213 | 1954115633 => {
            (*font).count = 1 as u32;
            (*font).offsets = __caryll_allocate_clean(
                (::core::mem::size_of::<u32>() as usize)
                    .wrapping_mul((*font).count as usize),
                44 as ::core::ffi::c_ulong,
            ) as *mut u32;
            (*font).packets = __caryll_allocate_clean(
                (::core::mem::size_of::<otfcc_Packet>() as usize)
                    .wrapping_mul((*font).count as usize),
                45 as ::core::ffi::c_ulong,
            ) as *mut otfcc_Packet;
            *(*font).offsets.offset(0 as ::core::ffi::c_int as isize) = 0 as u32;
            otfcc_read_packets(font, file);
        }
        1953784678 => {
            otfcc_get32u(file);
            (*font).count = otfcc_get32u(file);
            (*font).offsets = __caryll_allocate_clean(
                (::core::mem::size_of::<u32>() as usize)
                    .wrapping_mul((*font).count as usize),
                53 as ::core::ffi::c_ulong,
            ) as *mut u32;
            (*font).packets = __caryll_allocate_clean(
                (::core::mem::size_of::<otfcc_Packet>() as usize)
                    .wrapping_mul((*font).count as usize),
                54 as ::core::ffi::c_ulong,
            ) as *mut otfcc_Packet;
            let mut i: u32 = 0 as u32;
            while i < (*font).count {
                *(*font).offsets.offset(i as isize) = otfcc_get32u(file);
                i = i.wrapping_add(1);
            }
            otfcc_read_packets(font, file);
        }
        _ => {
            (*font).count = 0 as u32;
            (*font).offsets = ::core::ptr::null_mut::<u32>();
            (*font).packets = ::core::ptr::null_mut::<otfcc_Packet>();
        }
    }
    fclose(file);
    return font;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_deleteSFNT(mut font: *mut otfcc_SplineFontContainer) {
    if font.is_null() {
        return;
    }
    if (*font).count > 0 as u32 {
        let mut count: u32 = 0 as u32;
        while count < (*font).count {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*(*font).packets.offset(count as isize)).numTables as ::core::ffi::c_int {
                free(
                    (*(*(*font).packets.offset(count as isize))
                        .pieces
                        .offset(i as isize))
                    .data as *mut ::core::ffi::c_void,
                );
                let ref mut fresh2 = (*(*(*font).packets.offset(count as isize))
                    .pieces
                    .offset(i as isize))
                .data;
                *fresh2 = ::core::ptr::null_mut::<u8>();
                i += 1;
            }
            free((*(*font).packets.offset(count as isize)).pieces as *mut ::core::ffi::c_void);
            let ref mut fresh3 = (*(*font).packets.offset(count as isize)).pieces;
            *fresh3 = ::core::ptr::null_mut::<otfcc_PacketPiece>();
            count = count.wrapping_add(1);
        }
        free((*font).packets as *mut ::core::ffi::c_void);
        (*font).packets = ::core::ptr::null_mut::<otfcc_Packet>();
    }
    free((*font).offsets as *mut ::core::ffi::c_void);
    (*font).offsets = ::core::ptr::null_mut::<u32>();
    free(font as *mut ::core::ffi::c_void);
    font = ::core::ptr::null_mut::<otfcc_SplineFontContainer>();
}
#[inline]
unsafe extern "C" fn otfcc_check_endian() -> bool {
    let mut check_union: C2RustUnnamed_0 = C2RustUnnamed_0 {
        i2: 1 as ::core::ffi::c_int as u16,
    };
    return check_union.i1[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn otfcc_endian_convert16(mut i: u16) -> u16 {
    if otfcc_check_endian() {
        let mut src: C2RustUnnamed_1 = C2RustUnnamed_1 { i1: [0; 2] };
        let mut des: C2RustUnnamed_1 = C2RustUnnamed_1 { i1: [0; 2] };
        src.i2 = i;
        des.i1[0 as ::core::ffi::c_int as usize] = src.i1[1 as ::core::ffi::c_int as usize];
        des.i1[1 as ::core::ffi::c_int as usize] = src.i1[0 as ::core::ffi::c_int as usize];
        return des.i2;
    } else {
        return i;
    };
}
#[inline]
unsafe extern "C" fn otfcc_endian_convert32(mut i: u32) -> u32 {
    if otfcc_check_endian() {
        let mut src: C2RustUnnamed = C2RustUnnamed { i1: [0; 4] };
        let mut des: C2RustUnnamed = C2RustUnnamed { i1: [0; 4] };
        src.i4 = i;
        des.i1[0 as ::core::ffi::c_int as usize] = src.i1[3 as ::core::ffi::c_int as usize];
        des.i1[1 as ::core::ffi::c_int as usize] = src.i1[2 as ::core::ffi::c_int as usize];
        des.i1[2 as ::core::ffi::c_int as usize] = src.i1[1 as ::core::ffi::c_int as usize];
        des.i1[3 as ::core::ffi::c_int as usize] = src.i1[0 as ::core::ffi::c_int as usize];
        return des.i4;
    } else {
        return i;
    };
}
#[inline]
unsafe extern "C" fn otfcc_get16u(mut file: *mut FILE) -> u16 {
    let mut tmp: u16 = 0;
    let mut sizeRead: usize = fread(
        &raw mut tmp as *mut ::core::ffi::c_void,
        2 as usize,
        1 as usize,
        file,
    ) as usize;
    if sizeRead == 0 {
        fprintf(
            stderr,
            b"File corruption of terminated unexpectedly.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    return otfcc_endian_convert16(tmp);
}
#[inline]
unsafe extern "C" fn otfcc_get32u(mut file: *mut FILE) -> u32 {
    let mut tmp: u32 = 0;
    let mut sizeRead: usize = fread(
        &raw mut tmp as *mut ::core::ffi::c_void,
        4 as usize,
        1 as usize,
        file,
    ) as usize;
    if sizeRead == 0 {
        fprintf(
            stderr,
            b"File corruption of terminated unexpectedly.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    return otfcc_endian_convert32(tmp);
}
