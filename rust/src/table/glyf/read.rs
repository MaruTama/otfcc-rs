#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{memcpy};

use crate::support::handle::{handle_from_index, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::font_reader::{FontReader};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, logger_log_sds};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, F2Dot14, FontFilePointer, GlyphId, Pos, Scale, ShapeId};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::fvar::FvarTable;
use crate::table::glyf::{GlyfIOContext, RefAnchorStatus, ComponentFlags, PointFlags, ComponentReference, Contour, ContourList, Glyph, GlyphPtr, Point, GlyfTable};


use crate::vf::region::{VqAxisSpan, VqRegion};
use crate::vf::vq::{VQ, VqSegment, VqSegmentDelta};
use crate::support::primitives::{otfcc_f1616_muldiv, otfcc_from_f2dot14, otfcc_from_fixed, otfcc_to_fixed};
use crate::table::fvar::{fvar_register_region};
use crate::table::glyf::{glyf_component_reference_empty, glyf_contour_fill, otfcc_new_glyf_glyph};
use crate::vf::region::{vq_create_region};
use crate::vf::vq::{
    vq_add_delta, vq_copy_replace, vq_create_still, vq_inplace_plus, vq_neutral, vq_replace,
};

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct GlyphVariationData {
    pub tuple_variation_count: u16,
    pub data_offset: u16,
    pub tvhs: [TupleVariationHeader; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct TupleVariationHeader {
    pub variation_data_size: u16,
    pub tuple_index: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct GVARHeader {
    pub major_version: u16,
    pub minor_version: u16,
    pub axis_count: u16,
    pub shared_tuple_count: u16,
    pub shared_tuples_offset: u32,
    pub glyph_count: u16,
    pub flags: u16,
    pub glyph_variation_data_array_offset: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TuplePolymorphizerCtx {
    pub fvar: *mut FvarTable,
    pub dimensions: u16,
    pub shared_tuple_count: u16,
    pub shared_tuples: *mut F2Dot14,
    pub coord_dimensions: u8,
    pub allow_iup: bool,
    pub n_phantom_points: ShapeId,
}
pub type CoordPartGetter = Option<unsafe extern "C" fn(*mut Point) -> *mut VQ>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PackedDeltaRun {
    pub length: ShapeId,
    pub wide: bool,
    pub zero: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PackedPointRun {
    pub length: ShapeId,
    pub wide: bool,
}
unsafe extern "C" fn next_point(
    mut contours: *mut ContourList,
    mut cc: *mut ShapeId,
    mut cp: *mut ShapeId,
) -> *mut Point {
    if *cp as usize >= (&(*contours))[*cc as usize].len() {
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
unsafe fn otfcc_read_simple_glyph(
    body: &[u8],
    number_of_contours: ShapeId,
    _options: *const Options,
) -> Option<Box<Glyph>> {
    let mut g: Box<Glyph> = otfcc_new_glyf_glyph();
    let contours: *mut ContourList = &raw mut (*g).contours;
    let mut r = FontReader::new(body);
    r.require_room(number_of_contours as usize, 2).ok()?;
    let mut points_in_glyph: ShapeId = 0;
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
        let n = last_point_in_current_contour as i32 - points_in_glyph as i32 + 1;
        if n < 0 {
            return None;
        }
        let mut contour: Contour = Vec::new();
        glyf_contour_fill(&raw mut contour, n as usize);
        (*contours).push(contour);
        points_in_glyph = last_point_in_current_contour.wrapping_add(1);
    }
    let instruction_length: u16 = r.u16().ok()?;
    let instruction_bytes = r.bytes(instruction_length as usize).ok()?;
    let mut instructions: *mut u8 = ::core::ptr::null_mut::<u8>();
    if instruction_length > 0 {
        instructions = __caryll_allocate_clean(
            (::core::mem::size_of::<u8>() as usize).wrapping_mul(instruction_length as usize),
            31 as ::core::ffi::c_ulong,
        ) as *mut u8;
        memcpy(
            instructions as *mut ::core::ffi::c_void,
            instruction_bytes.as_ptr() as *const ::core::ffi::c_void,
            (::core::mem::size_of::<u8>() as usize).wrapping_mul(instruction_length as usize),
        );
    }
    (*g).instructions_length = instruction_length;
    (*g).instructions = instructions;
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
            if flag_0.contains(PointFlags::POSITIVE_X) { mag } else { -mag }
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
            if flag_1.contains(PointFlags::POSITIVE_Y) { mag } else { -mag }
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
    let mut cx: VQ =
        (vq_neutral)();
    let mut cy: VQ =
        (vq_neutral)();
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
unsafe fn otfcc_read_composite_glyph(
    body: &[u8],
    options: *const Options,
) -> Option<Box<Glyph>> {
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
                (*options).logger,
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
        let mut instructions: FontFilePointer = ::core::ptr::null_mut::<u8>();
        if instruction_length > 0 {
            instructions = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize)
                    .wrapping_mul(instruction_length as usize),
                201 as ::core::ffi::c_ulong,
            ) as FontFilePointer;
            memcpy(
                instructions as *mut ::core::ffi::c_void,
                instruction_bytes.as_ptr() as *const ::core::ffi::c_void,
                (::core::mem::size_of::<u8>() as usize)
                    .wrapping_mul(instruction_length as usize),
            );
        }
        (*g).instructions_length = instruction_length;
        (*g).instructions = instructions as *mut u8;
    } else {
        (*g).instructions_length = 0 as u16;
        (*g).instructions = ::core::ptr::null_mut::<u8>();
    }
    Some(g)
}
unsafe fn otfcc_read_glyph(
    data: FontFilePointer,
    offset: u32,
    length: u32,
    options: *const Options,
) -> Option<Box<Glyph>> {
    let glyph_bytes =
        ::core::slice::from_raw_parts(data.offset(offset as isize), length as usize);
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
        otfcc_read_simple_glyph(body, number_of_contours as ShapeId, options)?
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
#[inline]
unsafe fn next_tvh(
    mut current_header: *mut TupleVariationHeader,
    mut ctx: *const TuplePolymorphizerCtx,
) -> *mut TupleVariationHeader {
    let mut bump: u32 =
        2_usize.wrapping_mul(::core::mem::size_of::<u16>()) as u32;
    let mut tuple_index: u16 = be16((*current_header).tuple_index);
    if tuple_index as ::core::ffi::c_int & EMBEDDED_PEAK_TUPLE != 0 {
        bump = (bump as ::core::ffi::c_ulong).wrapping_add(
            ((*ctx).dimensions as usize).wrapping_mul(::core::mem::size_of::<F2Dot14>())
                as ::core::ffi::c_ulong,
        ) as u32 as u32;
    }
    if tuple_index as ::core::ffi::c_int & INTERMEDIATE_REGION != 0 {
        bump = (bump as ::core::ffi::c_ulong).wrapping_add(
            ((2 as ::core::ffi::c_int * (*ctx).dimensions as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<F2Dot14>())
                as ::core::ffi::c_ulong,
        ) as u32 as u32;
    }
    return (current_header as FontFilePointer).offset(bump as isize) as *mut TupleVariationHeader;
}
pub const POINT_COUNT_IS_WORD: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const POINT_COUNT_LONG_MASK: ::core::ffi::c_int = 0x7fff as ::core::ffi::c_int;
pub const POINT_RUN_COUNT_MASK: ::core::ffi::c_int = 0x7f as ::core::ffi::c_int;
pub const POINTS_ARE_WORDS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
/// Returns `(advanced data pointer, point indices)` instead of writing
/// through two out-params -- `pc` (the count) is just `point_indeces.
/// len()` once the array is a `Vec`, so it disappears entirely rather
/// than needing to stay in sync with a separately-tracked length.
///
/// Never a real FFI boundary -- internal call site only, same rationale
/// as every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
#[inline]
unsafe fn parse_point_numbers(
    mut data: FontFilePointer,
    mut total_points: ShapeId,
) -> (FontFilePointer, Vec<ShapeId>) {
    let mut n_points: u16 = 0 as u16;
    let mut first_byte: u8 = *data;
    if first_byte as ::core::ffi::c_int & POINT_COUNT_IS_WORD != 0 {
        n_points = (((*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            & POINT_COUNT_LONG_MASK) as u16;
        data = data.offset(2 as ::core::ffi::c_int as isize);
    } else {
        n_points = first_byte as u16;
        data = data.offset(1);
    }
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
                let fresh6 = data;
                data = data.offset(1);
                let mut run_header: u8 = *fresh6;
                run.wide = run_header as ::core::ffi::c_int & POINTS_ARE_WORDS != 0;
                run.length = ((run_header as ::core::ffi::c_int & POINT_RUN_COUNT_MASK)
                    + 1 as ::core::ffi::c_int) as ShapeId;
            }
            let mut point_number: i16 = j_point as i16;
            if run.wide {
                point_number = (point_number as ::core::ffi::c_int
                    + *(data as *mut u16) as ::core::ffi::c_int)
                    as i16;
                data = data.offset(2 as ::core::ffi::c_int as isize);
            } else {
                let fresh7 = data;
                data = data.offset(1);
                point_number =
                    (point_number as ::core::ffi::c_int + *fresh7 as ::core::ffi::c_int) as i16;
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
    return (data, point_indeces);
}
pub const DELTAS_ARE_ZERO: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const DELTAS_ARE_WORDS: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const DELTA_RUN_COUNT_MASK: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;
#[inline]
unsafe fn read_packed_delta(
    mut data: FontFilePointer,
    mut n_points: ShapeId,
    mut deltas: *mut Pos,
) -> FontFilePointer {
    let mut run: PackedDeltaRun = PackedDeltaRun {
        length: 0 as ShapeId,
        wide: false,
        zero: false,
    };
    let mut filled: ShapeId = 0 as ShapeId;
    while (filled as ::core::ffi::c_int) < n_points as ::core::ffi::c_int {
        let mut delta: i16 = 0 as i16;
        if run.length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let fresh5 = data;
            data = data.offset(1);
            let mut run_header: u8 = *fresh5;
            run.zero = run_header as ::core::ffi::c_int & DELTAS_ARE_ZERO != 0;
            run.wide = run_header as ::core::ffi::c_int & DELTAS_ARE_WORDS != 0;
            run.length = ((run_header as ::core::ffi::c_int & DELTA_RUN_COUNT_MASK)
                + 1 as ::core::ffi::c_int) as ShapeId;
        }
        if !run.zero {
            if run.wide {
                delta = be16(*(data as *mut u16)) as i16;
                data = data.offset(2 as ::core::ffi::c_int as isize);
            } else {
                delta = *data as i8 as i16;
                data = data.offset(1);
            }
        }
        *deltas.offset(filled as isize) = delta as Pos;
        filled = filled.wrapping_add(1);
        run.length = run.length.wrapping_sub(1);
    }
    return data;
}
pub unsafe extern "C" fn get_x(mut z: *mut Point) -> *mut VQ {
    return &raw mut (*z).x;
}
pub unsafe extern "C" fn get_y(mut z: *mut Point) -> *mut VQ {
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
    mut j_min: ShapeId,
    mut j_max: ShapeId,
    nudges: &mut [VqSegment],
    glyph_refs: &[*mut Point],
    mut getter: CoordPartGetter,
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
                let mut untouch_j: F16Dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(glyph_refs[j as usize]))
                        .kernel as ::core::ffi::c_double,
                );
                let mut untouch_prev: F16Dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(glyph_refs[j_prev as usize]))
                        .kernel as ::core::ffi::c_double,
                );
                let mut untouch_next: F16Dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(glyph_refs[j_next as usize]))
                        .kernel as ::core::ffi::c_double,
                );
                let mut delta_prev: F16Dot16 = otfcc_to_fixed(
                    nudges[j_prev as usize].unwrap_delta().quantity as ::core::ffi::c_double,
                );
                let mut delta_next: F16Dot16 = otfcc_to_fixed(
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
                    nudges[j as usize].delta_mut().quantity = otfcc_from_fixed(
                        otfcc_f1616_muldiv(d_max - d_min, untouch_j - u_min, u_max - u_min),
                    )
                        as Pos;
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
    mut glyph: *mut Glyph,
    glyph_refs: &[*mut Point],
    n_touched_points: ShapeId,
    mut tuple_delta: *const Pos,
    mut points: *const ShapeId,
    mut r: *const VqRegion,
    mut getter: CoordPartGetter,
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
            let mut coordinate_part: *mut VQ =
                getter.expect("non-null function pointer")(glyph_refs[j_1 as usize]);
            (*coordinate_part).shift.push(nudges[j_1 as usize]);
        }
        j_1 = j_1.wrapping_add(1);
    }
}
#[inline]
unsafe fn apply_polymorphism(
    total_points: ShapeId,
    mut glyph: GlyphPtr,
    n_touched_points: ShapeId,
    mut points: *const ShapeId,
    mut delta_x: *const Pos,
    mut delta_y: *const Pos,
    mut r: *const VqRegion,
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
            *delta_x.offset((total_points as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
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
            *delta_y.offset((total_points as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize),
        );
        vq_add_delta(
            &raw mut (*glyph).advance_height,
            true,
            r,
            *delta_y.offset((total_points as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                - *delta_y
                    .offset((total_points as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize),
        );
    }
}
unsafe fn create_region_from_tuples(
    mut dimensions: u16,
    mut peak: *mut F2Dot14,
    mut start: *mut F2Dot14,
    mut end: *mut F2Dot14,
) -> *mut VqRegion {
    let mut r: *mut VqRegion = vq_create_region(dimensions as ShapeId);
    let mut d: u16 = 0 as u16;
    while (d as ::core::ffi::c_int) < dimensions as ::core::ffi::c_int {
        let mut peak_val: Pos =
            otfcc_from_f2dot14(be16(*peak.offset(d as isize) as u16) as F2Dot14) as Pos;
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
        if !start.is_null() && !end.is_null() {
            span.start =
                otfcc_from_f2dot14(be16(*start.offset(d as isize) as u16) as F2Dot14) as Pos;
            span.end =
                otfcc_from_f2dot14(be16(*end.offset(d as isize) as u16) as F2Dot14) as Pos;
        }
        *(&raw mut (*r).spans as *mut VqAxisSpan).offset(d as isize) = span;
        d = d.wrapping_add(1);
    }
    return r;
}
#[inline]
unsafe fn polymorphize_glyph(
    mut _gid: GlyphId,
    mut glyph: GlyphPtr,
    mut ctx: *const TuplePolymorphizerCtx,
    mut gvd: *mut GlyphVariationData,
    mut _options: *const Options,
) {
    let mut total_points: ShapeId = 0 as ShapeId;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*glyph).contours.len() {
        let c: *mut Contour = &raw mut (&mut (*glyph).contours)[__caryll_index];
        while keep != 0 {
            total_points =
                (total_points as usize).wrapping_add((*c).len()) as ShapeId as ShapeId;
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    total_points =
        (total_points as usize).wrapping_add((*glyph).references.len()) as ShapeId as ShapeId;
    let mut total_delta_entries: ShapeId = (total_points as ::core::ffi::c_int
        + (*ctx).n_phantom_points as ::core::ffi::c_int)
        as ShapeId;
    let mut n_tuples: u16 = (be16((*gvd).tuple_variation_count) as ::core::ffi::c_int
        & 0xfff as ::core::ffi::c_int) as u16;
    let mut tvh: *mut TupleVariationHeader = &raw mut (*gvd).tvhs as *mut TupleVariationHeader;
    let mut has_shared_point_numbers: bool =
        be16((*gvd).tuple_variation_count) as ::core::ffi::c_int & 0x8000 as ::core::ffi::c_int != 0;
    // `shared_point_indeces` is a local `Vec<ShapeId>` now: empty means
    // "not present" (the old null pointer), matching every check below
    // that used to be `.is_null()`. `parse_point_numbers` no longer
    // writes through a separate count out-param either -- `.len()` is
    // always in sync with the data by construction.
    let mut shared_point_indeces: Vec<ShapeId> = Vec::new();
    let mut data: FontFilePointer =
        (gvd as FontFilePointer).offset(be16((*gvd).data_offset) as ::core::ffi::c_int as isize);
    if has_shared_point_numbers {
        let (new_data, indeces) = parse_point_numbers(data, total_delta_entries);
        data = new_data;
        shared_point_indeces = indeces;
    }
    let mut tsd_start: usize = 0 as usize;
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < n_tuples as ::core::ffi::c_int {
        let mut tuple_index: ShapeId =
            (be16((*tvh).tuple_index) as ::core::ffi::c_int & TUPLE_INDEX_MASK) as ShapeId;
        let mut has_embedded_peak: bool =
            be16((*tvh).tuple_index) as ::core::ffi::c_int & EMBEDDED_PEAK_TUPLE != 0;
        let mut has_intermediate: bool =
            be16((*tvh).tuple_index) as ::core::ffi::c_int & INTERMEDIATE_REGION != 0;
        let mut peak: *mut F2Dot14 = ::core::ptr::null_mut::<F2Dot14>();
        if has_embedded_peak {
            peak =
                (tvh as FontFilePointer).offset(4 as ::core::ffi::c_int as isize) as *mut F2Dot14;
        } else {
            peak = (*ctx).shared_tuples.offset(
                ((*ctx).dimensions as ::core::ffi::c_int * tuple_index as ::core::ffi::c_int)
                    as isize,
            );
        }
        let mut start: *mut F2Dot14 = ::core::ptr::null_mut::<F2Dot14>();
        let mut end: *mut F2Dot14 = ::core::ptr::null_mut::<F2Dot14>();
        if has_intermediate {
            start = (tvh as FontFilePointer)
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(
                    (2 as ::core::ffi::c_int
                        * (if has_embedded_peak as ::core::ffi::c_int != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                        * (*ctx).dimensions as ::core::ffi::c_int) as isize,
                ) as *mut F2Dot14;
            end = (tvh as FontFilePointer)
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(
                    (2 as ::core::ffi::c_int
                        * (if has_embedded_peak as ::core::ffi::c_int != 0 {
                            2 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        })
                        * (*ctx).dimensions as ::core::ffi::c_int) as isize,
                ) as *mut F2Dot14;
        }
        let mut r: *const VqRegion = fvar_register_region(
            (*ctx).fvar,
            create_region_from_tuples((*ctx).dimensions, peak, start, end),
        );
        let mut tsd: FontFilePointer = data.offset(tsd_start as isize);
        // `point_indeces` borrows `shared_point_indeces` by default (freed
        // once, after this whole loop, same as before) and only owns a
        // private `Vec` -- built fresh by `parse_point_numbers` -- when
        // `PRIVATE_POINT_NUMBERS` is set for this tuple, matching the
        // original's "usually an alias, occasionally a fresh allocation
        // freed within this same iteration" shape exactly, but as a
        // `Cow` instead of a raw pointer that is only sometimes owned.
        let n_points: ShapeId;
        let point_indeces: ::std::borrow::Cow<[ShapeId]>;
        if be16((*tvh).tuple_index) as ::core::ffi::c_int & PRIVATE_POINT_NUMBERS != 0 {
            let (new_tsd, private_point_numbers) = parse_point_numbers(tsd, total_delta_entries);
            tsd = new_tsd;
            n_points = private_point_numbers.len() as ShapeId;
            point_indeces = ::std::borrow::Cow::Owned(private_point_numbers);
        } else {
            n_points = shared_point_indeces.len() as ShapeId;
            point_indeces = ::std::borrow::Cow::Borrowed(&shared_point_indeces);
        }
        if !point_indeces.is_empty() {
            // Local `Vec<Pos>`s, not `__caryll_allocate_clean`'d/`free`'d
            // buffers -- dropped automatically at the end of this `if`.
            let mut delta_x: Vec<Pos> = vec![0 as Pos; n_points as usize];
            let mut delta_y: Vec<Pos> = vec![0 as Pos; n_points as usize];
            tsd = read_packed_delta(tsd, n_points, delta_x.as_mut_ptr());
            tsd = read_packed_delta(tsd, n_points, delta_y.as_mut_ptr());
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
        tsd_start = tsd_start.wrapping_add(be16((*tvh).variation_data_size) as usize);
        tvh = next_tvh(tvh, ctx);
        j = j.wrapping_add(1);
    }
}
#[inline]
unsafe fn polymorphize(
    packet: &Packet,
    mut options: *const Options,
    mut glyf: *mut GlyfTable,
    mut ctx: *const GlyfIOContext,
) {
    if (*ctx).fvar.is_null() || (*(*ctx).fvar).axes.is_empty() {
        return;
    }
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_GVAR {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    if (table.length as usize) < ::core::mem::size_of::<GVARHeader>() {
                        return;
                    }
                    let mut header: *mut GVARHeader = data as *mut GVARHeader;
                    if be16((*header).axis_count) as usize != (*(*ctx).fvar).axes.len() {
                        logger_log_sds(
                            (*options).logger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"Axes number in GVAR and FVAR are inequal",
                            ),
                        );
                        return;
                    }
                    let mut j: GlyphId = 0 as GlyphId;
                    while (j as usize) < (*glyf).len() {
                        let mut tpctx: TuplePolymorphizerCtx = TuplePolymorphizerCtx {
                            fvar: (*ctx).fvar,
                            dimensions: (*(*ctx).fvar).axes.len() as u16,
                            shared_tuple_count: be16((*header).shared_tuple_count),
                            shared_tuples: data.offset(be32((*header).shared_tuples_offset) as isize)
                                as *mut F2Dot14,
                            coord_dimensions: 2 as u8,
                            allow_iup: !(&(*glyf))[j as usize].as_deref().unwrap().contours.is_empty(),
                            n_phantom_points: (*ctx).n_phantom_points,
                        };
                        let mut glyph_variation_data_offset: u32 = 0 as u32;
                        if be16((*header).flags) as ::core::ffi::c_int & GVAR_OFFSETS_ARE_LONG != 0
                        {
                            glyph_variation_data_offset = be32(
                                *(data
                                    .offset(::core::mem::size_of::<GVARHeader>() as isize)
                                    as *mut u32)
                                    .offset(j as isize),
                            );
                        } else {
                            glyph_variation_data_offset = (2 as ::core::ffi::c_int
                                * be16(
                                    *(data.offset(
                                        ::core::mem::size_of::<GVARHeader>() as isize
                                    ) as *mut u16)
                                        .offset(j as isize),
                                ) as ::core::ffi::c_int)
                                as u32;
                        }
                        let mut gvd: *mut GlyphVariationData = data
                            .offset(be32((*header).glyph_variation_data_array_offset) as isize)
                            .offset(glyph_variation_data_offset as isize)
                            as *mut GlyphVariationData;
                        polymorphize_glyph(
                            j,
                            &raw mut **(&mut (*glyf))[j as usize].as_mut().unwrap(),
                            &raw mut tpctx,
                            gvd,
                            options,
                        );
                        j = j.wrapping_add(1);
                    }
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
}
pub unsafe fn otfcc_read_glyf(
    packet: &Packet,
    options: *const Options,
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
            (*options).logger,
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

    let glyf_piece = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_GLYF)?;
    if glyf_piece.length < offsets[num_glyphs as usize] {
        logger_log_sds(
            (*options).logger,
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
        glyf.as_mut().map_or(::core::ptr::null_mut(), |g| g as *mut GlyfTable),
        ctx,
    );
    glyf
}
#[inline]
unsafe fn be16(mut x: u16) -> u16 {
    return ((x as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | (x as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int)
        as u16;
}
#[inline]
unsafe fn be32(mut x: u32) -> u32 {
    return (x & 0xff as u32) << 24 as ::core::ffi::c_int
        | (x & 0xff00 as u32) << 8 as ::core::ffi::c_int
        | (x & 0xff0000 as u32) >> 8 as ::core::ffi::c_int
        | (x & 0xff000000 as u32) >> 24 as ::core::ffi::c_int;
}

#[cfg(test)]
mod glyf_read_tests {
    use super::*;
    use crate::vf::vq::vq_get_still;

    fn zeroed_options() -> Options {
        unsafe { ::core::mem::zeroed() }
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
                &options as *const Options,
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
                &options as *const Options,
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
                &options as *const Options,
            );
            assert!(g.is_none());
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
                &options as *const Options,
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
                &options as *const Options,
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
                &options as *const Options,
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
                &options as *const Options,
            );
            assert!(g.is_none());
        }
    }
}
