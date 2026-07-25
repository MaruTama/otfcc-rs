use libc::{exit, free, malloc, memcmp, memcpy, memset, qsort};
extern "C" {
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite_bufdel(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
}

use crate::table::otl::coverage::{coverage_entry, otl_Coverage};
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_dispose, otfcc_Handle, otfcc_GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::binio::{read_16u};
use crate::support::buffer::{caryll_Buffer};
use crate::support::primitives::{glyphclass_t, glyphid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_double, json_integer, json_object, json_pre_serialized, json_value};
use crate::support::{NULL};
use crate::support::glyph_order::{glyph_handle};
use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_ClassDef {
    pub numGlyphs: glyphid_t,
    pub capacity: u32,
    pub maxclass: glyphclass_t,
    pub glyphs: *mut otfcc_GlyphHandle,
    pub classes: *mut glyphclass_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __otfcc_IClassDef {
    pub init: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_ClassDef, *const otl_ClassDef) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_ClassDef, *mut otl_ClassDef) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_ClassDef>,
    pub free: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut otl_ClassDef, otfcc_GlyphHandle, glyphclass_t) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_ClassDef>,
    pub expand:
        Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_ClassDef) -> *mut otl_ClassDef>,
    pub dump: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_ClassDef>,
    pub build: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct classdef_sortrecord {
    pub gid: glyphid_t,
    pub cid: glyphclass_t,
}
#[inline]
unsafe extern "C" fn disposeClassDef(mut cd: *mut otl_ClassDef) {
    if !(*cd).glyphs.is_null() {
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
            otfcc_Handle_dispose(
                (*cd).glyphs.offset(j as isize) as *mut otfcc_Handle,
            );
            j = j.wrapping_add(1);
        }
        free((*cd).glyphs as *mut ::core::ffi::c_void);
        (*cd).glyphs = ::core::ptr::null_mut::<otfcc_GlyphHandle>();
    }
    free((*cd).classes as *mut ::core::ffi::c_void);
    (*cd).classes = ::core::ptr::null_mut::<glyphclass_t>();
}
#[inline]
pub(crate) unsafe extern "C" fn otl_ClassDef_replace(mut dst: *mut otl_ClassDef, src: otl_ClassDef) {
    otl_ClassDef_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_ClassDef>() as usize,
    );
}
#[inline]
pub(crate) unsafe extern "C" fn otl_ClassDef_copy(mut dst: *mut otl_ClassDef, mut src: *const otl_ClassDef) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_ClassDef>() as usize,
    );
}
#[inline]
pub(crate) unsafe extern "C" fn otl_ClassDef_free(mut x: *mut otl_ClassDef) {
    if x.is_null() {
        return;
    }
    otl_ClassDef_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
pub(crate) unsafe extern "C" fn otl_ClassDef_dispose(mut x: *mut otl_ClassDef) {
    disposeClassDef(x);
}
#[inline]
pub(crate) unsafe extern "C" fn otl_ClassDef_init(mut x: *mut otl_ClassDef) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<otl_ClassDef>() as usize,
    );
}
#[inline]
pub(crate) unsafe extern "C" fn otl_ClassDef_create() -> *mut otl_ClassDef {
    let mut x: *mut otl_ClassDef =
        malloc(::core::mem::size_of::<otl_ClassDef>() as usize) as *mut otl_ClassDef;
    otl_ClassDef_init(x);
    return x;
}
#[inline]
pub(crate) unsafe extern "C" fn otl_ClassDef_copyReplace(mut dst: *mut otl_ClassDef, src: otl_ClassDef) {
    otl_ClassDef_dispose(dst);
    otl_ClassDef_copy(dst, &raw const src);
}
#[inline]
pub(crate) unsafe extern "C" fn otl_ClassDef_move(mut dst: *mut otl_ClassDef, mut src: *mut otl_ClassDef) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otl_ClassDef>() as usize,
    );
    otl_ClassDef_init(src);
}
unsafe extern "C" fn growClassdef(mut cd: *mut otl_ClassDef, mut n: u32) {
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
            (::core::mem::size_of::<otfcc_GlyphHandle>() as usize)
                .wrapping_mul((*cd).capacity as usize),
            21 as ::core::ffi::c_ulong,
        ) as *mut otfcc_GlyphHandle;
        (*cd).classes = __caryll_reallocate(
            (*cd).classes as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<glyphclass_t>() as usize)
                .wrapping_mul((*cd).capacity as usize),
            22 as ::core::ffi::c_ulong,
        ) as *mut glyphclass_t;
    }
}
pub(crate) unsafe extern "C" fn pushClassDef(
    mut cd: *mut otl_ClassDef,
    mut h: otfcc_GlyphHandle,
    mut cls: glyphclass_t,
) {
    (*cd).numGlyphs =
        ((*cd).numGlyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
    growClassdef(cd, (*cd).numGlyphs as u32);
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
unsafe extern "C" fn by_covIndex(
    mut a: *mut coverage_entry,
    mut b: *mut coverage_entry,
) -> ::core::ffi::c_int {
    return (*a).covIndex - (*b).covIndex;
}
pub(crate) unsafe extern "C" fn readClassDef(
    mut data: *const u8,
    mut tableLength: u32,
    mut offset: u32,
) -> *mut otl_ClassDef {
    let mut cd: *mut otl_ClassDef = otl_ClassDef_create();
    if tableLength < offset.wrapping_add(4 as u32) {
        return cd;
    }
    let mut format: u16 = read_16u(data.offset(offset as isize));
    if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && tableLength >= offset.wrapping_add(6 as u32)
    {
        let mut startGID: glyphid_t = read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize),
        ) as glyphid_t;
        let mut count: glyphid_t = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize),
        ) as glyphid_t;
        if count as ::core::ffi::c_int != 0
            && tableLength
                >= offset.wrapping_add(6 as u32).wrapping_add(
                    (count as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                )
        {
            let mut j: glyphid_t = 0 as glyphid_t;
            while (j as ::core::ffi::c_int) < count as ::core::ffi::c_int {
                pushClassDef(
                    cd,
                    handle_fromIndex(
                        (startGID as ::core::ffi::c_int + j as ::core::ffi::c_int) as glyphid_t,
                    ) as otfcc_GlyphHandle,
                    read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize),
                    ) as glyphclass_t,
                );
                j = j.wrapping_add(1);
            }
            return cd;
        }
    } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        let mut rangeCount: u16 = read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize),
        );
        if tableLength
            < offset.wrapping_add(4 as u32).wrapping_add(
                (rangeCount as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as u32,
            )
        {
            return cd;
        }
        let mut hash: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
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
            let mut cls: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize),
            );
            let mut k: ::core::ffi::c_int = start as ::core::ffi::c_int;
            while k <= end as ::core::ffi::c_int {
                let mut item: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
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
                        70 as ::core::ffi::c_ulong,
                    ) as *mut coverage_entry;
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
                                }) as *mut UT_hash_handle;
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
                                }) as *mut UT_hash_handle;
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
                        as *mut coverage_entry as *mut coverage_entry;
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
            pushClassDef(
                cd,
                handle_fromIndex((*e).gid as glyphid_t)
                    as otfcc_GlyphHandle,
                (*e).covIndex as glyphclass_t,
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
                    let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*hash).hh.tbl).hho)
                        as *mut UT_hash_handle))
                        .next;
                    *fresh2 = (*_hd_hh_del).next;
                } else {
                    hash = (*_hd_hh_del).next as *mut coverage_entry as *mut coverage_entry;
                }
                if !(*_hd_hh_del).next.is_null() {
                    let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                        .offset((*(*hash).hh.tbl).hho)
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh3 = (*_hd_hh_del).prev;
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
        return cd;
    }
    return cd;
}
pub(crate) unsafe extern "C" fn expandClassDef(
    mut cov: *mut otl_Coverage,
    mut ocd: *mut otl_ClassDef,
) -> *mut otl_ClassDef {
    let mut cd: *mut otl_ClassDef = otl_ClassDef_create();
    let mut hash: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*ocd).numGlyphs as ::core::ffi::c_int {
        let mut gid: ::core::ffi::c_int =
            (*(*ocd).glyphs.offset(j as isize)).index as ::core::ffi::c_int;
        let mut cid: ::core::ffi::c_int = *(*ocd).classes.offset(j as isize) as ::core::ffi::c_int;
        let mut item: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
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
                        as *mut coverage_entry as *mut coverage_entry;
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
                98 as ::core::ffi::c_ulong,
            ) as *mut coverage_entry;
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
                (*item).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                    as *mut UT_hash_table as *mut UT_hash_table;
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
                    (*(*item).hh.tbl).hho = (&raw mut (*item).hh as *mut ::core::ffi::c_char)
                        .offset_from(item as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as isize;
                    (*(*item).hh.tbl).buckets = malloc(
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    ) as *mut UT_hash_bucket;
                    (*(*item).hh.tbl).signature = HASH_SIGNATURE as u32;
                    if (*(*item).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
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
                let mut _he_thh: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_hh_nxt: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_new_buckets: *mut UT_hash_bucket =
                    ::core::ptr::null_mut::<UT_hash_bucket>();
                let mut _he_newbkt: *mut UT_hash_bucket = ::core::ptr::null_mut::<UT_hash_bucket>();
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
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
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
                            as *mut UT_hash_handle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*item).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                            (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                            if (*_he_newbkt).count > (*(*item).hh.tbl).ideal_chain_maxlen {
                                (*(*item).hh.tbl).nonideal_items =
                                    (*(*item).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt).expand_mult = (*_he_newbkt)
                                    .count
                                    .wrapping_div((*(*item).hh.tbl).ideal_chain_maxlen);
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
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as ::core::ffi::c_int) < (*cov).numGlyphs as ::core::ffi::c_int {
        let mut gid_0: ::core::ffi::c_int =
            (*(*cov).glyphs.offset(j_0 as isize)).index as ::core::ffi::c_int;
        let mut item_0: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
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
        item_0 = ::core::ptr::null_mut::<coverage_entry>();
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
                        as *mut coverage_entry as *mut coverage_entry;
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
                109 as ::core::ffi::c_ulong,
            ) as *mut coverage_entry;
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
                (*item_0).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                    as *mut UT_hash_table as *mut UT_hash_table;
                if (*item_0).hh.tbl.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*item_0).hh.tbl as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<UT_hash_table>() as usize,
                    );
                    (*(*item_0).hh.tbl).tail = &raw mut (*item_0).hh as *mut UT_hash_handle;
                    (*(*item_0).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                    (*(*item_0).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                    (*(*item_0).hh.tbl).hho = (&raw mut (*item_0).hh as *mut ::core::ffi::c_char)
                        .offset_from(item_0 as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as isize;
                    (*(*item_0).hh.tbl).buckets = malloc(
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    ) as *mut UT_hash_bucket;
                    (*(*item_0).hh.tbl).signature = HASH_SIGNATURE as u32;
                    if (*(*item_0).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*item_0).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
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
                (*(*hash).hh.tbl).tail = &raw mut (*item_0).hh as *mut UT_hash_handle;
            }
            let mut _ha_bkt_0: ::core::ffi::c_uint = 0;
            (*(*hash).hh.tbl).num_items = (*(*hash).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt_0 = _ha_hashv_0
                & (*(*hash).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head_0: *mut UT_hash_bucket =
                (*(*hash).hh.tbl).buckets.offset(_ha_bkt_0 as isize) as *mut UT_hash_bucket;
            (*_ha_head_0).count = (*_ha_head_0).count.wrapping_add(1);
            (*item_0).hh.hh_next = (*_ha_head_0).hh_head as *mut UT_hash_handle;
            (*item_0).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
            if !(*_ha_head_0).hh_head.is_null() {
                (*(*_ha_head_0).hh_head).hh_prev = &raw mut (*item_0).hh as *mut UT_hash_handle;
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
                let mut _he_thh_0: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_hh_nxt_0: *mut UT_hash_handle =
                    ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_new_buckets_0: *mut UT_hash_bucket =
                    ::core::ptr::null_mut::<UT_hash_bucket>();
                let mut _he_newbkt_0: *mut UT_hash_bucket =
                    ::core::ptr::null_mut::<UT_hash_bucket>();
                _he_new_buckets_0 = malloc(
                    (2 as usize)
                        .wrapping_mul((*(*item_0).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                if _he_new_buckets_0.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        _he_new_buckets_0 as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (2 as usize)
                            .wrapping_mul((*(*item_0).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
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
                            .hh_head as *mut UT_hash_handle;
                        while !_he_thh_0.is_null() {
                            _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                            _he_bkt_0 = (*_he_thh_0).hashv
                                & (*(*item_0).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt_0 =
                                _he_new_buckets_0.offset(_he_bkt_0 as isize) as *mut UT_hash_bucket;
                            (*_he_newbkt_0).count = (*_he_newbkt_0).count.wrapping_add(1);
                            if (*_he_newbkt_0).count > (*(*item_0).hh.tbl).ideal_chain_maxlen {
                                (*(*item_0).hh.tbl).nonideal_items =
                                    (*(*item_0).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt_0).expand_mult = (*_he_newbkt_0)
                                    .count
                                    .wrapping_div((*(*item_0).hh.tbl).ideal_chain_maxlen);
                            }
                            (*_he_thh_0).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                            (*_he_thh_0).hh_next = (*_he_newbkt_0).hh_head as *mut UT_hash_handle;
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
    let mut e: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
    let mut tmp: *mut coverage_entry = ::core::ptr::null_mut::<coverage_entry>();
    e = hash;
    tmp = (if !hash.is_null() {
        (*hash).hh.next
    } else {
        NULL
    }) as *mut coverage_entry as *mut coverage_entry;
    while !e.is_null() {
        pushClassDef(
            cd,
            handle_fromIndex((*e).gid as glyphid_t)
                as otfcc_GlyphHandle,
            (*e).covIndex as glyphclass_t,
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
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hash).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                hash = (*_hd_hh_del).next as *mut coverage_entry as *mut coverage_entry;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*hash).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
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
    otl_ClassDef_free(ocd);
    return cd;
}
pub(crate) unsafe extern "C" fn dumpClassDef(mut cd: *const otl_ClassDef) -> *mut json_value {
    let mut a: *mut json_value = json_object_new((*cd).numGlyphs as usize);
    let mut j: glyphid_t = 0 as glyphid_t;
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
pub(crate) unsafe extern "C" fn parseClassDef(mut _cd: *const json_value) -> *mut otl_ClassDef {
    if _cd.is_null()
        || (*_cd).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<otl_ClassDef>();
    }
    let mut cd: *mut otl_ClassDef = otl_ClassDef_create();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*_cd).u.object.length {
        let mut h: glyph_handle =
            handle_fromName(sdsnewlen(
                (*(*_cd).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
                (*(*_cd).u.object.values.offset(j as isize)).name_length as usize,
            )) as glyph_handle;
        let mut _cid: *mut json_value =
            (*(*_cd).u.object.values.offset(j as isize)).value as *mut json_value;
        let mut cls: glyphclass_t = 0 as glyphclass_t;
        if (*_cid).type_0 as ::core::ffi::c_uint
            == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            cls = (*_cid).u.integer as glyphclass_t;
        } else if (*_cid).type_0 as ::core::ffi::c_uint
            == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            cls = (*_cid).u.dbl as glyphclass_t;
        }
        pushClassDef(cd, h as otfcc_GlyphHandle, cls);
        j = j.wrapping_add(1);
    }
    return cd;
}
unsafe extern "C" fn by_gid(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut classdef_sortrecord)).gid as ::core::ffi::c_int
        - (*(b as *mut classdef_sortrecord)).gid as ::core::ffi::c_int;
}
pub(crate) unsafe extern "C" fn buildClassDef(mut cd: *const otl_ClassDef) -> *mut caryll_Buffer {
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite16b(buf, 2 as u16);
    if (*cd).numGlyphs == 0 {
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    let mut r: *mut classdef_sortrecord = ::core::ptr::null_mut::<classdef_sortrecord>();
    r = __caryll_allocate_clean(
        (::core::mem::size_of::<classdef_sortrecord>() as usize)
            .wrapping_mul((*cd).numGlyphs as usize),
        167 as ::core::ffi::c_ulong,
    ) as *mut classdef_sortrecord;
    let mut jj: glyphid_t = 0 as glyphid_t;
    let mut j: glyphid_t = 0 as glyphid_t;
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
        r = ::core::ptr::null_mut::<classdef_sortrecord>();
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    qsort(
        r as *mut ::core::ffi::c_void,
        jj as usize,
        ::core::mem::size_of::<classdef_sortrecord>() as usize,
        Some(
            by_gid
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut startGID: glyphid_t = (*r.offset(0 as ::core::ffi::c_int as isize)).gid;
    let mut endGID: glyphid_t = startGID;
    let mut lastClass: glyphclass_t = (*r.offset(0 as ::core::ffi::c_int as isize)).cid;
    let mut nRanges: glyphid_t = 0 as glyphid_t;
    let mut lastGID: glyphid_t = startGID;
    let mut ranges: *mut caryll_Buffer = bufnew();
    let mut j_0: glyphid_t = 1 as glyphid_t;
    while (j_0 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        let mut current: glyphid_t = (*r.offset(j_0 as isize)).gid;
        if !(current as ::core::ffi::c_int <= lastGID as ::core::ffi::c_int) {
            if current as ::core::ffi::c_int
                == endGID as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                && (*r.offset(j_0 as isize)).cid as ::core::ffi::c_int
                    == lastClass as ::core::ffi::c_int
            {
                endGID = current;
            } else {
                bufwrite16b(ranges, startGID as u16);
                bufwrite16b(ranges, endGID as u16);
                bufwrite16b(ranges, lastClass as u16);
                nRanges = (nRanges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
                endGID = current;
                startGID = endGID;
                lastClass = (*r.offset(j_0 as isize)).cid;
            }
            lastGID = current;
        }
        j_0 = j_0.wrapping_add(1);
    }
    bufwrite16b(ranges, startGID as u16);
    bufwrite16b(ranges, endGID as u16);
    bufwrite16b(ranges, lastClass as u16);
    nRanges = (nRanges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
    bufwrite16b(buf, nRanges as u16);
    bufwrite_bufdel(buf, ranges);
    free(r as *mut ::core::ffi::c_void);
    r = ::core::ptr::null_mut::<classdef_sortrecord>();
    return buf;
}
pub(crate) unsafe extern "C" fn shrinkClassDef(mut cd: *mut otl_ClassDef) {
    let mut k: glyphid_t = 0 as glyphid_t;
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
        if !(*(*cd).glyphs.offset(j as isize)).name.is_null() {
            *(*cd).glyphs.offset(k as isize) = *(*cd).glyphs.offset(j as isize);
            *(*cd).classes.offset(k as isize) = *(*cd).classes.offset(j as isize);
            k = k.wrapping_add(1);
        } else {
            otfcc_Handle_dispose(
                (*cd).glyphs.offset(j as isize) as *mut otfcc_Handle,
            );
        }
        j = j.wrapping_add(1);
    }
    (*cd).numGlyphs = k;
}
#[no_mangle]
pub static mut otl_iClassDef: __otfcc_IClassDef = {
    __otfcc_IClassDef {
        init: Some(otl_ClassDef_init as unsafe extern "C" fn(*mut otl_ClassDef) -> ()),
        copy: Some(
            otl_ClassDef_copy as unsafe extern "C" fn(*mut otl_ClassDef, *const otl_ClassDef) -> (),
        ),
        move_0: Some(
            otl_ClassDef_move as unsafe extern "C" fn(*mut otl_ClassDef, *mut otl_ClassDef) -> (),
        ),
        dispose: Some(otl_ClassDef_dispose as unsafe extern "C" fn(*mut otl_ClassDef) -> ()),
        replace: Some(
            otl_ClassDef_replace as unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> (),
        ),
        copyReplace: Some(
            otl_ClassDef_copyReplace as unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> (),
        ),
        create: Some(otl_ClassDef_create),
        free: Some(otl_ClassDef_free as unsafe extern "C" fn(*mut otl_ClassDef) -> ()),
        push: Some(
            pushClassDef
                as unsafe extern "C" fn(*mut otl_ClassDef, otfcc_GlyphHandle, glyphclass_t) -> (),
        ),
        read: Some(
            readClassDef
                as unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_ClassDef,
        ),
        expand: Some(
            expandClassDef
                as unsafe extern "C" fn(*mut otl_Coverage, *mut otl_ClassDef) -> *mut otl_ClassDef,
        ),
        dump: Some(dumpClassDef as unsafe extern "C" fn(*const otl_ClassDef) -> *mut json_value),
        parse: Some(parseClassDef as unsafe extern "C" fn(*const json_value) -> *mut otl_ClassDef),
        build: Some(
            buildClassDef as unsafe extern "C" fn(*const otl_ClassDef) -> *mut caryll_Buffer,
        ),
        shrink: Some(shrinkClassDef as unsafe extern "C" fn(*mut otl_ClassDef) -> ()),
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
