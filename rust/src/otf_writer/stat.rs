#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{time, time_t};
unsafe extern "C" {
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_index, otfcc_handle_replace,
};

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};

use crate::font::caryll_font::{Font, FontSubtype};
use crate::support::options::Options;
use crate::support::primitives::{F16Dot16, GlyphId, Length, Pos, Scale, ShapeId};

use crate::table::cff::{CffFontMatrix, CffTable};

use crate::table::ltsh::LtshTable;

use crate::table::vorg::{VorgEntry, VorgTable};

use crate::table::glyf::{
    ComponentReference, Contour, GlyfTable, Glyph, GlyphStat, Point, RefAnchorStatus,
};

use crate::table::head::HeadTable;
use crate::table::hhea::HheaTable;
use crate::table::hmtx::{HmtxTable, HorizontalMetric};
use crate::table::maxp::MaxpTable;
use crate::table::os_2::Os2Table;
use crate::table::vhea::VheaTable;

use crate::table::otl::subtables::chaining::common::chaining_rule_mut;
use crate::table::otl::{
    ChainingSubtable, GsubLigatureEntry, GsubLigatureSubtable, GsubReverseSubtable, Lookup,
    OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE,
    OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_LIGATURE,
    OTL_TYPE_GSUB_REVERSE, OtlTable, Subtable, SubtablePtr, subtable_at,
};

use crate::table::vmtx::{VerticalMetric, VmtxTable};

use crate::font::caryll_font::delete_font_table;
use crate::table::glyf::glyf_component_reference_init;
use crate::vf::vq::VQ;
use crate::vf::vq::{vq_create_still, vq_get_still, vq_is_zero, vq_neutral, vq_replace};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StatStatus {
    NotStarted = 0,
    Doing = 1,
    Completed = 2,
}
pub const POS_MAX: ::core::ffi::c_float = FLT_MAX;
pub unsafe fn stat_single_glyph(
    table: *const GlyfTable,
    gr: *mut ComponentReference,
    stated: *mut StatStatus,
    depth: u8,
    topj: GlyphId,
    options: &Options,
) -> GlyphStat {
    let mut stat: GlyphStat = GlyphStat {
        x_min: 0_i32 as Pos,
        x_max: 0_i32 as Pos,
        y_min: 0_i32 as Pos,
        y_max: 0_i32 as Pos,
        nest_depth: 0_u16,
        n_points: 0_u16,
        n_contours: 0_u16,
        n_composite_points: 0_u16,
        n_composite_contours: 0_u16,
    };
    let j: GlyphId = (*gr).glyph.index;
    if depth as i32 >= 0xff_i32 {
        return stat;
    }
    if *stated.offset(j as isize) == StatStatus::Doing {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"[Stat] Circular glyph reference found in gid ",
                topj as i32,
                b" to gid ",
                j as i32,
                b". The reference will be dropped.\n",
            ),
        );
        *stated.offset(j as isize) = StatStatus::Completed;
        return stat;
    }
    let g: *const Glyph = (&(*table))[j as usize].as_deref().unwrap() as *const Glyph;
    *stated.offset(j as isize) = StatStatus::Doing;
    let mut xmin: Pos = POS_MAX as Pos;
    let mut xmax: Pos = -POS_MAX as Pos;
    let mut ymin: Pos = POS_MAX as Pos;
    let mut ymax: Pos = -POS_MAX as Pos;
    let mut nest_depth: u16 = 0_u16;
    let mut n_points: u16 = 0_u16;
    let mut n_composite_points: u16;
    let mut n_composite_contours: u16;
    for c in 0..(*g).contours.len() as ShapeId {
        let contour: *const Contour = &(&(*g).contours)[c as usize];
        for pj in 0..(*contour).len() as ShapeId {
            let p: *const Point = &(&(*contour))[pj as usize];
            let x: Pos = round(
                vq_get_still((*gr).x.clone()) as ::core::ffi::c_double
                    + (*gr).a as ::core::ffi::c_double
                        * vq_get_still((*p).x.clone()) as ::core::ffi::c_double
                    + (*gr).b as ::core::ffi::c_double
                        * vq_get_still((*p).y.clone()) as ::core::ffi::c_double,
            ) as Pos;
            let y: Pos = round(
                vq_get_still((*gr).y.clone()) as ::core::ffi::c_double
                    + (*gr).c as ::core::ffi::c_double
                        * vq_get_still((*p).x.clone()) as ::core::ffi::c_double
                    + (*gr).d as ::core::ffi::c_double
                        * vq_get_still((*p).y.clone()) as ::core::ffi::c_double,
            ) as Pos;
            if x < xmin {
                xmin = x;
            }
            if x > xmax {
                xmax = x;
            }
            if y < ymin {
                ymin = y;
            }
            if y > ymax {
                ymax = y;
            }
            n_points = (n_points as i32 + 1_i32) as u16;
        }
    }
    n_composite_points = n_points;
    n_composite_contours = (*g).contours.len() as u16;
    for r in 0..(*g).references.len() as ShapeId {
        let mut ref_0: ComponentReference = ComponentReference {
            x: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            y: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            round_to_grid: false,
            use_my_metrics: false,
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: Vec::new(),
            },
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            is_anchored: RefAnchorStatus::Xy,
            inner: 0,
            outer: 0,
        };
        glyf_component_reference_init(&raw mut ref_0);
        let rr: *const ComponentReference = &raw const (&(*g).references)[r as usize];
        otfcc_handle_replace(
            &raw mut ref_0.glyph,
            handle_from_index((*rr).glyph.index) as Handle,
        );
        ref_0.a = (*gr).a * (*rr).a + (*rr).b * (*gr).c;
        ref_0.b = (*rr).a * (*gr).b + (*rr).b * (*gr).d;
        ref_0.c = (*gr).a * (*rr).c + (*gr).c * (*rr).d;
        ref_0.d = (*gr).b * (*rr).c + (*rr).d * (*gr).d;
        vq_replace(
            &raw mut ref_0.x,
            vq_create_still(
                vq_get_still((*rr).x.clone())
                    + (*rr).a as Pos * vq_get_still((*gr).x.clone())
                    + (*rr).b as Pos * vq_get_still((*gr).y.clone()),
            ) as VQ,
        );
        vq_replace(
            &raw mut ref_0.y,
            vq_create_still(
                vq_get_still((*rr).y.clone())
                    + (*rr).c as Pos * vq_get_still((*gr).x.clone())
                    + (*rr).d as Pos * vq_get_still((*gr).y.clone()),
            ) as VQ,
        );
        let thatstat: GlyphStat = stat_single_glyph(
            table,
            &raw mut ref_0,
            stated,
            (depth as i32 + 1_i32) as u8,
            topj,
            options,
        );
        if thatstat.x_min < xmin {
            xmin = thatstat.x_min;
        }
        if thatstat.x_max > xmax {
            xmax = thatstat.x_max;
        }
        if thatstat.y_min < ymin {
            ymin = thatstat.y_min;
        }
        if thatstat.y_max > ymax {
            ymax = thatstat.y_max;
        }
        if thatstat.nest_depth as i32 + 1_i32
            > nest_depth as i32
        {
            nest_depth =
                (thatstat.nest_depth as i32 + 1_i32) as u16;
        }
        n_composite_points = (n_composite_points as i32
            + thatstat.n_composite_points as i32)
            as u16;
        n_composite_contours = (n_composite_contours as i32
            + thatstat.n_composite_contours as i32)
            as u16;
    }
    if xmin > xmax {
        xmax = 0_i32 as Pos;
        xmin = xmax;
    }
    if ymin > ymax {
        ymax = 0_i32 as Pos;
        ymin = ymax;
    }
    stat.x_min = xmin;
    stat.x_max = xmax;
    stat.y_min = ymin;
    stat.y_max = ymax;
    stat.nest_depth = nest_depth;
    stat.n_points = n_points;
    stat.n_contours = (*g).contours.len() as u16;
    stat.n_composite_points = n_composite_points;
    stat.n_composite_contours = n_composite_contours;
    *stated.offset(j as isize) = StatStatus::Completed;
    return stat;
}
pub unsafe fn stat_glyf(font: *mut Font, options: &Options) {
    // Only ever called (from `otfcc_stat_font`) under a `.head.is_some()`
    // guard.
    let head: *mut HeadTable = (*font).head.as_deref_mut().unwrap() as *mut HeadTable;
    // Only ever called (from `otfcc_stat_font`) under a `.glyf.is_some()`
    // guard.
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    // A local `Vec` scratch buffer instead of `__caryll_allocate_clean`/
    // `free` -- `stat_single_glyph` still takes `*mut StatStatus` unchanged
    // (its own body, including the recursive call passing `stated` through
    // unmodified, doesn't need to know its scratch buffer moved).
    let mut stated: Vec<StatStatus> = vec![StatStatus::NotStarted; (*glyf).len()];
    let mut xmin: Pos = 0xffffffff as ::core::ffi::c_uint as Pos;
    let mut xmax: Pos = (0xffffffff as ::core::ffi::c_uint).wrapping_neg() as Pos;
    let mut ymin: Pos = 0xffffffff as ::core::ffi::c_uint as Pos;
    let mut ymax: Pos = (0xffffffff as ::core::ffi::c_uint).wrapping_neg() as Pos;
    for j in 0..(*glyf).len() as GlyphId {
        let mut gr: ComponentReference = ComponentReference {
            x: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            y: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            round_to_grid: false,
            use_my_metrics: false,
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: Vec::new(),
            },
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            is_anchored: RefAnchorStatus::Xy,
            inner: 0,
            outer: 0,
        };
        gr.glyph = handle_from_index(j) as GlyphHandle;
        gr.x = vq_create_still(0_i32 as Pos);
        gr.y = vq_create_still(0_i32 as Pos);
        gr.a = 1_i32 as Scale;
        gr.b = 0_i32 as Scale;
        gr.c = 0_i32 as Scale;
        gr.d = 1_i32 as Scale;
        let thatstat: GlyphStat =
            stat_single_glyph(glyf, &raw mut gr, stated.as_mut_ptr(), 0_u8, j, options);
        (&mut (*glyf))[j as usize].as_mut().unwrap().stat = thatstat;
        if thatstat.x_min < xmin {
            xmin = thatstat.x_min;
        }
        if thatstat.x_max > xmax {
            xmax = thatstat.x_max;
        }
        if thatstat.y_min < ymin {
            ymin = thatstat.y_min;
        }
        if thatstat.y_max > ymax {
            ymax = thatstat.y_max;
        }
    }
    (*head).x_min = xmin as i16;
    (*head).x_max = xmax as i16;
    (*head).y_min = ymin as i16;
    (*head).y_max = ymax as i16;
}
pub unsafe fn stat_maxp(font: *mut Font) {
    // Only ever called (from `otfcc_stat_font`) under a `.maxp.is_some()`
    // guard.
    let maxp: *mut MaxpTable = (*font).maxp.as_deref_mut().unwrap() as *mut MaxpTable;
    let mut nest_depth: u16 = 0_u16;
    let mut n_points: u16 = 0_u16;
    let mut n_contours: u16 = 0_u16;
    let mut n_components: u16 = 0_u16;
    let mut n_composite_points: u16 = 0_u16;
    let mut n_composite_contours: u16 = 0_u16;
    let mut inst_size: u16 = 0_u16;
    // Only ever called (from `otfcc_stat_font`) under a `.glyf.is_some()`
    // guard.
    let glyf: *const GlyfTable = (*font).glyf.as_ref().unwrap() as *const GlyfTable;
    for j in 0..(*glyf).len() as GlyphId {
        let g: *const Glyph = (&(*glyf))[j as usize].as_deref().unwrap() as *const Glyph;
        if (*g).contours.len() > 0_usize {
            if (*g).stat.n_points as i32 > n_points as i32 {
                n_points = (*g).stat.n_points;
            }
            if (*g).stat.n_contours as i32 > n_contours as i32 {
                n_contours = (*g).stat.n_contours;
            }
        } else if (*g).references.len() > 0_usize {
            if (*g).stat.n_composite_points as i32
                > n_composite_points as i32
            {
                n_composite_points = (*g).stat.n_composite_points;
            }
            if (*g).stat.n_composite_contours as i32
                > n_composite_contours as i32
            {
                n_composite_contours = (*g).stat.n_composite_contours;
            }
            if (*g).stat.nest_depth as i32 > nest_depth as i32 {
                nest_depth = (*g).stat.nest_depth;
            }
            if (*g).references.len() > n_components as usize {
                n_components = (*g).references.len() as u16;
            }
        }
        if (*g).instructions.len() as i32 > inst_size as i32 {
            inst_size = (*g).instructions.len() as u16;
        }
    }
    (*maxp).max_points = n_points;
    (*maxp).max_contours = n_contours;
    (*maxp).max_composite_points = n_composite_points;
    (*maxp).max_composite_contours = n_composite_contours;
    (*maxp).max_component_depth = nest_depth;
    (*maxp).max_component_elements = n_components;
    (*maxp).max_size_of_instructions = inst_size;
}
unsafe fn stat_hmtx(font: *mut Font) {
    if (*font).glyf.is_none() {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    // Only ever called (from `otfcc_stat_font`) under a `.hhea.is_some()`
    // guard; `.head` is set unconditionally by the pipeline before this
    // point (used below to update `.flags`).
    let hhea: *mut HheaTable = (*font).hhea.as_deref_mut().unwrap() as *mut HheaTable;
    let head: *mut HeadTable = (*font)
        .head
        .as_deref_mut()
        .map_or(::core::ptr::null_mut(), |h| h as *mut HeadTable);
    let mut count_a: GlyphId = (*glyf).len() as GlyphId;
    let mut count_k: GlyphId = 0 as GlyphId;
    let mut lsb_at_x_0: bool = true;
    if (*font).subtype != FontSubtype::Cff {
        while count_a as i32 > 2_i32
            && vq_get_still(
                (&(*glyf))[(count_a as i32 - 1_i32) as usize]
                    .as_deref()
                    .unwrap()
                    .advance_width
                    .clone(),
            ) == vq_get_still(
                (&(*glyf))[(count_a as i32 - 2_i32) as usize]
                    .as_deref()
                    .unwrap()
                    .advance_width
                    .clone(),
            )
        {
            count_a = count_a.wrapping_sub(1);
        }
        count_k = (*glyf).len().wrapping_sub(count_a as usize) as GlyphId;
    }
    // Both arrays fill sequentially within the one loop below (`j < count_a`
    // covers `metrics`, the rest covers `left_side_bearing` in order), so a
    // `Vec` + `.push()` per branch reproduces the same content in the same
    // order as the old pre-sized, index-written arrays.
    let mut metrics: Vec<HorizontalMetric> = Vec::with_capacity(count_a as usize);
    let mut left_side_bearing: Vec<Pos> = Vec::with_capacity(count_k as usize);
    let mut min_lsb: Pos = 0x7fff_i32 as Pos;
    let mut min_rsb: Pos = 0x7fff_i32 as Pos;
    let mut max_extent: Pos = -0x8000_i32 as Pos;
    let mut max_width: Length = 0_i32 as Length;
    for j in 0..(*glyf).len() as GlyphId {
        let g: *mut Glyph = &raw mut **(&mut (*glyf))[j as usize].as_mut().unwrap();
        if vq_is_zero((*g).horizontal_origin.clone(), 1.0f64 / 1000.0f64) {
            vq_replace(&raw mut (*g).horizontal_origin, (vq_neutral)() as VQ);
        } else {
            lsb_at_x_0 = false;
        }
        let hori: Pos = vq_get_still((*g).horizontal_origin.clone()) as Pos;
        let advw: Pos = vq_get_still((*g).advance_width.clone()) as Pos;
        let lsb: Pos = (*g).stat.x_min - hori;
        let rsb: Pos = advw + hori - (*g).stat.x_max;
        if (j as i32) < count_a as i32 {
            metrics.push(HorizontalMetric {
                advance_width: advw as Length,
                lsb,
            });
        } else {
            left_side_bearing.push(lsb);
        }
        if advw > max_width {
            max_width = advw as Length;
        }
        if lsb < min_lsb {
            min_lsb = lsb;
        }
        if rsb < min_rsb {
            min_rsb = rsb;
        }
        if (*g).stat.x_max - hori > max_extent {
            max_extent = (*g).stat.x_max - hori;
        }
    }
    (*hhea).number_of_metrics = count_a as u16;
    (*hhea).min_left_side_bearing = min_lsb as i16;
    (*hhea).min_right_side_bearing = min_rsb as i16;
    (*hhea).x_max_extent = max_extent as i16;
    (*hhea).advance_width_max = max_width as u16;
    (*font).hmtx = Some(Box::new(HmtxTable {
        metrics,
        left_side_bearing,
    }));
    (*head).flags = ((*head).flags as i32 & !0x2_i32
        | (if lsb_at_x_0 {
            0x2_i32
        } else {
            0_i32
        })) as u16;
}
unsafe fn stat_vmtx(font: *mut Font, options: &Options) {
    if (*font).glyf.is_none() {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    // Only ever called (from `otfcc_stat_font`) under a `.vhea.is_some()`
    // guard.
    let vhea: *mut VheaTable = (*font).vhea.as_deref_mut().unwrap() as *mut VheaTable;
    let mut count_a: GlyphId = (*glyf).len() as GlyphId;
    let mut count_k: GlyphId = 0 as GlyphId;
    if !((*font).subtype == FontSubtype::Cff && !options.cff_short_vmtx) {
        while count_a as i32 > 2_i32
            && vq_get_still(
                (&(*glyf))[(count_a as i32 - 1_i32) as usize]
                    .as_deref()
                    .unwrap()
                    .advance_height
                    .clone(),
            ) == vq_get_still(
                (&(*glyf))[(count_a as i32 - 2_i32) as usize]
                    .as_deref()
                    .unwrap()
                    .advance_height
                    .clone(),
            )
        {
            count_a = count_a.wrapping_sub(1);
        }
        count_k = (*glyf).len().wrapping_sub(count_a as usize) as GlyphId;
    }
    // Same "Vec absorbs both sequential halves of the loop" shape as
    // `stat_hmtx`'s `metrics`/`left_side_bearing`.
    let mut metrics: Vec<VerticalMetric> = Vec::with_capacity(count_a as usize);
    let mut top_side_bearing: Vec<Pos> = Vec::with_capacity(count_k as usize);
    let mut min_tsb: Pos = 0x7fff_i32 as Pos;
    let mut min_bsb: Pos = 0x7fff_i32 as Pos;
    let mut max_extent: Pos = -0x8000_i32 as Pos;
    let mut max_height: Length = 0_i32 as Length;
    for j in 0..(*glyf).len() as GlyphId {
        let g: *const Glyph = (&(*glyf))[j as usize].as_deref().unwrap() as *const Glyph;
        let vori: Pos = vq_get_still((*g).vertical_origin.clone()) as Pos;
        let advh: Pos = vq_get_still((*g).advance_height.clone()) as Pos;
        let tsb: Pos = vori - (*g).stat.y_max;
        let bsb: Pos = (*g).stat.y_min - vori + advh;
        if (j as i32) < count_a as i32 {
            metrics.push(VerticalMetric {
                advance_height: advh as Length,
                tsb,
            });
        } else {
            top_side_bearing.push(tsb);
        }
        if advh > max_height {
            max_height = advh as Length;
        }
        if tsb < min_tsb {
            min_tsb = tsb;
        }
        if bsb < min_bsb {
            min_bsb = bsb;
        }
        if vori - (*g).stat.y_min > max_extent {
            max_extent = vori - (*g).stat.y_min;
        }
    }
    (*vhea).num_of_long_ver_metrics = count_a as u16;
    (*vhea).min_top = min_tsb as i16;
    (*vhea).min_bottom = min_bsb as i16;
    (*vhea).y_max_extent = max_extent as i16;
    (*vhea).advance_height_max = max_height as i16;
    (*font).vmtx = Some(Box::new(VmtxTable {
        metrics,
        top_side_bearing,
    }));
}
unsafe fn stat_os_2_unicode_ranges(font: *mut Font, options: &Options) {
    let os_2: *mut Os2Table = (*font).os_2.as_deref_mut().unwrap() as *mut Os2Table;
    let mut u1: u32 = 0_u32;
    let mut u2: u32 = 0_u32;
    let mut u3: u32 = 0_u32;
    let mut u4: u32 = 0_u32;
    let mut min_unicode: i32 = 0xffff_i32;
    let mut max_unicode: i32 = 0_i32;
    for (&u, _) in (*font).cmap.as_ref().unwrap().unicodes.iter() {
        if u < min_unicode {
            min_unicode = u;
        }
        if u > max_unicode {
            max_unicode = u;
        }
        if (0_i32..=0x7f_i32).contains(&u) {
            u1 |= (1_i32 << 0_i32) as u32;
        }
        if (0x80_i32..=0xff_i32).contains(&u) {
            u1 |= (1_i32 << 1_i32) as u32;
        }
        if (0x100_i32..=0x17f_i32).contains(&u) {
            u1 |= (1_i32 << 2_i32) as u32;
        }
        if (0x180_i32..=0x24f_i32).contains(&u) {
            u1 |= (1_i32 << 3_i32) as u32;
        }
        if (0x250_i32..=0x2af_i32).contains(&u)
            || (0x1d00_i32..=0x1d7f_i32).contains(&u)
            || (0x1d80_i32..=0x1dbf_i32).contains(&u)
        {
            u1 |= (1_i32 << 4_i32) as u32;
        }
        if (0x2b0_i32..=0x2ff_i32).contains(&u)
            || (0xa700_i32..=0xa71f_i32).contains(&u)
        {
            u1 |= (1_i32 << 5_i32) as u32;
        }
        if (0x300_i32..=0x36f_i32).contains(&u)
            || (0x1dc0_i32..=0x1dff_i32).contains(&u)
        {
            u1 |= (1_i32 << 6_i32) as u32;
        }
        if (0x370_i32..=0x3ff_i32).contains(&u) {
            u1 |= (1_i32 << 7_i32) as u32;
        }
        if (0x2c80_i32..=0x2cff_i32).contains(&u) {
            u1 |= (1_i32 << 8_i32) as u32;
        }
        if (0x400_i32..=0x4ff_i32).contains(&u)
            || (0x500_i32..=0x52f_i32).contains(&u)
            || (0x2de0_i32..=0x2dff_i32).contains(&u)
            || (0xa640_i32..=0xa69f_i32).contains(&u)
        {
            u1 |= (1_i32 << 9_i32) as u32;
        }
        if (0x530_i32..=0x58f_i32).contains(&u) {
            u1 |= (1_i32 << 10_i32) as u32;
        }
        if (0x590_i32..=0x5ff_i32).contains(&u) {
            u1 |= (1_i32 << 11_i32) as u32;
        }
        if (0xa500_i32..=0xa63f_i32).contains(&u) {
            u1 |= (1_i32 << 12_i32) as u32;
        }
        if (0x600_i32..=0x6ff_i32).contains(&u)
            || (0x750_i32..=0x77f_i32).contains(&u)
        {
            u1 |= (1_i32 << 13_i32) as u32;
        }
        if (0x7c0_i32..=0x7ff_i32).contains(&u) {
            u1 |= (1_i32 << 14_i32) as u32;
        }
        if (0x900_i32..=0x97f_i32).contains(&u) {
            u1 |= (1_i32 << 15_i32) as u32;
        }
        if (0x980_i32..=0x9ff_i32).contains(&u) {
            u1 |= (1_i32 << 16_i32) as u32;
        }
        if (0xa00_i32..=0xa7f_i32).contains(&u) {
            u1 |= (1_i32 << 17_i32) as u32;
        }
        if (0xa80_i32..=0xaff_i32).contains(&u) {
            u1 |= (1_i32 << 18_i32) as u32;
        }
        if (0xb00_i32..=0xb7f_i32).contains(&u) {
            u1 |= (1_i32 << 19_i32) as u32;
        }
        if (0xb80_i32..=0xbff_i32).contains(&u) {
            u1 |= (1_i32 << 20_i32) as u32;
        }
        if (0xc00_i32..=0xc7f_i32).contains(&u) {
            u1 |= (1_i32 << 21_i32) as u32;
        }
        if (0xc80_i32..=0xcff_i32).contains(&u) {
            u1 |= (1_i32 << 22_i32) as u32;
        }
        if (0xd00_i32..=0xd7f_i32).contains(&u) {
            u1 |= (1_i32 << 23_i32) as u32;
        }
        if (0xe00_i32..=0xe7f_i32).contains(&u) {
            u1 |= (1_i32 << 24_i32) as u32;
        }
        if (0xe80_i32..=0xeff_i32).contains(&u) {
            u1 |= (1_i32 << 25_i32) as u32;
        }
        if (0x10a0_i32..=0x10ff_i32).contains(&u)
            || (0x2d00_i32..=0x2d2f_i32).contains(&u)
        {
            u1 |= (1_i32 << 26_i32) as u32;
        }
        if (0x1b00_i32..=0x1b7f_i32).contains(&u) {
            u1 |= (1_i32 << 27_i32) as u32;
        }
        if (0x1100_i32..=0x11ff_i32).contains(&u) {
            u1 |= (1_i32 << 28_i32) as u32;
        }
        if (0x1e00_i32..=0x1eff_i32).contains(&u)
            || (0x2c60_i32..=0x2c7f_i32).contains(&u)
            || (0xa720_i32..=0xa7ff_i32).contains(&u)
        {
            u1 |= (1_i32 << 29_i32) as u32;
        }
        if (0x1f00_i32..=0x1fff_i32).contains(&u) {
            u1 |= (1_i32 << 30_i32) as u32;
        }
        if (0x2000_i32..=0x206f_i32).contains(&u)
            || (0x2e00_i32..=0x2e7f_i32).contains(&u)
        {
            u1 |= (1_i32 << 31_i32) as u32;
        }
        if (0x2070_i32..=0x209f_i32).contains(&u) {
            u2 |= (1_i32 << 0_i32) as u32;
        }
        if (0x20a0_i32..=0x20cf_i32).contains(&u) {
            u2 |= (1_i32 << 1_i32) as u32;
        }
        if (0x20d0_i32..=0x20ff_i32).contains(&u) {
            u2 |= (1_i32 << 2_i32) as u32;
        }
        if (0x2100_i32..=0x214f_i32).contains(&u) {
            u2 |= (1_i32 << 3_i32) as u32;
        }
        if (0x2150_i32..=0x218f_i32).contains(&u) {
            u2 |= (1_i32 << 4_i32) as u32;
        }
        if (0x2190_i32..=0x21ff_i32).contains(&u)
            || (0x27f0_i32..=0x27ff_i32).contains(&u)
            || (0x2900_i32..=0x297f_i32).contains(&u)
            || (0x2b00_i32..=0x2bff_i32).contains(&u)
        {
            u2 |= (1_i32 << 5_i32) as u32;
        }
        if (0x2200_i32..=0x22ff_i32).contains(&u)
            || (0x2a00_i32..=0x2aff_i32).contains(&u)
            || (0x27c0_i32..=0x27ef_i32).contains(&u)
            || (0x2980_i32..=0x29ff_i32).contains(&u)
        {
            u2 |= (1_i32 << 6_i32) as u32;
        }
        if (0x2300_i32..=0x23ff_i32).contains(&u) {
            u2 |= (1_i32 << 7_i32) as u32;
        }
        if (0x2400_i32..=0x243f_i32).contains(&u) {
            u2 |= (1_i32 << 8_i32) as u32;
        }
        if (0x2440_i32..=0x245f_i32).contains(&u) {
            u2 |= (1_i32 << 9_i32) as u32;
        }
        if (0x2460_i32..=0x24ff_i32).contains(&u) {
            u2 |= (1_i32 << 10_i32) as u32;
        }
        if (0x2500_i32..=0x257f_i32).contains(&u) {
            u2 |= (1_i32 << 11_i32) as u32;
        }
        if (0x2580_i32..=0x259f_i32).contains(&u) {
            u2 |= (1_i32 << 12_i32) as u32;
        }
        if (0x25a0_i32..=0x25ff_i32).contains(&u) {
            u2 |= (1_i32 << 13_i32) as u32;
        }
        if (0x2600_i32..=0x26ff_i32).contains(&u) {
            u2 |= (1_i32 << 14_i32) as u32;
        }
        if (0x2700_i32..=0x27bf_i32).contains(&u) {
            u2 |= (1_i32 << 15_i32) as u32;
        }
        if (0x3000_i32..=0x303f_i32).contains(&u) {
            u2 |= (1_i32 << 16_i32) as u32;
        }
        if (0x3040_i32..=0x309f_i32).contains(&u) {
            u2 |= (1_i32 << 17_i32) as u32;
        }
        if (0x30a0_i32..=0x30ff_i32).contains(&u)
            || (0x31f0_i32..=0x31ff_i32).contains(&u)
        {
            u2 |= (1_i32 << 18_i32) as u32;
        }
        if (0x3100_i32..=0x312f_i32).contains(&u)
            || (0x31a0_i32..=0x31bf_i32).contains(&u)
        {
            u2 |= (1_i32 << 19_i32) as u32;
        }
        if (0x3130_i32..=0x318f_i32).contains(&u) {
            u2 |= (1_i32 << 20_i32) as u32;
        }
        if (0xa840_i32..=0xa87f_i32).contains(&u) {
            u2 |= (1_i32 << 21_i32) as u32;
        }
        if (0x3200_i32..=0x32ff_i32).contains(&u) {
            u2 |= (1_i32 << 22_i32) as u32;
        }
        if (0x3300_i32..=0x33ff_i32).contains(&u) {
            u2 |= (1_i32 << 23_i32) as u32;
        }
        if (0xac00_i32..=0xd7af_i32).contains(&u) {
            u2 |= (1_i32 << 24_i32) as u32;
        }
        if (0xd800_i32..=0xdfff_i32).contains(&u)
            || u > 0xffff_i32
        {
            u2 |= (1_i32 << 25_i32) as u32;
        }
        if (0x10900_i32..=0x1091f_i32).contains(&u) {
            u2 |= (1_i32 << 26_i32) as u32;
        }
        if (0x4e00_i32..=0x9fff_i32).contains(&u)
            || (0x2e80_i32..=0x2eff_i32).contains(&u)
            || (0x2f00_i32..=0x2fdf_i32).contains(&u)
            || (0x2ff0_i32..=0x2fff_i32).contains(&u)
            || (0x3400_i32..=0x4dbf_i32).contains(&u)
            || (0x20000_i32..=0x2f7ff_i32).contains(&u)
            || (0x3190_i32..=0x319f_i32).contains(&u)
        {
            u2 |= (1_i32 << 27_i32) as u32;
        }
        if (0xe000_i32..=0xf8ff_i32).contains(&u) {
            u2 |= (1_i32 << 28_i32) as u32;
        }
        if (0x31c0_i32..=0x31ef_i32).contains(&u)
            || (0xf900_i32..=0xfaff_i32).contains(&u)
            || (0x2f800_i32..=0x2fa1f_i32).contains(&u)
        {
            u2 |= (1_i32 << 29_i32) as u32;
        }
        if (0xfb00_i32..=0xfb4f_i32).contains(&u) {
            u2 |= (1_i32 << 30_i32) as u32;
        }
        if (0xfb50_i32..=0xfdff_i32).contains(&u) {
            u2 |= (1_i32 << 31_i32) as u32;
        }
        if (0xfe20_i32..=0xfe2f_i32).contains(&u) {
            u3 |= (1_i32 << 0_i32) as u32;
        }
        if (0xfe10_i32..=0xfe1f_i32).contains(&u)
            || (0xfe30_i32..=0xfe4f_i32).contains(&u)
        {
            u3 |= (1_i32 << 1_i32) as u32;
        }
        if (0xfe50_i32..=0xfe6f_i32).contains(&u) {
            u3 |= (1_i32 << 2_i32) as u32;
        }
        if (0xfe70_i32..=0xfeff_i32).contains(&u) {
            u3 |= (1_i32 << 3_i32) as u32;
        }
        if (0xff00_i32..=0xffef_i32).contains(&u) {
            u3 |= (1_i32 << 4_i32) as u32;
        }
        if (0xfff0_i32..=0xffff_i32).contains(&u) {
            u3 |= (1_i32 << 5_i32) as u32;
        }
        if (0xf00_i32..=0xfff_i32).contains(&u) {
            u3 |= (1_i32 << 6_i32) as u32;
        }
        if (0x700_i32..=0x74f_i32).contains(&u) {
            u3 |= (1_i32 << 7_i32) as u32;
        }
        if (0x780_i32..=0x7bf_i32).contains(&u) {
            u3 |= (1_i32 << 8_i32) as u32;
        }
        if (0xd80_i32..=0xdff_i32).contains(&u) {
            u3 |= (1_i32 << 9_i32) as u32;
        }
        if (0x1000_i32..=0x109f_i32).contains(&u) {
            u3 |= (1_i32 << 10_i32) as u32;
        }
        if (0x1200_i32..=0x137f_i32).contains(&u)
            || (0x1380_i32..=0x139f_i32).contains(&u)
            || (0x2d80_i32..=0x2ddf_i32).contains(&u)
        {
            u3 |= (1_i32 << 11_i32) as u32;
        }
        if (0x13a0_i32..=0x13ff_i32).contains(&u) {
            u3 |= (1_i32 << 12_i32) as u32;
        }
        if (0x1400_i32..=0x167f_i32).contains(&u) {
            u3 |= (1_i32 << 13_i32) as u32;
        }
        if (0x1680_i32..=0x169f_i32).contains(&u) {
            u3 |= (1_i32 << 14_i32) as u32;
        }
        if (0x16a0_i32..=0x16ff_i32).contains(&u) {
            u3 |= (1_i32 << 15_i32) as u32;
        }
        if (0x1780_i32..=0x17ff_i32).contains(&u)
            || (0x19e0_i32..=0x19ff_i32).contains(&u)
        {
            u3 |= (1_i32 << 16_i32) as u32;
        }
        if (0x1800_i32..=0x18af_i32).contains(&u) {
            u3 |= (1_i32 << 17_i32) as u32;
        }
        if (0x2800_i32..=0x28ff_i32).contains(&u) {
            u3 |= (1_i32 << 18_i32) as u32;
        }
        if (0xa000_i32..=0xa48f_i32).contains(&u)
            || (0xa490_i32..=0xa4cf_i32).contains(&u)
        {
            u3 |= (1_i32 << 19_i32) as u32;
        }
        if (0x1700_i32..=0x171f_i32).contains(&u)
            || (0x1720_i32..=0x173f_i32).contains(&u)
            || (0x1740_i32..=0x175f_i32).contains(&u)
            || (0x1760_i32..=0x177f_i32).contains(&u)
        {
            u3 |= (1_i32 << 20_i32) as u32;
        }
        if (0x10300_i32..=0x1032f_i32).contains(&u) {
            u3 |= (1_i32 << 21_i32) as u32;
        }
        if (0x10330_i32..=0x1034f_i32).contains(&u) {
            u3 |= (1_i32 << 22_i32) as u32;
        }
        if (0x10400_i32..=0x1044f_i32).contains(&u) {
            u3 |= (1_i32 << 23_i32) as u32;
        }
        if (0x1d000_i32..=0x1d0ff_i32).contains(&u)
            || (0x1d100_i32..=0x1d1ff_i32).contains(&u)
            || (0x1d200_i32..=0x1d24f_i32).contains(&u)
        {
            u3 |= (1_i32 << 24_i32) as u32;
        }
        if (0x1d400_i32..=0x1d7ff_i32).contains(&u) {
            u3 |= (1_i32 << 25_i32) as u32;
        }
        if (0xff000_i32..=0xffffd_i32).contains(&u)
            || (0x100000_i32..=0x10fffd_i32).contains(&u)
        {
            u3 |= (1_i32 << 26_i32) as u32;
        }
        if (0xfe00_i32..=0xfe0f_i32).contains(&u)
            || (0xe0100_i32..=0xe01ef_i32).contains(&u)
        {
            u3 |= (1_i32 << 27_i32) as u32;
        }
        if (0xe0000_i32..=0xe007f_i32).contains(&u) {
            u3 |= (1_i32 << 28_i32) as u32;
        }
        if (0x1900_i32..=0x194f_i32).contains(&u) {
            u3 |= (1_i32 << 29_i32) as u32;
        }
        if (0x1950_i32..=0x197f_i32).contains(&u) {
            u3 |= (1_i32 << 30_i32) as u32;
        }
        if (0x1980_i32..=0x19df_i32).contains(&u) {
            u3 |= (1_i32 << 31_i32) as u32;
        }
        if (0x1a00_i32..=0x1a1f_i32).contains(&u) {
            u4 |= (1_i32 << 0_i32) as u32;
        }
        if (0x2c00_i32..=0x2c5f_i32).contains(&u) {
            u4 |= (1_i32 << 1_i32) as u32;
        }
        if (0x2d30_i32..=0x2d7f_i32).contains(&u) {
            u4 |= (1_i32 << 2_i32) as u32;
        }
        if (0x4dc0_i32..=0x4dff_i32).contains(&u) {
            u4 |= (1_i32 << 3_i32) as u32;
        }
        if (0xa800_i32..=0xa82f_i32).contains(&u) {
            u4 |= (1_i32 << 4_i32) as u32;
        }
        if (0x10000_i32..=0x1007f_i32).contains(&u)
            || (0x10080_i32..=0x100ff_i32).contains(&u)
            || (0x10100_i32..=0x1013f_i32).contains(&u)
        {
            u4 |= (1_i32 << 5_i32) as u32;
        }
        if (0x10140_i32..=0x1018f_i32).contains(&u) {
            u4 |= (1_i32 << 6_i32) as u32;
        }
        if (0x10380_i32..=0x1039f_i32).contains(&u) {
            u4 |= (1_i32 << 7_i32) as u32;
        }
        if (0x103a0_i32..=0x103df_i32).contains(&u) {
            u4 |= (1_i32 << 8_i32) as u32;
        }
        if (0x10450_i32..=0x1047f_i32).contains(&u) {
            u4 |= (1_i32 << 9_i32) as u32;
        }
        if (0x10480_i32..=0x104af_i32).contains(&u) {
            u4 |= (1_i32 << 10_i32) as u32;
        }
        if (0x10800_i32..=0x1083f_i32).contains(&u) {
            u4 |= (1_i32 << 11_i32) as u32;
        }
        if (0x10a00_i32..=0x10a5f_i32).contains(&u) {
            u4 |= (1_i32 << 12_i32) as u32;
        }
        if (0x1d300_i32..=0x1d35f_i32).contains(&u) {
            u4 |= (1_i32 << 13_i32) as u32;
        }
        if (0x12000_i32..=0x123ff_i32).contains(&u)
            || (0x12400_i32..=0x1247f_i32).contains(&u)
        {
            u4 |= (1_i32 << 14_i32) as u32;
        }
        if (0x1d360_i32..=0x1d37f_i32).contains(&u) {
            u4 |= (1_i32 << 15_i32) as u32;
        }
        if (0x1b80_i32..=0x1bbf_i32).contains(&u) {
            u4 |= (1_i32 << 16_i32) as u32;
        }
        if (0x1c00_i32..=0x1c4f_i32).contains(&u) {
            u4 |= (1_i32 << 17_i32) as u32;
        }
        if (0x1c50_i32..=0x1c7f_i32).contains(&u) {
            u4 |= (1_i32 << 18_i32) as u32;
        }
        if (0xa880_i32..=0xa8df_i32).contains(&u) {
            u4 |= (1_i32 << 19_i32) as u32;
        }
        if (0xa900_i32..=0xa92f_i32).contains(&u) {
            u4 |= (1_i32 << 20_i32) as u32;
        }
        if (0xa930_i32..=0xa95f_i32).contains(&u) {
            u4 |= (1_i32 << 21_i32) as u32;
        }
        if (0xaa00_i32..=0xaa5f_i32).contains(&u) {
            u4 |= (1_i32 << 22_i32) as u32;
        }
        if (0x10190_i32..=0x101cf_i32).contains(&u) {
            u4 |= (1_i32 << 23_i32) as u32;
        }
        if (0x101d0_i32..=0x101ff_i32).contains(&u) {
            u4 |= (1_i32 << 24_i32) as u32;
        }
        if (0x102a0_i32..=0x102df_i32).contains(&u)
            || (0x10280_i32..=0x1029f_i32).contains(&u)
            || (0x10920_i32..=0x1093f_i32).contains(&u)
        {
            u4 |= (1_i32 << 25_i32) as u32;
        }
        if (0x1f030_i32..=0x1f09f_i32).contains(&u)
            || (0x1f000_i32..=0x1f02f_i32).contains(&u)
        {
            u4 |= (1_i32 << 26_i32) as u32;
        }
    }
    if !options.keep_unicode_ranges {
        (*os_2).ul_unicode_range1 = u1;
        (*os_2).ul_unicode_range2 = u2;
        (*os_2).ul_unicode_range3 = u3;
        (*os_2).ul_unicode_range4 = u4;
    }
    if min_unicode < 0x10000_i32 {
        (*os_2).us_first_char_index = min_unicode as u16;
    } else {
        (*os_2).us_first_char_index = 0xffff_u16;
    }
    if max_unicode < 0x10000_i32 {
        (*os_2).us_last_char_index = max_unicode as u16;
    } else {
        (*os_2).us_last_char_index = 0xffff_u16;
    };
}
unsafe fn stat_os_2_average_width(font: *mut Font, options: &Options) {
    if options.keep_average_char_width {
        return;
    }
    let os_2: *mut Os2Table = (*font).os_2.as_deref_mut().unwrap() as *mut Os2Table;
    // Only ever called (from `otfcc_stat_font`, via `stat_os_2`) under a
    // `.glyf.is_some()` guard.
    let glyf: *const GlyfTable = (*font).glyf.as_ref().unwrap() as *const GlyfTable;
    let mut total_width: u32 = 0_u32;
    for j in 0..(*glyf).len() as GlyphId {
        let adw: Pos = vq_get_still(
            (&(*glyf))[j as usize]
                .as_deref()
                .unwrap()
                .advance_width
                .clone(),
        ) as Pos;
        if adw > 0_i32 as Pos {
            total_width = (total_width as Pos + adw) as u32;
        }
    }
    (*os_2).x_avg_char_width = (total_width as usize).wrapping_div((*glyf).len()) as i16;
}
unsafe fn stat_max_context_otl(table: *const OtlTable) -> u16 {
    // c2rust's translation of otfcc's own `foreach(item, vector) { ... }`
    // macro (c/lib/otf-writer/stat.c): the __caryll_index*/keep* variables
    // simulate a single-iteration inner while purely so the macro body can
    // `continue`/`break`; every occurrence here reduces to a plain indexed
    // for loop over the vector, confirmed against the original C source.
    let mut maxc: u16 = 1_u16;
    for i in 0..(*table).lookups.len() {
        let lookup: *const Lookup = &raw const *(&(*table).lookups)[i];
        match (*lookup).type_0 {
            OTL_TYPE_GPOS_PAIR
            | OTL_TYPE_GPOS_MARK_TO_BASE
            | OTL_TYPE_GPOS_MARK_TO_LIGATURE
            | OTL_TYPE_GPOS_MARK_TO_MARK => {
                if (maxc as i32) < 2_i32 {
                    maxc = 2_u16;
                }
            }
            OTL_TYPE_GSUB_LIGATURE => {
                for si in 0..(*lookup).subtables.len() {
                    let elem_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, si);
                    let Subtable::GsubLigature(mut_subtable) = &mut *elem_ptr else {
                        unreachable!()
                    };
                    let subtable: *mut GsubLigatureSubtable = mut_subtable;
                    for ei in 0..(*subtable).len() {
                        let entry: *mut GsubLigatureEntry =
                            &mut (&mut (*subtable))[ei] as *mut GsubLigatureEntry;
                        if (maxc as i32)
                            < (*(*entry).from).len() as i32
                        {
                            maxc = (*(*entry).from).len() as u16;
                        }
                    }
                }
            }
            OTL_TYPE_GSUB_CHAINING | OTL_TYPE_GPOS_CHAINING => {
                for si in 0..(*lookup).subtables.len() {
                    let elem_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, si);
                    let Subtable::Chaining(mut_subtable) = &mut *elem_ptr else {
                        unreachable!()
                    };
                    let subtable: *mut ChainingSubtable = mut_subtable;
                    let rule = chaining_rule_mut(subtable);
                    if (maxc as i32) < (*rule).match_count as i32 {
                        maxc = (*rule).match_count;
                    }
                }
            }
            OTL_TYPE_GSUB_REVERSE => {
                for si in 0..(*lookup).subtables.len() {
                    let elem_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, si);
                    let Subtable::GsubReverse(mut_subtable) = &mut *elem_ptr else {
                        unreachable!()
                    };
                    let subtable: *mut GsubReverseSubtable = mut_subtable;
                    if (maxc as i32) < (*subtable).match_count as i32
                    {
                        maxc = (*subtable).match_count;
                    }
                }
            }
            _ => {}
        }
    }
    return maxc;
}
unsafe fn stat_max_context(font: *mut Font) {
    let os_2: *mut Os2Table = (*font).os_2.as_deref_mut().unwrap() as *mut Os2Table;
    let mut maxc: u16 = 1_u16;
    if let Some(gsub) = (*font).gsub.as_deref() {
        let maxc_gsub: u16 = stat_max_context_otl(gsub as *const OtlTable);
        if maxc_gsub as i32 > maxc as i32 {
            maxc = maxc_gsub;
        }
    }
    if let Some(gpos) = (*font).gpos.as_deref() {
        let maxc_gpos: u16 = stat_max_context_otl(gpos as *const OtlTable);
        if maxc_gpos as i32 > maxc as i32 {
            maxc = maxc_gpos;
        }
    }
    (*os_2).us_max_context = maxc;
}
unsafe fn stat_os_2(font: *mut Font, options: &Options) {
    stat_os_2_unicode_ranges(font, options);
    stat_os_2_average_width(font, options);
    stat_max_context(font);
}
pub const MAX_STAT_METRIC: i32 = 4096_i32;
unsafe fn stat_cff_widths(font: *mut Font) {
    if (*font).glyf.is_none() || (*font).cff.is_none() {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    let cff: *mut CffTable = (*font).cff.as_deref_mut().unwrap() as *mut CffTable;
    // A local `Vec` scratch buffer instead of `__caryll_allocate_clean`/
    // `free`.
    let mut frequency: Vec<u32> = vec![0u32; MAX_STAT_METRIC as usize];
    for j in 0..(*glyf).len() as GlyphId {
        let int_width: u16 = vq_get_still(
            (&(*glyf))[j as usize]
                .as_deref()
                .unwrap()
                .advance_width
                .clone(),
        ) as u16;
        if (int_width as i32) < MAX_STAT_METRIC {
            frequency[int_width as usize] = frequency[int_width as usize].wrapping_add(1_u32);
        }
    }
    let mut maxfreq: u16 = 0_u16;
    let mut maxj: u16 = 0_u16;
    for j_0 in 0..MAX_STAT_METRIC as u16 {
        if frequency[j_0 as usize] > maxfreq as u32 {
            maxfreq = frequency[j_0 as usize] as u16;
            maxj = j_0;
        }
    }
    let mut nn: u16 = 0_u16;
    let mut nnsum: u32 = 0_u32;
    for j_1 in 0..(*glyf).len() as GlyphId {
        let adw: Pos = vq_get_still(
            (&(*glyf))[j_1 as usize]
                .as_deref()
                .unwrap()
                .advance_width
                .clone(),
        ) as Pos;
        if adw != maxj as i32 as Pos {
            nn = (nn as i32 + 1_i32) as u16;
            nnsum = (nnsum as Pos + adw) as u32;
        }
    }
    let mut nominal_width_x: i16 = 0_i16;
    if nn as i32 > 0_i32 {
        nominal_width_x = nnsum.wrapping_div(nn as u32) as i16;
    }
    if let Some(pd) = (*cff).private_dict.as_deref_mut() {
        pd.default_width_x = maxj as ::core::ffi::c_double;
        if nn as i32 != 0_i32 {
            pd.nominal_width_x = nominal_width_x as ::core::ffi::c_double;
        }
    }
    for fd in (*cff).fd_array.iter_mut() {
        let pd = fd.private_dict.as_deref_mut().unwrap();
        pd.default_width_x = maxj as ::core::ffi::c_double;
        pd.nominal_width_x = nominal_width_x as ::core::ffi::c_double;
    }
}
unsafe fn stat_vorg(font: *mut Font) {
    if (*font).glyf.is_none()
        || (*font).cff.is_none()
        || (*font).vhea.is_none()
        || (*font).vmtx.is_none()
    {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    // A local `Vec` scratch buffer instead of `__caryll_allocate_clean`/
    // `free`.
    let mut frequency: Vec<u32> = vec![0u32; MAX_STAT_METRIC as usize];
    for j in 0..(*glyf).len() as GlyphId {
        let vori: Pos = vq_get_still(
            (&(*glyf))[j as usize]
                .as_deref()
                .unwrap()
                .vertical_origin
                .clone(),
        ) as Pos;
        if vori >= 0_i32 as Pos && vori < MAX_STAT_METRIC as Pos {
            frequency[vori as u16 as usize] =
                frequency[vori as u16 as usize].wrapping_add(1_u32);
        }
    }
    let mut maxfreq: u32 = 0_u32;
    let mut maxj: GlyphId = 0 as GlyphId;
    for j_0 in 0..MAX_STAT_METRIC as GlyphId {
        if frequency[j_0 as usize] > maxfreq {
            maxfreq = frequency[j_0 as usize];
            maxj = j_0;
        }
    }
    let default_vertical_origin = maxj as Pos;
    let mut n_vert_origs: GlyphId = 0 as GlyphId;
    for j_1 in 0..(*glyf).len() as GlyphId {
        let vori_0: Pos = vq_get_still(
            (&(*glyf))[j_1 as usize]
                .as_deref()
                .unwrap()
                .vertical_origin
                .clone(),
        ) as Pos;
        if vori_0 != maxj as i32 as Pos {
            n_vert_origs =
                (n_vert_origs as i32 + 1_i32) as GlyphId;
        }
    }
    let mut entries: Vec<VorgEntry> = Vec::with_capacity(n_vert_origs as usize);
    for j_2 in 0..(*glyf).len() as GlyphId {
        let vori_1: Pos = vq_get_still(
            (&(*glyf))[j_2 as usize]
                .as_deref()
                .unwrap()
                .vertical_origin
                .clone(),
        ) as Pos;
        if vori_1 != maxj as i32 as Pos {
            entries.push(VorgEntry {
                gid: j_2,
                vertical_origin: vori_1 as i16,
            });
        }
    }
    (*font).vorg = Some(Box::new(VorgTable {
        num_vert_origin_y_metrics: n_vert_origs,
        default_vertical_origin,
        entries,
    }));
}
unsafe fn stat_ltsh(font: *mut Font) {
    if (*font).glyf.is_none() {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    let mut need_ltsh: bool = false;
    for j in 0..(*glyf).len() as GlyphId {
        if (&(*glyf))[j as usize].as_deref().unwrap().y_pel as i32
            > 1_i32
        {
            need_ltsh = true;
        }
    }
    if !need_ltsh {
        return;
    }
    let num_glyphs = (*glyf).len() as GlyphId;
    let mut y_pels: Vec<u8> = Vec::with_capacity(num_glyphs as usize);
    for j_0 in 0..(*glyf).len() as GlyphId {
        y_pels.push((&(*glyf))[j_0 as usize].as_deref().unwrap().y_pel);
    }
    (*font).ltsh = Some(Box::new(LtshTable {
        version: 0,
        num_glyphs,
        y_pels,
    }));
}
pub unsafe fn otfcc_stat_font(font: *mut Font, options: &Options) {
    // Raw-pointer aliases, derived once: `Font.{head,maxp,hhea,vhea}`
    // are never reassigned anywhere in this function's body (only the
    // table contents they point to are mutated, through calls like
    // `stat_glyf`/`stat_maxp` that themselves take `*mut Font`), so
    // deriving these once up front and using them exactly like the old
    // raw-pointer fields (including the `.is_null()` checks below,
    // unchanged) preserves every existing guard and control-flow path
    // without needing `Option`-aware rewriting at each of the ~35 call
    // sites below.
    let head: *mut HeadTable = (*font)
        .head
        .as_deref_mut()
        .map_or(::core::ptr::null_mut(), |h| h as *mut HeadTable);
    let maxp: *mut MaxpTable = (*font)
        .maxp
        .as_deref_mut()
        .map_or(::core::ptr::null_mut(), |m| m as *mut MaxpTable);
    let glyf: *mut GlyfTable = (*font)
        .glyf
        .as_mut()
        .map_or(::core::ptr::null_mut(), |g| g as *mut GlyfTable);
    if !glyf.is_null() && !head.is_null() {
        stat_glyf(font, options);
        if !options.keep_modified_time {
            (*head).modified = 2082844800_i64 + time(::core::ptr::null_mut::<time_t>()) as i64;
        }
    }
    if !head.is_null() && (*font).cff.is_some() {
        let cff: *mut CffTable = (*font).cff.as_deref_mut().unwrap() as *mut CffTable;
        if (*cff).font_b_box_bottom > (*head).y_min as i32 as ::core::ffi::c_double {
            (*cff).font_b_box_bottom = (*head).y_min as ::core::ffi::c_double;
        }
        if (*cff).font_b_box_top < (*head).y_max as i32 as ::core::ffi::c_double {
            (*cff).font_b_box_top = (*head).y_max as ::core::ffi::c_double;
        }
        if (*cff).font_b_box_left < (*head).x_min as i32 as ::core::ffi::c_double {
            (*cff).font_b_box_left = (*head).x_min as ::core::ffi::c_double;
        }
        if (*cff).font_b_box_right < (*head).x_max as i32 as ::core::ffi::c_double {
            (*cff).font_b_box_right = (*head).x_max as ::core::ffi::c_double;
        }
        if !glyf.is_null() && (*cff).is_cid {
            (*cff).cid_count = (*glyf).len() as u32;
        }
        if (*cff).is_cid {
            // `font_matrix` is `Option<Box<CffFontMatrix>>` now: dropping
            // the old value (reassignment to `None`) recurses through its
            // own field-drop glue for free -- no manual `vq_dispose`
            // calls needed anymore (`VQ`'s `Vec<VqSegment>` shift field
            // already self-drops).
            (*cff).font_matrix = None;
            for fd in (*cff).fd_array.iter_mut() {
                fd.font_matrix = None;
                if (*head).units_per_em as i32 == 1000_i32 {
                    fd.font_matrix = None;
                } else {
                    fd.font_matrix = Some(Box::new(CffFontMatrix {
                        a: (1.0f64
                            / (*head).units_per_em as i32 as ::core::ffi::c_double)
                            as Scale,
                        b: 0.0f64 as Scale,
                        c: 0.0f64 as Scale,
                        d: (1.0f64
                            / (*head).units_per_em as i32 as ::core::ffi::c_double)
                            as Scale,
                        x: (vq_neutral)(),
                        y: (vq_neutral)(),
                    }));
                }
            }
        } else if (*head).units_per_em as i32 == 1000_i32 {
            (*cff).font_matrix = None;
        } else {
            (*cff).font_matrix = Some(Box::new(CffFontMatrix {
                a: (1.0f64 / (*head).units_per_em as i32 as ::core::ffi::c_double)
                    as Scale,
                b: 0.0f64 as Scale,
                c: 0.0f64 as Scale,
                d: (1.0f64 / (*head).units_per_em as i32 as ::core::ffi::c_double)
                    as Scale,
                x: (vq_neutral)(),
                y: (vq_neutral)(),
            }));
        }
        stat_cff_widths(font);
    }
    if !glyf.is_null() && !maxp.is_null() {
        (*maxp).num_glyphs = (*glyf).len() as u16;
    }
    if !glyf.is_null() && (*font).post.is_some() {
        (*font).post.as_deref_mut().unwrap().max_mem_type42 = (*glyf).len() as u32;
    }
    if !glyf.is_null() && !maxp.is_null() && (*maxp).version == 0x10000 as F16Dot16 {
        stat_maxp(font);
        if let Some(fpgm) = &(*font).fpgm {
            let fpgm_length = fpgm.bytes.len() as u32;
            if fpgm_length > (*maxp).max_size_of_instructions as u32 {
                (*maxp).max_size_of_instructions = fpgm_length as u16;
            }
        }
        if let Some(prep) = &(*font).prep {
            let prep_length = prep.bytes.len() as u32;
            if prep_length > (*maxp).max_size_of_instructions as u32 {
                (*maxp).max_size_of_instructions = prep_length as u16;
            }
        }
    }
    if (*font).os_2.is_some() && (*font).cmap.is_some() && !glyf.is_null() {
        stat_os_2(font, options);
    }
    if (*font).subtype == FontSubtype::Ttf {
        if !maxp.is_null() {
            (*maxp).version = 0x10000_i32 as F16Dot16;
        }
    } else if !maxp.is_null() {
        (*maxp).version = 0x5000_i32 as F16Dot16;
    }
    if !glyf.is_null() && (*font).hhea.is_some() {
        stat_hmtx(font);
    }
    if !glyf.is_null() && (*font).vhea.is_some() {
        stat_vmtx(font, options);
        stat_vorg(font);
    }
    stat_ltsh(font);
}
pub unsafe fn otfcc_unstat_font(font: *mut Font) {
    delete_font_table(font, crate::tag::TAG_HDMX);
    delete_font_table(font, crate::tag::TAG_HMTX);
    delete_font_table(font, crate::tag::TAG_VORG);
    delete_font_table(font, crate::tag::TAG_VMTX);
    delete_font_table(font, crate::tag::TAG_LTSH);
}
pub const FLT_MAX: ::core::ffi::c_float = __FLT_MAX__;
pub const __FLT_MAX__: ::core::ffi::c_float = 3.40282347e+38f32;
