#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{fprintf, free};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BkBlock {
    pub _visitstate: BkCellVisitState,
    pub _index: u32,
    pub _height: u32,
    pub _depth: u32,
    pub length: u32,
    pub free: u32,
    pub cells: *mut BkCell,
}
// Was a C-shaped `struct { t: BkCellType, c2rust_unnamed: union { z: u32,
// p: *mut BkBlock } } }`. Unlike the crate's other tag+union conversions,
// `t`'s ten values don't map 1:1 onto the union's two arms -- `B8`/`B16`/
// `B32` share `.z`, `P16`/`P32`/`Sp16`/`Sp32`/`Copy`/`Embed` share `.p`, and
// `Over` uses neither (see `bkpushitems`/`otfcc_build_bkblock`'s catch-all
// `_ => {}` arms) -- so `t` stays a separate field carrying the width/kind
// distinctions the two-variant `BkCellValue` enum below can't express on
// its own; `bk_cell_is_pointer`'s `t >= BkCellType::P16` still decides
// which variant a given `t` implies. Every field here is `Copy` (no owned
// heap data -- `p` is a borrowed pointer into the same `BkGraph` that owns
// it), so the new enum stays `Copy` too.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BkCell {
    pub t: BkCellType,
    pub value: BkCellValue,
}
#[derive(Copy, Clone)]
pub enum BkCellValue {
    Int(u32),
    Ptr(*mut BkBlock),
}
impl BkCell {
    /// Panics instead of reading union garbage if `t` didn't actually
    /// imply a pointer cell -- every call site already established this via
    /// `bk_cell_is_pointer` or a `t`-keyed match arm before reaching here.
    pub fn as_ptr(&self) -> *mut BkBlock {
        match self.value {
            BkCellValue::Ptr(p) => p,
            BkCellValue::Int(_) => panic!("BkCell::as_ptr called on an integer cell"),
        }
    }
    pub fn as_int(&self) -> u32 {
        match self.value {
            BkCellValue::Int(z) => z,
            BkCellValue::Ptr(_) => panic!("BkCell::as_int called on a pointer cell"),
        }
    }
}
/// What a [`BkCell`] holds, and -- because the values are ordered, not just
/// distinct -- how wide it is and whether it is a pointer.
///
/// C classifies cells by comparing the raw number: `bk_cell_is_pointer` is
/// `cell->t >= BkCellType::P16`, `bkpushitems` takes the integer path for `t < BkCellType::P16`, and
/// `escalate_sppointers` walks the pointers with `t >= BkCellType::Sp16`. Those comparisons
/// survive here as `Ord`, which compares by *declaration* order -- so the
/// variants are declared in ascending discriminant order and
/// `bk_celltype_order_is_its_encoding` pins that they agree.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u32)]
pub enum BkCellType {
    Over = 0,
    B8 = 1,
    B16 = 2,
    B32 = 3,
    P16 = 16,
    P32 = 17,
    Sp16 = 128,
    Sp32 = 129,
    Copy = 254,
    Embed = 255,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BkCellVisitState {
    White = 0,
    Gray = 1,
    Black = 2,
}
use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::buffer::{Buffer};
use crate::support::buffer::{buffree};

unsafe extern "C" fn bkblock_acells(mut b: *mut BkBlock, mut len: u32) {
    if len <= (*b).length.wrapping_add((*b).free) {
        (*b).free = (*b).free.wrapping_sub(len.wrapping_sub((*b).length));
        (*b).length = len;
    } else {
        (*b).length = len;
        (*b).free = len >> 1 as ::core::ffi::c_int & 0xffffff as u32;
        (*b).cells = __caryll_reallocate(
            (*b).cells as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<BkCell>() as usize)
                .wrapping_mul((*b).length.wrapping_add((*b).free) as usize),
            12 as ::core::ffi::c_ulong,
        ) as *mut BkCell;
    };
}
pub unsafe extern "C" fn bk_cell_is_pointer(mut cell: *mut BkCell) -> bool {
    return (*cell).t >= BkCellType::P16;
}
unsafe extern "C" fn bkblock_grow(mut b: *mut BkBlock, mut len: u32) -> *mut BkCell {
    let mut olen: u32 = (*b).length;
    bkblock_acells(b, olen.wrapping_add(len));
    return (*b).cells.offset(olen as isize) as *mut BkCell;
}
pub unsafe extern "C" fn _bkblock_init() -> *mut BkBlock {
    let mut b: *mut BkBlock = ::core::ptr::null_mut::<BkBlock>();
    b = __caryll_allocate_clean(
        ::core::mem::size_of::<BkBlock>() as usize,
        27 as ::core::ffi::c_ulong,
    ) as *mut BkBlock;
    bkblock_acells(b, 0 as u32);
    return b;
}
pub unsafe extern "C" fn bkblock_pushint(
    mut b: *mut BkBlock,
    mut type_0: BkCellType,
    mut x: u32,
) {
    let mut cell: *mut BkCell = bkblock_grow(b, 1 as u32);
    (*cell).t = type_0;
    (*cell).value = BkCellValue::Int(x);
}
pub unsafe extern "C" fn bkblock_pushptr(
    mut b: *mut BkBlock,
    mut type_0: BkCellType,
    mut p: *mut BkBlock,
) {
    let mut cell: *mut BkCell = bkblock_grow(b, 1 as u32);
    (*cell).t = type_0;
    (*cell).value = BkCellValue::Ptr(p);
}
/// One (type, value) pair for [`bk_push`] / [`bk_new_block`].
///
/// C passed these as varargs -- `bk_push(b, BkCellType::B16, count, BkCellType::P16, child, BkCellType::Over)` --
/// with a sentinel to say where the list ended and the caller responsible for
/// keeping each type next to a value of the matching kind. A `BkCell` already
/// *is* a type plus either an integer or a block pointer, so the list is just a
/// slice of them, and the sentinel is gone along with the `c_variadic` feature.
///
/// Build them with [`bk_int`] and [`bk_ptr`] rather than by hand: which arm of
/// the union is live is decided by `t`, exactly as the old vararg reader decided
/// whether to pull a `c_int` or a pointer off the list.

/// A cell holding an integer. `t` must be `BkCellType::B8`, `BkCellType::B16` or `BkCellType::B32`.
#[inline]
pub fn bk_int(t: BkCellType, z: u32) -> BkCell {
    BkCell { t, value: BkCellValue::Int(z) }
}

/// A cell holding a block pointer -- `BkCellType::P16`/`BkCellType::P32`/`BkCellType::Sp16`/`BkCellType::Sp32` for an offset, or
/// `BkCellType::Copy`/`BkCellType::Embed` to splice the target's cells in.
#[inline]
pub fn bk_ptr(t: BkCellType, p: *mut BkBlock) -> BkCell {
    BkCell {
        t,
        value: BkCellValue::Ptr(p),
    }
}

unsafe fn bkpushitems(b: *mut BkBlock, items: &[BkCell]) {
    for item in items {
        let curtype = item.t;
        match curtype {
            BkCellType::Copy | BkCellType::Embed => {
                let par: *mut BkBlock = item.as_ptr();
                if !par.is_null() && !(*par).cells.is_null() {
                    for j in 0..(*par).length {
                        let cell = (*par).cells.offset(j as isize);
                        if bk_cell_is_pointer(cell) {
                            bkblock_pushptr(b, (*cell).t, (*cell).as_ptr());
                        } else {
                            bkblock_pushint(b, (*cell).t, (*cell).as_int());
                        }
                    }
                }
                if curtype == BkCellType::Embed && !par.is_null() {
                    free((*par).cells as *mut ::core::ffi::c_void);
                    (*par).cells = ::core::ptr::null_mut::<BkCell>();
                    free(par as *mut ::core::ffi::c_void);
                }
            }
            t if t < BkCellType::P16 => bkblock_pushint(b, curtype, item.as_int()),
            _ => bkblock_pushptr(b, curtype, item.as_ptr()),
        }
    }
}

/// A fresh block holding `items`.
pub unsafe fn bk_new_block(items: &[BkCell]) -> *mut BkBlock {
    let b: *mut BkBlock = _bkblock_init();
    bkpushitems(b, items);
    return b;
}

/// Append `items` to `b`, and hand `b` back so calls can be chained.
pub unsafe fn bk_push(b: *mut BkBlock, items: &[BkCell]) -> *mut BkBlock {
    bkpushitems(b, items);
    return b;
}
pub unsafe extern "C" fn bk_new_block_from_string_len(
    len: usize,
    str: *const ::core::ffi::c_char,
) -> *mut BkBlock {
    if str.is_null() {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let b: *mut BkBlock = bk_new_block(&[]);
    for j in 0..len {
        bkblock_pushint(b, BkCellType::B8, *str.offset(j as isize) as u32);
    }
    return b;
}
pub unsafe extern "C" fn bk_new_block_from_buffer(buf: *mut Buffer) -> *mut BkBlock {
    if buf.is_null() {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let b: *mut BkBlock = bk_new_block(&[]);
    for j in 0..(*buf).size {
        bkblock_pushint(b, BkCellType::B8, *(*buf).data.offset(j as isize) as u32);
    }
    buffree(buf);
    return b;
}
pub unsafe extern "C" fn bk_new_block_from_buffer_copy(buf: *const Buffer) -> *mut BkBlock {
    if buf.is_null() {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let b: *mut BkBlock = bk_new_block(&[]);
    for j in 0..(*buf).size {
        bkblock_pushint(b, BkCellType::B8, *(*buf).data.offset(j as isize) as u32);
    }
    return b;
}
pub unsafe extern "C" fn bk_print_block(b: *mut BkBlock) {
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
        if bk_cell_is_pointer(cell) {
            let p = (*cell).as_ptr();
            if !p.is_null() {
                fprintf(
                    stderr,
                    b"  %3d %p[%d]\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*cell).t as ::core::ffi::c_uint,
                    p,
                    (*p)._index,
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
                (*cell).as_int(),
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

    // `BkCellType`'s numbers are load-bearing twice over. `bkpushitems` sends
    // `t < BkCellType::P16` down the integer path and everything else down the pointer path
    // -- reading the wrong arm of a union if that split moved -- and
    // `escalate_sppointers` in bkgraph.rs picks the shared pointers with
    // `t >= BkCellType::Sp16`, which decides layout order and therefore the offsets written
    // into the font. Both are `Ord` on the enum now, and `Ord` follows
    // declaration order rather than the discriminants, so this pins that the two
    // orders are the same one.
    #[test]
    fn bk_celltype_order_is_its_encoding() {
        let all = [BkCellType::Over, BkCellType::B8, BkCellType::B16, BkCellType::B32, BkCellType::P16, BkCellType::P32, BkCellType::Sp16, BkCellType::Sp32, BkCellType::Copy, BkCellType::Embed];
        for w in all.windows(2) {
            assert!(w[0] < w[1], "{:?} should sort before {:?}", w[0], w[1]);
            assert!((w[0] as u32) < (w[1] as u32));
        }
        assert_eq!([BkCellType::Over as u32, BkCellType::B8 as u32, BkCellType::B16 as u32, BkCellType::B32 as u32], [0, 1, 2, 3]);
        assert_eq!([BkCellType::P16 as u32, BkCellType::P32 as u32, BkCellType::Sp16 as u32, BkCellType::Sp32 as u32], [16, 17, 128, 129]);
        assert_eq!([BkCellType::Copy as u32, BkCellType::Embed as u32], [254, 255]);
        assert_eq!(::core::mem::size_of::<BkCellType>(), 4);
    }
}
