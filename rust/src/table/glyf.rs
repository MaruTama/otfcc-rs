#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod build;
pub mod read;

use libc::{fprintf, free, malloc, memcmp, memcpy, memset, qsort, strcmp};
unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::handle::{HandleState, handle_from_name, FdHandle, GlyphHandle, Handle, otfcc_handle_copy, otfcc_handle_dispose, otfcc_handle_empty};
use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{ILogger};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, Pos, Scale, ShapeId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{JsonValue, JsonType};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::buffer::{Buffer};
use crate::support::{ComparFn, TRUE_0};
use crate::support::glyph_order::{GlyphOrder, GlyphOrderEntry};
use crate::table::fvar::{FvarTable};



use crate::vf::vq::{VQ, VqSegList, VqSegment};
use crate::support::json_funcs::{json_boolof, json_new_position, json_obj_get, json_obj_get_type, json_obj_getbool, json_obj_getint, json_obj_getnum, json_obj_getnum_fallback, json_obj_getsds, preserialize};
use crate::support::ttinstr::{dump_ttinstr, parse_ttinstr};
use crate::table::fvar::{json_new_vq, json_vq_of};
use crate::vendor::json::{json_value_free};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_boolean_new, json_integer_new, json_null_new, json_object_new, json_object_push, json_string_new, json_string_new_length};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree, sdsnewlen};
use crate::vf::vq::{I_VQ};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Point {
    pub x: VQ,
    pub y: VQ,
    pub onCurve: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PointElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut Point) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut Point, *const Point) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut Point, *mut Point) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut Point) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut Point, Point) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut Point, Point) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> Point>,
    pub dup: Option<unsafe extern "C" fn(Point) -> Point>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Contour {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut Point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContourVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut Contour) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut Contour, *const Contour) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut Contour, *mut Contour) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut Contour) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut Contour, Contour) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut Contour, Contour) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut Contour>,
    pub free: Option<unsafe extern "C" fn(*mut Contour) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut Contour, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut Contour, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut Contour>,
    pub fill: Option<unsafe extern "C" fn(*mut Contour, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut Contour) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut Contour, Point) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut Contour) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut Contour) -> Point>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut Contour, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut Contour,
            Option<unsafe extern "C" fn(*const Point, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut Contour,
            Option<unsafe extern "C" fn(*const Point, *const Point) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContourList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut Contour,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContourListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut ContourList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut ContourList, *const ContourList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut ContourList, *mut ContourList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ContourList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut ContourList, ContourList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut ContourList, ContourList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ContourList>,
    pub free: Option<unsafe extern "C" fn(*mut ContourList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut ContourList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut ContourList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut ContourList>,
    pub fill: Option<unsafe extern "C" fn(*mut ContourList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut ContourList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut ContourList, Contour) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut ContourList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut ContourList) -> Contour>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut ContourList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut ContourList,
            Option<unsafe extern "C" fn(*const Contour, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut ContourList,
            Option<
                unsafe extern "C" fn(
                    *const Contour,
                    *const Contour,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PostscriptStemDef {
    pub position: Pos,
    pub width: Pos,
    pub map: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PostscriptStemDefElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut PostscriptStemDef) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut PostscriptStemDef, *const PostscriptStemDef) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut PostscriptStemDef, *mut PostscriptStemDef) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut PostscriptStemDef) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut PostscriptStemDef, PostscriptStemDef) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut PostscriptStemDef, PostscriptStemDef) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StemDefList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut PostscriptStemDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StemDefListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut StemDefList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut StemDefList, *const StemDefList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut StemDefList, *mut StemDefList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut StemDefList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut StemDefList, StemDefList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut StemDefList, StemDefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut StemDefList>,
    pub free: Option<unsafe extern "C" fn(*mut StemDefList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut StemDefList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut StemDefList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut StemDefList>,
    pub fill: Option<unsafe extern "C" fn(*mut StemDefList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut StemDefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut StemDefList, PostscriptStemDef) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut StemDefList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut StemDefList) -> PostscriptStemDef>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut StemDefList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut StemDefList,
            Option<
                unsafe extern "C" fn(
                    *const PostscriptStemDef,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut StemDefList,
            Option<
                unsafe extern "C" fn(
                    *const PostscriptStemDef,
                    *const PostscriptStemDef,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PostscriptHintMask {
    pub pointsBefore: u16,
    pub contoursBefore: u16,
    pub maskH: [bool; 256],
    pub maskV: [bool; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PostscriptHintMaskElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut PostscriptHintMask) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut PostscriptHintMask, *const PostscriptHintMask) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut PostscriptHintMask, *mut PostscriptHintMask) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut PostscriptHintMask) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut PostscriptHintMask, PostscriptHintMask) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut PostscriptHintMask, PostscriptHintMask) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MaskList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut PostscriptHintMask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MaskListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut MaskList) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut MaskList, *const MaskList) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut MaskList, *mut MaskList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut MaskList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut MaskList, MaskList) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut MaskList, MaskList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut MaskList>,
    pub free: Option<unsafe extern "C" fn(*mut MaskList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut MaskList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut MaskList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut MaskList>,
    pub fill: Option<unsafe extern "C" fn(*mut MaskList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut MaskList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut MaskList, PostscriptHintMask) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut MaskList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut MaskList) -> PostscriptHintMask>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut MaskList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut MaskList,
            Option<
                unsafe extern "C" fn(
                    *const PostscriptHintMask,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut MaskList,
            Option<
                unsafe extern "C" fn(
                    *const PostscriptHintMask,
                    *const PostscriptHintMask,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RefAnchorStatus {
    Xy = 0,
    AnchorAnchor = 1,
    AnchorXy = 2,
    AnchorConsolidated = 3,
    AnchorConsolidatingAnchor = 4,
    AnchorConsolidatingXy = 5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComponentReference {
    pub x: VQ,
    pub y: VQ,
    pub roundToGrid: bool,
    pub useMyMetrics: bool,
    pub glyph: GlyphHandle,
    pub a: Scale,
    pub b: Scale,
    pub c: Scale,
    pub d: Scale,
    pub isAnchored: RefAnchorStatus,
    pub inner: ShapeId,
    pub outer: ShapeId,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComponentReferenceElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut ComponentReference) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut ComponentReference, *const ComponentReference) -> (),
    >,
    pub move_0: Option<
        unsafe extern "C" fn(*mut ComponentReference, *mut ComponentReference) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut ComponentReference) -> ()>,
    pub replace:
        Option<unsafe extern "C" fn(*mut ComponentReference, ComponentReference) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut ComponentReference, ComponentReference) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> ComponentReference>,
    pub dup: Option<unsafe extern "C" fn(ComponentReference) -> ComponentReference>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReferenceList {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut ComponentReference,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReferenceListVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut ReferenceList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut ReferenceList, *const ReferenceList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut ReferenceList, *mut ReferenceList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ReferenceList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut ReferenceList, ReferenceList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut ReferenceList, ReferenceList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ReferenceList>,
    pub free: Option<unsafe extern "C" fn(*mut ReferenceList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut ReferenceList, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut ReferenceList, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut ReferenceList>,
    pub fill: Option<unsafe extern "C" fn(*mut ReferenceList, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut ReferenceList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut ReferenceList, ComponentReference) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut ReferenceList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut ReferenceList) -> ComponentReference>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut ReferenceList, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut ReferenceList,
            Option<
                unsafe extern "C" fn(
                    *const ComponentReference,
                    *mut ::core::ffi::c_void,
                ) -> bool,
            >,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut ReferenceList,
            Option<
                unsafe extern "C" fn(
                    *const ComponentReference,
                    *const ComponentReference,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphStat {
    pub xMin: Pos,
    pub xMax: Pos,
    pub yMin: Pos,
    pub yMax: Pos,
    pub nestDepth: u16,
    pub nPoints: u16,
    pub nContours: u16,
    pub nCompositePoints: u16,
    pub nCompositeContours: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Glyph {
    pub name: SdsRaw,
    pub horizontalOrigin: VQ,
    pub advanceWidth: VQ,
    pub verticalOrigin: VQ,
    pub advanceHeight: VQ,
    pub contours: ContourList,
    pub references: ReferenceList,
    pub stemH: StemDefList,
    pub stemV: StemDefList,
    pub hintMasks: MaskList,
    pub contourMasks: MaskList,
    pub instructionsLength: u16,
    pub instructions: *mut u8,
    pub yPel: u8,
    pub fdSelect: FdHandle,
    pub cid: GlyphId,
    pub stat: GlyphStat,
}
pub type GlyphPtr = *mut Glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphPtrElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GlyphPtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut GlyphPtr, *const GlyphPtr) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut GlyphPtr, *mut GlyphPtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GlyphPtr) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut GlyphPtr, GlyphPtr) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut GlyphPtr, GlyphPtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyfTable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut GlyphPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyfTableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut GlyfTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut GlyfTable, *const GlyfTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut GlyfTable, *mut GlyfTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GlyfTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut GlyfTable, GlyfTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut GlyfTable, GlyfTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GlyfTable>,
    pub free: Option<unsafe extern "C" fn(*mut GlyfTable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut GlyfTable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut GlyfTable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut GlyfTable>,
    pub fill: Option<unsafe extern "C" fn(*mut GlyfTable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut GlyfTable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut GlyfTable, GlyphPtr) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut GlyfTable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut GlyfTable) -> GlyphPtr>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut GlyfTable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut GlyfTable,
            Option<unsafe extern "C" fn(*const GlyphPtr, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut GlyfTable,
            Option<
                unsafe extern "C" fn(
                    *const GlyphPtr,
                    *const GlyphPtr,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyfIOContext {
    pub locaIsLong: bool,
    pub numGlyphs: GlyphId,
    pub nPhantomPoints: ShapeId,
    pub fvar: *mut FvarTable,
    pub hasVerticalMetrics: bool,
    pub exportFDSelect: bool,
}
/// The only bit of [`Point::onCurve`] that means anything.
///
/// Not a flag *set*, despite C giving it a `glyf_OnCurveMask` type of its own:
/// `onCurve` is an `i8` holding 0 or 1, and both readers of the field mask it
/// down to bit 0 rather than trusting it -- so this is typed as the `i8` it is
/// applied to, which is what lets the two sites drop their casts.
pub const MASK_ON_CURVE: i8 = 1;
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
unsafe extern "C" fn create_point(mut p: *mut Point) {
    (*p).x = I_VQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
    (*p).y = I_VQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
    (*p).onCurve = TRUE_0 as i8;
}
unsafe extern "C" fn copy_point(mut dst: *mut Point, mut src: *const Point) {
    I_VQ.copy.expect("non-null function pointer")(&raw mut (*dst).x, &raw const (*src).x);
    I_VQ.copy.expect("non-null function pointer")(&raw mut (*dst).y, &raw const (*src).y);
    (*dst).onCurve = (*src).onCurve;
}
unsafe extern "C" fn dispose_point(mut p: *mut Point) {
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*p).x);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*p).y);
}
#[inline]
unsafe extern "C" fn glyf_point_copy_replace(mut dst: *mut Point, src: Point) {
    glyf_point_dispose(dst);
    glyf_point_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_point_dispose(mut x: *mut Point) {
    dispose_point(x);
}
#[inline]
unsafe extern "C" fn glyf_point_empty() -> Point {
    let mut x: Point = Point {
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
        onCurve: 0,
    };
    glyf_point_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn glyf_point_replace(mut dst: *mut Point, src: Point) {
    glyf_point_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Point>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_point_init(mut x: *mut Point) {
    create_point(x);
}
pub static GLYF_I_POINT: PointElementInterface = {
    PointElementInterface {
        init: Some(glyf_point_init as unsafe extern "C" fn(*mut Point) -> ()),
        copy: Some(
            glyf_point_copy as unsafe extern "C" fn(*mut Point, *const Point) -> (),
        ),
        move_0: Some(
            glyf_point_move as unsafe extern "C" fn(*mut Point, *mut Point) -> (),
        ),
        dispose: Some(glyf_point_dispose as unsafe extern "C" fn(*mut Point) -> ()),
        replace: Some(
            glyf_point_replace as unsafe extern "C" fn(*mut Point, Point) -> (),
        ),
        copyReplace: Some(
            glyf_point_copy_replace as unsafe extern "C" fn(*mut Point, Point) -> (),
        ),
        empty: Some(glyf_point_empty),
        dup: Some(glyf_point_dup as unsafe extern "C" fn(Point) -> Point),
    }
};
#[inline]
unsafe extern "C" fn glyf_point_dup(src: Point) -> Point {
    let mut dst: Point = Point {
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
        onCurve: 0,
    };
    glyf_point_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn glyf_point_move(mut dst: *mut Point, mut src: *mut Point) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Point>() as usize,
    );
    glyf_point_init(src);
}
#[inline]
unsafe extern "C" fn glyf_point_copy(mut dst: *mut Point, mut src: *const Point) {
    copy_point(dst, src);
}
#[inline]
unsafe extern "C" fn glyf_contour_free(mut x: *mut Contour) {
    if x.is_null() {
        return;
    }
    glyf_contour_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn glyf_contour_init_n(mut arr: *mut Contour, mut n: usize) {
    glyf_contour_init(arr);
    glyf_contour_grow_to_n(arr, n);
    glyf_contour_fill(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_contour_create_n(mut n: usize) -> *mut Contour {
    let mut t: *mut Contour =
        malloc(::core::mem::size_of::<Contour>() as usize) as *mut Contour;
    glyf_contour_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn glyf_contour_create() -> *mut Contour {
    let mut x: *mut Contour =
        malloc(::core::mem::size_of::<Contour>() as usize) as *mut Contour;
    glyf_contour_init(x);
    return x;
}
pub static GLYF_I_CONTOUR: ContourVectorInterface = {
    ContourVectorInterface {
        init: Some(glyf_contour_init as unsafe extern "C" fn(*mut Contour) -> ()),
        copy: Some(
            glyf_contour_copy as unsafe extern "C" fn(*mut Contour, *const Contour) -> (),
        ),
        move_0: Some(
            glyf_contour_move as unsafe extern "C" fn(*mut Contour, *mut Contour) -> (),
        ),
        dispose: Some(glyf_contour_dispose as unsafe extern "C" fn(*mut Contour) -> ()),
        replace: Some(
            glyf_contour_replace as unsafe extern "C" fn(*mut Contour, Contour) -> (),
        ),
        copyReplace: Some(
            glyf_contour_copy_replace as unsafe extern "C" fn(*mut Contour, Contour) -> (),
        ),
        create: Some(glyf_contour_create),
        free: Some(glyf_contour_free as unsafe extern "C" fn(*mut Contour) -> ()),
        initN: Some(glyf_contour_init_n as unsafe extern "C" fn(*mut Contour, usize) -> ()),
        initCapN: Some(
            glyf_contour_init_cap_n as unsafe extern "C" fn(*mut Contour, usize) -> (),
        ),
        createN: Some(glyf_contour_create_n as unsafe extern "C" fn(usize) -> *mut Contour),
        fill: Some(glyf_contour_fill as unsafe extern "C" fn(*mut Contour, usize) -> ()),
        clear: Some(glyf_contour_dispose as unsafe extern "C" fn(*mut Contour) -> ()),
        push: Some(glyf_contour_push as unsafe extern "C" fn(*mut Contour, Point) -> ()),
        shrinkToFit: Some(
            glyf_contour_shrink_to_fit as unsafe extern "C" fn(*mut Contour) -> (),
        ),
        pop: Some(glyf_contour_pop as unsafe extern "C" fn(*mut Contour) -> Point),
        disposeItem: Some(
            glyf_contour_dispose_item as unsafe extern "C" fn(*mut Contour, usize) -> (),
        ),
        filterEnv: Some(
            glyf_contour_filter_env
                as unsafe extern "C" fn(
                    *mut Contour,
                    Option<
                        unsafe extern "C" fn(*const Point, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            glyf_contour_sort
                as unsafe extern "C" fn(
                    *mut Contour,
                    Option<
                        unsafe extern "C" fn(
                            *const Point,
                            *const Point,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_contour_shrink_to_fit(mut arr: *mut Contour) {
    glyf_contour_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn glyf_contour_resize_to(arr: *mut Contour, target: usize) {
    cvec_resize_to(glyf_contour_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_contour_move(dst: *mut Contour, src: *mut Contour) {
    cvec_move(glyf_contour_as_cvec(dst), glyf_contour_as_cvec(src));
}
#[inline]
unsafe fn glyf_contour_as_cvec(arr: *mut Contour) -> *mut CVecRaw<Point> {
    arr as *mut CVecRaw<Point>
}
#[inline]
unsafe extern "C" fn glyf_contour_init(arr: *mut Contour) {
    cvec_init(glyf_contour_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_contour_filter_env(
    mut arr: *mut Contour,
    mut fn_0: Option<unsafe extern "C" fn(*const Point, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut Point,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if GLYF_I_POINT.dispose.is_some() {
                GLYF_I_POINT.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut Point,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn glyf_contour_dispose_item(mut arr: *mut Contour, mut n: usize) {
    if GLYF_I_POINT.dispose.is_some() {
        GLYF_I_POINT.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut Point
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn glyf_contour_sort(
    mut arr: *mut Contour,
    mut fn_0: Option<
        unsafe extern "C" fn(*const Point, *const Point) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<Point>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const Point, *const Point) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn glyf_contour_fill(mut arr: *mut Contour, mut n: usize) {
    while (*arr).length < n {
        let mut x: Point = Point {
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
            onCurve: 0,
        };
        if GLYF_I_POINT.init.is_some() {
            GLYF_I_POINT.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Point>() as usize,
            );
        }
        glyf_contour_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn glyf_contour_push(arr: *mut Contour, elem: Point) {
    cvec_push(glyf_contour_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_contour_grow(arr: *mut Contour) {
    cvec_grow(glyf_contour_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_contour_grow_to(arr: *mut Contour, target: usize) {
    cvec_grow_to(glyf_contour_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_contour_pop(arr: *mut Contour) -> Point {
    cvec_pop(glyf_contour_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn glyf_contour_copy_replace(mut dst: *mut Contour, src: Contour) {
    glyf_contour_dispose(dst);
    glyf_contour_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_contour_copy(mut dst: *mut Contour, mut src: *const Contour) {
    glyf_contour_init(dst);
    glyf_contour_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if GLYF_I_POINT.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GLYF_I_POINT.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut Point,
                (*src).items.offset(j as isize) as *mut Point as *const Point,
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
unsafe extern "C" fn glyf_contour_dispose(mut arr: *mut Contour) {
    if arr.is_null() {
        return;
    }
    if GLYF_I_POINT.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            GLYF_I_POINT.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut Point,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<Point>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn glyf_contour_replace(mut dst: *mut Contour, src: Contour) {
    glyf_contour_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Contour>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_contour_init_cap_n(mut arr: *mut Contour, mut n: usize) {
    glyf_contour_init(arr);
    glyf_contour_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_contour_grow_to_n(arr: *mut Contour, target: usize) {
    cvec_grow_to_n(glyf_contour_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_init_n(mut arr: *mut ContourList, mut n: usize) {
    glyf_contour_list_init(arr);
    glyf_contour_list_grow_to_n(arr, n);
    glyf_contour_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_filter_env(
    mut arr: *mut ContourList,
    mut fn_0: Option<unsafe extern "C" fn(*const Contour, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut Contour,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if GLYF_I_CONTOUR.dispose.is_some() {
                GLYF_I_CONTOUR.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut Contour,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn glyf_contour_list_dispose_item(mut arr: *mut ContourList, mut n: usize) {
    if GLYF_I_CONTOUR.dispose.is_some() {
        GLYF_I_CONTOUR.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut Contour
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn glyf_contour_list_replace(
    mut dst: *mut ContourList,
    src: ContourList,
) {
    glyf_contour_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ContourList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_contour_list_copy_replace(
    mut dst: *mut ContourList,
    src: ContourList,
) {
    glyf_contour_list_dispose(dst);
    glyf_contour_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_grow_to_n(arr: *mut ContourList, target: usize) {
    cvec_grow_to_n(glyf_contour_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_copy(
    mut dst: *mut ContourList,
    mut src: *const ContourList,
) {
    glyf_contour_list_init(dst);
    glyf_contour_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if GLYF_I_CONTOUR.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GLYF_I_CONTOUR.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut Contour,
                (*src).items.offset(j as isize) as *mut Contour as *const Contour,
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
unsafe extern "C" fn glyf_contour_list_free(mut x: *mut ContourList) {
    if x.is_null() {
        return;
    }
    glyf_contour_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_sort(
    mut arr: *mut ContourList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const Contour, *const Contour) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<Contour>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const Contour,
                    *const Contour,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn glyf_contour_list_create_n(mut n: usize) -> *mut ContourList {
    let mut t: *mut ContourList =
        malloc(::core::mem::size_of::<ContourList>() as usize) as *mut ContourList;
    glyf_contour_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn glyf_contour_list_create() -> *mut ContourList {
    let mut x: *mut ContourList =
        malloc(::core::mem::size_of::<ContourList>() as usize) as *mut ContourList;
    glyf_contour_list_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn glyf_contour_list_dispose(mut arr: *mut ContourList) {
    if arr.is_null() {
        return;
    }
    if GLYF_I_CONTOUR.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            GLYF_I_CONTOUR.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut Contour,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<Contour>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
pub static GLYF_I_CONTOUR_LIST: ContourListVectorInterface = {
    ContourListVectorInterface {
        init: Some(glyf_contour_list_init as unsafe extern "C" fn(*mut ContourList) -> ()),
        copy: Some(
            glyf_contour_list_copy
                as unsafe extern "C" fn(*mut ContourList, *const ContourList) -> (),
        ),
        move_0: Some(
            glyf_contour_list_move
                as unsafe extern "C" fn(*mut ContourList, *mut ContourList) -> (),
        ),
        dispose: Some(
            glyf_contour_list_dispose as unsafe extern "C" fn(*mut ContourList) -> (),
        ),
        replace: Some(
            glyf_contour_list_replace
                as unsafe extern "C" fn(*mut ContourList, ContourList) -> (),
        ),
        copyReplace: Some(
            glyf_contour_list_copy_replace
                as unsafe extern "C" fn(*mut ContourList, ContourList) -> (),
        ),
        create: Some(glyf_contour_list_create),
        free: Some(glyf_contour_list_free as unsafe extern "C" fn(*mut ContourList) -> ()),
        initN: Some(
            glyf_contour_list_init_n as unsafe extern "C" fn(*mut ContourList, usize) -> (),
        ),
        initCapN: Some(
            glyf_contour_list_init_cap_n as unsafe extern "C" fn(*mut ContourList, usize) -> (),
        ),
        createN: Some(
            glyf_contour_list_create_n as unsafe extern "C" fn(usize) -> *mut ContourList,
        ),
        fill: Some(
            glyf_contour_list_fill as unsafe extern "C" fn(*mut ContourList, usize) -> (),
        ),
        clear: Some(glyf_contour_list_dispose as unsafe extern "C" fn(*mut ContourList) -> ()),
        push: Some(
            glyf_contour_list_push
                as unsafe extern "C" fn(*mut ContourList, Contour) -> (),
        ),
        shrinkToFit: Some(
            glyf_contour_list_shrink_to_fit as unsafe extern "C" fn(*mut ContourList) -> (),
        ),
        pop: Some(
            glyf_contour_list_pop as unsafe extern "C" fn(*mut ContourList) -> Contour,
        ),
        disposeItem: Some(
            glyf_contour_list_dispose_item
                as unsafe extern "C" fn(*mut ContourList, usize) -> (),
        ),
        filterEnv: Some(
            glyf_contour_list_filter_env
                as unsafe extern "C" fn(
                    *mut ContourList,
                    Option<
                        unsafe extern "C" fn(*const Contour, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            glyf_contour_list_sort
                as unsafe extern "C" fn(
                    *mut ContourList,
                    Option<
                        unsafe extern "C" fn(
                            *const Contour,
                            *const Contour,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_contour_list_shrink_to_fit(mut arr: *mut ContourList) {
    glyf_contour_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_fill(mut arr: *mut ContourList, mut n: usize) {
    while (*arr).length < n {
        let mut x: Contour = Contour {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<Point>(),
        };
        if GLYF_I_CONTOUR.init.is_some() {
            GLYF_I_CONTOUR.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Contour>() as usize,
            );
        }
        glyf_contour_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn glyf_contour_list_push(arr: *mut ContourList, elem: Contour) {
    cvec_push(glyf_contour_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_grow(arr: *mut ContourList) {
    cvec_grow(glyf_contour_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_contour_list_grow_to(arr: *mut ContourList, target: usize) {
    cvec_grow_to(glyf_contour_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_pop(arr: *mut ContourList) -> Contour {
    cvec_pop(glyf_contour_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn glyf_contour_list_resize_to(arr: *mut ContourList, target: usize) {
    cvec_resize_to(glyf_contour_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_move(dst: *mut ContourList, src: *mut ContourList) {
    cvec_move(glyf_contour_list_as_cvec(dst), glyf_contour_list_as_cvec(src));
}
#[inline]
unsafe fn glyf_contour_list_as_cvec(arr: *mut ContourList) -> *mut CVecRaw<Contour> {
    arr as *mut CVecRaw<Contour>
}
#[inline]
unsafe extern "C" fn glyf_contour_list_init(arr: *mut ContourList) {
    cvec_init(glyf_contour_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_contour_list_init_cap_n(mut arr: *mut ContourList, mut n: usize) {
    glyf_contour_list_init(arr);
    glyf_contour_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn init_glyf_reference(mut ref_0: *mut ComponentReference) {
    (*ref_0).glyph = otfcc_handle_empty() as GlyphHandle;
    (*ref_0).x =
        I_VQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
    (*ref_0).y =
        I_VQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
    (*ref_0).a = 1 as ::core::ffi::c_int as Scale;
    (*ref_0).b = 0 as ::core::ffi::c_int as Scale;
    (*ref_0).c = 0 as ::core::ffi::c_int as Scale;
    (*ref_0).d = 1 as ::core::ffi::c_int as Scale;
    (*ref_0).isAnchored = RefAnchorStatus::Xy;
    (*ref_0).outer = 0 as ShapeId;
    (*ref_0).inner = (*ref_0).outer;
    (*ref_0).roundToGrid = false;
    (*ref_0).useMyMetrics = false;
}
unsafe extern "C" fn copy_glyf_reference(
    mut dst: *mut ComponentReference,
    mut src: *const ComponentReference,
) {
    I_VQ.copy.expect("non-null function pointer")(&raw mut (*dst).x, &raw const (*src).x);
    I_VQ.copy.expect("non-null function pointer")(&raw mut (*dst).y, &raw const (*src).y);
    otfcc_handle_copy(
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
unsafe extern "C" fn dispose_glyf_reference(mut ref_0: *mut ComponentReference) {
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*ref_0).x);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*ref_0).y);
    otfcc_handle_dispose(&raw mut (*ref_0).glyph);
}
#[inline]
unsafe extern "C" fn glyf_component_reference_dispose(mut x: *mut ComponentReference) {
    dispose_glyf_reference(x);
}
#[inline]
unsafe extern "C" fn glyf_component_reference_replace(
    mut dst: *mut ComponentReference,
    src: ComponentReference,
) {
    glyf_component_reference_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ComponentReference>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_component_reference_dup(
    src: ComponentReference,
) -> ComponentReference {
    let mut dst: ComponentReference = ComponentReference {
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
            state: HandleState::Empty,
            index: 0,
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        isAnchored: RefAnchorStatus::Xy,
        inner: 0,
        outer: 0,
    };
    glyf_component_reference_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn glyf_component_reference_copy(
    mut dst: *mut ComponentReference,
    mut src: *const ComponentReference,
) {
    copy_glyf_reference(dst, src);
}
#[inline]
unsafe extern "C" fn glyf_component_reference_move(
    mut dst: *mut ComponentReference,
    mut src: *mut ComponentReference,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ComponentReference>() as usize,
    );
    glyf_component_reference_init(src);
}
#[inline]
unsafe extern "C" fn glyf_component_reference_empty() -> ComponentReference {
    let mut x: ComponentReference = ComponentReference {
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
            state: HandleState::Empty,
            index: 0,
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        isAnchored: RefAnchorStatus::Xy,
        inner: 0,
        outer: 0,
    };
    glyf_component_reference_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn glyf_component_reference_init(mut x: *mut ComponentReference) {
    init_glyf_reference(x);
}
pub static GLYF_I_COMPONENT_REFERENCE: ComponentReferenceElementInterface = {
    ComponentReferenceElementInterface {
        init: Some(
            glyf_component_reference_init
                as unsafe extern "C" fn(*mut ComponentReference) -> (),
        ),
        copy: Some(
            glyf_component_reference_copy
                as unsafe extern "C" fn(
                    *mut ComponentReference,
                    *const ComponentReference,
                ) -> (),
        ),
        move_0: Some(
            glyf_component_reference_move
                as unsafe extern "C" fn(
                    *mut ComponentReference,
                    *mut ComponentReference,
                ) -> (),
        ),
        dispose: Some(
            glyf_component_reference_dispose
                as unsafe extern "C" fn(*mut ComponentReference) -> (),
        ),
        replace: Some(
            glyf_component_reference_replace
                as unsafe extern "C" fn(
                    *mut ComponentReference,
                    ComponentReference,
                ) -> (),
        ),
        copyReplace: Some(
            glyf_component_reference_copy_replace
                as unsafe extern "C" fn(
                    *mut ComponentReference,
                    ComponentReference,
                ) -> (),
        ),
        empty: Some(glyf_component_reference_empty),
        dup: Some(
            glyf_component_reference_dup
                as unsafe extern "C" fn(ComponentReference) -> ComponentReference,
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_component_reference_copy_replace(
    mut dst: *mut ComponentReference,
    src: ComponentReference,
) {
    glyf_component_reference_dispose(dst);
    glyf_component_reference_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_reference_list_copy_replace(
    mut dst: *mut ReferenceList,
    src: ReferenceList,
) {
    glyf_reference_list_dispose(dst);
    glyf_reference_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_reference_list_init_n(mut arr: *mut ReferenceList, mut n: usize) {
    glyf_reference_list_init(arr);
    glyf_reference_list_grow_to_n(arr, n);
    glyf_reference_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_reference_list_fill(mut arr: *mut ReferenceList, mut n: usize) {
    while (*arr).length < n {
        let mut x: ComponentReference = ComponentReference {
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
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            isAnchored: RefAnchorStatus::Xy,
            inner: 0,
            outer: 0,
        };
        if GLYF_I_COMPONENT_REFERENCE.init.is_some() {
            GLYF_I_COMPONENT_REFERENCE
                .init
                .expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ComponentReference>() as usize,
            );
        }
        glyf_reference_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn glyf_reference_list_push(arr: *mut ReferenceList, elem: ComponentReference) {
    cvec_push(glyf_reference_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_reference_list_grow(arr: *mut ReferenceList) {
    cvec_grow(glyf_reference_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_reference_list_grow_to(arr: *mut ReferenceList, target: usize) {
    cvec_grow_to(glyf_reference_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_reference_list_pop(arr: *mut ReferenceList) -> ComponentReference {
    cvec_pop(glyf_reference_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn glyf_reference_list_dispose(mut arr: *mut ReferenceList) {
    if arr.is_null() {
        return;
    }
    if GLYF_I_COMPONENT_REFERENCE.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh9 = j;
            j = j.wrapping_sub(1);
            if !(fresh9 != 0) {
                break;
            }
            GLYF_I_COMPONENT_REFERENCE
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut ComponentReference
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<ComponentReference>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn glyf_reference_list_copy(
    mut dst: *mut ReferenceList,
    mut src: *const ReferenceList,
) {
    glyf_reference_list_init(dst);
    glyf_reference_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if GLYF_I_COMPONENT_REFERENCE.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GLYF_I_COMPONENT_REFERENCE
                .copy
                .expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut ComponentReference,
                (*src).items.offset(j as isize) as *mut ComponentReference
                    as *const ComponentReference,
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
unsafe extern "C" fn glyf_reference_list_init_cap_n(mut arr: *mut ReferenceList, mut n: usize) {
    glyf_reference_list_init(arr);
    glyf_reference_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_reference_list_replace(
    mut dst: *mut ReferenceList,
    src: ReferenceList,
) {
    glyf_reference_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ReferenceList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_reference_list_grow_to_n(arr: *mut ReferenceList, target: usize) {
    cvec_grow_to_n(glyf_reference_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_reference_list_free(mut x: *mut ReferenceList) {
    if x.is_null() {
        return;
    }
    glyf_reference_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn glyf_reference_list_create_n(mut n: usize) -> *mut ReferenceList {
    let mut t: *mut ReferenceList =
        malloc(::core::mem::size_of::<ReferenceList>() as usize) as *mut ReferenceList;
    glyf_reference_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn glyf_reference_list_create() -> *mut ReferenceList {
    let mut x: *mut ReferenceList =
        malloc(::core::mem::size_of::<ReferenceList>() as usize) as *mut ReferenceList;
    glyf_reference_list_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn glyf_reference_list_move(dst: *mut ReferenceList, src: *mut ReferenceList) {
    cvec_move(glyf_reference_list_as_cvec(dst), glyf_reference_list_as_cvec(src));
}
#[inline]
unsafe extern "C" fn glyf_reference_list_shrink_to_fit(mut arr: *mut ReferenceList) {
    glyf_reference_list_resize_to(arr, (*arr).length);
}
pub static GLYF_I_REFERENCE_LIST: ReferenceListVectorInterface = {
    ReferenceListVectorInterface {
        init: Some(glyf_reference_list_init as unsafe extern "C" fn(*mut ReferenceList) -> ()),
        copy: Some(
            glyf_reference_list_copy
                as unsafe extern "C" fn(*mut ReferenceList, *const ReferenceList) -> (),
        ),
        move_0: Some(
            glyf_reference_list_move
                as unsafe extern "C" fn(*mut ReferenceList, *mut ReferenceList) -> (),
        ),
        dispose: Some(
            glyf_reference_list_dispose as unsafe extern "C" fn(*mut ReferenceList) -> (),
        ),
        replace: Some(
            glyf_reference_list_replace
                as unsafe extern "C" fn(*mut ReferenceList, ReferenceList) -> (),
        ),
        copyReplace: Some(
            glyf_reference_list_copy_replace
                as unsafe extern "C" fn(*mut ReferenceList, ReferenceList) -> (),
        ),
        create: Some(glyf_reference_list_create),
        free: Some(glyf_reference_list_free as unsafe extern "C" fn(*mut ReferenceList) -> ()),
        initN: Some(
            glyf_reference_list_init_n as unsafe extern "C" fn(*mut ReferenceList, usize) -> (),
        ),
        initCapN: Some(
            glyf_reference_list_init_cap_n
                as unsafe extern "C" fn(*mut ReferenceList, usize) -> (),
        ),
        createN: Some(
            glyf_reference_list_create_n as unsafe extern "C" fn(usize) -> *mut ReferenceList,
        ),
        fill: Some(
            glyf_reference_list_fill as unsafe extern "C" fn(*mut ReferenceList, usize) -> (),
        ),
        clear: Some(
            glyf_reference_list_dispose as unsafe extern "C" fn(*mut ReferenceList) -> (),
        ),
        push: Some(
            glyf_reference_list_push
                as unsafe extern "C" fn(*mut ReferenceList, ComponentReference) -> (),
        ),
        shrinkToFit: Some(
            glyf_reference_list_shrink_to_fit as unsafe extern "C" fn(*mut ReferenceList) -> (),
        ),
        pop: Some(
            glyf_reference_list_pop
                as unsafe extern "C" fn(*mut ReferenceList) -> ComponentReference,
        ),
        disposeItem: Some(
            glyf_reference_list_dispose_item
                as unsafe extern "C" fn(*mut ReferenceList, usize) -> (),
        ),
        filterEnv: Some(
            glyf_reference_list_filter_env
                as unsafe extern "C" fn(
                    *mut ReferenceList,
                    Option<
                        unsafe extern "C" fn(
                            *const ComponentReference,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            glyf_reference_list_sort
                as unsafe extern "C" fn(
                    *mut ReferenceList,
                    Option<
                        unsafe extern "C" fn(
                            *const ComponentReference,
                            *const ComponentReference,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe fn glyf_reference_list_as_cvec(arr: *mut ReferenceList) -> *mut CVecRaw<ComponentReference> {
    arr as *mut CVecRaw<ComponentReference>
}
#[inline]
unsafe extern "C" fn glyf_reference_list_init(arr: *mut ReferenceList) {
    cvec_init(glyf_reference_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_reference_list_filter_env(
    mut arr: *mut ReferenceList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const ComponentReference, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut ComponentReference,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if GLYF_I_COMPONENT_REFERENCE.dispose.is_some() {
                GLYF_I_COMPONENT_REFERENCE
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut ComponentReference,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn glyf_reference_list_resize_to(arr: *mut ReferenceList, target: usize) {
    cvec_resize_to(glyf_reference_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_reference_list_dispose_item(
    mut arr: *mut ReferenceList,
    mut n: usize,
) {
    if GLYF_I_COMPONENT_REFERENCE.dispose.is_some() {
        GLYF_I_COMPONENT_REFERENCE
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut ComponentReference
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn glyf_reference_list_sort(
    mut arr: *mut ReferenceList,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const ComponentReference,
            *const ComponentReference,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<ComponentReference>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ComponentReference,
                    *const ComponentReference,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn glyf_postscript_stem_def_copy_replace(
    mut dst: *mut PostscriptStemDef,
    src: PostscriptStemDef,
) {
    glyf_postscript_stem_def_dispose(dst);
    glyf_postscript_stem_def_copy(dst, &raw const src);
}
pub static GLYF_I_POSTSCRIPT_STEM_DEF: PostscriptStemDefElementInterface = {
    PostscriptStemDefElementInterface {
        init: Some(
            glyf_postscript_stem_def_init as unsafe extern "C" fn(*mut PostscriptStemDef) -> (),
        ),
        copy: Some(
            glyf_postscript_stem_def_copy
                as unsafe extern "C" fn(
                    *mut PostscriptStemDef,
                    *const PostscriptStemDef,
                ) -> (),
        ),
        move_0: Some(
            glyf_postscript_stem_def_move
                as unsafe extern "C" fn(
                    *mut PostscriptStemDef,
                    *mut PostscriptStemDef,
                ) -> (),
        ),
        dispose: Some(
            glyf_postscript_stem_def_dispose
                as unsafe extern "C" fn(*mut PostscriptStemDef) -> (),
        ),
        replace: Some(
            glyf_postscript_stem_def_replace
                as unsafe extern "C" fn(*mut PostscriptStemDef, PostscriptStemDef) -> (),
        ),
        copyReplace: Some(
            glyf_postscript_stem_def_copy_replace
                as unsafe extern "C" fn(*mut PostscriptStemDef, PostscriptStemDef) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_postscript_stem_def_replace(
    mut dst: *mut PostscriptStemDef,
    src: PostscriptStemDef,
) {
    glyf_postscript_stem_def_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<PostscriptStemDef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_postscript_stem_def_move(
    mut dst: *mut PostscriptStemDef,
    mut src: *mut PostscriptStemDef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<PostscriptStemDef>() as usize,
    );
    glyf_postscript_stem_def_init(src);
}
#[inline]
unsafe extern "C" fn glyf_postscript_stem_def_copy(
    mut dst: *mut PostscriptStemDef,
    mut src: *const PostscriptStemDef,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<PostscriptStemDef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_postscript_stem_def_dispose(mut _x: *mut PostscriptStemDef) {}
#[inline]
unsafe extern "C" fn glyf_postscript_stem_def_init(mut x: *mut PostscriptStemDef) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PostscriptStemDef>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_create_n(mut n: usize) -> *mut StemDefList {
    let mut t: *mut StemDefList =
        malloc(::core::mem::size_of::<StemDefList>() as usize) as *mut StemDefList;
    glyf_stem_def_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_shrink_to_fit(mut arr: *mut StemDefList) {
    glyf_stem_def_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_resize_to(arr: *mut StemDefList, target: usize) {
    cvec_resize_to(glyf_stem_def_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_move(dst: *mut StemDefList, src: *mut StemDefList) {
    cvec_move(glyf_stem_def_list_as_cvec(dst), glyf_stem_def_list_as_cvec(src));
}
#[inline]
unsafe fn glyf_stem_def_list_as_cvec(arr: *mut StemDefList) -> *mut CVecRaw<PostscriptStemDef> {
    arr as *mut CVecRaw<PostscriptStemDef>
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_init(arr: *mut StemDefList) {
    cvec_init(glyf_stem_def_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_filter_env(
    mut arr: *mut StemDefList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const PostscriptStemDef, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut PostscriptStemDef,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if GLYF_I_POSTSCRIPT_STEM_DEF.dispose.is_some() {
                GLYF_I_POSTSCRIPT_STEM_DEF
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut PostscriptStemDef,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_dispose_item(mut arr: *mut StemDefList, mut n: usize) {
    if GLYF_I_POSTSCRIPT_STEM_DEF.dispose.is_some() {
        GLYF_I_POSTSCRIPT_STEM_DEF
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut PostscriptStemDef
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_sort(
    mut arr: *mut StemDefList,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const PostscriptStemDef,
            *const PostscriptStemDef,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<PostscriptStemDef>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const PostscriptStemDef,
                    *const PostscriptStemDef,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_fill(mut arr: *mut StemDefList, mut n: usize) {
    while (*arr).length < n {
        let mut x: PostscriptStemDef = PostscriptStemDef {
            position: 0.,
            width: 0.,
            map: 0,
        };
        if GLYF_I_POSTSCRIPT_STEM_DEF.init.is_some() {
            GLYF_I_POSTSCRIPT_STEM_DEF
                .init
                .expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<PostscriptStemDef>() as usize,
            );
        }
        glyf_stem_def_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_push(arr: *mut StemDefList, elem: PostscriptStemDef) {
    cvec_push(glyf_stem_def_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_grow(arr: *mut StemDefList) {
    cvec_grow(glyf_stem_def_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_grow_to(arr: *mut StemDefList, target: usize) {
    cvec_grow_to(glyf_stem_def_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_pop(arr: *mut StemDefList) -> PostscriptStemDef {
    cvec_pop(glyf_stem_def_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_copy_replace(
    mut dst: *mut StemDefList,
    src: StemDefList,
) {
    glyf_stem_def_list_dispose(dst);
    glyf_stem_def_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_copy(
    mut dst: *mut StemDefList,
    mut src: *const StemDefList,
) {
    glyf_stem_def_list_init(dst);
    glyf_stem_def_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if GLYF_I_POSTSCRIPT_STEM_DEF.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GLYF_I_POSTSCRIPT_STEM_DEF
                .copy
                .expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut PostscriptStemDef,
                (*src).items.offset(j as isize) as *mut PostscriptStemDef
                    as *const PostscriptStemDef,
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
unsafe extern "C" fn glyf_stem_def_list_dispose(mut arr: *mut StemDefList) {
    if arr.is_null() {
        return;
    }
    if GLYF_I_POSTSCRIPT_STEM_DEF.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh5 = j;
            j = j.wrapping_sub(1);
            if !(fresh5 != 0) {
                break;
            }
            GLYF_I_POSTSCRIPT_STEM_DEF
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut PostscriptStemDef
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<PostscriptStemDef>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_replace(
    mut dst: *mut StemDefList,
    src: StemDefList,
) {
    glyf_stem_def_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<StemDefList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_init_cap_n(mut arr: *mut StemDefList, mut n: usize) {
    glyf_stem_def_list_init(arr);
    glyf_stem_def_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_grow_to_n(arr: *mut StemDefList, target: usize) {
    cvec_grow_to_n(glyf_stem_def_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_init_n(mut arr: *mut StemDefList, mut n: usize) {
    glyf_stem_def_list_init(arr);
    glyf_stem_def_list_grow_to_n(arr, n);
    glyf_stem_def_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_free(mut x: *mut StemDefList) {
    if x.is_null() {
        return;
    }
    glyf_stem_def_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_create() -> *mut StemDefList {
    let mut x: *mut StemDefList =
        malloc(::core::mem::size_of::<StemDefList>() as usize) as *mut StemDefList;
    glyf_stem_def_list_init(x);
    return x;
}
pub static GLYF_I_STEM_DEF_LIST: StemDefListVectorInterface = {
    StemDefListVectorInterface {
        init: Some(glyf_stem_def_list_init as unsafe extern "C" fn(*mut StemDefList) -> ()),
        copy: Some(
            glyf_stem_def_list_copy
                as unsafe extern "C" fn(*mut StemDefList, *const StemDefList) -> (),
        ),
        move_0: Some(
            glyf_stem_def_list_move
                as unsafe extern "C" fn(*mut StemDefList, *mut StemDefList) -> (),
        ),
        dispose: Some(
            glyf_stem_def_list_dispose as unsafe extern "C" fn(*mut StemDefList) -> (),
        ),
        replace: Some(
            glyf_stem_def_list_replace
                as unsafe extern "C" fn(*mut StemDefList, StemDefList) -> (),
        ),
        copyReplace: Some(
            glyf_stem_def_list_copy_replace
                as unsafe extern "C" fn(*mut StemDefList, StemDefList) -> (),
        ),
        create: Some(glyf_stem_def_list_create),
        free: Some(glyf_stem_def_list_free as unsafe extern "C" fn(*mut StemDefList) -> ()),
        initN: Some(
            glyf_stem_def_list_init_n as unsafe extern "C" fn(*mut StemDefList, usize) -> (),
        ),
        initCapN: Some(
            glyf_stem_def_list_init_cap_n as unsafe extern "C" fn(*mut StemDefList, usize) -> (),
        ),
        createN: Some(
            glyf_stem_def_list_create_n as unsafe extern "C" fn(usize) -> *mut StemDefList,
        ),
        fill: Some(
            glyf_stem_def_list_fill as unsafe extern "C" fn(*mut StemDefList, usize) -> (),
        ),
        clear: Some(glyf_stem_def_list_dispose as unsafe extern "C" fn(*mut StemDefList) -> ()),
        push: Some(
            glyf_stem_def_list_push
                as unsafe extern "C" fn(*mut StemDefList, PostscriptStemDef) -> (),
        ),
        shrinkToFit: Some(
            glyf_stem_def_list_shrink_to_fit as unsafe extern "C" fn(*mut StemDefList) -> (),
        ),
        pop: Some(
            glyf_stem_def_list_pop
                as unsafe extern "C" fn(*mut StemDefList) -> PostscriptStemDef,
        ),
        disposeItem: Some(
            glyf_stem_def_list_dispose_item
                as unsafe extern "C" fn(*mut StemDefList, usize) -> (),
        ),
        filterEnv: Some(
            glyf_stem_def_list_filter_env
                as unsafe extern "C" fn(
                    *mut StemDefList,
                    Option<
                        unsafe extern "C" fn(
                            *const PostscriptStemDef,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            glyf_stem_def_list_sort
                as unsafe extern "C" fn(
                    *mut StemDefList,
                    Option<
                        unsafe extern "C" fn(
                            *const PostscriptStemDef,
                            *const PostscriptStemDef,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
pub static GLYF_I_POSTSCRIPT_HINT_MASK: PostscriptHintMaskElementInterface = {
    PostscriptHintMaskElementInterface {
        init: Some(
            glyf_postscript_hint_mask_init
                as unsafe extern "C" fn(*mut PostscriptHintMask) -> (),
        ),
        copy: Some(
            glyf_postscript_hint_mask_copy
                as unsafe extern "C" fn(
                    *mut PostscriptHintMask,
                    *const PostscriptHintMask,
                ) -> (),
        ),
        move_0: Some(
            glyf_postscript_hint_mask_move
                as unsafe extern "C" fn(
                    *mut PostscriptHintMask,
                    *mut PostscriptHintMask,
                ) -> (),
        ),
        dispose: Some(
            glyf_postscript_hint_mask_dispose
                as unsafe extern "C" fn(*mut PostscriptHintMask) -> (),
        ),
        replace: Some(
            glyf_postscript_hint_mask_replace
                as unsafe extern "C" fn(
                    *mut PostscriptHintMask,
                    PostscriptHintMask,
                ) -> (),
        ),
        copyReplace: Some(
            glyf_postscript_hint_mask_copy_replace
                as unsafe extern "C" fn(
                    *mut PostscriptHintMask,
                    PostscriptHintMask,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_postscript_hint_mask_copy_replace(
    mut dst: *mut PostscriptHintMask,
    src: PostscriptHintMask,
) {
    glyf_postscript_hint_mask_dispose(dst);
    glyf_postscript_hint_mask_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_postscript_hint_mask_copy(
    mut dst: *mut PostscriptHintMask,
    mut src: *const PostscriptHintMask,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<PostscriptHintMask>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_postscript_hint_mask_dispose(mut _x: *mut PostscriptHintMask) {}
#[inline]
unsafe extern "C" fn glyf_postscript_hint_mask_replace(
    mut dst: *mut PostscriptHintMask,
    src: PostscriptHintMask,
) {
    glyf_postscript_hint_mask_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<PostscriptHintMask>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_postscript_hint_mask_move(
    mut dst: *mut PostscriptHintMask,
    mut src: *mut PostscriptHintMask,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<PostscriptHintMask>() as usize,
    );
    glyf_postscript_hint_mask_init(src);
}
#[inline]
unsafe extern "C" fn glyf_postscript_hint_mask_init(mut x: *mut PostscriptHintMask) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PostscriptHintMask>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_mask_list_copy(
    mut dst: *mut MaskList,
    mut src: *const MaskList,
) {
    glyf_mask_list_init(dst);
    glyf_mask_list_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if GLYF_I_POSTSCRIPT_HINT_MASK.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GLYF_I_POSTSCRIPT_HINT_MASK
                .copy
                .expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut PostscriptHintMask,
                (*src).items.offset(j as isize) as *mut PostscriptHintMask
                    as *const PostscriptHintMask,
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
pub static GLYF_I_MASK_LIST: MaskListVectorInterface = {
    MaskListVectorInterface {
        init: Some(glyf_mask_list_init as unsafe extern "C" fn(*mut MaskList) -> ()),
        copy: Some(
            glyf_mask_list_copy
                as unsafe extern "C" fn(*mut MaskList, *const MaskList) -> (),
        ),
        move_0: Some(
            glyf_mask_list_move
                as unsafe extern "C" fn(*mut MaskList, *mut MaskList) -> (),
        ),
        dispose: Some(glyf_mask_list_dispose as unsafe extern "C" fn(*mut MaskList) -> ()),
        replace: Some(
            glyf_mask_list_replace as unsafe extern "C" fn(*mut MaskList, MaskList) -> (),
        ),
        copyReplace: Some(
            glyf_mask_list_copy_replace
                as unsafe extern "C" fn(*mut MaskList, MaskList) -> (),
        ),
        create: Some(glyf_mask_list_create),
        free: Some(glyf_mask_list_free as unsafe extern "C" fn(*mut MaskList) -> ()),
        initN: Some(glyf_mask_list_init_n as unsafe extern "C" fn(*mut MaskList, usize) -> ()),
        initCapN: Some(
            glyf_mask_list_init_cap_n as unsafe extern "C" fn(*mut MaskList, usize) -> (),
        ),
        createN: Some(glyf_mask_list_create_n as unsafe extern "C" fn(usize) -> *mut MaskList),
        fill: Some(glyf_mask_list_fill as unsafe extern "C" fn(*mut MaskList, usize) -> ()),
        clear: Some(glyf_mask_list_dispose as unsafe extern "C" fn(*mut MaskList) -> ()),
        push: Some(
            glyf_mask_list_push
                as unsafe extern "C" fn(*mut MaskList, PostscriptHintMask) -> (),
        ),
        shrinkToFit: Some(
            glyf_mask_list_shrink_to_fit as unsafe extern "C" fn(*mut MaskList) -> (),
        ),
        pop: Some(
            glyf_mask_list_pop
                as unsafe extern "C" fn(*mut MaskList) -> PostscriptHintMask,
        ),
        disposeItem: Some(
            glyf_mask_list_dispose_item as unsafe extern "C" fn(*mut MaskList, usize) -> (),
        ),
        filterEnv: Some(
            glyf_mask_list_filter_env
                as unsafe extern "C" fn(
                    *mut MaskList,
                    Option<
                        unsafe extern "C" fn(
                            *const PostscriptHintMask,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            glyf_mask_list_sort
                as unsafe extern "C" fn(
                    *mut MaskList,
                    Option<
                        unsafe extern "C" fn(
                            *const PostscriptHintMask,
                            *const PostscriptHintMask,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_mask_list_shrink_to_fit(mut arr: *mut MaskList) {
    glyf_mask_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn glyf_mask_list_resize_to(arr: *mut MaskList, target: usize) {
    cvec_resize_to(glyf_mask_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_mask_list_move(dst: *mut MaskList, src: *mut MaskList) {
    cvec_move(glyf_mask_list_as_cvec(dst), glyf_mask_list_as_cvec(src));
}
#[inline]
unsafe fn glyf_mask_list_as_cvec(arr: *mut MaskList) -> *mut CVecRaw<PostscriptHintMask> {
    arr as *mut CVecRaw<PostscriptHintMask>
}
#[inline]
unsafe extern "C" fn glyf_mask_list_init(arr: *mut MaskList) {
    cvec_init(glyf_mask_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_mask_list_filter_env(
    mut arr: *mut MaskList,
    mut fn_0: Option<
        unsafe extern "C" fn(*const PostscriptHintMask, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut PostscriptHintMask,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if GLYF_I_POSTSCRIPT_HINT_MASK.dispose.is_some() {
                GLYF_I_POSTSCRIPT_HINT_MASK
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut PostscriptHintMask,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn glyf_mask_list_dispose_item(mut arr: *mut MaskList, mut n: usize) {
    if GLYF_I_POSTSCRIPT_HINT_MASK.dispose.is_some() {
        GLYF_I_POSTSCRIPT_HINT_MASK
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut PostscriptHintMask
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn glyf_mask_list_sort(
    mut arr: *mut MaskList,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const PostscriptHintMask,
            *const PostscriptHintMask,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<PostscriptHintMask>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const PostscriptHintMask,
                    *const PostscriptHintMask,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn glyf_mask_list_fill(mut arr: *mut MaskList, mut n: usize) {
    while (*arr).length < n {
        let mut x: PostscriptHintMask = PostscriptHintMask {
            pointsBefore: 0,
            contoursBefore: 0,
            maskH: [false; 256],
            maskV: [false; 256],
        };
        if GLYF_I_POSTSCRIPT_HINT_MASK.init.is_some() {
            GLYF_I_POSTSCRIPT_HINT_MASK
                .init
                .expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<PostscriptHintMask>() as usize,
            );
        }
        glyf_mask_list_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn glyf_mask_list_push(arr: *mut MaskList, elem: PostscriptHintMask) {
    cvec_push(glyf_mask_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_mask_list_grow(arr: *mut MaskList) {
    cvec_grow(glyf_mask_list_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_mask_list_grow_to(arr: *mut MaskList, target: usize) {
    cvec_grow_to(glyf_mask_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_mask_list_pop(arr: *mut MaskList) -> PostscriptHintMask {
    cvec_pop(glyf_mask_list_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn glyf_mask_list_copy_replace(mut dst: *mut MaskList, src: MaskList) {
    glyf_mask_list_dispose(dst);
    glyf_mask_list_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn glyf_mask_list_init_n(mut arr: *mut MaskList, mut n: usize) {
    glyf_mask_list_init(arr);
    glyf_mask_list_grow_to_n(arr, n);
    glyf_mask_list_fill(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_mask_list_init_cap_n(mut arr: *mut MaskList, mut n: usize) {
    glyf_mask_list_init(arr);
    glyf_mask_list_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_mask_list_replace(mut dst: *mut MaskList, src: MaskList) {
    glyf_mask_list_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MaskList>() as usize,
    );
}
#[inline]
unsafe extern "C" fn glyf_mask_list_dispose(mut arr: *mut MaskList) {
    if arr.is_null() {
        return;
    }
    if GLYF_I_POSTSCRIPT_HINT_MASK.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh7 = j;
            j = j.wrapping_sub(1);
            if !(fresh7 != 0) {
                break;
            }
            GLYF_I_POSTSCRIPT_HINT_MASK
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut PostscriptHintMask
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<PostscriptHintMask>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn glyf_mask_list_grow_to_n(arr: *mut MaskList, target: usize) {
    cvec_grow_to_n(glyf_mask_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_mask_list_create() -> *mut MaskList {
    let mut x: *mut MaskList =
        malloc(::core::mem::size_of::<MaskList>() as usize) as *mut MaskList;
    glyf_mask_list_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn glyf_mask_list_create_n(mut n: usize) -> *mut MaskList {
    let mut t: *mut MaskList =
        malloc(::core::mem::size_of::<MaskList>() as usize) as *mut MaskList;
    glyf_mask_list_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn glyf_mask_list_free(mut x: *mut MaskList) {
    if x.is_null() {
        return;
    }
    glyf_mask_list_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_new_glyf_glyph() -> *mut Glyph {
    let mut g: *mut Glyph = ::core::ptr::null_mut::<Glyph>();
    g = __caryll_allocate_clean(
        ::core::mem::size_of::<Glyph>() as usize,
        78 as ::core::ffi::c_ulong,
    ) as *mut Glyph;
    (*g).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    I_VQ.init.expect("non-null function pointer")(&raw mut (*g).horizontalOrigin);
    I_VQ.init.expect("non-null function pointer")(&raw mut (*g).advanceWidth);
    I_VQ.init.expect("non-null function pointer")(&raw mut (*g).verticalOrigin);
    I_VQ.init.expect("non-null function pointer")(&raw mut (*g).advanceHeight);
    GLYF_I_CONTOUR_LIST.init.expect("non-null function pointer")(&raw mut (*g).contours);
    GLYF_I_REFERENCE_LIST.init.expect("non-null function pointer")(&raw mut (*g).references);
    GLYF_I_STEM_DEF_LIST.init.expect("non-null function pointer")(&raw mut (*g).stemH);
    GLYF_I_STEM_DEF_LIST.init.expect("non-null function pointer")(&raw mut (*g).stemV);
    GLYF_I_MASK_LIST.init.expect("non-null function pointer")(&raw mut (*g).hintMasks);
    GLYF_I_MASK_LIST.init.expect("non-null function pointer")(&raw mut (*g).contourMasks);
    (*g).instructionsLength = 0 as u16;
    (*g).instructions = ::core::ptr::null_mut::<u8>();
    (*g).fdSelect = otfcc_handle_empty() as FdHandle;
    (*g).yPel = 0 as u8;
    (*g).stat.xMin = 0 as ::core::ffi::c_int as Pos;
    (*g).stat.xMax = 0 as ::core::ffi::c_int as Pos;
    (*g).stat.yMin = 0 as ::core::ffi::c_int as Pos;
    (*g).stat.yMax = 0 as ::core::ffi::c_int as Pos;
    (*g).stat.nestDepth = 0 as u16;
    (*g).stat.nPoints = 0 as u16;
    (*g).stat.nContours = 0 as u16;
    (*g).stat.nCompositePoints = 0 as u16;
    (*g).stat.nCompositeContours = 0 as u16;
    return g;
}
unsafe extern "C" fn otfcc_delete_glyf_glyph(mut g: *mut Glyph) {
    if g.is_null() {
        return;
    }
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*g).horizontalOrigin);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*g).advanceWidth);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*g).verticalOrigin);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*g).advanceHeight);
    sdsfree((*g).name);
    GLYF_I_CONTOUR_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).contours);
    GLYF_I_REFERENCE_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).references);
    GLYF_I_STEM_DEF_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).stemH);
    GLYF_I_STEM_DEF_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).stemV);
    GLYF_I_MASK_LIST.dispose.expect("non-null function pointer")(&raw mut (*g).hintMasks);
    GLYF_I_MASK_LIST.dispose.expect("non-null function pointer")(&raw mut (*g).contourMasks);
    if !(*g).instructions.is_null() {
        free((*g).instructions as *mut ::core::ffi::c_void);
        (*g).instructions = ::core::ptr::null_mut::<u8>();
    }
    otfcc_handle_dispose(&raw mut (*g).fdSelect);
    (*g).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(g as *mut ::core::ffi::c_void);
    g = ::core::ptr::null_mut::<Glyph>();
}
#[inline]
unsafe extern "C" fn init_glyf_ptr(mut g: *mut GlyphPtr) {
    *g = ::core::ptr::null_mut::<Glyph>();
}
unsafe extern "C" fn copy_glyf_ptr(mut dst: *mut GlyphPtr, mut src: *const GlyphPtr) {
    *dst = *src;
}
#[inline]
unsafe extern "C" fn dispose_glyf_ptr(mut g: *mut GlyphPtr) {
    otfcc_delete_glyf_glyph(*g);
}
pub static GLYF_I_GLYPH_PTR: GlyphPtrElementInterface = {
    GlyphPtrElementInterface {
        init: Some(init_glyf_ptr as unsafe extern "C" fn(*mut GlyphPtr) -> ()),
        copy: Some(
            copy_glyf_ptr as unsafe extern "C" fn(*mut GlyphPtr, *const GlyphPtr) -> (),
        ),
        move_0: None,
        dispose: Some(dispose_glyf_ptr as unsafe extern "C" fn(*mut GlyphPtr) -> ()),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn table_glyf_init_n(mut arr: *mut GlyfTable, mut n: usize) {
    table_glyf_init(arr);
    table_glyf_grow_to_n(arr, n);
    table_glyf_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_glyf_shrink_to_fit(mut arr: *mut GlyfTable) {
    table_glyf_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_glyf_resize_to(arr: *mut GlyfTable, target: usize) {
    cvec_resize_to(table_glyf_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_glyf_move(dst: *mut GlyfTable, src: *mut GlyfTable) {
    cvec_move(table_glyf_as_cvec(dst), table_glyf_as_cvec(src));
}
#[inline]
unsafe fn table_glyf_as_cvec(arr: *mut GlyfTable) -> *mut CVecRaw<GlyphPtr> {
    arr as *mut CVecRaw<GlyphPtr>
}
#[inline]
unsafe extern "C" fn table_glyf_init(arr: *mut GlyfTable) {
    cvec_init(table_glyf_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_glyf_filter_env(
    mut arr: *mut GlyfTable,
    mut fn_0: Option<unsafe extern "C" fn(*const GlyphPtr, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut GlyphPtr,
            env,
        ) {
            if j != k {
                let ref mut fresh10 = *(*arr).items.offset(j as isize);
                *fresh10 = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if GLYF_I_GLYPH_PTR.dispose.is_some() {
                GLYF_I_GLYPH_PTR.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut GlyphPtr,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_glyf_dispose_item(mut arr: *mut GlyfTable, mut n: usize) {
    if GLYF_I_GLYPH_PTR.dispose.is_some() {
        GLYF_I_GLYPH_PTR.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut GlyphPtr
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_glyf_sort(
    mut arr: *mut GlyfTable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const GlyphPtr, *const GlyphPtr) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<GlyphPtr>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const GlyphPtr,
                    *const GlyphPtr,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn table_glyf_fill(mut arr: *mut GlyfTable, mut n: usize) {
    while (*arr).length < n {
        let mut x: GlyphPtr = ::core::ptr::null_mut::<Glyph>();
        if GLYF_I_GLYPH_PTR.init.is_some() {
            GLYF_I_GLYPH_PTR.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<GlyphPtr>() as usize,
            );
        }
        table_glyf_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_glyf_push(arr: *mut GlyfTable, elem: GlyphPtr) {
    cvec_push(table_glyf_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_glyf_grow(arr: *mut GlyfTable) {
    cvec_grow(table_glyf_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_glyf_grow_to(arr: *mut GlyfTable, target: usize) {
    cvec_grow_to(table_glyf_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_glyf_pop(arr: *mut GlyfTable) -> GlyphPtr {
    cvec_pop(table_glyf_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_glyf_copy_replace(mut dst: *mut GlyfTable, src: GlyfTable) {
    table_glyf_dispose(dst);
    table_glyf_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_glyf_copy(mut dst: *mut GlyfTable, mut src: *const GlyfTable) {
    table_glyf_init(dst);
    table_glyf_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if GLYF_I_GLYPH_PTR.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            GLYF_I_GLYPH_PTR.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut GlyphPtr,
                (*src).items.offset(j as isize) as *mut GlyphPtr as *const GlyphPtr,
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
unsafe extern "C" fn table_glyf_dispose(mut arr: *mut GlyfTable) {
    if arr.is_null() {
        return;
    }
    if GLYF_I_GLYPH_PTR.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh14 = j;
            j = j.wrapping_sub(1);
            if !(fresh14 != 0) {
                break;
            }
            GLYF_I_GLYPH_PTR.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut GlyphPtr,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<GlyphPtr>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn table_glyf_replace(mut dst: *mut GlyfTable, src: GlyfTable) {
    table_glyf_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GlyfTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_glyf_init_cap_n(mut arr: *mut GlyfTable, mut n: usize) {
    table_glyf_init(arr);
    table_glyf_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn table_glyf_grow_to_n(arr: *mut GlyfTable, target: usize) {
    cvec_grow_to_n(table_glyf_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_glyf_free(mut x: *mut GlyfTable) {
    if x.is_null() {
        return;
    }
    table_glyf_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_glyf_create_n(mut n: usize) -> *mut GlyfTable {
    let mut t: *mut GlyfTable =
        malloc(::core::mem::size_of::<GlyfTable>() as usize) as *mut GlyfTable;
    table_glyf_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_glyf_create() -> *mut GlyfTable {
    let mut x: *mut GlyfTable =
        malloc(::core::mem::size_of::<GlyfTable>() as usize) as *mut GlyfTable;
    table_glyf_init(x);
    return x;
}
pub static TABLE_I_GLYF: GlyfTableVectorInterface = {
    GlyfTableVectorInterface {
        init: Some(table_glyf_init as unsafe extern "C" fn(*mut GlyfTable) -> ()),
        copy: Some(
            table_glyf_copy as unsafe extern "C" fn(*mut GlyfTable, *const GlyfTable) -> (),
        ),
        move_0: Some(
            table_glyf_move as unsafe extern "C" fn(*mut GlyfTable, *mut GlyfTable) -> (),
        ),
        dispose: Some(table_glyf_dispose as unsafe extern "C" fn(*mut GlyfTable) -> ()),
        replace: Some(
            table_glyf_replace as unsafe extern "C" fn(*mut GlyfTable, GlyfTable) -> (),
        ),
        copyReplace: Some(
            table_glyf_copy_replace as unsafe extern "C" fn(*mut GlyfTable, GlyfTable) -> (),
        ),
        create: Some(table_glyf_create),
        free: Some(table_glyf_free as unsafe extern "C" fn(*mut GlyfTable) -> ()),
        initN: Some(table_glyf_init_n as unsafe extern "C" fn(*mut GlyfTable, usize) -> ()),
        initCapN: Some(table_glyf_init_cap_n as unsafe extern "C" fn(*mut GlyfTable, usize) -> ()),
        createN: Some(table_glyf_create_n as unsafe extern "C" fn(usize) -> *mut GlyfTable),
        fill: Some(table_glyf_fill as unsafe extern "C" fn(*mut GlyfTable, usize) -> ()),
        clear: Some(table_glyf_dispose as unsafe extern "C" fn(*mut GlyfTable) -> ()),
        push: Some(table_glyf_push as unsafe extern "C" fn(*mut GlyfTable, GlyphPtr) -> ()),
        shrinkToFit: Some(table_glyf_shrink_to_fit as unsafe extern "C" fn(*mut GlyfTable) -> ()),
        pop: Some(table_glyf_pop as unsafe extern "C" fn(*mut GlyfTable) -> GlyphPtr),
        disposeItem: Some(
            table_glyf_dispose_item as unsafe extern "C" fn(*mut GlyfTable, usize) -> (),
        ),
        filterEnv: Some(
            table_glyf_filter_env
                as unsafe extern "C" fn(
                    *mut GlyfTable,
                    Option<
                        unsafe extern "C" fn(
                            *const GlyphPtr,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_glyf_sort
                as unsafe extern "C" fn(
                    *mut GlyfTable,
                    Option<
                        unsafe extern "C" fn(
                            *const GlyphPtr,
                            *const GlyphPtr,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
unsafe extern "C" fn glyf_glyph_dump_contours(
    mut g: *mut Glyph,
    mut target: *mut JsonValue,
    mut ctx: *const GlyfIOContext,
) {
    if (*g).contours.length == 0 {
        return;
    }
    let mut contours: *mut JsonValue = json_array_new((*g).contours.length);
    let mut k: ShapeId = 0 as ShapeId;
    while (k as usize) < (*g).contours.length {
        let mut c: *mut Contour = (*g).contours.items.offset(k as isize) as *mut Contour;
        let mut contour: *mut JsonValue = json_array_new((*c).length);
        let mut m: ShapeId = 0 as ShapeId;
        while (m as usize) < (*c).length {
            let mut point: *mut JsonValue = json_object_new(4 as usize);
            json_object_push(
                point,
                b"x\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_vq((*(*c).items.offset(m as isize)).x, (*ctx).fvar),
            );
            json_object_push(
                point,
                b"y\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_vq((*(*c).items.offset(m as isize)).y, (*ctx).fvar),
            );
            json_object_push(
                point,
                b"on\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    ((*(*c).items.offset(m as isize)).onCurve & MASK_ON_CURVE)
                        as ::core::ffi::c_int,
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
    mut g: *mut Glyph,
    mut target: *mut JsonValue,
    mut ctx: *const GlyfIOContext,
) {
    if (*g).references.length == 0 {
        return;
    }
    let mut references: *mut JsonValue = json_array_new((*g).references.length);
    let mut k: ShapeId = 0 as ShapeId;
    while (k as usize) < (*g).references.length {
        let mut r: *mut ComponentReference =
            (*g).references.items.offset(k as isize) as *mut ComponentReference;
        let mut ref_0: *mut JsonValue = json_object_new(9 as usize);
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
            json_new_vq((*r).x, (*ctx).fvar),
        );
        json_object_push(
            ref_0,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_vq((*r).y, (*ctx).fvar),
        );
        json_object_push(
            ref_0,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*r).a as Pos),
        );
        json_object_push(
            ref_0,
            b"b\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*r).b as Pos),
        );
        json_object_push(
            ref_0,
            b"c\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*r).c as Pos),
        );
        json_object_push(
            ref_0,
            b"d\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*r).d as Pos),
        );
        if (*r).isAnchored != RefAnchorStatus::Xy
        {
            json_object_push(
                ref_0,
                b"isAnchored\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(TRUE_0),
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
                json_boolean_new(TRUE_0),
            );
        }
        if (*r).useMyMetrics {
            json_object_push(
                ref_0,
                b"useMyMetrics\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(TRUE_0),
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
unsafe extern "C" fn glyf_glyph_dump_stemdefs(mut stems: *mut StemDefList) -> *mut JsonValue {
    let mut a: *mut JsonValue = json_array_new((*stems).length);
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < (*stems).length {
        let mut stem: *mut JsonValue = json_object_new(3 as usize);
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
    mut masks: *mut MaskList,
    mut hh: *mut StemDefList,
    mut vv: *mut StemDefList,
) -> *mut JsonValue {
    let mut a: *mut JsonValue = json_array_new((*masks).length);
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < (*masks).length {
        let mut mask: *mut JsonValue = json_object_new(3 as usize);
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
        let mut h: *mut JsonValue = json_array_new((*hh).length);
        let mut k: ShapeId = 0 as ShapeId;
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
        let mut v: *mut JsonValue = json_array_new((*vv).length);
        let mut k_0: ShapeId = 0 as ShapeId;
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
    mut g: *mut Glyph,
    mut options: *const Options,
    mut ctx: *const GlyfIOContext,
) -> *mut JsonValue {
    let mut glyph: *mut JsonValue = json_object_new(12 as usize);
    json_object_push(
        glyph,
        b"advanceWidth\0" as *const u8 as *const ::core::ffi::c_char,
        json_new_vq((*g).advanceWidth, (*ctx).fvar),
    );
    if I_VQ.isStill.expect("non-null function pointer")((*g).horizontalOrigin) as ::core::ffi::c_int
        != 0
        && fabs(
            I_VQ.getStill.expect("non-null function pointer")((*g).horizontalOrigin)
                as ::core::ffi::c_double,
        ) > 1.0f64 / 1000.0f64
    {
        json_object_push(
            glyph,
            b"horizontalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_vq((*g).horizontalOrigin, (*ctx).fvar),
        );
    }
    if (*ctx).hasVerticalMetrics {
        json_object_push(
            glyph,
            b"advanceHeight\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_vq((*g).advanceHeight, (*ctx).fvar),
        );
        json_object_push(
            glyph,
            b"verticalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_vq((*g).verticalOrigin, (*ctx).fvar),
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
pub unsafe extern "C" fn otfcc_dump_glyphorder(
    mut table: *const GlyfTable,
    mut root: *mut JsonValue,
) {
    if table.is_null() {
        return;
    }
    let mut order: *mut JsonValue = json_array_new((*table).length);
    let mut j: GlyphId = 0 as GlyphId;
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
pub unsafe extern "C" fn otfcc_dump_glyf(
    mut table: *const GlyfTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
    mut ctx: *const GlyfIOContext,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"glyf"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut glyf: *mut JsonValue = json_object_new((*table).length);
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < (*table).length {
            let mut g: *mut Glyph = *(*table).items.offset(j as isize) as *mut Glyph;
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
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
unsafe extern "C" fn glyf_parse_point(mut pointdump: *mut JsonValue) -> Point {
    let mut point: Point = Point {
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
        onCurve: 0,
    };
    GLYF_I_POINT.init.expect("non-null function pointer")(&raw mut point);
    if pointdump.is_null()
        || (*pointdump).type_0 != JsonType::Object
    {
        return point;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*pointdump).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char =
            (*(*pointdump).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut JsonValue =
            (*(*pointdump).u.object.values.offset(_k as isize)).value as *mut JsonValue;
        if strcmp(ck, b"x\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        {
            I_VQ.replace.expect("non-null function pointer")(
                &raw mut point.x,
                json_vq_of(cv, ::core::ptr::null::<FvarTable>()) as VQ,
            );
        } else if strcmp(ck, b"y\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            I_VQ.replace.expect("non-null function pointer")(
                &raw mut point.y,
                json_vq_of(cv, ::core::ptr::null::<FvarTable>()) as VQ,
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
unsafe extern "C" fn glyf_parse_contours(mut col: *mut JsonValue, mut g: *mut Glyph) {
    if col.is_null() {
        return;
    }
    let mut nContours: ShapeId = (*col).u.array.length as ShapeId;
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_int) < nContours as ::core::ffi::c_int {
        let mut contourdump: *mut JsonValue =
            *(*col).u.array.values.offset(j as isize) as *mut JsonValue;
        let mut contour: Contour = Contour {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<Point>(),
        };
        GLYF_I_CONTOUR.initCapN.expect("non-null function pointer")(
            &raw mut contour,
            (if !contourdump.is_null()
                && (*contourdump).type_0 == JsonType::Array
            {
                (*contourdump).u.array.length
            } else {
                1 as ::core::ffi::c_uint
            }) as usize,
        );
        if !contourdump.is_null()
            && (*contourdump).type_0 == JsonType::Array
        {
            let mut k: ShapeId = 0 as ShapeId;
            while (k as ::core::ffi::c_uint) < (*contourdump).u.array.length {
                GLYF_I_CONTOUR.push.expect("non-null function pointer")(
                    &raw mut contour,
                    glyf_parse_point(
                        *(*contourdump).u.array.values.offset(k as isize) as *mut JsonValue
                    ),
                );
                k = k.wrapping_add(1);
            }
        }
        GLYF_I_CONTOUR_LIST.push.expect("non-null function pointer")(&raw mut (*g).contours, contour);
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn glyf_parse_reference(mut refdump: *mut JsonValue) -> ComponentReference {
    let mut _gname: *mut JsonValue = json_obj_get_type(
        refdump,
        b"glyph\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::String,
    );
    let mut ref_0: ComponentReference =
        (
            GLYF_I_COMPONENT_REFERENCE
                .empty
                .expect("non-null function pointer"))();
    if !_gname.is_null() {
        ref_0.glyph = handle_from_name(sdsnewlen(
            (*_gname).u.string.ptr as *const ::core::ffi::c_void,
            (*_gname).u.string.length as usize,
        )) as GlyphHandle;
        I_VQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.x,
            json_vq_of(
                json_obj_get(refdump, b"x\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<FvarTable>(),
            ) as VQ,
        );
        I_VQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.y,
            json_vq_of(
                json_obj_get(refdump, b"y\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<FvarTable>(),
            ) as VQ,
        );
        ref_0.a = json_obj_getnum_fallback(
            refdump,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
            1.0f64,
        ) as Scale;
        ref_0.b = json_obj_getnum_fallback(
            refdump,
            b"b\0" as *const u8 as *const ::core::ffi::c_char,
            0.0f64,
        ) as Scale;
        ref_0.c = json_obj_getnum_fallback(
            refdump,
            b"c\0" as *const u8 as *const ::core::ffi::c_char,
            0.0f64,
        ) as Scale;
        ref_0.d = json_obj_getnum_fallback(
            refdump,
            b"d\0" as *const u8 as *const ::core::ffi::c_char,
            1.0f64,
        ) as Scale;
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
            ref_0.isAnchored = RefAnchorStatus::AnchorXy;
            ref_0.inner = json_obj_getint(
                refdump,
                b"inner\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ShapeId;
            ref_0.outer = json_obj_getint(
                refdump,
                b"outer\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ShapeId;
        }
    } else {
        ref_0.glyph.name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        I_VQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.x,
            I_VQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos)
                as VQ,
        );
        I_VQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.y,
            I_VQ.createStill.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos)
                as VQ,
        );
        ref_0.a = 1.0f64 as Scale;
        ref_0.b = 0.0f64 as Scale;
        ref_0.c = 0.0f64 as Scale;
        ref_0.d = 1.0f64 as Scale;
        ref_0.roundToGrid = false;
        ref_0.useMyMetrics = false;
    }
    return ref_0;
}
unsafe extern "C" fn glyf_parse_references(mut col: *mut JsonValue, mut g: *mut Glyph) {
    if col.is_null() {
        return;
    }
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_uint) < (*col).u.array.length {
        GLYF_I_REFERENCE_LIST.push.expect("non-null function pointer")(
            &raw mut (*g).references,
            glyf_parse_reference(*(*col).u.array.values.offset(j as isize) as *mut JsonValue),
        );
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn make_instrs_for_glyph(
    mut _g: *mut ::core::ffi::c_void,
    mut instrs: *mut u8,
    mut len: u32,
) {
    let mut g: *mut Glyph = _g as *mut Glyph;
    (*g).instructionsLength = len as u16;
    (*g).instructions = instrs;
}
unsafe extern "C" fn wrong_instrs_for_glyph(
    mut _g: *mut ::core::ffi::c_void,
    mut reason: *mut ::core::ffi::c_char,
    mut pos: ::core::ffi::c_int,
) {
    let mut g: *mut Glyph = _g as *mut Glyph;
    fprintf(
        stderr,
        b"[OTFCC] TrueType instructions parse error : %s, at %d in /%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        reason,
        pos,
        (*g).name,
    );
}
unsafe extern "C" fn parse_stems(mut sd: *mut JsonValue, mut stems: *mut StemDefList) {
    if sd.is_null() {
        return;
    }
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_uint) < (*sd).u.array.length {
        let mut s: *mut JsonValue = *(*sd).u.array.values.offset(j as isize) as *mut JsonValue;
        if !((*s).type_0 != JsonType::Object)
        {
            let mut sdef: PostscriptStemDef = PostscriptStemDef {
                position: 0.,
                width: 0.,
                map: 0,
            };
            sdef.map = 0 as u16;
            sdef.position =
                json_obj_getnum(s, b"position\0" as *const u8 as *const ::core::ffi::c_char)
                    as Pos;
            sdef.width =
                json_obj_getnum(s, b"width\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
            GLYF_I_STEM_DEF_LIST.push.expect("non-null function pointer")(stems, sdef);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn parse_maskbits(mut arr: *mut bool, mut bits: *mut JsonValue) {
    if bits.is_null() {
        let mut j: ShapeId = 0 as ShapeId;
        while (j as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int {
            *arr.offset(j as isize) = false;
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: ShapeId = 0 as ShapeId;
        while (j_0 as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int
            && (j_0 as ::core::ffi::c_uint) < (*bits).u.array.length
        {
            let mut b: *mut JsonValue =
                *(*bits).u.array.values.offset(j_0 as isize) as *mut JsonValue;
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
unsafe extern "C" fn parse_masks(mut md: *mut JsonValue, mut masks: *mut MaskList) {
    if md.is_null() {
        return;
    }
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_uint) < (*md).u.array.length {
        let mut m: *mut JsonValue = *(*md).u.array.values.offset(j as isize) as *mut JsonValue;
        if !((*m).type_0 != JsonType::Object)
        {
            let mut mask: PostscriptHintMask = PostscriptHintMask {
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
                    JsonType::Array,
                ),
            );
            parse_maskbits(
                (&raw mut mask.maskV as *mut bool).offset(0 as ::core::ffi::c_int as isize)
                    as *mut bool,
                json_obj_get_type(
                    m,
                    b"maskV\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                ),
            );
            GLYF_I_MASK_LIST.push.expect("non-null function pointer")(masks, mask);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn otfcc_glyf_parse_glyph(
    mut glyphdump: *mut JsonValue,
    mut order_entry: *mut GlyphOrderEntry,
    mut options: *const Options,
) -> *mut Glyph {
    let mut g: *mut Glyph = otfcc_new_glyf_glyph();
    (*g).name = sdsdup((*order_entry).name);
    I_VQ.replace.expect("non-null function pointer")(
        &raw mut (*g).advanceWidth,
        json_vq_of(
            json_obj_get(
                glyphdump,
                b"advanceWidth\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    I_VQ.replace.expect("non-null function pointer")(
        &raw mut (*g).horizontalOrigin,
        json_vq_of(
            json_obj_get(
                glyphdump,
                b"horizontalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    I_VQ.replace.expect("non-null function pointer")(
        &raw mut (*g).advanceHeight,
        json_vq_of(
            json_obj_get(
                glyphdump,
                b"advanceHeight\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    I_VQ.replace.expect("non-null function pointer")(
        &raw mut (*g).verticalOrigin,
        json_vq_of(
            json_obj_get(
                glyphdump,
                b"verticalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    glyf_parse_contours(
        json_obj_get_type(
            glyphdump,
            b"contours\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        ),
        g,
    );
    glyf_parse_references(
        json_obj_get_type(
            glyphdump,
            b"references\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
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
                make_instrs_for_glyph
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut u8, u32) -> (),
            ),
            Some(
                wrong_instrs_for_glyph
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
                JsonType::Array,
            ),
            &raw mut (*g).stemH,
        );
        parse_stems(
            json_obj_get_type(
                glyphdump,
                b"stemV\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            ),
            &raw mut (*g).stemV,
        );
        parse_masks(
            json_obj_get_type(
                glyphdump,
                b"hintMasks\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            ),
            &raw mut (*g).hintMasks,
        );
        parse_masks(
            json_obj_get_type(
                glyphdump,
                b"contourMasks\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            ),
            &raw mut (*g).contourMasks,
        );
        (*g).yPel = json_obj_getint(
            glyphdump,
            b"LTSH_yPel\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u8;
    }
    (*g).fdSelect = handle_from_name(json_obj_getsds(
        glyphdump,
        b"CFF_fdSelect\0" as *const u8 as *const ::core::ffi::c_char,
    )) as FdHandle;
    if (*g).yPel == 0 {
        (*g).yPel = json_obj_getint(
            glyphdump,
            b"yPel\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u8;
    }
    return g;
}
pub unsafe extern "C" fn otfcc_parse_glyf(
    mut root: *const JsonValue,
    mut glyph_order: *mut GlyphOrder,
    mut options: *const Options,
) -> *mut GlyfTable {
    if (*root).type_0 != JsonType::Object
        || glyph_order.is_null()
    {
        return ::core::ptr::null_mut::<GlyfTable>();
    }
    let mut glyf: *mut GlyfTable = ::core::ptr::null_mut::<GlyfTable>();
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"glyf"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut numGlyphs: GlyphId = (*table).u.object.length as GlyphId;
            glyf = TABLE_I_GLYF.createN.expect("non-null function pointer")(numGlyphs as usize);
            let mut j: GlyphId = 0 as GlyphId;
            while (j as ::core::ffi::c_int) < numGlyphs as ::core::ffi::c_int {
                let mut gname: SdsRaw = sdsnewlen(
                    (*(*table).u.object.values.offset(j as isize)).name
                        as *const ::core::ffi::c_void,
                    (*(*table).u.object.values.offset(j as isize)).name_length as usize,
                );
                let mut glyphdump: *mut JsonValue =
                    (*(*table).u.object.values.offset(j as isize)).value as *mut JsonValue;
                let mut order_entry: *mut GlyphOrderEntry =
                    ::core::ptr::null_mut::<GlyphOrderEntry>();
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
                order_entry = ::core::ptr::null_mut::<GlyphOrderEntry>();
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
                                as *mut GlyphOrderEntry
                                as *mut GlyphOrderEntry;
                        } else {
                            order_entry = ::core::ptr::null_mut::<GlyphOrderEntry>();
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
                                    as *mut GlyphOrderEntry
                                    as *mut GlyphOrderEntry;
                            } else {
                                order_entry = ::core::ptr::null_mut::<GlyphOrderEntry>();
                            }
                        }
                    }
                }
                if (*glyphdump).type_0 == JsonType::Object
                    && !order_entry.is_null()
                    && (*(*glyf).items.offset((*order_entry).gid as isize)).is_null()
                {
                    let ref mut fresh15 = *(*glyf).items.offset((*order_entry).gid as isize);
                    *fresh15 =
                        otfcc_glyf_parse_glyph(glyphdump, order_entry, options) as GlyphPtr;
                }
                json_value_free(glyphdump);
                let mut v: *mut JsonValue = json_null_new();
                (*v).parent = table as *mut JsonValue;
                let ref mut fresh16 = (*(*table).u.object.values.offset(j as isize)).value;
                *fresh16 = v as *mut JsonValue;
                sdsfree(gname);
                j = j.wrapping_add(1);
            }
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
        return glyf;
    }
    return ::core::ptr::null_mut::<GlyfTable>();
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyfAndLocaBuffers {
    pub glyf: *mut Buffer,
    pub loca: *mut Buffer,
}

bitflags::bitflags! {
    /// The flag byte that introduces each point of a simple glyph's outline, as
    /// `glyf` stores it: one byte per point, run-length encoded through
    /// [`REPEAT`](Self::REPEAT).
    ///
    /// `SAME_X`/`POSITIVE_X` are deliberately one bit, not two. The spec calls
    /// bit 4 `X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR`, and which of the two things
    /// it means depends on `X_SHORT`: with it, the delta is positive; without it,
    /// the delta is zero. Both readings are load-bearing, so both names stay --
    /// `bitflags` allows the alias, and
    /// `point_flag_aliases_are_the_same_bit` pins that they agree.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct PointFlags: u8 {
        const ON_CURVE = 1;
        const X_SHORT = 2;
        const Y_SHORT = 4;
        const REPEAT = 8;
        const SAME_X = 16;
        const POSITIVE_X = 16;
        const SAME_Y = 32;
        const POSITIVE_Y = 32;
    }
}

bitflags::bitflags! {
    /// The flag word that introduces each component of a composite glyph. Names
    /// are the OpenType spec's own, verbatim, so they can be grepped against it.
    ///
    /// [`OVERLAP_COMPOUND`](Self::OVERLAP_COMPOUND) is the one flag otfcc never
    /// reads or writes; it stays declared because a bit set with a hole in it is
    /// harder to check against the spec than one carrying an unused name.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct ComponentFlags: u16 {
        const ARG_1_AND_2_ARE_WORDS = 1;
        const ARGS_ARE_XY_VALUES = 2;
        const ROUND_XY_TO_GRID = 4;
        const WE_HAVE_A_SCALE = 8;
        const MORE_COMPONENTS = 32;
        const WE_HAVE_AN_X_AND_Y_SCALE = 64;
        const WE_HAVE_A_TWO_BY_TWO = 128;
        const WE_HAVE_INSTRUCTIONS = 256;
        const USE_MY_METRICS = 512;
        const OVERLAP_COMPOUND = 1024;
        const SCALED_COMPONENT_OFFSET = 2048;
        const UNSCALED_COMPONENT_OFFSET = 4096;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `glyf/build.rs` writes a component's flags based on
    // `isAnchored == RefAnchorStatus::AnchorConsolidated`, so these values pick which branch
    // of the composite-glyph encoding runs. They come from otfcc's own
    // consolidation pass, never off the wire, but they are still load-bearing for
    // the bytes that come out.
    // Bit 4 of a point flag means "same x" or "positive x" depending on
    // `X_SHORT`, and bit 5 the same for y. C spelled that as two constants with
    // one value each; `bitflags` keeps both names, so pin that they still are
    // one bit -- if a future edit split them, the outline coordinates would be
    // decoded against the wrong bit and every simple glyph would move.
    #[test]
    fn point_flag_aliases_are_the_same_bit() {
        assert_eq!(PointFlags::SAME_X, PointFlags::POSITIVE_X);
        assert_eq!(PointFlags::SAME_Y, PointFlags::POSITIVE_Y);
        assert_eq!(PointFlags::SAME_X.bits(), 16);
        assert_eq!(PointFlags::SAME_Y.bits(), 32);
    }

    // The flag byte/word goes to the wire exactly as built, so the encoding is
    // the output. `from_bits_retain` is what keeps a bit otfcc does not know
    // about from being dropped on the way in -- the same reason `LookupType`
    // is a newtype (see rust/README.md).
    #[test]
    fn glyf_flag_bits_are_the_wire_encoding() {
        assert_eq!(PointFlags::ON_CURVE.bits(), 1);
        assert_eq!(PointFlags::X_SHORT.bits(), 2);
        assert_eq!(PointFlags::Y_SHORT.bits(), 4);
        assert_eq!(PointFlags::REPEAT.bits(), 8);

        assert_eq!(ComponentFlags::ARG_1_AND_2_ARE_WORDS.bits(), 1);
        assert_eq!(ComponentFlags::ARGS_ARE_XY_VALUES.bits(), 2);
        assert_eq!(ComponentFlags::ROUND_XY_TO_GRID.bits(), 4);
        assert_eq!(ComponentFlags::WE_HAVE_A_SCALE.bits(), 8);
        assert_eq!(ComponentFlags::MORE_COMPONENTS.bits(), 32);
        assert_eq!(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE.bits(), 64);
        assert_eq!(ComponentFlags::WE_HAVE_A_TWO_BY_TWO.bits(), 128);
        assert_eq!(ComponentFlags::WE_HAVE_INSTRUCTIONS.bits(), 256);
        assert_eq!(ComponentFlags::USE_MY_METRICS.bits(), 512);
        assert_eq!(ComponentFlags::OVERLAP_COMPOUND.bits(), 1024);
        assert_eq!(ComponentFlags::SCALED_COMPONENT_OFFSET.bits(), 2048);
        assert_eq!(ComponentFlags::UNSCALED_COMPONENT_OFFSET.bits(), 4096);

        // Bit 3 of a component word (8 in the point set, `WE_HAVE_A_SCALE` here)
        // is the one number that means different things in the two sets, and
        // both types being distinct is what stops them being mixed up.
        let unknown = ComponentFlags::from_bits_retain(0x8000);
        assert_eq!(unknown.bits(), 0x8000);
        assert!(!unknown.contains(ComponentFlags::MORE_COMPONENTS));
    }

    #[test]
    fn refanchorstatus_discriminants_match_the_c_enum() {
        assert_eq!(RefAnchorStatus::Xy as u32, 0);
        assert_eq!(RefAnchorStatus::AnchorAnchor as u32, 1);
        assert_eq!(RefAnchorStatus::AnchorXy as u32, 2);
        assert_eq!(RefAnchorStatus::AnchorConsolidated as u32, 3);
        assert_eq!(RefAnchorStatus::AnchorConsolidatingAnchor as u32, 4);
        assert_eq!(RefAnchorStatus::AnchorConsolidatingXy as u32, 5);
    }
}
