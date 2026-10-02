//! The Type 2 CharString interpreter: runs one glyph's CharString (and the
//! subroutines it calls) and draws the result through the
//! `OutlineBuilderContext` callbacks in `table/cff.rs`.
//!
//! `cff_parse_outline` reads tokens and pushes operands; each operator is
//! one `op_*` function below, named after its `OP_*` constant. An operator
//! returns [`Flow::Stop`] when the rest of the outline must be ignored
//! (operand stack overflow, malformed hint mask data).

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
    MAX_SUBR_CALL_DEPTH, MAX_TOTAL_SUBR_CALLS, compute_subr_bias, locate_subr, reverse_stack,
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

// `methods: CffIOutlineBuilder` parameter dropped: this was called from
// exactly one call site (`table/cff.rs`), always passing the single static
// `DRAW_PASS` -- degenerate polymorphism like every other collapsed
// vtable, just structured as a by-value struct argument instead of a
// global static. Every field of `DRAW_PASS` is always `Some`, so the old
// per-field `.is_none()` fallback-to-`callback_nop_*` branches below were
// already unreachable dead code; deleted along with the extraction, not
// just the vtable shell.
//
// `outline` itself used to be a `*mut c_void`, cast back to `*mut
// OutlineBuilderContext` at each `callback_draw_*` call site -- more type
// erasure that was never actually needed, the same pattern as the vtable
// above: every one of the ~40 call sites below (including the two
// recursive `cff_parse_outline` calls for `callsubr`/`callgsubr`) already
// knows the concrete type at compile time. A real `&mut
// OutlineBuilderContext` carries the same information with no cast, and
// Rust's implicit-reborrow rule for `&mut` places (the same mechanism
// that lets a loop call `f(r)` on a `&mut` binding `r` repeatedly without
// "value moved" errors) means every one of those call sites, recursive
// calls included, keeps working unchanged.
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
        tracing::warn!("[libcff] Subroutine call nesting exceeded {}; the rest of this outline is ignored.\n", MAX_SUBR_CALL_DEPTH as i32);
        return;
    }
    let subrs = Subroutines {
        gsubr,
        lsubr,
        gsubr_bias: compute_subr_bias(gsubr.count as u16),
        lsubr_bias: compute_subr_bias(lsubr.count as u16),
    };
    // `pos` (into `data`) replaces `start` (a `*mut u8` cursor), the same
    // "cursor into a safe slice instead of raw pointer arithmetic" shape
    // the rest of this crate's parse-boundary work already uses.
    // `cff_decode_cs2_token` takes `&data[pos..]` directly.
    let data_slice: &[u8] = data;
    let mut pos: usize = 0;
    let mut advance: u32;
    let mut val: CffValue = CffValue::Unset;
    while pos < data_slice.len() {
        // The outer loop already bounds where a token can *start*, but
        // not that the token itself stays within `len` -- a token
        // starting near the end of a truncated CharString used to read
        // past it (see `cff_codecs.rs`'s own conversion). Stop cleanly
        // instead of reading on. (`op_hint_mask` bounds its mask bytes,
        // which follow the operator outside any token, the same way.)
        let Some(adv) = cff_decode_cs2_token(&data_slice[pos..], &mut val) else {
            break;
        };
        advance = adv;
        match val {
            CffValue::Operator(op) => {
                let flow = match CffCharstringOperator(op) {
                    OP_HSTEM | OP_VSTEM | OP_HSTEMHM | OP_VSTEMHM => op_stem_hints(stack, outline, op),
                    OP_HINTMASK | OP_CNTRMASK => {
                        // The mask bytes follow the operator directly.
                        let mask_bytes = data_slice.get(pos + advance as usize..).unwrap_or(&[]);
                        match op_hint_mask(stack, outline, op, mask_bytes) {
                            Some(mask_length) => {
                                advance = advance.wrapping_add(mask_length);
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
                if (stack.index as usize) < stack.stack.len() {
                    (&mut stack.stack)[(stack.index as isize) as usize] = val;
                    stack.index = stack.index.wrapping_add(1);
                } else {
                    tracing::warn!("[libcff] Operand stack overflow in Type 2 CharString; the rest of this outline is ignored.\n");
                    return;
                }
            }
            CffValue::Unset => {}
        }
        pos += advance as usize;
    }
}

fn op_stem_hints(stack: &mut CffStack, outline: &mut OutlineBuilderContext, op: i32) -> Flow {
    let mut hint_base: f64;
    if stack.index.wrapping_rem(2 as Arity) != 0 {
        callback_draw_setwidth(
            outline,
            cffnum(
                (&mut stack.stack)[(0_i32 as isize) as usize],
            ),
        );
    }
    // `saturating_add`, not `wrapping_add`: this counter
    // sizes the `hintmask`/`cntrmask` bit array below and
    // must never wrap back down to a small value while
    // `stem_h`/`stem_v` (unbounded, real counts) keep
    // growing -- see the `stem` field's doc comment.
    stack.stem = stack.stem.saturating_add(stack.index >> 1_i32);
    hint_base = 0_i32 as ::core::ffi::c_double;
    let j_start: Arity = stack.index.wrapping_rem(2 as Arity);
    for j in (j_start..stack.index).step_by(2) {
        let pos: ::core::ffi::c_double =
            cffnum((&mut stack.stack)[(j as isize) as usize]);
        let width: ::core::ffi::c_double =
            cffnum((&mut stack.stack)[(
                (j as i32 + 1_i32) as isize) as usize]);
        callback_draw_sethint(
            outline,
            op == OP_VSTEM.0 || op == OP_VSTEMHM.0,
            pos + hint_base,
            width,
        );
        hint_base += pos + width;
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_hint_mask(stack: &mut CffStack, outline: &mut OutlineBuilderContext, op: i32, mask_bytes: &[u8]) -> Option<u32> {
    if stack.index.wrapping_rem(2 as Arity) != 0 {
        callback_draw_setwidth(
            outline,
            cffnum(
                (&mut stack.stack)[(0_i32 as isize) as usize],
            ),
        );
    }
    let is_vertical: bool =
        stack.stem as i32 > 0_i32;
    // `saturating_add`, not `wrapping_add`: this counter
    // sizes the `hintmask`/`cntrmask` bit array below and
    // must never wrap back down to a small value while
    // `stem_h`/`stem_v` (unbounded, real counts) keep
    // growing -- see the `stem` field's doc comment.
    stack.stem = stack.stem.saturating_add(stack.index >> 1_i32);
    let mut hint_base_0: ::core::ffi::c_double =
        0_i32 as ::core::ffi::c_double;
    let j_0_start: Arity = stack.index.wrapping_rem(2 as Arity);
    for j_0 in (j_0_start..stack.index).step_by(2) {
        let pos_0: ::core::ffi::c_double =
            cffnum((&mut stack.stack)[(j_0 as isize) as usize]);
        let width_0: ::core::ffi::c_double =
            cffnum((&mut stack.stack)[(
                (j_0 as i32 + 1_i32) as isize) as usize]);
        callback_draw_sethint(
            outline,
            is_vertical,
            pos_0 + hint_base_0,
            width_0,
        );
        hint_base_0 += pos_0 + width_0;
    }
    let mask_length: u32 =
        ((stack.stem as i32 + 7_i32)
            >> 3_i32) as u32;
    // `hintmask`/`cntrmask`'s mask bytes are raw payload
    // embedded directly in the charstring right after
    // the opcode -- unlike every other operand, they
    // never go through `cff_decode_cs2_token`'s own
    // bounds checking, so nothing here previously
    // stopped `mask_length` (driven by `(*stack).stem`,
    // the accumulated hint count from every `hstem`/
    // `vstem` operator already seen) from reading past
    // the actual CharString buffer. A fuzz-found input
    // pushed enough stem hints to make `mask_length`
    // exceed what was left of the charstring by a
    // single byte -- an ASan-confirmed heap-buffer-
    // overflow. `mask_bytes` is exactly what is left of
    // the charstring after the operator; stop cleanly
    // instead of reading past it.
    if mask_length as usize > mask_bytes.len() {
        return None;
    }
    // Sized to exactly `(*stack).stem + 7` bools, same
    // as the original's `__caryll_allocate_clean` call
    // -- the largest index any byte in `0..mask_length`
    // writes is `((mask_length - 1) << 3) + 7`, and
    // `mask_length == ((*stack).stem + 7) >> 3` keeps
    // that within `(*stack).stem + 6` (one shy of this
    // Vec's length) regardless of whether `stem + 7` is
    // itself a multiple of 8.
    let mut mask: Vec<bool> =
        vec![false; (stack.stem as i32 + 7_i32) as usize];
    for byte in 0..mask_length {
        let mask_byte: u8 =
            mask_bytes[byte as usize];
        mask[(byte << 3_i32).wrapping_add(0_u32) as usize] =
            mask_byte as i32 >> 7_i32 & 1_i32 != 0;
        mask[(byte << 3_i32).wrapping_add(1_u32) as usize] =
            mask_byte as i32 >> 6_i32 & 1_i32 != 0;
        mask[(byte << 3_i32).wrapping_add(2_u32) as usize] =
            mask_byte as i32 >> 5_i32 & 1_i32 != 0;
        mask[(byte << 3_i32).wrapping_add(3_u32) as usize] =
            mask_byte as i32 >> 4_i32 & 1_i32 != 0;
        mask[(byte << 3_i32).wrapping_add(4_u32) as usize] =
            mask_byte as i32 >> 3_i32 & 1_i32 != 0;
        mask[(byte << 3_i32).wrapping_add(5_u32) as usize] =
            mask_byte as i32 >> 2_i32 & 1_i32 != 0;
        mask[(byte << 3_i32).wrapping_add(6_u32) as usize] =
            mask_byte as i32 >> 1_i32 & 1_i32 != 0;
        mask[(byte << 3_i32).wrapping_add(7_u32) as usize] =
            (mask_byte as i32) & 1_i32 != 0;
    }
    callback_draw_setmask(outline, op == OP_CNTRMASK.0, &mask);
    stack.index = 0 as Arity;
    Some(mask_length)
}

fn op_vmoveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_vmoveto"), OP_VMOVETO.0 as u32);
    } else {
        if stack.index > 1 as Arity {
            callback_draw_setwidth(
                outline,
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(2 as Arity) as isize) as usize],
                ),
            );
        }
        callback_draw_next_contour(outline);
        callback_draw_lineto(
            outline,
            0.0f64,
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
            ),
        );
        stack.index = 0 as Arity;
    }
    Flow::Continue
}

fn op_rmoveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_rmoveto"), OP_RMOVETO.0 as u32);
    } else {
        if stack.index > 2 as Arity {
            callback_draw_setwidth(
                outline,
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(3 as Arity) as isize) as usize],
                ),
            );
        }
        callback_draw_next_contour(outline);
        callback_draw_lineto(
            outline,
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
            ),
        );
        stack.index = 0 as Arity;
    }
    Flow::Continue
}

fn op_hmoveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_hmoveto"), OP_HMOVETO.0 as u32);
    } else {
        if stack.index > 1 as Arity {
            callback_draw_setwidth(
                outline,
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(2 as Arity) as isize) as usize],
                ),
            );
        }
        callback_draw_next_contour(outline);
        callback_draw_lineto(
            outline,
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
            ),
            0.0f64,
        );
        stack.index = 0 as Arity;
    }
    Flow::Continue
}

fn op_endchar(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index > 0 as Arity {
        callback_draw_setwidth(
            outline,
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
            ),
        );
    }
    Flow::Continue
}

fn op_rlineto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    for i in (0..stack.index).step_by(2) {
        callback_draw_lineto(
            outline,
            cffnum((&mut stack.stack)[(i as isize) as usize]),
            cffnum(
                (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
            ),
        );
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_vlineto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index.wrapping_rem(2 as Arity) == 1 as Arity {
        callback_draw_lineto(
            outline,
            0.0f64,
            cffnum(
                (&mut stack.stack)[(0_i32 as isize) as usize],
            ),
        );
        for i in (1..stack.index).step_by(2) {
            callback_draw_lineto(
                outline,
                cffnum((&mut stack.stack)[(i as isize) as usize]),
                0.0f64,
            );
            callback_draw_lineto(
                outline,
                0.0f64,
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                ),
            );
        }
    } else {
        for i in (0..stack.index).step_by(2) {
            callback_draw_lineto(
                outline,
                0.0f64,
                cffnum((&mut stack.stack)[(i as isize) as usize]),
            );
            callback_draw_lineto(
                outline,
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                ),
                0.0f64,
            );
        }
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_hlineto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index.wrapping_rem(2 as Arity) == 1 as Arity {
        callback_draw_lineto(
            outline,
            cffnum(
                (&mut stack.stack)[(0_i32 as isize) as usize],
            ),
            0.0f64,
        );
        for i in (1..stack.index).step_by(2) {
            callback_draw_lineto(
                outline,
                0.0f64,
                cffnum((&mut stack.stack)[(i as isize) as usize]),
            );
            callback_draw_lineto(
                outline,
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                ),
                0.0f64,
            );
        }
    } else {
        for i in (0..stack.index).step_by(2) {
            callback_draw_lineto(
                outline,
                cffnum((&mut stack.stack)[(i as isize) as usize]),
                0.0f64,
            );
            callback_draw_lineto(
                outline,
                0.0f64,
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                ),
            );
        }
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_rrcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    for i in (0..stack.index).step_by(6) {
        callback_draw_curveto(
            outline,
            cffnum((&mut stack.stack)[(i as isize) as usize]),
            cffnum(
                (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(i.wrapping_add(2_u32) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(i.wrapping_add(3_u32) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(i.wrapping_add(4_u32) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(i.wrapping_add(5_u32) as isize) as usize],
            ),
        );
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_rcurveline(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for op_rcurveline (24). This operation is ignored.\n");
    } else {
        for i in (0..stack.index.wrapping_sub(2 as Arity)).step_by(6) {
            callback_draw_curveto(
                outline,
                cffnum((&mut stack.stack)[(i as isize) as usize]),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(2_u32) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(3_u32) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(4_u32) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(5_u32) as isize) as usize],
                ),
            );
        }
        callback_draw_lineto(
            outline,
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
            ),
        );
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_rlinecurve(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 6 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for op_rlinecurve (25). This operation is ignored.\n");
    } else {
        for i in (0..stack.index.wrapping_sub(6 as Arity)).step_by(2) {
            callback_draw_lineto(
                outline,
                cffnum((&mut stack.stack)[(i as isize) as usize]),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                ),
            );
        }
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(6 as Arity) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(5 as Arity) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(4 as Arity) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(3 as Arity) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
            ),
        );
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_vvcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index.wrapping_rem(4 as Arity) == 1 as Arity {
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(0_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(1_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(2_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(3_i32 as isize) as usize],
            ),
            0.0f64,
            cffnum(
                (&mut stack.stack)[(4_i32 as isize) as usize],
            ),
        );
        for i in (5..stack.index).step_by(4) {
            callback_draw_curveto(
                outline,
                0.0f64,
                cffnum((&mut stack.stack)[(i as isize) as usize]),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(2_u32) as isize) as usize],
                ),
                0.0f64,
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(3_u32) as isize) as usize],
                ),
            );
        }
    } else {
        for i in (0..stack.index).step_by(4) {
            callback_draw_curveto(
                outline,
                0.0f64,
                cffnum((&mut stack.stack)[(i as isize) as usize]),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(2_u32) as isize) as usize],
                ),
                0.0f64,
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(3_u32) as isize) as usize],
                ),
            );
        }
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_hhcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index.wrapping_rem(4 as Arity) == 1 as Arity {
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(1_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(0_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(2_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(3_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(4_i32 as isize) as usize],
            ),
            0.0f64,
        );
        for i in (5..stack.index).step_by(4) {
            callback_draw_curveto(
                outline,
                cffnum((&mut stack.stack)[(i as isize) as usize]),
                0.0f64,
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(2_u32) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(3_u32) as isize) as usize],
                ),
                0.0f64,
            );
        }
    } else {
        for i in (0..stack.index).step_by(4) {
            callback_draw_curveto(
                outline,
                cffnum((&mut stack.stack)[(i as isize) as usize]),
                0.0f64,
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(2_u32) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(i.wrapping_add(3_u32) as isize) as usize],
                ),
                0.0f64,
            );
        }
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_vhcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    let cnt_bezier: u32;
    // `index % 4 == 1` alone doesn't guarantee enough
    // operands: the only value satisfying it below 5 is 1
    // itself, a single lone coordinate with no complete
    // curve to pair it with. Every read below (the
    // `index - 5` here and the `% 8 == 1` block's own
    // `index - 5`/`- 4`/`- 3`) assumes a full curve (4)
    // plus that odd trailing coordinate (1) are both
    // actually present, i.e. `index >= 5`.
    if stack.index.wrapping_rem(4 as Arity) == 1 as Arity
        && stack.index < 5 as Arity
    {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for op_vhcurveto (30). This operation is ignored.\n");
    } else {
        if stack.index.wrapping_rem(4 as Arity) == 1 as Arity {
            cnt_bezier = stack
                .index
                .wrapping_sub(5 as Arity)
                .wrapping_div(4 as Arity);
        } else {
            cnt_bezier = stack.index.wrapping_div(4 as Arity);
        }
        for i in (0..4_u32.wrapping_mul(cnt_bezier)).step_by(4) {
            if i.wrapping_div(4_u32).wrapping_rem(2_u32) == 0_u32 {
                callback_draw_curveto(
                    outline,
                    0.0f64,
                    cffnum((&mut stack.stack)[(i as isize) as usize]),
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                    ),
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(2_u32) as isize) as usize],
                    ),
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(3_u32) as isize) as usize],
                    ),
                    0.0f64,
                );
            } else {
                callback_draw_curveto(
                    outline,
                    cffnum((&mut stack.stack)[(i as isize) as usize]),
                    0.0f64,
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                    ),
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(2_u32) as isize) as usize],
                    ),
                    0.0f64,
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(3_u32) as isize) as usize],
                    ),
                );
            }
        }
        if stack.index.wrapping_rem(8 as Arity) == 5 as Arity {
            callback_draw_curveto(
                outline,
                0.0f64,
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(5 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(4 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(3 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(2 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(1 as Arity) as isize) as usize],
                ),
            );
        }
        if stack.index.wrapping_rem(8 as Arity) == 1 as Arity {
            callback_draw_curveto(
                outline,
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(5 as Arity) as isize) as usize],
                ),
                0.0f64,
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(4 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(3 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(1 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(2 as Arity) as isize) as usize],
                ),
            );
        }
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_hvcurveto(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    let cnt_bezier: u32;
    // Same reasoning as op 30 above: `index % 4 == 1`
    // with `index < 5` means exactly `index == 1`, a
    // lone coordinate with no complete curve behind it.
    if stack.index.wrapping_rem(4 as Arity) == 1 as Arity
        && stack.index < 5 as Arity
    {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for op_hvcurveto (31). This operation is ignored.\n");
    } else {
        if stack.index.wrapping_rem(4 as Arity) == 1 as Arity {
            cnt_bezier = stack
                .index
                .wrapping_sub(5 as Arity)
                .wrapping_div(4 as Arity);
        } else {
            cnt_bezier = stack.index.wrapping_div(4 as Arity);
        }
        for i in (0..4_u32.wrapping_mul(cnt_bezier)).step_by(4) {
            if i.wrapping_div(4_u32).wrapping_rem(2_u32) == 0_u32 {
                callback_draw_curveto(
                    outline,
                    cffnum((&mut stack.stack)[(i as isize) as usize]),
                    0.0f64,
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                    ),
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(2_u32) as isize) as usize],
                    ),
                    0.0f64,
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(3_u32) as isize) as usize],
                    ),
                );
            } else {
                callback_draw_curveto(
                    outline,
                    0.0f64,
                    cffnum((&mut stack.stack)[(i as isize) as usize]),
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(1_u32) as isize) as usize],
                    ),
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(2_u32) as isize) as usize],
                    ),
                    cffnum(
                        (&mut stack.stack)[(i.wrapping_add(3_u32) as isize) as usize],
                    ),
                    0.0f64,
                );
            }
        }
        if stack.index.wrapping_rem(8 as Arity) == 5 as Arity {
            callback_draw_curveto(
                outline,
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(5 as Arity) as isize) as usize],
                ),
                0.0f64,
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(4 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(3 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(1 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(2 as Arity) as isize) as usize],
                ),
            );
        }
        if stack.index.wrapping_rem(8 as Arity) == 1 as Arity {
            callback_draw_curveto(
                outline,
                0.0f64,
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(5 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(4 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(3 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(2 as Arity) as isize) as usize],
                ),
                cffnum(
                    (&mut stack.stack)[(
                        stack.index.wrapping_sub(1 as Arity) as isize) as usize],
                ),
            );
        }
    }
    stack.index = 0 as Arity;
    Flow::Continue
}

fn op_hflex(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 7 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_hflex"), OP_HFLEX.0 as u32);
    } else {
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(0_i32 as isize) as usize],
            ),
            0.0f64,
            cffnum(
                (&mut stack.stack)[(1_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(2_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(3_i32 as isize) as usize],
            ),
            0.0f64,
        );
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(4_i32 as isize) as usize],
            ),
            0.0f64,
            cffnum(
                (&mut stack.stack)[(5_i32 as isize) as usize],
            ),
            -cffnum(
                (&mut stack.stack)[(2_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(6_i32 as isize) as usize],
            ),
            0.0f64,
        );
        stack.index = 0 as Arity;
    }
    Flow::Continue
}

fn op_flex(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 12 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_flex"), OP_FLEX.0 as u32);
    } else {
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(0_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(1_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(2_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(3_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(4_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(5_i32 as isize) as usize],
            ),
        );
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(6_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(7_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(8_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(9_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(10_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(11_i32 as isize) as usize],
            ),
        );
        stack.index = 0 as Arity;
    }
    Flow::Continue
}

fn op_hflex1(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 9 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_hflex1"), OP_HFLEX1.0 as u32);
    } else {
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(0_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(1_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(2_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(3_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(4_i32 as isize) as usize],
            ),
            0.0f64,
        );
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(5_i32 as isize) as usize],
            ),
            0.0f64,
            cffnum(
                (&mut stack.stack)[(6_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(7_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(8_i32 as isize) as usize],
            ),
            -(cffnum(
                (&mut stack.stack)[(1_i32 as isize) as usize],
            ) + cffnum(
                (&mut stack.stack)[(3_i32 as isize) as usize],
            ) + cffnum(
                (&mut stack.stack)[(7_i32 as isize) as usize],
            )),
        );
        stack.index = 0 as Arity;
    }
    Flow::Continue
}

fn op_flex1(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if stack.index < 11 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_flex1"), OP_FLEX1.0 as u32);
    } else {
        let mut dx: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(0_i32 as isize) as usize],
        ) + cffnum(
            (&mut stack.stack)[(2_i32 as isize) as usize],
        ) + cffnum(
            (&mut stack.stack)[(4_i32 as isize) as usize],
        ) + cffnum(
            (&mut stack.stack)[(6_i32 as isize) as usize],
        ) + cffnum(
            (&mut stack.stack)[(8_i32 as isize) as usize],
        );
        let mut dy: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(1_i32 as isize) as usize],
        ) + cffnum(
            (&mut stack.stack)[(3_i32 as isize) as usize],
        ) + cffnum(
            (&mut stack.stack)[(5_i32 as isize) as usize],
        ) + cffnum(
            (&mut stack.stack)[(7_i32 as isize) as usize],
        ) + cffnum(
            (&mut stack.stack)[(9_i32 as isize) as usize],
        );
        if dx.abs() > dy.abs() {
            dx = cffnum(
                (&mut stack.stack)[(10_i32 as isize) as usize],
            );
            dy = -dy;
        } else {
            dx = -dx;
            dy = cffnum(
                (&mut stack.stack)[(10_i32 as isize) as usize],
            );
        }
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(0_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(1_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(2_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(3_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(4_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(5_i32 as isize) as usize],
            ),
        );
        callback_draw_curveto(
            outline,
            cffnum(
                (&mut stack.stack)[(6_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(7_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(8_i32 as isize) as usize],
            ),
            cffnum(
                (&mut stack.stack)[(9_i32 as isize) as usize],
            ),
            dx,
            dy,
        );
        stack.index = 0 as Arity;
    }
    Flow::Continue
}

fn op_and(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_and"), OP_AND.0 as u32);
    } else {
        let num1: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        let num2: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize]) =
            CffValue::Double(if num1 != 0. && num2 != 0. {
                1.0f64
            } else {
                0.0f64
            });
        stack.index = stack.index.wrapping_sub(1 as Arity);
    }
    Flow::Continue
}

fn op_or(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_or"), OP_OR.0 as u32);
    } else {
        let num1_0: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        let num2_0: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize]) =
            CffValue::Double(if num1_0 != 0. || num2_0 != 0. {
                1.0f64
            } else {
                0.0f64
            });
        stack.index = stack.index.wrapping_sub(1 as Arity);
    }
    Flow::Continue
}

fn op_not(stack: &mut CffStack) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_not"), OP_NOT.0 as u32);
    } else {
        let num: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize]) =
            CffValue::Double(if num != 0. { 0.0f64 } else { 1.0f64 });
    }
    Flow::Continue
}

fn op_abs(stack: &mut CffStack) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_abs"), OP_ABS.0 as u32);
    } else {
        let num_0: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize]) =
            CffValue::Double(if num_0 < 0.0f64 { -num_0 } else { num_0 });
    }
    Flow::Continue
}

fn op_add(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_add"), OP_ADD.0 as u32);
    } else {
        let num1_1: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        let num2_1: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize]) =
            CffValue::Double(num1_1 + num2_1);
        stack.index = stack.index.wrapping_sub(1 as Arity);
    }
    Flow::Continue
}

fn op_sub(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_sub"), OP_SUB.0 as u32);
    } else {
        let num1_2: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        );
        let num2_2: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize]) =
            CffValue::Double(num1_2 - num2_2);
        stack.index = stack.index.wrapping_sub(1 as Arity);
    }
    Flow::Continue
}

fn op_div(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_div"), OP_DIV.0 as u32);
    } else {
        let num1_3: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        );
        let num2_3: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize]) =
            CffValue::Double(num1_3 / num2_3);
        stack.index = stack.index.wrapping_sub(1 as Arity);
    }
    Flow::Continue
}

fn op_neg(stack: &mut CffStack) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_neg"), OP_NEG.0 as u32);
    } else {
        let num_1: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize]) =
            CffValue::Double(-num_1);
    }
    Flow::Continue
}

fn op_eq(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_eq"), OP_EQ.0 as u32);
    } else {
        let num1_4: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        let num2_4: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize]) =
            CffValue::Double(if num1_4 == num2_4 { 1.0f64 } else { 0.0f64 });
        stack.index = stack.index.wrapping_sub(1 as Arity);
    }
    Flow::Continue
}

fn op_drop(stack: &mut CffStack) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_drop"), OP_DROP.0 as u32);
    } else {
        stack.index = stack.index.wrapping_sub(1 as Arity);
    }
    Flow::Continue
}

fn op_put(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_put"), OP_PUT.0 as u32);
    } else {
        let val_0: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        );
        let i_0: i32 = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        ) as i32;
        // `i_0` is a charstring-supplied operand, not a
        // trusted cursor -- Rust's `%` keeps the
        // dividend's sign, so a negative `i_0` (e.g.
        // pushing `-1` before `put`) made this a
        // negative array index once cast `as usize`
        // (wrapping to a huge value), panicking. Real
        // Type 2 charstrings only ever address this
        // array with small in-range indices, so
        // `rem_euclid` (always non-negative for a
        // positive divisor) matches well-formed input
        // exactly and just gives malformed input a
        // well-defined slot instead of a crash.
        stack.transient
            [i_0.rem_euclid(TYPE2_TRANSIENT_ARRAY as i32) as usize] =
            CffValue::Double(val_0);
        stack.index = stack.index.wrapping_sub(2 as Arity);
    }
    Flow::Continue
}

fn op_get(stack: &mut CffStack) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_get"), OP_GET.0 as u32);
    } else {
        let i_1: i32 = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        ) as i32;
        ((&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize]) =
            CffValue::Double(cffnum(
                // Same fix as `op_put` above: `rem_euclid`
                // instead of `%` so a negative `i_1`
                // can't turn into an out-of-bounds
                // array index.
                stack.transient
                    [i_1.rem_euclid(TYPE2_TRANSIENT_ARRAY as i32) as usize],
            ));
    }
    Flow::Continue
}

fn op_ifelse(stack: &mut CffStack) -> Flow {
    if stack.index < 4 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_ifelse"), OP_IFELSE.0 as u32);
    } else {
        let v2: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        let v1: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        );
        let s2: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(3 as Arity) as isize) as usize],
        );
        let s1: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(4 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(4 as Arity) as isize) as usize]) =
            CffValue::Double(if v1 <= v2 { s1 } else { s2 });
        stack.index = stack.index.wrapping_sub(3 as Arity);
    }
    Flow::Continue
}

fn op_random(stack: &mut CffStack, outline: &mut OutlineBuilderContext) -> Flow {
    if (stack.index as usize) < stack.stack.len() {
        (&mut stack.stack)[(stack.index as isize) as usize] =
            CffValue::Double(callback_draw_getrand(outline));
        stack.index = stack.index.wrapping_add(1 as Arity);
    } else {
        tracing::warn!("[libcff] Operand stack overflow in Type 2 CharString; the rest of this outline is ignored.\n");
        return Flow::Stop;
    }
    Flow::Continue
}

fn op_mul(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_mul"), OP_MUL.0 as u32);
    } else {
        let num1_5: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        let num2_5: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize]) =
            CffValue::Double(num1_5 * num2_5);
        stack.index = stack.index.wrapping_sub(1 as Arity);
    }
    Flow::Continue
}

fn op_sqrt(stack: &mut CffStack) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_sqrt"), OP_SQRT.0 as u32);
    } else {
        let num_2: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize]) =
            CffValue::Double(num_2.sqrt());
    }
    Flow::Continue
}

fn op_dup(stack: &mut CffStack) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_dup"), OP_DUP.0 as u32);
    } else if (stack.index as usize) < stack.stack.len() {
        (&mut stack.stack)[(stack.index as isize) as usize] =
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize];
        stack.index = stack.index.wrapping_add(1 as Arity);
    } else {
        tracing::warn!("[libcff] Operand stack overflow in Type 2 CharString; the rest of this outline is ignored.\n");
        return Flow::Stop;
    }
    Flow::Continue
}

fn op_exch(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_exch"), OP_EXCH.0 as u32);
    } else {
        let num1_6: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        );
        let num2_6: ::core::ffi::c_double = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        );
        ((&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize]) =
            CffValue::Double(num2_6);
        ((&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize]) =
            CffValue::Double(num1_6);
    }
    Flow::Continue
}

fn op_index(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_index"), OP_INDEX.0 as u32);
    } else {
        let n: u8 = stack.index.wrapping_sub(1 as Arity) as u8;
        // `n` is `(*stack).index - 1` truncated to `u8`
        // -- the real value is always >= 1 here (the
        // `index < 2` guard above already ensures at
        // least 2 operands are on the stack), but
        // truncation wraps `n` back to 0 whenever the
        // real value is a multiple of 256 (a
        // charstring pushing 257+ operands before
        // `index`, well within the stack's real
        // capacity). `n` is used below both as the
        // modulus and as the stack offset the index
        // operand itself was read from, so a truncated
        // 0 divided by zero and panicked. Treat it the
        // same as "not enough operands": skip the
        // operation instead.
        if n == 0 {
            tracing::warn!("[libcff] op_index ({:04x}) operand count overflowed a byte; this operation is ignored.\n", OP_INDEX.0 as u32);
        } else {
            let j_1: u8 = (n as i32
                - 1_i32
                - cffnum((&mut stack.stack)[(n as isize) as usize]) as u8
                    as i32
                    % n as i32)
                as u8;
            (&mut stack.stack)[(n as isize) as usize] =
                (&mut stack.stack)[(j_1 as isize) as usize];
        }
    }
    Flow::Continue
}

fn op_roll(stack: &mut CffStack) -> Flow {
    if stack.index < 2 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_roll"), OP_ROLL.0 as u32);
    } else {
        let mut j_2: i32 = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(1 as Arity) as isize) as usize],
        ) as i32;
        let n_0: u32 = cffnum(
            (&mut stack.stack)[(stack.index.wrapping_sub(2 as Arity) as isize) as usize],
        ) as u32;
        if stack.index < 2_u32.wrapping_add(n_0) {
            tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_roll"), OP_ROLL.0 as u32);
        } else if n_0 == 0 {
            // `n_0` (the roll's element count operand)
            // is charstring-supplied and cast `as u32`
            // from a float, which saturates any
            // negative value to 0 -- so pushing `0` or
            // a negative count for N reaches here.
            // "roll 0 elements" is a legitimate no-op
            // (the `j_2 == 0` branch a few lines down
            // already treats "nothing to rotate" as a
            // no-op the same way), but the
            // `wrapping_rem(n_0)` below panics on a
            // zero divisor -- skip it instead.
        } else {
            j_2 = (-j_2 as u32).wrapping_rem(n_0) as i32;
            if j_2 < 0_i32 {
                j_2 = (j_2 as u32).wrapping_add(n_0) as i32;
            }
            if !(j_2 == 0) {
                let last: u8 =
                    stack.index.wrapping_sub(3 as Arity) as u8;
                let first: u8 = stack
                    .index
                    .wrapping_sub(2 as Arity)
                    .wrapping_sub(n_0 as Arity)
                    as u8;
                reverse_stack(&mut *stack, first, last);
                reverse_stack(
                    &mut *stack,
                    (last as i32 - j_2 + 1_i32) as u8,
                    last,
                );
                reverse_stack(&mut *stack, first, (last as i32 - j_2) as u8);
                stack.index = stack.index.wrapping_sub(2 as Arity);
            }
        }
    }
    Flow::Continue
}

fn op_callsubr(stack: &mut CffStack, outline: &mut OutlineBuilderContext, subrs: &Subroutines<'_>, depth: u32, total_calls: &mut u32) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_callsubr"), OP_CALLSUBR.0 as u32);
    } else {
        stack.index = stack.index.wrapping_sub(1);
        // `as i32`, not `as u32`: subroutine numbers are signed, and a
        // float-to-unsigned cast turns every negative one into 0.
        let subr: i32 = cffnum(
            (&mut stack.stack)[(stack.index as isize) as usize],
        ) as i32;
        if let Some(sub_data) = locate_subr(subrs.lsubr, subrs.lsubr_bias, subr) {
            *total_calls = (*total_calls).wrapping_add(1);
            if *total_calls > MAX_TOTAL_SUBR_CALLS {
                if *total_calls == MAX_TOTAL_SUBR_CALLS + 1 {
                    tracing::warn!("[libcff] Subroutine call budget ({}) exceeded; the rest of this outline is ignored.\n", MAX_TOTAL_SUBR_CALLS as i32);
                }
            } else {
                cff_parse_outline(
                    sub_data,
                    subrs.gsubr,
                    subrs.lsubr,
                    stack,
                    outline,
                    depth + 1,
                    total_calls,
                );
            }
        } else {
            tracing::warn!("[libcff] Invalid local subroutine index for {} ({:04x}). This call is ignored.\n", ByteStr("op_callsubr"), OP_CALLSUBR.0 as u32);
        }
    }
    Flow::Continue
}

fn op_callgsubr(stack: &mut CffStack, outline: &mut OutlineBuilderContext, subrs: &Subroutines<'_>, depth: u32, total_calls: &mut u32) -> Flow {
    if stack.index < 1 as Arity {
        tracing::warn!("[libcff] Stack cannot provide enough parameters for {} ({:04x}). This operation is ignored.\n", ByteStr("op_callgsubr"), OP_CALLGSUBR.0 as u32);
    } else {
        stack.index = stack.index.wrapping_sub(1);
        // Signed, as in `op_callsubr`.
        let subr_0: i32 = cffnum(
            (&mut stack.stack)[(stack.index as isize) as usize],
        ) as i32;
        if let Some(sub_data) = locate_subr(subrs.gsubr, subrs.gsubr_bias, subr_0) {
            *total_calls = (*total_calls).wrapping_add(1);
            if *total_calls > MAX_TOTAL_SUBR_CALLS {
                if *total_calls == MAX_TOTAL_SUBR_CALLS + 1 {
                    tracing::warn!("[libcff] Subroutine call budget ({}) exceeded; the rest of this outline is ignored.\n", MAX_TOTAL_SUBR_CALLS as i32);
                }
            } else {
                cff_parse_outline(
                    sub_data,
                    subrs.gsubr,
                    subrs.lsubr,
                    stack,
                    outline,
                    depth + 1,
                    total_calls,
                );
            }
        } else {
            tracing::warn!("[libcff] Invalid global subroutine index for {} ({:04x}). This call is ignored.\n", ByteStr("op_callgsubr"), OP_CALLGSUBR.0 as u32);
        }
    }
    Flow::Continue
}

#[cfg(test)]
mod cff_parse_outline_total_calls_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;

    use crate::table::glyf::{Glyph, otfcc_new_glyf_glyph};

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
        let mut g = otfcc_new_glyf_glyph();
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
        let mut g = otfcc_new_glyf_glyph();
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
    use crate::table::glyf::otfcc_new_glyf_glyph;

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
        let mut g = otfcc_new_glyf_glyph();
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
        let mut g = otfcc_new_glyf_glyph();
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

    use crate::table::glyf::otfcc_new_glyf_glyph;

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
        let mut g = otfcc_new_glyf_glyph();
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
        // `n_0` is `cffnum(...) as u32`, a saturating float-to-int cast
        // (a *negative* N reaches the same `n_0 == 0` path this way,
        // not just a literal 0 -- see the analogous comment in
        // `cff_parse_outline_total_calls_tests`). The pre-fix code fell
        // through to `wrapping_rem(n_0)` unconditionally once the
        // "enough operands" guard passed, panicking on the zero
        // divisor -- "roll 0 elements" is a legitimate no-op (the
        // `j_2 == 0` case a few lines below already treats "nothing to
        // rotate" the same way), not a malformed-input case.
        let data: Vec<u8> = vec![139, 139, 12, 30];
        let mut stack = fresh_stack();
        run(&data, &mut stack);
        // No-op: both pushed operands (J and N) are still on the stack,
        // untouched, exactly like the pre-existing `j_2 == 0` no-op case.
        assert_eq!(stack.index, 2);
    }

    #[test]
    fn op_index_with_operand_count_multiple_of_256_does_not_panic() {
        // Push 257 zero-operands (each 1 byte: value 0 encodes as byte
        // 139), then `index` (escape `12 29` = OP_INDEX). `(*stack).index
        // - 1 == 256` truncates to `0` once cast `as u8` -- the pre-fix
        // code then used that truncated `0` as both a stack offset and a
        // modulus divisor, panicking on the divide.
        let mut data: Vec<u8> = vec![139u8; 257];
        data.push(12);
        data.push(29);
        let mut stack = fresh_stack();
        run(&data, &mut stack);
        // The operation was skipped (truncated `n == 0`), not executed
        // -- reaching this assertion at all (rather than panicking
        // mid-parse) is the regression signal. All 257 pushed operands
        // are still on the stack, untouched.
        assert_eq!(stack.index, 257);
    }
}

#[cfg(test)]
mod cff_parse_outline_subr_number_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;

    use crate::table::glyf::otfcc_new_glyf_glyph;

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
        let mut g = otfcc_new_glyf_glyph();
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
