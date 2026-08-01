#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset, qsort};

use crate::support::json_funcs::{preserialize};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, Handle, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::support::buffer::{Buffer};
use crate::support::primitives::{GlyphId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::{NULL};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::support::buffer::{buffree, buflen, bufnew, bufwrite16b, bufwrite_bufdel};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_string_new};
use crate::vendor::sds::{sdsnewlen};
/// A glyph coverage set: C by way of c2rust had this as a hand-rolled
/// `malloc`/`realloc` array (`num_glyphs`/`capacity`/`glyphs: *mut
/// GlyphHandle`); it was never anything but a growable array of
/// `GlyphHandle`, so `Vec<GlyphHandle>` *is* `Coverage` now, not a struct
/// wrapping one -- same "C-native vector shape becomes a bare `pub type`"
/// call as `ColrTable`/`TsiTable` earlier in this migration.
pub type Coverage = Vec<GlyphHandle>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ICoverage {
    pub dump: Option<unsafe extern "C" fn(*const Coverage) -> *mut JsonValue>,
    pub parse: Option<unsafe extern "C" fn(*const JsonValue) -> *mut Coverage>,
    pub build: Option<unsafe extern "C" fn(*const Coverage) -> *mut Buffer>,
    pub build_format:
        Option<unsafe extern "C" fn(*const Coverage, u16) -> *mut Buffer>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoverageEntry {
    pub gid: ::core::ffi::c_int,
    pub cov_index: ::core::ffi::c_int,
    pub hh: UtHashHandle,
}
pub(crate) unsafe extern "C" fn otl_coverage_create() -> *mut Coverage {
    // `.write()`, not a field assignment: this is placement-constructing a
    // fresh `Vec` into unwritten `malloc`'d memory (`Coverage` is a bare
    // `Vec<GlyphHandle>` now), so there is nothing to read or drop first --
    // same reasoning as `ColrTable`/`TsiTable`.
    let x: *mut Coverage = malloc(::core::mem::size_of::<Coverage>() as usize) as *mut Coverage;
    x.write(Vec::new());
    x
}
pub(crate) unsafe extern "C" fn otl_coverage_free(mut x: *mut Coverage) {
    if x.is_null() {
        return;
    }
    otl_coverage_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub(crate) unsafe extern "C" fn otl_coverage_dispose(x: *mut Coverage) {
    // Dropping the old `Vec` here (via assignment) runs each element's
    // `Handle::drop` in turn, freeing every glyph name -- the explicit
    // per-element `otfcc_handle_dispose` loop this replaced is now
    // redundant, the same finding as the `Handle` Drop/Clone PR.
    *x = Vec::new();
}
unsafe extern "C" fn by_cov_index(
    mut a: *mut CoverageEntry,
    mut b: *mut CoverageEntry,
) -> ::core::ffi::c_int {
    return (*a).cov_index - (*b).cov_index;
}
pub(crate) unsafe extern "C" fn push_to_coverage(coverage: *mut Coverage, h: GlyphHandle) {
    (*coverage).push(h);
}
pub(crate) unsafe extern "C" fn read_coverage(
    mut data: *const u8,
    mut table_length: u32,
    mut offset: u32,
) -> *mut Coverage {
    let mut coverage: *mut Coverage = otl_coverage_create();
    if table_length < offset.wrapping_add(4 as u32) {
        return coverage;
    }
    let mut format: u16 = read_16u(data.offset(offset as isize));
    match format as ::core::ffi::c_int {
        1 => {
            let mut glyph_count: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize),
            );
            if table_length
                < offset.wrapping_add(4 as u32).wrapping_add(
                    (glyph_count as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                )
            {
                return coverage;
            }
            let mut hash: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
            let mut j: u16 = 0 as u16;
            while (j as ::core::ffi::c_int) < glyph_count as ::core::ffi::c_int {
                let mut item: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
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
                item = ::core::ptr::null_mut::<CoverageEntry>();
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
                                as *mut CoverageEntry
                                as *mut CoverageEntry;
                        } else {
                            item = ::core::ptr::null_mut::<CoverageEntry>();
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
                                    as *mut CoverageEntry
                                    as *mut CoverageEntry;
                            } else {
                                item = ::core::ptr::null_mut::<CoverageEntry>();
                            }
                        }
                    }
                }
                if item.is_null() {
                    item = __caryll_allocate_clean(
                        ::core::mem::size_of::<CoverageEntry>() as usize,
                        60 as ::core::ffi::c_ulong,
                    ) as *mut CoverageEntry;
                    (*item).gid = gid;
                    (*item).cov_index = j as ::core::ffi::c_int;
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
                        (*item).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                            as *mut UtHashTable
                            as *mut UtHashTable;
                        if (*item).hh.tbl.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*item).hh.tbl as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                ::core::mem::size_of::<UtHashTable>() as usize,
                            );
                            (*(*item).hh.tbl).tail = &raw mut (*item).hh as *mut UtHashHandle;
                            (*(*item).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                            (*(*item).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                            (*(*item).hh.tbl).hho = (&raw mut (*item).hh
                                as *mut ::core::ffi::c_char)
                                .offset_from(item as *mut ::core::ffi::c_char)
                                as ::core::ffi::c_long
                                as isize;
                            (*(*item).hh.tbl).buckets =
                                malloc((32 as usize).wrapping_mul(::core::mem::size_of::<
                                    UtHashBucket,
                                >(
                                )
                                    as usize))
                                    as *mut UtHashBucket;
                            (*(*item).hh.tbl).signature = HASH_SIGNATURE as u32;
                            if (*(*item).hh.tbl).buckets.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (32 as usize).wrapping_mul(
                                        ::core::mem::size_of::<UtHashBucket>() as usize,
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
                        (*(*hash).hh.tbl).tail = &raw mut (*item).hh as *mut UtHashHandle;
                    }
                    let mut _ha_bkt: ::core::ffi::c_uint = 0;
                    (*(*hash).hh.tbl).num_items = (*(*hash).hh.tbl).num_items.wrapping_add(1);
                    _ha_bkt = _ha_hashv
                        & (*(*hash).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _ha_head: *mut UtHashBucket =
                        (*(*hash).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
                    (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                    (*item).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
                    (*item).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                    if !(*_ha_head).hh_head.is_null() {
                        (*(*_ha_head).hh_head).hh_prev = &raw mut (*item).hh as *mut UtHashHandle;
                    }
                    (*_ha_head).hh_head = &raw mut (*item).hh as *mut UtHashHandle;
                    if (*_ha_head).count
                        >= (*_ha_head)
                            .expand_mult
                            .wrapping_add(1 as ::core::ffi::c_uint)
                            .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                        && (*(*item).hh.tbl).noexpand == 0
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
                                .wrapping_mul((*(*item).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        ) as *mut UtHashBucket;
                        if _he_new_buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                _he_new_buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (2 as usize)
                                    .wrapping_mul((*(*item).hh.tbl).num_buckets as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UtHashBucket>() as usize
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
                                    as *mut UtHashHandle;
                                while !_he_thh.is_null() {
                                    _he_hh_nxt = (*_he_thh).hh_next;
                                    _he_bkt = (*_he_thh).hashv
                                        & (*(*item).hh.tbl)
                                            .num_buckets
                                            .wrapping_mul(2 as ::core::ffi::c_uint)
                                            .wrapping_sub(1 as ::core::ffi::c_uint);
                                    _he_newbkt = _he_new_buckets.offset(_he_bkt as isize)
                                        as *mut UtHashBucket;
                                    (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                    if (*_he_newbkt).count > (*(*item).hh.tbl).ideal_chain_maxlen {
                                        (*(*item).hh.tbl).nonideal_items =
                                            (*(*item).hh.tbl).nonideal_items.wrapping_add(1);
                                        (*_he_newbkt).expand_mult = (*_he_newbkt)
                                            .count
                                            .wrapping_div((*(*item).hh.tbl).ideal_chain_maxlen);
                                    }
                                    (*_he_thh).hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                                    (*_he_thh).hh_next =
                                        (*_he_newbkt).hh_head as *mut UtHashHandle;
                                    if !(*_he_newbkt).hh_head.is_null() {
                                        (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                                    }
                                    (*_he_newbkt).hh_head = _he_thh as *mut UtHashHandle;
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
            let mut _hs_p: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _hs_q: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _hs_e: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _hs_list: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _hs_tail: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            if !hash.is_null() {
                _hs_insize = 1 as ::core::ffi::c_uint;
                _hs_looping = 1 as ::core::ffi::c_uint;
                _hs_list = &raw mut (*hash).hh as *mut UtHashHandle;
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
                                    .offset((*(*hash).hh.tbl).hho)
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
                                        .offset((*(*hash).hh.tbl).hho)
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
                                            .offset((*(*hash).hh.tbl).hho)
                                            as *mut UtHashHandle
                                    } else {
                                        ::core::ptr::null_mut::<UtHashHandle>()
                                    })
                                        as *mut UtHashHandle;
                                }
                                _hs_psize = _hs_psize.wrapping_sub(1);
                            } else if by_cov_index(
                                (_hs_p as *mut ::core::ffi::c_char)
                                    .offset(-(*(*hash).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut CoverageEntry,
                                (_hs_q as *mut ::core::ffi::c_char)
                                    .offset(-(*(*hash).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut CoverageEntry,
                            ) <= 0 as ::core::ffi::c_int
                            {
                                _hs_e = _hs_p;
                                if !_hs_p.is_null() {
                                    _hs_p = (if !(*_hs_p).next.is_null() {
                                        ((*_hs_p).next as *mut ::core::ffi::c_char)
                                            .offset((*(*hash).hh.tbl).hho)
                                            as *mut UtHashHandle
                                    } else {
                                        ::core::ptr::null_mut::<UtHashHandle>()
                                    })
                                        as *mut UtHashHandle;
                                }
                                _hs_psize = _hs_psize.wrapping_sub(1);
                            } else {
                                _hs_e = _hs_q;
                                _hs_q = (if !(*_hs_q).next.is_null() {
                                    ((*_hs_q).next as *mut ::core::ffi::c_char)
                                        .offset((*(*hash).hh.tbl).hho)
                                        as *mut UtHashHandle
                                } else {
                                    ::core::ptr::null_mut::<UtHashHandle>()
                                }) as *mut UtHashHandle;
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
                            as *mut CoverageEntry
                            as *mut CoverageEntry;
                    }
                    _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
                }
            }
            let mut e: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
            let mut tmp: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
            e = hash;
            tmp = (if !hash.is_null() {
                (*hash).hh.next
            } else {
                NULL
            }) as *mut CoverageEntry as *mut CoverageEntry;
            while !e.is_null() {
                push_to_coverage(
                    coverage,
                    handle_from_index(
                        (*e).gid as GlyphId,
                    ) as GlyphHandle,
                );
                let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*e).hh;
                if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                    free((*(*hash).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    free((*hash).hh.tbl as *mut ::core::ffi::c_void);
                    hash = ::core::ptr::null_mut::<CoverageEntry>();
                } else {
                    let mut _hd_bkt: ::core::ffi::c_uint = 0;
                    if _hd_hh_del == (*(*hash).hh.tbl).tail {
                        (*(*hash).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                            .offset((*(*hash).hh.tbl).hho)
                            as *mut UtHashHandle
                            as *mut UtHashHandle;
                    }
                    if !(*_hd_hh_del).prev.is_null() {
                        let ref mut fresh1 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                            .offset((*(*hash).hh.tbl).hho)
                            as *mut UtHashHandle))
                            .next;
                        *fresh1 = (*_hd_hh_del).next;
                    } else {
                        hash = (*_hd_hh_del).next as *mut CoverageEntry as *mut CoverageEntry;
                    }
                    if !(*_hd_hh_del).next.is_null() {
                        let ref mut fresh2 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                            .offset((*(*hash).hh.tbl).hho)
                            as *mut UtHashHandle))
                            .prev;
                        *fresh2 = (*_hd_hh_del).prev;
                    }
                    _hd_bkt = (*_hd_hh_del).hashv
                        & (*(*hash).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _hd_head: *mut UtHashBucket =
                        (*(*hash).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
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
                    (*(*hash).hh.tbl).num_items = (*(*hash).hh.tbl).num_items.wrapping_sub(1);
                }
                free(e as *mut ::core::ffi::c_void);
                e = ::core::ptr::null_mut::<CoverageEntry>();
                e = tmp;
                tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut CoverageEntry
                    as *mut CoverageEntry;
            }
        }
        2 => {
            let mut range_count: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize),
            );
            if table_length
                < offset.wrapping_add(4 as u32).wrapping_add(
                    (range_count as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as u32,
                )
            {
                return coverage;
            }
            let mut hash_0: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
            let mut j_0: u16 = 0 as u16;
            while (j_0 as ::core::ffi::c_int) < range_count as ::core::ffi::c_int {
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
                let mut start_coverage_index: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                );
                let mut k: ::core::ffi::c_int = start as ::core::ffi::c_int;
                while k <= end as ::core::ffi::c_int {
                    let mut item_0: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
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
                    item_0 = ::core::ptr::null_mut::<CoverageEntry>();
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
                                    as *mut CoverageEntry
                                    as *mut CoverageEntry;
                            } else {
                                item_0 = ::core::ptr::null_mut::<CoverageEntry>();
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
                                        as *mut CoverageEntry
                                        as *mut CoverageEntry;
                                } else {
                                    item_0 = ::core::ptr::null_mut::<CoverageEntry>();
                                }
                            }
                        }
                    }
                    if item_0.is_null() {
                        item_0 = __caryll_allocate_clean(
                            ::core::mem::size_of::<CoverageEntry>() as usize,
                            87 as ::core::ffi::c_ulong,
                        ) as *mut CoverageEntry;
                        (*item_0).gid = k;
                        (*item_0).cov_index = start_coverage_index as ::core::ffi::c_int + k;
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
                                malloc(::core::mem::size_of::<UtHashTable>() as usize)
                                    as *mut UtHashTable
                                    as *mut UtHashTable;
                            if (*item_0).hh.tbl.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*item_0).hh.tbl as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    ::core::mem::size_of::<UtHashTable>() as usize,
                                );
                                (*(*item_0).hh.tbl).tail =
                                    &raw mut (*item_0).hh as *mut UtHashHandle;
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
                                        UtHashBucket,
                                    >(
                                    )
                                        as usize))
                                        as *mut UtHashBucket;
                                (*(*item_0).hh.tbl).signature = HASH_SIGNATURE as u32;
                                if (*(*item_0).hh.tbl).buckets.is_null() {
                                    exit(-(1 as ::core::ffi::c_int));
                                } else {
                                    memset(
                                        (*(*item_0).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                        '\0' as i32,
                                        (32 as usize).wrapping_mul(::core::mem::size_of::<
                                            UtHashBucket,
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
                            (*(*hash_0).hh.tbl).tail = &raw mut (*item_0).hh as *mut UtHashHandle;
                        }
                        let mut _ha_bkt_0: ::core::ffi::c_uint = 0;
                        (*(*hash_0).hh.tbl).num_items =
                            (*(*hash_0).hh.tbl).num_items.wrapping_add(1);
                        _ha_bkt_0 = _ha_hashv_0
                            & (*(*hash_0).hh.tbl)
                                .num_buckets
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        let mut _ha_head_0: *mut UtHashBucket =
                            (*(*hash_0).hh.tbl).buckets.offset(_ha_bkt_0 as isize)
                                as *mut UtHashBucket;
                        (*_ha_head_0).count = (*_ha_head_0).count.wrapping_add(1);
                        (*item_0).hh.hh_next = (*_ha_head_0).hh_head as *mut UtHashHandle;
                        (*item_0).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                        if !(*_ha_head_0).hh_head.is_null() {
                            (*(*_ha_head_0).hh_head).hh_prev =
                                &raw mut (*item_0).hh as *mut UtHashHandle;
                        }
                        (*_ha_head_0).hh_head = &raw mut (*item_0).hh as *mut UtHashHandle;
                        if (*_ha_head_0).count
                            >= (*_ha_head_0)
                                .expand_mult
                                .wrapping_add(1 as ::core::ffi::c_uint)
                                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                            && (*(*item_0).hh.tbl).noexpand == 0
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
                                    .wrapping_mul((*(*item_0).hh.tbl).num_buckets as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UtHashBucket>() as usize
                                    ),
                            )
                                as *mut UtHashBucket;
                            if _he_new_buckets_0.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    _he_new_buckets_0 as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (2 as usize)
                                        .wrapping_mul((*(*item_0).hh.tbl).num_buckets as usize)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<UtHashBucket>() as usize
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
                                            as *mut UtHashHandle;
                                    while !_he_thh_0.is_null() {
                                        _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                                        _he_bkt_0 = (*_he_thh_0).hashv
                                            & (*(*item_0).hh.tbl)
                                                .num_buckets
                                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                                .wrapping_sub(1 as ::core::ffi::c_uint);
                                        _he_newbkt_0 = _he_new_buckets_0.offset(_he_bkt_0 as isize)
                                            as *mut UtHashBucket;
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
                                            ::core::ptr::null_mut::<UtHashHandle>();
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
            let mut _hs_p_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _hs_q_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _hs_e_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _hs_list_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _hs_tail_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            if !hash_0.is_null() {
                _hs_insize_0 = 1 as ::core::ffi::c_uint;
                _hs_looping_0 = 1 as ::core::ffi::c_uint;
                _hs_list_0 = &raw mut (*hash_0).hh as *mut UtHashHandle;
                while _hs_looping_0 != 0 as ::core::ffi::c_uint {
                    _hs_p_0 = _hs_list_0;
                    _hs_list_0 = ::core::ptr::null_mut::<UtHashHandle>();
                    _hs_tail_0 = ::core::ptr::null_mut::<UtHashHandle>();
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
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
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
                                        as *mut UtHashHandle
                                } else {
                                    ::core::ptr::null_mut::<UtHashHandle>()
                                }) as *mut UtHashHandle;
                                _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                            } else if _hs_qsize_0 == 0 as ::core::ffi::c_uint || _hs_q_0.is_null() {
                                _hs_e_0 = _hs_p_0;
                                if !_hs_p_0.is_null() {
                                    _hs_p_0 = (if !(*_hs_p_0).next.is_null() {
                                        ((*_hs_p_0).next as *mut ::core::ffi::c_char)
                                            .offset((*(*hash_0).hh.tbl).hho)
                                            as *mut UtHashHandle
                                    } else {
                                        ::core::ptr::null_mut::<UtHashHandle>()
                                    })
                                        as *mut UtHashHandle;
                                }
                                _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                            } else if by_cov_index(
                                (_hs_p_0 as *mut ::core::ffi::c_char)
                                    .offset(-(*(*hash_0).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut CoverageEntry,
                                (_hs_q_0 as *mut ::core::ffi::c_char)
                                    .offset(-(*(*hash_0).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut CoverageEntry,
                            ) <= 0 as ::core::ffi::c_int
                            {
                                _hs_e_0 = _hs_p_0;
                                if !_hs_p_0.is_null() {
                                    _hs_p_0 = (if !(*_hs_p_0).next.is_null() {
                                        ((*_hs_p_0).next as *mut ::core::ffi::c_char)
                                            .offset((*(*hash_0).hh.tbl).hho)
                                            as *mut UtHashHandle
                                    } else {
                                        ::core::ptr::null_mut::<UtHashHandle>()
                                    })
                                        as *mut UtHashHandle;
                                }
                                _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                            } else {
                                _hs_e_0 = _hs_q_0;
                                _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                                    ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                        .offset((*(*hash_0).hh.tbl).hho)
                                        as *mut UtHashHandle
                                } else {
                                    ::core::ptr::null_mut::<UtHashHandle>()
                                }) as *mut UtHashHandle;
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
                            as *mut CoverageEntry
                            as *mut CoverageEntry;
                    }
                    _hs_insize_0 = _hs_insize_0.wrapping_mul(2 as ::core::ffi::c_uint);
                }
            }
            let mut e_0: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
            let mut tmp_0: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
            e_0 = hash_0;
            tmp_0 = (if !hash_0.is_null() {
                (*hash_0).hh.next
            } else {
                NULL
            }) as *mut CoverageEntry as *mut CoverageEntry;
            while !e_0.is_null() {
                push_to_coverage(
                    coverage,
                    handle_from_index(
                        (*e_0).gid as GlyphId,
                    ) as GlyphHandle,
                );
                let mut _hd_hh_del_0: *mut UtHashHandle = &raw mut (*e_0).hh;
                if (*_hd_hh_del_0).prev.is_null() && (*_hd_hh_del_0).next.is_null() {
                    free((*(*hash_0).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    free((*hash_0).hh.tbl as *mut ::core::ffi::c_void);
                    hash_0 = ::core::ptr::null_mut::<CoverageEntry>();
                } else {
                    let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                    if _hd_hh_del_0 == (*(*hash_0).hh.tbl).tail {
                        (*(*hash_0).hh.tbl).tail = ((*_hd_hh_del_0).prev
                            as *mut ::core::ffi::c_char)
                            .offset((*(*hash_0).hh.tbl).hho)
                            as *mut UtHashHandle
                            as *mut UtHashHandle;
                    }
                    if !(*_hd_hh_del_0).prev.is_null() {
                        let ref mut fresh3 = (*(((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                            .offset((*(*hash_0).hh.tbl).hho)
                            as *mut UtHashHandle))
                            .next;
                        *fresh3 = (*_hd_hh_del_0).next;
                    } else {
                        hash_0 = (*_hd_hh_del_0).next as *mut CoverageEntry as *mut CoverageEntry;
                    }
                    if !(*_hd_hh_del_0).next.is_null() {
                        let ref mut fresh4 = (*(((*_hd_hh_del_0).next as *mut ::core::ffi::c_char)
                            .offset((*(*hash_0).hh.tbl).hho)
                            as *mut UtHashHandle))
                            .prev;
                        *fresh4 = (*_hd_hh_del_0).prev;
                    }
                    _hd_bkt_0 = (*_hd_hh_del_0).hashv
                        & (*(*hash_0).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _hd_head_0: *mut UtHashBucket =
                        (*(*hash_0).hh.tbl).buckets.offset(_hd_bkt_0 as isize)
                            as *mut UtHashBucket;
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
                    (*(*hash_0).hh.tbl).num_items = (*(*hash_0).hh.tbl).num_items.wrapping_sub(1);
                }
                free(e_0 as *mut ::core::ffi::c_void);
                e_0 = ::core::ptr::null_mut::<CoverageEntry>();
                e_0 = tmp_0;
                tmp_0 = (if !tmp_0.is_null() {
                    (*tmp_0).hh.next
                } else {
                    NULL
                }) as *mut CoverageEntry as *mut CoverageEntry;
            }
        }
        _ => {}
    }
    return coverage;
}
pub(crate) unsafe extern "C" fn dump_coverage(coverage: *const Coverage) -> *mut JsonValue {
    let mut a: *mut JsonValue = json_array_new((*coverage).len());
    for j in 0..(*coverage).len() {
        json_array_push(
            a,
            json_string_new((&(*coverage))[j].name as *const ::core::ffi::c_char),
        );
    }
    return preserialize(a);
}
pub(crate) unsafe extern "C" fn parse_coverage(mut cov: *const JsonValue) -> *mut Coverage {
    let mut c: *mut Coverage = otl_coverage_create();
    if cov.is_null()
        || (*cov).type_0 != JsonType::Array
    {
        return c;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*cov).u.array.length {
        if (**(*cov).u.array.values.offset(j as isize)).type_0 == JsonType::String
        {
            push_to_coverage(
                c,
                handle_from_name(sdsnewlen(
                    (**(*cov).u.array.values.offset(j as isize)).u.string.ptr
                        as *const ::core::ffi::c_void,
                    (**(*cov).u.array.values.offset(j as isize)).u.string.length as usize,
                )) as GlyphHandle,
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
    return *(a as *mut GlyphId) as ::core::ffi::c_int
        - *(b as *mut GlyphId) as ::core::ffi::c_int;
}
pub(crate) unsafe extern "C" fn build_coverage_format(
    mut coverage: *const Coverage,
    mut format: u16,
) -> *mut Buffer {
    if (*coverage).is_empty() {
        let mut buf: *mut Buffer = bufnew();
        bufwrite16b(buf, 2 as u16);
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    let mut r: *mut GlyphId = ::core::ptr::null_mut::<GlyphId>();
    r = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphId>() as usize).wrapping_mul((*coverage).len()),
        144 as ::core::ffi::c_ulong,
    ) as *mut GlyphId;
    let mut jj: GlyphId = 0 as GlyphId;
    for j in 0..(*coverage).len() {
        *r.offset(jj as isize) = (&(*coverage))[j].index;
        jj = jj.wrapping_add(1);
    }
    qsort(
        r as *mut ::core::ffi::c_void,
        jj as usize,
        ::core::mem::size_of::<GlyphId>() as usize,
        Some(
            by_gid
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut format1: *mut Buffer = bufnew();
    bufwrite16b(format1, 1 as u16);
    bufwrite16b(format1, jj as u16);
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        bufwrite16b(format1, *r.offset(j_0 as isize) as u16);
        j_0 = j_0.wrapping_add(1);
    }
    if (jj as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<GlyphId>();
        return format1;
    }
    let mut format2: *mut Buffer = bufnew();
    bufwrite16b(format2, 2 as u16);
    let mut ranges: *mut Buffer = bufnew();
    let mut start_gid: GlyphId = *r.offset(0 as ::core::ffi::c_int as isize);
    let mut end_gid: GlyphId = start_gid;
    let mut last_gid: GlyphId = start_gid;
    let mut n_ranges: GlyphId = 0 as GlyphId;
    let mut j_1: GlyphId = 1 as GlyphId;
    while (j_1 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        let mut current: GlyphId = *r.offset(j_1 as isize);
        if !(current as ::core::ffi::c_int <= last_gid as ::core::ffi::c_int) {
            if current as ::core::ffi::c_int
                == end_gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            {
                end_gid = current;
            } else {
                bufwrite16b(ranges, start_gid as u16);
                bufwrite16b(ranges, end_gid as u16);
                bufwrite16b(
                    ranges,
                    (j_1 as ::core::ffi::c_int + start_gid as ::core::ffi::c_int
                        - end_gid as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as u16,
                );
                n_ranges = (n_ranges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
                end_gid = current;
                start_gid = end_gid;
            }
            last_gid = current;
        }
        j_1 = j_1.wrapping_add(1);
    }
    bufwrite16b(ranges, start_gid as u16);
    bufwrite16b(ranges, end_gid as u16);
    bufwrite16b(
        ranges,
        (jj as ::core::ffi::c_int + start_gid as ::core::ffi::c_int
            - end_gid as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as u16,
    );
    n_ranges = (n_ranges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    bufwrite16b(format2, n_ranges as u16);
    bufwrite_bufdel(format2, ranges);
    if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        buffree(format2);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<GlyphId>();
        return format1;
    } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        buffree(format1);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<GlyphId>();
        return format2;
    } else if buflen(format1) < buflen(format2) {
        buffree(format2);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<GlyphId>();
        return format1;
    } else {
        buffree(format1);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<GlyphId>();
        return format2;
    };
}
pub(crate) unsafe extern "C" fn build_coverage(mut coverage: *const Coverage) -> *mut Buffer {
    return build_coverage_format(coverage, 0 as u16);
}
unsafe extern "C" fn by_handle_gid(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut GlyphHandle)).index as ::core::ffi::c_int
        - (*(b as *mut GlyphHandle)).index as ::core::ffi::c_int;
}
pub(crate) unsafe extern "C" fn shrink_coverage(coverage: *mut Coverage, dosort: bool) {
    if coverage.is_null() {
        return;
    }
    // Two `truncate`s, not one `num_glyphs = k` at the end as the original
    // did: each `truncate` lets `Vec`'s own drop glue free every handle
    // past the new length, including ones this function's own compaction
    // loops never got around to calling `otfcc_handle_dispose` on directly
    // (a survivor that gets superseded by a *later* compaction write, but
    // never becomes a write target itself, is exactly that case) -- the
    // original leaked that name; `truncate` doesn't.
    let mut k: usize = 0;
    for j in 0..(*coverage).len() {
        if !(&(*coverage))[j].name.is_null() {
            let elem = (&(*coverage))[j].clone();
            (&mut (*coverage))[k] = elem;
            k += 1;
        } else {
            otfcc_handle_dispose(&raw mut (&mut (*coverage))[j] as *mut Handle);
        }
    }
    (*coverage).truncate(k);
    if dosort {
        qsort(
            (*coverage).as_mut_ptr() as *mut ::core::ffi::c_void,
            (*coverage).len(),
            ::core::mem::size_of::<GlyphHandle>() as usize,
            Some(
                by_handle_gid
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut skip: usize = 0;
        let mut rear: usize = 1;
        while rear < (*coverage).len() {
            if (&(*coverage))[rear].index == (&(*coverage))[rear - skip - 1].index {
                otfcc_handle_dispose(&raw mut (&mut (*coverage))[rear] as *mut Handle);
                skip += 1;
            } else {
                let elem = (&(*coverage))[rear].clone();
                (&mut (*coverage))[rear - skip] = elem;
            }
            rear += 1;
        }
        let new_len = (*coverage).len() - skip;
        (*coverage).truncate(new_len);
    }
}
pub static OTL_I_COVERAGE: ICoverage = {
    ICoverage {
        dump: Some(dump_coverage as unsafe extern "C" fn(*const Coverage) -> *mut JsonValue),
        parse: Some(parse_coverage as unsafe extern "C" fn(*const JsonValue) -> *mut Coverage),
        build: Some(
            build_coverage as unsafe extern "C" fn(*const Coverage) -> *mut Buffer,
        ),
        build_format: Some(
            build_coverage_format
                as unsafe extern "C" fn(*const Coverage, u16) -> *mut Buffer,
        ),
    }
};
