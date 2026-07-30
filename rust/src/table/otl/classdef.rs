#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memcpy, memset, qsort};

use crate::support::json_funcs::{preserialize};
use crate::table::otl::coverage::{CoverageEntry, Coverage};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, Handle, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::binio::{read_16u};
use crate::support::buffer::{Buffer};
use crate::support::primitives::{GlyphClass, GlyphId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::{NULL};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite_bufdel};
use crate::vendor::json_builder::{json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClassDef {
    pub numGlyphs: GlyphId,
    pub capacity: u32,
    pub maxclass: GlyphClass,
    pub glyphs: *mut GlyphHandle,
    pub classes: *mut GlyphClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IClassDef {
    pub init: Option<unsafe extern "C" fn(*mut ClassDef) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut ClassDef, *const ClassDef) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut ClassDef, *mut ClassDef) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ClassDef) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut ClassDef, ClassDef) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut ClassDef, ClassDef) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ClassDef>,
    pub free: Option<unsafe extern "C" fn(*mut ClassDef) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut ClassDef, GlyphHandle, GlyphClass) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const u8, u32, u32) -> *mut ClassDef>,
    pub expand:
        Option<unsafe extern "C" fn(*mut Coverage, *mut ClassDef) -> *mut ClassDef>,
    pub dump: Option<unsafe extern "C" fn(*const ClassDef) -> *mut JsonValue>,
    pub parse: Option<unsafe extern "C" fn(*const JsonValue) -> *mut ClassDef>,
    pub build: Option<unsafe extern "C" fn(*const ClassDef) -> *mut Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut ClassDef) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClassDefSortRecord {
    pub gid: GlyphId,
    pub cid: GlyphClass,
}
#[inline]
unsafe extern "C" fn dispose_class_def(mut cd: *mut ClassDef) {
    if !(*cd).glyphs.is_null() {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
            otfcc_handle_dispose(
                (*cd).glyphs.offset(j as isize) as *mut Handle,
            );
            j = j.wrapping_add(1);
        }
        free((*cd).glyphs as *mut ::core::ffi::c_void);
        (*cd).glyphs = ::core::ptr::null_mut::<GlyphHandle>();
    }
    free((*cd).classes as *mut ::core::ffi::c_void);
    (*cd).classes = ::core::ptr::null_mut::<GlyphClass>();
}
#[inline]
pub(crate) unsafe extern "C" fn otl_class_def_replace(mut dst: *mut ClassDef, src: ClassDef) {
    otl_class_def_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ClassDef>() as usize,
    );
}
#[inline]
pub(crate) unsafe extern "C" fn otl_class_def_copy(mut dst: *mut ClassDef, mut src: *const ClassDef) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ClassDef>() as usize,
    );
}
#[inline]
pub(crate) unsafe extern "C" fn otl_class_def_free(mut x: *mut ClassDef) {
    if x.is_null() {
        return;
    }
    otl_class_def_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
pub(crate) unsafe extern "C" fn otl_class_def_dispose(mut x: *mut ClassDef) {
    dispose_class_def(x);
}
#[inline]
pub(crate) unsafe extern "C" fn otl_class_def_init(mut x: *mut ClassDef) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ClassDef>() as usize,
    );
}
#[inline]
pub(crate) unsafe extern "C" fn otl_class_def_create() -> *mut ClassDef {
    let mut x: *mut ClassDef =
        malloc(::core::mem::size_of::<ClassDef>() as usize) as *mut ClassDef;
    otl_class_def_init(x);
    return x;
}
#[inline]
pub(crate) unsafe extern "C" fn otl_class_def_copy_replace(mut dst: *mut ClassDef, src: ClassDef) {
    otl_class_def_dispose(dst);
    otl_class_def_copy(dst, &raw const src);
}
#[inline]
pub(crate) unsafe extern "C" fn otl_class_def_move(mut dst: *mut ClassDef, mut src: *mut ClassDef) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ClassDef>() as usize,
    );
    otl_class_def_init(src);
}
unsafe extern "C" fn grow_classdef(mut cd: *mut ClassDef, mut n: u32) {
    if n == 0 {
        return;
    }
    if n > (*cd).capacity {
        if (*cd).capacity == 0 {
            (*cd).capacity = 0x10 as u32;
        }
        while n > (*cd).capacity {
            (*cd).capacity = (*cd)
                .capacity
                .wrapping_add((*cd).capacity >> 1 as ::core::ffi::c_int & 0xffffff as u32);
        }
        (*cd).glyphs = __caryll_reallocate(
            (*cd).glyphs as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<GlyphHandle>() as usize)
                .wrapping_mul((*cd).capacity as usize),
            21 as ::core::ffi::c_ulong,
        ) as *mut GlyphHandle;
        (*cd).classes = __caryll_reallocate(
            (*cd).classes as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<GlyphClass>() as usize)
                .wrapping_mul((*cd).capacity as usize),
            22 as ::core::ffi::c_ulong,
        ) as *mut GlyphClass;
    }
}
pub(crate) unsafe extern "C" fn push_class_def(
    mut cd: *mut ClassDef,
    mut h: GlyphHandle,
    mut cls: GlyphClass,
) {
    (*cd).numGlyphs =
        ((*cd).numGlyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    grow_classdef(cd, (*cd).numGlyphs as u32);
    *(*cd)
        .glyphs
        .offset(((*cd).numGlyphs as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) = h;
    *(*cd)
        .classes
        .offset(((*cd).numGlyphs as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) = cls;
    if cls as ::core::ffi::c_int > (*cd).maxclass as ::core::ffi::c_int {
        (*cd).maxclass = cls;
    }
}
unsafe extern "C" fn by_cov_index(
    mut a: *mut CoverageEntry,
    mut b: *mut CoverageEntry,
) -> ::core::ffi::c_int {
    return (*a).covIndex - (*b).covIndex;
}
pub(crate) unsafe extern "C" fn read_class_def(
    mut data: *const u8,
    mut table_length: u32,
    mut offset: u32,
) -> *mut ClassDef {
    let mut cd: *mut ClassDef = otl_class_def_create();
    if table_length < offset.wrapping_add(4 as u32) {
        return cd;
    }
    let mut format: u16 = read_16u(data.offset(offset as isize));
    if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && table_length >= offset.wrapping_add(6 as u32)
    {
        let mut start_gid: GlyphId = read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize),
        ) as GlyphId;
        let mut count: GlyphId = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize),
        ) as GlyphId;
        if count as ::core::ffi::c_int != 0
            && table_length
                >= offset.wrapping_add(6 as u32).wrapping_add(
                    (count as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                )
        {
            let mut j: GlyphId = 0 as GlyphId;
            while (j as ::core::ffi::c_int) < count as ::core::ffi::c_int {
                push_class_def(
                    cd,
                    handle_from_index(
                        (start_gid as ::core::ffi::c_int + j as ::core::ffi::c_int) as GlyphId,
                    ) as GlyphHandle,
                    read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize),
                    ) as GlyphClass,
                );
                j = j.wrapping_add(1);
            }
            return cd;
        }
    } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        let mut range_count: u16 = read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize),
        );
        if table_length
            < offset.wrapping_add(4 as u32).wrapping_add(
                (range_count as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as u32,
            )
        {
            return cd;
        }
        let mut hash: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
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
            let mut cls: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize),
            );
            let mut k: ::core::ffi::c_int = start as ::core::ffi::c_int;
            while k <= end as ::core::ffi::c_int {
                let mut item: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
                let mut _hf_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i: ::core::ffi::c_uint = 0;
                let mut _hj_j: ::core::ffi::c_uint = 0;
                let mut _hj_k: ::core::ffi::c_uint = 0;
                let mut _hj_key: *const ::core::ffi::c_uchar =
                    &raw mut k as *const ::core::ffi::c_uchar;
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
                let mut current_block_61: u64;
                match _hj_k {
                    11 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_61 = 14624767404657188009;
                    }
                    10 => {
                        current_block_61 = 14624767404657188009;
                    }
                    9 => {
                        current_block_61 = 6840229733941229725;
                    }
                    8 => {
                        current_block_61 = 3094035989046081330;
                    }
                    7 => {
                        current_block_61 = 1184070620194684684;
                    }
                    6 => {
                        current_block_61 = 1362975655473755164;
                    }
                    5 => {
                        current_block_61 = 15703723867703157559;
                    }
                    4 => {
                        current_block_61 = 483194043190260627;
                    }
                    3 => {
                        current_block_61 = 9392992341002218192;
                    }
                    2 => {
                        current_block_61 = 14840068175916424037;
                    }
                    1 => {
                        current_block_61 = 2003362535987825465;
                    }
                    _ => {
                        current_block_61 = 9859671972921157070;
                    }
                }
                match current_block_61 {
                    14624767404657188009 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_61 = 6840229733941229725;
                    }
                    _ => {}
                }
                match current_block_61 {
                    6840229733941229725 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_61 = 3094035989046081330;
                    }
                    _ => {}
                }
                match current_block_61 {
                    3094035989046081330 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_61 = 1184070620194684684;
                    }
                    _ => {}
                }
                match current_block_61 {
                    1184070620194684684 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_61 = 1362975655473755164;
                    }
                    _ => {}
                }
                match current_block_61 {
                    1362975655473755164 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_61 = 15703723867703157559;
                    }
                    _ => {}
                }
                match current_block_61 {
                    15703723867703157559 => {
                        _hj_j = _hj_j
                            .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_61 = 483194043190260627;
                    }
                    _ => {}
                }
                match current_block_61 {
                    483194043190260627 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_61 = 9392992341002218192;
                    }
                    _ => {}
                }
                match current_block_61 {
                    9392992341002218192 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_61 = 14840068175916424037;
                    }
                    _ => {}
                }
                match current_block_61 {
                    14840068175916424037 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_61 = 2003362535987825465;
                    }
                    _ => {}
                }
                match current_block_61 {
                    2003362535987825465 => {
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
                                    &raw mut k as *const ::core::ffi::c_void,
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
                        70 as ::core::ffi::c_ulong,
                    ) as *mut CoverageEntry;
                    (*item).gid = k;
                    (*item).covIndex = cls as ::core::ffi::c_int;
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
                    let mut current_block_178: u64;
                    match _hj_k_0 {
                        11 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_178 = 259632281800862467;
                        }
                        10 => {
                            current_block_178 = 259632281800862467;
                        }
                        9 => {
                            current_block_178 = 9700417935590582947;
                        }
                        8 => {
                            current_block_178 = 4615042499405270376;
                        }
                        7 => {
                            current_block_178 = 528409004564197988;
                        }
                        6 => {
                            current_block_178 = 17134070298434916800;
                        }
                        5 => {
                            current_block_178 = 17671724849416320902;
                        }
                        4 => {
                            current_block_178 = 6734225487606566770;
                        }
                        3 => {
                            current_block_178 = 1276636934039996460;
                        }
                        2 => {
                            current_block_178 = 4347121995186969965;
                        }
                        1 => {
                            current_block_178 = 3125663975110725678;
                        }
                        _ => {
                            current_block_178 = 15993708482136914563;
                        }
                    }
                    match current_block_178 {
                        259632281800862467 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_178 = 9700417935590582947;
                        }
                        _ => {}
                    }
                    match current_block_178 {
                        9700417935590582947 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_178 = 4615042499405270376;
                        }
                        _ => {}
                    }
                    match current_block_178 {
                        4615042499405270376 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_178 = 528409004564197988;
                        }
                        _ => {}
                    }
                    match current_block_178 {
                        528409004564197988 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_178 = 17134070298434916800;
                        }
                        _ => {}
                    }
                    match current_block_178 {
                        17134070298434916800 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_178 = 17671724849416320902;
                        }
                        _ => {}
                    }
                    match current_block_178 {
                        17671724849416320902 => {
                            _hj_j_0 = _hj_j_0
                                .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_178 = 6734225487606566770;
                        }
                        _ => {}
                    }
                    match current_block_178 {
                        6734225487606566770 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_178 = 1276636934039996460;
                        }
                        _ => {}
                    }
                    match current_block_178 {
                        1276636934039996460 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_178 = 4347121995186969965;
                        }
                        _ => {}
                    }
                    match current_block_178 {
                        4347121995186969965 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_178 = 3125663975110725678;
                        }
                        _ => {}
                    }
                    match current_block_178 {
                        3125663975110725678 => {
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
                k += 1;
            }
            j_0 = j_0.wrapping_add(1);
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
                                }) as *mut UtHashHandle;
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
                                }) as *mut UtHashHandle;
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
                        as *mut CoverageEntry as *mut CoverageEntry;
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
            push_class_def(
                cd,
                handle_from_index((*e).gid as GlyphId)
                    as GlyphHandle,
                (*e).covIndex as GlyphClass,
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
                    let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*hash).hh.tbl).hho)
                        as *mut UtHashHandle))
                        .next;
                    *fresh2 = (*_hd_hh_del).next;
                } else {
                    hash = (*_hd_hh_del).next as *mut CoverageEntry as *mut CoverageEntry;
                }
                if !(*_hd_hh_del).next.is_null() {
                    let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                        .offset((*(*hash).hh.tbl).hho)
                        as *mut UtHashHandle))
                        .prev;
                    *fresh3 = (*_hd_hh_del).prev;
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
        return cd;
    }
    return cd;
}
pub(crate) unsafe extern "C" fn expand_class_def(
    mut cov: *mut Coverage,
    mut ocd: *mut ClassDef,
) -> *mut ClassDef {
    let mut cd: *mut ClassDef = otl_class_def_create();
    let mut hash: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < (*ocd).numGlyphs as ::core::ffi::c_int {
        let mut gid: ::core::ffi::c_int =
            (*(*ocd).glyphs.offset(j as isize)).index as ::core::ffi::c_int;
        let mut cid: ::core::ffi::c_int = *(*ocd).classes.offset(j as isize) as ::core::ffi::c_int;
        let mut item: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
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
        _hf_hashv = _hf_hashv
            .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
        let mut current_block_50: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 16087778969039934814;
            }
            10 => {
                current_block_50 = 16087778969039934814;
            }
            9 => {
                current_block_50 = 10292493007307731822;
            }
            8 => {
                current_block_50 = 10302433234345805146;
            }
            7 => {
                current_block_50 = 16244383241275317836;
            }
            6 => {
                current_block_50 = 13227768978888491025;
            }
            5 => {
                current_block_50 = 9158226259752878651;
            }
            4 => {
                current_block_50 = 9123231104951310097;
            }
            3 => {
                current_block_50 = 1731694550759493079;
            }
            2 => {
                current_block_50 = 4117435718141322982;
            }
            1 => {
                current_block_50 = 5527539487107527803;
            }
            _ => {
                current_block_50 = 15004371738079956865;
            }
        }
        match current_block_50 {
            16087778969039934814 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 10292493007307731822;
            }
            _ => {}
        }
        match current_block_50 {
            10292493007307731822 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 10302433234345805146;
            }
            _ => {}
        }
        match current_block_50 {
            10302433234345805146 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 16244383241275317836;
            }
            _ => {}
        }
        match current_block_50 {
            16244383241275317836 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 13227768978888491025;
            }
            _ => {}
        }
        match current_block_50 {
            13227768978888491025 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 9158226259752878651;
            }
            _ => {}
        }
        match current_block_50 {
            9158226259752878651 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_50 = 9123231104951310097;
            }
            _ => {}
        }
        match current_block_50 {
            9123231104951310097 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 1731694550759493079;
            }
            _ => {}
        }
        match current_block_50 {
            1731694550759493079 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 4117435718141322982;
            }
            _ => {}
        }
        match current_block_50 {
            4117435718141322982 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 5527539487107527803;
            }
            _ => {}
        }
        match current_block_50 {
            5527539487107527803 => {
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
                        as *mut CoverageEntry as *mut CoverageEntry;
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
                98 as ::core::ffi::c_ulong,
            ) as *mut CoverageEntry;
            (*item).gid = gid;
            (*item).covIndex = cid;
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
            _ha_hashv = _ha_hashv
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_167: u64;
            match _hj_k_0 {
                11 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_167 = 4791883040014495190;
                }
                10 => {
                    current_block_167 = 4791883040014495190;
                }
                9 => {
                    current_block_167 = 16596537683533330970;
                }
                8 => {
                    current_block_167 = 2539907918075352607;
                }
                7 => {
                    current_block_167 = 8999001423505666322;
                }
                6 => {
                    current_block_167 = 3054031367330404916;
                }
                5 => {
                    current_block_167 = 3363184723163677175;
                }
                4 => {
                    current_block_167 = 6898467595640461394;
                }
                3 => {
                    current_block_167 = 992178513905314417;
                }
                2 => {
                    current_block_167 = 203274669843346842;
                }
                1 => {
                    current_block_167 = 3344015233207987996;
                }
                _ => {
                    current_block_167 = 18221534353613080499;
                }
            }
            match current_block_167 {
                4791883040014495190 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_167 = 16596537683533330970;
                }
                _ => {}
            }
            match current_block_167 {
                16596537683533330970 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_167 = 2539907918075352607;
                }
                _ => {}
            }
            match current_block_167 {
                2539907918075352607 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_167 = 8999001423505666322;
                }
                _ => {}
            }
            match current_block_167 {
                8999001423505666322 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_167 = 3054031367330404916;
                }
                _ => {}
            }
            match current_block_167 {
                3054031367330404916 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_167 = 3363184723163677175;
                }
                _ => {}
            }
            match current_block_167 {
                3363184723163677175 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_167 = 6898467595640461394;
                }
                _ => {}
            }
            match current_block_167 {
                6898467595640461394 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_167 = 992178513905314417;
                }
                _ => {}
            }
            match current_block_167 {
                992178513905314417 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_167 = 203274669843346842;
                }
                _ => {}
            }
            match current_block_167 {
                203274669843346842 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_167 = 3344015233207987996;
                }
                _ => {}
            }
            match current_block_167 {
                3344015233207987996 => {
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
            (*item).hh.key =
                &raw mut (*item).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            (*item).hh.keylen = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            if hash.is_null() {
                (*item).hh.next = NULL;
                (*item).hh.prev = NULL;
                (*item).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                    as *mut UtHashTable as *mut UtHashTable;
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
                    (*(*item).hh.tbl).hho = (&raw mut (*item).hh as *mut ::core::ffi::c_char)
                        .offset_from(item as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as isize;
                    (*(*item).hh.tbl).buckets = malloc(
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    ) as *mut UtHashBucket;
                    (*(*item).hh.tbl).signature = HASH_SIGNATURE as u32;
                    if (*(*item).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
                let mut _he_thh: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _he_hh_nxt: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _he_new_buckets: *mut UtHashBucket =
                    ::core::ptr::null_mut::<UtHashBucket>();
                let mut _he_newbkt: *mut UtHashBucket = ::core::ptr::null_mut::<UtHashBucket>();
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
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
                        _he_thh = (*(*(*item).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UtHashHandle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*item).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                            (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                            if (*_he_newbkt).count > (*(*item).hh.tbl).ideal_chain_maxlen {
                                (*(*item).hh.tbl).nonideal_items =
                                    (*(*item).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt).expand_mult = (*_he_newbkt)
                                    .count
                                    .wrapping_div((*(*item).hh.tbl).ideal_chain_maxlen);
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
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < (*cov).numGlyphs as ::core::ffi::c_int {
        let mut gid_0: ::core::ffi::c_int =
            (*(*cov).glyphs.offset(j_0 as isize)).index as ::core::ffi::c_int;
        let mut item_0: *mut CoverageEntry = ::core::ptr::null_mut::<CoverageEntry>();
        let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
        let mut _hj_i_1: ::core::ffi::c_uint = 0;
        let mut _hj_j_1: ::core::ffi::c_uint = 0;
        let mut _hj_k_1: ::core::ffi::c_uint = 0;
        let mut _hj_key_1: *const ::core::ffi::c_uchar =
            &raw mut gid_0 as *const ::core::ffi::c_uchar;
        _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
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
        _hf_hashv_0 = _hf_hashv_0
            .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
        let mut current_block_359: u64;
        match _hj_k_1 {
            11 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_359 = 664873979551263117;
            }
            10 => {
                current_block_359 = 664873979551263117;
            }
            9 => {
                current_block_359 = 1478714033163649479;
            }
            8 => {
                current_block_359 = 465578627041062056;
            }
            7 => {
                current_block_359 = 777374825036219002;
            }
            6 => {
                current_block_359 = 9857860022602914719;
            }
            5 => {
                current_block_359 = 4281461987913842530;
            }
            4 => {
                current_block_359 = 1220405448249094221;
            }
            3 => {
                current_block_359 = 4804411386846960007;
            }
            2 => {
                current_block_359 = 8318136191919969087;
            }
            1 => {
                current_block_359 = 8265069324821677810;
            }
            _ => {
                current_block_359 = 7822482261646769021;
            }
        }
        match current_block_359 {
            664873979551263117 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_359 = 1478714033163649479;
            }
            _ => {}
        }
        match current_block_359 {
            1478714033163649479 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_359 = 465578627041062056;
            }
            _ => {}
        }
        match current_block_359 {
            465578627041062056 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_359 = 777374825036219002;
            }
            _ => {}
        }
        match current_block_359 {
            777374825036219002 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_359 = 9857860022602914719;
            }
            _ => {}
        }
        match current_block_359 {
            9857860022602914719 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_359 = 4281461987913842530;
            }
            _ => {}
        }
        match current_block_359 {
            4281461987913842530 => {
                _hj_j_1 =
                    _hj_j_1
                        .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_359 = 1220405448249094221;
            }
            _ => {}
        }
        match current_block_359 {
            1220405448249094221 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_359 = 4804411386846960007;
            }
            _ => {}
        }
        match current_block_359 {
            4804411386846960007 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_359 = 8318136191919969087;
            }
            _ => {}
        }
        match current_block_359 {
            8318136191919969087 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_359 = 8265069324821677810;
            }
            _ => {}
        }
        match current_block_359 {
            8265069324821677810 => {
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
        item_0 = ::core::ptr::null_mut::<CoverageEntry>();
        if !hash.is_null() {
            let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
            _hf_bkt_0 = _hf_hashv_0
                & (*(*hash).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if !(*(*(*hash).hh.tbl).buckets.offset(_hf_bkt_0 as isize))
                    .hh_head
                    .is_null()
                {
                    item_0 = ((*(*(*hash).hh.tbl).buckets.offset(_hf_bkt_0 as isize)).hh_head
                        as *mut ::core::ffi::c_char)
                        .offset(-(*(*hash).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut CoverageEntry as *mut CoverageEntry;
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
                            &raw mut gid_0 as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*item_0).hh.hh_next.is_null() {
                        item_0 = ((*item_0).hh.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*hash).hh.tbl).hho)
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
                109 as ::core::ffi::c_ulong,
            ) as *mut CoverageEntry;
            (*item_0).gid = gid_0;
            (*item_0).covIndex = 0 as ::core::ffi::c_int;
            let mut _ha_hashv_0: ::core::ffi::c_uint = 0;
            let mut _hj_i_2: ::core::ffi::c_uint = 0;
            let mut _hj_j_2: ::core::ffi::c_uint = 0;
            let mut _hj_k_2: ::core::ffi::c_uint = 0;
            let mut _hj_key_2: *const ::core::ffi::c_uchar =
                &raw mut (*item_0).gid as *const ::core::ffi::c_uchar;
            _ha_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_2 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_2 = _hj_j_2;
            _hj_k_2 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            while _hj_k_2 >= 12 as ::core::ffi::c_uint {
                _hj_i_2 = _hj_i_2.wrapping_add(
                    (*_hj_key_2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
                    (*_hj_key_2.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
                    (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_476: u64;
            match _hj_k_2 {
                11 => {
                    _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                        (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_476 = 18086183351333264011;
                }
                10 => {
                    current_block_476 = 18086183351333264011;
                }
                9 => {
                    current_block_476 = 6581424524861903918;
                }
                8 => {
                    current_block_476 = 4831707637779038646;
                }
                7 => {
                    current_block_476 = 2289199519686980607;
                }
                6 => {
                    current_block_476 = 8613016784100927070;
                }
                5 => {
                    current_block_476 = 9372486881324206913;
                }
                4 => {
                    current_block_476 = 5111587104494200972;
                }
                3 => {
                    current_block_476 = 12956173469424200534;
                }
                2 => {
                    current_block_476 = 14308946614392844471;
                }
                1 => {
                    current_block_476 = 15967253824745734317;
                }
                _ => {
                    current_block_476 = 11989315111553324117;
                }
            }
            match current_block_476 {
                18086183351333264011 => {
                    _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                        (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_476 = 6581424524861903918;
                }
                _ => {}
            }
            match current_block_476 {
                6581424524861903918 => {
                    _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                        (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_476 = 4831707637779038646;
                }
                _ => {}
            }
            match current_block_476 {
                4831707637779038646 => {
                    _hj_j_2 = _hj_j_2.wrapping_add(
                        (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_476 = 2289199519686980607;
                }
                _ => {}
            }
            match current_block_476 {
                2289199519686980607 => {
                    _hj_j_2 = _hj_j_2.wrapping_add(
                        (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_476 = 8613016784100927070;
                }
                _ => {}
            }
            match current_block_476 {
                8613016784100927070 => {
                    _hj_j_2 = _hj_j_2.wrapping_add(
                        (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_476 = 9372486881324206913;
                }
                _ => {}
            }
            match current_block_476 {
                9372486881324206913 => {
                    _hj_j_2 = _hj_j_2
                        .wrapping_add(*_hj_key_2.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_476 = 5111587104494200972;
                }
                _ => {}
            }
            match current_block_476 {
                5111587104494200972 => {
                    _hj_i_2 = _hj_i_2.wrapping_add(
                        (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_476 = 12956173469424200534;
                }
                _ => {}
            }
            match current_block_476 {
                12956173469424200534 => {
                    _hj_i_2 = _hj_i_2.wrapping_add(
                        (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_476 = 14308946614392844471;
                }
                _ => {}
            }
            match current_block_476 {
                14308946614392844471 => {
                    _hj_i_2 = _hj_i_2.wrapping_add(
                        (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_476 = 15967253824745734317;
                }
                _ => {}
            }
            match current_block_476 {
                15967253824745734317 => {
                    _hj_i_2 = _hj_i_2
                        .wrapping_add(*_hj_key_2.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
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
            (*item_0).hh.key =
                &raw mut (*item_0).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            (*item_0).hh.keylen =
                ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            if hash.is_null() {
                (*item_0).hh.next = NULL;
                (*item_0).hh.prev = NULL;
                (*item_0).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                    as *mut UtHashTable as *mut UtHashTable;
                if (*item_0).hh.tbl.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*item_0).hh.tbl as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<UtHashTable>() as usize,
                    );
                    (*(*item_0).hh.tbl).tail = &raw mut (*item_0).hh as *mut UtHashHandle;
                    (*(*item_0).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                    (*(*item_0).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                    (*(*item_0).hh.tbl).hho = (&raw mut (*item_0).hh as *mut ::core::ffi::c_char)
                        .offset_from(item_0 as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as isize;
                    (*(*item_0).hh.tbl).buckets = malloc(
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    ) as *mut UtHashBucket;
                    (*(*item_0).hh.tbl).signature = HASH_SIGNATURE as u32;
                    if (*(*item_0).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*item_0).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        );
                    }
                }
                hash = item_0;
            } else {
                (*item_0).hh.tbl = (*hash).hh.tbl;
                (*item_0).hh.next = NULL;
                (*item_0).hh.prev = ((*(*hash).hh.tbl).tail as *mut ::core::ffi::c_char)
                    .offset(-(*(*hash).hh.tbl).hho)
                    as *mut ::core::ffi::c_void;
                (*(*(*hash).hh.tbl).tail).next = item_0 as *mut ::core::ffi::c_void;
                (*(*hash).hh.tbl).tail = &raw mut (*item_0).hh as *mut UtHashHandle;
            }
            let mut _ha_bkt_0: ::core::ffi::c_uint = 0;
            (*(*hash).hh.tbl).num_items = (*(*hash).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt_0 = _ha_hashv_0
                & (*(*hash).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head_0: *mut UtHashBucket =
                (*(*hash).hh.tbl).buckets.offset(_ha_bkt_0 as isize) as *mut UtHashBucket;
            (*_ha_head_0).count = (*_ha_head_0).count.wrapping_add(1);
            (*item_0).hh.hh_next = (*_ha_head_0).hh_head as *mut UtHashHandle;
            (*item_0).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
            if !(*_ha_head_0).hh_head.is_null() {
                (*(*_ha_head_0).hh_head).hh_prev = &raw mut (*item_0).hh as *mut UtHashHandle;
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
                let mut _he_thh_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _he_hh_nxt_0: *mut UtHashHandle =
                    ::core::ptr::null_mut::<UtHashHandle>();
                let mut _he_new_buckets_0: *mut UtHashBucket =
                    ::core::ptr::null_mut::<UtHashBucket>();
                let mut _he_newbkt_0: *mut UtHashBucket =
                    ::core::ptr::null_mut::<UtHashBucket>();
                _he_new_buckets_0 = malloc(
                    (2 as usize)
                        .wrapping_mul((*(*item_0).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                if _he_new_buckets_0.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        _he_new_buckets_0 as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (2 as usize)
                            .wrapping_mul((*(*item_0).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    );
                    (*(*item_0).hh.tbl).ideal_chain_maxlen = ((*(*item_0).hh.tbl).num_items
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
                        _he_thh_0 = (*(*(*item_0).hh.tbl).buckets.offset(_he_bkt_i_0 as isize))
                            .hh_head as *mut UtHashHandle;
                        while !_he_thh_0.is_null() {
                            _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                            _he_bkt_0 = (*_he_thh_0).hashv
                                & (*(*item_0).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt_0 =
                                _he_new_buckets_0.offset(_he_bkt_0 as isize) as *mut UtHashBucket;
                            (*_he_newbkt_0).count = (*_he_newbkt_0).count.wrapping_add(1);
                            if (*_he_newbkt_0).count > (*(*item_0).hh.tbl).ideal_chain_maxlen {
                                (*(*item_0).hh.tbl).nonideal_items =
                                    (*(*item_0).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt_0).expand_mult = (*_he_newbkt_0)
                                    .count
                                    .wrapping_div((*(*item_0).hh.tbl).ideal_chain_maxlen);
                            }
                            (*_he_thh_0).hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                            (*_he_thh_0).hh_next = (*_he_newbkt_0).hh_head as *mut UtHashHandle;
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
                    (*(*item_0).hh.tbl).ineff_expands = if (*(*item_0).hh.tbl).nonideal_items
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
        j_0 = j_0.wrapping_add(1);
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
        push_class_def(
            cd,
            handle_from_index((*e).gid as GlyphId)
                as GlyphHandle,
            (*e).covIndex as GlyphClass,
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
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hash).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                hash = (*_hd_hh_del).next as *mut CoverageEntry as *mut CoverageEntry;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*hash).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
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
    otl_class_def_free(ocd);
    return cd;
}
pub(crate) unsafe extern "C" fn dump_class_def(mut cd: *const ClassDef) -> *mut JsonValue {
    let mut a: *mut JsonValue = json_object_new((*cd).numGlyphs as usize);
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
        json_object_push(
            a,
            (*(*cd).glyphs.offset(j as isize)).name as *const ::core::ffi::c_char,
            json_integer_new(*(*cd).classes.offset(j as isize) as i64),
        );
        j = j.wrapping_add(1);
    }
    return preserialize(a);
}
pub(crate) unsafe extern "C" fn parse_class_def(mut _cd: *const JsonValue) -> *mut ClassDef {
    if _cd.is_null()
        || (*_cd).type_0 != JsonType::Object
    {
        return ::core::ptr::null_mut::<ClassDef>();
    }
    let mut cd: *mut ClassDef = otl_class_def_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*_cd).u.object.length {
        let mut h: GlyphHandle =
            handle_from_name(sdsnewlen(
                (*(*_cd).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
                (*(*_cd).u.object.values.offset(j as isize)).name_length as usize,
            )) as GlyphHandle;
        let mut _cid: *mut JsonValue =
            (*(*_cd).u.object.values.offset(j as isize)).value as *mut JsonValue;
        let mut cls: GlyphClass = 0 as GlyphClass;
        if (*_cid).type_0 == JsonType::Integer
        {
            cls = (*_cid).u.integer as GlyphClass;
        } else if (*_cid).type_0 == JsonType::Double
        {
            cls = (*_cid).u.dbl as GlyphClass;
        }
        push_class_def(cd, h as GlyphHandle, cls);
        j = j.wrapping_add(1);
    }
    return cd;
}
unsafe extern "C" fn by_gid(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut ClassDefSortRecord)).gid as ::core::ffi::c_int
        - (*(b as *mut ClassDefSortRecord)).gid as ::core::ffi::c_int;
}
pub(crate) unsafe extern "C" fn build_class_def(mut cd: *const ClassDef) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 2 as u16);
    if (*cd).numGlyphs == 0 {
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    let mut r: *mut ClassDefSortRecord = ::core::ptr::null_mut::<ClassDefSortRecord>();
    r = __caryll_allocate_clean(
        (::core::mem::size_of::<ClassDefSortRecord>() as usize)
            .wrapping_mul((*cd).numGlyphs as usize),
        167 as ::core::ffi::c_ulong,
    ) as *mut ClassDefSortRecord;
    let mut jj: GlyphId = 0 as GlyphId;
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
        if *(*cd).classes.offset(j as isize) != 0 {
            (*r.offset(jj as isize)).gid = (*(*cd).glyphs.offset(j as isize)).index;
            (*r.offset(jj as isize)).cid = *(*cd).classes.offset(j as isize);
            jj = jj.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    if jj == 0 {
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<ClassDefSortRecord>();
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    qsort(
        r as *mut ::core::ffi::c_void,
        jj as usize,
        ::core::mem::size_of::<ClassDefSortRecord>() as usize,
        Some(
            by_gid
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut start_gid: GlyphId = (*r.offset(0 as ::core::ffi::c_int as isize)).gid;
    let mut end_gid: GlyphId = start_gid;
    let mut last_class: GlyphClass = (*r.offset(0 as ::core::ffi::c_int as isize)).cid;
    let mut n_ranges: GlyphId = 0 as GlyphId;
    let mut last_gid: GlyphId = start_gid;
    let mut ranges: *mut Buffer = bufnew();
    let mut j_0: GlyphId = 1 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        let mut current: GlyphId = (*r.offset(j_0 as isize)).gid;
        if !(current as ::core::ffi::c_int <= last_gid as ::core::ffi::c_int) {
            if current as ::core::ffi::c_int
                == end_gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                && (*r.offset(j_0 as isize)).cid as ::core::ffi::c_int
                    == last_class as ::core::ffi::c_int
            {
                end_gid = current;
            } else {
                bufwrite16b(ranges, start_gid as u16);
                bufwrite16b(ranges, end_gid as u16);
                bufwrite16b(ranges, last_class as u16);
                n_ranges = (n_ranges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
                end_gid = current;
                start_gid = end_gid;
                last_class = (*r.offset(j_0 as isize)).cid;
            }
            last_gid = current;
        }
        j_0 = j_0.wrapping_add(1);
    }
    bufwrite16b(ranges, start_gid as u16);
    bufwrite16b(ranges, end_gid as u16);
    bufwrite16b(ranges, last_class as u16);
    n_ranges = (n_ranges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    bufwrite16b(buf, n_ranges as u16);
    bufwrite_bufdel(buf, ranges);
    free(r as *mut ::core::ffi::c_void);
    r = ::core::ptr::null_mut::<ClassDefSortRecord>();
    return buf;
}
pub(crate) unsafe extern "C" fn shrink_class_def(mut cd: *mut ClassDef) {
    let mut k: GlyphId = 0 as GlyphId;
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
        if !(*(*cd).glyphs.offset(j as isize)).name.is_null() {
            *(*cd).glyphs.offset(k as isize) = *(*cd).glyphs.offset(j as isize);
            *(*cd).classes.offset(k as isize) = *(*cd).classes.offset(j as isize);
            k = k.wrapping_add(1);
        } else {
            otfcc_handle_dispose(
                (*cd).glyphs.offset(j as isize) as *mut Handle,
            );
        }
        j = j.wrapping_add(1);
    }
    (*cd).numGlyphs = k;
}
pub static OTL_I_CLASS_DEF: IClassDef = {
    IClassDef {
        init: Some(otl_class_def_init as unsafe extern "C" fn(*mut ClassDef) -> ()),
        copy: Some(
            otl_class_def_copy as unsafe extern "C" fn(*mut ClassDef, *const ClassDef) -> (),
        ),
        move_0: Some(
            otl_class_def_move as unsafe extern "C" fn(*mut ClassDef, *mut ClassDef) -> (),
        ),
        dispose: Some(otl_class_def_dispose as unsafe extern "C" fn(*mut ClassDef) -> ()),
        replace: Some(
            otl_class_def_replace as unsafe extern "C" fn(*mut ClassDef, ClassDef) -> (),
        ),
        copyReplace: Some(
            otl_class_def_copy_replace as unsafe extern "C" fn(*mut ClassDef, ClassDef) -> (),
        ),
        create: Some(otl_class_def_create),
        free: Some(otl_class_def_free as unsafe extern "C" fn(*mut ClassDef) -> ()),
        push: Some(
            push_class_def
                as unsafe extern "C" fn(*mut ClassDef, GlyphHandle, GlyphClass) -> (),
        ),
        read: Some(
            read_class_def
                as unsafe extern "C" fn(*const u8, u32, u32) -> *mut ClassDef,
        ),
        expand: Some(
            expand_class_def
                as unsafe extern "C" fn(*mut Coverage, *mut ClassDef) -> *mut ClassDef,
        ),
        dump: Some(dump_class_def as unsafe extern "C" fn(*const ClassDef) -> *mut JsonValue),
        parse: Some(parse_class_def as unsafe extern "C" fn(*const JsonValue) -> *mut ClassDef),
        build: Some(
            build_class_def as unsafe extern "C" fn(*const ClassDef) -> *mut Buffer,
        ),
        shrink: Some(shrink_class_def as unsafe extern "C" fn(*mut ClassDef) -> ()),
    }
};
