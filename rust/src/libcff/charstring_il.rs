#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::free;

use crate::support::alloc::__caryll_allocate_clean;

use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::primitives::{Arity, Pos, ShapeId};

use crate::libcff::CffCharstringOperator;
use crate::libcff::{
    OP_CNTRMASK, OP_ENDCHAR, OP_HHCURVETO, OP_HINTMASK, OP_HLINETO, OP_HMOVETO, OP_HSTEM,
    OP_HSTEMHM, OP_HVCURVETO, OP_RCURVELINE, OP_RLINECURVE, OP_RLINETO, OP_RMOVETO, OP_RRCURVETO,
    OP_VHCURVETO, OP_VLINETO, OP_VMOVETO, OP_VSTEM, OP_VSTEMHM, OP_VVCURVETO, TYPE2_ARGUMENT_STACK,
};
use crate::support::TRUE_0;
use crate::table::glyf::{
    Contour, Glyph, MaskList, PostscriptHintMask, PostscriptStemDef, StemDefList,
};

use crate::libcff::cff_opmean::cff_get_standard_arity;
use crate::libcff::cff_writer::{
    cff_merge_cs2_operand, cff_merge_cs2_operator, cff_merge_cs2_special,
};
use crate::support::buffer::bufnew;
use crate::table::glyf::glyf_point_dup;
use crate::vf::vq::VQ;
use crate::vf::vq::{vq_copy_replace, vq_get_still, vq_minus, vq_neutral, vq_replace};
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CffInstructionType {
    Operand = 0,
    Operator = 1,
    Special = 2,
    PhantomOperator = 3,
    PhantomOperand = 4,
}
// Was a C-shaped `struct { type_0: CffInstructionType, arity: Arity,
// c2rust_unnamed: union { d: f64, i: i32 } }`. `type_0`'s five values don't
// map 1:1 onto the union's two arms -- `Operand`/`PhantomOperand` share
// `.d`, `Operator`/`Special`/`PhantomOperator` share `.i` -- so `type_0`
// stays a separate field (its own value still matters beyond "which arm is
// live": it distinguishes a real operator from a `Special` non-operator
// byte occupying the same `.i` storage, and the "Phantom" variants from
// their non-phantom counterparts, both invisible to `CffCharstringArgument`
// alone). Both arms are `Copy` (no owned heap data), so this enum stays
// `Copy` too.
#[derive(Copy, Clone)]
pub struct CffCharstringInstruction {
    pub type_0: CffInstructionType,
    pub arity: Arity,
    pub arg: CffCharstringArgument,
}
#[derive(Copy, Clone)]
pub enum CffCharstringArgument {
    D(::core::ffi::c_double),
    I(i32),
}
impl CffCharstringInstruction {
    /// Panics instead of reading union garbage if this instruction's
    /// `type_0` didn't actually imply the `D` arm -- every call site
    /// already established that via `type_0` before reaching here.
    pub fn d(&self) -> ::core::ffi::c_double {
        match self.arg {
            CffCharstringArgument::D(d) => d,
            CffCharstringArgument::I(_) => {
                panic!("CffCharstringInstruction::d called on an integer instruction")
            }
        }
    }
    pub fn i(&self) -> i32 {
        match self.arg {
            CffCharstringArgument::I(i) => i,
            CffCharstringArgument::D(_) => {
                panic!("CffCharstringInstruction::i called on a double instruction")
            }
        }
    }
    pub fn set_d(&mut self, v: ::core::ffi::c_double) {
        self.arg = CffCharstringArgument::D(v);
    }
    pub fn set_i(&mut self, v: i32) {
        self.arg = CffCharstringArgument::I(v);
    }
}
// `instr` was `__caryll_reallocate`'d in 256-instruction blocks by
// `ensure_there_is_space`, tracked by a hand-rolled `length`/`free` pair --
// exactly what `Vec` already provides, so both counters are dropped
// entirely (`length` duplicated `.len()`; `free` duplicated spare
// capacity) and the three push helpers below become plain `.push()`.
#[derive(Clone)]
pub struct CffCharstringIl {
    pub instr: Vec<CffCharstringInstruction>,
}
pub unsafe fn il_push_operand(il: *mut CffCharstringIl, x: ::core::ffi::c_double) {
    (*il).instr.push(CffCharstringInstruction {
        type_0: CffInstructionType::Operand,
        arity: 0 as Arity,
        arg: CffCharstringArgument::D(x),
    });
}
pub unsafe fn il_push_vq(il: *mut CffCharstringIl, x: VQ) {
    il_push_operand(il, vq_get_still(x) as ::core::ffi::c_double);
}
pub unsafe fn il_push_special(il: *mut CffCharstringIl, s: i32) {
    (*il).instr.push(CffCharstringInstruction {
        type_0: CffInstructionType::Special,
        arity: 0 as Arity,
        arg: CffCharstringArgument::I(s),
    });
}
pub unsafe fn il_push_op(il: *mut CffCharstringIl, op: CffCharstringOperator) {
    // The `.i` arm stays a bare `i32`: `CffInstructionType::Special` stores
    // non-operator bytes in the very same field, so the type lives on the way
    // in, not in the storage.
    (*il).instr.push(CffCharstringInstruction {
        type_0: CffInstructionType::Operator,
        arity: cff_get_standard_arity(op) as Arity,
        arg: CffCharstringArgument::I(op.0),
    });
}
unsafe fn il_moveto(il: *mut CffCharstringIl, dx: VQ, dy: VQ) {
    il_push_vq(il, dx);
    il_push_vq(il, dy);
    il_push_op(il, OP_RMOVETO);
}
unsafe fn il_lineto(il: *mut CffCharstringIl, dx: VQ, dy: VQ) {
    il_push_vq(il, dx);
    il_push_vq(il, dy);
    il_push_op(il, OP_RLINETO);
}
unsafe fn il_curveto(
    il: *mut CffCharstringIl,
    dx1: VQ,
    dy1: VQ,
    dx2: VQ,
    dy2: VQ,
    dx3: VQ,
    dy3: VQ,
) {
    il_push_vq(il, dx1);
    il_push_vq(il, dy1);
    il_push_vq(il, dx2);
    il_push_vq(il, dy2);
    il_push_vq(il, dx3);
    il_push_vq(il, dy3);
    il_push_op(il, OP_RRCURVETO);
}
unsafe fn _il_push_maskgroup(
    il: *mut CffCharstringIl,
    masks: *const MaskList,
    contours: u16,
    points: u16,
    nh: u16,
    nv: u16,
    jm: *mut u16,
    op: CffCharstringOperator,
) {
    let masks: &Vec<PostscriptHintMask> = &*masks;
    let n: ShapeId = masks.len() as ShapeId;
    while (*jm as ::core::ffi::c_int) < n as ::core::ffi::c_int
        && ((masks[*jm as usize].contours_before as ::core::ffi::c_int)
            < contours as ::core::ffi::c_int
            || masks[*jm as usize].contours_before as ::core::ffi::c_int
                == contours as ::core::ffi::c_int
                && masks[*jm as usize].points_before as ::core::ffi::c_int
                    <= points as ::core::ffi::c_int)
    {
        il_push_op(il, op);
        let mut mask_byte: u8 = 0_u8;
        let mut bits: u8 = 0_u8;
        let mut j: u16 = 0_u16;
        while (j as ::core::ffi::c_int) < nh as ::core::ffi::c_int {
            mask_byte = ((mask_byte as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | masks[*jm as usize].mask_h[j as usize] as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int) as u8;
            bits = (bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
            if bits as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                il_push_special(il, mask_byte as i32);
                bits = 0_u8;
            }
            j = j.wrapping_add(1);
        }
        let mut j_0: u16 = 0_u16;
        while (j_0 as ::core::ffi::c_int) < nv as ::core::ffi::c_int {
            mask_byte = ((mask_byte as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | masks[*jm as usize].mask_v[j_0 as usize] as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int) as u8;
            bits = (bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
            if bits as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                il_push_special(il, mask_byte as i32);
                bits = 0_u8;
            }
            j_0 = j_0.wrapping_add(1);
        }
        if bits != 0 {
            mask_byte = ((mask_byte as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - bits as ::core::ffi::c_int)
                as u8;
            il_push_special(il, mask_byte as i32);
        }
        *jm = (*jm as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
    }
}
unsafe fn il_push_masks(
    il: *mut CffCharstringIl,
    g: *const Glyph,
    contours: u16,
    points: u16,
    jh: *mut u16,
    jm: *mut u16,
) {
    if (*g).stem_h.is_empty() && (*g).stem_v.is_empty() {
        return;
    }
    let stem_h_len = (*g).stem_h.len() as u16;
    let stem_v_len = (*g).stem_v.len() as u16;
    _il_push_maskgroup(
        il,
        &raw const (*g).contour_masks,
        contours,
        points,
        stem_h_len,
        stem_v_len,
        jh,
        OP_CNTRMASK,
    );
    _il_push_maskgroup(
        il,
        &raw const (*g).hint_masks,
        contours,
        points,
        stem_h_len,
        stem_v_len,
        jm,
        OP_HINTMASK,
    );
}
unsafe fn _il_push_stemgroup(
    il: *mut CffCharstringIl,
    stems: *const StemDefList,
    hasmask: bool,
    haswidth: bool,
    ophm: CffCharstringOperator,
    oph: CffCharstringOperator,
) {
    if stems.is_null() || (*stems).is_empty() {
        return;
    }
    let stems: &Vec<PostscriptStemDef> = &*stems;
    let mut ref_0: Pos = 0 as ::core::ffi::c_int as Pos;
    let mut nn: u16 = (if haswidth as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as u16;
    let mut j: u16 = 0_u16;
    while (j as usize) < stems.len() {
        il_push_operand(
            il,
            stems[j as usize].position as ::core::ffi::c_double - ref_0 as ::core::ffi::c_double,
        );
        il_push_operand(il, stems[j as usize].width as ::core::ffi::c_double);
        ref_0 = stems[j as usize].position + stems[j as usize].width;
        nn = nn.wrapping_add(1);
        if nn as u32 >= TYPE2_ARGUMENT_STACK {
            if hasmask {
                il_push_op(il, OP_HSTEMHM);
            } else {
                il_push_op(il, OP_HSTEM);
            }
            (*(*il)
                .instr
                .as_mut_ptr()
                .offset(((*il).instr.len() as u32).wrapping_sub(1_u32) as isize))
            .arity = nn as Arity;
            nn = 0_u16;
        }
        j = j.wrapping_add(1);
    }
    if hasmask {
        il_push_op(il, ophm);
    } else {
        il_push_op(il, oph);
    }
    (*(*il)
        .instr
        .as_mut_ptr()
        .offset(((*il).instr.len() as u32).wrapping_sub(1_u32) as isize))
    .arity = nn as Arity;
}
unsafe fn il_push_stems(
    il: *mut CffCharstringIl,
    g: *const Glyph,
    hasmask: bool,
    haswidth: bool,
) {
    _il_push_stemgroup(
        il,
        &raw const (*g).stem_h,
        hasmask,
        haswidth,
        OP_HSTEMHM,
        OP_HSTEM,
    );
    _il_push_stemgroup(
        il,
        &raw const (*g).stem_v,
        hasmask,
        haswidth,
        OP_VSTEMHM,
        OP_VSTEM,
    );
}
pub unsafe fn cff_compile_glyph_to_il(
    g: *const Glyph,
    default_width: u16,
    nominal_width: u16,
) -> *mut CffCharstringIl {
    let il: *mut CffCharstringIl =
        Box::into_raw(Box::new(CffCharstringIl { instr: Vec::new() }));
    let temp_contours: *mut Contour;
    let mut x: VQ = (vq_neutral)();
    let mut y: VQ = (vq_neutral)();
    temp_contours = __caryll_allocate_clean(
        (::core::mem::size_of::<Contour>() as usize).wrapping_mul((*g).contours.len()),
        149 as ::core::ffi::c_ulong,
    ) as *mut Contour;
    let mut c: u16 = 0_u16;
    while (c as usize) < (*g).contours.len() {
        let contour: *const Contour = &(&(*g).contours)[c as usize];
        let newcontour: *mut Contour = temp_contours.offset(c as isize);
        // The comment this replaced argued a plain `=` here was safe
        // because dropping an all-zero (not garbage) `Contour` is a no-op
        // -- but that belief is exactly what this crate's own Miri-found
        // UB (see [[otfcc-vec-field-assign-needs-calloc]]) retracted: a
        // plain assignment drops the *old* value first, and constructing
        // an all-zero bit pattern as a typed `Vec` value is UB the
        // instant it happens, independent of whether the drop body goes
        // on to read/free anything. `ptr::write` doesn't run that drop at
        // all, which is what a freshly calloc'd, not-yet-valid slot
        // actually needs.
        ::core::ptr::write(newcontour, Vec::new());
        let mut j: ShapeId = 0 as ShapeId;
        while (j as usize) < (*contour).len() {
            (*newcontour).push(glyf_point_dup((&(*contour))[j as usize].clone()));
            j = j.wrapping_add(1);
        }
        if (*newcontour).len() > 2_usize
            && (&(*newcontour))[(*newcontour).len().wrapping_sub(1_usize)].on_curve == 0
        {
            let first = (&(*newcontour))[0_usize].clone();
            (*newcontour).push(glyf_point_dup(first));
        }
        let mut j_0: ShapeId = 0 as ShapeId;
        while (j_0 as usize) < (*newcontour).len() {
            let dx: VQ = vq_minus((&(*newcontour))[j_0 as usize].x.clone(), x.clone());
            let dy: VQ = vq_minus((&(*newcontour))[j_0 as usize].y.clone(), y.clone());
            vq_copy_replace(&raw mut x, (&(*newcontour))[j_0 as usize].x.clone());
            vq_copy_replace(&raw mut y, (&(*newcontour))[j_0 as usize].y.clone());
            vq_replace(&raw mut (&mut (*newcontour))[j_0 as usize].x, dx);
            vq_replace(&raw mut (&mut (*newcontour))[j_0 as usize].y, dy);
            j_0 = j_0.wrapping_add(1);
        }
        c = c.wrapping_add(1);
    }
    // `x`/`y` are plain owned locals, never moved out, so they auto-drop
    // when this function returns -- no explicit dispose call is needed.
    let hasmask: bool = !(*g).hint_masks.is_empty() || !(*g).contour_masks.is_empty();
    let glyph_adw_const: Pos = vq_get_still((*g).advance_width.clone()) as Pos;
    let haswidth: bool = glyph_adw_const != default_width as ::core::ffi::c_int as Pos;
    if haswidth {
        // `glyph_adw_const` is attacker-controlled JSON (`advanceWidth`),
        // cast from `f64` -- `as c_int` already saturates a huge magnitude
        // to `i32::MIN`/`MAX` rather than wrapping, but the *subtraction*
        // right after was still plain `-`, so a saturated `i32::MIN` minus
        // a positive `nominal_width` underflowed past `i32::MIN`: a panic
        // under debug-assertions (found by fuzzing), silent wraparound
        // producing a nonsensically wrong advance-width delta in an
        // ordinary release build otherwise. `saturating_sub` makes the
        // extreme case clamp instead of either.
        il_push_operand(
            il,
            (glyph_adw_const as ::core::ffi::c_int)
                .saturating_sub(nominal_width as ::core::ffi::c_int)
                as ::core::ffi::c_double,
        );
    }
    il_push_stems(il, g, hasmask, haswidth);
    let mut contours_sofar: ShapeId = 0 as ShapeId;
    let mut points_sofar: ShapeId = 0 as ShapeId;
    let mut jh: ShapeId = 0 as ShapeId;
    let mut jm: ShapeId = 0 as ShapeId;
    if hasmask {
        il_push_masks(
            il,
            g,
            contours_sofar as u16,
            points_sofar as u16,
            &raw mut jh,
            &raw mut jm,
        );
    }
    let mut c_0: ShapeId = 0 as ShapeId;
    while (c_0 as usize) < (*g).contours.len() {
        let contour_0: *const Contour = temp_contours.offset(c_0 as isize);
        let n: ShapeId = (*contour_0).len() as ShapeId;
        if !(n as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
            il_moveto(
                il,
                (&(*contour_0))[0_usize].x.clone(),
                (&(*contour_0))[0_usize].y.clone(),
            );
            points_sofar = points_sofar.wrapping_add(1);
            if hasmask {
                il_push_masks(
                    il,
                    g,
                    contours_sofar as u16,
                    points_sofar as u16,
                    &raw mut jh,
                    &raw mut jm,
                );
            }
            let mut j_1: ShapeId = 1 as ShapeId;
            while (j_1 as ::core::ffi::c_int) < n as ::core::ffi::c_int {
                if (&(*contour_0))[j_1 as usize].on_curve != 0 {
                    il_lineto(
                        il,
                        (&(*contour_0))[j_1 as usize].x.clone(),
                        (&(*contour_0))[j_1 as usize].y.clone(),
                    );
                    points_sofar =
                        (points_sofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
                } else if (j_1 as ::core::ffi::c_int)
                    < n as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    && (&(*contour_0))
                        [(j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        .on_curve
                        == 0
                    && (&(*contour_0))
                        [(j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                        .on_curve as ::core::ffi::c_int
                        != 0
                {
                    il_curveto(
                        il,
                        (&(*contour_0))[j_1 as usize].x.clone(),
                        (&(*contour_0))[j_1 as usize].y.clone(),
                        (&(*contour_0))
                            [(j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                            .x
                            .clone(),
                        (&(*contour_0))
                            [(j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                            .y
                            .clone(),
                        (&(*contour_0))
                            [(j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                            .x
                            .clone(),
                        (&(*contour_0))
                            [(j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                            .y
                            .clone(),
                    );
                    points_sofar =
                        (points_sofar as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as ShapeId;
                    j_1 = (j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as ShapeId;
                } else {
                    il_lineto(
                        il,
                        (&(*contour_0))[j_1 as usize].x.clone(),
                        (&(*contour_0))[j_1 as usize].y.clone(),
                    );
                    points_sofar = points_sofar.wrapping_add(1);
                }
                if hasmask {
                    il_push_masks(
                        il,
                        g,
                        contours_sofar as u16,
                        points_sofar as u16,
                        &raw mut jh,
                        &raw mut jm,
                    );
                }
                j_1 = j_1.wrapping_add(1);
            }
            contours_sofar =
                (contours_sofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
            points_sofar = 0 as ShapeId;
        }
        c_0 = c_0.wrapping_add(1);
    }
    il_push_op(il, OP_ENDCHAR);
    let mut c_1: ShapeId = 0 as ShapeId;
    while (c_1 as usize) < (*g).contours.len() {
        // Each slot is a genuine `Vec<Point>` placement-constructed above,
        // so this is an ordinary drop -- not the calloc-garbage case (see
        // the field-assignment comment above).
        ::core::ptr::drop_in_place(temp_contours.offset(c_1 as isize));
        c_1 = c_1.wrapping_add(1);
    }
    free(temp_contours as *mut ::core::ffi::c_void);
    return il;
}
unsafe fn il_matchtype(
    il: *mut CffCharstringIl,
    j: u32,
    k: u32,
    t: CffInstructionType,
) -> bool {
    if k >= (*il).instr.len() as u32 {
        return false;
    }
    let mut m: u32 = j;
    while m < k {
        if (*(*il).instr.as_mut_ptr().offset(m as isize)).type_0 as ::core::ffi::c_uint
            != t as ::core::ffi::c_uint
        {
            return false;
        }
        m = m.wrapping_add(1);
    }
    return true;
}
unsafe fn il_matchop(
    il: *mut CffCharstringIl,
    j: u32,
    op: CffCharstringOperator,
) -> bool {
    if (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 != CffInstructionType::Operator {
        return false;
    }
    if (*(*il).instr.as_mut_ptr().offset(j as isize)).i() != op.0 {
        return false;
    }
    return true;
}
/// Collapse `op` into `op2` when the operands flagged in `zeros` are all zero.
///
/// `zeros` was a vararg list of `arity` ints -- the count implied by
/// `cff_get_standard_arity(op)` and trusted, never checked. As a slice the two can
/// be compared, and the flags read as the booleans they always were.
unsafe fn zroll(
    il: *mut CffCharstringIl,
    j: u32,
    op: CffCharstringOperator,
    op2: CffCharstringOperator,
    zeros: &[bool],
) -> u8 {
    let arity: u8 = cff_get_standard_arity(op);
    if arity as ::core::ffi::c_int > 16 as ::core::ffi::c_int
        || j.wrapping_add(arity as u32) >= (*il).instr.len() as u32
    {
        return 0_u8;
    }
    if (j == 0_u32
        || !il_matchtype(
            il,
            j.wrapping_sub(1_u32),
            j,
            CffInstructionType::PhantomOperator,
        ))
        && il_matchop(il, j.wrapping_add(arity as u32), op) as ::core::ffi::c_int != 0
        && il_matchtype(
            il,
            j,
            j.wrapping_add(arity as u32),
            CffInstructionType::Operand,
        ) as ::core::ffi::c_int
            != 0
    {
        let mut check: u8 = TRUE_0 as u8;
        let mut result_arity: u8 = arity;
        let mut mask: [bool; 16] = [false; 16];
        debug_assert_eq!(
            zeros.len(),
            arity as usize,
            "zroll: flag count must match the operator's arity"
        );
        let mut m: u32 = 0_u32;
        while m < arity as u32 {
            let checkzero: bool = zeros[m as usize];
            mask[m as usize] = checkzero;
            if checkzero {
                result_arity = (result_arity as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u8;
                check = (check as ::core::ffi::c_int != 0
                    && (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(m) as isize)).d()
                        == 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int as u8;
            }
            m = m.wrapping_add(1);
        }
        if check != 0 {
            let mut m_0: u32 = 0_u32;
            while m_0 < arity as u32 {
                if mask[m_0 as usize] {
                    (*(*il)
                        .instr
                        .as_mut_ptr()
                        .offset(j.wrapping_add(m_0) as isize))
                    .type_0 = CffInstructionType::PhantomOperand;
                }
                m_0 = m_0.wrapping_add(1);
            }
            (*(*il)
                .instr
                .as_mut_ptr()
                .offset(j.wrapping_add(arity as u32) as isize))
            .set_i(op2.0);
            (*(*il)
                .instr
                .as_mut_ptr()
                .offset(j.wrapping_add(arity as u32) as isize))
            .arity = result_arity as Arity;
            return arity;
        } else {
            return 0_u8;
        }
    } else {
        return 0_u8;
    };
}
unsafe fn opop_roll(
    il: *mut CffCharstringIl,
    j: u32,
    op1: CffCharstringOperator,
    arity: i32,
    op2: CffCharstringOperator,
    resultop: CffCharstringOperator,
) -> u8 {
    if j.wrapping_add(1_u32).wrapping_add(arity as u32) >= (*il).instr.len() as u32 {
        return 0_u8;
    }
    let current: *mut CffCharstringInstruction = (*il).instr.as_mut_ptr().offset(j as isize);
    let nextop: *mut CffCharstringInstruction = (*il)
        .instr
        .as_mut_ptr()
        .offset(j.wrapping_add(1_u32).wrapping_add(arity as u32) as isize);
    if il_matchop(il, j, op1) as ::core::ffi::c_int != 0
        && il_matchtype(
            il,
            j.wrapping_add(1_u32),
            j.wrapping_add(1_u32).wrapping_add(arity as u32),
            CffInstructionType::Operand,
        ) as ::core::ffi::c_int
            != 0
        && il_matchop(il, j.wrapping_add(1_u32).wrapping_add(arity as u32), op2)
            as ::core::ffi::c_int
            != 0
        && (*current).arity.wrapping_add((*nextop).arity) <= TYPE2_ARGUMENT_STACK
    {
        (*current).type_0 = CffInstructionType::PhantomOperator;
        (*nextop).set_i(resultop.0);
        (*nextop).arity = (*nextop).arity.wrapping_add((*current).arity);
        return (arity + 1_i32) as u8;
    } else {
        return 0_u8;
    };
}
unsafe fn hvlineto_roll(il: *mut CffCharstringIl, j: u32) -> u8 {
    if j.wrapping_add(3_u32) >= (*il).instr.len() as u32 {
        return 0_u8;
    }
    let current: *mut CffCharstringInstruction =
        (*il).instr.as_mut_ptr().offset(j as isize);
    // `checkdelta`'s computation reads `current.i()`, which is only valid
    // once `current`'s `type_0` is confirmed `Operator` -- moved below the
    // `il_matchop(HLINETO)/(VLINETO)` check that establishes that (the
    // original C-shaped union could read this before the check with no ill
    // effect beyond a discarded garbage value; a real enum panics instead,
    // so the check now runs first, preserving the exact same behavior --
    // `checkdelta` was always discarded whenever that check failed).
    if !(il_matchop(il, j, OP_HLINETO) as ::core::ffi::c_int != 0
        || il_matchop(il, j, OP_VLINETO) as ::core::ffi::c_int != 0)
    {
        return 0_u8;
    }
    let checkdelta: u32 = (if ((*current).arity & 1 as Arity != 0) as ::core::ffi::c_int
        ^ ((*current).i() == OP_VLINETO.0) as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as u32;
    if il_matchop(il, j.wrapping_add(3_u32), OP_RLINETO) as ::core::ffi::c_int != 0
        && il_matchtype(
            il,
            j.wrapping_add(1_u32),
            j.wrapping_add(3_u32),
            CffInstructionType::Operand,
        ) as ::core::ffi::c_int
            != 0
        && (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(checkdelta) as isize))
        .d() == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*current).arity.wrapping_add(1 as Arity) <= TYPE2_ARGUMENT_STACK
    {
        (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(checkdelta) as isize))
        .type_0 = CffInstructionType::PhantomOperand;
        (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 = CffInstructionType::PhantomOperator;
        let current_i = (*current).i();
        (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(3_u32) as isize))
        .set_i(current_i);
        (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(3_u32) as isize))
        .arity = (*current).arity.wrapping_add(1 as Arity);
        return 3_u8;
    } else {
        return 0_u8;
    };
}
unsafe fn hvvhcurve_roll(il: *mut CffCharstringIl, j: u32) -> u8 {
    if !il_matchop(il, j, OP_HVCURVETO) && !il_matchop(il, j, OP_VHCURVETO) {
        return 0_u8;
    }
    let current: *mut CffCharstringInstruction =
        (*il).instr.as_mut_ptr().offset(j as isize);
    if j.wrapping_add(7_u32) >= (*il).instr.len() as u32 || (*current).arity & 1 as Arity != 0 {
        return 0_u8;
    }
    let hvcase: bool = ((*current).arity >> 2 as ::core::ffi::c_int & 1 as Arity != 0)
        as ::core::ffi::c_int
        ^ ((*current).i() == OP_HVCURVETO.0) as ::core::ffi::c_int
        != 0;
    let checkdelta1: u32 = (if hvcase as ::core::ffi::c_int != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as u32;
    let checkdelta2: u32 = (if hvcase as ::core::ffi::c_int != 0 {
        5 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as u32;
    if il_matchop(il, j.wrapping_add(7_u32), OP_RRCURVETO) as ::core::ffi::c_int != 0
        && il_matchtype(
            il,
            j.wrapping_add(1_u32),
            j.wrapping_add(7_u32),
            CffInstructionType::Operand,
        ) as ::core::ffi::c_int
            != 0
        && (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(checkdelta1) as isize))
        .d() == 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(checkdelta2) as isize))
        .d() == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*current).arity.wrapping_add(4 as Arity) <= TYPE2_ARGUMENT_STACK
        {
            (*(*il)
                .instr
                .as_mut_ptr()
                .offset(j.wrapping_add(checkdelta1) as isize))
            .type_0 = CffInstructionType::PhantomOperand;
            (*(*il)
                .instr
                .as_mut_ptr()
                .offset(j.wrapping_add(checkdelta2) as isize))
            .type_0 = CffInstructionType::PhantomOperand;
            (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 =
                CffInstructionType::PhantomOperator;
            let current_i = (*current).i();
            (*(*il)
                .instr
                .as_mut_ptr()
                .offset(j.wrapping_add(7_u32) as isize))
            .set_i(current_i);
            (*(*il)
                .instr
                .as_mut_ptr()
                .offset(j.wrapping_add(7_u32) as isize))
            .arity = (*current).arity.wrapping_add(4 as Arity);
            return 7_u8;
        } else if (*current).arity.wrapping_add(5 as Arity) <= TYPE2_ARGUMENT_STACK {
            (*(*il)
                .instr
                .as_mut_ptr()
                .offset(j.wrapping_add(checkdelta1) as isize))
            .type_0 = CffInstructionType::PhantomOperand;
            (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 =
                CffInstructionType::PhantomOperator;
            let current_i = (*current).i();
            (*(*il)
                .instr
                .as_mut_ptr()
                .offset(j.wrapping_add(7_u32) as isize))
            .set_i(current_i);
            (*(*il)
                .instr
                .as_mut_ptr()
                .offset(j.wrapping_add(7_u32) as isize))
            .arity = (*current).arity.wrapping_add(5 as Arity);
            if hvcase {
                let t: ::core::ffi::c_double = (*(*il)
                    .instr
                    .as_mut_ptr()
                    .offset(j.wrapping_add(5_u32) as isize))
                .d();
                let swap_val = (*(*il)
                    .instr
                    .as_mut_ptr()
                    .offset(j.wrapping_add(6_u32) as isize))
                .d();
                (*(*il)
                    .instr
                    .as_mut_ptr()
                    .offset(j.wrapping_add(5_u32) as isize))
                .set_d(swap_val);
                (*(*il)
                    .instr
                    .as_mut_ptr()
                    .offset(j.wrapping_add(6_u32) as isize))
                .set_d(t);
            }
            return 7_u8;
        } else {
            return 0_u8;
        }
    } else {
        return 0_u8;
    };
}
unsafe fn hhvvcurve_roll(il: *mut CffCharstringIl, j: u32) -> u8 {
    if !il_matchop(il, j, OP_HHCURVETO) && !il_matchop(il, j, OP_VVCURVETO) {
        return 0_u8;
    }
    let current: *mut CffCharstringInstruction =
        (*il).instr.as_mut_ptr().offset(j as isize);
    if j.wrapping_add(7_u32) >= (*il).instr.len() as u32 {
        return 0_u8;
    }
    let hh: bool = (*current).i() == OP_HHCURVETO.0;
    let checkdelta1: u32 = (if hh as ::core::ffi::c_int != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as u32;
    let checkdelta2: u32 = (if hh as ::core::ffi::c_int != 0 {
        6 as ::core::ffi::c_int
    } else {
        5 as ::core::ffi::c_int
    }) as u32;
    if il_matchop(il, j.wrapping_add(7_u32), OP_RRCURVETO) as ::core::ffi::c_int != 0
        && il_matchtype(
            il,
            j.wrapping_add(1_u32),
            j.wrapping_add(7_u32),
            CffInstructionType::Operand,
        ) as ::core::ffi::c_int
            != 0
        && (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(checkdelta1) as isize))
        .d() == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(checkdelta2) as isize))
        .d() == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*current).arity.wrapping_add(4 as Arity) <= TYPE2_ARGUMENT_STACK
    {
        (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(checkdelta1) as isize))
        .type_0 = CffInstructionType::PhantomOperand;
        (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(checkdelta2) as isize))
        .type_0 = CffInstructionType::PhantomOperand;
        (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 = CffInstructionType::PhantomOperator;
        let current_i = (*current).i();
        (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(7_u32) as isize))
        .set_i(current_i);
        (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(7_u32) as isize))
        .arity = (*current).arity.wrapping_add(4 as Arity);
        return 7_u8;
    } else {
        return 0_u8;
    };
}
unsafe fn nextstop(il: *mut CffCharstringIl, j: u32) -> u32 {
    let mut delta: u32 = 0_u32;
    while j.wrapping_add(delta) < (*il).instr.len() as u32
        && (*(*il)
            .instr
            .as_mut_ptr()
            .offset(j.wrapping_add(delta) as isize))
        .type_0
            == CffInstructionType::Operand
    {
        delta = delta.wrapping_add(1);
    }
    return delta;
}
unsafe fn decide_advance(il: *mut CffCharstringIl, j: u32, mut _optimize_level: u8) -> u8 {
    let mut r: u8;
    r = zroll(il, j, OP_RLINETO, OP_HLINETO, &[false, true]);
    if r != 0 {
        return r;
    }
    r = zroll(il, j, OP_RLINETO, OP_VLINETO, &[true, false]);
    if r != 0 {
        return r;
    }
    r = zroll(il, j, OP_RMOVETO, OP_HMOVETO, &[false, true]);
    if r != 0 {
        return r;
    }
    r = zroll(il, j, OP_RMOVETO, OP_VMOVETO, &[true, false]);
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        OP_RRCURVETO,
        OP_HVCURVETO,
        &[false, true, false, false, true, false],
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        OP_RRCURVETO,
        OP_VHCURVETO,
        &[true, false, false, false, false, true],
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        OP_RRCURVETO,
        OP_HHCURVETO,
        &[false, true, false, false, false, true],
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        OP_RRCURVETO,
        OP_VVCURVETO,
        &[true, false, false, false, true, false],
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_RRCURVETO, 6_i32, OP_RRCURVETO, OP_RRCURVETO);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_RRCURVETO, 2_i32, OP_RLINETO, OP_RCURVELINE);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_RLINETO, 6_i32, OP_RRCURVETO, OP_RLINECURVE);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_RLINETO, 2_i32, OP_RLINETO, OP_RLINETO);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_HSTEMHM, 0_i32, OP_HINTMASK, OP_HINTMASK);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_VSTEMHM, 0_i32, OP_HINTMASK, OP_HINTMASK);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_HSTEMHM, 0_i32, OP_CNTRMASK, OP_CNTRMASK);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_VSTEMHM, 0_i32, OP_CNTRMASK, OP_CNTRMASK);
    if r != 0 {
        return r;
    }
    r = hvlineto_roll(il, j);
    if r != 0 {
        return r;
    }
    r = hhvvcurve_roll(il, j);
    if r != 0 {
        return r;
    }
    r = hvvhcurve_roll(il, j);
    if r != 0 {
        return r;
    }
    r = nextstop(il, j) as u8;
    if r != 0 {
        return r;
    }
    return 1_u8;
}
pub unsafe fn cff_optimize_il(il: *mut CffCharstringIl, options: &Options) {
    if !options.cff_roll_char_string {
        return;
    }
    let mut j: u32 = 0_u32;
    while j < (*il).instr.len() as u32 {
        j = j.wrapping_add(decide_advance(il, j, options.cff_roll_char_string as u8) as u32);
    }
}
pub unsafe fn cff_build_il(il: *mut CffCharstringIl) -> *mut Buffer {
    let blob: *mut Buffer = bufnew();
    let mut j: u16 = 0_u16;
    while (j as u32) < (*il).instr.len() as u32 {
        match (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                cff_merge_cs2_operand(blob, (*(*il).instr.as_mut_ptr().offset(j as isize)).d());
            }
            1 => {
                cff_merge_cs2_operator(
                    blob,
                    CffCharstringOperator((*(*il).instr.as_mut_ptr().offset(j as isize)).i()),
                );
            }
            2 => {
                cff_merge_cs2_special(
                    blob,
                    (*(*il).instr.as_mut_ptr().offset(j as isize)).i() as u8,
                );
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return blob;
}
pub unsafe fn cff_shrink_il(il: *mut CffCharstringIl) -> *mut CffCharstringIl {
    let out: *mut CffCharstringIl =
        Box::into_raw(Box::new(CffCharstringIl { instr: Vec::new() }));
    let mut j: u16 = 0_u16;
    while (j as u32) < (*il).instr.len() as u32 {
        match (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                il_push_operand(out, (*(*il).instr.as_mut_ptr().offset(j as isize)).d());
            }
            1 => {
                il_push_op(
                    out,
                    CffCharstringOperator((*(*il).instr.as_mut_ptr().offset(j as isize)).i()),
                );
            }
            2 => {
                il_push_special(out, (*(*il).instr.as_mut_ptr().offset(j as isize)).i());
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return out;
}
pub unsafe fn cff_i_lmerge_il(self_0: *mut CffCharstringIl, il: *mut CffCharstringIl) {
    let mut j: u16 = 0_u16;
    while (j as u32) < (*il).instr.len() as u32 {
        match (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                il_push_operand(self_0, (*(*il).instr.as_mut_ptr().offset(j as isize)).d());
            }
            1 => {
                il_push_op(
                    self_0,
                    CffCharstringOperator((*(*il).instr.as_mut_ptr().offset(j as isize)).i()),
                );
            }
            2 => {
                il_push_special(self_0, (*(*il).instr.as_mut_ptr().offset(j as isize)).i());
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe fn instruction_eq(
    z1: *mut CffCharstringInstruction,
    z2: *mut CffCharstringInstruction,
) -> bool {
    if (*z1).type_0 as ::core::ffi::c_uint == (*z2).type_0 as ::core::ffi::c_uint {
        if (*z1).type_0 == CffInstructionType::Operand
            || (*z1).type_0 == CffInstructionType::PhantomOperand
        {
            return (*z1).d() == (*z2).d();
        } else {
            return (*z1).i() == (*z2).i();
        }
    } else {
        return false;
    };
}
pub unsafe fn cff_il_equal(a: *mut CffCharstringIl, b: *mut CffCharstringIl) -> bool {
    if a.is_null() || b.is_null() {
        return false;
    }
    if (*a).instr.len() as u32 != (*b).instr.len() as u32 {
        return false;
    }
    let mut j: u32 = 0_u32;
    while j < (*a).instr.len() as u32 {
        if !instruction_eq(
            (*a).instr.as_mut_ptr().offset(j as isize),
            (*b).instr.as_mut_ptr().offset(j as isize),
        ) {
            return false;
        }
        j = j.wrapping_add(1);
    }
    return true;
}

#[cfg(test)]
mod cff_compile_glyph_to_il_tests {
    use super::*;
    use crate::table::glyf::{Point, otfcc_new_glyf_glyph};
    use crate::vf::vq::vq_create_still;

    // `cff_compile_glyph_to_il` calloc's a scratch `*mut Contour` array
    // (one slot per source contour) and used to write each slot's first
    // value via a plain `*newcontour = Vec::new();` -- an all-zero bit
    // pattern is not a valid `Vec`, so that assignment's implicit drop of
    // the "old" value was UB under Miri regardless of whether a real
    // contour ever made it through afterward (see
    // [[otfcc-vec-field-assign-needs-calloc]]; this crate's own
    // in-code comment arguing the opposite -- "a Vec with capacity 0
    // never touches its pointer field when dropped" -- is exactly the
    // belief the project's README later retracted). Needs at least one
    // contour with at least one point to actually reach the scratch
    // array's write at all.
    #[test]
    fn compiling_a_glyph_with_one_contour_does_not_construct_invalid_scratch_values() {
        unsafe {
            let mut g = otfcc_new_glyf_glyph();
            g.contours.push(vec![Point {
                x: vq_create_still(0.0),
                y: vq_create_still(0.0),
                on_curve: 1,
            }]);
            let il = cff_compile_glyph_to_il(&*g as *const Glyph, 0, 0);
            assert!(!il.is_null());
            drop(Box::from_raw(il));
        }
    }
}
