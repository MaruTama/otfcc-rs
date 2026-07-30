#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{Buffer};
use crate::support::buffer::{bufnew};

/// Which FDSelect format a CID font uses, or `UNSPECED` for a font that has
/// none and for a format byte otfcc does not recognise.
///
/// Not the format byte itself: `cff_extract_fd_select` matches the byte from the
/// file and stores one of these three, so the value is the crate's own and every
/// unknown byte lands on `UNSPECED`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CffFdSelectType {
    Format0 = 0,
    Format3 = 1,
    Unspecified = 2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffFdSelectFormat0 {
    pub format: u8,
    pub fds: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffFdSelectRangeFormat3 {
    pub first: u16,
    pub fd: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffFdSelectFormat3 {
    pub format: u8,
    pub nranges: u16,
    pub range3: *mut CffFdSelectRangeFormat3,
    pub sentinel: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffFdSelect {
    pub t: CffFdSelectType,
    pub s: u32,
    pub c2rust_unnamed: CffFdSelectBody,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CffFdSelectBody {
    pub f0: CffFdSelectFormat0,
    pub f3: CffFdSelectFormat3,
}
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
pub unsafe extern "C" fn cff_close_fd_select(mut fds: CffFdSelect) {
    match fds.t {
        CffFdSelectType::Format0 => {
            if !fds.c2rust_unnamed.f0.fds.is_null() {
                free(fds.c2rust_unnamed.f0.fds as *mut ::core::ffi::c_void);
                fds.c2rust_unnamed.f0.fds = ::core::ptr::null_mut::<u8>();
            }
        }
        CffFdSelectType::Format3 => {
            if !fds.c2rust_unnamed.f3.range3.is_null() {
                free(fds.c2rust_unnamed.f3.range3 as *mut ::core::ffi::c_void);
                fds.c2rust_unnamed.f3.range3 = ::core::ptr::null_mut::<CffFdSelectRangeFormat3>();
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn cff_build_fd_select(mut fd: CffFdSelect) -> *mut Buffer {
    match fd.t {
        CffFdSelectType::Unspecified => return bufnew(),
        CffFdSelectType::Format0 => {
            let mut blob: *mut Buffer = bufnew();
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
        CffFdSelectType::Format3 => {
            let mut blob_0: *mut Buffer = bufnew();
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
    };
}
pub unsafe extern "C" fn cff_extract_fd_select(
    mut data: *mut u8,
    mut offset: i32,
    mut nchars: u16,
    mut fdselect: *mut CffFdSelect,
) {
    match *data.offset(offset as isize) as ::core::ffi::c_int {
        0 => {
            (*fdselect).t = CffFdSelectType::Format0;
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
            (*fdselect).t = CffFdSelectType::Format3;
            (*fdselect).c2rust_unnamed.f3.format = 3 as u8;
            (*fdselect).c2rust_unnamed.f3.nranges =
                gu2(data, (offset + 1 as i32) as u32) as u16;
            (*fdselect).c2rust_unnamed.f3.range3 = __caryll_allocate_clean(
                (::core::mem::size_of::<CffFdSelectRangeFormat3>() as usize)
                    .wrapping_mul((*fdselect).c2rust_unnamed.f3.nranges as usize),
                66 as ::core::ffi::c_ulong,
            ) as *mut CffFdSelectRangeFormat3;
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
            (*fdselect).t = CffFdSelectType::Unspecified;
        }
    };
}
