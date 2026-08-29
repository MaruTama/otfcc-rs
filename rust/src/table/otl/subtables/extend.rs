use crate::support::font_reader::FontReader;
use crate::support::options::Options;
use crate::support::primitives::{FontFilePointer, GlyphId};

use crate::table::otl::read::otfcc_read_otl_subtable;
use crate::table::otl::{
    ExtendSubtable, LookupType, OTL_TYPE_GPOS_UNKNOWN, OTL_TYPE_GSUB_UNKNOWN, Subtable,
};

// Was: allocate a whole `Subtable`-sized block directly, then take
// `&raw mut (*_subtable).extend` and fill the field in place -- sound only
// because `Subtable` was a union (every field starts at offset 0). Once it
// is an enum with its own discriminant, there is no "the block" to allocate
// ahead of knowing which variant it will hold; build the `ExtendSubtable`
// value locally instead and hand it to `Box::new(Subtable::Extend(..))` the
// same way every other subtable's read function now does via
// `subtable_from_raw`. `type_0` is still computed before `subtable` (the
// recursive read needs it as the nested lookup's type), so the dependency
// order is unchanged.
///
/// `extensionOffset` (the field this reads at `subtable_offset + 4`) is the
/// whole reason the Extension mechanism exists: it lets GSUB/GPOS carry a
/// real 32-bit subtable offset where every other lookup type is limited to
/// Offset16. That makes it a fully attacker-controlled `u32` (unlike
/// `subtable_offset` itself, which arrives here already bounded to a few
/// `u16` offsets summed together by the caller) -- the original combined
/// the two with `subtable_offset.wrapping_add(extensionOffset)`, which for
/// an `extensionOffset` near `u32::MAX` wraps the sum back down to a small,
/// wrong-but-in-bounds value instead of the real (out-of-range) one, so a
/// downstream `otfcc_read_otl_subtable` call would silently read whatever
/// happens to live at that wrong small offset. `checked_add` rejects it
/// outright instead.
unsafe fn _caryll_read_otl_extend(
    data: FontFilePointer,
    table_length: u32,
    subtable_offset: u32,
    basis: LookupType,
    max_glyphs: GlyphId,
    options: &Options,
) -> *mut Subtable {
    unsafe {
        let slice = ::core::slice::from_raw_parts(data as *const u8, table_length as usize);
        let Ok(mut r) = FontReader::new(slice).at(subtable_offset as usize) else {
            return ::core::ptr::null_mut::<Subtable>();
        };
        let Ok(header) = r.bytes(8) else {
            return ::core::ptr::null_mut::<Subtable>();
        };
        let extension_lookup_type = u16::from_be_bytes([header[2], header[3]]);
        let extension_offset =
            u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
        let Some(real_subtable_offset) = subtable_offset.checked_add(extension_offset) else {
            return ::core::ptr::null_mut::<Subtable>();
        };
        let type_0 = LookupType::from_file(basis, extension_lookup_type);
        let subtable = otfcc_read_otl_subtable(
            data,
            table_length,
            real_subtable_offset,
            type_0,
            max_glyphs,
            options,
        );
        Box::into_raw(Box::new(Subtable::Extend(ExtendSubtable {
            type_0,
            subtable,
        })))
    }
}
pub unsafe fn otfcc_read_otl_gsub_extend(
    data: FontFilePointer,
    table_length: u32,
    subtable_offset: u32,
    max_glyphs: GlyphId,
    options: &Options,
) -> *mut Subtable {
    return unsafe {
        _caryll_read_otl_extend(
            data,
            table_length,
            subtable_offset,
            OTL_TYPE_GSUB_UNKNOWN,
            max_glyphs,
            options,
        )
    };
}
pub unsafe fn otfcc_read_otl_gpos_extend(
    data: FontFilePointer,
    table_length: u32,
    subtable_offset: u32,
    max_glyphs: GlyphId,
    options: &Options,
) -> *mut Subtable {
    return unsafe {
        _caryll_read_otl_extend(
            data,
            table_length,
            subtable_offset,
            OTL_TYPE_GPOS_UNKNOWN,
            max_glyphs,
            options,
        )
    };
}

#[cfg(test)]
mod caryll_read_otl_extend_tests {
    use super::*;

    #[test]
    fn extension_offset_overflowing_u32_is_rejected_not_wrapped() {
        // subtable_offset (16, bounded -- summed from a couple of u16
        // lookup-table offsets by the caller) + extensionOffset (a raw,
        // fully attacker-controlled u32 read straight from the file) must
        // not be combined with `wrapping_add`: an extensionOffset this
        // close to u32::MAX makes the true sum overflow u32 entirely, and
        // the original's wraparound would silently hand a small,
        // wrong-but-in-bounds offset to `otfcc_read_otl_subtable` instead
        // of rejecting the request.
        let mut data = [0u8; 24];
        data[16..18].copy_from_slice(&1u16.to_be_bytes()); // substFormat
        data[18..20].copy_from_slice(&1u16.to_be_bytes()); // extensionLookupType
        data[20..24].copy_from_slice(&0xFFFF_FFF0u32.to_be_bytes()); // extensionOffset
        let options = Options::default();
        let result = unsafe {
            _caryll_read_otl_extend(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                16,
                OTL_TYPE_GSUB_UNKNOWN,
                0,
                &options,
            )
        };
        assert!(result.is_null());
    }

    #[test]
    fn truncated_extension_header_is_rejected_not_read_oob() {
        let data = [0u8; 20]; // subtable_offset=16 needs 8 more bytes, only 4 remain
        let options = Options::default();
        let result = unsafe {
            _caryll_read_otl_extend(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                16,
                OTL_TYPE_GSUB_UNKNOWN,
                0,
                &options,
            )
        };
        assert!(result.is_null());
    }
}
