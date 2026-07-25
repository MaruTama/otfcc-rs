#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memcpy, memset, qsort};
unsafe extern "C" {
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_string_new(_: *const ::core::ffi::c_char) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn buflen(buf: *mut caryll_Buffer) -> usize;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite_bufdel(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
}

use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_dispose, otfcc_Handle, otfcc_GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::binio::{read_16u};
use crate::support::buffer::{caryll_Buffer};
use crate::support::primitives::{glyphid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_array, json_pre_serialized, json_string, json_value};
use crate::support::{NULL};
use crate::support::glyph_order::{glyph_handle};
use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_Coverage {
    pub numGlyphs: glyphid_t,
    pub capacity: u32,
    pub glyphs: *mut otfcc_GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __otfcc_ICoverage {
    pub init: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_Coverage, *const otl_Coverage) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_Coverage) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_Coverage>,
    pub free: Option<unsafe extern "C" fn(*mut otl_Coverage) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut otl_Coverage, u32) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_Coverage>,
    pub dump: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_Coverage>,
    pub build: Option<unsafe extern "C" fn(*const otl_Coverage) -> *mut caryll_Buffer>,
    pub buildFormat:
        Option<unsafe extern "C" fn(*const otl_Coverage, u16) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_Coverage, bool) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otl_Coverage, otfcc_GlyphHandle) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coverage_entry {
    pub gid: ::core::ffi::c_int,
    pub covIndex: ::core::ffi::c_int,
    pub hh: UT_hash_handle,
}
#[inline]
unsafe extern "C" fn disposeCoverage(mut coverage: *mut otl_Coverage) {
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*coverage).numGlyphs as ::core::ffi::c_int {
        otfcc_Handle_dispose(
            (*coverage).glyphs.offset(j as isize) as *mut otfcc_Handle,
        );
        j = j.wrapping_add(1);
    }
    free((*coverage).glyphs as *mut ::core::ffi::c_void);
    (*coverage).glyphs = ::core::ptr::null_mut::<otfcc_GlyphHandle>();
}
#[inline]
pub(crate) unsafe extern "C" fn otl_Coverage_move(mut dst: *mut otl_Coverage, mut src: *mut otl_Coverage) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_Coverage>() as usize,
    );
    otl_Coverage_init(src);
}
#[inline]
pub(crate) unsafe extern "C" fn otl_Coverage_copyReplace(mut dst: *mut otl_Coverage, src: otl_Coverage) {
    otl_Coverage_dispose(dst);
    otl_Coverage_copy(dst, &raw const src);
}
#[inline]
pub(crate) unsafe extern "C" fn otl_Coverage_create() -> *mut otl_Coverage {
    let mut x: *mut otl_Coverage =
        malloc(::core::mem::size_of::<otl_Coverage>() as usize) as *mut otl_Coverage;
    otl_Coverage_init(x);
    return x;
}
#[inline]
pub(crate) unsafe extern "C" fn otl_Coverage_free(mut x: *mut otl_Coverage) {
    if x.is_null() {
        return;
    }
    otl_Coverage_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
pub(crate) unsafe extern "C" fn otl_Coverage_dispose(mut x: *mut otl_Coverage) {
    disposeCoverage(x);
}
#[inline]
pub(crate) unsafe extern "C" fn otl_Coverage_copy(mut dst: *mut otl_Coverage, mut src: *const otl_Coverage) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_Coverage>() as usize,
    );
}
#[inline]
pub(crate) unsafe extern "C" fn otl_Coverage_init(mut x: *mut otl_Coverage) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<otl_Coverage>() as usize,
    );
}
#[inline]
pub(crate) unsafe extern "C" fn otl_Coverage_replace(mut dst: *mut otl_Coverage, src: otl_Coverage) {
    otl_Coverage_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_Coverage>() as usize,
    );
}
unsafe extern "C" fn growCoverage(mut coverage: *mut otl_Coverage, mut n: u32) {
    if n == 0 {
        return;
    }
    if n > (*coverage).capacity {
        if (*coverage).capacity == 0 {
            (*coverage).capacity = 0x10 as u32;
        }
        while n > (*coverage).capacity {
            (*coverage).capacity = (*coverage).capacity.wrapping_add(
                (*coverage).capacity >> 1 as ::core::ffi::c_int & 0xffffff as u32,
            );
        }
        (*coverage).glyphs = __caryll_reallocate(
            (*coverage).glyphs as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<otfcc_GlyphHandle>() as usize)
                .wrapping_mul((*coverage).capacity as usize),
            18 as ::core::ffi::c_ulong,
        ) as *mut otfcc_GlyphHandle;
    }
}
pub(crate) unsafe extern "C" fn clearCoverage(mut coverage: *mut otl_Coverage, mut n: u32) {
    if coverage.is_null() || (*coverage).glyphs.is_null() {
        return;
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*coverage).numGlyphs as ::core::ffi::c_int {
        otfcc_Handle_dispose(
            (*coverage).glyphs.offset(j as isize) as *mut otfcc_Handle,
        );
        j = j.wrapping_add(1);
    }
    growCoverage(coverage, n);
    (*coverage).numGlyphs = n as glyphid_t;
}
unsafe extern "C" fn by_covIndex(
    mut a: *mut coverage_entry,
    mut b: *mut coverage_entry,
) -> ::core::ffi::c_int {
    return (*a).covIndex - (*b).covIndex;
}
pub(crate) unsafe extern "C" fn pushToCoverage(mut coverage: *mut otl_Coverage, mut h: otfcc_GlyphHandle) {
    (*coverage).numGlyphs =
        ((*coverage).numGlyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
    growCoverage(coverage, (*coverage).numGlyphs as u32);
    *(*coverage)
        .glyphs
        .offset(((*coverage).numGlyphs as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
        h;
}
pub(crate) unsafe extern "C" fn readCoverage(
    mut data: *const u8,
    mut tableLength: u32,
    mut offset: u32,
) -> *mut otl_Coverage {
    let mut coverage: *mut otl_Coverage = otl_Coverage_create();
    if tableLength < offset.wrapping_add(4 as u32) {
        return coverage;
    }
    let mut format: u16 = read_16u(data.offset(offset as isize));
    match format as ::core::ffi::c_int {
        1 => {
            let mut glyphCount: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize),
            );
            if tableLength
                < offset.wrapping_add(4 as u32).wrapping_add(
                    (glyphCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                )
            {
                return coverage;
            }
            let mut hash: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
            let mut j: u16 = 0 as u16;
            while (j as ::core::ffi::c_int) < glyphCount as ::core::ffi::c_int {
                let mut item: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
                let mut gid: ::core::ffi::c_int = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize),
                ) as ::core::ffi::c_int;
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
                _hf_hashv = _hf_hashv.wrapping_add(
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint
                );
                let mut current_block_54: u64;
                match _hj_k {
                    11 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_54 = 11787676855009352887;
                    }
                    10 => {
                        current_block_54 = 11787676855009352887;
                    }
                    9 => {
                        current_block_54 = 9205717943706311272;
                    }
                    8 => {
                        current_block_54 = 1760194286177680343;
                    }
                    7 => {
                        current_block_54 = 459104737412379174;
                    }
                    6 => {
                        current_block_54 = 1017447862132263635;
                    }
                    5 => {
                        current_block_54 = 3812530106350735563;
                    }
                    4 => {
                        current_block_54 = 17144525467468587132;
                    }
                    3 => {
                        current_block_54 = 17951525393883168212;
                    }
                    2 => {
                        current_block_54 = 10420308214689385288;
                    }
                    1 => {
                        current_block_54 = 839679318605074425;
                    }
                    _ => {
                        current_block_54 = 6476622998065200121;
                    }
                }
                match current_block_54 {
                    11787676855009352887 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_54 = 9205717943706311272;
                    }
                    _ => {}
                }
                match current_block_54 {
                    9205717943706311272 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_54 = 1760194286177680343;
                    }
                    _ => {}
                }
                match current_block_54 {
                    1760194286177680343 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_54 = 459104737412379174;
                    }
                    _ => {}
                }
                match current_block_54 {
                    459104737412379174 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_54 = 1017447862132263635;
                    }
                    _ => {}
                }
                match current_block_54 {
                    1017447862132263635 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_54 = 3812530106350735563;
                    }
                    _ => {}
                }
                match current_block_54 {
                    3812530106350735563 => {
                        _hj_j = _hj_j
                            .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_54 = 17144525467468587132;
                    }
                    _ => {}
                }
                match current_block_54 {
                    17144525467468587132 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_54 = 17951525393883168212;
                    }
                    _ => {}
                }
                match current_block_54 {
                    17951525393883168212 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_54 = 10420308214689385288;
                    }
                    _ => {}
                }
                match current_block_54 {
                    10420308214689385288 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_54 = 839679318605074425;
                    }
                    _ => {}
                }
                match current_block_54 {
                    839679318605074425 => {
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
                item = ::core::ptr::null_mut::<coverage_entry>();
                if !hash.is_null() {
                    let mut _hf_bkt: ::core::ffi::c_uint = 0;
                    _hf_bkt = _hf_hashv
                        & (*(*hash).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if !(*(*(*hash).hh.tbl).buckets.offset(_hf_bkt as isize))
                            .hh_head
                            .is_null()
                        {
                            item = ((*(*(*hash).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(*hash).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut coverage_entry
                                as *mut coverage_entry;
                        } else {
                            item = ::core::ptr::null_mut::<coverage_entry>();
                        }
                        while !item.is_null() {
                            if (*item).hh.hashv == _hf_hashv
                                && (*item).hh.keylen as usize
                                    == ::core::mem::size_of::<::core::ffi::c_int>()
                            {
                                if memcmp(
                                    (*item).hh.key,
                                    &raw mut gid as *const ::core::ffi::c_void,
                                    ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if !(*item).hh.hh_next.is_null() {
                                item = ((*item).hh.hh_next as *mut ::core::ffi::c_char)
                                    .offset(-(*(*hash).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut coverage_entry
                                    as *mut coverage_entry;
                            } else {
                                item = ::core::ptr::null_mut::<coverage_entry>();
                            }
                        }
                    }
                }
                if item.is_null() {
                    item = __caryll_allocate_clean(
                        ::core::mem::size_of::<coverage_entry>() as usize,
                        60 as ::core::ffi::c_ulong,
                    ) as *mut coverage_entry;
                    (*item).gid = gid;
                    (*item).covIndex = j as ::core::ffi::c_int;
                    let mut _ha_hashv: ::core::ffi::c_uint = 0;
                    let mut _hj_i_0: ::core::ffi::c_uint = 0;
                    let mut _hj_j_0: ::core::ffi::c_uint = 0;
                    let mut _hj_k_0: ::core::ffi::c_uint = 0;
                    let mut _hj_key_0: *const ::core::ffi::c_uchar =
                        &raw mut (*item).gid as *const ::core::ffi::c_uchar;
                    _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                    _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                    _hj_i_0 = _hj_j_0;
                    _hj_k_0 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                    while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint,
                    );
                    let mut current_block_171: u64;
                    match _hj_k_0 {
                        11 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_171 = 10602302762135974562;
                        }
                        10 => {
                            current_block_171 = 10602302762135974562;
                        }
                        9 => {
                            current_block_171 = 6710005217062049649;
                        }
                        8 => {
                            current_block_171 = 10358134266333554474;
                        }
                        7 => {
                            current_block_171 = 15389940015085131153;
                        }
                        6 => {
                            current_block_171 = 7736769285171211731;
                        }
                        5 => {
                            current_block_171 = 1615522192696774067;
                        }
                        4 => {
                            current_block_171 = 8445812812378343230;
                        }
                        3 => {
                            current_block_171 = 16742436990799704850;
                        }
                        2 => {
                            current_block_171 = 5963426190170342884;
                        }
                        1 => {
                            current_block_171 = 14317173050247206952;
                        }
                        _ => {
                            current_block_171 = 15614898248724990345;
                        }
                    }
                    match current_block_171 {
                        10602302762135974562 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_171 = 6710005217062049649;
                        }
                        _ => {}
                    }
                    match current_block_171 {
                        6710005217062049649 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_171 = 10358134266333554474;
                        }
                        _ => {}
                    }
                    match current_block_171 {
                        10358134266333554474 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_171 = 15389940015085131153;
                        }
                        _ => {}
                    }
                    match current_block_171 {
                        15389940015085131153 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_171 = 7736769285171211731;
                        }
                        _ => {}
                    }
                    match current_block_171 {
                        7736769285171211731 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_171 = 1615522192696774067;
                        }
                        _ => {}
                    }
                    match current_block_171 {
                        1615522192696774067 => {
                            _hj_j_0 = _hj_j_0
                                .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_171 = 8445812812378343230;
                        }
                        _ => {}
                    }
                    match current_block_171 {
                        8445812812378343230 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_171 = 16742436990799704850;
                        }
                        _ => {}
                    }
                    match current_block_171 {
                        16742436990799704850 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_171 = 5963426190170342884;
                        }
                        _ => {}
                    }
                    match current_block_171 {
                        5963426190170342884 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_171 = 14317173050247206952;
                        }
                        _ => {}
                    }
                    match current_block_171 {
                        14317173050247206952 => {
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
                    (*item).hh.hashv = _ha_hashv;
                    (*item).hh.key = &raw mut (*item).gid as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void;
                    (*item).hh.keylen =
                        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                    if hash.is_null() {
                        (*item).hh.next = NULL;
                        (*item).hh.prev = NULL;
                        (*item).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                            as *mut UT_hash_table
                            as *mut UT_hash_table;
                        if (*item).hh.tbl.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*item).hh.tbl as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                ::core::mem::size_of::<UT_hash_table>() as usize,
                            );
                            (*(*item).hh.tbl).tail = &raw mut (*item).hh as *mut UT_hash_handle;
                            (*(*item).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                            (*(*item).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                            (*(*item).hh.tbl).hho = (&raw mut (*item).hh
                                as *mut ::core::ffi::c_char)
                                .offset_from(item as *mut ::core::ffi::c_char)
                                as ::core::ffi::c_long
                                as isize;
                            (*(*item).hh.tbl).buckets =
                                malloc((32 as usize).wrapping_mul(::core::mem::size_of::<
                                    UT_hash_bucket,
                                >(
                                )
                                    as usize))
                                    as *mut UT_hash_bucket;
                            (*(*item).hh.tbl).signature = HASH_SIGNATURE as u32;
                            if (*(*item).hh.tbl).buckets.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (32 as usize).wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as usize,
                                    ),
                                );
                            }
                        }
                        hash = item;
                    } else {
                        (*item).hh.tbl = (*hash).hh.tbl;
                        (*item).hh.next = NULL;
                        (*item).hh.prev = ((*(*hash).hh.tbl).tail as *mut ::core::ffi::c_char)
                            .offset(-(*(*hash).hh.tbl).hho)
                            as *mut ::core::ffi::c_void;
                        (*(*(*hash).hh.tbl).tail).next = item as *mut ::core::ffi::c_void;
                        (*(*hash).hh.tbl).tail = &raw mut (*item).hh as *mut UT_hash_handle;
                    }
                    let mut _ha_bkt: ::core::ffi::c_uint = 0;
                    (*(*hash).hh.tbl).num_items = (*(*hash).hh.tbl).num_items.wrapping_add(1);
                    _ha_bkt = _ha_hashv
                        & (*(*hash).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _ha_head: *mut UT_hash_bucket =
                        (*(*hash).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                    (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                    (*item).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
                    (*item).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                    if !(*_ha_head).hh_head.is_null() {
                        (*(*_ha_head).hh_head).hh_prev = &raw mut (*item).hh as *mut UT_hash_handle;
                    }
                    (*_ha_head).hh_head = &raw mut (*item).hh as *mut UT_hash_handle;
                    if (*_ha_head).count
                        >= (*_ha_head)
                            .expand_mult
                            .wrapping_add(1 as ::core::ffi::c_uint)
                            .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                        && (*(*item).hh.tbl).noexpand == 0
                    {
                        let mut _he_bkt: ::core::ffi::c_uint = 0;
                        let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                        let mut _he_thh: *mut UT_hash_handle =
                            ::core::ptr::null_mut::<UT_hash_handle>();
                        let mut _he_hh_nxt: *mut UT_hash_handle =
                            ::core::ptr::null_mut::<UT_hash_handle>();
                        let mut _he_new_buckets: *mut UT_hash_bucket =
                            ::core::ptr::null_mut::<UT_hash_bucket>();
                        let mut _he_newbkt: *mut UT_hash_bucket =
                            ::core::ptr::null_mut::<UT_hash_bucket>();
                        _he_new_buckets = malloc(
                            (2 as usize)
                                .wrapping_mul((*(*item).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        ) as *mut UT_hash_bucket;
                        if _he_new_buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                _he_new_buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (2 as usize)
                                    .wrapping_mul((*(*item).hh.tbl).num_buckets as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as usize
                                    ),
                            );
                            (*(*item).hh.tbl).ideal_chain_maxlen = ((*(*item).hh.tbl).num_items
                                >> (*(*item).hh.tbl)
                                    .log2_num_buckets
                                    .wrapping_add(1 as ::core::ffi::c_uint))
                            .wrapping_add(
                                if (*(*item).hh.tbl).num_items
                                    & (*(*item).hh.tbl)
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
                            (*(*item).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                            _he_bkt_i = 0 as ::core::ffi::c_uint;
                            while _he_bkt_i < (*(*item).hh.tbl).num_buckets {
                                _he_thh = (*(*(*item).hh.tbl).buckets.offset(_he_bkt_i as isize))
                                    .hh_head
                                    as *mut UT_hash_handle;
                                while !_he_thh.is_null() {
                                    _he_hh_nxt = (*_he_thh).hh_next;
                                    _he_bkt = (*_he_thh).hashv
                                        & (*(*item).hh.tbl)
                                            .num_buckets
                                            .wrapping_mul(2 as ::core::ffi::c_uint)
                                            .wrapping_sub(1 as ::core::ffi::c_uint);
                                    _he_newbkt = _he_new_buckets.offset(_he_bkt as isize)
                                        as *mut UT_hash_bucket;
                                    (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                    if (*_he_newbkt).count > (*(*item).hh.tbl).ideal_chain_maxlen {
                                        (*(*item).hh.tbl).nonideal_items =
                                            (*(*item).hh.tbl).nonideal_items.wrapping_add(1);
                                        (*_he_newbkt).expand_mult = (*_he_newbkt)
                                            .count
                                            .wrapping_div((*(*item).hh.tbl).ideal_chain_maxlen);
                                    }
                                    (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                                    (*_he_thh).hh_next =
                                        (*_he_newbkt).hh_head as *mut UT_hash_handle;
                                    if !(*_he_newbkt).hh_head.is_null() {
                                        (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                                    }
                                    (*_he_newbkt).hh_head = _he_thh as *mut UT_hash_handle;
                                    _he_thh = _he_hh_nxt;
                                }
                                _he_bkt_i = _he_bkt_i.wrapping_add(1);
                            }
                            free((*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void);
                            (*(*item).hh.tbl).num_buckets = (*(*item).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint);
                            (*(*item).hh.tbl).log2_num_buckets =
                                (*(*item).hh.tbl).log2_num_buckets.wrapping_add(1);
                            (*(*item).hh.tbl).buckets = _he_new_buckets;
                            (*(*item).hh.tbl).ineff_expands = if (*(*item).hh.tbl).nonideal_items
                                > (*(*item).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                            {
                                (*(*item).hh.tbl)
                                    .ineff_expands
                                    .wrapping_add(1 as ::core::ffi::c_uint)
                            } else {
                                0 as ::core::ffi::c_uint
                            };
                            if (*(*item).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                                (*(*item).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                            }
                        }
                    }
                }
                j = j.wrapping_add(1);
            }
            let mut _hs_i: ::core::ffi::c_uint = 0;
            let mut _hs_looping: ::core::ffi::c_uint = 0;
            let mut _hs_nmerges: ::core::ffi::c_uint = 0;
            let mut _hs_insize: ::core::ffi::c_uint = 0;
            let mut _hs_psize: ::core::ffi::c_uint = 0;
            let mut _hs_qsize: ::core::ffi::c_uint = 0;
            let mut _hs_p: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _hs_q: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _hs_e: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _hs_list: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _hs_tail: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            if !hash.is_null() {
                _hs_insize = 1 as ::core::ffi::c_uint;
                _hs_looping = 1 as ::core::ffi::c_uint;
                _hs_list = &raw mut (*hash).hh as *mut UT_hash_handle;
                while _hs_looping != 0 as ::core::ffi::c_uint {
                    _hs_p = _hs_list;
                    _hs_list = ::core::ptr::null_mut::<UT_hash_handle>();
                    _hs_tail = ::core::ptr::null_mut::<UT_hash_handle>();
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
                                    .offset((*(*hash).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
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
                                        .offset((*(*hash).hh.tbl).hho)
                                        as *mut UT_hash_handle
                                } else {
                                    ::core::ptr::null_mut::<UT_hash_handle>()
                                }) as *mut UT_hash_handle;
                                _hs_qsize = _hs_qsize.wrapping_sub(1);
                            } else if _hs_qsize == 0 as ::core::ffi::c_uint || _hs_q.is_null() {
                                _hs_e = _hs_p;
                                if !_hs_p.is_null() {
                                    _hs_p = (if !(*_hs_p).next.is_null() {
                                        ((*_hs_p).next as *mut ::core::ffi::c_char)
                                            .offset((*(*hash).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                    } else {
                                        ::core::ptr::null_mut::<UT_hash_handle>()
                                    })
                                        as *mut UT_hash_handle;
                                }
                                _hs_psize = _hs_psize.wrapping_sub(1);
                            } else if by_covIndex(
                                (_hs_p as *mut ::core::ffi::c_char)
                                    .offset(-(*(*hash).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut coverage_entry,
                                (_hs_q as *mut ::core::ffi::c_char)
                                    .offset(-(*(*hash).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut coverage_entry,
                            ) <= 0 as ::core::ffi::c_int
                            {
                                _hs_e = _hs_p;
                                if !_hs_p.is_null() {
                                    _hs_p = (if !(*_hs_p).next.is_null() {
                                        ((*_hs_p).next as *mut ::core::ffi::c_char)
                                            .offset((*(*hash).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                    } else {
                                        ::core::ptr::null_mut::<UT_hash_handle>()
                                    })
                                        as *mut UT_hash_handle;
                                }
                                _hs_psize = _hs_psize.wrapping_sub(1);
                            } else {
                                _hs_e = _hs_q;
                                _hs_q = (if !(*_hs_q).next.is_null() {
                                    ((*_hs_q).next as *mut ::core::ffi::c_char)
                                        .offset((*(*hash).hh.tbl).hho)
                                        as *mut UT_hash_handle
                                } else {
                                    ::core::ptr::null_mut::<UT_hash_handle>()
                                }) as *mut UT_hash_handle;
                                _hs_qsize = _hs_qsize.wrapping_sub(1);
                            }
                            if !_hs_tail.is_null() {
                                (*_hs_tail).next = if !_hs_e.is_null() {
                                    (_hs_e as *mut ::core::ffi::c_char)
                                        .offset(-(*(*hash).hh.tbl).hho)
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
                                        .offset(-(*(*hash).hh.tbl).hho)
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
                        (*(*hash).hh.tbl).tail = _hs_tail;
                        hash = (_hs_list as *mut ::core::ffi::c_char)
                            .offset(-(*(*hash).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut coverage_entry
                            as *mut coverage_entry;
                    }
                    _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
                }
            }
            let mut e: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
            let mut tmp: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
            e = hash;
            tmp = (if !hash.is_null() {
                (*hash).hh.next
            } else {
                NULL
            }) as *mut coverage_entry as *mut coverage_entry;
            while !e.is_null() {
                pushToCoverage(
                    coverage,
                    handle_fromIndex(
                        (*e).gid as glyphid_t,
                    ) as otfcc_GlyphHandle,
                );
                let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*e).hh;
                if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                    free((*(*hash).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    free((*hash).hh.tbl as *mut ::core::ffi::c_void);
                    hash = ::core::ptr::null_mut::<coverage_entry>();
                } else {
                    let mut _hd_bkt: ::core::ffi::c_uint = 0;
                    if _hd_hh_del == (*(*hash).hh.tbl).tail {
                        (*(*hash).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                            .offset((*(*hash).hh.tbl).hho)
                            as *mut UT_hash_handle
                            as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del).prev.is_null() {
                        let ref mut fresh1 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                            .offset((*(*hash).hh.tbl).hho)
                            as *mut UT_hash_handle))
                            .next;
                        *fresh1 = (*_hd_hh_del).next;
                    } else {
                        hash = (*_hd_hh_del).next as *mut coverage_entry as *mut coverage_entry;
                    }
                    if !(*_hd_hh_del).next.is_null() {
                        let ref mut fresh2 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                            .offset((*(*hash).hh.tbl).hho)
                            as *mut UT_hash_handle))
                            .prev;
                        *fresh2 = (*_hd_hh_del).prev;
                    }
                    _hd_bkt = (*_hd_hh_del).hashv
                        & (*(*hash).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _hd_head: *mut UT_hash_bucket =
                        (*(*hash).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UT_hash_bucket;
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
                    (*(*hash).hh.tbl).num_items = (*(*hash).hh.tbl).num_items.wrapping_sub(1);
                }
                free(e as *mut ::core::ffi::c_void);
                e = ::core::ptr::null_mut::<coverage_entry>();
                e = tmp;
                tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut coverage_entry
                    as *mut coverage_entry;
            }
        }
        2 => {
            let mut rangeCount: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize),
            );
            if tableLength
                < offset.wrapping_add(4 as u32).wrapping_add(
                    (rangeCount as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as u32,
                )
            {
                return coverage;
            }
            let mut hash_0: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
            let mut j_0: u16 = 0 as u16;
            while (j_0 as ::core::ffi::c_int) < rangeCount as ::core::ffi::c_int {
                let mut start: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize),
                );
                let mut end: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize)
                        .offset(2 as ::core::ffi::c_int as isize),
                );
                let mut startCoverageIndex: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                );
                let mut k: ::core::ffi::c_int = start as ::core::ffi::c_int;
                while k <= end as ::core::ffi::c_int {
                    let mut item_0: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
                    let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
                    let mut _hj_i_1: ::core::ffi::c_uint = 0;
                    let mut _hj_j_1: ::core::ffi::c_uint = 0;
                    let mut _hj_k_1: ::core::ffi::c_uint = 0;
                    let mut _hj_key_1: *const ::core::ffi::c_uchar =
                        &raw mut k as *const ::core::ffi::c_uchar;
                    _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
                    _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
                    _hj_i_1 = _hj_j_1;
                    _hj_k_1 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                    while _hj_k_1 >= 12 as ::core::ffi::c_uint {
                        _hj_i_1 = _hj_i_1.wrapping_add(
                            (*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint,
                    );
                    let mut current_block_479: u64;
                    match _hj_k_1 {
                        11 => {
                            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                                (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_479 = 6699201740905612191;
                        }
                        10 => {
                            current_block_479 = 6699201740905612191;
                        }
                        9 => {
                            current_block_479 = 8648268707402713046;
                        }
                        8 => {
                            current_block_479 = 8227823895810557789;
                        }
                        7 => {
                            current_block_479 = 427879732619283140;
                        }
                        6 => {
                            current_block_479 = 2713130422797569290;
                        }
                        5 => {
                            current_block_479 = 2238069592525523793;
                        }
                        4 => {
                            current_block_479 = 12591124246248289442;
                        }
                        3 => {
                            current_block_479 = 17777263985481006496;
                        }
                        2 => {
                            current_block_479 = 9014961502161419347;
                        }
                        1 => {
                            current_block_479 = 4628848160879612205;
                        }
                        _ => {
                            current_block_479 = 15926959532756003978;
                        }
                    }
                    match current_block_479 {
                        6699201740905612191 => {
                            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                                (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_479 = 8648268707402713046;
                        }
                        _ => {}
                    }
                    match current_block_479 {
                        8648268707402713046 => {
                            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                                (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_479 = 8227823895810557789;
                        }
                        _ => {}
                    }
                    match current_block_479 {
                        8227823895810557789 => {
                            _hj_j_1 = _hj_j_1.wrapping_add(
                                (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_479 = 427879732619283140;
                        }
                        _ => {}
                    }
                    match current_block_479 {
                        427879732619283140 => {
                            _hj_j_1 = _hj_j_1.wrapping_add(
                                (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_479 = 2713130422797569290;
                        }
                        _ => {}
                    }
                    match current_block_479 {
                        2713130422797569290 => {
                            _hj_j_1 = _hj_j_1.wrapping_add(
                                (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_479 = 2238069592525523793;
                        }
                        _ => {}
                    }
                    match current_block_479 {
                        2238069592525523793 => {
                            _hj_j_1 = _hj_j_1
                                .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_479 = 12591124246248289442;
                        }
                        _ => {}
                    }
                    match current_block_479 {
                        12591124246248289442 => {
                            _hj_i_1 = _hj_i_1.wrapping_add(
                                (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_479 = 17777263985481006496;
                        }
                        _ => {}
                    }
                    match current_block_479 {
                        17777263985481006496 => {
                            _hj_i_1 = _hj_i_1.wrapping_add(
                                (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_479 = 9014961502161419347;
                        }
                        _ => {}
                    }
                    match current_block_479 {
                        9014961502161419347 => {
                            _hj_i_1 = _hj_i_1.wrapping_add(
                                (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_479 = 4628848160879612205;
                        }
                        _ => {}
                    }
                    match current_block_479 {
                        4628848160879612205 => {
                            _hj_i_1 = _hj_i_1
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
                    item_0 = ::core::ptr::null_mut::<coverage_entry>();
                    if !hash_0.is_null() {
                        let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
                        _hf_bkt_0 = _hf_hashv_0
                            & (*(*hash_0).hh.tbl)
                                .num_buckets
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            if !(*(*(*hash_0).hh.tbl).buckets.offset(_hf_bkt_0 as isize))
                                .hh_head
                                .is_null()
                            {
                                item_0 = ((*(*(*hash_0).hh.tbl).buckets.offset(_hf_bkt_0 as isize))
                                    .hh_head
                                    as *mut ::core::ffi::c_char)
                                    .offset(-(*(*hash_0).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut coverage_entry
                                    as *mut coverage_entry;
                            } else {
                                item_0 = ::core::ptr::null_mut::<coverage_entry>();
                            }
                            while !item_0.is_null() {
                                if (*item_0).hh.hashv == _hf_hashv_0
                                    && (*item_0).hh.keylen as usize
                                        == ::core::mem::size_of::<::core::ffi::c_int>()
                                {
                                    if memcmp(
                                        (*item_0).hh.key,
                                        &raw mut k as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        break;
                                    }
                                }
                                if !(*item_0).hh.hh_next.is_null() {
                                    item_0 = ((*item_0).hh.hh_next as *mut ::core::ffi::c_char)
                                        .offset(-(*(*hash_0).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut coverage_entry
                                        as *mut coverage_entry;
                                } else {
                                    item_0 = ::core::ptr::null_mut::<coverage_entry>();
                                }
                            }
                        }
                    }
                    if item_0.is_null() {
                        item_0 = __caryll_allocate_clean(
                            ::core::mem::size_of::<coverage_entry>() as usize,
                            87 as ::core::ffi::c_ulong,
                        ) as *mut coverage_entry;
                        (*item_0).gid = k;
                        (*item_0).covIndex = startCoverageIndex as ::core::ffi::c_int + k;
                        let mut _ha_hashv_0: ::core::ffi::c_uint = 0;
                        let mut _hj_i_2: ::core::ffi::c_uint = 0;
                        let mut _hj_j_2: ::core::ffi::c_uint = 0;
                        let mut _hj_k_2: ::core::ffi::c_uint = 0;
                        let mut _hj_key_2: *const ::core::ffi::c_uchar =
                            &raw mut (*item_0).gid as *const ::core::ffi::c_uchar;
                        _ha_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
                        _hj_j_2 = 0x9e3779b9 as ::core::ffi::c_uint;
                        _hj_i_2 = _hj_j_2;
                        _hj_k_2 =
                            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
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
                            _ha_hashv_0 = _ha_hashv_0.wrapping_add(
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
                            _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                            _hj_i_2 ^= _ha_hashv_0 >> 13 as ::core::ffi::c_int;
                            _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                            _hj_j_2 ^= _hj_i_2 << 8 as ::core::ffi::c_int;
                            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                            _ha_hashv_0 ^= _hj_j_2 >> 13 as ::core::ffi::c_int;
                            _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                            _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                            _hj_i_2 ^= _ha_hashv_0 >> 12 as ::core::ffi::c_int;
                            _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                            _hj_j_2 ^= _hj_i_2 << 16 as ::core::ffi::c_int;
                            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                            _ha_hashv_0 ^= _hj_j_2 >> 5 as ::core::ffi::c_int;
                            _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                            _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                            _hj_i_2 ^= _ha_hashv_0 >> 3 as ::core::ffi::c_int;
                            _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                            _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                            _hj_j_2 ^= _hj_i_2 << 10 as ::core::ffi::c_int;
                            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                            _ha_hashv_0 ^= _hj_j_2 >> 15 as ::core::ffi::c_int;
                            _hj_key_2 = _hj_key_2.offset(12 as ::core::ffi::c_int as isize);
                            _hj_k_2 = _hj_k_2.wrapping_sub(12 as ::core::ffi::c_uint);
                        }
                        _ha_hashv_0 = _ha_hashv_0
                            .wrapping_add(
                                ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint
                            );
                        let mut current_block_596: u64;
                        match _hj_k_2 {
                            11 => {
                                _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                                    (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_596 = 573882853410388416;
                            }
                            10 => {
                                current_block_596 = 573882853410388416;
                            }
                            9 => {
                                current_block_596 = 299352355811277932;
                            }
                            8 => {
                                current_block_596 = 13043562310048694990;
                            }
                            7 => {
                                current_block_596 = 12340946700440249448;
                            }
                            6 => {
                                current_block_596 = 6440256058613935855;
                            }
                            5 => {
                                current_block_596 = 6341329326336760484;
                            }
                            4 => {
                                current_block_596 = 5704352076678889949;
                            }
                            3 => {
                                current_block_596 = 1206395552884852128;
                            }
                            2 => {
                                current_block_596 = 5068356807518902766;
                            }
                            1 => {
                                current_block_596 = 17517446760338295470;
                            }
                            _ => {
                                current_block_596 = 18321550374714997443;
                            }
                        }
                        match current_block_596 {
                            573882853410388416 => {
                                _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                                    (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_596 = 299352355811277932;
                            }
                            _ => {}
                        }
                        match current_block_596 {
                            299352355811277932 => {
                                _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                                    (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_596 = 13043562310048694990;
                            }
                            _ => {}
                        }
                        match current_block_596 {
                            13043562310048694990 => {
                                _hj_j_2 = _hj_j_2.wrapping_add(
                                    (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_596 = 12340946700440249448;
                            }
                            _ => {}
                        }
                        match current_block_596 {
                            12340946700440249448 => {
                                _hj_j_2 = _hj_j_2.wrapping_add(
                                    (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_596 = 6440256058613935855;
                            }
                            _ => {}
                        }
                        match current_block_596 {
                            6440256058613935855 => {
                                _hj_j_2 = _hj_j_2.wrapping_add(
                                    (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_596 = 6341329326336760484;
                            }
                            _ => {}
                        }
                        match current_block_596 {
                            6341329326336760484 => {
                                _hj_j_2 = _hj_j_2.wrapping_add(
                                    *_hj_key_2.offset(4 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint,
                                );
                                current_block_596 = 5704352076678889949;
                            }
                            _ => {}
                        }
                        match current_block_596 {
                            5704352076678889949 => {
                                _hj_i_2 = _hj_i_2.wrapping_add(
                                    (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                );
                                current_block_596 = 1206395552884852128;
                            }
                            _ => {}
                        }
                        match current_block_596 {
                            1206395552884852128 => {
                                _hj_i_2 = _hj_i_2.wrapping_add(
                                    (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                );
                                current_block_596 = 5068356807518902766;
                            }
                            _ => {}
                        }
                        match current_block_596 {
                            5068356807518902766 => {
                                _hj_i_2 = _hj_i_2.wrapping_add(
                                    (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                );
                                current_block_596 = 17517446760338295470;
                            }
                            _ => {}
                        }
                        match current_block_596 {
                            17517446760338295470 => {
                                _hj_i_2 = _hj_i_2.wrapping_add(
                                    *_hj_key_2.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint,
                                );
                            }
                            _ => {}
                        }
                        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                        _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                        _hj_i_2 ^= _ha_hashv_0 >> 13 as ::core::ffi::c_int;
                        _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                        _hj_j_2 ^= _hj_i_2 << 8 as ::core::ffi::c_int;
                        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                        _ha_hashv_0 ^= _hj_j_2 >> 13 as ::core::ffi::c_int;
                        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                        _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                        _hj_i_2 ^= _ha_hashv_0 >> 12 as ::core::ffi::c_int;
                        _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                        _hj_j_2 ^= _hj_i_2 << 16 as ::core::ffi::c_int;
                        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                        _ha_hashv_0 ^= _hj_j_2 >> 5 as ::core::ffi::c_int;
                        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                        _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv_0);
                        _hj_i_2 ^= _ha_hashv_0 >> 3 as ::core::ffi::c_int;
                        _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv_0);
                        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                        _hj_j_2 ^= _hj_i_2 << 10 as ::core::ffi::c_int;
                        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_2);
                        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_2);
                        _ha_hashv_0 ^= _hj_j_2 >> 15 as ::core::ffi::c_int;
                        (*item_0).hh.hashv = _ha_hashv_0;
                        (*item_0).hh.key = &raw mut (*item_0).gid as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void;
                        (*item_0).hh.keylen =
                            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                        if hash_0.is_null() {
                            (*item_0).hh.next = NULL;
                            (*item_0).hh.prev = NULL;
                            (*item_0).hh.tbl =
                                malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                                    as *mut UT_hash_table
                                    as *mut UT_hash_table;
                            if (*item_0).hh.tbl.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*item_0).hh.tbl as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    ::core::mem::size_of::<UT_hash_table>() as usize,
                                );
                                (*(*item_0).hh.tbl).tail =
                                    &raw mut (*item_0).hh as *mut UT_hash_handle;
                                (*(*item_0).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                                (*(*item_0).hh.tbl).log2_num_buckets =
                                    HASH_INITIAL_NUM_BUCKETS_LOG2;
                                (*(*item_0).hh.tbl).hho = (&raw mut (*item_0).hh
                                    as *mut ::core::ffi::c_char)
                                    .offset_from(item_0 as *mut ::core::ffi::c_char)
                                    as ::core::ffi::c_long
                                    as isize;
                                (*(*item_0).hh.tbl).buckets =
                                    malloc((32 as usize).wrapping_mul(::core::mem::size_of::<
                                        UT_hash_bucket,
                                    >(
                                    )
                                        as usize))
                                        as *mut UT_hash_bucket;
                                (*(*item_0).hh.tbl).signature = HASH_SIGNATURE as u32;
                                if (*(*item_0).hh.tbl).buckets.is_null() {
                                    exit(-(1 as ::core::ffi::c_int));
                                } else {
                                    memset(
                                        (*(*item_0).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                        '\0' as i32,
                                        (32 as usize).wrapping_mul(::core::mem::size_of::<
                                            UT_hash_bucket,
                                        >(
                                        )
                                            as usize),
                                    );
                                }
                            }
                            hash_0 = item_0;
                        } else {
                            (*item_0).hh.tbl = (*hash_0).hh.tbl;
                            (*item_0).hh.next = NULL;
                            (*item_0).hh.prev = ((*(*hash_0).hh.tbl).tail
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(*hash_0).hh.tbl).hho)
                                as *mut ::core::ffi::c_void;
                            (*(*(*hash_0).hh.tbl).tail).next = item_0 as *mut ::core::ffi::c_void;
                            (*(*hash_0).hh.tbl).tail = &raw mut (*item_0).hh as *mut UT_hash_handle;
                        }
                        let mut _ha_bkt_0: ::core::ffi::c_uint = 0;
                        (*(*hash_0).hh.tbl).num_items =
                            (*(*hash_0).hh.tbl).num_items.wrapping_add(1);
                        _ha_bkt_0 = _ha_hashv_0
                            & (*(*hash_0).hh.tbl)
                                .num_buckets
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        let mut _ha_head_0: *mut UT_hash_bucket =
                            (*(*hash_0).hh.tbl).buckets.offset(_ha_bkt_0 as isize)
                                as *mut UT_hash_bucket;
                        (*_ha_head_0).count = (*_ha_head_0).count.wrapping_add(1);
                        (*item_0).hh.hh_next = (*_ha_head_0).hh_head as *mut UT_hash_handle;
                        (*item_0).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                        if !(*_ha_head_0).hh_head.is_null() {
                            (*(*_ha_head_0).hh_head).hh_prev =
                                &raw mut (*item_0).hh as *mut UT_hash_handle;
                        }
                        (*_ha_head_0).hh_head = &raw mut (*item_0).hh as *mut UT_hash_handle;
                        if (*_ha_head_0).count
                            >= (*_ha_head_0)
                                .expand_mult
                                .wrapping_add(1 as ::core::ffi::c_uint)
                                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                            && (*(*item_0).hh.tbl).noexpand == 0
                        {
                            let mut _he_bkt_0: ::core::ffi::c_uint = 0;
                            let mut _he_bkt_i_0: ::core::ffi::c_uint = 0;
                            let mut _he_thh_0: *mut UT_hash_handle =
                                ::core::ptr::null_mut::<UT_hash_handle>();
                            let mut _he_hh_nxt_0: *mut UT_hash_handle =
                                ::core::ptr::null_mut::<UT_hash_handle>();
                            let mut _he_new_buckets_0: *mut UT_hash_bucket =
                                ::core::ptr::null_mut::<UT_hash_bucket>();
                            let mut _he_newbkt_0: *mut UT_hash_bucket =
                                ::core::ptr::null_mut::<UT_hash_bucket>();
                            _he_new_buckets_0 = malloc(
                                (2 as usize)
                                    .wrapping_mul((*(*item_0).hh.tbl).num_buckets as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as usize
                                    ),
                            )
                                as *mut UT_hash_bucket;
                            if _he_new_buckets_0.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    _he_new_buckets_0 as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (2 as usize)
                                        .wrapping_mul((*(*item_0).hh.tbl).num_buckets as usize)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<UT_hash_bucket>() as usize
                                        ),
                                );
                                (*(*item_0).hh.tbl).ideal_chain_maxlen = ((*(*item_0).hh.tbl)
                                    .num_items
                                    >> (*(*item_0).hh.tbl)
                                        .log2_num_buckets
                                        .wrapping_add(1 as ::core::ffi::c_uint))
                                .wrapping_add(
                                    if (*(*item_0).hh.tbl).num_items
                                        & (*(*item_0).hh.tbl)
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
                                (*(*item_0).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                                _he_bkt_i_0 = 0 as ::core::ffi::c_uint;
                                while _he_bkt_i_0 < (*(*item_0).hh.tbl).num_buckets {
                                    _he_thh_0 =
                                        (*(*(*item_0).hh.tbl).buckets.offset(_he_bkt_i_0 as isize))
                                            .hh_head
                                            as *mut UT_hash_handle;
                                    while !_he_thh_0.is_null() {
                                        _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                                        _he_bkt_0 = (*_he_thh_0).hashv
                                            & (*(*item_0).hh.tbl)
                                                .num_buckets
                                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                                .wrapping_sub(1 as ::core::ffi::c_uint);
                                        _he_newbkt_0 = _he_new_buckets_0.offset(_he_bkt_0 as isize)
                                            as *mut UT_hash_bucket;
                                        (*_he_newbkt_0).count =
                                            (*_he_newbkt_0).count.wrapping_add(1);
                                        if (*_he_newbkt_0).count
                                            > (*(*item_0).hh.tbl).ideal_chain_maxlen
                                        {
                                            (*(*item_0).hh.tbl).nonideal_items =
                                                (*(*item_0).hh.tbl).nonideal_items.wrapping_add(1);
                                            (*_he_newbkt_0).expand_mult =
                                                (*_he_newbkt_0).count.wrapping_div(
                                                    (*(*item_0).hh.tbl).ideal_chain_maxlen,
                                                );
                                        }
                                        (*_he_thh_0).hh_prev =
                                            ::core::ptr::null_mut::<UT_hash_handle>();
                                        (*_he_thh_0).hh_next =
                                            (*_he_newbkt_0).hh_head as *mut UT_hash_handle;
                                        if !(*_he_newbkt_0).hh_head.is_null() {
                                            (*(*_he_newbkt_0).hh_head).hh_prev = _he_thh_0;
                                        }
                                        (*_he_newbkt_0).hh_head = _he_thh_0 as *mut UT_hash_handle;
                                        _he_thh_0 = _he_hh_nxt_0;
                                    }
                                    _he_bkt_i_0 = _he_bkt_i_0.wrapping_add(1);
                                }
                                free((*(*item_0).hh.tbl).buckets as *mut ::core::ffi::c_void);
                                (*(*item_0).hh.tbl).num_buckets = (*(*item_0).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint);
                                (*(*item_0).hh.tbl).log2_num_buckets =
                                    (*(*item_0).hh.tbl).log2_num_buckets.wrapping_add(1);
                                (*(*item_0).hh.tbl).buckets = _he_new_buckets_0;
                                (*(*item_0).hh.tbl).ineff_expands = if (*(*item_0).hh.tbl)
                                    .nonideal_items
                                    > (*(*item_0).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                                {
                                    (*(*item_0).hh.tbl)
                                        .ineff_expands
                                        .wrapping_add(1 as ::core::ffi::c_uint)
                                } else {
                                    0 as ::core::ffi::c_uint
                                };
                                if (*(*item_0).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                                    (*(*item_0).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                                }
                            }
                        }
                    }
                    k += 1;
                }
                j_0 = j_0.wrapping_add(1);
            }
            let mut _hs_i_0: ::core::ffi::c_uint = 0;
            let mut _hs_looping_0: ::core::ffi::c_uint = 0;
            let mut _hs_nmerges_0: ::core::ffi::c_uint = 0;
            let mut _hs_insize_0: ::core::ffi::c_uint = 0;
            let mut _hs_psize_0: ::core::ffi::c_uint = 0;
            let mut _hs_qsize_0: ::core::ffi::c_uint = 0;
            let mut _hs_p_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _hs_q_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _hs_e_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _hs_list_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _hs_tail_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            if !hash_0.is_null() {
                _hs_insize_0 = 1 as ::core::ffi::c_uint;
                _hs_looping_0 = 1 as ::core::ffi::c_uint;
                _hs_list_0 = &raw mut (*hash_0).hh as *mut UT_hash_handle;
                while _hs_looping_0 != 0 as ::core::ffi::c_uint {
                    _hs_p_0 = _hs_list_0;
                    _hs_list_0 = ::core::ptr::null_mut::<UT_hash_handle>();
                    _hs_tail_0 = ::core::ptr::null_mut::<UT_hash_handle>();
                    _hs_nmerges_0 = 0 as ::core::ffi::c_uint;
                    while !_hs_p_0.is_null() {
                        _hs_nmerges_0 = _hs_nmerges_0.wrapping_add(1);
                        _hs_q_0 = _hs_p_0;
                        _hs_psize_0 = 0 as ::core::ffi::c_uint;
                        _hs_i_0 = 0 as ::core::ffi::c_uint;
                        while _hs_i_0 < _hs_insize_0 {
                            _hs_psize_0 = _hs_psize_0.wrapping_add(1);
                            _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                                ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                    .offset((*(*hash_0).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                            if _hs_q_0.is_null() {
                                break;
                            }
                            _hs_i_0 = _hs_i_0.wrapping_add(1);
                        }
                        _hs_qsize_0 = _hs_insize_0;
                        while _hs_psize_0 != 0 as ::core::ffi::c_uint
                            || _hs_qsize_0 != 0 as ::core::ffi::c_uint && !_hs_q_0.is_null()
                        {
                            if _hs_psize_0 == 0 as ::core::ffi::c_uint {
                                _hs_e_0 = _hs_q_0;
                                _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                                    ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                        .offset((*(*hash_0).hh.tbl).hho)
                                        as *mut UT_hash_handle
                                } else {
                                    ::core::ptr::null_mut::<UT_hash_handle>()
                                }) as *mut UT_hash_handle;
                                _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                            } else if _hs_qsize_0 == 0 as ::core::ffi::c_uint || _hs_q_0.is_null() {
                                _hs_e_0 = _hs_p_0;
                                if !_hs_p_0.is_null() {
                                    _hs_p_0 = (if !(*_hs_p_0).next.is_null() {
                                        ((*_hs_p_0).next as *mut ::core::ffi::c_char)
                                            .offset((*(*hash_0).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                    } else {
                                        ::core::ptr::null_mut::<UT_hash_handle>()
                                    })
                                        as *mut UT_hash_handle;
                                }
                                _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                            } else if by_covIndex(
                                (_hs_p_0 as *mut ::core::ffi::c_char)
                                    .offset(-(*(*hash_0).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut coverage_entry,
                                (_hs_q_0 as *mut ::core::ffi::c_char)
                                    .offset(-(*(*hash_0).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut coverage_entry,
                            ) <= 0 as ::core::ffi::c_int
                            {
                                _hs_e_0 = _hs_p_0;
                                if !_hs_p_0.is_null() {
                                    _hs_p_0 = (if !(*_hs_p_0).next.is_null() {
                                        ((*_hs_p_0).next as *mut ::core::ffi::c_char)
                                            .offset((*(*hash_0).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                    } else {
                                        ::core::ptr::null_mut::<UT_hash_handle>()
                                    })
                                        as *mut UT_hash_handle;
                                }
                                _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                            } else {
                                _hs_e_0 = _hs_q_0;
                                _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                                    ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                        .offset((*(*hash_0).hh.tbl).hho)
                                        as *mut UT_hash_handle
                                } else {
                                    ::core::ptr::null_mut::<UT_hash_handle>()
                                }) as *mut UT_hash_handle;
                                _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                            }
                            if !_hs_tail_0.is_null() {
                                (*_hs_tail_0).next = if !_hs_e_0.is_null() {
                                    (_hs_e_0 as *mut ::core::ffi::c_char)
                                        .offset(-(*(*hash_0).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                } else {
                                    NULL
                                };
                            } else {
                                _hs_list_0 = _hs_e_0;
                            }
                            if !_hs_e_0.is_null() {
                                (*_hs_e_0).prev = if !_hs_tail_0.is_null() {
                                    (_hs_tail_0 as *mut ::core::ffi::c_char)
                                        .offset(-(*(*hash_0).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                } else {
                                    NULL
                                };
                            }
                            _hs_tail_0 = _hs_e_0;
                        }
                        _hs_p_0 = _hs_q_0;
                    }
                    if !_hs_tail_0.is_null() {
                        (*_hs_tail_0).next = NULL;
                    }
                    if _hs_nmerges_0 <= 1 as ::core::ffi::c_uint {
                        _hs_looping_0 = 0 as ::core::ffi::c_uint;
                        (*(*hash_0).hh.tbl).tail = _hs_tail_0;
                        hash_0 = (_hs_list_0 as *mut ::core::ffi::c_char)
                            .offset(-(*(*hash_0).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut coverage_entry
                            as *mut coverage_entry;
                    }
                    _hs_insize_0 = _hs_insize_0.wrapping_mul(2 as ::core::ffi::c_uint);
                }
            }
            let mut e_0: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
            let mut tmp_0: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
            e_0 = hash_0;
            tmp_0 = (if !hash_0.is_null() {
                (*hash_0).hh.next
            } else {
                NULL
            }) as *mut coverage_entry as *mut coverage_entry;
            while !e_0.is_null() {
                pushToCoverage(
                    coverage,
                    handle_fromIndex(
                        (*e_0).gid as glyphid_t,
                    ) as otfcc_GlyphHandle,
                );
                let mut _hd_hh_del_0: *mut UT_hash_handle = &raw mut (*e_0).hh;
                if (*_hd_hh_del_0).prev.is_null() && (*_hd_hh_del_0).next.is_null() {
                    free((*(*hash_0).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    free((*hash_0).hh.tbl as *mut ::core::ffi::c_void);
                    hash_0 = ::core::ptr::null_mut::<coverage_entry>();
                } else {
                    let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                    if _hd_hh_del_0 == (*(*hash_0).hh.tbl).tail {
                        (*(*hash_0).hh.tbl).tail = ((*_hd_hh_del_0).prev
                            as *mut ::core::ffi::c_char)
                            .offset((*(*hash_0).hh.tbl).hho)
                            as *mut UT_hash_handle
                            as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del_0).prev.is_null() {
                        let ref mut fresh3 = (*(((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                            .offset((*(*hash_0).hh.tbl).hho)
                            as *mut UT_hash_handle))
                            .next;
                        *fresh3 = (*_hd_hh_del_0).next;
                    } else {
                        hash_0 = (*_hd_hh_del_0).next as *mut coverage_entry as *mut coverage_entry;
                    }
                    if !(*_hd_hh_del_0).next.is_null() {
                        let ref mut fresh4 = (*(((*_hd_hh_del_0).next as *mut ::core::ffi::c_char)
                            .offset((*(*hash_0).hh.tbl).hho)
                            as *mut UT_hash_handle))
                            .prev;
                        *fresh4 = (*_hd_hh_del_0).prev;
                    }
                    _hd_bkt_0 = (*_hd_hh_del_0).hashv
                        & (*(*hash_0).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _hd_head_0: *mut UT_hash_bucket =
                        (*(*hash_0).hh.tbl).buckets.offset(_hd_bkt_0 as isize)
                            as *mut UT_hash_bucket;
                    (*_hd_head_0).count = (*_hd_head_0).count.wrapping_sub(1);
                    if (*_hd_head_0).hh_head == _hd_hh_del_0 {
                        (*_hd_head_0).hh_head = (*_hd_hh_del_0).hh_next as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del_0).hh_prev.is_null() {
                        (*(*_hd_hh_del_0).hh_prev).hh_next = (*_hd_hh_del_0).hh_next;
                    }
                    if !(*_hd_hh_del_0).hh_next.is_null() {
                        (*(*_hd_hh_del_0).hh_next).hh_prev = (*_hd_hh_del_0).hh_prev;
                    }
                    (*(*hash_0).hh.tbl).num_items = (*(*hash_0).hh.tbl).num_items.wrapping_sub(1);
                }
                free(e_0 as *mut ::core::ffi::c_void);
                e_0 = ::core::ptr::null_mut::<coverage_entry>();
                e_0 = tmp_0;
                tmp_0 = (if !tmp_0.is_null() {
                    (*tmp_0).hh.next
                } else {
                    NULL
                }) as *mut coverage_entry as *mut coverage_entry;
            }
        }
        _ => {}
    }
    return coverage;
}
pub(crate) unsafe extern "C" fn dumpCoverage(mut coverage: *const otl_Coverage) -> *mut json_value {
    let mut a: *mut json_value = json_array_new((*coverage).numGlyphs as usize);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*coverage).numGlyphs as ::core::ffi::c_int {
        json_array_push(
            a,
            json_string_new(
                (*(*coverage).glyphs.offset(j as isize)).name as *const ::core::ffi::c_char,
            ),
        );
        j = j.wrapping_add(1);
    }
    return preserialize(a);
}
pub(crate) unsafe extern "C" fn parseCoverage(mut cov: *const json_value) -> *mut otl_Coverage {
    let mut c: *mut otl_Coverage = otl_Coverage_create();
    if cov.is_null()
        || (*cov).type_0 as ::core::ffi::c_uint
            != json_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return c;
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*cov).u.array.length {
        if (**(*cov).u.array.values.offset(j as isize)).type_0 as ::core::ffi::c_uint
            == json_string as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            pushToCoverage(
                c,
                handle_fromName(sdsnewlen(
                    (**(*cov).u.array.values.offset(j as isize)).u.string.ptr
                        as *const ::core::ffi::c_void,
                    (**(*cov).u.array.values.offset(j as isize)).u.string.length as usize,
                )) as otfcc_GlyphHandle,
            );
        }
        j = j.wrapping_add(1);
    }
    return c;
}
unsafe extern "C" fn by_gid(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return *(a as *mut glyphid_t) as ::core::ffi::c_int
        - *(b as *mut glyphid_t) as ::core::ffi::c_int;
}
pub(crate) unsafe extern "C" fn buildCoverageFormat(
    mut coverage: *const otl_Coverage,
    mut format: u16,
) -> *mut caryll_Buffer {
    if (*coverage).numGlyphs == 0 {
        let mut buf: *mut caryll_Buffer = bufnew();
        bufwrite16b(buf, 2 as u16);
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    let mut r: *mut glyphid_t = ::core::ptr::null_mut::<glyphid_t>();
    r = __caryll_allocate_clean(
        (::core::mem::size_of::<glyphid_t>() as usize)
            .wrapping_mul((*coverage).numGlyphs as usize),
        144 as ::core::ffi::c_ulong,
    ) as *mut glyphid_t;
    let mut jj: glyphid_t = 0 as glyphid_t;
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*coverage).numGlyphs as ::core::ffi::c_int {
        *r.offset(jj as isize) = (*(*coverage).glyphs.offset(j as isize)).index;
        jj = jj.wrapping_add(1);
        j = j.wrapping_add(1);
    }
    qsort(
        r as *mut ::core::ffi::c_void,
        jj as usize,
        ::core::mem::size_of::<glyphid_t>() as usize,
        Some(
            by_gid
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut format1: *mut caryll_Buffer = bufnew();
    bufwrite16b(format1, 1 as u16);
    bufwrite16b(format1, jj as u16);
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        bufwrite16b(format1, *r.offset(j_0 as isize) as u16);
        j_0 = j_0.wrapping_add(1);
    }
    if (jj as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<glyphid_t>();
        return format1;
    }
    let mut format2: *mut caryll_Buffer = bufnew();
    bufwrite16b(format2, 2 as u16);
    let mut ranges: *mut caryll_Buffer = bufnew();
    let mut startGID: glyphid_t = *r.offset(0 as ::core::ffi::c_int as isize);
    let mut endGID: glyphid_t = startGID;
    let mut lastGID: glyphid_t = startGID;
    let mut nRanges: glyphid_t = 0 as glyphid_t;
    let mut j_1: glyphid_t = 1 as glyphid_t;
    while (j_1 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        let mut current: glyphid_t = *r.offset(j_1 as isize);
        if !(current as ::core::ffi::c_int <= lastGID as ::core::ffi::c_int) {
            if current as ::core::ffi::c_int
                == endGID as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            {
                endGID = current;
            } else {
                bufwrite16b(ranges, startGID as u16);
                bufwrite16b(ranges, endGID as u16);
                bufwrite16b(
                    ranges,
                    (j_1 as ::core::ffi::c_int + startGID as ::core::ffi::c_int
                        - endGID as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as u16,
                );
                nRanges = (nRanges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
                endGID = current;
                startGID = endGID;
            }
            lastGID = current;
        }
        j_1 = j_1.wrapping_add(1);
    }
    bufwrite16b(ranges, startGID as u16);
    bufwrite16b(ranges, endGID as u16);
    bufwrite16b(
        ranges,
        (jj as ::core::ffi::c_int + startGID as ::core::ffi::c_int
            - endGID as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as u16,
    );
    nRanges = (nRanges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
    bufwrite16b(format2, nRanges as u16);
    bufwrite_bufdel(format2, ranges);
    if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        buffree(format2);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<glyphid_t>();
        return format1;
    } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        buffree(format1);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<glyphid_t>();
        return format2;
    } else if buflen(format1) < buflen(format2) {
        buffree(format2);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<glyphid_t>();
        return format1;
    } else {
        buffree(format1);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<glyphid_t>();
        return format2;
    };
}
pub(crate) unsafe extern "C" fn buildCoverage(mut coverage: *const otl_Coverage) -> *mut caryll_Buffer {
    return buildCoverageFormat(coverage, 0 as u16);
}
unsafe extern "C" fn byHandleGID(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut glyph_handle)).index as ::core::ffi::c_int
        - (*(b as *mut glyph_handle)).index as ::core::ffi::c_int;
}
pub(crate) unsafe extern "C" fn shrinkCoverage(mut coverage: *mut otl_Coverage, mut dosort: bool) {
    if coverage.is_null() {
        return;
    }
    let mut k: glyphid_t = 0 as glyphid_t;
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*coverage).numGlyphs as ::core::ffi::c_int {
        if !(*(*coverage).glyphs.offset(j as isize)).name.is_null() {
            let fresh0 = k;
            k = k.wrapping_add(1);
            *(*coverage).glyphs.offset(fresh0 as isize) = *(*coverage).glyphs.offset(j as isize);
        } else {
            otfcc_Handle_dispose(
                (*coverage).glyphs.offset(j as isize) as *mut otfcc_Handle,
            );
        }
        j = j.wrapping_add(1);
    }
    if dosort {
        qsort(
            (*coverage).glyphs as *mut ::core::ffi::c_void,
            k as usize,
            ::core::mem::size_of::<glyph_handle>() as usize,
            Some(
                byHandleGID
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut skip: glyphid_t = 0 as glyphid_t;
        let mut rear: glyphid_t = 1 as glyphid_t;
        while (rear as ::core::ffi::c_int) < k as ::core::ffi::c_int {
            if (*(*coverage).glyphs.offset(rear as isize)).index as ::core::ffi::c_int
                == (*(*coverage).glyphs.offset(
                    (rear as ::core::ffi::c_int
                        - skip as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as isize,
                ))
                .index as ::core::ffi::c_int
            {
                otfcc_Handle_dispose(
                    (*coverage).glyphs.offset(rear as isize) as *mut otfcc_Handle,
                );
                skip = (skip as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
            } else {
                *(*coverage)
                    .glyphs
                    .offset((rear as ::core::ffi::c_int - skip as ::core::ffi::c_int) as isize) =
                    *(*coverage).glyphs.offset(rear as isize);
            }
            rear = rear.wrapping_add(1);
        }
        k = (k as ::core::ffi::c_int - skip as ::core::ffi::c_int) as glyphid_t;
    }
    (*coverage).numGlyphs = k;
}
#[unsafe(no_mangle)]
pub static otl_iCoverage: __otfcc_ICoverage = {
    __otfcc_ICoverage {
        init: Some(otl_Coverage_init as unsafe extern "C" fn(*mut otl_Coverage) -> ()),
        copy: Some(
            otl_Coverage_copy as unsafe extern "C" fn(*mut otl_Coverage, *const otl_Coverage) -> (),
        ),
        move_0: Some(
            otl_Coverage_move as unsafe extern "C" fn(*mut otl_Coverage, *mut otl_Coverage) -> (),
        ),
        dispose: Some(otl_Coverage_dispose as unsafe extern "C" fn(*mut otl_Coverage) -> ()),
        replace: Some(
            otl_Coverage_replace as unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> (),
        ),
        copyReplace: Some(
            otl_Coverage_copyReplace as unsafe extern "C" fn(*mut otl_Coverage, otl_Coverage) -> (),
        ),
        create: Some(otl_Coverage_create),
        free: Some(otl_Coverage_free as unsafe extern "C" fn(*mut otl_Coverage) -> ()),
        clear: Some(clearCoverage as unsafe extern "C" fn(*mut otl_Coverage, u32) -> ()),
        read: Some(
            readCoverage
                as unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_Coverage,
        ),
        dump: Some(dumpCoverage as unsafe extern "C" fn(*const otl_Coverage) -> *mut json_value),
        parse: Some(parseCoverage as unsafe extern "C" fn(*const json_value) -> *mut otl_Coverage),
        build: Some(
            buildCoverage as unsafe extern "C" fn(*const otl_Coverage) -> *mut caryll_Buffer,
        ),
        buildFormat: Some(
            buildCoverageFormat
                as unsafe extern "C" fn(*const otl_Coverage, u16) -> *mut caryll_Buffer,
        ),
        shrink: Some(shrinkCoverage as unsafe extern "C" fn(*mut otl_Coverage, bool) -> ()),
        push: Some(
            pushToCoverage as unsafe extern "C" fn(*mut otl_Coverage, otfcc_GlyphHandle) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn preserialize(mut x: *mut json_value) -> *mut json_value {
    let mut opts: json_serialize_opts = json_serialize_opts {
        mode: json_serialize_mode_packed,
        opts: 0,
        indent_size: 0,
    };
    let mut preserialize_len: usize = json_measure_ex(x, opts);
    let mut buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let mut xx: *mut json_value = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as usize) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = json_pre_serialized;
    return xx;
}
