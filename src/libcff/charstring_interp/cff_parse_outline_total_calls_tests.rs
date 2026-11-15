use super::*;
use crate::libcff::TYPE2_TRANSIENT_ARRAY;
use crate::libcff::index::CffIndexCountType;

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
// 0` = index 107, this index's own last entry.
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
// *recursive `parse_outline` calls*, not the array size. That's
// inherent to what this test proves, the same way
// `total_language_count_across_the_whole_table_is_capped`
// (`table/otl/read.rs`) can't shrink its own N below the real cap
// either. `cargo test` (native) stays the real regression guard.
#[cfg_attr(miri, ignore = "far too slow to run meaningfully under Miri's interpreter; needs ~10,000 recursive parse_outline calls to exercise the real MAX_TOTAL_SUBR_CALLS budget")]
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
    // operator, so the outline context is never actually touched.
    let mut g = new_glyf_glyph();
    let mut ctx = dummy_outline_context(&mut g);
    parse_outline(
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
// `total_calls` (threaded through every recursive `parse_outline`
// call, shared across the whole glyph) is what actually bounds it.
#[test]
// Same Miri cost shape as the sibling test above: recursing up to
// `MAX_TOTAL_SUBR_CALLS` (10,000) before stopping is the point of
// this test, not something a smaller N could substitute for.
#[cfg_attr(miri, ignore = "far too slow to run meaningfully under Miri's interpreter; needs ~10,000 recursive parse_outline calls before the real MAX_TOTAL_SUBR_CALLS budget stops it")]
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
    parse_outline(
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
