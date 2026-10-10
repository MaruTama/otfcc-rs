
use otfcc_binary::Buffer;
use otfcc_binary::FontReader;

#[derive(Copy, Clone, Debug)]
pub struct CffFdSelectRangeFormat3 {
    pub first: u16,
    pub fd: u8,
}
/// A CFF FDSelect in format 0 or 3. `sentinel` is Format3's one-past-the-
/// last glyph index, which the final range extends to.
#[derive(Clone, Debug)]
pub enum CffFdSelect {
    Unspecified,
    Format0(Vec<u8>),
    Format3 {
        range3: Vec<CffFdSelectRangeFormat3>,
        sentinel: u16,
    },
}
// `gu1`/`gu2` (no bounds checking, no length parameter at all) are gone --
// see `libcff/index.rs`'s own conversion for the same move.
//
// Takes `&CffFdSelect` instead of by value -- it only ever reads the data to
// serialize it, same reasoning as `cff_build_charset`.
pub fn cff_build_fd_select(fd: &CffFdSelect) -> Buffer {
    match fd {
        CffFdSelect::Unspecified => Buffer::new(),
        CffFdSelect::Format0(fds) => {
            let mut blob = Buffer::new();
            for &b in fds.iter() {
                blob.write_u8(b);
            }
            blob
        }
        CffFdSelect::Format3 { range3, sentinel } => {
            let mut blob_0 = Buffer::new();
            let nranges = range3.len() as i32;
            blob_0.write_u8(3_u8);
            blob_0.write_u8((nranges / 256_i32) as u8);
            blob_0.write_u8((nranges % 256_i32) as u8);
            for r in range3.iter() {
                blob_0.write_u8((r.first as i32 / 256_i32) as u8);
                blob_0.write_u8((r.first as i32 % 256_i32) as u8);
                blob_0.write_u8(r.fd);
            }
            blob_0.write_u8((*sentinel as i32 / 256_i32) as u8);
            blob_0.write_u8((*sentinel as i32 % 256_i32) as u8);
            blob_0
        }
    }
}
// Every read goes through one sequential `FontReader` -- format0's array
// and format3's range array plus the sentinel that immediately follows it
// are laid out with no gaps, so a single reader walking forward covers the
// whole record. On any bounds failure, a negative `offset` (reachable from
// a malformed DICT key), or an unrecognized format byte, this falls back to
// `Unspecified`.
pub fn cff_extract_fd_select(slice: &[u8], offset: i32, nchars: u16) -> CffFdSelect {
    if offset < 0 {
        return CffFdSelect::Unspecified;
    }
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
        let CffFdSelect::Format0(fds) = cff_extract_fd_select(&data, 0, 2) else {
            panic!("expected Format0");
        };
        assert_eq!(fds, vec![3, 7]);
    }

    #[test]
    fn format0_truncated_fd_array_falls_back_to_unspecified_instead_of_reading_oob() {
        let data = [0x00u8, 3]; // format=0, one fd, but 2 declared
        let result = cff_extract_fd_select(&data, 0, 2);
        assert!(matches!(result, CffFdSelect::Unspecified));
    }

    #[test]
    fn format3_reads_ranges_and_the_trailing_sentinel() {
        let data = [0x03u8, 0x00, 0x01, 0x00, 0x05, 0x02, 0x00, 0x0A];
        let CffFdSelect::Format3 { range3, sentinel } = cff_extract_fd_select(&data, 0, 0) else {
            panic!("expected Format3");
        };
        assert_eq!(range3.len(), 1);
        assert_eq!(range3[0].first, 5);
        assert_eq!(range3[0].fd, 2);
        assert_eq!(sentinel, 10);
    }

    #[test]
    fn format3_huge_nranges_against_a_tiny_table_falls_back_to_unspecified_instead_of_reading_oob()
    {
        // The table must actually hold `nranges` 3-byte entries.
        let data = [0x03u8, 0xFF, 0xFF]; // format=3, nranges=65535, nothing else
        let result = cff_extract_fd_select(&data, 0, 0);
        assert!(matches!(result, CffFdSelect::Unspecified));
    }

    #[test]
    fn negative_offset_falls_back_to_unspecified_instead_of_reading_before_the_buffer() {
        let data = [0u8; 8];
        let result = cff_extract_fd_select(&data, -5, 10);
        assert!(matches!(result, CffFdSelect::Unspecified));
    }
}
