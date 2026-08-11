#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md




use crate::support::options::{Options};



use crate::font::caryll_font::{Font};

























use crate::table::otl::{Subtable, GposPairSubtable, OtlTable};
use crate::consolidate::otl::common::{fontop_consolidate_class_def};
use crate::table::otl::classdef::{ClassDef, OTL_I_CLASS_DEF};









pub unsafe extern "C" fn consolidate_gpos_pair(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let Subtable::GposPair(mut_subtable) = &mut *_subtable else { unreachable!() };
    let subtable: *mut GposPairSubtable = mut_subtable;
    let first_cd: *mut ClassDef = (*subtable).first.as_deref_mut().unwrap();
    let second_cd: *mut ClassDef = (*subtable).second.as_deref_mut().unwrap();
    fontop_consolidate_class_def(font, first_cd, options);
    fontop_consolidate_class_def(font, second_cd, options);
    OTL_I_CLASS_DEF.shrink.expect("non-null function pointer")(first_cd);
    OTL_I_CLASS_DEF.shrink.expect("non-null function pointer")(second_cd);
    return (*first_cd).glyphs.is_empty();
}
