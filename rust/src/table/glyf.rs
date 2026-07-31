#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
#![allow(improper_ctypes_definitions)] // VQ now owns a Vec; these extern "C" fns are internal-only (vtable dispatch, no real FFI boundary) -- goes away with the vtable/extern "C" cleanup, see rust/README.md
pub mod build;
pub mod read;

use libc::{fprintf, free, malloc, memcmp, memcpy, memset, qsort, strcmp};
unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::handle::{HandleState, handle_from_name, FdHandle, GlyphHandle, Handle, otfcc_handle_copy, otfcc_handle_dispose, otfcc_handle_empty};
use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{ILogger};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, Pos, Scale, ShapeId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{JsonValue, JsonType};
use crate::support::cvec::{CVecRaw, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::buffer::{Buffer};
use crate::support::{ComparFn, TRUE_0};
use crate::support::glyph_order::{GlyphOrder, GlyphOrderEntry};
use crate::table::fvar::{FvarTable};



use crate::vf::vq::{VQ};
use crate::support::json_funcs::{json_boolof, json_new_position, json_obj_get, json_obj_get_type, json_obj_getbool, json_obj_getint, json_obj_getnum, json_obj_getnum_fallback, json_obj_getsds, preserialize};
use crate::support::ttinstr::{dump_ttinstr, parse_ttinstr};
use crate::table::fvar::{json_new_vq, json_vq_of};
use crate::vendor::json::{json_value_free};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_boolean_new, json_integer_new, json_null_new, json_object_new, json_object_push, json_string_new, json_string_new_length};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree, sdsnewlen};
use crate::vf::vq::{I_VQ};

#[derive(Clone)]
#[repr(C)]
pub struct Point {
    pub x: VQ,
    pub y: VQ,
    pub on_curve: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PointElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut Point) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut Point, *const Point) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut Point) -> ()>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut Contour) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut Contour>,
    pub free: Option<unsafe extern "C" fn(*mut Contour) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut Contour, usize) -> ()>,
    pub fill: Option<unsafe extern "C" fn(*mut Contour, usize) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut Contour, Point) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut Contour) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut Contour) -> Point>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut ContourList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ContourList>,
    pub free: Option<unsafe extern "C" fn(*mut ContourList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut ContourList, Contour) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut ContourList) -> ()>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut ContourList, usize) -> ()>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut PostscriptStemDef) -> ()>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut StemDefList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut StemDefList>,
    pub free: Option<unsafe extern "C" fn(*mut StemDefList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut StemDefList, PostscriptStemDef) -> ()>,
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
    pub points_before: u16,
    pub contours_before: u16,
    pub mask_h: [bool; 256],
    pub mask_v: [bool; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PostscriptHintMaskElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut PostscriptHintMask) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut PostscriptHintMask, *const PostscriptHintMask) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut PostscriptHintMask) -> ()>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut MaskList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut MaskList>,
    pub free: Option<unsafe extern "C" fn(*mut MaskList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut MaskList, PostscriptHintMask) -> ()>,
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
#[derive(Clone)]
#[repr(C)]
pub struct ComponentReference {
    pub x: VQ,
    pub y: VQ,
    pub round_to_grid: bool,
    pub use_my_metrics: bool,
    pub glyph: GlyphHandle,
    pub a: Scale,
    pub b: Scale,
    pub c: Scale,
    pub d: Scale,
    pub is_anchored: RefAnchorStatus,
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
    pub dispose: Option<unsafe extern "C" fn(*mut ComponentReference) -> ()>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut ReferenceList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ReferenceList>,
    pub free: Option<unsafe extern "C" fn(*mut ReferenceList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut ReferenceList, ComponentReference) -> ()>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut ReferenceList, usize) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphStat {
    pub x_min: Pos,
    pub x_max: Pos,
    pub y_min: Pos,
    pub y_max: Pos,
    pub nest_depth: u16,
    pub n_points: u16,
    pub n_contours: u16,
    pub n_composite_points: u16,
    pub n_composite_contours: u16,
}
#[derive(Clone)]
#[repr(C)]
pub struct Glyph {
    pub name: SdsRaw,
    pub horizontal_origin: VQ,
    pub advance_width: VQ,
    pub vertical_origin: VQ,
    pub advance_height: VQ,
    pub contours: ContourList,
    pub references: ReferenceList,
    pub stem_h: StemDefList,
    pub stem_v: StemDefList,
    pub hint_masks: MaskList,
    pub contour_masks: MaskList,
    pub instructions_length: u16,
    pub instructions: *mut u8,
    pub y_pel: u8,
    pub fd_select: FdHandle,
    pub cid: GlyphId,
    pub stat: GlyphStat,
}
pub type GlyphPtr = *mut Glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphPtrElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GlyphPtr) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut GlyphPtr, *const GlyphPtr) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GlyphPtr) -> ()>,
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
    pub dispose: Option<unsafe extern "C" fn(*mut GlyfTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GlyfTable>,
    pub free: Option<unsafe extern "C" fn(*mut GlyfTable) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut GlyfTable>,
    pub push: Option<unsafe extern "C" fn(*mut GlyfTable, GlyphPtr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyfIOContext {
    pub loca_is_long: bool,
    pub num_glyphs: GlyphId,
    pub n_phantom_points: ShapeId,
    pub fvar: *mut FvarTable,
    pub has_vertical_metrics: bool,
    pub export_fd_select: bool,
}
/// The only bit of [`Point::on_curve`] that means anything.
///
/// Not a flag *set*, despite C giving it a `glyf_OnCurveMask` type of its own:
/// `on_curve` is an `i8` holding 0 or 1, and both readers of the field mask it
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
    (*p).x = I_VQ.create_still.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
    (*p).y = I_VQ.create_still.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
    (*p).on_curve = TRUE_0 as i8;
}
unsafe extern "C" fn copy_point(mut dst: *mut Point, mut src: *const Point) {
    I_VQ.copy.expect("non-null function pointer")(&raw mut (*dst).x, &raw const (*src).x);
    I_VQ.copy.expect("non-null function pointer")(&raw mut (*dst).y, &raw const (*src).y);
    (*dst).on_curve = (*src).on_curve;
}
unsafe extern "C" fn dispose_point(mut p: *mut Point) {
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*p).x);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*p).y);
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
            shift: Vec::new(),
        },
        y: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        on_curve: 0,
    };
    glyf_point_init(&raw mut x);
    return x;
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
        dispose: Some(glyf_point_dispose as unsafe extern "C" fn(*mut Point) -> ()),
        empty: Some(glyf_point_empty),
        dup: Some(glyf_point_dup as unsafe extern "C" fn(Point) -> Point),
    }
};
#[inline]
unsafe extern "C" fn glyf_point_dup(src: Point) -> Point {
    let mut dst: Point = Point {
        x: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        y: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        on_curve: 0,
    };
    glyf_point_copy(&raw mut dst, &raw const src);
    return dst;
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
        dispose: Some(glyf_contour_dispose as unsafe extern "C" fn(*mut Contour) -> ()),
        create: Some(glyf_contour_create),
        free: Some(glyf_contour_free as unsafe extern "C" fn(*mut Contour) -> ()),
        init_cap_n: Some(
            glyf_contour_init_cap_n as unsafe extern "C" fn(*mut Contour, usize) -> (),
        ),
        fill: Some(glyf_contour_fill as unsafe extern "C" fn(*mut Contour, usize) -> ()),
        push: Some(glyf_contour_push as unsafe extern "C" fn(*mut Contour, Point) -> ()),
        shrink_to_fit: Some(
            glyf_contour_shrink_to_fit as unsafe extern "C" fn(*mut Contour) -> (),
        ),
        pop: Some(glyf_contour_pop as unsafe extern "C" fn(*mut Contour) -> Point),
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
unsafe fn glyf_contour_as_cvec(arr: *mut Contour) -> *mut CVecRaw<Point> {
    arr as *mut CVecRaw<Point>
}
#[inline]
unsafe extern "C" fn glyf_contour_init(arr: *mut Contour) {
    cvec_init(glyf_contour_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn glyf_contour_fill(mut arr: *mut Contour, mut n: usize) {
    while (*arr).length < n {
        let mut x: Point = Point {
            x: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            y: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            on_curve: 0,
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
unsafe extern "C" fn glyf_contour_grow_to(arr: *mut Contour, target: usize) {
    cvec_grow_to(glyf_contour_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_contour_pop(arr: *mut Contour) -> Point {
    cvec_pop(glyf_contour_as_cvec(arr))
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
            *(*dst).items.offset(j_0 as isize) = (*(*src).items.offset(j_0 as isize)).clone();
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
unsafe extern "C" fn glyf_contour_init_cap_n(mut arr: *mut Contour, mut n: usize) {
    glyf_contour_init(arr);
    glyf_contour_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn glyf_contour_grow_to_n(arr: *mut Contour, target: usize) {
    cvec_grow_to_n(glyf_contour_as_cvec(arr), target);
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
        dispose: Some(
            glyf_contour_list_dispose as unsafe extern "C" fn(*mut ContourList) -> (),
        ),
        create: Some(glyf_contour_list_create),
        free: Some(glyf_contour_list_free as unsafe extern "C" fn(*mut ContourList) -> ()),
        push: Some(
            glyf_contour_list_push
                as unsafe extern "C" fn(*mut ContourList, Contour) -> (),
        ),
        shrink_to_fit: Some(
            glyf_contour_list_shrink_to_fit as unsafe extern "C" fn(*mut ContourList) -> (),
        ),
        dispose_item: Some(
            glyf_contour_list_dispose_item
                as unsafe extern "C" fn(*mut ContourList, usize) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_contour_list_shrink_to_fit(mut arr: *mut ContourList) {
    glyf_contour_list_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_push(arr: *mut ContourList, elem: Contour) {
    cvec_push(glyf_contour_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_grow_to(arr: *mut ContourList, target: usize) {
    cvec_grow_to(glyf_contour_list_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn glyf_contour_list_resize_to(arr: *mut ContourList, target: usize) {
    cvec_resize_to(glyf_contour_list_as_cvec(arr), target);
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
unsafe extern "C" fn init_glyf_reference(mut ref_0: *mut ComponentReference) {
    (*ref_0).glyph = otfcc_handle_empty() as GlyphHandle;
    (*ref_0).x =
        I_VQ.create_still.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
    (*ref_0).y =
        I_VQ.create_still.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos);
    (*ref_0).a = 1 as ::core::ffi::c_int as Scale;
    (*ref_0).b = 0 as ::core::ffi::c_int as Scale;
    (*ref_0).c = 0 as ::core::ffi::c_int as Scale;
    (*ref_0).d = 1 as ::core::ffi::c_int as Scale;
    (*ref_0).is_anchored = RefAnchorStatus::Xy;
    (*ref_0).outer = 0 as ShapeId;
    (*ref_0).inner = (*ref_0).outer;
    (*ref_0).round_to_grid = false;
    (*ref_0).use_my_metrics = false;
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
    (*dst).is_anchored = (*src).is_anchored;
    (*dst).inner = (*src).inner;
    (*dst).outer = (*src).outer;
    (*dst).round_to_grid = (*src).round_to_grid;
    (*dst).use_my_metrics = (*src).use_my_metrics;
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
unsafe extern "C" fn glyf_component_reference_dup(
    src: ComponentReference,
) -> ComponentReference {
    let mut dst: ComponentReference = ComponentReference {
        x: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        y: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        round_to_grid: false,
        use_my_metrics: false,
        glyph: Handle {
            state: HandleState::Empty,
            index: 0,
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        is_anchored: RefAnchorStatus::Xy,
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
unsafe extern "C" fn glyf_component_reference_empty() -> ComponentReference {
    let mut x: ComponentReference = ComponentReference {
        x: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        y: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        round_to_grid: false,
        use_my_metrics: false,
        glyph: Handle {
            state: HandleState::Empty,
            index: 0,
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        is_anchored: RefAnchorStatus::Xy,
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
        dispose: Some(
            glyf_component_reference_dispose
                as unsafe extern "C" fn(*mut ComponentReference) -> (),
        ),
        empty: Some(glyf_component_reference_empty),
        dup: Some(
            glyf_component_reference_dup
                as unsafe extern "C" fn(ComponentReference) -> ComponentReference,
        ),
    }
};
#[inline]
unsafe extern "C" fn glyf_reference_list_push(arr: *mut ReferenceList, elem: ComponentReference) {
    cvec_push(glyf_reference_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_reference_list_grow_to(arr: *mut ReferenceList, target: usize) {
    cvec_grow_to(glyf_reference_list_as_cvec(arr), target);
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
            *(*dst).items.offset(j_0 as isize) = (*(*src).items.offset(j_0 as isize)).clone();
            j_0 = j_0.wrapping_add(1);
        }
    };
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
unsafe extern "C" fn glyf_reference_list_create() -> *mut ReferenceList {
    let mut x: *mut ReferenceList =
        malloc(::core::mem::size_of::<ReferenceList>() as usize) as *mut ReferenceList;
    glyf_reference_list_init(x);
    return x;
}
pub static GLYF_I_REFERENCE_LIST: ReferenceListVectorInterface = {
    ReferenceListVectorInterface {
        init: Some(glyf_reference_list_init as unsafe extern "C" fn(*mut ReferenceList) -> ()),
        copy: Some(
            glyf_reference_list_copy
                as unsafe extern "C" fn(*mut ReferenceList, *const ReferenceList) -> (),
        ),
        dispose: Some(
            glyf_reference_list_dispose as unsafe extern "C" fn(*mut ReferenceList) -> (),
        ),
        create: Some(glyf_reference_list_create),
        free: Some(glyf_reference_list_free as unsafe extern "C" fn(*mut ReferenceList) -> ()),
        push: Some(
            glyf_reference_list_push
                as unsafe extern "C" fn(*mut ReferenceList, ComponentReference) -> (),
        ),
        dispose_item: Some(
            glyf_reference_list_dispose_item
                as unsafe extern "C" fn(*mut ReferenceList, usize) -> (),
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
        dispose: Some(
            glyf_postscript_stem_def_dispose
                as unsafe extern "C" fn(*mut PostscriptStemDef) -> (),
        ),
    }
};
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
unsafe fn glyf_stem_def_list_as_cvec(arr: *mut StemDefList) -> *mut CVecRaw<PostscriptStemDef> {
    arr as *mut CVecRaw<PostscriptStemDef>
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_init(arr: *mut StemDefList) {
    cvec_init(glyf_stem_def_list_as_cvec(arr));
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
unsafe extern "C" fn glyf_stem_def_list_push(arr: *mut StemDefList, elem: PostscriptStemDef) {
    cvec_push(glyf_stem_def_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_stem_def_list_grow_to(arr: *mut StemDefList, target: usize) {
    cvec_grow_to(glyf_stem_def_list_as_cvec(arr), target);
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
        dispose: Some(
            glyf_stem_def_list_dispose as unsafe extern "C" fn(*mut StemDefList) -> (),
        ),
        create: Some(glyf_stem_def_list_create),
        free: Some(glyf_stem_def_list_free as unsafe extern "C" fn(*mut StemDefList) -> ()),
        push: Some(
            glyf_stem_def_list_push
                as unsafe extern "C" fn(*mut StemDefList, PostscriptStemDef) -> (),
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
        dispose: Some(
            glyf_postscript_hint_mask_dispose
                as unsafe extern "C" fn(*mut PostscriptHintMask) -> (),
        ),
    }
};
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
        dispose: Some(glyf_mask_list_dispose as unsafe extern "C" fn(*mut MaskList) -> ()),
        create: Some(glyf_mask_list_create),
        free: Some(glyf_mask_list_free as unsafe extern "C" fn(*mut MaskList) -> ()),
        push: Some(
            glyf_mask_list_push
                as unsafe extern "C" fn(*mut MaskList, PostscriptHintMask) -> (),
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
unsafe fn glyf_mask_list_as_cvec(arr: *mut MaskList) -> *mut CVecRaw<PostscriptHintMask> {
    arr as *mut CVecRaw<PostscriptHintMask>
}
#[inline]
unsafe extern "C" fn glyf_mask_list_init(arr: *mut MaskList) {
    cvec_init(glyf_mask_list_as_cvec(arr));
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
unsafe extern "C" fn glyf_mask_list_push(arr: *mut MaskList, elem: PostscriptHintMask) {
    cvec_push(glyf_mask_list_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn glyf_mask_list_grow_to(arr: *mut MaskList, target: usize) {
    cvec_grow_to(glyf_mask_list_as_cvec(arr), target);
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
unsafe extern "C" fn glyf_mask_list_create() -> *mut MaskList {
    let mut x: *mut MaskList =
        malloc(::core::mem::size_of::<MaskList>() as usize) as *mut MaskList;
    glyf_mask_list_init(x);
    return x;
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
    I_VQ.init.expect("non-null function pointer")(&raw mut (*g).horizontal_origin);
    I_VQ.init.expect("non-null function pointer")(&raw mut (*g).advance_width);
    I_VQ.init.expect("non-null function pointer")(&raw mut (*g).vertical_origin);
    I_VQ.init.expect("non-null function pointer")(&raw mut (*g).advance_height);
    GLYF_I_CONTOUR_LIST.init.expect("non-null function pointer")(&raw mut (*g).contours);
    GLYF_I_REFERENCE_LIST.init.expect("non-null function pointer")(&raw mut (*g).references);
    GLYF_I_STEM_DEF_LIST.init.expect("non-null function pointer")(&raw mut (*g).stem_h);
    GLYF_I_STEM_DEF_LIST.init.expect("non-null function pointer")(&raw mut (*g).stem_v);
    GLYF_I_MASK_LIST.init.expect("non-null function pointer")(&raw mut (*g).hint_masks);
    GLYF_I_MASK_LIST.init.expect("non-null function pointer")(&raw mut (*g).contour_masks);
    (*g).instructions_length = 0 as u16;
    (*g).instructions = ::core::ptr::null_mut::<u8>();
    (*g).fd_select = otfcc_handle_empty() as FdHandle;
    (*g).y_pel = 0 as u8;
    (*g).stat.x_min = 0 as ::core::ffi::c_int as Pos;
    (*g).stat.x_max = 0 as ::core::ffi::c_int as Pos;
    (*g).stat.y_min = 0 as ::core::ffi::c_int as Pos;
    (*g).stat.y_max = 0 as ::core::ffi::c_int as Pos;
    (*g).stat.nest_depth = 0 as u16;
    (*g).stat.n_points = 0 as u16;
    (*g).stat.n_contours = 0 as u16;
    (*g).stat.n_composite_points = 0 as u16;
    (*g).stat.n_composite_contours = 0 as u16;
    return g;
}
unsafe extern "C" fn otfcc_delete_glyf_glyph(mut g: *mut Glyph) {
    if g.is_null() {
        return;
    }
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*g).horizontal_origin);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*g).advance_width);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*g).vertical_origin);
    I_VQ.dispose.expect("non-null function pointer")(&raw mut (*g).advance_height);
    sdsfree((*g).name);
    GLYF_I_CONTOUR_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).contours);
    GLYF_I_REFERENCE_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).references);
    GLYF_I_STEM_DEF_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).stem_h);
    GLYF_I_STEM_DEF_LIST
        .dispose
        .expect("non-null function pointer")(&raw mut (*g).stem_v);
    GLYF_I_MASK_LIST.dispose.expect("non-null function pointer")(&raw mut (*g).hint_masks);
    GLYF_I_MASK_LIST.dispose.expect("non-null function pointer")(&raw mut (*g).contour_masks);
    if !(*g).instructions.is_null() {
        free((*g).instructions as *mut ::core::ffi::c_void);
        (*g).instructions = ::core::ptr::null_mut::<u8>();
    }
    otfcc_handle_dispose(&raw mut (*g).fd_select);
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
        dispose: Some(dispose_glyf_ptr as unsafe extern "C" fn(*mut GlyphPtr) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_glyf_init_n(mut arr: *mut GlyfTable, mut n: usize) {
    table_glyf_init(arr);
    table_glyf_grow_to_n(arr, n);
    table_glyf_fill(arr, n);
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
unsafe extern "C" fn table_glyf_grow_to(arr: *mut GlyfTable, target: usize) {
    cvec_grow_to(table_glyf_as_cvec(arr), target);
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
        dispose: Some(table_glyf_dispose as unsafe extern "C" fn(*mut GlyfTable) -> ()),
        create: Some(table_glyf_create),
        free: Some(table_glyf_free as unsafe extern "C" fn(*mut GlyfTable) -> ()),
        create_n: Some(table_glyf_create_n as unsafe extern "C" fn(usize) -> *mut GlyfTable),
        push: Some(table_glyf_push as unsafe extern "C" fn(*mut GlyfTable, GlyphPtr) -> ()),
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
                json_new_vq((*(*c).items.offset(m as isize)).x.clone(), (*ctx).fvar),
            );
            json_object_push(
                point,
                b"y\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_vq((*(*c).items.offset(m as isize)).y.clone(), (*ctx).fvar),
            );
            json_object_push(
                point,
                b"on\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(
                    ((*(*c).items.offset(m as isize)).on_curve & MASK_ON_CURVE)
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
            json_new_vq((*r).x.clone(), (*ctx).fvar),
        );
        json_object_push(
            ref_0,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_vq((*r).y.clone(), (*ctx).fvar),
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
        if (*r).is_anchored != RefAnchorStatus::Xy
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
        if (*r).round_to_grid {
            json_object_push(
                ref_0,
                b"roundToGrid\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new(TRUE_0),
            );
        }
        if (*r).use_my_metrics {
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
            json_integer_new((*(*masks).items.offset(j as isize)).contours_before as i64),
        );
        json_object_push(
            mask,
            b"pointsBefore\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*(*masks).items.offset(j as isize)).points_before as i64),
        );
        let mut h: *mut JsonValue = json_array_new((*hh).length);
        let mut k: ShapeId = 0 as ShapeId;
        while (k as usize) < (*hh).length {
            json_array_push(
                h,
                json_boolean_new(
                    (*(*masks).items.offset(j as isize)).mask_h[k as usize] as ::core::ffi::c_int,
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
                    (*(*masks).items.offset(j as isize)).mask_v[k_0 as usize] as ::core::ffi::c_int,
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
        json_new_vq((*g).advance_width.clone(), (*ctx).fvar),
    );
    if I_VQ.is_still.expect("non-null function pointer")((*g).horizontal_origin.clone()) as ::core::ffi::c_int
        != 0
        && fabs(
            I_VQ.get_still.expect("non-null function pointer")((*g).horizontal_origin.clone())
                as ::core::ffi::c_double,
        ) > 1.0f64 / 1000.0f64
    {
        json_object_push(
            glyph,
            b"horizontalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_vq((*g).horizontal_origin.clone(), (*ctx).fvar),
        );
    }
    if (*ctx).has_vertical_metrics {
        json_object_push(
            glyph,
            b"advanceHeight\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_vq((*g).advance_height.clone(), (*ctx).fvar),
        );
        json_object_push(
            glyph,
            b"verticalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_vq((*g).vertical_origin.clone(), (*ctx).fvar),
        );
    }
    glyf_glyph_dump_contours(g, glyph, ctx);
    glyf_glyph_dump_references(g, glyph, ctx);
    if (*ctx).export_fd_select {
        json_object_push(
            glyph,
            b"CFF_fdSelect\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new((*g).fd_select.name as *const ::core::ffi::c_char),
        );
        json_object_push(
            glyph,
            b"CFF_CID\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*g).cid as i64),
        );
    }
    if !(*options).ignore_hints {
        if !(*g).instructions.is_null() && (*g).instructions_length as ::core::ffi::c_int != 0 {
            json_object_push(
                glyph,
                b"instructions\0" as *const u8 as *const ::core::ffi::c_char,
                dump_ttinstr(
                    (*g).instructions,
                    (*g).instructions_length as u32,
                    options,
                ),
            );
        }
        if (*g).stem_h.length != 0 {
            json_object_push(
                glyph,
                b"stemH\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_stemdefs(&raw mut (*g).stem_h)),
            );
        }
        if (*g).stem_v.length != 0 {
            json_object_push(
                glyph,
                b"stemV\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_stemdefs(&raw mut (*g).stem_v)),
            );
        }
        if (*g).hint_masks.length != 0 {
            json_object_push(
                glyph,
                b"hintMasks\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_maskdefs(
                    &raw mut (*g).hint_masks,
                    &raw mut (*g).stem_h,
                    &raw mut (*g).stem_v,
                )),
            );
        }
        if (*g).contour_masks.length != 0 {
            json_object_push(
                glyph,
                b"contourMasks\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_maskdefs(
                    &raw mut (*g).contour_masks,
                    &raw mut (*g).stem_h,
                    &raw mut (*g).stem_v,
                )),
            );
        }
        if (*g).y_pel != 0 {
            json_object_push(
                glyph,
                b"LTSH_yPel\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*g).y_pel as i64),
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
        .start_sds
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
            shift: Vec::new(),
        },
        y: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        on_curve: 0,
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
            point.on_curve = json_boolof(cv) as i8;
        }
        _k = _k.wrapping_add(1);
    }
    return point;
}
unsafe extern "C" fn glyf_parse_contours(mut col: *mut JsonValue, mut g: *mut Glyph) {
    if col.is_null() {
        return;
    }
    let mut n_contours: ShapeId = (*col).u.array.length as ShapeId;
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_int) < n_contours as ::core::ffi::c_int {
        let mut contourdump: *mut JsonValue =
            *(*col).u.array.values.offset(j as isize) as *mut JsonValue;
        let mut contour: Contour = Contour {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut::<Point>(),
        };
        GLYF_I_CONTOUR.init_cap_n.expect("non-null function pointer")(
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
        ref_0.round_to_grid = json_obj_getbool(
            refdump,
            b"roundToGrid\0" as *const u8 as *const ::core::ffi::c_char,
        );
        ref_0.use_my_metrics = json_obj_getbool(
            refdump,
            b"useMyMetrics\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if json_obj_getbool(
            refdump,
            b"isAnchored\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            ref_0.is_anchored = RefAnchorStatus::AnchorXy;
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
            I_VQ.create_still.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos)
                as VQ,
        );
        I_VQ.replace.expect("non-null function pointer")(
            &raw mut ref_0.y,
            I_VQ.create_still.expect("non-null function pointer")(0 as ::core::ffi::c_int as Pos)
                as VQ,
        );
        ref_0.a = 1.0f64 as Scale;
        ref_0.b = 0.0f64 as Scale;
        ref_0.c = 0.0f64 as Scale;
        ref_0.d = 1.0f64 as Scale;
        ref_0.round_to_grid = false;
        ref_0.use_my_metrics = false;
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
    (*g).instructions_length = len as u16;
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
                points_before: 0,
                contours_before: 0,
                mask_h: [false; 256],
                mask_v: [false; 256],
            };
            mask.points_before = json_obj_getint(
                m,
                b"pointsBefore\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            mask.contours_before = json_obj_getint(
                m,
                b"contoursBefore\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            parse_maskbits(
                (&raw mut mask.mask_h as *mut bool).offset(0 as ::core::ffi::c_int as isize)
                    as *mut bool,
                json_obj_get_type(
                    m,
                    b"maskH\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                ),
            );
            parse_maskbits(
                (&raw mut mask.mask_v as *mut bool).offset(0 as ::core::ffi::c_int as isize)
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
        &raw mut (*g).advance_width,
        json_vq_of(
            json_obj_get(
                glyphdump,
                b"advanceWidth\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    I_VQ.replace.expect("non-null function pointer")(
        &raw mut (*g).horizontal_origin,
        json_vq_of(
            json_obj_get(
                glyphdump,
                b"horizontalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    I_VQ.replace.expect("non-null function pointer")(
        &raw mut (*g).advance_height,
        json_vq_of(
            json_obj_get(
                glyphdump,
                b"advanceHeight\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    I_VQ.replace.expect("non-null function pointer")(
        &raw mut (*g).vertical_origin,
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
            &raw mut (*g).stem_h,
        );
        parse_stems(
            json_obj_get_type(
                glyphdump,
                b"stemV\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            ),
            &raw mut (*g).stem_v,
        );
        parse_masks(
            json_obj_get_type(
                glyphdump,
                b"hintMasks\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            ),
            &raw mut (*g).hint_masks,
        );
        parse_masks(
            json_obj_get_type(
                glyphdump,
                b"contourMasks\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            ),
            &raw mut (*g).contour_masks,
        );
        (*g).y_pel = json_obj_getint(
            glyphdump,
            b"LTSH_yPel\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u8;
    }
    (*g).fd_select = handle_from_name(json_obj_getsds(
        glyphdump,
        b"CFF_fdSelect\0" as *const u8 as *const ::core::ffi::c_char,
    )) as FdHandle;
    if (*g).y_pel == 0 {
        (*g).y_pel = json_obj_getint(
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
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"glyf"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut num_glyphs: GlyphId = (*table).u.object.length as GlyphId;
            glyf = TABLE_I_GLYF.create_n.expect("non-null function pointer")(num_glyphs as usize);
            let mut j: GlyphId = 0 as GlyphId;
            while (j as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
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
                if !(*glyph_order).by_name.is_null() {
                    let mut _hf_bkt: ::core::ffi::c_uint = 0;
                    _hf_bkt = _hf_hashv
                        & (*(*(*glyph_order).by_name).hh_name.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if !(*(*(*(*glyph_order).by_name).hh_name.tbl)
                            .buckets
                            .offset(_hf_bkt as isize))
                        .hh_head
                        .is_null()
                        {
                            order_entry = ((*(*(*(*glyph_order).by_name).hh_name.tbl)
                                .buckets
                                .offset(_hf_bkt as isize))
                            .hh_head
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(*(*glyph_order).by_name).hh_name.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut GlyphOrderEntry
                                as *mut GlyphOrderEntry;
                        } else {
                            order_entry = ::core::ptr::null_mut::<GlyphOrderEntry>();
                        }
                        while !order_entry.is_null() {
                            if (*order_entry).hh_name.hashv == _hf_hashv
                                && (*order_entry).hh_name.keylen as usize == sdslen(gname)
                            {
                                if memcmp(
                                    (*order_entry).hh_name.key,
                                    gname as *const ::core::ffi::c_void,
                                    sdslen(gname),
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if !(*order_entry).hh_name.hh_next.is_null() {
                                order_entry = ((*order_entry).hh_name.hh_next
                                    as *mut ::core::ffi::c_char)
                                    .offset(-(*(*(*glyph_order).by_name).hh_name.tbl).hho)
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
    // `is_anchored == RefAnchorStatus::AnchorConsolidated`, so these values pick which branch
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
