#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memcpy, memset, strtol};

use crate::support::json_funcs::{json_obj_get_type};
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_dispose, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_8u, read_16u, read_24u, read_32u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId, Unicode};
use crate::vendor::sds::{Hex4Upper, SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_Block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::{NULL};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::bk::bkblock::{bk_newBlockFromBuffer, bk_newBlockFromBufferCopy};
use crate::bk::bkgraph::{bk_build_Block};
use crate::support::buffer::{buffree, buflen, bufnew, bufseek, bufwrite16b, bufwrite24b, bufwrite32b, bufwrite8, bufwrite_buf};
use crate::vendor::json_builder::{json_object_new, json_object_push, json_string_new_length};
use crate::vendor::sds::{sdsempty, sdsfree, sdsfromlonglong, sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CmapEntry {
    pub hh: UtHashHandle,
    pub unicode: ::core::ffi::c_int,
    pub glyph: GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CmapUvsKey {
    pub unicode: u32,
    pub selector: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CmapUvsEntry {
    pub hh: UtHashHandle,
    pub key: CmapUvsKey,
    pub glyph: GlyphHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CmapTable {
    pub unicodes: *mut CmapEntry,
    pub uvs: *mut CmapUvsEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CmapTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CmapTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CmapTable, *const CmapTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut CmapTable, *mut CmapTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CmapTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut CmapTable, CmapTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut CmapTable, CmapTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut CmapTable>,
    pub free: Option<unsafe extern "C" fn(*mut CmapTable) -> ()>,
    pub encodeByIndex:
        Option<unsafe extern "C" fn(*mut CmapTable, ::core::ffi::c_int, u16) -> bool>,
    pub encodeByName:
        Option<unsafe extern "C" fn(*mut CmapTable, ::core::ffi::c_int, SdsRaw) -> bool>,
    pub unmap: Option<unsafe extern "C" fn(*mut CmapTable, ::core::ffi::c_int) -> bool>,
    pub lookup: Option<
        unsafe extern "C" fn(*const CmapTable, ::core::ffi::c_int) -> *mut GlyphHandle,
    >,
    pub encodeUVSByIndex:
        Option<unsafe extern "C" fn(*mut CmapTable, CmapUvsKey, u16) -> bool>,
    pub encodeUVSByName: Option<unsafe extern "C" fn(*mut CmapTable, CmapUvsKey, SdsRaw) -> bool>,
    pub unmapUVS: Option<unsafe extern "C" fn(*mut CmapTable, CmapUvsKey) -> bool>,
    pub lookupUVS:
        Option<unsafe extern "C" fn(*const CmapTable, CmapUvsKey) -> *mut GlyphHandle>,
}
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn initCmap(mut cmap: *mut CmapTable) {
    (*cmap).unicodes = ::core::ptr::null_mut::<CmapEntry>();
    (*cmap).uvs = ::core::ptr::null_mut::<CmapUvsEntry>();
}
#[inline]
unsafe extern "C" fn disposeCmap(mut cmap: *mut CmapTable) {
    let mut s: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    let mut tmp: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    s = (*cmap).unicodes;
    tmp = (if !(*cmap).unicodes.is_null() {
        (*(*cmap).unicodes).hh.next
    } else {
        NULL
    }) as *mut CmapEntry as *mut CmapEntry;
    while !s.is_null() {
        otfcc_Handle_dispose(&raw mut (*s).glyph);
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*cmap).unicodes).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*cmap).unicodes).hh.tbl as *mut ::core::ffi::c_void);
            (*cmap).unicodes = ::core::ptr::null_mut::<CmapEntry>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*cmap).unicodes).hh.tbl).tail {
                (*(*(*cmap).unicodes).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*cmap).unicodes).hh.tbl).hho)
                        as *mut UtHashHandle as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh4 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*cmap).unicodes).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh4 = (*_hd_hh_del).next;
            } else {
                (*cmap).unicodes = (*_hd_hh_del).next as *mut CmapEntry as *mut CmapEntry;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh5 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*cmap).unicodes).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh5 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*cmap).unicodes).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket = (*(*(*cmap).unicodes).hh.tbl)
                .buckets
                .offset(_hd_bkt as isize)
                as *mut UtHashBucket;
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
            (*(*(*cmap).unicodes).hh.tbl).num_items =
                (*(*(*cmap).unicodes).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<CmapEntry>();
        s = tmp;
        tmp = (if !tmp.is_null() {
            (*tmp).hh.next
        } else {
            NULL
        }) as *mut CmapEntry as *mut CmapEntry;
    }
    let mut s_0: *mut CmapUvsEntry = ::core::ptr::null_mut::<CmapUvsEntry>();
    let mut tmp_0: *mut CmapUvsEntry = ::core::ptr::null_mut::<CmapUvsEntry>();
    s_0 = (*cmap).uvs;
    tmp_0 = (if !(*cmap).uvs.is_null() {
        (*(*cmap).uvs).hh.next
    } else {
        NULL
    }) as *mut CmapUvsEntry as *mut CmapUvsEntry;
    while !s_0.is_null() {
        otfcc_Handle_dispose(&raw mut (*s_0).glyph);
        let mut _hd_hh_del_0: *mut UtHashHandle = &raw mut (*s_0).hh;
        if (*_hd_hh_del_0).prev.is_null() && (*_hd_hh_del_0).next.is_null() {
            free((*(*(*cmap).uvs).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*cmap).uvs).hh.tbl as *mut ::core::ffi::c_void);
            (*cmap).uvs = ::core::ptr::null_mut::<CmapUvsEntry>();
        } else {
            let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
            if _hd_hh_del_0 == (*(*(*cmap).uvs).hh.tbl).tail {
                (*(*(*cmap).uvs).hh.tbl).tail = ((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del_0).prev.is_null() {
                let ref mut fresh6 = (*(((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh6 = (*_hd_hh_del_0).next;
            } else {
                (*cmap).uvs = (*_hd_hh_del_0).next as *mut CmapUvsEntry as *mut CmapUvsEntry;
            }
            if !(*_hd_hh_del_0).next.is_null() {
                let ref mut fresh7 = (*(((*_hd_hh_del_0).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh7 = (*_hd_hh_del_0).prev;
            }
            _hd_bkt_0 = (*_hd_hh_del_0).hashv
                & (*(*(*cmap).uvs).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head_0: *mut UtHashBucket =
                (*(*(*cmap).uvs).hh.tbl).buckets.offset(_hd_bkt_0 as isize) as *mut UtHashBucket;
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
            (*(*(*cmap).uvs).hh.tbl).num_items = (*(*(*cmap).uvs).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<CmapUvsEntry>();
        s_0 = tmp_0;
        tmp_0 = (if !tmp_0.is_null() {
            (*tmp_0).hh.next
        } else {
            NULL
        }) as *mut CmapUvsEntry as *mut CmapUvsEntry;
    }
}
#[inline]
unsafe extern "C" fn table_cmap_dispose(mut x: *mut CmapTable) {
    disposeCmap(x);
}
#[inline]
unsafe extern "C" fn table_cmap_create() -> *mut CmapTable {
    let mut x: *mut CmapTable =
        malloc(::core::mem::size_of::<CmapTable>() as usize) as *mut CmapTable;
    table_cmap_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_cmap_free(mut x: *mut CmapTable) {
    if x.is_null() {
        return;
    }
    table_cmap_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_cmap_copyReplace(mut dst: *mut CmapTable, src: CmapTable) {
    table_cmap_dispose(dst);
    table_cmap_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_cmap_copy(mut dst: *mut CmapTable, mut src: *const CmapTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CmapTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_cmap_replace(mut dst: *mut CmapTable, src: CmapTable) {
    table_cmap_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CmapTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_cmap_move(mut dst: *mut CmapTable, mut src: *mut CmapTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CmapTable>() as usize,
    );
    table_cmap_init(src);
}
#[inline]
unsafe extern "C" fn table_cmap_init(mut x: *mut CmapTable) {
    initCmap(x);
}
pub unsafe extern "C" fn otfcc_encodeCmapByIndex(
    mut cmap: *mut CmapTable,
    mut c: ::core::ffi::c_int,
    mut gid: u16,
) -> bool {
    let mut s: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut c as *const ::core::ffi::c_uchar;
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
    _hf_hashv =
        _hf_hashv.wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 9044378114796470248;
        }
        10 => {
            current_block_50 = 9044378114796470248;
        }
        9 => {
            current_block_50 = 9917382994331299318;
        }
        8 => {
            current_block_50 = 6972679091644067937;
        }
        7 => {
            current_block_50 = 14017527278585488367;
        }
        6 => {
            current_block_50 = 8587760982578482760;
        }
        5 => {
            current_block_50 = 5199535445667379257;
        }
        4 => {
            current_block_50 = 11154876609143343672;
        }
        3 => {
            current_block_50 = 13617326970112485193;
        }
        2 => {
            current_block_50 = 746863429919991827;
        }
        1 => {
            current_block_50 = 8502775811736688250;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        9044378114796470248 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 9917382994331299318;
        }
        _ => {}
    }
    match current_block_50 {
        9917382994331299318 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 6972679091644067937;
        }
        _ => {}
    }
    match current_block_50 {
        6972679091644067937 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 14017527278585488367;
        }
        _ => {}
    }
    match current_block_50 {
        14017527278585488367 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 8587760982578482760;
        }
        _ => {}
    }
    match current_block_50 {
        8587760982578482760 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 5199535445667379257;
        }
        _ => {}
    }
    match current_block_50 {
        5199535445667379257 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 11154876609143343672;
        }
        _ => {}
    }
    match current_block_50 {
        11154876609143343672 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 13617326970112485193;
        }
        _ => {}
    }
    match current_block_50 {
        13617326970112485193 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 746863429919991827;
        }
        _ => {}
    }
    match current_block_50 {
        746863429919991827 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 8502775811736688250;
        }
        _ => {}
    }
    match current_block_50 {
        8502775811736688250 => {
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
    s = ::core::ptr::null_mut::<CmapEntry>();
    if !(*cmap).unicodes.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*cmap).unicodes).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*cmap).unicodes).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                s = ((*(*(*(*cmap).unicodes).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut CmapEntry
                    as *mut CmapEntry;
            } else {
                s = ::core::ptr::null_mut::<CmapEntry>();
            }
            while !s.is_null() {
                if (*s).hh.hashv == _hf_hashv
                    && (*s).hh.keylen as usize
                        == ::core::mem::size_of::<::core::ffi::c_int>()
                {
                    if memcmp(
                        (*s).hh.key,
                        &raw mut c as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh.hh_next.is_null() {
                    s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut CmapEntry
                        as *mut CmapEntry;
                } else {
                    s = ::core::ptr::null_mut::<CmapEntry>();
                }
            }
        }
    }
    if s.is_null() {
        s = __caryll_allocate_clean(
            ::core::mem::size_of::<CmapEntry>() as usize,
            38 as ::core::ffi::c_ulong,
        ) as *mut CmapEntry;
        (*s).glyph = handle_fromIndex(gid as GlyphId)
            as GlyphHandle;
        (*s).unicode = c;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            &raw mut (*s).unicode as *const ::core::ffi::c_uchar;
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
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 13927762439963197780;
            }
            10 => {
                current_block_167 = 13927762439963197780;
            }
            9 => {
                current_block_167 = 11198643297954182352;
            }
            8 => {
                current_block_167 = 9428507068885265186;
            }
            7 => {
                current_block_167 = 1638505983517891218;
            }
            6 => {
                current_block_167 = 1700976555235367789;
            }
            5 => {
                current_block_167 = 15531660343075847752;
            }
            4 => {
                current_block_167 = 3664563030710462942;
            }
            3 => {
                current_block_167 = 13454419807287865238;
            }
            2 => {
                current_block_167 = 4897586879458547343;
            }
            1 => {
                current_block_167 = 3332484924777882288;
            }
            _ => {
                current_block_167 = 12608488225262500095;
            }
        }
        match current_block_167 {
            13927762439963197780 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 11198643297954182352;
            }
            _ => {}
        }
        match current_block_167 {
            11198643297954182352 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 9428507068885265186;
            }
            _ => {}
        }
        match current_block_167 {
            9428507068885265186 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 1638505983517891218;
            }
            _ => {}
        }
        match current_block_167 {
            1638505983517891218 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 1700976555235367789;
            }
            _ => {}
        }
        match current_block_167 {
            1700976555235367789 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 15531660343075847752;
            }
            _ => {}
        }
        match current_block_167 {
            15531660343075847752 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_167 = 3664563030710462942;
            }
            _ => {}
        }
        match current_block_167 {
            3664563030710462942 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 13454419807287865238;
            }
            _ => {}
        }
        match current_block_167 {
            13454419807287865238 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 4897586879458547343;
            }
            _ => {}
        }
        match current_block_167 {
            4897586879458547343 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 3332484924777882288;
            }
            _ => {}
        }
        match current_block_167 {
            3332484924777882288 => {
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
        (*s).hh.hashv = _ha_hashv;
        (*s).hh.key = &raw mut (*s).unicode as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hh.keylen = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
        if (*cmap).unicodes.is_null() {
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
                    as ::core::ffi::c_long as isize;
                (*(*s).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
            (*cmap).unicodes = s;
        } else {
            (*s).hh.tbl = (*(*cmap).unicodes).hh.tbl;
            (*s).hh.next = NULL;
            (*s).hh.prev = ((*(*(*cmap).unicodes).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*cmap).unicodes).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
            (*(*(*cmap).unicodes).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*cmap).unicodes).hh.tbl).num_items =
            (*(*(*cmap).unicodes).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*cmap).unicodes).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket = (*(*(*cmap).unicodes).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UtHashBucket;
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
                (*(*s).hh.tbl).log2_num_buckets = (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
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
        return true;
    } else {
        return false;
    };
}
pub unsafe extern "C" fn otfcc_encodeCmapByName(
    mut cmap: *mut CmapTable,
    mut c: ::core::ffi::c_int,
    mut name: SdsRaw,
) -> bool {
    let mut s: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut c as *const ::core::ffi::c_uchar;
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
    _hf_hashv =
        _hf_hashv.wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 6682450900777640555;
        }
        10 => {
            current_block_50 = 6682450900777640555;
        }
        9 => {
            current_block_50 = 2995490886174867074;
        }
        8 => {
            current_block_50 = 17471200600043765800;
        }
        7 => {
            current_block_50 = 16372678144794202059;
        }
        6 => {
            current_block_50 = 133000372198906578;
        }
        5 => {
            current_block_50 = 10688671130867452721;
        }
        4 => {
            current_block_50 = 18173353048957708837;
        }
        3 => {
            current_block_50 = 11206427327417317866;
        }
        2 => {
            current_block_50 = 15862337167129804910;
        }
        1 => {
            current_block_50 = 18056802818615439220;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        6682450900777640555 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 2995490886174867074;
        }
        _ => {}
    }
    match current_block_50 {
        2995490886174867074 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 17471200600043765800;
        }
        _ => {}
    }
    match current_block_50 {
        17471200600043765800 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 16372678144794202059;
        }
        _ => {}
    }
    match current_block_50 {
        16372678144794202059 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 133000372198906578;
        }
        _ => {}
    }
    match current_block_50 {
        133000372198906578 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 10688671130867452721;
        }
        _ => {}
    }
    match current_block_50 {
        10688671130867452721 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 18173353048957708837;
        }
        _ => {}
    }
    match current_block_50 {
        18173353048957708837 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 11206427327417317866;
        }
        _ => {}
    }
    match current_block_50 {
        11206427327417317866 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 15862337167129804910;
        }
        _ => {}
    }
    match current_block_50 {
        15862337167129804910 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 18056802818615439220;
        }
        _ => {}
    }
    match current_block_50 {
        18056802818615439220 => {
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
    s = ::core::ptr::null_mut::<CmapEntry>();
    if !(*cmap).unicodes.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*cmap).unicodes).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*cmap).unicodes).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                s = ((*(*(*(*cmap).unicodes).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut CmapEntry
                    as *mut CmapEntry;
            } else {
                s = ::core::ptr::null_mut::<CmapEntry>();
            }
            while !s.is_null() {
                if (*s).hh.hashv == _hf_hashv
                    && (*s).hh.keylen as usize
                        == ::core::mem::size_of::<::core::ffi::c_int>()
                {
                    if memcmp(
                        (*s).hh.key,
                        &raw mut c as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh.hh_next.is_null() {
                    s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut CmapEntry
                        as *mut CmapEntry;
                } else {
                    s = ::core::ptr::null_mut::<CmapEntry>();
                }
            }
        }
    }
    if s.is_null() {
        s = __caryll_allocate_clean(
            ::core::mem::size_of::<CmapEntry>() as usize,
            51 as ::core::ffi::c_ulong,
        ) as *mut CmapEntry;
        (*s).glyph =
            handle_fromName(name) as GlyphHandle;
        (*s).unicode = c;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            &raw mut (*s).unicode as *const ::core::ffi::c_uchar;
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
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 17991326672772475971;
            }
            10 => {
                current_block_167 = 17991326672772475971;
            }
            9 => {
                current_block_167 = 1966471982336213573;
            }
            8 => {
                current_block_167 = 17477459671792975562;
            }
            7 => {
                current_block_167 = 17167037485604423375;
            }
            6 => {
                current_block_167 = 4726878511052239120;
            }
            5 => {
                current_block_167 = 13581405347896860154;
            }
            4 => {
                current_block_167 = 12426868535934713195;
            }
            3 => {
                current_block_167 = 6502958868035307762;
            }
            2 => {
                current_block_167 = 18220756614549086033;
            }
            1 => {
                current_block_167 = 6419724591471093776;
            }
            _ => {
                current_block_167 = 12608488225262500095;
            }
        }
        match current_block_167 {
            17991326672772475971 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 1966471982336213573;
            }
            _ => {}
        }
        match current_block_167 {
            1966471982336213573 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 17477459671792975562;
            }
            _ => {}
        }
        match current_block_167 {
            17477459671792975562 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 17167037485604423375;
            }
            _ => {}
        }
        match current_block_167 {
            17167037485604423375 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 4726878511052239120;
            }
            _ => {}
        }
        match current_block_167 {
            4726878511052239120 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 13581405347896860154;
            }
            _ => {}
        }
        match current_block_167 {
            13581405347896860154 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_167 = 12426868535934713195;
            }
            _ => {}
        }
        match current_block_167 {
            12426868535934713195 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 6502958868035307762;
            }
            _ => {}
        }
        match current_block_167 {
            6502958868035307762 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 18220756614549086033;
            }
            _ => {}
        }
        match current_block_167 {
            18220756614549086033 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 6419724591471093776;
            }
            _ => {}
        }
        match current_block_167 {
            6419724591471093776 => {
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
        (*s).hh.hashv = _ha_hashv;
        (*s).hh.key = &raw mut (*s).unicode as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hh.keylen = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
        if (*cmap).unicodes.is_null() {
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
                    as ::core::ffi::c_long as isize;
                (*(*s).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
            (*cmap).unicodes = s;
        } else {
            (*s).hh.tbl = (*(*cmap).unicodes).hh.tbl;
            (*s).hh.next = NULL;
            (*s).hh.prev = ((*(*(*cmap).unicodes).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*cmap).unicodes).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
            (*(*(*cmap).unicodes).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*cmap).unicodes).hh.tbl).num_items =
            (*(*(*cmap).unicodes).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*cmap).unicodes).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket = (*(*(*cmap).unicodes).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UtHashBucket;
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
                (*(*s).hh.tbl).log2_num_buckets = (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
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
        return true;
    } else {
        return false;
    };
}
pub unsafe extern "C" fn otfcc_unmapCmap(
    mut cmap: *mut CmapTable,
    mut c: ::core::ffi::c_int,
) -> bool {
    let mut s: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut c as *const ::core::ffi::c_uchar;
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
    _hf_hashv =
        _hf_hashv.wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 5902317255534392816;
        }
        10 => {
            current_block_50 = 5902317255534392816;
        }
        9 => {
            current_block_50 = 11010643160548443151;
        }
        8 => {
            current_block_50 = 3232805687429972406;
        }
        7 => {
            current_block_50 = 9374551397133760835;
        }
        6 => {
            current_block_50 = 4280470333003720874;
        }
        5 => {
            current_block_50 = 13709724089118602981;
        }
        4 => {
            current_block_50 = 1140016624855719843;
        }
        3 => {
            current_block_50 = 4195936686283542742;
        }
        2 => {
            current_block_50 = 16167378424684552699;
        }
        1 => {
            current_block_50 = 1941254239296963753;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        5902317255534392816 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 11010643160548443151;
        }
        _ => {}
    }
    match current_block_50 {
        11010643160548443151 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 3232805687429972406;
        }
        _ => {}
    }
    match current_block_50 {
        3232805687429972406 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 9374551397133760835;
        }
        _ => {}
    }
    match current_block_50 {
        9374551397133760835 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 4280470333003720874;
        }
        _ => {}
    }
    match current_block_50 {
        4280470333003720874 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 13709724089118602981;
        }
        _ => {}
    }
    match current_block_50 {
        13709724089118602981 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 1140016624855719843;
        }
        _ => {}
    }
    match current_block_50 {
        1140016624855719843 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 4195936686283542742;
        }
        _ => {}
    }
    match current_block_50 {
        4195936686283542742 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 16167378424684552699;
        }
        _ => {}
    }
    match current_block_50 {
        16167378424684552699 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 1941254239296963753;
        }
        _ => {}
    }
    match current_block_50 {
        1941254239296963753 => {
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
    s = ::core::ptr::null_mut::<CmapEntry>();
    if !(*cmap).unicodes.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*cmap).unicodes).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*cmap).unicodes).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                s = ((*(*(*(*cmap).unicodes).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut CmapEntry
                    as *mut CmapEntry;
            } else {
                s = ::core::ptr::null_mut::<CmapEntry>();
            }
            while !s.is_null() {
                if (*s).hh.hashv == _hf_hashv
                    && (*s).hh.keylen as usize
                        == ::core::mem::size_of::<::core::ffi::c_int>()
                {
                    if memcmp(
                        (*s).hh.key,
                        &raw mut c as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh.hh_next.is_null() {
                    s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut CmapEntry
                        as *mut CmapEntry;
                } else {
                    s = ::core::ptr::null_mut::<CmapEntry>();
                }
            }
        }
    }
    if !s.is_null() {
        otfcc_Handle_dispose(&raw mut (*s).glyph);
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*cmap).unicodes).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*cmap).unicodes).hh.tbl as *mut ::core::ffi::c_void);
            (*cmap).unicodes = ::core::ptr::null_mut::<CmapEntry>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*cmap).unicodes).hh.tbl).tail {
                (*(*(*cmap).unicodes).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*cmap).unicodes).hh.tbl).hho)
                        as *mut UtHashHandle as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*cmap).unicodes).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh2 = (*_hd_hh_del).next;
            } else {
                (*cmap).unicodes = (*_hd_hh_del).next as *mut CmapEntry as *mut CmapEntry;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*cmap).unicodes).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh3 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*cmap).unicodes).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket = (*(*(*cmap).unicodes).hh.tbl)
                .buckets
                .offset(_hd_bkt as isize)
                as *mut UtHashBucket;
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
            (*(*(*cmap).unicodes).hh.tbl).num_items =
                (*(*(*cmap).unicodes).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<CmapEntry>();
        return true;
    } else {
        return false;
    };
}
pub unsafe extern "C" fn otfcc_cmapLookup(
    mut cmap: *const CmapTable,
    mut c: ::core::ffi::c_int,
) -> *mut GlyphHandle {
    let mut s: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut c as *const ::core::ffi::c_uchar;
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
    _hf_hashv =
        _hf_hashv.wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 15626024973067977090;
        }
        10 => {
            current_block_50 = 15626024973067977090;
        }
        9 => {
            current_block_50 = 5373162655536212628;
        }
        8 => {
            current_block_50 = 4885580515847958508;
        }
        7 => {
            current_block_50 = 6398731384216643951;
        }
        6 => {
            current_block_50 = 1719487891348337170;
        }
        5 => {
            current_block_50 = 10842987486580778971;
        }
        4 => {
            current_block_50 = 8668419037924894788;
        }
        3 => {
            current_block_50 = 5817729713930350314;
        }
        2 => {
            current_block_50 = 7682842965055680945;
        }
        1 => {
            current_block_50 = 1293547607176583601;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        15626024973067977090 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 5373162655536212628;
        }
        _ => {}
    }
    match current_block_50 {
        5373162655536212628 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 4885580515847958508;
        }
        _ => {}
    }
    match current_block_50 {
        4885580515847958508 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 6398731384216643951;
        }
        _ => {}
    }
    match current_block_50 {
        6398731384216643951 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 1719487891348337170;
        }
        _ => {}
    }
    match current_block_50 {
        1719487891348337170 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 10842987486580778971;
        }
        _ => {}
    }
    match current_block_50 {
        10842987486580778971 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 8668419037924894788;
        }
        _ => {}
    }
    match current_block_50 {
        8668419037924894788 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 5817729713930350314;
        }
        _ => {}
    }
    match current_block_50 {
        5817729713930350314 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 7682842965055680945;
        }
        _ => {}
    }
    match current_block_50 {
        7682842965055680945 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 1293547607176583601;
        }
        _ => {}
    }
    match current_block_50 {
        1293547607176583601 => {
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
    s = ::core::ptr::null_mut::<CmapEntry>();
    if !(*cmap).unicodes.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*cmap).unicodes).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*cmap).unicodes).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                s = ((*(*(*(*cmap).unicodes).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut CmapEntry
                    as *mut CmapEntry;
            } else {
                s = ::core::ptr::null_mut::<CmapEntry>();
            }
            while !s.is_null() {
                if (*s).hh.hashv == _hf_hashv
                    && (*s).hh.keylen as usize
                        == ::core::mem::size_of::<::core::ffi::c_int>()
                {
                    if memcmp(
                        (*s).hh.key,
                        &raw mut c as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh.hh_next.is_null() {
                    s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut CmapEntry
                        as *mut CmapEntry;
                } else {
                    s = ::core::ptr::null_mut::<CmapEntry>();
                }
            }
        }
    }
    if !s.is_null() {
        return &raw mut (*s).glyph;
    } else {
        return ::core::ptr::null_mut::<GlyphHandle>();
    };
}
pub unsafe extern "C" fn otfcc_encodeCmapUVSByIndex(
    mut cmap: *mut CmapTable,
    mut c: CmapUvsKey,
    mut gid: u16,
) -> bool {
    let mut s: *mut CmapUvsEntry = ::core::ptr::null_mut::<CmapUvsEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut c as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint;
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
    _hf_hashv =
        _hf_hashv.wrapping_add(::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 322669159951527645;
        }
        10 => {
            current_block_50 = 322669159951527645;
        }
        9 => {
            current_block_50 = 5206281362889120867;
        }
        8 => {
            current_block_50 = 13133319464457553044;
        }
        7 => {
            current_block_50 = 10054832596883997805;
        }
        6 => {
            current_block_50 = 9536396861727289318;
        }
        5 => {
            current_block_50 = 10593234660913827991;
        }
        4 => {
            current_block_50 = 13138328607886192104;
        }
        3 => {
            current_block_50 = 12229600276408061123;
        }
        2 => {
            current_block_50 = 9460353118185870455;
        }
        1 => {
            current_block_50 = 528482631692581198;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        322669159951527645 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 5206281362889120867;
        }
        _ => {}
    }
    match current_block_50 {
        5206281362889120867 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 13133319464457553044;
        }
        _ => {}
    }
    match current_block_50 {
        13133319464457553044 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 10054832596883997805;
        }
        _ => {}
    }
    match current_block_50 {
        10054832596883997805 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 9536396861727289318;
        }
        _ => {}
    }
    match current_block_50 {
        9536396861727289318 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 10593234660913827991;
        }
        _ => {}
    }
    match current_block_50 {
        10593234660913827991 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 13138328607886192104;
        }
        _ => {}
    }
    match current_block_50 {
        13138328607886192104 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 12229600276408061123;
        }
        _ => {}
    }
    match current_block_50 {
        12229600276408061123 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 9460353118185870455;
        }
        _ => {}
    }
    match current_block_50 {
        9460353118185870455 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 528482631692581198;
        }
        _ => {}
    }
    match current_block_50 {
        528482631692581198 => {
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
    s = ::core::ptr::null_mut::<CmapUvsEntry>();
    if !(*cmap).uvs.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*cmap).uvs).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*cmap).uvs).hh.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                s = ((*(*(*(*cmap).uvs).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut CmapUvsEntry
                    as *mut CmapUvsEntry;
            } else {
                s = ::core::ptr::null_mut::<CmapUvsEntry>();
            }
            while !s.is_null() {
                if (*s).hh.hashv == _hf_hashv
                    && (*s).hh.keylen as usize == ::core::mem::size_of::<CmapUvsKey>()
                {
                    if memcmp(
                        (*s).hh.key,
                        &raw mut c as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<CmapUvsKey>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh.hh_next.is_null() {
                    s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut CmapUvsEntry
                        as *mut CmapUvsEntry;
                } else {
                    s = ::core::ptr::null_mut::<CmapUvsEntry>();
                }
            }
        }
    }
    if s.is_null() {
        s = __caryll_allocate_clean(
            ::core::mem::size_of::<CmapUvsEntry>() as usize,
            87 as ::core::ffi::c_ulong,
        ) as *mut CmapUvsEntry;
        (*s).glyph = handle_fromIndex(gid as GlyphId)
            as GlyphHandle;
        (*s).key = c;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            &raw mut (*s).key as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = ::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint;
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
            _ha_hashv.wrapping_add(::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint);
        let mut current_block_167: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 5653165791503131018;
            }
            10 => {
                current_block_167 = 5653165791503131018;
            }
            9 => {
                current_block_167 = 15693441303895049031;
            }
            8 => {
                current_block_167 = 8754147740681098638;
            }
            7 => {
                current_block_167 = 3856747812629247609;
            }
            6 => {
                current_block_167 = 7113409298466229034;
            }
            5 => {
                current_block_167 = 8593202007706503865;
            }
            4 => {
                current_block_167 = 1795139904671849378;
            }
            3 => {
                current_block_167 = 15451326646815098194;
            }
            2 => {
                current_block_167 = 13245741422963388543;
            }
            1 => {
                current_block_167 = 6353543376077280580;
            }
            _ => {
                current_block_167 = 12608488225262500095;
            }
        }
        match current_block_167 {
            5653165791503131018 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 15693441303895049031;
            }
            _ => {}
        }
        match current_block_167 {
            15693441303895049031 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 8754147740681098638;
            }
            _ => {}
        }
        match current_block_167 {
            8754147740681098638 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 3856747812629247609;
            }
            _ => {}
        }
        match current_block_167 {
            3856747812629247609 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 7113409298466229034;
            }
            _ => {}
        }
        match current_block_167 {
            7113409298466229034 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 8593202007706503865;
            }
            _ => {}
        }
        match current_block_167 {
            8593202007706503865 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_167 = 1795139904671849378;
            }
            _ => {}
        }
        match current_block_167 {
            1795139904671849378 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 15451326646815098194;
            }
            _ => {}
        }
        match current_block_167 {
            15451326646815098194 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 13245741422963388543;
            }
            _ => {}
        }
        match current_block_167 {
            13245741422963388543 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 6353543376077280580;
            }
            _ => {}
        }
        match current_block_167 {
            6353543376077280580 => {
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
        (*s).hh.hashv = _ha_hashv;
        (*s).hh.key = &raw mut (*s).key as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hh.keylen = ::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint;
        if (*cmap).uvs.is_null() {
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
                    as ::core::ffi::c_long as isize;
                (*(*s).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
            (*cmap).uvs = s;
        } else {
            (*s).hh.tbl = (*(*cmap).uvs).hh.tbl;
            (*s).hh.next = NULL;
            (*s).hh.prev = ((*(*(*cmap).uvs).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*cmap).uvs).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
            (*(*(*cmap).uvs).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*cmap).uvs).hh.tbl).num_items = (*(*(*cmap).uvs).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*cmap).uvs).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket =
            (*(*(*cmap).uvs).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
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
                (*(*s).hh.tbl).log2_num_buckets = (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
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
        return true;
    } else {
        return false;
    };
}
pub unsafe extern "C" fn otfcc_encodeCmapUVSByName(
    mut cmap: *mut CmapTable,
    mut c: CmapUvsKey,
    mut name: SdsRaw,
) -> bool {
    let mut s: *mut CmapUvsEntry = ::core::ptr::null_mut::<CmapUvsEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut c as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint;
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
    _hf_hashv =
        _hf_hashv.wrapping_add(::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 3419577943856654172;
        }
        10 => {
            current_block_50 = 3419577943856654172;
        }
        9 => {
            current_block_50 = 2270793391029349669;
        }
        8 => {
            current_block_50 = 4828993725674887990;
        }
        7 => {
            current_block_50 = 7360702717819258348;
        }
        6 => {
            current_block_50 = 1221739417720807732;
        }
        5 => {
            current_block_50 = 1570477825169348060;
        }
        4 => {
            current_block_50 = 14199474539700374465;
        }
        3 => {
            current_block_50 = 7988266359792952127;
        }
        2 => {
            current_block_50 = 16687080193289705592;
        }
        1 => {
            current_block_50 = 9669664513702107797;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        3419577943856654172 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 2270793391029349669;
        }
        _ => {}
    }
    match current_block_50 {
        2270793391029349669 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 4828993725674887990;
        }
        _ => {}
    }
    match current_block_50 {
        4828993725674887990 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 7360702717819258348;
        }
        _ => {}
    }
    match current_block_50 {
        7360702717819258348 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 1221739417720807732;
        }
        _ => {}
    }
    match current_block_50 {
        1221739417720807732 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 1570477825169348060;
        }
        _ => {}
    }
    match current_block_50 {
        1570477825169348060 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 14199474539700374465;
        }
        _ => {}
    }
    match current_block_50 {
        14199474539700374465 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 7988266359792952127;
        }
        _ => {}
    }
    match current_block_50 {
        7988266359792952127 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 16687080193289705592;
        }
        _ => {}
    }
    match current_block_50 {
        16687080193289705592 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 9669664513702107797;
        }
        _ => {}
    }
    match current_block_50 {
        9669664513702107797 => {
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
    s = ::core::ptr::null_mut::<CmapUvsEntry>();
    if !(*cmap).uvs.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*cmap).uvs).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*cmap).uvs).hh.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                s = ((*(*(*(*cmap).uvs).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut CmapUvsEntry
                    as *mut CmapUvsEntry;
            } else {
                s = ::core::ptr::null_mut::<CmapUvsEntry>();
            }
            while !s.is_null() {
                if (*s).hh.hashv == _hf_hashv
                    && (*s).hh.keylen as usize == ::core::mem::size_of::<CmapUvsKey>()
                {
                    if memcmp(
                        (*s).hh.key,
                        &raw mut c as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<CmapUvsKey>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh.hh_next.is_null() {
                    s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut CmapUvsEntry
                        as *mut CmapUvsEntry;
                } else {
                    s = ::core::ptr::null_mut::<CmapUvsEntry>();
                }
            }
        }
    }
    if s.is_null() {
        s = __caryll_allocate_clean(
            ::core::mem::size_of::<CmapUvsEntry>() as usize,
            100 as ::core::ffi::c_ulong,
        ) as *mut CmapUvsEntry;
        (*s).glyph =
            handle_fromName(name) as GlyphHandle;
        (*s).key = c;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            &raw mut (*s).key as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = ::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint;
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
            _ha_hashv.wrapping_add(::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint);
        let mut current_block_167: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 17703247306261445513;
            }
            10 => {
                current_block_167 = 17703247306261445513;
            }
            9 => {
                current_block_167 = 11053768700258126603;
            }
            8 => {
                current_block_167 = 896523993098841782;
            }
            7 => {
                current_block_167 = 18401480588297710244;
            }
            6 => {
                current_block_167 = 10133674040774954369;
            }
            5 => {
                current_block_167 = 13624143383088606119;
            }
            4 => {
                current_block_167 = 17011164281425506503;
            }
            3 => {
                current_block_167 = 3142581409041571743;
            }
            2 => {
                current_block_167 = 14121867561516531824;
            }
            1 => {
                current_block_167 = 2346692697102523595;
            }
            _ => {
                current_block_167 = 12608488225262500095;
            }
        }
        match current_block_167 {
            17703247306261445513 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 11053768700258126603;
            }
            _ => {}
        }
        match current_block_167 {
            11053768700258126603 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 896523993098841782;
            }
            _ => {}
        }
        match current_block_167 {
            896523993098841782 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 18401480588297710244;
            }
            _ => {}
        }
        match current_block_167 {
            18401480588297710244 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 10133674040774954369;
            }
            _ => {}
        }
        match current_block_167 {
            10133674040774954369 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 13624143383088606119;
            }
            _ => {}
        }
        match current_block_167 {
            13624143383088606119 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_167 = 17011164281425506503;
            }
            _ => {}
        }
        match current_block_167 {
            17011164281425506503 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 3142581409041571743;
            }
            _ => {}
        }
        match current_block_167 {
            3142581409041571743 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 14121867561516531824;
            }
            _ => {}
        }
        match current_block_167 {
            14121867561516531824 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 2346692697102523595;
            }
            _ => {}
        }
        match current_block_167 {
            2346692697102523595 => {
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
        (*s).hh.hashv = _ha_hashv;
        (*s).hh.key = &raw mut (*s).key as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*s).hh.keylen = ::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint;
        if (*cmap).uvs.is_null() {
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
                    as ::core::ffi::c_long as isize;
                (*(*s).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
            (*cmap).uvs = s;
        } else {
            (*s).hh.tbl = (*(*cmap).uvs).hh.tbl;
            (*s).hh.next = NULL;
            (*s).hh.prev = ((*(*(*cmap).uvs).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*cmap).uvs).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
            (*(*(*cmap).uvs).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*cmap).uvs).hh.tbl).num_items = (*(*(*cmap).uvs).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*cmap).uvs).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket =
            (*(*(*cmap).uvs).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
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
                (*(*s).hh.tbl).log2_num_buckets = (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
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
        return true;
    } else {
        return false;
    };
}
pub unsafe extern "C" fn otfcc_unmapCmapUVS(
    mut cmap: *mut CmapTable,
    mut c: CmapUvsKey,
) -> bool {
    let mut s: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut c as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint;
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
    _hf_hashv =
        _hf_hashv.wrapping_add(::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 16979327958298180893;
        }
        10 => {
            current_block_50 = 16979327958298180893;
        }
        9 => {
            current_block_50 = 10325693711116906500;
        }
        8 => {
            current_block_50 = 16657580605428744710;
        }
        7 => {
            current_block_50 = 1061737911073990113;
        }
        6 => {
            current_block_50 = 5021830257878723497;
        }
        5 => {
            current_block_50 = 8088101872378080883;
        }
        4 => {
            current_block_50 = 12788681663890280165;
        }
        3 => {
            current_block_50 = 11988725688425306594;
        }
        2 => {
            current_block_50 = 339271371264965088;
        }
        1 => {
            current_block_50 = 5446939044677969361;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        16979327958298180893 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 10325693711116906500;
        }
        _ => {}
    }
    match current_block_50 {
        10325693711116906500 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 16657580605428744710;
        }
        _ => {}
    }
    match current_block_50 {
        16657580605428744710 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 1061737911073990113;
        }
        _ => {}
    }
    match current_block_50 {
        1061737911073990113 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 5021830257878723497;
        }
        _ => {}
    }
    match current_block_50 {
        5021830257878723497 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 8088101872378080883;
        }
        _ => {}
    }
    match current_block_50 {
        8088101872378080883 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 12788681663890280165;
        }
        _ => {}
    }
    match current_block_50 {
        12788681663890280165 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 11988725688425306594;
        }
        _ => {}
    }
    match current_block_50 {
        11988725688425306594 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 339271371264965088;
        }
        _ => {}
    }
    match current_block_50 {
        339271371264965088 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 5446939044677969361;
        }
        _ => {}
    }
    match current_block_50 {
        5446939044677969361 => {
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
    s = ::core::ptr::null_mut::<CmapEntry>();
    if !(*cmap).uvs.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*cmap).uvs).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*cmap).uvs).hh.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                s = ((*(*(*(*cmap).uvs).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut CmapEntry
                    as *mut CmapEntry;
            } else {
                s = ::core::ptr::null_mut::<CmapEntry>();
            }
            while !s.is_null() {
                if (*s).hh.hashv == _hf_hashv
                    && (*s).hh.keylen as usize == ::core::mem::size_of::<CmapUvsKey>()
                {
                    if memcmp(
                        (*s).hh.key,
                        &raw mut c as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<CmapUvsKey>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh.hh_next.is_null() {
                    s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut CmapEntry
                        as *mut CmapEntry;
                } else {
                    s = ::core::ptr::null_mut::<CmapEntry>();
                }
            }
        }
    }
    if !s.is_null() {
        otfcc_Handle_dispose(&raw mut (*s).glyph);
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*cmap).uvs).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*cmap).uvs).hh.tbl as *mut ::core::ffi::c_void);
            (*cmap).uvs = ::core::ptr::null_mut::<CmapUvsEntry>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*cmap).uvs).hh.tbl).tail {
                (*(*(*cmap).uvs).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                (*cmap).uvs = (*_hd_hh_del).next as *mut CmapUvsEntry as *mut CmapUvsEntry;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*cmap).uvs).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket =
                (*(*(*cmap).uvs).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
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
            (*(*(*cmap).uvs).hh.tbl).num_items = (*(*(*cmap).uvs).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<CmapEntry>();
        return true;
    } else {
        return false;
    };
}
pub unsafe extern "C" fn otfcc_cmapLookupUVS(
    mut cmap: *const CmapTable,
    mut c: CmapUvsKey,
) -> *mut GlyphHandle {
    let mut s: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut c as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint;
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
    _hf_hashv =
        _hf_hashv.wrapping_add(::core::mem::size_of::<CmapUvsKey>() as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 6504943376239214222;
        }
        10 => {
            current_block_50 = 6504943376239214222;
        }
        9 => {
            current_block_50 = 13407439525415555836;
        }
        8 => {
            current_block_50 = 10521088773665199643;
        }
        7 => {
            current_block_50 = 5870691524712373942;
        }
        6 => {
            current_block_50 = 11935699476900889662;
        }
        5 => {
            current_block_50 = 12213172939624008635;
        }
        4 => {
            current_block_50 = 7964364836200268519;
        }
        3 => {
            current_block_50 = 6886367987138400562;
        }
        2 => {
            current_block_50 = 8338408844382576637;
        }
        1 => {
            current_block_50 = 15891596581160801907;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        6504943376239214222 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 13407439525415555836;
        }
        _ => {}
    }
    match current_block_50 {
        13407439525415555836 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 10521088773665199643;
        }
        _ => {}
    }
    match current_block_50 {
        10521088773665199643 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 5870691524712373942;
        }
        _ => {}
    }
    match current_block_50 {
        5870691524712373942 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 11935699476900889662;
        }
        _ => {}
    }
    match current_block_50 {
        11935699476900889662 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 12213172939624008635;
        }
        _ => {}
    }
    match current_block_50 {
        12213172939624008635 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 7964364836200268519;
        }
        _ => {}
    }
    match current_block_50 {
        7964364836200268519 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 6886367987138400562;
        }
        _ => {}
    }
    match current_block_50 {
        6886367987138400562 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 8338408844382576637;
        }
        _ => {}
    }
    match current_block_50 {
        8338408844382576637 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 15891596581160801907;
        }
        _ => {}
    }
    match current_block_50 {
        15891596581160801907 => {
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
    s = ::core::ptr::null_mut::<CmapEntry>();
    if !(*cmap).uvs.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*cmap).uvs).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*cmap).uvs).hh.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                s = ((*(*(*(*cmap).uvs).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut CmapEntry
                    as *mut CmapEntry;
            } else {
                s = ::core::ptr::null_mut::<CmapEntry>();
            }
            while !s.is_null() {
                if (*s).hh.hashv == _hf_hashv
                    && (*s).hh.keylen as usize == ::core::mem::size_of::<CmapUvsKey>()
                {
                    if memcmp(
                        (*s).hh.key,
                        &raw mut c as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<CmapUvsKey>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*s).hh.hh_next.is_null() {
                    s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut CmapEntry
                        as *mut CmapEntry;
                } else {
                    s = ::core::ptr::null_mut::<CmapEntry>();
                }
            }
        }
    }
    if !s.is_null() {
        return &raw mut (*s).glyph;
    } else {
        return ::core::ptr::null_mut::<GlyphHandle>();
    };
}
pub static TABLE_I_CMAP: CmapTableElementInterface = {
    CmapTableElementInterface {
        init: Some(table_cmap_init as unsafe extern "C" fn(*mut CmapTable) -> ()),
        copy: Some(
            table_cmap_copy as unsafe extern "C" fn(*mut CmapTable, *const CmapTable) -> (),
        ),
        move_0: Some(
            table_cmap_move as unsafe extern "C" fn(*mut CmapTable, *mut CmapTable) -> (),
        ),
        dispose: Some(table_cmap_dispose as unsafe extern "C" fn(*mut CmapTable) -> ()),
        replace: Some(
            table_cmap_replace as unsafe extern "C" fn(*mut CmapTable, CmapTable) -> (),
        ),
        copyReplace: Some(
            table_cmap_copyReplace as unsafe extern "C" fn(*mut CmapTable, CmapTable) -> (),
        ),
        create: Some(table_cmap_create),
        free: Some(table_cmap_free as unsafe extern "C" fn(*mut CmapTable) -> ()),
        encodeByIndex: Some(
            otfcc_encodeCmapByIndex
                as unsafe extern "C" fn(*mut CmapTable, ::core::ffi::c_int, u16) -> bool,
        ),
        encodeByName: Some(
            otfcc_encodeCmapByName
                as unsafe extern "C" fn(*mut CmapTable, ::core::ffi::c_int, SdsRaw) -> bool,
        ),
        unmap: Some(
            otfcc_unmapCmap as unsafe extern "C" fn(*mut CmapTable, ::core::ffi::c_int) -> bool,
        ),
        lookup: Some(
            otfcc_cmapLookup
                as unsafe extern "C" fn(
                    *const CmapTable,
                    ::core::ffi::c_int,
                ) -> *mut GlyphHandle,
        ),
        encodeUVSByIndex: Some(
            otfcc_encodeCmapUVSByIndex
                as unsafe extern "C" fn(*mut CmapTable, CmapUvsKey, u16) -> bool,
        ),
        encodeUVSByName: Some(
            otfcc_encodeCmapUVSByName
                as unsafe extern "C" fn(*mut CmapTable, CmapUvsKey, SdsRaw) -> bool,
        ),
        unmapUVS: Some(
            otfcc_unmapCmapUVS as unsafe extern "C" fn(*mut CmapTable, CmapUvsKey) -> bool,
        ),
        lookupUVS: Some(
            otfcc_cmapLookupUVS
                as unsafe extern "C" fn(*const CmapTable, CmapUvsKey) -> *mut GlyphHandle,
        ),
    }
};
unsafe extern "C" fn readFormat12(
    mut start: FontFilePointer,
    mut lengthLimit: u32,
    mut cmap: *mut CmapTable,
) {
    if lengthLimit < 16 as u32 {
        return;
    }
    let mut nGroups: u32 =
        read_32u(start.offset(12 as ::core::ffi::c_int as isize) as *const u8);
    if lengthLimit < (16 as u32).wrapping_add((12 as u32).wrapping_mul(nGroups)) {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < nGroups {
        let mut startCode: u32 = read_32u(
            start
                .offset(16 as ::core::ffi::c_int as isize)
                .offset((12 as u32).wrapping_mul(j) as isize) as *const u8,
        );
        let mut endCode: u32 = read_32u(
            start
                .offset(16 as ::core::ffi::c_int as isize)
                .offset((12 as u32).wrapping_mul(j) as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        );
        let mut startGID: u32 = read_32u(
            start
                .offset(16 as ::core::ffi::c_int as isize)
                .offset((12 as u32).wrapping_mul(j) as isize)
                .offset(8 as ::core::ffi::c_int as isize) as *const u8,
        );
        let mut c: u32 = startCode;
        while c <= endCode {
            otfcc_encodeCmapByIndex(
                cmap,
                c as ::core::ffi::c_int,
                c.wrapping_sub(startCode).wrapping_add(startGID) as u16,
            );
            c = c.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn readFormat4(
    mut start: FontFilePointer,
    mut lengthLimit: u32,
    mut cmap: *mut CmapTable,
) {
    if lengthLimit < 14 as u32 {
        return;
    }
    let mut segmentsCount: u16 =
        (read_16u(start.offset(6 as ::core::ffi::c_int as isize) as *const u8)
            as ::core::ffi::c_int
            / 2 as ::core::ffi::c_int) as u16;
    if lengthLimit
        < (16 as ::core::ffi::c_int + segmentsCount as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as u32
    {
        return;
    }
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < segmentsCount as ::core::ffi::c_int {
        let mut endCode: u16 = read_16u(
            start
                .offset(14 as ::core::ffi::c_int as isize)
                .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                as *const u8,
        );
        let mut startCode: u16 = read_16u(
            start
                .offset(14 as ::core::ffi::c_int as isize)
                .offset((segmentsCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                as *const u8,
        );
        let mut idDelta: i16 = read_16u(
            start
                .offset(14 as ::core::ffi::c_int as isize)
                .offset((segmentsCount as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                as *const u8,
        ) as i16;
        let mut idRangeOffsetOffset: u32 = (14 as ::core::ffi::c_int
            + segmentsCount as ::core::ffi::c_int * 6 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + j as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            as u32;
        let mut idRangeOffset: u16 =
            read_16u(start.offset(idRangeOffsetOffset as isize) as *const u8);
        if idRangeOffset as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut c: u32 = startCode as u32;
            while c < 0xffff as u32 && c <= endCode as u32 {
                let mut gid: u16 =
                    (c.wrapping_add(idDelta as u32) & 0xffff as u32) as u16;
                otfcc_encodeCmapByIndex(cmap, c as ::core::ffi::c_int, gid);
                c = c.wrapping_add(1);
            }
        } else {
            let mut c_0: u32 = startCode as u32;
            while c_0 < 0xffff as u32 && c_0 <= endCode as u32 {
                let mut glyphOffset: u32 = (idRangeOffset as u32)
                    .wrapping_add(
                        c_0.wrapping_sub(startCode as u32)
                            .wrapping_mul(2 as u32),
                    )
                    .wrapping_add(idRangeOffsetOffset);
                if !(glyphOffset.wrapping_add(2 as u32) > lengthLimit) {
                    let mut gid_0: u16 =
                        (read_16u(start.offset(glyphOffset as isize) as *const u8)
                            as ::core::ffi::c_int
                            + idDelta as ::core::ffi::c_int
                            & 0xffff as ::core::ffi::c_int) as u16;
                    otfcc_encodeCmapByIndex(cmap, c_0 as ::core::ffi::c_int, gid_0);
                }
                c_0 = c_0.wrapping_add(1);
            }
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn readUVSDefault(
    mut start: FontFilePointer,
    mut lengthLimit: u32,
    mut selector: Unicode,
    mut cmap: *mut CmapTable,
) {
    if lengthLimit < 4 as u32 {
        return;
    }
    let mut numUnicodeValueRanges: u32 = read_32u(start as *const u8);
    if lengthLimit
        < (4 as u32).wrapping_add((4 as u32).wrapping_mul(numUnicodeValueRanges))
    {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < numUnicodeValueRanges {
        let mut vsr: FontFilePointer = start
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((4 as u32).wrapping_mul(j) as isize);
        let mut startUnicodeValue: Unicode = read_24u(vsr as *const u8) as Unicode;
        let mut additionalCount: u8 =
            read_8u(vsr.offset(3 as ::core::ffi::c_int as isize) as *const u8);
        let mut u: Unicode = startUnicodeValue;
        while u <= startUnicodeValue.wrapping_add(additionalCount as Unicode) {
            let mut g: *mut GlyphHandle = TABLE_I_CMAP
                .lookup
                .expect("non-null function pointer")(
                cmap, u as ::core::ffi::c_int
            );
            if !g.is_null() {
                TABLE_I_CMAP
                    .encodeUVSByIndex
                    .expect("non-null function pointer")(
                    cmap,
                    CmapUvsKey {
                        unicode: u as u32,
                        selector: selector as u32,
                    },
                    (*g).index as u16,
                );
            }
            u = u.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn readUVSNonDefault(
    mut start: FontFilePointer,
    mut lengthLimit: u32,
    mut selector: Unicode,
    mut cmap: *mut CmapTable,
) {
    if lengthLimit < 4 as u32 {
        return;
    }
    let mut numUVSMappings: u32 = read_32u(start as *const u8);
    if lengthLimit < (4 as u32).wrapping_add((5 as u32).wrapping_mul(numUVSMappings)) {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < numUVSMappings {
        let mut vsr: FontFilePointer = start
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((5 as u32).wrapping_mul(j) as isize);
        let mut unicodeValue: Unicode = read_24u(vsr as *const u8) as Unicode;
        let mut glyphID: GlyphId =
            read_16u(vsr.offset(3 as ::core::ffi::c_int as isize) as *const u8) as GlyphId;
        TABLE_I_CMAP
            .encodeUVSByIndex
            .expect("non-null function pointer")(
            cmap,
            CmapUvsKey {
                unicode: unicodeValue as u32,
                selector: selector as u32,
            },
            glyphID as u16,
        );
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn readFormat14(
    mut start: FontFilePointer,
    mut lengthLimit: u32,
    mut cmap: *mut CmapTable,
) {
    if lengthLimit < 10 as u32 {
        return;
    }
    let mut nGroups: u32 =
        read_32u(start.offset(6 as ::core::ffi::c_int as isize) as *const u8);
    if lengthLimit < (11 as u32).wrapping_add((11 as u32).wrapping_mul(nGroups)) {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < nGroups {
        let mut vsr: FontFilePointer = start
            .offset(10 as ::core::ffi::c_int as isize)
            .offset((11 as u32).wrapping_mul(j) as isize);
        let mut selector: Unicode = read_24u(vsr as *const u8) as Unicode;
        let mut defaultUVSOffset: u32 =
            read_32u(vsr.offset(3 as ::core::ffi::c_int as isize) as *const u8);
        let mut nonDefaultUVSOffset: u32 =
            read_32u(vsr.offset(7 as ::core::ffi::c_int as isize) as *const u8);
        if defaultUVSOffset != 0 {
            readUVSDefault(
                start.offset(defaultUVSOffset as isize),
                lengthLimit.wrapping_sub(defaultUVSOffset),
                selector,
                cmap,
            );
        }
        if nonDefaultUVSOffset != 0 {
            readUVSNonDefault(
                start.offset(nonDefaultUVSOffset as isize),
                lengthLimit.wrapping_sub(nonDefaultUVSOffset),
                selector,
                cmap,
            );
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn readCmapMappingTable(
    mut start: FontFilePointer,
    mut lengthLimit: u32,
    mut cmap: *mut CmapTable,
    mut requiredFormat: TableId,
) {
    let mut format: u16 = read_16u(start as *const u8);
    if format as ::core::ffi::c_int == requiredFormat as ::core::ffi::c_int {
        if format as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            readFormat4(start, lengthLimit, cmap);
        } else if format as ::core::ffi::c_int == 12 as ::core::ffi::c_int {
            readFormat12(start, lengthLimit, cmap);
        }
    }
}
unsafe extern "C" fn readCmapMappingTableUVS(
    mut start: FontFilePointer,
    mut lengthLimit: u32,
    mut cmap: *mut CmapTable,
) {
    let mut format: u16 = read_16u(start as *const u8);
    if format as ::core::ffi::c_int == 14 as ::core::ffi::c_int {
        readFormat14(start, lengthLimit, cmap);
    }
}
unsafe extern "C" fn by_unicode(
    mut a: *mut CmapEntry,
    mut b: *mut CmapEntry,
) -> ::core::ffi::c_int {
    return (*a).unicode - (*b).unicode;
}
unsafe extern "C" fn by_uvs_key(
    mut a: *mut CmapUvsEntry,
    mut b: *mut CmapUvsEntry,
) -> ::core::ffi::c_int {
    if (*a).key.unicode == (*b).key.unicode {
        return (*a).key.selector.wrapping_sub((*b).key.selector) as ::core::ffi::c_int;
    } else {
        return (*a).key.unicode.wrapping_sub((*b).key.unicode) as ::core::ffi::c_int;
    };
}
#[inline]
unsafe extern "C" fn isValidCmapEncoding(mut platform: u16, mut encoding: u16) -> bool {
    return platform as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && encoding as ::core::ffi::c_int == 3 as ::core::ffi::c_int
        || platform as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && encoding as ::core::ffi::c_int == 4 as ::core::ffi::c_int
        || platform as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && encoding as ::core::ffi::c_int == 5 as ::core::ffi::c_int
        || platform as ::core::ffi::c_int == 3 as ::core::ffi::c_int
            && encoding as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        || platform as ::core::ffi::c_int == 3 as ::core::ffi::c_int
            && encoding as ::core::ffi::c_int == 10 as ::core::ffi::c_int;
}
pub static FORMAT_PRIORITIES: [TableId; 3] = [
    12 as ::core::ffi::c_int as TableId,
    4 as ::core::ffi::c_int as TableId,
    0 as ::core::ffi::c_int as TableId,
];
pub unsafe extern "C" fn otfcc_readCmap(
    packet: Packet,
    mut options: *const Options,
) -> *mut CmapTable {
    let mut numTables: u16 = 0;
    let mut cmap: *mut CmapTable = ::core::ptr::null_mut::<CmapTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1668112752i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if !(length < 4 as u32) {
                        cmap = (
                            TABLE_I_CMAP.create.expect("non-null function pointer"))();
                        numTables = read_16u(
                            data.offset(2 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if !(length
                            < (4 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * numTables as ::core::ffi::c_int)
                                as u32)
                        {
                            let mut kSubtableType: usize = 0 as usize;
                            while FORMAT_PRIORITIES[kSubtableType] != 0 {
                                let mut j: u16 = 0 as u16;
                                while (j as ::core::ffi::c_int) < numTables as ::core::ffi::c_int {
                                    let mut platform: u16 = read_16u(
                                        data.offset(4 as ::core::ffi::c_int as isize).offset(
                                            (8 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                    );
                                    let mut encoding: u16 = read_16u(
                                        data.offset(4 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (8 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8,
                                    );
                                    if isValidCmapEncoding(platform, encoding) {
                                        let mut tableOffset: u32 = read_32u(
                                            data.offset(4 as ::core::ffi::c_int as isize)
                                                .offset(
                                                    (8 as ::core::ffi::c_int
                                                        * j as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                .offset(4 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        );
                                        readCmapMappingTable(
                                            data.offset(tableOffset as isize),
                                            length.wrapping_sub(tableOffset),
                                            cmap,
                                            FORMAT_PRIORITIES[kSubtableType],
                                        );
                                    }
                                    j = j.wrapping_add(1);
                                }
                                kSubtableType = kSubtableType.wrapping_add(1);
                            }
                            let mut _hs_i: ::core::ffi::c_uint = 0;
                            let mut _hs_looping: ::core::ffi::c_uint = 0;
                            let mut _hs_nmerges: ::core::ffi::c_uint = 0;
                            let mut _hs_insize: ::core::ffi::c_uint = 0;
                            let mut _hs_psize: ::core::ffi::c_uint = 0;
                            let mut _hs_qsize: ::core::ffi::c_uint = 0;
                            let mut _hs_p: *mut UtHashHandle =
                                ::core::ptr::null_mut::<UtHashHandle>();
                            let mut _hs_q: *mut UtHashHandle =
                                ::core::ptr::null_mut::<UtHashHandle>();
                            let mut _hs_e: *mut UtHashHandle =
                                ::core::ptr::null_mut::<UtHashHandle>();
                            let mut _hs_list: *mut UtHashHandle =
                                ::core::ptr::null_mut::<UtHashHandle>();
                            let mut _hs_tail: *mut UtHashHandle =
                                ::core::ptr::null_mut::<UtHashHandle>();
                            if !(*cmap).unicodes.is_null() {
                                _hs_insize = 1 as ::core::ffi::c_uint;
                                _hs_looping = 1 as ::core::ffi::c_uint;
                                _hs_list = &raw mut (*(*cmap).unicodes).hh as *mut UtHashHandle;
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
                                                ((*_hs_q).next as *mut ::core::ffi::c_char).offset(
                                                    (*(*(*cmap).unicodes).hh.tbl).hho,
                                                )
                                                    as *mut UtHashHandle
                                            } else {
                                                ::core::ptr::null_mut::<UtHashHandle>()
                                            })
                                                as *mut UtHashHandle;
                                            if _hs_q.is_null() {
                                                break;
                                            }
                                            _hs_i = _hs_i.wrapping_add(1);
                                        }
                                        _hs_qsize = _hs_insize;
                                        while _hs_psize != 0 as ::core::ffi::c_uint
                                            || _hs_qsize != 0 as ::core::ffi::c_uint
                                                && !_hs_q.is_null()
                                        {
                                            if _hs_psize == 0 as ::core::ffi::c_uint {
                                                _hs_e = _hs_q;
                                                _hs_q = (if !(*_hs_q).next.is_null() {
                                                    ((*_hs_q).next as *mut ::core::ffi::c_char)
                                                        .offset(
                                                            (*(*(*cmap).unicodes).hh.tbl).hho,
                                                        )
                                                        as *mut UtHashHandle
                                                } else {
                                                    ::core::ptr::null_mut::<UtHashHandle>()
                                                })
                                                    as *mut UtHashHandle;
                                                _hs_qsize = _hs_qsize.wrapping_sub(1);
                                            } else if _hs_qsize == 0 as ::core::ffi::c_uint
                                                || _hs_q.is_null()
                                            {
                                                _hs_e = _hs_p;
                                                if !_hs_p.is_null() {
                                                    _hs_p = (if !(*_hs_p).next.is_null() {
                                                        ((*_hs_p).next as *mut ::core::ffi::c_char)
                                                            .offset(
                                                                (*(*(*cmap).unicodes).hh.tbl).hho,
                                                            )
                                                            as *mut UtHashHandle
                                                    } else {
                                                        ::core::ptr::null_mut::<UtHashHandle>()
                                                    })
                                                        as *mut UtHashHandle;
                                                }
                                                _hs_psize = _hs_psize.wrapping_sub(1);
                                            } else if by_unicode(
                                                (_hs_p as *mut ::core::ffi::c_char).offset(
                                                    -(*(*(*cmap).unicodes).hh.tbl).hho,
                                                )
                                                    as *mut ::core::ffi::c_void
                                                    as *mut CmapEntry,
                                                (_hs_q as *mut ::core::ffi::c_char).offset(
                                                    -(*(*(*cmap).unicodes).hh.tbl).hho,
                                                )
                                                    as *mut ::core::ffi::c_void
                                                    as *mut CmapEntry,
                                            ) <= 0 as ::core::ffi::c_int
                                            {
                                                _hs_e = _hs_p;
                                                if !_hs_p.is_null() {
                                                    _hs_p = (if !(*_hs_p).next.is_null() {
                                                        ((*_hs_p).next as *mut ::core::ffi::c_char)
                                                            .offset(
                                                                (*(*(*cmap).unicodes).hh.tbl).hho,
                                                            )
                                                            as *mut UtHashHandle
                                                    } else {
                                                        ::core::ptr::null_mut::<UtHashHandle>()
                                                    })
                                                        as *mut UtHashHandle;
                                                }
                                                _hs_psize = _hs_psize.wrapping_sub(1);
                                            } else {
                                                _hs_e = _hs_q;
                                                _hs_q = (if !(*_hs_q).next.is_null() {
                                                    ((*_hs_q).next as *mut ::core::ffi::c_char)
                                                        .offset(
                                                            (*(*(*cmap).unicodes).hh.tbl).hho,
                                                        )
                                                        as *mut UtHashHandle
                                                } else {
                                                    ::core::ptr::null_mut::<UtHashHandle>()
                                                })
                                                    as *mut UtHashHandle;
                                                _hs_qsize = _hs_qsize.wrapping_sub(1);
                                            }
                                            if !_hs_tail.is_null() {
                                                (*_hs_tail).next = if !_hs_e.is_null() {
                                                    (_hs_e as *mut ::core::ffi::c_char).offset(
                                                        -(*(*(*cmap).unicodes).hh.tbl).hho,
                                                    )
                                                        as *mut ::core::ffi::c_void
                                                } else {
                                                    NULL
                                                };
                                            } else {
                                                _hs_list = _hs_e;
                                            }
                                            if !_hs_e.is_null() {
                                                (*_hs_e).prev = if !_hs_tail.is_null() {
                                                    (_hs_tail as *mut ::core::ffi::c_char).offset(
                                                        -(*(*(*cmap).unicodes).hh.tbl).hho,
                                                    )
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
                                        (*(*(*cmap).unicodes).hh.tbl).tail = _hs_tail;
                                        (*cmap).unicodes = (_hs_list as *mut ::core::ffi::c_char)
                                            .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                            as *mut CmapEntry
                                            as *mut CmapEntry;
                                    }
                                    _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
                                }
                            }
                            let mut j_0: u16 = 0 as u16;
                            while (j_0 as ::core::ffi::c_int) < numTables as ::core::ffi::c_int {
                                let mut platform_0: u16 = read_16u(
                                    data.offset(4 as ::core::ffi::c_int as isize).offset(
                                        (8 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                                );
                                let mut encoding_0: u16 = read_16u(
                                    data.offset(4 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (8 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                if isValidCmapEncoding(platform_0, encoding_0) {
                                    let mut tableOffset_0: u32 = read_32u(
                                        data.offset(4 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (8 as ::core::ffi::c_int
                                                    * j_0 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(4 as ::core::ffi::c_int as isize)
                                            as *const u8,
                                    );
                                    readCmapMappingTableUVS(
                                        data.offset(tableOffset_0 as isize),
                                        length.wrapping_sub(tableOffset_0),
                                        cmap,
                                    );
                                }
                                j_0 = j_0.wrapping_add(1);
                            }
                            let mut _hs_i_0: ::core::ffi::c_uint = 0;
                            let mut _hs_looping_0: ::core::ffi::c_uint = 0;
                            let mut _hs_nmerges_0: ::core::ffi::c_uint = 0;
                            let mut _hs_insize_0: ::core::ffi::c_uint = 0;
                            let mut _hs_psize_0: ::core::ffi::c_uint = 0;
                            let mut _hs_qsize_0: ::core::ffi::c_uint = 0;
                            let mut _hs_p_0: *mut UtHashHandle =
                                ::core::ptr::null_mut::<UtHashHandle>();
                            let mut _hs_q_0: *mut UtHashHandle =
                                ::core::ptr::null_mut::<UtHashHandle>();
                            let mut _hs_e_0: *mut UtHashHandle =
                                ::core::ptr::null_mut::<UtHashHandle>();
                            let mut _hs_list_0: *mut UtHashHandle =
                                ::core::ptr::null_mut::<UtHashHandle>();
                            let mut _hs_tail_0: *mut UtHashHandle =
                                ::core::ptr::null_mut::<UtHashHandle>();
                            if !(*cmap).uvs.is_null() {
                                _hs_insize_0 = 1 as ::core::ffi::c_uint;
                                _hs_looping_0 = 1 as ::core::ffi::c_uint;
                                _hs_list_0 = &raw mut (*(*cmap).uvs).hh as *mut UtHashHandle;
                                while _hs_looping_0 != 0 as ::core::ffi::c_uint {
                                    _hs_p_0 = _hs_list_0;
                                    _hs_list_0 = ::core::ptr::null_mut::<UtHashHandle>();
                                    _hs_tail_0 = ::core::ptr::null_mut::<UtHashHandle>();
                                    _hs_nmerges_0 = 0 as ::core::ffi::c_uint;
                                    while !_hs_p_0.is_null() {
                                        _hs_nmerges_0 = _hs_nmerges_0.wrapping_add(1);
                                        _hs_q_0 = _hs_p_0;
                                        _hs_psize_0 = 0 as ::core::ffi::c_uint;
                                        _hs_i_0 = 0 as ::core::ffi::c_uint;
                                        while _hs_i_0 < _hs_insize_0 {
                                            _hs_psize_0 = _hs_psize_0.wrapping_add(1);
                                            _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                                                ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                                    .offset((*(*(*cmap).uvs).hh.tbl).hho)
                                                    as *mut UtHashHandle
                                            } else {
                                                ::core::ptr::null_mut::<UtHashHandle>()
                                            })
                                                as *mut UtHashHandle;
                                            if _hs_q_0.is_null() {
                                                break;
                                            }
                                            _hs_i_0 = _hs_i_0.wrapping_add(1);
                                        }
                                        _hs_qsize_0 = _hs_insize_0;
                                        while _hs_psize_0 != 0 as ::core::ffi::c_uint
                                            || _hs_qsize_0 != 0 as ::core::ffi::c_uint
                                                && !_hs_q_0.is_null()
                                        {
                                            if _hs_psize_0 == 0 as ::core::ffi::c_uint {
                                                _hs_e_0 = _hs_q_0;
                                                _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                                                    ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                                        .offset(
                                                            (*(*(*cmap).uvs).hh.tbl).hho,
                                                        )
                                                        as *mut UtHashHandle
                                                } else {
                                                    ::core::ptr::null_mut::<UtHashHandle>()
                                                })
                                                    as *mut UtHashHandle;
                                                _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                                            } else if _hs_qsize_0 == 0 as ::core::ffi::c_uint
                                                || _hs_q_0.is_null()
                                            {
                                                _hs_e_0 = _hs_p_0;
                                                if !_hs_p_0.is_null() {
                                                    _hs_p_0 = (if !(*_hs_p_0).next.is_null() {
                                                        ((*_hs_p_0).next
                                                            as *mut ::core::ffi::c_char)
                                                            .offset(
                                                                (*(*(*cmap).uvs).hh.tbl).hho,
                                                            )
                                                            as *mut UtHashHandle
                                                    } else {
                                                        ::core::ptr::null_mut::<UtHashHandle>()
                                                    })
                                                        as *mut UtHashHandle;
                                                }
                                                _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                                            } else if by_uvs_key(
                                                (_hs_p_0 as *mut ::core::ffi::c_char).offset(
                                                    -(*(*(*cmap).uvs).hh.tbl).hho,
                                                )
                                                    as *mut ::core::ffi::c_void
                                                    as *mut CmapUvsEntry,
                                                (_hs_q_0 as *mut ::core::ffi::c_char).offset(
                                                    -(*(*(*cmap).uvs).hh.tbl).hho,
                                                )
                                                    as *mut ::core::ffi::c_void
                                                    as *mut CmapUvsEntry,
                                            ) <= 0 as ::core::ffi::c_int
                                            {
                                                _hs_e_0 = _hs_p_0;
                                                if !_hs_p_0.is_null() {
                                                    _hs_p_0 = (if !(*_hs_p_0).next.is_null() {
                                                        ((*_hs_p_0).next
                                                            as *mut ::core::ffi::c_char)
                                                            .offset(
                                                                (*(*(*cmap).uvs).hh.tbl).hho,
                                                            )
                                                            as *mut UtHashHandle
                                                    } else {
                                                        ::core::ptr::null_mut::<UtHashHandle>()
                                                    })
                                                        as *mut UtHashHandle;
                                                }
                                                _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                                            } else {
                                                _hs_e_0 = _hs_q_0;
                                                _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                                                    ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                                        .offset(
                                                            (*(*(*cmap).uvs).hh.tbl).hho,
                                                        )
                                                        as *mut UtHashHandle
                                                } else {
                                                    ::core::ptr::null_mut::<UtHashHandle>()
                                                })
                                                    as *mut UtHashHandle;
                                                _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                                            }
                                            if !_hs_tail_0.is_null() {
                                                (*_hs_tail_0).next = if !_hs_e_0.is_null() {
                                                    (_hs_e_0 as *mut ::core::ffi::c_char).offset(
                                                        -(*(*(*cmap).uvs).hh.tbl).hho,
                                                    )
                                                        as *mut ::core::ffi::c_void
                                                } else {
                                                    NULL
                                                };
                                            } else {
                                                _hs_list_0 = _hs_e_0;
                                            }
                                            if !_hs_e_0.is_null() {
                                                (*_hs_e_0).prev = if !_hs_tail_0.is_null() {
                                                    (_hs_tail_0 as *mut ::core::ffi::c_char).offset(
                                                        -(*(*(*cmap).uvs).hh.tbl).hho,
                                                    )
                                                        as *mut ::core::ffi::c_void
                                                } else {
                                                    NULL
                                                };
                                            }
                                            _hs_tail_0 = _hs_e_0;
                                        }
                                        _hs_p_0 = _hs_q_0;
                                    }
                                    if !_hs_tail_0.is_null() {
                                        (*_hs_tail_0).next = NULL;
                                    }
                                    if _hs_nmerges_0 <= 1 as ::core::ffi::c_uint {
                                        _hs_looping_0 = 0 as ::core::ffi::c_uint;
                                        (*(*(*cmap).uvs).hh.tbl).tail = _hs_tail_0;
                                        (*cmap).uvs = (_hs_list_0 as *mut ::core::ffi::c_char)
                                            .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                            as *mut CmapUvsEntry
                                            as *mut CmapUvsEntry;
                                    }
                                    _hs_insize_0 =
                                        _hs_insize_0.wrapping_mul(2 as ::core::ffi::c_uint);
                                }
                            }
                            return cmap;
                        }
                    }
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(sdsempty(), b"table 'cmap' corrupted.\n"),
                    );
                    if !cmap.is_null() {
                        free(cmap as *mut ::core::ffi::c_void);
                        cmap = ::core::ptr::null_mut::<CmapTable>();
                        cmap = ::core::ptr::null_mut::<CmapTable>();
                    }
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<CmapTable>();
}
pub unsafe extern "C" fn otfcc_dumpCmap(
    mut table: *const CmapTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"cmap"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        if !(*table).unicodes.is_null() {
            let mut cmap: *mut JsonValue = json_object_new(
                (if !(*table).unicodes.is_null() {
                    (*(*(*table).unicodes).hh.tbl).num_items
                } else {
                    0 as ::core::ffi::c_uint
                }) as usize,
            );
            let mut item: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
            item = (*table).unicodes;
            while !item.is_null() {
                if !(*item).glyph.name.is_null() {
                    let mut key: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    if (*options).decimal_cmap {
                        key = sdsfromlonglong((*item).unicode as ::core::ffi::c_longlong);
                    } else {
                        key = crate::sdsbuild!(sdsempty(), b"U+", Hex4Upper(((*item).unicode) as u32));
                    }
                    json_object_push(
                        cmap,
                        key as *const ::core::ffi::c_char,
                        json_string_new_length(
                            sdslen((*item).glyph.name) as ::core::ffi::c_uint,
                            (*item).glyph.name as *const ::core::ffi::c_char,
                        ),
                    );
                    sdsfree(key);
                }
                item = (*item).hh.next as *mut CmapEntry;
            }
            json_object_push(
                root,
                b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
                cmap,
            );
        }
        if !(*table).uvs.is_null() {
            let mut uvs: *mut JsonValue = json_object_new(
                (if !(*table).uvs.is_null() {
                    (*(*(*table).uvs).hh.tbl).num_items
                } else {
                    0 as ::core::ffi::c_uint
                }) as usize,
            );
            let mut item_0: *mut CmapUvsEntry = ::core::ptr::null_mut::<CmapUvsEntry>();
            item_0 = (*table).uvs;
            while !item_0.is_null() {
                if !(*item_0).glyph.name.is_null() {
                    let mut key_0: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    if (*options).decimal_cmap {
                        key_0 = crate::sdsbuild!(
                            sdsempty(),
                            (*item_0).key.unicode,
                            b" ",
                            (*item_0).key.selector,
                        );
                    } else {
                        key_0 = crate::sdsbuild!(
                            sdsempty(),
                            b"U+",
                            Hex4Upper(((*item_0).key.unicode) as u32),
                            b" U+",
                            Hex4Upper(((*item_0).key.selector) as u32),
                        );
                    }
                    json_object_push(
                        uvs,
                        key_0 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            sdslen((*item_0).glyph.name) as ::core::ffi::c_uint,
                            (*item_0).glyph.name as *const ::core::ffi::c_char,
                        ),
                    );
                    sdsfree(key_0);
                }
                item_0 = (*item_0).hh.next as *mut CmapUvsEntry;
            }
            json_object_push(
                root,
                b"cmap_uvs\0" as *const u8 as *const ::core::ffi::c_char,
                uvs,
            );
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
#[inline]
unsafe extern "C" fn parseUnicode(unicodeStr: SdsRaw) -> Unicode {
    if sdslen(unicodeStr) > 2 as usize
        && *unicodeStr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'U' as i32
        && *unicodeStr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '+' as i32
    {
        return strtol(
            unicodeStr.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            16 as ::core::ffi::c_int,
        ) as Unicode;
    } else {
        return atoi(unicodeStr as *const ::core::ffi::c_char) as Unicode;
    };
}
unsafe extern "C" fn parseCmapUnicodes(
    mut cmap: *mut CmapTable,
    mut table: *const JsonValue,
    mut options: *const Options,
) {
    if table.is_null()
        || (*table).type_0 != JsonType::Object
    {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < (*table).u.object.length as u32 {
        let mut unicodeStr: SdsRaw = sdsnewlen(
            (*(*table).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*table).u.object.values.offset(j as isize)).name_length as usize,
        );
        let mut item: *mut JsonValue =
            (*(*table).u.object.values.offset(j as isize)).value as *mut JsonValue;
        let mut unicode: Unicode = parseUnicode(unicodeStr);
        sdsfree(unicodeStr);
        if (*item).type_0 == JsonType::String
            && unicode > 0 as Unicode
            && unicode <= 0x10ffff as Unicode
        {
            let mut gname: SdsRaw = sdsnewlen(
                (*item).u.string.ptr as *const ::core::ffi::c_void,
                (*item).u.string.length as usize,
            );
            if !otfcc_encodeCmapByName(cmap, unicode as ::core::ffi::c_int, gname) {
                let mut currentMap: *mut GlyphHandle =
                    otfcc_cmapLookup(cmap, unicode as ::core::ffi::c_int) as *mut GlyphHandle;
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"U+",
                        Hex4Upper(unicode as u32),
                        b" is already mapped to ",
                        (*currentMap).name,
                        b". Assignment to ",
                        gname,
                        b" is ignored.",
                    ),
                );
            }
        }
        j = j.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn parseUVSKey(uvsStr: SdsRaw) -> CmapUvsKey {
    let mut len: usize = sdslen(uvsStr);
    let mut k: CmapUvsKey = CmapUvsKey {
        unicode: 0 as u32,
        selector: 0 as u32,
    };
    let mut scan: SdsRaw = uvsStr;
    while scan < uvsStr.offset(len as isize) {
        if *scan as ::core::ffi::c_int == ' ' as i32 {
            k.unicode = parseUnicode(uvsStr) as u32;
            k.selector = parseUnicode(scan.offset(1 as ::core::ffi::c_int as isize)) as u32;
            return k;
        }
        scan = scan.offset(1);
    }
    return k;
}
unsafe extern "C" fn parseCmapUVS(
    mut cmap: *mut CmapTable,
    mut table: *const JsonValue,
    mut options: *const Options,
) {
    if table.is_null()
        || (*table).type_0 != JsonType::Object
    {
        return;
    }
    let mut j: u32 = 0 as u32;
    while j < (*table).u.object.length as u32 {
        let mut uvsStr: SdsRaw = sdsnewlen(
            (*(*table).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*table).u.object.values.offset(j as isize)).name_length as usize,
        );
        let mut k: CmapUvsKey = parseUVSKey(uvsStr);
        let mut item: *mut JsonValue =
            (*(*table).u.object.values.offset(j as isize)).value as *mut JsonValue;
        if (*item).type_0 == JsonType::String
            && k.unicode > 0 as u32
            && k.unicode <= 0x10ffff as u32
            && k.selector > 0 as u32
            && k.selector <= 0x10ffff as u32
        {
            let mut gname: SdsRaw = sdsnewlen(
                (*item).u.string.ptr as *const ::core::ffi::c_void,
                (*item).u.string.length as usize,
            );
            if !otfcc_encodeCmapUVSByName(cmap, k, gname) {
                let mut currentMap: *mut GlyphHandle =
                    otfcc_cmapLookupUVS(cmap, k) as *mut GlyphHandle;
                (*(*options).logger)
                    .logSDS
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"UVS U+",
                        Hex4Upper((k.unicode) as u32),
                        b" U+",
                        Hex4Upper((k.selector) as u32),
                        b" is already mapped to ",
                        (*currentMap).name,
                        b". Assignment to ",
                        gname,
                        b" is ignored.",
                    ),
                );
            }
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe extern "C" fn otfcc_parseCmap(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut CmapTable {
    if (*root).type_0 != JsonType::Object
    {
        return ::core::ptr::null_mut::<CmapTable>();
    }
    let mut cmap: *mut CmapTable = (
        TABLE_I_CMAP.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"cmap"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        parseCmapUnicodes(
            cmap,
            json_obj_get_type(
                root,
                b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Object,
            ),
            options,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"cmap_uvs"),
    );
    let mut ___loggedstep_v_0: bool = true;
    while ___loggedstep_v_0 {
        parseCmapUVS(
            cmap,
            json_obj_get_type(
                root,
                b"cmap_uvs\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Object,
            ),
            options,
        );
        ___loggedstep_v_0 = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
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
    if !(*cmap).unicodes.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*(*cmap).unicodes).hh as *mut UtHashHandle;
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
                            .offset((*(*(*cmap).unicodes).hh.tbl).hho)
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
                                .offset((*(*(*cmap).unicodes).hh.tbl).hho)
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
                                    .offset((*(*(*cmap).unicodes).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if by_unicode(
                        (_hs_p as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut CmapEntry,
                        (_hs_q as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                            as *mut ::core::ffi::c_void as *mut CmapEntry,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*(*cmap).unicodes).hh.tbl).hho)
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
                                .offset((*(*(*cmap).unicodes).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
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
                                .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
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
                (*(*(*cmap).unicodes).hh.tbl).tail = _hs_tail;
                (*cmap).unicodes = (_hs_list as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*cmap).unicodes).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut CmapEntry as *mut CmapEntry;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut _hs_i_0: ::core::ffi::c_uint = 0;
    let mut _hs_looping_0: ::core::ffi::c_uint = 0;
    let mut _hs_nmerges_0: ::core::ffi::c_uint = 0;
    let mut _hs_insize_0: ::core::ffi::c_uint = 0;
    let mut _hs_psize_0: ::core::ffi::c_uint = 0;
    let mut _hs_qsize_0: ::core::ffi::c_uint = 0;
    let mut _hs_p_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_q_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_e_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_list_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_tail_0: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    if !(*cmap).uvs.is_null() {
        _hs_insize_0 = 1 as ::core::ffi::c_uint;
        _hs_looping_0 = 1 as ::core::ffi::c_uint;
        _hs_list_0 = &raw mut (*(*cmap).uvs).hh as *mut UtHashHandle;
        while _hs_looping_0 != 0 as ::core::ffi::c_uint {
            _hs_p_0 = _hs_list_0;
            _hs_list_0 = ::core::ptr::null_mut::<UtHashHandle>();
            _hs_tail_0 = ::core::ptr::null_mut::<UtHashHandle>();
            _hs_nmerges_0 = 0 as ::core::ffi::c_uint;
            while !_hs_p_0.is_null() {
                _hs_nmerges_0 = _hs_nmerges_0.wrapping_add(1);
                _hs_q_0 = _hs_p_0;
                _hs_psize_0 = 0 as ::core::ffi::c_uint;
                _hs_i_0 = 0 as ::core::ffi::c_uint;
                while _hs_i_0 < _hs_insize_0 {
                    _hs_psize_0 = _hs_psize_0.wrapping_add(1);
                    _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                        ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                            .offset((*(*(*cmap).uvs).hh.tbl).hho)
                            as *mut UtHashHandle
                    } else {
                        ::core::ptr::null_mut::<UtHashHandle>()
                    }) as *mut UtHashHandle;
                    if _hs_q_0.is_null() {
                        break;
                    }
                    _hs_i_0 = _hs_i_0.wrapping_add(1);
                }
                _hs_qsize_0 = _hs_insize_0;
                while _hs_psize_0 != 0 as ::core::ffi::c_uint
                    || _hs_qsize_0 != 0 as ::core::ffi::c_uint && !_hs_q_0.is_null()
                {
                    if _hs_psize_0 == 0 as ::core::ffi::c_uint {
                        _hs_e_0 = _hs_q_0;
                        _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                            ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                .offset((*(*(*cmap).uvs).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                    } else if _hs_qsize_0 == 0 as ::core::ffi::c_uint || _hs_q_0.is_null() {
                        _hs_e_0 = _hs_p_0;
                        if !_hs_p_0.is_null() {
                            _hs_p_0 = (if !(*_hs_p_0).next.is_null() {
                                ((*_hs_p_0).next as *mut ::core::ffi::c_char)
                                    .offset((*(*(*cmap).uvs).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                    } else if by_uvs_key(
                        (_hs_p_0 as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut CmapUvsEntry,
                        (_hs_q_0 as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut CmapUvsEntry,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e_0 = _hs_p_0;
                        if !_hs_p_0.is_null() {
                            _hs_p_0 = (if !(*_hs_p_0).next.is_null() {
                                ((*_hs_p_0).next as *mut ::core::ffi::c_char)
                                    .offset((*(*(*cmap).uvs).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize_0 = _hs_psize_0.wrapping_sub(1);
                    } else {
                        _hs_e_0 = _hs_q_0;
                        _hs_q_0 = (if !(*_hs_q_0).next.is_null() {
                            ((*_hs_q_0).next as *mut ::core::ffi::c_char)
                                .offset((*(*(*cmap).uvs).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize_0 = _hs_qsize_0.wrapping_sub(1);
                    }
                    if !_hs_tail_0.is_null() {
                        (*_hs_tail_0).next = if !_hs_e_0.is_null() {
                            (_hs_e_0 as *mut ::core::ffi::c_char)
                                .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    } else {
                        _hs_list_0 = _hs_e_0;
                    }
                    if !_hs_e_0.is_null() {
                        (*_hs_e_0).prev = if !_hs_tail_0.is_null() {
                            (_hs_tail_0 as *mut ::core::ffi::c_char)
                                .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    }
                    _hs_tail_0 = _hs_e_0;
                }
                _hs_p_0 = _hs_q_0;
            }
            if !_hs_tail_0.is_null() {
                (*_hs_tail_0).next = NULL;
            }
            if _hs_nmerges_0 <= 1 as ::core::ffi::c_uint {
                _hs_looping_0 = 0 as ::core::ffi::c_uint;
                (*(*(*cmap).uvs).hh.tbl).tail = _hs_tail_0;
                (*cmap).uvs = (_hs_list_0 as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*cmap).uvs).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut CmapUvsEntry
                    as *mut CmapUvsEntry;
            }
            _hs_insize_0 = _hs_insize_0.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    return cmap;
}
unsafe extern "C" fn otfcc_buildCmap_format4(mut cmap: *const CmapTable) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
    let mut endCount: *mut Buffer = bufnew();
    let mut startCount: *mut Buffer = bufnew();
    let mut idDelta: *mut Buffer = bufnew();
    let mut idRangeOffset: *mut Buffer = bufnew();
    let mut glyphIdArray: *mut Buffer = bufnew();
    let mut started: bool = false;
    let mut lastUnicodeStart: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut lastUnicodeEnd: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut lastGIDStart: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut lastGIDEnd: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut lastGlyphIdArrayOffset: usize = 0 as usize;
    let mut isSequencial: bool = true;
    let mut segmentsCount: u16 = 0 as u16;
    let mut item: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    item = (*cmap).unicodes;
    while !item.is_null() {
        if (*item).unicode <= 0xffff as ::core::ffi::c_int {
            if !started {
                started = true;
                lastUnicodeEnd = (*item).unicode;
                lastUnicodeStart = lastUnicodeEnd;
                lastGIDEnd = (*item).glyph.index as ::core::ffi::c_int;
                lastGIDStart = lastGIDEnd;
                isSequencial = true;
            } else if (*item).unicode == lastUnicodeEnd + 1 as ::core::ffi::c_int
                && !((*item).glyph.index as ::core::ffi::c_int
                    != lastGIDEnd + 1 as ::core::ffi::c_int
                    && isSequencial as ::core::ffi::c_int != 0
                    && lastGIDEnd - lastGIDStart >= 4 as ::core::ffi::c_int)
            {
                if isSequencial as ::core::ffi::c_int != 0
                    && !((*item).glyph.index as ::core::ffi::c_int
                        == lastGIDEnd + 1 as ::core::ffi::c_int)
                {
                    lastGlyphIdArrayOffset = (*glyphIdArray).cursor;
                    let mut j: ::core::ffi::c_int = lastGIDStart;
                    while j <= lastGIDEnd {
                        bufwrite16b(glyphIdArray, j as u16);
                        j += 1;
                    }
                }
                lastUnicodeEnd = (*item).unicode;
                isSequencial = isSequencial as ::core::ffi::c_int != 0
                    && (*item).glyph.index as ::core::ffi::c_int
                        == lastGIDEnd + 1 as ::core::ffi::c_int;
                lastGIDEnd = (*item).glyph.index as ::core::ffi::c_int;
                if !isSequencial {
                    bufwrite16b(glyphIdArray, lastGIDEnd as u16);
                }
            } else {
                bufwrite16b(endCount, lastUnicodeEnd as u16);
                bufwrite16b(startCount, lastUnicodeStart as u16);
                if isSequencial {
                    bufwrite16b(idDelta, (lastGIDStart - lastUnicodeStart) as u16);
                    bufwrite16b(idRangeOffset, 0 as u16);
                } else {
                    bufwrite16b(idDelta, 0 as u16);
                    bufwrite16b(
                        idRangeOffset,
                        lastGlyphIdArrayOffset.wrapping_add(1 as usize) as u16,
                    );
                }
                segmentsCount =
                    (segmentsCount as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
                lastUnicodeEnd = (*item).unicode;
                lastUnicodeStart = lastUnicodeEnd;
                lastGIDEnd = (*item).glyph.index as ::core::ffi::c_int;
                lastGIDStart = lastGIDEnd;
                isSequencial = true;
            }
        }
        item = (*item).hh.next as *mut CmapEntry;
    }
    bufwrite16b(endCount, lastUnicodeEnd as u16);
    bufwrite16b(startCount, lastUnicodeStart as u16);
    if isSequencial {
        bufwrite16b(idDelta, (lastGIDStart - lastUnicodeStart) as u16);
        bufwrite16b(idRangeOffset, 0 as u16);
    } else {
        bufwrite16b(idDelta, 0 as u16);
        bufwrite16b(
            idRangeOffset,
            lastGlyphIdArrayOffset.wrapping_add(1 as usize) as u16,
        );
    }
    segmentsCount = (segmentsCount as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
    if lastGIDEnd < 0xffff as ::core::ffi::c_int {
        bufwrite16b(endCount, 0xffff as u16);
        bufwrite16b(startCount, 0xffff as u16);
        bufwrite16b(idDelta, 1 as u16);
        bufwrite16b(idRangeOffset, 0 as u16);
        segmentsCount = (segmentsCount as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
    }
    let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j_0 < segmentsCount as ::core::ffi::c_int {
        let mut ro: u16 = read_16u(
            (*idRangeOffset)
                .data
                .offset((j_0 * 2 as ::core::ffi::c_int) as isize),
        );
        if ro != 0 {
            ro = (ro as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u16;
            ro = (ro as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * (segmentsCount as ::core::ffi::c_int - j_0))
                as u16;
            bufseek(idRangeOffset, (2 as ::core::ffi::c_int * j_0) as usize);
            bufwrite16b(idRangeOffset, ro);
        }
        j_0 += 1;
    }
    bufwrite16b(buf, 4 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(
        buf,
        ((segmentsCount as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as u16,
    );
    let mut i: u32 = 0;
    let mut j_1: u32 = 0;
    j_1 = 0 as u32;
    i = 1 as u32;
    while i <= segmentsCount as u32 {
        i <<= 1 as ::core::ffi::c_int;
        j_1 = j_1.wrapping_add(1);
    }
    bufwrite16b(buf, i as u16);
    bufwrite16b(buf, j_1.wrapping_sub(1 as u32) as u16);
    bufwrite16b(
        buf,
        ((2 as ::core::ffi::c_int * segmentsCount as ::core::ffi::c_int) as u32)
            .wrapping_sub(i) as u16,
    );
    bufwrite_buf(buf, endCount);
    bufwrite16b(buf, 0 as u16);
    bufwrite_buf(buf, startCount);
    bufwrite_buf(buf, idDelta);
    bufwrite_buf(buf, idRangeOffset);
    bufwrite_buf(buf, glyphIdArray);
    bufseek(buf, 2 as usize);
    bufwrite16b(buf, buflen(buf) as u16);
    buffree(endCount);
    buffree(startCount);
    buffree(idDelta);
    buffree(idRangeOffset);
    buffree(glyphIdArray);
    return buf;
}
unsafe extern "C" fn otfcc_tryBuildCmap_format4(mut cmap: *const CmapTable) -> *mut Buffer {
    let mut buf: *mut Buffer = otfcc_buildCmap_format4(cmap);
    if buflen(buf) > UINT16_MAX as usize {
        buffree(buf);
        return ::core::ptr::null_mut::<Buffer>();
    } else {
        return buf;
    };
}
unsafe extern "C" fn otfcc_buildCmap_format12(mut cmap: *const CmapTable) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 12 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite32b(buf, 0 as u32);
    bufwrite32b(buf, 0 as u32);
    bufwrite32b(buf, 0 as u32);
    let mut nGroups: u32 = 0 as u32;
    let mut started: bool = false;
    let mut lastUnicodeStart: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut lastUnicodeEnd: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut lastGIDStart: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut lastGIDEnd: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
    let mut item: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    item = (*cmap).unicodes;
    while !item.is_null() {
        if !started {
            started = true;
            lastUnicodeEnd = (*item).unicode;
            lastUnicodeStart = lastUnicodeEnd;
            lastGIDEnd = (*item).glyph.index as ::core::ffi::c_int;
            lastGIDStart = lastGIDEnd;
        } else if (*item).unicode == lastUnicodeEnd + 1 as ::core::ffi::c_int
            && (*item).glyph.index as ::core::ffi::c_int == lastGIDEnd + 1 as ::core::ffi::c_int
        {
            lastUnicodeEnd = (*item).unicode;
            lastGIDEnd = (*item).glyph.index as ::core::ffi::c_int;
        } else {
            bufwrite32b(buf, lastUnicodeStart as u32);
            bufwrite32b(buf, lastUnicodeEnd as u32);
            bufwrite32b(buf, lastGIDStart as u32);
            nGroups = nGroups.wrapping_add(1 as u32);
            lastUnicodeEnd = (*item).unicode;
            lastUnicodeStart = lastUnicodeEnd;
            lastGIDEnd = (*item).glyph.index as ::core::ffi::c_int;
            lastGIDStart = lastGIDEnd;
        }
        item = (*item).hh.next as *mut CmapEntry;
    }
    bufwrite32b(buf, lastUnicodeStart as u32);
    bufwrite32b(buf, lastUnicodeEnd as u32);
    bufwrite32b(buf, lastGIDStart as u32);
    nGroups = nGroups.wrapping_add(1 as u32);
    bufseek(buf, 4 as usize);
    bufwrite32b(buf, buflen(buf) as u32);
    bufseek(buf, 12 as usize);
    bufwrite32b(buf, nGroups);
    return buf;
}
pub const MAX_UNICODE: ::core::ffi::c_int = 0x110001 as ::core::ffi::c_int;
pub const HAS_DEFAULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HAS_NON_DEFAULT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn writeDefaultRange(
    mut dflt: *mut Buffer,
    mut nRanges: *mut u32,
    mut start: Unicode,
    mut end: Unicode,
) {
    while end.wrapping_sub(start) > 0xff as Unicode {
        bufwrite24b(dflt, start as u32);
        bufwrite8(dflt, 0xff as u8);
        start = start.wrapping_add(0x100 as Unicode);
        *nRanges = (*nRanges).wrapping_add(1 as u32);
    }
    bufwrite24b(dflt, start as u32);
    bufwrite8(dflt, end.wrapping_sub(start) as u8);
    *nRanges = (*nRanges).wrapping_add(1 as u32);
}
unsafe extern "C" fn buildFormat14ForSelector(
    mut cmap: *const CmapTable,
    mut selector: Unicode,
    mut dflt: *mut Buffer,
    mut nondflt: *mut Buffer,
) -> u8 {
    let mut defaults: *mut GlyphId = ::core::ptr::null_mut::<GlyphId>();
    let mut nonDefaults: *mut GlyphId = ::core::ptr::null_mut::<GlyphId>();
    defaults = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphId>() as usize)
            .wrapping_mul(0x110001 as ::core::ffi::c_int as usize),
        626 as ::core::ffi::c_ulong,
    ) as *mut GlyphId;
    nonDefaults = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphId>() as usize)
            .wrapping_mul(0x110001 as ::core::ffi::c_int as usize),
        627 as ::core::ffi::c_ulong,
    ) as *mut GlyphId;
    let mut s: Unicode = 0 as Unicode;
    while s < MAX_UNICODE as Unicode {
        *defaults.offset(s as isize) = 0xffff as GlyphId;
        *nonDefaults.offset(s as isize) = 0xffff as GlyphId;
        s = s.wrapping_add(1);
    }
    let mut item: *mut CmapUvsEntry = ::core::ptr::null_mut::<CmapUvsEntry>();
    item = (*cmap).uvs;
    while !item.is_null() {
        let mut u: Unicode = (*item).key.unicode as Unicode;
        if !((*item).key.selector != selector || u >= MAX_UNICODE as Unicode) {
            if !(*item).glyph.name.is_null() {
                let mut uvsGID: GlyphId = (*item).glyph.index;
                let mut g: *mut GlyphHandle = TABLE_I_CMAP
                    .lookup
                    .expect("non-null function pointer")(
                    cmap, u as ::core::ffi::c_int
                );
                if g.is_null() {
                    *nonDefaults.offset(u as isize) = uvsGID;
                } else if uvsGID as ::core::ffi::c_int == (*g).index as ::core::ffi::c_int {
                    *defaults.offset(u as isize) = uvsGID;
                } else {
                    *nonDefaults.offset(u as isize) = uvsGID;
                }
            }
        }
        item = (*item).hh.next as *mut CmapUvsEntry;
    }
    let ref mut fresh8 = *nonDefaults.offset(0 as ::core::ffi::c_int as isize);
    *fresh8 = 0xffff as GlyphId;
    *defaults.offset(0 as ::core::ffi::c_int as isize) = *fresh8;
    let ref mut fresh9 = *nonDefaults.offset((MAX_UNICODE - 1 as ::core::ffi::c_int) as isize);
    *fresh9 = 0xffff as GlyphId;
    *defaults.offset((MAX_UNICODE - 1 as ::core::ffi::c_int) as isize) = *fresh9;
    let mut numUnicodeValueRanges: u32 = 0 as u32;
    let mut startUnicodeValue: Unicode = 0 as Unicode;
    let mut numUVSMappings: u32 = 0 as u32;
    bufwrite32b(dflt, 0 as u32);
    bufwrite32b(nondflt, 0 as u32);
    let mut u_0: Unicode = 1 as Unicode;
    while u_0 < MAX_UNICODE as Unicode {
        if *defaults.offset(u_0 as isize) as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int
            && *defaults.offset(u_0.wrapping_sub(1 as Unicode) as isize) as ::core::ffi::c_int
                == 0xffff as ::core::ffi::c_int
        {
            startUnicodeValue = u_0;
        }
        if *defaults.offset(u_0 as isize) as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
            && *defaults.offset(u_0.wrapping_sub(1 as Unicode) as isize) as ::core::ffi::c_int
                != 0xffff as ::core::ffi::c_int
        {
            writeDefaultRange(
                dflt,
                &raw mut numUnicodeValueRanges,
                startUnicodeValue,
                u_0.wrapping_sub(1 as Unicode),
            );
        }
        if *nonDefaults.offset(u_0 as isize) as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
            bufwrite24b(nondflt, u_0 as u32);
            bufwrite16b(nondflt, *nonDefaults.offset(u_0 as isize) as u16);
            numUVSMappings = numUVSMappings.wrapping_add(1);
        }
        u_0 = u_0.wrapping_add(1);
    }
    bufseek(dflt, 0 as usize);
    bufwrite32b(dflt, numUnicodeValueRanges);
    bufseek(nondflt, 0 as usize);
    bufwrite32b(nondflt, numUVSMappings);
    free(defaults as *mut ::core::ffi::c_void);
    defaults = ::core::ptr::null_mut::<GlyphId>();
    free(nonDefaults as *mut ::core::ffi::c_void);
    nonDefaults = ::core::ptr::null_mut::<GlyphId>();
    return ((if numUnicodeValueRanges != 0 {
        HAS_DEFAULT
    } else {
        0 as ::core::ffi::c_int
    }) | (if numUVSMappings != 0 {
        HAS_NON_DEFAULT
    } else {
        0 as ::core::ffi::c_int
    })) as u8;
}
unsafe extern "C" fn otfcc_buildCmap_format14(mut cmap: *const CmapTable) -> *mut Buffer {
    let mut validSelectors: *mut bool = ::core::ptr::null_mut::<bool>();
    validSelectors = __caryll_allocate_clean(
        (::core::mem::size_of::<bool>() as usize)
            .wrapping_mul(0x110001 as ::core::ffi::c_int as usize),
        681 as ::core::ffi::c_ulong,
    ) as *mut bool;
    let mut item: *mut CmapUvsEntry = ::core::ptr::null_mut::<CmapUvsEntry>();
    item = (*cmap).uvs;
    while !item.is_null() {
        if (*item).key.selector < MAX_UNICODE as u32 {
            *validSelectors.offset((*item).key.selector as isize) = true;
        }
        item = (*item).hh.next as *mut CmapUvsEntry;
    }
    let mut nSelectors: u32 = 0 as u32;
    let mut selector: Unicode = 0 as Unicode;
    while selector < MAX_UNICODE as Unicode {
        if *validSelectors.offset(selector as isize) {
            nSelectors = nSelectors.wrapping_add(1);
        }
        selector = selector.wrapping_add(1);
    }
    let mut st: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, 14 as u32), bk_int(BkCellType::B32, 0 as u32), bk_int(BkCellType::B32, nSelectors as u32)]);
    let mut selector_0: Unicode = 0 as Unicode;
    while selector_0 < MAX_UNICODE as Unicode {
        if *validSelectors.offset(selector_0 as isize) {
            let mut dflt: *mut Buffer = bufnew();
            let mut nondflt: *mut Buffer = bufnew();
            let mut results: u8 = buildFormat14ForSelector(cmap, selector_0, dflt, nondflt);
            if results as ::core::ffi::c_int & HAS_DEFAULT == 0 {
                buffree(dflt);
                dflt = ::core::ptr::null_mut::<Buffer>();
            }
            if results as ::core::ffi::c_int & HAS_NON_DEFAULT == 0 {
                buffree(nondflt);
                nondflt = ::core::ptr::null_mut::<Buffer>();
            }
            bk_push(st, &[bk_int(BkCellType::B8, (selector_0 >> 16 as ::core::ffi::c_int & 0xff as Unicode) as u32), bk_int(BkCellType::B8, (selector_0 >> 8 as ::core::ffi::c_int & 0xff as Unicode) as u32), bk_int(BkCellType::B8, (selector_0 >> 0 as ::core::ffi::c_int & 0xff as Unicode) as u32), bk_ptr(BkCellType::P32, bk_newBlockFromBuffer(dflt)), bk_ptr(BkCellType::P32, bk_newBlockFromBuffer(nondflt))]);
        }
        selector_0 = selector_0.wrapping_add(1);
    }
    let mut buf: *mut Buffer = bk_build_Block(st);
    bufseek(buf, 2 as usize);
    bufwrite32b(buf, buflen(buf) as u32);
    return buf;
}
pub unsafe extern "C" fn otfcc_buildCmap(
    mut cmap: *const CmapTable,
    mut options: *const Options,
) -> *mut Buffer {
    if cmap.is_null() || (*cmap).unicodes.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut entry: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    let mut requiresFormat12: bool = false;
    let mut hasUVS: bool = !(*cmap).uvs.is_null()
        && (if !(*cmap).uvs.is_null() {
            (*(*(*cmap).uvs).hh.tbl).num_items
        } else {
            0 as ::core::ffi::c_uint
        }) > 0 as ::core::ffi::c_uint;
    entry = (*cmap).unicodes;
    while !entry.is_null() {
        if (*entry).unicode > 0xffff as ::core::ffi::c_int {
            requiresFormat12 = true;
        }
        entry = (*entry).hh.next as *mut CmapEntry;
    }
    let mut format4: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    if !requiresFormat12 || !(*options).stub_cmap4 {
        format4 = otfcc_tryBuildCmap_format4(cmap);
        if format4.is_null() {
            requiresFormat12 = true;
        }
    }
    let mut nTables: u8 = (if requiresFormat12 as ::core::ffi::c_int != 0 {
        4 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as u8;
    if hasUVS {
        nTables = (nTables as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
    }
    if format4.is_null() {
        format4 = bufnew();
        bufwrite16b(format4, 4 as u16);
        bufwrite16b(format4, 32 as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 4 as u16);
        bufwrite16b(format4, 4 as u16);
        bufwrite16b(format4, 1 as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 0xffff as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 0xffff as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 1 as u16);
        bufwrite16b(format4, 0 as u16);
        bufwrite16b(format4, 0 as u16);
    }
    let mut format12: *mut Buffer = otfcc_buildCmap_format12(cmap);
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, (nTables as ::core::ffi::c_int) as u32)]);
    bk_push(root, &[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, 3 as u32), bk_ptr(BkCellType::P32, bk_newBlockFromBufferCopy(format4))]);
    if requiresFormat12 {
        bk_push(root, &[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, 4 as u32), bk_ptr(BkCellType::P32, bk_newBlockFromBufferCopy(format12))]);
    }
    if hasUVS {
        let mut format14: *mut Buffer = otfcc_buildCmap_format14(cmap);
        bk_push(root, &[bk_int(BkCellType::B16, 0 as u32), bk_int(BkCellType::B16, 5 as u32), bk_ptr(BkCellType::P32, bk_newBlockFromBuffer(format14))]);
    }
    bk_push(root, &[bk_int(BkCellType::B16, 3 as u32), bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P32, bk_newBlockFromBufferCopy(format4))]);
    if requiresFormat12 {
        bk_push(root, &[bk_int(BkCellType::B16, 3 as u32), bk_int(BkCellType::B16, 10 as u32), bk_ptr(BkCellType::P32, bk_newBlockFromBufferCopy(format12))]);
    }
    buffree(format4);
    buffree(format12);
    return bk_build_Block(root);
}
