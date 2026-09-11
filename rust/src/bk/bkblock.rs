use libc::fprintf;

// Stage D (2026-09): `BkBlock`/`BkCellValue::Ptr` become an owned, `Box`-based
// recursive tree. The previous version of this comment (added 2026-09-07,
// correcting an even earlier "single-parent forest" claim) argued this was
// *impossible*: `bk_minimize_graph`/`replaceptr` (in `bkgraph.rs`) make
// multiple cells alias the same block after minimization, so -- the argument
// went -- two `Box`es would have to alias one allocation.
//
// That argument conflated two different types. `bk_minimize_graph` operates
// entirely inside `bkgraph.rs`'s `BkGraph` arena, on `ArenaCellValue::
// Ptr(Option<BlockId>)` -- a `Copy` index, not a pointer, and a completely
// separate type from this file's `BkCellValue::Ptr`. By the time any
// aliasing happens, `bk_new_graph_from_root_block`'s `dfs_convert` has
// already walked the *raw* `BkBlock` tree once, post-order, converting every
// node into the arena and freeing every original raw block via `to_free` --
// so there are zero `BkCellValue::Ptr` values left alive to alias. An
// exhaustive audit of every one of this crate's ~19 files that call into
// `bk/` (2026-09, re-run independently of the above argument) confirms
// aliasing never happens on the construction side either: every block
// pointer produced by `bk_new_block`/`bk_push`/helper functions is consumed
// by exactly one later `bk_ptr` call (folded into exactly one parent), or
// flows straight into `bk_build_block`. `BkCellType::Copy` (the one cell
// kind whose contract doesn't consume its target) has zero callers anywhere
// in the crate -- only `Embed` (splice-and-free, i.e. still single-owner) is
// ever used.
//
// So the ownership model this file actually needs is a plain tree, not an
// arena: `Ptr(*mut BkBlock)` -> `Ptr(Option<Box<BkBlock>>)`. Post-minimize
// sharing is real, but it happens one level up, entirely inside `bkgraph.rs`'s
// already-arena-based `BkGraph` -- this file's raw tree is consumed, not
// retained, by the time that sharing occurs.
pub struct BkBlock {
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
// which variant a given `t` implies.
//
// No longer `Copy`, and no longer `Clone` either: `Ptr` now owns a
// `Box<BkBlock>`, and every construction-time consumer moves cells (built
// fresh as `vec![...]` literals) exactly once -- nothing needs a second copy.
pub struct BkCell {
    pub t: BkCellType,
    pub value: BkCellValue,
}
pub enum BkCellValue {
    Int(u32),
    Ptr(Option<Box<BkBlock>>),
}
impl BkCell {
    /// Takes the pointer cell's target, consuming `self`. Panics instead of
    /// reading union garbage if `t` didn't actually imply a pointer cell --
    /// every call site already established this via `bk_cell_is_pointer` or
    /// a `t`-keyed match arm before reaching here.
    pub fn into_ptr(self) -> Option<Box<BkBlock>> {
        match self.value {
            BkCellValue::Ptr(p) => p,
            BkCellValue::Int(_) => panic!("BkCell::into_ptr called on an integer cell"),
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
use crate::support::buffer::Buffer;
use crate::support::stdio::stderr;

pub fn bk_cell_is_pointer(cell: &BkCell) -> bool {
    cell.t >= BkCellType::P16
}
fn bkpushitems(b: &mut BkBlock, items: Vec<BkCell>) {
    for item in items {
        let curtype = item.t;
        match curtype {
            BkCellType::Copy | BkCellType::Embed => {
                // Splices the target's cells into `b`, consuming the
                // target itself -- the one remaining genuine single-owner
                // teardown in this file, distinct from a `BkBlock` that
                // survives into a `BkGraph` (see the module-level comment).
                // `Copy`/`Embed` only differed (in the old raw-pointer
                // world) when the source was *also* reachable through some
                // other cell -- "splice without freeing the source" vs
                // "splice and free" -- but an owned `Box` tree makes a
                // second reference to the same target impossible by
                // construction, and `BkCellType::Copy` has zero callers
                // anywhere in this crate (confirmed by grep), so both arms
                // collapse to the same "take ownership, move the cells
                // over" operation; the (unreachable) `Copy` shell simply
                // drops, empty, at the end of this block.
                if let Some(par) = item.into_ptr() {
                    for cell in par.cells {
                        b.cells.push(cell);
                    }
                }
            }
            _ => b.cells.push(item),
        }
    }
}

/// A fresh block holding `items`.
pub fn bk_new_block(items: Vec<BkCell>) -> BkBlock {
    let mut b = BkBlock { cells: Vec::new() };
    bkpushitems(&mut b, items);
    b
}

/// Append `items` to `b`.
pub fn bk_push(b: &mut BkBlock, items: Vec<BkCell>) {
    bkpushitems(b, items);
}
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
/// `p` is `None` for what used to be a null pointer -- a real, frequently
/// hit state (an absent optional sub-table, for instance).
#[inline]
pub fn bk_ptr(t: BkCellType, p: Option<BkBlock>) -> BkCell {
    BkCell {
        t,
        value: BkCellValue::Ptr(p.map(Box::new)),
    }
}
/// A fresh block holding `data`'s bytes as `B8` cells. Replaces the former
/// `bk_new_block_from_string_len(len, *const c_char)`, whose C-string-cast
/// signature was always gratuitous -- its one caller already held the bytes
/// in a `Vec<u8>`.
pub fn bk_new_block_from_bytes(data: &[u8]) -> BkBlock {
    let mut b = bk_new_block(Vec::new());
    for &byte in data {
        b.cells.push(bk_int(BkCellType::B8, byte as u32));
    }
    b
}
pub fn bk_new_block_from_buffer(buf: Option<Buffer>) -> Option<BkBlock> {
    let buf = buf?;
    Some(bk_new_block_from_bytes(&buf.data))
}
pub fn bk_new_block_from_buffer_copy(buf: Option<&Buffer>) -> Option<BkBlock> {
    let buf = buf?;
    Some(bk_new_block_from_bytes(&buf.data))
}
/// Debug-print `b`'s cells to stderr. Zero callers anywhere in this crate
/// today (confirmed by grep before this conversion, same as `bufprint`'s
/// status when Stage 9 reached it) -- kept as a manual-debugging tool rather
/// than deleted, per that same precedent's resolution.
pub fn bk_print_block(b: &BkBlock) {
    unsafe {
        fprintf(
            stderr,
            b"Block size %08x\n\0" as *const u8 as *const ::core::ffi::c_char,
            b.cells.len() as u32,
        );
        fprintf(
            stderr,
            b"------------------\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    for cell in b.cells.iter() {
        if bk_cell_is_pointer(cell) {
            match &cell.value {
                BkCellValue::Ptr(Some(p)) => unsafe {
                    fprintf(
                        stderr,
                        b"  %3d %p\n\0" as *const u8 as *const ::core::ffi::c_char,
                        cell.t as ::core::ffi::c_uint,
                        &**p as *const BkBlock,
                    );
                },
                _ => unsafe {
                    fprintf(
                        stderr,
                        b"  %3d [NULL]\n\0" as *const u8 as *const ::core::ffi::c_char,
                        cell.t as ::core::ffi::c_uint,
                    );
                },
            }
        } else {
            unsafe {
                fprintf(
                    stderr,
                    b"  %3d %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                    cell.t as ::core::ffi::c_uint,
                    cell.as_int(),
                );
            }
        }
    }
    unsafe {
        fprintf(
            stderr,
            b"------------------\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
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
