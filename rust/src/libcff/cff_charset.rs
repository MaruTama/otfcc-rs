use libc::{free};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{caryll_Buffer};
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const cff_CHARSET_FORMAT2: C2RustUnnamed = 5;
pub const cff_CHARSET_FORMAT1: C2RustUnnamed = 4;
pub const cff_CHARSET_FORMAT0: C2RustUnnamed = 3;
pub const cff_CHARSET_EXPERTSUBSET: C2RustUnnamed = 2;
pub const cff_CHARSET_EXPERT: C2RustUnnamed = 1;
pub const cff_CHARSET_UNSPECED: C2RustUnnamed = 0;
pub const cff_CHARSET_ISOADOBE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat0 {
    pub format: u8,
    pub glyph: *mut u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetRangeFormat1 {
    pub first: u16,
    pub nleft: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat1 {
    pub format: u8,
    pub range1: *mut cff_CharsetRangeFormat1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetRangeFormat2 {
    pub first: u16,
    pub nleft: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat2 {
    pub format: u8,
    pub range2: *mut cff_CharsetRangeFormat2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Charset {
    pub t: u32,
    pub s: u32,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub f0: cff_CharsetFormat0,
    pub f1: cff_CharsetFormat1,
    pub f2: cff_CharsetFormat2,
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
pub unsafe extern "C" fn cff_extract_Charset(
    mut data: *mut u8,
    mut offset: i32,
    mut nchars: u16,
    mut charsets: *mut cff_Charset,
) {
    let mut i: u32 = 0;
    if offset == cff_CHARSET_ISOADOBE as ::core::ffi::c_int as i32 {
        (*charsets).t = cff_CHARSET_ISOADOBE as ::core::ffi::c_int as u32;
    } else if offset == cff_CHARSET_EXPERT as ::core::ffi::c_int as i32 {
        (*charsets).t = cff_CHARSET_EXPERT as ::core::ffi::c_int as u32;
    } else if offset == cff_CHARSET_EXPERTSUBSET as ::core::ffi::c_int as i32 {
        (*charsets).t = cff_CHARSET_EXPERTSUBSET as ::core::ffi::c_int as u32;
    } else {
        match *data.offset(offset as isize) as ::core::ffi::c_int {
            0 => {
                (*charsets).t = cff_CHARSET_FORMAT0 as ::core::ffi::c_int as u32;
                (*charsets).s =
                    (nchars as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u32;
                (*charsets).c2rust_unnamed.f0.glyph = __caryll_allocate_clean(
                    (::core::mem::size_of::<u16>() as usize).wrapping_mul(
                        (nchars as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize,
                    ),
                    18 as ::core::ffi::c_ulong,
                ) as *mut u16;
                i = 0 as u32;
                while i < (*charsets).s {
                    *(*charsets).c2rust_unnamed.f0.glyph.offset(i as isize) = gu2(
                        data,
                        ((offset + 1 as i32) as u32)
                            .wrapping_add(i.wrapping_mul(2 as u32)),
                    )
                        as u16;
                    i = i.wrapping_add(1);
                }
            }
            1 => {
                (*charsets).t = cff_CHARSET_FORMAT1 as ::core::ffi::c_int as u32;
                let mut size: u32 = 0;
                let mut glyphsEncodedSofar: u32 = 1 as u32;
                i = 0 as u32;
                while glyphsEncodedSofar < nchars as u32 {
                    glyphsEncodedSofar = glyphsEncodedSofar.wrapping_add(
                        (1 as u32).wrapping_add(gu1(
                            data,
                            ((offset + 3 as i32) as u32)
                                .wrapping_add(i.wrapping_mul(3 as u32)),
                        )),
                    );
                    i = i.wrapping_add(1);
                }
                size = i;
                (*charsets).s = size;
                (*charsets).c2rust_unnamed.f1.range1 = __caryll_allocate_clean(
                    (::core::mem::size_of::<cff_CharsetRangeFormat1>() as usize)
                        .wrapping_mul(i.wrapping_add(1 as u32) as usize),
                    35 as ::core::ffi::c_ulong,
                )
                    as *mut cff_CharsetRangeFormat1;
                i = 0 as u32;
                while i < size {
                    (*(*charsets).c2rust_unnamed.f1.range1.offset(i as isize)).first = gu2(
                        data,
                        ((offset + 1 as i32) as u32)
                            .wrapping_add(i.wrapping_mul(3 as u32)),
                    )
                        as u16;
                    (*(*charsets).c2rust_unnamed.f1.range1.offset(i as isize)).nleft = gu1(
                        data,
                        ((offset + 3 as i32) as u32)
                            .wrapping_add(i.wrapping_mul(3 as u32)),
                    )
                        as u8;
                    i = i.wrapping_add(1);
                }
            }
            2 => {
                (*charsets).t = cff_CHARSET_FORMAT2 as ::core::ffi::c_int as u32;
                let mut size_0: u32 = 0;
                let mut glyphsEncodedSofar_0: u32 = 1 as u32;
                i = 0 as u32;
                while glyphsEncodedSofar_0 < nchars as u32 {
                    glyphsEncodedSofar_0 = glyphsEncodedSofar_0.wrapping_add(
                        (1 as u32).wrapping_add(gu2(
                            data,
                            ((offset + 3 as i32) as u32)
                                .wrapping_add(i.wrapping_mul(4 as u32)),
                        )),
                    );
                    i = i.wrapping_add(1);
                }
                size_0 = i;
                (*charsets).s = size_0;
                (*charsets).c2rust_unnamed.f2.range2 = __caryll_allocate_clean(
                    (::core::mem::size_of::<cff_CharsetRangeFormat2>() as usize)
                        .wrapping_mul(i.wrapping_add(1 as u32) as usize),
                    53 as ::core::ffi::c_ulong,
                )
                    as *mut cff_CharsetRangeFormat2;
                i = 0 as u32;
                while i < size_0 {
                    (*(*charsets).c2rust_unnamed.f2.range2.offset(i as isize)).first = gu2(
                        data,
                        ((offset + 1 as i32) as u32)
                            .wrapping_add(i.wrapping_mul(4 as u32)),
                    )
                        as u16;
                    (*(*charsets).c2rust_unnamed.f2.range2.offset(i as isize)).nleft = gu2(
                        data,
                        ((offset + 3 as i32) as u32)
                            .wrapping_add(i.wrapping_mul(4 as u32)),
                    )
                        as u16;
                    i = i.wrapping_add(1);
                }
            }
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_build_Charset(mut cset: cff_Charset) -> *mut caryll_Buffer {
    match cset.t {
        0 | 1 | 2 => return bufnew(),
        3 => {
            let mut blob: *mut caryll_Buffer = bufnew();
            (*blob).size =
                (1 as u32).wrapping_add(cset.s.wrapping_mul(2 as u32)) as usize;
            (*blob).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob).size),
                75 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob).data.offset(0 as ::core::ffi::c_int as isize) = 0 as u8;
            let mut i: u32 = 0 as u32;
            while i < cset.s {
                *(*blob).data.offset(
                    (1 as u32).wrapping_add((2 as u32).wrapping_mul(i)) as isize,
                ) = (*cset.c2rust_unnamed.f0.glyph.offset(i as isize) as ::core::ffi::c_int
                    / 256 as ::core::ffi::c_int) as u8;
                *(*blob).data.offset(
                    (2 as u32).wrapping_add((2 as u32).wrapping_mul(i)) as isize,
                ) = (*cset.c2rust_unnamed.f0.glyph.offset(i as isize) as ::core::ffi::c_int
                    % 256 as ::core::ffi::c_int) as u8;
                i = i.wrapping_add(1);
            }
            (*blob).cursor = (*blob).size;
            return blob;
        }
        4 => {
            let mut blob_0: *mut caryll_Buffer = bufnew();
            (*blob_0).size =
                (1 as u32).wrapping_add(cset.s.wrapping_mul(3 as u32)) as usize;
            (*blob_0).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob_0).size),
                85 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob_0).data.offset(0 as ::core::ffi::c_int as isize) = 1 as u8;
            let mut i_0: u32 = 0 as u32;
            while i_0 < cset.s {
                *(*blob_0).data.offset(
                    (1 as u32).wrapping_add((3 as u32).wrapping_mul(i_0)) as isize,
                ) = ((*cset.c2rust_unnamed.f1.range1.offset(i_0 as isize)).first
                    as ::core::ffi::c_int
                    / 256 as ::core::ffi::c_int) as u8;
                *(*blob_0).data.offset(
                    (2 as u32).wrapping_add((3 as u32).wrapping_mul(i_0)) as isize,
                ) = ((*cset.c2rust_unnamed.f1.range1.offset(i_0 as isize)).first
                    as ::core::ffi::c_int
                    % 256 as ::core::ffi::c_int) as u8;
                *(*blob_0).data.offset(
                    (3 as u32).wrapping_add((3 as u32).wrapping_mul(i_0)) as isize,
                ) = (*cset.c2rust_unnamed.f1.range1.offset(i_0 as isize)).nleft;
                i_0 = i_0.wrapping_add(1);
            }
            return blob_0;
        }
        5 => {
            let mut blob_1: *mut caryll_Buffer = bufnew();
            (*blob_1).size =
                (1 as u32).wrapping_add(cset.s.wrapping_mul(4 as u32)) as usize;
            (*blob_1).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob_1).size),
                96 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob_1).data.offset(0 as ::core::ffi::c_int as isize) = 2 as u8;
            let mut i_1: u32 = 0 as u32;
            while i_1 < cset.s {
                *(*blob_1).data.offset(
                    (1 as u32).wrapping_add((4 as u32).wrapping_mul(i_1)) as isize,
                ) = ((*cset.c2rust_unnamed.f2.range2.offset(i_1 as isize)).first
                    as ::core::ffi::c_int
                    / 256 as ::core::ffi::c_int) as u8;
                *(*blob_1).data.offset(
                    (2 as u32).wrapping_add((4 as u32).wrapping_mul(i_1)) as isize,
                ) = ((*cset.c2rust_unnamed.f2.range2.offset(i_1 as isize)).first
                    as ::core::ffi::c_int
                    % 256 as ::core::ffi::c_int) as u8;
                *(*blob_1).data.offset(
                    (3 as u32).wrapping_add((4 as u32).wrapping_mul(i_1)) as isize,
                ) = ((*cset.c2rust_unnamed.f2.range2.offset(i_1 as isize)).nleft
                    as ::core::ffi::c_int
                    / 256 as ::core::ffi::c_int) as u8;
                *(*blob_1).data.offset(
                    (4 as u32).wrapping_add((4 as u32).wrapping_mul(i_1)) as isize,
                ) = ((*cset.c2rust_unnamed.f2.range2.offset(i_1 as isize)).nleft
                    as ::core::ffi::c_int
                    % 256 as ::core::ffi::c_int) as u8;
                i_1 = i_1.wrapping_add(1);
            }
            (*blob_1).cursor = (*blob_1).size;
            return blob_1;
        }
        _ => {}
    }
    return ::core::ptr::null_mut::<caryll_Buffer>();
}
#[no_mangle]
pub unsafe extern "C" fn cff_close_Charset(mut cset: cff_Charset) {
    match cset.t {
        3 => {
            if !cset.c2rust_unnamed.f0.glyph.is_null() {
                free(cset.c2rust_unnamed.f0.glyph as *mut ::core::ffi::c_void);
                cset.c2rust_unnamed.f0.glyph = ::core::ptr::null_mut::<u16>();
            }
        }
        4 => {
            if !cset.c2rust_unnamed.f1.range1.is_null() {
                free(cset.c2rust_unnamed.f1.range1 as *mut ::core::ffi::c_void);
                cset.c2rust_unnamed.f1.range1 = ::core::ptr::null_mut::<cff_CharsetRangeFormat1>();
            }
        }
        5 => {
            if !cset.c2rust_unnamed.f2.range2.is_null() {
                free(cset.c2rust_unnamed.f2.range2 as *mut ::core::ffi::c_void);
                cset.c2rust_unnamed.f2.range2 = ::core::ptr::null_mut::<cff_CharsetRangeFormat2>();
            }
        }
        1 | 2 | 0 | _ => {}
    };
}
