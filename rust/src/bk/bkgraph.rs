use libc::fprintf;

use crate::bk::bkblock::{BkBlock, BkCellType, BkCellValue, BkCellVisitState};
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
// every surviving block (`bk_delete_graph` frees by walking `entries`
// directly, never by walking cell pointers) -- a flat arena wearing raw
// pointers as if they were indices.
//
// This file makes that arena explicit. `BlockId` is a stable identity
// assigned once, in post-order, by `dfs_convert` (the one remaining
// genuinely unsafe walk in this module, and the only place that still
// receives a raw `*mut BkBlock` from `bkblock.rs`'s untouched
// construction API). `blocks: Vec<ArenaBlock>` is that identity space:
// push-only, insertion-order-stable, indexed directly by `BlockId.0`,
// and NEVER physically reordered -- unlike `entries`, which this file's
// own algorithms sort (by height, then repeatedly by traversal order
// inside `bk_untangle_graph`'s retry loop) exactly as `entries` always
// has. Splitting "stable identity" (`blocks`) from "current traversal
// position" (`entries`, `ArenaBlock.index`) preserves the same
// separation the raw-pointer design already had implicitly (pointer =
// stable identity; the old `_index` field = current position, rewritten
// after every sort) -- the design task here was to preserve that
// separation, not invent it.
//
// Every public function below keeps its original `*mut BkGraph`/
// `*mut BkBlock`-shaped signature (the ~20 files across this crate that
// call into this module do so only through this opaque API, confirmed
// by grep, so none of them need to change), but each is now a thin
// `unsafe fn` wrapper around one narrow `unsafe { &mut *f }`/
// `unsafe { &*f }` reborrow, delegating to a fully safe function that
// does the actual work against `&BkGraph`/`&mut BkGraph` -- the same
// "safe fn, narrow unsafe bridge" shape used throughout this migration
// (e.g. `vf/vq.rs`'s `vqs_compare`).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct BlockId(u32);

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

// `BkBlock`'s own `_visitstate`/`_depth`/`_index` scratch fields (in
// `bkblock.rs`, untouched by this conversion) still drive `dfs_convert`'s
// walk over the still-raw tree below, exactly as `dfs_insert_cells` used
// to. This struct is their arena-resident counterpart, addressable by
// `BlockId` once a block has been converted, with one deliberate
// omission: `_height` on the raw `BkBlock` was write-only even in the
// original code (every height *read* anywhere in this file, before or
// after this conversion, goes through the `BkGraphNode`/`entries` copy
// of it, never through the block itself) -- dropped here rather than
// carried forward as genuinely dead state.
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

// The one remaining genuinely unsafe walk in this module: it receives a
// raw `*mut BkBlock` tree from `bkblock.rs`'s construction API and
// converts it, post-order, into the arena above. `b`'s own
// `_visitstate`/`_index` fields drive the walk (`_index` is repurposed,
// once a block turns Black, to remember the `BlockId` just assigned to
// it -- see the Black arm below); every raw block is freed exactly once,
// from `to_free`, only after the whole tree has been walked, so a block
// revisited before its first conversion finishes (the Black-memoization
// path) is never read after being freed. `to_free` accumulates in the
// same post-order sequence as `blocks` itself; freeing it in one pass at
// the end (rather than freeing each block the moment its `ArenaBlock` is
// pushed) is what makes memoized revisits safe even for a
// hypothetically-shared raw tree -- confirmed absent from every one of
// this crate's ~20 consumer files today (a full audit, not a sample),
// but this module doesn't get to assume that stays true forever.
//
// A genuine *cycle* in the raw tree (a cell pointing back at a Gray
// ancestor still being converted) is a different case from ordinary
// sharing: that ancestor has no `BlockId` yet (post-order assignment
// hasn't reached it), so there is no value this function could honestly
// return for that one cell. Unlike ordinary sharing, no evidence of this
// ever having been exercised exists anywhere in this crate's history
// (the original `dfs_insert_cells`'s own Gray guard is annotated, in
// `bkblock.rs`, as "unverified circumstantial evidence" the authors
// themselves were never sure applied at construction time) -- so this
// conversion treats a Gray revisit as if the cell were simply null
// (`None`, no height contribution), the same fallback this migration has
// used elsewhere for pathological-and-unreached shapes, rather than
// inventing forward-reference plumbing for a path nothing has ever hit.
unsafe fn dfs_convert(
    b: *mut BkBlock,
    blocks: &mut Vec<ArenaBlock>,
    entries: &mut Vec<BkGraphNode>,
    to_free: &mut Vec<*mut BkBlock>,
    order: &mut u32,
) -> Option<BlockId> {
    unsafe {
        if b.is_null() || (*b)._visitstate == BkCellVisitState::Gray {
            return None;
        }
        if (*b)._visitstate == BkCellVisitState::Black {
            return Some(BlockId((*b)._index));
        }
        (*b)._visitstate = BkCellVisitState::Gray;
        let mut height: u32 = 0;
        let mut new_cells: Vec<ArenaCell> = Vec::with_capacity((*b).cells.len());
        for cell in (*b).cells.iter() {
            let value = match cell.value {
                BkCellValue::Int(z) => ArenaCellValue::Int(z),
                BkCellValue::Ptr(p) if !p.is_null() => {
                    let child = dfs_convert(p, blocks, entries, to_free, order);
                    if let Some(id) = child {
                        let child_height = entries[id.0 as usize].height;
                        if child_height.wrapping_add(1) > height {
                            height = child_height.wrapping_add(1);
                        }
                    }
                    ArenaCellValue::Ptr(child)
                }
                BkCellValue::Ptr(_) => ArenaCellValue::Ptr(None),
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
        (*b)._index = id.0;
        (*b)._visitstate = BkCellVisitState::Black;
        to_free.push(b);
        Some(id)
    }
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
pub unsafe fn bk_new_graph_from_root_block(b: *mut BkBlock) -> *mut BkGraph {
    let mut blocks: Vec<ArenaBlock> = Vec::new();
    let mut entries: Vec<BkGraphNode> = Vec::new();
    let mut to_free: Vec<*mut BkBlock> = Vec::new();
    let mut ts_order: u32 = 0;
    unsafe {
        dfs_convert(b, &mut blocks, &mut entries, &mut to_free, &mut ts_order);
        for raw in to_free {
            drop(Box::from_raw(raw));
        }
    }
    // `qsort` isn't guaranteed stable; `sort_by` is, matching the
    // conservative choice already made for `Coverage`/`ClassDef`/
    // `gpos_pair.rs`'s own qsort-scratch-buffer conversions.
    entries.sort_by(by_height_cmp);
    for (j, entry) in entries.iter_mut().enumerate() {
        blocks[entry.block.0 as usize].index = j as u32;
        entry.alias = j as u32;
    }
    Box::into_raw(Box::new(BkGraph { blocks, entries }))
}
pub unsafe fn bk_delete_graph(f: *mut BkGraph) {
    if f.is_null() {
        return;
    }
    // `blocks`/`entries` are plain `Vec`s of `Copy`/owned data now -- no
    // cell holds a raw pointer needing a manual walk-and-free, so
    // dropping the boxed `BkGraph` (which drops both `Vec`s) is the
    // whole teardown.
    unsafe {
        drop(Box::from_raw(f));
    }
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
pub unsafe fn bk_minimize_graph(f: *mut BkGraph) {
    minimize_graph(unsafe { &mut *f });
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
pub unsafe fn bk_build_graph(f: *mut BkGraph) -> Buffer {
    build_graph(unsafe { &*f })
}
fn estimate_size_of_graph(graph: &BkGraph) -> usize {
    let offsets: Vec<usize> = compute_block_offsets(&graph.blocks, &graph.entries);
    offsets[graph.entries.len()]
}
pub unsafe fn bk_estimate_size_of_graph(f: *mut BkGraph) -> usize {
    estimate_size_of_graph(unsafe { &*f })
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
pub unsafe fn bk_untangle_graph(f: *mut BkGraph) {
    untangle_graph(unsafe { &mut *f });
}
pub unsafe fn bk_build_block(root: *mut BkBlock) -> Buffer {
    unsafe {
        let f: *mut BkGraph = bk_new_graph_from_root_block(root);
        bk_minimize_graph(f);
        bk_untangle_graph(f);
        let buf = bk_build_graph(f);
        bk_delete_graph(f);
        buf
    }
}
pub unsafe fn bk_build_block_no_minimize(root: *mut BkBlock) -> Buffer {
    unsafe {
        let f: *mut BkGraph = bk_new_graph_from_root_block(root);
        bk_untangle_graph(f);
        let buf = bk_build_graph(f);
        bk_delete_graph(f);
        buf
    }
}
