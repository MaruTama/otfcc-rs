use libc::{exit, free, malloc, memcmp, memset};
extern "C" {
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static iSubtable_gsub_multi: __caryll_vectorinterface_subtable_gsub_multi;
    static otl_iCoverage: __otfcc_ICoverage;
    fn fontop_consolidateCoverage(
        font: *mut otfcc_Font,
        coverage: *mut otl_Coverage,
        options: *const otfcc_Options,
    );
}

use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, shrinkCoverage};
use crate::support::handle::{handle_fromConsolidated, otfcc_GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};

use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphid_t};
use crate::vendor::sds::{sds};

use crate::font::caryll_font::{otfcc_Font};
use crate::support::{NULL};
use crate::support::glyph_order::{otfcc_GlyphOrderPackage};
























use crate::table::otl::{__caryll_vectorinterface_subtable_gsub_multi, otl_GsubMultiEntry, otl_Subtable, subtable_gsub_multi, table_OTL};





use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UT_hash_bucket, UT_hash_handle, UT_hash_table};




#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsub_multi_hash {
    pub fromid: ::core::ffi::c_int,
    pub fromname: sds,
    pub to: *mut otl_Coverage,
    pub hh: UT_hash_handle,
}
unsafe extern "C" fn by_from_id_multi(
    mut a: *mut gsub_multi_hash,
    mut b: *mut gsub_multi_hash,
) -> ::core::ffi::c_int {
    return (*a).fromid - (*b).fromid;
}
#[no_mangle]
pub unsafe extern "C" fn consolidate_gsub_multi(
    mut font: *mut otfcc_Font,
    mut _table: *mut table_OTL,
    mut _subtable: *mut otl_Subtable,
    mut options: *const otfcc_Options,
) -> bool {
    let mut subtable: *mut subtable_gsub_multi = &raw mut (*_subtable).gsub_multi;
    let mut h: *mut gsub_multi_hash = ::core::ptr::null_mut::<gsub_multi_hash>();
    let mut k: glyphid_t = 0 as glyphid_t;
    while (k as usize) < (*subtable).length {
        if !otfcc_pkgGlyphOrder
            .consolidateHandle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (*(*subtable).items.offset(k as isize)).from,
        ) {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"[Consolidate] Ignored missing glyph /%s.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*(*subtable).items.offset(k as isize)).from.name,
                ),
            );
        } else {
            fontop_consolidateCoverage(font, (*(*subtable).items.offset(k as isize)).to, options);
            shrinkCoverage(
                (*(*subtable).items.offset(k as isize)).to,
                false,
            );
            if (*(*(*subtable).items.offset(k as isize)).to).numGlyphs == 0 {
                (*(*options).logger)
                    .logSDS
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_important as ::core::ffi::c_int as u8,
                    log_type_warning,
                    sdscatprintf(
                        sdsempty(),
                        b"[Consolidate] Ignoring empty one-to-many / alternative substitution for glyph /%s.\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        (*(*subtable).items.offset(k as isize)).from.name,
                    ),
                );
            } else {
                let mut s: *mut gsub_multi_hash = ::core::ptr::null_mut::<gsub_multi_hash>();
                let mut fromid: ::core::ffi::c_int =
                    (*(*subtable).items.offset(k as isize)).from.index as ::core::ffi::c_int;
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
                let mut current_block_56: u64;
                match _hj_k {
                    11 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_56 = 12236482770265525966;
                    }
                    10 => {
                        current_block_56 = 12236482770265525966;
                    }
                    9 => {
                        current_block_56 = 988185777200269682;
                    }
                    8 => {
                        current_block_56 = 10686949387425854293;
                    }
                    7 => {
                        current_block_56 = 14202788594208270599;
                    }
                    6 => {
                        current_block_56 = 5236565738929723439;
                    }
                    5 => {
                        current_block_56 = 9202226192260729389;
                    }
                    4 => {
                        current_block_56 = 3731248701347877511;
                    }
                    3 => {
                        current_block_56 = 11706360267266548301;
                    }
                    2 => {
                        current_block_56 = 15181912925471310088;
                    }
                    1 => {
                        current_block_56 = 9979634286774146994;
                    }
                    _ => {
                        current_block_56 = 11793792312832361944;
                    }
                }
                match current_block_56 {
                    12236482770265525966 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_56 = 988185777200269682;
                    }
                    _ => {}
                }
                match current_block_56 {
                    988185777200269682 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_56 = 10686949387425854293;
                    }
                    _ => {}
                }
                match current_block_56 {
                    10686949387425854293 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_56 = 14202788594208270599;
                    }
                    _ => {}
                }
                match current_block_56 {
                    14202788594208270599 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_56 = 5236565738929723439;
                    }
                    _ => {}
                }
                match current_block_56 {
                    5236565738929723439 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_56 = 9202226192260729389;
                    }
                    _ => {}
                }
                match current_block_56 {
                    9202226192260729389 => {
                        _hj_j = _hj_j
                            .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_56 = 3731248701347877511;
                    }
                    _ => {}
                }
                match current_block_56 {
                    3731248701347877511 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_56 = 11706360267266548301;
                    }
                    _ => {}
                }
                match current_block_56 {
                    11706360267266548301 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_56 = 15181912925471310088;
                    }
                    _ => {}
                }
                match current_block_56 {
                    15181912925471310088 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_56 = 9979634286774146994;
                    }
                    _ => {}
                }
                match current_block_56 {
                    9979634286774146994 => {
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
                s = ::core::ptr::null_mut::<gsub_multi_hash>();
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
                                as *mut gsub_multi_hash
                                as *mut gsub_multi_hash;
                        } else {
                            s = ::core::ptr::null_mut::<gsub_multi_hash>();
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
                                    as *mut gsub_multi_hash
                                    as *mut gsub_multi_hash;
                            } else {
                                s = ::core::ptr::null_mut::<gsub_multi_hash>();
                            }
                        }
                    }
                }
                if s.is_null() {
                    s = __caryll_allocate_clean(
                        ::core::mem::size_of::<gsub_multi_hash>() as usize,
                        36 as ::core::ffi::c_ulong,
                    ) as *mut gsub_multi_hash;
                    (*s).fromid =
                        (*(*subtable).items.offset(k as isize)).from.index as ::core::ffi::c_int;
                    (*s).fromname = sdsdup((*(*subtable).items.offset(k as isize)).from.name);
                    (*s).to = (*(*subtable).items.offset(k as isize)).to;
                    let ref mut fresh0 = (*(*subtable).items.offset(k as isize)).to;
                    *fresh0 = ::core::ptr::null_mut::<otl_Coverage>();
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
                    let mut current_block_175: u64;
                    match _hj_k_0 {
                        11 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_175 = 12833738958244716968;
                        }
                        10 => {
                            current_block_175 = 12833738958244716968;
                        }
                        9 => {
                            current_block_175 = 260625557895029612;
                        }
                        8 => {
                            current_block_175 = 14167539630123950245;
                        }
                        7 => {
                            current_block_175 = 3031444430934624633;
                        }
                        6 => {
                            current_block_175 = 10544429332333896974;
                        }
                        5 => {
                            current_block_175 = 13571897817355867746;
                        }
                        4 => {
                            current_block_175 = 2524143026478004996;
                        }
                        3 => {
                            current_block_175 = 6202706136215307904;
                        }
                        2 => {
                            current_block_175 = 4858086721458057569;
                        }
                        1 => {
                            current_block_175 = 10630599433215428982;
                        }
                        _ => {
                            current_block_175 = 4338462691184853296;
                        }
                    }
                    match current_block_175 {
                        12833738958244716968 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_175 = 260625557895029612;
                        }
                        _ => {}
                    }
                    match current_block_175 {
                        260625557895029612 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_175 = 14167539630123950245;
                        }
                        _ => {}
                    }
                    match current_block_175 {
                        14167539630123950245 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_175 = 3031444430934624633;
                        }
                        _ => {}
                    }
                    match current_block_175 {
                        3031444430934624633 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_175 = 10544429332333896974;
                        }
                        _ => {}
                    }
                    match current_block_175 {
                        10544429332333896974 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_175 = 13571897817355867746;
                        }
                        _ => {}
                    }
                    match current_block_175 {
                        13571897817355867746 => {
                            _hj_j_0 = _hj_j_0
                                .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_175 = 2524143026478004996;
                        }
                        _ => {}
                    }
                    match current_block_175 {
                        2524143026478004996 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_175 = 6202706136215307904;
                        }
                        _ => {}
                    }
                    match current_block_175 {
                        6202706136215307904 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_175 = 4858086721458057569;
                        }
                        _ => {}
                    }
                    match current_block_175 {
                        4858086721458057569 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_175 = 10630599433215428982;
                        }
                        _ => {}
                    }
                    match current_block_175 {
                        10630599433215428982 => {
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
                    (*s).hh.key = &raw mut (*s).fromid as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void;
                    (*s).hh.keylen =
                        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
                    if h.is_null() {
                        (*s).hh.next = NULL;
                        (*s).hh.prev = NULL;
                        (*s).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                            as *mut UT_hash_table
                            as *mut UT_hash_table;
                        if (*s).hh.tbl.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*s).hh.tbl as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                ::core::mem::size_of::<UT_hash_table>() as usize,
                            );
                            (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
                            (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                            (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                            (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                                .offset_from(s as *mut ::core::ffi::c_char)
                                as ::core::ffi::c_long
                                as isize;
                            (*(*s).hh.tbl).buckets =
                                malloc((32 as usize).wrapping_mul(::core::mem::size_of::<
                                    UT_hash_bucket,
                                >(
                                )
                                    as usize))
                                    as *mut UT_hash_bucket;
                            (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                            if (*(*s).hh.tbl).buckets.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (32 as usize).wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as usize,
                                    ),
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
                        (*(*h).hh.tbl).tail = &raw mut (*s).hh as *mut UT_hash_handle;
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
                    (*s).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
                    (*s).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                    if !(*_ha_head).hh_head.is_null() {
                        (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UT_hash_handle;
                    }
                    (*_ha_head).hh_head = &raw mut (*s).hh as *mut UT_hash_handle;
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
                                .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                        ) as *mut UT_hash_bucket;
                        if _he_new_buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                _he_new_buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (2 as usize)
                                    .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UT_hash_bucket>() as usize
                                    ),
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
                                _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize))
                                    .hh_head
                                    as *mut UT_hash_handle;
                                while !_he_thh.is_null() {
                                    _he_hh_nxt = (*_he_thh).hh_next;
                                    _he_bkt = (*_he_thh).hashv
                                        & (*(*s).hh.tbl)
                                            .num_buckets
                                            .wrapping_mul(2 as ::core::ffi::c_uint)
                                            .wrapping_sub(1 as ::core::ffi::c_uint);
                                    _he_newbkt = _he_new_buckets.offset(_he_bkt as isize)
                                        as *mut UT_hash_bucket;
                                    (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                    if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                        (*(*s).hh.tbl).nonideal_items =
                                            (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                        (*_he_newbkt).expand_mult = (*_he_newbkt)
                                            .count
                                            .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
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
    let mut _hs_p: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_q: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_e: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_list: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_tail: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    if !h.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*h).hh as *mut UT_hash_handle;
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
                            .offset((*(*h).hh.tbl).hho)
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
                                .offset((*(*h).hh.tbl).hho)
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
                                    .offset((*(*h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if by_from_id_multi(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut gsub_multi_hash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut gsub_multi_hash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*h).hh.tbl).hho)
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
                                .offset((*(*h).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
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
                    as *mut ::core::ffi::c_void as *mut gsub_multi_hash
                    as *mut gsub_multi_hash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    iSubtable_gsub_multi
        .clear
        .expect("non-null function pointer")(subtable);
    let mut s_0: *mut gsub_multi_hash = ::core::ptr::null_mut::<gsub_multi_hash>();
    let mut tmp: *mut gsub_multi_hash = ::core::ptr::null_mut::<gsub_multi_hash>();
    s_0 = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut gsub_multi_hash
        as *mut gsub_multi_hash;
    while !s_0.is_null() {
        iSubtable_gsub_multi
            .push
            .expect("non-null function pointer")(
            subtable,
            otl_GsubMultiEntry {
                from: handle_fromConsolidated(
                    (*s_0).fromid as glyphid_t,
                    (*s_0).fromname,
                ) as otfcc_GlyphHandle,
                to: (*s_0).to,
            },
        );
        sdsfree((*s_0).fromname);
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s_0).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<gsub_multi_hash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh1 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut gsub_multi_hash as *mut gsub_multi_hash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh2 = (*_hd_hh_del).prev;
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
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<gsub_multi_hash>();
        s_0 = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut gsub_multi_hash
            as *mut gsub_multi_hash;
    }
    return (*subtable).length == 0 as usize;
}
#[no_mangle]
pub unsafe extern "C" fn consolidate_gsub_alternative(
    mut font: *mut otfcc_Font,
    mut table: *mut table_OTL,
    mut _subtable: *mut otl_Subtable,
    mut options: *const otfcc_Options,
) -> bool {
    return consolidate_gsub_multi(font, table, _subtable, options);
}
