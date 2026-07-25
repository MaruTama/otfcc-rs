extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite8(buf: *mut caryll_Buffer, byte: uint8_t);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: uint32_t);
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_cellIsPointer(cell: *mut bk_Cell) -> bool;
}

pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_bkblock {
    pub _visitstate: bk_cell_visit_state,
    pub _index: uint32_t,
    pub _height: uint32_t,
    pub _depth: uint32_t,
    pub length: uint32_t,
    pub free: uint32_t,
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
    pub z: uint32_t,
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
    pub alias: uint32_t,
    pub order: uint32_t,
    pub height: uint32_t,
    pub hash: uint32_t,
    pub block: *mut bk_Block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bk_Graph {
    pub length: uint32_t,
    pub free: uint32_t,
    pub entries: *mut bk_GraphNode,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
use crate::support::stdio::{FILE, stderr};
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn _bkgraph_grow(mut f: *mut bk_Graph) -> *mut bk_GraphNode {
    if (*f).free != 0 {
        (*f).length = (*f).length.wrapping_add(1);
        (*f).free = (*f).free.wrapping_sub(1);
    } else {
        (*f).length = (*f).length.wrapping_add(1 as uint32_t);
        (*f).free = (*f).length >> 1 as ::core::ffi::c_int & 0xffffff as uint32_t;
        (*f).entries = __caryll_reallocate(
            (*f).entries as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<bk_GraphNode>() as size_t)
                .wrapping_mul((*f).length.wrapping_add((*f).free) as size_t),
            10 as ::core::ffi::c_ulong,
        ) as *mut bk_GraphNode;
    }
    return (*f)
        .entries
        .offset((*f).length.wrapping_sub(1 as uint32_t) as isize) as *mut bk_GraphNode;
}
unsafe extern "C" fn dfs_insert_cells(
    b: *mut bk_Block,
    f: *mut bk_Graph,
    order: *mut uint32_t,
) -> uint32_t {
    if b.is_null() || (*b)._visitstate == VISIT_GRAY {
        return 0;
    }
    if (*b)._visitstate == VISIT_BLACK {
        return (*b)._height;
    }
    (*b)._visitstate = VISIT_GRAY;
    let mut height: uint32_t = 0;
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
        ((*(*b).block)._visitstate as uint32_t).wrapping_sub((*(*a).block)._visitstate as uint32_t)
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
        ::core::mem::size_of::<bk_Graph>() as size_t,
        55 as ::core::ffi::c_ulong,
    ) as *mut bk_Graph;
    let mut ts_order: uint32_t = 0;
    dfs_insert_cells(b, forest, &raw mut ts_order);
    qsort(
        (*forest).entries as *mut ::core::ffi::c_void,
        (*forest).length as size_t,
        ::core::mem::size_of::<bk_GraphNode>() as size_t,
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
unsafe extern "C" fn gethash(b: *mut bk_Block) -> uint32_t {
    let mut h: uint32_t = 5381;
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        h = (h << 5).wrapping_add(h).wrapping_add((*cell).t as uint32_t);
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
                    let mut index: uint32_t = (*(*cell).c2rust_unnamed.p)._index;
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
    let mut rear: uint32_t = (*f).length.wrapping_sub(1);
    while rear > 0 {
        // front/rear bracket a run of same-height entries; the run's extent
        // is data-dependent, so this scan must stay a while loop. Everything
        // below it operates over the now-fixed [front, rear] (or [0, front))
        // range and is a plain for loop.
        let mut front: uint32_t = rear;
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
unsafe extern "C" fn otfcc_bkblock_size(b: *mut bk_Block) -> size_t {
    let mut size: size_t = 0;
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
    mut offsets: *mut size_t,
    mut ref_0: *mut bk_Block,
    mut target: *mut bk_Block,
    mut bits: uint8_t,
) -> uint32_t {
    let mut offref: size_t = *offsets.offset((*ref_0)._index as isize);
    let mut offtgt: size_t = *offsets.offset((*target)._index as isize);
    if (bits as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        && (offtgt < offref || offtgt.wrapping_sub(offref) >> bits as ::core::ffi::c_int != 0)
    {
        fprintf(
            stderr,
            b"[otfcc-bk] Warning : Unable to fit offset %d into %d bits; output may be corrupted.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            offtgt.wrapping_sub(offref) as int32_t,
            bits as ::core::ffi::c_int,
        );
    }
    return offtgt.wrapping_sub(offref) as uint32_t;
}
unsafe extern "C" fn getoffset_untangle(
    mut offsets: *mut size_t,
    mut ref_0: *mut bk_Block,
    mut target: *mut bk_Block,
) -> int64_t {
    let mut offref: size_t = *offsets.offset((*ref_0)._index as isize);
    let mut offtgt: size_t = *offsets.offset((*target)._index as isize);
    return offtgt.wrapping_sub(offref) as int64_t;
}
unsafe extern "C" fn escalate_sppointers(
    b: *mut bk_Block,
    f: *mut bk_Graph,
    order: *mut uint32_t,
    depth: uint32_t,
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
    order: *mut uint32_t,
    depth: uint32_t,
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
    let mut order: uint32_t = 0;
    dfs_attract_cells((*(*f).entries).block, f, &raw mut order, 0);
    qsort(
        (*f).entries as *mut ::core::ffi::c_void,
        (*f).length as size_t,
        ::core::mem::size_of::<bk_GraphNode>() as size_t,
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
    offsets: *mut size_t,
    _passes: uint16_t,
) -> bool {
    let mut did_copy: bool = false;
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            p16 | sp16 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    let offset: int64_t =
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
unsafe fn compute_block_offsets(f: *mut bk_Graph, line: ::core::ffi::c_ulong) -> *mut size_t {
    let offsets: *mut size_t = __caryll_allocate_clean(
        (::core::mem::size_of::<size_t>() as size_t).wrapping_mul((*f).length.wrapping_add(1) as size_t),
        line,
    ) as *mut size_t;
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
unsafe extern "C" fn try_untangle(f: *mut bk_Graph, passes: uint16_t) -> bool {
    let offsets: *mut size_t = compute_block_offsets(f, 294);
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
unsafe extern "C" fn otfcc_build_bkblock(buf: *mut caryll_Buffer, b: *mut bk_Block, offsets: *mut size_t) {
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        match (*cell).t {
            b8 => {
                bufwrite8(buf, (*cell).c2rust_unnamed.z as uint8_t);
            }
            b16 => {
                bufwrite16b(buf, (*cell).c2rust_unnamed.z as uint16_t);
            }
            b32 => {
                bufwrite32b(buf, (*cell).c2rust_unnamed.z);
            }
            p16 | sp16 => {
                if !(*cell).c2rust_unnamed.p.is_null() {
                    bufwrite16b(
                        buf,
                        getoffset(offsets, b, (*cell).c2rust_unnamed.p as *mut bk_Block, 16) as uint16_t,
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
    let offsets: *mut size_t = compute_block_offsets(f, 352);
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
pub unsafe extern "C" fn bk_estimateSizeOfGraph(f: *mut bk_Graph) -> size_t {
    let offsets: *mut size_t = compute_block_offsets(f, 373);
    let estimated_size: size_t = *offsets.offset((*f).length as isize);
    free(offsets as *mut ::core::ffi::c_void);
    return estimated_size;
}
#[no_mangle]
pub unsafe extern "C" fn bk_untangleGraph(f: *mut bk_Graph) {
    let mut passes: uint16_t = 0;
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
