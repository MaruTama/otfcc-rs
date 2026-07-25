use libc::{free, memcpy};
extern "C" {
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn otfcc_from_f2dot14(x: f2dot14) -> ::core::ffi::c_double;
    fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
    fn otfcc_f1616_muldiv(a: f16dot16, b: f16dot16, c: f16dot16) -> f16dot16;
    fn vq_createRegion(dimensions: shapeid_t) -> *mut vq_Region;
    static vq_iSegList: __caryll_vectorinterface_vq_SegList;
    static iVQ: __caryll_vectorinterface_VQ;
    static table_iFvar: __caryll_elementinterface_table_fvar;
    static glyf_iContour: __caryll_vectorinterface_glyf_Contour;
    static glyf_iContourList: __caryll_vectorinterface_glyf_ContourList;
    static glyf_iComponentReference: __caryll_elementinterface_glyf_ComponentReference;
    static glyf_iReferenceList: __caryll_vectorinterface_glyf_ReferenceList;
    static table_iGlyf: __caryll_vectorinterface_table_glyf;
    fn otfcc_newGlyf_glyph() -> *mut glyf_Glyph;
}

use crate::support::handle::{handle_fromIndex, otfcc_Handle, otfcc_GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_8u, read_8s, read_16u, read_16s, read_32u};
use crate::logger::{log_type_warning, otfcc_ILogger};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{f16dot16, f2dot14, font_file_pointer, glyphid_t, pos_t, scale_t, shapeid_t};
use crate::vendor::sds::{sds};
pub type otfcc_FDHandle = otfcc_Handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: ::core::ffi::c_uint,
    pub expand_mult: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut ::core::ffi::c_void,
    pub next: *mut ::core::ffi::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut ::core::ffi::c_void,
    pub keylen: ::core::ffi::c_uint,
    pub hashv: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: ::core::ffi::c_uint,
    pub log2_num_buckets: ::core::ffi::c_uint,
    pub num_items: ::core::ffi::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: isize,
    pub ideal_chain_maxlen: ::core::ffi::c_uint,
    pub nonideal_items: ::core::ffi::c_uint,
    pub ineff_expands: ::core::ffi::c_uint,
    pub noexpand: ::core::ffi::c_uint,
    pub signature: u32,
}
pub type otfcc_LoggerVerbosity = ::core::ffi::c_uint;
pub const log_vl_progress: otfcc_LoggerVerbosity = 10;
pub const log_vl_info: otfcc_LoggerVerbosity = 5;
pub const log_vl_notice: otfcc_LoggerVerbosity = 2;
pub const log_vl_important: otfcc_LoggerVerbosity = 1;
pub const log_vl_critical: otfcc_LoggerVerbosity = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_PacketPiece {
    pub tag: u32,
    pub checkSum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Packet {
    pub sfnt_version: u32,
    pub numTables: u16,
    pub searchRange: u16,
    pub entrySelector: u16,
    pub rangeShift: u16,
    pub pieces: *mut otfcc_PacketPiece,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VV {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut pos_t,
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
pub struct __caryll_vectorinterface_vq_SegList {
    pub init: Option<unsafe extern "C" fn(*mut vq_SegList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut vq_SegList, *const vq_SegList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut vq_SegList, *mut vq_SegList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vq_SegList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vq_SegList, vq_SegList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut vq_SegList, vq_SegList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut vq_SegList>,
    pub free: Option<unsafe extern "C" fn(*mut vq_SegList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut vq_SegList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut vq_SegList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut vq_SegList>,
    pub fill: Option<unsafe extern "C" fn(*mut vq_SegList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut vq_SegList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut vq_SegList, vq_Segment) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut vq_SegList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut vq_SegList) -> vq_Segment>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut vq_SegList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut vq_SegList,
            Option<unsafe extern "C" fn(*const vq_Segment, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut vq_SegList,
            Option<unsafe extern "C" fn(*const vq_Segment, *const vq_Segment) -> ::core::ffi::c_int>,
        ) -> (),
    >,
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
pub struct vf_Axis {
    pub tag: u32,
    pub minValue: pos_t,
    pub defaultValue: pos_t,
    pub maxValue: pos_t,
    pub flags: u16,
    pub axisNameID: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vf_Axes {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut vf_Axis,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fvar_Instance {
    pub subfamilyNameID: u16,
    pub flags: u16,
    pub coordinates: VV,
    pub postScriptNameID: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fvar_InstanceList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut fvar_Instance,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fvar_Master {
    pub name: sds,
    pub region: *mut vq_Region,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_fvar {
    pub majorVersion: u16,
    pub minorVersion: u16,
    pub axes: vf_Axes,
    pub instances: fvar_InstanceList,
    pub masters: *mut fvar_Master,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_fvar {
    pub init: Option<unsafe extern "C" fn(*mut table_fvar) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_fvar, *const table_fvar) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_fvar, *mut table_fvar) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_fvar) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_fvar, table_fvar) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_fvar, table_fvar) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_fvar>,
    pub free: Option<unsafe extern "C" fn(*mut table_fvar) -> ()>,
    pub registerRegion:
        Option<unsafe extern "C" fn(*mut table_fvar, *mut vq_Region) -> *const vq_Region>,
    pub findMasterByRegion:
        Option<unsafe extern "C" fn(*const table_fvar, *const vq_Region) -> *const fvar_Master>,
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
pub struct __caryll_vectorinterface_glyf_Contour {
    pub init: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_Contour, *const glyf_Contour) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_Contour, *mut glyf_Contour) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_Contour, glyf_Contour) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_Contour, glyf_Contour) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_Contour>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_Contour, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_Contour, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut glyf_Contour>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_Contour, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_Contour, glyf_Point) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_Contour) -> glyf_Point>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_Contour, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_Contour,
            Option<unsafe extern "C" fn(*const glyf_Point, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_Contour,
            Option<unsafe extern "C" fn(*const glyf_Point, *const glyf_Point) -> ::core::ffi::c_int>,
        ) -> (),
    >,
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
pub struct __caryll_vectorinterface_glyf_ContourList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_ContourList, *const glyf_ContourList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_ContourList, *mut glyf_ContourList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_ContourList, glyf_ContourList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_ContourList, glyf_ContourList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_ContourList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_ContourList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_ContourList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut glyf_ContourList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_ContourList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_ContourList, glyf_Contour) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_ContourList) -> glyf_Contour>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_ContourList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_ContourList,
            Option<unsafe extern "C" fn(*const glyf_Contour, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_ContourList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_Contour,
                    *const glyf_Contour,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
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
pub struct __caryll_elementinterface_glyf_ComponentReference {
    pub init: Option<unsafe extern "C" fn(*mut glyf_ComponentReference) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut glyf_ComponentReference, *const glyf_ComponentReference) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut glyf_ComponentReference, *mut glyf_ComponentReference) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_ComponentReference) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut glyf_ComponentReference, glyf_ComponentReference) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut glyf_ComponentReference, glyf_ComponentReference) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> glyf_ComponentReference>,
    pub dup: Option<unsafe extern "C" fn(glyf_ComponentReference) -> glyf_ComponentReference>,
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
pub struct __caryll_vectorinterface_glyf_ReferenceList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut glyf_ReferenceList, *const glyf_ReferenceList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut glyf_ReferenceList, *mut glyf_ReferenceList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, glyf_ReferenceList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut glyf_ReferenceList, glyf_ReferenceList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_ReferenceList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut glyf_ReferenceList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, glyf_ComponentReference) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_ReferenceList) -> glyf_ComponentReference>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_ReferenceList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_ComponentReference,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_ReferenceList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_ComponentReference,
                    *const glyf_ComponentReference,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
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
pub struct __caryll_vectorinterface_table_glyf {
    pub init: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_glyf, *const table_glyf) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_glyf, *mut table_glyf) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_glyf, table_glyf) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_glyf, table_glyf) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_glyf>,
    pub free: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_glyf, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_glyf, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut table_glyf>,
    pub fill: Option<unsafe extern "C" fn(*mut table_glyf, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_glyf, glyf_GlyphPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_glyf) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_glyf) -> glyf_GlyphPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_glyf, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_glyf,
            Option<unsafe extern "C" fn(*const glyf_GlyphPtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_glyf,
            Option<
                unsafe extern "C" fn(
                    *const glyf_GlyphPtr,
                    *const glyf_GlyphPtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyfIOContext {
    pub locaIsLong: bool,
    pub numGlyphs: glyphid_t,
    pub nPhantomPoints: shapeid_t,
    pub fvar: *mut table_fvar,
    pub hasVerticalMetrics: bool,
    pub exportFDSelect: bool,
}
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
    pub fvar: *mut table_fvar,
    pub dimensions: u16,
    pub sharedTupleCount: u16,
    pub sharedTuples: *mut f2dot14,
    pub coordDimensions: u8,
    pub allowIUP: bool,
    pub nPhantomPoints: shapeid_t,
}
pub type CoordPartGetter = Option<unsafe extern "C" fn(*mut glyf_Point) -> *mut VQ>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PackedDeltaRun {
    pub length: shapeid_t,
    pub wide: bool,
    pub zero: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PackedPointRun {
    pub length: shapeid_t,
    pub wide: bool,
}
pub const MORE_COMPONENTS: glyf_ComponentFlags = 32;
pub const WE_HAVE_INSTRUCTIONS: glyf_ComponentFlags = 256;
pub const WE_HAVE_A_TWO_BY_TWO: glyf_ComponentFlags = 128;
pub const WE_HAVE_AN_X_AND_Y_SCALE: glyf_ComponentFlags = 64;
pub const SCALED_COMPONENT_OFFSET: glyf_ComponentFlags = 2048;
pub const USE_MY_METRICS: glyf_ComponentFlags = 512;
pub const ROUND_XY_TO_GRID: glyf_ComponentFlags = 4;
pub const WE_HAVE_A_SCALE: glyf_ComponentFlags = 8;
pub const ARG_1_AND_2_ARE_WORDS: glyf_ComponentFlags = 1;
pub const ARGS_ARE_XY_VALUES: glyf_ComponentFlags = 2;
pub const GLYF_FLAG_SAME_Y: glyf_PointFlags = 32;
pub const GLYF_FLAG_POSITIVE_Y: glyf_PointFlags = 32;
pub const GLYF_FLAG_Y_SHORT: glyf_PointFlags = 4;
pub const GLYF_FLAG_SAME_X: glyf_PointFlags = 16;
pub const GLYF_FLAG_POSITIVE_X: glyf_PointFlags = 16;
pub const GLYF_FLAG_X_SHORT: glyf_PointFlags = 2;
pub const GLYF_FLAG_ON_CURVE: glyf_PointFlags = 1;
pub const GLYF_FLAG_REPEAT: glyf_PointFlags = 8;
pub type glyf_PointFlags = ::core::ffi::c_uint;
pub type glyf_ComponentFlags = ::core::ffi::c_uint;
pub const UNSCALED_COMPONENT_OFFSET: glyf_ComponentFlags = 4096;
pub const OVERLAP_COMPOUND: glyf_ComponentFlags = 1024;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn next_point(
    mut contours: *mut glyf_ContourList,
    mut cc: *mut shapeid_t,
    mut cp: *mut shapeid_t,
) -> *mut glyf_Point {
    if *cp as usize >= (*(*contours).items.offset(*cc as isize)).length {
        *cp = 0 as shapeid_t;
        *cc = (*cc as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
    }
    let fresh8 = *cp;
    *cp = (*cp).wrapping_add(1);
    return (*(*contours).items.offset(*cc as isize))
        .items
        .offset(fresh8 as isize) as *mut glyf_Point;
}
unsafe extern "C" fn otfcc_read_simple_glyph(
    mut start: font_file_pointer,
    mut numberOfContours: shapeid_t,
    mut _options: *const otfcc_Options,
) -> *mut glyf_Glyph {
    let mut g: *mut glyf_Glyph = otfcc_newGlyf_glyph();
    let mut contours: *mut glyf_ContourList = &raw mut (*g).contours;
    let mut pointsInGlyph: shapeid_t = 0 as shapeid_t;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as ::core::ffi::c_int) < numberOfContours as ::core::ffi::c_int {
        let mut lastPointInCurrentContour: shapeid_t = read_16u(
            start.offset((2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize)
                as *const u8,
        ) as shapeid_t;
        let mut contour: glyf_Contour = glyf_Contour {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<glyf_Point>(),
        };
        glyf_iContour.init.expect("non-null function pointer")(&raw mut contour);
        glyf_iContour.fill.expect("non-null function pointer")(
            &raw mut contour,
            (lastPointInCurrentContour as ::core::ffi::c_int - pointsInGlyph as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as usize,
        );
        glyf_iContourList.push.expect("non-null function pointer")(contours, contour);
        pointsInGlyph = (lastPointInCurrentContour as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as shapeid_t;
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
    let mut flags: font_file_pointer = ::core::ptr::null_mut::<u8>();
    flags = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul(pointsInGlyph as usize),
        41 as ::core::ffi::c_ulong,
    ) as font_file_pointer;
    let mut flagStart: font_file_pointer = start
        .offset((2 as ::core::ffi::c_int * numberOfContours as ::core::ffi::c_int) as isize)
        .offset(2 as ::core::ffi::c_int as isize)
        .offset(instructionLength as ::core::ffi::c_int as isize);
    let mut flagsReadSofar: shapeid_t = 0 as shapeid_t;
    let mut flagBytesReadSofar: shapeid_t = 0 as shapeid_t;
    let mut currentContour: shapeid_t = 0 as shapeid_t;
    let mut currentContourPointIndex: shapeid_t = 0 as shapeid_t;
    while (flagsReadSofar as ::core::ffi::c_int) < pointsInGlyph as ::core::ffi::c_int {
        let mut flag: u8 = *flagStart.offset(flagBytesReadSofar as isize);
        *flags.offset(flagsReadSofar as isize) = flag;
        flagBytesReadSofar =
            (flagBytesReadSofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
        flagsReadSofar =
            (flagsReadSofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
        (*next_point(
            contours,
            &raw mut currentContour,
            &raw mut currentContourPointIndex,
        ))
        .onCurve =
            (flag as ::core::ffi::c_int & GLYF_FLAG_ON_CURVE as ::core::ffi::c_int) as i8;
        if flag as ::core::ffi::c_int & GLYF_FLAG_REPEAT as ::core::ffi::c_int != 0 {
            let mut repeat: u8 = *flagStart.offset(flagBytesReadSofar as isize);
            flagBytesReadSofar =
                (flagBytesReadSofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
            let mut j_0: u8 = 0 as u8;
            while (j_0 as ::core::ffi::c_int) < repeat as ::core::ffi::c_int {
                *flags.offset(
                    (flagsReadSofar as ::core::ffi::c_int + j_0 as ::core::ffi::c_int) as isize,
                ) = flag;
                (*next_point(
                    contours,
                    &raw mut currentContour,
                    &raw mut currentContourPointIndex,
                ))
                .onCurve = (flag as ::core::ffi::c_int & GLYF_FLAG_ON_CURVE as ::core::ffi::c_int)
                    as i8;
                j_0 = j_0.wrapping_add(1);
            }
            flagsReadSofar =
                (flagsReadSofar as ::core::ffi::c_int + repeat as ::core::ffi::c_int) as shapeid_t;
        }
    }
    let mut coordinatesStart: font_file_pointer =
        flagStart.offset(flagBytesReadSofar as ::core::ffi::c_int as isize);
    let mut coordinatesOffset: u32 = 0 as u32;
    let mut coordinatesRead: shapeid_t = 0 as shapeid_t;
    currentContour = 0 as shapeid_t;
    currentContourPointIndex = 0 as shapeid_t;
    while (coordinatesRead as ::core::ffi::c_int) < pointsInGlyph as ::core::ffi::c_int {
        let mut flag_0: u8 = *flags.offset(coordinatesRead as isize);
        let mut x: i16 = 0;
        if flag_0 as ::core::ffi::c_int & GLYF_FLAG_X_SHORT as ::core::ffi::c_int != 0 {
            x = ((if flag_0 as ::core::ffi::c_int & GLYF_FLAG_POSITIVE_X as ::core::ffi::c_int != 0
            {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            }) * read_8u(coordinatesStart.offset(coordinatesOffset as isize) as *const u8)
                as ::core::ffi::c_int) as i16;
            coordinatesOffset = coordinatesOffset.wrapping_add(1 as u32);
        } else if flag_0 as ::core::ffi::c_int & GLYF_FLAG_SAME_X as ::core::ffi::c_int != 0 {
            x = 0 as i16;
        } else {
            x = read_16s(coordinatesStart.offset(coordinatesOffset as isize) as *const u8);
            coordinatesOffset = coordinatesOffset.wrapping_add(2 as u32);
        }
        iVQ.replace.expect("non-null function pointer")(
            &raw mut (*(next_point
                as unsafe extern "C" fn(
                    *mut glyf_ContourList,
                    *mut shapeid_t,
                    *mut shapeid_t,
                ) -> *mut glyf_Point)(
                contours,
                &raw mut currentContour,
                &raw mut currentContourPointIndex,
            ))
            .x,
            iVQ.createStill.expect("non-null function pointer")(x as pos_t) as VQ,
        );
        coordinatesRead =
            (coordinatesRead as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
    }
    coordinatesRead = 0 as shapeid_t;
    currentContour = 0 as shapeid_t;
    currentContourPointIndex = 0 as shapeid_t;
    while (coordinatesRead as ::core::ffi::c_int) < pointsInGlyph as ::core::ffi::c_int {
        let mut flag_1: u8 = *flags.offset(coordinatesRead as isize);
        let mut y: i16 = 0;
        if flag_1 as ::core::ffi::c_int & GLYF_FLAG_Y_SHORT as ::core::ffi::c_int != 0 {
            y = ((if flag_1 as ::core::ffi::c_int & GLYF_FLAG_POSITIVE_Y as ::core::ffi::c_int != 0
            {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            }) * read_8u(coordinatesStart.offset(coordinatesOffset as isize) as *const u8)
                as ::core::ffi::c_int) as i16;
            coordinatesOffset = coordinatesOffset.wrapping_add(1 as u32);
        } else if flag_1 as ::core::ffi::c_int & GLYF_FLAG_SAME_Y as ::core::ffi::c_int != 0 {
            y = 0 as i16;
        } else {
            y = read_16s(coordinatesStart.offset(coordinatesOffset as isize) as *const u8);
            coordinatesOffset = coordinatesOffset.wrapping_add(2 as u32);
        }
        iVQ.replace.expect("non-null function pointer")(
            &raw mut (*(next_point
                as unsafe extern "C" fn(
                    *mut glyf_ContourList,
                    *mut shapeid_t,
                    *mut shapeid_t,
                ) -> *mut glyf_Point)(
                contours,
                &raw mut currentContour,
                &raw mut currentContourPointIndex,
            ))
            .y,
            iVQ.createStill.expect("non-null function pointer")(y as pos_t) as VQ,
        );
        coordinatesRead =
            (coordinatesRead as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
    }
    free(flags as *mut ::core::ffi::c_void);
    flags = ::core::ptr::null_mut::<u8>();
    let mut cx: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut cy: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut j_1: shapeid_t = 0 as shapeid_t;
    while (j_1 as ::core::ffi::c_int) < numberOfContours as ::core::ffi::c_int {
        let mut k: shapeid_t = 0 as shapeid_t;
        while (k as usize) < (*(*contours).items.offset(j_1 as isize)).length {
            let mut z: *mut glyf_Point = (*(*contours).items.offset(j_1 as isize))
                .items
                .offset(k as isize) as *mut glyf_Point;
            iVQ.inplacePlus.expect("non-null function pointer")(&raw mut cx, (*z).x);
            iVQ.inplacePlus.expect("non-null function pointer")(&raw mut cy, (*z).y);
            iVQ.copyReplace.expect("non-null function pointer")(&raw mut (*z).x, cx);
            iVQ.copyReplace.expect("non-null function pointer")(&raw mut (*z).y, cy);
            k = k.wrapping_add(1);
        }
        glyf_iContour
            .shrinkToFit
            .expect("non-null function pointer")(
            (*contours).items.offset(j_1 as isize) as *mut glyf_Contour
        );
        j_1 = j_1.wrapping_add(1);
    }
    glyf_iContourList
        .shrinkToFit
        .expect("non-null function pointer")(contours);
    iVQ.dispose.expect("non-null function pointer")(&raw mut cx);
    iVQ.dispose.expect("non-null function pointer")(&raw mut cy);
    return g;
}
unsafe extern "C" fn otfcc_read_composite_glyph(
    mut start: font_file_pointer,
    mut options: *const otfcc_Options,
) -> *mut glyf_Glyph {
    let mut g: *mut glyf_Glyph = otfcc_newGlyf_glyph();
    let mut flags: u16 = 0 as u16;
    let mut offset: u32 = 0 as u32;
    let mut glyphHasInstruction: bool = false;
    loop {
        flags = read_16u(start.offset(offset as isize) as *const u8);
        let mut index: glyphid_t = read_16u(
            start
                .offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as glyphid_t;
        let mut ref_0: glyf_ComponentReference =
            (
                glyf_iComponentReference
                    .empty
                    .expect("non-null function pointer"))();
        ref_0.glyph =
            handle_fromIndex(index) as otfcc_GlyphHandle;
        offset = offset.wrapping_add(4 as u32);
        if flags as ::core::ffi::c_int & ARGS_ARE_XY_VALUES as ::core::ffi::c_int != 0 {
            ref_0.isAnchored = REF_XY;
            if flags as ::core::ffi::c_int & ARG_1_AND_2_ARE_WORDS as ::core::ffi::c_int != 0 {
                ref_0.x = iVQ.createStill.expect("non-null function pointer")(read_16s(
                    start.offset(offset as isize) as *const u8,
                )
                    as pos_t);
                ref_0.y = iVQ.createStill.expect("non-null function pointer")(read_16s(
                    start
                        .offset(offset as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const u8,
                )
                    as pos_t);
                offset = offset.wrapping_add(4 as u32);
            } else {
                ref_0.x = iVQ.createStill.expect("non-null function pointer")(read_8s(
                    start.offset(offset as isize) as *const u8,
                )
                    as pos_t);
                ref_0.y = iVQ.createStill.expect("non-null function pointer")(read_8s(
                    start
                        .offset(offset as isize)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *const u8,
                )
                    as pos_t);
                offset = offset.wrapping_add(2 as u32);
            }
        } else {
            ref_0.isAnchored = REF_ANCHOR_ANCHOR;
            if flags as ::core::ffi::c_int & ARG_1_AND_2_ARE_WORDS as ::core::ffi::c_int != 0 {
                ref_0.outer =
                    read_16u(start.offset(offset as isize) as *const u8) as shapeid_t;
                ref_0.inner = read_16u(
                    start
                        .offset(offset as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as shapeid_t;
                offset = offset.wrapping_add(4 as u32);
            } else {
                ref_0.outer = read_8u(start.offset(offset as isize) as *const u8) as shapeid_t;
                ref_0.inner = read_8u(
                    start
                        .offset(offset as isize)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as shapeid_t;
                offset = offset.wrapping_add(2 as u32);
            }
        }
        if flags as ::core::ffi::c_int & WE_HAVE_A_SCALE as ::core::ffi::c_int != 0 {
            ref_0.d = otfcc_from_f2dot14(
                read_16s(start.offset(offset as isize) as *const u8) as f2dot14
            ) as scale_t;
            ref_0.a = ref_0.d;
            offset = offset.wrapping_add(2 as u32);
        } else if flags as ::core::ffi::c_int & WE_HAVE_AN_X_AND_Y_SCALE as ::core::ffi::c_int != 0
        {
            ref_0.a = otfcc_from_f2dot14(
                read_16s(start.offset(offset as isize) as *const u8) as f2dot14
            ) as scale_t;
            ref_0.d = otfcc_from_f2dot14(read_16s(
                start
                    .offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as f2dot14) as scale_t;
            offset = offset.wrapping_add(4 as u32);
        } else if flags as ::core::ffi::c_int & WE_HAVE_A_TWO_BY_TWO as ::core::ffi::c_int != 0 {
            ref_0.a = otfcc_from_f2dot14(
                read_16s(start.offset(offset as isize) as *const u8) as f2dot14
            ) as scale_t;
            ref_0.b = otfcc_from_f2dot14(read_16s(
                start
                    .offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as f2dot14) as scale_t;
            ref_0.c = otfcc_from_f2dot14(read_16s(
                start
                    .offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as f2dot14) as scale_t;
            ref_0.d = otfcc_from_f2dot14(read_16s(
                start
                    .offset(offset as isize)
                    .offset(6 as ::core::ffi::c_int as isize) as *const u8,
            ) as f2dot14) as scale_t;
            offset = offset.wrapping_add(8 as u32);
        }
        ref_0.roundToGrid =
            flags as ::core::ffi::c_int & ROUND_XY_TO_GRID as ::core::ffi::c_int != 0;
        ref_0.useMyMetrics =
            flags as ::core::ffi::c_int & USE_MY_METRICS as ::core::ffi::c_int != 0;
        if flags as ::core::ffi::c_int & SCALED_COMPONENT_OFFSET as ::core::ffi::c_int != 0
            && (flags as ::core::ffi::c_int & WE_HAVE_AN_X_AND_Y_SCALE as ::core::ffi::c_int != 0
                || flags as ::core::ffi::c_int & WE_HAVE_A_TWO_BY_TWO as ::core::ffi::c_int != 0)
        {
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                sdscatprintf(
                    sdsempty(),
                    b"glyf: SCALED_COMPONENT_OFFSET is not supported.\0" as *const u8
                        as *const ::core::ffi::c_char,
                ),
            );
        }
        if flags as ::core::ffi::c_int & WE_HAVE_INSTRUCTIONS as ::core::ffi::c_int != 0 {
            glyphHasInstruction = true;
        }
        glyf_iReferenceList.push.expect("non-null function pointer")(
            &raw mut (*g).references,
            ref_0,
        );
        if !(flags as ::core::ffi::c_int & MORE_COMPONENTS as ::core::ffi::c_int != 0) {
            break;
        }
    }
    if glyphHasInstruction {
        let mut instructionLength: u16 =
            read_16u(start.offset(offset as isize) as *const u8);
        let mut instructions: font_file_pointer = ::core::ptr::null_mut::<u8>();
        if instructionLength as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            instructions = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize)
                    .wrapping_mul(instructionLength as usize),
                201 as ::core::ffi::c_ulong,
            ) as font_file_pointer;
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
    mut data: font_file_pointer,
    mut offset: u32,
    mut options: *const otfcc_Options,
) -> *mut glyf_Glyph {
    let mut start: font_file_pointer = data.offset(offset as isize);
    let mut numberOfContours: i16 = read_16u(start as *const u8) as i16;
    let mut g: *mut glyf_Glyph = ::core::ptr::null_mut::<glyf_Glyph>();
    if numberOfContours as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        g = otfcc_read_simple_glyph(
            start.offset(10 as ::core::ffi::c_int as isize),
            numberOfContours as shapeid_t,
            options,
        );
    } else {
        g = otfcc_read_composite_glyph(start.offset(10 as ::core::ffi::c_int as isize), options);
    }
    (*g).stat.xMin =
        read_16s(start.offset(2 as ::core::ffi::c_int as isize) as *const u8) as pos_t;
    (*g).stat.yMin =
        read_16s(start.offset(4 as ::core::ffi::c_int as isize) as *const u8) as pos_t;
    (*g).stat.xMax =
        read_16s(start.offset(6 as ::core::ffi::c_int as isize) as *const u8) as pos_t;
    (*g).stat.yMax =
        read_16s(start.offset(8 as ::core::ffi::c_int as isize) as *const u8) as pos_t;
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
            ((*ctx).dimensions as usize).wrapping_mul(::core::mem::size_of::<f2dot14>())
                as ::core::ffi::c_ulong,
        ) as u32 as u32;
    }
    if tupleIndex as ::core::ffi::c_int & INTERMEDIATE_REGION != 0 {
        bump = (bump as ::core::ffi::c_ulong).wrapping_add(
            ((2 as ::core::ffi::c_int * (*ctx).dimensions as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<f2dot14>())
                as ::core::ffi::c_ulong,
        ) as u32 as u32;
    }
    return (currentHeader as font_file_pointer).offset(bump as isize) as *mut TupleVariationHeader;
}
pub const POINT_COUNT_IS_WORD: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const POINT_COUNT_LONG_MASK: ::core::ffi::c_int = 0x7fff as ::core::ffi::c_int;
pub const POINT_RUN_COUNT_MASK: ::core::ffi::c_int = 0x7f as ::core::ffi::c_int;
pub const POINTS_ARE_WORDS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn parsePointNumbers(
    mut data: font_file_pointer,
    mut pointIndeces: *mut *mut shapeid_t,
    mut pc: *mut shapeid_t,
    mut totalPoints: shapeid_t,
) -> font_file_pointer {
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
        let mut run: glyf_PackedPointRun = glyf_PackedPointRun {
            length: 0 as shapeid_t,
            wide: false,
        };
        let mut filled: shapeid_t = 0 as shapeid_t;
        let mut jPoint: shapeid_t = 0 as shapeid_t;
        *pointIndeces = __caryll_allocate_clean(
            (::core::mem::size_of::<shapeid_t>() as usize).wrapping_mul(nPoints as usize),
            305 as ::core::ffi::c_ulong,
        ) as *mut shapeid_t;
        while (filled as ::core::ffi::c_int) < nPoints as ::core::ffi::c_int {
            if run.length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let fresh6 = data;
                data = data.offset(1);
                let mut runHeader: u8 = *fresh6;
                run.wide = runHeader as ::core::ffi::c_int & POINTS_ARE_WORDS != 0;
                run.length = ((runHeader as ::core::ffi::c_int & POINT_RUN_COUNT_MASK)
                    + 1 as ::core::ffi::c_int) as shapeid_t;
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
            *(*pointIndeces).offset(filled as isize) = pointNumber as shapeid_t;
            filled = filled.wrapping_add(1);
            jPoint = pointNumber as shapeid_t;
            run.length = run.length.wrapping_sub(1);
        }
        *pc = nPoints as shapeid_t;
    } else {
        *pointIndeces = __caryll_allocate_clean(
            (::core::mem::size_of::<shapeid_t>() as usize).wrapping_mul(totalPoints as usize),
            326 as ::core::ffi::c_ulong,
        ) as *mut shapeid_t;
        let mut j: shapeid_t = 0 as shapeid_t;
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
    mut data: font_file_pointer,
    mut nPoints: shapeid_t,
    mut deltas: *mut pos_t,
) -> font_file_pointer {
    let mut run: glyf_PackedDeltaRun = glyf_PackedDeltaRun {
        length: 0 as shapeid_t,
        wide: false,
        zero: false,
    };
    let mut filled: shapeid_t = 0 as shapeid_t;
    while (filled as ::core::ffi::c_int) < nPoints as ::core::ffi::c_int {
        let mut delta: i16 = 0 as i16;
        if run.length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let fresh5 = data;
            data = data.offset(1);
            let mut runHeader: u8 = *fresh5;
            run.zero = runHeader as ::core::ffi::c_int & DELTAS_ARE_ZERO != 0;
            run.wide = runHeader as ::core::ffi::c_int & DELTAS_ARE_WORDS != 0;
            run.length = ((runHeader as ::core::ffi::c_int & DELTA_RUN_COUNT_MASK)
                + 1 as ::core::ffi::c_int) as shapeid_t;
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
        *deltas.offset(filled as isize) = delta as pos_t;
        filled = filled.wrapping_add(1);
        run.length = run.length.wrapping_sub(1);
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn getX(mut z: *mut glyf_Point) -> *mut VQ {
    return &raw mut (*z).x;
}
#[no_mangle]
pub unsafe extern "C" fn getY(mut z: *mut glyf_Point) -> *mut VQ {
    return &raw mut (*z).y;
}
#[inline]
unsafe extern "C" fn fillTheGaps(
    mut jMin: shapeid_t,
    mut jMax: shapeid_t,
    mut nudges: *mut vq_Segment,
    mut glyphRefs: *mut *mut glyf_Point,
    mut getter: CoordPartGetter,
) {
    let mut j: shapeid_t = jMin;
    while (j as ::core::ffi::c_int) < jMax as ::core::ffi::c_int {
        if !(*nudges.offset(j as isize)).val.delta.touched {
            let mut jNext: shapeid_t = j;
            while !(*nudges.offset(jNext as isize)).val.delta.touched {
                if jNext as ::core::ffi::c_int
                    == jMax as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    jNext = jMin;
                } else {
                    jNext = (jNext as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
                }
                if jNext as ::core::ffi::c_int == j as ::core::ffi::c_int {
                    break;
                }
            }
            let mut jPrev: shapeid_t = j;
            while !(*nudges.offset(jPrev as isize)).val.delta.touched {
                if jPrev as ::core::ffi::c_int == jMin as ::core::ffi::c_int {
                    jPrev = (jMax as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as shapeid_t;
                } else {
                    jPrev = (jPrev as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as shapeid_t;
                }
                if jPrev as ::core::ffi::c_int == j as ::core::ffi::c_int {
                    break;
                }
            }
            if (*nudges.offset(jNext as isize)).val.delta.touched as ::core::ffi::c_int != 0
                && (*nudges.offset(jPrev as isize)).val.delta.touched as ::core::ffi::c_int != 0
            {
                let mut untouchJ: f16dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(*glyphRefs.offset(j as isize)))
                        .kernel as ::core::ffi::c_double,
                );
                let mut untouchPrev: f16dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(*glyphRefs.offset(jPrev as isize)))
                        .kernel as ::core::ffi::c_double,
                );
                let mut untouchNext: f16dot16 = otfcc_to_fixed(
                    (*getter.expect("non-null function pointer")(*glyphRefs.offset(jNext as isize)))
                        .kernel as ::core::ffi::c_double,
                );
                let mut deltaPrev: f16dot16 = otfcc_to_fixed(
                    (*nudges.offset(jPrev as isize)).val.delta.quantity as ::core::ffi::c_double,
                );
                let mut deltaNext: f16dot16 = otfcc_to_fixed(
                    (*nudges.offset(jNext as isize)).val.delta.quantity as ::core::ffi::c_double,
                );
                let mut uMin: f16dot16 = untouchPrev;
                let mut uMax: f16dot16 = untouchNext;
                let mut dMin: f16dot16 = deltaPrev;
                let mut dMax: f16dot16 = deltaNext;
                if untouchPrev > untouchNext {
                    uMin = untouchNext;
                    uMax = untouchPrev;
                    dMin = deltaNext;
                    dMax = deltaPrev;
                }
                if untouchJ <= uMin {
                    (*nudges.offset(j as isize)).val.delta.quantity =
                        otfcc_from_fixed(dMin) as pos_t;
                } else if untouchJ >= uMax {
                    (*nudges.offset(j as isize)).val.delta.quantity =
                        otfcc_from_fixed(dMax) as pos_t;
                } else {
                    (*nudges.offset(j as isize)).val.delta.quantity = otfcc_from_fixed(
                        otfcc_f1616_muldiv(dMax - dMin, untouchJ - uMin, uMax - uMin),
                    )
                        as pos_t;
                }
            }
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn applyCoords(
    totalPoints: shapeid_t,
    mut glyph: *mut glyf_Glyph,
    mut glyphRefs: *mut *mut glyf_Point,
    nTouchedPoints: shapeid_t,
    mut tupleDelta: *const pos_t,
    mut points: *const shapeid_t,
    mut r: *const vq_Region,
    mut getter: CoordPartGetter,
) {
    let mut nudges: *mut vq_Segment = ::core::ptr::null_mut::<vq_Segment>();
    nudges = __caryll_allocate_clean(
        (::core::mem::size_of::<vq_Segment>() as usize).wrapping_mul(totalPoints as usize),
        441 as ::core::ffi::c_ulong,
    ) as *mut vq_Segment;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as ::core::ffi::c_int) < totalPoints as ::core::ffi::c_int {
        (*nudges.offset(j as isize)).type_0 = VQ_DELTA;
        (*nudges.offset(j as isize)).val.delta.touched = false;
        (*nudges.offset(j as isize)).val.delta.quantity = 0 as ::core::ffi::c_int as pos_t;
        let ref mut fresh4 = (*nudges.offset(j as isize)).val.delta.region;
        *fresh4 = r;
        j = j.wrapping_add(1);
    }
    let mut j_0: shapeid_t = 0 as shapeid_t;
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
    let mut jFirst: shapeid_t = 0 as shapeid_t;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*glyph).contours.length {
        let mut c: *mut glyf_Contour = (*glyph).contours.items.offset(__caryll_index as isize);
        while keep != 0 {
            fillTheGaps(
                jFirst,
                (jFirst as usize).wrapping_add((*c).length) as shapeid_t,
                nudges,
                glyphRefs,
                Some(getX as unsafe extern "C" fn(*mut glyf_Point) -> *mut VQ),
            );
            jFirst = (jFirst as usize).wrapping_add((*c).length) as shapeid_t as shapeid_t;
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut j_1: shapeid_t = 0 as shapeid_t;
    while (j_1 as ::core::ffi::c_int) < totalPoints as ::core::ffi::c_int {
        if !((*nudges.offset(j_1 as isize)).val.delta.quantity == 0.
            && (*nudges.offset(j_1 as isize)).val.delta.touched as ::core::ffi::c_int != 0)
        {
            let mut coordinatePart: *mut VQ =
                getter.expect("non-null function pointer")(*glyphRefs.offset(j_1 as isize));
            vq_iSegList.push.expect("non-null function pointer")(
                &raw mut (*coordinatePart).shift,
                *nudges.offset(j_1 as isize),
            );
        }
        j_1 = j_1.wrapping_add(1);
    }
    free(nudges as *mut ::core::ffi::c_void);
    nudges = ::core::ptr::null_mut::<vq_Segment>();
}
#[inline]
unsafe extern "C" fn applyPolymorphism(
    totalPoints: shapeid_t,
    mut glyph: glyf_GlyphPtr,
    nTouchedPoints: shapeid_t,
    mut points: *const shapeid_t,
    mut deltaX: *const pos_t,
    mut deltaY: *const pos_t,
    mut r: *const vq_Region,
) {
    let mut glyphRefs: *mut *mut glyf_Point = ::core::ptr::null_mut::<*mut glyf_Point>();
    glyphRefs = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut glyf_Point>() as usize).wrapping_mul(totalPoints as usize),
        473 as ::core::ffi::c_ulong,
    ) as *mut *mut glyf_Point;
    let mut j: shapeid_t = 0 as shapeid_t;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*glyph).contours.length {
        let mut c: *mut glyf_Contour = (*glyph).contours.items.offset(__caryll_index as isize);
        while keep != 0 {
            let mut __caryll_index_0: usize = 0 as usize;
            let mut keep_0: usize = 1 as usize;
            while keep_0 != 0 && __caryll_index_0 < (*c).length {
                let mut g: *mut glyf_Point = (*c).items.offset(__caryll_index_0 as isize);
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
        let mut r_0: *mut glyf_ComponentReference =
            (*glyph).references.items.offset(__caryll_index_1 as isize);
        while keep_1 != 0 {
            let fresh2 = j;
            j = j.wrapping_add(1);
            let ref mut fresh3 = *glyphRefs.offset(fresh2 as isize);
            *fresh3 = &raw mut (*r_0).x as *mut glyf_Point;
            keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
        }
        keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
        __caryll_index_1 = __caryll_index_1.wrapping_add(1);
    }
    applyCoords(
        totalPoints,
        glyph as *mut glyf_Glyph,
        glyphRefs,
        nTouchedPoints,
        deltaX,
        points,
        r,
        Some(getX as unsafe extern "C" fn(*mut glyf_Point) -> *mut VQ),
    );
    applyCoords(
        totalPoints,
        glyph as *mut glyf_Glyph,
        glyphRefs,
        nTouchedPoints,
        deltaY,
        points,
        r,
        Some(getY as unsafe extern "C" fn(*mut glyf_Point) -> *mut VQ),
    );
    if (totalPoints as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        < nTouchedPoints as ::core::ffi::c_int
    {
        iVQ.addDelta.expect("non-null function pointer")(
            &raw mut (*glyph).horizontalOrigin,
            true,
            r,
            *deltaX.offset(totalPoints as isize),
        );
        iVQ.addDelta.expect("non-null function pointer")(
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
        iVQ.addDelta.expect("non-null function pointer")(
            &raw mut (*glyph).verticalOrigin,
            true,
            r,
            *deltaY.offset((totalPoints as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize),
        );
        iVQ.addDelta.expect("non-null function pointer")(
            &raw mut (*glyph).advanceHeight,
            true,
            r,
            *deltaY.offset((totalPoints as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                - *deltaY
                    .offset((totalPoints as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize),
        );
    }
    free(glyphRefs as *mut ::core::ffi::c_void);
    glyphRefs = ::core::ptr::null_mut::<*mut glyf_Point>();
}
unsafe extern "C" fn createRegionFromTuples(
    mut dimensions: u16,
    mut peak: *mut f2dot14,
    mut start: *mut f2dot14,
    mut end: *mut f2dot14,
) -> *mut vq_Region {
    let mut r: *mut vq_Region = vq_createRegion(dimensions as shapeid_t);
    let mut d: u16 = 0 as u16;
    while (d as ::core::ffi::c_int) < dimensions as ::core::ffi::c_int {
        let mut peakVal: pos_t =
            otfcc_from_f2dot14(be16(*peak.offset(d as isize) as u16) as f2dot14) as pos_t;
        let mut span: vq_AxisSpan = vq_AxisSpan {
            start: (if peakVal <= 0 as ::core::ffi::c_int as pos_t {
                -(1 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            }) as pos_t,
            peak: peakVal,
            end: (if peakVal >= 0 as ::core::ffi::c_int as pos_t {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as pos_t,
        };
        if !start.is_null() && !end.is_null() {
            span.start =
                otfcc_from_f2dot14(be16(*start.offset(d as isize) as u16) as f2dot14) as pos_t;
            span.end =
                otfcc_from_f2dot14(be16(*end.offset(d as isize) as u16) as f2dot14) as pos_t;
        }
        *(&raw mut (*r).spans as *mut vq_AxisSpan).offset(d as isize) = span;
        d = d.wrapping_add(1);
    }
    return r;
}
#[inline]
unsafe extern "C" fn polymorphizeGlyph(
    mut _gid: glyphid_t,
    mut glyph: glyf_GlyphPtr,
    mut ctx: *const TuplePolymorphizerCtx,
    mut gvd: *mut GlyphVariationData,
    mut _options: *const otfcc_Options,
) {
    let mut totalPoints: shapeid_t = 0 as shapeid_t;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*glyph).contours.length {
        let mut c: *mut glyf_Contour = (*glyph).contours.items.offset(__caryll_index as isize);
        while keep != 0 {
            totalPoints =
                (totalPoints as usize).wrapping_add((*c).length) as shapeid_t as shapeid_t;
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    totalPoints =
        (totalPoints as usize).wrapping_add((*glyph).references.length) as shapeid_t as shapeid_t;
    let mut totalDeltaEntries: shapeid_t = (totalPoints as ::core::ffi::c_int
        + (*ctx).nPhantomPoints as ::core::ffi::c_int)
        as shapeid_t;
    let mut nTuples: u16 = (be16((*gvd).tupleVariationCount) as ::core::ffi::c_int
        & 0xfff as ::core::ffi::c_int) as u16;
    let mut tvh: *mut TupleVariationHeader = &raw mut (*gvd).tvhs as *mut TupleVariationHeader;
    let mut hasSharedPointNumbers: bool =
        be16((*gvd).tupleVariationCount) as ::core::ffi::c_int & 0x8000 as ::core::ffi::c_int != 0;
    let mut sharedPointCount: shapeid_t = 0 as shapeid_t;
    let mut sharedPointIndeces: *mut shapeid_t = ::core::ptr::null_mut::<shapeid_t>();
    let mut data: font_file_pointer =
        (gvd as font_file_pointer).offset(be16((*gvd).dataOffset) as ::core::ffi::c_int as isize);
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
        let mut tupleIndex: shapeid_t =
            (be16((*tvh).tupleIndex) as ::core::ffi::c_int & TUPLE_INDEX_MASK) as shapeid_t;
        let mut hasEmbeddedPeak: bool =
            be16((*tvh).tupleIndex) as ::core::ffi::c_int & EMBEDDED_PEAK_TUPLE != 0;
        let mut hasIntermediate: bool =
            be16((*tvh).tupleIndex) as ::core::ffi::c_int & INTERMEDIATE_REGION != 0;
        let mut peak: *mut f2dot14 = ::core::ptr::null_mut::<f2dot14>();
        if hasEmbeddedPeak {
            peak =
                (tvh as font_file_pointer).offset(4 as ::core::ffi::c_int as isize) as *mut f2dot14;
        } else {
            peak = (*ctx).sharedTuples.offset(
                ((*ctx).dimensions as ::core::ffi::c_int * tupleIndex as ::core::ffi::c_int)
                    as isize,
            );
        }
        let mut start: *mut f2dot14 = ::core::ptr::null_mut::<f2dot14>();
        let mut end: *mut f2dot14 = ::core::ptr::null_mut::<f2dot14>();
        if hasIntermediate {
            start = (tvh as font_file_pointer)
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(
                    (2 as ::core::ffi::c_int
                        * (if hasEmbeddedPeak as ::core::ffi::c_int != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                        * (*ctx).dimensions as ::core::ffi::c_int) as isize,
                ) as *mut f2dot14;
            end = (tvh as font_file_pointer)
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(
                    (2 as ::core::ffi::c_int
                        * (if hasEmbeddedPeak as ::core::ffi::c_int != 0 {
                            2 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        })
                        * (*ctx).dimensions as ::core::ffi::c_int) as isize,
                ) as *mut f2dot14;
        }
        let mut r: *const vq_Region = table_iFvar
            .registerRegion
            .expect("non-null function pointer")(
            (*ctx).fvar,
            createRegionFromTuples((*ctx).dimensions, peak, start, end),
        );
        let mut tsd: font_file_pointer = data.offset(tsdStart as isize);
        let mut nPoints: shapeid_t = sharedPointCount;
        let mut pointIndeces: *mut shapeid_t = sharedPointIndeces;
        if be16((*tvh).tupleIndex) as ::core::ffi::c_int & PRIVATE_POINT_NUMBERS != 0 {
            let mut privatePointCount: shapeid_t = 0 as shapeid_t;
            let mut privatePointNumbers: *mut shapeid_t = ::core::ptr::null_mut::<shapeid_t>();
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
            let mut deltaX: *mut pos_t = ::core::ptr::null_mut::<pos_t>();
            let mut deltaY: *mut pos_t = ::core::ptr::null_mut::<pos_t>();
            deltaX = __caryll_allocate_clean(
                (::core::mem::size_of::<pos_t>() as usize).wrapping_mul(nPoints as usize),
                586 as ::core::ffi::c_ulong,
            ) as *mut pos_t;
            deltaY = __caryll_allocate_clean(
                (::core::mem::size_of::<pos_t>() as usize).wrapping_mul(nPoints as usize),
                587 as ::core::ffi::c_ulong,
            ) as *mut pos_t;
            tsd = readPackedDelta(tsd, nPoints, deltaX);
            tsd = readPackedDelta(tsd, nPoints, deltaY);
            applyPolymorphism(totalPoints, glyph, nPoints, pointIndeces, deltaX, deltaY, r);
            free(deltaX as *mut ::core::ffi::c_void);
            deltaX = ::core::ptr::null_mut::<pos_t>();
            free(deltaY as *mut ::core::ffi::c_void);
            deltaY = ::core::ptr::null_mut::<pos_t>();
        }
        if be16((*tvh).tupleIndex) as ::core::ffi::c_int & PRIVATE_POINT_NUMBERS != 0 {
            free(pointIndeces as *mut ::core::ffi::c_void);
            pointIndeces = ::core::ptr::null_mut::<shapeid_t>();
        }
        tsdStart = tsdStart.wrapping_add(be16((*tvh).variationDataSize) as usize);
        tvh = nextTVH(tvh, ctx);
        j = j.wrapping_add(1);
    }
    free(sharedPointIndeces as *mut ::core::ffi::c_void);
    sharedPointIndeces = ::core::ptr::null_mut::<shapeid_t>();
}
#[inline]
unsafe extern "C" fn polymorphize(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
    mut glyf: *mut table_glyf,
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
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1735811442i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    if (table.length as usize) < ::core::mem::size_of::<GVARHeader>() {
                        return;
                    }
                    let mut header: *mut GVARHeader = data as *mut GVARHeader;
                    if be16((*header).axisCount) as usize != (*(*ctx).fvar).axes.length {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Axes number in GVAR and FVAR are inequal\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                        return;
                    }
                    let mut j: glyphid_t = 0 as glyphid_t;
                    while (j as usize) < (*glyf).length {
                        let mut tpctx: TuplePolymorphizerCtx = TuplePolymorphizerCtx {
                            fvar: (*ctx).fvar,
                            dimensions: (*(*ctx).fvar).axes.length as u16,
                            sharedTupleCount: be16((*header).sharedTupleCount),
                            sharedTuples: data.offset(be32((*header).sharedTuplesOffset) as isize)
                                as *mut f2dot14,
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
#[no_mangle]
pub unsafe extern "C" fn otfcc_readGlyf(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
    mut ctx: *const GlyfIOContext,
) -> *mut table_glyf {
    let mut foundLoca: bool = false;
    let mut current_block: u64;
    let mut offsets: *mut u32 = ::core::ptr::null_mut::<u32>();
    let mut glyf: *mut table_glyf = ::core::ptr::null_mut::<table_glyf>();
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
            let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
            while __fortable_keep != 0 {
                if table.tag == 1819239265i32 as u32 {
                    let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    while __fortable_k2 != 0 {
                        let mut data: font_file_pointer = table.data as font_file_pointer;
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
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"table 'loca' corrupted.\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
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
                let mut table_0: otfcc_PacketPiece =
                    *packet.pieces.offset(__fortable_count_0 as isize);
                while __fortable_keep_0 != 0 {
                    if table_0.tag == 1735162214i32 as u32 {
                        let mut __fortable_k2_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        while __fortable_k2_0 != 0 {
                            let mut data_0: font_file_pointer = table_0.data as font_file_pointer;
                            let mut length_0: u32 = table_0.length;
                            if length_0 < *offsets.offset((*ctx).numGlyphs as isize) {
                                (*(*options).logger)
                                    .logSDS
                                    .expect("non-null function pointer")(
                                    (*options).logger as *mut otfcc_ILogger,
                                    log_vl_important as ::core::ffi::c_int as u8,
                                    log_type_warning,
                                    sdscatprintf(
                                        sdsempty(),
                                        b"table 'glyf' corrupted.\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    ),
                                );
                                if !glyf.is_null() {
                                    table_iGlyf.free.expect("non-null function pointer")(glyf);
                                    glyf = ::core::ptr::null_mut::<table_glyf>();
                                    glyf = ::core::ptr::null_mut::<table_glyf>();
                                }
                                __fortable_k2_0 = 0 as ::core::ffi::c_int;
                                __notfound_0 = 0 as ::core::ffi::c_int;
                            } else {
                                glyf = (
                                    table_iGlyf.create.expect("non-null function pointer"))();
                                let mut j_0: glyphid_t = 0 as glyphid_t;
                                while (j_0 as ::core::ffi::c_int)
                                    < (*ctx).numGlyphs as ::core::ffi::c_int
                                {
                                    if *offsets.offset(j_0 as isize)
                                        < *offsets.offset(
                                            (j_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                    {
                                        table_iGlyf.push.expect("non-null function pointer")(
                                            glyf,
                                            otfcc_read_glyph(
                                                data_0,
                                                *offsets.offset(j_0 as isize),
                                                options,
                                            )
                                                as glyf_GlyphPtr,
                                        );
                                    } else {
                                        table_iGlyf.push.expect("non-null function pointer")(
                                            glyf,
                                            otfcc_newGlyf_glyph() as glyf_GlyphPtr,
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
        glyf = ::core::ptr::null_mut::<table_glyf>();
        glyf = ::core::ptr::null_mut::<table_glyf>();
    }
    return ::core::ptr::null_mut::<table_glyf>();
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
