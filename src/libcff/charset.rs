use otfcc_binary::Buffer;
use otfcc_binary::{FontReader, ReadError};

/// The Top DICT's Charset offset is overloaded by spec: values 0/1/2 select
/// one of the three predefined charsets (ISOAdobe/Expert/ExpertSubset)
/// outright, and any other value is a real offset into an embedded charset
/// table. These three constants are the spec's special-cased offset values
/// `extract_charset` compares against before treating an offset as real
/// -- the same role `CFF_STANDARD_ENCODING_OFFSET`/`CFF_EXPERT_ENCODING_
/// OFFSET` play for `CffEncoding`.
const CFF_CHARSET_OFFSET_ISO_ADOBE: i32 = 0;
const CFF_CHARSET_OFFSET_EXPERT: i32 = 1;
const CFF_CHARSET_OFFSET_EXPERT_SUBSET: i32 = 2;

#[derive(Copy, Clone, Debug)]
pub struct CffCharsetRangeFormat1 {
    pub first: u16,
    pub nleft: u8,
}
#[derive(Copy, Clone, Debug)]
pub struct CffCharsetRangeFormat2 {
    pub first: u16,
    pub nleft: u16,
}
/// A CFF charset in one of its three formats.
#[derive(Clone, Debug)]
pub enum CffCharset {
    IsoAdobe,
    Expert,
    ExpertSubset,
    Format0(Vec<u16>),
    Format1(Vec<CffCharsetRangeFormat1>),
    Format2(Vec<CffCharsetRangeFormat2>),
}
// Every read goes through `FontReader`, checked against the slice; on any
// bounds failure, a negative `offset` (reachable from a malformed DICT
// key), or an unrecognized format byte, this falls back to `IsoAdobe`.
// Format 0's entry count is `nchars - 1`, saturating so `nchars == 0`
// cannot wrap to a huge allocation.
pub fn extract_charset(slice: &[u8], offset: i32, nchars: u16) -> CffCharset {
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
    read_charset(slice, offset as usize, nchars).unwrap_or(CffCharset::IsoAdobe)
}
fn read_charset(slice: &[u8], offset: usize, nchars: u16) -> Result<CffCharset, ReadError> {
    let mut r = FontReader::new(slice).at(offset)?;
    match r.u8()? {
        0 => {
            let count = (nchars as usize).saturating_sub(1);
            let mut glyph: Vec<u16> = Vec::with_capacity(count);
            let mut r2 = FontReader::new(slice).at(offset + 1)?;
            for _ in 0..count {
                glyph.push(r2.u16()?);
            }
            Ok(CffCharset::Format0(glyph))
        }
        1 => {
            let size = count_ranges(slice, offset, nchars, 1)?;
            let mut range1: Vec<CffCharsetRangeFormat1> = Vec::with_capacity(size);
            let mut r2 = FontReader::new(slice).at(offset + 1)?;
            for _ in 0..size {
                let first = r2.u16()?;
                let nleft = r2.u8()?;
                range1.push(CffCharsetRangeFormat1 { first, nleft });
            }
            Ok(CffCharset::Format1(range1))
        }
        2 => {
            let size = count_ranges(slice, offset, nchars, 2)?;
            let mut range2: Vec<CffCharsetRangeFormat2> = Vec::with_capacity(size);
            let mut r2 = FontReader::new(slice).at(offset + 1)?;
            for _ in 0..size {
                let first = r2.u16()?;
                let nleft = r2.u16()?;
                range2.push(CffCharsetRangeFormat2 { first, nleft });
            }
            Ok(CffCharset::Format2(range2))
        }
        _ => Ok(CffCharset::IsoAdobe),
    }
}
/// How many ranges a format 1 or 2 charset needs to cover `nchars` glyphs
/// (glyph 0, `.notdef`, is implicit). Each range is a 2-byte first SID and an
/// `nleft` of `nleft_size` bytes (1 for format 1, 2 for format 2); this reads
/// each `nleft` until the glyphs are covered and fails if the data ends first.
fn count_ranges(
    slice: &[u8],
    offset: usize,
    nchars: u16,
    nleft_size: usize,
) -> Result<usize, ReadError> {
    let range_size = 2 + nleft_size;
    let mut size: usize = 0;
    let mut glyphs_encoded_sofar: u32 = 1;
    while glyphs_encoded_sofar < u32::from(nchars) {
        let mut r = FontReader::new(slice).at(offset + 3 + range_size * size)?;
        let nleft = if nleft_size == 1 {
            u16::from(r.u8()?)
        } else {
            r.u16()?
        };
        glyphs_encoded_sofar = glyphs_encoded_sofar.saturating_add(1 + u32::from(nleft));
        size += 1;
    }
    Ok(size)
}
// Takes `&CffCharset` instead of by value -- it only ever reads the data,
// same reasoning as every other builder function in this migration that
// doesn't need ownership.
pub fn build_charset(cset: &CffCharset) -> Buffer {
    match cset {
        CffCharset::IsoAdobe | CffCharset::Expert | CffCharset::ExpertSubset => Buffer::new(),
        CffCharset::Format0(glyph) => {
            let mut blob = Buffer::new();
            blob.write_u8(0);
            for &g in glyph.iter() {
                blob.write_u8((g / 256) as u8);
                blob.write_u8((g % 256) as u8);
            }
            blob
        }
        CffCharset::Format1(range1) => {
            let mut blob_0 = Buffer::new();
            blob_0.write_u8(1);
            for r in range1.iter() {
                blob_0.write_u8((r.first / 256) as u8);
                blob_0.write_u8((r.first % 256) as u8);
                blob_0.write_u8(r.nleft);
            }
            blob_0
        }
        CffCharset::Format2(range2) => {
            let mut blob_1 = Buffer::new();
            blob_1.write_u8(2);
            for r in range2.iter() {
                blob_1.write_u8((r.first / 256) as u8);
                blob_1.write_u8((r.first % 256) as u8);
                blob_1.write_u8((r.nleft / 256) as u8);
                blob_1.write_u8((r.nleft % 256) as u8);
            }
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
        let CffCharset::Format0(glyph) = extract_charset(&data, 3, 3) else {
            panic!("expected Format0");
        };
        assert_eq!(glyph, vec![5, 10]);
    }

    #[test]
    fn format0_nchars_zero_does_not_attempt_a_huge_allocation() {
        // `nchars - 1` for `nchars == 0` must not wrap to `0xFFFFFFFF` and
        // abort in `Vec::with_capacity`.
        let data = [0u8, 0, 0, 0x00];
        let CffCharset::Format0(glyph) = extract_charset(&data, 3, 0) else {
            panic!("expected Format0");
        };
        assert!(glyph.is_empty());
    }

    #[test]
    fn format0_truncated_glyph_array_falls_back_to_iso_adobe_instead_of_reading_oob() {
        let data = [0u8, 0, 0, 0x00, 0x00, 0x05]; // format=0, one glyph's worth of bytes, but 2 needed
        let result = extract_charset(&data, 3, 3);
        assert!(matches!(result, CffCharset::IsoAdobe));
    }

    #[test]
    fn negative_offset_falls_back_to_iso_adobe_instead_of_reading_before_the_buffer() {
        let data = [0u8; 8];
        let result = extract_charset(&data, -5, 10);
        assert!(matches!(result, CffCharset::IsoAdobe));
    }

    #[test]
    fn format1_reads_a_single_range_covering_the_remaining_glyphs() {
        // nchars=3: glyph 0 is implicit, one range covering 2 more
        // glyphs (first=100, nleft=1 -> glyphs 100,101) exactly reaches
        // glyphs_encoded_sofar == nchars.
        let data = [0u8, 0, 0, 0x01, 0x00, 0x64, 0x01];
        let CffCharset::Format1(range1) = extract_charset(&data, 3, 3) else {
            panic!("expected Format1");
        };
        assert_eq!(range1.len(), 1);
        assert_eq!(range1[0].first, 100);
        assert_eq!(range1[0].nleft, 1);
    }

    #[test]
    fn format1_truncated_during_the_counting_pass_falls_back_to_iso_adobe() {
        let data = [0u8, 0, 0, 0x01]; // format=1, nothing else -- not even the first range's nleft
        let result = extract_charset(&data, 3, 3);
        assert!(matches!(result, CffCharset::IsoAdobe));
    }

    #[test]
    fn format2_reads_a_single_range_with_a_two_byte_nleft() {
        // format=2 at offset 3: first=100, nleft=1 reaches nchars=3.
        let data = [0u8, 0, 0, 0x02, 0x00, 0x64, 0x00, 0x01];
        let CffCharset::Format2(range2) = extract_charset(&data, 3, 3) else {
            panic!("expected Format2");
        };
        assert_eq!(range2.len(), 1);
        assert_eq!(range2[0].first, 100);
        assert_eq!(range2[0].nleft, 1);
    }

    #[test]
    fn format2_truncated_during_the_counting_pass_falls_back_to_iso_adobe() {
        let data = [0u8, 0, 0, 0x02, 0x00, 0x64, 0x00]; // the second nleft byte is missing
        let result = extract_charset(&data, 3, 3);
        assert!(matches!(result, CffCharset::IsoAdobe));
    }
}
