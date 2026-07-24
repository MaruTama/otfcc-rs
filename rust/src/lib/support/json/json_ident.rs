extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}

use crate::src::lib::support::stdio::FILE;
use crate::src::lib::support::alloc::{__caryll_allocate_clean};
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type json_type = ::core::ffi::c_uint;
pub const json_pre_serialized: json_type = 8;
pub const json_null: json_type = 7;
pub const json_boolean: json_type = 6;
pub const json_string: json_type = 5;
pub const json_double: json_type = 4;
pub const json_integer: json_type = 3;
pub const json_array: json_type = 2;
pub const json_object: json_type = 1;
pub const json_none: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_value {
    pub parent: *mut _json_value,
    pub type_0: json_type,
    pub u: C2RustUnnamed_0,
    pub _reserved: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub next_alloc: *mut _json_value,
    pub object_mem: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub boolean: ::core::ffi::c_int,
    pub integer: int64_t,
    pub dbl: ::core::ffi::c_double,
    pub string: C2RustUnnamed_3,
    pub object: C2RustUnnamed_2,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut json_object_entry,
}
pub type json_object_entry = _json_object_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_object_entry {
    pub name: *mut ::core::ffi::c_char,
    pub name_length: ::core::ffi::c_uint,
    pub value: *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub length: ::core::ffi::c_uint,
    pub ptr: *mut ::core::ffi::c_char,
}
pub type json_value = _json_value;
pub type ptrdiff_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: ::core::ffi::c_uint,
    pub expand_mult: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut ::core::ffi::c_void,
    pub next: *mut ::core::ffi::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut ::core::ffi::c_void,
    pub keylen: ::core::ffi::c_uint,
    pub hashv: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: ::core::ffi::c_uint,
    pub log2_num_buckets: ::core::ffi::c_uint,
    pub num_items: ::core::ffi::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: ::core::ffi::c_uint,
    pub nonideal_items: ::core::ffi::c_uint,
    pub ineff_expands: ::core::ffi::c_uint,
    pub noexpand: ::core::ffi::c_uint,
    pub signature: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_obj_entry {
    pub key: *mut ::core::ffi::c_char,
    pub val: *mut json_value,
    pub check: bool,
    pub hh: UT_hash_handle,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const HASH_INITIAL_NUM_BUCKETS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const HASH_INITIAL_NUM_BUCKETS_LOG2: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const HASH_BKT_CAPACITY_THRESH: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
pub const HASH_SIGNATURE: ::core::ffi::c_uint = 0xa0111fe1 as ::core::ffi::c_uint;
unsafe extern "C" fn compare_json_arrays(
    mut a: *const json_value,
    mut b: *const json_value,
) -> bool {
    let mut j: uint16_t = 0 as uint16_t;
    while (j as ::core::ffi::c_uint) < (*a).u.array.length {
        if !json_ident(
            *(*a).u.array.values.offset(j as isize),
            *(*b).u.array.values.offset(j as isize),
        ) {
            return false;
        }
        j = j.wrapping_add(1);
    }
    return true;
}
unsafe extern "C" fn compare_json_objects(
    mut a: *const json_value,
    mut b: *const json_value,
) -> bool {
    let mut h: *mut json_obj_entry = ::core::ptr::null_mut::<json_obj_entry>();
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*a).u.object.length as uint32_t {
        let mut k: *mut ::core::ffi::c_char = (*(*a).u.object.values.offset(j as isize)).name;
        let mut e: *mut json_obj_entry = ::core::ptr::null_mut::<json_obj_entry>();
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key: *const ::core::ffi::c_uchar = k as *const ::core::ffi::c_uchar;
        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = strlen(k) as ::core::ffi::c_uint;
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
        _hf_hashv = _hf_hashv.wrapping_add(strlen(k) as ::core::ffi::c_uint);
        let mut current_block_50: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 4758579651109969911;
            }
            10 => {
                current_block_50 = 4758579651109969911;
            }
            9 => {
                current_block_50 = 16493020593694829808;
            }
            8 => {
                current_block_50 = 16983257999075342595;
            }
            7 => {
                current_block_50 = 7599852599096951354;
            }
            6 => {
                current_block_50 = 14930345736155127440;
            }
            5 => {
                current_block_50 = 6402997211775321878;
            }
            4 => {
                current_block_50 = 7918515792888272265;
            }
            3 => {
                current_block_50 = 9050093969003559074;
            }
            2 => {
                current_block_50 = 17182639843621089094;
            }
            1 => {
                current_block_50 = 7225355339896812573;
            }
            _ => {
                current_block_50 = 1847472278776910194;
            }
        }
        match current_block_50 {
            4758579651109969911 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 16493020593694829808;
            }
            _ => {}
        }
        match current_block_50 {
            16493020593694829808 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 16983257999075342595;
            }
            _ => {}
        }
        match current_block_50 {
            16983257999075342595 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 7599852599096951354;
            }
            _ => {}
        }
        match current_block_50 {
            7599852599096951354 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 14930345736155127440;
            }
            _ => {}
        }
        match current_block_50 {
            14930345736155127440 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 6402997211775321878;
            }
            _ => {}
        }
        match current_block_50 {
            6402997211775321878 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_50 = 7918515792888272265;
            }
            _ => {}
        }
        match current_block_50 {
            7918515792888272265 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 9050093969003559074;
            }
            _ => {}
        }
        match current_block_50 {
            9050093969003559074 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 17182639843621089094;
            }
            _ => {}
        }
        match current_block_50 {
            17182639843621089094 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 7225355339896812573;
            }
            _ => {}
        }
        match current_block_50 {
            7225355339896812573 => {
                _hj_i =
                    _hj_i
                        .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
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
        e = ::core::ptr::null_mut::<json_obj_entry>();
        if !h.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if !(*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize))
                    .hh_head
                    .is_null()
                {
                    e = ((*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                        as *mut ::core::ffi::c_char)
                        .offset(-(*(*h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut json_obj_entry
                        as *mut json_obj_entry;
                } else {
                    e = ::core::ptr::null_mut::<json_obj_entry>();
                }
                while !e.is_null() {
                    if (*e).hh.hashv == _hf_hashv
                        && (*e).hh.keylen == strlen(k) as ::core::ffi::c_uint
                    {
                        if memcmp(
                            (*e).hh.key,
                            k as *const ::core::ffi::c_void,
                            strlen(k) as ::core::ffi::c_uint as size_t,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*e).hh.hh_next.is_null() {
                        e = ((*e).hh.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut json_obj_entry
                            as *mut json_obj_entry;
                    } else {
                        e = ::core::ptr::null_mut::<json_obj_entry>();
                    }
                }
            }
        }
        if e.is_null() {
            e = __caryll_allocate_clean(
                ::core::mem::size_of::<json_obj_entry>() as size_t,
                28 as ::core::ffi::c_ulong,
            ) as *mut json_obj_entry;
            (*e).key = strdup(k);
            (*e).val = (*(*a).u.object.values.offset(j as isize)).value as *mut json_value;
            (*e).check = false;
            let mut _ha_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i_0: ::core::ffi::c_uint = 0;
            let mut _hj_j_0: ::core::ffi::c_uint = 0;
            let mut _hj_k_0: ::core::ffi::c_uint = 0;
            let mut _hj_key_0: *const ::core::ffi::c_uchar =
                (*e).key.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_uchar;
            _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_0 = _hj_j_0;
            _hj_k_0 = strlen((*e).key) as ::core::ffi::c_uint;
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
            _ha_hashv = _ha_hashv.wrapping_add(strlen((*e).key) as ::core::ffi::c_uint);
            let mut current_block_168: u64;
            match _hj_k_0 {
                11 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_168 = 15790096718549127373;
                }
                10 => {
                    current_block_168 = 15790096718549127373;
                }
                9 => {
                    current_block_168 = 7420332858505820598;
                }
                8 => {
                    current_block_168 = 18329605127095459026;
                }
                7 => {
                    current_block_168 = 5848836170976635232;
                }
                6 => {
                    current_block_168 = 3120016620003239749;
                }
                5 => {
                    current_block_168 = 14617131505524785108;
                }
                4 => {
                    current_block_168 = 569867509670242043;
                }
                3 => {
                    current_block_168 = 8816581272328801728;
                }
                2 => {
                    current_block_168 = 66573856554016451;
                }
                1 => {
                    current_block_168 = 15439611127644004305;
                }
                _ => {
                    current_block_168 = 16937825661756021828;
                }
            }
            match current_block_168 {
                15790096718549127373 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_168 = 7420332858505820598;
                }
                _ => {}
            }
            match current_block_168 {
                7420332858505820598 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_168 = 18329605127095459026;
                }
                _ => {}
            }
            match current_block_168 {
                18329605127095459026 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_168 = 5848836170976635232;
                }
                _ => {}
            }
            match current_block_168 {
                5848836170976635232 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_168 = 3120016620003239749;
                }
                _ => {}
            }
            match current_block_168 {
                3120016620003239749 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_168 = 14617131505524785108;
                }
                _ => {}
            }
            match current_block_168 {
                14617131505524785108 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_168 = 569867509670242043;
                }
                _ => {}
            }
            match current_block_168 {
                569867509670242043 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_168 = 8816581272328801728;
                }
                _ => {}
            }
            match current_block_168 {
                8816581272328801728 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_168 = 66573856554016451;
                }
                _ => {}
            }
            match current_block_168 {
                66573856554016451 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_168 = 15439611127644004305;
                }
                _ => {}
            }
            match current_block_168 {
                15439611127644004305 => {
                    _hj_i_0 = _hj_i_0
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
            (*e).hh.hashv = _ha_hashv;
            (*e).hh.key = (*e).key.offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            (*e).hh.keylen = strlen((*e).key) as ::core::ffi::c_uint;
            if h.is_null() {
                (*e).hh.next = NULL;
                (*e).hh.prev = NULL;
                (*e).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                    as *mut UT_hash_table as *mut UT_hash_table;
                if (*e).hh.tbl.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*e).hh.tbl as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<UT_hash_table>() as size_t,
                    );
                    (*(*e).hh.tbl).tail = &raw mut (*e).hh as *mut UT_hash_handle;
                    (*(*e).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                    (*(*e).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                    (*(*e).hh.tbl).hho = (&raw mut (*e).hh as *mut ::core::ffi::c_char)
                        .offset_from(e as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as ptrdiff_t;
                    (*(*e).hh.tbl).buckets = malloc(
                        (32 as size_t)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                    ) as *mut UT_hash_bucket;
                    (*(*e).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                    if (*(*e).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*e).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as size_t)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                        );
                    }
                }
                h = e;
            } else {
                (*e).hh.tbl = (*h).hh.tbl;
                (*e).hh.next = NULL;
                (*e).hh.prev = ((*(*h).hh.tbl).tail as *mut ::core::ffi::c_char)
                    .offset(-(*(*h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void;
                (*(*(*h).hh.tbl).tail).next = e as *mut ::core::ffi::c_void;
                (*(*h).hh.tbl).tail = &raw mut (*e).hh as *mut UT_hash_handle;
            }
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UT_hash_bucket =
                (*(*h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
            (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
            (*e).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
            (*e).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
            if !(*_ha_head).hh_head.is_null() {
                (*(*_ha_head).hh_head).hh_prev = &raw mut (*e).hh as *mut UT_hash_handle;
            }
            (*_ha_head).hh_head = &raw mut (*e).hh as *mut UT_hash_handle;
            if (*_ha_head).count
                >= (*_ha_head)
                    .expand_mult
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                && (*(*e).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: ::core::ffi::c_uint = 0;
                let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                let mut _he_thh: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_hh_nxt: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_new_buckets: *mut UT_hash_bucket =
                    ::core::ptr::null_mut::<UT_hash_bucket>();
                let mut _he_newbkt: *mut UT_hash_bucket = ::core::ptr::null_mut::<UT_hash_bucket>();
                _he_new_buckets = malloc(
                    (2 as size_t)
                        .wrapping_mul((*(*e).hh.tbl).num_buckets as size_t)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                ) as *mut UT_hash_bucket;
                if _he_new_buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        _he_new_buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (2 as size_t)
                            .wrapping_mul((*(*e).hh.tbl).num_buckets as size_t)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                    );
                    (*(*e).hh.tbl).ideal_chain_maxlen = ((*(*e).hh.tbl).num_items
                        >> (*(*e).hh.tbl)
                            .log2_num_buckets
                            .wrapping_add(1 as ::core::ffi::c_uint))
                    .wrapping_add(
                        if (*(*e).hh.tbl).num_items
                            & (*(*e).hh.tbl)
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
                    (*(*e).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                    _he_bkt_i = 0 as ::core::ffi::c_uint;
                    while _he_bkt_i < (*(*e).hh.tbl).num_buckets {
                        _he_thh = (*(*(*e).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UT_hash_handle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*e).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                            (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                            if (*_he_newbkt).count > (*(*e).hh.tbl).ideal_chain_maxlen {
                                (*(*e).hh.tbl).nonideal_items =
                                    (*(*e).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt).expand_mult = (*_he_newbkt)
                                    .count
                                    .wrapping_div((*(*e).hh.tbl).ideal_chain_maxlen);
                            }
                            (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UT_hash_handle;
                            if !(*_he_newbkt).hh_head.is_null() {
                                (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                            }
                            (*_he_newbkt).hh_head = _he_thh as *mut UT_hash_handle;
                            _he_thh = _he_hh_nxt;
                        }
                        _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    }
                    free((*(*e).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    (*(*e).hh.tbl).num_buckets = (*(*e).hh.tbl)
                        .num_buckets
                        .wrapping_mul(2 as ::core::ffi::c_uint);
                    (*(*e).hh.tbl).log2_num_buckets =
                        (*(*e).hh.tbl).log2_num_buckets.wrapping_add(1);
                    (*(*e).hh.tbl).buckets = _he_new_buckets;
                    (*(*e).hh.tbl).ineff_expands = if (*(*e).hh.tbl).nonideal_items
                        > (*(*e).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                    {
                        (*(*e).hh.tbl)
                            .ineff_expands
                            .wrapping_add(1 as ::core::ffi::c_uint)
                    } else {
                        0 as ::core::ffi::c_uint
                    };
                    if (*(*e).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                        (*(*e).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                    }
                }
            }
        }
        j = j.wrapping_add(1);
    }
    let mut allcheck: bool = true;
    let mut j_0: uint32_t = 0 as uint32_t;
    while j_0 < (*b).u.object.length as uint32_t {
        let mut k_0: *mut ::core::ffi::c_char = (*(*b).u.object.values.offset(j_0 as isize)).name;
        let mut e_0: *mut json_obj_entry = ::core::ptr::null_mut::<json_obj_entry>();
        let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
        let mut _hj_i_1: ::core::ffi::c_uint = 0;
        let mut _hj_j_1: ::core::ffi::c_uint = 0;
        let mut _hj_k_1: ::core::ffi::c_uint = 0;
        let mut _hj_key_1: *const ::core::ffi::c_uchar = k_0 as *const ::core::ffi::c_uchar;
        _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_1 = _hj_j_1;
        _hj_k_1 = strlen(k_0) as ::core::ffi::c_uint;
        while _hj_k_1 >= 12 as ::core::ffi::c_uint {
            _hj_i_1 = _hj_i_1.wrapping_add(
                (*_hj_key_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j_1 = _hj_j_1.wrapping_add(
                (*_hj_key_1.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_1.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
            _hj_key_1 = _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _hf_hashv_0 = _hf_hashv_0.wrapping_add(strlen(k_0) as ::core::ffi::c_uint);
        let mut current_block_360: u64;
        match _hj_k_1 {
            11 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_360 = 7320604406085031110;
            }
            10 => {
                current_block_360 = 7320604406085031110;
            }
            9 => {
                current_block_360 = 7496737740521371255;
            }
            8 => {
                current_block_360 = 9235732285572946732;
            }
            7 => {
                current_block_360 = 14590825336193814119;
            }
            6 => {
                current_block_360 = 17563157546870414475;
            }
            5 => {
                current_block_360 = 15705713603850667399;
            }
            4 => {
                current_block_360 = 2257294634313950809;
            }
            3 => {
                current_block_360 = 7914844113294112577;
            }
            2 => {
                current_block_360 = 5747291758229515797;
            }
            1 => {
                current_block_360 = 17049613883679747052;
            }
            _ => {
                current_block_360 = 9260825484694736987;
            }
        }
        match current_block_360 {
            7320604406085031110 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_360 = 7496737740521371255;
            }
            _ => {}
        }
        match current_block_360 {
            7496737740521371255 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_360 = 9235732285572946732;
            }
            _ => {}
        }
        match current_block_360 {
            9235732285572946732 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_360 = 14590825336193814119;
            }
            _ => {}
        }
        match current_block_360 {
            14590825336193814119 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_360 = 17563157546870414475;
            }
            _ => {}
        }
        match current_block_360 {
            17563157546870414475 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_360 = 15705713603850667399;
            }
            _ => {}
        }
        match current_block_360 {
            15705713603850667399 => {
                _hj_j_1 =
                    _hj_j_1
                        .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_360 = 2257294634313950809;
            }
            _ => {}
        }
        match current_block_360 {
            2257294634313950809 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_360 = 7914844113294112577;
            }
            _ => {}
        }
        match current_block_360 {
            7914844113294112577 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_360 = 5747291758229515797;
            }
            _ => {}
        }
        match current_block_360 {
            5747291758229515797 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_360 = 17049613883679747052;
            }
            _ => {}
        }
        match current_block_360 {
            17049613883679747052 => {
                _hj_i_1 =
                    _hj_i_1
                        .wrapping_add(*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
        _hj_i_1 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
        _hf_hashv_0 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
        _hj_i_1 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
        _hf_hashv_0 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
        _hj_i_1 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
        _hf_hashv_0 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
        e_0 = ::core::ptr::null_mut::<json_obj_entry>();
        if !h.is_null() {
            let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
            _hf_bkt_0 = _hf_hashv_0
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if !(*(*(*h).hh.tbl).buckets.offset(_hf_bkt_0 as isize))
                    .hh_head
                    .is_null()
                {
                    e_0 = ((*(*(*h).hh.tbl).buckets.offset(_hf_bkt_0 as isize)).hh_head
                        as *mut ::core::ffi::c_char)
                        .offset(-(*(*h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut json_obj_entry
                        as *mut json_obj_entry;
                } else {
                    e_0 = ::core::ptr::null_mut::<json_obj_entry>();
                }
                while !e_0.is_null() {
                    if (*e_0).hh.hashv == _hf_hashv_0
                        && (*e_0).hh.keylen == strlen(k_0) as ::core::ffi::c_uint
                    {
                        if memcmp(
                            (*e_0).hh.key,
                            k_0 as *const ::core::ffi::c_void,
                            strlen(k_0) as ::core::ffi::c_uint as size_t,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*e_0).hh.hh_next.is_null() {
                        e_0 = ((*e_0).hh.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut json_obj_entry
                            as *mut json_obj_entry;
                    } else {
                        e_0 = ::core::ptr::null_mut::<json_obj_entry>();
                    }
                }
            }
        }
        if e_0.is_null() {
            allcheck = false;
            break;
        } else {
            let mut check: bool = json_ident(
                (*e_0).val,
                (*(*b).u.object.values.offset(j_0 as isize)).value,
            );
            if !check {
                allcheck = false;
                break;
            } else {
                (*e_0).check = true;
                j_0 = j_0.wrapping_add(1);
            }
        }
    }
    let mut e_1: *mut json_obj_entry = ::core::ptr::null_mut::<json_obj_entry>();
    let mut tmp: *mut json_obj_entry = ::core::ptr::null_mut::<json_obj_entry>();
    e_1 = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut json_obj_entry
        as *mut json_obj_entry;
    while !e_1.is_null() {
        allcheck = allcheck as ::core::ffi::c_int != 0 && (*e_1).check as ::core::ffi::c_int != 0;
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*e_1).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<json_obj_entry>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut json_obj_entry as *mut json_obj_entry;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_sub(1);
        }
        free((*e_1).key as *mut ::core::ffi::c_void);
        (*e_1).key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free(e_1 as *mut ::core::ffi::c_void);
        e_1 = ::core::ptr::null_mut::<json_obj_entry>();
        e_1 = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut json_obj_entry
            as *mut json_obj_entry;
    }
    return allcheck;
}
#[no_mangle]
pub unsafe extern "C" fn json_ident(mut a: *const json_value, mut b: *const json_value) -> bool {
    if a.is_null() && b.is_null() {
        return true;
    }
    if a.is_null() || b.is_null() {
        return false;
    }
    if (*a).type_0 as ::core::ffi::c_uint != (*b).type_0 as ::core::ffi::c_uint {
        return false;
    }
    match (*a).type_0 as ::core::ffi::c_uint {
        0 | 7 => return true,
        3 => return (*a).u.integer == (*b).u.integer,
        4 => return (*a).u.dbl == (*b).u.dbl,
        6 => return (*a).u.boolean == (*b).u.boolean,
        5 => {
            return (*a).u.string.length == (*b).u.string.length
                && strcmp((*a).u.string.ptr, (*b).u.string.ptr) == 0 as ::core::ffi::c_int;
        }
        2 => {
            return (*a).u.array.length == (*b).u.array.length
                && compare_json_arrays(a, b) as ::core::ffi::c_int != 0;
        }
        1 => {
            return (*a).u.object.length == (*b).u.object.length
                && compare_json_objects(a, b) as ::core::ffi::c_int != 0;
        }
        _ => return false,
    };
}
