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

use crate::support::primitives::otfcc_to_f2dot14;
use crate::vf::vq::vq_get_still;
pub fn shrink_flags(flags: Buffer) -> Buffer {
    if flags.len() == 0 {
        return flags;
    }
    let mut shrunk = Buffer::new();
    let flags_data: &Vec<u8> = &flags.data;
    shrunk.write_u8(flags_data[0]);
    let mut repeating: i32 = 0_i32;
    let mut j: usize = 1_usize;
    while j < flags.len() {
        if flags_data[j] as i32
            == flags_data[j.wrapping_sub(1_usize)] as i32
        {
            if repeating != 0 && repeating < 0xfe_i32 {
                let idx = shrunk.cursor.wrapping_sub(1_usize);
                shrunk.data[idx] = shrunk.data[idx].wrapping_add(1);
                repeating += 1_i32;
            } else if repeating == 0_i32 {
                let idx = shrunk.cursor.wrapping_sub(1_usize);
                shrunk.data[idx] |= PointFlags::REPEAT.bits();
                shrunk.write_u8(1_u8);
                repeating += 1_i32;
            } else {
                repeating = 0_i32;
                shrunk.write_u8(flags_data[j]);
            }
        } else {
            repeating = 0_i32;
            shrunk.write_u8(flags_data[j]);
        }
        j = j.wrapping_add(1);
    }
    shrunk
}
pub const EPSILON: ::core::ffi::c_double = 1e-5f64;
unsafe fn glyf_build_simple(g: *const Glyph, gbuf: &mut Buffer) {
    let mut flags = Buffer::new();
    let mut xs = Buffer::new();
    let mut ys = Buffer::new();
    gbuf.write_u16be((*g).contours.len() as u16);
    gbuf.write_u16be(pos_to_u16((*g).stat.x_min));
    gbuf.write_u16be(pos_to_u16((*g).stat.y_min));
    gbuf.write_u16be(pos_to_u16((*g).stat.x_max));
    gbuf.write_u16be(pos_to_u16((*g).stat.y_max));
    let mut ptid: ShapeId = 0 as ShapeId;
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < (*g).contours.len() {
        ptid =
            (ptid as usize).wrapping_add((&(*g).contours)[j as usize].len()) as ShapeId as ShapeId;
        gbuf.write_u16be((ptid as i32 - 1_i32) as u16);
        j = j.wrapping_add(1);
    }
    gbuf.write_u16be((*g).instructions.len() as u16);
    if !(*g).instructions.is_empty() {
        gbuf.write_bytes(&(*g).instructions);
    }
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
                    xs.write_u8(dx as u8);
                } else {
                    xs.write_u8(-(dx as i32) as u8);
                }
            } else {
                xs.write_u16be(dx as u16);
            }
            if dy as i32 == 0_i32 {
                flag.insert(PointFlags::SAME_Y);
            } else if dy as i32 >= -0xff_i32
                && dy as i32 <= 0xff_i32
            {
                flag.insert(PointFlags::Y_SHORT);
                if dy as i32 > 0_i32 {
                    flag.insert(PointFlags::POSITIVE_Y);
                    ys.write_u8(dy as u8);
                } else {
                    ys.write_u8(-(dy as i32) as u8);
                }
            } else {
                ys.write_u16be(dy as u16);
            }
            flags.write_u8(flag.bits());
            cx = px;
            cy = py;
            k = k.wrapping_add(1);
        }
        cj = cj.wrapping_add(1);
    }
    let flags = shrink_flags(flags);
    gbuf.write_buffer(&flags);
    gbuf.write_buffer(&xs);
    gbuf.write_buffer(&ys);
}
unsafe fn glyf_build_composite(g: *const Glyph, gbuf: &mut Buffer) {
    gbuf.write_u16be(-1_i32 as u16);
    gbuf.write_u16be(pos_to_u16((*g).stat.x_min));
    gbuf.write_u16be(pos_to_u16((*g).stat.y_min));
    gbuf.write_u16be(pos_to_u16((*g).stat.x_max));
    gbuf.write_u16be(pos_to_u16((*g).stat.y_max));
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
        gbuf.write_u16be(flags.bits());
        gbuf.write_u16be((*r).glyph.index as u16);
        if flags.contains(ComponentFlags::ARG_1_AND_2_ARE_WORDS) {
            gbuf.write_u16be(arg1);
            gbuf.write_u16be(arg2);
        } else {
            gbuf.write_u8(arg1 as u8);
            gbuf.write_u8(arg2 as u8);
        }
        if flags.contains(ComponentFlags::WE_HAVE_A_SCALE) {
            gbuf.write_u16be(otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u16);
        } else if flags.contains(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE) {
            gbuf.write_u16be(otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u16);
            gbuf.write_u16be(otfcc_to_f2dot14((*r).d as ::core::ffi::c_double) as u16);
        } else if flags.contains(ComponentFlags::WE_HAVE_A_TWO_BY_TWO) {
            gbuf.write_u16be(otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u16);
            gbuf.write_u16be(otfcc_to_f2dot14((*r).b as ::core::ffi::c_double) as u16);
            gbuf.write_u16be(otfcc_to_f2dot14((*r).c as ::core::ffi::c_double) as u16);
            gbuf.write_u16be(otfcc_to_f2dot14((*r).d as ::core::ffi::c_double) as u16);
        }
        rj = rj.wrapping_add(1);
    }
    if !(*g).instructions.is_empty() {
        gbuf.write_u16be((*g).instructions.len() as u16);
        gbuf.write_bytes(&(*g).instructions);
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_glyf(
    table: Option<&GlyfTable>,
    head: *mut HeadTable,
) -> GlyfAndLocaBuffers {
    let table: *const GlyfTable = table.map_or(::core::ptr::null(), |t| t as *const GlyfTable);
    let mut bufglyf = Buffer::new();
    let mut bufloca = Buffer::new();
    if !table.is_null() && !head.is_null() {
        let mut gbuf = Buffer::new();
        let mut loca: Vec<u32> = vec![0; (*table).len().wrapping_add(1_usize)];
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < (*table).len() {
            loca[j as usize] = bufglyf.pos() as u32;
            let g: *const Glyph = (&(*table))[j as usize].as_deref().unwrap() as *const Glyph;
            gbuf.clear();
            if !(*g).contours.is_empty() {
                glyf_build_simple(g, &mut gbuf);
            } else if !(*g).references.is_empty() {
                glyf_build_composite(g, &mut gbuf);
            }
            gbuf.long_align();
            bufglyf.write_buffer(&gbuf);
            j = j.wrapping_add(1);
        }
        loca[(*table).len()] = bufglyf.pos() as u32;
        if bufglyf.pos() >= 0x20000_i32 as usize {
            (*head).index_to_loc_format = 1_i16;
        } else {
            (*head).index_to_loc_format = 0_i16;
        }
        let mut j_0: u32 = 0_u32;
        while j_0 as usize <= (*table).len() {
            if (*head).index_to_loc_format != 0 {
                bufloca.write_u32be(loca[j_0 as usize]);
            } else {
                bufloca.write_u16be((loca[j_0 as usize] >> 1_i32) as u16);
            }
            j_0 = j_0.wrapping_add(1);
        }
    }
    let pair: GlyfAndLocaBuffers = GlyfAndLocaBuffers {
        glyf: bufglyf,
        loca: bufloca,
    };
    return pair;
}
