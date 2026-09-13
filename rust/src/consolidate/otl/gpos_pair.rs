use crate::support::options::Options;

use crate::font::caryll_font::Font;

use crate::consolidate::otl::common::fontop_consolidate_class_def;
use crate::table::otl::classdef::shrink_class_def;
use crate::table::otl::{OtlTable, Subtable};

pub fn consolidate_gpos_pair(
    font: &Font,
    _table: *const OtlTable,
    _subtable: &mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::GposPair(subtable) = _subtable else {
        unreachable!()
    };
    let glyph_order = font.glyph_order.as_deref();
    fontop_consolidate_class_def(glyph_order, subtable.first.as_deref_mut(), options);
    fontop_consolidate_class_def(glyph_order, subtable.second.as_deref_mut(), options);
    shrink_class_def(subtable.first.as_deref_mut().unwrap());
    shrink_class_def(subtable.second.as_deref_mut().unwrap());
    subtable.first.as_deref().unwrap().glyphs.is_empty()
}
