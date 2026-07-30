#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset};

use crate::table::otl::classdef::{ClassDef, otl_class_def_create, push_class_def};
use crate::table::otl::coverage::{Coverage};
use crate::support::handle::{handle_from_consolidated, handle_from_index, otfcc_handle_dup, Handle, GlyphHandle, LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{Buffer};
use crate::support::primitives::{GlyphClass, GlyphId, TableId};
use crate::vendor::sds::{SdsRaw};

use crate::support::{NULL};
use crate::table::otl::{ChainLookupApplication, ChainingRule, Lookup, Subtable, ChainingType, ChainingSubtable};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::table::otl::subtables::chaining::build::{otfcc_build_chaining, otfcc_build_contextual, otfcc_chaining_lookup_is_contextual_lookup};
use crate::table::otl::subtables::chaining::common::{I_SUBTABLE_CHAINING};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClassifierHash {
    pub gid: ::core::ffi::c_int,
    pub gname: SdsRaw,
    pub cls: ::core::ffi::c_int,
    pub hh: UtHashHandle,
}
unsafe extern "C" fn by_gid_clsh(
    mut a: *mut ClassifierHash,
    mut b: *mut ClassifierHash,
) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
unsafe extern "C" fn class_compatible(
    mut h: *mut *mut ClassifierHash,
    mut cov: *mut Coverage,
    mut past: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
    if (*cov).num_glyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    let mut gid: ::core::ffi::c_int =
        (*(*cov).glyphs.offset(0 as ::core::ffi::c_int as isize)).index as ::core::ffi::c_int;
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut gid as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
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
    _hf_hashv =
        _hf_hashv.wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
    let mut current_block_52: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 17973765248428962823;
        }
        10 => {
            current_block_52 = 17973765248428962823;
        }
        9 => {
            current_block_52 = 10894406484810894095;
        }
        8 => {
            current_block_52 = 5827541530980171595;
        }
        7 => {
            current_block_52 = 17297664016000909940;
        }
        6 => {
            current_block_52 = 5722522607137982007;
        }
        5 => {
            current_block_52 = 14113050768010423039;
        }
        4 => {
            current_block_52 = 3051017608055437122;
        }
        3 => {
            current_block_52 = 16033535018094182927;
        }
        2 => {
            current_block_52 = 391718139872164506;
        }
        1 => {
            current_block_52 = 7289879652441223483;
        }
        _ => {
            current_block_52 = 12997042908615822766;
        }
    }
    match current_block_52 {
        17973765248428962823 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 10894406484810894095;
        }
        _ => {}
    }
    match current_block_52 {
        10894406484810894095 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 5827541530980171595;
        }
        _ => {}
    }
    match current_block_52 {
        5827541530980171595 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 17297664016000909940;
        }
        _ => {}
    }
    match current_block_52 {
        17297664016000909940 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 5722522607137982007;
        }
        _ => {}
    }
    match current_block_52 {
        5722522607137982007 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 14113050768010423039;
        }
        _ => {}
    }
    match current_block_52 {
        14113050768010423039 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_52 = 3051017608055437122;
        }
        _ => {}
    }
    match current_block_52 {
        3051017608055437122 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 16033535018094182927;
        }
        _ => {}
    }
    match current_block_52 {
        16033535018094182927 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 391718139872164506;
        }
        _ => {}
    }
    match current_block_52 {
        391718139872164506 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 7289879652441223483;
        }
        _ => {}
    }
    match current_block_52 {
        7289879652441223483 => {
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
    s = ::core::ptr::null_mut::<ClassifierHash>();
    if !(*h).is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(**h).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                s = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(**h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut ClassifierHash
                    as *mut ClassifierHash;
            } else {
                s = ::core::ptr::null_mut::<ClassifierHash>();
            }
            while !s.is_null() {
                if (*s).hh.hashv == _hf_hashv
                    && (*s).hh.keylen as usize
                        == ::core::mem::size_of::<::core::ffi::c_int>()
                {
                    if memcmp(
                        (*s).hh.key,
                        &raw mut gid as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh.hh_next.is_null() {
                    s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(**h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut ClassifierHash
                        as *mut ClassifierHash;
                } else {
                    s = ::core::ptr::null_mut::<ClassifierHash>();
                }
            }
        }
    }
    if !s.is_null() {
        let mut ss: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
        let mut tmp: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
        let mut j: GlyphId = 1 as GlyphId;
        while (j as ::core::ffi::c_int) < (*cov).num_glyphs as ::core::ffi::c_int {
            let mut gid_0: ::core::ffi::c_int =
                (*(*cov).glyphs.offset(j as isize)).index as ::core::ffi::c_int;
            let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
            let mut _hj_i_0: ::core::ffi::c_uint = 0;
            let mut _hj_j_0: ::core::ffi::c_uint = 0;
            let mut _hj_k_0: ::core::ffi::c_uint = 0;
            let mut _hj_key_0: *const ::core::ffi::c_uchar =
                &raw mut gid_0 as *const ::core::ffi::c_uchar;
            _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_0 = _hj_j_0;
            _hj_k_0 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
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
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
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
                _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                _hj_i_0 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                _hf_hashv_0 ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                _hj_i_0 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                _hf_hashv_0 ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
                _hj_i_0 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
                _hf_hashv_0 ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
                _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
                _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _hf_hashv_0 = _hf_hashv_0
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_166: u64;
            match _hj_k_0 {
                11 => {
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 16062721960627083313;
                }
                10 => {
                    current_block_166 = 16062721960627083313;
                }
                9 => {
                    current_block_166 = 13892574498553082172;
                }
                8 => {
                    current_block_166 = 12778119795020627781;
                }
                7 => {
                    current_block_166 = 14370374874000929669;
                }
                6 => {
                    current_block_166 = 90724059652043718;
                }
                5 => {
                    current_block_166 = 2134548532439857700;
                }
                4 => {
                    current_block_166 = 2659614451833647169;
                }
                3 => {
                    current_block_166 = 4264577162065482447;
                }
                2 => {
                    current_block_166 = 3213742308163944612;
                }
                1 => {
                    current_block_166 = 648665693531906211;
                }
                _ => {
                    current_block_166 = 5431927413890720344;
                }
            }
            match current_block_166 {
                16062721960627083313 => {
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 13892574498553082172;
                }
                _ => {}
            }
            match current_block_166 {
                13892574498553082172 => {
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 12778119795020627781;
                }
                _ => {}
            }
            match current_block_166 {
                12778119795020627781 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 14370374874000929669;
                }
                _ => {}
            }
            match current_block_166 {
                14370374874000929669 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 90724059652043718;
                }
                _ => {}
            }
            match current_block_166 {
                90724059652043718 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 2134548532439857700;
                }
                _ => {}
            }
            match current_block_166 {
                2134548532439857700 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_166 = 2659614451833647169;
                }
                _ => {}
            }
            match current_block_166 {
                2659614451833647169 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 4264577162065482447;
                }
                _ => {}
            }
            match current_block_166 {
                4264577162065482447 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 3213742308163944612;
                }
                _ => {}
            }
            match current_block_166 {
                3213742308163944612 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 648665693531906211;
                }
                _ => {}
            }
            match current_block_166 {
                648665693531906211 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                }
                _ => {}
            }
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
            _hj_i_0 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
            _hf_hashv_0 ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
            _hj_i_0 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
            _hf_hashv_0 ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
            _hj_i_0 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
            _hf_hashv_0 ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            ss = ::core::ptr::null_mut::<ClassifierHash>();
            if !(*h).is_null() {
                let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
                _hf_bkt_0 = _hf_hashv_0
                    & (*(**h).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt_0 as isize))
                        .hh_head
                        .is_null()
                    {
                        ss = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt_0 as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ClassifierHash
                            as *mut ClassifierHash;
                    } else {
                        ss = ::core::ptr::null_mut::<ClassifierHash>();
                    }
                    while !ss.is_null() {
                        if (*ss).hh.hashv == _hf_hashv_0
                            && (*ss).hh.keylen as usize
                                == ::core::mem::size_of::<::core::ffi::c_int>()
                        {
                            if memcmp(
                                (*ss).hh.key,
                                &raw mut gid_0 as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*ss).hh.hh_next.is_null() {
                            ss = ((*ss).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut ClassifierHash
                                as *mut ClassifierHash;
                        } else {
                            ss = ::core::ptr::null_mut::<ClassifierHash>();
                        }
                    }
                }
            }
            if ss.is_null() || (*ss).cls != (*s).cls {
                return 0 as ::core::ffi::c_int;
            }
            j = j.wrapping_add(1);
        }
        let mut revh: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
        let mut j_0: GlyphId = 0 as GlyphId;
        while (j_0 as ::core::ffi::c_int) < (*cov).num_glyphs as ::core::ffi::c_int {
            let mut gid_1: ::core::ffi::c_int =
                (*(*cov).glyphs.offset(j_0 as isize)).index as ::core::ffi::c_int;
            let mut rss: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
            let mut _hf_hashv_1: ::core::ffi::c_uint = 0;
            let mut _hj_i_1: ::core::ffi::c_uint = 0;
            let mut _hj_j_1: ::core::ffi::c_uint = 0;
            let mut _hj_k_1: ::core::ffi::c_uint = 0;
            let mut _hj_key_1: *const ::core::ffi::c_uchar =
                &raw mut gid_1 as *const ::core::ffi::c_uchar;
            _hf_hashv_1 = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_1 = _hj_j_1;
            _hj_k_1 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
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
                _hf_hashv_1 = _hf_hashv_1.wrapping_add(
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
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                _hj_i_1 ^= _hf_hashv_1 >> 13 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                _hf_hashv_1 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                _hj_i_1 ^= _hf_hashv_1 >> 12 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                _hf_hashv_1 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                _hj_i_1 ^= _hf_hashv_1 >> 3 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                _hf_hashv_1 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
                _hj_key_1 = _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
                _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _hf_hashv_1 = _hf_hashv_1
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_284: u64;
            match _hj_k_1 {
                11 => {
                    _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                        (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_284 = 6971704384790945690;
                }
                10 => {
                    current_block_284 = 6971704384790945690;
                }
                9 => {
                    current_block_284 = 15431556227574440530;
                }
                8 => {
                    current_block_284 = 8617618040855383894;
                }
                7 => {
                    current_block_284 = 15718843545605209551;
                }
                6 => {
                    current_block_284 = 15143330236766987690;
                }
                5 => {
                    current_block_284 = 4222273867521650206;
                }
                4 => {
                    current_block_284 = 3054179581528647280;
                }
                3 => {
                    current_block_284 = 974532107441439561;
                }
                2 => {
                    current_block_284 = 6724730390713225968;
                }
                1 => {
                    current_block_284 = 1237544401963626511;
                }
                _ => {
                    current_block_284 = 4320962301225890772;
                }
            }
            match current_block_284 {
                6971704384790945690 => {
                    _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                        (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_284 = 15431556227574440530;
                }
                _ => {}
            }
            match current_block_284 {
                15431556227574440530 => {
                    _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                        (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_284 = 8617618040855383894;
                }
                _ => {}
            }
            match current_block_284 {
                8617618040855383894 => {
                    _hj_j_1 = _hj_j_1.wrapping_add(
                        (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_284 = 15718843545605209551;
                }
                _ => {}
            }
            match current_block_284 {
                15718843545605209551 => {
                    _hj_j_1 = _hj_j_1.wrapping_add(
                        (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_284 = 15143330236766987690;
                }
                _ => {}
            }
            match current_block_284 {
                15143330236766987690 => {
                    _hj_j_1 = _hj_j_1.wrapping_add(
                        (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_284 = 4222273867521650206;
                }
                _ => {}
            }
            match current_block_284 {
                4222273867521650206 => {
                    _hj_j_1 = _hj_j_1
                        .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_284 = 3054179581528647280;
                }
                _ => {}
            }
            match current_block_284 {
                3054179581528647280 => {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_284 = 974532107441439561;
                }
                _ => {}
            }
            match current_block_284 {
                974532107441439561 => {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_284 = 6724730390713225968;
                }
                _ => {}
            }
            match current_block_284 {
                6724730390713225968 => {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_284 = 1237544401963626511;
                }
                _ => {}
            }
            match current_block_284 {
                1237544401963626511 => {
                    _hj_i_1 = _hj_i_1
                        .wrapping_add(*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                }
                _ => {}
            }
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
            _hj_i_1 ^= _hf_hashv_1 >> 13 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
            _hf_hashv_1 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
            _hj_i_1 ^= _hf_hashv_1 >> 12 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
            _hf_hashv_1 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
            _hj_i_1 ^= _hf_hashv_1 >> 3 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
            _hf_hashv_1 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
            rss = ::core::ptr::null_mut::<ClassifierHash>();
            if !revh.is_null() {
                let mut _hf_bkt_1: ::core::ffi::c_uint = 0;
                _hf_bkt_1 = _hf_hashv_1
                    & (*(*revh).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(*revh).hh.tbl).buckets.offset(_hf_bkt_1 as isize))
                        .hh_head
                        .is_null()
                    {
                        rss = ((*(*(*revh).hh.tbl).buckets.offset(_hf_bkt_1 as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(*revh).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ClassifierHash
                            as *mut ClassifierHash;
                    } else {
                        rss = ::core::ptr::null_mut::<ClassifierHash>();
                    }
                    while !rss.is_null() {
                        if (*rss).hh.hashv == _hf_hashv_1
                            && (*rss).hh.keylen as usize
                                == ::core::mem::size_of::<::core::ffi::c_int>()
                        {
                            if memcmp(
                                (*rss).hh.key,
                                &raw mut gid_1 as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*rss).hh.hh_next.is_null() {
                            rss = ((*rss).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(*revh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut ClassifierHash
                                as *mut ClassifierHash;
                        } else {
                            rss = ::core::ptr::null_mut::<ClassifierHash>();
                        }
                    }
                }
            }
            if rss.is_null() {
                rss = __caryll_allocate_clean(
                    ::core::mem::size_of::<ClassifierHash>() as usize,
                    38 as ::core::ffi::c_ulong,
                ) as *mut ClassifierHash;
                (*rss).gid = gid_1;
                (*rss).gname = (*(*cov).glyphs.offset(j_0 as isize)).name;
                (*rss).cls = (*s).cls;
                let mut _ha_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i_2: ::core::ffi::c_uint = 0;
                let mut _hj_j_2: ::core::ffi::c_uint = 0;
                let mut _hj_k_2: ::core::ffi::c_uint = 0;
                let mut _hj_key_2: *const ::core::ffi::c_uchar =
                    &raw mut (*rss).gid as *const ::core::ffi::c_uchar;
                _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_2 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_2 = _hj_j_2;
                _hj_k_2 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                while _hj_k_2 >= 12 as ::core::ffi::c_uint {
                    _hj_i_2 = _hj_i_2.wrapping_add(
                        (*_hj_key_2.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_j_2 = _hj_j_2.wrapping_add(
                        (*_hj_key_2.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_2.offset(11 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                    _hj_i_2 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 8 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                    _ha_hashv ^= _hj_j_2 >> 13 as ::core::ffi::c_int;
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                    _hj_i_2 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 16 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                    _ha_hashv ^= _hj_j_2 >> 5 as ::core::ffi::c_int;
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                    _hj_i_2 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 10 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                    _ha_hashv ^= _hj_j_2 >> 15 as ::core::ffi::c_int;
                    _hj_key_2 = _hj_key_2.offset(12 as ::core::ffi::c_int as isize);
                    _hj_k_2 = _hj_k_2.wrapping_sub(12 as ::core::ffi::c_uint);
                }
                _ha_hashv = _ha_hashv.wrapping_add(
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint
                );
                let mut current_block_402: u64;
                match _hj_k_2 {
                    11 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_402 = 3691502689732819101;
                    }
                    10 => {
                        current_block_402 = 3691502689732819101;
                    }
                    9 => {
                        current_block_402 = 10607098743120199864;
                    }
                    8 => {
                        current_block_402 = 8719056897555590949;
                    }
                    7 => {
                        current_block_402 = 1276191155120476369;
                    }
                    6 => {
                        current_block_402 = 4002740024174479193;
                    }
                    5 => {
                        current_block_402 = 12945203596588168180;
                    }
                    4 => {
                        current_block_402 = 16342964101157071148;
                    }
                    3 => {
                        current_block_402 = 17005239525384301966;
                    }
                    2 => {
                        current_block_402 = 2863132192705104890;
                    }
                    1 => {
                        current_block_402 = 7039392228028577323;
                    }
                    _ => {
                        current_block_402 = 14066878990533686878;
                    }
                }
                match current_block_402 {
                    3691502689732819101 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_402 = 10607098743120199864;
                    }
                    _ => {}
                }
                match current_block_402 {
                    10607098743120199864 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_402 = 8719056897555590949;
                    }
                    _ => {}
                }
                match current_block_402 {
                    8719056897555590949 => {
                        _hj_j_2 = _hj_j_2.wrapping_add(
                            (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_402 = 1276191155120476369;
                    }
                    _ => {}
                }
                match current_block_402 {
                    1276191155120476369 => {
                        _hj_j_2 = _hj_j_2.wrapping_add(
                            (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_402 = 4002740024174479193;
                    }
                    _ => {}
                }
                match current_block_402 {
                    4002740024174479193 => {
                        _hj_j_2 = _hj_j_2.wrapping_add(
                            (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_402 = 12945203596588168180;
                    }
                    _ => {}
                }
                match current_block_402 {
                    12945203596588168180 => {
                        _hj_j_2 = _hj_j_2
                            .wrapping_add(*_hj_key_2.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_402 = 16342964101157071148;
                    }
                    _ => {}
                }
                match current_block_402 {
                    16342964101157071148 => {
                        _hj_i_2 = _hj_i_2.wrapping_add(
                            (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_402 = 17005239525384301966;
                    }
                    _ => {}
                }
                match current_block_402 {
                    17005239525384301966 => {
                        _hj_i_2 = _hj_i_2.wrapping_add(
                            (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_402 = 2863132192705104890;
                    }
                    _ => {}
                }
                match current_block_402 {
                    2863132192705104890 => {
                        _hj_i_2 = _hj_i_2.wrapping_add(
                            (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_402 = 7039392228028577323;
                    }
                    _ => {}
                }
                match current_block_402 {
                    7039392228028577323 => {
                        _hj_i_2 = _hj_i_2
                            .wrapping_add(*_hj_key_2.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                _hj_i_2 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                _hj_j_2 ^= _hj_i_2 << 8 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                _ha_hashv ^= _hj_j_2 >> 13 as ::core::ffi::c_int;
                _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                _hj_i_2 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                _hj_j_2 ^= _hj_i_2 << 16 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                _ha_hashv ^= _hj_j_2 >> 5 as ::core::ffi::c_int;
                _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                _hj_i_2 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                _hj_j_2 ^= _hj_i_2 << 10 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                _ha_hashv ^= _hj_j_2 >> 15 as ::core::ffi::c_int;
                (*rss).hh.hashv = _ha_hashv;
                (*rss).hh.key =
                    &raw mut (*rss).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
                (*rss).hh.keylen =
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                if revh.is_null() {
                    (*rss).hh.next = NULL;
                    (*rss).hh.prev = NULL;
                    (*rss).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                        as *mut UtHashTable
                        as *mut UtHashTable;
                    if (*rss).hh.tbl.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*rss).hh.tbl as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            ::core::mem::size_of::<UtHashTable>() as usize,
                        );
                        (*(*rss).hh.tbl).tail = &raw mut (*rss).hh as *mut UtHashHandle;
                        (*(*rss).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                        (*(*rss).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                        (*(*rss).hh.tbl).hho = (&raw mut (*rss).hh as *mut ::core::ffi::c_char)
                            .offset_from(rss as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long
                            as isize;
                        (*(*rss).hh.tbl).buckets = malloc(
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        ) as *mut UtHashBucket;
                        (*(*rss).hh.tbl).signature = HASH_SIGNATURE as u32;
                        if (*(*rss).hh.tbl).buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*(*rss).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (32 as usize).wrapping_mul(
                                    ::core::mem::size_of::<UtHashBucket>() as usize,
                                ),
                            );
                        }
                    }
                    revh = rss;
                } else {
                    (*rss).hh.tbl = (*revh).hh.tbl;
                    (*rss).hh.next = NULL;
                    (*rss).hh.prev = ((*(*revh).hh.tbl).tail as *mut ::core::ffi::c_char)
                        .offset(-(*(*revh).hh.tbl).hho)
                        as *mut ::core::ffi::c_void;
                    (*(*(*revh).hh.tbl).tail).next = rss as *mut ::core::ffi::c_void;
                    (*(*revh).hh.tbl).tail = &raw mut (*rss).hh as *mut UtHashHandle;
                }
                let mut _ha_bkt: ::core::ffi::c_uint = 0;
                (*(*revh).hh.tbl).num_items = (*(*revh).hh.tbl).num_items.wrapping_add(1);
                _ha_bkt = _ha_hashv
                    & (*(*revh).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head: *mut UtHashBucket =
                    (*(*revh).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
                (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                (*rss).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
                (*rss).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                if !(*_ha_head).hh_head.is_null() {
                    (*(*_ha_head).hh_head).hh_prev = &raw mut (*rss).hh as *mut UtHashHandle;
                }
                (*_ha_head).hh_head = &raw mut (*rss).hh as *mut UtHashHandle;
                if (*_ha_head).count
                    >= (*_ha_head)
                        .expand_mult
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                    && (*(*rss).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt: ::core::ffi::c_uint = 0;
                    let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                    let mut _he_thh: *mut UtHashHandle =
                        ::core::ptr::null_mut::<UtHashHandle>();
                    let mut _he_hh_nxt: *mut UtHashHandle =
                        ::core::ptr::null_mut::<UtHashHandle>();
                    let mut _he_new_buckets: *mut UtHashBucket =
                        ::core::ptr::null_mut::<UtHashBucket>();
                    let mut _he_newbkt: *mut UtHashBucket =
                        ::core::ptr::null_mut::<UtHashBucket>();
                    _he_new_buckets = malloc(
                        (2 as usize)
                            .wrapping_mul((*(*rss).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    ) as *mut UtHashBucket;
                    if _he_new_buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            _he_new_buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (2 as usize)
                                .wrapping_mul((*(*rss).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        );
                        (*(*rss).hh.tbl).ideal_chain_maxlen = ((*(*rss).hh.tbl).num_items
                            >> (*(*rss).hh.tbl)
                                .log2_num_buckets
                                .wrapping_add(1 as ::core::ffi::c_uint))
                        .wrapping_add(
                            if (*(*rss).hh.tbl).num_items
                                & (*(*rss).hh.tbl)
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
                        (*(*rss).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                        _he_bkt_i = 0 as ::core::ffi::c_uint;
                        while _he_bkt_i < (*(*rss).hh.tbl).num_buckets {
                            _he_thh = (*(*(*rss).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                                as *mut UtHashHandle;
                            while !_he_thh.is_null() {
                                _he_hh_nxt = (*_he_thh).hh_next;
                                _he_bkt = (*_he_thh).hashv
                                    & (*(*rss).hh.tbl)
                                        .num_buckets
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint);
                                _he_newbkt =
                                    _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                                (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                if (*_he_newbkt).count > (*(*rss).hh.tbl).ideal_chain_maxlen {
                                    (*(*rss).hh.tbl).nonideal_items =
                                        (*(*rss).hh.tbl).nonideal_items.wrapping_add(1);
                                    (*_he_newbkt).expand_mult = (*_he_newbkt)
                                        .count
                                        .wrapping_div((*(*rss).hh.tbl).ideal_chain_maxlen);
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
                        free((*(*rss).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        (*(*rss).hh.tbl).num_buckets = (*(*rss).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint);
                        (*(*rss).hh.tbl).log2_num_buckets =
                            (*(*rss).hh.tbl).log2_num_buckets.wrapping_add(1);
                        (*(*rss).hh.tbl).buckets = _he_new_buckets;
                        (*(*rss).hh.tbl).ineff_expands = if (*(*rss).hh.tbl).nonideal_items
                            > (*(*rss).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                        {
                            (*(*rss).hh.tbl)
                                .ineff_expands
                                .wrapping_add(1 as ::core::ffi::c_uint)
                        } else {
                            0 as ::core::ffi::c_uint
                        };
                        if (*(*rss).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                            (*(*rss).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                        }
                    }
                }
            }
            j_0 = j_0.wrapping_add(1);
        }
        let mut allcheck: bool = true;
        ss = *h;
        while !ss.is_null() {
            if (*ss).cls == (*s).cls {
                let mut gid_2: ::core::ffi::c_int = (*ss).gid;
                let mut rss_0: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
                let mut _hf_hashv_2: ::core::ffi::c_uint = 0;
                let mut _hj_i_3: ::core::ffi::c_uint = 0;
                let mut _hj_j_3: ::core::ffi::c_uint = 0;
                let mut _hj_k_3: ::core::ffi::c_uint = 0;
                let mut _hj_key_3: *const ::core::ffi::c_uchar =
                    &raw mut gid_2 as *const ::core::ffi::c_uchar;
                _hf_hashv_2 = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_3 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_3 = _hj_j_3;
                _hj_k_3 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                while _hj_k_3 >= 12 as ::core::ffi::c_uint {
                    _hj_i_3 = _hj_i_3.wrapping_add(
                        (*_hj_key_3.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_3.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_j_3 = _hj_j_3.wrapping_add(
                        (*_hj_key_3.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_3.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hf_hashv_2 = _hf_hashv_2.wrapping_add(
                        (*_hj_key_3.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_3.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_3.offset(11 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                    _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_2);
                    _hj_i_3 ^= _hf_hashv_2 >> 13 as ::core::ffi::c_int;
                    _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_2);
                    _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                    _hj_j_3 ^= _hj_i_3 << 8 as ::core::ffi::c_int;
                    _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_3);
                    _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_3);
                    _hf_hashv_2 ^= _hj_j_3 >> 13 as ::core::ffi::c_int;
                    _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                    _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_2);
                    _hj_i_3 ^= _hf_hashv_2 >> 12 as ::core::ffi::c_int;
                    _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_2);
                    _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                    _hj_j_3 ^= _hj_i_3 << 16 as ::core::ffi::c_int;
                    _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_3);
                    _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_3);
                    _hf_hashv_2 ^= _hj_j_3 >> 5 as ::core::ffi::c_int;
                    _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                    _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_2);
                    _hj_i_3 ^= _hf_hashv_2 >> 3 as ::core::ffi::c_int;
                    _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_2);
                    _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                    _hj_j_3 ^= _hj_i_3 << 10 as ::core::ffi::c_int;
                    _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_3);
                    _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_3);
                    _hf_hashv_2 ^= _hj_j_3 >> 15 as ::core::ffi::c_int;
                    _hj_key_3 = _hj_key_3.offset(12 as ::core::ffi::c_int as isize);
                    _hj_k_3 = _hj_k_3.wrapping_sub(12 as ::core::ffi::c_uint);
                }
                _hf_hashv_2 = _hf_hashv_2.wrapping_add(
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint,
                );
                let mut current_block_595: u64;
                match _hj_k_3 {
                    11 => {
                        _hf_hashv_2 = _hf_hashv_2.wrapping_add(
                            (*_hj_key_3.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_595 = 6772833500325681427;
                    }
                    10 => {
                        current_block_595 = 6772833500325681427;
                    }
                    9 => {
                        current_block_595 = 6915265441720649259;
                    }
                    8 => {
                        current_block_595 = 7250900328645245868;
                    }
                    7 => {
                        current_block_595 = 6449899445877603964;
                    }
                    6 => {
                        current_block_595 = 3435458315056947510;
                    }
                    5 => {
                        current_block_595 = 12530574122810026297;
                    }
                    4 => {
                        current_block_595 = 18288536805647483271;
                    }
                    3 => {
                        current_block_595 = 1961997521763284341;
                    }
                    2 => {
                        current_block_595 = 1564725485732696560;
                    }
                    1 => {
                        current_block_595 = 11894547121031089777;
                    }
                    _ => {
                        current_block_595 = 13651177166292620943;
                    }
                }
                match current_block_595 {
                    6772833500325681427 => {
                        _hf_hashv_2 = _hf_hashv_2.wrapping_add(
                            (*_hj_key_3.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_595 = 6915265441720649259;
                    }
                    _ => {}
                }
                match current_block_595 {
                    6915265441720649259 => {
                        _hf_hashv_2 = _hf_hashv_2.wrapping_add(
                            (*_hj_key_3.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_595 = 7250900328645245868;
                    }
                    _ => {}
                }
                match current_block_595 {
                    7250900328645245868 => {
                        _hj_j_3 = _hj_j_3.wrapping_add(
                            (*_hj_key_3.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_595 = 6449899445877603964;
                    }
                    _ => {}
                }
                match current_block_595 {
                    6449899445877603964 => {
                        _hj_j_3 = _hj_j_3.wrapping_add(
                            (*_hj_key_3.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_595 = 3435458315056947510;
                    }
                    _ => {}
                }
                match current_block_595 {
                    3435458315056947510 => {
                        _hj_j_3 = _hj_j_3.wrapping_add(
                            (*_hj_key_3.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_595 = 12530574122810026297;
                    }
                    _ => {}
                }
                match current_block_595 {
                    12530574122810026297 => {
                        _hj_j_3 = _hj_j_3
                            .wrapping_add(*_hj_key_3.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_595 = 18288536805647483271;
                    }
                    _ => {}
                }
                match current_block_595 {
                    18288536805647483271 => {
                        _hj_i_3 = _hj_i_3.wrapping_add(
                            (*_hj_key_3.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_595 = 1961997521763284341;
                    }
                    _ => {}
                }
                match current_block_595 {
                    1961997521763284341 => {
                        _hj_i_3 = _hj_i_3.wrapping_add(
                            (*_hj_key_3.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_595 = 1564725485732696560;
                    }
                    _ => {}
                }
                match current_block_595 {
                    1564725485732696560 => {
                        _hj_i_3 = _hj_i_3.wrapping_add(
                            (*_hj_key_3.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_595 = 11894547121031089777;
                    }
                    _ => {}
                }
                match current_block_595 {
                    11894547121031089777 => {
                        _hj_i_3 = _hj_i_3
                            .wrapping_add(*_hj_key_3.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_2);
                _hj_i_3 ^= _hf_hashv_2 >> 13 as ::core::ffi::c_int;
                _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_2);
                _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                _hj_j_3 ^= _hj_i_3 << 8 as ::core::ffi::c_int;
                _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_3);
                _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_3);
                _hf_hashv_2 ^= _hj_j_3 >> 13 as ::core::ffi::c_int;
                _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_2);
                _hj_i_3 ^= _hf_hashv_2 >> 12 as ::core::ffi::c_int;
                _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_2);
                _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                _hj_j_3 ^= _hj_i_3 << 16 as ::core::ffi::c_int;
                _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_3);
                _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_3);
                _hf_hashv_2 ^= _hj_j_3 >> 5 as ::core::ffi::c_int;
                _hj_i_3 = _hj_i_3.wrapping_sub(_hj_j_3);
                _hj_i_3 = _hj_i_3.wrapping_sub(_hf_hashv_2);
                _hj_i_3 ^= _hf_hashv_2 >> 3 as ::core::ffi::c_int;
                _hj_j_3 = _hj_j_3.wrapping_sub(_hf_hashv_2);
                _hj_j_3 = _hj_j_3.wrapping_sub(_hj_i_3);
                _hj_j_3 ^= _hj_i_3 << 10 as ::core::ffi::c_int;
                _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_i_3);
                _hf_hashv_2 = _hf_hashv_2.wrapping_sub(_hj_j_3);
                _hf_hashv_2 ^= _hj_j_3 >> 15 as ::core::ffi::c_int;
                rss_0 = ::core::ptr::null_mut::<ClassifierHash>();
                if !revh.is_null() {
                    let mut _hf_bkt_2: ::core::ffi::c_uint = 0;
                    _hf_bkt_2 = _hf_hashv_2
                        & (*(*revh).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if !(*(*(*revh).hh.tbl).buckets.offset(_hf_bkt_2 as isize))
                            .hh_head
                            .is_null()
                        {
                            rss_0 = ((*(*(*revh).hh.tbl).buckets.offset(_hf_bkt_2 as isize)).hh_head
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(*revh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut ClassifierHash
                                as *mut ClassifierHash;
                        } else {
                            rss_0 = ::core::ptr::null_mut::<ClassifierHash>();
                        }
                        while !rss_0.is_null() {
                            if (*rss_0).hh.hashv == _hf_hashv_2
                                && (*rss_0).hh.keylen as usize
                                    == ::core::mem::size_of::<::core::ffi::c_int>()
                            {
                                if memcmp(
                                    (*rss_0).hh.key,
                                    &raw mut gid_2 as *const ::core::ffi::c_void,
                                    ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if !(*rss_0).hh.hh_next.is_null() {
                                rss_0 = ((*rss_0).hh.hh_next as *mut ::core::ffi::c_char)
                                    .offset(-(*(*revh).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut ClassifierHash
                                    as *mut ClassifierHash;
                            } else {
                                rss_0 = ::core::ptr::null_mut::<ClassifierHash>();
                            }
                        }
                    }
                }
                if rss_0.is_null() {
                    allcheck = false;
                    break;
                }
            }
            ss = (*ss).hh.next as *mut ClassifierHash;
        }
        ss = revh;
        tmp = (if !revh.is_null() {
            (*revh).hh.next
        } else {
            NULL
        }) as *mut ClassifierHash as *mut ClassifierHash;
        while !ss.is_null() {
            let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*ss).hh;
            if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                free((*(*revh).hh.tbl).buckets as *mut ::core::ffi::c_void);
                free((*revh).hh.tbl as *mut ::core::ffi::c_void);
                revh = ::core::ptr::null_mut::<ClassifierHash>();
            } else {
                let mut _hd_bkt: ::core::ffi::c_uint = 0;
                if _hd_hh_del == (*(*revh).hh.tbl).tail {
                    (*(*revh).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*revh).hh.tbl).hho)
                        as *mut UtHashHandle
                        as *mut UtHashHandle;
                }
                if !(*_hd_hh_del).prev.is_null() {
                    let ref mut fresh11 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*revh).hh.tbl).hho)
                        as *mut UtHashHandle))
                        .next;
                    *fresh11 = (*_hd_hh_del).next;
                } else {
                    revh = (*_hd_hh_del).next as *mut ClassifierHash as *mut ClassifierHash;
                }
                if !(*_hd_hh_del).next.is_null() {
                    let ref mut fresh12 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                        .offset((*(*revh).hh.tbl).hho)
                        as *mut UtHashHandle))
                        .prev;
                    *fresh12 = (*_hd_hh_del).prev;
                }
                _hd_bkt = (*_hd_hh_del).hashv
                    & (*(*revh).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _hd_head: *mut UtHashBucket =
                    (*(*revh).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
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
                (*(*revh).hh.tbl).num_items = (*(*revh).hh.tbl).num_items.wrapping_sub(1);
            }
            free(ss as *mut ::core::ffi::c_void);
            ss = ::core::ptr::null_mut::<ClassifierHash>();
            ss = tmp;
            tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut ClassifierHash
                as *mut ClassifierHash;
        }
        return if allcheck as ::core::ffi::c_int != 0 {
            (*s).cls
        } else {
            0 as ::core::ffi::c_int
        };
    } else {
        let mut ss_0: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
        let mut j_1: GlyphId = 1 as GlyphId;
        while (j_1 as ::core::ffi::c_int) < (*cov).num_glyphs as ::core::ffi::c_int {
            let mut gid_3: ::core::ffi::c_int =
                (*(*cov).glyphs.offset(j_1 as isize)).index as ::core::ffi::c_int;
            let mut _hf_hashv_3: ::core::ffi::c_uint = 0;
            let mut _hj_i_4: ::core::ffi::c_uint = 0;
            let mut _hj_j_4: ::core::ffi::c_uint = 0;
            let mut _hj_k_4: ::core::ffi::c_uint = 0;
            let mut _hj_key_4: *const ::core::ffi::c_uchar =
                &raw mut gid_3 as *const ::core::ffi::c_uchar;
            _hf_hashv_3 = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_4 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_4 = _hj_j_4;
            _hj_k_4 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            while _hj_k_4 >= 12 as ::core::ffi::c_uint {
                _hj_i_4 = _hj_i_4.wrapping_add(
                    (*_hj_key_4.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_4.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j_4 = _hj_j_4.wrapping_add(
                    (*_hj_key_4.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_4.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv_3 = _hf_hashv_3.wrapping_add(
                    (*_hj_key_4.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_4.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_4.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
                _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_3);
                _hj_i_4 ^= _hf_hashv_3 >> 13 as ::core::ffi::c_int;
                _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_3);
                _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
                _hj_j_4 ^= _hj_i_4 << 8 as ::core::ffi::c_int;
                _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_i_4);
                _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_j_4);
                _hf_hashv_3 ^= _hj_j_4 >> 13 as ::core::ffi::c_int;
                _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
                _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_3);
                _hj_i_4 ^= _hf_hashv_3 >> 12 as ::core::ffi::c_int;
                _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_3);
                _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
                _hj_j_4 ^= _hj_i_4 << 16 as ::core::ffi::c_int;
                _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_i_4);
                _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_j_4);
                _hf_hashv_3 ^= _hj_j_4 >> 5 as ::core::ffi::c_int;
                _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
                _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_3);
                _hj_i_4 ^= _hf_hashv_3 >> 3 as ::core::ffi::c_int;
                _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_3);
                _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
                _hj_j_4 ^= _hj_i_4 << 10 as ::core::ffi::c_int;
                _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_i_4);
                _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_j_4);
                _hf_hashv_3 ^= _hj_j_4 >> 15 as ::core::ffi::c_int;
                _hj_key_4 = _hj_key_4.offset(12 as ::core::ffi::c_int as isize);
                _hj_k_4 = _hj_k_4.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _hf_hashv_3 = _hf_hashv_3
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_756: u64;
            match _hj_k_4 {
                11 => {
                    _hf_hashv_3 = _hf_hashv_3.wrapping_add(
                        (*_hj_key_4.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_756 = 14823390095034223592;
                }
                10 => {
                    current_block_756 = 14823390095034223592;
                }
                9 => {
                    current_block_756 = 9585313542997869438;
                }
                8 => {
                    current_block_756 = 6805930155676632364;
                }
                7 => {
                    current_block_756 = 9551776425448678334;
                }
                6 => {
                    current_block_756 = 8698763507244745683;
                }
                5 => {
                    current_block_756 = 5780628887795834159;
                }
                4 => {
                    current_block_756 = 3824249416376747117;
                }
                3 => {
                    current_block_756 = 12722485907546857267;
                }
                2 => {
                    current_block_756 = 10433906804748019285;
                }
                1 => {
                    current_block_756 = 8417752359729347026;
                }
                _ => {
                    current_block_756 = 18170962488665502312;
                }
            }
            match current_block_756 {
                14823390095034223592 => {
                    _hf_hashv_3 = _hf_hashv_3.wrapping_add(
                        (*_hj_key_4.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_756 = 9585313542997869438;
                }
                _ => {}
            }
            match current_block_756 {
                9585313542997869438 => {
                    _hf_hashv_3 = _hf_hashv_3.wrapping_add(
                        (*_hj_key_4.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_756 = 6805930155676632364;
                }
                _ => {}
            }
            match current_block_756 {
                6805930155676632364 => {
                    _hj_j_4 = _hj_j_4.wrapping_add(
                        (*_hj_key_4.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_756 = 9551776425448678334;
                }
                _ => {}
            }
            match current_block_756 {
                9551776425448678334 => {
                    _hj_j_4 = _hj_j_4.wrapping_add(
                        (*_hj_key_4.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_756 = 8698763507244745683;
                }
                _ => {}
            }
            match current_block_756 {
                8698763507244745683 => {
                    _hj_j_4 = _hj_j_4.wrapping_add(
                        (*_hj_key_4.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_756 = 5780628887795834159;
                }
                _ => {}
            }
            match current_block_756 {
                5780628887795834159 => {
                    _hj_j_4 = _hj_j_4
                        .wrapping_add(*_hj_key_4.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_756 = 3824249416376747117;
                }
                _ => {}
            }
            match current_block_756 {
                3824249416376747117 => {
                    _hj_i_4 = _hj_i_4.wrapping_add(
                        (*_hj_key_4.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_756 = 12722485907546857267;
                }
                _ => {}
            }
            match current_block_756 {
                12722485907546857267 => {
                    _hj_i_4 = _hj_i_4.wrapping_add(
                        (*_hj_key_4.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_756 = 10433906804748019285;
                }
                _ => {}
            }
            match current_block_756 {
                10433906804748019285 => {
                    _hj_i_4 = _hj_i_4.wrapping_add(
                        (*_hj_key_4.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_756 = 8417752359729347026;
                }
                _ => {}
            }
            match current_block_756 {
                8417752359729347026 => {
                    _hj_i_4 = _hj_i_4
                        .wrapping_add(*_hj_key_4.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                }
                _ => {}
            }
            _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_3);
            _hj_i_4 ^= _hf_hashv_3 >> 13 as ::core::ffi::c_int;
            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_3);
            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
            _hj_j_4 ^= _hj_i_4 << 8 as ::core::ffi::c_int;
            _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_i_4);
            _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_j_4);
            _hf_hashv_3 ^= _hj_j_4 >> 13 as ::core::ffi::c_int;
            _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_3);
            _hj_i_4 ^= _hf_hashv_3 >> 12 as ::core::ffi::c_int;
            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_3);
            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
            _hj_j_4 ^= _hj_i_4 << 16 as ::core::ffi::c_int;
            _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_i_4);
            _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_j_4);
            _hf_hashv_3 ^= _hj_j_4 >> 5 as ::core::ffi::c_int;
            _hj_i_4 = _hj_i_4.wrapping_sub(_hj_j_4);
            _hj_i_4 = _hj_i_4.wrapping_sub(_hf_hashv_3);
            _hj_i_4 ^= _hf_hashv_3 >> 3 as ::core::ffi::c_int;
            _hj_j_4 = _hj_j_4.wrapping_sub(_hf_hashv_3);
            _hj_j_4 = _hj_j_4.wrapping_sub(_hj_i_4);
            _hj_j_4 ^= _hj_i_4 << 10 as ::core::ffi::c_int;
            _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_i_4);
            _hf_hashv_3 = _hf_hashv_3.wrapping_sub(_hj_j_4);
            _hf_hashv_3 ^= _hj_j_4 >> 15 as ::core::ffi::c_int;
            ss_0 = ::core::ptr::null_mut::<ClassifierHash>();
            if !(*h).is_null() {
                let mut _hf_bkt_3: ::core::ffi::c_uint = 0;
                _hf_bkt_3 = _hf_hashv_3
                    & (*(**h).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt_3 as isize))
                        .hh_head
                        .is_null()
                    {
                        ss_0 = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt_3 as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ClassifierHash
                            as *mut ClassifierHash;
                    } else {
                        ss_0 = ::core::ptr::null_mut::<ClassifierHash>();
                    }
                    while !ss_0.is_null() {
                        if (*ss_0).hh.hashv == _hf_hashv_3
                            && (*ss_0).hh.keylen as usize
                                == ::core::mem::size_of::<::core::ffi::c_int>()
                        {
                            if memcmp(
                                (*ss_0).hh.key,
                                &raw mut gid_3 as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*ss_0).hh.hh_next.is_null() {
                            ss_0 = ((*ss_0).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut ClassifierHash
                                as *mut ClassifierHash;
                        } else {
                            ss_0 = ::core::ptr::null_mut::<ClassifierHash>();
                        }
                    }
                }
            }
            if !ss_0.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            j_1 = j_1.wrapping_add(1);
        }
        let mut j_2: GlyphId = 0 as GlyphId;
        while (j_2 as ::core::ffi::c_int) < (*cov).num_glyphs as ::core::ffi::c_int {
            let mut gid_4: ::core::ffi::c_int =
                (*(*cov).glyphs.offset(j_2 as isize)).index as ::core::ffi::c_int;
            let mut s_0: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
            let mut _hf_hashv_4: ::core::ffi::c_uint = 0;
            let mut _hj_i_5: ::core::ffi::c_uint = 0;
            let mut _hj_j_5: ::core::ffi::c_uint = 0;
            let mut _hj_k_5: ::core::ffi::c_uint = 0;
            let mut _hj_key_5: *const ::core::ffi::c_uchar =
                &raw mut gid_4 as *const ::core::ffi::c_uchar;
            _hf_hashv_4 = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_5 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_5 = _hj_j_5;
            _hj_k_5 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            while _hj_k_5 >= 12 as ::core::ffi::c_uint {
                _hj_i_5 = _hj_i_5.wrapping_add(
                    (*_hj_key_5.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_5.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_5.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_5.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j_5 = _hj_j_5.wrapping_add(
                    (*_hj_key_5.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_5.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_5.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_5.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv_4 = _hf_hashv_4.wrapping_add(
                    (*_hj_key_5.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_5.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_5.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_5.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
                _hj_i_5 = _hj_i_5.wrapping_sub(_hf_hashv_4);
                _hj_i_5 ^= _hf_hashv_4 >> 13 as ::core::ffi::c_int;
                _hj_j_5 = _hj_j_5.wrapping_sub(_hf_hashv_4);
                _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
                _hj_j_5 ^= _hj_i_5 << 8 as ::core::ffi::c_int;
                _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_i_5);
                _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_j_5);
                _hf_hashv_4 ^= _hj_j_5 >> 13 as ::core::ffi::c_int;
                _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
                _hj_i_5 = _hj_i_5.wrapping_sub(_hf_hashv_4);
                _hj_i_5 ^= _hf_hashv_4 >> 12 as ::core::ffi::c_int;
                _hj_j_5 = _hj_j_5.wrapping_sub(_hf_hashv_4);
                _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
                _hj_j_5 ^= _hj_i_5 << 16 as ::core::ffi::c_int;
                _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_i_5);
                _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_j_5);
                _hf_hashv_4 ^= _hj_j_5 >> 5 as ::core::ffi::c_int;
                _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
                _hj_i_5 = _hj_i_5.wrapping_sub(_hf_hashv_4);
                _hj_i_5 ^= _hf_hashv_4 >> 3 as ::core::ffi::c_int;
                _hj_j_5 = _hj_j_5.wrapping_sub(_hf_hashv_4);
                _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
                _hj_j_5 ^= _hj_i_5 << 10 as ::core::ffi::c_int;
                _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_i_5);
                _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_j_5);
                _hf_hashv_4 ^= _hj_j_5 >> 15 as ::core::ffi::c_int;
                _hj_key_5 = _hj_key_5.offset(12 as ::core::ffi::c_int as isize);
                _hj_k_5 = _hj_k_5.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _hf_hashv_4 = _hf_hashv_4
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_874: u64;
            match _hj_k_5 {
                11 => {
                    _hf_hashv_4 = _hf_hashv_4.wrapping_add(
                        (*_hj_key_5.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_874 = 5467124032662967689;
                }
                10 => {
                    current_block_874 = 5467124032662967689;
                }
                9 => {
                    current_block_874 = 4540631061681808643;
                }
                8 => {
                    current_block_874 = 17956984801620020792;
                }
                7 => {
                    current_block_874 = 16060206028006833166;
                }
                6 => {
                    current_block_874 = 2669132161675588279;
                }
                5 => {
                    current_block_874 = 8886454637855882972;
                }
                4 => {
                    current_block_874 = 1939644305994994943;
                }
                3 => {
                    current_block_874 = 1185040227185769797;
                }
                2 => {
                    current_block_874 = 12588062962663961950;
                }
                1 => {
                    current_block_874 = 3853437452440905620;
                }
                _ => {
                    current_block_874 = 13479751590013074546;
                }
            }
            match current_block_874 {
                5467124032662967689 => {
                    _hf_hashv_4 = _hf_hashv_4.wrapping_add(
                        (*_hj_key_5.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_874 = 4540631061681808643;
                }
                _ => {}
            }
            match current_block_874 {
                4540631061681808643 => {
                    _hf_hashv_4 = _hf_hashv_4.wrapping_add(
                        (*_hj_key_5.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_874 = 17956984801620020792;
                }
                _ => {}
            }
            match current_block_874 {
                17956984801620020792 => {
                    _hj_j_5 = _hj_j_5.wrapping_add(
                        (*_hj_key_5.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_874 = 16060206028006833166;
                }
                _ => {}
            }
            match current_block_874 {
                16060206028006833166 => {
                    _hj_j_5 = _hj_j_5.wrapping_add(
                        (*_hj_key_5.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_874 = 2669132161675588279;
                }
                _ => {}
            }
            match current_block_874 {
                2669132161675588279 => {
                    _hj_j_5 = _hj_j_5.wrapping_add(
                        (*_hj_key_5.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_874 = 8886454637855882972;
                }
                _ => {}
            }
            match current_block_874 {
                8886454637855882972 => {
                    _hj_j_5 = _hj_j_5
                        .wrapping_add(*_hj_key_5.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_874 = 1939644305994994943;
                }
                _ => {}
            }
            match current_block_874 {
                1939644305994994943 => {
                    _hj_i_5 = _hj_i_5.wrapping_add(
                        (*_hj_key_5.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_874 = 1185040227185769797;
                }
                _ => {}
            }
            match current_block_874 {
                1185040227185769797 => {
                    _hj_i_5 = _hj_i_5.wrapping_add(
                        (*_hj_key_5.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_874 = 12588062962663961950;
                }
                _ => {}
            }
            match current_block_874 {
                12588062962663961950 => {
                    _hj_i_5 = _hj_i_5.wrapping_add(
                        (*_hj_key_5.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_874 = 3853437452440905620;
                }
                _ => {}
            }
            match current_block_874 {
                3853437452440905620 => {
                    _hj_i_5 = _hj_i_5
                        .wrapping_add(*_hj_key_5.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                }
                _ => {}
            }
            _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
            _hj_i_5 = _hj_i_5.wrapping_sub(_hf_hashv_4);
            _hj_i_5 ^= _hf_hashv_4 >> 13 as ::core::ffi::c_int;
            _hj_j_5 = _hj_j_5.wrapping_sub(_hf_hashv_4);
            _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
            _hj_j_5 ^= _hj_i_5 << 8 as ::core::ffi::c_int;
            _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_i_5);
            _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_j_5);
            _hf_hashv_4 ^= _hj_j_5 >> 13 as ::core::ffi::c_int;
            _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
            _hj_i_5 = _hj_i_5.wrapping_sub(_hf_hashv_4);
            _hj_i_5 ^= _hf_hashv_4 >> 12 as ::core::ffi::c_int;
            _hj_j_5 = _hj_j_5.wrapping_sub(_hf_hashv_4);
            _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
            _hj_j_5 ^= _hj_i_5 << 16 as ::core::ffi::c_int;
            _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_i_5);
            _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_j_5);
            _hf_hashv_4 ^= _hj_j_5 >> 5 as ::core::ffi::c_int;
            _hj_i_5 = _hj_i_5.wrapping_sub(_hj_j_5);
            _hj_i_5 = _hj_i_5.wrapping_sub(_hf_hashv_4);
            _hj_i_5 ^= _hf_hashv_4 >> 3 as ::core::ffi::c_int;
            _hj_j_5 = _hj_j_5.wrapping_sub(_hf_hashv_4);
            _hj_j_5 = _hj_j_5.wrapping_sub(_hj_i_5);
            _hj_j_5 ^= _hj_i_5 << 10 as ::core::ffi::c_int;
            _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_i_5);
            _hf_hashv_4 = _hf_hashv_4.wrapping_sub(_hj_j_5);
            _hf_hashv_4 ^= _hj_j_5 >> 15 as ::core::ffi::c_int;
            s_0 = ::core::ptr::null_mut::<ClassifierHash>();
            if !(*h).is_null() {
                let mut _hf_bkt_4: ::core::ffi::c_uint = 0;
                _hf_bkt_4 = _hf_hashv_4
                    & (*(**h).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt_4 as isize))
                        .hh_head
                        .is_null()
                    {
                        s_0 = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt_4 as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ClassifierHash
                            as *mut ClassifierHash;
                    } else {
                        s_0 = ::core::ptr::null_mut::<ClassifierHash>();
                    }
                    while !s_0.is_null() {
                        if (*s_0).hh.hashv == _hf_hashv_4
                            && (*s_0).hh.keylen as usize
                                == ::core::mem::size_of::<::core::ffi::c_int>()
                        {
                            if memcmp(
                                (*s_0).hh.key,
                                &raw mut gid_4 as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*s_0).hh.hh_next.is_null() {
                            s_0 = ((*s_0).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut ClassifierHash
                                as *mut ClassifierHash;
                        } else {
                            s_0 = ::core::ptr::null_mut::<ClassifierHash>();
                        }
                    }
                }
            }
            if s_0.is_null() {
                s_0 = __caryll_allocate_clean(
                    ::core::mem::size_of::<ClassifierHash>() as usize,
                    74 as ::core::ffi::c_ulong,
                ) as *mut ClassifierHash;
                (*s_0).gid = (*(*cov).glyphs.offset(j_2 as isize)).index as ::core::ffi::c_int;
                (*s_0).gname = (*(*cov).glyphs.offset(j_2 as isize)).name;
                (*s_0).cls = *past + 1 as ::core::ffi::c_int;
                let mut _ha_hashv_0: ::core::ffi::c_uint = 0;
                let mut _hj_i_6: ::core::ffi::c_uint = 0;
                let mut _hj_j_6: ::core::ffi::c_uint = 0;
                let mut _hj_k_6: ::core::ffi::c_uint = 0;
                let mut _hj_key_6: *const ::core::ffi::c_uchar =
                    &raw mut (*s_0).gid as *const ::core::ffi::c_uchar;
                _ha_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_6 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_6 = _hj_j_6;
                _hj_k_6 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                while _hj_k_6 >= 12 as ::core::ffi::c_uint {
                    _hj_i_6 = _hj_i_6.wrapping_add(
                        (*_hj_key_6.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_6.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_6.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_6.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_j_6 = _hj_j_6.wrapping_add(
                        (*_hj_key_6.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_6.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_6.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_6.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                        (*_hj_key_6.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_6.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_6.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_6.offset(11 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
                    _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_0);
                    _hj_i_6 ^= _ha_hashv_0 >> 13 as ::core::ffi::c_int;
                    _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_0);
                    _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
                    _hj_j_6 ^= _hj_i_6 << 8 as ::core::ffi::c_int;
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_6);
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_6);
                    _ha_hashv_0 ^= _hj_j_6 >> 13 as ::core::ffi::c_int;
                    _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
                    _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_0);
                    _hj_i_6 ^= _ha_hashv_0 >> 12 as ::core::ffi::c_int;
                    _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_0);
                    _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
                    _hj_j_6 ^= _hj_i_6 << 16 as ::core::ffi::c_int;
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_6);
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_6);
                    _ha_hashv_0 ^= _hj_j_6 >> 5 as ::core::ffi::c_int;
                    _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
                    _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_0);
                    _hj_i_6 ^= _ha_hashv_0 >> 3 as ::core::ffi::c_int;
                    _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_0);
                    _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
                    _hj_j_6 ^= _hj_i_6 << 10 as ::core::ffi::c_int;
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_6);
                    _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_6);
                    _ha_hashv_0 ^= _hj_j_6 >> 15 as ::core::ffi::c_int;
                    _hj_key_6 = _hj_key_6.offset(12 as ::core::ffi::c_int as isize);
                    _hj_k_6 = _hj_k_6.wrapping_sub(12 as ::core::ffi::c_uint);
                }
                _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint,
                );
                let mut current_block_992: u64;
                match _hj_k_6 {
                    11 => {
                        _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                            (*_hj_key_6.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_992 = 12336535586471661431;
                    }
                    10 => {
                        current_block_992 = 12336535586471661431;
                    }
                    9 => {
                        current_block_992 = 2082904494386641978;
                    }
                    8 => {
                        current_block_992 = 16142662908955679255;
                    }
                    7 => {
                        current_block_992 = 9984103270937231169;
                    }
                    6 => {
                        current_block_992 = 17982760481556857108;
                    }
                    5 => {
                        current_block_992 = 15172961618074657874;
                    }
                    4 => {
                        current_block_992 = 5486751686763416122;
                    }
                    3 => {
                        current_block_992 = 9667549555488425946;
                    }
                    2 => {
                        current_block_992 = 3550651938674953058;
                    }
                    1 => {
                        current_block_992 = 3368139742351851129;
                    }
                    _ => {
                        current_block_992 = 9478888234706187765;
                    }
                }
                match current_block_992 {
                    12336535586471661431 => {
                        _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                            (*_hj_key_6.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_992 = 2082904494386641978;
                    }
                    _ => {}
                }
                match current_block_992 {
                    2082904494386641978 => {
                        _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                            (*_hj_key_6.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_992 = 16142662908955679255;
                    }
                    _ => {}
                }
                match current_block_992 {
                    16142662908955679255 => {
                        _hj_j_6 = _hj_j_6.wrapping_add(
                            (*_hj_key_6.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_992 = 9984103270937231169;
                    }
                    _ => {}
                }
                match current_block_992 {
                    9984103270937231169 => {
                        _hj_j_6 = _hj_j_6.wrapping_add(
                            (*_hj_key_6.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_992 = 17982760481556857108;
                    }
                    _ => {}
                }
                match current_block_992 {
                    17982760481556857108 => {
                        _hj_j_6 = _hj_j_6.wrapping_add(
                            (*_hj_key_6.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_992 = 15172961618074657874;
                    }
                    _ => {}
                }
                match current_block_992 {
                    15172961618074657874 => {
                        _hj_j_6 = _hj_j_6
                            .wrapping_add(*_hj_key_6.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_992 = 5486751686763416122;
                    }
                    _ => {}
                }
                match current_block_992 {
                    5486751686763416122 => {
                        _hj_i_6 = _hj_i_6.wrapping_add(
                            (*_hj_key_6.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_992 = 9667549555488425946;
                    }
                    _ => {}
                }
                match current_block_992 {
                    9667549555488425946 => {
                        _hj_i_6 = _hj_i_6.wrapping_add(
                            (*_hj_key_6.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_992 = 3550651938674953058;
                    }
                    _ => {}
                }
                match current_block_992 {
                    3550651938674953058 => {
                        _hj_i_6 = _hj_i_6.wrapping_add(
                            (*_hj_key_6.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_992 = 3368139742351851129;
                    }
                    _ => {}
                }
                match current_block_992 {
                    3368139742351851129 => {
                        _hj_i_6 = _hj_i_6
                            .wrapping_add(*_hj_key_6.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
                _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_0);
                _hj_i_6 ^= _ha_hashv_0 >> 13 as ::core::ffi::c_int;
                _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_0);
                _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
                _hj_j_6 ^= _hj_i_6 << 8 as ::core::ffi::c_int;
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_6);
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_6);
                _ha_hashv_0 ^= _hj_j_6 >> 13 as ::core::ffi::c_int;
                _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
                _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_0);
                _hj_i_6 ^= _ha_hashv_0 >> 12 as ::core::ffi::c_int;
                _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_0);
                _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
                _hj_j_6 ^= _hj_i_6 << 16 as ::core::ffi::c_int;
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_6);
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_6);
                _ha_hashv_0 ^= _hj_j_6 >> 5 as ::core::ffi::c_int;
                _hj_i_6 = _hj_i_6.wrapping_sub(_hj_j_6);
                _hj_i_6 = _hj_i_6.wrapping_sub(_ha_hashv_0);
                _hj_i_6 ^= _ha_hashv_0 >> 3 as ::core::ffi::c_int;
                _hj_j_6 = _hj_j_6.wrapping_sub(_ha_hashv_0);
                _hj_j_6 = _hj_j_6.wrapping_sub(_hj_i_6);
                _hj_j_6 ^= _hj_i_6 << 10 as ::core::ffi::c_int;
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_6);
                _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_6);
                _ha_hashv_0 ^= _hj_j_6 >> 15 as ::core::ffi::c_int;
                (*s_0).hh.hashv = _ha_hashv_0;
                (*s_0).hh.key =
                    &raw mut (*s_0).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
                (*s_0).hh.keylen =
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                if (*h).is_null() {
                    (*s_0).hh.next = NULL;
                    (*s_0).hh.prev = NULL;
                    (*s_0).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                        as *mut UtHashTable
                        as *mut UtHashTable;
                    if (*s_0).hh.tbl.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*s_0).hh.tbl as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            ::core::mem::size_of::<UtHashTable>() as usize,
                        );
                        (*(*s_0).hh.tbl).tail = &raw mut (*s_0).hh as *mut UtHashHandle;
                        (*(*s_0).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                        (*(*s_0).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                        (*(*s_0).hh.tbl).hho = (&raw mut (*s_0).hh as *mut ::core::ffi::c_char)
                            .offset_from(s_0 as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long
                            as isize;
                        (*(*s_0).hh.tbl).buckets = malloc(
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        ) as *mut UtHashBucket;
                        (*(*s_0).hh.tbl).signature = HASH_SIGNATURE as u32;
                        if (*(*s_0).hh.tbl).buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*(*s_0).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (32 as usize).wrapping_mul(
                                    ::core::mem::size_of::<UtHashBucket>() as usize,
                                ),
                            );
                        }
                    }
                    *h = s_0;
                } else {
                    (*s_0).hh.tbl = (**h).hh.tbl;
                    (*s_0).hh.next = NULL;
                    (*s_0).hh.prev = ((*(**h).hh.tbl).tail as *mut ::core::ffi::c_char)
                        .offset(-(*(**h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void;
                    (*(*(**h).hh.tbl).tail).next = s_0 as *mut ::core::ffi::c_void;
                    (*(**h).hh.tbl).tail = &raw mut (*s_0).hh as *mut UtHashHandle;
                }
                let mut _ha_bkt_0: ::core::ffi::c_uint = 0;
                (*(**h).hh.tbl).num_items = (*(**h).hh.tbl).num_items.wrapping_add(1);
                _ha_bkt_0 = _ha_hashv_0
                    & (*(**h).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head_0: *mut UtHashBucket =
                    (*(**h).hh.tbl).buckets.offset(_ha_bkt_0 as isize) as *mut UtHashBucket;
                (*_ha_head_0).count = (*_ha_head_0).count.wrapping_add(1);
                (*s_0).hh.hh_next = (*_ha_head_0).hh_head as *mut UtHashHandle;
                (*s_0).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                if !(*_ha_head_0).hh_head.is_null() {
                    (*(*_ha_head_0).hh_head).hh_prev = &raw mut (*s_0).hh as *mut UtHashHandle;
                }
                (*_ha_head_0).hh_head = &raw mut (*s_0).hh as *mut UtHashHandle;
                if (*_ha_head_0).count
                    >= (*_ha_head_0)
                        .expand_mult
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                    && (*(*s_0).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt_0: ::core::ffi::c_uint = 0;
                    let mut _he_bkt_i_0: ::core::ffi::c_uint = 0;
                    let mut _he_thh_0: *mut UtHashHandle =
                        ::core::ptr::null_mut::<UtHashHandle>();
                    let mut _he_hh_nxt_0: *mut UtHashHandle =
                        ::core::ptr::null_mut::<UtHashHandle>();
                    let mut _he_new_buckets_0: *mut UtHashBucket =
                        ::core::ptr::null_mut::<UtHashBucket>();
                    let mut _he_newbkt_0: *mut UtHashBucket =
                        ::core::ptr::null_mut::<UtHashBucket>();
                    _he_new_buckets_0 = malloc(
                        (2 as usize)
                            .wrapping_mul((*(*s_0).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    ) as *mut UtHashBucket;
                    if _he_new_buckets_0.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            _he_new_buckets_0 as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (2 as usize)
                                .wrapping_mul((*(*s_0).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        );
                        (*(*s_0).hh.tbl).ideal_chain_maxlen = ((*(*s_0).hh.tbl).num_items
                            >> (*(*s_0).hh.tbl)
                                .log2_num_buckets
                                .wrapping_add(1 as ::core::ffi::c_uint))
                        .wrapping_add(
                            if (*(*s_0).hh.tbl).num_items
                                & (*(*s_0).hh.tbl)
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
                        (*(*s_0).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                        _he_bkt_i_0 = 0 as ::core::ffi::c_uint;
                        while _he_bkt_i_0 < (*(*s_0).hh.tbl).num_buckets {
                            _he_thh_0 = (*(*(*s_0).hh.tbl).buckets.offset(_he_bkt_i_0 as isize))
                                .hh_head
                                as *mut UtHashHandle;
                            while !_he_thh_0.is_null() {
                                _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                                _he_bkt_0 = (*_he_thh_0).hashv
                                    & (*(*s_0).hh.tbl)
                                        .num_buckets
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint);
                                _he_newbkt_0 = _he_new_buckets_0.offset(_he_bkt_0 as isize)
                                    as *mut UtHashBucket;
                                (*_he_newbkt_0).count = (*_he_newbkt_0).count.wrapping_add(1);
                                if (*_he_newbkt_0).count > (*(*s_0).hh.tbl).ideal_chain_maxlen {
                                    (*(*s_0).hh.tbl).nonideal_items =
                                        (*(*s_0).hh.tbl).nonideal_items.wrapping_add(1);
                                    (*_he_newbkt_0).expand_mult = (*_he_newbkt_0)
                                        .count
                                        .wrapping_div((*(*s_0).hh.tbl).ideal_chain_maxlen);
                                }
                                (*_he_thh_0).hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                                (*_he_thh_0).hh_next =
                                    (*_he_newbkt_0).hh_head as *mut UtHashHandle;
                                if !(*_he_newbkt_0).hh_head.is_null() {
                                    (*(*_he_newbkt_0).hh_head).hh_prev = _he_thh_0;
                                }
                                (*_he_newbkt_0).hh_head = _he_thh_0 as *mut UtHashHandle;
                                _he_thh_0 = _he_hh_nxt_0;
                            }
                            _he_bkt_i_0 = _he_bkt_i_0.wrapping_add(1);
                        }
                        free((*(*s_0).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        (*(*s_0).hh.tbl).num_buckets = (*(*s_0).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint);
                        (*(*s_0).hh.tbl).log2_num_buckets =
                            (*(*s_0).hh.tbl).log2_num_buckets.wrapping_add(1);
                        (*(*s_0).hh.tbl).buckets = _he_new_buckets_0;
                        (*(*s_0).hh.tbl).ineff_expands = if (*(*s_0).hh.tbl).nonideal_items
                            > (*(*s_0).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                        {
                            (*(*s_0).hh.tbl)
                                .ineff_expands
                                .wrapping_add(1 as ::core::ffi::c_uint)
                        } else {
                            0 as ::core::ffi::c_uint
                        };
                        if (*(*s_0).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                            (*(*s_0).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                        }
                    }
                }
            }
            j_2 = j_2.wrapping_add(1);
        }
        *past += 1 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn build_rule(
    mut rule: *mut ChainingRule,
    mut hb: *mut ClassifierHash,
    mut hi: *mut ClassifierHash,
    mut hf: *mut ClassifierHash,
) -> *mut ChainingRule {
    let mut new_rule: *mut ChainingRule = ::core::ptr::null_mut::<ChainingRule>();
    new_rule = __caryll_allocate_clean(
        ::core::mem::size_of::<ChainingRule>() as usize,
        88 as ::core::ffi::c_ulong,
    ) as *mut ChainingRule;
    (*new_rule).match_count = (*rule).match_count;
    (*new_rule).input_begins = (*rule).input_begins;
    (*new_rule).input_ends = (*rule).input_ends;
    (*new_rule).match_0 = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut Coverage>() as usize)
            .wrapping_mul((*new_rule).match_count as usize),
        92 as ::core::ffi::c_ulong,
    ) as *mut *mut Coverage;
    let mut m: TableId = 0 as TableId;
    while (m as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
        let ref mut fresh9 = *(*new_rule).match_0.offset(m as isize);
        *fresh9 = __caryll_allocate_clean(
            ::core::mem::size_of::<Coverage>() as usize,
            94 as ::core::ffi::c_ulong,
        ) as *mut Coverage;
        (**(*new_rule).match_0.offset(m as isize)).num_glyphs = 1 as GlyphId;
        let ref mut fresh10 = (**(*new_rule).match_0.offset(m as isize)).glyphs;
        *fresh10 = __caryll_allocate_clean(
            ::core::mem::size_of::<GlyphHandle>() as usize,
            96 as ::core::ffi::c_ulong,
        ) as *mut GlyphHandle;
        if (**(*rule).match_0.offset(m as isize)).num_glyphs as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            let mut h: *mut ClassifierHash =
                if (m as ::core::ffi::c_int) < (*rule).input_begins as ::core::ffi::c_int {
                    hb
                } else if (m as ::core::ffi::c_int) < (*rule).input_ends as ::core::ffi::c_int {
                    hi
                } else {
                    hf
                };
            let mut s: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
            let mut gid: ::core::ffi::c_int = (*(**(*rule).match_0.offset(m as isize))
                .glyphs
                .offset(0 as ::core::ffi::c_int as isize))
            .index as ::core::ffi::c_int;
            let mut _hf_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i: ::core::ffi::c_uint = 0;
            let mut _hj_j: ::core::ffi::c_uint = 0;
            let mut _hj_k: ::core::ffi::c_uint = 0;
            let mut _hj_key: *const ::core::ffi::c_uchar =
                &raw mut gid as *const ::core::ffi::c_uchar;
            _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i = _hj_j;
            _hj_k = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            while _hj_k >= 12 as ::core::ffi::c_uint {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
            _hf_hashv = _hf_hashv
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_58: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_58 = 5346948480966423038;
                }
                10 => {
                    current_block_58 = 5346948480966423038;
                }
                9 => {
                    current_block_58 = 7366647099252788322;
                }
                8 => {
                    current_block_58 = 9060342226317981968;
                }
                7 => {
                    current_block_58 = 15044609465002249597;
                }
                6 => {
                    current_block_58 = 10463626729690625243;
                }
                5 => {
                    current_block_58 = 16635126955682556087;
                }
                4 => {
                    current_block_58 = 6593776997697360244;
                }
                3 => {
                    current_block_58 = 16816136316488185196;
                }
                2 => {
                    current_block_58 = 16529377375228795976;
                }
                1 => {
                    current_block_58 = 8361605951270903918;
                }
                _ => {
                    current_block_58 = 13678349939556791712;
                }
            }
            match current_block_58 {
                5346948480966423038 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_58 = 7366647099252788322;
                }
                _ => {}
            }
            match current_block_58 {
                7366647099252788322 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_58 = 9060342226317981968;
                }
                _ => {}
            }
            match current_block_58 {
                9060342226317981968 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_58 = 15044609465002249597;
                }
                _ => {}
            }
            match current_block_58 {
                15044609465002249597 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_58 = 10463626729690625243;
                }
                _ => {}
            }
            match current_block_58 {
                10463626729690625243 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_58 = 16635126955682556087;
                }
                _ => {}
            }
            match current_block_58 {
                16635126955682556087 => {
                    _hj_j = _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_58 = 6593776997697360244;
                }
                _ => {}
            }
            match current_block_58 {
                6593776997697360244 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_58 = 16816136316488185196;
                }
                _ => {}
            }
            match current_block_58 {
                16816136316488185196 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_58 = 16529377375228795976;
                }
                _ => {}
            }
            match current_block_58 {
                16529377375228795976 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_58 = 8361605951270903918;
                }
                _ => {}
            }
            match current_block_58 {
                8361605951270903918 => {
                    _hj_i = _hj_i
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
            s = ::core::ptr::null_mut::<ClassifierHash>();
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
                        s = ((*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ClassifierHash
                            as *mut ClassifierHash;
                    } else {
                        s = ::core::ptr::null_mut::<ClassifierHash>();
                    }
                    while !s.is_null() {
                        if (*s).hh.hashv == _hf_hashv
                            && (*s).hh.keylen as usize
                                == ::core::mem::size_of::<::core::ffi::c_int>()
                        {
                            if memcmp(
                                (*s).hh.key,
                                &raw mut gid as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*s).hh.hh_next.is_null() {
                            s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(*h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut ClassifierHash
                                as *mut ClassifierHash;
                        } else {
                            s = ::core::ptr::null_mut::<ClassifierHash>();
                        }
                    }
                }
            }
            *(**(*new_rule).match_0.offset(m as isize))
                .glyphs
                .offset(0 as ::core::ffi::c_int as isize) =
                handle_from_index((*s).cls as GlyphId)
                    as GlyphHandle;
        } else {
            *(**(*new_rule).match_0.offset(m as isize))
                .glyphs
                .offset(0 as ::core::ffi::c_int as isize) =
                handle_from_index(0 as GlyphId)
                    as GlyphHandle;
        }
        m = m.wrapping_add(1);
    }
    (*new_rule).apply_count = (*rule).apply_count;
    (*new_rule).apply = __caryll_allocate_clean(
        (::core::mem::size_of::<ChainLookupApplication>() as usize)
            .wrapping_mul((*new_rule).apply_count as usize),
        108 as ::core::ffi::c_ulong,
    ) as *mut ChainLookupApplication;
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*rule).apply_count as ::core::ffi::c_int {
        (*(*new_rule).apply.offset(j as isize)).index = (*(*rule).apply.offset(j as isize)).index;
        (*(*new_rule).apply.offset(j as isize)).lookup =
            otfcc_handle_dup(
                (*(*rule).apply.offset(j as isize)).lookup as Handle,
            ) as LookupHandle;
        j = j.wrapping_add(1);
    }
    return new_rule;
}
unsafe extern "C" fn to_class(mut h: *mut *mut ClassifierHash) -> *mut ClassDef {
    let mut cd: *mut ClassDef = otl_class_def_create();
    let mut item: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
    let mut _hs_i: ::core::ffi::c_uint = 0;
    let mut _hs_looping: ::core::ffi::c_uint = 0;
    let mut _hs_nmerges: ::core::ffi::c_uint = 0;
    let mut _hs_insize: ::core::ffi::c_uint = 0;
    let mut _hs_psize: ::core::ffi::c_uint = 0;
    let mut _hs_qsize: ::core::ffi::c_uint = 0;
    let mut _hs_p: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_q: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_e: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_list: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_tail: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    if !(*h).is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (**h).hh as *mut UtHashHandle;
        while _hs_looping != 0 as ::core::ffi::c_uint {
            _hs_p = _hs_list;
            _hs_list = ::core::ptr::null_mut::<UtHashHandle>();
            _hs_tail = ::core::ptr::null_mut::<UtHashHandle>();
            _hs_nmerges = 0 as ::core::ffi::c_uint;
            while !_hs_p.is_null() {
                _hs_nmerges = _hs_nmerges.wrapping_add(1);
                _hs_q = _hs_p;
                _hs_psize = 0 as ::core::ffi::c_uint;
                _hs_i = 0 as ::core::ffi::c_uint;
                while _hs_i < _hs_insize {
                    _hs_psize = _hs_psize.wrapping_add(1);
                    _hs_q = (if !(*_hs_q).next.is_null() {
                        ((*_hs_q).next as *mut ::core::ffi::c_char)
                            .offset((*(**h).hh.tbl).hho)
                            as *mut UtHashHandle
                    } else {
                        ::core::ptr::null_mut::<UtHashHandle>()
                    }) as *mut UtHashHandle;
                    if _hs_q.is_null() {
                        break;
                    }
                    _hs_i = _hs_i.wrapping_add(1);
                }
                _hs_qsize = _hs_insize;
                while _hs_psize != 0 as ::core::ffi::c_uint
                    || _hs_qsize != 0 as ::core::ffi::c_uint && !_hs_q.is_null()
                {
                    if _hs_psize == 0 as ::core::ffi::c_uint {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(**h).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    } else if _hs_qsize == 0 as ::core::ffi::c_uint || _hs_q.is_null() {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(**h).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if by_gid_clsh(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ClassifierHash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ClassifierHash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(**h).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(**h).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    } else {
                        _hs_list = _hs_e;
                    }
                    if !_hs_e.is_null() {
                        (*_hs_e).prev = if !_hs_tail.is_null() {
                            (_hs_tail as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    }
                    _hs_tail = _hs_e;
                }
                _hs_p = _hs_q;
            }
            if !_hs_tail.is_null() {
                (*_hs_tail).next = NULL;
            }
            if _hs_nmerges <= 1 as ::core::ffi::c_uint {
                _hs_looping = 0 as ::core::ffi::c_uint;
                (*(**h).hh.tbl).tail = _hs_tail;
                *h = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut ClassifierHash
                    as *mut ClassifierHash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    item = *h;
    while !item.is_null() {
        push_class_def(
            cd,
            handle_from_consolidated(
                (*item).gid as GlyphId, (*item).gname
            ) as GlyphHandle,
            (*item).cls as GlyphClass,
        );
        item = (*item).hh.next as *mut ClassifierHash;
    }
    return cd;
}
pub unsafe extern "C" fn try_classify_around(
    mut lookup: *const Lookup,
    mut j: TableId,
    mut classified_st: *mut *mut ChainingSubtable,
) -> TableId {
    let mut current_block: u64;
    let mut compatible_count: TableId = 0 as TableId;
    let mut hb: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
    let mut hi: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
    let mut hf: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
    let mut subtable0: *mut ChainingSubtable =
        &raw mut (**(*lookup).subtables.items.offset(j as isize)).chaining;
    let mut classno_b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut classno_i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut classno_f: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rule0: *mut ChainingRule = &raw mut (*subtable0).c2rust_unnamed.rule;
    let mut m: TableId = 0 as TableId;
    loop {
        if !((m as ::core::ffi::c_int) < (*rule0).match_count as ::core::ffi::c_int) {
            current_block = 12349973810996921269;
            break;
        }
        let mut check: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (m as ::core::ffi::c_int) < (*rule0).input_begins as ::core::ffi::c_int {
            check = class_compatible(
                &raw mut hb,
                *(*rule0).match_0.offset(m as isize),
                &raw mut classno_b,
            );
        } else if (m as ::core::ffi::c_int) < (*rule0).input_ends as ::core::ffi::c_int {
            check = class_compatible(
                &raw mut hi,
                *(*rule0).match_0.offset(m as isize),
                &raw mut classno_i,
            );
        } else {
            check = class_compatible(
                &raw mut hf,
                *(*rule0).match_0.offset(m as isize),
                &raw mut classno_f,
            );
        }
        if check == 0 {
            current_block = 1622411330066726685;
            break;
        }
        m = m.wrapping_add(1);
    }
    match current_block {
        12349973810996921269 => {
            let mut k: TableId = (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
            's_74: while (k as usize) < (*lookup).subtables.length {
                let mut rule: *mut ChainingRule =
                    &raw mut (**(*lookup).subtables.items.offset(k as isize))
                        .chaining
                        .c2rust_unnamed
                        .rule;
                let mut allcheck: bool = true;
                let mut m_0: TableId = 0 as TableId;
                while (m_0 as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
                    let mut check_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if (m_0 as ::core::ffi::c_int) < (*rule).input_begins as ::core::ffi::c_int {
                        check_0 = class_compatible(
                            &raw mut hb,
                            *(*rule).match_0.offset(m_0 as isize),
                            &raw mut classno_b,
                        );
                    } else if (m_0 as ::core::ffi::c_int) < (*rule).input_ends as ::core::ffi::c_int
                    {
                        check_0 = class_compatible(
                            &raw mut hi,
                            *(*rule).match_0.offset(m_0 as isize),
                            &raw mut classno_i,
                        );
                    } else {
                        check_0 = class_compatible(
                            &raw mut hf,
                            *(*rule).match_0.offset(m_0 as isize),
                            &raw mut classno_f,
                        );
                    }
                    if check_0 == 0 {
                        allcheck = false;
                        break 's_74;
                    } else {
                        m_0 = m_0.wrapping_add(1);
                    }
                }
                if allcheck {
                    compatible_count = (compatible_count as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as TableId;
                }
                k = k.wrapping_add(1);
            }
            if compatible_count as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                subtable0 = __caryll_allocate_clean(
                    ::core::mem::size_of::<ChainingSubtable>() as usize,
                    170 as ::core::ffi::c_ulong,
                ) as *mut ChainingSubtable;
                (*subtable0).c2rust_unnamed.c2rust_unnamed.rules_count =
                    (compatible_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
                (*subtable0).c2rust_unnamed.c2rust_unnamed.rules = __caryll_allocate_clean(
                    (::core::mem::size_of::<*mut ChainingRule>() as usize).wrapping_mul(
                        (compatible_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
                    ),
                    172 as ::core::ffi::c_ulong,
                )
                    as *mut *mut ChainingRule;
                let ref mut fresh1 = *(*subtable0)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rules
                    .offset(0 as ::core::ffi::c_int as isize);
                *fresh1 = build_rule(rule0, hb, hi, hf);
                let mut kk: TableId = 1 as TableId;
                let mut k_0: TableId =
                    (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
                while (k_0 as usize) < (*lookup).subtables.length
                    && (kk as ::core::ffi::c_int)
                        < compatible_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                {
                    let mut rule_0: *mut ChainingRule =
                        &raw mut (**(*lookup).subtables.items.offset(k_0 as isize))
                            .chaining
                            .c2rust_unnamed
                            .rule;
                    let ref mut fresh2 = *(*subtable0)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .rules
                        .offset(kk as isize);
                    *fresh2 = build_rule(rule_0, hb, hi, hf);
                    kk = kk.wrapping_add(1);
                    k_0 = k_0.wrapping_add(1);
                }
                (*subtable0).type_0 = ChainingType::Classified;
                (*subtable0).c2rust_unnamed.c2rust_unnamed.bc = to_class(&raw mut hb);
                (*subtable0).c2rust_unnamed.c2rust_unnamed.ic = to_class(&raw mut hi);
                (*subtable0).c2rust_unnamed.c2rust_unnamed.fc = to_class(&raw mut hf);
                *classified_st = subtable0;
            }
        }
        _ => {}
    }
    if !hb.is_null() {
        let mut s: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
        let mut tmp: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
        s = hb;
        tmp = (if !hb.is_null() { (*hb).hh.next } else { NULL }) as *mut ClassifierHash
            as *mut ClassifierHash;
        while !s.is_null() {
            let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s).hh;
            if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                free((*(*hb).hh.tbl).buckets as *mut ::core::ffi::c_void);
                free((*hb).hh.tbl as *mut ::core::ffi::c_void);
                hb = ::core::ptr::null_mut::<ClassifierHash>();
            } else {
                let mut _hd_bkt: ::core::ffi::c_uint = 0;
                if _hd_hh_del == (*(*hb).hh.tbl).tail {
                    (*(*hb).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*hb).hh.tbl).hho)
                        as *mut UtHashHandle
                        as *mut UtHashHandle;
                }
                if !(*_hd_hh_del).prev.is_null() {
                    let ref mut fresh3 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*hb).hh.tbl).hho)
                        as *mut UtHashHandle))
                        .next;
                    *fresh3 = (*_hd_hh_del).next;
                } else {
                    hb = (*_hd_hh_del).next as *mut ClassifierHash as *mut ClassifierHash;
                }
                if !(*_hd_hh_del).next.is_null() {
                    let ref mut fresh4 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                        .offset((*(*hb).hh.tbl).hho)
                        as *mut UtHashHandle))
                        .prev;
                    *fresh4 = (*_hd_hh_del).prev;
                }
                _hd_bkt = (*_hd_hh_del).hashv
                    & (*(*hb).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _hd_head: *mut UtHashBucket =
                    (*(*hb).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
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
                (*(*hb).hh.tbl).num_items = (*(*hb).hh.tbl).num_items.wrapping_sub(1);
            }
            free(s as *mut ::core::ffi::c_void);
            s = ::core::ptr::null_mut::<ClassifierHash>();
            s = tmp;
            tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut ClassifierHash
                as *mut ClassifierHash;
        }
    }
    if !hi.is_null() {
        let mut s_0: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
        let mut tmp_0: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
        s_0 = hi;
        tmp_0 = (if !hi.is_null() { (*hi).hh.next } else { NULL }) as *mut ClassifierHash
            as *mut ClassifierHash;
        while !s_0.is_null() {
            let mut _hd_hh_del_0: *mut UtHashHandle = &raw mut (*s_0).hh;
            if (*_hd_hh_del_0).prev.is_null() && (*_hd_hh_del_0).next.is_null() {
                free((*(*hi).hh.tbl).buckets as *mut ::core::ffi::c_void);
                free((*hi).hh.tbl as *mut ::core::ffi::c_void);
                hi = ::core::ptr::null_mut::<ClassifierHash>();
            } else {
                let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                if _hd_hh_del_0 == (*(*hi).hh.tbl).tail {
                    (*(*hi).hh.tbl).tail = ((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                        .offset((*(*hi).hh.tbl).hho)
                        as *mut UtHashHandle
                        as *mut UtHashHandle;
                }
                if !(*_hd_hh_del_0).prev.is_null() {
                    let ref mut fresh5 = (*(((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                        .offset((*(*hi).hh.tbl).hho)
                        as *mut UtHashHandle))
                        .next;
                    *fresh5 = (*_hd_hh_del_0).next;
                } else {
                    hi = (*_hd_hh_del_0).next as *mut ClassifierHash as *mut ClassifierHash;
                }
                if !(*_hd_hh_del_0).next.is_null() {
                    let ref mut fresh6 = (*(((*_hd_hh_del_0).next as *mut ::core::ffi::c_char)
                        .offset((*(*hi).hh.tbl).hho)
                        as *mut UtHashHandle))
                        .prev;
                    *fresh6 = (*_hd_hh_del_0).prev;
                }
                _hd_bkt_0 = (*_hd_hh_del_0).hashv
                    & (*(*hi).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _hd_head_0: *mut UtHashBucket =
                    (*(*hi).hh.tbl).buckets.offset(_hd_bkt_0 as isize) as *mut UtHashBucket;
                (*_hd_head_0).count = (*_hd_head_0).count.wrapping_sub(1);
                if (*_hd_head_0).hh_head == _hd_hh_del_0 {
                    (*_hd_head_0).hh_head = (*_hd_hh_del_0).hh_next as *mut UtHashHandle;
                }
                if !(*_hd_hh_del_0).hh_prev.is_null() {
                    (*(*_hd_hh_del_0).hh_prev).hh_next = (*_hd_hh_del_0).hh_next;
                }
                if !(*_hd_hh_del_0).hh_next.is_null() {
                    (*(*_hd_hh_del_0).hh_next).hh_prev = (*_hd_hh_del_0).hh_prev;
                }
                (*(*hi).hh.tbl).num_items = (*(*hi).hh.tbl).num_items.wrapping_sub(1);
            }
            free(s_0 as *mut ::core::ffi::c_void);
            s_0 = ::core::ptr::null_mut::<ClassifierHash>();
            s_0 = tmp_0;
            tmp_0 = (if !tmp_0.is_null() {
                (*tmp_0).hh.next
            } else {
                NULL
            }) as *mut ClassifierHash as *mut ClassifierHash;
        }
    }
    if !hf.is_null() {
        let mut s_1: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
        let mut tmp_1: *mut ClassifierHash = ::core::ptr::null_mut::<ClassifierHash>();
        s_1 = hf;
        tmp_1 = (if !hf.is_null() { (*hf).hh.next } else { NULL }) as *mut ClassifierHash
            as *mut ClassifierHash;
        while !s_1.is_null() {
            let mut _hd_hh_del_1: *mut UtHashHandle = &raw mut (*s_1).hh;
            if (*_hd_hh_del_1).prev.is_null() && (*_hd_hh_del_1).next.is_null() {
                free((*(*hf).hh.tbl).buckets as *mut ::core::ffi::c_void);
                free((*hf).hh.tbl as *mut ::core::ffi::c_void);
                hf = ::core::ptr::null_mut::<ClassifierHash>();
            } else {
                let mut _hd_bkt_1: ::core::ffi::c_uint = 0;
                if _hd_hh_del_1 == (*(*hf).hh.tbl).tail {
                    (*(*hf).hh.tbl).tail = ((*_hd_hh_del_1).prev as *mut ::core::ffi::c_char)
                        .offset((*(*hf).hh.tbl).hho)
                        as *mut UtHashHandle
                        as *mut UtHashHandle;
                }
                if !(*_hd_hh_del_1).prev.is_null() {
                    let ref mut fresh7 = (*(((*_hd_hh_del_1).prev as *mut ::core::ffi::c_char)
                        .offset((*(*hf).hh.tbl).hho)
                        as *mut UtHashHandle))
                        .next;
                    *fresh7 = (*_hd_hh_del_1).next;
                } else {
                    hf = (*_hd_hh_del_1).next as *mut ClassifierHash as *mut ClassifierHash;
                }
                if !(*_hd_hh_del_1).next.is_null() {
                    let ref mut fresh8 = (*(((*_hd_hh_del_1).next as *mut ::core::ffi::c_char)
                        .offset((*(*hf).hh.tbl).hho)
                        as *mut UtHashHandle))
                        .prev;
                    *fresh8 = (*_hd_hh_del_1).prev;
                }
                _hd_bkt_1 = (*_hd_hh_del_1).hashv
                    & (*(*hf).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _hd_head_1: *mut UtHashBucket =
                    (*(*hf).hh.tbl).buckets.offset(_hd_bkt_1 as isize) as *mut UtHashBucket;
                (*_hd_head_1).count = (*_hd_head_1).count.wrapping_sub(1);
                if (*_hd_head_1).hh_head == _hd_hh_del_1 {
                    (*_hd_head_1).hh_head = (*_hd_hh_del_1).hh_next as *mut UtHashHandle;
                }
                if !(*_hd_hh_del_1).hh_prev.is_null() {
                    (*(*_hd_hh_del_1).hh_prev).hh_next = (*_hd_hh_del_1).hh_next;
                }
                if !(*_hd_hh_del_1).hh_next.is_null() {
                    (*(*_hd_hh_del_1).hh_next).hh_prev = (*_hd_hh_del_1).hh_prev;
                }
                (*(*hf).hh.tbl).num_items = (*(*hf).hh.tbl).num_items.wrapping_sub(1);
            }
            free(s_1 as *mut ::core::ffi::c_void);
            s_1 = ::core::ptr::null_mut::<ClassifierHash>();
            s_1 = tmp_1;
            tmp_1 = (if !tmp_1.is_null() {
                (*tmp_1).hh.next
            } else {
                NULL
            }) as *mut ClassifierHash as *mut ClassifierHash;
        }
    }
    if compatible_count as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        return compatible_count;
    } else {
        return 0 as TableId;
    };
}
pub unsafe extern "C" fn otfcc_classified_build_chaining(
    mut lookup: *const Lookup,
    mut subtable_buffers: *mut *mut *mut Buffer,
    mut last_offset: *mut usize,
) -> TableId {
    let mut is_contextual: bool = otfcc_chaining_lookup_is_contextual_lookup(lookup);
    let mut subtables_written: TableId = 0 as TableId;
    *subtable_buffers = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut Buffer>() as usize)
            .wrapping_mul((*lookup).subtables.length),
        223 as ::core::ffi::c_ulong,
    ) as *mut *mut Buffer;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*lookup).subtables.length {
        let mut st0: *mut ChainingSubtable =
            &raw mut (**(*lookup).subtables.items.offset(j as isize)).chaining;
        if !((*st0).type_0 as u64 != 0) {
            let mut st: *mut ChainingSubtable = st0;
            j = (j as ::core::ffi::c_int
                + try_classify_around(lookup, j, &raw mut st) as ::core::ffi::c_int)
                as TableId;
            let mut buf: *mut Buffer = if is_contextual as ::core::ffi::c_int != 0 {
                otfcc_build_contextual(st as *mut Subtable)
            } else {
                otfcc_build_chaining(st as *mut Subtable)
            };
            if st != st0 {
                I_SUBTABLE_CHAINING.free.expect("non-null function pointer")(st);
            }
            let ref mut fresh0 = *(*subtable_buffers).offset(subtables_written as isize);
            *fresh0 = buf;
            *last_offset = (*last_offset).wrapping_add((*buf).size);
            subtables_written =
                (subtables_written as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
        }
        j = j.wrapping_add(1);
    }
    return subtables_written;
}
