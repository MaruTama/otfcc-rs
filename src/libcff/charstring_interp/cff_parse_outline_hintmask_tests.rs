use super::*;
use crate::libcff::TYPE2_TRANSIENT_ARRAY;
use crate::libcff::index::CffIndexCountType;

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
// `decode_cs2_token`'s own bounds checking, so nothing stopped
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
    parse_outline(
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

// Fuzz-found: a charstring chaining enough `hstem` operators to push
// the real cumulative hint count (`context.g.stem_h`) past 255. The
// hint count used to be a `u8` that wrapped back down to a small value,
// and `callback_draw_setmask` then indexed an undersized mask with the
// real (large) `stem_h.len()`.
//
// 256 single-hint `hstem` calls (push 0, push 0, `hstem`) push exactly
// 256 real entries into `stem_h`; `stem` is `u32` and must read back
// the true 256.
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
    parse_outline(
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
