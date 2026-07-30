#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};



use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{Arity, Pos, ShapeId};

use crate::libcff::{OP_CNTRMASK, OP_ENDCHAR, OP_HHCURVETO, OP_HINTMASK, OP_HLINETO, OP_HMOVETO, OP_HSTEM, OP_HSTEMHM, OP_HVCURVETO, OP_RCURVELINE, OP_RLINECURVE, OP_RLINETO, OP_RMOVETO, OP_RRCURVETO, OP_VHCURVETO, OP_VLINETO, OP_VMOVETO, OP_VSTEM, OP_VSTEMHM, OP_VVCURVETO, TYPE2_ARGUMENT_STACK};
use crate::support::{TRUE_0};
use crate::table::glyf::{Contour, Glyph, MaskList, StemDefList};

use crate::vf::vq::VQ;
use crate::libcff::cff_opmean::{cff_getStandardArity};
use crate::libcff::cff_writer::{cff_mergeCS2Operand, cff_mergeCS2Operator, cff_mergeCS2Special};
use crate::support::buffer::{bufnew};
use crate::table::glyf::{GLYF_I_CONTOUR, GLYF_I_POINT};
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffCharstringInstruction {
    pub type_0: CffInstructionType,
    pub arity: Arity,
    pub c2rust_unnamed: CffCharstringArgument,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CffCharstringArgument {
    pub d: ::core::ffi::c_double,
    pub i: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffCharstringIl {
    pub length: u32,
    pub free: u32,
    pub instr: *mut CffCharstringInstruction,
}
unsafe extern "C" fn ensureThereIsSpace(mut il: *mut CffCharstringIl) {
    if (*il).free != 0 {
        return;
    }
    (*il).free = 0x100 as u32;
    (*il).instr = __caryll_reallocate(
        (*il).instr as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<CffCharstringInstruction>() as usize)
            .wrapping_mul((*il).length.wrapping_add((*il).free) as usize),
        8 as ::core::ffi::c_ulong,
    ) as *mut CffCharstringInstruction;
}
pub unsafe extern "C" fn il_push_operand(
    mut il: *mut CffCharstringIl,
    mut x: ::core::ffi::c_double,
) {
    ensureThereIsSpace(il);
    (*(*il).instr.offset((*il).length as isize)).type_0 = CffInstructionType::Operand;
    (*(*il).instr.offset((*il).length as isize))
        .c2rust_unnamed
        .d = x;
    (*(*il).instr.offset((*il).length as isize)).arity = 0 as Arity;
    (*il).length = (*il).length.wrapping_add(1);
    (*il).free = (*il).free.wrapping_sub(1);
}
pub unsafe extern "C" fn il_push_VQ(mut il: *mut CffCharstringIl, mut x: VQ) {
    il_push_operand(
        il,
        I_VQ.getStill.expect("non-null function pointer")(x) as ::core::ffi::c_double,
    );
}
pub unsafe extern "C" fn il_push_special(mut il: *mut CffCharstringIl, mut s: i32) {
    ensureThereIsSpace(il);
    (*(*il).instr.offset((*il).length as isize)).type_0 = CffInstructionType::Special;
    (*(*il).instr.offset((*il).length as isize))
        .c2rust_unnamed
        .i = s;
    (*(*il).instr.offset((*il).length as isize)).arity = 0 as Arity;
    (*il).length = (*il).length.wrapping_add(1);
    (*il).free = (*il).free.wrapping_sub(1);
}
pub unsafe extern "C" fn il_push_op(mut il: *mut CffCharstringIl, mut op: i32) {
    ensureThereIsSpace(il);
    (*(*il).instr.offset((*il).length as isize)).type_0 = CffInstructionType::Operator;
    (*(*il).instr.offset((*il).length as isize))
        .c2rust_unnamed
        .i = op;
    (*(*il).instr.offset((*il).length as isize)).arity =
        cff_getStandardArity(op as u32) as Arity;
    (*il).length = (*il).length.wrapping_add(1);
    (*il).free = (*il).free.wrapping_sub(1);
}
unsafe extern "C" fn il_moveto(mut il: *mut CffCharstringIl, mut dx: VQ, mut dy: VQ) {
    il_push_VQ(il, dx);
    il_push_VQ(il, dy);
    il_push_op(il, OP_RMOVETO);
}
unsafe extern "C" fn il_lineto(mut il: *mut CffCharstringIl, mut dx: VQ, mut dy: VQ) {
    il_push_VQ(il, dx);
    il_push_VQ(il, dy);
    il_push_op(il, OP_RLINETO);
}
unsafe extern "C" fn il_curveto(
    mut il: *mut CffCharstringIl,
    mut dx1: VQ,
    mut dy1: VQ,
    mut dx2: VQ,
    mut dy2: VQ,
    mut dx3: VQ,
    mut dy3: VQ,
) {
    il_push_VQ(il, dx1);
    il_push_VQ(il, dy1);
    il_push_VQ(il, dx2);
    il_push_VQ(il, dy2);
    il_push_VQ(il, dx3);
    il_push_VQ(il, dy3);
    il_push_op(il, OP_RRCURVETO);
}
unsafe extern "C" fn _il_push_maskgroup(
    mut il: *mut CffCharstringIl,
    mut masks: *mut MaskList,
    mut contours: u16,
    mut points: u16,
    mut nh: u16,
    mut nv: u16,
    mut jm: *mut u16,
    mut op: i32,
) {
    let mut n: ShapeId = (*masks).length as ShapeId;
    while (*jm as ::core::ffi::c_int) < n as ::core::ffi::c_int
        && (((*(*masks).items.offset(*jm as isize)).contoursBefore as ::core::ffi::c_int)
            < contours as ::core::ffi::c_int
            || (*(*masks).items.offset(*jm as isize)).contoursBefore as ::core::ffi::c_int
                == contours as ::core::ffi::c_int
                && (*(*masks).items.offset(*jm as isize)).pointsBefore as ::core::ffi::c_int
                    <= points as ::core::ffi::c_int)
    {
        il_push_op(il, op);
        let mut mask_byte: u8 = 0 as u8;
        let mut bits: u8 = 0 as u8;
        let mut j: u16 = 0 as u16;
        while (j as ::core::ffi::c_int) < nh as ::core::ffi::c_int {
            mask_byte = ((mask_byte as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (*(*masks).items.offset(*jm as isize)).maskH[j as usize] as ::core::ffi::c_int
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
                | (*(*masks).items.offset(*jm as isize)).maskV[j_0 as usize] as ::core::ffi::c_int
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
unsafe extern "C" fn il_push_masks(
    mut il: *mut CffCharstringIl,
    mut g: *mut Glyph,
    mut contours: u16,
    mut points: u16,
    mut jh: *mut u16,
    mut jm: *mut u16,
) {
    if (*g).stemH.length == 0 && (*g).stemV.length == 0 {
        return;
    }
    _il_push_maskgroup(
        il,
        &raw mut (*g).contourMasks,
        contours,
        points,
        (*g).stemH.length as u16,
        (*g).stemV.length as u16,
        jh,
        OP_CNTRMASK,
    );
    _il_push_maskgroup(
        il,
        &raw mut (*g).hintMasks,
        contours,
        points,
        (*g).stemH.length as u16,
        (*g).stemV.length as u16,
        jm,
        OP_HINTMASK,
    );
}
unsafe extern "C" fn _il_push_stemgroup(
    mut il: *mut CffCharstringIl,
    mut stems: *mut StemDefList,
    mut hasmask: bool,
    mut haswidth: bool,
    mut ophm: i32,
    mut oph: i32,
) {
    if stems.is_null() || (*stems).length == 0 {
        return;
    }
    let mut ref_0: Pos = 0 as ::core::ffi::c_int as Pos;
    let mut nn: u16 = (if haswidth as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as u16;
    let mut j: u16 = 0 as u16;
    while (j as usize) < (*stems).length {
        il_push_operand(
            il,
            (*(*stems).items.offset(j as isize)).position as ::core::ffi::c_double
                - ref_0 as ::core::ffi::c_double,
        );
        il_push_operand(
            il,
            (*(*stems).items.offset(j as isize)).width as ::core::ffi::c_double,
        );
        ref_0 = (*(*stems).items.offset(j as isize)).position
            + (*(*stems).items.offset(j as isize)).width;
        nn = nn.wrapping_add(1);
        if nn as u32 >= TYPE2_ARGUMENT_STACK {
            if hasmask {
                il_push_op(il, OP_HSTEMHM);
            } else {
                il_push_op(il, OP_HSTEM);
            }
            (*(*il)
                .instr
                .offset((*il).length.wrapping_sub(1 as u32) as isize))
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
        .instr
        .offset((*il).length.wrapping_sub(1 as u32) as isize))
    .arity = nn as Arity;
}
unsafe extern "C" fn il_push_stems(
    mut il: *mut CffCharstringIl,
    mut g: *mut Glyph,
    mut hasmask: bool,
    mut haswidth: bool,
) {
    _il_push_stemgroup(
        il,
        &raw mut (*g).stemH,
        hasmask,
        haswidth,
        OP_HSTEMHM,
        OP_HSTEM,
    );
    _il_push_stemgroup(
        il,
        &raw mut (*g).stemV,
        hasmask,
        haswidth,
        OP_VSTEMHM,
        OP_VSTEM,
    );
}
pub unsafe extern "C" fn cff_compileGlyphToIL(
    mut g: *mut Glyph,
    mut defaultWidth: u16,
    mut nominal_width: u16,
) -> *mut CffCharstringIl {
    let mut il: *mut CffCharstringIl = ::core::ptr::null_mut::<CffCharstringIl>();
    il = __caryll_allocate_clean(
        ::core::mem::size_of::<CffCharstringIl>() as usize,
        143 as ::core::ffi::c_ulong,
    ) as *mut CffCharstringIl;
    let mut temp_contours: *mut Contour = ::core::ptr::null_mut::<Contour>();
    let mut x: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    let mut y: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    temp_contours = __caryll_allocate_clean(
        (::core::mem::size_of::<Contour>() as usize).wrapping_mul((*g).contours.length),
        149 as ::core::ffi::c_ulong,
    ) as *mut Contour;
    let mut c: u16 = 0 as u16;
    while (c as usize) < (*g).contours.length {
        let mut contour: *mut Contour =
            (*g).contours.items.offset(c as isize) as *mut Contour;
        let mut newcontour: *mut Contour =
            temp_contours.offset(c as isize) as *mut Contour;
        GLYF_I_CONTOUR.init.expect("non-null function pointer")(newcontour);
        let mut j: ShapeId = 0 as ShapeId;
        while (j as usize) < (*contour).length {
            GLYF_I_CONTOUR.push.expect("non-null function pointer")(
                newcontour,
                GLYF_I_POINT.dup.expect("non-null function pointer")(
                    *(*contour).items.offset(j as isize),
                ),
            );
            j = j.wrapping_add(1);
        }
        if (*newcontour).length > 2 as usize
            && (*(*newcontour)
                .items
                .offset((*newcontour).length.wrapping_sub(1 as usize) as isize))
            .onCurve
                == 0
        {
            GLYF_I_CONTOUR.push.expect("non-null function pointer")(
                newcontour,
                GLYF_I_POINT.dup.expect("non-null function pointer")(
                    *(*newcontour).items.offset(0 as ::core::ffi::c_int as isize),
                ),
            );
        }
        let mut j_0: ShapeId = 0 as ShapeId;
        while (j_0 as usize) < (*newcontour).length {
            let mut dx: VQ = I_VQ.minus.expect("non-null function pointer")(
                (*(*newcontour).items.offset(j_0 as isize)).x,
                x,
            );
            let mut dy: VQ = I_VQ.minus.expect("non-null function pointer")(
                (*(*newcontour).items.offset(j_0 as isize)).y,
                y,
            );
            I_VQ.copyReplace.expect("non-null function pointer")(
                &raw mut x,
                (*(*newcontour).items.offset(j_0 as isize)).x,
            );
            I_VQ.copyReplace.expect("non-null function pointer")(
                &raw mut y,
                (*(*newcontour).items.offset(j_0 as isize)).y,
            );
            I_VQ.replace.expect("non-null function pointer")(
                &raw mut (*(*newcontour).items.offset(j_0 as isize)).x,
                dx,
            );
            I_VQ.replace.expect("non-null function pointer")(
                &raw mut (*(*newcontour).items.offset(j_0 as isize)).y,
                dy,
            );
            j_0 = j_0.wrapping_add(1);
        }
        c = c.wrapping_add(1);
    }
    I_VQ.dispose.expect("non-null function pointer")(&raw mut x);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut y);
    let mut hasmask: bool = (*g).hintMasks.length != 0 || (*g).contourMasks.length != 0;
    let glyph_adw_const: Pos =
        I_VQ.getStill.expect("non-null function pointer")((*g).advanceWidth) as Pos;
    let mut haswidth: bool = glyph_adw_const != defaultWidth as ::core::ffi::c_int as Pos;
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
    while (c_0 as usize) < (*g).contours.length {
        let mut contour_0: *mut Contour =
            temp_contours.offset(c_0 as isize) as *mut Contour;
        let mut n: ShapeId = (*contour_0).length as ShapeId;
        if !(n as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
            il_moveto(
                il,
                (*(*contour_0).items.offset(0 as ::core::ffi::c_int as isize)).x,
                (*(*contour_0).items.offset(0 as ::core::ffi::c_int as isize)).y,
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
                if (*(*contour_0).items.offset(j_1 as isize)).onCurve != 0 {
                    il_lineto(
                        il,
                        (*(*contour_0).items.offset(j_1 as isize)).x,
                        (*(*contour_0).items.offset(j_1 as isize)).y,
                    );
                    points_sofar =
                        (points_sofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
                } else if (j_1 as ::core::ffi::c_int)
                    < n as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    && (*(*contour_0)
                        .items
                        .offset((j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .onCurve
                        == 0
                    && (*(*contour_0)
                        .items
                        .offset((j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                    .onCurve as ::core::ffi::c_int
                        != 0
                {
                    il_curveto(
                        il,
                        (*(*contour_0).items.offset(j_1 as isize)).x,
                        (*(*contour_0).items.offset(j_1 as isize)).y,
                        (*(*contour_0).items.offset(
                            (j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .x,
                        (*(*contour_0).items.offset(
                            (j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .y,
                        (*(*contour_0).items.offset(
                            (j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                        ))
                        .x,
                        (*(*contour_0).items.offset(
                            (j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                        ))
                        .y,
                    );
                    points_sofar =
                        (points_sofar as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as ShapeId;
                    j_1 = (j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as ShapeId;
                } else {
                    il_lineto(
                        il,
                        (*(*contour_0).items.offset(j_1 as isize)).x,
                        (*(*contour_0).items.offset(j_1 as isize)).y,
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
    while (c_1 as usize) < (*g).contours.length {
        GLYF_I_CONTOUR.dispose.expect("non-null function pointer")(
            temp_contours.offset(c_1 as isize) as *mut Contour,
        );
        c_1 = c_1.wrapping_add(1);
    }
    free(temp_contours as *mut ::core::ffi::c_void);
    temp_contours = ::core::ptr::null_mut::<Contour>();
    return il;
}
unsafe extern "C" fn il_matchtype(
    mut il: *mut CffCharstringIl,
    mut j: u32,
    mut k: u32,
    mut t: CffInstructionType,
) -> bool {
    if k >= (*il).length {
        return false;
    }
    let mut m: u32 = j;
    while m < k {
        if (*(*il).instr.offset(m as isize)).type_0 as ::core::ffi::c_uint
            != t as ::core::ffi::c_uint
        {
            return false;
        }
        m = m.wrapping_add(1);
    }
    return true;
}
unsafe extern "C" fn il_matchop(
    mut il: *mut CffCharstringIl,
    mut j: u32,
    mut op: i32,
) -> bool {
    if (*(*il).instr.offset(j as isize)).type_0 != CffInstructionType::Operator
    {
        return false;
    }
    if (*(*il).instr.offset(j as isize)).c2rust_unnamed.i != op {
        return false;
    }
    return true;
}
/// Collapse `op` into `op2` when the operands flagged in `zeros` are all zero.
///
/// `zeros` was a vararg list of `arity` ints -- the count implied by
/// `cff_getStandardArity(op)` and trusted, never checked. As a slice the two can
/// be compared, and the flags read as the booleans they always were.
unsafe fn zroll(
    mut il: *mut CffCharstringIl,
    mut j: u32,
    mut op: i32,
    mut op2: i32,
    zeros: &[bool],
) -> u8 {
    let mut arity: u8 = cff_getStandardArity(op as u32);
    if arity as ::core::ffi::c_int > 16 as ::core::ffi::c_int
        || j.wrapping_add(arity as u32) >= (*il).length
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
                    && (*(*il).instr.offset(j.wrapping_add(m) as isize))
                        .c2rust_unnamed
                        .d
                        == 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int as u8;
            }
            m = m.wrapping_add(1);
        }
        if check != 0 {
            let mut m_0: u32 = 0 as u32;
            while m_0 < arity as u32 {
                if mask[m_0 as usize] {
                    (*(*il).instr.offset(j.wrapping_add(m_0) as isize)).type_0 =
                        CffInstructionType::PhantomOperand;
                }
                m_0 = m_0.wrapping_add(1);
            }
            (*(*il)
                .instr
                .offset(j.wrapping_add(arity as u32) as isize))
            .c2rust_unnamed
            .i = op2;
            (*(*il)
                .instr
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
unsafe extern "C" fn opop_roll(
    mut il: *mut CffCharstringIl,
    mut j: u32,
    mut op1: i32,
    mut arity: i32,
    mut op2: i32,
    mut resultop: i32,
) -> u8 {
    if j.wrapping_add(1 as u32)
        .wrapping_add(arity as u32)
        >= (*il).length
    {
        return 0 as u8;
    }
    let mut current: *mut CffCharstringInstruction =
        (*il).instr.offset(j as isize) as *mut CffCharstringInstruction;
    let mut nextop: *mut CffCharstringInstruction = (*il).instr.offset(
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
        (*nextop).c2rust_unnamed.i = resultop;
        (*nextop).arity = (*nextop).arity.wrapping_add((*current).arity);
        return (arity + 1 as i32) as u8;
    } else {
        return 0 as u8;
    };
}
unsafe extern "C" fn hvlineto_roll(mut il: *mut CffCharstringIl, mut j: u32) -> u8 {
    if j.wrapping_add(3 as u32) >= (*il).length {
        return 0 as u8;
    }
    let mut current: *mut CffCharstringInstruction =
        (*il).instr.offset(j as isize) as *mut CffCharstringInstruction;
    let mut checkdelta: u32 = (if ((*current).arity & 1 as Arity != 0) as ::core::ffi::c_int
        ^ ((*current).c2rust_unnamed.i == OP_VLINETO)
            as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as u32;
    if (il_matchop(il, j, OP_HLINETO) as ::core::ffi::c_int != 0
        || il_matchop(il, j, OP_VLINETO) as ::core::ffi::c_int
            != 0)
        && il_matchop(
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
        && (*(*il).instr.offset(j.wrapping_add(checkdelta) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*current).arity.wrapping_add(1 as Arity)
            <= TYPE2_ARGUMENT_STACK
    {
        (*(*il).instr.offset(j.wrapping_add(checkdelta) as isize)).type_0 = CffInstructionType::PhantomOperand;
        (*(*il).instr.offset(j as isize)).type_0 = CffInstructionType::PhantomOperator;
        (*(*il).instr.offset(j.wrapping_add(3 as u32) as isize))
            .c2rust_unnamed
            .i = (*current).c2rust_unnamed.i;
        (*(*il).instr.offset(j.wrapping_add(3 as u32) as isize)).arity =
            (*current).arity.wrapping_add(1 as Arity);
        return 3 as u8;
    } else {
        return 0 as u8;
    };
}
unsafe extern "C" fn hvvhcurve_roll(mut il: *mut CffCharstringIl, mut j: u32) -> u8 {
    if !il_matchop(il, j, OP_HVCURVETO)
        && !il_matchop(il, j, OP_VHCURVETO)
    {
        return 0 as u8;
    }
    let mut current: *mut CffCharstringInstruction =
        (*il).instr.offset(j as isize) as *mut CffCharstringInstruction;
    if j.wrapping_add(7 as u32) >= (*il).length || (*current).arity & 1 as Arity != 0 {
        return 0 as u8;
    }
    let mut hvcase: bool = ((*current).arity >> 2 as ::core::ffi::c_int & 1 as Arity != 0)
        as ::core::ffi::c_int
        ^ ((*current).c2rust_unnamed.i == OP_HVCURVETO)
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
        && (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if (*(*il).instr.offset(j.wrapping_add(checkdelta2) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*current).arity.wrapping_add(4 as Arity)
                <= TYPE2_ARGUMENT_STACK
        {
            (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
                CffInstructionType::PhantomOperand;
            (*(*il).instr.offset(j.wrapping_add(checkdelta2) as isize)).type_0 =
                CffInstructionType::PhantomOperand;
            (*(*il).instr.offset(j as isize)).type_0 = CffInstructionType::PhantomOperator;
            (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize))
                .c2rust_unnamed
                .i = (*current).c2rust_unnamed.i;
            (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize)).arity =
                (*current).arity.wrapping_add(4 as Arity);
            return 7 as u8;
        } else if (*current).arity.wrapping_add(5 as Arity)
            <= TYPE2_ARGUMENT_STACK
        {
            (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
                CffInstructionType::PhantomOperand;
            (*(*il).instr.offset(j as isize)).type_0 = CffInstructionType::PhantomOperator;
            (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize))
                .c2rust_unnamed
                .i = (*current).c2rust_unnamed.i;
            (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize)).arity =
                (*current).arity.wrapping_add(5 as Arity);
            if hvcase {
                let mut t: ::core::ffi::c_double =
                    (*(*il).instr.offset(j.wrapping_add(5 as u32) as isize))
                        .c2rust_unnamed
                        .d;
                (*(*il).instr.offset(j.wrapping_add(5 as u32) as isize))
                    .c2rust_unnamed
                    .d = (*(*il).instr.offset(j.wrapping_add(6 as u32) as isize))
                    .c2rust_unnamed
                    .d;
                (*(*il).instr.offset(j.wrapping_add(6 as u32) as isize))
                    .c2rust_unnamed
                    .d = t;
            }
            return 7 as u8;
        } else {
            return 0 as u8;
        }
    } else {
        return 0 as u8;
    };
}
unsafe extern "C" fn hhvvcurve_roll(mut il: *mut CffCharstringIl, mut j: u32) -> u8 {
    if !il_matchop(il, j, OP_HHCURVETO)
        && !il_matchop(il, j, OP_VVCURVETO)
    {
        return 0 as u8;
    }
    let mut current: *mut CffCharstringInstruction =
        (*il).instr.offset(j as isize) as *mut CffCharstringInstruction;
    if j.wrapping_add(7 as u32) >= (*il).length {
        return 0 as u8;
    }
    let mut hh: bool = (*current).c2rust_unnamed.i == OP_HHCURVETO;
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
        && (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*(*il).instr.offset(j.wrapping_add(checkdelta2) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*current).arity.wrapping_add(4 as Arity)
            <= TYPE2_ARGUMENT_STACK
    {
        (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
            CffInstructionType::PhantomOperand;
        (*(*il).instr.offset(j.wrapping_add(checkdelta2) as isize)).type_0 =
            CffInstructionType::PhantomOperand;
        (*(*il).instr.offset(j as isize)).type_0 = CffInstructionType::PhantomOperator;
        (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize))
            .c2rust_unnamed
            .i = (*current).c2rust_unnamed.i;
        (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize)).arity =
            (*current).arity.wrapping_add(4 as Arity);
        return 7 as u8;
    } else {
        return 0 as u8;
    };
}
unsafe extern "C" fn nextstop(mut il: *mut CffCharstringIl, mut j: u32) -> u32 {
    let mut delta: u32 = 0 as u32;
    while j.wrapping_add(delta) < (*il).length
        && (*(*il).instr.offset(j.wrapping_add(delta) as isize)).type_0 == CffInstructionType::Operand
    {
        delta = delta.wrapping_add(1);
    }
    return delta;
}
unsafe extern "C" fn decideAdvance(
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
pub unsafe extern "C" fn cff_optimizeIL(
    mut il: *mut CffCharstringIl,
    mut options: *const Options,
) {
    if !(*options).cff_rollCharString {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < (*il).length {
        j = j.wrapping_add(
            decideAdvance(il, j, (*options).cff_rollCharString as u8) as u32,
        );
    }
}
pub unsafe extern "C" fn cff_build_IL(mut il: *mut CffCharstringIl) -> *mut Buffer {
    let mut blob: *mut Buffer = bufnew();
    let mut j: u16 = 0 as u16;
    while (j as u32) < (*il).length {
        match (*(*il).instr.offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                cff_mergeCS2Operand(blob, (*(*il).instr.offset(j as isize)).c2rust_unnamed.d);
            }
            1 => {
                cff_mergeCS2Operator(blob, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
            }
            2 => {
                cff_mergeCS2Special(
                    blob,
                    (*(*il).instr.offset(j as isize)).c2rust_unnamed.i as u8,
                );
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return blob;
}
pub unsafe extern "C" fn cff_shrinkIL(mut il: *mut CffCharstringIl) -> *mut CffCharstringIl {
    let mut out: *mut CffCharstringIl = ::core::ptr::null_mut::<CffCharstringIl>();
    out = __caryll_allocate_clean(
        ::core::mem::size_of::<CffCharstringIl>() as usize,
        457 as ::core::ffi::c_ulong,
    ) as *mut CffCharstringIl;
    let mut j: u16 = 0 as u16;
    while (j as u32) < (*il).length {
        match (*(*il).instr.offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                il_push_operand(out, (*(*il).instr.offset(j as isize)).c2rust_unnamed.d);
            }
            1 => {
                il_push_op(out, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
            }
            2 => {
                il_push_special(out, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return out;
}
pub unsafe extern "C" fn cff_ILmergeIL(
    mut self_0: *mut CffCharstringIl,
    mut il: *mut CffCharstringIl,
) {
    let mut j: u16 = 0 as u16;
    while (j as u32) < (*il).length {
        match (*(*il).instr.offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                il_push_operand(self_0, (*(*il).instr.offset(j as isize)).c2rust_unnamed.d);
            }
            1 => {
                il_push_op(self_0, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
            }
            2 => {
                il_push_special(self_0, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe extern "C" fn instruction_eq(
    mut z1: *mut CffCharstringInstruction,
    mut z2: *mut CffCharstringInstruction,
) -> bool {
    if (*z1).type_0 as ::core::ffi::c_uint == (*z2).type_0 as ::core::ffi::c_uint {
        if (*z1).type_0 == CffInstructionType::Operand
            || (*z1).type_0 == CffInstructionType::PhantomOperand
        {
            return (*z1).c2rust_unnamed.d == (*z2).c2rust_unnamed.d;
        } else {
            return (*z1).c2rust_unnamed.i == (*z2).c2rust_unnamed.i;
        }
    } else {
        return false;
    };
}
pub unsafe extern "C" fn cff_ilEqual(
    mut a: *mut CffCharstringIl,
    mut b: *mut CffCharstringIl,
) -> bool {
    if a.is_null() || b.is_null() {
        return false;
    }
    if (*a).length != (*b).length {
        return false;
    }
    let mut j: u32 = 0 as u32;
    while j < (*a).length {
        if !instruction_eq((*a).instr.offset(j as isize), (*b).instr.offset(j as isize)) {
            return false;
        }
        j = j.wrapping_add(1);
    }
    return true;
}
