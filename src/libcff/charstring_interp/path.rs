use super::*;
use crate::support::primitives::Arity;
use crate::table::cff::OutlineBuilderContext;
use crate::table::cff::read::{
    callback_draw_curveto, callback_draw_lineto,
    callback_draw_next_contour, callback_draw_sethint, callback_draw_setmask, callback_draw_setwidth,
};
use crate::libcff::{CffStack, OP_CNTRMASK, OP_FLEX, OP_FLEX1, OP_HFLEX, OP_HFLEX1, OP_HMOVETO, OP_RMOVETO, OP_VMOVETO, OP_VSTEM, OP_VSTEMHM};

/// Reads the stem hints on the stack: pairs of `edge width`, each edge
/// relative to the end of the previous stem. An odd operand count means
/// the first operand is the glyph's advance width.
pub(super) fn read_stem_hints(stack: &mut CffStack, outline: &mut OutlineBuilderContext, vertical: bool) {
    if !stack.index.is_multiple_of(2) {
        callback_draw_setwidth(outline, stack.num(0));
    }
    // `saturating_add`, not `wrapping_add`: this counter sizes the
    // `hintmask`/`cntrmask` bit array below and must never wrap back down
    // to a small value while `stem_h`/`stem_v` (unbounded, real counts)
    // keep growing -- see the `stem` field's doc comment.
    stack.stem = stack.stem.saturating_add(stack.index / 2);
    let mut previous_end = 0.0;
    for j in (stack.index % 2..stack.index).step_by(2) {
        let (edge, width) = (stack.num(j), stack.num(j + 1));
        callback_draw_sethint(outline, vertical, edge + previous_end, width);
        previous_end += edge + width;
    }
}

/// `hstem`, `vstem`, `hstemhm`, `vstemhm`
pub(super) fn op_stem_hints(stack: &mut CffStack, outline: &mut OutlineBuilderContext, op: i32) -> Flow {
    read_stem_hints(stack, outline, op == OP_VSTEM.0 || op == OP_VSTEMHM.0);
    stack.clear();
    Flow::Continue
}

/// `hintmask`/`cntrmask`: one bit per stem hint, in bytes that follow the
/// operator in the CharString. Operands before it are stem hints too:
/// vertical ones when some stems were declared already (the implicit
/// `vstem` after `hstem`s), horizontal otherwise. Returns the number of
/// mask bytes read, or `None` when the CharString ends before them.
pub(super) fn op_hint_mask(stack: &mut CffStack, outline: &mut OutlineBuilderContext, op: i32, mask_bytes: &[u8]) -> Option<u32> {
    let vertical = stack.stem > 0;
    read_stem_hints(stack, outline, vertical);
    let mask_length = stack.stem.div_ceil(8);
    // The mask bytes never go through `decode_cs2_token`'s bounds
    // checking, and `mask_length` follows the stem count: a fuzz-found
    // input declared enough stems to read one byte past the CharString
    // (an ASan-confirmed heap-buffer-overflow). `mask_bytes` is exactly
    // what is left of the CharString; stop cleanly instead.
    if mask_length as usize > mask_bytes.len() {
        return None;
    }
    // One flag per stem, plus room for the last byte's padding bits.
    let mut mask = vec![false; stack.stem as usize + 7];
    for (byte, &bits) in mask_bytes[..mask_length as usize].iter().enumerate() {
        for bit in 0..8 {
            mask[byte * 8 + bit] = bits & (0x80 >> bit) != 0;
        }
    }
    callback_draw_setmask(outline, op == OP_CNTRMASK.0, &mask);
    stack.clear();
    Some(mask_length)
}

pub(super) fn op_vmoveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 1 {
        too_few_operands("op_vmoveto", OP_VMOVETO);
    } else {
        if stack.index > 1 {
            callback_draw_setwidth(outline, stack.top(2));
        }
        callback_draw_next_contour(outline);
        callback_draw_lineto(outline, 0.0, stack.top(1));
        stack.clear();
    }
    Flow::Continue
}

pub(super) fn op_rmoveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 2 {
        too_few_operands("op_rmoveto", OP_RMOVETO);
    } else {
        if stack.index > 2 {
            callback_draw_setwidth(outline, stack.top(3));
        }
        callback_draw_next_contour(outline);
        callback_draw_lineto(outline, stack.top(2), stack.top(1));
        stack.clear();
    }
    Flow::Continue
}

pub(super) fn op_hmoveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 1 {
        too_few_operands("op_hmoveto", OP_HMOVETO);
    } else {
        if stack.index > 1 {
            callback_draw_setwidth(outline, stack.top(2));
        }
        callback_draw_next_contour(outline);
        callback_draw_lineto(outline, stack.top(1), 0.0);
        stack.clear();
    }
    Flow::Continue
}

pub(super) fn op_endchar(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index > 0 {
        callback_draw_setwidth(outline, stack.top(1));
    }
    Flow::Continue
}

/// End of the operands that form complete `group`-sized groups: the
/// line/curve operators below draw one segment per group and drop an
/// incomplete trailing group (as FreeType does) instead of reading slots
/// past `index`, which hold stale values from earlier operators or glyphs
/// -- or, near the end of the operand stack, nothing at all.
pub(super) fn complete_groups_end(index: Arity, group: Arity) -> Arity {
    index - index % group
}

/// `{dxa dya}+ rlineto`
pub(super) fn op_rlineto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    for i in (0..complete_groups_end(stack.index, 2)).step_by(2) {
        callback_draw_lineto(outline, stack.num(i), stack.num(i + 1));
    }
    stack.clear();
    Flow::Continue
}

/// Alternating vertical and horizontal lines, the first one vertical
/// (`vlineto`) or horizontal (`hlineto`): one operand per line.
pub(super) fn alternating_lines(stack: &mut CffStack, outline: &mut OutlineBuilderContext, first_vertical: bool) {
    let line = |outline: &mut OutlineBuilderContext, d: f64, vertical: bool| {
        if vertical {
            callback_draw_lineto(outline, 0.0, d);
        } else {
            callback_draw_lineto(outline, d, 0.0);
        }
    };
    // An odd count starts with a lone line; the rest come in pairs.
    let mut vertical = first_vertical;
    let mut start = 0;
    if stack.index % 2 == 1 {
        line(outline, stack.num(0), vertical);
        vertical = !vertical;
        start = 1;
    }
    for i in (start..stack.index).step_by(2) {
        line(outline, stack.num(i), vertical);
        line(outline, stack.num(i + 1), !vertical);
    }
    stack.clear();
}

/// `dy1 {dxa dyb}* vlineto` or `{dya dxb}+ vlineto`
pub(super) fn op_vlineto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    alternating_lines(stack, outline, true);
    Flow::Continue
}

/// `dx1 {dya dxb}* hlineto` or `{dxa dyb}+ hlineto`
pub(super) fn op_hlineto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    alternating_lines(stack, outline, false);
    Flow::Continue
}

/// Draws the curve whose six operands start at `i`.
pub(super) fn curve_at(stack: &CffStack, outline: &mut OutlineBuilderContext, i: Arity) {
    callback_draw_curveto(
        outline,
        stack.num(i),
        stack.num(i + 1),
        stack.num(i + 2),
        stack.num(i + 3),
        stack.num(i + 4),
        stack.num(i + 5),
    );
}

/// `{dxa dya dxb dyb dxc dyc}+ rrcurveto`
pub(super) fn op_rrcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    for i in (0..complete_groups_end(stack.index, 6)).step_by(6) {
        curve_at(stack, outline, i);
    }
    stack.clear();
    Flow::Continue
}

/// `{dxa dya dxb dyb dxc dyc}+ dxd dyd rcurveline`
pub(super) fn op_rcurveline(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 2 {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for op_rcurveline (24). This operation is ignored.\n");
    } else {
        for i in (0..complete_groups_end(stack.index - 2, 6)).step_by(6) {
            curve_at(stack, outline, i);
        }
        callback_draw_lineto(outline, stack.top(2), stack.top(1));
    }
    stack.clear();
    Flow::Continue
}

/// `{dxa dya}+ dxb dyb dxc dyc dxd dyd rlinecurve`
pub(super) fn op_rlinecurve(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 6 {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for op_rlinecurve (25). This operation is ignored.\n");
    } else {
        let curve = stack.index - 6;
        for i in (0..curve).step_by(2) {
            callback_draw_lineto(outline, stack.num(i), stack.num(i + 1));
        }
        curve_at(stack, outline, curve);
    }
    stack.clear();
    Flow::Continue
}

/// `dx1? {dya dxb dyb dyc}+ vvcurveto` (`vertical`) or
/// `dy1? {dxa dxb dyb dxc}+ hhcurveto`: curves that start and end in the
/// same direction, four operands each. An odd leading operand offsets the
/// first curve's start across that direction.
pub(super) fn same_direction_curves(stack: &mut CffStack, outline: &mut OutlineBuilderContext, vertical: bool) {
    let curve = |outline: &mut OutlineBuilderContext, across: f64, i: Arity| {
        let (d1, x2, y2, d3) = (stack.num(i), stack.num(i + 1), stack.num(i + 2), stack.num(i + 3));
        if vertical {
            callback_draw_curveto(outline, across, d1, x2, y2, 0.0, d3);
        } else {
            callback_draw_curveto(outline, d1, across, x2, y2, d3, 0.0);
        }
    };
    let mut start = 0;
    if stack.index % 4 == 1 {
        curve(outline, stack.num(0), 1);
        start = 5;
    }
    for i in (start..start + complete_groups_end(stack.index - start, 4)).step_by(4) {
        curve(outline, 0.0, i);
    }
    stack.clear();
}

pub(super) fn op_vvcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    // `index == 1` is the leading odd operand with no curve after it:
    // nothing to draw (the curve below reads slots 1..=4).
    if stack.index == 1 {
        stack.clear();
        return Flow::Continue;
    }
    same_direction_curves(stack, outline, true);
    Flow::Continue
}

pub(super) fn op_hhcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    // `index == 1` is the leading odd operand with no curve after it:
    // nothing to draw (the curve below reads slots 1..=4).
    if stack.index == 1 {
        stack.clear();
        return Flow::Continue;
    }
    same_direction_curves(stack, outline, false);
    Flow::Continue
}

/// `{dya dxb dyb dxc}+ df?` (`vhcurveto`, first curve starting vertical)
/// or `{dxa dxb dyb dyc}+ df?` (`hvcurveto`, starting horizontal): curves
/// that alternate between starting vertical and starting horizontal, four
/// operands each. An odd trailing operand `df` is the last curve's final
/// coordinate across its end direction.
pub(super) fn alternating_curves(stack: &mut CffStack, outline: &mut OutlineBuilderContext, first_vertical: bool) {
    let has_final = stack.index % 4 == 1;
    let curves = (if has_final { stack.index - 5 } else { stack.index }) / 4;
    let mut vertical = first_vertical;
    for k in 0..curves {
        let i = 4 * k;
        let (d1, x2, y2, d3) = (stack.num(i), stack.num(i + 1), stack.num(i + 2), stack.num(i + 3));
        if vertical {
            callback_draw_curveto(outline, 0.0, d1, x2, y2, d3, 0.0);
        } else {
            callback_draw_curveto(outline, d1, 0.0, x2, y2, 0.0, d3);
        }
        vertical = !vertical;
    }
    if has_final {
        let (d1, x2, y2, d3, df) = (stack.top(5), stack.top(4), stack.top(3), stack.top(2), stack.top(1));
        if vertical {
            callback_draw_curveto(outline, 0.0, d1, x2, y2, d3, df);
        } else {
            callback_draw_curveto(outline, d1, 0.0, x2, y2, df, d3);
        }
    }
}

pub(super) fn op_vhcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    // `index % 4 == 1` with `index < 5` means exactly `index == 1`: a lone
    // coordinate with no complete curve to pair it with.
    if stack.index == 1 {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for op_vhcurveto (30). This operation is ignored.\n");
    } else {
        alternating_curves(stack, outline, true);
    }
    stack.clear();
    Flow::Continue
}

pub(super) fn op_hvcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    // See `op_vhcurveto`.
    if stack.index == 1 {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for op_hvcurveto (31). This operation is ignored.\n");
    } else {
        alternating_curves(stack, outline, false);
    }
    stack.clear();
    Flow::Continue
}

/// `dx1 dx2 dy2 dx3 dx4 dx5 dx6 hflex`
pub(super) fn op_hflex(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 7 {
        too_few_operands("op_hflex", OP_HFLEX);
    } else {
        let [dx1, dx2, dy2, dx3, dx4, dx5, dx6] = std::array::from_fn(|i| stack.num(i as Arity));
        callback_draw_curveto(outline, dx1, 0.0, dx2, dy2, dx3, 0.0);
        callback_draw_curveto(outline, dx4, 0.0, dx5, -dy2, dx6, 0.0);
        stack.clear();
    }
    Flow::Continue
}

/// `dx1 dy1 dx2 dy2 dx3 dy3 dx4 dy4 dx5 dy5 dx6 dy6 fd flex` (`fd`, the
/// flex depth, is ignored: the curves are always drawn).
pub(super) fn op_flex(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 12 {
        too_few_operands("op_flex", OP_FLEX);
    } else {
        curve_at(stack, outline, 0);
        curve_at(stack, outline, 6);
        stack.clear();
    }
    Flow::Continue
}

/// `dx1 dy1 dx2 dy2 dx3 dx4 dx5 dy5 dx6 hflex1`: the curves end at the
/// starting height.
pub(super) fn op_hflex1(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 9 {
        too_few_operands("op_hflex1", OP_HFLEX1);
    } else {
        let [dx1, dy1, dx2, dy2, dx3, dx4, dx5, dy5, dx6] = std::array::from_fn(|i| stack.num(i as Arity));
        callback_draw_curveto(outline, dx1, dy1, dx2, dy2, dx3, 0.0);
        callback_draw_curveto(outline, dx4, 0.0, dx5, dy5, dx6, -(dy1 + dy2 + dy5));
        stack.clear();
    }
    Flow::Continue
}

/// `dx1 dy1 dx2 dy2 dx3 dy3 dx4 dy4 dx5 dy5 d6 flex1`: `d6` is the last
/// point's offset along whichever axis the curves travel further on; it
/// returns to the start along the other.
pub(super) fn op_flex1(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 11 {
        too_few_operands("op_flex1", OP_FLEX1);
    } else {
        let dx = stack.num(0) + stack.num(2) + stack.num(4) + stack.num(6) + stack.num(8);
        let dy = stack.num(1) + stack.num(3) + stack.num(5) + stack.num(7) + stack.num(9);
        let d6 = stack.num(10);
        let (dx6, dy6) = if dx.abs() > dy.abs() { (d6, -dy) } else { (-dx, d6) };
        curve_at(stack, outline, 0);
        callback_draw_curveto(outline, stack.num(6), stack.num(7), stack.num(8), stack.num(9), dx6, dy6);
        stack.clear();
    }
    Flow::Continue
}
