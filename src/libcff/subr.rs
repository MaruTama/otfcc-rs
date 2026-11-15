use otfcc_binary::Buffer;

/// Index into `CffSubrGraph.nodes`; `RuleId` indexes `rules`. The graph is an
/// arena of doubly linked nodes.
///
/// A deleted node's slot is never reused (see `CffSubrGraph::delete_node`):
/// reusing it would let a stale `NodeId` silently resolve to an unrelated
/// node. Dead slots stay allocated until the graph is dropped, after one
/// CFF table's subroutinization.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct NodeId(usize);
/// Rules are never individually removed mid-algorithm (only accumulated,
/// via `CffSubrGraph::alloc_rule`, and torn down all at once in
/// `CffSubrGraph::dispose`), so plain indices need no tombstone scheme.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct RuleId(usize);

#[derive(Clone, Debug)]
struct CffSubrNode {
    prev: Option<NodeId>,
    rule: Option<RuleId>,
    next: Option<NodeId>,
    terminal: Option<Buffer>,
    hard: bool,
    guard: bool,
    last: bool,
    /// Set once by `CffSubrGraph::delete_node`; see `NodeId`'s own
    /// comment for why a dead slot is never reused. `node`/`node_mut`
    /// assert against touching a tombstoned slot again, so a bug in this
    /// graph panics loudly instead of silently reading a dead node's
    /// already-cleared fields.
    dead: bool,
}
#[derive(Copy, Clone, Debug)]
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
/// A diagram's arity and the node it starts at.
#[derive(Copy, Clone, Debug)]
struct CffSubrDiagramIndexEntry {
    arity: u8,
    start: NodeId,
}
/// `diagram_index` holds both "singlet" (arity 1) and "doublet" (arity 2)
/// entries in one table, keyed by a variable-length byte fingerprint
/// (`get_singlet_hash_key`/`get_doublet_hash_key`) whose first byte
/// ('1' vs '2') keeps the two arities from ever colliding. Its order never
/// matters, so it is a `HashMap`.
#[derive(Debug)]
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
impl CffSubrGraph {
    /// An empty graph holding only the root rule.
    pub fn new(do_subroutinize: bool) -> Self {
        // `root`/`last` are placeholders until `alloc_rule` below replaces them.
        let mut g = CffSubrGraph {
            nodes: Vec::new(),
            rules: Vec::new(),
            root: RuleId(0),
            last: RuleId(0),
            diagram_index: std::collections::HashMap::new(),
            total_rules: 0,
            total_char_strings: 0,
            do_subroutinize,
        };
        let root = g.alloc_rule();
        g.root = root;
        g.last = root;
        g
    }
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
            terminal: None,
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
    fn clean_node(&mut self, x: NodeId) {
        if let Some(r) = self.node(x).rule {
            self.rule_mut(r).refcount = self.rule(r).refcount.wrapping_sub(1);
        }
        let n = self.node_mut(x);
        n.rule = None;
        n.terminal = None;
    }
    fn delete_node(&mut self, x: NodeId) {
        self.clean_node(x);
        // Tombstone, not a reused slot -- see `NodeId`'s own comment.
        self.nodes[x.0].dead = true;
    }
}

mod graph;
mod serialize;

pub use graph::insert_il_to_graph;
pub use serialize::il_graph_to_buffers;

/// Replicates `strncmp(a, b, n) == 0` for two byte slices of length at
/// least `n`: compares at most `n` bytes, stopping (and reporting equal)
/// as soon as a NUL byte is reached at the same position in both, exactly
/// matching `strncmp`'s own early termination on an embedded or trailing
/// NUL.
fn strncmp_eq(a: &[u8], b: &[u8], n: usize) -> bool {
    for i in 0..n {
        let (ac, bc) = (a[i], b[i]);
        if ac != bc {
            return false;
        }
        if ac == 0 {
            return true;
        }
    }
    true
}
#[cfg(test)]
mod strncmp_eq_tests;
// calls (`insert_il_to_graph`, `il_graph_to_buffers`), independent
// of how the graph is represented internally. Otherwise `-O2`
// subroutinization is covered only end-to-end, by
// `KRName-Regular-O2.otf`'s golden checksum.
#[cfg(test)]
mod subr_graph_tests;

