#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};
use crate::support::primitives::{Pos};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::{ComparFn};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfAxis {
    pub tag: u32,
    pub min_value: Pos,
    pub default_value: Pos,
    pub max_value: Pos,
    pub flags: u16,
    pub axis_name_id: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfAxisElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VfAxis) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VfAxis, *const VfAxis) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VfAxis, *mut VfAxis) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VfAxis) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VfAxis, VfAxis) -> ()>,
    pub copy_replace: Option<unsafe extern "C" fn(*mut VfAxis, VfAxis) -> ()>,
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut VfAxes, VfAxes) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VfAxes>,
    pub free: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut VfAxes, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut VfAxes, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut VfAxes>,
    pub fill: Option<unsafe extern "C" fn(*mut VfAxes, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VfAxes, VfAxis) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut VfAxes) -> VfAxis>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut VfAxes, usize) -> ()>,
    pub filter_env: Option<
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
unsafe extern "C" fn vf_axis_init(mut x: *mut VfAxis) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VfAxis>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_axis_copy_replace(mut dst: *mut VfAxis, src: VfAxis) {
    vf_axis_dispose(dst);
    vf_axis_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vf_axis_copy(mut dst: *mut VfAxis, mut src: *const VfAxis) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VfAxis>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_axis_empty() -> VfAxis {
    let mut x: VfAxis = VfAxis {
        tag: 0,
        min_value: 0.,
        default_value: 0.,
        max_value: 0.,
        flags: 0,
        axis_name_id: 0,
    };
    vf_axis_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn vf_axis_replace(mut dst: *mut VfAxis, src: VfAxis) {
    vf_axis_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VfAxis>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_axis_dispose(mut _x: *mut VfAxis) {}
#[inline]
unsafe extern "C" fn vf_axis_move(mut dst: *mut VfAxis, mut src: *mut VfAxis) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VfAxis>() as usize,
    );
    vf_axis_init(src);
}
pub static VF_I_AXIS: VfAxisElementInterface = {
    VfAxisElementInterface {
        init: Some(vf_axis_init as unsafe extern "C" fn(*mut VfAxis) -> ()),
        copy: Some(vf_axis_copy as unsafe extern "C" fn(*mut VfAxis, *const VfAxis) -> ()),
        move_0: Some(vf_axis_move as unsafe extern "C" fn(*mut VfAxis, *mut VfAxis) -> ()),
        dispose: Some(vf_axis_dispose as unsafe extern "C" fn(*mut VfAxis) -> ()),
        replace: Some(vf_axis_replace as unsafe extern "C" fn(*mut VfAxis, VfAxis) -> ()),
        copy_replace: Some(vf_axis_copy_replace as unsafe extern "C" fn(*mut VfAxis, VfAxis) -> ()),
        empty: Some(vf_axis_empty),
        dup: Some(vf_axis_dup as unsafe extern "C" fn(VfAxis) -> VfAxis),
    }
};
#[inline]
unsafe extern "C" fn vf_axis_dup(src: VfAxis) -> VfAxis {
    let mut dst: VfAxis = VfAxis {
        tag: 0,
        min_value: 0.,
        default_value: 0.,
        max_value: 0.,
        flags: 0,
        axis_name_id: 0,
    };
    vf_axis_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn vf_axes_resize_to(arr: *mut VfAxes, target: usize) {
    cvec_resize_to(vf_axes_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vf_axes_shrink_to_fit(mut arr: *mut VfAxes) {
    vf_axes_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vf_axes_move(dst: *mut VfAxes, src: *mut VfAxes) {
    cvec_move(vf_axes_as_cvec(dst), vf_axes_as_cvec(src));
}
#[inline]
unsafe fn vf_axes_as_cvec(arr: *mut VfAxes) -> *mut CVecRaw<VfAxis> {
    arr as *mut CVecRaw<VfAxis>
}
#[inline]
unsafe extern "C" fn vf_axes_init(arr: *mut VfAxes) {
    cvec_init(vf_axes_as_cvec(arr));
}
pub static VF_I_AXES: VfAxesVectorInterface = {
    VfAxesVectorInterface {
        init: Some(vf_axes_init as unsafe extern "C" fn(*mut VfAxes) -> ()),
        copy: Some(vf_axes_copy as unsafe extern "C" fn(*mut VfAxes, *const VfAxes) -> ()),
        move_0: Some(vf_axes_move as unsafe extern "C" fn(*mut VfAxes, *mut VfAxes) -> ()),
        dispose: Some(vf_axes_dispose as unsafe extern "C" fn(*mut VfAxes) -> ()),
        replace: Some(vf_axes_replace as unsafe extern "C" fn(*mut VfAxes, VfAxes) -> ()),
        copy_replace: Some(vf_axes_copy_replace as unsafe extern "C" fn(*mut VfAxes, VfAxes) -> ()),
        create: Some(vf_axes_create),
        free: Some(vf_axes_free as unsafe extern "C" fn(*mut VfAxes) -> ()),
        init_n: Some(vf_axes_init_n as unsafe extern "C" fn(*mut VfAxes, usize) -> ()),
        init_cap_n: Some(vf_axes_init_cap_n as unsafe extern "C" fn(*mut VfAxes, usize) -> ()),
        create_n: Some(vf_axes_create_n as unsafe extern "C" fn(usize) -> *mut VfAxes),
        fill: Some(vf_axes_fill as unsafe extern "C" fn(*mut VfAxes, usize) -> ()),
        clear: Some(vf_axes_dispose as unsafe extern "C" fn(*mut VfAxes) -> ()),
        push: Some(vf_axes_push as unsafe extern "C" fn(*mut VfAxes, VfAxis) -> ()),
        shrink_to_fit: Some(vf_axes_shrink_to_fit as unsafe extern "C" fn(*mut VfAxes) -> ()),
        pop: Some(vf_axes_pop as unsafe extern "C" fn(*mut VfAxes) -> VfAxis),
        dispose_item: Some(vf_axes_dispose_item as unsafe extern "C" fn(*mut VfAxes, usize) -> ()),
        filter_env: Some(
            vf_axes_filter_env
                as unsafe extern "C" fn(
                    *mut VfAxes,
                    Option<unsafe extern "C" fn(*const VfAxis, *mut ::core::ffi::c_void) -> bool>,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            vf_axes_sort
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
unsafe extern "C" fn vf_axes_filter_env(
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
            if VF_I_AXIS.dispose.is_some() {
                VF_I_AXIS.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn vf_axes_dispose_item(mut arr: *mut VfAxes, mut n: usize) {
    if VF_I_AXIS.dispose.is_some() {
        VF_I_AXIS.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut VfAxis
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn vf_axes_sort(
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
unsafe extern "C" fn vf_axes_fill(mut arr: *mut VfAxes, mut n: usize) {
    while (*arr).length < n {
        let mut x: VfAxis = VfAxis {
            tag: 0,
            min_value: 0.,
            default_value: 0.,
            max_value: 0.,
            flags: 0,
            axis_name_id: 0,
        };
        if VF_I_AXIS.init.is_some() {
            VF_I_AXIS.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VfAxis>() as usize,
            );
        }
        vf_axes_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn vf_axes_push(arr: *mut VfAxes, elem: VfAxis) {
    cvec_push(vf_axes_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vf_axes_grow_to(arr: *mut VfAxes, target: usize) {
    cvec_grow_to(vf_axes_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vf_axes_grow(arr: *mut VfAxes) {
    cvec_grow(vf_axes_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vf_axes_copy_replace(mut dst: *mut VfAxes, src: VfAxes) {
    vf_axes_dispose(dst);
    vf_axes_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vf_axes_copy(mut dst: *mut VfAxes, mut src: *const VfAxes) {
    vf_axes_init(dst);
    vf_axes_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if VF_I_AXIS.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            VF_I_AXIS.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn vf_axes_dispose(mut arr: *mut VfAxes) {
    if arr.is_null() {
        return;
    }
    if VF_I_AXIS.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            VF_I_AXIS.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn vf_axes_pop(arr: *mut VfAxes) -> VfAxis {
    cvec_pop(vf_axes_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn vf_axes_replace(mut dst: *mut VfAxes, src: VfAxes) {
    vf_axes_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VfAxes>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_axes_init_cap_n(mut arr: *mut VfAxes, mut n: usize) {
    vf_axes_init(arr);
    vf_axes_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn vf_axes_grow_to_n(arr: *mut VfAxes, target: usize) {
    cvec_grow_to_n(vf_axes_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vf_axes_init_n(mut arr: *mut VfAxes, mut n: usize) {
    vf_axes_init(arr);
    vf_axes_grow_to_n(arr, n);
    vf_axes_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vf_axes_free(mut x: *mut VfAxes) {
    if x.is_null() {
        return;
    }
    vf_axes_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vf_axes_create_n(mut n: usize) -> *mut VfAxes {
    let mut t: *mut VfAxes = malloc(::core::mem::size_of::<VfAxes>() as usize) as *mut VfAxes;
    vf_axes_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn vf_axes_create() -> *mut VfAxes {
    let mut x: *mut VfAxes = malloc(::core::mem::size_of::<VfAxes>() as usize) as *mut VfAxes;
    vf_axes_init(x);
    return x;
}
