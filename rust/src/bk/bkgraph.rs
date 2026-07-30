#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{fprintf, free, qsort};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BkGraphNode {
    pub alias: u32,
    pub order: u32,
    pub height: u32,
    pub hash: u32,
    pub block: *mut BkBlock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BkGraph {
    pub length: u32,
    pub free: u32,
    pub entries: *mut BkGraphNode,
}
use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::buffer::{Buffer};
use crate::bk::bkblock::{BkCellVisitState, BkBlock, BkCellType, BkCell, bk_new_block, bk_ptr};
use crate::bk::bkblock::{bk_cell_is_pointer};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b, bufwrite8};


unsafe extern "C" fn _bkgraph_grow(mut f: *mut BkGraph) -> *mut BkGraphNode {
    if (*f).free != 0 {
        (*f).length = (*f).length.wrapping_add(1);
        (*f).free = (*f).free.wrapping_sub(1);
    } else {
        (*f).length = (*f).length.wrapping_add(1 as u32);
        (*f).free = (*f).length >> 1 as ::core::ffi::c_int & 0xffffff as u32;
        (*f).entries = __caryll_reallocate(
            (*f).entries as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<BkGraphNode>() as usize)
                .wrapping_mul((*f).length.wrapping_add((*f).free) as usize),
            10 as ::core::ffi::c_ulong,
        ) as *mut BkGraphNode;
    }
    return (*f)
        .entries
        .offset((*f).length.wrapping_sub(1 as u32) as isize) as *mut BkGraphNode;
}
unsafe extern "C" fn dfs_insert_cells(
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
        if bk_cell_is_pointer(cell) && !(*cell).c2rust_unnamed.p.is_null() {
            let that_height = dfs_insert_cells((*cell).c2rust_unnamed.p as *mut BkBlock, f, order);
            if that_height.wrapping_add(1) > height {
                height = that_height.wrapping_add(1);
            }
        }
    }
    let e: *mut BkGraphNode = _bkgraph_grow(f);
    (*e).alias = 0;
    (*e).block = b;
    *order = (*order).wrapping_add(1);
    (*e).order = *order;
    (*b)._height = height;
    (*e).height = (*b)._height;
    (*b)._visitstate = BkCellVisitState::Black;
    return height;
}
unsafe extern "C" fn _by_height(
    mut _a: *const ::core::ffi::c_void,
    mut _b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const BkGraphNode = _a as *const BkGraphNode;
    let mut b: *const BkGraphNode = _b as *const BkGraphNode;
    return (if (*a).height == (*b).height {
        (*a).order.wrapping_sub((*b).order)
    } else {
        (*b).height.wrapping_sub((*a).height)
    }) as ::core::ffi::c_int;
}
unsafe extern "C" fn _by_order(
    mut _a: *const ::core::ffi::c_void,
    mut _b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const BkGraphNode = _a as *const BkGraphNode;
    let mut b: *const BkGraphNode = _b as *const BkGraphNode;
    return (if !(*a).block.is_null()
        && !(*b).block.is_null()
        && (*(*a).block)._visitstate as ::core::ffi::c_uint
            != (*(*b).block)._visitstate as ::core::ffi::c_uint
    {
        ((*(*b).block)._visitstate as u32).wrapping_sub((*(*a).block)._visitstate as u32)
    } else if !(*a).block.is_null()
        && !(*b).block.is_null()
        && (*(*a).block)._depth != (*(*b).block)._depth
    {
        (*(*a).block)._depth.wrapping_sub((*(*b).block)._depth)
    } else {
        (*b).order.wrapping_sub((*a).order)
    }) as ::core::ffi::c_int;
}
pub unsafe extern "C" fn bk_new_graph_from_root_block(b: *mut BkBlock) -> *mut BkGraph {
    let forest: *mut BkGraph = __caryll_allocate_clean(
        ::core::mem::size_of::<BkGraph>() as usize,
        55 as ::core::ffi::c_ulong,
    ) as *mut BkGraph;
    let mut ts_order: u32 = 0;
    dfs_insert_cells(b, forest, &raw mut ts_order);
    qsort(
        (*forest).entries as *mut ::core::ffi::c_void,
        (*forest).length as usize,
        ::core::mem::size_of::<BkGraphNode>() as usize,
        Some(
            _by_height
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    for j in 0..(*forest).length {
        let entry = (*forest).entries.offset(j as isize);
        (*(*entry).block)._index = j;
        (*entry).alias = j;
    }
    return forest;
}
pub unsafe extern "C" fn bk_delete_graph(f: *mut BkGraph) {
    if f.is_null() || (*f).entries.is_null() {
        return;
    }
    for j in 0..(*f).length {
        let b: *mut BkBlock = (*(*f).entries.offset(j as isize)).block;
        if !b.is_null() && !(*b).cells.is_null() {
            free((*b).cells as *mut ::core::ffi::c_void);
            (*b).cells = ::core::ptr::null_mut::<BkCell>();
        }
        free(b as *mut ::core::ffi::c_void);
    }
    free((*f).entries as *mut ::core::ffi::c_void);
    (*f).entries = ::core::ptr::null_mut::<BkGraphNode>();
    free(f as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn gethash(b: *mut BkBlock) -> u32 {
    let mut h: u32 = 5381;
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        h = (h << 5).wrapping_add(h).wrapping_add((*cell).t as u32);
        h = (h << 5).wrapping_add(h);
        match (*cell).t {
            BkCellType::B8 | BkCellType::B16 | BkCellType::B32 => {
                h = h.wrapping_add((*cell).c2rust_unnamed.z);
            }
            BkCellType::P16 | BkCellType::P32 | BkCellType::Sp16 | BkCellType::Sp32 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    h = h.wrapping_add((*(*cell).c2rust_unnamed.p)._index);
                }
            }
            _ => {}
        }
    }
    return h;
}
unsafe extern "C" fn compareblock(a: *mut BkBlock, b: *mut BkBlock) -> bool {
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
                if (*ca).c2rust_unnamed.z != (*cb).c2rust_unnamed.z {
                    return false;
                }
            }
            BkCellType::P16 | BkCellType::P32 | BkCellType::Sp16 | BkCellType::Sp32 => {
                if (*ca).c2rust_unnamed.p != (*cb).c2rust_unnamed.p {
                    return false;
                }
            }
            _ => {}
        }
    }
    return true;
}
unsafe extern "C" fn compare_entry(a: *mut BkGraphNode, b: *mut BkGraphNode) -> bool {
    if (*a).hash != (*b).hash {
        return false;
    }
    return compareblock((*a).block, (*b).block);
}
unsafe extern "C" fn replaceptr(f: *mut BkGraph, b: *mut BkBlock) {
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            BkCellType::P16 | BkCellType::P32 | BkCellType::Sp16 | BkCellType::Sp32 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    let mut index: u32 = (*(*cell).c2rust_unnamed.p)._index;
                    while (*(*f).entries.offset(index as isize)).alias != index {
                        index = (*(*f).entries.offset(index as isize)).alias;
                    }
                    (*cell).c2rust_unnamed.p =
                        (*(*f).entries.offset(index as isize)).block as *mut BkBlock;
                }
            }
            _ => {}
        }
    }
}
pub unsafe extern "C" fn bk_minimize_graph(f: *mut BkGraph) {
    let mut rear: u32 = (*f).length.wrapping_sub(1);
    while rear > 0 {
        // front/rear bracket a run of same-height entries; the run's extent
        // is data-dependent, so this scan must stay a while loop. Everything
        // below it operates over the now-fixed [front, rear] (or [0, front))
        // range and is a plain for loop.
        let mut front: u32 = rear;
        while (*(*f).entries.offset(front as isize)).height
            == (*(*f).entries.offset(rear as isize)).height
            && front > 0
        {
            front = front.wrapping_sub(1);
        }
        front = front.wrapping_add(1);
        for j in front..=rear {
            let entry = (*f).entries.offset(j as isize);
            (*entry).hash = gethash((*entry).block);
        }
        for j in front..=rear {
            let a: *mut BkGraphNode = (*f).entries.offset(j as isize);
            if (*a).alias == j {
                for k in (j + 1)..=rear {
                    let b: *mut BkGraphNode = (*f).entries.offset(k as isize);
                    if (*b).alias == k && compare_entry(a, b) {
                        (*b).alias = j;
                    }
                }
            }
        }
        for j in 0..front {
            replaceptr(f, (*(*f).entries.offset(j as isize)).block);
        }
        rear = front.wrapping_sub(1);
    }
}
unsafe extern "C" fn otfcc_bkblock_size(b: *mut BkBlock) -> usize {
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
unsafe extern "C" fn getoffset(
    mut offsets: *mut usize,
    mut ref_0: *mut BkBlock,
    mut target: *mut BkBlock,
    mut bits: u8,
) -> u32 {
    let mut offref: usize = *offsets.offset((*ref_0)._index as isize);
    let mut offtgt: usize = *offsets.offset((*target)._index as isize);
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
unsafe extern "C" fn getoffset_untangle(
    mut offsets: *mut usize,
    mut ref_0: *mut BkBlock,
    mut target: *mut BkBlock,
) -> i64 {
    let mut offref: usize = *offsets.offset((*ref_0)._index as isize);
    let mut offtgt: usize = *offsets.offset((*target)._index as isize);
    return offtgt.wrapping_sub(offref) as i64;
}
unsafe extern "C" fn escalate_sppointers(
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
        if bk_cell_is_pointer(cell) && !(*cell).c2rust_unnamed.p.is_null() && (*cell).t >= BkCellType::Sp16 {
            escalate_sppointers((*cell).c2rust_unnamed.p as *mut BkBlock, f, order, depth);
        }
    }
    (*b)._depth = depth;
    *order = (*order).wrapping_add(1);
    (*(*f).entries.offset((*b)._index as isize)).order = *order;
}
unsafe extern "C" fn dfs_attract_cells(
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
        if bk_cell_is_pointer(cell) && !(*cell).c2rust_unnamed.p.is_null() {
            dfs_attract_cells(
                (*cell).c2rust_unnamed.p as *mut BkBlock,
                f,
                order,
                depth.wrapping_add(1),
            );
        }
    }
    *order = (*order).wrapping_add(1);
    (*(*f).entries.offset((*b)._index as isize)).order = *order;
    escalate_sppointers(b, f, order, depth);
    (*b)._visitstate = BkCellVisitState::Black;
}
unsafe extern "C" fn attract_bkgraph(f: *mut BkGraph) {
    for j in 0..(*f).length {
        let entry = (*f).entries.offset(j as isize);
        (*(*entry).block)._visitstate = BkCellVisitState::White;
        (*entry).order = 0;
        (*(*entry).block)._index = j;
        (*(*entry).block)._depth = 0;
    }
    let mut order: u32 = 0;
    dfs_attract_cells((*(*f).entries).block, f, &raw mut order, 0);
    qsort(
        (*f).entries as *mut ::core::ffi::c_void,
        (*f).length as usize,
        ::core::mem::size_of::<BkGraphNode>() as usize,
        Some(
            _by_order
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    for j in 0..(*f).length {
        (*(*(*f).entries.offset(j as isize)).block)._index = j;
    }
}
unsafe extern "C" fn try_untabgle_block(
    f: *mut BkGraph,
    b: *mut BkBlock,
    offsets: *mut usize,
    _passes: u16,
) -> bool {
    let mut did_copy: bool = false;
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            BkCellType::P16 | BkCellType::Sp16 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    let offset: i64 =
                        getoffset_untangle(offsets, b, (*cell).c2rust_unnamed.p as *mut BkBlock);
                    if !(0..=0xffff).contains(&offset) {
                        let e: *mut BkGraphNode = _bkgraph_grow(f);
                        (*e).order = 0;
                        (*e).alias = 0;
                        (*e).block = bk_new_block(&[bk_ptr(BkCellType::Copy, (*cell).c2rust_unnamed.p)]);
                        (*cell).t = BkCellType::Sp16;
                        (*cell).c2rust_unnamed.p = (*e).block as *mut BkBlock;
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
// before their own pass over the graph. `line` is forwarded to
// __caryll_allocate_clean only to keep its OOM message's [line] tag matching
// what each original call site reported.
unsafe fn compute_block_offsets(f: *mut BkGraph, line: ::core::ffi::c_ulong) -> *mut usize {
    let offsets: *mut usize = __caryll_allocate_clean(
        (::core::mem::size_of::<usize>() as usize).wrapping_mul((*f).length.wrapping_add(1) as usize),
        line,
    ) as *mut usize;
    *offsets = 0;
    for j in 0..(*f).length {
        let block = (*(*f).entries.offset(j as isize)).block;
        let running = *offsets.offset(j as isize);
        *offsets.offset(j as isize + 1) = if (*block)._visitstate == BkCellVisitState::Black {
            running.wrapping_add(otfcc_bkblock_size(block))
        } else {
            running
        };
    }
    offsets
}
unsafe extern "C" fn try_untangle(f: *mut BkGraph, passes: u16) -> bool {
    let offsets: *mut usize = compute_block_offsets(f, 294);
    let mut did_untangle: bool = false;
    for j in 0..(*f).length {
        let block = (*(*f).entries.offset(j as isize)).block;
        if (*block)._visitstate == BkCellVisitState::Black {
            did_untangle |= try_untabgle_block(f, block, offsets, passes);
        }
    }
    free(offsets as *mut ::core::ffi::c_void);
    return did_untangle;
}
unsafe extern "C" fn otfcc_build_bkblock(buf: *mut Buffer, b: *mut BkBlock, offsets: *mut usize) {
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            BkCellType::B8 => {
                bufwrite8(buf, (*cell).c2rust_unnamed.z as u8);
            }
            BkCellType::B16 => {
                bufwrite16b(buf, (*cell).c2rust_unnamed.z as u16);
            }
            BkCellType::B32 => {
                bufwrite32b(buf, (*cell).c2rust_unnamed.z);
            }
            BkCellType::P16 | BkCellType::Sp16 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    bufwrite16b(
                        buf,
                        getoffset(offsets, b, (*cell).c2rust_unnamed.p as *mut BkBlock, 16) as u16,
                    );
                } else {
                    bufwrite16b(buf, 0);
                }
            }
            BkCellType::P32 | BkCellType::Sp32 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    bufwrite32b(
                        buf,
                        getoffset(offsets, b, (*cell).c2rust_unnamed.p as *mut BkBlock, 32),
                    );
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
    let offsets: *mut usize = compute_block_offsets(f, 352);
    for j in 0..(*f).length {
        let block = (*(*f).entries.offset(j as isize)).block;
        if (*block)._visitstate == BkCellVisitState::Black {
            otfcc_build_bkblock(buf, block, offsets);
        }
    }
    free(offsets as *mut ::core::ffi::c_void);
    return buf;
}
pub unsafe extern "C" fn bk_estimate_size_of_graph(f: *mut BkGraph) -> usize {
    let offsets: *mut usize = compute_block_offsets(f, 373);
    let estimated_size: usize = *offsets.offset((*f).length as isize);
    free(offsets as *mut ::core::ffi::c_void);
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
