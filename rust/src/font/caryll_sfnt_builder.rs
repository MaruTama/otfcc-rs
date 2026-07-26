#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset};
unsafe extern "C" {
    fn sdsempty() -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn buflen(buf: *mut caryll_Buffer) -> usize;
    fn bufseek(buf: *mut caryll_Buffer, pos: usize);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn bufwrite_buf(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    fn buflongalign(buf: *mut caryll_Buffer);
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{log_type_progress, log_vl_progress, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::vendor::sds::{Byte, sds};
use crate::support::{NULL};
use crate::support::binio::{otfcc_EndianProbe16, otfcc_EndianProbe32};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_SFNTTableEntry {
    pub tag: ::core::ffi::c_int,
    pub length: u32,
    pub checksum: u32,
    pub buffer: *mut caryll_Buffer,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_SFNTBuilder {
    pub count: u32,
    pub header: u32,
    pub tables: *mut otfcc_SFNTTableEntry,
    pub options: *const otfcc_Options,
}
#[inline]
unsafe extern "C" fn otfcc_check_endian() -> bool {
    let mut check_union: otfcc_EndianProbe16 = otfcc_EndianProbe16 {
        i2: 1 as ::core::ffi::c_int as u16,
    };
    return check_union.i1[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn otfcc_endian_convert32(mut i: u32) -> u32 {
    if otfcc_check_endian() {
        let mut src: otfcc_EndianProbe32 = otfcc_EndianProbe32 { i1: [0; 4] };
        let mut des: otfcc_EndianProbe32 = otfcc_EndianProbe32 { i1: [0; 4] };
        src.i4 = i;
        des.i1[0 as ::core::ffi::c_int as usize] = src.i1[3 as ::core::ffi::c_int as usize];
        des.i1[1 as ::core::ffi::c_int as usize] = src.i1[2 as ::core::ffi::c_int as usize];
        des.i1[2 as ::core::ffi::c_int as usize] = src.i1[1 as ::core::ffi::c_int as usize];
        des.i1[3 as ::core::ffi::c_int as usize] = src.i1[0 as ::core::ffi::c_int as usize];
        return des.i4;
    } else {
        return i;
    };
}
unsafe extern "C" fn buf_checksum(mut buffer: *mut caryll_Buffer) -> u32 {
    let mut actualLength: u32 = buflen(buffer) as u32;
    buflongalign(buffer);
    let mut sum: u32 = 0 as u32;
    let mut start: *mut u32 = (*buffer).data as *mut u32;
    let mut end: *mut u32 = start.offset(
        ((actualLength.wrapping_add(3 as u32) & !(3 as ::core::ffi::c_int) as u32)
            as usize)
            .wrapping_div(::core::mem::size_of::<u32>()) as isize,
    );
    while start < end {
        let fresh3 = start;
        start = start.offset(1);
        sum = sum.wrapping_add(otfcc_endian_convert32(*fresh3));
    }
    return sum;
}
unsafe extern "C" fn createSegment(
    mut tag: u32,
    mut buffer: *mut caryll_Buffer,
) -> *mut otfcc_SFNTTableEntry {
    let mut table: *mut otfcc_SFNTTableEntry = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
    table = __caryll_allocate_clean(
        ::core::mem::size_of::<otfcc_SFNTTableEntry>() as usize,
        20 as ::core::ffi::c_ulong,
    ) as *mut otfcc_SFNTTableEntry;
    (*table).tag = tag as ::core::ffi::c_int;
    (*table).length = buflen(buffer) as u32;
    buflongalign(buffer);
    (*table).buffer = buffer;
    let mut sum: u32 = 0 as u32;
    let mut start: *mut u32 = (*buffer).data as *mut u32;
    let mut end: *mut u32 = start.offset(
        (((*table).length.wrapping_add(3 as u32) & !(3 as ::core::ffi::c_int) as u32)
            as usize)
            .wrapping_div(::core::mem::size_of::<u32>()) as isize,
    );
    while start < end {
        let fresh0 = start;
        start = start.offset(1);
        sum = sum.wrapping_add(otfcc_endian_convert32(*fresh0));
    }
    (*table).checksum = sum;
    return table;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_newSFNTBuilder(
    mut header: u32,
    mut options: *const otfcc_Options,
) -> *mut otfcc_SFNTBuilder {
    let mut builder: *mut otfcc_SFNTBuilder = ::core::ptr::null_mut::<otfcc_SFNTBuilder>();
    builder = __caryll_allocate_clean(
        ::core::mem::size_of::<otfcc_SFNTBuilder>() as usize,
        40 as ::core::ffi::c_ulong,
    ) as *mut otfcc_SFNTBuilder;
    (*builder).count = 0 as u32;
    (*builder).header = header;
    (*builder).tables = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
    (*builder).options = options;
    return builder;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_deleteSFNTBuilder(mut builder: *mut otfcc_SFNTBuilder) {
    if builder.is_null() {
        return;
    }
    let mut item: *mut otfcc_SFNTTableEntry = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
    let mut tmp: *mut otfcc_SFNTTableEntry = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
    item = (*builder).tables;
    tmp = (if !(*builder).tables.is_null() {
        (*(*builder).tables).hh.next
    } else {
        NULL
    }) as *mut otfcc_SFNTTableEntry as *mut otfcc_SFNTTableEntry;
    while !item.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*item).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*builder).tables).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*builder).tables).hh.tbl as *mut ::core::ffi::c_void);
            (*builder).tables = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*builder).tables).hh.tbl).tail {
                (*(*(*builder).tables).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*builder).tables).hh.tbl).hho)
                        as *mut UT_hash_handle as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*builder).tables).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh1 = (*_hd_hh_del).next;
            } else {
                (*builder).tables =
                    (*_hd_hh_del).next as *mut otfcc_SFNTTableEntry as *mut otfcc_SFNTTableEntry;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*builder).tables).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh2 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*builder).tables).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = (*(*(*builder).tables).hh.tbl)
                .buckets
                .offset(_hd_bkt as isize)
                as *mut UT_hash_bucket;
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
            (*(*(*builder).tables).hh.tbl).num_items =
                (*(*(*builder).tables).hh.tbl).num_items.wrapping_sub(1);
        }
        buffree((*item).buffer);
        free(item as *mut ::core::ffi::c_void);
        item = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
        item = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut otfcc_SFNTTableEntry
            as *mut otfcc_SFNTTableEntry;
    }
    free(builder as *mut ::core::ffi::c_void);
    builder = ::core::ptr::null_mut::<otfcc_SFNTBuilder>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_SFNTBuilder_pushTable(
    mut builder: *mut otfcc_SFNTBuilder,
    mut tag: u32,
    mut buffer: *mut caryll_Buffer,
) {
    if builder.is_null() || buffer.is_null() {
        return;
    }
    let mut item: *mut otfcc_SFNTTableEntry = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
    let mut options: *const otfcc_Options = (*builder).options;
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut tag as *const ::core::ffi::c_uchar;
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
    let mut current_block_52: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 11816678978794677779;
        }
        10 => {
            current_block_52 = 11816678978794677779;
        }
        9 => {
            current_block_52 = 4243706015880627267;
        }
        8 => {
            current_block_52 = 7621604651192204360;
        }
        7 => {
            current_block_52 = 3908957553821751928;
        }
        6 => {
            current_block_52 = 4833902535699402850;
        }
        5 => {
            current_block_52 = 1827290030907757406;
        }
        4 => {
            current_block_52 = 5504193643923593725;
        }
        3 => {
            current_block_52 = 6556730086778316492;
        }
        2 => {
            current_block_52 = 13661682152946820046;
        }
        1 => {
            current_block_52 = 12051365830982011504;
        }
        _ => {
            current_block_52 = 12997042908615822766;
        }
    }
    match current_block_52 {
        11816678978794677779 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 4243706015880627267;
        }
        _ => {}
    }
    match current_block_52 {
        4243706015880627267 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 7621604651192204360;
        }
        _ => {}
    }
    match current_block_52 {
        7621604651192204360 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 3908957553821751928;
        }
        _ => {}
    }
    match current_block_52 {
        3908957553821751928 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 4833902535699402850;
        }
        _ => {}
    }
    match current_block_52 {
        4833902535699402850 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 1827290030907757406;
        }
        _ => {}
    }
    match current_block_52 {
        1827290030907757406 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_52 = 5504193643923593725;
        }
        _ => {}
    }
    match current_block_52 {
        5504193643923593725 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 6556730086778316492;
        }
        _ => {}
    }
    match current_block_52 {
        6556730086778316492 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 13661682152946820046;
        }
        _ => {}
    }
    match current_block_52 {
        13661682152946820046 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 12051365830982011504;
        }
        _ => {}
    }
    match current_block_52 {
        12051365830982011504 => {
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
    item = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
    if !(*builder).tables.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*builder).tables).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*builder).tables).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                item = ((*(*(*(*builder).tables).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*builder).tables).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut otfcc_SFNTTableEntry
                    as *mut otfcc_SFNTTableEntry;
            } else {
                item = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
            }
            while !item.is_null() {
                if (*item).hh.hashv == _hf_hashv
                    && (*item).hh.keylen as usize
                        == ::core::mem::size_of::<::core::ffi::c_int>()
                {
                    if memcmp(
                        (*item).hh.key,
                        &raw mut tag as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*item).hh.hh_next.is_null() {
                    item = ((*item).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*builder).tables).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut otfcc_SFNTTableEntry
                        as *mut otfcc_SFNTTableEntry;
                } else {
                    item = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
                }
            }
        }
    }
    if item.is_null() {
        item = createSegment(tag, buffer);
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            &raw mut (*item).tag as *const ::core::ffi::c_uchar;
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
                current_block_167 = 13938925147629323197;
            }
            10 => {
                current_block_167 = 13938925147629323197;
            }
            9 => {
                current_block_167 = 14450390747582745672;
            }
            8 => {
                current_block_167 = 3592265407673908041;
            }
            7 => {
                current_block_167 = 4513808746663773291;
            }
            6 => {
                current_block_167 = 17451296227299785165;
            }
            5 => {
                current_block_167 = 9736524532524653554;
            }
            4 => {
                current_block_167 = 72977384475979312;
            }
            3 => {
                current_block_167 = 10268651838987659859;
            }
            2 => {
                current_block_167 = 5964095679288439187;
            }
            1 => {
                current_block_167 = 18267297637889893186;
            }
            _ => {
                current_block_167 = 17394276730598727748;
            }
        }
        match current_block_167 {
            13938925147629323197 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 14450390747582745672;
            }
            _ => {}
        }
        match current_block_167 {
            14450390747582745672 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 3592265407673908041;
            }
            _ => {}
        }
        match current_block_167 {
            3592265407673908041 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 4513808746663773291;
            }
            _ => {}
        }
        match current_block_167 {
            4513808746663773291 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 17451296227299785165;
            }
            _ => {}
        }
        match current_block_167 {
            17451296227299785165 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 9736524532524653554;
            }
            _ => {}
        }
        match current_block_167 {
            9736524532524653554 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_167 = 72977384475979312;
            }
            _ => {}
        }
        match current_block_167 {
            72977384475979312 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_167 = 10268651838987659859;
            }
            _ => {}
        }
        match current_block_167 {
            10268651838987659859 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_167 = 5964095679288439187;
            }
            _ => {}
        }
        match current_block_167 {
            5964095679288439187 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_167 = 18267297637889893186;
            }
            _ => {}
        }
        match current_block_167 {
            18267297637889893186 => {
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
        (*item).hh.hashv = _ha_hashv;
        (*item).hh.key =
            &raw mut (*item).tag as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*item).hh.keylen = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
        if (*builder).tables.is_null() {
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
                    as ::core::ffi::c_long as isize;
                (*(*item).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
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
            (*builder).tables = item;
        } else {
            (*item).hh.tbl = (*(*builder).tables).hh.tbl;
            (*item).hh.next = NULL;
            (*item).hh.prev = ((*(*(*builder).tables).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*builder).tables).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*builder).tables).hh.tbl).tail).next = item as *mut ::core::ffi::c_void;
            (*(*(*builder).tables).hh.tbl).tail = &raw mut (*item).hh as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*builder).tables).hh.tbl).num_items =
            (*(*(*builder).tables).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*builder).tables).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = (*(*(*builder).tables).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UT_hash_bucket;
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
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_progress,
            log_type_progress,
            crate::sdsbuild!(
                sdsempty(),
                b"OpenType table ",
                Byte((tag >> 24 as ::core::ffi::c_int & 0xff as u32) as u8),
                Byte((tag >> 16 as ::core::ffi::c_int & 0xff as u32) as u8),
                Byte((tag >> 8 as ::core::ffi::c_int & 0xff as u32) as u8),
                Byte((tag & 0xff as u32) as u8),
                b" successfully built.\n",
            ),
        );
    } else {
        buffree(buffer);
    };
}
unsafe extern "C" fn byTag(
    mut a: *mut otfcc_SFNTTableEntry,
    mut b: *mut otfcc_SFNTTableEntry,
) -> ::core::ffi::c_int {
    return (*a).tag - (*b).tag;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_SFNTBuilder_serialize(
    mut builder: *mut otfcc_SFNTBuilder,
) -> *mut caryll_Buffer {
    let mut buffer: *mut caryll_Buffer = bufnew();
    if builder.is_null() {
        return buffer;
    }
    let mut nTables: u16 = (if !(*builder).tables.is_null() {
        (*(*(*builder).tables).hh.tbl).num_items
    } else {
        0 as ::core::ffi::c_uint
    }) as u16;
    let mut searchRange: u16 = ((if (nTables as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
    {
        8 as ::core::ffi::c_int
    } else {
        if (nTables as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            16 as ::core::ffi::c_int
        } else {
            if (nTables as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
                32 as ::core::ffi::c_int
            } else {
                64 as ::core::ffi::c_int
            }
        }
    }) * 16 as ::core::ffi::c_int) as u16;
    bufwrite32b(buffer, (*builder).header);
    bufwrite16b(buffer, nTables);
    bufwrite16b(buffer, searchRange);
    bufwrite16b(
        buffer,
        (if (nTables as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            3 as ::core::ffi::c_int
        } else if (nTables as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            4 as ::core::ffi::c_int
        } else if (nTables as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            5 as ::core::ffi::c_int
        } else {
            6 as ::core::ffi::c_int
        }) as u16,
    );
    bufwrite16b(
        buffer,
        (nTables as ::core::ffi::c_int * 16 as ::core::ffi::c_int
            - searchRange as ::core::ffi::c_int) as u16,
    );
    let mut table: *mut otfcc_SFNTTableEntry = ::core::ptr::null_mut::<otfcc_SFNTTableEntry>();
    let mut offset: usize = (12 as ::core::ffi::c_int
        + nTables as ::core::ffi::c_int * 16 as ::core::ffi::c_int)
        as usize;
    let mut headOffset: usize = offset;
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
    if !(*builder).tables.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*(*builder).tables).hh as *mut UT_hash_handle;
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
                            .offset((*(*(*builder).tables).hh.tbl).hho)
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
                                .offset((*(*(*builder).tables).hh.tbl).hho)
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
                                    .offset((*(*(*builder).tables).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if byTag(
                        (_hs_p as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*builder).tables).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut otfcc_SFNTTableEntry,
                        (_hs_q as *mut ::core::ffi::c_char)
                            .offset(-(*(*(*builder).tables).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut otfcc_SFNTTableEntry,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*(*builder).tables).hh.tbl).hho)
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
                                .offset((*(*(*builder).tables).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*(*builder).tables).hh.tbl).hho)
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
                                .offset(-(*(*(*builder).tables).hh.tbl).hho)
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
                (*(*(*builder).tables).hh.tbl).tail = _hs_tail;
                (*builder).tables = (_hs_list as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*builder).tables).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut otfcc_SFNTTableEntry
                    as *mut otfcc_SFNTTableEntry;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    table = (*builder).tables;
    while !table.is_null() {
        bufwrite32b(buffer, (*table).tag as u32);
        bufwrite32b(buffer, (*table).checksum);
        bufwrite32b(buffer, offset as u32);
        bufwrite32b(buffer, (*table).length);
        let mut cp: usize = (*buffer).cursor;
        bufseek(buffer, offset);
        bufwrite_buf(buffer, (*table).buffer);
        bufseek(buffer, cp);
        if (*table).tag == 1751474532i32 {
            headOffset = offset;
        }
        offset = offset.wrapping_add(buflen((*table).buffer));
        table = (*table).hh.next as *mut otfcc_SFNTTableEntry;
    }
    let mut wholeChecksum: u32 = buf_checksum(buffer);
    bufseek(buffer, headOffset.wrapping_add(8 as usize));
    bufwrite32b(buffer, (0xb1b0afba as u32).wrapping_sub(wholeChecksum));
    return buffer;
}
