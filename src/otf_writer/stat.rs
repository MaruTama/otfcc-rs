use crate::support::handle::{Handle, HandleState, handle_from_index};


use crate::font::caryll_font::{Font, FontSubtype};
use crate::support::options::Options;
use crate::support::primitives::{F16Dot16, GlyphId, Length, Pos, Scale, count_u16};

use crate::table::cff::CffFontMatrix;

use crate::table::ltsh::LtshTable;

use crate::table::vorg::{VorgEntry, VorgTable};

use crate::table::glyf::{
    ComponentReference, GlyfTable, Glyph, GlyphStat, RefAnchorStatus, iter_glyphs,
};

use crate::table::hmtx::{HmtxTable, HorizontalMetric};

use crate::table::otl::OtlTable;
use crate::table::otl::kind::lookup_kind;

use crate::table::vmtx::{VerticalMetric, VmtxTable};

use crate::font::caryll_font::delete_font_table;
use crate::table::glyf::glyf_component_reference_init;
use crate::vf::vq::VQ;
use crate::vf::vq::{vq_create_still, vq_get_still, vq_is_zero, vq_neutral};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StatStatus {
    NotStarted = 0,
    Doing = 1,
    Completed = 2,
}
pub const POS_MAX: ::core::ffi::c_float = FLT_MAX;
pub fn stat_single_glyph(
    table: &GlyfTable,
    gr: &mut ComponentReference,
    stated: &mut [StatStatus],
    depth: u8,
    topj: GlyphId,
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
    let j: GlyphId = gr.glyph.index;
    if depth as i32 >= 0xff_i32 {
        return stat;
    }
    if stated[j as usize] == StatStatus::Doing {
        tracing::warn!("[Stat] Circular glyph reference found in gid {} to gid {}. The reference will be dropped.\n", topj as i32, j as i32);
        stated[j as usize] = StatStatus::Completed;
        return stat;
    }
    let g: &Glyph = table[j as usize].as_deref().unwrap();
    stated[j as usize] = StatStatus::Doing;
    let mut xmin: Pos = POS_MAX as Pos;
    let mut xmax: Pos = -POS_MAX as Pos;
    let mut ymin: Pos = POS_MAX as Pos;
    let mut ymax: Pos = -POS_MAX as Pos;
    let mut nest_depth: u16 = 0_u16;
    let mut n_points: u16 = 0_u16;
    let mut n_composite_points: u16;
    let mut n_composite_contours: u16;
    for contour in &g.contours {
        for p in contour {
            // `f64::round` rounds half away from zero, the exact contract
            // C99's `round` specifies (and propagates NaN/preserves
            // +/-infinity/+/-0.0 identically) -- a direct replacement for
            // this file's `unsafe extern "C" { fn round(...) }` import
            // (removed in Stage M-45; see RUST_MIGRATION.md).
            let x: Pos = (vq_get_still(gr.x.borrow().clone()) as f64
                + gr.a * vq_get_still(p.x.clone()) as f64
                + gr.b * vq_get_still(p.y.clone()) as f64)
                .round() as Pos;
            let y: Pos = (vq_get_still(gr.y.borrow().clone()) as f64
                + gr.c * vq_get_still(p.x.clone()) as f64
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
            n_points = (n_points as i32 + 1_i32) as u16;
        }
    }
    n_composite_points = n_points;
    n_composite_contours = g.contours.len() as u16;
    for rr in &g.references {
        let mut ref_0: ComponentReference = ComponentReference {
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
            glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            is_anchored: std::cell::Cell::new(RefAnchorStatus::Xy),
            inner: 0,
            outer: 0,
        };
        glyf_component_reference_init(&mut ref_0);
        ref_0.glyph = handle_from_index(rr.glyph.index);
        ref_0.a = gr.a * rr.a + rr.b * gr.c;
        ref_0.b = rr.a * gr.b + rr.b * gr.d;
        ref_0.c = gr.a * rr.c + gr.c * rr.d;
        ref_0.d = gr.b * rr.c + rr.d * gr.d;
        ref_0.x = std::cell::RefCell::new(vq_create_still(
            vq_get_still(rr.x.borrow().clone())
                + rr.a as Pos * vq_get_still(gr.x.borrow().clone())
                + rr.b as Pos * vq_get_still(gr.y.borrow().clone()),
        ));
        ref_0.y = std::cell::RefCell::new(vq_create_still(
            vq_get_still(rr.y.borrow().clone())
                + rr.c as Pos * vq_get_still(gr.x.borrow().clone())
                + rr.d as Pos * vq_get_still(gr.y.borrow().clone()),
        ));
        let thatstat: GlyphStat = stat_single_glyph(
            table,
            &mut ref_0,
            stated,
            (depth as i32 + 1_i32) as u8,
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
    stat.n_contours = g.contours.len() as u16;
    stat.n_composite_points = n_composite_points;
    stat.n_composite_contours = n_composite_contours;
    stated[j as usize] = StatStatus::Completed;
    return stat;
}
pub fn stat_glyf(font: &mut Font) {
    // Only ever called (from `stat_font`) under a `.head.is_some()`/
    // `.glyf.is_some()` guard, so `.unwrap()` here just turns "this
    // invariant broke" from a null-pointer dereference into a panic.
    let head = font.head.as_deref_mut().unwrap();
    let glyf = font.glyf.as_mut().unwrap();
    let mut stated: Vec<StatStatus> = vec![StatStatus::NotStarted; glyf.len()];
    let mut xmin: Pos = 0xffffffff as ::core::ffi::c_uint as Pos;
    let mut xmax: Pos = (0xffffffff as ::core::ffi::c_uint).wrapping_neg() as Pos;
    let mut ymin: Pos = 0xffffffff as ::core::ffi::c_uint as Pos;
    let mut ymax: Pos = (0xffffffff as ::core::ffi::c_uint).wrapping_neg() as Pos;
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
        gr.x = std::cell::RefCell::new(vq_create_still(0_i32 as Pos));
        gr.y = std::cell::RefCell::new(vq_create_still(0_i32 as Pos));
        gr.a = 1_i32 as Scale;
        gr.b = 0_i32 as Scale;
        gr.c = 0_i32 as Scale;
        gr.d = 1_i32 as Scale;
        let thatstat: GlyphStat = stat_single_glyph(glyf, &mut gr, &mut stated, 0_u8, j);
        glyf[j as usize].as_mut().unwrap().stat = thatstat;
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
    head.x_min = xmin as i16;
    head.x_max = xmax as i16;
    head.y_min = ymin as i16;
    head.y_max = ymax as i16;
}
pub fn stat_maxp(font: &mut Font) {
    // Only ever called (from `stat_font`) under a `.maxp.is_some()`/
    // `.glyf.is_some()` guard.
    let maxp = font.maxp.as_deref_mut().unwrap();
    let mut nest_depth: u16 = 0_u16;
    let mut n_points: u16 = 0_u16;
    let mut n_contours: u16 = 0_u16;
    let mut n_components: u16 = 0_u16;
    let mut n_composite_points: u16 = 0_u16;
    let mut n_composite_contours: u16 = 0_u16;
    let mut inst_size: u16 = 0_u16;
    let glyf = font.glyf.as_ref().unwrap();
    for g in glyf.iter() {
        let g = g.as_deref().unwrap();
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
        if g.instructions.len() as i32 > inst_size as i32 {
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
    let glyf = font.glyf.as_mut().unwrap();
    // Only ever called (from `stat_font`) under a `.hhea.is_some()`
    // guard; `.head` is set unconditionally by the pipeline before this
    // point (used below to update `.flags`).
    let mut count_a: GlyphId = count_u16(glyf.len());
    let mut count_k: GlyphId = 0 as GlyphId;
    let mut lsb_at_x_0: bool = true;
    if font.subtype != FontSubtype::Cff {
        while count_a as i32 > 2_i32
            && vq_get_still(
                glyf[(count_a as i32 - 1_i32) as usize]
                    .as_deref()
                    .unwrap()
                    .advance_width
                    .clone(),
            ) == vq_get_still(
                glyf[(count_a as i32 - 2_i32) as usize]
                    .as_deref()
                    .unwrap()
                    .advance_width
                    .clone(),
            )
        {
            count_a = count_a.wrapping_sub(1);
        }
        count_k = count_u16(glyf.len().wrapping_sub(count_a as usize));
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
    for (j, slot) in glyf.iter_mut().enumerate() {
        let g = slot.as_mut().unwrap();
        if vq_is_zero(g.horizontal_origin.clone(), 1.0f64 / 1000.0f64) {
            g.horizontal_origin = vq_neutral();
        } else {
            lsb_at_x_0 = false;
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
    let hhea = font.hhea.as_deref_mut().unwrap();
    hhea.number_of_metrics = count_a as u16;
    hhea.min_left_side_bearing = min_lsb as i16;
    hhea.min_right_side_bearing = min_rsb as i16;
    hhea.x_max_extent = max_extent as i16;
    hhea.advance_width_max = max_width as u16;
    font.hmtx = Some(Box::new(HmtxTable {
        metrics,
        left_side_bearing,
    }));
    let head = font.head.as_deref_mut().unwrap();
    head.flags = (head.flags as i32 & !0x2_i32
        | (if lsb_at_x_0 {
            0x2_i32
        } else {
            0_i32
        })) as u16;
}
fn stat_vmtx(font: &mut Font, options: &Options) {
    if font.glyf.is_none() {
        return;
    }
    let glyf = font.glyf.as_mut().unwrap();
    let mut count_a: GlyphId = count_u16(glyf.len());
    let mut count_k: GlyphId = 0 as GlyphId;
    if !(font.subtype == FontSubtype::Cff && !options.cff_short_vmtx) {
        while count_a as i32 > 2_i32
            && vq_get_still(
                glyf[(count_a as i32 - 1_i32) as usize]
                    .as_deref()
                    .unwrap()
                    .advance_height
                    .clone(),
            ) == vq_get_still(
                glyf[(count_a as i32 - 2_i32) as usize]
                    .as_deref()
                    .unwrap()
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
    let mut min_tsb: Pos = 0x7fff_i32 as Pos;
    let mut min_bsb: Pos = 0x7fff_i32 as Pos;
    let mut max_extent: Pos = -0x8000_i32 as Pos;
    let mut max_height: Length = 0_i32 as Length;
    for (j, slot) in glyf.iter().enumerate() {
        let g = slot.as_deref().unwrap();
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
    let vhea = font.vhea.as_deref_mut().unwrap();
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
    (9, &[(0x0400, 0x04FF), (0x0500, 0x052F), (0x2DE0, 0x2DFF), (0xA640, 0xA69F)]),
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
    (37, &[(0x2190, 0x21FF), (0x27F0, 0x27FF), (0x2900, 0x297F), (0x2B00, 0x2BFF)]),
    (38, &[(0x2200, 0x22FF), (0x2A00, 0x2AFF), (0x27C0, 0x27EF), (0x2980, 0x29FF)]),
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
    (57, &[(0xD800, 0xDFFF), (0x10000, i32::MAX)]),  // surrogates, and every code point beyond the BMP
    (58, &[(0x10900, 0x1091F)]),
    (59, &[(0x4E00, 0x9FFF), (0x2E80, 0x2EFF), (0x2F00, 0x2FDF), (0x2FF0, 0x2FFF), (0x3400, 0x4DBF), (0x20000, 0x2F7FF), (0x3190, 0x319F)]),
    (60, &[(0xE000, 0xF8FF)]),
    (61, &[(0x31C0, 0x31EF), (0xF900, 0xFAFF), (0x2F800, 0x2FA1F)]),
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
    (84, &[(0x1700, 0x171F), (0x1720, 0x173F), (0x1740, 0x175F), (0x1760, 0x177F)]),
    (85, &[(0x10300, 0x1032F)]),
    (86, &[(0x10330, 0x1034F)]),
    (87, &[(0x10400, 0x1044F)]),
    (88, &[(0x1D000, 0x1D0FF), (0x1D100, 0x1D1FF), (0x1D200, 0x1D24F)]),
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
    (101, &[(0x10000, 0x1007F), (0x10080, 0x100FF), (0x10100, 0x1013F)]),
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
    (121, &[(0x102A0, 0x102DF), (0x10280, 0x1029F), (0x10920, 0x1093F)]),
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
    let mut min_unicode: i32 = 0xffff_i32;
    let mut max_unicode: i32 = 0_i32;
    for &u in font.cmap.as_ref().unwrap().unicodes.keys() {
        min_unicode = min_unicode.min(u);
        max_unicode = max_unicode.max(u);
        for (range, bits) in ranges.iter_mut().zip(unicode_range_bits(u)) {
            *range |= bits;
        }
    }
    let [u1, u2, u3, u4] = ranges;
    let os_2 = font.os_2.as_deref_mut().unwrap();
    if !options.keep_unicode_ranges {
        os_2.ul_unicode_range1 = u1;
        os_2.ul_unicode_range2 = u2;
        os_2.ul_unicode_range3 = u3;
        os_2.ul_unicode_range4 = u4;
    }
    if min_unicode < 0x10000_i32 {
        os_2.us_first_char_index = min_unicode as u16;
    } else {
        os_2.us_first_char_index = 0xffff_u16;
    }
    if max_unicode < 0x10000_i32 {
        os_2.us_last_char_index = max_unicode as u16;
    } else {
        os_2.us_last_char_index = 0xffff_u16;
    };
}
fn stat_os_2_average_width(font: &mut Font, options: &Options) {
    if options.keep_average_char_width {
        return;
    }
    // Only ever called (from `stat_font`, via `stat_os_2`) under a
    // `.glyf.is_some()` guard.
    let glyf = font.glyf.as_ref().unwrap();
    let mut total_width: u32 = 0_u32;
    for slot in glyf.iter() {
        let adw: Pos = vq_get_still(slot.as_deref().unwrap().advance_width.clone()) as Pos;
        if adw > 0_i32 as Pos {
            total_width = (total_width as Pos + adw) as u32;
        }
    }
    let glyf_len = glyf.len();
    let os_2 = font.os_2.as_deref_mut().unwrap();
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
    let mut maxc: u16 = 1_u16;
    // This runs on the post-consolidation table (part of `otf_writer.rs`'s
    // build path), so a `None` slot here is a real, possible hole
    // consolidation punched, not a bug -- skip it, same as everywhere else
    // that reads `OtlTable.lookups` post-consolidation.
    for lookup in table.lookups.iter().flatten() {
        if let Some(kind) = lookup_kind(lookup.type_0) {
            kind.raise_max_context(lookup, &mut maxc);
        }
    }
    maxc
}
fn stat_max_context(font: &mut Font) {
    let mut maxc: u16 = 1_u16;
    if let Some(gsub) = font.gsub.as_deref() {
        let maxc_gsub: u16 = stat_max_context_otl(gsub);
        if maxc_gsub as i32 > maxc as i32 {
            maxc = maxc_gsub;
        }
    }
    if let Some(gpos) = font.gpos.as_deref() {
        let maxc_gpos: u16 = stat_max_context_otl(gpos);
        if maxc_gpos as i32 > maxc as i32 {
            maxc = maxc_gpos;
        }
    }
    font.os_2.as_deref_mut().unwrap().us_max_context = maxc;
}
fn stat_os_2(font: &mut Font, options: &Options) {
    stat_os_2_unicode_ranges(font, options);
    stat_os_2_average_width(font, options);
    stat_max_context(font);
}
pub const MAX_STAT_METRIC: i32 = 4096_i32;
fn stat_cff_widths(font: &mut Font) {
    if font.glyf.is_none() || font.cff.is_none() {
        return;
    }
    let glyf = font.glyf.as_ref().unwrap();
    // A local `Vec` scratch buffer instead of `__caryll_allocate_clean`/
    // `free`.
    let mut frequency: Vec<u32> = vec![0u32; MAX_STAT_METRIC as usize];
    for g in iter_glyphs(glyf) {
        let int_width: u16 = vq_get_still(g.advance_width.clone()) as u16;
        if (int_width as i32) < MAX_STAT_METRIC {
            frequency[int_width as usize] = frequency[int_width as usize].wrapping_add(1_u32);
        }
    }
    let mut maxfreq: u16 = 0_u16;
    let mut maxj: u16 = 0_u16;
    for (width, &count) in frequency.iter().enumerate() {
        if count > maxfreq as u32 {
            maxfreq = count as u16;
            maxj = width as u16;
        }
    }
    let mut nn: u16 = 0_u16;
    let mut nnsum: u32 = 0_u32;
    for g in iter_glyphs(glyf) {
        let adw: Pos = vq_get_still(g.advance_width.clone()) as Pos;
        if adw != maxj as i32 as Pos {
            nn = (nn as i32 + 1_i32) as u16;
            nnsum = (nnsum as Pos + adw) as u32;
        }
    }
    let mut nominal_width_x: i16 = 0_i16;
    if nn as i32 > 0_i32 {
        nominal_width_x = nnsum.wrapping_div(nn as u32) as i16;
    }
    let cff = font.cff.as_deref_mut().unwrap();
    if let Some(pd) = cff.private_dict.as_deref_mut() {
        pd.default_width_x = maxj as f64;
        if nn as i32 != 0_i32 {
            pd.nominal_width_x = nominal_width_x as f64;
        }
    }
    for fd in cff.fd_array.iter_mut() {
        let pd = fd.private_dict.as_deref_mut().unwrap();
        pd.default_width_x = maxj as f64;
        pd.nominal_width_x = nominal_width_x as f64;
    }
}
fn stat_vorg(font: &mut Font) {
    if font.glyf.is_none()
        || font.cff.is_none()
        || font.vhea.is_none()
        || font.vmtx.is_none()
    {
        return;
    }
    let glyf = font.glyf.as_ref().unwrap();
    // A local `Vec` scratch buffer instead of `__caryll_allocate_clean`/
    // `free`.
    let mut frequency: Vec<u32> = vec![0u32; MAX_STAT_METRIC as usize];
    for g in iter_glyphs(glyf) {
        let vori: Pos = vq_get_still(g.vertical_origin.clone()) as Pos;
        if vori >= 0_i32 as Pos && vori < MAX_STAT_METRIC as Pos {
            frequency[vori as u16 as usize] =
                frequency[vori as u16 as usize].wrapping_add(1_u32);
        }
    }
    let mut maxfreq: u32 = 0_u32;
    let mut maxj: GlyphId = 0 as GlyphId;
    for (origin, &count) in frequency.iter().enumerate() {
        if count > maxfreq {
            maxfreq = count;
            maxj = origin as GlyphId;
        }
    }
    let default_vertical_origin = maxj as Pos;
    let mut n_vert_origs: GlyphId = 0 as GlyphId;
    for g in iter_glyphs(glyf) {
        let vori_0: Pos = vq_get_still(g.vertical_origin.clone()) as Pos;
        if vori_0 != maxj as i32 as Pos {
            n_vert_origs =
                (n_vert_origs as i32 + 1_i32) as GlyphId;
        }
    }
    let mut entries: Vec<VorgEntry> = Vec::with_capacity(n_vert_origs as usize);
    for (gid, g) in iter_glyphs(glyf).enumerate() {
        let vori_1: Pos = vq_get_still(g.vertical_origin.clone()) as Pos;
        if vori_1 != maxj as i32 as Pos {
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
    let glyf = font.glyf.as_ref().unwrap();
    if !iter_glyphs(glyf).any(|g| g.y_pel > 1) {
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
// This function's own comment used to justify deriving `*mut HeadTable`/
// `*mut MaxpTable`/`*mut GlyfTable` aliases once up front and reusing them
// through ~35 `.is_null()`-guarded call/field sites, specifically to avoid
// "needing `Option`-aware rewriting". Converting to safe references means
// doing exactly that rewriting -- each `!x.is_null()` becomes `x.is_some()`,
// and each block that both reads a scalar `HeadTable` field *and* mutably
// borrows a different `Font` field first copies that field out (all the
// `HeadTable` fields read here are plain `Copy` integers) rather than
// holding a `&HeadTable` alongside the `&mut CffTable`/`&mut MaxpTable`
// borrow -- the same technique this migration used for `charstring_il.rs`'s
// `*_roll` functions.
pub fn stat_font(font: &mut Font, options: &Options) {
    if font.glyf.is_some() && font.head.is_some() {
        stat_glyf(font);
        if !options.keep_modified_time {
            // `std::time::SystemTime` measured against `UNIX_EPOCH` gives the
            // same "whole seconds since 1970-01-01 UTC" value `libc::time`'s
            // C99 contract does; `unwrap_or(0)` only matters if the system
            // clock is set before the epoch, which no real caller of this
            // font-build path can hit -- a direct replacement for this
            // file's `unsafe extern "C" { fn time(...) }`/`libc::time_t`
            // import (removed in Stage M-45; see RUST_MIGRATION.md).
            let now = ::std::time::SystemTime::now()
                .duration_since(::std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);
            font.head.as_deref_mut().unwrap().modified = 2082844800_i64 + now;
        }
    }
    if font.head.is_some() && font.cff.is_some() {
        let head_y_min = font.head.as_deref().unwrap().y_min;
        let head_y_max = font.head.as_deref().unwrap().y_max;
        let head_x_min = font.head.as_deref().unwrap().x_min;
        let head_x_max = font.head.as_deref().unwrap().x_max;
        let units_per_em = font.head.as_deref().unwrap().units_per_em;
        let glyf_len = font.glyf.as_ref().map(|g| g.len() as u32);
        let cff = font.cff.as_deref_mut().unwrap();
        if cff.font_b_box_bottom > head_y_min as i32 as f64 {
            cff.font_b_box_bottom = head_y_min as i32 as f64;
        }
        if cff.font_b_box_top < head_y_max as i32 as f64 {
            cff.font_b_box_top = head_y_max as i32 as f64;
        }
        if cff.font_b_box_left < head_x_min as i32 as f64 {
            cff.font_b_box_left = head_x_min as i32 as f64;
        }
        if cff.font_b_box_right < head_x_max as i32 as f64 {
            cff.font_b_box_right = head_x_max as i32 as f64;
        }
        if let Some(len) = glyf_len
            && cff.is_cid {
                cff.cid_count = len;
            }
        if cff.is_cid {
            // `font_matrix` is `Option<Box<CffFontMatrix>>` now: dropping
            // the old value (reassignment to `None`) recurses through its
            // own field-drop glue for free -- no manual `vq_dispose`
            // calls needed anymore (`VQ`'s `Vec<VqSegment>` shift field
            // already self-drops).
            cff.font_matrix = None;
            for fd in cff.fd_array.iter_mut() {
                fd.font_matrix = None;
                if units_per_em as i32 == 1000_i32 {
                    fd.font_matrix = None;
                } else {
                    fd.font_matrix = Some(Box::new(CffFontMatrix {
                        a: (1.0f64 / units_per_em as i32 as f64) as Scale,
                        b: 0.0f64 as Scale,
                        c: 0.0f64 as Scale,
                        d: (1.0f64 / units_per_em as i32 as f64) as Scale,
                        x: vq_neutral(),
                        y: vq_neutral(),
                    }));
                }
            }
        } else if units_per_em as i32 == 1000_i32 {
            cff.font_matrix = None;
        } else {
            cff.font_matrix = Some(Box::new(CffFontMatrix {
                a: (1.0f64 / units_per_em as i32 as f64) as Scale,
                b: 0.0f64 as Scale,
                c: 0.0f64 as Scale,
                d: (1.0f64 / units_per_em as i32 as f64) as Scale,
                x: vq_neutral(),
                y: vq_neutral(),
            }));
        }
        stat_cff_widths(font);
    }
    if let Some(len) = font.glyf.as_ref().map(|g| g.len() as u16)
        && let Some(maxp) = font.maxp.as_deref_mut() {
            maxp.num_glyphs = len;
        }
    if let Some(len) = font.glyf.as_ref().map(|g| g.len() as u32)
        && let Some(post) = font.post.as_deref_mut() {
            post.max_mem_type42 = len;
        }
    let maxp_version_is_10000 =
        font.maxp.as_deref().map(|m| m.version) == Some(0x10000 as F16Dot16);
    if font.glyf.is_some() && font.maxp.is_some() && maxp_version_is_10000 {
        stat_maxp(font);
        if let Some(fpgm_length) = font.fpgm.as_ref().map(|f| f.bytes.len() as u32) {
            let maxp = font.maxp.as_deref_mut().unwrap();
            if fpgm_length > maxp.max_size_of_instructions as u32 {
                maxp.max_size_of_instructions = fpgm_length as u16;
            }
        }
        if let Some(prep_length) = font.prep.as_ref().map(|p| p.bytes.len() as u32) {
            let maxp = font.maxp.as_deref_mut().unwrap();
            if prep_length > maxp.max_size_of_instructions as u32 {
                maxp.max_size_of_instructions = prep_length as u16;
            }
        }
    }
    if font.os_2.is_some() && font.cmap.is_some() && font.glyf.is_some() {
        stat_os_2(font, options);
    }
    if font.subtype == FontSubtype::Ttf {
        if let Some(maxp) = font.maxp.as_deref_mut() {
            maxp.version = 0x10000_i32 as F16Dot16;
        }
    } else if let Some(maxp) = font.maxp.as_deref_mut() {
        maxp.version = 0x5000_i32 as F16Dot16;
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
pub fn unstat_font(font: &mut Font) {
    delete_font_table(font, crate::tag::TAG_HDMX);
    delete_font_table(font, crate::tag::TAG_HMTX);
    delete_font_table(font, crate::tag::TAG_VORG);
    delete_font_table(font, crate::tag::TAG_VMTX);
    delete_font_table(font, crate::tag::TAG_LTSH);
}
pub const FLT_MAX: ::core::ffi::c_float = __FLT_MAX__;
pub const __FLT_MAX__: ::core::ffi::c_float = 3.402_823_5e38_f32;

#[cfg(test)]
mod stat_os_2_average_width_tests {
    use super::*;
    use crate::font::caryll_font::Font;
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

// `stat_glyf`'s `unsafe extern "C" { fn round(...) }` import was dropped in
// Stage M-45 in favor of `f64::round`. C99's `round` is specified as
// "round half away from zero, propagate NaN, preserve +/-infinity and
// +/-0.0" -- `f64::round`'s own documented contract is the identical
// "round half away from zero", pinned here against that documented
// contract rather than a live libc comparison, the same choice
// `libcff/cff_writer.rs`'s own `modf_tests` module already made.
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
            assert_eq!(bit as usize, i, "bits are listed in order, 0..=122, once each");
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
