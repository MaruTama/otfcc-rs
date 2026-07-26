#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};





use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u};

use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};

use crate::table::otl::{LookupType, Subtable, otl_type_gpos_unknown, otl_type_gsub_unknown, ExtendSubtable};
use crate::table::otl::read::{otfcc_readOtl_subtable};

unsafe extern "C" fn _caryll_read_otl_extend(
    mut data: FontFilePointer,
    mut tableLength: u32,
    mut subtableOffset: u32,
    mut BASIS: LookupType,
    maxGlyphs: GlyphId,
    mut options: *const Options,
) -> *mut Subtable {
    let mut subtable: *mut ExtendSubtable = ::core::ptr::null_mut::<ExtendSubtable>();
    let mut _subtable: *mut Subtable = ::core::ptr::null_mut::<Subtable>();
    _subtable = __caryll_allocate_clean(
        ::core::mem::size_of::<Subtable>() as usize,
        10 as ::core::ffi::c_ulong,
    ) as *mut Subtable;
    if tableLength < subtableOffset.wrapping_add(8 as u32) {
        free(_subtable as *mut ::core::ffi::c_void);
        _subtable = ::core::ptr::null_mut::<Subtable>();
    } else {
        subtable = &raw mut (*_subtable).extend;
        (*subtable).type_0 = LookupType::from_file(
            BASIS,
            read_16u(
                data.offset(subtableOffset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ),
        );
        (*subtable).subtable = otfcc_readOtl_subtable(
            data as *mut u8,
            tableLength,
            subtableOffset.wrapping_add(read_32u(
                data.offset(subtableOffset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            )),
            (*subtable).type_0,
            maxGlyphs,
            options,
        ) as *mut Subtable;
    }
    return _subtable;
}
pub unsafe extern "C" fn otfcc_readOtl_gsub_extend(
    mut data: FontFilePointer,
    mut tableLength: u32,
    mut subtableOffset: u32,
    maxGlyphs: GlyphId,
    mut options: *const Options,
) -> *mut Subtable {
    return _caryll_read_otl_extend(
        data,
        tableLength,
        subtableOffset,
        otl_type_gsub_unknown,
        maxGlyphs,
        options,
    );
}
pub unsafe extern "C" fn otfcc_readOtl_gpos_extend(
    mut data: FontFilePointer,
    mut tableLength: u32,
    mut subtableOffset: u32,
    maxGlyphs: GlyphId,
    mut options: *const Options,
) -> *mut Subtable {
    return _caryll_read_otl_extend(
        data,
        tableLength,
        subtableOffset,
        otl_type_gpos_unknown,
        maxGlyphs,
        options,
    );
}
