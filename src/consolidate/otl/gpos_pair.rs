use crate::support::options::Options;

use crate::support::glyph_order::GlyphOrder;

use crate::consolidate::otl::common::fontop_consolidate_class_def;
use crate::table::otl::classdef::shrink_class_def;
use crate::table::otl::Subtable;

pub fn consolidate_gpos_pair(
    glyph_order: &GlyphOrder,
    _subtable: &mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::GposPair(subtable) = _subtable else {
        unreachable!()
    };
    fontop_consolidate_class_def(Some(glyph_order), subtable.first.as_deref_mut(), options);
    fontop_consolidate_class_def(Some(glyph_order), subtable.second.as_deref_mut(), options);
    shrink_class_def(subtable.first.as_deref_mut().unwrap());
    shrink_class_def(subtable.second.as_deref_mut().unwrap());
    subtable.first.as_deref().unwrap().glyphs.is_empty()
}
