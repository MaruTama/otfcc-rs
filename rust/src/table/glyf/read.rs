#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{GlyphHandle, handle_from_index};

use crate::font::caryll_sfnt::Packet;
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::font_reader::FontReader;
use crate::support::options::Options;
use crate::support::primitives::{
    F2Dot14, F16Dot16, FontFilePointer, GlyphId, Pos, Scale, ShapeId,
};

use crate::table::fvar::FvarTable;
use crate::table::glyf::{
    ComponentFlags, ComponentReference, Contour, ContourList, GlyfIOContext, GlyfTable, Glyph,
    GlyphPtr, Point, PointFlags, RefAnchorStatus,
};

use crate::support::primitives::{
    otfcc_f1616_muldiv, otfcc_from_f2dot14, otfcc_from_fixed, otfcc_to_fixed,
};
use crate::table::fvar::fvar_register_region;
use crate::table::glyf::{glyf_component_reference_empty, glyf_contour_fill, otfcc_new_glyf_glyph};
use crate::vf::region::{VqAxisSpan, VqRegion};
use crate::vf::region::{vq_create_region, vq_delete_region};
use crate::vf::vq::{VQ, VqSegment, VqSegmentDelta};
use crate::vf::vq::{
    vq_add_delta, vq_copy_replace, vq_create_still, vq_inplace_plus, vq_neutral, vq_replace,
};

// `GlyphVariationData`/`TupleVariationHeader`/`GVARHeader` (`#[repr(C,
// packed)]` structs cast directly onto raw `gvar` bytes) are gone: every
// field they described is now read by byte offset through `FontReader`
// instead (`polymorphize`/`polymorphize_glyph`/`next_tvh_offset`), which
// needed the real buffer length these pointer casts never carried. `be16`/
// `be32` (the manual byte-swaps those native-endian pointer reads needed)
// are gone with them -- `FontReader`'s reads are big-endian by
// construction.
#[derive(Copy, Clone)]
pub struct TuplePolymorphizerCtx {
    pub fvar: *mut FvarTable,
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
pub type CoordPartGetter = Option<unsafe extern "C" fn(*mut Point) -> *mut VQ>;
#[derive(Copy, Clone)]
pub struct PackedDeltaRun {
    pub length: ShapeId,
    pub wide: bool,
    pub zero: bool,
}
#[derive(Copy, Clone)]
pub struct PackedPointRun {
    pub length: ShapeId,
    pub wide: bool,
}
unsafe extern "C" fn next_point(
    contours: *mut ContourList,
    cc: *mut ShapeId,
    cp: *mut ShapeId,
) -> *mut Point {
    // A contour can be zero-length: `otfcc_read_simple_glyph`'s endpoint
    // arithmetic allows `n == 0` (a contour whose endpoint equals the
    // running total minus one), and the wire format has no rule against
    // it. A single `if` here only advances past *one* exhausted contour
    // per call -- two or more consecutive zero-length contours land on
    // the next-but-one empty contour and index it at 0 anyway, panicking
    // ("index out of bounds: the len is 0"), a fuzzer-found crash. `while`
    // instead skips every exhausted contour in a row, however many there
    // are, before indexing.
    while *cp as usize >= (&(*contours))[*cc as usize].len() {
        *cp = 0 as ShapeId;
        *cc = (*cc as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
    }
    let fresh8 = *cp;
    *cp = (*cp).wrapping_add(1);
    return &raw mut (&mut (*contours))[*cc as usize][fresh8 as usize];
}
// `otfcc_read_simple_glyph`/`otfcc_read_composite_glyph`/`otfcc_read_glyph`
// used to take no length at all -- just a raw `start: FontFilePointer` --
// and walk forward on nothing but the shapes the wire format implies
// (`endPtsOfContours[numberOfContours-1]+1` points, a run-length-coded flag
// stream terminated only by having read that many flags, a component chain
// terminated only by the `MORE_COMPONENTS` bit). Every one of those is
// attacker-controlled: a malformed `endPtsOfContours` never reaching the
// declared point count, or a composite glyph that never clears
// `MORE_COMPONENTS`, read straight past this glyph's own bytes with no
// guard at all (the plan's own writeup flags this file by name for exactly
// this). `otfcc_read_glyf` below now derives this glyph's exact byte range
// from its own (already-validated, monotonic) `loca` entries and passes it
// down as a `&[u8]`; every read here goes through `FontReader`, so running
// past that range now fails cleanly (`None`, the glyph becomes empty)
// instead of reading adjacent memory.
unsafe fn otfcc_read_simple_glyph(body: &[u8], number_of_contours: ShapeId) -> Option<Box<Glyph>> {
    let mut g: Box<Glyph> = otfcc_new_glyf_glyph();
    let contours: *mut ContourList = &raw mut (*g).contours;
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
        // The original computed this length in `c_int` (so a non-monotonic
        // `endPtsOfContours` -- a later entry smaller than the previous
        // one plus one -- went negative) and then cast straight to
        // `usize` for the fill count below; that cast turns a negative
        // `c_int` into a number near `usize::MAX`, and `glyf_contour_fill`
        // would then try to push that many points -- an unbounded-
        // allocation DoS on a malformed but otherwise tiny font. Reject
        // instead.
        let n = last_point_in_current_contour as i64 - points_in_glyph as i64 + 1;
        if n < 0 {
            return None;
        }
        let mut contour: Contour = Vec::new();
        glyf_contour_fill(&raw mut contour, n as usize);
        (*contours).push(contour);
        points_in_glyph = last_point_in_current_contour as u32 + 1;
    }
    let instruction_length: u16 = r.u16().ok()?;
    let instruction_bytes = r.bytes(instruction_length as usize).ok()?;
    (*g).instructions = instruction_bytes.to_vec();
    // A local `Vec<u8>` now, not a `__caryll_allocate_clean`'d/`free`'d
    // buffer -- dropped automatically at the end of this function.
    let mut flags: Vec<u8> = vec![0u8; points_in_glyph as usize];
    let mut flags_read_sofar: usize = 0;
    let mut current_contour: ShapeId = 0 as ShapeId;
    let mut current_contour_point_index: ShapeId = 0 as ShapeId;
    while flags_read_sofar < points_in_glyph as usize {
        let flag: PointFlags = PointFlags::from_bits_retain(r.u8().ok()?);
        flags[flags_read_sofar] = flag.bits();
        flags_read_sofar += 1;
        (*next_point(
            contours,
            &raw mut current_contour,
            &raw mut current_contour_point_index,
        ))
        .on_curve = flag.contains(PointFlags::ON_CURVE) as i8;
        if flag.contains(PointFlags::REPEAT) {
            let repeat: u8 = r.u8().ok()?;
            // The original indexed `flags[flags_read_sofar + j_0]` (a
            // fixed-size `Vec` now, pre-sized to exactly
            // `points_in_glyph`) with no check that a malformed repeat
            // run doesn't overrun the declared point count -- in C this
            // silently wrote past the buffer; in Rust it would panic.
            // Reject instead of either.
            if flags_read_sofar + repeat as usize > points_in_glyph as usize {
                return None;
            }
            for _ in 0..repeat {
                flags[flags_read_sofar] = flag.bits();
                (*next_point(
                    contours,
                    &raw mut current_contour,
                    &raw mut current_contour_point_index,
                ))
                .on_curve = flag.contains(PointFlags::ON_CURVE) as i8;
                flags_read_sofar += 1;
            }
        }
    }
    let mut coordinates_read: usize = 0;
    current_contour = 0 as ShapeId;
    current_contour_point_index = 0 as ShapeId;
    while coordinates_read < points_in_glyph as usize {
        let flag_0: PointFlags = PointFlags::from_bits_retain(flags[coordinates_read]);
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
        vq_replace(
            &raw mut (*(next_point
                as unsafe extern "C" fn(
                    *mut ContourList,
                    *mut ShapeId,
                    *mut ShapeId,
                ) -> *mut Point)(
                contours,
                &raw mut current_contour,
                &raw mut current_contour_point_index,
            ))
            .x,
            vq_create_still(x as Pos) as VQ,
        );
        coordinates_read += 1;
    }
    coordinates_read = 0;
    current_contour = 0 as ShapeId;
    current_contour_point_index = 0 as ShapeId;
    while coordinates_read < points_in_glyph as usize {
        let flag_1: PointFlags = PointFlags::from_bits_retain(flags[coordinates_read]);
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
        vq_replace(
            &raw mut (*(next_point
                as unsafe extern "C" fn(
                    *mut ContourList,
                    *mut ShapeId,
                    *mut ShapeId,
                ) -> *mut Point)(
                contours,
                &raw mut current_contour,
                &raw mut current_contour_point_index,
            ))
            .y,
            vq_create_still(y as Pos) as VQ,
        );
        coordinates_read += 1;
    }
    let mut cx: VQ = (vq_neutral)();
    let mut cy: VQ = (vq_neutral)();
    let mut j_1: ShapeId = 0 as ShapeId;
    while (j_1 as ::core::ffi::c_int) < number_of_contours as ::core::ffi::c_int {
        let mut k: ShapeId = 0 as ShapeId;
        while (k as usize) < (&(*contours))[j_1 as usize].len() {
            let z: *mut Point = &raw mut (&mut (*contours))[j_1 as usize][k as usize];
            vq_inplace_plus(&raw mut cx, (*z).x.clone());
            vq_inplace_plus(&raw mut cy, (*z).y.clone());
            vq_copy_replace(&raw mut (*z).x, cx.clone());
            vq_copy_replace(&raw mut (*z).y, cy.clone());
            k = k.wrapping_add(1);
        }
        (&mut (*contours))[j_1 as usize].shrink_to_fit();
        j_1 = j_1.wrapping_add(1);
    }
    (*contours).shrink_to_fit();
    // `cx`/`cy` are plain owned locals, never moved out, so they auto-drop
    // when this function returns -- no explicit dispose call is needed.
    Some(g)
}
unsafe fn otfcc_read_composite_glyph(body: &[u8], options: &Options) -> Option<Box<Glyph>> {
    let mut g: Box<Glyph> = otfcc_new_glyf_glyph();
    let mut r = FontReader::new(body);
    let mut glyph_has_instruction: bool = false;
    // The original's only loop terminator was the `MORE_COMPONENTS` bit --
    // a malformed composite glyph that never clears it read components
    // forever, straight past this glyph's own data (the plan's own
    // writeup calls this out by name). Every field read below now goes
    // through `FontReader`, so running out of bytes fails the `?` and
    // rejects the glyph instead of reading on.
    loop {
        let flags = ComponentFlags::from_bits_retain(r.u16().ok()?);
        let index: GlyphId = r.u16().ok()? as GlyphId;
        let mut ref_0: ComponentReference = (glyf_component_reference_empty)();
        ref_0.glyph = handle_from_index(index) as GlyphHandle;
        if flags.contains(ComponentFlags::ARGS_ARE_XY_VALUES) {
            ref_0.is_anchored = RefAnchorStatus::Xy;
            if flags.contains(ComponentFlags::ARG_1_AND_2_ARE_WORDS) {
                ref_0.x = vq_create_still(r.i16().ok()? as Pos);
                ref_0.y = vq_create_still(r.i16().ok()? as Pos);
            } else {
                ref_0.x = vq_create_still(r.i8().ok()? as Pos);
                ref_0.y = vq_create_still(r.i8().ok()? as Pos);
            }
        } else {
            ref_0.is_anchored = RefAnchorStatus::AnchorAnchor;
            if flags.contains(ComponentFlags::ARG_1_AND_2_ARE_WORDS) {
                ref_0.outer = r.u16().ok()? as ShapeId;
                ref_0.inner = r.u16().ok()? as ShapeId;
            } else {
                ref_0.outer = r.u8().ok()? as ShapeId;
                ref_0.inner = r.u8().ok()? as ShapeId;
            }
        }
        if flags.contains(ComponentFlags::WE_HAVE_A_SCALE) {
            ref_0.d = otfcc_from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
            ref_0.a = ref_0.d;
        } else if flags.contains(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE) {
            ref_0.a = otfcc_from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
            ref_0.d = otfcc_from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
        } else if flags.contains(ComponentFlags::WE_HAVE_A_TWO_BY_TWO) {
            ref_0.a = otfcc_from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
            ref_0.b = otfcc_from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
            ref_0.c = otfcc_from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
            ref_0.d = otfcc_from_f2dot14(r.i16().ok()? as F2Dot14) as Scale;
        }
        ref_0.round_to_grid = flags.contains(ComponentFlags::ROUND_XY_TO_GRID);
        ref_0.use_my_metrics = flags.contains(ComponentFlags::USE_MY_METRICS);
        if flags.contains(ComponentFlags::SCALED_COMPONENT_OFFSET)
            && (flags.contains(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE)
                || flags.contains(ComponentFlags::WE_HAVE_A_TWO_BY_TWO))
        {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"glyf: SCALED_COMPONENT_OFFSET is not supported."),
            );
        }
        if flags.contains(ComponentFlags::WE_HAVE_INSTRUCTIONS) {
            glyph_has_instruction = true;
        }
        (*g).references.push(ref_0);
        if !(flags.contains(ComponentFlags::MORE_COMPONENTS)) {
            break;
        }
    }
    if glyph_has_instruction {
        let instruction_length: u16 = r.u16().ok()?;
        let instruction_bytes = r.bytes(instruction_length as usize).ok()?;
        (*g).instructions = instruction_bytes.to_vec();
    } else {
        (*g).instructions = Vec::new();
    }
    Some(g)
}
unsafe fn otfcc_read_glyph(
    data: FontFilePointer,
    offset: u32,
    length: u32,
    options: &Options,
) -> Option<Box<Glyph>> {
    let glyph_bytes = ::core::slice::from_raw_parts(data.offset(offset as isize), length as usize);
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
        otfcc_read_simple_glyph(body, number_of_contours as ShapeId)?
    } else {
        otfcc_read_composite_glyph(body, options)?
    };
    g.stat.x_min = x_min;
    g.stat.y_min = y_min;
    g.stat.x_max = x_max;
    g.stat.y_max = y_max;
    Some(g)
}
pub const GVAR_OFFSETS_ARE_LONG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EMBEDDED_PEAK_TUPLE: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const INTERMEDIATE_REGION: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const PRIVATE_POINT_NUMBERS: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const TUPLE_INDEX_MASK: ::core::ffi::c_int = 0xfff as ::core::ffi::c_int;
// A `TupleVariationHeader` array has no length of its own -- each header's
// end (and so the next one's start) is only known after reading *this*
// header's own `tupleIndex` flags, which is why this can't be a simple
// `size_of::<TupleVariationHeader>() * n` stride. `tvh_offset` is an
// absolute byte offset into `gvar` (not a pointer) so every read here goes
// through `FontReader`'s bounds check instead of `.offset()`ing off the
// end of the table.
#[inline]
unsafe fn next_tvh_offset(gvar: &[u8], tvh_offset: usize, dimensions: u16) -> Option<usize> {
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
pub const POINT_COUNT_IS_WORD: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const POINT_COUNT_LONG_MASK: ::core::ffi::c_int = 0x7fff as ::core::ffi::c_int;
pub const POINT_RUN_COUNT_MASK: ::core::ffi::c_int = 0x7f as ::core::ffi::c_int;
pub const POINTS_ARE_WORDS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
/// Returns `(new absolute offset into `gvar`, point indices)` instead of
/// writing through two out-params -- `pc` (the count) is just
/// `point_indeces.len()` once the array is a `Vec`, so it disappears
/// entirely rather than needing to stay in sync with a separately-tracked
/// length. `None` on any read running past `gvar`'s own length -- this
/// used to walk a bare `FontFilePointer` with no length at all.
#[inline]
unsafe fn parse_point_numbers(
    gvar: &[u8],
    offset: usize,
    total_points: ShapeId,
) -> Option<(usize, Vec<ShapeId>)> {
    let mut r = FontReader::new(gvar).at(offset).ok()?;
    let first_byte: u8 = r.u8().ok()?;
    let n_points: u16 = if first_byte as ::core::ffi::c_int & POINT_COUNT_IS_WORD != 0 {
        let second_byte: u8 = r.u8().ok()?;
        (((first_byte as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | second_byte as ::core::ffi::c_int)
            & POINT_COUNT_LONG_MASK) as u16
    } else {
        first_byte as u16
    };
    let mut point_indeces: Vec<ShapeId>;
    if n_points as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut run: PackedPointRun = PackedPointRun {
            length: 0 as ShapeId,
            wide: false,
        };
        let mut j_point: ShapeId = 0 as ShapeId;
        point_indeces = Vec::with_capacity(n_points as usize);
        while (point_indeces.len() as ::core::ffi::c_int) < n_points as ::core::ffi::c_int {
            if run.length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let run_header: u8 = r.u8().ok()?;
                run.wide = run_header as ::core::ffi::c_int & POINTS_ARE_WORDS != 0;
                run.length = ((run_header as ::core::ffi::c_int & POINT_RUN_COUNT_MASK)
                    + 1 as ::core::ffi::c_int) as ShapeId;
            }
            let mut point_number: i16 = j_point as i16;
            if run.wide {
                // Deliberately native-endian, not big-endian, to match a
                // pre-existing bug in the original: `read_packed_delta`'s
                // own wide-run case calls `be16()` before use, but this
                // one read `*(data as *mut u16)` directly with no swap at
                // all. That's out of this PR's scope (parse-boundary
                // safety, not general correctness) to fix -- it would
                // change output for any well-formed font whose gvar data
                // actually uses a wide point-number run, which the golden
                // fixtures don't currently exercise either way. Preserved
                // as-is; see rust/README.md.
                let b = r.bytes(2).ok()?;
                let raw = u16::from_ne_bytes([b[0], b[1]]);
                point_number =
                    (point_number as ::core::ffi::c_int + raw as ::core::ffi::c_int) as i16;
            } else {
                let fresh7: u8 = r.u8().ok()?;
                point_number =
                    (point_number as ::core::ffi::c_int + fresh7 as ::core::ffi::c_int) as i16;
            }
            point_indeces.push(point_number as ShapeId);
            j_point = point_number as ShapeId;
            run.length = run.length.wrapping_sub(1);
        }
    } else {
        point_indeces = Vec::with_capacity(total_points as usize);
        let mut j: ShapeId = 0 as ShapeId;
        while (j as ::core::ffi::c_int) < total_points as ::core::ffi::c_int {
            point_indeces.push(j);
            j = j.wrapping_add(1);
        }
    }
    Some((r.pos(), point_indeces))
}
pub const DELTAS_ARE_ZERO: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const DELTAS_ARE_WORDS: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const DELTA_RUN_COUNT_MASK: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;
#[inline]
unsafe fn read_packed_delta(
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
    let mut filled: ShapeId = 0 as ShapeId;
    while (filled as ::core::ffi::c_int) < n_points as ::core::ffi::c_int {
        let mut delta: i16 = 0 as i16;
        if run.length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let run_header: u8 = r.u8().ok()?;
            run.zero = run_header as ::core::ffi::c_int & DELTAS_ARE_ZERO != 0;
            run.wide = run_header as ::core::ffi::c_int & DELTAS_ARE_WORDS != 0;
            run.length = ((run_header as ::core::ffi::c_int & DELTA_RUN_COUNT_MASK)
                + 1 as ::core::ffi::c_int) as ShapeId;
        }
        if !run.zero {
            if run.wide {
                delta = r.i16().ok()?;
            } else {
                delta = r.i8().ok()? as i16;
            }
        }
        deltas[filled as usize] = delta as Pos;
        filled = filled.wrapping_add(1);
        run.length = run.length.wrapping_sub(1);
    }
    Some(r.pos())
}
pub unsafe extern "C" fn get_x(z: *mut Point) -> *mut VQ {
    return &raw mut (*z).x;
}
pub unsafe extern "C" fn get_y(z: *mut Point) -> *mut VQ {
    return &raw mut (*z).y;
}
#[inline]
// `nudges`/`glyph_refs` are borrowed slices now, not raw pointers: this
// function neither owns nor frees either array, only reads (`glyph_refs`)
// or reads-then-writes (`nudges`) into them by index -- the same access
// shape `&mut [_]`/`&[_]` already model directly.
//
// Never a real FFI boundary -- internal call site only, same rationale
// as every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
unsafe fn fill_the_gaps(
    j_min: ShapeId,
    j_max: ShapeId,
    nudges: &mut [VqSegment],
    glyph_refs: &[*mut Point],
    getter: CoordPartGetter,
) {
    let mut j: ShapeId = j_min;
    while (j as ::core::ffi::c_int) < j_max as ::core::ffi::c_int {
        if !nudges[j as usize].is_touched() {
            let mut j_next: ShapeId = j;
            while !nudges[j_next as usize].is_touched() {
                if j_next as ::core::ffi::c_int
                    == j_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    j_next = j_min;
                } else {
                    j_next = (j_next as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
                }
                if j_next as ::core::ffi::c_int == j as ::core::ffi::c_int {
                    break;
                }
            }
            let mut j_prev: ShapeId = j;
            while !nudges[j_prev as usize].is_touched() {
                if j_prev as ::core::ffi::c_int == j_min as ::core::ffi::c_int {
                    j_prev = (j_max as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ShapeId;
                } else {
                    j_prev = (j_prev as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ShapeId;
                }
                if j_prev as ::core::ffi::c_int == j as ::core::ffi::c_int {
                    break;
                }
            }
            if nudges[j_next as usize].is_touched() && nudges[j_prev as usize].is_touched() {
                let untouch_j: F16Dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(glyph_refs[j as usize])).kernel
                        as ::core::ffi::c_double,
                );
                let untouch_prev: F16Dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(glyph_refs[j_prev as usize]))
                        .kernel as ::core::ffi::c_double,
                );
                let untouch_next: F16Dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(glyph_refs[j_next as usize]))
                        .kernel as ::core::ffi::c_double,
                );
                let delta_prev: F16Dot16 = otfcc_to_fixed(
                    nudges[j_prev as usize].unwrap_delta().quantity as ::core::ffi::c_double,
                );
                let delta_next: F16Dot16 = otfcc_to_fixed(
                    nudges[j_next as usize].unwrap_delta().quantity as ::core::ffi::c_double,
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
                    nudges[j as usize].delta_mut().quantity = otfcc_from_fixed(d_min) as Pos;
                } else if untouch_j >= u_max {
                    nudges[j as usize].delta_mut().quantity = otfcc_from_fixed(d_max) as Pos;
                } else {
                    nudges[j as usize].delta_mut().quantity = otfcc_from_fixed(otfcc_f1616_muldiv(
                        d_max - d_min,
                        untouch_j - u_min,
                        u_max - u_min,
                    )) as Pos;
                }
            }
        }
        j = j.wrapping_add(1);
    }
}
// `nudges` is a local `Vec<VqSegment>` now, not a `__caryll_allocate_
// clean`'d/`free`'d buffer -- built with exactly `total_points` entries
// by construction (the fill loop below runs exactly that many times),
// dropped automatically at the end of this function instead of needing
// an explicit `free` to match. `glyph_refs` is a borrowed slice, read
// but never owned or freed here (unchanged from before -- it was never
// this function's allocation).
//
// Never a real FFI boundary -- internal call site only, same rationale
// as every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
unsafe fn apply_coords(
    total_points: ShapeId,
    glyph: *mut Glyph,
    glyph_refs: &[*mut Point],
    n_touched_points: ShapeId,
    tuple_delta: *const Pos,
    points: *const ShapeId,
    r: *const VqRegion,
    getter: CoordPartGetter,
) {
    let mut nudges: Vec<VqSegment> = Vec::with_capacity(total_points as usize);
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_int) < total_points as ::core::ffi::c_int {
        nudges.push(VqSegment::Delta(VqSegmentDelta {
            quantity: 0 as ::core::ffi::c_int as Pos,
            touched: false,
            region: r,
        }));
        j = j.wrapping_add(1);
    }
    let mut j_0: ShapeId = 0 as ShapeId;
    while (j_0 as ::core::ffi::c_int) < n_touched_points as ::core::ffi::c_int {
        if !(*points.offset(j_0 as isize) as ::core::ffi::c_int
            >= total_points as ::core::ffi::c_int)
        {
            let idx = *points.offset(j_0 as isize) as usize;
            let d = nudges[idx].delta_mut();
            d.touched = true;
            d.quantity += *tuple_delta.offset(j_0 as isize);
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut j_first: ShapeId = 0 as ShapeId;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*glyph).contours.len() {
        let c: *mut Contour = &raw mut (&mut (*glyph).contours)[__caryll_index];
        while keep != 0 {
            fill_the_gaps(
                j_first,
                (j_first as usize).wrapping_add((*c).len()) as ShapeId,
                &mut nudges,
                glyph_refs,
                Some(get_x as unsafe extern "C" fn(*mut Point) -> *mut VQ),
            );
            j_first = (j_first as usize).wrapping_add((*c).len()) as ShapeId as ShapeId;
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut j_1: ShapeId = 0 as ShapeId;
    while (j_1 as ::core::ffi::c_int) < total_points as ::core::ffi::c_int {
        if !(nudges[j_1 as usize].unwrap_delta().quantity == 0.
            && nudges[j_1 as usize].is_touched())
        {
            let coordinate_part: *mut VQ =
                getter.expect("non-null function pointer")(glyph_refs[j_1 as usize]);
            (*coordinate_part).shift.push(nudges[j_1 as usize]);
        }
        j_1 = j_1.wrapping_add(1);
    }
}
#[inline]
unsafe fn apply_polymorphism(
    total_points: ShapeId,
    glyph: GlyphPtr,
    n_touched_points: ShapeId,
    points: *const ShapeId,
    delta_x: *const Pos,
    delta_y: *const Pos,
    r: *const VqRegion,
) {
    // A local `Vec<*mut Point>` now, not a `__caryll_allocate_clean`'d/
    // `free`'d array -- built with exactly `total_points` entries by
    // construction (the two fill loops below run exactly that many times
    // between them, matching what the array used to be pre-sized to),
    // dropped automatically at the end of this function.
    let mut glyph_refs: Vec<*mut Point> = Vec::with_capacity(total_points as usize);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*glyph).contours.len() {
        let c: *mut Contour = &raw mut (&mut (*glyph).contours)[__caryll_index];
        while keep != 0 {
            let mut __caryll_index_0: usize = 0 as usize;
            let mut keep_0: usize = 1 as usize;
            while keep_0 != 0 && __caryll_index_0 < (*c).len() {
                let g: *mut Point = &raw mut (&mut (*c))[__caryll_index_0];
                while keep_0 != 0 {
                    glyph_refs.push(g);
                    keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                }
                keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                __caryll_index_0 = __caryll_index_0.wrapping_add(1);
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut __caryll_index_1: usize = 0 as usize;
    let mut keep_1: usize = 1 as usize;
    while keep_1 != 0 && __caryll_index_1 < (*glyph).references.len() {
        let r_0: *mut ComponentReference = &raw mut (&mut (*glyph).references)[__caryll_index_1];
        while keep_1 != 0 {
            glyph_refs.push(&raw mut (*r_0).x as *mut Point);
            keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
        }
        keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
        __caryll_index_1 = __caryll_index_1.wrapping_add(1);
    }
    apply_coords(
        total_points,
        glyph as *mut Glyph,
        &glyph_refs,
        n_touched_points,
        delta_x,
        points,
        r,
        Some(get_x as unsafe extern "C" fn(*mut Point) -> *mut VQ),
    );
    apply_coords(
        total_points,
        glyph as *mut Glyph,
        &glyph_refs,
        n_touched_points,
        delta_y,
        points,
        r,
        Some(get_y as unsafe extern "C" fn(*mut Point) -> *mut VQ),
    );
    if (total_points as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        < n_touched_points as ::core::ffi::c_int
    {
        vq_add_delta(
            &raw mut (*glyph).horizontal_origin,
            true,
            r,
            *delta_x.offset(total_points as isize),
        );
        vq_add_delta(
            &raw mut (*glyph).advance_width,
            true,
            r,
            *delta_x
                .offset((total_points as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                - *delta_x.offset(total_points as isize),
        );
    }
    if (total_points as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
        < n_touched_points as ::core::ffi::c_int
    {
        vq_add_delta(
            &raw mut (*glyph).vertical_origin,
            true,
            r,
            *delta_y
                .offset((total_points as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize),
        );
        vq_add_delta(
            &raw mut (*glyph).advance_height,
            true,
            r,
            *delta_y
                .offset((total_points as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                - *delta_y.offset(
                    (total_points as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ),
        );
    }
}
// `peak_offset`/`range_offset` are absolute byte offsets into `gvar`
// instead of `*mut F2Dot14` -- the original read these with no bounds
// checking at all (a peak or intermediate-region array embedded in a
// `TupleVariationHeader`, itself found by nothing but the wire format's
// own self-description, per `next_tvh_offset`'s comment). On any read
// running past `gvar`'s length, the region already allocated by
// `vq_create_region` above is freed before returning `None` -- unlike the
// original, which never had a failure path to unwind at all.
unsafe fn create_region_from_tuples(
    gvar: &[u8],
    dimensions: u16,
    peak_offset: usize,
    range_offset: Option<usize>,
) -> Option<*mut VqRegion> {
    let r: *mut VqRegion = vq_create_region(dimensions as ShapeId);
    let mut d: u16 = 0 as u16;
    while (d as ::core::ffi::c_int) < dimensions as ::core::ffi::c_int {
        let Ok(peak_raw) = FontReader::new(gvar)
            .at(peak_offset + d as usize * 2)
            .and_then(|mut x| x.i16())
        else {
            vq_delete_region(r);
            return None;
        };
        let peak_val: Pos = otfcc_from_f2dot14(peak_raw as F2Dot14) as Pos;
        let mut span: VqAxisSpan = VqAxisSpan {
            start: (if peak_val <= 0 as ::core::ffi::c_int as Pos {
                -(1 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            }) as Pos,
            peak: peak_val,
            end: (if peak_val >= 0 as ::core::ffi::c_int as Pos {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
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
                    span.start = otfcc_from_f2dot14(sv as F2Dot14) as Pos;
                    span.end = otfcc_from_f2dot14(ev as F2Dot14) as Pos;
                }
                _ => {
                    vq_delete_region(r);
                    return None;
                }
            }
        }
        (*r).spans.push(span);
        d = d.wrapping_add(1);
    }
    Some(r)
}
// `gvd_offset` is an absolute byte offset into `gvar` instead of a `*mut
// GlyphVariationData` -- every read below goes through `FontReader`,
// checked against `gvar`'s real length, instead of walking off a pointer
// with no length of its own at all (this glyph's entire tuple-variation
// record, and each tuple header inside it, used to be found purely by
// self-description: `next_tvh`'s "bump" logic and `tsd_start`'s
// accumulation of each tuple's own declared `variationDataSize`). `None`
// on any read failing stops processing this glyph's remaining tuples
// (whatever deltas were already applied by earlier tuples in the loop
// stay applied) rather than reading adjacent bytes -- the caller doesn't
// need to do anything with the result, matching the original always
// "succeeding" (it never checked anything to fail on).
#[inline]
unsafe fn polymorphize_glyph(
    glyph: GlyphPtr,
    ctx: &TuplePolymorphizerCtx,
    gvar: &[u8],
    gvd_offset: usize,
) -> Option<()> {
    let mut total_points: ShapeId = 0 as ShapeId;
    for c in &(*glyph).contours {
        total_points = (total_points as usize).wrapping_add(c.len()) as ShapeId;
    }
    total_points = (total_points as usize).wrapping_add((*glyph).references.len()) as ShapeId;
    let total_delta_entries: ShapeId = (total_points as ::core::ffi::c_int
        + ctx.n_phantom_points as ::core::ffi::c_int)
        as ShapeId;

    let mut header = FontReader::new(gvar).at(gvd_offset).ok()?;
    let raw_tuple_variation_count = header.u16().ok()?;
    let data_offset_field = header.u16().ok()?;
    let n_tuples: u16 = raw_tuple_variation_count & 0xfff as u16;
    let has_shared_point_numbers: bool = raw_tuple_variation_count & 0x8000 as u16 != 0;
    let mut tvh_offset: usize = gvd_offset + 4;

    // `shared_point_indeces` is a local `Vec<ShapeId>` now: empty means
    // "not present" (the old null pointer), matching every check below
    // that used to be `.is_null()`. `parse_point_numbers` no longer
    // writes through a separate count out-param either -- `.len()` is
    // always in sync with the data by construction.
    let mut shared_point_indeces: Vec<ShapeId> = Vec::new();
    let mut data_offset: usize = gvd_offset + data_offset_field as usize;
    if has_shared_point_numbers {
        let (new_offset, indeces) = parse_point_numbers(gvar, data_offset, total_delta_entries)?;
        data_offset = new_offset;
        shared_point_indeces = indeces;
    }
    let mut tsd_start: usize = 0 as usize;
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
        let r: *const VqRegion = fvar_register_region(ctx.fvar, region);

        let tsd = data_offset + tsd_start;
        // `point_indeces` borrows `shared_point_indeces` by default (freed
        // once, after this whole loop, same as before) and only owns a
        // private `Vec` -- built fresh by `parse_point_numbers` -- when
        // `PRIVATE_POINT_NUMBERS` is set for this tuple, matching the
        // original's "usually an alias, occasionally a fresh allocation
        // freed within this same iteration" shape exactly, but as a
        // `Cow` instead of a raw pointer that is only sometimes owned.
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
            // Local `Vec<Pos>`s, not `__caryll_allocate_clean`'d/`free`'d
            // buffers -- dropped automatically at the end of this `if`.
            let mut delta_x: Vec<Pos> = vec![0 as Pos; n_points as usize];
            let mut delta_y: Vec<Pos> = vec![0 as Pos; n_points as usize];
            let after_x = read_packed_delta(gvar, after_points, n_points, &mut delta_x)?;
            read_packed_delta(gvar, after_x, n_points, &mut delta_y)?;
            apply_polymorphism(
                total_points,
                glyph,
                n_points,
                point_indeces.as_ptr(),
                delta_x.as_ptr(),
                delta_y.as_ptr(),
                r,
            );
        }
        tsd_start = tsd_start.wrapping_add(variation_data_size as usize);
        tvh_offset = next_tvh_offset(gvar, tvh_offset, ctx.dimensions)?;
    }
    Some(())
}
// The `GVARHeader`/`glyphVariationDataOffsets` array reads below used to
// run straight off `data.offset(...)` with no bounds checking at all --
// the per-glyph offset array in particular (`glyphVariationDataOffsets[j]`
// for every `j` up to `num_glyphs`) had no guard whatsoever against a
// `gvar` table too short to actually hold that many entries, and the
// `glyphVariationDataArrayOffset + glyphVariationDataOffset` sum that
// follows it was never checked against the table's own length either --
// both real, previously-undocumented "zero guard" bugs, the same class
// `read_contextual_format2`'s `ChainSubClassSet` array had
// (`otl/subtables/chaining/read.rs`). `__fortable_*` (goto emulation) ->
// the same `.iter().find()` idiom every other migrated table reader uses.
#[inline]
unsafe fn polymorphize(
    packet: &Packet,
    options: &Options,
    glyf: *mut GlyfTable,
    ctx: *const GlyfIOContext,
) {
    if (*ctx).fvar.is_null() || (*(*ctx).fvar).axes.is_empty() {
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
    } // majorVersion/minorVersion: never read by the original either
    let Ok(axis_count) = header.u16() else { return };
    if axis_count as usize != (*(*ctx).fvar).axes.len() {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"Axes number in GVAR and FVAR are inequal"),
        );
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

    for j in 0..(*glyf).len() {
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

        let tpctx = TuplePolymorphizerCtx {
            fvar: (*ctx).fvar,
            dimensions,
            shared_tuple_count,
            shared_tuples_offset: shared_tuples_offset as usize,
            coord_dimensions: 2 as u8,
            allow_iup: !(&(*glyf))[j].as_deref().unwrap().contours.is_empty(),
            n_phantom_points: (*ctx).n_phantom_points,
        };
        polymorphize_glyph(
            &raw mut **(&mut (*glyf))[j].as_mut().unwrap(),
            &tpctx,
            gvar,
            gvd_offset,
        );
    }
}
pub unsafe fn otfcc_read_glyf(
    packet: &Packet,
    options: &Options,
    ctx: *const GlyfIOContext,
) -> Option<GlyfTable> {
    let num_glyphs = (*ctx).num_glyphs;
    // A local `Vec<u32>` now, not a `__caryll_allocate_clean`'d/`free`'d
    // buffer -- `Vec`'s own allocator aborts rather than returning null on
    // failure, so the `!offsets.is_null()` guard this used to need at
    // every entry/exit point is gone; the `Vec` drops itself wherever
    // this function returns.
    let mut offsets: Vec<u32> = vec![0u32; num_glyphs as usize + 1];

    // `__fortable_*`/`current_block` (goto emulation) -> the same
    // `.iter().find()` idiom every other already-migrated table reader in
    // this crate uses.
    let loca_corrupted = || {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"table 'loca' corrupted.\n"),
        );
    };
    let Some(loca) = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_LOCA) else {
        loca_corrupted();
        return None;
    };
    // The original's own guard here (`length < 2*num_glyphs+2`) used the
    // *short*-format byte count unconditionally, even when `loca_is_long`
    // -- a long-format `loca` table needs twice that (4 bytes/entry, not
    // 2), so this let a table too short for the format it actually claims
    // pass the guard and read past its own end in the `read_32u` calls
    // below. `FontReader` needs no separate upfront guard at all: each
    // `u16()`/`u32()` read below is checked against the real remaining
    // length, for whichever format this table actually is.
    let mut loca_r = FontReader::new(&loca.data);
    let mut found_loca = true;
    for j in 0..=(num_glyphs as u32) {
        let v = if (*ctx).loca_is_long {
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
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"table 'glyf' corrupted.\n"),
        );
        return None;
    }
    let data_0: FontFilePointer = glyf_piece.data.as_ptr() as FontFilePointer;
    let mut glyf_val: GlyfTable = Vec::with_capacity(num_glyphs as usize);
    for j0 in 0..num_glyphs {
        if offsets[j0 as usize] < offsets[j0 as usize + 1] {
            let glyph_length = offsets[j0 as usize + 1] - offsets[j0 as usize];
            // A malformed individual glyph (an unbounded component chain,
            // a flag/coordinate stream that runs past its own declared
            // byte range, ...) now fails cleanly inside
            // `otfcc_read_glyph` instead of reading adjacent bytes --
            // fall back to an empty glyph for this one GID rather than
            // failing the whole table, the same degradation the
            // zero-length-range case below already used.
            let g = otfcc_read_glyph(data_0, offsets[j0 as usize], glyph_length, options)
                .unwrap_or_else(|| otfcc_new_glyf_glyph());
            glyf_val.push(Some(g));
        } else {
            glyf_val.push(Some(otfcc_new_glyf_glyph()));
        }
    }
    let mut glyf = Some(glyf_val);
    polymorphize(
        packet,
        options,
        glyf.as_mut()
            .map_or(::core::ptr::null_mut(), |g| g as *mut GlyfTable),
        ctx,
    );
    glyf
}

#[cfg(test)]
mod glyf_read_tests {
    use super::*;
    use crate::vf::vq::vq_get_still;

    fn zeroed_options() -> Options {
        Options::default()
    }

    unsafe fn still(v: &VQ) -> Pos {
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
        let options = zeroed_options();
        unsafe {
            let g = otfcc_read_glyph(
                data.as_ptr() as FontFilePointer,
                0,
                data.len() as u32,
                &options,
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
        let options = zeroed_options();
        unsafe {
            let g = otfcc_read_glyph(
                data.as_ptr() as FontFilePointer,
                0,
                data.len() as u32,
                &options,
            );
            let g = g.unwrap();
            assert_eq!(g.references.len(), 1);
            assert_eq!(g.references[0].glyph.index, 5);
            assert_eq!(still(&g.references[0].x), 10.0);
            assert_eq!(still(&g.references[0].y), 20.0);
            assert_eq!(g.references[0].is_anchored, RefAnchorStatus::Xy);
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
        let options = zeroed_options();
        unsafe {
            let g = otfcc_read_glyph(
                data.as_ptr() as FontFilePointer,
                0,
                data.len() as u32,
                &options,
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
        let options = zeroed_options();
        unsafe {
            let g = otfcc_read_glyph(
                data.as_ptr() as FontFilePointer,
                0,
                data.len() as u32,
                &options,
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
        // The original's only loop terminator was the MORE_COMPONENTS
        // bit -- a component chain that always sets it and then runs out
        // of data used to read straight past the glyph's own bytes with
        // no bound at all. One full component record with
        // MORE_COMPONENTS set, then nothing: must terminate (not hang)
        // and reject.
        let mut data = [0u8; 18];
        data[0..2].copy_from_slice(&(-1i16).to_be_bytes());
        data[10..12].copy_from_slice(&35u16.to_be_bytes()); // flags = 2|1|32 (MORE_COMPONENTS)
        data[12..14].copy_from_slice(&5u16.to_be_bytes());
        data[14..16].copy_from_slice(&10i16.to_be_bytes());
        data[16..18].copy_from_slice(&20i16.to_be_bytes());
        let options = zeroed_options();
        unsafe {
            let g = otfcc_read_glyph(
                data.as_ptr() as FontFilePointer,
                0,
                data.len() as u32,
                &options,
            );
            assert!(g.is_none());
        }
    }

    #[test]
    fn non_monotonic_end_points_of_contours_is_rejected_not_a_huge_allocation() {
        // contour 0 ends at point 5 (6 points); contour 1 ends at point 2
        // -- fewer than contour 0 already claimed. The original computed
        // this contour's point count in signed arithmetic then cast
        // straight to `usize`, so a negative result became a number near
        // `usize::MAX` and `glyf_contour_fill` tried to allocate that
        // many points. Must reject instead.
        let mut data = [0u8; 14];
        data[0..2].copy_from_slice(&2i16.to_be_bytes());
        data[10..12].copy_from_slice(&5u16.to_be_bytes());
        data[12..14].copy_from_slice(&2u16.to_be_bytes());
        let options = zeroed_options();
        unsafe {
            let g = otfcc_read_glyph(
                data.as_ptr() as FontFilePointer,
                0,
                data.len() as u32,
                &options,
            );
            assert!(g.is_none());
        }
    }

    #[test]
    fn repeat_run_overrunning_the_declared_point_count_is_rejected_not_a_panic() {
        // endPtsOfContours[0]=1 declares exactly 2 points. The first flag
        // sets REPEAT with a run of 5 -- 1 + 5 = 6 total flags, four more
        // than declared. The original indexed a now-`Vec`-backed,
        // fixed-size `flags` array with no check that a repeat run stays
        // within the declared point count, which would panic in Rust
        // (a silent overflow write in the original C). Must reject
        // instead of either.
        let mut data = [0u8; 16];
        data[0..2].copy_from_slice(&1i16.to_be_bytes());
        data[10..12].copy_from_slice(&1u16.to_be_bytes());
        data[12..14].copy_from_slice(&0u16.to_be_bytes());
        data[14] = 0x09; // REPEAT | ON_CURVE
        data[15] = 5; // repeat count
        let options = zeroed_options();
        unsafe {
            let g = otfcc_read_glyph(
                data.as_ptr() as FontFilePointer,
                0,
                data.len() as u32,
                &options,
            );
            assert!(g.is_none());
        }
    }

    #[test]
    fn header_shorter_than_ten_bytes_is_rejected_instead_of_reading_oob() {
        let data = [0u8; 5];
        let options = zeroed_options();
        unsafe {
            let g = otfcc_read_glyph(
                data.as_ptr() as FontFilePointer,
                0,
                data.len() as u32,
                &options,
            );
            assert!(g.is_none());
        }
    }
}

#[cfg(test)]
mod gvar_polymorphize_tests {
    use super::*;

    #[test]
    fn next_tvh_offset_truncated_header_is_rejected_instead_of_reading_oob() {
        // The original walked this array with nothing but pointer
        // arithmetic and no length at all.
        let gvar: [u8; 0] = [];
        unsafe {
            assert!(next_tvh_offset(&gvar, 0, 1).is_none());
        }
    }

    #[test]
    fn next_tvh_offset_bumps_past_an_embedded_peak_tuple() {
        // variationDataSize=0, tupleIndex=EMBEDDED_PEAK_TUPLE (0x8000) ->
        // bump = 4 (header) + 1 dimension * 2 bytes = 6.
        let mut data = [0u8; 4];
        data[2..4].copy_from_slice(&0x8000u16.to_be_bytes());
        unsafe {
            assert_eq!(next_tvh_offset(&data, 0, 1), Some(6));
        }
    }

    #[test]
    fn create_region_from_tuples_truncated_peak_is_rejected_not_leaked() {
        // The original had no bounds checking here at all -- reading a
        // peak/start/end F2Dot14 ran straight off whatever `gvar` bytes
        // happened to exist. This also exercises the new cleanup path:
        // `vq_create_region`'s allocation must be freed on this failure,
        // not leaked (verified by `cargo miri test`, which would flag an
        // unreachable allocation).
        let gvar: [u8; 0] = [];
        unsafe {
            assert!(create_region_from_tuples(&gvar, 1, 0, None).is_none());
        }
    }

    #[test]
    fn create_region_from_tuples_reads_a_single_dimension_peak() {
        let gvar = [0x40u8, 0x00u8]; // F2Dot14 0x4000 = 1.0
        unsafe {
            let region = create_region_from_tuples(&gvar, 1, 0, None).unwrap();
            assert_eq!((*region).dimensions, 1);
            vq_delete_region(region);
        }
    }

    #[test]
    fn parse_point_numbers_truncated_run_is_rejected_instead_of_reading_oob() {
        // n_points=2 (first byte, not POINT_COUNT_IS_WORD), then the
        // buffer ends before the run header that should follow -- the
        // original had no length parameter to check against at all.
        let data = [0x02u8];
        unsafe {
            assert!(parse_point_numbers(&data, 0, 5).is_none());
        }
    }

    #[test]
    fn parse_point_numbers_zero_count_returns_every_point_in_order() {
        let data = [0x00u8]; // n_points=0 -> "every point", 0..total_points
        unsafe {
            let (new_offset, indeces) = parse_point_numbers(&data, 0, 3).unwrap();
            assert_eq!(new_offset, 1);
            assert_eq!(indeces, vec![0, 1, 2]);
        }
    }

    #[test]
    fn read_packed_delta_truncated_run_is_rejected_instead_of_reading_oob() {
        let data: [u8; 0] = [];
        let mut deltas = [0.0; 1];
        unsafe {
            assert!(read_packed_delta(&data, 0, 1, &mut deltas).is_none());
        }
    }

    #[test]
    fn read_packed_delta_reads_a_single_narrow_delta() {
        // run header 0x00: not zero, not wide, run length = 0+1 = 1; one
        // signed byte delta of 5.
        let data = [0x00u8, 5u8];
        let mut deltas = [0.0; 1];
        unsafe {
            let new_offset = read_packed_delta(&data, 0, 1, &mut deltas).unwrap();
            assert_eq!(new_offset, 2);
            assert_eq!(deltas[0], 5.0);
        }
    }
}
