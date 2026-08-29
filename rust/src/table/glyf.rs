#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod build;
pub mod read;

use libc::{fprintf, strcmp};
unsafe extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::logger::{logger_finish, logger_start_sds};
use crate::support::TRUE_0;
use crate::support::buffer::Buffer;
use crate::support::glyph_order::{GlyphOrder, GlyphOrderEntry};
use crate::support::handle::{
    FdHandle, GlyphHandle, Handle, HandleState, handle_from_name, otfcc_handle_empty,
};
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Pos, Scale, ShapeId};
use crate::support::stdio::stderr;
use crate::table::fvar::FvarTable;
use crate::vendor::json::JsonType;

use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_boolean_new, json_integer_new,
    json_new_position, json_object_new, json_object_push, json_object_push_bytes_key,
    json_string_new_from_bytes, preserialize,
};
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_bool_val, json_boolof, json_dbl_val, json_int_val,
    json_obj_get, json_obj_get_type, json_obj_getbool, json_obj_getint, json_obj_getnum,
    json_obj_getnum_fallback, json_obj_getsds, json_obj_key_at, json_obj_key_bytes_at,
    json_obj_len, json_obj_null_out_val_at, json_obj_val_at, json_str_bytes, json_type_of,
};
use crate::support::ttinstr::{dump_ttinstr, parse_ttinstr};
use crate::table::fvar::{json_new_vq, json_vq_of};
use crate::vf::vq::VQ;
use crate::vf::vq::{vq_copy, vq_create_still, vq_get_still, vq_is_still, vq_replace};

#[derive(Clone)]
pub struct Point {
    pub x: VQ,
    pub y: VQ,
    pub on_curve: i8,
}
/// A single outline contour, owned point-by-point. Plain `Vec<Point>`: every
/// point's `VQ` fields are themselves `Vec`s, so dropping a `Contour` already
/// recursively frees everything it owns -- no element embeds a `Handle`, so
/// unlike [`ReferenceList`] there is nothing that needs an explicit dispose
/// loop before a container of these is torn down.
pub type Contour = Vec<Point>;
pub type ContourList = Vec<Contour>;
#[derive(Copy, Clone)]
pub struct PostscriptStemDef {
    pub position: Pos,
    pub width: Pos,
    pub map: u16,
}
pub type StemDefList = Vec<PostscriptStemDef>;
#[derive(Copy, Clone)]
pub struct PostscriptHintMask {
    pub points_before: u16,
    pub contours_before: u16,
    pub mask_h: [bool; 256],
    pub mask_v: [bool; 256],
}
pub type MaskList = Vec<PostscriptHintMask>;
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
/// A glyph's component references. Each [`ComponentReference`] embeds two
/// `VQ`s and a `GlyphHandle`, all of which now own their allocations for
/// real and auto-drop, so -- like [`Contour`] -- dropping/clearing this
/// container needs no explicit per-element dispose pass.
pub type ReferenceList = Vec<ComponentReference>;
#[derive(Copy, Clone)]
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
pub struct Glyph {
    pub name: Vec<u8>,
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
    pub instructions: Vec<u8>,
    pub y_pel: u8,
    pub fd_select: FdHandle,
    pub cid: GlyphId,
    pub stat: GlyphStat,
}
/// `instructions` is now a plain `Vec<u8>` (Stage 7-2-c), same as `name` (a
/// `Vec<u8>` since the `sds` sweep reached that field) -- both tear down
/// for free, and everything else either has no allocation of its own
/// (`stat`, `cid`, ...) or is already a real Rust owner that auto-drops
/// correctly on its own: `horizontal_origin`/`advance_width`/
/// `vertical_origin`/`advance_height` (`VQ`, a plain `struct { kernel: Pos,
/// shift: Vec<VqSegment> }` with no `Drop` impl of its own -- `vq_dispose`
/// is just `shift = Vec::new()`, which is exactly what happens for free
/// when a `VQ` field is dropped), `contours`/`references`/`stem_h`/
/// `stem_v`/`hint_masks`/`contour_masks` (plain `Vec`s per the comments on
/// [`Contour`]/[`ReferenceList`] above), and `fd_select` (a `Handle`, which
/// has owned its `name` and had a real `Drop` impl since the crate-wide
/// `Handle` conversion -- calling `otfcc_handle_dispose` on it here too,
/// the way the old manual `otfcc_delete_glyf_glyph` did, would be
/// redundant with that, not wrong). No field needs manual teardown any
/// more, so the `Drop` impl that used to free `instructions` by hand is
/// gone -- `#[derive(Clone)]` above is also sound now: every field
/// (`instructions` included) is a real deep-copying Rust owner, so cloning
/// a `Glyph` wholesale no longer aliases a raw pointer between the
/// original and the copy the way the old `*mut u8` did.
pub type GlyphPtr = *mut Glyph;
/// The font's glyph table: an array of owned glyphs, indexed by GID. Unlike
/// the other three containers in this "owned pointer array" group
/// (`LangSystemList`/`FeatureList`/`LookupList`, all `Vec<Box<T>>`), a slot
/// here can be legitimately unset -- `table_glyf_create_n` pre-sizes the
/// table to the glyph count before parsing/extracting fills each GID in,
/// and `consolidate_glyf` patches any GID that never got filled with a
/// fresh empty glyph. `Box<Glyph>` cannot represent "no glyph here" (a
/// `Box` is never null), so the element type stays `Option<Box<Glyph>>`.
pub type GlyfTable = Vec<Option<Box<Glyph>>>;
#[derive(Copy, Clone)]
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
unsafe fn create_point(p: *mut Point) {
    (*p).x = vq_create_still(0_i32 as Pos);
    (*p).y = vq_create_still(0_i32 as Pos);
    (*p).on_curve = TRUE_0 as i8;
}
unsafe fn copy_point(dst: *mut Point, src: *const Point) {
    vq_copy(&raw mut (*dst).x, &raw const (*src).x);
    vq_copy(&raw mut (*dst).y, &raw const (*src).y);
    (*dst).on_curve = (*src).on_curve;
}
#[inline]
pub unsafe fn glyf_point_init(x: *mut Point) {
    create_point(x);
}
#[inline]
pub unsafe fn glyf_point_dup(src: Point) -> Point {
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
unsafe fn glyf_point_copy(dst: *mut Point, src: *const Point) {
    copy_point(dst, src);
}
/// Grows `arr` to `n` points, default-constructing each new one via
/// [`glyf_point_init`] -- also called directly from `libcff/charstring_il.rs`
/// and `table/cff.rs`, since the `PointElementInterface` vtable this used to
/// dispatch through is gone (single static, no real polymorphism).
#[inline]
unsafe fn glyf_contour_fill(arr: *mut Contour, n: usize) {
    while (*arr).len() < n {
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
        (*arr).push(x);
    }
}
#[inline]
unsafe fn init_glyf_reference(ref_0: *mut ComponentReference) {
    (*ref_0).glyph = otfcc_handle_empty() as GlyphHandle;
    (*ref_0).x = vq_create_still(0_i32 as Pos);
    (*ref_0).y = vq_create_still(0_i32 as Pos);
    (*ref_0).a = 1_i32 as Scale;
    (*ref_0).b = 0_i32 as Scale;
    (*ref_0).c = 0_i32 as Scale;
    (*ref_0).d = 1_i32 as Scale;
    (*ref_0).is_anchored = RefAnchorStatus::Xy;
    (*ref_0).outer = 0 as ShapeId;
    (*ref_0).inner = (*ref_0).outer;
    (*ref_0).round_to_grid = false;
    (*ref_0).use_my_metrics = false;
}
#[inline]
pub unsafe fn glyf_component_reference_empty() -> ComponentReference {
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
            name: Vec::new(),
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
pub unsafe fn glyf_component_reference_init(x: *mut ComponentReference) {
    init_glyf_reference(x);
}
/// `Box::new` is the allocation and the struct literal is the zero-init
/// `__caryll_allocate_clean` (calloc) used to provide -- same shape as
/// `new_lookup`/`new_feature`/`new_language`. Kept the `otfcc_`-prefixed C
/// name (unlike those three) since this one is still called from outside
/// this file (`consolidate.rs`, `table/cff.rs`, `table/glyf/read.rs`).
pub unsafe fn otfcc_new_glyf_glyph() -> Box<Glyph> {
    Box::new(Glyph {
        name: Vec::new(),
        horizontal_origin: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        advance_width: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        vertical_origin: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        advance_height: VQ {
            kernel: 0.,
            shift: Vec::new(),
        },
        contours: Vec::new(),
        references: Vec::new(),
        stem_h: Vec::new(),
        stem_v: Vec::new(),
        hint_masks: Vec::new(),
        contour_masks: Vec::new(),
        instructions: Vec::new(),
        y_pel: 0_u8,
        fd_select: otfcc_handle_empty() as FdHandle,
        cid: 0 as GlyphId,
        stat: GlyphStat {
            x_min: 0_i32 as Pos,
            x_max: 0_i32 as Pos,
            y_min: 0_i32 as Pos,
            y_max: 0_i32 as Pos,
            nest_depth: 0_u16,
            n_points: 0_u16,
            n_contours: 0_u16,
            n_composite_points: 0_u16,
            n_composite_contours: 0_u16,
        },
    })
}
// Stage 6-4 "Box化": `Font.glyf` becomes `Option<Vec<Option<Box<Glyph>>>>`
// (not `Option<Box<Vec<...>>>` -- `Vec` already owns its own heap buffer).
// `table_glyf_create_n` stays: `table/cff.rs`'s CFF glyph extraction still
// builds a `GlyfTable` through it as a bare `*mut GlyfTable`, so
// `unwrap_glyf_table` below "adopts" that raw pointer into a genuine owned
// value at the one point it actually needs to become `Font.glyf`. Since
// Stage 7-2-d, `table_glyf_create_n` builds via `Box::into_raw(Box::new(..))`
// rather than `malloc`, so `raw` already points at a real `Box<GlyfTable>`
// allocation -- `Box::from_raw` is the exact inverse (dereferencing moves
// the `Vec` value out and drops the now-empty `Box` shell in the same
// step), no separate `free` call needed. Same technique as
// `table/cff.rs`'s `unwrap_cff_table`.
pub(crate) unsafe fn unwrap_glyf_table(raw: *mut GlyfTable) -> Option<GlyfTable> {
    if raw.is_null() {
        return None;
    }
    Some(*Box::from_raw(raw))
}
pub(crate) unsafe fn table_glyf_create_n(n: usize) -> *mut GlyfTable {
    let mut v: GlyfTable = Vec::with_capacity(n);
    v.resize_with(n, || None);
    Box::into_raw(Box::new(v))
}
unsafe fn glyf_glyph_dump_contours(
    g: *const Glyph,
    target: *mut BuiltValue,
    ctx: *const GlyfIOContext,
) {
    if (*g).contours.is_empty() {
        return;
    }
    let contours: *mut BuiltValue = json_array_new((*g).contours.len());
    let mut k: ShapeId = 0 as ShapeId;
    while (k as usize) < (*g).contours.len() {
        let c: &Contour = &(&(*g).contours)[k as usize];
        let contour: *mut BuiltValue = json_array_new(c.len());
        let mut m: ShapeId = 0 as ShapeId;
        while (m as usize) < c.len() {
            let point: *mut BuiltValue = json_object_new(4_usize);
            json_object_push(
                point,
                b"x\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_vq(c[m as usize].x.clone(), (*ctx).fvar),
            );
            json_object_push(
                point,
                b"y\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_vq(c[m as usize].y.clone(), (*ctx).fvar),
            );
            json_object_push(
                point,
                b"on\0" as *const u8 as *const ::core::ffi::c_char,
                json_boolean_new((c[m as usize].on_curve & MASK_ON_CURVE) as i32),
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
unsafe fn glyf_glyph_dump_references(
    g: *const Glyph,
    target: *mut BuiltValue,
    ctx: *const GlyfIOContext,
) {
    if (*g).references.is_empty() {
        return;
    }
    let references: *mut BuiltValue = json_array_new((*g).references.len());
    let mut k: ShapeId = 0 as ShapeId;
    while (k as usize) < (*g).references.len() {
        let r: *const ComponentReference = &raw const (&(*g).references)[k as usize];
        let ref_0: *mut BuiltValue = json_object_new(9_usize);
        json_object_push(
            ref_0,
            b"glyph\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new_from_bytes(&(*r).glyph.name),
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
        if (*r).is_anchored != RefAnchorStatus::Xy {
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
unsafe fn glyf_glyph_dump_stemdefs(stems: *const StemDefList) -> *mut BuiltValue {
    let stems: &Vec<PostscriptStemDef> = &*stems;
    let a: *mut BuiltValue = json_array_new(stems.len());
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < stems.len() {
        let stem: *mut BuiltValue = json_object_new(3_usize);
        json_object_push(
            stem,
            b"position\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(stems[j as usize].position),
        );
        json_object_push(
            stem,
            b"width\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(stems[j as usize].width),
        );
        json_array_push(a, stem);
        j = j.wrapping_add(1);
    }
    return a;
}
unsafe fn glyf_glyph_dump_maskdefs(
    masks: *const MaskList,
    hh: *const StemDefList,
    vv: *const StemDefList,
) -> *mut BuiltValue {
    let masks: &Vec<PostscriptHintMask> = &*masks;
    let hh: &Vec<PostscriptStemDef> = &*hh;
    let vv: &Vec<PostscriptStemDef> = &*vv;
    let a: *mut BuiltValue = json_array_new(masks.len());
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < masks.len() {
        let mask: *mut BuiltValue = json_object_new(3_usize);
        json_object_push(
            mask,
            b"contoursBefore\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new(masks[j as usize].contours_before as i64),
        );
        json_object_push(
            mask,
            b"pointsBefore\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new(masks[j as usize].points_before as i64),
        );
        let h: *mut BuiltValue = json_array_new(hh.len());
        let mut k: ShapeId = 0 as ShapeId;
        while (k as usize) < hh.len() {
            json_array_push(
                h,
                json_boolean_new(masks[j as usize].mask_h[k as usize] as i32),
            );
            k = k.wrapping_add(1);
        }
        json_object_push(
            mask,
            b"maskH\0" as *const u8 as *const ::core::ffi::c_char,
            h,
        );
        let v: *mut BuiltValue = json_array_new(vv.len());
        let mut k_0: ShapeId = 0 as ShapeId;
        while (k_0 as usize) < vv.len() {
            json_array_push(
                v,
                json_boolean_new(masks[j as usize].mask_v[k_0 as usize] as i32),
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
unsafe fn glyf_dump_glyph(
    g: *const Glyph,
    options: &Options,
    ctx: *const GlyfIOContext,
) -> *mut BuiltValue {
    let glyph: *mut BuiltValue = json_object_new(12_usize);
    json_object_push(
        glyph,
        b"advanceWidth\0" as *const u8 as *const ::core::ffi::c_char,
        json_new_vq((*g).advance_width.clone(), (*ctx).fvar),
    );
    if vq_is_still((*g).horizontal_origin.clone()) as i32 != 0
        && fabs(vq_get_still((*g).horizontal_origin.clone()) as ::core::ffi::c_double)
            > 1.0f64 / 1000.0f64
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
            json_string_new_from_bytes(&(*g).fd_select.name),
        );
        json_object_push(
            glyph,
            b"CFF_CID\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*g).cid as i64),
        );
    }
    if !options.ignore_hints {
        if !(*g).instructions.is_empty() {
            json_object_push(
                glyph,
                b"instructions\0" as *const u8 as *const ::core::ffi::c_char,
                dump_ttinstr(
                    (*g).instructions.as_ptr() as *mut u8,
                    (*g).instructions.len() as u32,
                    options,
                ),
            );
        }
        if !(*g).stem_h.is_empty() {
            json_object_push(
                glyph,
                b"stemH\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_stemdefs(&raw const (*g).stem_h)),
            );
        }
        if !(*g).stem_v.is_empty() {
            json_object_push(
                glyph,
                b"stemV\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_stemdefs(&raw const (*g).stem_v)),
            );
        }
        if !(*g).hint_masks.is_empty() {
            json_object_push(
                glyph,
                b"hintMasks\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_maskdefs(
                    &raw const (*g).hint_masks,
                    &raw const (*g).stem_h,
                    &raw const (*g).stem_v,
                )),
            );
        }
        if !(*g).contour_masks.is_empty() {
            json_object_push(
                glyph,
                b"contourMasks\0" as *const u8 as *const ::core::ffi::c_char,
                preserialize(glyf_glyph_dump_maskdefs(
                    &raw const (*g).contour_masks,
                    &raw const (*g).stem_h,
                    &raw const (*g).stem_v,
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
pub unsafe fn otfcc_dump_glyphorder(table: *const GlyfTable, root: *mut BuiltValue) {
    if table.is_null() {
        return;
    }
    let order: *mut BuiltValue = json_array_new((*table).len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*table).len() {
        let g: *const Glyph = (&(*table))[j as usize].as_deref().unwrap() as *const Glyph;
        json_array_push(order, json_string_new_from_bytes(&(*g).name));
        j = j.wrapping_add(1);
    }
    json_object_push(
        root,
        b"glyph_order\0" as *const u8 as *const ::core::ffi::c_char,
        preserialize(order),
    );
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_glyf(
    table: Option<&GlyfTable>,
    root: *mut BuiltValue,
    options: &Options,
    ctx: *const GlyfIOContext,
) {
    let table = match table {
        Some(t) => t as *const GlyfTable,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"glyf"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let glyf: *mut BuiltValue = json_object_new((*table).len());
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < (*table).len() {
            let g: *const Glyph = (&(*table))[j as usize].as_deref().unwrap() as *const Glyph;
            json_object_push_bytes_key(glyf, &(*g).name, glyf_dump_glyph(g, options, ctx));
            j = j.wrapping_add(1);
        }
        json_object_push(
            root,
            b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
            glyf,
        );
        if !options.ignore_glyph_order {
            otfcc_dump_glyphorder(table, root);
        }
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
unsafe fn glyf_parse_point(pointdump: *const ParsedValue) -> Point {
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
    glyf_point_init(&raw mut point);
    if pointdump.is_null() || json_type_of(pointdump) != JsonType::Object {
        return point;
    }
    let mut _k: u32 = 0_u32;
    while _k < json_obj_len(pointdump) {
        let ck: *mut ::core::ffi::c_char = json_obj_key_at(pointdump, _k);
        let cv: *const ParsedValue = json_obj_val_at(pointdump, _k);
        if strcmp(ck, b"x\0" as *const u8 as *const ::core::ffi::c_char) == 0_i32
        {
            vq_replace(
                &raw mut point.x,
                json_vq_of(cv, ::core::ptr::null::<FvarTable>()) as VQ,
            );
        } else if strcmp(ck, b"y\0" as *const u8 as *const ::core::ffi::c_char)
            == 0_i32
        {
            vq_replace(
                &raw mut point.y,
                json_vq_of(cv, ::core::ptr::null::<FvarTable>()) as VQ,
            );
        } else if strcmp(ck, b"on\0" as *const u8 as *const ::core::ffi::c_char)
            == 0_i32
        {
            point.on_curve = json_boolof(cv) as i8;
        }
        _k = _k.wrapping_add(1);
    }
    return point;
}
unsafe fn glyf_parse_contours(col: *const ParsedValue, g: *mut Glyph) {
    if col.is_null() {
        return;
    }
    let n_contours: ShapeId = json_arr_len(col) as ShapeId;
    let mut j: ShapeId = 0 as ShapeId;
    while (j as i32) < n_contours as i32 {
        let contourdump: *const ParsedValue = json_arr_at(col, j as u32);
        let mut contour: Contour = Vec::with_capacity(
            (if !contourdump.is_null() && json_type_of(contourdump) == JsonType::Array {
                json_arr_len(contourdump)
            } else {
                1 as ::core::ffi::c_uint
            }) as usize,
        );
        if !contourdump.is_null() && json_type_of(contourdump) == JsonType::Array {
            let mut k: ShapeId = 0 as ShapeId;
            while (k as ::core::ffi::c_uint) < json_arr_len(contourdump) {
                contour.push(glyf_parse_point(json_arr_at(contourdump, k as u32)));
                k = k.wrapping_add(1);
            }
        }
        (*g).contours.push(contour);
        j = j.wrapping_add(1);
    }
}
unsafe fn glyf_parse_reference(refdump: *const ParsedValue) -> ComponentReference {
    let mut _gname: *const ParsedValue = json_obj_get_type(
        refdump,
        b"glyph\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::String,
    );
    let mut ref_0: ComponentReference = (glyf_component_reference_empty)();
    if !_gname.is_null() {
        ref_0.glyph = handle_from_name(Some(json_str_bytes(_gname))) as GlyphHandle;
        vq_replace(
            &raw mut ref_0.x,
            json_vq_of(
                json_obj_get(refdump, b"x\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<FvarTable>(),
            ) as VQ,
        );
        vq_replace(
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
        ref_0.glyph.name = Vec::new();
        vq_replace(
            &raw mut ref_0.x,
            vq_create_still(0_i32 as Pos) as VQ,
        );
        vq_replace(
            &raw mut ref_0.y,
            vq_create_still(0_i32 as Pos) as VQ,
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
unsafe fn glyf_parse_references(col: *const ParsedValue, g: *mut Glyph) {
    if col.is_null() {
        return;
    }
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_uint) < json_arr_len(col) {
        (*g).references
            .push(glyf_parse_reference(json_arr_at(col, j as u32)));
        j = j.wrapping_add(1);
    }
}
unsafe fn make_instrs_for_glyph(mut _g: *mut ::core::ffi::c_void, instrs: Vec<u8>) {
    let g: *mut Glyph = _g as *mut Glyph;
    (*g).instructions = instrs;
}
unsafe fn wrong_instrs_for_glyph(
    mut _g: *mut ::core::ffi::c_void,
    reason: *mut ::core::ffi::c_char,
    pos: i32,
) {
    let g: *mut Glyph = _g as *mut Glyph;
    // `fprintf`'s `%s` needs a NUL-terminated buffer, so a NUL is appended
    // to a byte-copy of `name` here -- this is a diagnostic-only print to
    // stderr (never part of dumped/built output), so it doesn't need the
    // NUL-truncation care the crate's other `Handle`/glyph-name-to-JSON
    // sites take.
    let mut name_cstr: Vec<u8> = (*g).name.clone();
    name_cstr.push(0);
    fprintf(
        stderr,
        b"[OTFCC] TrueType instructions parse error : %s, at %d in /%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        reason,
        pos,
        name_cstr.as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe fn parse_stems(sd: *const ParsedValue, stems: *mut StemDefList) {
    if sd.is_null() {
        return;
    }
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_uint) < json_arr_len(sd) {
        let s: *const ParsedValue = json_arr_at(sd, j as u32);
        if !(json_type_of(s) != JsonType::Object) {
            let mut sdef: PostscriptStemDef = PostscriptStemDef {
                position: 0.,
                width: 0.,
                map: 0,
            };
            sdef.map = 0_u16;
            sdef.position =
                json_obj_getnum(s, b"position\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
            sdef.width =
                json_obj_getnum(s, b"width\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
            (*stems).push(sdef);
        }
        j = j.wrapping_add(1);
    }
}
unsafe fn parse_maskbits(arr: *mut bool, bits: *const ParsedValue) {
    if bits.is_null() {
        let mut j: ShapeId = 0 as ShapeId;
        while (j as i32) < 0x100_i32 {
            *arr.offset(j as isize) = false;
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: ShapeId = 0 as ShapeId;
        while (j_0 as i32) < 0x100_i32
            && (j_0 as ::core::ffi::c_uint) < json_arr_len(bits)
        {
            let b: *const ParsedValue = json_arr_at(bits, j_0 as u32);
            match json_type_of(b) as ::core::ffi::c_uint {
                6 => {
                    *arr.offset(j_0 as isize) = json_bool_val(b);
                }
                3 => {
                    *arr.offset(j_0 as isize) = json_int_val(b) != 0;
                }
                4 => {
                    *arr.offset(j_0 as isize) = json_dbl_val(b) != 0.;
                }
                _ => {
                    *arr.offset(j_0 as isize) = false;
                }
            }
            j_0 = j_0.wrapping_add(1);
        }
    };
}
unsafe fn parse_masks(md: *const ParsedValue, masks: *mut MaskList) {
    if md.is_null() {
        return;
    }
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_uint) < json_arr_len(md) {
        let m: *const ParsedValue = json_arr_at(md, j as u32);
        if !(json_type_of(m) != JsonType::Object) {
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
                (&raw mut mask.mask_h as *mut bool).offset(0_i32 as isize),
                json_obj_get_type(
                    m,
                    b"maskH\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                ),
            );
            parse_maskbits(
                (&raw mut mask.mask_v as *mut bool).offset(0_i32 as isize),
                json_obj_get_type(
                    m,
                    b"maskV\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                ),
            );
            (*masks).push(mask);
        }
        j = j.wrapping_add(1);
    }
}
unsafe fn otfcc_glyf_parse_glyph(
    glyphdump: *const ParsedValue,
    order_entry: &GlyphOrderEntry,
    options: &Options,
) -> Box<Glyph> {
    let mut g: Box<Glyph> = otfcc_new_glyf_glyph();
    (*g).name = order_entry.name.clone();
    vq_replace(
        &raw mut (*g).advance_width,
        json_vq_of(
            json_obj_get(
                glyphdump,
                b"advanceWidth\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    vq_replace(
        &raw mut (*g).horizontal_origin,
        json_vq_of(
            json_obj_get(
                glyphdump,
                b"horizontalOrigin\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    vq_replace(
        &raw mut (*g).advance_height,
        json_vq_of(
            json_obj_get(
                glyphdump,
                b"advanceHeight\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    vq_replace(
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
        &raw mut *g,
    );
    glyf_parse_references(
        json_obj_get_type(
            glyphdump,
            b"references\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        ),
        &raw mut *g,
    );
    if !options.ignore_hints {
        parse_ttinstr(
            json_obj_get(
                glyphdump,
                b"instructions\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            (&raw mut *g) as *mut ::core::ffi::c_void,
            Some(make_instrs_for_glyph as unsafe fn(*mut ::core::ffi::c_void, Vec<u8>) -> ()),
            Some(
                wrong_instrs_for_glyph
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_char,
                        i32,
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
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_parse_glyf(
    root: *const ParsedValue,
    glyph_order: *mut GlyphOrder,
    options: &Options,
) -> Option<GlyfTable> {
    if json_type_of(root) != JsonType::Object || glyph_order.is_null() {
        return None;
    }
    let mut glyf: Option<GlyfTable> = None;
    let table: *const ParsedValue;
    table = json_obj_get_type(
        root,
        b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"glyf"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let num_glyphs: GlyphId = json_obj_len(table) as GlyphId;
            let mut glyf_val: GlyfTable = Vec::with_capacity(num_glyphs as usize);
            glyf_val.resize_with(num_glyphs as usize, || None);
            let mut j: GlyphId = 0 as GlyphId;
            while (j as i32) < num_glyphs as i32 {
                let name_bytes: Vec<u8> = json_obj_key_bytes_at(table, j as u32);
                let glyphdump: *const ParsedValue = json_obj_val_at(table, j as u32);
                let order_idx: Option<usize> = (*glyph_order).by_name.get(&name_bytes).copied();
                if json_type_of(glyphdump) == JsonType::Object {
                    if let Some(idx) = order_idx {
                        let order_entry = &(&(*glyph_order).entries)[idx];
                        if glyf_val[order_entry.gid as usize].is_none() {
                            glyf_val[order_entry.gid as usize] =
                                Some(otfcc_glyf_parse_glyph(glyphdump, order_entry, options));
                        }
                    }
                }
                json_obj_null_out_val_at(table as *mut ParsedValue, j as u32);
                j = j.wrapping_add(1);
            }
            glyf = Some(glyf_val);
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
        return glyf;
    }
    return None;
}

#[derive(Copy, Clone)]
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
