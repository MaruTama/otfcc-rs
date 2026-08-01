#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md




use crate::support::options::{Options};



use crate::font::caryll_font::{Font};

























use crate::table::otl::{Subtable, GposPairSubtable, OtlTable};
use crate::consolidate::otl::common::{fontop_consolidate_class_def};
use crate::table::otl::classdef::{OTL_I_CLASS_DEF};









pub unsafe extern "C" fn consolidate_gpos_pair(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GposPairSubtable = &raw mut (*_subtable).gpos_pair;
    fontop_consolidate_class_def(font, (*subtable).first, options);
    fontop_consolidate_class_def(font, (*subtable).second, options);
    OTL_I_CLASS_DEF.shrink.expect("non-null function pointer")((*subtable).first);
    OTL_I_CLASS_DEF.shrink.expect("non-null function pointer")((*subtable).second);
    return (*(*subtable).first).glyphs.is_empty();
}
