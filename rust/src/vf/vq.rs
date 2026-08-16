#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
#![allow(improper_ctypes_definitions)] // VQ now owns a Vec; these extern "C" fns are internal-only (vtable dispatch, no real FFI boundary) -- goes away with the vtable/extern "C" cleanup, see rust/README.md
use libc::{fprintf};
unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::stdio::{stderr};
use crate::support::primitives::{Pos, Scale};

use crate::vf::region::{VqRegion};
use crate::vf::region::{vq_compare_region, vq_show_region};
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VqSegmentDelta {
    pub quantity: Pos,
    pub touched: bool,
    pub region: *const VqRegion,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VqSegmentElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VqSegment) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VqSegment, *const VqSegment) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VqSegment) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> VqSegment>,
    pub dup: Option<unsafe extern "C" fn(VqSegment) -> VqSegment>,
    pub show: Option<unsafe extern "C" fn(VqSegment) -> ()>,
    pub equal: Option<unsafe extern "C" fn(VqSegment, VqSegment) -> bool>,
    pub compare: Option<unsafe extern "C" fn(VqSegment, VqSegment) -> ::core::ffi::c_int>,
    pub compare_ref:
        Option<unsafe extern "C" fn(*const VqSegment, *const VqSegment) -> ::core::ffi::c_int>,
    pub create_still: Option<unsafe extern "C" fn(Pos) -> VqSegment>,
    pub create_delta: Option<unsafe extern "C" fn(Pos, *mut VqRegion) -> VqSegment>,
}
#[derive(Clone)]
pub struct VQ {
    pub kernel: Pos,
    pub shift: Vec<VqSegment>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VqVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VQ, *const VQ) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub copy_replace: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> VQ>,
    pub dup: Option<unsafe extern "C" fn(VQ) -> VQ>,
    pub neutral: Option<unsafe extern "C" fn() -> VQ>,
    pub plus: Option<unsafe extern "C" fn(VQ, VQ) -> VQ>,
    pub inplace_plus: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub inplace_negate: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub negate: Option<unsafe extern "C" fn(VQ) -> VQ>,
    pub inplace_minus: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub minus: Option<unsafe extern "C" fn(VQ, VQ) -> VQ>,
    pub inplace_scale: Option<unsafe extern "C" fn(*mut VQ, Scale) -> ()>,
    pub inplace_plus_scale: Option<unsafe extern "C" fn(*mut VQ, Scale, VQ) -> ()>,
    pub scale: Option<unsafe extern "C" fn(VQ, Scale) -> VQ>,
    pub equal: Option<unsafe extern "C" fn(VQ, VQ) -> bool>,
    pub compare: Option<unsafe extern "C" fn(VQ, VQ) -> ::core::ffi::c_int>,
    pub compare_ref: Option<unsafe extern "C" fn(*const VQ, *const VQ) -> ::core::ffi::c_int>,
    pub show: Option<unsafe extern "C" fn(VQ) -> ()>,
    pub get_still: Option<unsafe extern "C" fn(VQ) -> Pos>,
    pub create_still: Option<unsafe extern "C" fn(Pos) -> VQ>,
    pub is_still: Option<unsafe extern "C" fn(VQ) -> bool>,
    pub is_zero: Option<unsafe extern "C" fn(VQ, Pos) -> bool>,
    pub point_linear_tfm: Option<unsafe extern "C" fn(VQ, Pos, VQ, Pos, VQ) -> VQ>,
    pub add_delta: Option<unsafe extern "C" fn(*mut VQ, bool, *const VqRegion, Pos) -> ()>,
}
// `VV` は `Vec<Pos>`（`vf/vv.rs`）。要素(`Pos`)は所有物なしのプリミティブなので
// 専用のvtable/dup関数は不要——生存していた `.init`/`.push`/`.shrink_to_fit`/
// `.dispose` は呼び出し側(`table/fvar.rs`)で直接 `Vec` のメソッドに置き換えた。
// `.copy`/`.create`/`.free`/`.init_n`/`.neutral`（`create_neutral_vv`)は
// crate全体で一度も呼ばれておらず削除。
#[inline]
unsafe fn init_vq_segment(mut vqs: *mut VqSegment) {
    *vqs = VqSegment::Still(0 as ::core::ffi::c_int as Pos);
}
#[inline]
unsafe fn copy_vq_segment(mut dst: *mut VqSegment, mut src: *const VqSegment) {
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
unsafe fn dispose_vq_segment(mut vqs: *mut VqSegment) {
    init_vq_segment(vqs);
}
#[inline]
unsafe extern "C" fn vq_segment_empty() -> VqSegment {
    let mut x: VqSegment = VqSegment::Still(0.);
    vq_segment_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn vq_segment_copy(mut dst: *mut VqSegment, mut src: *const VqSegment) {
    copy_vq_segment(dst, src);
}
#[inline]
unsafe extern "C" fn vq_segment_dup(src: VqSegment) -> VqSegment {
    let mut dst: VqSegment = VqSegment::Still(0.);
    vq_segment_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn vq_segment_init(mut x: *mut VqSegment) {
    init_vq_segment(x);
}
#[inline]
unsafe extern "C" fn vq_segment_dispose(mut x: *mut VqSegment) {
    dispose_vq_segment(x);
}
unsafe extern "C" fn vqs_create_still(mut x: Pos) -> VqSegment {
    let mut vqs: VqSegment = VqSegment::Still(0.);
    VQ_I_SEGMENT.init.expect("non-null function pointer")(&raw mut vqs);
    vqs = VqSegment::Still(x);
    return vqs;
}
unsafe extern "C" fn vqs_create_delta(mut delta: Pos, mut region: *mut VqRegion) -> VqSegment {
    let mut vqs: VqSegment = VqSegment::Still(0.);
    VQ_I_SEGMENT.init.expect("non-null function pointer")(&raw mut vqs);
    // `.touched` was never set here in the original either (confirmed dead
    // code: `vqs_create_delta`/`VQ_I_SEGMENT.create_delta` is assigned into
    // the vtable but never actually called anywhere in the crate) -- `false`
    // replaces the old uninitialized read.
    vqs = VqSegment::Delta(VqSegmentDelta {
        quantity: delta,
        touched: false,
        region,
    });
    return vqs;
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
unsafe extern "C" fn vq_segment_compare(a: VqSegment, b: VqSegment) -> ::core::ffi::c_int {
    return vqs_compare(a, b);
}
#[inline]
unsafe extern "C" fn vq_segment_compare_ref(
    mut a: *const VqSegment,
    mut b: *const VqSegment,
) -> ::core::ffi::c_int {
    return vqs_compare(*a, *b);
}
#[inline]
unsafe extern "C" fn vq_segment_equal(a: VqSegment, b: VqSegment) -> bool {
    return vqs_compare(a, b) == 0;
}
unsafe fn show_vqs(x: VqSegment) {
    match x {
        VqSegment::Still(still) => {
            fprintf(
                stderr,
                b"%g\0" as *const u8 as *const ::core::ffi::c_char,
                still,
            );
        }
        VqSegment::Delta(delta) => {
            fprintf(
                stderr,
                b"{%g%s\0" as *const u8 as *const ::core::ffi::c_char,
                delta.quantity,
                if delta.touched as ::core::ffi::c_int != 0 {
                    b" \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"* \0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            vq_show_region(delta.region);
            fprintf(stderr, b"}\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
    };
}
#[inline]
unsafe extern "C" fn vq_segment_show(a: VqSegment) {
    return show_vqs(a);
}
pub static VQ_I_SEGMENT: VqSegmentElementInterface = {
    VqSegmentElementInterface {
        init: Some(vq_segment_init as unsafe extern "C" fn(*mut VqSegment) -> ()),
        copy: Some(
            vq_segment_copy as unsafe extern "C" fn(*mut VqSegment, *const VqSegment) -> (),
        ),
        dispose: Some(vq_segment_dispose as unsafe extern "C" fn(*mut VqSegment) -> ()),
        empty: Some(vq_segment_empty),
        dup: Some(vq_segment_dup as unsafe extern "C" fn(VqSegment) -> VqSegment),
        show: Some(vq_segment_show as unsafe extern "C" fn(VqSegment) -> ()),
        equal: Some(vq_segment_equal as unsafe extern "C" fn(VqSegment, VqSegment) -> bool),
        compare: Some(
            vq_segment_compare
                as unsafe extern "C" fn(VqSegment, VqSegment) -> ::core::ffi::c_int,
        ),
        compare_ref: Some(
            vq_segment_compare_ref
                as unsafe extern "C" fn(*const VqSegment, *const VqSegment) -> ::core::ffi::c_int,
        ),
        create_still: Some(vqs_create_still as unsafe extern "C" fn(Pos) -> VqSegment),
        create_delta: Some(
            vqs_create_delta as unsafe extern "C" fn(Pos, *mut VqRegion) -> VqSegment,
        ),
    }
};
#[inline]
unsafe extern "C" fn vq_init(mut x: *mut VQ) {
    (*x).kernel = 0 as ::core::ffi::c_int as Pos;
    (*x).shift = Vec::new();
}
#[inline]
unsafe extern "C" fn vq_copy(mut dst: *mut VQ, mut src: *const VQ) {
    (*dst).kernel = (*src).kernel;
    (*dst).shift = (*src).shift.clone();
}
#[inline]
unsafe extern "C" fn vq_dispose(mut x: *mut VQ) {
    (*x).kernel = 0 as ::core::ffi::c_int as Pos;
    (*x).shift = Vec::new();
}
#[inline]
unsafe extern "C" fn vq_dup(src: VQ) -> VQ {
    let mut dst: VQ = VQ {
        kernel: 0.,
        shift: Vec::new(),
    };
    vq_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn vq_empty() -> VQ {
    let mut x: VQ = VQ {
        kernel: 0.,
        shift: Vec::new(),
    };
    vq_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn vq_copy_replace(mut dst: *mut VQ, src: VQ) {
    vq_dispose(dst);
    vq_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vq_replace(mut dst: *mut VQ, src: VQ) {
    vq_dispose(dst);
    *dst = src;
}
unsafe extern "C" fn vq_neutral() -> VQ {
    return I_VQ.create_still.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
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
unsafe fn simplify_vq(mut x: *mut VQ) {
    if (*x).shift.is_empty() {
        return;
    }
    let shift: &mut Vec<VqSegment> = &mut (*x).shift;
    shift.sort_by(|a, b| vqs_compare(*a, *b).cmp(&(0 as ::core::ffi::c_int)));
    let mut k: usize = 0 as usize;
    let mut j: usize = 1 as usize;
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
            VQ_I_SEGMENT.dispose.expect("non-null function pointer")(&raw mut shift[j]);
        } else {
            shift[k] = shift[j];
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    shift.truncate(k.wrapping_add(1 as usize));
}
unsafe extern "C" fn vq_inplace_plus(mut a: *mut VQ, b: VQ) {
    (*a).kernel += b.kernel;
    let mut p: usize = 0 as usize;
    while p < b.shift.len() {
        let k: VqSegment = b.shift[p];
        if let VqSegment::Still(still) = k
        {
            (*a).kernel += still;
        } else {
            let mut s: VqSegment = VqSegment::Still(0.);
            VQ_I_SEGMENT.copy.expect("non-null function pointer")(&raw mut s, &raw const k);
            (*a).shift.push(s);
        }
        p = p.wrapping_add(1);
    }
    simplify_vq(a);
}
#[inline]
unsafe extern "C" fn vq_plus(a: VQ, b: VQ) -> VQ {
    let mut result: VQ = vq_neutral();
    vq_inplace_plus(&raw mut result, a);
    vq_inplace_plus(&raw mut result, b);
    return result;
}
unsafe extern "C" fn vq_inplace_scale(mut a: *mut VQ, mut b: Pos) {
    (*a).kernel *= b;
    let shift: &mut Vec<VqSegment> = &mut (*a).shift;
    let mut j: usize = 0 as usize;
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
unsafe extern "C" fn vq_inplace_negate(mut a: *mut VQ) {
    vq_inplace_scale(a, -(1 as ::core::ffi::c_int) as Pos);
}
#[inline]
unsafe extern "C" fn vq_minus(a: VQ, b: VQ) -> VQ {
    let mut result: VQ = vq_neutral();
    vq_inplace_plus(&raw mut result, a);
    vq_inplace_minus(&raw mut result, b);
    return result;
}
#[inline]
unsafe extern "C" fn vq_inplace_minus(mut a: *mut VQ, b: VQ) {
    let mut tb: VQ = vq_negate(b);
    vq_inplace_plus(a, tb.clone());
    vq_dispose(&raw mut tb);
}
#[inline]
unsafe extern "C" fn vq_negate(a: VQ) -> VQ {
    let mut result: VQ = VQ {
        kernel: 0.,
        shift: Vec::new(),
    };
    vq_copy(&raw mut result, &raw const a);
    vq_inplace_negate(&raw mut result);
    return result;
}
#[inline]
unsafe extern "C" fn vq_inplace_plus_scale(mut a: *mut VQ, mut b: Pos, c: VQ) {
    let mut x: VQ = vq_scale(c, b);
    vq_inplace_plus(a, x.clone());
    vq_dispose(&raw mut x);
}
#[inline]
unsafe extern "C" fn vq_scale(a: VQ, mut b: Pos) -> VQ {
    let mut result: VQ = VQ {
        kernel: 0.,
        shift: Vec::new(),
    };
    vq_copy(&raw mut result, &raw const a);
    vq_inplace_scale(&raw mut result, b);
    return result;
}
unsafe extern "C" fn vq_compare(a: VQ, b: VQ) -> ::core::ffi::c_int {
    if a.shift.len() < b.shift.len() {
        return -(1 as ::core::ffi::c_int);
    }
    if a.shift.len() > b.shift.len() {
        return 1 as ::core::ffi::c_int;
    }
    let mut j: usize = 0 as usize;
    while j < a.shift.len() {
        let mut cr: ::core::ffi::c_int = vqs_compare(a.shift[j], b.shift[j]);
        if cr != 0 {
            return cr;
        }
        j = j.wrapping_add(1);
    }
    return (a.kernel - b.kernel) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn vq_compare_ref(mut a: *const VQ, mut b: *const VQ) -> ::core::ffi::c_int {
    return vq_compare((*a).clone(), (*b).clone());
}
#[inline]
unsafe extern "C" fn vq_equal(a: VQ, b: VQ) -> bool {
    return vq_compare(a, b) == 0;
}
unsafe fn show_vq(x: VQ) {
    fprintf(
        stderr,
        b"%g + {\0" as *const u8 as *const ::core::ffi::c_char,
        x.kernel,
    );
    let mut j: usize = 0 as usize;
    while j < x.shift.len() {
        if j != 0 {
            fprintf(stderr, b" \0" as *const u8 as *const ::core::ffi::c_char);
        }
        VQ_I_SEGMENT.show.expect("non-null function pointer")(x.shift[j]);
        j = j.wrapping_add(1);
    }
    fprintf(stderr, b"}\n\0" as *const u8 as *const ::core::ffi::c_char);
}
#[inline]
unsafe extern "C" fn vq_show(a: VQ) {
    return show_vq(a);
}
unsafe extern "C" fn vq_get_still(v: VQ) -> Pos {
    let mut result: Pos = v.kernel;
    let mut j: usize = 0 as usize;
    while j < v.shift.len() {
        if let VqSegment::Still(still) = v.shift[j] {
            result += still;
        }
        j = j.wrapping_add(1);
    }
    return result;
}
unsafe extern "C" fn vq_create_still(mut x: Pos) -> VQ {
    let mut vq: VQ = VQ {
        kernel: 0.,
        shift: Vec::new(),
    };
    I_VQ.init.expect("non-null function pointer")(&raw mut vq);
    vq.kernel = x;
    return vq;
}
unsafe extern "C" fn vq_is_still(v: VQ) -> bool {
    let mut j: usize = 0 as usize;
    while j < v.shift.len() {
        if !matches!(v.shift[j], VqSegment::Still(_)) {
            return false;
        }
        j = j.wrapping_add(1);
    }
    return true;
}
unsafe extern "C" fn vq_is_zero(v: VQ, err: Pos) -> bool {
    return vq_is_still(v.clone()) as ::core::ffi::c_int != 0
        && fabs(vq_get_still(v) as ::core::ffi::c_double) < err;
}
unsafe extern "C" fn vq_add_delta(
    mut v: *mut VQ,
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
unsafe extern "C" fn vq_point_linear_tfm(ax: VQ, mut a: Pos, x: VQ, mut b: Pos, y: VQ) -> VQ {
    let mut target_x: VQ = I_VQ.dup.expect("non-null function pointer")(ax);
    I_VQ.inplace_plus_scale.expect("non-null function pointer")(&raw mut target_x, a as Scale, x);
    I_VQ.inplace_plus_scale.expect("non-null function pointer")(&raw mut target_x, b as Scale, y);
    return target_x;
}
pub static I_VQ: VqVectorInterface = {
    VqVectorInterface {
        init: Some(vq_init as unsafe extern "C" fn(*mut VQ) -> ()),
        copy: Some(vq_copy as unsafe extern "C" fn(*mut VQ, *const VQ) -> ()),
        dispose: Some(vq_dispose as unsafe extern "C" fn(*mut VQ) -> ()),
        replace: Some(vq_replace as unsafe extern "C" fn(*mut VQ, VQ) -> ()),
        copy_replace: Some(vq_copy_replace as unsafe extern "C" fn(*mut VQ, VQ) -> ()),
        empty: Some(vq_empty),
        dup: Some(vq_dup as unsafe extern "C" fn(VQ) -> VQ),
        neutral: Some(vq_neutral),
        plus: Some(vq_plus as unsafe extern "C" fn(VQ, VQ) -> VQ),
        inplace_plus: Some(vq_inplace_plus as unsafe extern "C" fn(*mut VQ, VQ) -> ()),
        inplace_negate: Some(vq_inplace_negate as unsafe extern "C" fn(*mut VQ) -> ()),
        negate: Some(vq_negate as unsafe extern "C" fn(VQ) -> VQ),
        inplace_minus: Some(vq_inplace_minus as unsafe extern "C" fn(*mut VQ, VQ) -> ()),
        minus: Some(vq_minus as unsafe extern "C" fn(VQ, VQ) -> VQ),
        inplace_scale: Some(vq_inplace_scale as unsafe extern "C" fn(*mut VQ, Pos) -> ()),
        inplace_plus_scale: Some(
            vq_inplace_plus_scale as unsafe extern "C" fn(*mut VQ, Pos, VQ) -> (),
        ),
        scale: Some(vq_scale as unsafe extern "C" fn(VQ, Pos) -> VQ),
        equal: Some(vq_equal as unsafe extern "C" fn(VQ, VQ) -> bool),
        compare: Some(vq_compare as unsafe extern "C" fn(VQ, VQ) -> ::core::ffi::c_int),
        compare_ref: Some(
            vq_compare_ref as unsafe extern "C" fn(*const VQ, *const VQ) -> ::core::ffi::c_int,
        ),
        show: Some(vq_show as unsafe extern "C" fn(VQ) -> ()),
        get_still: Some(vq_get_still as unsafe extern "C" fn(VQ) -> Pos),
        create_still: Some(vq_create_still as unsafe extern "C" fn(Pos) -> VQ),
        is_still: Some(vq_is_still as unsafe extern "C" fn(VQ) -> bool),
        is_zero: Some(vq_is_zero as unsafe extern "C" fn(VQ, Pos) -> bool),
        point_linear_tfm: Some(
            vq_point_linear_tfm as unsafe extern "C" fn(VQ, Pos, VQ, Pos, VQ) -> VQ,
        ),
        add_delta: Some(
            vq_add_delta as unsafe extern "C" fn(*mut VQ, bool, *const VqRegion, Pos) -> (),
        ),
    }
};

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
