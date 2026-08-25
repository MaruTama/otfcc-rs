use crate::support::binio::{read_16u, read_32u};

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
unsafe fn _caryll_read_otl_extend(
    data: FontFilePointer,
    table_length: u32,
    subtable_offset: u32,
    basis: LookupType,
    max_glyphs: GlyphId,
    options: &Options,
) -> *mut Subtable {
    unsafe {
        if table_length < subtable_offset.wrapping_add(8 as u32) {
            return ::core::ptr::null_mut::<Subtable>();
        }
        let type_0 = LookupType::from_file(
            basis,
            read_16u(
                data.offset(subtable_offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ),
        );
        let subtable = otfcc_read_otl_subtable(
            data as *mut u8,
            table_length,
            subtable_offset.wrapping_add(read_32u(
                data.offset(subtable_offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            )),
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
