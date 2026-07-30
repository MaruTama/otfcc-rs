#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite8};


#[derive(Copy, Clone)]
#[repr(C)]
pub struct LtshTable {
    pub version: u16,
    pub numGlyphs: GlyphId,
    pub yPels: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LtshTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut LtshTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut LtshTable, *const LtshTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut LtshTable, *mut LtshTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut LtshTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut LtshTable, LtshTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut LtshTable, LtshTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LtshTable>,
    pub free: Option<unsafe extern "C" fn(*mut LtshTable) -> ()>,
}
#[inline]
unsafe extern "C" fn disposeLTSH(mut ltsh: *mut LtshTable) {
    if !ltsh.is_null() {
        free((*ltsh).yPels as *mut ::core::ffi::c_void);
        (*ltsh).yPels = ::core::ptr::null_mut::<u8>();
    }
}
#[inline]
unsafe extern "C" fn table_LTSH_free(mut x: *mut LtshTable) {
    if x.is_null() {
        return;
    }
    table_LTSH_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub static TABLE_I_LTSH: LtshTableElementInterface = {
    LtshTableElementInterface {
        init: Some(table_LTSH_init as unsafe extern "C" fn(*mut LtshTable) -> ()),
        copy: Some(
            table_LTSH_copy as unsafe extern "C" fn(*mut LtshTable, *const LtshTable) -> (),
        ),
        move_0: Some(
            table_LTSH_move as unsafe extern "C" fn(*mut LtshTable, *mut LtshTable) -> (),
        ),
        dispose: Some(table_LTSH_dispose as unsafe extern "C" fn(*mut LtshTable) -> ()),
        replace: Some(
            table_LTSH_replace as unsafe extern "C" fn(*mut LtshTable, LtshTable) -> (),
        ),
        copyReplace: Some(
            table_LTSH_copyReplace as unsafe extern "C" fn(*mut LtshTable, LtshTable) -> (),
        ),
        create: Some(table_LTSH_create),
        free: Some(table_LTSH_free as unsafe extern "C" fn(*mut LtshTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_LTSH_dispose(mut x: *mut LtshTable) {
    disposeLTSH(x);
}
#[inline]
unsafe extern "C" fn table_LTSH_create() -> *mut LtshTable {
    let mut x: *mut LtshTable =
        malloc(::core::mem::size_of::<LtshTable>() as usize) as *mut LtshTable;
    table_LTSH_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_LTSH_init(mut x: *mut LtshTable) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<LtshTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_LTSH_copy(mut dst: *mut LtshTable, mut src: *const LtshTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LtshTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_LTSH_copyReplace(mut dst: *mut LtshTable, src: LtshTable) {
    table_LTSH_dispose(dst);
    table_LTSH_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_LTSH_move(mut dst: *mut LtshTable, mut src: *mut LtshTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LtshTable>() as usize,
    );
    table_LTSH_init(src);
}
#[inline]
unsafe extern "C" fn table_LTSH_replace(mut dst: *mut LtshTable, src: LtshTable) {
    table_LTSH_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LtshTable>() as usize,
    );
}
pub unsafe extern "C" fn otfcc_readLTSH(
    packet: Packet,
    mut _options: *const Options,
) -> *mut LtshTable {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1280594760i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut LTSH: *mut LtshTable = ::core::ptr::null_mut::<LtshTable>();
                    LTSH = __caryll_allocate_clean(
                        ::core::mem::size_of::<LtshTable>() as usize,
                        15 as ::core::ffi::c_ulong,
                    ) as *mut LtshTable;
                    (*LTSH).version = read_16u(data as *const u8);
                    (*LTSH).numGlyphs =
                        read_16u(data.offset(2 as ::core::ffi::c_int as isize) as *const u8)
                            as GlyphId;
                    (*LTSH).yPels = __caryll_allocate_clean(
                        (::core::mem::size_of::<u8>() as usize)
                            .wrapping_mul((*LTSH).numGlyphs as usize),
                        18 as ::core::ffi::c_ulong,
                    ) as *mut u8;
                    memcpy(
                        (*LTSH).yPels as *mut ::core::ffi::c_void,
                        data.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                        (*LTSH).numGlyphs as usize,
                    );
                    return LTSH;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<LtshTable>();
}
pub unsafe extern "C" fn otfcc_buildLTSH(
    mut ltsh: *const LtshTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if ltsh.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, (*ltsh).numGlyphs as u16);
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < (*ltsh).numGlyphs as ::core::ffi::c_int {
        bufwrite8(buf, *(*ltsh).yPels.offset(j as isize));
        j = j.wrapping_add(1);
    }
    return buf;
}
