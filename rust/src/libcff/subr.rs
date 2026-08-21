#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, strncmp};

use crate::logger::{LoggerType, LOG_VL_PROGRESS, logger_log_sds};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};

use crate::libcff::CffCharstringOperator;
use crate::libcff::{OP_CALLGSUBR, OP_CALLSUBR, OP_ENDCHAR, OP_RETURN, TYPE2_MAX_SUBRS, TYPE2_SUBR_NESTING};
use crate::libcff::cff_index::CffIndex;
use crate::libcff::charstring_il::{CffCharstringIl};
use crate::libcff::cff_index::{new_index_by_callback, build_index, cff_index_free};
use crate::libcff::cff_writer::{cff_merge_cs2_int, cff_merge_cs2_operand, cff_merge_cs2_operator, cff_merge_cs2_special};
use crate::support::buffer::{buffree, buflen, bufnew, bufwrite_buf};

/// Index into `CffSubrGraph.nodes`. This (and `RuleId`) replaces the
/// intrusive `*mut CffSubrNode`/`*mut CffSubrRule` doubly-linked-list
/// pointers this file used to be built from -- the plan's own writeup
/// flags this as the hardest remaining piece in the whole migration, and
/// prescribes exactly this shape ("アリーナ(Vec) + インデックス").
///
/// A "deleted" node's slot is never reused -- see
/// `CffSubrGraph::delete_node` -- it's left as a permanent tombstone
/// instead. That is the one property that makes this conversion actually
/// safe rather than just differently unsafe: a naive arena that *does*
/// recycle freed slots would let a stale `NodeId` silently start
/// resolving to a totally unrelated later node the moment that slot gets
/// reused, which is worse than the raw-pointer use-after-free it would
/// replace (a dangling pointer at least tends to crash; a reused index
/// just corrupts the graph quietly). The cost is that dead node slots
/// stay allocated for the rest of the graph's lifetime -- exactly one
/// CFF table's subroutinize build pass, not something long-lived.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct NodeId(usize);
/// Rules are never individually removed mid-algorithm (only accumulated,
/// via `CffSubrGraph::alloc_rule`, and torn down all at once in
/// `CffSubrGraph::dispose`), so plain indices need no tombstone scheme
/// here at all -- `Vec::push`'s possible reallocation never invalidates
/// an already-issued index the way it would a raw pointer into the same
/// backing storage.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct RuleId(usize);

#[derive(Copy, Clone)]
struct CffSubrNode {
    prev: Option<NodeId>,
    rule: Option<RuleId>,
    next: Option<NodeId>,
    // Still a raw `*mut Buffer`, not `Vec<u8>` -- `Buffer` itself (and its
    // `bufnew`/`buffree` malloc-shaped lifecycle) is a separate, larger,
    // already-planned conversion (Stage 7-2-e) that touches every file in
    // this crate, not just this one's linked list.
    terminal: *mut Buffer,
    hard: bool,
    guard: bool,
    last: bool,
    /// Set once by `CffSubrGraph::delete_node`; see `NodeId`'s own
    /// comment for why a dead slot is never reused. `node`/`node_mut`
    /// assert against touching a tombstoned slot again, the same
    /// "provably safe, but assert the invariant instead of trusting it
    /// silently" posture this migration has used since `otfcc-vec-
    /// field-assign-needs-calloc` -- a live bug in this graph would now
    /// panic loudly here instead of silently reading a dead node's
    /// already-cleared fields.
    dead: bool,
}
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
    guard: NodeId,
    next: Option<RuleId>,
}
/// Replaces the uthash-based `CffSubrDiagramIndex` (which also carried
/// `key: *mut u8` and `hh: UtHashHandle`, both now subsumed by
/// `HashMap`'s own key/bucket machinery). `key` was write-only outside
/// insertion/disposal (grepped: never read for anything but the hash
/// itself), so it needs no home in the value at all -- the two fields
/// that were actually read back (`arity`, `start`) are all that remain.
#[derive(Copy, Clone)]
struct CffSubrDiagramIndexEntry {
    arity: u8,
    start: NodeId,
}
/// `diagram_index` holds both "singlet" (arity 1) and "doublet" (arity 2)
/// entries in one table, keyed by a variable-length byte fingerprint
/// (`get_singlet_hash_key`/`get_doublet_hash_key`) whose first byte
/// ('1' vs '2') keeps the two arities from ever colliding -- order never
/// matters (no `HASH_SORT`, and the only whole-table walk is disposal),
/// so `HashMap` applies directly.
pub struct CffSubrGraph {
    nodes: Vec<CffSubrNode>,
    rules: Vec<CffSubrRule>,
    root: RuleId,
    last: RuleId,
    diagram_index: std::collections::HashMap<Vec<u8>, CffSubrDiagramIndexEntry>,
    total_rules: u32,
    total_char_strings: u32,
    pub do_subroutinize: bool,
}
/// `root`/`last` here are placeholders, valid but not yet meaningful --
/// every real construction site calls `cff_subr_graph_init` (which
/// allocates the actual root rule into `nodes`/`rules` and overwrites
/// both) immediately afterward. This exists so callers outside this file
/// (`table/cff.rs`) can build a `CffSubrGraph` without needing to name
/// `NodeId`/`RuleId`, which are private to this module.
impl Default for CffSubrGraph {
    fn default() -> Self {
        CffSubrGraph {
            nodes: Vec::new(),
            rules: Vec::new(),
            root: RuleId(0),
            last: RuleId(0),
            diagram_index: std::collections::HashMap::new(),
            total_rules: 0,
            total_char_strings: 0,
            do_subroutinize: false,
        }
    }
}
impl CffSubrGraph {
    fn node(&self, id: NodeId) -> &CffSubrNode {
        let n = &self.nodes[id.0];
        debug_assert!(!n.dead, "use of a deleted CffSubrNode");
        n
    }
    fn node_mut(&mut self, id: NodeId) -> &mut CffSubrNode {
        let n = &mut self.nodes[id.0];
        debug_assert!(!n.dead, "use of a deleted CffSubrNode");
        n
    }
    fn rule(&self, id: RuleId) -> &CffSubrRule {
        &self.rules[id.0]
    }
    fn rule_mut(&mut self, id: RuleId) -> &mut CffSubrRule {
        &mut self.rules[id.0]
    }
    fn alloc_node(&mut self) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(CffSubrNode {
            prev: None,
            rule: None,
            next: None,
            terminal: ::core::ptr::null_mut(),
            hard: false,
            guard: false,
            last: false,
            dead: false,
        });
        id
    }
    fn alloc_rule(&mut self) -> RuleId {
        let rid = RuleId(self.rules.len());
        let guard = self.alloc_node();
        self.rules.push(CffSubrRule {
            printed: false,
            numbered: false,
            number: 0,
            height: 0,
            unique_index: 0,
            cff_index: 0,
            refcount: 0,
            effective_length: 0,
            guard,
            next: None,
        });
        let n = self.node_mut(guard);
        n.prev = Some(guard);
        n.next = Some(guard);
        n.guard = true;
        n.rule = Some(rid);
        rid
    }
    unsafe fn clean_node(&mut self, x: NodeId) {
        if let Some(r) = self.node(x).rule {
            self.rule_mut(r).refcount = self.rule(r).refcount.wrapping_sub(1);
        }
        let terminal = self.node(x).terminal;
        buffree(terminal);
        let n = self.node_mut(x);
        n.rule = None;
        n.terminal = ::core::ptr::null_mut();
    }
    unsafe fn delete_node(&mut self, x: NodeId) {
        self.clean_node(x);
        // Tombstone, not a reused slot -- see `NodeId`'s own comment.
        self.nodes[x.0].dead = true;
    }
    unsafe fn delete_full_rule(&mut self, r: RuleId) {
        let guard = self.rule(r).guard;
        let mut e = self.node(guard).next.unwrap();
        while e != guard {
            let next = self.node(e).next.unwrap();
            let terminal = self.node(e).terminal;
            if !terminal.is_null() {
                buffree(terminal);
            }
            e = next;
        }
        // No explicit free for the node/rule slots themselves -- they
        // live in `self.nodes`/`self.rules`, dropped along with the
        // graph. Only `terminal` (a still-malloc-shaped `Buffer`, per
        // `CffSubrNode`'s own comment) needs an explicit release.
    }
    unsafe fn init(&mut self) {
        let root = self.alloc_rule();
        self.root = root;
        self.last = root;
        self.diagram_index = std::collections::HashMap::new();
        self.total_rules = 0;
        self.total_char_strings = 0;
        self.do_subroutinize = false;
    }
    unsafe fn dispose(&mut self) {
        let mut r = Some(self.root);
        while let Some(rid) = r {
            let next = self.rule(rid).next;
            self.delete_full_rule(rid);
            r = next;
        }
        // The map's values own nothing (both fields are `Copy`), so
        // dropping it is the whole disposal -- no manual `HASH_ITER`+
        // `HASH_DEL`+`free` walk needed.
        self.diagram_index = std::collections::HashMap::new();
        // `self.nodes`/`self.rules` are deliberately left as-is (not
        // cleared) -- nothing reads them again after `dispose` (matches
        // every call site: the graph itself goes out of scope right
        // after), and leaving them populated means even a *mistaken*
        // future touch would resolve to a real, still-in-bounds slot
        // instead of a dangling pointer the way the original's `free`d
        // `CffSubrRule`/`CffSubrNode` structs would have.
    }
}
#[inline]
pub unsafe fn cff_subr_graph_init(x: *mut CffSubrGraph) {
    (*x).init();
}
#[inline]
pub unsafe fn cff_subr_graph_dispose(x: *mut CffSubrGraph) {
    (*x).dispose();
}
/// The byte layout (header bytes, payload, trailing NUL) matches the
/// original `malloc`+`memcpy` construction exactly, so keys built here
/// compare equal to keys built there -- the leading `'1'`/`'2'` (singlet
/// vs doublet) keeps the two arities from ever colliding in one table.
unsafe fn get_singlet_hash_key(g: &CffSubrGraph, n: NodeId) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();
    key.push(b'1');
    let node = g.node(n);
    key.push(if node.rule.is_some() { b'1' } else { b'0' });
    key.push(b'0');
    if let Some(r) = node.rule {
        key.extend_from_slice(&g.rule(r).unique_index.to_ne_bytes());
    } else {
        key.extend_from_slice(std::slice::from_raw_parts(
            (*node.terminal).data,
            buflen(node.terminal),
        ));
    }
    key.push(0);
    key
}
unsafe fn get_doublet_hash_key(g: &CffSubrGraph, n: NodeId) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();
    key.push(b'2');
    let node = g.node(n);
    let next = g.node(node.next.unwrap());
    key.push(if node.rule.is_some() { b'1' } else { b'0' });
    key.push(if next.rule.is_some() { b'1' } else { b'0' });
    if let Some(r) = node.rule {
        key.extend_from_slice(&g.rule(r).unique_index.to_ne_bytes());
    } else {
        key.extend_from_slice(std::slice::from_raw_parts(
            (*node.terminal).data,
            buflen(node.terminal),
        ));
    }
    if let Some(r) = next.rule {
        key.extend_from_slice(&g.rule(r).unique_index.to_ne_bytes());
    } else {
        key.extend_from_slice(std::slice::from_raw_parts(
            (*next.terminal).data,
            buflen(next.terminal),
        ));
    }
    key.push(0);
    key
}
unsafe fn last_node_of(g: &CffSubrGraph, r: RuleId) -> NodeId {
    g.node(g.rule(r).guard).prev.unwrap()
}
unsafe fn copy_node(g: &mut CffSubrGraph, n: NodeId) -> NodeId {
    let m = g.alloc_node();
    let (rule, last) = {
        let nn = g.node(n);
        (nn.rule, nn.last)
    };
    if let Some(r) = rule {
        g.node_mut(m).rule = Some(r);
        g.rule_mut(r).refcount = g.rule(r).refcount.wrapping_add(1);
    } else {
        let terminal_src = g.node(n).terminal;
        let terminal = bufnew();
        bufwrite_buf(terminal, terminal_src);
        g.node_mut(m).terminal = terminal;
    }
    g.node_mut(m).last = last;
    m
}
unsafe fn unlink_node(g: &mut CffSubrGraph, a: NodeId) {
    let an = g.node(a);
    if an.hard || an.guard {
        return;
    }
    // Only drop an index entry if it is still pointing at the node being
    // unlinked -- some *other* node may have since claimed this key's
    // slot, and that entry must be left alone.
    let doublet_key = get_doublet_hash_key(g, a);
    if g.diagram_index.get(&doublet_key).is_some_and(|di| di.start == a) {
        g.diagram_index.remove(&doublet_key);
    }
    let singlet_key = get_singlet_hash_key(g, a);
    if g.diagram_index.get(&singlet_key).is_some_and(|di| di.start == a) {
        g.diagram_index.remove(&singlet_key);
    }
}
unsafe fn add_doublet(g: &mut CffSubrGraph, n: Option<NodeId>) {
    let Some(n) = n else { return };
    let nn = *g.node(n);
    if nn.guard || nn.hard {
        return;
    }
    let Some(next) = nn.next else { return };
    let nextn = g.node(next);
    if nextn.hard || nextn.guard {
        return;
    }
    let key = get_doublet_hash_key(g, n);
    g.diagram_index.insert(key, CffSubrDiagramIndexEntry { arity: 2, start: n });
}
unsafe fn add_singlet(g: &mut CffSubrGraph, n: Option<NodeId>) {
    let Some(n) = n else { return };
    let nn = g.node(n);
    if nn.guard || nn.hard {
        return;
    }
    let key = get_singlet_hash_key(g, n);
    g.diagram_index.insert(key, CffSubrDiagramIndexEntry { arity: 1, start: n });
}
unsafe fn ident_node(g: &CffSubrGraph, m: NodeId, n: NodeId) -> bool {
    let mn = g.node(m);
    let nn = g.node(n);
    if let Some(mr) = mn.rule {
        return Some(mr) == nn.rule;
    } else if nn.rule.is_some() {
        return false;
    } else {
        return (*mn.terminal).size == (*nn.terminal).size
            && strncmp(
                (*mn.terminal).data as *mut ::core::ffi::c_char,
                (*nn.terminal).data as *mut ::core::ffi::c_char,
                (*mn.terminal).size,
            ) == 0 as ::core::ffi::c_int;
    };
}
unsafe fn join_nodes(g: &mut CffSubrGraph, m: NodeId, n: NodeId) {
    if g.node(m).next.is_some() {
        unlink_node(g, m);
        let n_prev = g.node(n).prev;
        let n_next = g.node(n).next;
        if let (Some(np), Some(nx)) = (n_prev, n_next) {
            if ident_node(g, np, n) && ident_node(g, n, nx) {
                add_doublet(g, Some(n));
            }
        }
        let m_prev = g.node(m).prev;
        let m_next = g.node(m).next;
        if let (Some(mp), Some(mx)) = (m_prev, m_next) {
            if ident_node(g, mp, m) && ident_node(g, m, mx) {
                add_doublet(g, Some(mp));
            }
        }
    }
    g.node_mut(m).next = Some(n);
    g.node_mut(n).prev = Some(m);
}
unsafe fn x_insert_node_after(g: &mut CffSubrGraph, m: NodeId, n: NodeId) {
    let m_next = g.node(m).next.unwrap();
    join_nodes(g, n, m_next);
    join_nodes(g, m, n);
}
unsafe fn remove_node_from_graph(g: &mut CffSubrGraph, a: NodeId) {
    let (prev, next) = {
        let an = g.node(a);
        (an.prev.unwrap(), an.next.unwrap())
    };
    join_nodes(g, prev, next);
    if !g.node(a).guard {
        unlink_node(g, a);
        g.delete_node(a);
    }
}
unsafe fn expand_call(g: &mut CffSubrGraph, a: NodeId) {
    let aprev = g.node(a).prev.unwrap();
    let anext = g.node(a).next.unwrap();
    let r = g.node(a).rule.unwrap();
    let guard = g.rule(r).guard;
    let r1 = g.node(guard).next.unwrap();
    let r2 = g.node(guard).prev.unwrap();
    unlink_node(g, a);
    join_nodes(g, aprev, r1);
    join_nodes(g, r2, anext);
    add_doublet(g, Some(r2));
    g.node_mut(guard).next = Some(guard);
    g.node_mut(guard).prev = Some(guard);
    g.rule_mut(r).refcount = g.rule(r).refcount.wrapping_sub(1);
    g.delete_node(a);
}
unsafe fn substitute_doublet_with_rule(g: &mut CffSubrGraph, m: NodeId, r: RuleId) {
    let prev = g.node(m).prev.unwrap();
    let first = g.node(prev).next.unwrap();
    remove_node_from_graph(g, first);
    let second = g.node(prev).next.unwrap();
    remove_node_from_graph(g, second);
    let invoke = g.alloc_node();
    g.node_mut(invoke).rule = Some(r);
    g.rule_mut(r).refcount = g.rule(r).refcount.wrapping_add(1);
    x_insert_node_after(g, prev, invoke);
    add_doublet(g, Some(prev));
    add_doublet(g, Some(invoke));
    add_singlet(g, Some(invoke));
    if !check_doublet_match(g, prev) {
        let prev_next = g.node(prev).next.unwrap();
        check_doublet_match(g, prev_next);
    }
}
unsafe fn substitute_singlet_with_rule(g: &mut CffSubrGraph, m: NodeId, r: RuleId) {
    let prev = g.node(m).prev.unwrap();
    let first = g.node(prev).next.unwrap();
    remove_node_from_graph(g, first);
    let invoke = g.alloc_node();
    g.node_mut(invoke).rule = Some(r);
    g.rule_mut(r).refcount = g.rule(r).refcount.wrapping_add(1);
    x_insert_node_after(g, prev, invoke);
    add_doublet(g, Some(prev));
    add_doublet(g, Some(invoke));
    add_singlet(g, Some(invoke));
}
unsafe fn process_match_doublet(g: &mut CffSubrGraph, m: NodeId, n: NodeId) {
    let m_prev = g.node(m).prev.unwrap();
    let m_next = g.node(m).next.unwrap();
    let m_next_next = g.node(m_next).next.unwrap();
    let rule: RuleId;
    if g.node(m_prev).guard && g.node(m_next_next).guard {
        rule = g.node(m_prev).rule.unwrap();
        substitute_doublet_with_rule(g, n, rule);
    } else {
        rule = g.alloc_rule();
        g.rule_mut(rule).unique_index = g.total_rules;
        g.total_rules = g.total_rules.wrapping_add(1);
        g.rule_mut(g.last).next = Some(rule);
        g.last = rule;
        let last_of_rule = last_node_of(g, rule);
        let cm = copy_node(g, m);
        x_insert_node_after(g, last_of_rule, cm);
        let last_of_rule = last_node_of(g, rule);
        let m_next = g.node(m).next.unwrap();
        let cmn = copy_node(g, m_next);
        x_insert_node_after(g, last_of_rule, cmn);
        substitute_doublet_with_rule(g, m, rule);
        substitute_doublet_with_rule(g, n, rule);
        let guard = g.rule(rule).guard;
        let first = g.node(guard).next.unwrap();
        add_doublet(g, Some(first));
        add_singlet(g, Some(first));
        let second = g.node(first).next.unwrap();
        add_singlet(g, Some(second));
    }
    let guard = g.rule(rule).guard;
    let first = g.node(guard).next.unwrap();
    if let Some(fr) = g.node(first).rule {
        if g.rule(fr).refcount == 1 {
            expand_call(g, first);
        }
    }
}
unsafe fn process_match_singlet(g: &mut CffSubrGraph, m: NodeId, n: NodeId) {
    let m_prev = g.node(m).prev.unwrap();
    let m_next = g.node(m).next.unwrap();
    let rule: RuleId;
    if g.node(m_prev).guard && g.node(m_next).guard {
        rule = g.node(m_prev).rule.unwrap();
        substitute_singlet_with_rule(g, n, rule);
    } else {
        rule = g.alloc_rule();
        g.rule_mut(rule).unique_index = g.total_rules;
        g.total_rules = g.total_rules.wrapping_add(1);
        g.rule_mut(g.last).next = Some(rule);
        g.last = rule;
        let last_of_rule = last_node_of(g, rule);
        let cm = copy_node(g, m);
        x_insert_node_after(g, last_of_rule, cm);
        substitute_singlet_with_rule(g, m, rule);
        substitute_singlet_with_rule(g, n, rule);
        let guard = g.rule(rule).guard;
        let first = g.node(guard).next.unwrap();
        add_singlet(g, Some(first));
    };
}
unsafe fn check_doublet_match(g: &mut CffSubrGraph, n: NodeId) -> bool {
    // `n.next` is trusted unchecked here, matching the original exactly
    // (it dereferenced `(*n).next` with no null check at all) -- every
    // call site reaches this only with an already-linked node, so the
    // invariant always holds in practice.
    let next = g.node(n).next.unwrap();
    if g.node(n).guard || g.node(next).guard || g.node(n).hard || g.node(next).hard {
        return false;
    }
    let key = get_doublet_hash_key(g, n);
    match g.diagram_index.entry(key) {
        std::collections::hash_map::Entry::Vacant(v) => {
            v.insert(CffSubrDiagramIndexEntry { arity: 2, start: n });
            false
        }
        std::collections::hash_map::Entry::Occupied(o) => {
            let di = *o.get();
            let start_next = g.node(di.start).next.unwrap();
            if di.arity == 2 && di.start != n && !g.node(di.start).guard && !g.node(start_next).guard {
                process_match_doublet(g, di.start, n);
                true
            } else {
                true
            }
        }
    }
}
unsafe fn check_singlet_match(g: &mut CffSubrGraph, n: NodeId) -> bool {
    let nn = g.node(n);
    if nn.guard || nn.hard {
        return false;
    }
    let key = get_singlet_hash_key(g, n);
    match g.diagram_index.entry(key) {
        std::collections::hash_map::Entry::Vacant(v) => {
            v.insert(CffSubrDiagramIndexEntry { arity: 1, start: n });
            false
        }
        std::collections::hash_map::Entry::Occupied(o) => {
            let di = *o.get();
            if di.arity == 1 && di.start != n && !g.node(di.start).guard {
                process_match_singlet(g, di.start, n);
                true
            } else {
                false
            }
        }
    }
}
unsafe fn append_node_to_graph(g: &mut CffSubrGraph, n: NodeId) {
    let root = g.root;
    let last = last_node_of(g, root);
    x_insert_node_after(g, last, n);
    if g.do_subroutinize {
        if !check_doublet_match(g, last) {
            if buflen(g.node(n).terminal) > 15 as usize {
                check_singlet_match(g, n);
            }
        }
    }
}
pub unsafe fn cff_insert_il_to_graph(
    g: *mut CffSubrGraph,
    il: *mut CffCharstringIl,
) {
    let g = &mut *g;
    let mut blob: *mut Buffer = bufnew();
    let mut flush: bool = false;
    let mut last: bool = false;
    let mut j: u32 = 0 as u32;
    while j < (*il).instr.len() as u32 {
        match (*(*il).instr.as_mut_ptr().offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                if flush {
                    let n = g.alloc_node();
                    g.node_mut(n).rule = None;
                    g.node_mut(n).terminal = blob;
                    g.node_mut(n).last = last;
                    append_node_to_graph(g, n);
                    blob = bufnew();
                    flush = false;
                }
                cff_merge_cs2_operand(blob, (*(*il).instr.as_mut_ptr().offset(j as isize)).d());
            }
            1 => {
                cff_merge_cs2_operator(
                    blob,
                    CffCharstringOperator((*(*il).instr.as_mut_ptr().offset(j as isize)).i()),
                );
                if (*(*il).instr.as_mut_ptr().offset(j as isize)).i()
                    == OP_ENDCHAR.0
                {
                    last = true;
                }
                flush = true;
            }
            2 => {
                cff_merge_cs2_special(
                    blob,
                    (*(*il).instr.as_mut_ptr().offset(j as isize)).i() as u8,
                );
                flush = true;
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    if (*blob).size != 0 {
        let n_0 = g.alloc_node();
        g.node_mut(n_0).rule = None;
        g.node_mut(n_0).last = last;
        g.node_mut(n_0).terminal = blob;
        append_node_to_graph(g, n_0);
    }
    // A leftover empty `blob` here (only reachable for an IL with zero
    // instructions -- e.g. a genuinely blank glyph) is never freed before
    // being reassigned below, in the original too: a real but pre-
    // existing, low-severity leak of one small `Buffer`, preserved as-is
    // rather than silently fixed as a side effect of this conversion --
    // an intentional decision, not an oversight.
    blob = bufnew();
    let n_1 = g.alloc_node();
    g.node_mut(n_1).rule = None;
    g.node_mut(n_1).terminal = blob;
    g.node_mut(n_1).hard = true;
    append_node_to_graph(g, n_1);
    g.total_char_strings = g.total_char_strings.wrapping_add(1 as u32);
}
unsafe fn cff_stat_height(g: &mut CffSubrGraph, r: RuleId, height: u32) {
    if height > g.rule(r).height {
        g.rule_mut(r).height = height;
    }
    let mut effective_length: u32 = 0 as u32;
    let guard = g.rule(r).guard;
    let mut e = g.node(guard).next.unwrap();
    while e != guard {
        let (rule, terminal) = {
            let en = g.node(e);
            (en.rule, en.terminal)
        };
        if let Some(er) = rule {
            cff_stat_height(g, er, height.wrapping_add(1 as u32));
            effective_length = effective_length.wrapping_add(4 as u32);
        } else {
            effective_length = (effective_length as usize).wrapping_add((*terminal).size) as u32;
        }
        e = g.node(e).next.unwrap();
    }
    g.rule_mut(r).effective_length = effective_length;
}
unsafe fn number_a_subroutine(g: &mut CffSubrGraph, r: RuleId, current: &mut u32) {
    if g.rule(r).numbered {
        return;
    }
    if g.rule(r).height >= TYPE2_SUBR_NESTING {
        return;
    }
    if g.rule(r)
        .effective_length
        .wrapping_sub(4 as u32)
        .wrapping_mul(g.rule(r).refcount.wrapping_sub(1 as u32))
        .wrapping_sub(4 as u32)
        <= 0 as u32
    {
        return;
    }
    g.rule_mut(r).number = *current;
    *current = (*current).wrapping_add(1);
    g.rule_mut(r).numbered = true;
    let guard = g.rule(r).guard;
    let mut e = g.node(guard).next.unwrap();
    while e != guard {
        if let Some(er) = g.node(e).rule {
            number_a_subroutine(g, er, current);
        }
        e = g.node(e).next.unwrap();
    }
}
unsafe fn cff_number_subroutines(g: &mut CffSubrGraph) -> u32 {
    let mut current: u32 = 0 as u32;
    let root = g.root;
    let guard = g.rule(root).guard;
    let mut e = g.node(guard).next.unwrap();
    while e != guard {
        if let Some(er) = g.node(e).rule {
            number_a_subroutine(g, er, &mut current);
        }
        e = g.node(e).next.unwrap();
    }
    return current;
}
#[inline]
unsafe fn subroutine_bias(mut cnt: i32) -> i32 {
    if cnt < 1240 as i32 {
        return 107 as i32;
    } else if cnt < 33900 as i32 {
        return 1131 as i32;
    } else {
        return 32768 as i32;
    };
}
unsafe fn ends_with_end_char(g: &CffSubrGraph, rule: RuleId) -> bool {
    let node = last_node_of(g, rule);
    let n = g.node(node);
    if !n.terminal.is_null() {
        return n.last;
    } else {
        return ends_with_end_char(g, n.rule.unwrap());
    };
}
unsafe fn serialize_node_to_buffer(
    g: &mut CffSubrGraph,
    node: NodeId,
    buf: *mut Buffer,
    gsubrs: *mut Buffer,
    max_g_subrs: u32,
    lsubrs: *mut Buffer,
    max_l_subrs: u32,
) {
    let (rule, terminal) = {
        let n = g.node(node);
        (n.rule, n.terminal)
    };
    if let Some(r) = rule {
        let (numbered, number, r_height) = {
            let ru = g.rule(r);
            (ru.numbered, ru.number, ru.height)
        };
        if numbered && number < max_l_subrs.wrapping_add(max_g_subrs) && r_height < TYPE2_SUBR_NESTING {
            let target: *mut Buffer;
            if number < max_l_subrs {
                let stacknum: i32 =
                    number.wrapping_sub(subroutine_bias(max_l_subrs as i32) as u32) as i32;
                target = lsubrs.offset(number as isize);
                cff_merge_cs2_int(buf, stacknum);
                cff_merge_cs2_operator(buf, OP_CALLSUBR);
            } else {
                let stacknum_0: i32 = number
                    .wrapping_sub(max_l_subrs)
                    .wrapping_sub(subroutine_bias(max_g_subrs as i32) as u32)
                    as i32;
                target = gsubrs.offset(number.wrapping_sub(max_l_subrs) as isize);
                cff_merge_cs2_int(buf, stacknum_0);
                cff_merge_cs2_operator(buf, OP_CALLGSUBR);
            }
            if !g.rule(r).printed {
                g.rule_mut(r).printed = true;
                let guard = g.rule(r).guard;
                let mut e = g.node(guard).next.unwrap();
                while e != guard {
                    let next = g.node(e).next.unwrap();
                    serialize_node_to_buffer(g, e, target, gsubrs, max_g_subrs, lsubrs, max_l_subrs);
                    e = next;
                }
                if !ends_with_end_char(g, r) {
                    cff_merge_cs2_operator(target, OP_RETURN);
                }
            }
        } else {
            let guard = g.rule(r).guard;
            let mut e_0 = g.node(guard).next.unwrap();
            while e_0 != guard {
                let next = g.node(e_0).next.unwrap();
                serialize_node_to_buffer(g, e_0, buf, gsubrs, max_g_subrs, lsubrs, max_l_subrs);
                e_0 = next;
            }
        }
    } else {
        bufwrite_buf(buf, terminal);
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
pub unsafe fn cff_il_graph_to_buffers(
    g: *mut CffSubrGraph,
    mut s: *mut *mut Buffer,
    mut gs: *mut *mut Buffer,
    mut ls: *mut *mut Buffer,
    mut options: &Options,
) {
    let g = &mut *g;
    let root = g.root;
    cff_stat_height(g, root, 0 as u32);
    let mut max_subroutines: u32 = cff_number_subroutines(g);
    logger_log_sds(
        options.logger,
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
        vec![zero_buffer; g.total_char_strings.wrapping_add(1 as u32) as usize];
    let mut lsubrs: Vec<Buffer> = vec![zero_buffer; max_l_subrs.wrapping_add(1 as u32) as usize];
    let mut gsubrs: Vec<Buffer> = vec![zero_buffer; max_g_subrs.wrapping_add(1 as u32) as usize];
    let mut j: u32 = 0 as u32;
    let root = g.root;
    let guard = g.rule(root).guard;
    let mut e = g.node(guard).next.unwrap();
    while e != guard {
        let next = g.node(e).next.unwrap();
        let (e_rule, e_terminal, e_hard) = {
            let en = g.node(e);
            (en.rule, en.terminal, en.hard)
        };
        serialize_node_to_buffer(
            g,
            e,
            char_strings.as_mut_ptr().add(j as usize),
            gsubrs.as_mut_ptr(),
            max_g_subrs,
            lsubrs.as_mut_ptr(),
            max_l_subrs,
        );
        if e_rule.is_none() && !e_terminal.is_null() && e_hard {
            j = j.wrapping_add(1);
        }
        e = next;
    }
    let mut is: *mut CffIndex = new_index_by_callback(
        char_strings.as_mut_ptr() as *mut ::core::ffi::c_void,
        g.total_char_strings,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
        ),
    );
    let mut igs: *mut CffIndex = new_index_by_callback(
        gsubrs.as_mut_ptr() as *mut ::core::ffi::c_void,
        max_g_subrs,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
        ),
    );
    let mut ils: *mut CffIndex = new_index_by_callback(
        lsubrs.as_mut_ptr() as *mut ::core::ffi::c_void,
        max_l_subrs,
        Some(
            from_array
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
        ),
    );
    for entry in char_strings.iter_mut().take(g.total_char_strings as usize) {
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
    *s = build_index(is);
    *gs = build_index(igs);
    *ls = build_index(ils);
    cff_index_free(is);
    cff_index_free(igs);
    cff_index_free(ils);
}

// Safety-net coverage for the intrusive doubly-linked-list subroutinizer
// above, added *before* any structural change to it (per the plan's own
// note that this file -- the CFF subroutinize build path -- needs tests
// in place first: `-O2` subroutinize is otherwise covered only by
// `KRName-Regular-O2.otf`'s golden checksum, one payload, exercised only
// end-to-end). These pin the current (still raw-pointer/intrusive-list)
// behavior at the two public entry points `table/cff.rs` actually calls
// (`cff_insert_il_to_graph`, `cff_il_graph_to_buffers`), independent of
// how the graph ends up represented internally.
#[cfg(test)]
mod subr_graph_tests {
    use super::*;
    use crate::libcff::charstring_il::{CffCharstringIl, il_push_op, il_push_operand};
    use crate::libcff::cff_index::{cff_index_create, extract_index};
    use crate::libcff::{OP_HLINETO, OP_RMOVETO};

    fn zeroed_options() -> Options {
        unsafe { ::core::mem::zeroed() }
    }

    unsafe fn simple_glyph_il(x: f64, y: f64) -> CffCharstringIl {
        let mut il = CffCharstringIl { instr: Vec::new() };
        let il_ptr = &raw mut il;
        il_push_operand(il_ptr, x);
        il_push_op(il_ptr, OP_RMOVETO);
        il_push_operand(il_ptr, y);
        il_push_op(il_ptr, OP_HLINETO);
        il_push_op(il_ptr, OP_ENDCHAR);
        il
    }

    unsafe fn index_count(buf: *mut Buffer) -> u32 {
        let idx = cff_index_create();
        extract_index((*buf).data, (*buf).size as u32, 0, idx);
        let count = (*idx).count;
        cff_index_free(idx);
        count
    }

    unsafe fn build(
        glyphs: &[CffCharstringIl],
        do_subroutinize: bool,
    ) -> (*mut Buffer, *mut Buffer, *mut Buffer) {
        // `nodes`/`rules` start empty; `cff_subr_graph_init` allocates the
        // root rule (and its guard node) into them immediately below --
        // matches `table/cff.rs`'s own construction of this same struct.
        let mut g = CffSubrGraph::default();
        cff_subr_graph_init(&raw mut g);
        g.do_subroutinize = do_subroutinize;
        for il in glyphs {
            let mut il = il.clone();
            cff_insert_il_to_graph(&raw mut g, &raw mut il);
        }
        // `cff_il_graph_to_buffers` always logs a progress message
        // unconditionally, so `options.logger` must be a real one, not a
        // zeroed (null) one -- same requirement as `chaining/read.rs`'s
        // own unsupported-format-log test.
        let mut options = zeroed_options();
        options.logger =
            crate::logger::otfcc_new_logger(crate::logger::otfcc_new_empty_target());
        let mut s: *mut Buffer = ::core::ptr::null_mut();
        let mut gs: *mut Buffer = ::core::ptr::null_mut();
        let mut ls: *mut Buffer = ::core::ptr::null_mut();
        cff_il_graph_to_buffers(
            &raw mut g,
            &raw mut s,
            &raw mut gs,
            &raw mut ls,
            &options,
        );
        cff_subr_graph_dispose(&raw mut g);
        crate::logger::logger_dispose(options.logger);
        (s, gs, ls)
    }

    #[test]
    fn empty_graph_produces_an_empty_char_strings_index() {
        unsafe {
            let (s, gs, ls) = build(&[], false);
            assert_eq!(index_count(s), 0);
            assert_eq!(index_count(gs), 0);
            assert_eq!(index_count(ls), 0);
            buffree(s);
            buffree(gs);
            buffree(ls);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "calls libc::modf via cff_merge_cs2_operand, unsupported under Miri")]
    fn one_glyph_with_subroutinize_off_produces_one_char_string_and_no_subroutines() {
        unsafe {
            let il = simple_glyph_il(10.0, 20.0);
            let (s, gs, ls) = build(&[il], false);
            assert_eq!(index_count(s), 1);
            assert_eq!(index_count(gs), 0);
            assert_eq!(index_count(ls), 0);
            buffree(s);
            buffree(gs);
            buffree(ls);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "calls libc::modf via cff_merge_cs2_operand, unsupported under Miri")]
    fn two_identical_glyphs_with_subroutinize_on_extract_a_shared_subroutine() {
        unsafe {
            let il1 = simple_glyph_il(10.0, 20.0);
            let il2 = simple_glyph_il(10.0, 20.0);
            let (s, gs, ls) = build(&[il1, il2], true);
            assert_eq!(index_count(s), 2);
            // The identical [rmoveto, hlineto] pair repeated across both
            // glyphs is exactly the doublet `append_node_to_graph` checks
            // for on every append -- it should be extracted into one
            // shared subroutine (local or global depending on the
            // max_l_subrs/max_g_subrs split, so check both).
            assert!(index_count(gs) + index_count(ls) >= 1);
            buffree(s);
            buffree(gs);
            buffree(ls);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "calls libc::modf via cff_merge_cs2_operand, unsupported under Miri")]
    fn two_identical_glyphs_subroutinized_have_a_smaller_char_strings_index() {
        // The *total* size (char strings + subr indexes) isn't guaranteed
        // to shrink for an example this tiny -- the subr INDEX header and
        // the CALLSUBR/CALLGSUBR bytes are overhead that can outweigh the
        // savings for just two short glyphs; that's expected, not a bug.
        // What subroutinization does guarantee is that the *char strings*
        // themselves get smaller once the duplicated sequence is replaced
        // by a short subroutine call in each glyph.
        unsafe {
            let il1 = simple_glyph_il(10.0, 20.0);
            let il2 = simple_glyph_il(10.0, 20.0);
            let (s_on, gs_on, ls_on) = build(&[il1.clone(), il2.clone()], true);
            let char_strings_on = (*s_on).size;
            buffree(s_on);
            buffree(gs_on);
            buffree(ls_on);

            let (s_off, gs_off, ls_off) = build(&[il1, il2], false);
            let char_strings_off = (*s_off).size;
            buffree(s_off);
            buffree(gs_off);
            buffree(ls_off);

            assert!(char_strings_on < char_strings_off);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "calls libc::modf via cff_merge_cs2_operand, unsupported under Miri")]
    fn two_different_glyphs_with_subroutinize_on_extract_no_subroutine() {
        unsafe {
            let il1 = simple_glyph_il(10.0, 20.0);
            let il2 = simple_glyph_il(30.0, 40.0);
            let (s, gs, ls) = build(&[il1, il2], true);
            assert_eq!(index_count(s), 2);
            assert_eq!(index_count(gs), 0);
            assert_eq!(index_count(ls), 0);
            buffree(s);
            buffree(gs);
            buffree(ls);
        }
    }
}
