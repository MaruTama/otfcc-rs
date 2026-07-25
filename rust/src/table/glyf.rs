pub mod build;
pub mod read;

use libc::{fprintf, free, malloc, memcmp, memcpy, memset, qsort, strcmp};
extern "C" {
    fn json_value_free(_: *mut json_value);
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    static iVQ: __caryll_vectorinterface_VQ;
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new(_: *const ::core::ffi::c_char) -> *mut json_value;
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
    fn json_null_new() -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn json_new_VQ(z: VQ, fvar: *const table_fvar) -> *mut json_value;
    fn json_vqOf(cv: *const json_value, fvar: *const table_fvar) -> VQ;
    fn parse_ttinstr(
        col: *mut json_value,
        context: *mut ::core::ffi::c_void,
        Make: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut u8, u32) -> ()>,
        Wrong: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> (),
        >,
    );
    fn dump_ttinstr(
        instructions: *mut u8,
        length: u32,
        options: *const otfcc_Options,
    ) -> *mut json_value;
}

use crate::support::handle::{HANDLE_STATE_EMPTY, handle_fromName, otfcc_FDHandle, otfcc_GlyphHandle, otfcc_Handle, otfcc_Handle_copy, otfcc_Handle_dispose, otfcc_Handle_empty};
use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{otfcc_ILogger};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphid_t, pos_t, scale_t, shapeid_t};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};
use crate::vendor::json::{_json_value, json_array, json_boolean, json_double, json_integer, json_object, json_pre_serialized, json_string, json_type, json_value};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::buffer::{caryll_Buffer};
use crate::support::{__compar_fn_t, true_0};
use crate::support::glyph_order::{otfcc_GlyphOrder, otfcc_GlyphOrderEntry};
use crate::table::fvar::{table_fvar};
use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};



use crate::vf::vq::{VQ, __caryll_vectorinterface_VQ, vq_SegList, vq_Segment};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Point {
    pub x: VQ,
    pub y: VQ,
    pub onCurve: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_glyf_Point {
    pub init: Option<unsafe extern "C" fn(*mut glyf_Point) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_Point, *const glyf_Point) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_Point, *mut glyf_Point) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_Point) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_Point, glyf_Point) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_Point, glyf_Point) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> glyf_Point>,
    pub dup: Option<unsafe extern "C" fn(glyf_Point) -> glyf_Point>,
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
pub struct __caryll_elementinterface_glyf_PostscriptStemDef {
    pub init: Option<unsafe extern "C" fn(*mut glyf_PostscriptStemDef) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut glyf_PostscriptStemDef, *const glyf_PostscriptStemDef) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut glyf_PostscriptStemDef, *mut glyf_PostscriptStemDef) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_PostscriptStemDef) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut glyf_PostscriptStemDef, glyf_PostscriptStemDef) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut glyf_PostscriptStemDef, glyf_PostscriptStemDef) -> ()>,
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
pub struct __caryll_vectorinterface_glyf_StemDefList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_StemDefList, *const glyf_StemDefList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_StemDefList, *mut glyf_StemDefList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_StemDefList, glyf_StemDefList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_StemDefList, glyf_StemDefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_StemDefList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut glyf_StemDefList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_StemDefList, glyf_PostscriptStemDef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_StemDefList) -> glyf_PostscriptStemDef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_StemDefList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_PostscriptStemDef,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_StemDefList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_PostscriptStemDef,
                    *const glyf_PostscriptStemDef,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
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
pub struct __caryll_elementinterface_glyf_PostscriptHintMask {
    pub init: Option<unsafe extern "C" fn(*mut glyf_PostscriptHintMask) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut glyf_PostscriptHintMask, *const glyf_PostscriptHintMask) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut glyf_PostscriptHintMask, *mut glyf_PostscriptHintMask) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_PostscriptHintMask) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut glyf_PostscriptHintMask, glyf_PostscriptHintMask) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut glyf_PostscriptHintMask, glyf_PostscriptHintMask) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_MaskList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut glyf_PostscriptHintMask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_glyf_MaskList {
    pub init: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_MaskList, *const glyf_MaskList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_MaskList, *mut glyf_MaskList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_MaskList, glyf_MaskList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_MaskList, glyf_MaskList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_MaskList>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_MaskList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_MaskList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut glyf_MaskList>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_MaskList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_MaskList, glyf_PostscriptHintMask) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_MaskList) -> glyf_PostscriptHintMask>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_MaskList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_MaskList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_PostscriptHintMask,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_MaskList,
            Option<
                unsafe extern "C" fn(
                    *const glyf_PostscriptHintMask,
                    *const glyf_PostscriptHintMask,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RefAnchorStatus {
    REF_XY = 0,
    REF_ANCHOR_ANCHOR = 1,
    REF_ANCHOR_XY = 2,
    REF_ANCHOR_CONSOLIDATED = 3,
    REF_ANCHOR_CONSOLIDATING_ANCHOR = 4,
    REF_ANCHOR_CONSOLIDATING_XY = 5,
}
pub use RefAnchorStatus::*;
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
pub struct __caryll_elementinterface_glyf_GlyphPtr {
    pub init: Option<unsafe extern "C" fn(*mut glyf_GlyphPtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_GlyphPtr, *const glyf_GlyphPtr) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_GlyphPtr, *mut glyf_GlyphPtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_GlyphPtr) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_GlyphPtr, glyf_GlyphPtr) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_GlyphPtr, glyf_GlyphPtr) -> ()>,
}
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
pub const MASK_ON_CURVE: glyf_OnCurveMask = 1;
pub type glyf_OnCurveMask = ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
unsafe extern "C" fn createPoint(mut p: *mut glyf_Point) {
    (*p).x = iVQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as pos_t);
    (*p).y = iVQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as pos_t);
    (*p).onCurve = true_0 as i8;
}
unsafe extern "C" fn copyPoint(mut dst: *mut glyf_Point, mut src: *const glyf_Point) {
    iVQ.copy.expect("non-null function pointer")(&raw mut (*dst).x, &raw const (*src).x);
    iVQ.copy.expect("non-null function pointer")(&raw mut (*dst).y, &raw const (*src).y);
    (*dst).onCurve = (*src).onCurve;
}
unsafe extern "C" fn disposePoint(mut p: *mut glyf_Point) {
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*p).x);
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*p).y);
}
#[inline]
unsafe extern "C" fn glyf_Point_copyReplace(mut dst: *mut glyf_Point, src: glyf_Point) {
    glyf_Point_dispose(dst);
    glyf_Point_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_Point_dispose(mut x: *mut glyf_Point) {
    disposePoint(x);
}
#[inline]
unsafe extern "C" fn glyf_Point_empty() -> glyf_Point {
    let mut x: glyf_Point = glyf_Point {
        x: VQ {
            kernel: 0.,
            shift: vq_SegList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vq_Segment>(),
            },
        },
        y: VQ {
            kernel: 0.,
            shift: vq_SegList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vq_Segment>(),
            },
        },
        onCurve: 0,
    };
    glyf_Point_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn glyf_Point_replace(mut dst: *mut glyf_Point, src: glyf_Point) {
    glyf_Point_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_Point>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_Point_init(mut x: *mut glyf_Point) {
    createPoint(x);
}
#[no_mangle]
pub static glyf_iPoint: __caryll_elementinterface_glyf_Point = {
    __caryll_elementinterface_glyf_Point {
        init: Some(glyf_Point_init as unsafe extern "C" fn(*mut glyf_Point) -> ()),
        copy: Some(
            glyf_Point_copy as unsafe extern "C" fn(*mut glyf_Point, *const glyf_Point) -> (),
        ),
        move_0: Some(
            glyf_Point_move as unsafe extern "C" fn(*mut glyf_Point, *mut glyf_Point) -> (),
        ),
        dispose: Some(glyf_Point_dispose as unsafe extern "C" fn(*mut glyf_Point) -> ()),
        replace: Some(
            glyf_Point_replace as unsafe extern "C" fn(*mut glyf_Point, glyf_Point) -> (),
        ),
        copyReplace: Some(
            glyf_Point_copyReplace as unsafe extern "C" fn(*mut glyf_Point, glyf_Point) -> (),
        ),
        empty: Some(glyf_Point_empty),
        dup: Some(glyf_Point_dup as unsafe extern "C" fn(glyf_Point) -> glyf_Point),
    }
};
#[inline]
unsafe extern "C" fn glyf_Point_dup(src: glyf_Point) -> glyf_Point {
    let mut dst: glyf_Point = glyf_Point {
        x: VQ {
            kernel: 0.,
            shift: vq_SegList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vq_Segment>(),
            },
        },
        y: VQ {
            kernel: 0.,
            shift: vq_SegList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vq_Segment>(),
            },
        },
        onCurve: 0,
    };
    glyf_Point_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn glyf_Point_move(mut dst: *mut glyf_Point, mut src: *mut glyf_Point) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_Point>() as usize,
    );
    glyf_Point_init(src);
}
#[inline]
unsafe extern "C" fn glyf_Point_copy(mut dst: *mut glyf_Point, mut src: *const glyf_Point) {
    copyPoint(dst, src);
}
#[inline]
unsafe extern "C" fn glyf_Contour_free(mut x: *mut glyf_Contour) {
    if x.is_null() {
        return;
    }
    glyf_Contour_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn glyf_Contour_initN(mut arr: *mut glyf_Contour, mut n: usize) {
    glyf_Contour_init(arr);
    glyf_Contour_growToN(arr, n);
    glyf_Contour_fill(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_Contour_createN(mut n: usize) -> *mut glyf_Contour {
    let mut t: *mut glyf_Contour =
        malloc(::core::mem::size_of::<glyf_Contour>() as usize) as *mut glyf_Contour;
    glyf_Contour_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn glyf_Contour_create() -> *mut glyf_Contour {
    let mut x: *mut glyf_Contour =
        malloc(::core::mem::size_of::<glyf_Contour>() as usize) as *mut glyf_Contour;
    glyf_Contour_init(x);
    return x;
}
#[no_mangle]
pub static glyf_iContour: __caryll_vectorinterface_glyf_Contour = {
    __caryll_vectorinterface_glyf_Contour {
        init: Some(glyf_Contour_init as unsafe extern "C" fn(*mut glyf_Contour) -> ()),
        copy: Some(
            glyf_Contour_copy as unsafe extern "C" fn(*mut glyf_Contour, *const glyf_Contour) -> (),
        ),
        move_0: Some(
            glyf_Contour_move as unsafe extern "C" fn(*mut glyf_Contour, *mut glyf_Contour) -> (),
        ),
        dispose: Some(glyf_Contour_dispose as unsafe extern "C" fn(*mut glyf_Contour) -> ()),
        replace: Some(
            glyf_Contour_replace as unsafe extern "C" fn(*mut glyf_Contour, glyf_Contour) -> (),
        ),
        copyReplace: Some(
            glyf_Contour_copyReplace as unsafe extern "C" fn(*mut glyf_Contour, glyf_Contour) -> (),
        ),
        create: Some(glyf_Contour_create),
        free: Some(glyf_Contour_free as unsafe extern "C" fn(*mut glyf_Contour) -> ()),
        initN: Some(glyf_Contour_initN as unsafe extern "C" fn(*mut glyf_Contour, usize) -> ()),
        initCapN: Some(
            glyf_Contour_initCapN as unsafe extern "C" fn(*mut glyf_Contour, usize) -> (),
        ),
        createN: Some(glyf_Contour_createN as unsafe extern "C" fn(usize) -> *mut glyf_Contour),
        fill: Some(glyf_Contour_fill as unsafe extern "C" fn(*mut glyf_Contour, usize) -> ()),
        clear: Some(glyf_Contour_dispose as unsafe extern "C" fn(*mut glyf_Contour) -> ()),
        push: Some(glyf_Contour_push as unsafe extern "C" fn(*mut glyf_Contour, glyf_Point) -> ()),
        shrinkToFit: Some(
            glyf_Contour_shrinkToFit as unsafe extern "C" fn(*mut glyf_Contour) -> (),
        ),
        pop: Some(glyf_Contour_pop as unsafe extern "C" fn(*mut glyf_Contour) -> glyf_Point),
        disposeItem: Some(
            glyf_Contour_disposeItem as unsafe extern "C" fn(*mut glyf_Contour, usize) -> (),
        ),
        filterEnv: Some(
            glyf_Contour_filterEnv
                as unsafe extern "C" fn(
                    *mut glyf_Contour,
                    Option<
                        unsafe extern "C" fn(*const glyf_Point, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            glyf_Contour_sort
                as unsafe extern "C" fn(
                    *mut glyf_Contour,
                    Option<
                        unsafe extern "C" fn(
                            *const glyf_Point,
                            *const glyf_Point,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_Contour_shrinkToFit(mut arr: *mut glyf_Contour) {
    glyf_Contour_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn glyf_Contour_resizeTo(arr: *mut glyf_Contour, target: usize) {
    cvec_resize_to(glyf_Contour_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_Contour_move(dst: *mut glyf_Contour, src: *mut glyf_Contour) {
    cvec_move(glyf_Contour_as_cvec(dst), glyf_Contour_as_cvec(src));
}
#[inline]
unsafe fn glyf_Contour_as_cvec(arr: *mut glyf_Contour) -> *mut CVecRaw<glyf_Point> {
    arr as *mut CVecRaw<glyf_Point>
}
#[inline]
unsafe extern "C" fn glyf_Contour_init(arr: *mut glyf_Contour) {
    cvec_init(glyf_Contour_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_Contour_filterEnv(
    mut arr: *mut glyf_Contour,
    mut fn_0: Option<unsafe extern "C" fn(*const glyf_Point, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut glyf_Point,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if glyf_iPoint.dispose.is_some() {
                glyf_iPoint.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut glyf_Point,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn glyf_Contour_disposeItem(mut arr: *mut glyf_Contour, mut n: usize) {
    if glyf_iPoint.dispose.is_some() {
        glyf_iPoint.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut glyf_Point
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn glyf_Contour_sort(
    mut arr: *mut glyf_Contour,
    mut fn_0: Option<
        unsafe extern "C" fn(*const glyf_Point, *const glyf_Point) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<glyf_Point>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const glyf_Point, *const glyf_Point) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn glyf_Contour_fill(mut arr: *mut glyf_Contour, mut n: usize) {
    while (*arr).length < n {
        let mut x: glyf_Point = glyf_Point {
            x: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            onCurve: 0,
        };
        if glyf_iPoint.init.is_some() {
            glyf_iPoint.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<glyf_Point>() as usize,
            );
        }
        glyf_Contour_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn glyf_Contour_push(arr: *mut glyf_Contour, elem: glyf_Point) {
    cvec_push(glyf_Contour_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_Contour_grow(arr: *mut glyf_Contour) {
    cvec_grow(glyf_Contour_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_Contour_growTo(arr: *mut glyf_Contour, target: usize) {
    cvec_grow_to(glyf_Contour_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_Contour_pop(arr: *mut glyf_Contour) -> glyf_Point {
    cvec_pop(glyf_Contour_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn glyf_Contour_copyReplace(mut dst: *mut glyf_Contour, src: glyf_Contour) {
    glyf_Contour_dispose(dst);
    glyf_Contour_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_Contour_copy(mut dst: *mut glyf_Contour, mut src: *const glyf_Contour) {
    glyf_Contour_init(dst);
    glyf_Contour_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if glyf_iPoint.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            glyf_iPoint.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut glyf_Point,
                (*src).items.offset(j as isize) as *mut glyf_Point as *const glyf_Point,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn glyf_Contour_dispose(mut arr: *mut glyf_Contour) {
    if arr.is_null() {
        return;
    }
    if glyf_iPoint.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            glyf_iPoint.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut glyf_Point,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<glyf_Point>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn glyf_Contour_replace(mut dst: *mut glyf_Contour, src: glyf_Contour) {
    glyf_Contour_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_Contour>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_Contour_initCapN(mut arr: *mut glyf_Contour, mut n: usize) {
    glyf_Contour_init(arr);
    glyf_Contour_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_Contour_growToN(arr: *mut glyf_Contour, target: usize) {
    cvec_grow_to_n(glyf_Contour_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_ContourList_initN(mut arr: *mut glyf_ContourList, mut n: usize) {
    glyf_ContourList_init(arr);
    glyf_ContourList_growToN(arr, n);
    glyf_ContourList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_ContourList_filterEnv(
    mut arr: *mut glyf_ContourList,
    mut fn_0: Option<unsafe extern "C" fn(*const glyf_Contour, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut glyf_Contour,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if glyf_iContour.dispose.is_some() {
                glyf_iContour.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut glyf_Contour,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn glyf_ContourList_disposeItem(mut arr: *mut glyf_ContourList, mut n: usize) {
    if glyf_iContour.dispose.is_some() {
        glyf_iContour.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut glyf_Contour
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn glyf_ContourList_replace(
    mut dst: *mut glyf_ContourList,
    src: glyf_ContourList,
) {
    glyf_ContourList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_ContourList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_ContourList_copyReplace(
    mut dst: *mut glyf_ContourList,
    src: glyf_ContourList,
) {
    glyf_ContourList_dispose(dst);
    glyf_ContourList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_ContourList_growToN(arr: *mut glyf_ContourList, target: usize) {
    cvec_grow_to_n(glyf_ContourList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_ContourList_copy(
    mut dst: *mut glyf_ContourList,
    mut src: *const glyf_ContourList,
) {
    glyf_ContourList_init(dst);
    glyf_ContourList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if glyf_iContour.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            glyf_iContour.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut glyf_Contour,
                (*src).items.offset(j as isize) as *mut glyf_Contour as *const glyf_Contour,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn glyf_ContourList_free(mut x: *mut glyf_ContourList) {
    if x.is_null() {
        return;
    }
    glyf_ContourList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn glyf_ContourList_sort(
    mut arr: *mut glyf_ContourList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const glyf_Contour, *const glyf_Contour) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<glyf_Contour>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const glyf_Contour,
                    *const glyf_Contour,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn glyf_ContourList_createN(mut n: usize) -> *mut glyf_ContourList {
    let mut t: *mut glyf_ContourList =
        malloc(::core::mem::size_of::<glyf_ContourList>() as usize) as *mut glyf_ContourList;
    glyf_ContourList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn glyf_ContourList_create() -> *mut glyf_ContourList {
    let mut x: *mut glyf_ContourList =
        malloc(::core::mem::size_of::<glyf_ContourList>() as usize) as *mut glyf_ContourList;
    glyf_ContourList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn glyf_ContourList_dispose(mut arr: *mut glyf_ContourList) {
    if arr.is_null() {
        return;
    }
    if glyf_iContour.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            glyf_iContour.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut glyf_Contour,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<glyf_Contour>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[no_mangle]
pub static glyf_iContourList: __caryll_vectorinterface_glyf_ContourList = {
    __caryll_vectorinterface_glyf_ContourList {
        init: Some(glyf_ContourList_init as unsafe extern "C" fn(*mut glyf_ContourList) -> ()),
        copy: Some(
            glyf_ContourList_copy
                as unsafe extern "C" fn(*mut glyf_ContourList, *const glyf_ContourList) -> (),
        ),
        move_0: Some(
            glyf_ContourList_move
                as unsafe extern "C" fn(*mut glyf_ContourList, *mut glyf_ContourList) -> (),
        ),
        dispose: Some(
            glyf_ContourList_dispose as unsafe extern "C" fn(*mut glyf_ContourList) -> (),
        ),
        replace: Some(
            glyf_ContourList_replace
                as unsafe extern "C" fn(*mut glyf_ContourList, glyf_ContourList) -> (),
        ),
        copyReplace: Some(
            glyf_ContourList_copyReplace
                as unsafe extern "C" fn(*mut glyf_ContourList, glyf_ContourList) -> (),
        ),
        create: Some(glyf_ContourList_create),
        free: Some(glyf_ContourList_free as unsafe extern "C" fn(*mut glyf_ContourList) -> ()),
        initN: Some(
            glyf_ContourList_initN as unsafe extern "C" fn(*mut glyf_ContourList, usize) -> (),
        ),
        initCapN: Some(
            glyf_ContourList_initCapN as unsafe extern "C" fn(*mut glyf_ContourList, usize) -> (),
        ),
        createN: Some(
            glyf_ContourList_createN as unsafe extern "C" fn(usize) -> *mut glyf_ContourList,
        ),
        fill: Some(
            glyf_ContourList_fill as unsafe extern "C" fn(*mut glyf_ContourList, usize) -> (),
        ),
        clear: Some(glyf_ContourList_dispose as unsafe extern "C" fn(*mut glyf_ContourList) -> ()),
        push: Some(
            glyf_ContourList_push
                as unsafe extern "C" fn(*mut glyf_ContourList, glyf_Contour) -> (),
        ),
        shrinkToFit: Some(
            glyf_ContourList_shrinkToFit as unsafe extern "C" fn(*mut glyf_ContourList) -> (),
        ),
        pop: Some(
            glyf_ContourList_pop as unsafe extern "C" fn(*mut glyf_ContourList) -> glyf_Contour,
        ),
        disposeItem: Some(
            glyf_ContourList_disposeItem
                as unsafe extern "C" fn(*mut glyf_ContourList, usize) -> (),
        ),
        filterEnv: Some(
            glyf_ContourList_filterEnv
                as unsafe extern "C" fn(
                    *mut glyf_ContourList,
                    Option<
                        unsafe extern "C" fn(*const glyf_Contour, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            glyf_ContourList_sort
                as unsafe extern "C" fn(
                    *mut glyf_ContourList,
                    Option<
                        unsafe extern "C" fn(
                            *const glyf_Contour,
                            *const glyf_Contour,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_ContourList_shrinkToFit(mut arr: *mut glyf_ContourList) {
    glyf_ContourList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn glyf_ContourList_fill(mut arr: *mut glyf_ContourList, mut n: usize) {
    while (*arr).length < n {
        let mut x: glyf_Contour = glyf_Contour {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<glyf_Point>(),
        };
        if glyf_iContour.init.is_some() {
            glyf_iContour.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<glyf_Contour>() as usize,
            );
        }
        glyf_ContourList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn glyf_ContourList_push(arr: *mut glyf_ContourList, elem: glyf_Contour) {
    cvec_push(glyf_ContourList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_ContourList_grow(arr: *mut glyf_ContourList) {
    cvec_grow(glyf_ContourList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_ContourList_growTo(arr: *mut glyf_ContourList, target: usize) {
    cvec_grow_to(glyf_ContourList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_ContourList_pop(arr: *mut glyf_ContourList) -> glyf_Contour {
    cvec_pop(glyf_ContourList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn glyf_ContourList_resizeTo(arr: *mut glyf_ContourList, target: usize) {
    cvec_resize_to(glyf_ContourList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_ContourList_move(dst: *mut glyf_ContourList, src: *mut glyf_ContourList) {
    cvec_move(glyf_ContourList_as_cvec(dst), glyf_ContourList_as_cvec(src));
}
#[inline]
unsafe fn glyf_ContourList_as_cvec(arr: *mut glyf_ContourList) -> *mut CVecRaw<glyf_Contour> {
    arr as *mut CVecRaw<glyf_Contour>
}
#[inline]
unsafe extern "C" fn glyf_ContourList_init(arr: *mut glyf_ContourList) {
    cvec_init(glyf_ContourList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_ContourList_initCapN(mut arr: *mut glyf_ContourList, mut n: usize) {
    glyf_ContourList_init(arr);
    glyf_ContourList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn initGlyfReference(mut ref_0: *mut glyf_ComponentReference) {
    (*ref_0).glyph = otfcc_Handle_empty() as otfcc_GlyphHandle;
    (*ref_0).x =
        iVQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as pos_t);
    (*ref_0).y =
        iVQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as pos_t);
    (*ref_0).a = 1 as ::core::ffi::c_int as scale_t;
    (*ref_0).b = 0 as ::core::ffi::c_int as scale_t;
    (*ref_0).c = 0 as ::core::ffi::c_int as scale_t;
    (*ref_0).d = 1 as ::core::ffi::c_int as scale_t;
    (*ref_0).isAnchored = REF_XY;
    (*ref_0).outer = 0 as shapeid_t;
    (*ref_0).inner = (*ref_0).outer;
    (*ref_0).roundToGrid = false;
    (*ref_0).useMyMetrics = false;
}
unsafe extern "C" fn copyGlyfReference(
    mut dst: *mut glyf_ComponentReference,
    mut src: *const glyf_ComponentReference,
) {
    iVQ.copy.expect("non-null function pointer")(&raw mut (*dst).x, &raw const (*src).x);
    iVQ.copy.expect("non-null function pointer")(&raw mut (*dst).y, &raw const (*src).y);
    otfcc_Handle_copy(
        &raw mut (*dst).glyph,
        &raw const (*src).glyph,
    );
    (*dst).a = (*src).a;
    (*dst).b = (*src).b;
    (*dst).c = (*src).c;
    (*dst).d = (*src).d;
    (*dst).isAnchored = (*src).isAnchored;
    (*dst).inner = (*src).inner;
    (*dst).outer = (*src).outer;
    (*dst).roundToGrid = (*src).roundToGrid;
    (*dst).useMyMetrics = (*src).useMyMetrics;
}
#[inline]
unsafe extern "C" fn disposeGlyfReference(mut ref_0: *mut glyf_ComponentReference) {
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*ref_0).x);
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*ref_0).y);
    otfcc_Handle_dispose(&raw mut (*ref_0).glyph);
}
#[inline]
unsafe extern "C" fn glyf_ComponentReference_dispose(mut x: *mut glyf_ComponentReference) {
    disposeGlyfReference(x);
}
#[inline]
unsafe extern "C" fn glyf_ComponentReference_replace(
    mut dst: *mut glyf_ComponentReference,
    src: glyf_ComponentReference,
) {
    glyf_ComponentReference_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_ComponentReference>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_ComponentReference_dup(
    src: glyf_ComponentReference,
) -> glyf_ComponentReference {
    let mut dst: glyf_ComponentReference = glyf_ComponentReference {
        x: VQ {
            kernel: 0.,
            shift: vq_SegList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vq_Segment>(),
            },
        },
        y: VQ {
            kernel: 0.,
            shift: vq_SegList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vq_Segment>(),
            },
        },
        roundToGrid: false,
        useMyMetrics: false,
        glyph: otfcc_Handle {
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
    glyf_ComponentReference_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn glyf_ComponentReference_copy(
    mut dst: *mut glyf_ComponentReference,
    mut src: *const glyf_ComponentReference,
) {
    copyGlyfReference(dst, src);
}
#[inline]
unsafe extern "C" fn glyf_ComponentReference_move(
    mut dst: *mut glyf_ComponentReference,
    mut src: *mut glyf_ComponentReference,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_ComponentReference>() as usize,
    );
    glyf_ComponentReference_init(src);
}
#[inline]
unsafe extern "C" fn glyf_ComponentReference_empty() -> glyf_ComponentReference {
    let mut x: glyf_ComponentReference = glyf_ComponentReference {
        x: VQ {
            kernel: 0.,
            shift: vq_SegList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vq_Segment>(),
            },
        },
        y: VQ {
            kernel: 0.,
            shift: vq_SegList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vq_Segment>(),
            },
        },
        roundToGrid: false,
        useMyMetrics: false,
        glyph: otfcc_Handle {
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
    glyf_ComponentReference_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn glyf_ComponentReference_init(mut x: *mut glyf_ComponentReference) {
    initGlyfReference(x);
}
#[no_mangle]
pub static glyf_iComponentReference: __caryll_elementinterface_glyf_ComponentReference = {
    __caryll_elementinterface_glyf_ComponentReference {
        init: Some(
            glyf_ComponentReference_init
                as unsafe extern "C" fn(*mut glyf_ComponentReference) -> (),
        ),
        copy: Some(
            glyf_ComponentReference_copy
                as unsafe extern "C" fn(
                    *mut glyf_ComponentReference,
                    *const glyf_ComponentReference,
                ) -> (),
        ),
        move_0: Some(
            glyf_ComponentReference_move
                as unsafe extern "C" fn(
                    *mut glyf_ComponentReference,
                    *mut glyf_ComponentReference,
                ) -> (),
        ),
        dispose: Some(
            glyf_ComponentReference_dispose
                as unsafe extern "C" fn(*mut glyf_ComponentReference) -> (),
        ),
        replace: Some(
            glyf_ComponentReference_replace
                as unsafe extern "C" fn(
                    *mut glyf_ComponentReference,
                    glyf_ComponentReference,
                ) -> (),
        ),
        copyReplace: Some(
            glyf_ComponentReference_copyReplace
                as unsafe extern "C" fn(
                    *mut glyf_ComponentReference,
                    glyf_ComponentReference,
                ) -> (),
        ),
        empty: Some(glyf_ComponentReference_empty),
        dup: Some(
            glyf_ComponentReference_dup
                as unsafe extern "C" fn(glyf_ComponentReference) -> glyf_ComponentReference,
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_ComponentReference_copyReplace(
    mut dst: *mut glyf_ComponentReference,
    src: glyf_ComponentReference,
) {
    glyf_ComponentReference_dispose(dst);
    glyf_ComponentReference_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_copyReplace(
    mut dst: *mut glyf_ReferenceList,
    src: glyf_ReferenceList,
) {
    glyf_ReferenceList_dispose(dst);
    glyf_ReferenceList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_initN(mut arr: *mut glyf_ReferenceList, mut n: usize) {
    glyf_ReferenceList_init(arr);
    glyf_ReferenceList_growToN(arr, n);
    glyf_ReferenceList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_fill(mut arr: *mut glyf_ReferenceList, mut n: usize) {
    while (*arr).length < n {
        let mut x: glyf_ComponentReference = glyf_ComponentReference {
            x: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            roundToGrid: false,
            useMyMetrics: false,
            glyph: otfcc_Handle {
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
        if glyf_iComponentReference.init.is_some() {
            glyf_iComponentReference
                .init
                .expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<glyf_ComponentReference>() as usize,
            );
        }
        glyf_ReferenceList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_push(arr: *mut glyf_ReferenceList, elem: glyf_ComponentReference) {
    cvec_push(glyf_ReferenceList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_grow(arr: *mut glyf_ReferenceList) {
    cvec_grow(glyf_ReferenceList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_growTo(arr: *mut glyf_ReferenceList, target: usize) {
    cvec_grow_to(glyf_ReferenceList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_pop(arr: *mut glyf_ReferenceList) -> glyf_ComponentReference {
    cvec_pop(glyf_ReferenceList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_dispose(mut arr: *mut glyf_ReferenceList) {
    if arr.is_null() {
        return;
    }
    if glyf_iComponentReference.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh9 = j;
            j = j.wrapping_sub(1);
            if !(fresh9 != 0) {
                break;
            }
            glyf_iComponentReference
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut glyf_ComponentReference
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<glyf_ComponentReference>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_copy(
    mut dst: *mut glyf_ReferenceList,
    mut src: *const glyf_ReferenceList,
) {
    glyf_ReferenceList_init(dst);
    glyf_ReferenceList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if glyf_iComponentReference.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            glyf_iComponentReference
                .copy
                .expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut glyf_ComponentReference,
                (*src).items.offset(j as isize) as *mut glyf_ComponentReference
                    as *const glyf_ComponentReference,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_initCapN(mut arr: *mut glyf_ReferenceList, mut n: usize) {
    glyf_ReferenceList_init(arr);
    glyf_ReferenceList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_replace(
    mut dst: *mut glyf_ReferenceList,
    src: glyf_ReferenceList,
) {
    glyf_ReferenceList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_ReferenceList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_growToN(arr: *mut glyf_ReferenceList, target: usize) {
    cvec_grow_to_n(glyf_ReferenceList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_free(mut x: *mut glyf_ReferenceList) {
    if x.is_null() {
        return;
    }
    glyf_ReferenceList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_createN(mut n: usize) -> *mut glyf_ReferenceList {
    let mut t: *mut glyf_ReferenceList =
        malloc(::core::mem::size_of::<glyf_ReferenceList>() as usize) as *mut glyf_ReferenceList;
    glyf_ReferenceList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_create() -> *mut glyf_ReferenceList {
    let mut x: *mut glyf_ReferenceList =
        malloc(::core::mem::size_of::<glyf_ReferenceList>() as usize) as *mut glyf_ReferenceList;
    glyf_ReferenceList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_move(dst: *mut glyf_ReferenceList, src: *mut glyf_ReferenceList) {
    cvec_move(glyf_ReferenceList_as_cvec(dst), glyf_ReferenceList_as_cvec(src));
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_shrinkToFit(mut arr: *mut glyf_ReferenceList) {
    glyf_ReferenceList_resizeTo(arr, (*arr).length);
}
#[no_mangle]
pub static glyf_iReferenceList: __caryll_vectorinterface_glyf_ReferenceList = {
    __caryll_vectorinterface_glyf_ReferenceList {
        init: Some(glyf_ReferenceList_init as unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()),
        copy: Some(
            glyf_ReferenceList_copy
                as unsafe extern "C" fn(*mut glyf_ReferenceList, *const glyf_ReferenceList) -> (),
        ),
        move_0: Some(
            glyf_ReferenceList_move
                as unsafe extern "C" fn(*mut glyf_ReferenceList, *mut glyf_ReferenceList) -> (),
        ),
        dispose: Some(
            glyf_ReferenceList_dispose as unsafe extern "C" fn(*mut glyf_ReferenceList) -> (),
        ),
        replace: Some(
            glyf_ReferenceList_replace
                as unsafe extern "C" fn(*mut glyf_ReferenceList, glyf_ReferenceList) -> (),
        ),
        copyReplace: Some(
            glyf_ReferenceList_copyReplace
                as unsafe extern "C" fn(*mut glyf_ReferenceList, glyf_ReferenceList) -> (),
        ),
        create: Some(glyf_ReferenceList_create),
        free: Some(glyf_ReferenceList_free as unsafe extern "C" fn(*mut glyf_ReferenceList) -> ()),
        initN: Some(
            glyf_ReferenceList_initN as unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> (),
        ),
        initCapN: Some(
            glyf_ReferenceList_initCapN
                as unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> (),
        ),
        createN: Some(
            glyf_ReferenceList_createN as unsafe extern "C" fn(usize) -> *mut glyf_ReferenceList,
        ),
        fill: Some(
            glyf_ReferenceList_fill as unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> (),
        ),
        clear: Some(
            glyf_ReferenceList_dispose as unsafe extern "C" fn(*mut glyf_ReferenceList) -> (),
        ),
        push: Some(
            glyf_ReferenceList_push
                as unsafe extern "C" fn(*mut glyf_ReferenceList, glyf_ComponentReference) -> (),
        ),
        shrinkToFit: Some(
            glyf_ReferenceList_shrinkToFit as unsafe extern "C" fn(*mut glyf_ReferenceList) -> (),
        ),
        pop: Some(
            glyf_ReferenceList_pop
                as unsafe extern "C" fn(*mut glyf_ReferenceList) -> glyf_ComponentReference,
        ),
        disposeItem: Some(
            glyf_ReferenceList_disposeItem
                as unsafe extern "C" fn(*mut glyf_ReferenceList, usize) -> (),
        ),
        filterEnv: Some(
            glyf_ReferenceList_filterEnv
                as unsafe extern "C" fn(
                    *mut glyf_ReferenceList,
                    Option<
                        unsafe extern "C" fn(
                            *const glyf_ComponentReference,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            glyf_ReferenceList_sort
                as unsafe extern "C" fn(
                    *mut glyf_ReferenceList,
                    Option<
                        unsafe extern "C" fn(
                            *const glyf_ComponentReference,
                            *const glyf_ComponentReference,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe fn glyf_ReferenceList_as_cvec(arr: *mut glyf_ReferenceList) -> *mut CVecRaw<glyf_ComponentReference> {
    arr as *mut CVecRaw<glyf_ComponentReference>
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_init(arr: *mut glyf_ReferenceList) {
    cvec_init(glyf_ReferenceList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_filterEnv(
    mut arr: *mut glyf_ReferenceList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const glyf_ComponentReference, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut glyf_ComponentReference,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if glyf_iComponentReference.dispose.is_some() {
                glyf_iComponentReference
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut glyf_ComponentReference,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_resizeTo(arr: *mut glyf_ReferenceList, target: usize) {
    cvec_resize_to(glyf_ReferenceList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_disposeItem(
    mut arr: *mut glyf_ReferenceList,
    mut n: usize,
) {
    if glyf_iComponentReference.dispose.is_some() {
        glyf_iComponentReference
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut glyf_ComponentReference
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn glyf_ReferenceList_sort(
    mut arr: *mut glyf_ReferenceList,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const glyf_ComponentReference,
            *const glyf_ComponentReference,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<glyf_ComponentReference>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const glyf_ComponentReference,
                    *const glyf_ComponentReference,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn glyf_PostscriptStemDef_copyReplace(
    mut dst: *mut glyf_PostscriptStemDef,
    src: glyf_PostscriptStemDef,
) {
    glyf_PostscriptStemDef_dispose(dst);
    glyf_PostscriptStemDef_copy(dst, &raw const src);
}
#[no_mangle]
pub static glyf_iPostscriptStemDef: __caryll_elementinterface_glyf_PostscriptStemDef = {
    __caryll_elementinterface_glyf_PostscriptStemDef {
        init: Some(
            glyf_PostscriptStemDef_init as unsafe extern "C" fn(*mut glyf_PostscriptStemDef) -> (),
        ),
        copy: Some(
            glyf_PostscriptStemDef_copy
                as unsafe extern "C" fn(
                    *mut glyf_PostscriptStemDef,
                    *const glyf_PostscriptStemDef,
                ) -> (),
        ),
        move_0: Some(
            glyf_PostscriptStemDef_move
                as unsafe extern "C" fn(
                    *mut glyf_PostscriptStemDef,
                    *mut glyf_PostscriptStemDef,
                ) -> (),
        ),
        dispose: Some(
            glyf_PostscriptStemDef_dispose
                as unsafe extern "C" fn(*mut glyf_PostscriptStemDef) -> (),
        ),
        replace: Some(
            glyf_PostscriptStemDef_replace
                as unsafe extern "C" fn(*mut glyf_PostscriptStemDef, glyf_PostscriptStemDef) -> (),
        ),
        copyReplace: Some(
            glyf_PostscriptStemDef_copyReplace
                as unsafe extern "C" fn(*mut glyf_PostscriptStemDef, glyf_PostscriptStemDef) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_PostscriptStemDef_replace(
    mut dst: *mut glyf_PostscriptStemDef,
    src: glyf_PostscriptStemDef,
) {
    glyf_PostscriptStemDef_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_PostscriptStemDef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_PostscriptStemDef_move(
    mut dst: *mut glyf_PostscriptStemDef,
    mut src: *mut glyf_PostscriptStemDef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_PostscriptStemDef>() as usize,
    );
    glyf_PostscriptStemDef_init(src);
}
#[inline]
unsafe extern "C" fn glyf_PostscriptStemDef_copy(
    mut dst: *mut glyf_PostscriptStemDef,
    mut src: *const glyf_PostscriptStemDef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_PostscriptStemDef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_PostscriptStemDef_dispose(mut _x: *mut glyf_PostscriptStemDef) {}
#[inline]
unsafe extern "C" fn glyf_PostscriptStemDef_init(mut x: *mut glyf_PostscriptStemDef) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<glyf_PostscriptStemDef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_createN(mut n: usize) -> *mut glyf_StemDefList {
    let mut t: *mut glyf_StemDefList =
        malloc(::core::mem::size_of::<glyf_StemDefList>() as usize) as *mut glyf_StemDefList;
    glyf_StemDefList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_shrinkToFit(mut arr: *mut glyf_StemDefList) {
    glyf_StemDefList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_resizeTo(arr: *mut glyf_StemDefList, target: usize) {
    cvec_resize_to(glyf_StemDefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_move(dst: *mut glyf_StemDefList, src: *mut glyf_StemDefList) {
    cvec_move(glyf_StemDefList_as_cvec(dst), glyf_StemDefList_as_cvec(src));
}
#[inline]
unsafe fn glyf_StemDefList_as_cvec(arr: *mut glyf_StemDefList) -> *mut CVecRaw<glyf_PostscriptStemDef> {
    arr as *mut CVecRaw<glyf_PostscriptStemDef>
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_init(arr: *mut glyf_StemDefList) {
    cvec_init(glyf_StemDefList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_filterEnv(
    mut arr: *mut glyf_StemDefList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const glyf_PostscriptStemDef, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut glyf_PostscriptStemDef,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if glyf_iPostscriptStemDef.dispose.is_some() {
                glyf_iPostscriptStemDef
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut glyf_PostscriptStemDef,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_disposeItem(mut arr: *mut glyf_StemDefList, mut n: usize) {
    if glyf_iPostscriptStemDef.dispose.is_some() {
        glyf_iPostscriptStemDef
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut glyf_PostscriptStemDef
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_sort(
    mut arr: *mut glyf_StemDefList,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const glyf_PostscriptStemDef,
            *const glyf_PostscriptStemDef,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<glyf_PostscriptStemDef>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const glyf_PostscriptStemDef,
                    *const glyf_PostscriptStemDef,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_fill(mut arr: *mut glyf_StemDefList, mut n: usize) {
    while (*arr).length < n {
        let mut x: glyf_PostscriptStemDef = glyf_PostscriptStemDef {
            position: 0.,
            width: 0.,
            map: 0,
        };
        if glyf_iPostscriptStemDef.init.is_some() {
            glyf_iPostscriptStemDef
                .init
                .expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<glyf_PostscriptStemDef>() as usize,
            );
        }
        glyf_StemDefList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_push(arr: *mut glyf_StemDefList, elem: glyf_PostscriptStemDef) {
    cvec_push(glyf_StemDefList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_grow(arr: *mut glyf_StemDefList) {
    cvec_grow(glyf_StemDefList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_growTo(arr: *mut glyf_StemDefList, target: usize) {
    cvec_grow_to(glyf_StemDefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_pop(arr: *mut glyf_StemDefList) -> glyf_PostscriptStemDef {
    cvec_pop(glyf_StemDefList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_copyReplace(
    mut dst: *mut glyf_StemDefList,
    src: glyf_StemDefList,
) {
    glyf_StemDefList_dispose(dst);
    glyf_StemDefList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_copy(
    mut dst: *mut glyf_StemDefList,
    mut src: *const glyf_StemDefList,
) {
    glyf_StemDefList_init(dst);
    glyf_StemDefList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if glyf_iPostscriptStemDef.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            glyf_iPostscriptStemDef
                .copy
                .expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut glyf_PostscriptStemDef,
                (*src).items.offset(j as isize) as *mut glyf_PostscriptStemDef
                    as *const glyf_PostscriptStemDef,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_dispose(mut arr: *mut glyf_StemDefList) {
    if arr.is_null() {
        return;
    }
    if glyf_iPostscriptStemDef.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh5 = j;
            j = j.wrapping_sub(1);
            if !(fresh5 != 0) {
                break;
            }
            glyf_iPostscriptStemDef
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut glyf_PostscriptStemDef
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<glyf_PostscriptStemDef>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_replace(
    mut dst: *mut glyf_StemDefList,
    src: glyf_StemDefList,
) {
    glyf_StemDefList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_StemDefList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_initCapN(mut arr: *mut glyf_StemDefList, mut n: usize) {
    glyf_StemDefList_init(arr);
    glyf_StemDefList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_growToN(arr: *mut glyf_StemDefList, target: usize) {
    cvec_grow_to_n(glyf_StemDefList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_initN(mut arr: *mut glyf_StemDefList, mut n: usize) {
    glyf_StemDefList_init(arr);
    glyf_StemDefList_growToN(arr, n);
    glyf_StemDefList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_free(mut x: *mut glyf_StemDefList) {
    if x.is_null() {
        return;
    }
    glyf_StemDefList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn glyf_StemDefList_create() -> *mut glyf_StemDefList {
    let mut x: *mut glyf_StemDefList =
        malloc(::core::mem::size_of::<glyf_StemDefList>() as usize) as *mut glyf_StemDefList;
    glyf_StemDefList_init(x);
    return x;
}
#[no_mangle]
pub static glyf_iStemDefList: __caryll_vectorinterface_glyf_StemDefList = {
    __caryll_vectorinterface_glyf_StemDefList {
        init: Some(glyf_StemDefList_init as unsafe extern "C" fn(*mut glyf_StemDefList) -> ()),
        copy: Some(
            glyf_StemDefList_copy
                as unsafe extern "C" fn(*mut glyf_StemDefList, *const glyf_StemDefList) -> (),
        ),
        move_0: Some(
            glyf_StemDefList_move
                as unsafe extern "C" fn(*mut glyf_StemDefList, *mut glyf_StemDefList) -> (),
        ),
        dispose: Some(
            glyf_StemDefList_dispose as unsafe extern "C" fn(*mut glyf_StemDefList) -> (),
        ),
        replace: Some(
            glyf_StemDefList_replace
                as unsafe extern "C" fn(*mut glyf_StemDefList, glyf_StemDefList) -> (),
        ),
        copyReplace: Some(
            glyf_StemDefList_copyReplace
                as unsafe extern "C" fn(*mut glyf_StemDefList, glyf_StemDefList) -> (),
        ),
        create: Some(glyf_StemDefList_create),
        free: Some(glyf_StemDefList_free as unsafe extern "C" fn(*mut glyf_StemDefList) -> ()),
        initN: Some(
            glyf_StemDefList_initN as unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> (),
        ),
        initCapN: Some(
            glyf_StemDefList_initCapN as unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> (),
        ),
        createN: Some(
            glyf_StemDefList_createN as unsafe extern "C" fn(usize) -> *mut glyf_StemDefList,
        ),
        fill: Some(
            glyf_StemDefList_fill as unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> (),
        ),
        clear: Some(glyf_StemDefList_dispose as unsafe extern "C" fn(*mut glyf_StemDefList) -> ()),
        push: Some(
            glyf_StemDefList_push
                as unsafe extern "C" fn(*mut glyf_StemDefList, glyf_PostscriptStemDef) -> (),
        ),
        shrinkToFit: Some(
            glyf_StemDefList_shrinkToFit as unsafe extern "C" fn(*mut glyf_StemDefList) -> (),
        ),
        pop: Some(
            glyf_StemDefList_pop
                as unsafe extern "C" fn(*mut glyf_StemDefList) -> glyf_PostscriptStemDef,
        ),
        disposeItem: Some(
            glyf_StemDefList_disposeItem
                as unsafe extern "C" fn(*mut glyf_StemDefList, usize) -> (),
        ),
        filterEnv: Some(
            glyf_StemDefList_filterEnv
                as unsafe extern "C" fn(
                    *mut glyf_StemDefList,
                    Option<
                        unsafe extern "C" fn(
                            *const glyf_PostscriptStemDef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            glyf_StemDefList_sort
                as unsafe extern "C" fn(
                    *mut glyf_StemDefList,
                    Option<
                        unsafe extern "C" fn(
                            *const glyf_PostscriptStemDef,
                            *const glyf_PostscriptStemDef,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[no_mangle]
pub static glyf_iPostscriptHintMask: __caryll_elementinterface_glyf_PostscriptHintMask = {
    __caryll_elementinterface_glyf_PostscriptHintMask {
        init: Some(
            glyf_PostscriptHintMask_init
                as unsafe extern "C" fn(*mut glyf_PostscriptHintMask) -> (),
        ),
        copy: Some(
            glyf_PostscriptHintMask_copy
                as unsafe extern "C" fn(
                    *mut glyf_PostscriptHintMask,
                    *const glyf_PostscriptHintMask,
                ) -> (),
        ),
        move_0: Some(
            glyf_PostscriptHintMask_move
                as unsafe extern "C" fn(
                    *mut glyf_PostscriptHintMask,
                    *mut glyf_PostscriptHintMask,
                ) -> (),
        ),
        dispose: Some(
            glyf_PostscriptHintMask_dispose
                as unsafe extern "C" fn(*mut glyf_PostscriptHintMask) -> (),
        ),
        replace: Some(
            glyf_PostscriptHintMask_replace
                as unsafe extern "C" fn(
                    *mut glyf_PostscriptHintMask,
                    glyf_PostscriptHintMask,
                ) -> (),
        ),
        copyReplace: Some(
            glyf_PostscriptHintMask_copyReplace
                as unsafe extern "C" fn(
                    *mut glyf_PostscriptHintMask,
                    glyf_PostscriptHintMask,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_PostscriptHintMask_copyReplace(
    mut dst: *mut glyf_PostscriptHintMask,
    src: glyf_PostscriptHintMask,
) {
    glyf_PostscriptHintMask_dispose(dst);
    glyf_PostscriptHintMask_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_PostscriptHintMask_copy(
    mut dst: *mut glyf_PostscriptHintMask,
    mut src: *const glyf_PostscriptHintMask,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_PostscriptHintMask>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_PostscriptHintMask_dispose(mut _x: *mut glyf_PostscriptHintMask) {}
#[inline]
unsafe extern "C" fn glyf_PostscriptHintMask_replace(
    mut dst: *mut glyf_PostscriptHintMask,
    src: glyf_PostscriptHintMask,
) {
    glyf_PostscriptHintMask_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_PostscriptHintMask>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_PostscriptHintMask_move(
    mut dst: *mut glyf_PostscriptHintMask,
    mut src: *mut glyf_PostscriptHintMask,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_PostscriptHintMask>() as usize,
    );
    glyf_PostscriptHintMask_init(src);
}
#[inline]
unsafe extern "C" fn glyf_PostscriptHintMask_init(mut x: *mut glyf_PostscriptHintMask) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<glyf_PostscriptHintMask>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_MaskList_copy(
    mut dst: *mut glyf_MaskList,
    mut src: *const glyf_MaskList,
) {
    glyf_MaskList_init(dst);
    glyf_MaskList_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if glyf_iPostscriptHintMask.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            glyf_iPostscriptHintMask
                .copy
                .expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut glyf_PostscriptHintMask,
                (*src).items.offset(j as isize) as *mut glyf_PostscriptHintMask
                    as *const glyf_PostscriptHintMask,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[no_mangle]
pub static glyf_iMaskList: __caryll_vectorinterface_glyf_MaskList = {
    __caryll_vectorinterface_glyf_MaskList {
        init: Some(glyf_MaskList_init as unsafe extern "C" fn(*mut glyf_MaskList) -> ()),
        copy: Some(
            glyf_MaskList_copy
                as unsafe extern "C" fn(*mut glyf_MaskList, *const glyf_MaskList) -> (),
        ),
        move_0: Some(
            glyf_MaskList_move
                as unsafe extern "C" fn(*mut glyf_MaskList, *mut glyf_MaskList) -> (),
        ),
        dispose: Some(glyf_MaskList_dispose as unsafe extern "C" fn(*mut glyf_MaskList) -> ()),
        replace: Some(
            glyf_MaskList_replace as unsafe extern "C" fn(*mut glyf_MaskList, glyf_MaskList) -> (),
        ),
        copyReplace: Some(
            glyf_MaskList_copyReplace
                as unsafe extern "C" fn(*mut glyf_MaskList, glyf_MaskList) -> (),
        ),
        create: Some(glyf_MaskList_create),
        free: Some(glyf_MaskList_free as unsafe extern "C" fn(*mut glyf_MaskList) -> ()),
        initN: Some(glyf_MaskList_initN as unsafe extern "C" fn(*mut glyf_MaskList, usize) -> ()),
        initCapN: Some(
            glyf_MaskList_initCapN as unsafe extern "C" fn(*mut glyf_MaskList, usize) -> (),
        ),
        createN: Some(glyf_MaskList_createN as unsafe extern "C" fn(usize) -> *mut glyf_MaskList),
        fill: Some(glyf_MaskList_fill as unsafe extern "C" fn(*mut glyf_MaskList, usize) -> ()),
        clear: Some(glyf_MaskList_dispose as unsafe extern "C" fn(*mut glyf_MaskList) -> ()),
        push: Some(
            glyf_MaskList_push
                as unsafe extern "C" fn(*mut glyf_MaskList, glyf_PostscriptHintMask) -> (),
        ),
        shrinkToFit: Some(
            glyf_MaskList_shrinkToFit as unsafe extern "C" fn(*mut glyf_MaskList) -> (),
        ),
        pop: Some(
            glyf_MaskList_pop
                as unsafe extern "C" fn(*mut glyf_MaskList) -> glyf_PostscriptHintMask,
        ),
        disposeItem: Some(
            glyf_MaskList_disposeItem as unsafe extern "C" fn(*mut glyf_MaskList, usize) -> (),
        ),
        filterEnv: Some(
            glyf_MaskList_filterEnv
                as unsafe extern "C" fn(
                    *mut glyf_MaskList,
                    Option<
                        unsafe extern "C" fn(
                            *const glyf_PostscriptHintMask,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            glyf_MaskList_sort
                as unsafe extern "C" fn(
                    *mut glyf_MaskList,
                    Option<
                        unsafe extern "C" fn(
                            *const glyf_PostscriptHintMask,
                            *const glyf_PostscriptHintMask,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_MaskList_shrinkToFit(mut arr: *mut glyf_MaskList) {
    glyf_MaskList_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn glyf_MaskList_resizeTo(arr: *mut glyf_MaskList, target: usize) {
    cvec_resize_to(glyf_MaskList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_MaskList_move(dst: *mut glyf_MaskList, src: *mut glyf_MaskList) {
    cvec_move(glyf_MaskList_as_cvec(dst), glyf_MaskList_as_cvec(src));
}
#[inline]
unsafe fn glyf_MaskList_as_cvec(arr: *mut glyf_MaskList) -> *mut CVecRaw<glyf_PostscriptHintMask> {
    arr as *mut CVecRaw<glyf_PostscriptHintMask>
}
#[inline]
unsafe extern "C" fn glyf_MaskList_init(arr: *mut glyf_MaskList) {
    cvec_init(glyf_MaskList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_MaskList_filterEnv(
    mut arr: *mut glyf_MaskList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const glyf_PostscriptHintMask, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut glyf_PostscriptHintMask,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if glyf_iPostscriptHintMask.dispose.is_some() {
                glyf_iPostscriptHintMask
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut glyf_PostscriptHintMask,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn glyf_MaskList_disposeItem(mut arr: *mut glyf_MaskList, mut n: usize) {
    if glyf_iPostscriptHintMask.dispose.is_some() {
        glyf_iPostscriptHintMask
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut glyf_PostscriptHintMask
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn glyf_MaskList_sort(
    mut arr: *mut glyf_MaskList,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const glyf_PostscriptHintMask,
            *const glyf_PostscriptHintMask,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<glyf_PostscriptHintMask>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const glyf_PostscriptHintMask,
                    *const glyf_PostscriptHintMask,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn glyf_MaskList_fill(mut arr: *mut glyf_MaskList, mut n: usize) {
    while (*arr).length < n {
        let mut x: glyf_PostscriptHintMask = glyf_PostscriptHintMask {
            pointsBefore: 0,
            contoursBefore: 0,
            maskH: [false; 256],
            maskV: [false; 256],
        };
        if glyf_iPostscriptHintMask.init.is_some() {
            glyf_iPostscriptHintMask
                .init
                .expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<glyf_PostscriptHintMask>() as usize,
            );
        }
        glyf_MaskList_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn glyf_MaskList_push(arr: *mut glyf_MaskList, elem: glyf_PostscriptHintMask) {
    cvec_push(glyf_MaskList_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_MaskList_grow(arr: *mut glyf_MaskList) {
    cvec_grow(glyf_MaskList_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_MaskList_growTo(arr: *mut glyf_MaskList, target: usize) {
    cvec_grow_to(glyf_MaskList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_MaskList_pop(arr: *mut glyf_MaskList) -> glyf_PostscriptHintMask {
    cvec_pop(glyf_MaskList_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn glyf_MaskList_copyReplace(mut dst: *mut glyf_MaskList, src: glyf_MaskList) {
    glyf_MaskList_dispose(dst);
    glyf_MaskList_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_MaskList_initN(mut arr: *mut glyf_MaskList, mut n: usize) {
    glyf_MaskList_init(arr);
    glyf_MaskList_growToN(arr, n);
    glyf_MaskList_fill(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_MaskList_initCapN(mut arr: *mut glyf_MaskList, mut n: usize) {
    glyf_MaskList_init(arr);
    glyf_MaskList_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_MaskList_replace(mut dst: *mut glyf_MaskList, src: glyf_MaskList) {
    glyf_MaskList_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<glyf_MaskList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_MaskList_dispose(mut arr: *mut glyf_MaskList) {
    if arr.is_null() {
        return;
    }
    if glyf_iPostscriptHintMask.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh7 = j;
            j = j.wrapping_sub(1);
            if !(fresh7 != 0) {
                break;
            }
            glyf_iPostscriptHintMask
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut glyf_PostscriptHintMask
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<glyf_PostscriptHintMask>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn glyf_MaskList_growToN(arr: *mut glyf_MaskList, target: usize) {
    cvec_grow_to_n(glyf_MaskList_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_MaskList_create() -> *mut glyf_MaskList {
    let mut x: *mut glyf_MaskList =
        malloc(::core::mem::size_of::<glyf_MaskList>() as usize) as *mut glyf_MaskList;
    glyf_MaskList_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn glyf_MaskList_createN(mut n: usize) -> *mut glyf_MaskList {
    let mut t: *mut glyf_MaskList =
        malloc(::core::mem::size_of::<glyf_MaskList>() as usize) as *mut glyf_MaskList;
    glyf_MaskList_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn glyf_MaskList_free(mut x: *mut glyf_MaskList) {
    if x.is_null() {
        return;
    }
    glyf_MaskList_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_newGlyf_glyph() -> *mut glyf_Glyph {
    let mut g: *mut glyf_Glyph = ::core::ptr::null_mut::<glyf_Glyph>();
    g = __caryll_allocate_clean(
        ::core::mem::size_of::<glyf_Glyph>() as usize,
        78 as ::core::ffi::c_ulong,
    ) as *mut glyf_Glyph;
    (*g).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    iVQ.init.expect("non-null function pointer")(&raw mut (*g).horizontalOrigin);
    iVQ.init.expect("non-null function pointer")(&raw mut (*g).advanceWidth);
    iVQ.init.expect("non-null function pointer")(&raw mut (*g).verticalOrigin);
    iVQ.init.expect("non-null function pointer")(&raw mut (*g).advanceHeight);
    glyf_iContourList.init.expect("non-null function pointer")(&raw mut (*g).contours);
    glyf_iReferenceList.init.expect("non-null function pointer")(&raw mut (*g).references);
    glyf_iStemDefList.init.expect("non-null function pointer")(&raw mut (*g).stemH);
    glyf_iStemDefList.init.expect("non-null function pointer")(&raw mut (*g).stemV);
    glyf_iMaskList.init.expect("non-null function pointer")(&raw mut (*g).hintMasks);
    glyf_iMaskList.init.expect("non-null function pointer")(&raw mut (*g).contourMasks);
    (*g).instructionsLength = 0 as u16;
    (*g).instructions = ::core::ptr::null_mut::<u8>();
    (*g).fdSelect = otfcc_Handle_empty() as otfcc_FDHandle;
    (*g).yPel = 0 as u8;
    (*g).stat.xMin = 0 as ::core::ffi::c_int as pos_t;
    (*g).stat.xMax = 0 as ::core::ffi::c_int as pos_t;
    (*g).stat.yMin = 0 as ::core::ffi::c_int as pos_t;
    (*g).stat.yMax = 0 as ::core::ffi::c_int as pos_t;
    (*g).stat.nestDepth = 0 as u16;
    (*g).stat.nPoints = 0 as u16;
    (*g).stat.nContours = 0 as u16;
    (*g).stat.nCompositePoints = 0 as u16;
    (*g).stat.nCompositeContours = 0 as u16;
    return g;
}
unsafe extern "C" fn otfcc_deleteGlyf_glyph(mut g: *mut glyf_Glyph) {
    if g.is_null() {
        return;
    }
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*g).horizontalOrigin);
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*g).advanceWidth);
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*g).verticalOrigin);
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*g).advanceHeight);
    sdsfree((*g).name);
    glyf_iContourList
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).contours);
    glyf_iReferenceList
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).references);
    glyf_iStemDefList
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).stemH);
    glyf_iStemDefList
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).stemV);
    glyf_iMaskList.dispose.expect("non-null function pointer")(&raw mut (*g).hintMasks);
    glyf_iMaskList.dispose.expect("non-null function pointer")(&raw mut (*g).contourMasks);
    if !(*g).instructions.is_null() {
        free((*g).instructions as *mut ::core::ffi::c_void);
        (*g).instructions = ::core::ptr::null_mut::<u8>();
    }
    otfcc_Handle_dispose(&raw mut (*g).fdSelect);
    (*g).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(g as *mut ::core::ffi::c_void);
    g = ::core::ptr::null_mut::<glyf_Glyph>();
}
#[inline]
unsafe extern "C" fn initGlyfPtr(mut g: *mut glyf_GlyphPtr) {
    *g = ::core::ptr::null_mut::<glyf_Glyph>();
}
unsafe extern "C" fn copyGlyfPtr(mut dst: *mut glyf_GlyphPtr, mut src: *const glyf_GlyphPtr) {
    *dst = *src;
}
#[inline]
unsafe extern "C" fn disposeGlyfPtr(mut g: *mut glyf_GlyphPtr) {
    otfcc_deleteGlyf_glyph(*g);
}
#[no_mangle]
pub static glyf_iGlyphPtr: __caryll_elementinterface_glyf_GlyphPtr = {
    __caryll_elementinterface_glyf_GlyphPtr {
        init: Some(initGlyfPtr as unsafe extern "C" fn(*mut glyf_GlyphPtr) -> ()),
        copy: Some(
            copyGlyfPtr as unsafe extern "C" fn(*mut glyf_GlyphPtr, *const glyf_GlyphPtr) -> (),
        ),
        move_0: None,
        dispose: Some(disposeGlyfPtr as unsafe extern "C" fn(*mut glyf_GlyphPtr) -> ()),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn table_glyf_initN(mut arr: *mut table_glyf, mut n: usize) {
    table_glyf_init(arr);
    table_glyf_growToN(arr, n);
    table_glyf_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_glyf_shrinkToFit(mut arr: *mut table_glyf) {
    table_glyf_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_glyf_resizeTo(arr: *mut table_glyf, target: usize) {
    cvec_resize_to(table_glyf_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_glyf_move(dst: *mut table_glyf, src: *mut table_glyf) {
    cvec_move(table_glyf_as_cvec(dst), table_glyf_as_cvec(src));
}
#[inline]
unsafe fn table_glyf_as_cvec(arr: *mut table_glyf) -> *mut CVecRaw<glyf_GlyphPtr> {
    arr as *mut CVecRaw<glyf_GlyphPtr>
}
#[inline]
unsafe extern "C" fn table_glyf_init(arr: *mut table_glyf) {
    cvec_init(table_glyf_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_glyf_filterEnv(
    mut arr: *mut table_glyf,
    mut fn_0: Option<unsafe extern "C" fn(*const glyf_GlyphPtr, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut glyf_GlyphPtr,
            env,
        ) {
            if j != k {
                let ref mut fresh10 = *(*arr).items.offset(j as isize);
                *fresh10 = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if glyf_iGlyphPtr.dispose.is_some() {
                glyf_iGlyphPtr.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut glyf_GlyphPtr,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_glyf_disposeItem(mut arr: *mut table_glyf, mut n: usize) {
    if glyf_iGlyphPtr.dispose.is_some() {
        glyf_iGlyphPtr.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut glyf_GlyphPtr
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_glyf_sort(
    mut arr: *mut table_glyf,
    mut fn_0: Option<
        unsafe extern "C" fn(*const glyf_GlyphPtr, *const glyf_GlyphPtr) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<glyf_GlyphPtr>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const glyf_GlyphPtr,
                    *const glyf_GlyphPtr,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn table_glyf_fill(mut arr: *mut table_glyf, mut n: usize) {
    while (*arr).length < n {
        let mut x: glyf_GlyphPtr = ::core::ptr::null_mut::<glyf_Glyph>();
        if glyf_iGlyphPtr.init.is_some() {
            glyf_iGlyphPtr.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<glyf_GlyphPtr>() as usize,
            );
        }
        table_glyf_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_glyf_push(arr: *mut table_glyf, elem: glyf_GlyphPtr) {
    cvec_push(table_glyf_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_glyf_grow(arr: *mut table_glyf) {
    cvec_grow(table_glyf_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_glyf_growTo(arr: *mut table_glyf, target: usize) {
    cvec_grow_to(table_glyf_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_glyf_pop(arr: *mut table_glyf) -> glyf_GlyphPtr {
    cvec_pop(table_glyf_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_glyf_copyReplace(mut dst: *mut table_glyf, src: table_glyf) {
    table_glyf_dispose(dst);
    table_glyf_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_glyf_copy(mut dst: *mut table_glyf, mut src: *const table_glyf) {
    table_glyf_init(dst);
    table_glyf_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if glyf_iGlyphPtr.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            glyf_iGlyphPtr.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut glyf_GlyphPtr,
                (*src).items.offset(j as isize) as *mut glyf_GlyphPtr as *const glyf_GlyphPtr,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            let ref mut fresh13 = *(*dst).items.offset(j_0 as isize);
            *fresh13 = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn table_glyf_dispose(mut arr: *mut table_glyf) {
    if arr.is_null() {
        return;
    }
    if glyf_iGlyphPtr.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh14 = j;
            j = j.wrapping_sub(1);
            if !(fresh14 != 0) {
                break;
            }
            glyf_iGlyphPtr.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut glyf_GlyphPtr,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<glyf_GlyphPtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn table_glyf_replace(mut dst: *mut table_glyf, src: table_glyf) {
    table_glyf_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_glyf>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_glyf_initCapN(mut arr: *mut table_glyf, mut n: usize) {
    table_glyf_init(arr);
    table_glyf_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn table_glyf_growToN(arr: *mut table_glyf, target: usize) {
    cvec_grow_to_n(table_glyf_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_glyf_free(mut x: *mut table_glyf) {
    if x.is_null() {
        return;
    }
    table_glyf_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_glyf_createN(mut n: usize) -> *mut table_glyf {
    let mut t: *mut table_glyf =
        malloc(::core::mem::size_of::<table_glyf>() as usize) as *mut table_glyf;
    table_glyf_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_glyf_create() -> *mut table_glyf {
    let mut x: *mut table_glyf =
        malloc(::core::mem::size_of::<table_glyf>() as usize) as *mut table_glyf;
    table_glyf_init(x);
    return x;
}
#[no_mangle]
pub static table_iGlyf: __caryll_vectorinterface_table_glyf = {
    __caryll_vectorinterface_table_glyf {
        init: Some(table_glyf_init as unsafe extern "C" fn(*mut table_glyf) -> ()),
        copy: Some(
            table_glyf_copy as unsafe extern "C" fn(*mut table_glyf, *const table_glyf) -> (),
        ),
        move_0: Some(
            table_glyf_move as unsafe extern "C" fn(*mut table_glyf, *mut table_glyf) -> (),
        ),
        dispose: Some(table_glyf_dispose as unsafe extern "C" fn(*mut table_glyf) -> ()),
        replace: Some(
            table_glyf_replace as unsafe extern "C" fn(*mut table_glyf, table_glyf) -> (),
        ),
        copyReplace: Some(
            table_glyf_copyReplace as unsafe extern "C" fn(*mut table_glyf, table_glyf) -> (),
        ),
        create: Some(table_glyf_create),
        free: Some(table_glyf_free as unsafe extern "C" fn(*mut table_glyf) -> ()),
        initN: Some(table_glyf_initN as unsafe extern "C" fn(*mut table_glyf, usize) -> ()),
        initCapN: Some(table_glyf_initCapN as unsafe extern "C" fn(*mut table_glyf, usize) -> ()),
        createN: Some(table_glyf_createN as unsafe extern "C" fn(usize) -> *mut table_glyf),
        fill: Some(table_glyf_fill as unsafe extern "C" fn(*mut table_glyf, usize) -> ()),
        clear: Some(table_glyf_dispose as unsafe extern "C" fn(*mut table_glyf) -> ()),
        push: Some(table_glyf_push as unsafe extern "C" fn(*mut table_glyf, glyf_GlyphPtr) -> ()),
        shrinkToFit: Some(table_glyf_shrinkToFit as unsafe extern "C" fn(*mut table_glyf) -> ()),
        pop: Some(table_glyf_pop as unsafe extern "C" fn(*mut table_glyf) -> glyf_GlyphPtr),
        disposeItem: Some(
            table_glyf_disposeItem as unsafe extern "C" fn(*mut table_glyf, usize) -> (),
        ),
        filterEnv: Some(
            table_glyf_filterEnv
                as unsafe extern "C" fn(
                    *mut table_glyf,
                    Option<
                        unsafe extern "C" fn(
                            *const glyf_GlyphPtr,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_glyf_sort
                as unsafe extern "C" fn(
                    *mut table_glyf,
                    Option<
                        unsafe extern "C" fn(
                            *const glyf_GlyphPtr,
                            *const glyf_GlyphPtr,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
unsafe extern "C" fn glyf_glyph_dump_contours(
    mut g: *mut glyf_Glyph,
    mut target: *mut json_value,
    mut ctx: *const GlyfIOContext,
) {
    if (*g).contours.length == 0 {
        return;
    }
    let mut contours: *mut json_value = json_array_new((*g).contours.length);
    let mut k: shapeid_t = 0 as shapeid_t;
    while (k as usize) < (*g).contours.length {
        let mut c: *mut glyf_Contour = (*g).contours.items.offset(k as isize) as *mut glyf_Contour;
        let mut contour: *mut json_value = json_array_new((*c).length);
        let mut m: shapeid_t = 0 as shapeid_t;
        while (m as usize) < (*c).length {
            let mut point: *mut json_value = json_object_new(4 as usize);
            json_object_push(
                point,
                b"x\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_VQ((*(*c).items.offset(m as isize)).x, (*ctx).fvar),
            );
            json_object_push(
                point,
                b"y\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_VQ((*(*c).items.offset(m as isize)).y, (*ctx).fvar),
            );
            json_object_push(
                point,
                b"on\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    (*(*c).items.offset(m as isize)).onCurve as ::core::ffi::c_int
                        & MASK_ON_CURVE as ::core::ffi::c_int,
                ),
            );
            json_array_push(contour, point);
            m = m.wrapping_add(1);
        }
        json_array_push(contours, preserialize(contour));
        k = k.wrapping_add(1);
    }
    json_object_push(
        target,
        b"contours\0" as *const u8 as *const ::core::ffi::c_char,
        contours,
    );
}
unsafe extern "C" fn glyf_glyph_dump_references(
    mut g: *mut glyf_Glyph,
    mut target: *mut json_value,
    mut ctx: *const GlyfIOContext,
) {
    if (*g).references.length == 0 {
        return;
    }
    let mut references: *mut json_value = json_array_new((*g).references.length);
    let mut k: shapeid_t = 0 as shapeid_t;
    while (k as usize) < (*g).references.length {
        let mut r: *mut glyf_ComponentReference =
            (*g).references.items.offset(k as isize) as *mut glyf_ComponentReference;
        let mut ref_0: *mut json_value = json_object_new(9 as usize);
        json_object_push(
            ref_0,
            b"glyph\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new_length(
                sdslen((*r).glyph.name) as ::core::ffi::c_uint,
                (*r).glyph.name as *const ::core::ffi::c_char,
            ),
        );
        json_object_push(
            ref_0,
            b"x\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_VQ((*r).x, (*ctx).fvar),
        );
        json_object_push(
            ref_0,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_VQ((*r).y, (*ctx).fvar),
        );
        json_object_push(
            ref_0,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*r).a as pos_t),
        );
        json_object_push(
            ref_0,
            b"b\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*r).b as pos_t),
        );
        json_object_push(
            ref_0,
            b"c\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*r).c as pos_t),
        );
        json_object_push(
            ref_0,
            b"d\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*r).d as pos_t),
        );
        if (*r).isAnchored != REF_XY
        {
            json_object_push(
                ref_0,
                b"isAnchored\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(true_0),
            );
            json_object_push(
                ref_0,
                b"inner\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).inner as i64),
            );
            json_object_push(
                ref_0,
                b"outer\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).outer as i64),
            );
        }
        if (*r).roundToGrid {
            json_object_push(
                ref_0,
                b"roundToGrid\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(true_0),
            );
        }
        if (*r).useMyMetrics {
            json_object_push(
                ref_0,
                b"useMyMetrics\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(true_0),
            );
        }
        json_array_push(references, preserialize(ref_0));
        k = k.wrapping_add(1);
    }
    json_object_push(
        target,
        b"references\0" as *const u8 as *const ::core::ffi::c_char,
        references,
    );
}
unsafe extern "C" fn glyf_glyph_dump_stemdefs(mut stems: *mut glyf_StemDefList) -> *mut json_value {
    let mut a: *mut json_value = json_array_new((*stems).length);
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as usize) < (*stems).length {
        let mut stem: *mut json_value = json_object_new(3 as usize);
        json_object_push(
            stem,
            b"position\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*(*stems).items.offset(j as isize)).position),
        );
        json_object_push(
            stem,
            b"width\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*(*stems).items.offset(j as isize)).width),
        );
        json_array_push(a, stem);
        j = j.wrapping_add(1);
    }
    return a;
}
unsafe extern "C" fn glyf_glyph_dump_maskdefs(
    mut masks: *mut glyf_MaskList,
    mut hh: *mut glyf_StemDefList,
    mut vv: *mut glyf_StemDefList,
) -> *mut json_value {
    let mut a: *mut json_value = json_array_new((*masks).length);
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as usize) < (*masks).length {
        let mut mask: *mut json_value = json_object_new(3 as usize);
        json_object_push(
            mask,
            b"contoursBefore\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*(*masks).items.offset(j as isize)).contoursBefore as i64),
        );
        json_object_push(
            mask,
            b"pointsBefore\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*(*masks).items.offset(j as isize)).pointsBefore as i64),
        );
        let mut h: *mut json_value = json_array_new((*hh).length);
        let mut k: shapeid_t = 0 as shapeid_t;
        while (k as usize) < (*hh).length {
            json_array_push(
                h,
                json_boolean_new(
                    (*(*masks).items.offset(j as isize)).maskH[k as usize] as ::core::ffi::c_int,
                ),
            );
            k = k.wrapping_add(1);
        }
        json_object_push(
            mask,
            b"maskH\0" as *const u8 as *const ::core::ffi::c_char,
            h,
        );
        let mut v: *mut json_value = json_array_new((*vv).length);
        let mut k_0: shapeid_t = 0 as shapeid_t;
        while (k_0 as usize) < (*vv).length {
            json_array_push(
                v,
                json_boolean_new(
                    (*(*masks).items.offset(j as isize)).maskV[k_0 as usize] as ::core::ffi::c_int,
                ),
            );
            k_0 = k_0.wrapping_add(1);
        }
        json_object_push(
            mask,
            b"maskV\0" as *const u8 as *const ::core::ffi::c_char,
            v,
        );
        json_array_push(a, mask);
        j = j.wrapping_add(1);
    }
    return a;
}
unsafe extern "C" fn glyf_dump_glyph(
    mut g: *mut glyf_Glyph,
    mut options: *const otfcc_Options,
    mut ctx: *const GlyfIOContext,
) -> *mut json_value {
    let mut glyph: *mut json_value = json_object_new(12 as usize);
    json_object_push(
        glyph,
        b"advanceWidth\0" as *const u8 as *const ::core::ffi::c_char,
        json_new_VQ((*g).advanceWidth, (*ctx).fvar),
    );
    if iVQ.isStill.expect("non-null function pointer")((*g).horizontalOrigin) as ::core::ffi::c_int
        != 0
        && fabs(
            iVQ.getStill.expect("non-null function pointer")((*g).horizontalOrigin)
                as ::core::ffi::c_double,
        ) > 1.0f64 / 1000.0f64
    {
        json_object_push(
            glyph,
            b"horizontalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_VQ((*g).horizontalOrigin, (*ctx).fvar),
        );
    }
    if (*ctx).hasVerticalMetrics {
        json_object_push(
            glyph,
            b"advanceHeight\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_VQ((*g).advanceHeight, (*ctx).fvar),
        );
        json_object_push(
            glyph,
            b"verticalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_VQ((*g).verticalOrigin, (*ctx).fvar),
        );
    }
    glyf_glyph_dump_contours(g, glyph, ctx);
    glyf_glyph_dump_references(g, glyph, ctx);
    if (*ctx).exportFDSelect {
        json_object_push(
            glyph,
            b"CFF_fdSelect\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new((*g).fdSelect.name as *const ::core::ffi::c_char),
        );
        json_object_push(
            glyph,
            b"CFF_CID\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*g).cid as i64),
        );
    }
    if !(*options).ignore_hints {
        if !(*g).instructions.is_null() && (*g).instructionsLength as ::core::ffi::c_int != 0 {
            json_object_push(
                glyph,
                b"instructions\0" as *const u8 as *const ::core::ffi::c_char,
                dump_ttinstr(
                    (*g).instructions,
                    (*g).instructionsLength as u32,
                    options,
                ),
            );
        }
        if (*g).stemH.length != 0 {
            json_object_push(
                glyph,
                b"stemH\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_stemdefs(&raw mut (*g).stemH)),
            );
        }
        if (*g).stemV.length != 0 {
            json_object_push(
                glyph,
                b"stemV\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_stemdefs(&raw mut (*g).stemV)),
            );
        }
        if (*g).hintMasks.length != 0 {
            json_object_push(
                glyph,
                b"hintMasks\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_maskdefs(
                    &raw mut (*g).hintMasks,
                    &raw mut (*g).stemH,
                    &raw mut (*g).stemV,
                )),
            );
        }
        if (*g).contourMasks.length != 0 {
            json_object_push(
                glyph,
                b"contourMasks\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_maskdefs(
                    &raw mut (*g).contourMasks,
                    &raw mut (*g).stemH,
                    &raw mut (*g).stemV,
                )),
            );
        }
        if (*g).yPel != 0 {
            json_object_push(
                glyph,
                b"LTSH_yPel\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*g).yPel as i64),
            );
        }
    }
    return glyph;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dump_glyphorder(
    mut table: *const table_glyf,
    mut root: *mut json_value,
) {
    if table.is_null() {
        return;
    }
    let mut order: *mut json_value = json_array_new((*table).length);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*table).length {
        json_array_push(
            order,
            json_string_new_length(
                sdslen((**(*table).items.offset(j as isize)).name) as ::core::ffi::c_uint,
                (**(*table).items.offset(j as isize)).name as *const ::core::ffi::c_char,
            ),
        );
        j = j.wrapping_add(1);
    }
    json_object_push(
        root,
        b"glyph_order\0" as *const u8 as *const ::core::ffi::c_char,
        preserialize(order),
    );
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpGlyf(
    mut table: *const table_glyf,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
    mut ctx: *const GlyfIOContext,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"glyf"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut glyf: *mut json_value = json_object_new((*table).length);
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as usize) < (*table).length {
            let mut g: *mut glyf_Glyph = *(*table).items.offset(j as isize) as *mut glyf_Glyph;
            json_object_push(
                glyf,
                (*g).name as *const ::core::ffi::c_char,
                glyf_dump_glyph(g, options, ctx),
            );
            j = j.wrapping_add(1);
        }
        json_object_push(
            root,
            b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
            glyf,
        );
        if !(*options).ignore_glyph_order {
            otfcc_dump_glyphorder(table, root);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
unsafe extern "C" fn glyf_parse_point(mut pointdump: *mut json_value) -> glyf_Point {
    let mut point: glyf_Point = glyf_Point {
        x: VQ {
            kernel: 0.,
            shift: vq_SegList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vq_Segment>(),
            },
        },
        y: VQ {
            kernel: 0.,
            shift: vq_SegList {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<vq_Segment>(),
            },
        },
        onCurve: 0,
    };
    glyf_iPoint.init.expect("non-null function pointer")(&raw mut point);
    if pointdump.is_null()
        || (*pointdump).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return point;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*pointdump).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char =
            (*(*pointdump).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*pointdump).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, b"x\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        {
            iVQ.replace.expect("non-null function pointer")(
                &raw mut point.x,
                json_vqOf(cv, ::core::ptr::null::<table_fvar>()) as VQ,
            );
        } else if strcmp(ck, b"y\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            iVQ.replace.expect("non-null function pointer")(
                &raw mut point.y,
                json_vqOf(cv, ::core::ptr::null::<table_fvar>()) as VQ,
            );
        } else if strcmp(ck, b"on\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            point.onCurve = json_boolof(cv) as i8;
        }
        _k = _k.wrapping_add(1);
    }
    return point;
}
unsafe extern "C" fn glyf_parse_contours(mut col: *mut json_value, mut g: *mut glyf_Glyph) {
    if col.is_null() {
        return;
    }
    let mut nContours: shapeid_t = (*col).u.array.length as shapeid_t;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as ::core::ffi::c_int) < nContours as ::core::ffi::c_int {
        let mut contourdump: *mut json_value =
            *(*col).u.array.values.offset(j as isize) as *mut json_value;
        let mut contour: glyf_Contour = glyf_Contour {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<glyf_Point>(),
        };
        glyf_iContour.initCapN.expect("non-null function pointer")(
            &raw mut contour,
            (if !contourdump.is_null()
                && (*contourdump).type_0 as ::core::ffi::c_uint
                    == json_array as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*contourdump).u.array.length
            } else {
                1 as ::core::ffi::c_uint
            }) as usize,
        );
        if !contourdump.is_null()
            && (*contourdump).type_0 as ::core::ffi::c_uint
                == json_array as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut k: shapeid_t = 0 as shapeid_t;
            while (k as ::core::ffi::c_uint) < (*contourdump).u.array.length {
                glyf_iContour.push.expect("non-null function pointer")(
                    &raw mut contour,
                    glyf_parse_point(
                        *(*contourdump).u.array.values.offset(k as isize) as *mut json_value
                    ),
                );
                k = k.wrapping_add(1);
            }
        }
        glyf_iContourList.push.expect("non-null function pointer")(&raw mut (*g).contours, contour);
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn glyf_parse_reference(mut refdump: *mut json_value) -> glyf_ComponentReference {
    let mut _gname: *mut json_value = json_obj_get_type(
        refdump,
        b"glyph\0" as *const u8 as *const ::core::ffi::c_char,
        json_string,
    );
    let mut ref_0: glyf_ComponentReference =
        (
            glyf_iComponentReference
                .empty
                .expect("non-null function pointer"))();
    if !_gname.is_null() {
        ref_0.glyph = handle_fromName(sdsnewlen(
            (*_gname).u.string.ptr as *const ::core::ffi::c_void,
            (*_gname).u.string.length as usize,
        )) as otfcc_GlyphHandle;
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.x,
            json_vqOf(
                json_obj_get(refdump, b"x\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<table_fvar>(),
            ) as VQ,
        );
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.y,
            json_vqOf(
                json_obj_get(refdump, b"y\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<table_fvar>(),
            ) as VQ,
        );
        ref_0.a = json_obj_getnum_fallback(
            refdump,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
            1.0f64,
        ) as scale_t;
        ref_0.b = json_obj_getnum_fallback(
            refdump,
            b"b\0" as *const u8 as *const ::core::ffi::c_char,
            0.0f64,
        ) as scale_t;
        ref_0.c = json_obj_getnum_fallback(
            refdump,
            b"c\0" as *const u8 as *const ::core::ffi::c_char,
            0.0f64,
        ) as scale_t;
        ref_0.d = json_obj_getnum_fallback(
            refdump,
            b"d\0" as *const u8 as *const ::core::ffi::c_char,
            1.0f64,
        ) as scale_t;
        ref_0.roundToGrid = json_obj_getbool(
            refdump,
            b"roundToGrid\0" as *const u8 as *const ::core::ffi::c_char,
        );
        ref_0.useMyMetrics = json_obj_getbool(
            refdump,
            b"useMyMetrics\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if json_obj_getbool(
            refdump,
            b"isAnchored\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            ref_0.isAnchored = REF_ANCHOR_XY;
            ref_0.inner = json_obj_getint(
                refdump,
                b"inner\0" as *const u8 as *const ::core::ffi::c_char,
            ) as shapeid_t;
            ref_0.outer = json_obj_getint(
                refdump,
                b"outer\0" as *const u8 as *const ::core::ffi::c_char,
            ) as shapeid_t;
        }
    } else {
        ref_0.glyph.name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.x,
            iVQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as pos_t)
                as VQ,
        );
        iVQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.y,
            iVQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as pos_t)
                as VQ,
        );
        ref_0.a = 1.0f64 as scale_t;
        ref_0.b = 0.0f64 as scale_t;
        ref_0.c = 0.0f64 as scale_t;
        ref_0.d = 1.0f64 as scale_t;
        ref_0.roundToGrid = false;
        ref_0.useMyMetrics = false;
    }
    return ref_0;
}
unsafe extern "C" fn glyf_parse_references(mut col: *mut json_value, mut g: *mut glyf_Glyph) {
    if col.is_null() {
        return;
    }
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as ::core::ffi::c_uint) < (*col).u.array.length {
        glyf_iReferenceList.push.expect("non-null function pointer")(
            &raw mut (*g).references,
            glyf_parse_reference(*(*col).u.array.values.offset(j as isize) as *mut json_value),
        );
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn makeInstrsForGlyph(
    mut _g: *mut ::core::ffi::c_void,
    mut instrs: *mut u8,
    mut len: u32,
) {
    let mut g: *mut glyf_Glyph = _g as *mut glyf_Glyph;
    (*g).instructionsLength = len as u16;
    (*g).instructions = instrs;
}
unsafe extern "C" fn wrongInstrsForGlyph(
    mut _g: *mut ::core::ffi::c_void,
    mut reason: *mut ::core::ffi::c_char,
    mut pos: ::core::ffi::c_int,
) {
    let mut g: *mut glyf_Glyph = _g as *mut glyf_Glyph;
    fprintf(
        stderr,
        b"[OTFCC] TrueType instructions parse error : %s, at %d in /%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        reason,
        pos,
        (*g).name,
    );
}
unsafe extern "C" fn parse_stems(mut sd: *mut json_value, mut stems: *mut glyf_StemDefList) {
    if sd.is_null() {
        return;
    }
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as ::core::ffi::c_uint) < (*sd).u.array.length {
        let mut s: *mut json_value = *(*sd).u.array.values.offset(j as isize) as *mut json_value;
        if !((*s).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            let mut sdef: glyf_PostscriptStemDef = glyf_PostscriptStemDef {
                position: 0.,
                width: 0.,
                map: 0,
            };
            sdef.map = 0 as u16;
            sdef.position =
                json_obj_getnum(s, b"position\0" as *const u8 as *const ::core::ffi::c_char)
                    as pos_t;
            sdef.width =
                json_obj_getnum(s, b"width\0" as *const u8 as *const ::core::ffi::c_char) as pos_t;
            glyf_iStemDefList.push.expect("non-null function pointer")(stems, sdef);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn parse_maskbits(mut arr: *mut bool, mut bits: *mut json_value) {
    if bits.is_null() {
        let mut j: shapeid_t = 0 as shapeid_t;
        while (j as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int {
            *arr.offset(j as isize) = false;
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: shapeid_t = 0 as shapeid_t;
        while (j_0 as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int
            && (j_0 as ::core::ffi::c_uint) < (*bits).u.array.length
        {
            let mut b: *mut json_value =
                *(*bits).u.array.values.offset(j_0 as isize) as *mut json_value;
            match (*b).type_0 as ::core::ffi::c_uint {
                6 => {
                    *arr.offset(j_0 as isize) = (*b).u.boolean != 0;
                }
                3 => {
                    *arr.offset(j_0 as isize) = (*b).u.integer != 0;
                }
                4 => {
                    *arr.offset(j_0 as isize) = (*b).u.dbl != 0.;
                }
                _ => {
                    *arr.offset(j_0 as isize) = false;
                }
            }
            j_0 = j_0.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn parse_masks(mut md: *mut json_value, mut masks: *mut glyf_MaskList) {
    if md.is_null() {
        return;
    }
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as ::core::ffi::c_uint) < (*md).u.array.length {
        let mut m: *mut json_value = *(*md).u.array.values.offset(j as isize) as *mut json_value;
        if !((*m).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            let mut mask: glyf_PostscriptHintMask = glyf_PostscriptHintMask {
                pointsBefore: 0,
                contoursBefore: 0,
                maskH: [false; 256],
                maskV: [false; 256],
            };
            mask.pointsBefore = json_obj_getint(
                m,
                b"pointsBefore\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            mask.contoursBefore = json_obj_getint(
                m,
                b"contoursBefore\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            parse_maskbits(
                (&raw mut mask.maskH as *mut bool).offset(0 as ::core::ffi::c_int as isize)
                    as *mut bool,
                json_obj_get_type(
                    m,
                    b"maskH\0" as *const u8 as *const ::core::ffi::c_char,
                    json_array,
                ),
            );
            parse_maskbits(
                (&raw mut mask.maskV as *mut bool).offset(0 as ::core::ffi::c_int as isize)
                    as *mut bool,
                json_obj_get_type(
                    m,
                    b"maskV\0" as *const u8 as *const ::core::ffi::c_char,
                    json_array,
                ),
            );
            glyf_iMaskList.push.expect("non-null function pointer")(masks, mask);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn otfcc_glyf_parse_glyph(
    mut glyphdump: *mut json_value,
    mut order_entry: *mut otfcc_GlyphOrderEntry,
    mut options: *const otfcc_Options,
) -> *mut glyf_Glyph {
    let mut g: *mut glyf_Glyph = otfcc_newGlyf_glyph();
    (*g).name = sdsdup((*order_entry).name);
    iVQ.replace.expect("non-null function pointer")(
        &raw mut (*g).advanceWidth,
        json_vqOf(
            json_obj_get(
                glyphdump,
                b"advanceWidth\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<table_fvar>(),
        ) as VQ,
    );
    iVQ.replace.expect("non-null function pointer")(
        &raw mut (*g).horizontalOrigin,
        json_vqOf(
            json_obj_get(
                glyphdump,
                b"horizontalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<table_fvar>(),
        ) as VQ,
    );
    iVQ.replace.expect("non-null function pointer")(
        &raw mut (*g).advanceHeight,
        json_vqOf(
            json_obj_get(
                glyphdump,
                b"advanceHeight\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<table_fvar>(),
        ) as VQ,
    );
    iVQ.replace.expect("non-null function pointer")(
        &raw mut (*g).verticalOrigin,
        json_vqOf(
            json_obj_get(
                glyphdump,
                b"verticalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<table_fvar>(),
        ) as VQ,
    );
    glyf_parse_contours(
        json_obj_get_type(
            glyphdump,
            b"contours\0" as *const u8 as *const ::core::ffi::c_char,
            json_array,
        ),
        g,
    );
    glyf_parse_references(
        json_obj_get_type(
            glyphdump,
            b"references\0" as *const u8 as *const ::core::ffi::c_char,
            json_array,
        ),
        g,
    );
    if !(*options).ignore_hints {
        parse_ttinstr(
            json_obj_get(
                glyphdump,
                b"instructions\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            g as *mut ::core::ffi::c_void,
            Some(
                makeInstrsForGlyph
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut u8, u32) -> (),
            ),
            Some(
                wrongInstrsForGlyph
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        parse_stems(
            json_obj_get_type(
                glyphdump,
                b"stemH\0" as *const u8 as *const ::core::ffi::c_char,
                json_array,
            ),
            &raw mut (*g).stemH,
        );
        parse_stems(
            json_obj_get_type(
                glyphdump,
                b"stemV\0" as *const u8 as *const ::core::ffi::c_char,
                json_array,
            ),
            &raw mut (*g).stemV,
        );
        parse_masks(
            json_obj_get_type(
                glyphdump,
                b"hintMasks\0" as *const u8 as *const ::core::ffi::c_char,
                json_array,
            ),
            &raw mut (*g).hintMasks,
        );
        parse_masks(
            json_obj_get_type(
                glyphdump,
                b"contourMasks\0" as *const u8 as *const ::core::ffi::c_char,
                json_array,
            ),
            &raw mut (*g).contourMasks,
        );
        (*g).yPel = json_obj_getint(
            glyphdump,
            b"LTSH_yPel\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u8;
    }
    (*g).fdSelect = handle_fromName(json_obj_getsds(
        glyphdump,
        b"CFF_fdSelect\0" as *const u8 as *const ::core::ffi::c_char,
    )) as otfcc_FDHandle;
    if (*g).yPel == 0 {
        (*g).yPel = json_obj_getint(
            glyphdump,
            b"yPel\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u8;
    }
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseGlyf(
    mut root: *const json_value,
    mut glyph_order: *mut otfcc_GlyphOrder,
    mut options: *const otfcc_Options,
) -> *mut table_glyf {
    if (*root).type_0 as ::core::ffi::c_uint
        != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
        || glyph_order.is_null()
    {
        return ::core::ptr::null_mut::<table_glyf>();
    }
    let mut glyf: *mut table_glyf = ::core::ptr::null_mut::<table_glyf>();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"glyf"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut numGlyphs: glyphid_t = (*table).u.object.length as glyphid_t;
            glyf = table_iGlyf.createN.expect("non-null function pointer")(numGlyphs as usize);
            let mut j: glyphid_t = 0 as glyphid_t;
            while (j as ::core::ffi::c_int) < numGlyphs as ::core::ffi::c_int {
                let mut gname: sds = sdsnewlen(
                    (*(*table).u.object.values.offset(j as isize)).name
                        as *const ::core::ffi::c_void,
                    (*(*table).u.object.values.offset(j as isize)).name_length as usize,
                );
                let mut glyphdump: *mut json_value =
                    (*(*table).u.object.values.offset(j as isize)).value as *mut json_value;
                let mut order_entry: *mut otfcc_GlyphOrderEntry =
                    ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
                let mut _hf_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i: ::core::ffi::c_uint = 0;
                let mut _hj_j: ::core::ffi::c_uint = 0;
                let mut _hj_k: ::core::ffi::c_uint = 0;
                let mut _hj_key: *const ::core::ffi::c_uchar = gname as *const ::core::ffi::c_uchar;
                _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i = _hj_j;
                _hj_k = sdslen(gname) as ::core::ffi::c_uint;
                while _hj_k >= 12 as ::core::ffi::c_uint {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_i = _hj_i.wrapping_sub(_hj_j);
                    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                    _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
                    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                    _hj_j = _hj_j.wrapping_sub(_hj_i);
                    _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                    _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
                    _hj_i = _hj_i.wrapping_sub(_hj_j);
                    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                    _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
                    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                    _hj_j = _hj_j.wrapping_sub(_hj_i);
                    _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                    _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
                    _hj_i = _hj_i.wrapping_sub(_hj_j);
                    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                    _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
                    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                    _hj_j = _hj_j.wrapping_sub(_hj_i);
                    _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                    _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
                    _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
                    _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
                }
                _hf_hashv = _hf_hashv.wrapping_add(sdslen(gname) as ::core::ffi::c_uint);
                let mut current_block_53: u64;
                match _hj_k {
                    11 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_53 = 15301932405498136070;
                    }
                    10 => {
                        current_block_53 = 15301932405498136070;
                    }
                    9 => {
                        current_block_53 = 5604290665302916367;
                    }
                    8 => {
                        current_block_53 = 2913595961553146217;
                    }
                    7 => {
                        current_block_53 = 1663355323994064757;
                    }
                    6 => {
                        current_block_53 = 3032986562397890031;
                    }
                    5 => {
                        current_block_53 = 5185893132852962855;
                    }
                    4 => {
                        current_block_53 = 4881172391704927252;
                    }
                    3 => {
                        current_block_53 = 12735873851622692391;
                    }
                    2 => {
                        current_block_53 = 8609698614768814962;
                    }
                    1 => {
                        current_block_53 = 1398023371624501419;
                    }
                    _ => {
                        current_block_53 = 14220266465818359136;
                    }
                }
                match current_block_53 {
                    15301932405498136070 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_53 = 5604290665302916367;
                    }
                    _ => {}
                }
                match current_block_53 {
                    5604290665302916367 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_53 = 2913595961553146217;
                    }
                    _ => {}
                }
                match current_block_53 {
                    2913595961553146217 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_53 = 1663355323994064757;
                    }
                    _ => {}
                }
                match current_block_53 {
                    1663355323994064757 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_53 = 3032986562397890031;
                    }
                    _ => {}
                }
                match current_block_53 {
                    3032986562397890031 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_53 = 5185893132852962855;
                    }
                    _ => {}
                }
                match current_block_53 {
                    5185893132852962855 => {
                        _hj_j = _hj_j
                            .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_53 = 4881172391704927252;
                    }
                    _ => {}
                }
                match current_block_53 {
                    4881172391704927252 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_53 = 12735873851622692391;
                    }
                    _ => {}
                }
                match current_block_53 {
                    12735873851622692391 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_53 = 8609698614768814962;
                    }
                    _ => {}
                }
                match current_block_53 {
                    8609698614768814962 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_53 = 1398023371624501419;
                    }
                    _ => {}
                }
                match current_block_53 {
                    1398023371624501419 => {
                        _hj_i = _hj_i
                            .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
                order_entry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
                if !(*glyph_order).byName.is_null() {
                    let mut _hf_bkt: ::core::ffi::c_uint = 0;
                    _hf_bkt = _hf_hashv
                        & (*(*(*glyph_order).byName).hhName.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if !(*(*(*(*glyph_order).byName).hhName.tbl)
                            .buckets
                            .offset(_hf_bkt as isize))
                        .hh_head
                        .is_null()
                        {
                            order_entry = ((*(*(*(*glyph_order).byName).hhName.tbl)
                                .buckets
                                .offset(_hf_bkt as isize))
                            .hh_head
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(*(*glyph_order).byName).hhName.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut otfcc_GlyphOrderEntry
                                as *mut otfcc_GlyphOrderEntry;
                        } else {
                            order_entry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
                        }
                        while !order_entry.is_null() {
                            if (*order_entry).hhName.hashv == _hf_hashv
                                && (*order_entry).hhName.keylen as usize == sdslen(gname)
                            {
                                if memcmp(
                                    (*order_entry).hhName.key,
                                    gname as *const ::core::ffi::c_void,
                                    sdslen(gname),
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if !(*order_entry).hhName.hh_next.is_null() {
                                order_entry = ((*order_entry).hhName.hh_next
                                    as *mut ::core::ffi::c_char)
                                    .offset(-(*(*(*glyph_order).byName).hhName.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut otfcc_GlyphOrderEntry
                                    as *mut otfcc_GlyphOrderEntry;
                            } else {
                                order_entry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
                            }
                        }
                    }
                }
                if (*glyphdump).type_0 as ::core::ffi::c_uint
                    == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !order_entry.is_null()
                    && (*(*glyf).items.offset((*order_entry).gid as isize)).is_null()
                {
                    let ref mut fresh15 = *(*glyf).items.offset((*order_entry).gid as isize);
                    *fresh15 =
                        otfcc_glyf_parse_glyph(glyphdump, order_entry, options) as glyf_GlyphPtr;
                }
                json_value_free(glyphdump);
                let mut v: *mut json_value = json_null_new();
                (*v).parent = table as *mut _json_value;
                let ref mut fresh16 = (*(*table).u.object.values.offset(j as isize)).value;
                *fresh16 = v as *mut _json_value;
                sdsfree(gname);
                j = j.wrapping_add(1);
            }
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        return glyf;
    }
    return ::core::ptr::null_mut::<table_glyf>();
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_getsds(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> sds {
    let mut v: *mut json_value = json_obj_get_type(obj, key, json_string);
    if v.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        return sdsnewlen(
            (*v).u.string.ptr as *const ::core::ffi::c_void,
            (*v).u.string.length as usize,
        );
    };
}
#[inline]
unsafe extern "C" fn json_new_position(mut z: pos_t) -> *mut json_value {
    if round(z as ::core::ffi::c_double) == z {
        return json_integer_new(z as i64);
    } else {
        return json_double_new(z as ::core::ffi::c_double);
    };
}
#[inline]
unsafe extern "C" fn json_obj_getnum(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0.0f64;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0.0f64;
}
#[inline]
unsafe extern "C" fn json_obj_getint(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> i32 {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as i32;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as i32;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl as i32;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn json_obj_getnum_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return fallback;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return fallback;
}
#[inline]
unsafe extern "C" fn json_boolof(mut cv: *const json_value) -> bool {
    if !cv.is_null()
        && (*cv).type_0 as ::core::ffi::c_uint
            == json_boolean as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*cv).u.boolean != 0;
    }
    return false;
}
#[inline]
unsafe extern "C" fn json_obj_getbool(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> bool {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_boolean as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.boolean != 0;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return false;
}
#[inline]
unsafe extern "C" fn preserialize(mut x: *mut json_value) -> *mut json_value {
    let mut opts: json_serialize_opts = json_serialize_opts {
        mode: json_serialize_mode_packed,
        opts: 0,
        indent_size: 0,
    };
    let mut preserialize_len: usize = json_measure_ex(x, opts);
    let mut buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let mut xx: *mut json_value = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as usize) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = json_pre_serialized;
    return xx;
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

pub const ARG_1_AND_2_ARE_WORDS: glyf_ComponentFlags = 1;

pub const UNSCALED_COMPONENT_OFFSET: glyf_ComponentFlags = 4096;

pub const USE_MY_METRICS: glyf_ComponentFlags = 512;

pub const ROUND_XY_TO_GRID: glyf_ComponentFlags = 4;

pub const ARGS_ARE_XY_VALUES: glyf_ComponentFlags = 2;

pub const GLYF_FLAG_REPEAT: glyf_PointFlags = 8;

pub const GLYF_FLAG_ON_CURVE: glyf_PointFlags = 1;

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

#[cfg(test)]
mod tests {
    use super::*;

    // `glyf/build.rs` writes a component's flags based on
    // `isAnchored == REF_ANCHOR_CONSOLIDATED`, so these values pick which branch
    // of the composite-glyph encoding runs. They come from otfcc's own
    // consolidation pass, never off the wire, but they are still load-bearing for
    // the bytes that come out.
    #[test]
    fn refanchorstatus_discriminants_match_the_c_enum() {
        assert_eq!(REF_XY as u32, 0);
        assert_eq!(REF_ANCHOR_ANCHOR as u32, 1);
        assert_eq!(REF_ANCHOR_XY as u32, 2);
        assert_eq!(REF_ANCHOR_CONSOLIDATED as u32, 3);
        assert_eq!(REF_ANCHOR_CONSOLIDATING_ANCHOR as u32, 4);
        assert_eq!(REF_ANCHOR_CONSOLIDATING_XY as u32, 5);
    }
}
