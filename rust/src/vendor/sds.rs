#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcmp, memcpy, memmove, memset, realloc, strchr, strlen};

use crate::support::ctype_compat::{c_isprint, c_isspace, c_tolower, c_toupper};
pub type SdsRaw = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SdsHdr5 {
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SdsHdr8 {
    pub len: u8,
    pub alloc: u8,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SdsHdr16 {
    pub len: u16,
    pub alloc: u16,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SdsHdr32 {
    pub len: u32,
    pub alloc: u32,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SdsHdr64 {
    pub len: u64,
    pub alloc: u64,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
pub const SDS_MAX_PREALLOC: ::core::ffi::c_int =
    1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[inline]
unsafe extern "C" fn sdsavail(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return 0 as usize,
        SDS_TYPE_8 => {
            let mut sh: *mut SdsHdr8 = s
                .offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut ::core::ffi::c_void as *mut SdsHdr8;
            return ((*sh).alloc as ::core::ffi::c_int - (*sh).len as ::core::ffi::c_int) as usize;
        }
        SDS_TYPE_16 => {
            let mut sh_0: *mut SdsHdr16 =
                s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                    as *mut ::core::ffi::c_void as *mut SdsHdr16;
            return ((*sh_0).alloc as ::core::ffi::c_int - (*sh_0).len as ::core::ffi::c_int)
                as usize;
        }
        SDS_TYPE_32 => {
            let mut sh_1: *mut SdsHdr32 =
                s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                    as *mut ::core::ffi::c_void as *mut SdsHdr32;
            return (*sh_1).alloc.wrapping_sub((*sh_1).len) as usize;
        }
        SDS_TYPE_64 => {
            let mut sh_2: *mut SdsHdr64 =
                s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                    as *mut ::core::ffi::c_void as *mut SdsHdr64;
            return ((*sh_2).alloc as usize).wrapping_sub((*sh_2).len as usize);
        }
        _ => {}
    }
    return 0 as usize;
}
#[inline]
unsafe extern "C" fn sdssetlen(mut s: SdsRaw, mut newlen: usize) {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => {
            let mut fp: *mut ::core::ffi::c_uchar =
                (s as *mut ::core::ffi::c_uchar).offset(-(1 as ::core::ffi::c_int as isize));
            *fp = (SDS_TYPE_5 as usize | newlen << SDS_TYPE_BITS) as ::core::ffi::c_uchar;
        }
        SDS_TYPE_8 => {
            (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize)) as *mut SdsHdr8))
                .len = newlen as u8;
        }
        SDS_TYPE_16 => {
            (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len = newlen as u16;
        }
        SDS_TYPE_32 => {
            (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len = newlen as u32;
        }
        SDS_TYPE_64 => {
            (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len = newlen as u64;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn sdsalloc(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .alloc as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .alloc as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .alloc as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .alloc as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[inline]
unsafe extern "C" fn sdssetalloc(mut s: SdsRaw, mut newlen: usize) {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_8 => {
            (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize)) as *mut SdsHdr8))
                .alloc = newlen as u8;
        }
        SDS_TYPE_16 => {
            (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .alloc = newlen as u16;
        }
        SDS_TYPE_32 => {
            (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .alloc = newlen as u32;
        }
        SDS_TYPE_64 => {
            (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .alloc = newlen as u64;
        }
        SDS_TYPE_5 | _ => {}
    };
}
#[inline]
unsafe extern "C" fn sds_hdr_size(mut type_0: ::core::ffi::c_char) -> ::core::ffi::c_int {
    match type_0 as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return ::core::mem::size_of::<SdsHdr5>() as ::core::ffi::c_int,
        SDS_TYPE_8 => return ::core::mem::size_of::<SdsHdr8>() as ::core::ffi::c_int,
        SDS_TYPE_16 => return ::core::mem::size_of::<SdsHdr16>() as ::core::ffi::c_int,
        SDS_TYPE_32 => return ::core::mem::size_of::<SdsHdr32>() as ::core::ffi::c_int,
        SDS_TYPE_64 => return ::core::mem::size_of::<SdsHdr64>() as ::core::ffi::c_int,
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn sds_req_type(mut string_size: usize) -> ::core::ffi::c_char {
    if string_size < 32 as usize {
        return SDS_TYPE_5 as ::core::ffi::c_char;
    }
    if string_size < 0xff as usize {
        return SDS_TYPE_8 as ::core::ffi::c_char;
    }
    if string_size < 0xffff as usize {
        return SDS_TYPE_16 as ::core::ffi::c_char;
    }
    if string_size < 0xffffffff as ::core::ffi::c_uint as usize {
        return SDS_TYPE_32 as ::core::ffi::c_char;
    }
    return SDS_TYPE_64 as ::core::ffi::c_char;
}
pub unsafe extern "C" fn sdsnewlen(
    mut init: *const ::core::ffi::c_void,
    mut initlen: usize,
) -> SdsRaw {
    let mut sh: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut s: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut type_0: ::core::ffi::c_char = sds_req_type(initlen);
    if type_0 as ::core::ffi::c_int == SDS_TYPE_5 && initlen == 0 as usize {
        type_0 = SDS_TYPE_8 as ::core::ffi::c_char;
    }
    let mut hdrlen: ::core::ffi::c_int = sds_hdr_size(type_0);
    let mut fp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    sh = malloc(
        (hdrlen as usize)
            .wrapping_add(initlen)
            .wrapping_add(1 as usize),
    );
    if init.is_null() {
        memset(
            sh,
            0 as ::core::ffi::c_int,
            (hdrlen as usize)
                .wrapping_add(initlen)
                .wrapping_add(1 as usize),
        );
    }
    if sh.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    s = (sh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as SdsRaw;
    fp = (s as *mut ::core::ffi::c_uchar).offset(-(1 as ::core::ffi::c_int as isize));
    match type_0 as ::core::ffi::c_int {
        SDS_TYPE_5 => {
            *fp = (type_0 as usize | initlen << SDS_TYPE_BITS) as ::core::ffi::c_uchar;
        }
        SDS_TYPE_8 => {
            let mut sh_0: *mut SdsHdr8 =
                s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                    as *mut ::core::ffi::c_void as *mut SdsHdr8;
            (*sh_0).len = initlen as u8;
            (*sh_0).alloc = initlen as u8;
            *fp = type_0 as ::core::ffi::c_uchar;
        }
        SDS_TYPE_16 => {
            let mut sh_1: *mut SdsHdr16 =
                s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                    as *mut ::core::ffi::c_void as *mut SdsHdr16;
            (*sh_1).len = initlen as u16;
            (*sh_1).alloc = initlen as u16;
            *fp = type_0 as ::core::ffi::c_uchar;
        }
        SDS_TYPE_32 => {
            let mut sh_2: *mut SdsHdr32 =
                s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                    as *mut ::core::ffi::c_void as *mut SdsHdr32;
            (*sh_2).len = initlen as u32;
            (*sh_2).alloc = initlen as u32;
            *fp = type_0 as ::core::ffi::c_uchar;
        }
        SDS_TYPE_64 => {
            let mut sh_3: *mut SdsHdr64 =
                s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                    as *mut ::core::ffi::c_void as *mut SdsHdr64;
            (*sh_3).len = initlen as u64;
            (*sh_3).alloc = initlen as u64;
            *fp = type_0 as ::core::ffi::c_uchar;
        }
        _ => {}
    }
    if initlen != 0 && !init.is_null() {
        memcpy(s as *mut ::core::ffi::c_void, init, initlen);
    }
    *s.offset(initlen as isize) = '\0' as i32 as ::core::ffi::c_char;
    return s;
}
pub unsafe extern "C" fn sdsempty() -> SdsRaw {
    return sdsnewlen(
        b"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        0 as usize,
    );
}
pub unsafe extern "C" fn sdsnew(mut init: *const ::core::ffi::c_char) -> SdsRaw {
    let mut initlen: usize = if init.is_null() {
        0 as usize
    } else {
        strlen(init)
    };
    return sdsnewlen(init as *const ::core::ffi::c_void, initlen);
}
pub unsafe extern "C" fn sdsdup(s: SdsRaw) -> SdsRaw {
    return sdsnewlen(s as *const ::core::ffi::c_void, sdslen(s));
}
pub unsafe extern "C" fn sdsfree(mut s: SdsRaw) {
    if s.is_null() {
        return;
    }
    free(
        s.offset(-(sds_hdr_size(*s.offset(-(1 as ::core::ffi::c_int) as isize)) as isize))
            as *mut ::core::ffi::c_void,
    );
}
pub unsafe extern "C" fn sdsupdatelen(mut s: SdsRaw) {
    let mut reallen: ::core::ffi::c_int =
        strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_int;
    sdssetlen(s, reallen as usize);
}
pub unsafe extern "C" fn sdsclear(mut s: SdsRaw) {
    sdssetlen(s, 0 as usize);
    *s.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
}
pub unsafe extern "C" fn sds_make_room_for(mut s: SdsRaw, mut addlen: usize) -> SdsRaw {
    let mut sh: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut newsh: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut avail: usize = sdsavail(s);
    let mut len: usize = 0;
    let mut newlen: usize = 0;
    let mut type_0: ::core::ffi::c_char = 0;
    let mut oldtype: ::core::ffi::c_char = (*s.offset(-(1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        & SDS_TYPE_MASK) as ::core::ffi::c_char;
    let mut hdrlen: ::core::ffi::c_int = 0;
    if avail >= addlen {
        return s;
    }
    len = sdslen(s);
    sh = s.offset(-(sds_hdr_size(oldtype) as isize)) as *mut ::core::ffi::c_void;
    newlen = len.wrapping_add(addlen);
    if newlen < SDS_MAX_PREALLOC as usize {
        newlen = newlen.wrapping_mul(2 as usize);
    } else {
        newlen = newlen.wrapping_add(SDS_MAX_PREALLOC as usize);
    }
    type_0 = sds_req_type(newlen);
    if type_0 as ::core::ffi::c_int == SDS_TYPE_5 {
        type_0 = SDS_TYPE_8 as ::core::ffi::c_char;
    }
    hdrlen = sds_hdr_size(type_0);
    if oldtype as ::core::ffi::c_int == type_0 as ::core::ffi::c_int {
        newsh = realloc(
            sh,
            (hdrlen as usize)
                .wrapping_add(newlen)
                .wrapping_add(1 as usize),
        );
        if newsh.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        s = (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as SdsRaw;
    } else {
        newsh = malloc(
            (hdrlen as usize)
                .wrapping_add(newlen)
                .wrapping_add(1 as usize),
        );
        if newsh.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        memcpy(
            (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            len.wrapping_add(1 as usize),
        );
        free(sh);
        s = (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as SdsRaw;
        *s.offset(-(1 as ::core::ffi::c_int) as isize) = type_0;
        sdssetlen(s, len);
    }
    sdssetalloc(s, newlen);
    return s;
}
pub unsafe extern "C" fn sds_remove_free_space(mut s: SdsRaw) -> SdsRaw {
    let mut sh: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut newsh: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut type_0: ::core::ffi::c_char = 0;
    let mut oldtype: ::core::ffi::c_char = (*s.offset(-(1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        & SDS_TYPE_MASK) as ::core::ffi::c_char;
    let mut hdrlen: ::core::ffi::c_int = 0;
    let mut len: usize = sdslen(s);
    sh = s.offset(-(sds_hdr_size(oldtype) as isize)) as *mut ::core::ffi::c_void;
    type_0 = sds_req_type(len);
    hdrlen = sds_hdr_size(type_0);
    if oldtype as ::core::ffi::c_int == type_0 as ::core::ffi::c_int {
        newsh = realloc(
            sh,
            (hdrlen as usize)
                .wrapping_add(len)
                .wrapping_add(1 as usize),
        );
        if newsh.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        s = (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as SdsRaw;
    } else {
        newsh = malloc(
            (hdrlen as usize)
                .wrapping_add(len)
                .wrapping_add(1 as usize),
        );
        if newsh.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        memcpy(
            (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            len.wrapping_add(1 as usize),
        );
        free(sh);
        s = (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as SdsRaw;
        *s.offset(-(1 as ::core::ffi::c_int) as isize) = type_0;
        sdssetlen(s, len);
    }
    sdssetalloc(s, len);
    return s;
}
pub unsafe extern "C" fn sds_alloc_size(mut s: SdsRaw) -> usize {
    let mut alloc: usize = sdsalloc(s);
    return (sds_hdr_size(*s.offset(-(1 as ::core::ffi::c_int) as isize)) as usize)
        .wrapping_add(alloc)
        .wrapping_add(1 as usize);
}
pub unsafe extern "C" fn sds_alloc_ptr(mut s: SdsRaw) -> *mut ::core::ffi::c_void {
    return s.offset(-(sds_hdr_size(*s.offset(-(1 as ::core::ffi::c_int) as isize)) as isize))
        as *mut ::core::ffi::c_void;
}
pub unsafe extern "C" fn sds_incr_len(mut s: SdsRaw, mut incr: ::core::ffi::c_int) {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    let mut len: usize = 0;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => {
            let mut fp: *mut ::core::ffi::c_uchar =
                (s as *mut ::core::ffi::c_uchar).offset(-(1 as ::core::ffi::c_int as isize));
            let mut oldlen: ::core::ffi::c_uchar =
                (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as ::core::ffi::c_uchar;
            *fp = (SDS_TYPE_5 | oldlen as ::core::ffi::c_int + incr << SDS_TYPE_BITS)
                as ::core::ffi::c_uchar;
            len = (oldlen as ::core::ffi::c_int + incr) as usize;
        }
        SDS_TYPE_8 => {
            let mut sh: *mut SdsHdr8 = s
                .offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut ::core::ffi::c_void as *mut SdsHdr8;
            (*sh).len = ((*sh).len as ::core::ffi::c_int + incr) as u8;
            len = (*sh).len as usize;
        }
        SDS_TYPE_16 => {
            let mut sh_0: *mut SdsHdr16 =
                s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                    as *mut ::core::ffi::c_void as *mut SdsHdr16;
            (*sh_0).len = ((*sh_0).len as ::core::ffi::c_int + incr) as u16;
            len = (*sh_0).len as usize;
        }
        SDS_TYPE_32 => {
            let mut sh_1: *mut SdsHdr32 =
                s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                    as *mut ::core::ffi::c_void as *mut SdsHdr32;
            (*sh_1).len = (*sh_1).len.wrapping_add(incr as u32);
            len = (*sh_1).len as usize;
        }
        SDS_TYPE_64 => {
            let mut sh_2: *mut SdsHdr64 =
                s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                    as *mut ::core::ffi::c_void as *mut SdsHdr64;
            (*sh_2).len = (*sh_2).len.wrapping_add(incr as u64);
            len = (*sh_2).len as usize;
        }
        _ => {
            len = 0 as usize;
        }
    }
    *s.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
}
pub unsafe extern "C" fn sdsgrowzero(mut s: SdsRaw, mut len: usize) -> SdsRaw {
    let mut curlen: usize = sdslen(s);
    if len <= curlen {
        return s;
    }
    s = sds_make_room_for(s, len.wrapping_sub(curlen));
    if s.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memset(
        s.offset(curlen as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        len.wrapping_sub(curlen).wrapping_add(1 as usize),
    );
    sdssetlen(s, len);
    return s;
}
pub unsafe extern "C" fn sdscatlen(
    mut s: SdsRaw,
    mut t: *const ::core::ffi::c_void,
    mut len: usize,
) -> SdsRaw {
    let mut curlen: usize = sdslen(s);
    s = sds_make_room_for(s, len);
    if s.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memcpy(
        s.offset(curlen as isize) as *mut ::core::ffi::c_void,
        t,
        len,
    );
    sdssetlen(s, curlen.wrapping_add(len));
    *s.offset(curlen.wrapping_add(len) as isize) = '\0' as i32 as ::core::ffi::c_char;
    return s;
}
pub unsafe extern "C" fn sdscat(mut s: SdsRaw, mut t: *const ::core::ffi::c_char) -> SdsRaw {
    return sdscatlen(s, t as *const ::core::ffi::c_void, strlen(t));
}
pub unsafe extern "C" fn sdscatsds(mut s: SdsRaw, t: SdsRaw) -> SdsRaw {
    return sdscatlen(s, t as *const ::core::ffi::c_void, sdslen(t));
}
pub unsafe extern "C" fn sdscpylen(
    mut s: SdsRaw,
    mut t: *const ::core::ffi::c_char,
    mut len: usize,
) -> SdsRaw {
    if sdsalloc(s) < len {
        s = sds_make_room_for(s, len.wrapping_sub(sdslen(s)));
        if s.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    memcpy(
        s as *mut ::core::ffi::c_void,
        t as *const ::core::ffi::c_void,
        len,
    );
    *s.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    sdssetlen(s, len);
    return s;
}
pub unsafe extern "C" fn sdscpy(mut s: SdsRaw, mut t: *const ::core::ffi::c_char) -> SdsRaw {
    return sdscpylen(s, t, strlen(t));
}
pub unsafe extern "C" fn sdsll2str(
    mut s: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aux: ::core::ffi::c_char = 0;
    let mut v: ::core::ffi::c_ulonglong = 0;
    let mut l: usize = 0;
    v = (if value < 0 as ::core::ffi::c_longlong {
        -value
    } else {
        value
    }) as ::core::ffi::c_ulonglong;
    p = s;
    loop {
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = ('0' as i32 as ::core::ffi::c_ulonglong)
            .wrapping_add(v.wrapping_rem(10 as ::core::ffi::c_ulonglong))
            as ::core::ffi::c_char;
        v = v.wrapping_div(10 as ::core::ffi::c_ulonglong);
        if !(v != 0) {
            break;
        }
    }
    if value < 0 as ::core::ffi::c_longlong {
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = '-' as i32 as ::core::ffi::c_char;
    }
    l = p.offset_from(s) as ::core::ffi::c_long as usize;
    *p = '\0' as i32 as ::core::ffi::c_char;
    p = p.offset(-1);
    while s < p {
        aux = *s;
        *s = *p;
        *p = aux;
        s = s.offset(1);
        p = p.offset(-1);
    }
    return l as ::core::ffi::c_int;
}
pub unsafe extern "C" fn sdsull2str(
    mut s: *mut ::core::ffi::c_char,
    mut v: ::core::ffi::c_ulonglong,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aux: ::core::ffi::c_char = 0;
    let mut l: usize = 0;
    p = s;
    loop {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = ('0' as i32 as ::core::ffi::c_ulonglong)
            .wrapping_add(v.wrapping_rem(10 as ::core::ffi::c_ulonglong))
            as ::core::ffi::c_char;
        v = v.wrapping_div(10 as ::core::ffi::c_ulonglong);
        if !(v != 0) {
            break;
        }
    }
    l = p.offset_from(s) as ::core::ffi::c_long as usize;
    *p = '\0' as i32 as ::core::ffi::c_char;
    p = p.offset(-1);
    while s < p {
        aux = *s;
        *s = *p;
        *p = aux;
        s = s.offset(1);
        p = p.offset(-1);
    }
    return l as ::core::ffi::c_int;
}
pub unsafe extern "C" fn sdsfromlonglong(mut value: ::core::ffi::c_longlong) -> SdsRaw {
    let mut buf: [::core::ffi::c_char; 21] = [0; 21];
    let mut len: ::core::ffi::c_int = sdsll2str(&raw mut buf as *mut ::core::ffi::c_char, value);
    return sdsnewlen(
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as usize,
    );
}
// ---------------------------------------------------------------------------
// Building an SdsRaw from typed pieces
// ---------------------------------------------------------------------------

/// One piece of a string being assembled by [`sdsbuild!`].
///
/// This replaces `sdscatprintf`/`sdscatfmt`. Every one of their 252 call sites
/// passed a *literal* format string, so the format never had to be interpreted
/// at run time: the pieces can simply be appended, and the compiler can check
/// that each argument matches the conversion the C code asked for -- which a
/// `printf` cannot.
///
/// Text is appended **as bytes**, deliberately. The `%s` arguments here are C
/// strings that came out of a font file: glyph names, table strings, PostScript
/// names. Routing them through Rust's `format!` would mean a `CStr` -> `str`
/// conversion, and a glyph name that is not valid UTF-8 would then come out
/// different (`to_str` fails; `to_string_lossy` substitutes U+FFFD) -- a change
/// no test payload would catch. Integers *are* formatted with `format!`, where
/// the output is ASCII digits and no such hazard exists.
pub trait SdsPart {
    /// Append this piece to `s`, returning the (possibly reallocated) string.
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw;
}

impl SdsPart for &[u8] {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        sdscatlen(s, self.as_ptr() as *const ::core::ffi::c_void, self.len())
    }
}

impl<const N: usize> SdsPart for &[u8; N] {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        (&self[..]).append_to(s)
    }
}

/// A C string (`%s`): the bytes up to the terminating NUL.
///
/// A null pointer appends `(null)`, which is what both glibc and Apple's libc
/// print for `%s`. The old code handed the pointer straight to `vsnprintf`, so
/// any call site that can pass null was already relying on that.
impl SdsPart for *const ::core::ffi::c_char {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        if self.is_null() {
            return b"(null)".append_to(s);
        }
        sdscatlen(s, self as *const ::core::ffi::c_void, strlen(self))
    }
}

/// `SdsRaw` is `*mut c_char`, so this covers both a plain C string and an SdsRaw
/// passed to `%s` -- which, like C, measures it with `strlen` and therefore
/// stops at an embedded NUL. Use [`Sds`] for `%S`.
impl SdsPart for *mut ::core::ffi::c_char {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        (self as *const ::core::ffi::c_char).append_to(s)
    }
}

/// A static C string (`%s`), for the label tables that reach the log and the
/// JSON output. Identical to the `*const c_char` impl above, minus the `strlen`
/// and the null check: a `CStr` carries its own length and cannot be null.
impl SdsPart for &::core::ffi::CStr {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        self.to_bytes().append_to(s)
    }
}

/// An SdsRaw appended by its stored length (`%S`), so unlike `%s` it keeps any
/// embedded NUL bytes.
pub struct Sds(pub SdsRaw);

impl SdsPart for Sds {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        sdscatsds(s, self.0)
    }
}

/// A single byte (`%c`).
///
/// C converts the argument to `unsigned char`, so this is one raw byte and
/// *not* a `char`: formatting a `char` would UTF-8 encode anything above 0x7f
/// into two bytes. `otl/read.rs` builds lookup names out of the four bytes of
/// an OpenType tag this way, and those names reach the JSON output.
pub struct Byte(pub u8);

impl SdsPart for Byte {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        sdscatlen(
            s,
            &self.0 as *const u8 as *const ::core::ffi::c_void,
            1 as usize,
        )
    }
}

/// `%04x`
pub struct Hex4(pub u32);
/// `%04X`
pub struct Hex4Upper(pub u32);
/// `%02x`
pub struct Hex2(pub u32);
/// `%02X`
pub struct Hex2Upper(pub u32);
/// `%05d`
pub struct Dec5(pub ::core::ffi::c_int);

/// Append an ASCII rendering of an integer.
///
/// Safe to route through `format!` because the result is digits: see the note
/// on [`SdsPart`] for why text may not be.
unsafe fn cat_ascii(s: SdsRaw, digits: &str) -> SdsRaw {
    digits.as_bytes().append_to(s)
}

impl SdsPart for ::core::ffi::c_int {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{self}"))
    }
}

impl SdsPart for ::core::ffi::c_uint {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{self}"))
    }
}

impl SdsPart for Dec5 {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{:05}", self.0))
    }
}

/// The `u32` casts at the call sites are not cosmetic: C's `%x` reads an
/// `unsigned int`, so a negative `int` argument prints as its 32-bit two's
/// complement -- eight digits, not four. `as u32` reproduces exactly that,
/// and widens a `u16` the same way C's default promotion does.
impl SdsPart for Hex4 {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{:04x}", self.0))
    }
}

impl SdsPart for Hex4Upper {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{:04X}", self.0))
    }
}

impl SdsPart for Hex2 {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{:02x}", self.0))
    }
}

impl SdsPart for Hex2Upper {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{:02X}", self.0))
    }
}

/// Append pieces to an SdsRaw, in order, and evaluate to the result.
///
/// ```ignore
/// sdscatprintf(sdsempty(), "lookup_%s_%02x_%d\0", name, kind, index)
/// ```
/// becomes
/// ```ignore
/// sdsbuild!(sdsempty(), b"lookup_", name, b"_", Hex2(kind as u32), b"_", index)
/// ```
///
/// Each piece is appended through [`SdsPart`], so its type decides how it is
/// rendered. Returns null if any reallocation fails, exactly as the `sdscat*`
/// functions do.
#[macro_export]
macro_rules! sdsbuild {
    ($base:expr $(, $part:expr)* $(,)?) => {{
        let mut __sds: $crate::vendor::sds::SdsRaw = $base;
        $(
            __sds = $crate::vendor::sds::SdsPart::append_to($part, __sds);
        )*
        __sds
    }};
}

pub unsafe extern "C" fn sdstrim(mut s: SdsRaw, mut cset: *const ::core::ffi::c_char) -> SdsRaw {
    let mut start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: usize = 0;
    start = s as *mut ::core::ffi::c_char;
    sp = start;
    end = s
        .offset(sdslen(s) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_char;
    ep = end;
    while sp <= end && !strchr(cset, *sp as ::core::ffi::c_int).is_null() {
        sp = sp.offset(1);
    }
    while ep > sp && !strchr(cset, *ep as ::core::ffi::c_int).is_null() {
        ep = ep.offset(-1);
    }
    len = (if sp > ep {
        0 as ::core::ffi::c_long
    } else {
        ep.offset_from(sp) as ::core::ffi::c_long + 1 as ::core::ffi::c_long
    }) as usize;
    if s != sp {
        memmove(
            s as *mut ::core::ffi::c_void,
            sp as *const ::core::ffi::c_void,
            len,
        );
    }
    *s.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    sdssetlen(s, len);
    return s;
}
pub unsafe extern "C" fn sdsrange(
    mut s: SdsRaw,
    mut start: ::core::ffi::c_int,
    mut end: ::core::ffi::c_int,
) {
    let mut newlen: usize = 0;
    let mut len: usize = sdslen(s);
    if len == 0 as usize {
        return;
    }
    if start < 0 as ::core::ffi::c_int {
        start = len.wrapping_add(start as usize) as ::core::ffi::c_int;
        if start < 0 as ::core::ffi::c_int {
            start = 0 as ::core::ffi::c_int;
        }
    }
    if end < 0 as ::core::ffi::c_int {
        end = len.wrapping_add(end as usize) as ::core::ffi::c_int;
        if end < 0 as ::core::ffi::c_int {
            end = 0 as ::core::ffi::c_int;
        }
    }
    newlen = (if start > end {
        0 as ::core::ffi::c_int
    } else {
        end - start + 1 as ::core::ffi::c_int
    }) as usize;
    if newlen != 0 as usize {
        if start >= len as ::core::ffi::c_int {
            newlen = 0 as usize;
        } else if end >= len as ::core::ffi::c_int {
            end = len.wrapping_sub(1 as usize) as ::core::ffi::c_int;
            newlen = (if start > end {
                0 as ::core::ffi::c_int
            } else {
                end - start + 1 as ::core::ffi::c_int
            }) as usize;
        }
    } else {
        start = 0 as ::core::ffi::c_int;
    }
    if start != 0 && newlen != 0 {
        memmove(
            s as *mut ::core::ffi::c_void,
            s.offset(start as isize) as *const ::core::ffi::c_void,
            newlen,
        );
    }
    *s.offset(newlen as isize) = 0 as ::core::ffi::c_char;
    sdssetlen(s, newlen);
}
pub unsafe extern "C" fn sdstolower(mut s: SdsRaw) {
    let mut len: ::core::ffi::c_int = sdslen(s) as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < len {
        *s.offset(j as isize) = (c_tolower(*s.offset(j as isize) as ::core::ffi::c_int)) as ::core::ffi::c_char;
        j += 1;
    }
}
pub unsafe extern "C" fn sdstoupper(mut s: SdsRaw) {
    let mut len: ::core::ffi::c_int = sdslen(s) as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < len {
        *s.offset(j as isize) = (c_toupper(*s.offset(j as isize) as ::core::ffi::c_int)) as ::core::ffi::c_char;
        j += 1;
    }
}
pub unsafe extern "C" fn sdscmp(s1: SdsRaw, s2: SdsRaw) -> ::core::ffi::c_int {
    let mut l1: usize = 0;
    let mut l2: usize = 0;
    let mut minlen: usize = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    l1 = sdslen(s1);
    l2 = sdslen(s2);
    minlen = if l1 < l2 { l1 } else { l2 };
    cmp = memcmp(
        s1 as *const ::core::ffi::c_void,
        s2 as *const ::core::ffi::c_void,
        minlen,
    );
    if cmp == 0 as ::core::ffi::c_int {
        return l1.wrapping_sub(l2) as ::core::ffi::c_int;
    }
    return cmp;
}
pub unsafe extern "C" fn sdssplitlen(
    mut s: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut sep: *const ::core::ffi::c_char,
    mut seplen: ::core::ffi::c_int,
    mut count: *mut ::core::ffi::c_int,
) -> *mut SdsRaw {
    let mut current_block: u64;
    let mut elements: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut slots: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut start: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut tokens: *mut SdsRaw = ::core::ptr::null_mut::<SdsRaw>();
    if seplen < 1 as ::core::ffi::c_int || len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<SdsRaw>();
    }
    tokens =
        malloc((::core::mem::size_of::<SdsRaw>() as usize).wrapping_mul(slots as usize)) as *mut SdsRaw;
    if tokens.is_null() {
        return ::core::ptr::null_mut::<SdsRaw>();
    }
    if len == 0 as ::core::ffi::c_int {
        *count = 0 as ::core::ffi::c_int;
        return tokens;
    }
    j = 0 as ::core::ffi::c_int;
    loop {
        if !(j < len - (seplen - 1 as ::core::ffi::c_int)) {
            current_block = 15904375183555213903;
            break;
        }
        if slots < elements + 2 as ::core::ffi::c_int {
            let mut newtokens: *mut SdsRaw = ::core::ptr::null_mut::<SdsRaw>();
            slots *= 2 as ::core::ffi::c_int;
            newtokens = realloc(
                tokens as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<SdsRaw>() as usize).wrapping_mul(slots as usize),
            ) as *mut SdsRaw;
            if newtokens.is_null() {
                current_block = 2896259319996730917;
                break;
            }
            tokens = newtokens;
        }
        if seplen == 1 as ::core::ffi::c_int
            && *s.offset(j as isize) as ::core::ffi::c_int
                == *sep.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            || memcmp(
                s.offset(j as isize) as *const ::core::ffi::c_void,
                sep as *const ::core::ffi::c_void,
                seplen as usize,
            ) == 0 as ::core::ffi::c_int
        {
            let fresh9 = &raw mut *tokens.offset(elements as isize);
            *fresh9 = sdsnewlen(
                s.offset(start as isize) as *const ::core::ffi::c_void,
                (j - start) as usize,
            );
            if (*tokens.offset(elements as isize)).is_null() {
                current_block = 2896259319996730917;
                break;
            }
            elements += 1;
            start = j + seplen;
            j = j + seplen - 1 as ::core::ffi::c_int;
        }
        j += 1;
    }
    match current_block {
        15904375183555213903 => {
            let fresh10 = &raw mut *tokens.offset(elements as isize);
            *fresh10 = sdsnewlen(
                s.offset(start as isize) as *const ::core::ffi::c_void,
                (len - start) as usize,
            );
            if !(*tokens.offset(elements as isize)).is_null() {
                elements += 1;
                *count = elements;
                return tokens;
            }
        }
        _ => {}
    }
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < elements {
        sdsfree(*tokens.offset(i as isize));
        i += 1;
    }
    free(tokens as *mut ::core::ffi::c_void);
    *count = 0 as ::core::ffi::c_int;
    return ::core::ptr::null_mut::<SdsRaw>();
}
pub unsafe extern "C" fn sdsfreesplitres(mut tokens: *mut SdsRaw, mut count: ::core::ffi::c_int) {
    if tokens.is_null() {
        return;
    }
    loop {
        let fresh11 = count;
        count = count - 1;
        if !(fresh11 != 0) {
            break;
        }
        sdsfree(*tokens.offset(count as isize));
    }
    free(tokens as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn sdscatrepr(
    mut s: SdsRaw,
    mut p: *const ::core::ffi::c_char,
    mut len: usize,
) -> SdsRaw {
    s = sdscatlen(
        s,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as usize,
    );
    loop {
        let fresh12 = len;
        len = len.wrapping_sub(1);
        if !(fresh12 != 0) {
            break;
        }
        match *p as ::core::ffi::c_int {
            92 | 34 => {
                s = crate::sdsbuild!(s, b"\\", Byte((*p as ::core::ffi::c_int) as u8));
            }
            10 => {
                s = sdscatlen(
                    s,
                    b"\\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as usize,
                );
            }
            13 => {
                s = sdscatlen(
                    s,
                    b"\\r\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as usize,
                );
            }
            9 => {
                s = sdscatlen(
                    s,
                    b"\\t\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as usize,
                );
            }
            7 => {
                s = sdscatlen(
                    s,
                    b"\\a\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as usize,
                );
            }
            8 => {
                s = sdscatlen(
                    s,
                    b"\\b\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as usize,
                );
            }
            _ => {
                if c_isprint(*p as ::core::ffi::c_int)
                {
                    s = crate::sdsbuild!(s, Byte((*p as ::core::ffi::c_int) as u8));
                } else {
                    s = crate::sdsbuild!(
                        s,
                        b"\\x",
                        Hex2((*p as ::core::ffi::c_uchar as ::core::ffi::c_int) as u32),
                    );
                }
            }
        }
        p = p.offset(1);
    }
    return sdscatlen(
        s,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as usize,
    );
}
pub unsafe extern "C" fn is_hex_digit(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32
        || c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'f' as i32
        || c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32)
        as ::core::ffi::c_int;
}
pub unsafe extern "C" fn hex_digit_to_int(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    match c as ::core::ffi::c_int {
        48 => return 0 as ::core::ffi::c_int,
        49 => return 1 as ::core::ffi::c_int,
        50 => return 2 as ::core::ffi::c_int,
        51 => return 3 as ::core::ffi::c_int,
        52 => return 4 as ::core::ffi::c_int,
        53 => return 5 as ::core::ffi::c_int,
        54 => return 6 as ::core::ffi::c_int,
        55 => return 7 as ::core::ffi::c_int,
        56 => return 8 as ::core::ffi::c_int,
        57 => return 9 as ::core::ffi::c_int,
        97 | 65 => return 10 as ::core::ffi::c_int,
        98 | 66 => return 11 as ::core::ffi::c_int,
        99 | 67 => return 12 as ::core::ffi::c_int,
        100 | 68 => return 13 as ::core::ffi::c_int,
        101 | 69 => return 14 as ::core::ffi::c_int,
        102 | 70 => return 15 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
pub unsafe extern "C" fn sdssplitargs(
    mut line: *const ::core::ffi::c_char,
    mut argc: *mut ::core::ffi::c_int,
) -> *mut SdsRaw {
    let mut p: *const ::core::ffi::c_char = line;
    let mut current: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut vector: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    *argc = 0 as ::core::ffi::c_int;
    's_13: loop {
        while *p as ::core::ffi::c_int != 0
            && c_isspace(*p as ::core::ffi::c_int)
        {
            p = p.offset(1);
        }
        if *p != 0 {
            let mut inq: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut insq: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if current.is_null() {
                current = sdsempty() as *mut ::core::ffi::c_char;
            }
            while done == 0 {
                if inq != 0 {
                    if *p as ::core::ffi::c_int == '\\' as i32
                        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'x' as i32
                        && is_hex_digit(*p.offset(2 as ::core::ffi::c_int as isize)) != 0
                        && is_hex_digit(*p.offset(3 as ::core::ffi::c_int as isize)) != 0
                    {
                        let mut byte: ::core::ffi::c_uchar = 0;
                        byte = (hex_digit_to_int(*p.offset(2 as ::core::ffi::c_int as isize))
                            * 16 as ::core::ffi::c_int
                            + hex_digit_to_int(*p.offset(3 as ::core::ffi::c_int as isize)))
                            as ::core::ffi::c_uchar;
                        current = sdscatlen(
                            current as SdsRaw,
                            &raw mut byte as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                            1 as usize,
                        ) as *mut ::core::ffi::c_char;
                        p = p.offset(3 as ::core::ffi::c_int as isize);
                    } else if *p as ::core::ffi::c_int == '\\' as i32
                        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
                    {
                        let mut c: ::core::ffi::c_char = 0;
                        p = p.offset(1);
                        match *p as ::core::ffi::c_int {
                            110 => {
                                c = '\n' as i32 as ::core::ffi::c_char;
                            }
                            114 => {
                                c = '\r' as i32 as ::core::ffi::c_char;
                            }
                            116 => {
                                c = '\t' as i32 as ::core::ffi::c_char;
                            }
                            98 => {
                                c = '\u{8}' as i32 as ::core::ffi::c_char;
                            }
                            97 => {
                                c = '\u{7}' as i32 as ::core::ffi::c_char;
                            }
                            _ => {
                                c = *p;
                            }
                        }
                        current = sdscatlen(
                            current as SdsRaw,
                            &raw mut c as *const ::core::ffi::c_void,
                            1 as usize,
                        ) as *mut ::core::ffi::c_char;
                    } else if *p as ::core::ffi::c_int == '"' as i32 {
                        if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
                            && !c_isspace(*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        {
                            break 's_13;
                        }
                        done = 1 as ::core::ffi::c_int;
                    } else if *p == 0 {
                        break 's_13;
                    } else {
                        current =
                            sdscatlen(current as SdsRaw, p as *const ::core::ffi::c_void, 1 as usize)
                                as *mut ::core::ffi::c_char;
                    }
                } else if insq != 0 {
                    if *p as ::core::ffi::c_int == '\\' as i32
                        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == '\'' as i32
                    {
                        p = p.offset(1);
                        current = sdscatlen(
                            current as SdsRaw,
                            b"'\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            1 as usize,
                        ) as *mut ::core::ffi::c_char;
                    } else if *p as ::core::ffi::c_int == '\'' as i32 {
                        if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
                            && !c_isspace(*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        {
                            break 's_13;
                        }
                        done = 1 as ::core::ffi::c_int;
                    } else {
                        if *p == 0 {
                            break 's_13;
                        }
                        current =
                            sdscatlen(current as SdsRaw, p as *const ::core::ffi::c_void, 1 as usize)
                                as *mut ::core::ffi::c_char;
                    }
                } else {
                    match *p as ::core::ffi::c_int {
                        32 | 10 | 13 | 9 | 0 => {
                            done = 1 as ::core::ffi::c_int;
                        }
                        34 => {
                            inq = 1 as ::core::ffi::c_int;
                        }
                        39 => {
                            insq = 1 as ::core::ffi::c_int;
                        }
                        _ => {
                            current = sdscatlen(
                                current as SdsRaw,
                                p as *const ::core::ffi::c_void,
                                1 as usize,
                            ) as *mut ::core::ffi::c_char;
                        }
                    }
                }
                if *p != 0 {
                    p = p.offset(1);
                }
            }
            vector = realloc(
                vector as *mut ::core::ffi::c_void,
                ((*argc + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize),
            ) as *mut *mut ::core::ffi::c_char;
            let fresh13 = &raw mut *vector.offset(*argc as isize);
            *fresh13 = current;
            *argc += 1;
            current = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            if vector.is_null() {
                vector = malloc(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                    as *mut *mut ::core::ffi::c_char;
            }
            return vector as *mut SdsRaw;
        }
    }
    loop {
        let fresh14 = *argc;
        *argc = *argc - 1;
        if !(fresh14 != 0) {
            break;
        }
        sdsfree(*vector.offset(*argc as isize) as SdsRaw);
    }
    free(vector as *mut ::core::ffi::c_void);
    if !current.is_null() {
        sdsfree(current as SdsRaw);
    }
    *argc = 0 as ::core::ffi::c_int;
    return ::core::ptr::null_mut::<SdsRaw>();
}
pub unsafe extern "C" fn sdsmapchars(
    mut s: SdsRaw,
    mut from: *const ::core::ffi::c_char,
    mut to: *const ::core::ffi::c_char,
    mut setlen: usize,
) -> SdsRaw {
    let mut j: usize = 0;
    let mut i: usize = 0;
    let mut l: usize = sdslen(s);
    j = 0 as usize;
    while j < l {
        i = 0 as usize;
        while i < setlen {
            if *s.offset(j as isize) as ::core::ffi::c_int
                == *from.offset(i as isize) as ::core::ffi::c_int
            {
                *s.offset(j as isize) = *to.offset(i as isize);
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        j = j.wrapping_add(1);
    }
    return s;
}
pub unsafe extern "C" fn sdsjoin(
    mut argv: *mut *mut ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut sep: *mut ::core::ffi::c_char,
) -> SdsRaw {
    let mut join: SdsRaw = sdsempty();
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < argc {
        join = sdscat(join, *argv.offset(j as isize));
        if j != argc - 1 as ::core::ffi::c_int {
            join = sdscat(join, sep);
        }
        j += 1;
    }
    return join;
}
pub unsafe extern "C" fn sdsjoinsds(
    mut argv: *mut SdsRaw,
    mut argc: ::core::ffi::c_int,
    mut sep: *const ::core::ffi::c_char,
    mut seplen: usize,
) -> SdsRaw {
    let mut join: SdsRaw = sdsempty();
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < argc {
        join = sdscatsds(join, *argv.offset(j as isize));
        if j != argc - 1 as ::core::ffi::c_int {
            join = sdscatlen(join, sep as *const ::core::ffi::c_void, seplen);
        }
        j += 1;
    }
    return join;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What C's `printf` makes of the same conversion and argument.
    ///
    /// The helpers are checked against the C library rather than against
    /// hand-written expectations: the whole point of them is to reproduce
    /// `sdscatprintf` byte for byte, and only libc can settle what that was.
    macro_rules! assert_matches_printf {
        ($fmt:expr, $c_arg:expr, $built:expr) => {{
            let mut expect = [0 as ::core::ffi::c_char; 64];
            let n = libc::snprintf(
                expect.as_mut_ptr(),
                expect.len(),
                concat!($fmt, "\0").as_ptr() as *const ::core::ffi::c_char,
                $c_arg,
            );
            assert!(n >= 0 && (n as usize) < expect.len(), "snprintf overflowed");
            let expect =
                ::core::slice::from_raw_parts(expect.as_ptr() as *const u8, n as usize).to_vec();
            let got = $built;
            let got_bytes = ::core::slice::from_raw_parts(got as *const u8, sdslen(got)).to_vec();
            sdsfree(got);
            assert_eq!(
                String::from_utf8_lossy(&got_bytes),
                String::from_utf8_lossy(&expect),
                "conversion {} disagrees with libc",
                $fmt
            );
        }};
    }

    #[test]
    fn decimal_matches_printf() {
        unsafe {
            for v in [0, 1, -1, 42, -42, i32::MAX, i32::MIN] {
                assert_matches_printf!("%d", v, sdsbuild!(sdsempty(), v));
                assert_matches_printf!("%05d", v, sdsbuild!(sdsempty(), Dec5(v)));
            }
            for v in [0u32, 1, 65535, u32::MAX] {
                assert_matches_printf!("%u", v, sdsbuild!(sdsempty(), v));
            }
        }
    }

    // A negative `int` reaches `%x` as an `unsigned int`, so it prints eight
    // digits and not four. Casting to `u16` at the call site -- the obvious
    // reading of "%04x" -- would silently drop the top half.
    #[test]
    fn hex_matches_printf_including_negatives() {
        unsafe {
            for v in [0i32, 1, 0x0a, 0xabcd, 0xfffff, -1, -32768] {
                assert_matches_printf!("%04x", v, sdsbuild!(sdsempty(), Hex4(v as u32)));
                assert_matches_printf!("%04X", v, sdsbuild!(sdsempty(), Hex4Upper(v as u32)));
                assert_matches_printf!("%02x", v, sdsbuild!(sdsempty(), Hex2(v as u32)));
                assert_matches_printf!("%02X", v, sdsbuild!(sdsempty(), Hex2Upper(v as u32)));
            }
        }
    }

    // `%c` is a byte, not a character: 0xe9 is one byte for C, and would be the
    // two bytes of U+00E9 if it went through Rust's `char` formatting.
    #[test]
    fn byte_is_one_byte_not_a_char() {
        unsafe {
            for v in [b'A' as i32, 0, 0x7f, 0x80, 0xe9, 0xff] {
                assert_matches_printf!("%c", v, sdsbuild!(sdsempty(), Byte(v as u8)));
            }
            let got = sdsbuild!(sdsempty(), Byte(0xe9));
            assert_eq!(sdslen(got), 1);
            sdsfree(got);
            assert_eq!('\u{e9}'.to_string().len(), 2); // ...which this is not
        }
    }

    // The reason these helpers exist instead of `format!`: a glyph name that is
    // not valid UTF-8 has to survive unchanged. `to_string_lossy` would replace
    // the 0xe9 with U+FFFD and the font would come out with a different name.
    #[test]
    fn c_string_is_copied_as_bytes_even_when_not_utf8() {
        unsafe {
            let name = b"caf\xe9\0";
            let got = sdsbuild!(sdsempty(), name.as_ptr() as *const ::core::ffi::c_char);
            let bytes = ::core::slice::from_raw_parts(got as *const u8, sdslen(got));
            assert_eq!(bytes, b"caf\xe9");
            sdsfree(got);
        }
    }

    #[test]
    fn null_c_string_prints_like_libc() {
        unsafe {
            assert_matches_printf!(
                "%s",
                ::core::ptr::null::<::core::ffi::c_char>(),
                sdsbuild!(sdsempty(), ::core::ptr::null::<::core::ffi::c_char>())
            );
        }
    }

    // `%s` stops at the NUL and `%S` does not -- the distinction sdscatfmt drew
    // by calling `strlen` for one and `sdslen` for the other.
    #[test]
    fn sds_part_keeps_embedded_nul_but_c_string_does_not() {
        unsafe {
            let s = sdsnewlen(b"ab\0cd".as_ptr() as *const ::core::ffi::c_void, 5 as usize);
            let by_len = sdsbuild!(sdsempty(), Sds(s));
            assert_eq!(sdslen(by_len), 5);
            let by_nul = sdsbuild!(sdsempty(), s);
            assert_eq!(sdslen(by_nul), 2);
            sdsfree(by_len);
            sdsfree(by_nul);
            sdsfree(s);
        }
    }

    #[test]
    fn pieces_are_appended_in_order() {
        unsafe {
            let got = sdsbuild!(
                sdsempty(),
                b"lookup_",
                b"ccmp\0".as_ptr() as *const ::core::ffi::c_char,
                b"_",
                Hex2(0x1f),
                b"_",
                7 as ::core::ffi::c_int,
            );
            let bytes = ::core::slice::from_raw_parts(got as *const u8, sdslen(got));
            assert_eq!(bytes, b"lookup_ccmp_1f_7");
            sdsfree(got);
        }
    }
}
