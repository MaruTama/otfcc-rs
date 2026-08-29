#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::binio::pos_to_u16;

use crate::support::buffer::Buffer;
use crate::support::primitives::{GlyphId, ShapeId};

use crate::table::glyf::{
    ComponentFlags, ComponentReference, GlyfAndLocaBuffers, GlyfTable, Glyph, MASK_ON_CURVE, Point,
    PointFlags, RefAnchorStatus,
};
use crate::table::head::HeadTable;

use crate::support::buffer::{
    bufclear, buffree, buflen, buflongalign, bufnew, bufwrite_buf, bufwrite_bytes, bufwrite8,
    bufwrite16b, bufwrite32b,
};
use crate::support::primitives::otfcc_to_f2dot14;
use crate::vf::vq::vq_get_still;
pub unsafe fn shrink_flags(flags: *mut Buffer) -> *mut Buffer {
    if buflen(flags) == 0 {
        return flags;
    }
    let shrunk: *mut Buffer = bufnew();
    let flags_data: &Vec<u8> = &(*flags).data;
    bufwrite8(shrunk, flags_data[0]);
    let mut repeating: i32 = 0_i32;
    let mut j: usize = 1_usize;
    while j < buflen(flags) {
        if flags_data[j] as i32
            == flags_data[j.wrapping_sub(1_usize)] as i32
        {
            if repeating != 0 && repeating < 0xfe_i32 {
                let shrunk_data: &mut Vec<u8> = &mut (*shrunk).data;
                let fresh0 = &mut shrunk_data[(*shrunk).cursor.wrapping_sub(1_usize)];
                *fresh0 = (*fresh0 as i32 + 1_i32) as u8;
                repeating += 1_i32;
            } else if repeating == 0_i32 {
                let shrunk_data: &mut Vec<u8> = &mut (*shrunk).data;
                let fresh1 = &mut shrunk_data[(*shrunk).cursor.wrapping_sub(1_usize)];
                *fresh1 |= PointFlags::REPEAT.bits();
                bufwrite8(shrunk, 1_u8);
                repeating += 1_i32;
            } else {
                repeating = 0_i32;
                bufwrite8(shrunk, flags_data[j]);
            }
        } else {
            repeating = 0_i32;
            bufwrite8(shrunk, flags_data[j]);
        }
        j = j.wrapping_add(1);
    }
    buffree(flags);
    return shrunk;
}
pub const EPSILON: ::core::ffi::c_double = 1e-5f64;
unsafe fn glyf_build_simple(g: *const Glyph, gbuf: *mut Buffer) {
    let mut flags: *mut Buffer = bufnew();
    let xs: *mut Buffer = bufnew();
    let ys: *mut Buffer = bufnew();
    bufwrite16b(gbuf, (*g).contours.len() as u16);
    bufwrite16b(gbuf, pos_to_u16((*g).stat.x_min));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.y_min));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.x_max));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.y_max));
    let mut ptid: ShapeId = 0 as ShapeId;
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < (*g).contours.len() {
        ptid =
            (ptid as usize).wrapping_add((&(*g).contours)[j as usize].len()) as ShapeId as ShapeId;
        bufwrite16b(
            gbuf,
            (ptid as i32 - 1_i32) as u16,
        );
        j = j.wrapping_add(1);
    }
    bufwrite16b(gbuf, (*g).instructions.len() as u16);
    if !(*g).instructions.is_empty() {
        bufwrite_bytes(gbuf, (*g).instructions.len(), (*g).instructions.as_ptr());
    }
    bufclear(flags);
    bufclear(xs);
    bufclear(ys);
    let mut cx: i32 = 0_i32;
    let mut cy: i32 = 0_i32;
    let mut cj: ShapeId = 0 as ShapeId;
    while (cj as usize) < (*g).contours.len() {
        let mut k: ShapeId = 0 as ShapeId;
        while (k as usize) < (&(*g).contours)[cj as usize].len() {
            let p: *const Point = &(&(*g).contours)[cj as usize][k as usize];
            let mut flag: PointFlags = if (*p).on_curve & MASK_ON_CURVE != 0 {
                PointFlags::ON_CURVE
            } else {
                PointFlags::empty()
            };
            let px: i32 = round(vq_get_still((*p).x.clone()) as ::core::ffi::c_double) as i32;
            let py: i32 = round(vq_get_still((*p).y.clone()) as ::core::ffi::c_double) as i32;
            let dx: i16 = (px - cx) as i16;
            let dy: i16 = (py - cy) as i16;
            if dx as i32 == 0_i32 {
                flag.insert(PointFlags::SAME_X);
            } else if dx as i32 >= -0xff_i32
                && dx as i32 <= 0xff_i32
            {
                flag.insert(PointFlags::X_SHORT);
                if dx as i32 > 0_i32 {
                    flag.insert(PointFlags::POSITIVE_X);
                    bufwrite8(xs, dx as u8);
                } else {
                    bufwrite8(xs, -(dx as i32) as u8);
                }
            } else {
                bufwrite16b(xs, dx as u16);
            }
            if dy as i32 == 0_i32 {
                flag.insert(PointFlags::SAME_Y);
            } else if dy as i32 >= -0xff_i32
                && dy as i32 <= 0xff_i32
            {
                flag.insert(PointFlags::Y_SHORT);
                if dy as i32 > 0_i32 {
                    flag.insert(PointFlags::POSITIVE_Y);
                    bufwrite8(ys, dy as u8);
                } else {
                    bufwrite8(ys, -(dy as i32) as u8);
                }
            } else {
                bufwrite16b(ys, dy as u16);
            }
            bufwrite8(flags, flag.bits());
            cx = px;
            cy = py;
            k = k.wrapping_add(1);
        }
        cj = cj.wrapping_add(1);
    }
    flags = shrink_flags(flags);
    bufwrite_buf(gbuf, flags);
    bufwrite_buf(gbuf, xs);
    bufwrite_buf(gbuf, ys);
    buffree(flags);
    buffree(xs);
    buffree(ys);
}
unsafe fn glyf_build_composite(g: *const Glyph, gbuf: *mut Buffer) {
    bufwrite16b(gbuf, -1_i32 as u16);
    bufwrite16b(gbuf, pos_to_u16((*g).stat.x_min));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.y_min));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.x_max));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.y_max));
    let mut rj: ShapeId = 0 as ShapeId;
    while (rj as usize) < (*g).references.len() {
        let r: *const ComponentReference = &(&(*g).references)[rj as usize];
        let mut flags: ComponentFlags =
            if (rj as usize) < (*g).references.len().wrapping_sub(1_usize) {
                ComponentFlags::MORE_COMPONENTS
            } else if !(*g).instructions.is_empty() {
                ComponentFlags::WE_HAVE_INSTRUCTIONS
            } else {
                ComponentFlags::empty()
            };
        let output_anchor: bool = (*r).is_anchored == RefAnchorStatus::AnchorConsolidated;
        // Was a `union { pointid: u16, coord: i16 }` -- `arg1`/`arg2` are
        // written as whichever type this glyph's arguments actually are,
        // then always read back as `u16` further down (`bufwrite16b`/
        // `bufwrite8`), relying on the union's same-size storage to
        // reinterpret an `i16` coordinate's bits as `u16` for writing.
        // Plain `as u16` casts on the same-width integers do the identical
        // bit-preserving reinterpretation without a union.
        let (arg1, arg2): (u16, u16) = if output_anchor {
            let a1 = (*r).outer as u16;
            let a2 = (*r).inner as u16;
            if !((a1 as i32) < 0x100_i32
                && (a2 as i32) < 0x100_i32)
            {
                flags.insert(ComponentFlags::ARG_1_AND_2_ARE_WORDS);
            }
            (a1, a2)
        } else {
            flags.insert(ComponentFlags::ARGS_ARE_XY_VALUES);
            let c1 = vq_get_still((*r).x.clone()) as i16;
            let c2 = vq_get_still((*r).y.clone()) as i16;
            if !((c1 as i32) < 128_i32
                && c1 as i32 >= -128_i32
                && (c2 as i32) < 128_i32
                && c2 as i32 >= -128_i32)
            {
                flags.insert(ComponentFlags::ARG_1_AND_2_ARE_WORDS);
            }
            (c1 as u16, c2 as u16)
        };
        if fabs((*r).b as ::core::ffi::c_double) > EPSILON
            || fabs((*r).c as ::core::ffi::c_double) > EPSILON
        {
            flags.insert(ComponentFlags::WE_HAVE_A_TWO_BY_TWO);
        } else if fabs(
            (*r).a as ::core::ffi::c_double - 1_i32 as ::core::ffi::c_double,
        ) > EPSILON
            || fabs(
                (*r).d as ::core::ffi::c_double - 1_i32 as ::core::ffi::c_double,
            ) > EPSILON
        {
            if fabs((*r).a as ::core::ffi::c_double - (*r).d as ::core::ffi::c_double) > EPSILON {
                flags.insert(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE);
            } else {
                flags.insert(ComponentFlags::WE_HAVE_A_SCALE);
            }
        }
        if (*r).round_to_grid {
            flags.insert(ComponentFlags::ROUND_XY_TO_GRID);
        }
        if (*r).use_my_metrics {
            flags.insert(ComponentFlags::USE_MY_METRICS);
        }
        flags.insert(ComponentFlags::UNSCALED_COMPONENT_OFFSET);
        bufwrite16b(gbuf, flags.bits());
        bufwrite16b(gbuf, (*r).glyph.index as u16);
        if flags.contains(ComponentFlags::ARG_1_AND_2_ARE_WORDS) {
            bufwrite16b(gbuf, arg1);
            bufwrite16b(gbuf, arg2);
        } else {
            bufwrite8(gbuf, arg1 as u8);
            bufwrite8(gbuf, arg2 as u8);
        }
        if flags.contains(ComponentFlags::WE_HAVE_A_SCALE) {
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u16,
            );
        } else if flags.contains(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE) {
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u16,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).d as ::core::ffi::c_double) as u16,
            );
        } else if flags.contains(ComponentFlags::WE_HAVE_A_TWO_BY_TWO) {
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u16,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).b as ::core::ffi::c_double) as u16,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).c as ::core::ffi::c_double) as u16,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).d as ::core::ffi::c_double) as u16,
            );
        }
        rj = rj.wrapping_add(1);
    }
    if !(*g).instructions.is_empty() {
        bufwrite16b(gbuf, (*g).instructions.len() as u16);
        bufwrite_bytes(gbuf, (*g).instructions.len(), (*g).instructions.as_ptr());
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_glyf(
    table: Option<&GlyfTable>,
    head: *mut HeadTable,
) -> GlyfAndLocaBuffers {
    let table: *const GlyfTable = table.map_or(::core::ptr::null(), |t| t as *const GlyfTable);
    let bufglyf: *mut Buffer = bufnew();
    let bufloca: *mut Buffer = bufnew();
    if !table.is_null() && !head.is_null() {
        let gbuf: *mut Buffer = bufnew();
        let mut loca: Vec<u32> = vec![0; (*table).len().wrapping_add(1_usize)];
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < (*table).len() {
            loca[j as usize] = (*bufglyf).cursor as u32;
            let g: *const Glyph = (&(*table))[j as usize].as_deref().unwrap() as *const Glyph;
            bufclear(gbuf);
            if !(*g).contours.is_empty() {
                glyf_build_simple(g, gbuf);
            } else if !(*g).references.is_empty() {
                glyf_build_composite(g, gbuf);
            }
            buflongalign(gbuf);
            bufwrite_buf(bufglyf, gbuf);
            j = j.wrapping_add(1);
        }
        loca[(*table).len()] = (*bufglyf).cursor as u32;
        if (*bufglyf).cursor >= 0x20000_i32 as usize {
            (*head).index_to_loc_format = 1_i16;
        } else {
            (*head).index_to_loc_format = 0_i16;
        }
        let mut j_0: u32 = 0_u32;
        while j_0 as usize <= (*table).len() {
            if (*head).index_to_loc_format != 0 {
                bufwrite32b(bufloca, loca[j_0 as usize]);
            } else {
                bufwrite16b(
                    bufloca,
                    (loca[j_0 as usize] >> 1_i32) as u16,
                );
            }
            j_0 = j_0.wrapping_add(1);
        }
        buffree(gbuf);
    }
    let pair: GlyfAndLocaBuffers = GlyfAndLocaBuffers {
        glyf: bufglyf,
        loca: bufloca,
    };
    return pair;
}
