#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset};


use crate::support::handle::{handle_from_consolidated, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphClass, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::font::caryll_font::{Font};
use crate::support::{NULL};
























use crate::table::otl::{Anchor, BaseArray, BaseRecord, LigatureArray, LigatureBaseRecord, MarkArray, MarkRecord, Subtable, GposMarkToLigatureSubtable, GposMarkToSingleSubtable, OtlTable};





use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::table::otl::subtables::gpos_common::{OTL_I_MARK_ARRAY};
use crate::table::otl::subtables::gpos_mark_to_ligature::{OTL_I_LIGATURE_ARRAY};
use crate::table::otl::subtables::gpos_mark_to_single::{OTL_I_BASE_ARRAY};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree};




#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseHash {
    pub gid: ::core::ffi::c_int,
    pub name: SdsRaw,
    pub anchors: *mut Anchor,
    pub hh: UtHashHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MarkHash {
    pub gid: ::core::ffi::c_int,
    pub name: SdsRaw,
    pub markClass: GlyphClass,
    pub anchor: Anchor,
    pub hh: UtHashHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LigHash {
    pub gid: ::core::ffi::c_int,
    pub name: SdsRaw,
    pub componentCount: GlyphId,
    pub anchors: *mut *mut Anchor,
    pub hh: UtHashHandle,
}
unsafe extern "C" fn mark_by_gid(
    mut a: *mut MarkHash,
    mut b: *mut MarkHash,
) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
unsafe extern "C" fn base_by_gid(
    mut a: *mut BaseHash,
    mut b: *mut BaseHash,
) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
unsafe extern "C" fn lig_by_gid(mut a: *mut LigHash, mut b: *mut LigHash) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
unsafe extern "C" fn consolidate_mark_array(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut options: *const Options,
    mut markArray: *mut MarkArray,
    mut classCount: GlyphClass,
) {
    let mut hm: *mut MarkHash = ::core::ptr::null_mut::<MarkHash>();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*markArray).length {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*markArray).items.offset(k as isize)).glyph,
        ) {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored unknown glyph name ",
                    (*(*markArray).items.offset(k as isize)).glyph.name,
                    b".",
                ),
            );
        } else {
            let mut s: *mut MarkHash = ::core::ptr::null_mut::<MarkHash>();
            let mut gid: ::core::ffi::c_int =
                (*(*markArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
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
            let mut current_block_52: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 15766119939431011442;
                }
                10 => {
                    current_block_52 = 15766119939431011442;
                }
                9 => {
                    current_block_52 = 16082293127231038334;
                }
                8 => {
                    current_block_52 = 6924315704091482277;
                }
                7 => {
                    current_block_52 = 8817668411986532499;
                }
                6 => {
                    current_block_52 = 17613857163787856897;
                }
                5 => {
                    current_block_52 = 7171273293905213987;
                }
                4 => {
                    current_block_52 = 4496227623580412362;
                }
                3 => {
                    current_block_52 = 16130385434440591865;
                }
                2 => {
                    current_block_52 = 13809408577757465348;
                }
                1 => {
                    current_block_52 = 6834373174349270986;
                }
                _ => {
                    current_block_52 = 1345366029464561491;
                }
            }
            match current_block_52 {
                15766119939431011442 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 16082293127231038334;
                }
                _ => {}
            }
            match current_block_52 {
                16082293127231038334 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 6924315704091482277;
                }
                _ => {}
            }
            match current_block_52 {
                6924315704091482277 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 8817668411986532499;
                }
                _ => {}
            }
            match current_block_52 {
                8817668411986532499 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 17613857163787856897;
                }
                _ => {}
            }
            match current_block_52 {
                17613857163787856897 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 7171273293905213987;
                }
                _ => {}
            }
            match current_block_52 {
                7171273293905213987 => {
                    _hj_j = _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_52 = 4496227623580412362;
                }
                _ => {}
            }
            match current_block_52 {
                4496227623580412362 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 16130385434440591865;
                }
                _ => {}
            }
            match current_block_52 {
                16130385434440591865 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 13809408577757465348;
                }
                _ => {}
            }
            match current_block_52 {
                13809408577757465348 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 6834373174349270986;
                }
                _ => {}
            }
            match current_block_52 {
                6834373174349270986 => {
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
            s = ::core::ptr::null_mut::<MarkHash>();
            if !hm.is_null() {
                let mut _hf_bkt: ::core::ffi::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize))
                        .hh_head
                        .is_null()
                    {
                        s = ((*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut MarkHash as *mut MarkHash;
                    } else {
                        s = ::core::ptr::null_mut::<MarkHash>();
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
                                .offset(-(*(*hm).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut MarkHash as *mut MarkHash;
                        } else {
                            s = ::core::ptr::null_mut::<MarkHash>();
                        }
                    }
                }
            }
            if s.is_null()
                && (*(*markArray).items.offset(k as isize)).anchor.present as ::core::ffi::c_int
                    != 0
                && ((*(*markArray).items.offset(k as isize)).markClass as ::core::ffi::c_int)
                    < classCount as ::core::ffi::c_int
            {
                s = __caryll_allocate_clean(
                    ::core::mem::size_of::<MarkHash>() as usize,
                    47 as ::core::ffi::c_ulong,
                ) as *mut MarkHash;
                (*s).gid =
                    (*(*markArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
                (*s).name = sdsdup((*(*markArray).items.offset(k as isize)).glyph.name);
                (*s).markClass = (*(*markArray).items.offset(k as isize)).markClass;
                (*s).anchor = (*(*markArray).items.offset(k as isize)).anchor;
                let mut _ha_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i_0: ::core::ffi::c_uint = 0;
                let mut _hj_j_0: ::core::ffi::c_uint = 0;
                let mut _hj_k_0: ::core::ffi::c_uint = 0;
                let mut _hj_key_0: *const ::core::ffi::c_uchar =
                    &raw mut (*s).gid as *const ::core::ffi::c_uchar;
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
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint
                );
                let mut current_block_171: u64;
                match _hj_k_0 {
                    11 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 9869338346707858197;
                    }
                    10 => {
                        current_block_171 = 9869338346707858197;
                    }
                    9 => {
                        current_block_171 = 7158800297742905591;
                    }
                    8 => {
                        current_block_171 = 17374360098714674690;
                    }
                    7 => {
                        current_block_171 = 15108445819848477191;
                    }
                    6 => {
                        current_block_171 = 7080490894523740831;
                    }
                    5 => {
                        current_block_171 = 706379200111713019;
                    }
                    4 => {
                        current_block_171 = 14540267986305250866;
                    }
                    3 => {
                        current_block_171 = 11423875456617891677;
                    }
                    2 => {
                        current_block_171 = 11721289334896627849;
                    }
                    1 => {
                        current_block_171 = 3913562009144861594;
                    }
                    _ => {
                        current_block_171 = 7315983924538012637;
                    }
                }
                match current_block_171 {
                    9869338346707858197 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 7158800297742905591;
                    }
                    _ => {}
                }
                match current_block_171 {
                    7158800297742905591 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 17374360098714674690;
                    }
                    _ => {}
                }
                match current_block_171 {
                    17374360098714674690 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 15108445819848477191;
                    }
                    _ => {}
                }
                match current_block_171 {
                    15108445819848477191 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 7080490894523740831;
                    }
                    _ => {}
                }
                match current_block_171 {
                    7080490894523740831 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 706379200111713019;
                    }
                    _ => {}
                }
                match current_block_171 {
                    706379200111713019 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_171 = 14540267986305250866;
                    }
                    _ => {}
                }
                match current_block_171 {
                    14540267986305250866 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 11423875456617891677;
                    }
                    _ => {}
                }
                match current_block_171 {
                    11423875456617891677 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 11721289334896627849;
                    }
                    _ => {}
                }
                match current_block_171 {
                    11721289334896627849 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 3913562009144861594;
                    }
                    _ => {}
                }
                match current_block_171 {
                    3913562009144861594 => {
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
                    &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
                (*s).hh.keylen =
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                if hm.is_null() {
                    (*s).hh.next = NULL;
                    (*s).hh.prev = NULL;
                    (*s).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                        as *mut UtHashTable
                        as *mut UtHashTable;
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
                                (32 as usize).wrapping_mul(
                                    ::core::mem::size_of::<UtHashBucket>() as usize,
                                ),
                            );
                        }
                    }
                    hm = s;
                } else {
                    (*s).hh.tbl = (*hm).hh.tbl;
                    (*s).hh.next = NULL;
                    (*s).hh.prev = ((*(*hm).hh.tbl).tail as *mut ::core::ffi::c_char)
                        .offset(-(*(*hm).hh.tbl).hho)
                        as *mut ::core::ffi::c_void;
                    (*(*(*hm).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                    (*(*hm).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
                }
                let mut _ha_bkt: ::core::ffi::c_uint = 0;
                (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_add(1);
                _ha_bkt = _ha_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head: *mut UtHashBucket =
                    (*(*hm).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
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
            } else {
                (*(*options).logger)
                    .logSDS
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignored invalid or double-mapping mark definition for /",
                        (*(*markArray).items.offset(k as isize)).glyph.name,
                        b".",
                    ),
                );
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
    if !hm.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*hm).hh as *mut UtHashHandle;
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
                            .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
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
                                    .offset((*(*hm).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if mark_by_gid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut MarkHash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut MarkHash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*hm).hh.tbl).hho)
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
                                .offset(-(*(*hm).hh.tbl).hho)
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
                (*(*hm).hh.tbl).tail = _hs_tail;
                hm = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut MarkHash
                    as *mut MarkHash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    OTL_I_MARK_ARRAY.clear.expect("non-null function pointer")(markArray);
    let mut s_0: *mut MarkHash = ::core::ptr::null_mut::<MarkHash>();
    let mut tmp: *mut MarkHash = ::core::ptr::null_mut::<MarkHash>();
    s_0 = hm;
    tmp = (if !hm.is_null() { (*hm).hh.next } else { NULL }) as *mut MarkHash as *mut MarkHash;
    while !s_0.is_null() {
        OTL_I_MARK_ARRAY.push.expect("non-null function pointer")(
            markArray,
            MarkRecord {
                glyph: handle_from_consolidated(
                    (*s_0).gid as GlyphId, (*s_0).name
                ) as GlyphHandle,
                markClass: (*s_0).markClass,
                anchor: (*s_0).anchor,
            },
        );
        sdsfree((*s_0).name);
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s_0).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*hm).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*hm).hh.tbl as *mut ::core::ffi::c_void);
            hm = ::core::ptr::null_mut::<MarkHash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*hm).hh.tbl).tail {
                (*(*hm).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh3 = (*_hd_hh_del).next;
            } else {
                hm = (*_hd_hh_del).next as *mut MarkHash as *mut MarkHash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh4 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh4 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*hm).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket =
                (*(*hm).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
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
            (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<MarkHash>();
        s_0 = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut MarkHash
            as *mut MarkHash;
    }
}
unsafe extern "C" fn consolidate_base_array(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut options: *const Options,
    mut baseArray: *mut BaseArray,
) {
    let mut hm: *mut BaseHash = ::core::ptr::null_mut::<BaseHash>();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*baseArray).length {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*baseArray).items.offset(k as isize)).glyph,
        ) {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored unknown glyph name ",
                    (*(*baseArray).items.offset(k as isize)).glyph.name,
                    b".",
                ),
            );
        } else {
            let mut s: *mut BaseHash = ::core::ptr::null_mut::<BaseHash>();
            let mut gid: ::core::ffi::c_int =
                (*(*baseArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
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
            let mut current_block_52: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 7016587201154547590;
                }
                10 => {
                    current_block_52 = 7016587201154547590;
                }
                9 => {
                    current_block_52 = 3980250441984174877;
                }
                8 => {
                    current_block_52 = 6148045088452986653;
                }
                7 => {
                    current_block_52 = 18186708060314969588;
                }
                6 => {
                    current_block_52 = 18271855001797298824;
                }
                5 => {
                    current_block_52 = 12249008867511416487;
                }
                4 => {
                    current_block_52 = 5550823838973015271;
                }
                3 => {
                    current_block_52 = 7202498605496425099;
                }
                2 => {
                    current_block_52 = 15113091096903121241;
                }
                1 => {
                    current_block_52 = 3727798181405880336;
                }
                _ => {
                    current_block_52 = 1345366029464561491;
                }
            }
            match current_block_52 {
                7016587201154547590 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 3980250441984174877;
                }
                _ => {}
            }
            match current_block_52 {
                3980250441984174877 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 6148045088452986653;
                }
                _ => {}
            }
            match current_block_52 {
                6148045088452986653 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 18186708060314969588;
                }
                _ => {}
            }
            match current_block_52 {
                18186708060314969588 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 18271855001797298824;
                }
                _ => {}
            }
            match current_block_52 {
                18271855001797298824 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 12249008867511416487;
                }
                _ => {}
            }
            match current_block_52 {
                12249008867511416487 => {
                    _hj_j = _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_52 = 5550823838973015271;
                }
                _ => {}
            }
            match current_block_52 {
                5550823838973015271 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 7202498605496425099;
                }
                _ => {}
            }
            match current_block_52 {
                7202498605496425099 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 15113091096903121241;
                }
                _ => {}
            }
            match current_block_52 {
                15113091096903121241 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 3727798181405880336;
                }
                _ => {}
            }
            match current_block_52 {
                3727798181405880336 => {
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
            s = ::core::ptr::null_mut::<BaseHash>();
            if !hm.is_null() {
                let mut _hf_bkt: ::core::ffi::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize))
                        .hh_head
                        .is_null()
                    {
                        s = ((*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut BaseHash as *mut BaseHash;
                    } else {
                        s = ::core::ptr::null_mut::<BaseHash>();
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
                                .offset(-(*(*hm).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut BaseHash as *mut BaseHash;
                        } else {
                            s = ::core::ptr::null_mut::<BaseHash>();
                        }
                    }
                }
            }
            if s.is_null() {
                s = __caryll_allocate_clean(
                    ::core::mem::size_of::<BaseHash>() as usize,
                    87 as ::core::ffi::c_ulong,
                ) as *mut BaseHash;
                (*s).gid =
                    (*(*baseArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
                (*s).name = sdsdup((*(*baseArray).items.offset(k as isize)).glyph.name);
                (*s).anchors = (*(*baseArray).items.offset(k as isize)).anchors;
                let ref mut fresh0 = (*(*baseArray).items.offset(k as isize)).anchors;
                *fresh0 = ::core::ptr::null_mut::<Anchor>();
                let mut _ha_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i_0: ::core::ffi::c_uint = 0;
                let mut _hj_j_0: ::core::ffi::c_uint = 0;
                let mut _hj_k_0: ::core::ffi::c_uint = 0;
                let mut _hj_key_0: *const ::core::ffi::c_uchar =
                    &raw mut (*s).gid as *const ::core::ffi::c_uchar;
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
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint
                );
                let mut current_block_171: u64;
                match _hj_k_0 {
                    11 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 17956245827096646122;
                    }
                    10 => {
                        current_block_171 = 17956245827096646122;
                    }
                    9 => {
                        current_block_171 = 17451773831962767405;
                    }
                    8 => {
                        current_block_171 = 2555747926156542244;
                    }
                    7 => {
                        current_block_171 = 3671894898333869379;
                    }
                    6 => {
                        current_block_171 = 18122161107652318248;
                    }
                    5 => {
                        current_block_171 = 10637280720788854375;
                    }
                    4 => {
                        current_block_171 = 3528141965437604235;
                    }
                    3 => {
                        current_block_171 = 15534641122025353471;
                    }
                    2 => {
                        current_block_171 = 7633517610621306592;
                    }
                    1 => {
                        current_block_171 = 387405325351757541;
                    }
                    _ => {
                        current_block_171 = 7315983924538012637;
                    }
                }
                match current_block_171 {
                    17956245827096646122 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 17451773831962767405;
                    }
                    _ => {}
                }
                match current_block_171 {
                    17451773831962767405 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 2555747926156542244;
                    }
                    _ => {}
                }
                match current_block_171 {
                    2555747926156542244 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 3671894898333869379;
                    }
                    _ => {}
                }
                match current_block_171 {
                    3671894898333869379 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 18122161107652318248;
                    }
                    _ => {}
                }
                match current_block_171 {
                    18122161107652318248 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 10637280720788854375;
                    }
                    _ => {}
                }
                match current_block_171 {
                    10637280720788854375 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_171 = 3528141965437604235;
                    }
                    _ => {}
                }
                match current_block_171 {
                    3528141965437604235 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_171 = 15534641122025353471;
                    }
                    _ => {}
                }
                match current_block_171 {
                    15534641122025353471 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_171 = 7633517610621306592;
                    }
                    _ => {}
                }
                match current_block_171 {
                    7633517610621306592 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_171 = 387405325351757541;
                    }
                    _ => {}
                }
                match current_block_171 {
                    387405325351757541 => {
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
                    &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
                (*s).hh.keylen =
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                if hm.is_null() {
                    (*s).hh.next = NULL;
                    (*s).hh.prev = NULL;
                    (*s).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                        as *mut UtHashTable
                        as *mut UtHashTable;
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
                                (32 as usize).wrapping_mul(
                                    ::core::mem::size_of::<UtHashBucket>() as usize,
                                ),
                            );
                        }
                    }
                    hm = s;
                } else {
                    (*s).hh.tbl = (*hm).hh.tbl;
                    (*s).hh.next = NULL;
                    (*s).hh.prev = ((*(*hm).hh.tbl).tail as *mut ::core::ffi::c_char)
                        .offset(-(*(*hm).hh.tbl).hho)
                        as *mut ::core::ffi::c_void;
                    (*(*(*hm).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                    (*(*hm).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
                }
                let mut _ha_bkt: ::core::ffi::c_uint = 0;
                (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_add(1);
                _ha_bkt = _ha_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head: *mut UtHashBucket =
                    (*(*hm).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
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
            } else {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignored anchor double-definition for /",
                        (*(*baseArray).items.offset(k as isize)).glyph.name,
                        b".",
                    ),
                );
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
    if !hm.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*hm).hh as *mut UtHashHandle;
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
                            .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
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
                                    .offset((*(*hm).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if base_by_gid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut BaseHash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut BaseHash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*hm).hh.tbl).hho)
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
                                .offset(-(*(*hm).hh.tbl).hho)
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
                (*(*hm).hh.tbl).tail = _hs_tail;
                hm = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut BaseHash
                    as *mut BaseHash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    OTL_I_BASE_ARRAY.clear.expect("non-null function pointer")(baseArray);
    let mut s_0: *mut BaseHash = ::core::ptr::null_mut::<BaseHash>();
    let mut tmp: *mut BaseHash = ::core::ptr::null_mut::<BaseHash>();
    s_0 = hm;
    tmp = (if !hm.is_null() { (*hm).hh.next } else { NULL }) as *mut BaseHash as *mut BaseHash;
    while !s_0.is_null() {
        OTL_I_BASE_ARRAY.push.expect("non-null function pointer")(
            baseArray,
            BaseRecord {
                glyph: handle_from_consolidated(
                    (*s_0).gid as GlyphId, (*s_0).name
                ) as GlyphHandle,
                anchors: (*s_0).anchors,
            },
        );
        sdsfree((*s_0).name);
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s_0).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*hm).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*hm).hh.tbl as *mut ::core::ffi::c_void);
            hm = ::core::ptr::null_mut::<BaseHash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*hm).hh.tbl).tail {
                (*(*hm).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh1 = (*_hd_hh_del).next;
            } else {
                hm = (*_hd_hh_del).next as *mut BaseHash as *mut BaseHash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh2 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*hm).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket =
                (*(*hm).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
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
            (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<BaseHash>();
        s_0 = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut BaseHash
            as *mut BaseHash;
    }
}
unsafe extern "C" fn consolidate_lig_array(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut options: *const Options,
    mut ligArray: *mut LigatureArray,
) {
    let mut hm: *mut LigHash = ::core::ptr::null_mut::<LigHash>();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*ligArray).length {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*ligArray).items.offset(k as isize)).glyph,
        ) {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored unknown glyph name ",
                    (*(*ligArray).items.offset(k as isize)).glyph.name,
                    b".",
                ),
            );
        } else {
            let mut s: *mut LigHash = ::core::ptr::null_mut::<LigHash>();
            let mut gid: ::core::ffi::c_int =
                (*(*ligArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
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
            let mut current_block_52: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 671631640629127466;
                }
                10 => {
                    current_block_52 = 671631640629127466;
                }
                9 => {
                    current_block_52 = 2507948425875615653;
                }
                8 => {
                    current_block_52 = 11781834747162053735;
                }
                7 => {
                    current_block_52 = 4976633191839108100;
                }
                6 => {
                    current_block_52 = 874960838666993744;
                }
                5 => {
                    current_block_52 = 16560270646560938773;
                }
                4 => {
                    current_block_52 = 4552817509067871589;
                }
                3 => {
                    current_block_52 = 10488484428681960669;
                }
                2 => {
                    current_block_52 = 10475785614112867771;
                }
                1 => {
                    current_block_52 = 9334142406440380371;
                }
                _ => {
                    current_block_52 = 1345366029464561491;
                }
            }
            match current_block_52 {
                671631640629127466 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 2507948425875615653;
                }
                _ => {}
            }
            match current_block_52 {
                2507948425875615653 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 11781834747162053735;
                }
                _ => {}
            }
            match current_block_52 {
                11781834747162053735 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 4976633191839108100;
                }
                _ => {}
            }
            match current_block_52 {
                4976633191839108100 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 874960838666993744;
                }
                _ => {}
            }
            match current_block_52 {
                874960838666993744 => {
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 16560270646560938773;
                }
                _ => {}
            }
            match current_block_52 {
                16560270646560938773 => {
                    _hj_j = _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_52 = 4552817509067871589;
                }
                _ => {}
            }
            match current_block_52 {
                4552817509067871589 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_52 = 10488484428681960669;
                }
                _ => {}
            }
            match current_block_52 {
                10488484428681960669 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_52 = 10475785614112867771;
                }
                _ => {}
            }
            match current_block_52 {
                10475785614112867771 => {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_52 = 9334142406440380371;
                }
                _ => {}
            }
            match current_block_52 {
                9334142406440380371 => {
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
            s = ::core::ptr::null_mut::<LigHash>();
            if !hm.is_null() {
                let mut _hf_bkt: ::core::ffi::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize))
                        .hh_head
                        .is_null()
                    {
                        s = ((*(*(*hm).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut LigHash
                            as *mut LigHash;
                    } else {
                        s = ::core::ptr::null_mut::<LigHash>();
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
                                .offset(-(*(*hm).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut LigHash as *mut LigHash;
                        } else {
                            s = ::core::ptr::null_mut::<LigHash>();
                        }
                    }
                }
            }
            if s.is_null() {
                s = __caryll_allocate_clean(
                    ::core::mem::size_of::<LigHash>() as usize,
                    125 as ::core::ffi::c_ulong,
                ) as *mut LigHash;
                (*s).gid =
                    (*(*ligArray).items.offset(k as isize)).glyph.index as ::core::ffi::c_int;
                (*s).name = sdsdup((*(*ligArray).items.offset(k as isize)).glyph.name);
                (*s).componentCount = (*(*ligArray).items.offset(k as isize)).componentCount;
                (*s).anchors = (*(*ligArray).items.offset(k as isize)).anchors;
                let ref mut fresh5 = (*(*ligArray).items.offset(k as isize)).anchors;
                *fresh5 = ::core::ptr::null_mut::<*mut Anchor>();
                let mut _ha_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i_0: ::core::ffi::c_uint = 0;
                let mut _hj_j_0: ::core::ffi::c_uint = 0;
                let mut _hj_k_0: ::core::ffi::c_uint = 0;
                let mut _hj_key_0: *const ::core::ffi::c_uchar =
                    &raw mut (*s).gid as *const ::core::ffi::c_uchar;
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
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint
                );
                let mut current_block_172: u64;
                match _hj_k_0 {
                    11 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_172 = 6992567993738996300;
                    }
                    10 => {
                        current_block_172 = 6992567993738996300;
                    }
                    9 => {
                        current_block_172 = 10765054353708945446;
                    }
                    8 => {
                        current_block_172 = 8954403608661305380;
                    }
                    7 => {
                        current_block_172 = 1261730542405161010;
                    }
                    6 => {
                        current_block_172 = 4662756573773047525;
                    }
                    5 => {
                        current_block_172 = 5898464050339554315;
                    }
                    4 => {
                        current_block_172 = 4359070003714544862;
                    }
                    3 => {
                        current_block_172 = 10064774401823359594;
                    }
                    2 => {
                        current_block_172 = 114102700035871186;
                    }
                    1 => {
                        current_block_172 = 6262880948454574332;
                    }
                    _ => {
                        current_block_172 = 939350892795860671;
                    }
                }
                match current_block_172 {
                    6992567993738996300 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_172 = 10765054353708945446;
                    }
                    _ => {}
                }
                match current_block_172 {
                    10765054353708945446 => {
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_172 = 8954403608661305380;
                    }
                    _ => {}
                }
                match current_block_172 {
                    8954403608661305380 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_172 = 1261730542405161010;
                    }
                    _ => {}
                }
                match current_block_172 {
                    1261730542405161010 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_172 = 4662756573773047525;
                    }
                    _ => {}
                }
                match current_block_172 {
                    4662756573773047525 => {
                        _hj_j_0 = _hj_j_0.wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_172 = 5898464050339554315;
                    }
                    _ => {}
                }
                match current_block_172 {
                    5898464050339554315 => {
                        _hj_j_0 = _hj_j_0
                            .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_172 = 4359070003714544862;
                    }
                    _ => {}
                }
                match current_block_172 {
                    4359070003714544862 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_172 = 10064774401823359594;
                    }
                    _ => {}
                }
                match current_block_172 {
                    10064774401823359594 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_172 = 114102700035871186;
                    }
                    _ => {}
                }
                match current_block_172 {
                    114102700035871186 => {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_172 = 6262880948454574332;
                    }
                    _ => {}
                }
                match current_block_172 {
                    6262880948454574332 => {
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
                    &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
                (*s).hh.keylen =
                    ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                if hm.is_null() {
                    (*s).hh.next = NULL;
                    (*s).hh.prev = NULL;
                    (*s).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                        as *mut UtHashTable
                        as *mut UtHashTable;
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
                                (32 as usize).wrapping_mul(
                                    ::core::mem::size_of::<UtHashBucket>() as usize,
                                ),
                            );
                        }
                    }
                    hm = s;
                } else {
                    (*s).hh.tbl = (*hm).hh.tbl;
                    (*s).hh.next = NULL;
                    (*s).hh.prev = ((*(*hm).hh.tbl).tail as *mut ::core::ffi::c_char)
                        .offset(-(*(*hm).hh.tbl).hho)
                        as *mut ::core::ffi::c_void;
                    (*(*(*hm).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                    (*(*hm).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
                }
                let mut _ha_bkt: ::core::ffi::c_uint = 0;
                (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_add(1);
                _ha_bkt = _ha_hashv
                    & (*(*hm).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head: *mut UtHashBucket =
                    (*(*hm).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
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
            } else {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignored anchor double-definition for /",
                        (*(*ligArray).items.offset(k as isize)).glyph.name,
                        b".",
                    ),
                );
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
    if !hm.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*hm).hh as *mut UtHashHandle;
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
                            .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
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
                                    .offset((*(*hm).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if lig_by_gid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut LigHash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut LigHash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*hm).hh.tbl).hho)
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
                                .offset((*(*hm).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*hm).hh.tbl).hho)
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
                                .offset(-(*(*hm).hh.tbl).hho)
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
                (*(*hm).hh.tbl).tail = _hs_tail;
                hm = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(*hm).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut LigHash
                    as *mut LigHash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    OTL_I_LIGATURE_ARRAY.clear.expect("non-null function pointer")(ligArray);
    let mut s_0: *mut LigHash = ::core::ptr::null_mut::<LigHash>();
    let mut tmp: *mut LigHash = ::core::ptr::null_mut::<LigHash>();
    s_0 = hm;
    tmp = (if !hm.is_null() { (*hm).hh.next } else { NULL }) as *mut LigHash as *mut LigHash;
    while !s_0.is_null() {
        OTL_I_LIGATURE_ARRAY.push.expect("non-null function pointer")(
            ligArray,
            LigatureBaseRecord {
                glyph: handle_from_consolidated(
                    (*s_0).gid as GlyphId, (*s_0).name
                ) as GlyphHandle,
                componentCount: (*s_0).componentCount,
                anchors: (*s_0).anchors,
            },
        );
        sdsfree((*s_0).name);
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s_0).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*hm).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*hm).hh.tbl as *mut ::core::ffi::c_void);
            hm = ::core::ptr::null_mut::<LigHash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*hm).hh.tbl).tail {
                (*(*hm).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh6 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh6 = (*_hd_hh_del).next;
            } else {
                hm = (*_hd_hh_del).next as *mut LigHash as *mut LigHash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh7 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*hm).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh7 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*hm).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket =
                (*(*hm).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
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
            (*(*hm).hh.tbl).num_items = (*(*hm).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<LigHash>();
        s_0 = tmp;
        tmp =
            (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut LigHash as *mut LigHash;
    }
}
pub unsafe extern "C" fn consolidate_mark_to_single(
    mut font: *mut Font,
    mut table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GposMarkToSingleSubtable = &raw mut (*_subtable).gpos_markToSingle;
    consolidate_mark_array(
        font,
        table,
        options,
        &raw mut (*subtable).markArray,
        (*subtable).classCount,
    );
    consolidate_base_array(font, table, options, &raw mut (*subtable).baseArray);
    return (*subtable).markArray.length == 0 as usize
        || (*subtable).baseArray.length == 0 as usize;
}
pub unsafe extern "C" fn consolidate_mark_to_ligature(
    mut font: *mut Font,
    mut table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GposMarkToLigatureSubtable = &raw mut (*_subtable).gpos_markToLigature;
    consolidate_mark_array(
        font,
        table,
        options,
        &raw mut (*subtable).markArray,
        (*subtable).classCount,
    );
    consolidate_lig_array(font, table, options, &raw mut (*subtable).ligArray);
    return (*subtable).markArray.length == 0 as usize
        || (*subtable).ligArray.length == 0 as usize;
}
