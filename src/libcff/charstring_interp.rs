//! The Type 2 CharString interpreter: runs one glyph's CharString (and the
//! subroutines it calls) and draws the result through the
//! `OutlineBuilderContext` callbacks in `table/cff.rs`.
//!
//! `parse_outline` reads tokens and pushes operands; each operator is
//! one `op_*` function below, named after its `OP_*` constant, and families
//! of operators that differ only in direction or arity share a helper. An
//! operator returns [`Flow::Stop`] when the rest of the outline must be
//! ignored (operand stack overflow, malformed hint mask data).

use crate::logger::ByteStr;
use crate::libcff::codecs::decode_cs2_token;
use crate::libcff::index::CffIndex;
use crate::libcff::value::{CffValue, cffnum};
use crate::support::primitives::Arity;
use crate::table::cff::{
    OutlineBuilderContext, callback_draw_curveto, callback_draw_getrand, callback_draw_lineto,
    callback_draw_next_contour, callback_draw_sethint, callback_draw_setmask, callback_draw_setwidth,
};
use crate::libcff::parser::{
    MAX_SUBR_CALL_DEPTH, MAX_TOTAL_SUBR_CALLS, compute_subr_bias, locate_subr,
};
use crate::libcff::{CffCharstringOperator, CffStack, TYPE2_TRANSIENT_ARRAY, OP_ABS, OP_ADD, OP_AND, OP_CALLGSUBR, OP_CALLSUBR, OP_CNTRMASK, OP_DIV, OP_DROP, OP_DUP, OP_ENDCHAR, OP_EQ, OP_EXCH, OP_FLEX, OP_FLEX1, OP_GET, OP_HFLEX, OP_HFLEX1, OP_HHCURVETO, OP_HINTMASK, OP_HLINETO, OP_HMOVETO, OP_HSTEM, OP_HSTEMHM, OP_HVCURVETO, OP_IFELSE, OP_INDEX, OP_MUL, OP_NEG, OP_NOT, OP_OR, OP_PUT, OP_RANDOM, OP_RCURVELINE, OP_RETURN, OP_RLINECURVE, OP_RLINETO, OP_RMOVETO, OP_ROLL, OP_RRCURVETO, OP_SQRT, OP_SUB, OP_VHCURVETO, OP_VLINETO, OP_VMOVETO, OP_VSTEM, OP_VSTEMHM, OP_VVCURVETO};

/// What the interpreter does after an operator.
enum Flow {
    Continue,
    Stop,
}

/// The two subroutine INDEXes a CharString can call into, with their biases.
struct Subroutines<'a> {
    gsubr: &'a CffIndex,
    lsubr: &'a CffIndex,
    gsubr_bias: u16,
    lsubr_bias: u16,
}

/// Operand access for the operators below. `index` is the number of
/// operands pushed; every caller checks it covers the slots it reads.
impl CffStack {
    /// The operand at `i`, counting from the bottom of the stack.
    fn num(&self, i: Arity) -> f64 {
        cffnum(self.stack[i as usize])
    }

    /// The operand `n` places from the top: `top(1)` was pushed last.
    fn top(&self, n: Arity) -> f64 {
        cffnum(self.stack[(self.index - n) as usize])
    }

    /// Replaces the operand `n` places from the top with the number `value`.
    fn set_top(&mut self, n: Arity, value: f64) {
        self.stack[(self.index - n) as usize] = CffValue::Double(value);
    }

    /// Pops every operand (path and hint operators consume the whole stack).
    fn clear(&mut self) {
        self.index = 0;
    }

    /// Whether another operand fits on the stack.
    fn has_room(&self) -> bool {
        (self.index as usize) < self.stack.len()
    }

    /// Pushes `value`; the caller has checked `has_room`.
    fn push(&mut self, value: CffValue) {
        self.stack[self.index as usize] = value;
        self.index += 1;
    }
}

/// Runs the CharString `data` and draws it into `outline`. Each subroutine
/// call runs this again on the same stack and outline, with `depth` one
/// deeper; `total_calls` counts the subroutine calls made for this glyph.
pub fn parse_outline(
    data: &[u8],
    gsubr: &CffIndex,
    lsubr: &CffIndex,
    stack: &mut CffStack,
    outline: &mut OutlineBuilderContext,
    depth: u32,
    total_calls: &mut u32,
) {
    if depth > MAX_SUBR_CALL_DEPTH {
        tracing::warn!("[libcff] Subroutine call nesting exceeded {}; the rest of this outline is ignored.\n", MAX_SUBR_CALL_DEPTH);
        return;
    }
    let subrs = Subroutines {
        gsubr,
        lsubr,
        gsubr_bias: compute_subr_bias(gsubr.count as u16),
        lsubr_bias: compute_subr_bias(lsubr.count as u16),
    };
    let mut pos = 0;
    let mut val = CffValue::Unset;
    while pos < data.len() {
        // A token starting near the end of a truncated CharString would
        // run past it; stop instead of reading on. (`op_hint_mask` bounds
        // its mask bytes, which follow the operator outside any token, the
        // same way.)
        let Some(token_length) = decode_cs2_token(&data[pos..], &mut val) else {
            break;
        };
        let mut advance = token_length as usize;
        match val {
            CffValue::Operator(op) => {
                let flow = match CffCharstringOperator(op) {
                    OP_HSTEM | OP_VSTEM | OP_HSTEMHM | OP_VSTEMHM => op_stem_hints(stack, outline, op),
                    OP_HINTMASK | OP_CNTRMASK => {
                        // The mask bytes follow the operator directly.
                        let mask_bytes = data.get(pos + advance..).unwrap_or(&[]);
                        match op_hint_mask(stack, outline, op, mask_bytes) {
                            Some(mask_length) => {
                                advance += mask_length as usize;
                                Flow::Continue
                            }
                            None => Flow::Stop,
                        }
                    }
                    OP_VMOVETO => op_vmoveto(stack, outline),
                    OP_RMOVETO => op_rmoveto(stack, outline),
                    OP_HMOVETO => op_hmoveto(stack, outline),
                    OP_ENDCHAR => op_endchar(stack, outline),
                    OP_RLINETO => op_rlineto(stack, outline),
                    OP_VLINETO => op_vlineto(stack, outline),
                    OP_HLINETO => op_hlineto(stack, outline),
                    OP_RRCURVETO => op_rrcurveto(stack, outline),
                    OP_RCURVELINE => op_rcurveline(stack, outline),
                    OP_RLINECURVE => op_rlinecurve(stack, outline),
                    OP_VVCURVETO => op_vvcurveto(stack, outline),
                    OP_HHCURVETO => op_hhcurveto(stack, outline),
                    OP_VHCURVETO => op_vhcurveto(stack, outline),
                    OP_HVCURVETO => op_hvcurveto(stack, outline),
                    OP_HFLEX => op_hflex(stack, outline),
                    OP_FLEX => op_flex(stack, outline),
                    OP_HFLEX1 => op_hflex1(stack, outline),
                    OP_FLEX1 => op_flex1(stack, outline),
                    OP_AND => op_and(stack),
                    OP_OR => op_or(stack),
                    OP_NOT => op_not(stack),
                    OP_ABS => op_abs(stack),
                    OP_ADD => op_add(stack),
                    OP_SUB => op_sub(stack),
                    OP_DIV => op_div(stack),
                    OP_NEG => op_neg(stack),
                    OP_EQ => op_eq(stack),
                    OP_DROP => op_drop(stack),
                    OP_PUT => op_put(stack),
                    OP_GET => op_get(stack),
                    OP_IFELSE => op_ifelse(stack),
                    OP_RANDOM => op_random(stack, outline),
                    OP_MUL => op_mul(stack),
                    OP_SQRT => op_sqrt(stack),
                    OP_DUP => op_dup(stack),
                    OP_EXCH => op_exch(stack),
                    OP_INDEX => op_index(stack),
                    OP_ROLL => op_roll(stack),
                    OP_CALLSUBR => op_callsubr(stack, outline, &subrs, depth, total_calls),
                    OP_CALLGSUBR => op_callgsubr(stack, outline, &subrs, depth, total_calls),
                    OP_RETURN => Flow::Stop,
                    _ => {
                        tracing::warn!("Warning: unknown operator {} occurs in Type 2 CharString. It may caused by file corruption.", op);
                        Flow::Stop
                    }
                };
                if let Flow::Stop = flow {
                    return;
                }
            }
            CffValue::Integer(_) | CffValue::Double(_) => {
                if stack.has_room() {
                    stack.push(val);
                } else {
                    stack_overflow();
                    return;
                }
            }
            CffValue::Unset => {}
        }
        pos += advance;
    }
}

/// Reads the stem hints on the stack: pairs of `edge width`, each edge
/// relative to the end of the previous stem. An odd operand count means
/// the first operand is the glyph's advance width.
fn read_stem_hints(stack: &mut CffStack, outline: &mut OutlineBuilderContext, vertical: bool) {
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
fn op_stem_hints(stack: &mut CffStack, outline: &mut OutlineBuilderContext, op: i32) -> Flow {
    read_stem_hints(stack, outline, op == OP_VSTEM.0 || op == OP_VSTEMHM.0);
    stack.clear();
    Flow::Continue
}

/// `hintmask`/`cntrmask`: one bit per stem hint, in bytes that follow the
/// operator in the CharString. Operands before it are stem hints too:
/// vertical ones when some stems were declared already (the implicit
/// `vstem` after `hstem`s), horizontal otherwise. Returns the number of
/// mask bytes read, or `None` when the CharString ends before them.
fn op_hint_mask(stack: &mut CffStack, outline: &mut OutlineBuilderContext, op: i32, mask_bytes: &[u8]) -> Option<u32> {
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

fn op_vmoveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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

fn op_rmoveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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

fn op_hmoveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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

fn op_endchar(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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
fn complete_groups_end(index: Arity, group: Arity) -> Arity {
    index - index % group
}

/// `{dxa dya}+ rlineto`
fn op_rlineto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    for i in (0..complete_groups_end(stack.index, 2)).step_by(2) {
        callback_draw_lineto(outline, stack.num(i), stack.num(i + 1));
    }
    stack.clear();
    Flow::Continue
}

/// Alternating vertical and horizontal lines, the first one vertical
/// (`vlineto`) or horizontal (`hlineto`): one operand per line.
fn alternating_lines(stack: &mut CffStack, outline: &mut OutlineBuilderContext, first_vertical: bool) {
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
fn op_vlineto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    alternating_lines(stack, outline, true);
    Flow::Continue
}

/// `dx1 {dya dxb}* hlineto` or `{dxa dyb}+ hlineto`
fn op_hlineto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    alternating_lines(stack, outline, false);
    Flow::Continue
}

/// Draws the curve whose six operands start at `i`.
fn curve_at(stack: &CffStack, outline: &mut OutlineBuilderContext, i: Arity) {
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
fn op_rrcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    for i in (0..complete_groups_end(stack.index, 6)).step_by(6) {
        curve_at(stack, outline, i);
    }
    stack.clear();
    Flow::Continue
}

/// `{dxa dya dxb dyb dxc dyc}+ dxd dyd rcurveline`
fn op_rcurveline(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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
fn op_rlinecurve(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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
fn same_direction_curves(stack: &mut CffStack, outline: &mut OutlineBuilderContext, vertical: bool) {
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

fn op_vvcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    // `index == 1` is the leading odd operand with no curve after it:
    // nothing to draw (the curve below reads slots 1..=4).
    if stack.index == 1 {
        stack.clear();
        return Flow::Continue;
    }
    same_direction_curves(stack, outline, true);
    Flow::Continue
}

fn op_hhcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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
fn alternating_curves(stack: &mut CffStack, outline: &mut OutlineBuilderContext, first_vertical: bool) {
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

fn op_vhcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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

fn op_hvcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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
fn op_hflex(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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
fn op_flex(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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
fn op_hflex1(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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
fn op_flex1(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
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

fn too_few_operands(name: &str, op: CffCharstringOperator) {
    tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr(name), op.0 as u32);
}

fn stack_overflow() {
    tracing::warn!("[libcff] Operand stack overflow in Type 2 CharString; the rest of this outline is ignored.\n");
}

fn truth(condition: bool) -> f64 {
    if condition { 1.0 } else { 0.0 }
}

/// Replaces the top operand `a` with `f(a)`.
fn unary(stack: &mut CffStack, name: &str, op: CffCharstringOperator, f: impl FnOnce(f64) -> f64) -> Flow {
    if stack.index < 1 {
        too_few_operands(name, op);
    } else {
        let a = stack.top(1);
        stack.set_top(1, f(a));
    }
    Flow::Continue
}

/// Replaces the top two operands `a b` (`b` pushed last) with `f(a, b)`.
fn binary(stack: &mut CffStack, name: &str, op: CffCharstringOperator, f: impl FnOnce(f64, f64) -> f64) -> Flow {
    if stack.index < 2 {
        too_few_operands(name, op);
    } else {
        let (a, b) = (stack.top(2), stack.top(1));
        stack.set_top(2, f(a, b));
        stack.index -= 1;
    }
    Flow::Continue
}

fn op_and(stack: &mut CffStack) -> Flow {
    binary(stack, "op_and", OP_AND, |a, b| truth(b != 0.0 && a != 0.0))
}

fn op_or(stack: &mut CffStack) -> Flow {
    binary(stack, "op_or", OP_OR, |a, b| truth(b != 0.0 || a != 0.0))
}

fn op_not(stack: &mut CffStack) -> Flow {
    unary(stack, "op_not", OP_NOT, |a| truth(a == 0.0))
}

fn op_abs(stack: &mut CffStack) -> Flow {
    // Not `f64::abs`: that would also clear the sign of `-0.0` and NaN.
    unary(stack, "op_abs", OP_ABS, |a| if a < 0.0 { -a } else { a })
}

fn op_add(stack: &mut CffStack) -> Flow {
    binary(stack, "op_add", OP_ADD, |a, b| b + a)
}

fn op_sub(stack: &mut CffStack) -> Flow {
    binary(stack, "op_sub", OP_SUB, |a, b| a - b)
}

fn op_div(stack: &mut CffStack) -> Flow {
    binary(stack, "op_div", OP_DIV, |a, b| a / b)
}

fn op_neg(stack: &mut CffStack) -> Flow {
    unary(stack, "op_neg", OP_NEG, |a| -a)
}

fn op_eq(stack: &mut CffStack) -> Flow {
    binary(stack, "op_eq", OP_EQ, |a, b| truth(b == a))
}

fn op_mul(stack: &mut CffStack) -> Flow {
    binary(stack, "op_mul", OP_MUL, |a, b| b * a)
}

fn op_sqrt(stack: &mut CffStack) -> Flow {
    unary(stack, "op_sqrt", OP_SQRT, |a| a.sqrt())
}

fn op_drop(stack: &mut CffStack) -> Flow {
    if stack.index < 1 {
        too_few_operands("op_drop", OP_DROP);
    } else {
        stack.index -= 1;
    }
    Flow::Continue
}

/// The transient array slot an operand names. Real CharStrings only use
/// small in-range indices; `rem_euclid` (never negative, unlike `%`) maps
/// anything else to some slot instead of an out-of-bounds index.
fn transient_slot(i: f64) -> usize {
    (i as i32).rem_euclid(TYPE2_TRANSIENT_ARRAY as i32) as usize
}

/// `val i put`
fn op_put(stack: &mut CffStack) -> Flow {
    if stack.index < 2 {
        too_few_operands("op_put", OP_PUT);
    } else {
        stack.transient[transient_slot(stack.top(1))] = CffValue::Double(stack.top(2));
        stack.index -= 2;
    }
    Flow::Continue
}

/// `i get`
fn op_get(stack: &mut CffStack) -> Flow {
    if stack.index < 1 {
        too_few_operands("op_get", OP_GET);
    } else {
        let value = cffnum(stack.transient[transient_slot(stack.top(1))]);
        stack.set_top(1, value);
    }
    Flow::Continue
}

/// `s1 s2 v1 v2 ifelse`: `s1` if `v1 <= v2`, else `s2`.
fn op_ifelse(stack: &mut CffStack) -> Flow {
    if stack.index < 4 {
        too_few_operands("op_ifelse", OP_IFELSE);
    } else {
        let (s1, s2, v1, v2) = (stack.top(4), stack.top(3), stack.top(2), stack.top(1));
        stack.set_top(4, if v1 <= v2 { s1 } else { s2 });
        stack.index -= 3;
    }
    Flow::Continue
}

fn op_random(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    // Check for room before drawing the number, so a full stack leaves the
    // generator's state untouched.
    if !stack.has_room() {
        stack_overflow();
        return Flow::Stop;
    }
    stack.push(CffValue::Double(callback_draw_getrand(outline)));
    Flow::Continue
}

fn op_dup(stack: &mut CffStack) -> Flow {
    if stack.index < 1 {
        too_few_operands("op_dup", OP_DUP);
    } else if stack.has_room() {
        stack.push(stack.stack[stack.index as usize - 1]);
    } else {
        stack_overflow();
        return Flow::Stop;
    }
    Flow::Continue
}

fn op_exch(stack: &mut CffStack) -> Flow {
    if stack.index < 2 {
        too_few_operands("op_exch", OP_EXCH);
    } else {
        let (a, b) = (stack.top(2), stack.top(1));
        stack.set_top(1, a);
        stack.set_top(2, b);
    }
    Flow::Continue
}

/// `num(N-1) ... num(0) i index`: replaces `i` with a copy of `num(i)`,
/// the operand `i` places below it (`num(0)` when `i` is negative). An `i`
/// past the bottom of the stack is out of range and ignored.
fn op_index(stack: &mut CffStack) -> Flow {
    if stack.index < 2 {
        too_few_operands("op_index", OP_INDEX);
    } else {
        let i = stack.top(1);
        let below = stack.index - 1;
        let depth = if i < 0.0 { 0 } else { i as Arity };
        if depth >= below {
            too_few_operands("op_index", OP_INDEX);
        } else {
            stack.stack[below as usize] = stack.stack[(below - 1 - depth) as usize];
        }
    }
    Flow::Continue
}

/// `num(N-1) ... num(0) N J roll`: rotates the `N` operands below `N J` by
/// `J` places toward the top (toward the bottom for a negative `J`). `N`
/// and `J` are always popped; a negative `N` rolls nothing.
fn op_roll(stack: &mut CffStack) -> Flow {
    if stack.index < 2 {
        too_few_operands("op_roll", OP_ROLL);
        return Flow::Continue;
    }
    let j = stack.top(1) as i64;
    let n = stack.top(2) as Arity;
    stack.index -= 2;
    if n > stack.index {
        too_few_operands("op_roll", OP_ROLL);
    } else if n > 0 {
        let end = stack.index as usize;
        let rolled = &mut stack.stack[end - n as usize..end];
        rolled.rotate_right(j.rem_euclid(i64::from(n)) as usize);
    }
    Flow::Continue
}

/// `subr# callsubr` / `subr# callgsubr`: runs a local or global subroutine
/// on the same stack and outline.
fn call_subroutine(
    stack: &mut CffStack,
    outline: &mut OutlineBuilderContext,
    subrs: &Subroutines<'_>,
    global: bool,
    depth: u32,
    total_calls: &mut u32,
) -> Flow {
    let (name, op, kind, index, bias) = if global {
        ("op_callgsubr", OP_CALLGSUBR, "global", subrs.gsubr, subrs.gsubr_bias)
    } else {
        ("op_callsubr", OP_CALLSUBR, "local", subrs.lsubr, subrs.lsubr_bias)
    };
    if stack.index < 1 {
        too_few_operands(name, op);
        return Flow::Continue;
    }
    stack.index -= 1;
    // `as i32`, not `as u32`: subroutine numbers are signed, and a
    // float-to-unsigned cast turns every negative one into 0.
    let number = stack.num(stack.index) as i32;
    let Some(sub_data) = locate_subr(index, bias, number) else {
        tracing::warn!("[libcff] Invalid {} subroutine index for {} ({:04x}). This call is ignored.\n", kind, ByteStr(name), op.0 as u32);
        return Flow::Continue;
    };
    *total_calls = total_calls.wrapping_add(1);
    if *total_calls > MAX_TOTAL_SUBR_CALLS {
        if *total_calls == MAX_TOTAL_SUBR_CALLS + 1 {
            tracing::warn!("[libcff] Subroutine call budget ({}) exceeded; the rest of this outline is ignored.\n", MAX_TOTAL_SUBR_CALLS);
        }
    } else {
        parse_outline(sub_data, subrs.gsubr, subrs.lsubr, stack, outline, depth + 1, total_calls);
    }
    Flow::Continue
}

fn op_callsubr(stack: &mut CffStack, outline: &mut OutlineBuilderContext, subrs: &Subroutines<'_>, depth: u32, total_calls: &mut u32) -> Flow {
    call_subroutine(stack, outline, subrs, false, depth, total_calls)
}

fn op_callgsubr(stack: &mut CffStack, outline: &mut OutlineBuilderContext, subrs: &Subroutines<'_>, depth: u32, total_calls: &mut u32) -> Flow {
    call_subroutine(stack, outline, subrs, true, depth, total_calls)
}

#[cfg(test)]
mod cff_parse_outline_total_calls_tests;

#[cfg(test)]
mod cff_parse_outline_hintmask_tests;

#[cfg(test)]
mod cff_parse_outline_stack_operator_tests;

#[cfg(test)]
mod cff_parse_outline_subr_number_tests;

#[cfg(test)]
mod cff_parse_outline_operand_group_tests;
