use super::*;
use crate::libcff::TYPE2_TRANSIENT_ARRAY;
use crate::libcff::index::CffIndexCountType;

use crate::table::glyf::new_glyf_glyph;

// Subroutine numbers are signed: with the bias of 107 that applies to
// INDEXes of fewer than 1240 subroutines, the first one is called as
// `-107`. The operand used to be converted with `as u32`, which
// saturates every negative number to 0 -- so any font with more than
// 107 subroutines had its lower ones replaced by the one at `bias`.

/// 108 subroutines: number 0 (called as `-107`) draws `1 2 rlineto`,
/// the rest only `return`.
fn subrs_108() -> CffIndex {
    let mut data: Vec<u8> = vec![140, 141, 5, 11];
    let mut offset: Vec<u32> = vec![1, 5];
    for _ in 1..108 {
        data.push(11);
        offset.push(data.len() as u32 + 1);
    }
    CffIndex {
        count_type: CffIndexCountType::U16,
        count: 108,
        off_size: 1,
        offset,
        data,
    }
}

fn empty_cff_index() -> CffIndex {
    CffIndex {
        count_type: CffIndexCountType::U16,
        count: 0,
        off_size: 0,
        offset: Vec::new(),
        data: Vec::new(),
    }
}

/// Runs `0 0 rmoveto -107 <call_op> endchar`; returns the points drawn.
fn points_after_calling_minus_107(call_op: u8, gsubr: &CffIndex, lsubr: &CffIndex) -> Vec<(f64, f64)> {
    let data: Vec<u8> = vec![139, 139, 21, 32, call_op, 14];
    let mut stack = CffStack {
        stack: vec![CffValue::Unset; 16],
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
    parse_outline(&data, gsubr, lsubr, &mut stack, &mut ctx, 0, &mut total_calls);
    g.contours
        .iter()
        .flatten()
        .map(|p| (p.x.kernel, p.y.kernel))
        .collect()
}

#[test]
fn callgsubr_with_a_negative_number_calls_the_subroutine_below_the_bias() {
    let points = points_after_calling_minus_107(29, &subrs_108(), &empty_cff_index());
    assert_eq!(points, vec![(0.0, 0.0), (1.0, 2.0)]);
}

#[test]
fn callsubr_with_a_negative_number_calls_the_subroutine_below_the_bias() {
    let points = points_after_calling_minus_107(10, &empty_cff_index(), &subrs_108());
    assert_eq!(points, vec![(0.0, 0.0), (1.0, 2.0)]);
}
