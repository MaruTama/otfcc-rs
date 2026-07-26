#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::{__compar_fn_t};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_Record {
    pub yPelHeight: u16,
    pub yMax: i16,
    pub yMin: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_vdmx_Record {
    pub init: Option<unsafe extern "C" fn(*mut vdmx_Record) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut vdmx_Record, *const vdmx_Record) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut vdmx_Record, *mut vdmx_Record) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vdmx_Record) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vdmx_Record, vdmx_Record) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut vdmx_Record, vdmx_Record) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> vdmx_Record>,
    pub dup: Option<unsafe extern "C" fn(vdmx_Record) -> vdmx_Record>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_Group {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut vdmx_Record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_vdmx_Group {
    pub init: Option<unsafe extern "C" fn(*mut vdmx_Group) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut vdmx_Group, *const vdmx_Group) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut vdmx_Group, *mut vdmx_Group) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vdmx_Group) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vdmx_Group, vdmx_Group) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut vdmx_Group, vdmx_Group) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut vdmx_Group>,
    pub free: Option<unsafe extern "C" fn(*mut vdmx_Group) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut vdmx_Group, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut vdmx_Group, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut vdmx_Group>,
    pub fill: Option<unsafe extern "C" fn(*mut vdmx_Group, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut vdmx_Group) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut vdmx_Group, vdmx_Record) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut vdmx_Group) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut vdmx_Group) -> vdmx_Record>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut vdmx_Group, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut vdmx_Group,
            Option<unsafe extern "C" fn(*const vdmx_Record, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut vdmx_Group,
            Option<
                unsafe extern "C" fn(*const vdmx_Record, *const vdmx_Record) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_RatioRange {
    pub bCharset: u8,
    pub xRatio: u8,
    pub yStartRatio: u8,
    pub yEndRatio: u8,
    pub records: vdmx_Group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_vdmx_RatioRange {
    pub init: Option<unsafe extern "C" fn(*mut vdmx_RatioRange) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut vdmx_RatioRange, *const vdmx_RatioRange) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut vdmx_RatioRange, *mut vdmx_RatioRange) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vdmx_RatioRange) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vdmx_RatioRange, vdmx_RatioRange) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut vdmx_RatioRange, vdmx_RatioRange) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_RatioRagneList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut vdmx_RatioRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_vdmx_RatioRagneList {
    pub init: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, *const vdmx_RatioRagneList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, *mut vdmx_RatioRagneList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, vdmx_RatioRagneList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, vdmx_RatioRagneList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut vdmx_RatioRagneList>,
    pub free: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut vdmx_RatioRagneList>,
    pub fill: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, vdmx_RatioRange) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> vdmx_RatioRange>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut vdmx_RatioRagneList,
            Option<unsafe extern "C" fn(*const vdmx_RatioRange, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut vdmx_RatioRagneList,
            Option<
                unsafe extern "C" fn(
                    *const vdmx_RatioRange,
                    *const vdmx_RatioRange,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_VDMX {
    pub version: u16,
    pub ratios: vdmx_RatioRagneList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_VDMX {
    pub init: Option<unsafe extern "C" fn(*mut table_VDMX) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_VDMX, *const table_VDMX) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_VDMX, *mut table_VDMX) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_VDMX) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_VDMX, table_VDMX) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_VDMX, table_VDMX) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_VDMX>,
    pub free: Option<unsafe extern "C" fn(*mut table_VDMX) -> ()>,
}
#[inline]
unsafe extern "C" fn vdmx_Record_init(mut x: *mut vdmx_Record) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<vdmx_Record>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_Record_copy(mut dst: *mut vdmx_Record, mut src: *const vdmx_Record) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vdmx_Record>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_Record_empty() -> vdmx_Record {
    let mut x: vdmx_Record = vdmx_Record {
        yPelHeight: 0,
        yMax: 0,
        yMin: 0,
    };
    vdmx_Record_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn vdmx_Record_replace(mut dst: *mut vdmx_Record, src: vdmx_Record) {
    vdmx_Record_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vdmx_Record>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_Record_dispose(mut _x: *mut vdmx_Record) {}
#[inline]
unsafe extern "C" fn vdmx_Record_move(mut dst: *mut vdmx_Record, mut src: *mut vdmx_Record) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vdmx_Record>() as usize,
    );
    vdmx_Record_init(src);
}
#[inline]
unsafe extern "C" fn vdmx_Record_dup(src: vdmx_Record) -> vdmx_Record {
    let mut dst: vdmx_Record = vdmx_Record {
        yPelHeight: 0,
        yMax: 0,
        yMin: 0,
    };
    vdmx_Record_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn vdmx_Record_copyReplace(mut dst: *mut vdmx_Record, src: vdmx_Record) {
    vdmx_Record_dispose(dst);
    vdmx_Record_copy(dst, &raw const src);
}
pub static vdmx_iRecord: __caryll_elementinterface_vdmx_Record = {
    __caryll_elementinterface_vdmx_Record {
        init: Some(vdmx_Record_init as unsafe extern "C" fn(*mut vdmx_Record) -> ()),
        copy: Some(
            vdmx_Record_copy as unsafe extern "C" fn(*mut vdmx_Record, *const vdmx_Record) -> (),
        ),
        move_0: Some(
            vdmx_Record_move as unsafe extern "C" fn(*mut vdmx_Record, *mut vdmx_Record) -> (),
        ),
        dispose: Some(vdmx_Record_dispose as unsafe extern "C" fn(*mut vdmx_Record) -> ()),
        replace: Some(
            vdmx_Record_replace as unsafe extern "C" fn(*mut vdmx_Record, vdmx_Record) -> (),
        ),
        copyReplace: Some(
            vdmx_Record_copyReplace as unsafe extern "C" fn(*mut vdmx_Record, vdmx_Record) -> (),
        ),
        empty: Some(vdmx_Record_empty),
        dup: Some(vdmx_Record_dup as unsafe extern "C" fn(vdmx_Record) -> vdmx_Record),
    }
};
#[inline]
unsafe extern "C" fn vdmx_Group_createN(mut n: usize) -> *mut vdmx_Group {
    let mut t: *mut vdmx_Group =
        malloc(::core::mem::size_of::<vdmx_Group>() as usize) as *mut vdmx_Group;
    vdmx_Group_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn vdmx_Group_push(arr: *mut vdmx_Group, elem: vdmx_Record) {
    cvec_push(vdmx_Group_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vdmx_Group_grow(arr: *mut vdmx_Group) {
    cvec_grow(vdmx_Group_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_Group_growTo(arr: *mut vdmx_Group, target: usize) {
    cvec_grow_to(vdmx_Group_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_Group_pop(arr: *mut vdmx_Group) -> vdmx_Record {
    cvec_pop(vdmx_Group_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn vdmx_Group_copyReplace(mut dst: *mut vdmx_Group, src: vdmx_Group) {
    vdmx_Group_dispose(dst);
    vdmx_Group_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vdmx_Group_copy(mut dst: *mut vdmx_Group, mut src: *const vdmx_Group) {
    vdmx_Group_init(dst);
    vdmx_Group_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if vdmx_iRecord.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            vdmx_iRecord.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut vdmx_Record,
                (*src).items.offset(j as isize) as *mut vdmx_Record as *const vdmx_Record,
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
unsafe extern "C" fn vdmx_Group_dispose(mut arr: *mut vdmx_Group) {
    if arr.is_null() {
        return;
    }
    if vdmx_iRecord.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            vdmx_iRecord.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut vdmx_Record,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<vdmx_Record>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn vdmx_Group_replace(mut dst: *mut vdmx_Group, src: vdmx_Group) {
    vdmx_Group_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vdmx_Group>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_Group_initCapN(mut arr: *mut vdmx_Group, mut n: usize) {
    vdmx_Group_init(arr);
    vdmx_Group_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_Group_growToN(arr: *mut vdmx_Group, target: usize) {
    cvec_grow_to_n(vdmx_Group_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_Group_initN(mut arr: *mut vdmx_Group, mut n: usize) {
    vdmx_Group_init(arr);
    vdmx_Group_growToN(arr, n);
    vdmx_Group_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_Group_free(mut x: *mut vdmx_Group) {
    if x.is_null() {
        return;
    }
    vdmx_Group_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vdmx_Group_shrinkToFit(mut arr: *mut vdmx_Group) {
    vdmx_Group_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vdmx_Group_create() -> *mut vdmx_Group {
    let mut x: *mut vdmx_Group =
        malloc(::core::mem::size_of::<vdmx_Group>() as usize) as *mut vdmx_Group;
    vdmx_Group_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn vdmx_Group_resizeTo(arr: *mut vdmx_Group, target: usize) {
    cvec_resize_to(vdmx_Group_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_Group_fill(mut arr: *mut vdmx_Group, mut n: usize) {
    while (*arr).length < n {
        let mut x: vdmx_Record = vdmx_Record {
            yPelHeight: 0,
            yMax: 0,
            yMin: 0,
        };
        if vdmx_iRecord.init.is_some() {
            vdmx_iRecord.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<vdmx_Record>() as usize,
            );
        }
        vdmx_Group_push(arr, x);
    }
}
#[inline]
unsafe fn vdmx_Group_as_cvec(arr: *mut vdmx_Group) -> *mut CVecRaw<vdmx_Record> {
    arr as *mut CVecRaw<vdmx_Record>
}
#[inline]
unsafe extern "C" fn vdmx_Group_init(arr: *mut vdmx_Group) {
    cvec_init(vdmx_Group_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_Group_move(dst: *mut vdmx_Group, src: *mut vdmx_Group) {
    cvec_move(vdmx_Group_as_cvec(dst), vdmx_Group_as_cvec(src));
}
pub static vdmx_iGroup: __caryll_vectorinterface_vdmx_Group = {
    __caryll_vectorinterface_vdmx_Group {
        init: Some(vdmx_Group_init as unsafe extern "C" fn(*mut vdmx_Group) -> ()),
        copy: Some(
            vdmx_Group_copy as unsafe extern "C" fn(*mut vdmx_Group, *const vdmx_Group) -> (),
        ),
        move_0: Some(
            vdmx_Group_move as unsafe extern "C" fn(*mut vdmx_Group, *mut vdmx_Group) -> (),
        ),
        dispose: Some(vdmx_Group_dispose as unsafe extern "C" fn(*mut vdmx_Group) -> ()),
        replace: Some(
            vdmx_Group_replace as unsafe extern "C" fn(*mut vdmx_Group, vdmx_Group) -> (),
        ),
        copyReplace: Some(
            vdmx_Group_copyReplace as unsafe extern "C" fn(*mut vdmx_Group, vdmx_Group) -> (),
        ),
        create: Some(vdmx_Group_create),
        free: Some(vdmx_Group_free as unsafe extern "C" fn(*mut vdmx_Group) -> ()),
        initN: Some(vdmx_Group_initN as unsafe extern "C" fn(*mut vdmx_Group, usize) -> ()),
        initCapN: Some(vdmx_Group_initCapN as unsafe extern "C" fn(*mut vdmx_Group, usize) -> ()),
        createN: Some(vdmx_Group_createN as unsafe extern "C" fn(usize) -> *mut vdmx_Group),
        fill: Some(vdmx_Group_fill as unsafe extern "C" fn(*mut vdmx_Group, usize) -> ()),
        clear: Some(vdmx_Group_dispose as unsafe extern "C" fn(*mut vdmx_Group) -> ()),
        push: Some(vdmx_Group_push as unsafe extern "C" fn(*mut vdmx_Group, vdmx_Record) -> ()),
        shrinkToFit: Some(vdmx_Group_shrinkToFit as unsafe extern "C" fn(*mut vdmx_Group) -> ()),
        pop: Some(vdmx_Group_pop as unsafe extern "C" fn(*mut vdmx_Group) -> vdmx_Record),
        disposeItem: Some(
            vdmx_Group_disposeItem as unsafe extern "C" fn(*mut vdmx_Group, usize) -> (),
        ),
        filterEnv: Some(
            vdmx_Group_filterEnv
                as unsafe extern "C" fn(
                    *mut vdmx_Group,
                    Option<
                        unsafe extern "C" fn(*const vdmx_Record, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            vdmx_Group_sort
                as unsafe extern "C" fn(
                    *mut vdmx_Group,
                    Option<
                        unsafe extern "C" fn(
                            *const vdmx_Record,
                            *const vdmx_Record,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn vdmx_Group_filterEnv(
    mut arr: *mut vdmx_Group,
    mut fn_0: Option<unsafe extern "C" fn(*const vdmx_Record, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut vdmx_Record,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if vdmx_iRecord.dispose.is_some() {
                vdmx_iRecord.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut vdmx_Record,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn vdmx_Group_disposeItem(mut arr: *mut vdmx_Group, mut n: usize) {
    if vdmx_iRecord.dispose.is_some() {
        vdmx_iRecord.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut vdmx_Record
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn vdmx_Group_sort(
    mut arr: *mut vdmx_Group,
    mut fn_0: Option<
        unsafe extern "C" fn(*const vdmx_Record, *const vdmx_Record) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<vdmx_Record>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const vdmx_Record, *const vdmx_Record) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
unsafe extern "C" fn initRR(mut rr: *mut vdmx_RatioRange) {
    memset(
        rr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<vdmx_RatioRange>() as usize,
    );
    vdmx_iGroup.init.expect("non-null function pointer")(&raw mut (*rr).records);
}
unsafe extern "C" fn disposeRR(mut rr: *mut vdmx_RatioRange) {
    vdmx_iGroup.dispose.expect("non-null function pointer")(&raw mut (*rr).records);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_dispose(mut x: *mut vdmx_RatioRange) {
    disposeRR(x);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_init(mut x: *mut vdmx_RatioRange) {
    initRR(x);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_move(
    mut dst: *mut vdmx_RatioRange,
    mut src: *mut vdmx_RatioRange,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vdmx_RatioRange>() as usize,
    );
    vdmx_RatioRange_init(src);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_copy(
    mut dst: *mut vdmx_RatioRange,
    mut src: *const vdmx_RatioRange,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vdmx_RatioRange>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_copyReplace(
    mut dst: *mut vdmx_RatioRange,
    src: vdmx_RatioRange,
) {
    vdmx_RatioRange_dispose(dst);
    vdmx_RatioRange_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_replace(mut dst: *mut vdmx_RatioRange, src: vdmx_RatioRange) {
    vdmx_RatioRange_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vdmx_RatioRange>() as usize,
    );
}
pub static vdmx_iRatioRange: __caryll_elementinterface_vdmx_RatioRange = {
    __caryll_elementinterface_vdmx_RatioRange {
        init: Some(vdmx_RatioRange_init as unsafe extern "C" fn(*mut vdmx_RatioRange) -> ()),
        copy: Some(
            vdmx_RatioRange_copy
                as unsafe extern "C" fn(*mut vdmx_RatioRange, *const vdmx_RatioRange) -> (),
        ),
        move_0: Some(
            vdmx_RatioRange_move
                as unsafe extern "C" fn(*mut vdmx_RatioRange, *mut vdmx_RatioRange) -> (),
        ),
        dispose: Some(vdmx_RatioRange_dispose as unsafe extern "C" fn(*mut vdmx_RatioRange) -> ()),
        replace: Some(
            vdmx_RatioRange_replace
                as unsafe extern "C" fn(*mut vdmx_RatioRange, vdmx_RatioRange) -> (),
        ),
        copyReplace: Some(
            vdmx_RatioRange_copyReplace
                as unsafe extern "C" fn(*mut vdmx_RatioRange, vdmx_RatioRange) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_disposeItem(
    mut arr: *mut vdmx_RatioRagneList,
    mut n: usize,
) {
    if vdmx_iRatioRange.dispose.is_some() {
        vdmx_iRatioRange.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut vdmx_RatioRange,
        );
    } else {
    };
}
pub static vdmx_iRatioRangeList: __caryll_vectorinterface_vdmx_RatioRagneList = {
    __caryll_vectorinterface_vdmx_RatioRagneList {
        init: Some(
            vdmx_RatioRagneList_init as unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> (),
        ),
        copy: Some(
            vdmx_RatioRagneList_copy
                as unsafe extern "C" fn(*mut vdmx_RatioRagneList, *const vdmx_RatioRagneList) -> (),
        ),
        move_0: Some(
            vdmx_RatioRagneList_move
                as unsafe extern "C" fn(*mut vdmx_RatioRagneList, *mut vdmx_RatioRagneList) -> (),
        ),
        dispose: Some(
            vdmx_RatioRagneList_dispose as unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> (),
        ),
        replace: Some(
            vdmx_RatioRagneList_replace
                as unsafe extern "C" fn(*mut vdmx_RatioRagneList, vdmx_RatioRagneList) -> (),
        ),
        copyReplace: Some(
            vdmx_RatioRagneList_copyReplace
                as unsafe extern "C" fn(*mut vdmx_RatioRagneList, vdmx_RatioRagneList) -> (),
        ),
        create: Some(vdmx_RatioRagneList_create),
        free: Some(
            vdmx_RatioRagneList_free as unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> (),
        ),
        initN: Some(
            vdmx_RatioRagneList_initN
                as unsafe extern "C" fn(*mut vdmx_RatioRagneList, usize) -> (),
        ),
        initCapN: Some(
            vdmx_RatioRagneList_initCapN
                as unsafe extern "C" fn(*mut vdmx_RatioRagneList, usize) -> (),
        ),
        createN: Some(
            vdmx_RatioRagneList_createN as unsafe extern "C" fn(usize) -> *mut vdmx_RatioRagneList,
        ),
        fill: Some(
            vdmx_RatioRagneList_fill
                as unsafe extern "C" fn(*mut vdmx_RatioRagneList, usize) -> (),
        ),
        clear: Some(
            vdmx_RatioRagneList_dispose as unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> (),
        ),
        push: Some(
            vdmx_RatioRagneList_push
                as unsafe extern "C" fn(*mut vdmx_RatioRagneList, vdmx_RatioRange) -> (),
        ),
        shrinkToFit: Some(
            vdmx_RatioRagneList_shrinkToFit as unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> (),
        ),
        pop: Some(
            vdmx_RatioRagneList_pop
                as unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> vdmx_RatioRange,
        ),
        disposeItem: Some(
            vdmx_RatioRagneList_disposeItem
                as unsafe extern "C" fn(*mut vdmx_RatioRagneList, usize) -> (),
        ),
        filterEnv: Some(
            vdmx_RatioRagneList_filterEnv
                as unsafe extern "C" fn(
                    *mut vdmx_RatioRagneList,
                    Option<
                        unsafe extern "C" fn(
                            *const vdmx_RatioRange,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            vdmx_RatioRagneList_sort
                as unsafe extern "C" fn(
                    *mut vdmx_RatioRagneList,
                    Option<
                        unsafe extern "C" fn(
                            *const vdmx_RatioRange,
                            *const vdmx_RatioRange,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_shrinkToFit(mut arr: *mut vdmx_RatioRagneList) {
    vdmx_RatioRagneList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_resizeTo(arr: *mut vdmx_RatioRagneList, target: usize) {
    cvec_resize_to(vdmx_RatioRagneList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_move(dst: *mut vdmx_RatioRagneList, src: *mut vdmx_RatioRagneList) {
    cvec_move(vdmx_RatioRagneList_as_cvec(dst), vdmx_RatioRagneList_as_cvec(src));
}
#[inline]
unsafe fn vdmx_RatioRagneList_as_cvec(arr: *mut vdmx_RatioRagneList) -> *mut CVecRaw<vdmx_RatioRange> {
    arr as *mut CVecRaw<vdmx_RatioRange>
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_init(arr: *mut vdmx_RatioRagneList) {
    cvec_init(vdmx_RatioRagneList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_filterEnv(
    mut arr: *mut vdmx_RatioRagneList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const vdmx_RatioRange, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut vdmx_RatioRange,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if vdmx_iRatioRange.dispose.is_some() {
                vdmx_iRatioRange.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut vdmx_RatioRange,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_sort(
    mut arr: *mut vdmx_RatioRagneList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const vdmx_RatioRange, *const vdmx_RatioRange) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<vdmx_RatioRange>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const vdmx_RatioRange,
                    *const vdmx_RatioRange,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_fill(mut arr: *mut vdmx_RatioRagneList, mut n: usize) {
    while (*arr).length < n {
        let mut x: vdmx_RatioRange = vdmx_RatioRange {
            bCharset: 0,
            xRatio: 0,
            yStartRatio: 0,
            yEndRatio: 0,
            records: vdmx_Group {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vdmx_Record>(),
            },
        };
        if vdmx_iRatioRange.init.is_some() {
            vdmx_iRatioRange.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<vdmx_RatioRange>() as usize,
            );
        }
        vdmx_RatioRagneList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_push(arr: *mut vdmx_RatioRagneList, elem: vdmx_RatioRange) {
    cvec_push(vdmx_RatioRagneList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_grow(arr: *mut vdmx_RatioRagneList) {
    cvec_grow(vdmx_RatioRagneList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_growTo(arr: *mut vdmx_RatioRagneList, target: usize) {
    cvec_grow_to(vdmx_RatioRagneList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_pop(arr: *mut vdmx_RatioRagneList) -> vdmx_RatioRange {
    cvec_pop(vdmx_RatioRagneList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_copyReplace(
    mut dst: *mut vdmx_RatioRagneList,
    src: vdmx_RatioRagneList,
) {
    vdmx_RatioRagneList_dispose(dst);
    vdmx_RatioRagneList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_copy(
    mut dst: *mut vdmx_RatioRagneList,
    mut src: *const vdmx_RatioRagneList,
) {
    vdmx_RatioRagneList_init(dst);
    vdmx_RatioRagneList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if vdmx_iRatioRange.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            vdmx_iRatioRange.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut vdmx_RatioRange,
                (*src).items.offset(j as isize) as *mut vdmx_RatioRange as *const vdmx_RatioRange,
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
unsafe extern "C" fn vdmx_RatioRagneList_dispose(mut arr: *mut vdmx_RatioRagneList) {
    if arr.is_null() {
        return;
    }
    if vdmx_iRatioRange.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            vdmx_iRatioRange.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut vdmx_RatioRange,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<vdmx_RatioRange>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_replace(
    mut dst: *mut vdmx_RatioRagneList,
    src: vdmx_RatioRagneList,
) {
    vdmx_RatioRagneList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<vdmx_RatioRagneList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_initCapN(
    mut arr: *mut vdmx_RatioRagneList,
    mut n: usize,
) {
    vdmx_RatioRagneList_init(arr);
    vdmx_RatioRagneList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_growToN(arr: *mut vdmx_RatioRagneList, target: usize) {
    cvec_grow_to_n(vdmx_RatioRagneList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_initN(mut arr: *mut vdmx_RatioRagneList, mut n: usize) {
    vdmx_RatioRagneList_init(arr);
    vdmx_RatioRagneList_growToN(arr, n);
    vdmx_RatioRagneList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_free(mut x: *mut vdmx_RatioRagneList) {
    if x.is_null() {
        return;
    }
    vdmx_RatioRagneList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_createN(mut n: usize) -> *mut vdmx_RatioRagneList {
    let mut t: *mut vdmx_RatioRagneList =
        malloc(::core::mem::size_of::<vdmx_RatioRagneList>() as usize) as *mut vdmx_RatioRagneList;
    vdmx_RatioRagneList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_create() -> *mut vdmx_RatioRagneList {
    let mut x: *mut vdmx_RatioRagneList =
        malloc(::core::mem::size_of::<vdmx_RatioRagneList>() as usize) as *mut vdmx_RatioRagneList;
    vdmx_RatioRagneList_init(x);
    return x;
}
unsafe extern "C" fn initVDMX(mut t: *mut table_VDMX) {
    vdmx_iRatioRangeList
        .init
        .expect("non-null function pointer")(&raw mut (*t).ratios);
}
unsafe extern "C" fn disposeVDMX(mut t: *mut table_VDMX) {
    vdmx_iRatioRangeList
        .dispose
        .expect("non-null function pointer")(&raw mut (*t).ratios);
}
#[inline]
unsafe extern "C" fn table_VDMX_free(mut x: *mut table_VDMX) {
    if x.is_null() {
        return;
    }
    table_VDMX_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_VDMX_move(mut dst: *mut table_VDMX, mut src: *mut table_VDMX) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_VDMX>() as usize,
    );
    table_VDMX_init(src);
}
#[inline]
unsafe extern "C" fn table_VDMX_copy(mut dst: *mut table_VDMX, mut src: *const table_VDMX) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_VDMX>() as usize,
    );
}
pub static table_iVDMX: __caryll_elementinterface_table_VDMX = {
    __caryll_elementinterface_table_VDMX {
        init: Some(table_VDMX_init as unsafe extern "C" fn(*mut table_VDMX) -> ()),
        copy: Some(
            table_VDMX_copy as unsafe extern "C" fn(*mut table_VDMX, *const table_VDMX) -> (),
        ),
        move_0: Some(
            table_VDMX_move as unsafe extern "C" fn(*mut table_VDMX, *mut table_VDMX) -> (),
        ),
        dispose: Some(table_VDMX_dispose as unsafe extern "C" fn(*mut table_VDMX) -> ()),
        replace: Some(
            table_VDMX_replace as unsafe extern "C" fn(*mut table_VDMX, table_VDMX) -> (),
        ),
        copyReplace: Some(
            table_VDMX_copyReplace as unsafe extern "C" fn(*mut table_VDMX, table_VDMX) -> (),
        ),
        create: Some(table_VDMX_create),
        free: Some(table_VDMX_free as unsafe extern "C" fn(*mut table_VDMX) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_VDMX_init(mut x: *mut table_VDMX) {
    initVDMX(x);
}
#[inline]
unsafe extern "C" fn table_VDMX_dispose(mut x: *mut table_VDMX) {
    disposeVDMX(x);
}
#[inline]
unsafe extern "C" fn table_VDMX_replace(mut dst: *mut table_VDMX, src: table_VDMX) {
    table_VDMX_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_VDMX>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_VDMX_create() -> *mut table_VDMX {
    let mut x: *mut table_VDMX =
        malloc(::core::mem::size_of::<table_VDMX>() as usize) as *mut table_VDMX;
    table_VDMX_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_VDMX_copyReplace(mut dst: *mut table_VDMX, src: table_VDMX) {
    table_VDMX_dispose(dst);
    table_VDMX_copy(dst, &raw const src);
}
