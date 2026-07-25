extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn buflen(buf: *mut caryll_Buffer) -> size_t;
    fn bufclear(buf: *mut caryll_Buffer);
    fn bufwrite8(buf: *mut caryll_Buffer, byte: uint8_t);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: uint32_t);
    fn bufwrite_bytes(buf: *mut caryll_Buffer, size: size_t, str: *const uint8_t);
    fn bufwrite_buf(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    fn buflongalign(buf: *mut caryll_Buffer);
    fn otfcc_to_f2dot14(x: ::core::ffi::c_double) -> int16_t;
    static iVQ: __caryll_vectorinterface_VQ;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::handle::{otfcc_Handle, otfcc_GlyphHandle};
use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type f16dot16 = int32_t;
pub type glyphid_t = uint16_t;
pub type shapeid_t = uint16_t;
pub type pos_t = ::core::ffi::c_double;
pub type scale_t = ::core::ffi::c_double;
pub type otfcc_FDHandle = otfcc_Handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILoggerTarget {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()>,
}
pub type otfcc_LoggerType = ::core::ffi::c_uint;
pub const log_type_progress: otfcc_LoggerType = 3;
pub const log_type_info: otfcc_LoggerType = 2;
pub const log_type_warning: otfcc_LoggerType = 1;
pub const log_type_error: otfcc_LoggerType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILogger {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub indent: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub indentSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub start: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub startSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub log: Option<
        unsafe extern "C" fn(
            *mut otfcc_ILogger,
            uint8_t,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t) -> ()>,
    pub getTarget: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Options {
    pub debug_wait_on_start: bool,
    pub ignore_glyph_order: bool,
    pub ignore_hints: bool,
    pub has_vertical_metrics: bool,
    pub export_fdselect: bool,
    pub keep_average_char_width: bool,
    pub keep_unicode_ranges: bool,
    pub short_post: bool,
    pub dummy_DSIG: bool,
    pub keep_modified_time: bool,
    pub instr_as_bytes: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub cff_short_vmtx: bool,
    pub merge_lookups: bool,
    pub merge_features: bool,
    pub force_cid: bool,
    pub cff_rollCharString: bool,
    pub cff_doSubroutinize: bool,
    pub stub_cmap4: bool,
    pub decimal_cmap: bool,
    pub name_glyphs_by_hash: bool,
    pub name_glyphs_by_gid: bool,
    pub glyph_name_prefix: *mut ::core::ffi::c_char,
    pub logger: *mut otfcc_ILogger,
}
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
    pub val: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub still: pos_t,
    pub delta: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub quantity: pos_t,
    pub touched: bool,
    pub region: *const vq_Region,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_SegList {
    pub length: size_t,
    pub capacity: size_t,
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
    pub fontRevision: uint32_t,
    pub checkSumAdjustment: uint32_t,
    pub magicNumber: uint32_t,
    pub flags: uint16_t,
    pub unitsPerEm: uint16_t,
    pub created: int64_t,
    pub modified: int64_t,
    pub xMin: int16_t,
    pub yMin: int16_t,
    pub xMax: int16_t,
    pub yMax: int16_t,
    pub macStyle: uint16_t,
    pub lowestRecPPEM: uint16_t,
    pub fontDirectoryHint: int16_t,
    pub indexToLocFormat: int16_t,
    pub glyphDataFormat: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Point {
    pub x: VQ,
    pub y: VQ,
    pub onCurve: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Contour {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_Point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ContourList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_Contour,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PostscriptStemDef {
    pub position: pos_t,
    pub width: pos_t,
    pub map: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_StemDefList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_PostscriptStemDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PostscriptHintMask {
    pub pointsBefore: uint16_t,
    pub contoursBefore: uint16_t,
    pub maskH: [bool; 256],
    pub maskV: [bool; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_MaskList {
    pub length: size_t,
    pub capacity: size_t,
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
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_ComponentReference,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_GlyphStat {
    pub xMin: pos_t,
    pub xMax: pos_t,
    pub yMin: pos_t,
    pub yMax: pos_t,
    pub nestDepth: uint16_t,
    pub nPoints: uint16_t,
    pub nContours: uint16_t,
    pub nCompositePoints: uint16_t,
    pub nCompositeContours: uint16_t,
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
    pub instructionsLength: uint16_t,
    pub instructions: *mut uint8_t,
    pub yPel: uint8_t,
    pub fdSelect: otfcc_FDHandle,
    pub cid: glyphid_t,
    pub stat: glyf_GlyphStat,
}
pub type glyf_GlyphPtr = *mut glyf_Glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_glyf {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_GlyphPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_GlyfAndLocaBuffers {
    pub glyf: *mut caryll_Buffer,
    pub loca: *mut caryll_Buffer,
}
pub const WE_HAVE_A_TWO_BY_TWO: C2RustUnnamed_3 = 128;
pub const WE_HAVE_INSTRUCTIONS: C2RustUnnamed_3 = 256;
pub const MORE_COMPONENTS: C2RustUnnamed_3 = 32;
pub const WE_HAVE_AN_X_AND_Y_SCALE: C2RustUnnamed_3 = 64;
pub const WE_HAVE_A_SCALE: C2RustUnnamed_3 = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub pointid: uint16_t,
    pub coord: int16_t,
}
pub const ARG_1_AND_2_ARE_WORDS: C2RustUnnamed_3 = 1;
pub const UNSCALED_COMPONENT_OFFSET: C2RustUnnamed_3 = 4096;
pub const USE_MY_METRICS: C2RustUnnamed_3 = 512;
pub const ROUND_XY_TO_GRID: C2RustUnnamed_3 = 4;
pub const ARGS_ARE_XY_VALUES: C2RustUnnamed_3 = 2;
pub const GLYF_FLAG_REPEAT: C2RustUnnamed_2 = 8;
pub const GLYF_FLAG_ON_CURVE: C2RustUnnamed_2 = 1;
pub const MASK_ON_CURVE: C2RustUnnamed_4 = 1;
pub const GLYF_FLAG_POSITIVE_Y: C2RustUnnamed_2 = 32;
pub const GLYF_FLAG_Y_SHORT: C2RustUnnamed_2 = 4;
pub const GLYF_FLAG_SAME_Y: C2RustUnnamed_2 = 32;
pub const GLYF_FLAG_POSITIVE_X: C2RustUnnamed_2 = 16;
pub const GLYF_FLAG_X_SHORT: C2RustUnnamed_2 = 2;
pub const GLYF_FLAG_SAME_X: C2RustUnnamed_2 = 16;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const SCALED_COMPONENT_OFFSET: C2RustUnnamed_3 = 2048;
pub const OVERLAP_COMPOUND: C2RustUnnamed_3 = 1024;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
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
    let mut j: size_t = 1 as size_t;
    while j < buflen(flags) {
        if *(*flags).data.offset(j as isize) as ::core::ffi::c_int
            == *(*flags).data.offset(j.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        {
            if repeating != 0 && repeating < 0xfe as ::core::ffi::c_int {
                let ref mut fresh0 = *(*shrunk)
                    .data
                    .offset((*shrunk).cursor.wrapping_sub(1 as size_t) as isize);
                *fresh0 = (*fresh0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
                repeating += 1 as ::core::ffi::c_int;
            } else if repeating == 0 as ::core::ffi::c_int {
                let ref mut fresh1 = *(*shrunk)
                    .data
                    .offset((*shrunk).cursor.wrapping_sub(1 as size_t) as isize);
                *fresh1 = (*fresh1 as ::core::ffi::c_int | GLYF_FLAG_REPEAT as ::core::ffi::c_int)
                    as uint8_t;
                bufwrite8(shrunk, 1 as uint8_t);
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
    bufwrite16b(gbuf, (*g).contours.length as uint16_t);
    bufwrite16b(gbuf, (*g).stat.xMin as int16_t as uint16_t);
    bufwrite16b(gbuf, (*g).stat.yMin as int16_t as uint16_t);
    bufwrite16b(gbuf, (*g).stat.xMax as int16_t as uint16_t);
    bufwrite16b(gbuf, (*g).stat.yMax as int16_t as uint16_t);
    let mut ptid: shapeid_t = 0 as shapeid_t;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as size_t) < (*g).contours.length {
        ptid = (ptid as size_t).wrapping_add((*(*g).contours.items.offset(j as isize)).length)
            as shapeid_t as shapeid_t;
        bufwrite16b(
            gbuf,
            (ptid as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint16_t,
        );
        j = j.wrapping_add(1);
    }
    bufwrite16b(gbuf, (*g).instructionsLength);
    if !(*g).instructions.is_null() {
        bufwrite_bytes(gbuf, (*g).instructionsLength as size_t, (*g).instructions);
    }
    bufclear(flags);
    bufclear(xs);
    bufclear(ys);
    let mut cx: int32_t = 0 as int32_t;
    let mut cy: int32_t = 0 as int32_t;
    let mut cj: shapeid_t = 0 as shapeid_t;
    while (cj as size_t) < (*g).contours.length {
        let mut k: shapeid_t = 0 as shapeid_t;
        while (k as size_t) < (*(*g).contours.items.offset(cj as isize)).length {
            let mut p: *mut glyf_Point = (*(*g).contours.items.offset(cj as isize))
                .items
                .offset(k as isize) as *mut glyf_Point;
            let mut flag: uint8_t =
                (if (*p).onCurve as ::core::ffi::c_int & MASK_ON_CURVE as ::core::ffi::c_int != 0 {
                    GLYF_FLAG_ON_CURVE as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t;
            let mut px: int32_t =
                round(iVQ.getStill.expect("non-null function pointer")((*p).x)
                    as ::core::ffi::c_double) as int32_t;
            let mut py: int32_t =
                round(iVQ.getStill.expect("non-null function pointer")((*p).y)
                    as ::core::ffi::c_double) as int32_t;
            let mut dx: int16_t = (px - cx) as int16_t;
            let mut dy: int16_t = (py - cy) as int16_t;
            if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                flag = (flag as ::core::ffi::c_int | GLYF_FLAG_SAME_X as ::core::ffi::c_int)
                    as uint8_t;
            } else if dx as ::core::ffi::c_int >= -(0xff as ::core::ffi::c_int)
                && dx as ::core::ffi::c_int <= 0xff as ::core::ffi::c_int
            {
                flag = (flag as ::core::ffi::c_int | GLYF_FLAG_X_SHORT as ::core::ffi::c_int)
                    as uint8_t;
                if dx as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    flag = (flag as ::core::ffi::c_int | GLYF_FLAG_POSITIVE_X as ::core::ffi::c_int)
                        as uint8_t;
                    bufwrite8(xs, dx as uint8_t);
                } else {
                    bufwrite8(xs, -(dx as ::core::ffi::c_int) as uint8_t);
                }
            } else {
                bufwrite16b(xs, dx as uint16_t);
            }
            if dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                flag = (flag as ::core::ffi::c_int | GLYF_FLAG_SAME_Y as ::core::ffi::c_int)
                    as uint8_t;
            } else if dy as ::core::ffi::c_int >= -(0xff as ::core::ffi::c_int)
                && dy as ::core::ffi::c_int <= 0xff as ::core::ffi::c_int
            {
                flag = (flag as ::core::ffi::c_int | GLYF_FLAG_Y_SHORT as ::core::ffi::c_int)
                    as uint8_t;
                if dy as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    flag = (flag as ::core::ffi::c_int | GLYF_FLAG_POSITIVE_Y as ::core::ffi::c_int)
                        as uint8_t;
                    bufwrite8(ys, dy as uint8_t);
                } else {
                    bufwrite8(ys, -(dy as ::core::ffi::c_int) as uint8_t);
                }
            } else {
                bufwrite16b(ys, dy as uint16_t);
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
    bufwrite16b(gbuf, -(1 as ::core::ffi::c_int) as uint16_t);
    bufwrite16b(gbuf, (*g).stat.xMin as int16_t as uint16_t);
    bufwrite16b(gbuf, (*g).stat.yMin as int16_t as uint16_t);
    bufwrite16b(gbuf, (*g).stat.xMax as int16_t as uint16_t);
    bufwrite16b(gbuf, (*g).stat.yMax as int16_t as uint16_t);
    let mut rj: shapeid_t = 0 as shapeid_t;
    while (rj as size_t) < (*g).references.length {
        let mut r: *mut glyf_ComponentReference =
            (*g).references.items.offset(rj as isize) as *mut glyf_ComponentReference;
        let mut flags: uint16_t =
            (if (rj as size_t) < (*g).references.length.wrapping_sub(1 as size_t) {
                MORE_COMPONENTS as ::core::ffi::c_int
            } else if (*g).instructionsLength as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                WE_HAVE_INSTRUCTIONS as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint16_t;
        let mut outputAnchor: bool = (*r).isAnchored as ::core::ffi::c_uint
            == REF_ANCHOR_CONSOLIDATED as ::core::ffi::c_int as ::core::ffi::c_uint;
        let mut arg1: C2RustUnnamed_1 = C2RustUnnamed_1 { pointid: 0 };
        let mut arg2: C2RustUnnamed_1 = C2RustUnnamed_1 { pointid: 0 };
        if outputAnchor {
            arg1.pointid = (*r).outer as uint16_t;
            arg2.pointid = (*r).inner as uint16_t;
            if !((arg1.pointid as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int
                && (arg2.pointid as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int)
            {
                flags = (flags as ::core::ffi::c_int | ARG_1_AND_2_ARE_WORDS as ::core::ffi::c_int)
                    as uint16_t;
            }
        } else {
            flags = (flags as ::core::ffi::c_int | ARGS_ARE_XY_VALUES as ::core::ffi::c_int)
                as uint16_t;
            arg1.coord = iVQ.getStill.expect("non-null function pointer")((*r).x) as int16_t;
            arg2.coord = iVQ.getStill.expect("non-null function pointer")((*r).y) as int16_t;
            if !((arg1.coord as ::core::ffi::c_int) < 128 as ::core::ffi::c_int
                && arg1.coord as ::core::ffi::c_int >= -(128 as ::core::ffi::c_int)
                && (arg2.coord as ::core::ffi::c_int) < 128 as ::core::ffi::c_int
                && arg2.coord as ::core::ffi::c_int >= -(128 as ::core::ffi::c_int))
            {
                flags = (flags as ::core::ffi::c_int | ARG_1_AND_2_ARE_WORDS as ::core::ffi::c_int)
                    as uint16_t;
            }
        }
        if fabs((*r).b as ::core::ffi::c_double) > EPSILON
            || fabs((*r).c as ::core::ffi::c_double) > EPSILON
        {
            flags = (flags as ::core::ffi::c_int | WE_HAVE_A_TWO_BY_TWO as ::core::ffi::c_int)
                as uint16_t;
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
                    as uint16_t;
            } else {
                flags = (flags as ::core::ffi::c_int | WE_HAVE_A_SCALE as ::core::ffi::c_int)
                    as uint16_t;
            }
        }
        if (*r).roundToGrid {
            flags =
                (flags as ::core::ffi::c_int | ROUND_XY_TO_GRID as ::core::ffi::c_int) as uint16_t;
        }
        if (*r).useMyMetrics {
            flags =
                (flags as ::core::ffi::c_int | USE_MY_METRICS as ::core::ffi::c_int) as uint16_t;
        }
        flags = (flags as ::core::ffi::c_int | UNSCALED_COMPONENT_OFFSET as ::core::ffi::c_int)
            as uint16_t;
        bufwrite16b(gbuf, flags);
        bufwrite16b(gbuf, (*r).glyph.index as uint16_t);
        if flags as ::core::ffi::c_int & ARG_1_AND_2_ARE_WORDS as ::core::ffi::c_int != 0 {
            bufwrite16b(gbuf, arg1.pointid);
            bufwrite16b(gbuf, arg2.pointid);
        } else {
            bufwrite8(gbuf, arg1.pointid as uint8_t);
            bufwrite8(gbuf, arg2.pointid as uint8_t);
        }
        if flags as ::core::ffi::c_int & WE_HAVE_A_SCALE as ::core::ffi::c_int != 0 {
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as uint16_t,
            );
        } else if flags as ::core::ffi::c_int & WE_HAVE_AN_X_AND_Y_SCALE as ::core::ffi::c_int != 0
        {
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as uint16_t,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).d as ::core::ffi::c_double) as uint16_t,
            );
        } else if flags as ::core::ffi::c_int & WE_HAVE_A_TWO_BY_TWO as ::core::ffi::c_int != 0 {
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).a as ::core::ffi::c_double) as uint16_t,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).b as ::core::ffi::c_double) as uint16_t,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).c as ::core::ffi::c_double) as uint16_t,
            );
            bufwrite16b(
                gbuf,
                otfcc_to_f2dot14((*r).d as ::core::ffi::c_double) as uint16_t,
            );
        }
        rj = rj.wrapping_add(1);
    }
    if (*g).instructionsLength != 0 {
        bufwrite16b(gbuf, (*g).instructionsLength);
        if !(*g).instructions.is_null() {
            bufwrite_bytes(gbuf, (*g).instructionsLength as size_t, (*g).instructions);
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
        let mut loca: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        loca = __caryll_allocate_clean(
            (::core::mem::size_of::<uint32_t>() as size_t)
                .wrapping_mul((*table).length.wrapping_add(1 as size_t)),
            189 as ::core::ffi::c_ulong,
        ) as *mut uint32_t;
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as size_t) < (*table).length {
            *loca.offset(j as isize) = (*bufglyf).cursor as uint32_t;
            let mut g: *mut glyf_Glyph = *(*table).items.offset(j as isize) as *mut glyf_Glyph;
            bufclear(gbuf);
            if (*g).contours.length > 0 as size_t {
                glyf_build_simple(g, gbuf);
            } else if (*g).references.length > 0 as size_t {
                glyf_build_composite(g, gbuf);
            }
            buflongalign(gbuf);
            bufwrite_buf(bufglyf, gbuf);
            j = j.wrapping_add(1);
        }
        *loca.offset((*table).length as isize) = (*bufglyf).cursor as uint32_t;
        if (*bufglyf).cursor >= 0x20000 as ::core::ffi::c_int as size_t {
            (*head).indexToLocFormat = 1 as int16_t;
        } else {
            (*head).indexToLocFormat = 0 as int16_t;
        }
        let mut j_0: uint32_t = 0 as uint32_t;
        while j_0 as size_t <= (*table).length {
            if (*head).indexToLocFormat != 0 {
                bufwrite32b(bufloca, *loca.offset(j_0 as isize));
            } else {
                bufwrite16b(
                    bufloca,
                    (*loca.offset(j_0 as isize) >> 1 as ::core::ffi::c_int) as uint16_t,
                );
            }
            j_0 = j_0.wrapping_add(1);
        }
        buffree(gbuf);
        free(loca as *mut ::core::ffi::c_void);
        loca = ::core::ptr::null_mut::<uint32_t>();
    }
    let mut pair: table_GlyfAndLocaBuffers = table_GlyfAndLocaBuffers {
        glyf: bufglyf,
        loca: bufloca,
    };
    return pair;
}
