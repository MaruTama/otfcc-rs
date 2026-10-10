use crate::support::options::Options;
use crate::support::primitives::{Arity, Pos, ShapeId};

use crate::libcff::CffCharstringOperator;
use crate::libcff::{
    OP_CNTRMASK, OP_ENDCHAR, OP_HHCURVETO, OP_HINTMASK, OP_HLINETO, OP_HMOVETO, OP_HSTEM,
    OP_HSTEMHM, OP_HVCURVETO, OP_RCURVELINE, OP_RLINECURVE, OP_RLINETO, OP_RMOVETO, OP_RRCURVETO,
    OP_VHCURVETO, OP_VLINETO, OP_VMOVETO, OP_VSTEM, OP_VSTEMHM, OP_VVCURVETO, TYPE2_ARGUMENT_STACK,
};
use crate::table::glyf::{Contour, Glyph, MaskList, StemDefList};

use crate::libcff::opmean::cff_get_standard_arity;
use crate::table::glyf::glyf_point_dup;
use crate::vf::vq::VQ;
use crate::vf::vq::{vq_get_still, vq_minus, vq_neutral};
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CffInstructionType {
    Operand = 0,
    Operator = 1,
    Special = 2,
    PhantomOperator = 3,
    PhantomOperand = 4,
}
// `kind`'s five values don't map 1:1 onto `CffCharstringArgument`'s two
// arms -- `Operand`/`PhantomOperand` hold a number, `Operator`/`Special`/
// `PhantomOperator` an opcode -- so `kind` stays a separate field: it also
// distinguishes a real operator from a `Special` non-operator byte, and
// the "Phantom" variants from their non-phantom counterparts.
#[derive(Copy, Clone, Debug)]
pub struct CffCharstringInstruction {
    pub kind: CffInstructionType,
    pub arity: Arity,
    pub arg: CffCharstringArgument,
}
#[derive(Copy, Clone, Debug)]
pub enum CffCharstringArgument {
    D(f64),
    I(i32),
}
impl CffCharstringInstruction {
    /// Panics instead of reading union garbage if this instruction's
    /// `type_0` didn't actually imply the `D` arm -- every call site
    /// already established that via `type_0` before reaching here.
    pub fn d(&self) -> f64 {
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
    pub fn set_d(&mut self, v: f64) {
        self.arg = CffCharstringArgument::D(v);
    }
    pub fn set_i(&mut self, v: i32) {
        self.arg = CffCharstringArgument::I(v);
    }
}
#[derive(Clone, Debug)]
pub struct CffCharstringIl {
    pub instr: Vec<CffCharstringInstruction>,
}
pub fn il_push_operand(il: &mut CffCharstringIl, x: f64) {
    il.instr.push(CffCharstringInstruction {
        kind: CffInstructionType::Operand,
        arity: 0 as Arity,
        arg: CffCharstringArgument::D(x),
    });
}
pub fn il_push_vq(il: &mut CffCharstringIl, x: VQ) {
    il_push_operand(il, vq_get_still(x) as f64);
}
pub fn il_push_special(il: &mut CffCharstringIl, s: i32) {
    il.instr.push(CffCharstringInstruction {
        kind: CffInstructionType::Special,
        arity: 0 as Arity,
        arg: CffCharstringArgument::I(s),
    });
}
pub fn il_push_op(il: &mut CffCharstringIl, op: CffCharstringOperator) {
    // The `.i` arm stays a bare `i32`: `CffInstructionType::Special` stores
    // non-operator bytes in the very same field, so the type lives on the way
    // in, not in the storage.
    il.instr.push(CffCharstringInstruction {
        kind: CffInstructionType::Operator,
        arity: cff_get_standard_arity(op) as Arity,
        arg: CffCharstringArgument::I(op.0),
    });
}
fn il_moveto(il: &mut CffCharstringIl, dx: VQ, dy: VQ) {
    il_push_vq(il, dx);
    il_push_vq(il, dy);
    il_push_op(il, OP_RMOVETO);
}
fn il_lineto(il: &mut CffCharstringIl, dx: VQ, dy: VQ) {
    il_push_vq(il, dx);
    il_push_vq(il, dy);
    il_push_op(il, OP_RLINETO);
}
fn il_curveto(il: &mut CffCharstringIl, dx1: VQ, dy1: VQ, dx2: VQ, dy2: VQ, dx3: VQ, dy3: VQ) {
    il_push_vq(il, dx1);
    il_push_vq(il, dy1);
    il_push_vq(il, dx2);
    il_push_vq(il, dy2);
    il_push_vq(il, dx3);
    il_push_vq(il, dy3);
    il_push_op(il, OP_RRCURVETO);
}
/// Where the mask-group walk below is in the glyph's own contour/point
/// count -- `contours`/`points` are always passed together (both call
/// sites in `il_push_masks` pass the same pair through unchanged), so
/// bundling avoids the 8-argument form `clippy::too_many_arguments`
/// flagged.
struct ShapePosition {
    contours: u16,
    points: u16,
}
/// How many stem-hint bits each mask byte packs, per axis -- `nh`/`nv` are
/// likewise always passed together (`il_push_masks`'s own `stem_h_len`/
/// `stem_v_len`, identical at both call sites).
struct StemCounts {
    h: u16,
    v: u16,
}
fn _il_push_maskgroup(
    il: &mut CffCharstringIl,
    masks: &MaskList,
    position: &ShapePosition,
    stems: &StemCounts,
    jm: &mut u16,
    op: CffCharstringOperator,
) {
    let n = masks.len() as ShapeId;
    while *jm < n {
        let mask = &masks[*jm as usize];
        let reached = mask.contours_before < position.contours
            || mask.contours_before == position.contours && mask.points_before <= position.points;
        if !reached {
            break;
        }
        il_push_op(il, op);
        // One bit per stem, horizontal stems first, packed most significant
        // bit first and padded with zeros to a whole byte.
        let mut mask_byte: u8 = 0;
        let mut bits: u8 = 0;
        let h_bits = (0..stems.h).map(|j| mask.mask_h.get(j as usize));
        let v_bits = (0..stems.v).map(|j| mask.mask_v.get(j as usize));
        for bit in h_bits.chain(v_bits) {
            mask_byte = mask_byte << 1 | bit as u8;
            bits += 1;
            if bits == 8 {
                il_push_special(il, mask_byte as i32);
                bits = 0;
            }
        }
        if bits != 0 {
            mask_byte <<= 8 - bits;
            il_push_special(il, mask_byte as i32);
        }
        *jm += 1;
    }
}
fn il_push_masks(
    il: &mut CffCharstringIl,
    g: &Glyph,
    contours: u16,
    points: u16,
    jh: &mut u16,
    jm: &mut u16,
) {
    if g.stem_h.is_empty() && g.stem_v.is_empty() {
        return;
    }
    let position = ShapePosition { contours, points };
    let stems = StemCounts {
        h: g.stem_h.len() as u16,
        v: g.stem_v.len() as u16,
    };
    _il_push_maskgroup(il, &g.contour_masks, &position, &stems, jh, OP_CNTRMASK);
    _il_push_maskgroup(il, &g.hint_masks, &position, &stems, jm, OP_HINTMASK);
}
// A genuinely empty stem list is a normal, well-formed glyph, so it pushes
// nothing.
fn _il_push_stemgroup(
    il: &mut CffCharstringIl,
    stems: &StemDefList,
    hasmask: bool,
    haswidth: bool,
    ophm: CffCharstringOperator,
    oph: CffCharstringOperator,
) {
    if stems.is_empty() {
        return;
    }
    // Stems are written as (distance from the previous stem's far edge,
    // width) pairs. A stem operator takes at most a full argument stack, so
    // a long list is split; the width, when present, counts as the first
    // argument of the first operator.
    let mut last_edge: Pos = 0.0;
    let mut nn = u16::from(haswidth);
    for stem in stems {
        il_push_operand(il, stem.position - last_edge);
        il_push_operand(il, stem.width);
        last_edge = stem.position + stem.width;
        nn += 1;
        if nn as u32 >= TYPE2_ARGUMENT_STACK {
            il_push_op(il, if hasmask { OP_HSTEMHM } else { OP_HSTEM });
            let last_idx = il.instr.len() - 1;
            il.instr[last_idx].arity = nn as Arity;
            nn = 0;
        }
    }
    il_push_op(il, if hasmask { ophm } else { oph });
    let last_idx = il.instr.len() - 1;
    il.instr[last_idx].arity = nn as Arity;
}
fn il_push_stems(il: &mut CffCharstringIl, g: &Glyph, hasmask: bool, haswidth: bool) {
    _il_push_stemgroup(il, &g.stem_h, hasmask, haswidth, OP_HSTEMHM, OP_HSTEM);
    _il_push_stemgroup(il, &g.stem_v, hasmask, haswidth, OP_VSTEMHM, OP_VSTEM);
}
pub fn cff_compile_glyph_to_il(
    g: &Glyph,
    default_width: u16,
    nominal_width: u16,
) -> CffCharstringIl {
    let mut il = CffCharstringIl { instr: Vec::new() };
    // Each contour with every point turned into a delta from the point
    // before it (the first from the previous contour's last point), closed
    // by repeating its first point when it ends off the curve.
    let mut relative_contours: Vec<Contour> = Vec::with_capacity(g.contours.len());
    let mut x: VQ = vq_neutral();
    let mut y: VQ = vq_neutral();
    for contour in &g.contours {
        let mut newcontour: Contour = contour.iter().map(|p| glyf_point_dup(p.clone())).collect();
        if newcontour.len() > 2 && newcontour[newcontour.len() - 1].on_curve == 0 {
            let first = newcontour[0].clone();
            newcontour.push(glyf_point_dup(first));
        }
        let n = point_count(&newcontour);
        for point in newcontour.iter_mut().take(n) {
            let dx: VQ = vq_minus(point.x.clone(), x.clone());
            let dy: VQ = vq_minus(point.y.clone(), y.clone());
            x = std::mem::replace(&mut point.x, dx);
            y = std::mem::replace(&mut point.y, dy);
        }
        relative_contours.push(newcontour);
    }
    let hasmask: bool = !g.hint_masks.is_empty() || !g.contour_masks.is_empty();
    let glyph_adw_const: Pos = vq_get_still(g.advance_width.clone());
    let haswidth: bool = glyph_adw_const != default_width as Pos;
    if haswidth {
        // `advanceWidth` comes from the JSON, so the cast can saturate to
        // `i32::MIN`/`MAX`; `saturating_sub` keeps the delta from
        // overflowing then.
        il_push_operand(
            &mut il,
            (glyph_adw_const as i32).saturating_sub(nominal_width as i32) as f64,
        );
    }
    il_push_stems(&mut il, g, hasmask, haswidth);
    // How far the outline has got, in the units the hint and counter masks
    // record their positions in: contours finished, and points into the
    // current contour.
    let mut contours_sofar: ShapeId = 0;
    let mut points_sofar: ShapeId = 0;
    let mut jh: ShapeId = 0;
    let mut jm: ShapeId = 0;
    if hasmask {
        il_push_masks(&mut il, g, contours_sofar, points_sofar, &mut jh, &mut jm);
    }
    for contour in &relative_contours {
        let n = point_count(contour);
        if n == 0 {
            continue;
        }
        il_moveto(&mut il, contour[0].x.clone(), contour[0].y.clone());
        points_sofar += 1;
        if hasmask {
            il_push_masks(&mut il, g, contours_sofar, points_sofar, &mut jh, &mut jm);
        }
        let mut j = 1;
        while j < n {
            if contour[j].on_curve != 0 {
                il_lineto(&mut il, contour[j].x.clone(), contour[j].y.clone());
                points_sofar += 1;
            } else if j + 2 < n && contour[j + 1].on_curve == 0 && contour[j + 2].on_curve != 0 {
                il_curveto(
                    &mut il,
                    contour[j].x.clone(),
                    contour[j].y.clone(),
                    contour[j + 1].x.clone(),
                    contour[j + 1].y.clone(),
                    contour[j + 2].x.clone(),
                    contour[j + 2].y.clone(),
                );
                points_sofar += 3;
                j += 2;
            } else {
                il_lineto(&mut il, contour[j].x.clone(), contour[j].y.clone());
                points_sofar += 1;
            }
            if hasmask {
                il_push_masks(&mut il, g, contours_sofar, points_sofar, &mut jh, &mut jm);
            }
            j += 1;
        }
        contours_sofar += 1;
        points_sofar = 0;
    }
    il_push_op(&mut il, OP_ENDCHAR);
    il
}
/// A contour's point count as the charstring writer counts it, in 16 bits.
/// A JSON contour has at most 65,535 points, but closing it can add one, and
/// a contour of 65,536 points then counts as empty, as it always has.
fn point_count(contour: &Contour) -> usize {
    contour.len() as ShapeId as usize
}
fn il_matchtype(il: &CffCharstringIl, j: u32, k: u32, t: CffInstructionType) -> bool {
    if k >= il.instr.len() as u32 {
        return false;
    }
    for m in j..k {
        if il.instr[m as usize].kind as u32 != t as u32 {
            return false;
        }
    }
    return true;
}
fn il_matchop(il: &CffCharstringIl, j: u32, op: CffCharstringOperator) -> bool {
    if il.instr[j as usize].kind != CffInstructionType::Operator {
        return false;
    }
    if il.instr[j as usize].i() != op.0 {
        return false;
    }
    return true;
}
/// Collapse `op` into `op2` when the operands flagged in `zeros` are all zero.
///
/// `zeros` was a vararg list of `arity` ints -- the count implied by
/// `cff_get_standard_arity(op)` and trusted, never checked. As a slice the two can
/// be compared, and the flags read as the booleans they always were.
fn zroll(
    il: &mut CffCharstringIl,
    j: u32,
    op: CffCharstringOperator,
    op2: CffCharstringOperator,
    zeros: &[bool],
) -> u8 {
    let arity: u8 = cff_get_standard_arity(op);
    let end = j + arity as u32;
    if arity > 16 || end >= il.instr.len() as u32 {
        return 0;
    }
    let follows_phantom = j > 0 && il_matchtype(il, j - 1, j, CffInstructionType::PhantomOperator);
    if follows_phantom
        || !il_matchop(il, end, op)
        || !il_matchtype(il, j, end, CffInstructionType::Operand)
    {
        return 0;
    }
    debug_assert_eq!(
        zeros.len(),
        arity as usize,
        "zroll: flag count must match the operator's arity"
    );
    let flagged = || (0..arity as usize).filter(|&m| zeros[m]);
    if !flagged().all(|m| il.instr[j as usize + m].d() == 0.0) {
        return 0;
    }
    let result_arity = arity as usize - flagged().count();
    for m in flagged() {
        il.instr[j as usize + m].kind = CffInstructionType::PhantomOperand;
    }
    il.instr[end as usize].set_i(op2.0);
    il.instr[end as usize].arity = result_arity as Arity;
    return arity;
}
fn opop_roll(
    il: &mut CffCharstringIl,
    j: u32,
    op1: CffCharstringOperator,
    arity: u32,
    op2: CffCharstringOperator,
    resultop: CffCharstringOperator,
) -> u8 {
    let next_idx = j + 1 + arity;
    if next_idx >= il.instr.len() as u32 {
        return 0;
    }
    // `CffCharstringInstruction` is `Copy`: reading both instructions out
    // first avoids holding two borrows into `il.instr` at once.
    let current = il.instr[j as usize];
    let nextop = il.instr[next_idx as usize];
    if il_matchop(il, j, op1)
        && il_matchtype(il, j + 1, next_idx, CffInstructionType::Operand)
        && il_matchop(il, next_idx, op2)
        && current.arity + nextop.arity <= TYPE2_ARGUMENT_STACK
    {
        il.instr[j as usize].kind = CffInstructionType::PhantomOperator;
        il.instr[next_idx as usize].set_i(resultop.0);
        il.instr[next_idx as usize].arity = nextop.arity + current.arity;
        return (arity + 1) as u8;
    } else {
        return 0;
    };
}
fn hvlineto_roll(il: &mut CffCharstringIl, j: u32) -> u8 {
    if j + 3 >= il.instr.len() as u32 {
        return 0;
    }
    if !(il_matchop(il, j, OP_HLINETO) || il_matchop(il, j, OP_VLINETO)) {
        return 0;
    }
    // Read out after the check above: `i()` is only valid on an operator.
    let current = il.instr[j as usize];
    let odd_arity = current.arity & 1 != 0;
    let checkdelta: u32 = if odd_arity ^ (current.i() == OP_VLINETO.0) { 1 } else { 2 };
    if il_matchop(il, j + 3, OP_RLINETO)
        && il_matchtype(il, j + 1, j + 3, CffInstructionType::Operand)
        && il.instr[(j + checkdelta) as usize].d() == 0.0
        && current.arity < TYPE2_ARGUMENT_STACK
    {
        il.instr[(j + checkdelta) as usize].kind = CffInstructionType::PhantomOperand;
        il.instr[j as usize].kind = CffInstructionType::PhantomOperator;
        let end_idx = (j + 3) as usize;
        il.instr[end_idx].set_i(current.i());
        il.instr[end_idx].arity = current.arity + 1;
        return 3;
    } else {
        return 0;
    };
}
fn hvvhcurve_roll(il: &mut CffCharstringIl, j: u32) -> u8 {
    if !il_matchop(il, j, OP_HVCURVETO) && !il_matchop(il, j, OP_VHCURVETO) {
        return 0;
    }
    let current = il.instr[j as usize];
    if j + 7 >= il.instr.len() as u32 || current.arity & 1 != 0 {
        return 0;
    }
    let hvcase: bool = (current.arity >> 2 & 1 != 0) ^ (current.i() == OP_HVCURVETO.0);
    let checkdelta1: u32 = if hvcase { 2 } else { 1 };
    let checkdelta2: u32 = if hvcase { 5 } else { 6 };
    let end_idx = (j + 7) as usize;
    if il_matchop(il, j + 7, OP_RRCURVETO)
        && il_matchtype(il, j + 1, j + 7, CffInstructionType::Operand)
        && il.instr[(j + checkdelta1) as usize].d() == 0.0
    {
        if il.instr[(j + checkdelta2) as usize].d() == 0.0
            && current.arity + 4 <= TYPE2_ARGUMENT_STACK
        {
            il.instr[(j + checkdelta1) as usize].kind = CffInstructionType::PhantomOperand;
            il.instr[(j + checkdelta2) as usize].kind = CffInstructionType::PhantomOperand;
            il.instr[j as usize].kind = CffInstructionType::PhantomOperator;
            il.instr[end_idx].set_i(current.i());
            il.instr[end_idx].arity = current.arity + 4;
            return 7;
        } else if current.arity + 5 <= TYPE2_ARGUMENT_STACK {
            il.instr[(j + checkdelta1) as usize].kind = CffInstructionType::PhantomOperand;
            il.instr[j as usize].kind = CffInstructionType::PhantomOperator;
            il.instr[end_idx].set_i(current.i());
            il.instr[end_idx].arity = current.arity + 5;
            if hvcase {
                il.instr.swap((j + 5) as usize, (j + 6) as usize);
            }
            return 7;
        } else {
            return 0;
        }
    } else {
        return 0;
    };
}
fn hhvvcurve_roll(il: &mut CffCharstringIl, j: u32) -> u8 {
    if !il_matchop(il, j, OP_HHCURVETO) && !il_matchop(il, j, OP_VVCURVETO) {
        return 0;
    }
    let current = il.instr[j as usize];
    if j + 7 >= il.instr.len() as u32 {
        return 0;
    }
    let hh: bool = current.i() == OP_HHCURVETO.0;
    let checkdelta1: u32 = if hh { 2 } else { 1 };
    let checkdelta2: u32 = if hh { 6 } else { 5 };
    if il_matchop(il, j + 7, OP_RRCURVETO)
        && il_matchtype(il, j + 1, j + 7, CffInstructionType::Operand)
        && il.instr[(j + checkdelta1) as usize].d() == 0.0
        && il.instr[(j + checkdelta2) as usize].d() == 0.0
        && current.arity + 4 <= TYPE2_ARGUMENT_STACK
    {
        il.instr[(j + checkdelta1) as usize].kind = CffInstructionType::PhantomOperand;
        il.instr[(j + checkdelta2) as usize].kind = CffInstructionType::PhantomOperand;
        il.instr[j as usize].kind = CffInstructionType::PhantomOperator;
        let end_idx = (j + 7) as usize;
        il.instr[end_idx].set_i(current.i());
        il.instr[end_idx].arity = current.arity + 4;
        return 7;
    } else {
        return 0;
    };
}
fn nextstop(il: &CffCharstringIl, j: u32) -> u32 {
    let operands = il
        .instr
        .iter()
        .skip(j as usize)
        .take_while(|instr| instr.kind == CffInstructionType::Operand)
        .count();
    return operands as u32;
}
fn decide_advance(il: &mut CffCharstringIl, j: u32, mut _optimize_level: u8) -> u8 {
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
    r = opop_roll(il, j, OP_RRCURVETO, 6, OP_RRCURVETO, OP_RRCURVETO);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_RRCURVETO, 2, OP_RLINETO, OP_RCURVELINE);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_RLINETO, 6, OP_RRCURVETO, OP_RLINECURVE);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_RLINETO, 2, OP_RLINETO, OP_RLINETO);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_HSTEMHM, 0, OP_HINTMASK, OP_HINTMASK);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_VSTEMHM, 0, OP_HINTMASK, OP_HINTMASK);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_HSTEMHM, 0, OP_CNTRMASK, OP_CNTRMASK);
    if r != 0 {
        return r;
    }
    r = opop_roll(il, j, OP_VSTEMHM, 0, OP_CNTRMASK, OP_CNTRMASK);
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
    return 1;
}
pub fn cff_optimize_il(il: &mut CffCharstringIl, options: &Options) {
    if !options.cff_roll_char_string {
        return;
    }
    let mut j: u32 = 0;
    while j < il.instr.len() as u32 {
        j += decide_advance(il, j, options.cff_roll_char_string as u8) as u32;
    }
}
#[cfg(test)]
mod cff_compile_glyph_to_il_tests {
    use super::*;
    use crate::table::glyf::{Point, new_glyf_glyph};
    use crate::vf::vq::vq_create_still;

    // Compiling a glyph with at least one contour with at least one point
    // exercises `cff_compile_glyph_to_il`'s per-contour scratch copies;
    // under Miri this checks no invalid value is ever constructed or
    // dropped there.
    #[test]
    fn compiling_a_glyph_with_one_contour_does_not_construct_invalid_scratch_values() {
        let mut g = new_glyf_glyph();
        g.contours.push(vec![Point {
            x: vq_create_still(0.0),
            y: vq_create_still(0.0),
            on_curve: 1,
        }]);
        let il = cff_compile_glyph_to_il(&g, 0, 0);
        assert!(!il.instr.is_empty());
    }
}
