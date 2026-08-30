#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::fprintf;

// `BkBlock` used to be allocated via `__caryll_allocate_clean`/
// `__caryll_reallocate`/raw `free`, with `cells` a hand-managed `*mut BkCell`
// array (`length`=used count, `free`=slack, grown by `bkblock_acells`/
// `bkblock_grow`). Stage 7-2-f converts `cells` to `Vec<BkCell>` (which
// tracks its own length/capacity, the same simplification `BkGraph.entries`
// already got) and `BkBlock` itself to `Box::into_raw`/`Box::from_raw`,
// matching every other `_create()`-shaped malloc shell this migration has
// removed.
//
// `BkCellValue::Ptr(*mut BkBlock)` cross-references between blocks stay raw
// pointers rather than becoming arena indices: a focused survey (see
// rust/README.md) confirmed every `BkBlock` is single-parent-owned (a forest
// of independently-built trees, never shared across two graphs, never
// cyclic -- real call sites always finish building a child completely, via
// `bk_new_block`/`bk_push`, before splicing it into a parent), and teardown
// is centralized to exactly two places (`bkgraph.rs`'s `bk_delete_graph`,
// a flat walk over an already-fully-built tree, and `bkpushitems`'s `Embed`
// arm below, which frees a block immediately after copying its cells and
// before any other cell can reference it). That ownership discipline is
// what makes a bare pointer sound here, the same reasoning
// `CffTable.fd_array: Vec<Box<CffTable>>` relies on for its own child
// pointers -- an index scheme (`libcff/subr.rs`'s arena-with-tombstones
// template) is only needed where slots get deleted and revisited
// mid-algorithm, which never happens to a `BkBlock`.
pub struct BkBlock {
    pub _visitstate: BkCellVisitState,
    pub _index: u32,
    pub _height: u32,
    pub _depth: u32,
    pub cells: Vec<BkCell>,
}
// Was a C-shaped `struct { t: BkCellType, c2rust_unnamed: union { z: u32,
// p: *mut BkBlock } } }`. Unlike the crate's other tag+union conversions,
// `t`'s ten values don't map 1:1 onto the union's two arms -- `B8`/`B16`/
// `B32` share `.z`, `P16`/`P32`/`Sp16`/`Sp32`/`Copy`/`Embed` share `.p`, and
// `Over` uses neither (see `bkpushitems`/`otfcc_build_bkblock`'s catch-all
// `_ => {}` arms) -- so `t` stays a separate field carrying the width/kind
// distinctions the two-variant `BkCellValue` enum below can't express on
// its own; `bk_cell_is_pointer`'s `t >= BkCellType::P16` still decides
// which variant a given `t` implies. Every field here is `Copy` (`p` is a
// pointer *value*, not owned data -- see `BkBlock`'s own comment above for
// what makes that sound), so the new enum stays `Copy` too, independent of
// `BkBlock` itself no longer being `Copy` once `cells` became a `Vec`.
#[derive(Copy, Clone)]
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
use crate::support::buffer::Buffer;
use crate::support::stdio::stderr;

pub fn bk_cell_is_pointer(cell: &BkCell) -> bool {
    cell.t >= BkCellType::P16
}
pub unsafe fn _bkblock_init() -> *mut BkBlock {
    Box::into_raw(Box::new(BkBlock {
        _visitstate: BkCellVisitState::White,
        _index: 0,
        _height: 0,
        _depth: 0,
        cells: Vec::new(),
    }))
}
pub unsafe fn bkblock_pushint(b: *mut BkBlock, type_0: BkCellType, x: u32) {
    (*b).cells.push(BkCell {
        t: type_0,
        value: BkCellValue::Int(x),
    });
}
pub unsafe fn bkblock_pushptr(b: *mut BkBlock, type_0: BkCellType, p: *mut BkBlock) {
    (*b).cells.push(BkCell {
        t: type_0,
        value: BkCellValue::Ptr(p),
    });
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
    BkCell {
        t,
        value: BkCellValue::Int(z),
    }
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
                // Cloned rather than borrowed: `curtype == Embed` frees `par`
                // (and, in principle, `par` could alias `b` -- never true in
                // practice per the ownership note on `BkBlock` above, but a
                // clone up front means this loop is sound even if it were).
                if !par.is_null() {
                    for cell in (*par).cells.clone() {
                        if bk_cell_is_pointer(&cell) {
                            bkblock_pushptr(b, cell.t, cell.as_ptr());
                        } else {
                            bkblock_pushint(b, cell.t, cell.as_int());
                        }
                    }
                }
                if curtype == BkCellType::Embed && !par.is_null() {
                    drop(Box::from_raw(par));
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
pub unsafe fn bk_new_block_from_string_len(
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
pub unsafe fn bk_new_block_from_buffer(buf: Option<Buffer>) -> *mut BkBlock {
    let Some(buf) = buf else {
        return ::core::ptr::null_mut::<BkBlock>();
    };
    let b: *mut BkBlock = bk_new_block(&[]);
    for &byte in buf.data.iter() {
        bkblock_pushint(b, BkCellType::B8, byte as u32);
    }
    return b;
}
pub unsafe fn bk_new_block_from_buffer_copy(buf: Option<&Buffer>) -> *mut BkBlock {
    let Some(buf) = buf else {
        return ::core::ptr::null_mut::<BkBlock>();
    };
    let b: *mut BkBlock = bk_new_block(&[]);
    for &byte in buf.data.iter() {
        bkblock_pushint(b, BkCellType::B8, byte as u32);
    }
    return b;
}
pub unsafe fn bk_print_block(b: *mut BkBlock) {
    fprintf(
        stderr,
        b"Block size %08x\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*b).cells.len() as u32,
    );
    fprintf(
        stderr,
        b"------------------\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    for cell in (*b).cells.iter() {
        if bk_cell_is_pointer(cell) {
            let p = cell.as_ptr();
            if !p.is_null() {
                fprintf(
                    stderr,
                    b"  %3d %p[%d]\n\0" as *const u8 as *const ::core::ffi::c_char,
                    cell.t as ::core::ffi::c_uint,
                    p,
                    (*p)._index,
                );
            } else {
                fprintf(
                    stderr,
                    b"  %3d [NULL]\n\0" as *const u8 as *const ::core::ffi::c_char,
                    cell.t as ::core::ffi::c_uint,
                );
            }
        } else {
            fprintf(
                stderr,
                b"  %3d %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                cell.t as ::core::ffi::c_uint,
                cell.as_int(),
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
        let all = [
            BkCellType::Over,
            BkCellType::B8,
            BkCellType::B16,
            BkCellType::B32,
            BkCellType::P16,
            BkCellType::P32,
            BkCellType::Sp16,
            BkCellType::Sp32,
            BkCellType::Copy,
            BkCellType::Embed,
        ];
        for w in all.windows(2) {
            assert!(w[0] < w[1], "{:?} should sort before {:?}", w[0], w[1]);
            assert!((w[0] as u32) < (w[1] as u32));
        }
        assert_eq!(
            [
                BkCellType::Over as u32,
                BkCellType::B8 as u32,
                BkCellType::B16 as u32,
                BkCellType::B32 as u32
            ],
            [0, 1, 2, 3]
        );
        assert_eq!(
            [
                BkCellType::P16 as u32,
                BkCellType::P32 as u32,
                BkCellType::Sp16 as u32,
                BkCellType::Sp32 as u32
            ],
            [16, 17, 128, 129]
        );
        assert_eq!(
            [BkCellType::Copy as u32, BkCellType::Embed as u32],
            [254, 255]
        );
        assert_eq!(::core::mem::size_of::<BkCellType>(), 4);
    }
}
