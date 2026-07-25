use libc::{free};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn buflen(buf: *mut caryll_Buffer) -> usize;
    fn bufclear(buf: *mut caryll_Buffer);
    fn bufwrite8(buf: *mut caryll_Buffer, byte: u8);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn bufwrite_bytes(buf: *mut caryll_Buffer, size: usize, str: *const u8);
    fn bufwrite_buf(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    fn buflongalign(buf: *mut caryll_Buffer);
    fn otfcc_to_f2dot14(x: ::core::ffi::c_double) -> i16;
    static iVQ: __caryll_vectorinterface_VQ;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::handle::{otfcc_Handle, otfcc_GlyphHandle};

use crate::support::binio::{pos_to_u16};
use crate::support::alloc::{__caryll_allocate_clean};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{f16dot16, glyphid_t, pos_t, scale_t, shapeid_t};
use crate::vendor::sds::{sds};
pub type otfcc_FDHandle = otfcc_Handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_AxisSpan {
    pub start: pos_t,
    pub peak: pos_t,
    pub end: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_Region {
    pub dimensions: shapeid_t,
    pub spans: [vq_AxisSpan; 0],
}
pub type VQSegType = ::core::ffi::c_uint;
pub const VQ_DELTA: VQSegType = 1;
pub const VQ_STILL: VQSegType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_Segment {
    pub type_0: VQSegType,
    pub val: vq_SegmentValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union vq_SegmentValue {
    pub still: pos_t,
    pub delta: vq_SegmentDelta,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_SegmentDelta {
    pub quantity: pos_t,
    pub touched: bool,
    pub region: *const vq_Region,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_SegList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut vq_Segment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VQ {
    pub kernel: pos_t,
    pub shift: vq_SegList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_VQ {
    pub init: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VQ, *const VQ) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VQ, *mut VQ) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> VQ>,
    pub dup: Option<unsafe extern "C" fn(VQ) -> VQ>,
    pub neutral: Option<unsafe extern "C" fn() -> VQ>,
    pub plus: Option<unsafe extern "C" fn(VQ, VQ) -> VQ>,
    pub inplacePlus: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub inplaceNegate: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub negate: Option<unsafe extern "C" fn(VQ) -> VQ>,
    pub inplaceMinus: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub minus: Option<unsafe extern "C" fn(VQ, VQ) -> VQ>,
    pub inplaceScale: Option<unsafe extern "C" fn(*mut VQ, scale_t) -> ()>,
    pub inplacePlusScale: Option<unsafe extern "C" fn(*mut VQ, scale_t, VQ) -> ()>,
    pub scale: Option<unsafe extern "C" fn(VQ, scale_t) -> VQ>,
    pub equal: Option<unsafe extern "C" fn(VQ, VQ) -> bool>,
    pub compare: Option<unsafe extern "C" fn(VQ, VQ) -> ::core::ffi::c_int>,
    pub compareRef: Option<unsafe extern "C" fn(*const VQ, *const VQ) -> ::core::ffi::c_int>,
    pub show: Option<unsafe extern "C" fn(VQ) -> ()>,
    pub getStill: Option<unsafe extern "C" fn(VQ) -> pos_t>,
    pub createStill: Option<unsafe extern "C" fn(pos_t) -> VQ>,
    pub isStill: Option<unsafe extern "C" fn(VQ) -> bool>,
    pub isZero: Option<unsafe extern "C" fn(VQ, pos_t) -> bool>,
    pub pointLinearTfm: Option<unsafe extern "C" fn(VQ, pos_t, VQ, pos_t, VQ) -> VQ>,
    pub addDelta: Option<unsafe extern "C" fn(*mut VQ, bool, *const vq_Region, pos_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_head {
    pub version: f16dot16,
    pub fontRevision: u32,
    pub checkSumAdjustment: u32,
    pub magicNumber: u32,
    pub flags: u16,
    pub unitsPerEm: u16,
    pub created: i64,
    pub modified: i64,
    pub xMin: i16,
    pub yMin: i16,
    pub xMax: i16,
    pub yMax: i16,
    pub macStyle: u16,
    pub lowestRecPPEM: u16,
    pub fontDirectoryHint: i16,
    pub indexToLocFormat: i16,
    pub glyphDataFormat: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Point {
    pub x: VQ,
    pub y: VQ,
    pub onCurve: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Contour {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_Point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ContourList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_Contour,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PostscriptStemDef {
    pub position: pos_t,
    pub width: pos_t,
    pub map: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_StemDefList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_PostscriptStemDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PostscriptHintMask {
    pub pointsBefore: u16,
    pub contoursBefore: u16,
    pub maskH: [bool; 256],
    pub maskV: [bool; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_MaskList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_PostscriptHintMask,
}
pub type RefAnchorStatus = ::core::ffi::c_uint;
pub const REF_ANCHOR_CONSOLIDATING_XY: RefAnchorStatus = 5;
pub const REF_ANCHOR_CONSOLIDATING_ANCHOR: RefAnchorStatus = 4;
pub const REF_ANCHOR_CONSOLIDATED: RefAnchorStatus = 3;
pub const REF_ANCHOR_XY: RefAnchorStatus = 2;
pub const REF_ANCHOR_ANCHOR: RefAnchorStatus = 1;
pub const REF_XY: RefAnchorStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ComponentReference {
    pub x: VQ,
    pub y: VQ,
    pub roundToGrid: bool,
    pub useMyMetrics: bool,
    pub glyph: otfcc_GlyphHandle,
    pub a: scale_t,
    pub b: scale_t,
    pub c: scale_t,
    pub d: scale_t,
    pub isAnchored: RefAnchorStatus,
    pub inner: shapeid_t,
    pub outer: shapeid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ReferenceList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_ComponentReference,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_GlyphStat {
    pub xMin: pos_t,
    pub xMax: pos_t,
    pub yMin: pos_t,
    pub yMax: pos_t,
    pub nestDepth: u16,
    pub nPoints: u16,
    pub nContours: u16,
    pub nCompositePoints: u16,
    pub nCompositeContours: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Glyph {
    pub name: sds,
    pub horizontalOrigin: VQ,
    pub advanceWidth: VQ,
    pub verticalOrigin: VQ,
    pub advanceHeight: VQ,
    pub contours: glyf_ContourList,
    pub references: glyf_ReferenceList,
    pub stemH: glyf_StemDefList,
    pub stemV: glyf_StemDefList,
    pub hintMasks: glyf_MaskList,
    pub contourMasks: glyf_MaskList,
    pub instructionsLength: u16,
    pub instructions: *mut u8,
    pub yPel: u8,
    pub fdSelect: otfcc_FDHandle,
    pub cid: glyphid_t,
    pub stat: glyf_GlyphStat,
}
pub type glyf_GlyphPtr = *mut glyf_Glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_glyf {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_GlyphPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_GlyfAndLocaBuffers {
    pub glyf: *mut caryll_Buffer,
    pub loca: *mut caryll_Buffer,
}
pub const WE_HAVE_A_TWO_BY_TWO: glyf_ComponentFlags = 128;
pub const WE_HAVE_INSTRUCTIONS: glyf_ComponentFlags = 256;
pub const MORE_COMPONENTS: glyf_ComponentFlags = 32;
pub const WE_HAVE_AN_X_AND_Y_SCALE: glyf_ComponentFlags = 64;
pub const WE_HAVE_A_SCALE: glyf_ComponentFlags = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union glyf_ComponentArg {
    pub pointid: u16,
    pub coord: i16,
}
pub const ARG_1_AND_2_ARE_WORDS: glyf_ComponentFlags = 1;
pub const UNSCALED_COMPONENT_OFFSET: glyf_ComponentFlags = 4096;
pub const USE_MY_METRICS: glyf_ComponentFlags = 512;
pub const ROUND_XY_TO_GRID: glyf_ComponentFlags = 4;
pub const ARGS_ARE_XY_VALUES: glyf_ComponentFlags = 2;
pub const GLYF_FLAG_REPEAT: glyf_PointFlags = 8;
pub const GLYF_FLAG_ON_CURVE: glyf_PointFlags = 1;
pub const MASK_ON_CURVE: glyf_OnCurveMask = 1;
pub const GLYF_FLAG_POSITIVE_Y: glyf_PointFlags = 32;
pub const GLYF_FLAG_Y_SHORT: glyf_PointFlags = 4;
pub const GLYF_FLAG_SAME_Y: glyf_PointFlags = 32;
pub const GLYF_FLAG_POSITIVE_X: glyf_PointFlags = 16;
pub const GLYF_FLAG_X_SHORT: glyf_PointFlags = 2;
pub const GLYF_FLAG_SAME_X: glyf_PointFlags = 16;
pub type glyf_PointFlags = ::core::ffi::c_uint;
pub type glyf_ComponentFlags = ::core::ffi::c_uint;
pub const SCALED_COMPONENT_OFFSET: glyf_ComponentFlags = 2048;
pub const OVERLAP_COMPOUND: glyf_ComponentFlags = 1024;
pub type glyf_OnCurveMask = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn shrinkFlags(mut flags: *mut caryll_Buffer) -> *mut caryll_Buffer {
    if buflen(flags) == 0 {
        return flags;
    }
    let mut shrunk: *mut caryll_Buffer = bufnew();
    bufwrite8(
        shrunk,
        *(*flags).data.offset(0 as ::core::ffi::c_int as isize),
    );
    let mut repeating: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: usize = 1 as usize;
    while j < buflen(flags) {
        if *(*flags).data.offset(j as isize) as ::core::ffi::c_int
            == *(*flags).data.offset(j.wrapping_sub(1 as usize) as isize) as ::core::ffi::c_int
        {
            if repeating != 0 && repeating < 0xfe as ::core::ffi::c_int {
                let ref mut fresh0 = *(*shrunk)
                    .data
                    .offset((*shrunk).cursor.wrapping_sub(1 as usize) as isize);
                *fresh0 = (*fresh0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
                repeating += 1 as ::core::ffi::c_int;
            } else if repeating == 0 as ::core::ffi::c_int {
                let ref mut fresh1 = *(*shrunk)
                    .data
                    .offset((*shrunk).cursor.wrapping_sub(1 as usize) as isize);
                *fresh1 = (*fresh1 as ::core::ffi::c_int | GLYF_FLAG_REPEAT as ::core::ffi::c_int)
                    as u8;
                bufwrite8(shrunk, 1 as u8);
                repeating += 1 as ::core::ffi::c_int;
            } else {
                repeating = 0 as ::core::ffi::c_int;
                bufwrite8(shrunk, *(*flags).data.offset(j as isize));
            }
        } else {
            repeating = 0 as ::core::ffi::c_int;
            bufwrite8(shrunk, *(*flags).data.offset(j as isize));
        }
        j = j.wrapping_add(1);
    }
    buffree(flags);
    return shrunk;
}
pub const EPSILON: ::core::ffi::c_double = 1e-5f64;
unsafe extern "C" fn glyf_build_simple(mut g: *const glyf_Glyph, mut gbuf: *mut caryll_Buffer) {
    let mut flags: *mut caryll_Buffer = bufnew();
    let mut xs: *mut caryll_Buffer = bufnew();
    let mut ys: *mut caryll_Buffer = bufnew();
    bufwrite16b(gbuf, (*g).contours.length as u16);
    bufwrite16b(gbuf, pos_to_u16((*g).stat.xMin));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.yMin));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.xMax));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.yMax));
    let mut ptid: shapeid_t = 0 as shapeid_t;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as usize) < (*g).contours.length {
        ptid = (ptid as usize).wrapping_add((*(*g).contours.items.offset(j as isize)).length)
            as shapeid_t as shapeid_t;
        bufwrite16b(
            gbuf,
            (ptid as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u16,
        );
        j = j.wrapping_add(1);
    }
    bufwrite16b(gbuf, (*g).instructionsLength);
    if !(*g).instructions.is_null() {
        bufwrite_bytes(gbuf, (*g).instructionsLength as usize, (*g).instructions);
    }
    bufclear(flags);
    bufclear(xs);
    bufclear(ys);
    let mut cx: i32 = 0 as i32;
    let mut cy: i32 = 0 as i32;
    let mut cj: shapeid_t = 0 as shapeid_t;
    while (cj as usize) < (*g).contours.length {
        let mut k: shapeid_t = 0 as shapeid_t;
        while (k as usize) < (*(*g).contours.items.offset(cj as isize)).length {
            let mut p: *mut glyf_Point = (*(*g).contours.items.offset(cj as isize))
                .items
                .offset(k as isize) as *mut glyf_Point;
            let mut flag: u8 =
                (if (*p).onCurve as ::core::ffi::c_int & MASK_ON_CURVE as ::core::ffi::c_int != 0 {
                    GLYF_FLAG_ON_CURVE as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as u8;
            let mut px: i32 =
                round(iVQ.getStill.expect("non-null function pointer")((*p).x)
                    as ::core::ffi::c_double) as i32;
            let mut py: i32 =
                round(iVQ.getStill.expect("non-null function pointer")((*p).y)
                    as ::core::ffi::c_double) as i32;
            let mut dx: i16 = (px - cx) as i16;
            let mut dy: i16 = (py - cy) as i16;
            if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                flag = (flag as ::core::ffi::c_int | GLYF_FLAG_SAME_X as ::core::ffi::c_int)
                    as u8;
            } else if dx as ::core::ffi::c_int >= -(0xff as ::core::ffi::c_int)
                && dx as ::core::ffi::c_int <= 0xff as ::core::ffi::c_int
            {
                flag = (flag as ::core::ffi::c_int | GLYF_FLAG_X_SHORT as ::core::ffi::c_int)
                    as u8;
                if dx as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    flag = (flag as ::core::ffi::c_int | GLYF_FLAG_POSITIVE_X as ::core::ffi::c_int)
                        as u8;
                    bufwrite8(xs, dx as u8);
                } else {
                    bufwrite8(xs, -(dx as ::core::ffi::c_int) as u8);
                }
            } else {
                bufwrite16b(xs, dx as u16);
            }
            if dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                flag = (flag as ::core::ffi::c_int | GLYF_FLAG_SAME_Y as ::core::ffi::c_int)
                    as u8;
            } else if dy as ::core::ffi::c_int >= -(0xff as ::core::ffi::c_int)
                && dy as ::core::ffi::c_int <= 0xff as ::core::ffi::c_int
            {
                flag = (flag as ::core::ffi::c_int | GLYF_FLAG_Y_SHORT as ::core::ffi::c_int)
                    as u8;
                if dy as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    flag = (flag as ::core::ffi::c_int | GLYF_FLAG_POSITIVE_Y as ::core::ffi::c_int)
                        as u8;
                    bufwrite8(ys, dy as u8);
                } else {
                    bufwrite8(ys, -(dy as ::core::ffi::c_int) as u8);
                }
            } else {
                bufwrite16b(ys, dy as u16);
            }
            bufwrite8(flags, flag);
            cx = px;
            cy = py;
            k = k.wrapping_add(1);
        }
        cj = cj.wrapping_add(1);
    }
    flags = shrinkFlags(flags);
    bufwrite_buf(gbuf, flags);
    bufwrite_buf(gbuf, xs);
    bufwrite_buf(gbuf, ys);
    buffree(flags);
    buffree(xs);
    buffree(ys);
}
unsafe extern "C" fn glyf_build_composite(mut g: *const glyf_Glyph, mut gbuf: *mut caryll_Buffer) {
    bufwrite16b(gbuf, -(1 as ::core::ffi::c_int) as u16);
    bufwrite16b(gbuf, pos_to_u16((*g).stat.xMin));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.yMin));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.xMax));
    bufwrite16b(gbuf, pos_to_u16((*g).stat.yMax));
    let mut rj: shapeid_t = 0 as shapeid_t;
    while (rj as usize) < (*g).references.length {
        let mut r: *mut glyf_ComponentReference =
            (*g).references.items.offset(rj as isize) as *mut glyf_ComponentReference;
        let mut flags: u16 =
            (if (rj as usize) < (*g).references.length.wrapping_sub(1 as usize) {
                MORE_COMPONENTS as ::core::ffi::c_int
            } else if (*g).instructionsLength as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                WE_HAVE_INSTRUCTIONS as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as u16;
        let mut outputAnchor: bool = (*r).isAnchored as ::core::ffi::c_uint
            == REF_ANCHOR_CONSOLIDATED as ::core::ffi::c_int as ::core::ffi::c_uint;
        let mut arg1: glyf_ComponentArg = glyf_ComponentArg { pointid: 0 };
        let mut arg2: glyf_ComponentArg = glyf_ComponentArg { pointid: 0 };
        if outputAnchor {
            arg1.pointid = (*r).outer as u16;
            arg2.pointid = (*r).inner as u16;
            if !((arg1.pointid as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int
                && (arg2.pointid as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int)
            {
                flags = (flags as ::core::ffi::c_int | ARG_1_AND_2_ARE_WORDS as ::core::ffi::c_int)
                    as u16;
            }
        } else {
            flags = (flags as ::core::ffi::c_int | ARGS_ARE_XY_VALUES as ::core::ffi::c_int)
                as u16;
            arg1.coord = iVQ.getStill.expect("non-null function pointer")((*r).x) as i16;
            arg2.coord = iVQ.getStill.expect("non-null function pointer")((*r).y) as i16;
            if !((arg1.coord as ::core::ffi::c_int) < 128 as ::core::ffi::c_int
                && arg1.coord as ::core::ffi::c_int >= -(128 as ::core::ffi::c_int)
                && (arg2.coord as ::core::ffi::c_int) < 128 as ::core::ffi::c_int
                && arg2.coord as ::core::ffi::c_int >= -(128 as ::core::ffi::c_int))
            {
                flags = (flags as ::core::ffi::c_int | ARG_1_AND_2_ARE_WORDS as ::core::ffi::c_int)
                    as u16;
            }
        }
        if fabs((*r).b as ::core::ffi::c_double) > EPSILON
            || fabs((*r).c as ::core::ffi::c_double) > EPSILON
        {
            flags = (flags as ::core::ffi::c_int | WE_HAVE_A_TWO_BY_TWO as ::core::ffi::c_int)
                as u16;
        } else if fabs(
            (*r).a as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) > EPSILON
            || fabs(
                (*r).d as ::core::ffi::c_double - 1 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) > EPSILON
        {
            if fabs((*r).a as ::core::ffi::c_double - (*r).d as ::core::ffi::c_double) > EPSILON {
                flags = (flags as ::core::ffi::c_int
                    | WE_HAVE_AN_X_AND_Y_SCALE as ::core::ffi::c_int)
                    as u16;
            } else {
                flags = (flags as ::core::ffi::c_int | WE_HAVE_A_SCALE as ::core::ffi::c_int)
                    as u16;
            }
        }
        if (*r).roundToGrid {
            flags =
                (flags as ::core::ffi::c_int | ROUND_XY_TO_GRID as ::core::ffi::c_int) as u16;
        }
        if (*r).useMyMetrics {
            flags =
                (flags as ::core::ffi::c_int | USE_MY_METRICS as ::core::ffi::c_int) as u16;
        }
        flags = (flags as ::core::ffi::c_int | UNSCALED_COMPONENT_OFFSET as ::core::ffi::c_int)
            as u16;
        bufwrite16b(gbuf, flags);
        bufwrite16b(gbuf, (*r).glyph.index as u16);
        if flags as ::core::ffi::c_int & ARG_1_AND_2_ARE_WORDS as ::core::ffi::c_int != 0 {
            bufwrite16b(gbuf, arg1.pointid);
            bufwrite16b(gbuf, arg2.pointid);
        } else {
            bufwrite8(gbuf, arg1.pointid as u8);
            bufwrite8(gbuf, arg2.pointid as u8);
        }
        if flags as ::core::ffi::c_int & WE_HAVE_A_SCALE as ::core::ffi::c_int != 0 {
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u16,
            );
        } else if flags as ::core::ffi::c_int & WE_HAVE_AN_X_AND_Y_SCALE as ::core::ffi::c_int != 0
        {
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u16,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).d as ::core::ffi::c_double) as u16,
            );
        } else if flags as ::core::ffi::c_int & WE_HAVE_A_TWO_BY_TWO as ::core::ffi::c_int != 0 {
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as u16,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).b as ::core::ffi::c_double) as u16,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).c as ::core::ffi::c_double) as u16,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).d as ::core::ffi::c_double) as u16,
            );
        }
        rj = rj.wrapping_add(1);
    }
    if (*g).instructionsLength != 0 {
        bufwrite16b(gbuf, (*g).instructionsLength);
        if !(*g).instructions.is_null() {
            bufwrite_bytes(gbuf, (*g).instructionsLength as usize, (*g).instructions);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildGlyf(
    mut table: *const table_glyf,
    mut head: *mut table_head,
    mut _options: *const otfcc_Options,
) -> table_GlyfAndLocaBuffers {
    let mut bufglyf: *mut caryll_Buffer = bufnew();
    let mut bufloca: *mut caryll_Buffer = bufnew();
    if !table.is_null() && !head.is_null() {
        let mut gbuf: *mut caryll_Buffer = bufnew();
        let mut loca: *mut u32 = ::core::ptr::null_mut::<u32>();
        loca = __caryll_allocate_clean(
            (::core::mem::size_of::<u32>() as usize)
                .wrapping_mul((*table).length.wrapping_add(1 as usize)),
            189 as ::core::ffi::c_ulong,
        ) as *mut u32;
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as usize) < (*table).length {
            *loca.offset(j as isize) = (*bufglyf).cursor as u32;
            let mut g: *mut glyf_Glyph = *(*table).items.offset(j as isize) as *mut glyf_Glyph;
            bufclear(gbuf);
            if (*g).contours.length > 0 as usize {
                glyf_build_simple(g, gbuf);
            } else if (*g).references.length > 0 as usize {
                glyf_build_composite(g, gbuf);
            }
            buflongalign(gbuf);
            bufwrite_buf(bufglyf, gbuf);
            j = j.wrapping_add(1);
        }
        *loca.offset((*table).length as isize) = (*bufglyf).cursor as u32;
        if (*bufglyf).cursor >= 0x20000 as ::core::ffi::c_int as usize {
            (*head).indexToLocFormat = 1 as i16;
        } else {
            (*head).indexToLocFormat = 0 as i16;
        }
        let mut j_0: u32 = 0 as u32;
        while j_0 as usize <= (*table).length {
            if (*head).indexToLocFormat != 0 {
                bufwrite32b(bufloca, *loca.offset(j_0 as isize));
            } else {
                bufwrite16b(
                    bufloca,
                    (*loca.offset(j_0 as isize) >> 1 as ::core::ffi::c_int) as u16,
                );
            }
            j_0 = j_0.wrapping_add(1);
        }
        buffree(gbuf);
        free(loca as *mut ::core::ffi::c_void);
        loca = ::core::ptr::null_mut::<u32>();
    }
    let mut pair: table_GlyfAndLocaBuffers = table_GlyfAndLocaBuffers {
        glyf: bufglyf,
        loca: bufloca,
    };
    return pair;
}
