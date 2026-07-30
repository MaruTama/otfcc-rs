#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{pos_to_u16, read_16u, read_16s};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, Length, Pos};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::maxp::{MaxpTable};
use crate::table::vhea::{VheaTable};
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::vendor::sds::{sdsempty};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VerticalMetric {
    pub advance_height: Length,
    pub tsb: Pos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VmtxTable {
    pub metrics: *mut VerticalMetric,
    pub top_side_bearing: *mut Pos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VmtxTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut VmtxTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VmtxTable, *const VmtxTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VmtxTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut VmtxTable>,
    pub free: Option<unsafe extern "C" fn(*mut VmtxTable) -> ()>,
}
#[inline]
unsafe extern "C" fn dispose_vmtx(mut table: *mut VmtxTable) {
    if !(*table).metrics.is_null() {
        free((*table).metrics as *mut ::core::ffi::c_void);
        (*table).metrics = ::core::ptr::null_mut::<VerticalMetric>();
    }
    if !(*table).top_side_bearing.is_null() {
        free((*table).top_side_bearing as *mut ::core::ffi::c_void);
        (*table).top_side_bearing = ::core::ptr::null_mut::<Pos>();
    }
}
#[inline]
unsafe extern "C" fn table_vmtx_dispose(mut x: *mut VmtxTable) {
    dispose_vmtx(x);
}
#[inline]
unsafe extern "C" fn table_vmtx_copy(mut dst: *mut VmtxTable, mut src: *const VmtxTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<VmtxTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_vmtx_create() -> *mut VmtxTable {
    let mut x: *mut VmtxTable =
        malloc(::core::mem::size_of::<VmtxTable>() as usize) as *mut VmtxTable;
    table_vmtx_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_vmtx_init(mut x: *mut VmtxTable) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VmtxTable>() as usize,
    );
}
pub static TABLE_I_VMTX: VmtxTableElementInterface = {
    VmtxTableElementInterface {
        init: Some(table_vmtx_init as unsafe extern "C" fn(*mut VmtxTable) -> ()),
        copy: Some(
            table_vmtx_copy as unsafe extern "C" fn(*mut VmtxTable, *const VmtxTable) -> (),
        ),
        dispose: Some(table_vmtx_dispose as unsafe extern "C" fn(*mut VmtxTable) -> ()),
        create: Some(table_vmtx_create),
        free: Some(table_vmtx_free as unsafe extern "C" fn(*mut VmtxTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_vmtx_free(mut x: *mut VmtxTable) {
    if x.is_null() {
        return;
    }
    table_vmtx_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_read_vmtx(
    packet: Packet,
    mut options: *const Options,
    mut vhea: *mut VheaTable,
    mut maxp: *mut MaxpTable,
) -> *mut VmtxTable {
    if vhea.is_null()
        || maxp.is_null()
        || (*vhea).num_of_long_ver_metrics as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || ((*maxp).num_glyphs as ::core::ffi::c_int)
            < (*vhea).num_of_long_ver_metrics as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<VmtxTable>();
    }
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1986884728i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    let mut vmtx: *mut VmtxTable = ::core::ptr::null_mut::<VmtxTable>();
                    let mut count_a: GlyphId = (*vhea).num_of_long_ver_metrics as GlyphId;
                    let mut count_k: GlyphId = ((*maxp).num_glyphs as ::core::ffi::c_int
                        - (*vhea).num_of_long_ver_metrics as ::core::ffi::c_int)
                        as GlyphId;
                    if length
                        < (count_a as ::core::ffi::c_int * 4 as ::core::ffi::c_int
                            + count_k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as u32
                    {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::sdsbuild!(sdsempty(), b"Table 'vmtx' corrupted.\n"),
                        );
                        if !vmtx.is_null() {
                            TABLE_I_VMTX.free.expect("non-null function pointer")(vmtx);
                            vmtx = ::core::ptr::null_mut::<VmtxTable>();
                        }
                    } else {
                        vmtx = __caryll_allocate_clean(
                            ::core::mem::size_of::<VmtxTable>() as usize,
                            27 as ::core::ffi::c_ulong,
                        ) as *mut VmtxTable;
                        (*vmtx).metrics = __caryll_allocate_clean(
                            (::core::mem::size_of::<VerticalMetric>() as usize)
                                .wrapping_mul(count_a as usize),
                            28 as ::core::ffi::c_ulong,
                        ) as *mut VerticalMetric;
                        (*vmtx).top_side_bearing = __caryll_allocate_clean(
                            (::core::mem::size_of::<Pos>() as usize)
                                .wrapping_mul(count_k as usize),
                            29 as ::core::ffi::c_ulong,
                        ) as *mut Pos;
                        let mut ia: GlyphId = 0 as GlyphId;
                        while (ia as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
                            (*(*vmtx).metrics.offset(ia as isize)).advance_height =
                                read_16u(data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                ) as *const u8) as Length;
                            (*(*vmtx).metrics.offset(ia as isize)).tsb = read_16s(
                                data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                )
                                .offset(2 as ::core::ffi::c_int as isize)
                                    as *const u8,
                            )
                                as Pos;
                            ia = ia.wrapping_add(1);
                        }
                        let mut ik: GlyphId = 0 as GlyphId;
                        while (ik as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
                            *(*vmtx).top_side_bearing.offset(ik as isize) = read_16s(
                                data.offset(
                                    (count_a as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                .offset(
                                    (ik as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                            )
                                as Pos;
                            ik = ik.wrapping_add(1);
                        }
                        return vmtx;
                    }
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<VmtxTable>();
}
pub unsafe extern "C" fn otfcc_build_vmtx(
    mut vmtx: *const VmtxTable,
    mut count_a: GlyphId,
    mut count_k: GlyphId,
    mut _options: *const Options,
) -> *mut Buffer {
    if vmtx.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    if !(*vmtx).metrics.is_null() {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            bufwrite16b(
                buf,
                (*(*vmtx).metrics.offset(j as isize)).advance_height as u16,
            );
            bufwrite16b(buf, pos_to_u16((*(*vmtx).metrics.offset(j as isize)).tsb));
            j = j.wrapping_add(1);
        }
    }
    if !(*vmtx).top_side_bearing.is_null() {
        let mut j_0: GlyphId = 0 as GlyphId;
        while (j_0 as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
            bufwrite16b(
                buf,
                pos_to_u16(*(*vmtx).top_side_bearing.offset(j_0 as isize)),
            );
            j_0 = j_0.wrapping_add(1);
        }
    }
    return buf;
}
