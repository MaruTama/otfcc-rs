#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::font_reader::{FontReader, ReadError};

use crate::font::caryll_sfnt::Packet;

use crate::table::maxp::MaxpTable;

// `widths` was `__caryll_allocate_clean`'d/`free`'d, sized from `maxp`'s
// `num_glyphs`. `Copy` is dropped along with the raw pointer -- same
// "owning heap data through a raw pointer, freed elsewhere" smell a final
// full-crate audit flagged for `Packet`/`PacketPiece` before those were
// fixed; `HdmxTable`/`DeviceRecord` are confirmed dead code (see below) so
// this was inert, but converted for consistency rather than left as the
// one remaining instance of the pattern.
#[repr(C)]
pub struct DeviceRecord {
    pub pixel_size: u8,
    pub max_width: u8,
    pub widths: Vec<u8>,
}
#[repr(C)]
pub struct HdmxTable {
    pub version: u16,
    pub num_records: u16,
    pub size_device_record: u32,
    pub records: Vec<DeviceRecord>,
}
// Stage 6-4 "Box化": `HdmxTable` is entirely dead code -- `otfcc_read_hdmx`
// is never called from `otf_reader.rs` (HDMX has no wired build/dump path
// in this crate at all, confirmed by grepping the whole crate for
// `HdmxTable`/`hdmx` outside this file and `Font`'s own field list), so
// this conversion has zero call sites to update. Converted anyway for
// consistency with every other `Font` field. `records` (and each
// `DeviceRecord.widths`) is now a plain `Vec`, so the old manual
// `dispose_hdmx`/vtable-`Drop`-impl pair this replaced is no longer
// needed at all -- `Vec`'s own drop glue reaches every level.
// Confirmed dead code crate-wide (see the module doc comment above) -- no
// logger call on failure, matching this function's existing unused
// `_options` parameter; kept purely `Result`-shaped for the same
// "everything parses into owned values before any allocation" discipline
// as every other table this migration touches.
fn parse_hdmx(
    data: &[u8],
    num_glyphs: usize,
) -> Result<(u16, u16, u32, Vec<DeviceRecord>), ReadError> {
    let mut r = FontReader::new(data);
    let version = r.u16()?;
    let num_records = r.u16()?;
    let size_device_record = r.u32()?;
    let stride = 2usize.wrapping_add(num_glyphs);
    r.require_room(num_records as usize, stride)?;
    let mut records = Vec::with_capacity(num_records as usize);
    for i in 0..num_records as usize {
        let mut rr = r.at(8usize.wrapping_add(i.wrapping_mul(stride)))?;
        let pixel_size = rr.u8()?;
        let max_width = rr.u8()?;
        let widths = rr.bytes(num_glyphs)?.to_vec();
        records.push(DeviceRecord {
            pixel_size,
            max_width,
            widths,
        });
    }
    Ok((version, num_records, size_device_record, records))
}

pub unsafe fn otfcc_read_hdmx(packet: &Packet, mut maxp: *mut MaxpTable) -> Option<Box<HdmxTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_HDMX)?;
    let (version, num_records, size_device_record, records) =
        parse_hdmx(&table.data, (*maxp).num_glyphs as usize).ok()?;
    Some(Box::new(HdmxTable {
        version,
        num_records,
        size_device_record,
        records,
    }))
}

#[cfg(test)]
mod parse_hdmx_tests {
    use super::*;

    #[test]
    fn valid_table_resolves_all_records() {
        // header: version=0, num_records=2, size_device_record=4
        // records (stride = 2 + num_glyphs = 4, num_glyphs = 2):
        //   record 0: pixel_size=8,  max_width=9,  widths=[1,2]
        //   record 1: pixel_size=10, max_width=11, widths=[3,4]
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 8, 9, 1, 2, 10, 11, 3, 4,
        ];
        let (version, num_records, size_device_record, records) = parse_hdmx(data, 2).unwrap();
        assert_eq!(version, 0);
        assert_eq!(num_records, 2);
        assert_eq!(size_device_record, 4);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].pixel_size, 8);
        assert_eq!(records[0].max_width, 9);
        assert_eq!(records[0].widths, vec![1, 2]);
        assert_eq!(records[1].pixel_size, 10);
        assert_eq!(records[1].max_width, 11);
        assert_eq!(records[1].widths, vec![3, 4]);
    }

    #[test]
    fn truncated_records_errs_instead_of_reading_oob() {
        // num_records says 2 (stride 4 each = 8 bytes), but only one
        // record's worth of bytes actually follows the 8-byte header --
        // this used to walk straight past the allocation computing
        // `record_base` from an untrusted `i * stride` with no check that
        // the whole records array actually fits.
        let data: &[u8] = &[0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 8, 9, 1, 2];
        assert!(parse_hdmx(data, 2).is_err());
    }
}
