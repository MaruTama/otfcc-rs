#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{pos_to_u16, read_16u, read_16s};
use crate::logger::{LoggerType, log_vl_important, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, Pos};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::vendor::sds::{sdsempty};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VorgEntry {
    pub gid: GlyphId,
    pub verticalOrigin: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VorgTable {
    pub numVertOriginYMetrics: GlyphId,
    pub defaultVerticalOrigin: Pos,
    pub entries: *mut VorgEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VorgTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VorgTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VorgTable, *const VorgTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VorgTable, *mut VorgTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VorgTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VorgTable, VorgTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VorgTable, VorgTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VorgTable>,
    pub free: Option<unsafe extern "C" fn(*mut VorgTable) -> ()>,
}
#[inline]
unsafe extern "C" fn disposeVORG(mut vorg: *mut VorgTable) {
    free((*vorg).entries as *mut ::core::ffi::c_void);
    (*vorg).entries = ::core::ptr::null_mut::<VorgEntry>();
}
pub static table_iVORG: VorgTableElementInterface = {
    VorgTableElementInterface {
        init: Some(table_VORG_init as unsafe extern "C" fn(*mut VorgTable) -> ()),
        copy: Some(
            table_VORG_copy as unsafe extern "C" fn(*mut VorgTable, *const VorgTable) -> (),
        ),
        move_0: Some(
            table_VORG_move as unsafe extern "C" fn(*mut VorgTable, *mut VorgTable) -> (),
        ),
        dispose: Some(table_VORG_dispose as unsafe extern "C" fn(*mut VorgTable) -> ()),
        replace: Some(
            table_VORG_replace as unsafe extern "C" fn(*mut VorgTable, VorgTable) -> (),
        ),
        copyReplace: Some(
            table_VORG_copyReplace as unsafe extern "C" fn(*mut VorgTable, VorgTable) -> (),
        ),
        create: Some(table_VORG_create),
        free: Some(table_VORG_free as unsafe extern "C" fn(*mut VorgTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_VORG_free(mut x: *mut VorgTable) {
    if x.is_null() {
        return;
    }
    table_VORG_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_VORG_dispose(mut x: *mut VorgTable) {
    disposeVORG(x);
}
#[inline]
unsafe extern "C" fn table_VORG_init(mut x: *mut VorgTable) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VorgTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_VORG_copyReplace(mut dst: *mut VorgTable, src: VorgTable) {
    table_VORG_dispose(dst);
    table_VORG_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_VORG_copy(mut dst: *mut VorgTable, mut src: *const VorgTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VorgTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_VORG_replace(mut dst: *mut VorgTable, src: VorgTable) {
    table_VORG_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VorgTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_VORG_move(mut dst: *mut VorgTable, mut src: *mut VorgTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VorgTable>() as usize,
    );
    table_VORG_init(src);
}
#[inline]
unsafe extern "C" fn table_VORG_create() -> *mut VorgTable {
    let mut x: *mut VorgTable =
        malloc(::core::mem::size_of::<VorgTable>() as usize) as *mut VorgTable;
    table_VORG_init(x);
    return x;
}
pub unsafe extern "C" fn otfcc_readVORG(
    packet: Packet,
    mut options: *const Options,
) -> *mut VorgTable {
    let mut numVertOriginYMetrics: u16 = 0;
    let mut vorg: *mut VorgTable = ::core::ptr::null_mut::<VorgTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1448038983i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if !(length < 8 as u32) {
                        numVertOriginYMetrics = read_16u(
                            data.offset(6 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        if !(length
                            < (8 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                    * numVertOriginYMetrics as ::core::ffi::c_int)
                                as u32)
                        {
                            vorg = (
                                table_iVORG.create.expect("non-null function pointer"))();
                            (*vorg).defaultVerticalOrigin = read_16s(
                                data.offset(4 as ::core::ffi::c_int as isize) as *const u8,
                            ) as Pos;
                            (*vorg).numVertOriginYMetrics = numVertOriginYMetrics as GlyphId;
                            (*vorg).entries = __caryll_allocate_clean(
                                (::core::mem::size_of::<VorgEntry>() as usize)
                                    .wrapping_mul(numVertOriginYMetrics as usize),
                                22 as ::core::ffi::c_ulong,
                            ) as *mut VorgEntry;
                            let mut j: u16 = 0 as u16;
                            while (j as ::core::ffi::c_int)
                                < numVertOriginYMetrics as ::core::ffi::c_int
                            {
                                (*(*vorg).entries.offset(j as isize)).gid = read_16u(
                                    data.offset(8 as ::core::ffi::c_int as isize).offset(
                                        (4 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                                )
                                    as GlyphId;
                                (*(*vorg).entries.offset(j as isize)).verticalOrigin = read_16s(
                                    data.offset(8 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (4 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                j = j.wrapping_add(1);
                            }
                            return vorg;
                        }
                    }
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        log_vl_important,
                        LoggerType::Warning,
                        crate::sdsbuild!(sdsempty(), b"Table 'VORG' corrupted."),
                    );
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<VorgTable>();
}
pub unsafe extern "C" fn otfcc_buildVORG(
    mut table: *const VorgTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if table.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 1 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, pos_to_u16((*table).defaultVerticalOrigin));
    bufwrite16b(buf, (*table).numVertOriginYMetrics as u16);
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < (*table).numVertOriginYMetrics as ::core::ffi::c_int {
        bufwrite16b(buf, (*(*table).entries.offset(j as isize)).gid as u16);
        bufwrite16b(
            buf,
            (*(*table).entries.offset(j as isize)).verticalOrigin as u16,
        );
        j = j.wrapping_add(1);
    }
    return buf;
}
