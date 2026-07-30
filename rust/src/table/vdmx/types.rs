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
unsafe extern "C" fn vdmx_record_init(mut x: *mut VdmxRecord) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VdmxRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_record_copy(mut dst: *mut VdmxRecord, mut src: *const VdmxRecord) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_record_empty() -> VdmxRecord {
    let mut x: VdmxRecord = VdmxRecord {
        yPelHeight: 0,
        yMax: 0,
        yMin: 0,
    };
    vdmx_record_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn vdmx_record_replace(mut dst: *mut VdmxRecord, src: VdmxRecord) {
    vdmx_record_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_record_dispose(mut _x: *mut VdmxRecord) {}
#[inline]
unsafe extern "C" fn vdmx_record_move(mut dst: *mut VdmxRecord, mut src: *mut VdmxRecord) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRecord>() as usize,
    );
    vdmx_record_init(src);
}
#[inline]
unsafe extern "C" fn vdmx_record_dup(src: VdmxRecord) -> VdmxRecord {
    let mut dst: VdmxRecord = VdmxRecord {
        yPelHeight: 0,
        yMax: 0,
        yMin: 0,
    };
    vdmx_record_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn vdmx_record_copy_replace(mut dst: *mut VdmxRecord, src: VdmxRecord) {
    vdmx_record_dispose(dst);
    vdmx_record_copy(dst, &raw const src);
}
pub static VDMX_I_RECORD: VdmxRecordElementInterface = {
    VdmxRecordElementInterface {
        init: Some(vdmx_record_init as unsafe extern "C" fn(*mut VdmxRecord) -> ()),
        copy: Some(
            vdmx_record_copy as unsafe extern "C" fn(*mut VdmxRecord, *const VdmxRecord) -> (),
        ),
        move_0: Some(
            vdmx_record_move as unsafe extern "C" fn(*mut VdmxRecord, *mut VdmxRecord) -> (),
        ),
        dispose: Some(vdmx_record_dispose as unsafe extern "C" fn(*mut VdmxRecord) -> ()),
        replace: Some(
            vdmx_record_replace as unsafe extern "C" fn(*mut VdmxRecord, VdmxRecord) -> (),
        ),
        copyReplace: Some(
            vdmx_record_copy_replace as unsafe extern "C" fn(*mut VdmxRecord, VdmxRecord) -> (),
        ),
        empty: Some(vdmx_record_empty),
        dup: Some(vdmx_record_dup as unsafe extern "C" fn(VdmxRecord) -> VdmxRecord),
    }
};
#[inline]
unsafe extern "C" fn vdmx_group_create_n(mut n: usize) -> *mut VdmxGroup {
    let mut t: *mut VdmxGroup =
        malloc(::core::mem::size_of::<VdmxGroup>() as usize) as *mut VdmxGroup;
    vdmx_group_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn vdmx_group_push(arr: *mut VdmxGroup, elem: VdmxRecord) {
    cvec_push(vdmx_group_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vdmx_group_grow(arr: *mut VdmxGroup) {
    cvec_grow(vdmx_group_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_group_grow_to(arr: *mut VdmxGroup, target: usize) {
    cvec_grow_to(vdmx_group_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_group_pop(arr: *mut VdmxGroup) -> VdmxRecord {
    cvec_pop(vdmx_group_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn vdmx_group_copy_replace(mut dst: *mut VdmxGroup, src: VdmxGroup) {
    vdmx_group_dispose(dst);
    vdmx_group_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vdmx_group_copy(mut dst: *mut VdmxGroup, mut src: *const VdmxGroup) {
    vdmx_group_init(dst);
    vdmx_group_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if VDMX_I_RECORD.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            VDMX_I_RECORD.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn vdmx_group_dispose(mut arr: *mut VdmxGroup) {
    if arr.is_null() {
        return;
    }
    if VDMX_I_RECORD.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            VDMX_I_RECORD.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn vdmx_group_replace(mut dst: *mut VdmxGroup, src: VdmxGroup) {
    vdmx_group_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxGroup>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_group_init_cap_n(mut arr: *mut VdmxGroup, mut n: usize) {
    vdmx_group_init(arr);
    vdmx_group_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_group_grow_to_n(arr: *mut VdmxGroup, target: usize) {
    cvec_grow_to_n(vdmx_group_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_group_init_n(mut arr: *mut VdmxGroup, mut n: usize) {
    vdmx_group_init(arr);
    vdmx_group_grow_to_n(arr, n);
    vdmx_group_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_group_free(mut x: *mut VdmxGroup) {
    if x.is_null() {
        return;
    }
    vdmx_group_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vdmx_group_shrink_to_fit(mut arr: *mut VdmxGroup) {
    vdmx_group_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vdmx_group_create() -> *mut VdmxGroup {
    let mut x: *mut VdmxGroup =
        malloc(::core::mem::size_of::<VdmxGroup>() as usize) as *mut VdmxGroup;
    vdmx_group_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn vdmx_group_resize_to(arr: *mut VdmxGroup, target: usize) {
    cvec_resize_to(vdmx_group_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_group_fill(mut arr: *mut VdmxGroup, mut n: usize) {
    while (*arr).length < n {
        let mut x: VdmxRecord = VdmxRecord {
            yPelHeight: 0,
            yMax: 0,
            yMin: 0,
        };
        if VDMX_I_RECORD.init.is_some() {
            VDMX_I_RECORD.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VdmxRecord>() as usize,
            );
        }
        vdmx_group_push(arr, x);
    }
}
#[inline]
unsafe fn vdmx_group_as_cvec(arr: *mut VdmxGroup) -> *mut CVecRaw<VdmxRecord> {
    arr as *mut CVecRaw<VdmxRecord>
}
#[inline]
unsafe extern "C" fn vdmx_group_init(arr: *mut VdmxGroup) {
    cvec_init(vdmx_group_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_group_move(dst: *mut VdmxGroup, src: *mut VdmxGroup) {
    cvec_move(vdmx_group_as_cvec(dst), vdmx_group_as_cvec(src));
}
pub static VDMX_I_GROUP: VdmxGroupVectorInterface = {
    VdmxGroupVectorInterface {
        init: Some(vdmx_group_init as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        copy: Some(
            vdmx_group_copy as unsafe extern "C" fn(*mut VdmxGroup, *const VdmxGroup) -> (),
        ),
        move_0: Some(
            vdmx_group_move as unsafe extern "C" fn(*mut VdmxGroup, *mut VdmxGroup) -> (),
        ),
        dispose: Some(vdmx_group_dispose as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        replace: Some(
            vdmx_group_replace as unsafe extern "C" fn(*mut VdmxGroup, VdmxGroup) -> (),
        ),
        copyReplace: Some(
            vdmx_group_copy_replace as unsafe extern "C" fn(*mut VdmxGroup, VdmxGroup) -> (),
        ),
        create: Some(vdmx_group_create),
        free: Some(vdmx_group_free as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        initN: Some(vdmx_group_init_n as unsafe extern "C" fn(*mut VdmxGroup, usize) -> ()),
        initCapN: Some(vdmx_group_init_cap_n as unsafe extern "C" fn(*mut VdmxGroup, usize) -> ()),
        createN: Some(vdmx_group_create_n as unsafe extern "C" fn(usize) -> *mut VdmxGroup),
        fill: Some(vdmx_group_fill as unsafe extern "C" fn(*mut VdmxGroup, usize) -> ()),
        clear: Some(vdmx_group_dispose as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        push: Some(vdmx_group_push as unsafe extern "C" fn(*mut VdmxGroup, VdmxRecord) -> ()),
        shrinkToFit: Some(vdmx_group_shrink_to_fit as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        pop: Some(vdmx_group_pop as unsafe extern "C" fn(*mut VdmxGroup) -> VdmxRecord),
        disposeItem: Some(
            vdmx_group_dispose_item as unsafe extern "C" fn(*mut VdmxGroup, usize) -> (),
        ),
        filterEnv: Some(
            vdmx_group_filter_env
                as unsafe extern "C" fn(
                    *mut VdmxGroup,
                    Option<
                        unsafe extern "C" fn(*const VdmxRecord, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            vdmx_group_sort
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
unsafe extern "C" fn vdmx_group_filter_env(
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
            if VDMX_I_RECORD.dispose.is_some() {
                VDMX_I_RECORD.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn vdmx_group_dispose_item(mut arr: *mut VdmxGroup, mut n: usize) {
    if VDMX_I_RECORD.dispose.is_some() {
        VDMX_I_RECORD.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut VdmxRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn vdmx_group_sort(
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
unsafe extern "C" fn init_rr(mut rr: *mut VdmxRatioRange) {
    memset(
        rr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VdmxRatioRange>() as usize,
    );
    VDMX_I_GROUP.init.expect("non-null function pointer")(&raw mut (*rr).records);
}
unsafe extern "C" fn dispose_rr(mut rr: *mut VdmxRatioRange) {
    VDMX_I_GROUP.dispose.expect("non-null function pointer")(&raw mut (*rr).records);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_range_dispose(mut x: *mut VdmxRatioRange) {
    dispose_rr(x);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_range_init(mut x: *mut VdmxRatioRange) {
    init_rr(x);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_range_move(
    mut dst: *mut VdmxRatioRange,
    mut src: *mut VdmxRatioRange,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRatioRange>() as usize,
    );
    vdmx_ratio_range_init(src);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_range_copy(
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
unsafe extern "C" fn vdmx_ratio_range_copy_replace(
    mut dst: *mut VdmxRatioRange,
    src: VdmxRatioRange,
) {
    vdmx_ratio_range_dispose(dst);
    vdmx_ratio_range_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_range_replace(mut dst: *mut VdmxRatioRange, src: VdmxRatioRange) {
    vdmx_ratio_range_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRatioRange>() as usize,
    );
}
pub static VDMX_I_RATIO_RANGE: VdmxRatioRangeElementInterface = {
    VdmxRatioRangeElementInterface {
        init: Some(vdmx_ratio_range_init as unsafe extern "C" fn(*mut VdmxRatioRange) -> ()),
        copy: Some(
            vdmx_ratio_range_copy
                as unsafe extern "C" fn(*mut VdmxRatioRange, *const VdmxRatioRange) -> (),
        ),
        move_0: Some(
            vdmx_ratio_range_move
                as unsafe extern "C" fn(*mut VdmxRatioRange, *mut VdmxRatioRange) -> (),
        ),
        dispose: Some(vdmx_ratio_range_dispose as unsafe extern "C" fn(*mut VdmxRatioRange) -> ()),
        replace: Some(
            vdmx_ratio_range_replace
                as unsafe extern "C" fn(*mut VdmxRatioRange, VdmxRatioRange) -> (),
        ),
        copyReplace: Some(
            vdmx_ratio_range_copy_replace
                as unsafe extern "C" fn(*mut VdmxRatioRange, VdmxRatioRange) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_dispose_item(
    mut arr: *mut VdmxRatioRangeList,
    mut n: usize,
) {
    if VDMX_I_RATIO_RANGE.dispose.is_some() {
        VDMX_I_RATIO_RANGE.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut VdmxRatioRange,
        );
    } else {
    };
}
pub static VDMX_I_RATIO_RANGE_LIST: VdmxRatioRangeListVectorInterface = {
    VdmxRatioRangeListVectorInterface {
        init: Some(
            vdmx_ratio_ragne_list_init as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        copy: Some(
            vdmx_ratio_ragne_list_copy
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, *const VdmxRatioRangeList) -> (),
        ),
        move_0: Some(
            vdmx_ratio_ragne_list_move
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, *mut VdmxRatioRangeList) -> (),
        ),
        dispose: Some(
            vdmx_ratio_ragne_list_dispose as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        replace: Some(
            vdmx_ratio_ragne_list_replace
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRangeList) -> (),
        ),
        copyReplace: Some(
            vdmx_ratio_ragne_list_copy_replace
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRangeList) -> (),
        ),
        create: Some(vdmx_ratio_ragne_list_create),
        free: Some(
            vdmx_ratio_ragne_list_free as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        initN: Some(
            vdmx_ratio_ragne_list_init_n
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> (),
        ),
        initCapN: Some(
            vdmx_ratio_ragne_list_init_cap_n
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> (),
        ),
        createN: Some(
            vdmx_ratio_ragne_list_create_n as unsafe extern "C" fn(usize) -> *mut VdmxRatioRangeList,
        ),
        fill: Some(
            vdmx_ratio_ragne_list_fill
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> (),
        ),
        clear: Some(
            vdmx_ratio_ragne_list_dispose as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        push: Some(
            vdmx_ratio_ragne_list_push
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRange) -> (),
        ),
        shrinkToFit: Some(
            vdmx_ratio_ragne_list_shrink_to_fit as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        pop: Some(
            vdmx_ratio_ragne_list_pop
                as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> VdmxRatioRange,
        ),
        disposeItem: Some(
            vdmx_ratio_ragne_list_dispose_item
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, usize) -> (),
        ),
        filterEnv: Some(
            vdmx_ratio_ragne_list_filter_env
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
            vdmx_ratio_ragne_list_sort
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
unsafe extern "C" fn vdmx_ratio_ragne_list_shrink_to_fit(mut arr: *mut VdmxRatioRangeList) {
    vdmx_ratio_ragne_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_resize_to(arr: *mut VdmxRatioRangeList, target: usize) {
    cvec_resize_to(vdmx_ratio_ragne_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_move(dst: *mut VdmxRatioRangeList, src: *mut VdmxRatioRangeList) {
    cvec_move(vdmx_ratio_ragne_list_as_cvec(dst), vdmx_ratio_ragne_list_as_cvec(src));
}
#[inline]
unsafe fn vdmx_ratio_ragne_list_as_cvec(arr: *mut VdmxRatioRangeList) -> *mut CVecRaw<VdmxRatioRange> {
    arr as *mut CVecRaw<VdmxRatioRange>
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_init(arr: *mut VdmxRatioRangeList) {
    cvec_init(vdmx_ratio_ragne_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_filter_env(
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
            if VDMX_I_RATIO_RANGE.dispose.is_some() {
                VDMX_I_RATIO_RANGE.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn vdmx_ratio_ragne_list_sort(
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
unsafe extern "C" fn vdmx_ratio_ragne_list_fill(mut arr: *mut VdmxRatioRangeList, mut n: usize) {
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
        if VDMX_I_RATIO_RANGE.init.is_some() {
            VDMX_I_RATIO_RANGE.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VdmxRatioRange>() as usize,
            );
        }
        vdmx_ratio_ragne_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_push(arr: *mut VdmxRatioRangeList, elem: VdmxRatioRange) {
    cvec_push(vdmx_ratio_ragne_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_grow(arr: *mut VdmxRatioRangeList) {
    cvec_grow(vdmx_ratio_ragne_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_grow_to(arr: *mut VdmxRatioRangeList, target: usize) {
    cvec_grow_to(vdmx_ratio_ragne_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_pop(arr: *mut VdmxRatioRangeList) -> VdmxRatioRange {
    cvec_pop(vdmx_ratio_ragne_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_copy_replace(
    mut dst: *mut VdmxRatioRangeList,
    src: VdmxRatioRangeList,
) {
    vdmx_ratio_ragne_list_dispose(dst);
    vdmx_ratio_ragne_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_copy(
    mut dst: *mut VdmxRatioRangeList,
    mut src: *const VdmxRatioRangeList,
) {
    vdmx_ratio_ragne_list_init(dst);
    vdmx_ratio_ragne_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if VDMX_I_RATIO_RANGE.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            VDMX_I_RATIO_RANGE.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn vdmx_ratio_ragne_list_dispose(mut arr: *mut VdmxRatioRangeList) {
    if arr.is_null() {
        return;
    }
    if VDMX_I_RATIO_RANGE.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            VDMX_I_RATIO_RANGE.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn vdmx_ratio_ragne_list_replace(
    mut dst: *mut VdmxRatioRangeList,
    src: VdmxRatioRangeList,
) {
    vdmx_ratio_ragne_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxRatioRangeList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_init_cap_n(
    mut arr: *mut VdmxRatioRangeList,
    mut n: usize,
) {
    vdmx_ratio_ragne_list_init(arr);
    vdmx_ratio_ragne_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_grow_to_n(arr: *mut VdmxRatioRangeList, target: usize) {
    cvec_grow_to_n(vdmx_ratio_ragne_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_init_n(mut arr: *mut VdmxRatioRangeList, mut n: usize) {
    vdmx_ratio_ragne_list_init(arr);
    vdmx_ratio_ragne_list_grow_to_n(arr, n);
    vdmx_ratio_ragne_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_free(mut x: *mut VdmxRatioRangeList) {
    if x.is_null() {
        return;
    }
    vdmx_ratio_ragne_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_create_n(mut n: usize) -> *mut VdmxRatioRangeList {
    let mut t: *mut VdmxRatioRangeList =
        malloc(::core::mem::size_of::<VdmxRatioRangeList>() as usize) as *mut VdmxRatioRangeList;
    vdmx_ratio_ragne_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_create() -> *mut VdmxRatioRangeList {
    let mut x: *mut VdmxRatioRangeList =
        malloc(::core::mem::size_of::<VdmxRatioRangeList>() as usize) as *mut VdmxRatioRangeList;
    vdmx_ratio_ragne_list_init(x);
    return x;
}
unsafe extern "C" fn init_vdmx(mut t: *mut VdmxTable) {
    VDMX_I_RATIO_RANGE_LIST
        .init
        .expect("non-null function pointer")(&raw mut (*t).ratios);
}
unsafe extern "C" fn dispose_vdmx(mut t: *mut VdmxTable) {
    VDMX_I_RATIO_RANGE_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*t).ratios);
}
#[inline]
unsafe extern "C" fn table_vdmx_free(mut x: *mut VdmxTable) {
    if x.is_null() {
        return;
    }
    table_vdmx_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_vdmx_move(mut dst: *mut VdmxTable, mut src: *mut VdmxTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxTable>() as usize,
    );
    table_vdmx_init(src);
}
#[inline]
unsafe extern "C" fn table_vdmx_copy(mut dst: *mut VdmxTable, mut src: *const VdmxTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxTable>() as usize,
    );
}
pub static TABLE_I_VDMX: VdmxTableElementInterface = {
    VdmxTableElementInterface {
        init: Some(table_vdmx_init as unsafe extern "C" fn(*mut VdmxTable) -> ()),
        copy: Some(
            table_vdmx_copy as unsafe extern "C" fn(*mut VdmxTable, *const VdmxTable) -> (),
        ),
        move_0: Some(
            table_vdmx_move as unsafe extern "C" fn(*mut VdmxTable, *mut VdmxTable) -> (),
        ),
        dispose: Some(table_vdmx_dispose as unsafe extern "C" fn(*mut VdmxTable) -> ()),
        replace: Some(
            table_vdmx_replace as unsafe extern "C" fn(*mut VdmxTable, VdmxTable) -> (),
        ),
        copyReplace: Some(
            table_vdmx_copy_replace as unsafe extern "C" fn(*mut VdmxTable, VdmxTable) -> (),
        ),
        create: Some(table_vdmx_create),
        free: Some(table_vdmx_free as unsafe extern "C" fn(*mut VdmxTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_vdmx_init(mut x: *mut VdmxTable) {
    init_vdmx(x);
}
#[inline]
unsafe extern "C" fn table_vdmx_dispose(mut x: *mut VdmxTable) {
    dispose_vdmx(x);
}
#[inline]
unsafe extern "C" fn table_vdmx_replace(mut dst: *mut VdmxTable, src: VdmxTable) {
    table_vdmx_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VdmxTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_vdmx_create() -> *mut VdmxTable {
    let mut x: *mut VdmxTable =
        malloc(::core::mem::size_of::<VdmxTable>() as usize) as *mut VdmxTable;
    table_vdmx_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_vdmx_copy_replace(mut dst: *mut VdmxTable, src: VdmxTable) {
    table_vdmx_dispose(dst);
    table_vdmx_copy(dst, &raw const src);
}
