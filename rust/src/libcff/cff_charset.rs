#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::alloc::__caryll_allocate_clean;
use crate::support::buffer::Buffer;
use crate::support::buffer::bufnew;
use crate::support::font_reader::FontReader;

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
// `gu1`/`gu2` (no bounds checking, no length parameter at all) are gone --
// see `libcff/cff_index.rs`'s own conversion for the same move.
//
// Returns `CffCharset` by value instead of writing through a `*mut
// CffCharset` out-param -- the same "unwrap_X_table" shape used throughout
// this migration.
//
// The original had no bounds checking anywhere in this function -- not on
// `offset` itself (a negative value, reachable from a malformed DICT key,
// moved the read pointer *before* the buffer via `.offset()`), not on any
// of the three formats' arrays. Format0's `count` computation had the
// same wraparound-to-huge-allocation shape `cff_index.rs`'s 4GB `memcpy`
// bug had: `nchars as c_int - 1` for `nchars == 0` went negative in `c_int`
// arithmetic and was then cast straight to `u32`, producing `0xFFFFFFFF`
// and an immediate `Vec::with_capacity` abort. `.saturating_sub(1)` closes
// that; every other read goes through `FontReader`, checked against
// `table_length`. On any bounds failure, or a negative `offset`, this
// falls back to `IsoAdobe` -- the same fallback the original already used
// for an unrecognized format byte, just extended to cover "malformed"
// too, since the original drew no distinction between the two.
pub unsafe fn cff_extract_charset(
    data: *mut u8,
    table_length: u32,
    offset: i32,
    nchars: u16,
) -> CffCharset {
    if offset == CFF_CHARSET_OFFSET_ISO_ADOBE {
        return CffCharset::IsoAdobe;
    } else if offset == CFF_CHARSET_OFFSET_EXPERT {
        return CffCharset::Expert;
    } else if offset == CFF_CHARSET_OFFSET_EXPERT_SUBSET {
        return CffCharset::ExpertSubset;
    }
    if offset < 0 {
        return CffCharset::IsoAdobe;
    }
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let offset = offset as usize;
    let result: Option<CffCharset> = 'parse: {
        let Ok(mut r) = FontReader::new(slice).at(offset) else {
            break 'parse None;
        };
        let Ok(format) = r.u8() else {
            break 'parse None;
        };
        match format {
            0 => {
                let count = (nchars as usize).saturating_sub(1);
                let mut glyph: Vec<u16> = Vec::with_capacity(count);
                let Ok(mut r2) = FontReader::new(slice).at(offset + 1) else {
                    break 'parse None;
                };
                for _ in 0..count {
                    let Ok(v) = r2.u16() else { break 'parse None };
                    glyph.push(v);
                }
                break 'parse Some(CffCharset::Format0(glyph));
            }
            1 => {
                let mut size: usize = 0;
                let mut glyphs_encoded_sofar: u32 = 1 as u32;
                while glyphs_encoded_sofar < nchars as u32 {
                    let Ok(nleft) = FontReader::new(slice)
                        .at(offset + 3 + 3 * size)
                        .and_then(|mut x| x.u8())
                    else {
                        break 'parse None;
                    };
                    glyphs_encoded_sofar = glyphs_encoded_sofar.saturating_add(1 + nleft as u32);
                    size += 1;
                }
                let mut range1: Vec<CffCharsetRangeFormat1> = Vec::with_capacity(size);
                let Ok(mut r2) = FontReader::new(slice).at(offset + 1) else {
                    break 'parse None;
                };
                for _ in 0..size {
                    let Ok(first) = r2.u16() else {
                        break 'parse None;
                    };
                    let Ok(nleft) = r2.u8() else {
                        break 'parse None;
                    };
                    range1.push(CffCharsetRangeFormat1 { first, nleft });
                }
                break 'parse Some(CffCharset::Format1(range1));
            }
            2 => {
                let mut size: usize = 0;
                let mut glyphs_encoded_sofar: u32 = 1 as u32;
                while glyphs_encoded_sofar < nchars as u32 {
                    let Ok(nleft) = FontReader::new(slice)
                        .at(offset + 3 + 4 * size)
                        .and_then(|mut x| x.u16())
                    else {
                        break 'parse None;
                    };
                    glyphs_encoded_sofar = glyphs_encoded_sofar.saturating_add(1 + nleft as u32);
                    size += 1;
                }
                let mut range2: Vec<CffCharsetRangeFormat2> = Vec::with_capacity(size);
                let Ok(mut r2) = FontReader::new(slice).at(offset + 1) else {
                    break 'parse None;
                };
                for _ in 0..size {
                    let Ok(first) = r2.u16() else {
                        break 'parse None;
                    };
                    let Ok(nleft) = r2.u16() else {
                        break 'parse None;
                    };
                    range2.push(CffCharsetRangeFormat2 { first, nleft });
                }
                break 'parse Some(CffCharset::Format2(range2));
            }
            _ => break 'parse Some(CffCharset::IsoAdobe),
        }
    };
    result.unwrap_or(CffCharset::IsoAdobe)
}
// Takes `&CffCharset` instead of by value -- it only ever reads the data,
// same reasoning as every other builder function in this migration that
// doesn't need ownership.
pub unsafe fn cff_build_charset(cset: &CffCharset) -> *mut Buffer {
    match cset {
        CffCharset::IsoAdobe | CffCharset::Expert | CffCharset::ExpertSubset => bufnew(),
        CffCharset::Format0(glyph) => {
            let blob: *mut Buffer = bufnew();
            (*blob).size =
                (1 as u32).wrapping_add((glyph.len() as u32).wrapping_mul(2 as u32)) as usize;
            (*blob).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob).size),
                75 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob).data.offset(0 as ::core::ffi::c_int as isize) = 0 as u8;
            for (i, &g) in glyph.iter().enumerate() {
                let i = i as u32;
                *(*blob)
                    .data
                    .offset((1 as u32).wrapping_add((2 as u32).wrapping_mul(i)) as isize) =
                    (g as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8;
                *(*blob)
                    .data
                    .offset((2 as u32).wrapping_add((2 as u32).wrapping_mul(i)) as isize) =
                    (g as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8;
            }
            (*blob).cursor = (*blob).size;
            blob
        }
        CffCharset::Format1(range1) => {
            let blob_0: *mut Buffer = bufnew();
            (*blob_0).size =
                (1 as u32).wrapping_add((range1.len() as u32).wrapping_mul(3 as u32)) as usize;
            (*blob_0).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob_0).size),
                85 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob_0).data.offset(0 as ::core::ffi::c_int as isize) = 1 as u8;
            for (i, r) in range1.iter().enumerate() {
                let i = i as u32;
                *(*blob_0)
                    .data
                    .offset((1 as u32).wrapping_add((3 as u32).wrapping_mul(i)) as isize) =
                    (r.first as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8;
                *(*blob_0)
                    .data
                    .offset((2 as u32).wrapping_add((3 as u32).wrapping_mul(i)) as isize) =
                    (r.first as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8;
                *(*blob_0)
                    .data
                    .offset((3 as u32).wrapping_add((3 as u32).wrapping_mul(i)) as isize) = r.nleft;
            }
            blob_0
        }
        CffCharset::Format2(range2) => {
            let blob_1: *mut Buffer = bufnew();
            (*blob_1).size =
                (1 as u32).wrapping_add((range2.len() as u32).wrapping_mul(4 as u32)) as usize;
            (*blob_1).data = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob_1).size),
                96 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *(*blob_1).data.offset(0 as ::core::ffi::c_int as isize) = 2 as u8;
            for (i, r) in range2.iter().enumerate() {
                let i = i as u32;
                *(*blob_1)
                    .data
                    .offset((1 as u32).wrapping_add((4 as u32).wrapping_mul(i)) as isize) =
                    (r.first as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8;
                *(*blob_1)
                    .data
                    .offset((2 as u32).wrapping_add((4 as u32).wrapping_mul(i)) as isize) =
                    (r.first as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8;
                *(*blob_1)
                    .data
                    .offset((3 as u32).wrapping_add((4 as u32).wrapping_mul(i)) as isize) =
                    (r.nleft as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8;
                *(*blob_1)
                    .data
                    .offset((4 as u32).wrapping_add((4 as u32).wrapping_mul(i)) as isize) =
                    (r.nleft as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8;
            }
            (*blob_1).cursor = (*blob_1).size;
            blob_1
        }
    }
}

#[cfg(test)]
mod cff_extract_charset_tests {
    use super::*;

    #[test]
    fn format0_reads_the_glyph_id_array() {
        // offset=0/1/2 are reserved predefined-charset sentinels, so this
        // (and every other non-sentinel test below) starts the real data
        // at offset 3.
        let data = [0u8, 0, 0, 0x00, 0x00, 0x05, 0x00, 0x0A]; // format=0, glyphs=[5,10]
        unsafe {
            let CffCharset::Format0(glyph) =
                cff_extract_charset(data.as_ptr() as *mut u8, data.len() as u32, 3, 3)
            else {
                panic!("expected Format0");
            };
            assert_eq!(glyph, vec![5, 10]);
        }
    }

    #[test]
    fn format0_nchars_zero_does_not_attempt_a_huge_allocation() {
        // The original computed `count` as `nchars as c_int - 1` then cast
        // straight to `u32`; for `nchars == 0` that wrapped to
        // `0xFFFFFFFF` and `Vec::with_capacity` aborted immediately.
        let data = [0u8, 0, 0, 0x00];
        unsafe {
            let CffCharset::Format0(glyph) =
                cff_extract_charset(data.as_ptr() as *mut u8, data.len() as u32, 3, 0)
            else {
                panic!("expected Format0");
            };
            assert!(glyph.is_empty());
        }
    }

    #[test]
    fn format0_truncated_glyph_array_falls_back_to_iso_adobe_instead_of_reading_oob() {
        let data = [0u8, 0, 0, 0x00, 0x00, 0x05]; // format=0, one glyph's worth of bytes, but 2 needed
        unsafe {
            let result = cff_extract_charset(data.as_ptr() as *mut u8, data.len() as u32, 3, 3);
            assert!(matches!(result, CffCharset::IsoAdobe));
        }
    }

    #[test]
    fn negative_offset_falls_back_to_iso_adobe_instead_of_reading_before_the_buffer() {
        let data = [0u8; 8];
        unsafe {
            let result = cff_extract_charset(data.as_ptr() as *mut u8, data.len() as u32, -5, 10);
            assert!(matches!(result, CffCharset::IsoAdobe));
        }
    }

    #[test]
    fn format1_reads_a_single_range_covering_the_remaining_glyphs() {
        // nchars=3: glyph 0 is implicit, one range covering 2 more
        // glyphs (first=100, nleft=1 -> glyphs 100,101) exactly reaches
        // glyphs_encoded_sofar == nchars.
        let data = [0u8, 0, 0, 0x01, 0x00, 0x64, 0x01];
        unsafe {
            let CffCharset::Format1(range1) =
                cff_extract_charset(data.as_ptr() as *mut u8, data.len() as u32, 3, 3)
            else {
                panic!("expected Format1");
            };
            assert_eq!(range1.len(), 1);
            assert_eq!(range1[0].first, 100);
            assert_eq!(range1[0].nleft, 1);
        }
    }

    #[test]
    fn format1_truncated_during_the_counting_pass_falls_back_to_iso_adobe() {
        let data = [0u8, 0, 0, 0x01]; // format=1, nothing else -- not even the first range's nleft
        unsafe {
            let result = cff_extract_charset(data.as_ptr() as *mut u8, data.len() as u32, 3, 3);
            assert!(matches!(result, CffCharset::IsoAdobe));
        }
    }
}
