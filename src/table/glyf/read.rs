use crate::support::handle::{GlyphHandle, handle_from_index};

use crate::font::sfnt::Packet;
use otfcc_binary::FontReader;
use crate::support::primitives::{F2Dot14, F16Dot16, GlyphId, Pos, Scale, ShapeId};

use crate::table::fvar::FvarTable;
use crate::table::glyf::{
    ComponentFlags, ComponentReference, Contour, ContourList, GlyfIOContext, GlyfTable, Glyph,
    Point, PointFlags, RefAnchorStatus,
};

use crate::support::primitives::{
    f1616_muldiv, from_f2dot14, from_fixed, to_fixed,
};
use crate::table::fvar::fvar_register_region;
use crate::table::glyf::{glyf_component_reference_empty, glyf_contour_fill, new_glyf_glyph};
use crate::vf::region::{VqAxisSpan, VqRegion};
use crate::vf::vq::{VQ, VqSegment, VqSegmentDelta};
use crate::vf::vq::{
    vq_add_delta, vq_create_still, vq_inplace_plus, vq_neutral,
};
use std::rc::Rc;

// `gvar` is read field by field at byte offsets through `FontReader`, so
// every read is checked against the table's length.
#[derive(Debug)]
pub struct TuplePolymorphizerCtx<'a> {
    pub fvar: Option<&'a mut FvarTable>,
    pub dimensions: u16,
    pub shared_tuple_count: u16,
    // An absolute byte offset into `gvar` instead of a `*mut F2Dot14` --
    // `polymorphize_glyph` reads through it via `FontReader`, checked
    // against `gvar`'s real length every time, instead of walking off an
    // unbounded pointer.
    pub shared_tuples_offset: usize,
    pub coord_dimensions: u8,
    pub allow_iup: bool,
    pub n_phantom_points: ShapeId,
}
// `CoordRef` (an enum over `*mut Point`/`*mut ComponentReference`,
// replacing the still-older `CoordPartGetter`/`get_x`/`get_y`
// function-pointer design that punned a `*mut Point` to sometimes really
// point at a `ComponentReference`) is gone. `coord_of`'s only job was
// resolving `(CoordRef, Axis)` down to a `&mut VQ`, but every one of its
// callers only ever read/wrote that `VQ`'s scalar `.kernel`/`.shift`
// fields, never any other field of the whole `Point`/`ComponentReference`
// -- so `apply_polymorphism` below now flattens `.contours`/`.references`
// into plain `Vec<Pos>` kernel arrays once (immutably) for
// `apply_coords`/`fill_the_gaps` to do their scalar math over, then makes
// one more flattening pass (mutably, the only place in this cluster that
// still needs `&mut Point`/`&mut ComponentReference`) to write the
// results back -- both passes in the same `contours`-then-`references`
// order, so point `j` in one pass is the same point as `j` in the other.
#[derive(Copy, Clone, Debug)]
pub struct PackedDeltaRun {
    pub length: ShapeId,
    pub wide: bool,
    pub zero: bool,
}
#[derive(Copy, Clone, Debug)]
pub struct PackedPointRun {
    pub length: ShapeId,
    pub wide: bool,
}
fn next_point<'a>(contours: &'a mut ContourList, cc: &mut ShapeId, cp: &mut ShapeId) -> &'a mut Point {
    // A contour can be zero-length: `read_simple_glyph`'s endpoint
    // arithmetic allows `n == 0` (a contour whose endpoint equals the
    // running total minus one), and the wire format has no rule against
    // it. A single `if` here only advances past *one* exhausted contour
    // per call -- two or more consecutive zero-length contours land on
    // the next-but-one empty contour and index it at 0 anyway, panicking
    // ("index out of bounds: the len is 0"), a fuzzer-found crash. `while`
    // instead skips every exhausted contour in a row, however many there
    // are, before indexing.
    while *cp as usize >= contours[*cc as usize].len() {
        *cp = 0 as ShapeId;
        *cc = (*cc as i32 + 1_i32) as ShapeId;
    }
    let point = &mut contours[*cc as usize][*cp as usize];
    *cp = (*cp).wrapping_add(1);
    point
}
// Each glyph's bytes come from its own `loca` range. The outline format only
// describes itself (point counts from `endPtsOfContours`, a run-length flag
// stream, a component chain ended by a flag bit), so every read goes
// through `FontReader`: running past the glyph's range fails and the glyph
// is left empty.
fn read_simple_glyph(body: &[u8], number_of_contours: ShapeId) -> Option<Box<Glyph>> {
    let mut g: Box<Glyph> = new_glyf_glyph();
    let mut r = FontReader::new(body);
    r.require_room(number_of_contours as usize, 2).ok()?;
    // `u32`, not `ShapeId` (`u16`): the running total is `lastPoint + 1`,
    // and a wire-legal `lastPoint` of 0xFFFF makes that 65536 -- one past
    // `u16::MAX`. A `ShapeId` total would wrap that back to 0, silently
    // under-reading every flag/coordinate below (the loops bounded by it
    // would run zero times) despite `contours` having been built with the
    // full, correct point capacity.
    let mut points_in_glyph: u32 = 0;
    for _ in 0..number_of_contours {
        let last_point_in_current_contour: ShapeId = r.u16().unwrap(); // room already validated above
        // A non-monotonic `endPtsOfContours` gives a negative length; reject
        // it rather than turn it into a huge point count.
        let n = last_point_in_current_contour as i64 - points_in_glyph as i64 + 1;
        if n < 0 {
            return None;
        }
        let mut contour: Contour = Vec::new();
        glyf_contour_fill(&mut contour, n as usize);
        g.contours.push(contour);
        points_in_glyph = last_point_in_current_contour as u32 + 1;
    }
    let instruction_length: u16 = r.u16().ok()?;
    let instruction_bytes = r.bytes(instruction_length as usize).ok()?;
    g.instructions = instruction_bytes.to_vec();
    let mut flags: Vec<u8> = vec![0u8; points_in_glyph as usize];
    let mut flags_read_sofar: usize = 0;
    let mut current_contour: ShapeId = 0 as ShapeId;
    let mut current_contour_point_index: ShapeId = 0 as ShapeId;
    while flags_read_sofar < points_in_glyph as usize {
        let flag: PointFlags = PointFlags::from_bits_retain(r.u8().ok()?);
        flags[flags_read_sofar] = flag.bits();
        flags_read_sofar += 1;
        next_point(&mut g.contours, &mut current_contour, &mut current_contour_point_index).on_curve = flag.contains(PointFlags::ON_CURVE) as i8;
        if flag.contains(PointFlags::REPEAT) {
            let repeat: u8 = r.u8().ok()?;
            // A repeat run must not go past the declared point count.
            if flags_read_sofar + repeat as usize > points_in_glyph as usize {
                return None;
            }
            for _ in 0..repeat {
                flags[flags_read_sofar] = flag.bits();
                next_point(&mut g.contours, &mut current_contour, &mut current_contour_point_index).on_curve = flag.contains(PointFlags::ON_CURVE) as i8;
                flags_read_sofar += 1;
            }
        }
    }
    current_contour = 0 as ShapeId;
    current_contour_point_index = 0 as ShapeId;
    for &f in flags.iter() {
        let flag_0: PointFlags = PointFlags::from_bits_retain(f);
        let x: i16 = if flag_0.contains(PointFlags::X_SHORT) {
            let mag = r.u8().ok()? as i16;
            if flag_0.contains(PointFlags::POSITIVE_X) {
                mag
            } else {
                -mag
            }
        } else if flag_0.contains(PointFlags::SAME_X) {
            0
        } else {
            r.i16().ok()?
        };
        next_point(&mut g.contours, &mut current_contour, &mut current_contour_point_index).x =
            vq_create_still(x as Pos);
    }
    current_contour = 0 as ShapeId;
    current_contour_point_index = 0 as ShapeId;
    for &f in flags.iter() {
        let flag_1: PointFlags = PointFlags::from_bits_retain(f);
        let y: i16 = if flag_1.contains(PointFlags::Y_SHORT) {
            let mag = r.u8().ok()? as i16;
            if flag_1.contains(PointFlags::POSITIVE_Y) {
                mag
            } else {
                -mag
            }
        } else if flag_1.contains(PointFlags::SAME_Y) {
            0
        } else {
            r.i16().ok()?
        };
        next_point(&mut g.contours, &mut current_contour, &mut current_contour_point_index).y =
            vq_create_still(y as Pos);
    }
    let mut cx: VQ = (vq_neutral)();
    let mut cy: VQ = (vq_neutral)();
    for contour in g.contours.iter_mut() {
        for z in contour.iter_mut() {
            vq_inplace_plus(&mut cx, z.x.clone());
            vq_inplace_plus(&mut cy, z.y.clone());
            z.x = cx.clone();
            z.y = cy.clone();
        }
        contour.shrink_to_fit();
    }
    g.contours.shrink_to_fit();
    // `cx`/`cy` are plain owned locals, never moved out, so they auto-drop
    // when this function returns -- no explicit dispose call is needed.
    Some(g)
}
fn read_composite_glyph(body: &[u8]) -> Option<Box<Glyph>> {
    let mut g: Box<Glyph> = new_glyf_glyph();
    let mut r = FontReader::new(body);
    let mut glyph_has_instruction: bool = false;
    // A component chain that never clears `MORE_COMPONENTS` runs out of
    // bytes, which fails a read and rejects the glyph.
    loop {
        let flags = ComponentFlags::from_bits_retain(r.u16().ok()?);
        let index: GlyphId = r.u16().ok()? as GlyphId;
        let mut glyph_ref: ComponentReference = (glyf_component_reference_empty)();
        glyph_ref.glyph = handle_from_index(index) as GlyphHandle;
        if flags.contains(ComponentFlags::ARGS_ARE_XY_VALUES) {
            glyph_ref.is_anchored = std::cell::Cell::new(RefAnchorStatus::Xy);
            if flags.contains(ComponentFlags::ARG_1_AND_2_ARE_WORDS) {
                glyph_ref.x = std::cell::RefCell::new(vq_create_still(r.i16().ok()? as Pos));
                glyph_ref.y = std::cell::RefCell::new(vq_create_still(r.i16().ok()? as Pos));
            } else {
                glyph_ref.x = std::cell::RefCell::new(vq_create_still(r.i8().ok()? as Pos));
                glyph_ref.y = std::cell::RefCell::new(vq_create_still(r.i8().ok()? as Pos));
            }
        } else {
            glyph_ref.is_anchored = std::cell::Cell::new(RefAnchorStatus::AnchorAnchor);
            if flags.contains(ComponentFlags::ARG_1_AND_2_ARE_WORDS) {
                glyph_ref.outer = r.u16().ok()? as ShapeId;
                glyph_ref.inner = r.u16().ok()? as ShapeId;
            } else {
                glyph_ref.outer = r.u8().ok()? as ShapeId;
                glyph_ref.inner = r.u8().ok()? as ShapeId;
            }
        }
        if flags.contains(ComponentFlags::WE_HAVE_A_SCALE) {
            glyph_ref.d = from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
            glyph_ref.a = glyph_ref.d;
        } else if flags.contains(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE) {
            glyph_ref.a = from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
            glyph_ref.d = from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
        } else if flags.contains(ComponentFlags::WE_HAVE_A_TWO_BY_TWO) {
            glyph_ref.a = from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
            glyph_ref.b = from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
            glyph_ref.c = from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
            glyph_ref.d = from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
        }
        glyph_ref.round_to_grid = flags.contains(ComponentFlags::ROUND_XY_TO_GRID);
        glyph_ref.use_my_metrics = flags.contains(ComponentFlags::USE_MY_METRICS);
        if flags.contains(ComponentFlags::SCALED_COMPONENT_OFFSET)
            && (flags.contains(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE)
                || flags.contains(ComponentFlags::WE_HAVE_A_TWO_BY_TWO))
        {
            tracing::warn!("glyf: SCALED_COMPONENT_OFFSET is not supported.");
        }
        if flags.contains(ComponentFlags::WE_HAVE_INSTRUCTIONS) {
            glyph_has_instruction = true;
        }
        g.references.push(glyph_ref);
        if !(flags.contains(ComponentFlags::MORE_COMPONENTS)) {
            break;
        }
    }
    if glyph_has_instruction {
        let instruction_length: u16 = r.u16().ok()?;
        let instruction_bytes = r.bytes(instruction_length as usize).ok()?;
        g.instructions = instruction_bytes.to_vec();
    } else {
        g.instructions = Vec::new();
    }
    Some(g)
}
fn read_glyph(body: &[u8], offset: usize, length: usize) -> Option<Box<Glyph>> {
    let glyph_bytes = body.get(offset..)?.get(..length)?;
    let mut r = FontReader::new(glyph_bytes);
    let number_of_contours: i16 = r.i16().ok()?;
    let x_min = r.i16().ok()? as Pos;
    let y_min = r.i16().ok()? as Pos;
    let x_max = r.i16().ok()? as Pos;
    let y_max = r.i16().ok()? as Pos;
    // Every one of the 5 header reads above succeeded, so at least 10
    // bytes exist -- slicing the body here can't panic.
    let body = &glyph_bytes[10..];
    let mut g = if number_of_contours > 0 {
        read_simple_glyph(body, number_of_contours as ShapeId)?
    } else {
        read_composite_glyph(body)?
    };
    g.stat.x_min = x_min;
    g.stat.y_min = y_min;
    g.stat.x_max = x_max;
    g.stat.y_max = y_max;
    Some(g)
}
pub const GVAR_OFFSETS_ARE_LONG: i32 = 1_i32;
pub const EMBEDDED_PEAK_TUPLE: i32 = 0x8000_i32;
pub const INTERMEDIATE_REGION: i32 = 0x4000_i32;
pub const PRIVATE_POINT_NUMBERS: i32 = 0x2000_i32;
pub const TUPLE_INDEX_MASK: i32 = 0xfff_i32;
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
    if tuple_index & EMBEDDED_PEAK_TUPLE as u16 != 0 {
        bump += dimensions as usize * ::core::mem::size_of::<F2Dot14>();
    }
    if tuple_index & INTERMEDIATE_REGION as u16 != 0 {
        bump += 2 * dimensions as usize * ::core::mem::size_of::<F2Dot14>();
    }
    Some(tvh_offset + bump)
}
pub const POINT_COUNT_IS_WORD: i32 = 0x80_i32;
pub const POINT_COUNT_LONG_MASK: i32 = 0x7fff_i32;
pub const POINT_RUN_COUNT_MASK: i32 = 0x7f_i32;
pub const POINTS_ARE_WORDS: i32 = 0x80_i32;
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
    let n_points: u16 = if first_byte as i32 & POINT_COUNT_IS_WORD != 0 {
        let second_byte: u8 = r.u8().ok()?;
        (((first_byte as i32) << 8_i32
            | second_byte as i32)
            & POINT_COUNT_LONG_MASK) as u16
    } else {
        first_byte as u16
    };
    let mut point_indeces: Vec<ShapeId>;
    if n_points as i32 > 0_i32 {
        let mut run: PackedPointRun = PackedPointRun {
            length: 0 as ShapeId,
            wide: false,
        };
        let mut j_point: ShapeId = 0 as ShapeId;
        point_indeces = Vec::with_capacity(n_points as usize);
        for _ in 0..n_points {
            if run.length == 0 {
                let run_header: u8 = r.u8().ok()?;
                run.wide = run_header as i32 & POINTS_ARE_WORDS != 0;
                run.length = ((run_header as i32 & POINT_RUN_COUNT_MASK)
                    + 1_i32) as ShapeId;
            }
            let mut point_number: i16 = j_point as i16;
            if run.wide {
                // Native-endian, unlike the big-endian wide runs elsewhere:
                // a long-standing bug kept so output does not change for
                // fonts that use a wide point-number run here. See
                // RUST_MIGRATION.md.
                let b = r.bytes(2).ok()?;
                let raw = u16::from_ne_bytes([b[0], b[1]]);
                point_number =
                    (point_number as i32 + raw as i32) as i16;
            } else {
                point_number = (point_number as i32 + r.u8().ok()? as i32) as i16;
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
pub const DELTAS_ARE_ZERO: i32 = 0x80_i32;
pub const DELTAS_ARE_WORDS: i32 = 0x40_i32;
pub const DELTA_RUN_COUNT_MASK: i32 = 0x3f_i32;
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
        let mut delta: i16 = 0_i16;
        if run.length == 0 {
            let run_header: u8 = r.u8().ok()?;
            run.zero = run_header as i32 & DELTAS_ARE_ZERO != 0;
            run.wide = run_header as i32 & DELTAS_ARE_WORDS != 0;
            run.length = ((run_header as i32 & DELTA_RUN_COUNT_MASK)
                + 1_i32) as ShapeId;
        }
        if !run.zero {
            if run.wide {
                delta = r.i16().ok()?;
            } else {
                delta = r.i8().ok()? as i16;
            }
        }
        deltas[filled as usize] = delta as Pos;
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
                if j_next as i32
                    == j_max as i32 - 1_i32
                {
                    j_next = j_min;
                } else {
                    j_next = (j_next as i32 + 1_i32) as ShapeId;
                }
                if j_next as i32 == j as i32 {
                    break;
                }
            }
            let mut j_prev: ShapeId = j;
            while !nudges[j_prev as usize].is_touched() {
                if j_prev as i32 == j_min as i32 {
                    j_prev = (j_max as i32 - 1_i32) as ShapeId;
                } else {
                    j_prev = (j_prev as i32 - 1_i32) as ShapeId;
                }
                if j_prev as i32 == j as i32 {
                    break;
                }
            }
            if nudges[j_next as usize].is_touched() && nudges[j_prev as usize].is_touched() {
                let untouch_j: F16Dot16 =
                    to_fixed(kernel[j as usize]);
                let untouch_prev: F16Dot16 =
                    to_fixed(kernel[j_prev as usize] as f64);
                let untouch_next: F16Dot16 =
                    to_fixed(kernel[j_next as usize] as f64);
                let delta_prev: F16Dot16 = to_fixed(
                    nudges[j_prev as usize].unwrap_delta().quantity as f64,
                );
                let delta_next: F16Dot16 = to_fixed(
                    nudges[j_next as usize].unwrap_delta().quantity as f64,
                );
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
fn apply_coords(
    total_points: ShapeId,
    contour_lens: &[usize],
    kernel: &[Pos],
    n_touched_points: ShapeId,
    tuple_delta: &[Pos],
    points: &[ShapeId],
    r: &Rc<VqRegion>,
) -> Vec<VqSegment> {
    let mut nudges: Vec<VqSegment> = Vec::with_capacity(total_points as usize);
    for _ in 0..total_points {
        nudges.push(VqSegment::Delta(VqSegmentDelta {
            quantity: 0_i32 as Pos,
            touched: false,
            region: Rc::clone(r),
        }));
    }
    // `n_touched_points` may be smaller than the arrays it counts.
    for (&idx, &delta) in points
        .iter()
        .zip(tuple_delta.iter())
        .take(n_touched_points as usize)
    {
        if (idx as i32) < total_points as i32 {
            let d = nudges[idx as usize].delta_mut();
            d.touched = true;
            d.quantity += delta;
        }
    }
    let mut j_first: ShapeId = 0 as ShapeId;
    for &len in contour_lens {
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
#[inline]
fn apply_polymorphism(
    total_points: ShapeId,
    glyph: &mut Glyph,
    n_touched_points: ShapeId,
    points: &[ShapeId],
    delta_x: &[Pos],
    delta_y: &[Pos],
    r: &Rc<VqRegion>,
) {
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

    let nudges_x = apply_coords(
        total_points,
        &contour_lens,
        &kernel_x,
        n_touched_points,
        delta_x,
        points,
        r,
    );
    let nudges_y = apply_coords(
        total_points,
        &contour_lens,
        &kernel_y,
        n_touched_points,
        delta_y,
        points,
        r,
    );

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

    if (total_points as i32 + 1_i32) < n_touched_points as i32 {
        vq_add_delta(
            &mut glyph.horizontal_origin,
            true,
            r,
            delta_x[total_points as usize],
        );
        vq_add_delta(
            &mut glyph.advance_width,
            true,
            r,
            delta_x[(total_points as i32 + 1_i32) as usize] - delta_x[total_points as usize],
        );
    }
    if (total_points as i32 + 3_i32) < n_touched_points as i32 {
        vq_add_delta(
            &mut glyph.vertical_origin,
            true,
            r,
            delta_y[(total_points as i32 + 2_i32) as usize],
        );
        vq_add_delta(
            &mut glyph.advance_height,
            true,
            r,
            delta_y[(total_points as i32 + 2_i32) as usize]
                - delta_y[(total_points as i32 + 3_i32) as usize],
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
            start: (if peak_val <= 0_i32 as Pos {
                -1_i32
            } else {
                0_i32
            }) as Pos,
            peak: peak_val,
            end: (if peak_val >= 0_i32 as Pos {
                1_i32
            } else {
                0_i32
            }) as Pos,
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
    let total_delta_entries: ShapeId = (total_points as i32
        + ctx.n_phantom_points as i32)
        as ShapeId;

    let mut header = FontReader::new(gvar).at(gvd_offset).ok()?;
    let raw_tuple_variation_count = header.u16().ok()?;
    let data_offset_field = header.u16().ok()?;
    let n_tuples: u16 = raw_tuple_variation_count & 0xfff_u16;
    let has_shared_point_numbers: bool = raw_tuple_variation_count & 0x8000_u16 != 0;
    let mut tvh_offset: usize = gvd_offset + 4;

    // Empty means the glyph has no shared point numbers.
    let mut shared_point_indeces: Vec<ShapeId> = Vec::new();
    let mut data_offset: usize = gvd_offset + data_offset_field as usize;
    if has_shared_point_numbers {
        let (new_offset, indeces) = parse_point_numbers(gvar, data_offset, total_delta_entries)?;
        data_offset = new_offset;
        shared_point_indeces = indeces;
    }
    let mut tsd_start: usize = 0_usize;
    for _ in 0..n_tuples {
        let mut th = FontReader::new(gvar).at(tvh_offset).ok()?;
        let variation_data_size = th.u16().ok()?;
        let tuple_index_raw = th.u16().ok()?;
        let tuple_index: ShapeId = (tuple_index_raw & TUPLE_INDEX_MASK as u16) as ShapeId;
        let has_embedded_peak: bool = tuple_index_raw & EMBEDDED_PEAK_TUPLE as u16 != 0;
        let has_intermediate: bool = tuple_index_raw & INTERMEDIATE_REGION as u16 != 0;

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
        let r: Rc<VqRegion> =
            fvar_register_region(ctx.fvar.as_deref_mut().expect("fvar checked non-null by polymorphize"), region);

        let tsd = data_offset + tsd_start;
        // `point_indeces` borrows `shared_point_indeces` by default and
        // only owns a private `Vec` -- built fresh by `parse_point_numbers`
        // -- when `PRIVATE_POINT_NUMBERS` is set for this tuple.
        let n_points: ShapeId;
        let point_indeces: ::std::borrow::Cow<[ShapeId]>;
        let after_points: usize;
        if tuple_index_raw & PRIVATE_POINT_NUMBERS as u16 != 0 {
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
            let mut delta_x: Vec<Pos> = vec![0 as Pos; n_points as usize];
            let mut delta_y: Vec<Pos> = vec![0 as Pos; n_points as usize];
            let after_x = read_packed_delta(gvar, after_points, n_points, &mut delta_x)?;
            read_packed_delta(gvar, after_x, n_points, &mut delta_y)?;
            // `apply_polymorphism` is a safe `fn` now; `glyph` reborrows
            // here exactly as it does across every iteration of this loop.
            apply_polymorphism(total_points, glyph, n_points, &point_indeces, &delta_x, &delta_y, &r);
        }
        tsd_start = tsd_start.wrapping_add(variation_data_size as usize);
        tvh_offset = next_tvh_offset(gvar, tvh_offset, ctx.dimensions)?;
    }
    Some(())
}
// Every `GVARHeader`/`glyphVariationDataOffsets` read below is checked: the
// per-glyph offset array (`glyphVariationDataOffsets[j]` for every `j` up
// to `num_glyphs`) must fit the `gvar` table, and so must each
// `glyphVariationDataArrayOffset + glyphVariationDataOffset` sum.
#[inline]
fn polymorphize(packet: &Packet, glyf: &mut GlyfTable, ctx: &mut GlyfIOContext<'_>) {
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
    let offsets_are_long = flags & GVAR_OFFSETS_ARE_LONG as u16 != 0;
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
                .map(|v| v as u32 * 2)
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
            coord_dimensions: 2_u8,
            allow_iup: !glyph_slot.as_deref().unwrap().contours.is_empty(),
            n_phantom_points: ctx.n_phantom_points,
        };
        polymorphize_glyph(glyph_slot.as_deref_mut().unwrap(), &mut tpctx, gvar, gvd_offset);
    }
}
pub fn read_glyf(packet: &Packet, ctx: &mut GlyfIOContext<'_>) -> Option<GlyfTable> {
    let num_glyphs = ctx.num_glyphs;
    let mut offsets: Vec<u32> = vec![0u32; num_glyphs as usize + 1];

    // `__fortable_*`/`current_block` (goto emulation) -> the same
    // `.iter().find()` idiom every other already-migrated table reader in
    // this crate uses.
    let loca_corrupted = || {
        tracing::warn!("table 'loca' corrupted.\n");
    };
    let Some(loca) = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_LOCA) else {
        loca_corrupted();
        return None;
    };
    // Each `loca` read is checked, for whichever format (short or long)
    // the table uses.
    let mut loca_r = FontReader::new(&loca.data);
    let mut found_loca = true;
    for j in 0..=(num_glyphs as u32) {
        let v = if ctx.loca_is_long {
            match loca_r.u32() {
                Ok(v) => v,
                Err(_) => {
                    found_loca = false;
                    break;
                }
            }
        } else {
            match loca_r.u16() {
                Ok(v) => (v as u32) * 2,
                Err(_) => {
                    found_loca = false;
                    break;
                }
            }
        };
        if j > 0 && v < offsets[(j - 1) as usize] {
            found_loca = false;
            break;
        }
        offsets[j as usize] = v;
    }
    if !found_loca {
        loca_corrupted();
        return None;
    }

    let glyf_piece = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_GLYF)?;
    if glyf_piece.length < offsets[num_glyphs as usize] {
        tracing::warn!("table 'glyf' corrupted.\n");
        return None;
    }
    let mut glyf_val: GlyfTable = Vec::with_capacity(num_glyphs as usize);
    for j0 in 0..num_glyphs {
        if offsets[j0 as usize] < offsets[j0 as usize + 1] {
            let glyph_length = offsets[j0 as usize + 1] - offsets[j0 as usize];
            // A malformed individual glyph (an unbounded component chain,
            // a flag/coordinate stream that runs past its own declared
            // byte range, ...) now fails cleanly inside
            // `read_glyph` instead of reading adjacent bytes --
            // fall back to an empty glyph for this one GID rather than
            // failing the whole table, the same degradation the
            // zero-length-range case below already used.
            let g = read_glyph(
                &glyf_piece.data,
                offsets[j0 as usize] as usize,
                glyph_length as usize,
            )
            .unwrap_or_else(new_glyf_glyph);
            glyf_val.push(Some(g));
        } else {
            glyf_val.push(Some(new_glyf_glyph()));
        }
    }
    let mut glyf = Some(glyf_val);
    if let Some(g) = glyf.as_mut() {
        polymorphize(packet, g, ctx);
    }
    glyf
}

#[cfg(test)]
mod glyf_read_tests {
    use super::*;
    use crate::vf::vq::vq_get_still;

    fn still(v: &VQ) -> Pos {
        vq_get_still(v.clone())
    }

    #[test]
    fn simple_glyph_reads_one_contour_with_full_width_coordinates() {
        // numberOfContours=1, bbox, endPtsOfContours[0]=1 (2 points),
        // instructionLength=0, flags=[ON_CURVE, ON_CURVE] (no X_SHORT/
        // SAME_X or Y_SHORT/SAME_Y, so each coordinate is a full i16).
        let mut data = [0u8; 24];
        data[0..2].copy_from_slice(&1i16.to_be_bytes());
        data[10..12].copy_from_slice(&1u16.to_be_bytes()); // endPts[0]
        data[12..14].copy_from_slice(&0u16.to_be_bytes()); // instructionLength
        data[14] = 0x01; // flag point0: ON_CURVE
        data[15] = 0x01; // flag point1: ON_CURVE
        data[16..18].copy_from_slice(&5i16.to_be_bytes()); // x0
        data[18..20].copy_from_slice(&7i16.to_be_bytes()); // x1
        data[20..22].copy_from_slice(&3i16.to_be_bytes()); // y0
        data[22..24].copy_from_slice(&9i16.to_be_bytes()); // y1
        {
            let g = read_glyph(
                &data,
                0,
                data.len(),
            );
            let g = g.unwrap();
            assert_eq!(g.contours.len(), 1);
            assert_eq!(g.contours[0].len(), 2);
            // glyf coordinates are deltas from the previous point (point 0
            // from the implicit origin), accumulated by the function's own
            // trailing cx/cy pass -- point1 = point0 + its own delta.
            assert_eq!(still(&g.contours[0][0].x), 5.0);
            assert_eq!(still(&g.contours[0][1].x), 12.0);
            assert_eq!(still(&g.contours[0][0].y), 3.0);
            assert_eq!(still(&g.contours[0][1].y), 12.0);
        }
    }

    #[test]
    fn composite_glyph_reads_one_xy_anchored_component() {
        // numberOfContours=-1 (composite), bbox, one component:
        // flags=ARGS_ARE_XY_VALUES|ARG_1_AND_2_ARE_WORDS (no
        // MORE_COMPONENTS), glyphIndex=5, x=10, y=20.
        let mut data = [0u8; 18];
        data[0..2].copy_from_slice(&(-1i16).to_be_bytes());
        data[10..12].copy_from_slice(&3u16.to_be_bytes()); // flags = 2|1
        data[12..14].copy_from_slice(&5u16.to_be_bytes()); // glyphIndex
        data[14..16].copy_from_slice(&10i16.to_be_bytes());
        data[16..18].copy_from_slice(&20i16.to_be_bytes());
        {
            let g = read_glyph(
                &data,
                0,
                data.len(),
            );
            let g = g.unwrap();
            assert_eq!(g.references.len(), 1);
            assert_eq!(g.references[0].glyph.index, 5);
            assert_eq!(still(&g.references[0].x.borrow()), 10.0);
            assert_eq!(still(&g.references[0].y.borrow()), 20.0);
            assert_eq!(g.references[0].is_anchored.get(), RefAnchorStatus::Xy);
        }
    }

    #[test]
    fn simple_glyph_truncated_flag_stream_is_rejected_instead_of_reading_oob() {
        // Same header as the well-formed case above but cut off right
        // after the first flag byte -- the second flag and every
        // coordinate are missing.
        let mut data = [0u8; 15];
        data[0..2].copy_from_slice(&1i16.to_be_bytes());
        data[10..12].copy_from_slice(&1u16.to_be_bytes());
        data[12..14].copy_from_slice(&0u16.to_be_bytes());
        data[14] = 0x01;
        {
            let g = read_glyph(
                &data,
                0,
                data.len(),
            );
            assert!(g.is_none());
        }
    }

    #[test]
    fn simple_glyph_with_a_zero_length_contour_between_two_real_ones_does_not_panic() {
        // A fuzzer found this: `endPtsOfContours` has no rule against a
        // contour whose endpoint equals the running point total minus
        // one -- a zero-length contour, geometrically meaningless but
        // arithmetically legal. `next_point` (the shared point-cursor
        // walked while reading flags/coordinates) used to skip past only
        // *one* exhausted contour per call; landing on a zero-length
        // contour immediately after skipped nothing further and indexed
        // it at 0 anyway, panicking ("index out of bounds: the len is 0
        // but the index is 0").
        //
        // Three contours, endPtsOfContours = [0, 0, 1]: contour 0 has 1
        // point (running total 0->1), contour 1 has 0 points (endpoint 0
        // against a running total of 1 gives length 0), contour 2 has 1
        // point (running total 1->2). Reading the second of the 2 total
        // flags must walk through contour 1 without landing on it.
        let mut data = [0u8; 28];
        data[0..2].copy_from_slice(&3i16.to_be_bytes()); // numberOfContours
        data[10..12].copy_from_slice(&0u16.to_be_bytes()); // endPts[0]
        data[12..14].copy_from_slice(&0u16.to_be_bytes()); // endPts[1] (zero-length contour)
        data[14..16].copy_from_slice(&1u16.to_be_bytes()); // endPts[2]
        data[16..18].copy_from_slice(&0u16.to_be_bytes()); // instructionLength
        data[18] = 0x01; // flag for contour 0's point: ON_CURVE
        data[19] = 0x01; // flag for contour 2's point: ON_CURVE
        data[20..22].copy_from_slice(&5i16.to_be_bytes()); // x0
        data[22..24].copy_from_slice(&7i16.to_be_bytes()); // x1
        data[24..26].copy_from_slice(&3i16.to_be_bytes()); // y0
        data[26..28].copy_from_slice(&9i16.to_be_bytes()); // y1
        {
            let g = read_glyph(
                &data,
                0,
                data.len(),
            );
            let g = g.unwrap();
            assert_eq!(g.contours.len(), 3);
            assert_eq!(g.contours[0].len(), 1);
            assert_eq!(g.contours[1].len(), 0);
            assert_eq!(g.contours[2].len(), 1);
            assert_eq!(still(&g.contours[0][0].x), 5.0);
            assert_eq!(still(&g.contours[2][0].x), 12.0);
        }
    }

    #[test]
    fn composite_glyph_more_components_never_cleared_terminates_and_is_rejected() {
        // The only loop terminator is the MORE_COMPONENTS bit, so a
        // component chain that always sets it and then runs out of data
        // must stop at the glyph's own bytes. One full component record
        // with MORE_COMPONENTS set, then nothing: must terminate (not hang)
        // and reject.
        let mut data = [0u8; 18];
        data[0..2].copy_from_slice(&(-1i16).to_be_bytes());
        data[10..12].copy_from_slice(&35u16.to_be_bytes()); // flags = 2|1|32 (MORE_COMPONENTS)
        data[12..14].copy_from_slice(&5u16.to_be_bytes());
        data[14..16].copy_from_slice(&10i16.to_be_bytes());
        data[16..18].copy_from_slice(&20i16.to_be_bytes());
        {
            let g = read_glyph(
                &data,
                0,
                data.len(),
            );
            assert!(g.is_none());
        }
    }

    #[test]
    fn non_monotonic_end_points_of_contours_is_rejected_not_a_huge_allocation() {
        // contour 0 ends at point 5 (6 points); contour 1 ends at point 2
        // -- fewer than contour 0 already claimed. Computing this contour's
        // point count in signed arithmetic and casting to `usize` would turn
        // the negative result into a number near `usize::MAX`. Must reject
        // instead.
        let mut data = [0u8; 14];
        data[0..2].copy_from_slice(&2i16.to_be_bytes());
        data[10..12].copy_from_slice(&5u16.to_be_bytes());
        data[12..14].copy_from_slice(&2u16.to_be_bytes());
        {
            let g = read_glyph(
                &data,
                0,
                data.len(),
            );
            assert!(g.is_none());
        }
    }

    #[test]
    fn repeat_run_overrunning_the_declared_point_count_is_rejected_not_a_panic() {
        // endPtsOfContours[0]=1 declares exactly 2 points. The first flag
        // sets REPEAT with a run of 5 -- 1 + 5 = 6 total flags, four more
        // than declared. A repeat run must stay within the declared point
        // count; must reject.
        let mut data = [0u8; 16];
        data[0..2].copy_from_slice(&1i16.to_be_bytes());
        data[10..12].copy_from_slice(&1u16.to_be_bytes());
        data[12..14].copy_from_slice(&0u16.to_be_bytes());
        data[14] = 0x09; // REPEAT | ON_CURVE
        data[15] = 5; // repeat count
        {
            let g = read_glyph(
                &data,
                0,
                data.len(),
            );
            assert!(g.is_none());
        }
    }

    #[test]
    fn header_shorter_than_ten_bytes_is_rejected_instead_of_reading_oob() {
        let data = [0u8; 5];
        {
            let g = read_glyph(
                &data,
                0,
                data.len(),
            );
            assert!(g.is_none());
        }
    }
}

#[cfg(test)]
mod gvar_polymorphize_tests {
    use super::*;
    use crate::vf::region::vq_create_region;

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
        apply_polymorphism(3, &mut glyph, 2, &points, &delta_x, &delta_y, &r);

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
        apply_polymorphism(2, &mut glyph, 2, &points, &delta_x, &delta_y, &r);

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
