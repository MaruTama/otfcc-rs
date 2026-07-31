#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{pos_to_u16, read_16u, read_16s};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
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
    pub vertical_origin: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VorgTable {
    pub num_vert_origin_y_metrics: GlyphId,
    pub default_vertical_origin: Pos,
    pub entries: *mut VorgEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VorgTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VorgTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VorgTable, *const VorgTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VorgTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VorgTable>,
    pub free: Option<unsafe extern "C" fn(*mut VorgTable) -> ()>,
}
#[inline]
unsafe extern "C" fn dispose_vorg(mut vorg: *mut VorgTable) {
    free((*vorg).entries as *mut ::core::ffi::c_void);
    (*vorg).entries = ::core::ptr::null_mut::<VorgEntry>();
}
pub static TABLE_I_VORG: VorgTableElementInterface = {
    VorgTableElementInterface {
        init: Some(table_vorg_init as unsafe extern "C" fn(*mut VorgTable) -> ()),
        copy: Some(
            table_vorg_copy as unsafe extern "C" fn(*mut VorgTable, *const VorgTable) -> (),
        ),
        dispose: Some(table_vorg_dispose as unsafe extern "C" fn(*mut VorgTable) -> ()),
        create: Some(table_vorg_create),
        free: Some(table_vorg_free as unsafe extern "C" fn(*mut VorgTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_vorg_free(mut x: *mut VorgTable) {
    if x.is_null() {
        return;
    }
    table_vorg_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_vorg_dispose(mut x: *mut VorgTable) {
    dispose_vorg(x);
}
#[inline]
unsafe extern "C" fn table_vorg_init(mut x: *mut VorgTable) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VorgTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_vorg_copy(mut dst: *mut VorgTable, mut src: *const VorgTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VorgTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_vorg_create() -> *mut VorgTable {
    let mut x: *mut VorgTable =
        malloc(::core::mem::size_of::<VorgTable>() as usize) as *mut VorgTable;
    table_vorg_init(x);
    return x;
}
pub unsafe extern "C" fn otfcc_read_vorg(
    packet: Packet,
    mut options: *const Options,
) -> *mut VorgTable {
    let mut num_vert_origin_y_metrics: u16 = 0;
    let mut vorg: *mut VorgTable = ::core::ptr::null_mut::<VorgTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1448038983i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if !(length < 8 as u32) {
                        num_vert_origin_y_metrics = read_16u(
                            data.offset(6 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        if !(length
                            < (8 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                    * num_vert_origin_y_metrics as ::core::ffi::c_int)
                                as u32)
                        {
                            vorg = (
                                TABLE_I_VORG.create.expect("non-null function pointer"))();
                            (*vorg).default_vertical_origin = read_16s(
                                data.offset(4 as ::core::ffi::c_int as isize) as *const u8,
                            ) as Pos;
                            (*vorg).num_vert_origin_y_metrics = num_vert_origin_y_metrics as GlyphId;
                            (*vorg).entries = __caryll_allocate_clean(
                                (::core::mem::size_of::<VorgEntry>() as usize)
                                    .wrapping_mul(num_vert_origin_y_metrics as usize),
                                22 as ::core::ffi::c_ulong,
                            ) as *mut VorgEntry;
                            let mut j: u16 = 0 as u16;
                            while (j as ::core::ffi::c_int)
                                < num_vert_origin_y_metrics as ::core::ffi::c_int
                            {
                                (*(*vorg).entries.offset(j as isize)).gid = read_16u(
                                    data.offset(8 as ::core::ffi::c_int as isize).offset(
                                        (4 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                                )
                                    as GlyphId;
                                (*(*vorg).entries.offset(j as isize)).vertical_origin = read_16s(
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
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
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
pub unsafe extern "C" fn otfcc_build_vorg(
    mut table: *const VorgTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if table.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 1 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, pos_to_u16((*table).default_vertical_origin));
    bufwrite16b(buf, (*table).num_vert_origin_y_metrics as u16);
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < (*table).num_vert_origin_y_metrics as ::core::ffi::c_int {
        bufwrite16b(buf, (*(*table).entries.offset(j as isize)).gid as u16);
        bufwrite16b(
            buf,
            (*(*table).entries.offset(j as isize)).vertical_origin as u16,
        );
        j = j.wrapping_add(1);
    }
    return buf;
}
