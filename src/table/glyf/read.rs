//! Reading the `glyf` and `loca` tables into outlines. When the font has
//! variation axes, `gvar` is applied afterwards (see `gvar`).

use crate::font::sfnt::Packet;
use crate::support::handle::{GlyphHandle, handle_from_index};
use crate::support::primitives::{F2Dot14, GlyphId, Pos, Scale, ShapeId, from_f2dot14};
use crate::table::glyf::gvar::polymorphize;
use crate::table::glyf::{
    ComponentFlags, ComponentReference, Contour, ContourList, GlyfIOContext, GlyfTable, Glyph,
    Point, PointFlags, RefAnchorStatus,
};
use crate::table::glyf::{glyf_component_reference_empty, glyf_contour_fill, new_glyf_glyph};
use crate::vf::vq::{VQ, vq_create_still, vq_inplace_plus, vq_neutral};
use otfcc_binary::FontReader;

fn next_point<'a>(
    contours: &'a mut ContourList,
    cc: &mut ShapeId,
    cp: &mut ShapeId,
) -> &'a mut Point {
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
        *cc = (i32::from(*cc) + 1) as ShapeId;
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
    let points_in_glyph = read_contour_ends(&mut r, number_of_contours, &mut g.contours)?;
    let instruction_length: u16 = r.u16().ok()?;
    let instruction_bytes = r.bytes(instruction_length as usize).ok()?;
    g.instructions = instruction_bytes.to_vec();
    let flags = read_point_flags(&mut r, points_in_glyph, &mut g.contours)?;
    let mut current_contour: ShapeId = 0 as ShapeId;
    let mut current_contour_point_index: ShapeId = 0 as ShapeId;
    for &f in flags.iter() {
        let x = read_coordinate(&mut r, PointFlags::from_bits_retain(f), &X_FLAGS)?;
        next_point(
            &mut g.contours,
            &mut current_contour,
            &mut current_contour_point_index,
        )
        .x = vq_create_still(Pos::from(x));
    }
    current_contour = 0 as ShapeId;
    current_contour_point_index = 0 as ShapeId;
    for &f in flags.iter() {
        let y = read_coordinate(&mut r, PointFlags::from_bits_retain(f), &Y_FLAGS)?;
        next_point(
            &mut g.contours,
            &mut current_contour,
            &mut current_contour_point_index,
        )
        .y = vq_create_still(Pos::from(y));
    }
    make_coordinates_absolute(&mut g.contours);
    Some(g)
}
/// Reads `endPtsOfContours` and pushes one contour of the right length per
/// entry onto `contours`. Returns the glyph's point count, or `None` if the
/// array does not fit or the end points go backwards.
fn read_contour_ends(
    r: &mut FontReader<'_>,
    number_of_contours: ShapeId,
    contours: &mut ContourList,
) -> Option<u32> {
    r.require_room(number_of_contours as usize, 2).ok()?;
    // `u32`, not `ShapeId` (`u16`): the running total is `lastPoint + 1`,
    // and a wire-legal `lastPoint` of 0xFFFF makes that 65536 -- one past
    // `u16::MAX`. A `ShapeId` total would wrap that back to 0, silently
    // under-reading every flag/coordinate below (the loops bounded by it
    // would run zero times) despite `contours` having been built with the
    // full, correct point capacity.
    let mut points_in_glyph: u32 = 0;
    for _ in 0..number_of_contours {
        let last_point_in_current_contour: ShapeId =
            r.u16().expect("require_room above covered this read");
        // A non-monotonic `endPtsOfContours` gives a negative length; reject
        // it rather than turn it into a huge point count.
        let n = i64::from(last_point_in_current_contour) - i64::from(points_in_glyph) + 1;
        if n < 0 {
            return None;
        }
        let mut contour: Contour = Vec::new();
        glyf_contour_fill(&mut contour, n as usize);
        contours.push(contour);
        points_in_glyph = u32::from(last_point_in_current_contour) + 1;
    }
    Some(points_in_glyph)
}
/// Reads the run-length coded flags of `points_in_glyph` points, one byte
/// per point in the result, and sets each point's `on_curve`. Returns `None`
/// if the flags run past the glyph or a repeat run past the point count.
fn read_point_flags(
    r: &mut FontReader<'_>,
    points_in_glyph: u32,
    contours: &mut ContourList,
) -> Option<Vec<u8>> {
    let mut flags: Vec<u8> = vec![0u8; points_in_glyph as usize];
    let mut flags_read_sofar: usize = 0;
    let mut current_contour: ShapeId = 0 as ShapeId;
    let mut current_contour_point_index: ShapeId = 0 as ShapeId;
    while flags_read_sofar < points_in_glyph as usize {
        let flag: PointFlags = PointFlags::from_bits_retain(r.u8().ok()?);
        flags[flags_read_sofar] = flag.bits();
        flags_read_sofar += 1;
        next_point(
            contours,
            &mut current_contour,
            &mut current_contour_point_index,
        )
        .on_curve = i8::from(flag.contains(PointFlags::ON_CURVE));
        if flag.contains(PointFlags::REPEAT) {
            let repeat: u8 = r.u8().ok()?;
            // A repeat run must not go past the declared point count.
            if flags_read_sofar + repeat as usize > points_in_glyph as usize {
                return None;
            }
            for _ in 0..repeat {
                flags[flags_read_sofar] = flag.bits();
                next_point(
                    contours,
                    &mut current_contour,
                    &mut current_contour_point_index,
                )
                .on_curve = i8::from(flag.contains(PointFlags::ON_CURVE));
                flags_read_sofar += 1;
            }
        }
    }
    Some(flags)
}
/// The three flag bits that say how one axis' coordinate is stored.
struct AxisFlags {
    short: PointFlags,
    positive: PointFlags,
    same: PointFlags,
}
const X_FLAGS: AxisFlags = AxisFlags {
    short: PointFlags::X_SHORT,
    positive: PointFlags::POSITIVE_X,
    same: PointFlags::SAME_X,
};
const Y_FLAGS: AxisFlags = AxisFlags {
    short: PointFlags::Y_SHORT,
    positive: PointFlags::POSITIVE_Y,
    same: PointFlags::SAME_Y,
};
/// Reads one point's coordinate change along one axis: a signed byte when
/// the short bit is set, nothing (zero) when the same bit is set, else a
/// 16-bit value.
fn read_coordinate(r: &mut FontReader<'_>, flags: PointFlags, axis: &AxisFlags) -> Option<i16> {
    let value = if flags.contains(axis.short) {
        let mag = i16::from(r.u8().ok()?);
        if flags.contains(axis.positive) {
            mag
        } else {
            -mag
        }
    } else if flags.contains(axis.same) {
        0
    } else {
        r.i16().ok()?
    };
    Some(value)
}
/// Turns the stored coordinate changes into absolute coordinates, running
/// on from one contour to the next.
fn make_coordinates_absolute(contours: &mut ContourList) {
    let mut cx: VQ = (vq_neutral)();
    let mut cy: VQ = (vq_neutral)();
    for contour in contours.iter_mut() {
        for z in contour.iter_mut() {
            vq_inplace_plus(&mut cx, z.x.clone());
            vq_inplace_plus(&mut cy, z.y.clone());
            z.x = cx.clone();
            z.y = cy.clone();
        }
        contour.shrink_to_fit();
    }
    contours.shrink_to_fit();
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
                glyph_ref.x = std::cell::RefCell::new(vq_create_still(Pos::from(r.i16().ok()?)));
                glyph_ref.y = std::cell::RefCell::new(vq_create_still(Pos::from(r.i16().ok()?)));
            } else {
                glyph_ref.x = std::cell::RefCell::new(vq_create_still(Pos::from(r.i8().ok()?)));
                glyph_ref.y = std::cell::RefCell::new(vq_create_still(Pos::from(r.i8().ok()?)));
            }
        } else {
            glyph_ref.is_anchored = std::cell::Cell::new(RefAnchorStatus::AnchorAnchor);
            if flags.contains(ComponentFlags::ARG_1_AND_2_ARE_WORDS) {
                glyph_ref.outer = r.u16().ok()? as ShapeId;
                glyph_ref.inner = r.u16().ok()? as ShapeId;
            } else {
                glyph_ref.outer = ShapeId::from(r.u8().ok()?);
                glyph_ref.inner = ShapeId::from(r.u8().ok()?);
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
        // Both flags set is invalid, and both are ignored without
        // ARGS_ARE_XY_VALUES: the rasterizer's default, unscaled, applies.
        glyph_ref.scaled_offset = flags.contains(ComponentFlags::SCALED_COMPONENT_OFFSET)
            && !flags.contains(ComponentFlags::UNSCALED_COMPONENT_OFFSET)
            && flags.contains(ComponentFlags::ARGS_ARE_XY_VALUES);
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
    let x_min = Pos::from(r.i16().ok()?);
    let y_min = Pos::from(r.i16().ok()?);
    let x_max = Pos::from(r.i16().ok()?);
    let y_max = Pos::from(r.i16().ok()?);
    // Every one of the 5 header reads above succeeded, so at least 10
    // bytes exist -- slicing the body here can't panic.
    let body = &glyph_bytes[10..];
    // A glyph with no contours is still a simple glyph, and can still have
    // instructions; only a negative count marks a composite.
    let mut g = if number_of_contours >= 0 {
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
/// Reads every glyph's outline, using `ctx.num_glyphs` and
/// `ctx.loca_is_long` to read `loca`. Returns `None` if `loca` or `glyf` is missing or
/// `loca` is malformed; a single malformed glyph is read as empty instead.
/// The table always holds `ctx.num_glyphs` glyphs, each `Some`. If
/// `ctx.fvar` has axes, `gvar` is applied before returning.
pub fn read_glyf(packet: &Packet, ctx: &mut GlyfIOContext<'_>) -> Option<GlyfTable> {
    let num_glyphs = ctx.num_glyphs;
    let mut offsets: Vec<u32> = vec![0u32; num_glyphs as usize + 1];

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
    for j in 0..=u32::from(num_glyphs) {
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
                Ok(v) => u32::from(v) * 2,
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
            let g = read_glyph(&data, 0, data.len());
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
            let g = read_glyph(&data, 0, data.len());
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
            let g = read_glyph(&data, 0, data.len());
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
            let g = read_glyph(&data, 0, data.len());
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
            let g = read_glyph(&data, 0, data.len());
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
            let g = read_glyph(&data, 0, data.len());
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
            let g = read_glyph(&data, 0, data.len());
            assert!(g.is_none());
        }
    }

    #[test]
    fn header_shorter_than_ten_bytes_is_rejected_instead_of_reading_oob() {
        let data = [0u8; 5];
        {
            let g = read_glyph(&data, 0, data.len());
            assert!(g.is_none());
        }
    }
}
