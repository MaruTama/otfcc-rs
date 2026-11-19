//! Reading `gvar`: the glyph variation data that turns each point and
//! component offset of a `glyf` glyph into a variable quantity.

use crate::font::sfnt::Packet;
use crate::support::primitives::{F2Dot14, F16Dot16, Pos, ShapeId};
use crate::support::primitives::{f1616_muldiv, from_f2dot14, from_fixed, to_fixed};
use crate::table::fvar::{FvarTable, fvar_register_region};
use crate::table::glyf::{GlyfIOContext, GlyfTable, Glyph};
use crate::vf::region::{VqAxisSpan, VqRegion};
use crate::vf::vq::{VqSegment, VqSegmentDelta, vq_add_delta};
use otfcc_binary::FontReader;
use std::rc::Rc;

/// What applying one glyph's variation data needs from the rest of the
/// font: `fvar` to register regions in, the axis count, the shared tuples
/// and the number of phantom points after the outline's own points.
// `gvar` is read field by field at byte offsets through `FontReader`, so
// every read is checked against the table's length.
#[derive(Debug)]
pub struct TuplePolymorphizerCtx<'a> {
    pub fvar: Option<&'a mut FvarTable>,
    /// The number of axes, which is the length of every tuple.
    pub dimensions: u16,
    pub shared_tuple_count: u16,
    /// The byte offset of the shared tuples in `gvar`. Reads through it are
    /// checked against the table's length.
    pub shared_tuples_offset: usize,
    pub n_phantom_points: ShapeId,
}
/// The header of one run of packed deltas: how many, whether they are
/// 16-bit, and whether they are all zero (and so not stored).
#[derive(Copy, Clone, Debug)]
pub struct PackedDeltaRun {
    pub length: ShapeId,
    pub wide: bool,
    pub zero: bool,
}
/// The header of one run of packed point numbers: how many, and whether
/// they are 16-bit.
#[derive(Copy, Clone, Debug)]
pub struct PackedPointRun {
    pub length: ShapeId,
    pub wide: bool,
}
/// `gvar` header flag: the glyph data offsets are 32-bit, not 16-bit
/// halves.
pub const GVAR_OFFSETS_ARE_LONG: u16 = 1;
/// `tupleIndex` flag: the header carries its own peak tuple.
pub const EMBEDDED_PEAK_TUPLE: u16 = 0x8000;
/// `tupleIndex` flag: the header carries a start and an end tuple.
pub const INTERMEDIATE_REGION: u16 = 0x4000;
/// `tupleIndex` flag: the variation data has its own point numbers.
pub const PRIVATE_POINT_NUMBERS: u16 = 0x2000;
/// `tupleIndex` mask: the shared tuple used as the peak, when there is no
/// embedded one.
pub const TUPLE_INDEX_MASK: u16 = 0xfff;
// A `TupleVariationHeader` array has no length of its own -- each header's
// end (and so the next one's start) is only known after reading *this*
// header's own `tupleIndex` flags, which is why this can't be a simple
// `size_of::<TupleVariationHeader>() * n` stride. `tvh_offset` is an
// absolute byte offset into `gvar` (not a pointer) so every read here goes
// through `FontReader`'s bounds check instead of `.offset()`ing off the
// end of the table.
#[inline]
fn next_tvh_offset(gvar: &[u8], tvh_offset: usize, dimensions: u16) -> Option<usize> {
    let tuple_index = FontReader::new(gvar).at(tvh_offset + 2).ok()?.u16().ok()?;
    let mut bump: usize = 4; // variationDataSize(2) + tupleIndex(2)
    if tuple_index & EMBEDDED_PEAK_TUPLE != 0 {
        bump += dimensions as usize * ::core::mem::size_of::<F2Dot14>();
    }
    if tuple_index & INTERMEDIATE_REGION != 0 {
        bump += 2 * dimensions as usize * ::core::mem::size_of::<F2Dot14>();
    }
    Some(tvh_offset + bump)
}
/// Packed point count: the count takes two bytes.
pub const POINT_COUNT_IS_WORD: u8 = 0x80;
/// Packed point count: the count's bits when it takes two bytes.
pub const POINT_COUNT_LONG_MASK: u16 = 0x7fff;
/// Packed point run header: the run length minus one.
pub const POINT_RUN_COUNT_MASK: u8 = 0x7f;
/// Packed point run header: the point numbers are 16-bit.
pub const POINTS_ARE_WORDS: u8 = 0x80;
/// Reads a packed point-number list at `offset`. Returns the offset just
/// past it and the point indices, or `None` if it runs past `gvar`.
#[inline]
fn parse_point_numbers(
    gvar: &[u8],
    offset: usize,
    total_points: ShapeId,
) -> Option<(usize, Vec<ShapeId>)> {
    let mut r = FontReader::new(gvar).at(offset).ok()?;
    let first_byte: u8 = r.u8().ok()?;
    let n_points: u16 = if first_byte & POINT_COUNT_IS_WORD != 0 {
        let second_byte: u8 = r.u8().ok()?;
        (u16::from(first_byte) << 8 | u16::from(second_byte)) & POINT_COUNT_LONG_MASK
    } else {
        u16::from(first_byte)
    };
    let mut point_indeces: Vec<ShapeId>;
    if n_points > 0 {
        let mut run: PackedPointRun = PackedPointRun {
            length: 0 as ShapeId,
            wide: false,
        };
        let mut j_point: ShapeId = 0 as ShapeId;
        point_indeces = Vec::with_capacity(n_points as usize);
        for _ in 0..n_points {
            if run.length == 0 {
                let run_header: u8 = r.u8().ok()?;
                run.wide = run_header & POINTS_ARE_WORDS != 0;
                run.length = ShapeId::from(run_header & POINT_RUN_COUNT_MASK) + 1;
            }
            let mut point_number: i16 = j_point as i16;
            if run.wide {
                // Native-endian, unlike the big-endian wide runs elsewhere:
                // a long-standing bug kept so output does not change for
                // fonts that use a wide point-number run here. See
                // RUST_MIGRATION.md.
                let b = r.bytes(2).ok()?;
                let raw = u16::from_ne_bytes([b[0], b[1]]);
                point_number = (i32::from(point_number) + i32::from(raw)) as i16;
            } else {
                point_number = (i32::from(point_number) + i32::from(r.u8().ok()?)) as i16;
            }
            point_indeces.push(point_number as ShapeId);
            j_point = point_number as ShapeId;
            run.length = run.length.wrapping_sub(1);
        }
    } else {
        point_indeces = (0..total_points).collect();
    }
    Some((r.pos(), point_indeces))
}
/// Packed delta run header: the deltas are all zero and not stored.
pub const DELTAS_ARE_ZERO: u8 = 0x80;
/// Packed delta run header: the deltas are 16-bit.
pub const DELTAS_ARE_WORDS: u8 = 0x40;
/// Packed delta run header: the run length minus one.
pub const DELTA_RUN_COUNT_MASK: u8 = 0x3f;
#[inline]
fn read_packed_delta(
    gvar: &[u8],
    offset: usize,
    n_points: ShapeId,
    deltas: &mut [Pos],
) -> Option<usize> {
    let mut r = FontReader::new(gvar).at(offset).ok()?;
    let mut run: PackedDeltaRun = PackedDeltaRun {
        length: 0 as ShapeId,
        wide: false,
        zero: false,
    };
    for filled in 0..n_points {
        let mut delta: i16 = 0;
        if run.length == 0 {
            let run_header: u8 = r.u8().ok()?;
            run.zero = run_header & DELTAS_ARE_ZERO != 0;
            run.wide = run_header & DELTAS_ARE_WORDS != 0;
            run.length = ShapeId::from(run_header & DELTA_RUN_COUNT_MASK) + 1;
        }
        if !run.zero {
            if run.wide {
                delta = r.i16().ok()?;
            } else {
                delta = i16::from(r.i8().ok()?);
            }
        }
        deltas[filled as usize] = Pos::from(delta);
        run.length = run.length.wrapping_sub(1);
    }
    Some(r.pos())
}
#[inline]
// `nudges`/`kernel` are borrowed slices: this function neither owns nor
// frees either array, only reads (`kernel`) or reads-then-writes
// (`nudges`) into them by index.
fn fill_the_gaps(j_min: ShapeId, j_max: ShapeId, nudges: &mut [VqSegment], kernel: &[Pos]) {
    for j in j_min..j_max {
        if !nudges[j as usize].is_touched() {
            let mut j_next: ShapeId = j;
            while !nudges[j_next as usize].is_touched() {
                if i32::from(j_next) == i32::from(j_max) - 1 {
                    j_next = j_min;
                } else {
                    j_next = (i32::from(j_next) + 1) as ShapeId;
                }
                if i32::from(j_next) == i32::from(j) {
                    break;
                }
            }
            let mut j_prev: ShapeId = j;
            while !nudges[j_prev as usize].is_touched() {
                if i32::from(j_prev) == i32::from(j_min) {
                    j_prev = (i32::from(j_max) - 1) as ShapeId;
                } else {
                    j_prev = (i32::from(j_prev) - 1) as ShapeId;
                }
                if i32::from(j_prev) == i32::from(j) {
                    break;
                }
            }
            if nudges[j_next as usize].is_touched() && nudges[j_prev as usize].is_touched() {
                let untouch_j: F16Dot16 = to_fixed(kernel[j as usize]);
                let untouch_prev: F16Dot16 = to_fixed(kernel[j_prev as usize] as f64);
                let untouch_next: F16Dot16 = to_fixed(kernel[j_next as usize] as f64);
                let delta_prev: F16Dot16 =
                    to_fixed(nudges[j_prev as usize].unwrap_delta().quantity as f64);
                let delta_next: F16Dot16 =
                    to_fixed(nudges[j_next as usize].unwrap_delta().quantity as f64);
                let mut u_min: F16Dot16 = untouch_prev;
                let mut u_max: F16Dot16 = untouch_next;
                let mut d_min: F16Dot16 = delta_prev;
                let mut d_max: F16Dot16 = delta_next;
                if untouch_prev > untouch_next {
                    u_min = untouch_next;
                    u_max = untouch_prev;
                    d_min = delta_next;
                    d_max = delta_prev;
                }
                if untouch_j <= u_min {
                    nudges[j as usize].delta_mut().quantity = from_fixed(d_min) as Pos;
                } else if untouch_j >= u_max {
                    nudges[j as usize].delta_mut().quantity = from_fixed(d_max) as Pos;
                } else {
                    nudges[j as usize].delta_mut().quantity = from_fixed(f1616_muldiv(
                        d_max - d_min,
                        untouch_j - u_min,
                        u_max - u_min,
                    )) as Pos;
                }
            }
        }
    }
}
// Computes one axis' nudges (`VqSegment`s that `apply_polymorphism` adds to
// that axis' `.shift`). `contour_lens` gives each contour's point count, in
// the order `apply_polymorphism` flattened them, because gap filling never
// crosses a contour.
/// One tuple's deltas: the region they apply in, the points they touch,
/// and each touched point's x and y delta, in the same order as `points`.
struct TupleDeltas<'a> {
    region: &'a Rc<VqRegion>,
    points: &'a [ShapeId],
    x: &'a [Pos],
    y: &'a [Pos],
}
/// The shape of a glyph's flattened points: how many there are, and the
/// length of each contour (the component references follow the contours).
struct FlatOutline {
    total_points: ShapeId,
    contour_lens: Vec<usize>,
}
/// The nudge of every point along one axis: `deltas` (the tuple's x or y
/// deltas) on the touched points, and interpolated ones on the rest of each
/// contour. `kernel` is every point's coordinate on that axis.
fn apply_coords(
    outline: &FlatOutline,
    kernel: &[Pos],
    tuple: &TupleDeltas<'_>,
    deltas: &[Pos],
) -> Vec<VqSegment> {
    let total_points = outline.total_points;
    let mut nudges: Vec<VqSegment> = Vec::with_capacity(total_points as usize);
    for _ in 0..total_points {
        nudges.push(VqSegment::Delta(VqSegmentDelta {
            quantity: Pos::from(0),
            touched: false,
            region: Rc::clone(tuple.region),
        }));
    }
    for (&idx, &delta) in tuple.points.iter().zip(deltas.iter()) {
        if i32::from(idx) < i32::from(total_points) {
            let d = nudges[idx as usize].delta_mut();
            d.touched = true;
            d.quantity += delta;
        }
    }
    let mut j_first: ShapeId = 0 as ShapeId;
    for &len in &outline.contour_lens {
        fill_the_gaps(
            j_first,
            (j_first as usize).wrapping_add(len) as ShapeId,
            &mut nudges,
            kernel,
        );
        j_first = (j_first as usize).wrapping_add(len) as ShapeId;
    }
    nudges
}
// The points (contours first, then references) are flattened into plain
// `Vec<Pos>` kernels for `apply_coords`/`fill_the_gaps`, and the results
// are written back by a second pass in the same order, so index `j` names
// the same point in both passes.
#[inline]
fn apply_polymorphism(glyph: &mut Glyph, total_points: ShapeId, tuple: &TupleDeltas<'_>) {
    // Flatten the contours, then the references, in the order the write-back
    // pass below walks them: each point's per-axis `.kernel`, and each
    // contour's length (gap filling stays within a contour).
    let mut contour_lens: Vec<usize> = Vec::with_capacity(glyph.contours.len());
    let mut kernel_x: Vec<Pos> = Vec::with_capacity(total_points as usize);
    let mut kernel_y: Vec<Pos> = Vec::with_capacity(total_points as usize);
    for c in &glyph.contours {
        contour_lens.push(c.len());
        for p in c {
            kernel_x.push(p.x.kernel);
            kernel_y.push(p.y.kernel);
        }
    }
    for rf in &glyph.references {
        kernel_x.push(rf.x.borrow().kernel);
        kernel_y.push(rf.y.borrow().kernel);
    }

    let outline = FlatOutline {
        total_points,
        contour_lens,
    };
    let nudges_x = apply_coords(&outline, &kernel_x, tuple, tuple.x);
    let nudges_y = apply_coords(&outline, &kernel_y, tuple, tuple.y);

    // Write-back: the only place in this cluster that still needs `&mut
    // Point`/`&mut ComponentReference` -- one more `contours`-then-
    // `references` pass, this time mutable, in lockstep with the
    // immutable pass above (point `j` here is the same point `j` that
    // contributed `kernel_x[j]`/`kernel_y[j]`, so it gets `nudges_x[j]`/
    // `nudges_y[j]` back).
    let mut j: usize = 0;
    for c in glyph.contours.iter_mut() {
        for p in c.iter_mut() {
            let dx = nudges_x[j].clone();
            if !(dx.unwrap_delta().quantity == 0. && dx.is_touched()) {
                p.x.shift.push(dx);
            }
            let dy = nudges_y[j].clone();
            if !(dy.unwrap_delta().quantity == 0. && dy.is_touched()) {
                p.y.shift.push(dy);
            }
            j += 1;
        }
    }
    for rf in glyph.references.iter_mut() {
        let dx = nudges_x[j].clone();
        if !(dx.unwrap_delta().quantity == 0. && dx.is_touched()) {
            rf.x.get_mut().shift.push(dx);
        }
        let dy = nudges_y[j].clone();
        if !(dy.unwrap_delta().quantity == 0. && dy.is_touched()) {
            rf.y.get_mut().shift.push(dy);
        }
        j += 1;
    }

    // The phantom points follow the outline's points. Their deltas are taken
    // by position in the tuple (entry `total_points` onward), not by point
    // number, as otfcc does, so they apply only when the tuple has entries
    // past the outline's points.
    let n_touched = tuple.points.len() as ShapeId;
    if (i32::from(total_points) + 1) < i32::from(n_touched) {
        vq_add_delta(
            &mut glyph.horizontal_origin,
            true,
            tuple.region,
            tuple.x[total_points as usize],
        );
        vq_add_delta(
            &mut glyph.advance_width,
            true,
            tuple.region,
            tuple.x[(i32::from(total_points) + 1) as usize] - tuple.x[total_points as usize],
        );
    }
    if (i32::from(total_points) + 3) < i32::from(n_touched) {
        vq_add_delta(
            &mut glyph.vertical_origin,
            true,
            tuple.region,
            tuple.y[(i32::from(total_points) + 2) as usize],
        );
        vq_add_delta(
            &mut glyph.advance_height,
            true,
            tuple.region,
            tuple.y[(i32::from(total_points) + 2) as usize]
                - tuple.y[(i32::from(total_points) + 3) as usize],
        );
    }
}
// `peak_offset`/`range_offset` are byte offsets into `gvar` of a tuple's peak
// and intermediate coordinates. The region is built in a local `Vec` and
// boxed only once nothing can fail any more.
fn create_region_from_tuples(
    gvar: &[u8],
    dimensions: u16,
    peak_offset: usize,
    range_offset: Option<usize>,
) -> Option<Box<VqRegion>> {
    let mut spans: Vec<VqAxisSpan> = Vec::with_capacity(dimensions as usize);
    for d in 0..dimensions {
        let Ok(peak_raw) = FontReader::new(gvar)
            .at(peak_offset + d as usize * 2)
            .and_then(|mut x| x.i16())
        else {
            return None;
        };
        let peak_val: Pos = from_f2dot14(peak_raw as F2Dot14) as Pos;
        let mut span: VqAxisSpan = VqAxisSpan {
            start: Pos::from(if peak_val <= Pos::from(0) { -1 } else { 0 }),
            peak: peak_val,
            end: Pos::from(if peak_val >= Pos::from(0) { 1 } else { 0 }),
        };
        if let Some(start_offset) = range_offset {
            let end_offset = start_offset + dimensions as usize * 2;
            let start_read = FontReader::new(gvar)
                .at(start_offset + d as usize * 2)
                .and_then(|mut x| x.i16());
            let end_read = FontReader::new(gvar)
                .at(end_offset + d as usize * 2)
                .and_then(|mut x| x.i16());
            match (start_read, end_read) {
                (Ok(sv), Ok(ev)) => {
                    span.start = from_f2dot14(sv as F2Dot14) as Pos;
                    span.end = from_f2dot14(ev as F2Dot14) as Pos;
                }
                _ => {
                    return None;
                }
            }
        }
        spans.push(span);
    }
    Some(Box::new(VqRegion {
        dimensions: dimensions as ShapeId,
        spans,
    }))
}
// Applies the tuple variations of one glyph, found at `gvd_offset` in `gvar`.
// The record and its tuple headers describe their own sizes, so every read is
// checked; a failed read stops at that tuple and keeps the deltas already
// applied.
#[inline]
fn polymorphize_glyph(
    glyph: &mut Glyph,
    ctx: &mut TuplePolymorphizerCtx<'_>,
    gvar: &[u8],
    gvd_offset: usize,
) -> Option<()> {
    let mut total_points: ShapeId = 0 as ShapeId;
    for c in &glyph.contours {
        total_points = (total_points as usize).wrapping_add(c.len()) as ShapeId;
    }
    total_points = (total_points as usize).wrapping_add(glyph.references.len()) as ShapeId;
    let total_delta_entries: ShapeId =
        (i32::from(total_points) + i32::from(ctx.n_phantom_points)) as ShapeId;

    let mut header = FontReader::new(gvar).at(gvd_offset).ok()?;
    let raw_tuple_variation_count = header.u16().ok()?;
    let data_offset_field = header.u16().ok()?;
    let n_tuples: u16 = raw_tuple_variation_count & 0xfff;
    let has_shared_point_numbers: bool = raw_tuple_variation_count & 0x8000 != 0;
    let mut tvh_offset: usize = gvd_offset + 4;

    // Empty means the glyph has no shared point numbers.
    let mut shared_point_indeces: Vec<ShapeId> = Vec::new();
    let mut data_offset: usize = gvd_offset + data_offset_field as usize;
    if has_shared_point_numbers {
        let (new_offset, indeces) = parse_point_numbers(gvar, data_offset, total_delta_entries)?;
        data_offset = new_offset;
        shared_point_indeces = indeces;
    }
    let mut tsd_start: usize = 0;
    for _ in 0..n_tuples {
        let mut th = FontReader::new(gvar).at(tvh_offset).ok()?;
        let variation_data_size = th.u16().ok()?;
        let tuple_index_raw = th.u16().ok()?;
        let tuple_index: ShapeId = (tuple_index_raw & TUPLE_INDEX_MASK) as ShapeId;
        let has_embedded_peak: bool = tuple_index_raw & EMBEDDED_PEAK_TUPLE != 0;
        let has_intermediate: bool = tuple_index_raw & INTERMEDIATE_REGION != 0;

        let peak_offset = if has_embedded_peak {
            tvh_offset + 4
        } else {
            ctx.shared_tuples_offset + ctx.dimensions as usize * tuple_index as usize * 2
        };
        let range_offset = if has_intermediate {
            let embedded_slots: usize = if has_embedded_peak { 1 } else { 0 };
            Some(tvh_offset + 4 + embedded_slots * ctx.dimensions as usize * 2)
        } else {
            None
        };
        let region = create_region_from_tuples(gvar, ctx.dimensions, peak_offset, range_offset)?;
        // `polymorphize`'s caller-side guard (`axes_len` computed via
        // `ctx.fvar.as_deref()`) already returned early if there was no
        // `fvar` table, so every `polymorphize_glyph` call is guaranteed
        // a `Some` here; `fvar_register_region` takes a real `&mut
        // FvarTable` (out of this file's scope, `fvar.rs`'s own
        // region-dedup table) -- reborrowed fresh each iteration of this
        // loop, same as the reborrow that built `ctx.fvar` itself in
        // `polymorphize`.
        let r: Rc<VqRegion> = fvar_register_region(
            ctx.fvar
                .as_deref_mut()
                .expect("fvar checked non-null by polymorphize"),
            region,
        );

        let tsd = data_offset + tsd_start;
        // `point_indeces` borrows `shared_point_indeces` by default and
        // only owns a private `Vec` -- built fresh by `parse_point_numbers`
        // -- when `PRIVATE_POINT_NUMBERS` is set for this tuple.
        let n_points: ShapeId;
        let point_indeces: ::std::borrow::Cow<[ShapeId]>;
        let after_points: usize;
        if tuple_index_raw & PRIVATE_POINT_NUMBERS != 0 {
            let (new_tsd, private_point_numbers) =
                parse_point_numbers(gvar, tsd, total_delta_entries)?;
            after_points = new_tsd;
            n_points = private_point_numbers.len() as ShapeId;
            point_indeces = ::std::borrow::Cow::Owned(private_point_numbers);
        } else {
            after_points = tsd;
            n_points = shared_point_indeces.len() as ShapeId;
            point_indeces = ::std::borrow::Cow::Borrowed(&shared_point_indeces);
        }
        if !point_indeces.is_empty() {
            let mut delta_x: Vec<Pos> = vec![Pos::from(0); n_points as usize];
            let mut delta_y: Vec<Pos> = vec![Pos::from(0); n_points as usize];
            let after_x = read_packed_delta(gvar, after_points, n_points, &mut delta_x)?;
            read_packed_delta(gvar, after_x, n_points, &mut delta_y)?;
            // `apply_polymorphism` is a safe `fn` now; `glyph` reborrows
            // here exactly as it does across every iteration of this loop.
            let tuple = TupleDeltas {
                region: &r,
                points: &point_indeces,
                x: &delta_x,
                y: &delta_y,
            };
            apply_polymorphism(glyph, total_points, &tuple);
        }
        tsd_start = tsd_start.wrapping_add(variation_data_size as usize);
        tvh_offset = next_tvh_offset(gvar, tvh_offset, ctx.dimensions)?;
    }
    Some(())
}
/// Applies `gvar` to every glyph of `glyf`, turning point and component
/// offsets into variable quantities. Does nothing unless `ctx.fvar` has
/// axes and the font has a `gvar` whose axis count matches.
// Every `GVARHeader`/`glyphVariationDataOffsets` read below is checked: the
// per-glyph offset array (`glyphVariationDataOffsets[j]` for every `j` up
// to `num_glyphs`) must fit the `gvar` table, and so must each
// `glyphVariationDataArrayOffset + glyphVariationDataOffset` sum.
#[inline]
pub(super) fn polymorphize(packet: &Packet, glyf: &mut GlyfTable, ctx: &mut GlyfIOContext<'_>) {
    let Some(axes_len) = ctx.fvar.as_deref().map(|f| f.axes.len()) else {
        return;
    };
    if axes_len == 0 {
        return;
    }
    let Some(table) = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_GVAR) else {
        return;
    };
    let gvar: &[u8] = &table.data;

    // GVARHeader: majorVersion(2) + minorVersion(2) + axisCount(2) +
    // sharedTupleCount(2) + sharedTuplesOffset(4) + glyphCount(2) +
    // flags(2) + glyphVariationDataArrayOffset(4) = 20 bytes.
    let Ok(mut header) = FontReader::new(gvar).at(0) else {
        return;
    };
    if header.skip(4).is_err() {
        return;
    } // majorVersion/minorVersion: not read
    let Ok(axis_count) = header.u16() else { return };
    if axis_count as usize != axes_len {
        tracing::warn!("Axes number in GVAR and FVAR are inequal");
        return;
    }
    let Ok(shared_tuple_count) = header.u16() else {
        return;
    };
    let Ok(shared_tuples_offset) = header.u32() else {
        return;
    };
    let Ok(_glyph_count) = header.u16() else {
        return;
    };
    let Ok(flags) = header.u16() else { return };
    let Ok(glyph_variation_data_array_offset) = header.u32() else {
        return;
    };

    let dimensions = axis_count;
    let offsets_are_long = flags & GVAR_OFFSETS_ARE_LONG != 0;
    const OFFSET_ARRAY_BASE: usize = 20; // sizeof(GVARHeader)

    for (j, glyph_slot) in glyf.iter_mut().enumerate() {
        let Some(glyph_variation_data_offset) = (if offsets_are_long {
            FontReader::new(gvar)
                .at(OFFSET_ARRAY_BASE + j * 4)
                .ok()
                .and_then(|mut r| r.u32().ok())
        } else {
            FontReader::new(gvar)
                .at(OFFSET_ARRAY_BASE + j * 2)
                .ok()
                .and_then(|mut r| r.u16().ok())
                .map(|v| u32::from(v) * 2)
        }) else {
            continue;
        };
        let Some(gvd_offset) = (glyph_variation_data_array_offset as usize)
            .checked_add(glyph_variation_data_offset as usize)
        else {
            continue;
        };

        // `ctx.fvar.as_deref_mut()` reborrows the `&mut FvarTable` fresh
        // for this one iteration -- `tpctx` (and the reborrow it holds)
        // is dropped at the end of the loop body, so the next iteration
        // reborrows again rather than aliasing the previous one.
        let mut tpctx = TuplePolymorphizerCtx {
            fvar: ctx.fvar.as_deref_mut(),
            dimensions,
            shared_tuple_count,
            shared_tuples_offset: shared_tuples_offset as usize,
            n_phantom_points: ctx.n_phantom_points,
        };
        polymorphize_glyph(
            glyph_slot
                .as_deref_mut()
                .expect("read_glyf pushes Some for every glyph before polymorphize runs"),
            &mut tpctx,
            gvar,
            gvd_offset,
        );
    }
}

#[cfg(test)]
mod gvar_polymorphize_tests {
    use super::*;
    use crate::table::glyf::{Contour, Point, glyf_component_reference_empty, new_glyf_glyph};
    use crate::vf::region::vq_create_region;
    use crate::vf::vq::vq_create_still;

    #[test]
    fn next_tvh_offset_truncated_header_is_rejected_instead_of_reading_oob() {
        // The axis-coordinate array must fit the gvar bytes.
        let gvar: [u8; 0] = [];
        {
            assert!(next_tvh_offset(&gvar, 0, 1).is_none());
        }
    }

    #[test]
    fn next_tvh_offset_bumps_past_an_embedded_peak_tuple() {
        // variationDataSize=0, tupleIndex=EMBEDDED_PEAK_TUPLE (0x8000) ->
        // bump = 4 (header) + 1 dimension * 2 bytes = 6.
        let mut data = [0u8; 4];
        data[2..4].copy_from_slice(&0x8000u16.to_be_bytes());
        {
            assert_eq!(next_tvh_offset(&data, 0, 1), Some(6));
        }
    }

    #[test]
    fn create_region_from_tuples_truncated_peak_is_rejected_not_leaked() {
        // A peak/start/end F2Dot14 must fit the gvar bytes. Also exercises
        // the failure path's cleanup: `vq_create_region`'s allocation must
        // not leak (`cargo miri test` would flag it).
        let gvar: [u8; 0] = [];
        {
            assert!(create_region_from_tuples(&gvar, 1, 0, None).is_none());
        }
    }

    #[test]
    fn create_region_from_tuples_reads_a_single_dimension_peak() {
        let gvar = [0x40u8, 0x00u8]; // F2Dot14 0x4000 = 1.0
        let region = create_region_from_tuples(&gvar, 1, 0, None).unwrap();
        assert_eq!(region.dimensions, 1);
    }

    #[test]
    fn parse_point_numbers_truncated_run_is_rejected_instead_of_reading_oob() {
        // n_points=2 (first byte, not POINT_COUNT_IS_WORD), then the
        // buffer ends before the run header that should follow -- the
        // original had no length parameter to check against at all.
        let data = [0x02u8];
        {
            assert!(parse_point_numbers(&data, 0, 5).is_none());
        }
    }

    #[test]
    fn parse_point_numbers_zero_count_returns_every_point_in_order() {
        let data = [0x00u8]; // n_points=0 -> "every point", 0..total_points
        {
            let (new_offset, indeces) = parse_point_numbers(&data, 0, 3).unwrap();
            assert_eq!(new_offset, 1);
            assert_eq!(indeces, vec![0, 1, 2]);
        }
    }

    #[test]
    fn read_packed_delta_truncated_run_is_rejected_instead_of_reading_oob() {
        let data: [u8; 0] = [];
        let mut deltas = [0.0; 1];
        {
            assert!(read_packed_delta(&data, 0, 1, &mut deltas).is_none());
        }
    }

    #[test]
    fn read_packed_delta_reads_a_single_narrow_delta() {
        // run header 0x00: not zero, not wide, run length = 0+1 = 1; one
        // signed byte delta of 5.
        let data = [0x00u8, 5u8];
        let mut deltas = [0.0; 1];
        {
            let new_offset = read_packed_delta(&data, 0, 1, &mut deltas).unwrap();
            assert_eq!(new_offset, 2);
            assert_eq!(deltas[0], 5.0);
        }
    }

    #[test]
    // Regression guard for the IUP X/Y axis mix-up; see the comment below.
    fn fill_the_gaps_interpolates_an_untouched_points_delta_using_its_own_axis_kernel() {
        // Regression test for a bug inherited from upstream otfcc (commit
        // 2ddee94f, 2017-11-13): the Y-axis pass filled in untouched
        // points' deltas by interpolating against neighbors' X coordinates
        // instead of their Y coordinates.
        //
        // Three points in one contour: P0/P2 touched, P1 untouched. P1's
        // X and Y original coordinates sit at different fractional
        // positions between P0 and P2 (25% along X, 75% along Y) so the
        // correct interpolated Y-delta (using Y's own 75% ratio) differs
        // sharply from what reusing X's 25% ratio would produce.
        let mut glyph = new_glyf_glyph();
        let mut contour: Contour = Vec::new();
        for (x, y) in [(0.0, 0.0), (5.0, 150.0), (20.0, 200.0)] {
            contour.push(Point {
                x: vq_create_still(x),
                y: vq_create_still(y),
                on_curve: 1,
            });
        }
        glyph.contours.push(contour);

        let r: Rc<VqRegion> = Rc::from(vq_create_region(1));
        let points: [ShapeId; 2] = [0, 2];
        let delta_x: [Pos; 2] = [0.0, 100.0];
        let delta_y: [Pos; 2] = [0.0, 1000.0];
        let tuple = TupleDeltas {
            region: &r,
            points: &points,
            x: &delta_x,
            y: &delta_y,
        };
        apply_polymorphism(&mut glyph, 3, &tuple);

        let p1 = &glyph.contours[0][1];
        // X: P1 sits 25% of the way from P0 to P2 -> interpolated
        // delta_x is 25% of the way from 0 to 100.
        assert_eq!(p1.x.shift.len(), 1);
        assert_eq!(p1.x.shift[0].unwrap_delta().quantity, 25.0);
        // Y: P1 sits 75% of the way from P0 to P2 -> interpolated
        // delta_y is 75% of the way from 0 to 1000 (750), not the
        // 25%-of-1000 = 250 the bug would have produced by reusing
        // X's ratio.
        assert_eq!(p1.y.shift.len(), 1);
        assert_eq!(p1.y.shift[0].unwrap_delta().quantity, 750.0);
    }

    #[test]
    // Targeted regression for the `CoordRef`-elimination redesign: the old
    // code resolved each flattened index back to a `Point`/
    // `ComponentReference` through a per-index `CoordRef` built once, up
    // front. The redesign instead makes two separate `contours`-then-
    // `references` flattening passes (one immutable, to collect
    // `kernel_x`/`kernel_y`; one mutable, to write `nudges_x`/`nudges_y`
    // back) that must visit points in lockstep for index `j` to mean the
    // same point in both passes. No existing test in this file had a
    // `ComponentReference` at all, so a flatten-order mistake (e.g. the
    // write-back pass visiting references before contours, or skipping a
    // point) would have gone undetected.
    fn apply_polymorphism_writes_nudges_back_to_the_matching_point_or_reference() {
        let mut glyph = new_glyf_glyph();
        // One contour with one touched point (flattened index 0) ...
        let contour: Contour = vec![Point {
            x: vq_create_still(0.0),
            y: vq_create_still(0.0),
            on_curve: 1,
        }];
        glyph.contours.push(contour);
        // ... followed by one touched component reference (flattened
        // index 1, per the same contours-then-references order the
        // original `CoordRef`-building loop used).
        let mut reference = glyf_component_reference_empty();
        reference.x = std::cell::RefCell::new(vq_create_still(0.0));
        reference.y = std::cell::RefCell::new(vq_create_still(0.0));
        glyph.references.push(reference);

        let r: Rc<VqRegion> = Rc::from(vq_create_region(1));
        let points: [ShapeId; 2] = [0, 1];
        let delta_x: [Pos; 2] = [5.0, 100.0];
        let delta_y: [Pos; 2] = [50.0, 200.0];
        let tuple = TupleDeltas {
            region: &r,
            points: &points,
            x: &delta_x,
            y: &delta_y,
        };
        apply_polymorphism(&mut glyph, 2, &tuple);

        let p0 = &glyph.contours[0][0];
        assert_eq!(p0.x.shift.len(), 1);
        assert_eq!(p0.x.shift[0].unwrap_delta().quantity, 5.0);
        assert_eq!(p0.y.shift.len(), 1);
        assert_eq!(p0.y.shift[0].unwrap_delta().quantity, 50.0);

        let c0 = &glyph.references[0];
        assert_eq!(c0.x.borrow().shift.len(), 1);
        assert_eq!(c0.x.borrow().shift[0].unwrap_delta().quantity, 100.0);
        assert_eq!(c0.y.borrow().shift.len(), 1);
        assert_eq!(c0.y.borrow().shift[0].unwrap_delta().quantity, 200.0);
    }
}
