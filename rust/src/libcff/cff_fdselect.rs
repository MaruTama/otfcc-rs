#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::buffer::Buffer;
use crate::support::buffer::{bufnew, bufwrite8};
use crate::support::font_reader::FontReader;

#[derive(Copy, Clone)]
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
    Format3 {
        range3: Vec<CffFdSelectRangeFormat3>,
        sentinel: u16,
    },
}
// `gu1`/`gu2` (no bounds checking, no length parameter at all) are gone --
// see `libcff/cff_index.rs`'s own conversion for the same move.
//
// Takes `&CffFdSelect` instead of by value -- it only ever reads the data to
// serialize it, same reasoning as `cff_build_charset`.
pub unsafe fn cff_build_fd_select(fd: &CffFdSelect) -> *mut Buffer {
    match fd {
        CffFdSelect::Unspecified => bufnew(),
        CffFdSelect::Format0(fds) => {
            let blob: *mut Buffer = bufnew();
            for &b in fds.iter() {
                bufwrite8(blob, b);
            }
            blob
        }
        CffFdSelect::Format3 { range3, sentinel } => {
            let blob_0: *mut Buffer = bufnew();
            let nranges = range3.len() as ::core::ffi::c_int;
            bufwrite8(blob_0, 3 as u8);
            bufwrite8(blob_0, (nranges / 256 as ::core::ffi::c_int) as u8);
            bufwrite8(blob_0, (nranges % 256 as ::core::ffi::c_int) as u8);
            for r in range3.iter() {
                bufwrite8(
                    blob_0,
                    (r.first as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8,
                );
                bufwrite8(
                    blob_0,
                    (r.first as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8,
                );
                bufwrite8(blob_0, r.fd);
            }
            bufwrite8(
                blob_0,
                (*sentinel as ::core::ffi::c_int / 256 as ::core::ffi::c_int) as u8,
            );
            bufwrite8(
                blob_0,
                (*sentinel as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as u8,
            );
            blob_0
        }
    }
}
// Returns `CffFdSelect` by value instead of writing through a `*mut
// CffFdSelect` out-param -- the same "unwrap_X_table" shape used throughout
// this migration.
//
// The original had no bounds checking anywhere here either: not on
// `offset` (a negative value, reachable from a malformed DICT key, moved
// the read pointer *before* the buffer), not on `nranges` (an attacker-
// controlled `u16` up to 65535, driving both a `Vec::with_capacity` and a
// read loop with no check that the table actually holds that many 3-byte
// range entries). Every read now goes through one sequential `FontReader`
// -- format0's array and format3's range array plus the sentinel that
// immediately follows it are laid out with no gaps, so a single reader
// walking forward covers the whole record. On any bounds failure, or a
// negative `offset`, this falls back to `Unspecified` -- the same
// fallback the original already used for an unrecognized format byte,
// just extended to cover "malformed" too.
pub unsafe fn cff_extract_fd_select(
    data: *mut u8,
    table_length: u32,
    offset: i32,
    nchars: u16,
) -> CffFdSelect {
    if offset < 0 {
        return CffFdSelect::Unspecified;
    }
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let result: Option<CffFdSelect> = 'parse: {
        let Ok(mut r) = FontReader::new(slice).at(offset as usize) else {
            break 'parse None;
        };
        let Ok(format) = r.u8() else {
            break 'parse None;
        };
        match format {
            0 => {
                let mut fds: Vec<u8> = Vec::with_capacity(nchars as usize);
                for _ in 0..nchars {
                    let Ok(v) = r.u8() else { break 'parse None };
                    fds.push(v);
                }
                break 'parse Some(CffFdSelect::Format0(fds));
            }
            3 => {
                let Ok(nranges) = r.u16() else {
                    break 'parse None;
                };
                let mut range3: Vec<CffFdSelectRangeFormat3> = Vec::with_capacity(nranges as usize);
                for _ in 0..nranges {
                    let Ok(first) = r.u16() else {
                        break 'parse None;
                    };
                    let Ok(fd) = r.u8() else { break 'parse None };
                    range3.push(CffFdSelectRangeFormat3 { first, fd });
                }
                let Ok(sentinel) = r.u16() else {
                    break 'parse None;
                };
                break 'parse Some(CffFdSelect::Format3 { range3, sentinel });
            }
            _ => break 'parse Some(CffFdSelect::Unspecified),
        }
    };
    result.unwrap_or(CffFdSelect::Unspecified)
}

#[cfg(test)]
mod cff_extract_fd_select_tests {
    use super::*;

    #[test]
    fn format0_reads_the_fd_array() {
        let data = [0x00u8, 3, 7]; // format=0, fds=[3,7]
        unsafe {
            let CffFdSelect::Format0(fds) =
                cff_extract_fd_select(data.as_ptr() as *mut u8, data.len() as u32, 0, 2)
            else {
                panic!("expected Format0");
            };
            assert_eq!(fds, vec![3, 7]);
        }
    }

    #[test]
    fn format0_truncated_fd_array_falls_back_to_unspecified_instead_of_reading_oob() {
        let data = [0x00u8, 3]; // format=0, one fd, but 2 declared
        unsafe {
            let result = cff_extract_fd_select(data.as_ptr() as *mut u8, data.len() as u32, 0, 2);
            assert!(matches!(result, CffFdSelect::Unspecified));
        }
    }

    #[test]
    fn format3_reads_ranges_and_the_trailing_sentinel() {
        let data = [0x03u8, 0x00, 0x01, 0x00, 0x05, 0x02, 0x00, 0x0A];
        unsafe {
            let CffFdSelect::Format3 { range3, sentinel } =
                cff_extract_fd_select(data.as_ptr() as *mut u8, data.len() as u32, 0, 0)
            else {
                panic!("expected Format3");
            };
            assert_eq!(range3.len(), 1);
            assert_eq!(range3[0].first, 5);
            assert_eq!(range3[0].fd, 2);
            assert_eq!(sentinel, 10);
        }
    }

    #[test]
    fn format3_huge_nranges_against_a_tiny_table_falls_back_to_unspecified_instead_of_reading_oob()
    {
        // The original had no check at all that the table actually held
        // `nranges` 3-byte entries.
        let data = [0x03u8, 0xFF, 0xFF]; // format=3, nranges=65535, nothing else
        unsafe {
            let result = cff_extract_fd_select(data.as_ptr() as *mut u8, data.len() as u32, 0, 0);
            assert!(matches!(result, CffFdSelect::Unspecified));
        }
    }

    #[test]
    fn negative_offset_falls_back_to_unspecified_instead_of_reading_before_the_buffer() {
        let data = [0u8; 8];
        unsafe {
            let result = cff_extract_fd_select(data.as_ptr() as *mut u8, data.len() as u32, -5, 10);
            assert!(matches!(result, CffFdSelect::Unspecified));
        }
    }
}
