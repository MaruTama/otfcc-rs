unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::primitives::{Pos, Scale};
use std::rc::Rc;

use crate::vf::region::VqRegion;
use crate::vf::region::vq_compare_region;
// Was a C-shaped `struct { type_0: VQSegType, val: union { still: Pos,
// delta: VqSegmentDelta } }` -- the same "tag fully determines the live
// union arm" shape already converted elsewhere in the crate (`CffEncoding`,
// `ChainingSubtable`, etc.). Every field used to be `Copy` (no owned heap
// data -- `region: *const VqRegion` was a borrowed, non-owning pointer), so
// the enum stayed `Copy` too. Stage M-28 makes `region` an `Rc<VqRegion>`
// (shared ownership of the same `FvarTable.masters`-owned allocation this
// pointer used to alias, see `VqSegmentDelta`'s own doc comment below), and
// `Rc` is `Clone` but not `Copy`, so this enum drops `Copy` here -- every
// call site that used to rely on an implicit copy now calls `.clone()`
// explicitly (a refcount bump, not a content copy).
#[derive(Clone, Debug)]
pub enum VqSegment {
    Still(Pos),
    Delta(VqSegmentDelta),
}
impl VqSegment {
    // The byte `hash_vqs` (`otf_reader/unconsolidate.rs`) writes into the
    // glyph hash, byte-for-byte -- 0 for `Still`, 1 for `Delta`, matching
    // the old `VQSegType` discriminant values exactly. Renumbering would
    // silently change which glyphs get treated as duplicates. A plain `as`
    // cast can't do this any more now that the variants carry data, so
    // this stays an explicit, exhaustively-matched method instead.
    pub fn discriminant_byte(&self) -> u8 {
        match self {
            VqSegment::Still(_) => 0,
            VqSegment::Delta(_) => 1,
        }
    }
    // `table/glyf/read.rs`'s IUP-style gap-filling (`fill_the_gaps`)
    // constructs every element of its `nudges` array as `Delta` up front
    // (see `apply_coords`) and only ever reads/writes it as such -- these
    // three accessors replace the old unconditional `.val.delta.*` field
    // access, panicking instead of reading union garbage if that invariant
    // is ever violated.
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
// Stage 7-2-f closed out with `region` staying a raw, non-owning pointer
// into `table/fvar.rs`'s `fvar_register_region`-returned canonical
// `VqRegion` (individually `Box`-owned inside `FvarTable.masters`,
// disposed once by `FvarTable`'s own `Drop` at final `Font` teardown). That
// pointer's read-back site (OTF-write time, well after the borrow that
// registered it ended) was the last genuine raw-pointer wall in this
// crate -- Stage M-28 retires it by making `region` an `Rc<VqRegion>`
// instead: `FvarMaster.region` became `Rc<VqRegion>` in Stage M-27, and
// every `VqSegmentDelta` now holds its own `Rc::clone` of that same
// allocation (a refcount bump, not a copy) rather than a raw alias into
// it. A `RegionKey`-owned-value field was considered and rejected: it
// risked silently changing sort order, since `vqs_compare`/
// `vqs_compatible` below sort by true `f64` numeric value via
// `vq_compare_region`, while `RegionKey` compares IEEE-754 bit patterns
// (built for `Eq`/`Hash`, not order) -- this project treats output byte
// order as sacred. An index-into-`masters` approach was also rejected:
// sorting needs real region content, not just an index, which would have
// needed `&FvarTable` threaded through `consolidate.rs`/`table/cff.rs`/
// `libcff/charstring_il.rs`, well beyond this pointer's actual reach.
// `Rc::clone` sidesteps both problems (still compares the same `VqRegion`
// content through the same `vq_compare_region`, no lifetime threading
// needed) -- this crate has zero threading (no `Send`/`Sync` bounds
// anywhere), so `Rc`, not `Arc`, is the right tool. A region that turns
// out to be a content-duplicate during registration is freed immediately,
// before any `Rc::clone` of it is ever handed to a `VqSegmentDelta` -- see
// `fvar_register_region`'s own comment.
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
// `VV` は `Vec<Pos>`（`vf/vv.rs`）。要素(`Pos`)は所有物なしのプリミティブなので
// 専用のvtable/dup関数は不要——生存していた `.init`/`.push`/`.shrink_to_fit`/
// `.dispose` は呼び出し側(`table/fvar.rs`)で直接 `Vec` のメソッドに置き換えた。
// `.copy`/`.create`/`.free`/`.init_n`/`.neutral`（`create_neutral_vv`)は
// crate全体で一度も呼ばれておらず削除。
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
            // The original only copied `.quantity`/`.region`, leaving
            // `.touched` at whatever bits already sat in `dst`'s memory --
            // meaningful when `dst` was already a `Delta` (preserved here
            // the same way), undefined when it wasn't (every call site in
            // this crate passes a freshly-`Still`-initialized `dst`, so
            // this is the only case that actually occurs; `false` replaces
            // the old uninitialized read with a defined, safe value).
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
// Both take `&VqSegment` now, not `VqSegment` by value: `VqSegment` lost
// `Copy` in this stage (its `Delta` variant now holds an `Rc<VqRegion>`),
// and every call site here only ever reads its argument, so borrowing
// avoids an `Rc::clone` purely to satisfy a by-value parameter.
// `ad.region`/`bd.region` are plain `&Rc<VqRegion>` here, dereferenced
// through `Rc`'s own `Deref` (`&**` -- or just `&ad.region`/`vq_compare_region`
// taking `&VqRegion` and `Rc<T>: Deref<Target = T>` coercing) -- no
// `unsafe {}` needed, unlike the raw-pointer form this replaces.
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
    let mut j: usize = 1_usize;
    while j < shift.len() {
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
        j = j.wrapping_add(1);
    }
    shift.truncate(k.wrapping_add(1_usize));
}
pub(crate) fn vq_inplace_plus(a: &mut VQ, b: VQ) {
    a.kernel += b.kernel;
    let mut p: usize = 0_usize;
    while p < b.shift.len() {
        let k: VqSegment = b.shift[p].clone();
        if let VqSegment::Still(still) = k {
            a.kernel += still;
        } else {
            let mut s: VqSegment = VqSegment::Still(0.);
            vq_segment_copy(&mut s, &k);
            a.shift.push(s);
        }
        p = p.wrapping_add(1);
    }
    simplify_vq(a);
}
fn vq_inplace_scale(a: &mut VQ, b: Pos) {
    a.kernel *= b;
    let shift: &mut Vec<VqSegment> = &mut a.shift;
    let mut j: usize = 0_usize;
    while j < shift.len() {
        let s: &mut VqSegment = &mut shift[j];
        match s {
            VqSegment::Still(sv) => {
                *sv *= b;
            }
            VqSegment::Delta(sd) => {
                sd.quantity *= b;
            }
        }
        j = j.wrapping_add(1);
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
    let mut j: usize = 0_usize;
    while j < v.shift.len() {
        if let VqSegment::Still(still) = &v.shift[j] {
            result += *still;
        }
        j = j.wrapping_add(1);
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
    return vq_is_still(v.clone()) as i32 != 0
        && unsafe { fabs(vq_get_still(v) as ::core::ffi::c_double) } < err;
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
