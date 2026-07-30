#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{Buffer};
use crate::support::buffer::{bufnew};

/// Which charset a CFF font carries: one of the three predefined ones, or the
/// format of an embedded charset.
///
/// Like [`CffFdSelectType`](crate::libcff::cff_fdselect::CffFdSelectType),
/// this is the crate's own classification and not the byte from the file --
/// `cff_extract_Charset` reads the format and stores one of these.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CffCharsetType {
    IsoAdobe = 0,
    Expert = 1,
    ExpertSubset = 2,
    Format0 = 3,
    Format1 = 4,
    Format2 = 5,
}

/// "No charset given", which C spelled as a second name for 0 -- the CFF
/// default when a font's Top DICT has no charset entry *is* ISOAdobe, so the
/// two are the same state and only ever differ in what the reader was trying to
/// say. A const rather than a variant, since Rust cannot give one value two
/// variant names; it still works in a pattern.
pub const CFF_CHARSET_UNSPECED: CffCharsetType = CffCharsetType::IsoAdobe;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffCharsetFormat0 {
    pub format: u8,
    pub glyph: *mut u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffCharsetRangeFormat1 {
    pub first: u16,
    pub nleft: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffCharsetFormat1 {
    pub format: u8,
    pub range1: *mut CffCharsetRangeFormat1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffCharsetRangeFormat2 {
    pub first: u16,
    pub nleft: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffCharsetFormat2 {
    pub format: u8,
    pub range2: *mut CffCharsetRangeFormat2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffCharset {
    pub t: CffCharsetType,
    pub s: u32,
    pub c2rust_unnamed: CffCharsetBody,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CffCharsetBody {
    pub f0: CffCharsetFormat0,
    pub f1: CffCharsetFormat1,
    pub f2: CffCharsetFormat2,
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
pub unsafe extern "C" fn cff_extract_Charset(
    mut data: *mut u8,
    mut offset: i32,
    mut nchars: u16,
    mut charsets: *mut CffCharset,
) {
    let mut i: u32 = 0;
    if offset == CffCharsetType::IsoAdobe as ::core::ffi::c_int as i32 {
        (*charsets).t = CffCharsetType::IsoAdobe;
    } else if offset == CffCharsetType::Expert as ::core::ffi::c_int as i32 {
        (*charsets).t = CffCharsetType::Expert;
    } else if offset == CffCharsetType::ExpertSubset as ::core::ffi::c_int as i32 {
        (*charsets).t = CffCharsetType::ExpertSubset;
    } else {
        match *data.offset(offset as isize) as ::core::ffi::c_int {
            0 => {
                (*charsets).t = CffCharsetType::Format0;
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
                (*charsets).t = CffCharsetType::Format1;
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
                    (::core::mem::size_of::<CffCharsetRangeFormat1>() as usize)
                        .wrapping_mul(i.wrapping_add(1 as u32) as usize),
                    35 as ::core::ffi::c_ulong,
                )
                    as *mut CffCharsetRangeFormat1;
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
                (*charsets).t = CffCharsetType::Format2;
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
                    (::core::mem::size_of::<CffCharsetRangeFormat2>() as usize)
                        .wrapping_mul(i.wrapping_add(1 as u32) as usize),
                    53 as ::core::ffi::c_ulong,
                )
                    as *mut CffCharsetRangeFormat2;
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
pub unsafe extern "C" fn cff_build_Charset(mut cset: CffCharset) -> *mut Buffer {
    match cset.t {
        CffCharsetType::IsoAdobe | CffCharsetType::Expert | CffCharsetType::ExpertSubset => return bufnew(),
        CffCharsetType::Format0 => {
            let mut blob: *mut Buffer = bufnew();
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
        CffCharsetType::Format1 => {
            let mut blob_0: *mut Buffer = bufnew();
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
        CffCharsetType::Format2 => {
            let mut blob_1: *mut Buffer = bufnew();
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
    }
}
pub unsafe extern "C" fn cff_close_Charset(mut cset: CffCharset) {
    match cset.t {
        CffCharsetType::Format0 => {
            if !cset.c2rust_unnamed.f0.glyph.is_null() {
                free(cset.c2rust_unnamed.f0.glyph as *mut ::core::ffi::c_void);
                cset.c2rust_unnamed.f0.glyph = ::core::ptr::null_mut::<u16>();
            }
        }
        CffCharsetType::Format1 => {
            if !cset.c2rust_unnamed.f1.range1.is_null() {
                free(cset.c2rust_unnamed.f1.range1 as *mut ::core::ffi::c_void);
                cset.c2rust_unnamed.f1.range1 = ::core::ptr::null_mut::<CffCharsetRangeFormat1>();
            }
        }
        CffCharsetType::Format2 => {
            if !cset.c2rust_unnamed.f2.range2.is_null() {
                free(cset.c2rust_unnamed.f2.range2 as *mut ::core::ffi::c_void);
                cset.c2rust_unnamed.f2.range2 = ::core::ptr::null_mut::<CffCharsetRangeFormat2>();
            }
        }
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // `CFF_CHARSET_UNSPECED` and `CffCharsetType::IsoAdobe` are the same state, not
    // two states that happen to share a number: a CFF font whose Top DICT has no
    // charset entry *is* ISOAdobe by the spec's default, and otfcc uses whichever
    // name reads better at each site (`cff_close_CFF` says UNSPECED, the reader
    // says ISOADOBE). Rust cannot give one value two variant names, so one of
    // them is a const -- and this pins that they stay interchangeable, including
    // in patterns, which is what the reader and the builder rely on.
    #[test]
    fn unspeced_and_isoadobe_are_one_state() {
        assert_eq!(CFF_CHARSET_UNSPECED, CffCharsetType::IsoAdobe);
        assert_eq!(CffCharsetType::IsoAdobe as u32, 0);
        assert!(matches!(CFF_CHARSET_UNSPECED, CffCharsetType::IsoAdobe));
        // A `CffCharset` arrives from `__caryll_allocate_clean`, so all-zero has
        // to be a legal value of the field.
        assert_eq!(::core::mem::size_of::<CffCharsetType>(), 4);
    }
}
