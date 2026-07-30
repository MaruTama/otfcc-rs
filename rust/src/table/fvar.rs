#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memcpy, memset, qsort};

use crate::support::json_funcs::{json_new_position, json_numof, json_object_push_tag, preserialize};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, FontFilePointer, Pos};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::JsonValue;
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_pop, cvec_push, cvec_resize_to};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::{NULL, ComparFn};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::vf::axis::{VfAxes, VfAxis};
use crate::vf::region::{VqAxisSpan, VqRegion};
use crate::vf::vq::{VQ, VqSegment};
use crate::vf::vv::VV;
use crate::support::primitives::{otfcc_from_fixed};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_boolean_new, json_double_new, json_integer_new, json_object_new, json_object_push, json_object_push_length, json_string_new, json_string_new_length};
use crate::vendor::sds::{sdscatsds, sdsempty, sdsfree, sdsfromlonglong, sdsnew};
use crate::vf::axis::{VF_I_AXES};
use crate::vf::region::{vq_axis_span_is_one, vq_delete_region};
use crate::vf::vq::{I_VQ, I_VV};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FvarInstance {
    pub subfamily_name_id: u16,
    pub flags: u16,
    pub coordinates: VV,
    pub post_script_name_id: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FvarInstanceElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut FvarInstance) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut FvarInstance, *const FvarInstance) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut FvarInstance) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FvarInstanceList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut FvarInstance,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FvarInstanceListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut FvarInstanceList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut FvarInstanceList, *const FvarInstanceList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut FvarInstanceList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut FvarInstanceList>,
    pub free: Option<unsafe extern "C" fn(*mut FvarInstanceList) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut FvarInstanceList, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut FvarInstanceList, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut FvarInstanceList>,
    pub fill: Option<unsafe extern "C" fn(*mut FvarInstanceList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut FvarInstanceList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut FvarInstanceList, FvarInstance) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut FvarInstanceList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut FvarInstanceList) -> FvarInstance>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut FvarInstanceList, usize) -> ()>,
    pub filter_env: Option<
        unsafe extern "C" fn(
            *mut FvarInstanceList,
            Option<unsafe extern "C" fn(*const FvarInstance, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut FvarInstanceList,
            Option<
                unsafe extern "C" fn(
                    *const FvarInstance,
                    *const FvarInstance,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FvarMaster {
    pub name: SdsRaw,
    pub region: *mut VqRegion,
    pub hh: UtHashHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FvarTable {
    pub major_version: u16,
    pub minor_version: u16,
    pub axes: VfAxes,
    pub instances: FvarInstanceList,
    pub masters: *mut FvarMaster,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FvarTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut FvarTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut FvarTable, *const FvarTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut FvarTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut FvarTable>,
    pub free: Option<unsafe extern "C" fn(*mut FvarTable) -> ()>,
    pub register_region:
        Option<unsafe extern "C" fn(*mut FvarTable, *mut VqRegion) -> *const VqRegion>,
    pub find_master_by_region:
        Option<unsafe extern "C" fn(*const FvarTable, *const VqRegion) -> *const FvarMaster>,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct InstanceRecord {
    pub subfamily_name_id: u16,
    pub flags: u16,
    pub coordinates: [F16Dot16; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct FVARHeader {
    pub major_version: u16,
    pub minor_version: u16,
    pub axes_array_offset: u16,
    pub reserved1: u16,
    pub axis_count: u16,
    pub axis_size: u16,
    pub instance_count: u16,
    pub instance_size: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct VariationAxisRecord {
    pub axis_tag: u32,
    pub min_value: F16Dot16,
    pub default_value: F16Dot16,
    pub max_value: F16Dot16,
    pub flags: u16,
    pub axis_name_id: u16,
}
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[inline]
unsafe extern "C" fn init_fvar_instance(mut inst: *mut FvarInstance) {
    memset(
        inst as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<FvarInstance>() as usize,
    );
    I_VV.init.expect("non-null function pointer")(&raw mut (*inst).coordinates);
}
#[inline]
unsafe extern "C" fn dispose_fvar_instance(mut inst: *mut FvarInstance) {
    I_VV.dispose.expect("non-null function pointer")(&raw mut (*inst).coordinates);
}
pub static FVAR_I_INSTANCE: FvarInstanceElementInterface = {
    FvarInstanceElementInterface {
        init: Some(fvar_instance_init as unsafe extern "C" fn(*mut FvarInstance) -> ()),
        copy: Some(
            fvar_instance_copy
                as unsafe extern "C" fn(*mut FvarInstance, *const FvarInstance) -> (),
        ),
        dispose: Some(fvar_instance_dispose as unsafe extern "C" fn(*mut FvarInstance) -> ()),
    }
};
#[inline]
unsafe extern "C" fn fvar_instance_dispose(mut x: *mut FvarInstance) {
    dispose_fvar_instance(x);
}
#[inline]
unsafe extern "C" fn fvar_instance_init(mut x: *mut FvarInstance) {
    init_fvar_instance(x);
}
#[inline]
unsafe extern "C" fn fvar_instance_copy(
    mut dst: *mut FvarInstance,
    mut src: *const FvarInstance,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FvarInstance>() as usize,
    );
}
#[inline]
unsafe extern "C" fn fvar_instance_list_free(mut x: *mut FvarInstanceList) {
    if x.is_null() {
        return;
    }
    fvar_instance_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn fvar_instance_list_resize_to(arr: *mut FvarInstanceList, target: usize) {
    cvec_resize_to(fvar_instance_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn fvar_instance_list_create_n(mut n: usize) -> *mut FvarInstanceList {
    let mut t: *mut FvarInstanceList =
        malloc(::core::mem::size_of::<FvarInstanceList>() as usize) as *mut FvarInstanceList;
    fvar_instance_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn fvar_instance_list_create() -> *mut FvarInstanceList {
    let mut x: *mut FvarInstanceList =
        malloc(::core::mem::size_of::<FvarInstanceList>() as usize) as *mut FvarInstanceList;
    fvar_instance_list_init(x);
    return x;
}
#[inline]
unsafe fn fvar_instance_list_as_cvec(arr: *mut FvarInstanceList) -> *mut CVecRaw<FvarInstance> {
    arr as *mut CVecRaw<FvarInstance>
}
#[inline]
unsafe extern "C" fn fvar_instance_list_init(arr: *mut FvarInstanceList) {
    cvec_init(fvar_instance_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn fvar_instance_list_filter_env(
    mut arr: *mut FvarInstanceList,
    mut fn_0: Option<unsafe extern "C" fn(*const FvarInstance, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut FvarInstance,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if FVAR_I_INSTANCE.dispose.is_some() {
                FVAR_I_INSTANCE.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut FvarInstance,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn fvar_instance_list_dispose_item(mut arr: *mut FvarInstanceList, mut n: usize) {
    if FVAR_I_INSTANCE.dispose.is_some() {
        FVAR_I_INSTANCE.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut FvarInstance
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn fvar_instance_list_sort(
    mut arr: *mut FvarInstanceList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const FvarInstance, *const FvarInstance) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<FvarInstance>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const FvarInstance,
                    *const FvarInstance,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn fvar_instance_list_fill(mut arr: *mut FvarInstanceList, mut n: usize) {
    while (*arr).length < n {
        let mut x: FvarInstance = FvarInstance {
            subfamily_name_id: 0,
            flags: 0,
            coordinates: VV {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<Pos>(),
            },
            post_script_name_id: 0,
        };
        if FVAR_I_INSTANCE.init.is_some() {
            FVAR_I_INSTANCE.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<FvarInstance>() as usize,
            );
        }
        fvar_instance_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn fvar_instance_list_push(arr: *mut FvarInstanceList, elem: FvarInstance) {
    cvec_push(fvar_instance_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn fvar_instance_list_grow_to(arr: *mut FvarInstanceList, target: usize) {
    cvec_grow_to(fvar_instance_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn fvar_instance_list_pop(arr: *mut FvarInstanceList) -> FvarInstance {
    cvec_pop(fvar_instance_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn fvar_instance_list_copy(
    mut dst: *mut FvarInstanceList,
    mut src: *const FvarInstanceList,
) {
    fvar_instance_list_init(dst);
    fvar_instance_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if FVAR_I_INSTANCE.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            FVAR_I_INSTANCE.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut FvarInstance,
                (*src).items.offset(j as isize) as *mut FvarInstance as *const FvarInstance,
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
unsafe extern "C" fn fvar_instance_list_dispose(mut arr: *mut FvarInstanceList) {
    if arr.is_null() {
        return;
    }
    if FVAR_I_INSTANCE.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            FVAR_I_INSTANCE.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut FvarInstance,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<FvarInstance>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn fvar_instance_list_init_cap_n(mut arr: *mut FvarInstanceList, mut n: usize) {
    fvar_instance_list_init(arr);
    fvar_instance_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn fvar_instance_list_grow_to_n(arr: *mut FvarInstanceList, target: usize) {
    cvec_grow_to_n(fvar_instance_list_as_cvec(arr), target);
}
pub static FVAR_I_INSTANCE_LIST: FvarInstanceListVectorInterface = {
    FvarInstanceListVectorInterface {
        init: Some(fvar_instance_list_init as unsafe extern "C" fn(*mut FvarInstanceList) -> ()),
        copy: Some(
            fvar_instance_list_copy
                as unsafe extern "C" fn(*mut FvarInstanceList, *const FvarInstanceList) -> (),
        ),
        dispose: Some(
            fvar_instance_list_dispose as unsafe extern "C" fn(*mut FvarInstanceList) -> (),
        ),
        create: Some(fvar_instance_list_create),
        free: Some(fvar_instance_list_free as unsafe extern "C" fn(*mut FvarInstanceList) -> ()),
        init_n: Some(
            fvar_instance_list_init_n as unsafe extern "C" fn(*mut FvarInstanceList, usize) -> (),
        ),
        init_cap_n: Some(
            fvar_instance_list_init_cap_n
                as unsafe extern "C" fn(*mut FvarInstanceList, usize) -> (),
        ),
        create_n: Some(
            fvar_instance_list_create_n as unsafe extern "C" fn(usize) -> *mut FvarInstanceList,
        ),
        fill: Some(
            fvar_instance_list_fill as unsafe extern "C" fn(*mut FvarInstanceList, usize) -> (),
        ),
        clear: Some(
            fvar_instance_list_dispose as unsafe extern "C" fn(*mut FvarInstanceList) -> (),
        ),
        push: Some(
            fvar_instance_list_push
                as unsafe extern "C" fn(*mut FvarInstanceList, FvarInstance) -> (),
        ),
        shrink_to_fit: Some(
            fvar_instance_list_shrink_to_fit as unsafe extern "C" fn(*mut FvarInstanceList) -> (),
        ),
        pop: Some(
            fvar_instance_list_pop as unsafe extern "C" fn(*mut FvarInstanceList) -> FvarInstance,
        ),
        dispose_item: Some(
            fvar_instance_list_dispose_item
                as unsafe extern "C" fn(*mut FvarInstanceList, usize) -> (),
        ),
        filter_env: Some(
            fvar_instance_list_filter_env
                as unsafe extern "C" fn(
                    *mut FvarInstanceList,
                    Option<
                        unsafe extern "C" fn(
                            *const FvarInstance,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            fvar_instance_list_sort
                as unsafe extern "C" fn(
                    *mut FvarInstanceList,
                    Option<
                        unsafe extern "C" fn(
                            *const FvarInstance,
                            *const FvarInstance,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn fvar_instance_list_init_n(mut arr: *mut FvarInstanceList, mut n: usize) {
    fvar_instance_list_init(arr);
    fvar_instance_list_grow_to_n(arr, n);
    fvar_instance_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn fvar_instance_list_shrink_to_fit(mut arr: *mut FvarInstanceList) {
    fvar_instance_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn dispose_fvar_master(mut m: *mut FvarMaster) {
    sdsfree((*m).name);
    vq_delete_region((*m).region);
}
#[inline]
unsafe extern "C" fn init_fvar(mut fvar: *mut FvarTable) {
    memset(
        fvar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<FvarTable>() as usize,
    );
    VF_I_AXES.init.expect("non-null function pointer")(&raw mut (*fvar).axes);
    FVAR_I_INSTANCE_LIST.init.expect("non-null function pointer")(&raw mut (*fvar).instances);
}
#[inline]
unsafe extern "C" fn dispose_fvar(mut fvar: *mut FvarTable) {
    VF_I_AXES.dispose.expect("non-null function pointer")(&raw mut (*fvar).axes);
    FVAR_I_INSTANCE_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*fvar).instances);
    let mut current: *mut FvarMaster = ::core::ptr::null_mut::<FvarMaster>();
    let mut tmp: *mut FvarMaster = ::core::ptr::null_mut::<FvarMaster>();
    current = (*fvar).masters;
    tmp = (if !(*fvar).masters.is_null() {
        (*(*fvar).masters).hh.next
    } else {
        NULL
    }) as *mut FvarMaster as *mut FvarMaster;
    while !current.is_null() {
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*current).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*fvar).masters).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*fvar).masters).hh.tbl as *mut ::core::ffi::c_void);
            (*fvar).masters = ::core::ptr::null_mut::<FvarMaster>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*fvar).masters).hh.tbl).tail {
                (*(*(*fvar).masters).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*fvar).masters).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*fvar).masters).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh2 = (*_hd_hh_del).next;
            } else {
                (*fvar).masters = (*_hd_hh_del).next as *mut FvarMaster as *mut FvarMaster;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*fvar).masters).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh3 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*fvar).masters).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket = (*(*(*fvar).masters).hh.tbl)
                .buckets
                .offset(_hd_bkt as isize)
                as *mut UtHashBucket;
            (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*(*fvar).masters).hh.tbl).num_items =
                (*(*(*fvar).masters).hh.tbl).num_items.wrapping_sub(1);
        }
        dispose_fvar_master(current);
        free(current as *mut ::core::ffi::c_void);
        current = ::core::ptr::null_mut::<FvarMaster>();
        current = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut FvarMaster
            as *mut FvarMaster;
    }
}
unsafe extern "C" fn fvar_register_region(
    mut fvar: *mut FvarTable,
    mut region: *mut VqRegion,
) -> *const VqRegion {
    let mut m: *mut FvarMaster = ::core::ptr::null_mut::<FvarMaster>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = region as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::core::mem::size_of::<VqRegion>().wrapping_add(
        ::core::mem::size_of::<VqAxisSpan>()
            .wrapping_mul((*region).dimensions as usize),
    ) as ::core::ffi::c_uint;
    while _hj_k >= 12 as ::core::ffi::c_uint {
        _hj_i = _hj_i.wrapping_add(
            (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hj_j = _hj_j.wrapping_add(
            (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hf_hashv = _hf_hashv.wrapping_add(
            (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
        _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
    }
    _hf_hashv = _hf_hashv.wrapping_add(
        ::core::mem::size_of::<VqRegion>().wrapping_add(
            ::core::mem::size_of::<VqAxisSpan>()
                .wrapping_mul((*region).dimensions as usize),
        ) as ::core::ffi::c_uint,
    );
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 11098432890987736715;
        }
        10 => {
            current_block_50 = 11098432890987736715;
        }
        9 => {
            current_block_50 = 7788850179560822105;
        }
        8 => {
            current_block_50 = 2013626843157172960;
        }
        7 => {
            current_block_50 = 7680992524440278500;
        }
        6 => {
            current_block_50 = 14601631620111087220;
        }
        5 => {
            current_block_50 = 11029710244996856751;
        }
        4 => {
            current_block_50 = 16753638405504927854;
        }
        3 => {
            current_block_50 = 13847968192452473061;
        }
        2 => {
            current_block_50 = 13091112611283870258;
        }
        1 => {
            current_block_50 = 18027894311151487420;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        11098432890987736715 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 7788850179560822105;
        }
        _ => {}
    }
    match current_block_50 {
        7788850179560822105 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 2013626843157172960;
        }
        _ => {}
    }
    match current_block_50 {
        2013626843157172960 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 7680992524440278500;
        }
        _ => {}
    }
    match current_block_50 {
        7680992524440278500 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 14601631620111087220;
        }
        _ => {}
    }
    match current_block_50 {
        14601631620111087220 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 11029710244996856751;
        }
        _ => {}
    }
    match current_block_50 {
        11029710244996856751 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 16753638405504927854;
        }
        _ => {}
    }
    match current_block_50 {
        16753638405504927854 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 13847968192452473061;
        }
        _ => {}
    }
    match current_block_50 {
        13847968192452473061 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 13091112611283870258;
        }
        _ => {}
    }
    match current_block_50 {
        13091112611283870258 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 18027894311151487420;
        }
        _ => {}
    }
    match current_block_50 {
        18027894311151487420 => {
            _hj_i = _hj_i.wrapping_add(
                *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
        }
        _ => {}
    }
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
    m = ::core::ptr::null_mut::<FvarMaster>();
    if !(*fvar).masters.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*fvar).masters).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*fvar).masters).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                m = ((*(*(*(*fvar).masters).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*fvar).masters).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut FvarMaster
                    as *mut FvarMaster;
            } else {
                m = ::core::ptr::null_mut::<FvarMaster>();
            }
            while !m.is_null() {
                if (*m).hh.hashv == _hf_hashv
                    && (*m).hh.keylen as usize
                        == ::core::mem::size_of::<VqRegion>().wrapping_add(
                            ::core::mem::size_of::<VqAxisSpan>()
                                .wrapping_mul((*region).dimensions as usize),
                        )
                {
                    if memcmp(
                        (*m).hh.key,
                        region as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<VqRegion>() as usize).wrapping_add(
                            (::core::mem::size_of::<VqAxisSpan>() as usize)
                                .wrapping_mul((*region).dimensions as usize),
                        ),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*m).hh.hh_next.is_null() {
                    m = ((*m).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*fvar).masters).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut FvarMaster
                        as *mut FvarMaster;
                } else {
                    m = ::core::ptr::null_mut::<FvarMaster>();
                }
            }
        }
    }
    if !m.is_null() {
        vq_delete_region(region);
        return (*m).region;
    } else {
        m = __caryll_allocate_clean(
            ::core::mem::size_of::<FvarMaster>() as usize,
            47 as ::core::ffi::c_ulong,
        ) as *mut FvarMaster;
        let mut s_master_id: SdsRaw = sdsfromlonglong((1 as ::core::ffi::c_uint).wrapping_add(
            if !(*fvar).masters.is_null() {
                (*(*(*fvar).masters).hh.tbl).num_items
            } else {
                0 as ::core::ffi::c_uint
            },
        ) as ::core::ffi::c_longlong);
        (*m).name = sdscatsds(
            sdsnew(b"m\0" as *const u8 as *const ::core::ffi::c_char),
            s_master_id,
        );
        sdsfree(s_master_id);
        (*m).region = region;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar = (*m).region as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = ::core::mem::size_of::<VqRegion>().wrapping_add(
            ::core::mem::size_of::<VqAxisSpan>()
                .wrapping_mul((*region).dimensions as usize),
        ) as ::core::ffi::c_uint;
        while _hj_k_0 >= 12 as ::core::ffi::c_uint {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j_0 = _hj_j_0.wrapping_add(
                (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _ha_hashv = _ha_hashv.wrapping_add(
                (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv = _ha_hashv.wrapping_add(
            ::core::mem::size_of::<VqRegion>().wrapping_add(
                ::core::mem::size_of::<VqAxisSpan>()
                    .wrapping_mul((*region).dimensions as usize),
            ) as ::core::ffi::c_uint,
        );
        let mut current_block_171: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_171 = 6827241531168806533;
            }
            10 => {
                current_block_171 = 6827241531168806533;
            }
            9 => {
                current_block_171 = 7490234768345424691;
            }
            8 => {
                current_block_171 = 2571479547849027551;
            }
            7 => {
                current_block_171 = 5065576992453236399;
            }
            6 => {
                current_block_171 = 2708817167913782276;
            }
            5 => {
                current_block_171 = 9658771359317796075;
            }
            4 => {
                current_block_171 = 16102792977521885693;
            }
            3 => {
                current_block_171 = 6851027814222055606;
            }
            2 => {
                current_block_171 = 7597280631034036803;
            }
            1 => {
                current_block_171 = 5043988931478781221;
            }
            _ => {
                current_block_171 = 9587810615301548814;
            }
        }
        match current_block_171 {
            6827241531168806533 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_171 = 7490234768345424691;
            }
            _ => {}
        }
        match current_block_171 {
            7490234768345424691 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_171 = 2571479547849027551;
            }
            _ => {}
        }
        match current_block_171 {
            2571479547849027551 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_171 = 5065576992453236399;
            }
            _ => {}
        }
        match current_block_171 {
            5065576992453236399 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_171 = 2708817167913782276;
            }
            _ => {}
        }
        match current_block_171 {
            2708817167913782276 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_171 = 9658771359317796075;
            }
            _ => {}
        }
        match current_block_171 {
            9658771359317796075 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_171 = 16102792977521885693;
            }
            _ => {}
        }
        match current_block_171 {
            16102792977521885693 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_171 = 6851027814222055606;
            }
            _ => {}
        }
        match current_block_171 {
            6851027814222055606 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_171 = 7597280631034036803;
            }
            _ => {}
        }
        match current_block_171 {
            7597280631034036803 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_171 = 5043988931478781221;
            }
            _ => {}
        }
        match current_block_171 {
            5043988931478781221 => {
                _hj_i_0 =
                    _hj_i_0
                        .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
        (*m).hh.hashv = _ha_hashv;
        (*m).hh.key = (*m).region as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*m).hh.keylen = ::core::mem::size_of::<VqRegion>().wrapping_add(
            ::core::mem::size_of::<VqAxisSpan>()
                .wrapping_mul((*region).dimensions as usize),
        ) as ::core::ffi::c_uint;
        if (*fvar).masters.is_null() {
            (*m).hh.next = NULL;
            (*m).hh.prev = NULL;
            (*m).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*m).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*m).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*m).hh.tbl).tail = &raw mut (*m).hh as *mut UtHashHandle;
                (*(*m).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*m).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*m).hh.tbl).hho = (&raw mut (*m).hh as *mut ::core::ffi::c_char)
                    .offset_from(m as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*m).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*m).hh.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*m).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*m).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    );
                }
            }
            (*fvar).masters = m;
        } else {
            (*m).hh.tbl = (*(*fvar).masters).hh.tbl;
            (*m).hh.next = NULL;
            (*m).hh.prev = ((*(*(*fvar).masters).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*fvar).masters).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*fvar).masters).hh.tbl).tail).next = m as *mut ::core::ffi::c_void;
            (*(*(*fvar).masters).hh.tbl).tail = &raw mut (*m).hh as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*fvar).masters).hh.tbl).num_items =
            (*(*(*fvar).masters).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*fvar).masters).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket = (*(*(*fvar).masters).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UtHashBucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*m).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
        (*m).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*m).hh as *mut UtHashHandle;
        }
        (*_ha_head).hh_head = &raw mut (*m).hh as *mut UtHashHandle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*m).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: ::core::ffi::c_uint = 0;
            let mut _he_bkt_i: ::core::ffi::c_uint = 0;
            let mut _he_thh: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _he_hh_nxt: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _he_new_buckets: *mut UtHashBucket =
                ::core::ptr::null_mut::<UtHashBucket>();
            let mut _he_newbkt: *mut UtHashBucket = ::core::ptr::null_mut::<UtHashBucket>();
            _he_new_buckets = malloc(
                (2 as usize)
                    .wrapping_mul((*(*m).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*m).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                );
                (*(*m).hh.tbl).ideal_chain_maxlen = ((*(*m).hh.tbl).num_items
                    >> (*(*m).hh.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*m).hh.tbl).num_items
                        & (*(*m).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint)
                            .wrapping_sub(1 as ::core::ffi::c_uint)
                        != 0 as ::core::ffi::c_uint
                    {
                        1 as ::core::ffi::c_uint
                    } else {
                        0 as ::core::ffi::c_uint
                    },
                );
                (*(*m).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*m).hh.tbl).num_buckets {
                    _he_thh = (*(*(*m).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UtHashHandle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*m).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*m).hh.tbl).ideal_chain_maxlen {
                            (*(*m).hh.tbl).nonideal_items =
                                (*(*m).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*m).hh.tbl).ideal_chain_maxlen);
                        }
                        (*_he_thh).hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                        (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UtHashHandle;
                        if !(*_he_newbkt).hh_head.is_null() {
                            (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                        }
                        (*_he_newbkt).hh_head = _he_thh as *mut UtHashHandle;
                        _he_thh = _he_hh_nxt;
                    }
                    _he_bkt_i = _he_bkt_i.wrapping_add(1);
                }
                free((*(*m).hh.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*m).hh.tbl).num_buckets = (*(*m).hh.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*m).hh.tbl).log2_num_buckets = (*(*m).hh.tbl).log2_num_buckets.wrapping_add(1);
                (*(*m).hh.tbl).buckets = _he_new_buckets;
                (*(*m).hh.tbl).ineff_expands = if (*(*m).hh.tbl).nonideal_items
                    > (*(*m).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*m).hh.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*m).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*m).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        return (*m).region;
    };
}
unsafe extern "C" fn fvar_find_master_by_region(
    mut fvar: *const FvarTable,
    mut region: *const VqRegion,
) -> *const FvarMaster {
    let mut m: *mut FvarMaster = ::core::ptr::null_mut::<FvarMaster>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = region as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::core::mem::size_of::<VqRegion>().wrapping_add(
        ::core::mem::size_of::<VqAxisSpan>()
            .wrapping_mul((*region).dimensions as usize),
    ) as ::core::ffi::c_uint;
    while _hj_k >= 12 as ::core::ffi::c_uint {
        _hj_i = _hj_i.wrapping_add(
            (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hj_j = _hj_j.wrapping_add(
            (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hf_hashv = _hf_hashv.wrapping_add(
            (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
        _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
    }
    _hf_hashv = _hf_hashv.wrapping_add(
        ::core::mem::size_of::<VqRegion>().wrapping_add(
            ::core::mem::size_of::<VqAxisSpan>()
                .wrapping_mul((*region).dimensions as usize),
        ) as ::core::ffi::c_uint,
    );
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 16983614438056130870;
        }
        10 => {
            current_block_50 = 16983614438056130870;
        }
        9 => {
            current_block_50 = 15525165297982684156;
        }
        8 => {
            current_block_50 = 17129624834029794688;
        }
        7 => {
            current_block_50 = 18376437513952032856;
        }
        6 => {
            current_block_50 = 6454216577031963914;
        }
        5 => {
            current_block_50 = 6870917165266285974;
        }
        4 => {
            current_block_50 = 26157140621613139;
        }
        3 => {
            current_block_50 = 7257937163290155083;
        }
        2 => {
            current_block_50 = 8009893845190326358;
        }
        1 => {
            current_block_50 = 11128669157540593563;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        16983614438056130870 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 15525165297982684156;
        }
        _ => {}
    }
    match current_block_50 {
        15525165297982684156 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 17129624834029794688;
        }
        _ => {}
    }
    match current_block_50 {
        17129624834029794688 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 18376437513952032856;
        }
        _ => {}
    }
    match current_block_50 {
        18376437513952032856 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 6454216577031963914;
        }
        _ => {}
    }
    match current_block_50 {
        6454216577031963914 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 6870917165266285974;
        }
        _ => {}
    }
    match current_block_50 {
        6870917165266285974 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 26157140621613139;
        }
        _ => {}
    }
    match current_block_50 {
        26157140621613139 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 7257937163290155083;
        }
        _ => {}
    }
    match current_block_50 {
        7257937163290155083 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 8009893845190326358;
        }
        _ => {}
    }
    match current_block_50 {
        8009893845190326358 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 11128669157540593563;
        }
        _ => {}
    }
    match current_block_50 {
        11128669157540593563 => {
            _hj_i = _hj_i.wrapping_add(
                *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
        }
        _ => {}
    }
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
    m = ::core::ptr::null_mut::<FvarMaster>();
    if !(*fvar).masters.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*fvar).masters).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*fvar).masters).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                m = ((*(*(*(*fvar).masters).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*fvar).masters).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut FvarMaster
                    as *mut FvarMaster;
            } else {
                m = ::core::ptr::null_mut::<FvarMaster>();
            }
            while !m.is_null() {
                if (*m).hh.hashv == _hf_hashv
                    && (*m).hh.keylen as usize
                        == ::core::mem::size_of::<VqRegion>().wrapping_add(
                            ::core::mem::size_of::<VqAxisSpan>()
                                .wrapping_mul((*region).dimensions as usize),
                        )
                {
                    if memcmp(
                        (*m).hh.key,
                        region as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<VqRegion>() as usize).wrapping_add(
                            (::core::mem::size_of::<VqAxisSpan>() as usize)
                                .wrapping_mul((*region).dimensions as usize),
                        ),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*m).hh.hh_next.is_null() {
                    m = ((*m).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*fvar).masters).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut FvarMaster
                        as *mut FvarMaster;
                } else {
                    m = ::core::ptr::null_mut::<FvarMaster>();
                }
            }
        }
    }
    return m;
}
#[inline]
unsafe extern "C" fn table_fvar_free(mut x: *mut FvarTable) {
    if x.is_null() {
        return;
    }
    table_fvar_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_fvar_dispose(mut x: *mut FvarTable) {
    dispose_fvar(x);
}
#[inline]
unsafe extern "C" fn table_fvar_init(mut x: *mut FvarTable) {
    init_fvar(x);
}
#[inline]
unsafe extern "C" fn table_fvar_create() -> *mut FvarTable {
    let mut x: *mut FvarTable =
        malloc(::core::mem::size_of::<FvarTable>() as usize) as *mut FvarTable;
    table_fvar_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_fvar_copy(mut dst: *mut FvarTable, mut src: *const FvarTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FvarTable>() as usize,
    );
}
pub static TABLE_I_FVAR: FvarTableElementInterface = {
    FvarTableElementInterface {
        init: Some(table_fvar_init as unsafe extern "C" fn(*mut FvarTable) -> ()),
        copy: Some(
            table_fvar_copy as unsafe extern "C" fn(*mut FvarTable, *const FvarTable) -> (),
        ),
        dispose: Some(table_fvar_dispose as unsafe extern "C" fn(*mut FvarTable) -> ()),
        create: Some(table_fvar_create),
        free: Some(table_fvar_free as unsafe extern "C" fn(*mut FvarTable) -> ()),
        register_region: Some(
            fvar_register_region
                as unsafe extern "C" fn(*mut FvarTable, *mut VqRegion) -> *const VqRegion,
        ),
        find_master_by_region: Some(
            fvar_find_master_by_region
                as unsafe extern "C" fn(*const FvarTable, *const VqRegion) -> *const FvarMaster,
        ),
    }
};
pub unsafe extern "C" fn otfcc_read_fvar(
    packet: Packet,
    mut options: *const Options,
) -> *mut FvarTable {
    let mut header: *mut FVARHeader = ::core::ptr::null_mut::<FVARHeader>();
    let mut n_axes: u16 = 0;
    let mut instance_size_without_psnid: u16 = 0;
    let mut instance_size_with_psnid: u16 = 0;
    let mut axis_record: *mut VariationAxisRecord = ::core::ptr::null_mut::<VariationAxisRecord>();
    let mut n_instances: u16 = 0;
    let mut has_postscript_name_id: bool = false;
    let mut instance: *mut InstanceRecord = ::core::ptr::null_mut::<InstanceRecord>();
    let mut fvar: *mut FvarTable = ::core::ptr::null_mut::<FvarTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1719034226i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    if !((table.length as usize) < ::core::mem::size_of::<FVARHeader>()) {
                        header = data as *mut FVARHeader;
                        if !(be16((*header).major_version) as ::core::ffi::c_int
                            != 1 as ::core::ffi::c_int)
                        {
                            if !(be16((*header).minor_version) as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int)
                            {
                                if !(be16((*header).axes_array_offset) as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int)
                                {
                                    if !(be16((*header).axis_count) as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int)
                                    {
                                        if !(be16((*header).axis_size) as usize
                                            != ::core::mem::size_of::<VariationAxisRecord>())
                                        {
                                            n_axes = be16((*header).axis_count);
                                            instance_size_without_psnid = 4_usize.wrapping_add(
                                                (n_axes as usize).wrapping_mul(
                                                    ::core::mem::size_of::<F16Dot16>(),
                                                ),
                                            )
                                                as u16;
                                            instance_size_with_psnid = (2 as ::core::ffi::c_int
                                                + instance_size_without_psnid as ::core::ffi::c_int)
                                                as u16;
                                            if !(be16((*header).instance_size) as ::core::ffi::c_int
                                                != instance_size_without_psnid as ::core::ffi::c_int
                                                && be16((*header).instance_size)
                                                    as ::core::ffi::c_int
                                                    != instance_size_with_psnid as ::core::ffi::c_int)
                                            {
                                                if !((table.length as usize)
                                                    < (be16((*header).axes_array_offset) as usize)
                                                        .wrapping_add(
                                                            ::core::mem::size_of::<
                                                                VariationAxisRecord,
                                                            >(
                                                            )
                                                                .wrapping_mul(n_axes as usize),
                                                        )
                                                        .wrapping_add(
                                                            (be16((*header).instance_size)
                                                                as ::core::ffi::c_int
                                                                * be16((*header).instance_count)
                                                                    as ::core::ffi::c_int)
                                                                as usize,
                                                        ))
                                                {
                                                    fvar = ::core::mem::transmute::<
                                                        _,
                                                        fn() -> *mut FvarTable,
                                                    >(
                                                        TABLE_I_FVAR
                                                            .create
                                                            .expect("non-null function pointer"),
                                                    )(
                                                    );
                                                    axis_record =
                                                        data.offset(be16((*header).axes_array_offset)
                                                            as ::core::ffi::c_int
                                                            as isize)
                                                            as *mut VariationAxisRecord;
                                                    let mut j: u16 = 0 as u16;
                                                    while (j as ::core::ffi::c_int)
                                                        < n_axes as ::core::ffi::c_int
                                                    {
                                                        let mut axis: VfAxis = VfAxis {
                                                            tag: be32((*axis_record).axis_tag),
                                                            min_value: otfcc_from_fixed(be32(
                                                                (*axis_record).min_value as u32,
                                                            )
                                                                as F16Dot16)
                                                                as Pos,
                                                            default_value: otfcc_from_fixed(be32(
                                                                (*axis_record).default_value
                                                                    as u32,
                                                            )
                                                                as F16Dot16)
                                                                as Pos,
                                                            max_value: otfcc_from_fixed(be32(
                                                                (*axis_record).max_value as u32,
                                                            )
                                                                as F16Dot16)
                                                                as Pos,
                                                            flags: be16((*axis_record).flags),
                                                            axis_name_id: be16(
                                                                (*axis_record).axis_name_id,
                                                            ),
                                                        };
                                                        VF_I_AXES
                                                            .push
                                                            .expect("non-null function pointer")(
                                                            &raw mut (*fvar).axes,
                                                            axis,
                                                        );
                                                        axis_record = axis_record.offset(1);
                                                        j = j.wrapping_add(1);
                                                    }
                                                    n_instances = be16((*header).instance_count);
                                                    has_postscript_name_id =
                                                        be16((*header).instance_size)
                                                            as ::core::ffi::c_int
                                                            == instance_size_with_psnid
                                                                as ::core::ffi::c_int;
                                                    instance = axis_record as *mut InstanceRecord;
                                                    let mut j_0: u16 = 0 as u16;
                                                    while (j_0 as ::core::ffi::c_int)
                                                        < n_instances as ::core::ffi::c_int
                                                    {
                                                        let mut inst: FvarInstance =
                                                            FvarInstance {
                                                                subfamily_name_id: 0,
                                                                flags: 0,
                                                                coordinates: VV {
                                                                    length: 0,
                                                                    capacity: 0,
                                                                    items: ::core::ptr::null_mut::<
                                                                        Pos,
                                                                    >(
                                                                    ),
                                                                },
                                                                post_script_name_id: 0,
                                                            };
                                                        FVAR_I_INSTANCE
                                                            .init
                                                            .expect("non-null function pointer")(
                                                            &raw mut inst,
                                                        );
                                                        inst.subfamily_name_id =
                                                            be16((*instance).subfamily_name_id);
                                                        inst.flags = be16((*instance).flags);
                                                        let mut k: u16 = 0 as u16;
                                                        while (k as ::core::ffi::c_int)
                                                            < n_axes as ::core::ffi::c_int
                                                        {
                                                            I_VV.push.expect(
                                                                "non-null function pointer",
                                                            )(
                                                                &raw mut inst.coordinates,
                                                                otfcc_from_fixed(be32(
                                                                    *(&raw mut (*instance)
                                                                        .coordinates
                                                                        as *mut F16Dot16)
                                                                        .offset(k as isize)
                                                                        as u32,
                                                                )
                                                                    as F16Dot16)
                                                                    as Pos,
                                                            );
                                                            k = k.wrapping_add(1);
                                                        }
                                                        I_VV.shrink_to_fit
                                                            .expect("non-null function pointer")(
                                                            &raw mut inst.coordinates,
                                                        );
                                                        if has_postscript_name_id {
                                                            inst.post_script_name_id = be16(
                                                                *((instance as FontFilePointer)
                                                                    .offset(
                                                                        instance_size_without_psnid
                                                                            as ::core::ffi::c_int
                                                                            as isize,
                                                                    )
                                                                    as *mut u16),
                                                            );
                                                        }
                                                        FVAR_I_INSTANCE_LIST
                                                            .push
                                                            .expect("non-null function pointer")(
                                                            &raw mut (*fvar).instances,
                                                            inst,
                                                        );
                                                        instance = (instance as FontFilePointer)
                                                            .offset(be16((*header).instance_size)
                                                                as ::core::ffi::c_int
                                                                as isize)
                                                            as *mut InstanceRecord;
                                                        j_0 = j_0.wrapping_add(1);
                                                    }
                                                    VF_I_AXES
                                                        .shrink_to_fit
                                                        .expect("non-null function pointer")(
                                                        &raw mut (*fvar).axes,
                                                    );
                                                    FVAR_I_INSTANCE_LIST
                                                        .shrink_to_fit
                                                        .expect("non-null function pointer")(
                                                        &raw mut (*fvar).instances,
                                                    );
                                                    return fvar;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(sdsempty(), b"table 'fvar' corrupted.\n"),
                    );
                    TABLE_I_FVAR.free.expect("non-null function pointer")(fvar);
                    fvar = ::core::ptr::null_mut::<FvarTable>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<FvarTable>();
}
pub unsafe extern "C" fn otfcc_dump_fvar(
    mut table: *const FvarTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"fvar"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut t: *mut JsonValue = json_object_new(2 as usize);
        let mut _axes: *mut JsonValue = json_object_new((*table).axes.length);
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < (*table).axes.length {
            let mut axis: *mut VfAxis = (*table).axes.items.offset(__caryll_index as isize);
            while keep != 0 {
                let mut _axis: *mut JsonValue = json_object_new(5 as usize);
                json_object_push(
                    _axis,
                    b"minValue\0" as *const u8 as *const ::core::ffi::c_char,
                    json_double_new((*axis).min_value as ::core::ffi::c_double),
                );
                json_object_push(
                    _axis,
                    b"defaultValue\0" as *const u8 as *const ::core::ffi::c_char,
                    json_double_new((*axis).default_value as ::core::ffi::c_double),
                );
                json_object_push(
                    _axis,
                    b"maxValue\0" as *const u8 as *const ::core::ffi::c_char,
                    json_double_new((*axis).max_value as ::core::ffi::c_double),
                );
                json_object_push(
                    _axis,
                    b"flags\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*axis).flags as i64),
                );
                json_object_push(
                    _axis,
                    b"axisNameID\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*axis).axis_name_id as i64),
                );
                json_object_push_tag(_axes, (*axis).tag, _axis);
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            t,
            b"axes\0" as *const u8 as *const ::core::ffi::c_char,
            _axes,
        );
        let mut _instances: *mut JsonValue = json_array_new((*table).instances.length);
        let mut __caryll_index_0: usize = 0 as usize;
        let mut keep_0: usize = 1 as usize;
        while keep_0 != 0 && __caryll_index_0 < (*table).instances.length {
            let mut instance: *mut FvarInstance =
                (*table).instances.items.offset(__caryll_index_0 as isize);
            while keep_0 != 0 {
                let mut _instance: *mut JsonValue = json_object_new(4 as usize);
                json_object_push(
                    _instance,
                    b"subfamilyNameID\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*instance).subfamily_name_id as i64),
                );
                if (*instance).post_script_name_id != 0 {
                    json_object_push(
                        _instance,
                        b"postScriptNameID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer_new((*instance).post_script_name_id as i64),
                    );
                }
                json_object_push(
                    _instance,
                    b"flags\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*instance).flags as i64),
                );
                json_object_push(
                    _instance,
                    b"coordinates\0" as *const u8 as *const ::core::ffi::c_char,
                    json_new_v_vp(&raw mut (*instance).coordinates, table),
                );
                json_array_push(_instances, _instance);
                keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
            }
            keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
            __caryll_index_0 = __caryll_index_0.wrapping_add(1);
        }
        json_object_push(
            t,
            b"instances\0" as *const u8 as *const ::core::ffi::c_char,
            _instances,
        );
        let mut _masters: *mut JsonValue = json_object_new(
            (if !(*table).masters.is_null() {
                (*(*(*table).masters).hh.tbl).num_items
            } else {
                0 as ::core::ffi::c_uint
            }) as usize,
        );
        let mut current: *mut FvarMaster = ::core::ptr::null_mut::<FvarMaster>();
        let mut tmp: *mut FvarMaster = ::core::ptr::null_mut::<FvarMaster>();
        current = (*table).masters;
        tmp = (if !(*table).masters.is_null() {
            (*(*table).masters).hh.next
        } else {
            NULL
        }) as *mut FvarMaster as *mut FvarMaster;
        while !current.is_null() {
            json_object_push(
                _masters,
                (*current).name as *const ::core::ffi::c_char,
                preserialize(json_new_vq_region_explicit((*current).region, table)),
            );
            current = tmp;
            tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut FvarMaster
                as *mut FvarMaster;
        }
        json_object_push(
            t,
            b"masters\0" as *const u8 as *const ::core::ffi::c_char,
            _masters,
        );
        json_object_push(
            root,
            b"fvar\0" as *const u8 as *const ::core::ffi::c_char,
            t,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn json_new_vq_segment(
    mut s: *const VqSegment,
    mut fvar: *const FvarTable,
) -> *mut JsonValue {
    let mut d: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    match (*s).type_0 as ::core::ffi::c_uint {
        0 => return json_new_position((*s).val.still),
        1 => {
            d = json_object_new(3 as usize);
            json_object_push(
                d,
                b"delta\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_position((*s).val.delta.quantity),
            );
            if !(*s).val.delta.touched {
                json_object_push(
                    d,
                    b"implicit\0" as *const u8 as *const ::core::ffi::c_char,
                    json_boolean_new(!(*s).val.delta.touched as ::core::ffi::c_int),
                );
            }
            json_object_push(
                d,
                b"on\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_vq_region((*s).val.delta.region, fvar),
            );
            return d;
        }
        _ => return json_integer_new(0 as i64),
    };
}
pub unsafe extern "C" fn json_new_vq(z: VQ, mut fvar: *const FvarTable) -> *mut JsonValue {
    if z.shift.length == 0 {
        return preserialize(json_new_position(I_VQ
            .get_still
            .expect("non-null function pointer")(
            z
        )));
    } else {
        let mut a: *mut JsonValue = json_array_new(z.shift.length.wrapping_add(1 as usize));
        json_array_push(a, json_new_position(z.kernel));
        let mut j: usize = 0 as usize;
        while j < z.shift.length {
            json_array_push(
                a,
                json_new_vq_segment(z.shift.items.offset(j as isize) as *mut VqSegment, fvar),
            );
            j = j.wrapping_add(1);
        }
        return preserialize(a);
    };
}
pub unsafe extern "C" fn json_new_vv(x: VV, mut fvar: *const FvarTable) -> *mut JsonValue {
    let mut axes: *const VfAxes = &raw const (*fvar).axes;
    if !axes.is_null() && (*axes).length == x.length {
        let mut _coord: *mut JsonValue = json_object_new((*axes).length);
        let mut m: usize = 0 as usize;
        while m < x.length {
            let mut axis: *mut VfAxis = (*axes).items.offset(m as isize) as *mut VfAxis;
            let mut tag: [::core::ffi::c_char; 4] = [
                (((*axis).tag & 0xff000000 as u32) >> 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                (((*axis).tag & 0xff0000 as u32) >> 16 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                (((*axis).tag & 0xff00 as u32) >> 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                ((*axis).tag & 0xff as u32) as ::core::ffi::c_char,
            ];
            json_object_push_length(
                _coord,
                4 as ::core::ffi::c_uint,
                &raw mut tag as *mut ::core::ffi::c_char,
                json_new_position(*x.items.offset(m as isize)),
            );
            m = m.wrapping_add(1);
        }
        return preserialize(_coord);
    } else {
        let mut _coord_0: *mut JsonValue = json_array_new(x.length);
        let mut m_0: usize = 0 as usize;
        while m_0 < x.length {
            json_array_push(_coord_0, json_new_position(*x.items.offset(m_0 as isize)));
            m_0 = m_0.wrapping_add(1);
        }
        return preserialize(_coord_0);
    };
}
pub unsafe extern "C" fn json_new_v_vp(
    mut x: *const VV,
    mut fvar: *const FvarTable,
) -> *mut JsonValue {
    let mut axes: *const VfAxes = &raw const (*fvar).axes;
    if !axes.is_null() && (*axes).length == (*x).length {
        let mut _coord: *mut JsonValue = json_object_new((*axes).length);
        let mut m: usize = 0 as usize;
        while m < (*x).length {
            let mut axis: *mut VfAxis = (*axes).items.offset(m as isize) as *mut VfAxis;
            let mut tag: [::core::ffi::c_char; 4] = [
                (((*axis).tag & 0xff000000 as u32) >> 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                (((*axis).tag & 0xff0000 as u32) >> 16 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                (((*axis).tag & 0xff00 as u32) >> 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                ((*axis).tag & 0xff as u32) as ::core::ffi::c_char,
            ];
            json_object_push_length(
                _coord,
                4 as ::core::ffi::c_uint,
                &raw mut tag as *mut ::core::ffi::c_char,
                json_new_position(*(*x).items.offset(m as isize)),
            );
            m = m.wrapping_add(1);
        }
        return preserialize(_coord);
    } else {
        let mut _coord_0: *mut JsonValue = json_array_new((*x).length);
        let mut m_0: usize = 0 as usize;
        while m_0 < (*x).length {
            json_array_push(
                _coord_0,
                json_new_position(*(*x).items.offset(m_0 as isize)),
            );
            m_0 = m_0.wrapping_add(1);
        }
        return preserialize(_coord_0);
    };
}
pub unsafe extern "C" fn json_vq_of(mut cv: *const JsonValue, mut _fvar: *const FvarTable) -> VQ {
    return I_VQ.create_still.expect("non-null function pointer")(json_numof(cv) as Pos);
}
pub unsafe extern "C" fn json_new_vq_axis_span(mut s: *const VqAxisSpan) -> *mut JsonValue {
    if vq_axis_span_is_one(s) {
        return json_string_new(b"*\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        let mut a: *mut JsonValue = json_object_new(3 as usize);
        json_object_push(
            a,
            b"start\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*s).start),
        );
        json_object_push(
            a,
            b"peak\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*s).peak),
        );
        json_object_push(
            a,
            b"end\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*s).end),
        );
        return a;
    };
}
pub unsafe extern "C" fn json_new_vq_region_explicit(
    mut rs: *const VqRegion,
    mut fvar: *const FvarTable,
) -> *mut JsonValue {
    let mut axes: *const VfAxes = &raw const (*fvar).axes;
    if !axes.is_null() && (*axes).length == (*rs).dimensions as usize {
        let mut r: *mut JsonValue = json_object_new((*rs).dimensions as usize);
        let mut j: usize = 0 as usize;
        while j < (*rs).dimensions as usize {
            json_object_push_tag(
                r,
                (*(*axes).items.offset(j as isize)).tag,
                json_new_vq_axis_span(
                    (&raw const (*rs).spans as *const VqAxisSpan).offset(j as isize)
                        as *const VqAxisSpan,
                ),
            );
            j = j.wrapping_add(1);
        }
        return r;
    } else {
        let mut r_0: *mut JsonValue = json_array_new((*rs).dimensions as usize);
        let mut j_0: usize = 0 as usize;
        while j_0 < (*rs).dimensions as usize {
            json_array_push(
                r_0,
                json_new_vq_axis_span(
                    (&raw const (*rs).spans as *const VqAxisSpan).offset(j_0 as isize)
                        as *const VqAxisSpan,
                ),
            );
            j_0 = j_0.wrapping_add(1);
        }
        return r_0;
    };
}
pub unsafe extern "C" fn json_new_vq_region(
    mut rs: *const VqRegion,
    mut fvar: *const FvarTable,
) -> *mut JsonValue {
    let mut m: *const FvarMaster = TABLE_I_FVAR
        .find_master_by_region
        .expect("non-null function pointer")(fvar, rs);
    if !m.is_null() && !(*m).name.is_null() {
        return json_string_new_length(
            sdslen((*m).name) as ::core::ffi::c_uint,
            (*m).name as *const ::core::ffi::c_char,
        );
    } else {
        return json_new_vq_region_explicit(rs, fvar);
    };
}
#[inline]
unsafe extern "C" fn be16(mut x: u16) -> u16 {
    return ((x as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | (x as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int)
        as u16;
}
#[inline]
unsafe extern "C" fn be32(mut x: u32) -> u32 {
    return (x & 0xff as u32) << 24 as ::core::ffi::c_int
        | (x & 0xff00 as u32) << 8 as ::core::ffi::c_int
        | (x & 0xff0000 as u32) >> 8 as ::core::ffi::c_int
        | (x & 0xff000000 as u32) >> 24 as ::core::ffi::c_int;
}
