use super::*;
use otfcc_binary::Buffer;

use crate::libcff::CffCharstringOperator;
use crate::libcff::writer::{
    merge_cs2_operand, merge_cs2_operator, merge_cs2_special,
};
use crate::libcff::charstring_il::CffCharstringIl;
use crate::libcff::OP_ENDCHAR;

/// The key is header bytes, payload, then a trailing NUL; the leading
/// `'1'`/`'2'` (singlet vs doublet) keeps the two arities from ever
/// colliding in one table.
fn get_singlet_hash_key(g: &CffSubrGraph, n: NodeId) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();
    key.push(b'1');
    let node = g.node(n);
    key.push(if node.rule.is_some() { b'1' } else { b'0' });
    key.push(b'0');
    if let Some(r) = node.rule {
        key.extend_from_slice(&g.rule(r).unique_index.to_ne_bytes());
    } else {
        key.extend_from_slice(&node.terminal.as_ref().expect("a node without a rule holds bytes").data);
    }
    key.push(0);
    key
}
fn get_doublet_hash_key(g: &CffSubrGraph, n: NodeId) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();
    key.push(b'2');
    let node = g.node(n);
    let next = g.node(node.next.expect("a doublet starts at a node that has a successor"));
    key.push(if node.rule.is_some() { b'1' } else { b'0' });
    key.push(if next.rule.is_some() { b'1' } else { b'0' });
    if let Some(r) = node.rule {
        key.extend_from_slice(&g.rule(r).unique_index.to_ne_bytes());
    } else {
        key.extend_from_slice(&node.terminal.as_ref().expect("a node without a rule holds bytes").data);
    }
    if let Some(r) = next.rule {
        key.extend_from_slice(&g.rule(r).unique_index.to_ne_bytes());
    } else {
        key.extend_from_slice(&next.terminal.as_ref().expect("a node without a rule holds bytes").data);
    }
    key.push(0);
    key
}
pub(super) fn last_node_of(g: &CffSubrGraph, r: RuleId) -> NodeId {
    g.node(g.rule(r).guard).prev.expect("a rule's guard node is linked to its last node")
}
fn copy_node(g: &mut CffSubrGraph, n: NodeId) -> NodeId {
    let m = g.alloc_node();
    let (rule, last) = {
        let nn = g.node(n);
        (nn.rule, nn.last)
    };
    if let Some(r) = rule {
        g.node_mut(m).rule = Some(r);
        g.rule_mut(r).refcount = g.rule(r).refcount.wrapping_add(1);
    } else {
        let terminal = g.node(n).terminal.clone();
        g.node_mut(m).terminal = terminal;
    }
    g.node_mut(m).last = last;
    m
}
fn unlink_node(g: &mut CffSubrGraph, a: NodeId) {
    let an = g.node(a);
    if an.hard || an.guard {
        return;
    }
    // Only drop an index entry if it is still pointing at the node being
    // unlinked -- some *other* node may have since claimed this key's
    // slot, and that entry must be left alone.
    let doublet_key = get_doublet_hash_key(g, a);
    if g.diagram_index
        .get(&doublet_key)
        .is_some_and(|di| di.start == a)
    {
        g.diagram_index.remove(&doublet_key);
    }
    let singlet_key = get_singlet_hash_key(g, a);
    if g.diagram_index
        .get(&singlet_key)
        .is_some_and(|di| di.start == a)
    {
        g.diagram_index.remove(&singlet_key);
    }
}
fn add_doublet(g: &mut CffSubrGraph, n: Option<NodeId>) {
    let Some(n) = n else { return };
    let nn = g.node(n);
    if nn.guard || nn.hard {
        return;
    }
    let Some(next) = nn.next else { return };
    let nextn = g.node(next);
    if nextn.hard || nextn.guard {
        return;
    }
    let key = get_doublet_hash_key(g, n);
    g.diagram_index
        .insert(key, CffSubrDiagramIndexEntry { arity: 2, start: n });
}
fn add_singlet(g: &mut CffSubrGraph, n: Option<NodeId>) {
    let Some(n) = n else { return };
    let nn = g.node(n);
    if nn.guard || nn.hard {
        return;
    }
    let key = get_singlet_hash_key(g, n);
    g.diagram_index
        .insert(key, CffSubrDiagramIndexEntry { arity: 1, start: n });
}
fn ident_node(g: &CffSubrGraph, m: NodeId, n: NodeId) -> bool {
    let mn = g.node(m);
    let nn = g.node(n);
    if let Some(mr) = mn.rule {
        return Some(mr) == nn.rule;
    } else if nn.rule.is_some() {
        return false;
    } else {
        let m_terminal = mn.terminal.as_ref().expect("a node without a rule holds bytes");
        let n_terminal = nn.terminal.as_ref().expect("a node without a rule holds bytes");
        // Compares as C's `strncmp` would, stopping at a NUL in either side
        // even though this is charstring bytecode: two sequences equal up to
        // a NUL compare equal. Kept so output does not change.
        return m_terminal.data.len() == n_terminal.data.len()
            && strncmp_eq(&m_terminal.data, &n_terminal.data, m_terminal.data.len());
    };
}
fn join_nodes(g: &mut CffSubrGraph, m: NodeId, n: NodeId) {
    if g.node(m).next.is_some() {
        unlink_node(g, m);
        let n_prev = g.node(n).prev;
        let n_next = g.node(n).next;
        if let (Some(np), Some(nx)) = (n_prev, n_next)
            && ident_node(g, np, n) && ident_node(g, n, nx) {
                add_doublet(g, Some(n));
            }
        let m_prev = g.node(m).prev;
        let m_next = g.node(m).next;
        if let (Some(mp), Some(mx)) = (m_prev, m_next)
            && ident_node(g, mp, m) && ident_node(g, m, mx) {
                add_doublet(g, Some(mp));
            }
    }
    g.node_mut(m).next = Some(n);
    g.node_mut(n).prev = Some(m);
}
fn x_insert_node_after(g: &mut CffSubrGraph, m: NodeId, n: NodeId) {
    let m_next = g.node(m).next.unwrap();
    join_nodes(g, n, m_next);
    join_nodes(g, m, n);
}
fn remove_node_from_graph(g: &mut CffSubrGraph, a: NodeId) {
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
fn expand_call(g: &mut CffSubrGraph, a: NodeId) {
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
fn substitute_doublet_with_rule(g: &mut CffSubrGraph, m: NodeId, r: RuleId) {
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
fn substitute_singlet_with_rule(g: &mut CffSubrGraph, m: NodeId, r: RuleId) {
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
fn process_match_doublet(g: &mut CffSubrGraph, m: NodeId, n: NodeId) {
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
        g.total_rules += 1;
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
    if let Some(fr) = g.node(first).rule
        && g.rule(fr).refcount == 1 {
            expand_call(g, first);
        }
}
fn process_match_singlet(g: &mut CffSubrGraph, m: NodeId, n: NodeId) {
    let m_prev = g.node(m).prev.unwrap();
    let m_next = g.node(m).next.unwrap();
    let rule: RuleId;
    if g.node(m_prev).guard && g.node(m_next).guard {
        rule = g.node(m_prev).rule.unwrap();
        substitute_singlet_with_rule(g, n, rule);
    } else {
        rule = g.alloc_rule();
        g.rule_mut(rule).unique_index = g.total_rules;
        g.total_rules += 1;
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
fn check_doublet_match(g: &mut CffSubrGraph, n: NodeId) -> bool {
    // Every caller passes a linked node, so `next` is set.
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
            if di.arity == 2
                && di.start != n
                && !g.node(di.start).guard
                && !g.node(start_next).guard
            {
                process_match_doublet(g, di.start, n);
                true
            } else {
                true
            }
        }
    }
}
fn check_singlet_match(g: &mut CffSubrGraph, n: NodeId) -> bool {
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
fn append_node_to_graph(g: &mut CffSubrGraph, n: NodeId) {
    let root = g.root;
    let last = last_node_of(g, root);
    x_insert_node_after(g, last, n);
    if g.do_subroutinize
        && !check_doublet_match(g, last)
            && g.node(n).terminal.as_ref().unwrap().len() > 15 {
                check_singlet_match(g, n);
            }
}
pub fn insert_il_to_graph(g: &mut CffSubrGraph, il: &CffCharstringIl) {
    let mut blob = Buffer::new();
    let mut flush: bool = false;
    let mut last: bool = false;
    for j in 0..il.instr.len() as u32 {
        match il.instr[j as usize].kind as u32 {
            0 => {
                if flush {
                    let n = g.alloc_node();
                    g.node_mut(n).rule = None;
                    g.node_mut(n).terminal = Some(blob);
                    g.node_mut(n).last = last;
                    append_node_to_graph(g, n);
                    blob = Buffer::new();
                    flush = false;
                }
                merge_cs2_operand(&mut blob, il.instr[j as usize].d());
            }
            1 => {
                merge_cs2_operator(
                    &mut blob,
                    CffCharstringOperator(il.instr[j as usize].i()),
                );
                if il.instr[j as usize].i() == OP_ENDCHAR.0 {
                    last = true;
                }
                flush = true;
            }
            2 => {
                merge_cs2_special(&mut blob, il.instr[j as usize].i() as u8);
                flush = true;
            }
            _ => {}
        }
    }
    if !blob.data.is_empty() {
        let tail = g.alloc_node();
        g.node_mut(tail).rule = None;
        g.node_mut(tail).last = last;
        g.node_mut(tail).terminal = Some(blob);
        append_node_to_graph(g, tail);
    }
    let end = g.alloc_node();
    g.node_mut(end).rule = None;
    g.node_mut(end).terminal = Some(Buffer::new());
    g.node_mut(end).hard = true;
    append_node_to_graph(g, end);
    g.total_char_strings += 1;
}
