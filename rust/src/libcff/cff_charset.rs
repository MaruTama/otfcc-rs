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
    pub format: uint8_t,
    pub glyph: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetRangeFormat1 {
    pub first: uint16_t,
    pub nleft: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat1 {
    pub format: uint8_t,
    pub range1: *mut cff_CharsetRangeFormat1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetRangeFormat2 {
    pub first: uint16_t,
    pub nleft: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharsetFormat2 {
    pub format: uint8_t,
    pub range2: *mut cff_CharsetRangeFormat2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Charset {
    pub t: uint32_t,
    pub s: uint32_t,
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
pub unsafe extern "C" fn cff_extract_Charset(
    mut data: *mut uint8_t,
    mut offset: int32_t,
    mut nchars: uint16_t,
    mut charsets: *mut cff_Charset,
) {
    let mut i: uint32_t = 0;
    if offset == cff_CHARSET_ISOADOBE as ::core::ffi::c_int as int32_t {
        (*charsets).t = cff_CHARSET_ISOADOBE as ::core::ffi::c_int as uint32_t;
    } else if offset == cff_CHARSET_EXPERT as ::core::ffi::c_int as int32_t {
        (*charsets).t = cff_CHARSET_EXPERT as ::core::ffi::c_int as uint32_t;
    } else if offset == cff_CHARSET_EXPERTSUBSET as ::core::ffi::c_int as int32_t {
        (*charsets).t = cff_CHARSET_EXPERTSUBSET as ::core::ffi::c_int as uint32_t;
    } else {
        match *data.offset(offset as isize) as ::core::ffi::c_int {
            0 => {
                (*charsets).t = cff_CHARSET_FORMAT0 as ::core::ffi::c_int as uint32_t;
                (*charsets).s =
                    (nchars as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint32_t;
                (*charsets).c2rust_unnamed.f0.glyph = __caryll_allocate_clean(
                    (::core::mem::size_of::<uint16_t>() as size_t).wrapping_mul(
                        (nchars as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t,
                    ),
                    18 as ::core::ffi::c_ulong,
                ) as *mut uint16_t;
                i = 0 as uint32_t;
                while i < (*charsets).s {
                    *(*charsets).c2rust_unnamed.f0.glyph.offset(i as isize) = gu2(
                        data,
                        ((offset + 1 as int32_t) as uint32_t)
                            .wrapping_add(i.wrapping_mul(2 as uint32_t)),
                    )
                        as uint16_t;
                    i = i.wrapping_add(1);
                }
            }
            1 => {
                (*charsets).t = cff_CHARSET_FORMAT1 as ::core::ffi::c_int as uint32_t;
                let mut size: uint32_t = 0;
                let mut glyphsEncodedSofar: uint32_t = 1 as uint32_t;
                i = 0 as uint32_t;
                while glyphsEncodedSofar < nchars as uint32_t {
                    glyphsEncodedSofar = glyphsEncodedSofar.wrapping_add(
                        (1 as uint32_t).wrapping_add(gu1(
                            data,
                            ((offset + 3 as int32_t) as uint32_t)
                                .wrapping_add(i.wrapping_mul(3 as uint32_t)),
                        )),
                    );
                    i = i.wrapping_add(1);
                }
                size = i;
                (*charsets).s = size;
                (*charsets).c2rust_unnamed.f1.range1 = __caryll_allocate_clean(
                    (::core::mem::size_of::<cff_CharsetRangeFormat1>() as size_t)
                        .wrapping_mul(i.wrapping_add(1 as uint32_t) as size_t),
                    35 as ::core::ffi::c_ulong,
                )
                    as *mut cff_CharsetRangeFormat1;
                i = 0 as uint32_t;
                while i < size {
                    (*(*charsets).c2rust_unnamed.f1.range1.offset(i as isize)).first = gu2(
                        data,
                        ((offset + 1 as int32_t) as uint32_t)
                            .wrapping_add(i.wrapping_mul(3 as uint32_t)),
                    )
                        as uint16_t;
                    (*(*charsets).c2rust_unnamed.f1.range1.offset(i as isize)).nleft = gu1(
                        data,
                        ((offset + 3 as int32_t) as uint32_t)
                            .wrapping_add(i.wrapping_mul(3 as uint32_t)),
                    )
                        as uint8_t;
                    i = i.wrapping_add(1);
                }
            }
            2 => {
                (*charsets).t = cff_CHARSET_FORMAT2 as ::core::ffi::c_int as uint32_t;
                let mut size_0: uint32_t = 0;
                let mut glyphsEncodedSofar_0: uint32_t = 1 as uint32_t;
                i = 0 as uint32_t;
                while glyphsEncodedSofar_0 < nchars as uint32_t {
                    glyphsEncodedSofar_0 = glyphsEncodedSofar_0.wrapping_add(
                        (1 as uint32_t).wrapping_add(gu2(
                            data,
                            ((offset + 3 as int32_t) as uint32_t)
                                .wrapping_add(i.wrapping_mul(4 as uint32_t)),
                        )),
                    );
                    i = i.wrapping_add(1);
                }
                size_0 = i;
                (*charsets).s = size_0;
                (*charsets).c2rust_unnamed.f2.range2 = __caryll_allocate_clean(
                    (::core::mem::size_of::<cff_CharsetRangeFormat2>() as size_t)
                        .wrapping_mul(i.wrapping_add(1 as uint32_t) as size_t),
                    53 as ::core::ffi::c_ulong,
                )
                    as *mut cff_CharsetRangeFormat2;
                i = 0 as uint32_t;
                while i < size_0 {
                    (*(*charsets).c2rust_unnamed.f2.range2.offset(i as isize)).first = gu2(
                        data,
                        ((offset + 1 as int32_t) as uint32_t)
                            .wrapping_add(i.wrapping_mul(4 as uint32_t)),
                    )
                        as uint16_t;
                    (*(*charsets).c2rust_unnamed.f2.range2.offset(i as isize)).nleft = gu2(
                        data,
                        ((offset + 3 as int32_t) as uint32_t)
                            .wrapping_add(i.wrapping_mul(4 as uint32_t)),
                    )
                        as uint16_t;
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
                (1 as uint32_t).wrapping_add(cset.s.wrapping_mul(2 as uint32_t)) as size_t;
            (*blob).data = __caryll_allocate_clean(
                (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*blob).size),
                75 as ::core::ffi::c_ulong,
            ) as *mut uint8_t;
            *(*blob).data.offset(0 as ::core::ffi::c_int as isize) = 0 as uint8_t;
            let mut i: uint32_t = 0 as uint32_t;
            while i < cset.s {
                *(*blob).data.offset(
                    (1 as uint32_t).wrapping_add((2 as uint32_t).wrapping_mul(i)) as isize,
                ) = (*cset.c2rust_unnamed.f0.glyph.offset(i as isize) as ::core::ffi::c_int
                    / 256 as ::core::ffi::c_int) as uint8_t;
                *(*blob).data.offset(
                    (2 as uint32_t).wrapping_add((2 as uint32_t).wrapping_mul(i)) as isize,
                ) = (*cset.c2rust_unnamed.f0.glyph.offset(i as isize) as ::core::ffi::c_int
                    % 256 as ::core::ffi::c_int) as uint8_t;
                i = i.wrapping_add(1);
            }
            (*blob).cursor = (*blob).size;
            return blob;
        }
        4 => {
            let mut blob_0: *mut caryll_Buffer = bufnew();
            (*blob_0).size =
                (1 as uint32_t).wrapping_add(cset.s.wrapping_mul(3 as uint32_t)) as size_t;
            (*blob_0).data = __caryll_allocate_clean(
                (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*blob_0).size),
                85 as ::core::ffi::c_ulong,
            ) as *mut uint8_t;
            *(*blob_0).data.offset(0 as ::core::ffi::c_int as isize) = 1 as uint8_t;
            let mut i_0: uint32_t = 0 as uint32_t;
            while i_0 < cset.s {
                *(*blob_0).data.offset(
                    (1 as uint32_t).wrapping_add((3 as uint32_t).wrapping_mul(i_0)) as isize,
                ) = ((*cset.c2rust_unnamed.f1.range1.offset(i_0 as isize)).first
                    as ::core::ffi::c_int
                    / 256 as ::core::ffi::c_int) as uint8_t;
                *(*blob_0).data.offset(
                    (2 as uint32_t).wrapping_add((3 as uint32_t).wrapping_mul(i_0)) as isize,
                ) = ((*cset.c2rust_unnamed.f1.range1.offset(i_0 as isize)).first
                    as ::core::ffi::c_int
                    % 256 as ::core::ffi::c_int) as uint8_t;
                *(*blob_0).data.offset(
                    (3 as uint32_t).wrapping_add((3 as uint32_t).wrapping_mul(i_0)) as isize,
                ) = (*cset.c2rust_unnamed.f1.range1.offset(i_0 as isize)).nleft;
                i_0 = i_0.wrapping_add(1);
            }
            return blob_0;
        }
        5 => {
            let mut blob_1: *mut caryll_Buffer = bufnew();
            (*blob_1).size =
                (1 as uint32_t).wrapping_add(cset.s.wrapping_mul(4 as uint32_t)) as size_t;
            (*blob_1).data = __caryll_allocate_clean(
                (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*blob_1).size),
                96 as ::core::ffi::c_ulong,
            ) as *mut uint8_t;
            *(*blob_1).data.offset(0 as ::core::ffi::c_int as isize) = 2 as uint8_t;
            let mut i_1: uint32_t = 0 as uint32_t;
            while i_1 < cset.s {
                *(*blob_1).data.offset(
                    (1 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(i_1)) as isize,
                ) = ((*cset.c2rust_unnamed.f2.range2.offset(i_1 as isize)).first
                    as ::core::ffi::c_int
                    / 256 as ::core::ffi::c_int) as uint8_t;
                *(*blob_1).data.offset(
                    (2 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(i_1)) as isize,
                ) = ((*cset.c2rust_unnamed.f2.range2.offset(i_1 as isize)).first
                    as ::core::ffi::c_int
                    % 256 as ::core::ffi::c_int) as uint8_t;
                *(*blob_1).data.offset(
                    (3 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(i_1)) as isize,
                ) = ((*cset.c2rust_unnamed.f2.range2.offset(i_1 as isize)).nleft
                    as ::core::ffi::c_int
                    / 256 as ::core::ffi::c_int) as uint8_t;
                *(*blob_1).data.offset(
                    (4 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(i_1)) as isize,
                ) = ((*cset.c2rust_unnamed.f2.range2.offset(i_1 as isize)).nleft
                    as ::core::ffi::c_int
                    % 256 as ::core::ffi::c_int) as uint8_t;
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
                cset.c2rust_unnamed.f0.glyph = ::core::ptr::null_mut::<uint16_t>();
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
