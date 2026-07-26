#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memcpy, memset, strncmp};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{log_type_progress, log_vl_progress, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};

use crate::libcff::{op_callgsubr, op_callsubr, op_endchar, op_return, type2_max_subrs, type2_subr_nesting};
use crate::libcff::cff_index::CffIndex;
use crate::libcff::charstring_il::{CffCharstringIl};
use crate::support::{NULL};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::libcff::cff_index::{cff_iIndex};
use crate::libcff::cff_writer::{cff_mergeCS2Int, cff_mergeCS2Operand, cff_mergeCS2Operator, cff_mergeCS2Special};
use crate::support::buffer::{buffree, buflen, bufnew, bufwrite_buf};
use crate::vendor::sds::{sdsempty};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffSubrRule {
    pub printed: bool,
    pub numbered: bool,
    pub number: u32,
    pub height: u32,
    pub uniqueIndex: u32,
    pub cffIndex: u16,
    pub refcount: u32,
    pub effectiveLength: u32,
    pub guard: *mut CffSubrNode,
    pub next: *mut CffSubrRule,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffSubrNode {
    pub prev: *mut CffSubrNode,
    pub rule: *mut CffSubrRule,
    pub next: *mut CffSubrNode,
    pub terminal: *mut Buffer,
    pub hard: bool,
    pub guard: bool,
    pub last: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffSubrDiagramIndex {
    pub arity: u8,
    pub key: *mut u8,
    pub start: *mut CffSubrNode,
    pub hh: UtHashHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffSubrGraph {
    pub root: *mut CffSubrRule,
    pub last: *mut CffSubrRule,
    pub diagramIndex: *mut CffSubrDiagramIndex,
    pub totalRules: u32,
    pub totalCharStrings: u32,
    pub doSubroutinize: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffSubrGraphElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CffSubrGraph) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CffSubrGraph, *const CffSubrGraph) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut CffSubrGraph, *mut CffSubrGraph) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CffSubrGraph) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut CffSubrGraph, CffSubrGraph) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut CffSubrGraph, CffSubrGraph) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut CffSubrGraph>,
    pub free: Option<unsafe extern "C" fn(*mut CffSubrGraph) -> ()>,
}
unsafe extern "C" fn cff_new_Node() -> *mut CffSubrNode {
    let mut n: *mut CffSubrNode = ::core::ptr::null_mut::<CffSubrNode>();
    n = __caryll_allocate_clean(
        ::core::mem::size_of::<CffSubrNode>() as usize,
        19 as ::core::ffi::c_ulong,
    ) as *mut CffSubrNode;
    (*n).rule = ::core::ptr::null_mut::<CffSubrRule>();
    (*n).terminal = ::core::ptr::null_mut::<Buffer>();
    (*n).guard = false;
    (*n).hard = false;
    (*n).prev = ::core::ptr::null_mut::<CffSubrNode>();
    (*n).next = ::core::ptr::null_mut::<CffSubrNode>();
    return n;
}
unsafe extern "C" fn cff_new_Rule() -> *mut CffSubrRule {
    let mut r: *mut CffSubrRule = ::core::ptr::null_mut::<CffSubrRule>();
    r = __caryll_allocate_clean(
        ::core::mem::size_of::<CffSubrRule>() as usize,
        34 as ::core::ffi::c_ulong,
    ) as *mut CffSubrRule;
    (*r).refcount = 0 as u32;
    (*r).guard = cff_new_Node();
    (*(*r).guard).prev = (*r).guard;
    (*(*r).guard).next = (*r).guard;
    (*(*r).guard).terminal = ::core::ptr::null_mut::<Buffer>();
    (*(*r).guard).guard = true;
    (*(*r).guard).rule = r;
    (*r).next = ::core::ptr::null_mut::<CffSubrRule>();
    return r;
}
unsafe extern "C" fn initSubrGraph(mut g: *mut CffSubrGraph) {
    (*g).root = cff_new_Rule();
    (*g).last = (*g).root;
    (*g).diagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
    (*g).totalRules = 0 as u32;
    (*g).totalCharStrings = 0 as u32;
    (*g).doSubroutinize = false;
}
unsafe extern "C" fn clean_Node(mut x: *mut CffSubrNode) {
    if !(*x).rule.is_null() {
        (*(*x).rule).refcount = (*(*x).rule).refcount.wrapping_sub(1 as u32);
    }
    (*x).rule = ::core::ptr::null_mut::<CffSubrRule>();
    buffree((*x).terminal);
    (*x).terminal = ::core::ptr::null_mut::<Buffer>();
}
unsafe extern "C" fn delete_Node(mut x: *mut CffSubrNode) {
    if x.is_null() {
        return;
    }
    clean_Node(x);
    free(x as *mut ::core::ffi::c_void);
    x = ::core::ptr::null_mut::<CffSubrNode>();
}
unsafe extern "C" fn deleteFullRule(mut r: *mut CffSubrRule) {
    if !(*r).guard.is_null() {
        let mut e: *mut CffSubrNode = (*(*r).guard).next;
        while e != (*r).guard {
            let mut next: *mut CffSubrNode = (*e).next;
            if !(*e).terminal.is_null() {
                buffree((*e).terminal);
            }
            free(e as *mut ::core::ffi::c_void);
            e = ::core::ptr::null_mut::<CffSubrNode>();
            e = next;
        }
        free((*r).guard as *mut ::core::ffi::c_void);
        (*r).guard = ::core::ptr::null_mut::<CffSubrNode>();
    }
    free(r as *mut ::core::ffi::c_void);
    r = ::core::ptr::null_mut::<CffSubrRule>();
}
unsafe extern "C" fn disposeSubrGraph(mut g: *mut CffSubrGraph) {
    let mut r: *mut CffSubrRule = (*g).root;
    while !r.is_null() {
        let mut next: *mut CffSubrRule = (*r).next;
        deleteFullRule(r);
        r = next;
    }
    let mut s: *mut CffSubrDiagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
    let mut tmp: *mut CffSubrDiagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
    s = (*g).diagramIndex;
    tmp = (if !(*g).diagramIndex.is_null() {
        (*(*g).diagramIndex).hh.next
    } else {
        NULL
    }) as *mut CffSubrDiagramIndex as *mut CffSubrDiagramIndex;
    while !s.is_null() {
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*g).diagramIndex).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*g).diagramIndex).hh.tbl as *mut ::core::ffi::c_void);
            (*g).diagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*g).diagramIndex).hh.tbl).tail {
                (*(*(*g).diagramIndex).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut UtHashHandle as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh0 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh0 = (*_hd_hh_del).next;
            } else {
                (*g).diagramIndex =
                    (*_hd_hh_del).next as *mut CffSubrDiagramIndex as *mut CffSubrDiagramIndex;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh1 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh1 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*g).diagramIndex).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket = (*(*(*g).diagramIndex).hh.tbl)
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
            (*(*(*g).diagramIndex).hh.tbl).num_items =
                (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_sub(1);
        }
        free((*s).key as *mut ::core::ffi::c_void);
        (*s).key = ::core::ptr::null_mut::<u8>();
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
        s = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut CffSubrDiagramIndex
            as *mut CffSubrDiagramIndex;
    }
}
pub static cff_iSubrGraph: CffSubrGraphElementInterface = {
    CffSubrGraphElementInterface {
        init: Some(cff_SubrGraph_init as unsafe extern "C" fn(*mut CffSubrGraph) -> ()),
        copy: Some(
            cff_SubrGraph_copy
                as unsafe extern "C" fn(*mut CffSubrGraph, *const CffSubrGraph) -> (),
        ),
        move_0: Some(
            cff_SubrGraph_move
                as unsafe extern "C" fn(*mut CffSubrGraph, *mut CffSubrGraph) -> (),
        ),
        dispose: Some(cff_SubrGraph_dispose as unsafe extern "C" fn(*mut CffSubrGraph) -> ()),
        replace: Some(
            cff_SubrGraph_replace as unsafe extern "C" fn(*mut CffSubrGraph, CffSubrGraph) -> (),
        ),
        copyReplace: Some(
            cff_SubrGraph_copyReplace
                as unsafe extern "C" fn(*mut CffSubrGraph, CffSubrGraph) -> (),
        ),
        create: Some(cff_SubrGraph_create),
        free: Some(cff_SubrGraph_free as unsafe extern "C" fn(*mut CffSubrGraph) -> ()),
    }
};
#[inline]
unsafe extern "C" fn cff_SubrGraph_free(mut x: *mut CffSubrGraph) {
    if x.is_null() {
        return;
    }
    cff_SubrGraph_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_create() -> *mut CffSubrGraph {
    let mut x: *mut CffSubrGraph =
        malloc(::core::mem::size_of::<CffSubrGraph>() as usize) as *mut CffSubrGraph;
    cff_SubrGraph_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_copyReplace(mut dst: *mut CffSubrGraph, src: CffSubrGraph) {
    cff_SubrGraph_dispose(dst);
    cff_SubrGraph_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_init(mut x: *mut CffSubrGraph) {
    initSubrGraph(x);
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_dispose(mut x: *mut CffSubrGraph) {
    disposeSubrGraph(x);
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_copy(
    mut dst: *mut CffSubrGraph,
    mut src: *const CffSubrGraph,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CffSubrGraph>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_move(mut dst: *mut CffSubrGraph, mut src: *mut CffSubrGraph) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CffSubrGraph>() as usize,
    );
    cff_SubrGraph_init(src);
}
#[inline]
unsafe extern "C" fn cff_SubrGraph_replace(mut dst: *mut CffSubrGraph, src: CffSubrGraph) {
    cff_SubrGraph_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CffSubrGraph>() as usize,
    );
}
unsafe extern "C" fn getSingletHashKey(
    mut n: *mut CffSubrNode,
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
    mut n: *mut CffSubrNode,
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
unsafe extern "C" fn lastNodeOf(mut r: *mut CffSubrRule) -> *mut CffSubrNode {
    return (*(*r).guard).prev;
}
unsafe extern "C" fn copyNode(mut n: *mut CffSubrNode) -> *mut CffSubrNode {
    let mut m: *mut CffSubrNode = cff_new_Node();
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
unsafe extern "C" fn unlinkNode(mut g: *mut CffSubrGraph, mut a: *mut CffSubrNode) {
    if (*a).hard as ::core::ffi::c_int != 0 || (*a).guard as ::core::ffi::c_int != 0 {
        return;
    }
    let mut len: usize = 0;
    let mut key: *mut u8 = getDoubletHashKey(a, &raw mut len);
    let mut di: *mut CffSubrDiagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                    as *mut ::core::ffi::c_void as *mut CffSubrDiagramIndex
                    as *mut CffSubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                        as *mut CffSubrDiagramIndex
                        as *mut CffSubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
                }
            }
        }
    }
    if !di.is_null() && (*di).start == a {
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*di).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*(*g).diagramIndex).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*g).diagramIndex).hh.tbl as *mut ::core::ffi::c_void);
            (*g).diagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*(*g).diagramIndex).hh.tbl).tail {
                (*(*(*g).diagramIndex).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut UtHashHandle as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh2 = (*_hd_hh_del).next;
            } else {
                (*g).diagramIndex =
                    (*_hd_hh_del).next as *mut CffSubrDiagramIndex as *mut CffSubrDiagramIndex;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh3 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*g).diagramIndex).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket = (*(*(*g).diagramIndex).hh.tbl)
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
            (*(*(*g).diagramIndex).hh.tbl).num_items =
                (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_sub(1);
        }
        free((*di).key as *mut ::core::ffi::c_void);
        (*di).key = ::core::ptr::null_mut::<u8>();
        free(di as *mut ::core::ffi::c_void);
        di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
    }
    free(key as *mut ::core::ffi::c_void);
    key = ::core::ptr::null_mut::<u8>();
    key = getSingletHashKey(a, &raw mut len);
    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                    as *mut ::core::ffi::c_void as *mut CffSubrDiagramIndex
                    as *mut CffSubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                        as *mut CffSubrDiagramIndex
                        as *mut CffSubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
                }
            }
        }
    }
    if !di.is_null() && (*di).start == a {
        let mut _hd_hh_del_0: *mut UtHashHandle = &raw mut (*di).hh;
        if (*_hd_hh_del_0).prev.is_null() && (*_hd_hh_del_0).next.is_null() {
            free((*(*(*g).diagramIndex).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*(*g).diagramIndex).hh.tbl as *mut ::core::ffi::c_void);
            (*g).diagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
        } else {
            let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
            if _hd_hh_del_0 == (*(*(*g).diagramIndex).hh.tbl).tail {
                (*(*(*g).diagramIndex).hh.tbl).tail =
                    ((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                        as *mut UtHashHandle as *mut UtHashHandle;
            }
            if !(*_hd_hh_del_0).prev.is_null() {
                let ref mut fresh4 = (*(((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh4 = (*_hd_hh_del_0).next;
            } else {
                (*g).diagramIndex =
                    (*_hd_hh_del_0).next as *mut CffSubrDiagramIndex as *mut CffSubrDiagramIndex;
            }
            if !(*_hd_hh_del_0).next.is_null() {
                let ref mut fresh5 = (*(((*_hd_hh_del_0).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*g).diagramIndex).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh5 = (*_hd_hh_del_0).prev;
            }
            _hd_bkt_0 = (*_hd_hh_del_0).hashv
                & (*(*(*g).diagramIndex).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head_0: *mut UtHashBucket = (*(*(*g).diagramIndex).hh.tbl)
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
            (*(*(*g).diagramIndex).hh.tbl).num_items =
                (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_sub(1);
        }
        free((*di).key as *mut ::core::ffi::c_void);
        (*di).key = ::core::ptr::null_mut::<u8>();
        free(di as *mut ::core::ffi::c_void);
        di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
    }
    free(key as *mut ::core::ffi::c_void);
    key = ::core::ptr::null_mut::<u8>();
}
unsafe extern "C" fn addDoublet(mut g: *mut CffSubrGraph, mut n: *mut CffSubrNode) {
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
    let mut di: *mut CffSubrDiagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                    as *mut ::core::ffi::c_void as *mut CffSubrDiagramIndex
                    as *mut CffSubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                        as *mut CffSubrDiagramIndex
                        as *mut CffSubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
                }
            }
        }
    }
    if di.is_null() {
        di = __caryll_allocate_clean(
            ::core::mem::size_of::<CffSubrDiagramIndex>() as usize,
            232 as ::core::ffi::c_ulong,
        ) as *mut CffSubrDiagramIndex;
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
            (*di).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*di).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*di).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*di).hh.tbl).tail = &raw mut (*di).hh as *mut UtHashHandle;
                (*(*di).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*di).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*di).hh.tbl).hho = (&raw mut (*di).hh as *mut ::core::ffi::c_char)
                    .offset_from(di as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*di).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*di).hh.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*di).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
            (*(*(*g).diagramIndex).hh.tbl).tail = &raw mut (*di).hh as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*g).diagramIndex).hh.tbl).num_items =
            (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket = (*(*(*g).diagramIndex).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UtHashBucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*di).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
        (*di).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*di).hh as *mut UtHashHandle;
        }
        (*_ha_head).hh_head = &raw mut (*di).hh as *mut UtHashHandle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*di).hh.tbl).noexpand == 0
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
                    .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
                        as *mut UtHashHandle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*di).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*di).hh.tbl).ideal_chain_maxlen {
                            (*(*di).hh.tbl).nonideal_items =
                                (*(*di).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*di).hh.tbl).ideal_chain_maxlen);
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
unsafe extern "C" fn addSinglet(mut g: *mut CffSubrGraph, mut n: *mut CffSubrNode) {
    if n.is_null() || (*n).guard as ::core::ffi::c_int != 0 || (*n).hard as ::core::ffi::c_int != 0
    {
        return;
    }
    let mut len: usize = 0;
    let mut key: *mut u8 = getSingletHashKey(n, &raw mut len);
    let mut di: *mut CffSubrDiagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                    as *mut ::core::ffi::c_void as *mut CffSubrDiagramIndex
                    as *mut CffSubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                        as *mut CffSubrDiagramIndex
                        as *mut CffSubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
                }
            }
        }
    }
    if di.is_null() {
        di = __caryll_allocate_clean(
            ::core::mem::size_of::<CffSubrDiagramIndex>() as usize,
            249 as ::core::ffi::c_ulong,
        ) as *mut CffSubrDiagramIndex;
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
            (*di).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*di).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*di).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*di).hh.tbl).tail = &raw mut (*di).hh as *mut UtHashHandle;
                (*(*di).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*di).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*di).hh.tbl).hho = (&raw mut (*di).hh as *mut ::core::ffi::c_char)
                    .offset_from(di as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*di).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*di).hh.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*di).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
            (*(*(*g).diagramIndex).hh.tbl).tail = &raw mut (*di).hh as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*g).diagramIndex).hh.tbl).num_items =
            (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket = (*(*(*g).diagramIndex).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UtHashBucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*di).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
        (*di).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*di).hh as *mut UtHashHandle;
        }
        (*_ha_head).hh_head = &raw mut (*di).hh as *mut UtHashHandle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*di).hh.tbl).noexpand == 0
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
                    .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
                        as *mut UtHashHandle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*di).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*di).hh.tbl).ideal_chain_maxlen {
                            (*(*di).hh.tbl).nonideal_items =
                                (*(*di).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*di).hh.tbl).ideal_chain_maxlen);
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
unsafe extern "C" fn identNode(mut m: *mut CffSubrNode, mut n: *mut CffSubrNode) -> bool {
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
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut n: *mut CffSubrNode,
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
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut n: *mut CffSubrNode,
) {
    joinNodes(g, n, (*m).next);
    joinNodes(g, m, n);
}
unsafe extern "C" fn removeNodeFromGraph(mut g: *mut CffSubrGraph, mut a: *mut CffSubrNode) {
    joinNodes(g, (*a).prev, (*a).next);
    if !(*a).guard {
        unlinkNode(g, a);
        delete_Node(a);
    }
}
unsafe extern "C" fn expandCall(mut g: *mut CffSubrGraph, mut a: *mut CffSubrNode) {
    let mut aprev: *mut CffSubrNode = (*a).prev;
    let mut anext: *mut CffSubrNode = (*a).next;
    let mut r: *mut CffSubrRule = (*a).rule;
    let mut r1: *mut CffSubrNode = (*(*r).guard).next;
    let mut r2: *mut CffSubrNode = (*(*r).guard).prev;
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
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut r: *mut CffSubrRule,
) {
    let mut prev: *mut CffSubrNode = (*m).prev;
    removeNodeFromGraph(g, (*prev).next);
    removeNodeFromGraph(g, (*prev).next);
    let mut invoke: *mut CffSubrNode = cff_new_Node();
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
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut r: *mut CffSubrRule,
) {
    let mut prev: *mut CffSubrNode = (*m).prev;
    removeNodeFromGraph(g, (*prev).next);
    let mut invoke: *mut CffSubrNode = cff_new_Node();
    (*invoke).rule = r;
    (*(*invoke).rule).refcount = (*(*invoke).rule).refcount.wrapping_add(1 as u32);
    xInsertNodeAfter(g, prev, invoke);
    addDoublet(g, prev);
    addDoublet(g, invoke);
    addSinglet(g, invoke);
}
unsafe extern "C" fn processMatchDoublet(
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut n: *mut CffSubrNode,
) {
    let mut rule: *mut CffSubrRule = ::core::ptr::null_mut::<CffSubrRule>();
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
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut n: *mut CffSubrNode,
) {
    let mut rule: *mut CffSubrRule = ::core::ptr::null_mut::<CffSubrRule>();
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
    mut g: *mut CffSubrGraph,
    mut n: *mut CffSubrNode,
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
    let mut di: *mut CffSubrDiagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                    as *mut ::core::ffi::c_void as *mut CffSubrDiagramIndex
                    as *mut CffSubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                        as *mut CffSubrDiagramIndex
                        as *mut CffSubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
                }
            }
        }
    }
    if di.is_null() {
        di = __caryll_allocate_clean(
            ::core::mem::size_of::<CffSubrDiagramIndex>() as usize,
            390 as ::core::ffi::c_ulong,
        ) as *mut CffSubrDiagramIndex;
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
            (*di).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*di).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*di).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*di).hh.tbl).tail = &raw mut (*di).hh as *mut UtHashHandle;
                (*(*di).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*di).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*di).hh.tbl).hho = (&raw mut (*di).hh as *mut ::core::ffi::c_char)
                    .offset_from(di as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*di).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*di).hh.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*di).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
            (*(*(*g).diagramIndex).hh.tbl).tail = &raw mut (*di).hh as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*g).diagramIndex).hh.tbl).num_items =
            (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket = (*(*(*g).diagramIndex).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UtHashBucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*di).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
        (*di).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*di).hh as *mut UtHashHandle;
        }
        (*_ha_head).hh_head = &raw mut (*di).hh as *mut UtHashHandle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*di).hh.tbl).noexpand == 0
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
                    .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
                        as *mut UtHashHandle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*di).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*di).hh.tbl).ideal_chain_maxlen {
                            (*(*di).hh.tbl).nonideal_items =
                                (*(*di).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*di).hh.tbl).ideal_chain_maxlen);
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
    mut g: *mut CffSubrGraph,
    mut n: *mut CffSubrNode,
) -> bool {
    if (*n).guard as ::core::ffi::c_int != 0 || (*n).hard as ::core::ffi::c_int != 0 {
        return false;
    }
    let mut len: usize = 0;
    let mut key: *mut u8 = getSingletHashKey(n, &raw mut len);
    let mut di: *mut CffSubrDiagramIndex = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                    as *mut ::core::ffi::c_void as *mut CffSubrDiagramIndex
                    as *mut CffSubrDiagramIndex;
            } else {
                di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
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
                        as *mut CffSubrDiagramIndex
                        as *mut CffSubrDiagramIndex;
                } else {
                    di = ::core::ptr::null_mut::<CffSubrDiagramIndex>();
                }
            }
        }
    }
    if di.is_null() {
        di = __caryll_allocate_clean(
            ::core::mem::size_of::<CffSubrDiagramIndex>() as usize,
            413 as ::core::ffi::c_ulong,
        ) as *mut CffSubrDiagramIndex;
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
            (*di).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                as *mut UtHashTable as *mut UtHashTable;
            if (*di).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*di).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UtHashTable>() as usize,
                );
                (*(*di).hh.tbl).tail = &raw mut (*di).hh as *mut UtHashHandle;
                (*(*di).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*di).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*di).hh.tbl).hho = (&raw mut (*di).hh as *mut ::core::ffi::c_char)
                    .offset_from(di as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*di).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                (*(*di).hh.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*di).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*di).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
            (*(*(*g).diagramIndex).hh.tbl).tail = &raw mut (*di).hh as *mut UtHashHandle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*g).diagramIndex).hh.tbl).num_items =
            (*(*(*g).diagramIndex).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*g).diagramIndex).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UtHashBucket = (*(*(*g).diagramIndex).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UtHashBucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*di).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
        (*di).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*di).hh as *mut UtHashHandle;
        }
        (*_ha_head).hh_head = &raw mut (*di).hh as *mut UtHashHandle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*di).hh.tbl).noexpand == 0
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
                    .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
            ) as *mut UtHashBucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*di).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
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
                        as *mut UtHashHandle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*di).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*di).hh.tbl).ideal_chain_maxlen {
                            (*(*di).hh.tbl).nonideal_items =
                                (*(*di).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*di).hh.tbl).ideal_chain_maxlen);
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
unsafe extern "C" fn appendNodeToGraph(mut g: *mut CffSubrGraph, mut n: *mut CffSubrNode) {
    let mut last: *mut CffSubrNode = lastNodeOf((*g).root);
    xInsertNodeAfter(g, last, n);
    if (*g).doSubroutinize {
        if !checkDoubletMatch(g, last) {
            if buflen((*n).terminal) > 15 as usize {
                checkSingletMatch(g, n);
            }
        }
    }
}
pub unsafe extern "C" fn cff_insertILToGraph(
    mut g: *mut CffSubrGraph,
    mut il: *mut CffCharstringIl,
) {
    let mut blob: *mut Buffer = bufnew();
    let mut flush: bool = false;
    let mut last: bool = false;
    let mut j: u32 = 0 as u32;
    while j < (*il).length {
        match (*(*il).instr.offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                if flush {
                    let mut n: *mut CffSubrNode = cff_new_Node();
                    (*n).rule = ::core::ptr::null_mut::<CffSubrRule>();
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
                    == op_endchar
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
        let mut n_0: *mut CffSubrNode = cff_new_Node();
        (*n_0).rule = ::core::ptr::null_mut::<CffSubrRule>();
        (*n_0).last = last;
        (*n_0).terminal = blob;
        appendNodeToGraph(g, n_0);
    }
    blob = bufnew();
    let mut n_1: *mut CffSubrNode = cff_new_Node();
    (*n_1).rule = ::core::ptr::null_mut::<CffSubrRule>();
    (*n_1).terminal = blob;
    (*n_1).hard = true;
    appendNodeToGraph(g, n_1);
    (*g).totalCharStrings = (*g).totalCharStrings.wrapping_add(1 as u32);
}
unsafe extern "C" fn cff_statHeight(mut r: *mut CffSubrRule, mut height: u32) {
    if height > (*r).height {
        (*r).height = height;
    }
    let mut effectiveLength: u32 = 0 as u32;
    let mut e: *mut CffSubrNode = (*(*r).guard).next;
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
unsafe extern "C" fn numberASubroutine(mut r: *mut CffSubrRule, mut current: *mut u32) {
    if (*r).numbered {
        return;
    }
    if (*r).height >= type2_subr_nesting {
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
    let mut e: *mut CffSubrNode = (*(*r).guard).next;
    while e != (*r).guard {
        if !(*e).rule.is_null() {
            numberASubroutine((*e).rule, current);
        }
        e = (*e).next;
    }
}
unsafe extern "C" fn cff_numberSubroutines(mut g: *mut CffSubrGraph) -> u32 {
    let mut current: u32 = 0 as u32;
    let mut e: *mut CffSubrNode = (*(*(*g).root).guard).next;
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
unsafe extern "C" fn endsWithEndChar(mut rule: *mut CffSubrRule) -> bool {
    let mut node: *mut CffSubrNode = lastNodeOf(rule);
    if !(*node).terminal.is_null() {
        return (*node).last;
    } else {
        return endsWithEndChar((*node).rule);
    };
}
unsafe extern "C" fn serializeNodeToBuffer(
    mut node: *mut CffSubrNode,
    mut buf: *mut Buffer,
    mut gsubrs: *mut Buffer,
    mut maxGSubrs: u32,
    mut lsubrs: *mut Buffer,
    mut maxLSubrs: u32,
) {
    if !(*node).rule.is_null() {
        if (*(*node).rule).numbered as ::core::ffi::c_int != 0
            && (*(*node).rule).number < maxLSubrs.wrapping_add(maxGSubrs)
            && (*(*node).rule).height < type2_subr_nesting
        {
            let mut target: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
            if (*(*node).rule).number < maxLSubrs {
                let mut stacknum: i32 = (*(*node).rule)
                    .number
                    .wrapping_sub(subroutineBias(maxLSubrs as i32) as u32)
                    as i32;
                target = lsubrs.offset((*(*node).rule).number as isize);
                cff_mergeCS2Int(buf, stacknum);
                cff_mergeCS2Operator(buf, op_callsubr);
            } else {
                let mut stacknum_0: i32 = (*(*node).rule)
                    .number
                    .wrapping_sub(maxLSubrs)
                    .wrapping_sub(subroutineBias(maxGSubrs as i32) as u32)
                    as i32;
                target = gsubrs.offset((*(*node).rule).number.wrapping_sub(maxLSubrs) as isize);
                cff_mergeCS2Int(buf, stacknum_0);
                cff_mergeCS2Operator(buf, op_callgsubr);
            }
            let mut r: *mut CffSubrRule = (*node).rule;
            if !(*r).printed {
                (*r).printed = true;
                let mut e: *mut CffSubrNode = (*(*r).guard).next;
                while e != (*r).guard {
                    serializeNodeToBuffer(e, target, gsubrs, maxGSubrs, lsubrs, maxLSubrs);
                    e = (*e).next;
                }
                if !endsWithEndChar(r) {
                    cff_mergeCS2Operator(target, op_return);
                }
            }
        } else {
            let mut r_0: *mut CffSubrRule = (*node).rule;
            let mut e_0: *mut CffSubrNode = (*(*r_0).guard).next;
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
) -> *mut Buffer {
    let mut context: *mut Buffer = _context as *mut Buffer;
    let mut blob: *mut Buffer = bufnew();
    bufwrite_buf(blob, context.offset(j as isize));
    return blob;
}
pub unsafe extern "C" fn cff_ilGraphToBuffers(
    mut g: *mut CffSubrGraph,
    mut s: *mut *mut Buffer,
    mut gs: *mut *mut Buffer,
    mut ls: *mut *mut Buffer,
    mut options: *const Options,
) {
    cff_statHeight((*g).root, 0 as u32);
    let mut maxSubroutines: u32 = cff_numberSubroutines(g);
    (*(*options).logger)
        .logSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        log_vl_progress,
        log_type_progress,
        crate::sdsbuild!(sdsempty(), b"[libcff] Total ", maxSubroutines, b" subroutines extracted."),
    );
    let mut maxLSubrs: u32 = maxSubroutines;
    let mut maxGSubrs: u32 = 0 as u32;
    if maxLSubrs > type2_max_subrs {
        maxLSubrs = type2_max_subrs;
        maxGSubrs = maxSubroutines.wrapping_sub(maxLSubrs);
    }
    if maxGSubrs > type2_max_subrs {
        maxGSubrs = type2_max_subrs;
    }
    let mut total: u32 = maxLSubrs.wrapping_add(maxGSubrs);
    maxLSubrs = total.wrapping_div(2 as u32);
    maxGSubrs = total.wrapping_sub(maxLSubrs);
    let mut charStrings: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    let mut gsubrs: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    let mut lsubrs: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    charStrings = __caryll_allocate_clean(
        (::core::mem::size_of::<Buffer>() as usize)
            .wrapping_mul((*g).totalCharStrings.wrapping_add(1 as u32) as usize),
        608 as ::core::ffi::c_ulong,
    ) as *mut Buffer;
    lsubrs = __caryll_allocate_clean(
        (::core::mem::size_of::<Buffer>() as usize)
            .wrapping_mul(maxLSubrs.wrapping_add(1 as u32) as usize),
        609 as ::core::ffi::c_ulong,
    ) as *mut Buffer;
    gsubrs = __caryll_allocate_clean(
        (::core::mem::size_of::<Buffer>() as usize)
            .wrapping_mul(maxGSubrs.wrapping_add(1 as u32) as usize),
        610 as ::core::ffi::c_ulong,
    ) as *mut Buffer;
    let mut j: u32 = 0 as u32;
    let mut r: *mut CffSubrRule = (*g).root;
    let mut e: *mut CffSubrNode = (*(*r).guard).next;
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
    let mut is: *mut CffIndex = cff_iIndex.fromCallback.expect("non-null function pointer")(
        charStrings as *mut ::core::ffi::c_void,
        (*g).totalCharStrings,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
        ),
    );
    let mut igs: *mut CffIndex = cff_iIndex.fromCallback.expect("non-null function pointer")(
        gsubrs as *mut ::core::ffi::c_void,
        maxGSubrs,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
        ),
    );
    let mut ils: *mut CffIndex = cff_iIndex.fromCallback.expect("non-null function pointer")(
        lsubrs as *mut ::core::ffi::c_void,
        maxLSubrs,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
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
    charStrings = ::core::ptr::null_mut::<Buffer>();
    free(gsubrs as *mut ::core::ffi::c_void);
    gsubrs = ::core::ptr::null_mut::<Buffer>();
    free(lsubrs as *mut ::core::ffi::c_void);
    lsubrs = ::core::ptr::null_mut::<Buffer>();
    *s = cff_iIndex.build.expect("non-null function pointer")(is);
    *gs = cff_iIndex.build.expect("non-null function pointer")(igs);
    *ls = cff_iIndex.build.expect("non-null function pointer")(ils);
    cff_iIndex.free.expect("non-null function pointer")(is);
    cff_iIndex.free.expect("non-null function pointer")(igs);
    cff_iIndex.free.expect("non-null function pointer")(ils);
}
