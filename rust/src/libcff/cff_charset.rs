#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{Buffer};
use crate::support::buffer::{bufnew};

/// The Top DICT's Charset offset is overloaded by spec: values 0/1/2 select
/// one of the three predefined charsets (ISOAdobe/Expert/ExpertSubset)
/// outright, and any other value is a real offset into an embedded charset
/// table. These three constants are the spec's special-cased offset values
/// `cff_extract_charset` compares against before treating an offset as real
/// -- the same role `CFF_STANDARD_ENCODING_OFFSET`/`CFF_EXPERT_ENCODING_
/// OFFSET` play for `CffEncoding`.
const CFF_CHARSET_OFFSET_ISO_ADOBE: i32 = 0;
const CFF_CHARSET_OFFSET_EXPERT: i32 = 1;
const CFF_CHARSET_OFFSET_EXPERT_SUBSET: i32 = 2;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffCharsetRangeFormat1 {
    pub first: u16,
    pub nleft: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffCharsetRangeFormat2 {
    pub first: u16,
    pub nleft: u16,
}
/// Was a `t: CffCharsetType` discriminant plus a `c2rust_unnamed:
/// CffCharsetBody` union (`f0`/`f1`/`f2`, one raw-pointer array each) -- the
/// same shape `CffEncoding` had, and the same fix: a single enum,
/// discriminant and payload together. `s` (the entry count) is gone too --
/// it was write-only (set once while parsing or building, never read again
/// anywhere in the crate) and exactly duplicated each `Vec`'s own `.len()`.
#[derive(Clone)]
pub enum CffCharset {
    IsoAdobe,
    Expert,
    ExpertSubset,
    Format0(Vec<u16>),
    Format1(Vec<CffCharsetRangeFormat1>),
    Format2(Vec<CffCharsetRangeFormat2>),
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
// Returns `CffCharset` by value instead of writing through a `*mut
// CffCharset` out-param -- the same "unwrap_X_table" shape used throughout
// this migration.
pub unsafe fn cff_extract_charset(data: *mut u8, offset: i32, nchars: u16) -> CffCharset {
    if offset == CFF_CHARSET_OFFSET_ISO_ADOBE {
        return CffCharset::IsoAdobe;
    } else if offset == CFF_CHARSET_OFFSET_EXPERT {
        return CffCharset::Expert;
    } else if offset == CFF_CHARSET_OFFSET_EXPERT_SUBSET {
        return CffCharset::ExpertSubset;
    }
    match *data.offset(offset as isize) as ::core::ffi::c_int {
        0 => {
            let count = (nchars as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u32;
            let mut glyph: Vec<u16> = Vec::with_capacity(count as usize);
            let mut i: u32 = 0 as u32;
            while i < count {
                glyph.push(gu2(
                    data,
                    ((offset + 1 as i32) as u32).wrapping_add(i.wrapping_mul(2 as u32)),
                ) as u16);
                i = i.wrapping_add(1);
            }
            CffCharset::Format0(glyph)
        }
        1 => {
            let mut size: u32 = 0;
            let mut glyphs_encoded_sofar: u32 = 1 as u32;
            let mut i: u32 = 0 as u32;
            while glyphs_encoded_sofar < nchars as u32 {
                glyphs_encoded_sofar = glyphs_encoded_sofar.wrapping_add(
                    (1 as u32).wrapping_add(gu1(
                        data,
                        ((offset + 3 as i32) as u32).wrapping_add(i.wrapping_mul(3 as u32)),
                    )),
                );
                i = i.wrapping_add(1);
            }
            size = i;
            let mut range1: Vec<CffCharsetRangeFormat1> = Vec::with_capacity(size as usize);
            i = 0 as u32;
            while i < size {
                let first = gu2(
                    data,
                    ((offset + 1 as i32) as u32).wrapping_add(i.wrapping_mul(3 as u32)),
                ) as u16;
                let nleft = gu1(
                    data,
                    ((offset + 3 as i32) as u32).wrapping_add(i.wrapping_mul(3 as u32)),
                ) as u8;
                range1.push(CffCharsetRangeFormat1 { first, nleft });
                i = i.wrapping_add(1);
            }
            CffCharset::Format1(range1)
        }
        2 => {
            let mut size_0: u32 = 0;
            let mut glyphs_encoded_sofar_0: u32 = 1 as u32;
            let mut i: u32 = 0 as u32;
            while glyphs_encoded_sofar_0 < nchars as u32 {
                glyphs_encoded_sofar_0 = glyphs_encoded_sofar_0.wrapping_add(
                    (1 as u32).wrapping_add(gu2(
                        data,
                        ((offset + 3 as i32) as u32).wrapping_add(i.wrapping_mul(4 as u32)),
                    )),
                );
                i = i.wrapping_add(1);
            }
            size_0 = i;
            let mut range2: Vec<CffCharsetRangeFormat2> = Vec::with_capacity(size_0 as usize);
            i = 0 as u32;
            while i < size_0 {
                let first = gu2(
                    data,
                    ((offset + 1 as i32) as u32).wrapping_add(i.wrapping_mul(4 as u32)),
                ) as u16;
                let nleft = gu2(
                    data,
                    ((offset + 3 as i32) as u32).wrapping_add(i.wrapping_mul(4 as u32)),
                ) as u16;
                range2.push(CffCharsetRangeFormat2 { first, nleft });
                i = i.wrapping_add(1);
            }
            CffCharset::Format2(range2)
        }
        _ => CffCharset::IsoAdobe,
    }
}
// Takes `&CffCharset` instead of by value -- it only ever reads the data,
// same reasoning as every other builder function in this migration that
// doesn't need ownership.
pub unsafe fn cff_build_charset(cset: &CffCharset) -> *mut Buffer {
    match cset {
        CffCharset::IsoAdobe | CffCharset::Expert | CffCharset::ExpertSubset => bufnew(),
        CffCharset::Format0(glyph) => {
            let blob: *mut Buffer = bufnew();
            (*blob).size = (1 as u32).wrapping_add((glyph.len() as u32).wrapping_mul(2 as u32)) as usize;
            (*blob).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob).size),
                75 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob).data.offset(0 as ::core::ffi::c_int as isize) = 0 as u8;
            for (i, &g) in glyph.iter().enumerate() {
                let i = i as u32;
                *(*blob).data.offset((1 as u32).wrapping_add((2 as u32).wrapping_mul(i)) as isize) =
                    (g as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8;
                *(*blob).data.offset((2 as u32).wrapping_add((2 as u32).wrapping_mul(i)) as isize) =
                    (g as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8;
            }
            (*blob).cursor = (*blob).size;
            blob
        }
        CffCharset::Format1(range1) => {
            let blob_0: *mut Buffer = bufnew();
            (*blob_0).size = (1 as u32).wrapping_add((range1.len() as u32).wrapping_mul(3 as u32)) as usize;
            (*blob_0).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob_0).size),
                85 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob_0).data.offset(0 as ::core::ffi::c_int as isize) = 1 as u8;
            for (i, r) in range1.iter().enumerate() {
                let i = i as u32;
                *(*blob_0).data.offset((1 as u32).wrapping_add((3 as u32).wrapping_mul(i)) as isize) =
                    (r.first as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8;
                *(*blob_0).data.offset((2 as u32).wrapping_add((3 as u32).wrapping_mul(i)) as isize) =
                    (r.first as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8;
                *(*blob_0).data.offset((3 as u32).wrapping_add((3 as u32).wrapping_mul(i)) as isize) =
                    r.nleft;
            }
            blob_0
        }
        CffCharset::Format2(range2) => {
            let blob_1: *mut Buffer = bufnew();
            (*blob_1).size = (1 as u32).wrapping_add((range2.len() as u32).wrapping_mul(4 as u32)) as usize;
            (*blob_1).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob_1).size),
                96 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob_1).data.offset(0 as ::core::ffi::c_int as isize) = 2 as u8;
            for (i, r) in range2.iter().enumerate() {
                let i = i as u32;
                *(*blob_1).data.offset((1 as u32).wrapping_add((4 as u32).wrapping_mul(i)) as isize) =
                    (r.first as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8;
                *(*blob_1).data.offset((2 as u32).wrapping_add((4 as u32).wrapping_mul(i)) as isize) =
                    (r.first as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8;
                *(*blob_1).data.offset((3 as u32).wrapping_add((4 as u32).wrapping_mul(i)) as isize) =
                    (r.nleft as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8;
                *(*blob_1).data.offset((4 as u32).wrapping_add((4 as u32).wrapping_mul(i)) as isize) =
                    (r.nleft as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8;
            }
            (*blob_1).cursor = (*blob_1).size;
            blob_1
        }
    }
}
