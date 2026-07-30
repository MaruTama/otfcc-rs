#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset};

use crate::table::otl::coverage::{Coverage};
use crate::support::handle::{handle_from_consolidated, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, TableId};
use crate::font::caryll_font::{Font};
use crate::support::{NULL};

























use crate::table::otl::{Subtable, GsubReverseSubtable, OtlTable};

use crate::table::otl::subtables::gsub_single::{GsubSingleMapHash};




use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::consolidate::otl::common::{fontop_consolidate_coverage};
use crate::vendor::sds::{sdsempty};




unsafe extern "C" fn by_from_id(
    mut a: *mut GsubSingleMapHash,
    mut b: *mut GsubSingleMapHash,
) -> ::core::ffi::c_int {
    return (*a).fromid - (*b).fromid;
}
pub unsafe extern "C" fn consolidate_gsub_reverse(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GsubReverseSubtable = &raw mut (*_subtable).gsub_reverse;
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*subtable).match_count as ::core::ffi::c_int {
        fontop_consolidate_coverage(font, *(*subtable).match_0.offset(j as isize), options);
        j = j.wrapping_add(1);
    }
    fontop_consolidate_coverage(font, (*subtable).to, options);
    if (*subtable).input_index as ::core::ffi::c_int >= (*subtable).match_count as ::core::ffi::c_int
    {
        (*subtable).input_index =
            ((*subtable).match_count as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as TableId;
    }
    let mut h: *mut GsubSingleMapHash = ::core::ptr::null_mut::<GsubSingleMapHash>();
    let mut from: *mut Coverage = *(*subtable).match_0.offset((*subtable).input_index as isize);
    let mut k: GlyphId = 0 as GlyphId;
    while (k as ::core::ffi::c_int) < (*from).num_glyphs as ::core::ffi::c_int
        && (k as ::core::ffi::c_int) < (*(*subtable).to).num_glyphs as ::core::ffi::c_int
    {
        let mut s: *mut GsubSingleMapHash = ::core::ptr::null_mut::<GsubSingleMapHash>();
        let mut fromid: ::core::ffi::c_int =
            (*(*from).glyphs.offset(k as isize)).index as ::core::ffi::c_int;
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key: *const ::core::ffi::c_uchar =
            &raw mut fromid as *const ::core::ffi::c_uchar;
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
        let mut current_block_57: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_57 = 12350559058236795675;
            }
            10 => {
                current_block_57 = 12350559058236795675;
            }
            9 => {
                current_block_57 = 10419873034695718419;
            }
            8 => {
                current_block_57 = 15023088803550280096;
            }
            7 => {
                current_block_57 = 3356697356915278238;
            }
            6 => {
                current_block_57 = 13205676039122788963;
            }
            5 => {
                current_block_57 = 12615460156521959718;
            }
            4 => {
                current_block_57 = 6894971415495242001;
            }
            3 => {
                current_block_57 = 1580727996509200123;
            }
            2 => {
                current_block_57 = 14629213492470641298;
            }
            1 => {
                current_block_57 = 4679085629702540918;
            }
            _ => {
                current_block_57 = 9627623479216730126;
            }
        }
        match current_block_57 {
            12350559058236795675 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_57 = 10419873034695718419;
            }
            _ => {}
        }
        match current_block_57 {
            10419873034695718419 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_57 = 15023088803550280096;
            }
            _ => {}
        }
        match current_block_57 {
            15023088803550280096 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_57 = 3356697356915278238;
            }
            _ => {}
        }
        match current_block_57 {
            3356697356915278238 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_57 = 13205676039122788963;
            }
            _ => {}
        }
        match current_block_57 {
            13205676039122788963 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_57 = 12615460156521959718;
            }
            _ => {}
        }
        match current_block_57 {
            12615460156521959718 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_57 = 6894971415495242001;
            }
            _ => {}
        }
        match current_block_57 {
            6894971415495242001 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_57 = 1580727996509200123;
            }
            _ => {}
        }
        match current_block_57 {
            1580727996509200123 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_57 = 14629213492470641298;
            }
            _ => {}
        }
        match current_block_57 {
            14629213492470641298 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_57 = 4679085629702540918;
            }
            _ => {}
        }
        match current_block_57 {
            4679085629702540918 => {
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
        s = ::core::ptr::null_mut::<GsubSingleMapHash>();
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
                        as *mut GsubSingleMapHash
                        as *mut GsubSingleMapHash;
                } else {
                    s = ::core::ptr::null_mut::<GsubSingleMapHash>();
                }
                while !s.is_null() {
                    if (*s).hh.hashv == _hf_hashv
                        && (*s).hh.keylen as usize
                            == ::core::mem::size_of::<::core::ffi::c_int>()
                    {
                        if memcmp(
                            (*s).hh.key,
                            &raw mut fromid as *const ::core::ffi::c_void,
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
                            as *mut GsubSingleMapHash
                            as *mut GsubSingleMapHash;
                    } else {
                        s = ::core::ptr::null_mut::<GsubSingleMapHash>();
                    }
                }
            }
        }
        if !s.is_null() {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Double-mapping a glyph in a reverse substitution /",
                    (*(*from).glyphs.offset(k as isize)).name,
                    b".\n",
                ),
            );
        } else {
            s = __caryll_allocate_clean(
                ::core::mem::size_of::<GsubSingleMapHash>() as usize,
                31 as ::core::ffi::c_ulong,
            ) as *mut GsubSingleMapHash;
            (*s).fromid = (*(*from).glyphs.offset(k as isize)).index as ::core::ffi::c_int;
            (*s).toid = (*(*(*subtable).to).glyphs.offset(k as isize)).index as ::core::ffi::c_int;
            (*s).fromname = (*(*from).glyphs.offset(k as isize)).name;
            (*s).toname = (*(*(*subtable).to).glyphs.offset(k as isize)).name;
            let mut _ha_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i_0: ::core::ffi::c_uint = 0;
            let mut _hj_j_0: ::core::ffi::c_uint = 0;
            let mut _hj_k_0: ::core::ffi::c_uint = 0;
            let mut _hj_key_0: *const ::core::ffi::c_uchar =
                &raw mut (*s).fromid as *const ::core::ffi::c_uchar;
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
            let mut current_block_179: u64;
            match _hj_k_0 {
                11 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_179 = 11112991556185382089;
                }
                10 => {
                    current_block_179 = 11112991556185382089;
                }
                9 => {
                    current_block_179 = 17681268777694753330;
                }
                8 => {
                    current_block_179 = 7338651777899399442;
                }
                7 => {
                    current_block_179 = 17942627274227481263;
                }
                6 => {
                    current_block_179 = 4676144417340510455;
                }
                5 => {
                    current_block_179 = 12852691815975679413;
                }
                4 => {
                    current_block_179 = 11005737405096136282;
                }
                3 => {
                    current_block_179 = 10352098015658169381;
                }
                2 => {
                    current_block_179 = 13133635826799485799;
                }
                1 => {
                    current_block_179 = 18106367618607459138;
                }
                _ => {
                    current_block_179 = 5908482871227205451;
                }
            }
            match current_block_179 {
                11112991556185382089 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_179 = 17681268777694753330;
                }
                _ => {}
            }
            match current_block_179 {
                17681268777694753330 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_179 = 7338651777899399442;
                }
                _ => {}
            }
            match current_block_179 {
                7338651777899399442 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_179 = 17942627274227481263;
                }
                _ => {}
            }
            match current_block_179 {
                17942627274227481263 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_179 = 4676144417340510455;
                }
                _ => {}
            }
            match current_block_179 {
                4676144417340510455 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_179 = 12852691815975679413;
                }
                _ => {}
            }
            match current_block_179 {
                12852691815975679413 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_179 = 11005737405096136282;
                }
                _ => {}
            }
            match current_block_179 {
                11005737405096136282 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_179 = 10352098015658169381;
                }
                _ => {}
            }
            match current_block_179 {
                10352098015658169381 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_179 = 13133635826799485799;
                }
                _ => {}
            }
            match current_block_179 {
                13133635826799485799 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_179 = 18106367618607459138;
                }
                _ => {}
            }
            match current_block_179 {
                18106367618607459138 => {
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
            (*s).hh.hashv = _ha_hashv;
            (*s).hh.key =
                &raw mut (*s).fromid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            (*s).hh.keylen = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            if h.is_null() {
                (*s).hh.next = NULL;
                (*s).hh.prev = NULL;
                (*s).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                    as *mut UtHashTable as *mut UtHashTable;
                if (*s).hh.tbl.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*s).hh.tbl as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<UtHashTable>() as usize,
                    );
                    (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
                    (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                    (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                    (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                        .offset_from(s as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as isize;
                    (*(*s).hh.tbl).buckets = malloc(
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    ) as *mut UtHashBucket;
                    (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                    if (*(*s).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        );
                    }
                }
                h = s;
            } else {
                (*s).hh.tbl = (*h).hh.tbl;
                (*s).hh.next = NULL;
                (*s).hh.prev = ((*(*h).hh.tbl).tail as *mut ::core::ffi::c_char)
                    .offset(-(*(*h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void;
                (*(*(*h).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                (*(*h).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
            }
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UtHashBucket =
                (*(*h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
            (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
            (*s).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
            (*s).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
            if !(*_ha_head).hh_head.is_null() {
                (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UtHashHandle;
            }
            (*_ha_head).hh_head = &raw mut (*s).hh as *mut UtHashHandle;
            if (*_ha_head).count
                >= (*_ha_head)
                    .expand_mult
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                && (*(*s).hh.tbl).noexpand == 0
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
                        .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                if _he_new_buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        _he_new_buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (2 as usize)
                            .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    );
                    (*(*s).hh.tbl).ideal_chain_maxlen = ((*(*s).hh.tbl).num_items
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
                    (*(*s).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                    _he_bkt_i = 0 as ::core::ffi::c_uint;
                    while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                        _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UtHashHandle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*s).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                            (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                            if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                (*(*s).hh.tbl).nonideal_items =
                                    (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt).expand_mult = (*_he_newbkt)
                                    .count
                                    .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
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
                    free((*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    (*(*s).hh.tbl).num_buckets = (*(*s).hh.tbl)
                        .num_buckets
                        .wrapping_mul(2 as ::core::ffi::c_uint);
                    (*(*s).hh.tbl).log2_num_buckets =
                        (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
                    (*(*s).hh.tbl).buckets = _he_new_buckets;
                    (*(*s).hh.tbl).ineff_expands = if (*(*s).hh.tbl).nonideal_items
                        > (*(*s).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                    {
                        (*(*s).hh.tbl)
                            .ineff_expands
                            .wrapping_add(1 as ::core::ffi::c_uint)
                    } else {
                        0 as ::core::ffi::c_uint
                    };
                    if (*(*s).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                        (*(*s).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                    }
                }
            }
        }
        k = k.wrapping_add(1);
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
    if !h.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*h).hh as *mut UtHashHandle;
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
                            .offset((*(*h).hh.tbl).hho)
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
                                .offset((*(*h).hh.tbl).hho)
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
                                    .offset((*(*h).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if by_from_id(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut GsubSingleMapHash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut GsubSingleMapHash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*h).hh.tbl).hho)
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
                                .offset((*(*h).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*h).hh.tbl).hho)
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
                                .offset(-(*(*h).hh.tbl).hho)
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
                (*(*h).hh.tbl).tail = _hs_tail;
                h = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut GsubSingleMapHash
                    as *mut GsubSingleMapHash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    if (if !h.is_null() {
        (*(*h).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) != (*from).num_glyphs as ::core::ffi::c_uint
        || (if !h.is_null() {
            (*(*h).hh.tbl).num_items
        } else {
            0 as ::core::ffi::c_uint
        }) != (*(*subtable).to).num_glyphs as ::core::ffi::c_uint
    {
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::sdsbuild!(
                sdsempty(),
                b"[Consolidate] In this reverse subsitution lookup, some mappings are ignored.\n",
            ),
        );
    }
    (*from).num_glyphs = (if !h.is_null() {
        (*(*h).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) as GlyphId;
    (*(*subtable).to).num_glyphs = (if !h.is_null() {
        (*(*h).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) as GlyphId;
    let mut s_0: *mut GsubSingleMapHash = ::core::ptr::null_mut::<GsubSingleMapHash>();
    let mut tmp: *mut GsubSingleMapHash = ::core::ptr::null_mut::<GsubSingleMapHash>();
    let mut j_0: GlyphId = 0 as GlyphId;
    s_0 = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut GsubSingleMapHash
        as *mut GsubSingleMapHash;
    while !s_0.is_null() {
        *(*from).glyphs.offset(j_0 as isize) = handle_from_consolidated(
            (*s_0).fromid as GlyphId, (*s_0).fromname
        ) as GlyphHandle;
        *(*(*subtable).to).glyphs.offset(j_0 as isize) = handle_from_consolidated(
            (*s_0).toid as GlyphId, (*s_0).toname
        ) as GlyphHandle;
        j_0 = j_0.wrapping_add(1);
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s_0).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<GsubSingleMapHash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut GsubSingleMapHash as *mut GsubSingleMapHash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket =
                (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
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
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<GsubSingleMapHash>();
        s_0 = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut GsubSingleMapHash
            as *mut GsubSingleMapHash;
    }
    return false;
}
