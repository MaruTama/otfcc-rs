pub mod build;
pub mod read;

use crate::json_writer::DumpSink;
use crate::logger::ByteStr;
use crate::support::TRUE_0;
use otfcc_binary::Buffer;
use crate::support::glyph_order::{GlyphOrder, GlyphOrderEntry};
use crate::support::handle::{
    FdHandle, GlyphHandle, Handle, HandleState, handle_from_name, handle_empty,
};
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Pos, Scale, ShapeId, until_nul};
use crate::table::fvar::FvarTable;
use otfcc_json::JsonType;

use otfcc_json::BuiltValue;
use otfcc_json::ParsedValue;
use crate::support::ttinstr::{dump_ttinstr, parse_ttinstr};
use crate::table::fvar::{json_new_vq, json_vq_of};
use crate::vf::vq::VQ;
use crate::vf::vq::{vq_create_still, vq_get_still, vq_is_still};

#[derive(Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct PostscriptStemDef {
    pub position: Pos,
    pub width: Pos,
    pub map: u16,
}
pub type StemDefList = Vec<PostscriptStemDef>;
/// One axis of a hint mask: an on/off flag per stem hint, for up to 256
/// stems, packed into bits (a CID font can have tens of thousands of hinted
/// glyphs, each with several masks).
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub struct StemMask([u64; 4]);
impl StemMask {
    /// The number of stems a mask can hold.
    pub const LEN: usize = 256;

    /// Whether stem `i` is on. Panics for `i >= 256`, as indexing the
    /// array it replaces did.
    pub fn get(&self, i: usize) -> bool {
        self.0[i / 64] >> (i % 64) & 1 != 0
    }

    /// Turns stem `i` on or off. Panics for `i >= 256`.
    pub fn set(&mut self, i: usize, on: bool) {
        let bit = 1_u64 << (i % 64);
        if on {
            self.0[i / 64] |= bit;
        } else {
            self.0[i / 64] &= !bit;
        }
    }

    /// All 256 flags, stem 0 first.
    pub fn bits(&self) -> impl Iterator<Item = bool> + '_ {
        (0..Self::LEN).map(|i| self.get(i))
    }
}
#[derive(Copy, Clone, Debug)]
pub struct PostscriptHintMask {
    pub points_before: u16,
    pub contours_before: u16,
    pub mask_h: StemMask,
    pub mask_v: StemMask,
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
// `is_anchored`/`x`/`y` are the three fields `consolidate/glyf.rs`'s
// `get_point_coordinates`/`consolidate_anchor_ref` mutate while walking a
// *shared* `&GlyfTable` (see that file's own doc comment on the pair for
// why the walk itself can never hold `&mut` access to the table it
// recurses over: the reference graph it walks can revisit the same
// `ComponentReference` twice on different call paths, which is exactly
// what those two functions' cycle-detection guards exist to catch, not
// prevent). `is_anchored` is `Copy`, so `Cell` costs nothing beyond a
// `.get()`/`.set()` pair at each site and can never panic. `x`/`y` are
// `VQ`, not `Copy` (a `Vec`-backed `shift` list), so they need `RefCell`
// instead -- sound here specifically because every read or write of a
// given `ComponentReference`'s `x`/`y` in `consolidate/glyf.rs` happens as a
// single, non-recursive statement: `consolidate_anchor_ref` only touches
// its own `rr.x`/`rr.y` *after* both of its recursive
// `get_point_coordinates` calls have already returned (never while one is
// in flight), and any re-entrant call that reaches the very same
// `ComponentReference` while its resolution is already in progress is
// turned away by the `is_anchored` state-machine guard *before* it ever
// reaches the `x`/`y`-touching code -- so no borrow of a given
// `ComponentReference`'s `x`/`y` is ever still outstanding when a nested
// call could try to borrow that same one again. See `consolidate/glyf.rs`'s
// own doc comments on `get_point_coordinates`/`consolidate_anchor_ref` for
// the full trace this reasoning is based on.
#[derive(Clone, Debug)]
pub struct ComponentReference {
    pub x: std::cell::RefCell<VQ>,
    pub y: std::cell::RefCell<VQ>,
    pub round_to_grid: bool,
    pub use_my_metrics: bool,
    pub glyph: GlyphHandle,
    pub a: Scale,
    pub b: Scale,
    pub c: Scale,
    pub d: Scale,
    pub is_anchored: std::cell::Cell<RefAnchorStatus>,
    pub inner: ShapeId,
    pub outer: ShapeId,
}
/// A glyph's component references. Each [`ComponentReference`] embeds two
/// `VQ`s and a `GlyphHandle`, all of which now own their allocations for
/// real and auto-drop, so -- like [`Contour`] -- dropping/clearing this
/// container needs no explicit per-element dispose pass.
pub type ReferenceList = Vec<ComponentReference>;
#[derive(Copy, Clone, Debug)]
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
#[derive(Clone, Debug)]
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
/// The font's glyph table, indexed by GID. A slot is `None` until it is
/// filled: `table_glyf_create_n` sizes the table to the glyph count before
/// reading, and `consolidate_glyf` fills any slot left empty.
pub type GlyfTable = Vec<Option<Box<Glyph>>>;
/// Iterate a fully populated `GlyfTable` as `&Glyph`s in GID order,
/// panicking on the first unset slot reached (lazily, like the indexed
/// `glyf[j].as_deref().unwrap()` loops this replaces). For the passes that
/// run after `consolidate_glyf` has patched every hole -- not for a
/// hole-tolerant scan, which wants `.iter().flatten()` instead.
pub(crate) fn iter_glyphs(glyf: &GlyfTable) -> impl Iterator<Item = &Glyph> {
    glyf.iter().map(|slot| slot.as_deref().unwrap())
}
#[derive(Debug)]
pub struct GlyfIOContext<'a> {
    pub loca_is_long: bool,
    pub num_glyphs: GlyphId,
    pub n_phantom_points: ShapeId,
    // `None` when the font has no `fvar` table. Only reading mutates it,
    // to register regions.
    pub fvar: Option<&'a mut FvarTable>,
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
fn create_point(p: &mut Point) {
    p.x = vq_create_still(0_i32 as Pos);
    p.y = vq_create_still(0_i32 as Pos);
    p.on_curve = TRUE_0 as i8;
}
fn copy_point(dst: &mut Point, src: &Point) {
    dst.x = src.x.clone();
    dst.y = src.y.clone();
    dst.on_curve = src.on_curve;
}
#[inline]
pub fn glyf_point_init(x: &mut Point) {
    create_point(x);
}
#[inline]
pub fn glyf_point_dup(src: Point) -> Point {
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
    glyf_point_copy(&mut dst, &src);
    return dst;
}
#[inline]
fn glyf_point_copy(dst: &mut Point, src: &Point) {
    copy_point(dst, src);
}
/// Grows `arr` to `n` points, each a default point from
/// [`glyf_point_init`].
#[inline]
fn glyf_contour_fill(arr: &mut Contour, n: usize) {
    for _ in arr.len()..n {
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
        glyf_point_init(&mut x);
        arr.push(x);
    }
}
#[inline]
fn init_glyf_reference(glyph_ref: &mut ComponentReference) {
    glyph_ref.glyph = handle_empty() as GlyphHandle;
    glyph_ref.x = std::cell::RefCell::new(vq_create_still(0_i32 as Pos));
    glyph_ref.y = std::cell::RefCell::new(vq_create_still(0_i32 as Pos));
    glyph_ref.a = 1_i32 as Scale;
    glyph_ref.b = 0_i32 as Scale;
    glyph_ref.c = 0_i32 as Scale;
    glyph_ref.d = 1_i32 as Scale;
    glyph_ref.is_anchored = std::cell::Cell::new(RefAnchorStatus::Xy);
    glyph_ref.outer = 0 as ShapeId;
    glyph_ref.inner = glyph_ref.outer;
    glyph_ref.round_to_grid = false;
    glyph_ref.use_my_metrics = false;
}
#[inline]
pub fn glyf_component_reference_empty() -> ComponentReference {
    let mut x: ComponentReference = ComponentReference {
        x: std::cell::RefCell::new(VQ {
            kernel: 0.,
            shift: Vec::new(),
        }),
        y: std::cell::RefCell::new(VQ {
            kernel: 0.,
            shift: Vec::new(),
        }),
        round_to_grid: false,
        use_my_metrics: false,
        glyph: Handle::new(HandleState::Empty, 0, Vec::new()),
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        is_anchored: std::cell::Cell::new(RefAnchorStatus::Xy),
        inner: 0,
        outer: 0,
    };
    glyf_component_reference_init(&mut x);
    return x;
}
#[inline]
pub fn glyf_component_reference_init(x: &mut ComponentReference) {
    init_glyf_reference(x);
}
/// An empty glyph.
pub fn new_glyf_glyph() -> Box<Glyph> {
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
        fd_select: handle_empty() as FdHandle,
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
pub(crate) fn table_glyf_create_n(n: usize) -> GlyfTable {
    let mut v: GlyfTable = Vec::with_capacity(n);
    v.resize_with(n, || None);
    v
}
fn glyf_glyph_dump_contours(g: &Glyph, target: &mut BuiltValue, ctx: &GlyfIOContext<'_>) {
    if g.contours.is_empty() {
        return;
    }
    let mut contours = BuiltValue::new_array(g.contours.len());
    for c in g.contours.iter() {
        let mut contour = BuiltValue::new_array(c.len());
        for p in c.iter() {
            let mut point = BuiltValue::new_object(4);
            // `ctx.fvar` is a real `Option<&mut FvarTable>` now (Stage
            // M-12); `.as_deref()` downgrades it to the `Option<&FvarTable>`
            // `json_new_vq` wants, no `unsafe` needed.
            point.push_field(b"x", json_new_vq(p.x.clone(), ctx.fvar.as_deref()));
            point.push_field(b"y", json_new_vq(p.y.clone(), ctx.fvar.as_deref()));
            point.push_field(b"on", BuiltValue::Bool(p.on_curve & MASK_ON_CURVE != 0));
            contour.push_item(point);
        }
        contours.push_item(contour.preserialize());
    }
    target.push_field(b"contours", contours);
}
fn glyf_glyph_dump_references(g: &Glyph, target: &mut BuiltValue, ctx: &GlyfIOContext<'_>) {
    if g.references.is_empty() {
        return;
    }
    let mut references = BuiltValue::new_array(g.references.len());
    for r in g.references.iter() {
        let mut ref_json = BuiltValue::new_object(9);
        ref_json.push_field(b"glyph", BuiltValue::str_truncated_at_nul(&r.glyph.name));
        // See the comment on the `json_new_vq` calls in
        // `glyf_glyph_dump_contours` above.
        ref_json.push_field(b"x", json_new_vq(r.x.borrow().clone(), ctx.fvar.as_deref()));
        ref_json.push_field(b"y", json_new_vq(r.y.borrow().clone(), ctx.fvar.as_deref()));
        ref_json.push_field(b"a", BuiltValue::position(r.a as Pos));
        ref_json.push_field(b"b", BuiltValue::position(r.b as Pos));
        ref_json.push_field(b"c", BuiltValue::position(r.c as Pos));
        ref_json.push_field(b"d", BuiltValue::position(r.d as Pos));
        if r.is_anchored.get() != RefAnchorStatus::Xy {
            ref_json.push_field(b"isAnchored", BuiltValue::Bool(true));
            ref_json.push_field(b"inner", BuiltValue::Int(r.inner as i64));
            ref_json.push_field(b"outer", BuiltValue::Int(r.outer as i64));
        }
        if r.round_to_grid {
            ref_json.push_field(b"roundToGrid", BuiltValue::Bool(true));
        }
        if r.use_my_metrics {
            ref_json.push_field(b"useMyMetrics", BuiltValue::Bool(true));
        }
        references.push_item(ref_json.preserialize());
    }
    target.push_field(b"references", references);
}
fn glyf_glyph_dump_stemdefs(stems: &StemDefList) -> BuiltValue {
    let mut a = BuiltValue::new_array(stems.len());
    for stem_def in stems.iter() {
        let mut stem = BuiltValue::new_object(3);
        stem.push_field(b"position", BuiltValue::position(stem_def.position));
        stem.push_field(b"width", BuiltValue::position(stem_def.width));
        a.push_item(stem);
    }
    a
}
fn glyf_glyph_dump_maskdefs(masks: &MaskList, hh: &StemDefList, vv: &StemDefList) -> BuiltValue {
    let mut a = BuiltValue::new_array(masks.len());
    for entry in masks.iter() {
        let mut mask = BuiltValue::new_object(3);
        mask.push_field(
            b"contoursBefore",
            BuiltValue::Int(entry.contours_before as i64),
        );
        mask.push_field(b"pointsBefore", BuiltValue::Int(entry.points_before as i64));
        // Bounded by the glyph's own stem counts (and by the mask's 256
        // flags).
        let mut h = BuiltValue::new_array(hh.len());
        for bit in entry.mask_h.bits().take(hh.len()) {
            h.push_item(BuiltValue::Bool(bit));
        }
        mask.push_field(b"maskH", h);
        let mut v = BuiltValue::new_array(vv.len());
        for bit in entry.mask_v.bits().take(vv.len()) {
            v.push_item(BuiltValue::Bool(bit));
        }
        mask.push_field(b"maskV", v);
        a.push_item(mask);
    }
    a
}
fn glyf_dump_glyph(g: &Glyph, options: &Options, ctx: &GlyfIOContext<'_>) -> BuiltValue {
    let mut glyph = BuiltValue::new_object(12);
    glyph.push_field(
        b"advanceWidth",
        json_new_vq(g.advance_width.clone(), ctx.fvar.as_deref()),
    );
    if vq_is_still(g.horizontal_origin.clone())
        && (vq_get_still(g.horizontal_origin.clone()) as f64).abs()
            > 1.0f64 / 1000.0f64
    {
        glyph.push_field(
            b"horizontalOrigin",
            json_new_vq(g.horizontal_origin.clone(), ctx.fvar.as_deref()),
        );
    }
    if ctx.has_vertical_metrics {
        glyph.push_field(
            b"advanceHeight",
            json_new_vq(g.advance_height.clone(), ctx.fvar.as_deref()),
        );
        glyph.push_field(
            b"verticalOrigin",
            json_new_vq(g.vertical_origin.clone(), ctx.fvar.as_deref()),
        );
    }
    glyf_glyph_dump_contours(g, &mut glyph, ctx);
    glyf_glyph_dump_references(g, &mut glyph, ctx);
    if ctx.export_fd_select {
        glyph.push_field(
            b"CFF_fdSelect",
            BuiltValue::str_truncated_at_nul(&g.fd_select.name),
        );
        glyph.push_field(b"CFF_CID", BuiltValue::Int(g.cid as i64));
    }
    if !options.ignore_hints {
        if !g.instructions.is_empty() {
            glyph.push_field(b"instructions", dump_ttinstr(&g.instructions, options));
        }
        if !g.stem_h.is_empty() {
            glyph.push_field(b"stemH", glyf_glyph_dump_stemdefs(&g.stem_h).preserialize());
        }
        if !g.stem_v.is_empty() {
            glyph.push_field(b"stemV", glyf_glyph_dump_stemdefs(&g.stem_v).preserialize());
        }
        if !g.hint_masks.is_empty() {
            glyph.push_field(
                b"hintMasks",
                glyf_glyph_dump_maskdefs(&g.hint_masks, &g.stem_h, &g.stem_v).preserialize(),
            );
        }
        if !g.contour_masks.is_empty() {
            glyph.push_field(
                b"contourMasks",
                glyf_glyph_dump_maskdefs(&g.contour_masks, &g.stem_h, &g.stem_v).preserialize(),
            );
        }
        if g.y_pel != 0 {
            glyph.push_field(b"LTSH_yPel", BuiltValue::Int(g.y_pel as i64));
        }
    }
    glyph
}
fn dump_glyphorder(table: &GlyfTable) -> BuiltValue {
    let mut order = BuiltValue::new_array(table.len());
    for slot in table {
        let g = slot.as_deref().unwrap();
        order.push_item(BuiltValue::str_truncated_at_nul(&g.name));
    }
    order.preserialize()
}
/// Writes `glyf` and `glyph_order` into `sink` one glyph at a time, so a
/// sink that writes straight out never holds more than one glyph's JSON.
/// Glyph names are cut at the first NUL, as `push_field_bytes_key` does.
pub fn dump_glyf(
    table: Option<&GlyfTable>,
    sink: &mut dyn DumpSink,
    options: &Options,
    ctx: &GlyfIOContext<'_>,
) {
    let Some(table) = table else {
        return;
    };
    let stage = crate::logger::stage("glyf");
    sink.begin_field_object(b"glyf");
    for slot in table {
        let g = slot.as_deref().unwrap();
        sink.field(until_nul(&g.name), glyf_dump_glyph(g, options, ctx));
    }
    sink.end_object();
    if !options.ignore_glyph_order {
        sink.field(b"glyph_order", dump_glyphorder(table));
    }
    stage.finish();
}
fn glyf_parse_point(pointdump: &ParsedValue) -> Point {
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
    glyf_point_init(&mut point);
    let Some(fields) = pointdump.as_object() else {
        return point;
    };
    for (key, val) in fields {
        match &key[..key.len() - 1] {
            b"x" => point.x = json_vq_of(Some(val)),
            b"y" => point.y = json_vq_of(Some(val)),
            b"on" => point.on_curve = val.as_bool().unwrap_or(false) as i8,
            _ => {}
        }
    }
    point
}
fn glyf_parse_contours(col: Option<&ParsedValue>, g: &mut Glyph) {
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
        g.contours.push(contour);
    }
}
fn glyf_parse_reference(refdump: &ParsedValue) -> ComponentReference {
    let mut glyph_ref: ComponentReference = glyf_component_reference_empty();
    let Some(_gname) = refdump.get_typed(b"glyph", JsonType::String) else {
        glyph_ref.glyph.name = Vec::new();
        glyph_ref.x = std::cell::RefCell::new(vq_create_still(0_i32 as Pos));
        glyph_ref.y = std::cell::RefCell::new(vq_create_still(0_i32 as Pos));
        glyph_ref.a = 1.0f64 as Scale;
        glyph_ref.b = 0.0f64 as Scale;
        glyph_ref.c = 0.0f64 as Scale;
        glyph_ref.d = 1.0f64 as Scale;
        glyph_ref.round_to_grid = false;
        glyph_ref.use_my_metrics = false;
        return glyph_ref;
    };
    glyph_ref.glyph = handle_from_name(_gname.as_str_bytes().map(|b| b.to_vec()));
    glyph_ref.x = std::cell::RefCell::new(json_vq_of(refdump.get(b"x")));
    glyph_ref.y = std::cell::RefCell::new(json_vq_of(refdump.get(b"y")));
    glyph_ref.a = refdump.get_num_or(b"a", 1.0f64) as Scale;
    glyph_ref.b = refdump.get_num_or(b"b", 0.0f64) as Scale;
    glyph_ref.c = refdump.get_num_or(b"c", 0.0f64) as Scale;
    glyph_ref.d = refdump.get_num_or(b"d", 1.0f64) as Scale;
    glyph_ref.round_to_grid = refdump.get_bool(b"roundToGrid");
    glyph_ref.use_my_metrics = refdump.get_bool(b"useMyMetrics");
    if refdump.get_bool(b"isAnchored") {
        glyph_ref.is_anchored = std::cell::Cell::new(RefAnchorStatus::AnchorXy);
        glyph_ref.inner = refdump.get_int(b"inner") as ShapeId;
        glyph_ref.outer = refdump.get_int(b"outer") as ShapeId;
    }
    glyph_ref
}
fn glyf_parse_references(col: Option<&ParsedValue>, g: &mut Glyph) {
    let Some(items) = col.and_then(ParsedValue::as_array) else {
        return;
    };
    for refdump in items {
        g.references.push(glyf_parse_reference(refdump));
    }
}
fn parse_stems(sd: Option<&ParsedValue>, stems: &mut StemDefList) {
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
            stems.push(sdef);
        }
    }
}
/// Reads up to 256 flags into `mask`, which starts all off (both call sites
/// in `parse_masks` pass a fresh mask), so flags past the end of `bits`
/// stay off.
fn parse_maskbits(mask: &mut StemMask, bits: Option<&ParsedValue>) {
    let Some(items) = bits.and_then(ParsedValue::as_array) else {
        return;
    };
    for (i, b) in items.iter().take(StemMask::LEN).enumerate() {
        let on = match b {
            ParsedValue::Bool(v) => *v,
            ParsedValue::Int(v) => *v != 0,
            ParsedValue::Double(v) => *v != 0.,
            _ => false,
        };
        mask.set(i, on);
    }
}
fn parse_masks(md: Option<&ParsedValue>, masks: &mut MaskList) {
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
            mask_h: StemMask::default(),
            mask_v: StemMask::default(),
        };
        parse_maskbits(&mut mask.mask_h, m.get_typed(b"maskH", JsonType::Array));
        parse_maskbits(&mut mask.mask_v, m.get_typed(b"maskV", JsonType::Array));
        masks.push(mask);
    }
}
fn glyf_parse_glyph(
    glyphdump: &ParsedValue,
    order_entry: &GlyphOrderEntry,
    options: &Options,
) -> Box<Glyph> {
    let mut g: Box<Glyph> = new_glyf_glyph();
    g.name = order_entry.name.clone();
    g.advance_width = json_vq_of(glyphdump.get(b"advanceWidth"));
    g.horizontal_origin = json_vq_of(glyphdump.get(b"horizontalOrigin"));
    g.advance_height = json_vq_of(glyphdump.get(b"advanceHeight"));
    g.vertical_origin = json_vq_of(glyphdump.get(b"verticalOrigin"));
    glyf_parse_contours(glyphdump.get_typed(b"contours", JsonType::Array), &mut g);
    glyf_parse_references(glyphdump.get_typed(b"references", JsonType::Array), &mut g);
    if !options.ignore_hints {
        parse_ttinstr(
            glyphdump.get(b"instructions"),
            |instrs| g.instructions = instrs,
            |reason: &[u8], pos| {
                // Same idiom the rest of this file already uses for
                // `Options`-carrying diagnostics (`logger_start_sds`/
                // `logger_finish`, above and below): `options` is right
                // here in scope, so this drops the raw `fprintf`-to-stderr
                // call (and the NUL-terminated byte-copies it needed) for
                // a real `Logger` call, not just an `eprintln!`. Per
                // `tests/log_output.rs`'s own doc comment, this crate's
                // stderr-comparison tests pin only output written through
                // the `Logger` -- and this message wasn't reaching the
                // `Logger` at all before, so no golden fixture already
                // depends on its exact old wording.
                tracing::warn!("[OTFCC] TrueType instructions parse error : {}, at {} in /{}\n", ByteStr(reason), pos, ByteStr(&g.name));
            },
        );
        parse_stems(glyphdump.get_typed(b"stemH", JsonType::Array), &mut g.stem_h);
        parse_stems(glyphdump.get_typed(b"stemV", JsonType::Array), &mut g.stem_v);
        parse_masks(
            glyphdump.get_typed(b"hintMasks", JsonType::Array),
            &mut g.hint_masks,
        );
        parse_masks(
            glyphdump.get_typed(b"contourMasks", JsonType::Array),
            &mut g.contour_masks,
        );
        g.y_pel = glyphdump.get_int(b"LTSH_yPel") as u8;
    }
    g.fd_select = handle_from_name(glyphdump.get_bytes_owned(b"CFF_fdSelect"));
    if g.y_pel == 0 {
        g.y_pel = glyphdump.get_int(b"yPel") as u8;
    }
    return g;
}
// Reads `glyf` from the JSON, taking each glyph's value out of `root` as it
// goes. `glyph_order` is `None` when the JSON has none.
pub fn parse_glyf(
    root: &mut ParsedValue,
    glyph_order: Option<&GlyphOrder>,
    options: &Options,
) -> Option<GlyfTable> {
    let glyph_order = glyph_order?;
    root.as_object()?;
    let table = root.get_typed_mut(b"glyf", JsonType::Object)?;
    let stage = crate::logger::stage("glyf");
    let n = table.as_object().map_or(0, |f| f.len());
    let mut glyf_val: GlyfTable = Vec::with_capacity(n);
    glyf_val.resize_with(n, || None);
    // Each iteration reads glyph `j` fully (into an owned `Box<Glyph>`,
    // via `glyf_parse_glyph`) before nulling that same slot out --
    // never both at once -- so the immutable reborrow below (`fields`,
    // scoped to this iteration) is always finished before the mutable
    // `take_field` call that follows it.
    for j in 0..n {
        let Some(fields) = table.as_object() else {
            break;
        };
        let (name_key, glyphdump) = &fields[j];
        let name_bytes = &name_key[..name_key.len() - 1];
        let order_idx = glyph_order.by_name.get(name_bytes).copied();
        if glyphdump.as_object().is_some()
            && let Some(idx) = order_idx {
                let order_entry = &glyph_order.entries[idx];
                if glyf_val[order_entry.gid as usize].is_none() {
                    glyf_val[order_entry.gid as usize] =
                        Some(glyf_parse_glyph(glyphdump, order_entry, options));
                }
            }
        table.take_field(j);
    }
    stage.finish();
    Some(glyf_val)
}

#[derive(Debug)]
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
    // is a newtype (see RUST_MIGRATION.md).
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

#[cfg(test)]
mod stem_mask_tests {
    use super::StemMask;

    #[test]
    fn set_and_get_each_stem_independently() {
        let mut mask = StemMask::default();
        for i in [0, 1, 63, 64, 127, 128, 200, 255] {
            mask.set(i, true);
        }
        let on: Vec<usize> = (0..StemMask::LEN).filter(|&i| mask.get(i)).collect();
        assert_eq!(on, [0, 1, 63, 64, 127, 128, 200, 255]);
        mask.set(64, false);
        assert!(!mask.get(64));
        assert!(mask.get(63) && mask.get(127));
        assert_eq!(mask.bits().count(), 256);
        assert_eq!(mask.bits().filter(|&b| b).count(), 7);
    }

    #[test]
    #[should_panic]
    fn stem_256_is_out_of_range() {
        StemMask::default().get(256);
    }
}
