use libc::{free, malloc, memcpy, memset, qsort};
use crate::support::cvec::{
    cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push,
    cvec_resize_to, CVecRaw,
};
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type pos_t = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vf_Axis {
    pub tag: u32,
    pub minValue: pos_t,
    pub defaultValue: pos_t,
    pub maxValue: pos_t,
    pub flags: u16,
    pub axisNameID: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_vf_Axis {
    pub init: Option<unsafe extern "C" fn(*mut vf_Axis) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut vf_Axis, *const vf_Axis) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut vf_Axis, *mut vf_Axis) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vf_Axis) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vf_Axis, vf_Axis) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut vf_Axis, vf_Axis) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> vf_Axis>,
    pub dup: Option<unsafe extern "C" fn(vf_Axis) -> vf_Axis>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vf_Axes {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut vf_Axis,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_vf_Axes {
    pub init: Option<unsafe extern "C" fn(*mut vf_Axes) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut vf_Axes, *const vf_Axes) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut vf_Axes, *mut vf_Axes) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vf_Axes) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vf_Axes, vf_Axes) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut vf_Axes, vf_Axes) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut vf_Axes>,
    pub free: Option<unsafe extern "C" fn(*mut vf_Axes) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut vf_Axes, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut vf_Axes, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut vf_Axes>,
    pub fill: Option<unsafe extern "C" fn(*mut vf_Axes, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut vf_Axes) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut vf_Axes, vf_Axis) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut vf_Axes) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut vf_Axes) -> vf_Axis>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut vf_Axes, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut vf_Axes,
            Option<unsafe extern "C" fn(*const vf_Axis, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut vf_Axes,
            Option<unsafe extern "C" fn(*const vf_Axis, *const vf_Axis) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn vf_Axis_init(mut x: *mut vf_Axis) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<vf_Axis>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_Axis_copyReplace(mut dst: *mut vf_Axis, src: vf_Axis) {
    vf_Axis_dispose(dst);
    vf_Axis_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vf_Axis_copy(mut dst: *mut vf_Axis, mut src: *const vf_Axis) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vf_Axis>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_Axis_empty() -> vf_Axis {
    let mut x: vf_Axis = vf_Axis {
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
unsafe extern "C" fn vf_Axis_replace(mut dst: *mut vf_Axis, src: vf_Axis) {
    vf_Axis_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vf_Axis>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_Axis_dispose(mut _x: *mut vf_Axis) {}
#[inline]
unsafe extern "C" fn vf_Axis_move(mut dst: *mut vf_Axis, mut src: *mut vf_Axis) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vf_Axis>() as usize,
    );
    vf_Axis_init(src);
}
#[no_mangle]
pub static mut vf_iAxis: __caryll_elementinterface_vf_Axis = {
    __caryll_elementinterface_vf_Axis {
        init: Some(vf_Axis_init as unsafe extern "C" fn(*mut vf_Axis) -> ()),
        copy: Some(vf_Axis_copy as unsafe extern "C" fn(*mut vf_Axis, *const vf_Axis) -> ()),
        move_0: Some(vf_Axis_move as unsafe extern "C" fn(*mut vf_Axis, *mut vf_Axis) -> ()),
        dispose: Some(vf_Axis_dispose as unsafe extern "C" fn(*mut vf_Axis) -> ()),
        replace: Some(vf_Axis_replace as unsafe extern "C" fn(*mut vf_Axis, vf_Axis) -> ()),
        copyReplace: Some(vf_Axis_copyReplace as unsafe extern "C" fn(*mut vf_Axis, vf_Axis) -> ()),
        empty: Some(vf_Axis_empty),
        dup: Some(vf_Axis_dup as unsafe extern "C" fn(vf_Axis) -> vf_Axis),
    }
};
#[inline]
unsafe extern "C" fn vf_Axis_dup(src: vf_Axis) -> vf_Axis {
    let mut dst: vf_Axis = vf_Axis {
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
unsafe extern "C" fn vf_Axes_resizeTo(arr: *mut vf_Axes, target: usize) {
    cvec_resize_to(vf_Axes_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vf_Axes_shrinkToFit(mut arr: *mut vf_Axes) {
    vf_Axes_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vf_Axes_move(dst: *mut vf_Axes, src: *mut vf_Axes) {
    cvec_move(vf_Axes_as_cvec(dst), vf_Axes_as_cvec(src));
}
#[inline]
unsafe fn vf_Axes_as_cvec(arr: *mut vf_Axes) -> *mut CVecRaw<vf_Axis> {
    arr as *mut CVecRaw<vf_Axis>
}
#[inline]
unsafe extern "C" fn vf_Axes_init(arr: *mut vf_Axes) {
    cvec_init(vf_Axes_as_cvec(arr));
}
#[no_mangle]
pub static mut vf_iAxes: __caryll_vectorinterface_vf_Axes = {
    __caryll_vectorinterface_vf_Axes {
        init: Some(vf_Axes_init as unsafe extern "C" fn(*mut vf_Axes) -> ()),
        copy: Some(vf_Axes_copy as unsafe extern "C" fn(*mut vf_Axes, *const vf_Axes) -> ()),
        move_0: Some(vf_Axes_move as unsafe extern "C" fn(*mut vf_Axes, *mut vf_Axes) -> ()),
        dispose: Some(vf_Axes_dispose as unsafe extern "C" fn(*mut vf_Axes) -> ()),
        replace: Some(vf_Axes_replace as unsafe extern "C" fn(*mut vf_Axes, vf_Axes) -> ()),
        copyReplace: Some(vf_Axes_copyReplace as unsafe extern "C" fn(*mut vf_Axes, vf_Axes) -> ()),
        create: Some(vf_Axes_create),
        free: Some(vf_Axes_free as unsafe extern "C" fn(*mut vf_Axes) -> ()),
        initN: Some(vf_Axes_initN as unsafe extern "C" fn(*mut vf_Axes, usize) -> ()),
        initCapN: Some(vf_Axes_initCapN as unsafe extern "C" fn(*mut vf_Axes, usize) -> ()),
        createN: Some(vf_Axes_createN as unsafe extern "C" fn(usize) -> *mut vf_Axes),
        fill: Some(vf_Axes_fill as unsafe extern "C" fn(*mut vf_Axes, usize) -> ()),
        clear: Some(vf_Axes_dispose as unsafe extern "C" fn(*mut vf_Axes) -> ()),
        push: Some(vf_Axes_push as unsafe extern "C" fn(*mut vf_Axes, vf_Axis) -> ()),
        shrinkToFit: Some(vf_Axes_shrinkToFit as unsafe extern "C" fn(*mut vf_Axes) -> ()),
        pop: Some(vf_Axes_pop as unsafe extern "C" fn(*mut vf_Axes) -> vf_Axis),
        disposeItem: Some(vf_Axes_disposeItem as unsafe extern "C" fn(*mut vf_Axes, usize) -> ()),
        filterEnv: Some(
            vf_Axes_filterEnv
                as unsafe extern "C" fn(
                    *mut vf_Axes,
                    Option<unsafe extern "C" fn(*const vf_Axis, *mut ::core::ffi::c_void) -> bool>,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            vf_Axes_sort
                as unsafe extern "C" fn(
                    *mut vf_Axes,
                    Option<
                        unsafe extern "C" fn(*const vf_Axis, *const vf_Axis) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn vf_Axes_filterEnv(
    mut arr: *mut vf_Axes,
    mut fn_0: Option<unsafe extern "C" fn(*const vf_Axis, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut vf_Axis,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if vf_iAxis.dispose.is_some() {
                vf_iAxis.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut vf_Axis,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn vf_Axes_disposeItem(mut arr: *mut vf_Axes, mut n: usize) {
    if vf_iAxis.dispose.is_some() {
        vf_iAxis.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut vf_Axis
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn vf_Axes_sort(
    mut arr: *mut vf_Axes,
    mut fn_0: Option<unsafe extern "C" fn(*const vf_Axis, *const vf_Axis) -> ::core::ffi::c_int>,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<vf_Axis>() as usize,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const vf_Axis, *const vf_Axis) -> ::core::ffi::c_int>,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn vf_Axes_fill(mut arr: *mut vf_Axes, mut n: usize) {
    while (*arr).length < n {
        let mut x: vf_Axis = vf_Axis {
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
                ::core::mem::size_of::<vf_Axis>() as usize,
            );
        }
        vf_Axes_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn vf_Axes_push(arr: *mut vf_Axes, elem: vf_Axis) {
    cvec_push(vf_Axes_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vf_Axes_growTo(arr: *mut vf_Axes, target: usize) {
    cvec_grow_to(vf_Axes_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vf_Axes_grow(arr: *mut vf_Axes) {
    cvec_grow(vf_Axes_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vf_Axes_copyReplace(mut dst: *mut vf_Axes, src: vf_Axes) {
    vf_Axes_dispose(dst);
    vf_Axes_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vf_Axes_copy(mut dst: *mut vf_Axes, mut src: *const vf_Axes) {
    vf_Axes_init(dst);
    vf_Axes_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if vf_iAxis.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            vf_iAxis.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut vf_Axis,
                (*src).items.offset(j as isize) as *mut vf_Axis as *const vf_Axis,
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
unsafe extern "C" fn vf_Axes_dispose(mut arr: *mut vf_Axes) {
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
                (*arr).items.offset(j as isize) as *mut vf_Axis
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<vf_Axis>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn vf_Axes_pop(arr: *mut vf_Axes) -> vf_Axis {
    cvec_pop(vf_Axes_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn vf_Axes_replace(mut dst: *mut vf_Axes, src: vf_Axes) {
    vf_Axes_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vf_Axes>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vf_Axes_initCapN(mut arr: *mut vf_Axes, mut n: usize) {
    vf_Axes_init(arr);
    vf_Axes_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn vf_Axes_growToN(arr: *mut vf_Axes, target: usize) {
    cvec_grow_to_n(vf_Axes_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vf_Axes_initN(mut arr: *mut vf_Axes, mut n: usize) {
    vf_Axes_init(arr);
    vf_Axes_growToN(arr, n);
    vf_Axes_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vf_Axes_free(mut x: *mut vf_Axes) {
    if x.is_null() {
        return;
    }
    vf_Axes_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vf_Axes_createN(mut n: usize) -> *mut vf_Axes {
    let mut t: *mut vf_Axes = malloc(::core::mem::size_of::<vf_Axes>() as usize) as *mut vf_Axes;
    vf_Axes_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn vf_Axes_create() -> *mut vf_Axes {
    let mut x: *mut vf_Axes = malloc(::core::mem::size_of::<vf_Axes>() as usize) as *mut vf_Axes;
    vf_Axes_init(x);
    return x;
}
