use super::*;
use super::graph::last_node_of;
use otfcc_binary::Buffer;

use crate::libcff::index::CffIndex;
use crate::libcff::index::{build_index, new_index_by_callback};
use crate::support::error::BuildError;
use crate::libcff::writer::{
    merge_cs2_int, merge_cs2_operator,
};
use crate::libcff::{
    OP_CALLGSUBR, OP_CALLSUBR, OP_RETURN, TYPE2_MAX_SUBRS, TYPE2_SUBR_NESTING,
};

fn stat_height(g: &mut CffSubrGraph, r: RuleId, height: u32) {
    if height > g.rule(r).height {
        g.rule_mut(r).height = height;
    }
    let mut effective_length: u32 = 0;
    let guard = g.rule(r).guard;
    let mut e = g.node(guard).next.unwrap();
    while e != guard {
        let rule = g.node(e).rule;
        if let Some(er) = rule {
            stat_height(g, er, height + 1);
            effective_length += 4;
        } else {
            let terminal_len = g.node(e).terminal.as_ref().unwrap().data.len();
            effective_length += terminal_len as u32;
        }
        e = g.node(e).next.unwrap();
    }
    g.rule_mut(r).effective_length = effective_length;
}
fn number_a_subroutine(g: &mut CffSubrGraph, r: RuleId, current: &mut u32) {
    if g.rule(r).numbered {
        return;
    }
    if g.rule(r).height >= TYPE2_SUBR_NESTING {
        return;
    }
    if g.rule(r)
        .effective_length
        .wrapping_sub(4)
        .wrapping_mul(g.rule(r).refcount.wrapping_sub(1))
        .wrapping_sub(4)
        == 0
    {
        return;
    }
    g.rule_mut(r).number = *current;
    *current += 1;
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
fn number_subroutines(g: &mut CffSubrGraph) -> u32 {
    let mut current: u32 = 0;
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
fn subroutine_bias(cnt: i32) -> i32 {
    if cnt < 1240 {
        return 107;
    } else if cnt < 33900 {
        return 1131;
    } else {
        return 32768;
    };
}
fn ends_with_end_char(g: &CffSubrGraph, rule: RuleId) -> bool {
    let node = last_node_of(g, rule);
    let n = g.node(node);
    if n.terminal.is_some() {
        return n.last;
    } else {
        return ends_with_end_char(g, n.rule.unwrap());
    };
}
// Which buffer `serialize_node_to_buffer` writes to. It is resolved to a
// `&mut Buffer` only where it is used (`resolve_subr_ref`), because the
// recursive call also takes `lsubrs`/`gsubrs`.
#[derive(Clone, Copy, Debug)]
enum SubrRef {
    Top,
    LSubr(usize),
    GSubr(usize),
}
fn resolve_subr_ref<'a>(
    r: SubrRef,
    top: &'a mut Buffer,
    gsubrs: &'a mut [Buffer],
    lsubrs: &'a mut [Buffer],
) -> &'a mut Buffer {
    match r {
        SubrRef::Top => top,
        SubrRef::LSubr(i) => &mut lsubrs[i],
        SubrRef::GSubr(i) => &mut gsubrs[i],
    }
}
fn serialize_node_to_buffer(
    g: &mut CffSubrGraph,
    node: NodeId,
    buf: SubrRef,
    top: &mut Buffer,
    gsubrs: &mut [Buffer],
    max_g_subrs: u32,
    lsubrs: &mut [Buffer],
    max_l_subrs: u32,
) {
    let rule = g.node(node).rule;
    if let Some(r) = rule {
        let (numbered, number, r_height) = {
            let ru = g.rule(r);
            (ru.numbered, ru.number, ru.height)
        };
        if numbered
            && number < max_l_subrs + max_g_subrs
            && r_height < TYPE2_SUBR_NESTING
        {
            let target: SubrRef;
            if number < max_l_subrs {
                let stacknum: i32 = number as i32 - subroutine_bias(max_l_subrs as i32);
                target = SubrRef::LSubr(number as usize);
                merge_cs2_int(resolve_subr_ref(buf, top, gsubrs, lsubrs), stacknum);
                merge_cs2_operator(resolve_subr_ref(buf, top, gsubrs, lsubrs), OP_CALLSUBR);
            } else {
                let gsubr = number - max_l_subrs;
                let stacknum: i32 = gsubr as i32 - subroutine_bias(max_g_subrs as i32);
                target = SubrRef::GSubr(gsubr as usize);
                merge_cs2_int(resolve_subr_ref(buf, top, gsubrs, lsubrs), stacknum);
                merge_cs2_operator(resolve_subr_ref(buf, top, gsubrs, lsubrs), OP_CALLGSUBR);
            }
            if !g.rule(r).printed {
                g.rule_mut(r).printed = true;
                let guard = g.rule(r).guard;
                let mut e = g.node(guard).next.unwrap();
                while e != guard {
                    let next = g.node(e).next.unwrap();
                    serialize_node_to_buffer(
                        g,
                        e,
                        target,
                        top,
                        gsubrs,
                        max_g_subrs,
                        lsubrs,
                        max_l_subrs,
                    );
                    e = next;
                }
                if !ends_with_end_char(g, r) {
                    merge_cs2_operator(
                        resolve_subr_ref(target, top, gsubrs, lsubrs),
                        OP_RETURN,
                    );
                }
            }
        } else {
            let guard = g.rule(r).guard;
            let mut e = g.node(guard).next.unwrap();
            while e != guard {
                let next = g.node(e).next.unwrap();
                serialize_node_to_buffer(
                    g, e, buf, top, gsubrs, max_g_subrs, lsubrs, max_l_subrs,
                );
                e = next;
            }
        }
    } else {
        resolve_subr_ref(buf, top, gsubrs, lsubrs)
            .write_buffer(g.node(node).terminal.as_ref().unwrap());
    };
}
pub fn il_graph_to_buffers(
    g: &mut CffSubrGraph,
) -> Result<(Buffer, Buffer, Buffer), BuildError> {
    let root = g.root;
    stat_height(g, root, 0);
    let max_subroutines: u32 = number_subroutines(g);
    tracing::debug!("[libcff] Total {} subroutines extracted.", max_subroutines);
    let mut max_l_subrs: u32 = max_subroutines;
    let mut max_g_subrs: u32 = 0;
    if max_l_subrs > TYPE2_MAX_SUBRS {
        max_l_subrs = TYPE2_MAX_SUBRS;
        max_g_subrs = max_subroutines - max_l_subrs;
    }
    if max_g_subrs > TYPE2_MAX_SUBRS {
        max_g_subrs = TYPE2_MAX_SUBRS;
    }
    // Split evenly between local and global subroutines.
    let total: u32 = max_l_subrs + max_g_subrs;
    max_l_subrs = total / 2;
    max_g_subrs = total - max_l_subrs;
    let zero_buffer = Buffer {
        cursor: 0,
        data: Vec::new(),
    };
    let mut char_strings: Vec<Buffer> =
        vec![zero_buffer.clone(); g.total_char_strings as usize + 1];
    let mut lsubrs: Vec<Buffer> = vec![zero_buffer.clone(); max_l_subrs as usize + 1];
    let mut gsubrs: Vec<Buffer> = vec![zero_buffer; max_g_subrs as usize + 1];
    // The charstring the next top-level node belongs to.
    let mut j: u32 = 0;
    let root = g.root;
    let guard = g.rule(root).guard;
    let mut e = g.node(guard).next.unwrap();
    while e != guard {
        let next = g.node(e).next.unwrap();
        let (e_rule, e_terminal_is_some, e_hard) = {
            let en = g.node(e);
            (en.rule, en.terminal.is_some(), en.hard)
        };
        serialize_node_to_buffer(
            g,
            e,
            SubrRef::Top,
            &mut char_strings[j as usize],
            &mut gsubrs[..],
            max_g_subrs,
            &mut lsubrs[..],
            max_l_subrs,
        );
        if e_rule.is_none() && e_terminal_is_some && e_hard {
            j += 1;
        }
        e = next;
    }
    let is: CffIndex = new_index_by_callback(
        g.total_char_strings,
        char_strings[..g.total_char_strings as usize].iter().cloned(),
    );
    let igs: CffIndex =
        new_index_by_callback(max_g_subrs, gsubrs[..max_g_subrs as usize].iter().cloned());
    let ils: CffIndex =
        new_index_by_callback(max_l_subrs, lsubrs[..max_l_subrs as usize].iter().cloned());
    for entry in char_strings.iter_mut().take(g.total_char_strings as usize) {
        entry.data = Vec::new();
    }
    for entry in gsubrs.iter_mut().take(max_g_subrs as usize) {
        entry.data = Vec::new();
    }
    for entry in lsubrs.iter_mut().take(max_l_subrs as usize) {
        entry.data = Vec::new();
    }
    let s = build_index(&is)?;
    let gs = build_index(&igs)?;
    let ls = build_index(&ils)?;
    Ok((s, gs, ls))
}

// Coverage for the subroutinizer at the two entry points `table/cff.rs`
