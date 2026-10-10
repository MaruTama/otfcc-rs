use otfcc_binary::FontReader;
use crate::support::options::Options;
use crate::support::primitives::GlyphId;

use crate::table::otl::budget::OtlReadBudget;
use crate::table::otl::read::read_otl_subtable;
use crate::table::otl::{
    ExtendSubtable, LookupType, OTL_TYPE_GPOS_UNKNOWN, OTL_TYPE_GSUB_UNKNOWN, Subtable,
};

// `lookup_type` is computed before `subtable`, because the recursive read
// needs it as the nested lookup's type.
///
/// `extensionOffset` (the field this reads at `subtable_offset + 4`) is the
/// whole reason the Extension mechanism exists: it lets GSUB/GPOS carry a
/// real 32-bit subtable offset where every other lookup type is limited to
/// Offset16. That makes it a raw `u32` from the file (unlike
/// `subtable_offset` itself, which arrives here already bounded to a few
/// `u16` offsets summed together by the caller), so the two are combined
/// with `checked_add`: an `extensionOffset` near `u32::MAX` is rejected
/// rather than wrapped down to a small, wrong-but-in-bounds offset.
fn read_otl_extend(
    data: &[u8],
    subtable_offset: u32,
    basis: LookupType,
    max_glyphs: GlyphId,
    options: &Options,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    let mut r = FontReader::new(data).at(subtable_offset as usize).ok()?;
    let header = r.bytes(8).ok()?;
    let extension_lookup_type = u16::from_be_bytes([header[2], header[3]]);
    let extension_offset = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
    let real_subtable_offset = subtable_offset.checked_add(extension_offset)?;
    let lookup_type = LookupType::from_file(basis, extension_lookup_type);
    // `read_otl_subtable` returns `Option<Box<Subtable>>`, the same
    // type `ExtendSubtable.subtable` holds -- no conversion at this
    // boundary. A nested read that fails still yields an `Extend` with an
    // empty `subtable` (only a bad *header* above rejects the whole thing),
    // exactly as before.
    let subtable = read_otl_subtable(data, real_subtable_offset, lookup_type, max_glyphs, options, budget);
    Some(Subtable::Extend(ExtendSubtable { lookup_type, subtable }))
}
pub fn read_otl_gsub_extend(
    data: &[u8],
    subtable_offset: u32,
    max_glyphs: GlyphId,
    options: &Options,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    read_otl_extend(data, subtable_offset, OTL_TYPE_GSUB_UNKNOWN, max_glyphs, options, budget)
}
pub fn read_otl_gpos_extend(
    data: &[u8],
    subtable_offset: u32,
    max_glyphs: GlyphId,
    options: &Options,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    read_otl_extend(data, subtable_offset, OTL_TYPE_GPOS_UNKNOWN, max_glyphs, options, budget)
}

#[cfg(test)]
mod caryll_read_otl_extend_tests {
    use super::*;

    #[test]
    fn extension_offset_overflowing_u32_is_rejected_not_wrapped() {
        // subtable_offset (16, bounded -- summed from a couple of u16
        // lookup-table offsets by the caller) + extensionOffset (a raw u32
        // from the file): an extensionOffset this close to u32::MAX makes
        // the true sum overflow u32 entirely, and must be rejected rather
        // than wrapped to a small, wrong-but-in-bounds offset.
        let mut data = [0u8; 24];
        data[16..18].copy_from_slice(&1u16.to_be_bytes()); // substFormat
        data[18..20].copy_from_slice(&1u16.to_be_bytes()); // extensionLookupType
        data[20..24].copy_from_slice(&0xFFFF_FFF0u32.to_be_bytes()); // extensionOffset
        let options = Options::default();
        let result = read_otl_extend(&data, 16, OTL_TYPE_GSUB_UNKNOWN, 0, &options, &mut OtlReadBudget::new());
        assert!(result.is_none());
    }

    #[test]
    fn truncated_extension_header_is_rejected_not_read_oob() {
        let data = [0u8; 20]; // subtable_offset=16 needs 8 more bytes, only 4 remain
        let options = Options::default();
        let result = read_otl_extend(&data, 16, OTL_TYPE_GSUB_UNKNOWN, 0, &options, &mut OtlReadBudget::new());
        assert!(result.is_none());
    }
}
