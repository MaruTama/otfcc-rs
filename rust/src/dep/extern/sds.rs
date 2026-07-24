extern "C" {
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    #[cfg(not(target_os = "macos"))]
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    #[cfg(not(target_os = "macos"))]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[cfg(not(target_os = "macos"))]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
#[cfg(target_os = "macos")]
use crate::src::lib::support::ctype_compat::{
    __ctype_b_loc, __ctype_tolower_loc, __ctype_toupper_loc,
};
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut ::core::ffi::c_void,
    pub __gr_top: *mut ::core::ffi::c_void,
    pub __vr_top: *mut ::core::ffi::c_void,
    pub __gr_offs: ::core::ffi::c_int,
    pub __vr_offs: ::core::ffi::c_int,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type va_list = __gnuc_va_list;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr5 {
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
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
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as size_t,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as size_t;
        }
        _ => {}
    }
    return 0 as size_t;
}
#[inline]
unsafe extern "C" fn sdsavail(s: sds) -> size_t {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return 0 as size_t,
        SDS_TYPE_8 => {
            let mut sh: *mut sdshdr8 = s
                .offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut ::core::ffi::c_void as *mut sdshdr8;
            return ((*sh).alloc as ::core::ffi::c_int - (*sh).len as ::core::ffi::c_int) as size_t;
        }
        SDS_TYPE_16 => {
            let mut sh_0: *mut sdshdr16 =
                s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                    as *mut ::core::ffi::c_void as *mut sdshdr16;
            return ((*sh_0).alloc as ::core::ffi::c_int - (*sh_0).len as ::core::ffi::c_int)
                as size_t;
        }
        SDS_TYPE_32 => {
            let mut sh_1: *mut sdshdr32 =
                s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                    as *mut ::core::ffi::c_void as *mut sdshdr32;
            return (*sh_1).alloc.wrapping_sub((*sh_1).len) as size_t;
        }
        SDS_TYPE_64 => {
            let mut sh_2: *mut sdshdr64 =
                s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                    as *mut ::core::ffi::c_void as *mut sdshdr64;
            return ((*sh_2).alloc as size_t).wrapping_sub((*sh_2).len as size_t);
        }
        _ => {}
    }
    return 0 as size_t;
}
#[inline]
unsafe extern "C" fn sdssetlen(mut s: sds, mut newlen: size_t) {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => {
            let mut fp: *mut ::core::ffi::c_uchar =
                (s as *mut ::core::ffi::c_uchar).offset(-(1 as ::core::ffi::c_int as isize));
            *fp = (SDS_TYPE_5 as size_t | newlen << SDS_TYPE_BITS) as ::core::ffi::c_uchar;
        }
        SDS_TYPE_8 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize)) as *mut sdshdr8))
                .len = newlen as uint8_t;
        }
        SDS_TYPE_16 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len = newlen as uint16_t;
        }
        SDS_TYPE_32 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len = newlen as uint32_t;
        }
        SDS_TYPE_64 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len = newlen as uint64_t;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn sdsinclen(mut s: sds, mut inc: size_t) {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => {
            let mut fp: *mut ::core::ffi::c_uchar =
                (s as *mut ::core::ffi::c_uchar).offset(-(1 as ::core::ffi::c_int as isize));
            let mut newlen: ::core::ffi::c_uchar =
                ((flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as size_t).wrapping_add(inc)
                    as ::core::ffi::c_uchar;
            *fp = (SDS_TYPE_5 | (newlen as ::core::ffi::c_int) << SDS_TYPE_BITS)
                as ::core::ffi::c_uchar;
        }
        SDS_TYPE_8 => {
            let fresh0 = &raw mut
                (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                    as *mut sdshdr8))
                    .len;
            *fresh0 =
                (*fresh0 as ::core::ffi::c_int + inc as uint8_t as ::core::ffi::c_int) as uint8_t;
        }
        SDS_TYPE_16 => {
            let fresh1 = &raw mut (*(s
                .offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len;
            *fresh1 =
                (*fresh1 as ::core::ffi::c_int + inc as uint16_t as ::core::ffi::c_int) as uint16_t;
        }
        SDS_TYPE_32 => {
            let fresh2 = &raw mut (*(s
                .offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len;
            *fresh2 = (*fresh2).wrapping_add(inc as uint32_t);
        }
        SDS_TYPE_64 => {
            let fresh3 = &raw mut (*(s
                .offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len;
            *fresh3 = (*fresh3 as ::core::ffi::c_ulong).wrapping_add(inc as ::core::ffi::c_ulong)
                as uint64_t as uint64_t;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn sdsalloc(s: sds) -> size_t {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as size_t,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .alloc as size_t;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .alloc as size_t;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .alloc as size_t;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .alloc as size_t;
        }
        _ => {}
    }
    return 0 as size_t;
}
#[inline]
unsafe extern "C" fn sdssetalloc(mut s: sds, mut newlen: size_t) {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_8 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize)) as *mut sdshdr8))
                .alloc = newlen as uint8_t;
        }
        SDS_TYPE_16 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .alloc = newlen as uint16_t;
        }
        SDS_TYPE_32 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .alloc = newlen as uint32_t;
        }
        SDS_TYPE_64 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .alloc = newlen as uint64_t;
        }
        SDS_TYPE_5 | _ => {}
    };
}
#[inline]
unsafe extern "C" fn sdsHdrSize(mut type_0: ::core::ffi::c_char) -> ::core::ffi::c_int {
    match type_0 as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return ::core::mem::size_of::<sdshdr5>() as ::core::ffi::c_int,
        SDS_TYPE_8 => return ::core::mem::size_of::<sdshdr8>() as ::core::ffi::c_int,
        SDS_TYPE_16 => return ::core::mem::size_of::<sdshdr16>() as ::core::ffi::c_int,
        SDS_TYPE_32 => return ::core::mem::size_of::<sdshdr32>() as ::core::ffi::c_int,
        SDS_TYPE_64 => return ::core::mem::size_of::<sdshdr64>() as ::core::ffi::c_int,
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn sdsReqType(mut string_size: size_t) -> ::core::ffi::c_char {
    if string_size < 32 as size_t {
        return SDS_TYPE_5 as ::core::ffi::c_char;
    }
    if string_size < 0xff as size_t {
        return SDS_TYPE_8 as ::core::ffi::c_char;
    }
    if string_size < 0xffff as size_t {
        return SDS_TYPE_16 as ::core::ffi::c_char;
    }
    if string_size < 0xffffffff as ::core::ffi::c_uint as size_t {
        return SDS_TYPE_32 as ::core::ffi::c_char;
    }
    return SDS_TYPE_64 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sdsnewlen(
    mut init: *const ::core::ffi::c_void,
    mut initlen: size_t,
) -> sds {
    let mut sh: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut s: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut type_0: ::core::ffi::c_char = sdsReqType(initlen);
    if type_0 as ::core::ffi::c_int == SDS_TYPE_5 && initlen == 0 as size_t {
        type_0 = SDS_TYPE_8 as ::core::ffi::c_char;
    }
    let mut hdrlen: ::core::ffi::c_int = sdsHdrSize(type_0);
    let mut fp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    sh = malloc(
        (hdrlen as size_t)
            .wrapping_add(initlen)
            .wrapping_add(1 as size_t),
    );
    if init.is_null() {
        memset(
            sh,
            0 as ::core::ffi::c_int,
            (hdrlen as size_t)
                .wrapping_add(initlen)
                .wrapping_add(1 as size_t),
        );
    }
    if sh.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    s = (sh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as sds;
    fp = (s as *mut ::core::ffi::c_uchar).offset(-(1 as ::core::ffi::c_int as isize));
    match type_0 as ::core::ffi::c_int {
        SDS_TYPE_5 => {
            *fp = (type_0 as size_t | initlen << SDS_TYPE_BITS) as ::core::ffi::c_uchar;
        }
        SDS_TYPE_8 => {
            let mut sh_0: *mut sdshdr8 =
                s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                    as *mut ::core::ffi::c_void as *mut sdshdr8;
            (*sh_0).len = initlen as uint8_t;
            (*sh_0).alloc = initlen as uint8_t;
            *fp = type_0 as ::core::ffi::c_uchar;
        }
        SDS_TYPE_16 => {
            let mut sh_1: *mut sdshdr16 =
                s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                    as *mut ::core::ffi::c_void as *mut sdshdr16;
            (*sh_1).len = initlen as uint16_t;
            (*sh_1).alloc = initlen as uint16_t;
            *fp = type_0 as ::core::ffi::c_uchar;
        }
        SDS_TYPE_32 => {
            let mut sh_2: *mut sdshdr32 =
                s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                    as *mut ::core::ffi::c_void as *mut sdshdr32;
            (*sh_2).len = initlen as uint32_t;
            (*sh_2).alloc = initlen as uint32_t;
            *fp = type_0 as ::core::ffi::c_uchar;
        }
        SDS_TYPE_64 => {
            let mut sh_3: *mut sdshdr64 =
                s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                    as *mut ::core::ffi::c_void as *mut sdshdr64;
            (*sh_3).len = initlen as uint64_t;
            (*sh_3).alloc = initlen as uint64_t;
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
#[no_mangle]
pub unsafe extern "C" fn sdsempty() -> sds {
    return sdsnewlen(
        b"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        0 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sdsnew(mut init: *const ::core::ffi::c_char) -> sds {
    let mut initlen: size_t = if init.is_null() {
        0 as size_t
    } else {
        strlen(init)
    };
    return sdsnewlen(init as *const ::core::ffi::c_void, initlen);
}
#[no_mangle]
pub unsafe extern "C" fn sdsdup(s: sds) -> sds {
    return sdsnewlen(s as *const ::core::ffi::c_void, sdslen(s));
}
#[no_mangle]
pub unsafe extern "C" fn sdsfree(mut s: sds) {
    if s.is_null() {
        return;
    }
    free(
        s.offset(-(sdsHdrSize(*s.offset(-(1 as ::core::ffi::c_int) as isize)) as isize))
            as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sdsupdatelen(mut s: sds) {
    let mut reallen: ::core::ffi::c_int =
        strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_int;
    sdssetlen(s, reallen as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn sdsclear(mut s: sds) {
    sdssetlen(s, 0 as size_t);
    *s.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sdsMakeRoomFor(mut s: sds, mut addlen: size_t) -> sds {
    let mut sh: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut newsh: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut avail: size_t = sdsavail(s);
    let mut len: size_t = 0;
    let mut newlen: size_t = 0;
    let mut type_0: ::core::ffi::c_char = 0;
    let mut oldtype: ::core::ffi::c_char = (*s.offset(-(1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        & SDS_TYPE_MASK) as ::core::ffi::c_char;
    let mut hdrlen: ::core::ffi::c_int = 0;
    if avail >= addlen {
        return s;
    }
    len = sdslen(s);
    sh = s.offset(-(sdsHdrSize(oldtype) as isize)) as *mut ::core::ffi::c_void;
    newlen = len.wrapping_add(addlen);
    if newlen < SDS_MAX_PREALLOC as size_t {
        newlen = newlen.wrapping_mul(2 as size_t);
    } else {
        newlen = newlen.wrapping_add(SDS_MAX_PREALLOC as size_t);
    }
    type_0 = sdsReqType(newlen);
    if type_0 as ::core::ffi::c_int == SDS_TYPE_5 {
        type_0 = SDS_TYPE_8 as ::core::ffi::c_char;
    }
    hdrlen = sdsHdrSize(type_0);
    if oldtype as ::core::ffi::c_int == type_0 as ::core::ffi::c_int {
        newsh = realloc(
            sh,
            (hdrlen as size_t)
                .wrapping_add(newlen)
                .wrapping_add(1 as size_t),
        );
        if newsh.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        s = (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as sds;
    } else {
        newsh = malloc(
            (hdrlen as size_t)
                .wrapping_add(newlen)
                .wrapping_add(1 as size_t),
        );
        if newsh.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        memcpy(
            (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            len.wrapping_add(1 as size_t),
        );
        free(sh);
        s = (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as sds;
        *s.offset(-(1 as ::core::ffi::c_int) as isize) = type_0;
        sdssetlen(s, len);
    }
    sdssetalloc(s, newlen);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdsRemoveFreeSpace(mut s: sds) -> sds {
    let mut sh: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut newsh: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut type_0: ::core::ffi::c_char = 0;
    let mut oldtype: ::core::ffi::c_char = (*s.offset(-(1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        & SDS_TYPE_MASK) as ::core::ffi::c_char;
    let mut hdrlen: ::core::ffi::c_int = 0;
    let mut len: size_t = sdslen(s);
    sh = s.offset(-(sdsHdrSize(oldtype) as isize)) as *mut ::core::ffi::c_void;
    type_0 = sdsReqType(len);
    hdrlen = sdsHdrSize(type_0);
    if oldtype as ::core::ffi::c_int == type_0 as ::core::ffi::c_int {
        newsh = realloc(
            sh,
            (hdrlen as size_t)
                .wrapping_add(len)
                .wrapping_add(1 as size_t),
        );
        if newsh.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        s = (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as sds;
    } else {
        newsh = malloc(
            (hdrlen as size_t)
                .wrapping_add(len)
                .wrapping_add(1 as size_t),
        );
        if newsh.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        memcpy(
            (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            len.wrapping_add(1 as size_t),
        );
        free(sh);
        s = (newsh as *mut ::core::ffi::c_char).offset(hdrlen as isize) as sds;
        *s.offset(-(1 as ::core::ffi::c_int) as isize) = type_0;
        sdssetlen(s, len);
    }
    sdssetalloc(s, len);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdsAllocSize(mut s: sds) -> size_t {
    let mut alloc: size_t = sdsalloc(s);
    return (sdsHdrSize(*s.offset(-(1 as ::core::ffi::c_int) as isize)) as size_t)
        .wrapping_add(alloc)
        .wrapping_add(1 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn sdsAllocPtr(mut s: sds) -> *mut ::core::ffi::c_void {
    return s.offset(-(sdsHdrSize(*s.offset(-(1 as ::core::ffi::c_int) as isize)) as isize))
        as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn sdsIncrLen(mut s: sds, mut incr: ::core::ffi::c_int) {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    let mut len: size_t = 0;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => {
            let mut fp: *mut ::core::ffi::c_uchar =
                (s as *mut ::core::ffi::c_uchar).offset(-(1 as ::core::ffi::c_int as isize));
            let mut oldlen: ::core::ffi::c_uchar =
                (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as ::core::ffi::c_uchar;
            *fp = (SDS_TYPE_5 | oldlen as ::core::ffi::c_int + incr << SDS_TYPE_BITS)
                as ::core::ffi::c_uchar;
            len = (oldlen as ::core::ffi::c_int + incr) as size_t;
        }
        SDS_TYPE_8 => {
            let mut sh: *mut sdshdr8 = s
                .offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut ::core::ffi::c_void as *mut sdshdr8;
            (*sh).len = ((*sh).len as ::core::ffi::c_int + incr) as uint8_t;
            len = (*sh).len as size_t;
        }
        SDS_TYPE_16 => {
            let mut sh_0: *mut sdshdr16 =
                s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                    as *mut ::core::ffi::c_void as *mut sdshdr16;
            (*sh_0).len = ((*sh_0).len as ::core::ffi::c_int + incr) as uint16_t;
            len = (*sh_0).len as size_t;
        }
        SDS_TYPE_32 => {
            let mut sh_1: *mut sdshdr32 =
                s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                    as *mut ::core::ffi::c_void as *mut sdshdr32;
            (*sh_1).len = (*sh_1).len.wrapping_add(incr as uint32_t);
            len = (*sh_1).len as size_t;
        }
        SDS_TYPE_64 => {
            let mut sh_2: *mut sdshdr64 =
                s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                    as *mut ::core::ffi::c_void as *mut sdshdr64;
            (*sh_2).len = (*sh_2).len.wrapping_add(incr as uint64_t);
            len = (*sh_2).len as size_t;
        }
        _ => {
            len = 0 as size_t;
        }
    }
    *s.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sdsgrowzero(mut s: sds, mut len: size_t) -> sds {
    let mut curlen: size_t = sdslen(s);
    if len <= curlen {
        return s;
    }
    s = sdsMakeRoomFor(s, len.wrapping_sub(curlen));
    if s.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memset(
        s.offset(curlen as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        len.wrapping_sub(curlen).wrapping_add(1 as size_t),
    );
    sdssetlen(s, len);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdscatlen(
    mut s: sds,
    mut t: *const ::core::ffi::c_void,
    mut len: size_t,
) -> sds {
    let mut curlen: size_t = sdslen(s);
    s = sdsMakeRoomFor(s, len);
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
#[no_mangle]
pub unsafe extern "C" fn sdscat(mut s: sds, mut t: *const ::core::ffi::c_char) -> sds {
    return sdscatlen(s, t as *const ::core::ffi::c_void, strlen(t));
}
#[no_mangle]
pub unsafe extern "C" fn sdscatsds(mut s: sds, t: sds) -> sds {
    return sdscatlen(s, t as *const ::core::ffi::c_void, sdslen(t));
}
#[no_mangle]
pub unsafe extern "C" fn sdscpylen(
    mut s: sds,
    mut t: *const ::core::ffi::c_char,
    mut len: size_t,
) -> sds {
    if sdsalloc(s) < len {
        s = sdsMakeRoomFor(s, len.wrapping_sub(sdslen(s)));
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
#[no_mangle]
pub unsafe extern "C" fn sdscpy(mut s: sds, mut t: *const ::core::ffi::c_char) -> sds {
    return sdscpylen(s, t, strlen(t));
}
#[no_mangle]
pub unsafe extern "C" fn sdsll2str(
    mut s: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aux: ::core::ffi::c_char = 0;
    let mut v: ::core::ffi::c_ulonglong = 0;
    let mut l: size_t = 0;
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
    l = p.offset_from(s) as ::core::ffi::c_long as size_t;
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
#[no_mangle]
pub unsafe extern "C" fn sdsull2str(
    mut s: *mut ::core::ffi::c_char,
    mut v: ::core::ffi::c_ulonglong,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aux: ::core::ffi::c_char = 0;
    let mut l: size_t = 0;
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
    l = p.offset_from(s) as ::core::ffi::c_long as size_t;
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
#[no_mangle]
pub unsafe extern "C" fn sdsfromlonglong(mut value: ::core::ffi::c_longlong) -> sds {
    let mut buf: [::core::ffi::c_char; 21] = [0; 21];
    let mut len: ::core::ffi::c_int = sdsll2str(&raw mut buf as *mut ::core::ffi::c_char, value);
    return sdsnewlen(
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sdscatvprintf(
    mut s: sds,
    mut fmt: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) -> sds {
    let mut cpy: ::core::ffi::VaListImpl;
    let mut staticbuf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut buf: *mut ::core::ffi::c_char = &raw mut staticbuf as *mut ::core::ffi::c_char;
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buflen: size_t = strlen(fmt).wrapping_mul(2 as size_t);
    if buflen > ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() {
        buf = malloc(buflen) as *mut ::core::ffi::c_char;
        if buf.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    } else {
        buflen = ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t;
    }
    loop {
        *buf.offset(buflen.wrapping_sub(2 as size_t) as isize) = '\0' as i32 as ::core::ffi::c_char;
        cpy = ap.clone();
        vsnprintf(buf, buflen, fmt, cpy.as_va_list());
        if !(*buf.offset(buflen.wrapping_sub(2 as size_t) as isize) as ::core::ffi::c_int
            != '\0' as i32)
        {
            break;
        }
        if buf != &raw mut staticbuf as *mut ::core::ffi::c_char {
            free(buf as *mut ::core::ffi::c_void);
        }
        buflen = buflen.wrapping_mul(2 as size_t);
        buf = malloc(buflen) as *mut ::core::ffi::c_char;
        if buf.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    t = sdscat(s, buf) as *mut ::core::ffi::c_char;
    if buf != &raw mut staticbuf as *mut ::core::ffi::c_char {
        free(buf as *mut ::core::ffi::c_void);
    }
    return t as sds;
}
#[no_mangle]
pub unsafe extern "C" fn sdscatprintf(
    mut s: sds,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> sds {
    let mut ap: ::core::ffi::VaListImpl;
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    ap = args.clone();
    t = sdscatvprintf(s, fmt, ap.as_va_list()) as *mut ::core::ffi::c_char;
    return t as sds;
}
#[no_mangle]
pub unsafe extern "C" fn sdscatfmt(
    mut s: sds,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> sds {
    let mut initlen: size_t = sdslen(s);
    let mut f: *const ::core::ffi::c_char = fmt;
    let mut i: ::core::ffi::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    f = fmt;
    i = initlen as ::core::ffi::c_int;
    while *f != 0 {
        let mut next: ::core::ffi::c_char = 0;
        let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut l: size_t = 0;
        let mut num: ::core::ffi::c_longlong = 0;
        let mut unum: ::core::ffi::c_ulonglong = 0;
        if sdsavail(s) == 0 as size_t {
            s = sdsMakeRoomFor(s, 1 as size_t);
        }
        match *f as ::core::ffi::c_int {
            37 => {
                next = *f.offset(1 as ::core::ffi::c_int as isize);
                f = f.offset(1);
                match next as ::core::ffi::c_int {
                    115 | 83 => {
                        str = ap.arg::<*mut ::core::ffi::c_char>();
                        l = if next as ::core::ffi::c_int == 's' as i32 {
                            strlen(str)
                        } else {
                            sdslen(str as sds)
                        };
                        if sdsavail(s) < l {
                            s = sdsMakeRoomFor(s, l);
                        }
                        memcpy(
                            s.offset(i as isize) as *mut ::core::ffi::c_void,
                            str as *const ::core::ffi::c_void,
                            l,
                        );
                        sdsinclen(s, l);
                        i = (i as size_t).wrapping_add(l) as ::core::ffi::c_int
                            as ::core::ffi::c_int;
                    }
                    105 | 73 => {
                        if next as ::core::ffi::c_int == 'i' as i32 {
                            num = ap.arg::<::core::ffi::c_int>() as ::core::ffi::c_longlong;
                        } else {
                            num = ap.arg::<::core::ffi::c_longlong>();
                        }
                        let mut buf: [::core::ffi::c_char; 21] = [0; 21];
                        l = sdsll2str(&raw mut buf as *mut ::core::ffi::c_char, num) as size_t;
                        if sdsavail(s) < l {
                            s = sdsMakeRoomFor(s, l);
                        }
                        memcpy(
                            s.offset(i as isize) as *mut ::core::ffi::c_void,
                            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                            l,
                        );
                        sdsinclen(s, l);
                        i = (i as size_t).wrapping_add(l) as ::core::ffi::c_int
                            as ::core::ffi::c_int;
                    }
                    117 | 85 => {
                        if next as ::core::ffi::c_int == 'u' as i32 {
                            unum = ap.arg::<::core::ffi::c_uint>() as ::core::ffi::c_ulonglong;
                        } else {
                            unum = ap.arg::<::core::ffi::c_ulonglong>();
                        }
                        let mut buf_0: [::core::ffi::c_char; 21] = [0; 21];
                        l = sdsull2str(&raw mut buf_0 as *mut ::core::ffi::c_char, unum) as size_t;
                        if sdsavail(s) < l {
                            s = sdsMakeRoomFor(s, l);
                        }
                        memcpy(
                            s.offset(i as isize) as *mut ::core::ffi::c_void,
                            &raw mut buf_0 as *mut ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            l,
                        );
                        sdsinclen(s, l);
                        i = (i as size_t).wrapping_add(l) as ::core::ffi::c_int
                            as ::core::ffi::c_int;
                    }
                    _ => {
                        let fresh4 = i;
                        i = i + 1;
                        *s.offset(fresh4 as isize) = next;
                        sdsinclen(s, 1 as size_t);
                    }
                }
            }
            _ => {
                let fresh5 = i;
                i = i + 1;
                *s.offset(fresh5 as isize) = *f;
                sdsinclen(s, 1 as size_t);
            }
        }
        f = f.offset(1);
    }
    *s.offset(i as isize) = '\0' as i32 as ::core::ffi::c_char;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdstrim(mut s: sds, mut cset: *const ::core::ffi::c_char) -> sds {
    let mut start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
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
    }) as size_t;
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
#[no_mangle]
pub unsafe extern "C" fn sdsrange(
    mut s: sds,
    mut start: ::core::ffi::c_int,
    mut end: ::core::ffi::c_int,
) {
    let mut newlen: size_t = 0;
    let mut len: size_t = sdslen(s);
    if len == 0 as size_t {
        return;
    }
    if start < 0 as ::core::ffi::c_int {
        start = len.wrapping_add(start as size_t) as ::core::ffi::c_int;
        if start < 0 as ::core::ffi::c_int {
            start = 0 as ::core::ffi::c_int;
        }
    }
    if end < 0 as ::core::ffi::c_int {
        end = len.wrapping_add(end as size_t) as ::core::ffi::c_int;
        if end < 0 as ::core::ffi::c_int {
            end = 0 as ::core::ffi::c_int;
        }
    }
    newlen = (if start > end {
        0 as ::core::ffi::c_int
    } else {
        end - start + 1 as ::core::ffi::c_int
    }) as size_t;
    if newlen != 0 as size_t {
        if start >= len as ::core::ffi::c_int {
            newlen = 0 as size_t;
        } else if end >= len as ::core::ffi::c_int {
            end = len.wrapping_sub(1 as size_t) as ::core::ffi::c_int;
            newlen = (if start > end {
                0 as ::core::ffi::c_int
            } else {
                end - start + 1 as ::core::ffi::c_int
            }) as size_t;
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
#[no_mangle]
pub unsafe extern "C" fn sdstolower(mut s: sds) {
    let mut len: ::core::ffi::c_int = sdslen(s) as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < len {
        *s.offset(j as isize) = ({
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_char>() > 1_usize {
                if 0 != 0 {
                    let mut __c: ::core::ffi::c_int = *s.offset(j as isize) as ::core::ffi::c_int;
                    __res =
                        (if __c < -(128 as ::core::ffi::c_int) || __c > 255 as ::core::ffi::c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                } else {
                    __res = tolower(*s.offset(j as isize) as ::core::ffi::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*s.offset(j as isize) as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
            }
            __res
        }) as ::core::ffi::c_char;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sdstoupper(mut s: sds) {
    let mut len: ::core::ffi::c_int = sdslen(s) as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < len {
        *s.offset(j as isize) = ({
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_char>() > 1_usize {
                if 0 != 0 {
                    let mut __c: ::core::ffi::c_int = *s.offset(j as isize) as ::core::ffi::c_int;
                    __res =
                        (if __c < -(128 as ::core::ffi::c_int) || __c > 255 as ::core::ffi::c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                } else {
                    __res = toupper(*s.offset(j as isize) as ::core::ffi::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*s.offset(j as isize) as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
            }
            __res
        }) as ::core::ffi::c_char;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sdscmp(s1: sds, s2: sds) -> ::core::ffi::c_int {
    let mut l1: size_t = 0;
    let mut l2: size_t = 0;
    let mut minlen: size_t = 0;
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
#[no_mangle]
pub unsafe extern "C" fn sdssplitlen(
    mut s: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut sep: *const ::core::ffi::c_char,
    mut seplen: ::core::ffi::c_int,
    mut count: *mut ::core::ffi::c_int,
) -> *mut sds {
    let mut current_block: u64;
    let mut elements: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut slots: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut start: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut tokens: *mut sds = ::core::ptr::null_mut::<sds>();
    if seplen < 1 as ::core::ffi::c_int || len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<sds>();
    }
    tokens =
        malloc((::core::mem::size_of::<sds>() as size_t).wrapping_mul(slots as size_t)) as *mut sds;
    if tokens.is_null() {
        return ::core::ptr::null_mut::<sds>();
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
            let mut newtokens: *mut sds = ::core::ptr::null_mut::<sds>();
            slots *= 2 as ::core::ffi::c_int;
            newtokens = realloc(
                tokens as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<sds>() as size_t).wrapping_mul(slots as size_t),
            ) as *mut sds;
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
                seplen as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let fresh9 = &raw mut *tokens.offset(elements as isize);
            *fresh9 = sdsnewlen(
                s.offset(start as isize) as *const ::core::ffi::c_void,
                (j - start) as size_t,
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
                (len - start) as size_t,
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
    return ::core::ptr::null_mut::<sds>();
}
#[no_mangle]
pub unsafe extern "C" fn sdsfreesplitres(mut tokens: *mut sds, mut count: ::core::ffi::c_int) {
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
#[no_mangle]
pub unsafe extern "C" fn sdscatrepr(
    mut s: sds,
    mut p: *const ::core::ffi::c_char,
    mut len: size_t,
) -> sds {
    s = sdscatlen(
        s,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
    );
    loop {
        let fresh12 = len;
        len = len.wrapping_sub(1);
        if !(fresh12 != 0) {
            break;
        }
        match *p as ::core::ffi::c_int {
            92 | 34 => {
                s = sdscatprintf(
                    s,
                    b"\\%c\0" as *const u8 as *const ::core::ffi::c_char,
                    *p as ::core::ffi::c_int,
                );
            }
            10 => {
                s = sdscatlen(
                    s,
                    b"\\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as size_t,
                );
            }
            13 => {
                s = sdscatlen(
                    s,
                    b"\\r\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as size_t,
                );
            }
            9 => {
                s = sdscatlen(
                    s,
                    b"\\t\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as size_t,
                );
            }
            7 => {
                s = sdscatlen(
                    s,
                    b"\\a\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as size_t,
                );
            }
            8 => {
                s = sdscatlen(
                    s,
                    b"\\b\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as size_t,
                );
            }
            _ => {
                if *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISprint as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0
                {
                    s = sdscatprintf(
                        s,
                        b"%c\0" as *const u8 as *const ::core::ffi::c_char,
                        *p as ::core::ffi::c_int,
                    );
                } else {
                    s = sdscatprintf(
                        s,
                        b"\\x%02x\0" as *const u8 as *const ::core::ffi::c_char,
                        *p as ::core::ffi::c_uchar as ::core::ffi::c_int,
                    );
                }
            }
        }
        p = p.offset(1);
    }
    return sdscatlen(
        s,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn is_hex_digit(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32
        || c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'f' as i32
        || c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32)
        as ::core::ffi::c_int;
}
#[no_mangle]
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
#[no_mangle]
pub unsafe extern "C" fn sdssplitargs(
    mut line: *const ::core::ffi::c_char,
    mut argc: *mut ::core::ffi::c_int,
) -> *mut sds {
    let mut p: *const ::core::ffi::c_char = line;
    let mut current: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut vector: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    *argc = 0 as ::core::ffi::c_int;
    's_13: loop {
        while *p as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
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
                            current as sds,
                            &raw mut byte as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                            1 as size_t,
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
                            current as sds,
                            &raw mut c as *const ::core::ffi::c_void,
                            1 as size_t,
                        ) as *mut ::core::ffi::c_char;
                    } else if *p as ::core::ffi::c_int == '"' as i32 {
                        if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(*p.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    as isize) as ::core::ffi::c_int
                                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int
                                == 0
                        {
                            break 's_13;
                        }
                        done = 1 as ::core::ffi::c_int;
                    } else if *p == 0 {
                        break 's_13;
                    } else {
                        current =
                            sdscatlen(current as sds, p as *const ::core::ffi::c_void, 1 as size_t)
                                as *mut ::core::ffi::c_char;
                    }
                } else if insq != 0 {
                    if *p as ::core::ffi::c_int == '\\' as i32
                        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == '\'' as i32
                    {
                        p = p.offset(1);
                        current = sdscatlen(
                            current as sds,
                            b"'\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            1 as size_t,
                        ) as *mut ::core::ffi::c_char;
                    } else if *p as ::core::ffi::c_int == '\'' as i32 {
                        if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(*p.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    as isize) as ::core::ffi::c_int
                                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int
                                == 0
                        {
                            break 's_13;
                        }
                        done = 1 as ::core::ffi::c_int;
                    } else {
                        if *p == 0 {
                            break 's_13;
                        }
                        current =
                            sdscatlen(current as sds, p as *const ::core::ffi::c_void, 1 as size_t)
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
                                current as sds,
                                p as *const ::core::ffi::c_void,
                                1 as size_t,
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
                ((*argc + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
            ) as *mut *mut ::core::ffi::c_char;
            let fresh13 = &raw mut *vector.offset(*argc as isize);
            *fresh13 = current;
            *argc += 1;
            current = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            if vector.is_null() {
                vector = malloc(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                    as *mut *mut ::core::ffi::c_char;
            }
            return vector as *mut sds;
        }
    }
    loop {
        let fresh14 = *argc;
        *argc = *argc - 1;
        if !(fresh14 != 0) {
            break;
        }
        sdsfree(*vector.offset(*argc as isize) as sds);
    }
    free(vector as *mut ::core::ffi::c_void);
    if !current.is_null() {
        sdsfree(current as sds);
    }
    *argc = 0 as ::core::ffi::c_int;
    return ::core::ptr::null_mut::<sds>();
}
#[no_mangle]
pub unsafe extern "C" fn sdsmapchars(
    mut s: sds,
    mut from: *const ::core::ffi::c_char,
    mut to: *const ::core::ffi::c_char,
    mut setlen: size_t,
) -> sds {
    let mut j: size_t = 0;
    let mut i: size_t = 0;
    let mut l: size_t = sdslen(s);
    j = 0 as size_t;
    while j < l {
        i = 0 as size_t;
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
#[no_mangle]
pub unsafe extern "C" fn sdsjoin(
    mut argv: *mut *mut ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut sep: *mut ::core::ffi::c_char,
) -> sds {
    let mut join: sds = sdsempty();
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
#[no_mangle]
pub unsafe extern "C" fn sdsjoinsds(
    mut argv: *mut sds,
    mut argc: ::core::ffi::c_int,
    mut sep: *const ::core::ffi::c_char,
    mut seplen: size_t,
) -> sds {
    let mut join: sds = sdsempty();
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
