#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
#![allow(improper_ctypes_definitions)] // VQ now owns a Vec; these extern "C" fns are internal-only (vtable dispatch, no real FFI boundary) -- goes away with the vtable/extern "C" cleanup, see rust/README.md
use libc::{fprintf, free, malloc, memcpy, memset};
unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::stdio::{stderr};
use crate::support::primitives::{Pos, Scale, TableId};
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_push, cvec_resize_to};

use crate::vf::region::{VqRegion};
use crate::vf::vv::{VV, VvVectorInterface};
use crate::vf::region::{vq_compare_region, vq_show_region};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut Pos) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut Pos, *const Pos) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut Pos) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> Pos>,
    pub dup: Option<unsafe extern "C" fn(Pos) -> Pos>,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VQSegType {
    Still = 0,
    Delta = 1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VqSegment {
    pub type_0: VQSegType,
    pub val: VqSegmentValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union VqSegmentValue {
    pub still: Pos,
    pub delta: VqSegmentDelta,
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
#[inline]
unsafe extern "C" fn pos_t_dup(src: Pos) -> Pos {
    let mut dst: Pos = 0.;
    pos_t_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn pos_t_copy(mut dst: *mut Pos, mut src: *const Pos) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Pos>() as usize,
    );
}
#[inline]
unsafe extern "C" fn pos_t_empty() -> Pos {
    let mut x: Pos = 0.;
    pos_t_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn pos_t_init(mut x: *mut Pos) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Pos>() as usize,
    );
}
pub static VQ_I_POS_T: PosElementInterface = {
    PosElementInterface {
        init: Some(pos_t_init as unsafe extern "C" fn(*mut Pos) -> ()),
        copy: Some(pos_t_copy as unsafe extern "C" fn(*mut Pos, *const Pos) -> ()),
        dispose: Some(pos_t_dispose as unsafe extern "C" fn(*mut Pos) -> ()),
        empty: Some(pos_t_empty),
        dup: Some(pos_t_dup as unsafe extern "C" fn(Pos) -> Pos),
    }
};
#[inline]
unsafe extern "C" fn pos_t_dispose(mut _x: *mut Pos) {}
#[inline]
unsafe fn vv_as_cvec(arr: *mut VV) -> *mut CVecRaw<Pos> {
    arr as *mut CVecRaw<Pos>
}
#[inline]
unsafe extern "C" fn vv_init(arr: *mut VV) {
    cvec_init(vv_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vv_grow_to(arr: *mut VV, target: usize) {
    cvec_grow_to(vv_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vv_fill(mut arr: *mut VV, mut n: usize) {
    while (*arr).length < n {
        let mut x: Pos = 0.;
        if VQ_I_POS_T.init.is_some() {
            VQ_I_POS_T.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Pos>() as usize,
            );
        }
        vv_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn vv_push(arr: *mut VV, elem: Pos) {
    cvec_push(vv_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vv_copy(mut dst: *mut VV, mut src: *const VV) {
    vv_init(dst);
    vv_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if VQ_I_POS_T.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            VQ_I_POS_T.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut Pos,
                (*src).items.offset(j as isize) as *mut Pos as *const Pos,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn vv_dispose(mut arr: *mut VV) {
    if arr.is_null() {
        return;
    }
    if VQ_I_POS_T.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            VQ_I_POS_T.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut Pos
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<Pos>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn vv_grow_to_n(arr: *mut VV, target: usize) {
    cvec_grow_to_n(vv_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vv_init_n(mut arr: *mut VV, mut n: usize) {
    vv_init(arr);
    vv_grow_to_n(arr, n);
    vv_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vv_free(mut x: *mut VV) {
    if x.is_null() {
        return;
    }
    vv_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vv_shrink_to_fit(mut arr: *mut VV) {
    vv_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vv_create() -> *mut VV {
    let mut x: *mut VV = malloc(::core::mem::size_of::<VV>() as usize) as *mut VV;
    vv_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn vv_resize_to(arr: *mut VV, target: usize) {
    cvec_resize_to(vv_as_cvec(arr), target);
}
unsafe extern "C" fn create_neutral_vv(mut dimensions: TableId) -> VV {
    let mut vv: VV = VV {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<Pos>(),
    };
    I_VV.init_n.expect("non-null function pointer")(&raw mut vv, dimensions as usize);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < dimensions as ::core::ffi::c_int {
        *vv.items.offset(j as isize) = 0 as ::core::ffi::c_int as Pos;
        j = j.wrapping_add(1);
    }
    return vv;
}
pub static I_VV: VvVectorInterface = {
    VvVectorInterface {
        init: Some(vv_init as unsafe extern "C" fn(*mut VV) -> ()),
        copy: Some(vv_copy as unsafe extern "C" fn(*mut VV, *const VV) -> ()),
        dispose: Some(vv_dispose as unsafe extern "C" fn(*mut VV) -> ()),
        create: Some(vv_create),
        free: Some(vv_free as unsafe extern "C" fn(*mut VV) -> ()),
        init_n: Some(vv_init_n as unsafe extern "C" fn(*mut VV, usize) -> ()),
        push: Some(vv_push as unsafe extern "C" fn(*mut VV, Pos) -> ()),
        shrink_to_fit: Some(vv_shrink_to_fit as unsafe extern "C" fn(*mut VV) -> ()),
        neutral: Some(create_neutral_vv as unsafe extern "C" fn(TableId) -> VV),
    }
};
#[inline]
unsafe extern "C" fn init_vq_segment(mut vqs: *mut VqSegment) {
    (*vqs).type_0 = VQSegType::Still;
    (*vqs).val.still = 0 as ::core::ffi::c_int as Pos;
}
#[inline]
unsafe extern "C" fn copy_vq_segment(mut dst: *mut VqSegment, mut src: *const VqSegment) {
    (*dst).type_0 = (*src).type_0;
    match (*dst).type_0 as ::core::ffi::c_uint {
        0 => {
            (*dst).val.still = (*src).val.still;
        }
        1 => {
            (*dst).val.delta.quantity = (*src).val.delta.quantity;
            (*dst).val.delta.region = (*src).val.delta.region;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn dispose_vq_segment(mut vqs: *mut VqSegment) {
    match (*vqs).type_0 as ::core::ffi::c_uint {
        1 | _ => {}
    }
    init_vq_segment(vqs);
}
#[inline]
unsafe extern "C" fn vq_segment_empty() -> VqSegment {
    let mut x: VqSegment = VqSegment {
        type_0: VQSegType::Still,
        val: VqSegmentValue { still: 0. },
    };
    vq_segment_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn vq_segment_copy(mut dst: *mut VqSegment, mut src: *const VqSegment) {
    copy_vq_segment(dst, src);
}
#[inline]
unsafe extern "C" fn vq_segment_dup(src: VqSegment) -> VqSegment {
    let mut dst: VqSegment = VqSegment {
        type_0: VQSegType::Still,
        val: VqSegmentValue { still: 0. },
    };
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
    let mut vqs: VqSegment = VqSegment {
        type_0: VQSegType::Still,
        val: VqSegmentValue { still: 0. },
    };
    VQ_I_SEGMENT.init.expect("non-null function pointer")(&raw mut vqs);
    vqs.val.still = x;
    return vqs;
}
unsafe extern "C" fn vqs_create_delta(mut delta: Pos, mut region: *mut VqRegion) -> VqSegment {
    let mut vqs: VqSegment = VqSegment {
        type_0: VQSegType::Still,
        val: VqSegmentValue { still: 0. },
    };
    VQ_I_SEGMENT.init.expect("non-null function pointer")(&raw mut vqs);
    vqs.type_0 = VQSegType::Delta;
    vqs.val.delta.quantity = delta;
    vqs.val.delta.region = region;
    return vqs;
}
unsafe extern "C" fn vqs_compare(a: VqSegment, b: VqSegment) -> ::core::ffi::c_int {
    if (a.type_0 as ::core::ffi::c_uint) < b.type_0 as ::core::ffi::c_uint {
        return -(1 as ::core::ffi::c_int);
    }
    if a.type_0 as ::core::ffi::c_uint > b.type_0 as ::core::ffi::c_uint {
        return 1 as ::core::ffi::c_int;
    }
    match a.type_0 as ::core::ffi::c_uint {
        0 => {
            if a.val.still < b.val.still {
                return -(1 as ::core::ffi::c_int);
            }
            if a.val.still > b.val.still {
                return 1 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        1 => {
            let mut vqrc: ::core::ffi::c_int =
                vq_compare_region(a.val.delta.region, b.val.delta.region);
            if vqrc != 0 {
                return vqrc;
            }
            if a.val.delta.quantity < b.val.delta.quantity {
                return -(1 as ::core::ffi::c_int);
            }
            if a.val.delta.quantity > b.val.delta.quantity {
                return 1 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    panic!("Reached end of non-void function without returning");
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
unsafe extern "C" fn show_vqs(x: VqSegment) {
    match x.type_0 as ::core::ffi::c_uint {
        0 => {
            fprintf(
                stderr,
                b"%g\0" as *const u8 as *const ::core::ffi::c_char,
                x.val.still,
            );
            return;
        }
        1 => {
            fprintf(
                stderr,
                b"{%g%s\0" as *const u8 as *const ::core::ffi::c_char,
                x.val.delta.quantity,
                if x.val.delta.touched as ::core::ffi::c_int != 0 {
                    b" \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"* \0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            vq_show_region(x.val.delta.region);
            fprintf(stderr, b"}\n\0" as *const u8 as *const ::core::ffi::c_char);
            return;
        }
        _ => {}
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
unsafe extern "C" fn vqs_compatible(a: VqSegment, b: VqSegment) -> bool {
    if a.type_0 as ::core::ffi::c_uint != b.type_0 as ::core::ffi::c_uint {
        return false;
    }
    match a.type_0 as ::core::ffi::c_uint {
        0 => return true,
        1 => {
            return 0 as ::core::ffi::c_int
                == vq_compare_region(a.val.delta.region, b.val.delta.region);
        }
        _ => {}
    }
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn simplify_vq(mut x: *mut VQ) {
    if (*x).shift.is_empty() {
        return;
    }
    let shift: &mut Vec<VqSegment> = &mut (*x).shift;
    shift.sort_by(|a, b| vqs_compare(*a, *b).cmp(&(0 as ::core::ffi::c_int)));
    let mut k: usize = 0 as usize;
    let mut j: usize = 1 as usize;
    while j < shift.len() {
        if vqs_compatible(shift[k], shift[j]) {
            match shift[k].type_0 as ::core::ffi::c_uint {
                0 => {
                    shift[k].val.still += shift[j].val.still;
                }
                1 => {
                    shift[k].val.delta.quantity += shift[j].val.delta.quantity;
                }
                _ => {}
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
        if k.type_0 == VQSegType::Still
        {
            (*a).kernel += k.val.still;
        } else {
            let mut s: VqSegment = VqSegment {
                type_0: VQSegType::Still,
                val: VqSegmentValue { still: 0. },
            };
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
        match s.type_0 as ::core::ffi::c_uint {
            0 => {
                s.val.still *= b;
            }
            1 => {
                s.val.delta.quantity *= b;
            }
            _ => {}
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
unsafe extern "C" fn show_vq(x: VQ) {
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
        match v.shift[j].type_0 as ::core::ffi::c_uint {
            0 => {
                result += v.shift[j].val.still;
            }
            _ => {}
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
        match v.shift[j].type_0 as ::core::ffi::c_uint {
            0 => {}
            _ => return false,
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
    let mut nudge: VqSegment = VqSegment {
        type_0: VQSegType::Still,
        val: VqSegmentValue { still: 0. },
    };
    nudge.type_0 = VQSegType::Delta;
    nudge.val.delta.region = r;
    nudge.val.delta.touched = touched;
    nudge.val.delta.quantity = quantity;
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
        assert_eq!(VQSegType::Still as u8, 0);
        assert_eq!(VQSegType::Delta as u8, 1);
        assert_eq!(::core::mem::size_of::<VQSegType>(), 4);
    }
}
