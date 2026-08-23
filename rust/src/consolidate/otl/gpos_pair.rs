use crate::support::options::Options;

use crate::font::caryll_font::Font;

use crate::consolidate::otl::common::fontop_consolidate_class_def;
use crate::table::otl::classdef::{ClassDef, shrink_class_def};
use crate::table::otl::{GposPairSubtable, OtlTable, Subtable};

pub unsafe extern "C" fn consolidate_gpos_pair(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    unsafe {
        let Subtable::GposPair(mut_subtable) = &mut *_subtable else {
            unreachable!()
        };
        let subtable: *mut GposPairSubtable = mut_subtable;
        let first_cd: *mut ClassDef = (*subtable).first.as_deref_mut().unwrap();
        let second_cd: *mut ClassDef = (*subtable).second.as_deref_mut().unwrap();
        fontop_consolidate_class_def(font, first_cd, &*options);
        fontop_consolidate_class_def(font, second_cd, &*options);
        shrink_class_def(first_cd);
        shrink_class_def(second_cd);
        return (*first_cd).glyphs.is_empty();
    }
}
