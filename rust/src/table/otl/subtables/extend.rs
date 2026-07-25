#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};
unsafe extern "C" {
    fn otfcc_readOtl_subtable(
        data: *mut u8,
        tableLength: u32,
        subtableOffset: u32,
        lookupType: otl_LookupType,
        maxGlyphs: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut otl_Subtable;
}





use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u};

use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphid_t};

use crate::table::otl::{otl_LookupType, otl_Subtable, otl_type_gpos_unknown, otl_type_gsub_unknown, subtable_extend};

unsafe extern "C" fn _caryll_read_otl_extend(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut subtableOffset: u32,
    mut BASIS: otl_LookupType,
    maxGlyphs: glyphid_t,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut subtable: *mut subtable_extend = ::core::ptr::null_mut::<subtable_extend>();
    let mut _subtable: *mut otl_Subtable = ::core::ptr::null_mut::<otl_Subtable>();
    _subtable = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_Subtable>() as usize,
        10 as ::core::ffi::c_ulong,
    ) as *mut otl_Subtable;
    if tableLength < subtableOffset.wrapping_add(8 as u32) {
        free(_subtable as *mut ::core::ffi::c_void);
        _subtable = ::core::ptr::null_mut::<otl_Subtable>();
    } else {
        subtable = &raw mut (*_subtable).extend;
        (*subtable).type_0 = (read_16u(
            data.offset(subtableOffset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as ::core::ffi::c_uint)
            .wrapping_add(BASIS as ::core::ffi::c_uint)
            as otl_LookupType;
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
        ) as *mut otl_Subtable;
    }
    return _subtable;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_readOtl_gsub_extend(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut subtableOffset: u32,
    maxGlyphs: glyphid_t,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    return _caryll_read_otl_extend(
        data,
        tableLength,
        subtableOffset,
        otl_type_gsub_unknown,
        maxGlyphs,
        options,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_readOtl_gpos_extend(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut subtableOffset: u32,
    maxGlyphs: glyphid_t,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    return _caryll_read_otl_extend(
        data,
        tableLength,
        subtableOffset,
        otl_type_gpos_unknown,
        maxGlyphs,
        options,
    );
}
