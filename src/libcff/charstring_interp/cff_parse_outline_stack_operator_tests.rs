use super::*;
use crate::libcff::index::CffIndexCountType;

use crate::table::glyf::new_glyf_glyph;

// A charstring's `put`/`get`/`index`/`roll` operators each take a
// charstring-supplied stack *value* and use it as an array index or
// modulus divisor into a small fixed-size structure (`transient[32]`,
// or the operand stack itself). Each must be range-checked rather than
// panic (array-index or divide-by-zero).

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
    // context now that `parse_outline` takes one unconditionally.
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
    parse_outline(
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
