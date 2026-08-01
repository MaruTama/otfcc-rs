#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memcpy, memset, qsort};
use crate::support::json_funcs::{json_new_position, json_obj_get, json_obj_get_type, preserialize};
use crate::table::otl::classdef::{expand_class_def, ClassDef, otl_class_def_free, read_class_def};
use crate::table::otl::coverage::{Coverage, otl_coverage_free, read_coverage, shrink_coverage};
use crate::support::handle::{handle_from_index, otfcc_handle_dup, Handle, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId, Pos, TableId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::bkgraph::{BkGraph};
use crate::support::{NULL};
use crate::table::otl::{GposPairSubtableElementInterface, PositionValue, Subtable, GposPairSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_graph, bk_delete_graph, bk_estimate_size_of_graph, bk_minimize_graph, bk_new_graph_from_root_block, bk_untangle_graph};
use crate::table::otl::classdef::{OTL_I_CLASS_DEF};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::table::otl::subtables::gpos_common::{FORMAT_DWIDTH, bk_gpos_value, gpos_dump_value, gpos_parse_value, position_format_length, position_zero, read_gpos_value, required_position_format};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_object_new, json_object_push};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PairClassifierHash {
    pub gid: ::core::ffi::c_int,
    pub cid: ::core::ffi::c_int,
    pub hh: UtHashHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndividualGposPair {
    pub gid: GlyphId,
    pub fv: *mut PositionValue,
    pub sv: *mut PositionValue,
}
#[inline]
unsafe extern "C" fn init_gpos_pair(mut subtable: *mut GposPairSubtable) {
    (*subtable).first = ::core::ptr::null_mut::<ClassDef>();
    (*subtable).second = ::core::ptr::null_mut::<ClassDef>();
    (*subtable).first_values = ::core::ptr::null_mut::<*mut PositionValue>();
    (*subtable).second_values = ::core::ptr::null_mut::<*mut PositionValue>();
}
#[inline]
unsafe extern "C" fn dispose_gpos_pair(mut subtable: *mut GposPairSubtable) {
    if !(*subtable).first_values.is_null() {
        let mut j: GlyphClass = 0 as GlyphClass;
        while j as ::core::ffi::c_int <= (*(*subtable).first).maxclass as ::core::ffi::c_int {
            free(*(*subtable).first_values.offset(j as isize) as *mut ::core::ffi::c_void);
            let ref mut fresh0 = *(*subtable).first_values.offset(j as isize);
            *fresh0 = ::core::ptr::null_mut::<PositionValue>();
            j = j.wrapping_add(1);
        }
        free((*subtable).first_values as *mut ::core::ffi::c_void);
        (*subtable).first_values = ::core::ptr::null_mut::<*mut PositionValue>();
    }
    if !(*subtable).second_values.is_null() {
        let mut j_0: GlyphClass = 0 as GlyphClass;
        while j_0 as ::core::ffi::c_int <= (*(*subtable).first).maxclass as ::core::ffi::c_int {
            free(*(*subtable).second_values.offset(j_0 as isize) as *mut ::core::ffi::c_void);
            let ref mut fresh1 = *(*subtable).second_values.offset(j_0 as isize);
            *fresh1 = ::core::ptr::null_mut::<PositionValue>();
            j_0 = j_0.wrapping_add(1);
        }
        free((*subtable).second_values as *mut ::core::ffi::c_void);
        (*subtable).second_values = ::core::ptr::null_mut::<*mut PositionValue>();
    }
    otl_class_def_free((*subtable).first);
    (*subtable).first = ::core::ptr::null_mut::<ClassDef>();
    otl_class_def_free((*subtable).second);
    (*subtable).second = ::core::ptr::null_mut::<ClassDef>();
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_dispose(mut x: *mut GposPairSubtable) {
    dispose_gpos_pair(x);
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_copy(
    mut dst: *mut GposPairSubtable,
    mut src: *const GposPairSubtable,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GposPairSubtable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_create() -> *mut GposPairSubtable {
    let mut x: *mut GposPairSubtable =
        malloc(::core::mem::size_of::<GposPairSubtable>() as usize) as *mut GposPairSubtable;
    subtable_gpos_pair_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn subtable_gpos_pair_init(mut x: *mut GposPairSubtable) {
    init_gpos_pair(x);
}
pub static I_SUBTABLE_GPOS_PAIR: GposPairSubtableElementInterface = {
    GposPairSubtableElementInterface {
        init: Some(subtable_gpos_pair_init as unsafe extern "C" fn(*mut GposPairSubtable) -> ()),
        copy: Some(
            subtable_gpos_pair_copy
                as unsafe extern "C" fn(*mut GposPairSubtable, *const GposPairSubtable) -> (),
        ),
        dispose: Some(
            subtable_gpos_pair_dispose as unsafe extern "C" fn(*mut GposPairSubtable) -> (),
        ),
        create: Some(subtable_gpos_pair_create),
        free: Some(subtable_gpos_pair_free as unsafe extern "C" fn(*mut GposPairSubtable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_pair_free(mut x: *mut GposPairSubtable) {
    if x.is_null() {
        return;
    }
    subtable_gpos_pair_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otl_read_gpos_pair(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut subtable_format: u16 = 0;
    let mut current_block: u64;
    let mut subtable: *mut GposPairSubtable =
        (
            I_SUBTABLE_GPOS_PAIR
                .create
                .expect("non-null function pointer"))();
    if !(table_length < offset.wrapping_add(2 as u32)) {
        subtable_format = read_16u(data.offset(offset as isize) as *const u8);
        if subtable_format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            let mut cov: *mut Coverage = read_coverage(
                data as *const u8,
                table_length,
                offset.wrapping_add(read_16u(
                    data.offset(offset as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as u32),
            );
            (*subtable).first = __caryll_allocate_clean(
                ::core::mem::size_of::<ClassDef>() as usize,
                45 as ::core::ffi::c_ulong,
            ) as *mut ClassDef;
            (*(*subtable).first).num_glyphs = (*cov).num_glyphs;
            (*(*subtable).first).maxclass =
                ((*cov).num_glyphs as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as GlyphClass;
            (*(*subtable).first).glyphs = (*cov).glyphs;
            (*(*subtable).first).classes = __caryll_allocate_clean(
                (::core::mem::size_of::<GlyphClass>() as usize)
                    .wrapping_mul((*cov).num_glyphs as usize),
                49 as ::core::ffi::c_ulong,
            ) as *mut GlyphClass;
            let mut j: GlyphId = 0 as GlyphId;
            while (j as ::core::ffi::c_int) < (*cov).num_glyphs as ::core::ffi::c_int {
                *(*(*subtable).first).classes.offset(j as isize) = j as GlyphClass;
                j = j.wrapping_add(1);
            }
            free(cov as *mut ::core::ffi::c_void);
            cov = ::core::ptr::null_mut::<Coverage>();
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
            let mut pair_set_count: GlyphId = read_16u(
                data.offset(offset as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *const u8,
            ) as GlyphId;
            if !(pair_set_count as ::core::ffi::c_int
                != (*(*subtable).first).num_glyphs as ::core::ffi::c_int)
            {
                if !(table_length
                    < offset.wrapping_add(10 as u32).wrapping_add(
                        (2 as ::core::ffi::c_int * pair_set_count as ::core::ffi::c_int) as u32,
                    ))
                {
                    let mut j_0: GlyphId = 0 as GlyphId;
                    loop {
                        if !((j_0 as ::core::ffi::c_int) < pair_set_count as ::core::ffi::c_int) {
                            current_block = 14401909646449704462;
                            break;
                        }
                        let mut pair_set_offset: u32 = offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(10 as ::core::ffi::c_int as isize)
                                .offset(
                                    (2 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        if table_length < pair_set_offset.wrapping_add(2 as u32) {
                            current_block = 1524425613423851471;
                            break;
                        }
                        let mut pair_count: GlyphId =
                            read_16u(data.offset(pair_set_offset as isize) as *const u8)
                                as GlyphId;
                        if table_length
                            < pair_set_offset.wrapping_add(2 as u32).wrapping_add(
                                ((2 as ::core::ffi::c_int
                                    + len1 as ::core::ffi::c_int
                                    + len2 as ::core::ffi::c_int)
                                    * pair_count as ::core::ffi::c_int)
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
                            let mut h: *mut PairClassifierHash =
                                ::core::ptr::null_mut::<PairClassifierHash>();
                            let mut j_1: GlyphId = 0 as GlyphId;
                            while (j_1 as ::core::ffi::c_int) < pair_set_count as ::core::ffi::c_int {
                                let mut pair_set_offset_0: u32 = offset.wrapping_add(read_16u(
                                    data.offset(offset as isize)
                                        .offset(10 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int * j_1 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let mut pair_count_0: GlyphId = read_16u(
                                    data.offset(pair_set_offset_0 as isize) as *const u8,
                                )
                                    as GlyphId;
                                let mut k: GlyphId = 0 as GlyphId;
                                while (k as ::core::ffi::c_int) < pair_count_0 as ::core::ffi::c_int
                                {
                                    let mut second: ::core::ffi::c_int = read_16u(
                                        data.offset(pair_set_offset_0 as isize)
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
                                    let mut s: *mut PairClassifierHash =
                                        ::core::ptr::null_mut::<PairClassifierHash>();
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
                                    s = ::core::ptr::null_mut::<PairClassifierHash>();
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
                                                    as *mut PairClassifierHash
                                                    as *mut PairClassifierHash;
                                            } else {
                                                s = ::core::ptr::null_mut::<PairClassifierHash>();
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
                                                        as *mut PairClassifierHash
                                                        as *mut PairClassifierHash;
                                                } else {
                                                    s = ::core::ptr::null_mut::<PairClassifierHash>(
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    if s.is_null() {
                                        s = __caryll_allocate_clean(
                                            ::core::mem::size_of::<PairClassifierHash>()
                                                as usize,
                                            81 as ::core::ffi::c_ulong,
                                        )
                                            as *mut PairClassifierHash;
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
                                                malloc(::core::mem::size_of::<UtHashTable>()
                                                    as usize)
                                                    as *mut UtHashTable
                                                    as *mut UtHashTable;
                                            if (*s).hh.tbl.is_null() {
                                                exit(-(1 as ::core::ffi::c_int));
                                            } else {
                                                memset(
                                                    (*s).hh.tbl as *mut ::core::ffi::c_void,
                                                    '\0' as i32,
                                                    ::core::mem::size_of::<UtHashTable>()
                                                        as usize,
                                                );
                                                (*(*s).hh.tbl).tail =
                                                    &raw mut (*s).hh as *mut UtHashHandle;
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
                                                        ::core::mem::size_of::<UtHashBucket>()
                                                            as usize,
                                                    ))
                                                        as *mut UtHashBucket;
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
                                                            ::core::mem::size_of::<UtHashBucket>()
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
                                                &raw mut (*s).hh as *mut UtHashHandle;
                                        }
                                        let mut _ha_bkt: ::core::ffi::c_uint = 0;
                                        (*(*h).hh.tbl).num_items =
                                            (*(*h).hh.tbl).num_items.wrapping_add(1);
                                        _ha_bkt = _ha_hashv
                                            & (*(*h).hh.tbl)
                                                .num_buckets
                                                .wrapping_sub(1 as ::core::ffi::c_uint);
                                        let mut _ha_head: *mut UtHashBucket =
                                            (*(*h).hh.tbl).buckets.offset(_ha_bkt as isize)
                                                as *mut UtHashBucket;
                                        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                                        (*s).hh.hh_next =
                                            (*_ha_head).hh_head as *mut UtHashHandle;
                                        (*s).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                                        if !(*_ha_head).hh_head.is_null() {
                                            (*(*_ha_head).hh_head).hh_prev =
                                                &raw mut (*s).hh as *mut UtHashHandle;
                                        }
                                        (*_ha_head).hh_head =
                                            &raw mut (*s).hh as *mut UtHashHandle;
                                        if (*_ha_head).count
                                            >= (*_ha_head)
                                                .expand_mult
                                                .wrapping_add(1 as ::core::ffi::c_uint)
                                                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                                            && (*(*s).hh.tbl).noexpand == 0
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
                                                    .wrapping_mul(
                                                        (*(*s).hh.tbl).num_buckets as usize,
                                                    )
                                                    .wrapping_mul(::core::mem::size_of::<
                                                        UtHashBucket,
                                                    >(
                                                    )
                                                        as usize),
                                            )
                                                as *mut UtHashBucket;
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
                                                            UtHashBucket,
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
                                                        as *mut UtHashHandle;
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
                                                            as *mut UtHashBucket;
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
                                                            UtHashHandle,
                                                        >(
                                                        );
                                                        (*_he_thh).hh_next = (*_he_newbkt).hh_head
                                                            as *mut UtHashHandle;
                                                        if !(*_he_newbkt).hh_head.is_null() {
                                                            (*(*_he_newbkt).hh_head).hh_prev =
                                                                _he_thh;
                                                        }
                                                        (*_he_newbkt).hh_head =
                                                            _he_thh as *mut UtHashHandle;
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
                                ::core::mem::size_of::<ClassDef>() as usize,
                                89 as ::core::ffi::c_ulong,
                            ) as *mut ClassDef;
                            (*(*subtable).second).num_glyphs = (if !h.is_null() {
                                (*(*h).hh.tbl).num_items
                            } else {
                                0 as ::core::ffi::c_uint
                            })
                                as GlyphId;
                            (*(*subtable).second).maxclass = (if !h.is_null() {
                                (*(*h).hh.tbl).num_items
                            } else {
                                0 as ::core::ffi::c_uint
                            })
                                as GlyphClass;
                            (*(*subtable).second).classes = __caryll_allocate_clean(
                                (::core::mem::size_of::<GlyphClass>() as usize)
                                    .wrapping_mul((*(*subtable).second).num_glyphs as usize),
                                92 as ::core::ffi::c_ulong,
                            )
                                as *mut GlyphClass;
                            (*(*subtable).second).glyphs = __caryll_allocate_clean(
                                (::core::mem::size_of::<GlyphHandle>() as usize)
                                    .wrapping_mul((*(*subtable).second).num_glyphs as usize),
                                93 as ::core::ffi::c_ulong,
                            )
                                as *mut GlyphHandle;
                            let mut class2_count: GlyphClass = ((*(*subtable).second).maxclass
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as GlyphClass;
                            (*subtable).first_values = __caryll_allocate_clean(
                                (::core::mem::size_of::<*mut PositionValue>() as usize)
                                    .wrapping_mul(
                                        ((*(*subtable).first).maxclass as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int)
                                            as usize,
                                    ),
                                96 as ::core::ffi::c_ulong,
                            )
                                as *mut *mut PositionValue;
                            (*subtable).second_values = __caryll_allocate_clean(
                                (::core::mem::size_of::<*mut PositionValue>() as usize)
                                    .wrapping_mul(
                                        ((*(*subtable).first).maxclass as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int)
                                            as usize,
                                    ),
                                97 as ::core::ffi::c_ulong,
                            )
                                as *mut *mut PositionValue;
                            let mut j_2: GlyphClass = 0 as GlyphClass;
                            while j_2 as ::core::ffi::c_int
                                <= (*(*subtable).first).maxclass as ::core::ffi::c_int
                            {
                                let ref mut fresh2 = *(*subtable).first_values.offset(j_2 as isize);
                                *fresh2 = __caryll_allocate_clean(
                                    (::core::mem::size_of::<PositionValue>() as usize)
                                        .wrapping_mul(class2_count as usize),
                                    99 as ::core::ffi::c_ulong,
                                )
                                    as *mut PositionValue;
                                let ref mut fresh3 = *(*subtable).second_values.offset(j_2 as isize);
                                *fresh3 = __caryll_allocate_clean(
                                    (::core::mem::size_of::<PositionValue>() as usize)
                                        .wrapping_mul(class2_count as usize),
                                    100 as ::core::ffi::c_ulong,
                                )
                                    as *mut PositionValue;
                                let mut k_0: GlyphClass = 0 as GlyphClass;
                                while (k_0 as ::core::ffi::c_int)
                                    < class2_count as ::core::ffi::c_int
                                {
                                    *(*(*subtable).first_values.offset(j_2 as isize))
                                        .offset(k_0 as isize) = position_zero();
                                    *(*(*subtable).second_values.offset(j_2 as isize))
                                        .offset(k_0 as isize) = position_zero();
                                    k_0 = k_0.wrapping_add(1);
                                }
                                j_2 = j_2.wrapping_add(1);
                            }
                            let mut j_3: GlyphClass = 0 as GlyphClass;
                            while j_3 as ::core::ffi::c_int
                                <= (*(*subtable).first).maxclass as ::core::ffi::c_int
                            {
                                let mut pair_set_offset_1: u32 = offset.wrapping_add(read_16u(
                                    data.offset(offset as isize)
                                        .offset(10 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int * j_3 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let mut pair_count_1: GlyphId = read_16u(
                                    data.offset(pair_set_offset_1 as isize) as *const u8,
                                )
                                    as GlyphId;
                                let mut k_1: GlyphId = 0 as GlyphId;
                                while (k_1 as ::core::ffi::c_int)
                                    < pair_count_1 as ::core::ffi::c_int
                                {
                                    let mut second_0: ::core::ffi::c_int = read_16u(
                                        data.offset(pair_set_offset_1 as isize)
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
                                    let mut s_0: *mut PairClassifierHash =
                                        ::core::ptr::null_mut::<PairClassifierHash>();
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
                                    s_0 = ::core::ptr::null_mut::<PairClassifierHash>();
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
                                                    as *mut PairClassifierHash
                                                    as *mut PairClassifierHash;
                                            } else {
                                                s_0 =
                                                    ::core::ptr::null_mut::<PairClassifierHash>();
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
                                                        as *mut PairClassifierHash
                                                        as *mut PairClassifierHash;
                                                } else {
                                                    s_0 = ::core::ptr::null_mut::<
                                                        PairClassifierHash,
                                                    >(
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    if !s_0.is_null() {
                                        *(*(*subtable).first_values.offset(j_3 as isize))
                                            .offset((*s_0).cid as isize) = read_gpos_value(
                                            data,
                                            table_length,
                                            pair_set_offset_1
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
                                        *(*(*subtable).second_values.offset(j_3 as isize))
                                            .offset((*s_0).cid as isize) = read_gpos_value(
                                            data,
                                            table_length,
                                            pair_set_offset_1
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
                            let mut s_1: *mut PairClassifierHash =
                                ::core::ptr::null_mut::<PairClassifierHash>();
                            let mut tmp: *mut PairClassifierHash =
                                ::core::ptr::null_mut::<PairClassifierHash>();
                            let mut jj: GlyphId = 0 as GlyphId;
                            s_1 = h;
                            tmp = (if !h.is_null() { (*h).hh.next } else { NULL })
                                as *mut PairClassifierHash
                                as *mut PairClassifierHash;
                            while !s_1.is_null() {
                                *(*(*subtable).second).glyphs.offset(jj as isize) =
                                    handle_from_index(
                                        (*s_1).gid as GlyphId,
                                    ) as GlyphHandle;
                                *(*(*subtable).second).classes.offset(jj as isize) =
                                    (*s_1).cid as GlyphClass;
                                jj = jj.wrapping_add(1);
                                let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s_1).hh;
                                if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                                    free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
                                    free((*h).hh.tbl as *mut ::core::ffi::c_void);
                                    h = ::core::ptr::null_mut::<PairClassifierHash>();
                                } else {
                                    let mut _hd_bkt: ::core::ffi::c_uint = 0;
                                    if _hd_hh_del == (*(*h).hh.tbl).tail {
                                        (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev
                                            as *mut ::core::ffi::c_char)
                                            .offset((*(*h).hh.tbl).hho)
                                            as *mut UtHashHandle
                                            as *mut UtHashHandle;
                                    }
                                    if !(*_hd_hh_del).prev.is_null() {
                                        let ref mut fresh4 = (*(((*_hd_hh_del).prev
                                            as *mut ::core::ffi::c_char)
                                            .offset((*(*h).hh.tbl).hho)
                                            as *mut UtHashHandle))
                                            .next;
                                        *fresh4 = (*_hd_hh_del).next;
                                    } else {
                                        h = (*_hd_hh_del).next as *mut PairClassifierHash
                                            as *mut PairClassifierHash;
                                    }
                                    if !(*_hd_hh_del).next.is_null() {
                                        let ref mut fresh5 = (*(((*_hd_hh_del).next
                                            as *mut ::core::ffi::c_char)
                                            .offset((*(*h).hh.tbl).hho)
                                            as *mut UtHashHandle))
                                            .prev;
                                        *fresh5 = (*_hd_hh_del).prev;
                                    }
                                    _hd_bkt = (*_hd_hh_del).hashv
                                        & (*(*h).hh.tbl)
                                            .num_buckets
                                            .wrapping_sub(1 as ::core::ffi::c_uint);
                                    let mut _hd_head: *mut UtHashBucket =
                                        (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize)
                                            as *mut UtHashBucket;
                                    (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
                                    if (*_hd_head).hh_head == _hd_hh_del {
                                        (*_hd_head).hh_head =
                                            (*_hd_hh_del).hh_next as *mut UtHashHandle;
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
                                s_1 = ::core::ptr::null_mut::<PairClassifierHash>();
                                s_1 = tmp;
                                tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL })
                                    as *mut PairClassifierHash
                                    as *mut PairClassifierHash;
                            }
                            return subtable as *mut Subtable;
                        }
                    }
                }
            }
        } else if subtable_format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            if !(table_length < offset.wrapping_add(16 as u32)) {
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
                let mut cov_0: *mut Coverage =
                    read_coverage(
                        data as *const u8,
                        table_length,
                        offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const u8,
                        ) as u32),
                    );
                (*subtable).first = read_class_def(
                    data as *const u8,
                    table_length,
                    offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as u32),
                );
                (*subtable).first = expand_class_def(
                    cov_0,
                    (*subtable).first,
                );
                otl_coverage_free(cov_0);
                cov_0 = ::core::ptr::null_mut::<Coverage>();
                (*subtable).second = read_class_def(
                    data as *const u8,
                    table_length,
                    offset.wrapping_add(read_16u(
                        data.offset(offset as isize)
                            .offset(10 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as u32),
                );
                if !((*subtable).first.is_null() || (*subtable).second.is_null()) {
                    let mut class1_count: GlyphClass = read_16u(
                        data.offset(offset as isize)
                            .offset(12 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as GlyphClass;
                    let mut class2_count_0: GlyphClass = read_16u(
                        data.offset(offset as isize)
                            .offset(14 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ) as GlyphClass;
                    if !((*(*subtable).first).maxclass as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        != class1_count as ::core::ffi::c_int)
                    {
                        if !((*(*subtable).second).maxclass as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            != class2_count_0 as ::core::ffi::c_int)
                        {
                            if !(table_length
                                < offset.wrapping_add(16 as u32).wrapping_add(
                                    (class1_count as ::core::ffi::c_int
                                        * class2_count_0 as ::core::ffi::c_int
                                        * (len1_0 as ::core::ffi::c_int
                                            + len2_0 as ::core::ffi::c_int))
                                        as u32,
                                ))
                            {
                                (*subtable).first_values = __caryll_allocate_clean(
                                    (::core::mem::size_of::<*mut PositionValue>() as usize)
                                        .wrapping_mul(class1_count as usize),
                                    153 as ::core::ffi::c_ulong,
                                )
                                    as *mut *mut PositionValue;
                                (*subtable).second_values = __caryll_allocate_clean(
                                    (::core::mem::size_of::<*mut PositionValue>() as usize)
                                        .wrapping_mul(class1_count as usize),
                                    154 as ::core::ffi::c_ulong,
                                )
                                    as *mut *mut PositionValue;
                                let mut j_4: GlyphClass = 0 as GlyphClass;
                                while (j_4 as ::core::ffi::c_int)
                                    < class1_count as ::core::ffi::c_int
                                {
                                    let ref mut fresh6 =
                                        *(*subtable).first_values.offset(j_4 as isize);
                                    *fresh6 = __caryll_allocate_clean(
                                        (::core::mem::size_of::<PositionValue>() as usize)
                                            .wrapping_mul(class2_count_0 as usize),
                                        157 as ::core::ffi::c_ulong,
                                    )
                                        as *mut PositionValue;
                                    let ref mut fresh7 =
                                        *(*subtable).second_values.offset(j_4 as isize);
                                    *fresh7 = __caryll_allocate_clean(
                                        (::core::mem::size_of::<PositionValue>() as usize)
                                            .wrapping_mul(class2_count_0 as usize),
                                        158 as ::core::ffi::c_ulong,
                                    )
                                        as *mut PositionValue;
                                    let mut k_2: GlyphClass = 0 as GlyphClass;
                                    while (k_2 as ::core::ffi::c_int)
                                        < class2_count_0 as ::core::ffi::c_int
                                    {
                                        *(*(*subtable).first_values.offset(j_4 as isize))
                                            .offset(k_2 as isize) = read_gpos_value(
                                            data,
                                            table_length,
                                            offset.wrapping_add(16 as u32).wrapping_add(
                                                ((j_4 as ::core::ffi::c_int
                                                    * class2_count_0 as ::core::ffi::c_int
                                                    + k_2 as ::core::ffi::c_int)
                                                    * (len1_0 as ::core::ffi::c_int
                                                        + len2_0 as ::core::ffi::c_int))
                                                    as u32,
                                            ),
                                            format1_0,
                                        );
                                        *(*(*subtable).second_values.offset(j_4 as isize))
                                            .offset(k_2 as isize) = read_gpos_value(
                                            data,
                                            table_length,
                                            offset
                                                .wrapping_add(16 as u32)
                                                .wrapping_add(
                                                    ((j_4 as ::core::ffi::c_int
                                                        * class2_count_0 as ::core::ffi::c_int
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
                                return subtable as *mut Subtable;
                            }
                        }
                    }
                }
            }
        }
    }
    I_SUBTABLE_GPOS_PAIR.free.expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gpos_dump_pair(mut _subtable: *const Subtable) -> *mut JsonValue {
    let mut subtable: *const GposPairSubtable = &raw const (*_subtable).gpos_pair;
    let mut st: *mut JsonValue = json_object_new(3 as usize);
    json_object_push(
        st,
        b"first\0" as *const u8 as *const ::core::ffi::c_char,
        OTL_I_CLASS_DEF.dump.expect("non-null function pointer")((*subtable).first),
    );
    json_object_push(
        st,
        b"second\0" as *const u8 as *const ::core::ffi::c_char,
        OTL_I_CLASS_DEF.dump.expect("non-null function pointer")((*subtable).second),
    );
    let mut mat: *mut JsonValue = json_array_new(
        ((*(*subtable).first).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
    );
    let mut j: GlyphClass = 0 as GlyphClass;
    while j as ::core::ffi::c_int <= (*(*subtable).first).maxclass as ::core::ffi::c_int {
        let mut row: *mut JsonValue = json_array_new(
            ((*(*subtable).second).maxclass as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as usize,
        );
        let mut k: GlyphClass = 0 as GlyphClass;
        while k as ::core::ffi::c_int <= (*(*subtable).second).maxclass as ::core::ffi::c_int {
            let mut f1: u8 = required_position_format(
                *(*(*subtable).first_values.offset(j as isize)).offset(k as isize),
            );
            let mut f2: u8 = required_position_format(
                *(*(*subtable).second_values.offset(j as isize)).offset(k as isize),
            );
            if f1 as ::core::ffi::c_int | f2 as ::core::ffi::c_int != 0 {
                if f1 as ::core::ffi::c_int == FORMAT_DWIDTH as ::core::ffi::c_int
                    && f2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    json_array_push(
                        row,
                        json_new_position(
                            (*(*(*subtable).first_values.offset(j as isize)).offset(k as isize))
                                .d_width,
                        ),
                    );
                } else {
                    let mut pair: *mut JsonValue = json_object_new(2 as usize);
                    if f1 != 0 {
                        json_object_push(
                            pair,
                            b"first\0" as *const u8 as *const ::core::ffi::c_char,
                            gpos_dump_value(
                                *(*(*subtable).first_values.offset(j as isize)).offset(k as isize),
                            ),
                        );
                    }
                    if f2 != 0 {
                        json_object_push(
                            pair,
                            b"second\0" as *const u8 as *const ::core::ffi::c_char,
                            gpos_dump_value(
                                *(*(*subtable).second_values.offset(j as isize)).offset(k as isize),
                            ),
                        );
                    }
                    json_array_push(row, pair);
                }
            } else {
                json_array_push(row, json_new_position(0 as ::core::ffi::c_int as Pos));
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
pub unsafe extern "C" fn otl_gpos_parse_pair(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut class1_count: GlyphClass = 0;
    let mut class2_count: GlyphClass = 0;
    let mut subtable: *mut GposPairSubtable =
        (
            I_SUBTABLE_GPOS_PAIR
                .create
                .expect("non-null function pointer"))();
    let mut _mat: *mut JsonValue = json_obj_get_type(
        _subtable,
        b"matrix\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    (*subtable).first = OTL_I_CLASS_DEF.parse.expect("non-null function pointer")(json_obj_get_type(
        _subtable,
        b"first\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    ));
    (*subtable).second =
        OTL_I_CLASS_DEF.parse.expect("non-null function pointer")(json_obj_get_type(
            _subtable,
            b"second\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Object,
        ));
    if _mat.is_null() || (*subtable).first.is_null() || (*subtable).second.is_null() {
        I_SUBTABLE_GPOS_PAIR.free.expect("non-null function pointer")(subtable);
        return ::core::ptr::null_mut::<Subtable>();
    } else {
        class1_count = ((*(*subtable).first).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as GlyphClass;
        class2_count = ((*(*subtable).second).maxclass as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as GlyphClass;
        (*subtable).first_values = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut PositionValue>() as usize)
                .wrapping_mul(class1_count as usize),
            224 as ::core::ffi::c_ulong,
        ) as *mut *mut PositionValue;
        (*subtable).second_values = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut PositionValue>() as usize)
                .wrapping_mul(class1_count as usize),
            225 as ::core::ffi::c_ulong,
        ) as *mut *mut PositionValue;
        let mut j: GlyphClass = 0 as GlyphClass;
        while (j as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int {
            let ref mut fresh8 = *(*subtable).first_values.offset(j as isize);
            *fresh8 = __caryll_allocate_clean(
                (::core::mem::size_of::<PositionValue>() as usize)
                    .wrapping_mul(class2_count as usize),
                228 as ::core::ffi::c_ulong,
            ) as *mut PositionValue;
            let ref mut fresh9 = *(*subtable).second_values.offset(j as isize);
            *fresh9 = __caryll_allocate_clean(
                (::core::mem::size_of::<PositionValue>() as usize)
                    .wrapping_mul(class2_count as usize),
                229 as ::core::ffi::c_ulong,
            ) as *mut PositionValue;
            let mut k: GlyphClass = 0 as GlyphClass;
            while (k as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int {
                *(*(*subtable).first_values.offset(j as isize)).offset(k as isize) = position_zero();
                *(*(*subtable).second_values.offset(j as isize)).offset(k as isize) =
                    position_zero();
                k = k.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        let mut j_0: GlyphClass = 0 as GlyphClass;
        while (j_0 as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int
            && (j_0 as ::core::ffi::c_uint) < (*_mat).u.array.length
        {
            let mut _row: *mut JsonValue =
                *(*_mat).u.array.values.offset(j_0 as isize) as *mut JsonValue;
            if !(_row.is_null()
                || (*_row).type_0 != JsonType::Array)
            {
                let mut k_0: GlyphClass = 0 as GlyphClass;
                while (k_0 as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int
                    && (k_0 as ::core::ffi::c_uint) < (*_row).u.array.length
                {
                    let mut _item: *mut JsonValue =
                        *(*_row).u.array.values.offset(k_0 as isize) as *mut JsonValue;
                    if (*_item).type_0 == JsonType::Integer
                    {
                        (*(*(*subtable).first_values.offset(j_0 as isize)).offset(k_0 as isize))
                            .d_width = (*_item).u.integer as Pos;
                    } else if (*_item).type_0 == JsonType::Double
                    {
                        (*(*(*subtable).first_values.offset(j_0 as isize)).offset(k_0 as isize))
                            .d_width = (*_item).u.dbl as Pos;
                    } else if (*_item).type_0 == JsonType::Object
                    {
                        *(*(*subtable).first_values.offset(j_0 as isize)).offset(k_0 as isize) =
                            gpos_parse_value(json_obj_get(
                                _item,
                                b"first\0" as *const u8 as *const ::core::ffi::c_char,
                            ));
                        *(*(*subtable).second_values.offset(j_0 as isize)).offset(k_0 as isize) =
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
        return subtable as *mut Subtable;
    };
}
unsafe extern "C" fn cov_from_cd(mut cd: *mut ClassDef) -> *mut Coverage {
    let mut cov: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    cov = __caryll_allocate_clean(
        ::core::mem::size_of::<Coverage>() as usize,
        257 as ::core::ffi::c_ulong,
    ) as *mut Coverage;
    (*cov).num_glyphs = (*cd).num_glyphs;
    (*cov).glyphs = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphHandle>() as usize)
            .wrapping_mul((*cd).num_glyphs as usize),
        259 as ::core::ffi::c_ulong,
    ) as *mut GlyphHandle;
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < (*cd).num_glyphs as ::core::ffi::c_int {
        *(*cov).glyphs.offset(j as isize) = otfcc_handle_dup(
            (*(*cd).glyphs.offset(j as isize)).clone() as Handle,
        ) as GlyphHandle;
        j = j.wrapping_add(1);
    }
    return cov;
}
unsafe extern "C" fn by_pair_second_glyph(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut IndividualGposPair)).gid as ::core::ffi::c_int
        - (*(b as *mut IndividualGposPair)).gid as ::core::ffi::c_int;
}
pub unsafe extern "C" fn otfcc_build_gpos_pair_individual(
    mut _subtable: *const Subtable,
) -> *mut BkBlock {
    let mut subtable: *const GposPairSubtable = &raw const (*_subtable).gpos_pair;
    let mut format1: u16 = 0 as u16;
    let mut format2: u16 = 0 as u16;
    let mut class1_count: GlyphClass = ((*(*subtable).first).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut class2_count: GlyphClass = ((*(*subtable).second).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut j: GlyphClass = 0 as GlyphClass;
    while (j as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int {
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int {
            format1 = (format1 as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).first_values.offset(j as isize)).offset(k as isize),
                ) as ::core::ffi::c_int) as u16;
            format2 = (format2 as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).second_values.offset(j as isize)).offset(k as isize),
                ) as ::core::ffi::c_int) as u16;
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let mut pair_counts: *mut GlyphId = ::core::ptr::null_mut::<GlyphId>();
    pair_counts = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphId>() as usize)
            .wrapping_mul((*(*subtable).first).num_glyphs as usize),
        290 as ::core::ffi::c_ulong,
    ) as *mut GlyphId;
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < (*(*subtable).first).num_glyphs as ::core::ffi::c_int {
        *pair_counts.offset(j_0 as isize) = 0 as GlyphId;
        let mut k_0: GlyphId = 0 as GlyphId;
        while (k_0 as ::core::ffi::c_int) < (*(*subtable).second).num_glyphs as ::core::ffi::c_int {
            let mut c1: GlyphClass = *(*(*subtable).first).classes.offset(j_0 as isize);
            let mut c2: GlyphClass = *(*(*subtable).second).classes.offset(k_0 as isize);
            if required_position_format(
                *(*(*subtable).first_values.offset(c1 as isize)).offset(c2 as isize),
            ) as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).second_values.offset(c1 as isize)).offset(c2 as isize),
                ) as ::core::ffi::c_int
                != 0
            {
                let ref mut fresh10 = *pair_counts.offset(j_0 as isize);
                *fresh10 = (*fresh10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
            }
            k_0 = k_0.wrapping_add(1);
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut cov: *mut Coverage = cov_from_cd((*subtable).first);
    shrink_coverage(cov, true);
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(cov))), bk_int(BkCellType::B16, (format1 as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (format2 as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*(*subtable).first).num_glyphs as ::core::ffi::c_int) as u32)]);
    let mut j_1: GlyphId = 0 as GlyphId;
    while (j_1 as ::core::ffi::c_int) < (*cov).num_glyphs as ::core::ffi::c_int {
        let mut current_pair_count: TableId = 0 as TableId;
        let mut c1_0: GlyphClass = 0 as GlyphClass;
        let mut k_1: GlyphId = 0 as GlyphId;
        while (k_1 as ::core::ffi::c_int) < (*(*subtable).first).num_glyphs as ::core::ffi::c_int {
            if (*(*(*subtable).first).glyphs.offset(k_1 as isize)).index as ::core::ffi::c_int
                == (*(*cov).glyphs.offset(j_1 as isize)).index as ::core::ffi::c_int
            {
                c1_0 = *(*(*subtable).first).classes.offset(k_1 as isize);
                current_pair_count = *pair_counts.offset(k_1 as isize) as TableId;
            }
            k_1 = k_1.wrapping_add(1);
        }
        let mut pair_set: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (current_pair_count as ::core::ffi::c_int) as u32)]);
        let mut pairs: *mut IndividualGposPair = ::core::ptr::null_mut::<IndividualGposPair>();
        pairs = __caryll_allocate_clean(
            (::core::mem::size_of::<IndividualGposPair>() as usize)
                .wrapping_mul(current_pair_count as usize),
            324 as ::core::ffi::c_ulong,
        ) as *mut IndividualGposPair;
        let mut n: usize = 0 as usize;
        let mut k_2: GlyphId = 0 as GlyphId;
        while (k_2 as ::core::ffi::c_int) < (*(*subtable).second).num_glyphs as ::core::ffi::c_int {
            let mut c2_0: GlyphClass = *(*(*subtable).second).classes.offset(k_2 as isize);
            if required_position_format(
                *(*(*subtable).first_values.offset(c1_0 as isize)).offset(c2_0 as isize),
            ) as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).second_values.offset(c1_0 as isize)).offset(c2_0 as isize),
                ) as ::core::ffi::c_int
                != 0
            {
                (*pairs.offset(n as isize)).gid =
                    (*(*(*subtable).second).glyphs.offset(k_2 as isize)).index;
                let ref mut fresh11 = (*pairs.offset(n as isize)).fv;
                *fresh11 = (*(*subtable).first_values.offset(c1_0 as isize)).offset(c2_0 as isize)
                    as *mut PositionValue;
                let ref mut fresh12 = (*pairs.offset(n as isize)).sv;
                *fresh12 = (*(*subtable).second_values.offset(c1_0 as isize)).offset(c2_0 as isize)
                    as *mut PositionValue;
                n = n.wrapping_add(1);
            }
            k_2 = k_2.wrapping_add(1);
        }
        qsort(
            pairs as *mut ::core::ffi::c_void,
            current_pair_count as usize,
            ::core::mem::size_of::<IndividualGposPair>() as usize,
            Some(
                by_pair_second_glyph
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut n_0: usize = 0 as usize;
        while n_0 < current_pair_count as usize {
            bk_push(pair_set, &[bk_int(BkCellType::B16, ((*pairs.offset(n_0 as isize)).gid as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::Embed, bk_gpos_value(*(*pairs.offset(n_0 as isize)).fv, format1)), bk_ptr(BkCellType::Embed, bk_gpos_value(*(*pairs.offset(n_0 as isize)).sv, format2))]);
            n_0 = n_0.wrapping_add(1);
        }
        free(pairs as *mut ::core::ffi::c_void);
        pairs = ::core::ptr::null_mut::<IndividualGposPair>();
        bk_push(root, &[bk_ptr(BkCellType::P16, pair_set)]);
        j_1 = j_1.wrapping_add(1);
    }
    otl_coverage_free(cov);
    cov = ::core::ptr::null_mut::<Coverage>();
    free(pair_counts as *mut ::core::ffi::c_void);
    pair_counts = ::core::ptr::null_mut::<GlyphId>();
    return root;
}
pub unsafe extern "C" fn otfcc_build_gpos_pair_classes(
    mut _subtable: *const Subtable,
) -> *mut BkBlock {
    let mut subtable: *const GposPairSubtable = &raw const (*_subtable).gpos_pair;
    let mut format1: u16 = 0 as u16;
    let mut format2: u16 = 0 as u16;
    let mut class1_count: GlyphClass = ((*(*subtable).first).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut class2_count: GlyphClass = ((*(*subtable).second).maxclass as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as GlyphClass;
    let mut j: GlyphClass = 0 as GlyphClass;
    while (j as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int {
        let mut k: GlyphClass = 0 as GlyphClass;
        while (k as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int {
            format1 = (format1 as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).first_values.offset(j as isize)).offset(k as isize),
                ) as ::core::ffi::c_int) as u16;
            format2 = (format2 as ::core::ffi::c_int
                | required_position_format(
                    *(*(*subtable).second_values.offset(j as isize)).offset(k as isize),
                ) as ::core::ffi::c_int) as u16;
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let mut cov: *mut Coverage = cov_from_cd((*subtable).first);
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 2 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(cov))), bk_int(BkCellType::B16, (format1 as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (format2 as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_CLASS_DEF.build.expect("non-null function pointer")(
            (*subtable).first,
        ))), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_CLASS_DEF.build.expect("non-null function pointer")(
            (*subtable).second,
        ))), bk_int(BkCellType::B16, (class1_count as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (class2_count as ::core::ffi::c_int) as u32)]);
    let mut j_0: GlyphClass = 0 as GlyphClass;
    while (j_0 as ::core::ffi::c_int) < class1_count as ::core::ffi::c_int {
        let mut k_0: GlyphClass = 0 as GlyphClass;
        while (k_0 as ::core::ffi::c_int) < class2_count as ::core::ffi::c_int {
            bk_push(root, &[bk_ptr(BkCellType::Embed, bk_gpos_value(
                    *(*(*subtable).first_values.offset(j_0 as isize)).offset(k_0 as isize),
                    format1,
                )), bk_ptr(BkCellType::Embed, bk_gpos_value(
                    *(*(*subtable).second_values.offset(j_0 as isize)).offset(k_0 as isize),
                    format2,
                ))]);
            k_0 = k_0.wrapping_add(1);
        }
        j_0 = j_0.wrapping_add(1);
    }
    otl_coverage_free(cov);
    cov = ::core::ptr::null_mut::<Coverage>();
    return root;
}
pub unsafe extern "C" fn otfcc_build_gpos_pair(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let mut format1: *mut BkBlock = otfcc_build_gpos_pair_individual(_subtable);
    let mut format2: *mut BkBlock = otfcc_build_gpos_pair_classes(_subtable);
    let mut g1: *mut BkGraph = bk_new_graph_from_root_block(format1);
    let mut g2: *mut BkGraph = bk_new_graph_from_root_block(format2);
    bk_minimize_graph(g1);
    bk_minimize_graph(g2);
    if bk_estimate_size_of_graph(g1) > bk_estimate_size_of_graph(g2) {
        bk_delete_graph(g1);
        bk_untangle_graph(g2);
        let mut buf: *mut Buffer = bk_build_graph(g2);
        bk_delete_graph(g2);
        return buf;
    } else {
        bk_delete_graph(g2);
        bk_untangle_graph(g1);
        let mut buf_0: *mut Buffer = bk_build_graph(g1);
        bk_delete_graph(g1);
        return buf_0;
    };
}
