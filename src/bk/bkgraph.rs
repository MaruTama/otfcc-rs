use libc::fprintf;

use crate::bk::bkblock::{BkBlock, BkCellType, BkCellValue};
use crate::support::buffer::Buffer;
use crate::support::stdio::stderr;

// `BkGraph`/`BkGraphNode` used to hold `block: *mut BkBlock` -- a raw
// pointer into `bkblock.rs`'s construction API, alongside the
// (defensible-sounding, but ultimately wrong -- see `bkblock.rs`'s own
// comment) claim that every `BkBlock` has exactly one owner. The truth,
// once `bk_minimize_graph`/`replaceptr` are read closely: after
// minimization, many `Ptr` cells across the structure deliberately alias
// the same `BkBlock` (that's the entire point of minimizing), while
// `entries: Vec<BkGraphNode>` was *already* the graph's one true owner of
// every surviving block -- a flat arena wearing raw pointers as if they
// were indices.
//
// This file makes that arena explicit. `BlockId` is a stable identity
// assigned once, in post-order, by `dfs_convert` -- as of Stage D
// (2026-09), `bkblock.rs`'s construction API is *itself* an owned `Box`
// tree now (see that file's module comment for why that's sound), so
// `dfs_convert` receives and consumes an owned `BkBlock` by value rather
// than an unsafely-walked raw pointer, and needs no `unsafe` at all: a
// `Box` tree cannot alias or cycle, so the old Gray/Black
// revisit-in-progress guard (dead code even before Stage D -- see the
// removed comment on it) simply isn't needed here either. `blocks:
// Vec<ArenaBlock>` is the identity space `dfs_convert` populates:
// push-only, insertion-order-stable, indexed directly by `BlockId.0`, and
// NEVER physically reordered -- unlike `entries`, which this file's own
// algorithms sort (by height, then repeatedly by traversal order inside
// `bk_untangle_graph`'s retry loop). Splitting "stable identity" (`blocks`)
// from "current traversal position" (`entries`, `ArenaBlock.index`)
// preserves the same separation the raw-pointer design already had
// implicitly (pointer = stable identity; the old `_index` field = current
// position, rewritten after every sort).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct BlockId(u32);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
enum BkCellVisitState {
    White = 0,
    Gray = 1,
    Black = 2,
}

#[derive(Copy, Clone)]
enum ArenaCellValue {
    Int(u32),
    Ptr(Option<BlockId>),
}
#[derive(Copy, Clone)]
struct ArenaCell {
    t: BkCellType,
    value: ArenaCellValue,
}
impl ArenaCell {
    fn as_int(&self) -> u32 {
        match self.value {
            ArenaCellValue::Int(z) => z,
            ArenaCellValue::Ptr(_) => panic!("ArenaCell::as_int called on a pointer cell"),
        }
    }
    fn as_ptr(&self) -> Option<BlockId> {
        match self.value {
            ArenaCellValue::Ptr(p) => p,
            ArenaCellValue::Int(_) => panic!("ArenaCell::as_ptr called on an integer cell"),
        }
    }
}
fn arena_cell_is_pointer(cell: &ArenaCell) -> bool {
    cell.t >= BkCellType::P16
}

// The arena-resident counterpart of a converted `BkBlock`, addressable by
// `BlockId`. `visitstate`/`depth` are scratch state for this file's own
// `attract_bkgraph`/`dfs_attract_cells` untangling pass (unrelated to
// `dfs_convert`'s walk over the raw tree, which no longer needs any
// scratch fields on `BkBlock` itself now that it's an owned `Box` tree).
struct ArenaBlock {
    visitstate: BkCellVisitState,
    index: u32,
    depth: u32,
    cells: Vec<ArenaCell>,
}

#[derive(Copy, Clone)]
struct BkGraphNode {
    alias: u32,
    order: u32,
    height: u32,
    hash: u32,
    block: BlockId,
}
pub struct BkGraph {
    blocks: Vec<ArenaBlock>,
    entries: Vec<BkGraphNode>,
}
// `blocks`/`entries` are plain `Vec`s of `Copy`/owned data -- no cell holds
// a raw pointer needing a manual walk-and-free, so the derived `Drop` (just
// dropping both `Vec`s) is the whole teardown. `bk_delete_graph` is gone;
// callers just let a `BkGraph` go out of scope.

/// Consumes `b`, converting it (and everything it owns) into `blocks`/
/// `entries`, post-order. Fully safe: `b` is an owned `Box` tree (see
/// `bkblock.rs`'s module comment), so it cannot alias or cycle, and this
/// function simply recurses and lets each drained `BkBlock` shell drop
/// normally once its cells have been moved into a fresh `ArenaBlock`.
fn dfs_convert(
    b: BkBlock,
    blocks: &mut Vec<ArenaBlock>,
    entries: &mut Vec<BkGraphNode>,
    order: &mut u32,
) -> BlockId {
    let mut height: u32 = 0;
    let mut new_cells: Vec<ArenaCell> = Vec::with_capacity(b.cells.len());
    for cell in b.cells {
        let value = match cell.value {
            BkCellValue::Int(z) => ArenaCellValue::Int(z),
            BkCellValue::Ptr(Some(child)) => {
                let child_id = dfs_convert(*child, blocks, entries, order);
                let child_height = entries[child_id.0 as usize].height;
                if child_height.wrapping_add(1) > height {
                    height = child_height.wrapping_add(1);
                }
                ArenaCellValue::Ptr(Some(child_id))
            }
            BkCellValue::Ptr(None) => ArenaCellValue::Ptr(None),
        };
        new_cells.push(ArenaCell { t: cell.t, value });
    }
    *order = (*order).wrapping_add(1);
    let id = BlockId(blocks.len() as u32);
    blocks.push(ArenaBlock {
        visitstate: BkCellVisitState::Black,
        index: 0,
        depth: 0,
        cells: new_cells,
    });
    entries.push(BkGraphNode {
        alias: 0,
        order: *order,
        height,
        hash: 0,
        block: id,
    });
    id
}
fn by_height_cmp(a: &BkGraphNode, b: &BkGraphNode) -> ::core::cmp::Ordering {
    b.height.cmp(&a.height).then(a.order.cmp(&b.order))
}
fn by_order_cmp(blocks: &[ArenaBlock], a: &BkGraphNode, b: &BkGraphNode) -> ::core::cmp::Ordering {
    let ba = &blocks[a.block.0 as usize];
    let bb = &blocks[b.block.0 as usize];
    if ba.visitstate != bb.visitstate {
        (bb.visitstate as u32).cmp(&(ba.visitstate as u32))
    } else if ba.depth != bb.depth {
        ba.depth.cmp(&bb.depth)
    } else {
        b.order.cmp(&a.order)
    }
}
pub fn bk_new_graph_from_root_block(b: BkBlock) -> BkGraph {
    let mut blocks: Vec<ArenaBlock> = Vec::new();
    let mut entries: Vec<BkGraphNode> = Vec::new();
    let mut ts_order: u32 = 0;
    dfs_convert(b, &mut blocks, &mut entries, &mut ts_order);
    // `qsort` isn't guaranteed stable; `sort_by` is, matching the
    // conservative choice already made for `Coverage`/`ClassDef`/
    // `gpos_pair.rs`'s own qsort-scratch-buffer conversions.
    entries.sort_by(by_height_cmp);
    for (j, entry) in entries.iter_mut().enumerate() {
        blocks[entry.block.0 as usize].index = j as u32;
        entry.alias = j as u32;
    }
    BkGraph { blocks, entries }
}
fn gethash(blocks: &[ArenaBlock], block: &ArenaBlock) -> u32 {
    let mut h: u32 = 5381;
    for cell in block.cells.iter() {
        h = (h << 5).wrapping_add(h).wrapping_add(cell.t as u32);
        h = (h << 5).wrapping_add(h);
        match cell.t {
            BkCellType::B8 | BkCellType::B16 | BkCellType::B32 => {
                h = h.wrapping_add(cell.as_int());
            }
            BkCellType::P16 | BkCellType::P32 | BkCellType::Sp16 | BkCellType::Sp32 => {
                if let Some(p) = cell.as_ptr() {
                    // The original hashed the *target's current entries
                    // position* (`(*p)._index`), not any identity of the
                    // target itself -- preserved verbatim via the same
                    // `ArenaBlock.index` field `getoffset`/`replaceptr`
                    // already use for that meaning.
                    h = h.wrapping_add(blocks[p.0 as usize].index);
                }
            }
            _ => {}
        }
    }
    h
}
fn compareblock(blocks: &[ArenaBlock], a: BlockId, b: BlockId) -> bool {
    let ba = &blocks[a.0 as usize];
    let bb = &blocks[b.0 as usize];
    if ba.cells.len() != bb.cells.len() {
        return false;
    }
    for (ca, cb) in ba.cells.iter().zip(bb.cells.iter()) {
        if ca.t != cb.t {
            return false;
        }
        match ca.t {
            BkCellType::B8 | BkCellType::B16 | BkCellType::B32 => {
                if ca.as_int() != cb.as_int() {
                    return false;
                }
            }
            BkCellType::P16 | BkCellType::P32 | BkCellType::Sp16 | BkCellType::Sp32 => {
                if ca.as_ptr() != cb.as_ptr() {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}
fn compare_entry(blocks: &[ArenaBlock], a: &BkGraphNode, b: &BkGraphNode) -> bool {
    if a.hash != b.hash {
        return false;
    }
    compareblock(blocks, a.block, b.block)
}
fn replaceptr(blocks: &mut [ArenaBlock], entries: &[BkGraphNode], id: BlockId) {
    for i in 0..blocks[id.0 as usize].cells.len() {
        let cell = blocks[id.0 as usize].cells[i];
        match cell.t {
            BkCellType::P16 | BkCellType::P32 | BkCellType::Sp16 | BkCellType::Sp32 => {
                if let Some(p) = cell.as_ptr() {
                    let mut index = blocks[p.0 as usize].index;
                    while entries[index as usize].alias != index {
                        index = entries[index as usize].alias;
                    }
                    let resolved = entries[index as usize].block;
                    blocks[id.0 as usize].cells[i].value = ArenaCellValue::Ptr(Some(resolved));
                }
            }
            _ => {}
        }
    }
}
fn minimize_graph(graph: &mut BkGraph) {
    let blocks = &mut graph.blocks;
    let entries = &mut graph.entries;
    let mut rear: u32 = (entries.len() as u32).wrapping_sub(1);
    while rear > 0 {
        // front/rear bracket a run of same-height entries; the run's
        // extent is data-dependent, so this scan must stay a while loop.
        // Everything below it operates over the now-fixed [front, rear]
        // (or [0, front)) range and is a plain for loop.
        let mut front: u32 = rear;
        while entries[front as usize].height == entries[rear as usize].height && front > 0 {
            front = front.wrapping_sub(1);
        }
        front = front.wrapping_add(1);
        for j in front..=rear {
            let id = entries[j as usize].block;
            entries[j as usize].hash = gethash(blocks, &blocks[id.0 as usize]);
        }
        for j in front..=rear {
            if entries[j as usize].alias == j {
                for k in (j + 1)..=rear {
                    if entries[k as usize].alias == k
                        && compare_entry(blocks, &entries[j as usize], &entries[k as usize])
                    {
                        entries[k as usize].alias = j;
                    }
                }
            }
        }
        for j in 0..front {
            let id = entries[j as usize].block;
            replaceptr(blocks, entries, id);
        }
        rear = front.wrapping_sub(1);
    }
}
pub fn bk_minimize_graph(f: &mut BkGraph) {
    minimize_graph(f);
}
fn otfcc_bkblock_size(block: &ArenaBlock) -> usize {
    let mut size: usize = 0;
    for cell in block.cells.iter() {
        match cell.t {
            BkCellType::B8 => {
                size = size.wrapping_add(1);
            }
            BkCellType::B16 | BkCellType::P16 | BkCellType::Sp16 => {
                size = size.wrapping_add(2);
            }
            BkCellType::B32 | BkCellType::P32 | BkCellType::Sp32 => {
                size = size.wrapping_add(4);
            }
            _ => {}
        }
    }
    size
}
fn getoffset(
    offsets: &[usize],
    blocks: &[ArenaBlock],
    ref_id: BlockId,
    target: BlockId,
    bits: u8,
) -> u32 {
    let offref: usize = offsets[blocks[ref_id.0 as usize].index as usize];
    let offtgt: usize = offsets[blocks[target.0 as usize].index as usize];
    if (bits as i32) < 32_i32 && (offtgt < offref || offtgt.wrapping_sub(offref) >> bits as i32 != 0)
    {
        unsafe {
            fprintf(
                stderr,
                b"[otfcc-bk] Warning : Unable to fit offset %d into %d bits; output may be corrupted.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                offtgt.wrapping_sub(offref) as i32,
                bits as i32,
            );
        }
    }
    offtgt.wrapping_sub(offref) as u32
}
fn getoffset_untangle(
    offsets: &[usize],
    blocks: &[ArenaBlock],
    ref_id: BlockId,
    target: BlockId,
) -> i64 {
    let offref: usize = offsets[blocks[ref_id.0 as usize].index as usize];
    let offtgt: usize = offsets[blocks[target.0 as usize].index as usize];
    offtgt.wrapping_sub(offref) as i64
}
fn escalate_sppointers(
    blocks: &mut [ArenaBlock],
    entries: &mut [BkGraphNode],
    id: BlockId,
    order: &mut u32,
    depth: u32,
) {
    let children: Vec<BlockId> = blocks[id.0 as usize]
        .cells
        .iter()
        .filter(|c| arena_cell_is_pointer(c) && c.t >= BkCellType::Sp16)
        .filter_map(|c| c.as_ptr())
        .collect();
    for child in children {
        escalate_sppointers(blocks, entries, child, order, depth);
    }
    blocks[id.0 as usize].depth = depth;
    *order = (*order).wrapping_add(1);
    let pos = blocks[id.0 as usize].index;
    entries[pos as usize].order = *order;
}
fn dfs_attract_cells(
    blocks: &mut [ArenaBlock],
    entries: &mut [BkGraphNode],
    id: BlockId,
    order: &mut u32,
    depth: u32,
) {
    if blocks[id.0 as usize].visitstate != BkCellVisitState::White {
        if blocks[id.0 as usize].depth < depth {
            blocks[id.0 as usize].depth = depth;
        }
        return;
    }
    blocks[id.0 as usize].visitstate = BkCellVisitState::Gray;
    let children: Vec<BlockId> = blocks[id.0 as usize]
        .cells
        .iter()
        .rev()
        .filter(|c| arena_cell_is_pointer(c))
        .filter_map(|c| c.as_ptr())
        .collect();
    for child in children {
        dfs_attract_cells(blocks, entries, child, order, depth.wrapping_add(1));
    }
    *order = (*order).wrapping_add(1);
    let pos = blocks[id.0 as usize].index;
    entries[pos as usize].order = *order;
    escalate_sppointers(blocks, entries, id, order, depth);
    blocks[id.0 as usize].visitstate = BkCellVisitState::Black;
}
fn attract_bkgraph(graph: &mut BkGraph) {
    let blocks = &mut graph.blocks;
    let entries = &mut graph.entries;
    for (j, entry) in entries.iter_mut().enumerate() {
        let b = &mut blocks[entry.block.0 as usize];
        b.visitstate = BkCellVisitState::White;
        entry.order = 0;
        b.index = j as u32;
        b.depth = 0;
    }
    let mut order: u32 = 0;
    let root = entries[0].block;
    dfs_attract_cells(blocks, entries, root, &mut order, 0);
    // `qsort` isn't guaranteed stable; `sort_by` is, matching the
    // conservative choice already made for `Coverage`/`ClassDef`/
    // `gpos_pair.rs`'s own qsort-scratch-buffer conversions.
    entries.sort_by(|a, b| by_order_cmp(blocks, a, b));
    for (j, entry) in entries.iter().enumerate() {
        blocks[entry.block.0 as usize].index = j as u32;
    }
}
fn try_untabgle_block(
    blocks: &mut Vec<ArenaBlock>,
    entries: &mut Vec<BkGraphNode>,
    id: BlockId,
    offsets: &[usize],
) -> bool {
    let mut did_copy: bool = false;
    let cell_count = blocks[id.0 as usize].cells.len();
    for i in 0..cell_count {
        let cell = blocks[id.0 as usize].cells[i];
        match cell.t {
            BkCellType::P16 | BkCellType::Sp16 => {
                if let Some(p) = cell.as_ptr() {
                    let offset: i64 = getoffset_untangle(offsets, blocks, id, p);
                    if !(0..=0xffff).contains(&offset) {
                        // `BkCellType::Copy`'s only remaining use: append
                        // a "twin" of `p` (a fresh block whose cells are
                        // a shallow copy of `p`'s), placed right after
                        // every block seen so far in `entries`, so
                        // pointing at the twin instead of `p` fits in 16
                        // bits even when `p` itself doesn't -- the same
                        // trick the raw-pointer version implemented by
                        // pushing a `BkCellType::Copy` cell through the
                        // general construction API and letting
                        // `bkpushitems` splice `p`'s cells in; expressed
                        // directly here instead, since a `Copy` cell's
                        // only remaining job (per `bkblock.rs`'s own
                        // comment on it) is this exact arena-internal
                        // splice.
                        let twin_cells = blocks[p.0 as usize].cells.clone();
                        let twin_id = BlockId(blocks.len() as u32);
                        blocks.push(ArenaBlock {
                            visitstate: BkCellVisitState::White,
                            index: 0,
                            depth: 0,
                            cells: twin_cells,
                        });
                        entries.push(BkGraphNode {
                            alias: 0,
                            order: 0,
                            height: 0,
                            hash: 0,
                            block: twin_id,
                        });
                        let cell = &mut blocks[id.0 as usize].cells[i];
                        cell.t = BkCellType::Sp16;
                        cell.value = ArenaCellValue::Ptr(Some(twin_id));
                        did_copy = true;
                    }
                }
            }
            _ => {}
        }
    }
    did_copy
}
// Computes offsets[i+1] = offsets[i] + (serialized size of graph entry i,
// or 0 if bk_minimize_graph already merged it away and it's no longer
// BkCellVisitState::Black) for every entry, i.e. the running byte offset
// each surviving block will land at once serialized in order. Shared by
// try_untangle, build_graph, and estimate_size_of_graph, which each need
// this table before their own pass over the graph.
fn compute_block_offsets(blocks: &[ArenaBlock], entries: &[BkGraphNode]) -> Vec<usize> {
    let mut offsets: Vec<usize> = vec![0; entries.len() + 1];
    for (j, entry) in entries.iter().enumerate() {
        let block = &blocks[entry.block.0 as usize];
        let running = offsets[j];
        offsets[j + 1] = if block.visitstate == BkCellVisitState::Black {
            running.wrapping_add(otfcc_bkblock_size(block))
        } else {
            running
        };
    }
    offsets
}
fn try_untangle(graph: &mut BkGraph) -> bool {
    let offsets: Vec<usize> = compute_block_offsets(&graph.blocks, &graph.entries);
    let mut did_untangle: bool = false;
    let blocks = &mut graph.blocks;
    let entries = &mut graph.entries;
    for j in 0..entries.len() {
        let id = entries[j].block;
        if blocks[id.0 as usize].visitstate == BkCellVisitState::Black {
            did_untangle |= try_untabgle_block(blocks, entries, id, &offsets);
        }
    }
    did_untangle
}
fn otfcc_build_bkblock(buf: &mut Buffer, blocks: &[ArenaBlock], id: BlockId, offsets: &[usize]) {
    for cell in blocks[id.0 as usize].cells.iter() {
        match cell.t {
            BkCellType::B8 => {
                buf.write_u8(cell.as_int() as u8);
            }
            BkCellType::B16 => {
                buf.write_u16be(cell.as_int() as u16);
            }
            BkCellType::B32 => {
                buf.write_u32be(cell.as_int());
            }
            BkCellType::P16 | BkCellType::Sp16 => {
                if let Some(p) = cell.as_ptr() {
                    buf.write_u16be(getoffset(offsets, blocks, id, p, 16) as u16);
                } else {
                    buf.write_u16be(0);
                }
            }
            BkCellType::P32 | BkCellType::Sp32 => {
                if let Some(p) = cell.as_ptr() {
                    buf.write_u32be(getoffset(offsets, blocks, id, p, 32));
                } else {
                    buf.write_u32be(0);
                }
            }
            _ => {}
        }
    }
}
fn build_graph(graph: &BkGraph) -> Buffer {
    let mut buf = Buffer::new();
    let offsets: Vec<usize> = compute_block_offsets(&graph.blocks, &graph.entries);
    for entry in graph.entries.iter() {
        if graph.blocks[entry.block.0 as usize].visitstate == BkCellVisitState::Black {
            otfcc_build_bkblock(&mut buf, &graph.blocks, entry.block, &offsets);
        }
    }
    buf
}
pub fn bk_build_graph(f: &BkGraph) -> Buffer {
    build_graph(f)
}
fn estimate_size_of_graph(graph: &BkGraph) -> usize {
    let offsets: Vec<usize> = compute_block_offsets(&graph.blocks, &graph.entries);
    offsets[graph.entries.len()]
}
pub fn bk_estimate_size_of_graph(f: &BkGraph) -> usize {
    estimate_size_of_graph(f)
}
fn untangle_graph(graph: &mut BkGraph) {
    let mut passes: u16 = 0;
    attract_bkgraph(graph);
    loop {
        let tangled = try_untangle(graph);
        if tangled {
            attract_bkgraph(graph);
        }
        passes = passes.wrapping_add(1);
        if !(tangled && passes < 16) {
            break;
        }
    }
}
pub fn bk_untangle_graph(f: &mut BkGraph) {
    untangle_graph(f);
}
pub fn bk_build_block(root: BkBlock) -> Buffer {
    let mut f = bk_new_graph_from_root_block(root);
    bk_minimize_graph(&mut f);
    bk_untangle_graph(&mut f);
    bk_build_graph(&f)
}
pub fn bk_build_block_no_minimize(root: BkBlock) -> Buffer {
    let mut f = bk_new_graph_from_root_block(root);
    bk_untangle_graph(&mut f);
    bk_build_graph(&f)
}
