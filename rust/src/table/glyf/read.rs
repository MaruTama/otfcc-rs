#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, memcpy};

use crate::support::handle::{handle_fromIndex, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_8u, read_8s, read_16u, read_16s, read_32u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, F2Dot14, FontFilePointer, GlyphId, Pos, Scale, ShapeId};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::fvar::FvarTable;
use crate::table::glyf::{GlyfIOContext, RefAnchorStatus, ComponentFlags, PointFlags, ComponentReference, Contour, ContourList, Glyph, GlyphPtr, Point, GlyfTable};


use crate::vf::region::{VqAxisSpan, VqRegion};
use crate::vf::vq::{VQ, VQSegType, VqSegment};
use crate::support::primitives::{otfcc_f1616_muldiv, otfcc_from_f2dot14, otfcc_from_fixed, otfcc_to_fixed};
use crate::table::fvar::{TABLE_I_FVAR};
use crate::table::glyf::{GLYF_I_COMPONENT_REFERENCE, GLYF_I_CONTOUR, GLYF_I_CONTOUR_LIST, GLYF_I_REFERENCE_LIST, otfcc_newGlyf_glyph, TABLE_I_GLYF};
use crate::vendor::sds::{sdsempty};
use crate::vf::region::{vq_createRegion};
use crate::vf::vq::{I_VQ, VQ_I_SEG_LIST};

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct GlyphVariationData {
    pub tupleVariationCount: u16,
    pub dataOffset: u16,
    pub tvhs: [TupleVariationHeader; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct TupleVariationHeader {
    pub variationDataSize: u16,
    pub tupleIndex: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct GVARHeader {
    pub majorVersion: u16,
    pub minorVersion: u16,
    pub axisCount: u16,
    pub sharedTupleCount: u16,
    pub sharedTuplesOffset: u32,
    pub glyphCount: u16,
    pub flags: u16,
    pub glyphVariationDataArrayOffset: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TuplePolymorphizerCtx {
    pub fvar: *mut FvarTable,
    pub dimensions: u16,
    pub sharedTupleCount: u16,
    pub sharedTuples: *mut F2Dot14,
    pub coordDimensions: u8,
    pub allowIUP: bool,
    pub nPhantomPoints: ShapeId,
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
    if *cp as usize >= (*(*contours).items.offset(*cc as isize)).length {
        *cp = 0 as ShapeId;
        *cc = (*cc as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
    }
    let fresh8 = *cp;
    *cp = (*cp).wrapping_add(1);
    return (*(*contours).items.offset(*cc as isize))
        .items
        .offset(fresh8 as isize) as *mut Point;
}
unsafe extern "C" fn otfcc_read_simple_glyph(
    mut start: FontFilePointer,
    mut numberOfContours: ShapeId,
    mut _options: *const Options,
) -> *mut Glyph {
    let mut g: *mut Glyph = otfcc_newGlyf_glyph();
    let mut contours: *mut ContourList = &raw mut (*g).contours;
    let mut pointsInGlyph: ShapeId = 0 as ShapeId;
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_int) < numberOfContours as ::core::ffi::c_int {
        let mut lastPointInCurrentContour: ShapeId = read_16u(
            start.offset((2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize)
                as *const u8,
        ) as ShapeId;
        let mut contour: Contour = Contour {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<Point>(),
        };
        GLYF_I_CONTOUR.init.expect("non-null function pointer")(&raw mut contour);
        GLYF_I_CONTOUR.fill.expect("non-null function pointer")(
            &raw mut contour,
            (lastPointInCurrentContour as ::core::ffi::c_int - pointsInGlyph as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as usize,
        );
        GLYF_I_CONTOUR_LIST.push.expect("non-null function pointer")(contours, contour);
        pointsInGlyph = (lastPointInCurrentContour as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as ShapeId;
        j = j.wrapping_add(1);
    }
    let mut instructionLength: u16 = read_16u(
        start.offset((2 as ::core::ffi::c_int * numberOfContours as ::core::ffi::c_int) as isize)
            as *const u8,
    );
    let mut instructions: *mut u8 = ::core::ptr::null_mut::<u8>();
    if instructionLength as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        instructions = __caryll_allocate_clean(
            (::core::mem::size_of::<u8>() as usize).wrapping_mul(instructionLength as usize),
            31 as ::core::ffi::c_ulong,
        ) as *mut u8;
        memcpy(
            instructions as *mut ::core::ffi::c_void,
            start
                .offset((2 as ::core::ffi::c_int * numberOfContours as ::core::ffi::c_int) as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (::core::mem::size_of::<u8>() as usize).wrapping_mul(instructionLength as usize),
        );
    }
    (*g).instructionsLength = instructionLength;
    (*g).instructions = instructions;
    let mut flags: FontFilePointer = ::core::ptr::null_mut::<u8>();
    flags = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul(pointsInGlyph as usize),
        41 as ::core::ffi::c_ulong,
    ) as FontFilePointer;
    let mut flagStart: FontFilePointer = start
        .offset((2 as ::core::ffi::c_int * numberOfContours as ::core::ffi::c_int) as isize)
        .offset(2 as ::core::ffi::c_int as isize)
        .offset(instructionLength as ::core::ffi::c_int as isize);
    let mut flagsReadSofar: ShapeId = 0 as ShapeId;
    let mut flagBytesReadSofar: ShapeId = 0 as ShapeId;
    let mut currentContour: ShapeId = 0 as ShapeId;
    let mut currentContourPointIndex: ShapeId = 0 as ShapeId;
    while (flagsReadSofar as ::core::ffi::c_int) < pointsInGlyph as ::core::ffi::c_int {
        let mut flag: PointFlags =
            PointFlags::from_bits_retain(*flagStart.offset(flagBytesReadSofar as isize));
        *flags.offset(flagsReadSofar as isize) = flag.bits();
        flagBytesReadSofar =
            (flagBytesReadSofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
        flagsReadSofar =
            (flagsReadSofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
        (*next_point(
            contours,
            &raw mut currentContour,
            &raw mut currentContourPointIndex,
        ))
        .onCurve = flag.contains(PointFlags::ON_CURVE) as i8;
        if flag.contains(PointFlags::REPEAT) {
            let mut repeat: u8 = *flagStart.offset(flagBytesReadSofar as isize);
            flagBytesReadSofar =
                (flagBytesReadSofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
            let mut j_0: u8 = 0 as u8;
            while (j_0 as ::core::ffi::c_int) < repeat as ::core::ffi::c_int {
                *flags.offset(
                    (flagsReadSofar as ::core::ffi::c_int + j_0 as ::core::ffi::c_int) as isize,
                ) = flag.bits();
                (*next_point(
                    contours,
                    &raw mut currentContour,
                    &raw mut currentContourPointIndex,
                ))
                .onCurve = flag.contains(PointFlags::ON_CURVE) as i8;
                j_0 = j_0.wrapping_add(1);
            }
            flagsReadSofar =
                (flagsReadSofar as ::core::ffi::c_int + repeat as ::core::ffi::c_int) as ShapeId;
        }
    }
    let mut coordinatesStart: FontFilePointer =
        flagStart.offset(flagBytesReadSofar as ::core::ffi::c_int as isize);
    let mut coordinatesOffset: u32 = 0 as u32;
    let mut coordinatesRead: ShapeId = 0 as ShapeId;
    currentContour = 0 as ShapeId;
    currentContourPointIndex = 0 as ShapeId;
    while (coordinatesRead as ::core::ffi::c_int) < pointsInGlyph as ::core::ffi::c_int {
        let mut flag_0: PointFlags =
            PointFlags::from_bits_retain(*flags.offset(coordinatesRead as isize));
        let mut x: i16 = 0;
        if flag_0.contains(PointFlags::X_SHORT) {
            x = ((if flag_0.contains(PointFlags::POSITIVE_X) {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            }) * read_8u(coordinatesStart.offset(coordinatesOffset as isize) as *const u8)
                as ::core::ffi::c_int) as i16;
            coordinatesOffset = coordinatesOffset.wrapping_add(1 as u32);
        } else if flag_0.contains(PointFlags::SAME_X) {
            x = 0 as i16;
        } else {
            x = read_16s(coordinatesStart.offset(coordinatesOffset as isize) as *const u8);
            coordinatesOffset = coordinatesOffset.wrapping_add(2 as u32);
        }
        I_VQ.replace.expect("non-null function pointer")(
            &raw mut (*(next_point
                as unsafe extern "C" fn(
                    *mut ContourList,
                    *mut ShapeId,
                    *mut ShapeId,
                ) -> *mut Point)(
                contours,
                &raw mut currentContour,
                &raw mut currentContourPointIndex,
            ))
            .x,
            I_VQ.createStill.expect("non-null function pointer")(x as Pos) as VQ,
        );
        coordinatesRead =
            (coordinatesRead as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
    }
    coordinatesRead = 0 as ShapeId;
    currentContour = 0 as ShapeId;
    currentContourPointIndex = 0 as ShapeId;
    while (coordinatesRead as ::core::ffi::c_int) < pointsInGlyph as ::core::ffi::c_int {
        let mut flag_1: PointFlags =
            PointFlags::from_bits_retain(*flags.offset(coordinatesRead as isize));
        let mut y: i16 = 0;
        if flag_1.contains(PointFlags::Y_SHORT) {
            y = ((if flag_1.contains(PointFlags::POSITIVE_Y) {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            }) * read_8u(coordinatesStart.offset(coordinatesOffset as isize) as *const u8)
                as ::core::ffi::c_int) as i16;
            coordinatesOffset = coordinatesOffset.wrapping_add(1 as u32);
        } else if flag_1.contains(PointFlags::SAME_Y) {
            y = 0 as i16;
        } else {
            y = read_16s(coordinatesStart.offset(coordinatesOffset as isize) as *const u8);
            coordinatesOffset = coordinatesOffset.wrapping_add(2 as u32);
        }
        I_VQ.replace.expect("non-null function pointer")(
            &raw mut (*(next_point
                as unsafe extern "C" fn(
                    *mut ContourList,
                    *mut ShapeId,
                    *mut ShapeId,
                ) -> *mut Point)(
                contours,
                &raw mut currentContour,
                &raw mut currentContourPointIndex,
            ))
            .y,
            I_VQ.createStill.expect("non-null function pointer")(y as Pos) as VQ,
        );
        coordinatesRead =
            (coordinatesRead as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
    }
    free(flags as *mut ::core::ffi::c_void);
    flags = ::core::ptr::null_mut::<u8>();
    let mut cx: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    let mut cy: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    let mut j_1: ShapeId = 0 as ShapeId;
    while (j_1 as ::core::ffi::c_int) < numberOfContours as ::core::ffi::c_int {
        let mut k: ShapeId = 0 as ShapeId;
        while (k as usize) < (*(*contours).items.offset(j_1 as isize)).length {
            let mut z: *mut Point = (*(*contours).items.offset(j_1 as isize))
                .items
                .offset(k as isize) as *mut Point;
            I_VQ.inplacePlus.expect("non-null function pointer")(&raw mut cx, (*z).x);
            I_VQ.inplacePlus.expect("non-null function pointer")(&raw mut cy, (*z).y);
            I_VQ.copyReplace.expect("non-null function pointer")(&raw mut (*z).x, cx);
            I_VQ.copyReplace.expect("non-null function pointer")(&raw mut (*z).y, cy);
            k = k.wrapping_add(1);
        }
        GLYF_I_CONTOUR
            .shrinkToFit
            .expect("non-null function pointer")(
            (*contours).items.offset(j_1 as isize) as *mut Contour
        );
        j_1 = j_1.wrapping_add(1);
    }
    GLYF_I_CONTOUR_LIST
        .shrinkToFit
        .expect("non-null function pointer")(contours);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut cx);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut cy);
    return g;
}
unsafe extern "C" fn otfcc_read_composite_glyph(
    mut start: FontFilePointer,
    mut options: *const Options,
) -> *mut Glyph {
    let mut g: *mut Glyph = otfcc_newGlyf_glyph();
    let mut flags: ComponentFlags = ComponentFlags::empty();
    let mut offset: u32 = 0 as u32;
    let mut glyphHasInstruction: bool = false;
    loop {
        flags = ComponentFlags::from_bits_retain(read_16u(
            start.offset(offset as isize) as *const u8,
        ));
        let mut index: GlyphId = read_16u(
            start
                .offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as GlyphId;
        let mut ref_0: ComponentReference =
            (
                GLYF_I_COMPONENT_REFERENCE
                    .empty
                    .expect("non-null function pointer"))();
        ref_0.glyph =
            handle_fromIndex(index) as GlyphHandle;
        offset = offset.wrapping_add(4 as u32);
        if flags.contains(ComponentFlags::ARGS_ARE_XY_VALUES) {
            ref_0.isAnchored = RefAnchorStatus::Xy;
            if flags.contains(ComponentFlags::ARG_1_AND_2_ARE_WORDS) {
                ref_0.x = I_VQ.createStill.expect("non-null function pointer")(read_16s(
                    start.offset(offset as isize) as *const u8,
                )
                    as Pos);
                ref_0.y = I_VQ.createStill.expect("non-null function pointer")(read_16s(
                    start
                        .offset(offset as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const u8,
                )
                    as Pos);
                offset = offset.wrapping_add(4 as u32);
            } else {
                ref_0.x = I_VQ.createStill.expect("non-null function pointer")(read_8s(
                    start.offset(offset as isize) as *const u8,
                )
                    as Pos);
                ref_0.y = I_VQ.createStill.expect("non-null function pointer")(read_8s(
                    start
                        .offset(offset as isize)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *const u8,
                )
                    as Pos);
                offset = offset.wrapping_add(2 as u32);
            }
        } else {
            ref_0.isAnchored = RefAnchorStatus::AnchorAnchor;
            if flags.contains(ComponentFlags::ARG_1_AND_2_ARE_WORDS) {
                ref_0.outer =
                    read_16u(start.offset(offset as isize) as *const u8) as ShapeId;
                ref_0.inner = read_16u(
                    start
                        .offset(offset as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as ShapeId;
                offset = offset.wrapping_add(4 as u32);
            } else {
                ref_0.outer = read_8u(start.offset(offset as isize) as *const u8) as ShapeId;
                ref_0.inner = read_8u(
                    start
                        .offset(offset as isize)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as ShapeId;
                offset = offset.wrapping_add(2 as u32);
            }
        }
        if flags.contains(ComponentFlags::WE_HAVE_A_SCALE) {
            ref_0.d = otfcc_from_f2dot14(
                read_16s(start.offset(offset as isize) as *const u8) as F2Dot14
            ) as Scale;
            ref_0.a = ref_0.d;
            offset = offset.wrapping_add(2 as u32);
        } else if flags.contains(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE)
        {
            ref_0.a = otfcc_from_f2dot14(
                read_16s(start.offset(offset as isize) as *const u8) as F2Dot14
            ) as Scale;
            ref_0.d = otfcc_from_f2dot14(read_16s(
                start
                    .offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as F2Dot14) as Scale;
            offset = offset.wrapping_add(4 as u32);
        } else if flags.contains(ComponentFlags::WE_HAVE_A_TWO_BY_TWO) {
            ref_0.a = otfcc_from_f2dot14(
                read_16s(start.offset(offset as isize) as *const u8) as F2Dot14
            ) as Scale;
            ref_0.b = otfcc_from_f2dot14(read_16s(
                start
                    .offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as F2Dot14) as Scale;
            ref_0.c = otfcc_from_f2dot14(read_16s(
                start
                    .offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as F2Dot14) as Scale;
            ref_0.d = otfcc_from_f2dot14(read_16s(
                start
                    .offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            ) as F2Dot14) as Scale;
            offset = offset.wrapping_add(8 as u32);
        }
        ref_0.roundToGrid =
            flags.contains(ComponentFlags::ROUND_XY_TO_GRID);
        ref_0.useMyMetrics =
            flags.contains(ComponentFlags::USE_MY_METRICS);
        if flags.contains(ComponentFlags::SCALED_COMPONENT_OFFSET)
            && (flags.contains(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE)
                || flags.contains(ComponentFlags::WE_HAVE_A_TWO_BY_TWO))
        {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(sdsempty(), b"glyf: SCALED_COMPONENT_OFFSET is not supported."),
            );
        }
        if flags.contains(ComponentFlags::WE_HAVE_INSTRUCTIONS) {
            glyphHasInstruction = true;
        }
        GLYF_I_REFERENCE_LIST.push.expect("non-null function pointer")(
            &raw mut (*g).references,
            ref_0,
        );
        if !(flags.contains(ComponentFlags::MORE_COMPONENTS)) {
            break;
        }
    }
    if glyphHasInstruction {
        let mut instructionLength: u16 =
            read_16u(start.offset(offset as isize) as *const u8);
        let mut instructions: FontFilePointer = ::core::ptr::null_mut::<u8>();
        if instructionLength as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            instructions = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize)
                    .wrapping_mul(instructionLength as usize),
                201 as ::core::ffi::c_ulong,
            ) as FontFilePointer;
            memcpy(
                instructions as *mut ::core::ffi::c_void,
                start
                    .offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (::core::mem::size_of::<u8>() as usize)
                    .wrapping_mul(instructionLength as usize),
            );
        }
        (*g).instructionsLength = instructionLength;
        (*g).instructions = instructions as *mut u8;
    } else {
        (*g).instructionsLength = 0 as u16;
        (*g).instructions = ::core::ptr::null_mut::<u8>();
    }
    return g;
}
unsafe extern "C" fn otfcc_read_glyph(
    mut data: FontFilePointer,
    mut offset: u32,
    mut options: *const Options,
) -> *mut Glyph {
    let mut start: FontFilePointer = data.offset(offset as isize);
    let mut numberOfContours: i16 = read_16u(start as *const u8) as i16;
    let mut g: *mut Glyph = ::core::ptr::null_mut::<Glyph>();
    if numberOfContours as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        g = otfcc_read_simple_glyph(
            start.offset(10 as ::core::ffi::c_int as isize),
            numberOfContours as ShapeId,
            options,
        );
    } else {
        g = otfcc_read_composite_glyph(start.offset(10 as ::core::ffi::c_int as isize), options);
    }
    (*g).stat.xMin =
        read_16s(start.offset(2 as ::core::ffi::c_int as isize) as *const u8) as Pos;
    (*g).stat.yMin =
        read_16s(start.offset(4 as ::core::ffi::c_int as isize) as *const u8) as Pos;
    (*g).stat.xMax =
        read_16s(start.offset(6 as ::core::ffi::c_int as isize) as *const u8) as Pos;
    (*g).stat.yMax =
        read_16s(start.offset(8 as ::core::ffi::c_int as isize) as *const u8) as Pos;
    return g;
}
pub const GVAR_OFFSETS_ARE_LONG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EMBEDDED_PEAK_TUPLE: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const INTERMEDIATE_REGION: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const PRIVATE_POINT_NUMBERS: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const TUPLE_INDEX_MASK: ::core::ffi::c_int = 0xfff as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn nextTVH(
    mut currentHeader: *mut TupleVariationHeader,
    mut ctx: *const TuplePolymorphizerCtx,
) -> *mut TupleVariationHeader {
    let mut bump: u32 =
        2_usize.wrapping_mul(::core::mem::size_of::<u16>()) as u32;
    let mut tupleIndex: u16 = be16((*currentHeader).tupleIndex);
    if tupleIndex as ::core::ffi::c_int & EMBEDDED_PEAK_TUPLE != 0 {
        bump = (bump as ::core::ffi::c_ulong).wrapping_add(
            ((*ctx).dimensions as usize).wrapping_mul(::core::mem::size_of::<F2Dot14>())
                as ::core::ffi::c_ulong,
        ) as u32 as u32;
    }
    if tupleIndex as ::core::ffi::c_int & INTERMEDIATE_REGION != 0 {
        bump = (bump as ::core::ffi::c_ulong).wrapping_add(
            ((2 as ::core::ffi::c_int * (*ctx).dimensions as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<F2Dot14>())
                as ::core::ffi::c_ulong,
        ) as u32 as u32;
    }
    return (currentHeader as FontFilePointer).offset(bump as isize) as *mut TupleVariationHeader;
}
pub const POINT_COUNT_IS_WORD: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const POINT_COUNT_LONG_MASK: ::core::ffi::c_int = 0x7fff as ::core::ffi::c_int;
pub const POINT_RUN_COUNT_MASK: ::core::ffi::c_int = 0x7f as ::core::ffi::c_int;
pub const POINTS_ARE_WORDS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn parsePointNumbers(
    mut data: FontFilePointer,
    mut pointIndeces: *mut *mut ShapeId,
    mut pc: *mut ShapeId,
    mut totalPoints: ShapeId,
) -> FontFilePointer {
    let mut nPoints: u16 = 0 as u16;
    let mut firstByte: u8 = *data;
    if firstByte as ::core::ffi::c_int & POINT_COUNT_IS_WORD != 0 {
        nPoints = (((*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            & POINT_COUNT_LONG_MASK) as u16;
        data = data.offset(2 as ::core::ffi::c_int as isize);
    } else {
        nPoints = firstByte as u16;
        data = data.offset(1);
    }
    if nPoints as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut run: PackedPointRun = PackedPointRun {
            length: 0 as ShapeId,
            wide: false,
        };
        let mut filled: ShapeId = 0 as ShapeId;
        let mut jPoint: ShapeId = 0 as ShapeId;
        *pointIndeces = __caryll_allocate_clean(
            (::core::mem::size_of::<ShapeId>() as usize).wrapping_mul(nPoints as usize),
            305 as ::core::ffi::c_ulong,
        ) as *mut ShapeId;
        while (filled as ::core::ffi::c_int) < nPoints as ::core::ffi::c_int {
            if run.length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let fresh6 = data;
                data = data.offset(1);
                let mut runHeader: u8 = *fresh6;
                run.wide = runHeader as ::core::ffi::c_int & POINTS_ARE_WORDS != 0;
                run.length = ((runHeader as ::core::ffi::c_int & POINT_RUN_COUNT_MASK)
                    + 1 as ::core::ffi::c_int) as ShapeId;
            }
            let mut pointNumber: i16 = jPoint as i16;
            if run.wide {
                pointNumber = (pointNumber as ::core::ffi::c_int
                    + *(data as *mut u16) as ::core::ffi::c_int)
                    as i16;
                data = data.offset(2 as ::core::ffi::c_int as isize);
            } else {
                let fresh7 = data;
                data = data.offset(1);
                pointNumber =
                    (pointNumber as ::core::ffi::c_int + *fresh7 as ::core::ffi::c_int) as i16;
            }
            *(*pointIndeces).offset(filled as isize) = pointNumber as ShapeId;
            filled = filled.wrapping_add(1);
            jPoint = pointNumber as ShapeId;
            run.length = run.length.wrapping_sub(1);
        }
        *pc = nPoints as ShapeId;
    } else {
        *pointIndeces = __caryll_allocate_clean(
            (::core::mem::size_of::<ShapeId>() as usize).wrapping_mul(totalPoints as usize),
            326 as ::core::ffi::c_ulong,
        ) as *mut ShapeId;
        let mut j: ShapeId = 0 as ShapeId;
        while (j as ::core::ffi::c_int) < totalPoints as ::core::ffi::c_int {
            *(*pointIndeces).offset(j as isize) = j;
            j = j.wrapping_add(1);
        }
        *pc = totalPoints;
    }
    return data;
}
pub const DELTAS_ARE_ZERO: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const DELTAS_ARE_WORDS: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const DELTA_RUN_COUNT_MASK: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn readPackedDelta(
    mut data: FontFilePointer,
    mut nPoints: ShapeId,
    mut deltas: *mut Pos,
) -> FontFilePointer {
    let mut run: PackedDeltaRun = PackedDeltaRun {
        length: 0 as ShapeId,
        wide: false,
        zero: false,
    };
    let mut filled: ShapeId = 0 as ShapeId;
    while (filled as ::core::ffi::c_int) < nPoints as ::core::ffi::c_int {
        let mut delta: i16 = 0 as i16;
        if run.length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let fresh5 = data;
            data = data.offset(1);
            let mut runHeader: u8 = *fresh5;
            run.zero = runHeader as ::core::ffi::c_int & DELTAS_ARE_ZERO != 0;
            run.wide = runHeader as ::core::ffi::c_int & DELTAS_ARE_WORDS != 0;
            run.length = ((runHeader as ::core::ffi::c_int & DELTA_RUN_COUNT_MASK)
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
pub unsafe extern "C" fn getX(mut z: *mut Point) -> *mut VQ {
    return &raw mut (*z).x;
}
pub unsafe extern "C" fn getY(mut z: *mut Point) -> *mut VQ {
    return &raw mut (*z).y;
}
#[inline]
unsafe extern "C" fn fillTheGaps(
    mut jMin: ShapeId,
    mut jMax: ShapeId,
    mut nudges: *mut VqSegment,
    mut glyphRefs: *mut *mut Point,
    mut getter: CoordPartGetter,
) {
    let mut j: ShapeId = jMin;
    while (j as ::core::ffi::c_int) < jMax as ::core::ffi::c_int {
        if !(*nudges.offset(j as isize)).val.delta.touched {
            let mut jNext: ShapeId = j;
            while !(*nudges.offset(jNext as isize)).val.delta.touched {
                if jNext as ::core::ffi::c_int
                    == jMax as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    jNext = jMin;
                } else {
                    jNext = (jNext as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
                }
                if jNext as ::core::ffi::c_int == j as ::core::ffi::c_int {
                    break;
                }
            }
            let mut jPrev: ShapeId = j;
            while !(*nudges.offset(jPrev as isize)).val.delta.touched {
                if jPrev as ::core::ffi::c_int == jMin as ::core::ffi::c_int {
                    jPrev = (jMax as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ShapeId;
                } else {
                    jPrev = (jPrev as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ShapeId;
                }
                if jPrev as ::core::ffi::c_int == j as ::core::ffi::c_int {
                    break;
                }
            }
            if (*nudges.offset(jNext as isize)).val.delta.touched as ::core::ffi::c_int != 0
                && (*nudges.offset(jPrev as isize)).val.delta.touched as ::core::ffi::c_int != 0
            {
                let mut untouchJ: F16Dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(*glyphRefs.offset(j as isize)))
                        .kernel as ::core::ffi::c_double,
                );
                let mut untouchPrev: F16Dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(*glyphRefs.offset(jPrev as isize)))
                        .kernel as ::core::ffi::c_double,
                );
                let mut untouchNext: F16Dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(*glyphRefs.offset(jNext as isize)))
                        .kernel as ::core::ffi::c_double,
                );
                let mut deltaPrev: F16Dot16 = otfcc_to_fixed(
                    (*nudges.offset(jPrev as isize)).val.delta.quantity as ::core::ffi::c_double,
                );
                let mut deltaNext: F16Dot16 = otfcc_to_fixed(
                    (*nudges.offset(jNext as isize)).val.delta.quantity as ::core::ffi::c_double,
                );
                let mut uMin: F16Dot16 = untouchPrev;
                let mut uMax: F16Dot16 = untouchNext;
                let mut dMin: F16Dot16 = deltaPrev;
                let mut dMax: F16Dot16 = deltaNext;
                if untouchPrev > untouchNext {
                    uMin = untouchNext;
                    uMax = untouchPrev;
                    dMin = deltaNext;
                    dMax = deltaPrev;
                }
                if untouchJ <= uMin {
                    (*nudges.offset(j as isize)).val.delta.quantity =
                        otfcc_from_fixed(dMin) as Pos;
                } else if untouchJ >= uMax {
                    (*nudges.offset(j as isize)).val.delta.quantity =
                        otfcc_from_fixed(dMax) as Pos;
                } else {
                    (*nudges.offset(j as isize)).val.delta.quantity = otfcc_from_fixed(
                        otfcc_f1616_muldiv(dMax - dMin, untouchJ - uMin, uMax - uMin),
                    )
                        as Pos;
                }
            }
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn applyCoords(
    totalPoints: ShapeId,
    mut glyph: *mut Glyph,
    mut glyphRefs: *mut *mut Point,
    nTouchedPoints: ShapeId,
    mut tupleDelta: *const Pos,
    mut points: *const ShapeId,
    mut r: *const VqRegion,
    mut getter: CoordPartGetter,
) {
    let mut nudges: *mut VqSegment = ::core::ptr::null_mut::<VqSegment>();
    nudges = __caryll_allocate_clean(
        (::core::mem::size_of::<VqSegment>() as usize).wrapping_mul(totalPoints as usize),
        441 as ::core::ffi::c_ulong,
    ) as *mut VqSegment;
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_int) < totalPoints as ::core::ffi::c_int {
        (*nudges.offset(j as isize)).type_0 = VQSegType::Delta;
        (*nudges.offset(j as isize)).val.delta.touched = false;
        (*nudges.offset(j as isize)).val.delta.quantity = 0 as ::core::ffi::c_int as Pos;
        let ref mut fresh4 = (*nudges.offset(j as isize)).val.delta.region;
        *fresh4 = r;
        j = j.wrapping_add(1);
    }
    let mut j_0: ShapeId = 0 as ShapeId;
    while (j_0 as ::core::ffi::c_int) < nTouchedPoints as ::core::ffi::c_int {
        if !(*points.offset(j_0 as isize) as ::core::ffi::c_int
            >= totalPoints as ::core::ffi::c_int)
        {
            (*nudges.offset(*points.offset(j_0 as isize) as isize))
                .val
                .delta
                .touched = true;
            (*nudges.offset(*points.offset(j_0 as isize) as isize))
                .val
                .delta
                .quantity += *tupleDelta.offset(j_0 as isize);
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut jFirst: ShapeId = 0 as ShapeId;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*glyph).contours.length {
        let mut c: *mut Contour = (*glyph).contours.items.offset(__caryll_index as isize);
        while keep != 0 {
            fillTheGaps(
                jFirst,
                (jFirst as usize).wrapping_add((*c).length) as ShapeId,
                nudges,
                glyphRefs,
                Some(getX as unsafe extern "C" fn(*mut Point) -> *mut VQ),
            );
            jFirst = (jFirst as usize).wrapping_add((*c).length) as ShapeId as ShapeId;
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut j_1: ShapeId = 0 as ShapeId;
    while (j_1 as ::core::ffi::c_int) < totalPoints as ::core::ffi::c_int {
        if !((*nudges.offset(j_1 as isize)).val.delta.quantity == 0.
            && (*nudges.offset(j_1 as isize)).val.delta.touched as ::core::ffi::c_int != 0)
        {
            let mut coordinatePart: *mut VQ =
                getter.expect("non-null function pointer")(*glyphRefs.offset(j_1 as isize));
            VQ_I_SEG_LIST.push.expect("non-null function pointer")(
                &raw mut (*coordinatePart).shift,
                *nudges.offset(j_1 as isize),
            );
        }
        j_1 = j_1.wrapping_add(1);
    }
    free(nudges as *mut ::core::ffi::c_void);
    nudges = ::core::ptr::null_mut::<VqSegment>();
}
#[inline]
unsafe extern "C" fn applyPolymorphism(
    totalPoints: ShapeId,
    mut glyph: GlyphPtr,
    nTouchedPoints: ShapeId,
    mut points: *const ShapeId,
    mut deltaX: *const Pos,
    mut deltaY: *const Pos,
    mut r: *const VqRegion,
) {
    let mut glyphRefs: *mut *mut Point = ::core::ptr::null_mut::<*mut Point>();
    glyphRefs = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut Point>() as usize).wrapping_mul(totalPoints as usize),
        473 as ::core::ffi::c_ulong,
    ) as *mut *mut Point;
    let mut j: ShapeId = 0 as ShapeId;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*glyph).contours.length {
        let mut c: *mut Contour = (*glyph).contours.items.offset(__caryll_index as isize);
        while keep != 0 {
            let mut __caryll_index_0: usize = 0 as usize;
            let mut keep_0: usize = 1 as usize;
            while keep_0 != 0 && __caryll_index_0 < (*c).length {
                let mut g: *mut Point = (*c).items.offset(__caryll_index_0 as isize);
                while keep_0 != 0 {
                    let fresh0 = j;
                    j = j.wrapping_add(1);
                    let ref mut fresh1 = *glyphRefs.offset(fresh0 as isize);
                    *fresh1 = g;
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
    while keep_1 != 0 && __caryll_index_1 < (*glyph).references.length {
        let mut r_0: *mut ComponentReference =
            (*glyph).references.items.offset(__caryll_index_1 as isize);
        while keep_1 != 0 {
            let fresh2 = j;
            j = j.wrapping_add(1);
            let ref mut fresh3 = *glyphRefs.offset(fresh2 as isize);
            *fresh3 = &raw mut (*r_0).x as *mut Point;
            keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
        }
        keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
        __caryll_index_1 = __caryll_index_1.wrapping_add(1);
    }
    applyCoords(
        totalPoints,
        glyph as *mut Glyph,
        glyphRefs,
        nTouchedPoints,
        deltaX,
        points,
        r,
        Some(getX as unsafe extern "C" fn(*mut Point) -> *mut VQ),
    );
    applyCoords(
        totalPoints,
        glyph as *mut Glyph,
        glyphRefs,
        nTouchedPoints,
        deltaY,
        points,
        r,
        Some(getY as unsafe extern "C" fn(*mut Point) -> *mut VQ),
    );
    if (totalPoints as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        < nTouchedPoints as ::core::ffi::c_int
    {
        I_VQ.addDelta.expect("non-null function pointer")(
            &raw mut (*glyph).horizontalOrigin,
            true,
            r,
            *deltaX.offset(totalPoints as isize),
        );
        I_VQ.addDelta.expect("non-null function pointer")(
            &raw mut (*glyph).advanceWidth,
            true,
            r,
            *deltaX.offset((totalPoints as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                - *deltaX.offset(totalPoints as isize),
        );
    }
    if (totalPoints as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
        < nTouchedPoints as ::core::ffi::c_int
    {
        I_VQ.addDelta.expect("non-null function pointer")(
            &raw mut (*glyph).verticalOrigin,
            true,
            r,
            *deltaY.offset((totalPoints as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize),
        );
        I_VQ.addDelta.expect("non-null function pointer")(
            &raw mut (*glyph).advanceHeight,
            true,
            r,
            *deltaY.offset((totalPoints as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                - *deltaY
                    .offset((totalPoints as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize),
        );
    }
    free(glyphRefs as *mut ::core::ffi::c_void);
    glyphRefs = ::core::ptr::null_mut::<*mut Point>();
}
unsafe extern "C" fn createRegionFromTuples(
    mut dimensions: u16,
    mut peak: *mut F2Dot14,
    mut start: *mut F2Dot14,
    mut end: *mut F2Dot14,
) -> *mut VqRegion {
    let mut r: *mut VqRegion = vq_createRegion(dimensions as ShapeId);
    let mut d: u16 = 0 as u16;
    while (d as ::core::ffi::c_int) < dimensions as ::core::ffi::c_int {
        let mut peakVal: Pos =
            otfcc_from_f2dot14(be16(*peak.offset(d as isize) as u16) as F2Dot14) as Pos;
        let mut span: VqAxisSpan = VqAxisSpan {
            start: (if peakVal <= 0 as ::core::ffi::c_int as Pos {
                -(1 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            }) as Pos,
            peak: peakVal,
            end: (if peakVal >= 0 as ::core::ffi::c_int as Pos {
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
unsafe extern "C" fn polymorphizeGlyph(
    mut _gid: GlyphId,
    mut glyph: GlyphPtr,
    mut ctx: *const TuplePolymorphizerCtx,
    mut gvd: *mut GlyphVariationData,
    mut _options: *const Options,
) {
    let mut totalPoints: ShapeId = 0 as ShapeId;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*glyph).contours.length {
        let mut c: *mut Contour = (*glyph).contours.items.offset(__caryll_index as isize);
        while keep != 0 {
            totalPoints =
                (totalPoints as usize).wrapping_add((*c).length) as ShapeId as ShapeId;
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    totalPoints =
        (totalPoints as usize).wrapping_add((*glyph).references.length) as ShapeId as ShapeId;
    let mut totalDeltaEntries: ShapeId = (totalPoints as ::core::ffi::c_int
        + (*ctx).nPhantomPoints as ::core::ffi::c_int)
        as ShapeId;
    let mut nTuples: u16 = (be16((*gvd).tupleVariationCount) as ::core::ffi::c_int
        & 0xfff as ::core::ffi::c_int) as u16;
    let mut tvh: *mut TupleVariationHeader = &raw mut (*gvd).tvhs as *mut TupleVariationHeader;
    let mut hasSharedPointNumbers: bool =
        be16((*gvd).tupleVariationCount) as ::core::ffi::c_int & 0x8000 as ::core::ffi::c_int != 0;
    let mut sharedPointCount: ShapeId = 0 as ShapeId;
    let mut sharedPointIndeces: *mut ShapeId = ::core::ptr::null_mut::<ShapeId>();
    let mut data: FontFilePointer =
        (gvd as FontFilePointer).offset(be16((*gvd).dataOffset) as ::core::ffi::c_int as isize);
    if hasSharedPointNumbers {
        data = parsePointNumbers(
            data,
            &raw mut sharedPointIndeces,
            &raw mut sharedPointCount,
            totalDeltaEntries,
        );
    }
    let mut tsdStart: usize = 0 as usize;
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < nTuples as ::core::ffi::c_int {
        let mut tupleIndex: ShapeId =
            (be16((*tvh).tupleIndex) as ::core::ffi::c_int & TUPLE_INDEX_MASK) as ShapeId;
        let mut hasEmbeddedPeak: bool =
            be16((*tvh).tupleIndex) as ::core::ffi::c_int & EMBEDDED_PEAK_TUPLE != 0;
        let mut hasIntermediate: bool =
            be16((*tvh).tupleIndex) as ::core::ffi::c_int & INTERMEDIATE_REGION != 0;
        let mut peak: *mut F2Dot14 = ::core::ptr::null_mut::<F2Dot14>();
        if hasEmbeddedPeak {
            peak =
                (tvh as FontFilePointer).offset(4 as ::core::ffi::c_int as isize) as *mut F2Dot14;
        } else {
            peak = (*ctx).sharedTuples.offset(
                ((*ctx).dimensions as ::core::ffi::c_int * tupleIndex as ::core::ffi::c_int)
                    as isize,
            );
        }
        let mut start: *mut F2Dot14 = ::core::ptr::null_mut::<F2Dot14>();
        let mut end: *mut F2Dot14 = ::core::ptr::null_mut::<F2Dot14>();
        if hasIntermediate {
            start = (tvh as FontFilePointer)
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(
                    (2 as ::core::ffi::c_int
                        * (if hasEmbeddedPeak as ::core::ffi::c_int != 0 {
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
                        * (if hasEmbeddedPeak as ::core::ffi::c_int != 0 {
                            2 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        })
                        * (*ctx).dimensions as ::core::ffi::c_int) as isize,
                ) as *mut F2Dot14;
        }
        let mut r: *const VqRegion = TABLE_I_FVAR
            .registerRegion
            .expect("non-null function pointer")(
            (*ctx).fvar,
            createRegionFromTuples((*ctx).dimensions, peak, start, end),
        );
        let mut tsd: FontFilePointer = data.offset(tsdStart as isize);
        let mut nPoints: ShapeId = sharedPointCount;
        let mut pointIndeces: *mut ShapeId = sharedPointIndeces;
        if be16((*tvh).tupleIndex) as ::core::ffi::c_int & PRIVATE_POINT_NUMBERS != 0 {
            let mut privatePointCount: ShapeId = 0 as ShapeId;
            let mut privatePointNumbers: *mut ShapeId = ::core::ptr::null_mut::<ShapeId>();
            tsd = parsePointNumbers(
                tsd,
                &raw mut privatePointNumbers,
                &raw mut privatePointCount,
                totalDeltaEntries,
            );
            nPoints = privatePointCount;
            pointIndeces = privatePointNumbers;
        }
        if !pointIndeces.is_null() {
            let mut deltaX: *mut Pos = ::core::ptr::null_mut::<Pos>();
            let mut deltaY: *mut Pos = ::core::ptr::null_mut::<Pos>();
            deltaX = __caryll_allocate_clean(
                (::core::mem::size_of::<Pos>() as usize).wrapping_mul(nPoints as usize),
                586 as ::core::ffi::c_ulong,
            ) as *mut Pos;
            deltaY = __caryll_allocate_clean(
                (::core::mem::size_of::<Pos>() as usize).wrapping_mul(nPoints as usize),
                587 as ::core::ffi::c_ulong,
            ) as *mut Pos;
            tsd = readPackedDelta(tsd, nPoints, deltaX);
            tsd = readPackedDelta(tsd, nPoints, deltaY);
            applyPolymorphism(totalPoints, glyph, nPoints, pointIndeces, deltaX, deltaY, r);
            free(deltaX as *mut ::core::ffi::c_void);
            deltaX = ::core::ptr::null_mut::<Pos>();
            free(deltaY as *mut ::core::ffi::c_void);
            deltaY = ::core::ptr::null_mut::<Pos>();
        }
        if be16((*tvh).tupleIndex) as ::core::ffi::c_int & PRIVATE_POINT_NUMBERS != 0 {
            free(pointIndeces as *mut ::core::ffi::c_void);
            pointIndeces = ::core::ptr::null_mut::<ShapeId>();
        }
        tsdStart = tsdStart.wrapping_add(be16((*tvh).variationDataSize) as usize);
        tvh = nextTVH(tvh, ctx);
        j = j.wrapping_add(1);
    }
    free(sharedPointIndeces as *mut ::core::ffi::c_void);
    sharedPointIndeces = ::core::ptr::null_mut::<ShapeId>();
}
#[inline]
unsafe extern "C" fn polymorphize(
    packet: Packet,
    mut options: *const Options,
    mut glyf: *mut GlyfTable,
    mut ctx: *const GlyfIOContext,
) {
    if (*ctx).fvar.is_null() || (*(*ctx).fvar).axes.length == 0 {
        return;
    }
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1735811442i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    if (table.length as usize) < ::core::mem::size_of::<GVARHeader>() {
                        return;
                    }
                    let mut header: *mut GVARHeader = data as *mut GVARHeader;
                    if be16((*header).axisCount) as usize != (*(*ctx).fvar).axes.length {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Axes number in GVAR and FVAR are inequal",
                            ),
                        );
                        return;
                    }
                    let mut j: GlyphId = 0 as GlyphId;
                    while (j as usize) < (*glyf).length {
                        let mut tpctx: TuplePolymorphizerCtx = TuplePolymorphizerCtx {
                            fvar: (*ctx).fvar,
                            dimensions: (*(*ctx).fvar).axes.length as u16,
                            sharedTupleCount: be16((*header).sharedTupleCount),
                            sharedTuples: data.offset(be32((*header).sharedTuplesOffset) as isize)
                                as *mut F2Dot14,
                            coordDimensions: 2 as u8,
                            allowIUP: (**(*glyf).items.offset(j as isize)).contours.length
                                > 0 as usize,
                            nPhantomPoints: (*ctx).nPhantomPoints,
                        };
                        let mut glyphVariationDataOffset: u32 = 0 as u32;
                        if be16((*header).flags) as ::core::ffi::c_int & GVAR_OFFSETS_ARE_LONG != 0
                        {
                            glyphVariationDataOffset = be32(
                                *(data
                                    .offset(::core::mem::size_of::<GVARHeader>() as isize)
                                    as *mut u32)
                                    .offset(j as isize),
                            );
                        } else {
                            glyphVariationDataOffset = (2 as ::core::ffi::c_int
                                * be16(
                                    *(data.offset(
                                        ::core::mem::size_of::<GVARHeader>() as isize
                                    ) as *mut u16)
                                        .offset(j as isize),
                                ) as ::core::ffi::c_int)
                                as u32;
                        }
                        let mut gvd: *mut GlyphVariationData = data
                            .offset(be32((*header).glyphVariationDataArrayOffset) as isize)
                            .offset(glyphVariationDataOffset as isize)
                            as *mut GlyphVariationData;
                        polymorphizeGlyph(
                            j,
                            *(*glyf).items.offset(j as isize),
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
pub unsafe extern "C" fn otfcc_readGlyf(
    packet: Packet,
    mut options: *const Options,
    mut ctx: *const GlyfIOContext,
) -> *mut GlyfTable {
    let mut foundLoca: bool = false;
    let mut current_block: u64;
    let mut offsets: *mut u32 = ::core::ptr::null_mut::<u32>();
    let mut glyf: *mut GlyfTable = ::core::ptr::null_mut::<GlyfTable>();
    offsets = __caryll_allocate_clean(
        (::core::mem::size_of::<u32>() as usize).wrapping_mul(
            ((*ctx).numGlyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
        ),
        649 as ::core::ffi::c_ulong,
    ) as *mut u32;
    if !offsets.is_null() {
        foundLoca = false;
        let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while __notfound != 0
            && __fortable_keep != 0
            && __fortable_count < packet.numTables as ::core::ffi::c_int
        {
            let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
            while __fortable_keep != 0 {
                if table.tag == 1819239265i32 as u32 {
                    let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    while __fortable_k2 != 0 {
                        let mut data: FontFilePointer = table.data as FontFilePointer;
                        let mut length: u32 = table.length;
                        if !(length
                            < (2 as ::core::ffi::c_int * (*ctx).numGlyphs as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int)
                                as u32)
                        {
                            let mut j: u32 = 0 as u32;
                            loop {
                                if !(j
                                    < ((*ctx).numGlyphs as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int)
                                        as u32)
                                {
                                    current_block = 7149356873433890176;
                                    break;
                                }
                                if (*ctx).locaIsLong {
                                    *offsets.offset(j as isize) = read_32u(
                                        data.offset(j.wrapping_mul(4 as u32) as isize)
                                            as *const u8,
                                    );
                                } else {
                                    *offsets.offset(j as isize) = (read_16u(
                                        data.offset(j.wrapping_mul(2 as u32) as isize)
                                            as *const u8,
                                    )
                                        as ::core::ffi::c_int
                                        * 2 as ::core::ffi::c_int)
                                        as u32;
                                }
                                if j > 0 as u32
                                    && *offsets.offset(j as isize)
                                        < *offsets.offset(j.wrapping_sub(1 as u32) as isize)
                                {
                                    current_block = 15756379620357860923;
                                    break;
                                }
                                j = j.wrapping_add(1);
                            }
                            match current_block {
                                15756379620357860923 => {}
                                _ => {
                                    foundLoca = true;
                                    break;
                                }
                            }
                        }
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::sdsbuild!(sdsempty(), b"table 'loca' corrupted.\n"),
                        );
                        if !offsets.is_null() {
                            free(offsets as *mut ::core::ffi::c_void);
                            offsets = ::core::ptr::null_mut::<u32>();
                            offsets = ::core::ptr::null_mut::<u32>();
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
        if foundLoca {
            let mut __fortable_keep_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut __fortable_count_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut __notfound_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            's_126: loop {
                if !(__notfound_0 != 0
                    && __fortable_keep_0 != 0
                    && __fortable_count_0 < packet.numTables as ::core::ffi::c_int)
                {
                    current_block = 4135528745514935090;
                    break;
                }
                let mut table_0: PacketPiece =
                    *packet.pieces.offset(__fortable_count_0 as isize);
                while __fortable_keep_0 != 0 {
                    if table_0.tag == 1735162214i32 as u32 {
                        let mut __fortable_k2_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        while __fortable_k2_0 != 0 {
                            let mut data_0: FontFilePointer = table_0.data as FontFilePointer;
                            let mut length_0: u32 = table_0.length;
                            if length_0 < *offsets.offset((*ctx).numGlyphs as isize) {
                                (*(*options).logger)
                                    .logSDS
                                    .expect("non-null function pointer")(
                                    (*options).logger as *mut ILogger,
                                    LOG_VL_IMPORTANT,
                                    LoggerType::Warning,
                                    crate::sdsbuild!(sdsempty(), b"table 'glyf' corrupted.\n"),
                                );
                                if !glyf.is_null() {
                                    TABLE_I_GLYF.free.expect("non-null function pointer")(glyf);
                                    glyf = ::core::ptr::null_mut::<GlyfTable>();
                                    glyf = ::core::ptr::null_mut::<GlyfTable>();
                                }
                                __fortable_k2_0 = 0 as ::core::ffi::c_int;
                                __notfound_0 = 0 as ::core::ffi::c_int;
                            } else {
                                glyf = (
                                    TABLE_I_GLYF.create.expect("non-null function pointer"))();
                                let mut j_0: GlyphId = 0 as GlyphId;
                                while (j_0 as ::core::ffi::c_int)
                                    < (*ctx).numGlyphs as ::core::ffi::c_int
                                {
                                    if *offsets.offset(j_0 as isize)
                                        < *offsets.offset(
                                            (j_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                    {
                                        TABLE_I_GLYF.push.expect("non-null function pointer")(
                                            glyf,
                                            otfcc_read_glyph(
                                                data_0,
                                                *offsets.offset(j_0 as isize),
                                                options,
                                            )
                                                as GlyphPtr,
                                        );
                                    } else {
                                        TABLE_I_GLYF.push.expect("non-null function pointer")(
                                            glyf,
                                            otfcc_newGlyf_glyph() as GlyphPtr,
                                        );
                                    }
                                    j_0 = j_0.wrapping_add(1);
                                }
                                current_block = 5675710991063777755;
                                break 's_126;
                            }
                        }
                    }
                    __fortable_keep_0 = (__fortable_keep_0 == 0) as ::core::ffi::c_int;
                }
                __fortable_keep_0 = (__fortable_keep_0 == 0) as ::core::ffi::c_int;
                __fortable_count_0 += 1;
            }
            match current_block {
                4135528745514935090 => {}
                _ => {
                    if !offsets.is_null() {
                        free(offsets as *mut ::core::ffi::c_void);
                        offsets = ::core::ptr::null_mut::<u32>();
                        offsets = ::core::ptr::null_mut::<u32>();
                    }
                    polymorphize(packet, options, glyf, ctx);
                    return glyf;
                }
            }
        }
    }
    if !offsets.is_null() {
        free(offsets as *mut ::core::ffi::c_void);
        offsets = ::core::ptr::null_mut::<u32>();
        offsets = ::core::ptr::null_mut::<u32>();
    }
    if !glyf.is_null() {
        free(glyf as *mut ::core::ffi::c_void);
        glyf = ::core::ptr::null_mut::<GlyfTable>();
        glyf = ::core::ptr::null_mut::<GlyfTable>();
    }
    return ::core::ptr::null_mut::<GlyfTable>();
}
#[inline]
unsafe extern "C" fn be16(mut x: u16) -> u16 {
    return ((x as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | (x as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int)
        as u16;
}
#[inline]
unsafe extern "C" fn be32(mut x: u32) -> u32 {
    return (x & 0xff as u32) << 24 as ::core::ffi::c_int
        | (x & 0xff00 as u32) << 8 as ::core::ffi::c_int
        | (x & 0xff0000 as u32) >> 8 as ::core::ffi::c_int
        | (x & 0xff000000 as u32) >> 24 as ::core::ffi::c_int;
}
