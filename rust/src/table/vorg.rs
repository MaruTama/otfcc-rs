#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::binio::{pos_to_u16, read_16s, read_16u};
use crate::support::buffer::Buffer;
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::support::options::Options;
use crate::support::primitives::{FontFilePointer, GlyphId, Pos};

#[derive(Copy, Clone)]
pub struct VorgEntry {
    pub gid: GlyphId,
    pub vertical_origin: i16,
}
pub struct VorgTable {
    pub num_vert_origin_y_metrics: GlyphId,
    pub default_vertical_origin: Pos,
    pub entries: Vec<VorgEntry>,
}
// Stage 6-4 "Box化" Box-ified the outer `VorgTable` itself (replacing the
// entire `VorgTableElementInterface` vtable); Stage 7-2-c "inner Vec化"
// finishes the job here: `entries` was the only allocation this struct
// owned, so `Vec<VorgEntry>` plus its own drop glue replaces the manual
// `free`-based `impl Drop` that used to live here. `num_vert_origin_y_metrics`
// is kept as a real field (not collapsed into `entries.len()`, unlike
// `CvtTable.length`): it is read independently at
// `otf_reader/unconsolidate.rs`'s `merge_vmtx` as the loop bound, and always
// equals `entries.len()` by construction at every write site, so keeping it
// is a conservative choice that changes no call site beyond the storage
// mechanism.
pub unsafe fn otfcc_read_vorg(packet: &Packet, options: &Options) -> Option<Box<VorgTable>> {
    let mut num_vert_origin_y_metrics: u16;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_VORG {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    let length: u32 = table.length;
                    if !(length < 8 as u32) {
                        num_vert_origin_y_metrics =
                            read_16u(data.offset(6 as ::core::ffi::c_int as isize) as *const u8);
                        if !(length
                            < (8 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                    * num_vert_origin_y_metrics as ::core::ffi::c_int)
                                as u32)
                        {
                            let default_vertical_origin = read_16s(
                                data.offset(4 as ::core::ffi::c_int as isize) as *const u8,
                            ) as Pos;
                            let mut entries: Vec<VorgEntry> =
                                Vec::with_capacity(num_vert_origin_y_metrics as usize);
                            let mut j: u16 = 0 as u16;
                            while (j as ::core::ffi::c_int)
                                < num_vert_origin_y_metrics as ::core::ffi::c_int
                            {
                                let gid =
                                    read_16u(data.offset(8 as ::core::ffi::c_int as isize).offset(
                                        (4 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8) as GlyphId;
                                let vertical_origin = read_16s(
                                    data.offset(8 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (4 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                entries.push(VorgEntry {
                                    gid,
                                    vertical_origin,
                                });
                                j = j.wrapping_add(1);
                            }
                            return Some(Box::new(VorgTable {
                                num_vert_origin_y_metrics: num_vert_origin_y_metrics as GlyphId,
                                default_vertical_origin,
                                entries,
                            }));
                        }
                    }
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(b"Table 'VORG' corrupted."),
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
pub unsafe fn otfcc_build_vorg(table: Option<&VorgTable>) -> *mut Buffer {
    let table = match table {
        Some(t) => t,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 1 as u16);
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, pos_to_u16((*table).default_vertical_origin));
    bufwrite16b(buf, (*table).num_vert_origin_y_metrics as u16);
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < (*table).num_vert_origin_y_metrics as ::core::ffi::c_int {
        bufwrite16b(buf, (*table).entries[j as usize].gid as u16);
        bufwrite16b(buf, (*table).entries[j as usize].vertical_origin as u16);
        j = j.wrapping_add(1);
    }
    return buf;
}
