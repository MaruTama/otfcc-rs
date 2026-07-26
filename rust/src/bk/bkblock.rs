#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{fprintf, free};

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
    pub c2rust_unnamed: bk_CellValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union bk_CellValue {
    pub z: u32,
    pub p: *mut __caryll_bkblock,
}
/// What a [`bk_Cell`] holds, and -- because the values are ordered, not just
/// distinct -- how wide it is and whether it is a pointer.
///
/// C classifies cells by comparing the raw number: `bk_cellIsPointer` is
/// `cell->t >= p16`, `bkpushitems` takes the integer path for `t < p16`, and
/// `escalate_sppointers` walks the pointers with `t >= sp16`. Those comparisons
/// survive here as `Ord`, which compares by *declaration* order -- so the
/// variants are declared in ascending discriminant order and
/// `bk_celltype_order_is_its_encoding` pins that they agree.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u32)]
pub enum bk_CellType {
    bkover = 0,
    b8 = 1,
    b16 = 2,
    b32 = 3,
    p16 = 16,
    p32 = 17,
    sp16 = 128,
    sp32 = 129,
    bkcopy = 254,
    bkembed = 255,
}
pub use bk_CellType::*;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum bk_cell_visit_state {
    VISIT_WHITE = 0,
    VISIT_GRAY = 1,
    VISIT_BLACK = 2,
}
pub use bk_cell_visit_state::*;
pub type bk_Block = __caryll_bkblock;
use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::buffer::{caryll_Buffer};
use crate::support::buffer::{buffree};

unsafe extern "C" fn bkblock_acells(mut b: *mut bk_Block, mut len: u32) {
    if len <= (*b).length.wrapping_add((*b).free) {
        (*b).free = (*b).free.wrapping_sub(len.wrapping_sub((*b).length));
        (*b).length = len;
    } else {
        (*b).length = len;
        (*b).free = len >> 1 as ::core::ffi::c_int & 0xffffff as u32;
        (*b).cells = __caryll_reallocate(
            (*b).cells as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<bk_Cell>() as usize)
                .wrapping_mul((*b).length.wrapping_add((*b).free) as usize),
            12 as ::core::ffi::c_ulong,
        ) as *mut bk_Cell;
    };
}
pub unsafe extern "C" fn bk_cellIsPointer(mut cell: *mut bk_Cell) -> bool {
    return (*cell).t >= p16;
}
unsafe extern "C" fn bkblock_grow(mut b: *mut bk_Block, mut len: u32) -> *mut bk_Cell {
    let mut olen: u32 = (*b).length;
    bkblock_acells(b, olen.wrapping_add(len));
    return (*b).cells.offset(olen as isize) as *mut bk_Cell;
}
pub unsafe extern "C" fn _bkblock_init() -> *mut bk_Block {
    let mut b: *mut bk_Block = ::core::ptr::null_mut::<bk_Block>();
    b = __caryll_allocate_clean(
        ::core::mem::size_of::<bk_Block>() as usize,
        27 as ::core::ffi::c_ulong,
    ) as *mut bk_Block;
    bkblock_acells(b, 0 as u32);
    return b;
}
pub unsafe extern "C" fn bkblock_pushint(
    mut b: *mut bk_Block,
    mut type_0: bk_CellType,
    mut x: u32,
) {
    let mut cell: *mut bk_Cell = bkblock_grow(b, 1 as u32);
    (*cell).t = type_0;
    (*cell).c2rust_unnamed.z = x;
}
pub unsafe extern "C" fn bkblock_pushptr(
    mut b: *mut bk_Block,
    mut type_0: bk_CellType,
    mut p: *mut bk_Block,
) {
    let mut cell: *mut bk_Cell = bkblock_grow(b, 1 as u32);
    (*cell).t = type_0;
    (*cell).c2rust_unnamed.p = p as *mut __caryll_bkblock;
}
/// One (type, value) pair for [`bk_push`] / [`bk_new_Block`].
///
/// C passed these as varargs -- `bk_push(b, b16, count, p16, child, bkover)` --
/// with a sentinel to say where the list ended and the caller responsible for
/// keeping each type next to a value of the matching kind. A `bk_Cell` already
/// *is* a type plus either an integer or a block pointer, so the list is just a
/// slice of them, and the sentinel is gone along with the `c_variadic` feature.
///
/// Build them with [`bk_int`] and [`bk_ptr`] rather than by hand: which arm of
/// the union is live is decided by `t`, exactly as the old vararg reader decided
/// whether to pull a `c_int` or a pointer off the list.
pub type bk_Item = bk_Cell;

/// A cell holding an integer. `t` must be `b8`, `b16` or `b32`.
#[inline]
pub fn bk_int(t: bk_CellType, z: u32) -> bk_Item {
    bk_Cell { t, c2rust_unnamed: bk_CellValue { z } }
}

/// A cell holding a block pointer -- `p16`/`p32`/`sp16`/`sp32` for an offset, or
/// `bkcopy`/`bkembed` to splice the target's cells in.
#[inline]
pub fn bk_ptr(t: bk_CellType, p: *mut bk_Block) -> bk_Item {
    bk_Cell {
        t,
        c2rust_unnamed: bk_CellValue { p: p as *mut __caryll_bkblock },
    }
}

unsafe fn bkpushitems(b: *mut bk_Block, items: &[bk_Item]) {
    for item in items {
        let curtype = item.t;
        match curtype {
            bkcopy | bkembed => {
                let par: *mut bk_Block = item.c2rust_unnamed.p as *mut bk_Block;
                if !par.is_null() && !(*par).cells.is_null() {
                    for j in 0..(*par).length {
                        let cell = (*par).cells.offset(j as isize);
                        if bk_cellIsPointer(cell) {
                            bkblock_pushptr(b, (*cell).t, (*cell).c2rust_unnamed.p as *mut bk_Block);
                        } else {
                            bkblock_pushint(b, (*cell).t, (*cell).c2rust_unnamed.z);
                        }
                    }
                }
                if curtype == bkembed && !par.is_null() {
                    free((*par).cells as *mut ::core::ffi::c_void);
                    (*par).cells = ::core::ptr::null_mut::<bk_Cell>();
                    free(par as *mut ::core::ffi::c_void);
                }
            }
            t if t < p16 => bkblock_pushint(b, curtype, item.c2rust_unnamed.z),
            _ => bkblock_pushptr(b, curtype, item.c2rust_unnamed.p as *mut bk_Block),
        }
    }
}

/// A fresh block holding `items`.
pub unsafe fn bk_new_Block(items: &[bk_Item]) -> *mut bk_Block {
    let b: *mut bk_Block = _bkblock_init();
    bkpushitems(b, items);
    return b;
}

/// Append `items` to `b`, and hand `b` back so calls can be chained.
pub unsafe fn bk_push(b: *mut bk_Block, items: &[bk_Item]) -> *mut bk_Block {
    bkpushitems(b, items);
    return b;
}
pub unsafe extern "C" fn bk_newBlockFromStringLen(
    len: usize,
    str: *const ::core::ffi::c_char,
) -> *mut bk_Block {
    if str.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let b: *mut bk_Block = bk_new_Block(&[]);
    for j in 0..len {
        bkblock_pushint(b, b8, *str.offset(j as isize) as u32);
    }
    return b;
}
pub unsafe extern "C" fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block {
    if buf.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let b: *mut bk_Block = bk_new_Block(&[]);
    for j in 0..(*buf).size {
        bkblock_pushint(b, b8, *(*buf).data.offset(j as isize) as u32);
    }
    buffree(buf);
    return b;
}
pub unsafe extern "C" fn bk_newBlockFromBufferCopy(buf: *const caryll_Buffer) -> *mut bk_Block {
    if buf.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let b: *mut bk_Block = bk_new_Block(&[]);
    for j in 0..(*buf).size {
        bkblock_pushint(b, b8, *(*buf).data.offset(j as isize) as u32);
    }
    return b;
}
pub unsafe extern "C" fn bk_printBlock(b: *mut bk_Block) {
    fprintf(
        stderr,
        b"Block size %08x\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*b).length,
    );
    fprintf(
        stderr,
        b"------------------\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    for j in 0..(*b).length {
        let cell = (*b).cells.offset(j as isize);
        if bk_cellIsPointer(cell) {
            if !(*cell).c2rust_unnamed.p.is_null() {
                fprintf(
                    stderr,
                    b"  %3d %p[%d]\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*cell).t as ::core::ffi::c_uint,
                    (*cell).c2rust_unnamed.p,
                    (*(*cell).c2rust_unnamed.p)._index,
                );
            } else {
                fprintf(
                    stderr,
                    b"  %3d [NULL]\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*cell).t as ::core::ffi::c_uint,
                );
            }
        } else {
            fprintf(
                stderr,
                b"  %3d %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*cell).t as ::core::ffi::c_uint,
                (*cell).c2rust_unnamed.z,
            );
        }
    }
    fprintf(
        stderr,
        b"------------------\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // `bk_CellType`'s numbers are load-bearing twice over. `bkpushitems` sends
    // `t < p16` down the integer path and everything else down the pointer path
    // -- reading the wrong arm of a union if that split moved -- and
    // `escalate_sppointers` in bkgraph.rs picks the shared pointers with
    // `t >= sp16`, which decides layout order and therefore the offsets written
    // into the font. Both are `Ord` on the enum now, and `Ord` follows
    // declaration order rather than the discriminants, so this pins that the two
    // orders are the same one.
    #[test]
    fn bk_celltype_order_is_its_encoding() {
        let all = [bkover, b8, b16, b32, p16, p32, sp16, sp32, bkcopy, bkembed];
        for w in all.windows(2) {
            assert!(w[0] < w[1], "{:?} should sort before {:?}", w[0], w[1]);
            assert!((w[0] as u32) < (w[1] as u32));
        }
        assert_eq!([bkover as u32, b8 as u32, b16 as u32, b32 as u32], [0, 1, 2, 3]);
        assert_eq!([p16 as u32, p32 as u32, sp16 as u32, sp32 as u32], [16, 17, 128, 129]);
        assert_eq!([bkcopy as u32, bkembed as u32], [254, 255]);
        assert_eq!(::core::mem::size_of::<bk_CellType>(), 4);
    }
}
