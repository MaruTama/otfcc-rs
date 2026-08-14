#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::binio::{pos_to_u16, read_16u, read_16s};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, Length, Pos};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::hhea::{HheaTable};
use crate::table::maxp::{MaxpTable};
use crate::support::buffer::{bufnew, bufwrite16b};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HorizontalMetric {
    pub advance_width: Length,
    pub lsb: Pos,
}
// Both fields are now plain `Vec`s, so `HmtxTable` needs no custom `Drop`
// impl -- ordinary field-by-field drop glue reaches both allocations.
// `HmtxTable` never appears in JSON dump/parse (glyph-level metrics live on
// `Glyph.advance_width`/`.horizontal_origin` instead; this table exists
// purely as an `hmtx`-binary-serialization intermediate, confirmed by grep:
// its only touch points are this file's own read/build functions and
// `otf_writer/stat.rs`'s `stat_hmtx`, which constructs it), so there is no
// JSON-side fallout from this field type change.
#[repr(C)]
pub struct HmtxTable {
    pub metrics: Vec<HorizontalMetric>,
    pub left_side_bearing: Vec<Pos>,
}
pub unsafe extern "C" fn otfcc_read_hmtx(
    packet: Packet,
    mut options: *const Options,
    mut hhea: *mut HheaTable,
    mut maxp: *mut MaxpTable,
) -> Option<Box<HmtxTable>> {
    if hhea.is_null()
        || maxp.is_null()
        || (*hhea).number_of_metrics == 0
        || ((*maxp).num_glyphs as ::core::ffi::c_int) < (*hhea).number_of_metrics as ::core::ffi::c_int
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
            if table.tag == crate::tag::TAG_HMTX {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
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
                            crate::bytesbuild!(b"Table 'hmtx' corrupted.\n"),
                        );
                    } else {
                        let mut metrics: Vec<HorizontalMetric> = Vec::with_capacity(count_a as usize);
                        let mut ia: GlyphId = 0 as GlyphId;
                        while (ia as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
                            let advance_width = read_16u(data.offset(
                                (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as *const u8) as Length;
                            let lsb = read_16s(
                                data.offset(
                                    (ia as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                                )
                                .offset(2 as ::core::ffi::c_int as isize)
                                    as *const u8,
                            ) as Pos;
                            metrics.push(HorizontalMetric { advance_width, lsb });
                            ia = ia.wrapping_add(1);
                        }
                        let mut left_side_bearing: Vec<Pos> = Vec::with_capacity(count_k as usize);
                        let mut ik: GlyphId = 0 as GlyphId;
                        while (ik as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
                            left_side_bearing.push(read_16s(
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
                        return Some(Box::new(HmtxTable { metrics, left_side_bearing }));
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
pub unsafe extern "C" fn otfcc_build_hmtx(
    hmtx: Option<&HmtxTable>,
    mut count_a: GlyphId,
    mut count_k: GlyphId,
    mut _options: *const Options,
) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
    let hmtx = match hmtx {
        Some(h) => h,
        None => return buf,
    };
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
        bufwrite16b(buf, hmtx.metrics[j as usize].advance_width as u16);
        bufwrite16b(buf, pos_to_u16(hmtx.metrics[j as usize].lsb));
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < count_k as ::core::ffi::c_int {
        bufwrite16b(buf, pos_to_u16(hmtx.left_side_bearing[j_0 as usize]));
        j_0 = j_0.wrapping_add(1);
    }
    return buf;
}
