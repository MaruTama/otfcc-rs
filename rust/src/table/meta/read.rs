use crate::support::font_reader::{FontReader, ReadError};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, logger_log_sds};
use crate::support::options::{Options};
use crate::font::caryll_sfnt::{Packet};

use crate::table::meta::types::{MetaEntry, MetaTable};
// The original guarded the entry array with `table.length <
// 16.wrapping_add(12.wrapping_mul(data_maps_count))` -- a `data_maps_count`
// large enough to overflow `12 * count` (e.g. 0x1555_5556) wraps the sum
// back down to something small, so the guard passes even though the real
// entry array is nowhere near that short; the loop then read each entry's
// `tag`/`offset`/`length` straight past the table's actual end.
// `require_room` closes this the same way it does everywhere else in this
// stage: `checked_mul`/`checked_add`, so an overflowing count fails the
// guard instead of wrapping through it.
//
// Each entry's own data span (`offset..offset+length`) had the same
// wrapping-arithmetic gap (`table.length < offset.wrapping_add(length)`);
// `FontReader::sub`'s `checked_add` replaces it. Unlike the header guard,
// a single entry failing this check does not drop the whole table --
// matching the original, which silently skipped just that one entry and
// kept going.
fn parse_meta(data: &[u8]) -> Result<MetaTable, ReadError> {
    let mut r = FontReader::new(data);
    let version = r.u32()?;
    let flags = r.u32()?;
    r.skip(4)?; // reserved
    let data_maps_count = r.u32()? as usize;
    r.require_room(data_maps_count, 12)?;
    let mut entries = Vec::with_capacity(data_maps_count);
    for _ in 0..data_maps_count {
        let tag = r.u32()?;
        let offset = r.u32()?;
        let length = r.u32()?;
        if let Ok(bytes) = FontReader::new(data)
            .sub(offset as usize, length as usize)
            .and_then(|mut sr| sr.bytes(length as usize))
        {
            entries.push(MetaEntry { tag, data: bytes.to_vec() });
        }
    }
    Ok(MetaTable { version, flags, entries })
}
pub unsafe fn otfcc_read_meta(
    packet: &Packet,
    options: &Options,
) -> Option<Box<MetaTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_META)?;
    match parse_meta(&table.data) {
        Ok(meta) => Some(Box::new(meta)),
        Err(_) => {
            unsafe {
                logger_log_sds(
                    options.logger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(b"Table 'meta' corrupted.\n"),
                );
            }
            None
        }
    }
}

#[cfg(test)]
mod parse_meta_tests {
    use super::*;

    fn header(version: u32, flags: u32, data_maps_count: u32) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&version.to_be_bytes());
        b.extend_from_slice(&flags.to_be_bytes());
        b.extend_from_slice(&0u32.to_be_bytes()); // reserved
        b.extend_from_slice(&data_maps_count.to_be_bytes());
        b
    }

    #[test]
    fn well_formed_table_reads_one_entry() {
        let mut data = header(1, 0, 1);
        data.extend_from_slice(b"dlng"); // tag
        data.extend_from_slice(&28u32.to_be_bytes()); // offset: right after the 16-byte header + 12-byte entry
        data.extend_from_slice(&3u32.to_be_bytes()); // length
        data.extend_from_slice(b"en-US");
        let meta = parse_meta(&data).unwrap();
        assert_eq!(meta.entries.len(), 1);
        assert_eq!(meta.entries[0].data, b"en-".to_vec());
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(parse_meta(&header(1, 0, 0)[..10]).is_err());
    }

    #[test]
    fn data_maps_count_large_enough_to_overflow_the_multiplication_errs() {
        // 0x1555_5556 * 12 overflows u32/usize-on-32-bit math back to a
        // small number under wrapping arithmetic; `require_room`'s
        // `checked_mul` must reject this instead of wrapping through it.
        let data = header(1, 0, 0x1555_5556);
        assert!(parse_meta(&data).is_err());
    }

    #[test]
    fn entry_whose_span_overflows_offset_plus_length_is_dropped_not_the_whole_table() {
        let mut data = header(1, 0, 1);
        data.extend_from_slice(b"dlng");
        data.extend_from_slice(&0xFFFF_FFF0u32.to_be_bytes()); // offset
        data.extend_from_slice(&0x0000_0020u32.to_be_bytes()); // length; offset+length overflows u32
        let meta = parse_meta(&data).unwrap();
        assert!(meta.entries.is_empty());
    }

    #[test]
    fn entry_span_past_the_table_end_is_dropped_not_the_whole_table() {
        let mut data = header(1, 0, 1);
        data.extend_from_slice(b"dlng");
        data.extend_from_slice(&100u32.to_be_bytes()); // offset past the table end
        data.extend_from_slice(&3u32.to_be_bytes());
        let meta = parse_meta(&data).unwrap();
        assert!(meta.entries.is_empty());
    }
}
