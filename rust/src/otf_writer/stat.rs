#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, time, time_t};
unsafe extern "C" {
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}


use crate::support::handle::{HANDLE_STATE_EMPTY, handle_fromIndex, GlyphHandle, Handle, otfcc_Handle_replace};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{log_type_warning, log_vl_important, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, GlyphId, Length, Pos, Scale, ShapeId};
use crate::font::caryll_font::{FONTTYPE_CFF, FONTTYPE_TTF, Font};



use crate::table::CFF::{CffFontMatrix, CffTable};



use crate::table::LTSH::{LtshTable};



use crate::table::VORG::{VorgEntry, VorgTable};

use crate::table::cmap::{CmapEntry};




use crate::table::glyf::{REF_XY, ComponentReference, Glyph, GlyphStat, Point, GlyfTable};



use crate::table::hmtx::{HorizontalMetric, HmtxTable};



use crate::table::otl::{GsubLigatureEntry, Lookup, otl_type_gpos_chaining, otl_type_gpos_markToBase, otl_type_gpos_markToLigature, otl_type_gpos_markToMark, otl_type_gpos_pair, otl_type_gsub_chaining, otl_type_gsub_ligature, otl_type_gsub_reverse, ChainingSubtable, GsubLigatureSubtable, GsubReverseSubtable, OtlTable};




use crate::table::vmtx::{VmtxTable, VerticalMetric};



use crate::vf::vq::{VQ, VqSegList, VqSegment};
use crate::font::caryll_font::{otfcc_iFont};
use crate::table::glyf::{glyf_iComponentReference};
use crate::vendor::sds::{sdsempty};
use crate::vf::vq::{iVQ};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StatStatus {
    stat_not_started = 0,
    stat_doing = 1,
    stat_completed = 2,
}
pub use StatStatus::*;
pub const POS_MAX: ::core::ffi::c_float = FLT_MAX;
pub unsafe extern "C" fn stat_single_glyph(
    mut table: *mut GlyfTable,
    mut gr: *mut ComponentReference,
    mut stated: *mut StatStatus,
    mut depth: u8,
    mut topj: GlyphId,
    mut options: *const Options,
) -> GlyphStat {
    let mut stat: GlyphStat = GlyphStat {
        xMin: 0 as ::core::ffi::c_int as Pos,
        xMax: 0 as ::core::ffi::c_int as Pos,
        yMin: 0 as ::core::ffi::c_int as Pos,
        yMax: 0 as ::core::ffi::c_int as Pos,
        nestDepth: 0 as u16,
        nPoints: 0 as u16,
        nContours: 0 as u16,
        nCompositePoints: 0 as u16,
        nCompositeContours: 0 as u16,
    };
    let j: GlyphId = (*gr).glyph.index;
    if depth as ::core::ffi::c_int >= 0xff as ::core::ffi::c_int {
        return stat;
    }
    if *stated.offset(j as isize) == stat_doing {
        (*(*options).logger)
            .logSDS
            .expect(
                "non-null function pointer",
            )(
            (*options).logger as *mut ILogger,
            log_vl_important,
            log_type_warning,
            crate::sdsbuild!(
                sdsempty(),
                b"[Stat] Circular glyph reference found in gid ",
                topj as ::core::ffi::c_int,
                b" to gid ",
                j as ::core::ffi::c_int,
                b". The reference will be dropped.\n",
            ),
        );
        *stated.offset(j as isize) = stat_completed;
        return stat;
    }
    let g: *mut Glyph = *(*table).items.offset(j as isize) as *mut Glyph;
    *stated.offset(j as isize) = stat_doing;
    let mut xmin: Pos = POS_MAX as Pos;
    let mut xmax: Pos = -POS_MAX as Pos;
    let mut ymin: Pos = POS_MAX as Pos;
    let mut ymax: Pos = -POS_MAX as Pos;
    let mut nestDepth: u16 = 0 as u16;
    let mut nPoints: u16 = 0 as u16;
    let mut nCompositePoints: u16 = 0 as u16;
    let mut nCompositeContours: u16 = 0 as u16;
    for c in 0..(*g).contours.length as ShapeId {
        let contour = (*g).contours.items.offset(c as isize);
        for pj in 0..(*contour).length as ShapeId {
            let p: *mut Point = (*contour).items.offset(pj as isize) as *mut Point;
            let x: Pos = round(
                iVQ.getStill.expect("non-null function pointer")((*gr).x) as ::core::ffi::c_double
                    + (*gr).a as ::core::ffi::c_double
                        * iVQ.getStill.expect("non-null function pointer")((*p).x)
                            as ::core::ffi::c_double
                    + (*gr).b as ::core::ffi::c_double
                        * iVQ.getStill.expect("non-null function pointer")((*p).y)
                            as ::core::ffi::c_double,
            ) as Pos;
            let mut y: Pos = round(
                iVQ.getStill.expect("non-null function pointer")((*gr).y) as ::core::ffi::c_double
                    + (*gr).c as ::core::ffi::c_double
                        * iVQ.getStill.expect("non-null function pointer")((*p).x)
                            as ::core::ffi::c_double
                    + (*gr).d as ::core::ffi::c_double
                        * iVQ.getStill.expect("non-null function pointer")((*p).y)
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
            nPoints = (nPoints as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
        }
    }
    nCompositePoints = nPoints;
    nCompositeContours = (*g).contours.length as u16;
    for r in 0..(*g).references.length as ShapeId {
        let mut ref_0: ComponentReference = ComponentReference {
            x: VQ {
                kernel: 0.,
                shift: VqSegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<VqSegment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: VqSegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<VqSegment>(),
                },
            },
            roundToGrid: false,
            useMyMetrics: false,
            glyph: Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            isAnchored: REF_XY,
            inner: 0,
            outer: 0,
        };
        glyf_iComponentReference
            .init
            .expect("non-null function pointer")(&raw mut ref_0);
        let rr: *mut ComponentReference =
            (*g).references.items.offset(r as isize) as *mut ComponentReference;
        otfcc_Handle_replace(
            &raw mut ref_0.glyph,
            handle_fromIndex((*rr).glyph.index)
                as Handle,
        );
        ref_0.a = (*gr).a * (*rr).a + (*rr).b * (*gr).c;
        ref_0.b = (*rr).a * (*gr).b + (*rr).b * (*gr).d;
        ref_0.c = (*gr).a * (*rr).c + (*gr).c * (*rr).d;
        ref_0.d = (*gr).b * (*rr).c + (*rr).d * (*gr).d;
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.x,
            iVQ.createStill.expect("non-null function pointer")(
                iVQ.getStill.expect("non-null function pointer")((*rr).x)
                    + (*rr).a as Pos * iVQ.getStill.expect("non-null function pointer")((*gr).x)
                    + (*rr).b as Pos * iVQ.getStill.expect("non-null function pointer")((*gr).y),
            ) as VQ,
        );
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.y,
            iVQ.createStill.expect("non-null function pointer")(
                iVQ.getStill.expect("non-null function pointer")((*rr).y)
                    + (*rr).c as Pos * iVQ.getStill.expect("non-null function pointer")((*gr).x)
                    + (*rr).d as Pos * iVQ.getStill.expect("non-null function pointer")((*gr).y),
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
        if thatstat.xMin < xmin {
            xmin = thatstat.xMin;
        }
        if thatstat.xMax > xmax {
            xmax = thatstat.xMax;
        }
        if thatstat.yMin < ymin {
            ymin = thatstat.yMin;
        }
        if thatstat.yMax > ymax {
            ymax = thatstat.yMax;
        }
        if thatstat.nestDepth as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            > nestDepth as ::core::ffi::c_int
        {
            nestDepth =
                (thatstat.nestDepth as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
        }
        nCompositePoints = (nCompositePoints as ::core::ffi::c_int
            + thatstat.nCompositePoints as ::core::ffi::c_int)
            as u16;
        nCompositeContours = (nCompositeContours as ::core::ffi::c_int
            + thatstat.nCompositeContours as ::core::ffi::c_int)
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
    stat.xMin = xmin;
    stat.xMax = xmax;
    stat.yMin = ymin;
    stat.yMax = ymax;
    stat.nestDepth = nestDepth;
    stat.nPoints = nPoints;
    stat.nContours = (*g).contours.length as u16;
    stat.nCompositePoints = nCompositePoints;
    stat.nCompositeContours = nCompositeContours;
    *stated.offset(j as isize) = stat_completed;
    return stat;
}
pub unsafe extern "C" fn statGlyf(mut font: *mut Font, mut options: *const Options) {
    let mut stated: *mut StatStatus = ::core::ptr::null_mut::<StatStatus>();
    stated = __caryll_allocate_clean(
        (::core::mem::size_of::<StatStatus>() as usize).wrapping_mul((*(*font).glyf).length),
        99 as ::core::ffi::c_ulong,
    ) as *mut StatStatus;
    let mut xmin: Pos = 0xffffffff as ::core::ffi::c_uint as Pos;
    let mut xmax: Pos = (0xffffffff as ::core::ffi::c_uint).wrapping_neg() as Pos;
    let mut ymin: Pos = 0xffffffff as ::core::ffi::c_uint as Pos;
    let mut ymax: Pos = (0xffffffff as ::core::ffi::c_uint).wrapping_neg() as Pos;
    for j in 0..(*(*font).glyf).length as GlyphId {
        let mut gr: ComponentReference = ComponentReference {
            x: VQ {
                kernel: 0.,
                shift: VqSegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<VqSegment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: VqSegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<VqSegment>(),
                },
            },
            roundToGrid: false,
            useMyMetrics: false,
            glyph: Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            isAnchored: REF_XY,
            inner: 0,
            outer: 0,
        };
        gr.glyph =
            handle_fromIndex(j) as GlyphHandle;
        gr.x =
            iVQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
        gr.y =
            iVQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
        gr.a = 1 as ::core::ffi::c_int as Scale;
        gr.b = 0 as ::core::ffi::c_int as Scale;
        gr.c = 0 as ::core::ffi::c_int as Scale;
        gr.d = 1 as ::core::ffi::c_int as Scale;
        let ref mut fresh2 = (**(*(*font).glyf).items.offset(j as isize)).stat;
        *fresh2 = stat_single_glyph((*font).glyf, &raw mut gr, stated, 0 as u8, j, options);
        let mut thatstat: GlyphStat = *fresh2;
        if thatstat.xMin < xmin {
            xmin = thatstat.xMin;
        }
        if thatstat.xMax > xmax {
            xmax = thatstat.xMax;
        }
        if thatstat.yMin < ymin {
            ymin = thatstat.yMin;
        }
        if thatstat.yMax > ymax {
            ymax = thatstat.yMax;
        }
    }
    (*(*font).head).xMin = xmin as i16;
    (*(*font).head).xMax = xmax as i16;
    (*(*font).head).yMin = ymin as i16;
    (*(*font).head).yMax = ymax as i16;
    free(stated as *mut ::core::ffi::c_void);
    stated = ::core::ptr::null_mut::<StatStatus>();
}
pub unsafe extern "C" fn statMaxp(mut font: *mut Font) {
    let mut nestDepth: u16 = 0 as u16;
    let mut nPoints: u16 = 0 as u16;
    let mut nContours: u16 = 0 as u16;
    let mut nComponents: u16 = 0 as u16;
    let mut nCompositePoints: u16 = 0 as u16;
    let mut nCompositeContours: u16 = 0 as u16;
    let mut instSize: u16 = 0 as u16;
    for j in 0..(*(*font).glyf).length as GlyphId {
        let g: *mut Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut Glyph;
        if (*g).contours.length > 0 as usize {
            if (*g).stat.nPoints as ::core::ffi::c_int > nPoints as ::core::ffi::c_int {
                nPoints = (*g).stat.nPoints;
            }
            if (*g).stat.nContours as ::core::ffi::c_int > nContours as ::core::ffi::c_int {
                nContours = (*g).stat.nContours;
            }
        } else if (*g).references.length > 0 as usize {
            if (*g).stat.nCompositePoints as ::core::ffi::c_int
                > nCompositePoints as ::core::ffi::c_int
            {
                nCompositePoints = (*g).stat.nCompositePoints;
            }
            if (*g).stat.nCompositeContours as ::core::ffi::c_int
                > nCompositeContours as ::core::ffi::c_int
            {
                nCompositeContours = (*g).stat.nCompositeContours;
            }
            if (*g).stat.nestDepth as ::core::ffi::c_int > nestDepth as ::core::ffi::c_int {
                nestDepth = (*g).stat.nestDepth;
            }
            if (*g).references.length > nComponents as usize {
                nComponents = (*g).references.length as u16;
            }
        }
        if (*g).instructionsLength as ::core::ffi::c_int > instSize as ::core::ffi::c_int {
            instSize = (*g).instructionsLength;
        }
    }
    (*(*font).maxp).maxPoints = nPoints;
    (*(*font).maxp).maxContours = nContours;
    (*(*font).maxp).maxCompositePoints = nCompositePoints;
    (*(*font).maxp).maxCompositeContours = nCompositeContours;
    (*(*font).maxp).maxComponentDepth = nestDepth;
    (*(*font).maxp).maxComponentElements = nComponents;
    (*(*font).maxp).maxSizeOfInstructions = instSize;
}
unsafe extern "C" fn statHmtx(mut font: *mut Font, mut _options: *const Options) {
    if (*font).glyf.is_null() {
        return;
    }
    let mut hmtx: *mut HmtxTable = ::core::ptr::null_mut::<HmtxTable>();
    hmtx = __caryll_allocate_clean(
        ::core::mem::size_of::<HmtxTable>() as usize,
        162 as ::core::ffi::c_ulong,
    ) as *mut HmtxTable;
    let mut count_a: GlyphId = (*(*font).glyf).length as GlyphId;
    let mut count_k: GlyphId = 0 as GlyphId;
    let mut lsbAtX_0: bool = true;
    if (*font).subtype != FONTTYPE_CFF {
        while count_a as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            && iVQ.getStill.expect("non-null function pointer")(
                (**(*(*font).glyf)
                    .items
                    .offset((count_a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                .advanceWidth,
            ) == iVQ.getStill.expect("non-null function pointer")(
                (**(*(*font).glyf)
                    .items
                    .offset((count_a as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize))
                .advanceWidth,
            )
        {
            count_a = count_a.wrapping_sub(1);
        }
        count_k = (*(*font).glyf).length.wrapping_sub(count_a as usize) as GlyphId;
    }
    (*hmtx).metrics = __caryll_allocate_clean(
        (::core::mem::size_of::<HorizontalMetric>() as usize).wrapping_mul(count_a as usize),
        175 as ::core::ffi::c_ulong,
    ) as *mut HorizontalMetric;
    (*hmtx).leftSideBearing = __caryll_allocate_clean(
        (::core::mem::size_of::<Pos>() as usize).wrapping_mul(count_k as usize),
        176 as ::core::ffi::c_ulong,
    ) as *mut Pos;
    let mut minLSB: Pos = 0x7fff as ::core::ffi::c_int as Pos;
    let mut minRSB: Pos = 0x7fff as ::core::ffi::c_int as Pos;
    let mut maxExtent: Pos = -(0x8000 as ::core::ffi::c_int) as Pos;
    let mut maxWidth: Length = 0 as ::core::ffi::c_int as Length;
    for j in 0..(*(*font).glyf).length as GlyphId {
        let g: *mut Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut Glyph;
        if iVQ.isZero.expect("non-null function pointer")((*g).horizontalOrigin, 1.0f64 / 1000.0f64)
        {
            iVQ.replace.expect("non-null function pointer")(
                &raw mut (*g).horizontalOrigin,
                (
                    iVQ.neutral.expect("non-null function pointer"))() as VQ,
            );
        } else {
            lsbAtX_0 = false;
        }
        let hori: Pos =
            iVQ.getStill.expect("non-null function pointer")((*g).horizontalOrigin) as Pos;
        let advw: Pos =
            iVQ.getStill.expect("non-null function pointer")((*g).advanceWidth) as Pos;
        let lsb: Pos = (*g).stat.xMin - hori;
        let rsb: Pos = advw + hori - (*g).stat.xMax;
        if (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            (*(*hmtx).metrics.offset(j as isize)).advanceWidth = advw as Length;
            (*(*hmtx).metrics.offset(j as isize)).lsb = lsb;
        } else {
            *(*hmtx)
                .leftSideBearing
                .offset((j as ::core::ffi::c_int - count_a as ::core::ffi::c_int) as isize) = lsb;
        }
        if advw > maxWidth {
            maxWidth = advw as Length;
        }
        if lsb < minLSB {
            minLSB = lsb;
        }
        if rsb < minRSB {
            minRSB = rsb;
        }
        if (*g).stat.xMax - hori > maxExtent {
            maxExtent = (*g).stat.xMax - hori;
        }
    }
    (*(*font).hhea).numberOfMetrics = count_a as u16;
    (*(*font).hhea).minLeftSideBearing = minLSB as i16;
    (*(*font).hhea).minRightSideBearing = minRSB as i16;
    (*(*font).hhea).xMaxExtent = maxExtent as i16;
    (*(*font).hhea).advanceWidthMax = maxWidth as u16;
    (*font).hmtx = hmtx;
    (*(*font).head).flags = ((*(*font).head).flags as ::core::ffi::c_int
        & !(0x2 as ::core::ffi::c_int)
        | (if lsbAtX_0 { 0x2 as ::core::ffi::c_int } else { 0 as ::core::ffi::c_int }))
        as u16;
}
unsafe extern "C" fn statVmtx(mut font: *mut Font, mut options: *const Options) {
    if (*font).glyf.is_null() {
        return;
    }
    let mut vmtx: *mut VmtxTable = ::core::ptr::null_mut::<VmtxTable>();
    vmtx = __caryll_allocate_clean(
        ::core::mem::size_of::<VmtxTable>() as usize,
        218 as ::core::ffi::c_ulong,
    ) as *mut VmtxTable;
    let mut count_a: GlyphId = (*(*font).glyf).length as GlyphId;
    let mut count_k: GlyphId = 0 as GlyphId;
    if !((*font).subtype == FONTTYPE_CFF && !(*options).cff_short_vmtx) {
        while count_a as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            && iVQ.getStill.expect("non-null function pointer")(
                (**(*(*font).glyf)
                    .items
                    .offset((count_a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                .advanceHeight,
            ) == iVQ.getStill.expect("non-null function pointer")(
                (**(*(*font).glyf)
                    .items
                    .offset((count_a as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize))
                .advanceHeight,
            )
        {
            count_a = count_a.wrapping_sub(1);
        }
        count_k = (*(*font).glyf).length.wrapping_sub(count_a as usize) as GlyphId;
    }
    (*vmtx).metrics = __caryll_allocate_clean(
        (::core::mem::size_of::<VerticalMetric>() as usize).wrapping_mul(count_a as usize),
        230 as ::core::ffi::c_ulong,
    ) as *mut VerticalMetric;
    (*vmtx).topSideBearing = __caryll_allocate_clean(
        (::core::mem::size_of::<Pos>() as usize).wrapping_mul(count_k as usize),
        231 as ::core::ffi::c_ulong,
    ) as *mut Pos;
    let mut minTSB: Pos = 0x7fff as ::core::ffi::c_int as Pos;
    let mut minBSB: Pos = 0x7fff as ::core::ffi::c_int as Pos;
    let mut maxExtent: Pos = -(0x8000 as ::core::ffi::c_int) as Pos;
    let mut maxHeight: Length = 0 as ::core::ffi::c_int as Length;
    for j in 0..(*(*font).glyf).length as GlyphId {
        let g: *mut Glyph = *(*(*font).glyf).items.offset(j as isize) as *mut Glyph;
        let vori: Pos =
            iVQ.getStill.expect("non-null function pointer")((*g).verticalOrigin) as Pos;
        let advh: Pos =
            iVQ.getStill.expect("non-null function pointer")((*g).advanceHeight) as Pos;
        let tsb: Pos = vori - (*g).stat.yMax;
        let bsb: Pos = (*g).stat.yMin - vori + advh;
        if (j as ::core::ffi::c_int) < count_a as ::core::ffi::c_int {
            (*(*vmtx).metrics.offset(j as isize)).advanceHeight = advh as Length;
            (*(*vmtx).metrics.offset(j as isize)).tsb = tsb;
        } else {
            *(*vmtx)
                .topSideBearing
                .offset((j as ::core::ffi::c_int - count_a as ::core::ffi::c_int) as isize) = tsb;
        }
        if advh > maxHeight {
            maxHeight = advh as Length;
        }
        if tsb < minTSB {
            minTSB = tsb;
        }
        if bsb < minBSB {
            minBSB = bsb;
        }
        if vori - (*g).stat.yMin > maxExtent {
            maxExtent = vori - (*g).stat.yMin;
        }
    }
    (*(*font).vhea).numOfLongVerMetrics = count_a as u16;
    (*(*font).vhea).minTop = minTSB as i16;
    (*(*font).vhea).minBottom = minBSB as i16;
    (*(*font).vhea).yMaxExtent = maxExtent as i16;
    (*(*font).vhea).advanceHeightMax = maxHeight as i16;
    (*font).vmtx = vmtx;
}
unsafe extern "C" fn statOS_2UnicodeRanges(
    mut font: *mut Font,
    mut options: *const Options,
) {
    let mut item: *mut CmapEntry = ::core::ptr::null_mut::<CmapEntry>();
    let mut u1: u32 = 0 as u32;
    let mut u2: u32 = 0 as u32;
    let mut u3: u32 = 0 as u32;
    let mut u4: u32 = 0 as u32;
    let mut minUnicode: i32 = 0xffff as i32;
    let mut maxUnicode: i32 = 0 as i32;
    item = (*(*font).cmap).unicodes;
    while !item.is_null() {
        let mut u: ::core::ffi::c_int = (*item).unicode;
        if (u as i32) < minUnicode {
            minUnicode = u as i32;
        }
        if u as i32 > maxUnicode {
            maxUnicode = u as i32;
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
        item = (*item).hh.next as *mut CmapEntry;
    }
    if !(*options).keep_unicode_ranges {
        (*(*font).OS_2).ulUnicodeRange1 = u1;
        (*(*font).OS_2).ulUnicodeRange2 = u2;
        (*(*font).OS_2).ulUnicodeRange3 = u3;
        (*(*font).OS_2).ulUnicodeRange4 = u4;
    }
    if minUnicode < 0x10000 as i32 {
        (*(*font).OS_2).usFirstCharIndex = minUnicode as u16;
    } else {
        (*(*font).OS_2).usFirstCharIndex = 0xffff as u16;
    }
    if maxUnicode < 0x10000 as i32 {
        (*(*font).OS_2).usLastCharIndex = maxUnicode as u16;
    } else {
        (*(*font).OS_2).usLastCharIndex = 0xffff as u16;
    };
}
unsafe extern "C" fn statOS_2AverageWidth(
    mut font: *mut Font,
    mut options: *const Options,
) {
    if (*options).keep_average_char_width {
        return;
    }
    let mut totalWidth: u32 = 0 as u32;
    for j in 0..(*(*font).glyf).length as GlyphId {
        let adw: Pos = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j as isize)).advanceWidth,
        ) as Pos;
        if adw > 0 as ::core::ffi::c_int as Pos {
            totalWidth = (totalWidth as Pos + adw) as u32;
        }
    }
    (*(*font).OS_2).xAvgCharWidth =
        (totalWidth as usize).wrapping_div((*(*font).glyf).length) as i16;
}
unsafe extern "C" fn statMaxContextOTL(table: *const OtlTable) -> u16 {
    // c2rust's translation of otfcc's own `foreach(item, vector) { ... }`
    // macro (c/lib/otf-writer/stat.c): the __caryll_index*/keep* variables
    // simulate a single-iteration inner while purely so the macro body can
    // `continue`/`break`; every occurrence here reduces to a plain indexed
    // for loop over the vector, confirmed against the original C source.
    let mut maxc: u16 = 1 as u16;
    for i in 0..(*table).lookups.length {
        let lookup: *mut Lookup = *(*table).lookups.items.offset(i as isize);
        match (*lookup).type_0 {
            otl_type_gpos_pair | otl_type_gpos_markToBase | otl_type_gpos_markToLigature
            | otl_type_gpos_markToMark => {
                if (maxc as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                    maxc = 2 as u16;
                }
            }
            otl_type_gsub_ligature => {
                for si in 0..(*lookup).subtables.length {
                    let subtable: *mut GsubLigatureSubtable =
                        *(*lookup).subtables.items.offset(si as isize) as *mut GsubLigatureSubtable;
                    for ei in 0..(*subtable).length {
                        let entry: *mut GsubLigatureEntry = (*subtable).items.offset(ei as isize);
                        if (maxc as ::core::ffi::c_int) < (*(*entry).from).numGlyphs as ::core::ffi::c_int
                        {
                            maxc = (*(*entry).from).numGlyphs as u16;
                        }
                    }
                }
            }
            otl_type_gsub_chaining | otl_type_gpos_chaining => {
                for si in 0..(*lookup).subtables.length {
                    let subtable: *mut ChainingSubtable =
                        *(*lookup).subtables.items.offset(si as isize) as *mut ChainingSubtable;
                    if (maxc as ::core::ffi::c_int)
                        < (*subtable).c2rust_unnamed.rule.matchCount as ::core::ffi::c_int
                    {
                        maxc = (*subtable).c2rust_unnamed.rule.matchCount as u16;
                    }
                }
            }
            otl_type_gsub_reverse => {
                for si in 0..(*lookup).subtables.length {
                    let subtable: *mut GsubReverseSubtable =
                        *(*lookup).subtables.items.offset(si as isize) as *mut GsubReverseSubtable;
                    if (maxc as ::core::ffi::c_int) < (*subtable).matchCount as ::core::ffi::c_int {
                        maxc = (*subtable).matchCount as u16;
                    }
                }
            }
            _ => {}
        }
    }
    return maxc;
}
unsafe extern "C" fn statMaxContext(mut font: *mut Font, mut _options: *const Options) {
    let mut maxc: u16 = 1 as u16;
    if !(*font).GSUB.is_null() {
        let mut maxc_gsub: u16 = statMaxContextOTL((*font).GSUB);
        if maxc_gsub as ::core::ffi::c_int > maxc as ::core::ffi::c_int {
            maxc = maxc_gsub;
        }
    }
    if !(*font).GPOS.is_null() {
        let mut maxc_gpos: u16 = statMaxContextOTL((*font).GPOS);
        if maxc_gpos as ::core::ffi::c_int > maxc as ::core::ffi::c_int {
            maxc = maxc_gpos;
        }
    }
    (*(*font).OS_2).usMaxContext = maxc;
}
unsafe extern "C" fn statOS_2(mut font: *mut Font, mut options: *const Options) {
    statOS_2UnicodeRanges(font, options);
    statOS_2AverageWidth(font, options);
    statMaxContext(font, options);
}
pub const MAX_STAT_METRIC: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe extern "C" fn statCFFWidths(mut font: *mut Font) {
    if (*font).glyf.is_null() || (*font).CFF_.is_null() {
        return;
    }
    let mut frequency: *mut u32 = ::core::ptr::null_mut::<u32>();
    frequency = __caryll_allocate_clean(
        (::core::mem::size_of::<u32>() as usize).wrapping_mul(4096 as usize),
        524 as ::core::ffi::c_ulong,
    ) as *mut u32;
    for j in 0..(*(*font).glyf).length as GlyphId {
        let intWidth: u16 = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j as isize)).advanceWidth,
        ) as u16;
        if (intWidth as ::core::ffi::c_int) < MAX_STAT_METRIC {
            let fresh1 = frequency.offset(intWidth as isize);
            *fresh1 = (*fresh1).wrapping_add(1 as u32);
        }
    }
    let mut maxfreq: u16 = 0 as u16;
    let mut maxj: u16 = 0 as u16;
    for j_0 in 0..MAX_STAT_METRIC as u16 {
        if *frequency.offset(j_0 as isize) > maxfreq as u32 {
            maxfreq = *frequency.offset(j_0 as isize) as u16;
            maxj = j_0;
        }
    }
    let mut nn: u16 = 0 as u16;
    let mut nnsum: u32 = 0 as u32;
    for j_1 in 0..(*(*font).glyf).length as GlyphId {
        let adw: Pos = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j_1 as isize)).advanceWidth,
        ) as Pos;
        if adw != maxj as ::core::ffi::c_int as Pos {
            nn = (nn as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
            nnsum = (nnsum as Pos + adw) as u32;
        }
    }
    let mut nominalWidthX: i16 = 0 as i16;
    if nn as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        nominalWidthX = nnsum.wrapping_div(nn as u32) as i16;
    }
    if !(*(*font).CFF_).privateDict.is_null() {
        (*(*(*font).CFF_).privateDict).defaultWidthX = maxj as ::core::ffi::c_double;
        if nn as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            (*(*(*font).CFF_).privateDict).nominalWidthX = nominalWidthX as ::core::ffi::c_double;
        }
    }
    if !(*(*font).CFF_).fdArray.is_null() {
        for j_2 in 0..(*(*font).CFF_).fdArrayCount {
            let fd = *(*(*font).CFF_).fdArray.offset(j_2 as isize);
            (*(*fd).privateDict).defaultWidthX = maxj as ::core::ffi::c_double;
            (*(*fd).privateDict).nominalWidthX = nominalWidthX as ::core::ffi::c_double;
        }
    }
    free(frequency as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn statVORG(mut font: *mut Font) {
    if (*font).glyf.is_null()
        || (*font).CFF_.is_null()
        || (*font).vhea.is_null()
        || (*font).vmtx.is_null()
    {
        return;
    }
    let mut frequency: *mut u32 = ::core::ptr::null_mut::<u32>();
    frequency = __caryll_allocate_clean(
        (::core::mem::size_of::<u32>() as usize).wrapping_mul(4096 as usize),
        562 as ::core::ffi::c_ulong,
    ) as *mut u32;
    for j in 0..(*(*font).glyf).length as GlyphId {
        let vori: Pos = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j as isize)).verticalOrigin,
        ) as Pos;
        if vori >= 0 as ::core::ffi::c_int as Pos && vori < MAX_STAT_METRIC as Pos {
            let fresh0 = frequency.offset(vori as u16 as isize);
            *fresh0 = (*fresh0).wrapping_add(1 as u32);
        }
    }
    let mut maxfreq: u32 = 0 as u32;
    let mut maxj: GlyphId = 0 as GlyphId;
    for j_0 in 0..MAX_STAT_METRIC as GlyphId {
        if *frequency.offset(j_0 as isize) > maxfreq {
            maxfreq = *frequency.offset(j_0 as isize);
            maxj = j_0;
        }
    }
    let mut vorg: *mut VorgTable = ::core::ptr::null_mut::<VorgTable>();
    vorg = __caryll_allocate_clean(
        ::core::mem::size_of::<VorgTable>() as usize,
        578 as ::core::ffi::c_ulong,
    ) as *mut VorgTable;
    (*vorg).defaultVerticalOrigin = maxj as Pos;
    let mut nVertOrigs: GlyphId = 0 as GlyphId;
    for j_1 in 0..(*(*font).glyf).length as GlyphId {
        let vori_0: Pos = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j_1 as isize)).verticalOrigin,
        ) as Pos;
        if vori_0 != maxj as ::core::ffi::c_int as Pos {
            nVertOrigs = (nVertOrigs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
        }
    }
    (*vorg).numVertOriginYMetrics = nVertOrigs;
    (*vorg).entries = __caryll_allocate_clean(
        (::core::mem::size_of::<VorgEntry>() as usize).wrapping_mul(nVertOrigs as usize),
        587 as ::core::ffi::c_ulong,
    ) as *mut VorgEntry;
    let mut jj: GlyphId = 0 as GlyphId;
    for j_2 in 0..(*(*font).glyf).length as GlyphId {
        let vori_1: Pos = iVQ.getStill.expect("non-null function pointer")(
            (**(*(*font).glyf).items.offset(j_2 as isize)).verticalOrigin,
        ) as Pos;
        if vori_1 != maxj as ::core::ffi::c_int as Pos {
            (*(*vorg).entries.offset(jj as isize)).gid = j_2;
            (*(*vorg).entries.offset(jj as isize)).verticalOrigin = vori_1 as i16;
            jj = (jj as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
        }
    }
    free(frequency as *mut ::core::ffi::c_void);
    (*font).VORG = vorg;
}
unsafe extern "C" fn statLTSH(mut font: *mut Font) {
    if (*font).glyf.is_null() {
        return;
    }
    let mut needLTSH: bool = false;
    for j in 0..(*(*font).glyf).length as GlyphId {
        if (**(*(*font).glyf).items.offset(j as isize)).yPel as ::core::ffi::c_int
            > 1 as ::core::ffi::c_int
        {
            needLTSH = true;
        }
    }
    if !needLTSH {
        return;
    }
    let mut ltsh: *mut LtshTable = ::core::ptr::null_mut::<LtshTable>();
    ltsh = __caryll_allocate_clean(
        ::core::mem::size_of::<LtshTable>() as usize,
        610 as ::core::ffi::c_ulong,
    ) as *mut LtshTable;
    (*ltsh).numGlyphs = (*(*font).glyf).length as GlyphId;
    (*ltsh).yPels = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul((*ltsh).numGlyphs as usize),
        612 as ::core::ffi::c_ulong,
    ) as *mut u8;
    for j_0 in 0..(*(*font).glyf).length as GlyphId {
        *(*ltsh).yPels.offset(j_0 as isize) = (**(*(*font).glyf).items.offset(j_0 as isize)).yPel;
    }
    (*font).LTSH = ltsh;
}
pub unsafe extern "C" fn otfcc_statFont(
    mut font: *mut Font,
    mut options: *const Options,
) {
    if !(*font).glyf.is_null() && !(*font).head.is_null() {
        statGlyf(font, options);
        if !(*options).keep_modified_time {
            (*(*font).head).modified =
                2082844800 as i64 + time(::core::ptr::null_mut::<time_t>()) as i64;
        }
    }
    if !(*font).head.is_null() && !(*font).CFF_.is_null() {
        let mut cff: *mut CffTable = (*font).CFF_;
        if (*cff).fontBBoxBottom
            > (*(*font).head).yMin as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).fontBBoxBottom = (*(*font).head).yMin as ::core::ffi::c_double;
        }
        if (*cff).fontBBoxTop < (*(*font).head).yMax as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).fontBBoxTop = (*(*font).head).yMax as ::core::ffi::c_double;
        }
        if (*cff).fontBBoxLeft < (*(*font).head).xMin as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).fontBBoxLeft = (*(*font).head).xMin as ::core::ffi::c_double;
        }
        if (*cff).fontBBoxRight
            < (*(*font).head).xMax as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*cff).fontBBoxRight = (*(*font).head).xMax as ::core::ffi::c_double;
        }
        if !(*font).glyf.is_null() && (*cff).isCID {
            (*cff).cidCount = (*(*font).glyf).length as u32;
        }
        if (*cff).isCID {
            if !(*cff).fontMatrix.is_null() {
                iVQ.dispose.expect("non-null function pointer")(&raw mut (*(*cff).fontMatrix).x);
                iVQ.dispose.expect("non-null function pointer")(&raw mut (*(*cff).fontMatrix).y);
                free((*cff).fontMatrix as *mut ::core::ffi::c_void);
                (*cff).fontMatrix = ::core::ptr::null_mut::<CffFontMatrix>();
            }
            for j in 0..(*cff).fdArrayCount {
                let fd: *mut CffTable = *(*cff).fdArray.offset(j as isize);
                if !(*fd).fontMatrix.is_null() {
                    iVQ.dispose.expect("non-null function pointer")(&raw mut (*(*fd).fontMatrix).x);
                    iVQ.dispose.expect("non-null function pointer")(&raw mut (*(*fd).fontMatrix).y);
                    free((*fd).fontMatrix as *mut ::core::ffi::c_void);
                    (*fd).fontMatrix = ::core::ptr::null_mut::<CffFontMatrix>();
                }
                if (*(*font).head).unitsPerEm as ::core::ffi::c_int == 1000 as ::core::ffi::c_int {
                    (*fd).fontMatrix = ::core::ptr::null_mut::<CffFontMatrix>();
                } else {
                    (*fd).fontMatrix = __caryll_allocate_clean(
                        ::core::mem::size_of::<CffFontMatrix>() as usize,
                        651 as ::core::ffi::c_ulong,
                    ) as *mut CffFontMatrix;
                    (*(*fd).fontMatrix).a = (1.0f64
                        / (*(*font).head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double)
                        as Scale;
                    (*(*fd).fontMatrix).b = 0.0f64 as Scale;
                    (*(*fd).fontMatrix).c = 0.0f64 as Scale;
                    (*(*fd).fontMatrix).d = (1.0f64
                        / (*(*font).head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double)
                        as Scale;
                    (*(*fd).fontMatrix).x = (
                        iVQ.neutral.expect("non-null function pointer"))();
                    (*(*fd).fontMatrix).y = (
                        iVQ.neutral.expect("non-null function pointer"))();
                }
            }
        } else if (*(*font).head).unitsPerEm as ::core::ffi::c_int == 1000 as ::core::ffi::c_int {
            (*cff).fontMatrix = ::core::ptr::null_mut::<CffFontMatrix>();
        } else {
            (*cff).fontMatrix = __caryll_allocate_clean(
                ::core::mem::size_of::<CffFontMatrix>() as usize,
                664 as ::core::ffi::c_ulong,
            ) as *mut CffFontMatrix;
            (*(*cff).fontMatrix).a = (1.0f64
                / (*(*font).head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double)
                as Scale;
            (*(*cff).fontMatrix).b = 0.0f64 as Scale;
            (*(*cff).fontMatrix).c = 0.0f64 as Scale;
            (*(*cff).fontMatrix).d = (1.0f64
                / (*(*font).head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double)
                as Scale;
            (*(*cff).fontMatrix).x = (
                iVQ.neutral.expect("non-null function pointer"))();
            (*(*cff).fontMatrix).y = (
                iVQ.neutral.expect("non-null function pointer"))();
        }
        statCFFWidths(font);
    }
    if !(*font).glyf.is_null() && !(*font).maxp.is_null() {
        (*(*font).maxp).numGlyphs = (*(*font).glyf).length as u16;
    }
    if !(*font).glyf.is_null() && !(*font).post.is_null() {
        (*(*font).post).maxMemType42 = (*(*font).glyf).length as u32;
    }
    if !(*font).glyf.is_null()
        && !(*font).maxp.is_null()
        && (*(*font).maxp).version == 0x10000 as F16Dot16
    {
        statMaxp(font);
        if !(*font).fpgm.is_null()
            && (*(*font).fpgm).length > (*(*font).maxp).maxSizeOfInstructions as u32
        {
            (*(*font).maxp).maxSizeOfInstructions = (*(*font).fpgm).length as u16;
        }
        if !(*font).prep.is_null()
            && (*(*font).prep).length > (*(*font).maxp).maxSizeOfInstructions as u32
        {
            (*(*font).maxp).maxSizeOfInstructions = (*(*font).prep).length as u16;
        }
    }
    if !(*font).OS_2.is_null() && !(*font).cmap.is_null() && !(*font).glyf.is_null() {
        statOS_2(font, options);
    }
    if (*font).subtype == FONTTYPE_TTF {
        if !(*font).maxp.is_null() {
            (*(*font).maxp).version = 0x10000 as ::core::ffi::c_int as F16Dot16;
        }
    } else if !(*font).maxp.is_null() {
        (*(*font).maxp).version = 0x5000 as ::core::ffi::c_int as F16Dot16;
    }
    if !(*font).glyf.is_null() && !(*font).hhea.is_null() {
        statHmtx(font, options);
    }
    if !(*font).glyf.is_null() && !(*font).vhea.is_null() {
        statVmtx(font, options);
        statVORG(font);
    }
    statLTSH(font);
}
pub unsafe extern "C" fn otfcc_unstatFont(
    mut font: *mut Font,
    mut _options: *const Options,
) {
    otfcc_iFont.deleteTable.expect("non-null function pointer")(font, 1751412088i32 as u32);
    otfcc_iFont.deleteTable.expect("non-null function pointer")(font, 1752003704i32 as u32);
    otfcc_iFont.deleteTable.expect("non-null function pointer")(font, 1448038983i32 as u32);
    otfcc_iFont.deleteTable.expect("non-null function pointer")(font, 1986884728i32 as u32);
    otfcc_iFont.deleteTable.expect("non-null function pointer")(font, 1280594760i32 as u32);
}
pub const FLT_MAX: ::core::ffi::c_float = __FLT_MAX__;
pub const __FLT_MAX__: ::core::ffi::c_float = 3.40282347e+38f32;
