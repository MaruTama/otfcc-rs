use libc::{exit, free, malloc, memcmp, memcpy, memset, qsort, strcmp};
extern "C" {
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
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
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    static otl_iCoverage: __otfcc_ICoverage;
    static otl_iClassDef: __otfcc_IClassDef;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_newGraphFromRootBlock(b: *mut bk_Block) -> *mut bk_Graph;
    fn bk_delete_Graph(f: *mut bk_Graph);
    fn bk_minimizeGraph(f: *mut bk_Graph);
    fn bk_untangleGraph(f: *mut bk_Graph);
    fn bk_build_Graph(f: *mut bk_Graph) -> *mut caryll_Buffer;
    fn bk_estimateSizeOfGraph(f: *mut bk_Graph) -> usize;
    static FORMAT_DWIDTH: u8;
    fn position_format_length(format: u16) -> u8;
    fn position_zero() -> otl_PositionValue;
    fn read_gpos_value(
        data: font_file_pointer,
        tableLength: u32,
        offset: u32,
        format: u16,
    ) -> otl_PositionValue;
    fn required_position_format(v: otl_PositionValue) -> u8;
    fn bk_gpos_value(v: otl_PositionValue, format: u16) -> *mut bk_Block;
    fn gpos_dump_value(value: otl_PositionValue) -> *mut json_value;
    fn gpos_parse_value(pos: *mut json_value) -> otl_PositionValue;
}

use crate::table::otl::classdef::{__otfcc_IClassDef, expandClassDef, otl_ClassDef, otl_ClassDef_free, readClassDef};
use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, otl_Coverage_free, readCoverage, shrinkCoverage};
use crate::support::handle::{handle_fromIndex, otfcc_Handle_dup, otfcc_Handle, otfcc_GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphclass_t, glyphid_t, pos_t, tableid_t};
use crate::vendor::json::{json_array, json_double, json_integer, json_object, json_pre_serialized, json_type, json_value};
use crate::bk::bkblock::{b16, bk_Block, bkembed, bkover, p16};
use crate::bk::bkgraph::{bk_Graph};
use crate::support::{NULL};
use crate::table::otl::{__caryll_elementinterface_subtable_gpos_pair, otl_PositionValue, otl_Subtable, subtable_gpos_pair};
use crate::table::otl::subtables::{otl_BuildHeuristics};
use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UT_hash_bucket, UT_hash_handle, UT_hash_table};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pair_classifier_hash {
    pub gid: ::core::ffi::c_int,
    pub cid: ::core::ffi::c_int,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndividualGposPair {
    pub gid: glyphid_t,
    pub fv: *mut otl_PositionValue,
    pub sv: *mut otl_PositionValue,
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_new_position(mut z: pos_t) -> *mut json_value {
    if round(z as ::core::ffi::c_double) == z {
        return json_integer_new(z as i64);
    } else {
        return json_double_new(z as ::core::ffi::c_double);
    };
}
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
#[inline]
unsafe extern "C" fn initGposPair(mut subtable: *mut subtable_gpos_pair) {
    (*subtable).first = ::core::ptr::null_mut::<otl_ClassDef>();
    (*subtable).second = ::core::ptr::null_mut::<otl_ClassDef>();
    (*subtable).firstValues = ::core::ptr::null_mut::<*mut otl_PositionValue>();
    (*subtable).secondValues = ::core::ptr::null_mut::<*mut otl_PositionValue>();
}
#[inline]
unsafe extern "C" fn disposeGposPair(mut subtable: *mut subtable_gpos_pair) {
    if !(*subtable).firstValues.is_null() {
        let mut j: glyphclass_t = 0 as glyphclass_t;
        while j as ::core::ffi::c_int <= (*(*subtable).first).maxclass as ::core::ffi::c_int {
            free(*(*subtable).firstValues.offset(j as isize) as *mut ::core::ffi::c_void);
            let ref mut fresh0 = *(*subtable).firstValues.offset(j as isize);
            *fresh0 = ::core::ptr::null_mut::<otl_PositionValue>();
            j = j.wrapping_add(1);
        }
        free((*subtable).firstValues as *mut ::core::ffi::c_void);
        (*subtable).firstValues = ::core::ptr::null_mut::<*mut otl_PositionValue>();
    }
    if !(*subtable).secondValues.is_null() {
        let mut j_0: glyphclass_t = 0 as glyphclass_t;
        while j_0 as ::core::ffi::c_int <= (*(*subtable).first).maxclass as ::core::ffi::c_int {
            free(*(*subtable).secondValues.offset(j_0 as isize) as *mut ::core::ffi::c_void);
            let ref mut fresh1 = *(*subtable).secondValues.offset(j_0 as isize);
            *fresh1 = ::core::ptr::null_mut::<otl_PositionValue>();
            j_0 = j_0.wrapping_add(1);
        }
        free((*subtable).secondValues as *mut ::core::ffi::c_void);
        (*subtable).secondValues = ::core::ptr::null_mut::<*mut otl_PositionValue>();
    }
    otl_ClassDef_free((*subtable).first);
    (*subtable).first = ::core::ptr::null_mut::<otl_ClassDef>();
    otl_ClassDef_free((*subtable).second);
    (*subtable).second = ::core::ptr::null_mut::<otl_ClassDef>();
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_move(
    mut dst: *mut subtable_gpos_pair,
    mut src: *mut subtable_gpos_pair,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_pair>() as usize,
    );
    subtable_gpos_pair_init(src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_dispose(mut x: *mut subtable_gpos_pair) {
    disposeGposPair(x);
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_replace(
    mut dst: *mut subtable_gpos_pair,
    src: subtable_gpos_pair,
) {
    subtable_gpos_pair_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_pair>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_copy(
    mut dst: *mut subtable_gpos_pair,
    mut src: *const subtable_gpos_pair,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_pair>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_copyReplace(
    mut dst: *mut subtable_gpos_pair,
    src: subtable_gpos_pair,
) {
    subtable_gpos_pair_dispose(dst);
    subtable_gpos_pair_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_create() -> *mut subtable_gpos_pair {
    let mut x: *mut subtable_gpos_pair =
        malloc(::core::mem::size_of::<subtable_gpos_pair>() as usize) as *mut subtable_gpos_pair;
    subtable_gpos_pair_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_init(mut x: *mut subtable_gpos_pair) {
    initGposPair(x);
}
#[no_mangle]
pub static mut iSubtable_gpos_pair: __caryll_elementinterface_subtable_gpos_pair = {
    __caryll_elementinterface_subtable_gpos_pair {
        init: Some(subtable_gpos_pair_init as unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()),
        copy: Some(
            subtable_gpos_pair_copy
                as unsafe extern "C" fn(*mut subtable_gpos_pair, *const subtable_gpos_pair) -> (),
        ),
        move_0: Some(
            subtable_gpos_pair_move
                as unsafe extern "C" fn(*mut subtable_gpos_pair, *mut subtable_gpos_pair) -> (),
        ),
        dispose: Some(
            subtable_gpos_pair_dispose as unsafe extern "C" fn(*mut subtable_gpos_pair) -> (),
        ),
        replace: Some(
            subtable_gpos_pair_replace
                as unsafe extern "C" fn(*mut subtable_gpos_pair, subtable_gpos_pair) -> (),
        ),
        copyReplace: Some(
            subtable_gpos_pair_copyReplace
                as unsafe extern "C" fn(*mut subtable_gpos_pair, subtable_gpos_pair) -> (),
        ),
        create: Some(subtable_gpos_pair_create),
        free: Some(subtable_gpos_pair_free as unsafe extern "C" fn(*mut subtable_gpos_pair) -> ()),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_pair_free(mut x: *mut subtable_gpos_pair) {
    if x.is_null() {
        return;
    }
    subtable_gpos_pair_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gpos_pair(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut subtableFormat: u16 = 0;
    let mut current_block: u64;
    let mut subtable: *mut subtable_gpos_pair =
        (
            iSubtable_gpos_pair
                .create
                .expect("non-null function pointer"))();
    if !(tableLength < offset.wrapping_add(2 as u32)) {
        subtableFormat = read_16u(data.offset(offset as isize) as *const u8);
        if subtableFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            let mut cov: *mut otl_Coverage = readCoverage(
                data as *const u8,
                tableLength,
                offset.wrapping_add(read_16u(
                    data.offset(offset as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as u32),
            );
            (*subtable).first = __caryll_allocate_clean(
                ::core::mem::size_of::<otl_ClassDef>() as usize,
                45 as ::core::ffi::c_ulong,
            ) as *mut otl_ClassDef;
            (*(*subtable).first).numGlyphs = (*cov).numGlyphs;
            (*(*subtable).first).maxclass =
                ((*cov).numGlyphs as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as glyphclass_t;
            (*(*subtable).first).glyphs = (*cov).glyphs;
            (*(*subtable).first).classes = __caryll_allocate_clean(
                (::core::mem::size_of::<glyphclass_t>() as usize)
                    .wrapping_mul((*cov).numGlyphs as usize),
                49 as ::core::ffi::c_ulong,
            ) as *mut glyphclass_t;
            let mut j: glyphid_t = 0 as glyphid_t;
            while (j as ::core::ffi::c_int) < (*cov).numGlyphs as ::core::ffi::c_int {
                *(*(*subtable).first).classes.offset(j as isize) = j as glyphclass_t;
                j = j.wrapping_add(1);
            }
            free(cov as *mut ::core::ffi::c_void);
            cov = ::core::ptr::null_mut::<otl_Coverage>();
            let mut format1: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            );
            let mut format2: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            );
            let mut len1: u8 = position_format_length(format1);
            let mut len2: u8 = position_format_length(format2);
            let mut pairSetCount: glyphid_t = read_16u(
                data.offset(offset as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *const u8,
            ) as glyphid_t;
            if !(pairSetCount as ::core::ffi::c_int
                != (*(*subtable).first).numGlyphs as ::core::ffi::c_int)
            {
                if !(tableLength
                    < offset.wrapping_add(10 as u32).wrapping_add(
                        (2 as ::core::ffi::c_int * pairSetCount as ::core::ffi::c_int) as u32,
                    ))
                {
                    let mut j_0: glyphid_t = 0 as glyphid_t;
                    loop {
                        if !((j_0 as ::core::ffi::c_int) < pairSetCount as ::core::ffi::c_int) {
                            current_block = 14401909646449704462;
                            break;
                        }
                        let mut pairSetOffset: u32 = offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(10 as ::core::ffi::c_int as isize)
                                .offset(
                                    (2 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        if tableLength < pairSetOffset.wrapping_add(2 as u32) {
                            current_block = 1524425613423851471;
                            break;
                        }
                        let mut pairCount: glyphid_t =
                            read_16u(data.offset(pairSetOffset as isize) as *const u8)
                                as glyphid_t;
                        if tableLength
                            < pairSetOffset.wrapping_add(2 as u32).wrapping_add(
                                ((2 as ::core::ffi::c_int
                                    + len1 as ::core::ffi::c_int
                                    + len2 as ::core::ffi::c_int)
                                    * pairCount as ::core::ffi::c_int)
                                    as u32,
                            )
                        {
                            current_block = 1524425613423851471;
                            break;
                        }
                        j_0 = j_0.wrapping_add(1);
                    }
                    match current_block {
                        1524425613423851471 => {}
                        _ => {
                            let mut h: *mut pair_classifier_hash =
                                ::core::ptr::null_mut::<pair_classifier_hash>();
                            let mut j_1: glyphid_t = 0 as glyphid_t;
                            while (j_1 as ::core::ffi::c_int) < pairSetCount as ::core::ffi::c_int {
                                let mut pairSetOffset_0: u32 = offset.wrapping_add(read_16u(
                                    data.offset(offset as isize)
                                        .offset(10 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int * j_1 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let mut pairCount_0: glyphid_t = read_16u(
                                    data.offset(pairSetOffset_0 as isize) as *const u8,
                                )
                                    as glyphid_t;
                                let mut k: glyphid_t = 0 as glyphid_t;
                                while (k as ::core::ffi::c_int) < pairCount_0 as ::core::ffi::c_int
                                {
                                    let mut second: ::core::ffi::c_int = read_16u(
                                        data.offset(pairSetOffset_0 as isize)
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            .offset(
                                                ((2 as ::core::ffi::c_int
                                                    + len1 as ::core::ffi::c_int
                                                    + len2 as ::core::ffi::c_int)
                                                    * k as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            as *const u8,
                                    )
                                        as ::core::ffi::c_int;
                                    let mut s: *mut pair_classifier_hash =
                                        ::core::ptr::null_mut::<pair_classifier_hash>();
                                    let mut _hf_hashv: ::core::ffi::c_uint = 0;
                                    let mut _hj_i: ::core::ffi::c_uint = 0;
                                    let mut _hj_j: ::core::ffi::c_uint = 0;
                                    let mut _hj_k: ::core::ffi::c_uint = 0;
                                    let mut _hj_key: *const ::core::ffi::c_uchar =
                                        &raw mut second as *const ::core::ffi::c_uchar;
                                    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                                    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
                                    _hj_i = _hj_j;
                                    _hj_k = ::core::mem::size_of::<::core::ffi::c_int>()
                                        as ::core::ffi::c_uint;
                                    while _hj_k >= 12 as ::core::ffi::c_uint {
                                        _hj_i = _hj_i.wrapping_add(
                                            (*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uint)
                                                .wrapping_add(
                                                    (*_hj_key
                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 8 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key
                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 16 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key
                                                        .offset(3 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 24 as ::core::ffi::c_int,
                                                ),
                                        );
                                        _hj_j = _hj_j.wrapping_add(
                                            (*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uint)
                                                .wrapping_add(
                                                    (*_hj_key
                                                        .offset(5 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 8 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key
                                                        .offset(6 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 16 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key
                                                        .offset(7 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 24 as ::core::ffi::c_int,
                                                ),
                                        );
                                        _hf_hashv = _hf_hashv.wrapping_add(
                                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uint)
                                                .wrapping_add(
                                                    (*_hj_key
                                                        .offset(9 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 8 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key
                                                        .offset(10 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 16 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key
                                                        .offset(11 as ::core::ffi::c_int as isize)
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
                                    _hf_hashv = _hf_hashv.wrapping_add(::core::mem::size_of::<
                                        ::core::ffi::c_int,
                                    >(
                                    )
                                        as ::core::ffi::c_uint);
                                    let mut current_block_63: u64;
                                    match _hj_k {
                                        11 => {
                                            _hf_hashv = _hf_hashv.wrapping_add(
                                                (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 24 as ::core::ffi::c_int,
                                            );
                                            current_block_63 = 17527452374945666132;
                                        }
                                        10 => {
                                            current_block_63 = 17527452374945666132;
                                        }
                                        9 => {
                                            current_block_63 = 1284488772475364109;
                                        }
                                        8 => {
                                            current_block_63 = 13776434095799694680;
                                        }
                                        7 => {
                                            current_block_63 = 18245629441231827436;
                                        }
                                        6 => {
                                            current_block_63 = 13459369624805094279;
                                        }
                                        5 => {
                                            current_block_63 = 11323755229941482461;
                                        }
                                        4 => {
                                            current_block_63 = 9892725151128565583;
                                        }
                                        3 => {
                                            current_block_63 = 6536138360546159271;
                                        }
                                        2 => {
                                            current_block_63 = 15840492914139847558;
                                        }
                                        1 => {
                                            current_block_63 = 15611684505120711141;
                                        }
                                        _ => {
                                            current_block_63 = 2606304779496145856;
                                        }
                                    }
                                    match current_block_63 {
                                        17527452374945666132 => {
                                            _hf_hashv = _hf_hashv.wrapping_add(
                                                (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 16 as ::core::ffi::c_int,
                                            );
                                            current_block_63 = 1284488772475364109;
                                        }
                                        _ => {}
                                    }
                                    match current_block_63 {
                                        1284488772475364109 => {
                                            _hf_hashv = _hf_hashv.wrapping_add(
                                                (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 8 as ::core::ffi::c_int,
                                            );
                                            current_block_63 = 13776434095799694680;
                                        }
                                        _ => {}
                                    }
                                    match current_block_63 {
                                        13776434095799694680 => {
                                            _hj_j = _hj_j.wrapping_add(
                                                (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 24 as ::core::ffi::c_int,
                                            );
                                            current_block_63 = 18245629441231827436;
                                        }
                                        _ => {}
                                    }
                                    match current_block_63 {
                                        18245629441231827436 => {
                                            _hj_j = _hj_j.wrapping_add(
                                                (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 16 as ::core::ffi::c_int,
                                            );
                                            current_block_63 = 13459369624805094279;
                                        }
                                        _ => {}
                                    }
                                    match current_block_63 {
                                        13459369624805094279 => {
                                            _hj_j = _hj_j.wrapping_add(
                                                (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 8 as ::core::ffi::c_int,
                                            );
                                            current_block_63 = 11323755229941482461;
                                        }
                                        _ => {}
                                    }
                                    match current_block_63 {
                                        11323755229941482461 => {
                                            _hj_j = _hj_j.wrapping_add(
                                                *_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint,
                                            );
                                            current_block_63 = 9892725151128565583;
                                        }
                                        _ => {}
                                    }
                                    match current_block_63 {
                                        9892725151128565583 => {
                                            _hj_i = _hj_i.wrapping_add(
                                                (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 24 as ::core::ffi::c_int,
                                            );
                                            current_block_63 = 6536138360546159271;
                                        }
                                        _ => {}
                                    }
                                    match current_block_63 {
                                        6536138360546159271 => {
                                            _hj_i = _hj_i.wrapping_add(
                                                (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 16 as ::core::ffi::c_int,
                                            );
                                            current_block_63 = 15840492914139847558;
                                        }
                                        _ => {}
                                    }
                                    match current_block_63 {
                                        15840492914139847558 => {
                                            _hj_i = _hj_i.wrapping_add(
                                                (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 8 as ::core::ffi::c_int,
                                            );
                                            current_block_63 = 15611684505120711141;
                                        }
                                        _ => {}
                                    }
                                    match current_block_63 {
                                        15611684505120711141 => {
                                            _hj_i = _hj_i.wrapping_add(
                                                *_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint,
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
                                    s = ::core::ptr::null_mut::<pair_classifier_hash>();
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
                                                s = ((*(*(*h).hh.tbl)
                                                    .buckets
                                                    .offset(_hf_bkt as isize))
                                                .hh_head
                                                    as *mut ::core::ffi::c_char)
                                                    .offset(-(*(*h).hh.tbl).hho)
                                                    as *mut ::core::ffi::c_void
                                                    as *mut pair_classifier_hash
                                                    as *mut pair_classifier_hash;
                                            } else {
                                                s = ::core::ptr::null_mut::<pair_classifier_hash>();
                                            }
                                            while !s.is_null() {
                                                if (*s).hh.hashv == _hf_hashv
                                                    && (*s).hh.keylen as usize
                                                        == ::core::mem::size_of::<::core::ffi::c_int>(
                                                        )
                                                {
                                                    if memcmp(
                                                        (*s).hh.key,
                                                        &raw mut second
                                                            as *const ::core::ffi::c_void,
                                                        ::core::mem::size_of::<::core::ffi::c_int>()
                                                            as usize,
                                                    ) == 0 as ::core::ffi::c_int
                                                    {
                                                        break;
                                                    }
                                                }
                                                if !(*s).hh.hh_next.is_null() {
                                                    s = ((*s).hh.hh_next
                                                        as *mut ::core::ffi::c_char)
                                                        .offset(-(*(*h).hh.tbl).hho)
                                                        as *mut ::core::ffi::c_void
                                                        as *mut pair_classifier_hash
                                                        as *mut pair_classifier_hash;
                                                } else {
                                                    s = ::core::ptr::null_mut::<pair_classifier_hash>(
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    if s.is_null() {
                                        s = __caryll_allocate_clean(
                                            ::core::mem::size_of::<pair_classifier_hash>()
                                                as usize,
                                            81 as ::core::ffi::c_ulong,
                                        )
                                            as *mut pair_classifier_hash;
                                        (*s).gid = second;
                                        (*s).cid = (if !h.is_null() {
                                            (*(*h).hh.tbl).num_items
                                        } else {
                                            0 as ::core::ffi::c_uint
                                        })
                                        .wrapping_add(1 as ::core::ffi::c_uint)
                                            as ::core::ffi::c_int;
                                        let mut _ha_hashv: ::core::ffi::c_uint = 0;
                                        let mut _hj_i_0: ::core::ffi::c_uint = 0;
                                        let mut _hj_j_0: ::core::ffi::c_uint = 0;
                                        let mut _hj_k_0: ::core::ffi::c_uint = 0;
                                        let mut _hj_key_0: *const ::core::ffi::c_uchar =
                                            &raw mut (*s).gid as *const ::core::ffi::c_uchar;
                                        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                                        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                                        _hj_i_0 = _hj_j_0;
                                        _hj_k_0 = ::core::mem::size_of::<::core::ffi::c_int>()
                                            as ::core::ffi::c_uint;
                                        while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                                            _hj_i_0 =
                                                _hj_i_0.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            (*_hj_key_0.offset(
                                                                1 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_uint)
                                                                << 8 as ::core::ffi::c_int,
                                                        )
                                                        .wrapping_add(
                                                            (*_hj_key_0.offset(
                                                                2 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_uint)
                                                                << 16 as ::core::ffi::c_int,
                                                        )
                                                        .wrapping_add(
                                                            (*_hj_key_0.offset(
                                                                3 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_uint)
                                                                << 24 as ::core::ffi::c_int,
                                                        ),
                                                );
                                            _hj_j_0 =
                                                _hj_j_0.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(4 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            (*_hj_key_0.offset(
                                                                5 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_uint)
                                                                << 8 as ::core::ffi::c_int,
                                                        )
                                                        .wrapping_add(
                                                            (*_hj_key_0.offset(
                                                                6 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_uint)
                                                                << 16 as ::core::ffi::c_int,
                                                        )
                                                        .wrapping_add(
                                                            (*_hj_key_0.offset(
                                                                7 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_uint)
                                                                << 24 as ::core::ffi::c_int,
                                                        ),
                                                );
                                            _ha_hashv =
                                                _ha_hashv.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(8 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            (*_hj_key_0.offset(
                                                                9 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_uint)
                                                                << 8 as ::core::ffi::c_int,
                                                        )
                                                        .wrapping_add(
                                                            (*_hj_key_0.offset(
                                                                10 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_uint)
                                                                << 16 as ::core::ffi::c_int,
                                                        )
                                                        .wrapping_add(
                                                            (*_hj_key_0.offset(
                                                                11 as ::core::ffi::c_int as isize,
                                                            )
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
                                            _hj_key_0 =
                                                _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
                                            _hj_k_0 =
                                                _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
                                        }
                                        _ha_hashv = _ha_hashv.wrapping_add(::core::mem::size_of::<
                                            ::core::ffi::c_int,
                                        >(
                                        )
                                            as ::core::ffi::c_uint);
                                        let mut current_block_180: u64;
                                        match _hj_k_0 {
                                            11 => {
                                                _ha_hashv = _ha_hashv.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(10 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 24 as ::core::ffi::c_int,
                                                );
                                                current_block_180 = 18331491177742630429;
                                            }
                                            10 => {
                                                current_block_180 = 18331491177742630429;
                                            }
                                            9 => {
                                                current_block_180 = 13120855040837624029;
                                            }
                                            8 => {
                                                current_block_180 = 5389028970551061448;
                                            }
                                            7 => {
                                                current_block_180 = 11634030214995089741;
                                            }
                                            6 => {
                                                current_block_180 = 14050160523012167154;
                                            }
                                            5 => {
                                                current_block_180 = 1723857576044732699;
                                            }
                                            4 => {
                                                current_block_180 = 12635778430218748736;
                                            }
                                            3 => {
                                                current_block_180 = 13626785338223520346;
                                            }
                                            2 => {
                                                current_block_180 = 16871651390580484494;
                                            }
                                            1 => {
                                                current_block_180 = 3217027976071949983;
                                            }
                                            _ => {
                                                current_block_180 = 12969817083969514432;
                                            }
                                        }
                                        match current_block_180 {
                                            18331491177742630429 => {
                                                _ha_hashv = _ha_hashv.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(9 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 16 as ::core::ffi::c_int,
                                                );
                                                current_block_180 = 13120855040837624029;
                                            }
                                            _ => {}
                                        }
                                        match current_block_180 {
                                            13120855040837624029 => {
                                                _ha_hashv = _ha_hashv.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(8 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 8 as ::core::ffi::c_int,
                                                );
                                                current_block_180 = 5389028970551061448;
                                            }
                                            _ => {}
                                        }
                                        match current_block_180 {
                                            5389028970551061448 => {
                                                _hj_j_0 = _hj_j_0.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(7 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 24 as ::core::ffi::c_int,
                                                );
                                                current_block_180 = 11634030214995089741;
                                            }
                                            _ => {}
                                        }
                                        match current_block_180 {
                                            11634030214995089741 => {
                                                _hj_j_0 = _hj_j_0.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(6 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 16 as ::core::ffi::c_int,
                                                );
                                                current_block_180 = 14050160523012167154;
                                            }
                                            _ => {}
                                        }
                                        match current_block_180 {
                                            14050160523012167154 => {
                                                _hj_j_0 = _hj_j_0.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(5 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 8 as ::core::ffi::c_int,
                                                );
                                                current_block_180 = 1723857576044732699;
                                            }
                                            _ => {}
                                        }
                                        match current_block_180 {
                                            1723857576044732699 => {
                                                _hj_j_0 = _hj_j_0.wrapping_add(
                                                    *_hj_key_0
                                                        .offset(4 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint,
                                                );
                                                current_block_180 = 12635778430218748736;
                                            }
                                            _ => {}
                                        }
                                        match current_block_180 {
                                            12635778430218748736 => {
                                                _hj_i_0 = _hj_i_0.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(3 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 24 as ::core::ffi::c_int,
                                                );
                                                current_block_180 = 13626785338223520346;
                                            }
                                            _ => {}
                                        }
                                        match current_block_180 {
                                            13626785338223520346 => {
                                                _hj_i_0 = _hj_i_0.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 16 as ::core::ffi::c_int,
                                                );
                                                current_block_180 = 16871651390580484494;
                                            }
                                            _ => {}
                                        }
                                        match current_block_180 {
                                            16871651390580484494 => {
                                                _hj_i_0 = _hj_i_0.wrapping_add(
                                                    (*_hj_key_0
                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 8 as ::core::ffi::c_int,
                                                );
                                                current_block_180 = 3217027976071949983;
                                            }
                                            _ => {}
                                        }
                                        match current_block_180 {
                                            3217027976071949983 => {
                                                _hj_i_0 = _hj_i_0.wrapping_add(
                                                    *_hj_key_0
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint,
                                                );
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
                                        (*s).hh.hashv = _ha_hashv;
                                        (*s).hh.key = &raw mut (*s).gid as *mut ::core::ffi::c_char
                                            as *mut ::core::ffi::c_void;
                                        (*s).hh.keylen =
                                            ::core::mem::size_of::<::core::ffi::c_int>()
                                                as ::core::ffi::c_uint;
                                        if h.is_null() {
                                            (*s).hh.next = NULL;
                                            (*s).hh.prev = NULL;
                                            (*s).hh.tbl =
                                                malloc(::core::mem::size_of::<UT_hash_table>()
                                                    as usize)
                                                    as *mut UT_hash_table
                                                    as *mut UT_hash_table;
                                            if (*s).hh.tbl.is_null() {
                                                exit(-(1 as ::core::ffi::c_int));
                                            } else {
                                                memset(
                                                    (*s).hh.tbl as *mut ::core::ffi::c_void,
                                                    '\0' as i32,
                                                    ::core::mem::size_of::<UT_hash_table>()
                                                        as usize,
                                                );
                                                (*(*s).hh.tbl).tail =
                                                    &raw mut (*s).hh as *mut UT_hash_handle;
                                                (*(*s).hh.tbl).num_buckets =
                                                    HASH_INITIAL_NUM_BUCKETS;
                                                (*(*s).hh.tbl).log2_num_buckets =
                                                    HASH_INITIAL_NUM_BUCKETS_LOG2;
                                                (*(*s).hh.tbl).hho = (&raw mut (*s).hh
                                                    as *mut ::core::ffi::c_char)
                                                    .offset_from(s as *mut ::core::ffi::c_char)
                                                    as ::core::ffi::c_long
                                                    as isize;
                                                (*(*s).hh.tbl).buckets =
                                                    malloc((32 as usize).wrapping_mul(
                                                        ::core::mem::size_of::<UT_hash_bucket>()
                                                            as usize,
                                                    ))
                                                        as *mut UT_hash_bucket;
                                                (*(*s).hh.tbl).signature =
                                                    HASH_SIGNATURE as u32;
                                                if (*(*s).hh.tbl).buckets.is_null() {
                                                    exit(-(1 as ::core::ffi::c_int));
                                                } else {
                                                    memset(
                                                        (*(*s).hh.tbl).buckets
                                                            as *mut ::core::ffi::c_void,
                                                        '\0' as i32,
                                                        (32 as usize).wrapping_mul(
                                                            ::core::mem::size_of::<UT_hash_bucket>()
                                                                as usize,
                                                        ),
                                                    );
                                                }
                                            }
                                            h = s;
                                        } else {
                                            (*s).hh.tbl = (*h).hh.tbl;
                                            (*s).hh.next = NULL;
                                            (*s).hh.prev = ((*(*h).hh.tbl).tail
                                                as *mut ::core::ffi::c_char)
                                                .offset(-(*(*h).hh.tbl).hho)
                                                as *mut ::core::ffi::c_void;
                                            (*(*(*h).hh.tbl).tail).next =
                                                s as *mut ::core::ffi::c_void;
                                            (*(*h).hh.tbl).tail =
                                                &raw mut (*s).hh as *mut UT_hash_handle;
                                        }
                                        let mut _ha_bkt: ::core::ffi::c_uint = 0;
                                        (*(*h).hh.tbl).num_items =
                                            (*(*h).hh.tbl).num_items.wrapping_add(1);
                                        _ha_bkt = _ha_hashv
                                            & (*(*h).hh.tbl)
                                                .num_buckets
                                                .wrapping_sub(1 as ::core::ffi::c_uint);
                                        let mut _ha_head: *mut UT_hash_bucket =
                                            (*(*h).hh.tbl).buckets.offset(_ha_bkt as isize)
                                                as *mut UT_hash_bucket;
                                        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                                        (*s).hh.hh_next =
                                            (*_ha_head).hh_head as *mut UT_hash_handle;
                                        (*s).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                                        if !(*_ha_head).hh_head.is_null() {
                                            (*(*_ha_head).hh_head).hh_prev =
                                                &raw mut (*s).hh as *mut UT_hash_handle;
                                        }
                                        (*_ha_head).hh_head =
                                            &raw mut (*s).hh as *mut UT_hash_handle;
                                        if (*_ha_head).count
                                            >= (*_ha_head)
                                                .expand_mult
                                                .wrapping_add(1 as ::core::ffi::c_uint)
                                                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                                            && (*(*s).hh.tbl).noexpand == 0
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
                                                    .wrapping_mul(
                                                        (*(*s).hh.tbl).num_buckets as usize,
                                                    )
                                                    .wrapping_mul(::core::mem::size_of::<
                                                        UT_hash_bucket,
                                                    >(
                                                    )
                                                        as usize),
                                            )
                                                as *mut UT_hash_bucket;
                                            if _he_new_buckets.is_null() {
                                                exit(-(1 as ::core::ffi::c_int));
                                            } else {
                                                memset(
                                                    _he_new_buckets as *mut ::core::ffi::c_void,
                                                    '\0' as i32,
                                                    (2 as usize)
                                                        .wrapping_mul(
                                                            (*(*s).hh.tbl).num_buckets as usize,
                                                        )
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            UT_hash_bucket,
                                                        >(
                                                        )
                                                            as usize),
                                                );
                                                (*(*s).hh.tbl).ideal_chain_maxlen = ((*(*s)
                                                    .hh
                                                    .tbl)
                                                    .num_items
                                                    >> (*(*s).hh.tbl)
                                                        .log2_num_buckets
                                                        .wrapping_add(1 as ::core::ffi::c_uint))
                                                .wrapping_add(
                                                    if (*(*s).hh.tbl).num_items
                                                        & (*(*s).hh.tbl)
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
                                                (*(*s).hh.tbl).nonideal_items =
                                                    0 as ::core::ffi::c_uint;
                                                _he_bkt_i = 0 as ::core::ffi::c_uint;
                                                while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                                                    _he_thh = (*(*(*s).hh.tbl)
                                                        .buckets
                                                        .offset(_he_bkt_i as isize))
                                                    .hh_head
                                                        as *mut UT_hash_handle;
                                                    while !_he_thh.is_null() {
                                                        _he_hh_nxt = (*_he_thh).hh_next;
                                                        _he_bkt = (*_he_thh).hashv
                                                            & (*(*s).hh.tbl)
                                                                .num_buckets
                                                                .wrapping_mul(
                                                                    2 as ::core::ffi::c_uint,
                                                                )
                                                                .wrapping_sub(
                                                                    1 as ::core::ffi::c_uint,
                                                                );
                                                        _he_newbkt = _he_new_buckets
                                                            .offset(_he_bkt as isize)
                                                            as *mut UT_hash_bucket;
                                                        (*_he_newbkt).count =
                                                            (*_he_newbkt).count.wrapping_add(1);
                                                        if (*_he_newbkt).count
                                                            > (*(*s).hh.tbl).ideal_chain_maxlen
                                                        {
                                                            (*(*s).hh.tbl).nonideal_items =
                                                                (*(*s).hh.tbl)
                                                                    .nonideal_items
                                                                    .wrapping_add(1);
                                                            (*_he_newbkt).expand_mult =
                                                                (*_he_newbkt).count.wrapping_div(
                                                                    (*(*s).hh.tbl)
                                                                        .ideal_chain_maxlen,
                                                                );
                                                        }
                                                        (*_he_thh).hh_prev = ::core::ptr::null_mut::<
                                                            UT_hash_handle,
                                                        >(
                                                        );
                                                        (*_he_thh).hh_next = (*_he_newbkt).hh_head
                                                            as *mut UT_hash_handle;
                                                        if !(*_he_newbkt).hh_head.is_null() {
                                                            (*(*_he_newbkt).hh_head).hh_prev =
                                                                _he_thh;
                                                        }
                                                        (*_he_newbkt).hh_head =
                                                            _he_thh as *mut UT_hash_handle;
                                                        _he_thh = _he_hh_nxt;
                                                    }
                                                    _he_bkt_i = _he_bkt_i.wrapping_add(1);
                                                }
                                                free(
                                                    (*(*s).hh.tbl).buckets
                                                        as *mut ::core::ffi::c_void,
                                                );
                                                (*(*s).hh.tbl).num_buckets = (*(*s).hh.tbl)
                                                    .num_buckets
                                                    .wrapping_mul(2 as ::core::ffi::c_uint);
                                                (*(*s).hh.tbl).log2_num_buckets =
                                                    (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
                                                (*(*s).hh.tbl).buckets = _he_new_buckets;
                                                (*(*s).hh.tbl).ineff_expands = if (*(*s).hh.tbl)
                                                    .nonideal_items
                                                    > (*(*s).hh.tbl).num_items
                                                        >> 1 as ::core::ffi::c_int
                                                {
                                                    (*(*s).hh.tbl)
                                                        .ineff_expands
                                                        .wrapping_add(1 as ::core::ffi::c_uint)
                                                } else {
                                                    0 as ::core::ffi::c_uint
                                                };
                                                if (*(*s).hh.tbl).ineff_expands
                                                    > 1 as ::core::ffi::c_uint
                                                {
                                                    (*(*s).hh.tbl).noexpand =
                                                        1 as ::core::ffi::c_uint;
                                                }
                                            }
                                        }
                                    }
                                    k = k.wrapping_add(1);
                                }
                                j_1 = j_1.wrapping_add(1);
                            }
                            (*subtable).second = __caryll_allocate_clean(
                                ::core::mem::size_of::<otl_ClassDef>() as usize,
                                89 as ::core::ffi::c_ulong,
                            ) as *mut otl_ClassDef;
                            (*(*subtable).second).numGlyphs = (if !h.is_null() {
                                (*(*h).hh.tbl).num_items
                            } else {
                                0 as ::core::ffi::c_uint
                            })
                                as glyphid_t;
                            (*(*subtable).second).maxclass = (if !h.is_null() {
                                (*(*h).hh.tbl).num_items
                            } else {
                                0 as ::core::ffi::c_uint
                            })
                                as glyphclass_t;
                            (*(*subtable).second).classes = __caryll_allocate_clean(
                                (::core::mem::size_of::<glyphclass_t>() as usize)
                                    .wrapping_mul((*(*subtable).second).numGlyphs as usize),
                                92 as ::core::ffi::c_ulong,
                            )
                                as *mut glyphclass_t;
                            (*(*subtable).second).glyphs = __caryll_allocate_clean(
                                (::core::mem::size_of::<otfcc_GlyphHandle>() as usize)
                                    .wrapping_mul((*(*subtable).second).numGlyphs as usize),
                                93 as ::core::ffi::c_ulong,
                            )
                                as *mut otfcc_GlyphHandle;
                            let mut class2Count: glyphclass_t = ((*(*subtable).second).maxclass
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as glyphclass_t;
                            (*subtable).firstValues = __caryll_allocate_clean(
                                (::core::mem::size_of::<*mut otl_PositionValue>() as usize)
                                    .wrapping_mul(
                                        ((*(*subtable).first).maxclass as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int)
                                            as usize,
                                    ),
                                96 as ::core::ffi::c_ulong,
                            )
                                as *mut *mut otl_PositionValue;
                            (*subtable).secondValues = __caryll_allocate_clean(
                                (::core::mem::size_of::<*mut otl_PositionValue>() as usize)
                                    .wrapping_mul(
                                        ((*(*subtable).first).maxclass as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int)
                                            as usize,
                                    ),
                                97 as ::core::ffi::c_ulong,
                            )
                                as *mut *mut otl_PositionValue;
                            let mut j_2: glyphclass_t = 0 as glyphclass_t;
                            while j_2 as ::core::ffi::c_int
                                <= (*(*subtable).first).maxclass as ::core::ffi::c_int
                            {
                                let ref mut fresh2 = *(*subtable).firstValues.offset(j_2 as isize);
                                *fresh2 = __caryll_allocate_clean(
                                    (::core::mem::size_of::<otl_PositionValue>() as usize)
                                        .wrapping_mul(class2Count as usize),
                                    99 as ::core::ffi::c_ulong,
                                )
                                    as *mut otl_PositionValue;
                                let ref mut fresh3 = *(*subtable).secondValues.offset(j_2 as isize);
                                *fresh3 = __caryll_allocate_clean(
                                    (::core::mem::size_of::<otl_PositionValue>() as usize)
                                        .wrapping_mul(class2Count as usize),
                                    100 as ::core::ffi::c_ulong,
                                )
                                    as *mut otl_PositionValue;
                                let mut k_0: glyphclass_t = 0 as glyphclass_t;
                                while (k_0 as ::core::ffi::c_int)
                                    < class2Count as ::core::ffi::c_int
                                {
                                    *(*(*subtable).firstValues.offset(j_2 as isize))
                                        .offset(k_0 as isize) = position_zero();
                                    *(*(*subtable).secondValues.offset(j_2 as isize))
                                        .offset(k_0 as isize) = position_zero();
                                    k_0 = k_0.wrapping_add(1);
                                }
                                j_2 = j_2.wrapping_add(1);
                            }
                            let mut j_3: glyphclass_t = 0 as glyphclass_t;
                            while j_3 as ::core::ffi::c_int
                                <= (*(*subtable).first).maxclass as ::core::ffi::c_int
                            {
                                let mut pairSetOffset_1: u32 = offset.wrapping_add(read_16u(
                                    data.offset(offset as isize)
                                        .offset(10 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int * j_3 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let mut pairCount_1: glyphid_t = read_16u(
                                    data.offset(pairSetOffset_1 as isize) as *const u8,
                                )
                                    as glyphid_t;
                                let mut k_1: glyphid_t = 0 as glyphid_t;
                                while (k_1 as ::core::ffi::c_int)
                                    < pairCount_1 as ::core::ffi::c_int
                                {
                                    let mut second_0: ::core::ffi::c_int = read_16u(
                                        data.offset(pairSetOffset_1 as isize)
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            .offset(
                                                ((2 as ::core::ffi::c_int
                                                    + len1 as ::core::ffi::c_int
                                                    + len2 as ::core::ffi::c_int)
                                                    * k_1 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            as *const u8,
                                    )
                                        as ::core::ffi::c_int;
                                    let mut s_0: *mut pair_classifier_hash =
                                        ::core::ptr::null_mut::<pair_classifier_hash>();
                                    let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
                                    let mut _hj_i_1: ::core::ffi::c_uint = 0;
                                    let mut _hj_j_1: ::core::ffi::c_uint = 0;
                                    let mut _hj_k_1: ::core::ffi::c_uint = 0;
                                    let mut _hj_key_1: *const ::core::ffi::c_uchar =
                                        &raw mut second_0 as *const ::core::ffi::c_uchar;
                                    _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
                                    _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
                                    _hj_i_1 = _hj_j_1;
                                    _hj_k_1 = ::core::mem::size_of::<::core::ffi::c_int>()
                                        as ::core::ffi::c_uint;
                                    while _hj_k_1 >= 12 as ::core::ffi::c_uint {
                                        _hj_i_1 = _hj_i_1.wrapping_add(
                                            (*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uint)
                                                .wrapping_add(
                                                    (*_hj_key_1
                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 8 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key_1
                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 16 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key_1
                                                        .offset(3 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 24 as ::core::ffi::c_int,
                                                ),
                                        );
                                        _hj_j_1 = _hj_j_1.wrapping_add(
                                            (*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uint)
                                                .wrapping_add(
                                                    (*_hj_key_1
                                                        .offset(5 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 8 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key_1
                                                        .offset(6 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 16 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key_1
                                                        .offset(7 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 24 as ::core::ffi::c_int,
                                                ),
                                        );
                                        _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                                            (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uint)
                                                .wrapping_add(
                                                    (*_hj_key_1
                                                        .offset(9 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 8 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key_1
                                                        .offset(10 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uint)
                                                        << 16 as ::core::ffi::c_int,
                                                )
                                                .wrapping_add(
                                                    (*_hj_key_1
                                                        .offset(11 as ::core::ffi::c_int as isize)
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
                                        _hj_key_1 =
                                            _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
                                        _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
                                    }
                                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(::core::mem::size_of::<
                                        ::core::ffi::c_int,
                                    >(
                                    )
                                        as ::core::ffi::c_uint);
                                    let mut current_block_390: u64;
                                    match _hj_k_1 {
                                        11 => {
                                            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                                                (*_hj_key_1
                                                    .offset(10 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 24 as ::core::ffi::c_int,
                                            );
                                            current_block_390 = 4899997498697840198;
                                        }
                                        10 => {
                                            current_block_390 = 4899997498697840198;
                                        }
                                        9 => {
                                            current_block_390 = 2085934361839010130;
                                        }
                                        8 => {
                                            current_block_390 = 6263215873732679251;
                                        }
                                        7 => {
                                            current_block_390 = 746406627506480172;
                                        }
                                        6 => {
                                            current_block_390 = 17648238135976140395;
                                        }
                                        5 => {
                                            current_block_390 = 4804969265056275772;
                                        }
                                        4 => {
                                            current_block_390 = 10117027413191050543;
                                        }
                                        3 => {
                                            current_block_390 = 12112996543100875946;
                                        }
                                        2 => {
                                            current_block_390 = 13221271166207665853;
                                        }
                                        1 => {
                                            current_block_390 = 18158728836017841538;
                                        }
                                        _ => {
                                            current_block_390 = 3690279228315428065;
                                        }
                                    }
                                    match current_block_390 {
                                        4899997498697840198 => {
                                            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                                                (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 16 as ::core::ffi::c_int,
                                            );
                                            current_block_390 = 2085934361839010130;
                                        }
                                        _ => {}
                                    }
                                    match current_block_390 {
                                        2085934361839010130 => {
                                            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                                                (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 8 as ::core::ffi::c_int,
                                            );
                                            current_block_390 = 6263215873732679251;
                                        }
                                        _ => {}
                                    }
                                    match current_block_390 {
                                        6263215873732679251 => {
                                            _hj_j_1 = _hj_j_1.wrapping_add(
                                                (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 24 as ::core::ffi::c_int,
                                            );
                                            current_block_390 = 746406627506480172;
                                        }
                                        _ => {}
                                    }
                                    match current_block_390 {
                                        746406627506480172 => {
                                            _hj_j_1 = _hj_j_1.wrapping_add(
                                                (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 16 as ::core::ffi::c_int,
                                            );
                                            current_block_390 = 17648238135976140395;
                                        }
                                        _ => {}
                                    }
                                    match current_block_390 {
                                        17648238135976140395 => {
                                            _hj_j_1 = _hj_j_1.wrapping_add(
                                                (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 8 as ::core::ffi::c_int,
                                            );
                                            current_block_390 = 4804969265056275772;
                                        }
                                        _ => {}
                                    }
                                    match current_block_390 {
                                        4804969265056275772 => {
                                            _hj_j_1 = _hj_j_1.wrapping_add(
                                                *_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint,
                                            );
                                            current_block_390 = 10117027413191050543;
                                        }
                                        _ => {}
                                    }
                                    match current_block_390 {
                                        10117027413191050543 => {
                                            _hj_i_1 = _hj_i_1.wrapping_add(
                                                (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 24 as ::core::ffi::c_int,
                                            );
                                            current_block_390 = 12112996543100875946;
                                        }
                                        _ => {}
                                    }
                                    match current_block_390 {
                                        12112996543100875946 => {
                                            _hj_i_1 = _hj_i_1.wrapping_add(
                                                (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 16 as ::core::ffi::c_int,
                                            );
                                            current_block_390 = 13221271166207665853;
                                        }
                                        _ => {}
                                    }
                                    match current_block_390 {
                                        13221271166207665853 => {
                                            _hj_i_1 = _hj_i_1.wrapping_add(
                                                (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint)
                                                    << 8 as ::core::ffi::c_int,
                                            );
                                            current_block_390 = 18158728836017841538;
                                        }
                                        _ => {}
                                    }
                                    match current_block_390 {
                                        18158728836017841538 => {
                                            _hj_i_1 = _hj_i_1.wrapping_add(
                                                *_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uint,
                                            );
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
                                    s_0 = ::core::ptr::null_mut::<pair_classifier_hash>();
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
                                                s_0 = ((*(*(*h).hh.tbl)
                                                    .buckets
                                                    .offset(_hf_bkt_0 as isize))
                                                .hh_head
                                                    as *mut ::core::ffi::c_char)
                                                    .offset(-(*(*h).hh.tbl).hho)
                                                    as *mut ::core::ffi::c_void
                                                    as *mut pair_classifier_hash
                                                    as *mut pair_classifier_hash;
                                            } else {
                                                s_0 =
                                                    ::core::ptr::null_mut::<pair_classifier_hash>();
                                            }
                                            while !s_0.is_null() {
                                                if (*s_0).hh.hashv == _hf_hashv_0
                                                    && (*s_0).hh.keylen as usize
                                                        == ::core::mem::size_of::<::core::ffi::c_int>(
                                                        )
                                                {
                                                    if memcmp(
                                                        (*s_0).hh.key,
                                                        &raw mut second_0
                                                            as *const ::core::ffi::c_void,
                                                        ::core::mem::size_of::<::core::ffi::c_int>()
                                                            as usize,
                                                    ) == 0 as ::core::ffi::c_int
                                                    {
                                                        break;
                                                    }
                                                }
                                                if !(*s_0).hh.hh_next.is_null() {
                                                    s_0 = ((*s_0).hh.hh_next
                                                        as *mut ::core::ffi::c_char)
                                                        .offset(-(*(*h).hh.tbl).hho)
                                                        as *mut ::core::ffi::c_void
                                                        as *mut pair_classifier_hash
                                                        as *mut pair_classifier_hash;
                                                } else {
                                                    s_0 = ::core::ptr::null_mut::<
                                                        pair_classifier_hash,
                                                    >(
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    if !s_0.is_null() {
                                        *(*(*subtable).firstValues.offset(j_3 as isize))
                                            .offset((*s_0).cid as isize) = read_gpos_value(
                                            data,
                                            tableLength,
                                            pairSetOffset_1
                                                .wrapping_add(2 as u32)
                                                .wrapping_add(
                                                    ((2 as ::core::ffi::c_int
                                                        + len1 as ::core::ffi::c_int
                                                        + len2 as ::core::ffi::c_int)
                                                        * k_1 as ::core::ffi::c_int)
                                                        as u32,
                                                )
                                                .wrapping_add(2 as u32),
                                            format1,
                                        );
                                        *(*(*subtable).secondValues.offset(j_3 as isize))
                                            .offset((*s_0).cid as isize) = read_gpos_value(
                                            data,
                                            tableLength,
                                            pairSetOffset_1
                                                .wrapping_add(2 as u32)
                                                .wrapping_add(
                                                    ((2 as ::core::ffi::c_int
                                                        + len1 as ::core::ffi::c_int
                                                        + len2 as ::core::ffi::c_int)
                                                        * k_1 as ::core::ffi::c_int)
                                                        as u32,
                                                )
                                                .wrapping_add(2 as u32)
                                                .wrapping_add(len1 as u32),
                                            format2,
                                        );
                                    }
                                    k_1 = k_1.wrapping_add(1);
                                }
                                j_3 = j_3.wrapping_add(1);
                            }
                            let mut s_1: *mut pair_classifier_hash =
                                ::core::ptr::null_mut::<pair_classifier_hash>();
                            let mut tmp: *mut pair_classifier_hash =
                                ::core::ptr::null_mut::<pair_classifier_hash>();
                            let mut jj: glyphid_t = 0 as glyphid_t;
                            s_1 = h;
                            tmp = (if !h.is_null() { (*h).hh.next } else { NULL })
                                as *mut pair_classifier_hash
                                as *mut pair_classifier_hash;
                            while !s_1.is_null() {
                                *(*(*subtable).second).glyphs.offset(jj as isize) =
                                    handle_fromIndex(
                                        (*s_1).gid as glyphid_t,
                                    ) as otfcc_GlyphHandle;
                                *(*(*subtable).second).classes.offset(jj as isize) =
                                    (*s_1).cid as glyphclass_t;
                                jj = jj.wrapping_add(1);
                                let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s_1).hh;
                                if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                                    free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
                                    free((*h).hh.tbl as *mut ::core::ffi::c_void);
                                    h = ::core::ptr::null_mut::<pair_classifier_hash>();
                                } else {
                                    let mut _hd_bkt: ::core::ffi::c_uint = 0;
                                    if _hd_hh_del == (*(*h).hh.tbl).tail {
                                        (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev
                                            as *mut ::core::ffi::c_char)
                                            .offset((*(*h).hh.tbl).hho)
                                            as *mut UT_hash_handle
                                            as *mut UT_hash_handle;
                                    }
                                    if !(*_hd_hh_del).prev.is_null() {
                                        let ref mut fresh4 = (*(((*_hd_hh_del).prev
                                            as *mut ::core::ffi::c_char)
                                            .offset((*(*h).hh.tbl).hho)
                                            as *mut UT_hash_handle))
                                            .next;
                                        *fresh4 = (*_hd_hh_del).next;
                                    } else {
                                        h = (*_hd_hh_del).next as *mut pair_classifier_hash
                                            as *mut pair_classifier_hash;
                                    }
                                    if !(*_hd_hh_del).next.is_null() {
                                        let ref mut fresh5 = (*(((*_hd_hh_del).next
                                            as *mut ::core::ffi::c_char)
                                            .offset((*(*h).hh.tbl).hho)
                                            as *mut UT_hash_handle))
                                            .prev;
                                        *fresh5 = (*_hd_hh_del).prev;
                                    }
                                    _hd_bkt = (*_hd_hh_del).hashv
                                        & (*(*h).hh.tbl)
                                            .num_buckets
                                            .wrapping_sub(1 as ::core::ffi::c_uint);
                                    let mut _hd_head: *mut UT_hash_bucket =
                                        (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize)
                                            as *mut UT_hash_bucket;
                                    (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
                                    if (*_hd_head).hh_head == _hd_hh_del {
                                        (*_hd_head).hh_head =
                                            (*_hd_hh_del).hh_next as *mut UT_hash_handle;
                                    }
                                    if !(*_hd_hh_del).hh_prev.is_null() {
                                        (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
                                    }
                                    if !(*_hd_hh_del).hh_next.is_null() {
                                        (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
                                    }
                                    (*(*h).hh.tbl).num_items =
                                        (*(*h).hh.tbl).num_items.wrapping_sub(1);
                                }
                                free(s_1 as *mut ::core::ffi::c_void);
                                s_1 = ::core::ptr::null_mut::<pair_classifier_hash>();
                                s_1 = tmp;
                                tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL })
                                    as *mut pair_classifier_hash
                                    as *mut pair_classifier_hash;
                            }
                            return subtable as *mut otl_Subtable;
                        }
                    }
                }
            }
        } else if subtableFormat as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            if !(tableLength < offset.wrapping_add(16 as u32)) {
                let mut format1_0: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *const u8,
                );
                let mut format2_0: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(6 as ::core::ffi::c_int as isize)
                        as *const u8,
                );
                let mut len1_0: u8 = position_format_length(format1_0);
                let mut len2_0: u8 = position_format_length(format2_0);
                let mut cov_0: *mut otl_Coverage =
                    readCoverage(
                        data as *const u8,
                        tableLength,
                        offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const u8,
                        ) as u32),
                    );
                (*subtable).first = readClassDef(
                    data as *const u8,
                    tableLength,
                    offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as u32),
                );
                (*subtable).first = expandClassDef(
                    cov_0,
                    (*subtable).first,
                );
                otl_Coverage_free(cov_0);
                cov_0 = ::core::ptr::null_mut::<otl_Coverage>();
                (*subtable).second = readClassDef(
                    data as *const u8,
                    tableLength,
                    offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(10 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as u32),
                );
                if !((*subtable).first.is_null() || (*subtable).second.is_null()) {
                    let mut class1Count: glyphclass_t = read_16u(
                        data.offset(offset as isize)
                            .offset(12 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as glyphclass_t;
                    let mut class2Count_0: glyphclass_t = read_16u(
                        data.offset(offset as isize)
                            .offset(14 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as glyphclass_t;
                    if !((*(*subtable).first).maxclass as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        != class1Count as ::core::ffi::c_int)
                    {
                        if !((*(*subtable).second).maxclass as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            != class2Count_0 as ::core::ffi::c_int)
                        {
                            if !(tableLength
                                < offset.wrapping_add(16 as u32).wrapping_add(
                                    (class1Count as ::core::ffi::c_int
                                        * class2Count_0 as ::core::ffi::c_int
                                        * (len1_0 as ::core::ffi::c_int
                                            + len2_0 as ::core::ffi::c_int))
                                        as u32,
                                ))
                            {
                                (*subtable).firstValues = __caryll_allocate_clean(
                                    (::core::mem::size_of::<*mut otl_PositionValue>() as usize)
                                        .wrapping_mul(class1Count as usize),
                                    153 as ::core::ffi::c_ulong,
                                )
                                    as *mut *mut otl_PositionValue;
                                (*subtable).secondValues = __caryll_allocate_clean(
                                    (::core::mem::size_of::<*mut otl_PositionValue>() as usize)
                                        .wrapping_mul(class1Count as usize),
                                    154 as ::core::ffi::c_ulong,
                                )
                                    as *mut *mut otl_PositionValue;
                                let mut j_4: glyphclass_t = 0 as glyphclass_t;
                                while (j_4 as ::core::ffi::c_int)
                                    < class1Count as ::core::ffi::c_int
                                {
                                    let ref mut fresh6 =
                                        *(*subtable).firstValues.offset(j_4 as isize);
                                    *fresh6 = __caryll_allocate_clean(
                                        (::core::mem::size_of::<otl_PositionValue>() as usize)
                                            .wrapping_mul(class2Count_0 as usize),
                                        157 as ::core::ffi::c_ulong,
                                    )
                                        as *mut otl_PositionValue;
                                    let ref mut fresh7 =
                                        *(*subtable).secondValues.offset(j_4 as isize);
                                    *fresh7 = __caryll_allocate_clean(
                                        (::core::mem::size_of::<otl_PositionValue>() as usize)
                                            .wrapping_mul(class2Count_0 as usize),
                                        158 as ::core::ffi::c_ulong,
                                    )
                                        as *mut otl_PositionValue;
                                    let mut k_2: glyphclass_t = 0 as glyphclass_t;
                                    while (k_2 as ::core::ffi::c_int)
                                        < class2Count_0 as ::core::ffi::c_int
                                    {
                                        *(*(*subtable).firstValues.offset(j_4 as isize))
                                            .offset(k_2 as isize) = read_gpos_value(
                                            data,
                                            tableLength,
                                            offset.wrapping_add(16 as u32).wrapping_add(
                                                ((j_4 as ::core::ffi::c_int
                                                    * class2Count_0 as ::core::ffi::c_int
                                                    + k_2 as ::core::ffi::c_int)
                                                    * (len1_0 as ::core::ffi::c_int
                                                        + len2_0 as ::core::ffi::c_int))
                                                    as u32,
                                            ),
                                            format1_0,
                                        );
                                        *(*(*subtable).secondValues.offset(j_4 as isize))
                                            .offset(k_2 as isize) = read_gpos_value(
                                            data,
                                            tableLength,
                                            offset
                                                .wrapping_add(16 as u32)
                                                .wrapping_add(
                                                    ((j_4 as ::core::ffi::c_int
                                                        * class2Count_0 as ::core::ffi::c_int
                                                        + k_2 as ::core::ffi::c_int)
                                                        * (len1_0 as ::core::ffi::c_int
                                                            + len2_0 as ::core::ffi::c_int))
                                                        as u32,
                                                )
                                                .wrapping_add(len1_0 as u32),
                                            format2_0,
                                        );
                                        k_2 = k_2.wrapping_add(1);
                                    }
                                    j_4 = j_4.wrapping_add(1);
                                }
                                return subtable as *mut otl_Subtable;
                            }
                        }
                    }
                }
            }
        }
    }
    iSubtable_gpos_pair.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_dump_pair(mut _subtable: *const otl_Subtable) -> *mut json_value {
    let mut subtable: *const subtable_gpos_pair = &raw const (*_subtable).gpos_pair;
    let mut st: *mut json_value = json_object_new(3 as usize);
    json_object_push(
        st,
        b"first\0" as *const u8 as *const ::core::ffi::c_char,
        otl_iClassDef.dump.expect("non-null function pointer")((*subtable).first),
    );
    json_object_push(
        st,
        b"second\0" as *const u8 as *const ::core::ffi::c_char,
        otl_iClassDef.dump.expect("non-null function pointer")((*subtable).second),
    );
    let mut mat: *mut json_value = json_array_new(
        ((*(*subtable).first).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
    );
    let mut j: glyphclass_t = 0 as glyphclass_t;
    while j as ::core::ffi::c_int <= (*(*subtable).first).maxclass as ::core::ffi::c_int {
        let mut row: *mut json_value = json_array_new(
            ((*(*subtable).second).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as usize,
        );
        let mut k: glyphclass_t = 0 as glyphclass_t;
        while k as ::core::ffi::c_int <= (*(*subtable).second).maxclass as ::core::ffi::c_int {
            let mut f1: u8 = required_position_format(
                *(*(*subtable).firstValues.offset(j as isize)).offset(k as isize),
            );
            let mut f2: u8 = required_position_format(
                *(*(*subtable).secondValues.offset(j as isize)).offset(k as isize),
            );
            if f1 as ::core::ffi::c_int | f2 as ::core::ffi::c_int != 0 {
                if f1 as ::core::ffi::c_int == FORMAT_DWIDTH as ::core::ffi::c_int
                    && f2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    json_array_push(
                        row,
                        json_new_position(
                            (*(*(*subtable).firstValues.offset(j as isize)).offset(k as isize))
                                .dWidth,
                        ),
                    );
                } else {
                    let mut pair: *mut json_value = json_object_new(2 as usize);
                    if f1 != 0 {
                        json_object_push(
                            pair,
                            b"first\0" as *const u8 as *const ::core::ffi::c_char,
                            gpos_dump_value(
                                *(*(*subtable).firstValues.offset(j as isize)).offset(k as isize),
                            ),
                        );
                    }
                    if f2 != 0 {
                        json_object_push(
                            pair,
                            b"second\0" as *const u8 as *const ::core::ffi::c_char,
                            gpos_dump_value(
                                *(*(*subtable).secondValues.offset(j as isize)).offset(k as isize),
                            ),
                        );
                    }
                    json_array_push(row, pair);
                }
            } else {
                json_array_push(row, json_new_position(0 as ::core::ffi::c_int as pos_t));
            }
            k = k.wrapping_add(1);
        }
        json_array_push(mat, preserialize(row));
        j = j.wrapping_add(1);
    }
    json_object_push(
        st,
        b"matrix\0" as *const u8 as *const ::core::ffi::c_char,
        mat,
    );
    return st;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_parse_pair(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut class1Count: glyphclass_t = 0;
    let mut class2Count: glyphclass_t = 0;
    let mut subtable: *mut subtable_gpos_pair =
        (
            iSubtable_gpos_pair
                .create
                .expect("non-null function pointer"))();
    let mut _mat: *mut json_value = json_obj_get_type(
        _subtable,
        b"matrix\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    (*subtable).first = otl_iClassDef.parse.expect("non-null function pointer")(json_obj_get_type(
        _subtable,
        b"first\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    ));
    (*subtable).second =
        otl_iClassDef.parse.expect("non-null function pointer")(json_obj_get_type(
            _subtable,
            b"second\0" as *const u8 as *const ::core::ffi::c_char,
            json_object,
        ));
    if _mat.is_null() || (*subtable).first.is_null() || (*subtable).second.is_null() {
        iSubtable_gpos_pair.free.expect("non-null function pointer")(subtable);
        return ::core::ptr::null_mut::<otl_Subtable>();
    } else {
        class1Count = ((*(*subtable).first).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as glyphclass_t;
        class2Count = ((*(*subtable).second).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as glyphclass_t;
        (*subtable).firstValues = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut otl_PositionValue>() as usize)
                .wrapping_mul(class1Count as usize),
            224 as ::core::ffi::c_ulong,
        ) as *mut *mut otl_PositionValue;
        (*subtable).secondValues = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut otl_PositionValue>() as usize)
                .wrapping_mul(class1Count as usize),
            225 as ::core::ffi::c_ulong,
        ) as *mut *mut otl_PositionValue;
        let mut j: glyphclass_t = 0 as glyphclass_t;
        while (j as ::core::ffi::c_int) < class1Count as ::core::ffi::c_int {
            let ref mut fresh8 = *(*subtable).firstValues.offset(j as isize);
            *fresh8 = __caryll_allocate_clean(
                (::core::mem::size_of::<otl_PositionValue>() as usize)
                    .wrapping_mul(class2Count as usize),
                228 as ::core::ffi::c_ulong,
            ) as *mut otl_PositionValue;
            let ref mut fresh9 = *(*subtable).secondValues.offset(j as isize);
            *fresh9 = __caryll_allocate_clean(
                (::core::mem::size_of::<otl_PositionValue>() as usize)
                    .wrapping_mul(class2Count as usize),
                229 as ::core::ffi::c_ulong,
            ) as *mut otl_PositionValue;
            let mut k: glyphclass_t = 0 as glyphclass_t;
            while (k as ::core::ffi::c_int) < class2Count as ::core::ffi::c_int {
                *(*(*subtable).firstValues.offset(j as isize)).offset(k as isize) = position_zero();
                *(*(*subtable).secondValues.offset(j as isize)).offset(k as isize) =
                    position_zero();
                k = k.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        let mut j_0: glyphclass_t = 0 as glyphclass_t;
        while (j_0 as ::core::ffi::c_int) < class1Count as ::core::ffi::c_int
            && (j_0 as ::core::ffi::c_uint) < (*_mat).u.array.length
        {
            let mut _row: *mut json_value =
                *(*_mat).u.array.values.offset(j_0 as isize) as *mut json_value;
            if !(_row.is_null()
                || (*_row).type_0 as ::core::ffi::c_uint
                    != json_array as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                let mut k_0: glyphclass_t = 0 as glyphclass_t;
                while (k_0 as ::core::ffi::c_int) < class2Count as ::core::ffi::c_int
                    && (k_0 as ::core::ffi::c_uint) < (*_row).u.array.length
                {
                    let mut _item: *mut json_value =
                        *(*_row).u.array.values.offset(k_0 as isize) as *mut json_value;
                    if (*_item).type_0 as ::core::ffi::c_uint
                        == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*(*(*subtable).firstValues.offset(j_0 as isize)).offset(k_0 as isize))
                            .dWidth = (*_item).u.integer as pos_t;
                    } else if (*_item).type_0 as ::core::ffi::c_uint
                        == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*(*(*subtable).firstValues.offset(j_0 as isize)).offset(k_0 as isize))
                            .dWidth = (*_item).u.dbl as pos_t;
                    } else if (*_item).type_0 as ::core::ffi::c_uint
                        == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        *(*(*subtable).firstValues.offset(j_0 as isize)).offset(k_0 as isize) =
                            gpos_parse_value(json_obj_get(
                                _item,
                                b"first\0" as *const u8 as *const ::core::ffi::c_char,
                            ));
                        *(*(*subtable).secondValues.offset(j_0 as isize)).offset(k_0 as isize) =
                            gpos_parse_value(json_obj_get(
                                _item,
                                b"second\0" as *const u8 as *const ::core::ffi::c_char,
                            ));
                    }
                    k_0 = k_0.wrapping_add(1);
                }
            }
            j_0 = j_0.wrapping_add(1);
        }
        return subtable as *mut otl_Subtable;
    };
}
unsafe extern "C" fn covFromCD(mut cd: *mut otl_ClassDef) -> *mut otl_Coverage {
    let mut cov: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    cov = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_Coverage>() as usize,
        257 as ::core::ffi::c_ulong,
    ) as *mut otl_Coverage;
    (*cov).numGlyphs = (*cd).numGlyphs;
    (*cov).glyphs = __caryll_allocate_clean(
        (::core::mem::size_of::<otfcc_GlyphHandle>() as usize)
            .wrapping_mul((*cd).numGlyphs as usize),
        259 as ::core::ffi::c_ulong,
    ) as *mut otfcc_GlyphHandle;
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*cd).numGlyphs as ::core::ffi::c_int {
        *(*cov).glyphs.offset(j as isize) = otfcc_Handle_dup(
            *(*cd).glyphs.offset(j as isize) as otfcc_Handle,
        ) as otfcc_GlyphHandle;
        j = j.wrapping_add(1);
    }
    return cov;
}
unsafe extern "C" fn by_pairSecondGlyph(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut IndividualGposPair)).gid as ::core::ffi::c_int
        - (*(b as *mut IndividualGposPair)).gid as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gpos_pair_individual(
    mut _subtable: *const otl_Subtable,
) -> *mut bk_Block {
    let mut subtable: *const subtable_gpos_pair = &raw const (*_subtable).gpos_pair;
    let mut format1: u16 = 0 as u16;
    let mut format2: u16 = 0 as u16;
    let mut class1Count: glyphclass_t = ((*(*subtable).first).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as glyphclass_t;
    let mut class2Count: glyphclass_t = ((*(*subtable).second).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as glyphclass_t;
    let mut j: glyphclass_t = 0 as glyphclass_t;
    while (j as ::core::ffi::c_int) < class1Count as ::core::ffi::c_int {
        let mut k: glyphclass_t = 0 as glyphclass_t;
        while (k as ::core::ffi::c_int) < class2Count as ::core::ffi::c_int {
            format1 = (format1 as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).firstValues.offset(j as isize)).offset(k as isize),
                ) as ::core::ffi::c_int) as u16;
            format2 = (format2 as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).secondValues.offset(j as isize)).offset(k as isize),
                ) as ::core::ffi::c_int) as u16;
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let mut pairCounts: *mut glyphid_t = ::core::ptr::null_mut::<glyphid_t>();
    pairCounts = __caryll_allocate_clean(
        (::core::mem::size_of::<glyphid_t>() as usize)
            .wrapping_mul((*(*subtable).first).numGlyphs as usize),
        290 as ::core::ffi::c_ulong,
    ) as *mut glyphid_t;
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as ::core::ffi::c_int) < (*(*subtable).first).numGlyphs as ::core::ffi::c_int {
        *pairCounts.offset(j_0 as isize) = 0 as glyphid_t;
        let mut k_0: glyphid_t = 0 as glyphid_t;
        while (k_0 as ::core::ffi::c_int) < (*(*subtable).second).numGlyphs as ::core::ffi::c_int {
            let mut c1: glyphclass_t = *(*(*subtable).first).classes.offset(j_0 as isize);
            let mut c2: glyphclass_t = *(*(*subtable).second).classes.offset(k_0 as isize);
            if required_position_format(
                *(*(*subtable).firstValues.offset(c1 as isize)).offset(c2 as isize),
            ) as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).secondValues.offset(c1 as isize)).offset(c2 as isize),
                ) as ::core::ffi::c_int
                != 0
            {
                let ref mut fresh10 = *pairCounts.offset(j_0 as isize);
                *fresh10 = (*fresh10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
            }
            k_0 = k_0.wrapping_add(1);
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut cov: *mut otl_Coverage = covFromCD((*subtable).first);
    shrinkCoverage(cov, true);
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(cov)),
        b16 as ::core::ffi::c_int,
        format1 as ::core::ffi::c_int,
        b16 as ::core::ffi::c_int,
        format2 as ::core::ffi::c_int,
        b16 as ::core::ffi::c_int,
        (*(*subtable).first).numGlyphs as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j_1: glyphid_t = 0 as glyphid_t;
    while (j_1 as ::core::ffi::c_int) < (*cov).numGlyphs as ::core::ffi::c_int {
        let mut currentPairCount: tableid_t = 0 as tableid_t;
        let mut c1_0: glyphclass_t = 0 as glyphclass_t;
        let mut k_1: glyphid_t = 0 as glyphid_t;
        while (k_1 as ::core::ffi::c_int) < (*(*subtable).first).numGlyphs as ::core::ffi::c_int {
            if (*(*(*subtable).first).glyphs.offset(k_1 as isize)).index as ::core::ffi::c_int
                == (*(*cov).glyphs.offset(j_1 as isize)).index as ::core::ffi::c_int
            {
                c1_0 = *(*(*subtable).first).classes.offset(k_1 as isize);
                currentPairCount = *pairCounts.offset(k_1 as isize) as tableid_t;
            }
            k_1 = k_1.wrapping_add(1);
        }
        let mut pairSet: *mut bk_Block = bk_new_Block(
            b16 as ::core::ffi::c_int,
            currentPairCount as ::core::ffi::c_int,
            bkover as ::core::ffi::c_int,
        );
        let mut pairs: *mut IndividualGposPair = ::core::ptr::null_mut::<IndividualGposPair>();
        pairs = __caryll_allocate_clean(
            (::core::mem::size_of::<IndividualGposPair>() as usize)
                .wrapping_mul(currentPairCount as usize),
            324 as ::core::ffi::c_ulong,
        ) as *mut IndividualGposPair;
        let mut n: usize = 0 as usize;
        let mut k_2: glyphid_t = 0 as glyphid_t;
        while (k_2 as ::core::ffi::c_int) < (*(*subtable).second).numGlyphs as ::core::ffi::c_int {
            let mut c2_0: glyphclass_t = *(*(*subtable).second).classes.offset(k_2 as isize);
            if required_position_format(
                *(*(*subtable).firstValues.offset(c1_0 as isize)).offset(c2_0 as isize),
            ) as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).secondValues.offset(c1_0 as isize)).offset(c2_0 as isize),
                ) as ::core::ffi::c_int
                != 0
            {
                (*pairs.offset(n as isize)).gid =
                    (*(*(*subtable).second).glyphs.offset(k_2 as isize)).index;
                let ref mut fresh11 = (*pairs.offset(n as isize)).fv;
                *fresh11 = (*(*subtable).firstValues.offset(c1_0 as isize)).offset(c2_0 as isize)
                    as *mut otl_PositionValue;
                let ref mut fresh12 = (*pairs.offset(n as isize)).sv;
                *fresh12 = (*(*subtable).secondValues.offset(c1_0 as isize)).offset(c2_0 as isize)
                    as *mut otl_PositionValue;
                n = n.wrapping_add(1);
            }
            k_2 = k_2.wrapping_add(1);
        }
        qsort(
            pairs as *mut ::core::ffi::c_void,
            currentPairCount as usize,
            ::core::mem::size_of::<IndividualGposPair>() as usize,
            Some(
                by_pairSecondGlyph
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut n_0: usize = 0 as usize;
        while n_0 < currentPairCount as usize {
            bk_push(
                pairSet,
                b16 as ::core::ffi::c_int,
                (*pairs.offset(n_0 as isize)).gid as ::core::ffi::c_int,
                bkembed as ::core::ffi::c_int,
                bk_gpos_value(*(*pairs.offset(n_0 as isize)).fv, format1),
                bkembed as ::core::ffi::c_int,
                bk_gpos_value(*(*pairs.offset(n_0 as isize)).sv, format2),
                bkover as ::core::ffi::c_int,
            );
            n_0 = n_0.wrapping_add(1);
        }
        free(pairs as *mut ::core::ffi::c_void);
        pairs = ::core::ptr::null_mut::<IndividualGposPair>();
        bk_push(
            root,
            p16 as ::core::ffi::c_int,
            pairSet,
            bkover as ::core::ffi::c_int,
        );
        j_1 = j_1.wrapping_add(1);
    }
    otl_Coverage_free(cov);
    cov = ::core::ptr::null_mut::<otl_Coverage>();
    free(pairCounts as *mut ::core::ffi::c_void);
    pairCounts = ::core::ptr::null_mut::<glyphid_t>();
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gpos_pair_classes(
    mut _subtable: *const otl_Subtable,
) -> *mut bk_Block {
    let mut subtable: *const subtable_gpos_pair = &raw const (*_subtable).gpos_pair;
    let mut format1: u16 = 0 as u16;
    let mut format2: u16 = 0 as u16;
    let mut class1Count: glyphclass_t = ((*(*subtable).first).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as glyphclass_t;
    let mut class2Count: glyphclass_t = ((*(*subtable).second).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as glyphclass_t;
    let mut j: glyphclass_t = 0 as glyphclass_t;
    while (j as ::core::ffi::c_int) < class1Count as ::core::ffi::c_int {
        let mut k: glyphclass_t = 0 as glyphclass_t;
        while (k as ::core::ffi::c_int) < class2Count as ::core::ffi::c_int {
            format1 = (format1 as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).firstValues.offset(j as isize)).offset(k as isize),
                ) as ::core::ffi::c_int) as u16;
            format2 = (format2 as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).secondValues.offset(j as isize)).offset(k as isize),
                ) as ::core::ffi::c_int) as u16;
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let mut cov: *mut otl_Coverage = covFromCD((*subtable).first);
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iCoverage.build.expect("non-null function pointer")(cov)),
        b16 as ::core::ffi::c_int,
        format1 as ::core::ffi::c_int,
        b16 as ::core::ffi::c_int,
        format2 as ::core::ffi::c_int,
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
            (*subtable).first,
        )),
        p16 as ::core::ffi::c_int,
        bk_newBlockFromBuffer(otl_iClassDef.build.expect("non-null function pointer")(
            (*subtable).second,
        )),
        b16 as ::core::ffi::c_int,
        class1Count as ::core::ffi::c_int,
        b16 as ::core::ffi::c_int,
        class2Count as ::core::ffi::c_int,
        bkover as ::core::ffi::c_int,
    );
    let mut j_0: glyphclass_t = 0 as glyphclass_t;
    while (j_0 as ::core::ffi::c_int) < class1Count as ::core::ffi::c_int {
        let mut k_0: glyphclass_t = 0 as glyphclass_t;
        while (k_0 as ::core::ffi::c_int) < class2Count as ::core::ffi::c_int {
            bk_push(
                root,
                bkembed as ::core::ffi::c_int,
                bk_gpos_value(
                    *(*(*subtable).firstValues.offset(j_0 as isize)).offset(k_0 as isize),
                    format1,
                ),
                bkembed as ::core::ffi::c_int,
                bk_gpos_value(
                    *(*(*subtable).secondValues.offset(j_0 as isize)).offset(k_0 as isize),
                    format2,
                ),
                bkover as ::core::ffi::c_int,
            );
            k_0 = k_0.wrapping_add(1);
        }
        j_0 = j_0.wrapping_add(1);
    }
    otl_Coverage_free(cov);
    cov = ::core::ptr::null_mut::<otl_Coverage>();
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gpos_pair(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut format1: *mut bk_Block = otfcc_build_gpos_pair_individual(_subtable);
    let mut format2: *mut bk_Block = otfcc_build_gpos_pair_classes(_subtable);
    let mut g1: *mut bk_Graph = bk_newGraphFromRootBlock(format1);
    let mut g2: *mut bk_Graph = bk_newGraphFromRootBlock(format2);
    bk_minimizeGraph(g1);
    bk_minimizeGraph(g2);
    if bk_estimateSizeOfGraph(g1) > bk_estimateSizeOfGraph(g2) {
        bk_delete_Graph(g1);
        bk_untangleGraph(g2);
        let mut buf: *mut caryll_Buffer = bk_build_Graph(g2);
        bk_delete_Graph(g2);
        return buf;
    } else {
        bk_delete_Graph(g2);
        bk_untangleGraph(g1);
        let mut buf_0: *mut caryll_Buffer = bk_build_Graph(g1);
        bk_delete_Graph(g1);
        return buf_0;
    };
}
