#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


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
#[repr(C)]
pub struct VorgTable {
    pub num_vert_origin_y_metrics: GlyphId,
    pub default_vertical_origin: Pos,
    pub entries: *mut VorgEntry,
}
// Stage 6-4 "Box化": `entries` is the only allocation this struct owns, so
// `Box<VorgTable>` (via `Font.vorg: Option<Box<VorgTable>>`) plus this
// `Drop` impl replaces the entire `VorgTableElementInterface` vtable that
// used to exist here -- same shape as the `LtshTable` pilot. Grepping
// confirmed only `.free` was ever called from outside this file (from
// `font/caryll_font.rs`'s table disposal and `otf_reader/unconsolidate.rs`'s
// merge step); `.init`/`.copy`/`.create`/`.dispose` were never called at all
// (`otfcc_read_vorg`/`stat_vorg` already built via `__caryll_allocate_clean`
// directly, not through the vtable's `.create`). `Copy`/`Clone` dropped:
// a `Drop` impl and `Copy` are mutually exclusive, and `entries` needing
// single ownership means `Copy` was already semantically wrong before this
// PR, just unenforced.
impl Drop for VorgTable {
    fn drop(&mut self) {
        unsafe {
            if !self.entries.is_null() {
                free(self.entries as *mut ::core::ffi::c_void);
                self.entries = ::core::ptr::null_mut::<VorgEntry>();
            }
        }
    }
}
pub unsafe extern "C" fn otfcc_read_vorg(
    packet: Packet,
    mut options: *const Options,
) -> Option<Box<VorgTable>> {
    let mut num_vert_origin_y_metrics: u16 = 0;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_VORG {
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
                            let default_vertical_origin = read_16s(
                                data.offset(4 as ::core::ffi::c_int as isize) as *const u8,
                            ) as Pos;
                            let entries = __caryll_allocate_clean(
                                (::core::mem::size_of::<VorgEntry>() as usize)
                                    .wrapping_mul(num_vert_origin_y_metrics as usize),
                                22 as ::core::ffi::c_ulong,
                            ) as *mut VorgEntry;
                            let mut j: u16 = 0 as u16;
                            while (j as ::core::ffi::c_int)
                                < num_vert_origin_y_metrics as ::core::ffi::c_int
                            {
                                (*entries.offset(j as isize)).gid = read_16u(
                                    data.offset(8 as ::core::ffi::c_int as isize).offset(
                                        (4 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                                )
                                    as GlyphId;
                                (*entries.offset(j as isize)).vertical_origin = read_16s(
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
                            return Some(Box::new(VorgTable {
                                num_vert_origin_y_metrics: num_vert_origin_y_metrics as GlyphId,
                                default_vertical_origin,
                                entries,
                            }));
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
    return None;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_vorg(
    table: Option<&VorgTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let table = match table {
        Some(t) => t,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
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
