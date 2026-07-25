use libc::{free};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
    static iVQ: __caryll_vectorinterface_VQ;
    fn cff_getStandardArity(op: u32) -> u8;
    fn cff_mergeCS2Operator(blob: *mut caryll_Buffer, val: i32);
    fn cff_mergeCS2Operand(blob: *mut caryll_Buffer, val: ::core::ffi::c_double);
    fn cff_mergeCS2Special(blob: *mut caryll_Buffer, val: u8);
    static glyf_iPoint: __caryll_elementinterface_glyf_Point;
    static glyf_iContour: __caryll_vectorinterface_glyf_Contour;
}



use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{arity_t, pos_t, shapeid_t};

use crate::libcff::{op_cntrmask, op_endchar, op_hhcurveto, op_hintmask, op_hlineto, op_hmoveto, op_hstem, op_hstemhm, op_hvcurveto, op_rcurveline, op_rlinecurve, op_rlineto, op_rmoveto, op_rrcurveto, op_vhcurveto, op_vlineto, op_vmoveto, op_vstem, op_vstemhm, op_vvcurveto, type2_argument_stack};
use crate::support::{true_0};
use crate::table::glyf::{__caryll_elementinterface_glyf_Point, __caryll_vectorinterface_glyf_Contour, glyf_Contour, glyf_Glyph, glyf_MaskList, glyf_StemDefList};

use crate::vf::vq::{VQ, __caryll_vectorinterface_VQ};
pub type cff_InstructionType = ::core::ffi::c_uint;
pub const IL_ITEM_PHANTOM_OPERAND: cff_InstructionType = 4;
pub const IL_ITEM_PHANTOM_OPERATOR: cff_InstructionType = 3;
pub const IL_ITEM_SPECIAL: cff_InstructionType = 2;
pub const IL_ITEM_OPERATOR: cff_InstructionType = 1;
pub const IL_ITEM_OPERAND: cff_InstructionType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharstringInstruction {
    pub type_0: cff_InstructionType,
    pub arity: arity_t,
    pub c2rust_unnamed: cff_CharstringArgument,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cff_CharstringArgument {
    pub d: ::core::ffi::c_double,
    pub i: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharstringIL {
    pub length: u32,
    pub free: u32,
    pub instr: *mut cff_CharstringInstruction,
}
unsafe extern "C" fn ensureThereIsSpace(mut il: *mut cff_CharstringIL) {
    if (*il).free != 0 {
        return;
    }
    (*il).free = 0x100 as u32;
    (*il).instr = __caryll_reallocate(
        (*il).instr as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<cff_CharstringInstruction>() as usize)
            .wrapping_mul((*il).length.wrapping_add((*il).free) as usize),
        8 as ::core::ffi::c_ulong,
    ) as *mut cff_CharstringInstruction;
}
#[no_mangle]
pub unsafe extern "C" fn il_push_operand(
    mut il: *mut cff_CharstringIL,
    mut x: ::core::ffi::c_double,
) {
    ensureThereIsSpace(il);
    (*(*il).instr.offset((*il).length as isize)).type_0 = IL_ITEM_OPERAND;
    (*(*il).instr.offset((*il).length as isize))
        .c2rust_unnamed
        .d = x;
    (*(*il).instr.offset((*il).length as isize)).arity = 0 as arity_t;
    (*il).length = (*il).length.wrapping_add(1);
    (*il).free = (*il).free.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn il_push_VQ(mut il: *mut cff_CharstringIL, mut x: VQ) {
    il_push_operand(
        il,
        iVQ.getStill.expect("non-null function pointer")(x) as ::core::ffi::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn il_push_special(mut il: *mut cff_CharstringIL, mut s: i32) {
    ensureThereIsSpace(il);
    (*(*il).instr.offset((*il).length as isize)).type_0 = IL_ITEM_SPECIAL;
    (*(*il).instr.offset((*il).length as isize))
        .c2rust_unnamed
        .i = s;
    (*(*il).instr.offset((*il).length as isize)).arity = 0 as arity_t;
    (*il).length = (*il).length.wrapping_add(1);
    (*il).free = (*il).free.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn il_push_op(mut il: *mut cff_CharstringIL, mut op: i32) {
    ensureThereIsSpace(il);
    (*(*il).instr.offset((*il).length as isize)).type_0 = IL_ITEM_OPERATOR;
    (*(*il).instr.offset((*il).length as isize))
        .c2rust_unnamed
        .i = op;
    (*(*il).instr.offset((*il).length as isize)).arity =
        cff_getStandardArity(op as u32) as arity_t;
    (*il).length = (*il).length.wrapping_add(1);
    (*il).free = (*il).free.wrapping_sub(1);
}
unsafe extern "C" fn il_moveto(mut il: *mut cff_CharstringIL, mut dx: VQ, mut dy: VQ) {
    il_push_VQ(il, dx);
    il_push_VQ(il, dy);
    il_push_op(il, op_rmoveto as ::core::ffi::c_int as i32);
}
unsafe extern "C" fn il_lineto(mut il: *mut cff_CharstringIL, mut dx: VQ, mut dy: VQ) {
    il_push_VQ(il, dx);
    il_push_VQ(il, dy);
    il_push_op(il, op_rlineto as ::core::ffi::c_int as i32);
}
unsafe extern "C" fn il_curveto(
    mut il: *mut cff_CharstringIL,
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
    il_push_op(il, op_rrcurveto as ::core::ffi::c_int as i32);
}
unsafe extern "C" fn _il_push_maskgroup(
    mut il: *mut cff_CharstringIL,
    mut masks: *mut glyf_MaskList,
    mut contours: u16,
    mut points: u16,
    mut nh: u16,
    mut nv: u16,
    mut jm: *mut u16,
    mut op: i32,
) {
    let mut n: shapeid_t = (*masks).length as shapeid_t;
    while (*jm as ::core::ffi::c_int) < n as ::core::ffi::c_int
        && (((*(*masks).items.offset(*jm as isize)).contoursBefore as ::core::ffi::c_int)
            < contours as ::core::ffi::c_int
            || (*(*masks).items.offset(*jm as isize)).contoursBefore as ::core::ffi::c_int
                == contours as ::core::ffi::c_int
                && (*(*masks).items.offset(*jm as isize)).pointsBefore as ::core::ffi::c_int
                    <= points as ::core::ffi::c_int)
    {
        il_push_op(il, op);
        let mut maskByte: u8 = 0 as u8;
        let mut bits: u8 = 0 as u8;
        let mut j: u16 = 0 as u16;
        while (j as ::core::ffi::c_int) < nh as ::core::ffi::c_int {
            maskByte = ((maskByte as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (*(*masks).items.offset(*jm as isize)).maskH[j as usize] as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int) as u8;
            bits = (bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
            if bits as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                il_push_special(il, maskByte as i32);
                bits = 0 as u8;
            }
            j = j.wrapping_add(1);
        }
        let mut j_0: u16 = 0 as u16;
        while (j_0 as ::core::ffi::c_int) < nv as ::core::ffi::c_int {
            maskByte = ((maskByte as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (*(*masks).items.offset(*jm as isize)).maskV[j_0 as usize] as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int) as u8;
            bits = (bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
            if bits as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                il_push_special(il, maskByte as i32);
                bits = 0 as u8;
            }
            j_0 = j_0.wrapping_add(1);
        }
        if bits != 0 {
            maskByte = ((maskByte as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - bits as ::core::ffi::c_int)
                as u8;
            il_push_special(il, maskByte as i32);
        }
        *jm = (*jm as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
    }
}
unsafe extern "C" fn il_push_masks(
    mut il: *mut cff_CharstringIL,
    mut g: *mut glyf_Glyph,
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
        op_cntrmask as ::core::ffi::c_int as i32,
    );
    _il_push_maskgroup(
        il,
        &raw mut (*g).hintMasks,
        contours,
        points,
        (*g).stemH.length as u16,
        (*g).stemV.length as u16,
        jm,
        op_hintmask as ::core::ffi::c_int as i32,
    );
}
unsafe extern "C" fn _il_push_stemgroup(
    mut il: *mut cff_CharstringIL,
    mut stems: *mut glyf_StemDefList,
    mut hasmask: bool,
    mut haswidth: bool,
    mut ophm: i32,
    mut oph: i32,
) {
    if stems.is_null() || (*stems).length == 0 {
        return;
    }
    let mut ref_0: pos_t = 0 as ::core::ffi::c_int as pos_t;
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
        if nn as ::core::ffi::c_int >= type2_argument_stack as ::core::ffi::c_int {
            if hasmask {
                il_push_op(il, op_hstemhm as ::core::ffi::c_int as i32);
            } else {
                il_push_op(il, op_hstem as ::core::ffi::c_int as i32);
            }
            (*(*il)
                .instr
                .offset((*il).length.wrapping_sub(1 as u32) as isize))
            .arity = nn as arity_t;
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
    .arity = nn as arity_t;
}
unsafe extern "C" fn il_push_stems(
    mut il: *mut cff_CharstringIL,
    mut g: *mut glyf_Glyph,
    mut hasmask: bool,
    mut haswidth: bool,
) {
    _il_push_stemgroup(
        il,
        &raw mut (*g).stemH,
        hasmask,
        haswidth,
        op_hstemhm as ::core::ffi::c_int as i32,
        op_hstem as ::core::ffi::c_int as i32,
    );
    _il_push_stemgroup(
        il,
        &raw mut (*g).stemV,
        hasmask,
        haswidth,
        op_vstemhm as ::core::ffi::c_int as i32,
        op_vstem as ::core::ffi::c_int as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cff_compileGlyphToIL(
    mut g: *mut glyf_Glyph,
    mut defaultWidth: u16,
    mut nominalWidth: u16,
) -> *mut cff_CharstringIL {
    let mut il: *mut cff_CharstringIL = ::core::ptr::null_mut::<cff_CharstringIL>();
    il = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_CharstringIL>() as usize,
        143 as ::core::ffi::c_ulong,
    ) as *mut cff_CharstringIL;
    let mut tempContours: *mut glyf_Contour = ::core::ptr::null_mut::<glyf_Contour>();
    let mut x: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut y: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    tempContours = __caryll_allocate_clean(
        (::core::mem::size_of::<glyf_Contour>() as usize).wrapping_mul((*g).contours.length),
        149 as ::core::ffi::c_ulong,
    ) as *mut glyf_Contour;
    let mut c: u16 = 0 as u16;
    while (c as usize) < (*g).contours.length {
        let mut contour: *mut glyf_Contour =
            (*g).contours.items.offset(c as isize) as *mut glyf_Contour;
        let mut newcontour: *mut glyf_Contour =
            tempContours.offset(c as isize) as *mut glyf_Contour;
        glyf_iContour.init.expect("non-null function pointer")(newcontour);
        let mut j: shapeid_t = 0 as shapeid_t;
        while (j as usize) < (*contour).length {
            glyf_iContour.push.expect("non-null function pointer")(
                newcontour,
                glyf_iPoint.dup.expect("non-null function pointer")(
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
            glyf_iContour.push.expect("non-null function pointer")(
                newcontour,
                glyf_iPoint.dup.expect("non-null function pointer")(
                    *(*newcontour).items.offset(0 as ::core::ffi::c_int as isize),
                ),
            );
        }
        let mut j_0: shapeid_t = 0 as shapeid_t;
        while (j_0 as usize) < (*newcontour).length {
            let mut dx: VQ = iVQ.minus.expect("non-null function pointer")(
                (*(*newcontour).items.offset(j_0 as isize)).x,
                x,
            );
            let mut dy: VQ = iVQ.minus.expect("non-null function pointer")(
                (*(*newcontour).items.offset(j_0 as isize)).y,
                y,
            );
            iVQ.copyReplace.expect("non-null function pointer")(
                &raw mut x,
                (*(*newcontour).items.offset(j_0 as isize)).x,
            );
            iVQ.copyReplace.expect("non-null function pointer")(
                &raw mut y,
                (*(*newcontour).items.offset(j_0 as isize)).y,
            );
            iVQ.replace.expect("non-null function pointer")(
                &raw mut (*(*newcontour).items.offset(j_0 as isize)).x,
                dx,
            );
            iVQ.replace.expect("non-null function pointer")(
                &raw mut (*(*newcontour).items.offset(j_0 as isize)).y,
                dy,
            );
            j_0 = j_0.wrapping_add(1);
        }
        c = c.wrapping_add(1);
    }
    iVQ.dispose.expect("non-null function pointer")(&raw mut x);
    iVQ.dispose.expect("non-null function pointer")(&raw mut y);
    let mut hasmask: bool = (*g).hintMasks.length != 0 || (*g).contourMasks.length != 0;
    let glyphADWConst: pos_t =
        iVQ.getStill.expect("non-null function pointer")((*g).advanceWidth) as pos_t;
    let mut haswidth: bool = glyphADWConst != defaultWidth as ::core::ffi::c_int as pos_t;
    if haswidth {
        il_push_operand(
            il,
            (glyphADWConst as ::core::ffi::c_int - nominalWidth as ::core::ffi::c_int)
                as ::core::ffi::c_double,
        );
    }
    il_push_stems(il, g, hasmask, haswidth);
    let mut contoursSofar: shapeid_t = 0 as shapeid_t;
    let mut pointsSofar: shapeid_t = 0 as shapeid_t;
    let mut jh: shapeid_t = 0 as shapeid_t;
    let mut jm: shapeid_t = 0 as shapeid_t;
    if hasmask {
        il_push_masks(
            il,
            g,
            contoursSofar as u16,
            pointsSofar as u16,
            &raw mut jh,
            &raw mut jm,
        );
    }
    let mut c_0: shapeid_t = 0 as shapeid_t;
    while (c_0 as usize) < (*g).contours.length {
        let mut contour_0: *mut glyf_Contour =
            tempContours.offset(c_0 as isize) as *mut glyf_Contour;
        let mut n: shapeid_t = (*contour_0).length as shapeid_t;
        if !(n as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
            il_moveto(
                il,
                (*(*contour_0).items.offset(0 as ::core::ffi::c_int as isize)).x,
                (*(*contour_0).items.offset(0 as ::core::ffi::c_int as isize)).y,
            );
            pointsSofar = pointsSofar.wrapping_add(1);
            if hasmask {
                il_push_masks(
                    il,
                    g,
                    contoursSofar as u16,
                    pointsSofar as u16,
                    &raw mut jh,
                    &raw mut jm,
                );
            }
            let mut j_1: shapeid_t = 1 as shapeid_t;
            while (j_1 as ::core::ffi::c_int) < n as ::core::ffi::c_int {
                if (*(*contour_0).items.offset(j_1 as isize)).onCurve != 0 {
                    il_lineto(
                        il,
                        (*(*contour_0).items.offset(j_1 as isize)).x,
                        (*(*contour_0).items.offset(j_1 as isize)).y,
                    );
                    pointsSofar =
                        (pointsSofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
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
                    pointsSofar =
                        (pointsSofar as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as shapeid_t;
                    j_1 = (j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as shapeid_t;
                } else {
                    il_lineto(
                        il,
                        (*(*contour_0).items.offset(j_1 as isize)).x,
                        (*(*contour_0).items.offset(j_1 as isize)).y,
                    );
                    pointsSofar = pointsSofar.wrapping_add(1);
                }
                if hasmask {
                    il_push_masks(
                        il,
                        g,
                        contoursSofar as u16,
                        pointsSofar as u16,
                        &raw mut jh,
                        &raw mut jm,
                    );
                }
                j_1 = j_1.wrapping_add(1);
            }
            contoursSofar =
                (contoursSofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
            pointsSofar = 0 as shapeid_t;
        }
        c_0 = c_0.wrapping_add(1);
    }
    il_push_op(il, op_endchar as ::core::ffi::c_int as i32);
    let mut c_1: shapeid_t = 0 as shapeid_t;
    while (c_1 as usize) < (*g).contours.length {
        glyf_iContour.dispose.expect("non-null function pointer")(
            tempContours.offset(c_1 as isize) as *mut glyf_Contour,
        );
        c_1 = c_1.wrapping_add(1);
    }
    free(tempContours as *mut ::core::ffi::c_void);
    tempContours = ::core::ptr::null_mut::<glyf_Contour>();
    return il;
}
unsafe extern "C" fn il_matchtype(
    mut il: *mut cff_CharstringIL,
    mut j: u32,
    mut k: u32,
    mut t: cff_InstructionType,
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
    mut il: *mut cff_CharstringIL,
    mut j: u32,
    mut op: i32,
) -> bool {
    if (*(*il).instr.offset(j as isize)).type_0 as ::core::ffi::c_uint
        != IL_ITEM_OPERATOR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false;
    }
    if (*(*il).instr.offset(j as isize)).c2rust_unnamed.i != op {
        return false;
    }
    return true;
}
unsafe extern "C" fn zroll(
    mut il: *mut cff_CharstringIL,
    mut j: u32,
    mut op: i32,
    mut op2: i32,
    mut args: ...
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
            IL_ITEM_PHANTOM_OPERATOR,
        ))
        && il_matchop(il, j.wrapping_add(arity as u32), op) as ::core::ffi::c_int != 0
        && il_matchtype(il, j, j.wrapping_add(arity as u32), IL_ITEM_OPERAND)
            as ::core::ffi::c_int
            != 0
    {
        let mut ap: ::core::ffi::VaListImpl;
        let mut check: u8 = true_0 as u8;
        let mut resultArity: u8 = arity;
        let mut mask: [bool; 16] = [false; 16];
        ap = args.clone();
        let mut m: u32 = 0 as u32;
        while m < arity as u32 {
            let mut checkzero: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
            mask[m as usize] = checkzero != 0;
            if checkzero != 0 {
                resultArity =
                    (resultArity as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u8;
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
                        IL_ITEM_PHANTOM_OPERAND;
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
            .arity = resultArity as arity_t;
            return arity;
        } else {
            return 0 as u8;
        }
    } else {
        return 0 as u8;
    };
}
unsafe extern "C" fn opop_roll(
    mut il: *mut cff_CharstringIL,
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
    let mut current: *mut cff_CharstringInstruction =
        (*il).instr.offset(j as isize) as *mut cff_CharstringInstruction;
    let mut nextop: *mut cff_CharstringInstruction = (*il).instr.offset(
        j.wrapping_add(1 as u32)
            .wrapping_add(arity as u32) as isize,
    ) as *mut cff_CharstringInstruction;
    if il_matchop(il, j, op1) as ::core::ffi::c_int != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as u32),
            j.wrapping_add(1 as u32)
                .wrapping_add(arity as u32),
            IL_ITEM_OPERAND,
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
            <= type2_argument_stack as ::core::ffi::c_int as arity_t
    {
        (*current).type_0 = IL_ITEM_PHANTOM_OPERATOR;
        (*nextop).c2rust_unnamed.i = resultop;
        (*nextop).arity = (*nextop).arity.wrapping_add((*current).arity);
        return (arity + 1 as i32) as u8;
    } else {
        return 0 as u8;
    };
}
unsafe extern "C" fn hvlineto_roll(mut il: *mut cff_CharstringIL, mut j: u32) -> u8 {
    if j.wrapping_add(3 as u32) >= (*il).length {
        return 0 as u8;
    }
    let mut current: *mut cff_CharstringInstruction =
        (*il).instr.offset(j as isize) as *mut cff_CharstringInstruction;
    let mut checkdelta: u32 = (if ((*current).arity & 1 as arity_t != 0) as ::core::ffi::c_int
        ^ ((*current).c2rust_unnamed.i == op_vlineto as ::core::ffi::c_int as i32)
            as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as u32;
    if (il_matchop(il, j, op_hlineto as ::core::ffi::c_int as i32) as ::core::ffi::c_int != 0
        || il_matchop(il, j, op_vlineto as ::core::ffi::c_int as i32) as ::core::ffi::c_int
            != 0)
        && il_matchop(
            il,
            j.wrapping_add(3 as u32),
            op_rlineto as ::core::ffi::c_int as i32,
        ) as ::core::ffi::c_int
            != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as u32),
            j.wrapping_add(3 as u32),
            IL_ITEM_OPERAND,
        ) as ::core::ffi::c_int
            != 0
        && (*(*il).instr.offset(j.wrapping_add(checkdelta) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*current).arity.wrapping_add(1 as arity_t)
            <= type2_argument_stack as ::core::ffi::c_int as arity_t
    {
        (*(*il).instr.offset(j.wrapping_add(checkdelta) as isize)).type_0 = IL_ITEM_PHANTOM_OPERAND;
        (*(*il).instr.offset(j as isize)).type_0 = IL_ITEM_PHANTOM_OPERATOR;
        (*(*il).instr.offset(j.wrapping_add(3 as u32) as isize))
            .c2rust_unnamed
            .i = (*current).c2rust_unnamed.i;
        (*(*il).instr.offset(j.wrapping_add(3 as u32) as isize)).arity =
            (*current).arity.wrapping_add(1 as arity_t);
        return 3 as u8;
    } else {
        return 0 as u8;
    };
}
unsafe extern "C" fn hvvhcurve_roll(mut il: *mut cff_CharstringIL, mut j: u32) -> u8 {
    if !il_matchop(il, j, op_hvcurveto as ::core::ffi::c_int as i32)
        && !il_matchop(il, j, op_vhcurveto as ::core::ffi::c_int as i32)
    {
        return 0 as u8;
    }
    let mut current: *mut cff_CharstringInstruction =
        (*il).instr.offset(j as isize) as *mut cff_CharstringInstruction;
    if j.wrapping_add(7 as u32) >= (*il).length || (*current).arity & 1 as arity_t != 0 {
        return 0 as u8;
    }
    let mut hvcase: bool = ((*current).arity >> 2 as ::core::ffi::c_int & 1 as arity_t != 0)
        as ::core::ffi::c_int
        ^ ((*current).c2rust_unnamed.i == op_hvcurveto as ::core::ffi::c_int as i32)
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
        op_rrcurveto as ::core::ffi::c_int as i32,
    ) as ::core::ffi::c_int
        != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as u32),
            j.wrapping_add(7 as u32),
            IL_ITEM_OPERAND,
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
            && (*current).arity.wrapping_add(4 as arity_t)
                <= type2_argument_stack as ::core::ffi::c_int as arity_t
        {
            (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
                IL_ITEM_PHANTOM_OPERAND;
            (*(*il).instr.offset(j.wrapping_add(checkdelta2) as isize)).type_0 =
                IL_ITEM_PHANTOM_OPERAND;
            (*(*il).instr.offset(j as isize)).type_0 = IL_ITEM_PHANTOM_OPERATOR;
            (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize))
                .c2rust_unnamed
                .i = (*current).c2rust_unnamed.i;
            (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize)).arity =
                (*current).arity.wrapping_add(4 as arity_t);
            return 7 as u8;
        } else if (*current).arity.wrapping_add(5 as arity_t)
            <= type2_argument_stack as ::core::ffi::c_int as arity_t
        {
            (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
                IL_ITEM_PHANTOM_OPERAND;
            (*(*il).instr.offset(j as isize)).type_0 = IL_ITEM_PHANTOM_OPERATOR;
            (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize))
                .c2rust_unnamed
                .i = (*current).c2rust_unnamed.i;
            (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize)).arity =
                (*current).arity.wrapping_add(5 as arity_t);
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
unsafe extern "C" fn hhvvcurve_roll(mut il: *mut cff_CharstringIL, mut j: u32) -> u8 {
    if !il_matchop(il, j, op_hhcurveto as ::core::ffi::c_int as i32)
        && !il_matchop(il, j, op_vvcurveto as ::core::ffi::c_int as i32)
    {
        return 0 as u8;
    }
    let mut current: *mut cff_CharstringInstruction =
        (*il).instr.offset(j as isize) as *mut cff_CharstringInstruction;
    if j.wrapping_add(7 as u32) >= (*il).length {
        return 0 as u8;
    }
    let mut hh: bool = (*current).c2rust_unnamed.i == op_hhcurveto as ::core::ffi::c_int as i32;
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
        op_rrcurveto as ::core::ffi::c_int as i32,
    ) as ::core::ffi::c_int
        != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as u32),
            j.wrapping_add(7 as u32),
            IL_ITEM_OPERAND,
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
        && (*current).arity.wrapping_add(4 as arity_t)
            <= type2_argument_stack as ::core::ffi::c_int as arity_t
    {
        (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
            IL_ITEM_PHANTOM_OPERAND;
        (*(*il).instr.offset(j.wrapping_add(checkdelta2) as isize)).type_0 =
            IL_ITEM_PHANTOM_OPERAND;
        (*(*il).instr.offset(j as isize)).type_0 = IL_ITEM_PHANTOM_OPERATOR;
        (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize))
            .c2rust_unnamed
            .i = (*current).c2rust_unnamed.i;
        (*(*il).instr.offset(j.wrapping_add(7 as u32) as isize)).arity =
            (*current).arity.wrapping_add(4 as arity_t);
        return 7 as u8;
    } else {
        return 0 as u8;
    };
}
unsafe extern "C" fn nextstop(mut il: *mut cff_CharstringIL, mut j: u32) -> u32 {
    let mut delta: u32 = 0 as u32;
    while j.wrapping_add(delta) < (*il).length
        && (*(*il).instr.offset(j.wrapping_add(delta) as isize)).type_0 as ::core::ffi::c_uint
            == IL_ITEM_OPERAND as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        delta = delta.wrapping_add(1);
    }
    return delta;
}
unsafe extern "C" fn decideAdvance(
    mut il: *mut cff_CharstringIL,
    mut j: u32,
    mut _optimizeLevel: u8,
) -> u8 {
    let mut r: u8 = 0 as u8;
    r = zroll(
        il,
        j,
        op_rlineto as ::core::ffi::c_int as i32,
        op_hlineto as ::core::ffi::c_int as i32,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rlineto as ::core::ffi::c_int as i32,
        op_vlineto as ::core::ffi::c_int as i32,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rmoveto as ::core::ffi::c_int as i32,
        op_hmoveto as ::core::ffi::c_int as i32,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rmoveto as ::core::ffi::c_int as i32,
        op_vmoveto as ::core::ffi::c_int as i32,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as i32,
        op_hvcurveto as ::core::ffi::c_int as i32,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as i32,
        op_vhcurveto as ::core::ffi::c_int as i32,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as i32,
        op_hhcurveto as ::core::ffi::c_int as i32,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as i32,
        op_vvcurveto as ::core::ffi::c_int as i32,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as i32,
        6 as i32,
        op_rrcurveto as ::core::ffi::c_int as i32,
        op_rrcurveto as ::core::ffi::c_int as i32,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as i32,
        2 as i32,
        op_rlineto as ::core::ffi::c_int as i32,
        op_rcurveline as ::core::ffi::c_int as i32,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_rlineto as ::core::ffi::c_int as i32,
        6 as i32,
        op_rrcurveto as ::core::ffi::c_int as i32,
        op_rlinecurve as ::core::ffi::c_int as i32,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_rlineto as ::core::ffi::c_int as i32,
        2 as i32,
        op_rlineto as ::core::ffi::c_int as i32,
        op_rlineto as ::core::ffi::c_int as i32,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_hstemhm as ::core::ffi::c_int as i32,
        0 as i32,
        op_hintmask as ::core::ffi::c_int as i32,
        op_hintmask as ::core::ffi::c_int as i32,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_vstemhm as ::core::ffi::c_int as i32,
        0 as i32,
        op_hintmask as ::core::ffi::c_int as i32,
        op_hintmask as ::core::ffi::c_int as i32,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_hstemhm as ::core::ffi::c_int as i32,
        0 as i32,
        op_cntrmask as ::core::ffi::c_int as i32,
        op_cntrmask as ::core::ffi::c_int as i32,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_vstemhm as ::core::ffi::c_int as i32,
        0 as i32,
        op_cntrmask as ::core::ffi::c_int as i32,
        op_cntrmask as ::core::ffi::c_int as i32,
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
#[no_mangle]
pub unsafe extern "C" fn cff_optimizeIL(
    mut il: *mut cff_CharstringIL,
    mut options: *const otfcc_Options,
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
#[no_mangle]
pub unsafe extern "C" fn cff_build_IL(mut il: *mut cff_CharstringIL) -> *mut caryll_Buffer {
    let mut blob: *mut caryll_Buffer = bufnew();
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
#[no_mangle]
pub unsafe extern "C" fn cff_shrinkIL(mut il: *mut cff_CharstringIL) -> *mut cff_CharstringIL {
    let mut out: *mut cff_CharstringIL = ::core::ptr::null_mut::<cff_CharstringIL>();
    out = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_CharstringIL>() as usize,
        457 as ::core::ffi::c_ulong,
    ) as *mut cff_CharstringIL;
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
#[no_mangle]
pub unsafe extern "C" fn cff_ILmergeIL(
    mut self_0: *mut cff_CharstringIL,
    mut il: *mut cff_CharstringIL,
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
#[no_mangle]
pub unsafe extern "C" fn instruction_eq(
    mut z1: *mut cff_CharstringInstruction,
    mut z2: *mut cff_CharstringInstruction,
) -> bool {
    if (*z1).type_0 as ::core::ffi::c_uint == (*z2).type_0 as ::core::ffi::c_uint {
        if (*z1).type_0 as ::core::ffi::c_uint
            == IL_ITEM_OPERAND as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*z1).type_0 as ::core::ffi::c_uint
                == IL_ITEM_PHANTOM_OPERAND as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return (*z1).c2rust_unnamed.d == (*z2).c2rust_unnamed.d;
        } else {
            return (*z1).c2rust_unnamed.i == (*z2).c2rust_unnamed.i;
        }
    } else {
        return false;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_ilEqual(
    mut a: *mut cff_CharstringIL,
    mut b: *mut cff_CharstringIL,
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
