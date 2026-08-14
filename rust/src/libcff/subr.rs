#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, strncmp};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{LoggerType, LOG_VL_PROGRESS, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};

use crate::libcff::CffCharstringOperator;
use crate::libcff::{OP_CALLGSUBR, OP_CALLSUBR, OP_ENDCHAR, OP_RETURN, TYPE2_MAX_SUBRS, TYPE2_SUBR_NESTING};
use crate::libcff::cff_index::CffIndex;
use crate::libcff::charstring_il::{CffCharstringIl};
use crate::libcff::cff_index::{CFF_I_INDEX};
use crate::libcff::cff_writer::{cff_merge_cs2_int, cff_merge_cs2_operand, cff_merge_cs2_operator, cff_merge_cs2_special};
use crate::support::buffer::{buffree, buflen, bufnew, bufwrite_buf};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffSubrRule {
    pub printed: bool,
    pub numbered: bool,
    pub number: u32,
    pub height: u32,
    pub unique_index: u32,
    pub cff_index: u16,
    pub refcount: u32,
    pub effective_length: u32,
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
/// Replaces the uthash-based `CffSubrDiagramIndex` (which also carried
/// `key: *mut u8` and `hh: UtHashHandle`, both now subsumed by
/// `HashMap`'s own key/bucket machinery). `key` was write-only outside
/// insertion/disposal (grepped: never read for anything but the hash
/// itself), so it needs no home in the value at all -- the two fields
/// that were actually read back (`arity`, `start`) are all that remain.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffSubrDiagramIndexEntry {
    pub arity: u8,
    pub start: *mut CffSubrNode,
}
/// `diagram_index` holds both "singlet" (arity 1) and "doublet" (arity 2)
/// entries in one table, keyed by a variable-length byte fingerprint
/// (`get_singlet_hash_key`/`get_doublet_hash_key`) whose first byte
/// ('1' vs '2') keeps the two arities from ever colliding -- order never
/// matters (no `HASH_SORT`, and the only whole-table walk is disposal),
/// so `HashMap` applies directly.
#[repr(C)]
pub struct CffSubrGraph {
    pub root: *mut CffSubrRule,
    pub last: *mut CffSubrRule,
    pub diagram_index: std::collections::HashMap<Vec<u8>, CffSubrDiagramIndexEntry>,
    pub total_rules: u32,
    pub total_char_strings: u32,
    pub do_subroutinize: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffSubrGraphElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CffSubrGraph) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CffSubrGraph) -> ()>,
}
unsafe extern "C" fn cff_new_node() -> *mut CffSubrNode {
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
unsafe extern "C" fn cff_new_rule() -> *mut CffSubrRule {
    let mut r: *mut CffSubrRule = ::core::ptr::null_mut::<CffSubrRule>();
    r = __caryll_allocate_clean(
        ::core::mem::size_of::<CffSubrRule>() as usize,
        34 as ::core::ffi::c_ulong,
    ) as *mut CffSubrRule;
    (*r).refcount = 0 as u32;
    (*r).guard = cff_new_node();
    (*(*r).guard).prev = (*r).guard;
    (*(*r).guard).next = (*r).guard;
    (*(*r).guard).terminal = ::core::ptr::null_mut::<Buffer>();
    (*(*r).guard).guard = true;
    (*(*r).guard).rule = r;
    (*r).next = ::core::ptr::null_mut::<CffSubrRule>();
    return r;
}
unsafe extern "C" fn init_subr_graph(mut g: *mut CffSubrGraph) {
    (*g).root = cff_new_rule();
    (*g).last = (*g).root;
    (*g).diagram_index = std::collections::HashMap::new();
    (*g).total_rules = 0 as u32;
    (*g).total_char_strings = 0 as u32;
    (*g).do_subroutinize = false;
}
unsafe extern "C" fn clean_node(mut x: *mut CffSubrNode) {
    if !(*x).rule.is_null() {
        (*(*x).rule).refcount = (*(*x).rule).refcount.wrapping_sub(1 as u32);
    }
    (*x).rule = ::core::ptr::null_mut::<CffSubrRule>();
    buffree((*x).terminal);
    (*x).terminal = ::core::ptr::null_mut::<Buffer>();
}
unsafe extern "C" fn delete_node(mut x: *mut CffSubrNode) {
    if x.is_null() {
        return;
    }
    clean_node(x);
    free(x as *mut ::core::ffi::c_void);
    x = ::core::ptr::null_mut::<CffSubrNode>();
}
unsafe extern "C" fn delete_full_rule(mut r: *mut CffSubrRule) {
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
unsafe extern "C" fn dispose_subr_graph(mut g: *mut CffSubrGraph) {
    let mut r: *mut CffSubrRule = (*g).root;
    while !r.is_null() {
        let mut next: *mut CffSubrRule = (*r).next;
        delete_full_rule(r);
        r = next;
    }
    // The values own nothing (both fields are `Copy`), so dropping the map
    // is the whole disposal -- no manual `HASH_ITER`+`HASH_DEL`+`free` walk
    // needed.
    (*g).diagram_index = std::collections::HashMap::new();
}
pub static CFF_I_SUBR_GRAPH: CffSubrGraphElementInterface = {
    CffSubrGraphElementInterface {
        init: Some(cff_subr_graph_init as unsafe extern "C" fn(*mut CffSubrGraph) -> ()),
        dispose: Some(cff_subr_graph_dispose as unsafe extern "C" fn(*mut CffSubrGraph) -> ()),
    }
};
#[inline]
unsafe extern "C" fn cff_subr_graph_init(mut x: *mut CffSubrGraph) {
    init_subr_graph(x);
}
#[inline]
unsafe extern "C" fn cff_subr_graph_dispose(mut x: *mut CffSubrGraph) {
    dispose_subr_graph(x);
}
/// The byte layout (header bytes, payload, trailing NUL) matches the
/// original `malloc`+`memcpy` construction exactly, so keys built here
/// compare equal to keys built there -- the leading `'1'`/`'2'` (singlet
/// vs doublet) keeps the two arities from ever colliding in one table.
unsafe fn get_singlet_hash_key(mut n: *mut CffSubrNode) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();
    key.push(b'1');
    key.push(if !(*n).rule.is_null() { b'1' } else { b'0' });
    key.push(b'0');
    if !(*n).rule.is_null() {
        key.extend_from_slice(&(*(*n).rule).unique_index.to_ne_bytes());
    } else {
        key.extend_from_slice(std::slice::from_raw_parts(
            (*(*n).terminal).data,
            buflen((*n).terminal),
        ));
    }
    key.push(0);
    key
}
unsafe fn get_doublet_hash_key(mut n: *mut CffSubrNode) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();
    key.push(b'2');
    key.push(if !(*n).rule.is_null() { b'1' } else { b'0' });
    key.push(if !(*(*n).next).rule.is_null() { b'1' } else { b'0' });
    if !(*n).rule.is_null() {
        key.extend_from_slice(&(*(*n).rule).unique_index.to_ne_bytes());
    } else {
        key.extend_from_slice(std::slice::from_raw_parts(
            (*(*n).terminal).data,
            buflen((*n).terminal),
        ));
    }
    if !(*(*n).next).rule.is_null() {
        key.extend_from_slice(&(*(*(*n).next).rule).unique_index.to_ne_bytes());
    } else {
        key.extend_from_slice(std::slice::from_raw_parts(
            (*(*(*n).next).terminal).data,
            buflen((*(*n).next).terminal),
        ));
    }
    key.push(0);
    key
}
unsafe extern "C" fn last_node_of(mut r: *mut CffSubrRule) -> *mut CffSubrNode {
    return (*(*r).guard).prev;
}
unsafe extern "C" fn copy_node(mut n: *mut CffSubrNode) -> *mut CffSubrNode {
    let mut m: *mut CffSubrNode = cff_new_node();
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
unsafe extern "C" fn unlink_node(mut g: *mut CffSubrGraph, mut a: *mut CffSubrNode) {
    if (*a).hard || (*a).guard {
        return;
    }
    // Only drop an index entry if it is still pointing at the node being
    // unlinked -- some *other* node may have since claimed this key's
    // slot, and that entry must be left alone.
    let doublet_key = get_doublet_hash_key(a);
    if (*g).diagram_index.get(&doublet_key).is_some_and(|di| di.start == a) {
        (*g).diagram_index.remove(&doublet_key);
    }
    let singlet_key = get_singlet_hash_key(a);
    if (*g).diagram_index.get(&singlet_key).is_some_and(|di| di.start == a) {
        (*g).diagram_index.remove(&singlet_key);
    }
}
unsafe extern "C" fn add_doublet(mut g: *mut CffSubrGraph, mut n: *mut CffSubrNode) {
    if n.is_null()
        || (*n).next.is_null()
        || (*n).guard
        || (*n).hard
        || (*(*n).next).hard
        || (*(*n).next).guard
    {
        return;
    }
    let key = get_doublet_hash_key(n);
    (*g)
        .diagram_index
        .insert(key, CffSubrDiagramIndexEntry { arity: 2, start: n });
}
unsafe extern "C" fn add_singlet(mut g: *mut CffSubrGraph, mut n: *mut CffSubrNode) {
    if n.is_null() || (*n).guard || (*n).hard {
        return;
    }
    let key = get_singlet_hash_key(n);
    (*g)
        .diagram_index
        .insert(key, CffSubrDiagramIndexEntry { arity: 1, start: n });
}
unsafe extern "C" fn ident_node(mut m: *mut CffSubrNode, mut n: *mut CffSubrNode) -> bool {
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
unsafe extern "C" fn join_nodes(
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut n: *mut CffSubrNode,
) {
    if !(*m).next.is_null() {
        unlink_node(g, m);
        if !(*n).prev.is_null()
            && !(*n).next.is_null()
            && ident_node((*n).prev, n) as ::core::ffi::c_int != 0
            && ident_node(n, (*n).next) as ::core::ffi::c_int != 0
        {
            add_doublet(g, n);
        }
        if !(*m).prev.is_null()
            && !(*m).next.is_null()
            && ident_node((*m).prev, m) as ::core::ffi::c_int != 0
            && ident_node(m, (*m).next) as ::core::ffi::c_int != 0
        {
            add_doublet(g, (*m).prev);
        }
    }
    (*m).next = n;
    (*n).prev = m;
}
unsafe extern "C" fn x_insert_node_after(
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut n: *mut CffSubrNode,
) {
    join_nodes(g, n, (*m).next);
    join_nodes(g, m, n);
}
unsafe extern "C" fn remove_node_from_graph(mut g: *mut CffSubrGraph, mut a: *mut CffSubrNode) {
    join_nodes(g, (*a).prev, (*a).next);
    if !(*a).guard {
        unlink_node(g, a);
        delete_node(a);
    }
}
unsafe extern "C" fn expand_call(mut g: *mut CffSubrGraph, mut a: *mut CffSubrNode) {
    let mut aprev: *mut CffSubrNode = (*a).prev;
    let mut anext: *mut CffSubrNode = (*a).next;
    let mut r: *mut CffSubrRule = (*a).rule;
    let mut r1: *mut CffSubrNode = (*(*r).guard).next;
    let mut r2: *mut CffSubrNode = (*(*r).guard).prev;
    unlink_node(g, a);
    join_nodes(g, aprev, r1);
    join_nodes(g, r2, anext);
    add_doublet(g, r2);
    (*(*r).guard).next = (*r).guard;
    (*(*r).guard).prev = (*(*r).guard).next;
    (*r).refcount = (*r).refcount.wrapping_sub(1 as u32);
    delete_node(a);
}
unsafe extern "C" fn substitute_doublet_with_rule(
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut r: *mut CffSubrRule,
) {
    let mut prev: *mut CffSubrNode = (*m).prev;
    remove_node_from_graph(g, (*prev).next);
    remove_node_from_graph(g, (*prev).next);
    let mut invoke: *mut CffSubrNode = cff_new_node();
    (*invoke).rule = r;
    (*(*invoke).rule).refcount = (*(*invoke).rule).refcount.wrapping_add(1 as u32);
    x_insert_node_after(g, prev, invoke);
    add_doublet(g, prev);
    add_doublet(g, invoke);
    add_singlet(g, invoke);
    if !check_doublet_match(g, prev) {
        check_doublet_match(g, (*prev).next);
    }
}
unsafe extern "C" fn substitute_singlet_with_rule(
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut r: *mut CffSubrRule,
) {
    let mut prev: *mut CffSubrNode = (*m).prev;
    remove_node_from_graph(g, (*prev).next);
    let mut invoke: *mut CffSubrNode = cff_new_node();
    (*invoke).rule = r;
    (*(*invoke).rule).refcount = (*(*invoke).rule).refcount.wrapping_add(1 as u32);
    x_insert_node_after(g, prev, invoke);
    add_doublet(g, prev);
    add_doublet(g, invoke);
    add_singlet(g, invoke);
}
unsafe extern "C" fn process_match_doublet(
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut n: *mut CffSubrNode,
) {
    let mut rule: *mut CffSubrRule = ::core::ptr::null_mut::<CffSubrRule>();
    if (*(*m).prev).guard as ::core::ffi::c_int != 0
        && (*(*(*m).next).next).guard as ::core::ffi::c_int != 0
    {
        rule = (*(*m).prev).rule;
        substitute_doublet_with_rule(g, n, rule);
    } else {
        rule = cff_new_rule();
        (*rule).unique_index = (*g).total_rules;
        (*g).total_rules = (*g).total_rules.wrapping_add(1 as u32);
        (*(*g).last).next = rule;
        (*g).last = rule;
        x_insert_node_after(g, last_node_of(rule), copy_node(m));
        x_insert_node_after(g, last_node_of(rule), copy_node((*m).next));
        substitute_doublet_with_rule(g, m, rule);
        substitute_doublet_with_rule(g, n, rule);
        add_doublet(g, (*(*rule).guard).next);
        add_singlet(g, (*(*rule).guard).next);
        add_singlet(g, (*(*(*rule).guard).next).next);
    }
    if !(*(*(*rule).guard).next).rule.is_null()
        && (*(*(*(*rule).guard).next).rule).refcount == 1 as u32
    {
        expand_call(g, (*(*rule).guard).next);
    }
}
unsafe extern "C" fn process_match_singlet(
    mut g: *mut CffSubrGraph,
    mut m: *mut CffSubrNode,
    mut n: *mut CffSubrNode,
) {
    let mut rule: *mut CffSubrRule = ::core::ptr::null_mut::<CffSubrRule>();
    if (*(*m).prev).guard as ::core::ffi::c_int != 0
        && (*(*m).next).guard as ::core::ffi::c_int != 0
    {
        rule = (*(*m).prev).rule;
        substitute_singlet_with_rule(g, n, rule);
    } else {
        rule = cff_new_rule();
        (*rule).unique_index = (*g).total_rules;
        (*g).total_rules = (*g).total_rules.wrapping_add(1 as u32);
        (*(*g).last).next = rule;
        (*g).last = rule;
        x_insert_node_after(g, last_node_of(rule), copy_node(m));
        substitute_singlet_with_rule(g, m, rule);
        substitute_singlet_with_rule(g, n, rule);
        add_singlet(g, (*(*rule).guard).next);
    };
}
unsafe extern "C" fn check_doublet_match(mut g: *mut CffSubrGraph, mut n: *mut CffSubrNode) -> bool {
    if (*n).guard || (*(*n).next).guard || (*n).hard || (*(*n).next).hard {
        return false;
    }
    let key = get_doublet_hash_key(n);
    match (*g).diagram_index.entry(key) {
        std::collections::hash_map::Entry::Vacant(v) => {
            v.insert(CffSubrDiagramIndexEntry { arity: 2, start: n });
            false
        }
        std::collections::hash_map::Entry::Occupied(o) => {
            let di = *o.get();
            if di.arity == 2 && di.start != n && !(*di.start).guard && !(*(*di.start).next).guard {
                process_match_doublet(g, di.start, n);
                true
            } else {
                true
            }
        }
    }
}
unsafe extern "C" fn check_singlet_match(mut g: *mut CffSubrGraph, mut n: *mut CffSubrNode) -> bool {
    if (*n).guard || (*n).hard {
        return false;
    }
    let key = get_singlet_hash_key(n);
    match (*g).diagram_index.entry(key) {
        std::collections::hash_map::Entry::Vacant(v) => {
            v.insert(CffSubrDiagramIndexEntry { arity: 1, start: n });
            false
        }
        std::collections::hash_map::Entry::Occupied(o) => {
            let di = *o.get();
            if di.arity == 1 && di.start != n && !(*di.start).guard {
                process_match_singlet(g, di.start, n);
                true
            } else {
                false
            }
        }
    }
}
unsafe extern "C" fn append_node_to_graph(mut g: *mut CffSubrGraph, mut n: *mut CffSubrNode) {
    let mut last: *mut CffSubrNode = last_node_of((*g).root);
    x_insert_node_after(g, last, n);
    if (*g).do_subroutinize {
        if !check_doublet_match(g, last) {
            if buflen((*n).terminal) > 15 as usize {
                check_singlet_match(g, n);
            }
        }
    }
}
pub unsafe extern "C" fn cff_insert_il_to_graph(
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
                    let mut n: *mut CffSubrNode = cff_new_node();
                    (*n).rule = ::core::ptr::null_mut::<CffSubrRule>();
                    (*n).terminal = blob;
                    (*n).last = last;
                    append_node_to_graph(g, n);
                    blob = bufnew();
                    flush = false;
                }
                cff_merge_cs2_operand(blob, (*(*il).instr.offset(j as isize)).c2rust_unnamed.d);
            }
            1 => {
                cff_merge_cs2_operator(
                    blob,
                    CffCharstringOperator((*(*il).instr.offset(j as isize)).c2rust_unnamed.i),
                );
                if (*(*il).instr.offset(j as isize)).c2rust_unnamed.i
                    == OP_ENDCHAR.0
                {
                    last = true;
                }
                flush = true;
            }
            2 => {
                cff_merge_cs2_special(
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
        let mut n_0: *mut CffSubrNode = cff_new_node();
        (*n_0).rule = ::core::ptr::null_mut::<CffSubrRule>();
        (*n_0).last = last;
        (*n_0).terminal = blob;
        append_node_to_graph(g, n_0);
    }
    blob = bufnew();
    let mut n_1: *mut CffSubrNode = cff_new_node();
    (*n_1).rule = ::core::ptr::null_mut::<CffSubrRule>();
    (*n_1).terminal = blob;
    (*n_1).hard = true;
    append_node_to_graph(g, n_1);
    (*g).total_char_strings = (*g).total_char_strings.wrapping_add(1 as u32);
}
unsafe extern "C" fn cff_stat_height(mut r: *mut CffSubrRule, mut height: u32) {
    if height > (*r).height {
        (*r).height = height;
    }
    let mut effective_length: u32 = 0 as u32;
    let mut e: *mut CffSubrNode = (*(*r).guard).next;
    while e != (*r).guard {
        if !(*e).rule.is_null() {
            cff_stat_height((*e).rule, height.wrapping_add(1 as u32));
            effective_length = effective_length.wrapping_add(4 as u32);
        } else {
            effective_length = (effective_length as usize).wrapping_add((*(*e).terminal).size)
                as u32 as u32;
        }
        e = (*e).next;
    }
    (*r).effective_length = effective_length;
}
unsafe extern "C" fn number_a_subroutine(mut r: *mut CffSubrRule, mut current: *mut u32) {
    if (*r).numbered {
        return;
    }
    if (*r).height >= TYPE2_SUBR_NESTING {
        return;
    }
    if (*r)
        .effective_length
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
            number_a_subroutine((*e).rule, current);
        }
        e = (*e).next;
    }
}
unsafe extern "C" fn cff_number_subroutines(mut g: *mut CffSubrGraph) -> u32 {
    let mut current: u32 = 0 as u32;
    let mut e: *mut CffSubrNode = (*(*(*g).root).guard).next;
    while e != (*(*g).root).guard {
        if !(*e).rule.is_null() {
            number_a_subroutine((*e).rule, &raw mut current);
        }
        e = (*e).next;
    }
    return current;
}
#[inline]
unsafe extern "C" fn subroutine_bias(mut cnt: i32) -> i32 {
    if cnt < 1240 as i32 {
        return 107 as i32;
    } else if cnt < 33900 as i32 {
        return 1131 as i32;
    } else {
        return 32768 as i32;
    };
}
unsafe extern "C" fn ends_with_end_char(mut rule: *mut CffSubrRule) -> bool {
    let mut node: *mut CffSubrNode = last_node_of(rule);
    if !(*node).terminal.is_null() {
        return (*node).last;
    } else {
        return ends_with_end_char((*node).rule);
    };
}
unsafe extern "C" fn serialize_node_to_buffer(
    mut node: *mut CffSubrNode,
    mut buf: *mut Buffer,
    mut gsubrs: *mut Buffer,
    mut max_g_subrs: u32,
    mut lsubrs: *mut Buffer,
    mut max_l_subrs: u32,
) {
    if !(*node).rule.is_null() {
        if (*(*node).rule).numbered as ::core::ffi::c_int != 0
            && (*(*node).rule).number < max_l_subrs.wrapping_add(max_g_subrs)
            && (*(*node).rule).height < TYPE2_SUBR_NESTING
        {
            let mut target: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
            if (*(*node).rule).number < max_l_subrs {
                let mut stacknum: i32 = (*(*node).rule)
                    .number
                    .wrapping_sub(subroutine_bias(max_l_subrs as i32) as u32)
                    as i32;
                target = lsubrs.offset((*(*node).rule).number as isize);
                cff_merge_cs2_int(buf, stacknum);
                cff_merge_cs2_operator(buf, OP_CALLSUBR);
            } else {
                let mut stacknum_0: i32 = (*(*node).rule)
                    .number
                    .wrapping_sub(max_l_subrs)
                    .wrapping_sub(subroutine_bias(max_g_subrs as i32) as u32)
                    as i32;
                target = gsubrs.offset((*(*node).rule).number.wrapping_sub(max_l_subrs) as isize);
                cff_merge_cs2_int(buf, stacknum_0);
                cff_merge_cs2_operator(buf, OP_CALLGSUBR);
            }
            let mut r: *mut CffSubrRule = (*node).rule;
            if !(*r).printed {
                (*r).printed = true;
                let mut e: *mut CffSubrNode = (*(*r).guard).next;
                while e != (*r).guard {
                    serialize_node_to_buffer(e, target, gsubrs, max_g_subrs, lsubrs, max_l_subrs);
                    e = (*e).next;
                }
                if !ends_with_end_char(r) {
                    cff_merge_cs2_operator(target, OP_RETURN);
                }
            }
        } else {
            let mut r_0: *mut CffSubrRule = (*node).rule;
            let mut e_0: *mut CffSubrNode = (*(*r_0).guard).next;
            while e_0 != (*r_0).guard {
                serialize_node_to_buffer(e_0, buf, gsubrs, max_g_subrs, lsubrs, max_l_subrs);
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
pub unsafe extern "C" fn cff_il_graph_to_buffers(
    mut g: *mut CffSubrGraph,
    mut s: *mut *mut Buffer,
    mut gs: *mut *mut Buffer,
    mut ls: *mut *mut Buffer,
    mut options: *const Options,
) {
    cff_stat_height((*g).root, 0 as u32);
    let mut max_subroutines: u32 = cff_number_subroutines(g);
    (*(*options).logger)
        .log_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        LOG_VL_PROGRESS,
        LoggerType::Progress,
        crate::bytesbuild!(b"[libcff] Total ", max_subroutines, b" subroutines extracted."),
    );
    let mut max_l_subrs: u32 = max_subroutines;
    let mut max_g_subrs: u32 = 0 as u32;
    if max_l_subrs > TYPE2_MAX_SUBRS {
        max_l_subrs = TYPE2_MAX_SUBRS;
        max_g_subrs = max_subroutines.wrapping_sub(max_l_subrs);
    }
    if max_g_subrs > TYPE2_MAX_SUBRS {
        max_g_subrs = TYPE2_MAX_SUBRS;
    }
    let mut total: u32 = max_l_subrs.wrapping_add(max_g_subrs);
    max_l_subrs = total.wrapping_div(2 as u32);
    max_g_subrs = total.wrapping_sub(max_l_subrs);
    // Was three `__caryll_allocate_clean`'d `*mut Buffer` arrays, each freed
    // field-by-field (every `.data`) and then as a whole -- the same
    // "malloc'd scratch array of plain structs" shape already converted to
    // `Vec` for `table/glyf/read.rs`'s six scratch buffers. `Buffer` is
    // `Copy`/`repr(C)` and its zeroed state is exactly what `bufnew()`
    // itself produces (`__caryll_allocate_clean` zeroes, then `bufnew` only
    // re-asserts `free`/`size` are 0), so a `vec![zero_buffer; n]` starts
    // every slot in the same state a freshly-`bufnew`'d buffer would.
    let zero_buffer = Buffer { cursor: 0, size: 0, free: 0, data: ::core::ptr::null_mut() };
    let mut char_strings: Vec<Buffer> =
        vec![zero_buffer; (*g).total_char_strings.wrapping_add(1 as u32) as usize];
    let mut lsubrs: Vec<Buffer> = vec![zero_buffer; max_l_subrs.wrapping_add(1 as u32) as usize];
    let mut gsubrs: Vec<Buffer> = vec![zero_buffer; max_g_subrs.wrapping_add(1 as u32) as usize];
    let mut j: u32 = 0 as u32;
    let mut r: *mut CffSubrRule = (*g).root;
    let mut e: *mut CffSubrNode = (*(*r).guard).next;
    while e != (*r).guard {
        serialize_node_to_buffer(
            e,
            char_strings.as_mut_ptr().add(j as usize),
            gsubrs.as_mut_ptr(),
            max_g_subrs,
            lsubrs.as_mut_ptr(),
            max_l_subrs,
        );
        if (*e).rule.is_null() && !(*e).terminal.is_null() && (*e).hard as ::core::ffi::c_int != 0 {
            j = j.wrapping_add(1);
        }
        e = (*e).next;
    }
    let mut is: *mut CffIndex = CFF_I_INDEX.from_callback.expect("non-null function pointer")(
        char_strings.as_mut_ptr() as *mut ::core::ffi::c_void,
        (*g).total_char_strings,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
        ),
    );
    let mut igs: *mut CffIndex = CFF_I_INDEX.from_callback.expect("non-null function pointer")(
        gsubrs.as_mut_ptr() as *mut ::core::ffi::c_void,
        max_g_subrs,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
        ),
    );
    let mut ils: *mut CffIndex = CFF_I_INDEX.from_callback.expect("non-null function pointer")(
        lsubrs.as_mut_ptr() as *mut ::core::ffi::c_void,
        max_l_subrs,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
        ),
    );
    for entry in char_strings.iter_mut().take((*g).total_char_strings as usize) {
        free(entry.data as *mut ::core::ffi::c_void);
        entry.data = ::core::ptr::null_mut::<u8>();
    }
    for entry in gsubrs.iter_mut().take(max_g_subrs as usize) {
        free(entry.data as *mut ::core::ffi::c_void);
        entry.data = ::core::ptr::null_mut::<u8>();
    }
    for entry in lsubrs.iter_mut().take(max_l_subrs as usize) {
        free(entry.data as *mut ::core::ffi::c_void);
        entry.data = ::core::ptr::null_mut::<u8>();
    }
    *s = CFF_I_INDEX.build.expect("non-null function pointer")(is);
    *gs = CFF_I_INDEX.build.expect("non-null function pointer")(igs);
    *ls = CFF_I_INDEX.build.expect("non-null function pointer")(ils);
    CFF_I_INDEX.free.expect("non-null function pointer")(is);
    CFF_I_INDEX.free.expect("non-null function pointer")(igs);
    CFF_I_INDEX.free.expect("non-null function pointer")(ils);
}
