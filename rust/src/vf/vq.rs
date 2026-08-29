#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::primitives::{Pos, Scale};

use crate::vf::region::VqRegion;
use crate::vf::region::vq_compare_region;
// Was a C-shaped `struct { type_0: VQSegType, val: union { still: Pos,
// delta: VqSegmentDelta } }` -- the same "tag fully determines the live
// union arm" shape already converted elsewhere in the crate (`CffEncoding`,
// `ChainingSubtable`, etc.). Unlike those, every field here is `Copy` (no
// owned heap data -- `region: *const VqRegion` is a borrowed pointer), so
// the new enum stays `Copy` too, with none of the `Drop`/ownership
// bookkeeping those other conversions needed.
#[derive(Copy, Clone)]
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
            VqSegment::Delta(d) => *d,
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
// Stage 7-2-f closes out here: `region` stays a raw pointer rather than
// becoming an arena index, the last item this stage's plan named besides
// `BkCellValue::Ptr` (see `bk/bkblock.rs`'s Box化 comment for that one).
// Traced concretely, not assumed: `region` always ends up holding the
// *canonical* pointer `table/fvar.rs`'s `fvar_register_region` returns,
// which lives inside `FvarTable.masters` (an individually `Box`-owned
// `VqRegion` per `vf/region.rs`'s `vq_create_region`) and is disposed
// exactly once, by `FvarTable`'s own `Drop` impl, at final `Font` teardown
// -- the same "borrowed pointer into a longer-lived Box/collection-owned
// value, freed once, never revisited mid-algorithm" shape `Feature.lookups`/
// `LanguageSystem.features` (`table/otl.rs`) already rely on. A region that
// turns out to be a content-duplicate during registration is freed
// immediately, before its pointer is ever handed to a `VqSegmentDelta` --
// see `fvar_register_region`'s own comment.
#[derive(Copy, Clone)]
pub struct VqSegmentDelta {
    pub quantity: Pos,
    pub touched: bool,
    pub region: *const VqRegion,
}
#[derive(Clone)]
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
unsafe fn init_vq_segment(vqs: *mut VqSegment) {
    *vqs = VqSegment::Still(0 as ::core::ffi::c_int as Pos);
}
#[inline]
unsafe fn copy_vq_segment(dst: *mut VqSegment, src: *const VqSegment) {
    match *src {
        VqSegment::Still(v) => {
            *dst = VqSegment::Still(v);
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
                VqSegment::Delta(dd) => dd.touched,
                VqSegment::Still(_) => false,
            };
            *dst = VqSegment::Delta(VqSegmentDelta {
                quantity: sd.quantity,
                touched,
                region: sd.region,
            });
        }
    }
}
#[inline]
unsafe fn dispose_vq_segment(vqs: *mut VqSegment) {
    init_vq_segment(vqs);
}
#[inline]
unsafe fn vq_segment_copy(dst: *mut VqSegment, src: *const VqSegment) {
    copy_vq_segment(dst, src);
}
#[inline]
unsafe fn vq_segment_dispose(x: *mut VqSegment) {
    dispose_vq_segment(x);
}
unsafe fn vqs_compare(a: VqSegment, b: VqSegment) -> ::core::ffi::c_int {
    match (a, b) {
        (VqSegment::Still(_), VqSegment::Delta(_)) => -(1 as ::core::ffi::c_int),
        (VqSegment::Delta(_), VqSegment::Still(_)) => 1 as ::core::ffi::c_int,
        (VqSegment::Still(av), VqSegment::Still(bv)) => {
            if av < bv {
                return -(1 as ::core::ffi::c_int);
            }
            if av > bv {
                return 1 as ::core::ffi::c_int;
            }
            0 as ::core::ffi::c_int
        }
        (VqSegment::Delta(ad), VqSegment::Delta(bd)) => {
            let vqrc: ::core::ffi::c_int = vq_compare_region(ad.region, bd.region);
            if vqrc != 0 {
                return vqrc;
            }
            if ad.quantity < bd.quantity {
                return -(1 as ::core::ffi::c_int);
            }
            if ad.quantity > bd.quantity {
                return 1 as ::core::ffi::c_int;
            }
            0 as ::core::ffi::c_int
        }
    }
}
#[inline]
pub(crate) unsafe fn vq_init(x: *mut VQ) {
    (*x).kernel = 0 as ::core::ffi::c_int as Pos;
    (*x).shift = Vec::new();
}
#[inline]
pub(crate) unsafe fn vq_copy(dst: *mut VQ, src: *const VQ) {
    (*dst).kernel = (*src).kernel;
    (*dst).shift = (*src).shift.clone();
}
#[inline]
pub(crate) unsafe fn vq_dispose(x: *mut VQ) {
    (*x).kernel = 0 as ::core::ffi::c_int as Pos;
    (*x).shift = Vec::new();
}
#[inline]
pub(crate) unsafe fn vq_dup(src: VQ) -> VQ {
    let mut dst: VQ = VQ {
        kernel: 0.,
        shift: Vec::new(),
    };
    vq_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
pub(crate) unsafe fn vq_copy_replace(dst: *mut VQ, src: VQ) {
    vq_dispose(dst);
    vq_copy(dst, &raw const src);
}
#[inline]
pub(crate) unsafe fn vq_replace(dst: *mut VQ, src: VQ) {
    vq_dispose(dst);
    *dst = src;
}
pub(crate) unsafe fn vq_neutral() -> VQ {
    return vq_create_still(0 as ::core::ffi::c_int as Pos);
}
unsafe fn vqs_compatible(a: VqSegment, b: VqSegment) -> bool {
    match (a, b) {
        (VqSegment::Still(_), VqSegment::Still(_)) => true,
        (VqSegment::Delta(ad), VqSegment::Delta(bd)) => {
            0 as ::core::ffi::c_int == vq_compare_region(ad.region, bd.region)
        }
        _ => false,
    }
}
unsafe fn simplify_vq(x: *mut VQ) {
    if (*x).shift.is_empty() {
        return;
    }
    let shift: &mut Vec<VqSegment> = &mut (*x).shift;
    shift.sort_by(|a, b| vqs_compare(*a, *b).cmp(&(0 as ::core::ffi::c_int)));
    let mut k: usize = 0_usize;
    let mut j: usize = 1_usize;
    while j < shift.len() {
        if vqs_compatible(shift[k], shift[j]) {
            let other = shift[j];
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
            vq_segment_dispose(&raw mut shift[j]);
        } else {
            shift[k] = shift[j];
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    shift.truncate(k.wrapping_add(1_usize));
}
pub(crate) unsafe fn vq_inplace_plus(a: *mut VQ, b: VQ) {
    (*a).kernel += b.kernel;
    let mut p: usize = 0_usize;
    while p < b.shift.len() {
        let k: VqSegment = b.shift[p];
        if let VqSegment::Still(still) = k {
            (*a).kernel += still;
        } else {
            let mut s: VqSegment = VqSegment::Still(0.);
            vq_segment_copy(&raw mut s, &raw const k);
            (*a).shift.push(s);
        }
        p = p.wrapping_add(1);
    }
    simplify_vq(a);
}
unsafe fn vq_inplace_scale(a: *mut VQ, b: Pos) {
    (*a).kernel *= b;
    let shift: &mut Vec<VqSegment> = &mut (*a).shift;
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
unsafe fn vq_inplace_negate(a: *mut VQ) {
    vq_inplace_scale(a, -(1 as ::core::ffi::c_int) as Pos);
}
unsafe fn vq_negate(a: VQ) -> VQ {
    let mut result: VQ = VQ {
        kernel: 0.,
        shift: Vec::new(),
    };
    vq_copy(&raw mut result, &raw const a);
    vq_inplace_negate(&raw mut result);
    return result;
}
#[inline]
pub(crate) unsafe fn vq_minus(a: VQ, b: VQ) -> VQ {
    let mut result: VQ = vq_neutral();
    vq_inplace_plus(&raw mut result, a);
    vq_inplace_minus(&raw mut result, b);
    return result;
}
#[inline]
unsafe fn vq_inplace_minus(a: *mut VQ, b: VQ) {
    let mut tb: VQ = vq_negate(b);
    vq_inplace_plus(a, tb.clone());
    vq_dispose(&raw mut tb);
}
#[inline]
pub(crate) unsafe fn vq_inplace_plus_scale(a: *mut VQ, b: Pos, c: VQ) {
    let mut x: VQ = vq_scale(c, b);
    vq_inplace_plus(a, x.clone());
    vq_dispose(&raw mut x);
}
#[inline]
pub(crate) unsafe fn vq_scale(a: VQ, b: Pos) -> VQ {
    let mut result: VQ = VQ {
        kernel: 0.,
        shift: Vec::new(),
    };
    vq_copy(&raw mut result, &raw const a);
    vq_inplace_scale(&raw mut result, b);
    return result;
}
pub(crate) unsafe fn vq_compare(a: VQ, b: VQ) -> ::core::ffi::c_int {
    if a.shift.len() < b.shift.len() {
        return -(1 as ::core::ffi::c_int);
    }
    if a.shift.len() > b.shift.len() {
        return 1 as ::core::ffi::c_int;
    }
    let mut j: usize = 0_usize;
    while j < a.shift.len() {
        let cr: ::core::ffi::c_int = vqs_compare(a.shift[j], b.shift[j]);
        if cr != 0 {
            return cr;
        }
        j = j.wrapping_add(1);
    }
    return (a.kernel - b.kernel) as ::core::ffi::c_int;
}
pub(crate) unsafe fn vq_get_still(v: VQ) -> Pos {
    let mut result: Pos = v.kernel;
    let mut j: usize = 0_usize;
    while j < v.shift.len() {
        if let VqSegment::Still(still) = v.shift[j] {
            result += still;
        }
        j = j.wrapping_add(1);
    }
    return result;
}
pub(crate) unsafe fn vq_create_still(x: Pos) -> VQ {
    let mut vq: VQ = VQ {
        kernel: 0.,
        shift: Vec::new(),
    };
    vq_init(&raw mut vq);
    vq.kernel = x;
    return vq;
}
pub(crate) unsafe fn vq_is_still(v: VQ) -> bool {
    let mut j: usize = 0_usize;
    while j < v.shift.len() {
        if !matches!(v.shift[j], VqSegment::Still(_)) {
            return false;
        }
        j = j.wrapping_add(1);
    }
    return true;
}
pub(crate) unsafe fn vq_is_zero(v: VQ, err: Pos) -> bool {
    return vq_is_still(v.clone()) as ::core::ffi::c_int != 0
        && fabs(vq_get_still(v) as ::core::ffi::c_double) < err;
}
pub(crate) unsafe fn vq_add_delta(
    v: *mut VQ,
    touched: bool,
    r: *const VqRegion,
    quantity: Pos,
) {
    if quantity == 0. {
        return;
    }
    let nudge = VqSegment::Delta(VqSegmentDelta {
        quantity,
        touched,
        region: r,
    });
    (*v).shift.push(nudge);
}
pub(crate) unsafe fn vq_point_linear_tfm(ax: VQ, a: Pos, x: VQ, b: Pos, y: VQ) -> VQ {
    let mut target_x: VQ = vq_dup(ax);
    vq_inplace_plus_scale(&raw mut target_x, a as Scale, x);
    vq_inplace_plus_scale(&raw mut target_x, b as Scale, y);
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
                region: ::core::ptr::null(),
            })
            .discriminant_byte(),
            1
        );
    }
}
