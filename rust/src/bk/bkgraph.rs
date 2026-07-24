use libc::{fprintf, free, qsort};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite8(buf: *mut caryll_Buffer, byte: u8);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_cellIsPointer(cell: *mut bk_Cell) -> bool;
}

pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_bkblock {
    pub _visitstate: bk_cell_visit_state,
    pub _index: u32,
    pub _height: u32,
    pub _depth: u32,
    pub length: u32,
    pub free: u32,
    pub cells: *mut bk_Cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bk_Cell {
    pub t: bk_CellType,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub z: u32,
    pub p: *mut __caryll_bkblock,
}
pub type bk_CellType = ::core::ffi::c_uint;
pub const bkembed: bk_CellType = 255;
pub const bkcopy: bk_CellType = 254;
pub const sp32: bk_CellType = 129;
pub const sp16: bk_CellType = 128;
pub const p32: bk_CellType = 17;
pub const p16: bk_CellType = 16;
pub const b32: bk_CellType = 3;
pub const b16: bk_CellType = 2;
pub const b8: bk_CellType = 1;
pub const bkover: bk_CellType = 0;
pub type bk_cell_visit_state = ::core::ffi::c_uint;
pub const VISIT_BLACK: bk_cell_visit_state = 2;
pub const VISIT_GRAY: bk_cell_visit_state = 1;
pub const VISIT_WHITE: bk_cell_visit_state = 0;
pub type bk_Block = __caryll_bkblock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bk_GraphNode {
    pub alias: u32,
    pub order: u32,
    pub height: u32,
    pub hash: u32,
    pub block: *mut bk_Block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bk_Graph {
    pub length: u32,
    pub free: u32,
    pub entries: *mut bk_GraphNode,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::buffer::{caryll_Buffer};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn _bkgraph_grow(mut f: *mut bk_Graph) -> *mut bk_GraphNode {
    if (*f).free != 0 {
        (*f).length = (*f).length.wrapping_add(1);
        (*f).free = (*f).free.wrapping_sub(1);
    } else {
        (*f).length = (*f).length.wrapping_add(1 as u32);
        (*f).free = (*f).length >> 1 as ::core::ffi::c_int & 0xffffff as u32;
        (*f).entries = __caryll_reallocate(
            (*f).entries as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<bk_GraphNode>() as usize)
                .wrapping_mul((*f).length.wrapping_add((*f).free) as usize),
            10 as ::core::ffi::c_ulong,
        ) as *mut bk_GraphNode;
    }
    return (*f)
        .entries
        .offset((*f).length.wrapping_sub(1 as u32) as isize) as *mut bk_GraphNode;
}
unsafe extern "C" fn dfs_insert_cells(
    b: *mut bk_Block,
    f: *mut bk_Graph,
    order: *mut u32,
) -> u32 {
    if b.is_null() || (*b)._visitstate == VISIT_GRAY {
        return 0;
    }
    if (*b)._visitstate == VISIT_BLACK {
        return (*b)._height;
    }
    (*b)._visitstate = VISIT_GRAY;
    let mut height: u32 = 0;
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        if bk_cellIsPointer(cell) && !(*cell).c2rust_unnamed.p.is_null() {
            let that_height = dfs_insert_cells((*cell).c2rust_unnamed.p as *mut bk_Block, f, order);
            if that_height.wrapping_add(1) > height {
                height = that_height.wrapping_add(1);
            }
        }
    }
    let e: *mut bk_GraphNode = _bkgraph_grow(f);
    (*e).alias = 0;
    (*e).block = b;
    *order = (*order).wrapping_add(1);
    (*e).order = *order;
    (*b)._height = height;
    (*e).height = (*b)._height;
    (*b)._visitstate = VISIT_BLACK;
    return height;
}
unsafe extern "C" fn _by_height(
    mut _a: *const ::core::ffi::c_void,
    mut _b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const bk_GraphNode = _a as *const bk_GraphNode;
    let mut b: *const bk_GraphNode = _b as *const bk_GraphNode;
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
    let mut a: *const bk_GraphNode = _a as *const bk_GraphNode;
    let mut b: *const bk_GraphNode = _b as *const bk_GraphNode;
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
#[no_mangle]
pub unsafe extern "C" fn bk_newGraphFromRootBlock(b: *mut bk_Block) -> *mut bk_Graph {
    let forest: *mut bk_Graph = __caryll_allocate_clean(
        ::core::mem::size_of::<bk_Graph>() as usize,
        55 as ::core::ffi::c_ulong,
    ) as *mut bk_Graph;
    let mut ts_order: u32 = 0;
    dfs_insert_cells(b, forest, &raw mut ts_order);
    qsort(
        (*forest).entries as *mut ::core::ffi::c_void,
        (*forest).length as usize,
        ::core::mem::size_of::<bk_GraphNode>() as usize,
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
#[no_mangle]
pub unsafe extern "C" fn bk_delete_Graph(f: *mut bk_Graph) {
    if f.is_null() || (*f).entries.is_null() {
        return;
    }
    for j in 0..(*f).length {
        let b: *mut bk_Block = (*(*f).entries.offset(j as isize)).block;
        if !b.is_null() && !(*b).cells.is_null() {
            free((*b).cells as *mut ::core::ffi::c_void);
            (*b).cells = ::core::ptr::null_mut::<bk_Cell>();
        }
        free(b as *mut ::core::ffi::c_void);
    }
    free((*f).entries as *mut ::core::ffi::c_void);
    (*f).entries = ::core::ptr::null_mut::<bk_GraphNode>();
    free(f as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn gethash(b: *mut bk_Block) -> u32 {
    let mut h: u32 = 5381;
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        h = (h << 5).wrapping_add(h).wrapping_add((*cell).t as u32);
        h = (h << 5).wrapping_add(h);
        match (*cell).t {
            b8 | b16 | b32 => {
                h = h.wrapping_add((*cell).c2rust_unnamed.z);
            }
            p16 | p32 | sp16 | sp32 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    h = h.wrapping_add((*(*cell).c2rust_unnamed.p)._index);
                }
            }
            _ => {}
        }
    }
    return h;
}
unsafe extern "C" fn compareblock(a: *mut bk_Block, b: *mut bk_Block) -> bool {
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
            b8 | b16 | b32 => {
                if (*ca).c2rust_unnamed.z != (*cb).c2rust_unnamed.z {
                    return false;
                }
            }
            p16 | p32 | sp16 | sp32 => {
                if (*ca).c2rust_unnamed.p != (*cb).c2rust_unnamed.p {
                    return false;
                }
            }
            _ => {}
        }
    }
    return true;
}
unsafe extern "C" fn compareEntry(a: *mut bk_GraphNode, b: *mut bk_GraphNode) -> bool {
    if (*a).hash != (*b).hash {
        return false;
    }
    return compareblock((*a).block, (*b).block);
}
unsafe extern "C" fn replaceptr(f: *mut bk_Graph, b: *mut bk_Block) {
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            p16 | p32 | sp16 | sp32 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    let mut index: u32 = (*(*cell).c2rust_unnamed.p)._index;
                    while (*(*f).entries.offset(index as isize)).alias != index {
                        index = (*(*f).entries.offset(index as isize)).alias;
                    }
                    (*cell).c2rust_unnamed.p =
                        (*(*f).entries.offset(index as isize)).block as *mut __caryll_bkblock;
                }
            }
            _ => {}
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bk_minimizeGraph(f: *mut bk_Graph) {
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
            let a: *mut bk_GraphNode = (*f).entries.offset(j as isize);
            if (*a).alias == j {
                for k in (j + 1)..=rear {
                    let b: *mut bk_GraphNode = (*f).entries.offset(k as isize);
                    if (*b).alias == k && compareEntry(a, b) {
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
unsafe extern "C" fn otfcc_bkblock_size(b: *mut bk_Block) -> usize {
    let mut size: usize = 0;
    for j in 0..(*b).length {
        match (*(*b).cells.offset(j as isize)).t {
            b8 => {
                size = size.wrapping_add(1);
            }
            b16 | p16 | sp16 => {
                size = size.wrapping_add(2);
            }
            b32 | p32 | sp32 => {
                size = size.wrapping_add(4);
            }
            _ => {}
        }
    }
    return size;
}
unsafe extern "C" fn getoffset(
    mut offsets: *mut usize,
    mut ref_0: *mut bk_Block,
    mut target: *mut bk_Block,
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
    mut ref_0: *mut bk_Block,
    mut target: *mut bk_Block,
) -> i64 {
    let mut offref: usize = *offsets.offset((*ref_0)._index as isize);
    let mut offtgt: usize = *offsets.offset((*target)._index as isize);
    return offtgt.wrapping_sub(offref) as i64;
}
unsafe extern "C" fn escalate_sppointers(
    b: *mut bk_Block,
    f: *mut bk_Graph,
    order: *mut u32,
    depth: u32,
) {
    if b.is_null() {
        return;
    }
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        if bk_cellIsPointer(cell) && !(*cell).c2rust_unnamed.p.is_null() && (*cell).t >= sp16 {
            escalate_sppointers((*cell).c2rust_unnamed.p as *mut bk_Block, f, order, depth);
        }
    }
    (*b)._depth = depth;
    *order = (*order).wrapping_add(1);
    (*(*f).entries.offset((*b)._index as isize)).order = *order;
}
unsafe extern "C" fn dfs_attract_cells(
    b: *mut bk_Block,
    f: *mut bk_Graph,
    order: *mut u32,
    depth: u32,
) {
    if b.is_null() {
        return;
    }
    if (*b)._visitstate != VISIT_WHITE {
        if (*b)._depth < depth {
            (*b)._depth = depth;
        }
        return;
    }
    (*b)._visitstate = VISIT_GRAY;
    // Visits cells in reverse index order (length-1 downto 0); equivalent to
    // c2rust's `j = length; loop { let fresh = j; j -= 1; if fresh == 0 {
    // break } ... use fresh-1 ... }` underflow-sentinel trick.
    for j in (0..(*b).length).rev() {
        let cell = (*b).cells.offset(j as isize);
        if bk_cellIsPointer(cell) && !(*cell).c2rust_unnamed.p.is_null() {
            dfs_attract_cells(
                (*cell).c2rust_unnamed.p as *mut bk_Block,
                f,
                order,
                depth.wrapping_add(1),
            );
        }
    }
    *order = (*order).wrapping_add(1);
    (*(*f).entries.offset((*b)._index as isize)).order = *order;
    escalate_sppointers(b, f, order, depth);
    (*b)._visitstate = VISIT_BLACK;
}
unsafe extern "C" fn attract_bkgraph(f: *mut bk_Graph) {
    for j in 0..(*f).length {
        let entry = (*f).entries.offset(j as isize);
        (*(*entry).block)._visitstate = VISIT_WHITE;
        (*entry).order = 0;
        (*(*entry).block)._index = j;
        (*(*entry).block)._depth = 0;
    }
    let mut order: u32 = 0;
    dfs_attract_cells((*(*f).entries).block, f, &raw mut order, 0);
    qsort(
        (*f).entries as *mut ::core::ffi::c_void,
        (*f).length as usize,
        ::core::mem::size_of::<bk_GraphNode>() as usize,
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
    f: *mut bk_Graph,
    b: *mut bk_Block,
    offsets: *mut usize,
    _passes: u16,
) -> bool {
    let mut did_copy: bool = false;
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            p16 | sp16 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    let offset: i64 =
                        getoffset_untangle(offsets, b, (*cell).c2rust_unnamed.p as *mut bk_Block);
                    if !(0..=0xffff).contains(&offset) {
                        let e: *mut bk_GraphNode = _bkgraph_grow(f);
                        (*e).order = 0;
                        (*e).alias = 0;
                        (*e).block = bk_new_Block(
                            bkcopy as ::core::ffi::c_int,
                            (*cell).c2rust_unnamed.p,
                            bkover as ::core::ffi::c_int,
                        );
                        (*cell).t = sp16;
                        (*cell).c2rust_unnamed.p = (*e).block as *mut __caryll_bkblock;
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
// 0 if bk_minimizeGraph already merged it away and it's no longer
// VISIT_BLACK) for every entry, i.e. the running byte offset each surviving
// block will land at once serialized in order. Shared by try_untangle,
// bk_build_Graph, and bk_estimateSizeOfGraph, which each need this table
// before their own pass over the graph. `line` is forwarded to
// __caryll_allocate_clean only to keep its OOM message's [line] tag matching
// what each original call site reported.
unsafe fn compute_block_offsets(f: *mut bk_Graph, line: ::core::ffi::c_ulong) -> *mut usize {
    let offsets: *mut usize = __caryll_allocate_clean(
        (::core::mem::size_of::<usize>() as usize).wrapping_mul((*f).length.wrapping_add(1) as usize),
        line,
    ) as *mut usize;
    *offsets = 0;
    for j in 0..(*f).length {
        let block = (*(*f).entries.offset(j as isize)).block;
        let running = *offsets.offset(j as isize);
        *offsets.offset(j as isize + 1) = if (*block)._visitstate == VISIT_BLACK {
            running.wrapping_add(otfcc_bkblock_size(block))
        } else {
            running
        };
    }
    offsets
}
unsafe extern "C" fn try_untangle(f: *mut bk_Graph, passes: u16) -> bool {
    let offsets: *mut usize = compute_block_offsets(f, 294);
    let mut did_untangle: bool = false;
    for j in 0..(*f).length {
        let block = (*(*f).entries.offset(j as isize)).block;
        if (*block)._visitstate == VISIT_BLACK {
            did_untangle |= try_untabgle_block(f, block, offsets, passes);
        }
    }
    free(offsets as *mut ::core::ffi::c_void);
    return did_untangle;
}
unsafe extern "C" fn otfcc_build_bkblock(buf: *mut caryll_Buffer, b: *mut bk_Block, offsets: *mut usize) {
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            b8 => {
                bufwrite8(buf, (*cell).c2rust_unnamed.z as u8);
            }
            b16 => {
                bufwrite16b(buf, (*cell).c2rust_unnamed.z as u16);
            }
            b32 => {
                bufwrite32b(buf, (*cell).c2rust_unnamed.z);
            }
            p16 | sp16 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    bufwrite16b(
                        buf,
                        getoffset(offsets, b, (*cell).c2rust_unnamed.p as *mut bk_Block, 16) as u16,
                    );
                } else {
                    bufwrite16b(buf, 0);
                }
            }
            p32 | sp32 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    bufwrite32b(
                        buf,
                        getoffset(offsets, b, (*cell).c2rust_unnamed.p as *mut bk_Block, 32),
                    );
                } else {
                    bufwrite32b(buf, 0);
                }
            }
            _ => {}
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bk_build_Graph(f: *mut bk_Graph) -> *mut caryll_Buffer {
    let buf: *mut caryll_Buffer = bufnew();
    let offsets: *mut usize = compute_block_offsets(f, 352);
    for j in 0..(*f).length {
        let block = (*(*f).entries.offset(j as isize)).block;
        if (*block)._visitstate == VISIT_BLACK {
            otfcc_build_bkblock(buf, block, offsets);
        }
    }
    free(offsets as *mut ::core::ffi::c_void);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn bk_estimateSizeOfGraph(f: *mut bk_Graph) -> usize {
    let offsets: *mut usize = compute_block_offsets(f, 373);
    let estimated_size: usize = *offsets.offset((*f).length as isize);
    free(offsets as *mut ::core::ffi::c_void);
    return estimated_size;
}
#[no_mangle]
pub unsafe extern "C" fn bk_untangleGraph(f: *mut bk_Graph) {
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
#[no_mangle]
pub unsafe extern "C" fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer {
    let f: *mut bk_Graph = bk_newGraphFromRootBlock(root);
    bk_minimizeGraph(f);
    bk_untangleGraph(f);
    let buf: *mut caryll_Buffer = bk_build_Graph(f);
    bk_delete_Graph(f);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn bk_build_Block_noMinimize(root: *mut bk_Block) -> *mut caryll_Buffer {
    let f: *mut bk_Graph = bk_newGraphFromRootBlock(root);
    bk_untangleGraph(f);
    let buf: *mut caryll_Buffer = bk_build_Graph(f);
    bk_delete_Graph(f);
    return buf;
}
