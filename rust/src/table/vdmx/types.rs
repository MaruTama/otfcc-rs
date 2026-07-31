#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_init, cvec_push};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRecord {
    pub y_pel_height: u16,
    pub y_max: i16,
    pub y_min: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRecordElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VdmxRecord) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VdmxRecord, *const VdmxRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxRecord) -> ()>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxGroup) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VdmxGroup>,
    pub free: Option<unsafe extern "C" fn(*mut VdmxGroup) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VdmxGroup, VdmxRecord) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRatioRange {
    pub b_charset: u8,
    pub x_ratio: u8,
    pub y_start_ratio: u8,
    pub y_end_ratio: u8,
    pub records: VdmxGroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdmxRatioRangeElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VdmxRatioRange) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VdmxRatioRange, *const VdmxRatioRange) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxRatioRange) -> ()>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VdmxRatioRangeList>,
    pub free: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRange) -> ()>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut VdmxTable) -> ()>,
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
        y_pel_height: 0,
        y_max: 0,
        y_min: 0,
    };
    vdmx_record_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn vdmx_record_dispose(mut _x: *mut VdmxRecord) {}
#[inline]
unsafe extern "C" fn vdmx_record_dup(src: VdmxRecord) -> VdmxRecord {
    let mut dst: VdmxRecord = VdmxRecord {
        y_pel_height: 0,
        y_max: 0,
        y_min: 0,
    };
    vdmx_record_copy(&raw mut dst, &raw const src);
    return dst;
}
pub static VDMX_I_RECORD: VdmxRecordElementInterface = {
    VdmxRecordElementInterface {
        init: Some(vdmx_record_init as unsafe extern "C" fn(*mut VdmxRecord) -> ()),
        copy: Some(
            vdmx_record_copy as unsafe extern "C" fn(*mut VdmxRecord, *const VdmxRecord) -> (),
        ),
        dispose: Some(vdmx_record_dispose as unsafe extern "C" fn(*mut VdmxRecord) -> ()),
        empty: Some(vdmx_record_empty),
        dup: Some(vdmx_record_dup as unsafe extern "C" fn(VdmxRecord) -> VdmxRecord),
    }
};
#[inline]
unsafe extern "C" fn vdmx_group_push(arr: *mut VdmxGroup, elem: VdmxRecord) {
    cvec_push(vdmx_group_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vdmx_group_grow_to(arr: *mut VdmxGroup, target: usize) {
    cvec_grow_to(vdmx_group_as_cvec(arr), target);
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
unsafe extern "C" fn vdmx_group_free(mut x: *mut VdmxGroup) {
    if x.is_null() {
        return;
    }
    vdmx_group_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn vdmx_group_create() -> *mut VdmxGroup {
    let mut x: *mut VdmxGroup =
        malloc(::core::mem::size_of::<VdmxGroup>() as usize) as *mut VdmxGroup;
    vdmx_group_init(x);
    return x;
}
#[inline]
unsafe fn vdmx_group_as_cvec(arr: *mut VdmxGroup) -> *mut CVecRaw<VdmxRecord> {
    arr as *mut CVecRaw<VdmxRecord>
}
#[inline]
unsafe extern "C" fn vdmx_group_init(arr: *mut VdmxGroup) {
    cvec_init(vdmx_group_as_cvec(arr));
}
pub static VDMX_I_GROUP: VdmxGroupVectorInterface = {
    VdmxGroupVectorInterface {
        init: Some(vdmx_group_init as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        copy: Some(
            vdmx_group_copy as unsafe extern "C" fn(*mut VdmxGroup, *const VdmxGroup) -> (),
        ),
        dispose: Some(vdmx_group_dispose as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        create: Some(vdmx_group_create),
        free: Some(vdmx_group_free as unsafe extern "C" fn(*mut VdmxGroup) -> ()),
        push: Some(vdmx_group_push as unsafe extern "C" fn(*mut VdmxGroup, VdmxRecord) -> ()),
    }
};
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
pub static VDMX_I_RATIO_RANGE: VdmxRatioRangeElementInterface = {
    VdmxRatioRangeElementInterface {
        init: Some(vdmx_ratio_range_init as unsafe extern "C" fn(*mut VdmxRatioRange) -> ()),
        copy: Some(
            vdmx_ratio_range_copy
                as unsafe extern "C" fn(*mut VdmxRatioRange, *const VdmxRatioRange) -> (),
        ),
        dispose: Some(vdmx_ratio_range_dispose as unsafe extern "C" fn(*mut VdmxRatioRange) -> ()),
    }
};
pub static VDMX_I_RATIO_RANGE_LIST: VdmxRatioRangeListVectorInterface = {
    VdmxRatioRangeListVectorInterface {
        init: Some(
            vdmx_ratio_ragne_list_init as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        copy: Some(
            vdmx_ratio_ragne_list_copy
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, *const VdmxRatioRangeList) -> (),
        ),
        dispose: Some(
            vdmx_ratio_ragne_list_dispose as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        create: Some(vdmx_ratio_ragne_list_create),
        free: Some(
            vdmx_ratio_ragne_list_free as unsafe extern "C" fn(*mut VdmxRatioRangeList) -> (),
        ),
        push: Some(
            vdmx_ratio_ragne_list_push
                as unsafe extern "C" fn(*mut VdmxRatioRangeList, VdmxRatioRange) -> (),
        ),
    }
};
#[inline]
unsafe fn vdmx_ratio_ragne_list_as_cvec(arr: *mut VdmxRatioRangeList) -> *mut CVecRaw<VdmxRatioRange> {
    arr as *mut CVecRaw<VdmxRatioRange>
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_init(arr: *mut VdmxRatioRangeList) {
    cvec_init(vdmx_ratio_ragne_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_push(arr: *mut VdmxRatioRangeList, elem: VdmxRatioRange) {
    cvec_push(vdmx_ratio_ragne_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn vdmx_ratio_ragne_list_grow_to(arr: *mut VdmxRatioRangeList, target: usize) {
    cvec_grow_to(vdmx_ratio_ragne_list_as_cvec(arr), target);
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
unsafe extern "C" fn vdmx_ratio_ragne_list_free(mut x: *mut VdmxRatioRangeList) {
    if x.is_null() {
        return;
    }
    vdmx_ratio_ragne_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
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
        dispose: Some(table_vdmx_dispose as unsafe extern "C" fn(*mut VdmxTable) -> ()),
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
unsafe extern "C" fn table_vdmx_create() -> *mut VdmxTable {
    let mut x: *mut VdmxTable =
        malloc(::core::mem::size_of::<VdmxTable>() as usize) as *mut VdmxTable;
    table_vdmx_init(x);
    return x;
}
