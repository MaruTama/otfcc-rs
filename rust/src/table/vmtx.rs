#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::binio::{pos_to_u16};
use crate::support::font_reader::{FontReader, ReadError};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, logger_log_sds};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, Length, Pos};
use crate::font::caryll_sfnt::{Packet};

use crate::table::maxp::{MaxpTable};
use crate::table::vhea::{VheaTable};
use crate::support::buffer::{bufnew, bufwrite16b};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VerticalMetric {
    pub advance_height: Length,
    pub tsb: Pos,
}
// Both fields are now plain `Vec`s, its horizontal-axis mirror `HmtxTable`'s
// own comment explains why there's no JSON-side fallout: this table is a
// pure `vmtx`-binary-serialization intermediate, never touched by dump/
// parse.
#[repr(C)]
pub struct VmtxTable {
    pub metrics: Vec<VerticalMetric>,
    pub top_side_bearing: Vec<Pos>,
}
fn parse_vmtx(data: &[u8], count_a: usize, count_k: usize) -> Result<VmtxTable, ReadError> {
    let mut r = FontReader::new(data);
    let mut metrics = Vec::with_capacity(count_a);
    for _ in 0..count_a {
        let advance_height = r.u16()? as Length;
        let tsb = r.i16()? as Pos;
        metrics.push(VerticalMetric { advance_height, tsb });
    }
    let mut top_side_bearing = Vec::with_capacity(count_k);
    for _ in 0..count_k {
        top_side_bearing.push(r.i16()? as Pos);
    }
    Ok(VmtxTable { metrics, top_side_bearing })
}
pub unsafe fn otfcc_read_vmtx(
    packet: &Packet,
    mut options: *const Options,
    mut vhea: *mut VheaTable,
    mut maxp: *mut MaxpTable,
) -> Option<Box<VmtxTable>> {
    if vhea.is_null()
        || maxp.is_null()
        || (*vhea).num_of_long_ver_metrics as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || ((*maxp).num_glyphs as ::core::ffi::c_int)
            < (*vhea).num_of_long_ver_metrics as ::core::ffi::c_int
    {
        return None;
    }
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_VMTX)?;
    let count_a = (*vhea).num_of_long_ver_metrics as usize;
    let count_k = (*maxp).num_glyphs as usize - count_a;
    match parse_vmtx(&table.data, count_a, count_k) {
        Ok(vmtx) => Some(Box::new(vmtx)),
        Err(_) => {
            logger_log_sds(
                (*options).logger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"Table 'vmtx' corrupted.\n"),
            );
            None
        }
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_vmtx(
    vmtx: Option<&VmtxTable>,
    mut count_a: GlyphId,
    mut count_k: GlyphId,
) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
    let vmtx = match vmtx {
        Some(v) => v,
        None => return buf,
    };
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
        bufwrite16b(buf, vmtx.metrics[j as usize].advance_height as u16);
        bufwrite16b(buf, pos_to_u16(vmtx.metrics[j as usize].tsb));
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
        bufwrite16b(buf, pos_to_u16(vmtx.top_side_bearing[j_0 as usize]));
        j_0 = j_0.wrapping_add(1);
    }
    return buf;
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
