use libc::{free, malloc, memcpy, memset};
extern "C" {
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{pos_to_u16, read_16u, read_16s};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphid_t, length_t, pos_t};
use crate::vendor::sds::{sds};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

use crate::table::hhea::{table_hhea};
use crate::table::maxp::{table_maxp};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct horizontal_metric {
    pub advanceWidth: length_t,
    pub lsb: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_hmtx {
    pub metrics: *mut horizontal_metric,
    pub leftSideBearing: *mut pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_hmtx {
    pub init: Option<unsafe extern "C" fn(*mut table_hmtx) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_hmtx, *const table_hmtx) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_hmtx, *mut table_hmtx) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_hmtx) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_hmtx, table_hmtx) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_hmtx, table_hmtx) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_hmtx>,
    pub free: Option<unsafe extern "C" fn(*mut table_hmtx) -> ()>,
}
#[inline]
unsafe extern "C" fn disposeHmtx(mut table: *mut table_hmtx) {
    if !(*table).metrics.is_null() {
        free((*table).metrics as *mut ::core::ffi::c_void);
        (*table).metrics = ::core::ptr::null_mut::<horizontal_metric>();
    }
    if !(*table).leftSideBearing.is_null() {
        free((*table).leftSideBearing as *mut ::core::ffi::c_void);
        (*table).leftSideBearing = ::core::ptr::null_mut::<pos_t>();
    }
}
#[inline]
unsafe extern "C" fn table_hmtx_dispose(mut x: *mut table_hmtx) {
    disposeHmtx(x);
}
#[inline]
unsafe extern "C" fn table_hmtx_copy(mut dst: *mut table_hmtx, mut src: *const table_hmtx) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hmtx>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hmtx_create() -> *mut table_hmtx {
    let mut x: *mut table_hmtx =
        malloc(::core::mem::size_of::<table_hmtx>() as usize) as *mut table_hmtx;
    table_hmtx_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_hmtx_init(mut x: *mut table_hmtx) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_hmtx>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_hmtx_copyReplace(mut dst: *mut table_hmtx, src: table_hmtx) {
    table_hmtx_dispose(dst);
    table_hmtx_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_hmtx_move(mut dst: *mut table_hmtx, mut src: *mut table_hmtx) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hmtx>() as usize,
    );
    table_hmtx_init(src);
}
#[inline]
unsafe extern "C" fn table_hmtx_replace(mut dst: *mut table_hmtx, src: table_hmtx) {
    table_hmtx_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_hmtx>() as usize,
    );
}
#[no_mangle]
pub static mut table_iHmtx: __caryll_elementinterface_table_hmtx = {
    __caryll_elementinterface_table_hmtx {
        init: Some(table_hmtx_init as unsafe extern "C" fn(*mut table_hmtx) -> ()),
        copy: Some(
            table_hmtx_copy as unsafe extern "C" fn(*mut table_hmtx, *const table_hmtx) -> (),
        ),
        move_0: Some(
            table_hmtx_move as unsafe extern "C" fn(*mut table_hmtx, *mut table_hmtx) -> (),
        ),
        dispose: Some(table_hmtx_dispose as unsafe extern "C" fn(*mut table_hmtx) -> ()),
        replace: Some(
            table_hmtx_replace as unsafe extern "C" fn(*mut table_hmtx, table_hmtx) -> (),
        ),
        copyReplace: Some(
            table_hmtx_copyReplace as unsafe extern "C" fn(*mut table_hmtx, table_hmtx) -> (),
        ),
        create: Some(table_hmtx_create),
        free: Some(table_hmtx_free as unsafe extern "C" fn(*mut table_hmtx) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_hmtx_free(mut x: *mut table_hmtx) {
    if x.is_null() {
        return;
    }
    table_hmtx_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readHmtx(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
    mut hhea: *mut table_hhea,
    mut maxp: *mut table_maxp,
) -> *mut table_hmtx {
    if hhea.is_null()
        || maxp.is_null()
        || (*hhea).numberOfMetrics == 0
        || ((*maxp).numGlyphs as ::core::ffi::c_int) < (*hhea).numberOfMetrics as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<table_hmtx>();
    }
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1752003704i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    let mut hmtx: *mut table_hmtx = ::core::ptr::null_mut::<table_hmtx>();
                    let mut count_a: glyphid_t = (*hhea).numberOfMetrics as glyphid_t;
                    let mut count_k: glyphid_t = ((*maxp).numGlyphs as ::core::ffi::c_int
                        - (*hhea).numberOfMetrics as ::core::ffi::c_int)
                        as glyphid_t;
                    if length
                        < (count_a as ::core::ffi::c_int * 4 as ::core::ffi::c_int
                            + count_k as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as u32
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Table 'hmtx' corrupted.\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                        if !hmtx.is_null() {
                            table_iHmtx.free.expect("non-null function pointer")(hmtx);
                            hmtx = ::core::ptr::null_mut::<table_hmtx>();
                        }
                    } else {
                        hmtx = __caryll_allocate_clean(
                            ::core::mem::size_of::<table_hmtx>() as usize,
                            27 as ::core::ffi::c_ulong,
                        ) as *mut table_hmtx;
                        (*hmtx).metrics = __caryll_allocate_clean(
                            (::core::mem::size_of::<horizontal_metric>() as usize)
                                .wrapping_mul(count_a as usize),
                            28 as ::core::ffi::c_ulong,
                        ) as *mut horizontal_metric;
                        (*hmtx).leftSideBearing = __caryll_allocate_clean(
                            (::core::mem::size_of::<pos_t>() as usize)
                                .wrapping_mul(count_k as usize),
                            29 as ::core::ffi::c_ulong,
                        ) as *mut pos_t;
                        let mut ia: glyphid_t = 0 as glyphid_t;
                        while (ia as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
                            (*(*hmtx).metrics.offset(ia as isize)).advanceWidth =
                                read_16u(data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                ) as *const u8) as length_t;
                            (*(*hmtx).metrics.offset(ia as isize)).lsb = read_16s(
                                data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                )
                                .offset(2 as ::core::ffi::c_int as isize)
                                    as *const u8,
                            )
                                as pos_t;
                            ia = ia.wrapping_add(1);
                        }
                        let mut ik: glyphid_t = 0 as glyphid_t;
                        while (ik as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
                            *(*hmtx).leftSideBearing.offset(ik as isize) = read_16s(
                                data.offset(
                                    (count_a as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                .offset(
                                    (ik as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                            )
                                as pos_t;
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
    return ::core::ptr::null_mut::<table_hmtx>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildHmtx(
    mut hmtx: *const table_hmtx,
    mut count_a: glyphid_t,
    mut count_k: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    let mut buf: *mut caryll_Buffer = bufnew();
    if hmtx.is_null() {
        return buf;
    }
    if !(*hmtx).metrics.is_null() {
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            bufwrite16b(
                buf,
                (*(*hmtx).metrics.offset(j as isize)).advanceWidth as u16,
            );
            bufwrite16b(buf, pos_to_u16((*(*hmtx).metrics.offset(j as isize)).lsb));
            j = j.wrapping_add(1);
        }
    }
    if !(*hmtx).leftSideBearing.is_null() {
        let mut j_0: glyphid_t = 0 as glyphid_t;
        while (j_0 as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
            bufwrite16b(
                buf,
                pos_to_u16(*(*hmtx).leftSideBearing.offset(j_0 as isize)),
            );
            j_0 = j_0.wrapping_add(1);
        }
    }
    return buf;
}
