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
use crate::table::cff::OutlineBuilderContext;
use crate::libcff::parser::{
    MAX_SUBR_CALL_DEPTH, MAX_TOTAL_SUBR_CALLS, compute_subr_bias, locate_subr,
};
use crate::libcff::{CffCharstringOperator, CffStack, OP_ABS, OP_ADD, OP_AND, OP_CALLGSUBR, OP_CALLSUBR, OP_CNTRMASK, OP_DIV, OP_DROP, OP_DUP, OP_ENDCHAR, OP_EQ, OP_EXCH, OP_FLEX, OP_FLEX1, OP_GET, OP_HFLEX, OP_HFLEX1, OP_HHCURVETO, OP_HINTMASK, OP_HLINETO, OP_HMOVETO, OP_HSTEM, OP_HSTEMHM, OP_HVCURVETO, OP_IFELSE, OP_INDEX, OP_MUL, OP_NEG, OP_NOT, OP_OR, OP_PUT, OP_RANDOM, OP_RCURVELINE, OP_RETURN, OP_RLINECURVE, OP_RLINETO, OP_RMOVETO, OP_ROLL, OP_RRCURVETO, OP_SQRT, OP_SUB, OP_VHCURVETO, OP_VLINETO, OP_VMOVETO, OP_VSTEM, OP_VSTEMHM, OP_VVCURVETO};
mod arith;
mod path;

use arith::*;
use path::*;

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

