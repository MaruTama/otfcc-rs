#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{pos_to_u16, read_16u, read_16s};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, Length, Pos};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::hhea::{HheaTable};
use crate::table::maxp::{MaxpTable};
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::vendor::sds::{sdsempty};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HorizontalMetric {
    pub advance_width: Length,
    pub lsb: Pos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HmtxTable {
    pub metrics: *mut HorizontalMetric,
    pub left_side_bearing: *mut Pos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HmtxTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut HmtxTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut HmtxTable, *const HmtxTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut HmtxTable, *mut HmtxTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut HmtxTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut HmtxTable, HmtxTable) -> ()>,
    pub copy_replace: Option<unsafe extern "C" fn(*mut HmtxTable, HmtxTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut HmtxTable>,
    pub free: Option<unsafe extern "C" fn(*mut HmtxTable) -> ()>,
}
#[inline]
unsafe extern "C" fn dispose_hmtx(mut table: *mut HmtxTable) {
    if !(*table).metrics.is_null() {
        free((*table).metrics as *mut ::core::ffi::c_void);
        (*table).metrics = ::core::ptr::null_mut::<HorizontalMetric>();
    }
    if !(*table).left_side_bearing.is_null() {
        free((*table).left_side_bearing as *mut ::core::ffi::c_void);
        (*table).left_side_bearing = ::core::ptr::null_mut::<Pos>();
    }
}
#[inline]
unsafe extern "C" fn table_hmtx_dispose(mut x: *mut HmtxTable) {
    dispose_hmtx(x);
}
#[inline]
unsafe extern "C" fn table_hmtx_copy(mut dst: *mut HmtxTable, mut src: *const HmtxTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HmtxTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hmtx_create() -> *mut HmtxTable {
    let mut x: *mut HmtxTable =
        malloc(::core::mem::size_of::<HmtxTable>() as usize) as *mut HmtxTable;
    table_hmtx_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_hmtx_init(mut x: *mut HmtxTable) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<HmtxTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hmtx_copy_replace(mut dst: *mut HmtxTable, src: HmtxTable) {
    table_hmtx_dispose(dst);
    table_hmtx_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_hmtx_move(mut dst: *mut HmtxTable, mut src: *mut HmtxTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HmtxTable>() as usize,
    );
    table_hmtx_init(src);
}
#[inline]
unsafe extern "C" fn table_hmtx_replace(mut dst: *mut HmtxTable, src: HmtxTable) {
    table_hmtx_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HmtxTable>() as usize,
    );
}
pub static TABLE_I_HMTX: HmtxTableElementInterface = {
    HmtxTableElementInterface {
        init: Some(table_hmtx_init as unsafe extern "C" fn(*mut HmtxTable) -> ()),
        copy: Some(
            table_hmtx_copy as unsafe extern "C" fn(*mut HmtxTable, *const HmtxTable) -> (),
        ),
        move_0: Some(
            table_hmtx_move as unsafe extern "C" fn(*mut HmtxTable, *mut HmtxTable) -> (),
        ),
        dispose: Some(table_hmtx_dispose as unsafe extern "C" fn(*mut HmtxTable) -> ()),
        replace: Some(
            table_hmtx_replace as unsafe extern "C" fn(*mut HmtxTable, HmtxTable) -> (),
        ),
        copy_replace: Some(
            table_hmtx_copy_replace as unsafe extern "C" fn(*mut HmtxTable, HmtxTable) -> (),
        ),
        create: Some(table_hmtx_create),
        free: Some(table_hmtx_free as unsafe extern "C" fn(*mut HmtxTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_hmtx_free(mut x: *mut HmtxTable) {
    if x.is_null() {
        return;
    }
    table_hmtx_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_read_hmtx(
    packet: Packet,
    mut options: *const Options,
    mut hhea: *mut HheaTable,
    mut maxp: *mut MaxpTable,
) -> *mut HmtxTable {
    if hhea.is_null()
        || maxp.is_null()
        || (*hhea).number_of_metrics == 0
        || ((*maxp).num_glyphs as ::core::ffi::c_int) < (*hhea).number_of_metrics as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<HmtxTable>();
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
            if table.tag == 1752003704i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    let mut hmtx: *mut HmtxTable = ::core::ptr::null_mut::<HmtxTable>();
                    let mut count_a: GlyphId = (*hhea).number_of_metrics as GlyphId;
                    let mut count_k: GlyphId = ((*maxp).num_glyphs as ::core::ffi::c_int
                        - (*hhea).number_of_metrics as ::core::ffi::c_int)
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
                            crate::sdsbuild!(sdsempty(), b"Table 'hmtx' corrupted.\n"),
                        );
                        if !hmtx.is_null() {
                            TABLE_I_HMTX.free.expect("non-null function pointer")(hmtx);
                            hmtx = ::core::ptr::null_mut::<HmtxTable>();
                        }
                    } else {
                        hmtx = __caryll_allocate_clean(
                            ::core::mem::size_of::<HmtxTable>() as usize,
                            27 as ::core::ffi::c_ulong,
                        ) as *mut HmtxTable;
                        (*hmtx).metrics = __caryll_allocate_clean(
                            (::core::mem::size_of::<HorizontalMetric>() as usize)
                                .wrapping_mul(count_a as usize),
                            28 as ::core::ffi::c_ulong,
                        ) as *mut HorizontalMetric;
                        (*hmtx).left_side_bearing = __caryll_allocate_clean(
                            (::core::mem::size_of::<Pos>() as usize)
                                .wrapping_mul(count_k as usize),
                            29 as ::core::ffi::c_ulong,
                        ) as *mut Pos;
                        let mut ia: GlyphId = 0 as GlyphId;
                        while (ia as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
                            (*(*hmtx).metrics.offset(ia as isize)).advance_width =
                                read_16u(data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                ) as *const u8) as Length;
                            (*(*hmtx).metrics.offset(ia as isize)).lsb = read_16s(
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
                            *(*hmtx).left_side_bearing.offset(ik as isize) = read_16s(
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
                        return hmtx;
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
    return ::core::ptr::null_mut::<HmtxTable>();
}
pub unsafe extern "C" fn otfcc_build_hmtx(
    mut hmtx: *const HmtxTable,
    mut count_a: GlyphId,
    mut count_k: GlyphId,
    mut _options: *const Options,
) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
    if hmtx.is_null() {
        return buf;
    }
    if !(*hmtx).metrics.is_null() {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            bufwrite16b(
                buf,
                (*(*hmtx).metrics.offset(j as isize)).advance_width as u16,
            );
            bufwrite16b(buf, pos_to_u16((*(*hmtx).metrics.offset(j as isize)).lsb));
            j = j.wrapping_add(1);
        }
    }
    if !(*hmtx).left_side_bearing.is_null() {
        let mut j_0: GlyphId = 0 as GlyphId;
        while (j_0 as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
            bufwrite16b(
                buf,
                pos_to_u16(*(*hmtx).left_side_bearing.offset(j_0 as isize)),
            );
            j_0 = j_0.wrapping_add(1);
        }
    }
    return buf;
}
