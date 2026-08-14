#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::binio::{pos_to_u16, read_16u, read_16s};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, Length, Pos};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::maxp::{MaxpTable};
use crate::table::vhea::{VheaTable};
use crate::support::buffer::{bufnew, bufwrite16b};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VerticalMetric {
    pub advance_height: Length,
    pub tsb: Pos,
}
// Both fields are now plain `Vec`s, its horizontal-axis mirror `HmtxTable`'s
// own comment explains why there's no JSON-side fallout: this table is a
// pure `vmtx`-binary-serialization intermediate, never touched by dump/
// parse.
#[repr(C)]
pub struct VmtxTable {
    pub metrics: Vec<VerticalMetric>,
    pub top_side_bearing: Vec<Pos>,
}
pub unsafe extern "C" fn otfcc_read_vmtx(
    packet: Packet,
    mut options: *const Options,
    mut vhea: *mut VheaTable,
    mut maxp: *mut MaxpTable,
) -> Option<Box<VmtxTable>> {
    if vhea.is_null()
        || maxp.is_null()
        || (*vhea).num_of_long_ver_metrics as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || ((*maxp).num_glyphs as ::core::ffi::c_int)
            < (*vhea).num_of_long_ver_metrics as ::core::ffi::c_int
    {
        return None;
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
            if table.tag == crate::tag::TAG_VMTX {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
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
                            crate::bytesbuild!(b"Table 'vmtx' corrupted.\n"),
                        );
                    } else {
                        let mut metrics: Vec<VerticalMetric> = Vec::with_capacity(count_a as usize);
                        let mut ia: GlyphId = 0 as GlyphId;
                        while (ia as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
                            let advance_height = read_16u(data.offset(
                                (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as *const u8) as Length;
                            let tsb = read_16s(
                                data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                )
                                .offset(2 as ::core::ffi::c_int as isize)
                                    as *const u8,
                            ) as Pos;
                            metrics.push(VerticalMetric { advance_height, tsb });
                            ia = ia.wrapping_add(1);
                        }
                        let mut top_side_bearing: Vec<Pos> = Vec::with_capacity(count_k as usize);
                        let mut ik: GlyphId = 0 as GlyphId;
                        while (ik as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
                            top_side_bearing.push(read_16s(
                                data.offset(
                                    (count_a as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                .offset(
                                    (ik as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                            ) as Pos);
                            ik = ik.wrapping_add(1);
                        }
                        return Some(Box::new(VmtxTable { metrics, top_side_bearing }));
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
    return None;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_vmtx(
    vmtx: Option<&VmtxTable>,
    mut count_a: GlyphId,
    mut count_k: GlyphId,
    mut _options: *const Options,
) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
    let vmtx = match vmtx {
        Some(v) => v,
        None => return buf,
    };
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
        bufwrite16b(buf, vmtx.metrics[j as usize].advance_height as u16);
        bufwrite16b(buf, pos_to_u16(vmtx.metrics[j as usize].tsb));
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
        bufwrite16b(buf, pos_to_u16(vmtx.top_side_bearing[j_0 as usize]));
        j_0 = j_0.wrapping_add(1);
    }
    return buf;
}
