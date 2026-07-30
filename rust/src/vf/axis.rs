#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};
use crate::support::primitives::{Pos};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::{ComparFn};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfAxis {
    pub tag: u32,
    pub minValue: Pos,
    pub defaultValue: Pos,
    pub maxValue: Pos,
    pub flags: u16,
    pub axisNameID: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfAxisElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VfAxis) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VfAxis, *const VfAxis) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VfAxis, *mut VfAxis) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VfAxis) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VfAxis, VfAxis) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VfAxis, VfAxis) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> VfAxis>,
    pub dup: Option<unsafe extern "C" fn(VfAxis) -> VfAxis>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfAxes {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut VfAxis,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfAxesVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VfAxes, *const VfAxes) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VfAxes, *mut VfAxes) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VfAxes, VfAxes) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VfAxes, VfAxes) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VfAxes>,
    pub free: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut VfAxes, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut VfAxes, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut VfAxes>,
    pub fill: Option<unsafe extern "C" fn(*mut VfAxes, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VfAxes, VfAxis) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut VfAxes) -> VfAxis>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut VfAxes, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut VfAxes,
            Option<unsafe extern "C" fn(*const VfAxis, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut VfAxes,
            Option<unsafe extern "C" fn(*const VfAxis, *const VfAxis) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[inline]
unsafe extern "C" fn vf_Axis_init(mut x: *mut VfAxis) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VfAxis>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_Axis_copyReplace(mut dst: *mut VfAxis, src: VfAxis) {
    vf_Axis_dispose(dst);
    vf_Axis_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vf_Axis_copy(mut dst: *mut VfAxis, mut src: *const VfAxis) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VfAxis>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_Axis_empty() -> VfAxis {
    let mut x: VfAxis = VfAxis {
        tag: 0,
        minValue: 0.,
        defaultValue: 0.,
        maxValue: 0.,
        flags: 0,
        axisNameID: 0,
    };
    vf_Axis_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn vf_Axis_replace(mut dst: *mut VfAxis, src: VfAxis) {
    vf_Axis_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VfAxis>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_Axis_dispose(mut _x: *mut VfAxis) {}
#[inline]
unsafe extern "C" fn vf_Axis_move(mut dst: *mut VfAxis, mut src: *mut VfAxis) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VfAxis>() as usize,
    );
    vf_Axis_init(src);
}
pub static vf_iAxis: VfAxisElementInterface = {
    VfAxisElementInterface {
        init: Some(vf_Axis_init as unsafe extern "C" fn(*mut VfAxis) -> ()),
        copy: Some(vf_Axis_copy as unsafe extern "C" fn(*mut VfAxis, *const VfAxis) -> ()),
        move_0: Some(vf_Axis_move as unsafe extern "C" fn(*mut VfAxis, *mut VfAxis) -> ()),
        dispose: Some(vf_Axis_dispose as unsafe extern "C" fn(*mut VfAxis) -> ()),
        replace: Some(vf_Axis_replace as unsafe extern "C" fn(*mut VfAxis, VfAxis) -> ()),
        copyReplace: Some(vf_Axis_copyReplace as unsafe extern "C" fn(*mut VfAxis, VfAxis) -> ()),
        empty: Some(vf_Axis_empty),
        dup: Some(vf_Axis_dup as unsafe extern "C" fn(VfAxis) -> VfAxis),
    }
};
#[inline]
unsafe extern "C" fn vf_Axis_dup(src: VfAxis) -> VfAxis {
    let mut dst: VfAxis = VfAxis {
        tag: 0,
        minValue: 0.,
        defaultValue: 0.,
        maxValue: 0.,
        flags: 0,
        axisNameID: 0,
    };
    vf_Axis_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn vf_Axes_resizeTo(arr: *mut VfAxes, target: usize) {
    cvec_resize_to(vf_Axes_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vf_Axes_shrinkToFit(mut arr: *mut VfAxes) {
    vf_Axes_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vf_Axes_move(dst: *mut VfAxes, src: *mut VfAxes) {
    cvec_move(vf_Axes_as_cvec(dst), vf_Axes_as_cvec(src));
}
#[inline]
unsafe fn vf_Axes_as_cvec(arr: *mut VfAxes) -> *mut CVecRaw<VfAxis> {
    arr as *mut CVecRaw<VfAxis>
}
#[inline]
unsafe extern "C" fn vf_Axes_init(arr: *mut VfAxes) {
    cvec_init(vf_Axes_as_cvec(arr));
}
pub static vf_iAxes: VfAxesVectorInterface = {
    VfAxesVectorInterface {
        init: Some(vf_Axes_init as unsafe extern "C" fn(*mut VfAxes) -> ()),
        copy: Some(vf_Axes_copy as unsafe extern "C" fn(*mut VfAxes, *const VfAxes) -> ()),
        move_0: Some(vf_Axes_move as unsafe extern "C" fn(*mut VfAxes, *mut VfAxes) -> ()),
        dispose: Some(vf_Axes_dispose as unsafe extern "C" fn(*mut VfAxes) -> ()),
        replace: Some(vf_Axes_replace as unsafe extern "C" fn(*mut VfAxes, VfAxes) -> ()),
        copyReplace: Some(vf_Axes_copyReplace as unsafe extern "C" fn(*mut VfAxes, VfAxes) -> ()),
        create: Some(vf_Axes_create),
        free: Some(vf_Axes_free as unsafe extern "C" fn(*mut VfAxes) -> ()),
        initN: Some(vf_Axes_initN as unsafe extern "C" fn(*mut VfAxes, usize) -> ()),
        initCapN: Some(vf_Axes_initCapN as unsafe extern "C" fn(*mut VfAxes, usize) -> ()),
        createN: Some(vf_Axes_createN as unsafe extern "C" fn(usize) -> *mut VfAxes),
        fill: Some(vf_Axes_fill as unsafe extern "C" fn(*mut VfAxes, usize) -> ()),
        clear: Some(vf_Axes_dispose as unsafe extern "C" fn(*mut VfAxes) -> ()),
        push: Some(vf_Axes_push as unsafe extern "C" fn(*mut VfAxes, VfAxis) -> ()),
        shrinkToFit: Some(vf_Axes_shrinkToFit as unsafe extern "C" fn(*mut VfAxes) -> ()),
        pop: Some(vf_Axes_pop as unsafe extern "C" fn(*mut VfAxes) -> VfAxis),
        disposeItem: Some(vf_Axes_disposeItem as unsafe extern "C" fn(*mut VfAxes, usize) -> ()),
        filterEnv: Some(
            vf_Axes_filterEnv
                as unsafe extern "C" fn(
                    *mut VfAxes,
                    Option<unsafe extern "C" fn(*const VfAxis, *mut ::core::ffi::c_void) -> bool>,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            vf_Axes_sort
                as unsafe extern "C" fn(
                    *mut VfAxes,
                    Option<
                        unsafe extern "C" fn(*const VfAxis, *const VfAxis) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn vf_Axes_filterEnv(
    mut arr: *mut VfAxes,
    mut fn_0: Option<unsafe extern "C" fn(*const VfAxis, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut VfAxis,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if vf_iAxis.dispose.is_some() {
                vf_iAxis.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut VfAxis,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn vf_Axes_disposeItem(mut arr: *mut VfAxes, mut n: usize) {
    if vf_iAxis.dispose.is_some() {
        vf_iAxis.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut VfAxis
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn vf_Axes_sort(
    mut arr: *mut VfAxes,
    mut fn_0: Option<unsafe extern "C" fn(*const VfAxis, *const VfAxis) -> ::core::ffi::c_int>,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<VfAxis>() as usize,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const VfAxis, *const VfAxis) -> ::core::ffi::c_int>,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn vf_Axes_fill(mut arr: *mut VfAxes, mut n: usize) {
    while (*arr).length < n {
        let mut x: VfAxis = VfAxis {
            tag: 0,
            minValue: 0.,
            defaultValue: 0.,
            maxValue: 0.,
            flags: 0,
            axisNameID: 0,
        };
        if vf_iAxis.init.is_some() {
            vf_iAxis.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VfAxis>() as usize,
            );
        }
        vf_Axes_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn vf_Axes_push(arr: *mut VfAxes, elem: VfAxis) {
    cvec_push(vf_Axes_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vf_Axes_growTo(arr: *mut VfAxes, target: usize) {
    cvec_grow_to(vf_Axes_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vf_Axes_grow(arr: *mut VfAxes) {
    cvec_grow(vf_Axes_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vf_Axes_copyReplace(mut dst: *mut VfAxes, src: VfAxes) {
    vf_Axes_dispose(dst);
    vf_Axes_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vf_Axes_copy(mut dst: *mut VfAxes, mut src: *const VfAxes) {
    vf_Axes_init(dst);
    vf_Axes_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if vf_iAxis.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            vf_iAxis.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut VfAxis,
                (*src).items.offset(j as isize) as *mut VfAxis as *const VfAxis,
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
unsafe extern "C" fn vf_Axes_dispose(mut arr: *mut VfAxes) {
    if arr.is_null() {
        return;
    }
    if vf_iAxis.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            vf_iAxis.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut VfAxis
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<VfAxis>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn vf_Axes_pop(arr: *mut VfAxes) -> VfAxis {
    cvec_pop(vf_Axes_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn vf_Axes_replace(mut dst: *mut VfAxes, src: VfAxes) {
    vf_Axes_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VfAxes>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_Axes_initCapN(mut arr: *mut VfAxes, mut n: usize) {
    vf_Axes_init(arr);
    vf_Axes_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn vf_Axes_growToN(arr: *mut VfAxes, target: usize) {
    cvec_grow_to_n(vf_Axes_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vf_Axes_initN(mut arr: *mut VfAxes, mut n: usize) {
    vf_Axes_init(arr);
    vf_Axes_growToN(arr, n);
    vf_Axes_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vf_Axes_free(mut x: *mut VfAxes) {
    if x.is_null() {
        return;
    }
    vf_Axes_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vf_Axes_createN(mut n: usize) -> *mut VfAxes {
    let mut t: *mut VfAxes = malloc(::core::mem::size_of::<VfAxes>() as usize) as *mut VfAxes;
    vf_Axes_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn vf_Axes_create() -> *mut VfAxes {
    let mut x: *mut VfAxes = malloc(::core::mem::size_of::<VfAxes>() as usize) as *mut VfAxes;
    vf_Axes_init(x);
    return x;
}
