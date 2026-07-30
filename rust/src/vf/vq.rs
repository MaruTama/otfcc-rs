#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{fprintf, free, malloc, memcpy, memset, qsort};
unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::stdio::{stderr};
use crate::support::primitives::{Pos, Scale, TableId};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};

use crate::vf::region::{VqRegion};
use crate::vf::vv::{VV, VvVectorInterface};
use crate::support::{ComparFn};
use crate::vf::region::{vq_compareRegion, vq_showRegion};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut Pos) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut Pos, *const Pos) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut Pos, *mut Pos) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut Pos) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut Pos, Pos) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut Pos, Pos) -> ()>,
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
    pub move_0: Option<unsafe extern "C" fn(*mut VqSegment, *mut VqSegment) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VqSegment) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VqSegment, VqSegment) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VqSegment, VqSegment) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> VqSegment>,
    pub dup: Option<unsafe extern "C" fn(VqSegment) -> VqSegment>,
    pub show: Option<unsafe extern "C" fn(VqSegment) -> ()>,
    pub equal: Option<unsafe extern "C" fn(VqSegment, VqSegment) -> bool>,
    pub compare: Option<unsafe extern "C" fn(VqSegment, VqSegment) -> ::core::ffi::c_int>,
    pub compareRef:
        Option<unsafe extern "C" fn(*const VqSegment, *const VqSegment) -> ::core::ffi::c_int>,
    pub createStill: Option<unsafe extern "C" fn(Pos) -> VqSegment>,
    pub createDelta: Option<unsafe extern "C" fn(Pos, *mut VqRegion) -> VqSegment>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VqSegList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut VqSegment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VqSegListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut VqSegList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VqSegList, *const VqSegList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VqSegList, *mut VqSegList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VqSegList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VqSegList, VqSegList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VqSegList, VqSegList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VqSegList>,
    pub free: Option<unsafe extern "C" fn(*mut VqSegList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut VqSegList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut VqSegList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut VqSegList>,
    pub fill: Option<unsafe extern "C" fn(*mut VqSegList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut VqSegList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VqSegList, VqSegment) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut VqSegList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut VqSegList) -> VqSegment>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut VqSegList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut VqSegList,
            Option<unsafe extern "C" fn(*const VqSegment, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut VqSegList,
            Option<unsafe extern "C" fn(*const VqSegment, *const VqSegment) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VQ {
    pub kernel: Pos,
    pub shift: VqSegList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VqVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VQ, *const VQ) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VQ, *mut VQ) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> VQ>,
    pub dup: Option<unsafe extern "C" fn(VQ) -> VQ>,
    pub neutral: Option<unsafe extern "C" fn() -> VQ>,
    pub plus: Option<unsafe extern "C" fn(VQ, VQ) -> VQ>,
    pub inplacePlus: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub inplaceNegate: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub negate: Option<unsafe extern "C" fn(VQ) -> VQ>,
    pub inplaceMinus: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub minus: Option<unsafe extern "C" fn(VQ, VQ) -> VQ>,
    pub inplaceScale: Option<unsafe extern "C" fn(*mut VQ, Scale) -> ()>,
    pub inplacePlusScale: Option<unsafe extern "C" fn(*mut VQ, Scale, VQ) -> ()>,
    pub scale: Option<unsafe extern "C" fn(VQ, Scale) -> VQ>,
    pub equal: Option<unsafe extern "C" fn(VQ, VQ) -> bool>,
    pub compare: Option<unsafe extern "C" fn(VQ, VQ) -> ::core::ffi::c_int>,
    pub compareRef: Option<unsafe extern "C" fn(*const VQ, *const VQ) -> ::core::ffi::c_int>,
    pub show: Option<unsafe extern "C" fn(VQ) -> ()>,
    pub getStill: Option<unsafe extern "C" fn(VQ) -> Pos>,
    pub createStill: Option<unsafe extern "C" fn(Pos) -> VQ>,
    pub isStill: Option<unsafe extern "C" fn(VQ) -> bool>,
    pub isZero: Option<unsafe extern "C" fn(VQ, Pos) -> bool>,
    pub pointLinearTfm: Option<unsafe extern "C" fn(VQ, Pos, VQ, Pos, VQ) -> VQ>,
    pub addDelta: Option<unsafe extern "C" fn(*mut VQ, bool, *const VqRegion, Pos) -> ()>,
}
#[inline]
unsafe extern "C" fn pos_t_replace(mut dst: *mut Pos, src: Pos) {
    pos_t_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Pos>() as usize,
    );
}
#[inline]
unsafe extern "C" fn pos_t_move(mut dst: *mut Pos, mut src: *mut Pos) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Pos>() as usize,
    );
    pos_t_init(src);
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
        move_0: Some(pos_t_move as unsafe extern "C" fn(*mut Pos, *mut Pos) -> ()),
        dispose: Some(pos_t_dispose as unsafe extern "C" fn(*mut Pos) -> ()),
        replace: Some(pos_t_replace as unsafe extern "C" fn(*mut Pos, Pos) -> ()),
        copyReplace: Some(pos_t_copyReplace as unsafe extern "C" fn(*mut Pos, Pos) -> ()),
        empty: Some(pos_t_empty),
        dup: Some(pos_t_dup as unsafe extern "C" fn(Pos) -> Pos),
    }
};
#[inline]
unsafe extern "C" fn pos_t_copyReplace(mut dst: *mut Pos, src: Pos) {
    pos_t_dispose(dst);
    pos_t_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn pos_t_dispose(mut _x: *mut Pos) {}
#[inline]
unsafe extern "C" fn VV_createN(mut n: usize) -> *mut VV {
    let mut t: *mut VV = malloc(::core::mem::size_of::<VV>() as usize) as *mut VV;
    VV_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn VV_move(dst: *mut VV, src: *mut VV) {
    cvec_move(VV_as_cvec(dst), VV_as_cvec(src));
}
#[inline]
unsafe fn VV_as_cvec(arr: *mut VV) -> *mut CVecRaw<Pos> {
    arr as *mut CVecRaw<Pos>
}
#[inline]
unsafe extern "C" fn VV_init(arr: *mut VV) {
    cvec_init(VV_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn VV_growTo(arr: *mut VV, target: usize) {
    cvec_grow_to(VV_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn VV_filterEnv(
    mut arr: *mut VV,
    mut fn_0: Option<unsafe extern "C" fn(*const Pos, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut Pos,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if VQ_I_POS_T.dispose.is_some() {
                VQ_I_POS_T.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut Pos,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn VV_disposeItem(mut arr: *mut VV, mut n: usize) {
    if VQ_I_POS_T.dispose.is_some() {
        VQ_I_POS_T.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut Pos
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn VV_sort(
    mut arr: *mut VV,
    mut fn_0: Option<unsafe extern "C" fn(*const Pos, *const Pos) -> ::core::ffi::c_int>,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<Pos>() as usize,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const Pos, *const Pos) -> ::core::ffi::c_int>,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn VV_fill(mut arr: *mut VV, mut n: usize) {
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
        VV_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn VV_push(arr: *mut VV, elem: Pos) {
    cvec_push(VV_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn VV_grow(arr: *mut VV) {
    cvec_grow(VV_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn VV_pop(arr: *mut VV) -> Pos {
    cvec_pop(VV_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn VV_copy(mut dst: *mut VV, mut src: *const VV) {
    VV_init(dst);
    VV_growTo(dst, (*src).length);
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
unsafe extern "C" fn VV_copyReplace(mut dst: *mut VV, src: VV) {
    VV_dispose(dst);
    VV_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn VV_dispose(mut arr: *mut VV) {
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
unsafe extern "C" fn VV_replace(mut dst: *mut VV, src: VV) {
    VV_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VV>() as usize,
    );
}
#[inline]
unsafe extern "C" fn VV_initCapN(mut arr: *mut VV, mut n: usize) {
    VV_init(arr);
    VV_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn VV_growToN(arr: *mut VV, target: usize) {
    cvec_grow_to_n(VV_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn VV_initN(mut arr: *mut VV, mut n: usize) {
    VV_init(arr);
    VV_growToN(arr, n);
    VV_fill(arr, n);
}
#[inline]
unsafe extern "C" fn VV_free(mut x: *mut VV) {
    if x.is_null() {
        return;
    }
    VV_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn VV_shrinkToFit(mut arr: *mut VV) {
    VV_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn VV_create() -> *mut VV {
    let mut x: *mut VV = malloc(::core::mem::size_of::<VV>() as usize) as *mut VV;
    VV_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn VV_resizeTo(arr: *mut VV, target: usize) {
    cvec_resize_to(VV_as_cvec(arr), target);
}
unsafe extern "C" fn createNeutralVV(mut dimensions: TableId) -> VV {
    let mut vv: VV = VV {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<Pos>(),
    };
    I_VV.initN.expect("non-null function pointer")(&raw mut vv, dimensions as usize);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < dimensions as ::core::ffi::c_int {
        *vv.items.offset(j as isize) = 0 as ::core::ffi::c_int as Pos;
        j = j.wrapping_add(1);
    }
    return vv;
}
pub static I_VV: VvVectorInterface = {
    VvVectorInterface {
        init: Some(VV_init as unsafe extern "C" fn(*mut VV) -> ()),
        copy: Some(VV_copy as unsafe extern "C" fn(*mut VV, *const VV) -> ()),
        move_0: Some(VV_move as unsafe extern "C" fn(*mut VV, *mut VV) -> ()),
        dispose: Some(VV_dispose as unsafe extern "C" fn(*mut VV) -> ()),
        replace: Some(VV_replace as unsafe extern "C" fn(*mut VV, VV) -> ()),
        copyReplace: Some(VV_copyReplace as unsafe extern "C" fn(*mut VV, VV) -> ()),
        create: Some(VV_create),
        free: Some(VV_free as unsafe extern "C" fn(*mut VV) -> ()),
        initN: Some(VV_initN as unsafe extern "C" fn(*mut VV, usize) -> ()),
        initCapN: Some(VV_initCapN as unsafe extern "C" fn(*mut VV, usize) -> ()),
        createN: Some(VV_createN as unsafe extern "C" fn(usize) -> *mut VV),
        fill: Some(VV_fill as unsafe extern "C" fn(*mut VV, usize) -> ()),
        clear: Some(VV_dispose as unsafe extern "C" fn(*mut VV) -> ()),
        push: Some(VV_push as unsafe extern "C" fn(*mut VV, Pos) -> ()),
        shrinkToFit: Some(VV_shrinkToFit as unsafe extern "C" fn(*mut VV) -> ()),
        pop: Some(VV_pop as unsafe extern "C" fn(*mut VV) -> Pos),
        disposeItem: Some(VV_disposeItem as unsafe extern "C" fn(*mut VV, usize) -> ()),
        filterEnv: Some(
            VV_filterEnv
                as unsafe extern "C" fn(
                    *mut VV,
                    Option<unsafe extern "C" fn(*const Pos, *mut ::core::ffi::c_void) -> bool>,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            VV_sort
                as unsafe extern "C" fn(
                    *mut VV,
                    Option<unsafe extern "C" fn(*const Pos, *const Pos) -> ::core::ffi::c_int>,
                ) -> (),
        ),
        neutral: Some(createNeutralVV as unsafe extern "C" fn(TableId) -> VV),
    }
};
#[inline]
unsafe extern "C" fn initVQSegment(mut vqs: *mut VqSegment) {
    (*vqs).type_0 = VQSegType::Still;
    (*vqs).val.still = 0 as ::core::ffi::c_int as Pos;
}
#[inline]
unsafe extern "C" fn copyVQSegment(mut dst: *mut VqSegment, mut src: *const VqSegment) {
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
unsafe extern "C" fn disposeVQSegment(mut vqs: *mut VqSegment) {
    match (*vqs).type_0 as ::core::ffi::c_uint {
        1 | _ => {}
    }
    initVQSegment(vqs);
}
#[inline]
unsafe extern "C" fn vq_Segment_replace(mut dst: *mut VqSegment, src: VqSegment) {
    vq_Segment_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VqSegment>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vq_Segment_copyReplace(mut dst: *mut VqSegment, src: VqSegment) {
    vq_Segment_dispose(dst);
    vq_Segment_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vq_Segment_empty() -> VqSegment {
    let mut x: VqSegment = VqSegment {
        type_0: VQSegType::Still,
        val: VqSegmentValue { still: 0. },
    };
    vq_Segment_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn vq_Segment_copy(mut dst: *mut VqSegment, mut src: *const VqSegment) {
    copyVQSegment(dst, src);
}
#[inline]
unsafe extern "C" fn vq_Segment_dup(src: VqSegment) -> VqSegment {
    let mut dst: VqSegment = VqSegment {
        type_0: VQSegType::Still,
        val: VqSegmentValue { still: 0. },
    };
    vq_Segment_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn vq_Segment_init(mut x: *mut VqSegment) {
    initVQSegment(x);
}
#[inline]
unsafe extern "C" fn vq_Segment_dispose(mut x: *mut VqSegment) {
    disposeVQSegment(x);
}
#[inline]
unsafe extern "C" fn vq_Segment_move(mut dst: *mut VqSegment, mut src: *mut VqSegment) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VqSegment>() as usize,
    );
    vq_Segment_init(src);
}
unsafe extern "C" fn vqsCreateStill(mut x: Pos) -> VqSegment {
    let mut vqs: VqSegment = VqSegment {
        type_0: VQSegType::Still,
        val: VqSegmentValue { still: 0. },
    };
    VQ_I_SEGMENT.init.expect("non-null function pointer")(&raw mut vqs);
    vqs.val.still = x;
    return vqs;
}
unsafe extern "C" fn vqsCreateDelta(mut delta: Pos, mut region: *mut VqRegion) -> VqSegment {
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
unsafe extern "C" fn vqsCompare(a: VqSegment, b: VqSegment) -> ::core::ffi::c_int {
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
                vq_compareRegion(a.val.delta.region, b.val.delta.region);
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
unsafe extern "C" fn vq_Segment_compare(a: VqSegment, b: VqSegment) -> ::core::ffi::c_int {
    return vqsCompare(a, b);
}
#[inline]
unsafe extern "C" fn vq_Segment_compareRef(
    mut a: *const VqSegment,
    mut b: *const VqSegment,
) -> ::core::ffi::c_int {
    return vqsCompare(*a, *b);
}
#[inline]
unsafe extern "C" fn vq_Segment_equal(a: VqSegment, b: VqSegment) -> bool {
    return vqsCompare(a, b) == 0;
}
unsafe extern "C" fn showVQS(x: VqSegment) {
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
            vq_showRegion(x.val.delta.region);
            fprintf(stderr, b"}\n\0" as *const u8 as *const ::core::ffi::c_char);
            return;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn vq_Segment_show(a: VqSegment) {
    return showVQS(a);
}
pub static VQ_I_SEGMENT: VqSegmentElementInterface = {
    VqSegmentElementInterface {
        init: Some(vq_Segment_init as unsafe extern "C" fn(*mut VqSegment) -> ()),
        copy: Some(
            vq_Segment_copy as unsafe extern "C" fn(*mut VqSegment, *const VqSegment) -> (),
        ),
        move_0: Some(
            vq_Segment_move as unsafe extern "C" fn(*mut VqSegment, *mut VqSegment) -> (),
        ),
        dispose: Some(vq_Segment_dispose as unsafe extern "C" fn(*mut VqSegment) -> ()),
        replace: Some(
            vq_Segment_replace as unsafe extern "C" fn(*mut VqSegment, VqSegment) -> (),
        ),
        copyReplace: Some(
            vq_Segment_copyReplace as unsafe extern "C" fn(*mut VqSegment, VqSegment) -> (),
        ),
        empty: Some(vq_Segment_empty),
        dup: Some(vq_Segment_dup as unsafe extern "C" fn(VqSegment) -> VqSegment),
        show: Some(vq_Segment_show as unsafe extern "C" fn(VqSegment) -> ()),
        equal: Some(vq_Segment_equal as unsafe extern "C" fn(VqSegment, VqSegment) -> bool),
        compare: Some(
            vq_Segment_compare
                as unsafe extern "C" fn(VqSegment, VqSegment) -> ::core::ffi::c_int,
        ),
        compareRef: Some(
            vq_Segment_compareRef
                as unsafe extern "C" fn(*const VqSegment, *const VqSegment) -> ::core::ffi::c_int,
        ),
        createStill: Some(vqsCreateStill as unsafe extern "C" fn(Pos) -> VqSegment),
        createDelta: Some(
            vqsCreateDelta as unsafe extern "C" fn(Pos, *mut VqRegion) -> VqSegment,
        ),
    }
};
#[inline]
unsafe extern "C" fn vq_SegList_initN(mut arr: *mut VqSegList, mut n: usize) {
    vq_SegList_init(arr);
    vq_SegList_growToN(arr, n);
    vq_SegList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vq_SegList_shrinkToFit(mut arr: *mut VqSegList) {
    vq_SegList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vq_SegList_resizeTo(arr: *mut VqSegList, target: usize) {
    cvec_resize_to(vq_SegList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vq_SegList_move(dst: *mut VqSegList, src: *mut VqSegList) {
    cvec_move(vq_SegList_as_cvec(dst), vq_SegList_as_cvec(src));
}
#[inline]
unsafe fn vq_SegList_as_cvec(arr: *mut VqSegList) -> *mut CVecRaw<VqSegment> {
    arr as *mut CVecRaw<VqSegment>
}
#[inline]
unsafe extern "C" fn vq_SegList_init(arr: *mut VqSegList) {
    cvec_init(vq_SegList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vq_SegList_filterEnv(
    mut arr: *mut VqSegList,
    mut fn_0: Option<unsafe extern "C" fn(*const VqSegment, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut VqSegment,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if VQ_I_SEGMENT.dispose.is_some() {
                VQ_I_SEGMENT.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut VqSegment,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn vq_SegList_disposeItem(mut arr: *mut VqSegList, mut n: usize) {
    if VQ_I_SEGMENT.dispose.is_some() {
        VQ_I_SEGMENT.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut VqSegment
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn vq_SegList_sort(
    mut arr: *mut VqSegList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const VqSegment, *const VqSegment) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<VqSegment>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const VqSegment, *const VqSegment) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn vq_SegList_fill(mut arr: *mut VqSegList, mut n: usize) {
    while (*arr).length < n {
        let mut x: VqSegment = VqSegment {
            type_0: VQSegType::Still,
            val: VqSegmentValue { still: 0. },
        };
        if VQ_I_SEGMENT.init.is_some() {
            VQ_I_SEGMENT.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VqSegment>() as usize,
            );
        }
        vq_SegList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn vq_SegList_push(arr: *mut VqSegList, elem: VqSegment) {
    cvec_push(vq_SegList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vq_SegList_grow(arr: *mut VqSegList) {
    cvec_grow(vq_SegList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vq_SegList_growTo(arr: *mut VqSegList, target: usize) {
    cvec_grow_to(vq_SegList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vq_SegList_pop(arr: *mut VqSegList) -> VqSegment {
    cvec_pop(vq_SegList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn vq_SegList_copyReplace(mut dst: *mut VqSegList, src: VqSegList) {
    vq_SegList_dispose(dst);
    vq_SegList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vq_SegList_copy(mut dst: *mut VqSegList, mut src: *const VqSegList) {
    vq_SegList_init(dst);
    vq_SegList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if VQ_I_SEGMENT.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            VQ_I_SEGMENT.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut VqSegment,
                (*src).items.offset(j as isize) as *mut VqSegment as *const VqSegment,
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
unsafe extern "C" fn vq_SegList_dispose(mut arr: *mut VqSegList) {
    if arr.is_null() {
        return;
    }
    if VQ_I_SEGMENT.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            VQ_I_SEGMENT.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut VqSegment,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<VqSegment>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn vq_SegList_replace(mut dst: *mut VqSegList, src: VqSegList) {
    vq_SegList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VqSegList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vq_SegList_initCapN(mut arr: *mut VqSegList, mut n: usize) {
    vq_SegList_init(arr);
    vq_SegList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn vq_SegList_growToN(arr: *mut VqSegList, target: usize) {
    cvec_grow_to_n(vq_SegList_as_cvec(arr), target);
}
pub static VQ_I_SEG_LIST: VqSegListVectorInterface = {
    VqSegListVectorInterface {
        init: Some(vq_SegList_init as unsafe extern "C" fn(*mut VqSegList) -> ()),
        copy: Some(
            vq_SegList_copy as unsafe extern "C" fn(*mut VqSegList, *const VqSegList) -> (),
        ),
        move_0: Some(
            vq_SegList_move as unsafe extern "C" fn(*mut VqSegList, *mut VqSegList) -> (),
        ),
        dispose: Some(vq_SegList_dispose as unsafe extern "C" fn(*mut VqSegList) -> ()),
        replace: Some(
            vq_SegList_replace as unsafe extern "C" fn(*mut VqSegList, VqSegList) -> (),
        ),
        copyReplace: Some(
            vq_SegList_copyReplace as unsafe extern "C" fn(*mut VqSegList, VqSegList) -> (),
        ),
        create: Some(vq_SegList_create),
        free: Some(vq_SegList_free as unsafe extern "C" fn(*mut VqSegList) -> ()),
        initN: Some(vq_SegList_initN as unsafe extern "C" fn(*mut VqSegList, usize) -> ()),
        initCapN: Some(vq_SegList_initCapN as unsafe extern "C" fn(*mut VqSegList, usize) -> ()),
        createN: Some(vq_SegList_createN as unsafe extern "C" fn(usize) -> *mut VqSegList),
        fill: Some(vq_SegList_fill as unsafe extern "C" fn(*mut VqSegList, usize) -> ()),
        clear: Some(vq_SegList_dispose as unsafe extern "C" fn(*mut VqSegList) -> ()),
        push: Some(vq_SegList_push as unsafe extern "C" fn(*mut VqSegList, VqSegment) -> ()),
        shrinkToFit: Some(vq_SegList_shrinkToFit as unsafe extern "C" fn(*mut VqSegList) -> ()),
        pop: Some(vq_SegList_pop as unsafe extern "C" fn(*mut VqSegList) -> VqSegment),
        disposeItem: Some(
            vq_SegList_disposeItem as unsafe extern "C" fn(*mut VqSegList, usize) -> (),
        ),
        filterEnv: Some(
            vq_SegList_filterEnv
                as unsafe extern "C" fn(
                    *mut VqSegList,
                    Option<
                        unsafe extern "C" fn(*const VqSegment, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            vq_SegList_sort
                as unsafe extern "C" fn(
                    *mut VqSegList,
                    Option<
                        unsafe extern "C" fn(
                            *const VqSegment,
                            *const VqSegment,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn vq_SegList_free(mut x: *mut VqSegList) {
    if x.is_null() {
        return;
    }
    vq_SegList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vq_SegList_createN(mut n: usize) -> *mut VqSegList {
    let mut t: *mut VqSegList =
        malloc(::core::mem::size_of::<VqSegList>() as usize) as *mut VqSegList;
    vq_SegList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn vq_SegList_create() -> *mut VqSegList {
    let mut x: *mut VqSegList =
        malloc(::core::mem::size_of::<VqSegList>() as usize) as *mut VqSegList;
    vq_SegList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn vqInit(mut a: *mut VQ) {
    (*a).kernel = 0 as ::core::ffi::c_int as Pos;
    VQ_I_SEG_LIST.init.expect("non-null function pointer")(&raw mut (*a).shift);
}
#[inline]
unsafe extern "C" fn vqCopy(mut a: *mut VQ, mut b: *const VQ) {
    (*a).kernel = (*b).kernel;
    VQ_I_SEG_LIST.copy.expect("non-null function pointer")(
        &raw mut (*a).shift,
        &raw const (*b).shift,
    );
}
#[inline]
unsafe extern "C" fn vqDispose(mut a: *mut VQ) {
    (*a).kernel = 0 as ::core::ffi::c_int as Pos;
    VQ_I_SEG_LIST.dispose.expect("non-null function pointer")(&raw mut (*a).shift);
}
#[inline]
unsafe extern "C" fn VQ_dispose(mut x: *mut VQ) {
    vqDispose(x);
}
#[inline]
unsafe extern "C" fn VQ_copy(mut dst: *mut VQ, mut src: *const VQ) {
    vqCopy(dst, src);
}
#[inline]
unsafe extern "C" fn VQ_init(mut x: *mut VQ) {
    vqInit(x);
}
#[inline]
unsafe extern "C" fn VQ_dup(src: VQ) -> VQ {
    let mut dst: VQ = VQ {
        kernel: 0.,
        shift: VqSegList {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<VqSegment>(),
        },
    };
    VQ_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn VQ_empty() -> VQ {
    let mut x: VQ = VQ {
        kernel: 0.,
        shift: VqSegList {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<VqSegment>(),
        },
    };
    VQ_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn VQ_move(mut dst: *mut VQ, mut src: *mut VQ) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VQ>() as usize,
    );
    VQ_init(src);
}
#[inline]
unsafe extern "C" fn VQ_copyReplace(mut dst: *mut VQ, src: VQ) {
    VQ_dispose(dst);
    VQ_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn VQ_replace(mut dst: *mut VQ, src: VQ) {
    VQ_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VQ>() as usize,
    );
}
unsafe extern "C" fn vqNeutral() -> VQ {
    return I_VQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
}
unsafe extern "C" fn vqsCompatible(a: VqSegment, b: VqSegment) -> bool {
    if a.type_0 as ::core::ffi::c_uint != b.type_0 as ::core::ffi::c_uint {
        return false;
    }
    match a.type_0 as ::core::ffi::c_uint {
        0 => return true,
        1 => {
            return 0 as ::core::ffi::c_int
                == vq_compareRegion(a.val.delta.region, b.val.delta.region);
        }
        _ => {}
    }
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn simplifyVq(mut x: *mut VQ) {
    if (*x).shift.length == 0 {
        return;
    }
    VQ_I_SEG_LIST.sort.expect("non-null function pointer")(
        &raw mut (*x).shift,
        VQ_I_SEGMENT.compareRef,
    );
    let mut k: usize = 0 as usize;
    let mut j: usize = 1 as usize;
    while j < (*x).shift.length {
        if vqsCompatible(
            *(*x).shift.items.offset(k as isize),
            *(*x).shift.items.offset(j as isize),
        ) {
            match (*(*x).shift.items.offset(k as isize)).type_0 as ::core::ffi::c_uint {
                0 => {
                    (*(*x).shift.items.offset(k as isize)).val.still +=
                        (*(*x).shift.items.offset(j as isize)).val.still;
                }
                1 => {
                    (*(*x).shift.items.offset(k as isize)).val.delta.quantity +=
                        (*(*x).shift.items.offset(j as isize)).val.delta.quantity;
                }
                _ => {}
            }
            VQ_I_SEGMENT.dispose.expect("non-null function pointer")(
                (*x).shift.items.offset(j as isize) as *mut VqSegment,
            );
        } else {
            *(*x).shift.items.offset(k as isize) = *(*x).shift.items.offset(j as isize);
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    (*x).shift.length = k.wrapping_add(1 as usize);
}
unsafe extern "C" fn vqInplacePlus(mut a: *mut VQ, b: VQ) {
    (*a).kernel += b.kernel;
    let mut p: usize = 0 as usize;
    while p < b.shift.length {
        let mut k: *mut VqSegment = b.shift.items.offset(p as isize) as *mut VqSegment;
        if (*k).type_0 == VQSegType::Still
        {
            (*a).kernel += (*k).val.still;
        } else {
            let mut s: VqSegment = VqSegment {
                type_0: VQSegType::Still,
                val: VqSegmentValue { still: 0. },
            };
            VQ_I_SEGMENT.copy.expect("non-null function pointer")(&raw mut s, k);
            VQ_I_SEG_LIST.push.expect("non-null function pointer")(&raw mut (*a).shift, s);
        }
        p = p.wrapping_add(1);
    }
    simplifyVq(a);
}
#[inline]
unsafe extern "C" fn VQ_plus(a: VQ, b: VQ) -> VQ {
    let mut result: VQ = vqNeutral();
    vqInplacePlus(&raw mut result, a);
    vqInplacePlus(&raw mut result, b);
    return result;
}
#[inline]
unsafe extern "C" fn VQ_inplacePlus(mut a: *mut VQ, b: VQ) {
    vqInplacePlus(a, b);
}
#[inline]
unsafe extern "C" fn VQ_neutral() -> VQ {
    return vqNeutral();
}
unsafe extern "C" fn vqInplaceScale(mut a: *mut VQ, mut b: Pos) {
    (*a).kernel *= b;
    let mut j: usize = 0 as usize;
    while j < (*a).shift.length {
        let mut s: *mut VqSegment = (*a).shift.items.offset(j as isize) as *mut VqSegment;
        match (*s).type_0 as ::core::ffi::c_uint {
            0 => {
                (*s).val.still *= b;
            }
            1 => {
                (*s).val.delta.quantity *= b;
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn vqInplaceNegate(mut a: *mut VQ) {
    vqInplaceScale(a, -(1 as ::core::ffi::c_int) as Pos);
}
#[inline]
unsafe extern "C" fn VQ_minus(a: VQ, b: VQ) -> VQ {
    let mut result: VQ = VQ_neutral();
    VQ_inplacePlus(&raw mut result, a);
    VQ_inplaceMinus(&raw mut result, b);
    return result;
}
#[inline]
unsafe extern "C" fn VQ_inplaceMinus(mut a: *mut VQ, b: VQ) {
    let mut tb: VQ = VQ_negate(b);
    VQ_inplacePlus(a, tb);
    VQ_dispose(&raw mut tb);
}
#[inline]
unsafe extern "C" fn VQ_negate(a: VQ) -> VQ {
    let mut result: VQ = VQ {
        kernel: 0.,
        shift: VqSegList {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<VqSegment>(),
        },
    };
    VQ_copy(&raw mut result, &raw const a);
    VQ_inplaceNegate(&raw mut result);
    return result;
}
#[inline]
unsafe extern "C" fn VQ_inplaceNegate(mut a: *mut VQ) {
    vqInplaceNegate(a);
}
#[inline]
unsafe extern "C" fn VQ_inplaceScale(mut a: *mut VQ, mut b: Pos) {
    vqInplaceScale(a, b);
}
#[inline]
unsafe extern "C" fn VQ_inplacePlusScale(mut a: *mut VQ, mut b: Pos, c: VQ) {
    let mut x: VQ = VQ_scale(c, b);
    VQ_inplacePlus(a, x);
    VQ_dispose(&raw mut x);
}
#[inline]
unsafe extern "C" fn VQ_scale(a: VQ, mut b: Pos) -> VQ {
    let mut result: VQ = VQ {
        kernel: 0.,
        shift: VqSegList {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<VqSegment>(),
        },
    };
    VQ_copy(&raw mut result, &raw const a);
    VQ_inplaceScale(&raw mut result, b);
    return result;
}
unsafe extern "C" fn vqCompare(a: VQ, b: VQ) -> ::core::ffi::c_int {
    if a.shift.length < b.shift.length {
        return -(1 as ::core::ffi::c_int);
    }
    if a.shift.length > b.shift.length {
        return 1 as ::core::ffi::c_int;
    }
    let mut j: usize = 0 as usize;
    while j < a.shift.length {
        let mut cr: ::core::ffi::c_int = vqsCompare(
            *a.shift.items.offset(j as isize),
            *b.shift.items.offset(j as isize),
        );
        if cr != 0 {
            return cr;
        }
        j = j.wrapping_add(1);
    }
    return (a.kernel - b.kernel) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VQ_compareRef(mut a: *const VQ, mut b: *const VQ) -> ::core::ffi::c_int {
    return vqCompare(*a, *b);
}
#[inline]
unsafe extern "C" fn VQ_equal(a: VQ, b: VQ) -> bool {
    return vqCompare(a, b) == 0;
}
#[inline]
unsafe extern "C" fn VQ_compare(a: VQ, b: VQ) -> ::core::ffi::c_int {
    return vqCompare(a, b);
}
unsafe extern "C" fn showVQ(x: VQ) {
    fprintf(
        stderr,
        b"%g + {\0" as *const u8 as *const ::core::ffi::c_char,
        x.kernel,
    );
    let mut j: usize = 0 as usize;
    while j < x.shift.length {
        if j != 0 {
            fprintf(stderr, b" \0" as *const u8 as *const ::core::ffi::c_char);
        }
        VQ_I_SEGMENT.show.expect("non-null function pointer")(*x.shift.items.offset(j as isize));
        j = j.wrapping_add(1);
    }
    fprintf(stderr, b"}\n\0" as *const u8 as *const ::core::ffi::c_char);
}
#[inline]
unsafe extern "C" fn VQ_show(a: VQ) {
    return showVQ(a);
}
unsafe extern "C" fn vqGetStill(v: VQ) -> Pos {
    let mut result: Pos = v.kernel;
    let mut j: usize = 0 as usize;
    while j < v.shift.length {
        match (*v.shift.items.offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                result += (*v.shift.items.offset(j as isize)).val.still;
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return result;
}
unsafe extern "C" fn vqCreateStill(mut x: Pos) -> VQ {
    let mut vq: VQ = VQ {
        kernel: 0.,
        shift: VqSegList {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<VqSegment>(),
        },
    };
    I_VQ.init.expect("non-null function pointer")(&raw mut vq);
    vq.kernel = x;
    return vq;
}
unsafe extern "C" fn vqIsStill(v: VQ) -> bool {
    let mut j: usize = 0 as usize;
    while j < v.shift.length {
        match (*v.shift.items.offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {}
            _ => return false,
        }
        j = j.wrapping_add(1);
    }
    return true;
}
unsafe extern "C" fn vqIsZero(v: VQ, err: Pos) -> bool {
    return vqIsStill(v) as ::core::ffi::c_int != 0
        && fabs(vqGetStill(v) as ::core::ffi::c_double) < err;
}
unsafe extern "C" fn vqAddDelta(
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
    VQ_I_SEG_LIST.push.expect("non-null function pointer")(&raw mut (*v).shift, nudge);
}
unsafe extern "C" fn vqPointLinearTfm(ax: VQ, mut a: Pos, x: VQ, mut b: Pos, y: VQ) -> VQ {
    let mut targetX: VQ = I_VQ.dup.expect("non-null function pointer")(ax);
    I_VQ.inplacePlusScale.expect("non-null function pointer")(&raw mut targetX, a as Scale, x);
    I_VQ.inplacePlusScale.expect("non-null function pointer")(&raw mut targetX, b as Scale, y);
    return targetX;
}
pub static I_VQ: VqVectorInterface = {
    VqVectorInterface {
        init: Some(VQ_init as unsafe extern "C" fn(*mut VQ) -> ()),
        copy: Some(VQ_copy as unsafe extern "C" fn(*mut VQ, *const VQ) -> ()),
        move_0: Some(VQ_move as unsafe extern "C" fn(*mut VQ, *mut VQ) -> ()),
        dispose: Some(VQ_dispose as unsafe extern "C" fn(*mut VQ) -> ()),
        replace: Some(VQ_replace as unsafe extern "C" fn(*mut VQ, VQ) -> ()),
        copyReplace: Some(VQ_copyReplace as unsafe extern "C" fn(*mut VQ, VQ) -> ()),
        empty: Some(VQ_empty),
        dup: Some(VQ_dup as unsafe extern "C" fn(VQ) -> VQ),
        neutral: Some(VQ_neutral),
        plus: Some(VQ_plus as unsafe extern "C" fn(VQ, VQ) -> VQ),
        inplacePlus: Some(VQ_inplacePlus as unsafe extern "C" fn(*mut VQ, VQ) -> ()),
        inplaceNegate: Some(VQ_inplaceNegate as unsafe extern "C" fn(*mut VQ) -> ()),
        negate: Some(VQ_negate as unsafe extern "C" fn(VQ) -> VQ),
        inplaceMinus: Some(VQ_inplaceMinus as unsafe extern "C" fn(*mut VQ, VQ) -> ()),
        minus: Some(VQ_minus as unsafe extern "C" fn(VQ, VQ) -> VQ),
        inplaceScale: Some(VQ_inplaceScale as unsafe extern "C" fn(*mut VQ, Pos) -> ()),
        inplacePlusScale: Some(
            VQ_inplacePlusScale as unsafe extern "C" fn(*mut VQ, Pos, VQ) -> (),
        ),
        scale: Some(VQ_scale as unsafe extern "C" fn(VQ, Pos) -> VQ),
        equal: Some(VQ_equal as unsafe extern "C" fn(VQ, VQ) -> bool),
        compare: Some(VQ_compare as unsafe extern "C" fn(VQ, VQ) -> ::core::ffi::c_int),
        compareRef: Some(
            VQ_compareRef as unsafe extern "C" fn(*const VQ, *const VQ) -> ::core::ffi::c_int,
        ),
        show: Some(VQ_show as unsafe extern "C" fn(VQ) -> ()),
        getStill: Some(vqGetStill as unsafe extern "C" fn(VQ) -> Pos),
        createStill: Some(vqCreateStill as unsafe extern "C" fn(Pos) -> VQ),
        isStill: Some(vqIsStill as unsafe extern "C" fn(VQ) -> bool),
        isZero: Some(vqIsZero as unsafe extern "C" fn(VQ, Pos) -> bool),
        pointLinearTfm: Some(
            vqPointLinearTfm as unsafe extern "C" fn(VQ, Pos, VQ, Pos, VQ) -> VQ,
        ),
        addDelta: Some(
            vqAddDelta as unsafe extern "C" fn(*mut VQ, bool, *const VqRegion, Pos) -> (),
        ),
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    // This discriminant is written into the glyph hash byte-for-byte --
    // `hashVQS` in otf_reader/unconsolidate.rs does `bufwrite8(buf, s.type_0 as
    // u8)` -- and that hash decides which glyphs are treated as duplicates.
    // Renumbering the variants would silently change which glyphs get merged.
    #[test]
    fn vqsegtype_discriminants_are_the_hashed_values() {
        assert_eq!(VQSegType::Still as u8, 0);
        assert_eq!(VQSegType::Delta as u8, 1);
        assert_eq!(::core::mem::size_of::<VQSegType>(), 4);
    }
}
