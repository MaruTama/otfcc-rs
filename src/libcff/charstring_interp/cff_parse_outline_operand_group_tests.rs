use super::*;
use crate::libcff::TYPE2_TRANSIENT_ARRAY;
use crate::libcff::index::CffIndexCountType;

use crate::table::glyf::new_glyf_glyph;

// The line/curve operators take their operands in fixed-size groups
// (2 for `rlineto`, 6 for `rrcurveto`, 4 for `vvcurveto`/`hhcurveto`).
// A malformed CharString can push a count that leaves an incomplete
// group at the end; the interpreter used to draw that group anyway,
// reading slots at or past `index` -- stale values from an earlier
// operator or glyph (the operand stack is reused across a font's
// glyphs), or, with the stack nearly full, an index past the end of
// the `Vec` and a panic. Each test sizes the operand stack to exactly
// the operands it pushes, so any read past `index` panics here.

fn empty_cff_index() -> CffIndex {
    CffIndex {
        count_type: CffIndexCountType::U16,
        count: 0,
        off_size: 0,
        offset: Vec::new(),
        data: Vec::new(),
    }
}

/// Runs `0 0 rmoveto`, then `operands` copies of `1` followed by `op`,
/// on an operand stack with no spare slots; returns the number of
/// points drawn after the moveto's own start point.
fn points_drawn(operands: usize, op: u8) -> usize {
    let mut data: Vec<u8> = vec![139, 139, 21];
    data.extend(std::iter::repeat_n(140u8, operands));
    data.push(op);
    let gsubr = empty_cff_index();
    let lsubr = empty_cff_index();
    let mut stack = CffStack {
        stack: vec![CffValue::Unset; operands.max(2)],
        transient: [CffValue::Unset; TYPE2_TRANSIENT_ARRAY],
        index: 0,
        stem: 0,
    };
    let mut total_calls: u32 = 0;
    let mut g = new_glyf_glyph();
    let mut ctx = OutlineBuilderContext {
        g: &mut g,
        j_contour: 0,
        j_point: 0,
        default_width_x: 0.0,
        nominal_width_x: 0.0,
        defined_h_stems: 0,
        defined_v_stems: 0,
        defined_hint_masks: 0,
        defined_contour_masks: 0,
        randx: 0,
    };
    parse_outline(&data, &gsubr, &lsubr, &mut stack, &mut ctx, 0, &mut total_calls);
    assert_eq!(stack.index, 0);
    g.contours.iter().map(|c| c.len()).sum::<usize>() - 1
}

#[test]
fn rlineto_drops_an_incomplete_trailing_pair() {
    assert_eq!(points_drawn(4, 5), 2);
    assert_eq!(points_drawn(3, 5), 1);
}

#[test]
fn rrcurveto_drops_an_incomplete_trailing_curve() {
    assert_eq!(points_drawn(12, 8), 6);
    assert_eq!(points_drawn(7, 8), 3);
    assert_eq!(points_drawn(11, 8), 3);
    assert_eq!(points_drawn(5, 8), 0);
}

#[test]
fn rcurveline_drops_an_incomplete_curve_before_the_line() {
    assert_eq!(points_drawn(8, 24), 4);
    assert_eq!(points_drawn(9, 24), 4);
    assert_eq!(points_drawn(13, 24), 4);
}

#[test]
fn vvcurveto_and_hhcurveto_drop_an_incomplete_trailing_curve() {
    for op in [26u8, 27] {
        assert_eq!(points_drawn(4, op), 3);
        assert_eq!(points_drawn(5, op), 3);
        assert_eq!(points_drawn(6, op), 3);
        assert_eq!(points_drawn(7, op), 3);
        assert_eq!(points_drawn(9, op), 6);
    }
}

#[test]
fn vvcurveto_and_hhcurveto_with_only_the_leading_operand_draw_nothing() {
    for op in [26u8, 27] {
        assert_eq!(points_drawn(1, op), 0);
    }
}
