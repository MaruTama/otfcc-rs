use crate::support::glyph_order::GlyphOrder;

use crate::consolidate::otl::common::fontop_consolidate_class_def;
use crate::table::otl::Subtable;
use crate::table::otl::classdef::shrink_class_def;

pub fn consolidate_gpos_pair(glyph_order: &GlyphOrder, _subtable: &mut Subtable) -> bool {
    let Subtable::GposPair(subtable) = _subtable else {
        unreachable!()
    };
    fontop_consolidate_class_def(Some(glyph_order), subtable.first.as_deref_mut());
    fontop_consolidate_class_def(Some(glyph_order), subtable.second.as_deref_mut());
    let (first, second) = subtable.class_defs_mut();
    shrink_class_def(first);
    shrink_class_def(second);
    subtable.class_defs().0.glyphs.is_empty()
}
