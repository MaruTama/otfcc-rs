use crate::support::primitives::{Pos, Scale};
use std::rc::Rc;

use crate::vf::region::VqRegion;
use crate::vf::region::vq_compare_region;
#[derive(Clone, Debug)]
pub enum VqSegment {
    Still(Pos),
    Delta(VqSegmentDelta),
}
impl VqSegment {
    // The byte `hash_vqs` (`otf_reader/unconsolidate.rs`) writes into the
    // glyph hash: 0 for `Still`, 1 for `Delta`. Renumbering would change
    // which glyphs are treated as duplicates.
    pub fn discriminant_byte(&self) -> u8 {
        match self {
            VqSegment::Still(_) => 0,
            VqSegment::Delta(_) => 1,
        }
    }
    // For `fill_the_gaps` (`table/glyf/read.rs`), whose `nudges` are all
    // `Delta`s; these panic on a `Still`.
    pub fn is_touched(&self) -> bool {
        matches!(self, VqSegment::Delta(VqSegmentDelta { touched: true, .. }))
    }
    pub fn unwrap_delta(&self) -> VqSegmentDelta {
        match self {
            VqSegment::Delta(d) => d.clone(),
            VqSegment::Still(_) => panic!("VqSegment::unwrap_delta called on a Still segment"),
        }
    }
    pub fn delta_mut(&mut self) -> &mut VqSegmentDelta {
        match self {
            VqSegment::Delta(d) => d,
            VqSegment::Still(_) => panic!("VqSegment::delta_mut called on a Still segment"),
        }
    }
}
// `region` is shared with the `FvarTable.masters` entry it was registered
// as (`fvar_register_region`). It is an `Rc` rather than an index because
// sorting segments compares region contents numerically
// (`vq_compare_region`); a region found to duplicate an existing one is
// dropped before any segment gets it.
#[derive(Clone, Debug)]
pub struct VqSegmentDelta {
    pub quantity: Pos,
    pub touched: bool,
    pub region: Rc<VqRegion>,
}
#[derive(Clone, Default, Debug)]
pub struct VQ {
    pub kernel: Pos,
    pub shift: Vec<VqSegment>,
}
#[inline]
fn init_vq_segment(vqs: &mut VqSegment) {
    *vqs = VqSegment::Still(0_i32 as Pos);
}
#[inline]
fn copy_vq_segment(dst: &mut VqSegment, src: &VqSegment) {
    match src {
        VqSegment::Still(v) => {
            *dst = VqSegment::Still(*v);
        }
        VqSegment::Delta(sd) => {
            // `touched` is kept when `dst` is already a `Delta`, and
            // `false` otherwise.
            let touched = match *dst {
                VqSegment::Delta(ref dd) => dd.touched,
                VqSegment::Still(_) => false,
            };
            *dst = VqSegment::Delta(VqSegmentDelta {
                quantity: sd.quantity,
                touched,
                region: Rc::clone(&sd.region),
            });
        }
    }
}
#[inline]
fn dispose_vq_segment(vqs: &mut VqSegment) {
    init_vq_segment(vqs);
}
#[inline]
fn vq_segment_copy(dst: &mut VqSegment, src: &VqSegment) {
    copy_vq_segment(dst, src);
}
#[inline]
fn vq_segment_dispose(x: &mut VqSegment) {
    dispose_vq_segment(x);
}
// Both take `&VqSegment`: a `Delta` segment holds an `Rc<VqRegion>`, and
// every call site here only reads its argument.
fn vqs_compare(a: &VqSegment, b: &VqSegment) -> i32 {
    match (a, b) {
        (VqSegment::Still(_), VqSegment::Delta(_)) => -1_i32,
        (VqSegment::Delta(_), VqSegment::Still(_)) => 1_i32,
        (VqSegment::Still(av), VqSegment::Still(bv)) => {
            if av < bv {
                return -1_i32;
            }
            if av > bv {
                return 1_i32;
            }
            0_i32
        }
        (VqSegment::Delta(ad), VqSegment::Delta(bd)) => {
            let vqrc: i32 = vq_compare_region(&ad.region, &bd.region);
            if vqrc != 0 {
                return vqrc;
            }
            if ad.quantity < bd.quantity {
                return -1_i32;
            }
            if ad.quantity > bd.quantity {
                return 1_i32;
            }
            0_i32
        }
    }
}
pub(crate) fn vq_neutral() -> VQ {
    return vq_create_still(0_i32 as Pos);
}
fn vqs_compatible(a: &VqSegment, b: &VqSegment) -> bool {
    match (a, b) {
        (VqSegment::Still(_), VqSegment::Still(_)) => true,
        (VqSegment::Delta(ad), VqSegment::Delta(bd)) => {
            0_i32 == vq_compare_region(&ad.region, &bd.region)
        }
        _ => false,
    }
}
fn simplify_vq(x: &mut VQ) {
    if x.shift.is_empty() {
        return;
    }
    let shift: &mut Vec<VqSegment> = &mut x.shift;
    shift.sort_by(|a, b| vqs_compare(a, b).cmp(&0_i32));
    let mut k: usize = 0_usize;
    for j in 1..shift.len() {
        if vqs_compatible(&shift[k], &shift[j]) {
            let other = shift[j].clone();
            match &mut shift[k] {
                VqSegment::Still(sv) => {
                    if let VqSegment::Still(ov) = other {
                        *sv += ov;
                    }
                }
                VqSegment::Delta(sd) => {
                    if let VqSegment::Delta(od) = other {
                        sd.quantity += od.quantity;
                    }
                }
            }
            vq_segment_dispose(&mut shift[j]);
        } else {
            shift[k] = shift[j].clone();
            k = k.wrapping_add(1);
        }
    }
    shift.truncate(k.wrapping_add(1_usize));
}
pub(crate) fn vq_inplace_plus(a: &mut VQ, b: VQ) {
    a.kernel += b.kernel;
    for p in 0..b.shift.len() {
        let k: VqSegment = b.shift[p].clone();
        if let VqSegment::Still(still) = k {
            a.kernel += still;
        } else {
            let mut s: VqSegment = VqSegment::Still(0.);
            vq_segment_copy(&mut s, &k);
            a.shift.push(s);
        }
    }
    simplify_vq(a);
}
fn vq_inplace_scale(a: &mut VQ, b: Pos) {
    a.kernel *= b;
    let shift: &mut Vec<VqSegment> = &mut a.shift;
    for s in shift.iter_mut() {
        match s {
            VqSegment::Still(sv) => {
                *sv *= b;
            }
            VqSegment::Delta(sd) => {
                sd.quantity *= b;
            }
        }
    }
}
fn vq_inplace_negate(a: &mut VQ) {
    vq_inplace_scale(a, -1_i32 as Pos);
}
fn vq_negate(a: VQ) -> VQ {
    let mut result: VQ = a;
    vq_inplace_negate(&mut result);
    return result;
}
#[inline]
pub(crate) fn vq_minus(a: VQ, b: VQ) -> VQ {
    let mut result: VQ = vq_neutral();
    vq_inplace_plus(&mut result, a);
    vq_inplace_minus(&mut result, b);
    return result;
}
#[inline]
fn vq_inplace_minus(a: &mut VQ, b: VQ) {
    let tb: VQ = vq_negate(b);
    vq_inplace_plus(a, tb);
}
#[inline]
pub(crate) fn vq_inplace_plus_scale(a: &mut VQ, b: Pos, c: VQ) {
    let x: VQ = vq_scale(c, b);
    vq_inplace_plus(a, x);
}
#[inline]
pub(crate) fn vq_scale(a: VQ, b: Pos) -> VQ {
    let mut result: VQ = a;
    vq_inplace_scale(&mut result, b);
    return result;
}
pub(crate) fn vq_compare(a: VQ, b: VQ) -> i32 {
    if a.shift.len() < b.shift.len() {
        return -1_i32;
    }
    if a.shift.len() > b.shift.len() {
        return 1_i32;
    }
    for (av, bv) in a.shift.iter().zip(b.shift.iter()) {
        let cr: i32 = vqs_compare(av, bv);
        if cr != 0 {
            return cr;
        }
    }
    return (a.kernel - b.kernel) as i32;
}
pub(crate) fn vq_get_still(v: VQ) -> Pos {
    let mut result: Pos = v.kernel;
    for j in 0..v.shift.len() {
        if let VqSegment::Still(still) = &v.shift[j] {
            result += *still;
        }
    }
    return result;
}
pub(crate) fn vq_create_still(x: Pos) -> VQ {
    VQ {
        kernel: x,
        shift: Vec::new(),
    }
}
pub(crate) fn vq_is_still(v: VQ) -> bool {
    v.shift.iter().all(|s| matches!(s, VqSegment::Still(_)))
}
pub(crate) fn vq_is_zero(v: VQ, err: Pos) -> bool {
    // `f64::abs` gives the same result as C's `fabs` for every input.
    return vq_is_still(v.clone()) as i32 != 0
        && (vq_get_still(v) as f64).abs() < err;
}
// Takes `&Rc<VqRegion>`, not `Rc<VqRegion>`: `table/glyf/read.rs`'s four
// call sites in `apply_polymorphism` all share one region across several
// calls (two `apply_coords` calls plus up to four `vq_add_delta` calls per
// tuple), so borrowing here and cloning once per constructed
// `VqSegmentDelta` (below) avoids bumping the refcount at every call site
// just to satisfy a by-value parameter.
pub(crate) fn vq_add_delta(v: &mut VQ, touched: bool, r: &Rc<VqRegion>, quantity: Pos) {
    if quantity == 0. {
        return;
    }
    let nudge = VqSegment::Delta(VqSegmentDelta {
        quantity,
        touched,
        region: Rc::clone(r),
    });
    v.shift.push(nudge);
}
pub(crate) fn vq_point_linear_tfm(ax: VQ, a: Pos, x: VQ, b: Pos, y: VQ) -> VQ {
    let mut target_x: VQ = ax;
    vq_inplace_plus_scale(&mut target_x, a as Scale, x);
    vq_inplace_plus_scale(&mut target_x, b as Scale, y);
    return target_x;
}
#[cfg(test)]
mod tests {
    use super::*;

    // `f64::abs` matches C99's `fabs` (`|x|`, no rounding) for every
    // input class.
    #[test]
    fn f64_abs_matches_fabs_contract_on_every_input_class() {
        assert_eq!(1.5_f64.abs(), 1.5);
        assert_eq!((-1.5_f64).abs(), 1.5);
        assert_eq!(0.0_f64.abs().to_bits(), 0.0_f64.to_bits());
        assert_eq!((-0.0_f64).abs().to_bits(), 0.0_f64.to_bits());
        assert_eq!(f64::INFINITY.abs(), f64::INFINITY);
        assert_eq!(f64::NEG_INFINITY.abs(), f64::INFINITY);
        assert!(f64::NAN.abs().is_nan());
    }

    #[test]
    fn vq_is_zero_is_sign_insensitive_around_the_error_margin() {
        let err = 0.5;
        assert!(vq_is_zero(vq_create_still(0.4), err));
        assert!(vq_is_zero(vq_create_still(-0.4), err));
        assert!(!vq_is_zero(vq_create_still(0.6), err));
        assert!(!vq_is_zero(vq_create_still(-0.6), err));
    }

    // This discriminant is written into the glyph hash byte-for-byte --
    // `hash_vqs` in otf_reader/unconsolidate.rs does `bufwrite8(buf, s.type_0 as
    // u8)` -- and that hash decides which glyphs are treated as duplicates.
    // Renumbering the variants would silently change which glyphs get merged.
    #[test]
    fn vqsegtype_discriminants_are_the_hashed_values() {
        assert_eq!(VqSegment::Still(0.).discriminant_byte(), 0);
        assert_eq!(
            VqSegment::Delta(VqSegmentDelta {
                quantity: 0.,
                touched: false,
                region: Rc::new(VqRegion { dimensions: 0, spans: Vec::new() }),
            })
            .discriminant_byte(),
            1
        );
    }

    // `vq_is_still` was a manual index loop returning `false` on the first
    // non-`Still` segment, `true` otherwise (including the empty-shift
    // case). Converted to `.iter().all(...)` -- these cases pin down the
    // empty/all-still/short-circuit-on-first-mismatch behavior across the
    // conversion.
    #[test]
    fn vq_is_still_matches_manual_loop_semantics() {
        assert!(vq_is_still(VQ { kernel: 0., shift: Vec::new() }));
        assert!(vq_is_still(VQ {
            kernel: 0.,
            shift: vec![VqSegment::Still(1.), VqSegment::Still(2.)],
        }));
        let delta = VqSegment::Delta(VqSegmentDelta {
            quantity: 1.,
            touched: false,
            region: Rc::new(VqRegion { dimensions: 0, spans: Vec::new() }),
        });
        assert!(!vq_is_still(VQ {
            kernel: 0.,
            shift: vec![delta.clone()],
        }));
        // A non-`Still` segment after a `Still` one must still short-circuit
        // to `false` (not just check the first element).
        assert!(!vq_is_still(VQ {
            kernel: 0.,
            shift: vec![VqSegment::Still(1.), delta],
        }));
    }

    // `vq_compare` compares by shift length first, then element-wise via
    // `vqs_compare` with early return on the first nonzero result, then
    // falls back to `kernel` difference. Converted the element-wise loop
    // to `.iter().zip(...)` -- these cases cover the length short-circuit,
    // the zip's own early-return, and the kernel-difference fallback when
    // every element compares equal.
    #[test]
    fn vq_compare_matches_manual_loop_semantics() {
        let a = VQ { kernel: 0., shift: vec![VqSegment::Still(1.)] };
        let b = VQ { kernel: 0., shift: Vec::new() };
        assert_eq!(vq_compare(a.clone(), b.clone()), 1);
        assert_eq!(vq_compare(b, a), -1);

        // Equal-length shifts, first element already differs: must return
        // that element's comparison, not fall through to `kernel`.
        let a = VQ {
            kernel: 100.,
            shift: vec![VqSegment::Still(1.), VqSegment::Still(5.)],
        };
        let b = VQ {
            kernel: 0.,
            shift: vec![VqSegment::Still(2.), VqSegment::Still(5.)],
        };
        assert_eq!(vq_compare(a.clone(), b.clone()), -1);
        assert_eq!(vq_compare(b, a), 1);

        // Every element equal: falls back to kernel difference.
        let a = VQ {
            kernel: 3.,
            shift: vec![VqSegment::Still(1.), VqSegment::Still(5.)],
        };
        let b = VQ {
            kernel: 1.,
            shift: vec![VqSegment::Still(1.), VqSegment::Still(5.)],
        };
        assert_eq!(vq_compare(a, b), 2);

        // Both shifts empty: no zip iterations, kernel decides.
        let a = VQ { kernel: 0., shift: Vec::new() };
        let b = VQ { kernel: 0., shift: Vec::new() };
        assert_eq!(vq_compare(a, b), 0);
    }
}
