use libc::{fprintf, free};
extern "C" {
    fn buffree(buf: *mut caryll_Buffer);
}

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
use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::buffer::{caryll_Buffer};

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
#[no_mangle]
pub unsafe extern "C" fn bk_cellIsPointer(mut cell: *mut bk_Cell) -> bool {
    return (*cell).t as ::core::ffi::c_uint >= p16 as ::core::ffi::c_int as ::core::ffi::c_uint;
}
unsafe extern "C" fn bkblock_grow(mut b: *mut bk_Block, mut len: u32) -> *mut bk_Cell {
    let mut olen: u32 = (*b).length;
    bkblock_acells(b, olen.wrapping_add(len));
    return (*b).cells.offset(olen as isize) as *mut bk_Cell;
}
#[no_mangle]
pub unsafe extern "C" fn _bkblock_init() -> *mut bk_Block {
    let mut b: *mut bk_Block = ::core::ptr::null_mut::<bk_Block>();
    b = __caryll_allocate_clean(
        ::core::mem::size_of::<bk_Block>() as usize,
        27 as ::core::ffi::c_ulong,
    ) as *mut bk_Block;
    bkblock_acells(b, 0 as u32);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bkblock_pushint(
    mut b: *mut bk_Block,
    mut type_0: bk_CellType,
    mut x: u32,
) {
    let mut cell: *mut bk_Cell = bkblock_grow(b, 1 as u32);
    (*cell).t = type_0;
    (*cell).c2rust_unnamed.z = x;
}
#[no_mangle]
pub unsafe extern "C" fn bkblock_pushptr(
    mut b: *mut bk_Block,
    mut type_0: bk_CellType,
    mut p: *mut bk_Block,
) {
    let mut cell: *mut bk_Cell = bkblock_grow(b, 1 as u32);
    (*cell).t = type_0;
    (*cell).c2rust_unnamed.p = p as *mut __caryll_bkblock;
}
unsafe extern "C" fn vbkpushitems(b: *mut bk_Block, type0: bk_CellType, mut ap: ::core::ffi::VaList) {
    // bk_CellType's variants (bkover/b8/b16/b32/p16/p32/sp16/sp32/bkcopy/
    // bkembed) are all the same c_uint type, so the c2rust-generated
    // triple-cast comparisons (`x as c_uint == y as c_int as c_uint`) were
    // always just `x == y`; matching on the named consts directly is
    // equivalent and self-documenting.
    let mut curtype: bk_CellType = type0;
    while curtype != bkover {
        match curtype {
            bkcopy | bkembed => {
                let par: *mut bk_Block = ap.arg::<*mut bk_Block>();
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
            t if t < p16 => {
                let par_0: u32 = ap.arg::<::core::ffi::c_int>() as u32;
                bkblock_pushint(b, curtype, par_0);
            }
            _ => {
                let par_1: *mut bk_Block = ap.arg::<*mut bk_Block>();
                bkblock_pushptr(b, curtype, par_1);
            }
        }
        curtype = ap.arg::<::core::ffi::c_int>() as bk_CellType;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bk_new_Block(
    mut type0: ::core::ffi::c_int,
    mut args: ...
) -> *mut bk_Block {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    let mut b: *mut bk_Block = _bkblock_init();
    vbkpushitems(b, type0 as bk_CellType, ap.as_va_list());
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bk_push(
    mut b: *mut bk_Block,
    mut type0: ::core::ffi::c_int,
    mut args: ...
) -> *mut bk_Block {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    vbkpushitems(b, type0 as bk_CellType, ap.as_va_list());
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bk_newBlockFromStringLen(
    len: usize,
    str: *const ::core::ffi::c_char,
) -> *mut bk_Block {
    if str.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let b: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
    for j in 0..len {
        bkblock_pushint(b, b8, *str.offset(j as isize) as u32);
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block {
    if buf.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let b: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
    for j in 0..(*buf).size {
        bkblock_pushint(b, b8, *(*buf).data.offset(j as isize) as u32);
    }
    buffree(buf);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bk_newBlockFromBufferCopy(buf: *const caryll_Buffer) -> *mut bk_Block {
    if buf.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let b: *mut bk_Block = bk_new_Block(bkover as ::core::ffi::c_int);
    for j in 0..(*buf).size {
        bkblock_pushint(b, b8, *(*buf).data.offset(j as isize) as u32);
    }
    return b;
}
#[no_mangle]
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
