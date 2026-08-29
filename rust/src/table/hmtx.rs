#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::binio::pos_to_u16;
use crate::support::buffer::Buffer;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Length, Pos};

use crate::support::buffer::{bufnew, bufwrite16b};
use crate::table::hhea::HheaTable;
use crate::table::maxp::MaxpTable;
#[derive(Copy, Clone)]
pub struct HorizontalMetric {
    pub advance_width: Length,
    pub lsb: Pos,
}
// Both fields are now plain `Vec`s, so `HmtxTable` needs no custom `Drop`
// impl -- ordinary field-by-field drop glue reaches both allocations.
// `HmtxTable` never appears in JSON dump/parse (glyph-level metrics live on
// `Glyph.advance_width`/`.horizontal_origin` instead; this table exists
// purely as an `hmtx`-binary-serialization intermediate, confirmed by grep:
// its only touch points are this file's own read/build functions and
// `otf_writer/stat.rs`'s `stat_hmtx`, which constructs it), so there is no
// JSON-side fallout from this field type change.
pub struct HmtxTable {
    pub metrics: Vec<HorizontalMetric>,
    pub left_side_bearing: Vec<Pos>,
}
fn parse_hmtx(data: &[u8], count_a: usize, count_k: usize) -> Result<HmtxTable, ReadError> {
    let mut r = FontReader::new(data);
    let mut metrics = Vec::with_capacity(count_a);
    for _ in 0..count_a {
        let advance_width = r.u16()? as Length;
        let lsb = r.i16()? as Pos;
        metrics.push(HorizontalMetric { advance_width, lsb });
    }
    let mut left_side_bearing = Vec::with_capacity(count_k);
    for _ in 0..count_k {
        left_side_bearing.push(r.i16()? as Pos);
    }
    Ok(HmtxTable {
        metrics,
        left_side_bearing,
    })
}
pub unsafe fn otfcc_read_hmtx(
    packet: &Packet,
    options: &Options,
    hhea: *mut HheaTable,
    maxp: *mut MaxpTable,
) -> Option<Box<HmtxTable>> {
    if hhea.is_null()
        || maxp.is_null()
        || (*hhea).number_of_metrics == 0
        || ((*maxp).num_glyphs as i32)
            < (*hhea).number_of_metrics as i32
    {
        return None;
    }
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_HMTX)?;
    let count_a = (*hhea).number_of_metrics as usize;
    let count_k = (*maxp).num_glyphs as usize - count_a;
    match parse_hmtx(&table.data, count_a, count_k) {
        Ok(hmtx) => Some(Box::new(hmtx)),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"Table 'hmtx' corrupted.\n"),
            );
            None
        }
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_hmtx(
    hmtx: Option<&HmtxTable>,
    count_a: GlyphId,
    count_k: GlyphId,
) -> *mut Buffer {
    let buf: *mut Buffer = bufnew();
    let hmtx = match hmtx {
        Some(h) => h,
        None => return buf,
    };
    let mut j: GlyphId = 0 as GlyphId;
    while (j as i32) < count_a as i32 {
        bufwrite16b(buf, hmtx.metrics[j as usize].advance_width as u16);
        bufwrite16b(buf, pos_to_u16(hmtx.metrics[j as usize].lsb));
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as i32) < count_k as i32 {
        bufwrite16b(buf, pos_to_u16(hmtx.left_side_bearing[j_0 as usize]));
        j_0 = j_0.wrapping_add(1);
    }
    return buf;
}

#[cfg(test)]
mod parse_hmtx_tests {
    use super::*;

    #[test]
    fn reads_full_metrics_then_trailing_left_side_bearings() {
        let mut data = Vec::new();
        data.extend_from_slice(&500u16.to_be_bytes()); // metrics[0].advance_width
        data.extend_from_slice(&(-10i16).to_be_bytes()); // metrics[0].lsb
        data.extend_from_slice(&(20i16).to_be_bytes()); // left_side_bearing[0]
        let hmtx = parse_hmtx(&data, 1, 1).unwrap();
        assert_eq!(hmtx.metrics.len(), 1);
        assert_eq!(hmtx.metrics[0].advance_width, 500.0);
        assert_eq!(hmtx.metrics[0].lsb, -10.0);
        assert_eq!(hmtx.left_side_bearing, vec![20.0]);
    }

    #[test]
    fn table_shorter_than_declared_counts_is_rejected_instead_of_reading_oob() {
        let data = vec![0u8; 3]; // one byte short of a single 4-byte metric
        assert!(parse_hmtx(&data, 1, 0).is_err());
    }
}
