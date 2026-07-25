use libc::{free};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{caryll_Buffer};
pub type cff_FDSelectType = ::core::ffi::c_uint;
pub const cff_FDSELECT_UNSPECED: cff_FDSelectType = 2;
pub const cff_FDSELECT_FORMAT3: cff_FDSelectType = 1;
pub const cff_FDSELECT_FORMAT0: cff_FDSelectType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelectFormat0 {
    pub format: u8,
    pub fds: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelectRangeFormat3 {
    pub first: u16,
    pub fd: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelectFormat3 {
    pub format: u8,
    pub nranges: u16,
    pub range3: *mut cff_FDSelectRangeFormat3,
    pub sentinel: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FDSelect {
    pub t: u32,
    pub s: u32,
    pub c2rust_unnamed: cff_FDSelectBody,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cff_FDSelectBody {
    pub f0: cff_FDSelectFormat0,
    pub f3: cff_FDSelectFormat3,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn gu1(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 = *s.offset(p as isize) as u32;
    return b0;
}
#[inline]
unsafe extern "C" fn gu2(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
    let mut b1: u32 = *s
        .offset(p as isize)
        .offset(1 as ::core::ffi::c_int as isize) as u32;
    return b0 | b1;
}
#[no_mangle]
pub unsafe extern "C" fn cff_close_FDSelect(mut fds: cff_FDSelect) {
    match fds.t {
        0 => {
            if !fds.c2rust_unnamed.f0.fds.is_null() {
                free(fds.c2rust_unnamed.f0.fds as *mut ::core::ffi::c_void);
                fds.c2rust_unnamed.f0.fds = ::core::ptr::null_mut::<u8>();
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
            (*blob).size = (1 as u32).wrapping_add(fd.s) as usize;
            (*blob).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob).size),
                24 as ::core::ffi::c_ulong,
            ) as *mut u8;
            let mut j: u16 = 0 as u16;
            while (j as u32) < fd.s {
                *(*blob).data.offset(j as isize) = *fd.c2rust_unnamed.f0.fds.offset(j as isize);
                j = j.wrapping_add(1);
            }
            return blob;
        }
        1 => {
            let mut blob_0: *mut caryll_Buffer = bufnew();
            (*blob_0).size = (5 as ::core::ffi::c_int
                + fd.c2rust_unnamed.f3.nranges as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as usize;
            (*blob_0).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob_0).size),
                33 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob_0).data.offset(0 as ::core::ffi::c_int as isize) = 3 as u8;
            *(*blob_0).data.offset(1 as ::core::ffi::c_int as isize) =
                (fd.c2rust_unnamed.f3.nranges as ::core::ffi::c_int / 256 as ::core::ffi::c_int)
                    as u8;
            *(*blob_0).data.offset(2 as ::core::ffi::c_int as isize) =
                (fd.c2rust_unnamed.f3.nranges as ::core::ffi::c_int % 256 as ::core::ffi::c_int)
                    as u8;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < fd.c2rust_unnamed.f3.nranges as ::core::ffi::c_int {
                *(*blob_0)
                    .data
                    .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * i) as isize) =
                    ((*fd.c2rust_unnamed.f3.range3.offset(i as isize)).first as ::core::ffi::c_int
                        / 256 as ::core::ffi::c_int) as u8;
                *(*blob_0)
                    .data
                    .offset((4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * i) as isize) =
                    ((*fd.c2rust_unnamed.f3.range3.offset(i as isize)).first as ::core::ffi::c_int
                        % 256 as ::core::ffi::c_int) as u8;
                *(*blob_0)
                    .data
                    .offset((5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * i) as isize) =
                    (*fd.c2rust_unnamed.f3.range3.offset(i as isize)).fd;
                i += 1;
            }
            *(*blob_0)
                .data
                .offset((*blob_0).size.wrapping_sub(2 as usize) as isize) =
                (fd.c2rust_unnamed.f3.sentinel as ::core::ffi::c_int / 256 as ::core::ffi::c_int)
                    as u8;
            *(*blob_0)
                .data
                .offset((*blob_0).size.wrapping_sub(1 as usize) as isize) =
                (fd.c2rust_unnamed.f3.sentinel as ::core::ffi::c_int % 256 as ::core::ffi::c_int)
                    as u8;
            return blob_0;
        }
        _ => return ::core::ptr::null_mut::<caryll_Buffer>(),
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_extract_FDSelect(
    mut data: *mut u8,
    mut offset: i32,
    mut nchars: u16,
    mut fdselect: *mut cff_FDSelect,
) {
    match *data.offset(offset as isize) as ::core::ffi::c_int {
        0 => {
            (*fdselect).t = cff_FDSELECT_FORMAT0 as ::core::ffi::c_int as u32;
            (*fdselect).c2rust_unnamed.f0.format = 0 as u8;
            (*fdselect).s = nchars as u32;
            (*fdselect).c2rust_unnamed.f0.fds = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul(nchars as usize),
                55 as ::core::ffi::c_ulong,
            ) as *mut u8;
            let mut i: u32 = 0 as u32;
            while i < nchars as u32 {
                *(*fdselect).c2rust_unnamed.f0.fds.offset(i as isize) =
                    gu1(data, ((offset + 1 as i32) as u32).wrapping_add(i)) as u8;
                i = i.wrapping_add(1);
            }
        }
        3 => {
            (*fdselect).t = cff_FDSELECT_FORMAT3 as ::core::ffi::c_int as u32;
            (*fdselect).c2rust_unnamed.f3.format = 3 as u8;
            (*fdselect).c2rust_unnamed.f3.nranges =
                gu2(data, (offset + 1 as i32) as u32) as u16;
            (*fdselect).c2rust_unnamed.f3.range3 = __caryll_allocate_clean(
                (::core::mem::size_of::<cff_FDSelectRangeFormat3>() as usize)
                    .wrapping_mul((*fdselect).c2rust_unnamed.f3.nranges as usize),
                66 as ::core::ffi::c_ulong,
            ) as *mut cff_FDSelectRangeFormat3;
            let mut i_0: u32 = 0 as u32;
            while i_0 < (*fdselect).c2rust_unnamed.f3.nranges as u32 {
                (*(*fdselect).c2rust_unnamed.f3.range3.offset(i_0 as isize)).first = gu2(
                    data,
                    ((offset + 3 as i32) as u32)
                        .wrapping_add(i_0.wrapping_mul(3 as u32)),
                )
                    as u16;
                (*(*fdselect).c2rust_unnamed.f3.range3.offset(i_0 as isize)).fd = gu1(
                    data,
                    ((offset + 5 as i32) as u32)
                        .wrapping_add(i_0.wrapping_mul(3 as u32)),
                )
                    as u8;
                i_0 = i_0.wrapping_add(1);
            }
            (*fdselect).c2rust_unnamed.f3.sentinel = gu2(
                data,
                (offset
                    + ((*fdselect).c2rust_unnamed.f3.nranges as i32 + 1 as i32)
                        * 3 as i32) as u32,
            ) as u16;
        }
        _ => {
            (*fdselect).t = cff_FDSELECT_UNSPECED as ::core::ffi::c_int as u32;
        }
    };
}
