#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{fprintf, free};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BkGraphNode {
    pub alias: u32,
    pub order: u32,
    pub height: u32,
    pub hash: u32,
    pub block: *mut BkBlock,
}
// `length`/`free` (the realloc-with-slack growth bookkeeping `_bkgraph_grow`
// used to maintain by hand) are gone -- `entries: Vec<BkGraphNode>` tracks its
// own length and handles its own amortized growth, so every former `(*f).
// length` read is now `(*f).entries.len()`. Never `Copy`/`Clone`/`repr(C)`:
// `entries` owns heap data and this type is only ever reached through `*mut
// BkGraph`, never crosses the crate's FFI boundary (confirmed by grep -- the
// only other file touching `BkGraph` is `gpos_pair.rs`, always through the
// pointer `bk_new_graph_from_root_block` returns).
pub struct BkGraph {
    pub entries: Vec<BkGraphNode>,
}
use crate::support::stdio::{stderr};
use crate::support::buffer::{Buffer};
use crate::bk::bkblock::{BkCellVisitState, BkBlock, BkCellType, BkCellValue, bk_new_block, bk_ptr};
use crate::bk::bkblock::{bk_cell_is_pointer};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b, bufwrite8};


unsafe fn dfs_insert_cells(
    b: *mut BkBlock,
    f: *mut BkGraph,
    order: *mut u32,
) -> u32 {
    if b.is_null() || (*b)._visitstate == BkCellVisitState::Gray {
        return 0;
    }
    if (*b)._visitstate == BkCellVisitState::Black {
        return (*b)._height;
    }
    (*b)._visitstate = BkCellVisitState::Gray;
    let mut height: u32 = 0;
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        if bk_cell_is_pointer(cell) && !(*cell).as_ptr().is_null() {
            let that_height = dfs_insert_cells((*cell).as_ptr(), f, order);
            if that_height.wrapping_add(1) > height {
                height = that_height.wrapping_add(1);
            }
        }
    }
    *order = (*order).wrapping_add(1);
    (*b)._height = height;
    (*f).entries.push(BkGraphNode { alias: 0, order: *order, height, hash: 0, block: b });
    (*b)._visitstate = BkCellVisitState::Black;
    return height;
}
unsafe fn by_height_cmp(a: &BkGraphNode, b: &BkGraphNode) -> ::core::cmp::Ordering {
    b.height.cmp(&a.height).then(a.order.cmp(&b.order))
}
unsafe fn by_order_cmp(a: &BkGraphNode, b: &BkGraphNode) -> ::core::cmp::Ordering {
    if !a.block.is_null()
        && !b.block.is_null()
        && (*a.block)._visitstate as ::core::ffi::c_uint != (*b.block)._visitstate as ::core::ffi::c_uint
    {
        ((*b.block)._visitstate as u32).cmp(&((*a.block)._visitstate as u32))
    } else if !a.block.is_null() && !b.block.is_null() && (*a.block)._depth != (*b.block)._depth {
        (*a.block)._depth.cmp(&(*b.block)._depth)
    } else {
        b.order.cmp(&a.order)
    }
}
pub unsafe extern "C" fn bk_new_graph_from_root_block(b: *mut BkBlock) -> *mut BkGraph {
    let forest: *mut BkGraph = Box::into_raw(Box::new(BkGraph { entries: Vec::new() }));
    let mut ts_order: u32 = 0;
    dfs_insert_cells(b, forest, &raw mut ts_order);
    // `qsort` isn't guaranteed stable; `sort_by` is, matching the
    // conservative choice already made for `Coverage`/`ClassDef`/
    // `gpos_pair.rs`'s own qsort-scratch-buffer conversions.
    (*forest).entries.sort_by(|a, b| unsafe { by_height_cmp(a, b) });
    for (j, entry) in (*forest).entries.iter_mut().enumerate() {
        (*entry.block)._index = j as u32;
        entry.alias = j as u32;
    }
    return forest;
}
pub unsafe extern "C" fn bk_delete_graph(f: *mut BkGraph) {
    if f.is_null() {
        return;
    }
    for entry in &(*f).entries {
        let b: *mut BkBlock = entry.block;
        if !b.is_null() {
            if !(*b).cells.is_null() {
                free((*b).cells as *mut ::core::ffi::c_void);
            }
            free(b as *mut ::core::ffi::c_void);
        }
    }
    drop(Box::from_raw(f));
}
unsafe fn gethash(b: *mut BkBlock) -> u32 {
    let mut h: u32 = 5381;
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        h = (h << 5).wrapping_add(h).wrapping_add((*cell).t as u32);
        h = (h << 5).wrapping_add(h);
        match (*cell).t {
            BkCellType::B8 | BkCellType::B16 | BkCellType::B32 => {
                h = h.wrapping_add((*cell).as_int());
            }
            BkCellType::P16 | BkCellType::P32 | BkCellType::Sp16 | BkCellType::Sp32 => {
                let p = (*cell).as_ptr();
                if !p.is_null() {
                    h = h.wrapping_add((*p)._index);
                }
            }
            _ => {}
        }
    }
    return h;
}
unsafe fn compareblock(a: *mut BkBlock, b: *mut BkBlock) -> bool {
    if a.is_null() && b.is_null() {
        return true;
    }
    if a.is_null() || b.is_null() {
        return false;
    }
    if (*a).length != (*b).length {
        return false;
    }
    for j in 0..(*a).length {
        let ca = (*a).cells.offset(j as isize);
        let cb = (*b).cells.offset(j as isize);
        if (*ca).t != (*cb).t {
            return false;
        }
        match (*ca).t {
            BkCellType::B8 | BkCellType::B16 | BkCellType::B32 => {
                if (*ca).as_int() != (*cb).as_int() {
                    return false;
                }
            }
            BkCellType::P16 | BkCellType::P32 | BkCellType::Sp16 | BkCellType::Sp32 => {
                if (*ca).as_ptr() != (*cb).as_ptr() {
                    return false;
                }
            }
            _ => {}
        }
    }
    return true;
}
unsafe fn compare_entry(a: *const BkGraphNode, b: *const BkGraphNode) -> bool {
    if (*a).hash != (*b).hash {
        return false;
    }
    return compareblock((*a).block, (*b).block);
}
unsafe fn replaceptr(f: *mut BkGraph, b: *mut BkBlock) {
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            BkCellType::P16 | BkCellType::P32 | BkCellType::Sp16 | BkCellType::Sp32 => {
                let p = (*cell).as_ptr();
                if !p.is_null() {
                    let mut index: u32 = (*p)._index;
                    while (&(*f).entries)[index as usize].alias != index {
                        index = (&(*f).entries)[index as usize].alias;
                    }
                    (*cell).value =
                        BkCellValue::Ptr((&(*f).entries)[index as usize].block as *mut BkBlock);
                }
            }
            _ => {}
        }
    }
}
pub unsafe extern "C" fn bk_minimize_graph(f: *mut BkGraph) {
    let mut rear: u32 = ((*f).entries.len() as u32).wrapping_sub(1);
    while rear > 0 {
        // front/rear bracket a run of same-height entries; the run's extent
        // is data-dependent, so this scan must stay a while loop. Everything
        // below it operates over the now-fixed [front, rear] (or [0, front))
        // range and is a plain for loop.
        let mut front: u32 = rear;
        while (&(*f).entries)[front as usize].height == (&(*f).entries)[rear as usize].height
            && front > 0
        {
            front = front.wrapping_sub(1);
        }
        front = front.wrapping_add(1);
        for j in front..=rear {
            let block = (&(*f).entries)[j as usize].block;
            (&mut (*f).entries)[j as usize].hash = gethash(block);
        }
        for j in front..=rear {
            if (&(*f).entries)[j as usize].alias == j {
                for k in (j + 1)..=rear {
                    let a: *const BkGraphNode = &(&(*f).entries)[j as usize];
                    let b: *const BkGraphNode = &(&(*f).entries)[k as usize];
                    if (&(*f).entries)[k as usize].alias == k && compare_entry(a, b) {
                        (&mut (*f).entries)[k as usize].alias = j;
                    }
                }
            }
        }
        for j in 0..front {
            let block = (&(*f).entries)[j as usize].block;
            replaceptr(f, block);
        }
        rear = front.wrapping_sub(1);
    }
}
unsafe fn otfcc_bkblock_size(b: *mut BkBlock) -> usize {
    let mut size: usize = 0;
    for j in 0..(*b).length {
        match (*(*b).cells.offset(j as isize)).t {
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
    return size;
}
unsafe fn getoffset(
    offsets: &[usize],
    mut ref_0: *mut BkBlock,
    mut target: *mut BkBlock,
    mut bits: u8,
) -> u32 {
    let mut offref: usize = offsets[(*ref_0)._index as usize];
    let mut offtgt: usize = offsets[(*target)._index as usize];
    if (bits as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        && (offtgt < offref || offtgt.wrapping_sub(offref) >> bits as ::core::ffi::c_int != 0)
    {
        fprintf(
            stderr,
            b"[otfcc-bk] Warning : Unable to fit offset %d into %d bits; output may be corrupted.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            offtgt.wrapping_sub(offref) as i32,
            bits as ::core::ffi::c_int,
        );
    }
    return offtgt.wrapping_sub(offref) as u32;
}
unsafe fn getoffset_untangle(
    offsets: &[usize],
    mut ref_0: *mut BkBlock,
    mut target: *mut BkBlock,
) -> i64 {
    let mut offref: usize = offsets[(*ref_0)._index as usize];
    let mut offtgt: usize = offsets[(*target)._index as usize];
    return offtgt.wrapping_sub(offref) as i64;
}
unsafe fn escalate_sppointers(
    b: *mut BkBlock,
    f: *mut BkGraph,
    order: *mut u32,
    depth: u32,
) {
    if b.is_null() {
        return;
    }
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        if bk_cell_is_pointer(cell) && !(*cell).as_ptr().is_null() && (*cell).t >= BkCellType::Sp16 {
            escalate_sppointers((*cell).as_ptr(), f, order, depth);
        }
    }
    (*b)._depth = depth;
    *order = (*order).wrapping_add(1);
    (&mut (*f).entries)[(*b)._index as usize].order = *order;
}
unsafe fn dfs_attract_cells(
    b: *mut BkBlock,
    f: *mut BkGraph,
    order: *mut u32,
    depth: u32,
) {
    if b.is_null() {
        return;
    }
    if (*b)._visitstate != BkCellVisitState::White {
        if (*b)._depth < depth {
            (*b)._depth = depth;
        }
        return;
    }
    (*b)._visitstate = BkCellVisitState::Gray;
    // Visits cells in reverse index order (length-1 downto 0); equivalent to
    // c2rust's `j = length; loop { let fresh = j; j -= 1; if fresh == 0 {
    // break } ... use fresh-1 ... }` underflow-sentinel trick.
    for j in (0..(*b).length).rev() {
        let cell = (*b).cells.offset(j as isize);
        if bk_cell_is_pointer(cell) && !(*cell).as_ptr().is_null() {
            dfs_attract_cells(
                (*cell).as_ptr(),
                f,
                order,
                depth.wrapping_add(1),
            );
        }
    }
    *order = (*order).wrapping_add(1);
    (&mut (*f).entries)[(*b)._index as usize].order = *order;
    escalate_sppointers(b, f, order, depth);
    (*b)._visitstate = BkCellVisitState::Black;
}
unsafe fn attract_bkgraph(f: *mut BkGraph) {
    for (j, entry) in (*f).entries.iter_mut().enumerate() {
        (*entry.block)._visitstate = BkCellVisitState::White;
        entry.order = 0;
        (*entry.block)._index = j as u32;
        (*entry.block)._depth = 0;
    }
    let mut order: u32 = 0;
    dfs_attract_cells((&(*f).entries)[0].block, f, &raw mut order, 0);
    // `qsort` isn't guaranteed stable; `sort_by` is, matching the
    // conservative choice already made for `Coverage`/`ClassDef`/
    // `gpos_pair.rs`'s own qsort-scratch-buffer conversions.
    (*f).entries.sort_by(|a, b| unsafe { by_order_cmp(a, b) });
    for (j, entry) in (*f).entries.iter().enumerate() {
        (*entry.block)._index = j as u32;
    }
}
unsafe fn try_untabgle_block(
    f: *mut BkGraph,
    b: *mut BkBlock,
    offsets: &[usize],
    _passes: u16,
) -> bool {
    let mut did_copy: bool = false;
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            BkCellType::P16 | BkCellType::Sp16 => {
                let p = (*cell).as_ptr();
                if !p.is_null() {
                    let offset: i64 = getoffset_untangle(offsets, b, p);
                    if !(0..=0xffff).contains(&offset) {
                        let new_block = bk_new_block(&[bk_ptr(BkCellType::Copy, p)]);
                        (*f).entries.push(BkGraphNode {
                            alias: 0,
                            order: 0,
                            height: 0,
                            hash: 0,
                            block: new_block,
                        });
                        (*cell).t = BkCellType::Sp16;
                        (*cell).value = BkCellValue::Ptr(new_block);
                        did_copy = true;
                    }
                }
            }
            _ => {}
        }
    }
    return did_copy;
}
// Computes offsets[i+1] = offsets[i] + (serialized size of graph entry i, or
// 0 if bk_minimize_graph already merged it away and it's no longer
// BkCellVisitState::Black) for every entry, i.e. the running byte offset each surviving
// block will land at once serialized in order. Shared by try_untangle,
// bk_build_graph, and bk_estimate_size_of_graph, which each need this table
// before their own pass over the graph. `_line` is a vestige of the old
// `__caryll_allocate_clean` call (each original call site passed its own
// source line for the OOM message); kept as a parameter so callers don't
// need touching, unused now that the allocation is a plain `Vec`.
unsafe fn compute_block_offsets(f: *mut BkGraph, _line: ::core::ffi::c_ulong) -> Vec<usize> {
    let mut offsets: Vec<usize> = vec![0; (*f).entries.len() + 1];
    for (j, entry) in (*f).entries.iter().enumerate() {
        let block = entry.block;
        let running = offsets[j];
        offsets[j + 1] = if (*block)._visitstate == BkCellVisitState::Black {
            running.wrapping_add(otfcc_bkblock_size(block))
        } else {
            running
        };
    }
    offsets
}
unsafe fn try_untangle(f: *mut BkGraph, passes: u16) -> bool {
    let offsets: Vec<usize> = compute_block_offsets(f, 294);
    let mut did_untangle: bool = false;
    for j in 0..(*f).entries.len() {
        let block = (&(*f).entries)[j].block;
        if (*block)._visitstate == BkCellVisitState::Black {
            did_untangle |= try_untabgle_block(f, block, &offsets, passes);
        }
    }
    return did_untangle;
}
unsafe fn otfcc_build_bkblock(buf: *mut Buffer, b: *mut BkBlock, offsets: &[usize]) {
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            BkCellType::B8 => {
                bufwrite8(buf, (*cell).as_int() as u8);
            }
            BkCellType::B16 => {
                bufwrite16b(buf, (*cell).as_int() as u16);
            }
            BkCellType::B32 => {
                bufwrite32b(buf, (*cell).as_int());
            }
            BkCellType::P16 | BkCellType::Sp16 => {
                let p = (*cell).as_ptr();
                if !p.is_null() {
                    bufwrite16b(buf, getoffset(offsets, b, p, 16) as u16);
                } else {
                    bufwrite16b(buf, 0);
                }
            }
            BkCellType::P32 | BkCellType::Sp32 => {
                let p = (*cell).as_ptr();
                if !p.is_null() {
                    bufwrite32b(buf, getoffset(offsets, b, p, 32));
                } else {
                    bufwrite32b(buf, 0);
                }
            }
            _ => {}
        }
    }
}
pub unsafe extern "C" fn bk_build_graph(f: *mut BkGraph) -> *mut Buffer {
    let buf: *mut Buffer = bufnew();
    let offsets: Vec<usize> = compute_block_offsets(f, 352);
    for j in 0..(*f).entries.len() {
        let block = (&(*f).entries)[j].block;
        if (*block)._visitstate == BkCellVisitState::Black {
            otfcc_build_bkblock(buf, block, &offsets);
        }
    }
    return buf;
}
pub unsafe extern "C" fn bk_estimate_size_of_graph(f: *mut BkGraph) -> usize {
    let offsets: Vec<usize> = compute_block_offsets(f, 373);
    let estimated_size: usize = offsets[(*f).entries.len()];
    return estimated_size;
}
pub unsafe extern "C" fn bk_untangle_graph(f: *mut BkGraph) {
    let mut passes: u16 = 0;
    attract_bkgraph(f);
    loop {
        let tangled = try_untangle(f, passes);
        if tangled {
            attract_bkgraph(f);
        }
        passes = passes.wrapping_add(1);
        if !(tangled && passes < 16) {
            break;
        }
    }
}
pub unsafe extern "C" fn bk_build_block(root: *mut BkBlock) -> *mut Buffer {
    let f: *mut BkGraph = bk_new_graph_from_root_block(root);
    bk_minimize_graph(f);
    bk_untangle_graph(f);
    let buf: *mut Buffer = bk_build_graph(f);
    bk_delete_graph(f);
    return buf;
}
pub unsafe extern "C" fn bk_build_block_no_minimize(root: *mut BkBlock) -> *mut Buffer {
    let f: *mut BkGraph = bk_new_graph_from_root_block(root);
    bk_untangle_graph(f);
    let buf: *mut Buffer = bk_build_graph(f);
    bk_delete_graph(f);
    return buf;
}
