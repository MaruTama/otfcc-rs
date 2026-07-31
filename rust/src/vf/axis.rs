#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};
use crate::support::primitives::{Pos};
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_init, cvec_push, cvec_resize_to};

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
    pub dispose: Option<unsafe extern "C" fn(*mut VfAxis) -> ()>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VfAxes>,
    pub free: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VfAxes, VfAxis) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut VfAxes) -> ()>,
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
unsafe extern "C" fn vf_axis_dispose(mut _x: *mut VfAxis) {}
pub static VF_I_AXIS: VfAxisElementInterface = {
    VfAxisElementInterface {
        init: Some(vf_axis_init as unsafe extern "C" fn(*mut VfAxis) -> ()),
        copy: Some(vf_axis_copy as unsafe extern "C" fn(*mut VfAxis, *const VfAxis) -> ()),
        dispose: Some(vf_axis_dispose as unsafe extern "C" fn(*mut VfAxis) -> ()),
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
        dispose: Some(vf_axes_dispose as unsafe extern "C" fn(*mut VfAxes) -> ()),
        create: Some(vf_axes_create),
        free: Some(vf_axes_free as unsafe extern "C" fn(*mut VfAxes) -> ()),
        push: Some(vf_axes_push as unsafe extern "C" fn(*mut VfAxes, VfAxis) -> ()),
        shrink_to_fit: Some(vf_axes_shrink_to_fit as unsafe extern "C" fn(*mut VfAxes) -> ()),
    }
};
#[inline]
unsafe extern "C" fn vf_axes_push(arr: *mut VfAxes, elem: VfAxis) {
    cvec_push(vf_axes_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vf_axes_grow_to(arr: *mut VfAxes, target: usize) {
    cvec_grow_to(vf_axes_as_cvec(arr), target);
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
unsafe extern "C" fn vf_axes_free(mut x: *mut VfAxes) {
    if x.is_null() {
        return;
    }
    vf_axes_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vf_axes_create() -> *mut VfAxes {
    let mut x: *mut VfAxes = malloc(::core::mem::size_of::<VfAxes>() as usize) as *mut VfAxes;
    vf_axes_init(x);
    return x;
}
