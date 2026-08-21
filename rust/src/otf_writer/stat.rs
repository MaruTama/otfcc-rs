#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{time, time_t};
unsafe extern "C" {
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}


use crate::support::handle::{HandleState, handle_from_index, GlyphHandle, Handle, otfcc_handle_replace};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, logger_log_sds};

use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, GlyphId, Length, Pos, Scale, ShapeId};
use crate::font::caryll_font::{FontSubtype, Font};



use crate::table::cff::{CffFontMatrix, CffTable};



use crate::table::ltsh::{LtshTable};



use crate::table::vorg::{VorgEntry, VorgTable};





use crate::table::glyf::{RefAnchorStatus, ComponentReference, Contour, Glyph, GlyphStat, Point, GlyfTable};



use crate::table::hmtx::{HorizontalMetric, HmtxTable};
use crate::table::os_2::{Os2Table};
use crate::table::head::{HeadTable};
use crate::table::hhea::{HheaTable};
use crate::table::maxp::{MaxpTable};
use crate::table::vhea::{VheaTable};



use crate::table::otl::{GsubLigatureEntry, Lookup, Subtable, SubtablePtr, subtable_at, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_REVERSE, ChainingSubtable, GsubLigatureSubtable, GsubReverseSubtable, OtlTable};
use crate::table::otl::subtables::chaining::common::{chaining_rule_mut};




use crate::table::vmtx::{VmtxTable, VerticalMetric};



use crate::vf::vq::{VQ};
use crate::font::caryll_font::{delete_font_table};
use crate::table::glyf::{glyf_component_reference_init};
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
    mut table: *const GlyfTable,
    mut gr: *mut ComponentReference,
    mut stated: *mut StatStatus,
    mut depth: u8,
    mut topj: GlyphId,
    mut options: *const Options,
) -> GlyphStat {
    let mut stat: GlyphStat = GlyphStat {
        x_min: 0 as ::core::ffi::c_int as Pos,
        x_max: 0 as ::core::ffi::c_int as Pos,
        y_min: 0 as ::core::ffi::c_int as Pos,
        y_max: 0 as ::core::ffi::c_int as Pos,
        nest_depth: 0 as u16,
        n_points: 0 as u16,
        n_contours: 0 as u16,
        n_composite_points: 0 as u16,
        n_composite_contours: 0 as u16,
    };
    let j: GlyphId = (*gr).glyph.index;
    if depth as ::core::ffi::c_int >= 0xff as ::core::ffi::c_int {
        return stat;
    }
    if *stated.offset(j as isize) == StatStatus::Doing {
        logger_log_sds(
            (*options).logger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"[Stat] Circular glyph reference found in gid ",
                topj as ::core::ffi::c_int,
                b" to gid ",
                j as ::core::ffi::c_int,
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
    let mut nest_depth: u16 = 0 as u16;
    let mut n_points: u16 = 0 as u16;
    let mut n_composite_points: u16 = 0 as u16;
    let mut n_composite_contours: u16 = 0 as u16;
    for c in 0..(*g).contours.len() as ShapeId {
        let contour: *const Contour = &(&(*g).contours)[c as usize];
        for pj in 0..(*contour).len() as ShapeId {
            let p: *const Point = &(&(*contour))[pj as usize];
            let x: Pos = round(
                vq_get_still((*gr).x.clone()) as ::core::ffi::c_double
                    + (*gr).a as ::core::ffi::c_double
                        * vq_get_still((*p).x.clone())
                            as ::core::ffi::c_double
                    + (*gr).b as ::core::ffi::c_double
                        * vq_get_still((*p).y.clone())
                            as ::core::ffi::c_double,
            ) as Pos;
            let mut y: Pos = round(
                vq_get_still((*gr).y.clone()) as ::core::ffi::c_double
                    + (*gr).c as ::core::ffi::c_double
                        * vq_get_still((*p).x.clone())
                            as ::core::ffi::c_double
                    + (*gr).d as ::core::ffi::c_double
                        * vq_get_still((*p).y.clone())
                            as ::core::ffi::c_double,
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
            n_points = (n_points as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
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
            handle_from_index((*rr).glyph.index)
                as Handle,
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
        let mut thatstat: GlyphStat = stat_single_glyph(
            table,
            &raw mut ref_0,
            stated,
            (depth as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8,
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
        if thatstat.nest_depth as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            > nest_depth as ::core::ffi::c_int
        {
            nest_depth =
                (thatstat.nest_depth as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
        }
        n_composite_points = (n_composite_points as ::core::ffi::c_int
            + thatstat.n_composite_points as ::core::ffi::c_int)
            as u16;
        n_composite_contours = (n_composite_contours as ::core::ffi::c_int
            + thatstat.n_composite_contours as ::core::ffi::c_int)
            as u16;
    }
    if xmin > xmax {
        xmax = 0 as ::core::ffi::c_int as Pos;
        xmin = xmax;
    }
    if ymin > ymax {
        ymax = 0 as ::core::ffi::c_int as Pos;
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
pub unsafe fn stat_glyf(mut font: *mut Font, mut options: *const Options) {
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
        gr.glyph =
            handle_from_index(j) as GlyphHandle;
        gr.x =
            vq_create_still(0 as ::core::ffi::c_int as Pos);
        gr.y =
            vq_create_still(0 as ::core::ffi::c_int as Pos);
        gr.a = 1 as ::core::ffi::c_int as Scale;
        gr.b = 0 as ::core::ffi::c_int as Scale;
        gr.c = 0 as ::core::ffi::c_int as Scale;
        gr.d = 1 as ::core::ffi::c_int as Scale;
        let ref mut fresh2 = (&mut (*glyf))[j as usize].as_mut().unwrap().stat;
        *fresh2 = stat_single_glyph(glyf, &raw mut gr, stated.as_mut_ptr(), 0 as u8, j, options);
        let mut thatstat: GlyphStat = *fresh2;
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
pub unsafe fn stat_maxp(mut font: *mut Font) {
    // Only ever called (from `otfcc_stat_font`) under a `.maxp.is_some()`
    // guard.
    let maxp: *mut MaxpTable = (*font).maxp.as_deref_mut().unwrap() as *mut MaxpTable;
    let mut nest_depth: u16 = 0 as u16;
    let mut n_points: u16 = 0 as u16;
    let mut n_contours: u16 = 0 as u16;
    let mut n_components: u16 = 0 as u16;
    let mut n_composite_points: u16 = 0 as u16;
    let mut n_composite_contours: u16 = 0 as u16;
    let mut inst_size: u16 = 0 as u16;
    // Only ever called (from `otfcc_stat_font`) under a `.glyf.is_some()`
    // guard.
    let glyf: *const GlyfTable = (*font).glyf.as_ref().unwrap() as *const GlyfTable;
    for j in 0..(*glyf).len() as GlyphId {
        let g: *const Glyph = (&(*glyf))[j as usize].as_deref().unwrap() as *const Glyph;
        if (*g).contours.len() > 0 as usize {
            if (*g).stat.n_points as ::core::ffi::c_int > n_points as ::core::ffi::c_int {
                n_points = (*g).stat.n_points;
            }
            if (*g).stat.n_contours as ::core::ffi::c_int > n_contours as ::core::ffi::c_int {
                n_contours = (*g).stat.n_contours;
            }
        } else if (*g).references.len() > 0 as usize {
            if (*g).stat.n_composite_points as ::core::ffi::c_int
                > n_composite_points as ::core::ffi::c_int
            {
                n_composite_points = (*g).stat.n_composite_points;
            }
            if (*g).stat.n_composite_contours as ::core::ffi::c_int
                > n_composite_contours as ::core::ffi::c_int
            {
                n_composite_contours = (*g).stat.n_composite_contours;
            }
            if (*g).stat.nest_depth as ::core::ffi::c_int > nest_depth as ::core::ffi::c_int {
                nest_depth = (*g).stat.nest_depth;
            }
            if (*g).references.len() > n_components as usize {
                n_components = (*g).references.len() as u16;
            }
        }
        if (*g).instructions_length as ::core::ffi::c_int > inst_size as ::core::ffi::c_int {
            inst_size = (*g).instructions_length;
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
unsafe fn stat_hmtx(mut font: *mut Font) {
    if (*font).glyf.is_none() {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    // Only ever called (from `otfcc_stat_font`) under a `.hhea.is_some()`
    // guard; `.head` is set unconditionally by the pipeline before this
    // point (used below to update `.flags`).
    let hhea: *mut HheaTable = (*font).hhea.as_deref_mut().unwrap() as *mut HheaTable;
    let head: *mut HeadTable = (*font).head.as_deref_mut().map_or(::core::ptr::null_mut(), |h| h as *mut HeadTable);
    let mut count_a: GlyphId = (*glyf).len() as GlyphId;
    let mut count_k: GlyphId = 0 as GlyphId;
    let mut lsb_at_x_0: bool = true;
    if (*font).subtype != FontSubtype::Cff {
        while count_a as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            && vq_get_still(
                (&(*glyf))
                    [(count_a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize].as_deref().unwrap()
                .advance_width.clone(),
            ) == vq_get_still(
                (&(*glyf))
                    [(count_a as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize].as_deref().unwrap()
                .advance_width.clone(),
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
    let mut min_lsb: Pos = 0x7fff as ::core::ffi::c_int as Pos;
    let mut min_rsb: Pos = 0x7fff as ::core::ffi::c_int as Pos;
    let mut max_extent: Pos = -(0x8000 as ::core::ffi::c_int) as Pos;
    let mut max_width: Length = 0 as ::core::ffi::c_int as Length;
    for j in 0..(*glyf).len() as GlyphId {
        let g: *mut Glyph = &raw mut **(&mut (*glyf))[j as usize].as_mut().unwrap();
        if vq_is_zero((*g).horizontal_origin.clone(), 1.0f64 / 1000.0f64)
        {
            vq_replace(
                &raw mut (*g).horizontal_origin,
                (
                    vq_neutral)() as VQ,
            );
        } else {
            lsb_at_x_0 = false;
        }
        let hori: Pos =
            vq_get_still((*g).horizontal_origin.clone()) as Pos;
        let advw: Pos =
            vq_get_still((*g).advance_width.clone()) as Pos;
        let lsb: Pos = (*g).stat.x_min - hori;
        let rsb: Pos = advw + hori - (*g).stat.x_max;
        if (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            metrics.push(HorizontalMetric { advance_width: advw as Length, lsb });
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
    (*font).hmtx = Some(Box::new(HmtxTable { metrics, left_side_bearing }));
    (*head).flags = ((*head).flags as ::core::ffi::c_int
        & !(0x2 as ::core::ffi::c_int)
        | (if lsb_at_x_0 { 0x2 as ::core::ffi::c_int } else { 0 as ::core::ffi::c_int }))
        as u16;
}
unsafe fn stat_vmtx(mut font: *mut Font, mut options: *const Options) {
    if (*font).glyf.is_none() {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    // Only ever called (from `otfcc_stat_font`) under a `.vhea.is_some()`
    // guard.
    let vhea: *mut VheaTable = (*font).vhea.as_deref_mut().unwrap() as *mut VheaTable;
    let mut count_a: GlyphId = (*glyf).len() as GlyphId;
    let mut count_k: GlyphId = 0 as GlyphId;
    if !((*font).subtype == FontSubtype::Cff && !(*options).cff_short_vmtx) {
        while count_a as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            && vq_get_still(
                (&(*glyf))
                    [(count_a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize].as_deref().unwrap()
                .advance_height.clone(),
            ) == vq_get_still(
                (&(*glyf))
                    [(count_a as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize].as_deref().unwrap()
                .advance_height.clone(),
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
    let mut min_tsb: Pos = 0x7fff as ::core::ffi::c_int as Pos;
    let mut min_bsb: Pos = 0x7fff as ::core::ffi::c_int as Pos;
    let mut max_extent: Pos = -(0x8000 as ::core::ffi::c_int) as Pos;
    let mut max_height: Length = 0 as ::core::ffi::c_int as Length;
    for j in 0..(*glyf).len() as GlyphId {
        let g: *const Glyph = (&(*glyf))[j as usize].as_deref().unwrap() as *const Glyph;
        let vori: Pos =
            vq_get_still((*g).vertical_origin.clone()) as Pos;
        let advh: Pos =
            vq_get_still((*g).advance_height.clone()) as Pos;
        let tsb: Pos = vori - (*g).stat.y_max;
        let bsb: Pos = (*g).stat.y_min - vori + advh;
        if (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            metrics.push(VerticalMetric { advance_height: advh as Length, tsb });
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
    (*font).vmtx = Some(Box::new(VmtxTable { metrics, top_side_bearing }));
}
unsafe fn stat_os_2_unicode_ranges(
    mut font: *mut Font,
    mut options: *const Options,
) {
    let os_2: *mut Os2Table = (*font).os_2.as_deref_mut().unwrap() as *mut Os2Table;
    let mut u1: u32 = 0 as u32;
    let mut u2: u32 = 0 as u32;
    let mut u3: u32 = 0 as u32;
    let mut u4: u32 = 0 as u32;
    let mut min_unicode: i32 = 0xffff as i32;
    let mut max_unicode: i32 = 0 as i32;
    for (&u, _) in (*font).cmap.as_ref().unwrap().unicodes.iter() {
        if (u as i32) < min_unicode {
            min_unicode = u as i32;
        }
        if u as i32 > max_unicode {
            max_unicode = u as i32;
        }
        if u >= 0 as ::core::ffi::c_int && u <= 0x7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x80 as ::core::ffi::c_int && u <= 0xff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x100 as ::core::ffi::c_int && u <= 0x17f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x180 as ::core::ffi::c_int && u <= 0x24f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x250 as ::core::ffi::c_int && u <= 0x2af as ::core::ffi::c_int
            || u >= 0x1d00 as ::core::ffi::c_int && u <= 0x1d7f as ::core::ffi::c_int
            || u >= 0x1d80 as ::core::ffi::c_int && u <= 0x1dbf as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2b0 as ::core::ffi::c_int && u <= 0x2ff as ::core::ffi::c_int
            || u >= 0xa700 as ::core::ffi::c_int && u <= 0xa71f as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x300 as ::core::ffi::c_int && u <= 0x36f as ::core::ffi::c_int
            || u >= 0x1dc0 as ::core::ffi::c_int && u <= 0x1dff as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x370 as ::core::ffi::c_int && u <= 0x3ff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2c80 as ::core::ffi::c_int && u <= 0x2cff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x400 as ::core::ffi::c_int && u <= 0x4ff as ::core::ffi::c_int
            || u >= 0x500 as ::core::ffi::c_int && u <= 0x52f as ::core::ffi::c_int
            || u >= 0x2de0 as ::core::ffi::c_int && u <= 0x2dff as ::core::ffi::c_int
            || u >= 0xa640 as ::core::ffi::c_int && u <= 0xa69f as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x530 as ::core::ffi::c_int && u <= 0x58f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x590 as ::core::ffi::c_int && u <= 0x5ff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xa500 as ::core::ffi::c_int && u <= 0xa63f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x600 as ::core::ffi::c_int && u <= 0x6ff as ::core::ffi::c_int
            || u >= 0x750 as ::core::ffi::c_int && u <= 0x77f as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x7c0 as ::core::ffi::c_int && u <= 0x7ff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x900 as ::core::ffi::c_int && u <= 0x97f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x980 as ::core::ffi::c_int && u <= 0x9ff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xa00 as ::core::ffi::c_int && u <= 0xa7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xa80 as ::core::ffi::c_int && u <= 0xaff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xb00 as ::core::ffi::c_int && u <= 0xb7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xb80 as ::core::ffi::c_int && u <= 0xbff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xc00 as ::core::ffi::c_int && u <= 0xc7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xc80 as ::core::ffi::c_int && u <= 0xcff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xd00 as ::core::ffi::c_int && u <= 0xd7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xe00 as ::core::ffi::c_int && u <= 0xe7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xe80 as ::core::ffi::c_int && u <= 0xeff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 25 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10a0 as ::core::ffi::c_int && u <= 0x10ff as ::core::ffi::c_int
            || u >= 0x2d00 as ::core::ffi::c_int && u <= 0x2d2f as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 26 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1b00 as ::core::ffi::c_int && u <= 0x1b7f as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1100 as ::core::ffi::c_int && u <= 0x11ff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1e00 as ::core::ffi::c_int && u <= 0x1eff as ::core::ffi::c_int
            || u >= 0x2c60 as ::core::ffi::c_int && u <= 0x2c7f as ::core::ffi::c_int
            || u >= 0xa720 as ::core::ffi::c_int && u <= 0xa7ff as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1f00 as ::core::ffi::c_int && u <= 0x1fff as ::core::ffi::c_int {
            u1 |= ((1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2000 as ::core::ffi::c_int && u <= 0x206f as ::core::ffi::c_int
            || u >= 0x2e00 as ::core::ffi::c_int && u <= 0x2e7f as ::core::ffi::c_int
        {
            u1 |= ((1 as ::core::ffi::c_int) << 31 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2070 as ::core::ffi::c_int && u <= 0x209f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x20a0 as ::core::ffi::c_int && u <= 0x20cf as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x20d0 as ::core::ffi::c_int && u <= 0x20ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2100 as ::core::ffi::c_int && u <= 0x214f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2150 as ::core::ffi::c_int && u <= 0x218f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2190 as ::core::ffi::c_int && u <= 0x21ff as ::core::ffi::c_int
            || u >= 0x27f0 as ::core::ffi::c_int && u <= 0x27ff as ::core::ffi::c_int
            || u >= 0x2900 as ::core::ffi::c_int && u <= 0x297f as ::core::ffi::c_int
            || u >= 0x2b00 as ::core::ffi::c_int && u <= 0x2bff as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2200 as ::core::ffi::c_int && u <= 0x22ff as ::core::ffi::c_int
            || u >= 0x2a00 as ::core::ffi::c_int && u <= 0x2aff as ::core::ffi::c_int
            || u >= 0x27c0 as ::core::ffi::c_int && u <= 0x27ef as ::core::ffi::c_int
            || u >= 0x2980 as ::core::ffi::c_int && u <= 0x29ff as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2300 as ::core::ffi::c_int && u <= 0x23ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2400 as ::core::ffi::c_int && u <= 0x243f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2440 as ::core::ffi::c_int && u <= 0x245f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2460 as ::core::ffi::c_int && u <= 0x24ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2500 as ::core::ffi::c_int && u <= 0x257f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2580 as ::core::ffi::c_int && u <= 0x259f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x25a0 as ::core::ffi::c_int && u <= 0x25ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2600 as ::core::ffi::c_int && u <= 0x26ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2700 as ::core::ffi::c_int && u <= 0x27bf as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x3000 as ::core::ffi::c_int && u <= 0x303f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x3040 as ::core::ffi::c_int && u <= 0x309f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x30a0 as ::core::ffi::c_int && u <= 0x30ff as ::core::ffi::c_int
            || u >= 0x31f0 as ::core::ffi::c_int && u <= 0x31ff as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x3100 as ::core::ffi::c_int && u <= 0x312f as ::core::ffi::c_int
            || u >= 0x31a0 as ::core::ffi::c_int && u <= 0x31bf as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x3130 as ::core::ffi::c_int && u <= 0x318f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xa840 as ::core::ffi::c_int && u <= 0xa87f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x3200 as ::core::ffi::c_int && u <= 0x32ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x3300 as ::core::ffi::c_int && u <= 0x33ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xac00 as ::core::ffi::c_int && u <= 0xd7af as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xd800 as ::core::ffi::c_int && u <= 0xdfff as ::core::ffi::c_int
            || u > 0xffff as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 25 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10900 as ::core::ffi::c_int && u <= 0x1091f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 26 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x4e00 as ::core::ffi::c_int && u <= 0x9fff as ::core::ffi::c_int
            || u >= 0x2e80 as ::core::ffi::c_int && u <= 0x2eff as ::core::ffi::c_int
            || u >= 0x2f00 as ::core::ffi::c_int && u <= 0x2fdf as ::core::ffi::c_int
            || u >= 0x2ff0 as ::core::ffi::c_int && u <= 0x2fff as ::core::ffi::c_int
            || u >= 0x3400 as ::core::ffi::c_int && u <= 0x4dbf as ::core::ffi::c_int
            || u >= 0x20000 as ::core::ffi::c_int && u <= 0x2f7ff as ::core::ffi::c_int
            || u >= 0x3190 as ::core::ffi::c_int && u <= 0x319f as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xe000 as ::core::ffi::c_int && u <= 0xf8ff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x31c0 as ::core::ffi::c_int && u <= 0x31ef as ::core::ffi::c_int
            || u >= 0xf900 as ::core::ffi::c_int && u <= 0xfaff as ::core::ffi::c_int
            || u >= 0x2f800 as ::core::ffi::c_int && u <= 0x2fa1f as ::core::ffi::c_int
        {
            u2 |= ((1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xfb00 as ::core::ffi::c_int && u <= 0xfb4f as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xfb50 as ::core::ffi::c_int && u <= 0xfdff as ::core::ffi::c_int {
            u2 |= ((1 as ::core::ffi::c_int) << 31 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xfe20 as ::core::ffi::c_int && u <= 0xfe2f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xfe10 as ::core::ffi::c_int && u <= 0xfe1f as ::core::ffi::c_int
            || u >= 0xfe30 as ::core::ffi::c_int && u <= 0xfe4f as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xfe50 as ::core::ffi::c_int && u <= 0xfe6f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xfe70 as ::core::ffi::c_int && u <= 0xfeff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xff00 as ::core::ffi::c_int && u <= 0xffef as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xfff0 as ::core::ffi::c_int && u <= 0xffff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xf00 as ::core::ffi::c_int && u <= 0xfff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x700 as ::core::ffi::c_int && u <= 0x74f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x780 as ::core::ffi::c_int && u <= 0x7bf as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xd80 as ::core::ffi::c_int && u <= 0xdff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1000 as ::core::ffi::c_int && u <= 0x109f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1200 as ::core::ffi::c_int && u <= 0x137f as ::core::ffi::c_int
            || u >= 0x1380 as ::core::ffi::c_int && u <= 0x139f as ::core::ffi::c_int
            || u >= 0x2d80 as ::core::ffi::c_int && u <= 0x2ddf as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x13a0 as ::core::ffi::c_int && u <= 0x13ff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1400 as ::core::ffi::c_int && u <= 0x167f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1680 as ::core::ffi::c_int && u <= 0x169f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x16a0 as ::core::ffi::c_int && u <= 0x16ff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1780 as ::core::ffi::c_int && u <= 0x17ff as ::core::ffi::c_int
            || u >= 0x19e0 as ::core::ffi::c_int && u <= 0x19ff as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1800 as ::core::ffi::c_int && u <= 0x18af as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2800 as ::core::ffi::c_int && u <= 0x28ff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xa000 as ::core::ffi::c_int && u <= 0xa48f as ::core::ffi::c_int
            || u >= 0xa490 as ::core::ffi::c_int && u <= 0xa4cf as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1700 as ::core::ffi::c_int && u <= 0x171f as ::core::ffi::c_int
            || u >= 0x1720 as ::core::ffi::c_int && u <= 0x173f as ::core::ffi::c_int
            || u >= 0x1740 as ::core::ffi::c_int && u <= 0x175f as ::core::ffi::c_int
            || u >= 0x1760 as ::core::ffi::c_int && u <= 0x177f as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10300 as ::core::ffi::c_int && u <= 0x1032f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10330 as ::core::ffi::c_int && u <= 0x1034f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10400 as ::core::ffi::c_int && u <= 0x1044f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1d000 as ::core::ffi::c_int && u <= 0x1d0ff as ::core::ffi::c_int
            || u >= 0x1d100 as ::core::ffi::c_int && u <= 0x1d1ff as ::core::ffi::c_int
            || u >= 0x1d200 as ::core::ffi::c_int && u <= 0x1d24f as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1d400 as ::core::ffi::c_int && u <= 0x1d7ff as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 25 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xff000 as ::core::ffi::c_int && u <= 0xffffd as ::core::ffi::c_int
            || u >= 0x100000 as ::core::ffi::c_int && u <= 0x10fffd as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 26 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xfe00 as ::core::ffi::c_int && u <= 0xfe0f as ::core::ffi::c_int
            || u >= 0xe0100 as ::core::ffi::c_int && u <= 0xe01ef as ::core::ffi::c_int
        {
            u3 |= ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xe0000 as ::core::ffi::c_int && u <= 0xe007f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1900 as ::core::ffi::c_int && u <= 0x194f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1950 as ::core::ffi::c_int && u <= 0x197f as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1980 as ::core::ffi::c_int && u <= 0x19df as ::core::ffi::c_int {
            u3 |= ((1 as ::core::ffi::c_int) << 31 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1a00 as ::core::ffi::c_int && u <= 0x1a1f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2c00 as ::core::ffi::c_int && u <= 0x2c5f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x2d30 as ::core::ffi::c_int && u <= 0x2d7f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x4dc0 as ::core::ffi::c_int && u <= 0x4dff as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xa800 as ::core::ffi::c_int && u <= 0xa82f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10000 as ::core::ffi::c_int && u <= 0x1007f as ::core::ffi::c_int
            || u >= 0x10080 as ::core::ffi::c_int && u <= 0x100ff as ::core::ffi::c_int
            || u >= 0x10100 as ::core::ffi::c_int && u <= 0x1013f as ::core::ffi::c_int
        {
            u4 |= ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10140 as ::core::ffi::c_int && u <= 0x1018f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10380 as ::core::ffi::c_int && u <= 0x1039f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x103a0 as ::core::ffi::c_int && u <= 0x103df as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10450 as ::core::ffi::c_int && u <= 0x1047f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10480 as ::core::ffi::c_int && u <= 0x104af as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10800 as ::core::ffi::c_int && u <= 0x1083f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10a00 as ::core::ffi::c_int && u <= 0x10a5f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1d300 as ::core::ffi::c_int && u <= 0x1d35f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x12000 as ::core::ffi::c_int && u <= 0x123ff as ::core::ffi::c_int
            || u >= 0x12400 as ::core::ffi::c_int && u <= 0x1247f as ::core::ffi::c_int
        {
            u4 |= ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1d360 as ::core::ffi::c_int && u <= 0x1d37f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1b80 as ::core::ffi::c_int && u <= 0x1bbf as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1c00 as ::core::ffi::c_int && u <= 0x1c4f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1c50 as ::core::ffi::c_int && u <= 0x1c7f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xa880 as ::core::ffi::c_int && u <= 0xa8df as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xa900 as ::core::ffi::c_int && u <= 0xa92f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xa930 as ::core::ffi::c_int && u <= 0xa95f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as u32;
        }
        if u >= 0xaa00 as ::core::ffi::c_int && u <= 0xaa5f as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x10190 as ::core::ffi::c_int && u <= 0x101cf as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x101d0 as ::core::ffi::c_int && u <= 0x101ff as ::core::ffi::c_int {
            u4 |= ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x102a0 as ::core::ffi::c_int && u <= 0x102df as ::core::ffi::c_int
            || u >= 0x10280 as ::core::ffi::c_int && u <= 0x1029f as ::core::ffi::c_int
            || u >= 0x10920 as ::core::ffi::c_int && u <= 0x1093f as ::core::ffi::c_int
        {
            u4 |= ((1 as ::core::ffi::c_int) << 25 as ::core::ffi::c_int) as u32;
        }
        if u >= 0x1f030 as ::core::ffi::c_int && u <= 0x1f09f as ::core::ffi::c_int
            || u >= 0x1f000 as ::core::ffi::c_int && u <= 0x1f02f as ::core::ffi::c_int
        {
            u4 |= ((1 as ::core::ffi::c_int) << 26 as ::core::ffi::c_int) as u32;
        }
    }
    if !(*options).keep_unicode_ranges {
        (*os_2).ul_unicode_range1 = u1;
        (*os_2).ul_unicode_range2 = u2;
        (*os_2).ul_unicode_range3 = u3;
        (*os_2).ul_unicode_range4 = u4;
    }
    if min_unicode < 0x10000 as i32 {
        (*os_2).us_first_char_index = min_unicode as u16;
    } else {
        (*os_2).us_first_char_index = 0xffff as u16;
    }
    if max_unicode < 0x10000 as i32 {
        (*os_2).us_last_char_index = max_unicode as u16;
    } else {
        (*os_2).us_last_char_index = 0xffff as u16;
    };
}
unsafe fn stat_os_2_average_width(
    mut font: *mut Font,
    mut options: *const Options,
) {
    if (*options).keep_average_char_width {
        return;
    }
    let os_2: *mut Os2Table = (*font).os_2.as_deref_mut().unwrap() as *mut Os2Table;
    // Only ever called (from `otfcc_stat_font`, via `stat_os_2`) under a
    // `.glyf.is_some()` guard.
    let glyf: *const GlyfTable = (*font).glyf.as_ref().unwrap() as *const GlyfTable;
    let mut total_width: u32 = 0 as u32;
    for j in 0..(*glyf).len() as GlyphId {
        let adw: Pos = vq_get_still(
            (&(*glyf))[j as usize].as_deref().unwrap().advance_width.clone(),
        ) as Pos;
        if adw > 0 as ::core::ffi::c_int as Pos {
            total_width = (total_width as Pos + adw) as u32;
        }
    }
    (*os_2).x_avg_char_width =
        (total_width as usize).wrapping_div((*glyf).len()) as i16;
}
unsafe fn stat_max_context_otl(table: *const OtlTable) -> u16 {
    // c2rust's translation of otfcc's own `foreach(item, vector) { ... }`
    // macro (c/lib/otf-writer/stat.c): the __caryll_index*/keep* variables
    // simulate a single-iteration inner while purely so the macro body can
    // `continue`/`break`; every occurrence here reduces to a plain indexed
    // for loop over the vector, confirmed against the original C source.
    let mut maxc: u16 = 1 as u16;
    for i in 0..(*table).lookups.len() {
        let lookup: *const Lookup = &raw const *(&(*table).lookups)[i];
        match (*lookup).type_0 {
            OTL_TYPE_GPOS_PAIR | OTL_TYPE_GPOS_MARK_TO_BASE | OTL_TYPE_GPOS_MARK_TO_LIGATURE
            | OTL_TYPE_GPOS_MARK_TO_MARK => {
                if (maxc as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                    maxc = 2 as u16;
                }
            }
            OTL_TYPE_GSUB_LIGATURE => {
                for si in 0..(*lookup).subtables.len() {
                    let elem_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, si);
                    let Subtable::GsubLigature(mut_subtable) = &mut *elem_ptr else { unreachable!() };
                    let subtable: *mut GsubLigatureSubtable = mut_subtable;
                    for ei in 0..(*subtable).len() {
                        let entry: *mut GsubLigatureEntry = &mut (&mut (*subtable))[ei as usize] as *mut GsubLigatureEntry;
                        if (maxc as ::core::ffi::c_int) < (*(*entry).from).len() as ::core::ffi::c_int
                        {
                            maxc = (*(*entry).from).len() as u16;
                        }
                    }
                }
            }
            OTL_TYPE_GSUB_CHAINING | OTL_TYPE_GPOS_CHAINING => {
                for si in 0..(*lookup).subtables.len() {
                    let elem_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, si);
                    let Subtable::Chaining(mut_subtable) = &mut *elem_ptr else { unreachable!() };
                    let subtable: *mut ChainingSubtable = mut_subtable;
                    let rule = chaining_rule_mut(subtable);
                    if (maxc as ::core::ffi::c_int)
                        < (*rule).match_count as ::core::ffi::c_int
                    {
                        maxc = (*rule).match_count as u16;
                    }
                }
            }
            OTL_TYPE_GSUB_REVERSE => {
                for si in 0..(*lookup).subtables.len() {
                    let elem_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, si);
                    let Subtable::GsubReverse(mut_subtable) = &mut *elem_ptr else { unreachable!() };
                    let subtable: *mut GsubReverseSubtable = mut_subtable;
                    if (maxc as ::core::ffi::c_int) < (*subtable).match_count as ::core::ffi::c_int {
                        maxc = (*subtable).match_count as u16;
                    }
                }
            }
            _ => {}
        }
    }
    return maxc;
}
unsafe fn stat_max_context(mut font: *mut Font) {
    let os_2: *mut Os2Table = (*font).os_2.as_deref_mut().unwrap() as *mut Os2Table;
    let mut maxc: u16 = 1 as u16;
    if let Some(gsub) = (*font).gsub.as_deref() {
        let mut maxc_gsub: u16 = stat_max_context_otl(gsub as *const OtlTable);
        if maxc_gsub as ::core::ffi::c_int > maxc as ::core::ffi::c_int {
            maxc = maxc_gsub;
        }
    }
    if let Some(gpos) = (*font).gpos.as_deref() {
        let mut maxc_gpos: u16 = stat_max_context_otl(gpos as *const OtlTable);
        if maxc_gpos as ::core::ffi::c_int > maxc as ::core::ffi::c_int {
            maxc = maxc_gpos;
        }
    }
    (*os_2).us_max_context = maxc;
}
unsafe fn stat_os_2(mut font: *mut Font, mut options: *const Options) {
    stat_os_2_unicode_ranges(font, options);
    stat_os_2_average_width(font, options);
    stat_max_context(font);
}
pub const MAX_STAT_METRIC: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe fn stat_cff_widths(mut font: *mut Font) {
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
            (&(*glyf))[j as usize].as_deref().unwrap().advance_width.clone(),
        ) as u16;
        if (int_width as ::core::ffi::c_int) < MAX_STAT_METRIC {
            frequency[int_width as usize] = frequency[int_width as usize].wrapping_add(1 as u32);
        }
    }
    let mut maxfreq: u16 = 0 as u16;
    let mut maxj: u16 = 0 as u16;
    for j_0 in 0..MAX_STAT_METRIC as u16 {
        if frequency[j_0 as usize] > maxfreq as u32 {
            maxfreq = frequency[j_0 as usize] as u16;
            maxj = j_0;
        }
    }
    let mut nn: u16 = 0 as u16;
    let mut nnsum: u32 = 0 as u32;
    for j_1 in 0..(*glyf).len() as GlyphId {
        let adw: Pos = vq_get_still(
            (&(*glyf))[j_1 as usize].as_deref().unwrap().advance_width.clone(),
        ) as Pos;
        if adw != maxj as ::core::ffi::c_int as Pos {
            nn = (nn as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
            nnsum = (nnsum as Pos + adw) as u32;
        }
    }
    let mut nominal_width_x: i16 = 0 as i16;
    if nn as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        nominal_width_x = nnsum.wrapping_div(nn as u32) as i16;
    }
    if let Some(pd) = (*cff).private_dict.as_deref_mut() {
        pd.default_width_x = maxj as ::core::ffi::c_double;
        if nn as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            pd.nominal_width_x = nominal_width_x as ::core::ffi::c_double;
        }
    }
    for fd in (*cff).fd_array.iter_mut() {
        let pd = fd.private_dict.as_deref_mut().unwrap();
        pd.default_width_x = maxj as ::core::ffi::c_double;
        pd.nominal_width_x = nominal_width_x as ::core::ffi::c_double;
    }
}
unsafe fn stat_vorg(mut font: *mut Font) {
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
            (&(*glyf))[j as usize].as_deref().unwrap().vertical_origin.clone(),
        ) as Pos;
        if vori >= 0 as ::core::ffi::c_int as Pos && vori < MAX_STAT_METRIC as Pos {
            frequency[vori as u16 as usize] = frequency[vori as u16 as usize].wrapping_add(1 as u32);
        }
    }
    let mut maxfreq: u32 = 0 as u32;
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
            (&(*glyf))[j_1 as usize].as_deref().unwrap().vertical_origin.clone(),
        ) as Pos;
        if vori_0 != maxj as ::core::ffi::c_int as Pos {
            n_vert_origs = (n_vert_origs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
        }
    }
    let entries = __caryll_allocate_clean(
        (::core::mem::size_of::<VorgEntry>() as usize).wrapping_mul(n_vert_origs as usize),
        587 as ::core::ffi::c_ulong,
    ) as *mut VorgEntry;
    let mut jj: GlyphId = 0 as GlyphId;
    for j_2 in 0..(*glyf).len() as GlyphId {
        let vori_1: Pos = vq_get_still(
            (&(*glyf))[j_2 as usize].as_deref().unwrap().vertical_origin.clone(),
        ) as Pos;
        if vori_1 != maxj as ::core::ffi::c_int as Pos {
            (*entries.offset(jj as isize)).gid = j_2;
            (*entries.offset(jj as isize)).vertical_origin = vori_1 as i16;
            jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
        }
    }
    (*font).vorg = Some(Box::new(VorgTable {
        num_vert_origin_y_metrics: n_vert_origs,
        default_vertical_origin,
        entries,
    }));
}
unsafe fn stat_ltsh(mut font: *mut Font) {
    if (*font).glyf.is_none() {
        return;
    }
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().unwrap() as *mut GlyfTable;
    let mut need_ltsh: bool = false;
    for j in 0..(*glyf).len() as GlyphId {
        if (&(*glyf))[j as usize].as_deref().unwrap().y_pel as ::core::ffi::c_int
            > 1 as ::core::ffi::c_int
        {
            need_ltsh = true;
        }
    }
    if !need_ltsh {
        return;
    }
    let num_glyphs = (*glyf).len() as GlyphId;
    let y_pels = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul(num_glyphs as usize),
        612 as ::core::ffi::c_ulong,
    ) as *mut u8;
    for j_0 in 0..(*glyf).len() as GlyphId {
        *y_pels.offset(j_0 as isize) = (&(*glyf))[j_0 as usize].as_deref().unwrap().y_pel;
    }
    (*font).ltsh = Some(Box::new(LtshTable { version: 0, num_glyphs, y_pels }));
}
pub unsafe fn otfcc_stat_font(
    mut font: *mut Font,
    mut options: *const Options,
) {
    // Raw-pointer aliases, derived once: `Font.{head,maxp,hhea,vhea}`
    // are never reassigned anywhere in this function's body (only the
    // table contents they point to are mutated, through calls like
    // `stat_glyf`/`stat_maxp` that themselves take `*mut Font`), so
    // deriving these once up front and using them exactly like the old
    // raw-pointer fields (including the `.is_null()` checks below,
    // unchanged) preserves every existing guard and control-flow path
    // without needing `Option`-aware rewriting at each of the ~35 call
    // sites below.
    let head: *mut HeadTable = (*font).head.as_deref_mut().map_or(::core::ptr::null_mut(), |h| h as *mut HeadTable);
    let maxp: *mut MaxpTable = (*font).maxp.as_deref_mut().map_or(::core::ptr::null_mut(), |m| m as *mut MaxpTable);
    let glyf: *mut GlyfTable = (*font).glyf.as_mut().map_or(::core::ptr::null_mut(), |g| g as *mut GlyfTable);
    if !glyf.is_null() && !head.is_null() {
        stat_glyf(font, options);
        if !(*options).keep_modified_time {
            (*head).modified =
                2082844800 as i64 + time(::core::ptr::null_mut::<time_t>()) as i64;
        }
    }
    if !head.is_null() && (*font).cff.is_some() {
        let mut cff: *mut CffTable = (*font).cff.as_deref_mut().unwrap() as *mut CffTable;
        if (*cff).font_b_box_bottom
            > (*head).y_min as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).font_b_box_bottom = (*head).y_min as ::core::ffi::c_double;
        }
        if (*cff).font_b_box_top < (*head).y_max as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).font_b_box_top = (*head).y_max as ::core::ffi::c_double;
        }
        if (*cff).font_b_box_left < (*head).x_min as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).font_b_box_left = (*head).x_min as ::core::ffi::c_double;
        }
        if (*cff).font_b_box_right
            < (*head).x_max as ::core::ffi::c_int as ::core::ffi::c_double
        {
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
                if (*head).units_per_em as ::core::ffi::c_int == 1000 as ::core::ffi::c_int {
                    fd.font_matrix = None;
                } else {
                    fd.font_matrix = Some(Box::new(CffFontMatrix {
                        a: (1.0f64
                            / (*head).units_per_em as ::core::ffi::c_int as ::core::ffi::c_double)
                            as Scale,
                        b: 0.0f64 as Scale,
                        c: 0.0f64 as Scale,
                        d: (1.0f64
                            / (*head).units_per_em as ::core::ffi::c_int as ::core::ffi::c_double)
                            as Scale,
                        x: (vq_neutral)(),
                        y: (vq_neutral)(),
                    }));
                }
            }
        } else if (*head).units_per_em as ::core::ffi::c_int == 1000 as ::core::ffi::c_int {
            (*cff).font_matrix = None;
        } else {
            (*cff).font_matrix = Some(Box::new(CffFontMatrix {
                a: (1.0f64
                    / (*head).units_per_em as ::core::ffi::c_int as ::core::ffi::c_double)
                    as Scale,
                b: 0.0f64 as Scale,
                c: 0.0f64 as Scale,
                d: (1.0f64
                    / (*head).units_per_em as ::core::ffi::c_int as ::core::ffi::c_double)
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
    if !glyf.is_null()
        && !maxp.is_null()
        && (*maxp).version == 0x10000 as F16Dot16
    {
        stat_maxp(font);
        if let Some(fpgm) = &(*font).fpgm {
            if fpgm.length > (*maxp).max_size_of_instructions as u32 {
                (*maxp).max_size_of_instructions = fpgm.length as u16;
            }
        }
        if let Some(prep) = &(*font).prep {
            if prep.length > (*maxp).max_size_of_instructions as u32 {
                (*maxp).max_size_of_instructions = prep.length as u16;
            }
        }
    }
    if (*font).os_2.is_some() && (*font).cmap.is_some() && !glyf.is_null() {
        stat_os_2(font, options);
    }
    if (*font).subtype == FontSubtype::Ttf {
        if !maxp.is_null() {
            (*maxp).version = 0x10000 as ::core::ffi::c_int as F16Dot16;
        }
    } else if !maxp.is_null() {
        (*maxp).version = 0x5000 as ::core::ffi::c_int as F16Dot16;
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
pub unsafe fn otfcc_unstat_font(
    mut font: *mut Font,
) {
    delete_font_table(font, crate::tag::TAG_HDMX);
    delete_font_table(font, crate::tag::TAG_HMTX);
    delete_font_table(font, crate::tag::TAG_VORG);
    delete_font_table(font, crate::tag::TAG_VMTX);
    delete_font_table(font, crate::tag::TAG_LTSH);
}
pub const FLT_MAX: ::core::ffi::c_float = __FLT_MAX__;
pub const __FLT_MAX__: ::core::ffi::c_float = 3.40282347e+38f32;
