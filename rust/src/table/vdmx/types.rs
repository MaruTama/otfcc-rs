#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::{ComparFn};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRecord {
    pub yPelHeight: u16,
    pub yMax: i16,
    pub yMin: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRecordElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VdmxRecord) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VdmxRecord, *const VdmxRecord) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VdmxRecord, *mut VdmxRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxRecord) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VdmxRecord, VdmxRecord) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VdmxRecord, VdmxRecord) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> VdmxRecord>,
    pub dup: Option<unsafe extern "C" fn(VdmxRecord) -> VdmxRecord>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxGroup {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut VdmxRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxGroupVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut VdmxGroup) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VdmxGroup, *const VdmxGroup) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VdmxGroup, *mut VdmxGroup) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxGroup) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VdmxGroup, VdmxGroup) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VdmxGroup, VdmxGroup) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VdmxGroup>,
    pub free: Option<unsafe extern "C" fn(*mut VdmxGroup) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut VdmxGroup, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut VdmxGroup, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut VdmxGroup>,
    pub fill: Option<unsafe extern "C" fn(*mut VdmxGroup, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut VdmxGroup) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VdmxGroup, VdmxRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut VdmxGroup) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut VdmxGroup) -> VdmxRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut VdmxGroup, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut VdmxGroup,
            Option<unsafe extern "C" fn(*const VdmxRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut VdmxGroup,
            Option<
                unsafe extern "C" fn(*const VdmxRecord, *const VdmxRecord) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRatioRange {
    pub bCharset: u8,
    pub xRatio: u8,
    pub yStartRatio: u8,
    pub yEndRatio: u8,
    pub records: VdmxGroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRatioRangeElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VdmxRatioRange) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VdmxRatioRange, *const VdmxRatioRange) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VdmxRatioRange, *mut VdmxRatioRange) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxRatioRange) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VdmxRatioRange, VdmxRatioRange) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VdmxRatioRange, VdmxRatioRange) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRatioRangeList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut VdmxRatioRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRatioRangeListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut VdmxRatioRangeList, *const VdmxRatioRangeList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut VdmxRatioRangeList, *mut VdmxRatioRangeList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRangeList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRangeList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VdmxRatioRangeList>,
    pub free: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut VdmxRatioRangeList>,
    pub fill: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRange) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList) -> VdmxRatioRange>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut VdmxRatioRangeList,
            Option<unsafe extern "C" fn(*const VdmxRatioRange, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut VdmxRatioRangeList,
            Option<
                unsafe extern "C" fn(
                    *const VdmxRatioRange,
                    *const VdmxRatioRange,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxTable {
    pub version: u16,
    pub ratios: VdmxRatioRangeList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VdmxTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VdmxTable, *const VdmxTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VdmxTable, *mut VdmxTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VdmxTable, VdmxTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VdmxTable, VdmxTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VdmxTable>,
    pub free: Option<unsafe extern "C" fn(*mut VdmxTable) -> ()>,
}
#[inline]
unsafe extern "C" fn vdmx_Record_init(mut x: *mut VdmxRecord) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VdmxRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_Record_copy(mut dst: *mut VdmxRecord, mut src: *const VdmxRecord) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_Record_empty() -> VdmxRecord {
    let mut x: VdmxRecord = VdmxRecord {
        yPelHeight: 0,
        yMax: 0,
        yMin: 0,
    };
    vdmx_Record_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn vdmx_Record_replace(mut dst: *mut VdmxRecord, src: VdmxRecord) {
    vdmx_Record_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_Record_dispose(mut _x: *mut VdmxRecord) {}
#[inline]
unsafe extern "C" fn vdmx_Record_move(mut dst: *mut VdmxRecord, mut src: *mut VdmxRecord) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRecord>() as usize,
    );
    vdmx_Record_init(src);
}
#[inline]
unsafe extern "C" fn vdmx_Record_dup(src: VdmxRecord) -> VdmxRecord {
    let mut dst: VdmxRecord = VdmxRecord {
        yPelHeight: 0,
        yMax: 0,
        yMin: 0,
    };
    vdmx_Record_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn vdmx_Record_copyReplace(mut dst: *mut VdmxRecord, src: VdmxRecord) {
    vdmx_Record_dispose(dst);
    vdmx_Record_copy(dst, &raw const src);
}
pub static vdmx_iRecord: VdmxRecordElementInterface = {
    VdmxRecordElementInterface {
        init: Some(vdmx_Record_init as unsafe extern "C" fn(*mut VdmxRecord) -> ()),
        copy: Some(
            vdmx_Record_copy as unsafe extern "C" fn(*mut VdmxRecord, *const VdmxRecord) -> (),
        ),
        move_0: Some(
            vdmx_Record_move as unsafe extern "C" fn(*mut VdmxRecord, *mut VdmxRecord) -> (),
        ),
        dispose: Some(vdmx_Record_dispose as unsafe extern "C" fn(*mut VdmxRecord) -> ()),
        replace: Some(
            vdmx_Record_replace as unsafe extern "C" fn(*mut VdmxRecord, VdmxRecord) -> (),
        ),
        copyReplace: Some(
            vdmx_Record_copyReplace as unsafe extern "C" fn(*mut VdmxRecord, VdmxRecord) -> (),
        ),
        empty: Some(vdmx_Record_empty),
        dup: Some(vdmx_Record_dup as unsafe extern "C" fn(VdmxRecord) -> VdmxRecord),
    }
};
#[inline]
unsafe extern "C" fn vdmx_Group_createN(mut n: usize) -> *mut VdmxGroup {
    let mut t: *mut VdmxGroup =
        malloc(::core::mem::size_of::<VdmxGroup>() as usize) as *mut VdmxGroup;
    vdmx_Group_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn vdmx_Group_push(arr: *mut VdmxGroup, elem: VdmxRecord) {
    cvec_push(vdmx_Group_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vdmx_Group_grow(arr: *mut VdmxGroup) {
    cvec_grow(vdmx_Group_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_Group_growTo(arr: *mut VdmxGroup, target: usize) {
    cvec_grow_to(vdmx_Group_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_Group_pop(arr: *mut VdmxGroup) -> VdmxRecord {
    cvec_pop(vdmx_Group_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn vdmx_Group_copyReplace(mut dst: *mut VdmxGroup, src: VdmxGroup) {
    vdmx_Group_dispose(dst);
    vdmx_Group_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vdmx_Group_copy(mut dst: *mut VdmxGroup, mut src: *const VdmxGroup) {
    vdmx_Group_init(dst);
    vdmx_Group_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if vdmx_iRecord.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            vdmx_iRecord.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut VdmxRecord,
                (*src).items.offset(j as isize) as *mut VdmxRecord as *const VdmxRecord,
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
unsafe extern "C" fn vdmx_Group_dispose(mut arr: *mut VdmxGroup) {
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
                (*arr).items.offset(j as isize) as *mut VdmxRecord,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<VdmxRecord>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn vdmx_Group_replace(mut dst: *mut VdmxGroup, src: VdmxGroup) {
    vdmx_Group_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxGroup>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_Group_initCapN(mut arr: *mut VdmxGroup, mut n: usize) {
    vdmx_Group_init(arr);
    vdmx_Group_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_Group_growToN(arr: *mut VdmxGroup, target: usize) {
    cvec_grow_to_n(vdmx_Group_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_Group_initN(mut arr: *mut VdmxGroup, mut n: usize) {
    vdmx_Group_init(arr);
    vdmx_Group_growToN(arr, n);
    vdmx_Group_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_Group_free(mut x: *mut VdmxGroup) {
    if x.is_null() {
        return;
    }
    vdmx_Group_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vdmx_Group_shrinkToFit(mut arr: *mut VdmxGroup) {
    vdmx_Group_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vdmx_Group_create() -> *mut VdmxGroup {
    let mut x: *mut VdmxGroup =
        malloc(::core::mem::size_of::<VdmxGroup>() as usize) as *mut VdmxGroup;
    vdmx_Group_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn vdmx_Group_resizeTo(arr: *mut VdmxGroup, target: usize) {
    cvec_resize_to(vdmx_Group_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_Group_fill(mut arr: *mut VdmxGroup, mut n: usize) {
    while (*arr).length < n {
        let mut x: VdmxRecord = VdmxRecord {
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
                ::core::mem::size_of::<VdmxRecord>() as usize,
            );
        }
        vdmx_Group_push(arr, x);
    }
}
#[inline]
unsafe fn vdmx_Group_as_cvec(arr: *mut VdmxGroup) -> *mut CVecRaw<VdmxRecord> {
    arr as *mut CVecRaw<VdmxRecord>
}
#[inline]
unsafe extern "C" fn vdmx_Group_init(arr: *mut VdmxGroup) {
    cvec_init(vdmx_Group_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_Group_move(dst: *mut VdmxGroup, src: *mut VdmxGroup) {
    cvec_move(vdmx_Group_as_cvec(dst), vdmx_Group_as_cvec(src));
}
pub static vdmx_iGroup: VdmxGroupVectorInterface = {
    VdmxGroupVectorInterface {
        init: Some(vdmx_Group_init as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        copy: Some(
            vdmx_Group_copy as unsafe extern "C" fn(*mut VdmxGroup, *const VdmxGroup) -> (),
        ),
        move_0: Some(
            vdmx_Group_move as unsafe extern "C" fn(*mut VdmxGroup, *mut VdmxGroup) -> (),
        ),
        dispose: Some(vdmx_Group_dispose as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        replace: Some(
            vdmx_Group_replace as unsafe extern "C" fn(*mut VdmxGroup, VdmxGroup) -> (),
        ),
        copyReplace: Some(
            vdmx_Group_copyReplace as unsafe extern "C" fn(*mut VdmxGroup, VdmxGroup) -> (),
        ),
        create: Some(vdmx_Group_create),
        free: Some(vdmx_Group_free as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        initN: Some(vdmx_Group_initN as unsafe extern "C" fn(*mut VdmxGroup, usize) -> ()),
        initCapN: Some(vdmx_Group_initCapN as unsafe extern "C" fn(*mut VdmxGroup, usize) -> ()),
        createN: Some(vdmx_Group_createN as unsafe extern "C" fn(usize) -> *mut VdmxGroup),
        fill: Some(vdmx_Group_fill as unsafe extern "C" fn(*mut VdmxGroup, usize) -> ()),
        clear: Some(vdmx_Group_dispose as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        push: Some(vdmx_Group_push as unsafe extern "C" fn(*mut VdmxGroup, VdmxRecord) -> ()),
        shrinkToFit: Some(vdmx_Group_shrinkToFit as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        pop: Some(vdmx_Group_pop as unsafe extern "C" fn(*mut VdmxGroup) -> VdmxRecord),
        disposeItem: Some(
            vdmx_Group_disposeItem as unsafe extern "C" fn(*mut VdmxGroup, usize) -> (),
        ),
        filterEnv: Some(
            vdmx_Group_filterEnv
                as unsafe extern "C" fn(
                    *mut VdmxGroup,
                    Option<
                        unsafe extern "C" fn(*const VdmxRecord, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            vdmx_Group_sort
                as unsafe extern "C" fn(
                    *mut VdmxGroup,
                    Option<
                        unsafe extern "C" fn(
                            *const VdmxRecord,
                            *const VdmxRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn vdmx_Group_filterEnv(
    mut arr: *mut VdmxGroup,
    mut fn_0: Option<unsafe extern "C" fn(*const VdmxRecord, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut VdmxRecord,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if vdmx_iRecord.dispose.is_some() {
                vdmx_iRecord.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut VdmxRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn vdmx_Group_disposeItem(mut arr: *mut VdmxGroup, mut n: usize) {
    if vdmx_iRecord.dispose.is_some() {
        vdmx_iRecord.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut VdmxRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn vdmx_Group_sort(
    mut arr: *mut VdmxGroup,
    mut fn_0: Option<
        unsafe extern "C" fn(*const VdmxRecord, *const VdmxRecord) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<VdmxRecord>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const VdmxRecord, *const VdmxRecord) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
unsafe extern "C" fn initRR(mut rr: *mut VdmxRatioRange) {
    memset(
        rr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VdmxRatioRange>() as usize,
    );
    vdmx_iGroup.init.expect("non-null function pointer")(&raw mut (*rr).records);
}
unsafe extern "C" fn disposeRR(mut rr: *mut VdmxRatioRange) {
    vdmx_iGroup.dispose.expect("non-null function pointer")(&raw mut (*rr).records);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_dispose(mut x: *mut VdmxRatioRange) {
    disposeRR(x);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_init(mut x: *mut VdmxRatioRange) {
    initRR(x);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_move(
    mut dst: *mut VdmxRatioRange,
    mut src: *mut VdmxRatioRange,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRatioRange>() as usize,
    );
    vdmx_RatioRange_init(src);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_copy(
    mut dst: *mut VdmxRatioRange,
    mut src: *const VdmxRatioRange,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRatioRange>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_copyReplace(
    mut dst: *mut VdmxRatioRange,
    src: VdmxRatioRange,
) {
    vdmx_RatioRange_dispose(dst);
    vdmx_RatioRange_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRange_replace(mut dst: *mut VdmxRatioRange, src: VdmxRatioRange) {
    vdmx_RatioRange_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRatioRange>() as usize,
    );
}
pub static vdmx_iRatioRange: VdmxRatioRangeElementInterface = {
    VdmxRatioRangeElementInterface {
        init: Some(vdmx_RatioRange_init as unsafe extern "C" fn(*mut VdmxRatioRange) -> ()),
        copy: Some(
            vdmx_RatioRange_copy
                as unsafe extern "C" fn(*mut VdmxRatioRange, *const VdmxRatioRange) -> (),
        ),
        move_0: Some(
            vdmx_RatioRange_move
                as unsafe extern "C" fn(*mut VdmxRatioRange, *mut VdmxRatioRange) -> (),
        ),
        dispose: Some(vdmx_RatioRange_dispose as unsafe extern "C" fn(*mut VdmxRatioRange) -> ()),
        replace: Some(
            vdmx_RatioRange_replace
                as unsafe extern "C" fn(*mut VdmxRatioRange, VdmxRatioRange) -> (),
        ),
        copyReplace: Some(
            vdmx_RatioRange_copyReplace
                as unsafe extern "C" fn(*mut VdmxRatioRange, VdmxRatioRange) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_disposeItem(
    mut arr: *mut VdmxRatioRangeList,
    mut n: usize,
) {
    if vdmx_iRatioRange.dispose.is_some() {
        vdmx_iRatioRange.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut VdmxRatioRange,
        );
    } else {
    };
}
pub static vdmx_iRatioRangeList: VdmxRatioRangeListVectorInterface = {
    VdmxRatioRangeListVectorInterface {
        init: Some(
            vdmx_RatioRagneList_init as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        copy: Some(
            vdmx_RatioRagneList_copy
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, *const VdmxRatioRangeList) -> (),
        ),
        move_0: Some(
            vdmx_RatioRagneList_move
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, *mut VdmxRatioRangeList) -> (),
        ),
        dispose: Some(
            vdmx_RatioRagneList_dispose as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        replace: Some(
            vdmx_RatioRagneList_replace
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRangeList) -> (),
        ),
        copyReplace: Some(
            vdmx_RatioRagneList_copyReplace
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRangeList) -> (),
        ),
        create: Some(vdmx_RatioRagneList_create),
        free: Some(
            vdmx_RatioRagneList_free as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        initN: Some(
            vdmx_RatioRagneList_initN
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> (),
        ),
        initCapN: Some(
            vdmx_RatioRagneList_initCapN
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> (),
        ),
        createN: Some(
            vdmx_RatioRagneList_createN as unsafe extern "C" fn(usize) -> *mut VdmxRatioRangeList,
        ),
        fill: Some(
            vdmx_RatioRagneList_fill
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> (),
        ),
        clear: Some(
            vdmx_RatioRagneList_dispose as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        push: Some(
            vdmx_RatioRagneList_push
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRange) -> (),
        ),
        shrinkToFit: Some(
            vdmx_RatioRagneList_shrinkToFit as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        pop: Some(
            vdmx_RatioRagneList_pop
                as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> VdmxRatioRange,
        ),
        disposeItem: Some(
            vdmx_RatioRagneList_disposeItem
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> (),
        ),
        filterEnv: Some(
            vdmx_RatioRagneList_filterEnv
                as unsafe extern "C" fn(
                    *mut VdmxRatioRangeList,
                    Option<
                        unsafe extern "C" fn(
                            *const VdmxRatioRange,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            vdmx_RatioRagneList_sort
                as unsafe extern "C" fn(
                    *mut VdmxRatioRangeList,
                    Option<
                        unsafe extern "C" fn(
                            *const VdmxRatioRange,
                            *const VdmxRatioRange,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_shrinkToFit(mut arr: *mut VdmxRatioRangeList) {
    vdmx_RatioRagneList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_resizeTo(arr: *mut VdmxRatioRangeList, target: usize) {
    cvec_resize_to(vdmx_RatioRagneList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_move(dst: *mut VdmxRatioRangeList, src: *mut VdmxRatioRangeList) {
    cvec_move(vdmx_RatioRagneList_as_cvec(dst), vdmx_RatioRagneList_as_cvec(src));
}
#[inline]
unsafe fn vdmx_RatioRagneList_as_cvec(arr: *mut VdmxRatioRangeList) -> *mut CVecRaw<VdmxRatioRange> {
    arr as *mut CVecRaw<VdmxRatioRange>
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_init(arr: *mut VdmxRatioRangeList) {
    cvec_init(vdmx_RatioRagneList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_filterEnv(
    mut arr: *mut VdmxRatioRangeList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const VdmxRatioRange, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut VdmxRatioRange,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if vdmx_iRatioRange.dispose.is_some() {
                vdmx_iRatioRange.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut VdmxRatioRange,
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
    mut arr: *mut VdmxRatioRangeList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const VdmxRatioRange, *const VdmxRatioRange) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<VdmxRatioRange>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const VdmxRatioRange,
                    *const VdmxRatioRange,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_fill(mut arr: *mut VdmxRatioRangeList, mut n: usize) {
    while (*arr).length < n {
        let mut x: VdmxRatioRange = VdmxRatioRange {
            bCharset: 0,
            xRatio: 0,
            yStartRatio: 0,
            yEndRatio: 0,
            records: VdmxGroup {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<VdmxRecord>(),
            },
        };
        if vdmx_iRatioRange.init.is_some() {
            vdmx_iRatioRange.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VdmxRatioRange>() as usize,
            );
        }
        vdmx_RatioRagneList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_push(arr: *mut VdmxRatioRangeList, elem: VdmxRatioRange) {
    cvec_push(vdmx_RatioRagneList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_grow(arr: *mut VdmxRatioRangeList) {
    cvec_grow(vdmx_RatioRagneList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_growTo(arr: *mut VdmxRatioRangeList, target: usize) {
    cvec_grow_to(vdmx_RatioRagneList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_pop(arr: *mut VdmxRatioRangeList) -> VdmxRatioRange {
    cvec_pop(vdmx_RatioRagneList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_copyReplace(
    mut dst: *mut VdmxRatioRangeList,
    src: VdmxRatioRangeList,
) {
    vdmx_RatioRagneList_dispose(dst);
    vdmx_RatioRagneList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_copy(
    mut dst: *mut VdmxRatioRangeList,
    mut src: *const VdmxRatioRangeList,
) {
    vdmx_RatioRagneList_init(dst);
    vdmx_RatioRagneList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if vdmx_iRatioRange.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            vdmx_iRatioRange.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut VdmxRatioRange,
                (*src).items.offset(j as isize) as *mut VdmxRatioRange as *const VdmxRatioRange,
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
unsafe extern "C" fn vdmx_RatioRagneList_dispose(mut arr: *mut VdmxRatioRangeList) {
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
                (*arr).items.offset(j as isize) as *mut VdmxRatioRange,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<VdmxRatioRange>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_replace(
    mut dst: *mut VdmxRatioRangeList,
    src: VdmxRatioRangeList,
) {
    vdmx_RatioRagneList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRatioRangeList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_initCapN(
    mut arr: *mut VdmxRatioRangeList,
    mut n: usize,
) {
    vdmx_RatioRagneList_init(arr);
    vdmx_RatioRagneList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_growToN(arr: *mut VdmxRatioRangeList, target: usize) {
    cvec_grow_to_n(vdmx_RatioRagneList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_initN(mut arr: *mut VdmxRatioRangeList, mut n: usize) {
    vdmx_RatioRagneList_init(arr);
    vdmx_RatioRagneList_growToN(arr, n);
    vdmx_RatioRagneList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_free(mut x: *mut VdmxRatioRangeList) {
    if x.is_null() {
        return;
    }
    vdmx_RatioRagneList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_createN(mut n: usize) -> *mut VdmxRatioRangeList {
    let mut t: *mut VdmxRatioRangeList =
        malloc(::core::mem::size_of::<VdmxRatioRangeList>() as usize) as *mut VdmxRatioRangeList;
    vdmx_RatioRagneList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn vdmx_RatioRagneList_create() -> *mut VdmxRatioRangeList {
    let mut x: *mut VdmxRatioRangeList =
        malloc(::core::mem::size_of::<VdmxRatioRangeList>() as usize) as *mut VdmxRatioRangeList;
    vdmx_RatioRagneList_init(x);
    return x;
}
unsafe extern "C" fn initVDMX(mut t: *mut VdmxTable) {
    vdmx_iRatioRangeList
        .init
        .expect("non-null function pointer")(&raw mut (*t).ratios);
}
unsafe extern "C" fn disposeVDMX(mut t: *mut VdmxTable) {
    vdmx_iRatioRangeList
        .dispose
        .expect("non-null function pointer")(&raw mut (*t).ratios);
}
#[inline]
unsafe extern "C" fn table_VDMX_free(mut x: *mut VdmxTable) {
    if x.is_null() {
        return;
    }
    table_VDMX_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_VDMX_move(mut dst: *mut VdmxTable, mut src: *mut VdmxTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxTable>() as usize,
    );
    table_VDMX_init(src);
}
#[inline]
unsafe extern "C" fn table_VDMX_copy(mut dst: *mut VdmxTable, mut src: *const VdmxTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxTable>() as usize,
    );
}
pub static table_iVDMX: VdmxTableElementInterface = {
    VdmxTableElementInterface {
        init: Some(table_VDMX_init as unsafe extern "C" fn(*mut VdmxTable) -> ()),
        copy: Some(
            table_VDMX_copy as unsafe extern "C" fn(*mut VdmxTable, *const VdmxTable) -> (),
        ),
        move_0: Some(
            table_VDMX_move as unsafe extern "C" fn(*mut VdmxTable, *mut VdmxTable) -> (),
        ),
        dispose: Some(table_VDMX_dispose as unsafe extern "C" fn(*mut VdmxTable) -> ()),
        replace: Some(
            table_VDMX_replace as unsafe extern "C" fn(*mut VdmxTable, VdmxTable) -> (),
        ),
        copyReplace: Some(
            table_VDMX_copyReplace as unsafe extern "C" fn(*mut VdmxTable, VdmxTable) -> (),
        ),
        create: Some(table_VDMX_create),
        free: Some(table_VDMX_free as unsafe extern "C" fn(*mut VdmxTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_VDMX_init(mut x: *mut VdmxTable) {
    initVDMX(x);
}
#[inline]
unsafe extern "C" fn table_VDMX_dispose(mut x: *mut VdmxTable) {
    disposeVDMX(x);
}
#[inline]
unsafe extern "C" fn table_VDMX_replace(mut dst: *mut VdmxTable, src: VdmxTable) {
    table_VDMX_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_VDMX_create() -> *mut VdmxTable {
    let mut x: *mut VdmxTable =
        malloc(::core::mem::size_of::<VdmxTable>() as usize) as *mut VdmxTable;
    table_VDMX_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_VDMX_copyReplace(mut dst: *mut VdmxTable, src: VdmxTable) {
    table_VDMX_dispose(dst);
    table_VDMX_copy(dst, &raw const src);
}
