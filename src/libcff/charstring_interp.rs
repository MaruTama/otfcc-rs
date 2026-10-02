//! The Type 2 CharString interpreter: runs one glyph's CharString (and the
//! subroutines it calls) and draws the result through the
//! `OutlineBuilderContext` callbacks in `table/cff.rs`.
//!
//! `cff_parse_outline` reads tokens and pushes operands; each operator is
//! one `op_*` function below, named after its `OP_*` constant, and families
//! of operators that differ only in direction or arity share a helper. An
//! operator returns [`Flow::Stop`] when the rest of the outline must be
//! ignored (operand stack overflow, malformed hint mask data).

use crate::logger::ByteStr;
use crate::libcff::cff_codecs::cff_decode_cs2_token;
use crate::libcff::cff_index::CffIndex;
use crate::libcff::cff_value::{CffValue, cffnum};
use crate::support::primitives::Arity;
use crate::table::cff::{
    OutlineBuilderContext, callback_draw_curveto, callback_draw_getrand, callback_draw_lineto,
    callback_draw_next_contour, callback_draw_sethint, callback_draw_setmask, callback_draw_setwidth,
};
use crate::libcff::cff_parser::{
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
pub fn cff_parse_outline(
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
        let Some(token_length) = cff_decode_cs2_token(&data[pos..], &mut val) else {
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
    // The mask bytes never go through `cff_decode_cs2_token`'s bounds
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
        rolled.rotate_right(j.rem_euclid(n as i64) as usize);
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
        cff_parse_outline(sub_data, subrs.gsubr, subrs.lsubr, stack, outline, depth + 1, total_calls);
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
mod cff_parse_outline_total_calls_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;

    use crate::table::glyf::{Glyph, new_glyf_glyph};

    fn empty_cff_index() -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        }
    }

    fn dummy_outline_context(g: &mut Glyph) -> OutlineBuilderContext<'_> {
        OutlineBuilderContext {
            g,
            j_contour: 0,
            j_point: 0,
            default_width_x: 0.0,
            nominal_width_x: 0.0,
            defined_h_stems: 0,
            defined_v_stems: 0,
            defined_hint_masks: 0,
            defined_contour_masks: 0,
            randx: 0,
        }
    }

    // 108 global subroutines: the first 107 are empty (`offset[i] ==
    // offset[i + 1]`, a valid zero-length INDEX entry -- never invoked by
    // this test), the last is "push operand 0 (byte 139), return (11)".
    // With this index's count (108, `compute_subr_bias`'s bias-107
    // bracket), pushing operand 0 before `callgsubr` resolves to `bias +
    // 0` = index 107, this index's own last entry -- deliberately avoids
    // ever needing a *negative* pushed operand (which real fonts use to
    // reach a low subroutine index): `cffnum(...) as u32`, downstream of
    // this in `callgsubr`'s own handling, is a float-to-int cast, and
    // Rust's saturates a negative float to `0` rather than wrapping the
    // way the C original's cast did, so a negative operand here would
    // resolve to index `bias + 0` = 107 anyway, not to the small index it
    // looks like it should -- a real, pre-existing quirk of this already-
    // migrated cast, unrelated to this test's own purpose, sidestepped
    // instead of exercised.
    //
    // The subroutine's pushed operand is never popped by anything
    // (`return` doesn't touch the stack), so `stack.index` at the end
    // counts exactly how many times this subroutine actually *ran* -- the
    // caller's own operand push for the `callgsubr` index is always
    // popped by `callgsubr` itself before the recursion decision, so a
    // *skipped* call leaves no trace on the stack at all.
    fn one_trivial_gsubr() -> CffIndex {
        let mut offset = vec![1u32; 108];
        offset.push(3);
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 108,
            off_size: 1,
            offset,
            data: vec![139, 11], // push (byte 139 => operand 0), return
        }
    }

    // `n` copies of "push operand 0 (byte 139); callgsubr (29)" -- 2 bytes
    // each, never recursing past nesting depth 1 (see `one_trivial_gsubr`).
    fn charstring_calling_gsubr_n_times(n: u32) -> Vec<u8> {
        let mut b = Vec::with_capacity(n as usize * 2);
        for _ in 0..n {
            b.push(139); // encodes operand 0
            b.push(29); // callgsubr
        }
        b
    }

    #[test]
    // `requested` has to be within one of the real `MAX_TOTAL_SUBR_CALLS`
    // (10,000) to prove the budget lets everything through -- unlike the
    // sibling stack-operator tests in this file, shrinking the backing
    // `CffStack.stack` allocation (still done below, real but minor) barely
    // moved this test's Miri time, because the actual cost is ~10,000
    // *recursive `cff_parse_outline` calls*, not the array size. That's
    // inherent to what this test proves, the same way
    // `total_language_count_across_the_whole_table_is_capped`
    // (`table/otl/read.rs`) can't shrink its own N below the real cap
    // either. `cargo test` (native) stays the real regression guard.
    #[cfg_attr(miri, ignore = "far too slow to run meaningfully under Miri's interpreter; needs ~10,000 recursive cff_parse_outline calls to exercise the real MAX_TOTAL_SUBR_CALLS budget")]
    fn call_count_within_the_budget_all_execute() {
        let gsubr = one_trivial_gsubr();
        let lsubr = empty_cff_index();
        let requested = MAX_TOTAL_SUBR_CALLS - 1;
        let data = charstring_calling_gsubr_n_times(requested);
        // 11_000 (not the real 0x10000/65536): still shrunk from
        // production's generous capacity down to just above `requested`'s
        // ~10,000 pushes, even though (per the ignore above) this isn't
        // what dominates this test's Miri time.
        let mut stack = CffStack {
            stack: vec![CffValue::Unset; 11_000],
            transient: [CffValue::Unset; TYPE2_TRANSIENT_ARRAY],
            index: 0,
            stem: 0,
        };
        let mut total_calls: u32 = 0;
        // This charstring only calls `callgsubr`; it never reaches a draw
        // operator, so the outline context is never actually touched --
        // still needs to be a real `&mut Glyph`-backed context now that
        // `cff_parse_outline` takes one unconditionally rather than a
        // nullable `*mut c_void`.
        let mut g = new_glyf_glyph();
        let mut ctx = dummy_outline_context(&mut g);
        cff_parse_outline(
            &data,
            &gsubr,
            &lsubr,
            &mut stack,
            &mut ctx,
            0,
            &mut total_calls,
        );
        assert_eq!(total_calls, requested);
        // Every one of the `requested` calls actually recursed and ran
        // its subroutine's own push.
        assert_eq!(stack.index, requested as Arity);
    }

    // The bug this pins: `MAX_SUBR_CALL_DEPTH` bounds how deep `callsubr`/
    // `callgsubr` can *nest*, but said nothing about how many calls happen
    // *within* one nesting level -- a subroutine graph with wide fan-out
    // at a shallow, spec-legal depth could still do unbounded total work.
    // `total_calls` (threaded through every recursive `cff_parse_outline`
    // call, shared across the whole glyph) is what actually bounds it.
    #[test]
    // Same Miri cost shape as the sibling test above: recursing up to
    // `MAX_TOTAL_SUBR_CALLS` (10,000) before stopping is the point of
    // this test, not something a smaller N could substitute for.
    #[cfg_attr(miri, ignore = "far too slow to run meaningfully under Miri's interpreter; needs ~10,000 recursive cff_parse_outline calls before the real MAX_TOTAL_SUBR_CALLS budget stops it")]
    fn call_count_past_the_budget_stops_recursing() {
        let gsubr = one_trivial_gsubr();
        let lsubr = empty_cff_index();
        let attempted = MAX_TOTAL_SUBR_CALLS + 500;
        let data = charstring_calling_gsubr_n_times(attempted);
        // Same reasoning as the sibling test above: recursion stops at
        // `MAX_TOTAL_SUBR_CALLS` regardless of `attempted`, so 11_000
        // still has headroom above every push this test can actually
        // reach, while avoiding 0x10000's full-capacity initialization
        // cost under Miri.
        let mut stack = CffStack {
            stack: vec![CffValue::Unset; 11_000],
            transient: [CffValue::Unset; TYPE2_TRANSIENT_ARRAY],
            index: 0,
            stem: 0,
        };
        let mut total_calls: u32 = 0;
        let mut g = new_glyf_glyph();
        let mut ctx = dummy_outline_context(&mut g);
        cff_parse_outline(
            &data,
            &gsubr,
            &lsubr,
            &mut stack,
            &mut ctx,
            0,
            &mut total_calls,
        );
        // The counter itself still climbs past the budget (every
        // `callgsubr` byte pair the outer loop walks over is one
        // attempted call, counted before the budget check decides
        // whether to recurse) -- charstring interpretation for the
        // rest of this glyph isn't aborted, only further recursion is.
        assert_eq!(total_calls, attempted);
        // But only the first `MAX_TOTAL_SUBR_CALLS` calls actually
        // recursed and ran their subroutine's own push -- this is the
        // bug's actual fix: without it, `stack.index` would reach
        // `attempted` too (each recursive call's own push landing on
        // the shared stack), the same as the within-budget test above,
        // with no way to tell the two cases apart from this assertion
        // alone.
        assert_eq!(stack.index, MAX_TOTAL_SUBR_CALLS as Arity);
    }
}

#[cfg(test)]
mod cff_parse_outline_hintmask_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;

    use crate::table::cff::OutlineBuilderContext;
    use crate::table::glyf::new_glyf_glyph;

    fn empty_cff_index() -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        }
    }

    // A fuzz-found font found this: `hintmask`/`cntrmask`'s mask bytes are
    // raw payload embedded directly in the charstring right after the
    // opcode -- unlike every other operand, they never go through
    // `cff_decode_cs2_token`'s own bounds checking, so nothing stopped
    // `mask_length` (driven by `(*stack).stem`, the accumulated hint count
    // from every `hstem`/`vstem` operator already seen in this charstring)
    // from reading past the actual CharString buffer -- an ASan-confirmed
    // heap-buffer-overflow.
    //
    // Charstring: push 0, push 0, `hstem` (one hint pair -> stem count
    // becomes 1, needing a 1-byte mask), `hintmask` -- with the mask byte
    // itself missing (the charstring ends right at the opcode). Reaching
    // the end of this function at all, rather than reading one byte past
    // `data`'s 4-byte allocation, is the regression signal.
    #[test]
    fn hintmask_past_the_charstring_end_stops_cleanly_instead_of_reading_oob() {
        let data: Vec<u8> = vec![139, 139, 1, 19];
        let gsubr = empty_cff_index();
        let lsubr = empty_cff_index();
        let mut stack = CffStack {
            stack: vec![CffValue::Unset; 512],
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
        cff_parse_outline(
            &data,
            &gsubr,
            &lsubr,
            &mut stack,
            &mut ctx,
            0,
            &mut total_calls,
        );
        // The `hstem` operator ran (and only it -- `hintmask` bailed
        // before doing anything observable) -- `stem` reflects the one
        // hint pair pushed before the truncated `hintmask`.
        assert_eq!(stack.stem, 1);
    }

    // A second, independent fuzz-found crash in this same op family: a
    // charstring chaining enough `hstem` operators to push the *real*
    // cumulative hint count (tracked by `context.g.stem_h`, an unbounded
    // `Vec`) past 255, while `(*stack).stem` -- back when it was a `u8`
    // used to size the `hintmask` bit array -- silently wrapped back down
    // to a small value at the same point. `callback_draw_setmask` then
    // indexed the undersized array using the real (large) `stem_h.len()`,
    // an out-of-bounds panic (`table/cff.rs`, CI-found: "index out of
    // bounds: the len is 74 but the index is 716").
    //
    // 256 single-hint `hstem` calls (push 0, push 0, `hstem`) push exactly
    // 256 real entries into `stem_h` -- old `u8` arithmetic wrapped
    // `255 + 1` back to `0`; `stem` is now `u32` and must read back the
    // true 256.
    #[test]
    fn chained_hstem_operators_past_255_do_not_wrap_the_hint_count() {
        let mut data: Vec<u8> = Vec::new();
        for _ in 0..256 {
            data.extend_from_slice(&[139, 139, 1]); // push 0, push 0, hstem
        }
        data.push(19); // hintmask
        // mask_length = (256 + 7) >> 3 = 32 bytes.
        data.extend_from_slice(&[0u8; 32]);
        let gsubr = empty_cff_index();
        let lsubr = empty_cff_index();
        let mut stack = CffStack {
            stack: vec![CffValue::Unset; 512],
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
        cff_parse_outline(
            &data,
            &gsubr,
            &lsubr,
            &mut stack,
            &mut ctx,
            0,
            &mut total_calls,
        );
        assert_eq!(ctx.g.stem_h.len(), 256);
        assert_eq!(stack.stem, 256);
    }
}

#[cfg(test)]
mod cff_parse_outline_stack_operator_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;

    use crate::table::glyf::new_glyf_glyph;

    // A charstring's `put`/`get`/`index`/`roll` operators each take a
    // charstring-supplied stack *value* (not the trusted `(*stack).index`
    // cursor) and use it as an array index or modulus divisor into a
    // small fixed-size structure (`transient[32]`, or the operand stack
    // itself), with no range check. Found by reading the interpreter
    // directly (not fuzzing) while investigating this file as the
    // successor to `cff_dict.rs`'s Private-DICT-offset fix (PR #262):
    // that fix closed an out-of-bounds *read*, these are guaranteed
    // Rust *panics* (array-index or divide-by-zero) reachable with a
    // handful of ordinary charstring bytes -- a different bug class
    // (DoS, not memory corruption), but real and previously unguarded.

    fn empty_cff_index() -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        }
    }

    // Real `CffStack.stack` is 0x10000 entries (matching the operand
    // stack's generous production capacity), but every test in this
    // module pushes at most 257 operands -- allocating and initializing
    // the full 65536-entry Vec added ~10s per test under Miri's
    // per-element provenance tracking (an otherwise-sub-10ms test suite
    // module took over a minute combined) for headroom none of these
    // tests use. 512 comfortably covers the largest case
    // (`op_index_with_operand_count_multiple_of_256_does_not_panic`'s
    // 257 pushes) while cutting the allocation two orders of magnitude;
    // the bugs these tests guard against are all about index-computation
    // correctness (negative wraparound, zero-divisor guards), not
    // anything sensitive to the backing array's total capacity.
    fn fresh_stack() -> CffStack {
        CffStack {
            stack: vec![CffValue::Unset; 512],
            transient: [CffValue::Unset; TYPE2_TRANSIENT_ARRAY],
            index: 0,
            stem: 0,
        }
    }

    fn run(data: &[u8], stack: &mut CffStack) {
        let gsubr = empty_cff_index();
        let lsubr = empty_cff_index();
        let mut total_calls: u32 = 0;
        // None of this module's `put`/`get`/`index`/`roll` charstrings
        // reach a draw operator -- still needs a real `&mut Glyph`-backed
        // context now that `cff_parse_outline` takes one unconditionally.
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
        cff_parse_outline(
            data,
            &gsubr,
            &lsubr,
            stack,
            &mut ctx,
            0,
            &mut total_calls,
        );
    }

    #[test]
    fn op_get_with_negative_index_operand_does_not_panic() {
        // `-1` (byte 138) then `get` (escape `12 21` = OP_GET). The
        // pre-fix `i_1 % TYPE2_TRANSIENT_ARRAY as i32` kept the
        // dividend's sign (Rust's `%`), so `i_1 == -1` produced a
        // negative remainder that panicked once cast `as usize` for the
        // `transient[]` index.
        let data: Vec<u8> = vec![138, 12, 21];
        let mut stack = fresh_stack();
        run(&data, &mut stack);
        // `-1` `rem_euclid` 32 == 31, a never-written transient slot --
        // `cffnum` reads that as 0.0. Reaching this assertion at all
        // (rather than panicking mid-parse) is the regression signal.
        assert_eq!(stack.index, 1);
        assert!(matches!(stack.stack[0], CffValue::Double(v) if v == 0.0));
    }

    #[test]
    fn op_put_with_negative_index_operand_does_not_panic() {
        // Push a value (0), push `-1` (byte 138) as the index, then
        // `put` (escape `12 20` = OP_PUT). Same bug/fix as `get` above.
        let data: Vec<u8> = vec![139, 138, 12, 20];
        let mut stack = fresh_stack();
        run(&data, &mut stack);
        assert_eq!(stack.index, 0);
        assert!(matches!(stack.transient[31], CffValue::Double(v) if v == 0.0));
    }

    #[test]
    fn op_roll_with_zero_count_operand_does_not_panic() {
        // Push J=0, push N=0, then `roll` (escape `12 30` = OP_ROLL).
        // `wrapping_rem(n)` once panicked on this zero divisor. Rolling 0
        // operands rotates nothing, and N and J are popped as usual.
        let data: Vec<u8> = vec![139, 139, 12, 30];
        let mut stack = fresh_stack();
        run(&data, &mut stack);
        assert_eq!(stack.index, 0);
    }

    fn numbers(stack: &CffStack) -> Vec<f64> {
        (0..stack.index).map(|i| stack.num(i)).collect()
    }

    #[test]
    fn op_roll_rotates_toward_the_top_and_pops_n_and_j() {
        // `1 2 3 3 1 roll` -> `3 1 2`; `1 2 3 3 -1 roll` -> `2 3 1`.
        let mut stack = fresh_stack();
        run(&[140, 141, 142, 142, 140, 12, 30], &mut stack);
        assert_eq!(numbers(&stack), [3.0, 1.0, 2.0]);
        let mut stack = fresh_stack();
        run(&[140, 141, 142, 142, 138, 12, 30], &mut stack);
        assert_eq!(numbers(&stack), [2.0, 3.0, 1.0]);
        // A shift of 0 (or a multiple of N) still pops N and J.
        let mut stack = fresh_stack();
        run(&[140, 141, 142, 142, 145, 12, 30], &mut stack);
        assert_eq!(numbers(&stack), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn op_roll_past_256_operands_rolls_the_top_ones() {
        // 300 zeros, then `1 2 3 3 1 roll`: positions are no longer
        // truncated to a byte, so the three operands at the top roll.
        let mut data: Vec<u8> = vec![139u8; 300];
        data.extend_from_slice(&[140, 141, 142, 142, 140, 12, 30]);
        let mut stack = fresh_stack();
        run(&data, &mut stack);
        assert_eq!(stack.index, 303);
        assert_eq!(numbers(&stack)[300..], [3.0, 1.0, 2.0]);
    }

    #[test]
    fn op_roll_of_more_operands_than_the_stack_holds_is_ignored() {
        // `1 2 5 1 roll`: only two operands below N and J.
        let mut stack = fresh_stack();
        run(&[140, 141, 144, 140, 12, 30], &mut stack);
        assert_eq!(numbers(&stack), [1.0, 2.0]);
    }

    #[test]
    fn op_index_copies_the_operand_i_places_below() {
        // `1 2 3 1 index` -> `1 2 3 2`; a negative `i` copies the top one.
        let mut stack = fresh_stack();
        run(&[140, 141, 142, 140, 12, 29], &mut stack);
        assert_eq!(numbers(&stack), [1.0, 2.0, 3.0, 2.0]);
        let mut stack = fresh_stack();
        run(&[140, 141, 142, 138, 12, 29], &mut stack);
        assert_eq!(numbers(&stack), [1.0, 2.0, 3.0, 3.0]);
        // `i` past the bottom of the stack is ignored.
        let mut stack = fresh_stack();
        run(&[140, 141, 142, 143, 12, 29], &mut stack);
        assert_eq!(numbers(&stack), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn op_index_with_operand_count_multiple_of_256_does_not_panic() {
        // Push 256 ones (byte 140) and `i` = 0 (byte 139), then `index`
        // (escape `12 29` = OP_INDEX). The position of `i` was once
        // truncated to a byte, 256 to 0, and then used as a modulus,
        // panicking on the divide (and later skipping the operation).
        // `0 index` copies the operand just below.
        let mut data: Vec<u8> = vec![140u8; 256];
        data.extend_from_slice(&[139, 12, 29]);
        let mut stack = fresh_stack();
        run(&data, &mut stack);
        assert_eq!(stack.index, 257);
        assert_eq!(stack.num(256), 1.0);
    }
}

#[cfg(test)]
mod cff_parse_outline_subr_number_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;

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
        cff_parse_outline(&data, gsubr, lsubr, &mut stack, &mut ctx, 0, &mut total_calls);
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
}

#[cfg(test)]
mod cff_parse_outline_operand_group_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;

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
        cff_parse_outline(&data, &gsubr, &lsubr, &mut stack, &mut ctx, 0, &mut total_calls);
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
}
