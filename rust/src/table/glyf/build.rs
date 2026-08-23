#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}



use crate::support::binio::{pos_to_u16};

use crate::support::buffer::{Buffer};
use crate::support::primitives::{GlyphId, ShapeId};


use crate::table::glyf::{MASK_ON_CURVE, RefAnchorStatus, ComponentFlags, PointFlags, ComponentReference, Glyph, Point, GlyfAndLocaBuffers, GlyfTable};
use crate::table::head::{HeadTable};

use crate::support::buffer::{bufclear, buffree, buflen, buflongalign, bufnew, bufwrite16b, bufwrite32b, bufwrite8, bufwrite_buf, bufwrite_bytes};
use crate::support::primitives::{otfcc_to_f2dot14};
use crate::vf::vq::{vq_get_still};
#[derive(Copy, Clone)]
#[repr(C)]
pub union ComponentArg {
    pub pointid: u16,
    pub coord: i16,
}
pub unsafe fn shrink_flags(mut flags: *mut Buffer) -> *mut Buffer {
    if buflen(flags) == 0 {
        return flags;
    }
    let mut shrunk: *mut Buffer = bufnew();
    bufwrite8(
        shrunk,
        *(*flags).data.offset(0 as ::core::ffi::c_int as isize),
    );
    let mut repeating: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: usize = 1 as usize;
    while j < buflen(flags) {
        if *(*flags).data.offset(j as isize) as ::core::ffi::c_int
            == *(*flags).data.offset(j.wrapping_sub(1 as usize) as isize) as ::core::ffi::c_int
        {
            if repeating != 0 && repeating < 0xfe as ::core::ffi::c_int {
                let ref mut fresh0 = *(*shrunk)
                    .data
                    .offset((*shrunk).cursor.wrapping_sub(1 as usize) as isize);
                *fresh0 = (*fresh0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
                repeating += 1 as ::core::ffi::c_int;
            } else if repeating == 0 as ::core::ffi::c_int {
                let ref mut fresh1 = *(*shrunk)
                    .data
                    .offset((*shrunk).cursor.wrapping_sub(1 as usize) as isize);
                *fresh1 |= PointFlags::REPEAT.bits();
                bufwrite8(shrunk, 1 as u8);
                repeating += 1 as ::core::ffi::c_int;
            } else {
                repeating = 0 as ::core::ffi::c_int;
                bufwrite8(shrunk, *(*flags).data.offset(j as isize));
            }
        } else {
            repeating = 0 as ::core::ffi::c_int;
            bufwrite8(shrunk, *(*flags).data.offset(j as isize));
        }
        j = j.wrapping_add(1);
    }
    buffree(flags);
    return shrunk;
}
pub const EPSILON: ::core::ffi::c_double = 1e-5f64;
unsafe fn glyf_build_simple(mut g: *const Glyph, mut gbuf: *mut Buffer) {
    let mut flags: *mut Buffer = bufnew();
    let mut xs: *mut Buffer = bufnew();
    let mut ys: *mut Buffer = bufnew();
    bufwrite16b(gbuf, (*g).contours.len() as u16);
    bufwrite16b(gbuf, pos_to_u16((*g).stat.x_min));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.y_min));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.x_max));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.y_max));
    let mut ptid: ShapeId = 0 as ShapeId;
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < (*g).contours.len() {
        ptid = (ptid as usize).wrapping_add((&(*g).contours)[j as usize].len())
            as ShapeId as ShapeId;
        bufwrite16b(
            gbuf,
            (ptid as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u16,
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
    let mut cx: i32 = 0 as i32;
    let mut cy: i32 = 0 as i32;
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
            let mut px: i32 =
                round(vq_get_still((*p).x.clone())
                    as ::core::ffi::c_double) as i32;
            let mut py: i32 =
                round(vq_get_still((*p).y.clone())
                    as ::core::ffi::c_double) as i32;
            let mut dx: i16 = (px - cx) as i16;
            let mut dy: i16 = (py - cy) as i16;
            if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                flag.insert(PointFlags::SAME_X);
            } else if dx as ::core::ffi::c_int >= -(0xff as ::core::ffi::c_int)
                && dx as ::core::ffi::c_int <= 0xff as ::core::ffi::c_int
            {
                flag.insert(PointFlags::X_SHORT);
                if dx as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    flag.insert(PointFlags::POSITIVE_X);
                    bufwrite8(xs, dx as u8);
                } else {
                    bufwrite8(xs, -(dx as ::core::ffi::c_int) as u8);
                }
            } else {
                bufwrite16b(xs, dx as u16);
            }
            if dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                flag.insert(PointFlags::SAME_Y);
            } else if dy as ::core::ffi::c_int >= -(0xff as ::core::ffi::c_int)
                && dy as ::core::ffi::c_int <= 0xff as ::core::ffi::c_int
            {
                flag.insert(PointFlags::Y_SHORT);
                if dy as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    flag.insert(PointFlags::POSITIVE_Y);
                    bufwrite8(ys, dy as u8);
                } else {
                    bufwrite8(ys, -(dy as ::core::ffi::c_int) as u8);
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
unsafe fn glyf_build_composite(mut g: *const Glyph, mut gbuf: *mut Buffer) {
    bufwrite16b(gbuf, -(1 as ::core::ffi::c_int) as u16);
    bufwrite16b(gbuf, pos_to_u16((*g).stat.x_min));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.y_min));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.x_max));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.y_max));
    let mut rj: ShapeId = 0 as ShapeId;
    while (rj as usize) < (*g).references.len() {
        let r: *const ComponentReference = &(&(*g).references)[rj as usize];
        let mut flags: ComponentFlags =
            if (rj as usize) < (*g).references.len().wrapping_sub(1 as usize) {
                ComponentFlags::MORE_COMPONENTS
            } else if !(*g).instructions.is_empty() {
                ComponentFlags::WE_HAVE_INSTRUCTIONS
            } else {
                ComponentFlags::empty()
            };
        let mut output_anchor: bool = (*r).is_anchored == RefAnchorStatus::AnchorConsolidated;
        let mut arg1: ComponentArg = ComponentArg { pointid: 0 };
        let mut arg2: ComponentArg = ComponentArg { pointid: 0 };
        if output_anchor {
            arg1.pointid = (*r).outer as u16;
            arg2.pointid = (*r).inner as u16;
            if !((arg1.pointid as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int
                && (arg2.pointid as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int)
            {
                flags.insert(ComponentFlags::ARG_1_AND_2_ARE_WORDS);
            }
        } else {
            flags.insert(ComponentFlags::ARGS_ARE_XY_VALUES);
            arg1.coord = vq_get_still((*r).x.clone()) as i16;
            arg2.coord = vq_get_still((*r).y.clone()) as i16;
            if !((arg1.coord as ::core::ffi::c_int) < 128 as ::core::ffi::c_int
                && arg1.coord as ::core::ffi::c_int >= -(128 as ::core::ffi::c_int)
                && (arg2.coord as ::core::ffi::c_int) < 128 as ::core::ffi::c_int
                && arg2.coord as ::core::ffi::c_int >= -(128 as ::core::ffi::c_int))
            {
                flags.insert(ComponentFlags::ARG_1_AND_2_ARE_WORDS);
            }
        }
        if fabs((*r).b as ::core::ffi::c_double) > EPSILON
            || fabs((*r).c as ::core::ffi::c_double) > EPSILON
        {
            flags.insert(ComponentFlags::WE_HAVE_A_TWO_BY_TWO);
        } else if fabs(
            (*r).a as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) > EPSILON
            || fabs(
                (*r).d as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double,
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
            bufwrite16b(gbuf, arg1.pointid);
            bufwrite16b(gbuf, arg2.pointid);
        } else {
            bufwrite8(gbuf, arg1.pointid as u8);
            bufwrite8(gbuf, arg2.pointid as u8);
        }
        if flags.contains(ComponentFlags::WE_HAVE_A_SCALE) {
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u16,
            );
        } else if flags.contains(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE)
        {
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
    mut head: *mut HeadTable,
) -> GlyfAndLocaBuffers {
    let table: *const GlyfTable = table.map_or(::core::ptr::null(), |t| t as *const GlyfTable);
    let mut bufglyf: *mut Buffer = bufnew();
    let mut bufloca: *mut Buffer = bufnew();
    if !table.is_null() && !head.is_null() {
        let mut gbuf: *mut Buffer = bufnew();
        let mut loca: Vec<u32> = vec![0; (*table).len().wrapping_add(1 as usize)];
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
        if (*bufglyf).cursor >= 0x20000 as ::core::ffi::c_int as usize {
            (*head).index_to_loc_format = 1 as i16;
        } else {
            (*head).index_to_loc_format = 0 as i16;
        }
        let mut j_0: u32 = 0 as u32;
        while j_0 as usize <= (*table).len() {
            if (*head).index_to_loc_format != 0 {
                bufwrite32b(bufloca, loca[j_0 as usize]);
            } else {
                bufwrite16b(
                    bufloca,
                    (loca[j_0 as usize] >> 1 as ::core::ffi::c_int) as u16,
                );
            }
            j_0 = j_0.wrapping_add(1);
        }
        buffree(gbuf);
    }
    let mut pair: GlyfAndLocaBuffers = GlyfAndLocaBuffers {
        glyf: bufglyf,
        loca: bufloca,
    };
    return pair;
}
