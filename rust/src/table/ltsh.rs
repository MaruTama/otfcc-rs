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
    pub num_glyphs: GlyphId,
    pub y_pels: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LtshTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut LtshTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut LtshTable, *const LtshTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut LtshTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut LtshTable>,
    pub free: Option<unsafe extern "C" fn(*mut LtshTable) -> ()>,
}
#[inline]
unsafe extern "C" fn dispose_ltsh(mut ltsh: *mut LtshTable) {
    if !ltsh.is_null() {
        free((*ltsh).y_pels as *mut ::core::ffi::c_void);
        (*ltsh).y_pels = ::core::ptr::null_mut::<u8>();
    }
}
#[inline]
unsafe extern "C" fn table_ltsh_free(mut x: *mut LtshTable) {
    if x.is_null() {
        return;
    }
    table_ltsh_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub static TABLE_I_LTSH: LtshTableElementInterface = {
    LtshTableElementInterface {
        init: Some(table_ltsh_init as unsafe extern "C" fn(*mut LtshTable) -> ()),
        copy: Some(
            table_ltsh_copy as unsafe extern "C" fn(*mut LtshTable, *const LtshTable) -> (),
        ),
        dispose: Some(table_ltsh_dispose as unsafe extern "C" fn(*mut LtshTable) -> ()),
        create: Some(table_ltsh_create),
        free: Some(table_ltsh_free as unsafe extern "C" fn(*mut LtshTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_ltsh_dispose(mut x: *mut LtshTable) {
    dispose_ltsh(x);
}
#[inline]
unsafe extern "C" fn table_ltsh_create() -> *mut LtshTable {
    let mut x: *mut LtshTable =
        malloc(::core::mem::size_of::<LtshTable>() as usize) as *mut LtshTable;
    table_ltsh_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_ltsh_init(mut x: *mut LtshTable) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<LtshTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_ltsh_copy(mut dst: *mut LtshTable, mut src: *const LtshTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LtshTable>() as usize,
    );
}
pub unsafe extern "C" fn otfcc_read_ltsh(
    packet: Packet,
    mut _options: *const Options,
) -> *mut LtshTable {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1280594760i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut ltsh: *mut LtshTable = ::core::ptr::null_mut::<LtshTable>();
                    ltsh = __caryll_allocate_clean(
                        ::core::mem::size_of::<LtshTable>() as usize,
                        15 as ::core::ffi::c_ulong,
                    ) as *mut LtshTable;
                    (*ltsh).version = read_16u(data as *const u8);
                    (*ltsh).num_glyphs =
                        read_16u(data.offset(2 as ::core::ffi::c_int as isize) as *const u8)
                            as GlyphId;
                    (*ltsh).y_pels = __caryll_allocate_clean(
                        (::core::mem::size_of::<u8>() as usize)
                            .wrapping_mul((*ltsh).num_glyphs as usize),
                        18 as ::core::ffi::c_ulong,
                    ) as *mut u8;
                    memcpy(
                        (*ltsh).y_pels as *mut ::core::ffi::c_void,
                        data.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                        (*ltsh).num_glyphs as usize,
                    );
                    return ltsh;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<LtshTable>();
}
pub unsafe extern "C" fn otfcc_build_ltsh(
    mut ltsh: *const LtshTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if ltsh.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, (*ltsh).num_glyphs as u16);
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < (*ltsh).num_glyphs as ::core::ffi::c_int {
        bufwrite8(buf, *(*ltsh).y_pels.offset(j as isize));
        j = j.wrapping_add(1);
    }
    return buf;
}
