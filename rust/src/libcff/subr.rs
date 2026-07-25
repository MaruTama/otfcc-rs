use libc::{exit, free, malloc, memcmp, memcpy, memset, strncmp};
extern "C" {
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn buflen(buf: *mut caryll_Buffer) -> usize;
    fn bufwrite_buf(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    static cff_iIndex: __caryll_elementinterface_cff_Index;
    fn cff_mergeCS2Int(blob: *mut caryll_Buffer, val: i32);
    fn cff_mergeCS2Operator(blob: *mut caryll_Buffer, val: i32);
    fn cff_mergeCS2Operand(blob: *mut caryll_Buffer, val: ::core::ffi::c_double);
    fn cff_mergeCS2Special(blob: *mut caryll_Buffer, val: u8);
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{log_type_progress, log_vl_progress, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};

use crate::vendor::sds::{sds};
use crate::libcff::{op_callgsubr, op_callsubr, op_endchar, op_return, type2_max_subrs, type2_subr_nesting};
use crate::libcff::cff_index::{__caryll_elementinterface_cff_Index, cff_Index};
use crate::libcff::charstring_il::{cff_CharstringIL};
use crate::support::{NULL};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __cff_SubrRule {
    pub printed: bool,
    pub numbered: bool,
    pub number: u32,
    pub height: u32,
    pub uniqueIndex: u32,
    pub cffIndex: u16,
    pub refcount: u32,
    pub effectiveLength: u32,
    pub guard: *mut cff_SubrNode,
    pub next: *mut cff_SubrRule,
}
pub type cff_SubrRule = __cff_SubrRule;
pub type cff_SubrNode = __cff_SubrNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __cff_SubrNode {
    pub prev: *mut cff_SubrNode,
    pub rule: *mut cff_SubrRule,
    pub next: *mut cff_SubrNode,
    pub terminal: *mut caryll_Buffer,
    pub hard: bool,
    pub guard: bool,
    pub last: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_SubrDiagramIndex {
    pub arity: u8,
    pub key: *mut u8,
    pub start: *mut cff_SubrNode,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_SubrGraph {
    pub root: *mut cff_SubrRule,
    pub last: *mut cff_SubrRule,
    pub diagramIndex: *mut cff_SubrDiagramIndex,
    pub totalRules: u32,
    pub totalCharStrings: u32,
    pub doSubroutinize: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_cff_SubrGraph {
    pub init: Option<unsafe extern "C" fn(*mut cff_SubrGraph) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut cff_SubrGraph, *const cff_SubrGraph) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut cff_SubrGraph, *mut cff_SubrGraph) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut cff_SubrGraph) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut cff_SubrGraph, cff_SubrGraph) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut cff_SubrGraph, cff_SubrGraph) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut cff_SubrGraph>,
    pub free: Option<unsafe extern "C" fn(*mut cff_SubrGraph) -> ()>,
}
unsafe extern "C" fn cff_new_Node() -> *mut cff_SubrNode {
    let mut n: *mut cff_SubrNode = ::core::ptr::null_mut::<cff_SubrNode>();
    n = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_SubrNode>() as usize,
        19 as ::core::ffi::c_ulong,
    ) as *mut cff_SubrNode;
    (*n).rule = ::core::ptr::null_mut::<cff_SubrRule>();
    (*n).terminal = ::core::ptr::null_mut::<caryll_Buffer>();
    (*n).guard = false;
    (*n).hard = false;
    (*n).prev = ::core::ptr::null_mut::<cff_SubrNode>();
    (*n).next = ::core::ptr::null_mut::<cff_SubrNode>();
    return n;
}
unsafe extern "C" fn cff_new_Rule() -> *mut cff_SubrRule {
    let mut r: *mut cff_SubrRule = ::core::ptr::null_mut::<cff_SubrRule>();
    r = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_SubrRule>() as usize,
        34 as ::core::ffi::c_ulong,
    ) as *mut cff_SubrRule;
    (*r).refcount = 0 as u32;
    (*r).guard = cff_new_Node();
    (*(*r).guard).prev = (*r).guard;
    (*(*r).guard).next = (*r).guard;
    (*(*r).guard).terminal = ::core::ptr::null_mut::<caryll_Buffer>();
    (*(*r).guard).guard = true;
    (*(*r).guard).rule = r;
    (*r).next = ::core::ptr::null_mut::<cff_SubrRule>();
    return r;
}
unsafe extern "C" fn initSubrGraph(mut g: *mut cff_SubrGraph) {
    (*g).root = cff_new_Rule();
    (*g).last = (*g).root;
    (*g).diagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    (*g).totalRules = 0 as u32;
    (*g).totalCharStrings = 0 as u32;
    (*g).doSubroutinize = false;
}
unsafe extern "C" fn clean_Node(mut x: *mut cff_SubrNode) {
    if !(*x).rule.is_null() {
        (*(*x).rule).refcount = (*(*x).rule).refcount.wrapping_sub(1 as u32);
    }
    (*x).rule = ::core::ptr::null_mut::<cff_SubrRule>();
    buffree((*x).terminal);
    (*x).terminal = ::core::ptr::null_mut::<caryll_Buffer>();
}
unsafe extern "C" fn delete_Node(mut x: *mut cff_SubrNode) {
    if x.is_null() {
        return;
    }
    clean_Node(x);
    free(x as *mut ::core::ffi::c_void);
    x = ::core::ptr::null_mut::<cff_SubrNode>();
}
unsafe extern "C" fn deleteFullRule(mut r: *mut cff_SubrRule) {
    if !(*r).guard.is_null() {
        let mut e: *mut cff_SubrNode = (*(*r).guard).next;
        while e != (*r).guard {
            let mut next: *mut cff_SubrNode = (*e).next;
            if !(*e).terminal.is_null() {
                buffree((*e).terminal);
            }
            free(e as *mut ::core::ffi::c_void);
            e = ::core::ptr::null_mut::<cff_SubrNode>();
            e = next;
        }
        free((*r).guard as *mut ::core::ffi::c_void);
        (*r).guard = ::core::ptr::null_mut::<cff_SubrNode>();
    }
    free(r as *mut ::core::ffi::c_void);
    r = ::core::ptr::null_mut::<cff_SubrRule>();
}
unsafe extern "C" fn disposeSubrGraph(mut g: *mut cff_SubrGraph) {
    let mut r: *mut cff_SubrRule = (*g).root;
    while !r.is_null() {
        let mut next: *mut cff_SubrRule = (*r).next;
        deleteFullRule(r);
        r = next;
    }
    let mut s: *mut cff_SubrDiagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    let mut tmp: *mut cff_SubrDiagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    s = (*g).diagramIndex;
    tmp = (if !(*g).diagramIndex.is_null() {
        (*(*g).diagramIndex).hh.next
    } else {
        NULL
    }) as *mut cff_SubrDiagramIndex as *mut cff_SubrDiagramIndex;
    while !s.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*g).diagramIndex).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*g).diagramIndex).hh.tbl as *mut ::core::ffi::c_void);
            (*g).diagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*g).diagramIndex).hh.tbl).tail {
                (*(*(*g).diagramIndex).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut UT_hash_handle as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                (*g).diagramIndex =
                    (*_hd_hh_del).next as *mut cff_SubrDiagramIndex as *mut cff_SubrDiagramIndex;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*g).diagramIndex).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = (*(*(*g).diagramIndex).hh.tbl)
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
            (*(*(*g).diagramIndex).hh.tbl).num_items =
                (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_sub(1);
        }
        free((*s).key as *mut ::core::ffi::c_void);
        (*s).key = ::core::ptr::null_mut::<u8>();
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
        s = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut cff_SubrDiagramIndex
            as *mut cff_SubrDiagramIndex;
    }
}
#[no_mangle]
pub static mut cff_iSubrGraph: __caryll_elementinterface_cff_SubrGraph = {
    __caryll_elementinterface_cff_SubrGraph {
        init: Some(cff_SubrGraph_init as unsafe extern "C" fn(*mut cff_SubrGraph) -> ()),
        copy: Some(
            cff_SubrGraph_copy
                as unsafe extern "C" fn(*mut cff_SubrGraph, *const cff_SubrGraph) -> (),
        ),
        move_0: Some(
            cff_SubrGraph_move
                as unsafe extern "C" fn(*mut cff_SubrGraph, *mut cff_SubrGraph) -> (),
        ),
        dispose: Some(cff_SubrGraph_dispose as unsafe extern "C" fn(*mut cff_SubrGraph) -> ()),
        replace: Some(
            cff_SubrGraph_replace as unsafe extern "C" fn(*mut cff_SubrGraph, cff_SubrGraph) -> (),
        ),
        copyReplace: Some(
            cff_SubrGraph_copyReplace
                as unsafe extern "C" fn(*mut cff_SubrGraph, cff_SubrGraph) -> (),
        ),
        create: Some(cff_SubrGraph_create),
        free: Some(cff_SubrGraph_free as unsafe extern "C" fn(*mut cff_SubrGraph) -> ()),
    }
};
#[inline]
unsafe extern "C" fn cff_SubrGraph_free(mut x: *mut cff_SubrGraph) {
    if x.is_null() {
        return;
    }
    cff_SubrGraph_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_create() -> *mut cff_SubrGraph {
    let mut x: *mut cff_SubrGraph =
        malloc(::core::mem::size_of::<cff_SubrGraph>() as usize) as *mut cff_SubrGraph;
    cff_SubrGraph_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_copyReplace(mut dst: *mut cff_SubrGraph, src: cff_SubrGraph) {
    cff_SubrGraph_dispose(dst);
    cff_SubrGraph_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_init(mut x: *mut cff_SubrGraph) {
    initSubrGraph(x);
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_dispose(mut x: *mut cff_SubrGraph) {
    disposeSubrGraph(x);
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_copy(
    mut dst: *mut cff_SubrGraph,
    mut src: *const cff_SubrGraph,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_SubrGraph>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_move(mut dst: *mut cff_SubrGraph, mut src: *mut cff_SubrGraph) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_SubrGraph>() as usize,
    );
    cff_SubrGraph_init(src);
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_replace(mut dst: *mut cff_SubrGraph, src: cff_SubrGraph) {
    cff_SubrGraph_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_SubrGraph>() as usize,
    );
}
unsafe extern "C" fn getSingletHashKey(
    mut n: *mut cff_SubrNode,
    mut len: *mut usize,
) -> *mut u8 {
    let mut l1: usize = 0;
    if !(*n).rule.is_null() {
        l1 = ::core::mem::size_of::<u32>() as usize;
    } else {
        l1 = buflen((*n).terminal).wrapping_mul(::core::mem::size_of::<u8>() as usize);
    }
    *len = (3 as usize).wrapping_add(l1).wrapping_add(1 as usize);
    let mut key: *mut u8 = ::core::ptr::null_mut::<u8>();
    key = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul(*len),
        135 as ::core::ffi::c_ulong,
    ) as *mut u8;
    *key.offset(0 as ::core::ffi::c_int as isize) = '1' as i32 as u8;
    *key.offset(1 as ::core::ffi::c_int as isize) = (if !(*n).rule.is_null() {
        '1' as i32
    } else {
        '0' as i32
    }) as u8;
    *key.offset(2 as ::core::ffi::c_int as isize) = '0' as i32 as u8;
    *key.offset((*len).wrapping_sub(1 as usize) as isize) = 0 as u8;
    if !(*n).rule.is_null() {
        memcpy(
            key.offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            &raw mut (*(*n).rule).uniqueIndex as *const ::core::ffi::c_void,
            l1,
        );
    } else {
        memcpy(
            key.offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*(*n).terminal).data as *const ::core::ffi::c_void,
            l1,
        );
    }
    return key;
}
unsafe extern "C" fn getDoubletHashKey(
    mut n: *mut cff_SubrNode,
    mut len: *mut usize,
) -> *mut u8 {
    let mut l1: usize = 0;
    let mut l2: usize = 0;
    if !(*n).rule.is_null() {
        l1 = ::core::mem::size_of::<u32>() as usize;
    } else {
        l1 = buflen((*n).terminal).wrapping_mul(::core::mem::size_of::<u8>() as usize);
    }
    if !(*(*n).next).rule.is_null() {
        l2 = ::core::mem::size_of::<u32>() as usize;
    } else {
        l2 =
            buflen((*(*n).next).terminal).wrapping_mul(::core::mem::size_of::<u8>() as usize);
    }
    *len = (3 as usize)
        .wrapping_add(l1)
        .wrapping_add(l2)
        .wrapping_add(1 as usize);
    let mut key: *mut u8 = ::core::ptr::null_mut::<u8>();
    key = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul(*len),
        163 as ::core::ffi::c_ulong,
    ) as *mut u8;
    *key.offset(0 as ::core::ffi::c_int as isize) = '2' as i32 as u8;
    *key.offset(1 as ::core::ffi::c_int as isize) = (if !(*n).rule.is_null() {
        '1' as i32
    } else {
        '0' as i32
    }) as u8;
    *key.offset(2 as ::core::ffi::c_int as isize) = (if !(*(*n).next).rule.is_null() {
        '1' as i32
    } else {
        '0' as i32
    }) as u8;
    *key.offset((*len).wrapping_sub(1 as usize) as isize) = 0 as u8;
    if !(*n).rule.is_null() {
        memcpy(
            key.offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            &raw mut (*(*n).rule).uniqueIndex as *const ::core::ffi::c_void,
            l1,
        );
    } else {
        memcpy(
            key.offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*(*n).terminal).data as *const ::core::ffi::c_void,
            l1,
        );
    }
    if !(*(*n).next).rule.is_null() {
        memcpy(
            key.offset(3 as ::core::ffi::c_int as isize)
                .offset(l1 as isize) as *mut ::core::ffi::c_void,
            &raw mut (*(*(*n).next).rule).uniqueIndex as *const ::core::ffi::c_void,
            l2,
        );
    } else {
        memcpy(
            key.offset(3 as ::core::ffi::c_int as isize)
                .offset(l1 as isize) as *mut ::core::ffi::c_void,
            (*(*(*n).next).terminal).data as *const ::core::ffi::c_void,
            l2,
        );
    }
    return key;
}
unsafe extern "C" fn lastNodeOf(mut r: *mut cff_SubrRule) -> *mut cff_SubrNode {
    return (*(*r).guard).prev;
}
unsafe extern "C" fn copyNode(mut n: *mut cff_SubrNode) -> *mut cff_SubrNode {
    let mut m: *mut cff_SubrNode = cff_new_Node();
    if !(*n).rule.is_null() {
        (*m).rule = (*n).rule;
        (*(*m).rule).refcount = (*(*m).rule).refcount.wrapping_add(1 as u32);
    } else {
        (*m).terminal = bufnew();
        bufwrite_buf((*m).terminal, (*n).terminal);
    }
    (*m).last = (*n).last;
    return m;
}
unsafe extern "C" fn unlinkNode(mut g: *mut cff_SubrGraph, mut a: *mut cff_SubrNode) {
    if (*a).hard as ::core::ffi::c_int != 0 || (*a).guard as ::core::ffi::c_int != 0 {
        return;
    }
    let mut len: usize = 0;
    let mut key: *mut u8 = getDoubletHashKey(a, &raw mut len);
    let mut di: *mut cff_SubrDiagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = len as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(len as ::core::ffi::c_uint);
    let mut current_block_52: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 17477936505200415749;
        }
        10 => {
            current_block_52 = 17477936505200415749;
        }
        9 => {
            current_block_52 = 5832591389644720667;
        }
        8 => {
            current_block_52 = 12135727074587756205;
        }
        7 => {
            current_block_52 = 3321337894142066877;
        }
        6 => {
            current_block_52 = 18019133822719951476;
        }
        5 => {
            current_block_52 = 16916718873517062650;
        }
        4 => {
            current_block_52 = 6302033452012154459;
        }
        3 => {
            current_block_52 = 127856959443206287;
        }
        2 => {
            current_block_52 = 16073624881536712561;
        }
        1 => {
            current_block_52 = 8897246755417526358;
        }
        _ => {
            current_block_52 = 721385680381463314;
        }
    }
    match current_block_52 {
        17477936505200415749 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 5832591389644720667;
        }
        _ => {}
    }
    match current_block_52 {
        5832591389644720667 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 12135727074587756205;
        }
        _ => {}
    }
    match current_block_52 {
        12135727074587756205 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 3321337894142066877;
        }
        _ => {}
    }
    match current_block_52 {
        3321337894142066877 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 18019133822719951476;
        }
        _ => {}
    }
    match current_block_52 {
        18019133822719951476 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 16916718873517062650;
        }
        _ => {}
    }
    match current_block_52 {
        16916718873517062650 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_52 = 6302033452012154459;
        }
        _ => {}
    }
    match current_block_52 {
        6302033452012154459 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 127856959443206287;
        }
        _ => {}
    }
    match current_block_52 {
        127856959443206287 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 16073624881536712561;
        }
        _ => {}
    }
    match current_block_52 {
        16073624881536712561 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 8897246755417526358;
        }
        _ => {}
    }
    match current_block_52 {
        8897246755417526358 => {
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
    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    if !(*g).diagramIndex.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*g).diagramIndex).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                di = ((*(*(*(*g).diagramIndex).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut cff_SubrDiagramIndex
                    as *mut cff_SubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
            }
            while !di.is_null() {
                if (*di).hh.hashv == _hf_hashv && (*di).hh.keylen as usize == len {
                    if memcmp((*di).hh.key, key as *const ::core::ffi::c_void, len)
                        == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*di).hh.hh_next.is_null() {
                    di = ((*di).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut cff_SubrDiagramIndex
                        as *mut cff_SubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
                }
            }
        }
    }
    if !di.is_null() && (*di).start == a {
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*di).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*g).diagramIndex).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*g).diagramIndex).hh.tbl as *mut ::core::ffi::c_void);
            (*g).diagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*g).diagramIndex).hh.tbl).tail {
                (*(*(*g).diagramIndex).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut UT_hash_handle as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh2 = (*_hd_hh_del).next;
            } else {
                (*g).diagramIndex =
                    (*_hd_hh_del).next as *mut cff_SubrDiagramIndex as *mut cff_SubrDiagramIndex;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh3 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*g).diagramIndex).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = (*(*(*g).diagramIndex).hh.tbl)
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
            (*(*(*g).diagramIndex).hh.tbl).num_items =
                (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_sub(1);
        }
        free((*di).key as *mut ::core::ffi::c_void);
        (*di).key = ::core::ptr::null_mut::<u8>();
        free(di as *mut ::core::ffi::c_void);
        di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    }
    free(key as *mut ::core::ffi::c_void);
    key = ::core::ptr::null_mut::<u8>();
    key = getSingletHashKey(a, &raw mut len);
    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
    let mut _hj_i_0: ::core::ffi::c_uint = 0;
    let mut _hj_j_0: ::core::ffi::c_uint = 0;
    let mut _hj_k_0: ::core::ffi::c_uint = 0;
    let mut _hj_key_0: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
    _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i_0 = _hj_j_0;
    _hj_k_0 = len as ::core::ffi::c_uint;
    while _hj_k_0 >= 12 as ::core::ffi::c_uint {
        _hj_i_0 = _hj_i_0.wrapping_add(
            (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hj_j_0 = _hj_j_0.wrapping_add(
            (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hf_hashv_0 = _hf_hashv_0.wrapping_add(
            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key_0.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
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
    _hf_hashv_0 = _hf_hashv_0.wrapping_add(len as ::core::ffi::c_uint);
    let mut current_block_212: u64;
    match _hj_k_0 {
        11 => {
            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_212 = 6194918265873876893;
        }
        10 => {
            current_block_212 = 6194918265873876893;
        }
        9 => {
            current_block_212 = 11312125591604559950;
        }
        8 => {
            current_block_212 = 3938841353020120042;
        }
        7 => {
            current_block_212 = 2091748763977890046;
        }
        6 => {
            current_block_212 = 12995988452299032254;
        }
        5 => {
            current_block_212 = 17300962478708656041;
        }
        4 => {
            current_block_212 = 10186404607033281164;
        }
        3 => {
            current_block_212 = 7459417496506341382;
        }
        2 => {
            current_block_212 = 12646597411994191343;
        }
        1 => {
            current_block_212 = 10978766729946051380;
        }
        _ => {
            current_block_212 = 17648591037158480576;
        }
    }
    match current_block_212 {
        6194918265873876893 => {
            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_212 = 11312125591604559950;
        }
        _ => {}
    }
    match current_block_212 {
        11312125591604559950 => {
            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_212 = 3938841353020120042;
        }
        _ => {}
    }
    match current_block_212 {
        3938841353020120042 => {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_212 = 2091748763977890046;
        }
        _ => {}
    }
    match current_block_212 {
        2091748763977890046 => {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_212 = 12995988452299032254;
        }
        _ => {}
    }
    match current_block_212 {
        12995988452299032254 => {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_212 = 17300962478708656041;
        }
        _ => {}
    }
    match current_block_212 {
        17300962478708656041 => {
            _hj_j_0 = _hj_j_0.wrapping_add(
                *_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_212 = 10186404607033281164;
        }
        _ => {}
    }
    match current_block_212 {
        10186404607033281164 => {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_212 = 7459417496506341382;
        }
        _ => {}
    }
    match current_block_212 {
        7459417496506341382 => {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_212 = 12646597411994191343;
        }
        _ => {}
    }
    match current_block_212 {
        12646597411994191343 => {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_212 = 10978766729946051380;
        }
        _ => {}
    }
    match current_block_212 {
        10978766729946051380 => {
            _hj_i_0 = _hj_i_0.wrapping_add(
                *_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
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
    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    if !(*g).diagramIndex.is_null() {
        let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
        _hf_bkt_0 = _hf_hashv_0
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*g).diagramIndex).hh.tbl)
                .buckets
                .offset(_hf_bkt_0 as isize))
            .hh_head
            .is_null()
            {
                di = ((*(*(*(*g).diagramIndex).hh.tbl)
                    .buckets
                    .offset(_hf_bkt_0 as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut cff_SubrDiagramIndex
                    as *mut cff_SubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
            }
            while !di.is_null() {
                if (*di).hh.hashv == _hf_hashv_0 && (*di).hh.keylen as usize == len {
                    if memcmp((*di).hh.key, key as *const ::core::ffi::c_void, len)
                        == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*di).hh.hh_next.is_null() {
                    di = ((*di).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut cff_SubrDiagramIndex
                        as *mut cff_SubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
                }
            }
        }
    }
    if !di.is_null() && (*di).start == a {
        let mut _hd_hh_del_0: *mut UT_hash_handle = &raw mut (*di).hh;
        if (*_hd_hh_del_0).prev.is_null() && (*_hd_hh_del_0).next.is_null() {
            free((*(*(*g).diagramIndex).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*g).diagramIndex).hh.tbl as *mut ::core::ffi::c_void);
            (*g).diagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
        } else {
            let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
            if _hd_hh_del_0 == (*(*(*g).diagramIndex).hh.tbl).tail {
                (*(*(*g).diagramIndex).hh.tbl).tail =
                    ((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut UT_hash_handle as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del_0).prev.is_null() {
                let ref mut fresh4 = (*(((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh4 = (*_hd_hh_del_0).next;
            } else {
                (*g).diagramIndex =
                    (*_hd_hh_del_0).next as *mut cff_SubrDiagramIndex as *mut cff_SubrDiagramIndex;
            }
            if !(*_hd_hh_del_0).next.is_null() {
                let ref mut fresh5 = (*(((*_hd_hh_del_0).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh5 = (*_hd_hh_del_0).prev;
            }
            _hd_bkt_0 = (*_hd_hh_del_0).hashv
                & (*(*(*g).diagramIndex).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head_0: *mut UT_hash_bucket = (*(*(*g).diagramIndex).hh.tbl)
                .buckets
                .offset(_hd_bkt_0 as isize)
                as *mut UT_hash_bucket;
            (*_hd_head_0).count = (*_hd_head_0).count.wrapping_sub(1);
            if (*_hd_head_0).hh_head == _hd_hh_del_0 {
                (*_hd_head_0).hh_head = (*_hd_hh_del_0).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del_0).hh_prev.is_null() {
                (*(*_hd_hh_del_0).hh_prev).hh_next = (*_hd_hh_del_0).hh_next;
            }
            if !(*_hd_hh_del_0).hh_next.is_null() {
                (*(*_hd_hh_del_0).hh_next).hh_prev = (*_hd_hh_del_0).hh_prev;
            }
            (*(*(*g).diagramIndex).hh.tbl).num_items =
                (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_sub(1);
        }
        free((*di).key as *mut ::core::ffi::c_void);
        (*di).key = ::core::ptr::null_mut::<u8>();
        free(di as *mut ::core::ffi::c_void);
        di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    }
    free(key as *mut ::core::ffi::c_void);
    key = ::core::ptr::null_mut::<u8>();
}
unsafe extern "C" fn addDoublet(mut g: *mut cff_SubrGraph, mut n: *mut cff_SubrNode) {
    if n.is_null()
        || (*n).next.is_null()
        || (*n).guard as ::core::ffi::c_int != 0
        || (*n).hard as ::core::ffi::c_int != 0
        || (*(*n).next).hard as ::core::ffi::c_int != 0
        || (*(*n).next).guard as ::core::ffi::c_int != 0
    {
        return;
    }
    let mut len: usize = 0;
    let mut key: *mut u8 = getDoubletHashKey(n, &raw mut len);
    let mut di: *mut cff_SubrDiagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = len as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(len as ::core::ffi::c_uint);
    let mut current_block_52: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 4065295897985755008;
        }
        10 => {
            current_block_52 = 4065295897985755008;
        }
        9 => {
            current_block_52 = 324709762913668596;
        }
        8 => {
            current_block_52 = 9261061202561423017;
        }
        7 => {
            current_block_52 = 13929377212554857746;
        }
        6 => {
            current_block_52 = 11923795337831359737;
        }
        5 => {
            current_block_52 = 17956245827096646122;
        }
        4 => {
            current_block_52 = 17451773831962767405;
        }
        3 => {
            current_block_52 = 2555747926156542244;
        }
        2 => {
            current_block_52 = 3671894898333869379;
        }
        1 => {
            current_block_52 = 18122161107652318248;
        }
        _ => {
            current_block_52 = 721385680381463314;
        }
    }
    match current_block_52 {
        4065295897985755008 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 324709762913668596;
        }
        _ => {}
    }
    match current_block_52 {
        324709762913668596 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 9261061202561423017;
        }
        _ => {}
    }
    match current_block_52 {
        9261061202561423017 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 13929377212554857746;
        }
        _ => {}
    }
    match current_block_52 {
        13929377212554857746 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 11923795337831359737;
        }
        _ => {}
    }
    match current_block_52 {
        11923795337831359737 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 17956245827096646122;
        }
        _ => {}
    }
    match current_block_52 {
        17956245827096646122 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_52 = 17451773831962767405;
        }
        _ => {}
    }
    match current_block_52 {
        17451773831962767405 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 2555747926156542244;
        }
        _ => {}
    }
    match current_block_52 {
        2555747926156542244 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 3671894898333869379;
        }
        _ => {}
    }
    match current_block_52 {
        3671894898333869379 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 18122161107652318248;
        }
        _ => {}
    }
    match current_block_52 {
        18122161107652318248 => {
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
    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    if !(*g).diagramIndex.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*g).diagramIndex).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                di = ((*(*(*(*g).diagramIndex).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut cff_SubrDiagramIndex
                    as *mut cff_SubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
            }
            while !di.is_null() {
                if (*di).hh.hashv == _hf_hashv && (*di).hh.keylen as usize == len {
                    if memcmp((*di).hh.key, key as *const ::core::ffi::c_void, len)
                        == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*di).hh.hh_next.is_null() {
                    di = ((*di).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut cff_SubrDiagramIndex
                        as *mut cff_SubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
                }
            }
        }
    }
    if di.is_null() {
        di = __caryll_allocate_clean(
            ::core::mem::size_of::<cff_SubrDiagramIndex>() as usize,
            232 as ::core::ffi::c_ulong,
        ) as *mut cff_SubrDiagramIndex;
        (*di).arity = 2 as u8;
        (*di).key = key;
        (*di).start = n;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = len as ::core::ffi::c_uint;
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
        _ha_hashv = _ha_hashv.wrapping_add(len as ::core::ffi::c_uint);
        let mut current_block_170: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 3318815033043726393;
            }
            10 => {
                current_block_170 = 3318815033043726393;
            }
            9 => {
                current_block_170 = 9859328956803871766;
            }
            8 => {
                current_block_170 = 5339340440139523279;
            }
            7 => {
                current_block_170 = 12280221764675984621;
            }
            6 => {
                current_block_170 = 9962268526125952206;
            }
            5 => {
                current_block_170 = 11786492218901458792;
            }
            4 => {
                current_block_170 = 6478931724660870667;
            }
            3 => {
                current_block_170 = 3984539823505074296;
            }
            2 => {
                current_block_170 = 1322003779376755974;
            }
            1 => {
                current_block_170 = 12284659352054694716;
            }
            _ => {
                current_block_170 = 16937825661756021828;
            }
        }
        match current_block_170 {
            3318815033043726393 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 9859328956803871766;
            }
            _ => {}
        }
        match current_block_170 {
            9859328956803871766 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 5339340440139523279;
            }
            _ => {}
        }
        match current_block_170 {
            5339340440139523279 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 12280221764675984621;
            }
            _ => {}
        }
        match current_block_170 {
            12280221764675984621 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 9962268526125952206;
            }
            _ => {}
        }
        match current_block_170 {
            9962268526125952206 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 11786492218901458792;
            }
            _ => {}
        }
        match current_block_170 {
            11786492218901458792 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_170 = 6478931724660870667;
            }
            _ => {}
        }
        match current_block_170 {
            6478931724660870667 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 3984539823505074296;
            }
            _ => {}
        }
        match current_block_170 {
            3984539823505074296 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 1322003779376755974;
            }
            _ => {}
        }
        match current_block_170 {
            1322003779376755974 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 12284659352054694716;
            }
            _ => {}
        }
        match current_block_170 {
            12284659352054694716 => {
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
        (*di).hh.hashv = _ha_hashv;
        (*di).hh.key = key as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*di).hh.keylen = len as ::core::ffi::c_uint;
        if (*g).diagramIndex.is_null() {
            (*di).hh.next = NULL;
            (*di).hh.prev = NULL;
            (*di).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*di).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*di).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as usize,
                );
                (*(*di).hh.tbl).tail = &raw mut (*di).hh as *mut UT_hash_handle;
                (*(*di).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*di).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*di).hh.tbl).hho = (&raw mut (*di).hh as *mut ::core::ffi::c_char)
                    .offset_from(di as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*di).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                (*(*di).hh.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*di).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    );
                }
            }
            (*g).diagramIndex = di;
        } else {
            (*di).hh.tbl = (*(*g).diagramIndex).hh.tbl;
            (*di).hh.next = NULL;
            (*di).hh.prev = ((*(*(*g).diagramIndex).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*g).diagramIndex).hh.tbl).tail).next = di as *mut ::core::ffi::c_void;
            (*(*(*g).diagramIndex).hh.tbl).tail = &raw mut (*di).hh as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*g).diagramIndex).hh.tbl).num_items =
            (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = (*(*(*g).diagramIndex).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*di).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*di).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*di).hh as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*di).hh as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*di).hh.tbl).noexpand == 0
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
                    .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                );
                (*(*di).hh.tbl).ideal_chain_maxlen = ((*(*di).hh.tbl).num_items
                    >> (*(*di).hh.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*di).hh.tbl).num_items
                        & (*(*di).hh.tbl)
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
                (*(*di).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*di).hh.tbl).num_buckets {
                    _he_thh = (*(*(*di).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*di).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*di).hh.tbl).ideal_chain_maxlen {
                            (*(*di).hh.tbl).nonideal_items =
                                (*(*di).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*di).hh.tbl).ideal_chain_maxlen);
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
                free((*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*di).hh.tbl).num_buckets = (*(*di).hh.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*di).hh.tbl).log2_num_buckets = (*(*di).hh.tbl).log2_num_buckets.wrapping_add(1);
                (*(*di).hh.tbl).buckets = _he_new_buckets;
                (*(*di).hh.tbl).ineff_expands = if (*(*di).hh.tbl).nonideal_items
                    > (*(*di).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*di).hh.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*di).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*di).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
    } else {
        (*di).start = n;
        free(key as *mut ::core::ffi::c_void);
        key = ::core::ptr::null_mut::<u8>();
    };
}
unsafe extern "C" fn addSinglet(mut g: *mut cff_SubrGraph, mut n: *mut cff_SubrNode) {
    if n.is_null() || (*n).guard as ::core::ffi::c_int != 0 || (*n).hard as ::core::ffi::c_int != 0
    {
        return;
    }
    let mut len: usize = 0;
    let mut key: *mut u8 = getSingletHashKey(n, &raw mut len);
    let mut di: *mut cff_SubrDiagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = len as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(len as ::core::ffi::c_uint);
    let mut current_block_52: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 10069724166744912526;
        }
        10 => {
            current_block_52 = 10069724166744912526;
        }
        9 => {
            current_block_52 = 10770776859558409185;
        }
        8 => {
            current_block_52 = 4554551135110170831;
        }
        7 => {
            current_block_52 = 12040273246118600404;
        }
        6 => {
            current_block_52 = 5191371565135666240;
        }
        5 => {
            current_block_52 = 14036102755284481664;
        }
        4 => {
            current_block_52 = 8370305235499784343;
        }
        3 => {
            current_block_52 = 12683405533515026474;
        }
        2 => {
            current_block_52 = 14339572031004166383;
        }
        1 => {
            current_block_52 = 8485469831678063255;
        }
        _ => {
            current_block_52 = 721385680381463314;
        }
    }
    match current_block_52 {
        10069724166744912526 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 10770776859558409185;
        }
        _ => {}
    }
    match current_block_52 {
        10770776859558409185 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 4554551135110170831;
        }
        _ => {}
    }
    match current_block_52 {
        4554551135110170831 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 12040273246118600404;
        }
        _ => {}
    }
    match current_block_52 {
        12040273246118600404 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 5191371565135666240;
        }
        _ => {}
    }
    match current_block_52 {
        5191371565135666240 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 14036102755284481664;
        }
        _ => {}
    }
    match current_block_52 {
        14036102755284481664 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_52 = 8370305235499784343;
        }
        _ => {}
    }
    match current_block_52 {
        8370305235499784343 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 12683405533515026474;
        }
        _ => {}
    }
    match current_block_52 {
        12683405533515026474 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 14339572031004166383;
        }
        _ => {}
    }
    match current_block_52 {
        14339572031004166383 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 8485469831678063255;
        }
        _ => {}
    }
    match current_block_52 {
        8485469831678063255 => {
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
    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    if !(*g).diagramIndex.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*g).diagramIndex).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                di = ((*(*(*(*g).diagramIndex).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut cff_SubrDiagramIndex
                    as *mut cff_SubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
            }
            while !di.is_null() {
                if (*di).hh.hashv == _hf_hashv && (*di).hh.keylen as usize == len {
                    if memcmp((*di).hh.key, key as *const ::core::ffi::c_void, len)
                        == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*di).hh.hh_next.is_null() {
                    di = ((*di).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut cff_SubrDiagramIndex
                        as *mut cff_SubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
                }
            }
        }
    }
    if di.is_null() {
        di = __caryll_allocate_clean(
            ::core::mem::size_of::<cff_SubrDiagramIndex>() as usize,
            249 as ::core::ffi::c_ulong,
        ) as *mut cff_SubrDiagramIndex;
        (*di).arity = 1 as u8;
        (*di).key = key;
        (*di).start = n;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = len as ::core::ffi::c_uint;
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
        _ha_hashv = _ha_hashv.wrapping_add(len as ::core::ffi::c_uint);
        let mut current_block_170: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 16382130611335638154;
            }
            10 => {
                current_block_170 = 16382130611335638154;
            }
            9 => {
                current_block_170 = 14230297074336013306;
            }
            8 => {
                current_block_170 = 4181977508120650283;
            }
            7 => {
                current_block_170 = 11092747999842681840;
            }
            6 => {
                current_block_170 = 860580498617521596;
            }
            5 => {
                current_block_170 = 4950309270159869464;
            }
            4 => {
                current_block_170 = 2228698691573629089;
            }
            3 => {
                current_block_170 = 2386861183720123874;
            }
            2 => {
                current_block_170 = 12016608504014379768;
            }
            1 => {
                current_block_170 = 15550265556777550223;
            }
            _ => {
                current_block_170 = 16937825661756021828;
            }
        }
        match current_block_170 {
            16382130611335638154 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 14230297074336013306;
            }
            _ => {}
        }
        match current_block_170 {
            14230297074336013306 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 4181977508120650283;
            }
            _ => {}
        }
        match current_block_170 {
            4181977508120650283 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 11092747999842681840;
            }
            _ => {}
        }
        match current_block_170 {
            11092747999842681840 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 860580498617521596;
            }
            _ => {}
        }
        match current_block_170 {
            860580498617521596 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 4950309270159869464;
            }
            _ => {}
        }
        match current_block_170 {
            4950309270159869464 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_170 = 2228698691573629089;
            }
            _ => {}
        }
        match current_block_170 {
            2228698691573629089 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 2386861183720123874;
            }
            _ => {}
        }
        match current_block_170 {
            2386861183720123874 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 12016608504014379768;
            }
            _ => {}
        }
        match current_block_170 {
            12016608504014379768 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 15550265556777550223;
            }
            _ => {}
        }
        match current_block_170 {
            15550265556777550223 => {
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
        (*di).hh.hashv = _ha_hashv;
        (*di).hh.key = key as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*di).hh.keylen = len as ::core::ffi::c_uint;
        if (*g).diagramIndex.is_null() {
            (*di).hh.next = NULL;
            (*di).hh.prev = NULL;
            (*di).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*di).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*di).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as usize,
                );
                (*(*di).hh.tbl).tail = &raw mut (*di).hh as *mut UT_hash_handle;
                (*(*di).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*di).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*di).hh.tbl).hho = (&raw mut (*di).hh as *mut ::core::ffi::c_char)
                    .offset_from(di as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*di).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                (*(*di).hh.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*di).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    );
                }
            }
            (*g).diagramIndex = di;
        } else {
            (*di).hh.tbl = (*(*g).diagramIndex).hh.tbl;
            (*di).hh.next = NULL;
            (*di).hh.prev = ((*(*(*g).diagramIndex).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*g).diagramIndex).hh.tbl).tail).next = di as *mut ::core::ffi::c_void;
            (*(*(*g).diagramIndex).hh.tbl).tail = &raw mut (*di).hh as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*g).diagramIndex).hh.tbl).num_items =
            (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = (*(*(*g).diagramIndex).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*di).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*di).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*di).hh as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*di).hh as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*di).hh.tbl).noexpand == 0
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
                    .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                );
                (*(*di).hh.tbl).ideal_chain_maxlen = ((*(*di).hh.tbl).num_items
                    >> (*(*di).hh.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*di).hh.tbl).num_items
                        & (*(*di).hh.tbl)
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
                (*(*di).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*di).hh.tbl).num_buckets {
                    _he_thh = (*(*(*di).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*di).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*di).hh.tbl).ideal_chain_maxlen {
                            (*(*di).hh.tbl).nonideal_items =
                                (*(*di).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*di).hh.tbl).ideal_chain_maxlen);
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
                free((*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*di).hh.tbl).num_buckets = (*(*di).hh.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*di).hh.tbl).log2_num_buckets = (*(*di).hh.tbl).log2_num_buckets.wrapping_add(1);
                (*(*di).hh.tbl).buckets = _he_new_buckets;
                (*(*di).hh.tbl).ineff_expands = if (*(*di).hh.tbl).nonideal_items
                    > (*(*di).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*di).hh.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*di).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*di).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
    } else {
        (*di).start = n;
        free(key as *mut ::core::ffi::c_void);
        key = ::core::ptr::null_mut::<u8>();
    };
}
unsafe extern "C" fn identNode(mut m: *mut cff_SubrNode, mut n: *mut cff_SubrNode) -> bool {
    if !(*m).rule.is_null() {
        return (*m).rule == (*n).rule;
    } else if !(*n).rule.is_null() {
        return false;
    } else {
        return (*(*m).terminal).size == (*(*n).terminal).size
            && strncmp(
                (*(*m).terminal).data as *mut ::core::ffi::c_char,
                (*(*n).terminal).data as *mut ::core::ffi::c_char,
                (*(*m).terminal).size,
            ) == 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn joinNodes(
    mut g: *mut cff_SubrGraph,
    mut m: *mut cff_SubrNode,
    mut n: *mut cff_SubrNode,
) {
    if !(*m).next.is_null() {
        unlinkNode(g, m);
        if !(*n).prev.is_null()
            && !(*n).next.is_null()
            && identNode((*n).prev, n) as ::core::ffi::c_int != 0
            && identNode(n, (*n).next) as ::core::ffi::c_int != 0
        {
            addDoublet(g, n);
        }
        if !(*m).prev.is_null()
            && !(*m).next.is_null()
            && identNode((*m).prev, m) as ::core::ffi::c_int != 0
            && identNode(m, (*m).next) as ::core::ffi::c_int != 0
        {
            addDoublet(g, (*m).prev);
        }
    }
    (*m).next = n;
    (*n).prev = m;
}
unsafe extern "C" fn xInsertNodeAfter(
    mut g: *mut cff_SubrGraph,
    mut m: *mut cff_SubrNode,
    mut n: *mut cff_SubrNode,
) {
    joinNodes(g, n, (*m).next);
    joinNodes(g, m, n);
}
unsafe extern "C" fn removeNodeFromGraph(mut g: *mut cff_SubrGraph, mut a: *mut cff_SubrNode) {
    joinNodes(g, (*a).prev, (*a).next);
    if !(*a).guard {
        unlinkNode(g, a);
        delete_Node(a);
    }
}
unsafe extern "C" fn expandCall(mut g: *mut cff_SubrGraph, mut a: *mut cff_SubrNode) {
    let mut aprev: *mut cff_SubrNode = (*a).prev;
    let mut anext: *mut cff_SubrNode = (*a).next;
    let mut r: *mut cff_SubrRule = (*a).rule;
    let mut r1: *mut cff_SubrNode = (*(*r).guard).next;
    let mut r2: *mut cff_SubrNode = (*(*r).guard).prev;
    unlinkNode(g, a);
    joinNodes(g, aprev, r1);
    joinNodes(g, r2, anext);
    addDoublet(g, r2);
    (*(*r).guard).next = (*r).guard;
    (*(*r).guard).prev = (*(*r).guard).next;
    (*r).refcount = (*r).refcount.wrapping_sub(1 as u32);
    delete_Node(a);
}
unsafe extern "C" fn substituteDoubletWithRule(
    mut g: *mut cff_SubrGraph,
    mut m: *mut cff_SubrNode,
    mut r: *mut cff_SubrRule,
) {
    let mut prev: *mut cff_SubrNode = (*m).prev;
    removeNodeFromGraph(g, (*prev).next);
    removeNodeFromGraph(g, (*prev).next);
    let mut invoke: *mut cff_SubrNode = cff_new_Node();
    (*invoke).rule = r;
    (*(*invoke).rule).refcount = (*(*invoke).rule).refcount.wrapping_add(1 as u32);
    xInsertNodeAfter(g, prev, invoke);
    addDoublet(g, prev);
    addDoublet(g, invoke);
    addSinglet(g, invoke);
    if !checkDoubletMatch(g, prev) {
        checkDoubletMatch(g, (*prev).next);
    }
}
unsafe extern "C" fn substituteSingletWithRule(
    mut g: *mut cff_SubrGraph,
    mut m: *mut cff_SubrNode,
    mut r: *mut cff_SubrRule,
) {
    let mut prev: *mut cff_SubrNode = (*m).prev;
    removeNodeFromGraph(g, (*prev).next);
    let mut invoke: *mut cff_SubrNode = cff_new_Node();
    (*invoke).rule = r;
    (*(*invoke).rule).refcount = (*(*invoke).rule).refcount.wrapping_add(1 as u32);
    xInsertNodeAfter(g, prev, invoke);
    addDoublet(g, prev);
    addDoublet(g, invoke);
    addSinglet(g, invoke);
}
unsafe extern "C" fn processMatchDoublet(
    mut g: *mut cff_SubrGraph,
    mut m: *mut cff_SubrNode,
    mut n: *mut cff_SubrNode,
) {
    let mut rule: *mut cff_SubrRule = ::core::ptr::null_mut::<cff_SubrRule>();
    if (*(*m).prev).guard as ::core::ffi::c_int != 0
        && (*(*(*m).next).next).guard as ::core::ffi::c_int != 0
    {
        rule = (*(*m).prev).rule;
        substituteDoubletWithRule(g, n, rule);
    } else {
        rule = cff_new_Rule();
        (*rule).uniqueIndex = (*g).totalRules;
        (*g).totalRules = (*g).totalRules.wrapping_add(1 as u32);
        (*(*g).last).next = rule;
        (*g).last = rule;
        xInsertNodeAfter(g, lastNodeOf(rule), copyNode(m));
        xInsertNodeAfter(g, lastNodeOf(rule), copyNode((*m).next));
        substituteDoubletWithRule(g, m, rule);
        substituteDoubletWithRule(g, n, rule);
        addDoublet(g, (*(*rule).guard).next);
        addSinglet(g, (*(*rule).guard).next);
        addSinglet(g, (*(*(*rule).guard).next).next);
    }
    if !(*(*(*rule).guard).next).rule.is_null()
        && (*(*(*(*rule).guard).next).rule).refcount == 1 as u32
    {
        expandCall(g, (*(*rule).guard).next);
    }
}
unsafe extern "C" fn processMatchSinglet(
    mut g: *mut cff_SubrGraph,
    mut m: *mut cff_SubrNode,
    mut n: *mut cff_SubrNode,
) {
    let mut rule: *mut cff_SubrRule = ::core::ptr::null_mut::<cff_SubrRule>();
    if (*(*m).prev).guard as ::core::ffi::c_int != 0
        && (*(*m).next).guard as ::core::ffi::c_int != 0
    {
        rule = (*(*m).prev).rule;
        substituteSingletWithRule(g, n, rule);
    } else {
        rule = cff_new_Rule();
        (*rule).uniqueIndex = (*g).totalRules;
        (*g).totalRules = (*g).totalRules.wrapping_add(1 as u32);
        (*(*g).last).next = rule;
        (*g).last = rule;
        xInsertNodeAfter(g, lastNodeOf(rule), copyNode(m));
        substituteSingletWithRule(g, m, rule);
        substituteSingletWithRule(g, n, rule);
        addSinglet(g, (*(*rule).guard).next);
    };
}
unsafe extern "C" fn checkDoubletMatch(
    mut g: *mut cff_SubrGraph,
    mut n: *mut cff_SubrNode,
) -> bool {
    if (*n).guard as ::core::ffi::c_int != 0
        || (*(*n).next).guard as ::core::ffi::c_int != 0
        || (*n).hard as ::core::ffi::c_int != 0
        || (*(*n).next).hard as ::core::ffi::c_int != 0
    {
        return false;
    }
    let mut len: usize = 0;
    let mut key: *mut u8 = getDoubletHashKey(n, &raw mut len);
    let mut di: *mut cff_SubrDiagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = len as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(len as ::core::ffi::c_uint);
    let mut current_block_52: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 15091596139706490623;
        }
        10 => {
            current_block_52 = 15091596139706490623;
        }
        9 => {
            current_block_52 = 17241625601759356835;
        }
        8 => {
            current_block_52 = 5675292805541825004;
        }
        7 => {
            current_block_52 = 1451031817809626151;
        }
        6 => {
            current_block_52 = 13294789712330503413;
        }
        5 => {
            current_block_52 = 8695097831189777730;
        }
        4 => {
            current_block_52 = 13325691877627560001;
        }
        3 => {
            current_block_52 = 2988620532390260735;
        }
        2 => {
            current_block_52 = 12886844422053508718;
        }
        1 => {
            current_block_52 = 3499941675142865411;
        }
        _ => {
            current_block_52 = 721385680381463314;
        }
    }
    match current_block_52 {
        15091596139706490623 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 17241625601759356835;
        }
        _ => {}
    }
    match current_block_52 {
        17241625601759356835 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 5675292805541825004;
        }
        _ => {}
    }
    match current_block_52 {
        5675292805541825004 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 1451031817809626151;
        }
        _ => {}
    }
    match current_block_52 {
        1451031817809626151 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 13294789712330503413;
        }
        _ => {}
    }
    match current_block_52 {
        13294789712330503413 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 8695097831189777730;
        }
        _ => {}
    }
    match current_block_52 {
        8695097831189777730 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_52 = 13325691877627560001;
        }
        _ => {}
    }
    match current_block_52 {
        13325691877627560001 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 2988620532390260735;
        }
        _ => {}
    }
    match current_block_52 {
        2988620532390260735 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 12886844422053508718;
        }
        _ => {}
    }
    match current_block_52 {
        12886844422053508718 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 3499941675142865411;
        }
        _ => {}
    }
    match current_block_52 {
        3499941675142865411 => {
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
    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    if !(*g).diagramIndex.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*g).diagramIndex).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                di = ((*(*(*(*g).diagramIndex).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut cff_SubrDiagramIndex
                    as *mut cff_SubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
            }
            while !di.is_null() {
                if (*di).hh.hashv == _hf_hashv && (*di).hh.keylen as usize == len {
                    if memcmp((*di).hh.key, key as *const ::core::ffi::c_void, len)
                        == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*di).hh.hh_next.is_null() {
                    di = ((*di).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut cff_SubrDiagramIndex
                        as *mut cff_SubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
                }
            }
        }
    }
    if di.is_null() {
        di = __caryll_allocate_clean(
            ::core::mem::size_of::<cff_SubrDiagramIndex>() as usize,
            390 as ::core::ffi::c_ulong,
        ) as *mut cff_SubrDiagramIndex;
        (*di).arity = 2 as u8;
        (*di).key = key;
        (*di).start = n;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = len as ::core::ffi::c_uint;
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
        _ha_hashv = _ha_hashv.wrapping_add(len as ::core::ffi::c_uint);
        let mut current_block_170: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 16460569035642625659;
            }
            10 => {
                current_block_170 = 16460569035642625659;
            }
            9 => {
                current_block_170 = 18223283538573607610;
            }
            8 => {
                current_block_170 = 8271826669217136281;
            }
            7 => {
                current_block_170 = 3562347258404297153;
            }
            6 => {
                current_block_170 = 4002742741260996336;
            }
            5 => {
                current_block_170 = 11968114151537269781;
            }
            4 => {
                current_block_170 = 4455416393538530741;
            }
            3 => {
                current_block_170 = 15081387036960476447;
            }
            2 => {
                current_block_170 = 3314583722958503306;
            }
            1 => {
                current_block_170 = 931228513832161105;
            }
            _ => {
                current_block_170 = 16937825661756021828;
            }
        }
        match current_block_170 {
            16460569035642625659 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 18223283538573607610;
            }
            _ => {}
        }
        match current_block_170 {
            18223283538573607610 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 8271826669217136281;
            }
            _ => {}
        }
        match current_block_170 {
            8271826669217136281 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 3562347258404297153;
            }
            _ => {}
        }
        match current_block_170 {
            3562347258404297153 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 4002742741260996336;
            }
            _ => {}
        }
        match current_block_170 {
            4002742741260996336 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 11968114151537269781;
            }
            _ => {}
        }
        match current_block_170 {
            11968114151537269781 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_170 = 4455416393538530741;
            }
            _ => {}
        }
        match current_block_170 {
            4455416393538530741 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 15081387036960476447;
            }
            _ => {}
        }
        match current_block_170 {
            15081387036960476447 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 3314583722958503306;
            }
            _ => {}
        }
        match current_block_170 {
            3314583722958503306 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 931228513832161105;
            }
            _ => {}
        }
        match current_block_170 {
            931228513832161105 => {
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
        (*di).hh.hashv = _ha_hashv;
        (*di).hh.key = key as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*di).hh.keylen = len as ::core::ffi::c_uint;
        if (*g).diagramIndex.is_null() {
            (*di).hh.next = NULL;
            (*di).hh.prev = NULL;
            (*di).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*di).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*di).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as usize,
                );
                (*(*di).hh.tbl).tail = &raw mut (*di).hh as *mut UT_hash_handle;
                (*(*di).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*di).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*di).hh.tbl).hho = (&raw mut (*di).hh as *mut ::core::ffi::c_char)
                    .offset_from(di as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*di).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                (*(*di).hh.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*di).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    );
                }
            }
            (*g).diagramIndex = di;
        } else {
            (*di).hh.tbl = (*(*g).diagramIndex).hh.tbl;
            (*di).hh.next = NULL;
            (*di).hh.prev = ((*(*(*g).diagramIndex).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*g).diagramIndex).hh.tbl).tail).next = di as *mut ::core::ffi::c_void;
            (*(*(*g).diagramIndex).hh.tbl).tail = &raw mut (*di).hh as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*g).diagramIndex).hh.tbl).num_items =
            (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = (*(*(*g).diagramIndex).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*di).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*di).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*di).hh as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*di).hh as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*di).hh.tbl).noexpand == 0
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
                    .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                );
                (*(*di).hh.tbl).ideal_chain_maxlen = ((*(*di).hh.tbl).num_items
                    >> (*(*di).hh.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*di).hh.tbl).num_items
                        & (*(*di).hh.tbl)
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
                (*(*di).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*di).hh.tbl).num_buckets {
                    _he_thh = (*(*(*di).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*di).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*di).hh.tbl).ideal_chain_maxlen {
                            (*(*di).hh.tbl).nonideal_items =
                                (*(*di).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*di).hh.tbl).ideal_chain_maxlen);
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
                free((*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*di).hh.tbl).num_buckets = (*(*di).hh.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*di).hh.tbl).log2_num_buckets = (*(*di).hh.tbl).log2_num_buckets.wrapping_add(1);
                (*(*di).hh.tbl).buckets = _he_new_buckets;
                (*(*di).hh.tbl).ineff_expands = if (*(*di).hh.tbl).nonideal_items
                    > (*(*di).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*di).hh.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*di).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*di).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        return false;
    } else if (*di).arity as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        && (*di).start != n
        && !(*(*di).start).guard
        && !(*(*(*di).start).next).guard
    {
        free(key as *mut ::core::ffi::c_void);
        key = ::core::ptr::null_mut::<u8>();
        processMatchDoublet(g, (*di).start, n);
        return true;
    } else {
        free(key as *mut ::core::ffi::c_void);
        key = ::core::ptr::null_mut::<u8>();
        return true;
    };
}
unsafe extern "C" fn checkSingletMatch(
    mut g: *mut cff_SubrGraph,
    mut n: *mut cff_SubrNode,
) -> bool {
    if (*n).guard as ::core::ffi::c_int != 0 || (*n).hard as ::core::ffi::c_int != 0 {
        return false;
    }
    let mut len: usize = 0;
    let mut key: *mut u8 = getSingletHashKey(n, &raw mut len);
    let mut di: *mut cff_SubrDiagramIndex = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = len as ::core::ffi::c_uint;
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
    _hf_hashv = _hf_hashv.wrapping_add(len as ::core::ffi::c_uint);
    let mut current_block_52: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 5888882860707986980;
        }
        10 => {
            current_block_52 = 5888882860707986980;
        }
        9 => {
            current_block_52 = 8756645787125579341;
        }
        8 => {
            current_block_52 = 1667839473427452658;
        }
        7 => {
            current_block_52 = 5994689378794502271;
        }
        6 => {
            current_block_52 = 1856441548294692454;
        }
        5 => {
            current_block_52 = 18382014826127487805;
        }
        4 => {
            current_block_52 = 3714331854834949745;
        }
        3 => {
            current_block_52 = 9344057869356161503;
        }
        2 => {
            current_block_52 = 4922295335966035392;
        }
        1 => {
            current_block_52 = 3836700389814666292;
        }
        _ => {
            current_block_52 = 721385680381463314;
        }
    }
    match current_block_52 {
        5888882860707986980 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 8756645787125579341;
        }
        _ => {}
    }
    match current_block_52 {
        8756645787125579341 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 1667839473427452658;
        }
        _ => {}
    }
    match current_block_52 {
        1667839473427452658 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 5994689378794502271;
        }
        _ => {}
    }
    match current_block_52 {
        5994689378794502271 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 1856441548294692454;
        }
        _ => {}
    }
    match current_block_52 {
        1856441548294692454 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 18382014826127487805;
        }
        _ => {}
    }
    match current_block_52 {
        18382014826127487805 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_52 = 3714331854834949745;
        }
        _ => {}
    }
    match current_block_52 {
        3714331854834949745 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_52 = 9344057869356161503;
        }
        _ => {}
    }
    match current_block_52 {
        9344057869356161503 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_52 = 4922295335966035392;
        }
        _ => {}
    }
    match current_block_52 {
        4922295335966035392 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_52 = 3836700389814666292;
        }
        _ => {}
    }
    match current_block_52 {
        3836700389814666292 => {
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
    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
    if !(*g).diagramIndex.is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(*(*g).diagramIndex).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                di = ((*(*(*(*g).diagramIndex).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut cff_SubrDiagramIndex
                    as *mut cff_SubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
            }
            while !di.is_null() {
                if (*di).hh.hashv == _hf_hashv && (*di).hh.keylen as usize == len {
                    if memcmp((*di).hh.key, key as *const ::core::ffi::c_void, len)
                        == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*di).hh.hh_next.is_null() {
                    di = ((*di).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut cff_SubrDiagramIndex
                        as *mut cff_SubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<cff_SubrDiagramIndex>();
                }
            }
        }
    }
    if di.is_null() {
        di = __caryll_allocate_clean(
            ::core::mem::size_of::<cff_SubrDiagramIndex>() as usize,
            413 as ::core::ffi::c_ulong,
        ) as *mut cff_SubrDiagramIndex;
        (*di).arity = 1 as u8;
        (*di).key = key;
        (*di).start = n;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = len as ::core::ffi::c_uint;
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
        _ha_hashv = _ha_hashv.wrapping_add(len as ::core::ffi::c_uint);
        let mut current_block_170: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 11054281273194031488;
            }
            10 => {
                current_block_170 = 11054281273194031488;
            }
            9 => {
                current_block_170 = 17839318416088138101;
            }
            8 => {
                current_block_170 = 621808156054073165;
            }
            7 => {
                current_block_170 = 9521043475329100663;
            }
            6 => {
                current_block_170 = 10390157400189626493;
            }
            5 => {
                current_block_170 = 17487448190473239203;
            }
            4 => {
                current_block_170 = 228348456451232718;
            }
            3 => {
                current_block_170 = 7904285175733835103;
            }
            2 => {
                current_block_170 = 9058279731987806237;
            }
            1 => {
                current_block_170 = 12880570941515335930;
            }
            _ => {
                current_block_170 = 16937825661756021828;
            }
        }
        match current_block_170 {
            11054281273194031488 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 17839318416088138101;
            }
            _ => {}
        }
        match current_block_170 {
            17839318416088138101 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 621808156054073165;
            }
            _ => {}
        }
        match current_block_170 {
            621808156054073165 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 9521043475329100663;
            }
            _ => {}
        }
        match current_block_170 {
            9521043475329100663 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 10390157400189626493;
            }
            _ => {}
        }
        match current_block_170 {
            10390157400189626493 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 17487448190473239203;
            }
            _ => {}
        }
        match current_block_170 {
            17487448190473239203 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_170 = 228348456451232718;
            }
            _ => {}
        }
        match current_block_170 {
            228348456451232718 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_170 = 7904285175733835103;
            }
            _ => {}
        }
        match current_block_170 {
            7904285175733835103 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_170 = 9058279731987806237;
            }
            _ => {}
        }
        match current_block_170 {
            9058279731987806237 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_170 = 12880570941515335930;
            }
            _ => {}
        }
        match current_block_170 {
            12880570941515335930 => {
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
        (*di).hh.hashv = _ha_hashv;
        (*di).hh.key = key as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*di).hh.keylen = len as ::core::ffi::c_uint;
        if (*g).diagramIndex.is_null() {
            (*di).hh.next = NULL;
            (*di).hh.prev = NULL;
            (*di).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*di).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*di).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as usize,
                );
                (*(*di).hh.tbl).tail = &raw mut (*di).hh as *mut UT_hash_handle;
                (*(*di).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*di).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*di).hh.tbl).hho = (&raw mut (*di).hh as *mut ::core::ffi::c_char)
                    .offset_from(di as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*di).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                (*(*di).hh.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*di).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    );
                }
            }
            (*g).diagramIndex = di;
        } else {
            (*di).hh.tbl = (*(*g).diagramIndex).hh.tbl;
            (*di).hh.next = NULL;
            (*di).hh.prev = ((*(*(*g).diagramIndex).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*(*g).diagramIndex).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*g).diagramIndex).hh.tbl).tail).next = di as *mut ::core::ffi::c_void;
            (*(*(*g).diagramIndex).hh.tbl).tail = &raw mut (*di).hh as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*g).diagramIndex).hh.tbl).num_items =
            (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = (*(*(*g).diagramIndex).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*di).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*di).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*di).hh as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*di).hh as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*di).hh.tbl).noexpand == 0
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
                    .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                );
                (*(*di).hh.tbl).ideal_chain_maxlen = ((*(*di).hh.tbl).num_items
                    >> (*(*di).hh.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*di).hh.tbl).num_items
                        & (*(*di).hh.tbl)
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
                (*(*di).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*di).hh.tbl).num_buckets {
                    _he_thh = (*(*(*di).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*di).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*di).hh.tbl).ideal_chain_maxlen {
                            (*(*di).hh.tbl).nonideal_items =
                                (*(*di).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*di).hh.tbl).ideal_chain_maxlen);
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
                free((*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*di).hh.tbl).num_buckets = (*(*di).hh.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*di).hh.tbl).log2_num_buckets = (*(*di).hh.tbl).log2_num_buckets.wrapping_add(1);
                (*(*di).hh.tbl).buckets = _he_new_buckets;
                (*(*di).hh.tbl).ineff_expands = if (*(*di).hh.tbl).nonideal_items
                    > (*(*di).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*di).hh.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*di).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*di).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        return false;
    } else if (*di).arity as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && (*di).start != n
        && !(*(*di).start).guard
    {
        free(key as *mut ::core::ffi::c_void);
        key = ::core::ptr::null_mut::<u8>();
        processMatchSinglet(g, (*di).start, n);
        return true;
    } else {
        free(key as *mut ::core::ffi::c_void);
        key = ::core::ptr::null_mut::<u8>();
        return false;
    };
}
unsafe extern "C" fn appendNodeToGraph(mut g: *mut cff_SubrGraph, mut n: *mut cff_SubrNode) {
    let mut last: *mut cff_SubrNode = lastNodeOf((*g).root);
    xInsertNodeAfter(g, last, n);
    if (*g).doSubroutinize {
        if !checkDoubletMatch(g, last) {
            if buflen((*n).terminal) > 15 as usize {
                checkSingletMatch(g, n);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cff_insertILToGraph(
    mut g: *mut cff_SubrGraph,
    mut il: *mut cff_CharstringIL,
) {
    let mut blob: *mut caryll_Buffer = bufnew();
    let mut flush: bool = false;
    let mut last: bool = false;
    let mut j: u32 = 0 as u32;
    while j < (*il).length {
        match (*(*il).instr.offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                if flush {
                    let mut n: *mut cff_SubrNode = cff_new_Node();
                    (*n).rule = ::core::ptr::null_mut::<cff_SubrRule>();
                    (*n).terminal = blob;
                    (*n).last = last;
                    appendNodeToGraph(g, n);
                    blob = bufnew();
                    flush = false;
                }
                cff_mergeCS2Operand(blob, (*(*il).instr.offset(j as isize)).c2rust_unnamed.d);
            }
            1 => {
                cff_mergeCS2Operator(blob, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
                if (*(*il).instr.offset(j as isize)).c2rust_unnamed.i
                    == op_endchar as ::core::ffi::c_int as i32
                {
                    last = true;
                }
                flush = true;
            }
            2 => {
                cff_mergeCS2Special(
                    blob,
                    (*(*il).instr.offset(j as isize)).c2rust_unnamed.i as u8,
                );
                flush = true;
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    if (*blob).size != 0 {
        let mut n_0: *mut cff_SubrNode = cff_new_Node();
        (*n_0).rule = ::core::ptr::null_mut::<cff_SubrRule>();
        (*n_0).last = last;
        (*n_0).terminal = blob;
        appendNodeToGraph(g, n_0);
    }
    blob = bufnew();
    let mut n_1: *mut cff_SubrNode = cff_new_Node();
    (*n_1).rule = ::core::ptr::null_mut::<cff_SubrRule>();
    (*n_1).terminal = blob;
    (*n_1).hard = true;
    appendNodeToGraph(g, n_1);
    (*g).totalCharStrings = (*g).totalCharStrings.wrapping_add(1 as u32);
}
unsafe extern "C" fn cff_statHeight(mut r: *mut cff_SubrRule, mut height: u32) {
    if height > (*r).height {
        (*r).height = height;
    }
    let mut effectiveLength: u32 = 0 as u32;
    let mut e: *mut cff_SubrNode = (*(*r).guard).next;
    while e != (*r).guard {
        if !(*e).rule.is_null() {
            cff_statHeight((*e).rule, height.wrapping_add(1 as u32));
            effectiveLength = effectiveLength.wrapping_add(4 as u32);
        } else {
            effectiveLength = (effectiveLength as usize).wrapping_add((*(*e).terminal).size)
                as u32 as u32;
        }
        e = (*e).next;
    }
    (*r).effectiveLength = effectiveLength;
}
unsafe extern "C" fn numberASubroutine(mut r: *mut cff_SubrRule, mut current: *mut u32) {
    if (*r).numbered {
        return;
    }
    if (*r).height >= type2_subr_nesting as ::core::ffi::c_int as u32 {
        return;
    }
    if (*r)
        .effectiveLength
        .wrapping_sub(4 as u32)
        .wrapping_mul((*r).refcount.wrapping_sub(1 as u32))
        .wrapping_sub(4 as u32)
        <= 0 as u32
    {
        return;
    }
    (*r).number = *current;
    *current = (*current).wrapping_add(1);
    (*r).numbered = true;
    let mut e: *mut cff_SubrNode = (*(*r).guard).next;
    while e != (*r).guard {
        if !(*e).rule.is_null() {
            numberASubroutine((*e).rule, current);
        }
        e = (*e).next;
    }
}
unsafe extern "C" fn cff_numberSubroutines(mut g: *mut cff_SubrGraph) -> u32 {
    let mut current: u32 = 0 as u32;
    let mut e: *mut cff_SubrNode = (*(*(*g).root).guard).next;
    while e != (*(*g).root).guard {
        if !(*e).rule.is_null() {
            numberASubroutine((*e).rule, &raw mut current);
        }
        e = (*e).next;
    }
    return current;
}
#[inline]
unsafe extern "C" fn subroutineBias(mut cnt: i32) -> i32 {
    if cnt < 1240 as i32 {
        return 107 as i32;
    } else if cnt < 33900 as i32 {
        return 1131 as i32;
    } else {
        return 32768 as i32;
    };
}
unsafe extern "C" fn endsWithEndChar(mut rule: *mut cff_SubrRule) -> bool {
    let mut node: *mut cff_SubrNode = lastNodeOf(rule);
    if !(*node).terminal.is_null() {
        return (*node).last;
    } else {
        return endsWithEndChar((*node).rule);
    };
}
unsafe extern "C" fn serializeNodeToBuffer(
    mut node: *mut cff_SubrNode,
    mut buf: *mut caryll_Buffer,
    mut gsubrs: *mut caryll_Buffer,
    mut maxGSubrs: u32,
    mut lsubrs: *mut caryll_Buffer,
    mut maxLSubrs: u32,
) {
    if !(*node).rule.is_null() {
        if (*(*node).rule).numbered as ::core::ffi::c_int != 0
            && (*(*node).rule).number < maxLSubrs.wrapping_add(maxGSubrs)
            && (*(*node).rule).height < type2_subr_nesting as ::core::ffi::c_int as u32
        {
            let mut target: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
            if (*(*node).rule).number < maxLSubrs {
                let mut stacknum: i32 = (*(*node).rule)
                    .number
                    .wrapping_sub(subroutineBias(maxLSubrs as i32) as u32)
                    as i32;
                target = lsubrs.offset((*(*node).rule).number as isize);
                cff_mergeCS2Int(buf, stacknum);
                cff_mergeCS2Operator(buf, op_callsubr as ::core::ffi::c_int as i32);
            } else {
                let mut stacknum_0: i32 = (*(*node).rule)
                    .number
                    .wrapping_sub(maxLSubrs)
                    .wrapping_sub(subroutineBias(maxGSubrs as i32) as u32)
                    as i32;
                target = gsubrs.offset((*(*node).rule).number.wrapping_sub(maxLSubrs) as isize);
                cff_mergeCS2Int(buf, stacknum_0);
                cff_mergeCS2Operator(buf, op_callgsubr as ::core::ffi::c_int as i32);
            }
            let mut r: *mut cff_SubrRule = (*node).rule;
            if !(*r).printed {
                (*r).printed = true;
                let mut e: *mut cff_SubrNode = (*(*r).guard).next;
                while e != (*r).guard {
                    serializeNodeToBuffer(e, target, gsubrs, maxGSubrs, lsubrs, maxLSubrs);
                    e = (*e).next;
                }
                if !endsWithEndChar(r) {
                    cff_mergeCS2Operator(target, op_return as ::core::ffi::c_int as i32);
                }
            }
        } else {
            let mut r_0: *mut cff_SubrRule = (*node).rule;
            let mut e_0: *mut cff_SubrNode = (*(*r_0).guard).next;
            while e_0 != (*r_0).guard {
                serializeNodeToBuffer(e_0, buf, gsubrs, maxGSubrs, lsubrs, maxLSubrs);
                e_0 = (*e_0).next;
            }
        }
    } else {
        bufwrite_buf(buf, (*node).terminal);
    };
}
unsafe extern "C" fn from_array(
    mut _context: *mut ::core::ffi::c_void,
    mut j: u32,
) -> *mut caryll_Buffer {
    let mut context: *mut caryll_Buffer = _context as *mut caryll_Buffer;
    let mut blob: *mut caryll_Buffer = bufnew();
    bufwrite_buf(blob, context.offset(j as isize));
    return blob;
}
#[no_mangle]
pub unsafe extern "C" fn cff_ilGraphToBuffers(
    mut g: *mut cff_SubrGraph,
    mut s: *mut *mut caryll_Buffer,
    mut gs: *mut *mut caryll_Buffer,
    mut ls: *mut *mut caryll_Buffer,
    mut options: *const otfcc_Options,
) {
    cff_statHeight((*g).root, 0 as u32);
    let mut maxSubroutines: u32 = cff_numberSubroutines(g);
    (*(*options).logger)
        .logSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        log_vl_progress as ::core::ffi::c_int as u8,
        log_type_progress,
        sdscatprintf(
            sdsempty(),
            b"[libcff] Total %d subroutines extracted.\0" as *const u8
                as *const ::core::ffi::c_char,
            maxSubroutines,
        ),
    );
    let mut maxLSubrs: u32 = maxSubroutines;
    let mut maxGSubrs: u32 = 0 as u32;
    if maxLSubrs > type2_max_subrs as ::core::ffi::c_int as u32 {
        maxLSubrs = type2_max_subrs as ::core::ffi::c_int as u32;
        maxGSubrs = maxSubroutines.wrapping_sub(maxLSubrs);
    }
    if maxGSubrs > type2_max_subrs as ::core::ffi::c_int as u32 {
        maxGSubrs = type2_max_subrs as ::core::ffi::c_int as u32;
    }
    let mut total: u32 = maxLSubrs.wrapping_add(maxGSubrs);
    maxLSubrs = total.wrapping_div(2 as u32);
    maxGSubrs = total.wrapping_sub(maxLSubrs);
    let mut charStrings: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    let mut gsubrs: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    let mut lsubrs: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    charStrings = __caryll_allocate_clean(
        (::core::mem::size_of::<caryll_Buffer>() as usize)
            .wrapping_mul((*g).totalCharStrings.wrapping_add(1 as u32) as usize),
        608 as ::core::ffi::c_ulong,
    ) as *mut caryll_Buffer;
    lsubrs = __caryll_allocate_clean(
        (::core::mem::size_of::<caryll_Buffer>() as usize)
            .wrapping_mul(maxLSubrs.wrapping_add(1 as u32) as usize),
        609 as ::core::ffi::c_ulong,
    ) as *mut caryll_Buffer;
    gsubrs = __caryll_allocate_clean(
        (::core::mem::size_of::<caryll_Buffer>() as usize)
            .wrapping_mul(maxGSubrs.wrapping_add(1 as u32) as usize),
        610 as ::core::ffi::c_ulong,
    ) as *mut caryll_Buffer;
    let mut j: u32 = 0 as u32;
    let mut r: *mut cff_SubrRule = (*g).root;
    let mut e: *mut cff_SubrNode = (*(*r).guard).next;
    while e != (*r).guard {
        serializeNodeToBuffer(
            e,
            charStrings.offset(j as isize),
            gsubrs,
            maxGSubrs,
            lsubrs,
            maxLSubrs,
        );
        if (*e).rule.is_null() && !(*e).terminal.is_null() && (*e).hard as ::core::ffi::c_int != 0 {
            j = j.wrapping_add(1);
        }
        e = (*e).next;
    }
    let mut is: *mut cff_Index = cff_iIndex.fromCallback.expect("non-null function pointer")(
        charStrings as *mut ::core::ffi::c_void,
        (*g).totalCharStrings,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut caryll_Buffer,
        ),
    );
    let mut igs: *mut cff_Index = cff_iIndex.fromCallback.expect("non-null function pointer")(
        gsubrs as *mut ::core::ffi::c_void,
        maxGSubrs,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut caryll_Buffer,
        ),
    );
    let mut ils: *mut cff_Index = cff_iIndex.fromCallback.expect("non-null function pointer")(
        lsubrs as *mut ::core::ffi::c_void,
        maxLSubrs,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut caryll_Buffer,
        ),
    );
    let mut j_0: u32 = 0 as u32;
    while j_0 < (*g).totalCharStrings {
        free((*charStrings.offset(j_0 as isize)).data as *mut ::core::ffi::c_void);
        let ref mut fresh6 = (*charStrings.offset(j_0 as isize)).data;
        *fresh6 = ::core::ptr::null_mut::<u8>();
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_1: u32 = 0 as u32;
    while j_1 < maxGSubrs {
        free((*gsubrs.offset(j_1 as isize)).data as *mut ::core::ffi::c_void);
        let ref mut fresh7 = (*gsubrs.offset(j_1 as isize)).data;
        *fresh7 = ::core::ptr::null_mut::<u8>();
        j_1 = j_1.wrapping_add(1);
    }
    let mut j_2: u32 = 0 as u32;
    while j_2 < maxLSubrs {
        free((*lsubrs.offset(j_2 as isize)).data as *mut ::core::ffi::c_void);
        let ref mut fresh8 = (*lsubrs.offset(j_2 as isize)).data;
        *fresh8 = ::core::ptr::null_mut::<u8>();
        j_2 = j_2.wrapping_add(1);
    }
    free(charStrings as *mut ::core::ffi::c_void);
    charStrings = ::core::ptr::null_mut::<caryll_Buffer>();
    free(gsubrs as *mut ::core::ffi::c_void);
    gsubrs = ::core::ptr::null_mut::<caryll_Buffer>();
    free(lsubrs as *mut ::core::ffi::c_void);
    lsubrs = ::core::ptr::null_mut::<caryll_Buffer>();
    *s = cff_iIndex.build.expect("non-null function pointer")(is);
    *gs = cff_iIndex.build.expect("non-null function pointer")(igs);
    *ls = cff_iIndex.build.expect("non-null function pointer")(ils);
    cff_iIndex.free.expect("non-null function pointer")(is);
    cff_iIndex.free.expect("non-null function pointer")(igs);
    cff_iIndex.free.expect("non-null function pointer")(ils);
}
