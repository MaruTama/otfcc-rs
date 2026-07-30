#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset, strcmp, strtol};





use crate::support::json_funcs::{json_obj_get_type};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::otf_reader::FontBuilder;
use crate::logger::{LoggerType, LOG_VL_NOTICE, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{JsonType, JsonValue};
use crate::font::caryll_font::{FontSubtype, Font, IFontBuilder};
use crate::support::{NULL};
use crate::support::glyph_order::{GlyphOrderPass, GlyphOrder, GlyphOrderEntry};






use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::font::caryll_font::{OTFCC_I_FONT};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::table::BASE::{otfcc_parse_base};
use crate::table::CFF::{otfcc_parse_cff};
use crate::table::COLR::{otfcc_parse_colr};
use crate::table::CPAL::{otfcc_parse_cpal};
use crate::table::GDEF::{otfcc_parse_gdef};
use crate::table::OS_2::{otfcc_parse_os_2};
use crate::table::SVG::{otfcc_parse_svg};
use crate::table::TSI5::{otfcc_parse_tsi5};
use crate::table::_TSI::{otfcc_parse_tsi};
use crate::table::cmap::{otfcc_parse_cmap};
use crate::table::cvt::{otfcc_parse_cvt};
use crate::table::fpgm_prep::{otfcc_parse_fpgm_prep};
use crate::table::gasp::{otfcc_parse_gasp};
use crate::table::glyf::{otfcc_parse_glyf};
use crate::table::head::{otfcc_parse_head};
use crate::table::hhea::{otfcc_parse_hhea};
use crate::table::maxp::{otfcc_parse_maxp};
use crate::table::meta::parse::{otfcc_parse_meta};
use crate::table::name::{otfcc_parse_name};
use crate::table::otl::parse::{otfcc_parse_otl};
use crate::table::post::{otfcc_parse_post};
use crate::table::vdmx::funcs::{otfcc_parse_vdmx};
use crate::table::vhea::{otfcc_parse_vhea};
use crate::vendor::sds::{sdsempty, sdsfree, sdsnewlen};




#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
unsafe extern "C" fn otfcc_decide_font_subtype_from_json(
    mut root: *const JsonValue,
) -> FontSubtype {
    if !json_obj_get_type(
        root,
        b"CFF_\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    )
    .is_null()
    {
        return FontSubtype::Cff;
    } else {
        return FontSubtype::Ttf;
    };
}
unsafe extern "C" fn set_order_by_name(
    mut go: *mut GlyphOrder,
    mut name: SdsRaw,
    mut order_type: GlyphOrderPass,
    mut order_entry: u32,
) {
    let mut s: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = name as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = sdslen(name) as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(sdslen(name) as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 17804697211240665320;
        }
        10 => {
            current_block_50 = 17804697211240665320;
        }
        9 => {
            current_block_50 = 10934104523405478302;
        }
        8 => {
            current_block_50 = 18021056235773049229;
        }
        7 => {
            current_block_50 = 1545999744855823442;
        }
        6 => {
            current_block_50 = 5785022371778852742;
        }
        5 => {
            current_block_50 = 852838572082876989;
        }
        4 => {
            current_block_50 = 4171976486760612607;
        }
        3 => {
            current_block_50 = 8689672022250159862;
        }
        2 => {
            current_block_50 = 5254886831261953223;
        }
        1 => {
            current_block_50 = 380073568100959624;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        17804697211240665320 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 10934104523405478302;
        }
        _ => {}
    }
    match current_block_50 {
        10934104523405478302 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 18021056235773049229;
        }
        _ => {}
    }
    match current_block_50 {
        18021056235773049229 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 1545999744855823442;
        }
        _ => {}
    }
    match current_block_50 {
        1545999744855823442 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 5785022371778852742;
        }
        _ => {}
    }
    match current_block_50 {
        5785022371778852742 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 852838572082876989;
        }
        _ => {}
    }
    match current_block_50 {
        852838572082876989 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 4171976486760612607;
        }
        _ => {}
    }
    match current_block_50 {
        4171976486760612607 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 8689672022250159862;
        }
        _ => {}
    }
    match current_block_50 {
        8689672022250159862 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 5254886831261953223;
        }
        _ => {}
    }
    match current_block_50 {
        5254886831261953223 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 380073568100959624;
        }
        _ => {}
    }
    match current_block_50 {
        380073568100959624 => {
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
    s = ::core::ptr::null_mut::<GlyphOrderEntry>();
    if !(*go).by_name.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*go).by_name).hh_name.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*go).by_name).hh_name.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                s = ((*(*(*(*go).by_name).hh_name.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).by_name).hh_name.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut GlyphOrderEntry
                    as *mut GlyphOrderEntry;
            } else {
                s = ::core::ptr::null_mut::<GlyphOrderEntry>();
            }
            while !s.is_null() {
                if (*s).hh_name.hashv == _hf_hashv && (*s).hh_name.keylen as usize == sdslen(name) {
                    if memcmp(
                        (*s).hh_name.key,
                        name as *const ::core::ffi::c_void,
                        sdslen(name),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh_name.hh_next.is_null() {
                    s = ((*s).hh_name.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).by_name).hh_name.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut GlyphOrderEntry
                        as *mut GlyphOrderEntry;
                } else {
                    s = ::core::ptr::null_mut::<GlyphOrderEntry>();
                }
            }
        }
    }
    if s.is_null() {
        s = __caryll_allocate_clean(
            ::core::mem::size_of::<GlyphOrderEntry>() as usize,
            21 as ::core::ffi::c_ulong,
        ) as *mut GlyphOrderEntry;
        (*s).gid = -(1 as ::core::ffi::c_int) as GlyphId;
        (*s).name = name;
        (*s).order_type = order_type;
        (*s).order_entry = order_entry;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            (*s).name.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = sdslen((*s).name) as ::core::ffi::c_uint;
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
        _ha_hashv = _ha_hashv.wrapping_add(sdslen((*s).name) as ::core::ffi::c_uint);
        let mut current_block_169: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 12766349834733685609;
            }
            10 => {
                current_block_169 = 12766349834733685609;
            }
            9 => {
                current_block_169 = 921014338214990409;
            }
            8 => {
                current_block_169 = 10918256127192299627;
            }
            7 => {
                current_block_169 = 9578982516296139066;
            }
            6 => {
                current_block_169 = 4934683382580054635;
            }
            5 => {
                current_block_169 = 9066128494507323840;
            }
            4 => {
                current_block_169 = 11126302601059242051;
            }
            3 => {
                current_block_169 = 552284807196668465;
            }
            2 => {
                current_block_169 = 13163467591269913071;
            }
            1 => {
                current_block_169 = 9704386208929738663;
            }
            _ => {
                current_block_169 = 14714495436747744489;
            }
        }
        match current_block_169 {
            12766349834733685609 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 921014338214990409;
            }
            _ => {}
        }
        match current_block_169 {
            921014338214990409 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 10918256127192299627;
            }
            _ => {}
        }
        match current_block_169 {
            10918256127192299627 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 9578982516296139066;
            }
            _ => {}
        }
        match current_block_169 {
            9578982516296139066 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 4934683382580054635;
            }
            _ => {}
        }
        match current_block_169 {
            4934683382580054635 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 9066128494507323840;
            }
            _ => {}
        }
        match current_block_169 {
            9066128494507323840 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_169 = 11126302601059242051;
            }
            _ => {}
        }
        match current_block_169 {
            11126302601059242051 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 552284807196668465;
            }
            _ => {}
        }
        match current_block_169 {
            552284807196668465 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 13163467591269913071;
            }
            _ => {}
        }
        match current_block_169 {
            13163467591269913071 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 9704386208929738663;
            }
            _ => {}
        }
        match current_block_169 {
            9704386208929738663 => {
                _hj_i_0 =
                    _hj_i_0
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
        (*s).hh_name.hashv = _ha_hashv;
        (*s).hh_name.key = (*s).name.offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hh_name.keylen = sdslen((*s).name) as ::core::ffi::c_uint;
        if (*go).by_name.is_null() {
            (*s).hh_name.next = NULL;
            (*s).hh_name.prev = NULL;
            (*s).hh_name.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*s).hh_name.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*s).hh_name.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*s).hh_name.tbl).tail = &raw mut (*s).hh_name as *mut UtHashHandle;
                (*(*s).hh_name.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*s).hh_name.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*s).hh_name.tbl).hho = (&raw mut (*s).hh_name as *mut ::core::ffi::c_char)
                    .offset_from(s as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*s).hh_name.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*s).hh_name.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*s).hh_name.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*s).hh_name.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    );
                }
            }
            (*go).by_name = s;
        } else {
            (*s).hh_name.tbl = (*(*go).by_name).hh_name.tbl;
            (*s).hh_name.next = NULL;
            (*s).hh_name.prev = ((*(*(*go).by_name).hh_name.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*go).by_name).hh_name.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*go).by_name).hh_name.tbl).tail).next = s as *mut ::core::ffi::c_void;
            (*(*(*go).by_name).hh_name.tbl).tail = &raw mut (*s).hh_name as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*go).by_name).hh_name.tbl).num_items =
            (*(*(*go).by_name).hh_name.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*go).by_name).hh_name.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket = (*(*(*go).by_name).hh_name.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UtHashBucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*s).hh_name.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
        (*s).hh_name.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh_name as *mut UtHashHandle;
        }
        (*_ha_head).hh_head = &raw mut (*s).hh_name as *mut UtHashHandle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*s).hh_name.tbl).noexpand == 0
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
                    .wrapping_mul((*(*s).hh_name.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*s).hh_name.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                );
                (*(*s).hh_name.tbl).ideal_chain_maxlen = ((*(*s).hh_name.tbl).num_items
                    >> (*(*s).hh_name.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*s).hh_name.tbl).num_items
                        & (*(*s).hh_name.tbl)
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
                (*(*s).hh_name.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*s).hh_name.tbl).num_buckets {
                    _he_thh = (*(*(*s).hh_name.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UtHashHandle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*s).hh_name.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*s).hh_name.tbl).ideal_chain_maxlen {
                            (*(*s).hh_name.tbl).nonideal_items =
                                (*(*s).hh_name.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*s).hh_name.tbl).ideal_chain_maxlen);
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
                free((*(*s).hh_name.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*s).hh_name.tbl).num_buckets = (*(*s).hh_name.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*s).hh_name.tbl).log2_num_buckets =
                    (*(*s).hh_name.tbl).log2_num_buckets.wrapping_add(1);
                (*(*s).hh_name.tbl).buckets = _he_new_buckets;
                (*(*s).hh_name.tbl).ineff_expands = if (*(*s).hh_name.tbl).nonideal_items
                    > (*(*s).hh_name.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*s).hh_name.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*s).hh_name.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*s).hh_name.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
    } else if (*s).order_type > order_type {
        (*s).order_type = order_type;
        (*s).order_entry = order_entry;
    }
}
unsafe extern "C" fn _by_order(
    mut a: *mut GlyphOrderEntry,
    mut b: *mut GlyphOrderEntry,
) -> ::core::ffi::c_int {
    if (*a).order_type < (*b).order_type {
        return -(1 as ::core::ffi::c_int);
    }
    if (*a).order_type > (*b).order_type {
        return 1 as ::core::ffi::c_int;
    }
    if (*a).order_entry < (*b).order_entry {
        return -(1 as ::core::ffi::c_int);
    }
    if (*a).order_entry > (*b).order_entry {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn order_glyphs(mut go: *mut GlyphOrder) {
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
    if !(*go).by_name.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*(*go).by_name).hh_name as *mut UtHashHandle;
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
                            .offset((*(*(*go).by_name).hh_name.tbl).hho)
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
                                .offset((*(*(*go).by_name).hh_name.tbl).hho)
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
                                    .offset((*(*(*go).by_name).hh_name.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if _by_order(
                        (_hs_p as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*go).by_name).hh_name.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut GlyphOrderEntry,
                        (_hs_q as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*go).by_name).hh_name.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut GlyphOrderEntry,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*(*go).by_name).hh_name.tbl).hho)
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
                                .offset((*(*(*go).by_name).hh_name.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*(*go).by_name).hh_name.tbl).hho)
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
                                .offset(-(*(*(*go).by_name).hh_name.tbl).hho)
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
                (*(*(*go).by_name).hh_name.tbl).tail = _hs_tail;
                (*go).by_name = (_hs_list as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).by_name).hh_name.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut GlyphOrderEntry
                    as *mut GlyphOrderEntry;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut current: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
    let mut temp: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
    let mut gid: GlyphId = 0 as GlyphId;
    current = (*go).by_name;
    temp = (if !(*go).by_name.is_null() {
        (*(*go).by_name).hh_name.next
    } else {
        NULL
    }) as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
    while !current.is_null() {
        (*current).gid = gid;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key: *const ::core::ffi::c_uchar =
            &raw mut (*current).gid as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint;
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
            _ha_hashv = _ha_hashv.wrapping_add(
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
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv =
            _ha_hashv.wrapping_add(::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint);
        let mut current_block_122: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_122 = 6107916429970634913;
            }
            10 => {
                current_block_122 = 6107916429970634913;
            }
            9 => {
                current_block_122 = 16824043896303109816;
            }
            8 => {
                current_block_122 = 8056315725412367711;
            }
            7 => {
                current_block_122 = 3473396220534792688;
            }
            6 => {
                current_block_122 = 6187632812825568025;
            }
            5 => {
                current_block_122 = 15028834309073582064;
            }
            4 => {
                current_block_122 = 4789897784655735301;
            }
            3 => {
                current_block_122 = 16804586237124906222;
            }
            2 => {
                current_block_122 = 1446524650436596843;
            }
            1 => {
                current_block_122 = 3046610167049820456;
            }
            _ => {
                current_block_122 = 15622658527355336244;
            }
        }
        match current_block_122 {
            6107916429970634913 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_122 = 16824043896303109816;
            }
            _ => {}
        }
        match current_block_122 {
            16824043896303109816 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_122 = 8056315725412367711;
            }
            _ => {}
        }
        match current_block_122 {
            8056315725412367711 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_122 = 3473396220534792688;
            }
            _ => {}
        }
        match current_block_122 {
            3473396220534792688 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_122 = 6187632812825568025;
            }
            _ => {}
        }
        match current_block_122 {
            6187632812825568025 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_122 = 15028834309073582064;
            }
            _ => {}
        }
        match current_block_122 {
            15028834309073582064 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_122 = 4789897784655735301;
            }
            _ => {}
        }
        match current_block_122 {
            4789897784655735301 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_122 = 16804586237124906222;
            }
            _ => {}
        }
        match current_block_122 {
            16804586237124906222 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_122 = 1446524650436596843;
            }
            _ => {}
        }
        match current_block_122 {
            1446524650436596843 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_122 = 3046610167049820456;
            }
            _ => {}
        }
        match current_block_122 {
            3046610167049820456 => {
                _hj_i =
                    _hj_i
                        .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        (*current).hh_id.hashv = _ha_hashv;
        (*current).hh_id.key =
            &raw mut (*current).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*current).hh_id.keylen = ::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint;
        if (*go).by_gid.is_null() {
            (*current).hh_id.next = NULL;
            (*current).hh_id.prev = NULL;
            (*current).hh_id.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*current).hh_id.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*current).hh_id.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*current).hh_id.tbl).tail = &raw mut (*current).hh_id as *mut UtHashHandle;
                (*(*current).hh_id.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*current).hh_id.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*current).hh_id.tbl).hho = (&raw mut (*current).hh_id as *mut ::core::ffi::c_char)
                    .offset_from(current as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long
                    as isize;
                (*(*current).hh_id.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*current).hh_id.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*current).hh_id.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*current).hh_id.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    );
                }
            }
            (*go).by_gid = current;
        } else {
            (*current).hh_id.tbl = (*(*go).by_gid).hh_id.tbl;
            (*current).hh_id.next = NULL;
            (*current).hh_id.prev = ((*(*(*go).by_gid).hh_id.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*go).by_gid).hh_id.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*go).by_gid).hh_id.tbl).tail).next = current as *mut ::core::ffi::c_void;
            (*(*(*go).by_gid).hh_id.tbl).tail = &raw mut (*current).hh_id as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*go).by_gid).hh_id.tbl).num_items = (*(*(*go).by_gid).hh_id.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*go).by_gid).hh_id.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket =
            (*(*(*go).by_gid).hh_id.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*current).hh_id.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
        (*current).hh_id.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*current).hh_id as *mut UtHashHandle;
        }
        (*_ha_head).hh_head = &raw mut (*current).hh_id as *mut UtHashHandle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*current).hh_id.tbl).noexpand == 0
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
                    .wrapping_mul((*(*current).hh_id.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*current).hh_id.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                );
                (*(*current).hh_id.tbl).ideal_chain_maxlen = ((*(*current).hh_id.tbl).num_items
                    >> (*(*current).hh_id.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*current).hh_id.tbl).num_items
                        & (*(*current).hh_id.tbl)
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
                (*(*current).hh_id.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*current).hh_id.tbl).num_buckets {
                    _he_thh = (*(*(*current).hh_id.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UtHashHandle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*current).hh_id.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*current).hh_id.tbl).ideal_chain_maxlen {
                            (*(*current).hh_id.tbl).nonideal_items =
                                (*(*current).hh_id.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*current).hh_id.tbl).ideal_chain_maxlen);
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
                free((*(*current).hh_id.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*current).hh_id.tbl).num_buckets = (*(*current).hh_id.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*current).hh_id.tbl).log2_num_buckets =
                    (*(*current).hh_id.tbl).log2_num_buckets.wrapping_add(1);
                (*(*current).hh_id.tbl).buckets = _he_new_buckets;
                (*(*current).hh_id.tbl).ineff_expands = if (*(*current).hh_id.tbl).nonideal_items
                    > (*(*current).hh_id.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*current).hh_id.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*current).hh_id.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*current).hh_id.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        gid = (gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
        current = temp;
        temp = (if !temp.is_null() {
            (*temp).hh_name.next
        } else {
            NULL
        }) as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
    }
}
unsafe extern "C" fn escalate_glyph_order_by_name(
    mut go: *mut GlyphOrder,
    mut name: SdsRaw,
    mut order_type: GlyphOrderPass,
    mut order_entry: u32,
) {
    let mut s: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = name as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = sdslen(name) as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(sdslen(name) as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 16638419120543210315;
        }
        10 => {
            current_block_50 = 16638419120543210315;
        }
        9 => {
            current_block_50 = 552162828606560255;
        }
        8 => {
            current_block_50 = 2647375570052691271;
        }
        7 => {
            current_block_50 = 12476858771624613021;
        }
        6 => {
            current_block_50 = 13420836126193784560;
        }
        5 => {
            current_block_50 = 6204429805193992324;
        }
        4 => {
            current_block_50 = 6265671356496406540;
        }
        3 => {
            current_block_50 = 14904062521666713051;
        }
        2 => {
            current_block_50 = 4518118397577342293;
        }
        1 => {
            current_block_50 = 6707521803593403316;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        16638419120543210315 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 552162828606560255;
        }
        _ => {}
    }
    match current_block_50 {
        552162828606560255 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 2647375570052691271;
        }
        _ => {}
    }
    match current_block_50 {
        2647375570052691271 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 12476858771624613021;
        }
        _ => {}
    }
    match current_block_50 {
        12476858771624613021 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 13420836126193784560;
        }
        _ => {}
    }
    match current_block_50 {
        13420836126193784560 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 6204429805193992324;
        }
        _ => {}
    }
    match current_block_50 {
        6204429805193992324 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 6265671356496406540;
        }
        _ => {}
    }
    match current_block_50 {
        6265671356496406540 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 14904062521666713051;
        }
        _ => {}
    }
    match current_block_50 {
        14904062521666713051 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 4518118397577342293;
        }
        _ => {}
    }
    match current_block_50 {
        4518118397577342293 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 6707521803593403316;
        }
        _ => {}
    }
    match current_block_50 {
        6707521803593403316 => {
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
    s = ::core::ptr::null_mut::<GlyphOrderEntry>();
    if !(*go).by_name.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*go).by_name).hh_name.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*go).by_name).hh_name.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                s = ((*(*(*(*go).by_name).hh_name.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).by_name).hh_name.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut GlyphOrderEntry
                    as *mut GlyphOrderEntry;
            } else {
                s = ::core::ptr::null_mut::<GlyphOrderEntry>();
            }
            while !s.is_null() {
                if (*s).hh_name.hashv == _hf_hashv && (*s).hh_name.keylen as usize == sdslen(name) {
                    if memcmp(
                        (*s).hh_name.key,
                        name as *const ::core::ffi::c_void,
                        sdslen(name),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh_name.hh_next.is_null() {
                    s = ((*s).hh_name.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).by_name).hh_name.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut GlyphOrderEntry
                        as *mut GlyphOrderEntry;
                } else {
                    s = ::core::ptr::null_mut::<GlyphOrderEntry>();
                }
            }
        }
    }
    if !s.is_null() && (*s).order_type > order_type {
        (*s).order_type = order_type;
        (*s).order_entry = order_entry;
    }
}
unsafe extern "C" fn place_order_entries_from_glyf(
    mut table: *mut JsonValue,
    mut go: *mut GlyphOrder,
) {
    let mut j: u32 = 0 as u32;
    while j < (*table).u.object.length as u32 {
        let mut gname: SdsRaw = sdsnewlen(
            (*(*table).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*table).u.object.values.offset(j as isize)).name_length as usize,
        );
        if strcmp(
            gname as *const ::core::ffi::c_char,
            b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            set_order_by_name(
                go,
                gname,
                GlyphOrderPass::Notdef,
                0 as u32,
            );
        } else if strcmp(
            gname as *const ::core::ffi::c_char,
            b".null\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            set_order_by_name(
                go,
                gname,
                GlyphOrderPass::Notdef,
                1 as u32,
            );
        } else {
            set_order_by_name(go, gname, GlyphOrderPass::Glyf, j);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn place_order_entries_from_cmap(
    mut table: *mut JsonValue,
    mut go: *mut GlyphOrder,
) {
    let mut j: u32 = 0 as u32;
    while j < (*table).u.object.length as u32 {
        let mut unicode_str: SdsRaw = sdsnewlen(
            (*(*table).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*table).u.object.values.offset(j as isize)).name_length as usize,
        );
        let mut item: *mut JsonValue =
            (*(*table).u.object.values.offset(j as isize)).value as *mut JsonValue;
        let mut unicode: i32 = 0;
        if sdslen(unicode_str) > 2 as usize
            && *unicode_str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'U' as i32
            && *unicode_str.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '+' as i32
        {
            unicode = strtol(
                unicode_str.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                16 as ::core::ffi::c_int,
            ) as i32;
        } else {
            unicode = atoi(unicode_str as *const ::core::ffi::c_char) as i32;
        }
        sdsfree(unicode_str);
        if (*item).type_0 == JsonType::String
            && unicode > 0 as i32
            && unicode <= 0x10ffff as i32
        {
            let mut gname: SdsRaw = sdsnewlen(
                (*item).u.string.ptr as *const ::core::ffi::c_void,
                (*item).u.string.length as usize,
            );
            escalate_glyph_order_by_name(
                go,
                gname,
                GlyphOrderPass::Cmap,
                unicode as u32,
            );
            sdsfree(gname);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn place_order_entries_from_subtable(
    mut table: *mut JsonValue,
    mut go: *mut GlyphOrder,
    mut zero_only: bool,
) {
    let mut uplimit: u32 = (*table).u.array.length as u32;
    if uplimit >= 1 as u32 && zero_only as ::core::ffi::c_int != 0 {
        uplimit = 1 as u32;
    }
    let mut j: u32 = 0 as u32;
    while j < uplimit {
        let mut item: *mut JsonValue =
            *(*table).u.array.values.offset(j as isize) as *mut JsonValue;
        if (*item).type_0 == JsonType::String
        {
            let mut gname: SdsRaw = sdsnewlen(
                (*item).u.string.ptr as *const ::core::ffi::c_void,
                (*item).u.string.length as usize,
            );
            escalate_glyph_order_by_name(
                go,
                gname,
                GlyphOrderPass::GlyphOrder,
                j,
            );
            sdsfree(gname);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn parse_glyph_order(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut GlyphOrder {
    let mut go: *mut GlyphOrder = (
        OTFCC_PKG_GLYPH_ORDER
            .create
            .expect("non-null function pointer"))();
    if (*root).type_0 != JsonType::Object
    {
        return go;
    }
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        place_order_entries_from_glyf(table, go);
        table = json_obj_get_type(
            root,
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Object,
        );
        if !table.is_null() {
            place_order_entries_from_cmap(table, go);
        }
        table = json_obj_get_type(
            root,
            b"glyph_order\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        );
        if !table.is_null() {
            let mut ignore_glyph_order: bool = (*options).ignore_glyph_order;
            if ignore_glyph_order as ::core::ffi::c_int != 0
                && !json_obj_get_type(
                    root,
                    b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                )
                .is_null()
            {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_NOTICE,
                    LoggerType::Info,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"OpenType SVG table detected. Glyph order is preserved.",
                    ),
                );
                ignore_glyph_order = false;
            }
            place_order_entries_from_subtable(table, go, ignore_glyph_order);
        }
    }
    order_glyphs(go);
    return go;
}
struct JsonReader;
impl FontBuilder for JsonReader {
    unsafe fn read(
    mut _root: *mut ::core::ffi::c_void,
    mut _index: u32,
    options: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let options = options as *const Options;
    let mut root: *const JsonValue = _root as *mut JsonValue;
    let mut font: *mut Font = (
        OTFCC_I_FONT.create.expect("non-null function pointer"))();
    if font.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*font).subtype = otfcc_decide_font_subtype_from_json(root);
    (*font).glyph_order = parse_glyph_order(root, options);
    (*font).glyf = otfcc_parse_glyf(root, (*font).glyph_order, options);
    (*font).cff = otfcc_parse_cff(root, options);
    (*font).head = otfcc_parse_head(root, options);
    (*font).hhea = otfcc_parse_hhea(root, options);
    (*font).OS_2 = otfcc_parse_os_2(root, options);
    (*font).maxp = otfcc_parse_maxp(root, options);
    (*font).post = otfcc_parse_post(root, options);
    (*font).name = otfcc_parse_name(root, options);
    (*font).meta = otfcc_parse_meta(root, options);
    (*font).cmap = otfcc_parse_cmap(root, options);
    if !(*options).ignore_hints {
        (*font).fpgm = otfcc_parse_fpgm_prep(
            root,
            options,
            b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).prep = otfcc_parse_fpgm_prep(
            root,
            options,
            b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).cvt_ = otfcc_parse_cvt(
            root,
            options,
            b"cvt_\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).gasp = otfcc_parse_gasp(root, options);
    }
    (*font).vdmx = otfcc_parse_vdmx(root, options);
    (*font).vhea = otfcc_parse_vhea(root, options);
    if !(*font).glyf.is_null() {
        (*font).gsub = otfcc_parse_otl(
            root,
            options,
            b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).gpos = otfcc_parse_otl(
            root,
            options,
            b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).GDEF = otfcc_parse_gdef(root, options);
    }
    (*font).BASE = otfcc_parse_base(root, options);
    (*font).CPAL = otfcc_parse_cpal(root, options);
    (*font).COLR = otfcc_parse_colr(root, options);
    (*font).svg = otfcc_parse_svg(root, options);
    (*font).tsi_01 = otfcc_parse_tsi(
        root,
        options,
        b"TSI_01\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*font).tsi_23 = otfcc_parse_tsi(
        root,
        options,
        b"TSI_23\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*font).TSI5 = otfcc_parse_tsi5(root, options);
    return font as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn read_json(
    mut _root: *mut ::core::ffi::c_void,
    mut _index: u32,
    mut options: *const Options,
) -> *mut Font {
    <JsonReader as FontBuilder>::read(_root, _index, options as *const ::core::ffi::c_void)
        as *mut Font
}
#[inline]
unsafe extern "C" fn free_reader(mut self_0: *mut IFontBuilder) {
    free(self_0 as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_new_json_reader() -> *mut IFontBuilder {
    let mut reader: *mut IFontBuilder = ::core::ptr::null_mut::<IFontBuilder>();
    reader = __caryll_allocate_clean(
        ::core::mem::size_of::<IFontBuilder>() as usize,
        177 as ::core::ffi::c_ulong,
    ) as *mut IFontBuilder;
    (*reader).read = Some(
        read_json
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                u32,
                *const Options,
            ) -> *mut Font,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                u32,
                *const Options,
            ) -> *mut Font,
        >;
    (*reader).free = Some(free_reader as unsafe extern "C" fn(*mut IFontBuilder) -> ())
        as Option<unsafe extern "C" fn(*mut IFontBuilder) -> ()>;
    return reader;
}
