extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn bufnew() -> *mut caryll_Buffer;
}

use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const cff_FDSELECT_UNSPECED: C2RustUnnamed = 2;
pub const cff_FDSELECT_FORMAT3: C2RustUnnamed = 1;
pub const cff_FDSELECT_FORMAT0: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelectFormat0 {
    pub format: uint8_t,
    pub fds: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelectRangeFormat3 {
    pub first: uint16_t,
    pub fd: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelectFormat3 {
    pub format: uint8_t,
    pub nranges: uint16_t,
    pub range3: *mut cff_FDSelectRangeFormat3,
    pub sentinel: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelect {
    pub t: uint32_t,
    pub s: uint32_t,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub f0: cff_FDSelectFormat0,
    pub f3: cff_FDSelectFormat3,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn gu1(mut s: *mut uint8_t, mut p: uint32_t) -> uint32_t {
    let mut b0: uint32_t = *s.offset(p as isize) as uint32_t;
    return b0;
}
#[inline]
unsafe extern "C" fn gu2(mut s: *mut uint8_t, mut p: uint32_t) -> uint32_t {
    let mut b0: uint32_t =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t;
    let mut b1: uint32_t = *s
        .offset(p as isize)
        .offset(1 as ::core::ffi::c_int as isize) as uint32_t;
    return b0 | b1;
}
#[no_mangle]
pub unsafe extern "C" fn cff_close_FDSelect(mut fds: cff_FDSelect) {
    match fds.t {
        0 => {
            if !fds.c2rust_unnamed.f0.fds.is_null() {
                free(fds.c2rust_unnamed.f0.fds as *mut ::core::ffi::c_void);
                fds.c2rust_unnamed.f0.fds = ::core::ptr::null_mut::<uint8_t>();
            }
        }
        1 => {
            if !fds.c2rust_unnamed.f3.range3.is_null() {
                free(fds.c2rust_unnamed.f3.range3 as *mut ::core::ffi::c_void);
                fds.c2rust_unnamed.f3.range3 = ::core::ptr::null_mut::<cff_FDSelectRangeFormat3>();
            }
        }
        2 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_build_FDSelect(mut fd: cff_FDSelect) -> *mut caryll_Buffer {
    match fd.t {
        2 => return bufnew(),
        0 => {
            let mut blob: *mut caryll_Buffer = bufnew();
            (*blob).size = (1 as uint32_t).wrapping_add(fd.s) as size_t;
            (*blob).data = __caryll_allocate_clean(
                (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*blob).size),
                24 as ::core::ffi::c_ulong,
            ) as *mut uint8_t;
            let mut j: uint16_t = 0 as uint16_t;
            while (j as uint32_t) < fd.s {
                *(*blob).data.offset(j as isize) = *fd.c2rust_unnamed.f0.fds.offset(j as isize);
                j = j.wrapping_add(1);
            }
            return blob;
        }
        1 => {
            let mut blob_0: *mut caryll_Buffer = bufnew();
            (*blob_0).size = (5 as ::core::ffi::c_int
                + fd.c2rust_unnamed.f3.nranges as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as size_t;
            (*blob_0).data = __caryll_allocate_clean(
                (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*blob_0).size),
                33 as ::core::ffi::c_ulong,
            ) as *mut uint8_t;
            *(*blob_0).data.offset(0 as ::core::ffi::c_int as isize) = 3 as uint8_t;
            *(*blob_0).data.offset(1 as ::core::ffi::c_int as isize) =
                (fd.c2rust_unnamed.f3.nranges as ::core::ffi::c_int / 256 as ::core::ffi::c_int)
                    as uint8_t;
            *(*blob_0).data.offset(2 as ::core::ffi::c_int as isize) =
                (fd.c2rust_unnamed.f3.nranges as ::core::ffi::c_int % 256 as ::core::ffi::c_int)
                    as uint8_t;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < fd.c2rust_unnamed.f3.nranges as ::core::ffi::c_int {
                *(*blob_0)
                    .data
                    .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * i) as isize) =
                    ((*fd.c2rust_unnamed.f3.range3.offset(i as isize)).first as ::core::ffi::c_int
                        / 256 as ::core::ffi::c_int) as uint8_t;
                *(*blob_0)
                    .data
                    .offset((4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * i) as isize) =
                    ((*fd.c2rust_unnamed.f3.range3.offset(i as isize)).first as ::core::ffi::c_int
                        % 256 as ::core::ffi::c_int) as uint8_t;
                *(*blob_0)
                    .data
                    .offset((5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * i) as isize) =
                    (*fd.c2rust_unnamed.f3.range3.offset(i as isize)).fd;
                i += 1;
            }
            *(*blob_0)
                .data
                .offset((*blob_0).size.wrapping_sub(2 as size_t) as isize) =
                (fd.c2rust_unnamed.f3.sentinel as ::core::ffi::c_int / 256 as ::core::ffi::c_int)
                    as uint8_t;
            *(*blob_0)
                .data
                .offset((*blob_0).size.wrapping_sub(1 as size_t) as isize) =
                (fd.c2rust_unnamed.f3.sentinel as ::core::ffi::c_int % 256 as ::core::ffi::c_int)
                    as uint8_t;
            return blob_0;
        }
        _ => return ::core::ptr::null_mut::<caryll_Buffer>(),
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_extract_FDSelect(
    mut data: *mut uint8_t,
    mut offset: int32_t,
    mut nchars: uint16_t,
    mut fdselect: *mut cff_FDSelect,
) {
    match *data.offset(offset as isize) as ::core::ffi::c_int {
        0 => {
            (*fdselect).t = cff_FDSELECT_FORMAT0 as ::core::ffi::c_int as uint32_t;
            (*fdselect).c2rust_unnamed.f0.format = 0 as uint8_t;
            (*fdselect).s = nchars as uint32_t;
            (*fdselect).c2rust_unnamed.f0.fds = __caryll_allocate_clean(
                (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(nchars as size_t),
                55 as ::core::ffi::c_ulong,
            ) as *mut uint8_t;
            let mut i: uint32_t = 0 as uint32_t;
            while i < nchars as uint32_t {
                *(*fdselect).c2rust_unnamed.f0.fds.offset(i as isize) =
                    gu1(data, ((offset + 1 as int32_t) as uint32_t).wrapping_add(i)) as uint8_t;
                i = i.wrapping_add(1);
            }
        }
        3 => {
            (*fdselect).t = cff_FDSELECT_FORMAT3 as ::core::ffi::c_int as uint32_t;
            (*fdselect).c2rust_unnamed.f3.format = 3 as uint8_t;
            (*fdselect).c2rust_unnamed.f3.nranges =
                gu2(data, (offset + 1 as int32_t) as uint32_t) as uint16_t;
            (*fdselect).c2rust_unnamed.f3.range3 = __caryll_allocate_clean(
                (::core::mem::size_of::<cff_FDSelectRangeFormat3>() as size_t)
                    .wrapping_mul((*fdselect).c2rust_unnamed.f3.nranges as size_t),
                66 as ::core::ffi::c_ulong,
            ) as *mut cff_FDSelectRangeFormat3;
            let mut i_0: uint32_t = 0 as uint32_t;
            while i_0 < (*fdselect).c2rust_unnamed.f3.nranges as uint32_t {
                (*(*fdselect).c2rust_unnamed.f3.range3.offset(i_0 as isize)).first = gu2(
                    data,
                    ((offset + 3 as int32_t) as uint32_t)
                        .wrapping_add(i_0.wrapping_mul(3 as uint32_t)),
                )
                    as uint16_t;
                (*(*fdselect).c2rust_unnamed.f3.range3.offset(i_0 as isize)).fd = gu1(
                    data,
                    ((offset + 5 as int32_t) as uint32_t)
                        .wrapping_add(i_0.wrapping_mul(3 as uint32_t)),
                )
                    as uint8_t;
                i_0 = i_0.wrapping_add(1);
            }
            (*fdselect).c2rust_unnamed.f3.sentinel = gu2(
                data,
                (offset
                    + ((*fdselect).c2rust_unnamed.f3.nranges as int32_t + 1 as int32_t)
                        * 3 as int32_t) as uint32_t,
            ) as uint16_t;
        }
        _ => {
            (*fdselect).t = cff_FDSELECT_UNSPECED as ::core::ffi::c_int as uint32_t;
        }
    };
}
