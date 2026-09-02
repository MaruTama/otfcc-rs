#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod build;
pub mod read;

use libc::fprintf;
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

use crate::support::built_json::BuiltValue;
use crate::support::parsed_json::{ParsedValue, json_type_of};
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
unsafe fn glyf_glyph_dump_contours(g: *const Glyph, target: &mut BuiltValue, ctx: *const GlyfIOContext) {
    if (*g).contours.is_empty() {
        return;
    }
    let mut contours = BuiltValue::new_array((*g).contours.len());
    let mut k: ShapeId = 0 as ShapeId;
    while (k as usize) < (*g).contours.len() {
        let c: &Contour = &(&(*g).contours)[k as usize];
        let mut contour = BuiltValue::new_array(c.len());
        let mut m: ShapeId = 0 as ShapeId;
        while (m as usize) < c.len() {
            let mut point = BuiltValue::new_object(4);
            point.push_field(b"x", json_new_vq(c[m as usize].x.clone(), (*ctx).fvar));
            point.push_field(b"y", json_new_vq(c[m as usize].y.clone(), (*ctx).fvar));
            point.push_field(
                b"on",
                BuiltValue::Bool(c[m as usize].on_curve & MASK_ON_CURVE != 0),
            );
            contour.push_item(point);
            m = m.wrapping_add(1);
        }
        contours.push_item(contour.preserialize());
        k = k.wrapping_add(1);
    }
    target.push_field(b"contours", contours);
}
unsafe fn glyf_glyph_dump_references(
    g: *const Glyph,
    target: &mut BuiltValue,
    ctx: *const GlyfIOContext,
) {
    if (*g).references.is_empty() {
        return;
    }
    let mut references = BuiltValue::new_array((*g).references.len());
    let mut k: ShapeId = 0 as ShapeId;
    while (k as usize) < (*g).references.len() {
        let r: *const ComponentReference = &raw const (&(*g).references)[k as usize];
        let mut ref_0 = BuiltValue::new_object(9);
        ref_0.push_field(b"glyph", BuiltValue::str_truncated_at_nul(&(*r).glyph.name));
        ref_0.push_field(b"x", json_new_vq((*r).x.clone(), (*ctx).fvar));
        ref_0.push_field(b"y", json_new_vq((*r).y.clone(), (*ctx).fvar));
        ref_0.push_field(b"a", BuiltValue::position((*r).a as Pos));
        ref_0.push_field(b"b", BuiltValue::position((*r).b as Pos));
        ref_0.push_field(b"c", BuiltValue::position((*r).c as Pos));
        ref_0.push_field(b"d", BuiltValue::position((*r).d as Pos));
        if (*r).is_anchored != RefAnchorStatus::Xy {
            ref_0.push_field(b"isAnchored", BuiltValue::Bool(true));
            ref_0.push_field(b"inner", BuiltValue::Int((*r).inner as i64));
            ref_0.push_field(b"outer", BuiltValue::Int((*r).outer as i64));
        }
        if (*r).round_to_grid {
            ref_0.push_field(b"roundToGrid", BuiltValue::Bool(true));
        }
        if (*r).use_my_metrics {
            ref_0.push_field(b"useMyMetrics", BuiltValue::Bool(true));
        }
        references.push_item(ref_0.preserialize());
        k = k.wrapping_add(1);
    }
    target.push_field(b"references", references);
}
unsafe fn glyf_glyph_dump_stemdefs(stems: *const StemDefList) -> BuiltValue {
    let stems: &Vec<PostscriptStemDef> = &*stems;
    let mut a = BuiltValue::new_array(stems.len());
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < stems.len() {
        let mut stem = BuiltValue::new_object(3);
        stem.push_field(b"position", BuiltValue::position(stems[j as usize].position));
        stem.push_field(b"width", BuiltValue::position(stems[j as usize].width));
        a.push_item(stem);
        j = j.wrapping_add(1);
    }
    a
}
unsafe fn glyf_glyph_dump_maskdefs(
    masks: *const MaskList,
    hh: *const StemDefList,
    vv: *const StemDefList,
) -> BuiltValue {
    let masks: &Vec<PostscriptHintMask> = &*masks;
    let hh: &Vec<PostscriptStemDef> = &*hh;
    let vv: &Vec<PostscriptStemDef> = &*vv;
    let mut a = BuiltValue::new_array(masks.len());
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < masks.len() {
        let mut mask = BuiltValue::new_object(3);
        mask.push_field(
            b"contoursBefore",
            BuiltValue::Int(masks[j as usize].contours_before as i64),
        );
        mask.push_field(
            b"pointsBefore",
            BuiltValue::Int(masks[j as usize].points_before as i64),
        );
        let mut h = BuiltValue::new_array(hh.len());
        let mut k: ShapeId = 0 as ShapeId;
        while (k as usize) < hh.len() {
            h.push_item(BuiltValue::Bool(masks[j as usize].mask_h[k as usize]));
            k = k.wrapping_add(1);
        }
        mask.push_field(b"maskH", h);
        let mut v = BuiltValue::new_array(vv.len());
        let mut k_0: ShapeId = 0 as ShapeId;
        while (k_0 as usize) < vv.len() {
            v.push_item(BuiltValue::Bool(masks[j as usize].mask_v[k_0 as usize]));
            k_0 = k_0.wrapping_add(1);
        }
        mask.push_field(b"maskV", v);
        a.push_item(mask);
        j = j.wrapping_add(1);
    }
    a
}
unsafe fn glyf_dump_glyph(g: *const Glyph, options: &Options, ctx: *const GlyfIOContext) -> BuiltValue {
    let mut glyph = BuiltValue::new_object(12);
    glyph.push_field(
        b"advanceWidth",
        json_new_vq((*g).advance_width.clone(), (*ctx).fvar),
    );
    if vq_is_still((*g).horizontal_origin.clone()) as i32 != 0
        && fabs(vq_get_still((*g).horizontal_origin.clone()) as ::core::ffi::c_double)
            > 1.0f64 / 1000.0f64
    {
        glyph.push_field(
            b"horizontalOrigin",
            json_new_vq((*g).horizontal_origin.clone(), (*ctx).fvar),
        );
    }
    if (*ctx).has_vertical_metrics {
        glyph.push_field(
            b"advanceHeight",
            json_new_vq((*g).advance_height.clone(), (*ctx).fvar),
        );
        glyph.push_field(
            b"verticalOrigin",
            json_new_vq((*g).vertical_origin.clone(), (*ctx).fvar),
        );
    }
    glyf_glyph_dump_contours(g, &mut glyph, ctx);
    glyf_glyph_dump_references(g, &mut glyph, ctx);
    if (*ctx).export_fd_select {
        glyph.push_field(
            b"CFF_fdSelect",
            BuiltValue::str_truncated_at_nul(&(*g).fd_select.name),
        );
        glyph.push_field(b"CFF_CID", BuiltValue::Int((*g).cid as i64));
    }
    if !options.ignore_hints {
        if !(*g).instructions.is_empty() {
            glyph.push_field(
                b"instructions",
                dump_ttinstr(
                    (*g).instructions.as_ptr() as *mut u8,
                    (*g).instructions.len() as u32,
                    options,
                ),
            );
        }
        if !(*g).stem_h.is_empty() {
            glyph.push_field(
                b"stemH",
                glyf_glyph_dump_stemdefs(&raw const (*g).stem_h).preserialize(),
            );
        }
        if !(*g).stem_v.is_empty() {
            glyph.push_field(
                b"stemV",
                glyf_glyph_dump_stemdefs(&raw const (*g).stem_v).preserialize(),
            );
        }
        if !(*g).hint_masks.is_empty() {
            glyph.push_field(
                b"hintMasks",
                glyf_glyph_dump_maskdefs(
                    &raw const (*g).hint_masks,
                    &raw const (*g).stem_h,
                    &raw const (*g).stem_v,
                )
                .preserialize(),
            );
        }
        if !(*g).contour_masks.is_empty() {
            glyph.push_field(
                b"contourMasks",
                glyf_glyph_dump_maskdefs(
                    &raw const (*g).contour_masks,
                    &raw const (*g).stem_h,
                    &raw const (*g).stem_v,
                )
                .preserialize(),
            );
        }
        if (*g).y_pel != 0 {
            glyph.push_field(b"LTSH_yPel", BuiltValue::Int((*g).y_pel as i64));
        }
    }
    glyph
}
pub unsafe fn otfcc_dump_glyphorder(table: *const GlyfTable, root: &mut BuiltValue) {
    if table.is_null() {
        return;
    }
    let mut order = BuiltValue::new_array((*table).len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*table).len() {
        let g: *const Glyph = (&(*table))[j as usize].as_deref().unwrap() as *const Glyph;
        order.push_item(BuiltValue::str_truncated_at_nul(&(*g).name));
        j = j.wrapping_add(1);
    }
    root.push_field(b"glyph_order", order.preserialize());
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_glyf(
    table: Option<&GlyfTable>,
    root: &mut BuiltValue,
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
        let mut glyf = BuiltValue::new_object((*table).len());
        let mut j: GlyphId = 0 as GlyphId;
        while (j as usize) < (*table).len() {
            let g: *const Glyph = (&(*table))[j as usize].as_deref().unwrap() as *const Glyph;
            glyf.push_field_bytes_key(&(*g).name, glyf_dump_glyph(g, options, ctx));
            j = j.wrapping_add(1);
        }
        root.push_field(b"glyf", glyf);
        if !options.ignore_glyph_order {
            otfcc_dump_glyphorder(table, root);
        }
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
unsafe fn glyf_parse_point(pointdump: &ParsedValue) -> Point {
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
    let Some(fields) = pointdump.as_object() else {
        return point;
    };
    for (key, val) in fields {
        match &key[..key.len() - 1] {
            b"x" => vq_replace(
                &raw mut point.x,
                json_vq_of(val as *const ParsedValue, ::core::ptr::null::<FvarTable>()) as VQ,
            ),
            b"y" => vq_replace(
                &raw mut point.y,
                json_vq_of(val as *const ParsedValue, ::core::ptr::null::<FvarTable>()) as VQ,
            ),
            b"on" => point.on_curve = val.as_bool().unwrap_or(false) as i8,
            _ => {}
        }
    }
    point
}
unsafe fn glyf_parse_contours(col: Option<&ParsedValue>, g: *mut Glyph) {
    let Some(items) = col.and_then(ParsedValue::as_array) else {
        return;
    };
    for contourdump in items {
        let mut contour: Contour = Vec::with_capacity(contourdump.as_array().map_or(1, |a| a.len()));
        if let Some(points) = contourdump.as_array() {
            for pointdump in points {
                contour.push(glyf_parse_point(pointdump));
            }
        }
        (*g).contours.push(contour);
    }
}
unsafe fn glyf_parse_reference(refdump: &ParsedValue) -> ComponentReference {
    let mut ref_0: ComponentReference = (glyf_component_reference_empty)();
    let Some(_gname) = refdump.get_typed(b"glyph", JsonType::String) else {
        ref_0.glyph.name = Vec::new();
        vq_replace(&raw mut ref_0.x, vq_create_still(0_i32 as Pos) as VQ);
        vq_replace(&raw mut ref_0.y, vq_create_still(0_i32 as Pos) as VQ);
        ref_0.a = 1.0f64 as Scale;
        ref_0.b = 0.0f64 as Scale;
        ref_0.c = 0.0f64 as Scale;
        ref_0.d = 1.0f64 as Scale;
        ref_0.round_to_grid = false;
        ref_0.use_my_metrics = false;
        return ref_0;
    };
    ref_0.glyph = handle_from_name(_gname.as_str_bytes().map(|b| b.to_vec())) as GlyphHandle;
    vq_replace(
        &raw mut ref_0.x,
        json_vq_of(
            refdump.get(b"x").map_or(::core::ptr::null(), |v| v as *const ParsedValue),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    vq_replace(
        &raw mut ref_0.y,
        json_vq_of(
            refdump.get(b"y").map_or(::core::ptr::null(), |v| v as *const ParsedValue),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    ref_0.a = refdump.get_num_or(b"a", 1.0f64) as Scale;
    ref_0.b = refdump.get_num_or(b"b", 0.0f64) as Scale;
    ref_0.c = refdump.get_num_or(b"c", 0.0f64) as Scale;
    ref_0.d = refdump.get_num_or(b"d", 1.0f64) as Scale;
    ref_0.round_to_grid = refdump.get_bool(b"roundToGrid");
    ref_0.use_my_metrics = refdump.get_bool(b"useMyMetrics");
    if refdump.get_bool(b"isAnchored") {
        ref_0.is_anchored = RefAnchorStatus::AnchorXy;
        ref_0.inner = refdump.get_int(b"inner") as ShapeId;
        ref_0.outer = refdump.get_int(b"outer") as ShapeId;
    }
    ref_0
}
unsafe fn glyf_parse_references(col: Option<&ParsedValue>, g: *mut Glyph) {
    let Some(items) = col.and_then(ParsedValue::as_array) else {
        return;
    };
    for refdump in items {
        (*g).references.push(glyf_parse_reference(refdump));
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
unsafe fn parse_stems(sd: Option<&ParsedValue>, stems: *mut StemDefList) {
    let Some(items) = sd.and_then(ParsedValue::as_array) else {
        return;
    };
    for s in items {
        if s.as_object().is_some() {
            let sdef = PostscriptStemDef {
                position: s.get_num(b"position") as Pos,
                width: s.get_num(b"width") as Pos,
                map: 0_u16,
            };
            (*stems).push(sdef);
        }
    }
}
/// `arr` is always a freshly zero-initialized `[bool; 256]` field (both call
/// sites in `parse_masks` construct `PostscriptHintMask` with `mask_h`/
/// `mask_v: [false; 256]` immediately beforehand), so there's no separate
/// "no bits given" branch to re-zero it -- entries past `bits`'s length just
/// keep their already-`false` initial value.
fn parse_maskbits(arr: &mut [bool], bits: Option<&ParsedValue>) {
    let Some(items) = bits.and_then(ParsedValue::as_array) else {
        return;
    };
    for (slot, b) in arr.iter_mut().zip(items) {
        *slot = match b {
            ParsedValue::Bool(v) => *v,
            ParsedValue::Int(v) => *v != 0,
            ParsedValue::Double(v) => *v != 0.,
            _ => false,
        };
    }
}
unsafe fn parse_masks(md: Option<&ParsedValue>, masks: *mut MaskList) {
    let Some(items) = md.and_then(ParsedValue::as_array) else {
        return;
    };
    for m in items {
        if m.as_object().is_none() {
            continue;
        }
        let mut mask = PostscriptHintMask {
            points_before: m.get_int(b"pointsBefore") as u16,
            contours_before: m.get_int(b"contoursBefore") as u16,
            mask_h: [false; 256],
            mask_v: [false; 256],
        };
        parse_maskbits(&mut mask.mask_h, m.get_typed(b"maskH", JsonType::Array));
        parse_maskbits(&mut mask.mask_v, m.get_typed(b"maskV", JsonType::Array));
        (*masks).push(mask);
    }
}
unsafe fn otfcc_glyf_parse_glyph(
    glyphdump: &ParsedValue,
    order_entry: &GlyphOrderEntry,
    options: &Options,
) -> Box<Glyph> {
    let mut g: Box<Glyph> = otfcc_new_glyf_glyph();
    (*g).name = order_entry.name.clone();
    vq_replace(
        &raw mut (*g).advance_width,
        json_vq_of(
            glyphdump.get(b"advanceWidth").map_or(::core::ptr::null(), |v| v as *const ParsedValue),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    vq_replace(
        &raw mut (*g).horizontal_origin,
        json_vq_of(
            glyphdump
                .get(b"horizontalOrigin")
                .map_or(::core::ptr::null(), |v| v as *const ParsedValue),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    vq_replace(
        &raw mut (*g).advance_height,
        json_vq_of(
            glyphdump.get(b"advanceHeight").map_or(::core::ptr::null(), |v| v as *const ParsedValue),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    vq_replace(
        &raw mut (*g).vertical_origin,
        json_vq_of(
            glyphdump.get(b"verticalOrigin").map_or(::core::ptr::null(), |v| v as *const ParsedValue),
            ::core::ptr::null::<FvarTable>(),
        ) as VQ,
    );
    glyf_parse_contours(glyphdump.get_typed(b"contours", JsonType::Array), &raw mut *g);
    glyf_parse_references(glyphdump.get_typed(b"references", JsonType::Array), &raw mut *g);
    if !options.ignore_hints {
        parse_ttinstr(
            glyphdump.get(b"instructions").map_or(::core::ptr::null(), |v| v as *const ParsedValue),
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
        parse_stems(glyphdump.get_typed(b"stemH", JsonType::Array), &raw mut (*g).stem_h);
        parse_stems(glyphdump.get_typed(b"stemV", JsonType::Array), &raw mut (*g).stem_v);
        parse_masks(
            glyphdump.get_typed(b"hintMasks", JsonType::Array),
            &raw mut (*g).hint_masks,
        );
        parse_masks(
            glyphdump.get_typed(b"contourMasks", JsonType::Array),
            &raw mut (*g).contour_masks,
        );
        (*g).y_pel = glyphdump.get_int(b"LTSH_yPel") as u8;
    }
    (*g).fd_select = handle_from_name(glyphdump.get_bytes_owned(b"CFF_fdSelect")) as FdHandle;
    if (*g).y_pel == 0 {
        (*g).y_pel = glyphdump.get_int(b"yPel") as u8;
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
    let table: *mut ParsedValue = root
        .as_ref()
        .and_then(|r| r.get_typed(b"glyf", JsonType::Object))
        .map_or(::core::ptr::null_mut(), |v| {
            v as *const ParsedValue as *mut ParsedValue
        });
    if table.is_null() {
        return None;
    }
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"glyf"),
    );
    let n = table.as_ref().and_then(ParsedValue::as_object).map_or(0, |f| f.len());
    let mut glyf_val: GlyfTable = Vec::with_capacity(n);
    glyf_val.resize_with(n, || None);
    // Each iteration reads glyph `j` fully (into an owned `Box<Glyph>`,
    // via `otfcc_glyf_parse_glyph`) before nulling that same slot out --
    // never both at once -- so the immutable reborrow below (`fields`,
    // scoped to this iteration) is always finished before the mutable one
    // (`table.as_mut()`) begins.
    for j in 0..n {
        let Some(fields) = table.as_ref().and_then(ParsedValue::as_object) else {
            break;
        };
        let (name_key, glyphdump) = &fields[j];
        let name_bytes = &name_key[..name_key.len() - 1];
        let order_idx = (*glyph_order).by_name.get(name_bytes).copied();
        if glyphdump.as_object().is_some() {
            if let Some(idx) = order_idx {
                let order_entry = &(&(*glyph_order).entries)[idx];
                if glyf_val[order_entry.gid as usize].is_none() {
                    glyf_val[order_entry.gid as usize] =
                        Some(otfcc_glyf_parse_glyph(glyphdump, order_entry, options));
                }
            }
        }
        if let Some(t) = table.as_mut() {
            t.take_field(j);
        }
    }
    logger_finish(&mut *options.logger.borrow_mut());
    Some(glyf_val)
}

pub struct GlyfAndLocaBuffers {
    pub glyf: Buffer,
    pub loca: Buffer,
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
