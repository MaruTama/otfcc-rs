#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memcpy, memset};

use crate::support::handle::{handle_consolidate_to, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::support::{NULL};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::vendor::sds::{sdsempty, sdsfree};
/// Which pass of a JSON font's glyph naming placed a glyph, and therefore how
/// strongly it is placed: the *lowest* pass wins, because `set_order_by_name`
/// escalates an entry only when the new pass ranks below the one on record and
/// `_by_order` sorts ascending. That makes the ordering the meaning, so `Ord` is
/// derived -- and since it compares by *declaration* order, the variants are
/// declared in ascending discriminant order and
/// `glyphorderpass_order_is_its_encoding` pins that the two agree.
///
/// `GlyphOrderPass::Unset` is a name this port adds; C had none. Its `enum` lives inside
/// `json-reader.c` while this struct's field is a plain `uint8_t` in the shared
/// header, so the OTF path could leave the field at whatever `calloc` gave it --
/// and it does: `otfcc_set_glyph_order_by_gid` and `otfcc_set_glyph_order_by_name`
/// allocate an entry and set only `gid` and `name`. An enum without a zero
/// variant would make both of them UB. The state is meaningful, not padding:
/// zero outranks every named pass, so an entry placed by GID can never be
/// escalated by one.
///
/// The type lives here rather than in `json_reader` -- where C keeps it, and
/// where the values are still produced -- because this is the field it types.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum GlyphOrderPass {
    Unset = 0,
    GlyphOrder = 1,
    Notdef = 2,
    Cmap = 3,
    Glyf = 4,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphOrderEntry {
    pub gid: GlyphId,
    pub name: SdsRaw,
    pub orderType: GlyphOrderPass,
    pub orderEntry: u32,
    pub hhID: UtHashHandle,
    pub hhName: UtHashHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphOrder {
    pub byGID: *mut GlyphOrderEntry,
    pub byName: *mut GlyphOrderEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphOrderPackage {
    pub init: Option<unsafe extern "C" fn(*mut GlyphOrder) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut GlyphOrder, *const GlyphOrder) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut GlyphOrder, *mut GlyphOrder) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GlyphOrder) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut GlyphOrder, GlyphOrder) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut GlyphOrder, GlyphOrder) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GlyphOrder>,
    pub free: Option<unsafe extern "C" fn(*mut GlyphOrder) -> ()>,
    pub setByGID: Option<unsafe extern "C" fn(*mut GlyphOrder, GlyphId, SdsRaw) -> SdsRaw>,
    pub setByName: Option<unsafe extern "C" fn(*mut GlyphOrder, SdsRaw, GlyphId) -> bool>,
    pub nameAField_Shared:
        Option<unsafe extern "C" fn(*mut GlyphOrder, GlyphId, *mut SdsRaw) -> bool>,
    pub consolidateHandle:
        Option<unsafe extern "C" fn(*mut GlyphOrder, *mut GlyphHandle) -> bool>,
    pub lookupName: Option<unsafe extern "C" fn(*mut GlyphOrder, SdsRaw) -> bool>,
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
#[inline]
unsafe extern "C" fn init_glyph_order(mut go: *mut GlyphOrder) {
    (*go).byGID = ::core::ptr::null_mut::<GlyphOrderEntry>();
    (*go).byName = ::core::ptr::null_mut::<GlyphOrderEntry>();
}
#[inline]
unsafe extern "C" fn dispose_glyph_order(mut go: *mut GlyphOrder) {
    let mut current: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
    let mut temp: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
    current = (*go).byGID;
    temp = (if !(*go).byGID.is_null() {
        (*(*go).byGID).hhID.next
    } else {
        NULL
    }) as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
    while !current.is_null() {
        if !(*current).name.is_null() {
            sdsfree((*current).name);
        }
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*current).hhID;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*go).byGID).hhID.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*go).byGID).hhID.tbl as *mut ::core::ffi::c_void);
            (*go).byGID = ::core::ptr::null_mut::<GlyphOrderEntry>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*go).byGID).hhID.tbl).tail {
                (*(*(*go).byGID).hhID.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*go).byGID).hhID.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*go).byGID).hhID.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                (*go).byGID =
                    (*_hd_hh_del).next as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*go).byGID).hhID.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*go).byGID).hhID.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket =
                (*(*(*go).byGID).hhID.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
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
            (*(*(*go).byGID).hhID.tbl).num_items =
                (*(*(*go).byGID).hhID.tbl).num_items.wrapping_sub(1);
        }
        let mut _hd_hh_del_0: *mut UtHashHandle = &raw mut (*current).hhName;
        if (*_hd_hh_del_0).prev.is_null() && (*_hd_hh_del_0).next.is_null() {
            free((*(*(*go).byName).hhName.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*go).byName).hhName.tbl as *mut ::core::ffi::c_void);
            (*go).byName = ::core::ptr::null_mut::<GlyphOrderEntry>();
        } else {
            let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
            if _hd_hh_del_0 == (*(*(*go).byName).hhName.tbl).tail {
                (*(*(*go).byName).hhName.tbl).tail =
                    ((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*go).byName).hhName.tbl).hho)
                        as *mut UtHashHandle as *mut UtHashHandle;
            }
            if !(*_hd_hh_del_0).prev.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*go).byName).hhName.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh2 = (*_hd_hh_del_0).next;
            } else {
                (*go).byName = (*_hd_hh_del_0).next as *mut GlyphOrderEntry
                    as *mut GlyphOrderEntry;
            }
            if !(*_hd_hh_del_0).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del_0).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*go).byName).hhName.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh3 = (*_hd_hh_del_0).prev;
            }
            _hd_bkt_0 = (*_hd_hh_del_0).hashv
                & (*(*(*go).byName).hhName.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head_0: *mut UtHashBucket = (*(*(*go).byName).hhName.tbl)
                .buckets
                .offset(_hd_bkt_0 as isize)
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
            (*(*(*go).byName).hhName.tbl).num_items =
                (*(*(*go).byName).hhName.tbl).num_items.wrapping_sub(1);
        }
        free(current as *mut ::core::ffi::c_void);
        current = ::core::ptr::null_mut::<GlyphOrderEntry>();
        current = temp;
        temp = (if !temp.is_null() {
            (*temp).hhID.next
        } else {
            NULL
        }) as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
    }
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_init(mut x: *mut GlyphOrder) {
    init_glyph_order(x);
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_dispose(mut x: *mut GlyphOrder) {
    dispose_glyph_order(x);
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_free(mut x: *mut GlyphOrder) {
    if x.is_null() {
        return;
    }
    otfcc_glyph_order_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_create() -> *mut GlyphOrder {
    let mut x: *mut GlyphOrder =
        malloc(::core::mem::size_of::<GlyphOrder>() as usize) as *mut GlyphOrder;
    otfcc_glyph_order_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_replace(
    mut dst: *mut GlyphOrder,
    src: GlyphOrder,
) {
    otfcc_glyph_order_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GlyphOrder>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_move(
    mut dst: *mut GlyphOrder,
    mut src: *mut GlyphOrder,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GlyphOrder>() as usize,
    );
    otfcc_glyph_order_init(src);
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_copy(
    mut dst: *mut GlyphOrder,
    mut src: *const GlyphOrder,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GlyphOrder>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_copy_replace(
    mut dst: *mut GlyphOrder,
    src: GlyphOrder,
) {
    otfcc_glyph_order_dispose(dst);
    otfcc_glyph_order_copy(dst, &raw const src);
}
unsafe extern "C" fn otfcc_set_glyph_order_by_gid(
    mut go: *mut GlyphOrder,
    mut gid: GlyphId,
    mut name: SdsRaw,
) -> SdsRaw {
    let mut s: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut gid as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 15927764250131007534;
        }
        10 => {
            current_block_50 = 15927764250131007534;
        }
        9 => {
            current_block_50 = 16307342884694566710;
        }
        8 => {
            current_block_50 = 10727797739500725466;
        }
        7 => {
            current_block_50 = 4322921244590678397;
        }
        6 => {
            current_block_50 = 17999004377527793496;
        }
        5 => {
            current_block_50 = 11288263663399842285;
        }
        4 => {
            current_block_50 = 9363548342483475916;
        }
        3 => {
            current_block_50 = 18157531726959496515;
        }
        2 => {
            current_block_50 = 2346718609469148821;
        }
        1 => {
            current_block_50 = 7133810331402286908;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        15927764250131007534 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 16307342884694566710;
        }
        _ => {}
    }
    match current_block_50 {
        16307342884694566710 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 10727797739500725466;
        }
        _ => {}
    }
    match current_block_50 {
        10727797739500725466 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 4322921244590678397;
        }
        _ => {}
    }
    match current_block_50 {
        4322921244590678397 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 17999004377527793496;
        }
        _ => {}
    }
    match current_block_50 {
        17999004377527793496 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 11288263663399842285;
        }
        _ => {}
    }
    match current_block_50 {
        11288263663399842285 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 9363548342483475916;
        }
        _ => {}
    }
    match current_block_50 {
        9363548342483475916 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 18157531726959496515;
        }
        _ => {}
    }
    match current_block_50 {
        18157531726959496515 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 2346718609469148821;
        }
        _ => {}
    }
    match current_block_50 {
        2346718609469148821 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 7133810331402286908;
        }
        _ => {}
    }
    match current_block_50 {
        7133810331402286908 => {
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
    if !(*go).byGID.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*go).byGID).hhID.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*go).byGID).hhID.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                s = ((*(*(*(*go).byGID).hhID.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).byGID).hhID.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut GlyphOrderEntry
                    as *mut GlyphOrderEntry;
            } else {
                s = ::core::ptr::null_mut::<GlyphOrderEntry>();
            }
            while !s.is_null() {
                if (*s).hhID.hashv == _hf_hashv
                    && (*s).hhID.keylen as usize == ::core::mem::size_of::<GlyphId>()
                {
                    if memcmp(
                        (*s).hhID.key,
                        &raw mut gid as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<GlyphId>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hhID.hh_next.is_null() {
                    s = ((*s).hhID.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byGID).hhID.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut GlyphOrderEntry
                        as *mut GlyphOrderEntry;
                } else {
                    s = ::core::ptr::null_mut::<GlyphOrderEntry>();
                }
            }
        }
    }
    if !s.is_null() {
        sdsfree(name);
        return (*s).name;
    } else {
        let mut t: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
        let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar = name as *const ::core::ffi::c_uchar;
        _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = sdslen(name) as ::core::ffi::c_uint;
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
            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
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
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
            _hj_i_0 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
            _hf_hashv_0 ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
            _hj_i_0 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
            _hf_hashv_0 ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
            _hj_i_0 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
            _hf_hashv_0 ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _hf_hashv_0 = _hf_hashv_0.wrapping_add(sdslen(name) as ::core::ffi::c_uint);
        let mut current_block_167: u64;
        match _hj_k_0 {
            11 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 818722601840998770;
            }
            10 => {
                current_block_167 = 818722601840998770;
            }
            9 => {
                current_block_167 = 5926363569472234810;
            }
            8 => {
                current_block_167 = 1609275879724060114;
            }
            7 => {
                current_block_167 = 5525415357377260787;
            }
            6 => {
                current_block_167 = 8541211641755629352;
            }
            5 => {
                current_block_167 = 11294912796665367681;
            }
            4 => {
                current_block_167 = 16709217550526598015;
            }
            3 => {
                current_block_167 = 5949410589853724019;
            }
            2 => {
                current_block_167 = 5199815811827179141;
            }
            1 => {
                current_block_167 = 13162064672842250161;
            }
            _ => {
                current_block_167 = 7761909515536616543;
            }
        }
        match current_block_167 {
            818722601840998770 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 5926363569472234810;
            }
            _ => {}
        }
        match current_block_167 {
            5926363569472234810 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 1609275879724060114;
            }
            _ => {}
        }
        match current_block_167 {
            1609275879724060114 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 5525415357377260787;
            }
            _ => {}
        }
        match current_block_167 {
            5525415357377260787 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 8541211641755629352;
            }
            _ => {}
        }
        match current_block_167 {
            8541211641755629352 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 11294912796665367681;
            }
            _ => {}
        }
        match current_block_167 {
            11294912796665367681 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_167 = 16709217550526598015;
            }
            _ => {}
        }
        match current_block_167 {
            16709217550526598015 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 5949410589853724019;
            }
            _ => {}
        }
        match current_block_167 {
            5949410589853724019 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 5199815811827179141;
            }
            _ => {}
        }
        match current_block_167 {
            5199815811827179141 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 13162064672842250161;
            }
            _ => {}
        }
        match current_block_167 {
            13162064672842250161 => {
                _hj_i_0 =
                    _hj_i_0
                        .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
        _hj_i_0 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
        _hf_hashv_0 ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
        _hj_i_0 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
        _hf_hashv_0 ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
        _hj_i_0 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
        _hf_hashv_0 ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
        t = ::core::ptr::null_mut::<GlyphOrderEntry>();
        if !(*go).byName.is_null() {
            let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
            _hf_bkt_0 = _hf_hashv_0
                & (*(*(*go).byName).hhName.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if !(*(*(*(*go).byName).hhName.tbl)
                    .buckets
                    .offset(_hf_bkt_0 as isize))
                .hh_head
                .is_null()
                {
                    t = ((*(*(*(*go).byName).hhName.tbl)
                        .buckets
                        .offset(_hf_bkt_0 as isize))
                    .hh_head as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byName).hhName.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut GlyphOrderEntry
                        as *mut GlyphOrderEntry;
                } else {
                    t = ::core::ptr::null_mut::<GlyphOrderEntry>();
                }
                while !t.is_null() {
                    if (*t).hhName.hashv == _hf_hashv_0
                        && (*t).hhName.keylen as usize == sdslen(name)
                    {
                        if memcmp(
                            (*t).hhName.key,
                            name as *const ::core::ffi::c_void,
                            sdslen(name),
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*t).hhName.hh_next.is_null() {
                        t = ((*t).hhName.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*go).byName).hhName.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut GlyphOrderEntry
                            as *mut GlyphOrderEntry;
                    } else {
                        t = ::core::ptr::null_mut::<GlyphOrderEntry>();
                    }
                }
            }
        }
        if !t.is_null() {
            sdsfree(name);
            name = crate::sdsbuild!(sdsempty(), b"$$gid", gid as ::core::ffi::c_int);
        }
        s = __caryll_allocate_clean(
            ::core::mem::size_of::<GlyphOrderEntry>() as usize,
            36 as ::core::ffi::c_ulong,
        ) as *mut GlyphOrderEntry;
        (*s).gid = gid;
        (*s).name = name;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_1: ::core::ffi::c_uint = 0;
        let mut _hj_j_1: ::core::ffi::c_uint = 0;
        let mut _hj_k_1: ::core::ffi::c_uint = 0;
        let mut _hj_key_1: *const ::core::ffi::c_uchar =
            &raw mut (*s).gid as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_1 = _hj_j_1;
        _hj_k_1 = ::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint;
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
            _ha_hashv = _ha_hashv.wrapping_add(
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
            _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
            _hj_i_1 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
            _ha_hashv ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
            _hj_i_1 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
            _ha_hashv ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
            _hj_i_1 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
            _ha_hashv ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
            _hj_key_1 = _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv =
            _ha_hashv.wrapping_add(::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint);
        let mut current_block_288: u64;
        match _hj_k_1 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_288 = 10513301211021682562;
            }
            10 => {
                current_block_288 = 10513301211021682562;
            }
            9 => {
                current_block_288 = 7143536131258716470;
            }
            8 => {
                current_block_288 = 11235014462272329396;
            }
            7 => {
                current_block_288 = 9993108071385331198;
            }
            6 => {
                current_block_288 = 13673273054728200166;
            }
            5 => {
                current_block_288 = 17891366550729284889;
            }
            4 => {
                current_block_288 = 9895904232009999838;
            }
            3 => {
                current_block_288 = 16729612675251136371;
            }
            2 => {
                current_block_288 = 5474081111068890449;
            }
            1 => {
                current_block_288 = 13874584177866077742;
            }
            _ => {
                current_block_288 = 13733842304117558426;
            }
        }
        match current_block_288 {
            10513301211021682562 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_288 = 7143536131258716470;
            }
            _ => {}
        }
        match current_block_288 {
            7143536131258716470 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_288 = 11235014462272329396;
            }
            _ => {}
        }
        match current_block_288 {
            11235014462272329396 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_288 = 9993108071385331198;
            }
            _ => {}
        }
        match current_block_288 {
            9993108071385331198 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_288 = 13673273054728200166;
            }
            _ => {}
        }
        match current_block_288 {
            13673273054728200166 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_288 = 17891366550729284889;
            }
            _ => {}
        }
        match current_block_288 {
            17891366550729284889 => {
                _hj_j_1 =
                    _hj_j_1
                        .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_288 = 9895904232009999838;
            }
            _ => {}
        }
        match current_block_288 {
            9895904232009999838 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_288 = 16729612675251136371;
            }
            _ => {}
        }
        match current_block_288 {
            16729612675251136371 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_288 = 5474081111068890449;
            }
            _ => {}
        }
        match current_block_288 {
            5474081111068890449 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_288 = 13874584177866077742;
            }
            _ => {}
        }
        match current_block_288 {
            13874584177866077742 => {
                _hj_i_1 =
                    _hj_i_1
                        .wrapping_add(*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
        _hj_i_1 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
        _ha_hashv ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
        _hj_i_1 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
        _ha_hashv ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv);
        _hj_i_1 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_1);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_1);
        _ha_hashv ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
        (*s).hhID.hashv = _ha_hashv;
        (*s).hhID.key = &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hhID.keylen = ::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint;
        if (*go).byGID.is_null() {
            (*s).hhID.next = NULL;
            (*s).hhID.prev = NULL;
            (*s).hhID.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*s).hhID.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*s).hhID.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*s).hhID.tbl).tail = &raw mut (*s).hhID as *mut UtHashHandle;
                (*(*s).hhID.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*s).hhID.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*s).hhID.tbl).hho = (&raw mut (*s).hhID as *mut ::core::ffi::c_char)
                    .offset_from(s as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*s).hhID.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*s).hhID.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*s).hhID.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*s).hhID.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    );
                }
            }
            (*go).byGID = s;
        } else {
            (*s).hhID.tbl = (*(*go).byGID).hhID.tbl;
            (*s).hhID.next = NULL;
            (*s).hhID.prev = ((*(*(*go).byGID).hhID.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*go).byGID).hhID.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*go).byGID).hhID.tbl).tail).next = s as *mut ::core::ffi::c_void;
            (*(*(*go).byGID).hhID.tbl).tail = &raw mut (*s).hhID as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*go).byGID).hhID.tbl).num_items = (*(*(*go).byGID).hhID.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*go).byGID).hhID.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket =
            (*(*(*go).byGID).hhID.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*s).hhID.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
        (*s).hhID.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hhID as *mut UtHashHandle;
        }
        (*_ha_head).hh_head = &raw mut (*s).hhID as *mut UtHashHandle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*s).hhID.tbl).noexpand == 0
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
                    .wrapping_mul((*(*s).hhID.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*s).hhID.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                );
                (*(*s).hhID.tbl).ideal_chain_maxlen = ((*(*s).hhID.tbl).num_items
                    >> (*(*s).hhID.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*s).hhID.tbl).num_items
                        & (*(*s).hhID.tbl)
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
                (*(*s).hhID.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*s).hhID.tbl).num_buckets {
                    _he_thh = (*(*(*s).hhID.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UtHashHandle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*s).hhID.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*s).hhID.tbl).ideal_chain_maxlen {
                            (*(*s).hhID.tbl).nonideal_items =
                                (*(*s).hhID.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*s).hhID.tbl).ideal_chain_maxlen);
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
                free((*(*s).hhID.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*s).hhID.tbl).num_buckets = (*(*s).hhID.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*s).hhID.tbl).log2_num_buckets =
                    (*(*s).hhID.tbl).log2_num_buckets.wrapping_add(1);
                (*(*s).hhID.tbl).buckets = _he_new_buckets;
                (*(*s).hhID.tbl).ineff_expands = if (*(*s).hhID.tbl).nonideal_items
                    > (*(*s).hhID.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*s).hhID.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*s).hhID.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*s).hhID.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        let mut _ha_hashv_0: ::core::ffi::c_uint = 0;
        let mut _hj_i_2: ::core::ffi::c_uint = 0;
        let mut _hj_j_2: ::core::ffi::c_uint = 0;
        let mut _hj_k_2: ::core::ffi::c_uint = 0;
        let mut _hj_key_2: *const ::core::ffi::c_uchar =
            (*s).name.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_uchar;
        _ha_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_2 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_2 = _hj_j_2;
        _hj_k_2 = sdslen((*s).name) as ::core::ffi::c_uint;
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
        _ha_hashv_0 = _ha_hashv_0.wrapping_add(sdslen((*s).name) as ::core::ffi::c_uint);
        let mut current_block_476: u64;
        match _hj_k_2 {
            11 => {
                _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                    (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_476 = 18070827520340661055;
            }
            10 => {
                current_block_476 = 18070827520340661055;
            }
            9 => {
                current_block_476 = 10758446703012050218;
            }
            8 => {
                current_block_476 = 8202393673878014038;
            }
            7 => {
                current_block_476 = 17490606794343611540;
            }
            6 => {
                current_block_476 = 13369259558370356807;
            }
            5 => {
                current_block_476 = 2016610040826759079;
            }
            4 => {
                current_block_476 = 8067717456239466500;
            }
            3 => {
                current_block_476 = 9962287190801049050;
            }
            2 => {
                current_block_476 = 17434639650987103222;
            }
            1 => {
                current_block_476 = 16997448961873278270;
            }
            _ => {
                current_block_476 = 1771714053403391377;
            }
        }
        match current_block_476 {
            18070827520340661055 => {
                _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                    (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_476 = 10758446703012050218;
            }
            _ => {}
        }
        match current_block_476 {
            10758446703012050218 => {
                _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                    (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_476 = 8202393673878014038;
            }
            _ => {}
        }
        match current_block_476 {
            8202393673878014038 => {
                _hj_j_2 = _hj_j_2.wrapping_add(
                    (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_476 = 17490606794343611540;
            }
            _ => {}
        }
        match current_block_476 {
            17490606794343611540 => {
                _hj_j_2 = _hj_j_2.wrapping_add(
                    (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_476 = 13369259558370356807;
            }
            _ => {}
        }
        match current_block_476 {
            13369259558370356807 => {
                _hj_j_2 = _hj_j_2.wrapping_add(
                    (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_476 = 2016610040826759079;
            }
            _ => {}
        }
        match current_block_476 {
            2016610040826759079 => {
                _hj_j_2 =
                    _hj_j_2
                        .wrapping_add(*_hj_key_2.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_476 = 8067717456239466500;
            }
            _ => {}
        }
        match current_block_476 {
            8067717456239466500 => {
                _hj_i_2 = _hj_i_2.wrapping_add(
                    (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_476 = 9962287190801049050;
            }
            _ => {}
        }
        match current_block_476 {
            9962287190801049050 => {
                _hj_i_2 = _hj_i_2.wrapping_add(
                    (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_476 = 17434639650987103222;
            }
            _ => {}
        }
        match current_block_476 {
            17434639650987103222 => {
                _hj_i_2 = _hj_i_2.wrapping_add(
                    (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_476 = 16997448961873278270;
            }
            _ => {}
        }
        match current_block_476 {
            16997448961873278270 => {
                _hj_i_2 =
                    _hj_i_2
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
        (*s).hhName.hashv = _ha_hashv_0;
        (*s).hhName.key = (*s).name.offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hhName.keylen = sdslen((*s).name) as ::core::ffi::c_uint;
        if (*go).byName.is_null() {
            (*s).hhName.next = NULL;
            (*s).hhName.prev = NULL;
            (*s).hhName.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*s).hhName.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*s).hhName.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*s).hhName.tbl).tail = &raw mut (*s).hhName as *mut UtHashHandle;
                (*(*s).hhName.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*s).hhName.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*s).hhName.tbl).hho = (&raw mut (*s).hhName as *mut ::core::ffi::c_char)
                    .offset_from(s as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*s).hhName.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*s).hhName.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*s).hhName.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*s).hhName.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
            (*(*(*go).byName).hhName.tbl).tail = &raw mut (*s).hhName as *mut UtHashHandle;
        }
        let mut _ha_bkt_0: ::core::ffi::c_uint = 0;
        (*(*(*go).byName).hhName.tbl).num_items =
            (*(*(*go).byName).hhName.tbl).num_items.wrapping_add(1);
        _ha_bkt_0 = _ha_hashv_0
            & (*(*(*go).byName).hhName.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head_0: *mut UtHashBucket = (*(*(*go).byName).hhName.tbl)
            .buckets
            .offset(_ha_bkt_0 as isize)
            as *mut UtHashBucket;
        (*_ha_head_0).count = (*_ha_head_0).count.wrapping_add(1);
        (*s).hhName.hh_next = (*_ha_head_0).hh_head as *mut UtHashHandle;
        (*s).hhName.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head_0).hh_head.is_null() {
            (*(*_ha_head_0).hh_head).hh_prev = &raw mut (*s).hhName as *mut UtHashHandle;
        }
        (*_ha_head_0).hh_head = &raw mut (*s).hhName as *mut UtHashHandle;
        if (*_ha_head_0).count
            >= (*_ha_head_0)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*s).hhName.tbl).noexpand == 0
        {
            let mut _he_bkt_0: ::core::ffi::c_uint = 0;
            let mut _he_bkt_i_0: ::core::ffi::c_uint = 0;
            let mut _he_thh_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _he_hh_nxt_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _he_new_buckets_0: *mut UtHashBucket =
                ::core::ptr::null_mut::<UtHashBucket>();
            let mut _he_newbkt_0: *mut UtHashBucket = ::core::ptr::null_mut::<UtHashBucket>();
            _he_new_buckets_0 = malloc(
                (2 as usize)
                    .wrapping_mul((*(*s).hhName.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets_0.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets_0 as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*s).hhName.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
                _he_bkt_i_0 = 0 as ::core::ffi::c_uint;
                while _he_bkt_i_0 < (*(*s).hhName.tbl).num_buckets {
                    _he_thh_0 = (*(*(*s).hhName.tbl).buckets.offset(_he_bkt_i_0 as isize)).hh_head
                        as *mut UtHashHandle;
                    while !_he_thh_0.is_null() {
                        _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                        _he_bkt_0 = (*_he_thh_0).hashv
                            & (*(*s).hhName.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt_0 =
                            _he_new_buckets_0.offset(_he_bkt_0 as isize) as *mut UtHashBucket;
                        (*_he_newbkt_0).count = (*_he_newbkt_0).count.wrapping_add(1);
                        if (*_he_newbkt_0).count > (*(*s).hhName.tbl).ideal_chain_maxlen {
                            (*(*s).hhName.tbl).nonideal_items =
                                (*(*s).hhName.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt_0).expand_mult = (*_he_newbkt_0)
                                .count
                                .wrapping_div((*(*s).hhName.tbl).ideal_chain_maxlen);
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
                free((*(*s).hhName.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*s).hhName.tbl).num_buckets = (*(*s).hhName.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*s).hhName.tbl).log2_num_buckets =
                    (*(*s).hhName.tbl).log2_num_buckets.wrapping_add(1);
                (*(*s).hhName.tbl).buckets = _he_new_buckets_0;
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
    }
    return name;
}
unsafe extern "C" fn otfcc_set_glyph_order_by_name(
    mut go: *mut GlyphOrder,
    mut name: SdsRaw,
    mut gid: GlyphId,
) -> bool {
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
            current_block_50 = 13766097234214211950;
        }
        10 => {
            current_block_50 = 13766097234214211950;
        }
        9 => {
            current_block_50 = 9661106225323354372;
        }
        8 => {
            current_block_50 = 4031302742064954157;
        }
        7 => {
            current_block_50 = 15968806681367933537;
        }
        6 => {
            current_block_50 = 11439671316796346568;
        }
        5 => {
            current_block_50 = 1798345118324096392;
        }
        4 => {
            current_block_50 = 4122224188782333186;
        }
        3 => {
            current_block_50 = 5592364819767578823;
        }
        2 => {
            current_block_50 = 16248428628490818279;
        }
        1 => {
            current_block_50 = 2938922248466119513;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        13766097234214211950 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 9661106225323354372;
        }
        _ => {}
    }
    match current_block_50 {
        9661106225323354372 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 4031302742064954157;
        }
        _ => {}
    }
    match current_block_50 {
        4031302742064954157 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 15968806681367933537;
        }
        _ => {}
    }
    match current_block_50 {
        15968806681367933537 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 11439671316796346568;
        }
        _ => {}
    }
    match current_block_50 {
        11439671316796346568 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 1798345118324096392;
        }
        _ => {}
    }
    match current_block_50 {
        1798345118324096392 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 4122224188782333186;
        }
        _ => {}
    }
    match current_block_50 {
        4122224188782333186 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 5592364819767578823;
        }
        _ => {}
    }
    match current_block_50 {
        5592364819767578823 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 16248428628490818279;
        }
        _ => {}
    }
    match current_block_50 {
        16248428628490818279 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 2938922248466119513;
        }
        _ => {}
    }
    match current_block_50 {
        2938922248466119513 => {
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
                    as *mut ::core::ffi::c_void as *mut GlyphOrderEntry
                    as *mut GlyphOrderEntry;
            } else {
                s = ::core::ptr::null_mut::<GlyphOrderEntry>();
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
                        as *mut GlyphOrderEntry
                        as *mut GlyphOrderEntry;
                } else {
                    s = ::core::ptr::null_mut::<GlyphOrderEntry>();
                }
            }
        }
    }
    if !s.is_null() {
        return false;
    } else {
        s = __caryll_allocate_clean(
            ::core::mem::size_of::<GlyphOrderEntry>() as usize,
            54 as ::core::ffi::c_ulong,
        ) as *mut GlyphOrderEntry;
        (*s).gid = gid;
        (*s).name = name;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            &raw mut (*s).gid as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = ::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint;
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
        _ha_hashv =
            _ha_hashv.wrapping_add(::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint);
        let mut current_block_169: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 14553733556618083747;
            }
            10 => {
                current_block_169 = 14553733556618083747;
            }
            9 => {
                current_block_169 = 1728435738637247368;
            }
            8 => {
                current_block_169 = 12121443671363514266;
            }
            7 => {
                current_block_169 = 8904636288275021233;
            }
            6 => {
                current_block_169 = 6752182583140876968;
            }
            5 => {
                current_block_169 = 10445752435246325618;
            }
            4 => {
                current_block_169 = 15366051406109840598;
            }
            3 => {
                current_block_169 = 12638627665820101961;
            }
            2 => {
                current_block_169 = 7759576118018759449;
            }
            1 => {
                current_block_169 = 4612577614011027765;
            }
            _ => {
                current_block_169 = 16835199615365683821;
            }
        }
        match current_block_169 {
            14553733556618083747 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 1728435738637247368;
            }
            _ => {}
        }
        match current_block_169 {
            1728435738637247368 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 12121443671363514266;
            }
            _ => {}
        }
        match current_block_169 {
            12121443671363514266 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 8904636288275021233;
            }
            _ => {}
        }
        match current_block_169 {
            8904636288275021233 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 6752182583140876968;
            }
            _ => {}
        }
        match current_block_169 {
            6752182583140876968 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 10445752435246325618;
            }
            _ => {}
        }
        match current_block_169 {
            10445752435246325618 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_169 = 15366051406109840598;
            }
            _ => {}
        }
        match current_block_169 {
            15366051406109840598 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 12638627665820101961;
            }
            _ => {}
        }
        match current_block_169 {
            12638627665820101961 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 7759576118018759449;
            }
            _ => {}
        }
        match current_block_169 {
            7759576118018759449 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 4612577614011027765;
            }
            _ => {}
        }
        match current_block_169 {
            4612577614011027765 => {
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
        (*s).hhID.hashv = _ha_hashv;
        (*s).hhID.key = &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hhID.keylen = ::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint;
        if (*go).byGID.is_null() {
            (*s).hhID.next = NULL;
            (*s).hhID.prev = NULL;
            (*s).hhID.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*s).hhID.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*s).hhID.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*s).hhID.tbl).tail = &raw mut (*s).hhID as *mut UtHashHandle;
                (*(*s).hhID.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*s).hhID.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*s).hhID.tbl).hho = (&raw mut (*s).hhID as *mut ::core::ffi::c_char)
                    .offset_from(s as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*s).hhID.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*s).hhID.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*s).hhID.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*s).hhID.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    );
                }
            }
            (*go).byGID = s;
        } else {
            (*s).hhID.tbl = (*(*go).byGID).hhID.tbl;
            (*s).hhID.next = NULL;
            (*s).hhID.prev = ((*(*(*go).byGID).hhID.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*go).byGID).hhID.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*go).byGID).hhID.tbl).tail).next = s as *mut ::core::ffi::c_void;
            (*(*(*go).byGID).hhID.tbl).tail = &raw mut (*s).hhID as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*go).byGID).hhID.tbl).num_items = (*(*(*go).byGID).hhID.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*go).byGID).hhID.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket =
            (*(*(*go).byGID).hhID.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*s).hhID.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
        (*s).hhID.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hhID as *mut UtHashHandle;
        }
        (*_ha_head).hh_head = &raw mut (*s).hhID as *mut UtHashHandle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*s).hhID.tbl).noexpand == 0
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
                    .wrapping_mul((*(*s).hhID.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*s).hhID.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                );
                (*(*s).hhID.tbl).ideal_chain_maxlen = ((*(*s).hhID.tbl).num_items
                    >> (*(*s).hhID.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*s).hhID.tbl).num_items
                        & (*(*s).hhID.tbl)
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
                (*(*s).hhID.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*s).hhID.tbl).num_buckets {
                    _he_thh = (*(*(*s).hhID.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UtHashHandle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*s).hhID.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*s).hhID.tbl).ideal_chain_maxlen {
                            (*(*s).hhID.tbl).nonideal_items =
                                (*(*s).hhID.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*s).hhID.tbl).ideal_chain_maxlen);
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
                free((*(*s).hhID.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*s).hhID.tbl).num_buckets = (*(*s).hhID.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*s).hhID.tbl).log2_num_buckets =
                    (*(*s).hhID.tbl).log2_num_buckets.wrapping_add(1);
                (*(*s).hhID.tbl).buckets = _he_new_buckets;
                (*(*s).hhID.tbl).ineff_expands = if (*(*s).hhID.tbl).nonideal_items
                    > (*(*s).hhID.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*s).hhID.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*s).hhID.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*s).hhID.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        let mut _ha_hashv_0: ::core::ffi::c_uint = 0;
        let mut _hj_i_1: ::core::ffi::c_uint = 0;
        let mut _hj_j_1: ::core::ffi::c_uint = 0;
        let mut _hj_k_1: ::core::ffi::c_uint = 0;
        let mut _hj_key_1: *const ::core::ffi::c_uchar =
            (*s).name.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_uchar;
        _ha_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_1 = _hj_j_1;
        _hj_k_1 = sdslen((*s).name) as ::core::ffi::c_uint;
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
            _ha_hashv_0 = _ha_hashv_0.wrapping_add(
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
            _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_0);
            _hj_i_1 ^= _ha_hashv_0 >> 13 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_1);
            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_1);
            _ha_hashv_0 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_0);
            _hj_i_1 ^= _ha_hashv_0 >> 12 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_1);
            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_1);
            _ha_hashv_0 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_0);
            _hj_i_1 ^= _ha_hashv_0 >> 3 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_1);
            _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_1);
            _ha_hashv_0 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
            _hj_key_1 = _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv_0 = _ha_hashv_0.wrapping_add(sdslen((*s).name) as ::core::ffi::c_uint);
        let mut current_block_357: u64;
        match _hj_k_1 {
            11 => {
                _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_357 = 11646744538012452655;
            }
            10 => {
                current_block_357 = 11646744538012452655;
            }
            9 => {
                current_block_357 = 11047456882538071530;
            }
            8 => {
                current_block_357 = 4382598792125733247;
            }
            7 => {
                current_block_357 = 10807807859896235056;
            }
            6 => {
                current_block_357 = 2043102903017609497;
            }
            5 => {
                current_block_357 = 4640274730172317356;
            }
            4 => {
                current_block_357 = 7175012969253723235;
            }
            3 => {
                current_block_357 = 12291675875295941000;
            }
            2 => {
                current_block_357 = 8269852238465038781;
            }
            1 => {
                current_block_357 = 5368539957103448007;
            }
            _ => {
                current_block_357 = 2014163327383235100;
            }
        }
        match current_block_357 {
            11646744538012452655 => {
                _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_357 = 11047456882538071530;
            }
            _ => {}
        }
        match current_block_357 {
            11047456882538071530 => {
                _ha_hashv_0 = _ha_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_357 = 4382598792125733247;
            }
            _ => {}
        }
        match current_block_357 {
            4382598792125733247 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_357 = 10807807859896235056;
            }
            _ => {}
        }
        match current_block_357 {
            10807807859896235056 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_357 = 2043102903017609497;
            }
            _ => {}
        }
        match current_block_357 {
            2043102903017609497 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_357 = 4640274730172317356;
            }
            _ => {}
        }
        match current_block_357 {
            4640274730172317356 => {
                _hj_j_1 =
                    _hj_j_1
                        .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_357 = 7175012969253723235;
            }
            _ => {}
        }
        match current_block_357 {
            7175012969253723235 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_357 = 12291675875295941000;
            }
            _ => {}
        }
        match current_block_357 {
            12291675875295941000 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_357 = 8269852238465038781;
            }
            _ => {}
        }
        match current_block_357 {
            8269852238465038781 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_357 = 5368539957103448007;
            }
            _ => {}
        }
        match current_block_357 {
            5368539957103448007 => {
                _hj_i_1 =
                    _hj_i_1
                        .wrapping_add(*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_0);
        _hj_i_1 ^= _ha_hashv_0 >> 13 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_0);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_1);
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_1);
        _ha_hashv_0 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_0);
        _hj_i_1 ^= _ha_hashv_0 >> 12 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_0);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_1);
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_1);
        _ha_hashv_0 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_ha_hashv_0);
        _hj_i_1 ^= _ha_hashv_0 >> 3 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_ha_hashv_0);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_i_1);
        _ha_hashv_0 = _ha_hashv_0.wrapping_sub(_hj_j_1);
        _ha_hashv_0 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
        (*s).hhName.hashv = _ha_hashv_0;
        (*s).hhName.key = (*s).name.offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hhName.keylen = sdslen((*s).name) as ::core::ffi::c_uint;
        if (*go).byName.is_null() {
            (*s).hhName.next = NULL;
            (*s).hhName.prev = NULL;
            (*s).hhName.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*s).hhName.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*s).hhName.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*s).hhName.tbl).tail = &raw mut (*s).hhName as *mut UtHashHandle;
                (*(*s).hhName.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*s).hhName.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*s).hhName.tbl).hho = (&raw mut (*s).hhName as *mut ::core::ffi::c_char)
                    .offset_from(s as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*s).hhName.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*s).hhName.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*s).hhName.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*s).hhName.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
            (*(*(*go).byName).hhName.tbl).tail = &raw mut (*s).hhName as *mut UtHashHandle;
        }
        let mut _ha_bkt_0: ::core::ffi::c_uint = 0;
        (*(*(*go).byName).hhName.tbl).num_items =
            (*(*(*go).byName).hhName.tbl).num_items.wrapping_add(1);
        _ha_bkt_0 = _ha_hashv_0
            & (*(*(*go).byName).hhName.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head_0: *mut UtHashBucket = (*(*(*go).byName).hhName.tbl)
            .buckets
            .offset(_ha_bkt_0 as isize)
            as *mut UtHashBucket;
        (*_ha_head_0).count = (*_ha_head_0).count.wrapping_add(1);
        (*s).hhName.hh_next = (*_ha_head_0).hh_head as *mut UtHashHandle;
        (*s).hhName.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head_0).hh_head.is_null() {
            (*(*_ha_head_0).hh_head).hh_prev = &raw mut (*s).hhName as *mut UtHashHandle;
        }
        (*_ha_head_0).hh_head = &raw mut (*s).hhName as *mut UtHashHandle;
        if (*_ha_head_0).count
            >= (*_ha_head_0)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*s).hhName.tbl).noexpand == 0
        {
            let mut _he_bkt_0: ::core::ffi::c_uint = 0;
            let mut _he_bkt_i_0: ::core::ffi::c_uint = 0;
            let mut _he_thh_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _he_hh_nxt_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
            let mut _he_new_buckets_0: *mut UtHashBucket =
                ::core::ptr::null_mut::<UtHashBucket>();
            let mut _he_newbkt_0: *mut UtHashBucket = ::core::ptr::null_mut::<UtHashBucket>();
            _he_new_buckets_0 = malloc(
                (2 as usize)
                    .wrapping_mul((*(*s).hhName.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets_0.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets_0 as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*s).hhName.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
                _he_bkt_i_0 = 0 as ::core::ffi::c_uint;
                while _he_bkt_i_0 < (*(*s).hhName.tbl).num_buckets {
                    _he_thh_0 = (*(*(*s).hhName.tbl).buckets.offset(_he_bkt_i_0 as isize)).hh_head
                        as *mut UtHashHandle;
                    while !_he_thh_0.is_null() {
                        _he_hh_nxt_0 = (*_he_thh_0).hh_next;
                        _he_bkt_0 = (*_he_thh_0).hashv
                            & (*(*s).hhName.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt_0 =
                            _he_new_buckets_0.offset(_he_bkt_0 as isize) as *mut UtHashBucket;
                        (*_he_newbkt_0).count = (*_he_newbkt_0).count.wrapping_add(1);
                        if (*_he_newbkt_0).count > (*(*s).hhName.tbl).ideal_chain_maxlen {
                            (*(*s).hhName.tbl).nonideal_items =
                                (*(*s).hhName.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt_0).expand_mult = (*_he_newbkt_0)
                                .count
                                .wrapping_div((*(*s).hhName.tbl).ideal_chain_maxlen);
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
                free((*(*s).hhName.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*s).hhName.tbl).num_buckets = (*(*s).hhName.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*s).hhName.tbl).log2_num_buckets =
                    (*(*s).hhName.tbl).log2_num_buckets.wrapping_add(1);
                (*(*s).hhName.tbl).buckets = _he_new_buckets_0;
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
        return true;
    };
}
unsafe extern "C" fn otfcc_gord_name_a_field_shared(
    mut go: *mut GlyphOrder,
    mut gid: GlyphId,
    mut field: *mut SdsRaw,
) -> bool {
    let mut t: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut gid as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 16449761503251207207;
        }
        10 => {
            current_block_50 = 16449761503251207207;
        }
        9 => {
            current_block_50 = 7776096533163639048;
        }
        8 => {
            current_block_50 = 17600203686288049616;
        }
        7 => {
            current_block_50 = 7080652995821737316;
        }
        6 => {
            current_block_50 = 5056383038760548209;
        }
        5 => {
            current_block_50 = 7979724192159390756;
        }
        4 => {
            current_block_50 = 1422191376976956946;
        }
        3 => {
            current_block_50 = 9860194344760608158;
        }
        2 => {
            current_block_50 = 11533937652505984344;
        }
        1 => {
            current_block_50 = 5635688670345452493;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        16449761503251207207 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 7776096533163639048;
        }
        _ => {}
    }
    match current_block_50 {
        7776096533163639048 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 17600203686288049616;
        }
        _ => {}
    }
    match current_block_50 {
        17600203686288049616 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 7080652995821737316;
        }
        _ => {}
    }
    match current_block_50 {
        7080652995821737316 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 5056383038760548209;
        }
        _ => {}
    }
    match current_block_50 {
        5056383038760548209 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 7979724192159390756;
        }
        _ => {}
    }
    match current_block_50 {
        7979724192159390756 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 1422191376976956946;
        }
        _ => {}
    }
    match current_block_50 {
        1422191376976956946 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 9860194344760608158;
        }
        _ => {}
    }
    match current_block_50 {
        9860194344760608158 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 11533937652505984344;
        }
        _ => {}
    }
    match current_block_50 {
        11533937652505984344 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 5635688670345452493;
        }
        _ => {}
    }
    match current_block_50 {
        5635688670345452493 => {
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
    t = ::core::ptr::null_mut::<GlyphOrderEntry>();
    if !(*go).byGID.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*go).byGID).hhID.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*go).byGID).hhID.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                t = ((*(*(*(*go).byGID).hhID.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).byGID).hhID.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut GlyphOrderEntry
                    as *mut GlyphOrderEntry;
            } else {
                t = ::core::ptr::null_mut::<GlyphOrderEntry>();
            }
            while !t.is_null() {
                if (*t).hhID.hashv == _hf_hashv
                    && (*t).hhID.keylen as usize == ::core::mem::size_of::<GlyphId>()
                {
                    if memcmp(
                        (*t).hhID.key,
                        &raw mut gid as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<GlyphId>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*t).hhID.hh_next.is_null() {
                    t = ((*t).hhID.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byGID).hhID.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut GlyphOrderEntry
                        as *mut GlyphOrderEntry;
                } else {
                    t = ::core::ptr::null_mut::<GlyphOrderEntry>();
                }
            }
        }
    }
    if !t.is_null() {
        *field = (*t).name;
        return true;
    } else {
        *field = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return false;
    };
}
unsafe extern "C" fn otfcc_gord_consolidate_handle(
    mut go: *mut GlyphOrder,
    mut h: *mut GlyphHandle,
) -> bool {
    if (*h).state == HandleState::Consolidated
    {
        let mut t: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key: *const ::core::ffi::c_uchar = (*h).name as *const ::core::ffi::c_uchar;
        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = sdslen((*h).name) as ::core::ffi::c_uint;
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
        _hf_hashv = _hf_hashv.wrapping_add(sdslen((*h).name) as ::core::ffi::c_uint);
        let mut current_block_50: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 15991719514752142291;
            }
            10 => {
                current_block_50 = 15991719514752142291;
            }
            9 => {
                current_block_50 = 12792210781472814257;
            }
            8 => {
                current_block_50 = 4518154457160535003;
            }
            7 => {
                current_block_50 = 2054208007906464708;
            }
            6 => {
                current_block_50 = 16203646679947228034;
            }
            5 => {
                current_block_50 = 4784991822849584160;
            }
            4 => {
                current_block_50 = 10760133070394170635;
            }
            3 => {
                current_block_50 = 1508656169144074749;
            }
            2 => {
                current_block_50 = 9687774073468160227;
            }
            1 => {
                current_block_50 = 539500008083385321;
            }
            _ => {
                current_block_50 = 3222590281903869779;
            }
        }
        match current_block_50 {
            15991719514752142291 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 12792210781472814257;
            }
            _ => {}
        }
        match current_block_50 {
            12792210781472814257 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 4518154457160535003;
            }
            _ => {}
        }
        match current_block_50 {
            4518154457160535003 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 2054208007906464708;
            }
            _ => {}
        }
        match current_block_50 {
            2054208007906464708 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 16203646679947228034;
            }
            _ => {}
        }
        match current_block_50 {
            16203646679947228034 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 4784991822849584160;
            }
            _ => {}
        }
        match current_block_50 {
            4784991822849584160 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_50 = 10760133070394170635;
            }
            _ => {}
        }
        match current_block_50 {
            10760133070394170635 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 1508656169144074749;
            }
            _ => {}
        }
        match current_block_50 {
            1508656169144074749 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 9687774073468160227;
            }
            _ => {}
        }
        match current_block_50 {
            9687774073468160227 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 539500008083385321;
            }
            _ => {}
        }
        match current_block_50 {
            539500008083385321 => {
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
        t = ::core::ptr::null_mut::<GlyphOrderEntry>();
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
                    t = ((*(*(*(*go).byName).hhName.tbl)
                        .buckets
                        .offset(_hf_bkt as isize))
                    .hh_head as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byName).hhName.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut GlyphOrderEntry
                        as *mut GlyphOrderEntry;
                } else {
                    t = ::core::ptr::null_mut::<GlyphOrderEntry>();
                }
                while !t.is_null() {
                    if (*t).hhName.hashv == _hf_hashv
                        && (*t).hhName.keylen as usize == sdslen((*h).name)
                    {
                        if memcmp(
                            (*t).hhName.key,
                            (*h).name as *const ::core::ffi::c_void,
                            sdslen((*h).name),
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*t).hhName.hh_next.is_null() {
                        t = ((*t).hhName.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*go).byName).hhName.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut GlyphOrderEntry
                            as *mut GlyphOrderEntry;
                    } else {
                        t = ::core::ptr::null_mut::<GlyphOrderEntry>();
                    }
                }
            }
        }
        if !t.is_null() {
            handle_consolidate_to(
                h as *mut Handle, (*t).gid, (*t).name
            );
            return true;
        }
        let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            &raw mut (*h).index as *const ::core::ffi::c_uchar;
        _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = ::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint;
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
            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
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
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
            _hj_i_0 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
            _hf_hashv_0 ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
            _hj_i_0 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
            _hf_hashv_0 ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
            _hj_i_0 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
            _hf_hashv_0 ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _hf_hashv_0 =
            _hf_hashv_0.wrapping_add(::core::mem::size_of::<GlyphId>() as ::core::ffi::c_uint);
        let mut current_block_168: u64;
        match _hj_k_0 {
            11 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_168 = 760239620317279985;
            }
            10 => {
                current_block_168 = 760239620317279985;
            }
            9 => {
                current_block_168 = 13984953578200811817;
            }
            8 => {
                current_block_168 = 3132269629679312143;
            }
            7 => {
                current_block_168 = 18295911540010265553;
            }
            6 => {
                current_block_168 = 14762674017870801242;
            }
            5 => {
                current_block_168 = 789247138018192599;
            }
            4 => {
                current_block_168 = 16079277737210154450;
            }
            3 => {
                current_block_168 = 7806641026720744807;
            }
            2 => {
                current_block_168 = 12856826563613872931;
            }
            1 => {
                current_block_168 = 4668966650698264838;
            }
            _ => {
                current_block_168 = 10763371041174037105;
            }
        }
        match current_block_168 {
            760239620317279985 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_168 = 13984953578200811817;
            }
            _ => {}
        }
        match current_block_168 {
            13984953578200811817 => {
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_168 = 3132269629679312143;
            }
            _ => {}
        }
        match current_block_168 {
            3132269629679312143 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_168 = 18295911540010265553;
            }
            _ => {}
        }
        match current_block_168 {
            18295911540010265553 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_168 = 14762674017870801242;
            }
            _ => {}
        }
        match current_block_168 {
            14762674017870801242 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_168 = 789247138018192599;
            }
            _ => {}
        }
        match current_block_168 {
            789247138018192599 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_168 = 16079277737210154450;
            }
            _ => {}
        }
        match current_block_168 {
            16079277737210154450 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_168 = 7806641026720744807;
            }
            _ => {}
        }
        match current_block_168 {
            7806641026720744807 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_168 = 12856826563613872931;
            }
            _ => {}
        }
        match current_block_168 {
            12856826563613872931 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_168 = 4668966650698264838;
            }
            _ => {}
        }
        match current_block_168 {
            4668966650698264838 => {
                _hj_i_0 =
                    _hj_i_0
                        .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
        _hj_i_0 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
        _hf_hashv_0 ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
        _hj_i_0 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
        _hf_hashv_0 ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_hf_hashv_0);
        _hj_i_0 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_hf_hashv_0);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_0);
        _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_0);
        _hf_hashv_0 ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
        t = ::core::ptr::null_mut::<GlyphOrderEntry>();
        if !(*go).byGID.is_null() {
            let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
            _hf_bkt_0 = _hf_hashv_0
                & (*(*(*go).byGID).hhName.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if !(*(*(*(*go).byGID).hhName.tbl)
                    .buckets
                    .offset(_hf_bkt_0 as isize))
                .hh_head
                .is_null()
                {
                    t = ((*(*(*(*go).byGID).hhName.tbl)
                        .buckets
                        .offset(_hf_bkt_0 as isize))
                    .hh_head as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byGID).hhName.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut GlyphOrderEntry
                        as *mut GlyphOrderEntry;
                } else {
                    t = ::core::ptr::null_mut::<GlyphOrderEntry>();
                }
                while !t.is_null() {
                    if (*t).hhName.hashv == _hf_hashv_0
                        && (*t).hhName.keylen as usize
                            == ::core::mem::size_of::<GlyphId>()
                    {
                        if memcmp(
                            (*t).hhName.key,
                            &raw mut (*h).index as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<GlyphId>() as usize,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*t).hhName.hh_next.is_null() {
                        t = ((*t).hhName.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*go).byGID).hhName.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut GlyphOrderEntry
                            as *mut GlyphOrderEntry;
                    } else {
                        t = ::core::ptr::null_mut::<GlyphOrderEntry>();
                    }
                }
            }
        }
        if !t.is_null() {
            handle_consolidate_to(
                h as *mut Handle, (*t).gid, (*t).name
            );
            return true;
        }
    } else if (*h).state == HandleState::Name
    {
        let mut t_0: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
        let mut _hf_hashv_1: ::core::ffi::c_uint = 0;
        let mut _hj_i_1: ::core::ffi::c_uint = 0;
        let mut _hj_j_1: ::core::ffi::c_uint = 0;
        let mut _hj_k_1: ::core::ffi::c_uint = 0;
        let mut _hj_key_1: *const ::core::ffi::c_uchar = (*h).name as *const ::core::ffi::c_uchar;
        _hf_hashv_1 = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_1 = _hj_j_1;
        _hj_k_1 = sdslen((*h).name) as ::core::ffi::c_uint;
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
            _hf_hashv_1 = _hf_hashv_1.wrapping_add(
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
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
            _hj_i_1 ^= _hf_hashv_1 >> 13 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
            _hf_hashv_1 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
            _hj_i_1 ^= _hf_hashv_1 >> 12 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
            _hf_hashv_1 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
            _hj_i_1 ^= _hf_hashv_1 >> 3 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
            _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
            _hf_hashv_1 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
            _hj_key_1 = _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _hf_hashv_1 = _hf_hashv_1.wrapping_add(sdslen((*h).name) as ::core::ffi::c_uint);
        let mut current_block_287: u64;
        match _hj_k_1 {
            11 => {
                _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                    (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_287 = 14219904885461170020;
            }
            10 => {
                current_block_287 = 14219904885461170020;
            }
            9 => {
                current_block_287 = 4258678933699050402;
            }
            8 => {
                current_block_287 = 14171057916402477577;
            }
            7 => {
                current_block_287 = 11478952826456484776;
            }
            6 => {
                current_block_287 = 16202070073083318761;
            }
            5 => {
                current_block_287 = 11760919130173375889;
            }
            4 => {
                current_block_287 = 10239502229319570081;
            }
            3 => {
                current_block_287 = 11009496093137509081;
            }
            2 => {
                current_block_287 = 2467494953490654602;
            }
            1 => {
                current_block_287 = 14288244254501244872;
            }
            _ => {
                current_block_287 = 7929919437805978889;
            }
        }
        match current_block_287 {
            14219904885461170020 => {
                _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                    (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_287 = 4258678933699050402;
            }
            _ => {}
        }
        match current_block_287 {
            4258678933699050402 => {
                _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                    (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_287 = 14171057916402477577;
            }
            _ => {}
        }
        match current_block_287 {
            14171057916402477577 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_287 = 11478952826456484776;
            }
            _ => {}
        }
        match current_block_287 {
            11478952826456484776 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_287 = 16202070073083318761;
            }
            _ => {}
        }
        match current_block_287 {
            16202070073083318761 => {
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_287 = 11760919130173375889;
            }
            _ => {}
        }
        match current_block_287 {
            11760919130173375889 => {
                _hj_j_1 =
                    _hj_j_1
                        .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_287 = 10239502229319570081;
            }
            _ => {}
        }
        match current_block_287 {
            10239502229319570081 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_287 = 11009496093137509081;
            }
            _ => {}
        }
        match current_block_287 {
            11009496093137509081 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_287 = 2467494953490654602;
            }
            _ => {}
        }
        match current_block_287 {
            2467494953490654602 => {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_287 = 14288244254501244872;
            }
            _ => {}
        }
        match current_block_287 {
            14288244254501244872 => {
                _hj_i_1 =
                    _hj_i_1
                        .wrapping_add(*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
        _hj_i_1 ^= _hf_hashv_1 >> 13 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
        _hf_hashv_1 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
        _hj_i_1 ^= _hf_hashv_1 >> 12 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
        _hf_hashv_1 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
        _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
        _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
        _hj_i_1 ^= _hf_hashv_1 >> 3 as ::core::ffi::c_int;
        _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
        _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
        _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
        _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
        _hf_hashv_1 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
        t_0 = ::core::ptr::null_mut::<GlyphOrderEntry>();
        if !(*go).byName.is_null() {
            let mut _hf_bkt_1: ::core::ffi::c_uint = 0;
            _hf_bkt_1 = _hf_hashv_1
                & (*(*(*go).byName).hhName.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if !(*(*(*(*go).byName).hhName.tbl)
                    .buckets
                    .offset(_hf_bkt_1 as isize))
                .hh_head
                .is_null()
                {
                    t_0 = ((*(*(*(*go).byName).hhName.tbl)
                        .buckets
                        .offset(_hf_bkt_1 as isize))
                    .hh_head as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byName).hhName.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut GlyphOrderEntry
                        as *mut GlyphOrderEntry;
                } else {
                    t_0 = ::core::ptr::null_mut::<GlyphOrderEntry>();
                }
                while !t_0.is_null() {
                    if (*t_0).hhName.hashv == _hf_hashv_1
                        && (*t_0).hhName.keylen as usize == sdslen((*h).name)
                    {
                        if memcmp(
                            (*t_0).hhName.key,
                            (*h).name as *const ::core::ffi::c_void,
                            sdslen((*h).name),
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*t_0).hhName.hh_next.is_null() {
                        t_0 = ((*t_0).hhName.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*go).byName).hhName.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut GlyphOrderEntry
                            as *mut GlyphOrderEntry;
                    } else {
                        t_0 = ::core::ptr::null_mut::<GlyphOrderEntry>();
                    }
                }
            }
        }
        if !t_0.is_null() {
            handle_consolidate_to(
                h as *mut Handle,
                (*t_0).gid,
                (*t_0).name,
            );
            return true;
        }
    } else if (*h).state == HandleState::Index
    {
        let mut name: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
        otfcc_gord_name_a_field_shared(go, (*h).index, &raw mut name);
        if !name.is_null() {
            handle_consolidate_to(
                h as *mut Handle, (*h).index, name
            );
            return true;
        }
    }
    return false;
}
unsafe extern "C" fn gord_lookup_name(mut go: *mut GlyphOrder, mut name: SdsRaw) -> bool {
    let mut t: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
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
            current_block_50 = 4336512233812436522;
        }
        10 => {
            current_block_50 = 4336512233812436522;
        }
        9 => {
            current_block_50 = 2319262467835594579;
        }
        8 => {
            current_block_50 = 14043089401349837855;
        }
        7 => {
            current_block_50 = 3988899265591278347;
        }
        6 => {
            current_block_50 = 706659924762427255;
        }
        5 => {
            current_block_50 = 474038249524142407;
        }
        4 => {
            current_block_50 = 9140822319437794843;
        }
        3 => {
            current_block_50 = 6370810708105011042;
        }
        2 => {
            current_block_50 = 571594985947008453;
        }
        1 => {
            current_block_50 = 13549654466115975535;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        4336512233812436522 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 2319262467835594579;
        }
        _ => {}
    }
    match current_block_50 {
        2319262467835594579 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 14043089401349837855;
        }
        _ => {}
    }
    match current_block_50 {
        14043089401349837855 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 3988899265591278347;
        }
        _ => {}
    }
    match current_block_50 {
        3988899265591278347 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 706659924762427255;
        }
        _ => {}
    }
    match current_block_50 {
        706659924762427255 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 474038249524142407;
        }
        _ => {}
    }
    match current_block_50 {
        474038249524142407 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 9140822319437794843;
        }
        _ => {}
    }
    match current_block_50 {
        9140822319437794843 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 6370810708105011042;
        }
        _ => {}
    }
    match current_block_50 {
        6370810708105011042 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 571594985947008453;
        }
        _ => {}
    }
    match current_block_50 {
        571594985947008453 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 13549654466115975535;
        }
        _ => {}
    }
    match current_block_50 {
        13549654466115975535 => {
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
    t = ::core::ptr::null_mut::<GlyphOrderEntry>();
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
                t = ((*(*(*(*go).byName).hhName.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*go).byName).hhName.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut GlyphOrderEntry
                    as *mut GlyphOrderEntry;
            } else {
                t = ::core::ptr::null_mut::<GlyphOrderEntry>();
            }
            while !t.is_null() {
                if (*t).hhName.hashv == _hf_hashv && (*t).hhName.keylen as usize == sdslen(name) {
                    if memcmp(
                        (*t).hhName.key,
                        name as *const ::core::ffi::c_void,
                        sdslen(name),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*t).hhName.hh_next.is_null() {
                    t = ((*t).hhName.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*go).byName).hhName.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut GlyphOrderEntry
                        as *mut GlyphOrderEntry;
                } else {
                    t = ::core::ptr::null_mut::<GlyphOrderEntry>();
                }
            }
        }
    }
    if !t.is_null() {
        return true;
    }
    return false;
}
pub static OTFCC_PKG_GLYPH_ORDER: GlyphOrderPackage = {
    GlyphOrderPackage {
        init: Some(otfcc_glyph_order_init as unsafe extern "C" fn(*mut GlyphOrder) -> ()),
        copy: Some(
            otfcc_glyph_order_copy
                as unsafe extern "C" fn(*mut GlyphOrder, *const GlyphOrder) -> (),
        ),
        move_0: Some(
            otfcc_glyph_order_move
                as unsafe extern "C" fn(*mut GlyphOrder, *mut GlyphOrder) -> (),
        ),
        dispose: Some(
            otfcc_glyph_order_dispose as unsafe extern "C" fn(*mut GlyphOrder) -> (),
        ),
        replace: Some(
            otfcc_glyph_order_replace
                as unsafe extern "C" fn(*mut GlyphOrder, GlyphOrder) -> (),
        ),
        copyReplace: Some(
            otfcc_glyph_order_copy_replace
                as unsafe extern "C" fn(*mut GlyphOrder, GlyphOrder) -> (),
        ),
        create: Some(otfcc_glyph_order_create),
        free: Some(otfcc_glyph_order_free as unsafe extern "C" fn(*mut GlyphOrder) -> ()),
        setByGID: Some(
            otfcc_set_glyph_order_by_gid
                as unsafe extern "C" fn(*mut GlyphOrder, GlyphId, SdsRaw) -> SdsRaw,
        ),
        setByName: Some(
            otfcc_set_glyph_order_by_name
                as unsafe extern "C" fn(*mut GlyphOrder, SdsRaw, GlyphId) -> bool,
        ),
        nameAField_Shared: Some(
            otfcc_gord_name_a_field_shared
                as unsafe extern "C" fn(*mut GlyphOrder, GlyphId, *mut SdsRaw) -> bool,
        ),
        consolidateHandle: Some(
            otfcc_gord_consolidate_handle
                as unsafe extern "C" fn(*mut GlyphOrder, *mut GlyphHandle) -> bool,
        ),
        lookupName: Some(
            gord_lookup_name as unsafe extern "C" fn(*mut GlyphOrder, SdsRaw) -> bool,
        ),
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    // The passes are a priority, so `Ord` is the whole point of the type -- but
    // derived `Ord` compares by declaration order, which is only the encoding
    // because the declarations happen to be in ascending order. Pin that, and
    // pin the zero: `otfcc_set_glyph_order_by_gid` calloc's an entry and never
    // assigns this field, so `GlyphOrderPass::Unset` has to be the all-zero value for the
    // field to be a valid `GlyphOrderPass` at all.
    #[test]
    fn glyphorderpass_order_is_its_encoding() {
        let all = [GlyphOrderPass::Unset, GlyphOrderPass::GlyphOrder, GlyphOrderPass::Notdef, GlyphOrderPass::Cmap, GlyphOrderPass::Glyf];
        for w in all.windows(2) {
            assert!(w[0] < w[1], "{:?} should rank above {:?}", w[0], w[1]);
            assert!((w[0] as u8) < (w[1] as u8));
        }
        assert_eq!(GlyphOrderPass::Unset as u8, 0);
        assert_eq!(GlyphOrderPass::GlyphOrder as u8, 1);
        assert_eq!(GlyphOrderPass::Notdef as u8, 2);
        assert_eq!(GlyphOrderPass::Cmap as u8, 3);
        assert_eq!(GlyphOrderPass::Glyf as u8, 4);
        assert_eq!(::core::mem::size_of::<GlyphOrderPass>(), 1);
    }
}
