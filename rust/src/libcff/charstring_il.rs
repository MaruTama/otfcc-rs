#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
#![allow(improper_ctypes_definitions)] // VQ now owns a Vec; these extern "C" fns are internal-only (vtable dispatch, no real FFI boundary) -- goes away with the vtable/extern "C" cleanup, see rust/README.md
use libc::{free};



use crate::support::alloc::{__caryll_allocate_clean};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{Arity, Pos, ShapeId};

use crate::libcff::CffCharstringOperator;
use crate::libcff::{OP_CNTRMASK, OP_ENDCHAR, OP_HHCURVETO, OP_HINTMASK, OP_HLINETO, OP_HMOVETO, OP_HSTEM, OP_HSTEMHM, OP_HVCURVETO, OP_RCURVELINE, OP_RLINECURVE, OP_RLINETO, OP_RMOVETO, OP_RRCURVETO, OP_VHCURVETO, OP_VLINETO, OP_VMOVETO, OP_VSTEM, OP_VSTEMHM, OP_VVCURVETO, TYPE2_ARGUMENT_STACK};
use crate::support::{TRUE_0};
use crate::table::glyf::{Contour, Glyph, MaskList, PostscriptHintMask, PostscriptStemDef, StemDefList};

use crate::vf::vq::VQ;
use crate::libcff::cff_opmean::{cff_get_standard_arity};
use crate::libcff::cff_writer::{cff_merge_cs2_operand, cff_merge_cs2_operator, cff_merge_cs2_special};
use crate::support::buffer::{bufnew};
use crate::table::glyf::{glyf_point_dup};
use crate::vf::vq::{I_VQ};
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
#[repr(C)]
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
#[repr(C)]
pub struct CffCharstringIl {
    pub instr: Vec<CffCharstringInstruction>,
}
pub unsafe fn il_push_operand(
    mut il: *mut CffCharstringIl,
    mut x: ::core::ffi::c_double,
) {
    (*il).instr.push(CffCharstringInstruction {
        type_0: CffInstructionType::Operand,
        arity: 0 as Arity,
        arg: CffCharstringArgument::D(x),
    });
}
pub unsafe fn il_push_vq(mut il: *mut CffCharstringIl, mut x: VQ) {
    il_push_operand(
        il,
        I_VQ.get_still.expect("non-null function pointer")(x) as ::core::ffi::c_double,
    );
}
pub unsafe fn il_push_special(mut il: *mut CffCharstringIl, mut s: i32) {
    (*il).instr.push(CffCharstringInstruction {
        type_0: CffInstructionType::Special,
        arity: 0 as Arity,
        arg: CffCharstringArgument::I(s),
    });
}
pub unsafe fn il_push_op(
    mut il: *mut CffCharstringIl,
    mut op: CffCharstringOperator,
) {
    // The `.i` arm stays a bare `i32`: `CffInstructionType::Special` stores
    // non-operator bytes in the very same field, so the type lives on the way
    // in, not in the storage.
    (*il).instr.push(CffCharstringInstruction {
        type_0: CffInstructionType::Operator,
        arity: cff_get_standard_arity(op) as Arity,
        arg: CffCharstringArgument::I(op.0),
    });
}
unsafe fn il_moveto(mut il: *mut CffCharstringIl, mut dx: VQ, mut dy: VQ) {
    il_push_vq(il, dx);
    il_push_vq(il, dy);
    il_push_op(il, OP_RMOVETO);
}
unsafe fn il_lineto(mut il: *mut CffCharstringIl, mut dx: VQ, mut dy: VQ) {
    il_push_vq(il, dx);
    il_push_vq(il, dy);
    il_push_op(il, OP_RLINETO);
}
unsafe fn il_curveto(
    mut il: *mut CffCharstringIl,
    mut dx1: VQ,
    mut dy1: VQ,
    mut dx2: VQ,
    mut dy2: VQ,
    mut dx3: VQ,
    mut dy3: VQ,
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
    mut il: *mut CffCharstringIl,
    mut masks: *const MaskList,
    mut contours: u16,
    mut points: u16,
    mut nh: u16,
    mut nv: u16,
    mut jm: *mut u16,
    mut op: CffCharstringOperator,
) {
    let masks: &Vec<PostscriptHintMask> = &*masks;
    let mut n: ShapeId = masks.len() as ShapeId;
    while (*jm as ::core::ffi::c_int) < n as ::core::ffi::c_int
        && ((masks[*jm as usize].contours_before as ::core::ffi::c_int)
            < contours as ::core::ffi::c_int
            || masks[*jm as usize].contours_before as ::core::ffi::c_int
                == contours as ::core::ffi::c_int
                && masks[*jm as usize].points_before as ::core::ffi::c_int
                    <= points as ::core::ffi::c_int)
    {
        il_push_op(il, op);
        let mut mask_byte: u8 = 0 as u8;
        let mut bits: u8 = 0 as u8;
        let mut j: u16 = 0 as u16;
        while (j as ::core::ffi::c_int) < nh as ::core::ffi::c_int {
            mask_byte = ((mask_byte as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | masks[*jm as usize].mask_h[j as usize] as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int) as u8;
            bits = (bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
            if bits as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                il_push_special(il, mask_byte as i32);
                bits = 0 as u8;
            }
            j = j.wrapping_add(1);
        }
        let mut j_0: u16 = 0 as u16;
        while (j_0 as ::core::ffi::c_int) < nv as ::core::ffi::c_int {
            mask_byte = ((mask_byte as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | masks[*jm as usize].mask_v[j_0 as usize] as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int) as u8;
            bits = (bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
            if bits as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                il_push_special(il, mask_byte as i32);
                bits = 0 as u8;
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
    mut il: *mut CffCharstringIl,
    mut g: *const Glyph,
    mut contours: u16,
    mut points: u16,
    mut jh: *mut u16,
    mut jm: *mut u16,
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
    mut il: *mut CffCharstringIl,
    mut stems: *const StemDefList,
    mut hasmask: bool,
    mut haswidth: bool,
    mut ophm: CffCharstringOperator,
    mut oph: CffCharstringOperator,
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
    let mut j: u16 = 0 as u16;
    while (j as usize) < stems.len() {
        il_push_operand(
            il,
            stems[j as usize].position as ::core::ffi::c_double
                - ref_0 as ::core::ffi::c_double,
        );
        il_push_operand(
            il,
            stems[j as usize].width as ::core::ffi::c_double,
        );
        ref_0 = stems[j as usize].position
            + stems[j as usize].width;
        nn = nn.wrapping_add(1);
        if nn as u32 >= TYPE2_ARGUMENT_STACK {
            if hasmask {
                il_push_op(il, OP_HSTEMHM);
            } else {
                il_push_op(il, OP_HSTEM);
            }
            (*(*il)
                .instr.as_mut_ptr()
                .offset(((*il).instr.len() as u32).wrapping_sub(1 as u32) as isize))
            .arity = nn as Arity;
            nn = 0 as u16;
        }
        j = j.wrapping_add(1);
    }
    if hasmask {
        il_push_op(il, ophm);
    } else {
        il_push_op(il, oph);
    }
    (*(*il)
        .instr.as_mut_ptr()
        .offset(((*il).instr.len() as u32).wrapping_sub(1 as u32) as isize))
    .arity = nn as Arity;
}
unsafe fn il_push_stems(
    mut il: *mut CffCharstringIl,
    mut g: *const Glyph,
    mut hasmask: bool,
    mut haswidth: bool,
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
    mut g: *const Glyph,
    mut default_width: u16,
    mut nominal_width: u16,
) -> *mut CffCharstringIl {
    let mut il: *mut CffCharstringIl = Box::into_raw(Box::new(CffCharstringIl { instr: Vec::new() }));
    let mut temp_contours: *mut Contour = ::core::ptr::null_mut::<Contour>();
    let mut x: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    let mut y: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    temp_contours = __caryll_allocate_clean(
        (::core::mem::size_of::<Contour>() as usize).wrapping_mul((*g).contours.len()),
        149 as ::core::ffi::c_ulong,
    ) as *mut Contour;
    let mut c: u16 = 0 as u16;
    while (c as usize) < (*g).contours.len() {
        let contour: *const Contour = &(&(*g).contours)[c as usize];
        let newcontour: *mut Contour = temp_contours.offset(c as isize);
        // `__caryll_allocate_clean` is calloc, so this field assignment reads
        // an all-zero (not garbage) `Contour` first -- a `Vec` with capacity
        // 0 never touches its pointer field when dropped, so the drop this
        // assignment performs is a safe no-op regardless of the zero bytes
        // underneath. Same reasoning as `vq_init` on calloc'd memory
        // (rust/README.md).
        *newcontour = Vec::new();
        let mut j: ShapeId = 0 as ShapeId;
        while (j as usize) < (*contour).len() {
            (*newcontour).push(
                glyf_point_dup((&(*contour))[j as usize].clone()),
            );
            j = j.wrapping_add(1);
        }
        if (*newcontour).len() > 2 as usize
            && (&(*newcontour))[(*newcontour).len().wrapping_sub(1 as usize)]
                .on_curve
                == 0
        {
            let first = (&(*newcontour))[0 as usize].clone();
            (*newcontour).push(glyf_point_dup(first));
        }
        let mut j_0: ShapeId = 0 as ShapeId;
        while (j_0 as usize) < (*newcontour).len() {
            let mut dx: VQ = I_VQ.minus.expect("non-null function pointer")(
                (&(*newcontour))[j_0 as usize].x.clone(),
                x.clone(),
            );
            let mut dy: VQ = I_VQ.minus.expect("non-null function pointer")(
                (&(*newcontour))[j_0 as usize].y.clone(),
                y.clone(),
            );
            I_VQ.copy_replace.expect("non-null function pointer")(
                &raw mut x,
                (&(*newcontour))[j_0 as usize].x.clone(),
            );
            I_VQ.copy_replace.expect("non-null function pointer")(
                &raw mut y,
                (&(*newcontour))[j_0 as usize].y.clone(),
            );
            I_VQ.replace.expect("non-null function pointer")(
                &raw mut (&mut (*newcontour))[j_0 as usize].x,
                dx,
            );
            I_VQ.replace.expect("non-null function pointer")(
                &raw mut (&mut (*newcontour))[j_0 as usize].y,
                dy,
            );
            j_0 = j_0.wrapping_add(1);
        }
        c = c.wrapping_add(1);
    }
    // `x`/`y` are plain owned locals, never moved out, so they auto-drop
    // when this function returns -- no explicit dispose call is needed.
    let mut hasmask: bool = !(*g).hint_masks.is_empty() || !(*g).contour_masks.is_empty();
    let glyph_adw_const: Pos =
        I_VQ.get_still.expect("non-null function pointer")((*g).advance_width.clone()) as Pos;
    let mut haswidth: bool = glyph_adw_const != default_width as ::core::ffi::c_int as Pos;
    if haswidth {
        il_push_operand(
            il,
            (glyph_adw_const as ::core::ffi::c_int - nominal_width as ::core::ffi::c_int)
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
        let mut n: ShapeId = (*contour_0).len() as ShapeId;
        if !(n as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
            il_moveto(
                il,
                (&(*contour_0))[0 as usize].x.clone(),
                (&(*contour_0))[0 as usize].y.clone(),
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
                    && (&(*contour_0))[(j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        .on_curve
                        == 0
                    && (&(*contour_0))[(j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                        .on_curve as ::core::ffi::c_int
                        != 0
                {
                    il_curveto(
                        il,
                        (&(*contour_0))[j_1 as usize].x.clone(),
                        (&(*contour_0))[j_1 as usize].y.clone(),
                        (&(*contour_0))[(j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                            .x.clone(),
                        (&(*contour_0))[(j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                            .y.clone(),
                        (&(*contour_0))[(j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                            .x.clone(),
                        (&(*contour_0))[(j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                            .y.clone(),
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
    temp_contours = ::core::ptr::null_mut::<Contour>();
    return il;
}
unsafe fn il_matchtype(
    mut il: *mut CffCharstringIl,
    mut j: u32,
    mut k: u32,
    mut t: CffInstructionType,
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
    mut il: *mut CffCharstringIl,
    mut j: u32,
    mut op: CffCharstringOperator,
) -> bool {
    if (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 != CffInstructionType::Operator
    {
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
    mut il: *mut CffCharstringIl,
    mut j: u32,
    mut op: CffCharstringOperator,
    mut op2: CffCharstringOperator,
    zeros: &[bool],
) -> u8 {
    let mut arity: u8 = cff_get_standard_arity(op);
    if arity as ::core::ffi::c_int > 16 as ::core::ffi::c_int
        || j.wrapping_add(arity as u32) >= (*il).instr.len() as u32
    {
        return 0 as u8;
    }
    if (j == 0 as u32
        || !il_matchtype(
            il,
            j.wrapping_sub(1 as u32),
            j,
            CffInstructionType::PhantomOperator,
        ))
        && il_matchop(il, j.wrapping_add(arity as u32), op) as ::core::ffi::c_int != 0
        && il_matchtype(il, j, j.wrapping_add(arity as u32), CffInstructionType::Operand)
            as ::core::ffi::c_int
            != 0
    {
        let mut check: u8 = TRUE_0 as u8;
        let mut result_arity: u8 = arity;
        let mut mask: [bool; 16] = [false; 16];
        debug_assert_eq!(zeros.len(), arity as usize, "zroll: flag count must match the operator's arity");
        let mut m: u32 = 0 as u32;
        while m < arity as u32 {
            let checkzero: bool = zeros[m as usize];
            mask[m as usize] = checkzero;
            if checkzero {
                result_arity =
                    (result_arity as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u8;
                check = (check as ::core::ffi::c_int != 0
                    && (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(m) as isize)).d()
                        == 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int as u8;
            }
            m = m.wrapping_add(1);
        }
        if check != 0 {
            let mut m_0: u32 = 0 as u32;
            while m_0 < arity as u32 {
                if mask[m_0 as usize] {
                    (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(m_0) as isize)).type_0 =
                        CffInstructionType::PhantomOperand;
                }
                m_0 = m_0.wrapping_add(1);
            }
            (*(*il)
                .instr.as_mut_ptr()
                .offset(j.wrapping_add(arity as u32) as isize))
            .set_i(op2.0);
            (*(*il)
                .instr.as_mut_ptr()
                .offset(j.wrapping_add(arity as u32) as isize))
            .arity = result_arity as Arity;
            return arity;
        } else {
            return 0 as u8;
        }
    } else {
        return 0 as u8;
    };
}
unsafe fn opop_roll(
    mut il: *mut CffCharstringIl,
    mut j: u32,
    mut op1: CffCharstringOperator,
    mut arity: i32,
    mut op2: CffCharstringOperator,
    mut resultop: CffCharstringOperator,
) -> u8 {
    if j.wrapping_add(1 as u32)
        .wrapping_add(arity as u32)
        >= (*il).instr.len() as u32
    {
        return 0 as u8;
    }
    let mut current: *mut CffCharstringInstruction =
        (*il).instr.as_mut_ptr().offset(j as isize) as *mut CffCharstringInstruction;
    let mut nextop: *mut CffCharstringInstruction = (*il).instr.as_mut_ptr().offset(
        j.wrapping_add(1 as u32)
            .wrapping_add(arity as u32) as isize,
    ) as *mut CffCharstringInstruction;
    if il_matchop(il, j, op1) as ::core::ffi::c_int != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as u32),
            j.wrapping_add(1 as u32)
                .wrapping_add(arity as u32),
            CffInstructionType::Operand,
        ) as ::core::ffi::c_int
            != 0
        && il_matchop(
            il,
            j.wrapping_add(1 as u32)
                .wrapping_add(arity as u32),
            op2,
        ) as ::core::ffi::c_int
            != 0
        && (*current).arity.wrapping_add((*nextop).arity)
            <= TYPE2_ARGUMENT_STACK
    {
        (*current).type_0 = CffInstructionType::PhantomOperator;
        (*nextop).set_i(resultop.0);
        (*nextop).arity = (*nextop).arity.wrapping_add((*current).arity);
        return (arity + 1 as i32) as u8;
    } else {
        return 0 as u8;
    };
}
unsafe fn hvlineto_roll(mut il: *mut CffCharstringIl, mut j: u32) -> u8 {
    if j.wrapping_add(3 as u32) >= (*il).instr.len() as u32 {
        return 0 as u8;
    }
    let mut current: *mut CffCharstringInstruction =
        (*il).instr.as_mut_ptr().offset(j as isize) as *mut CffCharstringInstruction;
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
        return 0 as u8;
    }
    let mut checkdelta: u32 = (if ((*current).arity & 1 as Arity != 0) as ::core::ffi::c_int
        ^ ((*current).i() == OP_VLINETO.0)
            as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as u32;
    if il_matchop(
            il,
            j.wrapping_add(3 as u32),
            OP_RLINETO,
        ) as ::core::ffi::c_int
            != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as u32),
            j.wrapping_add(3 as u32),
            CffInstructionType::Operand,
        ) as ::core::ffi::c_int
            != 0
        && (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta) as isize)).d()
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*current).arity.wrapping_add(1 as Arity)
            <= TYPE2_ARGUMENT_STACK
    {
        (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta) as isize)).type_0 = CffInstructionType::PhantomOperand;
        (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 = CffInstructionType::PhantomOperator;
        let current_i = (*current).i();
        (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(3 as u32) as isize)).set_i(current_i);
        (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(3 as u32) as isize)).arity =
            (*current).arity.wrapping_add(1 as Arity);
        return 3 as u8;
    } else {
        return 0 as u8;
    };
}
unsafe fn hvvhcurve_roll(mut il: *mut CffCharstringIl, mut j: u32) -> u8 {
    if !il_matchop(il, j, OP_HVCURVETO)
        && !il_matchop(il, j, OP_VHCURVETO)
    {
        return 0 as u8;
    }
    let mut current: *mut CffCharstringInstruction =
        (*il).instr.as_mut_ptr().offset(j as isize) as *mut CffCharstringInstruction;
    if j.wrapping_add(7 as u32) >= (*il).instr.len() as u32 || (*current).arity & 1 as Arity != 0 {
        return 0 as u8;
    }
    let mut hvcase: bool = ((*current).arity >> 2 as ::core::ffi::c_int & 1 as Arity != 0)
        as ::core::ffi::c_int
        ^ ((*current).i() == OP_HVCURVETO.0)
            as ::core::ffi::c_int
        != 0;
    let mut checkdelta1: u32 = (if hvcase as ::core::ffi::c_int != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as u32;
    let mut checkdelta2: u32 = (if hvcase as ::core::ffi::c_int != 0 {
        5 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as u32;
    if il_matchop(
        il,
        j.wrapping_add(7 as u32),
        OP_RRCURVETO,
    ) as ::core::ffi::c_int
        != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as u32),
            j.wrapping_add(7 as u32),
            CffInstructionType::Operand,
        ) as ::core::ffi::c_int
            != 0
        && (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta1) as isize)).d()
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta2) as isize)).d()
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*current).arity.wrapping_add(4 as Arity)
                <= TYPE2_ARGUMENT_STACK
        {
            (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
                CffInstructionType::PhantomOperand;
            (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta2) as isize)).type_0 =
                CffInstructionType::PhantomOperand;
            (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 = CffInstructionType::PhantomOperator;
            let current_i = (*current).i();
            (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(7 as u32) as isize)).set_i(current_i);
            (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(7 as u32) as isize)).arity =
                (*current).arity.wrapping_add(4 as Arity);
            return 7 as u8;
        } else if (*current).arity.wrapping_add(5 as Arity)
            <= TYPE2_ARGUMENT_STACK
        {
            (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
                CffInstructionType::PhantomOperand;
            (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 = CffInstructionType::PhantomOperator;
            let current_i = (*current).i();
            (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(7 as u32) as isize)).set_i(current_i);
            (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(7 as u32) as isize)).arity =
                (*current).arity.wrapping_add(5 as Arity);
            if hvcase {
                let mut t: ::core::ffi::c_double =
                    (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(5 as u32) as isize)).d();
                let swap_val = (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(6 as u32) as isize)).d();
                (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(5 as u32) as isize)).set_d(swap_val);
                (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(6 as u32) as isize)).set_d(t);
            }
            return 7 as u8;
        } else {
            return 0 as u8;
        }
    } else {
        return 0 as u8;
    };
}
unsafe fn hhvvcurve_roll(mut il: *mut CffCharstringIl, mut j: u32) -> u8 {
    if !il_matchop(il, j, OP_HHCURVETO)
        && !il_matchop(il, j, OP_VVCURVETO)
    {
        return 0 as u8;
    }
    let mut current: *mut CffCharstringInstruction =
        (*il).instr.as_mut_ptr().offset(j as isize) as *mut CffCharstringInstruction;
    if j.wrapping_add(7 as u32) >= (*il).instr.len() as u32 {
        return 0 as u8;
    }
    let mut hh: bool = (*current).i() == OP_HHCURVETO.0;
    let mut checkdelta1: u32 = (if hh as ::core::ffi::c_int != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as u32;
    let mut checkdelta2: u32 = (if hh as ::core::ffi::c_int != 0 {
        6 as ::core::ffi::c_int
    } else {
        5 as ::core::ffi::c_int
    }) as u32;
    if il_matchop(
        il,
        j.wrapping_add(7 as u32),
        OP_RRCURVETO,
    ) as ::core::ffi::c_int
        != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as u32),
            j.wrapping_add(7 as u32),
            CffInstructionType::Operand,
        ) as ::core::ffi::c_int
            != 0
        && (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta1) as isize)).d()
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta2) as isize)).d()
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*current).arity.wrapping_add(4 as Arity)
            <= TYPE2_ARGUMENT_STACK
    {
        (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
            CffInstructionType::PhantomOperand;
        (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(checkdelta2) as isize)).type_0 =
            CffInstructionType::PhantomOperand;
        (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 = CffInstructionType::PhantomOperator;
        let current_i = (*current).i();
        (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(7 as u32) as isize)).set_i(current_i);
        (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(7 as u32) as isize)).arity =
            (*current).arity.wrapping_add(4 as Arity);
        return 7 as u8;
    } else {
        return 0 as u8;
    };
}
unsafe fn nextstop(mut il: *mut CffCharstringIl, mut j: u32) -> u32 {
    let mut delta: u32 = 0 as u32;
    while j.wrapping_add(delta) < (*il).instr.len() as u32
        && (*(*il).instr.as_mut_ptr().offset(j.wrapping_add(delta) as isize)).type_0 == CffInstructionType::Operand
    {
        delta = delta.wrapping_add(1);
    }
    return delta;
}
unsafe fn decide_advance(
    mut il: *mut CffCharstringIl,
    mut j: u32,
    mut _optimize_level: u8,
) -> u8 {
    let mut r: u8 = 0 as u8;
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
    r = zroll(il, j, OP_RRCURVETO, OP_HVCURVETO, &[false, true, false, false, true, false]);
    if r != 0 {
        return r;
    }
    r = zroll(il, j, OP_RRCURVETO, OP_VHCURVETO, &[true, false, false, false, false, true]);
    if r != 0 {
        return r;
    }
    r = zroll(il, j, OP_RRCURVETO, OP_HHCURVETO, &[false, true, false, false, false, true]);
    if r != 0 {
        return r;
    }
    r = zroll(il, j, OP_RRCURVETO, OP_VVCURVETO, &[true, false, false, false, true, false]);
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        OP_RRCURVETO,
        6 as i32,
        OP_RRCURVETO,
        OP_RRCURVETO,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        OP_RRCURVETO,
        2 as i32,
        OP_RLINETO,
        OP_RCURVELINE,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        OP_RLINETO,
        6 as i32,
        OP_RRCURVETO,
        OP_RLINECURVE,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        OP_RLINETO,
        2 as i32,
        OP_RLINETO,
        OP_RLINETO,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        OP_HSTEMHM,
        0 as i32,
        OP_HINTMASK,
        OP_HINTMASK,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        OP_VSTEMHM,
        0 as i32,
        OP_HINTMASK,
        OP_HINTMASK,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        OP_HSTEMHM,
        0 as i32,
        OP_CNTRMASK,
        OP_CNTRMASK,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        OP_VSTEMHM,
        0 as i32,
        OP_CNTRMASK,
        OP_CNTRMASK,
    );
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
    return 1 as u8;
}
pub unsafe fn cff_optimize_il(
    mut il: *mut CffCharstringIl,
    mut options: *const Options,
) {
    if !(*options).cff_roll_char_string {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < (*il).instr.len() as u32 {
        j = j.wrapping_add(
            decide_advance(il, j, (*options).cff_roll_char_string as u8) as u32,
        );
    }
}
pub unsafe fn cff_build_il(mut il: *mut CffCharstringIl) -> *mut Buffer {
    let mut blob: *mut Buffer = bufnew();
    let mut j: u16 = 0 as u16;
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
pub unsafe fn cff_shrink_il(mut il: *mut CffCharstringIl) -> *mut CffCharstringIl {
    let mut out: *mut CffCharstringIl = Box::into_raw(Box::new(CffCharstringIl { instr: Vec::new() }));
    let mut j: u16 = 0 as u16;
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
pub unsafe fn cff_i_lmerge_il(
    mut self_0: *mut CffCharstringIl,
    mut il: *mut CffCharstringIl,
) {
    let mut j: u16 = 0 as u16;
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
    mut z1: *mut CffCharstringInstruction,
    mut z2: *mut CffCharstringInstruction,
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
pub unsafe fn cff_il_equal(
    mut a: *mut CffCharstringIl,
    mut b: *mut CffCharstringIl,
) -> bool {
    if a.is_null() || b.is_null() {
        return false;
    }
    if (*a).instr.len() as u32 != (*b).instr.len() as u32 {
        return false;
    }
    let mut j: u32 = 0 as u32;
    while j < (*a).instr.len() as u32 {
        if !instruction_eq((*a).instr.as_mut_ptr().offset(j as isize), (*b).instr.as_mut_ptr().offset(j as isize)) {
            return false;
        }
        j = j.wrapping_add(1);
    }
    return true;
}
