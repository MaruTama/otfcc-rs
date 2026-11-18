use crate::font::sfnt::Packet;
use crate::support::primitives::{GlyphId, Length, Pos};
use otfcc_binary::Buffer;
use otfcc_binary::{FontReader, ReadError};

use crate::table::maxp::MaxpTable;
use crate::table::vhea::VheaTable;
/// One long `vmtx` record: advance height and top side bearing.
#[derive(Copy, Clone, Debug)]
pub struct VerticalMetric {
    pub advance_height: Length,
    pub tsb: Pos,
}
/// `metrics` holds one record per glyph up to `vhea.num_of_long_ver_metrics`;
/// the glyphs after that only have an entry in `top_side_bearing`.
///
/// This table never appears in JSON: glyph-level metrics live on `Glyph`
/// (`advance_height`, `vertical_origin`), and `otf_writer::stat` builds this
/// table only as the intermediate for serialisation.
#[derive(Debug)]
pub struct VmtxTable {
    pub metrics: Vec<VerticalMetric>,
    pub top_side_bearing: Vec<Pos>,
}
fn parse_vmtx(data: &[u8], count_a: usize, count_k: usize) -> Result<VmtxTable, ReadError> {
    let mut r = FontReader::new(data);
    let mut metrics = Vec::with_capacity(count_a);
    for _ in 0..count_a {
        let advance_height = Length::from(r.u16()?);
        let tsb = Pos::from(r.i16()?);
        metrics.push(VerticalMetric {
            advance_height,
            tsb,
        });
    }
    let mut top_side_bearing = Vec::with_capacity(count_k);
    for _ in 0..count_k {
        top_side_bearing.push(Pos::from(r.i16()?));
    }
    Ok(VmtxTable {
        metrics,
        top_side_bearing,
    })
}
/// Reads `vmtx`, using `vhea` and `maxp` for the record counts. Returns
/// `None` if either is missing, if `num_of_long_ver_metrics` is 0 or exceeds
/// `maxp.num_glyphs`, if the table is absent, or if it is shorter than those
/// counts imply (after logging "corrupted").
pub fn read_vmtx(
    packet: &Packet,
    vhea: Option<&VheaTable>,
    maxp: Option<&MaxpTable>,
) -> Option<Box<VmtxTable>> {
    let vhea = vhea?;
    let maxp = maxp?;
    if vhea.num_of_long_ver_metrics == 0 || maxp.num_glyphs < vhea.num_of_long_ver_metrics {
        return None;
    }
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_VMTX)?;
    let count_a = vhea.num_of_long_ver_metrics as usize;
    let count_k = maxp.num_glyphs as usize - count_a;
    match parse_vmtx(&table.data, count_a, count_k) {
        Ok(vmtx) => Some(Box::new(vmtx)),
        Err(_) => {
            tracing::warn!("Table 'vmtx' corrupted.\n");
            None
        }
    }
}
/// Serialises the first `count_a` long metrics and `count_k` trailing top
/// side bearings (`num_of_long_ver_metrics` and the rest of `num_glyphs`).
/// Entries beyond those counts are dropped, and a `None` table gives an empty
/// buffer.
pub fn build_vmtx(vmtx: Option<&VmtxTable>, count_a: GlyphId, count_k: GlyphId) -> Buffer {
    let mut buf = Buffer::new();
    let vmtx = match vmtx {
        Some(v) => v,
        None => return buf,
    };
    for m in vmtx.metrics.iter().take(count_a as usize) {
        buf.write_u16be(m.advance_height as u16);
        buf.write_i16be(m.tsb as i16);
    }
    for &tsb in vmtx.top_side_bearing.iter().take(count_k as usize) {
        buf.write_i16be(tsb as i16);
    }
    buf
}

#[cfg(test)]
mod parse_vmtx_tests {
    use super::*;

    #[test]
    fn reads_full_metrics_then_trailing_top_side_bearings() {
        let mut data = Vec::new();
        data.extend_from_slice(&600u16.to_be_bytes()); // metrics[0].advance_height
        data.extend_from_slice(&(-5i16).to_be_bytes()); // metrics[0].tsb
        data.extend_from_slice(&(15i16).to_be_bytes()); // top_side_bearing[0]
        let vmtx = parse_vmtx(&data, 1, 1).unwrap();
        assert_eq!(vmtx.metrics.len(), 1);
        assert_eq!(vmtx.metrics[0].advance_height, 600.0);
        assert_eq!(vmtx.metrics[0].tsb, -5.0);
        assert_eq!(vmtx.top_side_bearing, vec![15.0]);
    }

    #[test]
    fn table_shorter_than_declared_counts_is_rejected_instead_of_reading_oob() {
        let data = vec![0u8; 3]; // one byte short of a single 4-byte metric
        assert!(parse_vmtx(&data, 1, 0).is_err());
    }
}
