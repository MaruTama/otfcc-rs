#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset, strcmp, strtol};
unsafe extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static otfcc_iFont: __caryll_elementinterface_otfcc_Font;
    fn otfcc_parseHead(root: *const json_value, options: *const otfcc_Options) -> *mut table_head;
    fn otfcc_parseGlyf(
        root: *const json_value,
        glyph_order: *mut otfcc_GlyphOrder,
        options: *const otfcc_Options,
    ) -> *mut table_glyf;
    fn otfcc_parseCFF(root: *const json_value, options: *const otfcc_Options) -> *mut table_CFF;
    fn otfcc_parseMaxp(root: *const json_value, options: *const otfcc_Options) -> *mut table_maxp;
    fn otfcc_parseHhea(root: *const json_value, options: *const otfcc_Options) -> *mut table_hhea;
    fn otfcc_parseVhea(root: *const json_value, options: *const otfcc_Options) -> *mut table_vhea;
    fn otfcc_parseOS_2(root: *const json_value, options: *const otfcc_Options) -> *mut table_OS_2;
    fn otfcc_parsePost(root: *const json_value, options: *const otfcc_Options) -> *mut table_post;
    fn otfcc_parseName(root: *const json_value, options: *const otfcc_Options) -> *mut table_name;
    fn otfcc_parseMeta(root: *const json_value, options: *const otfcc_Options) -> *mut table_meta;
    fn otfcc_parseCmap(root: *const json_value, options: *const otfcc_Options) -> *mut table_cmap;
    fn otfcc_parseCvt(
        root: *const json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    ) -> *mut table_cvt;
    fn otfcc_parseFpgmPrep(
        root: *const json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    ) -> *mut table_fpgm_prep;
    fn otfcc_parseGasp(root: *const json_value, options: *const otfcc_Options) -> *mut table_gasp;
    fn otfcc_parseVDMX(root: *const json_value, options: *const otfcc_Options) -> *mut table_VDMX;
    fn otfcc_parseGDEF(root: *const json_value, options: *const otfcc_Options) -> *mut table_GDEF;
    fn otfcc_parseBASE(root: *const json_value, options: *const otfcc_Options) -> *mut table_BASE;
    fn otfcc_parseOtl(
        root: *const json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    ) -> *mut table_OTL;
    fn otfcc_parseCPAL(root: *const json_value, options: *const otfcc_Options) -> *mut table_CPAL;
    fn otfcc_parseCOLR(root: *const json_value, options: *const otfcc_Options) -> *mut table_COLR;
    fn otfcc_parseSVG(root: *const json_value, options: *const otfcc_Options) -> *mut table_SVG;
    fn otfcc_parseTSI(
        root: *const json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    ) -> *mut table_TSI;
    fn otfcc_parseTSI5(root: *const json_value, options: *const otfcc_Options) -> *mut table_TSI5;
}





use crate::support::json_funcs::{json_obj_get_type};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::otf_reader::FontBuilder;
use crate::logger::{log_type_info, log_vl_notice, otfcc_ILogger};

use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphid_t};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};
use crate::vendor::json::{json_array, json_object, json_string, json_value};
use crate::font::caryll_font::{FONTTYPE_CFF, FONTTYPE_TTF, __caryll_elementinterface_otfcc_Font, otfcc_Font, otfcc_IFontBuilder, otfcc_font_subtype};
use crate::support::{NULL};
use crate::support::glyph_order::{ORD_CMAP, ORD_GLYF, ORD_GLYPHORDER, ORD_NOTDEF, json_GlyphOrderPass, otfcc_GlyphOrder, otfcc_GlyphOrderEntry, otfcc_GlyphOrderPackage};
use crate::table::BASE::{table_BASE};
use crate::table::CFF::{table_CFF};
use crate::table::COLR::{table_COLR};
use crate::table::CPAL::{table_CPAL};
use crate::table::GDEF::{table_GDEF};

use crate::table::OS_2::{table_OS_2};
use crate::table::SVG::{table_SVG};
use crate::table::TSI5::{table_TSI5};

use crate::table::_TSI::{table_TSI};
use crate::table::cmap::{table_cmap};
use crate::table::cvt::{table_cvt};
use crate::table::fpgm_prep::{table_fpgm_prep};

use crate::table::gasp::{table_gasp};
use crate::table::glyf::{table_glyf};

use crate::table::head::{table_head};
use crate::table::hhea::{table_hhea};

use crate::table::maxp::{table_maxp};
use crate::table::meta::types::{table_meta};
use crate::table::name::{table_name};
use crate::table::otl::{table_OTL};
use crate::table::post::{table_post};
use crate::table::vdmx::types::{table_VDMX};
use crate::table::vhea::{table_vhea};

use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UT_hash_bucket, UT_hash_handle, UT_hash_table};




#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
unsafe extern "C" fn otfcc_decideFontSubtypeFromJson(
    mut root: *const json_value,
) -> otfcc_font_subtype {
    if !json_obj_get_type(
        root,
        b"CFF_\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    )
    .is_null()
    {
        return FONTTYPE_CFF;
    } else {
        return FONTTYPE_TTF;
    };
}
unsafe extern "C" fn setOrderByName(
    mut go: *mut otfcc_GlyphOrder,
    mut name: sds,
    mut orderType: json_GlyphOrderPass,
    mut orderEntry: u32,
) {
    let mut s: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
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
    s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
    if !(*go).byName.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*go).byName).hhName.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*go).byName).hhName.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                s = ((*(*(*(*go).byName).hhName.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).byName).hhName.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut otfcc_GlyphOrderEntry
                    as *mut otfcc_GlyphOrderEntry;
            } else {
                s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
            }
            while !s.is_null() {
                if (*s).hhName.hashv == _hf_hashv && (*s).hhName.keylen as usize == sdslen(name) {
                    if memcmp(
                        (*s).hhName.key,
                        name as *const ::core::ffi::c_void,
                        sdslen(name),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hhName.hh_next.is_null() {
                    s = ((*s).hhName.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byName).hhName.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut otfcc_GlyphOrderEntry
                        as *mut otfcc_GlyphOrderEntry;
                } else {
                    s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
                }
            }
        }
    }
    if s.is_null() {
        s = __caryll_allocate_clean(
            ::core::mem::size_of::<otfcc_GlyphOrderEntry>() as usize,
            21 as ::core::ffi::c_ulong,
        ) as *mut otfcc_GlyphOrderEntry;
        (*s).gid = -(1 as ::core::ffi::c_int) as glyphid_t;
        (*s).name = name;
        (*s).orderType = orderType;
        (*s).orderEntry = orderEntry;
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
        (*s).hhName.hashv = _ha_hashv;
        (*s).hhName.key = (*s).name.offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hhName.keylen = sdslen((*s).name) as ::core::ffi::c_uint;
        if (*go).byName.is_null() {
            (*s).hhName.next = NULL;
            (*s).hhName.prev = NULL;
            (*s).hhName.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*s).hhName.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*s).hhName.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as usize,
                );
                (*(*s).hhName.tbl).tail = &raw mut (*s).hhName as *mut UT_hash_handle;
                (*(*s).hhName.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*s).hhName.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*s).hhName.tbl).hho = (&raw mut (*s).hhName as *mut ::core::ffi::c_char)
                    .offset_from(s as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*s).hhName.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                (*(*s).hhName.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*s).hhName.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*s).hhName.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    );
                }
            }
            (*go).byName = s;
        } else {
            (*s).hhName.tbl = (*(*go).byName).hhName.tbl;
            (*s).hhName.next = NULL;
            (*s).hhName.prev = ((*(*(*go).byName).hhName.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*go).byName).hhName.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*go).byName).hhName.tbl).tail).next = s as *mut ::core::ffi::c_void;
            (*(*(*go).byName).hhName.tbl).tail = &raw mut (*s).hhName as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*go).byName).hhName.tbl).num_items =
            (*(*(*go).byName).hhName.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*go).byName).hhName.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = (*(*(*go).byName).hhName.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*s).hhName.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*s).hhName.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hhName as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*s).hhName as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*s).hhName.tbl).noexpand == 0
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
                    .wrapping_mul((*(*s).hhName.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*s).hhName.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                );
                (*(*s).hhName.tbl).ideal_chain_maxlen = ((*(*s).hhName.tbl).num_items
                    >> (*(*s).hhName.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*s).hhName.tbl).num_items
                        & (*(*s).hhName.tbl)
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
                (*(*s).hhName.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*s).hhName.tbl).num_buckets {
                    _he_thh = (*(*(*s).hhName.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*s).hhName.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*s).hhName.tbl).ideal_chain_maxlen {
                            (*(*s).hhName.tbl).nonideal_items =
                                (*(*s).hhName.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*s).hhName.tbl).ideal_chain_maxlen);
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
                free((*(*s).hhName.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*s).hhName.tbl).num_buckets = (*(*s).hhName.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*s).hhName.tbl).log2_num_buckets =
                    (*(*s).hhName.tbl).log2_num_buckets.wrapping_add(1);
                (*(*s).hhName.tbl).buckets = _he_new_buckets;
                (*(*s).hhName.tbl).ineff_expands = if (*(*s).hhName.tbl).nonideal_items
                    > (*(*s).hhName.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*s).hhName.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*s).hhName.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*s).hhName.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
    } else if (*s).orderType > orderType {
        (*s).orderType = orderType;
        (*s).orderEntry = orderEntry;
    }
}
unsafe extern "C" fn _byOrder(
    mut a: *mut otfcc_GlyphOrderEntry,
    mut b: *mut otfcc_GlyphOrderEntry,
) -> ::core::ffi::c_int {
    if (*a).orderType < (*b).orderType {
        return -(1 as ::core::ffi::c_int);
    }
    if (*a).orderType > (*b).orderType {
        return 1 as ::core::ffi::c_int;
    }
    if (*a).orderEntry < (*b).orderEntry {
        return -(1 as ::core::ffi::c_int);
    }
    if (*a).orderEntry > (*b).orderEntry {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn orderGlyphs(mut go: *mut otfcc_GlyphOrder) {
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
    if !(*go).byName.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*(*go).byName).hhName as *mut UT_hash_handle;
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
                            .offset((*(*(*go).byName).hhName.tbl).hho)
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
                                .offset((*(*(*go).byName).hhName.tbl).hho)
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
                                    .offset((*(*(*go).byName).hhName.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if _byOrder(
                        (_hs_p as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*go).byName).hhName.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut otfcc_GlyphOrderEntry,
                        (_hs_q as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*go).byName).hhName.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut otfcc_GlyphOrderEntry,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*(*go).byName).hhName.tbl).hho)
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
                                .offset((*(*(*go).byName).hhName.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*(*go).byName).hhName.tbl).hho)
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
                                .offset(-(*(*(*go).byName).hhName.tbl).hho)
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
                (*(*(*go).byName).hhName.tbl).tail = _hs_tail;
                (*go).byName = (_hs_list as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).byName).hhName.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut otfcc_GlyphOrderEntry
                    as *mut otfcc_GlyphOrderEntry;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut current: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
    let mut temp: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
    let mut gid: glyphid_t = 0 as glyphid_t;
    current = (*go).byName;
    temp = (if !(*go).byName.is_null() {
        (*(*go).byName).hhName.next
    } else {
        NULL
    }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
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
        _hj_k = ::core::mem::size_of::<glyphid_t>() as ::core::ffi::c_uint;
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
            _ha_hashv.wrapping_add(::core::mem::size_of::<glyphid_t>() as ::core::ffi::c_uint);
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
        (*current).hhID.hashv = _ha_hashv;
        (*current).hhID.key =
            &raw mut (*current).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*current).hhID.keylen = ::core::mem::size_of::<glyphid_t>() as ::core::ffi::c_uint;
        if (*go).byGID.is_null() {
            (*current).hhID.next = NULL;
            (*current).hhID.prev = NULL;
            (*current).hhID.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*current).hhID.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*current).hhID.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as usize,
                );
                (*(*current).hhID.tbl).tail = &raw mut (*current).hhID as *mut UT_hash_handle;
                (*(*current).hhID.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*current).hhID.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*current).hhID.tbl).hho = (&raw mut (*current).hhID as *mut ::core::ffi::c_char)
                    .offset_from(current as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long
                    as isize;
                (*(*current).hhID.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                (*(*current).hhID.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*current).hhID.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*current).hhID.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    );
                }
            }
            (*go).byGID = current;
        } else {
            (*current).hhID.tbl = (*(*go).byGID).hhID.tbl;
            (*current).hhID.next = NULL;
            (*current).hhID.prev = ((*(*(*go).byGID).hhID.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*go).byGID).hhID.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*go).byGID).hhID.tbl).tail).next = current as *mut ::core::ffi::c_void;
            (*(*(*go).byGID).hhID.tbl).tail = &raw mut (*current).hhID as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*go).byGID).hhID.tbl).num_items = (*(*(*go).byGID).hhID.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*go).byGID).hhID.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket =
            (*(*(*go).byGID).hhID.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*current).hhID.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*current).hhID.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*current).hhID as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*current).hhID as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*current).hhID.tbl).noexpand == 0
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
                    .wrapping_mul((*(*current).hhID.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*current).hhID.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                );
                (*(*current).hhID.tbl).ideal_chain_maxlen = ((*(*current).hhID.tbl).num_items
                    >> (*(*current).hhID.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*current).hhID.tbl).num_items
                        & (*(*current).hhID.tbl)
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
                (*(*current).hhID.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*current).hhID.tbl).num_buckets {
                    _he_thh = (*(*(*current).hhID.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*current).hhID.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*current).hhID.tbl).ideal_chain_maxlen {
                            (*(*current).hhID.tbl).nonideal_items =
                                (*(*current).hhID.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*current).hhID.tbl).ideal_chain_maxlen);
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
                free((*(*current).hhID.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*current).hhID.tbl).num_buckets = (*(*current).hhID.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*current).hhID.tbl).log2_num_buckets =
                    (*(*current).hhID.tbl).log2_num_buckets.wrapping_add(1);
                (*(*current).hhID.tbl).buckets = _he_new_buckets;
                (*(*current).hhID.tbl).ineff_expands = if (*(*current).hhID.tbl).nonideal_items
                    > (*(*current).hhID.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*current).hhID.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*current).hhID.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*current).hhID.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        gid = (gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
        current = temp;
        temp = (if !temp.is_null() {
            (*temp).hhName.next
        } else {
            NULL
        }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
    }
}
unsafe extern "C" fn escalateGlyphOrderByName(
    mut go: *mut otfcc_GlyphOrder,
    mut name: sds,
    mut orderType: json_GlyphOrderPass,
    mut orderEntry: u32,
) {
    let mut s: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
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
    s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
    if !(*go).byName.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*go).byName).hhName.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*go).byName).hhName.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                s = ((*(*(*(*go).byName).hhName.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).byName).hhName.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut otfcc_GlyphOrderEntry
                    as *mut otfcc_GlyphOrderEntry;
            } else {
                s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
            }
            while !s.is_null() {
                if (*s).hhName.hashv == _hf_hashv && (*s).hhName.keylen as usize == sdslen(name) {
                    if memcmp(
                        (*s).hhName.key,
                        name as *const ::core::ffi::c_void,
                        sdslen(name),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hhName.hh_next.is_null() {
                    s = ((*s).hhName.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byName).hhName.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut otfcc_GlyphOrderEntry
                        as *mut otfcc_GlyphOrderEntry;
                } else {
                    s = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
                }
            }
        }
    }
    if !s.is_null() && (*s).orderType > orderType {
        (*s).orderType = orderType;
        (*s).orderEntry = orderEntry;
    }
}
unsafe extern "C" fn placeOrderEntriesFromGlyf(
    mut table: *mut json_value,
    mut go: *mut otfcc_GlyphOrder,
) {
    let mut j: u32 = 0 as u32;
    while j < (*table).u.object.length as u32 {
        let mut gname: sds = sdsnewlen(
            (*(*table).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*table).u.object.values.offset(j as isize)).name_length as usize,
        );
        if strcmp(
            gname as *const ::core::ffi::c_char,
            b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            setOrderByName(
                go,
                gname,
                ORD_NOTDEF,
                0 as u32,
            );
        } else if strcmp(
            gname as *const ::core::ffi::c_char,
            b".null\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            setOrderByName(
                go,
                gname,
                ORD_NOTDEF,
                1 as u32,
            );
        } else {
            setOrderByName(go, gname, ORD_GLYF, j);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn placeOrderEntriesFromCmap(
    mut table: *mut json_value,
    mut go: *mut otfcc_GlyphOrder,
) {
    let mut j: u32 = 0 as u32;
    while j < (*table).u.object.length as u32 {
        let mut unicodeStr: sds = sdsnewlen(
            (*(*table).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*table).u.object.values.offset(j as isize)).name_length as usize,
        );
        let mut item: *mut json_value =
            (*(*table).u.object.values.offset(j as isize)).value as *mut json_value;
        let mut unicode: i32 = 0;
        if sdslen(unicodeStr) > 2 as usize
            && *unicodeStr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'U' as i32
            && *unicodeStr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '+' as i32
        {
            unicode = strtol(
                unicodeStr.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                16 as ::core::ffi::c_int,
            ) as i32;
        } else {
            unicode = atoi(unicodeStr as *const ::core::ffi::c_char) as i32;
        }
        sdsfree(unicodeStr);
        if (*item).type_0 == json_string
            && unicode > 0 as i32
            && unicode <= 0x10ffff as i32
        {
            let mut gname: sds = sdsnewlen(
                (*item).u.string.ptr as *const ::core::ffi::c_void,
                (*item).u.string.length as usize,
            );
            escalateGlyphOrderByName(
                go,
                gname,
                ORD_CMAP,
                unicode as u32,
            );
            sdsfree(gname);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn placeOrderEntriesFromSubtable(
    mut table: *mut json_value,
    mut go: *mut otfcc_GlyphOrder,
    mut zeroOnly: bool,
) {
    let mut uplimit: u32 = (*table).u.array.length as u32;
    if uplimit >= 1 as u32 && zeroOnly as ::core::ffi::c_int != 0 {
        uplimit = 1 as u32;
    }
    let mut j: u32 = 0 as u32;
    while j < uplimit {
        let mut item: *mut json_value =
            *(*table).u.array.values.offset(j as isize) as *mut json_value;
        if (*item).type_0 == json_string
        {
            let mut gname: sds = sdsnewlen(
                (*item).u.string.ptr as *const ::core::ffi::c_void,
                (*item).u.string.length as usize,
            );
            escalateGlyphOrderByName(
                go,
                gname,
                ORD_GLYPHORDER,
                j,
            );
            sdsfree(gname);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn parseGlyphOrder(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut otfcc_GlyphOrder {
    let mut go: *mut otfcc_GlyphOrder = (
        otfcc_pkgGlyphOrder
            .create
            .expect("non-null function pointer"))();
    if (*root).type_0 != json_object
    {
        return go;
    }
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        placeOrderEntriesFromGlyf(table, go);
        table = json_obj_get_type(
            root,
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
            json_object,
        );
        if !table.is_null() {
            placeOrderEntriesFromCmap(table, go);
        }
        table = json_obj_get_type(
            root,
            b"glyph_order\0" as *const u8 as *const ::core::ffi::c_char,
            json_array,
        );
        if !table.is_null() {
            let mut ignoreGlyphOrder: bool = (*options).ignore_glyph_order;
            if ignoreGlyphOrder as ::core::ffi::c_int != 0
                && !json_obj_get_type(
                    root,
                    b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
                    json_array,
                )
                .is_null()
            {
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut otfcc_ILogger,
                    log_vl_notice,
                    log_type_info,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"OpenType SVG table detected. Glyph order is preserved.",
                    ),
                );
                ignoreGlyphOrder = false;
            }
            placeOrderEntriesFromSubtable(table, go, ignoreGlyphOrder);
        }
    }
    orderGlyphs(go);
    return go;
}
struct JsonReader;
impl FontBuilder for JsonReader {
    unsafe fn read(
    mut _root: *mut ::core::ffi::c_void,
    mut _index: u32,
    options: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let options = options as *const otfcc_Options;
    let mut root: *const json_value = _root as *mut json_value;
    let mut font: *mut otfcc_Font = (
        otfcc_iFont.create.expect("non-null function pointer"))();
    if font.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*font).subtype = otfcc_decideFontSubtypeFromJson(root);
    (*font).glyph_order = parseGlyphOrder(root, options);
    (*font).glyf = otfcc_parseGlyf(root, (*font).glyph_order, options);
    (*font).CFF_ = otfcc_parseCFF(root, options);
    (*font).head = otfcc_parseHead(root, options);
    (*font).hhea = otfcc_parseHhea(root, options);
    (*font).OS_2 = otfcc_parseOS_2(root, options);
    (*font).maxp = otfcc_parseMaxp(root, options);
    (*font).post = otfcc_parsePost(root, options);
    (*font).name = otfcc_parseName(root, options);
    (*font).meta = otfcc_parseMeta(root, options);
    (*font).cmap = otfcc_parseCmap(root, options);
    if !(*options).ignore_hints {
        (*font).fpgm = otfcc_parseFpgmPrep(
            root,
            options,
            b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).prep = otfcc_parseFpgmPrep(
            root,
            options,
            b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).cvt_ = otfcc_parseCvt(
            root,
            options,
            b"cvt_\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).gasp = otfcc_parseGasp(root, options);
    }
    (*font).VDMX = otfcc_parseVDMX(root, options);
    (*font).vhea = otfcc_parseVhea(root, options);
    if !(*font).glyf.is_null() {
        (*font).GSUB = otfcc_parseOtl(
            root,
            options,
            b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).GPOS = otfcc_parseOtl(
            root,
            options,
            b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).GDEF = otfcc_parseGDEF(root, options);
    }
    (*font).BASE = otfcc_parseBASE(root, options);
    (*font).CPAL = otfcc_parseCPAL(root, options);
    (*font).COLR = otfcc_parseCOLR(root, options);
    (*font).SVG_ = otfcc_parseSVG(root, options);
    (*font).TSI_01 = otfcc_parseTSI(
        root,
        options,
        b"TSI_01\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*font).TSI_23 = otfcc_parseTSI(
        root,
        options,
        b"TSI_23\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*font).TSI5 = otfcc_parseTSI5(root, options);
    return font as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn readJson(
    mut _root: *mut ::core::ffi::c_void,
    mut _index: u32,
    mut options: *const otfcc_Options,
) -> *mut otfcc_Font {
    <JsonReader as FontBuilder>::read(_root, _index, options as *const ::core::ffi::c_void)
        as *mut otfcc_Font
}
#[inline]
unsafe extern "C" fn freeReader(mut self_0: *mut otfcc_IFontBuilder) {
    free(self_0 as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_newJsonReader() -> *mut otfcc_IFontBuilder {
    let mut reader: *mut otfcc_IFontBuilder = ::core::ptr::null_mut::<otfcc_IFontBuilder>();
    reader = __caryll_allocate_clean(
        ::core::mem::size_of::<otfcc_IFontBuilder>() as usize,
        177 as ::core::ffi::c_ulong,
    ) as *mut otfcc_IFontBuilder;
    (*reader).read = Some(
        readJson
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                u32,
                *const otfcc_Options,
            ) -> *mut otfcc_Font,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                u32,
                *const otfcc_Options,
            ) -> *mut otfcc_Font,
        >;
    (*reader).free = Some(freeReader as unsafe extern "C" fn(*mut otfcc_IFontBuilder) -> ())
        as Option<unsafe extern "C" fn(*mut otfcc_IFontBuilder) -> ()>;
    return reader;
}
