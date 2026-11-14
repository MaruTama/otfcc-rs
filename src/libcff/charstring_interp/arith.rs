use super::*;
use crate::logger::ByteStr;
use crate::libcff::value::{CffValue, cffnum};
use crate::support::primitives::Arity;
use crate::table::cff::OutlineBuilderContext;
use crate::table::cff::read::callback_draw_getrand;
use crate::libcff::{CffCharstringOperator, CffStack, TYPE2_TRANSIENT_ARRAY, OP_ABS, OP_ADD, OP_AND, OP_DIV, OP_DROP, OP_DUP, OP_EQ, OP_EXCH, OP_GET, OP_IFELSE, OP_INDEX, OP_MUL, OP_NEG, OP_NOT, OP_OR, OP_PUT, OP_ROLL, OP_SQRT, OP_SUB};

pub(super) fn too_few_operands(name: &str, op: CffCharstringOperator) {
    tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr(name), op.0 as u32);
}

pub(super) fn stack_overflow() {
    tracing::warn!("[libcff] Operand stack overflow in Type 2 CharString; the rest of this outline is ignored.\n");
}

pub(super) fn truth(condition: bool) -> f64 {
    if condition { 1.0 } else { 0.0 }
}

/// Replaces the top operand `a` with `f(a)`.
pub(super) fn unary(stack: &mut CffStack, name: &str, op: CffCharstringOperator, f: impl FnOnce(f64) -> f64) -> Flow {
    if stack.index < 1 {
        too_few_operands(name, op);
    } else {
        let a = stack.top(1);
        stack.set_top(1, f(a));
    }
    Flow::Continue
}

/// Replaces the top two operands `a b` (`b` pushed last) with `f(a, b)`.
pub(super) fn binary(stack: &mut CffStack, name: &str, op: CffCharstringOperator, f: impl FnOnce(f64, f64) -> f64) -> Flow {
    if stack.index < 2 {
        too_few_operands(name, op);
    } else {
        let (a, b) = (stack.top(2), stack.top(1));
        stack.set_top(2, f(a, b));
        stack.index -= 1;
    }
    Flow::Continue
}

pub(super) fn op_and(stack: &mut CffStack) -> Flow {
    binary(stack, "op_and", OP_AND, |a, b| truth(b != 0.0 && a != 0.0))
}

pub(super) fn op_or(stack: &mut CffStack) -> Flow {
    binary(stack, "op_or", OP_OR, |a, b| truth(b != 0.0 || a != 0.0))
}

pub(super) fn op_not(stack: &mut CffStack) -> Flow {
    unary(stack, "op_not", OP_NOT, |a| truth(a == 0.0))
}

pub(super) fn op_abs(stack: &mut CffStack) -> Flow {
    // Not `f64::abs`: that would also clear the sign of `-0.0` and NaN.
    unary(stack, "op_abs", OP_ABS, |a| if a < 0.0 { -a } else { a })
}

pub(super) fn op_add(stack: &mut CffStack) -> Flow {
    binary(stack, "op_add", OP_ADD, |a, b| b + a)
}

pub(super) fn op_sub(stack: &mut CffStack) -> Flow {
    binary(stack, "op_sub", OP_SUB, |a, b| a - b)
}

pub(super) fn op_div(stack: &mut CffStack) -> Flow {
    binary(stack, "op_div", OP_DIV, |a, b| a / b)
}

pub(super) fn op_neg(stack: &mut CffStack) -> Flow {
    unary(stack, "op_neg", OP_NEG, |a| -a)
}

pub(super) fn op_eq(stack: &mut CffStack) -> Flow {
    binary(stack, "op_eq", OP_EQ, |a, b| truth(b == a))
}

pub(super) fn op_mul(stack: &mut CffStack) -> Flow {
    binary(stack, "op_mul", OP_MUL, |a, b| b * a)
}

pub(super) fn op_sqrt(stack: &mut CffStack) -> Flow {
    unary(stack, "op_sqrt", OP_SQRT, |a| a.sqrt())
}

pub(super) fn op_drop(stack: &mut CffStack) -> Flow {
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
pub(super) fn transient_slot(i: f64) -> usize {
    (i as i32).rem_euclid(TYPE2_TRANSIENT_ARRAY as i32) as usize
}

/// `val i put`
pub(super) fn op_put(stack: &mut CffStack) -> Flow {
    if stack.index < 2 {
        too_few_operands("op_put", OP_PUT);
    } else {
        stack.transient[transient_slot(stack.top(1))] = CffValue::Double(stack.top(2));
        stack.index -= 2;
    }
    Flow::Continue
}

/// `i get`
pub(super) fn op_get(stack: &mut CffStack) -> Flow {
    if stack.index < 1 {
        too_few_operands("op_get", OP_GET);
    } else {
        let value = cffnum(stack.transient[transient_slot(stack.top(1))]);
        stack.set_top(1, value);
    }
    Flow::Continue
}

/// `s1 s2 v1 v2 ifelse`: `s1` if `v1 <= v2`, else `s2`.
pub(super) fn op_ifelse(stack: &mut CffStack) -> Flow {
    if stack.index < 4 {
        too_few_operands("op_ifelse", OP_IFELSE);
    } else {
        let (s1, s2, v1, v2) = (stack.top(4), stack.top(3), stack.top(2), stack.top(1));
        stack.set_top(4, if v1 <= v2 { s1 } else { s2 });
        stack.index -= 3;
    }
    Flow::Continue
}

pub(super) fn op_random(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    // Check for room before drawing the number, so a full stack leaves the
    // generator's state untouched.
    if !stack.has_room() {
        stack_overflow();
        return Flow::Stop;
    }
    stack.push(CffValue::Double(callback_draw_getrand(outline)));
    Flow::Continue
}

pub(super) fn op_dup(stack: &mut CffStack) -> Flow {
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

pub(super) fn op_exch(stack: &mut CffStack) -> Flow {
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
pub(super) fn op_index(stack: &mut CffStack) -> Flow {
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
pub(super) fn op_roll(stack: &mut CffStack) -> Flow {
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

