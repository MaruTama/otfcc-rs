#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{Buffer};
use crate::support::buffer::{bufnew};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffFdSelectRangeFormat3 {
    pub first: u16,
    pub fd: u8,
}
/// Was a `t: CffFdSelectType` discriminant plus a `c2rust_unnamed:
/// CffFdSelectBody` union (`f0`/`f3`, one raw-pointer array each) -- the
/// same shape `CffEncoding`/`CffCharset` had, and the same fix: a single
/// enum, discriminant and payload together. `s`/`nranges` are gone too --
/// both were write-only (set once while parsing or building, never read
/// again anywhere in the crate) and exactly duplicated `range3`'s own
/// `.len()`. `sentinel` is kept: unlike the counts, it is a genuine data
/// value (the one-past-the-last glyph index Format3's final range extends
/// to), not derivable from the `Vec` itself.
#[derive(Clone)]
pub enum CffFdSelect {
    Unspecified,
    Format0(Vec<u8>),
    Format3 { range3: Vec<CffFdSelectRangeFormat3>, sentinel: u16 },
}
#[inline]
unsafe fn gu1(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 = *s.offset(p as isize) as u32;
    return b0;
}
#[inline]
unsafe fn gu2(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
    let mut b1: u32 = *s
        .offset(p as isize)
        .offset(1 as ::core::ffi::c_int as isize) as u32;
    return b0 | b1;
}
// Takes `&CffFdSelect` instead of by value -- it only ever reads the data to
// serialize it, same reasoning as `cff_build_charset`.
pub unsafe fn cff_build_fd_select(fd: &CffFdSelect) -> *mut Buffer {
    match fd {
        CffFdSelect::Unspecified => bufnew(),
        CffFdSelect::Format0(fds) => {
            let blob: *mut Buffer = bufnew();
            (*blob).size = (1 as u32).wrapping_add(fds.len() as u32) as usize;
            (*blob).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob).size),
                24 as ::core::ffi::c_ulong,
            ) as *mut u8;
            for (j, &b) in fds.iter().enumerate() {
                *(*blob).data.offset(j as isize) = b;
            }
            blob
        }
        CffFdSelect::Format3 { range3, sentinel } => {
            let blob_0: *mut Buffer = bufnew();
            let nranges = range3.len() as ::core::ffi::c_int;
            (*blob_0).size = (5 as ::core::ffi::c_int + nranges * 3 as ::core::ffi::c_int) as usize;
            (*blob_0).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob_0).size),
                33 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob_0).data.offset(0 as ::core::ffi::c_int as isize) = 3 as u8;
            *(*blob_0).data.offset(1 as ::core::ffi::c_int as isize) =
                (nranges / 256 as ::core::ffi::c_int) as u8;
            *(*blob_0).data.offset(2 as ::core::ffi::c_int as isize) =
                (nranges % 256 as ::core::ffi::c_int) as u8;
            for (i, r) in range3.iter().enumerate() {
                let i = i as ::core::ffi::c_int;
                *(*blob_0)
                    .data
                    .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * i) as isize) =
                    (r.first as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8;
                *(*blob_0)
                    .data
                    .offset((4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * i) as isize) =
                    (r.first as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8;
                *(*blob_0)
                    .data
                    .offset((5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * i) as isize) = r.fd;
            }
            *(*blob_0)
                .data
                .offset((*blob_0).size.wrapping_sub(2 as usize) as isize) =
                (*sentinel as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8;
            *(*blob_0)
                .data
                .offset((*blob_0).size.wrapping_sub(1 as usize) as isize) =
                (*sentinel as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8;
            blob_0
        }
    }
}
// Returns `CffFdSelect` by value instead of writing through a `*mut
// CffFdSelect` out-param -- the same "unwrap_X_table" shape used throughout
// this migration.
pub unsafe fn cff_extract_fd_select(data: *mut u8, offset: i32, nchars: u16) -> CffFdSelect {
    match *data.offset(offset as isize) as ::core::ffi::c_int {
        0 => {
            let mut fds: Vec<u8> = Vec::with_capacity(nchars as usize);
            let mut i: u32 = 0 as u32;
            while i < nchars as u32 {
                fds.push(gu1(data, ((offset + 1 as i32) as u32).wrapping_add(i)) as u8);
                i = i.wrapping_add(1);
            }
            CffFdSelect::Format0(fds)
        }
        3 => {
            let nranges = gu2(data, (offset + 1 as i32) as u32) as u16;
            let mut range3: Vec<CffFdSelectRangeFormat3> = Vec::with_capacity(nranges as usize);
            let mut i_0: u32 = 0 as u32;
            while i_0 < nranges as u32 {
                let first = gu2(
                    data,
                    ((offset + 3 as i32) as u32).wrapping_add(i_0.wrapping_mul(3 as u32)),
                ) as u16;
                let fd = gu1(
                    data,
                    ((offset + 5 as i32) as u32).wrapping_add(i_0.wrapping_mul(3 as u32)),
                ) as u8;
                range3.push(CffFdSelectRangeFormat3 { first, fd });
                i_0 = i_0.wrapping_add(1);
            }
            let sentinel = gu2(
                data,
                (offset + (nranges as i32 + 1 as i32) * 3 as i32) as u32,
            ) as u16;
            CffFdSelect::Format3 { range3, sentinel }
        }
        _ => CffFdSelect::Unspecified,
    }
}
