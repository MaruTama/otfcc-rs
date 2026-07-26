#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md




use crate::support::options::{otfcc_Options};



use crate::font::caryll_font::{otfcc_Font};

























use crate::table::otl::{otl_Subtable, subtable_gpos_pair, table_OTL};
use crate::consolidate::otl::common::{fontop_consolidateClassDef};
use crate::table::otl::classdef::{otl_iClassDef};









pub unsafe extern "C" fn consolidate_gpos_pair(
    mut font: *mut otfcc_Font,
    mut _table: *mut table_OTL,
    mut _subtable: *mut otl_Subtable,
    mut options: *const otfcc_Options,
) -> bool {
    let mut subtable: *mut subtable_gpos_pair = &raw mut (*_subtable).gpos_pair;
    fontop_consolidateClassDef(font, (*subtable).first, options);
    fontop_consolidateClassDef(font, (*subtable).second, options);
    otl_iClassDef.shrink.expect("non-null function pointer")((*subtable).first);
    otl_iClassDef.shrink.expect("non-null function pointer")((*subtable).second);
    return (*(*subtable).first).numGlyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
}
