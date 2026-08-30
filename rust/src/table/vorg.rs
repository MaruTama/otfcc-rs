#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::font::caryll_sfnt::Packet;
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::binio::pos_to_u16;
use crate::support::buffer::Buffer;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Pos};

#[derive(Copy, Clone)]
pub struct VorgEntry {
    pub gid: GlyphId,
    pub vertical_origin: i16,
}
pub struct VorgTable {
    pub num_vert_origin_y_metrics: GlyphId,
    pub default_vertical_origin: Pos,
    pub entries: Vec<VorgEntry>,
}
// Stage 6-4 "Box化" Box-ified the outer `VorgTable` itself (replacing the
// entire `VorgTableElementInterface` vtable); Stage 7-2-c "inner Vec化"
// finishes the job here: `entries` was the only allocation this struct
// owned, so `Vec<VorgEntry>` plus its own drop glue replaces the manual
// `free`-based `impl Drop` that used to live here. `num_vert_origin_y_metrics`
// is kept as a real field (not collapsed into `entries.len()`, unlike
// `CvtTable.length`): it is read independently at
// `otf_reader/unconsolidate.rs`'s `merge_vmtx` as the loop bound, and always
// equals `entries.len()` by construction at every write site, so keeping it
// is a conservative choice that changes no call site beyond the storage
// mechanism.
// `data`'s first 4 bytes (majorVersion/minorVersion) are read by neither
// this nor the original C -- VORG only ever shipped as version 1.0, and
// nothing here branches on it.
fn parse_vorg(data: &[u8]) -> Result<(GlyphId, Pos, Vec<VorgEntry>), ReadError> {
    let mut r = FontReader::new(data);
    r.skip(4)?;
    let default_vertical_origin = r.i16()? as Pos;
    let num_vert_origin_y_metrics = r.u16()? as GlyphId;
    r.require_room(num_vert_origin_y_metrics as usize, 4)?;
    let mut entries = Vec::with_capacity(num_vert_origin_y_metrics as usize);
    for _ in 0..num_vert_origin_y_metrics {
        let gid = r.u16()? as GlyphId;
        let vertical_origin = r.i16()?;
        entries.push(VorgEntry { gid, vertical_origin });
    }
    Ok((num_vert_origin_y_metrics, default_vertical_origin, entries))
}

pub fn otfcc_read_vorg(packet: &Packet, options: &Options) -> Option<Box<VorgTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_VORG)?;
    let (num_vert_origin_y_metrics, default_vertical_origin, entries) =
        match parse_vorg(&table.data) {
            Ok(parsed) => parsed,
            Err(_) => {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(b"Table 'VORG' corrupted."),
                );
                return None;
            }
        };
    Some(Box::new(VorgTable {
        num_vert_origin_y_metrics,
        default_vertical_origin,
        entries,
    }))
}
pub fn otfcc_build_vorg(table: Option<&VorgTable>) -> Option<Buffer> {
    let table = table?;
    let mut buf = Buffer::new();
    buf.write_u16be(1_u16);
    buf.write_u16be(0_u16);
    buf.write_u16be(pos_to_u16(table.default_vertical_origin));
    buf.write_u16be(table.num_vert_origin_y_metrics);
    let mut j: u16 = 0_u16;
    while (j as i32) < table.num_vert_origin_y_metrics as i32 {
        buf.write_u16be(table.entries[j as usize].gid);
        buf.write_u16be(table.entries[j as usize].vertical_origin as u16);
        j = j.wrapping_add(1);
    }
    Some(buf)
}

#[cfg(test)]
mod parse_vorg_tests {
    use super::*;

    fn header(default_vertical_origin: i16, num_vert_origin_y_metrics: u16) -> Vec<u8> {
        let mut b = vec![0u8, 1, 0, 0]; // majorVersion, minorVersion (unused)
        b.extend_from_slice(&default_vertical_origin.to_be_bytes());
        b.extend_from_slice(&num_vert_origin_y_metrics.to_be_bytes());
        b
    }

    #[test]
    fn well_formed_table_reads_every_entry() {
        let mut data = header(-100, 2);
        data.extend_from_slice(&5u16.to_be_bytes());
        data.extend_from_slice(&10i16.to_be_bytes());
        data.extend_from_slice(&9u16.to_be_bytes());
        data.extend_from_slice(&(-20i16).to_be_bytes());
        let (num, default_vertical_origin, entries) = parse_vorg(&data).unwrap();
        assert_eq!(num, 2);
        assert_eq!(default_vertical_origin, -100.0);
        assert_eq!(entries[0].gid, 5);
        assert_eq!(entries[0].vertical_origin, 10);
        assert_eq!(entries[1].gid, 9);
        assert_eq!(entries[1].vertical_origin, -20);
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(parse_vorg(&header(0, 0)[..6]).is_err());
    }

    #[test]
    fn entries_array_shorter_than_declared_count_errs_instead_of_reading_oob() {
        // num_vert_origin_y_metrics says 2 but only one 4-byte entry
        // actually follows the 8-byte header -- the original read past the
        // table's real end here unconditionally.
        let mut data = header(0, 2);
        data.extend_from_slice(&5u16.to_be_bytes());
        data.extend_from_slice(&10i16.to_be_bytes());
        assert!(parse_vorg(&data).is_err());
    }
}
