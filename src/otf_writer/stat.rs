use crate::support::handle::{Handle, HandleState, handle_from_index};

use crate::font::model::{Font, FontSubtype};
use crate::support::options::Options;
use crate::support::primitives::{F16Dot16, GlyphId, Length, Pos, Scale, count_u16};

use crate::table::cff::CffFontMatrix;

use crate::table::ltsh::LtshTable;

use crate::table::vorg::{VorgEntry, VorgTable};

use crate::table::glyf::{
    ComponentReference, ContourList, GlyfTable, Glyph, GlyphStat, MASK_ON_CURVE, RefAnchorStatus,
    iter_glyphs,
};

use crate::table::hmtx::{HmtxTable, HorizontalMetric};

use crate::table::otl::OtlTable;
use crate::table::otl::kind::lookup_kind;

use crate::table::vmtx::{VerticalMetric, VmtxTable};

use crate::table::glyf::glyf_component_reference_empty;
use crate::vf::vq::VQ;
use crate::vf::vq::{vq_create_still, vq_get_still, vq_is_zero, vq_neutral};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StatStatus {
    NotStarted = 0,
    Doing = 1,
    Completed = 2,
}
pub const POS_MAX: f32 = f32::MAX;
pub fn stat_single_glyph(
    table: &GlyfTable,
    gr: &mut ComponentReference,
    stated: &mut [StatStatus],
    depth: u8,
    topj: GlyphId,
) -> GlyphStat {
    let mut stat: GlyphStat = GlyphStat {
        x_min: Pos::from(0),
        x_max: Pos::from(0),
        y_min: Pos::from(0),
        y_max: Pos::from(0),
        nest_depth: 0,
        n_points: 0,
        n_contours: 0,
        n_composite_points: 0,
        n_composite_contours: 0,
    };
    let j: GlyphId = gr.glyph.index;
    if i32::from(depth) >= 0xff {
        return stat;
    }
    if stated[j as usize] == StatStatus::Doing {
        tracing::warn!(
            "[Stat] Circular glyph reference found in gid {} to gid {}. The reference will be dropped.\n",
            i32::from(topj),
            i32::from(j)
        );
        stated[j as usize] = StatStatus::Completed;
        return stat;
    }
    let g: &Glyph = table[j as usize].as_deref().expect("glyph slot is filled");
    stated[j as usize] = StatStatus::Doing;
    let mut xmin: Pos = Pos::from(POS_MAX);
    let mut xmax: Pos = Pos::from(-POS_MAX);
    let mut ymin: Pos = Pos::from(POS_MAX);
    let mut ymax: Pos = Pos::from(-POS_MAX);
    let mut nest_depth: u16 = 0;
    let mut n_points: u16 = 0;
    let mut n_composite_points: u16;
    let mut n_composite_contours: u16;
    // A component's matrix maps (x, y) to (a·x + c·y, b·x + d·y): the glyf
    // chapter's x′ = xscale·x + scale10·y, y′ = scale01·x + yscale·y, with
    // `a`, `b`, `c`, `d` read from xscale, scale01, scale10, yscale.
    for contour in &g.contours {
        for p in contour {
            // `f64::round` rounds half away from zero, like C99's `round`.
            let x: Pos = (vq_get_still(gr.x.borrow().clone()) as f64
                + gr.a * vq_get_still(p.x.clone()) as f64
                + gr.c * vq_get_still(p.y.clone()) as f64)
                .round() as Pos;
            let y: Pos = (vq_get_still(gr.y.borrow().clone()) as f64
                + gr.b * vq_get_still(p.x.clone()) as f64
                + gr.d * vq_get_still(p.y.clone()) as f64)
                .round() as Pos;
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
            n_points = (i32::from(n_points) + 1) as u16;
        }
    }
    n_composite_points = n_points;
    n_composite_contours = g.contours.len() as u16;
    for rr in &g.references {
        let mut glyph_ref: ComponentReference = glyf_component_reference_empty();
        glyph_ref.glyph = handle_from_index(rr.glyph.index);
        // The component `rr` of `g`, placed where `gr` places `g`: its
        // matrix after `gr`'s, and its offset moved by `gr`'s matrix and
        // then by `gr`'s offset.
        glyph_ref.a = gr.a * rr.a + gr.c * rr.b;
        glyph_ref.b = gr.b * rr.a + gr.d * rr.b;
        glyph_ref.c = gr.a * rr.c + gr.c * rr.d;
        glyph_ref.d = gr.b * rr.c + gr.d * rr.d;
        // A scaled offset goes through the component's own matrix first.
        let (ox, oy) = (
            vq_get_still(rr.x.borrow().clone()),
            vq_get_still(rr.y.borrow().clone()),
        );
        let (ox, oy) = if rr.scaled_offset {
            (
                rr.a as Pos * ox + rr.c as Pos * oy,
                rr.b as Pos * ox + rr.d as Pos * oy,
            )
        } else {
            (ox, oy)
        };
        glyph_ref.x = std::cell::RefCell::new(vq_create_still(
            vq_get_still(gr.x.borrow().clone()) + gr.a as Pos * ox + gr.c as Pos * oy,
        ));
        glyph_ref.y = std::cell::RefCell::new(vq_create_still(
            vq_get_still(gr.y.borrow().clone()) + gr.b as Pos * ox + gr.d as Pos * oy,
        ));
        let thatstat: GlyphStat = stat_single_glyph(
            table,
            &mut glyph_ref,
            stated,
            (i32::from(depth) + 1) as u8,
            topj,
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
        if i32::from(thatstat.nest_depth) + 1 > i32::from(nest_depth) {
            nest_depth = (i32::from(thatstat.nest_depth) + 1) as u16;
        }
        n_composite_points =
            (i32::from(n_composite_points) + i32::from(thatstat.n_composite_points)) as u16;
        n_composite_contours =
            (i32::from(n_composite_contours) + i32::from(thatstat.n_composite_contours)) as u16;
    }
    if xmin > xmax {
        xmax = Pos::from(0);
        xmin = xmax;
    }
    if ymin > ymax {
        ymax = Pos::from(0);
        ymin = ymax;
    }
    stat.x_min = xmin;
    stat.x_max = xmax;
    stat.y_min = ymin;
    stat.y_max = ymax;
    stat.nest_depth = nest_depth;
    stat.n_points = n_points;
    stat.n_contours = g.contours.len() as u16;
    stat.n_composite_points = n_composite_points;
    stat.n_composite_contours = n_composite_contours;
    stated[j as usize] = StatStatus::Completed;
    stat
}
pub fn stat_glyf(font: &mut Font) {
    // A CFF outline is cubic, and its box is that of the curves.
    let cubic = font.subtype == FontSubtype::Cff;
    // `stat_font` calls this only when both tables exist.
    let head = font.head.as_deref_mut().expect("head table is present");
    let glyf = font.glyf.as_mut().expect("glyf table is present");
    let mut stated: Vec<StatStatus> = vec![StatStatus::NotStarted; glyf.len()];
    let mut xmin: Pos = Pos::from(POS_MAX);
    let mut xmax: Pos = Pos::from(-POS_MAX);
    let mut ymin: Pos = Pos::from(POS_MAX);
    let mut ymax: Pos = Pos::from(-POS_MAX);
    for j in 0..count_u16(glyf.len()) {
        let mut gr: ComponentReference = ComponentReference {
            x: std::cell::RefCell::new(VQ {
                kernel: 0.,
                shift: Vec::new(),
            }),
            y: std::cell::RefCell::new(VQ {
                kernel: 0.,
                shift: Vec::new(),
            }),
            round_to_grid: false,
            use_my_metrics: false,
            scaled_offset: false,
            glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            is_anchored: std::cell::Cell::new(RefAnchorStatus::Xy),
            inner: 0,
            outer: 0,
        };
        gr.glyph = handle_from_index(j);
        gr.x = std::cell::RefCell::new(vq_create_still(Pos::from(0)));
        gr.y = std::cell::RefCell::new(vq_create_still(Pos::from(0)));
        gr.a = Scale::from(1);
        gr.b = Scale::from(0);
        gr.c = Scale::from(0);
        gr.d = Scale::from(1);
        let mut thatstat: GlyphStat = stat_single_glyph(glyf, &mut gr, &mut stated, 0, j);
        if cubic {
            let g = glyf[j as usize].as_deref().expect("glyph slot is filled");
            if g.references.is_empty()
                && let Some([x_min, y_min, x_max, y_max]) = cubic_outline_bounds(&g.contours)
            {
                (thatstat.x_min, thatstat.y_min) = (x_min, y_min);
                (thatstat.x_max, thatstat.y_max) = (x_max, y_max);
            }
        }
        glyf[j as usize]
            .as_mut()
            .expect("glyph slot is filled")
            .stat = thatstat;
        // The font's box is that of the glyphs with an outline: an empty
        // glyph's (0, 0, 0, 0) is not a box.
        if !has_outline(&thatstat) {
            continue;
        }
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
    if xmin > xmax {
        // No glyph has an outline.
        (xmin, xmax, ymin, ymax) = (0.0, 0.0, 0.0, 0.0);
    }
    head.x_min = xmin as i16;
    head.x_max = xmax as i16;
    head.y_min = ymin as i16;
    head.y_max = ymax as i16;
}
/// The bounding box (xMin, yMin, xMax, yMax) of a cubic outline: of its
/// curves, not of their control points, as a CFF rasterizer sees it, rounded
/// outwards to whole units.
/// A contour is read as on-curve points joined by lines, or by a cubic
/// through two off-curve points; a lone off-curve point, which a cubic
/// outline does not have, counts as a point of the box. `None` when the
/// outline has no point.
fn cubic_outline_bounds(contours: &ContourList) -> Option<[Pos; 4]> {
    let mut b = [
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    ];
    let mut add = |x: f64, y: f64| {
        b[0] = b[0].min(x);
        b[1] = b[1].min(y);
        b[2] = b[2].max(x);
        b[3] = b[3].max(y);
    };
    for contour in contours {
        let pts: Vec<(f64, f64, bool)> = contour
            .iter()
            .map(|p| {
                (
                    vq_get_still(p.x.clone()),
                    vq_get_still(p.y.clone()),
                    p.on_curve & MASK_ON_CURVE != 0,
                )
            })
            .collect();
        let n = pts.len();
        // Walk the contour from an on-curve point, wrapping around.
        let Some(start) = pts.iter().position(|p| p.2) else {
            pts.iter().for_each(|p| add(p.0, p.1));
            continue;
        };
        let at = |k: usize| pts[(start + k) % n];
        let mut k = 0;
        while k < n {
            let p0 = at(k);
            add(p0.0, p0.1);
            // A cubic: this on-curve point, two off-curve points, and the
            // next on-curve point (the contour's first when it wraps).
            if p0.2 && k + 3 <= n {
                let (c1, c2, p3) = (at(k + 1), at(k + 2), at(k + 3));
                if !c1.2 && !c2.2 && p3.2 {
                    let tx = cubic_extremum_times(p0.0, c1.0, c2.0, p3.0);
                    let ty = cubic_extremum_times(p0.1, c1.1, c2.1, p3.1);
                    for t in tx.into_iter().chain(ty).flatten() {
                        add(
                            cubic_at(p0.0, c1.0, c2.0, p3.0, t),
                            cubic_at(p0.1, c1.1, c2.1, p3.1, t),
                        );
                    }
                    k += 3;
                    continue;
                }
            }
            k += 1;
        }
    }
    if b[0] > b[2] {
        return None;
    }
    // Rounded outwards, so the box is never smaller than the curves (a
    // curve's extreme is rarely a whole unit): the minimums down, the
    // maximums up, as fontTools rounds a CFF glyph's box.
    Some([b[0].floor(), b[1].floor(), b[2].ceil(), b[3].ceil()])
}
/// The value at `t` of the cubic Bezier with control values `p0`..`p3`.
fn cubic_at(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let u = 1.0 - t;
    u * u * u * p0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t * p3
}
/// The times strictly between 0 and 1 where one coordinate of a cubic Bezier
/// has a zero derivative: the roots of
/// 3[(p1 - p0)(1 - t)² + 2(p2 - p1)(1 - t)t + (p3 - p2)t²].
fn cubic_extremum_times(p0: f64, p1: f64, p2: f64, p3: f64) -> [Option<f64>; 2] {
    let (d0, d1, d2) = (p1 - p0, p2 - p1, p3 - p2);
    let a = d0 - 2.0 * d1 + d2;
    let b = 2.0 * (d1 - d0);
    let c = d0;
    let inside = |t: f64| (t > 0.0 && t < 1.0).then_some(t);
    if a.abs() < 1e-12 {
        if b.abs() < 1e-12 {
            return [None, None];
        }
        return [inside(-c / b), None];
    }
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return [None, None];
    }
    let r = disc.sqrt();
    [inside((-b + r) / (2.0 * a)), inside((-b - r) / (2.0 * a))]
}
/// Whether a glyph's flattened outline has any point, as its stat counts
/// them: the bounding box of one that does not is a placeholder of zeros.
fn has_outline(stat: &GlyphStat) -> bool {
    stat.n_composite_points > 0
}
pub fn stat_maxp(font: &mut Font) {
    // Only ever called (from `stat_font`) under a `.maxp.is_some()`/
    // `.glyf.is_some()` guard.
    let maxp = font.maxp.as_deref_mut().expect("maxp table is present");
    let mut nest_depth: u16 = 0;
    let mut n_points: u16 = 0;
    let mut n_contours: u16 = 0;
    let mut n_components: u16 = 0;
    let mut n_composite_points: u16 = 0;
    let mut n_composite_contours: u16 = 0;
    let mut inst_size: u16 = 0;
    let glyf = font.glyf.as_ref().expect("glyf table is present");
    for g in glyf.iter() {
        let g = g.as_deref().expect("glyph slot is filled");
        if !g.contours.is_empty() {
            if g.stat.n_points > n_points {
                n_points = g.stat.n_points;
            }
            if g.stat.n_contours > n_contours {
                n_contours = g.stat.n_contours;
            }
        } else if !g.references.is_empty() {
            if g.stat.n_composite_points > n_composite_points {
                n_composite_points = g.stat.n_composite_points;
            }
            if g.stat.n_composite_contours > n_composite_contours {
                n_composite_contours = g.stat.n_composite_contours;
            }
            if g.stat.nest_depth > nest_depth {
                nest_depth = g.stat.nest_depth;
            }
            if g.references.len() > n_components as usize {
                n_components = g.references.len() as u16;
            }
        }
        if g.instructions.len() as i32 > i32::from(inst_size) {
            inst_size = g.instructions.len() as u16;
        }
    }
    maxp.max_points = n_points;
    maxp.max_contours = n_contours;
    maxp.max_composite_points = n_composite_points;
    maxp.max_composite_contours = n_composite_contours;
    maxp.max_component_depth = nest_depth;
    maxp.max_component_elements = n_components;
    maxp.max_size_of_instructions = inst_size;
}
fn stat_hmtx(font: &mut Font) {
    if font.glyf.is_none() {
        return;
    }
    let glyf = font.glyf.as_mut().expect("glyf table is present");
    // Only ever called (from `stat_font`) under a `.hhea.is_some()`
    // guard; `.head` is set unconditionally by the pipeline before this
    // point (used below to update `.flags`).
    let mut count_a: GlyphId = count_u16(glyf.len());
    let mut count_k: GlyphId = 0 as GlyphId;
    let mut lsb_at_x_zero: bool = true;
    if font.subtype != FontSubtype::Cff {
        while i32::from(count_a) > 2
            && vq_get_still(
                glyf[(i32::from(count_a) - 1) as usize]
                    .as_deref()
                    .expect("glyph slot is filled")
                    .advance_width
                    .clone(),
            ) == vq_get_still(
                glyf[(i32::from(count_a) - 2) as usize]
                    .as_deref()
                    .expect("glyph slot is filled")
                    .advance_width
                    .clone(),
            )
        {
            count_a = count_a.wrapping_sub(1);
        }
        count_k = count_u16(glyf.len().wrapping_sub(count_a as usize));
    }
    // Both arrays fill sequentially within the one loop below (`j < count_a`
    // covers `metrics`, the rest covers `left_side_bearing` in order).
    let mut metrics: Vec<HorizontalMetric> = Vec::with_capacity(count_a as usize);
    let mut left_side_bearing: Vec<Pos> = Vec::with_capacity(count_k as usize);
    let mut min_lsb: Pos = Pos::from(0x7fff);
    let mut min_rsb: Pos = Pos::from(0x7fff);
    let mut max_extent: Pos = Pos::from(-0x8000);
    let mut max_width: Length = Length::from(0);
    for (j, slot) in glyf.iter_mut().enumerate() {
        let g = slot.as_mut().expect("glyph slot is filled");
        if vq_is_zero(g.horizontal_origin.clone(), 1.0 / 1000.0) {
            g.horizontal_origin = vq_neutral();
        } else {
            lsb_at_x_zero = false;
        }
        let hori: Pos = vq_get_still(g.horizontal_origin.clone()) as Pos;
        let advw: Pos = vq_get_still(g.advance_width.clone()) as Pos;
        let lsb: Pos = g.stat.x_min - hori;
        let rsb: Pos = advw + hori - g.stat.x_max;
        if j < count_a as usize {
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
        // The side bearing minimums and the extent are over the glyphs with
        // an outline, as `hhea` defines them.
        if !has_outline(&g.stat) {
            continue;
        }
        if lsb < min_lsb {
            min_lsb = lsb;
        }
        if rsb < min_rsb {
            min_rsb = rsb;
        }
        if g.stat.x_max - hori > max_extent {
            max_extent = g.stat.x_max - hori;
        }
    }
    if max_extent == Pos::from(-0x8000) {
        // No glyph has an outline.
        (min_lsb, min_rsb, max_extent) = (0.0, 0.0, 0.0);
    }
    let hhea = font.hhea.as_deref_mut().expect("hhea table is present");
    hhea.number_of_metrics = count_a as u16;
    hhea.min_left_side_bearing = min_lsb as i16;
    hhea.min_right_side_bearing = min_rsb as i16;
    hhea.x_max_extent = max_extent as i16;
    hhea.advance_width_max = max_width as u16;
    font.hmtx = Some(Box::new(HmtxTable {
        metrics,
        left_side_bearing,
    }));
    let head = font.head.as_deref_mut().expect("head table is present");
    head.flags = (i32::from(head.flags) & !0x2 | (if lsb_at_x_zero { 0x2 } else { 0 })) as u16;
}
fn stat_vmtx(font: &mut Font, options: &Options) {
    if font.glyf.is_none() {
        return;
    }
    let glyf = font.glyf.as_mut().expect("glyf table is present");
    let mut count_a: GlyphId = count_u16(glyf.len());
    let mut count_k: GlyphId = 0 as GlyphId;
    if !(font.subtype == FontSubtype::Cff && !options.cff_short_vmtx) {
        while i32::from(count_a) > 2
            && vq_get_still(
                glyf[(i32::from(count_a) - 1) as usize]
                    .as_deref()
                    .expect("glyph slot is filled")
                    .advance_height
                    .clone(),
            ) == vq_get_still(
                glyf[(i32::from(count_a) - 2) as usize]
                    .as_deref()
                    .expect("glyph slot is filled")
                    .advance_height
                    .clone(),
            )
        {
            count_a = count_a.wrapping_sub(1);
        }
        count_k = count_u16(glyf.len().wrapping_sub(count_a as usize));
    }
    // Same "Vec absorbs both sequential halves of the loop" shape as
    // `stat_hmtx`'s `metrics`/`left_side_bearing`.
    let mut metrics: Vec<VerticalMetric> = Vec::with_capacity(count_a as usize);
    let mut top_side_bearing: Vec<Pos> = Vec::with_capacity(count_k as usize);
    let mut min_tsb: Pos = Pos::from(0x7fff);
    let mut min_bsb: Pos = Pos::from(0x7fff);
    let mut max_extent: Pos = Pos::from(-0x8000);
    let mut max_height: Length = Length::from(0);
    for (j, slot) in glyf.iter().enumerate() {
        let g = slot.as_deref().expect("glyph slot is filled");
        let vori: Pos = vq_get_still(g.vertical_origin.clone()) as Pos;
        let advh: Pos = vq_get_still(g.advance_height.clone()) as Pos;
        let tsb: Pos = vori - g.stat.y_max;
        let bsb: Pos = g.stat.y_min - vori + advh;
        if j < count_a as usize {
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
        if vori - g.stat.y_min > max_extent {
            max_extent = vori - g.stat.y_min;
        }
    }
    // Only ever called (from `stat_font`) under a `.vhea.is_some()`
    // guard.
    let vhea = font.vhea.as_deref_mut().expect("vhea table is present");
    vhea.num_of_long_ver_metrics = count_a as u16;
    vhea.min_top = min_tsb as i16;
    vhea.min_bottom = min_bsb as i16;
    vhea.y_max_extent = max_extent as i16;
    vhea.advance_height_max = max_height as i16;
    font.vmtx = Some(Box::new(VmtxTable {
        metrics,
        top_side_bearing,
    }));
}
/// OS/2 `ulUnicodeRange1`..`ulUnicodeRange4` (OpenType spec, OS/2 table):
/// each entry is a bit number (0..=122) and the code point ranges that set it
/// when the cmap maps any code point inside one of them. Bit `n` lives in
/// `ulUnicodeRange{n / 32 + 1}`, at position `n % 32`.
static UNICODE_RANGE_BITS: [(u32, &[(i32, i32)]); 123] = [
    (0, &[(0x0000, 0x007F)]),
    (1, &[(0x0080, 0x00FF)]),
    (2, &[(0x0100, 0x017F)]),
    (3, &[(0x0180, 0x024F)]),
    (4, &[(0x0250, 0x02AF), (0x1D00, 0x1D7F), (0x1D80, 0x1DBF)]),
    (5, &[(0x02B0, 0x02FF), (0xA700, 0xA71F)]),
    (6, &[(0x0300, 0x036F), (0x1DC0, 0x1DFF)]),
    (7, &[(0x0370, 0x03FF)]),
    (8, &[(0x2C80, 0x2CFF)]),
    (
        9,
        &[
            (0x0400, 0x04FF),
            (0x0500, 0x052F),
            (0x2DE0, 0x2DFF),
            (0xA640, 0xA69F),
        ],
    ),
    (10, &[(0x0530, 0x058F)]),
    (11, &[(0x0590, 0x05FF)]),
    (12, &[(0xA500, 0xA63F)]),
    (13, &[(0x0600, 0x06FF), (0x0750, 0x077F)]),
    (14, &[(0x07C0, 0x07FF)]),
    (15, &[(0x0900, 0x097F)]),
    (16, &[(0x0980, 0x09FF)]),
    (17, &[(0x0A00, 0x0A7F)]),
    (18, &[(0x0A80, 0x0AFF)]),
    (19, &[(0x0B00, 0x0B7F)]),
    (20, &[(0x0B80, 0x0BFF)]),
    (21, &[(0x0C00, 0x0C7F)]),
    (22, &[(0x0C80, 0x0CFF)]),
    (23, &[(0x0D00, 0x0D7F)]),
    (24, &[(0x0E00, 0x0E7F)]),
    (25, &[(0x0E80, 0x0EFF)]),
    (26, &[(0x10A0, 0x10FF), (0x2D00, 0x2D2F)]),
    (27, &[(0x1B00, 0x1B7F)]),
    (28, &[(0x1100, 0x11FF)]),
    (29, &[(0x1E00, 0x1EFF), (0x2C60, 0x2C7F), (0xA720, 0xA7FF)]),
    (30, &[(0x1F00, 0x1FFF)]),
    (31, &[(0x2000, 0x206F), (0x2E00, 0x2E7F)]),
    (32, &[(0x2070, 0x209F)]),
    (33, &[(0x20A0, 0x20CF)]),
    (34, &[(0x20D0, 0x20FF)]),
    (35, &[(0x2100, 0x214F)]),
    (36, &[(0x2150, 0x218F)]),
    (
        37,
        &[
            (0x2190, 0x21FF),
            (0x27F0, 0x27FF),
            (0x2900, 0x297F),
            (0x2B00, 0x2BFF),
        ],
    ),
    (
        38,
        &[
            (0x2200, 0x22FF),
            (0x2A00, 0x2AFF),
            (0x27C0, 0x27EF),
            (0x2980, 0x29FF),
        ],
    ),
    (39, &[(0x2300, 0x23FF)]),
    (40, &[(0x2400, 0x243F)]),
    (41, &[(0x2440, 0x245F)]),
    (42, &[(0x2460, 0x24FF)]),
    (43, &[(0x2500, 0x257F)]),
    (44, &[(0x2580, 0x259F)]),
    (45, &[(0x25A0, 0x25FF)]),
    (46, &[(0x2600, 0x26FF)]),
    (47, &[(0x2700, 0x27BF)]),
    (48, &[(0x3000, 0x303F)]),
    (49, &[(0x3040, 0x309F)]),
    (50, &[(0x30A0, 0x30FF), (0x31F0, 0x31FF)]),
    (51, &[(0x3100, 0x312F), (0x31A0, 0x31BF)]),
    (52, &[(0x3130, 0x318F)]),
    (53, &[(0xA840, 0xA87F)]),
    (54, &[(0x3200, 0x32FF)]),
    (55, &[(0x3300, 0x33FF)]),
    (56, &[(0xAC00, 0xD7AF)]),
    (57, &[(0xD800, 0xDFFF), (0x10000, i32::MAX)]), // surrogates, and every code point beyond the BMP
    (58, &[(0x10900, 0x1091F)]),
    (
        59,
        &[
            (0x4E00, 0x9FFF),
            (0x2E80, 0x2EFF),
            (0x2F00, 0x2FDF),
            (0x2FF0, 0x2FFF),
            (0x3400, 0x4DBF),
            (0x20000, 0x2F7FF),
            (0x3190, 0x319F),
        ],
    ),
    (60, &[(0xE000, 0xF8FF)]),
    (
        61,
        &[(0x31C0, 0x31EF), (0xF900, 0xFAFF), (0x2F800, 0x2FA1F)],
    ),
    (62, &[(0xFB00, 0xFB4F)]),
    (63, &[(0xFB50, 0xFDFF)]),
    (64, &[(0xFE20, 0xFE2F)]),
    (65, &[(0xFE10, 0xFE1F), (0xFE30, 0xFE4F)]),
    (66, &[(0xFE50, 0xFE6F)]),
    (67, &[(0xFE70, 0xFEFF)]),
    (68, &[(0xFF00, 0xFFEF)]),
    (69, &[(0xFFF0, 0xFFFF)]),
    (70, &[(0x0F00, 0x0FFF)]),
    (71, &[(0x0700, 0x074F)]),
    (72, &[(0x0780, 0x07BF)]),
    (73, &[(0x0D80, 0x0DFF)]),
    (74, &[(0x1000, 0x109F)]),
    (75, &[(0x1200, 0x137F), (0x1380, 0x139F), (0x2D80, 0x2DDF)]),
    (76, &[(0x13A0, 0x13FF)]),
    (77, &[(0x1400, 0x167F)]),
    (78, &[(0x1680, 0x169F)]),
    (79, &[(0x16A0, 0x16FF)]),
    (80, &[(0x1780, 0x17FF), (0x19E0, 0x19FF)]),
    (81, &[(0x1800, 0x18AF)]),
    (82, &[(0x2800, 0x28FF)]),
    (83, &[(0xA000, 0xA48F), (0xA490, 0xA4CF)]),
    (
        84,
        &[
            (0x1700, 0x171F),
            (0x1720, 0x173F),
            (0x1740, 0x175F),
            (0x1760, 0x177F),
        ],
    ),
    (85, &[(0x10300, 0x1032F)]),
    (86, &[(0x10330, 0x1034F)]),
    (87, &[(0x10400, 0x1044F)]),
    (
        88,
        &[(0x1D000, 0x1D0FF), (0x1D100, 0x1D1FF), (0x1D200, 0x1D24F)],
    ),
    (89, &[(0x1D400, 0x1D7FF)]),
    (90, &[(0xFF000, 0xFFFFD), (0x100000, 0x10FFFD)]),
    (91, &[(0xFE00, 0xFE0F), (0xE0100, 0xE01EF)]),
    (92, &[(0xE0000, 0xE007F)]),
    (93, &[(0x1900, 0x194F)]),
    (94, &[(0x1950, 0x197F)]),
    (95, &[(0x1980, 0x19DF)]),
    (96, &[(0x1A00, 0x1A1F)]),
    (97, &[(0x2C00, 0x2C5F)]),
    (98, &[(0x2D30, 0x2D7F)]),
    (99, &[(0x4DC0, 0x4DFF)]),
    (100, &[(0xA800, 0xA82F)]),
    (
        101,
        &[(0x10000, 0x1007F), (0x10080, 0x100FF), (0x10100, 0x1013F)],
    ),
    (102, &[(0x10140, 0x1018F)]),
    (103, &[(0x10380, 0x1039F)]),
    (104, &[(0x103A0, 0x103DF)]),
    (105, &[(0x10450, 0x1047F)]),
    (106, &[(0x10480, 0x104AF)]),
    (107, &[(0x10800, 0x1083F)]),
    (108, &[(0x10A00, 0x10A5F)]),
    (109, &[(0x1D300, 0x1D35F)]),
    (110, &[(0x12000, 0x123FF), (0x12400, 0x1247F)]),
    (111, &[(0x1D360, 0x1D37F)]),
    (112, &[(0x1B80, 0x1BBF)]),
    (113, &[(0x1C00, 0x1C4F)]),
    (114, &[(0x1C50, 0x1C7F)]),
    (115, &[(0xA880, 0xA8DF)]),
    (116, &[(0xA900, 0xA92F)]),
    (117, &[(0xA930, 0xA95F)]),
    (118, &[(0xAA00, 0xAA5F)]),
    (119, &[(0x10190, 0x101CF)]),
    (120, &[(0x101D0, 0x101FF)]),
    (
        121,
        &[(0x102A0, 0x102DF), (0x10280, 0x1029F), (0x10920, 0x1093F)],
    ),
    (122, &[(0x1F030, 0x1F09F), (0x1F000, 0x1F02F)]),
];

/// The `ulUnicodeRange1`..`4` bits that code point `u` sets.
fn unicode_range_bits(u: i32) -> [u32; 4] {
    let mut ranges = [0u32; 4];
    for &(bit, bit_ranges) in &UNICODE_RANGE_BITS {
        if bit_ranges.iter().any(|&(lo, hi)| (lo..=hi).contains(&u)) {
            ranges[(bit / 32) as usize] |= 1 << (bit % 32);
        }
    }
    ranges
}
fn stat_os_2_unicode_ranges(font: &mut Font, options: &Options) {
    let mut ranges = [0u32; 4];
    let mut min_unicode: i32 = 0xffff;
    let mut max_unicode: i32 = 0;
    for &u in font
        .cmap
        .as_ref()
        .expect("cmap table is present")
        .unicodes
        .keys()
    {
        min_unicode = min_unicode.min(u);
        max_unicode = max_unicode.max(u);
        for (range, bits) in ranges.iter_mut().zip(unicode_range_bits(u)) {
            *range |= bits;
        }
    }
    let [u1, u2, u3, u4] = ranges;
    let os_2 = font.os_2.as_deref_mut().expect("os_2 table is present");
    if !options.keep_unicode_ranges {
        os_2.ul_unicode_range1 = u1;
        os_2.ul_unicode_range2 = u2;
        os_2.ul_unicode_range3 = u3;
        os_2.ul_unicode_range4 = u4;
    }
    if min_unicode < 0x10000 {
        os_2.us_first_char_index = min_unicode as u16;
    } else {
        os_2.us_first_char_index = 0xffff;
    }
    if max_unicode < 0x10000 {
        os_2.us_last_char_index = max_unicode as u16;
    } else {
        os_2.us_last_char_index = 0xffff;
    };
}
fn stat_os_2_average_width(font: &mut Font, options: &Options) {
    if options.keep_average_char_width {
        return;
    }
    // Only ever called (from `stat_font`, via `stat_os_2`) under a
    // `.glyf.is_some()` guard.
    let glyf = font.glyf.as_ref().expect("glyf table is present");
    let mut total_width: u32 = 0;
    for slot in glyf.iter() {
        let adw: Pos = vq_get_still(
            slot.as_deref()
                .expect("glyph slot is filled")
                .advance_width
                .clone(),
        ) as Pos;
        if adw > Pos::from(0) {
            total_width = (Pos::from(total_width) + adw) as u32;
        }
    }
    let glyf_len = glyf.len();
    let os_2 = font.os_2.as_deref_mut().expect("os_2 table is present");
    // `glyf_len` is attacker-controlled (a JSON `glyf` table can declare
    // zero glyphs) and was fed straight into `wrapping_div` -- unlike
    // `MIN / -1`, division by zero always panics regardless of which
    // division operation is used. A font with no glyphs has no average
    // width to compute, so this is the same "nothing to divide" guard
    // `stat_cff_widths` a few functions down already uses for its own
    // `nnsum.wrapping_div(nn)`.
    os_2.x_avg_char_width = if glyf_len == 0 {
        0
    } else {
        (total_width as usize).wrapping_div(glyf_len) as i16
    };
}
fn stat_max_context_otl(table: &OtlTable) -> u16 {
    let mut maxc: u16 = 1;
    // This runs on the post-consolidation table (part of `otf_writer.rs`'s
    // build path), so a `None` slot here is a real, possible hole
    // consolidation punched, not a bug -- skip it, same as everywhere else
    // that reads `OtlTable.lookups` post-consolidation.
    for lookup in table.lookups.iter().flatten() {
        if let Some(kind) = lookup_kind(lookup.lookup_type) {
            kind.raise_max_context(lookup, &mut maxc);
        }
    }
    maxc
}
fn stat_max_context(font: &mut Font) {
    let mut maxc: u16 = 1;
    if let Some(gsub) = font.gsub.as_deref() {
        let maxc_gsub: u16 = stat_max_context_otl(gsub);
        if i32::from(maxc_gsub) > i32::from(maxc) {
            maxc = maxc_gsub;
        }
    }
    if let Some(gpos) = font.gpos.as_deref() {
        let maxc_gpos: u16 = stat_max_context_otl(gpos);
        if i32::from(maxc_gpos) > i32::from(maxc) {
            maxc = maxc_gpos;
        }
    }
    font.os_2
        .as_deref_mut()
        .expect("os_2 table is present")
        .us_max_context = maxc;
}
fn stat_os_2(font: &mut Font, options: &Options) {
    stat_os_2_unicode_ranges(font, options);
    stat_os_2_average_width(font, options);
    stat_max_context(font);
}
pub const MAX_STAT_METRIC: i32 = 4096;
fn stat_cff_widths(font: &mut Font) {
    if font.glyf.is_none() || font.cff.is_none() {
        return;
    }
    let glyf = font.glyf.as_ref().expect("glyf table is present");
    let mut frequency: Vec<u32> = vec![0u32; MAX_STAT_METRIC as usize];
    for g in iter_glyphs(glyf) {
        let int_width: u16 = vq_get_still(g.advance_width.clone()) as u16;
        if i32::from(int_width) < MAX_STAT_METRIC {
            frequency[int_width as usize] = frequency[int_width as usize].wrapping_add(1);
        }
    }
    let mut maxfreq: u16 = 0;
    let mut maxj: u16 = 0;
    for (width, &count) in frequency.iter().enumerate() {
        if count > u32::from(maxfreq) {
            maxfreq = count as u16;
            maxj = width as u16;
        }
    }
    let mut nn: u16 = 0;
    let mut nnsum: u32 = 0;
    for g in iter_glyphs(glyf) {
        let adw: Pos = vq_get_still(g.advance_width.clone()) as Pos;
        if adw != Pos::from(i32::from(maxj)) {
            nn = (i32::from(nn) + 1) as u16;
            nnsum = (Pos::from(nnsum) + adw) as u32;
        }
    }
    let mut nominal_width_x: i16 = 0;
    if i32::from(nn) > 0 {
        nominal_width_x = nnsum.wrapping_div(u32::from(nn)) as i16;
    }
    let cff = font.cff.as_deref_mut().expect("cff table is present");
    if let Some(pd) = cff.private_dict.as_deref_mut() {
        pd.default_width_x = f64::from(maxj);
        if i32::from(nn) != 0 {
            pd.nominal_width_x = f64::from(nominal_width_x);
        }
    }
    for fd in cff.fd_array.iter_mut() {
        // A JSON `fdArray` entry that is not an object (or has no
        // `privates`) has no private dict; the build step writes a default
        // one for it, so there is nothing to update here.
        if let Some(pd) = fd.private_dict.as_deref_mut() {
            pd.default_width_x = f64::from(maxj);
            pd.nominal_width_x = f64::from(nominal_width_x);
        }
    }
}
fn stat_vorg(font: &mut Font) {
    if font.glyf.is_none() || font.cff.is_none() || font.vhea.is_none() || font.vmtx.is_none() {
        return;
    }
    let glyf = font.glyf.as_ref().expect("glyf table is present");
    let mut frequency: Vec<u32> = vec![0u32; MAX_STAT_METRIC as usize];
    for g in iter_glyphs(glyf) {
        let vori: Pos = vq_get_still(g.vertical_origin.clone()) as Pos;
        if vori >= Pos::from(0) && vori < Pos::from(MAX_STAT_METRIC) {
            frequency[vori as u16 as usize] = frequency[vori as u16 as usize].wrapping_add(1);
        }
    }
    let mut maxfreq: u32 = 0;
    let mut maxj: GlyphId = 0 as GlyphId;
    for (origin, &count) in frequency.iter().enumerate() {
        if count > maxfreq {
            maxfreq = count;
            maxj = origin as GlyphId;
        }
    }
    let default_vertical_origin = Pos::from(maxj);
    let mut n_vert_origs: GlyphId = 0 as GlyphId;
    for g in iter_glyphs(glyf) {
        let vertical_origin: Pos = vq_get_still(g.vertical_origin.clone()) as Pos;
        if vertical_origin != Pos::from(i32::from(maxj)) {
            n_vert_origs = (i32::from(n_vert_origs) + 1) as GlyphId;
        }
    }
    let mut entries: Vec<VorgEntry> = Vec::with_capacity(n_vert_origs as usize);
    for (gid, g) in iter_glyphs(glyf).enumerate() {
        let vori_1: Pos = vq_get_still(g.vertical_origin.clone()) as Pos;
        if vori_1 != Pos::from(i32::from(maxj)) {
            entries.push(VorgEntry {
                gid: gid as GlyphId,
                vertical_origin: vori_1 as i16,
            });
        }
    }
    font.vorg = Some(Box::new(VorgTable {
        num_vert_origin_y_metrics: n_vert_origs,
        default_vertical_origin,
        entries,
    }));
}
fn stat_ltsh(font: &mut Font) {
    if font.glyf.is_none() {
        return;
    }
    let glyf = font.glyf.as_ref().expect("glyf table is present");
    // A yPel of 1 (linear at every size) is information too; only a table
    // with nothing but zeros, which says nothing, is left out.
    if !iter_glyphs(glyf).any(|g| g.y_pel > 0) {
        return;
    }
    let num_glyphs = count_u16(glyf.len());
    let y_pels: Vec<u8> = iter_glyphs(glyf).map(|g| g.y_pel).collect();
    font.ltsh = Some(Box::new(LtshTable {
        version: 0,
        num_glyphs,
        y_pels,
    }));
}
/// The `FontMatrix` a CFF font or subfont needs for `units_per_em`: none for
/// the CFF default of 1000, otherwise `1/units_per_em` on both axes.
fn cff_font_matrix_for(units_per_em: u16) -> Option<Box<CffFontMatrix>> {
    if i32::from(units_per_em) == 1000 {
        return None;
    }
    let scale = (1.0 / f64::from(i32::from(units_per_em))) as Scale;
    Some(Box::new(CffFontMatrix {
        a: scale,
        b: 0.0 as Scale,
        c: 0.0 as Scale,
        d: scale,
        x: vq_neutral(),
        y: vq_neutral(),
    }))
}
/// Stamps `head.modified` with the current time, unless the options ask to
/// keep the one the font has.
fn stat_head_modified(font: &mut Font, options: &Options) {
    if options.keep_modified_time {
        return;
    }
    // Whole seconds since 1970-01-01 UTC; `unwrap_or(0)` only matters for a
    // system clock set before the epoch.
    let now = ::std::time::SystemTime::now()
        .duration_since(::std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    font.head
        .as_deref_mut()
        .expect("head table is present")
        .modified = 2082844800 + now;
}
/// Brings a CFF font's header in line with the glyphs and `head`: grows the
/// font bounding box to cover `head`'s, sets the CID count and the font
/// matrices from `units_per_em`, and recomputes the default and nominal
/// widths. Called only when both `head` and `cff` exist.
fn stat_cff_header(font: &mut Font) {
    let head = font.head.as_deref().expect("head table is present");
    let (head_y_min, head_y_max) = (head.y_min, head.y_max);
    let (head_x_min, head_x_max) = (head.x_min, head.x_max);
    let units_per_em = head.units_per_em;
    let glyf_len = font.glyf.as_ref().map(|g| g.len() as u32);
    let cff = font.cff.as_deref_mut().expect("cff table is present");
    if cff.font_b_box_bottom > f64::from(i32::from(head_y_min)) {
        cff.font_b_box_bottom = f64::from(i32::from(head_y_min));
    }
    if cff.font_b_box_top < f64::from(i32::from(head_y_max)) {
        cff.font_b_box_top = f64::from(i32::from(head_y_max));
    }
    // The box only grows to take in the glyphs': the left edge moves left.
    if cff.font_b_box_left > f64::from(i32::from(head_x_min)) {
        cff.font_b_box_left = f64::from(i32::from(head_x_min));
    }
    if cff.font_b_box_right < f64::from(i32::from(head_x_max)) {
        cff.font_b_box_right = f64::from(i32::from(head_x_max));
    }
    if let Some(len) = glyf_len
        && cff.is_cid
    {
        cff.cid_count = len;
    }
    if cff.is_cid {
        // In a CID font the matrix lives on each subfont, not the top dict.
        cff.font_matrix = None;
        for fd in cff.fd_array.iter_mut() {
            fd.font_matrix = cff_font_matrix_for(units_per_em);
        }
    } else {
        cff.font_matrix = cff_font_matrix_for(units_per_em);
    }
    stat_cff_widths(font);
}
/// Sets the glyph count in `maxp`, and for a version 1.0 `maxp`
/// recomputes the glyph-derived limits, raising the instruction size to
/// cover `fpgm` and `prep`.
fn stat_glyph_counts_and_maxp(font: &mut Font) {
    if let Some(len) = font.glyf.as_ref().map(|g| g.len() as u16)
        && let Some(maxp) = font.maxp.as_deref_mut()
    {
        maxp.num_glyphs = len;
    }
    let maxp_version_is_10000 =
        font.maxp.as_deref().map(|m| m.version) == Some(0x10000 as F16Dot16);
    if font.glyf.is_some() && font.maxp.is_some() && maxp_version_is_10000 {
        stat_maxp(font);
        let fpgm_length = font.fpgm.as_ref().map(|f| f.bytes.len() as u32);
        let prep_length = font.prep.as_ref().map(|p| p.bytes.len() as u32);
        let maxp = font.maxp.as_deref_mut().expect("maxp table is present");
        for length in [fpgm_length, prep_length].into_iter().flatten() {
            if length > u32::from(maxp.max_size_of_instructions) {
                maxp.max_size_of_instructions = length as u16;
            }
        }
    }
}
/// Recomputes everything derived from the glyphs before the font is built:
/// `head.modified`, the CFF header, the `maxp` counts, `OS/2`, and
/// the metric tables (`hmtx`, `vmtx`, `VORG`, `LTSH`).
pub fn stat_font(font: &mut Font, options: &Options) {
    if font.glyf.is_some() && font.head.is_some() {
        stat_glyf(font);
        stat_head_modified(font, options);
    }
    if font.head.is_some() && font.cff.is_some() {
        stat_cff_header(font);
    }
    stat_glyph_counts_and_maxp(font);
    if font.os_2.is_some() && font.cmap.is_some() && font.glyf.is_some() {
        stat_os_2(font, options);
    }
    if font.subtype == FontSubtype::Ttf {
        if let Some(maxp) = font.maxp.as_deref_mut() {
            maxp.version = 0x10000 as F16Dot16;
        }
    } else if let Some(maxp) = font.maxp.as_deref_mut() {
        maxp.version = 0x5000 as F16Dot16;
    }
    if font.glyf.is_some() && font.hhea.is_some() {
        stat_hmtx(font);
    }
    if font.glyf.is_some() && font.vhea.is_some() {
        stat_vmtx(font, options);
        stat_vorg(font);
    }
    stat_ltsh(font);
}
/// Drops the tables `stat_font` computed from the glyphs, so the font is
/// back to its consolidated form once it has been written.
pub fn unstat_font(font: &mut Font) {
    font.hmtx = None;
    font.vorg = None;
    font.vmtx = None;
    font.ltsh = None;
}

#[cfg(test)]
mod stat_os_2_average_width_tests {
    use super::*;
    use crate::font::model::Font;
    use crate::table::os_2::Os2Table;

    // A `glyf` table can legitimately be present-but-empty (a JSON font
    // with `"glyf": {}`) -- `glyf_len` then reaches `wrapping_div` as 0,
    // which panics (division by zero always panics, regardless of which
    // division operation is used, unlike `wrapping_div`'s only other
    // special case, `MIN / -1`). Found by fuzzing. A font with no glyphs
    // has no average width to compute, so 0 is the natural result -- the
    // same "nothing to divide" guard `stat_cff_widths` already has for
    // its own `nnsum.wrapping_div(nn)`.
    #[test]
    fn empty_glyf_table_does_not_panic_and_yields_a_zero_average() {
        let mut font: Box<Font> = Box::default();
        font.glyf = Some(Vec::new());
        font.os_2 = Some(Box::new(Os2Table {
            version: 0,
            x_avg_char_width: -1,
            us_weight_class: 0,
            us_width_class: 0,
            fs_type: 0,
            y_subscript_x_size: 0,
            y_subscript_y_size: 0,
            y_subscript_x_offset: 0,
            y_subscript_y_offset: 0,
            y_supscript_x_size: 0,
            y_supscript_y_size: 0,
            y_supscript_x_offset: 0,
            y_supscript_y_offset: 0,
            y_strikeout_size: 0,
            y_strikeout_position: 0,
            s_family_class: 0,
            panose: [0; 10],
            ul_unicode_range1: 0,
            ul_unicode_range2: 0,
            ul_unicode_range3: 0,
            ul_unicode_range4: 0,
            ach_vend_id: [0; 4],
            fs_selection: 0,
            us_first_char_index: 0,
            us_last_char_index: 0,
            s_typo_ascender: 0,
            s_typo_descender: 0,
            s_typo_line_gap: 0,
            us_win_ascent: 0,
            us_win_descent: 0,
            ul_code_page_range1: 0,
            ul_code_page_range2: 0,
            sx_height: 0,
            s_cap_height: 0,
            us_default_char: 0,
            us_break_char: 0,
            us_max_context: 0,
            us_lower_optical_point_size: 0,
            us_upper_optical_point_size: 0,
        }));
        let options = Options::default();
        stat_os_2_average_width(&mut font, &options);
        assert_eq!(font.os_2.as_deref().unwrap().x_avg_char_width, 0);
    }
}

// `stat_glyf` relies on `f64::round` rounding half away from zero (C99
// `round`'s contract, which upstream otfcc used), pinned here against that
// documented contract rather than a live libc comparison, the same choice
// `libcff/writer.rs`'s own `modf_tests` module makes.
#[cfg(test)]
mod round_tests {
    #[test]
    fn f64_round_matches_round_contract_half_away_from_zero() {
        assert_eq!(0.4_f64.round(), 0.0);
        assert_eq!(0.5_f64.round(), 1.0);
        assert_eq!(0.6_f64.round(), 1.0);
        assert_eq!((-0.4_f64).round(), -0.0);
        assert_eq!((-0.5_f64).round(), -1.0);
        assert_eq!((-0.6_f64).round(), -1.0);
        assert_eq!(2.5_f64.round(), 3.0);
        assert_eq!((-2.5_f64).round(), -3.0);
        assert_eq!(0.0_f64.round().to_bits(), 0.0_f64.to_bits());
        assert_eq!((-0.0_f64).round().to_bits(), (-0.0_f64).to_bits());
        assert_eq!(f64::INFINITY.round(), f64::INFINITY);
        assert_eq!(f64::NEG_INFINITY.round(), f64::NEG_INFINITY);
        assert!(f64::NAN.round().is_nan());
    }
}

#[cfg(test)]
mod unicode_range_tests {
    use super::*;

    fn has_bit(bits: [u32; 4], bit: u32) -> bool {
        bits[(bit / 32) as usize] & (1 << (bit % 32)) != 0
    }

    #[test]
    fn table_lists_each_bit_once_with_well_formed_ranges() {
        for (i, &(bit, ranges)) in UNICODE_RANGE_BITS.iter().enumerate() {
            assert_eq!(
                bit as usize, i,
                "bits are listed in order, 0..=122, once each"
            );
            assert!(!ranges.is_empty());
            assert!(ranges.iter().all(|&(lo, hi)| lo <= hi), "bit {bit}");
        }
    }

    #[test]
    fn known_code_points_set_their_bits() {
        assert!(has_bit(unicode_range_bits(0x41), 0)); // Basic Latin
        assert!(has_bit(unicode_range_bits(0xE9), 1)); // Latin-1 Supplement
        assert!(has_bit(unicode_range_bits(0x0301), 6)); // Combining Diacritical Marks
        assert!(has_bit(unicode_range_bits(0x1DC0), 6)); // ... and its supplement
        assert!(has_bit(unicode_range_bits(0xD800), 57)); // a surrogate
        assert!(has_bit(unicode_range_bits(0x1F600), 57)); // any non-BMP code point
        assert!(!has_bit(unicode_range_bits(0xFFFF), 57));
        assert_eq!(unicode_range_bits(0x41), [1, 0, 0, 0], "only bit 0");
    }
}

#[cfg(test)]
mod cubic_outline_bounds_tests {
    use super::*;
    use crate::table::glyf::Point;

    fn point(x: f64, y: f64, on: bool) -> Point {
        Point {
            x: vq_create_still(x),
            y: vq_create_still(y),
            on_curve: i8::from(on),
        }
    }

    // A cubic's control points lie outside its curve: the box is that of
    // the curve, rounded outwards.
    #[test]
    fn the_box_follows_the_curve_not_its_control_points() {
        // (0, 0) -> (0, 100), bulging right through controls at x = 100:
        // the curve reaches x = 75 at t = 1/2.
        let contour = vec![
            point(0.0, 0.0, true),
            point(100.0, 0.0, false),
            point(100.0, 100.0, false),
            point(0.0, 100.0, true),
        ];
        assert_eq!(
            cubic_outline_bounds(&vec![contour]),
            Some([0.0, 0.0, 75.0, 100.0])
        );
    }

    // The closing segment can be a cubic back to the contour's first point,
    // and a fractional extreme is rounded outwards.
    #[test]
    fn a_curve_back_to_the_start_counts_and_rounds_outwards() {
        let contour = vec![
            point(0.0, 0.0, true),
            point(10.0, 0.0, true),
            point(10.0, -11.0, false),
            point(0.0, -11.0, false),
        ];
        // The curve dips to y = -8.25.
        assert_eq!(
            cubic_outline_bounds(&vec![contour]),
            Some([0.0, -9.0, 10.0, 0.0])
        );
    }

    #[test]
    fn an_outline_without_points_has_no_box() {
        assert_eq!(cubic_outline_bounds(&Vec::new()), None);
    }
}
